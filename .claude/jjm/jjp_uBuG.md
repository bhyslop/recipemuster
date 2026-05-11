## Goal

Normalize and upgrade existing `.adoc` orchestration subdocuments to
the orchestration-spec discipline that emerged from the style-guide
design under heat ₣A-. The pattern was developed empirically against
`BUSJGW-GarrisonWsl.adoc` (Windows workload garrison via WSL) and is
captured in
`Memos/memo-20260511-orchestration-style-axla-draft.md`.

The exemplar that grounds this heat is the current state of BUSJGW:
explicit `{busc_*}` / `{rbbc_*}` control verbs in every step body,
FATAL recovery-scope discipline, trailing Rationale section
(load-bearing only), trailing References section (epistemic
provenance), step granularity matching the operation's transport
contract (WSG WSp-105 for Windows; REST atomicity for cloud), no
false probe-then-conditional branches when one idempotent dispatch
suffices.

## Locked Decisions

- **Concrete-first codification.** Stress-test the discipline on a
  second exemplar (`RBSAC-ark_conjure`, Cloud Build / REST
  transport) before committing the AXLA dimension. Memo notes this
  explicitly.
- **Codify early, not late.** Once the dimension shape survives
  both BUSJGW (Windows) and RBSAC (cloud REST), commit AXLA
  immediately. Late codification leaves remaining scrubs
  referencing an uncommitted memo -- the partial-products failure
  mode.
- **Proposed dimension name:** `axd_orchestrating`. Alternatives in
  the memo: `axd_composing`, `axd_compositional`. Final decision at
  the codification pace.
- **False-branches anti-pattern is load-bearing.** A failed step
  with a unique die string (`buc_die` / `rbbc_fatal` /
  `busc_fatal`) is sufficient diagnostic. Reserve real branching
  for dynamic discovery + iteration, or where the probe result is
  consumed by something other than the dispatch.
- **Runtime variables declared in sequence via control verbs.** No
  separate Runtime Values table.

## Candidate Orchestrations

Density signal: `axhob_operation` + `axhos_step` count + control-verb
count + `_fatal` count. Regenerate via grep over
`Tools/buk/vov_veiled/*.adoc` and `Tools/rbk/vov_veiled/*.adoc`
filtering for `axhob_operation`.

**Tier 1 -- Windows posture family** (direct siblings of BUSJGW;
lockstep treatment):

- `BUSJCW-CaparisonWindows` -- 14 steps / 12 calls / 14 fatals
- `BUSJIW-InvigilateWindows` -- 8 / 8 / 8
- `BUSJGC-GarrisonCygwin` -- 7 / 5 / 7
- `BUSJGB-GarrisonBash` -- 7 / 0 / 0 (needs full verb pass)

**Tier 2 -- Cross-platform mirrors:**

- `BUSJCM`, `BUSJCL` -- 5 / 5 / 5 each
- `BUSJIM`, `BUSJIL` -- 8 / 8 / 8 each

**Tier 3 -- High-density GCP orchestrations** (REST transport;
several have zero fatals despite being core composition):

- `RBSDE-depot_levy` -- 14 / 11 / 1
- `RBSDK-director_invest` -- 12 / 9 / 2
- `RBSDU-depot_unmake` -- 8 / 10 / 5
- `RBSGM-governor_mantle` -- 8 / 7 / 0 (no fatals -- major scrub)
- `RBSRK-retriever_invest` -- 8 / 6 / 2
- `RBSTB-trigger_build` -- 7 / 3 / 6 (empirical retry/poll lives
  here)
- `RBSAC-ark_conjure` -- 5 / 1 / 0 (stress-test candidate; core
  Cloud Build orchestration with no fatals)
- `RBSAV-ark_vouch` -- 8 / 7 / 1
- `RBSAB-ark_about` -- 4 / 6 / 0
- `RBSAA-ark_abjure` -- 5 / 2 / 5
- `RBSAG-ark_graft` -- 5 / 5 / 0
- `RBSAS-ark_summon` -- 5 / 3 / 1

**Tier 4 -- Probably not orchestrations** (review to confirm or
document discriminator):

- Handbooks: `BUSJHW`, `BUSJHM`, `BUSJHL`
- Regime list/render/validate: `BUSTLL`, `BUSTPL`, `BUSTLR`,
  `BUSTPR`, `BUSTLV`, `BUSTPV`
- Low-level wrappers: `BUSJWC`, `BUSJWS`, `BUSJWK`, `BUSJPS`,
  `RBSCE`
- Display/inventory ops: `RBSDL`, `RBSCL`, `RBSAP`

## Suggested Pace Order (Cut When Heat Engages)

1. RBSAC stress-test scrub (Cloud Build / REST transport)
2. AXLA codification: mint dimension, lint contract, false-branches
   anti-pattern; voice on BUSJGW and RBSAC definition sites
3. Tier 1 (BUSJCW, BUSJIW, BUSJGC, BUSJGB)
4. Tier 2 (cross-platform mirrors)
5. Tier 3 (GCP orchestrations)
6. Tier 4 review (confirm non-orchestration; document
   discriminator)

Steps 1 and 2 gate the rest: the dimension must survive both
transports before codification; remaining scrubs reference the
codified dimension.

## What Done Looks Like

- Every orchestrating procedure in `Tools/buk/vov_veiled/` and
  `Tools/rbk/vov_veiled/` voices the codified dimension at its
  definition site.
- Each conforms to the lint contract: control verb per step,
  trailing Rationale and References sections, FATAL recovery-scope
  discipline, false-branches anti-pattern applied.
- A simple prose ↔ control-verb consistency lint exists (regex
  pass, per the memo).
- Tier 4 specs either get the dimension or carry explicit
  documentation of why they don't.

## Notes

- The memo
  `Memos/memo-20260511-orchestration-style-axla-draft.md` is the
  working spec for the discipline. Read it first when mounting any
  pace.
- The BUSJGW exemplar at
  `Tools/buk/vov_veiled/BUSJGW-GarrisonWsl.adoc` is the worked
  reference.
- LLM failure modes to corral against (self-flagged):
  over-engineering retry/backoff trees, reintroducing rationale
  into step bodies, adding "improved" error handling not in the
  source spec, generating cross-references that break silently.
  Defer to the codified discipline; do not elaborate beyond it.