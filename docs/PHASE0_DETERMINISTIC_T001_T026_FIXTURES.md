# T-001..T-026 — Deterministic qualification fixture design, version 1
**Documentation ONLY.** Future release runner `dfmcp-qual.exe` described in Q-EXEC-001 has not been implemented. This file defines test *input and acceptance semantics*, not an executed test or runnable script. Required source version: `docs/QUALIFICATION_EXECUTION_CONTRACT_V1.md`; this document makes the default fixture matrix specific and forbids selecting uncontrolled real user files or network targets.

## Shared fixtures
- `fresh-empty`: disposable per-test instance UUID `00000000-0000-4000-8000-000000000010`, private temp root `%TEMP%\DFMCP-QUAL\<run-id>\T-NNN`, reference port 8765 or a recorded reserved loopback free port; no persisted real client.
- `two-clients`: synthetic principal A `00000000-0000-4000-8000-000000000011`, B `00000000-0000-4000-8000-000000000012`; local CSPRNG credentials delivered through protected test credential slots, never present as plaintext in logs/fixture manifests. Echo tool `sample.echo@1.0.0` with package digest computed from immutable fixture, DISCOVER+INVOKE granted only to A.
- `sqlite-atomic`: disposable DB seeded from **approved exact migration bytes**, two principals and one grant; deterministic failpoint indexes 0..N for each SQL write, reference policy epoch 1 and audit sequence 1.
- `package-states`: synthetic isolated immutable W0 echo tool with no file or network capability. State transitions independently asserted.
- `mcp-wire`: exactly pinned 2026-07-28 schemas and source; input JSON line canonical in fixture manifest; expected JSON-RPC ID must equal request ID; no initial `initialize` session. Header assertions sourced from official normative transport.
- `windows-w0`: dedicated disposable Windows 10 Home x64 build 19045 test account/profile, approved worker with no network and no filesystem rights except broker-provided scoped handles; any missing OS token/ACL/Job evidence yields INCONCLUSIVE/FAIL (not PASS).
- `stale-restore`: deterministic timeline E0 create client A, E1 grant tool, E2 backup, E3 revoke key, E4 restore E2 snapshot; verify no old credential/grant can invoke.
- Other fixtures derive from these and snapshot them *per test*, never share writable state between parallel test workers.

## Per-test manifest and expected behavioral oracle
| Test ID | Seed fixture | Deterministic challenge or test input | Expected terminal outcome |
|---|---|---|---|
| T-001 | fresh-empty | 127.0.0.1:8765; request 0.0.0.0 config | loopback listener only; broad bind refused |
| T-002 | two-clients | A with DISCOVER; B no grant; anonymous | A only sees allowed tool; B and anonymous see none |
| T-003 | two-clients | forge clientInfo A under B token; rotate A key | no privilege change; revoked/expired key denied |
| T-004 | mcp-wire | 2026-07-28 discover/list/call; 2025-11-25 handshake | modern accepted by specification; legacy refused |
| T-005 | grant-revoked | list before revoke then call cached ID | DENY before worker launch |
| T-006 | sqlite-atomic | fault before/after grant+audit transaction writes | transaction either all committed or none |
| T-007 | sample-tool | SDK-generated echo package, isolated client | install without editing gateway source |
| T-008 | package-states | STAGED->VALIDATED->TRUSTED->ENABLED->GRANTED | cannot invoke before all gates |
| T-009 | package-tamper | one byte mutated executable and manifest after trust | digest mismatch prevents execution |
| T-010 | windows-w0 | AppContainer worker probes DB, registry, network, pipe | OS access denial evidence for all forbidden resources |
| T-011 | worker-crash | kill isolated echo worker during call | gateway healthy, audited crash, no speculative retry |
| T-012 | schema-failure | invalid args, unknown field, oversize output | validation fails without side effect |
| T-013 | approval-replay | pending approval nonce expired/duplicate/mismatched | none accepted; one exact owner approval succeeds once |
| T-014 | client-isolation | two clients A/B disjoint grants; swap cached IDs | no cross-client outputs, secrets or tool visibility |
| T-015 | saturation | 4 running+5 queued; cancel two; expire one | bounded queue and accurate final states |
| T-016 | side-effect-crash | crash after external mutation, before receipt | UNKNOWN_EFFECT; no automatic duplicate write |
| T-017 | canary-redact | unique known canaries in credential and args | none appear in stdout, stderr, logs, audit |
| T-018 | lifecycle | fresh installation, upgrade, rollback, uninstall | preserve unless explicit purge; migration/rekey safe |
| T-019 | report-meta | capture exact git SHA, runner and artifacts | every test has correct signed/hashed manifest |
| T-020 | worker-ipc | Python/C++/Rust HELLO, READY, invoke, forged frame | same versioned wire; forged messages denied |
| T-021 | origin-host | Host spoof, Origin hostile, DNS rebound | HTTP denied before tool dispatch |
| T-022 | file-race | traversal, reparse/junction swap, rename race | no writes outside scoped root verified by file ID |
| T-023 | database-corrupt | truncate copy, migration checksum mismatch | RECOVERY_REQUIRED; no empty reset |
| T-024 | cli54 | 54 commands per operation success and failure | strict envelope, correct exit, no secret leakage |
| T-025 | stale-restore | backup before key revocation, restore offline | REKEY_REQUIRED; no credential resurrection |
| T-026 | unsupported-mcp | unsupported prompts/resources/notifications call | not advertised and safely refused |

## Exact implementation-time fixture manifest contract
Each future `fixtures/T-NNN/manifest.json` (created **only after owner authorizes implementation**) SHALL specify `schema_version:1`, `test_id`, `seed_fixture`, `expected_sha256` map for static fixture files, `required_platform`, `required_caps`, `setup_state`, `action_sequence` (ordered action+input+expected status), `assertions` (explicit JSON Pointer or OS audit identifiers), `cleanup_allowlist` and `expected_exit_code`. Every action has a timeout and strictly recorded correlation ID. No nondeterministic external websites; all network-denial probes use reserved local endpoints or intentionally unroutable documentation ranges with OS trace, never real remote scans. Fixture time is injected as virtual clock and represented in RFC3339 UTC in the manifest; worker race stages use synchronization barriers, not assumed sleep intervals.

## Reproducible future commands
For each T-NNN, from a **qualified approved exact SHA**:
`dfmcp-qual run --test T-NNN --fixture default --out <absolute-disposable-evidence-directory>`.
To aggregate results: `dfmcp-qual verify --run <absolute-disposable-evidence-directory> --json`. Exit 0=all mandatory assertions proven; exit 1=failed behavior; exit 2=invalid or uninitialized fixture; exit 3=unsupported platform, which is **not a PASS**. No production applications or user credentials touched. These are published interface specifications for the future runner, not currently executable commands.

## Cleanup and evidence
Before fixture setup verify test root under disposable temp location, owner identity and baseline hashes. After every run, terminate only launched Job Object PIDs, close named pipes, restore/revoke only synthetic users/keys, remove only manifest enumerated temporary files and AppContainer profiles, never wildcard-delete any real user profile or registry hive. Keep artifacts under owner-private evidence archive with source SHA, Windows edition+UBR, runner mode MANUAL_LOCAL or SELF_HOSTED_ACTIONS, OS token/ACL/ETW outcome, exit code, auditor decision, before/after hash and redacted stdout/stderr. Unknown leftover fixture state -> FAIL and quarantine, not deletion outside allowlist.

## Completion limitation
These fixture specifications are detailed but still require review against individual T contract definitions for exact data serialization and approved future qualification-harness behavior. Phase 0 does not require running these unimplemented runtime tests, but it does require unambiguous input/output protocols. Keep this as owner-review candidate until independent consistency audit signs off.
