# C-002A — Byte-accurate immutable package integrity and installation
**Status:** SELECTED — AGENT-DELEGATED TECHNICAL DETAIL; Phase 0 documentation only, not an architecture freeze.  
**Authority:** DEVELOPMENT_RULES.md (especially 1–6, 16–20, 22, 24–26); DEC-006 owner choice; C-002, C-005, C-007; R-007–R-010, R-022; T-007–T-010, T-022.

## 1. Problem and decision
A JSON reserialization-based fingerprint could fail to notice whitespace changes, and a directory hash without a byte-level framing contract invites ambiguity. Select a **byte-exact, path-framed, SHA-256 package digest**, where an internal digest field is replaced with a fixed-width zero placeholder **at a precisely located byte range only while calculating the digest**. All other original bytes, including comments where permitted, formatting, ordering and line endings, are included. For simplicity, manifests themselves are strict UTF-8 JSON, without comments or duplicate keys. No archive extraction or executing code occurs while calculating the digest.

A *single* fixed manifest path `manifest.json` is required at package root. The 64-byte ASCII lower-case hex `digest_sha256` value must be present **exactly once** as a top-level JSON string property; the parser must reject duplicate property names at every object depth and reject Unicode-escaped property names aliasing the digest key. For hash computation, replace only the 64 ASCII value bytes of the digest string with 64 ASCII `0` bytes; preserve all other raw manifest bytes. A JSON parser with byte-span tracking must establish that this is the literal top-level property value, not a substring match. Escape sequences or non-ASCII encoding in that value are rejected. This fixed-width substitution avoids circular self-hashing without canonicalizing the rest of the manifest.

The trusted package identity is `(tool_id, version, digest_sha256)`; SHA-256 is an integrity checksum tied to explicit owner trust, **not a signature, provenance guarantee, or malware scan**. Capability grants never follow a new digest automatically.

## 2. Package path and input grammar
- Input: one already-staged package directory, never an implicitly downloaded package or arbitrary archive. Staging and trusted execution directories must be inaccessible for writing by tool workers and ordinary MCP clients.
- Package paths are relative UTF-8 names using `/` as the only separator. Allowed each segment: ASCII `[A-Za-z0-9._-]` with 1–100 characters; reject `.`, `..`, trailing dot or space, leading dot for initial version, reserved Windows device names case-insensitively (including extensions), colon, drive/UNC/absolute paths, backslash, alternate data streams, and path length over 240 UTF-8 bytes.
- Reject any path having a case-insensitive collision under invariant ASCII case-folding. Reject empty directories from the digest-relevant package format (they are neither accepted nor preserved). Require at most 512 regular files and aggregate content <= 64 MiB; each file <= 16 MiB and manifest <= 256 KiB. Inputs exceeding a limit are rejected before execution or trust.
- Reject reparse points, symlinks, junctions, mountpoints, hardlinked files, devices, named streams and all non-regular files. The scanner must inspect opened file handles, not rely solely on path strings or an earlier listing. File identity/link count and reparse checks are required both before and after reading; any uncertainty causes rejection. Any file set or metadata that changes while scanning causes rejection.
- Manifest entry point must reference one included regular file. No install-time scripts, interpreter hooks, manifest-controlled network URLs, environment expansion, or absolute paths.
- A tool's full package consists of **all** regular files recursively below its root, including optional documentation, dependency files and the manifest itself. No ignored files, glob exclusions or implicit lockfile rewriting. If a file exists in the staged directory but cannot be enumerated or read, installation fails closed.

## 3. Digest byte stream, version 1
Hash these bytes in exactly this order; all integers are unsigned 64-bit **big-endian**:
1. ASCII `DFMCP-PACKAGE-V1\0`.
2. `uint64(file_count)`.
3. For each file, sorted by raw ASCII bytes of the validated relative path (case-sensitive), append:
   - `uint64(path_byte_length)`
   - path UTF-8 bytes with forward slashes;
   - `uint64(content_byte_length)`
   - exact file content bytes. For `manifest.json` alone, substitute 64 ASCII zero bytes at the validated top-level `digest_sha256` string-value span; byte length stays unchanged.
4. No final terminator, no timestamps, permissions, file system metadata, archive containers or JSON canonical re-encoding.

All lengths describe the actual contents, and integer overflow is an error. All package readers and SDK tooling must use this identical format. A tooling mismatch fails validation; it must not be auto-fixed by silently rehashing. The computed 32-byte SHA-256 is formatted as 64 lower-case hexadecimal characters and compared with the manifest field in constant time. Any mismatch fails validation.

A standalone owner trust record additionally binds the computed digest to *all parsed security-relevant manifest fields*, schema version, package identifier and immutable installation location. The digest algorithm version is a separate persisted field `package_hash_algorithm = "DFMCP-PACKAGE-V1-SHA256"`. An unknown algorithm is unsupported, never guessed.

## 4. Filesystem and TOCTOU enforcement
Stage under an owner-owned, restrictive directory; read and validate using handles opened without following reparse paths, and refuse hardlinks. Copy into a new random private quarantine directory through already validated handles, not via untrusted original path lookups; then independently rescan and rehash the **quarantine copy**. Atomically rename that directory into a digest-keyed immutable package location on the same volume. If atomic rename or ACL creation fails, leave it untrusted and unusable. Only the trusted gateway/owner maintenance service may read the immutable store, and only owner-authorized maintenance may remove a version. Never load a worker directly from the mutable staging location.

At every invocation, check current trust/enablement/grant state and immutable package identity. Establish safe read handles to files under the package root and confirm digest against the trust record **before worker launch**. To avoid a use-after-check substitution, launch using the protected snapshot and stable opened file identities; on platforms where secure loader semantics cannot be demonstrated, deny execution with `DIGEST_MISMATCH` or `ISOLATION_UNAVAILABLE`, never accept an unsafe fallback. The worker must have no write access to that snapshot. Hashing at install time alone is not an execution-time integrity guarantee.

## 5. Lifecycle and transaction contracts
States:
`STAGED -> VALIDATED -> TRUSTED -> ENABLED`; `ENABLED -> DISABLED`; `TRUSTED|DISABLED|ENABLED -> REVOKED`; `REVOKED|DISABLED -> REMOVED` after reference checks. `VALIDATED -> STAGED` only by replacing the entire staged candidate and resubmitting validation. Failures are recorded without promotion.

- **install** creates a staged record only; cannot execute or grant.
- **validate** compares byte-exact hash, path grammar and offline JSON Schemas; no import/activation of tool code.
- **trust** requires independently authenticated owner confirmation of ID, SemVer, full digest, effect and capabilities; commit trust and audit atomically.
- **enable** is separate from trust and only for a currently trusted exact digest.
- **grant** binds authenticated principal, tool/digest, discovery or invocation separately, and subset of declared scopes. A grant alone never enables a tool.
- **revoke** removes invocability at next admission and prelaunch recheck, invalidates waiting approvals, and safely cancels queued invocations. Previously completed side effects cannot be rolled back by revocation.
- **remove** is prohibited while active worker, pending approval, queued call, backup dependency or other durable reference requires content. No silent deletion of audit-referenced identity.
- Trust is never inherited across reinstallation, version increments, byte changes, altered schemas, backup restore or digest-algorithm revisions.

Atomic durable state changes are done only through gateway-owned C-005 transactions with C-009 audit. Crash during copying leaves an inert quarantine directory eligible for reviewed cleanup; crash after committed trust must not create an executable path without matching immutable files.

## 6. Exact negative and positive qualification fixtures (planned, not run)
| Fixture | Procedure concept | Expected |
|---|---|---|
| PKG-001 | Hash a minimal known fixture twice with independent Python and Rust implementations | Identical digest; test vectors to be frozen before Phase 1 |
| PKG-002 | Alter one raw manifest whitespace byte, with no rehash | DIGEST_MISMATCH |
| PKG-003 | Alter one code or README byte | DIGEST_MISMATCH |
| PKG-004 | Insert different filenames or reorder OS enumeration | Hash changes for inserted path; same hash across enumeration order |
| PKG-005 | Duplicate top-level or nested JSON key; escaped digest-key alias | SCHEMA_INVALID |
| PKG-006 | Replace package path with junction/symlink/hardlink/ADS or swap during copy | Validation/install denied |
| PKG-007 | Swap staged directory between validation and trust | Trusted bytes must still be quarantined verified snapshot or deny |
| PKG-008 | Mutate immutable installed file at use time | No worker start; audit denial |
| PKG-009 | Try install -> invoke before separate trust, enable or grant | ACCESS_DENIED |
| PKG-010 | Revoke grant after listing and while queued | No subsequent worker launch |
| PKG-011 | New digest with same tool_id and SemVer | Requires new trust and grants; old trust cannot authorize it |
| PKG-012 | Interrupt copy, rename, trust write or audit transaction | No partially trusted executable package |

For every fixture, the eventual test procedure must record exact SHA, Windows build, test identity, preconditions, commands, file digests, observed result, audit correlation and cleanup. These rows are **not** claims that any test has run.

## 7. Rejected alternatives, impacts and residual risks
Rejected: semantic JSON canonicalization (does not pin every raw byte); naive concatenate-file hash (ambiguous framing); filename-only trust (unsafe replacement); mandatory publisher PKI (contrary to DEC-006); running directly from mutable staging (TOCTOU); following symlinks (scope escape).

Impact: SDK packagers must emit the exact placeholder-compatible manifest and deterministic digest; C-002/C-005/C-007/C-009, architecture, threat model, T-008/T-009/T-022 and Phase 2 implementation must adopt this contract together. Future hashing changes require a new algorithm ID, explicit migration and independent owner re-trust; rollback retains previously trusted bytes **only where current policy and grants still permit them**. Residual risks include compromised owner/admin privileges, malicious code in correctly trusted bytes, filesystem/loader vulnerabilities and weakness of unverified worker isolation. None is eliminated by SHA-256. Exact stable Windows loader-handle procedures and independent byte-level test vectors are remaining qualification-specification dependencies, not proven security controls.
