# C-005C — Reconciled recovery durability, schema deltas and invariants
Status: SELECTED — AGENT-DELEGATED DOCUMENTATION; NOT EXECUTED OR FROZEN. Authority rules 1–26. Reconciles C-005A and C-005B; where inconsistent, this document defines the proposed migration and fail-closed rule. Does not assert protection against an adversary with full offline control of both DB and checkpoint.

## Decision and threat model
SQLite and an external file do not share a commit protocol. They cannot be atomically committed together by `BEGIN IMMEDIATE`. Therefore success is an **application-level two-stage publication barrier**, not a transactional two-file guarantee. Attackers: revoked clients, malicious workers, accidental operator restore, storage crash and compromised same-user process. Privileged owner/offline administrator may roll back *all* local data: no cryptographic local-only mechanism promises detection of a full-system rollback. Restore revocation safety therefore requires the gateway to enforce identity re-enrollment and approval/grant invalidation whenever continuity cannot be independently established. Design objective is fail-closed on uncertainty; it is not monotonicity against a fully controlling administrator.

## Corrected migration 0001 appendices
The C-005A DDL defines the base authority store and must incorporate the following **before architecture freeze**. These statements are documentation SQL, never run as Phase 0 implementation.

```sql
CREATE TABLE checkpoint_outbox (
  epoch INTEGER PRIMARY KEY CHECK (epoch >= 1),
  instance_id TEXT NOT NULL,
  authority_digest BLOB NOT NULL CHECK (length(authority_digest)=32),
  audit_hash BLOB NOT NULL CHECK (length(audit_hash)=32),
  state TEXT NOT NULL CHECK (state IN ('PENDING','PUBLISHED')),
  created_at TEXT NOT NULL,
  published_at TEXT,
  CHECK ((state='PUBLISHED') = (published_at IS NOT NULL))
) STRICT;
CREATE INDEX ix_checkpoint_pending ON checkpoint_outbox(state,epoch);

CREATE TABLE restore_authority (
  singleton INTEGER PRIMARY KEY CHECK(singleton=1),
  last_verified_epoch INTEGER NOT NULL CHECK(last_verified_epoch>=0),
  recovery_generation INTEGER NOT NULL CHECK(recovery_generation>=0),
  state TEXT NOT NULL CHECK(state IN ('LOCKED','VERIFIED','REKEY_REQUIRED')),
  checked_at TEXT
) STRICT;
INSERT INTO restore_authority(singleton,last_verified_epoch,recovery_generation,state)
VALUES(1,0,0,'LOCKED');

CREATE UNIQUE INDEX uq_credential_key_client ON credentials(key_id,client_id);
CREATE UNIQUE INDEX uq_invocation_client ON invocations(invocation_id,client_id);
```

C-005A's `capability_grants(grant_id,version_id)` must FK-reference the unique `client_tool_grants(grant_id,version_id)` and `(version_id,capability_id)` in `tool_capabilities` **together**, never capability_id alone. Any schema migration checksum is computed over exact UTF-8 migration bytes and stored with stable sequence; SQLite version >=3.37 required, foreign keys ON and explicitly verified on each connection. The full release DDL must be rendered as one ordered, validated source after the owner approves design; divergent duplicate snippets are not a frozen baseline.

## Durable authority publication protocol — CP-001
1. Gateway verifies private ACLs, full DB/WAL integrity, migration checksum, private checkpoint integrity, instance UUID, independently recoverable key and signed/audited epoch chain before READY. Never create an empty store in place of damaged authority.
2. Under the **single writer** `BEGIN IMMEDIATE`, perform security mutation + audit event + epoch increment + matching outbox record. Domain-separate the snapshot hash: SHA256('DFMCP-AUTHORITY-V1\\0' || instance_uuid_UTF8 || uint64_be(epoch) || audit_hash || normalized_authority_digest). Recheck expected row revisions. COMMIT with FULL synchronous durability.
3. Change gateway admission gate to `PUBLISHING`, rejecting all new protected operations. Build checkpoint envelope `{format:1,instance_id,epoch,authority_digest,audit_hash,previous_checkpoint_sha256,recovery_generation}`, canonical I-JSON, authenticate with HMAC-SHA256 under a DPAPI-protected distinct checkpoint key. Store in owner ACL directory on same volume: create unique file, write, FlushFileBuffers, same-volume MoveFileExW with MOVEFILE_REPLACE_EXISTING|MOVEFILE_WRITE_THROUGH, reopen and verify exact authenticated bytes. **No claim of perfect power-loss durability from a single Windows API**.
4. Reenter SQLite writer: require outbox record matches authenticated file epoch/hash, set PUBLISHED and published_at. Only now return mutation success and reopen admissions after all epochs <= current have a contiguous PUBLISHED chain.
5. If process crashes after DB COMMIT but before file publish: on startup validate chain and publish pending epoch in order, without replaying semantic authority mutations. If crash after file replace but before PUBLISHED: compare bytes and update the matching row only. If checkpoint ahead of SQLite: **RECOVERY_REQUIRED**; never reconstruct grants from checkpoint. Any hash mismatch or noncontiguous epochs -> LOCKED, owner recovery. Do not return ALLOW to a waiting request during PUBLISHING.
6. Audit access failures: if any mandatory audit write fails, rollback mutation and deny. An already committed security mutation with publication failure remains committed but service is locked; command returns uncertain/recovery-required, never false success.
7. `audit_events` has a hash chain but is only *logically append-only* under gateway control; privileged DB tampering is out of scope of absolute prevention.

## Restore and rollback classification — CP-002
- Same instance + exact matching backup epoch and independently verified checkpoint: restore only after schema/checksum, key/AEAD, audit hash, foreign keys and corrupt-file checks. Do not assume equivalent timestamp means equivalent content.
- Backup epoch < checkpoint epoch, checkpoint missing/unrecoverable, or mismatching checkpoint: remain offline in REKEY_REQUIRED. Re-enrollment is mandatory: invalidate all previous bearer credentials, outstanding approvals, capability handles and grants in a **new** authority generation; rotate checkpoint/pepper keys; require explicit owner-trusted re-grants and retrust according to current package hashes. Preserve old records as inert audit history. Never auto-approve or auto-grant restored records.
- Backup epoch > checkpoint epoch: refuse activation pending owner-reviewed forensic reconciliation; cannot infer whether backup or checkpoint is corrupted.
- Different instance UUID or incompatible schema: refuse as foreign backup until a separately specified import/migration process is authorized; never treat as normal restore.
- During restore prepare new private directory, verify entirely offline, use atomic rename at the same-volume boundary, preserve previous store and remain LOCKED through health verification; uncertain cutover => revert while offline.
- Restore of revoked/expired credentials can never make them active simply because a backup says they were previously valid.
- Lost independent key and encrypted backup must return explicit unrecoverable/re-enroll status; no silent reset.

## Migration ordering
0001 must be exact schema of C-005A **plus above checkpoint structures**. 0002+ only with unique migration ID and checksums, compatibility table and full release plan. Acquire offline maintenance lock, verified backup, quiesce gateway and worker, verify source migration chain, execute migration DDL+schema_migrations+PRAGMA user_version in one SQLite transaction, complete integrity/foreign-key checks, publish new checkpoint state before READY. If rollback cannot preserve revocations, full rekey is mandatory. Never perform live downmigration on a newer unknown schema.

## Required acceptance fixtures (planned only)
CP-T01 kill at each write/COMMIT/outbox checkpoint stage; CP-T02 duplicate/resumed publication; CP-T03 publish file ahead/behind DB; CP-T04 restore backup before credential revocation or approval consumption; CP-T05 tamper checkpoint/DB; CP-T06 power loss during WAL; CP-T07 concurrent owner approvals and grant changes; CP-T08 loss of pepper/export key; CP-T09 restore foreign instance; CP-T10 unknown schema/downgrade. Expected: no unauthorized invocation begins, all mismatch cases lock, all known prior revocations remain ineffective after rekey, and terminal UNKNOWN_EFFECT never silently replayed. Record exact SHA, Windows build, SQLite version, owner/worker identity, raw failures, audit and evidence hashes.

## Limitations and freeze dependencies
No application code was executed, no SQL migrations tested. Exact power-failure durability and anti-rollback claims need actual Windows verification; a full-system attacker able to roll back every local artifact is not preventable by local checkpoint alone. Separate backup key-recovery cryptography and 54 fully machine-validatable CLI schemas remain final freeze dependencies. This document removes claims of atomic SQLite+filesystem COMMIT and prohibits accidental owner privilege amplification.
