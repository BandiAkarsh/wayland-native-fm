# Wayland File Manager - Code Quality Standards

## Project Overview
- **Language**: Rust (2021 edition)
- **Target**: Linux with Wayland display server
- **GUI Framework**: GTK4

## Code Style

### Formatting
- Use `cargo fmt` with default settings
- 4 spaces indentation (no tabs)
- Maximum line length: 100 characters

### Naming Conventions
- **Modules**: `snake_case` (e.g., `file_manager`, `vfs_backend`)
- **Types/Structs**: `PascalCase` (e.g., `DirectoryEntry`, `FileListView`)
- **Functions**: `snake_case` (e.g., `copy_file`, `scan_directory`)
- **Constants**: `SCREAMING_SNAKE_CASE`

### Error Handling
- Use `thiserror` for error types
- Always provide context in error messages
- Use the `io_error` helper for I/O errors

### Async Patterns
- Use `tokio` for async runtime
- Avoid recursive async functions (use loops instead)
- Use `Box<dyn Future>` for async trait methods

### GUI (GTK4)
- Keep GUI code separate from business logic
- Use the builder pattern for widget construction
- Handle errors gracefully (don't panic)

## Testing
- Unit tests in-line with `#[cfg(test)]` modules
- Integration tests in `/tests/` directory
- Always test error paths

## Security
- Validate all file paths (prevent traversal)
- Don't expose sensitive data in errors
- Use non-blocking I/O for UI responsiveness