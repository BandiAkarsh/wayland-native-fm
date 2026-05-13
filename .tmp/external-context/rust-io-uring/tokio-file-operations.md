---
source: tokio github source + docs.rs
library: tokio
package: tokio
topic: file-operations
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/tokio/latest/tokio/fs/
---

# Tokio File Operations with io_uring

## tokio::fs API

Tokio provides async file operations through `tokio::fs` module. With io_uring enabled, these operations use io_uring for better performance.

## Opening Files

### Using OpenOptions

```rust
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open for writing
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.txt")
        .await?;

    // Write data
    file.write_all(b"Hello, io_uring!").await?;
    
    Ok(())
}
```

### Async File Creation

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

let mut file = File::create("newfile.txt").await?;
file.write_all(b"Content").await?;
```

## Reading Files

### Basic Read with AsyncReadExt

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt").await?;
    let mut contents = Vec::new();
    
    file.read_to_end(&mut contents).await?;
    
    println!("Read {} bytes", contents.len());
    Ok(())
}
```

### Read with AsyncBufReadExt

```rust
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

let file = File::open("data.csv").await?;
let mut reader = BufReader::new(file);
let mut line = String::new();

while reader.read_line(&mut line).await? > 0 {
    println!("{}", line.trim());
    line.clear();
}
```

## Writing Files

### Basic Write with AsyncWriteExt

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

let mut file = File::create("output.bin").await?;
file.write_all(&[0x00, 0x01, 0x02, 0x03]).await?;

// Flush to ensure data is written
file.flush().await?;
```

### AsyncWrite Trait Implementation

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

struct DataPacket {
    data: Vec<u8>,
}

impl AsyncWrite for DataPacket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<io::Result<usize>> {
        // Custom write implementation
        todo!()
    }
    
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        todo!()
    }
    
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        todo!()
    }
}
```

## Seeking

### AsyncSeek Implementation

```rust
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncReadExt, SeekFrom};

let mut file = File::open("data.bin").await?;

// Seek to position
file.seek(SeekFrom::Start(100)).await?;

// Read from new position
let mut buf = [0u8; 32];
file.read_exact(&mut buf).await?;

// Seek relative to current position
file.seek(SeekFrom::Current(-10)).await?;

// Seek from end
file.seek(SeekFrom::End(-32)).await?;
```

## File Metadata

```rust
use tokio::fs;
use tokio::io::AsyncReadExt;

let metadata = fs::metadata("file.txt").await?;

println!("Size: {} bytes", metadata.len());
println!("Is file: {}", metadata.is_file());
println!("Is dir: {}", metadata.is_dir());
println!("Modified: {:?}", metadata.modified());
println!("Created: {:?}", metadata.created());
```

## Copy Operations

### Copy File Contents

```rust
use tokio::fs;

let n = fs::copy("source.txt", "dest.txt").await?;
println!("Copied {} bytes", n);
```

### Copy with io_uring (Under the Hood)

```rust
// tokio::fs::copy automatically uses io_uring when enabled
use tokio::fs;

// This will use io_uring read/write operations internally
let n = fs::copy("large_file.bin", "large_copy.bin").await?;
```

## Directory Operations

### Read Directory Contents

```rust
use tokio::fs;

let mut entries = fs::read_dir(".").await?;

while let Some(entry) = entries.next_entry().await? {
    let path = entry.path();
    let metadata = entry.metadata().await?;
    
    if metadata.is_dir() {
        println!("[DIR]  {}", path.display());
    } else {
        println!("[FILE] {} ({} bytes)", path.display(), metadata.len());
    }
}
```

### Create/Remove Directories

```rust
use tokio::fs;

// Create directory
fs::create_dir("/tmp/new_dir").await?;

// Create with parents
fs::create_dir_all("/tmp/nested/dirs").await?;

// Remove empty directory
fs::remove_dir("/tmp/empty_dir").await?;

// Remove directory and contents recursively
fs::remove_dir_all("/tmp/old_dir").await?;
```

## Sync Operations

```rust
use tokio::fs::File;

let file = File::create("data.txt").await?;
file.write_all(b"Content").await?;

// Sync data to disk
file.sync_all().await?;

// Sync only data (not metadata)
file.sync_data().await?;
```

## tokio-uring Crate (Standalone)

For low-level io_uring usage outside Tokio runtime:

```rust
use tokio_uring::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio_uring::start(async {
        let file = File::open("input.txt").await?;
        let buf = vec![0u8; 4096];
        
        let (result, buf) = file.read_at(buf, 0).await;
        let n = result?;
        
        println!("Read {} bytes: {:?}", n, &buf[..n]);
        Ok(())
    })
}
```

## Key Differences: Normal vs io_uring

| Feature | Normal tokio::fs | io_uring |
|---------|-----------------|---------|
| Read/Write | spawn_blocking | Direct submission |
| Buffer ownership | Borrowed during operation | Ownership transferred |
| Close semantics | Synchronous | Asynchronous |
| Batch operations | Sequential | Can batch multiple |

## Recent Implementation Changes (2026)

### PR #7907: AsyncRead for io_uring Files

Added `AsyncRead` trait implementation for files using io_uring:

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

let file = File::open("data.txt").await?;
// Now implements AsyncRead without spawn_blocking
let mut buf = Vec::new();
file.read_to_end(&mut buf).await?;
```