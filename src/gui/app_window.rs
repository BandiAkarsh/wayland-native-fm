//! Wayland File Manager - Minimal working version

use std::path::PathBuf;
use std::sync::Mutex;

static CURRENT_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn get_current_dir() -> PathBuf {
    if let Ok(guard) = CURRENT_DIR.lock() {
        guard
            .clone()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home")))
    } else {
        PathBuf::from("/home")
    }
}

pub fn set_current_dir(path: PathBuf) {
    if let Ok(mut guard) = CURRENT_DIR.lock() {
        *guard = Some(path);
    }
}
