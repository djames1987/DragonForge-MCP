# Threat model — FROZEN V1 DESIGN, runtime controls unqualified

## Assets and trust
Assets: client credentials, tool trust records, policy/grants, local files, audit history, execution results, installed native packages and owner approvals. Attackers may be local unprivileged processes under same OS user, malicious MCP clients, malicious downloaded packages, compromised workers, hostile browser origins, and malicious tool updates. A local administrator / fully compromised OS is OUTSIDE claimed isolation guarantees; document this explicitly.

| Threat ID | Abuse case | Required mitigation | Negative test |
|---|---|---|---|
| TH-01 | Client pretends to be another client | Unforgeable credential and gateway-verified identity, independent of display name | T-003 |
| TH-02 | Loopback DNS rebinding or hostile browser Origin | Bind loopback, strict Host and Origin evaluation, authentication | T-021 |
| TH-03 | Unauthorized discovery/invoke | Separate per-client discovery and execution checks | T-002,T-005 |
| TH-04 | Revoked client uses cached tool list | Recheck policy on each invocation | T-005 |
| TH-05 | Package changed after trust | Digest pin and fail-closed trust invalidation | T-009 |
| TH-06 | Worker reads gateway secrets/DB | OS-enforced isolation and minimal inherited handles | T-010 |
| TH-07 | Traversal/reparse point escape | Canonical scoped resource handles, deny path changes/races | T-022 |
| TH-08 | Client fabricates owner approval | Trusted approval provenance + nonce/digest/argument binding | T-013 |
| TH-09 | Worker crash or malformed IPC | bounded frames and crash isolation | T-011 |
| TH-10 | Concurrent calls corrupt permissions | transactional storage and exact key constraints | T-006 |
| TH-11 | Request replay duplicates write | scoped idempotency; no ambiguous side-effect auto-retry | T-016 |
| TH-12 | Malicious oversized payload | bounded size, queue and timeout; reject before parse | T-012,T-015 |
| TH-13 | Logs leak credentials | redact before emission, deny worker access to secret env | T-017 |
| TH-14 | Backup leaks authorization state | per-user ACL, protected backup location and restore qualification | T-025 |

## Isolation requirements
A subprocess, Python venv, cwd restriction, or job object on its own is NOT a general sandbox. On Windows qualify per-OS-edition token privileges, file/registry ACLs, network egress, child process limits, environment inheritance, named pipe ACLs, shared memory, symlinks/junctions, device access, and escape attempts. If any required containment invariant is unachievable, do not permit tools requiring that boundary. Tests must use a hostile fixture process and account-separated probes where possible; record host build, token rights, package digest and denied access evidence.

## Critical abuse paths
1. Hostile HTML page -> browser fetch to loopback -> Host/Origin validation -> reject before MCP dispatch; do not rely on CORS alone.
2. Tool called with `approval=true` -> ignore client claim -> gateway asks trusted local owner interface -> verify exact invocation -> permit only once.
3. Malicious plugin attempts to open SQLite / credential store / parent process -> OS enforcement denies; detection alone is not sufficient.
4. Simultaneous revoke and execution: admission requires transactionally consistent policy snapshot; revocation ordering must define whether already-admitted execution is cancelled, not pretend all external effects reversible.
5. Executable swapped between inspection and launch: verify digest of opened artifact immediately before execution and enforce non-writable active content paths.

## Residual risk and documentation gate
No claim is made that local-only equals secure or that cross-user Windows isolation is already tested. Threat severity, exploit fixture definition, exact security-control implementations and accepted residual risks require formal owner ratification and qualification.
