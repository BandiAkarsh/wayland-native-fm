//! VFS backend trait

use crate::error::FileManagerError;
use crate::vfs::entry::EntryMetadata;
use crate::vfs::DirectoryEntry;
use std::path::{Path, PathBuf};

/// Future type for VFS operations
pub type VfsFuture<T> =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, FileManagerError>>>>;

/// Trait for VFS backend operations
pub trait VfsBackend: Send + Sync {
    fn read_dir(&self, path: &Path) -> VfsFuture<Vec<DirectoryEntry>>;
    fn metadata(&self, path: &Path) -> VfsFuture<EntryMetadata>;
    fn symlink_metadata(&self, path: &Path) -> VfsFuture<EntryMetadata>;
    fn normalize_path(&self, path: &Path) -> VfsFuture<PathBuf>;
}

/// Local filesystem backend
pub struct LocalVfsBackend;

impl LocalVfsBackend {
    pub fn new() -> Self {
        Self
    }
}
