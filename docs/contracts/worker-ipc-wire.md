# C-003 — Worker IPC wire contract, candidate v0.2

**Status:** PROPOSED / NOT OWNER-RATIFIED. **No implementation or test has been executed.** Extends the C-003 summary in `docs/CONTRACTS.md`; authoritative behavior must be reconciled there before freeze. Applies to R-010, R-011, R-012, R-015, R-016, R-020 and T-010, T-011, T-012, T-015, T-016, T-020. Requires ratification of ADR-002, ADR-007, DEC-004.

## 1. Endpoint and trust ownership

The trusted gateway spawns a specifically qualified worker under an OS confinement profile. The gateway controls creation, lifetime, concurrency, framing and request IDs. A worker may only return data and requested state; it may **never** issue authority-store mutations, mint identities, approvals or grants, or open a privileged control-plane endpoint. Gateway-to-worker IPC is a dedicated unidirectional-authority channel with bidirectional messages. Transport is a **proposed** Windows named pipe with a gateway-generated unpredictable endpoint and restrictive pipe DACL; concrete worker-token binding and client impersonation precautions are blocking DEC-004 security details. On Linux/macOS later, equivalent authenticated local IPC must be qualified.

Connection possession is not authentication. The gateway must verify the connecting peer's expected OS principal and intended worker instance before accepting messages. Worker process ID alone is insufficient, since PID reuse and handle inheritance are possible. Unverifiable peer identity means `ISOLATION_UNAVAILABLE` and **no tool execution**. No pipe endpoint or secret is exposed to unrelated client contexts.

## 2. Version and framing proposal

- IPC protocol identifier: `dfmcp.worker`; candidate major/minor `1.0`. Compatibility: reject unsupported major; unknown fields rejected in security-significant messages; minor extensions only by a documented allowlist.
- Each complete message is `uint32` **unsigned big-endian byte length**, then precisely that number of UTF-8 bytes encoding one JSON object. No NUL delimiter, newline separator or additional framing.
- `length = 0`, `length > 4,194,304`, truncated message, non-UTF-8, malformed JSON, duplicate object keys, missing required fields, excessive nesting or noncanonical field types -> close channel and record safe `IPC_PROTOCOL_INVALID` classification (add to C-004 during reconciliation). No partial message enters dispatch.
- A message must be fully buffered and validated before processing. No compression, network URLs, remote JSON Schema references, unbounded arrays or NaN/Infinity. Implementers must specify a depth limit (candidate 32) and cap message processing memory before freeze.
- Gateway strictly correlates all response IDs to a single previously admitted invocation on the same channel; unsolicited or duplicate terminal responses are protocol violations.
- JSON keys and enums are case-sensitive. Protocol timestamps use UTC RFC 3339 where wall-clock time is required; monotonic relative deadlines are gateway-controlled and never derived solely from worker wall clock.

## 3. Candidate message shapes

All messages share these required fields: `protocol="dfmcp.worker"`, `version="1.0"`, `kind` (enum), `channel_id` (gateway-created opaque 128-bit random identifier encoded as lower-case hex). No field grants permissions merely because it is present. The gateway binds `channel_id` to the qualified process/channel outside JSON.

**HELLO** (worker -> gateway; no tool execution):
```json
{"protocol":"dfmcp.worker","version":"1.0","kind":"hello","channel_id":"0123456789abcdef0123456789abcdef","worker_kind":"python","pid":1234}
```
`pid` is diagnostic-only, not an authenticator. The gateway accepts HELLO only once following OS peer verification and before the configured handshake timeout.

**READY** (gateway -> worker, following verified HELLO):
```json
{"protocol":"dfmcp.worker","version":"1.0","kind":"ready","channel_id":"0123456789abcdef0123456789abcdef"}
```
No invocation may start before READY; a failed or timed-out handshake terminates the process.

**INVOKE** (gateway -> worker; illustrative fields and values):
```json
{"protocol":"dfmcp.worker","version":"1.0","kind":"invoke","channel_id":"0123456789abcdef0123456789abcdef","invocation_id":"123e4567-e89b-42d3-a456-426614174000","tool_id":"example.echo","tool_version":"1.0.0","package_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","deadline_budget_ms":5000,"input":{"text":"hello"},"capability_handles":[]}
```
`invocation_id` is gateway-minted and globally unique; `tool_id/version/digest` must match the immutable authorized registry snapshot. `deadline_budget_ms` is a positive integer with gateway-enforced upper bound, not a worker-chosen extension. `capability_handles` are per-invocation opaque, narrowly scoped handles whose authority is mediated by the gateway and/or OS: **never a broad bearer credential**. Their structure is an unresolved dependent contract; empty list indicates no delegated capability.

**RESULT** (worker -> gateway, terminal):
```json
{"protocol":"dfmcp.worker","version":"1.0","kind":"result","channel_id":"0123456789abcdef0123456789abcdef","invocation_id":"123e4567-e89b-42d3-a456-426614174000","status":"ok","output":{"text":"hello"}}
```
`status` enum candidate: `ok | tool_error | cancelled`. Non-`ok` results require a documented safe error category and must omit `output`. A worker `ok` alone is not proof of an external side effect; gateway still validates output schema and commits a terminal audit transition.

**CANCEL** (gateway -> worker):
```json
{"protocol":"dfmcp.worker","version":"1.0","kind":"cancel","channel_id":"0123456789abcdef0123456789abcdef","invocation_id":"123e4567-e89b-42d3-a456-426614174000","reason":"client_cancelled"}
```
Worker receipt does not guarantee reversal of side effects. Gateway applies its own bounded grace period and OS-enforced termination, then records the most accurate state; uncertain side effects remain `UNKNOWN_EFFECT`.

## 4. State and concurrency invariants

Allowed connection lifecycle: `CREATED -> PEER_VERIFIED -> HELLO_VERIFIED -> READY -> CLOSED`. Any message outside its permitted state fails closed. Initial proposed concurrency is **one active invocation per worker process/channel**; scheduler-level parallelism uses separate workers, bounded by ratified resource budgets. This simplifies response correlation but requires an owner-ratified throughput acceptance target.

Invocation lifecycle is owned by the gateway, not by a worker-reported status: `ADMITTED -> QUEUED -> RUNNING -> SUCCEEDED | TOOL_FAILED | CANCELLED | TIMED_OUT | CRASHED | UNKNOWN_EFFECT`. An unauthorized/revoked call must not transition to RUNNING. A connection closure while active is treated as ambiguous until authoritative recovery reconciliation. Terminal invocation records are immutable; retries create a new invocation ID linked to predecessor.

**No blind retries** for `idempotency=none`. For keyed idempotent tools, the gateway checks its principal+tool-digest+key authoritative table before re-admission; a duplicate with different canonical argument fingerprint becomes `IDEMPOTENCY_CONFLICT`. Worker response duplication must not cause duplicate terminal transitions. Exact argument fingerprint canonicalization is a dependency on C-002/C-005, not resolved here.

## 5. Failure matrix — all pending qualification

| Input or incident | Required gateway response | Planned verification |
|---|---|---|
| Unverified pipe peer, wrong token or stolen channel ID | Refuse handshake; no invocation, no privileged IPC | T-010 |
| Invalid length, duplicate key, unknown message kind | Close pipe; safe protocol error; preserve gateway | T-011,T-020 |
| Wrong invocation ID, duplicate terminal response | Reject message; never attach result to another invocation | T-011,T-014 |
| Oversize request or response | Reject before unbounded allocation; terminate worker if hostile | T-012 |
| Worker process exits mid-write | Record crash/uncertainty, not success; gateway survives | T-011,T-016 |
| Deadline expires | Cancel then bounded terminate; side-effect state assessed separately | T-015,T-016 |
| Output schema malformed | Fail result validation; redact and audit safe error | T-012 |
| Permission revoked during queue | Recheck before launch; cancel queued execution | T-005 |
| Worker attempts gateway DB or admin IPC access | OS denial independent of protocol validation | T-010 |
| Replayed INVOKE / spoofed RESULT | Correlation and state checks reject | T-016,T-020 |

## 6. Portability and normative completion blockers

Native C++ and Rust workers must use the same frame format and semantic contract, without privileged database credentials or bypasses. Before approval, fill these exact gaps: (a) Windows named-pipe security descriptor, peer-token verification, and handle inheritance; (b) capability handle format and scope; (c) JSON Schema per message, strict canonicalization and duplicate-key decoder specification; (d) handshake and cancellation time limits; (e) hard resource budgets for each OS/profile; (f) worker package execution and use-time digest verification; (g) audit/store transition DDL and idempotency semantics; (h) all positive/negative test fixtures with actual future execution procedures. These are not optional implementation details.

This contract is **proposed**, not a guarantee of secure confinement, implemented IPC or passed interoperability testing. Changes to C-003 must reconcile C-002/C-004/C-005, ADR-007, R-010/R-020 and T-010/T-020 before architecture freeze.
