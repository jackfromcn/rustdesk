#!/usr/bin/env python3
import ctypes
import sys
import time

class VideoFrameData(ctypes.Structure):
    _fields_ = [
        ('data', ctypes.POINTER(ctypes.c_uint8)),
        ('data_len', ctypes.c_size_t),
        ('width', ctypes.c_uint32),
        ('height', ctypes.c_uint32),
        ('format', ctypes.c_uint32),
        ('timestamp', ctypes.c_int64),
        ('key_frame', ctypes.c_bool),
    ]

EVENT_CONNECTED = 1
EVENT_DISCONNECTED = 2
EVENT_CLIPBOARD_UPDATED = 17
EVENT_READY = 18
EVENT_FIRST_FRAME = 19
EVENT_ERROR = 20
EVENT_AUTH_REQUIRED = 21

CALLBACK = ctypes.CFUNCTYPE(None, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_void_p)

def load_lib():
    return ctypes.CDLL('../../target/release/liblibrustdesk.dylib')


def configure(lib):
    lib.rustdesk_register_event_callback.argtypes = [CALLBACK, ctypes.c_void_p]
    lib.rustdesk_connect.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p]
    lib.rustdesk_connect.restype = ctypes.c_void_p
    lib.rustdesk_session_get_state.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
    lib.rustdesk_session_get_state.restype = ctypes.c_int
    lib.rustdesk_get_frame.argtypes = [ctypes.c_void_p]
    lib.rustdesk_get_frame.restype = ctypes.POINTER(VideoFrameData)
    lib.rustdesk_free_frame.argtypes = [ctypes.POINTER(VideoFrameData)]
    lib.rustdesk_get_queue_size.argtypes = [ctypes.c_void_p]
    lib.rustdesk_get_queue_size.restype = ctypes.c_int
    lib.rustdesk_send_mouse_event.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int]
    lib.rustdesk_send_mouse_event.restype = ctypes.c_int
    lib.rustdesk_send_key_event.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_bool, ctypes.c_int]
    lib.rustdesk_send_key_event.restype = ctypes.c_int
    lib.rustdesk_send_clipboard.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
    lib.rustdesk_send_clipboard.restype = ctypes.c_int
    lib.rustdesk_disconnect.argtypes = [ctypes.c_void_p]
    lib.rustdesk_disconnect.restype = ctypes.c_int


def main():
    if len(sys.argv) < 3:
        print('usage: python3 minimal_v1_validate.py <peer_id> <password>')
        return 2

    peer = sys.argv[1].encode()
    password = sys.argv[2].encode()
    lib = load_lib()
    configure(lib)

    events = []
    def on_event(event_type, event_data, user_data):
        events.append(event_type)
        print('event:', event_type)

    cb = CALLBACK(on_event)
    lib.rustdesk_register_event_callback(cb, None)

    sess = lib.rustdesk_connect(peer, password, None)
    if not sess:
        print('FAIL: null session')
        return 2

    err = ctypes.create_string_buffer(512)
    connected = False
    frame_ok = False
    mouse_ok = False
    key_ok = False
    clipboard_ok = False

    for i in range(200):
        err.value = b''
        state = lib.rustdesk_session_get_state(sess, err, len(err))
        q = lib.rustdesk_get_queue_size(sess)
        print('poll', i, 'state', state, 'queue', q, 'err', err.value.decode(errors='ignore'))

        if q > 0 and not frame_ok:
            frame = lib.rustdesk_get_frame(sess)
            if frame:
                print('frame', frame.contents.width, frame.contents.height, frame.contents.data_len, frame.contents.format)
                lib.rustdesk_free_frame(frame)
                frame_ok = True

        if EVENT_CONNECTED in events:
            connected = True

        if frame_ok and not mouse_ok:
            mouse_ok = (lib.rustdesk_send_mouse_event(sess, 0, 20, 20, 0) == 0)
        if frame_ok and mouse_ok and not key_ok:
            key_ok = (
                lib.rustdesk_send_key_event(sess, ord('A'), True, 0) == 0 and
                lib.rustdesk_send_key_event(sess, ord('A'), False, 0) == 0
            )
        if frame_ok and key_ok and not clipboard_ok:
            txt = b'hello from minimal-v1'
            clipboard_ok = (lib.rustdesk_send_clipboard(sess, txt, len(txt)) == 0)

        if connected and frame_ok and mouse_ok and key_ok and clipboard_ok:
            break

        if state == -1:
            print('FAIL: error state')
            lib.rustdesk_disconnect(sess)
            return 3

        time.sleep(0.2)

    ret = lib.rustdesk_disconnect(sess)
    print('disconnect ret', ret)

    ok = connected and frame_ok and mouse_ok and key_ok and clipboard_ok and ret == 0
    print('events seen', events)
    print('RESULT:', 'PASS' if ok else 'FAIL')
    return 0 if ok else 4

if __name__ == '__main__':
    raise SystemExit(main())
