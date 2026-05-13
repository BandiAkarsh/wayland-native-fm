---
source: Docs.rs (walkdir 2.5.0)
library: walkdir
package: walkdir
topic: Directory traversal
fetched: 2026-05-02
official_docs: https://docs.rs/walkdir/latest/walkdir/
---

# walkdir - Recursive Directory Traversal (v2.5.0)

## Overview

The `walkdir` crate provides an efficient and cross-platform implementation of recursive directory traversal. It offers fine-grained control over iteration, symlink handling, and resource management.

## Installation

```toml
[dependencies]
walkdir = "2"
```

## Basic Usage

```rust
use walkdir::WalkDir;

for entry in WalkDir::new("foo") {
    println!("{}", entry?.path().display());
}
```

## Key Types

### WalkDir Builder

Creates iterators for recursively walking directories.

### DirEntry

Represents a directory entry with methods:
- `path()` - The path of the entry
- `file_name()` - The filename as `OsStr`
- `file_type()` - File type (file, dir, symlink)
- `metadata()` - Metadata (follows symlinks)
- `symlink_metadata()` - Metadata (doesn't follow symlinks)
- `depth()` - Depth of the entry in the walk

### Error

Wrapper around `std::io::Error` with additional info:
- Loop detection when following symlinks
- Path information about where error occurred

## Configuration Options

### Depth Control

```rust
// Only visit entries at depth 1, 2, or 3
WalkDir::new("foo").min_depth(1).max_depth(3);

// Skip the root directory itself
WalkDir::new("foo").min_depth(1);
```

### Symlink Handling

```rust
// Follow symbolic links (disabled by default)
WalkDir::new("foo").follow_links(true);

// Follow symlinks only at root
WalkDir::new("foo").follow_root_links(true);
```

### Sorting

```rust
// Sort by file name
WalkDir::new("foo").sort_by_file_name();

// Custom sort function
WalkDir::new("foo").sort_by(|a, b| a.file_name().cmp(b.file_name()));

// Sort by key
WalkDir::new("foo").sort_by_key(|e| e.file_name().to_owned());
```

### Contents First

```rust
// Yield contents before directory (useful for deletion)
WalkDir::new("foo").contents_first(true);

// Default: directory before contents
WalkDir::new("foo").contents_first(false);
```

### File Descriptor Limits

```rust
// Limit simultaneous open file descriptors
WalkDir::new("foo").max_open(10);

// Default is reasonably low
```

### File System Boundaries

```rust
// Don't cross filesystem boundaries (Unix/Windows only)
WalkDir::new("foo").same_file_system(true);
```

## Filtering Entries

### Using filter_map

```rust
// Skip errors silently
for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
}
```

### Using filter_entry

```rust
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
```

## Efficient Traversal Patterns

### Pattern 1: Process Files Only

```rust
use walkdir::WalkDir;

for entry in WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
{
    // Process file
}
```

### Pattern 2: Skip Directories

```rust
use walkdir::WalkDir;

for entry in WalkDir::new(".")
    .into_iter()
    .filter_entry(|e| !e.file_type().is_dir())
{
    // Skip descending into directories
}
```

### Pattern 3: Parallel Walking (with rayon)

```rust
use walkdir::WalkDir;
use rayon::prelude::*;

let entries: Vec<_> = WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok())
    .collect();

entries.par_iter().for_each(|entry| {
    // Process in parallel
});
```

### Pattern 4: Early Termination

```rust
use walkdir::WalkDir;

for entry in WalkDir::new("foo") {
    let entry = entry?;
    if entry.path().to_string_lossy().contains("target") {
        break; // Stop walking
    }
}
```

## Error Handling

```rust
use walkdir::WalkDir;

for entry in WalkDir::new("foo") {
    match entry {
        Ok(entry) => {
            println!("{}", entry.path().display());
        }
        Err(e) => {
            println!("Error: {}", e);
            // Error types:
            // - io::Error (permission denied, etc.)
            // - LoopError (symlink loop detected)
        }
    }
}
```

## Performance Tips

1. **Use `filter_map(|e| e.ok())`** - Skip entries that cause errors without stopping
2. **Set `max_open()` appropriately** - Balance between memory and FD usage
3. **Use `contents_first(true)` for deletion** - Process children before parent
4. **Avoid collecting all entries** - Process as you iterate when possible
5. **Use `same_file_system(true)`** - Avoid crossing mount points when not needed

## Comparison with std::fs::read_dir

| Feature | std::fs::read_dir | walkdir |
|---------|-------------------|---------|
| Recursive | Manual implementation | Built-in |
| Symlink handling | Manual | Configurable |
| Depth control | Manual | Built-in |
| Sorting | Manual | Built-in |
| FD management | Manual | Automatic |
| Error handling | Basic | Rich |