---
name: cluster-68
description: "Skill for the Cluster_68 area of rustdesk. 51 symbols across 4 files."
---

# Cluster_68

51 symbols | 4 files | Cohesion: 75%

## When to Use

- Working with code in `src/`
- Understanding how toggle_option, toggle_privacy_mode, toggle_virtual_display work
- Modifying cluster_68-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/ui_session_interface.rs` | toggle_option, toggle_privacy_mode, toggle_virtual_display, record_screen, take_screenshot (+26) |
| `src/flutter_ffi.rs` | session_login, session_take_screenshot, session_record_screen, session_toggle_privacy_mode, session_set_custom_fps (+13) |
| `src/flutter.rs` | session_update_virtual_display |
| `src/client.rs` | refresh |

## Entry Points

Start here when exploring this area:

- **`toggle_option`** (Function) — `src/ui_session_interface.rs:376`
- **`toggle_privacy_mode`** (Function) — `src/ui_session_interface.rs:387`
- **`toggle_virtual_display`** (Function) — `src/ui_session_interface.rs:433`
- **`record_screen`** (Function) — `src/ui_session_interface.rs:450`
- **`take_screenshot`** (Function) — `src/ui_session_interface.rs:458`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `toggle_option` | Function | `src/ui_session_interface.rs` | 376 |
| `toggle_privacy_mode` | Function | `src/ui_session_interface.rs` | 387 |
| `toggle_virtual_display` | Function | `src/ui_session_interface.rs` | 433 |
| `record_screen` | Function | `src/ui_session_interface.rs` | 450 |
| `take_screenshot` | Function | `src/ui_session_interface.rs` | 458 |
| `save_custom_image_quality` | Function | `src/ui_session_interface.rs` | 466 |
| `save_image_quality` | Function | `src/ui_session_interface.rs` | 475 |
| `set_custom_fps` | Function | `src/ui_session_interface.rs` | 494 |
| `set_write_override` | Function | `src/ui_session_interface.rs` | 504 |
| `update_supported_decodings` | Function | `src/ui_session_interface.rs` | 543 |
| `use_texture_render_changed` | Function | `src/ui_session_interface.rs` | 548 |
| `restart_remote_device` | Function | `src/ui_session_interface.rs` | 554 |
| `send_chat` | Function | `src/ui_session_interface.rs` | 774 |
| `open_terminal` | Function | `src/ui_session_interface.rs` | 786 |
| `send_terminal_input` | Function | `src/ui_session_interface.rs` | 799 |
| `resize_terminal` | Function | `src/ui_session_interface.rs` | 811 |
| `close_terminal` | Function | `src/ui_session_interface.rs` | 824 |
| `capture_displays` | Function | `src/ui_session_interface.rs` | 835 |
| `switch_display` | Function | `src/ui_session_interface.rs` | 848 |
| `input_string` | Function | `src/ui_session_interface.rs` | 904 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Client | 18 calls |
| Plugin | 1 calls |
| Cluster_59 | 1 calls |
| Cluster_95 | 1 calls |

## How to Explore

1. `gitnexus_context({name: "toggle_option"})` — see callers and callees
2. `gitnexus_query({query: "cluster_68"})` — find related execution flows
3. Read key files listed above for implementation details
