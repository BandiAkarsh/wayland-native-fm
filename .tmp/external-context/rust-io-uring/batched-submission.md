---
source: tokio PR #7961 + GitHub discussions
library: tokio
package: tokio
topic: batched-submission
fetched: 2026-05-02T00:00:00Z
official_docs: https://github.com/tokio-rs/tokio/pull/7961
---

# io_uring Batched Submission

## Overview

Batching allows submitting multiple operations in a single `io_uring_enter` system call, reducing syscall overhead significantly.

## Recent Implementation (PR #7961)

Added in December 2025 - Active development:

```rust
// From tokio source - batch submission support
pub struct Batch<Op, const N: usize> {
    ops: [Op; N],
    count: usize,
}

impl<Op: Cancellable, const N: usize> Batch<Op, N> {
    pub fn new() -> Self { /* ... */ }
    pub fn push(&mut self, op: Op) -> Result<(), BatchFull> { /* ... */ }
    pub async fn submit(self) -> Result<Vec<Op::Output>, OpError> { /* ... */ }
}
```

## Using Batched Operations

### Batch Read Example

```rust
use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    let rt = Builder::new_multi_thread()
        .enable_io_uring()
        .build()
        .unwrap();

    rt.block_on(async {
        let mut batch = tokio::io::uring::Batch::<tokio::io::uring::Read, 16>::new();
        
        // Queue multiple reads
        for (fd, offset) in files.iter().zip(0..) {
            batch.push(tokio::io::uring::Read::at(*fd, buffer.clone(), offset)).unwrap();
        }
        
        // Submit all at once
        let results = batch.submit().await;
        
        for result in results {
            match result {
                Ok(bytes) => println!("Read {} bytes", bytes),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    });
}
```

## Submitter API

```rust
use io_uring::{IoUring, opcode, types};

let mut ring = IoUring::new(64)?;
let sq = ring.submission();

// Build multiple operations
let ops = vec![
    opcode::Read::new(types::Fd(0), buf1.as_ptr(), 4096).build(),
    opcode::Read::new(types::Fd(1), buf2.as_ptr(), 4096).build(),
    opcode::Write::new(types::Fd(2), data.as_ptr(), data.len() as u32).build(),
];

// Push all to submission queue
for op in ops {
    sq.push(&op)?;
}

// Single submit call for all operations
ring.submit()?;
```

## Single-Shot vs Multi-Shot

### Single-Shot Operations

Each submission returns one completion event:

```rust
let read_op = opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), buf.len() as u32)
    .offset(offset)
    .build();
```

### Multi-Shot Operations

Single submission generates multiple completions:

```rust
use io_uring::opcode::ReadMulti;

let read_op = ReadMulti::new(types::Fd(fd), buf.as_mut_ptr(), buf.len() as u32).build();
```

## Linked Operations

Chain operations that must complete together:

```rust
use io_uring::squeue::Flags;

let read_op = opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), 4096).build();
let write_op = opcode::Write::new(types::Fd(out_fd), buf.as_ptr(), 4096).build();

// Link operations - write waits for read
write_op.flags(Flags::IO_LINK.bits());

sq.push(&read_op)?;
sq.push(&write_op)?;
```

## Buffer Selection

Use registered buffers for zero-copy operations:

```rust
use io_uring::opcode::{ProvideBuffers, Read, RemoveBuffers};

// Register buffer group
let register = ProvideBuffers::new(
    &buffers,     // &[&[u8]]
    4096,       // buffer size
    buffers.len() as u32,
    0,          // group ID
    0            // first buffer ID
).build();

// Read into registered buffer
let read_op = Read::new(types::Fd(fd), std::ptr::null_mut(), 4096)
    .buf_group(0)
    .build();
```

## Performance Considerations

### Queue Depth

| Queue Size | Use Case | Pros | Cons |
|-----------|---------|------|------|
| 32 | Low concurrency | Low memory | May bottleneck |
| 64 | Default | Balanced | - |
| 128+ | High throughput | Better batching | Higher memory |

### Batch Size Recommendations

```rust
// Low memory footprint
let ring = IoUring::builder()
    .queue_depth(32)
    .uring_low_mem_footprint()
    .build();

// High throughput
let ring = IoUring::builder()
    .queue_depth(256)
    .build();
```

## Error Handling

```rust
use io_uring::squeue::PushError;

match sq.push(&op) {
    Ok(()) => println!("Added to queue"),
    Err(PushError::QueueFull) => {
        // Need to submit first
        ring.submit()?;
        sq.push(&op)?;
    }
    Err(PushError::Invalid) => {
        eprintln!("Invalid operation");
    }
}
```

## Cancellation

```rust
use io_uring::opcode::AsyncCancel;

let read_op = opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), 4096).build();
let cancel_op = opcode::AsyncCancel::new(types::RegisteredFileFd(read_op.fd()), read_op.user_data())
    .build();

// Cancel the read operation
sq.push(&cancel_op)?;
```

## Best Practices

1. **Batch related operations** - Reduces syscall overhead
2. **Use appropriate queue depth** - Balance memory vs throughput
3. **Register buffers** - Zero-copy for repeated access
4. **Handle queue overflow** - Submit and retry when full
5. **Link dependent operations** - Atomic multi-step operations