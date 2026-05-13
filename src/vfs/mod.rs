//! Virtual File System (VFS) abstraction module

pub mod backend;
pub mod entry;

pub use backend::LocalVfsBackend;
pub use backend::VfsBackend;
pub use entry::DirectoryEntry;
