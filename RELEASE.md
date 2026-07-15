# Release Procedure

The release qualification ceremony for the project maintainer. The qualification ladder and the cut (steps 1–6) run roughly an hour, with cloud cost on the order of two GCP projects per run; the greenfield walk that gates the reveal (step 7) is a separate, multi-hour, operator-driven ceremony, and the reveal itself (step 8) is the one irreversible public act.
The ceremony exists to catch silent first-build assumptions that the routine `tt/rbw-tq.QualifyFast.sh` and `tt/rbw-tr.QualifyRelease.sh` tiers tolerate by design.

[Payor](README.md#Payor) OAuth is the only durable prerequisite credential — the system's sole standing secret.
No service-account keyfiles are restored from backup: the depot's [Mantle](README.md#Mantle) service accounts ([Governor](README.md#Governor), [Director](README.md#Director), [Retriever](README.md#Retriever)) are seated by the qualification's own founding verbs and reached only through short-lived federated tokens — [Avow](README.md#Avow) opens a [Sitting](README.md#Sitting), then [Don](README.md#Don) mints a mantle token per call.
The entire access chain must be reproducible from the Payor OAuth alone, against a clean local state.

**Failure mode contract.** Mid-qualification failure means start over from step 2, not patch-and-continue.
Patching forward is exactly the bug class this ceremony exists to catch — accumulated state hides first-build assumptions.

## 1. Confirm Payor health

- Probes the [Payor](README.md#Payor) OAuth credential — exchanges the stored refresh token for a live access token against Google Cloud, so subsequent [Payor](README.md#Payor)-authority operations do not fail mid-qualification on an expired token
- Success: the probe exits clean — proceed
- Failure: the stored refresh token is expired or absent. Re-[Install](README.md#Install) it with `tt/rbw-gPI.PayorInstall.sh «RBRR_SECRETS_DIR»/client_secrets/client_secret_*.json` (the JSON saved at [Establish](README.md#Establish); pass the exact path if several exist), or re-[Establish](README.md#Establish) the [Manor](README.md#Manor) if the OAuth client itself is gone; then re-run this step

```
tt/rbw-ap.CheckPayorCredential.sh
```

## 2. Open a fresh Sitting

- [Novate](README.md#Novate) the [Sitting](README.md#Sitting) — extinguish any standing one and open a fresh full-window sitting via the device-flow browser sign-in
- The qualification's build verbs are runway-gated: the long captures demand two hours of remaining [Sitting](README.md#Sitting) window, so a day-old standing sitting passes the early rungs and then dies mid-ladder at the floor — a fresh window removes the class by construction
- Success: the device-flow sign-in completes and the fresh sitting is reported (spot-check anytime with `tt/rbw-as.EspySitting.sh`)
- Failure: the avowal failed — re-run and complete the browser sign-in; if the trust itself is unhealthy, descry the active foedus with `tt/rbw-jd.FoedusDescry.sh`

```
tt/rbw-aN.NovateSitting.sh
```

## 3. Marshal zero

The marshal-zero operation returns local state to the blank onboarding-start template:

- Blanks the site-specific [Repo Regime](README.md#RBRR) field (`RBRR_RUNTIME_PREFIX`) and the [Depot](README.md#Depot)-identity fields (`RBRD_CLOUD_PREFIX`, `RBRD_DEPOT_MONIKER`)
- Blanks [Hallmark](README.md#Hallmark) pins (`RBRN_SENTRY_HALLMARK`, `RBRN_BOTTLE_HALLMARK`) in each [Nameplate](README.md#Nameplate) regime file ([RBRN](README.md#RBRN))
- Blanks depot-scoped fields (`RBRV_RELIQUARY`, `RBRV_IMAGE_*_ANCHOR`) in each [Vessel](README.md#Vessel) regime file ([RBRV](README.md#RBRV))
- **Preserves** the [Payor](README.md#Payor) OAuth credential — no credential files are deleted (there are none to delete: the federation era holds no service-account keyfiles)

Marshal zero **requires the intended tree's basename as its argument** — it blanks the regime of whatever tree it runs in, so that tree must be named, not assumed; naming the wrong one is refused against git's own `--show-toplevel`, before any prompt and regardless of `BURE_CONFIRM`. It then gates on a clean, pushed tree and **auto-commits** the blanked state as a single "Marshal Zero" commit. There is no manual commit step — the pristine baseline lands as one commit by construction.

- Success: working tree clean and `HEAD` is the marshal-zero commit
- Failure: review the marshal output for files the operation would not zero, resolve, and retry

```
tt/rbw-MZ.MarshalZeroes.sh rbm_alpha_recipemuster
```

## 4. Run the gauntlet

The release gate — the full release-qualification ladder, run from the marshal-zero baseline.

**Entry contract** — the gauntlet's first fixture (the marshal-zero attestation gate) refuses to run unless ALL of the following hold:

- working tree clean
- the site-specific [Repo Regime](README.md#RBRR) field (`RBRR_RUNTIME_PREFIX`) blank, and the [Depot](README.md#Depot)-identity fields (`RBRD_CLOUD_PREFIX`, `RBRD_DEPOT_MONIKER`) blank
- [Hallmark](README.md#Hallmark) pins blank in every [Nameplate](README.md#Nameplate) regime ([RBRN](README.md#RBRN))
- depot-scoped fields (`RBRV_RELIQUARY`, `RBRV_IMAGE_*_ANCHOR`) blank in every [Vessel](README.md#Vessel) regime file ([RBRV](README.md#RBRV))

The gate reads the actual blank fields — not a commit-message signature — and is enforced by the test itself, not by ceremony or operator discipline; that property is what lets this tier catch silent-first-build assumptions by construction.

**Cost** — ~1 hour wall-clock plus two GCP projects per run:

- one ephemeral leasehold [Depot](README.md#Depot) [Levied](README.md#Levy) and torn down by the `depot-lifecycle` fixture (the full create→destroy proof)
- one durable freehold [Depot](README.md#Depot) [Levied](README.md#Levy) by the `freehold-establish` fixture, inherited by every downstream fixture
- GCP project deletion is asynchronous (30-day soft delete) — each run leaves pending-delete projects in the project list; this is accepted cost

**Sequence** (fail-fast across fixtures):

- enrollment-validation, then the marshal-zero attestation gate (the entry contract above)
- `depot-lifecycle` — mint and tear down the ephemeral leasehold [Depot](README.md#Depot)
- `freehold-establish` — stand up the durable freehold [Depot](README.md#Depot): the [Levy](README.md#Levy) seats the three [Mantle](README.md#Mantle) service accounts, the [Payor](README.md#Payor) [Girds](README.md#Gird) the freehold subject as first [Governor](README.md#Governor), who then [Brevets](README.md#Brevet) the subject onto the [Director](README.md#Director) and [Retriever](README.md#Retriever) mantles and [Dons](README.md#Don) each — the federation replacement for the retired keyfile-deploy sequence
- credential-readiness — verify the [Sitting](README.md#Sitting) and the [Director](README.md#Director)/[Retriever](README.md#Retriever) [Dons](README.md#Don) before any build spends
- onboarding-sequence, the regime / hygiene / foundry fixtures, cupel, conformance, the chaining-fact band, hallmark-lifecycle
- the [Crucible](README.md#Crucible) suite (tadmor, moriah, srjcl, pluml)

Outcome:

- Success: green tally and clean exit
- Failure: return to step 2 — never patch-and-continue. Also unmake the failed run's stranded freehold [Depot](README.md#Depot) once its diagnosis is extracted (`tt/rbw-dU.PayorUnmakesDepot.sh «project-id»`): each strand holds a billing-account link, the account's link quota is small, and the [Levy](README.md#Levy) refuses at the quota wall — unmake releases the link while the strand's terrier records survive

```
tt/rbw-ts.TestSuite.gauntlet.sh
```

## 5. Cut and prove the candidate

**Prerequisites — two repositories, two very different lifetimes.**

- **The base — remote `ENGROSSMENT_UPSTREAM` → `git@github.com:scaleinv/recipebottle.git`.** The real, durable public repository. Expede clones it and cuts the candidate one commit atop its live `main`. Configure it once (`git remote add ENGROSSMENT_UPSTREAM git@github.com:scaleinv/recipebottle.git`); the ceremony never creates or destroys it. Expede only ever *reads* it — it clones, then severs the clone's origin, so nothing expede does can push here.
- **The quarantine — `git@github.com:scaleinv/recipebottle-staging.git`.** An ephemeral, **private** repository, reached only by explicit URL (never a configured remote of the candidate — the candidate holds zero remotes). Create it **empty and private** on GitHub before the cut; **delete it** once the candidate is dispositioned. Both acts are your own hands — no tooling creates or destroys it, and no delete-scoped token is ever minted.

The refspec that touches the real public `main` appears **exactly once** in this document — in step 8, the reveal, on the far side of the walk. Nowhere else does a command have public `main` as a default-shaped outcome: expede holds no remote, and the candidate's sole branch is `POSTULANT_LOCAL`, so every push must spell `POSTULANT_LOCAL:main` by hand.

Run the cut:

```
/rbk-expede
```

- Claude Code slash command, not a tabtarget — the delivery ceremony is interactive by design, and its last acts are the maintainer's own eyes
- Expede is a **pure local constructor**: it clones `ENGROSSMENT_UPSTREAM`, builds the candidate by **addition** (materializes only the paths the perambulation ships, transposes the consumer `CLAUDE.md`, sterilizes, commits once), severs the clone's origin, and **pushes nothing**. The finished candidate is one commit atop the public base, on branch `POSTULANT_LOCAL`, holding **zero remotes**. Nothing is stripped, because nothing withheld is ever put in
- The driver proves it before your eyes: the delivered wiring, release hygiene, the erasure of site identity, a consumer-seat reveille from a feigned station, and the base inventory of already-disclosed withheld history for you to acknowledge
- Success: every assay green, the base inventory acknowledged, and you have read the candidate's file list
- Failure: **re-cut, never patch forward** — abandon the candidate directory, repair on `main`, run the ceremony again from step 2. Patching a candidate is the same bug class this whole procedure exists to catch

## 6. Preview in the private quarantine (reversible)

A git commit is **content-addressed**: the object you would publish and the object you push to the quarantine are the *same commit, bit for bit*. So the quarantine push is a byte-faithful preview of the reveal — everything you inspect here is exactly what step 8 would publish — and it is **reversible**: delete the private quarantine and nothing escaped.

Push by **explicit URL and explicit refspec** — the candidate has no remote to lean on:

```
git -C «candidate» push git@github.com:scaleinv/recipebottle-staging.git POSTULANT_LOCAL:main
```

- Inspect the pushed branch on GitHub: the file tree, the rendered README, the single commit
- Success: the quarantine carries exactly the candidate, and it reads as a clean public face
- Failure: resolve the remote-side reason and retry — the local candidate from step 5 remains valid

**Do not proceed to the reveal until the greenfield walk (step 7) has closed green.**

## 7. The greenfield manor walk — the gate

The reveal is gated on a full first-contact walk: a stranger clones the quarantined candidate and founds the whole system from zero through the shipped docs alone, culminating in a green gauntlet on a fresh payor substrate. The walk gates **discoverability** (the reveal), never disclosure — step 6 already disclosed the bytes privately.

This is its own multi-hour, operator-driven ceremony — run it fresh, not tired. Record every divergence between the shipped docs and reality as a finding: a doc-only divergence census defaults to fast-follow disposition, while a delivered-bytes-bearing re-cut re-opens the full gate (fresh walk and gauntlet). Only once the walk is green with its census dispositioned does the reveal proceed.

## 8. Reveal: promote the candidate to public main (IRREVERSIBLE)

> ⚠️ **This is the irreversible public disclosure — the only command in this document that touches public `main`. It is your own hand: there is no tooling and no minted token. You type it.**

**Before `main` moves — promotion-day housekeeping:**

- Confirm the repository's **Pages source** config: the README render from `main` root is the intended Pages face. The promotion deletes the public repo's untracked `index.html` and `.nojekyll` from `main`; no landing page ships in the delivery and none moves to a side branch
- Close the public repository's **stale open PR**, if one stands

**The reveal** — explicit URL, explicit refspec, on the far side of the green walk:

```
git -C «candidate» push git@github.com:scaleinv/recipebottle.git POSTULANT_LOCAL:main
```

- The push is fast-forward-only by default — a **refused** push means `main` moved since the cut; **stop**, never `--force`, and re-cut atop the moved base
- Success: the candidate stands on public `main`, one commit atop the base

**Afterward — disposition, in order:**

1. Delete the **private quarantine** repository — its whole value is that it does not outlive the cut
2. Delete the **candidate directory**
3. Record the release

> **Rationale, in brief** — *this deep rationale migrates to a release spec sheaf when one exists; it is held here in the interim, not in a memo.* The candidate is built by **addition** in a clone of the real public base, so no private object ever enters the graph the push walks: construction is the prevention, and the quarantine is only containment. The base and the reveal target are the **same** public repository reached two ways — expede *reads* it (as the `ENGROSSMENT_UPSTREAM` remote) and can never *write* it, because it severs the clone's origin and holds no push endpoint. The reveal is therefore human-hands-only not by discipline but by **structural incapacity**. The preview is trustworthy because a commit is content-addressed: the object you inspect in the quarantine is, bit for bit, the object step 8 publishes.
