---
name: customactions
description: "Skill for the CustomActions area of rustdesk. 77 symbols across 13 files."
---

# CustomActions

77 symbols | 13 files | Cohesion: 81%

## When to Use

- Working with code in `res/`
- Understanding how flog, PrintXPSRawData, MyCreateServiceW work
- Modifying customactions-related functionality

## Key Files

| File | Symbols |
|------|---------|
| `res/msi/CustomActions/RemotePrinter.cpp` | commonEnum, isNameEqual, isPortExists, getPrinterInstalledOnPort, checkDeleteLocalPort (+16) |
| `res/msi/CustomActions/CustomActions.cpp` | SafeDeleteItem, RecursiveDelete, RemoveInstallFolder, AddFirewallRuleCmdline, RemoveFirewallRuleCmdline (+14) |
| `res/msi/CustomActions/Common.h` | UninstallDriver, QueryServiceStatusExW, IsServiceRunningW, MyCreateServiceW, MyDeleteServiceW (+6) |
| `res/msi/CustomActions/ServiceUtils.cpp` | MyCreateServiceW, MyDeleteServiceW, MyStartServiceW, QueryServiceStatusExW, IsServiceRunningW |
| `res/msi/CustomActions/FirewallRules.cpp` | RemoveFirewallRule, AddFirewallRuleWithEdgeTraversal, MakeRuleName, WFCOMCleanup, AddFirewallRule |
| `libs/clipboard/src/windows/wf_cliprdr.c` | create_cliprdr_window, cliprdr_thread_func, wf_cliprdr_server_format_data_response, wf_cliprdr_server_file_contents_response |
| `src/platform/windows.cc` | flog, InitXpsPrint, PrintXPSRawData |
| `src/platform/windows.rs` | set_cursor_pos, clip_cursor, is_user_token_admin |
| `res/msi/CustomActions/ReadConfig.cpp` | trim, ReadConfig |
| `res/msi/CustomActions/DeviceUtils.cpp` | UninstallDriver |

## Entry Points

Start here when exploring this area:

- **`flog`** (Function) — `src/platform/windows.cc:19`
- **`PrintXPSRawData`** (Function) — `src/platform/windows.cc:947`
- **`MyCreateServiceW`** (Function) — `res/msi/CustomActions/ServiceUtils.cpp:8`
- **`MyDeleteServiceW`** (Function) — `res/msi/CustomActions/ServiceUtils.cpp:56`
- **`MyStartServiceW`** (Function) — `res/msi/CustomActions/ServiceUtils.cpp:87`

## Key Symbols

| Symbol | Type | File | Line |
|--------|------|------|------|
| `flog` | Function | `src/platform/windows.cc` | 19 |
| `PrintXPSRawData` | Function | `src/platform/windows.cc` | 947 |
| `MyCreateServiceW` | Function | `res/msi/CustomActions/ServiceUtils.cpp` | 8 |
| `MyDeleteServiceW` | Function | `res/msi/CustomActions/ServiceUtils.cpp` | 56 |
| `MyStartServiceW` | Function | `res/msi/CustomActions/ServiceUtils.cpp` | 87 |
| `UninstallDriver` | Function | `res/msi/CustomActions/DeviceUtils.cpp` | 10 |
| `SafeDeleteItem` | Function | `res/msi/CustomActions/CustomActions.cpp` | 35 |
| `RecursiveDelete` | Function | `res/msi/CustomActions/CustomActions.cpp` | 80 |
| `RemoveInstallFolder` | Function | `res/msi/CustomActions/CustomActions.cpp` | 186 |
| `AddFirewallRuleCmdline` | Function | `res/msi/CustomActions/CustomActions.cpp` | 371 |
| `RemoveFirewallRuleCmdline` | Function | `res/msi/CustomActions/CustomActions.cpp` | 401 |
| `AddRegSoftwareSASGeneration` | Function | `res/msi/CustomActions/CustomActions.cpp` | 732 |
| `RemoveAmyuniIdd` | Function | `res/msi/CustomActions/CustomActions.cpp` | 782 |
| `UninstallDriver` | Function | `res/msi/CustomActions/Common.h` | 16 |
| `set_cursor_pos` | Function | `src/platform/windows.rs` | 130 |
| `clip_cursor` | Function | `src/platform/windows.rs` | 142 |
| `is_user_token_admin` | Function | `src/platform/windows.rs` | 2473 |
| `GetLastError` | Function | `libs/enigo/src/win/win_impl.rs` | 11 |
| `TerminateProcessIfNotContainsParam` | Function | `res/msi/CustomActions/CustomActions.cpp` | 236 |
| `TerminateProcessesByNameW` | Function | `res/msi/CustomActions/CustomActions.cpp` | 285 |

## How to Explore

1. `gitnexus_context({name: "flog"})` — see callers and callees
2. `gitnexus_query({query: "customactions"})` — find related execution flows
3. Read key files listed above for implementation details
