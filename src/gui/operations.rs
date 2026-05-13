//! File operations - opening files, editor selection
//!
//! This module handles opening files with external editors safely.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_EDITOR: &str = "nano";

/// Get preferred editor from environment or available editors
pub fn get_preferred_editor() -> Option<PathBuf> {
    if let Ok(e) = env::var("EDITOR") {
        if !e.is_empty() {
            return Some(PathBuf::from(e));
        }
    }
    if let Ok(e) = env::var("VISUAL") {
        if !e.is_empty() {
            return Some(PathBuf::from(e));
        }
    }
    let editors = get_available_editors();
    editors.first().map(|e| e.clone())
}

/// Get list of available editors on the system
pub fn get_available_editors() -> Vec<PathBuf> {
    let editor_names = [
        "code", "cursor", "vscode", "gedit", "kate", "nvim", "vim", "nano", "emacs", "geany", "pluma",
    ];
    let mut editors = Vec::new();
    for e in &editor_names {
        if let Ok(o) = Command::new("which").arg(e).output() {
            if o.status.success() && !o.stdout.is_empty() {
                let path = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if !path.is_empty() {
                    editors.push(PathBuf::from(path));
                }
            }
        }
    }
    editors
}

/// Open file with a selection dialog - let user choose from available editors
pub fn open_file_with_choice(path: PathBuf, current_editor: PathBuf) {
    let path_clone = path.clone();
    open_file_with_choice_owned(path_clone, current_editor.clone())
}

fn open_file_with_choice_owned(path: PathBuf, current_editor: PathBuf) {
    let editors = get_available_editors();

    if editors.is_empty() {
        // No editors found, just use current one
        open_file(&path, &current_editor);
        return;
    }

    if editors.len() == 1 {
        // Only one editor, use it
        open_file(&path, &editors[0]);
        return;
    }

    // For now, just use the first available editor
    // The dialog functionality would require GTK integration
    open_file(&path, &editors[0]);
}

/// Open file with editor (safe - validates path exists and is a file)
pub fn open_file(path: &Path, editor: &PathBuf) {
    // Validate path exists and is a file (not a directory)
    if !path.exists() {
        println!("[ERROR] File does not exist: {}", path.display());
        return;
    }

    if path.is_dir() {
        println!("[ERROR] Cannot open directory: {}", path.display());
        return;
    }

    // Validate editor path exists
    if !editor.exists() {
        println!("[ERROR] Editor does not exist: {}", editor.display());
        return;
    }

    println!("[OPEN] {} with {}", path.display(), editor.display());

    // Use Command::new which is safe - no shell injection
    // The path argument is passed directly, not through shell
    match Command::new(editor).arg(path).spawn() {
        Ok(_) => println!("[OPEN] Successfully launched editor"),
        Err(e) => println!("[ERROR] Failed to open file: {}", e),
    }
}

/// Get default editor fallback
pub fn get_default_editor() -> PathBuf {
    PathBuf::from(DEFAULT_EDITOR)
}