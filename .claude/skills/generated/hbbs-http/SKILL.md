---
name: hbbs-http
description: "Skill for the Hbbs_http area of rustdesk. 56 symbols across 13 files."
---

# Hbbs_http

56 symbols | 13 files | Cohesion: 87%

## When to Use

- Working with code in `src/`
- Understanding how http_request, http_request_sync, do_check_software_update work
- Modifying hbbs_http-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/ui_interface.rs` | http_request, get_option, resolve_avatar_url, get_app_name, video_save_directory (+8) |
| `src/hbbs_http/http_client.rs` | create_http_client, create_http_client_async, get_url_for_tls, create_http_client_with_url, create_http_client_with_url_ (+2) |
| `src/common.rs` | do_check_software_update, post_request_http, post_request_, get_http_response_async, http_request_http (+1) |
| `libs/hbb_common/src/tls.rs` | is_plain, get_domain_and_port_from_url, upsert_tls_cache, get_cached_tls_type, get_cached_tls_accept_invalid_cert (+1) |
| `src/common_old_backup.rs` | post_request_http, post_request_, get_http_response_async, http_request_http, http_request_sync |
| `src/hbbs_http/sync.rs` | heartbeat_url, handle_config_options, start_hbbs_sync, default, start_hbbs_sync_async |
| `src/hbbs_http/record_upload.rs` | run, handle_new_file, handle_frame, handle_tail, handle_remove |
| `libs/hbb_common/src/config.rs` | option2bool, allow_insecure_tls_fallback, get_option_from_file |
| `src/auth_2fa.rs` | from_str, get_2fa |
| `src/hbbs_http/downloader.rs` | do_download |

## Entry Points

Start here when exploring this area:

- **`http_request`** (Function) — `src/ui_interface.rs:856`
- **`http_request_sync`** (Function) — `src/common_old_backup.rs:1588`
- **`do_check_software_update`** (Function) — `src/common.rs:954`
- **`http_request_sync`** (Function) — `src/common.rs:1648`
- **`create_http_client`** (Function) — `src/hbbs_http/http_client.rs:98`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `http_request` | Function | `src/ui_interface.rs` | 856 |
| `http_request_sync` | Function | `src/common_old_backup.rs` | 1588 |
| `do_check_software_update` | Function | `src/common.rs` | 954 |
| `http_request_sync` | Function | `src/common.rs` | 1648 |
| `create_http_client` | Function | `src/hbbs_http/http_client.rs` | 98 |
| `create_http_client_async` | Function | `src/hbbs_http/http_client.rs` | 103 |
| `get_url_for_tls` | Function | `src/hbbs_http/http_client.rs` | 111 |
| `create_http_client_with_url` | Function | `src/hbbs_http/http_client.rs` | 122 |
| `create_http_client_async_with_url` | Function | `src/hbbs_http/http_client.rs` | 231 |
| `is_plain` | Function | `libs/hbb_common/src/tls.rs` | 17 |
| `upsert_tls_cache` | Function | `libs/hbb_common/src/tls.rs` | 41 |
| `get_cached_tls_type` | Function | `libs/hbb_common/src/tls.rs` | 74 |
| `get_cached_tls_accept_invalid_cert` | Function | `libs/hbb_common/src/tls.rs` | 83 |
| `version_check_request` | Function | `libs/hbb_common/src/lib.rs` | 494 |
| `get_fingerprint` | Function | `libs/hbb_common/src/fingerprint.rs` | 255 |
| `option2bool` | Function | `libs/hbb_common/src/config.rs` | 2725 |
| `allow_insecure_tls_fallback` | Function | `libs/hbb_common/src/config.rs` | 2744 |
| `get_option` | Function | `src/ui_interface.rs` | 160 |
| `resolve_avatar_url` | Function | `src/ui_interface.rs` | 250 |
| `get_app_name` | Function | `src/ui_interface.rs` | 765 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Models | 5 calls |
| Pages | 2 calls |
| Cluster_349 | 2 calls |
| Cluster_119 | 2 calls |
| Cluster_364 | 2 calls |
| Cluster_337 | 1 calls |
| Cluster_347 | 1 calls |
| Client | 1 calls |

## How to Explore

1. `gitnexus_context({name: "http_request"})` — see callers and callees
2. `gitnexus_query({query: "hbbs_http"})` — find related execution flows
3. Read key files listed above for implementation details
