# Claude Code Project Memory — Recipe Bottle

Recipe Bottle enables developers to safely run untrusted containers by interposing a security layer (sentry container) between untrusted containers (bottle containers) and system resources. It uses only `bash`, `git`, `curl`, `openssh`, `jq`, and `docker` natively. Infrastructure runs on Google Cloud Build and Google Artifact Registry with SLSA provenance verification.

Project page: https://scaleinv.github.io/recipebottle

## Getting Started

New users: start with the README.md at the project root. It walks through the full setup sequence from clone to running bottle.

After initial payor setup, the adaptive onboarding guide reads your current state and shows the next step:
```
tt/rbw-gO.Onboarding.sh
```

## Glossary

| Term | Meaning |
|------|---------|
| **Vessel** | A specification for a container workload — a directory in `rbev-vessels/` with `rbrv.env` and optionally a `Dockerfile` |
| **Ark** | The built result: an immutable container image artifact produced from a vessel |
| **Consecration** | A specific build instance of a vessel, identified by timestamp (e.g. `i20260101_120000-b20260101_130000`) |
| **Vouch** | SLSA provenance verification — proves an ark was built by trusted infrastructure |
| **Summon** | Pull a vouched ark image to the local workstation |
| **Conjure** | Trigger a Cloud Build to produce an ark from a vessel |
| **Inscribe** | Push build definitions from the local repo to the rubric repo for Cloud Build |
| **Abjure** | Revoke vouch on an ark (mark as no longer trusted) |
| **Depot** | The logical facility where container images are built and stored (GCP project + bucket + registry) |
| **Nameplate** | Ties a sentry vessel + bottle vessel into a runnable bottle. The moniker (e.g. `nsproto`) is the imprint in tabtargets. |
| **Regime** | A structured configuration unit: specification + assignment file (`.env`) + validation |
| **Sentry** | Security container that enforces network policies via `iptables` and `dnsmasq` |
| **Censer** | Privileged container that establishes the network namespace shared with the bottle |
| **Bottle** | Your workload container, running unmodified in a controlled network environment |
| **Rubric** | A separate GitLab repo where Cloud Build fetches build instructions (security boundary) |

## Roles

Recipe Bottle uses a role-based security model. Each role authenticates differently and has distinct capabilities:

| Role | Authentication | Purpose |
|------|---------------|---------|
| **Payor** | OAuth (browser flow) | Creates/funds GCP infrastructure, manages governor lifecycle |
| **Governor** | Service account credential | Administers director and retriever credentials within a depot |
| **Director** | Service account credential | Submits builds, manages images, verifies provenance |
| **Retriever** | Service account credential | Pulls images for local use |

The payor stands apart — requires manual console work and OAuth. All downstream roles authenticate via credential files, enabling automation without human interaction.

## Forbidden Shell Operations

**Never use `cd` in Bash commands — NO exceptions.**

The working directory persists between Bash tool calls. A single `cd` corrupts ALL subsequent commands that use relative paths, including every `./tt/` tabtarget.

- Use absolute paths instead of cd'ing

**There is no safe cd.** Do not reason that "I'll cd back" — the next tool call may be yours or another Claude Code session's, and it will break.

## Credential Safety

All credential files require `600` permissions and must never be committed to version control.

- **Payor OAuth**: `~/.rbw/rbro.env` — client secret + refresh token. Only on the administrator's workstation.
- **Governor/Director/Retriever**: credential files at paths defined in RBRR (`RBRR_SECRETS_DIR`). Each file contains a service account credential scoped to one role within one depot.

## Test Execution Discipline

Run test fixture tabtargets **sequentially, never in parallel**. Test fixtures share regime state and container/network namespaces — parallel execution causes resource conflicts and false failures.

```
# Correct: run one at a time
tt/rbw-tf.TestFixture.regime-validation.sh
tt/rbw-tf.TestFixture.nsproto-security.sh

# Wrong: never run fixtures concurrently
tt/rbw-tf.TestFixture.regime-validation.sh & tt/rbw-tf.TestFixture.nsproto-security.sh &
```

## TabTarget System

TabTargets are lightweight shell scripts in `tt/` that serve as the CLI entry point for all operations. They delegate to workbenches via launchers — no business logic lives in tabtargets.

**Discoverability**: `ls tt/` shows all available commands. Tab completion narrows by prefix: `tt/rbw-<TAB>`.

**Naming pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`

| Part | Purpose | Example |
|------|---------|---------|
| **Colophon** | Routing identifier (workbench matches on this) | `rbw-B` |
| **Frontispiece** | Human-readable description (PascalCase) | `ConnectBottle` |
| **Imprint** | Optional target parameter (nameplate moniker, fixture name, etc.) | `nsproto` |

Example: `tt/rbw-B.ConnectBottle.nsproto.sh` — colophon `rbw-B` routes to the bottle connect command, frontispiece tells you what it does, imprint `nsproto` selects the nameplate.

Multiple tabtargets can share the same colophon but differ by imprint:
```
tt/rbw-s.Start.nsproto.sh
tt/rbw-s.Start.srjcl.sh
tt/rbw-s.Start.pluml.sh
```

For full BUK infrastructure documentation, see `Tools/buk/README.md`.

## Command Reference

### Setup & Onboarding (Payor role)

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-gPE` | PayorEstablish | Guided GCP project + OAuth consent screen setup |
| `rbw-gPI` | PayorInstall | Ingest OAuth credentials from JSON key file |
| `rbw-gPL` | GitLabSetup | Rubric repo + project access token |
| `rbw-PC` | PayorCreatesDepot | Provision GCP depot project |
| `rbw-PG` | PayorResetsGovernor | Create/reset governor service account |
| `rbw-Pl` | PayorListsDepots | List all active depots |
| `rbw-PD` | PayorDestroysDepot | Permanently remove a depot |
| `rbw-gO` | Onboarding | Adaptive guide — reads current state, shows next step |
| `rbw-gPR` | PayorRefresh | Refresh expired OAuth tokens |

### Credential Administration (Governor role)

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-GD` | GovernorCreatesDirector | Provision director (build) service account |
| `rbw-GR` | GovernorCreatesRetriever | Provision retriever (image pull) service account |
| `rbw-Gl` | GovernorListsServiceAccounts | List issued service accounts |
| `rbw-GS` | GovernorDeletesServiceAccount | Revoke a service account |

### Build Lifecycle (Director role)

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-DPG` | DirectorRefreshesGcbPins | Resolve latest GCB tool image digests |
| `rbw-DPB` | DirectorRefreshesBinaryPins | Resolve latest slsa-verifier binary |
| `rbw-DI` | DirectorInscribesRubric | Push build definitions to rubric repo |
| `rbw-DC` | DirectorConjuresArk | Trigger Cloud Build for a vessel |
| `rbw-Dc` | DirectorChecksConsecrations | Verify builds completed |
| `rbw-DV` | DirectorVouchesConsecrations | SLSA provenance verification |
| `rbw-DA` | DirectorAbjuresArk | Revoke vouch on an ark |
| `rbw-DD` | DirectorDeletesImage | Remove image from registry |

### Retrieval & Inspection (Retriever role)

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-Rs` | RetrieverSummonsArk | Pull vouched image locally |
| `rbw-Rr` | RetrieverRetrievesImage | Raw image pull |
| `rbw-RiF` | RetrieverInspectsFull | Full provenance display (SBOM, build info, Dockerfile) |
| `rbw-Ric` | RetrieverInspectsCompact | Compact provenance summary |

### Bottle Operations (imprint = nameplate moniker)

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-s` | Start | Start bottle (sentry + censer + bottle containers) |
| `rbw-z` | Stop | Stop bottle |
| `rbw-B` | ConnectBottle | Shell into the bottle container |
| `rbw-S` | ConnectSentry | Shell into the sentry container |
| `rbw-C` | ConnectCenser | Shell into the censer container |
| `rbw-o` | ObserveNetworks | Display network state for a running bottle |

### Qualification & Testing

| Colophon | Frontispiece | Purpose |
|----------|-------------|---------|
| `rbw-Qf` | QualifyFast | Fast qualify: tabtargets, colophons, nameplate health |
| `rbw-QR` | QualifyRelease | Release qualify: + shellcheck, full test suite |
| `rbw-tf` | TestFixture | Run a single test fixture (imprint = fixture name) |
| `rbw-ts` | TestSuite | Run a test suite (imprint = suite name) |
| `rbw-to` | TestOne | Run a single test case |

Available test fixtures: `ls tt/rbw-tf.*`
Available test suites: `ls tt/rbw-ts.*`

### Regime Inspection

Regimes follow a consistent pattern: `rbw-r{code}{r|v|l}` where `r` = render, `v` = validate, `l` = list.

| Code | Regime | Purpose | Render | Validate |
|------|--------|---------|--------|----------|
| `rp` | RBRP | Payor — GCP billing project identity | `rbw-rpr` | `rbw-rpv` |
| `rr` | RBRR | Repo — depot project, region, build config | `rbw-rrr` | `rbw-rrv` |
| `rn` | RBRN | Nameplate — per-vessel consecrations, runtime | `rbw-rnr` | `rbw-rnv` |
| `rv` | RBRV | Vessel — container image build definitions | `rbw-rvr` | `rbw-rvv` |
| `rs` | RBRS | Station — developer machine paths | `rbw-rsr` | `rbw-rsv` |
| `ro` | RBRO | OAuth — payor refresh token (managed) | `rbw-ror` | `rbw-rov` |
| `rg` | RBRG | Pins — GCB image + binary digests (managed) | `rbw-rgr` | `rbw-rgv` |
| `ra` | RBRA | Auth — service account credentials (managed) | `rbw-rar` | `rbw-rav` |

**User-configured**: RBRP, RBRR, RBRN, RBRV, RBRS — you edit these during setup.
**Managed/generated**: RBRO (by payor install), RBRG (by pin refresh), RBRA (by credential creation).

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
- **RBRR** — Repository/depot configuration. Region, machine type, vessel directory, secrets directory, rubric repo URL, depot project ID.
- **RBRN** — Nameplate. Per-vessel: runtime (`docker`), vessel names, consecration values (set after builds complete).
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
│   ├── rbrn_*.env           # Nameplate regimes (per vessel)
│   └── rbrg.env             # GCB pins (managed)
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
- **Lost credential file**: Re-run the creation command for that role (payor install, governor reset, director/retriever create).
- **Tabtarget not found**: Run `tt/rbw-Qf.QualifyFast.sh` to check tabtarget and colophon health.
- **Build fails**: Check `tt/rbw-Dc.DirectorChecksConsecrations.sh` for build status. Review logs in the GCP Console for the depot project.
