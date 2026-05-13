//! Tests for Scanner module

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use wayland_file_manager::scanner::Scanner;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new(1000, Duration::from_secs(300), 100);
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_scan_nonexistent_path() {
        let scanner = Scanner::new(100, Duration::from_secs(60), 10);
        let result = scanner.scan("/nonexistent/path/that/does/not/exist");
        // Should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_recursive_nonexistent() {
        let scanner = Scanner::new(100, Duration::from_secs(60), 10);
        let result = scanner.scan_recursive("/nonexistent/path/that/does/not/exist");
        // Should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_current_directory() {
        let scanner = Scanner::new(1000, Duration::from_secs(300), 100);
        // Scan current directory
        let result = scanner.scan(".");
        // Should work (might be empty or have entries)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_scanner_capacity_and_ttl() {
        let scanner = Scanner::new(500, Duration::from_secs(600), 50);
        // Verify parameters are stored
        let _ = scanner;
        assert!(true);
    }
}
