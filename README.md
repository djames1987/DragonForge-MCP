# DragonForge MCP

Local-first, Windows-first MCP tool infrastructure for AI applications. **Status: Phase 0 architecture proposal; no implementation has been qualified.**

## Vision
One security-controlled local MCP gateway through which multiple compatible AI clients discover and invoke reusable tools. Python is the primary orchestration language; custom tools may use Python, native workers, or controlled external processes.

## Approved decisions (2026-10-21)
- On-demand startup.
- Sensitive/destructive actions require explicit approval, with deny-by-default when approval cannot be securely obtained.
- Manual third-party tool installation with explicit trust.
- CLI first; GUI later.
- Adding custom tools must be simple, without editing gateway internals.
- Windows 10 first, portable architecture; local-only operation.

## Architecture proposal
- Local Streamable HTTP MCP endpoint bound exclusively to 127.0.0.1; optional STDIO-to-gateway bridge.
- Verified client identity and deny-by-default discovery and execution permissions.
- Versioned tool registry, SDK/decorator API, manifest and worker adapters.
- Trusted in-process tools for narrowly vetted low-risk operations; isolated workers for risky workloads.
- SQLite for configuration, permissions and audit metadata with transactional migrations.
- Explicit tool lifecycle: discover/install -> inspect -> trust -> enable -> grant to client -> invoke -> revoke/disable.
- Tool provenance and pinned code/manifest digest; changes require renewed trust.

## Custom tool developer experience
Target: create a Python function, attach metadata with an SDK decorator, run validation, install through the CLI, inspect permissions, trust and enable it, then grant access to a specific client. No gateway source-code modification. A native-tool worker adapter will use a versioned JSON-based request/result contract. See [tool authoring contract](docs/contracts/tool-authoring.md).

## Security boundaries
Localhost is not an authentication boundary. Validate Origin/Host for HTTP, authenticate each client, verify permission at discovery and execution, avoid secrets in logs, reject path escapes, require approval for sensitive mutations, and never treat subprocess execution alone as sandboxing. Approval cannot be self-asserted by an AI agent. Third-party code is untrusted and requires real OS-backed isolation for sensitive use cases.

## Documentation-first development
Architecture decisions and contracts precede production implementation. No phase advances without tests and independently reproducible qualification evidence. See [development rules](docs/DEVELOPMENT_RULES.md).

## Phase roadmap
0. Requirements, ADRs, threat model, custom-tool lifecycle and protocol contracts; review and freeze.
1. Secure gateway, identity, permissions, SQLite migrations, trivial test tool, interoperability qualification.
2. Tool registry, CLI authoring/install workflow, worker protocol, lifecycle and isolation.
3. Scoped files, development and diagnostics tools; multi-client negative security tests.
4. Performance, recovery, packaging and Windows acceptance tests.

## Non-goals for initial release
Internet-facing hosting, automatic third-party plugin downloads/execution, GUI-first administration, unrestricted shell access, and integration with any other DragonForge project by default.
