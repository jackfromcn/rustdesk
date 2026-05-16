#include <chrono>
#include <cstdio>
#include <thread>
#include <vector>
#include "rustdesk.h"

static std::vector<uint32_t> g_events;

static void on_event(uint32_t event_type, const void *, void *) {
    g_events.push_back(event_type);
    std::printf("event: %u\n", event_type);
}

static bool seen(uint32_t event_type) {
    for (auto e : g_events) {
        if (e == event_type) return true;
    }
    return false;
}

int main(int argc, char **argv) {
    if (argc < 3) {
        std::printf("usage: %s <peer_id> <password>\n", argv[0]);
        return 2;
    }

    rustdesk_register_event_callback(on_event, nullptr);
    RustdeskSession session = rustdesk_connect(argv[1], argv[2], nullptr);
    if (!session) {
        std::printf("FAIL: null session\n");
        return 2;
    }

    char err[512] = {0};
    bool connected = false;
    bool frame_ok = false;
    bool mouse_ok = false;
    bool key_ok = false;
    bool clipboard_ok = false;

    for (int i = 0; i < 200; ++i) {
        err[0] = '\0';
        int state = rustdesk_session_get_state(session, err, sizeof(err));
        int q = rustdesk_get_queue_size(session);
        std::printf("poll %d state %d queue %d err %s\n", i, state, q, err);

        if (q > 0 && !frame_ok) {
            VideoFrameData *frame = rustdesk_get_frame(session);
            if (frame) {
                std::printf("frame %u %u %zu %u\n", frame->width, frame->height, frame->data_len, (unsigned)frame->format);
                rustdesk_free_frame(frame);
                frame_ok = true;
            }
        }

        if (seen(RUSTDESK_EVENT_CONNECTED)) connected = true;

        if (frame_ok && !mouse_ok) {
            mouse_ok = (rustdesk_send_mouse_event(session, MouseMove, 20, 20, 0) == RUSTDESK_OK);
        }
        if (frame_ok && mouse_ok && !key_ok) {
            key_ok = (rustdesk_send_key_event(session, 'A', true, 0) == RUSTDESK_OK &&
                      rustdesk_send_key_event(session, 'A', false, 0) == RUSTDESK_OK);
        }
        if (frame_ok && key_ok && !clipboard_ok) {
            const char *txt = "hello from minimal-v1";
            clipboard_ok = (rustdesk_send_clipboard(session, txt, 21) == RUSTDESK_OK);
        }

        if (connected && frame_ok && mouse_ok && key_ok && clipboard_ok) {
            break;
        }

        if (state == RUSTDESK_STATE_ERROR) {
            std::printf("FAIL: error state\n");
            rustdesk_disconnect(session);
            return 3;
        }

        std::this_thread::sleep_for(std::chrono::milliseconds(200));
    }

    int ret = rustdesk_disconnect(session);
    std::printf("disconnect ret %d\n", ret);
    bool ok = connected && frame_ok && mouse_ok && key_ok && clipboard_ok && ret == RUSTDESK_OK;
    std::printf("RESULT: %s\n", ok ? "PASS" : "FAIL");
    return ok ? 0 : 4;
}
