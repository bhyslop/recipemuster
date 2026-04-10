# Recipe Bottle

Recipe Bottle provides two independent container image capabilities:

- **[Foundry](#Foundry)**: orchestrate Google Cloud Build to produce and retain images in a private cloud registry — egress-locked builds with full supply-chain provenance
- **[Crucible](#Crucible)**: run untrusted containers behind enforced network isolation — DNS filtering and IP filtering — without modifying the workload image

> [!IMPORTANT]
> **Early-stage project — security review welcome in both domains**
>
> The [Foundry's](#Foundry) egress-locked Cloud Build configuration — including the SLSA attestation chain, build isolation, and digest-pinned toolchains — has not yet had broad independent review.
>
> The [Crucible](#Crucible) runtime containment — a multi-container apparatus where the workload runs unprivileged in a network namespace it does not control — has also not had broad review, particularly the iptables rules, privileged namespace setup, and network isolation enforcement.
>
> If you evaluate or deploy this, you are contributing to its hardening. Security-focused contributors and responsible disclosure are especially valued.

Recipe Bottle is a set of bash scripts designed for incorporation into arbitrary projects. The dependency footprint is deliberately narrow — `bash 3.2` and a handful of standard tools — with no Python runtime, no language-specific package manager, and no `gcloud` CLI. A small team can stand up a hardened build pipeline and a sandboxed runtime without specialized DevOps expertise. Recipe Bottle's goal is a workflow where every container image has a verified origin and a controlled version, running behind appropriate network safeguards.

**Project page**: https://scaleinv.github.io/recipebottle

<p align="center">
  <img src="rbm-abstract-drawio.svg" alt="Recipe Bottle architecture diagram" width="720" />
</p>

## Environment

Recipe Bottle addresses two orthogonal complexity domains: the [Foundry](#Foundry) builds container images with verifiable provenance, and the [Crucible](#Crucible) runs untrusted images with enforced network isolation. The two compose but neither requires the other.

<a id="Regime"></a>**[Regime](#Regime).** Structured configuration with typed validation. Each [Regime](#Regime) is a set of environment variables in an `.env` file with a render command (display current values) and a validate command (check correctness). Recipe Bottle uses [Regimes](#Regime) for everything from [Depot](#Depot) identity to [Vessel](#Vessel) build definitions to developer workstation paths.

<a id="Tabtarget"></a>**[Tabtarget](#Tabtarget).** Lightweight shell script in the `tt/` directory serving as the entry point for a single CLI operation. [Tabtargets](#Tabtarget) are named `{colophon}.{frontispiece}.sh` — the colophon routes to the right module, the frontispiece describes what it does. Tab completion narrows by prefix: `tt/rbw-<TAB>` shows all Recipe Bottle operations. The project's `CLAUDE.md` provides a complete command reference; the CLI's interactive walkthroughs are the authoritative procedure source.

## <a id="Foundry"></a>Foundry

Recipe Bottle's remote build orchestration system for producing, attesting, and distributing container images via Google Cloud Build and Google Artifact Registry. The [Foundry](#Foundry) encompasses [Depots](#Depot), [Vessels](#Vessel), [Hallmark](#Hallmark) tracking, and build definitions. Three [Vessel](#Vessel) modes determine how images enter the [Depot](#Depot): [Conjure](#Conjure) (egress-locked build from source with SLSA provenance), [Bind](#Bind) (digest-pinned upstream mirror), and [Graft](#Graft) (local push). Peer to [Crucible](#Crucible), which handles local runtime containment.

The [Foundry](#Foundry) orchestrates Google Cloud Build to produce container images with SLSA attestation, software bills of material, reproducible multi-architecture builds, and digest-pinned toolchains — so every image has a verifiable origin story. Builds run in an egress-locked configuration, drawing from upstream base images mirrored into a project-owned [Depot](#Depot) registry — a fixed, self-contained supply chain independent of third-party registry availability.

### <a id="Depot"></a>Depot

The facility where container images are built and stored — a GCP project with an artifact registry and a storage bucket. The [Payor](#Payor) [Levies](#Levy) a [Depot](#Depot), and the [Governor](#Governor) administers access to it. Each [Depot](#Depot) operates as an independent supply-chain boundary with its own credentials, builds, and registry.

<a id="Levy"></a>**[Levy](#Levy)** — Provision a new [Depot's](#Depot) GCP infrastructure. [Levying](#Levy) creates the GCP project, artifact registry, storage bucket, and build configuration. This is a [Payor](#Payor) operation that binds [Regime](#Regime) configuration to real cloud resources.

<a id="Unmake"></a>**[Unmake](#Unmake)** — Permanently destroy a [Depot's](#Depot) GCP infrastructure — project, artifact registry, storage bucket, and all contents. [Unmaking](#Unmake) is the reverse of [Levying](#Levy). This is a [Payor](#Payor) operation and is irreversible.

Each [Depot](#Depot) operates in one of two build egress profiles:

- <a id="Tethered"></a>**[Tethered](#Tethered)** — Build egress mode allowing public internet access during Cloud Build. [Tethered](#Tethered) builds pull base images from upstream registries at build time — simpler to set up, but dependent on upstream availability. Compare with [Airgap](#Airgap).
- <a id="Airgap"></a>**[Airgap](#Airgap)** — Build egress mode with no public internet access during Cloud Build. [Airgap](#Airgap) builds draw all dependencies from [Enshrined](#Enshrine) images in the [Depot's](#Depot) registry — fully self-contained, independent of upstream availability. Requires [Enshrining](#Enshrine) base images before the first build. Compare with [Tethered](#Tethered).

### <a id="Payor"></a>Payor

Owns the GCP project and funds it; authenticates via OAuth. The [Payor](#Payor) is the only role requiring manual Google Cloud Console interaction — establishing the project, configuring OAuth, and authorizing via browser flow. All other roles descend from credentials the [Payor's](#Payor) infrastructure creates.

### <a id="Governor"></a>Governor

Administers a [Depot](#Depot): creates service accounts, manages access. The [Governor](#Governor) is mantled by the [Payor](#Payor) and holds the administrative credential for the [Depot](#Depot). The [Governor](#Governor) issues two kinds of credentials:

- <a id="Knight"></a>**[Knight](#Knight)** — Create a [Director](#Director) service account. [Knighting](#Knight) provisions a new credential scoped to build and publish access within a single [Depot](#Depot).
- <a id="Charter"></a>**[Charter](#Charter)** — Create a [Retriever](#Retriever) service account. [Chartering](#Charter) provisions a new credential scoped to image pull access within a single [Depot](#Depot).

### <a id="Director"></a>Director

Builds and publishes [Vessel](#Vessel) images into a [Depot](#Depot). Each [Director](#Director) credential is scoped to one [Depot](#Depot). The [Director](#Director) manages the image lifecycle through five operations:

- <a id="Ordain"></a>**[Ordain](#Ordain)** — Create a [Hallmark](#Hallmark) with full attestation — the production build operation. [Ordaining](#Ordain) is mode-aware: it [Conjures](#Conjure), [Binds](#Bind), or [Grafts](#Graft) depending on the [Vessel's](#Vessel) configuration. Each [Ordain](#Ordain) produces an image in the [Depot](#Depot) registry with associated provenance metadata.
- <a id="Tally"></a>**[Tally](#Tally)** — Inventory [Hallmarks](#Hallmark) in the [Depot](#Depot) registry by health status. [Tallying](#Tally) shows which builds succeeded, which are pending, and which failed. The [Director](#Director) [Tallies](#Tally) before [Vouching](#Vouch) to confirm build completion.
- <a id="Vouch"></a>**[Vouch](#Vouch)** — Cryptographic attestation proving a [Hallmark](#Hallmark) was built by trusted infrastructure. The [Vouch](#Vouch) verdict is mode-aware: [Conjure](#Conjure) builds receive full SLSA provenance verification, [Bind](#Bind) builds receive digest-pin verification, and [Graft](#Graft) builds receive a GRAFTED verdict with no provenance chain. The [Director](#Director) [Vouches](#Vouch) [Hallmarks](#Hallmark) after [Tallying](#Tally) their build status.
- <a id="Abjure"></a>**[Abjure](#Abjure)** — Remove a [Hallmark's](#Hallmark) artifacts from the [Depot's](#Depot) registry — the `-image`, `-about`, and `-vouch` tags deleted as a coherent unit. [Abjuring](#Abjure) is the reverse of [Ordaining](#Ordain): it formally renounces a build instance. The [Director](#Director) [Abjures](#Abjure) [Hallmarks](#Hallmark) that are superseded, broken, or no longer needed.
- <a id="Jettison"></a>**[Jettison](#Jettison)** — Delete a specific image tag from the [Depot's](#Depot) registry. [Jettisoning](#Jettison) is lower-level than [Abjure](#Abjure) — it removes a single tag rather than a complete [Hallmark](#Hallmark) artifact set. Used for cleanup of individual registry entries.

### <a id="Retriever"></a>Retriever

Pulls and runs [Vessel](#Vessel) images from a [Depot](#Depot). This is the most constrained role — read-only access to the [Depot](#Depot) registry. The [Retriever](#Retriever) accesses the [Depot](#Depot) registry through two operations:

- <a id="Summon"></a>**[Summon](#Summon)** — Pull a [Hallmark](#Hallmark) image from the [Depot](#Depot) to your local machine. The [Retriever](#Retriever) [Summons](#Summon) [Vouched](#Vouch) images for local use — the final step before a [Hallmark](#Hallmark) can be used in a [Crucible](#Crucible).
- <a id="Plumb"></a>**[Plumb](#Plumb)** — Inspect an artifact's provenance — SBOM, build info, and [Vouch](#Vouch) chain. [Plumbing](#Plumb) provides full transparency into how an image was built and what it contains. Two views are available: full (SBOM, build info, Dockerfile) and compact (attestation summary).

### <a id="Vessel"></a>Vessel

A specification for a container image — built from source ([Conjure](#Conjure)), mirrored from upstream ([Bind](#Bind)), or pushed from local ([Graft](#Graft)). Each [Vessel](#Vessel) is a directory under `rbev-vessels/` containing at minimum an `rbrv.env` configuration file; [Conjure](#Conjure) [Vessels](#Vessel) also include a Dockerfile. Each [Vessel](#Vessel) operates in one of three [Ordain](#Ordain) modes, plus a local development mode:

- <a id="Conjure"></a>**[Conjure](#Conjure)** — Cloud Build creates the image from source. [Conjure](#Conjure) builds run in an egress-locked environment with digest-pinned toolchains, producing full SLSA attestation and SBOMs. This is the highest-trust build mode.
- <a id="Bind"></a>**[Bind](#Bind)** — Mirror an upstream image pinned by digest. [Binding](#Bind) captures an external image at a specific digest into the [Depot's](#Depot) registry. Trust is established through digest-pin verification rather than build provenance.
- <a id="Graft"></a>**[Graft](#Graft)** — Push a locally-built image to the [Depot](#Depot) registry. [Grafting](#Graft) uploads a local image to GAR via docker push — no Cloud Build for the image itself, though about and [Vouch](#Vouch) metadata still run in Cloud Build. This is the lowest-trust mode (GRAFTED verdict).
- <a id="Kludge"></a>**[Kludge](#Kludge)** — Build a [Vessel](#Vessel) image locally for fast iteration, without [Depot](#Depot) registry push. [Kludging](#Kludge) produces a local Docker image for development and testing without involving Cloud Build or the [Depot](#Depot). The resulting image can be used to [Charge](#Charge) a [Crucible](#Crucible) directly.

### <a id="Hallmark"></a>Hallmark

A specific build instance of a [Vessel](#Vessel), identified by timestamp. [Hallmarks](#Hallmark) are the unit of provenance tracking — each one records when and how the image was produced. Each [Hallmark](#Hallmark) produces three tagged artifacts in the [Depot](#Depot) registry: the container image (`-image`), the software bill of materials (`-about`), and the cryptographic attestation (`-vouch`). [Hallmark](#Hallmark) values are recorded into [Nameplate](#Nameplate) [Regime](#Regime) files to pin a [Crucible](#Crucible) to specific image versions.

### Foundry Lifecycle

Recipe Bottle uses a role-based security model with four roles, each building on the previous:

| Role | Authenticates via | Purpose |
|------|-------------------|---------|
| [**Payor**](#Payor) | OAuth (browser flow) | Creates/funds GCP infrastructure, manages [Governor](#Governor) lifecycle |
| [**Governor**](#Governor) | Service account credential | Administers [Director](#Director) and [Retriever](#Retriever) credentials within a [Depot](#Depot) |
| [**Director**](#Director) | Service account credential | Submits builds, manages images, verifies provenance |
| [**Retriever**](#Retriever) | Service account credential | Pulls images for local use |

The [Payor](#Payor) stands apart — it requires manual Google Cloud Console work and OAuth authentication. All downstream roles authenticate via credential files, enabling full automation.

#### Establishment and Provisioning

The [Payor](#Payor) begins by creating a GCP project and configuring an OAuth consent screen through the Google Cloud Console. After downloading the OAuth client credentials, the [Payor](#Payor) installs them via a browser authorization flow — the resulting refresh token is stored locally with restrictive permissions and does not need to be repeated.

With [Payor](#Payor) credentials in place, the [Payor](#Payor) [Levies](#Levy) a [Depot](#Depot), provisioning it with build infrastructure, artifact registry, and secrets storage. The [Payor](#Payor) then mantles a [Governor](#Governor) service account to administer the [Depot](#Depot).

Before the first build can run, the [Depot](#Depot) needs its supply-chain infrastructure in place:

- <a id="Enshrine"></a>**[Enshrine](#Enshrine)** — Mirror upstream base images into your [Depot's](#Depot) registry. [Enshrining](#Enshrine) ensures the build pipeline has a fixed, self-contained supply chain — builds draw from project-owned copies rather than depending on third-party registry availability at build time.
- <a id="Reliquary"></a>**[Reliquary](#Reliquary)** — Co-versioned set of builder tool images (skopeo, docker, gcloud, syft) inscribed from upstream into the [Depot's](#Depot) registry. Cloud Build jobs use [Reliquary](#Reliquary) images as step containers, ensuring builds run with known, project-owned toolchains rather than pulling tools from upstream at build time. The [Director](#Director) inscribes a [Reliquary](#Reliquary) before any [Ordain](#Ordain) or [Enshrine](#Enshrine) operation can run.

#### Credential Distribution

The [Governor](#Governor) creates downstream credentials: [Knighting](#Knight) a [Director](#Director) for build operations and [Chartering](#Charter) a [Retriever](#Retriever) for image pull access. Each credential is scoped to a single role within a single [Depot](#Depot).

#### Build and Retrieve

The [Director](#Director) [Ordains](#Ordain) [Hallmarks](#Hallmark) for each [Vessel](#Vessel) — [Conjuring](#Conjure) from source, [Binding](#Bind) from upstream, or [Grafting](#Graft) from local builds. After builds complete, the [Director](#Director) [Tallies](#Tally) [Hallmarks](#Hallmark) by health status and [Vouches](#Vouch) their provenance. [Hallmark](#Hallmark) values from the [Tally](#Tally) are recorded into [Nameplate](#Nameplate) [Regime](#Regime) files, completing the chain from build to runtime.

The [Retriever](#Retriever) [Summons](#Summon) [Vouched](#Vouch) images locally for use.

Recipe Bottle builds container images on Google Cloud Build (GCB) and stores them in Google Artifact Registry (GAR):

- Isolated build environments using Google-curated Cloud Build builder images
- Multi-architecture support via `docker buildx` with binfmt emulation
- SLSA provenance attestation and verification
- Software Bills of Material (SBOM) for every build
- Full build transcripts captured as auxiliary metadata artifacts
- Upstream base images [Enshrined](#Enshrine) into the [Depot's](#Depot) registry, so builds do not depend on third-party registry availability at build time
- `gcloud` never runs on the workstation — REST calls via `curl` and `jq` drive all remote operations, and the Google-supplied `gcloud` binary is confined to Cloud Build step containers on the server side

Each build passes through a <a id="Pouch"></a>**[Pouch](#Pouch)** — build context packaged as a FROM SCRATCH OCI image and pushed to the [Depot's](#Depot) registry before a Cloud Build job runs. The [Director](#Director) controls what enters the [Pouch](#Pouch) — Dockerfile, context files, build scripts — and the cloud receives only what the [Pouch](#Pouch) contains. This is the security boundary between workstation and build infrastructure.

## <a id="Crucible"></a>Crucible

The distinctive case Recipe Bottle addresses is *running untrusted code*: third-party tooling, experimental packages, binaries with uncertain provenance. Containers excel at packaging known applications, but running unvetted code poses security risks that ordinary container deployment does not solve. Recipe Bottle assembles a [Crucible](#Crucible) — three cooperating containers where a [Sentry](#Sentry) enforces network policy via `iptables` and `dnsmasq` — without requiring modifications to existing container images. The [Bottle](#Bottle) container runs unmodified, in a network namespace prepared by a privileged [Pentacle](#Pentacle), with all egress flowing through the [Sentry](#Sentry) gateway.

The [Sentry](#Sentry)/[Pentacle](#Pentacle)/[Bottle](#Bottle) triad running together as one unit for a [Nameplate](#Nameplate). The [Crucible](#Crucible) is the local safety orchestration — the apparatus that makes running untrusted code practical. [Charging](#Charge) starts all three containers; [Quenching](#Quench) stops and cleans them up.

### <a id="Nameplate"></a>Nameplate

Per-[Vessel](#Vessel) configuration tying a [Sentry](#Sentry) and [Bottle](#Bottle) together into a runnable [Crucible](#Crucible). The [Nameplate](#Nameplate) moniker (e.g. `tadmor`) identifies the unit across all operations. Each [Nameplate](#Nameplate) declares its [Vessel](#Vessel) selections, [Hallmark](#Hallmark) pins, and the network policy that the [Sentry](#Sentry) enforces.

### <a id="Sentry"></a>Sentry

Security container enforcing network policies via `iptables` and `dnsmasq`. The [Sentry](#Sentry) applies two layers of egress policy: DNS-level filtering (only allowed domain names resolve) and IP-level filtering (only allowed CIDR ranges pass). A compromised [Bottle](#Bottle) cannot bypass either layer — the [Sentry](#Sentry) is the sole gateway between the [Bottle](#Bottle) and the outside network.

### <a id="Pentacle"></a>Pentacle

Privileged container establishing the network namespace shared with the [Bottle](#Bottle). The [Pentacle](#Pentacle) runs briefly with elevated privileges to create the network topology, then remains as the namespace anchor. Security policies are enforced from the first packet because the [Sentry](#Sentry) configures the namespace before the [Bottle](#Bottle) starts.

### <a id="Bottle"></a>Bottle

Your workload container, running unmodified in a controlled network environment. The [Bottle](#Bottle) has no direct network access — all traffic routes through the [Sentry](#Sentry) gateway in a namespace prepared by the [Pentacle](#Pentacle). Any existing container image can run as a [Bottle](#Bottle) without modification.

### Crucible Lifecycle

For running containers with network services, Recipe Bottle orchestrates three containers working together:

- **[Sentry](#Sentry)** — enforces network security policies via `iptables` and `dnsmasq`
- **[Pentacle](#Pentacle)** — establishes a privileged network namespace and shares it with the [Bottle](#Bottle)
- **[Bottle](#Bottle)** — your workload container, running unmodified in a controlled network environment

This ensures security policies are enforced from the first packet, and the [Bottle](#Bottle) container experiences only a functional path to its [Sentry](#Sentry) gateway.

The [Sentry](#Sentry) applies two layers of egress policy: `dnsmasq` answers DNS queries only for explicitly allowed names, and `iptables` permits outbound IP traffic only to allowed CIDR ranges. The two layers combine into an enforced allowlist that a compromised or misbehaving [Bottle](#Bottle) cannot bypass — neither by DNS-based exfiltration nor by direct IP connection to an unapproved destination. Each [Crucible's](#Crucible) allowlist is declared in a [Nameplate](#Nameplate) [Regime](#Regime) file alongside the [Sentry](#Sentry) and [Bottle](#Bottle) [Vessel](#Vessel) selections, making network policy a reviewable artifact of the configuration rather than an implicit runtime behavior.

The [Crucible](#Crucible) has two lifecycle operations:

- <a id="Charge"></a>**[Charge](#Charge)** — Start the [Sentry](#Sentry)/[Pentacle](#Pentacle)/[Bottle](#Bottle) triad for a [Nameplate](#Nameplate). [Charging](#Charge) brings up the [Crucible](#Crucible) in dependency order: [Pentacle](#Pentacle) creates the namespace, [Sentry](#Sentry) configures policy, then the [Bottle](#Bottle) starts with its network already constrained.
- <a id="Quench"></a>**[Quench](#Quench)** — Stop and clean up a [Charged](#Charge) [Nameplate's](#Nameplate) containers. [Quenching](#Quench) tears down the [Crucible](#Crucible) in reverse order and removes the network resources created during [Charging](#Charge).

#### Day-to-Day Operations

To run a workload, [Charge](#Charge) the [Crucible](#Crucible) for a [Nameplate](#Nameplate). This starts the [Sentry](#Sentry), [Pentacle](#Pentacle), and [Bottle](#Bottle) containers together — the [Bottle](#Bottle) is ready for interactive use immediately.

For diagnostics, shell into the [Bottle](#Bottle) or the [Sentry](#Sentry), or observe network traffic across the [Crucible's](#Crucible) containers. When finished, [Quench](#Quench) the [Crucible](#Crucible) to stop and clean up all three containers.

To inspect an image's supply chain, [Plumb](#Plumb) its provenance — the full view shows the SBOM, build info, and Dockerfile; the compact view summarizes the attestation chain.

## Project Direction

**[Foundry](#Foundry).** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.

**Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the [Sentry's](#Sentry) CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.

**Networking.** [Bottle](#Bottle)-to-[Bottle](#Bottle) communication is feasible under the current [Sentry](#Sentry) model but not implemented; waiting for a concrete use case.

## Prerequisites

- macOS or Linux workstation
- `bash` (3.2+), `git`, `curl`, `openssh`, `jq`, `openssl`
- `docker` (container runtime)
- A Google Cloud account with billing enabled (credit card required for verification; free tier is sufficient to start)

## Configuration

### User-Configured Regimes

| Regime | File | Purpose |
|--------|------|---------|
| <a id="RBRP"></a>RBRP | `.rbk/rbrp.env` | [Payor](#Payor) GCP project identity |
| <a id="RBRR"></a>RBRR | `.rbk/rbrr.env` | [Depot](#Depot) project, region, build config |
| <a id="RBRN"></a>RBRN | `.rbk/*/rbrn.env` | Per-[Vessel](#Vessel): runtime, [Hallmarks](#Hallmark) |
| <a id="RBRV"></a>RBRV | [Vessel](#Vessel) dirs | Container image build definitions |
| RBRS | station file | Developer machine paths (not in git) |

### Managed Regimes (generated by operations)

| Regime | Purpose | Generated during |
|--------|---------|-----------------|
| RBRO | OAuth refresh token | [Payor](#Payor) installation |
| RBRA | Service account credentials | Mantling, [Knighting](#Knight), [Chartering](#Charter) |

### BUK Base Regimes

| Regime | File | Purpose |
|--------|------|---------|
| <a id="BURC"></a>BURC | `.buk/burc.env` | Project structure ([Tabtarget](#Tabtarget) dir, tools dir) |
| <a id="BURS"></a>BURS | `../station-files/burs.env` | Developer machine (log dir). Not in git. |

## Vessels and Nameplates

**[Vessels](#Vessel)** are build definitions — each is a directory under `rbev-vessels/`:

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

[Conjure](#Conjure) [Vessels](#Vessel) have a Dockerfile and are built by Cloud Build. [Bind](#Bind) [Vessels](#Vessel) (like `rbev-bottle-plantuml`) pin an external image by digest in `rbrv.env` — no Dockerfile, no build step. [Graft](#Graft) [Vessels](#Vessel) push a locally-built image to GAR via docker push — no Cloud Build for the image, but about and [Vouch](#Vouch) still run in Cloud Build. The same [Ordain](#Ordain) operation handles all three: it detects the [Vessel](#Vessel) mode and triggers a Cloud Build ([Conjure](#Conjure)), mirrors from upstream ([Bind](#Bind)), or pushes a local image ([Graft](#Graft)). Trust hierarchy: [Conjure](#Conjure) has full SLSA provenance, [Bind](#Bind) has digest-pin verification, [Graft](#Graft) has no provenance chain (GRAFTED verdict).

**[Nameplates](#Nameplate)** tie [Vessels](#Vessel) together into a runnable [Crucible](#Crucible). The [Nameplate](#Nameplate) moniker (e.g. `tadmor`) identifies the unit across all operations:

```
.rbk/tadmor/rbrn.env          # Maps tadmor → rbev-sentry-debian-slim + rbev-bottle-ifrit
```

[Charging](#Charge) `tadmor` starts the [Crucible](#Crucible) defined by the `tadmor` [Nameplate](#Nameplate), which selects its [Sentry](#Sentry) and [Bottle](#Bottle) [Vessel](#Vessel) images.

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

| Credential | Location | Created during |
|------------|----------|----------------|
| [Payor](#Payor) OAuth | `~/.rbw/rbro.env` | [Payor](#Payor) installation |
| [Governor](#Governor) | `RBRR_SECRETS_DIR/governor/rbra.env` | [Governor](#Governor) mantling |
| [Director](#Director) | `RBRR_SECRETS_DIR/director/rbra.env` | [Director](#Director) [Knighting](#Knight) |
| [Retriever](#Retriever) | `RBRR_SECRETS_DIR/retriever/rbra.env` | [Retriever](#Retriever) [Chartering](#Charter) |

Each credential file is scoped to one role within one [Depot](#Depot) and cannot operate outside its designation.

## Testing

Run test fixtures sequentially — they share regime state and container namespaces. Never run fixtures in parallel.

Two qualification gates exercise the full system: a fast qualify checks [Tabtarget](#Tabtarget) structure, colophon integrity, and [Nameplate](#Nameplate) health; a release qualify adds shellcheck analysis and the complete test suite.

## Recovery

- **Lost OAuth credentials**: Download a fresh JSON key from Google Cloud Console and re-run the [Payor](#Payor) installation
- **Expired tokens**: Run the [Payor](#Payor) refresh operation
- **Compromised [Governor](#Governor)**: Re-mantle the [Governor](#Governor) (replaces the service account, invalidates the old credential)
- **Compromised [Director](#Director)/[Retriever](#Retriever)**: Forfeit the compromised account, then re-[Knight](#Knight) or re-[Charter](#Charter)
- **Lost [Nameplate](#Nameplate) values**: Re-[Tally](#Tally) [Hallmarks](#Hallmark) to retrieve values from the [Depot](#Depot) registry
- **Build timeout or failure**: [Tally](#Tally) to check build status, review logs in the GCP Console for the [Depot](#Depot) project

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
