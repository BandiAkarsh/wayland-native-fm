---
source: tokio source + liburing examples
library: tokio
package: tokio
topic: examples
fetched: 2026-05-02T00:00:00Z
official_docs: https://github.com/tokio-rs/tokio/tree/master/examples
---

# io_uring Code Examples

## Basic File Read/Write

### Simple Read

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("data.txt").await?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).await?;
    println!("{}", contents);
    
    Ok(())
}
```

### Simple Write

```rust
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("output.txt")
        .await?;
    
    file.write_all(b"Hello, io_uring!\n").await?;
    file.flush().await?;
    
    Ok(())
}
```

## Low-Level io_uring Usage

### Using io-uring Crate Directly

```rust
use io_uring::{IoUring, opcode, types};

fn main() -> io_uring::Result<()> {
    let mut ring = IoUring::new(32)?;
    let mut sq = ring.submission();
    
    // Read from file
    let fd = std::fs::File::open("input.txt")?
        .into_raw_fd();
    
    let buf = vec![0u8; 4096];
    let read_op = opcode::Read::new(types::Fd(fd), buf.as_ptr() as _, 4096)
        .offset(0)
        .build();
    
    sq.push(&read_op)?;
    ring.submit()?;
    
    // Wait for completion
    ring.submit_and_wait(1)?;
    
    let cq = ring.completion();
    if let Some(cqe) = cq.next() {
        match cqe.result() {
            Ok(n) => println!("Read {} bytes", n),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

## tokio-uring Standalone

```rust
use tokio_uring::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio_uring::start(async {
        // Open file
        let file = File::open("data.bin").await?;
        
        // Prepare buffer
        let buf = vec![0u8; 8192];
        
        // Read at offset 0
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res?;
        
        println!("Read {} bytes: {:?}", n, &buf[..n]);
        
        // Write to another file
        let mut output = File::create("copy.bin").await?;
        let (_, _) = output.write_at(buf, 0).await;
        
        Ok(())
    })
}
```

## Concurrent File Operations

### Parallel Reads

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    let mut join_set = JoinSet::new();
    
    for (i, path) in files.iter().enumerate() {
        join_set.spawn(async move {
            let mut file = File::open(path).await?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await?;
            (i, contents)
        });
    }
    
    let mut results = Vec::new();
    while let Some(res) = join_set.join_next().await {
        results.push(res?);
    }
    
    for (i, data) in results {
        println!("File {}: {} bytes", i, data.len());
    }
    
    Ok(())
}
```

## Batched Operations

```rust
use io_uring::{IoUring, opcode, types};

fn batch_reads(files: &[(i32, Vec<u8>)]) -> io_uring::Result<Vec<usize>> {
    let mut ring = IoUring::builder().queue_depth(32).build();
    let mut sq = ring.submission();
    
    // Queue all reads
    for (fd, buf) in files {
        let op = opcode::Read::new(
            types::Fd(*fd),
            buf.as_ptr() as _,
            buf.len() as u32
        ).build();
        sq.push(&op)?;
    }
    
    // Single submit
    ring.submit()?;
    
    // Collect results
    let cq = ring.completion();
    let mut results = Vec::new();
    
    while let Some(cqe) = cq.next() {
        results.push(cqe.result()? as usize);
    }
    
    Ok(results)
}
```

## Registered Buffers

```rust
use io_uring::{IoUring, opcode, types};

fn registered_buffer_read(fd: i32) -> io_uring::Result<Vec<u8>> {
    let mut ring = IoUring::builder().queue_depth(32).build();
    let submitter = ring.submitter();
    
    // Create and register buffers
    let buffers: Vec<Vec<u8>> = vec![vec![0u8; 4096]; 32];
    submitter.register_buffers(&buffers)?;
    
    let mut sq = ring.submission();
    
    // Read using registered buffer (index 0)
    let read_op = opcode::Read::new(types::Fd(fd), std::ptr::null_mut(), 4096)
        .buf_group(0)  // Use buffer group 0
        .build();
    
    sq.push(&read_op)?;
    ring.submit_and_wait(1)?;
    
    // Get buffer back via completion data
    let cq = ring.completion();
    let cqe = cq.next().unwrap();
    
    // Extract registered buffer index from user_data
    let buf_idx = cqe.user_data() as usize;
    Ok(buffers[buf_idx].clone())
}
```

## Copy with Progress

```rust
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, SeekFrom};

async fn copy_with_progress(
    src: &str,
    dst: &str,
    chunk_size: usize,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut src_file = File::open(src).await?;
    let mut dst_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(dst)
        .await?;
    
    let mut total = 0u64;
    let mut chunk = vec![0u8; chunk_size];
    
    loop {
        let n = src_file.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        
        dst_file.write_all(&chunk[..n]).await?;
        total += n as u64;
        
        // Progress logging
        eprintln!("Copied {} bytes...", total);
    }
    
    Ok(total)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = copy_with_progress("large_file.bin", "copy.bin", 1024 * 1024).await?;
    println!("Total copied: {} bytes", bytes);
    Ok(())
}
```

## File Watcher Pattern

```rust
use tokio::fs;
use tokio::time::{interval, Duration};

async fn watch_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = interval(Duration::from_secs(1));
    
    let initial_entries = fs::read_dir(path).await?
        .collect::<Result<Vec<_>, _>>().await?;
    let mut last_modified = std::collections::HashMap::new();
    
    for entry in initial_entries {
        let meta = entry.metadata().await?;
        last_modified.insert(entry.file_name(), meta.modified()?.clone());
    }
    
    loop {
        interval.tick().await;
        
        let current = fs::read_dir(path).await?
            .collect::<Result<Vec<_>, _>>().await?;
        
        for entry in current {
            let name = entry.file_name();
            let meta = entry.metadata().await?;
            
            if let Some(last) = last_modified.get(&name) {
                if last != meta.modified()? {
                    println!("Changed: {:?}", name);
                }
            }
            last_modified.insert(name, meta.modified()?.clone());
        }
    }
}
```

## Linked Operations (Atomic)

```rust
use io_uring::{IoUring, opcode, types, squeue::Flags};

fn atomic_read_modify_write(
    fd: i32,
    data: &[u8],
) -> io_uring::Result<()> {
    let mut ring = IoUring::new(32)?;
    let mut sq = ring.submission();
    
    let read_buf = vec![0u8; 4096];
    let write_buf = data.to_vec();
    
    // Read operation
    let read_op = opcode::Read::new(types::Fd(fd), read_buf.as_ptr() as _, 4096)
        .offset(0)
        .build();
    
    // Write operation (linked to read)
    let write_op = opcode::Write::new(types::Fd(fd), write_buf.as_ptr() as _, write_buf.len() as u32)
        .offset(0)
        .build();
    
    // Link operations - write waits for read completion
    // (Requires manual flag setting in io-uring crate)
    
    sq.push(&read_op)?;
    sq.push(&write_op)?;
    
    ring.submit_and_wait(2)?;
    
    Ok(())
}
```

## Error Handling Best Practices

```rust
use io_uring::{IoUring, opcode, types};

fn read_with_retry(fd: i32, buf: &mut [u8], offset: u64) -> io_uring::Result<usize> {
    let mut ring = IoUring::new(32)?;
    let mut sq = ring.submission();
    
    let read_op = opcode::Read::new(types::Fd(fd), buf.as_ptr() as _, buf.len() as u32)
        .offset(offset)
        .build();
    
    sq.push(&read_op)?;
    ring.submit_and_wait(1)?;
    
    let cq = ring.completion();
    let cqe = cq.next().unwrap();
    
    match cqe.result() {
        Ok(n) => Ok(n as usize),
        Err(e) if e == libc::EAGAIN => {
            // Retry on EAGAIN
            read_with_retry(fd, buf, offset)
        }
        Err(e) => {
            Err(io_uring::Error::new(io_uring::ErrorKind::Other, -e))
        }
    }
}
```