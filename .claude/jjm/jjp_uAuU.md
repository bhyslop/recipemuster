## Paddock: rbk-14-mvp-release-finalize

## Context

A small cleanup-and-validation heat at the tail of the Recipe Bottle release
effort. Its original charter — supply-chain trust hardening plus
release-qualification tooling — has largely shipped or migrated. The
release-qualification charter now lives in the rbk-16 heat (₣BB): the
marshal-zero → gauntlet/pristine model, the RELEASE.md runbook, and the
prep-release ceremony all belong there. The prep-release ceremony rewrite and
the foundry/cloud-step trust audit that once lived here were restrung into ₣BB
so the release ceremony and the foundry surface each have a single owner.

What remains in ₣AU is genuinely local: developer-facing hygiene and end-to-end
validation that do not depend on the gauntlet model.

## Remaining Scope

- **Regime-render error hygiene** — every regime render/validate/info function
  should fail with an actionable parameter message, not a terse "module not
  kindled" crash or a set -u unbound-variable death. Includes a sweep test
  fixture that exercises every render/info and asserts clean exit, plus logging
  discipline for batch-sweep context.
- **Onboarding end-to-end walk** — exercise every handbook track in sequence
  from a learner's perspective; confirm windows render with prefixed resource
  names and probe outputs report the correct prefixed identifiers.
- **openssl base64 helper factoring** — the six base64 sites that ₣BB's base64
  sweep unified on openssl still carry a duplicated invocation; factor into
  rbgu_Utility.sh helpers, or close won't-fix with rationale recorded.
- **Small doc/file hygiene** — relocate the compose-probe sentinel out of the
  .rbk/ runtime config directory into Tools/rbk/ (the file move itself already
  appears done — verify references before closing); and a BCG clarification
  distinguishing the `$(<file)` builtin redirect from command substitution
  (verify against the landed substitution-rule refinement first — may already
  be covered).

## Relationship to other heats

- **₣BB (rbk-16-mvp-release-qualification)** — the live release-qualification
  heat; inherited this heat's prep-release ceremony work and the foundry trust
  audit. The audit is sequenced at ₣BB's tail so it reviews the post-reshaping
  foundry surface rather than transitional code.
- **₣Ak (rbk-mvp-1, retired)** — prior art for the directory/spec/test
  structure.

## Deferred technical debt (post-release itch candidates)

Catalogued during this heat's BCG-compliance work; deferred, not lost. Better
homed in jji_itch.md than in the paddock long-term:

- `z_` prefix on locals — naming convention, no runtime impact (pervasive)
- Unbraced `"$var"` expansions — mechanical
- `2>/dev/null` stderr suppression — case-by-case judgment
- `local -r` missing on single-assignment locals — pervasive
- Raw `echo` for user messages in display-heavy modules
- Bare file truncation (`> file` without `:`)

## References

- Tools/rbk/rbq_Qualify.sh — qualification orchestrator
- Tools/buk/buv_validation.sh — core `buv_render()` engine
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md — command-substitution rules
- Tools/rbk/rbgu_Utility.sh — utility home for the base64 helpers
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — main specification