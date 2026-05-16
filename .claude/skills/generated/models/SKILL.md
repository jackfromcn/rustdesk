---
name: models
description: "Skill for the Models area of rustdesk. 394 symbols across 38 files."
---

# Models

394 symbols | 38 files | Cohesion: 74%

## When to Use

- Working with code in `flutter/`
- Understanding how getHttpHeaders, get, post work
- Modifying models-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `flutter/lib/models/ab_model.dart` | pullAbImpl, pullAbImpl, _fetchPeers, _fetchTags, addPeers (+61) |
| `flutter/lib/models/input_model.dart` | toggleRelativeMouseMode, onPointHoverImage, onPointDownImage, onPointUpImage, onPointMoveImage (+48) |
| `flutter/lib/models/model.dart` | startEventListener, updateLastCursorId, handleCursorId, handlePlatformAdditions, updateBlockInputState (+41) |
| `flutter/lib/models/file_model.dart` | receiveFileDir, receiveEmptyDirs, postOverrideFileConfirm, toString, refreshAll (+41) |
| `flutter/lib/web/bridge.dart` | mainGetApiServer, mainGetProxyStatus, mainHttpRequest, mainGetHttpStatus, sessionSwitchSides (+31) |
| `flutter/lib/models/relative_mouse_model.dart` | initHostChannel, _onNativeMouseDelta, _enableNativeRelativeMouseMode, _disableNativeRelativeMouseMode, _handleExitShortcut (+27) |
| `flutter/lib/models/server_model.dart` | setShowElevation, updateVoiceCallState, startService, stopService, addConnection (+9) |
| `flutter/lib/models/terminal_model.dart` | getTerminalIdFromEvt, getSuccessFromEvt, handleTerminalResponse, _handleTerminalOpened, _handleTerminalData (+8) |
| `flutter/lib/common.dart` | getHttpHeaders, dismissByTag, disable, mouseButtonsToPeer, build (+6) |
| `flutter/lib/utils/http_service.dart` | HttpService, sendRequest, _pollFlutterHttp, _pollForResponse, _parseHttpResponse (+3) |

## Entry Points

Start here when exploring this area:

- **`getHttpHeaders`** (Function) — `flutter/lib/common.dart:2633`
- **`get`** (Function) — `flutter/lib/utils/http_service.dart:105`
- **`post`** (Function) — `flutter/lib/utils/http_service.dart:109`
- **`delete`** (Function) — `flutter/lib/utils/http_service.dart:121`
- **`addLocationUi`** (Function) — `flutter/lib/plugin/model.dart:72`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `HttpService` | Class | `flutter/lib/utils/http_service.dart` | 10 |
| `GestureInfo` | Class | `flutter/lib/mobile/widgets/gesture_help.dart` | 352 |
| `PathUtil` | Class | `flutter/lib/models/file_model.dart` | 1595 |
| `RelativeMouseDelta` | Class | `flutter/lib/utils/relative_mouse_accumulator.dart` | 5 |
| `TerminalPage` | Class | `flutter/lib/mobile/pages/terminal_page.dart` | 14 |
| `BaseAb` | Class | `flutter/lib/models/ab_model.dart` | 867 |
| `LegacyAb` | Class | `flutter/lib/models/ab_model.dart` | 971 |
| `Ab` | Class | `flutter/lib/models/ab_model.dart` | 1359 |
| `DummyAb` | Class | `flutter/lib/models/ab_model.dart` | 1881 |
| `BaseEvent` | Class | `flutter/lib/utils/event_loop.dart` | 6 |
| `BaseEventLoop` | Class | `flutter/lib/utils/event_loop.dart` | 27 |
| `FileDialogEventLoop` | Class | `flutter/lib/models/file_model.dart` | 1819 |
| `getHttpHeaders` | Function | `flutter/lib/common.dart` | 2633 |
| `get` | Function | `flutter/lib/utils/http_service.dart` | 105 |
| `post` | Function | `flutter/lib/utils/http_service.dart` | 109 |
| `delete` | Function | `flutter/lib/utils/http_service.dart` | 121 |
| `addLocationUi` | Function | `flutter/lib/plugin/model.dart` | 72 |
| `handlePluginEvent` | Function | `flutter/lib/plugin/event.dart` | 3 |
| `updateLastCursorId` | Function | `flutter/lib/models/model.dart` | 1356 |
| `handleCursorId` | Function | `flutter/lib/models/model.dart` | 1362 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Dispose → _disableNativeRelativeMouseMode` | cross_community | 7 |
| `Build → Remove` | cross_community | 6 |
| `InitState → HttpService` | cross_community | 6 |
| `Build → JumpTo` | cross_community | 6 |
| `Build → _findWindowsByType` | cross_community | 6 |
| `Build → Add` | cross_community | 6 |
| `Build → _findWindowsByType` | cross_community | 6 |
| `Dispose → MainClipCursor` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Widgets | 37 calls |
| Pages | 27 calls |
| Plugin | 20 calls |
| Web | 10 calls |

## How to Explore

1. `gitnexus_context({name: "getHttpHeaders"})` — see callers and callees
2. `gitnexus_query({query: "models"})` — find related execution flows
3. Read key files listed above for implementation details
