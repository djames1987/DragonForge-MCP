# Development Rules — Proposal v0.1

Status: proposed for Phase 0 review; do not represent as approved architecture freeze.

1. **Documentation before implementation.** Maintain requirements, architecture contracts, security assumptions, ADRs, and traceable acceptance criteria.
2. **No architectural drift.** Changes to frozen boundaries require a new ADR, impact analysis, and qualification update.
3. **Local-first.** Bind services to loopback; deny remote interfaces; avoid implicit cloud dependencies.
4. **Zero implicit trust.** Verify client identity per request, apply deny-by-default authorization both to visibility and execution, and separate installer approval from client grants.
5. **Custom-tool ergonomics are a release gate.** Add tools through the published SDK/manifest and CLI only, without modifying gateway routing or core registry source.
6. **Version and pin tool contracts.** Validate schemas, exact tool implementation fingerprint, registration metadata, permissions, and compatibility; changes require renewed review/trust.
7. **Fail closed.** Invalid config, schema, approval, identity, worker isolation, or permissions must not permit execution.
8. **No pretend sandboxing.** A Python subprocess by itself is not containment; restricted OS privileges or equivalent are required for untrusted execution.
9. **Test against attacks.** Include wrong-client grants, revoked credentials, malicious paths, worker timeouts/crashes, tool mutation/rug pulls, spoofed approval, localhost Origin abuse, and concurrency.
10. **Reproducible evidence.** Track exact commits and commands for lint, type-check, unit, integration, security, compatibility, and performance runs.
11. **No secrets in source control or logs.** Redact request and result data in audit metadata and use least privilege for local storage.
12. **Phase gating.** Do not claim a phase complete until exit criteria pass; correct CI failures before qualification.
13. **Portability.** Encapsulate Windows-specific security/process APIs behind tested interfaces.
14. **Preserve boundaries.** Tool packages never gain direct administrative access to the gateway database or registry.
