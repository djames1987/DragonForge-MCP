# Phase 0 — Owner-manual CLI request schema validation evidence (2026-10-23)

Evidence furnished by owner from Windows PowerShell in `C:\DragonForge-MCP`, using isolated `.phase0-validation\Scripts\python.exe` and Python `jsonschema` Draft202012Validator. This report records **user-supplied manual results**, not independent GitHub Actions execution. Source branch `docs/phase0-architecture-baseline-20261021`; repository head observed by GitHub connector before recording: `a3ad1dfea17510a33e58a1445e5a7b1c8b0660b4`. The user's validation command was based on the previously supplied schema-validation procedure; its actual `git rev-parse HEAD` was not printed, so an **exact executed checkout SHA is not independently demonstrated**.

## Observed outcomes

The owner reported CLI-001 through CLI-054 each `PASS`, followed by:

```text
CLI validation: 54/54 passed
CLI REQUEST SCHEMA VALIDATION: PASS
```

The validation procedure checks the Draft 2020-12 schema's metaschema, generates one valid required-argument request per operation (UUID/digest/enum/date patterns), checks each is accepted, then injects `unexpected_field` into each `args` and checks rejection. All 54 were reported accepted/rejected as intended.

## Scope and limitations
**PASS only for the 54 request fixtures and unknown-argument rejection checks.** Not a comprehensive proof of every accepted/rejected value, especially optional fields, boundary constraints, cross-field invariants, interprocess owner token, interactive confirmation, backup encryption, real CLI execution, or **response schema** validation. No authored application code, harness, workflow or CI run is implied. Follow-up: independently validate 54 success and standard error response fixtures; record exact tested SHA and package versions; reconcile dynamically typed fields with C-007A/C-007C. Preserve no-code-until-freeze rules.
