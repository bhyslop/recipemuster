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

The system uses only `bash`, `git`, `curl`, `openssh`, `jq`, and `docker` natively. No `gcloud` CLI is required on your workstation — cloud operations use REST APIs via `curl` and `jq`.

**Project page**: https://scaleinv.github.io/recipebottle

<p align="center">
  <img src="rbm-abstract-drawio.svg" alt="Recipe Bottle architecture diagram" width="720" />
</p>

## Key Concepts

| Term | Meaning |
|------|---------|
| **Vessel** | A specification for a container image — built from source (conjure), mirrored from upstream (bind), or pushed from local (graft) |
| **Ark** | An immutable container image artifact stored in your own Google Artifact Registry, produced from a vessel |
| **Hallmark** | A specific build instance of a vessel, identified by timestamp |
| **Vouch** | SLSA provenance verification — proves an ark was built by trusted infrastructure |
| **Depot** | The logical facility where container images are built and stored (GCP project + bucket + registry) |
| **Sentry** | Security container that enforces network policies via `iptables` and `dnsmasq` |
| **Pentacle** | Privileged container that establishes the network namespace shared with the bottle |
| **Bottle** | Your workload container, running unmodified in a controlled network environment |
| **Nameplate** | Per-vessel configuration: runtime, vessel names, hallmark values |

## How It Works

### Image Management

Recipe Bottle builds container images on Google Cloud Build (GCB) and stores them in Google Artifact Registry (GAR):

- Isolated build environments using Google-curated Cloud Build builder images
- Multi-architecture support via `docker buildx` with binfmt emulation
- SLSA provenance attestation and verification
- Software Bills of Material (SBOM) for every build
- Full build transcripts captured as auxiliary metadata artifacts

### Bottle Orchestration

For running containers with network services, Recipe Bottle orchestrates three containers working together:

- **Sentry** — enforces network security policies via `iptables` and `dnsmasq`
- **Pentacle** — establishes a privileged network namespace and shares it with the bottle
- **Bottle** — your workload container, running unmodified in a controlled network environment

This ensures security policies are enforced from the first packet, and the bottle container experiences only a functional path to its sentry gateway.

## Prerequisites

- macOS or Linux workstation
- `bash` (3.2+), `git`, `curl`, `openssh`, `jq`
- `docker` (container runtime)
- A Google Cloud account with billing enabled (credit card required for verification; free tier is sufficient to start)

## Using the CLI

All Recipe Bottle operations are **tabtargets** — lightweight shell scripts in the `tt/` directory. Run them from the project root.

**Discover commands**: `ls tt/` shows everything. Tab completion narrows by prefix: `tt/rbw-<TAB>`.

**Naming pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`
- **Colophon**: routing identifier (e.g. `rbw-cC`)
- **Frontispiece**: human-readable description (e.g. `Start`)
- **Imprint**: optional target parameter, often a vessel name (e.g. `tadmor`)

Example: `tt/rbw-cC.Charge.tadmor.sh` charges the `tadmor` crucible.

## Setup

Recipe Bottle uses a role-based security model with four roles:

| Role | Authenticates via | Purpose |
|------|-------------------|---------|
| **Payor** | OAuth (browser flow) | Creates/funds GCP infrastructure, manages governor lifecycle |
| **Governor** | Service account credential | Administers director and retriever credentials within a depot |
| **Director** | Service account credential | Submits builds, manages images, verifies provenance |
| **Retriever** | Service account credential | Pulls images for local use |

The payor stands apart — it requires manual Google Cloud Console work and OAuth authentication. All downstream roles authenticate via credential files, enabling full automation.

### Adaptive Onboarding Guide

The onboarding guide reads your current state and shows exactly what to do next:

```
tt/rbw-gO.Onboarding.sh
```

It probes configuration files, credentials, and local images to determine your progress through 9 levels, then displays the next required step with the exact command to run. Use it alongside the steps below to track your progress.

**Note**: The onboarding guide requires BUK regime files (`.buk/burc.env`, station file) to be in place before it can run. These ship with the repository — you only need to create your station file (see Phase 1 below).

### Phase 1: Payor Establishment

This phase involves manual work in the Google Cloud Console: creating a GCP project, enabling APIs, and configuring an OAuth consent screen. The guided procedure walks you through each click.

1. **Configure payor project** — Edit `.rbk/rbrp.env` and set `RBRP_PAYOR_PROJECT_ID` to your desired GCP project ID (must be globally unique across all of GCP, 6-30 characters, lowercase letters/numbers/hyphens).

2. **Establish payor** — Follow the guided procedure to create the GCP project and configure OAuth consent screen:
   ```
   tt/rbw-gPE.PayorEstablish.sh
   ```
   This displays step-by-step instructions for the Google Cloud Console. At the end you will download a JSON key file containing OAuth client credentials.

3. **Install OAuth credentials** — Ingest the JSON key file downloaded during establishment. A browser window will open for authorization:
   ```
   tt/rbw-gPI.PayorInstall.sh ~/Downloads/client_secret_*.json
   ```
   On success, the refresh token is stored in `~/.rbw/rbro.env` with `600` permissions. You will not need to repeat the browser flow.

### Phase 2: Infrastructure Provisioning

5. **Review depot configuration** — Edit `.rbk/rbrr.env` to set region, machine type, and other depot parameters. Use the render command to review:
   ```
   tt/rbw-rrr.RenderRepoRegime.sh
   ```

6. **Levy depot** — This provisions the GCP depot project with build infrastructure, artifact registry, and secrets. This binds your RBRR configuration to real cloud resources:
   ```
   tt/rbw-PL.PayorLeviesDepot.sh <depot-name>
   ```

7. **Mantle governor** — Admin service account for the depot:
   ```
   tt/rbw-PM.PayorMantlesGovernor.sh
   ```

### Phase 3: Credential Creation (Governor role)

8. **Knight director** — Build service account. The instance name labels this director — use a short identifier:
   ```
   tt/rbw-GK.GovernorKnightsDirector.sh <instance-name>
   ```

9. **Charter retriever** — Image pull service account. Use the same instance name as your director:
   ```
   tt/rbw-GC.GovernorChartersRetriever.sh <instance-name>
   ```

### Phase 4: Build & Retrieve (Director + Retriever roles)

10. **Ordain hallmark** — Build (conjure) or mirror (bind) each vessel's image (typically 10-20 minutes for conjure builds):
    ```
    tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/<vessel-name>
    ```

13. **Check & vouch** — Verify builds completed and SLSA provenance:
    ```
    tt/rbw-Dt.DirectorTalliesConsecrations.sh
    tt/rbw-DV.DirectorVouchesConsecrations.sh
    ```

14. **Record hallmarks** — Copy the hallmark values from the check output into your nameplate regime file:
    ```
    # Edit .rbk/rbrn_<vessel>.env and set:
    RBRN_SENTRY_HALLMARK=c260101120000-r260101130000
    RBRN_BOTTLE_HALLMARK=c260101120000-r260101140000
    ```

15. **Summon** — Pull vouched images locally (Retriever role):
    ```
    tt/rbw-Rs.RetrieverSummonsConsecration.sh <vessel-name> <hallmark>
    ```

## Day-to-Day Operations

The examples below use `tadmor` (the included test nameplate). Replace with your own nameplate moniker — imprints in tabtarget filenames match the nameplate name from `.rbk/*/rbrn.env`.

### Starting a Bottle

```
tt/rbw-cC.Charge.tadmor.sh
```

This charges the crucible — starts the sentry, pentacle, and bottle containers for the vessel.

### Diagnostics

```
tt/rbw-cr.Rack.sh tadmor    # Shell into bottle (rack the demon)
tt/rbw-ch.Hail.sh tadmor    # Shell into sentry (hail the guard)
tt/rbw-cs.Scry.sh tadmor    # Observe network traffic (scry the topology)
```

### Stopping

```
tt/rbw-cQ.Quench.tadmor.sh
```

### Plumbing Provenance

```
tt/rbw-RpF.RetrieverPlumbsFull.sh   # Full: SBOM, build info, Dockerfile
tt/rbw-Rpc.RetrieverPlumbsCompact.sh # Compact summary
```

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

| Credential | Location | Created by |
|------------|----------|------------|
| Payor OAuth | `~/.rbw/rbro.env` | `tt/rbw-gPI.PayorInstall.sh` |
| Governor | `RBRR_SECRETS_DIR/governor/rbra.env` | `tt/rbw-PM.PayorMantlesGovernor.sh` |
| Director | `RBRR_SECRETS_DIR/director/rbra.env` | `tt/rbw-GK.GovernorKnightsDirector.sh` |
| Retriever | `RBRR_SECRETS_DIR/retriever/rbra.env` | `tt/rbw-GC.GovernorChartersRetriever.sh` |

Each credential file is scoped to one role within one depot and cannot operate outside its designation.

## Configuration

Recipe Bottle uses a Config Regime system — structured configuration with typed validation. Each regime has a render command (display current values) and a validate command (check correctness).

### User-Configured Regimes

| Regime | File | Purpose | Render | Validate |
|--------|------|---------|--------|----------|
| RBRP | `.rbk/rbrp.env` | Payor GCP project identity | `tt/rbw-rpr.*` | `tt/rbw-rpv.*` |
| RBRR | `.rbk/rbrr.env` | Depot project, region, build config | `tt/rbw-rrr.*` | `tt/rbw-rrv.*` |
| RBRN | `.rbk/*/rbrn.env` | Per-vessel: runtime, hallmarks | `tt/rbw-rnr.*` | `tt/rbw-rnv.*` |
| RBRV | vessel dirs | Container image build definitions | `tt/rbw-rvr.*` | `tt/rbw-rvv.*` |
| RBRS | station file | Developer machine paths (not in git) | `tt/rbw-rsr.*` | `tt/rbw-rsv.*` |

### Managed Regimes (generated by commands)

| Regime | Purpose | Generated by |
|--------|---------|-------------|
| RBRO | OAuth refresh token | `tt/rbw-gPI.PayorInstall.sh` |
| RBRA | Service account credentials | `tt/rbw-PM.*` / `tt/rbw-GK.*` / `tt/rbw-GC.*` |

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

Conjure vessels have a Dockerfile and are built by Cloud Build. Bind vessels (like `rbev-bottle-plantuml`) pin an external image by digest in `rbrv.env` — no Dockerfile, no build step. Graft vessels push a locally-built image to GAR via docker push — no Cloud Build for the image, but about and vouch still run in Cloud Build. The same `tt/rbw-DO.DirectorOrdainsConsecration.sh` command handles all three: it detects the vessel mode and triggers a Cloud Build (conjure), mirrors from upstream (bind), or pushes a local image (graft). Trust hierarchy: conjure has full SLSA provenance, bind has digest-pin verification, graft has no provenance chain (GRAFTED verdict).

**Nameplates** tie vessels together into a runnable bottle. The nameplate moniker (e.g. `tadmor`) is what appears as the imprint in tabtarget filenames:

```
.rbk/tadmor/rbrn.env          # Maps tadmor → rbev-sentry-debian-slim + rbev-bottle-ifrit
```

So `tt/rbw-cC.Charge.tadmor.sh` charges the crucible defined by the `tadmor` nameplate, which selects its sentry and bottle vessel images.

## Testing

Run test fixtures **sequentially** — they share regime state and container namespaces. Never run fixtures in parallel.

```
tt/rbw-tf.TestFixture.regime-validation.sh
tt/rbw-tf.TestFixture.tadmor-security.sh
```

List all available fixtures and suites:
```
ls tt/rbw-tf.*
ls tt/rbw-ts.*
```

Qualification gates:
```
tt/rbw-Qf.QualifyFast.sh    # Fast: tabtargets, colophons, nameplate health
tt/rbw-QR.QualifyRelease.sh  # Release: + shellcheck, full test suite
```

## Recovery

- **Lost OAuth credentials**: Download a fresh JSON key from Google Cloud Console, re-run `tt/rbw-gPI.PayorInstall.sh`
- **Expired tokens**: `tt/rbw-gPR.PayorRefresh.sh`
- **Compromised governor**: `tt/rbw-PM.PayorMantlesGovernor.sh` (replaces service account, invalidates old credential)
- **Compromised director/retriever**: `tt/rbw-GF.GovernorForfeitsServiceAccount.sh` to revoke, then re-knight with `tt/rbw-GK.*` or re-charter with `tt/rbw-GC.*`
- **Lost nameplate values**: Re-run `tt/rbw-Dt.DirectorTalliesConsecrations.sh` to retrieve hallmark values from the registry
- **Build timeout or failure**: Check build status with `tt/rbw-Dt.DirectorTalliesConsecrations.sh`, review logs in the GCP Console for the depot project

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
