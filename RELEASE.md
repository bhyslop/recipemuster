# Release Procedure

The release qualification ceremony for the project maintainer. The qualification ladder plus the local cut and private preview (steps 1–6) run roughly an hour, with cloud cost on the order of two GCP projects per run; the public staging reveal (step 7) is the single irreversible disclosure; the greenfield walk (step 8) that gates promotion is a separate, multi-hour, operator-driven ceremony; and promotion with close-out (step 9) moves public `main`.
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

- **The base — remote `ENGROSSMENT_UPSTREAM`, read-only.** The real, durable public repository (`git@github.com:scaleinv/recipebottle.git`); expede clones it and cuts the candidate one commit atop its live `main`. Configure it once, and **neuter its push side** — the base is read-only, and a push-capable remote to the public target in the maintainer repo is the one catastrophe this ceremony forbids (a stray `git push ENGROSSMENT_UPSTREAM` would put your private tree on public `main`):
  ```
  git remote add ENGROSSMENT_UPSTREAM git@github.com:scaleinv/recipebottle.git
  git remote set-url --push ENGROSSMENT_UPSTREAM DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY
  ```
  Expede refuses to cut unless the push side is neutered to exactly that sentinel. The ceremony never creates or destroys the base; expede only ever *reads* it (fetch), then severs the clone's origin, so nothing it does can push there.
- **The quarantine — `git@github.com:scaleinv/recipebottle-staging.git`.** An ephemeral, **private** repository, reached only by explicit URL (never a configured remote of the candidate — the candidate holds zero remotes). Create it **empty and private** on GitHub before the cut; **delete it** once the candidate is dispositioned. Both acts are your own hands — no tooling creates or destroys it, and no delete-scoped token is ever minted.

Exactly **one** command in this document touches public `main` — the promotion (step 9), on the far side of the walk. Nowhere else is public `main` a default-shaped outcome: expede holds no remote, the base remote's push side is dead, and the candidate's sole branch is `POSTULANT_LOCAL`, so the preview and the staging reveal both push `POSTULANT_LOCAL:POSTULANT_LOCAL` and only promotion spells `POSTULANT_LOCAL:main`, by hand.

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

A git commit is **content-addressed**: the object you would stage publicly and the object you push here are the *same commit, bit for bit*. So this is a byte-faithful preview of the reveal — everything you inspect is exactly what step 7 will disclose — and it is **reversible**: delete the private quarantine and nothing escaped. Inspect it here, in private, before the irreversible public act.

Push by **explicit URL and explicit refspec** — the candidate has no remote to lean on, and the refspec is a branch, never `main`:

```
git -C «candidate» push git@github.com:scaleinv/recipebottle-staging.git POSTULANT_LOCAL:POSTULANT_LOCAL
```

- Inspect the `POSTULANT_LOCAL` branch on GitHub: the file tree, the rendered README, the single commit
- Success: the quarantine carries exactly the candidate, and it reads as a clean public face
- Failure: resolve the remote-side reason and retry — the local candidate from step 5 remains valid

**Do not proceed until you have inspected the preview and are ready for the irreversible public disclosure.**

## 7. Public staging reveal — the irreversible disclosure

> ⚠️ **This is the point of no return.** Pushing to the public repository discloses the bytes — a public object store cannot be un-disclosed. Everything before this was private and reversible; nothing after this un-happens. It is your own hand: explicit URL, explicit refspec, and it does **not** touch `main`.

Push the candidate to the **public** repository as an **unmerged branch** — the staging the greenfield walk will clone as a stranger. `main` is untouched:

```
git -C «candidate» push git@github.com:scaleinv/recipebottle.git POSTULANT_LOCAL:POSTULANT_LOCAL
```

- The refspec targets the `POSTULANT_LOCAL` branch, never `main`: staging discloses the bytes but changes nothing a visitor lands on
- Verify `main` is untouched — its SHA identical before and after, the only new ref being `POSTULANT_LOCAL` (the promotion step installs the mechanical before/after assert)
- Success: the candidate stands on the public repository as a public, anonymously-cloneable, unmerged branch
- Failure: resolve and retry — but the bytes are already disclosed; any re-cut from here re-opens the full gate

## 8. The greenfield manor walk — gates promotion

The walk clones the **public staging branch** (step 7) as a total stranger and founds the whole system from zero through the shipped docs alone, culminating in a green gauntlet on a fresh payor substrate. It gates **discoverability** (promotion), never disclosure — step 7 already disclosed the bytes publicly; the walk decides whether they become the default face.

Its own multi-hour, operator-driven ceremony — run it fresh, not tired. Record every divergence between the shipped docs and reality as a finding: a doc-only divergence census defaults to fast-follow disposition, while a delivered-bytes-bearing re-cut re-opens the full gate (fresh walk and gauntlet). Two finding classes are pre-waived: dead `main`-blob links (they go live only at promotion) and the stranger's bootstrap (cloning the staging branch rather than default `main`). Only once the walk is green with its census dispositioned does promotion proceed.

## 9. Promotion to public main — discoverability

The single command in this whole document that touches public `main`, on the far side of the green walk. It moves no bytes a stranger has not already seen at staging: it fast-forwards `main` to the walked staging branch, making it the default face and taking the README links live.

**Before `main` moves — promotion-day housekeeping:**

- Confirm the repository's **Pages source**: the README render from `main` root is the intended Pages face. Promotion deletes the public repo's untracked `index.html` and `.nojekyll` from `main`; no landing page ships and none moves to a side branch
- Close the public repository's **stale open PR**, if one stands

**The promotion** — fast-forward-only, from the walked public staging branch to `main`, asserting **tree-hash equality** (`main`'s tree after the move equals the walked candidate tip — the byte claim is checked, not assumed). A refused fast-forward means `main` moved since the cut: **stop**, never `--force`, re-cut atop the moved base. The one `POSTULANT_LOCAL:main` refspec lives here and nowhere else.

**Then — prove the promoted face and close out:**

- Liveness: the public README serves at its URL and every anchor resolves against the live page
- Production spot-check (thin — the walk already proved the bytes): repo front page, the github.io render, a fresh default-branch clone taken a few minutes into the onboarding entry
- Delete the public **staging branch** (ceremony hygiene), then the **private quarantine** repository and the **candidate directory** — the candidate directory may stand until here, its earliest safe disposal being once step 7 is tip-verified
- Record the release

> **Rationale, in brief** — *this migrates to a release spec sheaf when one exists; held here in the interim, not in a memo.* The candidate is built by **addition** in a clone of the real public base, so no private object ever enters the graph a push walks: construction is the prevention; the private quarantine is only containment. The base is reached **read-only** — expede reads the `ENGROSSMENT_UPSTREAM` fetch URL, its push side is neutered, it severs the clone's origin, and the finished candidate holds zero remotes — so every public act is human-hands-only by **structural incapacity**, never by discipline. The **staging push (step 7)** is the single irreversible disclosure; the private preview (6) is reversible, and the promotion (9) is discoverability, not new disclosure. The preview is trustworthy because a commit is content-addressed: the object you inspect privately is, bit for bit, the object you stage and then promote.
