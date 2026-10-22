# Q-EXEC-001 — Future local Windows qualification command contract
**DOCUMENTATION ONLY.** No harness, test runner, binary or scripts are implemented or executed. These are **exact proposed future commands**; the documented command set must be implemented and reviewed in the authorized phases before an owner may run them. It is forbidden to claim the commands currently exist. Governing rules 1–26: no Phase 0 executable test code and no hosted GitHub Actions.

## Required future runner interface
A future approved tool `dfmcp-qual.exe` SHALL support `run --test T-NNN --fixture default --out ABSOLUTE_PATH`, `list --json`, `verify --run DIR --json`. Run must require an isolated test instance, exact repo HEAD SHA, and a local owner-controlled Windows host. It must print one JSON result with fields `test_id,sha,status,expected,observed,artifacts,os_build,run_type,started_at_utc,ended_at_utc`; process exit 0=PASS, 1=FAIL, 2=fixture invalid, 3=environment unsupported. It writes no secrets to artifacts. For OS isolation probes it SHALL embed independent Win32/ETW evidence, not only gateway logs. Every test's fixture definition must list initial DB/package client identities, controlled side effects, before/after hashes and cleanup. The default fixtures are deterministic as part of Phase 1–5 implementation, not data invented in Phase 0.

## Reproducible common owner PowerShell procedure — after implementation authorization
```powershell
$ErrorActionPreference = 'Stop'
$repoPath = 'C:\\Projects\\DragonForge-MCP'  # set to actual approved local checkout
Set-Location -LiteralPath $repoPath
$sha = (git rev-parse HEAD).Trim()
if ((git status --porcelain).Length -ne 0) { throw 'Working tree not clean' }
$root = Join-Path $env:TEMP ('dfmcp-qual-' + (Get-Date -Format yyyyMMddTHHmmss))
New-Item -ItemType Directory -Path $root | Out-Null
"SHA=$sha" | Set-Content -LiteralPath (Join-Path $root 'baseline.txt')
Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture |
  ConvertTo-Json | Set-Content -LiteralPath (Join-Path $root 'os.json')
whoami /all | Set-Content -LiteralPath (Join-Path $root 'owner-token.txt')
$qual = Join-Path $repoPath 'out\\approved-qualification\\dfmcp-qual.exe'
if (-not (Test-Path -LiteralPath $qual)) { throw 'Qualification harness not implemented/approved' }
```
For an individually approved fixture use the per-test future command below with `<evidence-path>` replaced by `$root`. **Never execute against production credentials/files, unknown public IPs or owner personal data.** Run in disposable Windows 10 test VM / dedicated test user, and only after owner's Phase 1+ authorization.

## T-ID command and acceptance matrix
| Test | Exact future command pattern | Expected |
|---|---|---|
| T-001 | `dfmcp-qual run --test T-001 --fixture default --out "<evidence-path>\\T-001"` | only loopback binds; non-loopback config denied |
| T-002 | `dfmcp-qual run --test T-002 --fixture default --out "<evidence-path>\\T-002"` | unknown and revoked identities see no tools |
| T-003 | `dfmcp-qual run --test T-003 --fixture default --out "<evidence-path>\\T-003"` | forged metadata and expired/rotated tokens denied |
| T-004 | `dfmcp-qual run --test T-004 --fixture default --out "<evidence-path>\\T-004"` | modern request accepted; legacy/header mismatch rejected |
| T-005 | `dfmcp-qual run --test T-005 --fixture default --out "<evidence-path>\\T-005"` | revocation between list and call blocks spawn |
| T-006 | `dfmcp-qual run --test T-006 --fixture default --out "<evidence-path>\\T-006"` | SQL fault leaves audit and grant atomic |
| T-007 | `dfmcp-qual run --test T-007 --fixture default --out "<evidence-path>\\T-007"` | new independent hello tool installed without core edits |
| T-008 | `dfmcp-qual run --test T-008 --fixture default --out "<evidence-path>\\T-008"` | stage/trust/enable/grant each distinct and tested |
| T-009 | `dfmcp-qual run --test T-009 --fixture default --out "<evidence-path>\\T-009"` | package byte tampering blocks launch |
| T-010 | `dfmcp-qual run --test T-010 --fixture default --out "<evidence-path>\\T-010"` | all WQ OS denial probes pass independently |
| T-011 | `dfmcp-qual run --test T-011 --fixture default --out "<evidence-path>\\T-011"` | worker killed without gateway collapse |
| T-012 | `dfmcp-qual run --test T-012 --fixture default --out "<evidence-path>\\T-012"` | invalid JSON/schema/oversize rejected |
| T-013 | `dfmcp-qual run --test T-013 --fixture default --out "<evidence-path>\\T-013"` | approval spoof/replay/expiry and nonce race fail |
| T-014 | `dfmcp-qual run --test T-014 --fixture default --out "<evidence-path>\\T-014"` | cross-client list/invoke/results isolated |
| T-015 | `dfmcp-qual run --test T-015 --fixture default --out "<evidence-path>\\T-015"` | deadline/cancel queues bounded and terminal |
| T-016 | `dfmcp-qual run --test T-016 --fixture default --out "<evidence-path>\\T-016"` | crash and ambiguous effects never auto-replayed |
| T-017 | `dfmcp-qual run --test T-017 --fixture default --out "<evidence-path>\\T-017"` | canaries absent from all logs |
| T-018 | `dfmcp-qual run --test T-018 --fixture default --out "<evidence-path>\\T-018"` | fresh machine install upgrade restore uninstall safe |
| T-019 | `dfmcp-qual run --test T-019 --fixture default --out "<evidence-path>\\T-019"` | exact SHA/OS/evidence/runner captured |
| T-020 | `dfmcp-qual run --test T-020 --fixture default --out "<evidence-path>\\T-020"` | Python C++ Rust IPC compatible and hostile frames denied |
| T-021 | `dfmcp-qual run --test T-021 --fixture default --out "<evidence-path>\\T-021"` | invalid Host Origin DNS rebinding rejected predispatch |
| T-022 | `dfmcp-qual run --test T-022 --fixture default --out "<evidence-path>\\T-022"` | traversal reparse swap fails without outside access |
| T-023 | `dfmcp-qual run --test T-023 --fixture default --out "<evidence-path>\\T-023"` | corrupt DB/migration enters RECOVERY_REQUIRED |
| T-024 | `dfmcp-qual run --test T-024 --fixture default --out "<evidence-path>\\T-024"` | all 54 CLI schemas/error/exit tested |
| T-025 | `dfmcp-qual run --test T-025 --fixture default --out "<evidence-path>\\T-025"` | encrypted backup restore preserves revocation history |
| T-026 | `dfmcp-qual run --test T-026 --fixture default --out "<evidence-path>\\T-026"` | unsupported extensions denied and not advertised |

## Per-test evidence and destructive cleanup
For each test capture command argv (redacting only secrets), stdout, stderr, exit, exact binary SHA256, fixture manifest, test-only IDs and pre/post state hashes, SQLite integrity result, audit correlation, expected and actual OS API result. For T-010 and T-022, capture expected denial at OS boundary (ETW/ProcMon plus Win32 code). On crash/failpoints protect test-only snapshots and ensure no live DB handles remain. T-019 verifies the report set itself and rejects omitted tests/unsigned claims. Archival manifests SHA256 each artifact; redact token material and actor-private data. After archiving and owner inspection remove only **known-generated** temp profiles/files and disposable test users, never delete host app data by wildcard. A MANUAL_LOCAL report differs from SELF_HOSTED_ACTIONS, which requires actual owner-controlled local runner job evidence.

## Acceptance rules
All positive fixtures must succeed and all negative fixtures be rejected for the *intended reason*. An infrastructure exception is NOT a pass. A failed test is fixed in a later authorized phase and rerun against exact replacement SHA; pre-phase gates require no outstanding failures unless owner grants written exception. Benchmark baseline uses 100 registered tools, 4 concurrent workers, discovery p95 <=250ms, gateway idle working set <=200MiB, backup recovery <=30m and RPO <=24h under an explicitly recorded reference Windows machine; these are selected candidate acceptance thresholds, not measured evidence.
