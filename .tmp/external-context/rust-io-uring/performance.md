---
source: tokio docs + Linux kernel io_uring + liburing
library: tokio
package: tokio
topic: performance
fetched: 2026-05-02T00:00:00Z
official_docs: https://tokio.rs/blog/2023/tokio-uring
---

# io_uring Performance Considerations

## Overview

io_uring provides significant performance advantages over traditional epoll-based I/O by minimizing syscall overhead and enabling true async operations without thread pools.

## Performance Benefits

### Comparison with spawn_blocking

| Metric | spawn_blocking | io_uring |
|--------|-------------|---------|
| Latency | Higher (thread context switch) | Lower (kernel directly) |
| Throughput | Moderate | High |
| Memory | Thread stack overhead | Minimal |
| Concurrency | Limited by thread count | High (queue depth) |

## Zero-Copy Operations

### Registered Buffers

Register buffers once, reuse them multiple times:

```rust
use io_uring::{IoUring, opcode, types, squeue::Entry};

fn main() -> io_uring::Result<()> {
    let mut ring = IoUring::builder()
        .queue_depth(64)
        .build();
    
    let submitter = ring.submitter();
    
    // Register buffers once
    let buffers: Vec<Vec<u8>> = (0..32)
        .map(|_| vec![0u8; 4096])
        .collect();
    
    submitter.register_buffers(&buffers)?;
    
    // Now reads use registered buffers - zero copy
    let mut sq = ring.submission();
    let read_op = opcode::Read::new(types::Fd(3), std::ptr::null_mut(), 4096)
        .buf_group(0)  // Use registered buffer group
        .build();
    
    sq.push(&read_op)?;
    ring.submit()?;
    
    Ok(())
}
```

### Registered Files

```rust
use io_uring::opcode::FixedFd;

let install_op = opcode::FixedFd::new(types::Fd(3), types::FixedFd(0)).build();
```

## SQPOLL Mode (Kernel 5.1+)

Kernel thread polls submission queue - eliminates userspace/kernel transitions:

```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .enable_io_uring()
    .uring_setup_sqpoll(1000) // idle timeout in ms
    .build();
```

### SQPOLL Trade-offs

**Pros:**
- Minimal latency for high-throughput workloads
- No syscall overhead for submission

**Cons:**
- Higher kernel CPU usage
- Requires CAP_SYS_ADMIN (kernel < 5.11)
- May increase power consumption

## Queue Configuration

### High Throughput Configuration

```rust
let ring = IoUring::builder()
    .queue_depth(256)          // More concurrent operations
    .cq_size(256)            // Completion queue depth
    .build();
```

### Low Memory Footprint

```rust
let ring = IoUring::builder()
    .queue_depth(32)           // Minimum for batch efficiency
    .uring_low_mem_footprint() // Minimize kernel memory
    .build();
```

## Memory Considerations

### RLIMIT_MEMLOCK

io_uring accounts memory under `rlimit memlocked`:

```bash
# Check current limit
ulimit -l

# Increase (if root)
ulimit -l 65536
```

### Kernel Memory (5.11+)

Modern kernels are less dependent on RLIMIT_MEMLOCK:

```rust
// Only affects registered buffers
submitter.register_buffers(&buffers)?;
```

## Batch Optimization

### Operation Batching

```rust
// Build batch of operations
let mut batch = Vec::new();

for i in 0..100 {
    let op = opcode::Read::new(types::Fd(fd), bufs[i].as_mut_ptr(), 4096)
        .offset(i * 4096)
        .build();
    batch.push(op);
}

// Single syscall for all
for op in batch {
    sq.push(&op)?;
}
ring.submit()?;  // One syscall, 100 operations
```

### Scatter-Gather I/O

```rust
use io_uring::opcode::Readv;

let iovecs: Vec<libc::iovec> = bufs.iter()
    .map(|buf| libc::iovec {
        iov_base: buf.as_ptr() as *mut _,
        iov_len: buf.len(),
    })
    .collect();

let read_op = Readv::new(types::Fd(fd), iovecs.as_ptr(), iovecs.len() as u32)
    .build();
```

## Latency Optimization

### Polling Mode

```rust
use io_uring::Builder;

let ring = Builder::new(32)?
    .wait销售额(1)  // No sleep, poll completions
    .build();
```

### I/O Priority

```rust
use io_uring::squeue::Flags;

let op = opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), 4096)
    .build();

let flags = Flags::from_bits_retain(0)
    | Flags::IO_URING_F_SPEED_LIMIT;
```

## Best Practices Summary

### DO

1. **Use registered buffers** for repeated access patterns
2. **Batch operations** when possible
3. **Configure appropriate queue depth** for workload
4. **Use linked operations** for atomic multi-step tasks
5. **Enable SQPOLL** for ultra-low latency (high throughput apps)
6. **Check kernel version** - newer kernels have better io_uring support

### DON'T

1. **Don't use too small queue depth** - limits batching
2. **Don't register too many buffers** - increases memory pressure
3. **Don't mix SQPOLL with low throughput workloads** - wastes CPU
4. **Don't ignore errors** - can indicate resource exhaustion

## Benchmarking

```rust
use std::time::Instant;

fn benchmark<F>(mut f: F) -> std::time::Duration
where
    F: FnMut(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

// Example: 1M reads
let duration = benchmark(|| {
    for i in 0..1_000_000 {
        // read operation
    }
});

println!("{} reads/second", 1_000_000.0 / duration.as_secs_f64());
```

## Recent Performance Improvements (2025-2026)

1. **Batching support** (PR #7961) - Reduced syscall overhead
2. **SQPOLL integration** (PR #7960) - Kernel-level polling
3. **AsyncRead for files** (PR #7907) - Eliminated spawn_blocking overhead