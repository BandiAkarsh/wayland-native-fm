//! GUI module

pub mod app_window;
pub mod bookmarks;
pub mod filesystem;
pub mod operations;
pub mod tabs;
pub mod types;
pub mod views;

pub use app_window::{get_current_dir, set_current_dir};
pub use bookmarks::{get_bookmarks_path, load_bookmarks, save_bookmarks};
pub use filesystem::{
    create_new_folder, filter_and_sort, get_mounted_drives, move_to_trash, read_directory,
    rename_file,
};
pub use operations::{
    get_available_editors, get_default_editor, get_preferred_editor, open_file,
    open_file_with_choice,
};
pub use tabs::{create_tab, navigate_to_path, AppState};
pub use types::{Bookmark, FileEntry, NavState, SortBy, TabData, TabId, ViewMode};
pub use views::{build_icon_view, build_list_view, refresh_tab_view};
