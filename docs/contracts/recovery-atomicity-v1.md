# C-005B / C-008 — Crash-safe authority and restore contract
Status: SELECTED — AGENT-DELEGATED; Phase 0 design only. Depends on C-005A SQL schema, C-006A approvals and DEC-005. Never claim that Windows filesystem durability or anti-rollback is verified until local evidence exists.

## Fundamental limit
No independent writable local file and SQLite COMMIT can form an atomic cross-resource transaction without a shared transactional coordinator. DPAPI encrypts keys but does not provide a trustworthy monotonic counter against a machine administrator restoring an older snapshot. **Accordingly the initial local-only release SHALL fail closed on a checkpoint mismatch; it SHALL NOT promise prevention of rollback by a privileged owner or offline disk replacement.** Explicit owner recovery requires new client enrollments and approval invalidation before admitting calls. This resolves the former ambiguous guarantee honestly.

## Authority epoch and durable outbox
One SQLite transaction contains (a) protected authority mutation; (b) mandatory audit event; (c) increment of store_metadata.authority_epoch/policy_epoch; and (d) a new checkpoint_outbox row representing the new cumulative authority state. Amend migration v1 BEFORE FREEZE with:

```sql
CREATE TABLE checkpoint_outbox (
 epoch INTEGER PRIMARY KEY CHECK(epoch > 0),
 instance_id TEXT NOT NULL,
 payload_sha256 BLOB NOT NULL CHECK(length(payload_sha256)=32),
 audit_hash BLOB NOT NULL CHECK(length(audit_hash)=32),
 state TEXT NOT NULL CHECK(state IN ('PENDING','PUBLISHED')),
 created_at TEXT NOT NULL,
 published_at TEXT
) STRICT;
CREATE TABLE restore_authority (
 singleton INTEGER PRIMARY KEY CHECK(singleton=1),
 max_observed_epoch INTEGER NOT NULL CHECK(max_observed_epoch>=0),
 reconciled_at TEXT,
 state TEXT NOT NULL CHECK(state IN ('LOCKED','VERIFIED','REKEY_REQUIRED'))
) STRICT;
```

The gateway never reports a security mutation successful or begins new protected work until its committed outbox epoch is written to a separate owner-ACL-protected, DPAPI-authenticated checkpoint file using temporary creation, flush, and same-volume replace with post-write verification. Failure at any step places the gateway in RECOVERY_REQUIRED. The outbox remains durable and can be replayed idempotently by epoch. Only a published contiguous epoch chain may enable READY; missing, repeated with different hash, noncontiguous, or reverted epochs lock the gateway. Always recover first from SQLite, then reconcile the external checkpoint; never promote the external file to unilateral grant authority.

## Crash matrix
- Before SQLite commit: no authority mutation, audit, checkpoint outbox or approval consumption; discard pending temporary checkpoint.
- After SQLite commit / before checkpoint publish: outbox PENDING; lock admission; republish after integrity checks; do not falsely return success to client.
- After checkpoint replacement / before PUBLISHED flag: verify checkpoint hash and epoch, then idempotently mark PUBLISHED.
- After publish: return success only after gateway confirms DB epoch and file match.
- On crash in RUNNING external tool: mark UNKNOWN_EFFECT, never retry non-idempotent calls without owner review.
- On competing writers: BEGIN IMMEDIATE plus conditional revision/epoch updates; busy timeout then STORE_UNAVAILABLE, never stale trust.
- During WAL checkpoint: recovery relies on committed WAL; do not raw-copy an open database.

## Restore contract
Gateway goes offline and acquires exclusive lock; create verified pre-restore backup; verify backup AEAD, SHA256, manifest instance ID, SQLite integrity/foreign keys, schema migration checksums, audit sequence and key possession. Compare restored epoch against the separately retained checkpoint and available revocation/consumption tombstones. If backup epoch is older, enter REKEY_REQUIRED; permanently invalidate all existing credentials, approvals and grants, increment generation and enroll clients anew by authenticated owner action. There is no 'keep old grants' switch. A same-epoch mismatch is tamper/corruption → refuse restore. Snapshot promotion must be atomic on the same volume and preserve rollback copy until health verification; failure reverts to prior validated files while staying offline. A wholly lost checkpoint likewise enters owner-confirmed recovery with complete re-enrollment. Data loss up to 24h and RTO target 30 minutes are targets, not guarantees; interrupted key recovery must never produce an empty trusted store.

## Ordered migrations
0001 base authority schema and outbox additions; 0002 future indexed changes only after 0001 checksum verification. Each migration runs in exclusive maintenance state, with tested encrypted backup, transactional DDL, new schema_migrations entry and user_version in one commit, full integrity check before reopening. Unknown newer schema => refuse. Incompatible downgrade => restore prior compatible backup only with epoch reconciliation and owner-controlled rekey when required. Never modify production migrations in Phase 0.

## Acceptance
RC-01 fault after every SQL write and external checkpoint stage; RC-02 corrupt/missing checkpoint; RC-03 restore old backup following revocation/approval use; RC-04 concurrent writers/CAS; RC-05 WAL crash and restart; RC-06 key loss; RC-07 migration failure and rollback. Each must prove no unaudited security mutation is admitted; stale restoration never resurrects access. Negative recovery evidence is future MANUAL_LOCAL or SELF_HOSTED_ACTIONS, not executed here.
