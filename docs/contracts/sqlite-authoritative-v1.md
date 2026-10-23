# C-005 — Authoritative SQLite v1 schema and recovery specification
**Phase 0 documentation only; not executed.** This file consolidates C-005A, C-005B and C-005C as the **single proposed normative version 1**. Earlier C-005A/B/C are superseded where they conflict and retained for design history. Governing rules 1–26, owner DEC-001/006, delegated DEC-002..005. SQLite minimum 3.37.0. This is SQL documentation, NOT migration code.
 
## Startup and migration contract
On a brand-new instance explicitly provisioned by the owner, initialize via private owner-only storage. On an existing store never automatically regenerate tables, even when SQLite cannot open it. Use one authoritative writer and one instance-wide process lock. Every connection sets and verifies `foreign_keys=ON`, `trusted_schema=OFF`, busy timeout; the writer verifies WAL and `synchronous=FULL`. `schema_migrations.checksum` is SHA-256 of the exact immutable UTF-8 canonical migration SQL bytes supplied with release, excluding substitutions: migration variables are bound arguments, not text substitution. `PRAGMA user_version` and `schema_migrations` must match. Unknown newer user_version, altered checksum, failed integrity/foreign key check => RECOVERY_REQUIRED, not reset.
 
## Single ordered migration 0001 — definitive proposed source
The following ordered statements, without the surrounding Markdown fence, are the proposed migration payload. No other v1 DDL fragment is additive at execution time.
```sql
PRAGMA foreign_keys=ON;
PRAGMA trusted_schema=OFF;
PRAGMA busy_timeout=5000;
PRAGMA journal_mode=WAL;
PRAGMA synchronous=FULL;
BEGIN IMMEDIATE;
CREATE TABLE schema_migrations (
  migration_id INTEGER PRIMARY KEY CHECK(migration_id>0),
  checksum BLOB NOT NULL CHECK(length(checksum)=32),
  applied_at TEXT NOT NULL,
  app_version TEXT NOT NULL
) STRICT;
CREATE TABLE store_metadata (
  singleton INTEGER PRIMARY KEY CHECK(singleton=1),
  instance_id TEXT NOT NULL UNIQUE,
  schema_version INTEGER NOT NULL CHECK(schema_version>=1),
  authority_epoch INTEGER NOT NULL CHECK(authority_epoch>=0),
  policy_epoch INTEGER NOT NULL CHECK(policy_epoch>=0),
  recovery_generation INTEGER NOT NULL CHECK(recovery_generation>=0),
  created_at TEXT NOT NULL,
  last_reconciled_at TEXT
) STRICT;
CREATE TABLE clients (
  client_id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL UNIQUE CHECK(length(display_name) BETWEEN 1 AND 128),
  state TEXT NOT NULL CHECK(state IN ('ACTIVE','DISABLED','REVOKED')),
  created_at TEXT NOT NULL,
  revoked_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1 CHECK(revision>=1),
  CHECK((state='REVOKED')=(revoked_at IS NOT NULL))
) STRICT;
CREATE TABLE credentials (
  credential_id TEXT PRIMARY KEY,
  client_id TEXT NOT NULL REFERENCES clients(client_id) ON DELETE RESTRICT,
  key_id TEXT NOT NULL UNIQUE,
  verifier BLOB NOT NULL CHECK(length(verifier)=32),
  pepper_key_id TEXT NOT NULL,
  issued_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  revoked_at TEXT,
  rotation_parent_id TEXT REFERENCES credentials(credential_id) ON DELETE RESTRICT,
  revision INTEGER NOT NULL DEFAULT 1,
  CHECK(expires_at>issued_at)
) STRICT;
CREATE INDEX ix_credentials_client ON credentials(client_id,revoked_at,expires_at);
CREATE TABLE tool_packages (
  package_id TEXT PRIMARY KEY,
  digest BLOB NOT NULL UNIQUE CHECK(length(digest)=32),
  digest_algorithm TEXT NOT NULL CHECK(digest_algorithm='DFMCP-PACKAGE-V1-SHA256'),
  relative_store_path TEXT NOT NULL UNIQUE,
  state TEXT NOT NULL CHECK(state IN ('STAGED','VALIDATED','TRUSTED','ENABLED','DISABLED','REVOKED')),
  created_at TEXT NOT NULL,
  trusted_at TEXT,
  revoked_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1
) STRICT;
CREATE TABLE tool_versions (
  version_id TEXT PRIMARY KEY,
  package_id TEXT NOT NULL UNIQUE REFERENCES tool_packages(package_id) ON DELETE RESTRICT,
  tool_id TEXT NOT NULL,
  semver TEXT NOT NULL,
  entry_kind TEXT NOT NULL CHECK(entry_kind IN ('python','native')),
  entry_point TEXT NOT NULL,
  effect TEXT NOT NULL CHECK(effect IN ('read','write','external')),
  manifest_json TEXT NOT NULL,
  input_schema_json TEXT NOT NULL,
  output_schema_json TEXT NOT NULL,
  timeout_ms INTEGER NOT NULL CHECK(timeout_ms BETWEEN 1 AND 300000),
  max_input_bytes INTEGER NOT NULL CHECK(max_input_bytes BETWEEN 1 AND 1048576),
  max_output_bytes INTEGER NOT NULL CHECK(max_output_bytes BETWEEN 1 AND 4194304),
  max_parallel INTEGER NOT NULL CHECK(max_parallel BETWEEN 1 AND 4),
  idempotency TEXT NOT NULL CHECK(idempotency IN ('none','keyed')),
  UNIQUE(tool_id,semver,package_id)
) STRICT;
CREATE INDEX ix_tool_versions_name ON tool_versions(tool_id,semver);
CREATE TABLE tool_capabilities (
  version_id TEXT NOT NULL REFERENCES tool_versions(version_id) ON DELETE RESTRICT,
  capability_id TEXT NOT NULL,
  PRIMARY KEY(version_id,capability_id)
) STRICT, WITHOUT ROWID;
CREATE TABLE client_tool_grants (
  grant_id TEXT PRIMARY KEY,
  client_id TEXT NOT NULL REFERENCES clients(client_id) ON DELETE RESTRICT,
  version_id TEXT NOT NULL REFERENCES tool_versions(version_id) ON DELETE RESTRICT,
  grant_kind TEXT NOT NULL CHECK(grant_kind IN ('DISCOVER','INVOKE')),
  state TEXT NOT NULL CHECK(state IN ('ACTIVE','REVOKED')),
  granted_at TEXT NOT NULL,
  revoked_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1,
  CHECK((state='REVOKED')=(revoked_at IS NOT NULL))
) STRICT;
CREATE UNIQUE INDEX uq_active_client_grant ON client_tool_grants(client_id,version_id,grant_kind) WHERE state='ACTIVE';
CREATE UNIQUE INDEX uq_grant_version ON client_tool_grants(grant_id,version_id);
CREATE INDEX ix_grants_client ON client_tool_grants(client_id,state,grant_kind);
CREATE TABLE capability_grants (
  grant_id TEXT NOT NULL REFERENCES client_tool_grants(grant_id) ON DELETE RESTRICT,
  capability_id TEXT NOT NULL,
  scope_jcs TEXT NOT NULL,
  version_id TEXT NOT NULL,
  PRIMARY KEY(grant_id,capability_id),
  FOREIGN KEY(version_id,capability_id) REFERENCES tool_capabilities(version_id,capability_id),
  FOREIGN KEY(grant_id,version_id) REFERENCES client_tool_grants(grant_id,version_id)
) STRICT, WITHOUT ROWID;
CREATE TABLE pending_approvals (
  approval_id TEXT PRIMARY KEY,
  intent_id TEXT NOT NULL UNIQUE,
  nonce BLOB NOT NULL UNIQUE CHECK(length(nonce)=32),
  client_id TEXT NOT NULL REFERENCES clients(client_id) ON DELETE RESTRICT,
  version_id TEXT NOT NULL REFERENCES tool_versions(version_id) ON DELETE RESTRICT,
  argument_sha256 BLOB NOT NULL CHECK(length(argument_sha256)=32),
  capabilities_jcs TEXT NOT NULL,
  effect TEXT NOT NULL CHECK(effect IN ('write','external')),
  policy_epoch INTEGER NOT NULL,
  state TEXT NOT NULL CHECK(state IN ('PENDING','APPROVED','DENIED','EXPIRED','CANCELLED','CONSUMED','INVALIDATED')),
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  owner_sid TEXT,
  decided_at TEXT,
  consumed_at TEXT,
  revision INTEGER NOT NULL DEFAULT 1,
  CHECK(expires_at>created_at)
) STRICT;
CREATE INDEX ix_approvals_status ON pending_approvals(state,expires_at);
CREATE TABLE invocations (
  invocation_id TEXT PRIMARY KEY,
  intent_id TEXT NOT NULL UNIQUE,
  client_id TEXT NOT NULL REFERENCES clients(client_id) ON DELETE RESTRICT,
  version_id TEXT NOT NULL REFERENCES tool_versions(version_id) ON DELETE RESTRICT,
  approval_id TEXT UNIQUE REFERENCES pending_approvals(approval_id) ON DELETE RESTRICT,
  argument_sha256 BLOB NOT NULL CHECK(length(argument_sha256)=32),
  state TEXT NOT NULL CHECK(state IN ('ADMITTED','QUEUED','RUNNING','SUCCEEDED','TOOL_FAILED','CANCELLED','TIMED_OUT','CRASHED','UNKNOWN_EFFECT')),
  created_at TEXT NOT NULL,
  started_at TEXT,
  finished_at TEXT,
  predecessor_id TEXT REFERENCES invocations(invocation_id) ON DELETE RESTRICT,
  correlation_id TEXT NOT NULL UNIQUE,
  revision INTEGER NOT NULL DEFAULT 1
) STRICT;
CREATE INDEX ix_invocations_client ON invocations(client_id,created_at);
CREATE INDEX ix_invocations_state ON invocations(state,created_at);
CREATE TABLE idempotency_records (
  client_id TEXT NOT NULL REFERENCES clients(client_id) ON DELETE RESTRICT,
  version_id TEXT NOT NULL REFERENCES tool_versions(version_id) ON DELETE RESTRICT,
  idempotency_key TEXT NOT NULL CHECK(length(idempotency_key) BETWEEN 1 AND 128),
  argument_sha256 BLOB NOT NULL CHECK(length(argument_sha256)=32),
  invocation_id TEXT NOT NULL UNIQUE REFERENCES invocations(invocation_id) ON DELETE RESTRICT,
  created_at TEXT NOT NULL,
  PRIMARY KEY(client_id,version_id,idempotency_key)
) STRICT, WITHOUT ROWID;
CREATE TABLE audit_events (
  audit_seq INTEGER PRIMARY KEY AUTOINCREMENT,
  event_id TEXT NOT NULL UNIQUE,
  utc_time TEXT NOT NULL,
  actor_kind TEXT NOT NULL CHECK(actor_kind IN ('CLIENT','OWNER','GATEWAY','ANONYMOUS')),
  actor_id TEXT,
  operation TEXT NOT NULL,
  subject_id TEXT,
  correlation_id TEXT,
  outcome TEXT NOT NULL CHECK(outcome IN ('ALLOW','DENY','ERROR','UNKNOWN')),
  safe_code TEXT NOT NULL,
  detail_sha256 BLOB CHECK(detail_sha256 IS NULL OR length(detail_sha256)=32),
  prior_hash BLOB CHECK(prior_hash IS NULL OR length(prior_hash)=32),
  event_hash BLOB NOT NULL CHECK(length(event_hash)=32)
) STRICT;
CREATE INDEX ix_audit_time ON audit_events(utc_time);
CREATE INDEX ix_audit_correlation ON audit_events(correlation_id);
CREATE TABLE backup_records (
  backup_id TEXT PRIMARY KEY,
  generation INTEGER NOT NULL UNIQUE CHECK(generation>=1),
  authority_epoch INTEGER NOT NULL,
  schema_version INTEGER NOT NULL,
  artifact_sha256 BLOB NOT NULL CHECK(length(artifact_sha256)=32),
  recovery_key_id TEXT NOT NULL,
  created_at TEXT NOT NULL,
  verified_at TEXT,
  restore_tested_at TEXT,
  state TEXT NOT NULL CHECK(state IN ('CREATED','VERIFIED','INVALID','EXPIRED'))
) STRICT;
CREATE TABLE recovery_tombstones (
  tombstone_id TEXT PRIMARY KEY,
  entity_kind TEXT NOT NULL CHECK(entity_kind IN ('CLIENT','CREDENTIAL','GRANT','PACKAGE')),
  entity_id TEXT NOT NULL,
  authority_epoch INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(entity_kind,entity_id,authority_epoch)
) STRICT;
CREATE INDEX ix_tombstones_entity ON recovery_tombstones(entity_kind,entity_id);
CREATE TABLE recovery_operations (
  operation_id TEXT PRIMARY KEY,
  operation_kind TEXT NOT NULL CHECK(operation_kind IN ('BACKUP','RESTORE','MIGRATE','RECONCILE')),
  state TEXT NOT NULL CHECK(state IN ('STARTED','COMMITTED','FAILED')),
  started_at TEXT NOT NULL,
  finished_at TEXT,
  detail_sha256 BLOB CHECK(detail_sha256 IS NULL OR length(detail_sha256)=32)
) STRICT;
CREATE TABLE config_revisions (
  revision_id INTEGER PRIMARY KEY AUTOINCREMENT,
  config_sha256 BLOB NOT NULL CHECK(length(config_sha256)=32),
  created_at TEXT NOT NULL,
  owner_sid TEXT NOT NULL
) STRICT;
INSERT INTO store_metadata(singleton,instance_id,schema_version,authority_epoch,policy_epoch,recovery_generation,created_at)
VALUES(1, :instance_uuid,1,0,0,0,:utc_now);
CREATE TABLE checkpoint_outbox (
 epoch INTEGER PRIMARY KEY CHECK(epoch>0),
 instance_id TEXT NOT NULL,
 authority_digest BLOB NOT NULL CHECK(length(authority_digest)=32),
 audit_hash BLOB NOT NULL CHECK(length(audit_hash)=32),
 state TEXT NOT NULL CHECK(state IN ('PENDING','PUBLISHED')),
 created_at TEXT NOT NULL,
 published_at TEXT,
 CHECK ((state='PUBLISHED')=(published_at IS NOT NULL))
) STRICT;
CREATE INDEX ix_checkpoint_outbox_state ON checkpoint_outbox(state,epoch);
CREATE TABLE restore_authority (
 singleton INTEGER PRIMARY KEY CHECK(singleton=1),
 last_verified_epoch INTEGER NOT NULL CHECK(last_verified_epoch>=0),
 recovery_generation INTEGER NOT NULL CHECK(recovery_generation>=0),
 state TEXT NOT NULL CHECK(state IN ('LOCKED','VERIFIED','REKEY_REQUIRED')),
 checked_at TEXT
) STRICT;
INSERT INTO restore_authority(singleton,last_verified_epoch,recovery_generation,state)
VALUES (1,0,0,'LOCKED');
INSERT INTO schema_migrations(migration_id,checksum,applied_at,app_version)
VALUES(1,:migration_checksum,:utc_now,:app_version);
PRAGMA user_version=1;
COMMIT;
```
### Non-SQL integrity checks
1. `client_tool_grants` active grants must not outlive disabled/revoked principal/package/version. Enforce via gateway transaction and startup reconciliation; SQL foreign keys alone do not enforce policy.
2. `capability_grants` composite foreign keys must bind (grant_id,version_id) and (version_id,capability_id), never capability_id globally.
3. Security writes **always** include the authoritative mutation, redacted audit event, epoch increment and outbox insertion in one `BEGIN IMMEDIATE` transaction. Unpublished epochs disable protected admission.
4. Package byte digest, JSON Schema, owner identity and permission scope are verified before and again at invocation prelaunch; no tool processes see database, WAL/SHM or checkpoint files.
5. Audit's event_hash is a gateway-computed chained value over canonical redacted event fields and previous hash. No worker-supplied material is trusted.

## Transaction catalog
- TX01 client enrollment, credential insertion, audit, epoch, outbox; credential delivery failure -> subsequent audited revoke (never print secret in JSON).
- TX02 credential rotation/revocation, tombstone, policy epoch, audit, outbox. Rotation overlap at most 24 hours, issuance life 90 days.
- TX03 stage/validate/trust/enable/disable/revoke package by expected revision; fresh byte check and audit; privilege transitions bump epoch.
- TX04 client grant add/revoke by expected revision, scope subset recheck, tombstone on revoke, bump epoch.
- TX05 approval create/decide bound to principal+exact version/digest+argument SHA+scope+expiry+policy epoch and owner OS identity.
- TX06 admission consumes approved one-time nonce, inserts invocation plus optional unique idempotency record and audit in same transaction. No network call while writer lock is held.
- TX07 state CAS and terminal audit; interrupted RUNNING non-idempotent external effects => UNKNOWN_EFFECT and NEVER retry blindly.
- TX08 Online Backup API captures committed WAL-consistent snapshot; authenticated encryption performed outside live DB; metadata inserted with audit and epoch if change is authority-significant.
- TX09 offline restore classifies checkpoint/DB epoch: exact match permitted after integrity checks, stale/absent proof requires REKEY_REQUIRED, ahead/mismatched proof stays LOCKED; never infer safe from file timestamp.
- TX10 audit retention compaction after validated backup (90-day proposed metadata policy), preserve revocations and authority checkpoints, never delete legal preservation records early.
- TX11 migrations require maintenance lock, validated backup, transactional DDL+schema version/checksum, postmigration integrity then epoch publication; never down-migrate blindly.
- TX12 startup reconciliation replays only pending **checkpoint publication**, not tool side effects; stalled queued/running work becomes cancelled/UNKNOWN_EFFECT under audit.
 
## Cross-resource checkpoint publication
SQLite and the filesystem have no common atomic COMMIT. Security mutation+audit+outbox COMMIT precedes owner-ACL+DPAPI-authenticated checkpoint file publication. While any epoch is unpublished gateway remains `PUBLISHING`/not READY. File publication: temporary file on same volume, flush, same-volume atomic replace, flush/verify reopened canonical bytes; then mark matching outbox PUBLISHED in SQLite. On crash, reconcile the exact epoch chain and hashes; DB ahead -> republish matching committed outbox while LOCKED; file ahead or fork -> LOCKED and owner investigation. Mutation is not reported successful until durable verification and PUBLISHED. No claim this defeats full local disk rollback by privileged adversary.
 
## Restore invariants
Backup ciphertext AES-256-GCM with per-record unique 96-bit random nonce and associated data including instance UUID, schema, generation, authority epoch and backup UUID. Never restore live or without key+AEAD+SHA+SQLite integrity+FK+migration chain+audit verification. If backup is older than independently observed authority state, **invalidate all old credentials, approvals, grants, handles, and pending executions**; owner must re-enroll/re-grant/retrust before READY and rotate recovery generation/key. If all independently protected material was rolled back together, local-only offline adversary rollback cannot be reliably detected; explicitly document limitation, never assert tamperproof monotonic counter. Lost checkpoint or key => offline owner-confirmed recovery; foreign instance/schema => reject.
 
## Acceptance contract (future phases only)
Q-DB-01 exact migration on clean SQLite 3.37+ with `integrity_check=ok`, `foreign_key_check` empty and `user_version`/migration checksum match; Q-DB-02 writer concurrency; Q-DB-03 injected failure before/after each SQL statement; Q-DB-04 approval/idempotency races; Q-DB-05 all crash boundaries before and after checkpoint publication; Q-DB-06 WAL backup clean restore; Q-DB-07 stale authority/epoch restore; Q-DB-08 audit redaction/compaction. Windows file publication power-loss behavior requires independent OS qualification. None executed in Phase 0.
