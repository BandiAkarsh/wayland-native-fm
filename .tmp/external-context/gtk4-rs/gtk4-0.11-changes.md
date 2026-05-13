---
source: Context7 API
library: gtk4-rs
package: gtk4-rs
topic: GTK4 0.11.x changes and best practices
fetched: 2026-05-02T10:00:00Z
official_docs: https://gtk-rs.github.io/gtk4-rs/stable/latest/docs/gtk4/index.html
---

# GTK4 0.11.x Changes and Best Practices (as of May 2026)

## Key Changes in GTK4 (from GTK 4.10+)

### Deprecated APIs to Avoid

The following are deprecated since GTK 4.10 and should NOT be used in new code:

1. **GtkListStore** - Use `gio::ListStore` instead
2. **GtkTreeModel** - Use `gio::ListModel` instead
3. **GtkTreeView** - Use `ListView` or `ColumnView` instead
4. **GtkCellRenderer** - Use `ListView` with factories instead
5. **GtkAssistant** - Use custom window with pages
6. **GtkInfoBar** - Use custom widgets

### Migration to ListModel Pattern

**OLD (deprecated):**
```rust
// DON'T use this - deprecated
let store = GtkListStore::new(...);
let tree_view = GtkTreeView::new_with_model(&store);
```

**NEW (recommended):**
```rust
use gtk::gio::ListStore;
use gtk::{ListView, SingleSelection, SignalListItemFactory};

// Use gio::ListStore with GObject items
let model = ListStore::new(FileItem::static_type());

let factory = SignalListItemFactory::new();
factory.connect_setup(|_, list_item| {
    // Create widget
});

factory.connect_bind(|_, list_item| {
    // Bind data
});

let selection = SingleSelection::new(Some(model));
let list_view = ListView::new(Some(selection), Some(factory));
```

## Best Practices for GTK4 0.11.x

### 1. Always Use Builder Pattern

```rust
// ✅ CORRECT
let window = ApplicationWindow::builder()
    .application(app)
    .title("My App")
    .default_width(800)
    .default_height(600)
    .build();

// ❌ AVOID - direct construction doesn't work in gtk4-rs
let window = ApplicationWindow::new(app);
```

### 2. Use set_child() Instead of add()

```rust
// ✅ CORRECT (GTK4)
window.set_child(Some(&box));

// ❌ OLD (GTK3)
window.add(&box);
```

### 3. Use gio::ListStore for Models

```rust
use gtk::gio::ListStore;

// Create a list store for FileItem objects
let store = ListStore::new(FileItem::static_type());

// Add items
store.append(&file_item);

// Remove items
store.remove(position);

// The ListView automatically updates when model changes
```

### 4. Use SignalListItemFactory for ListView

```rust
use gtk::SignalListItemFactory;

let factory = SignalListItemFactory::new();

// Setup: create widget for list item
factory.connect_setup(|_, list_item| {
    let label = Label::new(None);
    list_item.set_child(Some(&label));
});

// Bind: connect item data to widget
factory.connect_bind(|_, list_item| {
    let item = list_item.item().unwrap();
    let label = list_item.child().unwrap().downcast::<Label>().unwrap();
    label.set_text(&item.name());
});

// Unbind: clean up when item is deselected
factory.connect_unbind(|_, list_item| {
    // Clean up if needed
});
```

### 5. Proper Signal Connection with move

```rust
// Always use move for closures that capture data
let path = current_path.clone();
button.connect_clicked(move |_| {
    navigate_to(&path);
});

// For simple callbacks without captured data:
button.connect_clicked(|_| {
    println!("Clicked!");
});
```

### 6. Window Presentation

```rust
// ✅ CORRECT - use present() to show window
window.present();

// ❌ OLD - don't use show() in GTK4
window.show();
```

### 7. ScrolledWindow for ListView/GridView

**ListView MUST be wrapped in ScrolledWindow:**

```rust
let scrolled = ScrolledWindow::builder()
    .hscrollbar_policy(PolicyType::Automatic)
    .vscrollbar_policy(PolicyType::Automatic)
    .child(&list_view)
    .build();
```

### 8. Selection Models

```rust
use gtk::{SingleSelection, MultiSelection, NoSelection};

// Single item selection (most common)
let selection = SingleSelection::new(Some(model));

// Multiple items selection
let selection = MultiSelection::new(Some(model));

// No selection (display only)
let selection = NoSelection::new(Some(model));
```

## Common Compilation Errors to Avoid

1. **"no such constructor"** - Use `ApplicationWindow::builder()` not `new()`
2. **"method not found"** - Use `set_child()` not `add()`
3. **"trait not satisfied"** - Import `gtk::prelude::*`
4. **"lifetime mismatch"** - Use `move` keyword in closures

## Version Check

To check your gtk4-rs version, look at your Cargo.toml:

```toml
[dependencies]
gtk4 = "0.11"  # or later
```

For the latest, check: https://crates.io/crates/gtk4