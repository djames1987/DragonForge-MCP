# Phase 0 — MCP release verification and decision evidence (2026-10-21)

Status: **RESEARCH EVIDENCE ONLY — NOT OWNER RATIFICATION, NOT A NORMATIVE CONTRACT.**
Authority: `docs/DEVELOPMENT_RULES.md`, especially Rules 1–6, 15–16, 25–26.
Baseline inspected: `main` 9aa340d110bb31732911cf088ce1fd3bd213d00b; Phase 0 branch b12b9f4e6c52765028d43194e57777080781d8ea; PR #1 open/draft.

## Confirmed release evidence

1. The MCP maintainers announced the **2026-07-28 specification release**, dated July 28, 2026: https://blog.modelcontextprotocol.io/posts/2026-07-28/
2. The published release states that 2026-07-28 removes protocol-level `initialize` / `initialized` and `Mcp-Session-Id` semantics, introduces `server/discover` and carries request metadata on individual requests. Thus the existing C-010 warning against mixing 2025-11-25 and 2026-07-28 is valid.
3. The released revision describes `Mcp-Method` and, where applicable, `Mcp-Name` headers for Streamable HTTP POST. See https://modelcontextprotocol.io/specification/draft/changelog and the published announcement; the **version-pinned released normative pages** must be cross-checked before finalizing each field.
4. The TypeScript SDK migration guide explicitly discusses separate 2025-era and 2026-era codecs and opt-in negotiation: https://ts.sdk.modelcontextprotocol.io/v2/migration/support-2026-07-28 . SDK defaults are not a substitute for the normative specification.

## DEC-001 recommendation — owner approval needed

**Propose** normative MCP `2026-07-28` first; retain `2025-11-25` compatibility only as a separately described, separately tested adapter if concrete local client interoperability requires it. Avoid a hybrid handshake. Do not mark DEC-001 approved based on this recommendation.

| Concern | Modern 2026-07-28 candidate | 2025-11-25 compatibility candidate | Freeze requirement |
|---|---|---|---|
| Initialization | No protocol-level initialize exchange | Uses initialize / initialized lifecycle | Document exact request/reply transcripts for both selected modes |
| Transport session | No protocol-level session ID | Earlier session semantics may apply | Define authentication independently from session ID |
| Discovery | Modern server/discover | Legacy initialization capabilities | Validate method and capability advertisement |
| Authorization | Authenticate and authorize every request regardless of supplied clientInfo | Authenticate and authorize every request regardless of session or display name | Explicitly reject client-supplied identity elevation |
| Interoperability | Recent clients must opt into revision | Existing clients may assume older behavior | Record clients, versions and evidence |

## C-010 remaining normative gap register

- P-001: authoritative pinned URL and normative text for **every** supported request/response shape; verify each method and error against that text.
- P-002: explicit selected transport mapping for unsupported methods, invalid JSON-RPC IDs, notifications, and HTTP status/body/content-type combinations.
- P-003: independent authentication transport, token binding and bridge credentials; do not equate MCP `_meta` clientInfo with authenticated identity.
- P-004: per-era request examples for discovery, tools/list, tools/call, cancellation, malformed headers and invalid versions.
- P-005: compatibility test matrix with actual target client versions; no claim of compatibility before qualification.
- P-006: reconcile `docs/CONTRACTS.md`, `docs/DECISIONS.md`, `docs/contracts/mcp-protocol-compatibility.md`, `docs/QUALIFICATION.md` and ADR-001 using one selected profile.

## Verification classification

Confirmed here: external release announcement and high-level semantics. **Not** confirmed here: full normative wire contract, schema-by-schema conformance, working gateway behavior or any executed tests. No implementation was created. Phase 0 remains **DOCUMENTATION IN PROGRESS**.
