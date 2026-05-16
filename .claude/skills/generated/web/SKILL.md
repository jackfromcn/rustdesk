---
name: web
description: "Skill for the Web area of rustdesk. 229 symbols across 36 files."
---

# Web

229 symbols | 36 files | Cohesion: 74%

## When to Use

- Working with code in `flutter/`
- Understanding how main, initEnv, runMainApp work
- Modifying web-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `flutter/lib/web/bridge.dart` | mainGetAppNameSync, mainCheckConnectStatus, mainGetSoftwareUpdateUrl, cmGetConfig, mainHideDock (+86) |
| `flutter/lib/common.dart` | initUniLinks, getWindowName, setResizable, disableWindowMovable, earlyAssert (+31) |
| `flutter/lib/main.dart` | main, initEnv, runMainApp, runMobileApp, runMultiWindow (+10) |
| `flutter/lib/utils/multi_window_manager.dart` | clearWindowType, closeAllSubWindows, _closeWindows, newSession, newRemoteDesktop (+6) |
| `flutter/lib/models/model.dart` | mobileReset, start, getSize, setViewOnly, updateGetKey (+3) |
| `flutter/lib/desktop/pages/desktop_setting_page.dart` | build, pluginCard, language, build, service (+1) |
| `flutter/lib/web/custom_cursor.dart` | registerCursor, FlutterCustomMemoryImageCursor, buildCursorOfCache, setSystemCursor, activate |
| `flutter/lib/models/file_model.dart` | fetchDirectory, loadLastJob, JobProgress, registerReadEmptyDirsTask, readEmptyDirs |
| `flutter/lib/models/user_model.dart` | refreshCurrentUser, getLocalUserInfo, reset, logOut |
| `flutter/lib/models/native_model.dart` | getVersion, init, tryHandle, _startListenEvent |

## Entry Points

Start here when exploring this area:

- **`main`** (Function) — `flutter/lib/main.dart:38`
- **`initEnv`** (Function) — `flutter/lib/main.dart:113`
- **`runMainApp`** (Function) — `flutter/lib/main.dart:126`
- **`runMobileApp`** (Function) — `flutter/lib/main.dart:171`
- **`runMultiWindow`** (Function) — `flutter/lib/main.dart:183`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `BlockInputState` | Class | `flutter/lib/common/shared_state.dart` | 7 |
| `CkbMenuButton` | Class | `flutter/lib/desktop/widgets/remote_toolbar.dart` | 2330 |
| `FlutterCustomMemoryImageCursor` | Class | `flutter/lib/web/custom_cursor.dart` | 67 |
| `MenuEntryDivider` | Class | `flutter/lib/desktop/widgets/popup_menu.dart` | 133 |
| `EventToUI` | Class | `flutter/lib/web/bridge.dart` | 17 |
| `EventToUI_Event` | Class | `flutter/lib/web/bridge.dart` | 30 |
| `EventToUI_Rgba` | Class | `flutter/lib/web/bridge.dart` | 36 |
| `EventToUI_Texture` | Class | `flutter/lib/web/bridge.dart` | 42 |
| `JobProgress` | Class | `flutter/lib/models/file_model.dart` | 1468 |
| `ButtonOP` | Class | `flutter/lib/common/widgets/login.dart` | 56 |
| `main` | Function | `flutter/lib/main.dart` | 38 |
| `initEnv` | Function | `flutter/lib/main.dart` | 113 |
| `runMainApp` | Function | `flutter/lib/main.dart` | 126 |
| `runMobileApp` | Function | `flutter/lib/main.dart` | 171 |
| `runMultiWindow` | Function | `flutter/lib/main.dart` | 183 |
| `runConnectionManagerScreen` | Function | `flutter/lib/main.dart` | 261 |
| `showCmWindow` | Function | `flutter/lib/main.dart` | 282 |
| `hideCmWindow` | Function | `flutter/lib/main.dart` | 309 |
| `runInstallPage` | Function | `flutter/lib/main.dart` | 363 |
| `getHiddenTitleBarWindowOptions` | Function | `flutter/lib/main.dart` | 377 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Network → SessionSetFlutterOption` | cross_community | 7 |
| `Build → Remove` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `Build → SessionReadLocalEmptyDirsRecursiveSync` | cross_community | 6 |
| `Build → SessionReadRemoteEmptyDirsRecursiveSync` | cross_community | 6 |
| `Build → ResetModifiers` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Widgets | 46 calls |
| Pages | 30 calls |
| Models | 20 calls |
| Cluster_228 | 3 calls |
| Examples | 2 calls |
| Plugin | 2 calls |

## How to Explore

1. `gitnexus_context({name: "main"})` — see callers and callees
2. `gitnexus_query({query: "web"})` — find related execution flows
3. Read key files listed above for implementation details
