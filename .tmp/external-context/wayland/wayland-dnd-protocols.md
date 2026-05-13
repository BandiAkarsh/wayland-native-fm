# Wayland Drag and Drop Protocols: Technical Specification

> **Source**: freedesktop.org Wayland Protocol Specification, python-wayland, pywayland, wayland-rs documentation  
> **Library**: Wayland protocols (wl_data_device_manager, wl_data_source, wl_data_offer, wl_data_device, wp_primary_selection)  
> **Package**: wayland-protocols  
> **Topic**: Drag and drop, clipboard, and data transfer mechanisms  
> **Fetched**: 2026-05-02  
> **Official Docs**: https://wayland.freedesktop.org/docs/html/apa.html  

---

## Table of Contents

1. [Overview of Wayland Data Transfer](#1-overview-of-wayland-data-transfer)
2. [Core Protocol Interfaces](#2-core-protocol-interfaces)
3. [Drag Source Implementation](#3-drag-source-implementation)
4. [Drop Target Implementation](#4-drop-target-implementation)
5. [MIME Type Handling](#5-mime-type-handling)
6. [Primary Selection Protocol](#6-primary-selection-protocol)
7. [Drag and Drop Actions](#7-drag-and-drop-actions)
8. [Data Transfer Methods](#8-data-transfer-methods)
9. [Protocol Versioning and Updates](#9-protocol-versioning-and-updates)
10. [Implementation Examples](#10-implementation-examples)

---

## 1. Overview of Wayland Data Transfer

The Wayland protocol provides clients with a mechanism for sharing data that enables copy-paste and drag-and-drop functionality. The architecture follows a producer-consumer model where:

- The **data source client** creates a `wl_data_source` object and advertises supported MIME types
- The **data offer client** receives a `wl_data_offer` object describing available data formats
- Data transfer occurs through file descriptors passed via the protocol
- The **data device** (`wl_data_device`) connects sources and offers, tied to each `wl_seat`

### 1.1 Protocol Flow

```
┌─────────────────────┐         ┌──────────────────────┐         ┌─────────────────────┐
│   Source Client      │         │    Compositor       │         │  Destination Client │
│                     │         │                      │         │                     │
│  wl_data_source ──────────────────> wl_data_offer ───────────────────> wl_data_device      │
│        │            │         │          │           │         │        │            │
│        │ offer()    │         │          │           │         │        │ accept()   │
│        │ set_actions         │          │           │         │        │ receive()  │
│        │            │         │          │           │         │        │            │
│  wl_data_device_manager       │          │           │         │        │ finish()   │
└─────────────────────┘         └──────────────────────┘         └─────────────────────┘
```

---

## 2. Core Protocol Interfaces

### 2.1 wl_data_device_manager

The `wl_data_device_manager` is a singleton global object that provides access to inter-client data transfer mechanisms.

#### Interface Definition

```xml
<interface name="wl_data_device_manager" version="3">
  <description summary="data transfer interface">
    The wl_data_device_manager is a singleton global object that provides access
    to inter-client data transfer mechanisms such as copy-and-paste and
    drag-and-drop.
  </description>

  <request name="create_data_source">
    <description summary="create a new data source">
      Create a new data source to offer for subsequent transfer requests.
    </description>
  </request>

  <request name="get_data_device">
    <description summary="get a data device for a seat">
      Get a data device for a given seat.
    </description>
    <arg name="seat" type="object" interface="wl_seat"/>
  </request>

  <enum name="dnd_action">
    <description summary="drag and drop actions">
      Bitfield of drag and drop actions.
    </description>
    <entry name="none" value="0" summary="no action"/>
    <entry name="copy" value="1" summary="copy action"/>
    <entry name="move" value="2" summary="move action"/>
    <entry name="ask" value="4" summary="ask user action"/>
  </enum>
</interface>
```

#### Rust API (smithay/wayland-rs)

```rust
pub trait WlDataDeviceManager {
    fn create_data_source(&self, qh: &QueueHandle, udata: U) -> WlDataSource;
    fn get_data_device(&self, seat: &WlSeat, qh: &QueueHandle, udata: U) -> WlDataDevice;
}
```

### 2.2 wl_data_source

The `wl_data_source` object is the source side of a data transfer, created by the source client.

#### Interface Definition

```xml
<interface name="wl_data_source" version="3">
  <description summary="offer to transfer data">
    The wl_data_source object is the source side of a wl_data_offer.
    It is created by the source client in a data transfer and provides a way
    to describe the offered data and a way to respond to requests to
    transfer the data.
  </description>

  <request name="offer">
    <description summary="add offered mime type">
      This request adds a mime type to the set of mime types advertised
      to targets. Can be called several times to offer multiple types.
    </description>
    <arg name="mime_type" type="string"/>
  </request>

  <request name="set_actions">
    <description summary="set the available drag-and-drop actions">
      Sets the actions that the source side client supports for this operation.
    </description>
    <arg name="dnd_actions" type="uint"/>
  </request>

  <event name="target">
    <description summary="notify the accepted mime type">
      Sent when a target accepts pointer_focus or motion events.
      If a target does not accept any of the offered types, type is NULL.
    </description>
    <arg name="mime_type" type="string"/>
  </event>

  <event name="send">
    <description summary="request the data">
      Request for data from the client. Send the data as the specified mime type
      over the passed file descriptor, then close it.
    </description>
    <arg name="mime_type" type="string"/>
    <arg name="fd" type="fd"/>
  </event>

  <event name="cancelled">
    <description summary="selection was cancelled">
      This data source is no longer valid. There are several reasons why this could happen:
      - The data source has been replaced by another data source.
      - The drag-and-drop operation was performed, but the drop destination
        did not accept any of the mime types offered.
      - The drag-and-drop operation was performed, but the drop destination
        did not select any of the actions present in the mask.
      - The drag-and-drop operation was performed but didn't happen over a surface.
      - The compositor cancelled the drag-and-drop operation.
    </description>
  </event>

  <event name="dnd_drop_performed" since="3">
    <description summary="the drag-and-drop operation physically finished">
      The user performed the drop action. Note that this does not indicate acceptance;
      wl_data_source.cancelled may still be emitted afterwards.
    </description>
  </event>

  <event name="dnd_finished" since="3">
    <description summary="the drag-and-drop operation concluded">
      The drop destination finished interoperating with this data source.
      The client is now free to destroy this data source.
      If the action used was "move", the source can delete the transferred data.
    </description>
  </event>

  <event name="action" since="3">
    <description summary="notify the selected action">
      This event indicates the action selected by the compositor after
      matching the source/destination side actions.
    </description>
    <arg name="dnd_action" type="uint"/>
  </event>
</interface>
```

#### Python API (python-wayland)

```python
class wl_data_source:
    def offer(self, mime_type: str) -> None:
        """Add a mime type to the set of mime types advertised to targets."""

    def set_actions(self, dnd_actions: int) -> None:
        """Set the available drag-and-drop actions."""

    def on_target(self, callback) -> None:
        """Called when a target accepts an offered mime type."""

    def on_send(self, callback) -> None:
        """Called when data is requested. Must send data to fd."""

    def on_cancelled(self, callback) -> None:
        """Called when the data source is no longer valid."""

    def on_dnd_drop_performed(self, callback) -> None:
        """Called when the drag-and-drop operation physically finished."""

    def on_dnd_finished(self, callback) -> None:
        """Called when the drag-and-drop operation concluded."""

    def on_action(self, callback) -> None:
        """Called when the compositor selects an action."""
```

### 2.3 wl_data_offer

The `wl_data_offer` represents data offered for transfer by another client.

#### Interface Definition

```xml
<interface name="wl_data_offer" version="3">
  <description summary="offer to transfer data">
    A wl_data_offer represents a piece of data offered for transfer by another
    client (the source client). It is used by copy-and-paste and drag-and-drop
    mechanisms. The offer describes the different types that the data can be
    converted to and provides the mechanism for transferring the data.
  </description>

  <request name="accept">
    <description summary="accept the offered data">
      The client issues this request to accept the data and indicate that it is
      ready to receive data of the given mime type. The transfer happens via
      the passed file descriptor.
    </description>
    <arg name="serial" type="uint"/>
    <arg name="mime_type" type="string"/>
  </request>

  <request name="receive">
    <description summary="request the data">
      To transfer the contents of the clipboard, the client issues this request
      and indicates the mime type it wants to receive.
    </description>
    <arg name="mime_type" type="string"/>
    <arg name="fd" type="fd"/>
  </request>

  <request name="set_actions">
    <description summary="set the available/preferred drag-and-drop actions">
      Sets the actions that the destination side supports for this operation.
    </description>
    <arg name="dnd_actions" type="uint"/>
  </request>

  <request name="finish">
    <description summary="the transfer is complete">
      Completes the drag-and-drop or clipboard data transfer.
    </description>
  </request>

  <event name="offer">
    <description summary="advertise offered mime type">
      Sent immediately upon creation to inform clients of the offered mime types.
    </description>
    <arg name="mime_type" type="string"/>
  </event>

  <event name="action" since="3">
    <description summary="notify the selected action">
      Sent to indicate the selected action after source/destination negotiation.
    </description>
    <arg name="dnd_action" type="uint"/>
  </event>

  <event name="finished" since="3">
    <description summary="the transfer is complete">
      Sent when the transfer is complete.
    </description>
  </event>
</interface>
```

### 2.4 wl_data_device

The `wl_data_device` provides access to data transfer mechanisms for a specific seat.

#### Interface Definition

```xml
<interface name="wl_data_device" version="3">
  <description summary="data transfer device">
    A wl_data_device provides access to inter-client data transfer mechanisms
    such as copy-and-paste and drag-and-drop. There is one wl_data_device per seat.
  </description>

  <request name="start_drag">
    <description summary="start drag-and-drop operation">
      This request asks the compositor to start a drag-and-drop operation
      on behalf of the client.
    </description>
    <arg name="source" type="object" interface="wl_data_source" allow-null="true"/>
    <arg name="origin" type="object" interface="wl_surface"/>
    <arg name="icon" type="object" interface="wl_surface" allow-null="true"/>
    <arg name="serial" type="uint"/>
  </request>

  <request name="set_selection">
    <description summary="copy to the selection">
      Set the clipboard selection.
    </description>
    <arg name="source" type="object" interface="wl_data_source" allow-null="true"/>
  </request>

  <event name="data_offer">
    <description summary="introduce new data offer">
      Introduces a new wl_data_offer object, subsequently used in either
      enter (for drag-and-drop) or selection (for clipboard).
    </description>
    <arg name="id" type="object" interface="wl_data_offer"/>
  </event>

  <event name="enter">
    <description summary="initiate drag-and-drop session">
      Enter event sent when cursor enters a surface during drag-and-drop.
    </description>
    <arg name="serial" type="uint"/>
    <arg name="surface" type="object" interface="wl_surface"/>
    <arg name="x" type="fixed"/>
    <arg name="y" type="fixed"/>
    <arg name="id" type="object" interface="wl_data_offer" allow-null="true"/>
  </event>

  <event name="motion">
    <description summary="cursor moves over surface">
      Motion event sent as cursor moves within a surface.
    </description>
    <arg name="serial" type="uint"/>
    <arg name="x" type="fixed"/>
    <arg name="y" type="fixed"/>
  </event>

  <event name="leave">
    <description summary="cursor leaves surface">
      Leave event sent when cursor leaves a surface.
    </description>
  </event>

  <event name="drop">
    <description summary="end drag-and-drop session successfully">
      The drag-and-drop destination should honor the last action received
      through wl_data_offer.action().
    </description>
  </event>

  <event name="selection">
    <description summary="advertise new selection">
      Notifies client of a new selection.
    </description>
    <arg name="id" type="object" interface="wl_data_offer" allow-null="true"/>
  </event>
</interface>
```

---

## 3. Drag Source Implementation

### 3.1 Creating a Drag Source

To implement a drag source, follow these steps:

```python
# Step 1: Get wl_data_device_manager from registry
def registry_global_added(self, registry, id_, interface, version):
    if interface == "wl_data_device_manager":
        self.data_device_manager = registry.bind(id_, wl_data_device_manager, version)

# Step 2: Create a data source
data_source = data_device_manager.create_data_source()

# Step 3: Offer MIME types
data_source.offer("text/plain")
data_source.offer("text/html")
data_source.offer("image/png")

# Step 4: Set supported drag actions (for drag-and-drop)
data_source.set_actions(
    WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE |
    WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY
)

# Step 5: Implement send handler
@data_source.on_send
def on_send(mime_type, fd):
    # Write data to the file descriptor in the requested format
    if mime_type == "text/plain":
        data = self.get_plain_text_data()
        os.write(fd, data.encode('utf-8'))
    elif mime_type == "text/html":
        data = self.get_html_data()
        os.write(fd, data.encode('utf-8'))
    # ... handle other MIME types
    os.close(fd)
```

### 3.2 Starting a Drag Operation

```python
@wl_pointer.on_button
def on_button(serial, time, button, state):
    if button == WL_POINTER_BUTTON_LEFT and state == WL_POINTER_BUTTON_STATE_PRESSED:
        # Must have pointer button press before starting drag
        data_device.start_drag(
            source=data_source,
            origin=origin_surface,  # Surface where drag originates
            icon=drag_icon_surface,  # Optional icon surface
            serial=serial
        )
```

### 3.3 Drag Source Event Handlers

```python
@data_source.on_target
def on_target(mime_type):
    """Called when a target accepts a MIME type."""
    if mime_type:
        print(f"Target accepted: {mime_type}")
    else:
        print("Target rejected all MIME types")

@data_source.on_dnd_drop_performed
def on_dnd_drop_performed():
    """Called when user drops the data (button released)."""
    print("Drop performed - data was physically dropped")
    # Note: This doesn't mean the target accepted the data

@data_source.on_action
def on_action(dnd_action):
    """Called when compositor selects an action."""
    action_names = {
        0: "none",
        1: "copy",
        2: "move",
        4: "ask"
    }
    print(f"Selected action: {action_names.get(dnd_action, 'unknown')}")

@data_source.on_dnd_finished
def on_dnd_finished():
    """Called when drop target is done with the data."""
    print("Drag-and-drop operation concluded")
    # If action was "move", delete the original data here

@data_source.on_cancelled
def on_cancelled():
    """Called when the data source is no longer valid."""
    print("Data source cancelled")
    data_source.destroy()
```

---

## 4. Drop Target Implementation

### 4.1 Accepting Drag-and-Drop

```python
class DropTarget:
    def __init__(self):
        self.current_offer = None
        self.current_serial = None
        self.accepted_mime_type = None

    def init_data_device(self, seat):
        """Initialize data device for accepting drops."""
        self.data_device = data_device_manager.get_data_device(seat)

    @data_device.on_data_offer
    def on_data_offer(id):
        """New data offer available."""
        self.current_offer = id

        # Listen for offered MIME types
        self.supported_types = []
        self.current_offer.add_listener({
            'offer': self.on_offer_offer,
        })

    def on_offer_offer(self, mime_type):
        """Receive MIME type advertisement."""
        self.supported_types.append(mime_type)

    @data_device.on_enter
    def on_enter(serial, surface, x, y, offer):
        """Cursor entered our surface."""
        self.current_serial = serial
        self.current_offer = offer
        # Store the serial for accept() call later
        # Note: Must accept in the offer event handler for version >= 2
        # For version 1: Accept on enter
        # For version >= 2: Accept can be delayed to drop handler

    @data_device.on_motion
    def on_motion(serial, x, y):
        """Cursor moved over surface."""
        # Update drag visual feedback based on position

    @data_device.on_leave
    def on_leave():
        """Cursor left our surface."""
        self.current_offer = None
        self.accepted_mime_type = None

    @data_device.on_drop
    def on_drop():
        """User dropped data on our surface."""
        if self.current_offer is None:
            return

        # Step 1: Accept the drop with a MIME type
        # Find a mutually supported MIME type
        for mime_type in self.supported_types:
            if self.can_accept(mime_type):
                self.accepted_mime_type = mime_type
                self.current_offer.accept(self.current_serial, mime_type)
                break
        else:
            # No matching MIME type - reject the drop
            self.current_offer.accept(self.current_serial, None)
            return

        # Step 2: Request the data transfer
        self.request_data_transfer()

    def on_offer_action(self, dnd_action):
        """Called when action is selected by compositor."""
        print(f"Compositor selected action: {dnd_action}")

    def request_data_transfer(self):
        """Request data from the source."""
        if self.accepted_mime_type is None:
            return

        # Create a pipe for data transfer
        read_fd, write_fd = os.pipe()

        # Request the data - this triggers wl_data_source.send on the source
        self.current_offer.receive(self.accepted_mime_type, write_fd)
        os.close(write_fd)

        # Read the data in a asynchronous manner
        # Or use a separate thread:
        def read_thread():
            data = os.read(read_fd, 4096)  # Read from pipe
            self.handle_received_data(data, self.accepted_mime_type)
            os.close(read_fd)
            # After reading, signal we're done
            self.current_offer.finish()

        threading.Thread(target=read_thread).start()

    def handle_received_data(self, data, mime_type):
        """Process received data."""
        if mime_type == "text/plain":
            # Handle plain text
            pass
        elif mime_type == "image/png":
            # Handle image
            pass
```

### 4.2 Providing Drag Feedback

```python
data_device.on_motion
def on_motion(serial, x, y):
    # Determine if this position accepts drops
    accept, supported_actions = self.calculate_drop_feedback(x, y)

    if self.current_offer:
        # Tell the compositor what actions we support
        self.current_offer.set_actions(supported_actions)
```

---

## 5. MIME Type Handling

### 5.1 Standard MIME Types

Common MIME types for Wayland data transfer:

| MIME Type | Description | Examples |
|-----------|-------------|----------|
| `text/plain` | Plain text | Text, source code |
| `text/plain;charset=utf-8` | UTF-8 encoded text | Unicode text |
| `text/html` | HTML formatted text | Web content |
| `text/uri-list` | URI list | File URIs |
| `image/png` | PNG image | Screenshots |
| `image/jpeg` | JPEG image | Photos |
| `application/x-moz-file` | GNOME file object | Files |
| `x-special/gnome-icon-list` | GNOME icon list | Icons |

### 5.2 MIME Type Negotiation Flow

```
Source Client                    Compositor                    Destination Client
     |                              |                              |
     |  offer("text/plain")         |                              |
     |  offer("text/html")         |                              |
     |  offer("image/png")         |                              |
     |---------------------------->|                              |
     |                              |  offer("text/plain")       |
     |                              |  offer("text/html")       |
     |                              |  offer("image/png")       |
     |                              |---------------------------->|
     |                              |                              |
     |                              |         [Cursor enters]   |
     |                              |<----------------------------|
     |                              |  accept(serial, "text/plain")
     |                              |---------------------------->|
     |  target("text/plain")     |<----------------------------|
     |<-----------------------------|                              |
```

### 5.3 Fallback MIME Types

Always provide `text/plain` as a fallback for text data:

```python
# Recommended: Offer multiple formats with fallback
def setup_data_source(source):
    # Primary format first
    source.offer("text/html")
    source.offer("text/plain;charset=utf-8")
    source.offer("text/plain")  # Fallback
```

---

## 6. Primary Selection Protocol

### 6.1 Overview

The `wp_primary_selection` protocol provides primary selection functionality (text selected with mouse, pasted with middle-click) equivalent to X11's PRIMARY selection.

- **Interface**: `zwp_primary_selection_device_manager_v1`
- **Version**: 1 (unstable)
- **Purpose**: Primary selection for middle-click paste

### 6.2 Protocol Interfaces

```xml
<interface name="zwp_primary_selection_device_manager_v1" version="1">
  <description summary="primary selection device manager">
    The primary selection device manager is a singleton global object that provides
    access to the primary selection.
  </description>

  <request name="create_source">
    <description summary="create a new primary selection source">
      Create a new primary selection source.
    </description>
  </request>

  <request name="get_device">
    <description summary="get a primary selection device">
      Get a primary selection device for a seat.
    </description>
    <arg name="seat" type="object" interface="wl_seat"/>
  </request>
</interface>

<interface name="zwp_primary_selection_source_v1" version="1">
  <description summary="offer to replace primary selection">
    The source side of a primary selection transfer.
  </description>

  <request name="offer">
    <arg name="mime_type" type="string"/>
  </request>

  <request name="send">
    <arg name="mime_type" type="string"/>
    <arg name="fd" type="fd"/>
  </request>

  <request name="cancelled">
    <description summary="selection was cancelled"/>
  </request>
</interface>

<interface name="zwp_primary_selection_device_v1" version="1">
  <description summary="primary selection device">
    Device for primary selection transfer.
  </description>

  <request name="set_selection">
    <arg name="source" type="object" interface="zwp_primary_selection_source_v1" allow-null="true"/>
  </request>

  <event name="data_offer">
    <arg name="offer" type="object" interface="zwp_primary_selection_offer_v1"/>
  </event>

  <event name="selection">
    <arg name="id" type="object" interface="zwp_primary_selection_offer_v1" allow-null="true"/>
  </event>
</interface>

<interface name="zwp_primary_selection_offer_v1" version="1">
  <description summary="offer to transfer primary selection">
    Offer to transfer primary selection contents.
  </description>

  <request name="receive">
    <arg name="mime_type" type="string"/>
    <arg name="fd" type="fd"/>
  </request>

  <event name="offer">
    <arg name="mime_type" type="string"/>
  </event>
</interface>
```

### 6.3 Implementation Example

```python
class PrimarySelection:
    def __init__(self, data_device_manager):
        self.data_device_manager = data_device_manager

    def set_primary_selection(self, text):
        """Set the primary selection (for text selection)."""
        source = self.data_device_manager.create_primary_selection_source()
        source.offer("text/plain")
        source.offer("text/plain;charset=utf-8")

        @source.on_send
        def on_send(mime_type, fd):
            os.write(fd, text.encode('utf-8'))
            os.close(fd)

        self.primary_device.set_selection(source)

    def handle_paste(self, event_queue):
        """Handle middle-click paste."""
        @self.primary_device.on_selection
        def on_selection(offer):
            if offer is None:
                return

            # Get available MIME types
            mime_types = []
            def track_offer(mime_type):
                mime_types.append(mime_type)
            offer.on_offer = track_offer

            # Wait for event processing
            event_queue.dispatch()

            # Request data
            read_fd, write_fd = os.pipe()
            offer.receive("text/plain", write_fd)
            os.close(write_fd)

            # Read the data
            data = os.read(read_fd, 4096)
            os.close(read_fd)
            return data
```

---

## 7. Drag and Drop Actions

### 7.1 Action Types

| Action | Value | Description |
|--------|-------|-------------|
| `none` | 0 | No action |
| `copy` | 1 | Copy to destination |
| `move` | 2 | Move to destination (delete original) |
| `ask` | 4 | Ask user (show dialog) |

### 7.2 Action Negotiation

```
Source: offers ACTION_MOVE | ACTION_COPY
      |
Dest: supports ACTION_COPY only
      |
Compositor: selects ACTION_COPY (intersection)
      |
Both: receive "copy" action
```

### 7.3 Action Preference by Modifier Keys

Recommended compositor behavior (modifier-based action switching):

- **No modifiers**: Prefer first matching action (typically "move" if supported)
- **Shift key pressed**: Prefer "move" action
- **Ctrl key pressed**: Prefer "copy" action
- **Alt key pressed**: Prefer "ask" action

---

## 8. Data Transfer Methods

### 8.1 File Descriptor Transfer

The core data transfer mechanism uses file descriptors:

```python
import os

def transfer_data(source_offer, mime_type):
    """Perform data transfer via file descriptor."""
    # Create pipe
    read_fd, write_fd = os.pipe()

    # Request data from source
    source_offer.receive(mime_type, write_fd)
    os.close(write_fd)  # Close write end in receiver

    # Read data from read end
    chunks = []
    while True:
        chunk = os.read(read_fd, 4096)
        if not chunk:
            break
        chunks.append(chunk)
    os.close(read_fd)

    return b''.join(chunks)
```

### 8.2 Asynchronous Transfer

```python
import asyncio
import threading

async def async_transfer_data(source_offer, mime_type):
    """Async data transfer."""
    loop = asyncio.get_event_loop()

    read_fd, write_fd = os.pipe()
    source_offer.receive(mime_type, write_fd)
    os.close(write_fd)

    def read_pipe():
        chunks = []
        while True:
            chunk = os.read(read_fd, 4096)
            if not chunk:
                break
            chunks.append(chunk)
        os.close(read_fd)
        return b''.join(chunks)

    data = await loop.run_in_executor(None, read_pipe)
    return data
```

### 8.3 Zero-Copy Transfer Considerations

For large data transfers, consider:

- **Memory mapping**: Use `mmap` for large files
- **Direct DMA**: Hardware-specific optimizations
- **Streaming**: Chunked transfer for very large data
- **Temporary files**: Write to temp file, pass URI

---

## 9. Protocol Versioning and Updates

### 9.1 Version History

| Version | Changes |
|---------|---------|
| 1 | Initial protocol definition |
| 2 | Added `set_actions` for destinations |
| 3 | Added `dnd_drop_performed`, `dnd_finished`, `action` events |

### 9.2 Version Requirements

- **wl_data_device_manager v1**: Basic clipboard support
- **wl_data_device_manager v2**: Action negotiation (destination must call `set_actions`)
- **wl_data_device_manager v3**: Source action events (`dnd_drop_performed`, `dnd_finished`, `action`)

### 9.3 Backward Compatibility

```python
def get_data_device_with_fallback(device_manager, seat):
    """Get data device handling version differences."""
    version = device_manager.version()

    if version >= 2:
        # Can use action negotiation
        device = device_manager.get_data_device(seat)
    else:
        # Version 1 - no action support
        device = device_manager.get_data_device(seat)

    return device
```

---

## 10. Implementation Examples

### 10.1 Complete Drag Source Example (Python)

```python
import os
from wayland.client import Display, Registry

class DragSource:
    def __init__(self, display):
        self.display = display
        self.data_device_manager = None
        self.data_device = None
        self.data_source = None

    def setup(self, seat):
        """Setup drag source for a seat."""
        # Get data device manager
        registry = self.display.get_registry()

        @registry.global
        def global_added(id_, interface, version):
            if interface == "wl_data_device_manager":
                self.data_device_manager = registry.bind(
                    id_, wl_data_device_manager, version
                )
                self.data_device = self.data_device_manager.get_data_device(seat)

        self.display.roundtrip()

    def start_drag(self, origin_surface, drag_icon, serial, text):
        """Start a drag operation."""
        # Create data source
        self.data_source = self.data_device_manager.create_data_source()

        # Offer MIME types
        self.data_source.offer("text/plain")
        self.data_source.offer("text/plain;charset=utf-8")

        # Set drag actions
        self.data_source.set_actions(
            WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE |
            WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY
        )

        # Setup handlers
        @self.data_source.on_send
        def on_send(mime_type, fd):
            os.write(fd, text.encode('utf-8'))
            os.close(fd)

        @self.data_source.on_action
        def on_action(dnd_action):
            if dnd_action == WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE:
                # Delete original data after successful drop
                self.delete_original_data()
            elif dnd_action == WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY:
                # Keep original (copy operation)
                pass

        @self.data_source.on_dnd_finished
        def on_dnd_finished():
            # Clean up
            self.data_source.destroy()

        # Start drag
        self.data_device.start_drag(
            source=self.data_source,
            origin=origin_surface,
            icon=drag_icon,
            serial=serial
        )

    def delete_original_data(self):
        """Delete the original data after a move operation."""
        # Implement deletion logic
        pass
```

### 10.2 Complete Drop Target Example (Python)

```python
class DropTarget:
    def __init__(self, display, seat, surface):
        self.display = display
        self.surface = surface
        self.current_offer = None
        self.supported_mime_types = []
        self.current_serial = None

    def setup(self):
        """Setup drop target."""
        registry = self.display.get_registry()

        @registry.global
        def global_added(id_, interface, version):
            if interface == "wl_data_device_manager":
                self.ddm = registry.bind(id_, wl_data_device_manager, version)
                self.data_device = self.ddm.get_data_device(seat)

        self.display.roundtrip()

        # Setup handlers
        @self.data_device.on_data_offer
        def on_data_offer(id_):
            self.current_offer = id_
            self.supported_mime_types = []

            @id_.on_offer
            def on_offer(mime_type):
                self.supported_mime_types.append(mime_type)

        @self.data_device.on_enter
        def on_enter(serial, surface, x, y, offer):
            self.current_serial = serial
            # For version >= 2, can defer accept to drop handler
            # This allows checking position before accepting

        @self.data_device.on_motion
        def on_motion(serial, x, y):
            # Update visual feedback based on position
            # Check if drop position is valid
            is_valid = self.is_drop_position_valid(x, y)

            if self.current_offer:
                # Set supported actions based on position
                if is_valid:
                    self.current_offer.set_actions(
                        WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY |
                        WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE
                    )
                else:
                    self.current_offer.set_actions(0)

        @self.data_device.on_leave
        def on_leave():
            self.current_offer = None
            self.current_serial = None

        @self.data_device.on_drop
        def on_drop():
            if not self.current_offer:
                return

            # Find accepted MIME type
            accepted_type = self.find_best_mime_type()
            if accepted_type:
                self.current_offer.accept(self.current_serial, accepted_type)
                self.perform_transfer(accepted_type)
            else:
                # Reject
                self.current_offer.accept(self.current_serial, None)

        @self.current_offer.on_action
        def on_action(action):
            print(f"Action selected: {action}")

    def is_drop_position_valid(self, x, y):
        """Check if drop position is valid."""
        # Implement position validation
        return True

    def find_best_mime_type(self):
        """Find best matching MIME type."""
        preferred = ["text/plain;charset=utf-8", "text/plain", "text/uri-list"]
        for mime in preferred:
            if mime in self.supported_mime_types:
                return mime
        return None

    def perform_transfer(self, mime_type):
        """Perform the actual data transfer."""
        read_fd, write_fd = os.pipe()
        self.current_offer.receive(mime_type, write_fd)
        os.close(write_fd)

        # Read in thread to avoid blocking
        def do_read():
            data = os.read(read_fd, 8192)
            os.close(read_fd)
            self.handle_received_data(data, mime_type)
            self.current_offer.finish()

        import threading
        threading.Thread(target=do_read).start()

    def handle_received_data(self, data, mime_type):
        """Handle received data."""
        print(f"Received {len(data)} bytes as {mime_type}")
        # Process data
```

---

## Summary

The Wayland drag and drop protocol provides a robust mechanism for inter-client data transfer through:

1. **wl_data_device_manager**: Factory for creating data sources and devices
2. **wl_data_source**: Source side of data transfer, advertises MIME types
3. **wl_data_offer**: Target side, receives MIME type offers
4. **wl_data_device**: Connects source and target, sends enter/leave/drop events
5. **wp_primary_selection**: Primary selection (X11-compatible middle-click paste)

Key implementation patterns:
- Create source → Offer MIME types → Start drag
- Accept offer → Request data → Read from pipe → Finish
- Use action negotiation for move/copy behavior
- Handle version differences for compatibility

---

## References

- [Wayland Protocol Specification](https://wayland.freedesktop.org/docs/html/apa.html)
- [Wayland Book: Protocol](https://wayland.freedesktop.org/docs/book/Protocol.html)
- [python-wayland Documentation](https://python-wayland.org/)
- [pywayland Documentation](https://pywayland.readthedocs.io/)
- [Primary Selection Protocol](https://wayland.app/protocols/primary-selection-unstable-v1)
- [Wayland-rs Documentation](https://smithay.github.io/wayland-rs/)