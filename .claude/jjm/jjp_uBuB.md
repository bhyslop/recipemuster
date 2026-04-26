## Context

Release qualification gap surfaced by the ₣A_ depot-regen heat. Every existing test tier (`rbw-tf` fast, `rbw-tr` release) tolerates accumulated depot state, so silent first-build assumptions slip through. Recent spooks — kludge-aware-charge-prereq (k-prefixed hallmarks at GAR), ZRBOB_PROJECT (compose project name without runtime prefix), rbob_charged_predicate prefix omission, reliquary integrity-broken on negative test — all share a property: they only manifest on first-build paths, and none were caught until live verification surfaced them.

This heat constructs `rbw-tP` QualifyPristine — a third tier that **refuses to run** unless marshal-zero state was just committed. The refusal is enforced by the test itself, not by ceremony or operator discipline.

## Design decisions

### Single operator prerequisite chain

Operator has exactly one prerequisite chain: confirm Payor health → marshal-zero → commit → run `rbw-tP`. Payor OAuth is the only credential the operator must have ready; everything else (governor, retriever, director SAs and their RBRA files) is **minted by the qualification itself**, not restored from backup.

### Entry contract — load-bearing

`rbw-tP` fails-fast unless ALL of the following hold:
- Working tree clean
- HEAD commit is a marshal-zero commit (detectable signature baked into `rblm_zero` per BBAAB)
- RBRR fields are blank (prefixes, depot project ID, etc.)
- No RBRA credential files present (governor, retriever, director — all deleted by marshal-zero, recreated by the qualification)
- No hallmark fields populated in nameplates
- No depot-scoped fields populated in vessel rbrv.env files

Operator cannot skip the prerequisite. This is the property that makes the tier catch the silent-first-build bug class by construction.

### Pristine-lifecycle fixture architecture

The marshal-zero gate AND the SA/depot lifecycle exercises live as cases inside one new theurge fixture (`pristine-lifecycle`):

| # | Case | Idempotent? | Cost |
|---|------|-------------|------|
| 1 | `marshal-zero-attestation` | N/A — gate | Free |
| 2 | `depot-lifecycle` | Yes (throwaway-named) | One GCP project per run (soft-delete graveyard accepted) |
| 3 | `governor-lifecycle` | Yes (throwaway-named) | Free |
| 4 | `retriever-lifecycle` | Yes (throwaway governor + retriever) | Free + light read probe |
| 5 | `director-lifecycle` | Yes (throwaway governor + director) | Light Cloud Build / GAR write probe |

Cases 4 and 5 each create their own throwaway governor (single-case independence is load-bearing); after the case, both throwaway SAs are forfeited. Cases 2-5 are idempotent against existing canonical state — they can run while a canonical depot/SA chain exists, against any state.

**Single-case run** (e.g., `tt/rbtd-s.SingleCase.pristine-lifecycle.sh depot-lifecycle`) skips case 1's gate and exercises just the named case. **Full-fixture run** goes through case 1 first; case 1 gates the rest.

### Canonical infrastructure setup is a separate phase

After the pristine-lifecycle fixture passes, `rbw-tP` proceeds to canonical infrastructure setup (BBAAC): mantle real governor + deploy real RBRA, charter real retriever + deploy RBRA, knight real director + deploy RBRA, levy canonical depot. This persistent state is what reliquary inscribe, hallmark builds, and crucible runs depend on.

Two depots created per pristine run: one throwaway (case 2 of fixture) + one canonical (BBAAC). Both quota-bearing.

### Failure mode contract

Mid-qualification failure means start-over-from-zero, not patch-and-continue. Documented explicitly in runbook to prevent the very accumulated-state bug class this tier exists to catch.

### Implementation patterns (locked during BBAAJ)

Durable choices binding all pristine-lifecycle case implementations (BBAAB, BBAAJ, BBAAK, and future BBAAC vessel-commit cases).

**Throwaway-prefix mechanism: edit `.rbk/rbrr.env` + git commit.** Cases 2-5 need non-blank `RBRR_CLOUD_PREFIX`/`RBRR_RUNTIME_PREFIX` to invoke depot/SA tabtargets. The fixture mimics the human workflow: open rbrr.env, edit, `git add` + `git commit`. After the fixture runs, HEAD has walked off marshal-zero — recovery is `rbw-MZ`, matching the "start-over-from-zero" failure mode above.

Rejected alternative — env-var override: `rbrr_cli.sh` kindle does `source rbrr_regime.sh; source ${RBBC_rbrr_file}; zrbrr_kindle; zrbrr_enforce` — sourcing rbrr.env overwrites any pre-set env, then enforce locks `readonly`. Pre-set env is silently discarded. Confirmed by tracing.

Rejected alternative — in-memory restore (RAII): mutates rbrr.env without committing, leaves a transient dirty-tree window where a crash or sibling tooling sees uncommitted state that isn't operator-authored. Edit-and-commit is honest.

**Idempotent install helper.** A shared function `rbtdrp_install_throwaway_prefixes` reads current rbrr.env values; if blank (post-marshal-zero), writes throwaway values and commits; if already throwaway, no-op. Cases 2-5 call this as their first step. One commit per fixture run, not per case.

**Tabtarget shell-out style.** Cases discover global tabtargets via `rbtdri_find_tabtarget_global` and invoke via raw `Command::new(&tt).current_dir(&root)` — same pattern as `rbtdrf_handbook.rs`. No BURV context plumbing for these tabtargets (they don't write fact files theurge needs to read). For imprint-channel tabtargets like `rbtd-ap` (cases 4-5), use `rbtdri_find_tabtarget` with the role as imprint.

**Section structure inside `RBTDRP_SECTIONS_PRISTINE_LIFECYCLE`.**
- §1 `pristine-lifecycle-gate` — case 1 only (BBAAB-landed)
- §2 `pristine-lifecycle-simple` — cases 2-3 (BBAAJ)
- §3 `pristine-lifecycle-credentialed` — cases 4-5 (BBAAK)

`rbtdrc_fixture_fail_fast` is true for pristine-lifecycle, so §1 failure short-circuits §2 and §3 in full-fixture mode. Single-case mode skips the gate.

**Vessel commits beyond BBAAJ/BBAAK.** As pristine-tier extends to BBAAC and beyond (reliquary inscribe / enshrine / ordain / kludge), additional commits in vessel `rbrv.env` files and similar regime artifacts use the same edit-and-commit shape. The qualification's commit trail is the operator's audit trail.

**Git shell-out style.** Mirror jjx/vvx — `Command::new("git").args([...]).current_dir(&root).output()`. Stage with explicit file path (never `git add -A`); commit messages produced via Rust string formatting. No `--no-verify`, no `--amend`.

### Tier layering

Three tiers, escalating cost:

| Colophon | Frontispiece | Cost | Entry contract |
|----------|--------------|------|----------------|
| `rbw-tf` | QualifyFast | seconds | none |
| `rbw-tr` | QualifyRelease | minutes | none (accumulated state OK) |
| `rbw-tP` | QualifyPristine *(new)* | ~1 hour + cloud $ | marshal-zero just committed |

`rbw-tP` is THE release gate. `rbw-tr` becomes the pre-pristine smoke test — cheap-and-frequent for development confidence; pristine for actual release.

### Operator scope

Single operator (project lead). Not designed for multi-operator workflow. Runbook lives in README.md release section (5 human steps).

### Coupling to ₣A_

This heat runs concurrently with ₣A_'s remaining paces. ₣A_'s AAE+AAF+AAG sequence is the one-time cutover that informs `rbw-tP`'s sequence; pristine-tier construction proceeds in parallel and codifies the lessons. BBAAG's first end-to-end run is the natural validation point for both heats together.

## References

- ₣A_ rbk-mvp-3-resource-prefix-and-depot-regen — surfaced the gap; AAE+AAF+AAG one-time burn-in informs but doesn't codify
- `Tools/rbk/rblm_cli.sh` — RBLM Lifecycle Marshal; `rbw-MZ` zeroes local regime, deletes all RBRAs, blanks hallmark/depot-scoped vessel fields. Marshal-zero signature is baked here per BBAAB.
- `tt/rbw-tr.QualifyRelease.sh` — current release qualify; layered alongside, not replaced
- `Tools/rbk/rbh0/rbhocd_credential_director.sh` / `rbhocr_credential_retriever.sh` — handbook tracks describing director "build access" and retriever "pull/tally access" — operations the lifecycle cases 4 and 5 must verify
- `.claude/commands/rbk-prep-release.md` — upstream contribution ceremony; pristine-pass becomes a precondition (BBAAH)
- Recent spook commits informing the bug class: kludge-aware-charge-prereq (₢BAAAH), ZRBOB_PROJECT extraction (f7146d1d), rbob_charged_predicate fix (a8eb7311), reliquary integrity-broken negative-test residue (recent ₣BA verification)
- `Tools/rbk/rbtd/src/rbtdrf_handbook.rs` — non-context tabtarget shell-out reference pattern for case implementations
- `Tools/rbk/rbrr_cli.sh` — kindle architecture documented in Implementation patterns; sourcing-over-env behavior locked the throwaway mechanism choice