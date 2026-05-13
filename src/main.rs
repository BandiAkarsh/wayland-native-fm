//! Wayland File Manager - Phase 4: Advanced Features (Tabs + Bookmarks)
//! Phase 1 + 2 + 3 + 4: Navigation, View, File Ops, Tabs, Bookmarks

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, ComboBoxText, Dialog, Entry, Label,
    Notebook, Orientation, ScrolledWindow, ToggleButton,
};
use std::path::PathBuf;
use std::rc::Rc;

use wayland_file_manager::gui::{
    create_tab, get_default_editor, get_mounted_drives, get_preferred_editor, load_bookmarks,
    open_file_with_choice, refresh_tab_view, AppState, SortBy, TabData, ViewMode,
};

fn main() {
    let editor = get_preferred_editor().unwrap_or_else(get_default_editor);

    let app = Application::new(None::<&str>, gtk4::gio::ApplicationFlags::empty());

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Files")
            .default_width(1000)
            .default_height(650)
            .build();

        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home"));

        let app_state = Rc::new(AppState::new());
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_scrollable(true);

        // Create first tab
        let _first_tab_id = create_tab(&app_state, &notebook, home.clone(), &editor);

        let hbox = Box::new(Orientation::Horizontal, 0);

        // ===== SIDEBAR =====
        let sidebar = Box::new(Orientation::Vertical, 0);
        sidebar.set_width_request(200);

        let places_lbl = Label::new(Some("📁  Places"));
        places_lbl.set_margin_start(16);
        places_lbl.set_margin_top(16);
        places_lbl.set_margin_bottom(12);
        sidebar.append(&places_lbl);

        let make_place =
            |label_text: &str, target: PathBuf, app_state: Rc<AppState>, editor: PathBuf| -> Button {
                let btn = Button::new();
                btn.set_label(label_text);
                btn.set_halign(gtk4::Align::Start);
                btn.set_margin_start(8);
                btn.set_margin_end(8);
                btn.set_margin_bottom(4);

                let app_state_clone = app_state.clone();
                let editor_clone = editor.clone();

                btn.connect_clicked(move |_| {
                    app_state_clone.navigate_current_tab(&editor_clone, target.clone());
                });
                btn
            };

        let app_state_clone = app_state.clone();
        let editor_clone = editor.clone();

        sidebar.append(&make_place(
            "🏠  Home",
            home.clone(),
            app_state_clone.clone(),
            editor_clone.clone(),
        ));

        if let Some(d) = dirs::desktop_dir() {
            sidebar.append(&make_place("🖥️  Desktop", d, app_state_clone.clone(), editor_clone.clone()));
        }
        if let Some(d) = dirs::document_dir() {
            sidebar.append(&make_place("📄  Documents", d, app_state_clone.clone(), editor_clone.clone()));
        }
        if let Some(d) = dirs::download_dir() {
            sidebar.append(&make_place("📥  Downloads", d, app_state_clone.clone(), editor_clone.clone()));
        }
        if let Some(d) = dirs::audio_dir() {
            sidebar.append(&make_place("🎵  Music", d, app_state_clone.clone(), editor_clone.clone()));
        }
        if let Some(d) = dirs::picture_dir() {
            sidebar.append(&make_place("🖼️  Pictures", d, app_state_clone.clone(), editor_clone.clone()));
        }
        if let Some(d) = dirs::video_dir() {
            sidebar.append(&make_place("🎬  Videos", d, app_state_clone.clone(), editor_clone.clone()));
        }

        // ===== MOUNTED DRIVES / USB =====
        let drives = get_mounted_drives();
        if !drives.is_empty() {
            let devices_lbl = Label::new(Some("💾  Devices"));
            devices_lbl.set_margin_start(16);
            devices_lbl.set_margin_top(16);
            devices_lbl.set_margin_bottom(12);
            sidebar.append(&devices_lbl);

            for (name, path) in &drives {
                let btn = Button::new();
                btn.set_label(name);
                btn.set_halign(gtk4::Align::Start);
                btn.set_margin_start(8);
                btn.set_margin_end(8);
                btn.set_margin_bottom(4);

                let path_clone = path.clone();
                let app_dr = app_state_clone.clone();
                let edit_dr = editor.clone();

                btn.connect_clicked(move |_| {
                    app_dr.navigate_current_tab(&edit_dr, path_clone.clone());
                });
                sidebar.append(&btn);
            }
        }

        // Trash
        let trash = Button::new();
        trash.set_label("🗑️  Trash");
        trash.set_halign(gtk4::Align::Start);
        trash.set_margin_start(8);
        trash.set_margin_end(8);
        trash.set_margin_top(16);
        sidebar.append(&trash);

        // ===== BOOKMARKS =====
        let bookmarks_lbl = Label::new(Some("🔖  Bookmarks"));
        bookmarks_lbl.set_margin_start(16);
        bookmarks_lbl.set_margin_top(16);
        bookmarks_lbl.set_margin_bottom(12);
        sidebar.append(&bookmarks_lbl);

        let bookmarks = load_bookmarks();
        let app_state_bm = app_state.clone();
        let editor_bm = editor.clone();

        for bookmark in &bookmarks {
            let btn = Button::new();
            btn.set_label(&format!("🔖 {}", bookmark.name));
            btn.set_halign(gtk4::Align::Start);
            btn.set_margin_start(8);
            btn.set_margin_end(8);
            btn.set_margin_bottom(4);

            let path = bookmark.path.clone();
            let app_state_clone = app_state_bm.clone();
            let editor_clone = editor_bm.clone();

            btn.connect_clicked(move |_| {
                app_state_clone.navigate_current_tab(&editor_clone, path.clone());
            });

            sidebar.append(&btn);
        }

        // Add Bookmark button
        let add_bm_btn = Button::new();
        add_bm_btn.set_label("+ Add Bookmark");
        add_bm_btn.set_halign(gtk4::Align::Start);
        add_bm_btn.set_margin_start(8);
        add_bm_btn.set_margin_end(8);
        add_bm_btn.set_margin_top(8);

        let app_state_add_bm = app_state.clone();
        add_bm_btn.connect_clicked(move |_| {
            let dialog = Dialog::new();
            dialog.set_title(Some("Add Bookmark"));
            dialog.set_default_size(400, 200);

            let content = dialog.content_area();
            let vbox = Box::new(Orientation::Vertical, 8);
            vbox.set_margin_start(16);
            vbox.set_margin_end(16);
            vbox.set_margin_top(16);
            vbox.set_margin_bottom(16);

            let lbl = Label::new(Some(
                "Add current folder to bookmarks, or click Browse to select a different folder:",
            ));
            vbox.append(&lbl);

            let current_path = if let Some(tab_data) = app_state_add_bm.get_current_tab() {
                tab_data.nav_state.borrow().current_path.clone()
            } else {
                dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home"))
            };

            let path_lbl = Label::new(Some(&current_path.display().to_string()));
            path_lbl.set_margin_top(8);
            vbox.append(&path_lbl);

            // Browse button (stub)
            let browse_btn = Button::new();
            browse_btn.set_label("Browse...");
            browse_btn.set_margin_top(8);

            let dialog_browse = dialog.clone();
            browse_btn.connect_clicked(move |_| {
                dialog_browse.close();
            });
            vbox.append(&browse_btn);

            // Add button
            let add_btn = Button::new();
            add_btn.set_label("Add Bookmark");
            add_btn.set_margin_top(16);

            let current_path_clone = current_path.clone();
            let dialog_add = dialog.clone();

            add_btn.connect_clicked(move |_| {
                let path = current_path_clone.clone();
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                let new_bookmark = wayland_file_manager::gui::types::Bookmark {
                    name: name.clone(),
                    path: path.clone(),
                };
                let mut bookmarks = load_bookmarks();

                if !bookmarks.iter().any(|b| b.path == path) {
                    bookmarks.push(new_bookmark);
                    if let Err(e) = wayland_file_manager::gui::save_bookmarks(&bookmarks) {
                        println!("[ERROR] Failed to save bookmarks: {}", e);
                    }
                    println!("[BOOKMARK] Added: {}", name);
                } else {
                    println!("[BOOKMARK] Already bookmarked: {}", name);
                }

                dialog_add.close();
            });
            vbox.append(&add_btn);

            // Cancel button
            let cancel_btn = Button::new();
            cancel_btn.set_label("Cancel");
            let dialog_cancel = dialog.clone();
            cancel_btn.connect_clicked(move |_| {
                dialog_cancel.close();
            });
            vbox.append(&cancel_btn);

            content.append(&vbox);
            dialog.show();
        });
        sidebar.append(&add_bm_btn);

        hbox.append(&sidebar);

        // ===== CONTENT =====
        let content = Box::new(Orientation::Vertical, 0);

        // Toolbar
        let toolbar = Box::new(Orientation::Horizontal, 0);
        toolbar.set_height_request(48);
        toolbar.set_margin_bottom(8);

        // Back
        let back_btn = Button::new();
        back_btn.set_label("←");
        back_btn.set_tooltip_text(Some("Back"));
        toolbar.append(&back_btn);

        // Forward
        let fwd_btn = Button::new();
        fwd_btn.set_label("→");
        fwd_btn.set_tooltip_text(Some("Forward"));
        toolbar.append(&fwd_btn);

        // Up
        let up_btn = Button::new();
        up_btn.set_label("↑");
        up_btn.set_tooltip_text(Some("Parent Directory"));
        toolbar.append(&up_btn);

        // New folder button
        let new_btn = Button::new();
        new_btn.set_label("➕");
        new_btn.set_tooltip_text(Some("New Folder (Ctrl+Shift+N)"));
        toolbar.append(&new_btn);

        // Delete button
        let delete_btn = Button::new();
        delete_btn.set_label("🗑");
        delete_btn.set_tooltip_text(Some("Delete (Delete key)"));
        toolbar.append(&delete_btn);

        // Context menu button
        let context_btn = Button::new();
        context_btn.set_label("☰");
        context_btn.set_tooltip_text(Some("Context Menu"));
        toolbar.append(&context_btn);

        // Context menu
        let app_state_context = app_state.clone();
        let editor_context = editor.clone();

        context_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_context.get_current_tab() {
                let selected = tab_data.nav_state.borrow().selected_paths.clone();

                if selected.is_empty() {
                    println!("[CONTEXT] No item selected");
                    return;
                }

                let dialog = Dialog::new();
                dialog.set_title(Some("Context Menu"));
                dialog.set_default_size(250, 200);

                let content = dialog.content_area();
                let vbox = Box::new(Orientation::Vertical, 4);
                vbox.set_margin_start(16);
                vbox.set_margin_end(16);
                vbox.set_margin_top(16);
                vbox.set_margin_bottom(16);

                let count = selected.len();
                let lbl = Label::new(Some(&format!("{} item(s) selected", count)));
                vbox.append(&lbl);

                // Open button
                let open_btn = Button::new();
                open_btn.set_label("Open");
                let app_open = app_state_context.clone();
                let editor_open = editor_context.clone();
                let selected_open = selected.clone();
                let dialog_open = dialog.clone();

                open_btn.connect_clicked(move |_| {
                    for path in &selected_open {
                        if path.is_dir() {
                            if let Some(tab) = app_open.get_current_tab() {
                                let entries = wayland_file_manager::gui::filesystem::read_directory(path);
                                tab.entries_store.replace(entries.clone());
                                tab.nav_state.borrow_mut().current_path = path.clone();
                                tab.nav_state.borrow_mut().history.push(path.clone());
                                tab.nav_state.borrow_mut().history_index =
                                    tab.nav_state.borrow().history.len() - 1;
                                refresh_tab_view(&tab, &editor_open);
                            }
                        } else {
                            open_file_with_choice(path.clone(), editor_open.clone());
                        }
                    }
                    dialog_open.close();
                });
                vbox.append(&open_btn);

                // Copy path button
                let copy_btn = Button::new();
                copy_btn.set_label("Copy Path");
                let selected_copy = selected.clone();
                let dialog_copy = dialog.clone();

                copy_btn.connect_clicked(move |_| {
                    for path in &selected_copy {
                        println!("[COPY PATH] {}", path.display());
                    }
                    println!("[CONTEXT] Path copied to clipboard (logged)");
                    dialog_copy.close();
                });
                vbox.append(&copy_btn);

                // Properties button
                let props_btn = Button::new();
                props_btn.set_label("Properties");
                let selected_props = selected.clone();
                let dialog_props = dialog.clone();

                props_btn.connect_clicked(move |_| {
                    if let Some(path) = selected_props.first() {
                        let dialog2 = Dialog::new();
                        dialog2.set_title(Some("Properties"));
                        dialog2.set_default_size(300, 200);

                        let content = dialog2.content_area();
                        let vbox2 = Box::new(Orientation::Vertical, 8);
                        vbox2.set_margin_start(16);
                        vbox2.set_margin_end(16);
                        vbox2.set_margin_top(16);
                        vbox2.set_margin_bottom(16);

                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "Unknown".to_string());
                        let is_dir = path.is_dir();
                        let size = if is_dir {
                            0
                        } else {
                            path.metadata().map(|m| m.len()).unwrap_or(0)
                        };
                        let modified = path.metadata().ok().and_then(|m| m.modified().ok());

                        let name_lbl = Label::new(Some(&format!("Name: {}", name)));
                        vbox2.append(&name_lbl);

                        let type_lbl =
                            Label::new(Some(&format!("Type: {}", if is_dir { "Folder" } else { "File" })));
                        vbox2.append(&type_lbl);

                        let size_lbl = Label::new(Some(&format!("Size: {} bytes", size)));
                        vbox2.append(&size_lbl);

                        if let Some(time) = modified {
                            let datetime =
                                chrono::DateTime::<chrono::Local>::from(time).format("%Y-%m-%d %H:%M:%S").to_string();
                            let mod_lbl = Label::new(Some(&format!("Modified: {}", datetime)));
                            vbox2.append(&mod_lbl);
                        }

                        let path_lbl = Label::new(Some(&format!("Path: {}", path.display())));
                        vbox2.append(&path_lbl);

                        let close2_btn = Button::new();
                        close2_btn.set_label("Close");
                        let dialog2_close = dialog2.clone();
                        close2_btn.connect_clicked(move |_| {
                            dialog2_close.close();
                        });
                        vbox2.append(&close2_btn);

                        content.append(&vbox2);
                        dialog2.show();
                    }
                    dialog_props.close();
                });
                vbox.append(&props_btn);

                // Cut button
                let cut_btn = Button::new();
                cut_btn.set_label("Cut");
                let selected_cut = selected.clone();
                let dialog_cut = dialog.clone();

                cut_btn.connect_clicked(move |_| {
                    println!("[CUT] {} items", selected_cut.len());
                    for path in &selected_cut {
                        println!("[CUT] {}", path.display());
                    }
                    dialog_cut.close();
                });
                vbox.append(&cut_btn);

                // Copy button
                let copy_btn = Button::new();
                copy_btn.set_label("Copy");
                let selected_copy = selected.clone();
                let dialog_copy = dialog.clone();

                copy_btn.connect_clicked(move |_| {
                    println!("[COPY] {} items", selected_copy.len());
                    for path in &selected_copy {
                        println!("[COPY] {}", path.display());
                    }
                    dialog_copy.close();
                });
                vbox.append(&copy_btn);

                // Close button
                let close_btn = Button::new();
                close_btn.set_label("Close");
                let dialog_close = dialog.clone();
                close_btn.connect_clicked(move |_| {
                    dialog_close.close();
                });
                vbox.append(&close_btn);

                content.append(&vbox);
                dialog.show();
            }
        });

        // Rename button
        let rename_btn = Button::new();
        rename_btn.set_label("✏");
        rename_btn.set_tooltip_text(Some("Rename (F2)"));
        toolbar.append(&rename_btn);

        // New tab button
        let new_tab_btn = Button::new();
        new_tab_btn.set_label("+");
        new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
        toolbar.append(&new_tab_btn);

        let path_label = Label::new(Some(""));
        path_label.set_margin_start(16);
        toolbar.append(&path_label);

        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        toolbar.append(&spacer);

        // View toggle
        let view_toggle = ToggleButton::new();
        view_toggle.set_label("▦");
        view_toggle.set_active(true);
        view_toggle.set_tooltip_text(Some("Toggle List/Icon View"));
        toolbar.append(&view_toggle);

        // Hidden toggle
        let hidden_toggle = ToggleButton::new();
        hidden_toggle.set_label("👁");
        hidden_toggle.set_active(true);
        hidden_toggle.set_tooltip_text(Some("Show/Hide Hidden Files"));
        toolbar.append(&hidden_toggle);

        // Sort
        let sort_combo = ComboBoxText::new();
        sort_combo.append(Some("name"), "Name");
        sort_combo.append(Some("date"), "Date");
        sort_combo.append(Some("size"), "Size");
        sort_combo.append(Some("type"), "Type");
        sort_combo.set_active_id(Some("name"));
        toolbar.append(&sort_combo);

        // Preview toggle
        let preview_toggle = ToggleButton::new();
        preview_toggle.set_label("👁");
        preview_toggle.set_active(false);
        preview_toggle.set_tooltip_text(Some("Toggle Preview Panel"));
        toolbar.append(&preview_toggle);

        content.append(&toolbar);
        content.append(&notebook);

        let main_area = Box::new(Orientation::Horizontal, 0);
        main_area.append(&content);

        // Preview panel (hidden)
        let preview_panel = Box::new(Orientation::Vertical, 0);
        preview_panel.set_width_request(300);
        preview_panel.set_margin_start(8);
        preview_panel.set_visible(false);

        let preview_lbl = Label::new(Some("Preview"));
        preview_lbl.set_margin_start(16);
        preview_lbl.set_margin_top(16);
        preview_lbl.set_margin_bottom(8);
        preview_panel.append(&preview_lbl);

        let preview_content = ScrolledWindow::new();
        preview_content.set_vexpand(true);
        let preview_text = Label::new(Some("Select a file to preview"));
        preview_text.set_margin_start(16);
        preview_text.set_margin_end(16);
        preview_text.set_margin_top(16);
        preview_text.set_wrap(true);
        preview_content.set_child(Some(&preview_text));
        preview_panel.append(&preview_content);

        main_area.append(&preview_panel);
        hbox.append(&main_area);

        window.set_child(Some(&hbox));

        // ===== SIGNALS =====

        // Back button
        let app_state_back = app_state.clone();
        let editor_back = editor.clone();

        back_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_back.get_current_tab() {
                let prev_path: Option<PathBuf>;
                let current_target: PathBuf;
                {
                    let s = tab_data.nav_state.borrow();
                    prev_path = if s.history_index > 0 {
                        Some(s.history[s.history_index - 1].clone())
                    } else {
                        None
                    };
                    current_target = s.current_path.clone();
                }

                if let Some(target) = prev_path {
                    {
                        let mut s = tab_data.nav_state.borrow_mut();
                        s.current_path = target.clone();
                        s.history_index -= 1;
                    }

                    let entries = wayland_file_manager::gui::filesystem::read_directory(&target);
                    tab_data.entries_store.replace(entries.clone());

                    tab_data.path_label.set_label(&target.display().to_string());
                    tab_data.search_entry.set_text("");

                    refresh_tab_view(&tab_data, &editor_back);
                    println!("[BACK] {} -> {}", current_target.display(), target.display());
                }
            }
        });

        // Forward button
        let app_state_fwd = app_state.clone();
        let editor_fwd = editor.clone();

        fwd_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_fwd.get_current_tab() {
                let next_path: Option<PathBuf>;
                let current_target: PathBuf;
                {
                    let s = tab_data.nav_state.borrow();
                    let hist_len = s.history.len();
                    next_path = if s.history_index < hist_len - 1 {
                        Some(s.history[s.history_index + 1].clone())
                    } else {
                        None
                    };
                    current_target = s.current_path.clone();
                }

                if let Some(target) = next_path {
                    {
                        let mut s = tab_data.nav_state.borrow_mut();
                        s.current_path = target.clone();
                        s.history_index += 1;
                    }

                    let entries = wayland_file_manager::gui::filesystem::read_directory(&target);
                    tab_data.entries_store.replace(entries.clone());

                    tab_data.path_label.set_label(&target.display().to_string());
                    tab_data.search_entry.set_text("");

                    refresh_tab_view(&tab_data, &editor_fwd);
                    println!("[FORWARD] {} -> {}", current_target.display(), target.display());
                }
            }
        });

        // Up button
        let app_state_up = app_state.clone();
        let editor_up = editor.clone();

        up_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_up.get_current_tab() {
                let parent: Option<PathBuf> = {
                    let s = tab_data.nav_state.borrow();
                    s.current_path.parent().map(|p| p.to_path_buf())
                };

                if let Some(target) = parent {
                    {
                        let mut s = tab_data.nav_state.borrow_mut();
                        let index = s.history_index;
                        let hist_len = s.history.len();

                        if index < hist_len - 1 {
                            s.history.truncate(index + 1);
                        }

                        if !s.history.contains(&target) {
                            s.history.push(target.clone());
                        }

                        s.current_path = target.clone();
                        s.history_index = s.history.len() - 1;
                    }

                    let entries = wayland_file_manager::gui::filesystem::read_directory(&target);
                    tab_data.entries_store.replace(entries.clone());

                    tab_data.path_label.set_label(&target.display().to_string());
                    tab_data.search_entry.set_text("");

                    refresh_tab_view(&tab_data, &editor_up);
                    println!("[UP] {}", target.display());
                }
            }
        });

        // New folder button
        let app_state_new = app_state.clone();
        let editor_new = editor.clone();

        new_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_new.get_current_tab() {
                let current_path = tab_data.nav_state.borrow().current_path.clone();

                let mut counter = 1;
                loop {
                    let folder_name = if counter == 1 {
                        "New Folder".to_string()
                    } else {
                        format!("New Folder {}", counter)
                    };
                    let folder_path = current_path.join(&folder_name);

                    if !folder_path.exists() {
                        if std::fs::create_dir(&folder_path).is_ok() {
                            println!("[NEW FOLDER] {}", folder_path.display());

                            let entries = wayland_file_manager::gui::filesystem::read_directory(&current_path);
                            tab_data.entries_store.replace(entries.clone());

                            refresh_tab_view(&tab_data, &editor_new);
                        }
                        break;
                    }
                    counter += 1;

                    if counter > 1000 {
                        println!("[ERROR] Could not create new folder");
                        break;
                    }
                }
            }
        });

        // Delete button
        let app_state_delete = app_state.clone();
        let editor_delete = editor.clone();

        delete_btn.connect_clicked(move |_| {
            println!("[DELETE] ========== BUTTON CLICKED ==========");
            let tab_opt = app_state_delete.get_current_tab();
            if let Some(tab_data) = tab_opt {
                let tab_id = tab_data._id;
                let current = tab_data.nav_state.borrow().current_path.clone();
                let selected = tab_data.nav_state.borrow().selected_paths.clone();
                println!("[DELETE] Tab: {:?}, path: {}", tab_id, current.display());
                println!(
                    "[DELETE] Selected: {:?}",
                    selected
                        .iter()
                        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                );

                if !selected.is_empty() {
                    let path = &selected[0];

                    let dialog = Dialog::new();
                    dialog.set_title(Some("Delete"));
                    dialog.set_default_size(300, 120);

                    let content = dialog.content_area();
                    let vbox = Box::new(Orientation::Vertical, 8);
                    vbox.set_margin_start(16);
                    vbox.set_margin_end(16);
                    vbox.set_margin_top(16);
                    vbox.set_margin_bottom(16);

                    let filename = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "this".to_string());

                    let lbl = Label::new(Some(&format!("Move '{}' to trash?", filename)));
                    vbox.append(&lbl);

                    let delete_yes_btn = Button::new();
                    delete_yes_btn.set_label("Move to Trash");

                    let dialog_close = dialog.clone();
                    let path_for_delete = path.clone();
                    let tab_data_delete = tab_data.clone();
                    let editor_del = editor_delete.clone();

                    delete_yes_btn.connect_clicked(move |_| {
                        if wayland_file_manager::gui::filesystem::move_to_trash(&path_for_delete) {
                            let current_path = tab_data_delete.nav_state.borrow().current_path.clone();
                            let entries = wayland_file_manager::gui::filesystem::read_directory(&current_path);
                            tab_data_delete.entries_store.replace(entries.clone());
                            tab_data_delete.nav_state.borrow_mut().selected_paths = vec![];
                            refresh_tab_view(&tab_data_delete, &editor_del);
                        }
                        dialog_close.close();
                    });
                    vbox.append(&delete_yes_btn);

                    let cancel_btn = Button::new();
                    cancel_btn.set_label("Cancel");
                    let dialog_cancel = dialog.clone();
                    cancel_btn.connect_clicked(move |_| {
                        dialog_cancel.close();
                    });
                    vbox.append(&cancel_btn);

                    content.append(&vbox);
                    dialog.show();
                } else {
                    println!("[DELETE] No item selected - click to select first");
                }
            }
        });

        // Rename button
        let app_state_rename = app_state.clone();
        let editor_rename = editor.clone();

        rename_btn.connect_clicked(move |_| {
            if let Some(tab_data) = app_state_rename.get_current_tab() {
                let selected = tab_data.nav_state.borrow().selected_paths.clone();

                if !selected.is_empty() {
                    let path = &selected[0];

                    let dialog = Dialog::new();
                    dialog.set_title(Some("Rename"));
                    dialog.set_default_size(350, 150);

                    let content = dialog.content_area();
                    let vbox = Box::new(Orientation::Vertical, 8);
                    vbox.set_margin_start(16);
                    vbox.set_margin_end(16);
                    vbox.set_margin_top(16);
                    vbox.set_margin_bottom(16);

                    let old_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "".to_string());

                    let lbl = Label::new(Some("Enter new name:"));
                    vbox.append(&lbl);

                    let entry = Entry::new();
                    entry.set_text(&old_name);
                    entry.set_width_request(300);
                    vbox.append(&entry);

                    let rename_inner_btn = Button::new();
                    rename_inner_btn.set_label("Rename");

                    let dialog_close = dialog.clone();
                    let path_for_rename = path.clone();
                    let tab_data_rename = tab_data.clone();
                    let editor_ren = editor_rename.clone();

                    rename_inner_btn.connect_clicked(move |_| {
                        let text = entry.text();
                        let new_name = text.trim().to_string();
                        if !new_name.is_empty() {
                            if let Some(_) = wayland_file_manager::gui::filesystem::rename_file(&path_for_rename, &new_name) {
                                let current_path = tab_data_rename.nav_state.borrow().current_path.clone();
                                let entries = wayland_file_manager::gui::filesystem::read_directory(&current_path);
                                tab_data_rename.entries_store.replace(entries.clone());
                                tab_data_rename.nav_state.borrow_mut().selected_paths = vec![];
                                refresh_tab_view(&tab_data_rename, &editor_ren);
                            } else {
                                println!("[RENAME] Failed - name may already exist");
                            }
                        }
                        dialog_close.close();
                    });
                    vbox.append(&rename_inner_btn);

                    let cancel_btn = Button::new();
                    cancel_btn.set_label("Cancel");
                    let dialog_cancel = dialog.clone();
                    cancel_btn.connect_clicked(move |_| {
                        dialog_cancel.close();
                    });
                    vbox.append(&cancel_btn);

                    content.append(&vbox);
                    dialog.show();
                } else {
                    println!("[RENAME] No item selected - click to select first");
                }
            }
        });

        // New tab button
        let app_state_new_tab = app_state.clone();
        let notebook_new_tab = notebook.clone();
        let editor_new_tab = editor.clone();

        new_tab_btn.connect_clicked(move |_| {
            let path = if let Some(tab_data) = app_state_new_tab.get_current_tab() {
                tab_data.nav_state.borrow().current_path.clone()
            } else {
                dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home"))
            };
            create_tab(&app_state_new_tab, &notebook_new_tab, path, &editor_new_tab);
        });

        // View toggle
        let app_state_view = app_state.clone();
        let editor_view = editor.clone();

        view_toggle.connect_toggled(move |btn| {
            if let Some(tab_data) = app_state_view.get_current_tab() {
                if btn.is_active() {
                    tab_data.nav_state.borrow_mut().view_mode = ViewMode::Icon;
                    btn.set_label("▦");
                } else {
                    tab_data.nav_state.borrow_mut().view_mode = ViewMode::List;
                    btn.set_label("☰");
                }

                refresh_tab_view(&tab_data, &editor_view);
            }
        });

        // Hidden toggle
        let app_state_hidden = app_state.clone();
        let editor_hidden = editor.clone();

        hidden_toggle.connect_toggled(move |btn| {
            if let Some(tab_data) = app_state_hidden.get_current_tab() {
                tab_data.nav_state.borrow_mut().show_hidden = btn.is_active();

                if let Some(tab_data) = app_state_hidden.get_current_tab() {
                    refresh_tab_view(&tab_data, &editor_hidden);
                }
                println!(
                    "[HIDDEN] {} all files",
                    if btn.is_active() { "showing" } else { "hiding" }
                );
            }
        });

        // Sort combo
        let app_state_sort = app_state.clone();
        let editor_sort = editor.clone();

        sort_combo.connect_changed(move |combo| {
            if let Some(tab_data) = app_state_sort.get_current_tab() {
                let sort_by = match combo.active_id().as_deref() {
                    Some("date") => SortBy::Date,
                    Some("size") => SortBy::Size,
                    Some("type") => SortBy::Type,
                    _ => SortBy::Name,
                };

                tab_data.nav_state.borrow_mut().sort_by = sort_by;
                refresh_tab_view(&tab_data, &editor_sort);
                println!("[SORT] {:?}", sort_by);
            }
        });

        // Notebook page switch
        let app_state_switch = app_state.clone();
        let view_toggle_switch = view_toggle.clone();
        let hidden_toggle_switch = hidden_toggle.clone();
        let sort_combo_switch = sort_combo.clone();
        let path_label_switch = path_label.clone();

        notebook.connect_switch_page(move |_, page_widget, _page_num| {
            let page_widget_ptr = page_widget as *const _ as usize;
            let mut tab_data_clone: Option<TabData> = None;
            let mut found_id: Option<wayland_file_manager::gui::types::TabId> = None;

            {
                let tabs = app_state_switch.tabs.borrow();
                for (id, tab_data) in tabs.iter() {
                    let content_box_ptr = &tab_data.content_box as *const _ as usize;
                    if content_box_ptr == page_widget_ptr {
                        found_id = Some(*id);
                        tab_data_clone = Some(tab_data.clone());
                        break;
                    }
                }
            }

            if let Some(id) = found_id {
                println!("[SWITCH] Switching to tab {:?}", id);
                app_state_switch.set_current_tab(Some(id));
            }

            if let Some(tab_data) = tab_data_clone {
                let s = tab_data.nav_state.borrow();
                let path_str = s.current_path.display().to_string();
                let view_mode = s.view_mode;
                let show_hidden = s.show_hidden;
                let sort_by = s.sort_by;
                drop(s);

                println!("[SWITCH] Setting toolbar for: {}", path_str);
                path_label_switch.set_label(&path_str);
                tab_data.search_entry.set_text("");

                view_toggle_switch.set_active(view_mode == ViewMode::Icon);
                view_toggle_switch.set_label(if view_mode == ViewMode::Icon { "▦" } else { "☰" });
                hidden_toggle_switch.set_active(show_hidden);

                match sort_by {
                    SortBy::Name => {
                        sort_combo_switch.set_active_id(Some("name"));
                    }
                    SortBy::Date => {
                        sort_combo_switch.set_active_id(Some("date"));
                    }
                    SortBy::Size => {
                        sort_combo_switch.set_active_id(Some("size"));
                    }
                    SortBy::Type => {
                        sort_combo_switch.set_active_id(Some("type"));
                    }
                }
            }
        });

        println!("[START] Editor: {}", editor.display());
        window.present();
    });

    app.run();
}