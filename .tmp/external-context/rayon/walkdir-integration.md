---
source: Official Walkdir Docs + Rayon Docs
library: rayon + walkdir
package: rayon, walkdir
topic: parallel-directory-scanning-walkdir
fetched: 2026-05-02T12:00:00Z
official_docs: https://docs.rs/walkdir/latest/walkdir/ + https://docs.rs/rayon/latest/rayon/
---

# Combining Rayon with Walkdir for Parallel Directory Scanning

## Key Limitation
Walkdir's `WalkDir` iterator is **sequential** (implements `Iterator`, not `ParallelIterator`). To parallelize directory scanning:
1. Collect Walkdir entries first (sequential)
2. Process collected entries in parallel with Rayon

## Basic Workflow
```rust
use rayon::prelude::*;
use walkdir::WalkDir;
use std::path::PathBuf;

fn parallel_directory_scan(root: &str) -> Vec<PathBuf> {
    // Step 1: Sequential walk to collect entries
    let entries: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok()) // Handle errors
        .map(|e| e.into_path())
        .collect();
    
    // Step 2: Parallel processing of collected entries
    entries.into_par_iter()
        .filter(|path| path.is_file()) // Example: keep only files
        .collect()
}

fn main() {
    let files = parallel_directory_scan("./large-directory");
    println!("Found {} files", files.len());
}
```

## Advanced: Parallel Metadata Processing
Process file metadata in parallel after collecting entries:
```rust
use rayon::prelude::*;
use walkdir::WalkDir;
use std::fs;

fn get_file_sizes(root: &str) -> Vec<(PathBuf, u64)> {
    let entries: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
    
    entries.into_par_iter()
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let path = e.into_path();
            let size = fs::metadata(&path)
                .map(|m| m.len())
                .unwrap_or(0);
            (path, size)
        })
        .collect()
}
```

## Optimization Notes
- For very large directories: Collect entries in chunks to avoid high memory usage
- Use `par_bridge()` if you want to interleave walking and processing (not recommended for pure directory scanning)
- Walkdir's `max_depth()` and `min_depth()` can pre-filter entries before parallel processing
