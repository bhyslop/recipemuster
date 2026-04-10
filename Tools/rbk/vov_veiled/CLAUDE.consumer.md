# Claude Code Project Memory — Recipe Bottle

Recipe Bottle enables developers to safely run untrusted containers by interposing a security layer (sentry container) between untrusted containers (bottle containers) and system resources. It uses only `bash`, `git`, `curl`, `openssh`, `jq`, and `docker` natively. Infrastructure runs on Google Cloud Build and Google Artifact Registry with SLSA provenance verification.

Project page: https://scaleinv.github.io/recipebottle

## Getting Started

New users: start with the README.md at the project root. It walks through the full setup sequence from clone to running bottle.

After initial payor setup, the adaptive onboarding guide reads your current state and shows the next step:
```
tt/rbw-go.OnboardMAIN.sh
```

## Glossary

| Term | Meaning |
|------|---------|
| **Vessel** | A specification for a container workload — a directory in `rbev-vessels/` with `rbrv.env` and optionally a `Dockerfile` |
| **Hallmark** | A specific build instance of a vessel, identified by timestamp (e.g. `c260101120000-r260101130000`). The immutable artifact set: image, about, and vouch |
| **Reliquary** | A datestamped namespace in GAR containing co-versioned builder tool images (gcloud, docker, syft, etc.). Created by inscribe; referenced by vessels via `RBRV_RELIQUARY`. One per depot setup. |
| **Crucible** | The three-container assembly (sentry + pentacle + bottle) that runs an untrusted workload under enforced network isolation |
| **Depot** | The logical facility where container images are built and stored (GCP project + bucket + registry) |
| **Nameplate** | Ties a sentry vessel + bottle vessel into a runnable crucible. The moniker (e.g. `tadmor`) is the imprint in tabtargets. |
| **Regime** | A structured configuration unit: specification + assignment file (`.env`) + validation |
| **Sentry** | Security container that enforces network policies via `iptables` and `dnsmasq` |
| **Pentacle** | Privileged container that establishes the network namespace shared with the bottle |
| **Bottle** | Your workload container, running unmodified in a controlled network environment |
| **Ifrit** | A Claude Code instance imprisoned inside a bottle for adversarial escape testing |

## Verb Guide

Recipe Bottle uses domain-specific verbs instead of generic ones (create, delete, start, stop). The frontispiece in each tabtarget filename uses these verbs — this guide maps what you want to do to the vocabulary you will encounter.

### How do I build a container image?

| Verb | What it does |
|------|-------------|
| **inscribe** | Create a reliquary — mirror builder tool images from upstream into a datestamped GAR namespace. Prerequisite for enshrine and ordain. |
| **enshrine** | Copy upstream base images into your private GAR, pinned by content hash. Supply-chain hardening: your builds pull from your own registry. |
| **ordain** | Build a hallmark. Mode-aware: detects the vessel type and dispatches accordingly (conjure, bind, or graft). |
| **conjure** | Build from source via Cloud Build (full SLSA provenance). A mode of ordain, not a separate command. |
| **bind** | Mirror an upstream image pinned by digest (digest-pin verification). A mode of ordain. |
| **graft** | Push a locally-built image to GAR (no provenance chain). A mode of ordain. |
| **kludge** | Build a vessel image locally for development iteration. No Cloud Build, no hallmark — just a local image. |

The supply chain has three layers: inscribe creates the reliquary (tool images), enshrine copies base images, ordain builds your vessel using both.

### How do I verify and inspect images?

| Verb | What it does |
|------|-------------|
| **tally** | Count and classify hallmarks in the registry by health state |
| **vouch** | Verify SLSA provenance — proves a hallmark was built by trusted infrastructure |
| **plumb** | Examine an image's provenance details: SBOM, build info, Dockerfile |

### How do I get images onto my workstation?

| Verb | What it does |
|------|-------------|
| **summon** | Pull a vouched hallmark image locally (full vouch ceremony first) |
| **wrest** | Pull a specific image by reference (direct pull, no vouch) |

### How do I remove images?

| Verb | What it does |
|------|-------------|
| **abjure** | Delete a hallmark's artifacts from GAR (the full set: image + about + vouch) |
| **jettison** | Delete a specific image tag from the registry (surgical, single artifact) |

### How do I run containers?

| Verb | What it does |
|------|-------------|
| **charge** | Start the crucible — bring up sentry, pentacle, and bottle containers |
| **quench** | Stop the crucible — tear down all three containers |

### How do I inspect running containers?

| Verb | What it does |
|------|-------------|
| **rack** | Shell into the bottle container (compel the demon to reveal its state) |
| **hail** | Shell into the sentry container (call out to the guard) |
| **scry** | Observe network traffic across crucible containers (divine the topology) |

### How do I manage infrastructure and credentials?

| Verb | What it does |
|------|-------------|
| **levy** | Provision a depot — GCP project, artifact registry, build infrastructure |
| **unmake** | Permanently remove a depot |
| **mantle** | Create or replace the governor service account (old authority cast off, new bestowed) |
| **knight** | Confer build authority on a director service account |
| **charter** | Grant a retriever read-only registry access |
| **forfeit** | Revoke any service account — seize authority back |

## Roles

Recipe Bottle uses a role-based security model. Each role authenticates differently and has distinct capabilities:

| Role | Authentication | Purpose |
|------|---------------|---------|
| **Payor** | OAuth (browser flow) | Creates/funds GCP infrastructure, manages governor lifecycle |
| **Governor** | Service account credential | Administers director and retriever credentials within a depot |
| **Director** | Service account credential | Submits builds, manages images, verifies provenance |
| **Retriever** | Service account credential | Pulls images for local use |

The payor stands apart — requires manual console work and OAuth. All downstream roles authenticate via credential files, enabling automation without human interaction.

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

- **Payor OAuth**: `~/.rbw/rbro.env` — client secret + refresh token. Only on the administrator's workstation.
- **Governor/Director/Retriever**: credential files at paths defined in RBRR (`RBRR_SECRETS_DIR`). Each file contains a service account credential scoped to one role within one depot.

@Tools/buk/buk-claude-context.md

@Tools/rbk/rbk-claude-tabtarget-context.md

Test suite/fixture tabtargets use `rbtd-s` and `rbtd-r` colophons:
- Available test suites: `ls tt/rbtd-s.TestSuite.*`
- Available test fixtures: `ls tt/rbtd-r.Run.*`

For theurge/ifrit crucible testing work (editing test cases, adding new security probes, debugging test failures), read `Tools/rbk/rbk-claude-theurge-ifrit-context.md` — covers the two-binary architecture, the kludge/charge/test/ordain iteration loop, and how to add new test cases.

### Regime Inspection

Regimes follow a consistent pattern: `rbw-r{code}{r|v|l}` where `r` = render, `v` = validate, `l` = list.

| Code | Regime | Purpose | Render | Validate |
|------|--------|---------|--------|----------|
| `rp` | RBRP | Payor — GCP billing project identity | `rbw-rpr` | `rbw-rpv` |
| `rr` | RBRR | Repo — depot project, region, build config | `rbw-rrr` | `rbw-rrv` |
| `rn` | RBRN | Nameplate — per-vessel hallmarks, runtime | `rbw-rnr` | `rbw-rnv` |
| `rv` | RBRV | Vessel — container image build definitions | `rbw-rvr` | `rbw-rvv` |
| `rs` | RBRS | Station — developer machine paths | `rbw-rsr` | `rbw-rsv` |
| `ro` | RBRO | OAuth — payor refresh token (managed) | `rbw-ror` | `rbw-rov` |
| `ra` | RBRA | Auth — service account credentials (managed) | `rbw-rar` | `rbw-rav` |

**User-configured**: RBRP, RBRR, RBRN, RBRV, RBRS — you edit these during setup.
**Managed/generated**: RBRO (by payor install), RBRA (by credential creation).

Cross-regime operations: `rbw-ni` (nameplate info/survey), `rbw-nv` (validate all nameplates).

### BUK Infrastructure

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `buw-tt-ll` | ListLaunchers | List all registered launchers |
| `buw-rcv` | ValidateConfigRegime | Validate BURC regime |
| `buw-rcr` | RenderConfigRegime | Render BURC regime |
| `buw-rsv` | ValidateStationRegime | Validate BURS regime |
| `buw-rsr` | RenderStationRegime | Render BURS regime |

## Configuration Regimes

A Config Regime is a structured configuration system: a specification document, a shell-sourceable assignment file (`.env`), and validation/render scripts. Each regime uses a unique uppercase variable prefix to prevent collisions.

**Two layers shared by all tools:**
- **BURC** (`.buk/burc.env`) — project structure: tabtarget dir, tools dir, temp/output dirs
- **BURS** (`../station-files/burs.env`) — developer machine: log directory. Not in git.

**Recipe Bottle regimes** (in `.rbk/`):
- **RBRP** — Payor project identity. Set `RBRP_PAYOR_PROJECT_ID` to your GCP project.
- **RBRR** — Repository/depot configuration. Region, machine type, vessel directory, secrets directory, depot project ID.
- **RBRN** — Nameplate. Per-vessel: runtime (`docker`), vessel names, hallmark values (set after builds complete).
- **RBRV** — Vessel definitions. One per container image you want to build.
- **RBRS** — Station. Developer-specific paths for Recipe Bottle. Not in git.

## Architecture

```
Project Root/
├── .buk/                    # BUK launcher directory
│   ├── burc.env             # Project structure config
│   └── launcher.*.sh        # Launcher scripts (environment gates)
├── .rbk/                    # Recipe Bottle config regimes
│   ├── rbrp.env             # Payor regime
│   ├── rbrr.env             # Repo/depot regime
│   └── {moniker}/rbrn.env   # Nameplate regimes (per vessel)
├── tt/                      # TabTargets (ls this to see all commands)
├── Tools/
│   ├── buk/                 # Bash Utility Kit (portable infrastructure)
│   └── rbk/                 # Recipe Bottle Kit (domain logic)
└── rbev-vessels/            # Vessel definitions (rbrv.env + optional Dockerfile per vessel)
```

## Bash Conventions

- **Bash 3.2 compatibility** — works with macOS default shell
- **`set -euo pipefail`** at script start — crash-fast error handling
- **Braced, quoted variable expansion** — always `"${var}"`, never `$var`
- **Functional style** with clear error handling
- **No `gcloud` on workstation** — only `bash`, `curl`, `jq` for cloud operations

## Documentation

- `.adoc` files — AsciiDoc specifications (formal, with linked term vocabulary)
- `.md` files — guides and procedures
- `Tools/buk/README.md` — full BUK infrastructure documentation (tabtargets, launchers, regimes, dispatch)

## Troubleshooting

- **Regime validation fails on startup**: Run the regime's render command to see current values, then validate to identify the specific error. Fix the `.env` file and retry.
- **OAuth token expired**: `tt/rbw-gPR.PayorRefresh.sh`
- **Lost credential file**: Re-run the creation command for that role (payor install, governor mantle, director knight, retriever charter).
- **Tabtarget not found**: Run `tt/rbw-tf.QualifyFast.sh` to check tabtarget and colophon health.
- **Build fails**: Check `tt/rbw-ht.DirectorTalliesHallmarks.sh` for build status. Review logs in the GCP Console for the depot project.
