---
name: platform
description: "Skill for the Platform area of rustdesk. 533 symbols across 58 files."
---

# Platform

533 symbols | 58 files | Cohesion: 80%

## When to Use

- Working with code in `src/`
- Understanding how main_set_cursor_position, get_cursor_pos, set_cursor_pos work
- Modifying platform-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/platform/windows.rs` | is_windows_10_or_greater, is_win_10_or_greater, bootstrap, install_me, run_before_uninstall (+150) |
| `src/platform/linux.rs` | get_cursor_pos, set_cursor_pos, sleep_millis, stop_server, stop_rustdesk_servers (+73) |
| `src/platform/macos.rs` | get_update_temp_dir, get_update_temp_dir_string, try_remove_temp_update_dir, update_from_dmg, update_to (+55) |
| `libs/libxdo-sys-stub/src/lib.rs` | load, get_lib, xdo_new, xdo_new_with_opened_display, xdo_free (+23) |
| `src/flutter_ffi.rs` | main_set_cursor_position, main_check_connect_status, session_send_mouse, session_read_local_empty_dirs_recursive_sync, main_set_common (+15) |
| `libs/hbb_common/src/platform/linux.rs` | get_values_of_seat0, get_display_server, is_active, is_active_and_seat0, is_session_locked (+15) |
| `libs/clipboard/src/platform/windows.rs` | empty_cliprdr, empty_clipboard, server_clip_file, server_monitor_ready, server_format_list_response (+13) |
| `src/platform/gtk_sudo.rs` | exec, cmd, ui, cmd_parent, ui_parent (+6) |
| `src/platform/linux_desktop_manager.rs` | pam_get_service_name, detect_headless, try_start_desktop, try_start_x_session, is_headless (+6) |
| `src/platform/windows.cc` | GetProcessUserName, GetLogonPid, GetFallbackUserPid, GetSessionUserTokenWin, LaunchProcessWin (+6) |

## Entry Points

Start here when exploring this area:

- **`main_set_cursor_position`** (Function) — `src/flutter_ffi.rs:1282`
- **`get_cursor_pos`** (Function) — `src/platform/linux.rs:186`
- **`set_cursor_pos`** (Function) — `src/platform/linux.rs:209`
- **`xdo_new`** (Function) — `libs/libxdo-sys-stub/src/lib.rs:260`
- **`xdo_new_with_opened_display`** (Function) — `libs/libxdo-sys-stub/src/lib.rs:264`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `main_set_cursor_position` | Function | `src/flutter_ffi.rs` | 1282 |
| `get_cursor_pos` | Function | `src/platform/linux.rs` | 186 |
| `set_cursor_pos` | Function | `src/platform/linux.rs` | 209 |
| `xdo_new` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 260 |
| `xdo_new_with_opened_display` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 264 |
| `xdo_free` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 276 |
| `xdo_send_keysequence_window` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 285 |
| `xdo_send_keysequence_window_down` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 296 |
| `xdo_send_keysequence_window_up` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 307 |
| `xdo_enter_text_window` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 318 |
| `xdo_click_window` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 329 |
| `xdo_mouse_down` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 339 |
| `xdo_mouse_up` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 345 |
| `xdo_move_mouse` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 351 |
| `xdo_move_mouse_relative` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 362 |
| `xdo_move_mouse_relative_to_window` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 368 |
| `xdo_get_mouse_location` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 379 |
| `xdo_get_mouse_location2` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 390 |
| `xdo_get_focused_window` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 411 |
| `xdo_get_focused_window_sane` | Function | `libs/libxdo-sys-stub/src/lib.rs` | 420 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Start_os_service → New` | cross_community | 7 |
| `Start_os_service → Write_all` | cross_community | 7 |
| `Start_os_service → Ipc_path` | cross_community | 6 |
| `Start_os_service → Next` | cross_community | 5 |
| `Core_main → Wide_string` | cross_community | 5 |
| `Core_main → Get_reg_of` | cross_community | 5 |
| `Core_main → Get_default_install_path` | cross_community | 5 |
| `Core_main → Get_subkey` | cross_community | 5 |
| `Start → DelegateState` | intra_community | 4 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Plugin | 18 calls |
| Server | 13 calls |
| Macos | 5 calls |
| Client | 4 calls |
| Win | 4 calls |
| Cluster_66 | 2 calls |
| Cluster_92 | 2 calls |
| Flutter_hbb | 2 calls |

## How to Explore

1. `gitnexus_context({name: "main_set_cursor_position"})` — see callers and callees
2. `gitnexus_query({query: "platform"})` — find related execution flows
3. Read key files listed above for implementation details
