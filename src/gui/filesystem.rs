//! Filesystem operations with VFS integration
//!
//! This module provides secure filesystem operations using the VFS layer
//! which properly handles symlinks to prevent path traversal attacks.

use crate::gui::types::{FileEntry, SortBy};
use crate::vfs::DirectoryEntry;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Get mounted drives and removable media
pub fn get_mounted_drives() -> Vec<(String, PathBuf)> {
    let mut drives = Vec::new();

    // Check /media/{username} for removable media
    if let Some(home) = dirs::home_dir() {
        let media_path = home.join("media");
        if media_path.exists() {
            if let Ok(entries) = fs::read_dir(&media_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name() {
                            let name_str = name.to_string_lossy().to_string();
                            drives.push((format!("💌 {}", name_str), path));
                        }
                    }
                }
            }
        }
    }

    // Check /mnt for manually mounted drives
    let mnt_path = PathBuf::from("/mnt");
    if mnt_path.exists() {
        if let Ok(entries) = fs::read_dir(&mnt_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy().to_string();
                        if !name_str.starts_with('.') {
                            drives.push((format!("💾 {}", name_str), path));
                        }
                    }
                }
            }
        }
    }

    // Check /run/media/{username} for udisks mounts
    if let Some(home) = dirs::home_dir() {
        let username = home
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "root".to_string());
        let run_media = PathBuf::from(format!("/run/media/{}", username));
        if run_media.exists() {
            if let Ok(entries) = fs::read_dir(&run_media) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name() {
                            let name_str = name.to_string_lossy().to_string();
                            drives.push((format!("💌 {}", name_str), path));
                        }
                    }
                }
            }
        }
    }

    drives
}

/// Read directory using VFS (secure - uses symlink_metadata to prevent traversal)
pub fn read_directory(path: &Path) -> Vec<FileEntry> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter_map(|e| get_file_info_vfs(&e.path()))
            .collect(),
        Err(_) => vec![],
    }
}

/// Get file info using VFS (secure - prevents symlink attacks)
fn get_file_info_vfs(path: &Path) -> Option<FileEntry> {
    // Use VFS DirectoryEntry which properly uses symlink_metadata
    match DirectoryEntry::from_path(path) {
        Ok(entry) => {
            let name = entry.name;
            let is_dir = entry.metadata.is_dir;
            let size = if is_dir { 0 } else { entry.metadata.size };
            let modified = entry
                .metadata
                .modified_at
                .map(|ts| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts))
                .unwrap_or(SystemTime::UNIX_EPOCH);
            let extension = path
                .extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();

            Some(FileEntry {
                name,
                path: path.to_path_buf(),
                is_dir,
                size,
                modified,
                extension,
            })
        }
        Err(_) => None,
    }
}

/// Filter and sort entries
pub fn filter_and_sort(
    entries: &[FileEntry],
    show_hidden: bool,
    search: &str,
    sort_by: SortBy,
) -> Vec<FileEntry> {
    let mut filtered: Vec<_> = entries
        .iter()
        .filter(|e| {
            if !show_hidden && e.name.starts_with('.') {
                return false;
            }
            if !search.is_empty() && !e.name.to_lowercase().contains(&search.to_lowercase()) {
                return false;
            }
            true
        })
        .cloned()
        .collect();

    filtered.sort_by(|a, b| match sort_by {
        SortBy::Name => match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        },
        SortBy::Date => a.modified.cmp(&b.modified).reverse(),
        SortBy::Size => a.size.cmp(&b.size).reverse(),
        SortBy::Type => match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.extension.cmp(&b.extension),
        },
    });

    filtered
}

/// Create a new folder in the current directory
#[allow(dead_code)]
pub fn create_new_folder(parent_path: &Path) -> Option<PathBuf> {
    let mut counter = 0;

    loop {
        let folder_name = if counter == 0 {
            "Untitled Folder".to_string()
        } else {
            format!("Untitled Folder {}", counter)
        };

        let new_path = parent_path.join(&folder_name);
        if !new_path.exists() {
            match fs::create_dir(&new_path) {
                Ok(_) => {
                    tracing::info!("[CREATE FOLDER] {}", new_path.display());
                    return Some(new_path);
                }
                Err(e) => {
                    tracing::warn!("Failed to create folder: {}", e);
                    return None;
                }
            }
        }
        counter += 1;
        if counter > 100 {
            tracing::warn!("Too many untitled folders");
            return None;
        }
    }
}

/// Move file/folder to trash
pub fn move_to_trash(path: &Path) -> bool {
    let trash_path = dirs::home_dir()
        .map(|h| h.join(".local/share/Trash/files"))
        .unwrap_or_else(|| PathBuf::from("/tmp/.trash"));

    let file_name = path
        .file_name()
        .map(|n| PathBuf::from(n))
        .unwrap_or_else(|| PathBuf::from("unknown"));

    let trash_destination = trash_path.join(&file_name);

    // Create trash directory if it doesn't exist
    if !trash_path.exists() {
        if let Err(e) = fs::create_dir_all(&trash_path) {
            tracing::warn!("Could not create trash: {}", e);
            return false;
        }
    }

    // Handle name collision
    let mut final_destination = trash_destination.clone();
    let mut counter = 1;
    while final_destination.exists() {
        let ext = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        let stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        final_destination = if ext.is_empty() {
            trash_path.join(format!("{} ({})", stem, counter))
        } else {
            trash_path.join(format!("{}.{} ({})", stem, ext, counter))
        };
        counter += 1;
    }

    match fs::rename(path, &final_destination) {
        Ok(_) => {
            tracing::info!("[DELETE] Moved to trash: {}", final_destination.display());
            true
        }
        Err(e) => {
            tracing::warn!("Could not delete: {}", e);
            false
        }
    }
}

/// Rename a file/folder
pub fn rename_file(old_path: &Path, new_name: &str) -> Option<PathBuf> {
    let old_name = old_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    if old_name == new_name {
        return Some(old_path.to_path_buf());
    }

    let parent = old_path.parent().unwrap_or(old_path);
    let new_path = parent.join(new_name);

    if new_path.exists() {
        tracing::warn!("Name already exists: {}", new_path.display());
        return None;
    }

    match fs::rename(old_path, &new_path) {
        Ok(_) => {
            tracing::info!("[RENAME] {} -> {}", old_path.display(), new_path.display());
            Some(new_path)
        }
        Err(e) => {
            tracing::warn!("Could not rename: {}", e);
            None
        }
    }
}
