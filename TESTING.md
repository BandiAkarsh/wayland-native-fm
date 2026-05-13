# Testing Guide

This document describes the testing infrastructure and how to run tests for the Wayland File Manager.

## Test Organization

Tests are located in two places:

1. **Unit tests** - Embedded in source files (`#[cfg(test)]` modules)
2. **Integration tests** - In `tests/` directory

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Modules

```bash
# VFS tests
cargo test vfs

# Scanner tests
cargo test scanner

# Watcher tests
cargo test watcher

# Operations tests
cargo test operations
```

### Run Specific Tests

```bash
# Run a specific test by name
cargo test test_copy_file_same_content
```

### Test Output

```bash
# Run with verbose output
cargo test -- --nocapture

# Run with detailed logging
RUST_LOG=debug cargo test
```

## Test Modules

### VFS Tests (`tests/vfs_tests.rs`)

Tests for the Virtual File System module:

| Test | Description |
|------|-------------|
| `test_entry_metadata_creation` | Verify EntryMetadata struct creation |
| `test_directory_entry_creation` | Verify DirectoryEntry struct creation |
| `test_entry_from_path_home` | Test reading home directory |
| `test_entry_metadata_file_types` | Verify file type detection |

**Key aspects tested:**
- Metadata extraction (size, modified time)
- File type detection (file, directory, symlink)
- Path handling

### Scanner Tests (`tests/scanner_tests.rs`)

Tests for directory scanning:

| Test | Description |
|------|-------------|
| `test_scanner_creation` | Verify Scanner struct creation |
| `test_scan_nonexistent` | Error handling for missing paths |
| `test_scan_file_not_directory` | Error handling for files |
| `test_scan_recursive` | Recursive directory traversal |
| `test_scan_empty_directory` | Handling empty directories |

**Key aspects tested:**
- Recursive vs non-recursive scanning
- Error handling for invalid paths
- Entry collection

### Watcher Tests (`tests/watcher_tests.rs`)

Tests for file system monitoring:

| Test | Description |
|------|-------------|
| `test_watcher_creation` | Verify FileWatcher creation |
| `test_watcher_watch` | Test watching a path |
| `test_watcher_unwatch` | Test stopping watch |
| `test_watcher_multiple_paths` | Test multiple watched paths |
| `test_watcher_event_types` | Test event type handling |

**Key aspects tested:**
- Watch/unwatch operations
- Event generation (Created, Modified, Deleted, Renamed)
- Multiple path watching

### Operations Tests (`tests/operations_tests.rs`)

Tests for file operations:

| Test | Description |
|------|-------------|
| `test_copy_options_default` | Default CopyOptions values |
| `test_copy_options_builder` | Builder pattern for options |
| `test_copy_options_overwrite` | Overwrite flag handling |
| `test_copy_options_buffer_size` | Buffer size configuration |
| `test_copy_file_same_content` | Basic file copy |
| `test_copy_file_overwrite` | Copy with overwrite |
| `test_copy_file_no_overwrite` | Copy without overwrite (should fail) |

**Key aspects tested:**
- Copy options (overwrite, buffer size)
- Async file operations
- Error handling for existing files

## Unit Tests in Source Files

Each module contains unit tests in `#[cfg(test)]` blocks:

### error.rs Tests

```rust
#[test]
fn test_io_error_conversion() { ... }

#[test]
fn test_permission_denied() { ... }
```

### scanner/mod.rs Tests

```rust
#[test]
fn test_scan_home_directory() { ... }
```

### watcher/mod.rs Tests

```rust
#[test]
fn test_watcher_creation() { ... }
```

### thumbnails/mod.rs Tests

```rust
#[test]
fn test_thumbnail_manager_creation() { ... }

#[test]
fn test_image_format_detection() { ... }
```

### operations/copy.rs Tests

```rust
#[tokio::test]
async fn test_copy_file() { ... }
```

## Writing Tests

### Basic Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange
        let input = something();

        // Act
        let result = do_something(input);

        // Assert
        assert!(result.is_ok());
    }
}
```

### Async Tests

For async operations (using tokio):

```rust
#[tokio::test]
async fn test_async_operation() {
    // Async test code
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

### Using TempDir for File Tests

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_files() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Test code here

    // TempDir automatically cleaned up
}
```

## Test Dependencies

The following dev dependencies are available:

- `tempfile` - For creating temporary directories in tests
- Standard Rust test framework (built-in)

## CI/CD Considerations

Tests are run automatically on:

- Every pull request
- Every push to main branch

Make sure all tests pass before submitting changes!