# ₣BH Fable Recommendation — Tolerate URLError in fire_delete

Date: 2026-06-10

Status: Recommendation from the Fable review of the cloud-dispatch delete architecture
(commit 4f8a5c703). Small consistency repair in the convergence step.

## The correctable behavior

`fire_delete` (`Tools/rbk/rbgjl/rbgjl06-package-delete.py`) catches only
`urllib.error.HTTPError`. A transient **`urllib.error.URLError`** — connection reset, DNS
blip, broken pipe — propagates uncaught, crashes the step with a traceback, and fails the
whole build.

That is inconsistent with the step's own philosophy: a TCP reset is morally the same event as
a 503, and the design explicitly declares that no single call's verdict matters — "absence is
the only truth; per-call delete errors are logged for the build trail, never branched on."
The convergence loop exists to absorb exactly this class of transient; the catch clause
absorbs only half of it.

## Recommended repair

In `fire_delete`, catch `urllib.error.URLError` alongside `HTTPError` (note `HTTPError` is a
subclass of `URLError`, so order or structure the handler accordingly), log it in the same
`(reconciling via absence poll)` form, and continue. The next round retries; the deadline
remains the failsafe.

**Deliberately do not extend this tolerance to the truth-readers**: `package_absent` and
`list_version_ids` should stay fail-fast (die loud on anything unexpected), because they are
the arbiters the convergence rests on — a flaky reader silently misreporting absence is the
one failure the design must never absorb. If their fail-fast proves noisy under real network
weather, a single bounded retry is the most they should gain.
