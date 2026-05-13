---
source: Official Rust Documentation (doc.rust-lang.org)
library: Rust Standard Library
package: std::path
topic: Path manipulation
fetched: 2026-05-02
official_docs: https://doc.rust-lang.org/std/path/index.html
---

# std::path - Path Manipulation (Rust 1.95.0)

## Overview

The `std::path` module provides types and functions for working with file paths in a platform-independent way.

## Key Types

### Path (Unsized)

A slice of a path (akin to `str`). Must be used behind a pointer like `&Path` or `Box<Path>`.

### PathBuf (Owned)

An owned, growable string that holds a path. Similar to `String` for paths.

## Path Creation

```rust
// From string slice (cost-free conversion)
let path = Path::new("foo.txt");

// From String or PathBuf
let path = PathBuf::from("foo.txt");

// From owned path
let owned = path.to_path_buf();
```

## Key Methods on Path

### Component Extraction

| Method | Description |
|--------|-------------|
| `file_name()` | Final component (filename or directory name) |
| `file_stem()` | Filename without extension |
| `file_prefix()` | Filename before first `.` |
| `extension()` | File extension (after last `.`) |
| `parent()` | Path without final component |
| `components()` | Iterator over path components |

```rust
use std::path::Path;

let path = Path::new("foo.tar.gz");

assert_eq!(path.file_name(), Some(OsStr::new("foo.tar.gz")));
assert_eq!(path.file_stem(), Some(OsStr::new("foo.tar")));  // Before last .
assert_eq!(path.file_prefix(), Some(OsStr::new("foo")));    // Before first .
assert_eq!(path.extension(), Some(OsStr::new("gz")));
```

### Path Inspection

| Method | Description |
|--------|-------------|
| `is_absolute()` | Returns true if path is absolute |
| `is_relative()` | Returns true if path is relative |
| `is_file()` | Returns true if path is a file |
| `is_dir()` | Returns true if path is a directory |
| `is_symlink()` | Returns true if path is a symlink |
| `exists()` | Returns true if path exists |
| `try_exists()` | Check existence without errors |

### Path Manipulation

```rust
// Join paths
let path = Path::new("/etc").join("passwd");  // "/etc/passwd"

// Replace components
let path = Path::new("/tmp/foo.png").with_file_name("bar");
let path = Path::new("foo.rs").with_extension("txt");

// Add extension (preserves existing)
let path = Path::new("foo.tar.gz").with_added_extension("xz");
// Result: "foo.tar.gz.xz"

// Strip prefix
let path = Path::new("/test/haha/foo.txt");
assert_eq!(path.strip_prefix("/test"), Ok(Path::new("haha/foo.txt")));

// Ancestors (all parent paths)
for ancestor in Path::new("/foo/bar").ancestors() {
    println!("{}", ancestor.display());
}
// "/foo/bar", "/foo", "/"
```

### String Conversion

```rust
// To string (may fail for non-UTF8)
path.to_str()  // Option<&str>

// To string with lossless conversion
path.to_string_lossy()  // Cow<str>

// To owned PathBuf
path.to_path_buf()
```

## PathBuf Specific Methods

```rust
let mut path = PathBuf::from("foo");

// Push components
path.push("bar");        // "foo/bar"
path.push("file.txt");   // "foo/bar/file.txt"

// Set file name
path.set_file_name("new.txt");

// Set extension
path.set_extension("md");

// Clear path
path.clear();
```

## Component Enum

The `Component` enum represents path components:
- `RootDir` - The root directory (`/` on Unix)
- `CurDir` - Current directory (`.`)
- `ParentDir` - Parent directory (`..`)
- `Normal` - Regular component (filename/directory name)
- `Prefix` - Windows-specific prefix (e.g., `C:`)

```rust
use std::path::{Path, Component};

let mut components = Path::new("/tmp/foo.txt").components();
assert_eq!(components.next(), Some(Component::RootDir));
assert_eq!(components.next(), Some(Component::Normal("tmp")));
assert_eq!(components.next(), Some(Component::Normal("foo.txt")));
```

## Best Practices

1. **Use `Path` for function parameters** - More flexible, accepts both `&str` and `PathBuf`
2. **Use `PathBuf` when you need ownership** - For building paths dynamically
3. **Prefer `to_path_buf()` over `to_string()`** - Handles non-UTF8 paths correctly
4. **Use `join()` for path construction** - Handles separators correctly across platforms
5. **Check `is_dir()`/`is_file()` before operations** - Avoid errors on wrong types