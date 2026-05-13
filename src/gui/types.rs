//! GUI types and data structures

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;
use gtk4::{Box, Entry, Label, ScrolledWindow};

/// Unique tab identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TabId(pub usize);

/// Sort options
#[derive(Clone, Copy, Debug)]
pub enum SortBy {
    Name,
    Date,
    Size,
    Type,
}

/// View mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    List,
    Icon,
}

/// Navigation state per tab
#[derive(Clone)]
pub struct NavState {
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
    pub show_hidden: bool,
    pub search_query: String,
    pub sort_by: SortBy,
    pub view_mode: ViewMode,
    pub selected_paths: Vec<PathBuf>,
    pub last_clicked_index: Option<usize>,
    pub clipboard: Option<Clipboard>,
}

impl NavState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            current_path: path.clone(),
            history: vec![path],
            history_index: 0,
            show_hidden: true,
            search_query: String::new(),
            sort_by: SortBy::Name,
            view_mode: ViewMode::Icon,
            selected_paths: Vec::new(),
            last_clicked_index: None,
            clipboard: None,
        }
    }
}

/// Clipboard for cut/copy operations
#[derive(Clone)]
pub struct Clipboard {
    pub files: Vec<PathBuf>,
    pub is_cut: bool,
}

/// File entry info
#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: SystemTime,
    pub extension: String,
}

/// Per-tab data
#[derive(Clone)]
pub struct TabData {
    pub _id: TabId,
    pub nav_state: Rc<RefCell<NavState>>,
    pub entries_store: Rc<RefCell<Vec<FileEntry>>>,
    pub file_scrolled: ScrolledWindow,
    pub path_label: Label,
    pub search_entry: Entry,
    pub content_box: Box,
}

/// Bookmark entry
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
}