//! Bookmark management

use crate::gui::types::Bookmark;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Get bookmarks file path
pub fn get_bookmarks_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("wayland-file-manager");
    path.push("bookmarks.json");
    path
}

/// Load bookmarks from file
pub fn load_bookmarks() -> Vec<Bookmark> {
    let path = get_bookmarks_path();
    if path.exists() {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            if let Ok(bookmarks) = serde_json::from_reader(reader) {
                return bookmarks;
            }
        }
    }
    // Return default bookmarks if file doesn't exist or can't be read
    vec![Bookmark {
        name: "Home".to_string(),
        path: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home")),
    }]
}

/// Save bookmarks to file
pub fn save_bookmarks(bookmarks: &[Bookmark]) -> Result<(), std::io::Error> {
    let path = get_bookmarks_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(&path, json)?;
    Ok(())
}
