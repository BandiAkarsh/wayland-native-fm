---
source: Rust Official Documentation (doc.rust-lang.org)
library: Rust Standard Library
package: std
topic: std::fs::Metadata - file size, mtime, file type detection
fetched: 2026-05-02T00:00:00Z
official_docs: https://doc.rust-lang.org/std/fs/struct.Metadata.html
---

# Rust std::fs::Metadata Documentation

## Overview

`std::fs::Metadata` provides metadata information about a file. This structure is returned from the `metadata` or `symlink_metadata` function or method and represents known metadata about a file such as its permissions, size, modification times, etc.

**Rust Version**: 1.95.0 (59807616e 2026-04-14)

## Key Methods for VFS DirectoryEntry Implementation

### File Size

#### `len()` - Get File Size in Bytes

```rust
pub fn len(&self) -> u64
```

Returns the size of the file, in bytes, this metadata is for.

**Example:**
```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("foo.txt")?;
    println!("File size: {} bytes", metadata.len());
    Ok(())
}
```

---

### Modification Time (mtime)

#### `modified()` - Get Last Modification Time

```rust
pub fn modified(&self) -> Result<SystemTime>
```

Returns the last modification time listed in this metadata.

The returned value corresponds to the `mtime` field of `stat` on Unix platforms and the `ftLastWriteTime` field on Windows platforms.

**Errors:** This field might not be available on all platforms, and will return an `Err` on platforms where it is not available.

**Example:**
```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("foo.txt")?;

    if let Ok(time) = metadata.modified() {
        println!("Modified: {time:?}");
        // Convert to UNIX timestamp if needed:
        let duration = time.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        println!("Modified (UNIX): {} seconds", duration.as_secs());
    } else {
        println!("Not supported on this platform");
    }
    Ok(())
}
```

**Converting SystemTime to useful formats:**
```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn get_mtime_secs(metadata: &std::fs::Metadata) -> Option<u64> {
    metadata.modified().ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
}
```

---

### File Type Detection

#### `is_file()` - Check if Regular File

```rust
pub fn is_file(&self) -> bool
```

Returns `true` if this metadata is for a regular file. The result is mutually exclusive to the result of `Metadata::is_dir`, and will be false for symlink metadata obtained from `symlink_metadata`.

**Example:**
```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("foo.txt")?;
    assert!(metadata.is_file());
    Ok(())
}
```

---

#### `is_dir()` - Check if Directory

```rust
pub fn is_dir(&self) -> bool
```

Returns `true` if this metadata is for a directory. The result is mutually exclusive to the result of `Metadata::is_file`, and will be false for symlink metadata obtained from `symlink_metadata`.

**Example:**
```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("foo.txt")?;
    assert!(!metadata.is_dir());
    Ok(())
}
```

---

#### `is_symlink()` - Check if Symbolic Link

```rust
pub fn is_symlink(&self) -> bool
```

Returns `true` if this metadata is for a symbolic link.

**Important:** To detect symlinks, use `fs::symlink_metadata()` instead of `fs::metadata()`. The `metadata()` function follows symlinks, so `is_symlink()` will always return `false` when using it.

**Example:**
```rust
use std::fs;
use std::path::Path;
use std::os::unix::fs::symlink;

fn main() -> std::io::Result<()> {
    let link_path = Path::new("link");
    symlink("/origin_does_not_exist/", link_path)?;

    // Use symlink_metadata to get symlink info without following it
    let metadata = fs::symlink_metadata(link_path)?;
    assert!(metadata.is_symlink());
    
    // Using metadata() follows the symlink, so is_symlink() returns false
    let followed_metadata = fs::metadata(link_path)?;
    assert!(!followed_metadata.is_symlink());
    
    Ok(())
}
```

---

#### `file_type()` - Get FileType Struct

```rust
pub fn file_type(&self) -> FileType
```

Returns the file type for this metadata.

**Example:**
```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("foo.txt")?;
    let file_type = metadata.file_type();
    
    println!("Is file: {}", file_type.is_file());
    println!("Is dir: {}", file_type.is_dir());
    println!("Is symlink: {}", file_type.is_symlink());
    Ok(())
}
```

---

## VFS DirectoryEntry Implementation Notes

### Recommended Approach for VFS Metadata Extraction

```rust
use std::fs;
use std::path::Path;

struct DirectoryEntry {
    path: String,
    size: u64,
    modified: Option<u64>, // UNIX timestamp in seconds
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
}

fn get_directory_entry<P: AsRef<Path>>(path: P) -> std::io::Result<DirectoryEntry> {
    let path = path.as_ref();
    
    // Use symlink_metadata to detect symlinks without following them
    let metadata = fs::symlink_metadata(path)?;
    
    // Get modification time as UNIX timestamp
    let modified = metadata.modified().ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    
    Ok(DirectoryEntry {
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        modified,
        is_file: metadata.is_file(),
        is_dir: metadata.is_dir(),
        is_symlink: metadata.is_symlink(),
    })
}
```

### Key Points for Wayland File Manager VFS

1. **Use `symlink_metadata()` instead of `metadata()`** if you need to detect symlinks. `metadata()` follows symlinks, losing the symlink information.

2. **Handle `modified()` errors gracefully** - the method returns `Result<SystemTime>` which may fail on some platforms or filesystems.

3. **File size with `len()`** - Returns `u64` bytes. For directories, this may be platform-specific (often 0 or a small number representing directory entry size).

4. **File type checks are mutually exclusive for real files** - A file cannot be both `is_file()` and `is_dir()` at the same time. However, symlinks can be detected separately with `is_symlink()`.

5. **Cross-platform considerations** - The `modified()` method works on both Unix and Windows, but the underlying system calls differ:
   - Unix: `stat` structure's `mtime` field
   - Windows: `ftLastWriteTime` field

---

## Additional Metadata Methods

### `accessed()` - Last Access Time
```rust
pub fn accessed(&self) -> Result<SystemTime>
```
Returns the last access time (atime). Note: not all platforms update this field automatically.

### `created()` - Creation Time
```rust
pub fn created(&self) -> Result<SystemTime>
```
Returns the creation time (btime on Linux 4.11+, birthtime on other Unix, ftCreationTime on Windows).

### `permissions()` - File Permissions
```rust
pub fn permissions(&self) -> Permissions
```
Returns the permissions of the file.

---

## Official Documentation Link

https://doc.rust-lang.org/std/fs/struct.Metadata.html
