# WIN-QUAL-001 — Windows 10 confinement qualification specification
**Status: Phase 0 planned qualification, NOT executed.** Related DEC-004/ADR-007, C-003/C-005, R-010/011/020/022, T-010/011/020/022.

## Platform profiles
Initial support candidate: 64-bit Windows 10 desktop, builds 19045 and compatible builds only **when individually qualified**; OS edition, UBR patch level, AppContainer/LPAC APIs, filesystem, Defender/firewall policy, and IPC semantics recorded for each execution. No blanket claim for all Windows 10 builds, Windows 11, Wine, server SKUs, or non-Windows. `ISOLATION_UNAVAILABLE` on missing API, expected denied control not actually denied, unusable supported capability, or incomplete proof. LPAC is preferred if fully viable for SDK runtimes; ordinary AppContainer is an explicitly controlled fallback only after separate equivalent security qualification, never silent fallback to full trust.

Profile W0 `NO_NETWORK_READONLY`: dedicated AppContainer or qualified LPAC identity; zero declared network capabilities; read only immutable package and explicitly approved broker-provided content; private scratch storage; zero direct host file write; max 1 worker invocation/channel; default 4 concurrent workers globally. W1 `BROKERED_FILES`: W0 plus brokered file read/write with validated allowlisted owner-scoped paths and permitted operations; no raw host path or unrestricted write handle provided. W2 `NETWORK`: **excluded from initial untrusted worker execution** pending documented OS-enforced per-destination control and separate owner approval; tools requiring network fail closed. W3 `ADMIN`: unsupported; administrative requests always through owner control plane and cannot originate from workers. In-process built-ins are separately audited and never a loophole for third-party tool code.

## Isolation launch mechanism
1. Trusted gateway creates stable private AppContainer profile SID; isolate each worker pool/tool identity and explicitly enumerate capability SIDs (empty for W0/W1). Use Windows documented profile management and `PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES` with `EXTENDED_STARTUPINFO_PRESENT`; use vetted `CreateProcessAsUser` or compatible supported API path with explicit token handling, restricted handle list and verified resulting token. Record primary vs impersonation token integrity, enabled groups, AppContainer SID and privilege list.
2. Add process to an OS Job Object with kill-on-close, active process limit, process/memory/CPU limits chosen in config; ensure association is complete **before** untrusted code executes (suspended launch if necessary). Handle failed assignment (including nested job restrictions) by terminating untrusted process before resumption. Job Objects control lifecycle/resources, not file/registry/network authority.
3. Worker startup environment must omit gateway secrets, DB locations, admin pipe names, sensitive variables, inherited privileged handles and user profile data. Set safe working directory, executable path and ACLs; ensure parent-owned process handles are non-inheritable. Signed binaries are not a substitute for immutable SHA256 checks.
4. Worker named-pipe endpoint uses a unique random name scoped to execution, restrictive DACL granting only gateway+expected worker identity; `PIPE_REJECT_REMOTE_CLIENTS`, identity check via supported OS client impersonation/access token, expected AppContainer SID and separate worker-instance handshake nonce. PID alone and channel possession do not authenticate. Server must reject anonymous tokens and privilege impersonation; never let workers access owner admin pipe.
5. Gateway-only file broker resolves preapproved scope from current permission record, opens every ancestor/target safely, refuses reparse points, junctions, ADS, hardlinks where applicable, and rechecks file identity/scope at actual use. No brokered unrestricted directory handle. Path-string prefix checks are insufficient.
6. Confirm AppContainer denies access to authority DB, WAL/SHM, owner protected credentials, other workers, sibling process, registry secrets, unrelated Windows named pipes, restricted filesystem paths, and network. Windows default built-in readable files are not assumed confidential; explicitly enumerate permitted ambient reads and call out exceptions.
7. Validate process mitigation policies, DLL search order and module loading for Python/native worker; writable path must not be earlier in sensitive DLL/script/module resolution. Compromised loader or runtime remains residual risk. Do not claim kernel escape protection.

## Threat/assurance table
| Threat | Enforcement | Independent OS evidence | Expected failure |
|---|---|---|---|
| Host DB/secret open | package/private owner ACL + AppContainer SID | Win32 ERROR_ACCESS_DENIED / NTSTATUS ACCESS_DENIED with independent file ACL inspection | no read/write |
| Unapproved write | broker plus inaccessible direct target | handle-open outcome and unchanged file digest | no write |
| Outbound TCP/DNS/UDP | capability-free AppContainer / observed firewall state | socket connection attempt and OS network event evidence | denied, not only app policy |
| Registry secrets | AppContainer token / registry DACL | RegOpenKeyEx denial with token evidence | denied |
| Admin named pipe | pipe DACL and owner SID verification | OS-denied CreateFile or verified identity rejection | no admin command |
| Worker IPC spoof | pipe peer token+instance nonce | attempt from wrong SID/PID/worker token | handshake rejected |
| Child/process escape | job/process token + handle list | child start, parent exit, cross-process access probes | job limit enforced |
| Symlink/junction race | broker handle-based scope | deterministic swap loop across call | outside file untouched |
| Changed package | digest+immutable snapshot | mutate staged/installed package after verification | no worker spawn |
| Crash/timeout | job termination and gateway audit | kill worker during running request | gateway healthy; UNKNOWN_EFFECT if uncertain |

## Reproducible local Windows qualification instructions (future Phase 2 only)
**Prerequisites:** approved owner-authorized Phase 1 and Phase 2 implementation; exact tagged local test build; owner-controlled Windows 10 x64 VM or dedicated test user; sacrificial files and test-only identities; no production secrets; administrator privileges only for fixture preparation; gateway runs as intended non-admin service; PowerShell 5.1+; Sysinternals Process Explorer/ProcMon if independently approved. Never run malicious tool fixtures on live production data.

**Preparation PowerShell (documentation commands for later manual use; not executed now):**
```powershell
$ErrorActionPreference = 'Stop'
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$root = Join-Path $env:TEMP "dfmcp-qual-$stamp"
New-Item -Path $root -ItemType Directory | Out-Null
"DO_NOT_MODIFY" | Set-Content -LiteralPath (Join-Path $root 'protected.txt')
Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture | Out-File (Join-Path $root 'os.txt')
[Environment]::OSVersion.VersionString | Out-File (Join-Path $root 'os-runtime.txt')
whoami /all | Out-File (Join-Path $root 'owner-token.txt')
Get-FileHash -Algorithm SHA256 (Join-Path $root 'protected.txt') | Out-File (Join-Path $root 'before-hash.txt')
```
Create test worker fixtures by the approved Phase 2 test tooling only; fixture names: WQ-001 token/profile, WQ-002 DB/secret read, WQ-003 registry, WQ-004 outbound network, WQ-005 owner pipe, WQ-006 wrong-worker pipe, WQ-007 child/job escape, WQ-008 reparse swap, WQ-009 package mutation, WQ-010 worker crash/timeout, WQ-011 file broker read/write scope, WQ-012 ambient file and DLL/module load inspection. Every fixture reports attempted OS API, expected denial, observed NTSTATUS/Win32 code, worker SID, gateway invocation/correlation ID and separate audit record. A JSON-level ACCESS_DENIED alone is **not** adequate OS-isolation evidence.

**Commands once the actual approved Phase 2 test harness exists** (placeholders intentionally not runnable and therefore NOT a claim of specified harness implementation):
`dfmcp diagnostics isolation --json` and `dfmcp diagnostics run-probe --fixture WQ-002 --json` through WQ-012; exact command definitions are contract proposals in C-007A. Preserve the JSON output, Windows ETW/ProcMon trace as appropriate, immutable package digests and OS build. Rehash protected file and assert unchanged, confirm job closed, verify gateway remains healthy. Fixture must not use broad filesystem permissions to make tests pass.

**Cleanup:**
```powershell
# Confirm all qualification workers have terminated and no handles remain before cleanup.
Get-FileHash -Algorithm SHA256 (Join-Path $root 'protected.txt') | Out-File (Join-Path $root 'after-hash.txt')
# Remove-Item -LiteralPath $root -Recurse -Force   # Owner reviews and archives evidence first.
```
Do not silently remove evidence or AppContainer profiles; enumerate generated SIDs and only remove identified disposable test profiles after archiving.

## Acceptance criteria
For each supported build/edition/patch profile: every W0/W1 denial with independent OS evidence, worker token correct, no owner process or resource access, path-race fixture unchanged, no network leakage, bounded job termination, gateway alive, accurate audit and no bypass. W2/W3 denied. Any failure -> profile not supported; record defect and rerun on **same exact SHA** after repair. No self-hosted runner available? classify results MANUAL_LOCAL and provide full transcript. Do not imply qualification on a Windows build that was not tested. A local self-hosted Actions runner, if later used, must have explicit `self-hosted` Windows-specific labels and no hosted fallback.

## References and limitations
Microsoft: https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation and https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer . Windows sandbox claims are conditional on real build-specific proofs; AppContainer and Job Objects do not protect against all kernel vulnerabilities or owner-account compromise. Exact desired resource budgets and precise executable fixture command line must be ratified in C-003/C-007 before full freeze.
