---
description: Prepare release candidate for upstream delivery
---

You are preparing a release candidate branch for upstream delivery to OPEN_SOURCE_UPSTREAM.

**This is an interactive ceremony.** Present each step, show output, and wait for user acknowledgment before proceeding to the next step. Do not race ahead.

**Important:**
- Be methodical — show output at each step
- Stop immediately on errors
- User maintains control throughout
- All destructive transforms happen inside a throwaway candidate clone, never in the working repository

**Where this ceremony runs.** Steps 0–3 run in the working repository. Step 4 creates an isolated **candidate clone** (`rbw-MP`); every step from 5 onward runs *inside that clone*. This is load-bearing, not hygiene theater: marshal zero (Step 5) blanks the regime `.env` files in place and auto-commits the result, and the candidate is built by squashing that blanked tree — doing this in a clone is what keeps the operator's working config untouched. It also gives marshal zero a tree whose `HEAD` equals its `origin` snapshot by construction, so its pushed-state gate passes without pushing anything (see Step 5). After Step 4, restart Claude Code (or your shell) inside the clone directory before continuing.

**Verify, then commit — not the other way round.** The theurge harness gates every fixture on a clean git tree, so the candidate commit (Step 12) lands *before* the assays that judge it (Steps 13–14). That is not a hole: the contract is **a finding means re-cut, never patch-forward**. A red assay does not get fixed on the candidate branch — the branch is abandoned, the finding is repaired on `main`, and the ceremony runs again from Step 1. Nothing is pushed until Step 15.

## Step 0: Request permissions

Ask the user for permission to execute all git operations needed (clone, checkout, branch creation, squash merge, file removal, commit). Get explicit approval before proceeding.

## Step 1: Verify main is clean and pushed

- Check `git status` on the main branch — must be clean
- Verify main is pushed to origin (`git rev-list --count origin/main..main` is `0`)
- If dirty or unpushed, **STOP** and ask user to resolve

The clone in Step 4 is taken from this working repo, so the candidate reflects exactly what is committed here.

## Step 2: Regime variable completeness check (LLM task)

Before stripping removes the spec documents, verify every enrolled RBK regime variable has spec treatment. This runs in the working repo, where the specs are still present. For each regime, read the enrollment file and corresponding spec:

| Regime file | Spec document |
|-------------|---------------|
| `Tools/rbk/rbrr_regime.sh` | `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` |
| `Tools/rbk/rbrd_regime.sh` | `Tools/rbk/vov_veiled/RBSRT-RegimeDepot.adoc` |
| `Tools/rbk/rbrf_regime.sh` | `Tools/rbk/vov_veiled/RBSRF-RegimeFederation.adoc` |
| `Tools/rbk/rbrn_regime.sh` | `Tools/rbk/vov_veiled/RBRN-RegimeNameplate.adoc` |
| `Tools/rbk/rbro_regime.sh` | `Tools/rbk/vov_veiled/RBSRO-RegimeOauth.adoc` |
| `Tools/rbk/rbrp_regime.sh` | `Tools/rbk/vov_veiled/RBSRP-RegimePayor.adoc` |
| `Tools/rbk/rbrv_regime.sh` | `Tools/rbk/vov_veiled/RBSRV-RegimeVessel.adoc` |
| `Tools/rbk/rbrw_regime.sh` | `Tools/rbk/vov_veiled/RBSRW-RegimeWorkforce.adoc` |

For each regime:
1. Extract all enrolled variable names (look for `buv_*_enroll` calls — first argument is the variable name)
2. Grep for each variable name in the corresponding spec document
3. Report any variables missing from their spec

Present results to the user. Gaps are informational — they don't block the ceremony but should be catalogued for future work.

Wait for user acknowledgment.

## Step 3: Pre-strip qualification

Run full release qualification in the working repo to verify the complete codebase is healthy before any transforms:

```
tt/rbw-tr.QualifyRelease.sh
```

This runs shellcheck and the `echelon` test suite. Two fixtures in that suite stand in for audits a maintainer once ran by hand: **cupel**, the command-dependency lint statically enforcing BCG's POSIX-floor / declared-dependency / eviction-table discipline across all kit bash; and **pyx**, the release-hygiene assay (crate licenses, root LICENSE, secret shapes, anchor resolution). Pyx runs again post-strip in Step 13, against the candidate tree — a different tree, with different files.

Then run **loupe**, the veil-leak sweep, by name:

```
tt/rbw-tf.FixtureRun.sh loupe
```

No shipping file may name what the distribution withholds — not the veiled tree by path, and not a withheld `.adoc`/`.md` by basename. **A veil finding here is a release blocker, not a nit**: this run is the only one that proves it, because loupe harvests its needle set from the veiled trees themselves and they are gone from every later tree.

Loupe is a member of **no suite**, which is why it is invoked here by name — the mirror of damnatio in Step 13. Its census is empty in any tree without veiled trees, so it is red by construction in the delivered one; and the reveille suite *ships*. A suite membership would have handed every consumer a red fixture asserting the absence of documents they were never given. Do not "repair" that by returning it to reveille.

If qualification or loupe fails, **STOP**. The full codebase must pass before we proceed.

Show both results and wait for user acknowledgment.

## Step 4: Create the candidate clone

Create the isolated clone that hosts every remaining step:

```
tt/rbw-MP.MarshalProofs.sh <absolute-target-dir>
```

**Name the target directory `rbm_candidate_{cut-date}`** — underscore form, the cut's own date, e.g. `rbm_candidate_20260713`. This supersedes the `rbm-proof-*` naming: what the directory holds is the candidate, and the date says which cut it is, so a clone left on disk months later still announces what it was for.

This clones the working repo to `<target-dir>/<repo-name>`, re-points the clone's `origin` at the real origin URL, carries `OPEN_SOURCE_UPSTREAM` across if configured, and copies the operator's station files and secrets (the payor OAuth credential the ceremony's tools need). The target directory must not already exist.

**Everything from here runs inside the clone.** Restart Claude Code (or `cd`) in the clone directory before Step 5. The working repository is now untouched for the rest of the ceremony.

Show the proof output and wait for user acknowledgment.

## Step 5: Marshal zero, then lustrate (in the clone)

Two transforms, in this order, on the clone's main branch. Together they return the tree to a state that carries no station of its own.

```
tt/rbw-MZ.MarshalZeroes.sh
tt/rbw-ML.MarshalLustrates.sh
```

**Marshal zero** returns the regime tree to the blank onboarding-start template. It:
- Blanks the site-specific `RBRR_RUNTIME_PREFIX` and pre-fills RBRR defaults (DNS server, GCB timeout, min concurrent builds, vessel dir, secrets dir) in `rbrr.env`
- Blanks depot identity (`RBRD_CLOUD_PREFIX`, `RBRD_DEPOT_MONIKER`) and pre-fills RBRD defaults (GCP region, GCB machine type) in `rbrd.env`
- Blanks hallmark pins (`RBRN_SENTRY_HALLMARK`, `RBRN_BOTTLE_HALLMARK`) in every nameplate `rbrn.env`
- Blanks depot-scoped vessel fields (`RBRV_RELIQUARY`, `RBRV_IMAGE_*_ANCHOR`) in every `rbrv.env`
- **Preserves** the Payor OAuth credential (`rbro.env`) — payor-scoped, survives a depot change. No credential files are deleted: the federation era mints short-lived mantle tokens, not RBRA keyfiles.

**Marshal zero is not enough, and never was.** It mints the *gauntlet's* entry state, and the gauntlet runs against the operator's live payor — so zero deliberately leaves the payor's own identity standing. What zero does not touch, and what therefore rode into every candidate before 2026-07-13: the payor regime (`rbrp.env` — project, billing account, OAuth client id, operator email), the workforce regime (`rbrw.env` — GCP org id, workforce pool id), the active foedus's federation regime (`rbrf.env` — IdP tenant, IdP client id, both device/token endpoints), one vessel's `RBRV_GRAFT_IMAGE`, and the freehold subject in `rbpc_constants.sh` (the operator's Entra `oid`, which the build also projects into generated Rust).

**Lustration** is the transform that erases those. It reads the **proscription** in `Tools/rbk/rblm_lustrate.sh` — the one table that judges every enrolled regime field either *site-scoped* (this station's) or *common* (the same at every installation) — and writes the sanctioned sterile value over every site-scoped home. It prompts for confirmation (type `lustrate`) and **auto-commits**. It is withheld from delivery alongside the other marshal tabtargets (Step 10c).

Lustration also sets `RBRR_PUBLIC_DOCS_URL` to the delivery base, because that field is site-scoped like any other: today it points at the maintainer's development repo, and the delivered tree must point consumers at the public home. The value is recorded in the proscription, not typed by hand here, and Step 12's damnatio run proves it landed.

**Run zero BEFORE lustrate.** `rbw-MZ` gates on a clean, pushed, lint-clean, colophon-complete tree *before* any mutation, and its own auto-commit leaves `HEAD` unpushed — so lustration must follow it, never precede it. A candidate branch is unpushed by construction, which is why both run here, on the clone's main, before the cut: a fresh clone's `HEAD` equals its `origin/main` snapshot, so zero's pushed-state gate is satisfied without pushing anything.

Show `git log -2 --stat` to confirm both commits, then wait for user acknowledgment.

## Step 6: Fetch upstream state

- `git fetch OPEN_SOURCE_UPSTREAM`
- If fetch fails, **ABORT** and ask user to verify remote configuration

Show results and wait for user acknowledgment.

## Step 7: Auto-detect next candidate branch

- Find max batch from upstream: `git ls-remote --heads OPEN_SOURCE_UPSTREAM | grep 'candidate-'`
- Find max batch from local branches
- If batch exists upstream: new batch (previous was merged)
- If batch not upstream: redo (increment revision)
- Tell user which branch name was chosen and why

Wait for user approval of the branch name.

## Step 8: Create candidate branch and squash merge

- Show commits that will be included: `git log OPEN_SOURCE_UPSTREAM/main..main --oneline`
- `git checkout -b candidate-NNN-R OPEN_SOURCE_UPSTREAM/main`
- Execute: `git merge --squash main` (main here is the clone's blanked main from Step 5)
- **Resolve any merge conflicts to match the blanked main.** When the upstream base has drifted far from main, the squash can conflict — a file main deleted but upstream modified (resolve by `git rm <file>`), or a content conflict on a surviving file (repopulate main's version with `git show main:<file> > <file>` then `git add <file>`; do not use `git checkout`/`git restore`). Confirm `git diff --name-only --diff-filter=U` is empty.
- Commit the squash so the working tree is clean for the steps that follow.
- Show the merge result summary

Wait for user acknowledgment.

## Step 9: Extract consumer templates

The consumer `CLAUDE.md` template lives in `vov_veiled/`, which will be stripped in Step 10. Extract it now. `README.md` is tracked directly at the repo root (consumer-facing) and needs no extraction.

```
cp Tools/rbk/vov_veiled/CLAUDE.consumer.md CLAUDE.md
```

Note: `CLAUDE.md` is overwritten (replacing the development version).

**The delivery docs URL is no longer set here.** `RBRR_PUBLIC_DOCS_URL` is a site-scoped field like any other, so it is the proscription's business: lustration (Step 5) wrote the recorded delivery base over it, and Step 12's damnatio run proves the value landed. A hand-typed URL and a hand-written grep to check it were two places for the same fact to be wrong; there is now one.

Show the user what was copied. Wait for acknowledgment.

## Step 10: Strip proprietary content

Remove all proprietary content from the candidate branch. Present the full strip plan to the user, then execute after approval.

**10a. Recursive glob — all vov_veiled directories:**
```
git rm -rf --ignore-unmatch Tools/buk/vov_veiled/
git rm -rf --ignore-unmatch Tools/cmk/vov_veiled/
git rm -rf --ignore-unmatch Tools/gad/vov_veiled/
git rm -rf --ignore-unmatch Tools/jjk/vov_veiled/
git rm -rf --ignore-unmatch Tools/rbk/vov_veiled/
git rm -rf --ignore-unmatch Tools/vok/vov_veiled/
git rm -rf --ignore-unmatch Tools/vvk/vov_veiled/
```

**10b. Whole directories — internal tools and infrastructure:**
```
git rm -rf --ignore-unmatch .claude/
git rm -rf --ignore-unmatch .idea/
git rm -rf --ignore-unmatch .jjk/
git rm -rf --ignore-unmatch lenses/
git rm -rf --ignore-unmatch Memos/
git rm -rf --ignore-unmatch Study/
git rm -rf --ignore-unmatch _slickedit/
git rm -rf --ignore-unmatch RBM-nameplates/
git rm -rf --ignore-unmatch Tools/apck/
git rm -rf --ignore-unmatch Tools/ccck/
git rm -rf --ignore-unmatch Tools/cmk/
git rm -rf --ignore-unmatch Tools/gad/
git rm -rf --ignore-unmatch Tools/hmk/
git rm -rf --ignore-unmatch Tools/jjk/
git rm -rf --ignore-unmatch Tools/lmci/
git rm -rf --ignore-unmatch Tools/temp-buk/
git rm -rf --ignore-unmatch Tools/vok/
git rm -rf --ignore-unmatch Tools/vslf-rbw/
git rm -rf --ignore-unmatch Tools/vslk/
git rm -rf --ignore-unmatch Tools/vvc/
git rm -rf --ignore-unmatch Tools/vvk/
git rm -rf --ignore-unmatch rbmm_moorings/rbmn_nodes/
git rm -rf --ignore-unmatch rbmm_moorings/rbmu_users/
```

`rbmn_nodes/` (BURN remote-node profiles) and `rbmu_users/` (BURP user profiles) carry operator-specific machine identities; their reader code (the BURN/BURP apparatus) is already veiled, so nothing consumer-visible consumes them.

**10c. Internal tabtargets (non-rbw, non-buw operational targets):**
```
git rm -f --ignore-unmatch tt/apcw-*.sh
git rm -f --ignore-unmatch tt/butctt.TestTarget.sh
git rm -f --ignore-unmatch tt/ccck-s.ConnectShell.sh
git rm -f --ignore-unmatch tt/jjw-*.sh
git rm -f --ignore-unmatch tt/study-mpt.Run.*.sh
git rm -f --ignore-unmatch tt/vow-*.sh
git rm -f --ignore-unmatch tt/vslk-*.sh
git rm -f --ignore-unmatch tt/vvw-*.sh
git rm -f --ignore-unmatch tt/rbw-MZ.MarshalZeroes.sh
git rm -f --ignore-unmatch tt/rbw-MP.MarshalProofs.sh
git rm -f --ignore-unmatch tt/rbw-ML.MarshalLustrates.sh
git rm -f --ignore-unmatch tt/rbw-MF.MarshalFeigns.sh
git rm -f --ignore-unmatch tt/rbw-mR.PayorRazesManor.sh
```

`tt/jjw-*.sh` are JJK fundus-test tabtargets whose workbench and launcher are stripped above.

**Every marshal tabtarget is withheld — all four.** They are the ceremony's own instruments and each is destructive in a consumer's hands: `rbw-MZ` blanks their regime, `rbw-ML` sterilizes it, `rbw-MF` writes a false station over it, `rbw-MP` clones their repo and copies their secrets. `rbw-ML` was omitted from this list until 2026-07-13 while the document claimed otherwise, so the lustrate accelerator shipped; the rule is now the simple one — the `rbw-M*` colophon family does not ship, and the library (`rblm_lustrate.sh`, `rblm_cli.sh`) does, since it carries no site value and the delivered tree has no tabtarget that reaches it.

`rbw-mR` (manor raze) is internal release-ladder infra — a one-keystroke workforce-pool destroyer. The verb `rbgp_manor_raze` ships (it lives in the surviving `Tools/rbk/rbgp_payor.sh`); only the tabtarget accelerator is withheld, so consumers never get the accelerator.

**10d. Internal launchers (for stripped workbenches):**

Launchers live under `rbmm_moorings/rbml_launchers/`. Only the `buw` and `rbw` launchers survive; strip every workbench launcher whose kit is stripped above.
```
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.apcw_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.cmw_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.jjw_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.study_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.vow_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.vslw_workbench.sh
git rm -f --ignore-unmatch rbmm_moorings/rbml_launchers/launcher.vvw_workbench.sh
```

**10e. Individual files:**
```
git rm -f --ignore-unmatch RELEASE.md
git rm -f --ignore-unmatch podman-gateway-proposal.md
git rm -f --ignore-unmatch brm_recipemuster.iml
git rm -f --ignore-unmatch index.html .nojekyll
git rm -f --ignore-unmatch .mcp.json
git rm -f --ignore-unmatch wsl@rocket
git rm -f --ignore-unmatch Tools/cccr.env
git rm -f --ignore-unmatch Tools/crgr.render.sh
git rm -f --ignore-unmatch Tools/crgv.validate.sh
git rm -f --ignore-unmatch Tools/xxx_rbn.info.sh
git rm -f --ignore-unmatch rbmm_moorings/fdkyclk/fdkyclk-proof.sh
git rm -f --ignore-unmatch rbmm_moorings/fdkyclk/fdkyclk-teardown.sh
```

The two `fdkyclk-*.sh` scripts are proof-stage scaffolding, withheld from 2026-07-13. They are not merely un-productized — a consumer **cannot run them**: `fdkyclk-proof.sh` reads the payor secret from `RBRR_SECRETS_DIR`, which never ships, and stands up a *separate* workforce pool (`fdkyclk-test`) precisely because, as its own header says, the real admission verbs assume the single manor pool. The shipped facility (`rbxk_keycloak.sh` behind `rbw-qjK`/`rbw-qjQ`) does the same work through the real verbs and never touches the payor credential. They also hardcode the operator's GCP org id and a depot project id — shapeless values no scanner can catch — so sterilizing them would have meant carrying roster rows for plain shell constants in scaffolding nobody can use. Nothing in the shipping tree references either script.

`RELEASE.md` is the maintainer's release-qualification procedure — it references tooling withheld from delivery (`rbw-MZ`, `rbw-MP`, `/rbk-prep-release`), so it is withheld too (decided 2026-07-12). `.mcp.json` configures the internal `vvx` MCP server (`Tools/vvk/`), which is stripped. `wsl@rocket` is a BURS station regime config for the `wsl@rocket` ssh test host (operator-site-specific: `BURS_USER`/`BURS_TINCTURE`/`BURS_LOG_DIR`), sitting at the repo root — a site-specific config of the same kind as the `rbmn_nodes/`/`rbmu_users/` profiles stripped in 10b, withheld for the same reason.

**10f. Stage the consumer templates and marshal zero changes:**
```
git add CLAUDE.md
git add -u
```

After all removals, verify with `git ls-files` that no proprietary content remains. Show the user a summary of what was removed and what survives. **Pause for careful review.**

### What should survive after stripping:

- `rbmm_moorings/` — the consumer-config tree (replaces the former `.buk/` + `.rbk/` homes):
  - `burc.env` and the regime `.env` files — `rbrr.env` and `rbrd.env` blanked by marshal zero, `rbrp.env` and `rbrw.env` blanked by **lustration** (Step 5). Marshal zero never touched the payor and workforce regimes; the sentence that once claimed it did is why the operator's project, billing account, OAuth client id, org id and pool id rode out unnoticed until 2026-07-13. Do not restore that claim: zero and lustration have different jobs, and only the proscription knows the whole set.
  - `rbmf_foedera/` — the foedus library (holds the federation regime: `rbef_entrada/rbrf.env`, lustrated, and the committed `rbef_keycloak/rbrf.env.template`, which is synthetic at every installation and ships as authored)
  - `rbml_launchers/` — only `launcher.buw_workbench.sh` and `launcher.rbw_workbench.sh` (the rest stripped in 10d)
  - `rbmv_vessels/` — vessel definitions and README (the former `rbev-vessels/`)
  - the per-nameplate dirs `ccyolo/ moriah/ nineveh/ pluml/ srjcl/ tadmor/ fdkyclk/` — the directories all ship: README documents each as an example crucible with its own anchor, and the onboarding handbooks walk several. `srjcl/workspace/` (the Jupyter sample content) ships with it. **The exception is inside `fdkyclk/`**: its two proof-stage `.sh` scripts are stripped in 10e (see there). This is a per-file carve-out on purpose — a directory-grain "all ship" is what carried those two scripts, with the operator's org id in them, into every candidate.
- `Tools/rbk/rbxk_cli.sh` + `rbxk_keycloak.sh`, `tt/rbw-qjK.*` / `rbw-qjQ.*` / `rbw-cC.Charge.fdkyclk.sh` / `rbw-cQ.Quench.fdkyclk.sh`, the `fdkyclk` nameplate + `rbev-bottle-fdkyclk` vessel + `rbef_keycloak` foedus template — the whole Keycloak synthetic-federation test facility **ships** as RB's federation test surface.
- `CLAUDE.md` — consumer version (copied in Step 9)
- `README.md` — consumer-facing, tracked directly at the repo root
- `LICENSE`
- `rbm-abstract-drawio.svg`
- `diagrams/` — the `rbdg*` PlantUML sources and rendered light/dark `.svg` pairs
- `Tools/buk/` — all `.sh` files, `busc_shellcheckrc`, `README.md`, `buts/` test support (minus `vov_veiled/`)
- `Tools/rbk/` — all `.sh` files (minus `vov_veiled/`), including the theurge crate `rbtd/`
- `tt/` — `rbw-*` and `buw-*` tabtargets only (minus the whole `rbw-M*` marshal family and `rbw-mR` manor-raze)

**Do NOT strip the fdkyclk caged credentials.** `rbmm_moorings/fdkyclk/fdkyclk-asserter-key.pem` (a real `BEGIN PRIVATE KEY`) and `fdkyclk-client-secret.txt` look alarming and a secret scanner will flag them, but they are **intentional committed test scaffolding** — the caged asserter keypair whose public half is baked into the realm's `publicKeySignatureVerifier`, and the confidential-client secret matching that realm (RBSFK "two-keys"). Git-ignoring or stripping them breaks the test-bed's determinism; the realm expects exactly those. The security posture is handled by documentation, not removal — see the fdkyclk caution in `README.md`. The realm's *own* signing key (the id-tokens Keycloak issues) is a separate, ephemeral key minted per charge and committed nowhere; the live `rbef_keycloak/rbrf.env` that snapshots it is git-ignored and never ships.

## Step 11: Regenerate derived files

The strip removed tabtargets from `tt/`, and the committed tabtarget context (`Tools/rbk/claude-rbk-tabtarget-context.md`) is generated by enumerating `tt/` on disk — so it is now stale, and Step 13's freshness gate will reject it. Regenerate from the stripped tree:

```
tt/rbw-tb.Build.sh
```

This rewrites `claude-rbk-tabtarget-context.md` to reflect the surviving tabtarget set. (The Rust colophon consts `rbtdgc_consts.rs` are zipper-driven, not disk-driven, so they do not change — the withheld `rbw-M*` and `rbw-mR` colophons stay enrolled there, which is fine: fast-qualify sweeps disk→registry, never registry→disk.) Stage the regenerated file:

```
git add Tools/rbk/claude-rbk-tabtarget-context.md
```

Show the user what regenerated and wait for acknowledgment.

## Step 12: Commit the candidate

The candidate must be a committed, clean tree before anything assays it: the theurge harness gates every fixture on a clean git tree, so an uncommitted strip cannot be tested. That is why the commit lands here, ahead of the verification it will be judged by — see **Verify, then commit** at the top. Re-read the contract before proceeding: **a finding in Step 13 or 14 means re-cut, never patch-forward.**

- Stage any remaining changes: `git add -u`
- Analyze all changes for a consolidated commit message
- Review `git log OPEN_SOURCE_UPSTREAM/main..main --oneline` to summarize what's included
- Create commit (no attribution footer — this is a release candidate)
- Show `git log -1 --stat`

Nothing is pushed. Wait for user acknowledgment.

## Step 13: Post-strip verification

Run fast qualification on the stripped candidate tree:

```
tt/rbw-tq.QualifyFast.sh
```

This validates that stripping didn't break wiring — tabtargets resolve, colophons match surviving modules, the generated context and Rust consts are fresh, and nameplate preflight passes. No shellcheck, no test suite — the full `echelon` test already passed pre-strip in Step 3, and the stripped tree lacks cloud infrastructure to run integration tests.

Then run the **pyx** release-hygiene fixture against the candidate tree:

```
tt/rbw-tf.FixtureRun.sh pyx
```

Pyx asserts what must hold of the tree we are about to publish: every crate in the shipping lockfile is license-vetted, the root LICENSE stands, no shipping file carries a credential shape, and every anchor the handbooks and README link to resolves. It ran green pre-strip inside Step 3's suite; running it again here is what proves the *stripped* tree — a different tree, with different files — is fit to publish. Its checks are deterministic tree-invariants over committed files: no credentials, no network, seconds to run.

Loupe is **not** run here. It is the assay of the tree BEFORE the cut (Step 3), and its census is empty in this one — see Step 3 for why that makes it red by construction rather than vacuous.

Note what pyx does NOT cover: the known-vulnerability advisory audit. That verdict moves with a live advisory database while the tree stands still, so it cannot be a fixture. It stays here, as a step you own.

Then run the **damnatio** fixture — the identity assay — against the candidate tree:

```
tt/rbw-tf.FixtureRun.sh damnatio
```

Where pyx asks whether the tree carries *any* secret, damnatio asks whether it carries *this operator*. Its four cases: a shape sweep for site-identity forms (a UUID bound as a value, a GCP billing-account id, a Google OAuth client id) across every shipping file; an assertion that every site-scoped field named by the proscription holds its sanctioned sterile value, which is the only net that can catch shapeless identity like a workforce pool id or a project id; a completeness check that reads the **live** enrollment rolls, so a regime field enrolled since the last release reddens here until someone judges it site-scoped or common; and an assertion that the strip removed the veiled trees.

**Damnatio is a member of no suite, and it is red against the maintainer's working tree by construction** — that tree is *supposed* to hold the live configuration. It means one thing, and only in this seat: this stripped, lustrated tree is fit to publish. That is why it is invoked by name here and nowhere else. It is also what makes the next step safe: the sterile-value case reddens on all thirteen feigned fields, so a probe branch could never be mistaken for a candidate.

**Ceremony link check** — pyx's README-anchor case (run above) proves file-to-file on the candidate tree that every anchor minted in `Tools/rbk/rbyc_common.sh` (the third argument of each `zrbyc_yk` call) is defined in the candidate `README.md` as a literal `<a id="…">`. A green pyx IS this check; no separate command. The URL base those links ride on (`RBRR_PUBLIC_DOCS_URL`) is proven by damnatio's proscribed-value case, which asserts it equals the delivery base recorded in the proscription — the hand-written grep that used to sit here was a second place for that fact to be wrong.

**If any check fails, STOP.** A fast-qualification failure means something in the consumer-visible code depends on stripped content. A pyx failure means the candidate is not fit to publish. A damnatio failure means the candidate carries the maintainer's identity: a leak that reaches a public repo cannot be recalled, so this one is absolute — never wave it through, never "fix it after the push." Each is a real finding that must be investigated before proceeding.

Show the result and wait for user acknowledgment.

## Step 14: The consumer-seat probe

Everything so far has judged the candidate from the maintainer's chair. This step sits in the consumer's: it asks whether the tree we are about to hand someone *works in their hands*. It is a standing step of every cut.

**14a. Leakage sweep.** Audit what actually survived, against what was meant to:

```
git ls-files
```

Read the whole list. Nothing proprietary, no stray site config, no tabtarget whose workbench was stripped. This is the human half of the strip review — pyx and damnatio mechanize the credential and identity halves, and loupe mechanized the veil half back in Step 3, but no fixture knows that a file simply has no business shipping.

**14b. The probe branch.** The candidate cannot run its own tests: it is lustrated, and thirteen of the blanked site fields carry format checks a blank cannot satisfy, so regime validation refuses the tree. That is correct behavior — a sterile tree *should* refuse — and it is exactly why the probe needs a station invented for it. Cut a throwaway branch and feign one:

```
git checkout -b probe-candidate-NNN-R
tt/rbw-MF.MarshalFeigns.sh
```

Feigning writes a shape-valid stand-in over every site field the proscription carries a feigned value for, and auto-commits — the harness demands a clean tree, so the seed must be committed, which is why this happens on a branch and not in the working tree. Every value is visibly false (zeros, the `.invalid` TLD). The verb refuses to run on `main` or any `candidate-*` branch, so the feigned station cannot reach the thing that ships.

The tabtarget was stripped in Step 10c. Restore it onto the probe branch alone:

```
git show main:tt/rbw-MF.MarshalFeigns.sh > tt/rbw-MF.MarshalFeigns.sh
chmod +x tt/rbw-MF.MarshalFeigns.sh
```

**14c. Reveille from the consumer's seat.** Run the credless base suite — the first thing a consumer runs, and the only suite that needs no cloud, no container, and no credential:

```
tt/rbw-ts.TestSuite.reveille.sh
```

Green here means the tree we are shipping can be validated, built, and tested by someone who has just cloned it. Red here means we were about to ship something broken, and no earlier step could have told us: every prior gate ran against a tree that still had the maintainer's station in it.

**14d. Discard the probe.** The probe branch is a throwaway. It is never pushed, never merged, and never becomes the candidate. Leave it in the clone (which is itself discarded) and return to the candidate branch:

```
git checkout candidate-NNN-R
```

**If the sweep or reveille fails, STOP — and re-cut.** A finding here is repaired on `main`, not on the candidate: the branch is abandoned and the ceremony runs again from Step 1. Never patch-forward a candidate.

Show the results and wait for user acknowledgment.

## Step 15: Final review

Show the user:
- The commit stat summary
- Push instructions (from the clone): `git push OPEN_SOURCE_UPSTREAM candidate-NNN-R`
- Confirmation that the push is from the **candidate** branch, never the probe branch
- Reminder: inspect the result on GitHub before merging to main
- Reminder: the candidate clone is a throwaway — delete its directory once the candidate is pushed

**STOP** — user reviews and pushes manually.
