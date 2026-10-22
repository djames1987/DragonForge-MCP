# DragonForge MCP requirement register — DRAFT

Authority: subordinate to docs/DEVELOPMENT_RULES.md and owner approvals. All quantities here are proposed acceptance thresholds, not owner-ratified.

Each requirement uses stable ID; owner: gateway team (G), SDK team (S), platform team (P), security team (SEC), release team (REL). Mapping to qualification IDs follows.

| ID | Owner | Phase | Requirement / measurable acceptance | Verification |
|---|---|---|---|---|
| R-001 | G | 1 | Bind HTTP exclusively to configured loopback address; attempts to bind non-loopback rejected by default | T-001 |
| R-002 | SEC | 1 | Unregistered/revoked clients see no tools and cannot invoke any tool | T-002 |
| R-003 | SEC | 1 | Authenticate using host-managed credentials; reject forged client identity in request metadata | T-003 |
| R-004 | G | 1 | Support one ratified MCP revision precisely; reject unsupported protocol versions deterministically | T-004 |
| R-005 | G | 1 | All tool invocation decisions re-evaluated at execution, even after discovery | T-005 |
| R-006 | P | 1 | Persist registration, grants, audit outcome atomically according to transaction contract | T-006 |
| R-007 | S | 2 | Author can install simple Python tool without modifying gateway code | T-007 |
| R-008 | S | 2 | Install, trust, enable, grant are independently enforced lifecycle steps | T-008 |
| R-009 | S | 2 | Altering a trusted tool manifest or code digest prevents invocation until re-trusted | T-009 |
| R-010 | SEC | 2 | Untrusted worker execution is denied unless verified OS-enforced isolation policy applies | T-010 |
| R-011 | P | 2 | Worker crash cannot crash gateway or grant permissions | T-011 |
| R-012 | G | 2 | Tool input and output conform to published JSON Schemas; malformed values rejected | T-012 |
| R-013 | SEC | 2 | Sensitive tool action cannot proceed without verifiable local human authorization | T-013 |
| R-014 | G | 3 | Two isolated clients cannot access each other's tools, scopes, execution results | T-014 |
| R-015 | G | 3 | Worker cancellation and timeout are bounded and auditable | T-015 |
| R-016 | P | 4 | Crash/restart preserves committed state, rejects replay that would duplicate side effects | T-016 |
| R-017 | SEC | 4 | Logs exclude secrets and unauthorized argument contents under supplied adversarial fixtures | T-017 |
| R-018 | REL | 5 | Clean Windows 10 install, upgrade, restore and uninstall succeed under documented procedures | T-018 |
| R-019 | REL | 5 | Every release qualified at exact final SHA on owner-controlled local runner or manual local host | T-019 |
| R-020 | S | 2 | Compatible C++ or Rust worker can implement versioned IPC without gateway edits | T-020 |
| R-021 | SEC | 1 | Unexpected Host or Origin rejected before MCP processing | T-021 |
| R-022 | SEC | 2 | Path traversal, junction/symlink escapes, and scope change races fail closed | T-022 |
| R-023 | P | 4 | Corrupt database enters explicit recovery mode rather than silent regeneration | T-023 |
| R-024 | REL | 5 | CLI has stable machine-readable JSON output and documented exit codes for all commands | T-024 |
| R-025 | P | 4 | Full backups are verified by clean restore, including revocations and audit history | T-025 |
| R-026 | G | 1 | Unsupported MCP features are explicitly advertised as unavailable and do not execute | T-026 |

## Out-of-scope
Cloud control plane, public-network access, arbitrary unrestricted shell execution, automatic untrusted plugin downloads, GUI-first release, silent integration into other DragonForge projects.

## Nonfunctional targets pending ratification
Propose gateway administration responses <= 1000ms p95 under idle local conditions; tool discovery <= 250ms p95 for 100 registered tools; gateway idle private working set <= 200MiB; maximum default 4 concurrent untrusted workers; per-request input 1MiB/output 4MiB; 30-day local metadata retention. All numbers DRAFT and require an explicit benchmark machine/workload agreement (DEC-004). None are guaranteed or release-qualified.

## Rule of change
When adding/revising R IDs, update CONTRACTS, QUALIFICATION, IMPLEMENTATION_PLAN, relevant ADRs and architecture change log in same PR. Do not reuse retired identifiers.

## Phase 0 detailed contract crosswalk
The canonical [requirements traceability matrix](PHASE0_REQUIREMENT_TRACEABILITY.md) maps R-001 through R-026 to architecture components, detailed contracts, Phases 1–5, qualification IDs and measurable acceptance. DEC-005's selected 90-day audit metadata retention supersedes the older proposed 30-day metadata retention figure above for planning; this is not owner freeze approval. The matrix is a documentation mapping, not executed test evidence.
