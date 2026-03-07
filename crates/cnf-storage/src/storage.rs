use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// Perform an atomic write: write data to a temporary file and then rename.
/// Returns `Ok(())` on success, or an io::Error on failure.
pub fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    // Implementation will go here; tests drive development.
    let tmp_path = path.with_extension("tmp");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)?;
    file.write_all(data)?;
    file.flush()?;
    // Ensure data is on disk
    file.sync_all()?;
    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

/// Storage manager for file operations
pub struct Storage {
    open_files: std::collections::HashMap<u64, std::fs::File>,
    next_handle: u64,
}

impl Storage {
    /// Create new storage instance
    pub fn new() -> Self {
        Storage {
            open_files: std::collections::HashMap::new(),
            next_handle: 1,
        }
    }

    /// Open a file and return a handle
    pub fn open_file(&mut self, path: &str) -> io::Result<u64> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let handle = self.next_handle;
        self.next_handle += 1;
        self.open_files.insert(handle, file);
        Ok(handle)
    }

    /// Read entire file content as string
    pub fn read_file(&mut self, handle: u64) -> io::Result<String> {
        let file = self.open_files.get_mut(&handle)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Invalid file handle"))?;
        let mut contents = String::new();
        std::io::Read::read_to_string(file, &mut contents)?;
        Ok(contents)
    }

    /// Write data to file
    pub fn write_file(&mut self, handle: u64, data: &str) -> io::Result<()> {
        let file = self.open_files.get_mut(&handle)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Invalid file handle"))?;
        file.write_all(data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Close file handle
    pub fn close_file(&mut self, handle: u64) -> io::Result<()> {
        self.open_files.remove(&handle)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Invalid file handle"))?;
        Ok(())
    }

    /// Checkpoint data (placeholder for now)
    pub fn checkpoint(&mut self, _data: &str) -> io::Result<()> {
        // TODO: Implement checkpoint using WAL/checkpoint modules
        Ok(())
    }

    /// Replay data (placeholder for now)
    pub fn replay(&mut self) -> io::Result<String> {
        // TODO: Implement replay using WAL/checkpoint modules
        Ok("".to_string())
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_atomic_write_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let data = b"hello world";
        atomic_write(&file_path, data).unwrap();
        let mut contents = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut contents)
            .unwrap();
        assert_eq!(contents, data);
    }

    #[test]
    fn test_atomic_write_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test2.txt");
        atomic_write(&file_path, b"first").unwrap();
        atomic_write(&file_path, b"second").unwrap();
        let mut contents = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut contents)
            .unwrap();
        assert_eq!(contents, b"second");
    }

    #[test]
    fn test_atomic_write_checksum() {
        // later, checksum requirement
    }
}
