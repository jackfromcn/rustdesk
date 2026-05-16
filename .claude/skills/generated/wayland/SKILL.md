---
name: wayland
description: "Skill for the Wayland area of rustdesk. 40 symbols across 4 files."
---

# Wayland

40 symbols | 4 files | Cohesion: 85%

## When to Use

- Working with code in `libs/`
- Understanding how close_session, try_close_session, new work
- Modifying wayland-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `libs/scrap/src/wayland/pipewire.rs` | close_session, try_close_session, new, get_res, new (+22) |
| `libs/scrap/src/wayland/display.rs` | clear_wayland_displays_cache, try_xrandr_primary, get_primary_monitor, get_displays, get_desktop_rect_for_uinput (+3) |
| `libs/hbb_common/src/platform/linux.rs` | new, get_wayland_displays, is_kde_session |
| `src/server/rdp_input.rs` | send_keysym, handle_mouse |

## Entry Points

Start here when exploring this area:

- **`close_session`** (Function) — `libs/scrap/src/wayland/pipewire.rs:70`
- **`try_close_session`** (Function) — `libs/scrap/src/wayland/pipewire.rs:81`
- **`new`** (Function) — `libs/scrap/src/wayland/pipewire.rs:266`
- **`get_portal`** (Function) — `libs/scrap/src/wayland/pipewire.rs:500`
- **`get_available_cursor_modes`** (Function) — `libs/scrap/src/wayland/pipewire.rs:592`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `close_session` | Function | `libs/scrap/src/wayland/pipewire.rs` | 70 |
| `try_close_session` | Function | `libs/scrap/src/wayland/pipewire.rs` | 81 |
| `new` | Function | `libs/scrap/src/wayland/pipewire.rs` | 266 |
| `get_portal` | Function | `libs/scrap/src/wayland/pipewire.rs` | 500 |
| `get_available_cursor_modes` | Function | `libs/scrap/src/wayland/pipewire.rs` | 592 |
| `request_remote_desktop` | Function | `libs/scrap/src/wayland/pipewire.rs` | 599 |
| `get_capturables` | Function | `libs/scrap/src/wayland/pipewire.rs` | 927 |
| `is_server_running` | Function | `libs/scrap/src/wayland/pipewire.rs` | 978 |
| `clear_wayland_displays_cache` | Function | `libs/scrap/src/wayland/display.rs` | 212 |
| `fill_displays` | Function | `libs/scrap/src/wayland/pipewire.rs` | 1057 |
| `get_displays` | Function | `libs/scrap/src/wayland/display.rs` | 170 |
| `get_desktop_rect_for_uinput` | Function | `libs/scrap/src/wayland/display.rs` | 217 |
| `get_wayland_displays` | Function | `libs/hbb_common/src/platform/linux.rs` | 379 |
| `is_kde_session` | Function | `libs/hbb_common/src/platform/linux.rs` | 105 |
| `send_keysym` | Function | `src/server/rdp_input.rs` | 423 |
| `handle_mouse` | Function | `src/server/rdp_input.rs` | 576 |
| `new` | Function | `libs/scrap/src/wayland/pipewire.rs` | 161 |
| `get_res` | Function | `libs/scrap/src/wayland/pipewire.rs` | 230 |
| `handle_response` | Function | `libs/scrap/src/wayland/pipewire.rs` | 456 |
| `streams_from_response` | Function | `libs/scrap/src/wayland/pipewire.rs` | 508 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Run → WaylandEnv` | cross_community | 7 |
| `Run → New` | cross_community | 7 |
| `Run → WaylandDisplayInfo` | cross_community | 7 |
| `Run → Try_xrandr_primary` | cross_community | 7 |
| `Run → Displays_to_key` | cross_community | 7 |
| `Run → PipewireDisplayOffsetCache` | cross_community | 7 |
| `Run → Displays` | cross_community | 6 |
| `Run → New` | cross_community | 6 |
| `Run → Sort_streams` | cross_community | 5 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Web | 1 calls |

## How to Explore

1. `gitnexus_context({name: "close_session"})` — see callers and callees
2. `gitnexus_query({query: "wayland"})` — find related execution flows
3. Read key files listed above for implementation details
