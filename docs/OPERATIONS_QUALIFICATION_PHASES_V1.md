# C-008A / Qualification / Phases 1–5 — Operational execution and acceptance
Status: Phase 0 future implementation plan, no code/tests/workflows authorized.

## Operations and release lifecycle
Install: choose per-user Windows install, owner private data directory and exclusive DB ACL; verify authenticode if supplied only informational (mandatory trust remains hash+owner); register on-demand local service, no public bind; secure Credential Manager/DPAPI enrollment. Initial bootstrap creates new database only when path is known absent and user explicitly provisions an instance; any corrupt/unreadable DB enters RECOVERY_REQUIRED. Upgrade: quiesce, checksum all packages, create encrypted verified backup, check migration checksum and source schema, run transactionally, qualify integrity and exact version, health-check loopback and CLI. Roll back binary with compatible store only; otherwise restore pre-upgrade backup with epoch/revocation reconciliation and rekey when required; never silently resurrect grants. Uninstall --preserve-data leaves ACL-protected data; --purge-data requires independently authenticated owner, confirmation and verified secure backup warning. No assumption of guaranteed secure erase on SSD. Daily encrypted backups, 30 points, weekly clean restore drill, 90-day audit default; failure degrades/locks security operations per C-005B. RPO 24h, RTO 30m, startup reconcile 60s are qualification targets on recorded reference fixture, not proven outcomes.

## Windows manual evidence capture (future only)
Prereq: authorized implementation phase and approved fixture harness on sacrificial Windows 10 x64. PowerShell setup:
```powershell
$ErrorActionPreference='Stop'
$repo='djames1987/DragonForge-MCP'
$sha=(git rev-parse HEAD).Trim()
$stamp=Get-Date -Format 'yyyyMMddTHHmmss'
$e=Join-Path $env:TEMP "DFMCP-evidence-$stamp"
New-Item -ItemType Directory -Path $e | Out-Null
git status --porcelain | Out-File "$e/git-status.txt"
Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,OsBuildNumber,OsArchitecture | Out-File "$e/os.txt"
whoami /all | Out-File "$e/token.txt"
"SHA=$sha" | Out-File "$e/sha.txt"
```
Do not run tests unless the exact test harness is implemented, reviewed and authorized. After each future fixture save command stdout/stderr and exit status, audit correlation, before/after SQLite integrity, file digests, owner/worker SID and OS traces. Archive evidence, then remove only identifiable disposable fixtures. Use MANUAL_LOCAL for PowerShell owner runs, SELF_HOSTED_ACTIONS only for actual local self-hosted runner jobs. Never GitHub-hosted.

## T-001–T-026 planned reproduction/acceptance
| T | Future step / positive + negative pass criteria |
|---|---|
| 001 | start gateway loopback, inspect Get-NetTCPConnection; non-loopback config fails |
| 002 | enroll client A, anon/revoked B list and call; no unauthorized tool |
| 003 | forge clientInfo and rotate/revoke credential; only host secret identifies principal |
| 004 | replay C-010A valid/invalid wire fixtures, assert status/errors/schema |
| 005 | discover then revoke and race prelaunch; no post-revoke worker |
| 006 | inject fault after each authority mutation and before audit COMMIT; all or none |
| 007 | scaffold and install hello package with no gateway edits |
| 008 | stage/validate/trust/enable/grant separately; no invocation before all |
| 009 | mutate package manifest/payload byte; digest mismatch, no spawn |
| 010 | run WQ-001..12 probes with independent OS token/denial evidence |
| 011 | crash worker midinvoke, gateway still serves safe request |
| 012 | send invalid input/output JSON schema, malformed keys, overlimits; reject |
| 013 | APR-001..010 spoof/replay/concurrent approval; one admission only |
| 014 | A/B disjoint clients try cached IDs and results; no cross-client exposure |
| 015 | cancellation, saturation, timeout; bounded state and accurate audit |
| 016 | crash at transaction checkpoints, verify UNKNOWN_EFFECT/replay denial |
| 017 | inject safe secret canaries, verify all logs redacted |
| 018 | clean-machine install/upgrade/restore/uninstall with state and key handling |
| 019 | exact qualified SHA and complete local evidence report |
| 020 | Python/C++/Rust IPC frame parity, rejects wrong token/version |
| 021 | hostile Host, Origin and DNS rebinding fixtures rejected predispatch |
| 022 | traversal, junction/reparse swap and race; no outside access |
| 023 | DB corruption/migration fault; RECOVERY_REQUIRED, no silent reset |
| 024 | CLI-001..054 arguments/JSON/error/exit codes and owner auth |
| 025 | verified encrypted clean restore, old authority rollback forces rekey |
| 026 | unsupported MCP features unadvertised, unavailable methods rejected |
Each test needs fixture initial state, cleanup, error code, independent pass/fail evidence and exact SHA. Complete executable commands depend on approved harness; **not yet executable reproduction** and remains freeze documentation gap.

## Implementation gates
Phase 1: SQLite writer, identity/auth, loopback+STDIO, MCP C-010A wire, no privileged worker; T001–006,021,026. Phase 2: immutable SDK, registry, worker W0/W1, owner approval and CLI subset; T007–013,020,022, WQ. Phase 3: scoped file tools, two-agent concurrency, revocation/cancel; T014–015. Phase 4: failpoints, backups/restore, performance reference workload (100 tools, four workers, p95 discovery <=250ms, idle <=200 MiB, restore <=30m fixture); T016,017,023,025. Phase 5: installer, upgrade/rollback, full CLI, local self-hosted or owner manual exact SHA qualification; T018,019,024 plus all regression. Any failed prior gate blocks next phase without written owner exception.

## Open qualification decisions
Pinned fixture names and commands, source-verified MCP schema conformance, CLI per-command actual JSON Schema, full Windows security descriptor proof and actual throughput benchmark hardware must be finalized before owner freeze; this document does not claim they are complete or passed.
