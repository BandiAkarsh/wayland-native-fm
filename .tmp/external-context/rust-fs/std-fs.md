---
source: Official Rust Documentation (doc.rust-lang.org)
library: Rust Standard Library
package: std::fs
topic: File system operations
fetched: 2026-05-02
official_docs: https://doc.rust-lang.org/std/fs/index.html
---

# std::fs - File System Operations (Rust 1.95.0)

## Overview

The `std::fs` module provides functions for interacting with the file system, including reading, writing, and manipulating files and directories.

## Key Functions for Directory Listing

### read_dir

```rust
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
```

Returns an iterator over the entries within a directory.

**Key Points:**
- Returns `Result<ReadDir>` - iterator yields `io::Result<DirEntry>`
- Entries for `.` and `..` are automatically skipped
- Order is NOT guaranteed - use `.sort()` if reproducible ordering needed
- Errors may occur during iteration (not just at construction)

**Example:**
```rust
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort(); // For reproducible ordering
    Ok(())
}
```

### DirEntry Methods

The `DirEntry` type provides:
- `path()` - Returns the path of the entry
- `file_name()` - Returns the file name as `OsString`
- `file_type()` - Returns the file type
- `metadata()` - Returns the metadata

## File Reading/Writing Functions

### Read Operations
- `read(path)` - Read entire file into `Vec<u8>`
- `read_to_string(path)` - Read entire file into `String`
- `read_to_end(&mut Vec<u8>)` - Append file contents to buffer

### Write Operations
- `write(path, data)` - Write entire slice to file
- `write_all(data)` - Write entire buffer (for open files)

### File Opening
```rust
File::open(path: &str) -> Result<File>
File::create(path: &str) -> Result<File>
OpenOptions::new()
    .read(true)
    .write(true)
    .append(true)
    .open(path)?
```

## Metadata Operations

- `metadata(path)` - Get file metadata (follows symlinks)
- `symlink_metadata(path)` - Get metadata without following symlinks
- `exists(path)` - Check if path exists
- `try_exists(path)` - Check existence without error on permission issues (Rust 1.63+)

## Directory Operations

- `create_dir(path)` - Create single directory
- `create_dir_all(path)` - Create directory with all parents
- `remove_dir(path)` - Remove empty directory
- `remove_dir_all(path)` - Recursively remove directory

## File Operations

- `remove_file(path)` - Remove file
- `rename(old, new)` - Rename/move file
- `copy(from, to)` - Copy file (Rust 1.76+)
- `hard_link(src, dst)` - Create hard link
- `symlink(src, dst)` - Create symbolic link

## Error Handling Best Practices

```rust
use std::fs;
use std::io;

// Use ? operator for propagation
fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// Handle specific errors
match fs::read_dir(path) {
    Ok(entries) => { /* process */ }
    Err(e) => match e.kind() {
        io::ErrorKind::NotFound => println!("Directory not found"),
        io::ErrorKind::PermissionDenied => println!("Permission denied"),
        _ => println!("Other error: {}", e),
    }
}

// Use ok() for graceful degradation
for entry in fs::read_dir(path).ok() {
    // Handle optional entries
}
```

## Platform-Specific Notes

- On Unix: Uses `opendir`/`readdir`
- On Windows: Uses `FindFirstFileEx`/`FindNextFile`
- Order varies between platforms and filesystems