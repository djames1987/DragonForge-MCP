# DragonForge MCP — Full Lifecycle Roadmap (Frozen V1)

Status: FROZEN V1 documentation under `phase0-architecture-v1` (2026-10-25); see [architecture freeze](ARCHITECTURE_FREEZE_V1.md) and [binding rules](DEVELOPMENT_RULES.md). **Phase 1 implementation remains unauthorized until the owner's separate instruction.**

## Phase 0 — Document the entire project
Build the complete implementation-ready, cross-referenced specification for Phases 1–5, including detailed contracts, requirement IDs, ADRs, data models, threat model, error taxonomy, lifecycle, CLI UX, tool authoring, install/trust/enable/grant, worker isolation, security qualification, test fixtures, performance, packaging, migration and rollback. Resolve all blocking open decisions. Publish a traceability matrix and exact-SHA baseline. **Exit:** owner explicitly signs off on documentation freeze and permits coding.

## Phase 1 — Gateway and authoritative security foundation
Implement the locally bound MCP transport and STDIO bridge contract, authenticated client identities, deny-by-default discovery/invocation permissions, transactional persistence/migrations, audit event contract, and harmless diagnostic tool. **Exit:** approved Phase 1 acceptance/negative tests pass on manual Windows or local self-hosted runner evidence, exact SHA qualified.

## Phase 2 — Extensible tool platform
Implement simple Python authoring SDK/decorator, versioned manifest contract, CLI generator/validation/install/trust/enable/grant lifecycle, gateway-independent tool registration, safe worker IPC, cancellation/timeouts and constrained execution. **Exit:** third-party sample tool installed without gateway code edits, authorization and tamper tests pass.

## Phase 3 — Supported tools and multi-agent interoperability
Implement documented limited file, development and system diagnostic capabilities and client adapters; test simultaneous agents, capability scoping, concurrency, revocation and failure recovery. **Exit:** interoperability matrix and adversarial qualification pass.

## Phase 4 — Reliability and performance
Meet defined latency, throughput and memory targets through measured benchmarks; test forced crashes, persistence recovery, logging redaction and lifecycle transitions; optimize implementation only where justified. **Exit:** reproducible performance/reliability evidence.

## Phase 5 — Packaging and release qualification
Package Windows-first on-demand distribution, configuration, upgrades and rollback, documentation/SDK examples, version migration and disaster recovery. **Exit:** complete release acceptance matrix, clean-machine install/upgrade/uninstall, exact-SHA qualification and owner sign-off.

## Global phase rules
- Phase 0 documents the **whole** project before any code, including tests and CI workflows.
- Every phase starts with the preceding-phase audit, complete rules/roadmap review and drift report.
- Every phase ends with exact-SHA evidence; never advance on unqualified results.
- All GitHub Actions use user-controlled local/self-hosted runners only.
- Until a runner is available, the owner may perform equivalent manual tests locally with evidence; do not claim GitHub Actions succeeded.
- Change of roadmap, contracts or frozen architecture requires change-control review, not implicit scope changes.
