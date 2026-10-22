# Architecture decision register — DRAFT / awaiting owner ratification

Owner-approved product decisions from README.md on 2026-10-21: on-demand server, explicit sensitive-action approval with default deny if unavailable, manual third-party installation and trust, CLI first, simple tool extensibility, Windows 10 first, local only. Technical decisions below are NOT automatically owner-approved.

## ADR-001 — MCP revision compatibility
Status: OWNER-DIRECTED CHOICE RECORDED 2026-10-21 (DEC-001); detailed C-010 completion and architecture freeze remain pending.
Context: The MCP 2026-07-28 revision uses server/discover and no protocol-level handshake/session; 2025-11-25 uses initialize and may involve session IDs. Combining semantics by accident creates failures and security gaps.
Decision DEC-001: **Support MCP 2026-07-28 only in the initial release.** Pin the released revision for HTTP and STDIO semantics. No legacy 2025-11-25 adapter in initial release, no automatic fallback, and no hybrid handshake. Reject unsupported versions according to exact released normative contract. Any later 2025 compatibility is a separately scoped, approved, implemented and qualified adapter, never an implicit promise.
Alternatives: 2025-only (broader legacy host reach, aging baseline); modern-only (simpler, excludes legacy hosts); explicit dual-era adapter (more test work).
Consequences: simpler, auditable version boundary but older clients may not connect; record interoperability exclusions and do not claim universal client compatibility. Authenticate every request independently from transport/client metadata. Qualification: T-004,T-026 with modern and explicit legacy-rejection transcripts. Released source: https://modelcontextprotocol.io/specification/2026-07-28 ; https://blog.modelcontextprotocol.io/posts/2026-07-28/ .
References: https://modelcontextprotocol.io/specification/draft/changelog ; https://py.sdk.modelcontextprotocol.io/whats-new/ (verify pinned released normative text before ratification).

## ADR-002 — Gateway/worker split
Status: PROPOSED, BLOCKING.
Decision: trusted administrative authority only in gateway; workers use minimal versioned IPC and never hold database writer credentials. Alternative: in-process plugins (rejected for untrusted code). Security consequence: requires qualified OS containment. Tests: T-010,T-011,T-020.

## ADR-003 — Storage
Status: PROPOSED, BLOCKING.
Decision: SQLite WAL, one authoritative storage boundary with write transactions and controlled migrations; never grant worker direct database write access. Alternative: separate server database (unnecessary cloud/service burden). Tests T-006,T-016,T-023,T-025.

## ADR-004 — HTTP/STDIO
Status: PROPOSED, BLOCKING.
Decision: HTTP loopback as primary multi-client entry; STDIO bridge delegates authenticated operations to gateway and does not forge its own identity. Alternative: STDIO-only (poor shared discovery); open network (outside approved scope). Tests T-001,T-003,T-021.

## ADR-005 — Tool packages
Status: PROPOSED, BLOCKING.
Decision: immutable versioned package with digest, schema and declared permissions; lifecycle staged -> verified -> trusted -> enabled -> client grant. Digest invalidation on any code/manifest mutation. Alternative: implicit auto-install and auto-trust (rejected). Tests T-007 through T-009.

## ADR-006 — Privileged approvals
Status: PROPOSED, BLOCKING.
Decision: approval originates only from a trusted owner-controlled local interface, bound to principal, tool digest, argument hash, scope, expiry, nonce and single use. MCP client-supplied approval claims cannot satisfy policy. Alternative: text prompt in model tool arguments (rejected). Test T-013. Exact desktop/CLI approval handshake remains OPEN.

## ADR-007 — OS isolation
Status: OPEN, BLOCKING.
Windows 10 editions vary in availability and behavior of AppContainer, restricted tokens, job objects, ACLs, firewall and desktop interaction. None alone is assumed a complete sandbox. A measured isolation profile must be chosen per tool capability and OS edition before enabling third-party tools. Otherwise deny. Test T-010,T-022.

## ADR-008 — Audit and secrets
Status: PROPOSED, BLOCKING.
Decision: gateway-owned append-only logical audit sequence in SQLite, protect integrity using database ACLs and checked ordering; redact secrets and sensitive arguments. Do not claim tamper resistance against owner/admin or filesystem adversary. Protect credentials under OS user identity. Tests T-006,T-017.

## ADR-009 — Performance/reliability
Status: OPEN, non-blocking for initial drafting but required for freeze.
Choose agreed hardware tier, tool fixture count, concurrency, resource thresholds, retention, backup interval and crash/restore targets; then derive measurable qualification.

## Owner ratification questions
DEC-001 RESOLVED by owner direction: modern-only 2026-07-28 in v1; older revisions excluded unless a future separately approved change adds them.
DEC-002 Concrete local client identity credential provisioning/rotation mechanism?
DEC-003 Owner approval UX: authenticated CLI pending-request flow or separate desktop UI?
DEC-004 Exact Windows security target, measured OS containment, and release resource thresholds?
DEC-005 Default retention and backup lifecycle?
DEC-006 RESOLVED by owner direction: initial release uses **mandatory SHA-256 content pinning and explicit local owner trust**, without mandatory third-party publisher signatures or certificate infrastructure. Optional signature data can be displayed as untrusted informational metadata; it does not bypass digest validation, local owner trust, enablement, grants, approval or isolation. If source attribution is essential for a deployment, the owner must verify publisher separately before trusting. Signing enforcement requires a separate approved versioned policy and qualification.
DEC-001 and DEC-006 were selected under the owner's explicit instruction to use agent recommendations (2026-10-21). Other decisions remain OPEN and no project-wide documentation freeze is approved.

## DEC-006 — package provenance tradeoff and qualification
Status: OWNER-DIRECTED CHOICE RECORDED 2026-10-21. Signing is not required for initial release. A cryptographic digest detects modification relative to the pinned bytes **but does not prove who authored those bytes or whether they are safe**. Each complete staged package must be hashed by a deterministic byte-accurate scheme defined in C-002; package paths, symlinks, embedded digest field, canonical encoding and time-of-check/time-of-use are freeze blockers. Trust is a privileged owner-reviewed operation tied to exact (tool ID, version, SHA-256, capabilities), and is invalidated by byte changes. SHA-256 pinning, owner trust, enablement and per-client grants remain separate. Negative fixtures T-008/T-009/T-022 must exercise mutations, reparse points and substitution races. Future signed provenance requires formal scope review, key lifecycle, revocation and authenticity qualification; no unvalidated claimed signature may confer privileges.

## Delegated Phase 0 technical selections — 2026-10-22

Authority: owner delegation recorded in current Phase 0 handoff. **Status: SELECTED — AGENT-DELEGATED AUTHORITY, NOT FROZEN.** Detailed selected designs, rejected alternatives, security limitations, tests, and outstanding contract ratification are in [PHASE0_DELEGATED_DECISIONS_2026-10-22.md](PHASE0_DELEGATED_DECISIONS_2026-10-22.md).

- **DEC-001** retained (owner selected): MCP 2026-07-28 only.
- **DEC-002** selected (agent delegated): individually owner-enrolled bearer principals, keyed protected credential verification, 90-day credential lifetime, short explicit rotation overlap, immediate revocation and protected STDIO adapter identity.
- **DEC-003** selected (agent delegated): independently OS-authenticated trusted owner CLI and restrictive local admin named-pipe channel; canonical, invocation-bound, expiring, single-use SQLite-atomic approvals.
- **DEC-004** selected (agent delegated, subject to OS-mechanism qualification): AppContainer baseline + ACL/broker controls + Job Objects + validated IPC, denying all unsupported third-party confinement profiles.
- **DEC-005** selected (agent delegated): 90-day audit metadata policy, daily encrypted local backups, 30 recovery points, weekly restoration drill and provisional measurable recovery targets.
- **DEC-006** retained (owner selected): SHA-256 content verification + explicit owner trust, no mandatory publisher signatures.

**Precedence:** The earlier OPEN/PROPOSED entries above are retained as historical draft rationale; where inconsistent with this delegated selection, the decision selection above applies for Phase 0 drafting only. The detailed contracts, SQL DDL, threat model, fixtures, tests and implementation plan must still be reconciled before an architecture freeze. No owner approval of freeze or Phase 1 implementation has been issued. No security test has been executed merely by documenting a selection.
