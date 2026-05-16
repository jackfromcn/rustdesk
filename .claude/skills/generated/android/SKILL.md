---
name: android
description: "Skill for the Android area of rustdesk. 29 symbols across 10 files."
---

# Android

29 symbols | 10 files | Cohesion: 77%

## When to Use

- Working with code in `libs/`
- Understanding how session_toggle_option, update_text_clipboard_required, update_file_clipboard_required work
- Modifying android-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `libs/scrap/src/android/ffi.rs` | _call_clipboard_manager, call_clipboard_manager_enable_client_clipboard, set_enable, Java_ffi_FFI_setFrameRawEnable, call_clipboard_manager_update_clipboard (+9) |
| `src/clipboard.rs` | from_clipboard, handle_msg_clipboard, handle_msg_multi_clipboards |
| `src/flutter_ffi.rs` | session_toggle_option, main_show_option |
| `src/flutter.rs` | update_text_clipboard_required, update_file_clipboard_required |
| `src/client.rs` | set_is_text_clipboard_required, set_is_file_clipboard_required |
| `libs/scrap/src/common/android.rs` | get_size, is_start |
| `src/common_old_backup.rs` | is_rustdesk |
| `src/common.rs` | is_rustdesk |
| `flutter/lib/mobile/pages/file_manager_page.dart` | eq |
| `libs/hbb_common/src/compress.rs` | decompress |

## Entry Points

Start here when exploring this area:

- **`session_toggle_option`** (Function) — `src/flutter_ffi.rs:321`
- **`update_text_clipboard_required`** (Function) — `src/flutter.rs:1432`
- **`update_file_clipboard_required`** (Function) — `src/flutter.rs:1442`
- **`set_is_text_clipboard_required`** (Function) — `src/client.rs:929`
- **`set_is_file_clipboard_required`** (Function) — `src/client.rs:935`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `session_toggle_option` | Function | `src/flutter_ffi.rs` | 321 |
| `update_text_clipboard_required` | Function | `src/flutter.rs` | 1432 |
| `update_file_clipboard_required` | Function | `src/flutter.rs` | 1442 |
| `set_is_text_clipboard_required` | Function | `src/client.rs` | 929 |
| `set_is_file_clipboard_required` | Function | `src/client.rs` | 935 |
| `call_clipboard_manager_enable_client_clipboard` | Function | `libs/scrap/src/android/ffi.rs` | 374 |
| `main_show_option` | Function | `src/flutter_ffi.rs` | 965 |
| `is_rustdesk` | Function | `src/common_old_backup.rs` | 948 |
| `is_rustdesk` | Function | `src/common.rs` | 1008 |
| `Java_ffi_FFI_setFrameRawEnable` | Function | `libs/scrap/src/android/ffi.rs` | 172 |
| `handle_msg_clipboard` | Function | `src/clipboard.rs` | 704 |
| `handle_msg_multi_clipboards` | Function | `src/clipboard.rs` | 720 |
| `decompress` | Function | `libs/hbb_common/src/compress.rs` | 31 |
| `call_clipboard_manager_update_clipboard` | Function | `libs/scrap/src/android/ffi.rs` | 354 |
| `is_start` | Function | `libs/scrap/src/common/android.rs` | 185 |
| `call_main_service_get_by_name` | Function | `libs/scrap/src/android/ffi.rs` | 382 |
| `Java_ffi_FFI_onVideoFrameUpdate` | Function | `libs/scrap/src/android/ffi.rs` | 123 |
| `Java_ffi_FFI_onAudioFrameUpdate` | Function | `libs/scrap/src/android/ffi.rs` | 137 |
| `Java_ffi_FFI_init` | Function | `libs/scrap/src/android/ffi.rs` | 191 |
| `JNI_OnLoad` | Function | `libs/scrap/src/android/ffi.rs` | 484 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Handle_msg_from_peer → Eq` | cross_community | 5 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Cluster_297 | 2 calls |
| Client | 1 calls |
| Cluster_109 | 1 calls |

## How to Explore

1. `gitnexus_context({name: "session_toggle_option"})` — see callers and callees
2. `gitnexus_query({query: "android"})` — find related execution flows
3. Read key files listed above for implementation details
