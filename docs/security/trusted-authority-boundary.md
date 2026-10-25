# Phase 0 — Trusted authority and execution boundary (proposal v0.1)

**Document status:** PROPOSED / NON-NORMATIVE until owner ratification. Phase 0 documentation only. Applies to R-002, R-003, R-005, R-006, R-010, R-013, R-014, R-016, R-021 and R-022, contracts C-001, C-003, C-005, C-006 and C-009, planned tests T-002, T-003, T-005, T-006, T-010, T-013, T-014, T-016, T-021 and T-022. This expands, but does not override, the governing documents.

## Authority matrix

| Operation | Authoritative actor | Untrusted inputs | Mandatory gate and failure |
|---|---|---|---|
| Bind HTTP | Gateway transport | Config, Host, Origin | Bind only configured loopback; reject invalid Host/Origin before dispatch |
| Enroll client | Owner-authenticated control plane | Display name and requested access | Verify independent administrative identity; no client self-enrollment |
| Issue credential | Gateway identity service | Enrollment identifier | Use OS cryptographic RNG; deliver via protected local channel once; persist only verifier |
| Authenticate request | Gateway identity service | Authorization header, MCP metadata | Compare verified credential under constant-time verifier strategy; invalid/revoked/expired -> deny |
| List tools | Gateway authorization + registry | Request filter, client metadata | Verified principal AND current discovery grant AND trusted enabled digest |
| Invoke tool | Gateway admission transaction | Tool name, args, request IDs | Current execution grant, effect/capability bounds, digest and input schema; reject before spawn |
| Trust tool | Owner-authenticated control plane | Files, manifest, claimed publisher | Validate immutable digest and review declarations; trust never implies enable or grant |
| Approve sensitive action | Independent owner-facing control plane | Pending request rendering | Reauthenticate owner and bind single-use decision to canonical request fingerprint; client assertions ignored |
| Spawn worker | Gateway scheduler + OS isolation backend | Package, capability request | Verified immutable package; required OS confinement profile must be attested; otherwise ISOLATION_UNAVAILABLE |
| Commit audit/security state | Gateway SQLite writer | Worker status, CLI/MCP requests | Atomic write+audit with foreign-key and uniqueness enforcement; failure -> rollback |
| Restore store | Offline owner-authorized recovery controller | Backup file and metadata | Validate backup and identity/revocation completeness before atomic activation |

No MCP client, Python tool, native worker, package installer or STDIO subprocess is entitled to write the authority store, mint approvals, change grants or override an admission decision. Localhost, subprocess boundaries and CLI process ancestry are not authenticators.

## Proposed request admission ordering (not yet ratified)

1. Parse and bound transport envelope; validate peer/bind assumptions, Host and Origin. A local socket does not prove client identity.
2. Resolve exactly one protocol adapter; validate applicable headers, methods and JSON-RPC structure against the pinned revision.
3. Verify an issued credential and immutable principal ID; never accept client-selected principal/permission fields.
4. For tools/list, compute visible tools from the current registry snapshot and discovery grants. Unknown and forbidden tool IDs must not leak sensitive metadata.
5. For tools/call, normalize tool ID, check version/digest pinned to registry state, validate input schema and size, check current invocation grant and declared capabilities, and evaluate effect classification.
6. For approval-required calls, persist a pending request with canonical principal/tool-digest/capability/argument fingerprint and expiry. The independent owner-control path either commits one bound approval or denies. An agent-provided boolean, token, argument or tool response cannot authorize this step.
7. Recheck current authorization, revocation epoch, digest and approval immediately before creating an admitted invocation. Consume bound single-use approval and create the invocation/audit record atomically.
8. Schedule only with an attested eligible worker isolation profile, resource budget, deadline and capability subset. Gateway grants workers opaque least-privilege handles; no database file, credential verifier, owner interface or unrestricted IPC authority.
9. Record terminal outcome. A crash, deadline or cancellation does not prove rollback of externally visible side effects. Never automatically retry ambiguous non-idempotent work.

The source of canonicalization for arguments, digest checks and idempotency remains **BLOCKING** until C-002/C-003 have byte-accurate definitions.

## Proposed execution state machine

`RECEIVED -> AUTHENTICATED -> AUTHORIZED -> (WAITING_APPROVAL -> APPROVED) -> ADMITTED -> QUEUED -> RUNNING -> {SUCCEEDED, TOOL_FAILED, CANCELLED, TIMED_OUT, CRASHED, UNKNOWN_EFFECT}`.

From each pre-admission state, a validation or policy failure transitions to `REJECTED` without worker launch. A denial or expiry while waiting yields `REJECTED`. Recheck revoked grants and digest before queue admission AND immediately before actual launch; queued calls become `CANCELLED` if revoked. A successful approval is never reusable after argument, scope, principal, version or digest changes. Terminal states cannot transition back to RUNNING; any permitted retry creates a new invocation linked to the prior record.

The transaction boundary covers **approval consumption, admission, uniqueness and audit**, not worker side effects. Never keep a SQLite write transaction open across worker execution. Crash recovery reconciles persisted `ADMITTED/QUEUED/RUNNING` as uncertain; do not report success without independent completion evidence. State enum, durable transition DDL and race handling still need normative C-005 definitions.

## Windows containment qualification gate

Windows 10 security architecture **cannot** be certified by naming a restricted token, AppContainer or Job Object alone. Proposed defense-in-depth profile must explicitly cover all of:

- Identity/token isolation from the gateway's administrative rights.
- Filesystem and registry denied paths, including gateway SQLite, credentials and package staging/trust stores.
- IPC namespace ACLs and strict worker endpoint identity; denial of privileged pipes and inherited handles.
- Network outbound allowlist or total OS-enforced denial according to the exact declared tool capability.
- Child process containment, resource/process/job limits, timeouts and forced termination.
- Symlink, junction, hardlink and path-reparse race prevention at *use* time, not only initial validation.
- Real negative probes on each supported Windows SKU/build/patch and relevant integrity/ACL context.

Job Objects primarily provide lifecycle/resource constraints and are **not** by themselves filesystem/network/registry security boundaries. Restricted tokens and ACLs require a specified security principal and concrete denied-access fixtures. AppContainer use requires proof of the intended filesystem/network and IPC restrictions on the actual Windows profile. A worker running under the same broadly privileged owner context is not automatically isolated. If any required property has no qualifying control on a device, return `ISOLATION_UNAVAILABLE`, log an audit-safe denial, and do not spawn the tool.

**Open decision:** DEC-004 / ADR-007 must identify each exact supported OS profile, confinement primitive combination, privileges, IPC mechanism, and capability compatibility table. No isolation guarantee is asserted here.

## Negative-security evidence templates (not executed)

| Fixture | Attacker action | Expected invariant | Planned test |
|---|---|---|---|
| ID-FORGE-01 | Set metadata client ID to a privileged principal, use unrelated credential | Request operates only as credential-bound principal or is denied | T-003 |
| GRANT-STALE-01 | Discover then revoke; call cached tool | No worker starts after revocation | T-005 |
| APPROVAL-REPLAY-01 | Reuse previously accepted decision | Second consumption denied and audited | T-013 |
| APPROVAL-MUTATE-01 | Change one argument byte/tool digest after approval | Binding mismatch; no execution | T-013 |
| CLIENT-CROSS-01 | Client B requests client A invocation ID/output | Denied without metadata leak | T-014 |
| PACKAGE-SWAP-01 | Replace validated package path with junction after inspection | Use-time verification denies | T-009 / T-022 |
| WORKER-STORE-01 | Worker opens gateway authority DB/audit/secrets | OS access denied; gateway still healthy | T-010 |
| WORKER-IPC-01 | Worker connects to privileged admin IPC endpoint | OS/channel authentication denies | T-010 |
| WORKER-NET-01 | No-network tool attempts outbound connection | OS-enforced connection failure | T-010 |
| WRITE-FAULT-01 | Interrupt between grant mutation and audit append | Neither partial mutation nor unaudited mutation visible | T-006 |
| CRASH-UNCERTAIN-01 | Crash worker during external side effect | No blind non-idempotent replay; persisted UNKNOWN_EFFECT | T-016 |
| ORIGIN-ATTACK-01 | Malicious browser Origin/Host and DNS rebinding | Rejected before MCP authorization/dispatch | T-021 |

For every future run record exact qualified Git SHA, Windows edition/build, user and worker account boundary, chosen OS primitives, fixture digest, commands, observed NTSTATUS/Win32 denial or equivalent evidence, gateway outcome, audit correlation, cleanup and rerun. A gateway-level denial alone does **not** prove OS-level isolation; T-010 demands independent OS-enforced failure evidence.

## Invariants requiring complete downstream contracts

- **C-001:** credential verifier algorithm, canonical client ID, store protection, rotation/revocation epoch and concurrent request semantics.
- **C-002:** immutable digest, package path and canonicalization.
- **C-003:** authenticated worker IPC framing, worker identity, timeout, cancellation, budget and capability handle structure.
- **C-005:** full DDL, migration/recovery, transactional state transitions and failpoint specification.
- **C-006:** independent owner-authentication transport, approval fingerprint, expiry and one-time replay proof.
- **C-009:** correlation/audit atomicity and retention policy.
- **DEC-004:** Windows supported build and isolation implementation matrix.

Until those are settled, this is an architectural boundary elaboration only—not an implementation-ready contract or owner-approved security guarantee.

## Governance and disposition

Read all 26 approved rules and the pre-phase checklist before progressing. This document does not modify their authority, expand release scope, approve any technical ADR, authorize executable tests or workflows, certify a Windows sandbox, freeze Phase 0, or permit Phase 1. All future GitHub Actions must use owner-controlled self-hosted runners; owner-run manual PowerShell evidence is separately classified. The current disposition remains **PHASE 0 — DOCUMENTATION IN PROGRESS**.
