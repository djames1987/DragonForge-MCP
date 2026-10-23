# C-010B — Source-verified 2026-07-28 transport conformance corrections
Status: Phase 0 contract addendum, source review on 2026-10-22. Owner-selected DEC-001 preserved: **only 2026-07-28**, no automatic legacy fallback. Normative authority remains the official versioned specification; this addendum supersedes incompatible illustrative statements in C-010A.

## Normative source ledger
- https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning — per-request version, modern/legacy boundaries, `server/discover`, `UnsupportedProtocolVersionError` `-32022`, response data `supported` and `requested`.
- https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http — POST transport, Origin, Accept, version/method/name/param header mirroring, errors and SSE.
- https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio — newline-delimited JSON-RPC, no header transport, cancellation and shutdown.
- https://modelcontextprotocol.io/specification/2026-07-28/basic/transports — client-sent requests/notifications only, server responses/notifications only, body metadata authoritative.

## Corrections to existing C-010A
1. `server/discover` is a **required server RPC** but it is **optional for a modern-only client to invoke before other methods**. Every request separately declares a revision in `params._meta`. The server supports exactly [2026-07-28].
2. Modern clients MAY send `tools/call` directly. An `initialize` request is not accepted as a handshake and cannot create authority. Legacy clients are excluded by DEC-001.
3. Each HTTP POST carries `MCP-Protocol-Version` matching the body, `Mcp-Method` matching the JSON-RPC method, and `Mcp-Name` for name/URI-bearing methods. Header names compare case-insensitively, method/name values case-sensitively. Missing or mismatching required routing header MUST yield HTTP 400 with JSON-RPC `-32020 HeaderMismatch`.
4. A name or annotated primitive tool parameter unsafe to express as plain ASCII (including leading/trailing whitespace or text resembling the sentinel) must use `=?base64?{Base64EncodedUtf8}?=`. Validate encoded header **after decoding**, before dispatch; malformed/duplicate conflicting headers fail closed.
5. Tool parameter `x-mcp-header` annotations are part of the normative 2026-07-28 HTTP transport. For supported tools, inspect `inputSchema` paths whose statically reachable scalar property has valid `x-mcp-header`, require `Mcp-Param-{name}` only when argument value is present and nonnull, canonicalize integer/boolean to HTTP-compatible values, then compare headers against parsed arguments. If the gateway cannot support the annotation and header behavior, **reject that tool definition at validation rather than serve it as conformant**. It must not silently ignore required headers.
6. Unsupported protocol revision: `-32022` with `data.supported=["2026-07-28"]` and `data.requested` matching the rejected revision; HTTP 400. Unsupported RPC method: HTTP 404 JSON-RPC `-32601`. Host/Origin policy denial is distinct from protocol version negotiation.
7. An HTTP request's `Accept` must list `application/json` and `text/event-stream`; a server can reply with one JSON object or request-scoped SSE (client must support both). DragonForge v1 chooses JSON-only responses where allowed. A request-scoped streaming response, if introduced, must propagate cancellation upon closure. Do not advertise or accept unsupported `subscriptions/listen` as implemented.
8. Modern core defines **no client-to-server Streamable HTTP notifications**; generic HTTP transport mechanics state that an accepted notification POST returns 202 without a body. Do not claim notification support just to satisfy this generic transport rule. On STDIO the `notifications/cancelled` flow is defined.
9. STDIO uses one UTF-8 JSON-RPC message per line; the line must not embed newline characters. Stdout contains protocol only and stderr diagnostics; client sends no JSON-RPC response messages. Client cancellation uses `notifications/cancelled`; no later response for cancelled request.
10. A gateway credential is **DragonForge-specific authorization**, never the MCP `clientInfo` display metadata. A client may report arbitrary `clientInfo` without obtaining elevated grants.

## Positive header-to-body examples
Example A, valid modern HTTP `tools/list` (Authorization credential intentionally redacted):
```http
POST /mcp HTTP/1.1
Host: 127.0.0.1:8765
Authorization: Bearer <REDACTED>
Content-Type: application/json
Accept: application/json, text/event-stream
MCP-Protocol-Version: 2026-07-28
Mcp-Method: tools/list

{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}}}
```
Example B, valid `tools/call` with schema annotation `x-mcp-header:"Region"` on a string `region` and `region="us-west1"`: add `Mcp-Method: tools/call`, `Mcp-Name: example.echo`, `Mcp-Param-Region: us-west1`; ensure body argument is exactly `"region":"us-west1"`. If schema has no annotations, `Mcp-Param-*` is not required.
Example C, non-ASCII tool name `naïve`: `Mcp-Name: =?base64?bmHDr3Zl?=` is compared to decoded body `params.name="naïve"`. (The encoded string is illustrative and must be verified as a conformance vector.)

## Negative transport matrix (future planned T-004/T-021/T-026)
| Fixture | Input | Expected |
|---|---|---|
| MCP-N01 | version 2025-11-25 in body+HTTP header | -32022, supported+requested metadata, no fallback |
| MCP-N02 | omitted HTTP version header | 400; no legacy inference |
| MCP-N03 | incorrect Mcp-Method | 400/-32020; no dispatch |
| MCP-N04 | omitted or mismatched Mcp-Name for tools/call | 400/-32020 |
| MCP-N05 | invalid base64 sentinel encoded header | 400/-32020 |
| MCP-N06 | annotated arg present but Mcp-Param-Region missing/wrong | 400/-32020 |
| MCP-N07 | invalid unrecognized x-mcp-header annotation in tool schema | reject tool definition, no advertisement |
| MCP-N08 | hostile Origin header | 403 before authorization |
| MCP-N09 | GET /mcp legacy stream attempt | not a modern GET stream; deny |
| MCP-N10 | server-initiated JSON-RPC request | never emitted |
| MCP-N11 | STDIO cancellation during running tool | terminate or mark uncertain, no post-cancel reply |
| MCP-N12 | unsupported extensions, subscriptions/listen | not advertised, no implementation handler |
| MCP-N13 | malformed JSON, client response envelope, batch message | reject before dispatch |
| MCP-N14 | arbitrary clientInfo forged identity | cannot change gateway credential-bound principal |

## Freeze evidence remaining
Official exact schema commit, formal type definitions for server/discover and tools result objects, request-scoped SSE choice, all required header value tests, conformance-suite fixture checks, and the contract's actual source-code-less PowerShell test recipe must be pinned. A documentation-only web verification is **not** a conformance test. No test is executed here.
