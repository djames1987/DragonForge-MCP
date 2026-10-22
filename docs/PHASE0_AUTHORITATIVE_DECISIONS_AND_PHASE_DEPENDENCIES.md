# Authoritative decision precedence and Phase 1–5 dependency plan — Phase 0
**Selected architecture planning, not owner-approved freeze or implementation authority.**

## Decision precedence
1. Binding `docs/DEVELOPMENT_RULES.md` rules 1–26; owner README product constraints.
2. DEC-001 owner selected: MCP 2026-07-28 only, no legacy/automatic fallback. DEC-006 owner selected: mandatory SHA-256 exact-byte pin plus explicit owner trust; no mandatory publisher signature.
3. DEC-002 agent delegated: owner-enrolled independently authenticated per-client credentials, verifier/HMAC/DPAPI, 90-day expiry, <=24h explicit overlap; browser/MCP clientInfo is NOT an identity.
4. DEC-003 agent delegated: owner SID/token-verified protected named pipe and noninteractive fail-closed sensitive approvals bound to exact arguments and package digest.
5. DEC-004 agent delegated: Windows AppContainer W0/W1 OS isolation subject to build-specific proof. W2 network and W3 admin not supported.
6. DEC-005 agent delegated: gateway-only SQLite WAL authoritative writer, 90-day audit metadata, daily encrypted backups with 30 recovery points, weekly clean-restoration qualification, checkpoint publication barrier and fail-closed full rekey after stale restore.
All previous `OPEN/PROPOSED` language in DECISIONS.md referring to the **selection of DEC-002..005** is superseded by this delegated ledger but their **actual qualification/documentation defects remain open**. No new owner approval claimed.

## Binding technical contract precedence for agents
- Database: `contracts/sqlite-authoritative-v1.md` supersedes earlier additive C-005A/B/C DDL fragments (those are history); implementation must use one approved migration, not merge snippets.
- CLI: `administrative-cli.md` syntax + `cli-command-schema-registry-v1.json` outer IPC + `cli-security-key-formats-v1.md` security; in case of contradiction, fail closed, file freeze defect, do not invent an unsafe permissive behavior.
- MCP: authoritative versioned 2026-07-28 source and `mcp-2026-07-28-conformance-addendum.md` over earlier hand-authored examples. Unknown cases require source verification, not guesswork.
- Windows: `windows-enforcement-profile-v1.md` narrow candidate support + `windows-enforcement-v1.md` API plan + `windows-confinement-qualification.md` test suite.
- Operations: `OPERATIONS_QUALIFICATION_PHASES_V1.md` + `QUALIFICATION_EXECUTION_CONTRACT_V1.md`; future runner commands are proposals, not implemented commands.

## Phase 1—5 task dependencies and exit checks
| Phase | Ordered tasks | Dependency | Exit gate |
|---|---|---|---|
| 1 | 1.0 exact-SHA pre-phase rules/ADR/drift audit; 1.1 private secure config and owner storage; 1.2 SQLite 0001 authority schema+checksum+recovery lock; 1.3 owner client enrollment and credential verification; 1.4 loopback HTTP Host/Origin validation; 1.5 MCP 2026-07-28 C-010; 1.6 STDIO enrolled bridge; 1.7 grant-scoped discovery and admission; 1.8 redacted audit and harmless gateway built-in | formal Phase 0 owner freeze+Phase 1 authorization | T-001..006,T-021,T-026 exact SHA PASS; no privileged third-party workers |
| 2 | 2.0 prior-phase audit; 2.1 package SDK+manifest; 2.2 immutable quarantine/digest; 2.3 owner CLI stage/validate/trust/enable/grant; 2.4 W0/W1 AppContainer launcher with token/Job/pipe proof; 2.5 broker capabilities and verified loader; 2.6 approval challenge/atomic consumption; 2.7 native/C++/Rust IPC compatibility; 2.8 cancellation/timeout | Phase 1 qualified; Windows build-specific OS security proof | T-007..013,T-020,T-022 + WQ-001..012 exact SHA PASS |
| 3 | 3.0 prior-phase audit; 3.1 narrow brokered file tools; 3.2 read-only diagnostics; 3.3 client adapter compatibility; 3.4 two disjoint identities concurrency; 3.5 revocation and cancellation races | Phase 2 qualified and no sandbox bypass | T-014,T-015 plus client interoperability and scope isolation matrix PASS |
| 4 | 4.0 prior-phase audit; 4.1 deterministic fault points; 4.2 backup and restore; 4.3 checkpoint recovery; 4.4 audit redaction and retention; 4.5 reference performance fixture (100 registered tools, 4 simultaneous workers); 4.6 controlled tuning and repeat regressions | Phase 3 qualified | T-016,T-017,T-023,T-025; discovery p95 <=250ms, idle WS <=200MiB, RTO <=30m, RPO <=24h under recorded Windows host PASS |
| 5 | 5.0 prior-phase audit; 5.1 reproducible packaged release; 5.2 clean Windows installation and on-demand launch; 5.3 upgrade schema and key preservation; 5.4 rollback stale restore and rekey; 5.5 uninstall preserve/purge; 5.6 full CLI-001..054 schema and UX qualification; 5.7 full exact-SHA regression; 5.8 final owner release decision | Phase 4 qualified | T-018,T-019,T-024 plus every prior T regression and clean machine proof PASS |

## Runner restriction and evidence
All automation jobs later target explicitly labeled owner-controlled Windows self-hosted runners; no GitHub-hosted fallback. Until then owner may run later *implemented* fixtures manually and archive os/build/runner type, exact SHA, environment, commands, logs and artifact hashes. Phase 0 documentation does not run tests. No transition to next phase on an unqualified gate without written owner exception.

## Deployment security
Only loopback listener; validate Host and Origin prior to MCP dispatch. Install current-user private directory ACLs, verify DB integrity, no silent recreate. Backup AES-256-GCM with independent recovery key export and restore drills; forced shutdown stops admission before drain, hard crash follows UNKNOWN_EFFECT. Never silently downgrade schema or revive grants after stale backup restore. Keep Windows platform operations behind explicit identity, package loader, ACL, process, pipe and key-store interfaces for future ports.

## Architecture freeze blockers
Mechanical SQLite parse verification, complete deep CLI schema definitions, selected Windows build OS evidence design proof, immutable upstream MCP source hash and conformance fixture pin, exact per-test deterministic qualification fixture implementation contract, full cross-file editorial contradiction repair. No owner freeze without all documentation gaps closed and independent audit.
