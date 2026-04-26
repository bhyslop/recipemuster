## Context

Sibling heat to ₣A_ (rbk-mvp-3-resource-prefix-and-depot-regen). Holds the post-AAK paces whose work is independent of AAE depot regeneration, so they can race in parallel to ₣A_'s live-infra spine. Intended execution pattern: a second officium working in `../rbm_beta_recipemuster` while the primary officium continues ₣A_ in the main tree.

All design decisions are baked into individual dockets — agents executing here should expect mechanical-apply work with explicit file lists and verbatim verification gates.

## Cross-officium discipline

This heat shares the repo with ₣A_; the parallel-tree split is geometric, not semantic. File-level boundaries enforce coordination — a pace's docket may carry an explicit blacklist of files belonging to ₣A_'s active paces, and those constraints persist across directories. When in doubt, additive only: no destructive git operations, no fixing "wrong" repo state without asking, commits land only the file lists each docket specifies.

## Test-suite reservation

During the parallel period, this heat runs fast suite only. Crucible, service, and complete suites are reserved for ₣A_'s live-infra paces — they share regime state and container/network namespaces, and concurrent runs will fail. Once ₣A_'s burn-in clears (or this heat's tail crosses with ₣A_'s wrap), the restriction can lift.

## Live-GCP coordination

Most paces in this heat are pure local refactor or spec churn — no GCP cost. Where a docket does call for live infrastructure, it carries explicit cost notes. Coordinate any billable runs against ₣A_'s active spend; ₣A_ owns the heavier live-infra workload during the parallel period.

## References

- Parent heat ₣A_ paddock — design history for resource prefixing (`RBRR_CLOUD_PREFIX`, `RBRR_RUNTIME_PREFIX`), GAR categorical layout migration (hallmarks/reliquaries/enshrines), payor subdir migration. Vocabulary established there is assumed background.
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — referenced for cross-module shellcheck discipline study.