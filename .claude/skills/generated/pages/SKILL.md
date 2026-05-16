---
name: pages
description: "Skill for the Pages area of rustdesk. 383 symbols across 60 files."
---

# Pages

383 symbols | 60 files | Cohesion: 74%

## When to Use

- Working with code in `flutter/`
- Understanding how isOptionFixed, customImageQualitySetting, otherDefaultSettings work
- Modifying pages-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `flutter/lib/desktop/pages/desktop_setting_page.dart` | build, theme, build, tfa, permissions (+39) |
| `flutter/lib/common.dart` | isOptionFixed, windowOnTop, reloadCurrentWindow, initGlobalFFI, unreadMessageCountBuilder (+26) |
| `flutter/lib/common/shared_state.dart` | init, init, ConnectionType, init, init (+26) |
| `flutter/lib/web/bridge.dart` | mainSetUserDefaultOption, mainGetUserDefaultOption, sessionToggleOption, sessionCloseTerminal, sessionInputString (+21) |
| `flutter/lib/mobile/pages/remote_page.dart` | _handleIOSSoftKeyboardInput, _handleNonIOSSoftKeyboardInput, handleSoftKeyboardInput, inputChar, openKeyboard (+18) |
| `flutter/lib/desktop/pages/server_page.dart` | build, _buildClientAvatar, _buildInitialAvatar, buildPermissionIcon, build (+14) |
| `flutter/lib/desktop/pages/file_manager_page.dart` | willPopScope, build, dropArea, statusList, handleDragDone (+12) |
| `flutter/lib/desktop/pages/terminal_tab_page.dart` | _closeAllTabs, initState, windowId, handleWindowCloseButton, _closeTab (+10) |
| `flutter/lib/mobile/pages/server_page.dart` | build, _buildDisconnectButton, build, build, checkService (+9) |
| `flutter/lib/desktop/pages/remote_page.dart` | _buildScrollAutoNonTextureRender, _BuildPaintTextureRender, _buildRawTouchAndPointerRegion, _buildRawPointerMouseRegion, getBodyForDesktop (+9) |

## Entry Points

Start here when exploring this area:

- **`isOptionFixed`** (Function) — `flutter/lib/common.dart:3788`
- **`customImageQualitySetting`** (Function) — `flutter/lib/common/widgets/setting_widgets.dart:149`
- **`otherDefaultSettings`** (Function) — `flutter/lib/common/widgets/setting_widgets.dart:215`
- **`windowOnTop`** (Function) — `flutter/lib/common.dart:720`
- **`reloadCurrentWindow`** (Function) — `flutter/lib/common.dart:2698`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `EdgeThicknessControl` | Class | `flutter/lib/desktop/widgets/remote_toolbar.dart` | 2630 |
| `AddButton` | Class | `flutter/lib/desktop/widgets/tabbar_widget.dart` | 1307 |
| `ConnectionType` | Class | `flutter/lib/common/shared_state.dart` | 53 |
| `TerminalConnectionManager` | Class | `flutter/lib/desktop/pages/terminal_connection_manager.dart` | 5 |
| `MenuButton` | Class | `flutter/lib/desktop/widgets/menu_button.dart` | 2 |
| `ConnectionTypeState` | Class | `flutter/lib/common/shared_state.dart` | 86 |
| `FingerprintState` | Class | `flutter/lib/common/shared_state.dart` | 108 |
| `OnlineStatusWidget` | Class | `flutter/lib/desktop/pages/connection_page.dart` | 23 |
| `CurrentDisplayState` | Class | `flutter/lib/common/shared_state.dart` | 30 |
| `PermissionRow` | Class | `flutter/lib/mobile/pages/server_page.dart` | 650 |
| `PaddingCard` | Class | `flutter/lib/mobile/pages/server_page.dart` | 809 |
| `SettingsPage` | Class | `flutter/lib/mobile/pages/settings_page.dart` | 23 |
| `ServerPage` | Class | `flutter/lib/mobile/pages/server_page.dart` | 17 |
| `PageShape` | Class | `flutter/lib/mobile/pages/home_page.dart` | 11 |
| `ConnectionPage` | Class | `flutter/lib/mobile/pages/connection_page.dart` | 21 |
| `ChatPage` | Class | `flutter/lib/common/widgets/chat_page.dart` | 14 |
| `PopupMenuItem` | Class | `flutter/lib/desktop/widgets/material_mod_popup_menu.dart` | 225 |
| `CheckedPopupMenuItem` | Class | `flutter/lib/desktop/widgets/material_mod_popup_menu.dart` | 460 |
| `ListSearchActionListener` | Class | `flutter/lib/desktop/widgets/list_search_action_listener.dart` | 2 |
| `AutocompletePeerTile` | Class | `flutter/lib/common/widgets/autocomplete.dart` | 107 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Build → Remove` | cross_community | 6 |
| `InitState → HttpService` | cross_community | 6 |
| `Build → JumpTo` | cross_community | 6 |
| `Build → JumpTo` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → _findWindowsByType` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → _findWindowsByType` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Widgets | 85 calls |
| Models | 32 calls |
| Web | 15 calls |
| Cluster_228 | 3 calls |
| Cluster_1295 | 2 calls |
| Plugin | 1 calls |
| CustomActions | 1 calls |

## How to Explore

1. `gitnexus_context({name: "isOptionFixed"})` — see callers and callees
2. `gitnexus_query({query: "pages"})` — find related execution flows
3. Read key files listed above for implementation details
