---
name: whiteboard
description: "Skill for the Whiteboard area of rustdesk. 31 symbols across 11 files."
---

# Whiteboard

31 symbols | 11 files | Cohesion: 68%

## When to Use

- Working with code in `src/`
- Understanding how new_listener, run, start_ipc work
- Modifying whiteboard-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/whiteboard/win_linux.rs` | create_font_face, move_to, line_to, quad_to, close (+1) |
| `src/whiteboard/server.rs` | run, start_ipc, argb_to_rgba, retain_active, get_radius_alpha |
| `src/whiteboard/linux.rs` | get_display_from_xwayland, preset_env, is_supported, run, new |
| `src/ipc.rs` | new_listener, get_pid_file, check_pid, write_pid |
| `src/whiteboard/macos.rs` | draw_cursors, set_window_properties, create_windows, create_event_loop |
| `src/whiteboard/client.rs` | update_whiteboard, tx_send_event |
| `libs/hbb_common/src/config.rs` | ipc_path |
| `src/whiteboard/windows.rs` | create_event_loop |
| `libs/scrap/src/common/hwcodec.rs` | clear |
| `src/flutter_ffi.rs` | main_hide_dock |

## Entry Points

Start here when exploring this area:

- **`new_listener`** (Function) — `src/ipc.rs:429`
- **`run`** (Function) — `src/whiteboard/server.rs:34`
- **`start_ipc`** (Function) — `src/whiteboard/server.rs:47`
- **`ipc_path`** (Function) — `libs/hbb_common/src/config.rs:805`
- **`create_event_loop`** (Function) — `src/whiteboard/windows.rs:17`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `new_listener` | Function | `src/ipc.rs` | 429 |
| `run` | Function | `src/whiteboard/server.rs` | 34 |
| `start_ipc` | Function | `src/whiteboard/server.rs` | 47 |
| `ipc_path` | Function | `libs/hbb_common/src/config.rs` | 805 |
| `create_event_loop` | Function | `src/whiteboard/windows.rs` | 17 |
| `create_font_face` | Function | `src/whiteboard/win_linux.rs` | 156 |
| `argb_to_rgba` | Function | `src/whiteboard/server.rs` | 136 |
| `retain_active` | Function | `src/whiteboard/server.rs` | 153 |
| `get_radius_alpha` | Function | `src/whiteboard/server.rs` | 157 |
| `clear` | Function | `libs/scrap/src/common/hwcodec.rs` | 651 |
| `main_hide_dock` | Function | `src/flutter_ffi.rs` | 2394 |
| `create_event_loop` | Function | `src/whiteboard/macos.rs` | 218 |
| `hide_dock` | Function | `src/platform/macos.rs` | 1016 |
| `draw_text` | Function | `src/whiteboard/win_linux.rs` | 47 |
| `is_supported` | Function | `src/whiteboard/linux.rs` | 92 |
| `run` | Function | `src/whiteboard/linux.rs` | 96 |
| `update_whiteboard` | Function | `src/whiteboard/client.rs` | 83 |
| `get_pid_file` | Function | `src/ipc.rs` | 1021 |
| `check_pid` | Function | `src/ipc.rs` | 1027 |
| `write_pid` | Function | `src/ipc.rs` | 1057 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Start_os_service → New` | cross_community | 7 |
| `Start_os_service → Write_all` | cross_community | 7 |
| `Start_os_service → Ipc_path` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Server | 5 calls |
| Plugin | 4 calls |
| Flutter_hbb | 4 calls |
| Platform | 2 calls |
| Macos | 1 calls |
| Win | 1 calls |
| CustomActions | 1 calls |

## How to Explore

1. `gitnexus_context({name: "new_listener"})` — see callers and callees
2. `gitnexus_query({query: "whiteboard"})` — find related execution flows
3. Read key files listed above for implementation details
