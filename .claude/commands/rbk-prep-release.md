---
description: Prepare release candidate for upstream delivery
---

You are preparing a release candidate branch for upstream delivery to OPEN_SOURCE_UPSTREAM.

**This is an interactive ceremony.** Present each step, show output, and wait for user acknowledgment before proceeding to the next step. Do not race ahead.

**Important:**
- Be methodical — show output at each step
- Stop immediately on errors
- User maintains control throughout
- All destructive transforms happen inside a throwaway proof clone, never in the working repository

**Where this ceremony runs.** Steps 0–3 run in the working repository. Step 4 creates an isolated **proof clone** (`rbw-MP`); every step from 5 onward runs *inside that clone*. This is load-bearing, not hygiene theater: marshal zero (Step 5) blanks the regime `.env` files in place and auto-commits the result, and the candidate is built by squashing that blanked tree — doing this in a clone is what keeps the operator's working config untouched. It also gives marshal zero a tree whose `HEAD` equals its `origin` snapshot by construction, so its pushed-state gate passes without pushing anything (see Step 5). After Step 4, restart Claude Code (or your shell) inside the clone directory before continuing.

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

This runs shellcheck and the `echelon` test suite. Two theurge fixtures in that suite stand in for audits a maintainer once ran by hand: **cupel**, the command-dependency lint statically enforcing BCG's POSIX-floor / declared-dependency / eviction-table discipline across all kit bash; and **pyx**, the release-hygiene assay (crate licenses, root LICENSE, secret shapes, anchor resolution). Pyx runs again post-strip in Step 12 against the candidate tree.

If qualification fails, **STOP**. The full codebase must pass before we proceed.

Show the qualification result and wait for user acknowledgment.

## Step 4: Create the proof clone

Create the isolated clone that hosts every remaining step:

```
tt/rbw-MP.MarshalProofs.sh <absolute-target-dir>
```

This clones the working repo to `<target-dir>/<repo-name>`, re-points the clone's `origin` at the real origin URL, carries `OPEN_SOURCE_UPSTREAM` across if configured, and copies the operator's station files and secrets (the payor OAuth credential the ceremony's tools need). The target directory must not already exist.

**Everything from here runs inside the clone.** Restart Claude Code (or `cd`) in the clone directory before Step 5. The working repository is now untouched for the rest of the ceremony.

Show the proof output and wait for user acknowledgment.

## Step 5: Marshal zero (in the clone)

Run marshal zero on the clone's main branch:

```
tt/rbw-MZ.MarshalZeroes.sh
```

Marshal zero returns the regime tree to the blank onboarding-start template. It:
- Blanks the site-specific `RBRR_RUNTIME_PREFIX` and pre-fills RBRR defaults (DNS server, GCB timeout, min concurrent builds, vessel dir, secrets dir) in `rbrr.env`
- Blanks depot identity (`RBRD_CLOUD_PREFIX`, `RBRD_DEPOT_MONIKER`) and pre-fills RBRD defaults (GCP region, GCB machine type) in `rbrd.env`
- Blanks hallmark pins (`RBRN_SENTRY_HALLMARK`, `RBRN_BOTTLE_HALLMARK`) in every nameplate `rbrn.env`
- Blanks depot-scoped vessel fields (`RBRV_RELIQUARY`, `RBRV_IMAGE_*_ANCHOR`) in every `rbrv.env`
- **Preserves** the Payor OAuth credential (`rbro.env`) — payor-scoped, survives a depot change. No credential files are deleted: the federation era mints short-lived mantle tokens, not RBRA keyfiles.

**Why this must be the clone, and before the candidate cut.** `rbw-MZ` is a source-side tool, withheld from delivery (it is stripped in Step 10); it gates on a clean, pushed, lint-clean, colophon-complete tree, *before* any mutation. A candidate branch is unpushed by construction, so marshal zero can never gate-pass there — that is why it runs here, on the clone's main, before the cut. A fresh clone's `HEAD` equals its `origin/main` snapshot, so the pushed-state gate is satisfied by construction without pushing. Marshal zero then prompts for confirmation (type `zero`) and **auto-commits** the blanked state as a single "Marshal Zero" commit on the clone's main.

Show `git log -1 --stat` to confirm the marshal-zero commit, then wait for user acknowledgment.

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

## Step 9: Extract consumer templates and set the delivery docs URL

The consumer `CLAUDE.md` template lives in `vov_veiled/`, which will be stripped in Step 10. Extract it now. `README.md` is tracked directly at the repo root (consumer-facing) and needs no extraction.

```
cp Tools/rbk/vov_veiled/CLAUDE.consumer.md CLAUDE.md
```

Note: `CLAUDE.md` is overwritten (replacing the development version).

**Set the delivery docs URL.** The working repo's `RBRR_PUBLIC_DOCS_URL` points at the maintainer's development repo; the delivered tree must point consumers at the public home instead. Marshal zero deliberately passes this field through untouched, so this ceremony step is the only place the delivery value lands. The recorded URL base — a delivery decision, revisited only when the public home moves (decided 2026-07-12: the public repo's README blob, because GitHub blob rendering preserves the literal `<a id>` anchors the handbook links resolve, while staging and candidate branches are transient plumbing) — is:

```
https://github.com/scaleinv/recipebottle/blob/main/README.md
```

Edit `rbmm_moorings/rbrr.env` so the field reads exactly:

```
RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
```

(Step 10f's `git add -u` stages this edit along with the other tracked changes.)

Show the user what was copied and the URL diff. Wait for acknowledgment.

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
git rm -f --ignore-unmatch tt/rbw-mR.PayorRazesManor.sh
```

`tt/jjw-*.sh` are JJK fundus-test tabtargets whose workbench and launcher are stripped above. `rbw-mR` (manor raze) is internal release-ladder infra — a one-keystroke workforce-pool destroyer. The verb `rbgp_manor_raze` ships (it lives in the surviving `Tools/rbk/rbgp_payor.sh`); only the tabtarget accelerator is withheld, so consumers never get the accelerator.

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
git rm -f --ignore-unmatch podman-gateway-proposal.md
git rm -f --ignore-unmatch brm_recipemuster.iml
git rm -f --ignore-unmatch index.html .nojekyll
git rm -f --ignore-unmatch .mcp.json
git rm -f --ignore-unmatch wsl@rocket
git rm -f --ignore-unmatch Tools/cccr.env
git rm -f --ignore-unmatch Tools/crgr.render.sh
git rm -f --ignore-unmatch Tools/crgv.validate.sh
git rm -f --ignore-unmatch Tools/xxx_rbn.info.sh
```

`.mcp.json` configures the internal `vvx` MCP server (`Tools/vvk/`), which is stripped. `wsl@rocket` is a BURS station regime config for the `wsl@rocket` ssh test host (operator-site-specific: `BURS_USER`/`BURS_TINCTURE`/`BURS_LOG_DIR`), sitting at the repo root — a site-specific config of the same kind as the `rbmn_nodes/`/`rbmu_users/` profiles stripped in 10b, withheld for the same reason.

**10f. Stage the consumer templates and marshal zero changes:**
```
git add CLAUDE.md
git add -u
```

After all removals, verify with `git ls-files` that no proprietary content remains. Show the user a summary of what was removed and what survives. **Pause for careful review.**

### What should survive after stripping:

- `rbmm_moorings/` — the consumer-config tree (replaces the former `.buk/` + `.rbk/` homes):
  - `burc.env` and the regime `.env` files (`rbrr.env`, `rbrd.env`, `rbrp.env`, `rbrw.env`) — already blanked by marshal zero in Step 5
  - `rbmf_foedera/` — the foedus library (holds the federation regime: `rbef_entrada/rbrf.env` and the committed `rbef_keycloak/rbrf.env.template`)
  - `rbml_launchers/` — only `launcher.buw_workbench.sh` and `launcher.rbw_workbench.sh` (the rest stripped in 10d)
  - `rbmv_vessels/` — vessel definitions and README (the former `rbev-vessels/`)
  - the per-nameplate dirs `ccyolo/ moriah/ nineveh/ pluml/ srjcl/ tadmor/ fdkyclk/` — **all ship**: README documents each as an example crucible with its own anchor, and the onboarding handbooks walk several. `srjcl/workspace/` (the Jupyter sample content) ships with it.
- `Tools/rbk/rbxk_cli.sh` + `rbxk_keycloak.sh`, `tt/rbw-qjK.*` / `rbw-qjQ.*` / `rbw-cC.Charge.fdkyclk.sh` / `rbw-cQ.Quench.fdkyclk.sh`, the `fdkyclk` nameplate + `rbev-bottle-fdkyclk` vessel + `rbef_keycloak` foedus template — the whole Keycloak synthetic-federation test facility **ships** as RB's federation test surface.
- `CLAUDE.md` — consumer version (copied in Step 9)
- `README.md` — consumer-facing, tracked directly at the repo root
- `LICENSE`
- `rbm-abstract-drawio.svg`
- `diagrams/` — the `rbdg*` PlantUML sources and rendered light/dark `.svg` pairs
- `Tools/buk/` — all `.sh` files, `busc_shellcheckrc`, `README.md`, `buts/` test support (minus `vov_veiled/`)
- `Tools/rbk/` — all `.sh` files (minus `vov_veiled/`), including the theurge crate `rbtd/`
- `tt/` — `rbw-*` and `buw-*` tabtargets only (minus `rbw-MZ`, `rbw-MP` marshal tabtargets and `rbw-mR` manor-raze)

**Do NOT strip the fdkyclk caged credentials.** `rbmm_moorings/fdkyclk/fdkyclk-asserter-key.pem` (a real `BEGIN PRIVATE KEY`) and `fdkyclk-client-secret.txt` look alarming and a secret scanner will flag them, but they are **intentional committed test scaffolding** — the caged asserter keypair whose public half is baked into the realm's `publicKeySignatureVerifier`, and the confidential-client secret matching that realm (RBSFK "two-keys"). Git-ignoring or stripping them breaks the test-bed's determinism; the realm expects exactly those. The security posture is handled by documentation, not removal — see the fdkyclk caution in `README.md`. The realm's *own* signing key (the id-tokens Keycloak issues) is a separate, ephemeral key minted per charge and committed nowhere; the live `rbef_keycloak/rbrf.env` that snapshots it is git-ignored and never ships.

## Step 11: Regenerate derived files

The strip removed tabtargets from `tt/`, and the committed tabtarget context (`Tools/rbk/claude-rbk-tabtarget-context.md`) is generated by enumerating `tt/` on disk — so it is now stale, and Step 12's freshness gate will reject it. Regenerate from the stripped tree:

```
tt/rbw-tb.Build.sh
```

This rewrites `claude-rbk-tabtarget-context.md` to reflect the surviving tabtarget set. (The Rust colophon consts `rbtdgc_consts.rs` are zipper-driven, not disk-driven, so they do not change — the withheld `rbw-MZ/MP/mR` colophons stay enrolled there, which is fine: Step 12's fast-qualify sweeps disk→registry, never registry→disk.) Stage the regenerated file:

```
git add Tools/rbk/claude-rbk-tabtarget-context.md
```

Show the user what regenerated and wait for acknowledgment.

## Step 12: Post-strip verification

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

Note what pyx does NOT cover: the known-vulnerability advisory audit. That verdict moves with a live advisory database while the tree stands still, so it cannot be a fixture. It stays here, as a step you own.

**Ceremony link check** — the standing step that makes the candidate's documentation links sound, in two halves:

1. **Anchor sweep** — pyx's README-anchor case (just run, above) proves file-to-file on the candidate tree that every anchor minted in `Tools/rbk/rbyc_common.sh` (the third argument of each `zrbyc_yk` call) is defined in the candidate `README.md` as a literal `<a id="…">`. A green pyx IS this half; no separate command.
2. **URL base** — every one of those links rides on `RBRR_PUBLIC_DOCS_URL`, which must equal the recorded delivery base from Step 9:

```
grep -F 'RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"' rbmm_moorings/rbrr.env
```

If the grep misses, Step 9's URL-set step was skipped or mistyped — fix `rbrr.env` and re-stage before committing.

**If any check fails, STOP.** A fast-qualification failure means something in the consumer-visible code depends on stripped content. A pyx failure means the candidate is not fit to publish. A link-check failure means consumers would land on the maintainer's private repo. Each is a real finding that must be investigated before proceeding.

Show the result and wait for user acknowledgment.

## Step 13: Generate commit

- Stage any remaining changes: `git add -u`
- Analyze all changes for a consolidated commit message
- Review `git log OPEN_SOURCE_UPSTREAM/main..main --oneline` to summarize what's included
- Create commit (no attribution footer — this is a release candidate)
- Show `git log -1 --stat`

## Step 14: Final review

Show the user:
- The commit stat summary
- Push instructions (from the clone): `git push OPEN_SOURCE_UPSTREAM candidate-NNN-R`
- Reminder: inspect the result on GitHub before merging to main
- Reminder: the proof clone is a throwaway — delete its directory once the candidate is pushed

**STOP** — user reviews and pushes manually.
