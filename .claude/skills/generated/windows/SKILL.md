---
name: windows
description: "Skill for the Windows area of rustdesk. 49 symbols across 1 files."
---

# Windows

49 symbols | 1 files | Cohesion: 77%

## When to Use

- Working with code in `libs/`
- Understanding how wf_do_empty_cliprdr, try_reset_event, wait_response_event work
- Modifying windows-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `libs/clipboard/src/windows/wf_cliprdr.c` | wf_do_empty_cliprdr, get_remote_format_id, cliprdr_send_data_request, cliprdr_send_request_filecontents, CliprdrStream_Read (+44) |

## Entry Points

Start here when exploring this area:

- **`wf_do_empty_cliprdr`** (Function) — `libs/clipboard/src/windows/wf_cliprdr.c:259`
- **`try_reset_event`** (Function) — `libs/clipboard/src/windows/wf_cliprdr.c:1520`
- **`wait_response_event`** (Function) — `libs/clipboard/src/windows/wf_cliprdr.c:1549`
- **`empty_cliprdr`** (Function) — `libs/clipboard/src/windows/wf_cliprdr.c:3297`
- **`wf_cliprdr_init`** (Function) — `libs/clipboard/src/windows/wf_cliprdr.c:257`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `wf_do_empty_cliprdr` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 259 |
| `try_reset_event` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 1520 |
| `wait_response_event` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 1549 |
| `empty_cliprdr` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 3297 |
| `wf_cliprdr_init` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 257 |
| `wf_cliprdr_uninit` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 258 |
| `init_cliprdr` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 3287 |
| `uninit_cliprdr` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 3292 |
| `get_remote_format_id` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 263 |
| `cliprdr_send_data_request` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 264 |
| `cliprdr_send_request_filecontents` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 267 |
| `CliprdrStream_Read` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 347 |
| `CliprdrStream_New` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 538 |
| `cliprdr_lookup_format` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 635 |
| `CliprdrDataObject_GetData` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 705 |
| `CliprdrDataObject_QueryGetData` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 827 |
| `clear_file_array` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 1993 |
| `wf_cliprdr_server_format_data_request` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 2556 |
| `wf_create_file_obj` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 261 |
| `CliprdrDataObject_Delete` | Function | `libs/clipboard/src/windows/wf_cliprdr.c` | 274 |

## Connected Areas

| Area | Connections |
|------|-------------|
| CustomActions | 7 calls |

## How to Explore

1. `gitnexus_context({name: "wf_do_empty_cliprdr"})` — see callers and callees
2. `gitnexus_query({query: "windows"})` — find related execution flows
3. Read key files listed above for implementation details
