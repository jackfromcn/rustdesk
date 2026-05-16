/**
 * RustDesk FFI C Example
 *
 * Demonstrates basic FFI usage for remote desktop connections.
 * Note: This example requires a RustDesk server to actually connect.
 *
 * Build: gcc -o client client.c -I../.. -L../../target/debug -lrustdesk
 * Run: ./client
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rustdesk.h"

// Event callback handler
void event_handler(uint32_t event_type, const void *event_data, void *user_data) {
    printf("Event received: type=%u\n", event_type);

    switch (event_type) {
        case 1:
            printf("  -> Connected to remote peer\n");
            break;
        case 2:
            printf("  -> Disconnected from remote peer\n");
            break;
        case 5:
            printf("  -> Display changed\n");
            break;
        case 7:
            printf("  -> Quality status update\n");
            break;
        case 8:
            printf("  -> Cursor data received\n");
            break;
        case 4:
            printf("  -> Message box event\n");
            break;
        default:
            printf("  -> Unknown event type\n");
            break;
    }
}

int main(int argc, char *argv[]) {
    printf("RustDesk FFI C Example\n");
    printf("======================\n\n");

    // Register event callback
    printf("1. Registering event callback...\n");
    rustdesk_register_event_callback(event_handler, NULL);
    printf("   Event callback registered.\n\n");

    // Check build constants
    printf("2. Checking build constants:\n");
    printf("   Constants are defined in Rust side and validated at runtime\n");
    printf("\n");

    // Connection example (requires server)
    const char *peer_id = argc > 1 ? argv[1] : "test_peer";
    const char *password = argc > 2 ? argv[2] : NULL;

    printf("3. Attempting connection (requires RustDesk server):\n");
    printf("   Peer ID: %s\n", peer_id);
    printf("   Password: %s\n", password ? "(provided)" : "(none)");

    // Note: rustdesk_connect requires a running RustDesk server
    // This will return NULL if no server is available
    RustdeskSession session = rustdesk_connect(peer_id, password, NULL);

    if (session) {
        printf("   Connection initiated!\n\n");

        // Process events
        printf("4. Processing events...\n");
        int state = rustdesk_session_get_state(session, NULL, 0);
        printf("   Session state: %d\n", state);

        // Get video frames (pull-based)
        printf("5. Getting video frames...\n");
        int queue_size = rustdesk_get_queue_size(session);
        printf("   Frame queue size: %d\n", queue_size);

        if (queue_size > 0) {
            VideoFrameData *frame = rustdesk_get_frame(session);
            if (frame) {
                printf("   Frame: %ux%u, format=%u, len=%zu\n",
                       frame->width, frame->height,
                       (uint32_t)frame->format, frame->data_len);
                rustdesk_free_frame(frame);
            }
        }

        // Send input (example)
        printf("6. Sending input (example)...\n");
        int result = rustdesk_send_mouse_event(session, MouseMove, 100, 100, 0);
        printf("   Mouse move result: %d\n", result);

        result = rustdesk_send_key_event(session, 65, true, 0); // 'A' key down
        printf("   Key press result: %d\n", result);

        // Disconnect
        printf("7. Disconnecting...\n");
        result = rustdesk_disconnect(session);
        printf("   Disconnect result: %d\n", result);
    } else {
        printf("   Connection failed - no RustDesk server available.\n");
        printf("   To test full functionality:\n");
        printf("   1. Install RustDesk server (hbbs + hbbr)\n");
        printf("   2. Run: ./client <peer_id> <password>\n");
    }

    printf("\nExample complete.\n");
    return 0;
}