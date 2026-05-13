---
source: Context7 API
library: Tokio
package: tokio
topic: async trait patterns (AsyncRead, AsyncWrite) for VfsBackend
fetched: 2026-05-02T12:00:00Z
official_docs: https://docs.rs/tokio/latest/tokio/
---

# Tokio Async Trait Patterns for VfsBackend Implementation

## Key Traits for Async I/O
Tokio provides `AsyncRead` and `AsyncWrite` traits (analogous to `std::io::Read`/`Write`) for asynchronous byte operations, critical for VfsBackend abstraction.

### AsyncRead Trait Definition
```rust
use tokio::io::{AsyncRead, ReadBuf};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait AsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>>;
}

// Common implementations: Box<T>, &mut T, Pin<P>, &[u8], io::Cursor<T>
```

### AsyncWrite Trait (Relevant for Write Operations)
```rust
pub trait AsyncWrite {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>>;

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>>;

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>>;
}
```

## VfsBackend Async Trait Pattern
For a VfsBackend trait, use async trait syntax (requires Rust 1.75+ or `async_trait` crate):
```rust
use tokio::io::{AsyncRead, AsyncWrite};
use std::path::Path;

// Async trait for VFS operations (using async_trait for compatibility)
#[async_trait::async_trait]
pub trait VfsBackend {
    async fn read_dir(&self, path: &Path) -> io::Result<Vec<DirEntry>>;
    async fn metadata(&self, path: &Path) -> io::Result<Metadata>;
    async fn symlink_metadata(&self, path: &Path) -> io::Result<Metadata>;
    fn read_file(&self, path: &Path) -> impl AsyncRead; // Returns async reader
}
```

## Common Pitfalls
1. **Async trait implementation**: Use `async_trait` crate if targeting Rust < 1.75, or native async fn in traits (Rust 1.75+).
2. **Waker notifications**: `AsyncRead`/`AsyncWrite` poll methods must schedule task wakeup via `cx.waker()` when returning `Poll::Pending`.
3. **Unpin requirements**: Many Tokio types require `Unpin` for simple usage; use `Box::pin` or `Pin` wrappers for !Unpin types.
