# Heat Trophy: rbk-14-mvp-testing-simplification

**Firemark:** ₣BJ
**Created:** 260512
**Retired:** 260529
**Status:** retired

## Paddock

## Paddock: rbk-14-mvp-testing-simplification

## Shape

Theurge — the crucible test orchestrator — should be a clean *consumer* of
operator tabtargets, never an owner of commands and never a reacher into module
internals. This heat closes the two places where it still violates that.

**1. Access-probe relocation.** Theurge currently owns the `rbtd-ap` colophon
family (the credential access probes), the only colophon it owns. This work
retires that family into four operator credential-check tabtargets
(`rbw-acg/acr/acd/acp`) dispatched by `rbw_workbench`, leaving theurge owning
zero colophons. The `rbgv_AccessProbe.sh` implementation is unchanged — only
ownership and dispatch move. Mechanical; the docket carries the full tabtarget
mapping and done-when.

**2. Contract-surface migration.** Theurge's fast-suite validation cases
bypass tabtargets — hardcoding module source paths, replicating kindle chains,
calling `z`-private functions. This work moves them onto contract surfaces.

## Locked decisions / findings (from grooming survey, 260520)

- **Sequencing behind ₣BK.** Both work items sit inside ₣BK's moorings-cutover
  blast radius (the same `rbtd-ap` tabtargets, `rbtdrp_pristine.rs`, and
  `rbtdrf_fast.rs` that ₣BK's filesystem-rename and literal-sweep touch). This
  heat mounts only *after* ₣BK lands. Doing it after — not concurrently —
  removes all concurrent-edit risk; and both work items shrink ₣BK's surface
  (the access-probe work deletes four tabtargets; the migration work strips the
  hardcoded `.rbk/` path literals that ₣BK's literal-sweep names as its
  canonical example). Operator holds this schedule.

- **The trampoline form is settled.** ₣BK's tabtarget trampoline already landed.
  New operator tabtargets are the one-line `exec .../z-launcher.sh rbml_rbw
  "${0##*/}" "${@}"` form — they carry no `.buk`/`.rbk` literal and are immune
  to ₣BK's later launcher-path flip.

- **The migration is one file.** `rbtdrf_fast.rs` is the sole bypass-heavy
  surface; the sibling theurge test files already route through tabtargets. The
  `rbtdrf_run_tt` / `rbtdrf_run_tt_neg` helpers already exist. regime-smoke and
  dockerfile-hygiene families already use tabtargets.

- **The work splits by family, not by pace.** regime-validation is the real
  migration target (candidate surfaces `rbw-rrv/rvv/rnv`). enrollment-validation
  is pure-utility (BUV type checks via public `buv_vet`) — leave it, do not
  invent a test-only tabtarget for it. Kept as a single study-gated pace;
  decompose only if the crux below forces invented test-API.

- **Pivotal unknown (settle first at mount).** Whether a validate tabtarget can
  be pointed at a *staged malformed regime* decides whether regime-validation
  negatives are tabtarget-coverable (mechanical) or need a declared
  `*_t_validate` entry point (a design sub-task).

- **Reliability spine.** A pre-migration verdict baseline (per-case Pass/Fail
  across the fast suite) is the safety net — the post-migration verdict set must
  match exactly, and migrated negatives must fail for the *same reason*, not
  merely fail.

## Done looks like

Theurge owns no colophons; the four credential checks are operator tabtargets.
`rbtdrf_fast.rs`'s regime-validation family exercises tabtargets (or declared
test entry points), with no hardcoded module paths or `z`-private reach. Fast,
complete, and gauntlet suites pass with verdicts matching the pre-migration
baseline.

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — migration surface
- `Tools/rbk/rbgv_AccessProbe.sh` — access-probe implementation (unchanged)
- `Tools/rbk/rbtd/rbte_cli.sh`, `rbte_engine.sh` — theurge colophon ownership to strip
- `Tools/rbk/rbz_zipper.sh`, `rbw_workbench.sh` — operator dispatch enrollment
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — fixture-to-colophon dependency tables
- `rbtdri_invocation.rs` header — "theurge invokes bottle operations exclusively through tabtargets"

## Paces

### access-probe-to-credential-check (₢BJAAA) [complete]

**[260521-0223] complete**

## Character
Mechanical refactor with one architectural correction — relocate dispatch out of theurge.

## Goal
Retire the rbtd-ap.AccessProbe.* tabtarget family. Replace with four rbw-ac[grdp].Check{Role}Credential.sh tabtargets dispatched by rbw_workbench. Theurge becomes a pure consumer (subprocess) of these tabtargets, matching its pattern for every other entry in rbtdrm_manifest.rs.

## Tabtarget mapping
| Old | New |
|---|---|
| tt/rbtd-ap.AccessProbe.governor.sh  | tt/rbw-acg.CheckGovernorCredential.sh |
| tt/rbtd-ap.AccessProbe.retriever.sh | tt/rbw-acr.CheckRetrieverCredential.sh |
| tt/rbtd-ap.AccessProbe.director.sh  | tt/rbw-acd.CheckDirectorCredential.sh |
| tt/rbtd-ap.AccessProbe.payor.sh     | tt/rbw-acp.CheckPayorCredential.sh |

## Shape change
Was: one colophon (rbtd-ap), role-as-imprint, dispatched via rbtw_workbench. Now: four colophons under rbw-ac*, no imprint, dispatched via rbw_workbench. The rbgv_AccessProbe.sh implementation file is unchanged in location and contents.

## Decoupling
Dispatch leaves Tools/rbk/rbtd/rbte_cli.sh and rbte_engine.sh. Theurge's owned-colophon machinery (ZRBTE_COLOPHONS and ZRBTE_FULL_MANIFEST) collapses — theurge no longer owns any colophons. Per the existing header in rbtdrm_manifest.rs ("Each names the bash tabtarget colophon theurge invokes for that operation"), the manifest becomes uniform with every entry rbw-*.

## Done when
- `ls tt/rbtd-ap.*` empty; `ls tt/rbw-ac*.sh` lists four files enrolled in rbz_zipper.sh
- rbw_workbench sources rbgv_AccessProbe.sh and dispatches the four new colophons
- rbte_cli.sh and rbte_engine.sh contain no rbtd-ap or rbgv_ references
- rbtdrm_manifest.rs's ACCESS_PROBE constant is replaced (either one role→colophon helper or four constants); fixture-to-colophon dependency tables reflect the new shape
- Three theurge fixtures — canonical-establish, pristine-lifecycle, and the access-probe fixture under crucible — invoke the new colophons through the existing per-role wrappers
- Test scaffolds in rbtdti_invocation.rs reference the new filenames
- Fast, complete, and gauntlet suites pass

## Not in scope
- Renaming the rbgv_AccessProbe.sh implementation file (separate decision)
- Reorganizing other rbtd-* tabtargets (suites, fixture runner, theurge build/test, etc.)
- CLAUDE.md suite-table documentation rot
- Introducing a buc_run_tabtarget helper (premature for a single-site need)

**[260512-1440] rough**

## Character
Mechanical refactor with one architectural correction — relocate dispatch out of theurge.

## Goal
Retire the rbtd-ap.AccessProbe.* tabtarget family. Replace with four rbw-ac[grdp].Check{Role}Credential.sh tabtargets dispatched by rbw_workbench. Theurge becomes a pure consumer (subprocess) of these tabtargets, matching its pattern for every other entry in rbtdrm_manifest.rs.

## Tabtarget mapping
| Old | New |
|---|---|
| tt/rbtd-ap.AccessProbe.governor.sh  | tt/rbw-acg.CheckGovernorCredential.sh |
| tt/rbtd-ap.AccessProbe.retriever.sh | tt/rbw-acr.CheckRetrieverCredential.sh |
| tt/rbtd-ap.AccessProbe.director.sh  | tt/rbw-acd.CheckDirectorCredential.sh |
| tt/rbtd-ap.AccessProbe.payor.sh     | tt/rbw-acp.CheckPayorCredential.sh |

## Shape change
Was: one colophon (rbtd-ap), role-as-imprint, dispatched via rbtw_workbench. Now: four colophons under rbw-ac*, no imprint, dispatched via rbw_workbench. The rbgv_AccessProbe.sh implementation file is unchanged in location and contents.

## Decoupling
Dispatch leaves Tools/rbk/rbtd/rbte_cli.sh and rbte_engine.sh. Theurge's owned-colophon machinery (ZRBTE_COLOPHONS and ZRBTE_FULL_MANIFEST) collapses — theurge no longer owns any colophons. Per the existing header in rbtdrm_manifest.rs ("Each names the bash tabtarget colophon theurge invokes for that operation"), the manifest becomes uniform with every entry rbw-*.

## Done when
- `ls tt/rbtd-ap.*` empty; `ls tt/rbw-ac*.sh` lists four files enrolled in rbz_zipper.sh
- rbw_workbench sources rbgv_AccessProbe.sh and dispatches the four new colophons
- rbte_cli.sh and rbte_engine.sh contain no rbtd-ap or rbgv_ references
- rbtdrm_manifest.rs's ACCESS_PROBE constant is replaced (either one role→colophon helper or four constants); fixture-to-colophon dependency tables reflect the new shape
- Three theurge fixtures — canonical-establish, pristine-lifecycle, and the access-probe fixture under crucible — invoke the new colophons through the existing per-role wrappers
- Test scaffolds in rbtdti_invocation.rs reference the new filenames
- Fast, complete, and gauntlet suites pass

## Not in scope
- Renaming the rbgv_AccessProbe.sh implementation file (separate decision)
- Reorganizing other rbtd-* tabtargets (suites, fixture runner, theurge build/test, etc.)
- CLAUDE.md suite-table documentation rot
- Introducing a buc_run_tabtarget helper (premature for a single-site need)

### theurge-contract-surface-migration (₢BJAAB) [complete]

**[260520-2050] complete**

## Character

Study-gated migration. One file. A cheap spike decides the repair shape; do
not commit to a form before it. The reliability spine is a verdict baseline,
not the pace boundary.

## Goal

Theurge's fast-suite cases in `Tools/rbk/rbtd/src/rbtdrf_fast.rs` exercise
contract surfaces, not module internals. The bypass — hardcoded module source
paths, replicated kindle chains, direct `z*` calls — is gone from the
regime-validation family. Every case's verdict is unchanged.

## Starting facts (from the ₣BJ survey — do not re-derive)

- `rbtdrf_fast.rs` is the sole bypass-heavy file. `rbtdrk_canonical.rs`,
  `rbtdro_onboarding.rs`, `rbtdrp_pristine.rs`, `rbtdrf_handbook.rs` already
  route through tabtargets.
- The tabtarget-invocation helpers `rbtdrf_run_tt` and `rbtdrf_run_tt_neg`
  already exist in `rbtdrf_fast.rs` — no minting needed.
- The regime-smoke and dockerfile-hygiene families already use tabtargets — no
  work there.
- Candidate surfaces for regime-validation: `rbw-rrv`, `rbw-rvv`, `rbw-rnv`.

## Step gate — settle this first

The regime-validation negatives feed deliberately-bad regime state. Determine
whether a validate tabtarget can be pointed at a *staged malformed regime*
(injectable regime dir / moorings-location override) so a negative runs through
`rbtdrf_run_tt_neg`.

- **Yes** → regime-validation is tabtarget-coverable; migration is mechanical.
  The RBRR / RBRV / RBRN negatives break into independent commits — notch each
  sub-family separately, baseline-diffing after each.
- **No** → these need a declared public/test entry point (sketch
  `<prefix>_t_validate <regime>`), NOT a test-only tabtarget. If this path is
  forced and the work is substantial, cantle it out.

## enrollment-validation disposition

Pure-utility — BUV type checks via the public `buv_vet`, which is itself a
legitimate contract surface. No natural tabtarget; do NOT invent one. The only
mess is path-coupling and any `z*` reach (e.g. enrollment reset). Replace those
with a stable reference / declared entry where warranted; confirm-and-leave is
an acceptable outcome.

## Reliability discipline

- Capture a verdict baseline (run the fast suite, record per-case Pass/Fail)
  BEFORE any edit. The post-migration verdict set must match exactly.
- A migrated negative must fail for the SAME reason, not merely fail.
  Spot-check the diagnostic, not just the exit status.
- Refactor-safety proof: temporarily move or rename a sourced module; migrated
  cases must still pass, or fail with a contract-level diagnostic — never "no
  such file."

## Out of scope

- New theurge cases. Migration only.
- New tabtargets purely for testing.
- BUK testbenches in `Tools/buk/`.
- Engine-helper redesign beyond what migration demands.

## References

- `rbtdri_invocation.rs` header — documented principle "theurge invokes bottle
  operations exclusively through tabtargets."
- ₢BBABJ design conversation — original mess characterization and principles.

**[260520-0844] rough**

## Character

Study-gated migration. One file. A cheap spike decides the repair shape; do
not commit to a form before it. The reliability spine is a verdict baseline,
not the pace boundary.

## Goal

Theurge's fast-suite cases in `Tools/rbk/rbtd/src/rbtdrf_fast.rs` exercise
contract surfaces, not module internals. The bypass — hardcoded module source
paths, replicated kindle chains, direct `z*` calls — is gone from the
regime-validation family. Every case's verdict is unchanged.

## Starting facts (from the ₣BJ survey — do not re-derive)

- `rbtdrf_fast.rs` is the sole bypass-heavy file. `rbtdrk_canonical.rs`,
  `rbtdro_onboarding.rs`, `rbtdrp_pristine.rs`, `rbtdrf_handbook.rs` already
  route through tabtargets.
- The tabtarget-invocation helpers `rbtdrf_run_tt` and `rbtdrf_run_tt_neg`
  already exist in `rbtdrf_fast.rs` — no minting needed.
- The regime-smoke and dockerfile-hygiene families already use tabtargets — no
  work there.
- Candidate surfaces for regime-validation: `rbw-rrv`, `rbw-rvv`, `rbw-rnv`.

## Step gate — settle this first

The regime-validation negatives feed deliberately-bad regime state. Determine
whether a validate tabtarget can be pointed at a *staged malformed regime*
(injectable regime dir / moorings-location override) so a negative runs through
`rbtdrf_run_tt_neg`.

- **Yes** → regime-validation is tabtarget-coverable; migration is mechanical.
  The RBRR / RBRV / RBRN negatives break into independent commits — notch each
  sub-family separately, baseline-diffing after each.
- **No** → these need a declared public/test entry point (sketch
  `<prefix>_t_validate <regime>`), NOT a test-only tabtarget. If this path is
  forced and the work is substantial, cantle it out.

## enrollment-validation disposition

Pure-utility — BUV type checks via the public `buv_vet`, which is itself a
legitimate contract surface. No natural tabtarget; do NOT invent one. The only
mess is path-coupling and any `z*` reach (e.g. enrollment reset). Replace those
with a stable reference / declared entry where warranted; confirm-and-leave is
an acceptable outcome.

## Reliability discipline

- Capture a verdict baseline (run the fast suite, record per-case Pass/Fail)
  BEFORE any edit. The post-migration verdict set must match exactly.
- A migrated negative must fail for the SAME reason, not merely fail.
  Spot-check the diagnostic, not just the exit status.
- Refactor-safety proof: temporarily move or rename a sourced module; migrated
  cases must still pass, or fail with a contract-level diagnostic — never "no
  such file."

## Out of scope

- New theurge cases. Migration only.
- New tabtargets purely for testing.
- BUK testbenches in `Tools/buk/`.
- Engine-helper redesign beyond what migration demands.

## References

- `rbtdri_invocation.rs` header — documented principle "theurge invokes bottle
  operations exclusively through tabtargets."
- ₢BBABJ design conversation — original mess characterization and principles.

**[260514-1234] rough**

## Character

Design conversation followed by careful migration. Theurge's enrollment-validation (~47 cases) and regime-validation (~27 cases) in `Tools/rbk/rbtd/src/rbtdrf_fast.rs` currently bypass tabtargets and exercise modules via direct source-and-call — encoding source paths, replicating kindle chains, and reaching into `z`-prefixed internals. Identified during BBABJ as legacy mess; this pace surveys, categorizes, and repairs.

Don't dictate repair shape at slate-time. Study first, let analysis inform the repair plan, execute. May decompose into multiple cantled paces if the work breaks naturally into independent migrations.

## Scope

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — primary surface, 74+ cases in enrollment-validation and regime-validation families plus the two all-* cases (`rbtdrf_rv_rbrv_all_vessels`, `rbtdrf_rv_rbrn_all_nameplates`).
- Sibling files in `Tools/rbk/rbtd/src/` following the same bypass pattern — verify at mount-time which apply: `rbtdrk_canonical.rs`, `rbtdro_onboarding.rs`, `rbtdrp_pristine.rs`, `rbtdrf_handbook.rs`.
- Engine helpers (`rbtdrf_run_ev`, `rbtdrf_run_rv`, `rbtdrf_run_bash`) — touched only if repair shape demands.

## Principles (from BBABJ design conversation — context for future minter)

- **Theurge tests should exercise contract surfaces, not module internals.** Where an operator tabtarget covers the operation, theurge invokes the tabtarget via `rbtdrf_run_tt` (or `rbtdrf_run_tt_neg` for expect-fail, minted in `₢BBABJ`).
- **No new tabtargets purely for testing.** Tabtargets are operator surface; adding test-only colophons pollutes that surface.
- **For sub-workflows or pure-library tests without natural tabtarget coverage**: declared test entry points in the relevant `*_cli.sh` module are preferable to reaching into `z`-internals. Naming sketch (not locked): `<prefix>_t_<op>` (`_t_` for theurge-target). Convention locks at first concrete use.
- **The mess being repaired**:
  - **Path coupling** — theurge encodes module source paths (`Tools/rbk/rbrr_regime.sh`, etc.) as string literals across many `format!` blocks. Module moves break theurge silently.
  - **Kindle-chain coupling** — theurge replicates source-and-kindle ordering that also lives in every `*_cli.sh`. Two sources of truth for module wakeup; BBABI's `zrbfh_kindle` addition demonstrated the drift surface.
  - **Private-function reach** — `z`-prefixed functions are conventionally private, yet theurge routinely calls `z*_kindle`. The convention has an undocumented exception ("private except for theurge tests"); the public/private boundary is fuzzy.
  - **Refactor risk** — internal restructure can break tests that reach inside; "what's testable from outside" is implicit, defined by accident of which functions happen to be public-named.

## Suggested approach (mount-time judgment)

1. **Survey**: categorize each in-scope case into one of:
   - **Tabtarget-coverable** — existing operator tabtarget serves as the test surface; migration is mechanical (source-and-call → `rbtdrf_run_tt`).
   - **Test-API-required** — case tests something without natural tabtarget coverage; needs a declared `*_t_*` entry point in a cli or library module.
   - **Pure-utility** — case tests a self-contained library function (e.g., BUV type checks via `buv_vet`) where direct sourcing of a public function is the cleanest shape; the mess is bounded and migration may not earn its keep.
2. **Plan**: decide whether to execute migration inline or decompose into cantled paces by category. Cantle if categories suggest independent effort sizes.
3. **Execute**: preserve case behavior — verdicts before and after must match across all 74+ cases.

## Verification

- `tt/rbtd-s.TestSuite.fast.sh` green at completion (case count preserved; verdicts match pre-migration baseline).
- Spot-check refactor safety claim: temporarily move or rename a module file, confirm theurge still passes (or fails with a contract-level diagnostic, not a "no such file" error).
- If new `*_t_*` entry points minted: their addition is reflected in CLAUDE.md acronym mappings (post-BBABK relocation, this may live in per-kit context files).

## Out of scope

- Adding new theurge cases. Migration-only.
- Bash testbenches in `Tools/buk/` (separate codepath, separate concern).
- User-facing tabtarget changes — no new ones, existing ones unchanged.
- Locking `*_t_*` as a universal naming policy — the first concrete use locks it.
- Engine-helper redesign beyond what migration demands.

## References

- ₢BBABJ design conversation — context for the mess characterization and proposed principles.
- `rbtdri_invocation.rs:19-20` — documented principle "theurge invokes bottle operations exclusively through tabtargets" (currently observed only for crucible-shaped ops; this pace extends to enrollment/regime).

## Silks

`theurge-contract-surface-migration` — destination shape. The work is "migrating theurge testing to contract surfaces (tabtargets where natural, declared test APIs otherwise)" with study preceding execution.

### rv-negative-test-api (₢BJAAD) [complete]

**[260522-1024] complete**

## Character

Design settled during slate dialogue — implementation is mechanical: mint one
public function per regime, migrate the negatives uniformly. No design judgment
remains.

## Goal

Migrate the regime-validation *negative* cases in `rbtdrf_fast.rs` off their
internal-reach bypass (z-private kindle/enforce, replicated kindle chains,
hardcoded module source paths) onto a public per-regime validation function.
Verdicts unchanged; each negative fails for the same diagnostic.

## Locked decisions (settled at slate — do not re-derive)

- **Entry is a public function, NOT a tabtarget.** Mint `rbr{r,d,v,n}_probate
  <file>` — sources the given file, runs the kindle→enforce chain,
  fails-on-first-fault. Theurge calls it directly (the blessed `buv_vet`
  precedent that enrollment-validation already uses), not via subprocess. The
  earlier step-gate "must be a tabtarget" framing is retired: the operator
  hand-edit-validation need is already served by the existing `rbw-r?v`
  canonical-file tabtargets, so the arbitrary-file capability is test-facing
  and stays off the operator surface — no new colophons/tabtargets, no zipper
  enrollment, no context regen.

- **Negatives use synthetic-baseline-to-temp, uniform across all four
  regimes.** Each declares a valid synthetic baseline + one violating override,
  writes it to the case temp dir, drives `*_probate` against it. This is the
  RBRV/RBRN baseline pattern already in the file; extend it to RBRR/RBRD. No
  `read_env_value`/`replace_env_fields`, no hoist, no touch to
  `rbtdrk_canonical.rs` / `rbtdrp_pristine.rs`.

- **Recategorize.** The `rbtdrf_rv_rbrr_*` block splits into true RBRR negatives
  and RBRD negatives (the `RBRD_DEPOT_MONIKER` and `RBRD_CLOUD_PREFIX` cases),
  per the now-finished RBRD-from-RBRR split. Fixture case count unchanged.

- **Split already landed in this pace.** `rbrr_cli.sh` furnish was stripped of
  the dead RBRD/RBDC coupling (commit 47e02720a). Do not redo.

## Reliability

Re-capture the fast baseline (currently absent at `/tmp/bjaab-fast-baseline.md`)
before migrating. Post-migration verdict set matches exactly; spot-check that
each negative fails for the SAME diagnostic, not merely non-zero.

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — `rbtdrf_run_rv` + the negative cases;
  the RBRV/RBRN baselines are the pattern to extend
- `Tools/rbk/rbr{r,d,v,n}_regime.sh` — the `z*_kindle`/`z*_enforce` the probate
  fn wraps; `rbr?_cli.sh` furnish shows the source→kindle→enforce sequence
- enrollment-validation cases (`rbtdrf_run_ev`) — the public-function-call
  precedent that blesses calling a contract surface directly

**[260522-0710] rough**

## Character

Design settled during slate dialogue — implementation is mechanical: mint one
public function per regime, migrate the negatives uniformly. No design judgment
remains.

## Goal

Migrate the regime-validation *negative* cases in `rbtdrf_fast.rs` off their
internal-reach bypass (z-private kindle/enforce, replicated kindle chains,
hardcoded module source paths) onto a public per-regime validation function.
Verdicts unchanged; each negative fails for the same diagnostic.

## Locked decisions (settled at slate — do not re-derive)

- **Entry is a public function, NOT a tabtarget.** Mint `rbr{r,d,v,n}_probate
  <file>` — sources the given file, runs the kindle→enforce chain,
  fails-on-first-fault. Theurge calls it directly (the blessed `buv_vet`
  precedent that enrollment-validation already uses), not via subprocess. The
  earlier step-gate "must be a tabtarget" framing is retired: the operator
  hand-edit-validation need is already served by the existing `rbw-r?v`
  canonical-file tabtargets, so the arbitrary-file capability is test-facing
  and stays off the operator surface — no new colophons/tabtargets, no zipper
  enrollment, no context regen.

- **Negatives use synthetic-baseline-to-temp, uniform across all four
  regimes.** Each declares a valid synthetic baseline + one violating override,
  writes it to the case temp dir, drives `*_probate` against it. This is the
  RBRV/RBRN baseline pattern already in the file; extend it to RBRR/RBRD. No
  `read_env_value`/`replace_env_fields`, no hoist, no touch to
  `rbtdrk_canonical.rs` / `rbtdrp_pristine.rs`.

- **Recategorize.** The `rbtdrf_rv_rbrr_*` block splits into true RBRR negatives
  and RBRD negatives (the `RBRD_DEPOT_MONIKER` and `RBRD_CLOUD_PREFIX` cases),
  per the now-finished RBRD-from-RBRR split. Fixture case count unchanged.

- **Split already landed in this pace.** `rbrr_cli.sh` furnish was stripped of
  the dead RBRD/RBDC coupling (commit 47e02720a). Do not redo.

## Reliability

Re-capture the fast baseline (currently absent at `/tmp/bjaab-fast-baseline.md`)
before migrating. Post-migration verdict set matches exactly; spot-check that
each negative fails for the SAME diagnostic, not merely non-zero.

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — `rbtdrf_run_rv` + the negative cases;
  the RBRV/RBRN baselines are the pattern to extend
- `Tools/rbk/rbr{r,d,v,n}_regime.sh` — the `z*_kindle`/`z*_enforce` the probate
  fn wraps; `rbr?_cli.sh` furnish shows the source→kindle→enforce sequence
- enrollment-validation cases (`rbtdrf_run_ev`) — the public-function-call
  precedent that blesses calling a contract surface directly

**[260520-2043] rough**

## Character

Design sub-task, forced by the ₢BJAAB step gate. Interface design + minting
first, then mechanical migration. Not a mechanical pace.

## Goal

Migrate the 24 regime-validation *negative* cases in `rbtdrf_fast.rs` off their
bypass (RBRR: source real regime + `export`-override one var; RBRV/RBRN: fully
synthetic env baselines; all then call `z`-private kindle/enforce) onto a
declared file-accepting validate entry point. Verdicts unchanged; each negative
must fail for the same reason.

## Why a new entry is required (step-gate finding — do not re-derive)

Existing validate tabtargets (`rbw-rrv`/`rvv`/`rnv`) read the fixed real moorings
tree: `RBCC_moorings_dir` is a non-overridable relative literal, and
`tt/z-launcher.sh` pins CWD to the real repo root (`cd -P` to `tt/..`). So
staging a malformed regime via cwd or env is impossible, and physically staging
malformed files under `rbmm_moorings/` pollutes the tree AND breaks the positive
all-`*` glob cases. The malformed state the negatives inject lives in-shell and
cannot cross a tabtarget process boundary.

## Shape

- Declare a public/test entry validating an arbitrary regime file — sketch
  `<prefix>_t_validate <regime>` on rbrr/rbrv/rbrn. NOT a test-only tabtarget.
  Mint per prefix discipline.
- Migrate the 24 negatives (12 RBRR, 4 RBRV, 8 RBRN) to write a staged malformed
  regime to the case temp dir and drive that entry against it.

## Reliability

Baseline at `/tmp/bjaab-fast-baseline.md` (re-capture if stale). Post-migration
verdict set matches exactly; each negative fails for the SAME diagnostic
(spot-check, not just exit status).

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — `rbtdrf_run_rv` + the 24 negative cases
- `Tools/rbk/rbrr_cli.sh`, `rbrv_cli.sh`, `rbrn_cli.sh` — `*_validate` entries
- `Tools/rbk/rbcc_Constants.sh` — `RBCC_moorings_dir`; `tt/z-launcher.sh` — CWD pin

### regime-enum-sprue-inventory (₢BJAAC) [abandoned]

**[260514-1357] abandoned**

## Character

Design conversation. Inventory + migration plan only — no rename, no
code edits. Mount-time picks the output format and spawns subsequent
migration paces that consume the inventory.

## Goal

Produce a complete listing of regime enum values across the project
that need to be sprue-prefixed, plus the data subsequent paces need
to decide migration cadence. Convention shape was locked in the
BBABJ wrap-thread: `<proj><n><regime><e>_<value>` — e.g.
`rbnve_conjure` for rb-vessel-regime, `bunne_linux` for bu-node-regime.
Existing `bubep_*` (BURN_PLATFORM) is the older-shape exemplar;
inventory explicitly recommends harmonize vs grandfather.

## Sweep scope — all languages

Bash, AsciiDoc specs, Python Cloud Build steps, Cloud Build YAML
substitutions, Rust test fixtures. Both declaration sites
(`buv_enum_enroll`, `buv_gate_enroll`) and every consuming use site
(equality test, case-statement arm, JSON literal, prose mention)
classified by language for cadence costing. Cadence decisions
themselves are deferred to the migration paces — this pace just
gathers the data they need.

## Discovery recipe

- `grep -rn 'buv_enum_enroll' Tools/{buk,rbk}/` → declarations + values
- Per discovered value V: `grep -rn '\bV\b' Tools/` filtered by
  extension; cross-check regime `.adoc` specs (RBSRV, RBSRN, BUS0,
  and any other `RB[SR]*` / `BU[S]*` regime-touching spec) for prose
  mentions
- Verify no enum missed: cross-walk `buv_gate_enroll` invocations
  against the discovered enum set

## Posture

Burn-bridges migration — no backwards compatibility. Depots reform
on execution. Single-sweep target; deep testing (regime-validation,
dockerfile-hygiene, gauntlet) catches stragglers post-migration.
Inventory is a consumable working artifact, not a permanent spec
fossil.

## Decision support to surface for migration paces

- Per-regime vs per-variable migration ordering (site counts inform)
- Which fixtures gate which migrations
- Harmonize-or-grandfather recommendation for `bubep_*`

## Out of scope

Rename execution. Cadence decision. Convention spec authoring (lives
with the migration paces). Sub-letter map design (locked: `n` for
namespace, regime-letter from the rbr*/bur* family, `e` for enum).

**[260514-1347] rough**

## Character

Design conversation. Inventory + migration plan only — no rename, no
code edits. Mount-time picks the output format and spawns subsequent
migration paces that consume the inventory.

## Goal

Produce a complete listing of regime enum values across the project
that need to be sprue-prefixed, plus the data subsequent paces need
to decide migration cadence. Convention shape was locked in the
BBABJ wrap-thread: `<proj><n><regime><e>_<value>` — e.g.
`rbnve_conjure` for rb-vessel-regime, `bunne_linux` for bu-node-regime.
Existing `bubep_*` (BURN_PLATFORM) is the older-shape exemplar;
inventory explicitly recommends harmonize vs grandfather.

## Sweep scope — all languages

Bash, AsciiDoc specs, Python Cloud Build steps, Cloud Build YAML
substitutions, Rust test fixtures. Both declaration sites
(`buv_enum_enroll`, `buv_gate_enroll`) and every consuming use site
(equality test, case-statement arm, JSON literal, prose mention)
classified by language for cadence costing. Cadence decisions
themselves are deferred to the migration paces — this pace just
gathers the data they need.

## Discovery recipe

- `grep -rn 'buv_enum_enroll' Tools/{buk,rbk}/` → declarations + values
- Per discovered value V: `grep -rn '\bV\b' Tools/` filtered by
  extension; cross-check regime `.adoc` specs (RBSRV, RBSRN, BUS0,
  and any other `RB[SR]*` / `BU[S]*` regime-touching spec) for prose
  mentions
- Verify no enum missed: cross-walk `buv_gate_enroll` invocations
  against the discovered enum set

## Posture

Burn-bridges migration — no backwards compatibility. Depots reform
on execution. Single-sweep target; deep testing (regime-validation,
dockerfile-hygiene, gauntlet) catches stragglers post-migration.
Inventory is a consumable working artifact, not a permanent spec
fossil.

## Decision support to surface for migration paces

- Per-regime vs per-variable migration ordering (site counts inform)
- Which fixtures gate which migrations
- Harmonize-or-grandfather recommendation for `bubep_*`

## Out of scope

Rename execution. Cadence decision. Convention spec authoring (lives
with the migration paces). Sub-letter map design (locked: `n` for
namespace, regime-letter from the rbr*/bur* family, `e` for enum).

### theurge-owns-suite-composition (₢BJAAE) [complete]

**[260529-1420] complete**

## Character

Architecture move requiring judgment — one load-bearing risk (single-process fixture isolation) gates an otherwise mechanical collapse.

## Goal

Make theurge the sole owner of suite→fixture composition. Suite names survive only as tabtarget imprints; no suite name or fixture list appears anywhere in bash.

## Locked constraints

- Supersedes the "suite is bash composition" decision (₢BBAAh, ₣BB) — a release-qual conformance call, not a principled ownership stance.
- Suite membership is expressed in Rust by reference to the existing `RBTDRC_FIXTURES` statics, so a bad member fails at compile time — not a name-string list mirrored across the boundary.
- The dirty-tree guard and per-run root allocation move to once-per-suite; today they run once per fixture.

## Shape

- Rust gains a suite registry and the binary a suite mode (suite name in, its fixtures run); the single-fixture and `single` modes stay. Bash's suite arrays, the suite-resolve, and the per-fixture loop collapse to an imprint pass-through; the suite-rationale comments (gauntlet state-walk, skirmish/dogfight operator preconditions) migrate to Rust doc on the registry.
- Load-bearing risk: the per-fixture loop becomes one process instead of N. Confirm crucible quench runs on the case-failure path, and settle quench-on-panic, before relying on the in-process loop.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — suite arrays, resolve, `rbte_suite` loop
- `Tools/rbk/rbtd/src/main.rs` — binary modes, dirty-tree guard, root allocation
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — `RBTDRC_FIXTURES` fixture statics
- the `rbw-ts.TestSuite.*` suite tabtargets — imprint channel

## What done looks like

Suite composition lives only in theurge; grep finds no suite names or fixture lists in bash; suite tabtargets pass only their imprint; a typo'd suite member fails compilation; all suites run green.

**[260529-1318] rough**

## Character

Architecture move requiring judgment — one load-bearing risk (single-process fixture isolation) gates an otherwise mechanical collapse.

## Goal

Make theurge the sole owner of suite→fixture composition. Suite names survive only as tabtarget imprints; no suite name or fixture list appears anywhere in bash.

## Locked constraints

- Supersedes the "suite is bash composition" decision (₢BBAAh, ₣BB) — a release-qual conformance call, not a principled ownership stance.
- Suite membership is expressed in Rust by reference to the existing `RBTDRC_FIXTURES` statics, so a bad member fails at compile time — not a name-string list mirrored across the boundary.
- The dirty-tree guard and per-run root allocation move to once-per-suite; today they run once per fixture.

## Shape

- Rust gains a suite registry and the binary a suite mode (suite name in, its fixtures run); the single-fixture and `single` modes stay. Bash's suite arrays, the suite-resolve, and the per-fixture loop collapse to an imprint pass-through; the suite-rationale comments (gauntlet state-walk, skirmish/dogfight operator preconditions) migrate to Rust doc on the registry.
- Load-bearing risk: the per-fixture loop becomes one process instead of N. Confirm crucible quench runs on the case-failure path, and settle quench-on-panic, before relying on the in-process loop.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — suite arrays, resolve, `rbte_suite` loop
- `Tools/rbk/rbtd/src/main.rs` — binary modes, dirty-tree guard, root allocation
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — `RBTDRC_FIXTURES` fixture statics
- the `rbw-ts.TestSuite.*` suite tabtargets — imprint channel

## What done looks like

Suite composition lives only in theurge; grep finds no suite names or fixture lists in bash; suite tabtargets pass only their imprint; a typo'd suite member fails compilation; all suites run green.

### regime-credentials-relocation-study (₢BJAAF) [complete]

**[260529-1728] complete**

## Character
Exploratory — study then plan; judgment, not mechanical. Output is a decision
(port vs delete) plus, if port, a concrete plan. No code changes in this pace
beyond possibly deleting confirmed-redundant dead code if the study is decisive.

## Goal
Decide the fate of `Tools/buk/buts/butcrg_RegimeCredentials.sh` — currently dead
code — and plan the cleanup: either port its coverage into RBK theurge, or delete
it if theurge already covers the equivalent.

## What butcrg is (and why it's dead)
It is the orphaned "regime-credentials" test suite: it verifies the three
credential-bearing regimes render+validate against the REAL credential files on a
configured workstation — RBRA per-role (`rbw-rav`/`rbw-rar`), RBRO payor-oauth
(`rbw-rov`/`rbw-ror`), RBRS station (`rbw-rsv`/`rbw-rsr`). It self-gates ("requires
a fully configured workstation"), so it is NOT CI-safe. Origin commit `077c81473`;
last live at `f720a7ac1` (₢AkAAc, "regime-credentials fixtures"). It lost its
enrollment during the test-infra consolidation into theurge: regime-smoke (CI-safe)
was ported to theurge (`rbtdrf_rs_*`), but the workstation-cred-gated
regime-credentials was not. Zero coverage since — a silently-dropped test.

## Study
Does theurge already cover this? theurge `regime-validation` validates SYNTHETIC
regime content (CI-safe); butcrg validates the operator's ACTUAL credential files.
Check whether any theurge service-tier fixture already validates real credential
files (not just synthetic). If equivalent coverage exists → delete butcrg. If not →
port. Look at `Tools/rbk/rbtd/src/rbtdrf_*` fixtures and the suite-tier definitions.

## Plan (if porting)
Port into theurge as a credential-gated fixture, mirroring the regime-smoke port,
and delete butcrg from `buk/` — this also advances the "move Recipe Bottle out of
BUK" direction. Reuse theurge's litmus skip-on-absent + tabtarget-invocation model;
the cases just invoke `rbw-rav rbnae_<role>` etc. and assert exit 0, exactly as the
bash does today. Do NOT spin up a parallel bash RBK testbench — that re-forks the
framework the consolidation just unified.

Open design point for plan-time: which tier. `rbw-rav`/`rbw-rar` validate local file
FORMAT only (no network); only an access-probe reaches GCP. So it sits between
`fast` (no creds) and `service` (reaches GCP). Decide service vs a new workstation
tier then.

## Layering note
BUK is the foundation; RBK builds on BUK. butcrg living in `buk/buts/` while
referencing RBK regime symbols (RBCC_*, zrbrr_kindle, RBDC_*) is a foundation-reaches-
up inversion — the root reason it cannot simply be re-enrolled in the BUK testbench.
The relocation fixes the inversion.

## Note
The minted-folio coverage butcrg would have given the ₣BM role/account split
(₢BMAAM) was already proven manually in that pace (rbw-rav rbnae_governor green,
rbw-acg HTTP 200 from a freshly-minted credential), so this pace owes nothing to
₣BM — it is pure test-infra hygiene.

**[260529-0709] rough**

## Character
Exploratory — study then plan; judgment, not mechanical. Output is a decision
(port vs delete) plus, if port, a concrete plan. No code changes in this pace
beyond possibly deleting confirmed-redundant dead code if the study is decisive.

## Goal
Decide the fate of `Tools/buk/buts/butcrg_RegimeCredentials.sh` — currently dead
code — and plan the cleanup: either port its coverage into RBK theurge, or delete
it if theurge already covers the equivalent.

## What butcrg is (and why it's dead)
It is the orphaned "regime-credentials" test suite: it verifies the three
credential-bearing regimes render+validate against the REAL credential files on a
configured workstation — RBRA per-role (`rbw-rav`/`rbw-rar`), RBRO payor-oauth
(`rbw-rov`/`rbw-ror`), RBRS station (`rbw-rsv`/`rbw-rsr`). It self-gates ("requires
a fully configured workstation"), so it is NOT CI-safe. Origin commit `077c81473`;
last live at `f720a7ac1` (₢AkAAc, "regime-credentials fixtures"). It lost its
enrollment during the test-infra consolidation into theurge: regime-smoke (CI-safe)
was ported to theurge (`rbtdrf_rs_*`), but the workstation-cred-gated
regime-credentials was not. Zero coverage since — a silently-dropped test.

## Study
Does theurge already cover this? theurge `regime-validation` validates SYNTHETIC
regime content (CI-safe); butcrg validates the operator's ACTUAL credential files.
Check whether any theurge service-tier fixture already validates real credential
files (not just synthetic). If equivalent coverage exists → delete butcrg. If not →
port. Look at `Tools/rbk/rbtd/src/rbtdrf_*` fixtures and the suite-tier definitions.

## Plan (if porting)
Port into theurge as a credential-gated fixture, mirroring the regime-smoke port,
and delete butcrg from `buk/` — this also advances the "move Recipe Bottle out of
BUK" direction. Reuse theurge's litmus skip-on-absent + tabtarget-invocation model;
the cases just invoke `rbw-rav rbnae_<role>` etc. and assert exit 0, exactly as the
bash does today. Do NOT spin up a parallel bash RBK testbench — that re-forks the
framework the consolidation just unified.

Open design point for plan-time: which tier. `rbw-rav`/`rbw-rar` validate local file
FORMAT only (no network); only an access-probe reaches GCP. So it sits between
`fast` (no creds) and `service` (reaches GCP). Decide service vs a new workstation
tier then.

## Layering note
BUK is the foundation; RBK builds on BUK. butcrg living in `buk/buts/` while
referencing RBK regime symbols (RBCC_*, zrbrr_kindle, RBDC_*) is a foundation-reaches-
up inversion — the root reason it cannot simply be re-enrolled in the BUK testbench.
The relocation fixes the inversion.

## Note
The minted-folio coverage butcrg would have given the ₣BM role/account split
(₢BMAAM) was already proven manually in that pace (rbw-rav rbnae_governor green,
rbw-acg HTTP 200 from a freshly-minted credential), so this pace owes nothing to
₣BM — it is pure test-infra hygiene.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A access-probe-to-credential-check
  2 B theurge-contract-surface-migration
  3 D rv-negative-test-api
  4 E theurge-owns-suite-composition
  5 F regime-credentials-relocation-study

ABDEF
·xx·x rbtdrf_fast.rs
x··x· rbtdrc_crucible.rs, rbte_engine.sh
x·x·· rbtdrm_manifest.rs
····x butcrg_RegimeCredentials.sh
···x· buq_qualify.sh, main.rs, rbq_Qualify.sh, rbtdre_engine.rs
··x·· rbrd_regime.sh, rbrn_regime.sh, rbrr_cli.sh, rbrr_regime.sh, rbrv_regime.sh
x···· rbgv_cli.sh, rbk-claude-tabtarget-context.md, rbtd-ap.AccessProbe.director.sh, rbtd-ap.AccessProbe.governor.sh, rbtd-ap.AccessProbe.payor.sh, rbtd-ap.AccessProbe.retriever.sh, rbtdrk_canonical.rs, rbtdro_onboarding.rs, rbtdrp_pristine.rs, rbtdti_invocation.rs, rbte_cli.sh, rbw-acd.CheckDirectorCredential.sh, rbw-acg.CheckGovernorCredential.sh, rbw-acp.CheckPayorCredential.sh, rbw-acr.CheckRetrieverCredential.sh, rbz_zipper.sh

Commit swim lanes (x = commit affiliated with pace):

  1 A access-probe-to-credential-check
  2 B theurge-contract-surface-migration
  3 D rv-negative-test-api
  4 E theurge-owns-suite-composition
  5 F regime-credentials-relocation-study

123456789abcdefghijklmnopqrs
·······xx···················  A  2c
·············xx·············  B  2c
··················xxx·······  D  3c
······················x·xx··  E  3c
··························xx  F  2c
```

## Steeplechase

### 2026-05-29 17:28 - ₢BJAAF - W

Studied and resolved the orphaned butcrg_RegimeCredentials dead-code suite. Verdict: RBRA + RBRO families already covered more strongly by the service-tier access-probe (enforces regime + mints real GCP token); only RBRS station validate was uncovered. Deleted butcrg from buk/ (fixes foundation-reaches-up inversion) and ported the RBRS slice as regime-smoke case rbtdrf_rs_rbrs -- skip-on-absent, render+validate via rbw-rsr/rbw-rsv. Build clean, 100 unit tests + 113 fast-suite cases pass (new rbrs case ran green against the live station file).

### 2026-05-29 17:27 - ₢BJAAF - n

Resolve the orphaned butcrg_RegimeCredentials dead-code test suite: delete it from buk/, porting only its one genuinely-uncovered slice into theurge. Study verdict: of butcrg's three credential-bearing regime families, RBRA (governor/director/retriever) and RBRO (payor OAuth) are already covered MORE strongly by the service-tier access-probe fixture, which sources+kindles+enforces the regime and then mints a real GCP token against the credential file -- strictly subsuming butcrg's format-only render+validate. Only RBRS (station regime: PODMAN_ROOT_DIR/VMIMAGE_CACHE_DIR/VM_PLATFORM) was uncovered -- it carries no GCP credential so there is no access probe for it, and it sits outside the probate surface (RBRR/RBRD/RBRV/RBRN). Ported as a new regime-smoke case rbtdrf_rs_rbrs beside its sibling rbtdrf_rs_burs (BUK station regime): it skip-on-absents when ../station-files/rbrs.env is missing (keeping the fast suite green on a fresh checkout) and render+validates via rbw-rsr/rbw-rsv on a configured workstation, referencing the generated RBTDGC_RENDER_STATION/VALIDATE_STATION/RBRS_FILE colophon constants. This fixes the foundation-reaches-up layering inversion (butcrg lived in buk/ while referencing RBK symbols) and advances moving Recipe Bottle out of BUK. Deliberately ported RBRS only -- adding RBRA/RBRO format-validates would be a non-load-bearing duplicate of stronger access-probe coverage. 100 theurge unit tests pass; build clean.

### 2026-05-29 14:20 - ₢BJAAE - W

Theurge is now the sole owner of suite->fixture composition. Added rbtdre_Suite struct + RBTDRC_SUITES registry (keyed by reference to fixture statics, so a bad member fails compilation) with the gauntlet/skirmish/dogfight/tadmor rationale migrated to Rust doc; binary gained a `suite` mode running the dirty-tree guard and root allocation once per suite, looping fixtures with per-fixture context and fail-fast matching the old set -e; abort-on-panic kept as documented behavior-preserving choice. Bash collapsed: suite arrays and zrbte_resolve_suite deleted, rbte_suite is an imprint pass-through. Validated green: fast 112/112, crucible 179/179 across 8 fixtures (three successful in-process charge/quench cycles), 117 unit tests. Folded in a shellcheck version pin (BUQ_SHELLCHECK_VERSION=0.11.0, exact-match die in buq_shellcheck): the charge preflight gate was silently broken by this station's shellcheck 0.9.0 lacking --rcfile; installed 0.11.0 and pinned to force linux/macos stations to stay in sync. macos/pym still needs shellcheck 0.11.0 installed to pass the gate.

### 2026-05-29 14:20 - ₢BJAAE - n

Pin shellcheck to an exact version in the qualification gate (buq_shellcheck). Surfaced while validating the suite work: every Charge.* tabtarget runs rbq_qualify_fast as a preflight, and this station's apt shellcheck 0.9.0 predates the --rcfile flag (added 0.10.0), so shellcheck rejected the flag, ran rule-less across 181 files, and failed the gate -- silently blocking all crucible charges. Resolved by installing shellcheck 0.11.0 (station-local, ~/.local/bin, ahead of /usr/bin on PATH) and adding a hard version pin: new BUQ_SHELLCHECK_VERSION=0.11.0 constant and a check in buq_shellcheck that parses `shellcheck --version` and buc_die's instantly on any mismatch, before running. Exact-pin (not floor) so cross-station drift between linux/cerebro and macos/pym becomes an immediate actionable failure rather than divergent silent behavior across the 0.10 rcfile-semantics boundary. After install, fast 112/112 and crucible 179/179 (8 fixtures, three in-process charge/quench cycles) both green.

### 2026-05-29 07:09 - Heat - S

regime-credentials-relocation-study

### 2026-05-29 13:35 - ₢BJAAE - n

Make theurge the sole owner of suite->fixture composition. Add rbtdre_Suite struct (engine) and the RBTDRC_SUITES registry + rbtdrc_lookup_suite (crucible), keyed by reference to the existing fixture statics so a mistyped/deleted member fails compilation; the gauntlet/skirmish/dogfight/tadmor rationale comments migrate from bash onto the registry entries. Binary gains a `suite` mode (rbtdb_run_suite): verifies each member's colophons up front, runs the dirty-tree guard and root allocation once per suite (was once per fixture under the bash loop), then loops fixtures with per-fixture context set/take, fail-fast across fixtures matching the old `set -e`, one aggregate summary. Documents that a panicking case aborts the suite with its crucible charged -- the same leak the per-process loop had; routine Fail verdicts quench normally via finally-shaped teardown. The misnamed single-fixture runner rbtdb_run_suite renamed to rbtdb_run_fixture. Bash collapses: ZRBTE_SUITE_* arrays and zrbte_resolve_suite deleted, rbte_suite is now an imprint pass-through to `rbtd suite`. rbq_Qualify comment repointed to the Rust registry. Build clean under deny(warnings), 117 theurge unit tests pass.

### 2026-05-29 13:18 - Heat - S

theurge-owns-suite-composition

### 2026-05-22 10:24 - ₢BJAAD - W

Migrated regime-validation negatives off their internal-reach bypass onto public per-regime rbr{r,d,v,n}_probate functions (source->kindle->enforce), called like the buv_vet precedent. Deleted rbtdrf_run_rv (7-module hardcoded chain + z-private replication); replaced with rbtdrf_run_probate driving synthetic baseline+override through the probate surface. Module/probate-fn names centralized as RBTDRM_* manifest consts. Recategorized 5 misnamed rv_rbrr_* (actually RBRD) cases into rv_rbrd_*. Strengthening beyond docket: completed the previously-invalid RBRV/RBRN baselines and added 5 pass-anchors proving each mutated baseline validates clean -- rbrn_port_conflict now actually exercises the cross-port check (never reached before). regime-validation 27->32; full fast suite 112/112; 114 unit tests; clean build under deny(warnings); shellcheck clean under busc_shellcheckrc.

### 2026-05-22 10:24 - ₢BJAAD - n

Migrate regime-validation negatives onto a public per-regime probate contract surface, plus baseline-validity strengthening. Mint rbr{r,d,v,n}_probate <file> in the four regime modules (source file -> kindle -> enforce, fail on first fault); theurge calls these like the buv_vet precedent. Delete rbtdrf_run_rv (the 7-module hardcoded source chain + inline z-private kindle/enforce replication) and replace with rbtdrf_run_probate: synthetic baseline+override to case temp dir, sources only buv + the one regime module, calls *_probate; expect_ok selects polarity. Module filenames and probate fn names added to rbtdrm_manifest as RBTDRM_MODULE_*/RBTDRM_PROBATE_* consts (String Boundary Discipline). Recategorize the 5 misnamed rv_rbrr_* cases that test RBRD_* into rv_rbrd_* against an RBRD baseline. Strengthening: completed the RBRV/RBRN synthetic baselines (RBRV_RELIQUARY, RBRV_EGRESS_MODE, RBRN_BOTTLE_READINESS_DELAY_SEC were missing, masking three negatives) and added 5 pass-anchors proving each mutated baseline validates clean -- rbrn_port_conflict now actually executes the cross-port check (previously never reached). regime-validation 27->32, full fast suite 112/112, 114 unit tests, build clean under deny(warnings), shellcheck clean under busc_shellcheckrc.

### 2026-05-22 05:57 - ₢BJAAD - n

Finish the unfinished RBRD-from-RBRR split: strip rbrr_cli furnish of the dead RBRD/RBDC coupling (rbrd_regime, rbrd.env, rbdc, rbgc sources plus zrbrd_kindle/enforce and zrbdc_kindle) it carried from before the split. RBRR's only commands (validate, render) operate purely on RBRR; furnish now mirrors rbrd_cli's light path. Dissolves the apparent rbrr/rbrd 'pair' that complicated the file-accepting validate-entry design for this pace.

### 2026-05-21 11:25 - Heat - n

Reinstate RBRD_DEPOT_MONIKER to canest3bhm100001 (next free in family) after unmaking the half-built canest3bhm100000 depot, clearing the placeholder used to pass the live-depot unmake guard. Sets up a direct rbw-dL levy retry now that Cloud Build API is enabled on the payor project.

### 2026-05-21 11:09 - Heat - n

Add Cloud Build API to the payor-establish handbook's Section 7 required-APIs list. The ceremony enabled the five payor-consumer APIs (Resource Manager, Billing, Service Usage, IAM SA Credentials, Artifact Registry) but omitted Cloud Build, whose API is consumed when the depot levy polls worker-pool LRO operations against the payor project. A freshly-established payor following this handbook always hits SERVICE_DISABLED 403 on the tether worker-pool poll; the old payor masked the gap by having Cloud Build incidentally enabled.

### 2026-05-21 10:59 - Heat - n

Bump canonical-establish family stem canest2->canest3 to escape a global projectId reservation collision: the prior gmail-identity payor's deleted canest2bhm100000 depot holds the globally-unique ID in DELETE_REQUESTED (~30-day reservation), and the active-only, single-identity allocator re-derived a reserved ID it can neither see nor own, yielding HTTP 409 on depot levy. Comment records the cross-identity cause.

### 2026-05-20 20:50 - ₢BJAAB - W

Migrated the 3 regime-validation positive cases in rbtdrf_fast.rs onto contract-surface tabtargets, each mirroring its proven regime-smoke sibling: rv_rbrr_repo -> rbw-rrv; rv_rbrv_all_vessels -> discover RBRR_VESSEL_DIR then loop rbw-rvv <sigil>; rv_rbrn_all_nameplates -> read_dir then loop rbw-rnv <moniker>. The hardcoded module-source paths, replicated kindle chains, and direct z-private calls are gone from these three; rbtdrf_run_rv and the rbtd_moorings_dir! macro remain in use by the still-bypassed negatives.

### 2026-05-20 20:50 - ₢BJAAB - n

Sorry, that scheduling call was an accident — ignore it. Here is the commit message:

### 2026-05-20 20:43 - Heat - S

rv-negative-test-api

### 2026-05-20 20:27 - Heat - n

Re-kludge tadmor bottle hallmark (k260520202731-b68443efc) into rbrn.env. Tadmor now fully hallmarked (sentry+bottle) and chargeable for the depot-free crucible-suite regression. Ephemeral local kludge state, not pace logic.

### 2026-05-20 20:27 - Heat - n

Re-kludge tadmor sentry hallmark (k260520202704-76191391e) into rbrn.env to satisfy the bottle-kludge clean-tree gate during ₢BJAAB baseline prep. Ephemeral local kludge state from the Marshal-Zero blanking restore, not pace logic.

### 2026-05-21 02:51 - Heat - n

Restore reliquary after Marshal-Zero blanking: inscribe r260521024212 into live depot canest2bhl100021 and yoke into all 9 vessels' rbrv.env (RBRV_RELIQUARY repopulated). Greens rbtdrf_rv_rbrv_all_vessels in the fast suite.

### 2026-05-21 02:23 - ₢BJAAA - W

Retired theurge-owned rbtd-ap.AccessProbe.* colophon family; replaced with four operator credential-check tabtargets (rbw-ac{g,r,d,p}) dispatched by rbw_workbench via new rbgv_cli.sh thin CLI. Theurge now owns zero colophons: stripped dispatch from rbte_cli.sh/rbte_engine.sh, replaced ACCESS_PROBE constant in rbtdrm_manifest.rs with four CHECK colophons + rbtdrm_credential_check_colophon helper + RBTDRM_ROLE_* constants. Fixtures (rbtdrc/rbtdrk/rbtdrp/rbtdro) repointed to invoke_global; rbtdti imprint test repointed to rbw-cC charge family. Validated: rbtd-b clean, rbtd-t 114/114, fast 107/107, then live credentialed runs on freshly-levied canest2bhl100021 — canonical-establish 4/4 (rbw-acr/rbw-acd live) and access-probe 4/4 (all four rbw-ac* probes: governor/director/retriever JWT + payor OAuth). Concurrency with live macOS bhm gauntlet proven safe both analytically (tincture-disjoint depot/SA namespaces; rbgg_Governor.sh: no concurrent writers per project) and in practice (zero collision).

### 2026-05-21 02:10 - ₢BJAAA - n

Retire theurge-owned rbtd-ap.AccessProbe.* colophon family; replace with four operator credential-check tabtargets (rbw-ac{g,r,d,p}) dispatched by rbw_workbench via new rbgv_cli.sh thin CLI. Theurge now owns zero colophons: strip dispatch from rbte_cli.sh/rbte_engine.sh, replace ACCESS_PROBE constant in rbtdrm_manifest.rs with four CHECK colophons + rbtdrm_credential_check_colophon helper + RBTDRM_ROLE_* constants. Fixtures (rbtdrc/rbtdrk/rbtdrp/rbtdro) repointed to invoke_global; rbtdti imprint test repointed to rbw-cC charge family. Verified: rbtd-b clean, rbtd-t 114/114, fast suite 107/107, live dispatch confirmed.

### 2026-05-20 08:45 - Heat - d

paddock curried: record grooming survey: BK sequencing, one-file migration scope, injectability crux, verdict-baseline spine

### 2026-05-14 13:57 - Heat - T

regime-enum-sprue-inventory

### 2026-05-14 13:47 - Heat - S

regime-enum-sprue-inventory

### 2026-05-14 12:34 - Heat - S

theurge-contract-surface-migration

### 2026-05-12 19:00 - Heat - f

racing

### 2026-05-12 14:40 - Heat - S

access-probe-to-credential-check

### 2026-05-12 14:39 - Heat - N

rbk-14-mvp-testing-simplification

