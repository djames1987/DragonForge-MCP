# C-010C — Immutable upstream MCP and conformance source pin
**Source identification verified through GitHub repository API on 2026-10-22. Documentation-only.** DEC-001 remains MCP `2026-07-28` only.
 
## Immutable source references
MCP specification upstream repository `modelcontextprotocol/modelcontextprotocol`, commit `80041ab0a139a8f4fb3adbc99f7ae9113ea59671` (upstream commit dated 2026-10-20). At that immutable commit:
- `schema/2026-07-28/schema.json`: git blob `213c58f6d9a1c2ce6ad055afe90bbdb095a29ee8`;
- `schema/2026-07-28/schema.ts`: git blob `9b55feeb412bc3ae877f2eac10b5c01ba29a2eed`.
Immutable URLs: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/80041ab0a139a8f4fb3adbc99f7ae9113ea59671/schema/2026-07-28/schema.json and analogous schema.ts. The 2026-07-28 version directory is explicit, not the unversioned /draft schema.

Official conformance repository candidate: `modelcontextprotocol/conformance` revision `c37eec888e1c6ff140af79987a40008548b7cc5f` retrieved as branch main HEAD during source check. **This is an immutable source reference, NOT proof the revision contains executable cases for 2026-07-28 or a passing compatibility run.** Verify test taxonomy/version applicability before freeze and do not silently adapt older-era fixture expectations.

## Source authority and protocol constraints
Use exact 2026-07-28 normative JSON Schema and released specification over handwritten contract examples. C-010A and C-010B are application guidance only and must be amended if source comparison disagrees. Server MUST implement `server/discover` even though client calling it before tools is optional; no 2025 `initialize` fallback and no MCP sessions. HTTP routing headers and parameter annotation behavior follow the pinned transport, not earlier manually devised fixtures.

## Planned transcript verification (not executed)
- MCP-P01 JSON-RPC `server/discover` valid input/response; MCP-P02 `tools/list` zero-visibility; MCP-P03 `tools/call` with correct name and body; MCP-P04 STDIO request + response newline framing; MCP-P05 known error including unsupported protocol; MCP-P06 HTTP annotated scalar param/header matching; MCP-P07 header encoding and cross-field duplicate/invalid Unicode; MCP-N01..14 negative matrix from C-010B.
- For every fixture expected JSON instance must validate against `schema.json` from pinned blob; use conformance tool only if version support confirmed; record exact spec blob and conformance commit with test evidence.
- A static source pin is documentary completion only. No HTTP/STDIO service or conformance test execution has occurred in Phase 0.

## Remaining material questions
Whether this conformance revision explicitly covers the 2026-07-28 revision, and whether every example passes the pinned schema, require a review of that repo's test manifest and complete concrete vectors. Do not declare those PASS merely because the source revisions are pinned.
