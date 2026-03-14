---
description: Prepare release candidate for upstream delivery
---

You are preparing a release candidate branch for upstream delivery to OPEN_SOURCE_UPSTREAM.

**This is an interactive ceremony.** Present each step, show output, and wait for user acknowledgment before proceeding to the next step. Do not race ahead.

**Important:**
- Be methodical — show output at each step
- Stop immediately on errors
- User maintains control throughout
- All destructive transforms happen on a candidate branch, never on develop or main

## Step 0: Request permissions

Ask the user for permission to execute all git operations needed (checkout, branch creation, squash merge, file removal, commit). Get explicit approval before proceeding.

## Step 1: Verify develop is clean and pushed

- Check `git status` on develop branch — must be clean
- Verify develop is pushed to origin
- If dirty or unpushed, **STOP** and ask user to resolve

## Step 2: Pre-strip qualification

Run full release qualification on develop to verify the complete codebase is healthy before any transforms:

```
tt/rbw-QR.QualifyRelease.sh
```

If qualification fails, **STOP**. The full codebase must pass before we proceed.

Show the qualification result and wait for user acknowledgment.

## Step 3: Regime variable completeness check (LLM task)

Before stripping removes the spec documents, verify every enrolled RBK regime variable has spec treatment. For each regime, read the enrollment file and corresponding spec:

| Regime file | Spec document |
|-------------|---------------|
| `Tools/rbk/rbrr_regime.sh` | `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` |
| `Tools/rbk/rbra_regime.sh` | `Tools/rbk/vov_veiled/RBSRA-CredentialFormat.adoc` |
| `Tools/rbk/rbrn_regime.sh` | `Tools/rbk/vov_veiled/RBRN-RegimeNameplate.adoc` |
| `Tools/rbk/rbrp_regime.sh` | `Tools/rbk/vov_veiled/RBSRP-RegimePayor.adoc` |
| `Tools/rbk/rbrv_regime.sh` | `Tools/rbk/vov_veiled/RBSRV-RegimeVessel.adoc` |
| `Tools/rbk/rbro_regime.sh` | `Tools/rbk/vov_veiled/RBSRO-RegimeOauth.adoc` |
| `Tools/rbk/rbrg_regime.sh` | `Tools/rbk/vov_veiled/RBSRG-RegimeGcbPins.adoc` |
| `Tools/rbk/rbrs_regime.sh` | `Tools/rbk/vov_veiled/RBSRS-RegimeStation.adoc` |

For each regime:
1. Extract all enrolled variable names (look for `buv_*_enroll` calls — first argument is the variable name)
2. Grep for each variable name in the corresponding spec document
3. Report any variables missing from their spec

Present results to the user. Gaps are informational — they don't block the ceremony but should be catalogued for future work.

Wait for user acknowledgment.

## Step 4: Sync main with upstream

- `git checkout main`
- `git fetch OPEN_SOURCE_UPSTREAM`
- `git pull OPEN_SOURCE_UPSTREAM main`
- If pull fails, **ABORT** and ask user to resolve
- `git push origin main`

Show results and wait for user acknowledgment.

## Step 5: Auto-detect next candidate branch

- Find max batch from upstream: `git ls-remote --heads OPEN_SOURCE_UPSTREAM | grep 'candidate-'`
- Find max batch from local branches
- If batch exists upstream: new batch (previous was merged)
- If batch not upstream: redo (increment revision)
- Tell user which branch name was chosen and why

Wait for user approval of the branch name.

## Step 6: Create candidate branch and squash merge

- `git checkout -b candidate-NNN-R main`
- Show commits that will be included: `git log main..develop --oneline`
- Execute: `git merge --squash develop`
- Show the merge result summary

Wait for user acknowledgment.

## Step 7: Extract consumer templates

The consumer templates live in `vov_veiled/` which will be stripped in the next step. Extract them now:

```
cp Tools/rbk/vov_veiled/CLAUDE.consumer.md CLAUDE.md
cp Tools/rbk/vov_veiled/README.consumer.md README.md
```

Note: `CLAUDE.md` is overwritten (replacing the development version). The existing `readme.md` (lowercase) will be removed in the strip step; `README.md` (uppercase) replaces it.

Show the user what was copied. Wait for acknowledgment.

## Step 8: Strip proprietary content

Remove all proprietary content from the candidate branch. Present the full strip plan to the user, then execute after approval.

**8a. Recursive glob — all vov_veiled directories:**
```
git rm -rf --ignore-unmatch Tools/buk/vov_veiled/
git rm -rf --ignore-unmatch Tools/cmk/vov_veiled/
git rm -rf --ignore-unmatch Tools/gad/vov_veiled/
git rm -rf --ignore-unmatch Tools/jjk/vov_veiled/
git rm -rf --ignore-unmatch Tools/rbk/vov_veiled/
git rm -rf --ignore-unmatch Tools/vok/vov_veiled/
git rm -rf --ignore-unmatch Tools/vvk/vov_veiled/
```

**8b. Whole directories — internal tools and infrastructure:**
```
git rm -rf --ignore-unmatch .claude/
git rm -rf --ignore-unmatch lenses/
git rm -rf --ignore-unmatch Memos/
git rm -rf --ignore-unmatch Study/
git rm -rf --ignore-unmatch _slickedit/
git rm -rf --ignore-unmatch RBM-nameplates/
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
```

**8c. Internal tabtargets (non-rbw, non-buw operational targets):**
```
git rm -f --ignore-unmatch tt/butctt.TestTarget.sh
git rm -f --ignore-unmatch tt/ccck-s.ConnectShell.sh
git rm -f --ignore-unmatch tt/study-mpt.Run.*.sh
git rm -f --ignore-unmatch tt/vow-*.sh
git rm -f --ignore-unmatch tt/vslk-*.sh
git rm -f --ignore-unmatch tt/vvw-*.sh
```

**8d. Internal .buk/ launchers (for stripped workbenches):**
```
git rm -f --ignore-unmatch .buk/launcher.cccw_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.cmw_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.jjw_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.study_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.vow_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.vslw_workbench.sh
git rm -f --ignore-unmatch .buk/launcher.vvw_workbench.sh
```

**8e. Individual files:**
```
git rm -f --ignore-unmatch podman-gateway-proposal.md
git rm -f --ignore-unmatch brm_recipemuster.iml
git rm -f --ignore-unmatch MBS.STATION-reference.sh
git rm -f --ignore-unmatch readme.md
```

**8f. Stage the consumer templates** (copied in step 7, replacing stripped originals):
```
git add CLAUDE.md README.md
```

After all removals, verify with `git ls-files` that no proprietary content remains. Show the user a summary of what was removed and what survives. **Pause for careful review.**

### What should survive after stripping:

- `.buk/` — `burc.env`, `rbbc_constants.sh`, `launcher.buw_workbench.sh`, `launcher.rbw_workbench.sh`
- `.rbk/` — all regime `.env` files (will be blanked by marshal reset later)
- `CLAUDE.md` — consumer version (copied in step 7)
- `README.md` — consumer version (copied in step 7)
- `LICENSE`
- `.nojekyll`
- `index.html`
- `rbm-abstract-drawio.svg`
- `rbev-vessels/` — vessel definitions and README
- `Tools/buk/` — all `.sh` files, `busc_shellcheckrc`, `README.md`, `buts/` test support (minus `vov_veiled/`)
- `Tools/rbk/` — all `.sh` files (minus `vov_veiled/`)
- `tt/` — `rbw-*` and `buw-*` tabtargets only

## Step 9: Post-strip qualification

Run release qualification on the stripped candidate tree:

```
tt/rbw-QR.QualifyRelease.sh
```

This validates that the stripped repo still works — tabtargets resolve, colophons match modules, nameplate health, shellcheck passes on surviving scripts, and the test suite runs.

**If qualification fails, STOP.** This means something in the consumer-visible code depends on stripped content. Report the specific failure to the user — this is a real finding that must be investigated before proceeding.

Show the qualification result and wait for user acknowledgment.

## Step 10: Marshal reset

Run the marshal reset command to blank site-specific configuration:

```
tt/rbw-MR.MarshalReset.sh
```

This will:
- Blank site-specific RBRR fields (depot project ID, GAR repository, connection name, worker pool, rubric URL)
- Pre-fill RBRR defaults (DNS server, machine type, timeout, region, vessel dir, secrets dir)
- Delete depot-scoped credential files (governor, director, retriever)
- Blank consecration values in all nameplate files

The command will prompt for confirmation — the user must type `reset` to proceed.

After the reset, show `git status` to confirm the changes. Wait for user acknowledgment.

## Step 11: Final cleanup

Remove any remaining uncatalogued files that should not ship:

```
git rm -f --ignore-unmatch podman-gateway-proposal.md
```

(This is a safety catch — it may already have been removed in step 8e.)

Stage all marshal reset changes:
```
git add -u
```

## Step 12: Generate commit

- Analyze all changes for a consolidated commit message
- Filter out commits that only touched stripped files
- Create commit (no attribution footer — this is a release candidate)
- Show `git log -1 --stat`

## Step 13: Final review

Show the user:
- The commit stat summary
- Push instructions: `git push OPEN_SOURCE_UPSTREAM candidate-NNN-R`
- Reminder: inspect the result on GitHub before merging to main

**STOP** — user reviews and pushes manually.
