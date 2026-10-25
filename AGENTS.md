# AI Agent Instructions — DragonForge MCP

**Mandatory:** Read [docs/DEVELOPMENT_RULES.md](docs/DEVELOPMENT_RULES.md) and [docs/ROADMAP.md](docs/ROADMAP.md) in full before any change. These are owner-approved project governance requirements.

Current status (2026-10-25): Phase 0 architecture documentation is **FROZEN** under the exact immutable `phase0-architecture-v1` tag and `docs/ARCHITECTURE_FREEZE_V1.md`. **DO NOT WRITE implementation code, executable project tests, prototypes, scaffolding, or GitHub Actions configurations until separate explicit Phase 1 owner implementation authorization.** Documentation freeze/merge alone does not authorize coding.

Before every new phase: audit prior phase evidence, rules, roadmap, frozen specifications, ADRs, and exact repository commit. Record drift evaluation and obtain any necessary approvals.

All GitHub Actions jobs must use the owner's **local self-hosted runner**; never GitHub-hosted runners. Before a runner is set up, provide reproducible PowerShell commands for manual local tests and record user-supplied evidence precisely; do not pretend manual tests are Actions results.

Tool extensibility is a first-class requirement: a new custom tool must be installable through the published SDK/manifest/CLI without core gateway code modifications.

For any handoff, carry the mandatory rules, current phase, exact baseline SHA, outstanding decisions, and testing restrictions into the next agent's context.
