//! View builders - list view and icon view

use crate::gui::filesystem::filter_and_sort;
use crate::gui::operations::open_file_with_choice;
use crate::gui::types::{FileEntry, NavState, SortBy, ViewMode};
use gtk4::prelude::*;
use gtk4::{Box, Button, Image, Label, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Build list view with clickable rows
pub fn build_list_view(
    entries: &[FileEntry],
    show_hidden: bool,
    search: &str,
    sort_by: SortBy,
    editor: &PathBuf,
    nav_state: &Rc<RefCell<NavState>>,
    entries_store: &Rc<RefCell<Vec<FileEntry>>>,
    file_scrolled: &ScrolledWindow,
) -> Box {
    let file_box = Box::new(Orientation::Vertical, 0);
    file_box.set_vexpand(true);

    let filtered = filter_and_sort(entries, show_hidden, search, sort_by);

    // Show empty folder message if no entries
    if filtered.is_empty() {
        let empty_lbl = Label::new(Some("This folder is empty"));
        empty_lbl.set_margin_top(32);
        empty_lbl.set_margin_bottom(32);
        empty_lbl.set_halign(gtk4::Align::Center);
        file_box.append(&empty_lbl);
        return file_box;
    }

    // Header
    let header = Box::new(Orientation::Horizontal, 0);
    header.set_margin_start(8);
    header.set_margin_end(8);
    header.set_margin_top(4);
    header.set_margin_bottom(4);

    let name_hdr = Label::new(Some("Name"));
    name_hdr.set_halign(gtk4::Align::Start);
    name_hdr.set_hexpand(true);
    header.append(&name_hdr);

    let size_hdr = Label::new(Some("Size"));
    size_hdr.set_width_request(80);
    size_hdr.set_halign(gtk4::Align::End);
    header.append(&size_hdr);

    let type_hdr = Label::new(Some("Type"));
    type_hdr.set_width_request(100);
    type_hdr.set_halign(gtk4::Align::End);
    header.append(&type_hdr);

    file_box.append(&header);

    for entry in filtered {
        let entry_path = entry.path.clone();
        let entry_is_dir = entry.is_dir;

        let row = Box::new(Orientation::Horizontal, 0);
        row.set_vexpand(true);
        row.set_margin_start(8);
        row.set_margin_end(8);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let icon = if entry_is_dir { "📁" } else { "📄" };
        let icon_lbl = Label::new(Some(icon));
        icon_lbl.set_margin_end(12);
        row.append(&icon_lbl);

        let name_lbl = Label::new(Some(&entry.name));
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.set_hexpand(true);
        row.append(&name_lbl);

        let size_str = if entry_is_dir {
            "-".to_string()
        } else if entry.size < 1024 {
            format!("{} B", entry.size)
        } else if entry.size < 1024 * 1024 {
            format!("{:.1} KB", entry.size as f64 / 1024.0)
        } else if entry.size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", entry.size as f64 / (1024.0 * 1024.0))
        } else {
            format!(
                "{:.1} GB",
                entry.size as f64 / (1024.0 * 1024.0 * 1024.0)
            )
        };
        let size_lbl = Label::new(Some(&size_str));
        size_lbl.set_width_request(80);
        size_lbl.set_halign(gtk4::Align::End);
        row.append(&size_lbl);

        let type_str = if entry_is_dir {
            "Folder".to_string()
        } else {
            entry.extension.clone()
        };
        let type_lbl = Label::new(Some(&type_str));
        type_lbl.set_width_request(100);
        type_lbl.set_halign(gtk4::Align::End);
        row.append(&type_lbl);

        // Click handler
        let state_clone = nav_state.clone();
        let entries_clone = entries_store.clone();
        let edit_clone = editor.clone();
        let file_scrolled_clone = file_scrolled.clone();
        let path_is_dir = entry_is_dir;
        let current_target = entry_path.clone();

        let btn = Button::new();
        btn.set_child(Some(&row));

        btn.connect_clicked(move |_| {
            // First: set selection
            {
                let mut s = state_clone.borrow_mut();
                s.selected_paths = vec![current_target.clone()];
                println!("[SELECT] {}", current_target.display());
            }

            // Then: if folder, navigate; if file, open with editor
            if path_is_dir {
                let new_path = current_target.clone();

                {
                    let mut s = state_clone.borrow_mut();
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

                let entries = crate::gui::filesystem::read_directory(&new_path);
                entries_clone.replace(entries.clone());

                // Get view settings
                let (view_mode, show_hidden, search, sort_by) = {
                    let s = state_clone.borrow();
                    (s.view_mode, s.show_hidden, s.search_query.clone(), s.sort_by)
                };

                // Refresh the view
                let file_box = match view_mode {
                    ViewMode::List => build_list_view(
                        &entries,
                        show_hidden,
                        &search,
                        sort_by,
                        &edit_clone,
                        &state_clone,
                        &entries_clone,
                        &file_scrolled_clone,
                    ),
                    ViewMode::Icon => build_icon_view(
                        &entries,
                        show_hidden,
                        &search,
                        sort_by,
                        &edit_clone,
                        &state_clone,
                        &entries_clone,
                        &file_scrolled_clone,
                        None,
                    ),
                };
                file_scrolled_clone.set_child(Some(&file_box));

                println!("[VIEW] {} ({} entries)", new_path.display(), entries.len());
            } else {
                // Open file with editor selection dialog
                open_file_with_choice(current_target.clone(), edit_clone.clone());
            }
        });

        file_box.append(&btn);
    }

    file_box
}

/// Build icon view with FlowBox (like GNOME Files/Nautilus)
pub fn build_icon_view(
    entries: &[FileEntry],
    show_hidden: bool,
    search: &str,
    sort_by: SortBy,
    editor: &PathBuf,
    nav_state: &Rc<RefCell<NavState>>,
    entries_store: &Rc<RefCell<Vec<FileEntry>>>,
    file_scrolled: &ScrolledWindow,
    path_label: Option<&Label>,
) -> Box {
    // Update path label if provided
    let current_path = nav_state.borrow().current_path.clone();
    if let Some(label) = path_label {
        label.set_label(&current_path.display().to_string());
    }

    println!("[GRID VIEW] {} ({} entries)", current_path.display(), entries.len());

    let container = Box::new(Orientation::Vertical, 0);
    container.set_vexpand(true);

    let flow_box = gtk4::FlowBox::new();
    flow_box.set_vexpand(true);
    flow_box.set_halign(gtk4::Align::Center);
    flow_box.set_valign(gtk4::Align::Start);
    flow_box.set_min_children_per_line(4);
    flow_box.set_max_children_per_line(4);
    flow_box.set_homogeneous(true);
    flow_box.set_column_spacing(16);
    flow_box.set_row_spacing(16);
    flow_box.set_margin_start(16);
    flow_box.set_margin_end(16);
    flow_box.set_margin_top(16);
    flow_box.set_margin_bottom(16);

    let filtered = filter_and_sort(entries, show_hidden, search, sort_by);

    // Show empty folder message if no entries
    if filtered.is_empty() {
        let empty_lbl = Label::new(Some("This folder is empty"));
        empty_lbl.set_margin_top(32);
        empty_lbl.set_margin_bottom(32);
        empty_lbl.set_halign(gtk4::Align::Center);
        empty_lbl.set_valign(gtk4::Align::Center);
        container.append(&empty_lbl);
    } else {
        for (index, entry) in filtered.iter().enumerate() {
            let entry_name = entry.name.clone();
            let entry_path = entry.path.clone();
            let is_dir = entry.is_dir;

            let vbox = Box::new(Orientation::Vertical, 2);
            vbox.set_margin_start(4);
            vbox.set_margin_end(4);
            vbox.set_margin_top(4);
            vbox.set_margin_bottom(4);

            // Use GTK themed icons
            let icon_name = if is_dir { "folder-symbolic" } else { "text-x-generic" };
            let icon_img = Image::from_icon_name(icon_name);
            icon_img.set_pixel_size(32);
            icon_img.set_margin_bottom(2);
            vbox.append(&icon_img);

            // Truncate long filenames
            let display_name = if entry_name.len() > 15 {
                format!("{}...", &entry_name[..12])
            } else {
                entry_name.clone()
            };

            let name_lbl = Label::new(Some(&display_name));
            name_lbl.set_wrap(true);
            name_lbl.set_width_request(64);
            name_lbl.set_justify(gtk4::Justification::Center);
            name_lbl.set_halign(gtk4::Align::Center);
            vbox.append(&name_lbl);

            // Click handler
            let state_clone = nav_state.clone();
            let entries_clone = entries_store.clone();
            let edit_clone = editor.clone();
            let file_scrolled_clone = file_scrolled.clone();
            let path_is_dir = is_dir;
            let target = entry_path.clone();
            let current_index = index;

            let btn = Button::new();
            btn.set_child(Some(&vbox));
            btn.set_has_frame(false);
            btn.set_focus_on_click(false);

            btn.connect_clicked(move |_| {
                let is_already_selected = {
                    let s = state_clone.borrow();
                    s.selected_paths.contains(&target)
                };

                {
                    let mut s = state_clone.borrow_mut();
                    if is_already_selected {
                        s.selected_paths.retain(|p| p != &target);
                        println!("[UNSELECT] {}", target.display());
                    } else {
                        s.selected_paths.push(target.clone());
                        s.last_clicked_index = Some(current_index);
                        println!("[SELECT] {} (toggle)", target.display());
                    }
                }

                if path_is_dir && !is_already_selected {
                    let new_path = target.clone();

                    {
                        let mut s = state_clone.borrow_mut();
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

                    let entries = crate::gui::filesystem::read_directory(&new_path);
                    entries_clone.replace(entries.clone());

                    let (view_mode, show_hidden, search, sort_by) = {
                        let s = state_clone.borrow();
                        (s.view_mode, s.show_hidden, s.search_query.clone(), s.sort_by)
                    };

                    let file_box = match view_mode {
                        ViewMode::List => build_list_view(
                            &entries,
                            show_hidden,
                            &search,
                            sort_by,
                            &edit_clone,
                            &state_clone,
                            &entries_clone,
                            &file_scrolled_clone,
                        ),
                        ViewMode::Icon => build_icon_view(
                            &entries,
                            show_hidden,
                            &search,
                            sort_by,
                            &edit_clone,
                            &state_clone,
                            &entries_clone,
                            &file_scrolled_clone,
                            None,
                        ),
                    };
                    file_scrolled_clone.set_child(Some(&file_box));

                    println!("[VIEW] {} ({} entries)", new_path.display(), entries.len());
                } else if !path_is_dir && !is_already_selected {
                    open_file_with_choice(target.clone(), edit_clone.clone());
                }
            });

            flow_box.insert(&btn, -1);
        }
    }

    container.append(&flow_box);
    container
}

/// Refresh file view for a tab
pub fn refresh_tab_view(tab_data: &crate::gui::types::TabData, editor: &PathBuf) {
    let entries = tab_data.entries_store.borrow().clone();

    let (view_mode, show_hidden, search, sort_by, current_path) = {
        let s = tab_data.nav_state.borrow();
        (
            s.view_mode,
            s.show_hidden,
            s.search_query.clone(),
            s.sort_by,
            s.current_path.clone(),
        )
    };

    let file_box = match view_mode {
        ViewMode::List => build_list_view(
            &entries,
            show_hidden,
            &search,
            sort_by,
            editor,
            &tab_data.nav_state,
            &tab_data.entries_store,
            &tab_data.file_scrolled,
        ),
        ViewMode::Icon => build_icon_view(
            &entries,
            show_hidden,
            &search,
            sort_by,
            editor,
            &tab_data.nav_state,
            &tab_data.entries_store,
            &tab_data.file_scrolled,
            Some(&tab_data.path_label),
        ),
    };

    tab_data.file_scrolled.set_child(Some(&file_box));
    tab_data.path_label.set_label(&current_path.display().to_string());

    println!("[REFRESH] {}", current_path.display());
}