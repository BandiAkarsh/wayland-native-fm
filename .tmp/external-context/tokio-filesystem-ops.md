---
source: Context7 API
library: Tokio
package: tokio
topic: async filesystem operations (read_dir, metadata, symlink handling)
fetched: 2026-05-02T12:00:00Z
official_docs: https://docs.rs/tokio/latest/tokio/fs/
---

# Tokio Async Filesystem Operations

## Key Functions for VfsBackend
All operations use `tokio::fs` module, with blocking calls offloaded to a thread pool via `spawn_blocking`.

### 1. read_dir (List Directory Entries)
Asynchronous version of `std::fs::read_dir`, returns a stream of `DirEntry` items.
```rust
use tokio::fs;
use std::path::Path;

pub async fn list_directory(path: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(path).await?;
    let mut result = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        result.push(entry);
    }
    Ok(result)
}

// DirEntry provides path(), file_type(), metadata() methods
```

### 2. metadata (File Metadata, Follows Symlinks)
Queries metadata for a path, following symlinks (uses `std::fs::metadata` internally).
```rust
use tokio::fs;
use std::path::Path;

pub async fn get_metadata(path: &Path) -> io::Result<std::fs::Metadata> {
    fs::metadata(path).await
}
```

### 3. symlink_metadata (Symlink-Aware Metadata)
Queries metadata without following symlinks (equivalent to `std::fs::symlink_metadata`).
```rust
use tokio::fs;
use std::path::Path;

pub async fn get_symlink_metadata(path: &Path) -> io::Result<std::fs::Metadata> {
    fs::symlink_metadata(path).await
}
```

## Implementation Notes for VfsBackend
- All `tokio::fs` functions return `impl Future` and must be awaited.
- For streaming large directories, use `ReadDir` stream directly instead of collecting all entries.
- Symlink handling: Use `symlink_metadata` to detect symlinks (check `file_type().is_symlink()`), then `metadata` to follow them if needed.

## Common Pitfalls
1. **Blocking operations**: `tokio::fs` uses `spawn_blocking` internally, but avoid wrapping synchronous fs calls manually.
2. **Symlink confusion**: `metadata` follows symlinks, `symlink_metadata` does not - choose based on use case.
3. **Error handling**: All operations return `io::Result` - propagate errors with `?` in async contexts.
