# Phase 0 traceability and freeze-work register — 2026-10-21

**Status: DRAFT; coverage index only.** No row here constitutes evidence of test execution, contract completeness, freeze approval or implementation authorization. Governing rules: `docs/DEVELOPMENT_RULES.md`.

## Existing requirement -> contract -> test -> implementation phase

| Requirement | Proposed contracts / documentation | Qualification | Phase |
|---|---|---|---|
| R-001 | C-010, ARCHITECTURE | T-001, T-021 | 1 |
| R-002 | C-001,C-002 | T-002 | 1 |
| R-003 | C-001 | T-003 | 1 |
| R-004 | C-010 | T-004 | 1 |
| R-005 | C-001,C-002,C-003 | T-005 | 1 |
| R-006 | C-005,C-009 | T-006 | 1 |
| R-007 | C-002, tool-authoring | T-007 | 2 |
| R-008 | C-002,C-007 | T-008 | 2 |
| R-009 | C-002 | T-009 | 2 |
| R-010 | C-003, SECURITY | T-010 | 2 |
| R-011 | C-003,C-004 | T-011 | 2 |
| R-012 | C-002,C-003,C-004 | T-012 | 2 |
| R-013 | C-006 | T-013 | 2 |
| R-014 | C-001,C-003 | T-014 | 3 |
| R-015 | C-003,C-004 | T-015 | 3 |
| R-016 | C-003,C-005,C-008 | T-016 | 4 |
| R-017 | C-009,SECURITY | T-017 | 4 |
| R-018 | C-007,C-008 | T-018 | 5 |
| R-019 | QUALIFICATION,IMPLEMENTATION_PLAN | T-019 | 5 |
| R-020 | C-003 | T-020 | 2 |
| R-021 | C-010,SECURITY | T-021 | 1 |
| R-022 | C-002,C-003,SECURITY | T-022 | 2 |
| R-023 | C-005,C-008 | T-023 | 4 |
| R-024 | C-007 | T-024 | 5 |
| R-025 | C-005,C-008 | T-025 | 4 |
| R-026 | C-010 | T-026 | 1 |

Every R-001..R-026 has a preliminary linked test and phase, **but mapping to a sketch contract is not specification completeness**. Existing T-001..T-026 lack fully reproducible fixtures. Scope expansion of requirements must use stable new IDs and simultaneously extend all relevant contracts, tests and phase gates.

## Contract completion backlog — order matters
1. **Protocol:** Ratify ADR-001 and exact modern/legacy baseline. Complete C-010 with pinned normative schemas and raw requests/responses; reconcile protocol methods and HTTP/STDIO.
2. **Identity and operator approval:** Define owner-controlled bootstrap and local authenticated administrative channel, credential issue/rotate/revoke, trusted human approval provenance and exact denial behavior; finalize C-001/C-006. Resolve DEC-002,DEC-003.
3. **Windows confinement:** Specify capability-scoped token, ACL, process, network and IPC controls and a supported Windows 10 SKU/build matrix. If a profile cannot be proven, hard-deny that tool. Resolve DEC-004.
4. **Tools and IPC:** Complete package canonicalization, manifests and JSON Schema dialect; versioned framing, worker authentication, bounded queues, cancellation/idempotency state machine, digest race defense; finalize C-002/C-003.
5. **Storage:** Full relational DDL, indexes, constraints, migration order, transaction/failure tables, consistency and backup integration; finalize C-005, C-008, C-009.
6. **CLI and operations:** Exact commands, schemas, authentication, exit codes, first-run, backup/restore/rollback/uninstall runbooks; finalize C-007 and C-008; resolve DEC-005,DEC-006.
7. **Qualification:** Each T-ID needs a complete platform-dependent or exact wire fixture with prerequisites, invocation, expected result, negative scenario, evidence and cleanup; add measurable benchmark cases.
8. **Full consistency review:** Validate direct R/C/T cross-references against every actual document and link; record review scope, residual risk and owner ratification, exact candidate SHA.

## Qualification evidence rule
Phase 0 documentation review and static Git tree inspection are **not** code test runs. Later test results must classify separately `MANUAL_LOCAL`, `SELF_HOSTED_ACTIONS`, or `NOT_RUN`, include exact tested SHA, fixture hashes, OS/build, expected and actual outputs, commands, reviewer, and any reruns. Never use hosted runners or silently claim a PASS.

## Handoff boundary
For another agent: first read `AGENTS.md`, `docs/DEVELOPMENT_RULES.md` (all 26 rules), `docs/ROADMAP.md`, existing Phase 0 audit, continuation audit, all ADRs/contracts and branch SHA. Stay on existing documentation PR when appropriate. Do not write implementation code or workflow YAML. Do not merge as a substitute for owner approval. Stop at owner-ratification boundary. Document proposed material choices explicitly rather than marking them approved.
