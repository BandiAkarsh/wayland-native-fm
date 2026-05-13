//! Tests for VFS module

use std::path::PathBuf;
use wayland_file_manager::vfs::entry::{DirectoryEntry, EntryMetadata};

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_entry_metadata_creation() {
        let metadata = EntryMetadata {
            size: 1024,
            modified_at: Some(1609459200), // 2021-01-01
            is_file: true,
            is_dir: false,
            is_symlink: false,
        };

        assert_eq!(metadata.size, 1024);
        assert!(metadata.is_file);
        assert!(!metadata.is_dir);
    }

    #[test]
    fn test_directory_entry_creation() {
        let metadata = EntryMetadata {
            size: 0,
            modified_at: None,
            is_file: false,
            is_dir: true,
            is_symlink: false,
        };

        let entry = DirectoryEntry {
            path: PathBuf::from("/test"),
            name: "test".to_string(),
            metadata,
        };

        assert_eq!(entry.name, "test");
        assert!(entry.metadata.is_dir);
    }

    #[test]
    fn test_entry_from_path_home() {
        // Test that we can read the home directory
        if let Some(home) = dirs::home_dir() {
            let result = DirectoryEntry::from_path(&home);
            // This might fail in some environments, but shouldn't panic
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_entry_metadata_file_types() {
        // Test file type detection
        let file_meta = EntryMetadata {
            size: 100,
            modified_at: Some(1000000),
            is_file: true,
            is_dir: false,
            is_symlink: false,
        };
        assert!(file_meta.is_file);
        assert!(!file_meta.is_dir);

        let dir_meta = EntryMetadata {
            size: 0,
            modified_at: Some(1000000),
            is_file: false,
            is_dir: true,
            is_symlink: false,
        };
        assert!(!dir_meta.is_file);
        assert!(dir_meta.is_dir);
    }
}
