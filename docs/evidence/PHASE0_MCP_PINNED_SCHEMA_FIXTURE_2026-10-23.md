# Phase 0 pinned MCP JSON Schema specimen validation (2026-10-23)

**Execution mode:** MANUAL_LOCAL via owner-requested Local Debugger; not GitHub Actions. Windows 10 build 19045, local Python virtual environment at `C:\DragonForge-MCP\.phase0-validation\Scripts\python.exe`; job `8eeec229-a107-4254-8d91-e1d6f53c4ac6`, exit=0, stderr empty, elapsed=0.75s. Script fetched immutable upstream JSON Schema at `https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/80041ab0a139a8f4fb3adbc99f7ae9113ea59671/schema/2026-07-28/schema.json` and used Draft202012Validator with root `$defs` context. Temporary inline Python code only, not committed.

## Exact observed output
```text
ListToolsRequest PASS
ListToolsResult PASS
CallToolRequest PASS
CallToolResult PASS
DiscoverResult PASS
```

Validated specimens: tools/list with per-request _meta, empty private scoped tools result including cacheScope/ttlMs, tools/call name/arguments with _meta, text success CallToolResult, tools-only server/discover result including capabilities/supportedVersions/cacheScope/ttlMs. **PASS for these five JSON Schema specimens, not universal MCP conformance**. This checks JSON object structure, not HTTP routing headers, request-scoped SSE, real server execution, negative transcripts, client auth, suite score or DNS rebinding defenses.

Reference project PR head before this evidence commit: `81e48a66358a472194fca5e10f15c432a1cf88d1`. This job validates inline specimen objects against upstream schema; its subject is the pinned spec + documented example structures, not a built DragonForge server. Source-independent conformance release anchor and scope caveats in independent audit remain binding.

## Scope caveat
The official frozen `modelcontextprotocol/conformance` file `requirements/2026-07-28.yaml` contains scored server features outside v1's intentionally advertised tools-only set (resources, prompts, input-required, SSE). Full official server score cannot be claimed without implementing/qualifying those. No owner Phase 0 freeze approval or Phase 1 implementation permission implied.
