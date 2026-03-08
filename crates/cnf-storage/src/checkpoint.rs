//! Checkpoint support for cnf-storage.
//!
//! Checkpoints provide snapshots of the storage state for fast recovery.

use cnf_security::sha256_hex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Checkpoint state representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointState {
    pub sequence: u64,                  // WAL sequence at checkpoint time
    pub timestamp: u64,                 // unix timestamp
    pub data: HashMap<String, Vec<u8>>, // key -> data mapping
    pub checksum: String,               // SHA-256 of the checkpoint data
}

impl CheckpointState {
    /// Create a new checkpoint state.
    pub fn new(sequence: u64, data: HashMap<String, Vec<u8>>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut state = CheckpointState {
            sequence,
            timestamp,
            data,
            checksum: String::new(),
        };

        // Compute checksum of the serialized data (with sorted keys for determinism)
        let mut sorted_data: Vec<_> = state.data.iter().collect();
        sorted_data.sort_by_key(|(k, _)| *k);
        let data_json = serde_json::to_string(&sorted_data).unwrap();
        state.checksum = sha256_hex(data_json.as_bytes());

        state
    }

    /// Verify the checkpoint's integrity using SHA-256.
    pub fn verify(&self) -> bool {
        let mut sorted_data: Vec<_> = self.data.iter().collect();
        sorted_data.sort_by_key(|(k, _)| *k);
        let data_json = serde_json::to_string(&sorted_data).unwrap();
        let computed_checksum = sha256_hex(data_json.as_bytes());
        computed_checksum == self.checksum
    }
}

/// Checkpoint manager.
pub struct CheckpointManager {
    checkpoint_dir: String,
}

impl CheckpointManager {
    /// Create a new checkpoint manager.
    pub fn new(checkpoint_dir: String) -> Self {
        // Ensure directory exists
        fs::create_dir_all(&checkpoint_dir).unwrap();
        CheckpointManager { checkpoint_dir }
    }

    /// Create a snapshot of the current state.
    pub fn snapshot(&self, sequence: u64, data: HashMap<String, Vec<u8>>) -> std::io::Result<()> {
        let state = CheckpointState::new(sequence, data);

        // Write to temporary file first (atomic)
        let temp_path = format!("{}/checkpoint_{}.tmp", self.checkpoint_dir, sequence);
        let final_path = format!("{}/checkpoint_{}.json", self.checkpoint_dir, sequence);

        let json_data = serde_json::to_string_pretty(&state)?;
        fs::write(&temp_path, &json_data)?;

        // Atomic rename
        fs::rename(&temp_path, &final_path)?;

        Ok(())
    }

    /// Restore the most recent valid checkpoint.
    pub fn restore(&self) -> std::io::Result<Option<CheckpointState>> {
        let mut checkpoints = Vec::new();

        // Find all checkpoint files
        for entry in fs::read_dir(&self.checkpoint_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if filename.starts_with("checkpoint_") && filename.ends_with(".json") {
                        checkpoints.push(path);
                    }
                }
            }
        }

        // Sort by sequence number (descending)
        checkpoints.sort_by(|a, b| {
            let seq_a = Self::extract_sequence(a).unwrap_or(0);
            let seq_b = Self::extract_sequence(b).unwrap_or(0);
            seq_b.cmp(&seq_a) // descending
        });

        // Try to load the most recent valid checkpoint
        for path in checkpoints {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str::<CheckpointState>(&content) {
                    if state.verify() {
                        return Ok(Some(state));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Extract sequence number from checkpoint filename.
    fn extract_sequence(path: &Path) -> Option<u64> {
        path.file_name()?
            .to_str()?
            .strip_prefix("checkpoint_")?
            .strip_suffix(".json")?
            .parse()
            .ok()
    }

    /// Clean up old checkpoints, keeping only the most recent N.
    pub fn cleanup_old(&self, keep_recent: usize) -> std::io::Result<()> {
        let mut checkpoints = Vec::new();

        // Collect all checkpoint files
        for entry in fs::read_dir(&self.checkpoint_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if filename.starts_with("checkpoint_") && filename.ends_with(".json") {
                        if let Some(seq) = Self::extract_sequence(&path) {
                            checkpoints.push((seq, path));
                        }
                    }
                }
            }
        }

        // Sort by sequence (ascending)
        checkpoints.sort_by_key(|(seq, _)| *seq);

        // Remove old checkpoints
        if checkpoints.len() > keep_recent {
            for (_, path) in checkpoints.iter().take(checkpoints.len() - keep_recent) {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_checkpoint_snapshot_restore() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = TempDir::new().unwrap();
        let checkpoint_dir = temp_dir.path().join(format!("checkpoints_{}", timestamp));
        let manager = CheckpointManager::new(checkpoint_dir.to_str().unwrap().to_string());

        // Create test data
        let mut data = HashMap::new();
        data.insert("key1".to_string(), b"value1".to_vec());
        data.insert("key2".to_string(), b"value2".to_vec());

        // Create snapshot
        manager.snapshot(42, data.clone()).unwrap();

        // Restore and verify
        let restored = manager.restore().unwrap().unwrap();
        assert_eq!(restored.sequence, 42);
        assert_eq!(restored.data, data);
        assert!(restored.verify());
    }

    #[test]
    fn test_checkpoint_integrity() {
        let mut data = HashMap::new();
        data.insert("test".to_string(), b"data".to_vec());

        let state = CheckpointState::new(1, data);
        assert!(state.verify());

        // Corrupt the checksum
        let mut corrupted = state.clone();
        corrupted.checksum = "invalid".to_string();
        assert!(!corrupted.verify());
    }

    #[test]
    fn test_checkpoint_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CheckpointManager::new(temp_dir.path().to_str().unwrap().to_string());

        // Create multiple checkpoints
        for i in 0..5 {
            let mut data = HashMap::new();
            data.insert(format!("key{}", i), vec![i as u8]);
            manager.snapshot(i as u64, data).unwrap();
        }

        // Cleanup keeping only 2 most recent
        manager.cleanup_old(2).unwrap();

        // Should have only 2 files left
        let remaining: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(remaining.len(), 2);
    }
}
