---
name: plugin
description: "Skill for the Plugin area of rustdesk. 218 symbols across 34 files."
---

# Plugin

218 symbols | 34 files | Cohesion: 74%

## When to Use

- Working with code in `src/`
- Understanding how send_to_cm, stop_main_window_process, connect work
- Modifying plugin-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/ipc.rs` | connect, send, send_config, next_timeout, get_config_async (+33) |
| `src/plugin/manager.rs` | _send_install_status, get_plugin_source_list, get_source_plugins, get_uninstalled_plugins, remove_uninstalled (+19) |
| `src/plugin/plugins.rs` | load_plugins, load_plugin_dir, load_plugin, unload_plugin, mark_uninstalled (+17) |
| `src/plugin/ipc.rs` | get_config, set_config, get_manager_config, set_manager_config, get_manager_plugin_config (+13) |
| `src/flutter_ffi.rs` | main_handle_wayland_screencast_restore_token, plugin_get_shared_option, plugin_is_enabled, plugin_enable, plugin_install (+6) |
| `flutter/lib/plugin/manager.dart` | setInstall, setUninstall, handleEvent, _sortPlugins, _handlePluginList (+6) |
| `src/plugin/mod.rs` | is_server_running, init, get_plugins_dir, get_plugin_dir, get_uninstall_file_path (+5) |
| `src/plugin/config.rs` | add_plugin, cb_get_local_peer_id, get, load_if_not_exists, load_if_not_exists (+5) |
| `flutter/lib/plugin/model.dart` | PluginModel, add, add, clear, clearLocations (+4) |
| `flutter/lib/models/model.dart` | _touchMode, notifyListeners, setShowMyCursor, loadOptions, updateEdgeScrollEdgeThickness (+3) |

## Entry Points

Start here when exploring this area:

- **`send_to_cm`** (Function) — `src/ui_interface.rs:1337`
- **`stop_main_window_process`** (Function) — `src/server.rs:803`
- **`connect`** (Function) — `src/ipc.rs:936`
- **`send`** (Function) — `src/ipc.rs:1083`
- **`next_timeout`** (Function) — `src/ipc.rs:1094`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `PluginModel` | Class | `flutter/lib/plugin/model.dart` | 20 |
| `OptionModel` | Class | `flutter/lib/plugin/model.dart` | 7 |
| `UiType` | Class | `flutter/lib/plugin/manager.dart` | 27 |
| `UiButton` | Class | `flutter/lib/plugin/manager.dart` | 52 |
| `UiCheckbox` | Class | `flutter/lib/plugin/manager.dart` | 68 |
| `NativeHandler` | Class | `flutter/lib/plugin/handlers.dart` | 8 |
| `NativeUiHandler` | Class | `flutter/lib/plugin/handlers.dart` | 17 |
| `NativeHandler` | Class | `flutter/lib/web/plugin/handlers.dart` | 0 |
| `NativeUiHandler` | Class | `flutter/lib/web/plugin/handlers.dart` | 4 |
| `send_to_cm` | Function | `src/ui_interface.rs` | 1337 |
| `stop_main_window_process` | Function | `src/server.rs` | 803 |
| `connect` | Function | `src/ipc.rs` | 936 |
| `send` | Function | `src/ipc.rs` | 1083 |
| `next_timeout` | Function | `src/ipc.rs` | 1094 |
| `set_config_async` | Function | `src/ipc.rs` | 1154 |
| `set_data` | Function | `src/ipc.rs` | 1161 |
| `get_options` | Function | `src/ipc.rs` | 1385 |
| `set_option` | Function | `src/ipc.rs` | 1397 |
| `set_options` | Function | `src/ipc.rs` | 1408 |
| `get_nat_type` | Function | `src/ipc.rs` | 1431 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Start_os_service → New` | cross_community | 7 |
| `Handle_msg_from_peer → Send` | cross_community | 6 |
| `Build → NotifyListeners` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Platform | 7 calls |
| Cluster_119 | 4 calls |
| Server | 3 calls |
| Cluster_228 | 3 calls |
| Cluster_92 | 3 calls |
| Whiteboard | 2 calls |
| Cluster_347 | 2 calls |
| Cluster_338 | 2 calls |

## How to Explore

1. `gitnexus_context({name: "send_to_cm"})` — see callers and callees
2. `gitnexus_query({query: "plugin"})` — find related execution flows
3. Read key files listed above for implementation details
