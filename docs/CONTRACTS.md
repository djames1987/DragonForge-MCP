# Contract catalog — DRAFT, not frozen

These are proposed normative application-level contracts, not a replacement for a pinned MCP specification. Wire-level MCP envelopes MUST follow the separately ratified protocol version.

## C-001 Local identity and authorization
Administrator registers a client ID and issues a unique 256-bit randomly generated secret via local owner-controlled channel. Store only a keyed verifier or protected secret representation; no plaintext credentials in database, logs, command arguments or tool context. On HTTP, authenticate before MCP dispatch using a dedicated authorization credential; never equate IP/Origin/display name with identity. Reject unknown, expired, revoked, malformed or ambiguous credentials. Every tool visibility and call decision uses authenticated client ID, scope, tool ID+digest and current policy state. Revoke takes effect at next authorization decision; active calls retain only already-committed side effects, while pending calls are cancelled when feasible. Credential rotation issues new identity material, supports explicitly bounded overlap if ratified. STDIO bridge must map to one explicitly provisioned gateway client, not inherit unverified client identity from model-provided metadata. Exact credential channel, verifier format and rotation timing remain architecture blockers.

## C-002 Tool registry/manifest
Manifest fields: `schema_version` (integer), `tool_id` (lowercase dot-namespaced string), `version` (SemVer), `entry_kind` (python|native), `entry_point` (relative canonical path), `digest_sha256` (64 lowercase hex), `description` (string), `input_schema` and `output_schema` (JSON Schema under pinned dialect), `capabilities` (array of canonical permission IDs), `effect` (read|write|external), `timeout_ms` (integer), `max_input_bytes`, `max_output_bytes`, `max_parallel`, `idempotency` (none|keyed). Additional properties rejected until extension policy ratified. The digest covers a canonicalized manifest excluding its own digest field plus all package files in sorted normalized-path order; a precise canonicalization algorithm and symlink policy must be ratified before executable implementation. Under owner-selected DEC-006, SHA-256 package-content pinning and explicit local owner trust are mandatory; publisher signatures are optional, informational, and cannot authorize tools. Hashing verifies consistency of pinned bytes, not the identity or safety of the publisher. Schema documents must reject duplicate keys, remote schema fetching and recursive explosions.

Registry key = (tool_id, SemVer, digest). Proposed lifecycle: STAGED -> VALIDATED -> TRUSTED -> ENABLED; DISABLED and REVOKED stop invocation; REMOVED only after no active reference. No skipped transitions. Trust approval includes reviewing fingerprint and requested capabilities. Permissions granted to client are distinct records and never implied by registry state.

### C-002A — Byte-exact package integrity and lifecycle (selected delegated detail)
The normative detailed proposal is [contracts/package-integrity-and-lifecycle.md](contracts/package-integrity-and-lifecycle.md). It selects a precisely byte-framed digest stream with fixed-width manifest digest placeholder, immutable snapshot staging, trust/enable/grant separation and re-verification before execution. C-002A supplements but does not yet fully reconcile all C-002 lifecycle and schema details; cross-document qualification and Windows loader verification remain freeze blockers. Do not treat the selection as an executed security test.

## C-003 Execution envelope
Internal request: `contract_version`, `invocation_id` (gateway-minted UUID), `tool_id`, `tool_version`, `digest`, `deadline_monotonic_ms`, `input`, `capability_tokens` (gateway-issued opaque handles, not raw broad credentials), `correlation_id`. Worker response: same invocation ID, `status` (ok|tool_error|cancelled|deadline|crashed), `output` on success or stable error code on failure, `duration_ms`. Framing must be explicitly length-delimited, maximum-size enforced, authenticated against the expected spawned worker, and strict about unsolicited responses. Persisted idempotency keys are scoped to principal + tool digest + key. Non-idempotent side effects are NEVER automatically retried after ambiguous failure. A timeout cannot guarantee that external side effects were rolled back.

## C-004 Error taxonomy
Application error codes: AUTH_MISSING, AUTH_INVALID, AUTH_REVOKED, ACCESS_DENIED, TOOL_NOT_FOUND, TOOL_NOT_ENABLED, TOOL_UNTRUSTED, DIGEST_MISMATCH, SCHEMA_INVALID, APPROVAL_REQUIRED, APPROVAL_EXPIRED, APPROVAL_INVALID, ISOLATION_UNAVAILABLE, QUEUE_FULL, DEADLINE_EXCEEDED, CANCELLED, WORKER_CRASHED, RESULT_TOO_LARGE, CONFIG_INVALID, STORE_UNAVAILABLE, MIGRATION_FAILED, RECOVERY_REQUIRED, IDEMPOTENCY_CONFLICT, INTERNAL_ERROR. Map to supported MCP error/result semantics per selected era; do not invent wire-specific codes. Caller receives stable safe message and correlation identifier, never stack trace, path to secrets or internal tokens. A tool's ordinary domain error differs from an MCP protocol error.

## C-005 Persistence and transaction boundary
Only gateway storage module may write SQLite. Proposed tables: schema_migrations, clients, credentials, tool_packages, tool_versions, tool_capability_declarations, client_tool_grants, scoped_capability_grants, owner_approvals, invocations, idempotency_records, audit_events, backups_metadata. All have stable primary keys; foreign keys enabled; events carry immutable created_at UTC and monotonic sequence for ordering. Migration transaction locks writer and atomically changes schema version. Registration + credential issuance metadata, approval consumption + invocation admission, permission mutation + audit event must be single transactions. Worker calls MUST occur outside a write transaction. On error rollback all affected records. WAL and busy timeout are implementation choices to ratify with consistency tests. Delete/retention must not remove active revocation data or invalidate audit dependencies. Full CREATE TABLE DDL, exact indexes, data types, cascading actions and upgrade order remain MISSING and are a freeze blocker.

## C-006 Human approval
A sensitive invocation is placed in WAITING_APPROVAL state, owner interface displays caller identity, tool/digest, exact effect, arguments (redacted as necessary), scope and expiry. Owner independently authenticates to management interface; decision signed/bound by gateway to pending request with single-use nonce. Reject client-originated approval tokens, reused approvals, expired approvals and argument/digest changes. Deny if trusted interface unavailable. Exact UI/IPC provenance is an open ADR.

### C-006A — Single-use trusted owner approval (selected delegated detail)
See [contracts/owner-approval-and-admission.md](contracts/owner-approval-and-admission.md) for immutable approval binding, RFC 8785 argument fingerprints, admin named-pipe identity checks, atomic admission transaction, and negative-case fixture IDs. C-005 SQL schema, anti-rollback restore reconciliation and Windows pipe verification remain freeze blockers; these are documented design choices, not implemented security guarantees.

## C-007 Administration CLI
Proposed root command `dfmcp`; groups `server`, `config`, `client`, `tool`, `policy`, `audit`, `db`, `backup`, `diagnostics`. Existing proposed examples remain non-executable documentation. All mutating commands accept explicit `--json` output, never print secret after issuance, and require locally authenticated administrative context. Draft exit codes: 0 success, 2 usage/validation, 3 not found, 4 denied, 5 conflict, 6 temporarily unavailable, 7 operation failed, 8 recovery required. Exact subcommands/arguments, help strings, JSON schema, and noninteractive behavior remain MISSING.

## C-008 Backup/upgrade
Write quiescence or SQLite online backup snapshot under owner-controlled directory; include version manifests and checksums; verify integrity and schema compatibility before restore. Restore must be atomic relative to serving gateway; never silently overwrite live DB. Migration rollback uses verified backup when downmigration cannot preserve state. Corrupt DB moves service to explicit read-only administrative recovery state, not an empty reinitialized registry. Retention and restore-time objectives are owner decisions.

## C-009 Audit
Each policy decision records UTC time, immutable correlation ID, authenticated principal ID or anonymous marker, tool/digest if applicable, outcome/error classification, and redacted request fingerprint. Never log raw tokens, secrets, or sensitive argument values. Ordered logical append-only audit record in protected SQLite; no claim of admin-resistant tamperproofing. Audit storage failure blocks sensitive mutations.

## Contract completion gate
C-001 through C-009 are sketches with known undecided details; NOT implementation-ready. Phase 0 freeze requires detailed schema grammar, API signatures, DDL, transaction matrices, negative-case examples, performance limits and cross-contract test coverage, all reviewed against selected MCP revision.

## New Phase 0 supplemental specifications
- C-005A: [SQLite schema and transaction contract](contracts/sqlite-authority-schema.md).
- C-007A: [Administrative CLI command catalog](contracts/administrative-cli.md).
- WIN-QUAL-001: [Windows isolation qualification](security/windows-confinement-qualification.md).
- [Requirement-to-test traceability](PHASE0_REQUIREMENT_TRACEABILITY.md).
These proposed documents are not security qualification results or freeze approval.

## Phase 0 seven-workstream completion documentation (2026-10-22)
- [C-005B/C-008 recovery atomicity](contracts/recovery-atomicity-v1.md) — checkpoint outbox, crash matrix, stale restore rekey.
- [WIN-SEC-002 Windows enforcement](security/windows-enforcement-v1.md) — exact API and security control design, WQ matrix.
- [C-007B CLI request and result schemas](contracts/cli-json-schemas-v1.md) — CLI-001..054 output typing and identity boundary.
- [C-001A/C-003A identity and worker IPC](contracts/identity-ipc-v1.md) — enrollment, credentials, channel/handle rules.
- [C-010A MCP 2026-07-28 wire](contracts/mcp-2026-07-28-wire-v1.md) — pinned modern-only messages and negative transcripts.
- [C-008A operations and qualification](OPERATIONS_QUALIFICATION_PHASES_V1.md) — Windows operational lifecycle and T-001..026 roadmap.
- [Independent cross-document audit](PHASE0_INDEPENDENT_CROSS_DOCUMENT_AUDIT_2026-10-22.md) — unresolved blocks and readiness classification.

These supplement rather than replace earlier contract documents. Contract completeness and executable qualification are not implied by document creation.

## Phase 0 seven-workstream reconciliation addenda
- [C-005C recovery checkpoint publication and reconciliation](contracts/recovery-reconciliation-v1.md) clarifies cross-resource non-atomicity, fail-closed restore, and migration deltas.
- [CLI-001..054 JSON Schema registry](contracts/cli-command-schema-registry-v1.json) provides command-specific outer IPC request and success envelopes; some nested payload schemas still need detailed closure.
- [C-010B source-verified protocol conformance](contracts/mcp-2026-07-28-conformance-addendum.md) clarifies required HTTP headers, parameter annotations, error codes and STDIO behavior.
- [Independent reconciliation audit](PHASE0_SEVEN_WORKSTREAM_RECONCILIATION_AUDIT_2026-10-22.md) lists remaining blockers explicitly. No document in this section represents executed tests or architecture-freeze approval.

## Authoritative Phase 0 consolidation — 2026-10-22
- [C-005 definitive proposed SQLite migration](contracts/sqlite-authoritative-v1.md) replaces additive/inconsistent C-005A/B/C schema fragments for any later implementation. Older docs remain design history, not a second migration to execute.
- [C-007C administrative authorization and backup key format](contracts/cli-security-key-formats-v1.md) adds owner confirmation, protected credential delivery, AEAD backup and Argon2id key export.
- [WIN-SEC-003 Windows supported-profile candidate](security/windows-enforcement-profile-v1.md) narrows candidate build and specifies OS boundary/negative evidence; this remains unqualified.
- [Q-EXEC-001 future qualification command contract](QUALIFICATION_EXECUTION_CONTRACT_V1.md) lists proposed (not yet implemented) future T-001..026 commands.
- [Final-pass audit](PHASE0_FINAL_PASS_AUDIT_2026-10-22.md) explicitly records remaining freeze blockers. No test execution or Phase 1 permission.
