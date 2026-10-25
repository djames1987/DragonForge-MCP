# Custom Tool Authoring Contract — Phase 0 C-002 integration profile

## Developer experience

A small, versioned Python decorator API is the default authoring path. The **illustrative** API below is not implemented yet:

```python
from dragonforge_tools import tool

@tool(name="example.hello", version="1.0.0", permissions=[])
def hello(name: str) -> str:
    """Greet a user."""
    return f"Hello, {name}!"
```

Authoritative planned Windows CLI authoring and enrollment workflow (C-007A, illustrative values; not executable before implementation):
```powershell
dfmcp tool scaffold --id example.hello --output C:\\DFMCP-Examples\\hello --language python
dfmcp tool validate --path C:\\DFMCP-Examples\\hello
dfmcp tool install --path C:\\DFMCP-Examples\\hello
dfmcp tool show example.hello --version 1.0.0 --digest <64-lowercase-hex-SHA256>
dfmcp tool trust example.hello --version 1.0.0 --digest <64-lowercase-hex-SHA256> --if-revision <current-revision>
dfmcp tool enable example.hello --version 1.0.0 --digest <64-lowercase-hex-SHA256> --if-revision <current-revision>
dfmcp grant add --client <enrolled-client-uuid> --tool example.hello --version 1.0.0 --digest <64-lowercase-hex-SHA256> --kind discover
dfmcp grant add --client <enrolled-client-uuid> --tool example.hello --version 1.0.0 --digest <64-lowercase-hex-SHA256> --kind invoke
```
Tools must use namespaced lowercase dotted IDs. Validation and installation do not imply trust, enablement or grants. The `tool trust` owner confirmation cannot be bypassed by `--json`; any rendered example is non-executable until the documented CLI implementation exists. There is no v1 `tool init`, `tool inspect`, `tool test` or `client grant` command; older draft examples are superseded.

## Authoring and execution contract
- Python tool functions use standard Python type hints; supported schemas derive deterministically from type annotations with explicit validation for unsupported forms.
- Docs and argument/result JSON Schemas are published through MCP tool discovery.
- Tool IDs are stable and namespaced; versions are immutable and content-digest pinned.
- Manifest describes permissions, implementation kind, input/output schemas, effect classification, timeouts, maximum input/output sizes, cancellation, and concurrency.
- Each request carries an internal authenticated execution context supplied by the gateway, not by the client arguments.
- Tool discovery is filtered by verified client permissions.
- Invocations are checked again against current authorization, capability scope, approval policy, and tool digest.
- Tool results and errors use a stable envelope; sensitive details remain redacted from client-facing errors.
- External native workers implement a versioned JSON request/result protocol using bounded IPC; raw shell commands are not interpreted from tool parameters.
- Tools may not directly modify client grants, trusted plugin state, or gateway audit records.

## Trust and safety
Installing a tool stages its code for inspection; it does not execute the tool's startup hooks. Explicit trust is required before activation, and authorization is separately granted to clients. If approval cannot be obtained from a trusted local human interface, high-risk invocations are denied. DEC-006 owner-selected: SHA-256 content pinning and explicit local owner trust are mandatory; third-party publisher signatures are not mandatory for the first release. Hashes detect changes to pinned bytes but do not establish source identity or software safety. Any displayed publisher-signature claim is informational and confers no authority. C-002 must specify deterministic package digest canonicalization, handling of symlinks/junctions, and launch-time digest checks before freeze. Trust, enablement, grants, approvals and OS isolation remain independent controls.

## Phase 0 qualification criteria
1. A trivial user-authored tool can be added with no gateway code edits.
2. Unauthorized clients cannot discover or invoke it.
3. Altering the trusted tool package invalidates trust.
4. An invalid schema or manifest cannot activate.
5. A failed worker cannot corrupt the gateway or grant permissions.
6. Approval cannot be spoofed through MCP tool arguments.
7. Version pinning and rollback behavior are reproducible.
8. Windows platform confinement is explicitly tested before untrusted tools are supported.

## Authoritative precedence — 2026-10-25
This developer-facing guide explains C-002 SDK integration. In conflicts, the byte-accurate package hashing/trust rules in `package-integrity-and-lifecycle.md`, the exact command grammar in `administrative-cli.md`, the 54 schemas in `cli-command-schema-registry-v1.json`, and the worker wire in `worker-ipc-wire.md` are authoritative. The decorator shown above is an illustrative future SDK surface, not a currently importable Python library, and permissions must be represented by the versioned manifest declared capability model.
