---
source: Context7 API
library: gtk4-rs
package: gtk4-rs
topic: ApplicationWindow constructor and API
fetched: 2026-05-02T10:00:00Z
official_docs: https://gtk-rs.github.io/gtk4-rs/stable/latest/docs/gtk/struct.ApplicationWindow.html
---

# GTK4 ApplicationWindow - Correct Constructor and API Usage

## Key Finding: Use Builder Pattern (NOT direct constructor)

In gtk4-rs, the recommended way to create an `ApplicationWindow` is using the **builder pattern**, NOT direct struct construction.

### ✅ CORRECT - Using Builder

```rust
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.FileManager")
        .build();

    app.connect_activate(|app| {
        // Use ApplicationWindow::builder() - NOT ApplicationWindow::new()
        let window = ApplicationWindow::builder()
            .application(app)           // Required - associate with app
            .default_width(800)         // Set default size
            .default_height(600)
            .title("File Manager")      // Window title
            .build();

        window.present();
    });

    app.run()
}
```

### ❌ INCORRECT - Direct Constructor (will not work)

```rust
// This does NOT exist in gtk4-rs
let window = ApplicationWindow::new(app);  // ERROR: no such constructor
```

## Setting Window Child

In GTK4, use `set_child()` instead of `add()`:

```rust
let window = ApplicationWindow::builder()
    .application(app)
    .title("File Manager")
    .default_width(800)
    .default_height(600)
    .child(&some_widget)    // Set child using builder
    .build();

// OR use the method:
window.set_child(Some(&box_container));
```

## Window Management Methods

- `window.present()` - Show and focus the window
- `window.set_title(&str)` - Set window title
- `window.set_default_size(width, height)` - Set default size
- `window.set_child(Some(&widget))` - Set the main content widget
- `window.close()` - Close the window

## Application Association

The `ApplicationWindow` MUST be associated with a `GtkApplication`:

```rust
// In the activate callback:
app.connect_activate(|app| {
    let window = ApplicationWindow::builder()
        .application(app)  // This is REQUIRED
        .title("My App")
        .build();
    window.present();
});
```