# Phase 1 pre-phase audit — 2026-10-25

**Disposition: HOLD. Phase 1 incomplete — blocked before implementation.**

The owner supplied the Phase 1 Development Agent Handoff and conditionally authorized implementation after a successful audit. That authorization is acknowledged; no second coding authorization is required after the gate passes. However, the handoff explicitly requires stopping implementation when a material contract gap or contradiction is found. Rules 3, 6 and 8 prohibit silently inventing missing architectural behavior. This report records three such gaps. It does not rescind the historical freeze, move its tag, or claim that prior documentation tests failed.

## Baseline and repository verification

- Repository: djames1987/DragonForge-MCP.
- main: `53c6522a1c1e99e54d5569ff285083df70be3860`.
- Annotated tag object: `e172ade2da812233d4be63548d6bde1ba5a61914`.
- Peeled `phase0-architecture-v1`: `d673a4eec911242f02c44d01ac14c3d17abe308c`.
- Both source trees: `deae1e9ac3e4707abb0ffee73ef501d50b292d21`.
- GitHub PR #1 is merged at the expected main SHA. Its old PR body still describes historical blockers; actual merge metadata and frozen precedence govern.
- GitHub branch inventory at inspection: main and docs/phase0-architecture-baseline-20261021. No open PRs at initial inspection.
- Fresh clone was clean. No application source, executable project tests, or Actions workflows existed. No intervening content changes after the tagged baseline.
- No implementation branch is created. This report may be committed on a documentation-only audit branch for review.

## Governance, prior phase and review coverage

Read AGENTS.md, all 26 DEVELOPMENT_RULES, the complete ROADMAP, ARCHITECTURE_FREEZE_V1, final Phase 0 zero-blocker review, REQUIREMENTS, ARCHITECTURE, SECURITY, DECISIONS, IMPLEMENTATION_PLAN, CONTRACTS, PHASE0_REQUIREMENT_TRACEABILITY, QUALIFICATION, Q-EXEC-001, deterministic fixture and operational/dependency plans. Reviewed Phase 1 identity, administrative CLI/schema/security, authoritative SQLite, recovery, MCP wire/addendum/source pin, and Windows authority/pipe contracts. Searched the repository for definitions that could close the findings below.

The freeze explicitly supersedes old OPEN/DRAFT wording. Such wording alone is not a blocker. Historical Phase 0 completion and owner conditional freeze are recorded and the exact tagged tree is intact. The newly identified semantic gaps mean its implementation-readiness conclusion cannot simply be inherited.

DEC-001/006 remain owner-directed; DEC-002..005 remain ratified through the conditional freeze. Preserve modern-only MCP, tools-only capability scope, independent credentials, separate discovery/invocation grants, owner-only administrative channel, authoritative SQLite transactions, checkpoint publication barrier and fail-closed workers.

README selects Python as primary orchestration language. The supplied handoff additionally requests a Rust/SQLite authority boundary. A later implementation must preserve Python orchestration and document the narrow Rust boundary; the handoff is not a reason to silently replace the whole gateway architecture.

## Blocking findings and concrete closure requirements

### P1-AUD-001 — Credential wire and verifier bytes are incomplete

Sources: C-001A, identity-ipc-v1.md, “Clients and credential semantics”; delegated DEC-002.

The contract specifies a 32-byte secret, independent 128-bit key_id, a verifier over domain + instance UUID + client UUID + key_id + secret, and HTTP `Authorization: Bearer <base64url>`. It says key_id “may be transport metadata,” but specifies neither its transport field nor whether it is inside the encoded bearer. UUID/key byte representations, padding/canonical base64url rules and exact token length are not defined. The selected design explicitly calls for lookup through non-secret key IDs.

Two independent adapters can therefore comply with the prose yet produce incompatible tokens/verifiers. Choosing a new public header or packed bearer layout would establish an observable identity contract, not merely name a private helper.

**Recommended closure for owner review:** specify a versioned packed bearer containing a fixed-width key ID and secret; define exact byte lengths, UUID/HMAC input encoding, canonical unpadded base64url and rejection rules. Select and document one layout before writing adapters. Include independently calculated valid and malformed token/HMAC vectors, unknown-key behavior and rotation cases. Do not treat key ID as proof of identity.

Impact: C-001, C-007 protected credential contents, HTTP/STDIO adapters; R-002/003; T-002/003/004. No layout is adopted by this report.

### P1-AUD-002 — Persistent authority and audit hashes lack complete serialization

Sources: sqlite-authoritative-v1.md non-SQL checks 3/5 and checkpoint publication; recovery-reconciliation-v1.md CP-001 steps 2–5; C-009.

C-005 requires an audit chain over “canonical redacted event fields.” CP-001 references `normalized_authority_digest` and a canonical I-JSON checkpoint envelope. The contracts do not enumerate the snapshot tables/columns, row ordering, value encodings, audit field inclusion/exclusion, genesis previous hash, or the precise on-disk MAC framing. I-JSON restricts JSON values but does not itself select a unique byte serialization. The outbox stores digests, not a complete serialized checkpoint.

Without those definitions a compatible restart, independent verifier or later migration cannot reliably reproduce the committed hashes. This is particularly relevant to crash recovery after checkpoint replacement and before its PUBLISHED flag is committed. Merely selecting SHA-256/HMAC does not define the hashed message.

**Recommended closure for owner review:** define versioned audit, authority-snapshot and authenticated-checkpoint formats with exact field sets, sort orders, encodings, domain separators, genesis rules and MAC coverage. Explicitly exclude self-referential/mutable publication bookkeeping from authority hashing, while covering all security authority records. Specify how replay reconstructs the identical committed checkpoint after each crash boundary, with independent expected-byte/hash vectors for genesis, enrollment, grant, revocation and publication recovery.

Impact: C-005/C-009 and CP-001, R-006 and future R-016/023/025; T-006 and future recovery suites. Preserve the 19-table baseline unless an expressly reviewed schema change is necessary. No replacement format is adopted here.

### P1-AUD-003 — Pending credential delivery has no durable reconciliation contract

Sources: cli-security-key-formats-v1.md secret delivery state machine; identity-ipc-v1.md admin reply contract; sqlite-authoritative-v1.md credentials table and TX01.

C-007C requires ISSUED_PENDING_DELIVERY -> DELIVERED or REVOKED_DELIVERY_FAILURE, and prevents authentication before a failed delivery is revoked. TX01 commits credential issuance and later revokes on delivery failure. The credentials table has no delivery state, and no mapping to another durable record, acknowledgment or restart rule is specified. The strict admin reply contains metadata only; the contract also does not define a separate secret-delivery frame.

Consider a crash after issuance commits or Credential Manager write occurs but before readback/delivery completion is recorded. The baseline does not define how restart distinguishes delivered from pending credentials, proves completion or revokes uncertain issuance before admitting calls. This is a specification gap, not evidence of an implemented exploit.

**Recommended closure for owner review:** specify durable delivery transitions and the protected storage/acknowledgment path; make pending credentials ineligible; reconcile incomplete delivery on startup by audited revocation before READY. Either map those states explicitly to existing durable records or propose a separately reviewed schema change. Specify failure of the compensating revoke/publication as locked recovery, and test every issuance/write/readback/acknowledgment crash boundary. Ordinary CLI JSON must remain metadata-only.

Impact: C-001/C-005/C-007, R-002/003/006; T-002/003/006 and CLI-S05/06. No schema change is made here.

## Requirement / contract / planned test crosswalk

| Requirement | Contract / component | Required Phase 1 acceptance |
|---|---|---|
| R-001 | C-010 loopback transport | T-001 bind validation and socket evidence |
| R-002 | C-001/C-005 authenticated policy | T-002 anonymous/unknown/revoked denial |
| R-003 | C-001/C-007 owner enrollment | T-003 forged metadata, expiry, rotation, revocation |
| R-004 | C-010A/B/C pinned modern MCP | T-004 HTTP and STDIO positive/negative transcripts |
| R-005 | C-001/C-005 admission and prelaunch | T-005 synchronized revocation race |
| R-006 | C-005/C-009 writer and checkpoint | T-006 mutation/audit/outbox fault injection |
| R-021 | C-010/C-001 Host/Origin | T-021 predispatch rejection |
| R-026 | C-010 tools-only capabilities | T-026 unsupported features refused |

All eight runtime test IDs are **NOT RUN**: no implementation exists. The blockers affect prerequisites for multiple workstreams; no unaffected code work was started.

## Documentation evidence independently checked

Execution: temporary inline validation in the agent's Linux scratch environment, Python 3.12.14, jsonschema 4.26.0, SQLite 3.53.1. **Not owner Windows MANUAL_LOCAL qualification and not SELF_HOSTED_ACTIONS.** These are disposable schema/document checks, not project test code or runtime acceptance.

| Check | Actual result |
|---|---|
| main/tag commit and tree equality | PASS |
| CLI registry Draft 2020-12 metaschema | PASS |
| Representative request objects, command schemas and root request union | 54/54 PASS |
| Representative success objects, command success schemas | 54/54 PASS |
| Unknown argument field rejected | 54/54 PASS |
| Each required argument individually removed and rejected | 51/51 PASS, across 34 commands |
| Canonical migration in disposable memory DB with bound parameters | 19 application tables; integrity ok; FK check empty; user_version 1 |
| Requirement and primary qualification ID inventory | 26 R IDs; 26 T IDs |
| Pinned MCP schema identity | blob 213c58f6d9a1c2ce6ad055afe90bbdb095a29ee8 confirmed |
| ListToolsRequest/ListToolsResult/CallToolRequest/CallToolResult/DiscoverResult specimens | 5/5 PASS |

Migration payload used: exact LF UTF-8 bytes between the authoritative SQL fence, SHA256 `a77af3b7fb0e61ac2845f6ee53fb11a77c407e10f0b35ce6f65589e236de36ec`. Statements executed in order with SQLite bound parameters, including the payload checksum. In-memory mode does not prove WAL, file durability, concurrency or Windows recovery.

The CLI specimens were generated from required properties, constants/enums and valid scalar examples, then validated with format checking. They confirm structural consistency, not independent acceptance of every semantic boundary. All 51 required-argument removals were tested individually. MCP specimens reproduce the documented examples against the fetched immutable upstream schema at commit 80041ab0a139a8f4fb3adbc99f7ae9113ea59671. No full conformance score is claimed.

Historical evidence reviewed under docs/evidence preserves its original limitations: some early CLI jobs did not capture the exact local checkout SHA; the SQLite run used a placeholder checksum; later final review records the root-union repair at 1fd316fb829e08f67eaa2c63bcc9faf01ee58460. This audit independently checks current frozen content rather than relabeling those jobs.

## Local qualification plan and environment limitations

Current owner runner connectivity, Windows UBR, toolchain and local working tree are **NOT VERIFIED**. The GitHub connector rejected the /actions/runners read as an unsupported endpoint; this does not mean the runner is offline. No workflow was submitted. No local-debugger job was run and no owner files were changed.

After contract closure and a PROCEED audit:
1. Create a Phase 1 implementation branch from freshly verified main.
2. Document precise module boundaries; implement private Windows platform adapters and the authoritative store from the exact approved migration.
3. Implement issuance/delivery, credential validation and owner administrative access.
4. Implement loopback modern MCP, harmless built-in diagnostics, separate grants and enrolled STDIO bridge.
5. Implement the specified dfmcp-qual interface and disposable fixtures; review negative and crash cases.
6. Verify owner Windows environment and use MANUAL_LOCAL or explicitly labeled self-hosted Windows/X64 jobs. Never hosted runners.
7. Run T-001..006, T-021, T-026 at the final implementation SHA, preserving redacted commands, expected/actual results, fixture/binary hashes and cleanup records. Fix failures and rerun affected checks.
8. Review drift and publish the completion report; no merge or Phase 2 transition without complete exit criteria.

## Change impact and decision log

Only this report is added. No frozen contract, migration, tag, requirement, workflow or application code is changed. This records newly discovered baseline defects rather than introducing architecture drift. Contract resolutions require change-impact review, traceability updates, an ADR where appropriate and owner review under rule 6.

1. AUD-DEC-001: Use the immutable frozen tree and precedence rules; do not treat superseded DRAFT labels or the stale PR body as new blockers.
2. AUD-DEC-002: Recheck documentation specimens independently; do not equate structural checks with Windows runtime qualification.
3. AUD-DEC-003: HOLD on the three substantive contract gaps instead of inventing security wire/persistence behavior under routine implementation authority.
4. AUD-DEC-004: Preserve findings on a documentation-only audit branch; do not create a Phase 1 implementation branch, change the freeze tag, or start Phase 2.

These are audit/disposition decisions. **No implementation decisions or features were delivered.** No qualified implementation SHA exists.

## Next action

Review and approve a documented contract-closure amendment for P1-AUD-001..003, including exact vectors and failure/recovery semantics. Retain the original tag as historical immutable baseline. Re-run the pre-phase audit against the approved amended baseline, then proceed under the owner's existing conditional Phase 1 authorization.

All 26 development rules remain binding. Future handoffs must carry them, the exact baseline, these three open findings, the local-only runner restriction and the NOT RUN runtime status.
