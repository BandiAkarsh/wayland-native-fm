//! Sidebar component

use gtk4::Box;

pub struct Sidebar {
    container: Box,
}

impl Sidebar {
    pub fn new() -> Self {
        let container = Box::new(gtk4::Orientation::Vertical, 0);
        Self { container }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}