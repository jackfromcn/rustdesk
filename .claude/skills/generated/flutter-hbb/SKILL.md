---
name: flutter-hbb
description: "Skill for the Flutter_hbb area of rustdesk. 121 symbols across 18 files."
---

# Flutter_hbb

121 symbols | 18 files | Cohesion: 78%

## When to Use

- Working with code in `flutter/`
- Understanding how build, isSupportVoiceCall, startAction work
- Modifying flutter_hbb-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | onDestroy, updateScreenInfo, onConfigurationChanged, createSurface, startCapture (+23) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/InputService.kt` | onMouseInput, onTouchInput, consumeWheelActions, performClick, longPress (+14) |
| `flutter/android/app/src/main/kotlin/ffi.kt` | onVideoFrameUpdate, onAudioFrameUpdate, refreshScreen, setFrameRawEnable, getBuildinOption (+8) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/FloatingWindowService.kt` | performClick, onTouch, showPopupMenu, openMainActivity, syncClipboard (+7) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/AudioRecordHandle.kt` | checkAudioReader, startAudioRecorder, onVoiceCallStarted, createAudioRecorder, switchToVoiceCall (+4) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainActivity.kt` | onStop, requestMediaProjection, initFlutterChannel, onVoiceCallStarted, configureFlutterEngine (+3) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/VolumeController.kt` | getVolume, setVolume, raiseVolume, lowerVolume, getMute (+2) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | AudioReader, next, readSync, isSupportVoiceCall, startAction (+1) |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/RdClipboardManager.kt` | checkPrimaryClip, isSupportedMimeType, isClipboardDataEqual, syncClipboard, RdClipboardManager |
| `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/KeyboardKeyEventMapper.kt` | toAndroidKeyEvent, convertModifier, convertUnicodeToKeyCode, convertControlKeyToKeyCode |

## Entry Points

Start here when exploring this area:

- **`build`** (Function) — `libs/scrap/src/quartz/config.rs:22`
- **`isSupportVoiceCall`** (Function) — `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt:65`
- **`startAction`** (Function) — `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt:85`
- **`contains`** (Function) — `src/server.rs:410`
- **`getScreenSize`** (Function) — `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt:136`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `AudioReader` | Class | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | 99 |
| `RdClipboardManager` | Class | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/RdClipboardManager.kt` | 18 |
| `build` | Function | `libs/scrap/src/quartz/config.rs` | 22 |
| `isSupportVoiceCall` | Function | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | 65 |
| `startAction` | Function | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | 85 |
| `contains` | Function | `src/server.rs` | 410 |
| `getScreenSize` | Function | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | 136 |
| `onVideoFrameUpdate` | Method | `flutter/android/app/src/main/kotlin/ffi.kt` | 19 |
| `onAudioFrameUpdate` | Method | `flutter/android/app/src/main/kotlin/ffi.kt` | 20 |
| `refreshScreen` | Method | `flutter/android/app/src/main/kotlin/ffi.kt` | 22 |
| `setFrameRawEnable` | Method | `flutter/android/app/src/main/kotlin/ffi.kt` | 23 |
| `readSync` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/common.kt` | 122 |
| `onDestroy` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 250 |
| `onConfigurationChanged` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 351 |
| `startCapture` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 405 |
| `stopCapture` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 439 |
| `destroy` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 476 |
| `checkMediaPermission` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 495 |
| `startAudioRecorder` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/AudioRecordHandle.kt` | 94 |
| `rustSetByName` | Method | `flutter/android/app/src/main/kotlin/com/carriez/flutter_hbb/MainService.kt` | 112 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Pages | 2 calls |
| Platform | 1 calls |
| Models | 1 calls |

## How to Explore

1. `gitnexus_context({name: "build"})` — see callers and callees
2. `gitnexus_query({query: "flutter_hbb"})` — find related execution flows
3. Read key files listed above for implementation details
