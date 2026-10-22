# C-007A — Administrative CLI command and JSON interface contract
**Status:** agent-delegated proposed contract, not frozen. Windows 10 first. No executable CLI or workflows created during Phase 0. Supports R-002/003/006/007–009/013/016/018/019/023–025, T-002/003/006/007–009/013/016/018/019/023–025. This contract refines C-007, not MCP transport.

## Protocol, identity and uniform behavior
Executable name `dfmcp`. All privileged operations use the authenticated owner-only named pipe; no mutating CLI calls via MCP. Authenticate actual OS peer token and expected owner SID, reject elevated/unverified/foreign token mismatches rather than trust user string, PID, `--owner` or environment variable. Diagnostics that display sensitive state require same channel. `--socket` / `--pipe` cannot override trust policy or select arbitrary remote addresses.
Global flags: `--json` (machine-readable object), `--config <path>` (owner-safe, must not bypass secure configuration validation), `--timeout-ms <integer>` (100–30000 default 5000), `--no-color`, `--help`, `--version`. Unknown flags, duplicate singleton flags, duplicate JSON keys and ambiguous abbreviations rejected. All IDs validated before IPC. `--yes` is **never accepted** for trust, owner approvals, restore, key export, credential recovery or destructive operations; interactive owner confirmation mandatory. Noninteractive sensitive mutations return exit 4.
On successful `--json`, stdout contains exactly one UTF-8 JSON object followed by LF:
```json
{"schema_version":1,"ok":true,"command":"tool list","correlation_id":"<uuid>","data":{},"warnings":[]}
```
On failure:
```json
{"schema_version":1,"ok":false,"command":"tool trust","correlation_id":"<uuid>","error":{"code":"ACCESS_DENIED","message":"Action not permitted"},"warnings":[]}
```
No log banner/progress/color on JSON stdout. stderr reserved for local transport/runtime diagnostics without secrets; errors contain safe stable code and no raw payload. Timestamp fields RFC3339 UTC milliseconds; binary digest shown as 64 lowercase hex; IDs lowercase UUIDs; array ordering deterministic by identifier unless documented. Sensitive credential issuance uses protected one-time interactive output, never persists plaintext in reports or audit or process command arguments. `--json` issuance returns *credential metadata only* and an owner-approved secure OS credential-store reference; no bearer secret in machine-readable history.

Exit codes: 0 success; 2 syntax/validation; 3 not found; 4 unauthorized/owner confirmation denied; 5 version/revision conflict; 6 service unavailable or timeout; 7 action failed, including policy/worker denial; 8 recovery required or integrity failure. `--help` and `--version` exit 0 without a privileged connection. All mutation success/failure is audited; failed unauthenticated pipe negotiation is recorded through bounded safe security event reporting. Retries are not automatic for non-idempotent commands. User may send `--request-id <UUID>` to idempotent mutation commands only; identical canonical command and actor replays return existing result, mismatched payload returns conflict.

### Argument grammar
`<client>` UUID; `<tool>` namespaced lowercase dotted ID; `<version>` SemVer; `<digest>` 64 lowercase SHA256 hex; `<grant-id>` UUID; `<approval-id>` UUID; `<path>` explicit absolute path validated by owner-only controller, not trusted solely from CLI string; `<scope-json-file>` local UTF-8 strict I-JSON file with canonicalization, no inline secret JSON. `--limit` 1–500 default 50; `--cursor` opaque continuation token; `--format` limited to documented formats. Every list emits `data.items` and `data.next_cursor` (null at end). No command inherits grants from caller identity.

## Complete v1 command inventory
Every row is normative proposed syntax; flags after `--` not supported. Listed read commands are idempotent, mutate commands require expected revision (`--if-revision <int>`) where applicable, or an explicit conflict error.

| ID | Syntax / required flags | Authority | Success data keys; effects / error classes |
|---|---|---|---|
| CLI-001 | `dfmcp server status [--json]` | owner read | `state,uptime_ms,instance_id,version,store_status`; unavailable 6 |
| CLI-002 | `dfmcp server start [--json]` | owner mutation | `state,process_id`; start only validated local config; conflicts 5 |
| CLI-003 | `dfmcp server stop [--grace-ms 0..30000] [--json]` | owner mutation | `state,remaining_jobs`; closes admission, cancellation, WAL flush; failure 7 |
| CLI-004 | `dfmcp server restart [--grace-ms N] [--json]` | owner mutation | `state,process_id`; non-idempotent, preserves durable state; 6/7 |
| CLI-005 | `dfmcp config show [--json]` | owner read | `config,revision` with all secrets redacted |
| CLI-006 | `dfmcp config validate --file <path> [--json]` | owner read | `valid,errors,proposed_diff`; cannot apply; invalid 2 |
| CLI-007 | `dfmcp config apply --file <path> --if-revision N [--json]` | owner mutation | `revision,restart_required`; transactional audit, conflict 5 |
| CLI-008 | `dfmcp client enroll --name <name> [--json]` | owner mutation | `client_id,credential_id,expires_at,delivery_reference`; unique name, no grants |
| CLI-009 | `dfmcp client list [--limit N] [--cursor X] [--json]` | owner read | `items,next_cursor`; secret-free |
| CLI-010 | `dfmcp client show <client> [--json]` | owner read | `client,credentials_metadata,grants`; missing 3 |
| CLI-011 | `dfmcp client disable <client> --if-revision N [--json]` | owner mutation | `state,revision`; denies new admission |
| CLI-012 | `dfmcp client revoke <client> --if-revision N [--json]` | owner mutation | `state,revoked_at,authority_epoch`; permanent tombstone |
| CLI-013 | `dfmcp client rotate <client> [--overlap-hours 0..24] [--json]` | owner mutation | `credential_id,expires_at,delivery_reference`; 90-day lifetime; 24h max overlap |
| CLI-014 | `dfmcp credential list --client <client> [--json]` | owner read | `items,next_cursor`; no verifiers |
| CLI-015 | `dfmcp credential revoke <credential-id> [--json]` | owner mutation | `revoked_at,authority_epoch`; immediate recheck |
| CLI-016 | `dfmcp client recover <client> [--json]` | owner sensitive | `replacement_client_id,delivery_reference`; re-enroll, never resurrect old grants |
| CLI-017 | `dfmcp tool scaffold --id <tool> --output <path> [--language python|cpp|rust] [--json]` | owner local authoring | `path,sdk_version`; generates future authored package; never installs or executes |
| CLI-018 | `dfmcp tool validate --path <path> [--json]` | owner read | `digest,manifest,validation_errors`; byte-exact C-002A; no execution |
| CLI-019 | `dfmcp tool install --path <path> [--json]` | owner mutation | `package_id,version_id,state=STAGED`; quarantine, no grants |
| CLI-020 | `dfmcp tool list [--limit N] [--cursor X] [--json]` | owner read | `items,next_cursor` |
| CLI-021 | `dfmcp tool show <tool> --version <version> --digest <digest> [--json]` | owner read | `manifest,state,capabilities,grants_count` |
| CLI-022 | `dfmcp tool trust <tool> --version <version> --digest <digest> --if-revision N` | owner sensitive | `state=TRUSTED,revision`; exact owner review; fingerprint and scopes |
| CLI-023 | `dfmcp tool enable <tool> --version <version> --digest <digest> --if-revision N` | owner mutation | `state=ENABLED,revision`; trust required |
| CLI-024 | `dfmcp tool disable <tool> --version <version> --digest <digest> --if-revision N` | owner mutation | `state=DISABLED,revision`; new invocations blocked |
| CLI-025 | `dfmcp tool revoke <tool> --version <version> --digest <digest> --if-revision N` | owner mutation | `state=REVOKED,revision`; pending/queued invalidated |
| CLI-026 | `dfmcp tool remove <tool> --version <version> --digest <digest>` | owner sensitive | `removed=true`; blocks if references or active jobs |
| CLI-027 | `dfmcp grant add --client <client> --tool <tool> --version <version> --digest <digest> --kind discover|invoke [--scope-file <path>] [--json]` | owner mutation | `grant_id,revision`; no implicit enable |
| CLI-028 | `dfmcp grant list [--client <client>] [--json]` | owner read | `items,next_cursor` |
| CLI-029 | `dfmcp grant revoke <grant-id> --if-revision N [--json]` | owner mutation | `state,revision`; audited tombstone |
| CLI-030 | `dfmcp policy show [--json]` | owner read | `epoch,settings,defaults` |
| CLI-031 | `dfmcp policy validate --file <path> [--json]` | owner read | `valid,errors`; dry validation |
| CLI-032 | `dfmcp policy apply --file <path> --if-revision N [--json]` | owner mutation | `policy_epoch`; invalidates stale approvals |
| CLI-033 | `dfmcp approvals list [--state pending|approved|all] [--json]` | owner read | `items,next_cursor`; 50 default |
| CLI-034 | `dfmcp approvals show <approval-id> [--json]` | owner read | `principal,tool,version,digest,argument_sha256,effect,capabilities,expires_at,state` |
| CLI-035 | `dfmcp approvals approve <approval-id>` | owner sensitive | `approval_id,state=APPROVED`; interactive nonce confirmation, exact binding |
| CLI-036 | `dfmcp approvals deny <approval-id>` | owner sensitive | `approval_id,state=DENIED`; terminal, interactive |
| CLI-037 | `dfmcp audit query [--from <utc>] [--to <utc>] [--client <id>] [--limit N] [--cursor X] [--json]` | owner read | `items,next_cursor,anchor_hash`; redacted |
| CLI-038 | `dfmcp audit verify [--json]` | owner read | `integrity,events_checked,last_hash`; failure exit 8 |
| CLI-039 | `dfmcp diagnostics health [--json]` | owner read | `ready,store,worker,config` |
| CLI-040 | `dfmcp diagnostics isolation [--json]` | owner read | `profile,build,qualification_status,limitations`; never implies passed test from configuration |
| CLI-041 | `dfmcp diagnostics run-probe --fixture <WQ-id> [--json]` | owner test-only | `fixture,observed,artifacts`; forbidden in production; unimplemented until authorized tests |
| CLI-042 | `dfmcp db status [--json]` | owner read | `schema_version,user_version,integrity,store_state` |
| CLI-043 | `dfmcp db check [--full] [--json]` | owner read | `integrity,foreign_keys,migrations`; does not repair |
| CLI-044 | `dfmcp db migrate --target <int> [--json]` | owner sensitive | `old_version,new_version,backup_id`; validated maintenance mode |
| CLI-045 | `dfmcp db recover --operation <id> [--json]` | owner sensitive | `state,reconciliation`; no silent DB reset |
| CLI-046 | `dfmcp backup create [--json]` | owner mutation | `backup_id,artifact_sha256,verified`; AEAD local encrypted |
| CLI-047 | `dfmcp backup list [--json]` | owner read | `items,next_cursor` |
| CLI-048 | `dfmcp backup verify <backup-id> [--json]` | owner read | `verified,integrity,authority_epoch` |
| CLI-049 | `dfmcp backup restore <backup-id> [--json]` | owner sensitive/offline | `restore_operation_id,reconciliation`; refuse stale authority rollback |
| CLI-050 | `dfmcp backup export-key --destination <path>` | owner sensitive/offline | `key_id,export_path`; separate owner confirmation, secure export, no stdout secret |
| CLI-051 | `dfmcp backup import-key --source <path>` | owner sensitive/offline | `key_id`; key validation/owner recovery; no auto-grants |
| CLI-052 | `dfmcp maintenance enter [--json]` | owner mutation | `state=MAINTENANCE`; stop admissions, reconcile jobs |
| CLI-053 | `dfmcp maintenance exit [--json]` | owner mutation | `state=READY`; only verified store/runner |
| CLI-054 | `dfmcp server uninstall --preserve-data|--purge-data` | owner sensitive/offline | `uninstalled,backup_id`; mutual exclusion required, explicit confirmation |
 
## Command categories and defaults
Read-only commands never mutate access policy except bounded safe audit of authenticated administrative reads; sensitive reads still cannot disclose raw credentials. Mutation commands require gateway `READY` unless explicitly permitted in `MAINTENANCE` or `RECOVERY_REQUIRED`: database check, verify, recover, backup verification/restore, key import/export and diagnostics. Administrative writes fail closed when audit store unavailable.
Tool trust, enable, grant and approvals are separate atomic operations; `tool validate` does not import arbitrary code or run hooks. Client recovery does not restore old grants. `server start` never silently migrates an unknown schema. Backup restore requires offline maintenance + independent anti-rollback reconciliation, is never performed over a live connection.

## Negative contract requirements and tests
CLI-Q-01 all 54 commands have help schema and documented `--json` output envelope. CLI-Q-02 error status table and exit codes exact. CLI-Q-03 wrong owner SID, spoofed PID, AppContainer worker and stale revision rejected. CLI-Q-04 JSON output contains no credential/key/argument leak. CLI-Q-05 concurrent approve/deny/revoke and duplicate request IDs safe; at most one approval consumption. CLI-Q-06 interrupted backup/restore/migration rolls back or enters explicit recovery. CLI-Q-07 tool lifecycle cannot skip trust/enable/grant; same SemVer changed digest requires fresh owner trust. CLI-Q-08 path traversal/unsafe extension/config path rejected. CLI-Q-09 noninteractive sensitive operations rejected. CLI-Q-10 versioned stable output across supported native clients. Evidence fields follow `docs/QUALIFICATION.md`. **Not executed during Phase 0.**

## Deferred elements and change-control
Full JSON Schema for each `data` structure, exact per-command interactive text, fixture tools/harness behavior, key export cryptographic envelope and Windows service manager integration need final freeze reconciliation. No reliance on secret-bearing CLI command-line flags. Later Linux/macOS admin peers use separate OS-authenticated backends, not weakened Windows checks. Distinguish CLI command ID from contract ID and from qualification ID.
