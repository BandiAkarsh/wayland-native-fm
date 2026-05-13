---
source: Context7 API + docs.rs + Rust documentation
library: lru (lru-rs)
package: lru
topic: Thread safety considerations and directory caching use case
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/lru/latest/lru/struct.LruCache.html
---

# LRU Cache: Thread Safety and Directory Caching

## Thread Safety Considerations

### `LruCache` is NOT Thread-Safe by Default

The `LruCache` struct does NOT implement interior mutability. All mutating methods require `&mut self`, which means:

- You **cannot** share `LruCache` directly across threads using `Arc`
- You **cannot** call `put()`, `get()`, `pop()`, etc. from multiple threads simultaneously

### Thread Safety Trait Implementations

According to the docs.rs documentation, `LruCache` has the following trait implementations:

```rust
impl<K, V, S> Send for LruCache<K, V, S>
impl<K, V, S> Sync for LruCache<K, V, S>
```

**However**, these implementations only mean:
- `Send`: The cache can be moved between threads (but only used by one thread at a time)
- `Sync`: The cache can be shared between threads via `&LruCache` (but you can't mutate through a shared reference)

Since all mutating operations require `&mut self`, you still need synchronization primitives for concurrent access.

### Making LRU Cache Thread-Safe

To use `LruCache` in a multi-threaded environment, wrap it in synchronization primitives:

#### Option 1: `Arc<Mutex<LruCache<...>>>` (Recommended for Write-Heavy Workloads)

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let cache: Arc<Mutex<LruCache<String, Vec<String>>>> = 
        Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

    let mut handles = vec![];

    for i in 0..5 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut cache = cache_clone.lock().unwrap();
            let key = format!("/path/dir_{}", i);
            cache.put(key, vec!["file1.txt".to_string(), "file2.txt".to_string()]);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let cache = cache.lock().unwrap();
    assert_eq!(cache.len(), 5);
}
```

#### Option 2: `Arc<RwLock<LruCache<...>>>` (Recommended for Read-Heavy Workloads)

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let cache: Arc<RwLock<LruCache<String, Vec<String>>>> = 
        Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

    // Writer thread
    let cache_write = Arc::clone(&cache);
    let writer = thread::spawn(move || {
        let mut cache = cache_write.write().unwrap();
        cache.put("/home/user/docs".to_string(), vec!["a.txt".to_string()]);
    });

    // Reader threads (can run concurrently)
    let mut readers = vec![];
    for _ in 0..3 {
        let cache_read = Arc::clone(&cache);
        let reader = thread::spawn(move || {
            let cache = cache_read.read().unwrap();
            let _ = cache.peek(&"/home/user/docs".to_string());
        });
        readers.push(reader);
    }

    writer.join().unwrap();
    for reader in readers {
        reader.join().unwrap();
    }
}
```

#### Option 3: Channel-Based Cache (For Async/Tokio)

For async Rust with Tokio, consider using a channel to serialize cache access:

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use tokio::sync::mpsc;

enum CacheCommand {
    Get(String, mpsc::Sender<Option<Vec<String>>>),
    Put(String, Vec<String>),
}

async fn cache_worker(mut rx: mpsc::Receiver<CacheCommand>) {
    let mut cache: LruCache<String, Vec<String>> = 
        LruCache::new(NonZeroUsize::new(100).unwrap());
    
    while let Some(cmd) = rx.recv().await {
        match cmd {
            CacheCommand::Get(key, resp) => {
                let _ = resp.send(cache.get(&key).cloned());
            }
            CacheCommand::Put(key, value) => {
                cache.put(key, value);
            }
        }
    }
}
```

## Directory Caching Example

Here's a complete example for caching directory contents in a file manager:

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, Duration};

#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
}

#[derive(Clone)]
pub struct CachedDir {
    pub entries: Vec<DirEntry>,
    pub cached_at: SystemTime,
}

pub struct DirectoryCache {
    cache: LruCache<PathBuf, CachedDir>,
    ttl: Duration,
}

impl DirectoryCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            ttl,
        }
    }

    /// Get directory contents from cache or scan if not cached/expired
    pub fn get_or_scan(&mut self, path: &Path) -> std::io::Result<Vec<DirEntry>> {
        let now = SystemTime::now();
        
        // Check cache first (use peek to avoid affecting LRU if we might rescan)
        if let Some(cached) = self.cache.peek(path) {
            if now.duration_since(cached.cached_at).unwrap_or(self.ttl) < self.ttl {
                return Ok(cached.entries.clone());
            }
        }

        // Scan directory
        let entries = self.scan_directory(path)?;
        
        // Cache the result
        self.cache.put(
            path.to_path_buf(),
            CachedDir {
                entries: entries.clone(),
                cached_at: now,
            }
        );

        Ok(entries)
    }

    /// Force rescan and update cache
    pub fn rescan(&mut self, path: &Path) -> std::io::Result<Vec<DirEntry>> {
        let entries = self.scan_directory(path)?;
        
        self.cache.put(
            path.to_path_buf(),
            CachedDir {
                entries: entries.clone(),
                cached_at: SystemTime::now(),
            }
        );

        Ok(entries)
    }

    /// Invalidate a specific directory in cache
    pub fn invalidate(&mut self, path: &Path) {
        self.cache.pop(path);
    }

    /// Clear all cached directories
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.cache.cap().get())
    }

    fn scan_directory(&self, path: &Path) -> std::io::Result<Vec<DirEntry>> {
        let mut entries = Vec::new();
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
            });
        }

        Ok(entries)
    }
}

// Usage example
fn main() -> std::io::Result<()> {
    let mut cache = DirectoryCache::new(50, Duration::from_secs(30));

    // First call - scans directory
    let entries1 = cache.get_or_scan(Path::new("/home/user"))?;
    println!("Scanned {} entries", entries1.len());

    // Second call - returns from cache (if within TTL)
    let entries2 = cache.get_or_scan(Path::new("/home/user"))?;
    println!("Got {} entries from cache", entries2.len());

    // Check cache stats
    let (len, cap) = cache.stats();
    println!("Cache: {}/{} entries", len, cap);

    Ok(())
}
```

### Thread-Safe Version for Directory Cache

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

// Wrap the cache in Arc<RwLock<...>> for thread-safe access
pub struct SharedDirectoryCache {
    inner: Arc<RwLock<DirectoryCache>>,
}

impl SharedDirectoryCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DirectoryCache::new(capacity, ttl))),
        }
    }

    pub fn get_or_scan(&self, path: &Path) -> std::io::Result<Vec<DirEntry>> {
        // For read-heavy workloads, we might want to check cache with read lock first
        {
            let cache = self.inner.read().unwrap();
            if let Some(cached) = cache.cache.peek(path) {
                let now = SystemTime::now();
                if now.duration_since(cached.cached_at).unwrap_or(cache.ttl) < cache.ttl {
                    return Ok(cached.entries.clone());
                }
            }
        }

        // Need to scan - acquire write lock
        let mut cache = self.inner.write().unwrap();
        cache.get_or_scan(path)
    }
}
```

## Key Takeaways

1. **`LruCache` is not thread-safe by default** - wrap in `Mutex` or `RwLock` for concurrent access
2. **Use `peek()` for read-only cache checks** without affecting eviction order
3. **Consider TTL (Time To Live)** for directory caching to handle external changes
4. **Choose capacity based on expected working set** - typical file managers might cache 50-200 directories
5. **Use `Arc<Mutex<...>>` for write-heavy or `Arc<RwLock<...>>` for read-heavy workloads**
