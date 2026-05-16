---
name: widgets
description: "Skill for the Widgets area of rustdesk. 652 symbols across 65 files."
---

# Widgets

652 symbols | 65 files | Cohesion: 80%

## When to Use

- Working with code in `flutter/`
- Understanding how main, closeConnection, createDialogContent work
- Modifying widgets-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `flutter/lib/common/widgets/dialog.dart` | clientClose, changeIdDialog, changeWhiteList, changeDirectAccessPort, changeAutoDisconnectTimeout (+44) |
| `flutter/lib/common/widgets/peer_card.dart` | _openNewConnInAction, _isForceAlwaysRelay, _removeAction, _getAlias, _editTagAction (+43) |
| `flutter/lib/desktop/widgets/remote_toolbar.dart` | build, build, build, build, build (+42) |
| `flutter/lib/web/bridge.dart` | sessionClose, sessionReconnect, setLocalKbLayoutType, sessionSetTrackpadSpeed, sessionGetCustomImageQuality (+38) |
| `flutter/lib/common.dart` | closeConnection, setOverlayState, dismissAll, show, showLoading (+33) |
| `flutter/lib/desktop/widgets/tabbar_widget.dart` | add, _buildPageView, closeConfirmDialog, jumpToByKey, existingInvisibleTab (+31) |
| `flutter/lib/common/widgets/peer_tab_page.dart` | _createRefresh, _portraitRightActions, RefreshWidget, build, _buildSearchBar (+26) |
| `flutter/lib/mobile/widgets/floating_mouse_widgets.dart` | _resetPosition, didChangeDependencies, initState, didChangeDependencies, _onMoveUpdateDelta (+23) |
| `flutter/lib/mobile/widgets/floating_mouse.dart` | tryCancel, tryStart, dispose, _onMoveUpdateDelta, _handlePointerMove (+22) |
| `flutter/lib/common/widgets/gestures.dart` | _trackTap, _resolve, _reset, _stopSecondTapDownTimer, _trackTap (+15) |

## Entry Points

Start here when exploring this area:

- **`main`** (Function) — `flutter/test/cm_test.dart:19`
- **`closeConnection`** (Function) — `flutter/lib/common.dart:700`
- **`createDialogContent`** (Function) — `flutter/lib/common.dart:1121`
- **`msgBox`** (Function) — `flutter/lib/common.dart:1165`
- **`msgboxIcon`** (Function) — `flutter/lib/common.dart:1272`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `CustomAlertDialog` | Class | `flutter/lib/common.dart` | 1048 |
| `LoginWidgetUserPass` | Class | `flutter/lib/common/widgets/login.dart` | 358 |
| `PasswordStrengthIndicator` | Class | `flutter/lib/common/widgets/custom_password.dart` | 71 |
| `AddressBookTag` | Class | `flutter/lib/common/widgets/address_book.dart` | 751 |
| `RefreshWidget` | Class | `flutter/lib/common/widgets/peer_tab_page.dart` | 938 |
| `ActionIcon` | Class | `flutter/lib/desktop/widgets/tabbar_widget.dart` | 1244 |
| `WindowActionPanel` | Class | `flutter/lib/desktop/widgets/tabbar_widget.dart` | 696 |
| `RemoteCountState` | Class | `flutter/lib/common/shared_state.dart` | 224 |
| `ShowRemoteCursorState` | Class | `flutter/lib/common/shared_state.dart` | 131 |
| `ShowRemoteCursorLockState` | Class | `flutter/lib/common/shared_state.dart` | 154 |
| `PeerBoolOption` | Class | `flutter/lib/common/shared_state.dart` | 247 |
| `MouseBody` | Class | `flutter/lib/mobile/widgets/floating_mouse.dart` | 734 |
| `PopupMenuChildrenItem` | Class | `flutter/lib/desktop/widgets/popup_menu.dart` | 9 |
| `PopupMenuEntry` | Class | `flutter/lib/desktop/widgets/material_mod_popup_menu.dart` | 71 |
| `PopupMenuDivider` | Class | `flutter/lib/desktop/widgets/material_mod_popup_menu.dart` | 109 |
| `BasePeersView` | Class | `flutter/lib/common/widgets/peers_view.dart` | 412 |
| `RecentPeersView` | Class | `flutter/lib/common/widgets/peers_view.dart` | 452 |
| `FavoritePeersView` | Class | `flutter/lib/common/widgets/peers_view.dart` | 472 |
| `DiscoveredPeersView` | Class | `flutter/lib/common/widgets/peers_view.dart` | 492 |
| `AddressBookPeersView` | Class | `flutter/lib/common/widgets/peers_view.dart` | 513 |

## Execution Flows

| Flow | Type | Steps |
|------|------|-------|
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Build → MultiWindowCallResult` | cross_community | 9 |
| `Dispose → _disableNativeRelativeMouseMode` | cross_community | 7 |
| `Network → SessionSetFlutterOption` | cross_community | 7 |
| `Build → JumpTo` | cross_community | 6 |
| `Build → JumpTo` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |
| `InitState → _findWindowsByType` | cross_community | 6 |
| `InitState → SessionSetFlutterOption` | cross_community | 6 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Pages | 48 calls |
| Models | 31 calls |
| Web | 19 calls |
| Plugin | 2 calls |
| Cluster_1295 | 2 calls |
| CustomActions | 1 calls |

## How to Explore

1. `gitnexus_context({name: "main"})` — see callers and callees
2. `gitnexus_query({query: "widgets"})` — find related execution flows
3. Read key files listed above for implementation details
