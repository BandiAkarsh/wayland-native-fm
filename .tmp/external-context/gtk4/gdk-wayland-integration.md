# GTK4 GDK Wayland Documentation (v4.23.1 - May 2026)

## Overview

GTK4 GDK Wayland integration provides native Wayland support through the `gtk4-wayland` pkg-config module. This documentation covers the key APIs for building native Wayland applications with GTK4.

## Wayland Backend Detection

### Compile-Time Check

```c
#ifdef GDK_WINDOWING_WAYLAND
#include <gdk/wayland/gdkwayland.h>
#endif
```

### Runtime Check

```c
#ifdef GDK_WINDOWING_WAYLAND
  if (GDK_IS_WAYLAND_DISPLAY (display))
    {
      // Wayland-specific calls
    }
#endif
```

## Core Wayland Functions

### GdkWaylandDisplay Functions

| Function | Description | Since |
|----------|-------------|-------|
| `gdk_wayland_display_get_wl_display()` | Returns native `wl_display*` | 4.0 |
| `gdk_wayland_display_get_wl_compositor()` | Returns `wl_compositor*` | 4.0 |
| `gdk_wayland_display_query_registry()` | Queries Wayland globals | 4.0 |
| `gdk_wayland_display_get_egl_display()` | Get EGL display (4.4+) | 4.4 |
| `gdk_wayland_display_set_cursor_theme()` | Set cursor theme | 4.0 |
| `gdk_wayland_display_set_startup_notification_id()` | Set startup ID | 4.0 |

### GdkWaylandSurface Functions

| Function | Description | Since |
|----------|-------------|-------|
| `gdk_wayland_surface_get_wl_surface()` | Get native `wl_surface*` | 4.0 |
| `gdk_wayland_surface_force_next_commit()` | Force next commit (4.18+) | 4.18 |

### GdkWaylandToplevel Functions

| Function | Description | Since |
|----------|-------------|-------|
| `gdk_wayland_toplevel_export_handle()` | Export for cross-process | 4.0 |
| `gdk_wayland_toplevel_set_transient_for_exported()` | Set parent via handle | 4.0 |
| `gdk_wayland_toplevel_set_application_id()` | Set app ID | 4.0 |

## Window Creation

### Create Toplevel Window

```c
// Using GdkSurface API
GdkSurface *surface = gdk_surface_new_toplevel (display);

// Configure via GdkToplevel interface
gdk_toplevel_set_title (GDK_TOPLEVEL (surface), "My App");
gdk_toplevel_set_application_id (GDK_TOPLEVEL (surface), "com.example.app");
```

### Create Popup Window

```c
GdkSurface *popup = gdk_surface_new_popup (parent, &anchor_rect);
```

## Rendering Functions

### OpenGL Context

```c
// Check if OpenGL available (4.4+)
gboolean GL_AVAILABLE;
gdk_display_prepare_gl (display, &GL_AVAILABLE);

// Create GL context
GdkGLContext *ctx = gdk_display_create_gl_context (display);
gdk_gl_context_realize (ctx, &error);
```

### DMA-buf Support (4.14+)

```c
GdkDMABufFormats *formats = gdk_display_get_dmabuf_formats (display);
```

### Cairo Rendering (deprecated 4.18)

```c
// DEPRECATED in 4.18 - Use GdkCairoContext via GtkWidget render APIs instead
cairo_t *cr = gdk_cairo_create (surface); // deprecated
```

## Event Handling

### Surface Signals

| Signal | Description |
|--------|-------------|
| `GdkSurface::event` | Input events received |
| `GdkSurface::render` | Redraw needed |
| `GdkSurface::layout` | Size changed |
| `GdkSurface::enter-monitor` | Surface entered monitor |
| `GdkSurface::leave-monitor` | Left monitor |

### Display Signals

| Signal | Description |
|--------|-------------|
| `GdkDisplay::closed` | Display closed |
| `GdkDisplay::opened` | Display opened |
| `GdkDisplay::seat-added` | Seat added |
| `GdkDisplay::seat-removed` | Seat removed |

### Event Loop Pattern

```c
// Connect to surface event signal
g_signal_connect (surface, "render", G_CALLBACK (on_render), NULL);
g_signal_connect (surface, "layout", G_CALLBACK (on_layout), NULL);

// Or use gtk_main() for standard GTK applications
```

## Initialization Sequence

```c
// 1. Open display (GDK automatically selects backend)
GdkDisplay *display = gdk_display_open (NULL); // NULL = default

// 2. Create toplevel surface
GdkSurface *surface = gdk_surface_new_toplevel (display);

// 3. Configure (via GdkToplevel API)
gdk_toplevel_set_title (GDK_TOPLEVEL (surface), "App Title");
gdk_toplevel_set_size (GDK_TOPLEVEL (surface), 800, 600);

// 4. Present window
gdk_toplevel_present (GDK_TOPLEVEL (surface), 0);

// 5. For OpenGL rendering (4.4+):
gdk_display_prepare_gl (display, &.gl_available);
GdkGLContext *gl_ctx = gdk_display_create_gl_context (display);
```

## Recent API Changes (2025-2026)

### GTK 4.18 (2025)
- `gdk_wayland_surface_force_next_commit()` added
- `gdk_surface_create_cairo_context()` deprecated
- Use GTK widget rendering APIs instead

### GTK 4.14 (Mid-2025)
- `gdk_display_get_dmabuf_formats()` added
- DMA-buf rendering support
- `gdk_display_supports_shadow_width()` added

### GTK 4.12 (Early 2025)
- Enhanced HiDPI support
- `gdk_surface_get_scale()` added

### GTK 4.10 (Late 2024)
- `gdk_display_get_default_seat()` improved
- Startup notification refactored

## Rendering Best Practices

1. **Use GTK4 widget rendering** - Don't directly use Cairo with surfaces; use GtkWidget render functions
2. **Prefer GL/VK** - For high-performance, use OpenGL/Vulkan via `gdk_display_create_gl_context()`
3. **DMA-buf for zero-copy** - Use DMA-buf when sharing with other processes (4.14+)
4. **Frame clock** - Always use `gdk_surface_get_frame_clock()` for frame timing
5. **Scale factor** - Query `gdk_surface_get_scale_factor()` for HiDPI
6. **Queue renders** - Use `gdk_surface_queue_render()` rather than forcing renders

## Build Requirements

```bash
# Compile with:
pkg-config --cflags --libs gtk4-wayland

# Alternative: use gtk4 which includes all backends
pkg-config --cflags --libs gtk4
```

## Source

Fetched from: https://docs.gtk.org/gdk4/ (v4.23.1)
Wayland-specific: https://docs.gtk.org/gdk4-wayland/
fetched: 2026-05-02