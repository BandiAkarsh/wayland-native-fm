//! Scanner - directory scanning with caching

pub mod cache;

use crate::error::FileManagerError;
use crate::vfs::entry::DirectoryEntry;
use std::path::Path;
use std::time::Duration;
use walkdir::WalkDir;

/// Scanner for recursive directory traversal
#[allow(dead_code)]
pub struct Scanner {
    capacity: usize,
    ttl: Duration,
    threshold: usize,
}

impl Scanner {
    pub fn new(capacity: usize, ttl: Duration, threshold: usize) -> Self {
        Self {
            capacity,
            ttl,
            threshold,
        }
    }

    /// Scan a directory recursively and return all entries
    pub fn scan_recursive<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<DirectoryEntry>, FileManagerError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(FileManagerError::NotFound(path.display().to_string()));
        }

        if !path.is_dir() {
            return Err(FileManagerError::NotDirectory(path.display().to_string()));
        }

        let mut entries = Vec::new();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // Skip the root directory itself
            if entry.path() == path {
                continue;
            }

            match DirectoryEntry::from_path(entry.path()) {
                Ok(dir_entry) => entries.push(dir_entry),
                Err(e) => {
                    tracing::warn!("Failed to read entry {}: {}", entry.path().display(), e);
                }
            }
        }

        tracing::debug!("Scanned {} entries from {}", entries.len(), path.display());
        Ok(entries)
    }

    /// Scan a single directory (non-recursive)
    pub fn scan<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DirectoryEntry>, FileManagerError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(FileManagerError::NotFound(path.display().to_string()));
        }

        if !path.is_dir() {
            return Err(FileManagerError::NotDirectory(path.display().to_string()));
        }

        let mut entries = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!("Failed to read directory entry: {}", e);
                    continue;
                }
            };

            match DirectoryEntry::from_path(entry.path()) {
                Ok(dir_entry) => entries.push(dir_entry),
                Err(e) => {
                    tracing::warn!("Failed to read entry {}: {}", entry.path().display(), e);
                }
            }
        }

        tracing::debug!("Scanned {} entries from {}", entries.len(), path.display());
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_scan_home_directory() {
        let scanner = Scanner::new(1000, Duration::from_secs(300), 100);
        let home = env::var("HOME").unwrap_or_else(|_| "/home".to_string());

        // Just test that it doesn't panic
        let result = scanner.scan(&home);
        assert!(
            result.is_ok()
                || result
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_default()
                    .contains("not found")
        );
    }
}
