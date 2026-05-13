---
source: GTK4 Documentation
library: GTK4
package: gtk4
topic: flowbox
fetched: 2026-05-03T12:00:00Z
official_docs: https://docs.gtk.org/gtk4/class.FlowBox.html
---

# GTK4 FlowBox Documentation

## Overview

`GtkFlowBox` is a GTK4 widget that arranges child widgets in a reflowing grid. It's an older widget that predates the model-based `GtkGridView`.

## Class Definition

```c
final class Gtk.FlowBox : Gtk.Widget
  implements Gtk.Accessible, Gtk.Buildable, Gtk.ConstraintTarget, Gtk.Orientable {
  /* No available fields */
}
```

## Key Features

- **Reflowing layout**: Widgets flow horizontally or vertically based on orientation
- **Direct children**: Add widgets directly (not model-based)
- **Filtering**: Built-in filter function support
- **Sorting**: Built-in sort function support
- **Selection**: Multiple selection modes supported

## Constructor

```c
GtkWidget *flow_box = gtk_flow_box_new ();
```

## Important Properties

### `max-children-per-line`
- **Type**: `int`
- Maximum number of children per line.
- Default: `7`

### `min-children-per-line`
- **Type**: `int`
- Minimum number of children per line.
- Default: `0`

### `column-spacing`
- **Type**: `int`
- Horizontal space between children.
- Default: `0`

### `row-spacing`
- **Type**: `int`
- Vertical space between children.
- Default: `0`

### `homogeneous`
- **Type**: `gboolean`
- Whether all children get equal space.
- Default: `FALSE`

### `selection-mode`
- **Type**: `GtkSelectionMode`
- The selection mode.
- Values: `GTK_SELECTION_NONE`, `GTK_SELECTION_SINGLE`, `GTK_SELECTION_BROWSE`, `GTK_SELECTION_MULTIPLE`

### `activate-on-single-click`
- **Type**: `gboolean`
- Whether children activate on single click.
- Default: `TRUE`

## Important Methods

### Adding/Removing Children

```c
void gtk_flow_box_insert (GtkFlowBox *box, GtkWidget *widget, int position);
void gtk_flow_box_append (GtkFlowBox *box, GtkWidget *widget);  // since 4.6
void gtk_flow_box_prepend (GtkFlowBox *box, GtkWidget *widget); // since 4.6
void gtk_flow_box_remove (GtkFlowBox *box, GtkWidget *widget);
void gtk_flow_box_remove_all (GtkFlowBox *box);                // since 4.12
```

### Filtering

```c
void gtk_flow_box_set_filter_func (GtkFlowBox *box,
                                   GtkFlowBoxFilterFunc filter_func,
                                   gpointer user_data,
                                   GDestroyNotify destroy);
void gtk_flow_box_invalidate_filter (GtkFlowBox *box);
```

### Sorting

```c
void gtk_flow_box_set_sort_func (GtkFlowBox *box,
                                 GtkFlowBoxSortFunc sort_func,
                                 gpointer user_data,
                                 GDestroyNotify destroy);
void gtk_flow_box_invalidate_sort (GtkFlowBox *box);
```

### Selection

```c
void gtk_flow_box_set_selection_mode (GtkFlowBox *box, GtkSelectionMode mode);
void gtk_flow_box_select_child (GtkFlowBox *box, GtkFlowBoxChild *child);
void gtk_flow_box_unselect_child (GtkFlowBox *box, GtkFlowBoxChild *child);
void gtk_flow_box_select_all (GtkFlowBox *box);
void gtk_flow_box_unselect_all (GtkFlowBox *box);
```

### Getting Children

```c
GtkFlowBoxChild *gtk_flow_box_get_child_at_index (GtkFlowBox *box, int idx);
GtkFlowBoxChild *gtk_flow_box_get_child_at_pos (GtkFlowBox *box, int x, int y);
GList *gtk_flow_box_get_selected_children (GtkFlowBox *box);
```

## GtkFlowBoxChild

Each child in a `GtkFlowBox` is automatically wrapped in a `GtkFlowBoxChild`:

```c
GtkWidget *child_widget = gtk_label_new ("Item");
gtk_flow_box_append (flow_box, child_widget);
/* child_widget is automatically wrapped in a GtkFlowBoxChild */
```

## CSS Nodes

```
flowbox
├── flowboxchild
│   ╰── <child>
├── flowboxchild
│   ╰── <child>
┊
╰── [rubberband]
```

## Signals

### `child-activated`
```c
void (*child_activated) (GtkFlowBox *self, GtkFlowBoxChild *child);
```
Emitted when a child has been activated by the user.

### `activate-cursor-child`
Emitted when the user activates the box.

### `selected-children-changed`
Emitted when the set of selected children changes.

## Accessibility

`GtkFlowBox` uses the `GTK_ACCESSIBLE_ROLE_GRID` role, and `GtkFlowBoxChild` uses the `GTK_ACCESSIBLE_ROLE_GRID_CELL` role.

## When to Use FlowBox vs GridView

### Use GtkFlowBox when:
- You have a small to medium number of items
- You want to add widgets directly without a model
- You need built-in filtering/sorting
- You're working with a simple static layout

### Use GtkGridView when:
- You have a large dynamic dataset
- You want model-based data management
- You need better performance with many items
- You want to use GTK4's newer list widget framework

## Nautilus Note

**Nautilus uses `GtkGridView`, not `GtkFlowBox`**, for its grid/icon view implementation. This is because:
1. Better performance with many files
2. Model-based approach fits their `NautilusViewModel`
3. More flexible item widget creation via factories
4. Better integration with GTK4's list widget framework
