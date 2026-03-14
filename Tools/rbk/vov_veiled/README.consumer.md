# Recipe Bottle

Recipe Bottle enables developers to safely run untrusted containers — a significantly distinct use case from typical container deployments of carefully crafted code.

While containers excel at packaging known applications, running third-party or experimental code poses security risks. Recipe Bottle addresses this by interposing a security layer (sentry container) between untrusted containers (bottle containers) and system resources, without requiring modifications to existing container images.

The system uses only `bash`, `git`, `curl`, `openssh`, `jq`, and `docker` natively. No `gcloud` CLI is required on your workstation — cloud operations use REST APIs via `curl` and `jq`.

**Project page**: https://scaleinv.github.io/recipebottle

## Key Concepts

| Term | Meaning |
|------|---------|
| **Vessel** | A specification for a container workload — what to build (Dockerfile + config) |
| **Ark** | The built result: an immutable container image artifact produced from a vessel |
| **Consecration** | A specific build instance of a vessel, identified by timestamp |
| **Vouch** | SLSA provenance verification — proves an ark was built by trusted infrastructure |
| **Depot** | The logical facility where container images are built and stored (GCP project + bucket + registry) |
| **Rubric repo** | A separate GitLab repository where Cloud Build fetches build instructions. This is a security boundary — Cloud Build never sees your main repository. You define vessels locally; the inscribe command translates them into build instructions and pushes to the rubric repo automatically. |
| **Sentry** | Security container that enforces network policies via `iptables` and `dnsmasq` |
| **Censer** | Privileged container that establishes the network namespace shared with the bottle |
| **Bottle** | Your workload container, running unmodified in a controlled network environment |
| **Nameplate** | Per-vessel configuration: runtime, vessel names, consecration values |

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
- **Censer** — establishes a privileged network namespace and shares it with the bottle
- **Bottle** — your workload container, running unmodified in a controlled network environment

This ensures security policies are enforced from the first packet, and the bottle container experiences only a functional path to its sentry gateway.

## Prerequisites

- macOS or Linux workstation
- `bash` (3.2+), `git`, `curl`, `openssh`, `jq`
- `docker` (container runtime)
- A Google Cloud account with billing enabled (credit card required for verification; free tier is sufficient to start)
- A GitLab account (the rubric repo requires GitLab's repository-scoped project access tokens, which Cloud Build's v2 connection API needs)

## Using the CLI

All Recipe Bottle operations are **tabtargets** — lightweight shell scripts in the `tt/` directory. Run them from the project root.

**Discover commands**: `ls tt/` shows everything. Tab completion narrows by prefix: `tt/rbw-<TAB>`.

**Naming pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`
- **Colophon**: routing identifier (e.g. `rbw-s`)
- **Frontispiece**: human-readable description (e.g. `Start`)
- **Imprint**: optional target parameter, often a vessel name (e.g. `nsproto`)

Example: `tt/rbw-s.Start.nsproto.sh` starts the `nsproto` vessel's bottle.

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

4. **Configure GitLab** — Set up the rubric repo. This is a separate, minimal repository that serves as the security boundary between your project and Google. You never edit it directly — the inscribe command pushes build instructions there automatically.
   ```
   tt/rbw-gPL.GitLabSetup.sh
   ```

### Phase 2: Infrastructure Provisioning

5. **Review depot configuration** — Edit `.rbk/rbrr.env` to set region, machine type, and other depot parameters. Use the render command to review:
   ```
   tt/rbw-rrr.RenderRepoRegime.sh
   ```

6. **Create depot** — This provisions the GCP depot project with build infrastructure, artifact registry, and secrets. This binds your RBRR configuration to real cloud resources:
   ```
   tt/rbw-PC.PayorCreatesDepot.sh <depot-name>
   ```

7. **Create governor** — Admin service account for the depot:
   ```
   tt/rbw-PG.PayorResetsGovernor.sh
   ```

### Phase 3: Credential Creation (Governor role)

8. **Create director** — Build service account. The instance name labels this director — use a short identifier:
   ```
   tt/rbw-GD.GovernorCreatesDirector.sh <instance-name>
   ```

9. **Create retriever** — Image pull service account. Use the same instance name as your director:
   ```
   tt/rbw-GR.GovernorCreatesRetriever.sh <instance-name>
   ```

### Phase 4: Build & Retrieve (Director + Retriever roles)

10. **Refresh pins** — Lock GCB tool images and slsa-verifier to exact digests for reproducible builds:
    ```
    tt/rbw-DPG.DirectorRefreshesGcbPins.sh
    tt/rbw-DPB.DirectorRefreshesBinaryPins.sh
    ```
    Commit the updated pin file before inscribing:
    ```
    git add .rbk/rbrg.env && git commit -m "Refresh GCB pins for inscribe"
    ```

11. **Inscribe** — Translate your vessel definitions into Cloud Build instructions and push to the rubric repo:
    ```
    tt/rbw-DI.DirectorInscribesRubric.sh
    ```

12. **Create ark** — Build (conjure) or mirror (bind) each vessel's image (typically 10-20 minutes for conjure builds):
    ```
    tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/<vessel-name>
    ```

13. **Check & vouch** — Verify builds completed and SLSA provenance:
    ```
    tt/rbw-Dc.DirectorChecksConsecrations.sh
    tt/rbw-DV.DirectorVouchesConsecrations.sh
    ```

14. **Record consecrations** — Copy the consecration values from the check output into your nameplate regime file:
    ```
    # Edit .rbk/rbrn_<vessel>.env and set:
    RBRN_SENTRY_CONSECRATION=i20260101_120000-b20260101_130000
    RBRN_BOTTLE_CONSECRATION=i20260101_120000-b20260101_140000
    ```

15. **Summon** — Pull vouched images locally (Retriever role):
    ```
    tt/rbw-Rs.RetrieverSummonsArk.sh <vessel-name> <consecration>
    ```

## Day-to-Day Operations

The examples below use `nsproto` (the included test nameplate). Replace with your own nameplate moniker — imprints in tabtarget filenames match the nameplate name from `.rbk/rbrn_*.env`.

### Starting a Bottle

```
tt/rbw-s.Start.nsproto.sh
```

This starts the sentry, censer, and bottle containers for the vessel.

### Connecting

```
tt/rbw-B.ConnectBottle.nsproto.sh    # Shell into bottle
tt/rbw-S.ConnectSentry.nsproto.sh    # Shell into sentry
tt/rbw-o.ObserveNetworks.nsproto.sh  # View network state
```

### Stopping

```
tt/rbw-z.Stop.nsproto.sh
```

### Inspecting Provenance

```
tt/rbw-RiF.RetrieverInspectsFull.sh   # Full: SBOM, build info, Dockerfile
tt/rbw-Ric.RetrieverInspectsCompact.sh # Compact summary
```

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

| Credential | Location | Created by |
|------------|----------|------------|
| Payor OAuth | `~/.rbw/rbro.env` | `tt/rbw-gPI.PayorInstall.sh` |
| Governor | `RBRR_SECRETS_DIR/rbra-governor.env` | `tt/rbw-PG.PayorResetsGovernor.sh` |
| Director | `RBRR_SECRETS_DIR/rbra-director.env` | `tt/rbw-GD.GovernorCreatesDirector.sh` |
| Retriever | `RBRR_SECRETS_DIR/rbra-retriever.env` | `tt/rbw-GR.GovernorCreatesRetriever.sh` |

Each credential file is scoped to one role within one depot and cannot operate outside its designation.

## Configuration

Recipe Bottle uses a Config Regime system — structured configuration with typed validation. Each regime has a render command (display current values) and a validate command (check correctness).

### User-Configured Regimes

| Regime | File | Purpose | Render | Validate |
|--------|------|---------|--------|----------|
| RBRP | `.rbk/rbrp.env` | Payor GCP project identity | `tt/rbw-rpr.*` | `tt/rbw-rpv.*` |
| RBRR | `.rbk/rbrr.env` | Depot project, region, build config | `tt/rbw-rrr.*` | `tt/rbw-rrv.*` |
| RBRN | `.rbk/rbrn_*.env` | Per-vessel: runtime, consecrations | `tt/rbw-rnr.*` | `tt/rbw-rnv.*` |
| RBRV | vessel dirs | Container image build definitions | `tt/rbw-rvr.*` | `tt/rbw-rvv.*` |
| RBRS | station file | Developer machine paths (not in git) | `tt/rbw-rsr.*` | `tt/rbw-rsv.*` |

### Managed Regimes (generated by commands)

| Regime | Purpose | Generated by |
|--------|---------|-------------|
| RBRO | OAuth refresh token | `tt/rbw-gPI.PayorInstall.sh` |
| RBRG | GCB image + binary digest pins | `tt/rbw-DPG.*` / `tt/rbw-DPB.*` |
| RBRA | Service account credentials | `tt/rbw-PG.*` / `tt/rbw-GD.*` / `tt/rbw-GR.*` |

### BUK Base Regimes

| Regime | File | Purpose |
|--------|------|---------|
| BURC | `.buk/burc.env` | Project structure (tabtarget dir, tools dir) |
| BURS | `../station-files/burs.env` | Developer machine (log dir). Not in git. |

## Vessels and Nameplates

**Vessels** are build definitions — each is a directory under `rbev-vessels/`:

```
rbev-vessels/
├── rbev-sentry-ubuntu-large/   # Sentry vessel (conjure — built from source)
│   ├── Dockerfile
│   └── rbrv.env
├── rbev-bottle-ubuntu-test/    # Bottle vessel (conjure — built from source)
│   ├── Dockerfile
│   └── rbrv.env
└── rbev-bottle-plantuml/       # PlantUML server (bind — upstream image pinned by digest)
    └── rbrv.env
```

Conjure vessels have a Dockerfile and are built by Cloud Build. Bind vessels (like `rbev-bottle-plantuml`) pin an external image by digest in `rbrv.env` — no Dockerfile, no build step. The same `tt/rbw-DC.DirectorCreatesArk.sh` command handles both: it detects the vessel mode and either triggers a Cloud Build (conjure) or mirrors the upstream image to GAR (bind). Trust for bind vessels is the digest pin itself.

**Nameplates** tie vessels together into a runnable bottle. The nameplate moniker (e.g. `nsproto`) is what appears as the imprint in tabtarget filenames:

```
.rbk/rbrn_nsproto.env          # Maps nsproto → rbev-sentry-ubuntu-large + rbev-bottle-ubuntu-test
```

So `tt/rbw-s.Start.nsproto.sh` starts the bottle defined by the `nsproto` nameplate, which selects its sentry and bottle vessel images.

## Testing

Run test fixtures **sequentially** — they share regime state and container namespaces. Never run fixtures in parallel.

```
tt/rbw-tf.TestFixture.regime-validation.sh
tt/rbw-tf.TestFixture.nsproto-security.sh
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
- **Compromised governor**: `tt/rbw-PG.PayorResetsGovernor.sh` (replaces service account, invalidates old credential)
- **Compromised director/retriever**: `tt/rbw-GS.GovernorDeletesServiceAccount.sh` to revoke, then recreate with `tt/rbw-GD.*` or `tt/rbw-GR.*`
- **Lost nameplate values**: Re-run `tt/rbw-Dc.DirectorChecksConsecrations.sh` to retrieve consecration values from the registry
- **Build timeout or failure**: Check build status with `tt/rbw-Dc.DirectorChecksConsecrations.sh`, review logs in the GCP Console for the depot project

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
