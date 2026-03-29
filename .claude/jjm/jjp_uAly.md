## Context

Establish adversarial AI escape testing infrastructure for Recipe Bottle. An Ifrit is an AI agent (Claude Code) running inside a bottle container with perfect information about its prison (read-only project mount), programmatic attack tooling (scapy), and persistence (read-write escape test directory). The Ifrit writes durable Python escape test modules — it is a test author, not a live pentester.

This heat creates the Ifrit's context: slim vessels, volume mount plumbing, the ifrit bottle, the tadmor nameplate, and the escape test framework. Ultimate refinement of the Ifrit's test corpus is future work.

## Themes

**Vessel simplification**: Current sentry and bottle vessels carry ~30 diagnostic packages each on ubuntu:24.04. Strip each to essential function on Debian bookworm-slim. Sentry needs 4 packages (bash, iptables, dnsmasq, iproute2) with iptables-legacy pinned. Bottle needs socat. The ifrit bottle is purpose-built: node, python3-scapy, strace.

**Base image decision**: Trade study (RBSIP-ifrit_pentester.adoc) evaluated Debian slim, Ubuntu LTS, Alpine, UBI minimal, Wolfi/Chainguard, and distroless. Two independent research streams converged on Debian bookworm-slim as the most conservative, auditable choice for a firewall trust boundary. RHEL is the reference OS for iptables documentation corpus.

**Tadmor nameplate**: Replaces nsproto as the primary sentry/bottle service definition. Named for the ancient city Solomon built in the wilderness — the same Solomon who imprisoned djinn in brass vessels. Anthropic API connectivity via DNS/CIDR allowlist. Future releases may open to additional LLM providers.

**Docker Compose lifecycle**: Replaces imperative docker CLI orchestration (rbob building --volume arrays, exec'ing sentry scripts, sequencing container starts) with declarative Docker Compose. Motivated by Windows volume mount path incompatibility (drive letter colons collide with -v syntax), but the real win is declarative security gating: health check chains ensure censer routing is configured before bottle starts, preventing firewall bypass. One static base compose file (rbob_compose.yml) + per-nameplate fragments for volume mounts. Nameplate .env files consumed directly by compose — no generation step.

**Volume mount infrastructure**: The ifrit bottle gets read-only project mount (full visibility into sentry scripts, iptables config, specs) and read-write mount for escape test persistence (rbtid/). Volume mount specs live in per-nameplate compose fragments as relative paths (portable across Windows/Linux/macOS), NOT in nameplate .env files.

**Escape test curation**: Python modules written by the Ifrit are committed to version control, reviewed, and maintained as first-class test infrastructure — not disposable probe scripts. The rbtid/ directory is the permanent home for these curated escape tests.

## Key Decisions

- Base image: Debian 12 slim (bookworm-slim) — see RBSIP trade study
- iptables backend: legacy, pinned via update-alternatives in Dockerfile
- "Validate" not "verify" throughout — legally wigglier, no certainty promised
- rbsi_ prefix for linked terms in RBS0
- rbti prefix for ifrit test infrastructure (rbtid/ directory, rbtiXXX modules)
- rbev-bottle-ifrit vessel sigil
- Anthropic-only for first release
- Nameplate file naming: {moniker}.rbrn.env (moniker leads, regime type as suffix)
- Compose file naming: rbob_compose.yml (base), {moniker}.compose.yml (fragment)
- Sentry/censer entrypoint scripts baked into sentry image (zero mounts for security containers)
- Container env vars forwarded via environment: with bare names from exported parent shell
- Compose --env-file for YAML interpolation, environment: for container injection — two distinct mechanisms
- Podman compose deferred (architecture accommodates via podman compose delegating to Docker Compose v2)

## References

- RBSIP-ifrit_pentester.adoc — system concept and trade study (committed)
- Tools/rbk/rbj_sentry.sh — sentry startup script (becomes baked-in entrypoint)
- Tools/rbk/rbob_bottle.sh — bottle lifecycle (refactored to invoke compose)
- .rbk/rbrr.env — repo regime (consumed directly by compose)
- rbev-vessels/rbev-sentry-debian-slim/ — sentry vessel (entrypoint scripts baked in)
- ₣AU (rbk-mvp-3-release-finalize) — parent MVP, ₢AUAAk superseded by this heat