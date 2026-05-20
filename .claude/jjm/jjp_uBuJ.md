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