# Architecture Overview

This document describes how the modules in the Wayland Native File Manager connect and work together.

## Desktop-Environment Independent

Unlike traditional Linux file managers (Nautilus, Dolphin, Thunar), this project:
- **No desktop environment required** - runs directly on Wayland compositors
- **Pure GTK4** - no Qt or desktop-specific toolkits
- **Works anywhere on Wayland** - Sway, Hyprland, Labwc, Cage, etc.
- **Minimal dependencies** - only GTK4 and standard Linux libraries

## Module Dependencies

```
┌─────────────────────────────────────────────────────────────────┐
│                        main.rs                                   │
│                  (GTK4 Application)                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         gui/                                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │
│  │ tabs.rs  │  │ views.rs │  │bookmarks │  │ filesystem.rs│   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └──────┬───────┘   │
│       │            │            │               │             │
│       └────────────┴─────┬──────┴───────────────┘             │
│                         ▼                                      │
│                    types.rs                                     │
│              (TabId, NavState, FileEntry)                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Core Modules                               │
│                                                                 │
│  ┌─────────┐    ┌─────────┐    ┌─────────────┐                │
│  │   vfs   │◄──►│ scanner │    │  operations │                │
│  └────┬────┘    └────┬────┘    └──────┬──────┘                │
│       │             │                │                         │
│       ▼             ▼                ▼                         │
│  ┌─────────┐    ┌─────────┐    ┌─────────────┐                │
│  │ watcher │    │thumbnails│   │   logging   │                │
│  └─────────┘    └─────────┘    └─────────────┘                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                       error.rs                                  │
│              (FileManagerError types)                           │
└─────────────────────────────────────────────────────────────────┘
```

## Core Modules

### VFS (Virtual File System)

**Location:** `src/vfs/`

The VFS module provides a security layer for file operations:

- `entry.rs` - `DirectoryEntry` struct with metadata (size, mtime, file type)
- `backend.rs` - Backend abstraction for different filesystem types
- `mod.rs` - Public API

**Key Security Feature:** Uses `symlink_metadata` instead of `metadata` to detect symlinks without following them, preventing path traversal attacks.

### Scanner

**Location:** `src/scanner/`

Provides directory scanning with optional caching:

- Recursive scanning using `walkdir`
- Single-directory scanning
- Cache integration for performance

### Watcher

**Location:** `src/watcher/`

File system monitoring using the `notify` crate:

- `FileWatcher` struct for monitoring directories
- Events: Created, Modified, Deleted, Renamed
- Configurable poll intervals

### Operations

**Location:** `src/operations/`

File operation implementations:

- `copy.rs` - Async file/directory copy with progress
- `move_.rs` - Move operations
- `delete.rs` - Delete with trash support

### Thumbnails

**Location:** `src/thumbnails/`

Image thumbnail generation:

- Uses `image` crate for processing
- LRU cache for performance
- Supports PNG, JPEG, GIF, WebP, BMP

### Logging

**Location:** `src/logging.rs`

Tracing-based logging with:

- Non-blocking writer (prevents memory leaks)
- Daily log rotation
- Console and file output
- Configurable via `RUST_LOG`

## GUI Module Structure

### types.rs

Core data structures:

```rust
TabId          // Unique tab identifier
NavState       // Navigation state (path, history, sort, view mode)
FileEntry      // File information (name, path, size, modified)
SortBy         // Sort options (Name, Date, Size, Type)
ViewMode       // View options (List, Icon)
Bookmark       // Persisted bookmark (name, path)
```

### tabs.rs

Tab management:

- `AppState` - Global application state with tab collection
- `create_tab()` - Create new tab with path
- Navigation methods (back, forward, up)

### views.rs

View rendering:

- `build_list_view()` - List view with columns
- `build_icon_view()` - Icon grid view
- `refresh_tab_view()` - Update view with current entries

### filesystem.rs

File operations using VFS:

- `read_directory()` - Read directory entries
- `filter_and_sort()` - Apply filters and sorting
- `move_to_trash()` - Safe delete to trash
- `rename_file()` - Rename with collision handling
- `get_mounted_drives()` - Detect removable media

### bookmarks.rs

Bookmark persistence:

- JSON storage in `~/.config/wayland-file-manager/bookmarks.json`
- Load/save operations

### operations.rs

External application integration:

- `get_preferred_editor()` - Get user's preferred editor
- `get_default_editor()` - Fallback editor
- `open_file()` - Open file with validation

## Data Flow

### Opening a Directory

1. User clicks on a directory in the view
2. `tabs.rs::navigate_to_path()` is called
3. `filesystem.rs::read_directory()` reads entries using VFS
4. VFS `DirectoryEntry::from_path()` validates and extracts metadata
5. Entries are filtered and sorted via `filter_and_sort()`
6. `views.rs::refresh_tab_view()` updates the GTK4 UI

### File Operations

1. User selects files and clicks operation (copy/move/delete)
2. GUI validates selection
3. Operations module performs the action
4. Watcher detects changes (if enabled)
5. View refreshes to show updated contents

## Error Handling

All modules use `FileManagerError` from `error.rs`:

```rust
FileManagerError {
    FileSystem(io::Error),
    InvalidPath(String),
    Ui(String),
    Operation(String),
    Vfs(String),
    Watcher(String),
    PermissionDenied(String),
    NotFound(String),
    AlreadyExists(String),
    NotDirectory(String),
    NotFile(String),
    Unknown(String),
}
```

## Testing Strategy

Tests are organized in `tests/` directory:

- `vfs_tests.rs` - VFS entry creation and metadata
- `scanner_tests.rs` - Directory scanning
- `watcher_tests.rs` - File watching
- `operations_tests.rs` - Copy operations with options

Run with: `cargo test`