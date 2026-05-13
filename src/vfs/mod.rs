//! Virtual File System (VFS) abstraction module

pub mod entry;
pub mod backend;

pub use entry::DirectoryEntry;
pub use backend::VfsBackend;
pub use backend::LocalVfsBackend;