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

Two strands, one gestalt: doctrine-pure negative testing.

Strand one — named refusals.
Theurge negative cases assert bare nonzero exit,
so a case expecting deliberate rejection also passes on any harness breakage
(unbound variable, missing file, refactor typo) —
the fast-tier fixtures carry this wrong-reason hole across their negative cases.
The fix: a small precision exit-code band, allocated per rejection gate
(roughly half a dozen codes; the census at the 2026-06-11 groom counted 6–8 gates),
homed as tinder in bubc,
projected into the theurge Rust consts by the existing zipper codegen,
and asserted specifically by the negative helpers.

Strand two — the tweak channel re-grounded (added at the 2026-06-11 groom).
The BUS0 Tweak Mechanism doctrine (landed at heat start) states what a tweak is for:
force one hard-to-produce condition for a test to observe handled correctly;
one tweak at a time per test/fixture/suite, by design;
a suite may reserve the slot for a standing guard.
This heat brings the live census into conformance:
the two non-conforming tweaks retire
(the immure convenience short-circuit becomes a real read-only dry-run colophon;
the graft parameter injection becomes the hallmark-installer election chain,
riding the existing hallmark fact + previous-dir chaining machinery),
the fast tier reserves the slot for the credless guard
(closing the recorded near-miss class: a passing fast suite that spends money and mutates the depot,
gated at the token-mint chokepoint with a band code),
and the regime-validation negatives convert from fabricated files driven through
test-only `*_probate` side doors to in-universe poisoning —
real validate verbs against real regimes, one BUK regime-load seam with set/unset semantics —
housed in a new fixture that runs in every suite above fast
(fast holds the guard; poison cases need the slot; the two never share a run).

Work order: census cleanup first (band-independent),
then band membrane + numeric codegen,
then the guard (its rejection code needs the band),
then the conversions, prose last.

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
- One tweak at a time is doctrine, not limitation;
  a dedicated constraint variable mints only when a genuine dual-tweak need survives scrutiny —
  never by widening the single slot.
- Fast carries the credless guard in the tweak slot;
  fast cases carry no tweaks of their own.
  A case that needs a seam has self-identified as not belonging in fast.
- In-universe over fabrication: negatives run real verbs against real regimes wherever the
  injection point is a value; the probate side doors delete with no parallel survival.
- The guard gates token mint (actual credential use), never zipper/dispatch — zippers untouched.
- The hallmark installer is graft-slot election only;
  any general election verb belongs to the made-side retrofit heat.

## Out of scope

- tadmor-security's containment cases:
  exit codes there cross docker-exec boundaries with their own laundering rules —
  a different problem, a different heat if it ever itches.
- enrollment-validation needs no in-universe conversion:
  it already stages values inline against the validator with no file fabrication and no tweak;
  it takes only the band-code assertion migration.
- recipe-validation and dockerfile-hygiene fabrication is honest:
  the artifact under test IS a file; they take only the band-code migration.

## Done when

Every fast-tier negative case asserts a specific band code,
the survival proof lives in the BUK self-test,
the tweak census conforms to the BUS0 doctrine (stamp, poison seam, guard — nothing else),
no regime `*_probate` side door survives,
fast is credless by construction,
and the BCG admission entry documents band semantics and the allocation rule.