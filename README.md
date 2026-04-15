# <a id="RecipeBottle"></a>Recipe Bottle

[Recipe Bottle](#RecipeBottle) provides two independent container image capabilities:

- **[Foundry](#Foundry)**: orchestrate Google Cloud Build to produce multiplatform container images (x86 + ARM), fetch, retain, and serve them using a role-managed private cloud registry — with optional [egress-locked](#BuildIsolation) builds and supply-chain [provenance](#Provenance)
- **[Crucible](#Crucible)**: run untrusted containers behind enforced network isolation — DNS filtering and IP filtering — even using images unmodified from "in the wild"

> [!IMPORTANT]
> **Early-stage project — security review welcome in both domains**
>
> The [Foundry's](#Foundry) egress-locked Cloud Build configuration — including the [SLSA](#Provenance) attestation chain, [build isolation](#BuildIsolation), and digest-pinned toolchains — has not yet had broad independent review.
>
> The [Crucible](#Crucible) runtime containment — a multi-container apparatus where the workload runs unprivileged in a network namespace it does not control — has also not had broad review, particularly the network isolation rules, privileged namespace setup, and egress enforcement.
>
> If you evaluate or deploy this, you are contributing to its hardening.
> Security-focused contributors and responsible disclosure are especially valued.

[Recipe Bottle](#RecipeBottle) is a set of bash scripts enabling enterprise grade container image management intended for incorporation into any project.
The dependency footprint is deliberately narrow — `bash 3.2` and a handful of standard tools — with no Python runtime, no language-specific package manager, and no `gcloud` CLI.
After initial manual setup, all cloud API calls use `openssl` + `curl`.
A small team can stand up a hardened build pipeline and a sandboxed runtime without specialized DevOps expertise.
[Recipe Bottle's](#RecipeBottle) goal is a workflow where every container image has a verified origin and a controlled version, running behind appropriate network safeguards.

**Project page**: https://scaleinv.github.io/recipebottle

<p align="center">
  <img src="rbm-abstract-drawio.svg" alt="Recipe Bottle architecture diagram" width="720" />
</p>

## Environment

[Recipe Bottle](#RecipeBottle) is organized around two independent capabilities: the [Foundry](#Foundry) builds container images with verifiable [provenance](#Provenance), and the [Crucible](#Crucible) runs untrusted images with enforced network isolation.
The two compose but neither requires the other.

<a id="Regime"></a>All configuration flows through [Regimes](#Regime) — structured `.env` files with typed validation, each with its own render and validate commands.
Some regimes are committed in the repo: [Vessel](#Vessel) definitions ([RBRV](#RBRV)), [Nameplate](#Nameplate) configurations ([RBRN](#RBRN)), [Depot](#Depot) identity ([RBRR](#RBRR)), and [Payor](#Payor) identity ([RBRP](#RBRP)).
Others live on the filesystem outside revision control: OAuth credentials ([RBRO](#RBRO)), role credentials ([RBRA](#RBRA)), and developer workstation paths ([BURS](#BURS)).

<a id="Tabtarget"></a>Every operation is launched through a [Tabtarget](#Tabtarget) — a shell script in the `tt/` directory.
The critical property: tab completion finds the command you want.
Type `tt/rbw-<TAB>` and the shell narrows to all [Recipe Bottle](#RecipeBottle) operations; type `tt/rbw-h<TAB>` to see just the [Hallmark](#Hallmark) commands.
Each [Tabtarget](#Tabtarget) is named `{colophon}.{frontispiece}.sh` — the colophon routes to the right module, the frontispiece tells you what it does.

<a id="Log"></a>Every state-changing [Tabtarget](#Tabtarget) writes three [Log](#Log) files to the directory named by `BURS_LOG_DIR` in your [BURS](#BURS) station file: a stable-name file (always the same path — easy for tooling to locate and evaluate the most recent run), a per-command file (same command name across runs — tools like SlickEdit sense diffs between executions), and a timestamped historical file (permanent record).
Disk space is cheap; [Log](#Log) unconditionally so the diagnostic evidence is always there when something fails.
Handbooks don't [Log](#Log) — teaching output is ephemeral.

<a id="Transcript"></a>A [Transcript](#Transcript) is a single file capturing key decision points and state transitions within a [Tabtarget's](#Tabtarget) execution.
Where [Logs](#Log) preserve full terminal output, a [Transcript](#Transcript) records the structured progress of sophisticated orchestration commands — the first thing to read when debugging a multi-step failure.

<a id="Output"></a>The [Output](#Output) directory is a fixed-path staging area cleared and recreated before each [Tabtarget](#Tabtarget) runs.
Commands that produce artifacts write them here.
Concurrent bash sessions share this path, so parallel commands can overwrite each other's [Output](#Output).

To begin, run the onboarding walkthrough:

```
tt/rbw-o.OnboardingStartHere.sh
```

## <a id="Foundry"></a>Foundry

[Recipe Bottle's](#RecipeBottle) remote build orchestration system for producing, attesting, and distributing container images via Google Cloud Build and Google Artifact Registry.
The [Foundry](#Foundry) manages [Depot](#Depot) access, [Vessels](#Vessel) choreography, [Hallmark](#Hallmark) tracking, and build definitions.
Three [Vessel](#Vessel) modes determine how images enter the [Depot](#Depot): [Conjure](#Conjure) ([egress-locked](#BuildIsolation) build from source with [SLSA provenance](#Provenance)), [Bind](#Bind) (digest-pinned upstream mirror), and [Graft](#Graft) (local push).
Peer to [Crucible](#Crucible), which handles local runtime containment.

The [Foundry](#Foundry) orchestrates Google Cloud Build to produce container images with [SLSA](#Provenance) attestation, software bills of material, reproducible multi-architecture builds, and digest-pinned toolchains — so every image has a verifiable origin story.
Builds run in an [egress-locked](#BuildIsolation) configuration, drawing from upstream base images mirrored into a project-owned [Depot](#Depot) registry — a fixed, self-contained supply chain independent of third-party registry availability.

### <a id="Depot"></a>Depot

The facility where container images are built and stored — has its own GCP project with an artifact registry and a storage bucket, funded under the [Manor's](#Manor) billing account.
The [Payor](#Payor) [Levies](#Levy) a [Depot](#Depot), and the [Governor](#Governor) administers access to it.
Each [Depot](#Depot) operates as an independent supply-chain boundary with its own credentials, builds, and registry.

Each [Depot](#Depot) supports two build egress profiles:

- <a id="Tethered"></a>**[Tethered](#Tethered)** — Build egress mode allowing public internet access during Cloud Build. [Tethered](#Tethered) builds pull base images from upstream registries at build time — simpler to set up, but dependent on upstream availability.
- <a id="Airgap"></a>**[Airgap](#Airgap)** — Build egress mode with no public internet access during Cloud Build.
[Airgap](#Airgap) builds draw all dependencies from [Enshrined](#Enshrine) images in the [Depot's](#Depot) registry — fully self-contained, independent of upstream availability.
Requires [Enshrining](#Enshrine) base images before the first build.
See [Build Isolation](#BuildIsolation) for the security rationale behind these profiles.

### <a id="Manor"></a>Manor

The [Payor's](#Payor) administrative seat — holds the billing account, OAuth client, and operator identity.
[Depot](#Depot) projects are created and funded under the [Manor's](#Manor) authority.
The [Manor](#Manor) has its own GCP project, distinct from any [Depot](#Depot) project.

### <a id="Payor"></a>Payor

[Establishes](#Establish) a [Manor](#Manor) and funds [Depot](#Depot) projects through it; authenticates via OAuth.
The [Payor](#Payor) is the only role requiring manual Google Cloud Console interaction — [Establishing](#Establish) the [Manor](#Manor), configuring OAuth, and [Installing](#Install) credentials via browser flow.
All other roles descend from credentials the [Payor's](#Payor) infrastructure creates.

### <a id="Governor"></a>Governor

Administers a [Depot](#Depot): creates service accounts, manages access.
The [Governor](#Governor) is [Mantled](#Mantle) by the [Payor](#Payor) and holds the administrative credential for the [Depot](#Depot).
The [Governor](#Governor) [Knights](#Knight) [Directors](#Director) for build access, [Charters](#Charter) [Retrievers](#Retriever) for pull access, and [Forfeits](#Forfeit) credentials when they are no longer needed.

### <a id="Director"></a>Director

Builds and publishes [Vessel](#Vessel) images into a [Depot](#Depot).
Each [Director](#Director) credential is scoped to one [Depot](#Depot).
The [Director](#Director) manages the image lifecycle: [Ordain](#Ordain) a build, [Tally](#Tally) registry health, [Rekon](#Rekon) raw tags, [Vouch](#Vouch) [provenance](#Provenance), [Abjure](#Abjure) superseded artifacts, and [Jettison](#Jettison) individual tags.

### <a id="Retriever"></a>Retriever

Pulls and runs [Vessel](#Vessel) images from a [Depot](#Depot).
This is the most constrained role — read-only access to the [Depot](#Depot) registry.
The [Retriever](#Retriever) [Summons](#Summon) [Vouched](#Vouch) images for local use, [Plumbs](#Plumb) their [provenance](#Provenance), or [Wrests](#Wrest) a specific image directly.

### <a id="Vessel"></a>Vessel

A specification for a container image — built from source ([Conjure](#Conjure)), mirrored from upstream ([Bind](#Bind)), or pushed from local ([Graft](#Graft)).
Each [Vessel](#Vessel) is a directory under `rbev-vessels/` containing at minimum an `rbrv.env` configuration file; [Conjure](#Conjure) [Vessels](#Vessel) also include a Dockerfile.
A fourth mode, [Kludge](#Kludge), builds locally for development without involving the [Depot](#Depot).

### <a id="Hallmark"></a>Hallmark

A specific build instance of a [Vessel](#Vessel), identified by timestamp.
[Hallmarks](#Hallmark) are the unit of [provenance](#Provenance) tracking — each one records when and how the image was produced.
Each [Hallmark](#Hallmark) produces three tagged artifacts in the [Depot](#Depot) registry: the container image (`-image`), the [software bill of materials](#SBOM) ([`-about`](#About)), and the cryptographic attestation ([`-vouch`](#Vouch)).
[Hallmark](#Hallmark) values are recorded into [Nameplate](#Nameplate) [Regime](#Regime) files to pin a [Crucible](#Crucible) to specific image versions.

### Foundry Lifecycle

[Recipe Bottle](#RecipeBottle) uses a role-based security model with four roles, each building on the previous:

| Role | Authenticates via | Purpose |
|------|-------------------|---------|
| [**Payor**](#Payor) | OAuth (browser flow) | Creates/funds GCP infrastructure, manages [Governor](#Governor) lifecycle |
| [**Governor**](#Governor) | Service account credential | Administers [Director](#Director) and [Retriever](#Retriever) credentials within a [Depot](#Depot) |
| [**Director**](#Director) | Service account credential | Submits builds, manages images, verifies [provenance](#Provenance) |
| [**Retriever**](#Retriever) | Service account credential | Pulls images for local use |

The [Payor](#Payor) stands apart — it requires manual Google Cloud Console work and OAuth authentication.
All downstream roles authenticate via credential files, enabling full automation.

#### Establishment and Provisioning

The [Payor](#Payor) begins by [Establishing](#Establish) a GCP project and configuring an OAuth consent screen through the Google Cloud Console.
After downloading the OAuth client credentials, the [Payor](#Payor) [Installs](#Install) them via a browser authorization flow — the resulting refresh token is stored locally with restrictive permissions and does not need to be repeated.

With [Payor](#Payor) credentials in place, the [Payor](#Payor) [Levies](#Levy) a [Depot](#Depot), provisioning it with build infrastructure, artifact registry, and secrets storage.
The [Payor](#Payor) then [Mantles](#Mantle) a [Governor](#Governor) service account to administer the [Depot](#Depot).

Before the first build can run, the [Depot](#Depot) needs its supply-chain infrastructure in place: upstream base images must be [Enshrined](#Enshrine) into the registry, and a [Reliquary](#Reliquary) of builder tool images must be inscribed.

#### Credential Distribution

The [Governor](#Governor) creates downstream credentials: [Knighting](#Knight) a [Director](#Director) for build operations and [Chartering](#Charter) a [Retriever](#Retriever) for image pull access.
Each credential is scoped to a single role within a single [Depot](#Depot).

#### Build and Retrieve

The [Director](#Director) [Ordains](#Ordain) [Hallmarks](#Hallmark) for each [Vessel](#Vessel) — [Conjuring](#Conjure) from source, [Binding](#Bind) from upstream, or [Grafting](#Graft) from local builds.
After builds complete, the [Director](#Director) [Tallies](#Tally) [Hallmarks](#Hallmark) by health status and [Vouches](#Vouch) their [provenance](#Provenance).
[Hallmark](#Hallmark) values from the [Tally](#Tally) are recorded into [Nameplate](#Nameplate) [Regime](#Regime) files, completing the chain from build to runtime.

The [Retriever](#Retriever) [Summons](#Summon) [Vouched](#Vouch) images locally for use.

[Recipe Bottle](#RecipeBottle) builds container images on Google Cloud Build (GCB) and stores them in Google Artifact Registry (GAR):

- Isolated build environments using Google-curated Cloud Build builder images
- Multi-architecture support via `docker buildx` with binfmt emulation
- [SLSA provenance](#Provenance) attestation and verification
- [Software Bills of Material (SBOM)](#SBOM) for every build
- Full build transcripts captured as auxiliary metadata artifacts
- Upstream base images [Enshrined](#Enshrine) into the [Depot's](#Depot) registry, so builds do not depend on third-party registry availability at build time
- `gcloud` never runs on the workstation — REST calls via `curl` and `jq` drive all remote operations, and the Google-supplied `gcloud` binary is confined to Cloud Build step containers on the server side

Each build's source context is packaged as a [Pouch](#Pouch) — the security boundary between workstation and build infrastructure.

## <a id="Crucible"></a>Crucible

The distinctive case [Recipe Bottle](#RecipeBottle) addresses is *running untrusted code*: third-party tooling, experimental packages, binaries with uncertain [provenance](#Provenance).
Containers excel at packaging known applications, but running unvetted code poses security risks that ordinary container deployment does not solve.
[Recipe Bottle](#RecipeBottle) assembles a [Crucible](#Crucible) — three cooperating containers where a [Sentry](#Sentry) enforces network policy — without requiring modifications to existing container images.
The [Bottle](#Bottle) container runs unmodified, in a network namespace prepared by a privileged [Pentacle](#Pentacle), with all egress flowing through the [Sentry](#Sentry) gateway.

The [Sentry](#Sentry)/[Pentacle](#Pentacle)/[Bottle](#Bottle) triad running together as one unit for a [Nameplate](#Nameplate).
The [Crucible](#Crucible) is the local safety orchestration — the apparatus that makes running untrusted code practical.
[Charging](#Charge) starts all three containers; [Quenching](#Quench) stops and cleans them up.

### <a id="Nameplate"></a>Nameplate

Per-[Vessel](#Vessel) configuration tying a [Sentry](#Sentry) and [Bottle](#Bottle) together into a runnable [Crucible](#Crucible).
The [Nameplate](#Nameplate) moniker (e.g. `tadmor`) identifies the unit across all operations.
Each [Nameplate](#Nameplate) declares its [Vessel](#Vessel) selections, [Hallmark](#Hallmark) pins, and the network policy that the [Sentry](#Sentry) enforces.

### Containers

- <a id="Sentry"></a>**[Sentry](#Sentry)** — Security container enforcing network policies via `iptables` and `dnsmasq`.
The [Sentry](#Sentry) applies two layers of egress policy: DNS-level filtering (only allowed domain names resolve) and IP-level filtering (only allowed CIDR ranges pass).
A compromised [Bottle](#Bottle) cannot bypass either layer — the [Sentry](#Sentry) is the sole gateway between the [Bottle](#Bottle) and the outside network.
- <a id="Pentacle"></a>**[Pentacle](#Pentacle)** — Privileged container establishing the network namespace shared with the [Bottle](#Bottle).
The [Pentacle](#Pentacle) runs briefly with elevated privileges to create the network topology, then remains as the namespace anchor.
Security policies are enforced from the first packet because the [Sentry](#Sentry) configures the namespace before the [Bottle](#Bottle) starts.
- <a id="Bottle"></a>**[Bottle](#Bottle)** — Your workload container, running unmodified in a controlled network environment.
The [Bottle](#Bottle) has no direct network access — all traffic routes through the [Sentry](#Sentry) gateway in a namespace prepared by the [Pentacle](#Pentacle).
Any existing container image can run as a [Bottle](#Bottle) without modification.

### Crucible Lifecycle

[Charge](#Charge) the [Crucible](#Crucible) for a [Nameplate](#Nameplate) to start the [Sentry](#Sentry), [Pentacle](#Pentacle), and [Bottle](#Bottle) together — the [Bottle](#Bottle) is ready for interactive use immediately.
[Rack](#Rack) the [Bottle](#Bottle) to shell in, [Hail](#Hail) the [Sentry](#Sentry) to inspect the gateway, or [Scry](#Scry) the network to observe traffic across [Crucible](#Crucible) containers.
When finished, [Quench](#Quench) the [Crucible](#Crucible) to stop and clean up all three containers.
To inspect an image's supply chain, [Plumb](#Plumb) its [provenance](#Provenance) — the full view shows the [SBOM](#SBOM), build info, and Dockerfile; the compact view summarizes the attestation chain.

### Reference Nameplates

Shipped [Nameplates](#Nameplate) demonstrating different [Crucible](#Crucible) configurations.
Each pairs a [Sentry](#Sentry) with a [Bottle](#Bottle) [Vessel](#Vessel) and defines the network policy for that deployment target.

<a id="ccyolo"></a>**[ccyolo](#ccyolo)** — Claude Code sandbox for network-contained AI development.
The [ccyolo](#ccyolo) [Crucible](#Crucible) runs Claude Code inside a [Bottle](#Bottle) that can only reach Anthropic — SSH entry from the workstation, OAuth authentication via copy/paste, everything else blocked.
[Kludge](#Kludge)-only development target: no cloud account, no service account credentials, fully self-contained on the developer's workstation.
The onboarding handbook's first hands-on track teaches the full [Crucible](#Crucible) lifecycle using [ccyolo](#ccyolo).

<a id="tadmor"></a>**[tadmor](#tadmor)** — Adversarial security testing [Crucible](#Crucible).
The [tadmor](#tadmor) [Nameplate](#Nameplate) pairs the [Sentry](#Sentry) with the [Ifrit](#Ifrit) attack [Vessel](#Vessel) under a restrictive network allowlist.
The [Theurge](#Theurge) test orchestrator [Charges](#Charge) [tadmor](#tadmor) and dispatches curated escape attempts to validate that the [Sentry's](#Sentry) containment holds under adversarial conditions.

## Appendix: Foundry Operations

Formal definitions for all [Foundry](#Foundry) operations, organized by lifecycle phase.

### Infrastructure

<a id="Establish"></a>**[Establish](#Establish)** — Guided setup of a new [Manor](#Manor) — creates the [Manor's](#Manor) GCP project and configures the OAuth consent screen through the Google Cloud Console.
[Establishing](#Establish) walks the [Payor](#Payor) through project creation, API enablement, and consent screen configuration — the manual prerequisites before any automated operations can run.

<a id="Install"></a>**[Install](#Install)** — Ingest OAuth client credentials from a downloaded JSON key file.
[Installing](#Install) triggers a browser authorization flow and stores the resulting refresh token locally with restrictive permissions (`600`).
This is a one-time [Payor](#Payor) operation — the refresh token persists until explicitly revoked.

<a id="Levy"></a>**[Levy](#Levy)** — Provision a new [Depot's](#Depot) GCP infrastructure.
[Levying](#Levy) creates the GCP project, artifact registry, storage bucket, and build configuration.
This is a [Payor](#Payor) operation that binds [Regime](#Regime) configuration to real cloud resources.

<a id="Unmake"></a>**[Unmake](#Unmake)** — Permanently destroy a [Depot's](#Depot) GCP infrastructure — project, artifact registry, storage bucket, and all contents.
[Unmaking](#Unmake) is the reverse of [Levying](#Levy).
This is a [Payor](#Payor) operation and is irreversible.

<a id="Refresh"></a>**[Refresh](#Refresh)** — Refresh an expired [Payor](#Payor) OAuth token.
OAuth tokens expire periodically; [Refreshing](#Refresh) re-authenticates via the stored refresh token without repeating the full [Install](#Install) flow.
Run when [Payor](#Payor) operations fail with authentication errors.

<a id="Quota"></a>**[Quota](#Quota)** — Review Cloud Build capacity and usage.
[Quota](#Quota) displays the current build minute allocation, consumption, and any throttling in effect for the [Depot's](#Depot) GCP project.

### Credentials

<a id="Mantle"></a>**[Mantle](#Mantle)** — Create or replace the [Governor](#Governor) service account for a [Depot](#Depot).
[Mantling](#Mantle) is a [Payor](#Payor) operation that provisions the administrative credential — the [Governor](#Governor) inherits the [Payor's](#Payor) authority to manage the [Depot](#Depot) but authenticates via service account key rather than OAuth.

<a id="Knight"></a>**[Knight](#Knight)** — Create a [Director](#Director) service account.
[Knighting](#Knight) provisions a new credential scoped to build and publish access within a single [Depot](#Depot).

<a id="Charter"></a>**[Charter](#Charter)** — Create a [Retriever](#Retriever) service account.
[Chartering](#Charter) provisions a new credential scoped to image pull access within a single [Depot](#Depot).

<a id="Forfeit"></a>**[Forfeit](#Forfeit)** — Revoke a service account credential.
[Forfeiting](#Forfeit) deletes the service account and its key material from the [Depot](#Depot) — the credential becomes permanently unusable.
This is a [Governor](#Governor) operation used when a credential is compromised, no longer needed, or being rotated.

<a id="ListSAs"></a>**[List Service Accounts](#ListSAs)** — Inventory all service accounts issued within a [Depot](#Depot).
Shows [Governor](#Governor), [Director](#Director), and [Retriever](#Retriever) credentials with their creation dates and status.

### Supply Chain

<a id="Enshrine"></a>**[Enshrine](#Enshrine)** — Mirror upstream base images into your [Depot's](#Depot) registry.
[Enshrining](#Enshrine) ensures the build pipeline has a fixed, self-contained supply chain — builds draw from project-owned copies rather than depending on third-party registry availability at build time.

<a id="Reliquary"></a>**[Reliquary](#Reliquary)** — Co-versioned set of builder tool images (skopeo, docker, gcloud, syft) inscribed from upstream into the [Depot's](#Depot) registry.
Cloud Build jobs use [Reliquary](#Reliquary) images as step containers, ensuring builds run with known, project-owned toolchains rather than pulling tools from upstream at build time.
The [Director](#Director) inscribes a [Reliquary](#Reliquary) before any [Ordain](#Ordain) or [Enshrine](#Enshrine) operation can run.

### Building

<a id="Ordain"></a>**[Ordain](#Ordain)** — Create a [Hallmark](#Hallmark) with full attestation — the production build operation.
[Ordaining](#Ordain) is mode-aware: it [Conjures](#Conjure), [Binds](#Bind), or [Grafts](#Graft) depending on the [Vessel's](#Vessel) configuration.
Each [Ordain](#Ordain) produces an image in the [Depot](#Depot) registry with associated [provenance](#Provenance) metadata.

<a id="Conjure"></a>**[Conjure](#Conjure)** — Cloud Build creates the image from source.
[Conjure](#Conjure) builds run in an [egress-locked](#BuildIsolation) environment with digest-pinned toolchains, producing full [SLSA](#Provenance) attestation and [SBOMs](#SBOM).
This is the highest-trust build mode.

<a id="Bind"></a>**[Bind](#Bind)** — Mirror an upstream image pinned by digest.
[Binding](#Bind) captures an external image at a specific digest into the [Depot's](#Depot) registry.
Trust is established through digest-pin verification rather than build [provenance](#Provenance).

<a id="Graft"></a>**[Graft](#Graft)** — Push a locally-built image to the [Depot](#Depot) registry.
[Grafting](#Graft) uploads a local image to GAR via docker push — no Cloud Build for the image itself, though [About](#About) and [Vouch](#Vouch) metadata still run in Cloud Build.
This is the lowest-trust mode (GRAFTED verdict).

<a id="Kludge"></a>**[Kludge](#Kludge)** — Build a [Vessel](#Vessel) image locally for fast iteration, without [Depot](#Depot) registry push.
[Kludging](#Kludge) produces a local Docker image for development and testing without involving Cloud Build or the [Depot](#Depot).
The resulting image can be used to [Charge](#Charge) a [Crucible](#Crucible) directly.

<a id="Pouch"></a>**[Pouch](#Pouch)** — Build context packaged as a FROM SCRATCH OCI image and pushed to the [Depot's](#Depot) registry before a Cloud Build job runs.
The [Director](#Director) controls what enters the [Pouch](#Pouch) — Dockerfile, context files, build scripts — and the cloud receives only what the [Pouch](#Pouch) contains.
This is the security boundary between workstation and build infrastructure.

### Verification

<a id="Tally"></a>**[Tally](#Tally)** — Inventory [Hallmarks](#Hallmark) in the [Depot](#Depot) registry by health status.
[Tallying](#Tally) shows which builds succeeded, which are pending, and which failed.
The [Director](#Director) [Tallies](#Tally) before [Vouching](#Vouch) to confirm build completion.

<a id="Rekon"></a>**[Rekon](#Rekon)** — Raw listing of image tags in the [Depot](#Depot) registry for a [Vessel](#Vessel) package.
[Rekon](#Rekon) is a [Director](#Director)-only diagnostic that shows exactly what exists in the registry without health interpretation.
Where [Tally](#Tally) groups [Hallmarks](#Hallmark) by status, [Rekon](#Rekon) shows the unprocessed tag inventory.

<a id="Vouch"></a>**[Vouch](#Vouch)** — Cryptographic attestation proving a [Hallmark](#Hallmark) was built by trusted infrastructure.
The [Vouch](#Vouch) verdict is mode-aware: [Conjure](#Conjure) builds receive full [SLSA provenance](#Provenance) verification, [Bind](#Bind) builds receive digest-pin verification, and [Graft](#Graft) builds receive a GRAFTED verdict with no [provenance](#Provenance) chain.
The [Director](#Director) [Vouches](#Vouch) [Hallmarks](#Hallmark) after [Tallying](#Tally) their build status.

<a id="About"></a>**[About](#About)** — Build metadata and [software bill of materials](#SBOM) for a [Hallmark](#Hallmark).
The [About](#About) artifact (`-about` tag) contains the [SBOM](#SBOM), build transcript, build configuration snapshot, and key package summaries — bundled as a compressed archive and stored as a Generic Artifact in GAR.
Every [Ordain](#Ordain) produces an [About](#About) alongside the image.

<a id="Plumb"></a>**[Plumb](#Plumb)** — Inspect an artifact's [provenance](#Provenance) — [SBOM](#SBOM), build info, and [Vouch](#Vouch) chain.
[Plumbing](#Plumb) provides full transparency into how an image was built and what it contains.
Two views are available: full ([SBOM](#SBOM), build info, Dockerfile) and compact (attestation summary).

### Distribution

<a id="Summon"></a>**[Summon](#Summon)** — Pull a [Hallmark](#Hallmark) image from the [Depot](#Depot) to your local machine.
The [Retriever](#Retriever) [Summons](#Summon) [Vouched](#Vouch) images for local use — the final step before a [Hallmark](#Hallmark) can be used in a [Crucible](#Crucible).

<a id="Wrest"></a>**[Wrest](#Wrest)** — Pull a specific image from the [Depot](#Depot) registry by reference.
[Wresting](#Wrest) is a direct pull without [Vouch](#Vouch) verification — used when you need a specific image tag regardless of attestation status.
Compare with [Summon](#Summon), which enforces the [Vouch](#Vouch) ceremony.

### Removal

<a id="Abjure"></a>**[Abjure](#Abjure)** — Remove a [Hallmark's](#Hallmark) artifacts from the [Depot's](#Depot) registry — the `-image`, [`-about`](#About), and [`-vouch`](#Vouch) tags deleted as a coherent unit.
[Abjuring](#Abjure) is the reverse of [Ordaining](#Ordain): it formally renounces a build instance.
The [Director](#Director) [Abjures](#Abjure) [Hallmarks](#Hallmark) that are superseded, broken, or no longer needed.

<a id="Jettison"></a>**[Jettison](#Jettison)** — Delete a specific image tag from the [Depot's](#Depot) registry.
[Jettisoning](#Jettison) is lower-level than [Abjure](#Abjure) — it removes a single tag rather than a complete [Hallmark](#Hallmark) artifact set.
Used for cleanup of individual registry entries.

### Diagnostics

<a id="ListDepots"></a>**[List Depots](#ListDepots)** — Inventory all active [Depots](#Depot) visible to the current [Payor](#Payor) credentials.
Shows project IDs, regions, and provisioning status.

<a id="JWTProbe"></a>**[JWT Probe](#JWTProbe)** — Test service account authentication.
The [JWT Probe](#JWTProbe) verifies that a [Governor](#Governor), [Director](#Director), or [Retriever](#Retriever) credential can successfully authenticate to the [Depot's](#Depot) GCP project — useful for diagnosing access failures after credential creation or rotation.

<a id="OAuthProbe"></a>**[OAuth Probe](#OAuthProbe)** — Test [Payor](#Payor) OAuth authentication.
The [OAuth Probe](#OAuthProbe) verifies that the stored refresh token can obtain a valid access token — useful for diagnosing [Payor](#Payor) operation failures before attempting a full [Refresh](#Refresh).

## Appendix: Crucible Operations

Formal definitions for all [Crucible](#Crucible) operations.

### Lifecycle

<a id="Charge"></a>**[Charge](#Charge)** — Start the [Sentry](#Sentry)/[Pentacle](#Pentacle)/[Bottle](#Bottle) triad for a [Nameplate](#Nameplate).
[Charging](#Charge) brings up the [Crucible](#Crucible) in dependency order: [Pentacle](#Pentacle) creates the namespace, [Sentry](#Sentry) configures policy, then the [Bottle](#Bottle) starts with its network already constrained.

<a id="Quench"></a>**[Quench](#Quench)** — Stop and clean up a [Charged](#Charge) [Nameplate's](#Nameplate) containers.
[Quenching](#Quench) tears down the [Crucible](#Crucible) in reverse order and removes the network resources created during [Charging](#Charge).

### Interaction

<a id="Rack"></a>**[Rack](#Rack)** — Shell into a [Bottle](#Bottle) container.
[Racking](#Rack) opens an interactive session inside the running workload — for debugging, inspecting state, or running commands as the [Bottle](#Bottle) user would experience them.

<a id="Hail"></a>**[Hail](#Hail)** — Shell into a [Sentry](#Sentry) container.
[Hailing](#Hail) opens an interactive session on the gateway — for inspecting `iptables` rules, `dnsmasq` configuration, network state, and egress logs.

<a id="Scry"></a>**[Scry](#Scry)** — Observe network traffic across [Crucible](#Crucible) containers.
[Scrying](#Scry) captures packets on the [Crucible's](#Crucible) network interfaces — for verifying that blocked traffic is actually blocked, diagnosing connectivity issues, or watching the [Sentry's](#Sentry) filtering in action.

## Appendix: Adversarial Test Method

The [Crucible's](#Crucible) containment is validated through coordinated escape testing using two components:

- <a id="Ifrit"></a>**[Ifrit](#Ifrit)** — Adversarial attack [Vessel](#Vessel) purpose-built to run inside a [Bottle](#Bottle), seeking escape.
The [Ifrit](#Ifrit) carries scapy (arbitrary packet construction), strace (syscall boundary probing), and a minimal footprint — tools chosen to probe every surface the [Sentry's](#Sentry) containment exposes.
Named for the djinn imprisoned in a bottle.
- <a id="Theurge"></a>**[Theurge](#Theurge)** — Test orchestrator running on the host, outside the [Crucible](#Crucible).
The [Theurge](#Theurge) [Charges](#Charge) a [Crucible](#Crucible) with the [Ifrit](#Ifrit) as its [Bottle](#Bottle), then dispatches curated, reproducible, version-controlled attack scripts targeting specific surfaces: DNS exfiltration, ICMP covert channels, cloud metadata probing, namespace breakout, and direct IP bypass attempts.
Each attack runs inside the [Bottle](#Bottle) while the [Theurge](#Theurge) simultaneously observes the [Sentry's](#Sentry) network from outside — confirming that blocked traffic is actually blocked, not merely unrequested.

The escape tests were developed through adversarial Claude Code sessions with full visibility into the [Sentry's](#Sentry) source, configuration, and the [Recipe Bottle](#RecipeBottle) specification.
The [Ifrit](#Ifrit) [Vessel](#Vessel) is the delivery vehicle; the intelligence came from the authoring process.
Every test that passes is evidence the containment holds — not proof.
The test suite grows as new attack surfaces are identified.

## <a id="Provenance"></a>Appendix: Supply Chain Provenance

Supply chain provenance is a cryptographically signed record of how a container image was produced — what source, what builder, what steps — so that consumers can verify an image came from trusted infrastructure and was not tampered with in transit or at rest.

[Recipe Bottle](#RecipeBottle) achieves [SLSA](https://slsa.dev) v1.0 Build Level 3 for [Conjure](#Conjure) builds, auto-generated by Google Cloud Build.
The [Vouch](#Vouch) step independently verifies each build's DSSE envelope signature against Google's attestor public keys from `projects/verified-builder` KMS — using Python standard library and `openssl` only, with no third-party verifier.

Provenance guarantees are mode-aware:

| [Vessel](#Vessel) Mode | Trust Basis | [Vouch](#Vouch) Verdict |
|------|-------------|------|
| [**Conjure**](#Conjure) | Full SLSA v1.0 Level 3 — signed build provenance from GCB | DSSE envelope signature verification |
| [**Bind**](#Bind) | Digest-pin comparison — image in GAR matches pinned upstream reference | Digest-pin match |
| [**Graft**](#Graft) | Locally built and pushed — no cloud build involvement | GRAFTED (explicit no-provenance marker) |

Deliberately excluded: no `slsa-verifier` binary, no `gcloud` CLI on the workstation, no `jq` in the verification path.
The [Vouch](#Vouch) verifier reconstructs Pre-Authenticated Encoding (PAE), decodes the base64url payload and signature, and verifies via `openssl dgst` against embedded attestor keys — a minimal, auditable trust chain.

## <a id="SBOM"></a>Appendix: Software Bill of Materials

A Software Bill of Materials ([SBOM](#SBOM)) is a machine-readable inventory of every component inside a container image — every OS package, every library, every binary, with versions.
Without one, a container image is an opaque filesystem whose contents you discover by running it, which is exactly the wrong time to find out it ships a vulnerable dependency.

[Recipe Bottle](#RecipeBottle) generates an [SBOM](#SBOM) for every build using [Syft](https://github.com/anchore/syft), scanning each per-platform image during the [About](#About) assembly step.
Each architecture gets its own [SBOM](#SBOM), bundled alongside the build transcript and configuration snapshot in the `-about` artifact stored as a Generic Artifact in GAR.

An [SBOM](#SBOM) enables three hygiene practices that opaque images cannot support:

- **CVE triage before deployment** — when a vulnerability is announced, search your [SBOMs](#SBOM) rather than scanning running containers
- **Pre-deployment audit** — know what you are granting network access to before a [Crucible](#Crucible) is [Charged](#Charge)
- **Build-over-build drift detection** — compare [SBOMs](#SBOM) across [Hallmarks](#Hallmark) to see what changed between builds

The [Plumb](#Plumb) command surfaces [SBOM](#SBOM) contents: the full view shows package inventories; the compact view summarizes key components.

## <a id="BuildIsolation"></a>Appendix: Build Isolation

[Recipe Bottle](#RecipeBottle) supports two build egress profiles — [Tethered](#Tethered) and [Airgap](#Airgap) — that determine whether a Cloud Build job can reach the public internet.
The distinction is not primarily about availability; it is a security boundary that controls what can enter and exit the build environment.

**What [Airgap](#Airgap) protects: exfiltration and supply chain injection.**
If a compromised dependency, build plugin, or Dockerfile instruction executes during your build, an [Airgapped](#Airgap) build cannot phone home — it cannot transmit source code, secrets, or intermediate artifacts to an external endpoint, and it cannot silently fetch malicious payloads.
This is defense-in-depth for proprietary code: even if a build step is compromised, the network is not available as an exfiltration channel.

**The curated gate principle.**
[Airgap](#Airgap) does not mean "nothing external." It means all external content enters through a single auditable gate — the [Enshrine](#Enshrine) ceremony — rather than ad-hoc network fetches during build.
The attack surface collapses from "any URL a Dockerfile mentions" to "the specific digests the [Director](#Director) [Enshrined](#Enshrine)."
Builder tool images enter through a parallel gate: the [Reliquary](#Reliquary), inscribed once and pinned by digest for all subsequent builds.

**What [Airgap](#Airgap) does not protect: the base image contents.**
Base images like `debian-slim` were themselves built with full internet access — `apt-get install` already ran inside them.
The [Airgap](#Airgap) protects *your* build steps on top of those bases, not the base image contents themselves.
Base images are vetted separately: digest-pinned at [Enshrine](#Enshrine) time, inspectable via [SBOM](#SBOM), and stored as project-owned copies in the [Depot's](#Depot) registry.
A [Tethered](#Tethered) build of the base image followed by an [Airgapped](#Airgap) build of your application is the expected pattern — the base image is a known input, your proprietary layers are the protected output.

**Regulatory alignment.**
No framework mandates build-time network blocking by name, but egress-locked builds are the simplest way to evidence several common controls: FedRAMP CM-7 (least functionality) and SC-7 (boundary protection), SOC 2 CC6.1 (logical access) and CC8.1 (change management), and SLSA Level 3's hermetic build requirement.

## Appendix: Roadmap

The following features are not yet implemented but are under consideration:

- **Crucible conduit for cloud services** - Encrypted tunnel from the [Sentry](#Sentry) to a VPC hosting PrivateLink endpoints, enabling [Bottles](#Bottle) to reach cloud AI services (AWS Bedrock, Vertex AI, Azure OpenAI) without exposing floating cloud IP ranges in the CIDR allowlist.
WireGuard terminated at the [Sentry](#Sentry) replaces per-service IP tracking with a single stable VPC CIDR.
Near-term, allowlist-only [Nameplates](#Nameplate) targeting specific service CIDRs and domains work today with existing [Sentry](#Sentry) machinery.
The tunnel adds defense-in-depth for PrivateLink-capable services; SaaS endpoints without PrivateLink (GitHub.com, GitLab.com) remain served by CIDR/domain allowlisting.

- **Credential confinement** - Move cloud service credentials (API keys, IAM keys, SSH keys) from the operator's workstation into the [Crucible](#Crucible), injected via [Nameplate](#Nameplate) [Regime](#Regime) configuration.
The workstation starts the [Crucible](#Crucible) but never holds service credentials — reducing the attack surface from "everything on the workstation" to "one minimal container."
Naturally paired with conduit work: credentials and network access are configured together.

- **VPC Service Controls** - Google Cloud security perimeters that prevent data from being copied out of a project even if an attacker holds valid credentials.
[Recipe Bottle's](#RecipeBottle) Cloud Build architecture uses private pools, which are the prerequisite for VPC enforcement; enabling the controls themselves is deferred until organizational policy or external distribution requires them.
If a VPC is stood up for the crucible conduit architecture, the VPC-SC perimeter should serve both Cloud Build [egress lockdown](#BuildIsolation) and [Bottle](#Bottle) conduit consumers.

- **Cosign container signing** - Cryptographic image signatures independent of registry trust.
Deferred alongside VPC Service Controls until external distribution triggers the need.

- **CDN-aware IP gating** - When allowed domains are CDN-hosted (e.g. Cloudflare), the [Sentry's](#Sentry) CIDR allowlist becomes coarse: DNS-level gating remains precise, but IP-level gating is porous across shared CDN address ranges.
A tighter mechanism is recognized but not yet designed.

- **Podman support** - The spec accommodates Podman as an alternative container runtime, but support is deferred.
On macOS, both Docker and Podman run Linux containers inside a hidden Linux VM — there is no native container runtime on Darwin.
Podman support would require managing that VM's lifecycle within the customer's [Depot](#Depot), adding infrastructure complexity with no architectural advantage over Docker Desktop.

- **[Crucible](#Crucible)-to-[Crucible](#Crucible) networking** - Under the current [Sentry](#Sentry) model, [Bottles](#Bottle) have no direct network path to each other; any inter-[Bottle](#Bottle) communication would route through their respective [Sentries](#Sentry).
The plumbing is feasible but not implemented, pending a concrete use case.

## Appendix: Reference Project

This repository is the reference implementation of [Recipe Bottle](#RecipeBottle).
The annotated tree below maps its files to the concepts defined above.

| Path | Description |
|------|-------------|
| `Project Root/` | |
| `├── CLAUDE.md` | [Claude Code](https://claude.com/claude-code) command reference, glossary, conventions |
| `├── tt/` | 136 [Tabtargets](#Tabtarget) — `tt/rbw-<TAB>` for all operations |
| `├── Tools/` | |
| `│   ├── buk/` | Bash Utility Kit — portable CLI infrastructure |
| `│   └── rbk/` | Recipe Bottle Kit — domain logic |
| `├── .buk/` | [BURC](#BURC) project structure [Regime](#Regime) |
| `├── .rbk/` | [Regime](#Regime) configuration root |
| `│   ��── rbrp.env` | [RBRP](#RBRP) — [Payor](#Payor) identity for this [Depot](#Depot) |
| `│   ├── rbrr.env` | [RBRR](#RBRR) — [Depot](#Depot) identity and build configuration |
| `│   ├── ccyolo/` | [Nameplate](#Nameplate) — [ccyolo](#ccyolo) Claude Code sandbox [Crucible](#Crucible) |
| `│   │   └── rbrn.env` | [RBRN](#RBRN) — [Sentry](#Sentry) + Claude Code, Anthropic-only allowlist |
| `│   ├── tadmor/` | [Nameplate](#Nameplate) — [tadmor](#tadmor) adversarial testing [Crucible](#Crucible) |
| `│   │   └── rbrn.env` | [RBRN](#RBRN) — [Sentry](#Sentry) + [Ifrit](#Ifrit), restrictive allowlist |
| `│   ├── srjcl/` | [Nameplate](#Nameplate) — Jupyter notebook [Crucible](#Crucible) |
| `│   │   └── rbrn.env` | [RBRN](#RBRN) — [Sentry](#Sentry) + Jupyter, academic-domain allowlist |
| `│   └── pluml/` | [Nameplate](#Nameplate) — PlantUML diagram server [Crucible](#Crucible) |
| `│       └── rbrn.env` | [RBRN](#RBRN) — [Sentry](#Sentry) + PlantUML, no-egress allowlist |
| `└── rbev-vessels/` | [Vessel](#Vessel) definitions |
| `    ├── common-sentry-context/` | Shared [Sentry](#Sentry)/[Pentacle](#Pentacle) build context |
| `    │   ├── Dockerfile` | debian-slim + iptables + dnsmasq |
| `    │   ├── rbjs_sentry.sh` | [Sentry](#Sentry) runtime — policy engine |
| `    │   └── rbjp_pentacle.sh` | [Pentacle](#Pentacle) runtime — namespace setup |
| `    ├── rbev-sentry-deb-tether/` | [Conjure](#Conjure) — [Sentry](#Sentry) (tethered, upstream pull) |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Conjure](#Conjure) mode, tether egress |
| `    ├── rbev-sentry-deb-airgap/` | [Conjure](#Conjure) — [Sentry](#Sentry) (airgapped, enshrined bases) |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Conjure](#Conjure) mode, airgap egress |
| `    ├── rbev-bottle-ccyolo/` | [Conjure](#Conjure) — [ccyolo](#ccyolo) Claude Code sandbox |
| `    │   ├── Dockerfile` | node:22-slim + SSH + Claude Code |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Conjure](#Conjure) mode |
| `    ├── rbev-bottle-ifrit/` | [Conjure](#Conjure) — [Ifrit](#Ifrit) attack binary |
| `    │   ├── Dockerfile` | Rust binary + scapy + strace |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Conjure](#Conjure) mode |
| `    ├── rbev-bottle-plantuml/` | [Bind](#Bind) — upstream image pinned by digest |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Bind](#Bind) mode, digest reference |
| `    ├── rbev-bottle-anthropic-jupyter/` | [Conjure](#Conjure) — Jupyter notebook server |
| `    │   ├── Dockerfile` | |
| `    │   └── rbrv.env` | [RBRV](#RBRV) — [Conjure](#Conjure) mode |
| `    └── (4 additional test vessels)` | busybox variants for [Theurge](#Theurge) fixture coverage |

## Appendix: Specific Regimes

<a id="BURC"></a>**[BURC](#BURC)** — Project structure configuration, in the repo.
[Tabtarget](#Tabtarget) directory, tools directory.

<a id="BURS"></a>**[BURS](#BURS)** — Developer workstation configuration.
Not in git.
Log directory, station paths.

<a id="RBRR"></a>**[RBRR](#RBRR)** — [Depot](#Depot) identity and build configuration — populated during [Levy](#Levy), consumed by [Director](#Director) and [Retriever](#Retriever) operations.

<a id="RBRP"></a>**[RBRP](#RBRP)** — [Manor](#Manor) identity — billing account, OAuth client ID, operator email, and the [Manor's](#Manor) GCP project.
In the repo.

<a id="RBRO"></a>**[RBRO](#RBRO)** — [Payor](#Payor) OAuth credentials — client secret and refresh token.
Not in the repo.

<a id="RBRA"></a>**[RBRA](#RBRA)** — Role credentials resident on user workstation, enabling [Governor](#Governor), [Director](#Director), or [Retriever](#Retriever) operations.
One credential file per role per [Depot](#Depot).

<a id="RBRV"></a>**[RBRV](#RBRV)** — [Vessel](#Vessel) configuration specifying [Bind](#Bind), [Conjure](#Conjure), or [Graft](#Graft) mode for creating [Hallmarks](#Hallmark).

<a id="RBRN"></a>**[RBRN](#RBRN)** — Per-[Nameplate](#Nameplate) [Crucible](#Crucible) configuration mapping two [Vessels](#Vessel) — [Sentry](#Sentry) and [Bottle](#Bottle) — with runtime and [Hallmark](#Hallmark) assignments.

## License

Copyright 2026 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0.
