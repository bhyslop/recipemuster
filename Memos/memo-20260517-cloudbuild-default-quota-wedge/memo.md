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

## Addendum (2026-05-17 ~18:50 UTC): anchor test pinpoints code regression

After the body above was written, an anchor test ran the gauntlet from
an earlier commit known to pass cleanly. **Result: pass.** That changes
the diagnosis — the wedge is not environmental drift on Google's side,
it is a regression we introduced in our own code.

### Regression bounds (precise)

- **Last known good: `ad3ea5e22`** — 2026-05-14 22:10 UTC, "Pristine
  qualification passed" in `../logs-buk/hist-rbw-tP-sh-20260514-203134-1368806-985.txt`.
- **First known bad: `e14654739` or later** — 2026-05-15 15:10 UTC at
  the earliest. The bench memo's wedge on `canest2bhl100012` (2026-05-16
  ~22:54 UTC) is in this commit's descendants. Today's reproduction on
  `canest2bhl100013` (2026-05-17 ~17:59 UTC) is also.
- **Anchor test of record:** branch `temp-bisect-may14`, started from
  `ad3ea5e22`, no cherry-picks, MZ no-op (anchor commit was itself a
  marshal-zero state), fresh canest levy at default 2-vCPU quota, full
  tP gauntlet. Outcome: `Suite 'gauntlet' complete (12 fixtures)` and
  `Pristine qualification passed`.

### Trial counts so far

- `ad3ea5e22` against today's Google: 1 trial, 1 pass.
- `f7e0a2952` (HEAD before today's memo commits) against today's Google:
  2 trials, 2 wedges (2026-05-16 bench + 2026-05-17 reproduction).

One pass is not statistically conclusive against a possibly-probabilistic
scheduler. But a 0/2 → 1/1 swing across a tractable code window is
strong enough to commit to "code regression" as the working hypothesis
and bisect within those commits.

### The 12-commit search space

Restricted to `Tools/rbk/` (everything else is test framework / docs /
kit infra unlikely to affect Cloud Build dispatch). Of 12 commits, only
two touch the Cloud-Build dispatch path:

| Commit | Date | Touches | Relevance |
|--------|------|---------|-----------|
| `e14654739` | 05-15 | `rbgp_Payor.sh` (+146), `rbgc_Constants.sh` (+24), `rbgl_GarLayout.sh`, `RBSDE-depot_levy.adoc` | **Adds per-pool probe build submission to levy.** Direct cause candidate. |
| `df0fb2651` | 05-16 | `rbgp_Payor.sh`, `RBSDE-depot_levy.adoc` | Wraps `workerPools.create` with LRO-await. Timing-of-pool-readiness change. |
| `fc179e39f` | 05-16 | `rbfd_FoundryDirectorBuild.sh` | Preflight diagnostic refinement (airgap empty-anchor). Branches a diagnostic path; does not touch dispatch. |
| `a7a98cd87` | 05-15 | `rblm_cli.sh` | MZ default changed to `e2-highcpu-32`. Reverted today by `f7e0a2952`. Both states resolve to `e2-standard-2` at bench/today, so this doesn't gate the wedge. |
| `37e454344` | 05-14 | `rbtdro_onboarding.rs` | Renames onboarding fixture cases (cosmetic). |
| `f7e0a2952` | 05-17 | `rblm_cli.sh` | Today's MZ-default revert. |
| `02d3439f4`, `4edd23206`, `fa2a6794f`, `331579824` | 05-15 | Crucible charge / `rbnnh_` per-nameplate hooks | Local crucible mechanics, not Cloud Build path. |
| `d7145e0c0` | 05-16 | `rbtd/` Rust | Theurge RCG cleanup, log macros. No dispatch impact. |
| `00d99794b` | 05-16 | `rbtd/` Rust | Theurge BURV sandbox anchoring. No dispatch impact. |

### Four theories, ranked most→least likely

**Theory 1 — `e14654739` (per-pool probe submission) is the cause.**

The commit adds 146 lines of new behavior at levy time: each pool gets
a Cloud Build submitted that materializes the quota row, asserts egress
posture, and pushes a marker to `rbi_df`. Two probe builds dispatch
within seconds of pool creation — *before* any other gauntlet work.

The "late-clearing-probe-collision" pattern documented above in the
main body is *exactly this commit's behavior*. Both timeline files
(`timeline-may16.txt`, `timeline-may17.txt`) show the airgap probe
sitting queued for ~19 minutes and draining at the exact moment the
next tether conjure is dispatched. The wedge bites that next tether
conjure. At `ad3ea5e22` no probes exist, no late-clearing collision is
possible, no wedge observed.

Plausible micro-mechanisms (any or all):
- The airgap probe holds quota state in Cloud Build's scheduler in a
  way that, when it finally releases, leaves a dangling slot that the
  next dispatch can't bind to.
- Project-scoped 2-vCPU quota means tether probe + queued airgap probe
  + queued inscribes + the first tether conjures all compete in a
  single 2-vCPU slot; the wedge candidate is the unlucky build that
  arrives during a transition.
- The probe writes to `rbi_df` (a brand-new GAR category); something
  about IAM propagation or repository-readiness for that category may
  interact with the worker bring-up.

Confidence: high. Mechanism + correlation + reach (this is the only
commit that touches the dispatched-builds queue at levy time).

**Theory 2 — `df0fb2651` (LRO-await for `workerPools.create`) is the cause.**

The commit changes `workerPools.create` to wait for the LRO to reach
its terminal `RUNNING` state before levy proceeds. Previously the levy
moved on after the POST returned the operation handle, allowing
subsequent steps (including probe submission per Theory 1) to fire
against a pool that was technically still spinning up.

If the wedge is sensitive to *when* a build is dispatched relative to
pool-state transitions, the LRO-await change could be the culprit: it
made pool-vs-build timing more deterministic in a way that exposes the
wedge. Without LRO-await (pre-`df0fb2651`), probes might have hit
not-yet-ready pools and been silently absorbed; with LRO-await, they
hit just-became-ready pools and tickle a scheduler edge.

Confidence: medium. Direct mechanism is weaker than Theory 1, but this
is one of only two commits that touch the right code.

**Theory 3 — combined effect of `e14654739` + `df0fb2651`.**

The two commits landed close together (May 15 evening → May 16) and
were both authored under bench-pace work in ₣BO. Each in isolation
might be benign; together they create the conditions for the wedge —
probes dispatch deterministically against a just-ready pool, and the
scheduler's response is the wedge.

This is mechanically the same as Theory 1 + Theory 2 stacked, but
worth its own line because reverting just one of the two may not
suffice. A bisect that reverts `e14654739` alone and still wedges
should immediately suspect this theory before declaring `df0fb2651`
the culprit.

Confidence: medium. Distinguishable from Theory 1 only by experiment.

**Theory 4 — something else, unidentified in the remaining 10 commits.**

The residual commits touch: crucible charge mechanics, the test
framework's Rust internals, per-nameplate hooks, MZ defaults, fixture
renames, and preflight diagnostics. None of these are *expected* to
affect Cloud Build worker-pool scheduling. But a subtle interaction —
an env var that leaks into the build context, a shellcheck-driven
refactor that altered an `if` arm by one character, a build-step body
that's now a few bytes longer — could in principle change worker
behavior without being obvious.

Confidence: low. Listed for humility; if Theories 1–3 all bisect away
without resolving the wedge, this is where to look next.

### Suggested next experiment

Bisect within the 2-commit window first. Cheapest first cut: revert
`e14654739` on a branch off current main, run tP, observe. If wedge
clears, Theory 1 is confirmed. If wedge persists, suspect Theory 2 or
3 and test by reverting `df0fb2651` (alone, or together with
`e14654739` reverted).

The probes themselves serve a real purpose per `e14654739`'s commit
message ("materializes its quota row," "asserts intended egress
posture"). If Theory 1 is confirmed, the fix is probably not "remove
probes" but "submit probes serially, or one at a time, or wait between
them so they don't collide." That design conversation belongs to a
follow-on pace, not this memo.

## Addendum (2026-05-18 ~01:00 UTC): Theory 1 confirmed, proximate cause is fire-and-forget probe submission

**Theory 1 experiment ran on main with `e14654739` reverted (revert
commit `936a7607b`). Outcome: `Suite 'gauntlet' complete (12 fixtures)`
and `Pristine qualification passed` on a fresh canest depot
(`canest2bhl100015`) at default 2-vCPU quota.** Same Google environment
as the wedges, same fixture sequence, same machine type. Just the probe
feature removed.

Updated trial tally:

| Config | Trials | Passes | Wedges |
|--------|--------|--------|--------|
| No probes (`ad3ea5e22`) | 1 | 1 | 0 |
| No probes (revert on main, `67604bdff`) | 1 | 1 | 0 |
| Probes active (`f7e0a2952`) | 2 | 0 | 2 |

Two passes with probes removed across two different commits; two
wedges with probes active across two independent depots. Theory 1
holds.

### Proximate cause: levy is fire-and-forget on probes

Inspection of `e14654739:Tools/rbk/rbgp_Payor.sh` line ~614 confirms
the probe submission is non-awaiting. The relevant code path:

```bash
rbgu_http_json "POST" "${z_build_url}" "${z_token}" "${z_infix}" "${z_build_file}"
z_submit_code=$(rbgu_http_code_capture "${z_infix}")
# ...
case "${z_submit_code}" in
  200|201)
    z_build_id=$(rbgu_json_field_capture "${z_infix}" '.metadata.build.id')
    buc_info "  Build:    ${z_build_id} (submitted — runs async)"
    ;;
  400) ... ;;
  *)   buc_die ... ;;
esac
```

The "submitted — runs async" log line is explicit: after submission,
control returns to `rbgp_depot_levy`, which returns to the gauntlet
fixture, which marks `rbtdrk_depot_levy` PASSED and proceeds to the
next case. **Two builds are now in `QUEUED` with no one awaiting
them.**

### Why fire-and-forget causes the wedge

Combined with the 2-vCPU project-scoped quota:

1. Tether probe and airgap probe both submitted within 1 second.
2. Tether probe gets the project's 2-vCPU slot first (alphabetical or
   submit-order priority — empirically tether wins).
3. Airgap probe queues.
4. Tether probe runs ~30 s and finishes. Slot freed.
5. Levy returns. Gauntlet proceeds. `canonical-establish` →
   `onboarding-sequence`. The first inscribe-reliquary build dispatches
   on the tether pool, takes the just-freed slot, runs ~5 minutes.
6. Each subsequent inscribe / kludge / sentry-conjure build similarly
   steals the slot the airgap probe should have gotten next.
7. ~19 minutes after levy started, the gauntlet runs out of contending
   builds momentarily. Airgap probe finally gets a slot, runs, finishes.
8. **At the exact moment the airgap probe finishes**, the next tether
   conjure (jupyter) is dispatched. It enters `QUEUED` and never
   transitions to `WORKING`. Eventually expires at `queueTtl=3600s`.

The 0-vs-2 pass/wedge split is consistent with this mechanism: when
no probes exist, no late-clearing collision is possible; when probes
exist without an await, the collision is statistically near-certain
because the gauntlet immediately follows the levy with a rapid burst
of dispatches.

### The repair

**Synchronize the levy on probe terminal state.** After submitting a
probe build via `builds.create`, poll its status until any terminal
state (SUCCESS | FAILURE | CANCELLED | TIMEOUT | EXPIRED |
INTERNAL_ERROR). All terminal states are acceptable — the probe is
designed to FAILURE (HTTP 400 inside the build asserts egress
posture); what matters is that the levy *waits* for that terminal
status before returning to the caller.

Open sub-decision (slated for the repair pace, not this memo): submit
both probes in parallel then await both, vs. submit-await-submit-await
serial. Parallel is preferred because:
- Wall-clock cost = `max(tether_wall, airgap_wall)` (parallel) vs.
  `tether_wall + airgap_wall` (serial). Airgap is the long pole
  regardless.
- The wedge cause is *external dispatchers stealing the airgap
  probe's slot*, not the two probes contending with each other. With
  no external dispatchers running during the await window, the two
  probes coexist fine on a 2-vCPU quota (tether finishes, then
  airgap gets the slot — same dynamics as before, just without the
  inscribe builds racing).

The probe's three documented purposes (quota row materialization,
egress posture assertion, `rbi_df` marker push) all survive this
repair — the probe still gets to do its work, the levy just waits
for it.

### Cost of the repair

Levy wall-clock grows by the airgap probe's runtime, which the
timelines suggest is dominated by airgap pool warm-up (~10–15 min on
fresh-levy) plus the probe's own ~30 s execution. So levy goes from
~2 minutes to ~15 minutes. That cost is *already being paid* — it
just currently manifests as a wedge in `ordain_conjure_jupyter` (60+
min wall-clock to discover, requires operator intervention) rather
than as a longer levy step (15 min wall-clock, no operator action
needed). The repair moves the cost to the right place.

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
