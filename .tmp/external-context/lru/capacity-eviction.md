---
source: Context7 API + docs.rs
library: lru (lru-rs)
package: lru
topic: Capacity management, resize, and eviction policies
fetched: 2026-05-02T00:00:00Z
official_docs: https://docs.rs/lru/latest/lru/struct.LruCache.html
---

# LRU Cache Capacity Management and Eviction Policies

## Understanding LRU Eviction

The LRU (Least Recently Used) cache automatically evicts the least recently used item when the cache reaches its capacity. "Recently used" includes:
- `get()` - marks item as most recently used
- `get_mut()` - marks item as most recently used
- `put()` - marks item as most recently used
- `contains()` and `peek()` do NOT affect LRU order

## Checking and Setting Capacity

### Get Current Capacity

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache: LruCache<isize, &str> = LruCache::new(NonZeroUsize::new(2).unwrap());
    assert_eq!(cache.cap().get(), 2);
}
```

### Get Current Size

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    assert_eq!(cache.len(), 0);

    cache.put(1, "a");
    assert_eq!(cache.len(), 1);

    cache.put(2, "b");
    assert_eq!(cache.len(), 2);

    // Adding when at capacity doesn't increase len (evicts instead)
    cache.put(3, "c");
    assert_eq!(cache.len(), 2);
}
```

### Check if Empty

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    assert!(cache.is_empty());

    cache.put(1, "a");
    assert!(!cache.is_empty());
}
```

## Dynamic Resizing: `resize()`

The cache capacity can be changed at runtime. If the new capacity is smaller, LRU items are evicted.

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(4).unwrap());
    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");
    cache.put(4, "d");

    // Shrink cache - evicts LRU items
    cache.resize(NonZeroUsize::new(2).unwrap());
    assert_eq!(cache.len(), 2);
    assert!(cache.get(&1).is_none()); // evicted (was LRU)
    assert!(cache.get(&2).is_none()); // evicted (was LRU)
    assert_eq!(cache.get(&3), Some(&"c"));
    assert_eq!(cache.get(&4), Some(&"d"));

    // Grow cache
    cache.resize(NonZeroUsize::new(10).unwrap());
    assert_eq!(cache.cap().get(), 10);
    assert_eq!(cache.len(), 2); // size unchanged, just more room
}
```

## Unbounded Cache (No Automatic Eviction)

For scenarios where you don't want automatic eviction, use `unbounded()` or `unbounded_with_hasher()`.

```rust
use lru::LruCache;

fn main() {
    // Create a cache that never automatically evicts items
    let mut cache: LruCache<isize, &str> = LruCache::unbounded();
    
    // Fill it up - no items will be evicted
    for i in 0..1000 {
        cache.put(i, "value");
    }
    assert_eq!(cache.len(), 1000);
}
```

**Warning**: Unbounded caches can grow indefinitely and cause memory issues. Use with caution or implement custom eviction logic.

## Eviction Behavior Examples

### Example 1: Basic LRU Eviction

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    
    cache.put("a", 1);  // Cache: [a]
    cache.put("b", 2);  // Cache: [b, a] (b is MRU, a is LRU)
    cache.get(&"a");    // Cache: [a, b] (a is now MRU, b is LRU)
    
    cache.put("c", 3);  // Evicts b (LRU), Cache: [c, a]
    
    assert!(cache.get(&"b").is_none());  // b was evicted
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"c"), Some(&3));
}
```

### Example 2: `peek()` Doesn't Affect Eviction

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    
    cache.put(1, "a");  // Cache: [1]
    cache.put(2, "b");  // Cache: [2, 1]
    
    let _ = cache.peek(&1);  // Doesn't change order! Cache still: [2, 1]
    
    cache.put(3, "c");  // Evicts 1 (LRU), not 2!
    
    assert!(cache.get(&1).is_none());  // 1 was evicted
    assert_eq!(cache.get(&2), Some(&"b"));
    assert_eq!(cache.get(&3), Some(&"c"));
}
```

### Example 3: `contains()` Doesn't Affect Eviction

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    
    cache.put(1, "a");
    cache.put(2, "b");
    
    assert!(cache.contains(&1));  // Doesn't affect order
    
    cache.put(3, "c");  // Evicts 1 (still LRU)
    
    assert!(!cache.contains(&1));
    assert!(cache.contains(&2));
    assert!(cache.contains(&3));
}
```

## Peeking at LRU and MRU Items

### `peek_lru()` - Inspect Least Recently Used Item

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);

    // Check LRU (least recently used)
    assert_eq!(cache.peek_lru(), Some((&"a", &1)));

    // Check MRU (most recently used)
    assert_eq!(cache.peek_mru(), Some((&"c", &3)));

    // Access "a" to make it MRU
    cache.get(&"a");
    assert_eq!(cache.peek_mru(), Some((&"a", &1)));
    assert_eq!(cache.peek_lru(), Some((&"b", &2)));
}
```

## Promoting and Demoting Items

### `promote()` - Mark as Most Recently Used

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());

    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");
    cache.get(&1);
    cache.get(&2);

    // Promote 3 to prevent it from being evicted
    assert!(cache.promote(&3));
    assert_eq!(cache.pop_lru(), Some((1, "a")));  // 1 is now LRU

    // Promoting non-existent key returns false
    assert!(!cache.promote(&4));
}
```

### `demote()` - Mark as Least Recently Used

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());

    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");
    cache.get(&1);
    cache.get(&2);

    // Demote 1 and 2 so they get evicted first
    assert!(cache.demote(&2));
    assert!(cache.demote(&1));
    assert_eq!(cache.pop_lru(), Some((1, "a")));
    assert_eq!(cache.pop_lru(), Some((2, "b")));

    // Demoting non-existent key returns false
    assert!(!cache.demote(&4));
}
```

## Best Practices for Capacity Management

1. **Choose capacity based on memory constraints**: Each entry consumes memory for the key, value, and internal bookkeeping.

2. **Use `resize()` for dynamic adjustment**: If your memory requirements change at runtime, use `resize()` rather than creating a new cache.

3. **Consider `unbounded()` carefully**: Only use unbounded caches when you have another mechanism to limit memory usage.

4. **Use `peek()` for read-only checks**: When you need to check a value without affecting eviction order, use `peek()` instead of `get()`.

5. **Monitor cache statistics**: Use `len()`, `cap()`, `peek_lru()`, and `peek_mru()` to understand your cache's state and tune capacity.
