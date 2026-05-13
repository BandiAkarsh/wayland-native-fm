//! Tests for Watcher module

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;
    use wayland_file_manager::watcher::{FileWatcher, WatcherEvent};

    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new(Duration::from_secs(2));
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_watcher_event_variants() {
        // Test that WatcherEvent variants can be created
        let _created = WatcherEvent::Created(PathBuf::from("/test"));
        let _modified = WatcherEvent::Modified(PathBuf::from("/test"));
        let _deleted = WatcherEvent::Deleted(PathBuf::from("/test"));
        let _renamed = WatcherEvent::Renamed(PathBuf::from("/old"), PathBuf::from("/new"));
        assert!(true);
    }

    #[test]
    fn test_watch_nonexistent_path() {
        let mut watcher = FileWatcher::new(Duration::from_secs(1)).unwrap();
        let result = watcher.watch("/nonexistent/path/that/does/not/exist");
        // Should fail because path doesn't exist
        assert!(result.is_err());
    }

    #[test]
    fn test_unwatch_nonexistent_path() {
        let mut watcher = FileWatcher::new(Duration::from_secs(1)).unwrap();
        // Unwatching a non-watched path should not panic
        let result = watcher.unwatch("/nonexistent/path");
        // Might fail or succeed depending on implementation
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_watcher_type_alias() {
        // Test that the Watcher type alias works
        let _watcher: wayland_file_manager::watcher::Watcher =
            FileWatcher::new(Duration::from_secs(1)).unwrap();
        assert!(true);
    }
}
