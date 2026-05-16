---
name: cluster-228
description: "Skill for the Cluster_228 area of rustdesk. 25 symbols across 8 files."
---

# Cluster_228

25 symbols | 8 files | Cohesion: 59%

## When to Use

- Working with code in `src/`
- Understanding how create_tcp_connection, accept_connection, accept work
- Modifying cluster_228-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `src/common_old_backup.rs` | tcp_proxy_request, get_key, secure_tcp_impl, secure_tcp, secure_tcp_silent (+1) |
| `src/common.rs` | tcp_proxy_request, get_key, secure_tcp_impl, secure_tcp, secure_tcp_silent (+1) |
| `libs/hbb_common/src/stream.rs` | set_key, send, next, local_addr |
| `src/server.rs` | accept_connection_, create_tcp_connection, accept_connection |
| `src/ipc.rs` | get_options_async, get_option_async, connect_to_user_session |
| `src/rendezvous_mediator.rs` | udp_nat_listen |
| `src/kcp_stream.rs` | accept |
| `libs/hbb_common/src/lib.rs` | timeout |

## Entry Points

Start here when exploring this area:

- **`create_tcp_connection`** (Function) — `src/server.rs:171`
- **`accept_connection`** (Function) — `src/server.rs:255`
- **`accept`** (Function) — `src/kcp_stream.rs:31`
- **`get_options_async`** (Function) — `src/ipc.rs:1380`
- **`get_option_async`** (Function) — `src/ipc.rs:1389`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `create_tcp_connection` | Function | `src/server.rs` | 171 |
| `accept_connection` | Function | `src/server.rs` | 255 |
| `accept` | Function | `src/kcp_stream.rs` | 31 |
| `get_options_async` | Function | `src/ipc.rs` | 1380 |
| `get_option_async` | Function | `src/ipc.rs` | 1389 |
| `connect_to_user_session` | Function | `src/ipc.rs` | 1531 |
| `get_key` | Function | `src/common_old_backup.rs` | 1717 |
| `secure_tcp` | Function | `src/common_old_backup.rs` | 1897 |
| `create_symmetric_key_msg` | Function | `src/common_old_backup.rs` | 1936 |
| `get_key` | Function | `src/common.rs` | 1804 |
| `secure_tcp` | Function | `src/common.rs` | 1984 |
| `create_symmetric_key_msg` | Function | `src/common.rs` | 2023 |
| `set_key` | Function | `libs/hbb_common/src/stream.rs` | 57 |
| `send` | Function | `libs/hbb_common/src/stream.rs` | 105 |
| `next` | Function | `libs/hbb_common/src/stream.rs` | 116 |
| `local_addr` | Function | `libs/hbb_common/src/stream.rs` | 126 |
| `timeout` | Function | `libs/hbb_common/src/lib.rs` | 116 |
| `accept_connection_` | Function | `src/server.rs` | 143 |
| `udp_nat_listen` | Function | `src/rendezvous_mediator.rs` | 870 |
| `tcp_proxy_request` | Function | `src/common_old_backup.rs` | 1134 |

## Connected Areas

| Area | Connections |
|------|-------------|
| Server | 4 calls |
| Plugin | 4 calls |
| Cluster_837 | 2 calls |
| Cluster_337 | 2 calls |
| Cluster_347 | 2 calls |
| Cluster_351 | 2 calls |
| Cluster_119 | 2 calls |
| Cluster_366 | 2 calls |

## How to Explore

1. `gitnexus_context({name: "create_tcp_connection"})` — see callers and callees
2. `gitnexus_query({query: "cluster_228"})` — find related execution flows
3. Read key files listed above for implementation details
