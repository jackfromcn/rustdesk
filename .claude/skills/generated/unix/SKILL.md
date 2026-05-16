---
name: unix
description: "Skill for the Unix area of rustdesk. 25 symbols across 6 files."
---

# Unix

25 symbols | 6 files | Cohesion: 90%

## When to Use

- Working with code in `libs/`
- Understanding how clip_2_msg, serve_clip_messages, read_file_contents work
- Modifying unix-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `libs/clipboard/src/platform/unix/serv_files.rs` | get_files_for_audit, serve_file_contents, read_file_contents, get_file_list_pdu, clear (+3) |
| `libs/clipboard/src/platform/unix/local_file.rs` | as_bin, generate_tree, generate_file, as_bin_parse_test, test_parse_file_descriptors (+3) |
| `src/clipboard_file.rs` | clip_2_msg, msg_resp_format_data_failure, resp_file_contents_fail, serve_clip_messages, get_format_list |
| `libs/clipboard/src/platform/unix/fuse/mod.rs` | format_data_response_to_urls, format_data_response_to_urls |
| `libs/clipboard/src/platform/unix/filetype.rs` | parse_file_descriptors |
| `libs/clipboard/src/platform/unix/mod.rs` | get_local_format |

## Entry Points

Start here when exploring this area:

- **`clip_2_msg`** (Function) — `src/clipboard_file.rs:3`
- **`serve_clip_messages`** (Function) — `src/clipboard_file.rs:275`
- **`read_file_contents`** (Function) — `libs/clipboard/src/platform/unix/serv_files.rs:220`
- **`get_file_list_pdu`** (Function) — `libs/clipboard/src/platform/unix/serv_files.rs:268`
- **`format_data_response_to_urls`** (Function) — `libs/clipboard/src/platform/unix/fuse/mod.rs:103`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `clip_2_msg` | Function | `src/clipboard_file.rs` | 3 |
| `serve_clip_messages` | Function | `src/clipboard_file.rs` | 275 |
| `read_file_contents` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 220 |
| `get_file_list_pdu` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 268 |
| `format_data_response_to_urls` | Function | `libs/clipboard/src/platform/unix/fuse/mod.rs` | 103 |
| `format_data_response_to_urls` | Function | `libs/clipboard/src/platform/unix/fuse/mod.rs` | 203 |
| `as_bin` | Function | `libs/clipboard/src/platform/unix/local_file.rs` | 87 |
| `parse_file_descriptors` | Function | `libs/clipboard/src/platform/unix/filetype.rs` | 157 |
| `try_open` | Function | `libs/clipboard/src/platform/unix/local_file.rs` | 44 |
| `construct_file_list` | Function | `libs/clipboard/src/platform/unix/local_file.rs` | 221 |
| `get_format_list` | Function | `src/clipboard_file.rs` | 245 |
| `get_local_format` | Function | `libs/clipboard/src/platform/unix/mod.rs` | 55 |
| `clear_files` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 216 |
| `sync_files` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 259 |
| `msg_resp_format_data_failure` | Function | `src/clipboard_file.rs` | 259 |
| `resp_file_contents_fail` | Function | `src/clipboard_file.rs` | 267 |
| `get_files_for_audit` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 72 |
| `serve_file_contents` | Function | `libs/clipboard/src/platform/unix/serv_files.rs` | 99 |
| `generate_tree` | Function | `libs/clipboard/src/platform/unix/local_file.rs` | 289 |
| `generate_file` | Function | `libs/clipboard/src/platform/unix/local_file.rs` | 296 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Cluster_328 | 1 calls |
| Fuse | 1 calls |
| Cluster_375 | 1 calls |
| Pages | 1 calls |

## How to Explore

1. `gitnexus_context({name: "clip_2_msg"})` — see callers and callees
2. `gitnexus_query({query: "unix"})` — find related execution flows
3. Read key files listed above for implementation details
