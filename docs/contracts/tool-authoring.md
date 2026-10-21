# Custom Tool Authoring Contract — Proposal v0.1

## Developer experience

A small, versioned Python decorator API is the default authoring path. The **illustrative** API below is not implemented yet:

```python
from dragonforge_tools import tool

@tool(name="hello", version="1.0.0", permissions=[])
def hello(name: str) -> str:
    """Greet a user."""
    return f"Hello, {name}!"
```

Intended workflow (CLI syntax is proposed, not yet available):
```powershell
dfmcp tool init hello
dfmcp tool validate ./hello
dfmcp tool install ./hello
dfmcp tool inspect hello
dfmcp tool trust hello --version 1.0.0
dfmcp tool enable hello
dfmcp client grant <client-id> hello
dfmcp tool test hello --args '{ "name": "World" }'
```

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
Installing a tool stages its code for inspection; it does not execute the tool's startup hooks. Explicit trust is required before activation, and authorization is separately granted to clients. If approval cannot be obtained from a trusted local human interface, high-risk invocations are denied. Signed provenance may be added later, but user approval is not a substitute for OS isolation.

## Phase 0 qualification criteria
1. A trivial user-authored tool can be added with no gateway code edits.
2. Unauthorized clients cannot discover or invoke it.
3. Altering the trusted tool package invalidates trust.
4. An invalid schema or manifest cannot activate.
5. A failed worker cannot corrupt the gateway or grant permissions.
6. Approval cannot be spoofed through MCP tool arguments.
7. Version pinning and rollback behavior are reproducible.
8. Windows platform confinement is explicitly tested before untrusted tools are supported.
