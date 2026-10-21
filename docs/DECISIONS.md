# Architecture decision register — DRAFT / awaiting owner ratification

Owner-approved product decisions from README.md on 2026-10-21: on-demand server, explicit sensitive-action approval with default deny if unavailable, manual third-party installation and trust, CLI first, simple tool extensibility, Windows 10 first, local only. Technical decisions below are NOT automatically owner-approved.

## ADR-001 — MCP revision compatibility
Status: PROPOSED, BLOCKING.
Context: The MCP 2026-07-28 revision uses server/discover and no protocol-level handshake/session; 2025-11-25 uses initialize and may involve session IDs. Combining semantics by accident creates failures and security gaps.
Proposal: use 2026-07-28 as normative modern baseline; support 2025-11-25 through explicitly separate compatibility behavior only if required by tested local clients. Return defined protocol errors for unsupported versions. No assumption that the old handshake is required in modern mode.
Alternatives: 2025-only (broader legacy host reach, aging baseline); modern-only (simpler, excludes legacy hosts); explicit dual-era adapter (more test work).
Consequences: maintain a two-era protocol matrix if selected. Security: authenticate each request, independent of transport session. Qualification: T-004, T-026 and cross-era transcripts.
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
DEC-001 Modern-only or verified dual-era MCP?
DEC-002 Concrete local client identity credential provisioning/rotation mechanism?
DEC-003 Owner approval UX: authenticated CLI pending-request flow or separate desktop UI?
DEC-004 Exact Windows security target, measured OS containment, and release resource thresholds?
DEC-005 Default retention and backup lifecycle?
DEC-006 Signed provenance requirement for third-party packages vs local SHA-256 trust alone?
Do not mark accepted before owner explicitly approves.
