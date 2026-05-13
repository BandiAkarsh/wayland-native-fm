---
source: Web Search (GitHub, Official Docs, Linux Links)
libraries: Nemo, Nautilus, Dolphin, PCManFM-Qt, Thunar
topic: File Manager Feature Research
fetched: 2026-05-02
---

# Linux File Manager Feature Research

Comprehensive feature analysis from Nemo, Nautilus (GNOME Files), Dolphin, PCManFM-Qt, and Thunar for implementing a Wayland file manager with GTK4.

---

## 1. Core Features Every File Manager Should Have

### Window Management
- **Multiple windows** - Open folders in new windows
- **Tabbed browsing** - Multiple folders in single window (Dolphin, PCManFM-Qt, Thunar, Nemo)
- **Split view** - Dual-pane view for side-by-side file operations (Nemo, Dolphin, PCManFM-Qt)
- **Window state persistence** - Remember window size, position, and view mode per folder
- **Full-screen mode** - Immersive file browsing

### Location Bar
- **Breadcrumb navigation** - Clickable path segments (All five managers)
- **Editable location bar** - Type paths directly (Nemo, Dolphin, Nautilus)
- **URL bar** - Support for `file://`, `ssh://`, `ftp://`, `smb://` URIs

### Side Pane
- **Places/Bookmarks panel** - Quick access to important locations
- **Tree view** - Hierarchical folder navigation
- **Device list** - Mounted drives and removable media
- **Network places** - Browse network shares
- **Collapsible/expandable** - Toggle side pane visibility

### Status Bar
- **Item count** - Number of files/folders in current directory
- **Free space** - Available disk space on current partition
- **Selection info** - Size/count of selected items
- **Progress indicator** - Show during file operations

---

## 2. Navigation Features

### Basic Navigation
- **Back/Forward** - Navigate through visited locations (all)
- **Up/Parent** - Go to parent directory
- **Home** - Quick return to home folder
- **Refresh/Reload** - Rescan current directory

### History Management
- **Navigation history** - Back/forward through visited folders
- **History dropdown** - Right-click back/forward for full history
- **Recent files** - Track recently accessed files

### Bookmarks
- **Add/Remove bookmarks** - Save frequently used locations
- **Bookmark management** - Rename, reorder, organize bookmarks
- **Default bookmarks** - Home, Desktop, Trash, Network (standard XDG)
- **Keyboard shortcuts** - Quick bookmark access (Ctrl+D)

### Keyboard Navigation
- **Type-to-search** - Start typing to find files by name
- **Arrow keys** - Navigate within view
- **Enter** - Open selected item
- **Backspace** - Go to parent directory
- **Tab** - Move between panes/elements

---

## 3. File Operations

### Copy/Move
- **Drag and drop** - Visual file manipulation
- **Cut/Copy/Paste** - Standard clipboard operations (Ctrl+X/C/V)
- **Copy to... / Move to...** - Destination picker dialog
- **Progress dialog** - Show operation progress with cancel option
- **Conflict handling** - Prompt on name collisions
- **Parallel transfers** - Multiple files transferred simultaneously (Thunar advanced)
- **Background operations** - Continue operations while browsing

### Delete
- **Move to Trash** - Safe deletion (default)
- **Permanent delete** - Shift+Delete for immediate removal
- **Delete confirmation** - Prompt before permanent deletion

### Rename
- **Inline rename** - Edit filename directly in view
- **Batch rename** - Rename multiple files with patterns (All five)
- **Rename templates** - Use patterns like `file_#.txt` with numbering

### Create New
- **New folder** - Create directory
- **New file** - Create empty files from templates
- **New link** - Create symbolic/hard links

### File Actions
- **Open** - Launch with default application
- **Open with...** - Choose specific application
- **Open in terminal** - Launch terminal in directory (Nemo, Dolphin, Thunar)
- **Open as root** - Elevated privileges for system files (Nemo, PCManFM-Qt)
- **Send to** - Quick share/attach files

---

## 4. View Modes

### Icon View
- **Grid layout** - Icons arranged in grid
- **Adjustable icon size** - Zoom in/out
- **Icon captions** - Show name and additional info below icons

### List View
- **Detailed list** - Multiple columns (name, size, date, type, permissions)
- **Compact list** - Smaller row height
- **Expandable folders** - Tree-like list showing subfolder contents

### Thumbnail View
- **Image previews** - Show actual image thumbnails
- **Video thumbnails** - Preview video files
- **Document previews** - Preview PDF, text files

### Compact View
- **Icon + name + info** - Combined view style
- **Column arrangement** - Similar to classic file managers

### View Customization
- **Per-folder settings** - Remember view mode per directory
- **Global defaults** - Set default view for all folders
- **Zoom controls** - Adjust icon/thumbnail size
- **Sort indicators** - Show current sort order

---

## 5. Sorting and Filtering

### Sorting
- **Sort by name** - Alphabetical (A-Z, Z-A)
- **Sort by size** - File size ascending/descending
- **Sort by date** - Modified, accessed, or created date
- **Sort by type** - Group by file extension
- **Sort by permissions** - By access rights
- **Sort by rating** - User-assigned ratings (Dolphin)
- **Sort by tags** - User-defined tags (Dolphin)
- **Folders first** - Option to keep folders at top
- **Manual sort** - Drag to reorder

### Filtering
- **Show hidden files** - Toggle visibility of dotfiles (Ctrl+H)
- **Filter bar** - Quick filter input (PCManFM-Qt, Dolphin)
- **Search** - Recursive search within directory
- **Pattern matching** - Filter by filename patterns

### Grouping
- **Group by** - Group files by type, date, size, etc.
- **Show in groups** - Visual grouping in view

---

## 6. File Previews and Thumbnails

### Thumbnail Generation
- **Image thumbnails** - JPEG, PNG, GIF, WebP, etc.
- **Video thumbnails** - MP4, MKV, AVI with frame extraction
- **Audio thumbnails** - Show album art from metadata
- **Document thumbnails** - PDF, Office documents
- **Folder previews** - Show contents as thumbnail

### Preview Pane
- **Information panel** - Show file details (Dolphin)
- **Preview panel** - Larger preview of selected item
- **Quick preview** - Spacebar for preview overlay

### Thumbnail Settings
- **Enable/disable** - Toggle thumbnail display
- **Local files only** - Skip network thumbnails for performance
- **Thumbnail size** - Configure default size
- **Thumbnail cache** - Store for performance

---

## 7. Permissions and Metadata

### File Properties
- **Basic info** - Name, location, size, type
- **Timestamps** - Modified, accessed, changed dates
- **Permissions** - Read, write, execute for owner/group/others
- **Owner/Group** - Display UID/GID
- **MIME type** - File type detection

### Permission Editing
- **Visual permission editor** - Checkboxes for rwx
- **Octal mode** - Enter numeric permissions
- **Recursive apply** - Apply to folder contents

### Extended Attributes
- **Custom emblems** - Visual markers on files (PCManFM-Qt, Thunar)
- **File comments** - User notes
- **Tags** - User-defined labels (Dolphin)
- **Ratings** - Star ratings (Dolphin)

### Metadata Storage
- **gvfs-metadata** - GNOME metadata storage (Nautilus, Nemo, Thunar)
- **XDG metadata** - Standard freedesktop.org metadata

---

## 8. Unique Features

### Nemo (Cinnamon)
- **Desktop management** - Can manage desktop icons and wallpaper
- **Built-in terminal** - Integrated terminal access
- **Open as root** - Built-in elevated privilege option
- **Spices/Actions** - Extension system via Cinnamon Spices
- **SSH/FTP native** - Built-in remote file system support
- **Full path in title** - Always show full path in window title

### Nautilus (GNOME Files)
- **Search** - Powerful recursive search
- **Trash integration** - Full trash management
- **Remote mounts** - Seamless GVFS remote access
- **Recent files** - Track recently used files
- **File rolling** - Open archives as folders

### Dolphin (KDE)
- **Integrated terminal** - F4 to show/hide terminal (Konsole)
- **Places panel** - Enhanced bookmarks system
- **Information panel** - Detailed file preview
- **Version control integration** - Git plugin support
- **Batch rename** - Advanced rename with templates
- **Split view** - Dual-pane navigation
- **Undo/Redo** - Full operation history
- **Non-intrusive design** - Features don't clutter UI

### PCManFM-Qt (LXQt)
- **Lightweight** - Minimal resource usage
- **Desktop mode** - Wallpaper and icon management
- **Profiles** - Multiple configuration profiles
- **Admin mode** - Elevated operations without root instance
- **Custom actions** - User-defined context menu items
- **.hidden files** - Hide files without renaming

### Thunar (XFCE)
- **Bulk renamer** - Advanced multi-file rename
- **Custom actions** - User-defined commands
- **Volume manager** - Auto-mount removable media
- **Mouse gestures** - Middle-click gestures in icon view
- **Fast startup** - Optimized for speed
- **Clean interface** - Minimal, no-bloat design

---

## 9. Remote File Systems

### Supported Protocols
- **SSH/SFTP** - Secure remote access
- **FTP** - File Transfer Protocol
- **SMB/CIFS** - Windows network shares
- **NFS** - Network File System
- **WebDAV** - HTTP-based file access
- **MTP** - Media Transfer Protocol for mobile devices

### Remote Features
- **Seamless integration** - Browse remote as local
- **Connection management** - Save server credentials
- **Auto-mount** - Connect on access
- **Offline caching** - Work with cached content

---

## 10. Extensibility and Plugins

### Custom Actions
- **Context menu integration** - Add items to right-click menu
- **Conditions** - Show based on file type, location, selection
- **Parameters** - Pass file paths to commands
- **Submenus** - Group actions in submenus

### Plugin Systems
- **Nemo Actions** - Cinnamon Spices extensions
- **Dolphin plugins** - Git, Dropbox, Nextcloud integration
- **Thunar plugins** - Bulk renamer, archive, media tags, VCS
- **PCManFM actions** - Desktop entry-based custom actions

### Archive Support
- **Create archives** - Compress selected files
- **Extract archives** - Uncompress archives in-place
- **Browse archives** - View contents without extracting

---

## 11. Wayland-Specific Considerations

### For Our Implementation
- **Wayland-native** - Use GTK4 with GDK backend
- **XDG directories** - Follow XDG Base Directory spec
- **Portal support** - Use FileChooser portal for dialogs
- **DND protocols** - Implement Wayland DND properly
- **Surface management** - Handle multi-window properly

### Missing from Legacy Managers
- **Modern UI patterns** - Clean, minimal interface
- **Touch support** - Gesture-friendly interactions
- **Accessibility** - Full screen reader support
- **Color schemes** - Follow system theme

---

## Priority Implementation Recommendations

### Must Have (MVP)
1. Tabbed browsing with split view option
2. Breadcrumb + editable location bar
3. Places sidebar with tree view
4. Icon/List/Detailed view modes
5. Copy/Move/Delete/Rename operations
6. Drag and drop
7. Hidden file toggle
8. Sort by name/size/date/type
9. Thumbnails for images
10. Keyboard navigation

### Should Have
1. Bookmarks management
2. Batch rename
3. Remote file system support (SSH/SMB)
4. Search/Filter functionality
5. File properties with permissions
6. Progress dialogs for operations
7. Custom actions/extensions
8. Terminal integration

### Nice to Have
1. Split view (dual pane)
2. Integrated terminal
3. Version control integration
4. Advanced metadata/tags
5. Plugin system
6. Multiple profiles
7. Mouse gestures

---

## Sources
- Nemo: https://github.com/linuxmint/nemo
- Nautilus: https://gitlab.gnome.org/GNOME/nautilus
- Dolphin: https://docs.kde.org/stable5/en/dolphin/dolphin/
- PCManFM-Qt: https://github.com/lxqt/pcmanfm-qt/wiki
- Thunar: https://docs.xfce.org/xfce/thunar/start