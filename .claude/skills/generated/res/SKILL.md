---
name: res
description: "Skill for the Res area of rustdesk. 114 symbols across 10 files."
---

# Res

114 symbols | 10 files | Cohesion: 81%

## When to Use

- Working with code in `res/`
- Understanding how create, upload_file, get_status work
- Modifying res-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `res/ab.py` | get_personal_ab, view_shared_abs, get_ab_by_name, view_ab_peers, update_peer (+19) |
| `res/audits.py` | format_timestamp, get_connection_type_name, get_console_type_name, get_console_operation_name, get_alarm_type_name (+9) |
| `res/job.py` | create, upload_file, get_status, download_files, download_one_file (+8) |
| `res/users.py` | view, disable, delete_user, invite_user, disable_2fa_enforce (+8) |
| `res/device-groups.py` | check_response, headers_with, list_groups, get_group_by_name, create_group (+7) |
| `res/strategies.py` | check_response, headers_with, get_strategy_by_guid, get_device_guid_by_id, get_user_guid_by_name (+7) |
| `res/user-groups.py` | check_response, headers_with, list_groups, get_group_by_name, create_group (+6) |
| `res/devices.py` | view, check, disable, enable, delete (+2) |
| `res/lang.py` | get_lang, line_split, main, expand, to_csv (+1) |
| `flutter/lib/common/widgets/login.dart` | WidgetOP, build |

## Entry Points

Start here when exploring this area:

- **`create`** (Function) — `res/job.py:29`
- **`upload_file`** (Function) — `res/job.py:46`
- **`get_status`** (Function) — `res/job.py:58`
- **`download_files`** (Function) — `res/job.py:65`
- **`download_one_file`** (Function) — `res/job.py:85`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `WidgetOP` | Class | `flutter/lib/common/widgets/login.dart` | 122 |
| `create` | Function | `res/job.py` | 29 |
| `upload_file` | Function | `res/job.py` | 46 |
| `get_status` | Function | `res/job.py` | 58 |
| `download_files` | Function | `res/job.py` | 65 |
| `download_one_file` | Function | `res/job.py` | 85 |
| `fetch` | Function | `res/job.py` | 103 |
| `update_status` | Function | `res/job.py` | 112 |
| `delete_task` | Function | `res/job.py` | 122 |
| `sign` | Function | `res/job.py` | 131 |
| `sign_one_file` | Function | `res/job.py` | 151 |
| `get_json` | Function | `res/job.py` | 175 |
| `sign_files` | Function | `res/job.py` | 200 |
| `main` | Function | `res/job.py` | 222 |
| `check_response` | Function | `res/device-groups.py` | 7 |
| `headers_with` | Function | `res/device-groups.py` | 33 |
| `list_groups` | Function | `res/device-groups.py` | 39 |
| `get_group_by_name` | Function | `res/device-groups.py` | 64 |
| `create_group` | Function | `res/device-groups.py` | 72 |
| `update_group` | Function | `res/device-groups.py` | 83 |

## How to Explore

1. `gitnexus_context({name: "create"})` — see callers and callees
2. `gitnexus_query({query: "res"})` — find related execution flows
3. Read key files listed above for implementation details
