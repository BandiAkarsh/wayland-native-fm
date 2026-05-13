---
source: Nautilus GitHub Repository
library: Nautilus (GNOME Files)
package: nautilus
topic: grid-view-implementation
fetched: 2026-05-03T12:00:00Z
official_docs: https://github.com/GNOME/nautilus
---

# Nautilus Grid View Implementation

## Overview

Nautilus (GNOME Files) uses **GtkGridView** as the primary widget for its grid/icon view. The implementation is spread across several files:

- `nautilus-grid-view.c` / `nautilus-grid-view.h` - Main grid view class
- `nautilus-grid-cell.c` / `nautilus-grid-cell.h` - Individual grid cell (icon + label)
- `nautilus-grid-cell.blp` - UI template for grid cells (Bluppi format)

## Key Widget: GtkGridView

Nautilus uses `GtkGridView` (GTK4's grid widget) instead of the older `GtkFlowBox` or custom implementations.

### Grid View Creation (from `nautilus-grid-view.c`):

```c
static GtkGridView *
create_view_ui (NautilusGridView *self)
{
    GtkListItemFactory *factory;
    GtkWidget *widget;

    factory = gtk_signal_list_item_factory_new ();
    g_signal_connect (factory, "setup", G_CALLBACK (setup_cell), self);
    g_signal_connect (factory, "bind", G_CALLBACK (bind_cell), self);
    g_signal_connect (factory, "unbind", G_CALLBACK (unbind_cell), self);

    widget = gtk_grid_view_new (NULL, factory);

    /* We don't use the built-in child activation feature for clicks */
    gtk_grid_view_set_single_click_activate (GTK_GRID_VIEW (widget), FALSE);
    gtk_grid_view_set_max_columns (GTK_GRID_VIEW (widget), 20);
    gtk_grid_view_set_tab_behavior (GTK_GRID_VIEW (widget), GTK_LIST_TAB_ITEM);

    /* ... accessibility setup ... */

    g_signal_connect (widget, "activate", G_CALLBACK (on_grid_view_item_activated), self);

    return GTK_GRID_VIEW (widget);
}
```

### Key Properties Set:
- **single-click-activate**: Set to `FALSE` - Nautilus handles click activation itself
- **max-columns**: Set to `20` - maximum number of columns allowed
- **tab-behavior**: Set to `GTK_LIST_TAB_ITEM` - tab behavior for item navigation

## Cell Factory Pattern

Nautilus uses `GtkSignalListItemFactory` to create and bind grid cells:

### Setup Phase (`setup_cell` callback):

```c
static void
setup_cell (GtkSignalListItemFactory *factory,
            GtkListItem              *listitem,
            gpointer                  user_data)
{
    NautilusGridView *self = NAUTILUS_GRID_VIEW (user_data);
    NautilusGridCell *cell;
    GtkExpression *expression;

    cell = nautilus_grid_cell_new (NAUTILUS_LIST_BASE (self));
    gtk_list_item_set_child (listitem, GTK_WIDGET (cell));
    
    /* Bind icon-size property from grid view to cell */
    g_object_bind_property (self, "icon-size",
                            cell, "icon-size",
                            G_BINDING_SYNC_CREATE);

    nautilus_grid_cell_set_caption_attributes (cell, self->caption_attributes);

    /* Set accessible label using file display name */
    expression = gtk_property_expression_new (GTK_TYPE_LIST_ITEM, NULL, "item");
    expression = gtk_property_expression_new (GTK_TYPE_TREE_LIST_ROW, expression, "item");
    expression = gtk_property_expression_new (NAUTILUS_TYPE_VIEW_ITEM, expression, "file");
    expression = gtk_property_expression_new (NAUTILUS_TYPE_FILE, expression, "a11y-name");
    gtk_expression_bind (expression, listitem, "accessible-label", listitem);
}
```

### Bind Phase (`bind_cell` callback):

```c
static void
bind_cell (GtkSignalListItemFactory *factory,
           GtkListItem              *listitem,
           gpointer                  user_data)
{
    g_autoptr (NautilusViewItem) item = get_view_item (listitem);
    GtkWidget *cell = gtk_list_item_get_child (listitem);
    GtkWidget *parent = gtk_widget_get_parent (cell);

    nautilus_view_item_set_item_ui (item, cell);

    /* Center the cell in its parent */
    gtk_widget_set_halign (parent, GTK_ALIGN_CENTER);
    gtk_widget_set_valign (parent, GTK_ALIGN_START);
}
```

## Zoom Levels and Icon Sizes

Nautilus supports multiple zoom levels with corresponding icon sizes:

```c
typedef enum {
    NAUTILUS_GRID_ZOOM_LEVEL_SMALL,
    NAUTILUS_GRID_ZOOM_LEVEL_SMALL_PLUS,
    NAUTILUS_GRID_ZOOM_LEVEL_MEDIUM,
    NAUTILUS_GRID_ZOOM_LEVEL_LARGE,
    NAUTILUS_GRID_ZOOM_LEVEL_EXTRA_LARGE,
} NautilusGridZoomLevel;

static guint
get_icon_size_for_zoom_level (NautilusGridZoomLevel zoom_level)
{
    switch (zoom_level)
    {
        case NAUTILUS_GRID_ZOOM_LEVEL_SMALL:      return NAUTILUS_GRID_ICON_SIZE_SMALL;
        case NAUTILUS_GRID_ZOOM_LEVEL_SMALL_PLUS: return NAUTILUS_GRID_ICON_SIZE_SMALL_PLUS;
        case NAUTILUS_GRID_ZOOM_LEVEL_MEDIUM:     return NAUTILUS_GRID_ICON_SIZE_MEDIUM;
        case NAUTILUS_GRID_ZOOM_LEVEL_LARGE:      return NAUTILUS_GRID_ICON_SIZE_LARGE;
        case NAUTILUS_GRID_ZOOM_LEVEL_EXTRA_LARGE:return NAUTILUS_GRID_ICON_SIZE_EXTRA_LARGE;
    }
}
```

## Model Binding

The grid view binds to a `NautilusViewModel` (which implements `GtkSelectionModel`):

```c
static void
on_model_changed (NautilusGridView *self)
{
    NautilusViewModel *model = nautilus_list_base_get_model (NAUTILUS_LIST_BASE (self));

    if (model != NULL)
    {
        gtk_grid_view_set_enable_rubberband (GTK_GRID_VIEW (self->view_ui),
                                             !nautilus_view_model_get_single_selection (model));
    }

    gtk_grid_view_set_model (self->view_ui, GTK_SELECTION_MODEL (model));
}
```

## Captions/Metadata Display

Nautilus supports displaying additional file metadata (captions) below the file name:

```c
static void
set_captions_from_preferences (NautilusGridView *self)
{
    g_auto (GStrv) value = NULL;
    gint n_captions_for_zoom_level;

    value = g_settings_get_strv (nautilus_icon_view_preferences,
                                 NAUTILUS_PREFERENCES_ICON_VIEW_CAPTIONS);

    /* Set a ceiling on the number of captions depending on the zoom level */
    n_captions_for_zoom_level = MIN ((uint) self->zoom_level + 1,
                                     G_N_ELEMENTS (self->caption_attributes));

    /* Reset array to zeros beforehand */
    memset (&self->caption_attributes, 0, sizeof (self->caption_attributes));
    for (gint i = 0, quark_i = 0;
         value[i] != NULL && quark_i < n_captions_for_zoom_level;
         i++)
    {
        if (g_strcmp0 (value[i], "none") == 0)
            continue;

        /* Convert to quarks in advance for performance */
        self->caption_attributes[quark_i] = g_quark_from_string (value[i]);
        quark_i++;
    }
}
```

## Class Hierarchy

```
NautilusGridView
└── NautilusListBase (base class for all Nautilus views)
    └── GtkWidget
        └── GObject
```

The `NautilusGridView` extends `NautilusListBase`, which is a custom base class that provides common functionality for all Nautilus view types (grid, list, etc.).
