# C-006A — Owner approval authorization and atomic admission
**Status:** SELECTED — AGENT-DELEGATED DETAIL, not frozen or implemented.
**Applies:** DEC-003, C-001, C-004, C-005, C-006, C-007, C-009, R-005/R-006/R-013/R-016, T-005/T-006/T-013/T-016. Development rules remain authoritative.

## Security invariant
An authenticated MCP client, whether HTTP or STDIO, can request a sensitive action but **cannot approve it**. Approval may occur only over the gateway's separate owner-authenticated administrative named pipe. The gateway owns the pending request and binds a single approving human decision to exactly one eligible invocation. Credentials, claimed user names, process IDs, client-provided booleans, model/tool messages and mere pipe possession cannot substitute for Windows token verification. If peer verification is unavailable, `APPROVAL_INVALID` and no worker launch.

## Argument canonicalization
For accepted JSON inputs, reject duplicate keys, ill-formed UTF-8, non-finite values, JSON numbers outside the explicitly pinned application numeric profile and schemas allowing ambiguous coercion. Use a single canonical JSON representation with lexicographically sorted object keys, minimal escaping and a fixed numeric policy across SDK, CLI, gateway and tests. The concrete chosen serializer is RFC 8785 JSON Canonicalization Scheme (JCS), limited to I-JSON input, with cross-language compliance vectors mandatory before freeze. The gateway—not the client—computes `SHA256(UTF8(JCS(validated_arguments)))`; it must never hash unsanitized transport text while authorizing a separately parsed object. If JCS is unavailable/ambiguous or a payload cannot be losslessly represented, reject, never use an alternate hash. The package digest and argument digest are distinct domains.

## Pending request
A gateway-minted pending request contains immutable: random 128-bit nonce, UUID request ID, invocation intent ID, authenticated client UUID, tool ID+SemVer+immutable digest, canonical argument SHA-256, declared effect, exact capability IDs, owner-visible impact summary, UTC creation/expiration and policy revision. Expiration is <=300 seconds; use a monotonic deadline during running gateway lifecycle and revalidate UTC expiry after restart, never extend it automatically. Argument values that could expose secrets are not written to audit or approval history; impact summary is derived only from reviewed manifest/schema and explicitly vetted structured parameters.

Only one outstanding pending request for a given invocation intent is allowed. Creating the request atomically records its audit event. Client timeout does not mean the owner has approved it; cancellation or client disconnect marks the request terminal and never launches an orphaned call.

## Approval states and transition controls
`PENDING -> APPROVED | DENIED | EXPIRED | CANCELLED`; `APPROVED -> CONSUMED | INVALIDATED | EXPIRED`. Terminal states must never revert. A single request can be consumed at most once. Denial and expiry cannot be overturned by replay or restart; new intent requires a new request/nonce.

Owner administrative commands `approvals list`, `approvals show <id>`, `approvals approve <id> --nonce <nonce>`, and `approvals deny <id> --nonce <nonce>` use **only** admin IPC, not MCP. The CLI must display the exact client/tool/version/digest, effect, capabilities, canonical argument fingerprint and expiry prior to a separate explicit owner confirmation. Shell command-history exposure of non-secret nonces is still minimized: a safer interactive prompt is the default; the `--nonce` switch is permitted only on the trusted interactive administrative interface, not MCP. A malicious IPC peer is never allowed to set a claimed SID or inject arbitrary approved invocations.

The pipe server must validate the actual Windows peer access token by a correctly impersonated client authentication procedure, prohibit remote clients, restrict the DACL to the owner-authorized SID and service identity, and refuse anonymous/low-integrity/AppContainer worker principals. Token verification must be based on trusted OS APIs and tested on supported Windows 10 builds; a process PID alone does not authenticate. Owner token change or failed impersonation denies every mutation.

## Single-use atomic admission (C-005 dependent)
Within one SQLite `BEGIN IMMEDIATE` gateway writer transaction:
1. Read current client credential epoch, current invocation grant, tool trust+enabled status, immutable digest, resource policy, pending request, policy revision and expiry.
2. Confirm owner-approved state and equality to authenticated client, exact parsed argument digest, tool ID/version/digest, effect and capabilities.
3. Conditional update `APPROVED -> CONSUMED` only if consumed count = 0, nonce matches, expiration valid and policy revision unchanged; require exactly one changed row.
4. Insert one unique admitted invocation and one immutable audit event identifying successful consumption.
5. Commit only when all three writes succeed. Rollback otherwise; no worker may be spawned before the commit. Under writer contention, reject or bounded retry inside the admission deadline; never admit on stale cached approval.

Perform **another** grant, revocation, digest and confinement recheck just before spawn. If policy changed after admission, persist cancellation/audit and do not spawn; the approval remains consumed and cannot be reissued as a bypass. Worker side effects occur outside all SQLite transactions.

On crash before commit, approval remains unconsumed; if still unexpired, a new explicitly initiated attempt must recheck current policy. On crash after commit but before spawn, recovery records cancelled/ambiguous state and never blindly relaunches a non-idempotent invocation. On crash during external work, record `UNKNOWN_EFFECT` when the outcome is unverifiable.

## Denial matrix (planned)
| Test fixture | Input/incident | Expected |
|---|---|---|
| APR-001 | MCP client submits `approved:true` or approval ID in tool input | Ignored as authority; no execution |
| APR-002 | Wrong owner SID, worker AppContainer, spoofed PID or network pipe client | Admin IPC denied independently of MCP |
| APR-003 | Reuse consumed nonce/approval | 0 rows updated, no second dispatch |
| APR-004 | Change any argument value, tool digest, capability or client | Binding mismatch and denial |
| APR-005 | Owner approves after expiry or client cancellation | Terminal denial |
| APR-006 | Two concurrent consumers of one approval | Exactly one admission; other denial |
| APR-007 | Revoke grant between approval and admission or between admission and spawn | No worker starts after revocation |
| APR-008 | Crash between approval update, invocation insert and audit append | All or none durable |
| APR-009 | Malformed/duplicate JSON, I-JSON violation or divergent numeric canonicalization | Reject before approval creation |
| APR-010 | Restore older database where approval was already consumed or credential revoked | Recovery reconciliation denies replay; no resurrected authority |

The last case requires a separate, authoritative revocation watermark/recovery journal or restore reconciliation designed in C-005/C-008; **do not declare this control fully specified** until that mechanism is completed.

## Alternatives and residual risk
Rejected MCP-side approval prompts or tool parameters (self-approval), raw-payload hashing (parser inconsistencies), trusting owner username/PID (spoofing), recording approval outside admission transaction (replay races), and continuing headlessly on admin-interface outage (breaks default deny). A compromised owner Windows session remains an authority-compromise risk. Canonical JSON vectors, detailed pipe DACL/impersonation APIs, the SQLite DDL and recovery anti-rollback mechanism are **freeze blockers**, not implementation details that may be guessed in Phase 2. No tests have run.
