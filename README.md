# Wayland Native File Manager

A **desktop-environment independent** GTK4 file manager for Wayland, written in Rust.

## Overview

This is a modern file manager that provides a **native Wayland experience** without relying on any desktop environments (GNOME, KDE, XFCE, etc.). It uses GTK4 with the gdk4-wayland backend directly, making it a truly standalone file manager that works on any Wayland compositor (Sway, Hyprland, Labwc, etc.).

## Key Specialty: Desktop-Environment Independent

Unlike most Linux file managers that depend on GNOME (Nautilus), KDE (Dolphin), or XFCE (Thunar), this file manager:
- **Does not require any desktop environment** - runs directly on Wayland compositors
- **No GNOME/KDE/Qt dependencies** - pure GTK4 implementation
- **Works on minimal Wayland setups** - ideal for Sway, Hyprland, Labwc, River, etc.
- **Lightweight** - no bloat from desktop environment integrations

## Features

- **Native Wayland Support** - Built with GTK4 and gdk4-wayland for proper Wayland integration
- **Tabbed Interface** - Multiple tabs for navigating different directories simultaneously
- **Bookmarks** - Save and quickly access favorite directories
- **Multiple View Modes** - Icon view and list view with sorting options
- **File Operations** - Copy, move, delete, and rename files with progress tracking
- **Mounted Drives** - Automatic detection of removable media and mounted drives
- **Hidden Files** - Toggle visibility of hidden files
- **Thumbnails** - Image thumbnail generation with caching
- **File Watching** - Real-time directory monitoring for changes

## Architecture

The project is organized into several core modules:

```
src/
├── lib.rs           # Library entry point
├── main.rs          # Application entry point
├── error.rs         # Error types and handling
├── logging.rs       # Tracing-based logging infrastructure
├── vfs/             # Virtual File System (security layer)
├── scanner/         # Directory scanning with caching
├── watcher/         # File system monitoring
├── operations/      # File operations (copy, move, delete)
├── thumbnails/      # Thumbnail generation and caching
└── gui/             # GTK4 GUI components
    ├── types.rs     # Core data types (TabId, NavState, FileEntry)
    ├── tabs.rs      # Tab management
    ├── bookmarks.rs # Bookmark persistence
    ├── filesystem.rs# File operations with VFS
    ├── operations.rs# Editor selection, file opening
    └── views.rs     # List and icon view builders
```

## Requirements

- Rust 1.75+
- GTK4 0.11
- Wayland display server

## Building

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release
```

## Running

```bash
# Run the application
cargo run

# Or use the binary directly
cargo build --release
./target/release/wayland-file-manager
```

## Environment Variables

- `RUST_LOG` - Configure logging level (e.g., `debug`, `info`)
- `XDG_DATA_HOME` - Override data directory location

## Testing

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test vfs
cargo test scanner
cargo test operations
cargo test watcher
```

## Security

The project implements several security measures:

- **Path Traversal Prevention** - VFS uses `symlink_metadata` to detect and prevent symlink attacks
- **Command Injection Prevention** - File paths are validated before spawning external editors
- **Safe File Operations** - All file operations use proper error handling and validation

See [SECURITY.md](SECURITY.md) for detailed security documentation.

## License

MIT License