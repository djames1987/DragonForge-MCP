# Phase 0 — end-to-end requirements traceability
**Status:** documentary crosswalk COMPLETE for R-001..R-026; qualification is PLANNED, not performed. Authority: rules 1–26; current requirements, contracts, Phase 1–5 plan. The table is a mapping, not evidence that a contract is fully finalized. C-002A = `docs/contracts/package-integrity-and-lifecycle.md`; C-005A = `docs/contracts/sqlite-authority-schema.md`; C-006A = `docs/contracts/owner-approval-and-admission.md`; C-007A = `docs/contracts/administrative-cli.md`; WIN-QUAL-001 = `docs/security/windows-confinement-qualification.md`. C-010 = MCP protocol compatibility.

## Requirement → component → contract → implementation phase → qualification → acceptance
| Requirement | Architectural component / owner | Contract/specification | Target | Qualification | Observable acceptance condition | Evidence |
|---|---|---|---|---|---|---|
| R-001 | G:Loopback transport | C-010 | Phase 1 | T-001 | Only loopback listener; non-loopback rejected | PLANNED |
| R-002 | SEC:Identity | C-001/C-005A | Phase 1 | T-002 | Unknown/revoked clients neither discover nor invoke | PLANNED |
| R-003 | SEC:Identity | C-001/C-005A | Phase 1 | T-003 | Forged metadata ignored; credential rotation and revocation enforced | PLANNED |
| R-004 | G:MCP dispatcher | C-010 | Phase 1 | T-004 | Exactly selected 2026-07-28 protocol revision | PLANNED |
| R-005 | G:Policy admission | C-001/C-005A/C-006A | Phase 1 | T-005 | Fresh grant/epoch checked at admission and prelaunch | PLANNED |
| R-006 | P:SQLite writer | C-005A/C-009 | Phase 1 | T-006 | All security mutation and audit outcomes atomic | PLANNED |
| R-007 | S:SDK/tool registrar | C-002/C-002A/C-007A | Phase 2 | T-007 | Add hello tool without gateway edit | PLANNED |
| R-008 | S:Package lifecycle | C-002A/C-005A/C-007A | Phase 2 | T-008 | Separate install, trust, enable and grant stages | PLANNED |
| R-009 | S:Package integrity | C-002A | Phase 2 | T-009 | Any byte alteration invalidates trusted digest | PLANNED |
| R-010 | SEC:AppContainer backend | WIN-QUAL-001/C-003 | Phase 2 | T-010 | OS-enforced denied access on qualified Windows build | PLANNED |
| R-011 | P:Worker supervisor | C-003/WIN-QUAL-001 | Phase 2 | T-011 | Worker crash cannot corrupt gateway or authority | PLANNED |
| R-012 | G:JSON Schema engine | C-002/C-004 | Phase 2 | T-012 | All input/output violations rejected | PLANNED |
| R-013 | SEC:Owner approval service | C-006A/C-005A/C-007A | Phase 2 | T-013 | Owner-only bound single-use approval | PLANNED |
| R-014 | G:Multi-client policy | C-001/C-005A | Phase 3 | T-014 | Disjoint client isolation including results | PLANNED |
| R-015 | G:Scheduler/worker | C-003/C-005A | Phase 3 | T-015 | Bounded cancellation/deadlines and audited uncertainty | PLANNED |
| R-016 | P:Recovery/idempotency | C-005A/C-008 | Phase 4 | T-016 | Crash restores committed state; ambiguous effects not replayed | PLANNED |
| R-017 | SEC:Audit/redaction | C-009/C-005A | Phase 4 | T-017 | No secret/argument canaries in logs | PLANNED |
| R-018 | REL:Installer/recovery | C-007A/C-008 | Phase 5 | T-018 | Clean Windows install/upgrade/restore/uninstall | PLANNED |
| R-019 | REL:Evidence gate | QUALIFICATION | Phase 5 | T-019 | Final exact SHA qualified locally or self-hosted | PLANNED |
| R-020 | S:Native worker ABI | C-003/WIN-QUAL-001 | Phase 2 | T-020 | C++/Rust IPC compatible without gateway changes | PLANNED |
| R-021 | SEC:Origin/Host | C-010/C-001 | Phase 1 | T-021 | Hostile Host/Origin blocked pre-dispatch | PLANNED |
| R-022 | SEC:File broker | C-002A/WIN-QUAL-001 | Phase 2 | T-022 | Traversal/junction/race never escapes scope | PLANNED |
| R-023 | P:Store recovery | C-005A/C-008 | Phase 4 | T-023 | Corruption requires recovery, no empty reset | PLANNED |
| R-024 | REL:CLI | C-007A | Phase 5 | T-024 | All 54 commands stable JSON and exit codes | PLANNED |
| R-025 | P:Encrypted backups | C-005A/C-008 | Phase 4 | T-025 | Clean restore preserves revocations/audits | PLANNED |
| R-026 | G:MCP capabilities | C-010 | Phase 1 | T-026 | Unsupported features unadvertised and refused | PLANNED |

## Phase-level gate completeness
- **Phase 1:** R-001–006, R-021, R-026 → T-001–006, T-021, T-026; store, identity, transport and MCP gates. Approval in general use is deferred to Phase 2; no sensitive action may run earlier.
- **Phase 2:** R-007–013, R-020, R-022 → T-007–013, T-020, T-022 plus PKG-001–012, APR-001–010, WQ-001–012 and CLI scope tests.
- **Phase 3:** R-014–015 → T-014–015, concurrency and isolation matrix.
- **Phase 4:** R-016–017, R-023, R-025 → T-016–017, T-023, T-025, Q-DB-01–08, backups/crash/retention.
- **Phase 5:** R-018–019, R-024 → T-018–019, T-024, clean-host and final regression.

## Evidence requirements by test
Each planned T-ID requires: exact code SHA, prereq schema/migration revision, Windows build/SKU/patch, qualified AppContainer profile, owner vs worker SID (redacted safely), package/fixture digests, locally executed PowerShell transcript, expected+observed results, gateway audit correlation, artifact checksums, cleanup and rerun. Must be labelled `PLANNED`, `MANUAL_LOCAL` or `SELF_HOSTED_ACTIONS`; never map a specification to PASS.

## Coverage audit and orphan checks
26 original R identifiers preserved, each has one primary T identifier and one phase, component and acceptance outcome. C-001..C-010 primary catalog remains; C-002A, C-005A, C-006A, C-007A and WIN-QUAL-001 provide supplements to those contracts, not replacements. Cross-contract gaps still exist; this mapping **does not** automatically mean every contract implementation-ready. Additional fixture families must be linked through the primary T-ID when evidence is collected.
- T-001/T-021: loopback and hostile Origin/Host.
- T-002/T-003/T-005/T-014: client/credential/grant epoch boundaries.
- T-006/T-016/T-023/T-025: SQL atomicity, migration, corruption and recovery.
- T-007/T-008/T-009/T-022: bytes, immutable staging and scope races.
- T-010/T-011/T-020: worker OS enforcement and IPC.
- T-012/T-026/T-004: JSON Schema, MCP version and unsupported features.
- T-013: independent owner confirmation + concurrent single-use.
- T-015/T-017: deadlines, logs, error handling.
- T-018/T-019/T-024: Windows installation, exact SHA and CLI.

## Open integration gaps, not hidden by the mapping
1. Detailed released MCP C-010 conformance transcripts and schema pinning not completed.
2. C-001 raw admin enrollment IPC wire envelope needs exhaustive normative message schema.
3. C-005A external anti-rollback checkpoint two-resource durability requires precise recovery-proof protocol; SQLite alone is insufficient.
4. Windows W0/W1 worker API/ACL/security descriptor fixture values and loader snapshot proof need build-specific demonstration.
5. CLI C-007A per-command full data JSON Schemas, secure one-time credential delivery and backup key export envelope need final agreement.
6. Full power/performance fixture acceptance limits and documentation-level exact PowerShell execution commands rely on future authorized harness naming.
7. Outstanding ADR status reconciliation and final independent freeze audit not completed.
No owner approval, implementation permission, or freeze is implied.

## Freeze reconciliation — 2026-10-25
The seven 'Open integration gaps' listed above are the historical pre-freeze worklist, not live unresolved decisions. C-010A/B/C pin upstream 2026-07-28 and the bounded tools-only transport; C-001/C-007C govern enrolled clients and owner IPC; C-005/CP-001 and definitive 19-table SQL govern checkpoint publication; WIN-SEC-003 specifies fail-closed OS-profile qualification; C-007G/H cover root CLI schemas, required identities and 54 command data keys; the T-001..026 fixture/operations documents specify future acceptance; decisions and cross-document review are ratified in `ARCHITECTURE_FREEZE_V1.md` and `PHASE0_FINAL_ZERO_BLOCKER_REVIEW_2026-10-25.md`. Executable harnesses, live OS confinement, cross-file durability, full MCP scored-suite testing and release verification remain **later-phase gates**, not claimed Phase 0 PASS.
