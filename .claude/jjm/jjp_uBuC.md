## Context

Framework self-certification for the theurge: the rbtd binary that runs every RB test fixture has unit-tested internals but no test of its own operator surface — exit codes, stderr diagnostics, fail-fast behavior.
Split from the retired mvp-3 release-qualification heat (₣BB) so framework polish never blocks the release ladder; the calibrant fixture family that heat landed (`rbtdrl_calibrant.rs`) is this heat's test subject.
Regroomed 260711 against the post-split landscape; the decisions below are cinched.

## Two pillars

**Surface certification.**
An in-crate surface fixture — a green, credless reveille member — spawns child rbtd runs through the real tabtarget chain against the deliberately-failing calibrant fixtures and asserts the child's exit code, stderr shape, and sentinel/trace files.
The watcher passes; the watched stay roster-only.
This replaces the original bash-testbench architecture: no new testbench module, no new colophon, no runbook entry; RBSTC's ratification note about re-minting the testbench family at this heat's mount is moot.
Self-hosting is accepted: the child is observed from outside (exit codes, files), and a child run traverses the full sandwich — tabtarget, launcher, workbench, rbte_engine, binary — wider coverage than a bash driver calling the binary directly.

**Census enforcement.**
Each fixture's `rbtdrm_required_colophons` list is actively maintained but consumed by nothing: the runtime existence check (`rbtdrm_verify`) was deliberately retired in favor of compile-time const projection, and usage alignment cannot be compile-checked — you only learn what a fixture invokes by running it.
Enforce both directions at the invocation chokepoint and report per-colophon usage.

## Cinched (260711 groom)

- Enforce the census, both directions; a declared-but-never-invoked colophon FAILS the fixture on a fully-green full-fixture run.
  This deliberately overrides the tariff precedent (`count_drift` warns, never affects verdict): census drift is a defect, not a curiosity.
- The surface fixture is a reveille member; the calibrant fixtures themselves stay out of every dependency-tier and release suite (deliberate failers).
- One small calibrant suite is registered in `RBTDRA_SUITES` solely as the suite-abort test subject, driven only by the surface fixture.
  The prior lock against it ("bash `set -e` provides suite fail-fast, no Rust suite needed") dissolved when suite composition moved into the binary — `rbte_suite` is now a passthrough.
- Keep-going is plumbed through the tabtarget-to-binary chain so the `rbtdre_resolve_fail_fast` policy — including the StateProgressing refusal — is reachable from outside; today no CLI flag exists and the gate is unit-test-only.
- Anchor word `calibrant`, module `rbtdrl_calibrant.rs` — consumed from ₣BB, unchanged.
- BURV chain (BUS0 §540-552) is contract, not hook: child runs nest BURV through real invocations and incidentally regression-test it.
- No per-fixture tabtargets for the calibrant family.

## Surface-fixture case catalog

Sections name behavior, not case counts (counts are mount-time):

- verdict-propagation — pass / fail / skip child exit codes + trace file presence
- fixture-fail-fast — intra-section and inter-section halt
- disposition-policy — Independent + keep-going runs all cases; StateProgressing + keep-going refused with policy stderr; StateProgressing default runs fail-fast
- probe-diagnostics — unmet-probe stderr carries the precondition and remediation lines
- suite-abort — a failing fixture halts the registered calibrant suite; the later fixture's sentinel absent
- cli-surface — unknown fixture errors clearly; missing arg yields usage
- coverage — aligned passes; undeclared fails naming the colophon; unused fails naming the colophon; single-case exempt from the negative check

## Win-series role

Theurge is the substrate-sensitive piece of the stack; the Cygwin failures bit exactly at the tabtarget-invocation boundary (`Memos/memo-20260517-windows-substrate-landscape-for-theurge.md`).
A reveille run on a Windows substrate therefore certifies that boundary on every pass — the surface fixture is the parity instrument.

## Out of scope

- Trace-file format invariants beyond a single smoke check; color / terminal-width contracts.
- Imprint coverage (which nameplate a per-imprint colophon was called with), argument patterns, compile-time/static analysis of usage.
- Fixture-level declaration edits in existing fixtures — a real stale entry surfaced by the negative check is a separate find.
- Deleting or reshaping the tariff mechanism.

## References

- `Tools/rbk/rbtd/src/rbtdrl_calibrant.rs` — calibrant family (test subjects)
- `Tools/rbk/rbtd/src/rbtdri_invocation.rs` — invoke primitives (the chokepoint)
- `Tools/rbk/rbtd/src/rbtdre_engine.rs` — `rbtdre_resolve_fail_fast`, tariff machinery
- `Tools/rbk/rbtd/src/rbtdra_almanac.rs` — `RBTDRA_SUITES` registry, roster-only doctrine
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `rbtdrm_required_colophons`
- `Tools/rbk/vov_veiled/RBSTC-theurge_cosmology.adoc` — cosmology; ratification note on the now-moot testbench re-mint