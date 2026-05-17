# Cloud Build worker-pool wedge on freshly-levied default-quota depot

Pace: heat ₣BO (rbk-11-mvp-tactical), follow-up to ₢BOAAA bench.
Capture window: 2026-05-16 22:54 UTC → 2026-05-17 18:30 UTC.

This memo corrects and supersedes the wedge diagnosis in
`Memos/memo-20260516-cloudbuild-machinetype-bench.md` "Gauntlet has a
latent precondition on a non-standard-2 levy". That memo speculated
"jupyter conjure needs 3 concurrent multi-arch children" and prescribed
a 6-vCPU quota bump as the fix. The substantiated mechanism is
different, the speculation was wrong, and the GCP depots that hold the
evidence will be torn down soon — hence the thoroughness here.

The companion evidence files in this directory preserve the GCP
build/pool state that will be destroyed when the canest depots are
unmade. They are the load-bearing artifacts; the prose below
interprets them.

## TL;DR

A conjure dispatched to a freshly-levied private worker pool can sit
in `QUEUED` indefinitely and eventually transition straight to
`EXPIRED` without ever reaching `WORKING`. Cloud Build's
`queueTtl=3600s` reaps the build at the 1-hour mark, but the agent's
poll loop keeps reporting `QUEUED` until a worker eventually tries to
claim the build and discovers the expiry. The pool itself stays
healthy: subsequent builds on the same pool schedule normally.

The wedge is **not** caused by:
- a multi-vCPU requirement of the build (one build = one e2-standard-2
  worker = 2 vCPUs, exactly matching default quota);
- `RBRR_GCB_MIN_CONCURRENT_BUILDS=3` (this is a preflight *warning*
  threshold at `rbfd_FoundryDirectorBuild.sh:253`, not a gate; it does
  not block dispatch).

The wedge **is** strongly correlated with:
- a freshly-levied depot at the 2-vCPU default quota grant;
- an older queued build on the *other* private pool (airgap probe from
  levy) that drained ~20 minutes after pool creation. The wedge appears
  on the next tether dispatch immediately after that older airgap
  finally clears.

Mechanism is not nailed down past correlation. See "What's still
unknown" below.

## Evidence files (load-bearing)

| File | What it captures |
|------|------------------|
| `may16-wedge-build.json` | The original wedge build's full state. `status=EXPIRED`, `createTime - finishTime ≈ 2h32m`, 97 ms between `startTime` and `finishTime`. |
| `may17-wedge-build.json` | Live reproduction during a controlled tP run on a separate freshly-levied depot. Same dispatch shape, same QUEUED state, same `queueTtl=3600s`. |
| `pool-tether-may16.json`, `pool-airgap-may16.json` | Worker pool configs for the May 16 depot. `machineType=e2-standard-2`, `state=RUNNING`, no `minInstanceCount`/`maxInstanceCount` (defaulted). |
| `pool-tether-may17.json`, `pool-airgap-may17.json` | Same pool configs on today's depot. Identical to May 16 modulo IDs. |
| `timeline-may16.txt` | Full Cloud Build createTime → startTime → finishTime timeline for project `cancbhl-d-canest2bhl100012`. The wedge is the row with status `EXPIRED`. |
| `timeline-may17.txt` | Same for today's depot `cancbhl-d-canest2bhl100013`. Captured mid-wedge. |

## Mechanism, from the May 16 build object

```
id:          20805fb1-8b0c-46df-8da0-a5165d296f6e
status:      EXPIRED
createTime:  2026-05-16T22:54:51.627Z
startTime:   2026-05-17T01:27:29.841Z   (+2h32m38s after create)
finishTime:  2026-05-17T01:27:29.938Z   (+97ms after start)
queueTtl:    3600s                       (Cloud Build expires queued builds older than this)
timeout:     2700s                       (never reached — execution timeout, unused)
pool:        cancbhl-canest2bhl100012-pool-tether
```

Two-and-a-half hours of `QUEUED`, then a 97 ms execution that did
nothing — a worker eventually attempted pickup, found `queueTtl` had
been exceeded long ago, and killed the build immediately. No steps
ran; no `timing` or `timeline` fields are populated.

The agent's outer poll loop (665 polls in
`../temp-buk/temp-20260516-223924-80330-649/rbtd/rbtdro_onboarding_ordain_conjure_jupyter/ordain-jupyter-stdout.txt`)
kept seeing `QUEUED` the whole time because Cloud Build's status field
doesn't transition lazily — the build is `QUEUED` until something tries
to schedule it. The operator's OAuth token expired around poll 665,
delivering the visible `HTTP 401` and ending the gauntlet. The 401 is
a downstream artifact of the wedge having lasted past the token TTL,
not an independent failure mode.

## Mechanism, from the May 17 live reproduction

Controlled reproduction: zeroed regime via `tt/rbw-MZ.MarshalZeroes.sh`
(with MZ's `RBRR_GCB_MACHINE_TYPE` default corrected from `e2-highcpu-32`
to `e2-standard-2` — that fix is commit `57fa7f25`, now on main as
`f7e0a2952` after rebase), then `tt/rbw-tP.QualifyPristine.sh`. The
gauntlet levied depot `cancbhl-d-canest2bhl100013` with default
quota and proceeded through the same fixture sequence as the May 16
failure.

The wedge reproduced exactly:

```
id:          6244f3b1-dfa5-410c-818d-2bd20e841fd6
status:      QUEUED            (still, ~30 min in at memo write time)
createTime:  2026-05-17T17:59:45.527Z
startTime:   null
queueTtl:    3600s
pool:        cancbhl-canest2bhl100013-pool-tether
```

The conjure that wedged is the same one as May 16:
`rbtdro_onboarding_ordain_conjure_jupyter`. Sentry conjure (immediately
prior in the fixture order) ran fine — `QUEUED→WORKING` in 8 polls
(~80 s), `SUCCESS` in ~5 minutes. So the wedge is **not**: "jupyter
specifically is too heavy," because sentry on the same pool with the
same quota scheduled normally; and **not**: "first cloud-build
conjure on a fresh depot wedges," because that first conjure (sentry)
did not wedge.

## The "older queued build clears, next dispatch wedges" pattern

Both timelines show the same shape. Excerpted from `timeline-may16.txt`:

```
22:34:22  tether pool created
22:35:01  airgap pool created
22:35:31  tether probe build dispatched, ran 51s queue + 31s run, FAILURE expected (probe)
22:35:32  airgap probe build dispatched, queued ~20 minutes
22:39:25  inscribe-step (tether), 47s queue, ran 5min — SUCCESS
22:45:37  inscribe-step (tether), 50s queue, ran 4m45s — SUCCESS
22:51:31  inscribe-step (airgap), 50s queue, ran 2m  — SUCCESS
22:54:43  sentry conjure (not in this list — it was a different build on a different fixture run; this is the inscribe-airgap finishing)
22:54:51  jupyter conjure dispatched  ← THE WEDGE BEGINS
22:55:03  the older airgap probe build (queued since 22:35:32) finally starts — 19m31s after dispatch
22:55:40  older airgap probe finishes
... 2h29m of pool idle, no new dispatches by anyone ...
01:24:54  operator restarts the gauntlet (post-quota-bump). New conjure → 51s queue → WORKING.
01:27:29  the wedged build is finally checked by a worker, found expired, killed in 97ms.
```

And from `timeline-may17.txt` (today, controlled run):

```
17:40:40  tether probe (levy)  →  54s queue, 88s wall, FAILURE (probe is supposed to)
17:40:41  airgap probe (levy)  →  queued ~19 minutes
17:44:41  inscribe (tether)    →  52s queue, SUCCESS
17:51:15  inscribe (tether)    →  49s queue, SUCCESS
17:56:41  inscribe (airgap)    →  41s queue, SUCCESS  (jumped ahead of the older airgap probe)
17:59:45  jupyter conjure dispatched (tether)  ← WEDGE BEGINS
17:59:52  older airgap probe (queued since 17:40:41) finally starts — 19m11s after dispatch
18:00:31  older airgap probe finishes
18:30+ (memo time) jupyter conjure still QUEUED; queueTtl=3600s expires at 18:59:45
```

The pattern is exact and worth a name: **late-clearing probe collision**.
Both depots' airgap probe build (a levy artifact — see
`rbgp_Payor.sh:912ish`, "Submit per-pool probe builds (materialize quota
rows + assert egress posture + GAR write)") sits in queue ~19 minutes
behind tether activity, then drains right at the moment a tether
conjure is dispatched. The tether conjure is the one that wedges.

Whether the probe-vs-conjure collision is causal or just correlated is
not established. What's clear:
- the wedge is reproducible across two independently-levied depots,
- the wedge pattern locks in on the *next* tether dispatch after the
  older airgap probe drains,
- once the tether build wedges, the pool keeps accepting and running
  *other* builds normally — except the wedged one. This is shown
  cleanly in `timeline-may16.txt`'s last column: from 01:24:54 onward,
  many tether builds run with normal ~50 s queue times. The wedge does
  not poison the pool, only the one stuck build.

## Worker pool config

Both depots, both pools, identical:

```
privatePoolV1Config:
  workerConfig:
    machineType:  e2-standard-2
    diskSizeGb:   100
  networkConfig:
    egressOption: PUBLIC_EGRESS  (tether)  /  NO_PUBLIC_EGRESS (airgap)
state: RUNNING
```

No `minInstanceCount`, no `maxInstanceCount` — defaulted. Default for
`maxInstanceCount` on private pools is 1, per Cloud Build docs (not
verified against API response, just recalled). If true, the pool can
host **exactly one running build at a time** regardless of project
quota — which matters for the "collision" theory below.

## Quota numbers, for the record

Both depots received `concurrent_private_pool_build_cpus = 2` at levy
time — the new low-default-quota regime Google instituted. The
preflight warning fires correctly:

```
Build quota insufficient: 2 vCPU quota / 2 vCPUs per build = 1 concurrent (need 3)
Fresh depots start with a low quota. After some build activity,
the Edit Quotas option becomes available.
```

`RBRR_GCB_MIN_CONCURRENT_BUILDS=3` is the source of `(need 3)`. It is
a configured threshold, not a measured requirement of any build.

The May 16 wedge was followed by an operator quota bump to 6 vCPU
(documented in the bench memo). After the bump, the gauntlet succeeded.
That outcome — "bumping quota unblocks the wedge" — is consistent with
the *original* theory ("we need 3 concurrent builds") but also
consistent with **a different**, more plausible theory:

**At quota=2 vCPU, the project supports exactly one concurrent build.
At quota=6 vCPU (with two pools × e2-standard-2 = 4 vCPU steady-state
plus headroom), the project comfortably supports both pools running
simultaneously without scheduler contention.** Bumping quota broke
whatever scheduler-state pathology held the wedged build in
indefinite QUEUED — possibly by giving Cloud Build's scheduler enough
slack to revisit older queued builds, possibly via some autoscaler
side-effect at the pool level.

So: the operator's bump-quota-to-6 workaround is **load-bearing in
practice** and should not be retired before we understand the
mechanism. But the bench memo's *justification* for it ("jupyter needs
3 concurrent multi-arch children") is wrong and should not be the
basis for any future design.

## What is and isn't dispatched by a conjure

For the record, since the bench memo got this wrong:

- A single conjure (jupyter included) submits **one** Cloud Build job
  via `builds.create` (see `rbfd_FoundryDirectorBuild.sh:1282`-ish).
- That job runs a serial sequence of steps:
  `rbgjb01-derive-tag-base` → `rbgjb02-qemu-binfmt` (if multi-platform)
  → `rbgjb03-buildx-push-image` → `rbgjb04-per-platform-pullback`
  → `rbgjb05-push-per-platform` → `rbgjb06-push-diags`.
- All steps run on **one** worker pool VM (e2-standard-2 → 2 vCPU).
- Multi-platform output (`linux/amd64,linux/arm64` for jupyter) is
  produced by `docker buildx build --platform=...` inside step 03,
  running both platforms on the same VM via QEMU emulation. This is
  not three Cloud Build jobs — it is one job, one VM, one buildx
  invocation that internally orchestrates per-platform builders.

The bench memo's claim "needs 3 concurrent multi-arch children" has no
support in the dispatch path.

## What's still unknown

The wedge mechanism past the correlation is speculative. Specifically:

1. **Why does the freshly-levied pool fail to schedule the wedged
   build, when it cheerfully schedules every other build on the same
   pool before *and* after?** The wedge is a per-build property, not a
   per-pool property. Something specific to the wedged build's state
   makes Cloud Build's scheduler refuse to allocate it a worker —
   despite the worker pool being healthy and idle. Hypotheses,
   unranked:
   - The build is dispatched while the pool is mid-scale-up after the
     older probe finally cleared. The scheduler associates the new
     build with a worker that doesn't materialize, and never re-runs
     the assignment.
   - A quota-row state machine: the older probe consumed a quota slot
     "ledger row" that gets stuck or duplicated when the slot
     transitions ownership. The wedged build is told quota is held by
     a worker that doesn't exist.
   - `maxInstanceCount=1` (default) plus a race between the probe's
     worker tearing down and the wedge's worker booting up. The
     scheduler may track instance count via TTL and refuse new
     assignments while in transition.

2. **Why does the operator quota bump from 2→6 reliably break the
   wedge?** Plausible: more quota = more scheduler slack to retry
   assignment, more concurrent slots so the late-clearing probe doesn't
   monopolize the single available slot at the wrong moment.

3. **Does the wedge ever clear without operator intervention?** No
   evidence either way. The May 16 wedge expired at queueTtl. Nobody
   waited longer to see if a worker would eventually retry. The May 17
   wedge is in flight at memo write time — its final state will be
   captured to this directory when tP completes.

4. **Is the wedge specific to the conjure-after-probe-drain pattern,
   or would any tether dispatch in that exact moment wedge?** A
   targeted reproduction (drive a non-conjure tether build into that
   window) would isolate this. Not done.

## Mitigation paths, ranked by tractability

1. **Document the quota bump as a required step in levy completion.**
   This is what the bench memo's `₢BOAAI` (`post-levy-quota-bump-flow`,
   slated post-rebase) intends. The workaround is empirical and
   reliable; codifying it removes the surprise. **Cost: cheap.
   Status: slated.**

2. **Drain the airgap probe before exiting levy.** Right now the levy
   completes once both probes are *dispatched*, but the airgap probe
   sits in the queue for ~20 minutes after — see both timelines. If
   levy waited for both probes to terminate before declaring "done,"
   the late-clearing collision would not happen at the next dispatch.
   **Cost: medium (changes levy completion semantics; extends levy
   wall-clock by ~20 minutes).**

3. **Set `queueTtl` shorter than the gauntlet poll loop's effective
   timeout.** Currently `queueTtl=3600s` and the poll loop is 960
   polls of (probably) 10 s = 9600 s. If `queueTtl` were e.g. 600 s
   (10 min), the agent would see EXPIRED on the first wedge, abort
   early with a clear diagnostic, and the operator would see a
   30-minute aborted gauntlet instead of a 1-hour blind hang plus a
   2-hour OAuth-expired death. **Cost: cheap. Tradeoff: legitimate
   slow scale-ups would also expire.**

4. **Investigate `maxInstanceCount` explicit configuration.** Pool
   creation in `rbgp_depot_levy` (see `rbgp_Payor.sh:895/921`) does
   not specify `maxInstanceCount`. Setting it explicitly to something
   ≥ 2 might give Cloud Build's scheduler enough flex to avoid the
   wedge. **Cost: cheap. Risk: unknown — different pool sizing might
   have downstream surprises.**

## Pointers

- Bench memo (theory now superseded for the wedge subsection):
  `Memos/memo-20260516-cloudbuild-machinetype-bench.md`.
- Original wedge trace (will outlive depot teardown — it's local):
  `../temp-buk/temp-20260516-223924-80330-649/rbtd/rbtdro_onboarding_ordain_conjure_jupyter/ordain-jupyter-stdout.txt`.
- Current live wedge trace:
  `../temp-buk/temp-20260517-173010-159370-846/rbtd/rbtdro_onboarding_ordain_conjure_jupyter/` (populated after tP terminates the case).
- MZ fix that aligned MZ's machineType default with the bench memo's
  locked decision: commit `57fa7f25` (now `f7e0a2952` after rebase).
- Quota preflight emitting the warning that was misinterpreted as a
  gate: `Tools/rbk/rbfd_FoundryDirectorBuild.sh:189` (`zrbfd_quota_preflight`).
- Bench memo's slated remediation for this class of issue: `₢BOAAI`
  (`post-levy-quota-bump-flow`), per the bench memo's "Decision
  implications" and the pre/post-rebase notes in `c5a2a9978`.

## How to recognize this if it recurs

Symptoms:
- A conjure (any conjure) sits at `QUEUED (poll N/960)` for hundreds of
  polls without transitioning to WORKING.
- Depot is fresh-levied; `rbfd_ordain` printed the quota insufficiency
  warning at preflight.
- A `gcloud builds describe ${BUILD} --project=${DEPOT} --region=...`
  shows `status: QUEUED` with `createTime` far in the past and no
  `startTime`. After queueTtl, the same query will show `status:
  EXPIRED` with `startTime ≈ finishTime` (sub-second gap).
- Inspecting the Cloud Build console build list (or the API
  `builds?pageSize=50` endpoint) shows: an older airgap probe build
  draining ~20 minutes after pool creation, with the wedge being the
  next dispatched tether build immediately after that probe clears.

First-line recovery (until ₢BOAAI lands):
- Operator bumps `concurrent_private_pool_build_cpus` quota for the
  depot project from 2 to ≥ 6 in Cloud Console.
- Re-dispatch the conjure. The wedged build will expire at queueTtl
  but is otherwise harmless.
