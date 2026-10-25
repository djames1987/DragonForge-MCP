# DragonForge MCP — Binding AI Agent Development Rules

Status: **APPROVED by project owner on 2026-10-21.** These rules supersede conflicting guidance in earlier proposals, handoffs, and agent-generated instructions. Changes require explicit owner approval and documented revision history.

## Mandatory gate 1 — Full-project documentation before coding
1. **No implementation coding until the entire project is documented from start to finish and the owner explicitly approves a documentation freeze.** This applies to application code, test code, workflow configuration, executable prototypes, and implementation scaffolding. Documentation-only commits are permitted.
2. Document the entire intended project, not only the next phase: complete scope and non-goals; functional/nonfunctional requirements; user journeys; use cases; architecture, components, responsibilities and trust boundaries; all contracts (MCP protocol, HTTP/STDIO adapter, client identity, authorization, tool SDK and schemas, worker IPC, persistence/migrations, audit, approval, error handling, cancellation, recovery, configuration, CLI, lifecycle, interoperability); security threat model; platform and deployment; observability; accessibility where applicable; disaster recovery; performance budgets; operational lifecycle; versioning and compatibility; packaging, upgrade and rollback; end-to-end roadmap; and acceptance/negative tests.
3. Specifications must be detailed enough that a new developer or AI agent can independently implement every phase with unambiguous behavior, interfaces, error states, invariants, test commands, and pass/fail criteria, without inventing material architecture decisions.
4. Each requirement and contract must have a stable identifier and traceability to planned phases and qualification tests. All material unknowns are recorded in an open-decision register. No unresolved architecture-blocking decisions at freeze.
5. Review cross-document consistency, resolve contradictions, and create a signed-off architecture baseline identifying the exact Git commit SHA. No code until the owner grants explicit authorization after this review.
6. Documentation modifications after freeze require change-impact assessment, ADR where appropriate, updated traceability/tests, and owner review of scope-changing changes. No silent architecture drift.

## Mandatory gate 2 — Review before every phase
7. Before starting any phase, review the **prior phase's plan, implementation, change log, actual test evidence and completion gate**, together with **these development rules, the entire active roadmap, architecture freeze, applicable ADRs, contracts, and current repository state**.
8. Produce a written pre-phase audit with exact baseline SHA, previous-phase outcomes, requirement/contract crosswalk, outstanding risks, divergences, changes proposed, and recommendation to proceed or hold. **Do not start the next phase if the previous phase has not passed all required exit criteria or an exception has not been explicitly authorized and documented by the owner.**
9. During and after each phase run drift checks against the frozen architecture, requirements, contracts, security model, and roadmap. Document deviations and request approval before making architecture changes.
10. Every phase ends with an evidence-based completion report including exact qualified SHA, test procedures/results, failures and remediations, requirement traceability, unresolved issues and next-phase readiness. Failed tests must be repaired and rerun before claiming PASS.

## Mandatory gate 3 — Local-only GitHub Actions runners
11. **All GitHub Actions jobs requiring runners must execute on a user-controlled local/self-hosted runner.** Do not use GitHub-hosted runners, including `ubuntu-latest`, `windows-latest`, or `macos-latest`.
12. **Until a local self-hosted GitHub Actions runner has been configured and verified, the owner may run the required tests manually on their local Windows machine.** Provide exact PowerShell commands, prerequisites, environmental setup, expected outputs, and an evidence checklist. Agent must accurately record user-provided results and must not invent a CI PASS.
13. When GitHub Actions are created later, workflows must target explicitly labeled local self-hosted runners. Runner setup and trust/security model must be documented before enabling workflows. Do not fall back to hosted runners when no local runner is available; jobs wait or equivalent manual qualification is used where allowed.
14. A manual run is not represented as an Actions run. Distinguish local/manual evidence, self-hosted CI evidence, and untested conditions. Prefer repeating qualification on the exact final commit.

## Ongoing engineering rules
15. Local-only deployment by default: bind HTTP to loopback, validate Origin and Host, avoid cloud requirements.
16. Deny by default: gateway-verifiable client identity, per-tool discovery and invocation authorization, no client-supplied permission claims or approvals.
17. Add custom tools through a stable public SDK/manifest and CLI without changing gateway source. Installation, trust, enablement, permission grants and execution are distinct steps.
18. Pin versioned tool definitions and code digests, validate schemas, and re-review after changes.
19. Fail closed on configuration, identity, authorization, approval, schema, worker confinement, and policy errors.
20. Do not call a subprocess a sandbox: third-party code needs documented OS-enforced isolation or is excluded until safe execution is available.
21. Protect credentials and sensitive data, restrict file permissions, and minimize/redact logs and audit records.
22. Maintain reproducible security tests including cross-client access, revocation, path traversal, Origin attacks, approval spoofing, altered tool digests, crashes, retries and concurrent calls.
23. Keep platform-dependent operations behind explicit interfaces; Windows 10 first, portability preserved.
24. Tools cannot bypass the authoritative gateway, registry, identity, permissions, or audit stores.
25. Never claim actions, test results, deployment readiness, security guarantees, or phase completion without supporting evidence.
26. Do not expand scope or skip stages without owner approval; handoffs must carry these rules and the exact project status forward.

## Pre-phase checklist (all required)
- [ ] Entire-project documentation freeze approved and implementation authorized
- [ ] Development rules re-read and acknowledged
- [ ] Current roadmap, architectural baseline and ADRs reviewed
- [ ] Previous phase completion and test evidence verified (for Phase 1+, including Phase 0)
- [ ] Current repository SHA and drift audit recorded
- [ ] All required contracts, acceptance tests and local test procedure mapped
- [ ] Test execution environment declared: manually local or verified self-hosted runner
- [ ] Owner-approved scope and open risks documented

## Current status
**Phase 0 architecture documentation is FROZEN as of 2026-10-25 under immutable `phase0-architecture-v1`, following owner-conditional zero-blocker review (see `ARCHITECTURE_FREEZE_V1.md`). No Phase 1 coding is authorized without separate explicit owner approval.** These 26 rules remain unchanged in force; this is a status update only.
