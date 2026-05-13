---
source: docs.rs io-uring crate
library: io-uring
package: io-uring
topic: core-api
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/io-uring/latest/io_uring/
---

# io_uring Core API Reference

## Creating an IoUring Instance

```rust
use io_uring::IoUring;

fn main() -> io_uring::Result<()> {
    // Basic creation with default settings
    let mut ring = IoUring::new(32)?; // entries should be power of 2
    
    // Advanced creation with Builder
    let ring = IoUring::builder()
        .queue_depth(64)
        .build();
    
    Ok(())
}
```

### IoUring Struct Methods

| Method | Description |
|--------|-------------|
| `new(entries: u32)` | Create with default configuration |
| `builder()` | Create Builder for customization |
| `submit()` | Initiate asynchronous I/O |
| `submit_and_wait(want: usize)` | Submit and wait for completions |
| `submitter()` | Get Submitter interface |
| `split()` | Get sq, cq, and submitter |
| `submission()` | Get submission queue |
| `completion()` | Get completion queue |
| `params()` | Get construction parameters |

## Submission Queue Operations

### Entry Types

```rust
use io_uring::squeue::Entry;      // 64-byte SQE
use io_uring::squeue::Entry128;   // 128-byte SQE (kernel 5.19+)
```

### Building Submission Entries

```rust
use io_uring::{IoUring, opcode, types};

let mut ring = IoUring::new(32)?;
let mut sq = ring.submission();

// Build a read operation
let read_op = opcode::Read::new(
    types::Fd(3),           // file descriptor
    buffer.as_mut_ptr(),    // buffer pointer
    4096                    // read length
)
.offset(0)                  // file offset
.build();

// Push to submission queue
sq.push(&read_op)?;
```

## File Operations

### Open File

```rust
use io_uring::opcode::OpenAt;
use io_uring::types::AtFlags;

let open_op = OpenAt::new(
    AtFlags::AT_FDCWD,
    path.as_ptr()
)
.flags(libc::O_RDONLY)
.mode(0)
.build();
```

### Read File

```rust
use io_uring::opcode::Read;

let read_op = Read::new(
    types::Fd(fd),
    buffer.as_mut_ptr(),
    buffer.len() as u32
)
.offset(file_offset)
.build();
```

### Write File

```rust
use io_uring::opcode::Write;

let write_op = Write::new(
    types::Fd(fd),
    data.as_ptr(),
    data.len() as u32
)
.offset(file_offset)
.build();
```

### Close File Descriptor

```rust
use io_uring::opcode::Close;

let close_op = Close::new(
    types::Fd(fd)
).build();
```

## Completion Queue Processing

```rust
use io_uring::cqueue::Entry;

let mut ring = IoUring::new(32)?;

// Submit operations
ring.submit()?;

// Process completions
let cq = ring.completion();
while let Some(cqe) = cq.next() {
    match cqe.result() {
        Ok(bytes) => println!("Read {} bytes", bytes),
        Err(e) => eprintln!("Error: {:?}", io::Error::from_raw_os_error(-e)),
    }
}
```

### CQE Result Handling

```rust
fn handle_completion(cqe: cqueue::Entry) {
    // Positive = bytes transferred
    // Negative = error code (negated)
    // Zero = no bytes transferred
    
    let result = cqe.result();
    if result >= 0 {
        println!("Success: {} bytes", result);
    } else {
        let err = -result;
        eprintln!("Error code: {}", err);
    }
}
```

## Register Operations

### Register Buffers

```rust
use io_uring::opcode::ProvideBuffers;
use io_uring::types::BufferNamespace;

let buf_register = ProvideBuffers::new(
    buffers,           // slice of byte slices
    1024,             // buffer size
    buffers.len(),     // number of buffers
    0,                // buffer group ID
    0                  // starting buffer ID
).build();
```

### Unregister Buffers

```rust
use io_uring::opcode::RemoveBuffers;

let unregister = RemoveBuffers::new(buffers.len() as u32, 0).build();
```

## Flags

### Submission Queue Flags

```rust
use io_uring::squeue::Flags;

let flags = Flags::empty()
    | Flags::IO_LINK        // link with next operation
    | Flags::IO_CQ_SEM    // CQ wait on wait sem
    | Flags::SUBMIT_DEADLINE;
```

## Supported Operations (OpCodes)

| Category | Operations |
|----------|----------|
| **File I/O** | Read, Write, ReadFixed, WriteFixed, Readv, Writev, SyncFileRange |
| **File Meta** | OpenAt, OpenAt2, Close, Fsync, Fallocate, Ftruncate, Statx |
| **Directory** | MkDirAt, LinkAt, SymlinkAt, UnlinkAt, RenameAt |
| **Socket** | Socket, Bind, Listen, Accept, Connect, Send, Recv, Shutdown |
| **Memory** | Madvise, Fadvise |
| **Async** | Timeout, Cancel, FutexWait, FutexWake |