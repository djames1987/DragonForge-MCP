# MCP protocol compatibility contract — PROPOSED v0.1

**Identifier:** C-010. **Authority:** Phase 0 proposal only. Uses owner-selected DEC-001 modern-only profile and `docs/DEVELOPMENT_RULES.md`. No implementation permission.

## Normative sources and pinned candidate
Selected initial release protocol baseline: **2026-07-28** (released specification). Consult the versioned normative specification and schemas at https://modelcontextprotocol.io/specification/2026-07-28 and https://github.com/modelcontextprotocol/modelcontextprotocol . A release blog is explanatory, **not** a substitute for exact versioned normative text. Official July 28 announcement: https://blog.modelcontextprotocol.io/posts/2026-07-28/ . Historical comparison: 2025-11-25 at https://modelcontextprotocol.io/specification/2025-11-25 . Before ratification, record exact spec URL, schema commit and conformance-suite revision, validate accessible normative paragraphs and test each wire example.

## Design-choice boundary
Owner-selected initial release: one modern `2026-07-28` implementation, with **no** 2025 compatibility adapter, no implicit fallback and no mixed-era negotiation. Future legacy support requires separate owner-approved scope and completed conformance tests. Never advertise an unimplemented protocol version. Client-provided `_meta` identity is descriptive, never an authentication assertion. Authorization is always by C-001's gateway-verified principal.

## Version behavior table

| Aspect | 2026-07-28 proposed modern adapter | 2025-11-25 EXCLUDED from initial release (comparison only) |
|---|---|---|
| Establishment | Stateless JSON-RPC requests; no initialize handshake | initialize negotiation followed by initialized notification |
| Discovery | Implement server/discover according to modern normative contract | No substitution of server/discover for initialize |
| Protocol version | MCP-Protocol-Version on applicable HTTP requests, checked consistently | Negotiate protocolVersion during initialize and obey legacy request rules |
| Protocol session | No Mcp-Session-Id protocol sessions | Legacy session support only if enabled per selected transport profile |
| Routing | Verify modern Mcp-Method and Mcp-Name headers against JSON-RPC body where required by spec | Never require modern routing headers in legacy mode |
| Tool visibility | Always apply gateway discovery authorization to tools/list | Same local security control, legacy wire encoding |
| Invocation | tools/call checked against current principal/grants/tool digest | Same gateway policy, legacy wire encoding |
| Identity | Credentials issued/verified under C-001, per request | Same local identity boundary; sessions do not grant permissions |
| Extensions | Do not advertise or execute unsupported extensions | Do not pretend legacy experimental features are modern extensions |
| Failure | Validate version/header/body per normative modern transport and JSON-RPC error mapping | Version-specific initialization, transport and error rules |

This table identifies semantic differences, not the complete normative wire specification. Missing normative packet-level details are freeze blockers.

## Gateway execution invariants
1. Transport reader enforces a bounded request size **before** JSON parsing; reject duplicate JSON object keys and malformed JSON. The size limit is controlled by a ratified performance/profile contract, not guessed by the client.
2. Validate loopback bind address, request Host and Origin with a strict allowlist; reject inappropriate browser-origin requests prior to MCP dispatch. CORS response headers alone are not authentication.
3. Authenticate the actual configured local client credential independent of user-agent, display name, JSON-RPC `id`, clientInfo, Origin, or session identifier.
4. A request must resolve to exactly one configured protocol adapter; ambiguous negotiation fails closed. Adapter selection must not switch security principal mid-request.
5. A version adapter only translates version-specific wire protocol; **all** policy, registry, approval and persistence operations pass through one gateway authority.
6. Tool names exposed to each principal are a deterministic projection of current discovery grants and trusted/enabled versions; a cached tool list conveys no execution right.
7. Re-check current grants, package digest, effect class, capability bounds, trusted approval, rate/concurrency budgets, and storage health during invocation admission.
8. Responses must not leak credentials, raw policy source, non-visible tool names, system paths or worker stack traces.

## Method/feature acceptance register
The initial release must enumerate every normative core method and notification and give one of: SUPPORTED, PROTOCOL-DEFINED OPTIONAL NOT ADVERTISED, or INCOMPATIBLE AND REJECTED. Candidate focus: `server/discover`, `tools/list`, `tools/call`, supported transport cancellation, and mandatory lifecycle/metadata behavior. Sampling, elicitation, roots, prompts, resources, subscriptions, tasks, experimental extensions and UI apps are **not automatically enabled** by the mere choice of protocol revision. Each requires explicit capability classification, wire fixtures and owner-approved scope if beyond the original tool-platform product. A required normative core response cannot be omitted simply because a feature is optional elsewhere.

## Proposed qualification scenarios (documentation only)
- **T-004a:** known-good modern `server/discover` request and valid modern tools/list; verify full expected schema, version and absence of legacy session establishment.
- **T-004b:** modern request wrongly containing initialize/session assumptions; verify normative reject/no fallback, no worker launch.
- **T-004c:** unsupported protocol version, absent/mismatched method routing headers or body method mismatch; verify version-specific wire failure, no worker launch, audit-safe outcome.
- **T-004d:** negative fixture sends legacy initialize/initialized handshake; verify no authenticated tool access and the specified version error.
- **T-004e:** confirm server does not advertise 2025-11-25 as supported, and creates no legacy transport session.
- **T-005a:** client lists a tool, administrator revokes grant, client submits cached tools/call; deny without worker execution.
- **T-021a:** Host mismatch, hostile Origin, and preflight/browser-rebinding fixtures fail before tool discovery.
- **T-026a:** probe unsupported optional methods/extensions and verify advertised capabilities are truthful.

Each test above needs concrete raw requests, JSON schema assertions, HTTP status/error behavior, host/build/fixture digest, and exact expected responses derived from the selected pinned normative spec. These are **named scenarios, not completed reproducible tests**.

## Remaining decisions before freeze
- DEC-001 adapter selection is resolved: 2026-07-28 only for initial release. The exact normative wire contract and release conformance artifacts are still required before freeze.
- Whether STDIO bridges use modern protocol directly or strictly translate through one gateway-adapter contract; which installed client versions actually interoperate.
- Conformance-suite revision, exact positive/negative transcripts, transport MIME/streaming behavior, cancellation and notification behavior.
- Complete version-specific JSON-RPC error mapping, optional capability inventory, message-size and time budgets.

**Gate:** C-010 remains NOT implementation-ready until all above details are resolved, tests T-004/T-005/T-021/T-026 have executable-quality documented fixtures (without creating code), and owner accepts ADR-001.
