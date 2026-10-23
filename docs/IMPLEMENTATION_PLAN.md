# Implementation phase gates and sequencing — DRAFT

This document expands docs/ROADMAP.md without superseding it. All implementation is forbidden until owner approval of complete Phase 0 documentation freeze.

## Common pre-phase gate
Re-read AGENTS.md, DEVELOPMENT_RULES.md, ROADMAP.md, approved freeze/ADRs/contracts; inspect exact main and candidate SHA; review prior phase deliverables, defect log, actual test evidence and manual-vs-self-hosted runner classification; document pre-phase audit and any owner-approved exception. Never silently adjust architecture. Each phase ends with documented exact-SHA evidence.

## Phase 0 — Complete architecture and contracts
Prereq: owner-approved governance (already available).
Sequence: repository audit -> stable requirements -> source-pinned MCP revision -> threat/identity/approval ADRs -> normative manifests/worker/CLI/API schemas -> complete relational schema -> recovery/operations -> quantitative test fixtures -> traceability and consistency audit -> owner ratification.
Deliverables: architecture freeze, reviewed decisions, complete contract catalog, formal test plan with local commands, zero blocking decision, explicit owner authorization.
Exit: owner signs exact baseline SHA and explicitly authorizes implementation. Current exit: FAIL/NOT READY.

## Phase 1 — Gateway/security foundation
Prereq: signed Phase 0 freeze.
Sequence: project foundation -> loopback endpoint and protocol negotiation -> authenticated client registration -> authorization at discovery and execution -> gateway-only SQLite writer -> audit -> STDIO adapter -> negative tests.
Deliverable: harmless diagnostic tool + repeatable local tests.
Gate: T-001..T-006,T-021,T-026 PASS on qualified SHA, no privilege bypass and complete report.

## Phase 2 — Extensibility and constrained workers
Prereq: qualified Phase 1.
Sequence: immutable manifest/SDK -> validator -> install/inspect/trust/enable/grant CLI -> native/Python worker IPC -> OS containment -> approval flow -> cancellation and idempotency handling.
Deliverable: developer creates hello tool independently of gateway source; third-party package remains deny-by-default.
Gate: T-007..T-013,T-020,T-022 PASS and real Windows isolation evidence.

## Phase 3 — Tool catalogue / interoperability
Prereq: qualified Phase 2.
Sequence: narrow filesystem read/write capabilities -> diagnostics and development tools with explicit scopes -> adapters for tested MCP clients -> concurrent/multiple identities -> revoke/cancel/error semantics.
Gate: T-014,T-015 and versioned client compatibility matrix PASS. No unrestricted shell capability.

## Phase 4 — Reliability and performance
Prereq: qualified Phase 3.
Sequence: crash and fault injection -> backup/restore and corruption drill -> scheduler limits -> measured optimization -> audit redaction -> documented resource envelope.
Gate: T-016,T-017,T-023,T-025, performance percentiles and loss/recovery SLO PASS on recorded Windows hardware.

## Phase 5 — Distribution and release
Prereq: qualified Phase 4.
Sequence: reproducible build -> Windows on-demand packaging -> upgrade and rollback migration drills -> uninstallation and credentials cleanup -> clean-machine tests -> SDK/user/admin documentation -> security acceptance -> tagged qualified release.
Gate: T-018,T-019,T-024 plus all release regression tests PASS; owner explicitly approves release.

## Required completion artifact
Each phase report must list base SHA, qualified SHA, scope, outputs, all test IDs and actual outcomes, runner identity, artifact links, defects, deviations, traceability changes, owner exceptions and next-phase recommendation. Only the owner may ratify frozen architecture, approve changes to requirements and authorize next release boundary.
