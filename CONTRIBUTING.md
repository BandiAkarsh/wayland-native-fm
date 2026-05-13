# Contributing Guide

Thank you for your interest in contributing to the Wayland File Manager!

## Getting Started

### Prerequisites

- Rust 1.75 or later
- GTK4 0.11 development libraries
- Wayland development libraries

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/wayland-file-manager
   cd wayland-file-manager
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Project Structure

```
wayland-file-manager/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── main.rs          # Application entry point
│   ├── error.rs         # Error types
│   ├── logging.rs       # Logging setup
│   ├── vfs/             # Virtual File System
│   ├── scanner/         # Directory scanning
│   ├── watcher/         # File watching
│   ├── operations/     # File operations
│   ├── thumbnails/     # Thumbnail generation
│   └── gui/            # GTK4 UI components
├── tests/              # Integration tests
├── Cargo.toml          # Project manifest
└── README.md           # Project overview
```

## Coding Standards

### Code Style

- Use `rustfmt` for code formatting:
  ```bash
  cargo fmt
  ```

- Run clippy for linting:
  ```bash
  cargo clippy
  ```

### Naming Conventions

- **Modules**: `snake_case` (e.g., `vfs`, `file_operations`)
- **Types**: `PascalCase` (e.g., `DirectoryEntry`, `FileWatcher`)
- **Functions/Methods**: `snake_case` (e.g., `read_directory`, `get_mounted_drives`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `THUMBNAIL_SIZE`)

### Documentation

- Document public APIs with doc comments (`///`)
- Include examples in documentation where helpful
- Keep documentation concise but complete

Example:
```rust
/// Read directory entries using VFS (secure - prevents symlink attacks)
///
/// # Arguments
/// * `path` - The directory path to read
///
/// # Returns
/// Vector of FileEntry objects
pub fn read_directory(path: &Path) -> Vec<FileEntry> {
    // ...
}
```

### Error Handling

- Use the custom `FileManagerError` type from `error.rs`
- Provide context in error messages
- Handle errors at the appropriate level

## Making Changes

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow the coding standards
- Add tests for new functionality
- Update documentation if needed

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run clippy
cargo clippy -- -D warnings
```

### 4. Commit Your Changes

Follow conventional commit format:

```
type(scope): description

[optional body]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style
- `refactor`: Code refactoring
- `test`: Tests
- `chore`: Maintenance

Example:
```
feat(vfs): add symlink detection for security

Add symlink_metadata usage to prevent path traversal attacks
in DirectoryEntry::from_path()
```

### 5. Submit a Pull Request

- Describe your changes
- Link any related issues
- Ensure all tests pass

## Areas for Contribution

### High Priority

- Security improvements (path validation, command injection prevention)
- Bug fixes
- Test coverage improvements

### Medium Priority

- UI/UX improvements
- Performance optimizations
- Additional file operation support

### Lower Priority

- New view modes
- Advanced filtering
- Plugin system

## Questions?

If you have questions, feel free to open an issue or start a discussion.

## Code of Conduct

Be respectful and inclusive. Follow the Rust community guidelines.

---

Thank you for contributing!