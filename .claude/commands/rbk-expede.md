---
description: Cut and prove the upstream release candidate (RELEASE.md step 5)
---

You are cutting the delivery candidate and proving it fit to publish. This is step 5 of `RELEASE.md`. **This driver performs no push** — it cuts, proves, and shows you the candidate. The reversible quarantine preview and the irreversible public reveal are your own hands, numbered steps 6 and 8 of `RELEASE.md`.

**Read this first — it is why the ceremony is short.**

The candidate is built by **addition**, not by stripping. `tt/rbw-ME.MarshalExpedes.sh` (*expede*) clones the **real public base** (the `ENGROSSMENT_UPSTREAM` remote), cuts the candidate one commit atop its live `main`, materializes every path the **perambulation** ships — from committed bytes, via `git archive` — sterilizes them, transposes the consumer `CLAUDE.md`, and lays down exactly one commit on branch `POSTULANT_LOCAL`. Then it **severs the clone's origin**: the finished candidate holds **zero remotes**, so expede cannot push it anywhere. No private object ever enters the candidate's object graph, because nothing withheld is ever put in.

Expede already gates on: a clean committed tree, a **total** perambulation (any unjudged tracked path or any dead row refuses the cut), `ENGROSSMENT_UPSTREAM` configured, the consumer `CLAUDE.md` template present, a base **inventory** of already-disclosed withheld history (loud, not fatal), a byte-assert that the candidate's `CLAUDE.md` equals the consumer template, `rev-list --count == 1` atop the base, a **fatal** sweep of the candidate's `base..HEAD` delta, and a final assert that the clone holds zero remotes. Do not re-implement any of these. Run the verb and read its verdict.

**The candidate has no station and no secrets directory, and that is load-bearing.** It is why the operator's freehold subject cannot reach the candidate's generated Rust. Never copy station files or secrets into it. Step 5 below writes a *fresh, identity-free* three-line station so the candidate can run its own tests; that is the consumer's first onboarding act, not a leak.

**A finding means re-cut, never patch forward.** A red assay does not get fixed on the candidate. Abandon the candidate directory, repair on `main`, run this ceremony again from the top. Nothing is pushed until you push it by hand, and this driver never does.

**Run every command below by absolute path from the working repository. Do not `cd`.** `tt/z-launcher.sh` normalizes its working directory to the repo root of *the tabtarget it is invoked as* — so `{cand}/tt/rbw-tf.FixtureRun.sh damnatio` assays the candidate even though your session never left the working repo.

This is an interactive ceremony: run one step, show its output, wait for the operator, then proceed. Stop on any error.

---

## 1. Working repository is clean and pushed

```
git status --porcelain
git rev-list --count @{u}..HEAD
```

Both must be empty / `0`. If not, stop and ask the operator to resolve.

Expect one benign exception: opening an officium writes a bookkeeping commit, so a session that ran `jjx_open` starts one commit ahead of the remote. Push it and re-read the gate. Anything else unpushed is the operator's to resolve.

## 2. Base and quarantine gates

**The base** — expede cuts atop the real public repository, so its remote must be configured **and its push side neutered** (the base is read-only; a push-capable remote to the public target is the one catastrophe the ceremony forbids — a stray `git push ENGROSSMENT_UPSTREAM` would put the private tree on public `main`). Confirm both:

```
git remote get-url ENGROSSMENT_UPSTREAM
git remote get-url --push ENGROSSMENT_UPSTREAM
```

The fetch URL must be the public repo; the push URL must be exactly `DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY`. If unconfigured or the push side is live:

```
git remote add ENGROSSMENT_UPSTREAM git@github.com:scaleinv/recipebottle.git
git remote set-url --push ENGROSSMENT_UPSTREAM DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY
```

Expede refuses the cut unless the push side is the sentinel.

**The quarantine** — an **ephemeral, private** repository, created empty by the operator before a cut and deleted by hand after the candidate is dispositioned. It is never created or destroyed by tooling: no delete-scoped token exists, and none may ever be minted. It is reached only by **explicit URL** — the candidate carries no remote for it. Check it, credential-free, with `<owner>/<repo>` = `scaleinv/recipebottle-staging`:

- **Unseeable by strangers** — an anonymous API read must 404:
  ```
  curl -s -o /dev/null -w '%{http_code}\n' https://api.github.com/repos/scaleinv/recipebottle-staging
  ```
  Anything but `404` means the quarantine repo is public (or the name is wrong). **Stop.**

- **Fresh** — the quarantine must be either empty (no refs at all) or carrying exactly `refs/heads/POSTULANT_LOCAL` (a preview from this same cut):
  ```
  git ls-remote --heads git@github.com:scaleinv/recipebottle-staging.git
  ```
  Anything else — a second branch, a stale candidate branch, or a `main` — means a prior cut was not dispositioned. **Stop**, and ask the operator to dispose of it.

## 3. Pre-cut assays, in the working repository

**`main` must be green before it is worth cutting.** Both 2026-07-14 candidates died on flaws that were already sitting in the working repository — one of them a live `cupel` red on `main` — and both cost a full cut to discover, because the ceremony asked the candidate a question it had never asked the source. Run the suite here first; a red here is cheaper than a red there by an entire cut.

```
tt/rbw-ts.TestSuite.reveille.sh
```

Then the two assays that read the **veiled trees**, and so are meaningful only here — in the candidate they are red by construction, which is why they cannot be deferred to it:

```
tt/rbw-tf.FixtureRun.sh loupe
tt/rbw-tf.FixtureRun.sh perambulation
```

Note what `cupel` (inside reveille) can and cannot see from here. It resolves every command against the functions defined **anywhere in the tree** — including the withheld modules. So a shipped file calling a withheld module's function is *green here and red in the candidate*: the census is present to answer for it here, and absent there. Reveille on `main` is therefore necessary and not sufficient — the candidate's own `cupel` in step 6 is the only reader that sees the delivered tree as a consumer holds it.

## 4. Cut the candidate

The target directory must not exist. Convention: `rbm_candidate_{YYYYMMDD}_{try}`, a sibling of the working repo.

```
tt/rbw-ME.MarshalExpedes.sh /absolute/path/to/rbm_candidate_20260714_1
```

Expede carries no typed confirmation: this ceremony drives it headlessly, so a prompt could only ever teach a bypass. Its refusals are structural — clean tree, absolute path, target must not exist, base remote configured, consumer template present, candidate-delta sweep, transposition byte-assert, zero-remotes assert — and a refusal is not a prompt: it cannot be answered, only satisfied.

The candidate lands at `{target}/candidate`, on branch `POSTULANT_LOCAL`. Read the verb's verdict: one commit atop the base, the base inventory of already-disclosed history **acknowledged**, the candidate delta clean, the `CLAUDE.md` transposition byte-asserted, and **zero remotes**.

## 5. Give the candidate a station

The consumer's first onboarding act, reproduced exactly: an identity-free station, and the empty secrets directory the regime requires. Both, or the candidate cannot validate — `RBRR_SECRETS_DIR` names a directory, and `rbw-rrv` refuses on its absence, which reads as a candidate flaw and is not one.

Write `{target}/station-files/burs.env`:

```
BURS_USER=candidate
BURS_TINCTURE=cnd
BURS_LOG_DIR=<target>/logs-buk
```

And create the directory it will look for — empty, and it stays empty:

```
mkdir -p {target}/station-files/secrets
```

## 6. Assay the candidate

Run the candidate's **own** tabtargets, by absolute path, on its sterile `POSTULANT_LOCAL` branch.

```
{cand}/tt/rbw-tq.QualifyFast.sh
{cand}/tt/rbw-tf.FixtureRun.sh cupel
{cand}/tt/rbw-tf.FixtureRun.sh pyx
{cand}/tt/rbw-tf.FixtureRun.sh damnatio
```

- **`rbw-tq`** — the delivered wiring is intact: colophons, tabtargets, nameplates.
- **`cupel`** — BCG command discipline **as the candidate resolves it**. This is the reading no assay in the working repository can perform: a shipped file that calls a withheld module's function resolves cleanly on `main` and names an unknown command here. It needs no station, so it runs on the sterile tree, before feigning — the 2026-07-14 try-2 candidate died on exactly this, and only discovered it a whole probe later.
- **`pyx`** — release hygiene of the tree being published. It reads the candidate's `CLAUDE.md` as the transposed consumer template, not the maintainer's.
- **`damnatio`** — the proof of erasure: no site identity survives anywhere the proscription reaches. This is the assay the whole design exists to pass. It must run **on `POSTULANT_LOCAL`, before any feigning** — damnatio reddens on feigned fields by construction, which is exactly what keeps a probe branch from ever being mistaken for a candidate.

Any red: abandon the candidate, repair on `main`, re-cut.

## 7. Consumer-seat probe, then drop the probe branch

A sterile candidate is correctly *unvalidatable* — it has no station identity, so it cannot run its own tests as-is. Feigning invents a visibly false one, on a **throwaway branch**, so the candidate can run the consumer's own reveille from the consumer's seat.

```
git -C {cand} checkout -b probe
```

Marshal tabtargets are withheld from delivery, so the candidate has no `rbw-MF` launcher. Create it as a copy of any other tabtarget in the candidate's `tt/` (the trampolines are byte-generic), named `tt/rbw-MF.MarshalFeigns.sh`, `chmod +x`, then build and commit it **on the probe branch only**:

```
{cand}/tt/rbw-tb.Build.sh
git -C {cand} add -A && git -C {cand} commit -m "probe: feign a station"
{cand}/tt/rbw-MF.MarshalFeigns.sh
{cand}/tt/rbw-ts.TestSuite.reveille.sh
```

Green reveille here is the consumer-seat proof: a stranger, cloning this candidate and onboarding, gets a working system.

Then return to the sterile branch and **delete the probe branch outright**:

```
git -C {cand} checkout POSTULANT_LOCAL
git -C {cand} branch -D probe
```

Deleting the probe now — before any push exists in the flow — is load-bearing: the probe branch deliberately holds a withheld path (the feigned station, the copied marshal tabtarget), and dropping it leaves the clone carrying exactly one branch, `POSTULANT_LOCAL`, and zero remotes. Even a forbidden fan-out push would then find nothing extra to carry. Do not re-run expede's sweep against the probe branch expecting green — it deliberately holds a withheld path.

## 8. Operator's own eyes

```
git -C {cand} ls-files
```

Show the operator the full file list — a few hundred lines — and **wait**. No machine judgment substitutes for the maintainer reading what they are about to publish. The root `CLAUDE.md` in that list is the **consumer** template's bytes, transposed onto `CLAUDE.md` by expede between materialization and the commit — expect a consumer-facing context file there, never the maintainer's. This driver performed no copy; expede did.

---

## The cut ends here — no push

The candidate stands built, proven, and **unpushable**: one commit atop the public base, on `POSTULANT_LOCAL`, holding zero remotes. This driver deliberately contains no push. Every push is your own hand, back in `RELEASE.md`:

- **Step 6** — the reversible preview into the **private** quarantine (`scaleinv/recipebottle-staging`), `POSTULANT_LOCAL:POSTULANT_LOCAL`, by explicit URL.
- **Step 7** — the **irreversible** public staging reveal: `POSTULANT_LOCAL:POSTULANT_LOCAL` onto the **public** repo (`scaleinv/recipebottle`) as an unmerged branch, `main` untouched — the disclosure the walk then clones.
- **Step 9** — promotion onto public `main`, the one main-touching push, on the far side of the greenfield walk.

Keep the candidate directory until step 7 is tip-verified; then, when the candidate is dispositioned, delete the public staging branch, the private quarantine repository, and the candidate directory. The quarantine's whole value is that it does not outlive the cut.
