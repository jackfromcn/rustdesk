# RustDesk SDK Integrator Guide

**Version:** 1.4.6
**Target audience:** Integrators building custom remote desktop clients

## Overview

RustDesk now exposes a **minimal headless remote-core library** for integrators who want to build their own remote desktop tools without inheriting RustDesk UI logic.

The minimal v1 surface is intentionally focused on core remote functionality:
- connect / disconnect / session state
- frame pull
- mouse / keyboard input
- text clipboard send
- event callback notifications

The library does **not** impose any UI, windowing, toolbar, or product-level presentation logic.

## Quick Start

### Step 1: Build RustDesk Library

```bash
# Install dependencies (see CLAUDE.md)
vcpkg install libvpx libyuv opus aom

# Build library
cargo build -p rustdesk --release --lib

# Optional: build service binary
cargo build -p rustdesk --release --bin service

# Output: target/release/liblibrustdesk.dylib (macOS)
#         target/release/liblibrustdesk.so (Linux)
#         target/release/liblibrustdesk.a (static)
```

### Step 2: Include C Header

```c
#include "rustdesk.h"
```

### Step 3: Basic Session

```c
RustdeskSession session = rustdesk_connect("peer_id", "password", NULL);
if (!session) return 1;

while (rustdesk_session_get_state(session, NULL, 0) == 0) {
    // wait for ready/connected
}

while (rustdesk_session_get_state(session, NULL, 0) == 1) {
    VideoFrameData* frame = rustdesk_get_frame(session);
    if (frame) {
        // display raw RGBA frame
        rustdesk_free_frame(frame);
        break;
    }
}

rustdesk_send_mouse_event(session, MouseMove, 20, 20, 0);
rustdesk_send_key_event(session, 'A', true, 0);
rustdesk_send_key_event(session, 'A', false, 0);
rustdesk_send_clipboard(session, "hello from ffi", 14);

rustdesk_disconnect(session);
```

## FFI Functions Reference

### Session Lifecycle

#### rustdesk_connect

```c
RustdeskSession rustdesk_connect(const char* peer_id, 
                                  const char* password, 
                                  const void* options);
```

**Parameters:**
- `peer_id`: Remote peer ID (9-digit string)
- `password`: Connection password (optional, may be NULL)
- `options`: OptionMessage protobuf bytes (optional, may be NULL)

**Returns:**
- Valid session handle on success
- NULL on failure

**Meaning:**
- Success here means the session object and background connection attempt were created.
- Authentication, readiness, and first-frame availability are reported later via state polling and event callbacks.

**Ownership:** Caller owns returned handle, must free with `rustdesk_disconnect`.

**Thread safety:** Can be called from any thread. Session handle is thread-safe.

---

#### rustdesk_disconnect

```c
int rustdesk_disconnect(RustdeskSession session);
```

**Parameters:**
- `session`: Session handle from `rustdesk_connect`

**Returns:**
- 0 on success
- Negative error code on failure

**Minimal v1 common error codes:**
- `-1`: invalid argument
- `-2`: not ready
- `-3`: invalid state
- `-4`: encoding/conversion failure

**Ownership:** After call, session handle is invalid. Must not be used.

**Thread safety:** Can be called from any thread.

---

#### rustdesk_session_get_state

```c
int rustdesk_session_get_state(RustdeskSession session, 
                                char* error_msg, 
                                int error_msg_len);
```

**Returns:**
- 0: Connecting
- 1: Connected / ready for control
- 2: Disconnected
- -1: Error

**Notes:**
- A session may emit callback events such as `READY`, `CONNECTED`, and `FIRST_FRAME` around the same time.
- For robust integrations, combine `rustdesk_session_get_state()` polling with the event callback API.

---

### Video Frames

#### rustdesk_get_frame

```c
VideoFrameData* rustdesk_get_frame(RustdeskSession session);
```

**Returns:**
- Pointer to VideoFrameData if frame available
- NULL if no frames in queue

**Ownership:** Caller owns returned frame, must free with `rustdesk_free_frame`.

**VideoFrameData structure:**
```c
struct VideoFrameData {
    uint8_t* data;      // Raw RGBA frame data (owned by struct)
    size_t data_len;    // Data length in bytes
    uint32_t width;     // Frame width
    uint32_t height;    // Frame height
    uint32_t format;    // CodecRawRgba = 255 for minimal v1
    int64_t timestamp;  // Reserved for future timing data
    bool key_frame;     // Reserved for future encoded-frame variants
};
```

---

#### rustdesk_free_frame

```c
void rustdesk_free_frame(VideoFrameData* frame);
```

**Ownership:** Invalidates frame pointer. Must not be called twice.

---

### Input Events

#### rustdesk_send_mouse_event

```c
int rustdesk_send_mouse_event(RustdeskSession session,
                               MouseEventType event_type,
                               int x, int y, int buttons);
```

**MouseEventType:**
- 0: Move
- 1: LeftDown
- 2: LeftUp
- 3: RightDown
- 4: RightUp
- 5: Wheel

**Returns:** 0 on success, negative error code on failure.

---

#### rustdesk_send_key_event

```c
int rustdesk_send_key_event(RustdeskSession session,
                             int keycode,
                             bool down,
                             int modifiers);
```

**Modifiers:**
- Bit 0: Shift
- Bit 1: Ctrl
- Bit 2: Alt
- Bit 3: Meta (Cmd/Win)

---

### Clipboard

#### rustdesk_send_clipboard

```c
int rustdesk_send_clipboard(RustdeskSession session,
                            const char* text,
                            int len);
```

**Scope:**
- Minimal v1 supports **text clipboard send only**.
- Rich clipboard formats and file clipboard belong to later extensions.

---

### Event Callback

#### rustdesk_register_event_callback

```c
void rustdesk_register_event_callback(EventCallback callback, void* user_data);
```

**Current minimal event types:**
- `1`  = CONNECTED
- `2`  = DISCONNECTED
- `17` = CLIPBOARD_UPDATED
- `18` = READY
- `19` = FIRST_FRAME
- `20` = ERROR
- `21` = AUTH_REQUIRED

**Typical ordering:**
- `AUTH_REQUIRED` may be emitted before the password is submitted.
- `READY` indicates peer/session metadata has been received.
- `CONNECTED` indicates the session entered control-capable state.
- `FIRST_FRAME` indicates at least one frame is available via `rustdesk_get_frame()`.
- `DISCONNECTED` can occur after a successful short-lived validation session; callers should treat it as a lifecycle event, not automatically as an error.

**Recommendation:**
Use the callback channel for lifecycle/UI orchestration, and use the polling APIs for state/frame retrieval.

---

### File Transfer

#### rustdesk_file_browse

```c
char* rustdesk_file_browse(RustdeskSession session,
                            const char* path,
                            bool include_hidden);
```

**Returns:** JSON array string of files, or NULL on failure.

**Ownership:** Caller owns returned string, must free with `rustdesk_free_string`.

---

## Validation Matrix

Minimal v1 should be validated with at least the following cases:

1. **Connect**
   - `rustdesk_connect()` returns non-null
   - state begins at `CONNECTING`

2. **Auth / readiness**
   - callback may emit `AUTH_REQUIRED`
   - callback emits `READY`
   - callback emits `CONNECTED`

3. **Frame availability**
   - `rustdesk_get_queue_size()` becomes positive
   - `rustdesk_get_frame()` returns a raw RGBA frame

4. **Control**
   - `rustdesk_send_mouse_event()` returns `0`
   - `rustdesk_send_key_event()` returns `0`

5. **Clipboard**
   - `rustdesk_send_clipboard()` returns `0`
   - callback may emit `CLIPBOARD_UPDATED`

6. **Disconnect**
   - `rustdesk_disconnect()` returns `0`
   - callback emits `DISCONNECTED`

Reference validation scripts:
- `examples/python/minimal_v1_validate.py`
- `examples/c/minimal_v1_validate.cpp`

## Language-Specific Integration

### C Integration

Full example in `.planning/phase8/PLAN-01.md`.

**Key points:**
- Link against `librustdesk` with `-lrustdesk`
- Call `rustdesk_free_*` for all returned pointers
- Check return values for NULL/error codes

### Python Integration

Full example in `.planning/phase8/PLAN-02.md`.

**Key points:**
- Use ctypes to load library
- Define structure mappings
- Set argtypes/restype for each function
- Use `ctypes.POINTER()` for returned structs

### Swift Integration

```swift
import Foundation

// Load library
let handle = dlopen("./librustdesk.dylib", RTLD_NOW)

// Define functions
typealias ConnectFunc = @convention(c) (
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
    UnsafeRawPointer?
) -> UnsafeMutableRawPointer?

let connect = unsafeBitCast(
    dlsym(handle, "rustdesk_connect"),
    to: ConnectFunc.self
)

// Use
let peer_id = "123456789"
let session = connect(peer_id, nil, nil)
```

### Kotlin (Android) Integration

```kotlin
import android.os.Bundle

// Load native library
System.loadLibrary("rustdesk")

// Declare native methods
external fun rustdesk_connect(
    peerId: String,
    password: String?,
    options: ByteArray?
): Long

external fun rustdesk_disconnect(session: Long): Int

// Use
val session = rustdesk_connect("peer_id", null, null)
rustdesk_disconnect(session)
```

## Memory Management Rules

### Core → Integrator

| Function | Ownership | Free Function |
|----------|-----------|---------------|
| `rustdesk_connect` | Caller owns handle | `rustdesk_disconnect` |
| `rustdesk_get_frame` | Caller owns frame | `rustdesk_free_frame` |
| `rustdesk_file_browse` | Caller owns string | `rustdesk_free_string` |

### Integrator → Core

| Function | Ownership | Notes |
|----------|-----------|-------|
| `rustdesk_send_clipboard` | Caller owns text | Core copies internally |
| `rustdesk_file_upload` | Caller owns paths | Core copies strings |

**Golden rule:** Anything returned by FFI must be freed. Anything passed to FFI can be kept by caller.

## Thread Safety

- All FFI functions are thread-safe
- Session handles can be shared across threads
- Frame queue is internally synchronized
- Do not call functions on invalid handles

## Error Handling

All functions return error codes:
- **0:** Success
- **-1:** Invalid handle
- **-2:** Null pointer
- **-3:** Buffer too small
- **-4:** Connection failed
- **-5:** Authentication failed
- **-6:** Not authorized
- **-7:** Feature disabled
- **-8:** Internal error

Always check return values. Zero is success.

## Feature Gates

RustDesk supports optional features compiled into library:

| Feature | Functions Available |
|---------|---------------------|
| `core` | Session, video, input |
| `2fa` | 2FA authentication |
| `terminal` | Terminal service |
| `port-forward` | Port forwarding |
| `lan-discovery` | LAN peer discovery |

Check library features before using optional functions.

## Troubleshooting

### Connection fails
- Verify peer ID is valid (9 digits)
- Check password if required
- Ensure RustDesk server is accessible

### No video frames
- Check session state (must be Connected)
- Verify frame queue size with `rustdesk_get_queue_size`
- Ensure remote peer has screen capture enabled

### Memory leaks
- Always call `rustdesk_free_*` for returned pointers
- Do not call free twice on same pointer
- Use ASAN/valgrind to verify

## References

- Protocol spec: `docs/PROTOCOL.md`
- C header: `rustdesk.h`
- FFI source: `src/ffi/`
- Examples: `.planning/phase8/PLAN-*.md`

---
*Guide version: 1.0*