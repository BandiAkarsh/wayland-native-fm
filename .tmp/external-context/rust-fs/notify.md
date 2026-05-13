---
source: Docs.rs (notify 8.2.0)
library: notify
package: notify
topic: File system watching (inotify)
fetched: 2026-05-02
official_docs: https://docs.rs/notify/latest/notify/
---

# notify - File System Watching (v8.2.0)

## Overview

Cross-platform file system notification library. On Linux, it uses the inotify API for efficient file system event monitoring.

## Installation

```toml
[dependencies]
notify = "8.1.0"

# Optional: for event serialization
notify = { version = "8.1.0", features = ["serde"] }
```

## Basic Usage

```rust
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // Use recommended_watcher() for best platform implementation
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch a path (recursive)
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    // Process events
    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
```

## Key Types

### RecommendedWatcher

The recommended `Watcher` implementation for the current platform. On Linux, this uses inotify.

### Event

Represents a file system event with:
- `kind` - The type of event
- `paths` - Affected paths
- `attrs` - Additional attributes (flags, mode)

### EventKind

Top-level event categories:
- `Create` - File/directory created
- `Modify` - File/directory modified
- `Remove` - File/directory removed
- `Access` - File accessed (may not work on all systems)
- `Any` - Any event
- `Other` - Unknown event type

### RecursiveMode

- `Recursive` - Watch directory and all subdirectories
- `NonRecursive` - Watch only the specified directory

### Watcher Trait

The core trait for watching:
```rust
pub trait Watcher {
    fn watch(&mut self, path: &Path, mode: RecursiveMode) -> Result<()>;
    fn unwatch(&mut self, path: &Path) -> Result<()>;
}
```

## Linux-Specific: inotify

On Linux, notify uses the inotify API directly via the `notify::inotify` module.

### INotifyWatcher

Direct access to inotify:

```rust
use notify::inotify::INotifyWatcher;

let watcher = INotifyWatcher::new(|res| {
    // Handle events
})?;
```

### Linux Limits

```bash
# Increase inotify limits (if hitting "Bad File Descriptor" errors)
sudo sysctl fs.inotify.max_user_instances=8192
sudo sysctl fs.inotify.max_user_watches=524288
sudo sysctl -p
```

## Event Handling

### Using Closures

```rust
use notify::{Config, RecommendedWatcher, Watcher};

let mut watcher = RecommendedWatcher::new(
    move |res: Result<Event>| {
        match res {
            Ok(event) => {
                for path in event.paths {
                    println!("{:?} - {:?}", event.kind, path);
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    },
    Config::default(),
)?;
```

### Using Custom Handler

```rust
use notify::{EventHandler, Event, Result};

struct MyHandler;

impl EventHandler for MyHandler {
    fn handle_event(&mut self, event: Result<Event>) {
        match event {
            Ok(e) => println!("{:?}", e),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

let mut watcher = notify::recommended_watcher(MyHandler)?;
```

## Watching Multiple Paths

```rust
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

let mut watcher = notify::recommended_watcher(|_| {})?;

watcher.watch(Path::new("/path/to/dir1"), RecursiveMode::Recursive)?;
watcher.watch(Path::new("/path/to/dir2"), RecursiveMode::NonRecursive)?;
watcher.unwatch(Path::new("/path/to/dir1"))?;
```

## Event Filtering

```rust
use notify::{EventKind, RecommendedWatcher, Watcher};

let mut watcher = RecommendedWatcher::new(|res| {
    if let Ok(event) = res {
        // Only process modify events
        if matches!(event.kind, EventKind::Modify(_)) {
            // Handle modification
        }
    }
}, Config::default())?;
```

## Known Issues on Linux

### Network Filesystems
Network filesystems (NFS, etc.) may not emit events. Use `PollWatcher` as workaround.

### Large Directories
For very large directories, inotify may not receive all events reliably.

### Pseudo Filesystems
`/proc` and `/sys` don't emit change events. Use `PollWatcher` with `compare_contents`.

### Editor Behavior
Editors may truncate files or create new files on save - actual events may differ from expected.

## Alternative Watchers

### PollWatcher

Polling-based watcher (works everywhere, higher CPU):

```rust
use notify::poll::PollWatcher;

let mut watcher = PollWatcher::new(
    |res| {},
    notify::Config::default().with_compare_contents(true), // Compare file contents
)?;
```

### NullWatcher

Stub watcher for testing:

```rust
use notify::null::NullWatcher;

let watcher = NullWatcher::new(|_| {}, notify::Config::default())?;
```

## Debouncing Events

For many file operations, you'll receive multiple rapid events. Use debouncer crates:

```toml
notify-debouncer-mini = "0.5"  # Simple debouncing
# or
notify-debouncer-full = "0.5"  # Full-featured
```

```rust
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::time::Duration;
use std::path::Path;

let mut debouncer = new_debouncer(Duration::from_millis(200), |res| {
    match res {
        Ok(events) => {
            for event in events {
                println!("{:?}", event.path);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
})?;

debouncer.watch(Path::new("."), RecursiveMode::Recursive)?;
```

## Best Practices

1. **Use `recommended_watcher()`** - Automatically selects best backend
2. **Handle errors** - Watcher can return errors during operation
3. **Use debouncing** - For file editing to avoid event floods
4. **Watch parent directories** - To detect deletion of watched directory
5. **Check permissions** - May need elevated privileges for inotify
6. **Monitor resource usage** - Large directory trees use many inotify watches