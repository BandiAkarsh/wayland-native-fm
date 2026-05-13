---
source: Rayon FAQ + Official Docs
library: rayon
package: rayon
topic: performance-considerations-parallel-benefits
fetched: 2026-05-02T12:00:00Z
official_docs: https://github.com/rayon-rs/rayon/blob/main/FAQ.md
---

# Rayon Performance Considerations

## When Parallel Scanning is Beneficial
### Good Use Cases (CPU-bound tasks):
- Large directories (1000+ entries)
- Heavy per-file processing (metadata analysis, hashing, content scanning)
- Multi-core systems with available CPU resources

### Bad Use Cases (Avoid parallel):
- Small directories (< 100 entries)
- IO-bound tasks (pure file reading/writing, network calls)
- Very fast operations (parallel overhead outweighs benefits)

## How Rayon Manages Threads
- **Default threads**: Matches number of logical CPU cores (includes hyperthreading)
- **Custom threads**: Set `RAYON_NUM_THREADS` env var or use `ThreadPoolBuilder`
- **Work stealing**: Idle threads steal work from busy threads' queues for dynamic load balancing

## Parallel Overhead Costs
- Thread coordination overhead (~microseconds per task)
- Collection cloning for `par_iter_with`/`par_iter_init`
- Result collection synchronization

## Performance Tips for Directory Scanning
1. **Collect Walkdir entries first**: Avoid parallelizing the walk itself (Walkdir is sequential)
2. **Batch processing**: For very large directories, process entries in chunks
3. **Avoid nested parallelism**: Don't create parallel iterators inside parallel iterators unnecessarily
4. **Profile first**: Use `criterion` or similar to measure if parallel is faster

## FAQ Excerpts
> "Rayon uses work stealing to dynamically ascertain how much parallelism is available"
> "Default threads = number of logical cores (includes hyperthreading)"
> "For small iterators, sequential iteration may be faster due to parallel overhead"

## When to Use Parallel Iteration
| Scenario | Parallel Recommended? |
|----------|------------------------|
| 10 files, simple metadata | ❌ No |
| 10,000 files, hash each file | ✅ Yes |
| 1,000 files, only read paths | ❌ No (IO-bound) |
| 5,000 files, compute checksums | ✅ Yes |
