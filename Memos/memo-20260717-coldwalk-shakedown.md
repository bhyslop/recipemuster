# Recipe Bottle Coldwalk Shakedown — 2026-07-17

Findings log for a first-time onboarding walkthrough of Recipe Bottle, done cold
(no outside knowledge beyond what the repo's own docs provide). Every time a doc
was unclear, a command failed, a step assumed knowledge not yet given, or a guess
was required, it's recorded here with the exact command and expected-vs-actual.

**Status: paused partway through Beat 1 (local setup), by design, to discuss
Finding 5 before continuing.** Git tree is clean; two small config commits have
been made locally (see "Current repo state" below). Nothing pushed anywhere.

## Executive summary

The walkthrough got through repo/station config and into the first hands-on
track (`ccyolo`, the fully-local "no cloud, no Depot" Kludge+Charge demo). It
surfaced one recurring *shape* of problem and one one-off documentation gap:

- **The recurring problem (Findings 3–5): regime over-enforcement.** Every
  `rbob_*` command (`kludge`, `charge`, `hail`, `rack`, ...) dispatches through
  one shared function, `zrbob_furnish` (`Tools/rbk/rbob_cli.sh`), which
  unconditionally validates the *entire* stack of cloud-identity regimes —
  Repo (RBRR), Depot (RBRD), Federation (RBRF), Workforce (RBRW) — before any
  command-specific logic runs, regardless of whether that command actually
  reads any of those values. The `ccyolo` onboarding track is documented,
  twice, as needing none of this ("no cloud account, no Manor or Depot — fully
  self-contained on the developer's workstation"), but in practice a bare
  clone cannot run even the first local `Kludge` command without first
  fabricating: a depot cloud-prefix/moniker, an OIDC issuer + client ID +
  device/token endpoints for an external identity provider, and a GCP
  organization ID + workforce pool ID. None of that is genuinely consumed on
  this code path (confirmed for RBRD via source trace; RBRF/RBRW not yet
  traced with the same rigor but structurally identical). This is the
  headline finding — see Finding 5 for the full scope and the open question of
  how to proceed.
- **The one-off gap (Findings 1–2): station-file bootstrap.** The very first
  setup step has an undocumented fix (`buw-SI.StationInit.sh`) for an
  undocumented raw crash, and that fix's own output doesn't pass its own
  validator without a manually-added field.

Three small, genuinely-inert local config edits were made and committed
locally to get this far (never touching a real GCP resource): a runtime name
prefix (`RBRR_RUNTIME_PREFIX`), a placeholder depot name
(`RBRD_CLOUD_PREFIX`/`RBRD_DEPOT_MONIKER`), and a created-but-empty secrets
directory. Federation (RBRF) and Workforce (RBRW) placeholders were *not*
applied — that's where we paused to discuss scope before fabricating a fake
OIDC provider and a fake GCP org ID.

## Beat 1: Local setup

### Finding 1 — `buw-SI.StationInit.sh` not mentioned by the onboarding handbook

`tt/rbw-Occ.OnboardingConfigureEnvironment.sh` (Step 4) said a validator
failure "names the field and tells you what to fill in." On a genuinely fresh
clone (no `station-files/` directory at all — one level above the repo root)
the actual result was a raw bash crash, not a guided error:

```
$ tt/buw-rsv.ValidateStationRegime.sh
/Users/.../Tools/buk/burs_cli.sh: line 61: /Users/.../../station-files/burs.env: No such file or directory
```

Expected: a message naming the missing field(s) and what to put there. Got: a
bash "No such file or directory", no mention of what command fixes it. I
discovered `tt/buw-SI.StationInit.sh` myself by listing `tt/` directory
contents — it is not referenced anywhere in the onboarding handbook text or in
README.md's Getting Started section. A newcomer following only the handbook's
literal instructions would be stuck here without independently listing `tt/`.

### Finding 2 — `buw-SI.StationInit.sh` generates an incomplete station file

Running `tt/buw-SI.StationInit.sh` created `../station-files/burs.env` with
only two of the three required variables (`BURS_USER`, `BURS_LOG_DIR`; missing
`BURS_TINCTURE`). Re-running the validator then failed with a *good*, specific
error: `BURS_TINCTURE: BURS_TINCTURE is not set (missing from .env?)`. The full
documented template — including `BURS_TINCTURE=a` and an explanation of its
purpose (a 1-3 char tag composed into per-station resource names for
concurrent-station disjointness) — exists only in `Tools/buk/bul_launcher.sh`,
a *different* code path (the one that fires when a tabtarget runs with no
station file at all). `StationInit` itself never shows that template. Fixed by
hand-adding `BURS_TINCTURE=a`, after which validation passed clean.

**Net for Beat 1 Step 4**: two real gaps — (a) the un-guided raw-crash path
when the whole file is missing, discoverable only via `buw-SI.StationInit.sh`,
itself undocumented in the onboarding flow; (b) that init script's output not
satisfying its own regime's validator without a manual follow-up field, whose
meaning is documented only in a different script's inline strings.

### Finding 3 — RBRR_RUNTIME_PREFIX / RBRR_SECRETS_DIR left blank/missing on fresh clone

Step 5 of the Occ handbook ran clean as promised: both
`rbw-rrv.ValidateRepoRegime.sh` and even `rbw-rrr.RenderRepoRegime.sh` (the
*renderer*, not just the validator) refused to run until two fields were
fixed:

- `RBRR_RUNTIME_PREFIX must not be empty` — fixed by editing
  `rbmm_moorings/rbrr.env`, setting it to `coldwlk-` per the in-file comment
  example (`prod-`, `bhyslop-`).
- `RBRR_SECRETS_DIR directory not found: ../station-files/secrets` — fixed by
  `mkdir -p ../station-files/secrets`.

This matches the handbook's promise (errors name the field and the fix is
inferable), but it's worth flagging that **the renderer itself is not
render-only** — `rbw-rrr.RenderRepoRegime.sh` hard-fails on an unset field
rather than rendering the regime with the blank shown, which is mildly
surprising for a command billed as inspection ("View the project config").
`RBRD` (depot regime) validation fails as expected on a bare fork
(`RBRD_CLOUD_PREFIX must not be empty`) at this step — exactly what the Occ
handbook predicted, and normally resolved by the Payor track in Beat 2, not
before. (It turned out to matter much sooner than that — see Finding 4.)

### Finding 4 — Kludge (documented as depot-free/local-only) hard-fails on empty RBRD

Followed `tt/rbw-Ofc.OnboardingFirstCrucible.sh` into the `ccyolo` track. Both
the README (`ccyolo` Nameplate description: "Kludge-only development target:
no cloud account, no Manor or Depot — fully self-contained on the developer's
workstation") and the Kludge glossary entry ("Kludging produces a local Docker
image ... without involving Cloud Build or the Depot") state plainly that
Kludge needs no Depot. In practice:

```
$ export HANDBOOK_NAMEPLATE=ccyolo
$ tt/rbw-cKS.KludgeSentry.sh ${HANDBOOK_NAMEPLATE}
ERROR: [rbob_kludge_sentry] RBRD_CLOUD_PREFIX: RBRD_CLOUD_PREFIX must not be empty
```

**Root cause, confirmed via code trace (background research agent)**:
incidental, not genuine. Every `rbob_*` command dispatches through one shared
function, `zrbob_furnish` (`Tools/rbk/rbob_cli.sh`), which unconditionally
sources and enforces the *entire* RBRD regime (`zrbrd_kindle` +
`zrbrd_enforce`) before any command-specific branching. The only per-command
exemption in that dispatcher skips *nameplate* (RBRN) enforcement for kludge,
not depot (RBRD) enforcement. On the actual `rbob_kludge_sentry` path, an
RBRD-derived value (`RBDC_GAR_REPOSITORY`) is computed but only *read* inside a
vessel-anchor branch that neither ccyolo vessel
(`rbev-sentry-deb-tether`, `rbev-bottle-ccyolo`) triggers — dead weight, paid
by every `rbob_*` command regardless of need. **Charge has the identical
exposure** — same shared dispatcher, same unconditional enforcement.

Confirmed `RBRD_CLOUD_PREFIX`/`RBRD_DEPOT_MONIKER` are inert until a real
Payor Levy runs (pure string concatenation in `rbdc_derived.sh`/
`rbgd_depot.sh`, no network or `gcloud` call tied to them) — so a placeholder
is genuinely safe. Asked the operator rather than picking a project name
unilaterally (those strings become the literal future GCP project ID/GAR
repo/pool name at Levy); they chose placeholders `RBRD_CLOUD_PREFIX=coldwlk-`
/ `RBRD_DEPOT_MONIKER=shakedown`, to be revisited before any real Beat 2 Levy.

**Could a human reader have reached "just use an obvious placeholder" on their
own, from the docs alone?** Assessed on request — no, not confidently. Three
gaps stack up:

1. The `ccyolo`/Kludge track's own handbook text and the README both assert
   *no* cloud/Depot involvement at all for this track. Nothing in the
   onboarding flow anticipates an RBRD failure here, so the error reads as a
   contradiction of what was just promised, not an expected step with a known
   next move.
2. The proximate tool error is fine in isolation — it names the field,
   matching the general "errors name the field" pattern Occ Step 4 primes the
   reader to expect. But Occ Step 5, minutes earlier in the same walkthrough,
   explicitly primed the *opposite* expectation for this exact regime: "RBRD
   fields are blank... the Payor must establish a Manor and Levy a Depot to
   populate them." A careful reader carrying that forward would reasonably
   conclude RBRD is off-limits pre-Payor, not "safe to placeholder."
3. `rbrd.env`'s own header reads "Values here are written by the operator
   before depot levy" — true, but phrased as if the operator is committing to
   *the* real depot identity, with no signal these fields are inert until Levy
   or that throwaway strings are an accepted stopgap. Establishing that safety
   required tracing three source files — well past what the docs equip a
   first-time reader to do or reassure them about.

Net: a human hitting this cold would face a real fork with no textual
guardrail either way — stop (docs said no Depot needed, this looks like a bug)
or guess-and-edit a file described as depot-defining, with no stated
reversibility. Recommend the docs either exempt Kludge/Charge from RBRD
enforcement (the code fix, since the dependency is confirmed incidental) or,
short of that, add one line to the `ccyolo` track / RBRD header stating these
two fields are inert placeholders pre-Levy and safe to fill with any
conformant string until a real Depot is provisioned.

### Finding 5 — the same over-enforcement extends to RBRF (federation) and RBRW (workforce) — PAUSED HERE

Fixing RBRD only got one step further. The very next `Kludge` invocation hit a
new blank-field error, this time for the Federation regime:

```
$ tt/rbw-cKS.KludgeSentry.sh ${HANDBOOK_NAMEPLATE}
ERROR: [rbob_kludge_sentry] RBRF_PROVIDER_ID: RBRF_PROVIDER_ID must not be empty
```

Reading `zrbob_furnish` directly (`Tools/rbk/rbob_cli.sh`) shows it
unconditionally kindles *and* enforces four regimes for every `rbob_*`
command, not just RBRD:

```
zrbrr_kindle / zrbrd_kindle / zrbrf_kindle / zrbrw_kindle
zrbrr_enforce / zrbrd_enforce / zrbrf_enforce / zrbrw_enforce
```

RBRP (Payor identity: billing account, OAuth client) is *not* in this chain —
only RBRR/RBRD/RBRF/RBRW. But RBRF and RBRW together are **8 more blank
fields**, and they're qualitatively different from the RBRD depot-naming
placeholder:

- `RBRF_PROVIDER_ID`, `RBRF_IDP_ISSUER`, `RBRF_IDP_CLIENT_ID`,
  `RBRF_IDP_DEVICE_ENDPOINT`, `RBRF_IDP_TOKEN_ENDPOINT` — these describe a
  real (or synthetic) external OIDC identity provider. The regime's own header
  calls Affiance (the operation that would normally populate this) "a Payor
  ceremony with founding gravity."
- `RBRW_ORG_ID`, `RBRW_WORKFORCE_POOL_ID` — a GCP organization numeric ID and
  workforce pool ID.

Validation here is only length-bounded string checks (`buv_string_enroll`
min/max, e.g. `RBRF_IDP_ISSUER` 8–512 chars) — no live network/format check
was hit yet — so fabricated strings would likely pass syntactically, same as
RBRD. By the same logic already established for RBRD (validation-only,
nothing on the Kludge/Charge path actually authenticates against an IdP or
touches a workforce pool), this is almost certainly the same *incidental*
over-enforcement pattern, just not yet confirmed with the same source-level
rigor as RBRD was.

**This is where the session paused.** I asked whether to keep placeholdering
through RBRF/RBRW (fabricating a fake issuer URL, fake client ID, fake
endpoints, fake GCP org ID) to force a working local Kludge/Charge, or to stop
and treat the full cascade — RBRR → RBRD → RBRF → RBRW, four regimes deep, to
run one *local* Docker build — as the headline finding in its own right. The
operator asked to discuss rather than pick blind, and then asked to write up
this memo and wrap the session. No RBRF/RBRW edits have been made; the repo
tree is clean past the two commits below.

## Current repo state

Two commits made locally on branch `coldwalk` (nothing pushed, nothing else
touched):

```
ecf82a8 Set placeholder RBRD depot identity to unblock local Kludge/Charge
6e07abe Set RBRR_RUNTIME_PREFIX for local onboarding walkthrough
```

Plus, outside the repo (as the docs intend — `BURS`/secrets are explicitly
not-in-git): `../station-files/burs.env` (with `BURS_TINCTURE=a` added) and an
empty `../station-files/secrets/` directory.

`rbmm_moorings/rbmf_foedera/rbef_entrada/rbrf.env` and `rbmm_moorings/rbrw.env`
are untouched — still blank, still the blocker.

## Open question for next session: how to unblock Finding 5

Options, not yet decided:

1. **Fabricate RBRF/RBRW placeholders too**, same spirit as the RBRD fix —
   fastest path to an actual charged local Crucible, but a noticeably bigger
   fiction (fake IdP + fake GCP org) for a track billed as needing neither.
2. **Patch the tool** instead of the config — since the dependency looks
   incidental (a shared dispatcher over-validating), scope `zrbob_furnish` so
   Kludge/Charge only enforce the regimes their vessels actually read. This is
   the "real" fix implied by Finding 4/5's own root-cause analysis, but it's a
   code change to the project under test, not just a config workaround — a
   bigger decision than a first-time onboarding user should make unilaterally.
3. **Stop the coldwalk here** and report Findings 1–5 as the deliverable —
   the local-only promise of `ccyolo` does not hold as shipped, and that's
   arguably the single most useful thing this shakedown could surface.
4. Something else the operator has in mind (e.g., check whether a *different*
   reference Nameplate has looser regime requirements than `ccyolo` and route
   the "first local build" demonstration through that one instead).

## Remaining beats (not started)

- **Beat 2 — Cloud setup** (Payor/Manor/Depot, `tt/rbw-Op.OnboardingPayor.sh`):
  previewed only (read-only), not executed. Requires operator-owned
  account/billing/IdP actions per the task's hard boundary.
- **Beat 3 — Adversarial suite** (`tadmor`, Theurge/Ifrit): not started.
- **Beat 4 — Airgap build** (`moriah`): not started.
