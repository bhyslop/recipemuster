## Goal

Make depot-time-immutable settings legible and enforceable. Today, four RBRR settings are read at depot levy and baked into Google Cloud resources; altering them post-levy is a silent no-op that confuses users. Close this gap with two coordinated changes: a renaming that makes the lifecycle visible in the name, and a tripwire that fails loudly on drift.

## The four depot-time settings

Read at depot levy, baked into persistent GCP resources, immutable thereafter without relevying:

- `RBRR_CLOUD_PREFIX` — composed into project ID, GAR repo name, pool stem, GCS bucket name (`rbdc_DerivedConstants.sh`)
- `RBRR_DEPOT_MONIKER` — same composition as above (`rbdc_DerivedConstants.sh`)
- `RBRR_GCP_REGION` — embedded in worker pool path, GCS bucket location, GAR location (`rbgp_Payor.sh` depot-levy path)
- `RBRR_GCB_MACHINE_TYPE` — baked into tether and airgap worker pool `workerConfig.machineType` (`rbgp_Payor.sh` depot-levy path)

The remaining eight RBRR settings are live (re-read per operation) and stay where they are: `RBRR_GCB_TIMEOUT`, `RBRR_GCB_MIN_CONCURRENT_BUILDS`, `RBRR_VESSEL_DIR`, `RBRR_BOTTLE_WORKSPACE`, `RBRR_DNS_SERVER`, `RBRR_RUNTIME_PREFIX`, `RBRR_SECRETS_DIR`, `RBRR_PUBLIC_DOCS_URL`.

## Axis A — Mint RBRD regime

Relocate the four depot-time settings out of RBRR (which today implies "live") into a new RBRD ("depot regime") whose name communicates lifecycle. Parallel structure to existing regimes:

- `Tools/rbk/rbrd_regime.sh` — enrollments
- `Tools/rbk/rbrd_cli.sh` — render/validate CLI partner
- `Tools/rbk/vov_veiled/RBSRD-RegimeDepot.adoc` — spec doc parallel to RBSRP / RBSRR
- Tabtargets `rbw-rdr` / `rbw-rdv` for render/validate
- Variable rename: `RBRR_CLOUD_PREFIX` → `RBRD_CLOUD_PREFIX`, etc.
- Lifecycle marshal (`rblm_cli.sh`) updated to emit RBRD blanks alongside RBRR

Why a new regime rather than folding into RBRP: payor identity (who pays, who authenticates) and depot shape (what gets levied) are different lifecycle categories. Conflating them muddles the operator's mental model. A regime named "depot" carries the immutability semantics in the name itself.

## Axis B — GCS manifest tripwire

At depot levy, write `gs://<depot-bucket>/depot-manifest.json` containing the four RBRD values in canonical order, a SHA256 hash over that canonical form, and the levy timestamp.

At kindle time on any command that sources RBRD, fetch the manifest, recompute the hash from the operator's loaded RBRD env, compare. On mismatch:

- Dump side-by-side diff of stored-vs-local values
- Fail with explanatory message naming which fields drifted and stating that the depot must be relevied (or the local change reverted) before any operation can proceed

Enforcement choke point: the kindle path that already sources the regime files. One fetch, one comparison, gates every downstream command in the session.

## Locked decisions

- **Burn the bridges — no migration.** RBRD applies to new depots only. Existing depots remain on the pre-RBRD code path until decommissioned. No dual-read, no compatibility shim, no `RBRR_` fallback for the four relocated names.
- **Conversion deferred.** Migrating an existing depot to RBRD is its own future heat, only built on demonstrated need. The plausible mechanism (unmake + relevy preserving payload) is mentioned here for closure, not scoped here.
- **Manifest substrate undecided.** At mount-time, choose between a GCS object in a non-build bucket and per-variable artifacts under `rbi_df` (the GAR depot-facts namespace, added in ₣BO; tagged artifacts persist indefinitely). The original GCS rationale held against project labels (charset/length), GAR repo description (clobber risk), and Secret Manager (IAM overhead) — but predated the `rbi_df` namespace existing as a depot-scoped OCI-artifact home.
- **Hash over canonical form.** SHA256 over sorted-key, whitespace-deterministic serialization. Hash gates the fast-path check; the stored manifest body provides the diff message on mismatch.
- **Kindle-time enforcement, not per-command.** One fetch per workbench session, regardless of how many commands run.
- **Both axes ship together.** The tripwire could in principle ship alone (snapshotting current RBRR names), but the rename is the user-clarity half — shipping only the tripwire keeps "RBRR means live" still false. Ship as one heat.

## What done looks like

- A new depot levied via `rbw-dL` writes its manifest atomically as part of levy.
- The four depot-time values live in `rbrd.env` / `rbrd_regime.sh`; references throughout `rbgp_Payor.sh`, `rbdc_DerivedConstants.sh`, and consumers updated.
- Any command touching a depot reads the manifest at kindle and fails loudly if the operator's local RBRD has drifted from the levied shape.
- RBSRD spec exists and explicitly names RBRD as immutable-after-levy.
- Handbook updates point users at RBRD when they hit the relevy-required error.
- The remaining eight RBRR settings stay in RBRR untouched; nothing about live-regime behavior changes.

## Open questions (slate-time, not now)

- Should `RBRP_PAYOR_PROJECT_ID` also be captured in the manifest? It's fixed at levy too, but lives in the payor-identity category rather than depot-shape.
- For non-payor commands (director, retriever) the relevy authority isn't theirs — what does the failure message say in those contexts?
- The conversion-from-old-depot heat: is its mechanism just "unmake + relevy" with manual artifact carryover, or does it warrant tooling? Defer until a real case forces the question.