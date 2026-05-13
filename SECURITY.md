# Security Model

This document describes the security measures implemented in the Wayland File Manager.

## Overview

The file manager handles untrusted file paths from user input and displays them in a GTK4 UI. Security measures focus on:

1. **Path Traversal Prevention** - Preventing symlink-based directory traversal attacks
2. **Command Injection Prevention** - Validating paths before spawning external editors
3. **Safe File Operations** - Using VFS layer for all file access

## Path Traversal Prevention

### The Problem

When a user navigates to a directory containing symlinks, a naive implementation might accidentally follow those symlinks, potentially:

- Exposing sensitive files outside the intended directory
- Creating infinite loops with circular symlinks
- Accessing files the user shouldn't have permission to view

### The Solution: VFS with symlink_metadata

The VFS module (`src/vfs/entry.rs`) uses `symlink_metadata` instead of `metadata`:

```rust
// WRONG - follows symlinks (vulnerable)
let metadata = std::fs::metadata(path)?;

// CORRECT - does not follow symlinks (secure)
let std_metadata = std::fs::symlink_metadata(path)?;
```

The `symlink_metadata` function returns metadata about the symlink itself, not the target it points to. This allows the code to:

1. **Detect symlinks** - Check if `file_type().is_symlink()` is true
2. **Get actual file type** - Determine if the entry is a file or directory without following
3. **Prevent traversal** - Reject or warn about symlinks in sensitive contexts

### Implementation in DirectoryEntry

```rust
impl DirectoryEntry {
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        // Use symlink_metadata to get metadata without following symlinks
        let std_metadata = std::fs::symlink_metadata(path)?;
        let is_symlink = std::fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        let metadata = EntryMetadata::from_std_metadata(&std_metadata, is_symlink);
        // ...
    }
}
```

## Command Injection Prevention

### The Problem

When opening files with external editors, the application spawns a new process. If the file path contains shell metacharacters, it could be exploited to run arbitrary commands.

### The Solution: Path Validation

The operations module (`src/gui/operations.rs`) validates paths before spawning:

```rust
pub fn open_file(path: &Path, editor: &Path) -> Result<(), FileManagerError> {
    // Validate path - no shell metacharacters allowed
    let path_str = path.display().to_string();
    if contains_shell_metacharacters(&path_str) {
        return Err(FileManagerError::Operation(
            "Path contains invalid characters".to_string()
        ));
    }

    // Use Command with proper argument handling
    let mut cmd = Command::new(editor);
    cmd.arg(path);
    // ...
}
```

### Validation Checks

- No shell metacharacters (`&`, `|`, `;`, `$`, `` ` ``, etc.)
- No path traversal sequences (`../`, `~` outside home)
- Path must exist and be accessible

## Safe File Operations

### All Operations Use VFS

All file operations in the GUI layer go through the VFS:

```rust
// gui/filesystem.rs
pub fn read_directory(path: &Path) -> Vec<FileEntry> {
    // Uses DirectoryEntry::from_path which uses symlink_metadata
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter_map(|e| get_file_info_vfs(&e.path()))  // VFS!
            .collect(),
        Err(_) => vec![],
    }
}
```

### Operation Safety

- **Copy** - Validates source exists, destination doesn't (unless overwrite)
- **Move** - Checks permissions, handles name collisions
- **Delete** - Moves to trash instead of permanent delete
- **Rename** - Prevents overwriting existing files

## Security Best Practices

### For Developers

1. **Always use VFS** - Never use `std::fs::metadata()` directly; use `DirectoryEntry::from_path()`
2. **Validate paths** - Check for shell metacharacters before spawning processes
3. **Handle errors** - Don't expose internal error details to users
4. **Log security events** - Use tracing to log suspicious activity

### For Users

1. **Be cautious with symlinks** - The file manager detects and warns about symlinks
2. **Use trash** - Delete operations move files to trash, not permanent deletion
3. **Review permissions** - Some operations require elevated permissions

## Security Considerations

### Current Protections

- Path traversal via symlinks prevented
- Command injection via path validation
- File operations use VFS layer

### Potential Future Improvements

- Sandboxed file operations (Linux namespaces)
- Audit logging for sensitive operations
- User confirmation for batch operations
- Encrypted bookmark storage

## Reporting Security Issues

If you discover a security vulnerability, please open an issue with the label "security".