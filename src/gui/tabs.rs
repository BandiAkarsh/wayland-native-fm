//! Tab management

use crate::gui::filesystem::read_directory;
use crate::gui::types::{NavState, TabData, TabId};
use crate::gui::views::{build_icon_view, build_list_view};
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Notebook, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

/// Application state holding all tabs
pub struct AppState {
    pub tabs: Rc<RefCell<HashMap<TabId, TabData>>>,
    pub current_tab_id: Rc<RefCell<Option<TabId>>>,
    pub next_id: Rc<RefCell<usize>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tabs: Rc::new(RefCell::new(HashMap::new())),
            current_tab_id: Rc::new(RefCell::new(None)),
            next_id: Rc::new(RefCell::new(0)),
        }
    }

    pub fn new_tab_id(&self) -> TabId {
        let mut id = self.next_id.borrow_mut();
        let tab_id = TabId(*id);
        *id += 1;
        tab_id
    }

    pub fn get_current_tab(&self) -> Option<TabData> {
        let current_id = self.current_tab_id.borrow().clone();
        current_id.and_then(|id| self.tabs.borrow().get(&id).cloned())
    }

    pub fn set_current_tab(&self, id: Option<TabId>) {
        *self.current_tab_id.borrow_mut() = id;
    }

    /// Navigate the current tab to a new path (does not create new tab)
    pub fn navigate_current_tab(&self, editor: &PathBuf, new_path: PathBuf) {
        if let Some(tab_data) = self.get_current_tab() {
            navigate_to_path(
                &tab_data.nav_state,
                &tab_data.entries_store,
                editor,
                &tab_data.file_scrolled,
                &tab_data.path_label,
                new_path,
            );
        }
    }

    /// Close a specific tab by ID
    pub fn close_tab(&self, tab_id: TabId, notebook: &Notebook) {
        let page_num = {
            let tabs = self.tabs.borrow();
            tabs.get(&tab_id).map(|t| t.content_box.clone())
        };

        if let Some(content_box) = page_num {
            if let Some(page) = notebook.page_num(&content_box) {
                if page > 0 {
                    notebook.set_current_page(Some(page - 1));
                }
                notebook.remove_page(Some(page));
            }
        }

        {
            let mut tabs = self.tabs.borrow_mut();
            tabs.remove(&tab_id);
        }

        if self.current_tab_id.borrow().as_ref() == Some(&tab_id) {
            let tabs = self.tabs.borrow();
            if let Some((new_id, _)) = tabs.iter().next() {
                *self.current_tab_id.borrow_mut() = Some(*new_id);
                println!("[CLOSE TAB] Switched to tab {:?}", new_id);
            } else {
                *self.current_tab_id.borrow_mut() = None;
            }
        }

        println!("[CLOSE TAB] Tab {:?} closed", tab_id);
    }
}

/// Navigate to a path in the current tab (does not create new tab)
pub fn navigate_to_path(
    nav_state: &Rc<RefCell<NavState>>,
    entries_store: &Rc<RefCell<Vec<FileEntry>>>,
    editor: &PathBuf,
    file_scrolled: &ScrolledWindow,
    path_label: &Label,
    new_path: PathBuf,
) {
    let new_path = new_path.clone();

    {
        let mut s = nav_state.borrow_mut();
        let index = s.history_index;
        let hist_len = s.history.len();

        if index < hist_len - 1 {
            s.history.truncate(index + 1);
        }

        if !s.history.contains(&new_path) {
            s.history.push(new_path.clone());
        }

        s.current_path = new_path.clone();
        s.history_index = s.history.len() - 1;
    }

    let entries = read_directory(&new_path);
    entries_store.replace(entries.clone());

    path_label.set_label(&new_path.display().to_string());

    let (view_mode, show_hidden, search, sort_by) = {
        let s = nav_state.borrow();
        (s.view_mode, s.show_hidden, s.search_query.clone(), s.sort_by)
    };

    let file_box = match view_mode {
        ViewMode::List => build_list_view(
            &entries,
            show_hidden,
            &search,
            sort_by,
            editor,
            nav_state,
            entries_store,
            file_scrolled,
        ),
        ViewMode::Icon => build_icon_view(
            &entries,
            show_hidden,
            &search,
            sort_by,
            editor,
            nav_state,
            entries_store,
            file_scrolled,
            None,
        ),
    };
    file_scrolled.set_child(Some(&file_box));

    println!("[VIEW] {} ({} entries)", new_path.display(), entries.len());
}

// Re-export for tabs.rs
use crate::gui::types::FileEntry;
use crate::gui::types::ViewMode;

/// Create a new tab
pub fn create_tab(
    app_state: &Rc<AppState>,
    notebook: &Notebook,
    path: PathBuf,
    editor: &PathBuf,
) -> TabId {
    let tab_id = app_state.new_tab_id();

    let nav_state = Rc::new(RefCell::new(NavState::new(path.clone())));
    let entries_store = Rc::new(RefCell::new(Vec::new()));

    let file_scrolled = ScrolledWindow::new();
    file_scrolled.set_vexpand(true);

    let path_label = Label::new(Some(&path.display().to_string()));
    path_label.set_margin_start(16);

    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some("Search..."));
    search_entry.set_width_request(150);

    let content_box = Box::new(Orientation::Vertical, 0);

    let path_header = Box::new(Orientation::Horizontal, 0);
    path_header.append(&path_label);
    let spacer = Box::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    path_header.append(&spacer);
    path_header.append(&search_entry);
    content_box.append(&path_header);

    content_box.append(&file_scrolled);

    // Create tab label with close button
    let tab_label_box = Box::new(Orientation::Horizontal, 4);
    let tab_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let tab_label = Label::new(Some(&tab_name));
    tab_label_box.append(&tab_label);

    let close_btn = Button::new();
    close_btn.set_label("×");
    close_btn.set_margin_start(4);
    let tab_id_clone = tab_id;
    let app_state_raw = app_state.clone();
    let notebook_clone = notebook.clone();
    close_btn.connect_clicked(move |_| {
        app_state_raw.close_tab(tab_id_clone, &notebook_clone);
    });
    tab_label_box.append(&close_btn);
    tab_label_box.set_spacing(4);

    let page_num = notebook.append_page(&content_box, Some(&tab_label_box));
    notebook.set_current_page(Some(page_num as u32));
    notebook.set_tab_reorderable(&content_box, true);

    let tab_data = TabData {
        _id: tab_id,
        nav_state: nav_state.clone(),
        entries_store: entries_store.clone(),
        file_scrolled: file_scrolled.clone(),
        path_label: path_label.clone(),
        search_entry: search_entry.clone(),
        content_box: content_box.clone(),
    };

    app_state.tabs.borrow_mut().insert(tab_id, tab_data);

    // Load directory and build view
    let entries = read_directory(&path);
    entries_store.replace(entries.clone());

    let view_mode = nav_state.borrow().view_mode;
    let file_box = match view_mode {
        ViewMode::List => build_list_view(
            &entries,
            true,
            "",
            crate::gui::types::SortBy::Name,
            editor,
            &nav_state,
            &entries_store,
            &file_scrolled,
        ),
        ViewMode::Icon => build_icon_view(
            &entries,
            true,
            "",
            crate::gui::types::SortBy::Name,
            editor,
            &nav_state,
            &entries_store,
            &file_scrolled,
            None,
        ),
    };
    file_scrolled.set_child(Some(&file_box));

    app_state.set_current_tab(Some(tab_id));

    // Connect search entry
    let state_search = nav_state.clone();
    let entries_search = entries_store.clone();
    let file_scrolled_clone = file_scrolled.clone();
    let editor_clone = editor.clone();
    search_entry.connect_changed(move |entry| {
        state_search.borrow_mut().search_query = entry.text().to_string();

        let entries = entries_search.borrow().clone();
        let s = state_search.borrow();
        let view_mode = s.view_mode;
        let show_hidden = s.show_hidden;
        let search = s.search_query.clone();
        let sort_by = s.sort_by;

        let file_box = match view_mode {
            ViewMode::List => build_list_view(
                &entries,
                show_hidden,
                &search,
                sort_by,
                &editor_clone,
                &state_search,
                &entries_search,
                &file_scrolled_clone,
            ),
            ViewMode::Icon => build_icon_view(
                &entries,
                show_hidden,
                &search,
                sort_by,
                &editor_clone,
                &state_search,
                &entries_search,
                &file_scrolled_clone,
                None,
            ),
        };
        file_scrolled_clone.set_child(Some(&file_box));
    });

    println!("[NEW TAB] {} (Tab {:?})", path.display(), tab_id);

    tab_id
}