---
source: Context7 API
library: Notify
package: notify (Rust crate)
topic: recursive directory watching with tokio async
fetched: 2026-05-02T10:30:00Z
official_docs: https://docs.rs/notify/latest/notify/
---

# Notify Crate (Rust) - Recursive Directory Watching with Tokio Async

## Key Types
- **RecommendedWatcher**: Cross-platform watcher that automatically selects the best backend for the current OS (inotify on Linux, kqueue on macOS, etc.)
- **RecursiveMode**: Enum to control watching behavior. Use `RecursiveMode::Recursive` to watch directories recursively.
- **EventKind**: Enum representing filesystem event types (create, modify, remove, rename, etc.)
- **Event**: Struct containing event details: `kind`, `paths`, `tracker` (for correlating rename events), `need_rescan()` flag.

## Tokio Async Integration
Use `tokio::sync::mpsc::unbounded_channel` as the event handler for direct async integration:

```rust
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> notify::Result<()> {
    // Create tokio unbounded channel - works directly as EventHandler
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Initialize RecommendedWatcher with the tokio channel
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch target directory recursively
    watcher.watch(Path::new("/path/to/watched"), RecursiveMode::Recursive)?;

    // Process events asynchronously
    while let Some(result) = rx.recv().await {
        match result {
            Ok(event) => handle_event(event),
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
```

## Event Handling (Create, Modify, Delete, Rename)
Match on `EventKind` to handle specific event types:

```rust
use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode, DataChange},
    EventKind, RecursiveMode, Watcher,
};

fn handle_event(event: notify::Event) {
    match event.kind {
        // Create events
        EventKind::Create(create_kind) => match create_kind {
            CreateKind::File => println!("File created: {:?}", event.paths),
            CreateKind::Folder => println!("Folder created: {:?}", event.paths),
            _ => println!("Unknown creation: {:?}", event.paths),
        },

        // Modify events
        EventKind::Modify(modify_kind) => match modify_kind {
            ModifyKind::Data(DataChange::Content) => println!("Content modified: {:?}", event.paths),
            ModifyKind::Name(rename) => match rename {
                RenameMode::From => println!("Renamed from: {:?}", event.paths.get(0)),
                RenameMode::To => println!("Renamed to: {:?}", event.paths.get(0)),
                RenameMode::Both => println!("Renamed: {:?} -> {:?}", event.paths.get(0), event.paths.get(1)),
                _ => println!("Rename event: {:?}", event.paths),
            },
            _ => println!("Modified: {:?}", event.paths),
        },

        // Remove events
        EventKind::Remove(remove_kind) => match remove_kind {
            RemoveKind::File => println!("File removed: {:?}", event.paths),
            RemoveKind::Folder => println!("Folder removed: {:?}", event.paths),
            _ => println!("Removed: {:?}", event.paths),
        },

        _ => println!("Other event: {:?}", event.kind),
    }

    // Check if rescan is needed (events may have been missed)
    if event.need_rescan() {
        eprintln!("WARNING: Rescan needed - some events may have been missed");
    }

    // Correlate rename events using tracker ID
    if let Some(tracker) = event.tracker() {
        println!("Event tracker ID: {}", tracker);
    }
}
```

## Error Handling
- **Watcher initialization errors**: `notify::recommended_watcher()` returns `Result<RecommendedWatcher, notify::Error>`. Handle with `?` operator or match.
- **Watch errors**: `watcher.watch()` returns `Result<(), notify::Error>` for invalid paths or permission issues.
- **Event processing errors**: The mpsc channel returns `Result<Event, notify::Error>` for watcher failures (e.g., OS backend errors).
- **Rescan flag**: `event.need_rescan()` indicates the watcher may have missed events (e.g., OS buffer overflow). Refresh file state when true.

## Event Filtering (Optional)
Use `Config` with `EventKindMask` to filter events at the watcher level (more efficient than filtering in event loop):

```rust
use notify::{Config, EventKindMask, RecommendedWatcher, RecursiveMode, Watcher};

fn main() -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Filter to only create, remove, and modify events (no access events)
    let config = Config::default()
        .with_event_kinds(EventKindMask::CORE); // CORE = create + remove + all modify

    let mut watcher = RecommendedWatcher::new(tx, config)?;
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}
```

## Notes
- `RecommendedWatcher` is the recommended backend for most use cases (automatic platform selection).
- Use `RecursiveMode::NonRecursive` to watch only the top-level directory.
- Rename events are split into `RenameMode::From` and `RenameMode::To` unless correlated via `tracker` ID.
- For version 8.x, ensure your `Cargo.toml` specifies `notify = "8.0"`.
