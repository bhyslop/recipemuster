# ₣BH Fable Recommendation — Progress-Aware Convergence Deadline

Date: 2026-06-10

Status: Recommendation from the Fable review of the cloud-dispatch delete architecture
(commit 4f8a5c703). Latent for today's package shapes; wrong-shaped for large webs.
TRIAGED 2026-06-10: declined for now — recorded as an RBSHR horizon entry (progress-aware convergence deadline); revisit if captured web sizes grow an order of magnitude.

## The correctable behavior

`converge_delete` (`Tools/rbk/rbgjl/rbgjl06-package-delete.py`) enforces a **fixed**
per-package ceiling, `DELETE_DEADLINE_SEC = 180`, measured from the package's start.

`fire_delete` is synchronous HTTP (~100–300ms per call), so one round over a large web costs
roughly `versions × latency`. A ≥1000-version package can spend its entire 180s inside
**round 1** — still making monotonic progress — and then die at the deadline check with
"delete did not converge," killing a delete that was converging. Today's largest observed web
(55 versions, ~11–16s/round) is far from the edge, but the deadline shape penalizes exactly
the packages that most need convergence.

Related budget edge, same shape: worst case is `N_packages × 180s` sequential in-pool
(`main()` loops `converge_delete`), against `RBRR_GCB_TIMEOUT=2700s` — 15 pathological
packages exceeds the build timeout. (Host-side ordering is correct and should be preserved:
`ZRBFC_BUILD_POLL_CEILING_DELETE=600 × 5s = 3000s > 2700s`, so the build always dies first
and the host sees a clean terminal FAILURE.)

## Recommended repair

Make the deadline progress-aware rather than purely chronological, in `converge_delete`:

- Track the remaining-version count per round (`list_version_ids` already supplies it).
- Die only when a round completes with **no reduction** in remaining count — two consecutive
  no-progress rounds is a reasonable trigger (mirrors the loop-until-dry shape).
- Retain an absolute ceiling as the backstop, scaled to the work:
  e.g. `max(180, initial_versions × 1s + 120)` — never unbounded.

Document the abjure-size budget in `zrbld_cloud_delete_dispatch` (`Tools/rbk/rbldd_Delete.sh`)
or RBSCB: per-package ceilings must sum inside `RBRR_GCB_TIMEOUT`; beyond that, either raise
the build timeout for delete builds specifically or split the dispatch into batched builds.
