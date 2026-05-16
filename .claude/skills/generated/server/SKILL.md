---
name: server
description: "Skill for the Server area of rustdesk. 430 symbols across 59 files."
---

# Server

430 symbols | 59 files | Cohesion: 76%

## When to Use

- Working with code in `src/`
- Understanding how get_fingerprint, change_id_shared_, remove_trusted_devices work
- Modifying server-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/server/input_service.rs` | set_relative_mouse_active, clear_relative_mouse_active, enigo_ignore_flags, handle_mouse, handle_mouse_ (+86) |
| `src/server/portable_service.rs` | i32_to_vec, ptr_to_i32, counter_equal, increase_counter, set_para (+28) |
| `src/server/video_service.rs` | new, reset, set_send, try_wait_next, is_monitor (+22) |
| `src/server/display_service.rs` | run, check_changed, check_display_changed, check_get_displays_changed_msg, check_displays_changed (+16) |
| `src/server/service.rs` | is_option_true, set_option_bool, has_subscribes, snapshot, name (+14) |
| `src/server/audio_service.rs` | get_voice_call_input_device, new, align_to_32, run, reset (+12) |
| `src/server/terminal_service.rs` | is_service_specified_user, remove_service, get_service, cleanup_inactive_services, new (+11) |
| `src/server/uinput.rs` | input_text_wayland, input_char_wayland_key_event, can_input_via_keysym, char_to_keysym, map_key (+9) |
| `src/server/terminal_helper.rs` | into_raw, as_raw, wait_for_pipe_connection, as_raw, get_user_sid_from_token (+9) |
| `libs/scrap/src/common/codec.rs` | negotiated_codec, set_fallback, use_i444, enable_vram_option, enable_directx_capture (+8) |

## Entry Points

Start here when exploring this area:

- **`get_fingerprint`** (Function) — `src/ui_interface.rs:1169`
- **`change_id_shared_`** (Function) — `src/ui_interface.rs:1354`
- **`remove_trusted_devices`** (Function) — `src/ipc.rs:1327`
- **`clear_trusted_devices`** (Function) — `src/ipc.rs:1334`
- **`get_id`** (Function) — `src/ipc.rs:1339`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `PeerInfo` | Class | `flutter/lib/models/model.dart` | 3754 |
| `get_fingerprint` | Function | `src/ui_interface.rs` | 1169 |
| `change_id_shared_` | Function | `src/ui_interface.rs` | 1354 |
| `remove_trusted_devices` | Function | `src/ipc.rs` | 1327 |
| `clear_trusted_devices` | Function | `src/ipc.rs` | 1334 |
| `get_id` | Function | `src/ipc.rs` | 1339 |
| `main_get_connect_status` | Function | `src/flutter_ffi.rs` | 1123 |
| `is_main` | Function | `src/common.rs` | 223 |
| `pk_to_fingerprint` | Function | `src/common.rs` | 1826 |
| `get_control_permission` | Function | `src/common.rs` | 2603 |
| `insert_switch_sides_uuid` | Function | `src/server/connection.rs` | 4940 |
| `get_control_permission_state` | Function | `src/server/connection.rs` | 5466 |
| `get_voice_call_input_device` | Function | `src/server/audio_service.rs` | 43 |
| `handle_plugin` | Function | `src/plugin/ipc.rs` | 177 |
| `get_plugin_option` | Function | `src/plugin/config.rs` | 267 |
| `set_plugin_option` | Function | `src/plugin/config.rs` | 293 |
| `is_pro` | Function | `src/hbbs_http/sync.rs` | 307 |
| `is_valid_custom_id` | Function | `libs/hbb_common/src/lib.rs` | 256 |
| `get_uuid` | Function | `libs/hbb_common/src/lib.rs` | 317 |
| `is_domain_port_str` | Function | `libs/hbb_common/src/lib.rs` | 413 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Run → WaylandEnv` | cross_community | 7 |
| `Run → New` | cross_community | 7 |
| `Run → WaylandDisplayInfo` | cross_community | 7 |
| `Run → Try_xrandr_primary` | cross_community | 7 |
| `Run → Displays_to_key` | cross_community | 7 |
| `Run → PipewireDisplayOffsetCache` | cross_community | 7 |
| `Start_os_service → New` | cross_community | 7 |
| `Start_os_service → Write_all` | cross_community | 7 |
| `Run → Displays` | cross_community | 6 |
| `Run → New` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Platform | 17 calls |
| Plugin | 15 calls |
| Wayland | 8 calls |
| Cluster_92 | 6 calls |
| Whiteboard | 5 calls |
| Cluster_129 | 3 calls |
| Client | 3 calls |
| Cluster_303 | 2 calls |

## How to Explore

1. `gitnexus_context({name: "get_fingerprint"})` — see callers and callees
2. `gitnexus_query({query: "server"})` — find related execution flows
3. Read key files listed above for implementation details
