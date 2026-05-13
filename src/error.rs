//! Error handling infrastructure

use thiserror::Error;

/// Main error type for the file manager
#[derive(Error, Debug)]
pub enum FileManagerError {
    /// File system related errors
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// Path related errors
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// GTK/UI related errors
    #[error("UI error: {0}")]
    Ui(String),

    /// File operation errors (copy, move, delete)
    #[error("Operation failed: {0}")]
    Operation(String),

    /// VFS errors
    #[error("VFS error: {0}")]
    Vfs(String),

    /// Watcher errors
    #[error("Watcher error: {0}")]
    Watcher(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// File not found
    #[error("File not found: {0}")]
    NotFound(String),

    /// Already exists
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Not a directory
    #[error("Not a directory: {0}")]
    NotDirectory(String),

    /// Not a file
    #[error("Not a file: {0}")]
    NotFile(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias using our custom error
pub type Result<T> = std::result::Result<T, FileManagerError>;

/// Convert a std::io::Error to our custom error with context
pub fn io_error(error: std::io::Error, context: &str) -> FileManagerError {
    match error.kind() {
        std::io::ErrorKind::NotFound => FileManagerError::NotFound(context.to_string()),
        std::io::ErrorKind::PermissionDenied => FileManagerError::PermissionDenied(context.to_string()),
        std::io::ErrorKind::AlreadyExists => FileManagerError::AlreadyExists(context.to_string()),
        std::io::ErrorKind::NotADirectory => FileManagerError::NotDirectory(context.to_string()),
        std::io::ErrorKind::IsADirectory => FileManagerError::NotFile(context.to_string()),
        _ => FileManagerError::FileSystem(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let not_found = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let error = io_error(not_found, "/some/path");
        assert!(matches!(error, FileManagerError::NotFound(_)));
    }

    #[test]
    fn test_permission_denied() {
        let perm_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let error = io_error(perm_err, "/protected/path");
        assert!(matches!(error, FileManagerError::PermissionDenied(_)));
    }
}