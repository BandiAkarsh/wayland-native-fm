---
source: Context7 API
library: gtk4-rs
package: gtk4-rs
topic: Sidebar + file list layout patterns
fetched: 2026-05-02T10:00:00Z
official_docs: https://gtk-rs.github.io/gtk4-rs/stable/latest/docs/gtk/struct.Paned.html
---

# GTK4 Modern Layout Patterns - Sidebar + File List

## Pattern 1: Using GtkPaned for Resizable Split View

The recommended way to create a sidebar + content layout with a draggable divider:

```rust
use gtk::prelude::*;
use gtk::{ApplicationWindow, Paned, ScrolledWindow, ListView, Orientation};

fn create_file_manager_layout(app: &Application) -> ApplicationWindow {
    // Create the main horizontal Paned (sidebar | content)
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .wide_handle(true)  // Makes the handle easier to grab
        .build();

    // === SIDEBAR (Left) ===
    let sidebar_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)  // Hide horizontal scrollbar
        .child(&sidebar_list_view)
        .build();

    // Set sidebar as the start (left) child
    paned.set_start_child(Some(&sidebar_scrolled));
    paned.set_start_child_resize(true);   // Sidebar can resize
    paned.set_start_child_shrink(false);  // Don't let it shrink below min size

    // === FILE LIST (Right) ===
    let file_list_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Automatic)
        .child(&file_list_view)
        .build();

    // Set file list as the end (right) child
    paned.set_end_child(Some(&file_list_scrolled));
    paned.set_end_child_resize(true);
    paned.set_end_child_shrink(false);

    // Create window with Paned as child
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Manager")
        .default_width(900)
        .default_height(600)
        .child(&paned)
        .build();

    window
}
```

## Pattern 2: Using GtkBox for Fixed Layout

If you don't need a resizable divider:

```rust
use gtk::{Box, Orientation};

let hbox = Box::builder()
    .orientation(Orientation::Horizontal)
    .spacing(0)  // No spacing between sidebar and content
    .build();

// Sidebar with fixed width
let sidebar = Box::builder()
    .orientation(Orientation::Vertical)
    .width_request(200)  // Fixed width sidebar
    .build();

// File list - takes remaining space
let content = Box::builder()
    .orientation(Orientation::Vertical)
    .build();

hbox.append(&sidebar);
hbox.append(&content);
```

## Pattern 3: Using AdwNavigationSplitView (libadwaita)

For modern GNOME-style layout with built-in sidebar support:

```rust
use adw::prelude::*;
use adw::NavigationSplitView;

// Requires adw crate (libadwaita)
let split_view = NavigationSplitView::builder()
    .sidebar(&sidebar_page)
    .content(&content_page)
    .build();

// Sidebar automatically collapses on small screens
split_view.set_show_sidebar(true);
```

## ListView for File Display

```rust
use gtk::{
    ListView, SingleSelection, SignalListItemFactory,
    ScrolledWindow, PolicyType, ListItem, Label
};

fn create_file_list(model: &gio::ListStore) -> ListView {
    let factory = SignalListItemFactory::new();

    // Setup: create widget for each row
    factory.connect_setup(|_, list_item| {
        let label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    // Bind: connect model data to widget
    factory.connect_bind(|_, list_item| {
        let list_item = list_item.downcast_ref::<ListItem>().unwrap();
        if let Some(file) = list_item.item().and_downcast::<FileObject>() {
            let label = list_item.child().and_downcast::<Label>().unwrap();
            label.set_text(&file.name());
        }
    });

    let selection = SingleSelection::new(Some(model.clone()));
    let list_view = ListView::new(Some(selection), Some(factory));

    // MUST be in ScrolledWindow
    list_view
}

// Create ScrolledWindow wrapper
let scrolled = ScrolledWindow::builder()
    .hscrollbar_policy(PolicyType::Never)
    .min_content_width(360)
    .child(&list_view)
    .build();
```

## Complete Example: File Manager Layout

```rust
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Paned, Box, Orientation,
    ScrolledWindow, PolicyType, ListView, SingleSelection,
    SignalListItemFactory, Button
};

fn build_file_manager_ui(app: &Application) {
    // Create sidebar with navigation buttons
    let sidebar = create_sidebar();
    
    // Create file list
    let file_list = create_file_list_view();
    
    // Create horizontal paned layout
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .wide_handle(true)
        .build();
    
    // Sidebar (left)
    let sidebar_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&sidebar)
        .build();
    paned.set_start_child(Some(&sidebar_scroll));
    paned.set_start_child_resize(true);
    paned.set_start_child_shrink(false);
    
    // File list (right)
    let file_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Automatic)
        .child(&file_list)
        .build();
    paned.set_end_child(Some(&file_scroll));
    paned.set_end_child_resize(true);
    paned.set_end_child_shrink(false);
    
    // Create window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Manager")
        .default_width(900)
        .default_height(600)
        .child(&paned)
        .build();
    
    window.present();
}

fn create_sidebar() -> Box {
    let sidebar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .build();
    
    // Add navigation buttons
    let home_btn = Button::with_label("Home");
    let desktop_btn = Button::with_label("Desktop");
    let documents_btn = Button::with_label("Documents");
    
    sidebar.append(&home_btn);
    sidebar.append(&desktop_btn);
    sidebar.append(&documents_btn);
    
    sidebar
}
```