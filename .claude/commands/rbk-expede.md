---
description: Cut and prove the upstream release candidate (RELEASE.md step 5)
---

You are cutting the delivery candidate and proving it fit to publish. This is step 5 of `RELEASE.md`.

**Read this first — it is why the ceremony is short.**

The candidate is built by **addition**, not by stripping. `tt/rbw-ME.MarshalExpedes.sh` (*expede*) clones the **public** upstream repository, clears its tree, materializes every path the **perambulation** ships — from committed bytes, via `git archive` — sterilizes them, and lays down exactly one commit. No private object ever enters the candidate's object graph, because nothing withheld is ever put in. Everything the old ceremony did by hand — squash merges, strip lists, do-not-commit landmines — is gone, and gone structurally, not by discipline.

Expede already gates on: a clean committed tree, a **total** perambulation (any unjudged tracked path or any dead row refuses the cut), `OPEN_SOURCE_UPSTREAM` configured, an object-graph leak sweep of the public base *before* adding to it, `rev-list --count == 1`, and a leak sweep of the finished candidate. Do not re-implement any of these. Run the verb and read its verdict.

**The candidate has no station and no secrets directory, and that is load-bearing.** It is why the operator's freehold subject cannot reach the candidate's generated Rust. Never copy station files or secrets into it. Step 5 below writes a *fresh, identity-free* three-line station so the candidate can run its own tests; that is the consumer's first onboarding act, not a leak.

**A finding means re-cut, never patch forward.** A red assay does not get fixed on the candidate. Abandon the candidate directory, repair on `main`, run this ceremony again from the top. Nothing is pushed until the operator pushes it by hand.

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

## 2. Quarantine-remote gate

The candidate is pushed to an **ephemeral quarantine repository** — private, created empty by the operator before a cut, deleted by hand after the candidate is dispositioned. It is never created or destroyed by tooling: no delete-scoped token exists, and none may ever be minted. The quarantine is *containment*; expede's construction is *prevention*. Containment never substitutes for prevention.

Read the remote and check it, credential-free:

```
git remote get-url OPEN_SOURCE_UPSTREAM
```

Then, with `<owner>/<repo>` taken from that URL:

- **Unseeable by strangers** — an anonymous API read must 404:
  ```
  curl -s -o /dev/null -w '%{http_code}\n' https://api.github.com/repos/<owner>/<repo>
  ```
  Anything but `404` means the quarantine repo is public (or the URL is wrong). **Stop.**

- **Fresh** — the quarantine must be either empty (no refs at all, the pre-first-publish state) or carrying exactly `refs/heads/main`:
  ```
  git ls-remote --heads OPEN_SOURCE_UPSTREAM
  ```
  Anything else — a second branch, a stale candidate branch — means a prior cut was not dispositioned. **Stop**, and ask the operator to dispose of it.

Once a true public release repository exists, its `main` SHA must also equal the quarantine's `main` SHA; until then that clause is vacuous.

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

Expede carries no typed confirmation: this ceremony drives it headlessly, so a prompt could only ever teach a bypass. Its refusals are structural — clean tree, absolute path, target must not exist, both object-graph sweeps — and a refusal is not a prompt: it cannot be answered, only satisfied.

The candidate lands at `{target}/candidate`. Read the verb's verdict: one commit, both sweeps clean.

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

Run the candidate's **own** tabtargets, by absolute path, on its sterile `main`.

```
{cand}/tt/rbw-tq.QualifyFast.sh
{cand}/tt/rbw-tf.FixtureRun.sh cupel
{cand}/tt/rbw-tf.FixtureRun.sh pyx
{cand}/tt/rbw-tf.FixtureRun.sh damnatio
```

- **`rbw-tq`** — the delivered wiring is intact: colophons, tabtargets, nameplates.
- **`cupel`** — BCG command discipline **as the candidate resolves it**. This is the reading no assay in the working repository can perform: a shipped file that calls a withheld module's function resolves cleanly on `main` and names an unknown command here. It needs no station, so it runs on the sterile tree, before feigning — the 2026-07-14 try-2 candidate died on exactly this, and only discovered it a whole probe later.
- **`pyx`** — release hygiene of the tree being published.
- **`damnatio`** — the proof of erasure: no site identity survives anywhere the proscription reaches. This is the assay the whole design exists to pass. It must run **on `main`, before any feigning** — damnatio reddens on feigned fields by construction, which is exactly what keeps a probe branch from ever being mistaken for a candidate.

Any red: abandon the candidate, repair on `main`, re-cut.

## 7. Consumer-seat probe

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

Then return to the sterile branch:

```
git -C {cand} checkout main
```

**The probe commits are unreachable from `main`**, so the push in step 9 cannot carry them. Do not push any branch but `main`, and do not re-run expede's sweep against the probe branch expecting green — it deliberately holds a withheld path.

## 8. Operator's own eyes

```
git -C {cand} ls-files
```

Show the operator the full file list — a few hundred lines — and **wait**. No machine judgment substitutes for the maintainer reading what they are about to publish.

## 9. Push, by the operator's hand

```
git -C {cand} push origin main
```

`origin` in the candidate *is* the quarantine repository — the clone came from it.

Then: inspect on GitHub, and when the candidate is dispositioned, **delete the quarantine repository and the candidate directory**. The quarantine's whole value is that it does not outlive the cut.
