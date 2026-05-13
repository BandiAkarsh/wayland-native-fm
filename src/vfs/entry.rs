//! VFS directory entry types
//!
//! Defines the `DirectoryEntry` struct representing a file system entry
//! with path, name, and metadata (size, mtime, file type).

use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

/// Metadata for a file system entry
#[derive(Debug, Clone, PartialEq)]
pub struct EntryMetadata {
    /// File size in bytes (0 for directories/symlinks)
    pub size: u64,
    /// Last modification time as UNIX timestamp in seconds (None if unavailable)
    pub modified_at: Option<u64>,
    /// Whether the entry is a regular file
    pub is_file: bool,
    /// Whether the entry is a directory
    pub is_dir: bool,
    /// Whether the entry is a symbolic link
    pub is_symlink: bool,
}

impl EntryMetadata {
    /// Create metadata from std::fs::Metadata, using symlink_metadata to detect symlinks
    pub fn from_std_metadata(metadata: &std::fs::Metadata, is_symlink: bool) -> Self {
        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        EntryMetadata {
            size: metadata.len(),
            modified_at,
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_symlink,
        }
    }
}

/// A directory entry in the VFS
#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryEntry {
    /// Full canonical path to the entry
    pub path: PathBuf,
    /// File name (leaf component of the path)
    pub name: String,
    /// Metadata for the entry
    pub metadata: EntryMetadata,
}

impl DirectoryEntry {
    /// Create a new DirectoryEntry from a path, extracting metadata automatically
    ///
    /// Uses symlink_metadata to detect symlinks without following them
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let name = path
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Path has no file name: {}", path.display()),
                )
            })?
            .to_string_lossy()
            .to_string();

        // Use symlink_metadata to get metadata without following symlinks
        let std_metadata = std::fs::symlink_metadata(path)?;
        let is_symlink = std::fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        let metadata = EntryMetadata::from_std_metadata(&std_metadata, is_symlink);

        Ok(Self {
            path: path.to_path_buf(),
            name,
            metadata,
        })
    }

    /// Normalize and canonicalize the entry's path
    pub fn normalize_path(&self) -> std::io::Result<PathBuf> {
        // First try canonicalize (resolves symlinks and .. components)
        // Fall back to lexical normalization if canonicalize fails (e.g., path doesn't exist)
        match self.path.canonicalize() {
            Ok(canonical) => Ok(canonical),
            Err(_) => {
                // Lexical normalization: resolve .. without filesystem access
                let mut components = Vec::new();
                for component in self.path.components() {
                    match component {
                        std::path::Component::ParentDir => {
                            if !components.is_empty() {
                                components.pop();
                            }
                        }
                        std::path::Component::CurDir => {} // Skip .
                        _ => components.push(component),
                    }
                }
                let normalized = components
                    .into_iter()
                    .collect::<PathBuf>();
                Ok(normalized)
            }
        }
    }
}
