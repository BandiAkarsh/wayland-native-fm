---
source: docs.rs + GitHub tokio-rs/tokio
library: tokio
package: tokio
topic: io-uring-setup
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/tokio/latest/tokio/
---

# Tokio io_uring Setup and Configuration

## Feature Flags

Tokio io_uring support is an **unstable feature** that must be explicitly enabled:

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# .cargo/config.toml (NOT Cargo.toml)
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

### Required Features for io_uring

- `io-uring`: Enables io_uring support (Linux only, requires `tokio_unstable`)
- `rt`: Runtime support
- `fs`: Filesystem operations
- `rt-multi-thread`: Multi-threaded scheduler

## Runtime Builder Configuration

```rust
use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    let rt = Builder::new_multi_thread()
        .enable_io_uring()
        .build()
        .unwrap();
    
    // io_uring is now enabled
}
```

### Configuration Options

| Method | Description |
|--------|-------------|
| `enable_io_uring()` | Enable io_uring driver for async file I/O |
| `uring_setup_sqpoll(idle_ms)` | Enable SQPOLL mode (kernel thread polls submission queue) |
| `uring_queue_depth(depth)` | Set submission queue depth |
| `uring_low_mem_footprint()` | Minimize memory usage |

## SQPOLL Support (NEW in 2026)

Added in PR #7960 - Allows kernel thread to poll the submission queue:

```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .enable_io_uring()
    .uring_setup_sqpoll(1000) // idle timeout in milliseconds
    .build()
    .unwrap();
```

**Requirements:**
- Linux kernel 5.1+
- `CAP_SYS_ADMIN` capability (required before Linux 5.11)

**Note:** First I/O operation will fail if SQPOLL is enabled but not supported by the system.

## Platform Requirements

- **Minimum kernel version**: 5.10 (for `tokio-uring` crate)
- **Supported platforms**: Linux only
- **Architectures**: x86_64, aarch64, and others supported by io-uring crate

## Recent Changes (2025-2026)

1. **PR #7960**: Added SQPOLL support
2. **PR #7961**: Support for batching multiple operations
3. **PR #7907**: AsyncRead implementation for io_uring files
4. **PR #7963**: AsyncBufRead implementation