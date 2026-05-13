//! File system watcher using notify crate

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::error::FileManagerError;

/// Watcher events
#[derive(Debug, Clone)]
pub enum WatcherEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed(PathBuf, PathBuf),
}

/// File system watcher
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    watched_paths: Arc<Mutex<HashMap<PathBuf, bool>>>,
    event_sender: Sender<WatcherEvent>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(duration: Duration) -> Result<Self, FileManagerError> {
        let (event_sender, _event_receiver) = channel();
        let watched_paths = Arc::new(Mutex::new(HashMap::new()));
        let sender_clone = event_sender.clone();

        let watcher = RecommendedWatcher::new(
            move |result: Result<Event, notify::Error>| {
                match result {
                    Ok(event) => {
                        for path in event.paths {
                            let watcher_event = match event.kind {
                                notify::EventKind::Create(_) => {
                                    WatcherEvent::Created(path.clone())
                                }
                                notify::EventKind::Modify(_) => {
                                    WatcherEvent::Modified(path.clone())
                                }
                                notify::EventKind::Remove(_) => {
                                    WatcherEvent::Deleted(path.clone())
                                }
                                _ => continue,
                            };
                            
                            if let Err(e) = sender_clone.send(watcher_event) {
                                tracing::error!("Failed to send watcher event: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Watch error: {:?}", e);
                    }
                }
            },
            Config::default().with_poll_interval(duration),
        )
        .map_err(|e| FileManagerError::Watcher(e.to_string()))?;

        Ok(Self {
            watcher,
            watched_paths,
            event_sender,
        })
    }

    /// Watch a path
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileManagerError> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() {
            return Err(FileManagerError::NotFound(path.display().to_string()));
        }

        self.watcher
            .watch(&path, RecursiveMode::Recursive)
            .map_err(|e| FileManagerError::Watcher(e.to_string()))?;

        self.watched_paths
            .lock()
            .unwrap()
            .insert(path.clone(), true);

        tracing::info!("Watching: {}", path.display());
        Ok(())
    }

    /// Unwatch a path
    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileManagerError> {
        let path = path.as_ref().to_path_buf();
        
        self.watcher
            .unwatch(&path)
            .map_err(|e| FileManagerError::Watcher(e.to_string()))?;

        self.watched_paths.lock().unwrap().remove(&path);
        
        tracing::info!("Stopped watching: {}", path.display());
        Ok(())
    }
}

/// Alias for backwards compatibility
pub type Watcher = FileWatcher;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new(Duration::from_secs(2));
        assert!(watcher.is_ok());
    }
}