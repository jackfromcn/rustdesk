# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RustDesk is a remote desktop solution written in Rust. It consists of a Rust backend and a Flutter-based UI (desktop and mobile).

## Build Commands

### Prerequisites
1. Install [vcpkg](https://github.com/microsoft/vcpkg) and set `VCPKG_ROOT`:
   - Windows: `vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static`
   - Linux/macOS: `vcpkg install libvpx libyuv opus aom`
2. Download Sciter dynamic library (for Sciter UI):
   - [Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll)
   - [Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so)
   - [macOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

### Development Commands
```bash
# Run the app (debug)
cargo run

# Run with Flutter UI (default)
cargo run --features flutter

# Run a single test
cargo test --lib <test_name>

# Build release
cargo build --release
```

### Flutter Development
```bash
cd flutter
flutter run
flutter build macos|linux|windows|android|ios
```

### Python Build Script
The project includes `build.py` for packaging builds:
```bash
python3 build.py --help
```

## Architecture

### Rust Backend (`src/`)
- **client.rs**: Peer connection handling (152KB - large file)
- **server.rs**: Server connection management
- **rendezvous_mediator.rs**: Communication with rustdesk-server, TCP hole punching, relay connections
- **server/**: Audio, clipboard, input, video services, terminal services
- **platform/**: Platform-specific code (Windows, macOS, Linux)
- **ui/**: Legacy Sciter UI (deprecated)

### Flutter UI (`flutter/`)
- **lib/desktop/**: Desktop-specific widgets and pages
- **lib/mobile/**: Mobile-specific widgets and pages
- **lib/common/**: Shared components
- **lib/models/**: Data models
- **lib/native/**: Platform bindings via flutter_rust_bridge

### Libraries (`libs/`)
- **hbb_common**: Video codec, config, TCP/UDP wrapper, protobuf, file transfer (git submodule)
- **scrap**: Screen capture
- **enigo**: Cross-platform keyboard/mouse control
- **clipboard**: File copy/paste across platforms

### Key Configuration
- All options are defined in `libs/hbb_common/src/config.rs`

## Important Rules

### Rust Rules
- Avoid `unwrap()` / `expect()` in production code
- Prefer `Result` + `?` or explicit error handling
- Avoid unnecessary `.clone()` - prefer borrowing when practical

### Tokio Rules
- Assume a Tokio runtime already exists
- Never create nested runtimes
- Never call `Runtime::block_on()` inside async code
- Do not hold locks across `.await`
- Use `spawn_blocking` for blocking work
- Do not use `std::thread::sleep()` in async code

### Editing Hygiene
- Change only what is required
- Prefer the smallest valid diff
- Do not refactor unrelated code
- Keep naming/style consistent with nearby code

## Notes

- The legacy Sciter UI in `src/ui/` is deprecated; use Flutter
- `libs/hbb_common` is a git submodule at https://github.com/rustdesk/hbb_common
- The project supports Windows, macOS, Linux, Android, and iOS
- Communication happens via custom protocol with rustdesk-server for NAT traversal

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **rustdesk** (19702 symbols, 41502 relationships, 300 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/rustdesk/context` | Codebase overview, check index freshness |
| `gitnexus://repo/rustdesk/clusters` | All functional areas |
| `gitnexus://repo/rustdesk/processes` | All execution flows |
| `gitnexus://repo/rustdesk/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |
| Work in the Widgets area (652 symbols) | `.claude/skills/generated/widgets/SKILL.md` |
| Work in the Platform area (533 symbols) | `.claude/skills/generated/platform/SKILL.md` |
| Work in the Server area (430 symbols) | `.claude/skills/generated/server/SKILL.md` |
| Work in the Models area (394 symbols) | `.claude/skills/generated/models/SKILL.md` |
| Work in the Pages area (383 symbols) | `.claude/skills/generated/pages/SKILL.md` |
| Work in the Web area (229 symbols) | `.claude/skills/generated/web/SKILL.md` |
| Work in the Client area (218 symbols) | `.claude/skills/generated/client/SKILL.md` |
| Work in the Plugin area (218 symbols) | `.claude/skills/generated/plugin/SKILL.md` |
| Work in the Flutter_hbb area (121 symbols) | `.claude/skills/generated/flutter-hbb/SKILL.md` |
| Work in the Res area (114 symbols) | `.claude/skills/generated/res/SKILL.md` |
| Work in the CustomActions area (77 symbols) | `.claude/skills/generated/customactions/SKILL.md` |
| Work in the Hbbs_http area (56 symbols) | `.claude/skills/generated/hbbs-http/SKILL.md` |
| Work in the Cluster_68 area (51 symbols) | `.claude/skills/generated/cluster-68/SKILL.md` |
| Work in the Windows area (49 symbols) | `.claude/skills/generated/windows/SKILL.md` |
| Work in the Wayland area (40 symbols) | `.claude/skills/generated/wayland/SKILL.md` |
| Work in the Whiteboard area (31 symbols) | `.claude/skills/generated/whiteboard/SKILL.md` |
| Work in the Android area (29 symbols) | `.claude/skills/generated/android/SKILL.md` |
| Work in the Examples area (27 symbols) | `.claude/skills/generated/examples/SKILL.md` |
| Work in the Cluster_228 area (25 symbols) | `.claude/skills/generated/cluster-228/SKILL.md` |
| Work in the Unix area (25 symbols) | `.claude/skills/generated/unix/SKILL.md` |

<!-- gitnexus:end -->
