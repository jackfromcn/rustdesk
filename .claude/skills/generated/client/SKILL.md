---
name: client
description: "Skill for the Client area of rustdesk. 218 symbols across 20 files."
---

# Client

218 symbols | 20 files | Cohesion: 78%

## When to Use

- Working with code in `src/`
- Understanding how get_view_style, get_scroll_style, get_edge_scroll_edge_thickness work
- Modifying client-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/flutter_ffi.rs` | session_get_remember, session_get_toggle_option, session_get_toggle_option_sync, session_get_option, session_get_enable_trusted_devices (+53) |
| `src/client/io_loop.rs` | check_view_camera_support, is_support_virtual_display, send_toggle_virtual_display_msg, send_toggle_privacy_mode_msg, contains_key_frame (+26) |
| `libs/hbb_common/src/fs.rs` | get_buf_data, default_overwrite_strategy, id, file_num, remove_download_file (+26) |
| `src/ui_session_interface.rs` | get_view_style, get_scroll_style, get_edge_scroll_edge_thickness, get_image_quality, get_trackpad_speed (+25) |
| `src/common_old_backup.rs` | is_support_multi_ui_session, is_support_multi_ui_session_num, is_support_file_copy_paste, is_support_file_copy_paste_num, is_support_remote_print (+9) |
| `src/client.rs` | save_ui_flutter, get_key_terminal_service_id, try_stop_clipboard, update_direct, on_establish_connection_error (+6) |
| `src/client/file_trait.rs` | cancel_job, remove_dir_all, send_files, add_job, resume_job (+5) |
| `src/common.rs` | is_support_file_copy_paste, is_support_file_copy_paste_num, is_support_remote_print, is_support_file_paste_if_macos, is_support_file_transfer_resume (+3) |
| `src/flutter.rs` | get_rgba, char_to_session_id, session_get_rgba_size, session_get_rgba, get_session_by_session_id (+1) |
| `libs/clipboard/src/lib.rs` | get_conn_id, get_rx_cliprdr_client, is_stopping_allowed, is_beginning_message, get_client_conn_id |

## Entry Points

Start here when exploring this area:

- **`get_view_style`** (Function) — `src/ui_session_interface.rs:237`
- **`get_scroll_style`** (Function) — `src/ui_session_interface.rs:241`
- **`get_edge_scroll_edge_thickness`** (Function) — `src/ui_session_interface.rs:245`
- **`get_image_quality`** (Function) — `src/ui_session_interface.rs:249`
- **`get_trackpad_speed`** (Function) — `src/ui_session_interface.rs:261`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `get_view_style` | Function | `src/ui_session_interface.rs` | 237 |
| `get_scroll_style` | Function | `src/ui_session_interface.rs` | 241 |
| `get_edge_scroll_edge_thickness` | Function | `src/ui_session_interface.rs` | 245 |
| `get_image_quality` | Function | `src/ui_session_interface.rs` | 249 |
| `get_trackpad_speed` | Function | `src/ui_session_interface.rs` | 261 |
| `get_reverse_mouse_wheel` | Function | `src/ui_session_interface.rs` | 315 |
| `get_displays_as_individual_windows` | Function | `src/ui_session_interface.rs` | 319 |
| `get_use_all_my_displays_for_the_remote_session` | Function | `src/ui_session_interface.rs` | 327 |
| `save_flutter_option` | Function | `src/ui_session_interface.rs` | 368 |
| `is_recording` | Function | `src/ui_session_interface.rs` | 462 |
| `get_remember` | Function | `src/ui_session_interface.rs` | 499 |
| `input_os_password` | Function | `src/ui_session_interface.rs` | 677 |
| `get_enable_trusted_devices` | Function | `src/ui_session_interface.rs` | 1388 |
| `session_get_remember` | Function | `src/flutter_ffi.rs` | 202 |
| `session_get_toggle_option` | Function | `src/flutter_ffi.rs` | 210 |
| `session_get_toggle_option_sync` | Function | `src/flutter_ffi.rs` | 218 |
| `session_get_option` | Function | `src/flutter_ffi.rs` | 223 |
| `session_get_enable_trusted_devices` | Function | `src/flutter_ffi.rs` | 249 |
| `session_is_multi_ui_session` | Function | `src/flutter_ffi.rs` | 292 |
| `session_get_is_recording` | Function | `src/flutter_ffi.rs` | 306 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Handle_msg_from_peer → Enable_hwcodec_option` | cross_community | 6 |
| `Handle_msg_from_peer → Already_set` | cross_community | 6 |
| `Handle_msg_from_peer → Send` | cross_community | 6 |
| `Handle_msg_from_peer → Eq` | cross_community | 5 |
| `Handle_msg_from_peer → New` | cross_community | 5 |
| `Handle_msg_from_peer → New` | cross_community | 4 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Android | 8 calls |
| Cluster_139 | 7 calls |
| Cluster_68 | 5 calls |
| Cluster_228 | 4 calls |
| Plugin | 3 calls |
| Cluster_63 | 3 calls |
| Cluster_66 | 3 calls |
| Platform | 3 calls |

## How to Explore

1. `gitnexus_context({name: "get_view_style"})` — see callers and callees
2. `gitnexus_query({query: "client"})` — find related execution flows
3. Read key files listed above for implementation details
