---
source: Context7 API + docs.rs
library: lru (lru-rs)
package: lru
topic: Basic LRU cache operations (create, put, get, contains, pop, clear)
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/lru/latest/lru/struct.LruCache.html
---

# LRU Cache Basic Operations

## Creating an LRU Cache

The `lru` crate provides an LRU (Least Recently Used) cache implementation with O(1) operations for `put`, `get`, `get_mut`, and `pop`.

### Basic Creation with Fixed Capacity

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    // Create a cache that holds at most 2 items
    let mut cache: LruCache<&str, i32> = LruCache::new(NonZeroUsize::new(2).unwrap());

    assert_eq!(cache.len(), 0);
    assert_eq!(cache.cap().get(), 2);
    assert!(cache.is_empty());
}
```

### Unbounded Cache (No Automatic Eviction)

```rust
use lru::LruCache;

fn main() {
    // Create a cache that never automatically evicts items
    let mut cache: LruCache<isize, &str> = LruCache::unbounded();
}
```

### With Custom Hasher

```rust
use lru::{LruCache, DefaultHasher};
use std::num::NonZeroUsize;

fn main() {
    let s = DefaultHasher::default();
    let mut cache: LruCache<isize, &str> = LruCache::with_hasher(NonZeroUsize::new(10).unwrap(), s);
}
```

## Inserting Items: `put()` and `push()`

### `put()` - Insert or Update

Inserts a key-value pair into the cache. If the key exists, its value is updated and the old value is returned. If the cache is at capacity, the least recently used item is evicted.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    // Insert new items - returns None
    assert_eq!(cache.put("apple", "red"), None);
    assert_eq!(cache.put("banana", "yellow"), None);

    // Update existing item - returns old value
    assert_eq!(cache.put("banana", "green"), Some("yellow"));

    // Insert when at capacity - evicts "apple" (LRU)
    assert_eq!(cache.put("cherry", "red"), None);
    assert!(cache.get(&"apple").is_none()); // apple was evicted
}
```

### `push()` - Insert and Get Evicted Item

Pushes a key-value pair into the cache. Returns the old entry's key-value pair if the key already exists, or the evicted entry if at capacity.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    assert_eq!(None, cache.push(1, "a"));
    assert_eq!(None, cache.push(2, "b"));

    // Returns old entry for existing key
    assert_eq!(Some((2, "b")), cache.push(2, "beta"));

    // Returns evicted entry when at capacity
    assert_eq!(Some((1, "a")), cache.push(3, "alpha"));

    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&"beta"));
    assert_eq!(cache.get(&3), Some(&"alpha"));
}
```

## Retrieving Items: `get()` and `peek()`

### `get()` - Retrieve and Mark as Recently Used

Returns a reference to the value and moves the key to the head of the LRU list (most recently used position).

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put("apple", 3);
    cache.put("banana", 2);

    // Get existing key - moves it to front of LRU list
    assert_eq!(cache.get(&"apple"), Some(&3));

    // Get non-existing key
    assert_eq!(cache.get(&"pear"), None);

    // Now "banana" is LRU, so adding new item evicts it
    cache.put("cherry", 5);
    assert!(cache.get(&"banana").is_none());
    assert_eq!(cache.get(&"apple"), Some(&3)); // apple still exists
}
```

### `peek()` - Retrieve Without Affecting LRU Order

Returns a reference to the value WITHOUT updating the LRU list. Useful for lookups that shouldn't affect eviction order.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    cache.put(1, "a");
    cache.put(2, "b");

    // peek doesn't affect LRU order
    assert_eq!(cache.peek(&1), Some(&"a"));
    assert_eq!(cache.peek(&2), Some(&"b"));
    
    // Since we used peek, adding a new item will still evict 1 (the LRU)
    cache.put(3, "c");
    assert!(cache.get(&1).is_none());
}
```

## Mutable Access: `get_mut()` and `peek_mut()`

### `get_mut()` - Mutable Reference and Mark as Recently Used

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put("counter", 0);

    // Modify value in place
    if let Some(value) = cache.get_mut(&"counter") {
        *value += 1;
    }

    assert_eq!(cache.get(&"counter"), Some(&1));

    // Chain multiple modifications
    for _ in 0..5 {
        if let Some(v) = cache.get_mut(&"counter") {
            *v += 1;
        }
    }

    assert_eq!(cache.get(&"counter"), Some(&6));
}
```

### `peek_mut()` - Mutable Reference Without Affecting LRU Order

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    cache.put(1, "a");
    cache.put(2, "b");

    assert_eq!(cache.peek_mut(&1), Some(&mut "a"));
    assert_eq!(cache.peek_mut(&2), Some(&mut "b"));
}
```

## Checking Existence: `contains()`

Checks if a key exists without updating LRU order.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put("apple", 1);
    cache.put("banana", 2);

    assert!(cache.contains(&"apple"));
    assert!(cache.contains(&"banana"));
    assert!(!cache.contains(&"cherry"));

    // contains() doesn't affect LRU order
    cache.put("cherry", 3);
    assert!(!cache.contains(&"apple")); // evicted
}
```

## Removing Items: `pop()`, `pop_lru()`, `pop_mru()`

### `pop()` - Remove Specific Key

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    cache.put(2, "a");

    assert_eq!(cache.pop(&1), None);
    assert_eq!(cache.pop(&2), Some("a"));
    assert_eq!(cache.pop(&2), None);
    assert_eq!(cache.len(), 0);
}
```

### `pop_lru()` - Remove Least Recently Used Item

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");

    // Remove LRU item
    assert_eq!(cache.pop_lru(), Some((1, "a")));
    assert_eq!(cache.len(), 2);
}
```

### `pop_mru()` - Remove Most Recently Used Item

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");

    // Remove MRU item
    assert_eq!(cache.pop_mru(), Some((3, "c")));
    assert_eq!(cache.len(), 2);
}
```

## Clearing the Cache: `clear()`

Removes all items while maintaining capacity.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);

    assert_eq!(cache.len(), 3);

    cache.clear();

    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.cap().get(), 3); // capacity unchanged
}
```

## Key-Value Pair Retrieval

### `get_key_value()` - Get Both Key and Value Reference

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    cache.put(String::from("1"), "a");
    cache.put(String::from("2"), "b");

    assert_eq!(cache.get_key_value("1"), Some((&String::from("1"), &"a")));
}
```

### `pop_entry()` - Remove and Return Key-Value Pair

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());

    cache.put(1, "a");
    cache.put(2, "a");

    assert_eq!(cache.pop_entry(&2), Some((2, "a")));
    assert_eq!(cache.len(), 1);
}
```
