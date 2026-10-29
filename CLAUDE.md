# <a id="rbsoc_context"></a>Recipe Bottle Project Context

Recipe Bottle lets you run untrusted containers safely: a sentry container stands between the untrusted workload, the bottle container, and everything the workload could reach.
Running it needs only `bash` 3.2, `curl`, `openssl`, `jq` and a handful of standard tools, with Docker as the container runtime, and no `gcloud` CLI on the workstation.
Images are built by Google Cloud Build and served from Google Artifact Registry, and an image built from source carries verifiable SLSA provenance.

Project page: [scaleinv.github.io/recipebottle](https://scaleinv.github.io/recipebottle)

## Getting Started

New users start with `README.md` at the project root, which walks the setup from clone to a running bottle.
The onboarding guide reads your current state and shows the next step, from the first payor setup onward:

```
tt/rbw-o.ONBOARDING.sh
```

## Domain Vocabulary

Recipe Bottle names its concepts and operations with words of its own, in its commands and its documentation alike.
When you meet an unfamiliar term, read `README.md`: its glossary, the last section of the page, defines every term in alphabetical order, each under an anchor named for the term.
Link to one directly, as in `README.md#Ordain`, `README.md#Depot` or `README.md#Charge`.

The terms fall into a few families:

- **Architecture**: Foundry, Manor, Depot, Crucible, Vessel, Hallmark, Ark, Nameplate, Regime, Tabtarget
- **Roles**: Payor, Governor, Director, Retriever, the last three being mantles a federated citizen wears
- **Identity**: Citizen, Mantle, Foedus, Sederunt, Terrier, Muniment
- **Containers**: Sentry, Pentacle, Bottle
- **Supply chain**: Lode, Touchmark, Reliquary, Capture
- **Crucible operations**: Charge, Quench, Rack, Hail, Scry
- **Testing**: Ifrit, Theurge

## Verb Guide

Recipe Bottle's operations take domain verbs in place of generic ones such as create, delete, start and stop, and each tabtarget's frontispiece carries its verb.
The tables below map what you want to do to the verb you will meet.

### Building images

| Verb | What it does |
| --- | --- |
| **Conclave** | Capture the co-versioned cohort of builder tool images into one Lode, the reliquary, named by its touchmark; Cloud Build jobs run their steps from its members. |
| **Ensconce** | Capture an upstream base image into your depot's registry as a Lode, recording the exact digest captured, so builds draw from your own copy. |
| **Ordain** | Build a hallmark: it reads the vessel's mode and conjures, binds or grafts. |
| **Conjure** | Build from source in Cloud Build with SLSA provenance; a mode of ordain. |
| **Bind** | Mirror an upstream image pinned by digest; a mode of ordain. |
| **Graft** | Push a locally built image to the depot's registry, with no provenance chain; a mode of ordain. |
| **Kludge** | Build a conjure vessel's image locally for fast iteration, with no Cloud Build and no registry push. |

The supply chain runs in that order: conclave captures the reliquary, ensconce captures base images, and ordain conjures your vessel from both once yoke and Feoff have recorded them into it.

### Verifying and inspecting images

| Verb | What it does |
| --- | --- |
| **Audit** | List the hallmark identifiers in the depot's registry, a flat catalog at the director tier. |
| **Tally** | Classify hallmarks by health, vouched, pending or incomplete; a retriever operation. |
| **Rekon** | Report which of one hallmark's artifacts stand in the registry. |
| **Vouch** | Record a hallmark's verification: SLSA provenance for conjure, the digest pin for bind, a GRAFTED verdict for graft. |
| **Plumb** | Inspect a hallmark's provenance: the full view shows its SBOM, build info and Dockerfile, the compact view its trust posture. |

### Getting and removing images

| Verb | What it does |
| --- | --- |
| **Summon** | Pull a hallmark's artifacts to your workstation under retriever credentials. |
| **Wrest** | Pull one image by its locator, a hallmark member or a Lode member alike, as a direct pull. |
| **Abjure** | Delete a hallmark's whole ark from the registry: its image, its about artifact and its vouch record. |
| **Jettison** | Delete one image tag by its locator, a hallmark member or a Lode member alike. |
| **Banish** | Delete a whole Lode. |

### Running and inspecting containers

| Verb | What it does |
| --- | --- |
| **Charge** | Start a crucible, the sentry first, then the pentacle, then the bottle; it refuses over a live crucible another working tree started. |
| **Quench** | Stop a crucible and remove its network resources; one another working tree started is named and confirmed first. |
| **Rack** | Open a shell inside the bottle. |
| **Hail** | Open a shell on the sentry, for its `iptables` rules, `dnsmasq` configuration and network state. |
| **Scry** | Capture packets across the crucible's network interfaces. |

### Managing infrastructure

| Verb | What it does |
| --- | --- |
| **Establish** | Stand up the manor, its GCP project, billing link, OAuth consent screen and OAuth client, by the one manual procedure, guided through the Google Cloud Console. |
| **Instaurate** | Found the manor's identity substrate: the workforce pool every depot's sign-in trusts, and the terrier that records who holds what. |
| **Affiance** | Seat an external identity provider under the manor's pool as a foedus. |
| **Jilt** | Dissolve one foedus, leaving the pool standing. |
| **Levy** | Provision a depot: its GCP project, artifact registry, build worker pools and three mantle service accounts. |
| **Unmake** | Destroy a depot; its project enters Google's thirty-day deletion retention, recoverable only within that window. |

### Admitting people and signing in

Nobody holds a long-lived key.
An operator is a citizen of a foedus, and a role is a mantle the citizen is admitted to wear.
Signing in opens a Sederunt, a time-bounded session, and each privileged call mints a short-lived token for one mantle.
Novate is the one way to sign in: a cloud command finding no live Sederunt never signs in on its own, but refuses and names `tt/rbw-aN.NovateSederunt.sh`, so run that before cloud work.

| Verb | What it does |
| --- | --- |
| **Avow** | The federated sign-in against the active foedus that opens a Sederunt; only Novate performs it. |
| **Novate** | Open a fresh full-window Sederunt, extinguishing any standing one. |
| **Espy** | Report whether a Sederunt stands and how much runway is left, from the local cache and with no network. |
| **Don** | Mint a short-lived token for one mantle. |
| **Gird** | The payor seats a fresh depot's first governor, the one admission made outside governor wielding. |
| **Brevet** | A governor admits a citizen onto a mantle in a depot. |
| **Unseat** | A governor withdraws one mantle from a citizen, a suspension rather than an erasure. |
| **Attaint** | A governor expels a citizen from the depot entirely. |
| **Rehearse** | Recount the terrier, who holds which mantle across the manor, reading only. |

## Roles

The payor stands apart; the other three roles are mantles worn by federated citizens.

| Role | Authenticates via | Purpose |
| --- | --- | --- |
| **Payor** | OAuth, browser flow | Establishes the manor, funds depots, Girds the first governor |
| **Governor** | Federated Sederunt, governor mantle | Admits citizens to mantles within a depot |
| **Director** | Federated Sederunt, director mantle | Submits builds, manages images, verifies provenance |
| **Retriever** | Federated Sederunt, retriever mantle | Pulls images for local use |

The payor is the only role needing manual console work.
Every other role signs in against the foedus and dons its mantle per call, so authority is short-lived by construction and each use traces to the person who wore it.
Each role, as the project defines it:

<a id="rbsfr_payor"></a>**Payor Role**

An RB Role that enables an Operator to manage billing accounts and create depot projects using OAuth user authentication rather than service accounts.
The [Payor Role](#rbsfr_payor) operates through OAuth 2.0 desktop application flow.
Everyday depot-side management requires no organization-level permissions; the payor's one org-scoped authority is exercising the Manor's command of its GCP Organization — founding the manor's workforce pool at manor setup and pledging or dissolving foedus providers (Affiance, Jilt) — the asymmetry that keeps every RB Depot org-blind.

<a id="rbsfr_governor"></a>**Governor Role**

An RB Role whose Mantle holds Owner permissions on the RB Depot Project without billing authority, granted at Levy; the Citizen donning it administers admission onto the [Director Role](#rbsfr_director) and [Retriever Role](#rbsfr_retriever) Mantles.

<a id="rbsfr_director"></a>**Director Role**

An RB Role that grants an Operator full build lifecycle permissions including Cloud Build submission and RB Container Repository management.

<a id="rbsfr_retriever"></a>**Retriever Role**

An RB Role that allows an Operator read-only access to the RB Container Repository for pulling RB Images.

## Credential Safety

**The payor's OAuth credential is the system's one standing secret**.
There are no service-account key files to protect, back up or rotate: the mantles hold no durable credential at all.

- **Payor OAuth** (`rbro.env`, in the payor's directory under `RBRR_SECRETS_DIR`): the client secret and refresh token, mode `600`, only on the administrator's workstation and never committed.
- **Mantle tokens**: minted per call from a live Sederunt and never written to disk as a durable credential; lose your Sederunt and you Novate again, with nothing to restore.

## Regimes

A regime is a structured family of configuration: a `.env` file of values, each typed and validated, with its own render and validate commands.
Each regime's variables share one uppercase prefix, so no two collide.
Its tabtargets follow one pattern, `rbw-r` and the regime's code, then `r` to render, `v` to validate and, where offered, `l` to list.

| Code | Regime | Holds | Render | Validate |
| --- | --- | --- | --- | --- |
| `rp` | RBRP | Manor identity: billing account, OAuth client, operator email, the payor's project | `rbw-rpr` | `rbw-rpv` |
| `rr` | RBRR | Repository settings: runtime prefix, vessel and secrets directories, Cloud Build limits, the active foedus | `rbw-rrr` | `rbw-rrv` |
| `rd` | RBRD | Depot identity: cloud prefix, moniker, region, build machine type | `rbw-rdr` | `rbw-rdv` |
| `rn` | RBRN | One nameplate: its sentry and bottle vessels, runtime and hallmarks | `rbw-rnr` | `rbw-rnv` |
| `rv` | RBRV | One vessel: how its image is built | `rbw-rvr` | `rbw-rvv` |
| `rf` | RBRF | One foedus: the identity provider's trust values | `rbw-rfr` | `rbw-rfv` |
| `rw` | RBRW | The manor's workforce pool | `rbw-rwr` | `rbw-rwv` |
| `ro` | RBRO | The payor's OAuth credential | `rbw-ror` | `rbw-rov` |

You edit the regimes during setup, save two: payor install writes RBRO, and RBRD is filled at levy and stands for the depot's life, so changing it means a fresh depot.
`tt/rbw-rnl.ListNameplateRegime.sh` lists every nameplate and `tt/rbw-rvl.ListVesselRegime.sh` every vessel; `tt/rbw-ni.NameplateInfo.sh` surveys the nameplates and `tt/rbw-nv.ValidateNameplates.sh` validates them all.

Two Bash Utility Kit regimes underlie every tool:

- **BURC** (`rbmm_moorings/burc.env`): the project's structure, its tabtarget and tools directories, its temp and output roots and the path of the station file; in git.
- **BURS** (`../station-files/burs.env` by default): this machine's settings, the log directory among them; not in git.

| Colophon | Frontispiece | Purpose |
| --- | --- | --- |
| `buw-tt-ll` | ListLaunchers | List every registered launcher |
| `buw-rcv` | ValidateConfigRegime | Validate BURC |
| `buw-rcr` | RenderConfigRegime | Render BURC |
| `buw-rsv` | ValidateStationRegime | Validate BURS |
| `buw-rsr` | RenderStationRegime | Render BURS |

## Layout

```
Project Root/
├── rbmm_moorings/           # the consumer configuration root
│   ├── burc.env             # BURC
│   ├── rbrp.env             # RBRP
│   ├── rbrr.env             # RBRR, naming the active foedus
│   ├── rbrd.env             # RBRD
│   ├── rbrw.env             # RBRW
│   ├── rbmf_foedera/        # one rbef_<foedus>/rbrf.env per standing foedus
│   ├── rben_<moniker>/      # one rbrn.env per nameplate
│   ├── rbmv_vessels/        # one rbev_<vessel>/rbrv.env per vessel, beside its build context
│   └── buml_launchers/      # launcher scripts
├── tt/                      # tabtargets; ls it to see every command
└── Tools/
    ├── buk/                 # Bash Utility Kit, the portable infrastructure
    └── rbk/                 # Recipe Bottle Kit, the domain logic
```

## Commands

`ls tt/` lists every command, and tab completion narrows them by prefix, as in `tt/rbw-<TAB>`.
For each command's folio channel and purpose, read `Tools/rbk/rbai0/rbaie_tabtarget.md`, the full reference generated from the command registry.
`Tools/buk/README.md` documents the Bash Utility Kit whole: tabtargets, launchers, regimes and dispatch.

## Bash Conventions

- Bash 3.2 compatible, so the macOS default shell runs it.
- `set -euo pipefail` at script start, so an error stops the script.
- Every variable expanded braced and quoted, `"${var}"` and never `$var`.
- Functional style with explicit error handling.
- No `gcloud` on the workstation: cloud calls are `curl`, `openssl` and `jq`.

## Testing

Suites, fixtures and cases each have a colophon: `rbw-ts` runs a suite, `rbw-tf` one fixture and `rbw-tc` one case.

- List the suites with `ls tt/rbw-ts.TestSuite.*`.
- Run one fixture with `tt/rbw-tf.FixtureRun.sh <fixture>`, and run `tt/rbw-tc.FixtureCase.sh` with no argument to list the fixtures.

For crucible testing work, editing test cases, adding security probes or debugging a failure, read `Tools/rbk/rbai0/rbaie_theurge.md` first: it covers the two-binary architecture, the kludge, Charge, test and ordain loop, and how to add a case.

## Kit Context

@Tools/buk/buai0/buais_buk.md

@Tools/rbk/rbai0/rbais_rbk.md

@Tools/rbk/rbai0/rbais_conduct.md

## Troubleshooting

- **A regime fails validation**: run its render command to see the values, then its validate command to name the faulty field; fix the `.env` file and retry.
- **The payor's OAuth token is dead**: re-run payor install with the saved client-secret file, `tt/rbw-gPI.PayorInstall.sh «secrets-dir»/client_secrets/client_secret_*.json`; a dead token needs no new file.
- **A command refuses for want of authority**: check the Sederunt with `tt/rbw-as.EspySederunt.sh` and open a fresh one with `tt/rbw-aN.NovateSederunt.sh`; if the Sederunt is healthy you hold no Brevet onto the mantle that command wears, so a governor must Brevet you with `tt/rbw-pB.GovernorBrevetsCitizen.sh`, and `tt/rbw-pr.GovernorRehearsesTerrier.sh` shows who holds what.
- **Federated sign-in fails outright**: Descry the active foedus with `tt/rbw-jd.FoedusDescry.sh`, which reports its provider's health under the manor's pool or names the deficit.
- **A tabtarget is not found**: `tt/rbw-tq.QualifyFast.sh` checks tabtarget and colophon health.
- **A build fails**: `tt/rbw-ft.RetrieverTalliesHallmarks.sh` shows which hallmarks are vouched, pending or incomplete, and the build's own logs stand in the Google Cloud Console under the depot's project.
- **"Already exists" right after a delete**: almost always Google's post-delete read flap rather than stale local state, so wait a few seconds and retry.
