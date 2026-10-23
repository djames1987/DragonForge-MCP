# C-007B — Versioned administrative request and result schemas for CLI-001..054
Status: Phase 0 normative design proposal, not executable code. C-007A supplies exact command syntax/flags and owner authority. This document supplies machine schema constraints and operation-specific payload contracts.

## Common strict JSON Schema rules
Use JSON Schema 2020-12, `additionalProperties:false`, every named field required unless explicitly nullable. IDs UUID; digest 64 lowercase hex; timestamp RFC3339 UTC millisecond precision; nonnegative integer revisions; no arbitrary command execution or dynamic schema URLs. All command results share envelope:
```json
{"schema_version":1,"ok":true,"command":"CLI-020","correlation_id":"00000000-0000-4000-8000-000000000001","data":{},"warnings":[]}
```
On error omit data, include error {code,message}, no raw secret. Syntax flags turn into admin IPC args with canonical field names. Requests: {protocol:"dfmcp.admin",version:1,request_id,operation:"CLI-NNN",args:{},expected_revision:null|integer,deadline_ms:5000}. No owner SID/client trust assertion in args. JSON output always one object LF, no banners. Unknown/extra fields fail SCHEMA_INVALID. Typed standard shapes: Entity={id,revision,state,created_at}, Page={items:[Entity],next_cursor:string|null}, Validation={valid:boolean,errors:[{code,path,message}]}, Receipt={correlation_id,authority_epoch,revision}, Backup={backup_id,artifact_sha256,authority_epoch,verified:boolean}. Specific command success data requirements below. Each row inherits common envelope and typed syntax from C-007A.

| Command IDs | Required data field names and types |
|---|---|
| CLI-001..004 | 001 state:string,uptime_ms:int,instance_id:UUID,version:string,store_status:string; 002 state:string,process_id:int; 003 state:string,remaining_jobs:int; 004 state:string,process_id:int |
| CLI-005..007 | 005 config:object redacted,revision:int; 006 Validation plus proposed_diff:array; 007 revision:int,restart_required:boolean |
| CLI-008..010 | 008 client_id:UUID,credential_id:UUID,expires_at:timestamp,delivery_reference:string; 009 Page; 010 client:Entity,credentials_metadata:array,grants:array |
| CLI-011..016 | 011 state,revision; 012 state,revoked_at,authority_epoch; 013 credential_id,expires_at,delivery_reference; 014 Page; 015 revoked_at,authority_epoch; 016 replacement_client_id,delivery_reference |
| CLI-017..021 | 017 path:string,sdk_version:string; 018 digest:string,manifest:object,validation_errors:array; 019 package_id:UUID,version_id:UUID,state:'STAGED'; 020 Page; 021 manifest:object,state:string,capabilities:array,grants_count:int |
| CLI-022..026 | 022 state:'TRUSTED',revision; 023 state:'ENABLED',revision; 024 state:'DISABLED',revision; 025 state:'REVOKED',revision; 026 removed:boolean |
| CLI-027..029 | 027 grant_id:UUID,revision:int; 028 Page; 029 state:'REVOKED',revision:int |
| CLI-030..032 | 030 epoch:int,settings:object,defaults:object; 031 Validation; 032 policy_epoch:int |
| CLI-033..036 | 033 Page; 034 principal:UUID,tool:string,version:string,digest:string,argument_sha256:string,effect:string,capabilities:array,expires_at:timestamp,state:string; 035 approval_id:UUID,state:'APPROVED'; 036 approval_id:UUID,state:'DENIED' |
| CLI-037..038 | 037 Page plus anchor_hash:string; 038 integrity:string,events_checked:int,last_hash:string |
| CLI-039..041 | 039 ready:boolean,store:string,worker:string,config:string; 040 profile:string,build:string,qualification_status:string,limitations:array; 041 fixture:string,observed:object,artifacts:array |
| CLI-042..045 | 042 schema_version:int,user_version:int,integrity:string,store_state:string; 043 integrity:string,foreign_keys:string,migrations:array; 044 old_version:int,new_version:int,backup_id:UUID; 045 state:string,reconciliation:object |
| CLI-046..051 | 046 backup_id:UUID,artifact_sha256:string,verified:boolean; 047 Page; 048 verified:boolean,integrity:string,authority_epoch:int; 049 restore_operation_id:UUID,reconciliation:object; 050 key_id:string,export_path:string; 051 key_id:string |
| CLI-052..054 | 052 state:'MAINTENANCE'; 053 state:'READY'; 054 uninstalled:boolean,backup_id:UUID|null |

For every row above, exact enumerated values and nested objects must be versioned under a stable `schema_version`, with data keys not silently renamed. Future detailed JSON Schema files must use this contract as generation source; do not claim these field/type matrices constitute mechanically validated complete JSON Schema documents. **This is still a freeze blocker for owner-approval readiness.**

## One-time secret and backup recovery format
Credential delivery: Windows Credential Manager target `DragonForgeMCP/<instance>/<client>/<credential_id>`, created from owner-authenticated channel, ACL-scoped current owner, no credential in command argv, env, logfile or --json result. If secure delivery fails after DB commit, immediately revoke and audit.
Backup export: owner-interactive offline only; generate 256-bit content encryption key, AEAD AES-256-GCM each backup with unique 96-bit nonce, associated data {instance,schema_version,generation,authority_epoch,backup_id}; key wrapping uses DPAPI locally and separately an explicit owner passphrase envelope with Argon2id parameters pinned in backup format version. No weak password fallback; define exact Argon2id version/parameters before freeze; fail if supported implementation unavailable. Key export never prints secret; key file created on exclusive owner-controlled path with restrictive ACL and confirmed readback. Reject wrong key, AEAD failure or unknown version without destructive restore. No unattended sensitive approval.

## CLI test acceptance
For CLI-001..054, assert documented required keys/types, no extra sensitive fields, stable error/exit mapping, owner SID enforcement, noninteractive refusal, atomic mutation and audit. Invalid argv, path, JSON, stale revision, unknown command, revoked credential, owner token spoof, key loss and interrupted restore must each produce deterministic nonzero exit. Future qualification T-024; not executed.

## C-007E output semantics clarification (2026-10-23)
Machine-readable `command` MUST equal canonical operation `CLI-NNN`, not free-form command display text. For list operations, `items` is specialized per client/credential/tool/grant/approval/audit/backup record family. See updated `cli-command-schema-registry-v1.json`. Older text-only `command` examples are superseded. Distinguish JSON Schema fixture conformance from OS-authenticated command execution.

## C-007F — CAS precondition closure (2026-10-23)
For CLI-007,011,012,022,023,024,025,029,032 the administrative IPC `expected_revision` is REQUIRED as an integer >=1; `null` is invalid even though the field is always present in the common envelope. Read-only/no-CAS operations may use `null`. `deadline_ms` 100..30000 follows the root CLI `--timeout-ms`; CLI-003/004 `grace_ms` is 0..30000, CLI-044 migration `target` >=1. A stale/nonpositive revision must be rejected before semantic mutation and must not bypass the protected audit/receipt transaction. See updated JSON Schema registry. Earlier example fixtures using `null` for all operations are outdated and must regenerate appropriate revisions.
