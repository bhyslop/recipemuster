# ₣BH Fable Recommendation — Bound urllib Calls with a Socket Timeout

Date: 2026-06-10

Status: Recommendation from the Fable review of the cloud-dispatch delete architecture
(commit 4f8a5c703). Cheap hang-bounding in the convergence step.

## The correctable behavior

Every `urllib.request.urlopen` call in `Tools/rbk/rbgjl/rbgjl06-package-delete.py` —
`metadata_token()` and `gar_fetch()` (which carries `package_absent`, `list_version_ids`,
and `fire_delete`) — passes **no timeout**. Python's default is `None`: an unbounded block
on a hung socket.

The failsafes that eventually fire are the wrong grain: a single hung GET stalls the entire
delete until the Cloud Build timeout (`RBRR_GCB_TIMEOUT=2700s`) or the host poll ceiling
kills the build — turning one stuck connection into a full-build failure with no log line
naming the stall.

## Recommended repair

Pass an explicit timeout to `urlopen` in `gar_fetch` and `metadata_token` — 30s is generous
for both the metadata server and GAR REST.

Classify the resulting `socket.timeout` / `TimeoutError` by caller, consistent with the
URLError-tolerance recommendation (companion memo):

- In `fire_delete`: tolerated and logged like any other per-call error — the absence poll
  arbitrates.
- In the truth-readers (`package_absent`, `list_version_ids`) and `metadata_token`: die loud
  (or one bounded retry) — a timed-out read names its stall in the build log instead of
  freezing silently to the build timeout.
