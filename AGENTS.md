# RustDesk Guide

## Project Layout

### Directory Structure
* `src/` Rust app
* `src/server/` audio / clipboard / input / video / network
* `src/platform/` platform-specific code
* `src/ui/` legacy Sciter UI (deprecated)
* `flutter/` current UI
* `libs/hbb_common/` config / proto / shared utils
* `libs/scrap/` screen capture
* `libs/enigo/` input control
* `libs/clipboard/` clipboard
* `libs/hbb_common/src/config.rs` all options

### Key Components
- **Remote Desktop Protocol**: Custom protocol implemented in `src/rendezvous_mediator.rs` for communicating with rustdesk-server
- **Screen Capture**: Platform-specific screen capture in `libs/scrap/`
- **Input Handling**: Cross-platform input simulation in `libs/enigo/`
- **Audio/Video Services**: Real-time audio/video streaming in `src/server/`
- **File Transfer**: Secure file transfer implementation in `libs/hbb_common/`

### UI Architecture
- **Legacy UI**: Sciter-based (deprecated) - files in `src/ui/`
- **Modern UI**: Flutter-based - files in `flutter/`
  - Desktop: `flutter/lib/desktop/`
  - Mobile: `flutter/lib/mobile/`
  - Shared: `flutter/lib/common/` and `flutter/lib/models/`

## Rust Rules

* Avoid `unwrap()` / `expect()` in production code.
* Exceptions:

  * tests;
  * lock acquisition where failure means poisoning, not normal control flow.
* Otherwise prefer `Result` + `?` or explicit handling.
* Do not ignore errors silently.
* Avoid unnecessary `.clone()`.
* Prefer borrowing when practical.
* Do not add dependencies unless needed.
* Keep code simple and idiomatic.

## Tokio Rules

* Assume a Tokio runtime already exists.
* Never create nested runtimes.
* Never call `Runtime::block_on()` inside Tokio / async code.
* Do not hide runtime creation inside helpers or libraries.
* Do not hold locks across `.await`.
* Prefer `.await`, `tokio::spawn`, channels.
* Use `spawn_blocking` or dedicated threads for blocking work.
* Do not use `std::thread::sleep()` in async code.

## Editing Hygiene

* Change only what is required.
* Prefer the smallest valid diff.
* Do not refactor unrelated code.
* Do not make formatting-only changes.
* Keep naming/style consistent with nearby code.

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
