---
source: Context7 API
library: gtk4-rs
package: gtk4-rs
topic: Signal handling and callbacks
fetched: 2026-05-02T10:00:00Z
official_docs: https://gtk-rs.github.io/gtk4-rs/stable/latest/docs/gtk/prelude/trait.ButtonExt.html
---

# GTK4 Signal Handling - Clicked Signals and Callbacks

## Button Clicked Signal

Use `connect_clicked()` to handle button clicks:

```rust
use gtk::prelude::*;
use gtk::Button;

// Create button
let button = Button::with_label("Navigate Up");

// Connect to clicked signal - use move closure for ownership
button.connect_clicked(move |btn| {
    println!("Button clicked!");
    // Navigate to parent directory
});

// With data passing
let path = std::path::PathBuf::from("/home/user");
button.connect_clicked(move |_| {
    println!("Navigating to: {:?}", path);
});
```

## Application Activate Signal

```rust
use gtk::Application;

app.connect_activate(|app| {
    // This is called when application starts
    // Create and show window here
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Manager")
        .build();
    window.present();
});
```

## Important: Use `move` Keyword

Signal callbacks require `'static` lifetime, so use `move` to capture ownership:

```rust
// ✅ CORRECT - use move
let directory = current_dir.clone();
button.connect_clicked(move |_| {
    navigate_to(&directory);
});

// ❌ INCORRECT - won't compile (missing move)
// button.connect_clicked(|_| {
//     navigate_to(&directory);  // directory doesn't live long enough
// });
```

## Signal Connection Patterns

### Simple callback (no data):
```rust
button.connect_clicked(|_| {
    println!("Clicked!");
});
```

### Callback with button reference:
```rust
button.connect_clicked(|btn| {
    btn.set_label("Clicked!");
});
```

### Callback with captured data (use move):
```rust
let target_path = PathBuf::from("/some/path");
button.connect_clicked(move |_| {
    println!("Navigate to {:?}", target_path);
});
```

### Using gsignal_connect for custom signals:
```rust
// For non-standard signals
button.connect_closure(
    "clicked",
    glib::closure_local! move |button: &gtk::Button| {
        button.set_label("Clicked!");
    },
);
```

## ListView Selection Signals

```rust
use gtk::SingleSelection;

// Get selection model
let selection = list_view.selection();

// Connect to selection changes
selection.connect_selected_item_notify(|selection| {
    if let Some(item) = selection.selected_item() {
        println!("Selected: {:?}", item);
    }
});
```