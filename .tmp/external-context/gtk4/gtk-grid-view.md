---
source: GTK4 Documentation
library: GTK4
package: gtk4
topic: grid-view
fetched: 2026-05-03T12:00:00Z
official_docs: https://docs.gtk.org/gtk4/class.GridView.html
---

# GTK4 GridView Documentation

## Overview

`GtkGridView` is a GTK4 widget that presents a large dynamic grid of items. It's the recommended widget for implementing grid/icon views in GTK4 applications.

## Class Definition

```c
final class Gtk.GridView : Gtk.ListBase
  implements Gtk.Accessible, Gtk.Buildable, Gtk.ConstraintTarget, Gtk.Orientable, Gtk.Scrollable {
  /* No available fields */
}
```

`GtkGridView` inherits from `GtkListBase`, which is the abstract base class for GTK's list widgets (both `GtkListView` and `GtkGridView`).

## Key Features

- **Dynamic grid layout**: Items are arranged in a reflowing grid
- **Model-based**: Uses `GListModel` / `GtkSelectionModel` for data
- **Factory-based item creation**: Uses `GtkListItemFactory` to create widgets for each item
- **Rubberband selection**: Supports selecting multiple items by dragging
- **Scrollable**: Implements `GtkScrollable` interface

## Constructor

```c
GtkWidget *grid_view = gtk_grid_view_new (GtkSelectionModel *model,
                                          GtkListItemFactory *factory);
```

- **model**: (nullable) The model to use, or `NULL`
- **factory**: (nullable) The factory to use, or `NULL`

## Important Properties

### `model`
- **Type**: `GtkSelectionModel`
- The model that provides the items to display.

### `factory`
- **Type**: `GtkListItemFactory`
- The factory that creates widgets for items.

### `max-columns`
- **Type**: `int`
- Maximum number of columns per row.
- Default: `0` (unlimited)

### `min-columns`
- **Type**: `int`
- Minimum number of columns per row.
- Default: `0` (unlimited)

### `enable-rubberband`
- **Type**: `gboolean`
- Whether rubberband selection is enabled.
- Default: `FALSE`

### `single-click-activate`
- **Type**: `gboolean`
- Whether items activate on single click (affects selection behavior too).
- Default: `FALSE`

### `tab-behavior`
- **Type**: `GtkListTabBehavior`
- Behavior of the Tab key.
- Values: `GTK_LIST_TAB_ITEM`, `GTK_LIST_TAB_CELL`, `GTK_LIST_TAB_NONE`

## Important Methods

### Setting Model and Factory

```c
void gtk_grid_view_set_model (GtkGridView *self, GtkSelectionModel *model);
GtkSelectionModel *gtk_grid_view_get_model (GtkGridView *self);

void gtk_grid_view_set_factory (GtkGridView *self, GtkListItemFactory *factory);
GtkListItemFactory *gtk_grid_view_get_factory (GtkGridView *self);
```

### Column Control

```c
void gtk_grid_view_set_max_columns (GtkGridView *self, guint max_columns);
guint gtk_grid_view_get_max_columns (GtkGridView *self);

void gtk_grid_view_set_min_columns (GtkGridView *self, guint min_columns);
guint gtk_grid_view_get_min_columns (GtkGridView *self);
```

### Rubberband Selection

```c
void gtk_grid_view_set_enable_rubberband (GtkGridView *self, gboolean enable);
gboolean gtk_grid_view_get_enable_rubberband (GtkGridView *self);
```

### Scrolling

```c
void gtk_grid_view_scroll_to (GtkGridView     *self,
                              guint            position,
                              GtkListScrollFlags flags,
                              GtkScrollInfo   *scroll);
```

## Signals

### `activate`
```c
void (*activate) (GtkGridView *self, guint position);
```
Emitted when a cell has been activated by the user (e.g., double-click or Enter key).

## Using GtkSignalListItemFactory

The most common way to create item widgets:

```c
static void
setup_cb (GtkSignalListItemFactory *factory,
          GtkListItem              *listitem)
{
    /* Create widget for the list item */
    GtkWidget *widget = gtk_image_new ();
    gtk_list_item_set_child (listitem, widget);
}

static void
bind_cb (GtkSignalListItemFactory *factory,
         GtkListItem              *listitem)
{
    /* Bind data to the widget */
    MyItem *item = gtk_list_item_get_item (listitem);
    GtkWidget *widget = gtk_list_item_get_child (listitem);
    gtk_image_set_from_icon_name (GTK_IMAGE (widget), my_item_get_icon (item));
}

static void
unbind_cb (GtkSignalListItemFactory *factory,
           GtkListItem              *listitem)
{
    /* Clean up bindings if needed */
}

/* ... */

GtkListItemFactory *factory = gtk_signal_list_item_factory_new ();
g_signal_connect (factory, "setup", G_CALLBACK (setup_cb), NULL);
g_signal_connect (factory, "bind", G_CALLBACK (bind_cb), NULL);
g_signal_connect (factory, "unbind", G_CALLBACK (unbind_cb), NULL);

GtkWidget *grid_view = gtk_grid_view_new (model, factory);
```

## CSS Nodes

```
gridview
├── child[.activatable]
│   ├── child[.activatable]
│   ┊
╰── [rubberband]
```

- `GtkGridView` uses a single CSS node with name `gridview`
- Each child uses a single CSS node with name `child`
- If `GtkListItem:activatable` is set, the row gets `.activatable` style class
- For rubberband selection, a subnode with name `rubberband` is used

## Accessibility

`GtkGridView` uses the `GTK_ACCESSIBLE_ROLE_GRID` role, and the items use the `GTK_ACCESSIBLE_ROLE_GRID_CELL` role.

## Actions

`GtkGridView` defines built-in actions:

- `list.activate-item`: Activates the item at given position by emitting the `activate` signal.

## Comparison: GridView vs FlowBox

| Feature | GtkGridView | GtkFlowBox |
|---------|-------------|------------|
| GTK Version | GTK4 only | GTK3/GTK4 |
| Model-based | Yes (GListModel) | No (children added directly) |
| Performance | Better for large datasets | Good for small/medium |
| Factory support | Yes | No (direct children) |
| Sorting/Filtering | Via model | Built-in methods |
| Rubberband | Supported | Not supported |

**Recommendation**: Use `GtkGridView` for new GTK4 applications, especially for large datasets or when you need model-based data management.
