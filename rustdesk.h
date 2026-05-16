#ifndef RUSTDESK_FFI_H
#define RUSTDESK_FFI_H

#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void *RustdeskSession;

/* Session state codes */
#define RUSTDESK_STATE_ERROR        (-1)
#define RUSTDESK_STATE_CONNECTING   0
#define RUSTDESK_STATE_CONNECTED    1
#define RUSTDESK_STATE_DISCONNECTED 2

/* Minimal v1 return codes */
#define RUSTDESK_OK                    0
#define RUSTDESK_ERR_INVALID_ARGUMENT (-1)
#define RUSTDESK_ERR_NOT_READY        (-2)
#define RUSTDESK_ERR_INVALID_STATE    (-3)
#define RUSTDESK_ERR_ENCODING         (-4)

/* Minimal v1 event codes */
#define RUSTDESK_EVENT_CONNECTED         1
#define RUSTDESK_EVENT_DISCONNECTED      2
#define RUSTDESK_EVENT_CLIPBOARD_UPDATED 17
#define RUSTDESK_EVENT_READY             18
#define RUSTDESK_EVENT_FIRST_FRAME       19
#define RUSTDESK_EVENT_ERROR             20
#define RUSTDESK_EVENT_AUTH_REQUIRED     21

typedef enum MouseEventType {
  MouseMove = 0,
  MouseLeftDown = 1,
  MouseLeftUp = 2,
  MouseRightDown = 3,
  MouseRightUp = 4,
  MouseWheel = 5,
} MouseEventType;

typedef enum CodecFormatFFI {
  CodecVP9 = 0,
  CodecVP8 = 1,
  CodecAV1 = 2,
  CodecH264 = 3,
  CodecH265 = 4,
  CodecRawRgba = 255,
} CodecFormatFFI;

typedef struct VideoFrameData {
  uint8_t *data;
  size_t data_len;
  uint32_t width;
  uint32_t height;
  CodecFormatFFI format;
  int64_t timestamp;
  bool key_frame;
} VideoFrameData;

typedef void (*EventCallback)(uint32_t event_type, const void *event_data, void *user_data);

void rustdesk_register_event_callback(EventCallback callback, void *user_data);
RustdeskSession rustdesk_connect(const char *peer_id, const char *password, const void *options);
int rustdesk_disconnect(RustdeskSession session);
int rustdesk_session_get_state(RustdeskSession session, char *error_msg, int error_msg_len);
VideoFrameData *rustdesk_get_frame(RustdeskSession session);
void rustdesk_free_frame(VideoFrameData *frame);
int rustdesk_get_queue_size(RustdeskSession session);
int rustdesk_send_mouse_event(RustdeskSession session, MouseEventType event_type, int x, int y, int buttons);
int rustdesk_send_key_event(RustdeskSession session, int keycode, bool down, int modifiers);
int rustdesk_send_clipboard(RustdeskSession session, const char *text, int len);
void rustdesk_free_string(char *s);

/* The functions below are legacy/bootstrap exports and are not part of the minimal v1 remote-core contract. */
bool rustdesk_core_main(void);
void handle_applicationShouldOpenUntitledFile(void);
char **rustdesk_core_main_args(int *args_len);
void free_c_args(char **ptr, int len);
int32_t get_rustdesk_app_name(uint16_t *buffer, int32_t length);
const uint8_t *session_get_rgba(const uint32_t *session_uuid_str, uintptr_t display);

#ifdef __cplusplus
}
#endif

#endif