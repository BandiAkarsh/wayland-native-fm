# Task Context: Cross-Platform GUI Assembly File Manager with Glyphs

Session ID: 2026-05-01-asm-file-manager
Created: 2026-05-01T00:00:00Z
Updated: 2026-05-01T00:30:00Z
Status: in_progress

## Current Request
Build a platform-agnostic, very fast, easy to use, optimized in resources and memory, **GUI-based** (not TUI) file manager in assembly language.

Key requirements:
- **GUI (not TUI)**: Native graphical window with file/folder icons, not terminal-based
- **Glyphs on files/folders**: Like `colorls`, each file and folder shows its type-specific glyph/icon next to the name — rendered directly in the GUI
- **Platform-agnostic**: Linux, Windows, macOS (x86_64, ARM64)
- **Very fast**: Minimize syscalls, efficient rendering, fast directory traversal
- **Resource/memory optimized**: Target < 500KB binary, < 5MB RAM
- **Easy to use**: Intuitive GUI with mouse + keyboard navigation

## Context Files (Standards to Follow)
- ~/.config/opencode/context/core/standards/code-quality.md
- ~/.config/opencode/context/core/standards/security-patterns.md
- ~/.config/opencode/context/core/standards/test-coverage.md
- ~/.config/opencode/context/core/standards/documentation.md
- ~/.config/opencode/context/core/standards/code-analysis.md
- ~/.config/opencode/context/core/essential-patterns.md
- ~/.config/opencode/context/core/workflows/feature-breakdown.md
- ~/.config/opencode/context/core/workflows/code-review.md
- ~/.config/opencode/context/core/workflows/session-management.md
- ~/.config/opencode/context/core/context-system/standards/structure.md
- ~/.config/opencode/context/development/principles/clean-code.md
- ~/.config/opencode/context/core/task-management/navigation.md

## Reference Files (Source Material to Look At)
- (Project is starting from scratch - no existing source files)

## External Docs Fetched

### GUI Framework Research (2025-2026)

#### 1. Best GUI Library for Assembly: Nuklear
- **Nuklear**: Single-header C89 GUI library, public domain, ~18kLOC, zero dependencies
- Immediate mode GUI paradigm (render every frame)
- Backend-agnostic: works with any rendering backend
- Perfect for assembly: simple C ABI, no complex OOP patterns
- GitHub: https://github.com/Immediate-Mode-UI/Nuklear

#### 2. Cross-Platform Window/Input: sokol
- **sokol_app.h**: Unified window creation across Win32/macOS/Linux(X11)/WASM/Android
- **sokol_gfx.h**: Unified graphics API abstraction (OpenGL/D3D/Metal)
- **sokol_input.h**: Keyboard + mouse input handling
- Single-file C headers, public domain, assembly-friendly C ABI
- GitHub: https://github.com/floooh/sokol

#### 3. Font Rendering: stb_truetype
- **stb_truetype**: Single-header TrueType font renderer, public domain
- No dependencies, renders glyphs to bitmap buffers
- Can render Nerd Fonts for file type icons
- GitHub: https://github.com/nothings/stb

#### 4. File Icon System
- **Nerd Fonts v3.4.0** (2025-04): 3,600+ icons from Font Awesome, Devicons, Octicons, Seti-UI
- Map file extensions → Nerd Font codepoints
- Content-based detection via magic byte signatures (938+ formats at filesignatures.org)
- Icon databases: file-icons, devicons, vscode-icons, material-design-icons

#### 5. Direct Platform GUI APIs
- **Linux X11**: Wire protocol over Unix socket — Philippe Gaultier's tutorial produces **1 KiB GUI binary** in 600 lines NASM
- **Windows Win32**: Smallest GUI PE is **268 bytes** (NASM, no linker)
- **macOS Cocoa**: Via `objc_msgSend` from assembly — most complex approach

#### 6. Reference Project: CHasm
- **CHasm**: Complete desktop environment in pure x86_64 assembly
- Includes: shell, terminal, window manager, file viewer, TTF rasterizer
- Total: **under 500 KB** with zero dependencies
- Updated April 2026
- GitHub: https://github.com/isene/chasm

#### 7. Alternative GUI Libraries
- **Dear ImGui via cimgui** (v1.92.7, 2026): C wrapper for Dear ImGui, more features but heavier
- **LVGL**: Embedded GUI library, C-based, very lightweight
- **raylib**: Simple C game/multimedia library, easy assembly FFI
- **NanoGUI**: Minimal C++ GUI, OpenGL-based

#### 8. Graphics APIs
- **OpenGL**: Legacy but universal, easiest for assembly
- **Vulkan**: Modern but very complex for assembly
- **Software rendering**: No GPU dependency, simplest but slower

### File System Research (from previous ExternalScout call)

#### Directory Listing
- **Linux**: `getdents64` (batch listing, 64KB+ buffer), `inotify` (real-time)
- **macOS**: `getattrlistbulk` (4.7x faster than readdir+stat), `FSEvents`
- **Windows**: `FindFirstFile`/`FindNextFile`, `ReadDirectoryChangesW`

#### Memory Optimization
- **Arena allocator**: O(1) alloc (pointer bump), O(1) free-all
- **Pool allocator**: Fixed-size slots with bitmap
- Target: < 5MB resident memory

#### Performance
- Minimize syscalls (lf uses 422 for 3,930 files vs nnn's 4,143)
- Use `d_type` from getdents64 to avoid stat() per file
- Lazy stat for metadata (only when needed)
- SIMD for string operations (AVX2/NEON)

## Components

| Component | Description | Priority |
|-----------|-------------|----------|
| **Build System** | Makefile/Meson for assembly + C library linking | Critical |
| **Platform Abstraction** | Syscall wrappers for Linux (primary), macOS, Windows | Critical |
| **Memory Management** | Arena allocator, pool allocator for file metadata | Critical |
| **Window/Event Loop** | sokol_app.h integration or direct X11/Win32/Cocoa | Critical |
| **Graphics Rendering** | sokol_gfx.h or OpenGL for GUI rendering | Critical |
| **GUI Framework** | Nuklear integration (file list, buttons, panels) | Critical |
| **Font Rendering** | stb_truetype for Nerd Font glyph rendering | Critical |
| **File System Operations** | Directory listing, file I/O, metadata | Critical |
| **Glyph/Icon System** | Extension → Nerd Font codepoint mapping, icon cache | High |
| **File Type Mapper** | Extension-based + magic byte content detection | High |
| **Input Handler** | Mouse (click, drag, scroll) + keyboard (shortcuts) | High |
| **File List View** | Scrollable file/folder list with icons and names | High |
| **Navigation** | Directory navigation, breadcrumbs, path bar | High |
| **Unicode/Text** | UTF-8 handling, text layout, string operations | Medium |
| **Real-time Updates** | inotify/FSEvents/ReadDirectoryChangesW | Low |

## Constraints

- **Core Logic**: Pure Assembly (NASM for x86_64) — syscalls, memory, file I/O, strings, icon mapping
- **GUI Layer**: C (system-level, closest to assembly) — window, rendering, event loop
- **GUI Library**: Nuklear (single-header C89, public domain, ~18kLOC, zero dependencies)
- **Window/Input**: sokol (sokol_app.h, sokol_gfx.h) or direct X11/Win32/Cocoa in C
- **Font Rendering**: stb_truetype (single-header C, public domain)
- **Assembly ↔ C Interface**: Assembly exports functions with C ABI, C calls assembly for system operations
- **Target platforms**: Linux (primary), macOS, Windows
- **Target architectures**: x86_64 (primary), ARM64 (secondary)
- **Binary size**: < 500KB (including linked C libraries)
- **Memory usage**: < 5MB resident
- **Icons**: Nerd Fonts v3.4.0 rendered via stb_truetype
- **No heavy frameworks**: No GTK, Qt, Electron, C++, or browser engines
- **Platform independence**: NOT tied to any desktop environment — works on GNOME, KDE, Sway, i3, Hyprland, or bare X server

## Exit Criteria
- [ ] Build system working (assembly + Nuklear + sokol + stb_truetype)
- [ ] Window creation and event loop functional on Linux
- [ ] Nuklear GUI rendering (panels, buttons, lists)
- [ ] stb_truetype font rendering (Nerd Font glyphs)
- [ ] Directory listing functional (getdents64 on Linux)
- [ ] File list view with icons next to file/folder names
- [ ] File type detection by extension → correct glyph displayed
- [ ] Mouse navigation (click to select, double-click to open)
- [ ] Keyboard navigation (arrow keys, enter, escape, shortcuts)
- [ ] Directory navigation (double-click folder, back button, breadcrumbs)
- [ ] Binary size < 500KB
- [ ] Memory usage < 5MB
