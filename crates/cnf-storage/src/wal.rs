//! Write-ahead log support for cnf-storage.
//!
//! WAL ensures durability of operations by logging them before applying.

use crc32fast::Hasher as Crc32Hasher;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// WAL entry structure for logging operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalEntry {
    pub sequence: u64,       // monotonic counter
    pub timestamp: u64,      // unix timestamp
    pub operation: String,   // "WRITE", "CHECKPOINT", etc.
    pub key: String,         // identifier
    pub data_hash: [u8; 32], // SHA-256 of data
    pub crc32: u32,          // integrity check
}

impl WalEntry {
    /// Create a new WAL entry with computed CRC32.
    pub fn new(sequence: u64, operation: String, key: String, data_hash: [u8; 32]) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Compute CRC32 of the entry data first
        let mut hasher = Crc32Hasher::new();
        hasher.update(&sequence.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(operation.as_bytes());
        hasher.update(key.as_bytes());
        hasher.update(&data_hash);
        let crc32 = hasher.finalize();

        WalEntry {
            sequence,
            timestamp,
            operation,
            key,
            data_hash,
            crc32,
        }
    }

    /// Verify the entry's CRC32 integrity.
    pub fn verify_integrity(&self) -> bool {
        let mut hasher = Crc32Hasher::new();
        hasher.update(&self.sequence.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(self.operation.as_bytes());
        hasher.update(self.key.as_bytes());
        hasher.update(&self.data_hash);
        hasher.finalize() == self.crc32
    }
}

/// Write-ahead log manager.
pub struct Wal {
    file: File,
    next_sequence: u64,
}

impl Wal {
    /// Open or create a WAL file.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        // Read existing entries to determine next sequence
        let mut next_sequence = 0u64;
        let mut reader = file.try_clone()?;
        while let Ok(Some(_)) = Self::read_next_entry(&mut reader) {
            next_sequence += 1;
        }

        Ok(Wal {
            file,
            next_sequence,
        })
    }

    /// Append an entry to the WAL (atomic operation).
    pub fn append(
        &mut self,
        operation: String,
        key: String,
        data_hash: [u8; 32],
    ) -> io::Result<()> {
        let entry = WalEntry::new(self.next_sequence, operation, key, data_hash);

        // Serialize entry
        let entry_bytes = serde_json::to_vec(&entry)?;

        // Write format: [length: u32][entry_bytes][crc32: u32]
        let length = entry_bytes.len() as u32;
        let crc32 = entry.crc32;

        // Atomic write: seek to end and write
        self.file.seek(SeekFrom::End(0))?;
        self.file.write_all(&length.to_le_bytes())?;
        self.file.write_all(&entry_bytes)?;
        self.file.write_all(&crc32.to_le_bytes())?;
        self.file.flush()?;
        self.file.sync_all()?;

        self.next_sequence += 1;
        Ok(())
    }

    /// Replay all entries from the WAL.
    pub fn replay(&mut self) -> io::Result<Vec<WalEntry>> {
        let mut entries = Vec::new();
        let mut reader = self.file.try_clone()?;
        reader.seek(SeekFrom::Start(0))?;

        while let Ok(Some(entry)) = Self::read_next_entry(&mut reader) {
            if entry.verify_integrity() {
                entries.push(entry);
            } else {
                // Corrupt entry - stop here (crash safety)
                break;
            }
        }

        Ok(entries)
    }

    /// Truncate entries before the given sequence number.
    pub fn truncate_before(&mut self, sequence: u64) -> io::Result<()> {
        // Read all entries
        let entries = self.replay()?;

        // Filter entries >= sequence
        let remaining: Vec<_> = entries
            .into_iter()
            .filter(|e| e.sequence >= sequence)
            .collect();

        if remaining.is_empty() {
            // Truncate entire file
            self.file.set_len(0)?;
            self.file.seek(SeekFrom::Start(0))?;
        } else {
            // Rewrite file with remaining entries
            self.file.set_len(0)?;
            self.file.seek(SeekFrom::Start(0))?;

            for entry in remaining {
                let entry_bytes = serde_json::to_vec(&entry)?;
                let length = entry_bytes.len() as u32;
                let crc32 = entry.crc32;

                self.file.write_all(&length.to_le_bytes())?;
                self.file.write_all(&entry_bytes)?;
                self.file.write_all(&crc32.to_le_bytes())?;
            }
        }

        self.file.flush()?;
        self.file.sync_all()?;
        Ok(())
    }

    /// Read the next entry from the reader.
    fn read_next_entry<R: Read>(reader: &mut R) -> io::Result<Option<WalEntry>> {
        // Read length
        let mut length_buf = [0u8; 4];
        match reader.read_exact(&mut length_buf) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }

        let length = u32::from_le_bytes(length_buf) as usize;

        // Read entry bytes
        let mut entry_buf = vec![0u8; length];
        reader.read_exact(&mut entry_buf)?;

        // Read CRC32
        let mut crc_buf = [0u8; 4];
        reader.read_exact(&mut crc_buf)?;
        let expected_crc = u32::from_le_bytes(crc_buf);

        // Deserialize entry
        let mut entry: WalEntry = serde_json::from_slice(&entry_buf)?;
        entry.crc32 = expected_crc; // Override with stored CRC

        Ok(Some(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_wal_append_and_replay() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut wal = Wal::open(temp_file.path()).unwrap();

        // Append some entries
        let hash1 = [1u8; 32];
        let hash2 = [2u8; 32];

        wal.append("WRITE".to_string(), "key1".to_string(), hash1)
            .unwrap();
        wal.append("WRITE".to_string(), "key2".to_string(), hash2)
            .unwrap();

        // Replay and verify
        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].operation, "WRITE");
        assert_eq!(entries[0].key, "key1");
        assert_eq!(entries[0].data_hash, hash1);
        assert_eq!(entries[1].key, "key2");
        assert_eq!(entries[1].data_hash, hash2);
    }

    #[test]
    fn test_wal_crash_recovery() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut wal = Wal::open(temp_file.path()).unwrap();

        // Append valid entries
        wal.append("WRITE".to_string(), "key1".to_string(), [1u8; 32])
            .unwrap();
        wal.append("WRITE".to_string(), "key2".to_string(), [2u8; 32])
            .unwrap();

        // Simulate crash by corrupting the last entry
        {
            let mut file = OpenOptions::new()
                .write(true)
                .open(temp_file.path())
                .unwrap();
            file.seek(SeekFrom::End(-4)).unwrap(); // Seek to last CRC32
            file.write_all(&[0xFF, 0xFF, 0xFF, 0xFF]).unwrap(); // Corrupt CRC
        }

        // Replay should stop at first corrupt entry
        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 1); // Only first entry should be valid
        assert_eq!(entries[0].key, "key1");
    }

    #[test]
    fn test_wal_entry_integrity() {
        let entry = WalEntry::new(1, "WRITE".to_string(), "key".to_string(), [42u8; 32]);
        assert!(entry.verify_integrity());

        // Corrupt the entry
        let mut corrupted = entry.clone();
        corrupted.sequence = 999;
        assert!(!corrupted.verify_integrity());
    }

    #[test]
    fn test_wal_truncate() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut wal = Wal::open(temp_file.path()).unwrap();

        // Append entries
        wal.append("WRITE".to_string(), "key1".to_string(), [1u8; 32])
            .unwrap();
        wal.append("WRITE".to_string(), "key2".to_string(), [2u8; 32])
            .unwrap();
        wal.append("WRITE".to_string(), "key3".to_string(), [3u8; 32])
            .unwrap();

        // Truncate before sequence 2
        wal.truncate_before(2).unwrap();

        // Replay should only have entries >= sequence 2
        let entries = wal.replay().unwrap();
        assert_eq!(entries.len(), 1); // Only sequence 2 should remain
        assert_eq!(entries[0].sequence, 2);
    }
}
