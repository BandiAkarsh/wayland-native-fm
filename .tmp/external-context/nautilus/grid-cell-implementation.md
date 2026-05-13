---
source: Nautilus GitHub Repository
library: Nautilus (GNOME Files)
package: nautilus
topic: grid-cell-implementation
fetched: 2026-05-03T12:00:00Z
official_docs: https://github.com/GNOME/nautilus
---

# Nautilus Grid Cell Implementation

## Overview

The `NautilusGridCell` widget represents a single item (file/folder) in the grid view. It handles the layout of the icon, emblems, file name label, and optional caption labels.

## UI Template (nautilus-grid-cell.blp)

Nautilus uses the **Bluppi (.blp)** format for UI templates (GTK4's newer template format):

```bluppi
using Gtk 4.0;

template $NautilusGridCell: $NautilusViewCell {
  valign: "start";

  $NautilusImage icon {
    halign: center;
    valign: center;
  }

  Box emblems_box {
    orientation: vertical;
    halign: end;
    spacing: 6;
    margin-start: 2;

    styles [
      "dim-label",
    ]
  }

  Box labels_box {
    orientation: vertical;

    styles [
      "icon-ui-labels-box",
    ]

    Label label {
      has-tooltip: true;
      ellipsize: middle;
      justify: center;
      lines: 3;
      wrap: true;
      wrap-mode: word_char;
      attributes: "0 -1 insert-hyphens false";
      label: bind template.item as <$NautilusViewItem>.file as <$NautilusFile>.display-name;
    }
  }
}
```

## Widget Structure

The grid cell contains three main child widgets:

1. **icon** (`NautilusImage`) - The file/folder icon or thumbnail
2. **emblems_box** (`GtkBox`) - Container for emblem icons (starred, shared, etc.)
3. **labels_box** (`GtkBox`) - Container for the file name label and caption labels

## Custom Size Allocation

Nautilus implements custom `measure` and `size_allocate` functions for precise control over layout:

### Constants:
```c
#define EMBLEMS_BOX_WIDTH 18
#define VERTICAL_PADDING 6
```

### Measure Function:
```c
static void
nautilus_grid_cell_measure (GtkWidget      *widget,
                            GtkOrientation  orientation,
                            int             for_size,
                            int            *minimum,
                            int            *natural,
                            int            *min_baseline,
                            int            *nat_baseline)
{
    NautilusGridCell *self = NAUTILUS_GRID_CELL (widget);
    guint icon_size;
    int width, child_min, child_nat;

    g_object_get (self, "icon-size", &icon_size, NULL);
    width = EMBLEMS_BOX_WIDTH + icon_size + EMBLEMS_BOX_WIDTH;

    if (orientation == GTK_ORIENTATION_HORIZONTAL)
    {
        /* Width is fixed based on icon size + emblem margins */
        *natural = *minimum = width;
    }
    else /* GTK_ORIENTATION_VERTICAL */
    {
        /* Height = icon_size + padding + labels height */
        gtk_widget_measure (self->labels_box, GTK_ORIENTATION_VERTICAL, width,
                            &child_min, &child_nat,
                            min_baseline, nat_baseline);

        *minimum = icon_size + VERTICAL_PADDING + child_min;
        *natural = icon_size + VERTICAL_PADDING + child_nat;
    }
}
```

### Size Allocate Function:
```c
static void
nautilus_grid_cell_size_allocate (GtkWidget *widget,
                                  int        width,
                                  int        height,
                                  int        baseline)
{
    NautilusGridCell *self = NAUTILUS_GRID_CELL (widget);
    GtkAllocation child_allocation;
    guint icon_size;

    g_object_get (self, "icon-size", &icon_size, NULL);

    /* 1. Icon at the top, centered horizontally */
    child_allocation = (GtkAllocation) {
        EMBLEMS_BOX_WIDTH, 0,
        width - EMBLEMS_BOX_WIDTH * 2, icon_size
    };
    gtk_widget_size_allocate (self->icon, &child_allocation, -1);

    /* 2. Emblems box at the end (right side in LTR) */
    child_allocation.width = EMBLEMS_BOX_WIDTH;
    if (gtk_widget_get_direction (widget) == GTK_TEXT_DIR_LTR)
        child_allocation.x = width - EMBLEMS_BOX_WIDTH;
    else
        child_allocation.x = 0;
    gtk_widget_size_allocate (self->emblems_box, &child_allocation, -1);

    /* 3. Labels box gets remaining space below icon */
    child_allocation = (GtkAllocation) {
        0, icon_size + VERTICAL_PADDING,
        width, height - (icon_size + VERTICAL_PADDING)
    };
    gtk_widget_size_allocate (self->labels_box, &child_allocation, baseline);
}
```

## Icon Update Logic

The icon is updated based on file state:

```c
static void
update_icon (NautilusGridCell *self)
{
    g_autoptr (NautilusViewItem) item = nautilus_view_cell_get_item (NAUTILUS_VIEW_CELL (self));
    NautilusFile *file = nautilus_view_item_get_file (item);
    guint icon_size;
    gint scale_factor = gtk_widget_get_scale_factor (GTK_WIDGET (self));
    
    g_object_get (self, "icon-size", &icon_size, NULL);
    
    /* Get the icon paintable */
    g_autoptr (GdkPaintable) icon_paintable = 
        nautilus_file_get_icon_paintable (file, icon_size, scale_factor, flags);
    
    /* Handle thumbnails */
    gboolean show_thumbnail = nautilus_file_should_show_thumbnail (file);
    if (show_thumbnail)
    {
        g_autoptr (GFile) location = nautilus_file_get_location (file);
        nautilus_image_set_source (NAUTILUS_IMAGE (self->icon), location);
    }
    
    /* Handle cut files (dimmed appearance) */
    gboolean is_cut;
    g_object_get (item, "is-cut", &is_cut, NULL);
    if (is_cut)
    {
        gtk_widget_set_visible (self->icon, FALSE);
    }
    
    /* Handle hidden files */
    if (nautilus_file_is_hidden_file (file))
        gtk_widget_add_css_class (self->icon, "hidden-file");
}
```

## Caption Labels

Nautilus supports up to 3 caption lines below the main file name:

```c
static void
update_captions (NautilusGridCell *self)
{
    for (guint i = 0; i < NAUTILUS_GRID_CELL_N_CAPTIONS; i++)
    {
        GQuark attribute_q = self->caption_attributes[i];
        gboolean show_caption = (attribute_q != 0);

        if (show_caption)
        {
            g_autofree gchar *string = NULL;
            string = nautilus_file_get_string_attribute_q (file, attribute_q);
            gtk_label_set_text (GTK_LABEL (self->caption_labels[i]), string);
        }
    }
}
```

Caption attributes are typically file metadata like: size, type, modification date, etc.

## Emblems Display

Emblems (small overlay icons) are displayed in the `emblems_box`:

```c
static void
update_emblems (NautilusGridCell *self)
{
    /* Check for starred state */
    if (nautilus_tag_manager_file_is_starred (tag_manager, file_uri))
    {
        gtk_box_append (GTK_BOX (self->emblems_box),
                        gtk_image_new_from_icon_name ("starred-symbolic"));
    }

    /* Add file emblems (shared, symlink, etc.) */
    g_autolist (GIcon) emblems = nautilus_file_get_emblem_icons (file);
    for (GList *l = emblems; l != NULL; l = l->next)
    {
        gtk_box_append (GTK_BOX (self->emblems_box),
                        gtk_image_new_from_gicon (l->data));
    }
}
```

## Class Hierarchy

```
NautilusGridCell
└── NautilusViewCell (base class for view cells)
    └── GtkWidget
        └── GObject
```

## Key Properties

- **icon-size**: The size of the icon (bound from parent `NautilusGridView`)
- **item**: The `NautilusViewItem` associated with this cell
- **view**: The parent `NautilusListBase` view

## Signal Connections

The cell connects to various signals:
- `notify::icon-size` - Updates icon and captions when zoom level changes
- `notify::scale-factor` - Handles display scaling changes
- `file-changed` (on item) - Updates icon, emblems, and captions when file changes
- `notify::is-cut` (on item) - Updates icon appearance for cut files
