## Context

Establish adversarial AI escape testing infrastructure for Recipe Bottle. An Ifrit is an AI agent (Claude Code) running inside a bottle container with perfect information about its prison (read-only project mount), programmatic attack tooling (scapy), and persistence (read-write escape test directory). The Ifrit writes durable Python escape test modules — it is a test author, not a live pentester.

This heat creates the Ifrit's context: slim vessels, volume mount plumbing, the ifrit bottle, the tadmor nameplate, and the escape test framework. Ultimate refinement of the Ifrit's test corpus is future work.

## Themes

**Vessel simplification**: Current sentry and bottle vessels carry ~30 diagnostic packages each on ubuntu:24.04. Strip each to essential function on Debian bookworm-slim. Sentry needs 4 packages (bash, iptables, dnsmasq, iproute2) with iptables-legacy pinned. Bottle needs socat. The ifrit bottle is purpose-built: node, python3-scapy, strace.

**Base image decision**: Trade study (RBSIP-ifrit_pentester.adoc) evaluated Debian slim, Ubuntu LTS, Alpine, UBI minimal, Wolfi/Chainguard, and distroless. Two independent research streams converged on Debian bookworm-slim as the most conservative, auditable choice for a firewall trust boundary. RHEL is the reference OS for iptables documentation corpus.

**Tadmor nameplate**: Replaces nsproto as the primary sentry/bottle service definition. Named for the ancient city Solomon built in the wilderness — the same Solomon who imprisoned djinn in brass vessels. Anthropic API connectivity via DNS/CIDR allowlist. Future releases may open to additional LLM providers.

**Volume mount infrastructure**: New work in sentry/bottle start flow. The ifrit bottle gets read-only project mount (full visibility into sentry scripts, iptables config, specs) and read-write mount for escape test persistence (rbtid/).

**Escape test curation**: Python modules written by the Ifrit are committed to version control, reviewed, and maintained as first-class test infrastructure — not disposable probe scripts. The rbtid/ directory is the permanent home for these curated escape tests.

## Key Decisions

- Base image: Debian 12 slim (bookworm-slim) — see RBSIP trade study
- iptables backend: legacy, pinned via update-alternatives in Dockerfile
- "Validate" not "verify" throughout — legally wigglier, no certainty promised
- rbsi_ prefix for linked terms in RBS0
- rbti prefix for ifrit test infrastructure (rbtid/ directory, rbtiXXX modules)
- rbev-bottle-ifrit vessel sigil
- Anthropic-only for first release

## References

- RBSIP-ifrit_pentester.adoc — system concept and trade study (committed)
- Tools/rbk/rbj_sentry.sh — sentry startup script (change surface for slim)
- Tools/rbk/rbob_bottle.sh — bottle lifecycle (volume mount plumbing)
- .rbk/rbrn_nsproto.env — current nameplate (to become rbrn_tadmor.env)
- rbev-vessels/rbev-sentry-ubuntu-large/ — current sentry vessel (to be slimmed)
- rbev-vessels/rbev-bottle-ubuntu-test/ — current bottle vessel (to be slimmed)
- ₣AU (rbk-mvp-3-release-finalize) — parent MVP, ₢AUAAk superseded by this heat