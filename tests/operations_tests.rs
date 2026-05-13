//! Tests for Operations module

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use wayland_file_manager::operations::copy::CopyOptions;

    #[test]
    fn test_copy_options_default() {
        let options = CopyOptions::new();
        assert!(!options.overwrite);
        assert_eq!(options.buffer_size, 64 * 1024);
    }

    #[test]
    fn test_copy_options_builder() {
        let options = CopyOptions::new()
            .with_overwrite(true)
            .with_buffer_size(128 * 1024);

        assert!(options.overwrite);
        assert_eq!(options.buffer_size, 128 * 1024);
    }

    #[test]
    fn test_copy_options_overwrite() {
        let options = CopyOptions::new().with_overwrite(true);
        assert!(options.overwrite);
    }

    #[test]
    fn test_copy_options_buffer_size() {
        let options = CopyOptions::new().with_buffer_size(1024);
        assert_eq!(options.buffer_size, 1024);
    }

    #[tokio::test]
    async fn test_copy_file_same_content() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source.txt");
        let dst = temp_dir.path().join("dest.txt");

        // Create source file
        std::fs::write(&src, "Hello, World!").unwrap();

        let options = CopyOptions::new();
        let result = wayland_file_manager::operations::copy::copy_file(&src, &dst, options).await;

        assert!(result.is_ok());
        assert!(dst.exists());
        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_copy_file_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source.txt");
        let dst = temp_dir.path().join("dest.txt");

        // Create source file
        std::fs::write(&src, "New content").unwrap();
        // Create destination file
        std::fs::write(&dst, "Old content").unwrap();

        let options = CopyOptions::new().with_overwrite(true);
        let result = wayland_file_manager::operations::copy::copy_file(&src, &dst, options).await;

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "New content");
    }

    #[tokio::test]
    async fn test_copy_file_no_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source.txt");
        let dst = temp_dir.path().join("dest.txt");

        // Create source file
        std::fs::write(&src, "New content").unwrap();
        // Create destination file
        std::fs::write(&dst, "Old content").unwrap();

        let options = CopyOptions::new(); // overwrite = false
        let result = wayland_file_manager::operations::copy::copy_file(&src, &dst, options).await;

        // Should fail because destination exists and overwrite is false
        assert!(result.is_err());
    }
}
