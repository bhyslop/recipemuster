# Recipe Bottle

> [!IMPORTANT]
> **Early-stage project — security review welcome**
>
> Recipe Bottle runs untrusted containers inside a three-container security apparatus: a privileged pentacle establishes the network namespace, a sentry enforces iptables policy on dual networks, and the bottle workload has no direct network access. The supply chain is hardened with SLSA provenance verification, least-privilege service accounts, and no secrets in version control.
>
> This architecture is deliberate, but it has not yet had broad independent review — particularly the runtime containment (iptables rules, privileged namespace setup, network isolation enforcement). If you evaluate or deploy this, you are contributing to its hardening. Security-focused contributors and responsible disclosure are especially valued.

Recipe Bottle helps you build container images with rigorous supply-chain provenance, and run untrusted containers behind enforced network isolation.

On the build side, Recipe Bottle orchestrates Google Cloud Build to produce images with SLSA attestation, software bills of material, reproducible multi-architecture builds, and digest-pinned toolchains — so every image has a verifiable origin story.

On the runtime side, Recipe Bottle interposes a sentry container between untrusted workloads and system resources, enforcing network policy via `iptables` and `dnsmasq` — without requiring modifications to existing container images.

The system uses only `bash`, `git`, `curl`, `openssh`, `jq`, `openssl`, and `docker` natively. No `gcloud` CLI is required on your workstation — cloud operations use REST APIs via `curl` and `jq`.

**Project page**: https://scaleinv.github.io/recipebottle

<p align="center">
  <img src="rbm-abstract-drawio.svg" alt="Recipe Bottle architecture diagram" width="720" />
</p>

Recipe Bottle treats two complexity domains as distinct, orthogonal concerns. The first is **image management**: producing container images from untrusted source with verifiable provenance. Builds run on Google Cloud Build in an egress-locked ("airgap") configuration using digest-pinned toolchains, produce SLSA attestations and SBOMs, and draw from upstream base images mirrored into a project-owned registry — so the build pipeline has a fixed, self-contained supply chain independent of third-party registry availability. The second domain is **crucible orchestration**: running those images — or any unmodified third-party image — behind a security apparatus that enforces network policy without touching the workload. A developer can adopt either domain alone; the two are designed to compose, not to depend on each other.

The distinctive case Recipe Bottle addresses is *running untrusted code*: third-party tooling, experimental packages, binaries with uncertain provenance. Containers excel at packaging known applications, but running unvetted code poses security risks that ordinary container deployment does not solve. The [bottle](#Bottle) container runs unmodified, in a network namespace prepared by a privileged [pentacle](#Pentacle), with all egress flowing through a [sentry](#Sentry) gateway configured via the mature tools `iptables` and `dnsmasq`.

Recipe Bottle is a set of bash scripts designed to be incorporated into arbitrary projects — via `git subtree`, `git subrepo`, `git submodule`, or simply by copying a few directories into place. The workstation floor is deliberately narrow: `bash 3.2`, `git`, `curl`, `openssh`, `jq`, `openssl`, and `docker`. There is no Python runtime, no language-specific package manager, no `gcloud` CLI on the workstation — Google Cloud operations run as REST calls over `curl` and `jq`. A small team can stand up a hardened build pipeline and a sandboxed runtime without specialized DevOps expertise.

## Key Concepts

| Term | Meaning |
|------|---------|
| <a id="Vessel"></a>**Vessel** | A specification for a container image — built from source ([conjure](#Conjure)), mirrored from upstream ([bind](#Bind)), or pushed from local ([graft](#Graft)) |
| <a id="Ark"></a>**Ark** | An immutable container image artifact in the registry, produced from a [vessel](#Vessel) |
| <a id="Hallmark"></a>**Hallmark** | A specific build instance of a [vessel](#Vessel), identified by timestamp |
| <a id="Vouch"></a>**Vouch** | Cryptographic attestation proving an [ark](#Ark) was built by trusted infrastructure |
| <a id="Depot"></a>**Depot** | The facility where container images are built and stored (GCP project + registry + bucket) |
| <a id="Nameplate"></a>**Nameplate** | Per-vessel configuration tying a [sentry](#Sentry) and [bottle](#Bottle) together into a runnable unit |
| <a id="Sentry"></a>**Sentry** | Security container enforcing network policies via `iptables` and `dnsmasq` |
| <a id="Pentacle"></a>**Pentacle** | Privileged container establishing the network namespace shared with the [bottle](#Bottle) |
| <a id="Bottle"></a>**Bottle** | Your workload container, running unmodified in a controlled network environment |
| <a id="Crucible"></a>**Crucible** | The [sentry](#Sentry)/[pentacle](#Pentacle)/[bottle](#Bottle) triad running together as one runnable unit for a [nameplate](#Nameplate) |
| <a id="Charge"></a>**Charge** | Start the [sentry](#Sentry)/[pentacle](#Pentacle)/[bottle](#Bottle) triad for a [nameplate](#Nameplate) |
| <a id="Quench"></a>**Quench** | Stop and clean up a charged [nameplate](#Nameplate)'s containers |
| <a id="Ordain"></a>**Ordain** | Create a [hallmark](#Hallmark) with full attestation — the production build command |
| <a id="Conjure"></a>**Conjure** | [Ordain](#Ordain) mode: Cloud Build creates the image from source |
| <a id="Bind"></a>**Bind** | [Ordain](#Ordain) mode: mirror an upstream image pinned by digest |
| <a id="Graft"></a>**Graft** | [Ordain](#Ordain) mode: push a locally-built image to the registry |
| <a id="Kludge"></a>**Kludge** | Build a [vessel](#Vessel) image locally for fast iteration (no registry push) |
| <a id="Enshrine"></a>**Enshrine** | Mirror upstream base images into your [depot](#Depot)'s registry |
| <a id="Summon"></a>**Summon** | Pull a [hallmark](#Hallmark) image from the [depot](#Depot) to your local machine |
| <a id="Plumb"></a>**Plumb** | Inspect an artifact's provenance (SBOM, build info, [vouch](#Vouch) chain) |
| <a id="Tally"></a>**Tally** | Inventory [hallmarks](#Hallmark) in the registry by health status |
| <a id="Levy"></a>**Levy** | Provision a new [depot](#Depot)'s GCP infrastructure |
| <a id="Payor"></a>**Payor** | Owns the GCP project and funds it; authenticates via OAuth |
| <a id="Governor"></a>**Governor** | Administers a [depot](#Depot): creates service accounts, manages access |
| <a id="Director"></a>**Director** | Builds and publishes [vessel](#Vessel) images into a [depot](#Depot) |
| <a id="Retriever"></a>**Retriever** | Pulls and runs [vessel](#Vessel) images from a [depot](#Depot) |
| <a id="Charter"></a>**Charter** | Create a [retriever](#Retriever) service account ([governor](#Governor) operation) |
| <a id="Knight"></a>**Knight** | Create a [director](#Director) service account ([governor](#Governor) operation) |

## How It Works

Recipe Bottle addresses two orthogonal complexity domains: building container images with verifiable provenance, and running untrusted images with enforced network isolation. The two tracks compose but neither requires the other.

### Image Management

Recipe Bottle builds container images on Google Cloud Build (GCB) and stores them in Google Artifact Registry (GAR):

- Isolated build environments using Google-curated Cloud Build builder images
- Multi-architecture support via `docker buildx` with binfmt emulation
- SLSA provenance attestation and verification
- Software Bills of Material (SBOM) for every build
- Full build transcripts captured as auxiliary metadata artifacts
- Upstream base images [enshrined](#Enshrine) into the [depot's](#Depot) registry, so builds do not depend on third-party registry availability at build time
- `gcloud` never runs on the workstation — REST calls via `curl` and `jq` drive all remote operations, and the Google-supplied `gcloud` binary is confined to Cloud Build step containers on the server side

### Crucible Orchestration

For running containers with network services, Recipe Bottle orchestrates three containers working together:

- **Sentry** — enforces network security policies via `iptables` and `dnsmasq`
- **Pentacle** — establishes a privileged network namespace and shares it with the bottle
- **Bottle** — your workload container, running unmodified in a controlled network environment

This ensures security policies are enforced from the first packet, and the bottle container experiences only a functional path to its sentry gateway.

The [sentry](#Sentry) applies two layers of egress policy: `dnsmasq` answers DNS queries only for explicitly allowed names, and `iptables` permits outbound IP traffic only to allowed CIDR ranges. The two layers combine into an enforced allowlist that a compromised or misbehaving [bottle](#Bottle) cannot bypass — neither by DNS-based exfiltration nor by direct IP connection to an unapproved destination. Each crucible's allowlist is declared in a [nameplate](#Nameplate) regime file alongside the sentry and bottle [vessel](#Vessel) selections, making network policy a reviewable artifact of the configuration rather than an implicit runtime behavior.

### Project Direction

**Build pipeline.** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.

**Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the [sentry's](#Sentry) CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.

**Networking.** [Bottle](#Bottle)-to-bottle communication is feasible under the current sentry model but not implemented; waiting for a concrete use case.

## Prerequisites

- macOS or Linux workstation
- `bash` (3.2+), `git`, `curl`, `openssh`, `jq`, `openssl`
- `docker` (container runtime)
- A Google Cloud account with billing enabled (credit card required for verification; free tier is sufficient to start)

## Using the CLI

All Recipe Bottle operations are invoked through lightweight shell scripts in the `tt/` directory, run from the project root. Tab completion narrows by prefix. The project's `CLAUDE.md` provides a complete command reference; the interactive onboarding walkthroughs are the authoritative procedure source.

## Setup

Recipe Bottle uses a role-based security model with four roles, each building on the previous:

| Role | Authenticates via | Purpose |
|------|-------------------|---------|
| [**Payor**](#Payor) | OAuth (browser flow) | Creates/funds GCP infrastructure, manages [governor](#Governor) lifecycle |
| [**Governor**](#Governor) | Service account credential | Administers [director](#Director) and [retriever](#Retriever) credentials within a [depot](#Depot) |
| [**Director**](#Director) | Service account credential | Submits builds, manages images, verifies provenance |
| [**Retriever**](#Retriever) | Service account credential | Pulls images for local use |

The [payor](#Payor) stands apart — it requires manual Google Cloud Console work and OAuth authentication. All downstream roles authenticate via credential files, enabling full automation.

### Onboarding

The interactive onboarding guide detects which roles you have credentials for and routes you to the appropriate walkthrough. Each role has its own guided procedure that probes your progress and shows the next step. A reference view provides a full health dashboard across all roles.

### Establishment and Provisioning

The [payor](#Payor) begins by creating a GCP project and configuring an OAuth consent screen through the Google Cloud Console. After downloading the OAuth client credentials, the payor installs them via a browser authorization flow — the resulting refresh token is stored locally with restrictive permissions and does not need to be repeated.

With payor credentials in place, the payor [levies](#Levy) a [depot](#Depot), provisioning it with build infrastructure, artifact registry, and secrets storage. The payor then mantles a [governor](#Governor) service account to administer the [depot](#Depot).

### Credential Distribution

The [governor](#Governor) creates downstream credentials: [knighting](#Knight) a [director](#Director) for build operations and [chartering](#Charter) a [retriever](#Retriever) for image pull access. Each credential is scoped to a single role within a single [depot](#Depot).

### Build and Retrieve

The [director](#Director) [ordains](#Ordain) [hallmarks](#Hallmark) for each [vessel](#Vessel) — [conjuring](#Conjure) from source, [binding](#Bind) from upstream, or [grafting](#Graft) from local builds. After builds complete, the director [tallies](#Tally) [hallmarks](#Hallmark) by health status and [vouches](#Vouch) their provenance. [Hallmark](#Hallmark) values from the tally are recorded into [nameplate](#Nameplate) regime files, completing the chain from build to runtime.

The [retriever](#Retriever) [summons](#Summon) vouched images locally for use.

## Day-to-Day Operations

To run a workload, [charge](#Charge) the [crucible](#Crucible) for a [nameplate](#Nameplate). This starts the [sentry](#Sentry), [pentacle](#Pentacle), and [bottle](#Bottle) containers together — the [bottle](#Bottle) is ready for interactive use immediately.

For diagnostics, shell into the [bottle](#Bottle) or the [sentry](#Sentry), or observe network traffic across the [crucible's](#Crucible) containers. When finished, [quench](#Quench) the [crucible](#Crucible) to stop and clean up all three containers.

To inspect an image's supply chain, [plumb](#Plumb) its provenance — the full view shows the SBOM, build info, and Dockerfile; the compact view summarizes the attestation chain.

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

| Credential | Location | Created during |
|------------|----------|----------------|
| [Payor](#Payor) OAuth | `~/.rbw/rbro.env` | [Payor](#Payor) installation |
| [Governor](#Governor) | `RBRR_SECRETS_DIR/governor/rbra.env` | [Governor](#Governor) mantling |
| [Director](#Director) | `RBRR_SECRETS_DIR/director/rbra.env` | [Director](#Director) [knighting](#Knight) |
| [Retriever](#Retriever) | `RBRR_SECRETS_DIR/retriever/rbra.env` | [Retriever](#Retriever) [chartering](#Charter) |

Each credential file is scoped to one role within one depot and cannot operate outside its designation.

## Configuration

Recipe Bottle uses a Config Regime system — structured configuration with typed validation. Each regime has a render command (display current values) and a validate command (check correctness).

### User-Configured Regimes

| Regime | File | Purpose |
|--------|------|---------|
| RBRP | `.rbk/rbrp.env` | [Payor](#Payor) GCP project identity |
| RBRR | `.rbk/rbrr.env` | [Depot](#Depot) project, region, build config |
| RBRN | `.rbk/*/rbrn.env` | Per-[vessel](#Vessel): runtime, [hallmarks](#Hallmark) |
| RBRV | vessel dirs | Container image build definitions |
| RBRS | station file | Developer machine paths (not in git) |

### Managed Regimes (generated by operations)

| Regime | Purpose | Generated during |
|--------|---------|-----------------|
| RBRO | OAuth refresh token | [Payor](#Payor) installation |
| RBRA | Service account credentials | Mantling, [knighting](#Knight), [chartering](#Charter) |

### BUK Base Regimes

| Regime | File | Purpose |
|--------|------|---------|
| BURC | `.buk/burc.env` | Project structure (tabtarget dir, tools dir) |
| BURS | `../station-files/burs.env` | Developer machine (log dir). Not in git. |

## Vessels and Nameplates

**Vessels** are build definitions — each is a directory under `rbev-vessels/`:

```
rbev-vessels/
├── rbev-sentry-debian-slim/    # Sentry vessel (conjure — built from source)
│   ├── Dockerfile
│   └── rbrv.env
├── rbev-bottle-ifrit/          # Ifrit bottle vessel (conjure — built from source)
│   ├── Dockerfile
│   └── rbrv.env
└── rbev-bottle-plantuml/       # PlantUML server (bind — upstream image pinned by digest)
    └── rbrv.env
```

Conjure vessels have a Dockerfile and are built by Cloud Build. Bind vessels (like `rbev-bottle-plantuml`) pin an external image by digest in `rbrv.env` — no Dockerfile, no build step. Graft vessels push a locally-built image to GAR via docker push — no Cloud Build for the image, but about and [vouch](#Vouch) still run in Cloud Build. The same [ordain](#Ordain) operation handles all three: it detects the [vessel](#Vessel) mode and triggers a Cloud Build ([conjure](#Conjure)), mirrors from upstream ([bind](#Bind)), or pushes a local image ([graft](#Graft)). Trust hierarchy: [conjure](#Conjure) has full SLSA provenance, [bind](#Bind) has digest-pin verification, [graft](#Graft) has no provenance chain (GRAFTED verdict).

**[Nameplates](#Nameplate)** tie [vessels](#Vessel) together into a runnable [crucible](#Crucible). The [nameplate](#Nameplate) moniker (e.g. `tadmor`) identifies the unit across all operations:

```
.rbk/tadmor/rbrn.env          # Maps tadmor → rbev-sentry-debian-slim + rbev-bottle-ifrit
```

[Charging](#Charge) `tadmor` starts the [crucible](#Crucible) defined by the `tadmor` [nameplate](#Nameplate), which selects its [sentry](#Sentry) and [bottle](#Bottle) [vessel](#Vessel) images.

## Testing

Run test fixtures sequentially — they share regime state and container namespaces. Never run fixtures in parallel.

Two qualification gates exercise the full system: a fast qualify checks tabtarget structure, colophon integrity, and [nameplate](#Nameplate) health; a release qualify adds shellcheck analysis and the complete test suite.

## Recovery

- **Lost OAuth credentials**: Download a fresh JSON key from Google Cloud Console and re-run the [payor](#Payor) installation
- **Expired tokens**: Run the [payor](#Payor) refresh operation
- **Compromised [governor](#Governor)**: Re-mantle the [governor](#Governor) (replaces the service account, invalidates the old credential)
- **Compromised [director](#Director)/[retriever](#Retriever)**: Forfeit the compromised account, then re-[knight](#Knight) or re-[charter](#Charter)
- **Lost [nameplate](#Nameplate) values**: Re-[tally](#Tally) [hallmarks](#Hallmark) to retrieve values from the registry
- **Build timeout or failure**: [Tally](#Tally) to check build status, review logs in the GCP Console for the [depot](#Depot) project

## Architecture

```
Project Root/
├── .buk/                    # BUK launcher directory + BURC config
├── .rbk/                    # Recipe Bottle config regimes
├── tt/                      # TabTargets — all CLI commands live here
├── Tools/
│   ├── buk/                 # Bash Utility Kit (portable CLI infrastructure)
│   └── rbk/                 # Recipe Bottle Kit (domain logic)
└── rbev-vessels/            # Vessel definitions (Dockerfile + rbrv.env per vessel)
```

## Claude Code

If you use [Claude Code](https://claude.com/claude-code), the project includes a `CLAUDE.md` with a full command reference table, glossary, and conventions for AI-assisted development.

## License

Copyright 2026 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0.
