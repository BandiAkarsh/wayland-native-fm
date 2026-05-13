//! File list view

use gtk4::Box;

pub struct FileListView {
    container: Box,
}

impl FileListView {
    pub fn new() -> Self {
        let container = Box::new(gtk4::Orientation::Vertical, 0);
        Self { container }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }
}

impl Default for FileListView {
    fn default() -> Self {
        Self::new()
    }
}