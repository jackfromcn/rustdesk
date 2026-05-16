"""
RustDesk FFI Python Example

Demonstrates basic FFI usage using ctypes for remote desktop connections.
Note: This example requires a RustDesk server to actually connect.

Build: cargo build
Run: python client.py [peer_id] [password]
"""

import ctypes
import ctypes.util
import os
import sys

# Load the RustDesk library
def load_rustdesk_lib():
    # Try different paths based on platform
    lib_paths = [
        os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'release', 'liblibrustdesk.dylib'),
        os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'release', 'librustdesk.dylib'),
        os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'debug', 'librustdesk.dylib'),
        os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'debug', 'librustdesk.so'),
        os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'debug', 'rustdesk.dll'),
    ]

    for path in lib_paths:
        if os.path.exists(path):
            return ctypes.CDLL(path)

    # Try system library search
    lib_name = ctypes.util.find_library('rustdesk')
    if lib_name:
        return ctypes.CDLL(lib_name)

    raise RuntimeError("Could not find RustDesk library. Build it with: cargo build")

# Load library
lib = load_rustdesk_lib()

# Define FFI types
class VideoFrameData(ctypes.Structure):
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("data_len", ctypes.c_size_t),
        ("width", ctypes.c_uint32),
        ("height", ctypes.c_uint32),
        ("format", ctypes.c_uint32),
        ("timestamp", ctypes.c_int64),
        ("key_frame", ctypes.c_bool),
    ]

# Event constants
RUSTDESK_EVENT_CONNECTED = 1
RUSTDESK_EVENT_DISCONNECTED = 2
RUSTDESK_EVENT_PERMISSION_CHANGED = 3
RUSTDESK_EVENT_MSGBOX = 4
RUSTDESK_EVENT_DISPLAY_CHANGED = 5
RUSTDESK_EVENT_SWITCH_DISPLAY = 6
RUSTDESK_EVENT_QUALITY_STATUS = 7
RUSTDESK_EVENT_CURSOR_DATA = 8
RUSTDESK_EVENT_CURSOR_POSITION = 9

# Mouse event types
MOUSE_TYPE_MOVE = 0
MOUSE_TYPE_LEFT_DOWN = 1
MOUSE_TYPE_LEFT_UP = 2
MOUSE_TYPE_RIGHT_DOWN = 3
MOUSE_TYPE_RIGHT_UP = 4
MOUSE_TYPE_WHEEL = 5

# Configure function signatures
lib.rustdesk_connect.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p]
lib.rustdesk_connect.restype = ctypes.c_void_p

lib.rustdesk_disconnect.argtypes = [ctypes.c_void_p]
lib.rustdesk_disconnect.restype = ctypes.c_int

lib.rustdesk_session_get_state.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
lib.rustdesk_session_get_state.restype = ctypes.c_int

lib.rustdesk_get_frame.argtypes = [ctypes.c_void_p]
lib.rustdesk_get_frame.restype = ctypes.POINTER(VideoFrameData)

lib.rustdesk_free_frame.argtypes = [ctypes.POINTER(VideoFrameData)]
lib.rustdesk_free_frame.restype = None

lib.rustdesk_get_queue_size.argtypes = [ctypes.c_void_p]
lib.rustdesk_get_queue_size.restype = ctypes.c_int

lib.rustdesk_send_mouse_event.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_int, ctypes.c_int, ctypes.c_int]
lib.rustdesk_send_mouse_event.restype = ctypes.c_int

lib.rustdesk_send_key_event.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_bool, ctypes.c_int]
lib.rustdesk_send_key_event.restype = ctypes.c_int

lib.rustdesk_send_clipboard.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
lib.rustdesk_send_clipboard.restype = ctypes.c_int

lib.rustdesk_free_string.argtypes = [ctypes.c_char_p]
lib.rustdesk_free_string.restype = None

# Event callback type
EventCallback = ctypes.CFUNCTYPE(None, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_void_p)

def event_handler(event_type, event_data, user_data):
    """Handle events from RustDesk core."""
    print(f"Event received: type={event_type}")

    event_names = {
        RUSTDESK_EVENT_CONNECTED: "Connected",
        RUSTDESK_EVENT_DISCONNECTED: "Disconnected",
        RUSTDESK_EVENT_DISPLAY_CHANGED: "Display Changed",
        RUSTDESK_EVENT_QUALITY_STATUS: "Quality Status",
        RUSTDESK_EVENT_CURSOR_DATA: "Cursor Data",
        RUSTDESK_EVENT_MSGBOX: "Message Box",
    }

    name = event_names.get(event_type, "Unknown")
    print(f"  -> {name}")

def main():
    print("RustDesk FFI Python Example")
    print("=" * 30)
    print()

    # Register event callback
    print("1. Registering event callback...")
    callback = EventCallback(event_handler)
    lib.rustdesk_register_event_callback(callback, None)
    print("   Event callback registered.")
    print()

    # Get connection parameters
    peer_id = sys.argv[1] if len(sys.argv) > 1 else "test_peer"
    password = sys.argv[2] if len(sys.argv) > 2 else None

    print("2. Attempting connection (requires RustDesk server):")
    print(f"   Peer ID: {peer_id}")
    print(f"   Password: {password or '(none)'}")

    # Connect
    session = lib.rustdesk_connect(
        peer_id.encode('utf-8'),
        password.encode('utf-8') if password else None,
        None
    )

    if session:
        print("   Connection initiated!")
        print()

        # Check session state
        print("3. Checking session state...")
        state = lib.rustdesk_session_get_state(session, None, 0)
        print(f"   Session state: {state}")
        print()

        # Get video frames
        print("4. Getting video frames...")
        queue_size = lib.rustdesk_get_queue_size(session)
        print(f"   Frame queue size: {queue_size}")

        if queue_size > 0:
            frame = lib.rustdesk_get_frame(session)
            if frame:
                print(f"   Frame: {frame.contents.width}x{frame.contents.height}, "
                      f"format={frame.contents.format}, len={frame.contents.data_len}")
                lib.rustdesk_free_frame(frame)
        print()

        # Send input examples
        print("5. Sending input (example)...")
        result = lib.rustdesk_send_mouse_event(session, MOUSE_TYPE_MOVE, 100, 100, 0)
        print(f"   Mouse move result: {result}")

        result = lib.rustdesk_send_key_event(session, 65, True, 0)  # 'A' key down
        print(f"   Key press result: {result}")
        print()

        # Disconnect
        print("6. Disconnecting...")
        result = lib.rustdesk_disconnect(session)
        print(f"   Disconnect result: {result}")
    else:
        print("   Connection failed - no RustDesk server available.")
        print()
        print("   To test full functionality:")
        print("   1. Install RustDesk server (hbbs + hbbr)")
        print("   2. Run: python client.py <peer_id> <password>")

    print()
    print("Example complete.")

if __name__ == "__main__":
    main()