## Gate

Do not start this heat until ₣Bb's bash-filename case-consolidation pace
(now first in that heat) has landed.
That pace renames PascalCase bash files across `Tools/rbk` —
including files this heat edits directly
(the rbcc constants module, the foundry-hygiene gate, the validation gate homes) —
and rename-on-one-side, edit-on-the-other is the worst git merge shape.
Once it lands, this heat runs comfortably in a parallel repo alongside the rest of ₣Bb;
verify the rename pace is wrapped via `jjx_show` on ₣Bb before mounting here.
Mind that pace's renames when reading this heat's dockets:
file references here predate the consolidation,
so resolve any stale PascalCase basename to its lowercase successor.

## Shape

Theurge negative cases assert bare nonzero exit,
so a case expecting deliberate rejection also passes on any harness breakage
(unbound variable, missing file, refactor typo) —
~60 negative cases across the fast-tier fixtures carry this wrong-reason hole.
The fix: a small precision exit-code band, allocated per rejection gate
(roughly half a dozen codes), homed as tinder in bubc,
projected into the theurge Rust consts by the existing zipper codegen,
and asserted specifically by the negative helpers.

Work order: membrane first (band tinder + `buc_die` band-propagation + survival proof),
then numeric codegen, then per-fixture migrations, then the BCG admission entry.

## Cinched

- Exit-code band over stderr sentinel:
  a sentinel is string interpretation minimized, not avoided,
  and is swallowed by the same wrappers that launder codes.
- Per-gate codes, never per-rule —
  the hole being closed is wrong-layer failure, not wrong-rule failure.
- Allocation rule: gates may share a code only if they never co-occur
  in one test case's spawn path — share across alternatives, never along a pipeline.
- `buc_die` propagates in-band `$?` values instead of remapping to 1,
  so existing `|| buc_die` call sites need no audit or change.
- No code minted outside the bubc tinder block.

## Out of scope

tadmor-security's containment cases:
exit codes there cross docker-exec boundaries with their own laundering rules —
a different problem, a different heat if it ever itches.

## Done when

Every fast-tier negative case asserts a specific band code,
the survival proof lives in the BUK self-test,
and the BCG admission entry documents band semantics and the allocation rule.