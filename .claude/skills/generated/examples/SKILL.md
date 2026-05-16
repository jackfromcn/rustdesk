---
name: examples
description: "Skill for the Examples area of rustdesk. 27 symbols across 11 files."
---

# Examples

27 symbols | 11 files | Cohesion: 95%

## When to Use

- Working with code in `libs/`
- Understanding how test, to, yuv work
- Modifying examples-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `libs/scrap/examples/benchmark.rs` | main, test_vpx, test_av1, test, test_encoder (+1) |
| `examples/ipc.rs` | main, ipc_server, ipc_client |
| `libs/hbb_common/examples/webrtc.rs` | main, read_loop, write_loop |
| `libs/hbb_common/src/webrtc.rs` | get_local_endpoint, set_remote_endpoint, next |
| `libs/scrap/examples/screenshot.rs` | main, get_display, record |
| `libs/scrap/examples/capture_mag.rs` | main, get_display, record |
| `libs/scrap/src/common/mod.rs` | to, yuv |
| `libs/scrap/examples/record-screen.rs` | main |
| `libs/portable/src/ui.rs` | exit |
| `libs/enigo/src/lib.rs` | key_sequence_parse |

## Entry Points

Start here when exploring this area:

- **`test`** (Function) — `libs/scrap/examples/benchmark.rs:236`
- **`to`** (Function) — `libs/scrap/src/common/mod.rs:189`
- **`yuv`** (Function) — `libs/scrap/src/common/mod.rs:211`
- **`get_local_endpoint`** (Function) — `libs/hbb_common/src/webrtc.rs:380`
- **`set_remote_endpoint`** (Function) — `libs/hbb_common/src/webrtc.rs:391`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `test` | Function | `libs/scrap/examples/benchmark.rs` | 236 |
| `to` | Function | `libs/scrap/src/common/mod.rs` | 189 |
| `yuv` | Function | `libs/scrap/src/common/mod.rs` | 211 |
| `get_local_endpoint` | Function | `libs/hbb_common/src/webrtc.rs` | 380 |
| `set_remote_endpoint` | Function | `libs/hbb_common/src/webrtc.rs` | 391 |
| `next` | Function | `libs/hbb_common/src/webrtc.rs` | 471 |
| `main` | Function | `examples/ipc.rs` | 29 |
| `ipc_server` | Function | `examples/ipc.rs` | 59 |
| `ipc_client` | Function | `examples/ipc.rs` | 72 |
| `main` | Function | `libs/scrap/examples/record-screen.rs` | 54 |
| `main` | Function | `libs/scrap/examples/benchmark.rs` | 40 |
| `test_vpx` | Function | `libs/scrap/examples/benchmark.rs` | 82 |
| `test_av1` | Function | `libs/scrap/examples/benchmark.rs` | 162 |
| `test_encoder` | Function | `libs/scrap/examples/benchmark.rs` | 249 |
| `test_decoder` | Function | `libs/scrap/examples/benchmark.rs` | 313 |
| `exit` | Function | `libs/portable/src/ui.rs` | 31 |
| `main` | Function | `libs/hbb_common/examples/webrtc.rs` | 13 |
| `read_loop` | Function | `libs/hbb_common/examples/webrtc.rs` | 116 |
| `write_loop` | Function | `libs/hbb_common/examples/webrtc.rs` | 136 |
| `main` | Function | `libs/scrap/examples/screenshot.rs` | 10 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Win | 1 calls |

## How to Explore

1. `gitnexus_context({name: "test"})` — see callers and callees
2. `gitnexus_query({query: "examples"})` — find related execution flows
3. Read key files listed above for implementation details
