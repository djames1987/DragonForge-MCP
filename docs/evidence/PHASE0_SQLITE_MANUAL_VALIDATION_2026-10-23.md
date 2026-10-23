# Phase 0 — Owner-run SQLite migration validation evidence (2026-10-23)

**Evidence source:** owner-provided PowerShell console transcript, manual local run on Windows 10 Home 19045 x64. This document records user-provided results; the AI did not execute the commands or independently inspect the owner's host.

- Repository branch: `docs/phase0-architecture-baseline-20261021`
- Tested Git HEAD after fast-forward: `d8ba1cc163d8602401846eb7d54e005243cb3a6a`
- Document: `docs/contracts/sqlite-authoritative-v1.md`
- Runner classification: **MANUAL_LOCAL**, not SELF_HOSTED_ACTIONS; local runner was not used for this test.
- Runtime: Python 3.12's sqlite3, SQLite **3.42.0**, in-memory database `:memory:`.
- Migration variables substituted with disposable `test-instance`, fixed timestamp, `zeroblob(32)` checksum placeholder, `phase0-validation` app version. The placeholder checksum does **not** verify real migration checksumming.
- Before re-run: preceding manual test of earlier SHA failed `sqlite3.OperationalError: near "version_id": syntax error`; fixed by moving `version_id` before table constraints in `capability_grants`, committed as `d8ba1cc163d8602401846eb7d54e005243cb3a6a`.

## Owner-provided output
```text
SQLite version: 3.42.0
Integrity: ok
Foreign key violations: []
Schema version: 1
Application tables: 19
SQLITE MIGRATION: PASS
```

**Disposition:** PASS for *fresh database v1 DDL syntax, creation, pragma integrity check, no preexisting FK violations, application-table inventory and schema version*. This does NOT establish insert-time foreign-key enforcement, migration idempotency, real SHA256 migration checksum computation, committed WAL durability on disk, transaction fault recovery, checkpoint publication consistency, credential revocation, anti-rollback, or proof on a self-hosted runner. All remain separate Phase 0 contract or later implementation qualification matters. No executable test or GitHub workflow committed.
