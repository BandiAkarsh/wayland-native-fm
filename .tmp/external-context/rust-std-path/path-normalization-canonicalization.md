---
source: Rust std::path Official Docs + Context7
library: rust-lang/rust
package: rust-std
topic: path normalization, canonicalize(), normalize_lexically()
fetched: 2026-05-02T12:00:00Z
official_docs: https://doc.rust-lang.org/std/path/index.html
---

# Rust std::path: Path Normalization & Canonicalization

## Path Normalization (Basic)
Several methods perform **basic path normalization** by disregarding:
- Repeated separators (e.g., `a/b` and `a//b` both yield components `a` and `b`)
- Non-leading `.` components (e.g., `a/./b` → `a/b`)
- Trailing separators (e.g., `/a/b` and `/a/b/` are equivalent)

These methods include:
- `Path::components()`, `Path::iter()`
- `Path::has_root()`
- Comparisons via `PartialEq`, `PartialOrd`, `Ord`
- `Path::join()`, `PathBuf::push()` (disregard trailing slashes)

⚠️ **Important**: Basic normalization does **NOT** resolve `..` components or symlinks.

## Full Normalization with `Path::canonicalize()`
- **Alias**: `fs::canonicalize()`
- **Purpose**: Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
- **Behavior**:
  - Accesses the filesystem (resolves symlinks, verifies existence)
  - Returns `PathBuf`
- **Errors**:
  - Path does not exist
  - A non-final component in the path is not a directory
- **Example**:
  ```rust
  use std::path::{Path, PathBuf};
  
  let path = Path::new("/foo/test/../test/bar.rs");
  assert_eq!(path.canonicalize().unwrap(), PathBuf::from("/foo/test/bar.rs"));
  ```

## Lexical Normalization with `Path::normalize_lexically()` (Experimental: Nightly-only)
- **Feature flag**: `#![feature(normalize_lexically)]`
- **Purpose**: Normalize a path including `..` *without* traversing the filesystem.
- **Behavior**:
  - Resolves `..` lexically (e.g., `a/b/../c` → `a/c`)
  - Does **NOT** resolve symlinks
  - Returns `Result<PathBuf, NormalizeError>`
- **Error**: Returns `NormalizeError` if normalization would leave leading `..` components.
- **Warning**: Lexical `..` resolution can change path meaning if intermediate components are symlinks (since symlinks are not followed).
- **Alternative**: Use `path::absolute()` to preserve `..` without filesystem access, or `Path::canonicalize()` to resolve via filesystem.

## Key Differences
| Method | Resolves `..` | Resolves Symlinks | Filesystem Access |
|---------|---------------|-------------------|-------------------|
| Basic normalization (components/join/etc) | ❌ | ❌ | ❌ |
| `normalize_lexically()` | ✅ (lexical) | ❌ | ❌ |
| `canonicalize()` | ✅ (full) | ✅ | ✅ |
| `path::absolute()` | ❌ | ❌ | ❌ |
