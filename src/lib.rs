//! Wayland File Manager Library

pub mod logging;
pub mod error;
pub mod vfs;
pub mod operations;
pub mod scanner;
pub mod watcher;
pub mod gui;
pub mod thumbnails;

#[cfg(test)]
mod tests {
    #[test]
    fn stub() {
        assert!(true);
    }
}

/// Run the file manager application
pub fn run() -> crate::error::Result<()> {
    // The real GUI is in gui::app_window
    // This is just a stub to verify modules compile
    tracing::info!("Wayland File Manager initialized");
    Ok(())
}