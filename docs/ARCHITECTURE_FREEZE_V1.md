# DragonForge MCP — Frozen V1 Architecture Baseline

**Status: Phase 0 architecture documentation FROZEN, 2026-10-25, pursuant to the owner's explicit conditional instruction to perform a zero-blocker review and then freeze.** Evidence and accepted limitations: [PHASE0_FINAL_ZERO_BLOCKER_REVIEW_2026-10-25.md](PHASE0_FINAL_ZERO_BLOCKER_REVIEW_2026-10-25.md). This is the owner's delegated approval of the **documentation baseline only**, not a cryptographic owner signature.

## Exact immutable source
The annotated Git tag `phase0-architecture-v1` points to the final audited Phase 0 documentation commit. **The commit resolved from that tag is the authoritative frozen SHA.** Any subsequent merge commit on main may have a different SHA but must preserve this tagged frozen content. Do not move or replace the tag.

## Frozen contract precedence
1. Owner's `DEVELOPMENT_RULES.md` rules 1–26 and accepted README product scope.
2. This architecture freeze + selected `DECISIONS.md`: DEC-001 modern MCP 2026-07-28 only, DEC-006 SHA256 exact-byte package pin plus explicit local owner trust (both owner-directed); DEC-002 owner-enrolled independent client identities; DEC-003 trusted owner CLI approvals; DEC-004 W0/W1 AppContainer candidate gated by actual proof; DEC-005 SQLite authoritative security state and daily encrypted backup (agent delegated and accepted via conditional freeze).
3. `REQUIREMENTS.md`, `ARCHITECTURE.md`, `SECURITY.md`, `ROADMAP.md`, `IMPLEMENTATION_PLAN.md`, `OPERATIONS_QUALIFICATION_PHASES_V1.md`.
4. Normative interfaces: C-005 `contracts/sqlite-authoritative-v1.md` (single ordered SQL); C-007 `contracts/administrative-cli.md`, `contracts/cli-command-schema-registry-v1.json`, `contracts/cli-security-key-formats-v1.md`; MCP `contracts/mcp-2026-07-28-conformance-addendum.md`, `contracts/mcp-2026-07-28-wire-v1.md`, pinned official upstream; worker IPC C-003 section 7 `contracts/worker-ipc-wire.md`; identity `contracts/identity-ipc-v1.md`; package integrity, approval, recovery and Windows enforcement contracts.
5. `PHASE0_REQUIREMENT_TRACEABILITY.md` and `PHASE0_DETERMINISTIC_T001_T026_FIXTURES.md` define all future requirement/test mappings; `QUALIFICATION_EXECUTION_CONTRACT_V1.md` defines future runner grammar. All older dated proposal/HOLD audit snapshots are **historical evidence**, not alternative binding architecture.

## V1 feature boundaries
On-demand local Windows-first gateway, loopback-only HTTP and protected STDIO, gateway-owned SQLite writer, independently verified client identity, separate DISCOVER/INVOKE grants, immutable package digest, distinct stage/trust/enable/grant/owner approval, trusted audit and recovery. Modern 2026-07-28 MCP **tools-only**; full scored conformance suite and unsupported resources/prompts/SSE/input-required are outside initial scope. W2 network and W3 admin are unsupported. Untrusted W0/W1 execution remains DISABLED until verified Win10 build/edition/patch confinement, all WQ negative checks and qualified worker loader/pipe/broker.

## Binding future governance
- **No Phase 1 implementation, executable tests/harnesses, prototypes, scaffolding or GitHub Actions workflows are authorized by freezing or merging these documents.** The owner must separately give explicit Phase 1 coding authorization.
- Before Phase 1 and every later phase: prior-phase evidence, exact frozen tag SHA, development rules, roadmap, ADRs, security and contract crosswalk; write pre-phase drift audit.
- All future GitHub Actions must use owner-controlled local self-hosted runner; manual host checks never count as GitHub Actions.
- Runtime containment, backup recovery, crash safety, tool execution, MCP server conformance, Windows patch support and performance remain later phase acceptance tests, not Phase 0 PASS claims.
- Post-freeze documentation changes require requirement/contract/test impact evaluation, ADR where material, and owner review; never silently edit the frozen tag.
