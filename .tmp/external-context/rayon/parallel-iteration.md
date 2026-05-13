---
source: Context7 API + Official Rayon Docs
library: rayon
package: rayon
topic: parallel-iteration-par_iter-collect-results
fetched: 2026-05-02T12:00:00Z
official_docs: https://docs.rs/rayon/latest/rayon/
---

# Rayon Parallel Iteration Basics

## Parallel Iteration Methods
Rayon provides three main ways to create parallel iterators:
1. `par_iter()` - Borrows collection items immutably (like `iter()`)
2. `par_iter_mut()` - Borrows collection items mutably (like `iter_mut()`)
3. `into_par_iter()` - Takes ownership of collection (like `into_iter()`)

All require importing `rayon::prelude::*`.

## Using `par_iter` for Collections
Works on slices, vectors, arrays, and standard collections:
```rust
use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Immutable parallel iteration
    let doubled: Vec<i32> = numbers.par_iter()
        .map(|x| x * 2)
        .collect();
    
    // Mutable parallel iteration
    let mut mutable_numbers = vec![1, 2, 3];
    mutable_numbers.par_iter_mut()
        .for_each(|x| *x *= 2);
    
    // Owned collection parallel iteration
    let owned_strings = vec!["hello", "world"];
    let uppercased: Vec<String> = owned_strings.into_par_iter()
        .map(|s| s.to_uppercase())
        .collect();
}
```

## Collecting Results from Parallel Iterators
Use the `collect()` method, which implements `FromParallelIterator` for common types:
- `Vec<T>`
- `HashSet<T>`
- `HashMap<K, V>`
- Tuples for unzipping: `(Vec<A>, Vec<B>)`

```rust
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    // Collect into Vec
    let squares: Vec<i32> = (0..100).into_par_iter()
        .map(|x| x * x)
        .collect();
    
    // Collect into HashSet
    let unique_squares: HashSet<i32> = (0..100).into_par_iter()
        .map(|x| x * x)
        .collect();
    
    // Unzip into two collections
    let pairs = vec![(1, 'a'), (2, 'b')];
    let (numbers, letters): (Vec<_>, Vec<_>) = pairs.into_par_iter()
        .unzip();
    
    // Partition into two collections
    let (evens, odds): (Vec<_>, Vec<_>) = (0..10).into_par_iter()
        .partition(|x| x % 2 == 0);
}
```

## Bridging Sequential Iterators
Use `par_bridge()` to convert any `Send`-able sequential iterator into a parallel iterator:
```rust
use rayon::prelude::*;
use std::io::BufReader;

fn process_lines(reader: impl BufRead + Send) -> Vec<String> {
    reader.lines()
        .filter_map(Result::ok)
        .par_bridge()  // Convert to parallel iterator
        .filter(|line| !line.is_empty())
        .map(|line| line.to_uppercase())
        .collect()
}
```
