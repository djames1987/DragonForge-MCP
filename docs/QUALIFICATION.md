# Requirements-to-tests qualification matrix — DRAFT

Execution POLICY: documentation only in Phase 0; these are planned tests. No test scripts or Actions workflows should be implemented now. In later phases use manual owner-run PowerShell or approved local self-hosted runner. Never label proposed tests PASS.

| Test | Requirements | Scenario and pass condition |
|---|---|---|
| T-001 | R-001 | Enumerate listening sockets; gateway binds only chosen 127.0.0.1 endpoint, rejects non-loopback configuration |
| T-002 | R-002 | Unregistered, denied and revoked client get zero restricted tools and cannot invoke them |
| T-003 | R-003 | Swap/forge display identity without secret; auth fails; rotate/revoke secret then old credential fails |
| T-004 | R-004 | Replay protocol version transcripts; accepted revision responds according to pinned spec, rejected version deterministic |
| T-005 | R-005 | Discover, revoke, then invoke cached tool; invocation denied with no worker launch |
| T-006 | R-006 | Fault injection between permission write and audit insert; neither partially commits |
| T-007 | R-007 | Install a new hello tool into clean gateway without editing gateway source; client can invoke after separate authorizations |
| T-008 | R-008 | Attempt execute after each install/validate/trust/enable/grant stage; blocked until all required stages satisfied |
| T-009 | R-009 | Mutate manifest and payload after trust; invocation refused until approved replacement digest |
| T-010 | R-010 | Hostile tool probes forbidden file, network, registry, sibling process and host DB; each prohibited access denied by OS |
| T-011 | R-011 | Kill worker midrequest; gateway still authenticates and serves new requests; audit identifies crash |
| T-012 | R-012 | Invalid schema, overlong body, unknown argument, malformed output; correct rejection/no privilege change |
| T-013 | R-013 | Forge, replay, expire and change arguments of approval; all fail; trusted owner approval works once |
| T-014 | R-014 | Two concurrent clients with disjoint grants exchange cached IDs/results; cross-client access denied |
| T-015 | R-015 | Queue saturates, timeouts and cancellation race; gateway enforces documented caps and records terminal states |
| T-016 | R-016 | Force crash after persist/around external side effect; committed state recovered, ambiguous non-idempotent call not blindly retried |
| T-017 | R-017 | Send canary token/secret and sensitive payload; all emitted gateway/worker logs contain no canary |
| T-018 | R-018 | Clean-machine install/upgrade/uninstall/reinstall with backup; state preserved or purged only as documented |
| T-019 | R-019 | Record exact tested SHA, Windows build, Python runtime, fixture digests, commands, stdout/stderr hashes, reviewer and disposition |
| T-020 | R-020 | C++ and Rust workers respond using unchanged documented IPC frames, with invalid versions rejected |
| T-021 | R-021 | Mismatched Host, malicious Origin, DNS rebinding fixture; rejected pre-dispatch |
| T-022 | R-022 | '..', absolute path, junction/symlink swap and rename race cannot access outside approved capability |
| T-023 | R-023 | Corrupt DB file and migration failure lead to explicit recovery state, not silently empty DB |
| T-024 | R-024 | Every CLI command produces documented JSON fields, statuses, exit codes, errors and no secret echo |
| T-025 | R-025 | Restore backup to separate clean environment and compare checksums/registry/grants/revocation/audit invariants |
| T-026 | R-026 | Enumerate disabled MCP capabilities; no accidental handler execution and responses match declared behavior |

## Evidence record schema (documentation)
Every test report must contain: test ID; requirement IDs; planned/actual status; exact commit SHA; OS version/build; runner type MANUAL_LOCAL or SELF_HOSTED_ACTIONS; dependencies; invoker; absolute UTC execution time; full command transcript (redacted); expected and observed; output/artifact digests; pass/fail; defects and rerun references. A manual PASS is never called a GitHub Actions PASS.

## Phased gates
Phase 1: T-001..T-006, T-021,T-026. Phase 2: T-007..T-013,T-020,T-022. Phase 3: T-014,T-015 and full interoperability cross-client matrix. Phase 4: T-016,T-017,T-023,T-025 plus ratified performance tests. Phase 5: T-018,T-019,T-024 and complete regression matrix. No next phase if failures remain unless owner documents exception.

## Missing before freeze
Precise fixtures, executable test commands, MCP transcripts per revision, performance workload/percentiles, threat probe methods, supported hardware/OS matrix and verified local runner security are not complete. This matrix is an index, NOT an executable qualification specification.

## Detailed planned suites
- [Windows confinement qualification](security/windows-confinement-qualification.md), WQ-001 through WQ-012, require independently observed OS denials and build-specific evidence.
- [SQLite authority schema](contracts/sqlite-authority-schema.md), Q-DB-01 through Q-DB-08, specify integrity, writer races, atomic transactions, recovery and faults.
- [CLI contract](contracts/administrative-cli.md), CLI-Q-01 through CLI-Q-10, specify command output, owner authentication, key and corruption safety.
- [Full R/T traceability](PHASE0_REQUIREMENT_TRACEABILITY.md) preserves the R-001..026 and T-001..026 crosswalk.
All statuses remain PLANNED. Do not claim test PASS from documentation.
