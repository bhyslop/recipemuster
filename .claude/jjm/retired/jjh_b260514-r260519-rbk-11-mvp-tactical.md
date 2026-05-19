# Heat Trophy: rbk-11-mvp-tactical

**Firemark:** ₣BO
**Created:** 260514
**Retired:** 260519
**Status:** retired

## Paddock

## Character

Tactical heat. Opportunistic fixes and measurements that surface during
release qualification without being part of the release-finalize path.
Each pace ships a discrete deliverable.

## Environment

- Canest depot: `cancbhl-d-canest2bhl100011`, with
  `RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` set. 96-core quota in effect;
  cannot be re-levied at default 2-cpu without forfeiting the quota.
- Mac credentials freshened 2026-05-16 via local payor → governor mantle →
  director/retriever investment. Live identities: `director-bhl` and
  `retriever-bhl` on canest 100011. Cerebro's prior `director-bhl` key
  was revoked by the re-investment.
- Cerebro's older `e2-standard-2` depot is GONE. No `gcloud builds list`
  / `describe` path to any baseline build records exists. Baseline
  analysis is log-only.

## Reference data

- May 14 cerebro clean-gauntlet fO logs (canonical baseline,
  whole-conjure granularity):
  `~/projects/rbm_alpha_recipemuster/../logs-buk/hist-rbw-fO-sh-20260514-2*.txt`
  on cerebro. Pulled to Mac at `/tmp/bench-bo/` (temp dir; refresh via
  scp if missing).
- Gauntlet rollup: `hist-rbw-tP-sh-20260514-203134-1368806-985.txt`.
- macOS-local 11-fixture clean passes: May 6 and May 12 (cross-host
  sanity reference).

## Constraints

- No production-config changes from tactical paces unless explicitly
  scoped. Machine-type rollback (if recommended) lands in a follow-up
  pace, not here.
- 96-core quota stays intact — no depot levy/unmake.

## Paces

### levy-establish-probes (₢BOAAD) [complete]

**[260515-1536] complete**

## Character
After levy creates pools and GAR, probe each pool with a Cloud Build
submission that materializes the quota row, asserts the pool's intended
egress posture, and (lean: yes) exercises GAR write.

## Shape
- Builder image: Google-hosted only. Airgap can't reach docker.io.
- Fresh-levy expected outcome: HTTP 400 at quota gate. This is the
  desired materialization side-effect, not a failure to surface.
- Egress-posture asserted in-build:
    tether — public reachable AND Google reachable
    airgap — public blocked AND Google reachable
  Assertion failure is a hard infra-drift signal.
- GAR push (if included): under a probe-namespace path that the
  hallmark / reliquary / enshrinement enumerators ignore.

## What done looks like
Levy emits a per-pool probe block. Onboarding handbook points to a
console where the quota row is populated, both pools have build records,
and (if push) the probe artifacts are visible at their isolated path.

**[260515-1431] rough**

## Character
After levy creates pools and GAR, probe each pool with a Cloud Build
submission that materializes the quota row, asserts the pool's intended
egress posture, and (lean: yes) exercises GAR write.

## Shape
- Builder image: Google-hosted only. Airgap can't reach docker.io.
- Fresh-levy expected outcome: HTTP 400 at quota gate. This is the
  desired materialization side-effect, not a failure to surface.
- Egress-posture asserted in-build:
    tether — public reachable AND Google reachable
    airgap — public blocked AND Google reachable
  Assertion failure is a hard infra-drift signal.
- GAR push (if included): under a probe-namespace path that the
  hallmark / reliquary / enshrinement enumerators ignore.

## What done looks like
Levy emits a per-pool probe block. Onboarding handbook points to a
console where the quota row is populated, both pools have build records,
and (if push) the probe artifacts are visible at their isolated path.

### highcpu-build-substep-bench (₢BOAAA) [complete]

**[260517-1558] complete**

## Character

Three-phase A/B of Cloud Build machineType (highcpu-32 vs standard-2)
across the canest2 depot family. Phase 1+2 measure highcpu-32 on
canest2bhl100011 (pool-time finding from ₢BLAAA empirically reconfirmed:
rbrr.env flip without re-levy is a no-op for build wall clock). Phase 3
adds a second depot canest2bhl100012 at standard-2 via the gauntlet's
establish+onboarding fixtures. Memo synthesizes the per-workload trade-off
of the locked decision to revert to standard-2. Posture: intricate but
mechanical, with judgment in memo synthesis.

## What's landed

- Setup commits: forge enshrined; airgap anchor pinned to forge hallmark
  c260516175159-r260516175203 (canest2bhl100011 GAR).
- Phase 1 (e2-highcpu-32 rbrr.env): 11/11 SUCCESS.
- Flip commit a1a6112c: RBRR_GCB_MACHINE_TYPE=e2-standard-2.
- Phase 2 (e2-standard-2 rbrr.env, same warm pool): 11/11 SUCCESS;
  per-workload wall-clocks within 5-15s of Phase 1 — confirms pool-time
  finding empirically.

## Phase 3 — second-depot measurements on real standard-2 pool

  1. tt/rbtd-r.FixtureRun.canonical-establish.sh
     Levies canest2bhl100012 with current rbrr.env machineType
     (e2-standard-2), mantles governor, invests canest-ret + canest-dir
     SAs. Fixture mutates and commits rbrr.env RBRR_DEPOT_MONIKER.

  2. tt/rbtd-r.FixtureRun.onboarding-sequence.sh
     Inscribes reliquary; kludges sentries; ordains conjure sentry,
     conjure jupyter, airgap chain (forge+airgap), bind plantuml,
     graft demo. First run of each workload — treat as warmup the way
     Phase 1/2 did.

  3. Manually dispatch the remaining workloads to match Phase 1/2:
       - jupyter conjure ×1 (data)
       - forge conjure ×1 (data)
       - airgap conjure ×2 (data 1, data 2)
       - rbev-busybox conjure ×1
       - tt/rbw-fV.DirectorVouchesHallmarks.sh ×1

## Phase 3 cleanup

Revert rbrr.env: RBRR_DEPOT_MONIKER=canest2bhl100011 and
RBRR_GCB_MACHINE_TYPE=e2-highcpu-32. Restore airgap RBRV_IMAGE_1_ANCHOR
to the 100011 forge hallmark. Both depots remain alive at pace close;
operator unmakes 100012 later when expressly chosen.

## Extraction

Grep rbfd_ordain / rbfd_enshrine event lines from each phase's logs.
Wall-clock per operation = timestamp delta between opening event
(Building vessel image / Loaded vessel) and matching terminal SUCCESS
line.

## Outcome

Memo at Memos/memo-20260516-cloudbuild-machinetype-bench.md:
  - Carries ₢BLAAA's pool-time finding (now empirically reconfirmed by
    Phase 2's identical timings to Phase 1).
  - Per-workload table: highcpu-32 (Phase 1+2 combined, n=2 or n=3 per
    workload) vs standard-2 (Phase 3).
  - Profile groupings: CPU-bound conjures, registry-I/O mirror, mixed
    about+vouch and vouch-chain.
  - Speedup ledger: what highcpu-32 delivers per workload, so a future
    "should we go back?" decision has data.
  - May 14 cerebro logs: cross-host sanity check (downgraded from
    primary 2-cpu reference).

## Constraints

  - Both canest2bhl100011 and canest2bhl100012 alive at pace close.
  - Linear commits on main (no branch). Fixtures commit rbrr.env
    mutations as part of their flow.
  - Sequential dispatch only (BUK discipline; shared regime state).

**[260516-2232] rough**

## Character

Three-phase A/B of Cloud Build machineType (highcpu-32 vs standard-2)
across the canest2 depot family. Phase 1+2 measure highcpu-32 on
canest2bhl100011 (pool-time finding from ₢BLAAA empirically reconfirmed:
rbrr.env flip without re-levy is a no-op for build wall clock). Phase 3
adds a second depot canest2bhl100012 at standard-2 via the gauntlet's
establish+onboarding fixtures. Memo synthesizes the per-workload trade-off
of the locked decision to revert to standard-2. Posture: intricate but
mechanical, with judgment in memo synthesis.

## What's landed

- Setup commits: forge enshrined; airgap anchor pinned to forge hallmark
  c260516175159-r260516175203 (canest2bhl100011 GAR).
- Phase 1 (e2-highcpu-32 rbrr.env): 11/11 SUCCESS.
- Flip commit a1a6112c: RBRR_GCB_MACHINE_TYPE=e2-standard-2.
- Phase 2 (e2-standard-2 rbrr.env, same warm pool): 11/11 SUCCESS;
  per-workload wall-clocks within 5-15s of Phase 1 — confirms pool-time
  finding empirically.

## Phase 3 — second-depot measurements on real standard-2 pool

  1. tt/rbtd-r.FixtureRun.canonical-establish.sh
     Levies canest2bhl100012 with current rbrr.env machineType
     (e2-standard-2), mantles governor, invests canest-ret + canest-dir
     SAs. Fixture mutates and commits rbrr.env RBRR_DEPOT_MONIKER.

  2. tt/rbtd-r.FixtureRun.onboarding-sequence.sh
     Inscribes reliquary; kludges sentries; ordains conjure sentry,
     conjure jupyter, airgap chain (forge+airgap), bind plantuml,
     graft demo. First run of each workload — treat as warmup the way
     Phase 1/2 did.

  3. Manually dispatch the remaining workloads to match Phase 1/2:
       - jupyter conjure ×1 (data)
       - forge conjure ×1 (data)
       - airgap conjure ×2 (data 1, data 2)
       - rbev-busybox conjure ×1
       - tt/rbw-fV.DirectorVouchesHallmarks.sh ×1

## Phase 3 cleanup

Revert rbrr.env: RBRR_DEPOT_MONIKER=canest2bhl100011 and
RBRR_GCB_MACHINE_TYPE=e2-highcpu-32. Restore airgap RBRV_IMAGE_1_ANCHOR
to the 100011 forge hallmark. Both depots remain alive at pace close;
operator unmakes 100012 later when expressly chosen.

## Extraction

Grep rbfd_ordain / rbfd_enshrine event lines from each phase's logs.
Wall-clock per operation = timestamp delta between opening event
(Building vessel image / Loaded vessel) and matching terminal SUCCESS
line.

## Outcome

Memo at Memos/memo-20260516-cloudbuild-machinetype-bench.md:
  - Carries ₢BLAAA's pool-time finding (now empirically reconfirmed by
    Phase 2's identical timings to Phase 1).
  - Per-workload table: highcpu-32 (Phase 1+2 combined, n=2 or n=3 per
    workload) vs standard-2 (Phase 3).
  - Profile groupings: CPU-bound conjures, registry-I/O mirror, mixed
    about+vouch and vouch-chain.
  - Speedup ledger: what highcpu-32 delivers per workload, so a future
    "should we go back?" decision has data.
  - May 14 cerebro logs: cross-host sanity check (downgraded from
    primary 2-cpu reference).

## Constraints

  - Both canest2bhl100011 and canest2bhl100012 alive at pace close.
  - Linear commits on main (no branch). Fixtures commit rbrr.env
    mutations as part of their flow.
  - Sequential dispatch only (BUK discipline; shared regime state).

**[260516-1045] rough**

## Character

Single-pace characterization A/B study. Operator has chosen to revert
RBRR_GCB_MACHINE_TYPE from e2-highcpu-32 to e2-standard-2 — this pace
captures the trade-off (what highcpu-32 delivered per workload) so the
choice is documented by data and the speedup being deferred is on the
record as forward reference. Cognitive posture: intricate but
mechanical, with judgment in the memo synthesis.

## Motivation

Original ₣BL pace ₢BLAAA called for a two-depot A/B; the 96-core quota
on canest blocks that path. This pace runs the A/B within one depot by
toggling machineType — both phases dispatched from the same host, so no
cross-host clock-skew confound versus the May 14 cerebro 2-cpu baseline
(which becomes supplementary reference, not primary comparison).
₢BLAAA's structural finding (machineType is pool-time, not build-time)
carries over.

## What — workloads, run sequentially, never in parallel

Phase 1 (e2-highcpu-32, current depot state) and Phase 2 (e2-standard-2,
after machine-type flip) each run the same 11-dispatch sequence in the
same order — worker-pool warm/cold state must be comparable across phases:

  - rbev-bottle-anthropic-jupyter conjure ×2  (warmup + data; biggest CPU target)
  - rbev-bottle-ifrit-forge conjure ×2        (warmup + data; multi-arch + apt-get + cargo)
  - rbev-bottle-ifrit-airgap conjure ×3       (warmup + 2 data; multi-arch FROM forge)
  - rbev-bottle-plantuml bind ×1              (skopeo mirror — registry-I/O reference)
  - rbev-graft-demo about+vouch ×1            (mixed mode reference)
  - rbev-busybox conjure ×1                   (tiny single-stage reference)
  - rbw-fV vouch ×1                           (sweeps all hallmarks)

Within each multi-run workload, the first run is warmup (discarded);
subsequent runs are data.

## How — five steps from the working directory

Setup (not measured):
  1. tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit-forge
     (pins rust:slim-bookworm; writes forge's RBRV_IMAGE_1_ANCHOR; commit)
  2. tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit-forge
     (produces forge hallmark; not counted as a measurement run)
  3. Capture forge hallmark from ../output-buk/current/rbf_fact_hallmark
  4. Edit rbev-vessels/rbev-bottle-ifrit-airgap/rbrv.env:
     RBRV_IMAGE_1_ANCHOR=rbi_hm/<HALLMARK>/image:<HALLMARK>
     and commit.

Phase 1 — e2-highcpu-32 measurements:
  Run the 11-dispatch sequence above, in the listed order. Tabtargets
  self-log to ../logs-buk/hist-rbw-fO-sh-*.txt and hist-rbw-fV-sh-*.txt
  with per-line timestamps.

Machine-type flip:
  Edit rbrr.env: RBRR_GCB_MACHINE_TYPE=e2-standard-2. Commit before
  Phase 2 begins. This is the implementing change for the locked decision.

Phase 2 — e2-standard-2 measurements:
  Same 11-dispatch sequence, same order as Phase 1. Logs land alongside
  Phase 1's.

Extraction:
  For each log, grep timestamped event lines from rbfd_ordain. Events
  of interest:
    Loaded vessel, Building vessel image,
    Conjure: SUCCESS, Mirror: SUCCESS,
    Vouch: SUCCESS, About+Vouch: SUCCESS.
  Wall-clock per operation is the timestamp delta between the opening
  event ("Building vessel image" for conjure; "Loaded vessel" for the
  others) and the matching terminal SUCCESS line. Line format:
    [YYYY-MM-DD HH:MM:SS] rbfd_ordain <event>
  May 14 cerebro logs (supplementary reference) live on cerebro at
  ../logs-buk/hist-rbw-fO-sh-20260514-2*.txt — obtain via scp if desired.

## Outcome

Memo at Memos/memo-20260516-cloudbuild-machinetype-bench.md:
  - Carries ₢BLAAA's structural finding (machineType is pool-time)
  - Decision record: revert to e2-standard-2 (locked a priori)
  - Per-workload table — highcpu-32 vs 2-cpu, same-host, with the
    first run discarded as warmup where >1 data run was collected
  - Profile groupings: CPU-bound conjures, registry-I/O mirror,
    mixed about+vouch and vouch-chain
  - Forward-reference ledger: what highcpu-32 delivered per workload,
    expressed as speedup ratios, so a future "should we go back?"
    decision has direct evidence

## Constraints

  - 96-core quota stays intact — no depot levy/unmake.
  - Linear commits on main: forge anchor, airgap anchor, machine-type
    flip, memo. No branch.
  - Sequential test execution only (BUK discipline; shared regime
    state and worker pool).
  - rbrr.env starts at RBRR_GCB_MACHINE_TYPE=e2-highcpu-32 and ends at
    e2-standard-2; the single flip commit lands between Phase 1 and
    Phase 2.

**[260516-1037] rough**

## Character

Single-pace characterization A/B study. Operator has chosen to revert
RBRR_GCB_MACHINE_TYPE from e2-highcpu-32 to e2-standard-2 — this pace
captures the trade-off (what highcpu-32 delivered per workload) so the
choice is documented by data and the speedup being deferred is on the
record as forward reference. Cognitive posture: intricate but
mechanical, with judgment in the memo synthesis.

## Motivation

Original ₣BL pace ₢BLAAA called for a two-depot A/B; the 96-core quota
on canest blocks that path. This pace runs the A/B within one depot by
toggling machineType — same-host (mac) measurement, no cross-host
confound versus the May 14 cerebro 2-cpu baseline (which becomes
supplementary reference, not primary comparison). ₢BLAAA's structural
finding (machineType is pool-time, not build-time) carries over.

## What — workloads, run sequentially, never in parallel

Phase 1 (e2-highcpu-32, current depot state) and Phase 2 (e2-standard-2,
after machine-type flip) each run the same 11-dispatch sequence:

  - rbev-bottle-anthropic-jupyter conjure ×2  (warmup + data; biggest CPU target)
  - rbev-bottle-ifrit-forge conjure ×2        (warmup + data; multi-arch + apt-get + cargo)
  - rbev-bottle-ifrit-airgap conjure ×3       (warmup + 2 data; multi-arch FROM forge)
  - rbev-bottle-plantuml bind ×1              (skopeo mirror — registry-I/O reference)
  - rbev-graft-demo about+vouch ×1            (mixed mode reference)
  - rbev-busybox conjure ×1                   (tiny single-stage reference)
  - rbw-fV vouch ×1                           (sweeps all hallmarks)

## How — three phases, working directory only

Phase 0 — setup (not measured):
  1. tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit-forge
     (pins rust:slim-bookworm; writes forge's RBRV_IMAGE_1_ANCHOR; commit)
  2. tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit-forge
     (produces forge hallmark; not counted as a measurement run)
  3. Capture forge hallmark from ../output-buk/current/rbf_fact_hallmark
  4. Edit rbev-vessels/rbev-bottle-ifrit-airgap/rbrv.env:
     RBRV_IMAGE_1_ANCHOR=rbi_hm/<HALLMARK>/image:<HALLMARK>
     and commit.

Phase 1 — e2-highcpu-32 measurements:
  Run the 11-dispatch sequence above. Tabtargets self-log to
  ../logs-buk/hist-rbw-fO-sh-*.txt and hist-rbw-fV-sh-*.txt with
  per-line timestamps.

Phase 2 — machine-type flip:
  Edit rbrr.env: RBRR_GCB_MACHINE_TYPE=e2-standard-2. Commit.
  This is the implementing change for the locked decision.

Phase 3 — e2-standard-2 measurements:
  Same 11-dispatch sequence. Logs land alongside Phase 1's.

Phase 4 — extraction:
  /tmp/bench-bo/extract.py over the new logs (and optionally the
  May 14 cerebro logs as supplementary reference). Group by profile.

## Outcome

Memo at Memos/memo-20260516-cloudbuild-machinetype-bench.md:
  - Carries ₢BLAAA's structural finding (machineType is pool-time)
  - Decision record: revert to e2-standard-2 (locked a priori)
  - Per-workload table — highcpu-32 vs 2-cpu, same-host, with both
    warmup-discarded runs averaged where >1 data run was collected
  - Profile groupings: CPU-bound conjures, registry-I/O mirror,
    mixed about+vouch and vouch-chain
  - Forward-reference ledger: what highcpu-32 delivered per workload,
    expressed as speedup ratios, so a future "should we go back?"
    decision has direct evidence

## Constraints

  - 96-core quota stays intact — no depot levy/unmake.
  - Linear commits on main: forge anchor, airgap anchor, machine-type
    flip, memo. No branch.
  - Sequential test execution only (BUK discipline; shared regime
    state and worker pool).
  - End state of rbrr.env is RBRR_GCB_MACHINE_TYPE=e2-standard-2
    (the implementing change, not a temporary measurement state).

**[260516-0953] rough**

## Character

Measurement pace. Hand-orchestrated; goal is data plus a memo
recommendation. Single deliverable. Skips the full gauntlet because
the current depot's 96-core quota cannot be re-levied at default
2-cpu without forfeiting it.

## Motivation

`RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` is currently in effect on the
canest depot. The original A/B benchmark in ₣BL's stabled pace
₢BLAAA called for two depot lifecycles; that structural path is
blocked by the quota fact. This pace measures the current depot's
per-workload conjure wall-clock against the May 14 cerebro clean-
gauntlet baseline. The structural finding from ₢BLAAA (machineType
is pool-time, not build-time) carries over and earns the memo its
keep regardless of measurement noise.

## What — three Cloud Build workloads via tabtargets

1. moriah bottle conjure (`rbev-bottle-ifrit-airgap`), 3× back-to-back
   — discard run 1 as worker-pool warmup; runs 2 and 3 are the data.
2. plantuml bind (`rbev-bottle-plantuml`), 1× — multi-arch + SBOM
   upper bound.
3. vouch (`rbw-fV`), 1× — airgap-pool verification chain.

Skip reliquary inscribe and enshrine — registry-I/O dominated, won't
move with CPU.

## How — tabtarget sequence, no raw bash for execution

Operate from the working directory; the credentials and depot are
prereq-ready per the paddock, no auth dance needed.

1. Prereq: airgap vessel needs a reliquary stamp. Check
   `rbev-vessels/rbev-bottle-ifrit-airgap/rbrv.env` —
   if `RBRV_RELIQUARY=` is empty:
     a. `tt/rbw-iar.DirectorAuditsReliquaries.sh` — does the canest
        depot already have a reliquary stamp from a prior inscribe?
     b. If yes: `tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh <stamp>`
        to write the stamp into every vessel's rbrv.env.
     c. If no: `tt/rbw-dI.DirectorInscribesReliquary.sh` to create
        one, then yoke. (This is a one-time setup cost, separate
        from the measurement workloads.)

2. Measurement workloads — sequential, never parallel:
   - `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit-airgap`  (×3)
   - `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-plantuml`  (×1)
   - `tt/rbw-fV.DirectorVouchesHallmarks.sh`  (×1)

   Each tabtarget self-logs to `../logs-buk/hist-rbw-fO-sh-*.txt` (or
   `hist-rbw-fV-sh-*.txt`) with `[YYYY-MM-DD HH:MM:SS]` timestamps on
   every line.

3. Extraction: grep `rbfd_ordain` event lines from the new logs and
   from the May 14 baseline at `/tmp/bench-bo/hist-rbw-fO-*.txt`
   (operator-pickable scratch root; re-scp from cerebro if local copy
   is stale). Events of interest: `Loaded vessel`, `Building vessel
   image`, `Conjure: SUCCESS`, `Mirror: SUCCESS`, `Vouch: SUCCESS`,
   `About+Vouch: SUCCESS`. Timestamp deltas give whole-operation
   wall-clock per workload.

## Outcome

Memo at `Memos/memo-20260516-cloudbuild-machinetype-bench.md` carrying
the structural finding from ₢BLAAA's docket plus:
  - Per-workload comparison table — 32cpu vs 2cpu, whole-conjure
    wall-clock, no step decomposition (asymmetric data path: baseline
    is log-only).
  - Verdict: CPU-bound, IO-bound, or mixed — per workload, not summed.
    Anchored by ₢BLAAA's structural finding that machineType is
    pool-time.
  - Recommendation: keep `e2-highcpu-32` as default, roll back to
    `e2-standard-2`, or pick a middle value.

## Constraints

  - No production-config changes from this pace. Rollback (or non-
    rollback) lives in the follow-up pace.
  - Comparison is cross-host (mac vs cerebro fundus). Whole-operation
    wall-clock comparison only — no per-step decomposition.
  - 96-core quota stays intact — no depot levy/unmake.

**[260516-0919] rough**

## Character

Measurement pace. Hand-orchestrated; goal is data plus a memo
recommendation. Single deliverable. Skips the full gauntlet because
the current depot's 96-core quota cannot be re-levied at default
2-cpu without forfeiting it.

## Motivation

`RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` is currently in effect on the
canest depot. The original A/B benchmark in ₣BL's stabled pace
₢BLAAA called for two depot lifecycles; that structural path is
blocked by the quota fact. This pace measures the current depot's
per-workload conjure wall-clock against the May 14 cerebro clean-
gauntlet baseline. The structural finding from ₢BLAAA (machineType
is pool-time, not build-time) carries over and earns the memo its
keep regardless of measurement noise.

## What — three Cloud Build workloads via tabtargets

1. moriah bottle conjure (`rbev-bottle-ifrit-airgap`), 3× back-to-back
   — discard run 1 as worker-pool warmup; runs 2 and 3 are the data.
2. plantuml bind (`rbev-bottle-plantuml`), 1× — multi-arch + SBOM
   upper bound.
3. vouch (`rbw-fV`), 1× — airgap-pool verification chain.

Skip reliquary inscribe and enshrine — registry-I/O dominated, won't
move with CPU.

## How — tabtarget sequence, no raw bash for execution

Operate against `/Users/bhyslop/projects/rbm_alpha_recipemuster` (alpha
is the canest-bearing repo). The credentials and depot are
prereq-ready per the paddock; no auth dance needed.

1. Prereq: airgap vessel needs a reliquary stamp. Check
   `rbev-vessels/rbev-bottle-ifrit-airgap/rbrv.env` —
   if `RBRV_RELIQUARY=` is empty:
     a. `tt/rbw-iar.DirectorAuditsReliquaries.sh` — does the canest
        depot already have a reliquary stamp from a prior inscribe?
     b. If yes: `tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh <stamp>`
        to write the stamp into every vessel's rbrv.env.
     c. If no: `tt/rbw-dI.DirectorInscribesReliquary.sh` to create
        one, then yoke. (This is a one-time setup cost, separate
        from the measurement workloads.)

2. Measurement workloads — sequential, never parallel:
   - `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit-airgap`  (×3)
   - `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-plantuml`  (×1)
   - `tt/rbw-fV.DirectorVouchesHallmarks.sh`  (×1)

   Each tabtarget self-logs to `../logs-buk/hist-rbw-fO-sh-*.txt` (or
   `hist-rbw-fV-sh-*.txt`) with `[YYYY-MM-DD HH:MM:SS]` timestamps on
   every line.

3. Extraction: grep `rbfd_ordain` event lines from the new logs and
   from the May 14 baseline at `/tmp/bench-bo/hist-rbw-fO-*.txt` (or
   re-scp from cerebro if local copy is stale). Events of interest:
   `Loaded vessel`, `Building vessel image`, `Conjure: SUCCESS`,
   `Mirror: SUCCESS`, `Vouch: SUCCESS`, `About+Vouch: SUCCESS`.
   Timestamp deltas give whole-operation wall-clock per workload.

## Outcome

Memo at `Memos/memo-20260516-cloudbuild-machinetype-bench.md` carrying
the structural finding from ₢BLAAA's docket plus:
  - Per-workload comparison table — 32cpu vs 2cpu, whole-conjure
    wall-clock, no step decomposition (asymmetric data path: baseline
    is log-only).
  - Verdict: CPU-bound, IO-bound, or mixed — per workload, not summed.
    Anchored by ₢BLAAA's structural finding that machineType is
    pool-time.
  - Recommendation: keep `e2-highcpu-32` as default, roll back to
    `e2-standard-2`, or pick a middle value.

## Constraints

  - No production-config changes from this pace. Rollback (or non-
    rollback) lives in the follow-up pace.
  - Comparison is cross-host (mac vs cerebro fundus). Whole-operation
    wall-clock comparison only — no per-step decomposition.
  - 96-core quota stays intact — no depot levy/unmake.

**[260516-0852] rough**

## Character

Measurement pace. Hand-orchestrated; goal is data plus a memo
recommendation. Single deliverable. Skips the full gauntlet because
the current depot's 96-core quota cannot be re-levied at default
2-cpu without forfeiting it.

## Motivation

`RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` is currently in effect on the
canest depot. The original A/B benchmark in ₣BL's stabled pace
₢BLAAA called for two depot lifecycles; that structural path is
blocked by the quota fact. This pace instead measures the current
depot's per-workload conjure wall-clock against the May 14 cerebro
clean-gauntlet baseline. The structural finding from ₢BLAAA
(machineType is pool-time, not build-time) carries over and earns
the memo its keep regardless of measurement noise.

## What — three Cloud Build workloads, single-case run via tabtargets

1. moriah bottle conjure (`rbev-bottle-ifrit-airgap`), 3× back-to-back
   — discard run 1 as worker-pool warmup; runs 2 and 3 are the data.
2. plantuml bind (`rbev-bottle-plantuml`), 1× — multi-arch + SBOM
   upper bound.
3. vouch (`rbw-fV`), 1× — airgap-pool verification chain.

Skip reliquary inscribe and enshrine — registry-I/O dominated, won't
move with CPU. Yesterday's levy already exercised one inscribe and
one busybox conjure on this depot; their durations fold into the
memo as side data.

## How

Depot stays as-is — no rbrr.env edits, no relevy. Drive each workload
directly via its tabtarget. Capture wall-clock from the tabtarget log
(`rbfd_ordain` Conjure/Vouch/Mirror SUCCESS timestamps). Granularity
is whole-operation only — tabtarget logs do not stream Cloud Build
step-level output, only `rbfd_ordain` polling summaries.

Baseline: May 14 cerebro fO logs at conjure-level granularity. The
cerebro e2-standard-2 depot project is gone — gcloud-describe baseline
is impossible — but the tabtarget logs are intact on cerebro under
`../logs-buk/hist-rbw-fO-sh-20260514-21*.txt`, and they already
contain the three planned workloads' baseline timings. Side log: the
gauntlet rollup `hist-rbw-tP-sh-20260514-203134-1368806-985.txt`.
Two macOS-local 11-fixture clean passes (May 6 and May 12) are
available for cross-host sanity; the May 14 cerebro run is the
canonical reference.

## Outcome

Memo at `Memos/memo-YYYYMMDD-cloudbuild-machinetype-bench.md` carrying
the structural finding from ₢BLAAA's docket plus:
  - Per-workload comparison table — 32cpu vs 2cpu, whole-conjure
    wall-clock, no step decomposition (asymmetric data path: baseline
    log-only).
  - Verdict: CPU-bound, IO-bound, or mixed — per workload, not summed.
    Anchored by ₢BLAAA's structural finding that machineType is
    pool-time.
  - Recommendation: keep `e2-highcpu-32` as default, roll back to
    `e2-standard-2`, or pick a middle value.

## Constraints

  - No production-config changes from this pace. Rollback (or non-
    rollback) lives in the follow-up pace.
  - Comparison is cross-host (mac vs cerebro fundus). Whole-operation
    wall-clock comparison only — no per-step decomposition.
  - 96-core quota stays intact — no depot levy/unmake.

**[260516-0824] rough**

## Character

Measurement pace. Hand-orchestrated; goal is data plus a memo
recommendation. Single deliverable. Skips the full gauntlet because
the current depot's 96-core quota cannot be re-levied at default
2-cpu without forfeiting it.

## Motivation

`RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` is currently in effect on the
canest depot. The original A/B benchmark in ₣BL's stabled pace
₢BLAAA called for two depot lifecycles; that structural path is
blocked by the quota fact. This pace instead measures the current
depot's per-build substep durations against a historic 12-fixture
clean-gauntlet baseline. The structural finding from ₢BLAAA
(machineType is pool-time, not build-time) carries over and earns
the memo its keep regardless of measurement noise.

## What — three Cloud Build workloads, single-case run via tabtargets

1. moriah bottle conjure (`rbev-bottle-ifrit-airgap`), 3× back-to-back
   — discard run 1 as worker-pool warmup; runs 2 and 3 are the data.
2. plantuml bind (`rbev-bottle-plantuml`), 1× — multi-arch + SBOM
   upper bound.
3. vouch (`rbw-fV`), 1× — airgap-pool verification chain.

Skip reliquary inscribe and enshrine — registry-I/O dominated, won't
move with CPU. Yesterday's levy already exercised one inscribe and
one busybox conjure on this depot; their durations fold into the
memo as side data.

## How

Depot stays as-is — no rbrr.env edits, no relevy. Drive each workload
directly via its tabtarget. Capture wall-clock from the tabtarget log
and per-step duration via `gcloud builds list` + `gcloud builds
describe` for both the current runs and the historic baseline.

Baseline: the May 14 cerebro clean-pass log
`hist-rbw-tP-sh-20260514-203134-1368806-985.txt` (12 fixtures, 1h 39m)
plus the corresponding GCB records on cerebro's e2-standard-2 depot
(retained 30+ days). Two macOS-local 11-fixture clean passes (May 6
and May 12) are available for cross-host sanity; the May 14 cerebro
run is the canonical reference.

## Outcome

Memo at `Memos/memo-YYYYMMDD-cloudbuild-machinetype-bench.md` carrying
the structural finding from ₢BLAAA's docket plus:
  - Per-workload comparison table — 32cpu vs 2cpu, decomposed by GCB
    step where the build's `steps[]` permits.
  - Verdict: CPU-bound, IO-bound, or mixed — per workload, not summed.
  - Recommendation: keep `e2-highcpu-32` as default, roll back to
    `e2-standard-2`, or pick a middle value.

## Constraints

  - No production-config changes from this pace. Rollback (or non-
    rollback) lives in the follow-up pace.
  - Comparison is cross-host (mac vs cerebro fundus). Per-step
    *ratios* between build phases are more defensible than absolute
    deltas; the memo states this caveat.
  - 96-core quota stays intact — no depot levy/unmake.

**[260516-0824] rough**

## Character

Measurement pace. Hand-orchestrated; goal is data plus a memo
recommendation. Single deliverable. Skips the full gauntlet because
the current depot's 96-core quota cannot be re-levied at default
2-cpu without forfeiting it.

## Motivation

`RBRR_GCB_MACHINE_TYPE=e2-highcpu-32` is currently in effect on the
canest depot. The original A/B benchmark in ₣BL's stabled pace
₢BLAAA called for two depot lifecycles; that structural path is
blocked by the quota fact. This pace instead measures the current
depot's per-build substep durations against a historic 12-fixture
clean-gauntlet baseline. The structural finding from ₢BLAAA
(machineType is pool-time, not build-time) carries over and earns
the memo its keep regardless of measurement noise.

## What — three Cloud Build workloads, single-case run via tabtargets

1. moriah bottle conjure (`rbev-bottle-ifrit-airgap`), 3× back-to-back
   — discard run 1 as worker-pool warmup; runs 2 and 3 are the data.
2. plantuml bind (`rbev-bottle-plantuml`), 1× — multi-arch + SBOM
   upper bound.
3. vouch (`rbw-fV`), 1× — airgap-pool verification chain.

Skip reliquary inscribe and enshrine — registry-I/O dominated, won't
move with CPU. Yesterday's levy already exercised one inscribe and
one busybox conjure on this depot; their durations fold into the
memo as side data.

## How

Depot stays as-is — no rbrr.env edits, no relevy. Drive each workload
directly via its tabtarget. Capture wall-clock from the tabtarget log
and per-step duration via `gcloud builds list` + `gcloud builds
describe` for both the current runs and the historic baseline.

Baseline: the May 14 cerebro clean-pass log
`hist-rbw-tP-sh-20260514-203134-1368806-985.txt` (12 fixtures, 1h 39m)
plus the corresponding GCB records on cerebro's e2-standard-2 depot
(retained 30+ days). Two macOS-local 11-fixture clean passes (May 6
and May 12) are available for cross-host sanity; the May 14 cerebro
run is the canonical reference.

## Outcome

Memo at `Memos/memo-YYYYMMDD-cloudbuild-machinetype-bench.md` carrying
the structural finding from ₢BLAAA's docket plus:
  - Per-workload comparison table — 32cpu vs 2cpu, decomposed by GCB
    step where the build's `steps[]` permits.
  - Verdict: CPU-bound, IO-bound, or mixed — per workload, not summed.
  - Recommendation: keep `e2-highcpu-32` as default, roll back to
    `e2-standard-2`, or pick a middle value.

## Constraints

  - No production-config changes from this pace. Rollback (or non-
    rollback) lives in the follow-up pace.
  - Comparison is cross-host (mac vs cerebro fundus). Per-step
    *ratios* between build phases are more defensible than absolute
    deltas; the memo states this caveat.
  - 96-core quota stays intact — no depot levy/unmake.

**[260514-1511] rough**

## Character
Mechanical — work through issues uncovered while running the gauntlet with the
e2-highcpu-32 GCB machine tweak. Fix what breaks, note what's flaky.

### probe-egress-assertion-repair (₢BOAAP) [complete]

**[260519-1117] complete**

## Character
Mechanical implementation with discipline-reading and spec-update prerequisites.

## Goal
Repair the tether and airgap egress assertion scripts in
`rbgp_Payor.sh` (~lines 1101-1127) so the probes actually validate
egress posture. Currently both probes fail at step 0 because
`curl -fsS https://storage.googleapis.com` exits 22 (HTTP 400 on root
path); the failure has been silently tolerated by the framework's
any-terminal-state policy, defeating the probe's stated purpose and
preventing step 2 (the `rbi_df` marker push) from ever running.

## Locked
- **Read BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) before
  writing any bash** — the assertion scripts are cloud-side bash but
  their host-side composition must be BCG-compliant.
- **Update `RBSDE-depot_levy.adoc`** to reflect the new assertion
  semantics. The spec currently describes the assertion in idealized
  terms; new text must accurately describe what the assertion tests
  (likely connectivity-vs-authorization distinction).

## Done
Probes return SUCCESS terminal state on a fresh-levy trial under
normal conditions; the `rbi_df` marker reaches GAR (step 2 runs to
completion). Egress assertion distinguishes reachability from
authorization and produces a clear diagnostic when posture is
actually wrong. Spec updated. A levy fixture trial confirms.

**[260519-1654] rough**

## Character
Mechanical implementation with discipline-reading and spec-update prerequisites.

## Goal
Repair the tether and airgap egress assertion scripts in
`rbgp_Payor.sh` (~lines 1101-1127) so the probes actually validate
egress posture. Currently both probes fail at step 0 because
`curl -fsS https://storage.googleapis.com` exits 22 (HTTP 400 on root
path); the failure has been silently tolerated by the framework's
any-terminal-state policy, defeating the probe's stated purpose and
preventing step 2 (the `rbi_df` marker push) from ever running.

## Locked
- **Read BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) before
  writing any bash** — the assertion scripts are cloud-side bash but
  their host-side composition must be BCG-compliant.
- **Update `RBSDE-depot_levy.adoc`** to reflect the new assertion
  semantics. The spec currently describes the assertion in idealized
  terms; new text must accurately describe what the assertion tests
  (likely connectivity-vs-authorization distinction).

## Done
Probes return SUCCESS terminal state on a fresh-levy trial under
normal conditions; the `rbi_df` marker reaches GAR (step 2 runs to
completion). Egress assertion distinguishes reachability from
authorization and produces a clear diagnostic when posture is
actually wrong. Spec updated. A levy fixture trial confirms.

### machine-type-restore-e2-standard-2 (₢BOAAE) [abandoned]

**[260519-1056] abandoned**

## Character

Mechanical config rollback. Decision posture from the bench pace's
memo recommendation — keep highcpu, roll back, or pick a middle value.

## What

Two locations carry the experiment's `e2-highcpu-32` setting:

  - `.rbk/rbrr.env` operational value (commit ba3b7b3d installed
    the bump)
  - marshal-zero default in the rblm CLI, raised together so
    regime resets preserved the experiment (commit a7a98cd8
    establishes the wiring location)

Both revert together to whatever the bench memo lands on.

## Constraints

  - Take-effect on rbrr.env requires re-levy of the depot. The
    current depot carries a 96-core quota; a relevy at default
    2-cpu forfeits it. Operator decides whether the re-levy
    happens under this pace or is deferred.
  - Skip entirely if the bench memo recommends keeping highcpu.

**[260516-0824] rough**

## Character

Mechanical config rollback. Decision posture from the bench pace's
memo recommendation — keep highcpu, roll back, or pick a middle value.

## What

Two locations carry the experiment's `e2-highcpu-32` setting:

  - `.rbk/rbrr.env` operational value (commit ba3b7b3d installed
    the bump)
  - marshal-zero default in the rblm CLI, raised together so
    regime resets preserved the experiment (commit a7a98cd8
    establishes the wiring location)

Both revert together to whatever the bench memo lands on.

## Constraints

  - Take-effect on rbrr.env requires re-levy of the depot. The
    current depot carries a 96-core quota; a relevy at default
    2-cpu forfeits it. Operator decides whether the re-levy
    happens under this pace or is deferred.
  - Skip entirely if the bench memo recommends keeping highcpu.

### depot-regime-now (₢BOAAB) [complete]

**[260517-0907] complete**

## Character
Design conversation requiring judgment — pull the depot regime work forward
from the 'win' timeframe into the MVP-tactical window. Assess shape, locked
constraints, and what done looks like for the near-term cut.

**[260514-1511] rough**

## Character
Design conversation requiring judgment — pull the depot regime work forward
from the 'win' timeframe into the MVP-tactical window. Assess shape, locked
constraints, and what done looks like for the near-term cut.

### srjcl-jupyter-reverify (₢BOAAC) [complete]

**[260515-0809] complete**

## Character
Mechanical with one design beat — reverify the srjcl jupyter notebook still
functions and can reach Anthropic. Consider surfacing the local port during
crucible charge so the operator doesn't have to fish for it.

**[260514-1511] rough**

## Character
Mechanical with one design beat — reverify the srjcl jupyter notebook still
functions and can reach Anthropic. Consider surfacing the local port during
crucible charge so the operator doesn't have to fish for it.

### levy-pool-lro-await (₢BOAAF) [complete]

**[260519-1722] complete**

## Character
Mechanical — the existing `{rbbc_await} returned operation` pattern in
RBSDE-depot_levy (used at "Create Depot Project" and "Create Container
Repository") is the template. Match it.

## Goal
Make `workerPools.create` await its LRO terminal state before any
downstream code uses the pool. Surface terminal errors (e.g., machine-type
quota denial) at the create step rather than as a misleading "workerpool
not found" five steps later at the new probe-submission step.

## Boundary
- Code: `rbgp_Payor.sh` pool-create call sites — match the existing
  project-create / repo-create LRO handling in the same module.
- Spec: RBSDE-depot_levy "Create Private Worker Pool" step gains
  `{rbbc_await} returned operation`, matching project-create and
  repo-create steps in the same spec.
- Spec drift in the same step: convert singular `WORKER_POOL_STEM`
  description to dual `[tether, airgap]` iteration matching the existing
  "Submit Per-Pool Probe Builds" step.

## Done
- pristine-lifecycle fixture's `depot_stand_up` case clears the
  "workerpool not found" failure that surfaced the e2-highcpu-32
  gauntlet attempt.
- RBSDE create and probe steps are internally consistent on pool count.

**[260516-1528] rough**

## Character
Mechanical — the existing `{rbbc_await} returned operation` pattern in
RBSDE-depot_levy (used at "Create Depot Project" and "Create Container
Repository") is the template. Match it.

## Goal
Make `workerPools.create` await its LRO terminal state before any
downstream code uses the pool. Surface terminal errors (e.g., machine-type
quota denial) at the create step rather than as a misleading "workerpool
not found" five steps later at the new probe-submission step.

## Boundary
- Code: `rbgp_Payor.sh` pool-create call sites — match the existing
  project-create / repo-create LRO handling in the same module.
- Spec: RBSDE-depot_levy "Create Private Worker Pool" step gains
  `{rbbc_await} returned operation`, matching project-create and
  repo-create steps in the same spec.
- Spec drift in the same step: convert singular `WORKER_POOL_STEM`
  description to dual `[tether, airgap]` iteration matching the existing
  "Submit Per-Pool Probe Builds" step.

## Done
- pristine-lifecycle fixture's `depot_stand_up` case clears the
  "workerpool not found" failure that surfaced the e2-highcpu-32
  gauntlet attempt.
- RBSDE create and probe steps are internally consistent on pool count.

### theurge-buk-temp-discipline (₢BOAAG) [complete]

**[260519-1126] complete**

## Character
Design-conversation — read theurge source first to understand why
`/tmp/rbtd-NNN/` was chosen originally. The fix shape isn't obvious until
that's known. Not mechanical.

## Goal
Theurge-sandboxed workbench invocations should write their workbench
`BURD_TEMP_DIR` and output dir under BUK roots (`../temp-buk/`,
`../output-buk/`) rather than under `/tmp/rbtd-NNN/`. Forensic artifacts
from failing gauntlet runs — specifically the per-call HTTP captures
written by `rbgu_http_json` — should survive reboot and
systemd-tmpfiles cleanup, matching standalone-workbench behavior.

## Done
After a failing fixture-case under theurge, the failed workbench
invocation's per-call HTTP captures remain reachable via BUK conventions
and survive reboot.

**[260516-1529] rough**

## Character
Design-conversation — read theurge source first to understand why
`/tmp/rbtd-NNN/` was chosen originally. The fix shape isn't obvious until
that's known. Not mechanical.

## Goal
Theurge-sandboxed workbench invocations should write their workbench
`BURD_TEMP_DIR` and output dir under BUK roots (`../temp-buk/`,
`../output-buk/`) rather than under `/tmp/rbtd-NNN/`. Forensic artifacts
from failing gauntlet runs — specifically the per-call HTTP captures
written by `rbgu_http_json` — should survive reboot and
systemd-tmpfiles cleanup, matching standalone-workbench behavior.

## Done
After a failing fixture-case under theurge, the failed workbench
invocation's per-call HTTP captures remain reachable via BUK conventions
and survive reboot.

### seed-skip-orphan-boaah (₢BOAAH) [abandoned]

**[260516-1003] abandoned**

## Character

Throwaway. Sole purpose is to advance the BO pace-seed past AAH so
new paces don't collide with the orphan ₢BOAAH coronet that exists
in git history (alpha-side theurge RCG retrofit, commit d7145e0c)
but not in beta's gallops state after the 2026-05-16 reconciliation.

To be dropped immediately after enroll.

**[260516-1003] rough**

## Character

Throwaway. Sole purpose is to advance the BO pace-seed past AAH so
new paces don't collide with the orphan ₢BOAAH coronet that exists
in git history (alpha-side theurge RCG retrofit, commit d7145e0c)
but not in beta's gallops state after the 2026-05-16 reconciliation.

To be dropped immediately after enroll.

### post-levy-quota-bump-flow (₢BOAAI) [abandoned]

**[260519-1132] abandoned**

## Character

Planning conversation. Design a tighter integration of the quota-bump
procedure into the levy flow, exploiting the fact that the levy's
degenerate build now unlocks Google's "Edit Quotas" Console option
immediately on the freshly-levied project.

## Motivation

Phase 3 of ₢BOAAA surfaced that fresh-project Cloud Build vCPU defaults
have dropped from the historical 10 vCPU baseline (encoded in stale
rbrr.env / rbhpq_quota_build.sh wording as "fits 5 concurrent in 10-CPU
quota") down to 2 vCPU. The first multi-arch ordain on a freshly-levied
e2-standard-2 depot now wedges in QUEUED because 2 vCPU only supports 1
concurrent build, not the 3 RBRR_GCB_MIN_CONCURRENT_BUILDS demands.

The rbw-gq handbook exists but is a separate ceremony the operator must
know to invoke. Levy completion is the natural moment to surface the
quota URL and the bump procedure, since the degenerate build inside levy
has just made Edit Quotas accessible on the new project.

## What done looks like

- Design notes capturing the proposed integration shape: where in the
  levy completion banner the quota guidance should appear, how strongly
  it should gate downstream operations, what the operator's action loop
  looks like.
- Implementation paces slated separately.
- Cleanup ticket for the stale "10-CPU quota" wording in rbrr.env
  comments and rbhpq_quota_build.sh prose.

## Constraints

- Plan-only; no implementation in this pace.
- Reference ₢BOAAA's empirical finding rather than restating it.

**[260517-0034] rough**

## Character

Planning conversation. Design a tighter integration of the quota-bump
procedure into the levy flow, exploiting the fact that the levy's
degenerate build now unlocks Google's "Edit Quotas" Console option
immediately on the freshly-levied project.

## Motivation

Phase 3 of ₢BOAAA surfaced that fresh-project Cloud Build vCPU defaults
have dropped from the historical 10 vCPU baseline (encoded in stale
rbrr.env / rbhpq_quota_build.sh wording as "fits 5 concurrent in 10-CPU
quota") down to 2 vCPU. The first multi-arch ordain on a freshly-levied
e2-standard-2 depot now wedges in QUEUED because 2 vCPU only supports 1
concurrent build, not the 3 RBRR_GCB_MIN_CONCURRENT_BUILDS demands.

The rbw-gq handbook exists but is a separate ceremony the operator must
know to invoke. Levy completion is the natural moment to surface the
quota URL and the bump procedure, since the degenerate build inside levy
has just made Edit Quotas accessible on the new project.

## What done looks like

- Design notes capturing the proposed integration shape: where in the
  levy completion banner the quota guidance should appear, how strongly
  it should gate downstream operations, what the operator's action loop
  looks like.
- Implementation paces slated separately.
- Cleanup ticket for the stale "10-CPU quota" wording in rbrr.env
  comments and rbhpq_quota_build.sh prose.

## Constraints

- Plan-only; no implementation in this pace.
- Reference ₢BOAAA's empirical finding rather than restating it.

### rbrd-regime-split (₢BOAAJ) [complete]

**[260519-1222] complete**

## Character

Intricate but mechanical. Sibling-regime archetype is well-established.

## Goal

Mint a new RBRD regime, peeling the four depot-time-immutable settings
out of RBRR. After this pace, names communicate the lifecycle category;
enforcement is the next pace.

## Locked decisions

- Four settings move: RBRR_CLOUD_PREFIX, RBRR_DEPOT_MONIKER,
  RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE → RBRD_*. The remaining eight
  RBRR settings stay unchanged in RBRR.
- rbrd_regime.sh follows the sibling regime archetype uniformly — no
  bespoke code. New RBSRT-RegimeDepot.adoc parallels RBSRP and RBSRR
  (T from "depoT"; RBSRD is occupied by retriever_divest, and RBSR*
  already breaks strict first-letter convention at RBSRG-RegimeGcbPins).
- Lifecycle marshal (rblm_cli.sh) emits an rbrd.env blank alongside the
  rbrr.env blank.
- rbgc_Constants.sh's rbi_df comment block carries a stale "future RBRD
  in rbi_df" promise that becomes wrong with the next pace's decision
  (RBRD ships as its own image at a separate tag). Refresh the comment
  to reflect actual shape.
- No tripwire wiring in this pace. After this pace, names are right but
  a hand-edit of rbrd.env post-levy is still a silent no-op.

## Done

- rbw-rdr / rbw-rdv operate cleanly against a fresh rbrd.env blank.
- The four moved variables are absent from rbrr.env, present in rbrd.env.
- All consumers (rbgp_Payor.sh, rbdc_DerivedConstants.sh, anywhere else
  they are referenced) use the RBRD_* names.
- RBSDE rename pass complete; RBS0 index has the RBSRT entry.
- README regime list, file tree, and appendix entry mention RBRD.
- Handbook surfaces the new regime category.

**[260519-1129] rough**

## Character

Intricate but mechanical. Sibling-regime archetype is well-established.

## Goal

Mint a new RBRD regime, peeling the four depot-time-immutable settings
out of RBRR. After this pace, names communicate the lifecycle category;
enforcement is the next pace.

## Locked decisions

- Four settings move: RBRR_CLOUD_PREFIX, RBRR_DEPOT_MONIKER,
  RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE → RBRD_*. The remaining eight
  RBRR settings stay unchanged in RBRR.
- rbrd_regime.sh follows the sibling regime archetype uniformly — no
  bespoke code. New RBSRT-RegimeDepot.adoc parallels RBSRP and RBSRR
  (T from "depoT"; RBSRD is occupied by retriever_divest, and RBSR*
  already breaks strict first-letter convention at RBSRG-RegimeGcbPins).
- Lifecycle marshal (rblm_cli.sh) emits an rbrd.env blank alongside the
  rbrr.env blank.
- rbgc_Constants.sh's rbi_df comment block carries a stale "future RBRD
  in rbi_df" promise that becomes wrong with the next pace's decision
  (RBRD ships as its own image at a separate tag). Refresh the comment
  to reflect actual shape.
- No tripwire wiring in this pace. After this pace, names are right but
  a hand-edit of rbrd.env post-levy is still a silent no-op.

## Done

- rbw-rdr / rbw-rdv operate cleanly against a fresh rbrd.env blank.
- The four moved variables are absent from rbrr.env, present in rbrd.env.
- All consumers (rbgp_Payor.sh, rbdc_DerivedConstants.sh, anywhere else
  they are referenced) use the RBRD_* names.
- RBSDE rename pass complete; RBS0 index has the RBSRT entry.
- README regime list, file tree, and appendix entry mention RBRD.
- Handbook surfaces the new regime category.

**[260517-0904] rough**

## Character

Intricate but mechanical. Sibling-regime archetype is well-established.

## Goal

Mint a new RBRD regime, peeling the four depot-time-immutable settings
out of RBRR. After this pace, names communicate the lifecycle category;
enforcement is the next pace.

## Locked decisions

- Four settings move: RBRR_CLOUD_PREFIX, RBRR_DEPOT_MONIKER,
  RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE → RBRD_*. The remaining eight
  RBRR settings stay unchanged in RBRR.
- rbrd_regime.sh follows the sibling regime archetype uniformly — no
  bespoke code. New RBSRD-RegimeDepot.adoc parallels RBSRP and RBSRR.
- Lifecycle marshal (rblm_cli.sh) emits an rbrd.env blank alongside the
  rbrr.env blank.
- rbgc_Constants.sh's rbi_df comment block carries a stale "future RBRD
  in rbi_df" promise that becomes wrong with the next pace's decision
  (RBRD ships as its own image at a separate tag). Refresh the comment
  to reflect actual shape.
- No tripwire wiring in this pace. After this pace, names are right but
  a hand-edit of rbrd.env post-levy is still a silent no-op.

## Done

- rbw-rdr / rbw-rdv operate cleanly against a fresh rbrd.env blank.
- The four moved variables are absent from rbrr.env, present in rbrd.env.
- All consumers (rbgp_Payor.sh, rbdc_DerivedConstants.sh, anywhere else
  they are referenced) use the RBRD_* names.
- RBSDE rename pass complete; RBS0 index has the RBSRD entry.
- README regime list, file tree, and appendix entry mention RBRD.
- Handbook surfaces the new regime category.

### rbrd-tripwire-wire (₢BOAAK) [complete]

**[260519-1531] complete**

## Character

Intricate but mechanical. Established host-docker and image-extract
patterns throughout.

## Goal

Make depot-immutable drift fail loud. Levy inscribes a FROM-scratch
image carrying rbrd.env to GAR; every cloud-submitting command pulls
and exact-match diffs the file before proceeding.

## Locked decisions

- rbrd_check and rbrd_inscribe are public functions in a separate
  bespoke implementation module — not in rbrd_regime.sh (which must
  stay uniform with sibling regimes) and not in rbrd_cli.sh's body.
  rbrd_cli.sh sources the bespoke module and exposes both via dispatch
  with differential furnish. New tabtargets rbw-rdc and rbw-rdi.
- Consuming CLIs source the bespoke module, kindle it, call rbrd_check
  inline before cloud submission. No tabtarget preflight orchestration.
- rbrd_check takes a bearer token parameter; role-agnostic. Each
  consuming CLI passes its role's token. No new IAM grants.
- rbrd_inscribe runs host-side at end of successful levy using Payor's
  OAuth access token. Cloud Build is not involved.
- Pre-push existence guard via docker manifest inspect: present → fatal
  "already inscribed, unmake and relevy"; absent → proceed.
- Exact byte-match diff on rbrd.env contents — comments, whitespace,
  line ordering all participate.
- Image lives under rbi_df namespace; specific tag is a mount-time call.

## Done

- Fresh levy ends with the rbrd.env image present in GAR.
- Re-levy on an already-inscribed depot dies with overwrite-fatal.
- Every cloud-submitting command pulls + diffs before submitting; drift
  produces a diff + recovery guidance + fatal; image absent → fatal.
- RBSDE updated for the inscribe step at end of levy; the spec covering
  the kindle-time check identified and updated.
- Handbook documents drift, missing image, and re-inscribe overwrite
  failure modes with recovery guidance.

**[260517-0904] rough**

## Character

Intricate but mechanical. Established host-docker and image-extract
patterns throughout.

## Goal

Make depot-immutable drift fail loud. Levy inscribes a FROM-scratch
image carrying rbrd.env to GAR; every cloud-submitting command pulls
and exact-match diffs the file before proceeding.

## Locked decisions

- rbrd_check and rbrd_inscribe are public functions in a separate
  bespoke implementation module — not in rbrd_regime.sh (which must
  stay uniform with sibling regimes) and not in rbrd_cli.sh's body.
  rbrd_cli.sh sources the bespoke module and exposes both via dispatch
  with differential furnish. New tabtargets rbw-rdc and rbw-rdi.
- Consuming CLIs source the bespoke module, kindle it, call rbrd_check
  inline before cloud submission. No tabtarget preflight orchestration.
- rbrd_check takes a bearer token parameter; role-agnostic. Each
  consuming CLI passes its role's token. No new IAM grants.
- rbrd_inscribe runs host-side at end of successful levy using Payor's
  OAuth access token. Cloud Build is not involved.
- Pre-push existence guard via docker manifest inspect: present → fatal
  "already inscribed, unmake and relevy"; absent → proceed.
- Exact byte-match diff on rbrd.env contents — comments, whitespace,
  line ordering all participate.
- Image lives under rbi_df namespace; specific tag is a mount-time call.

## Done

- Fresh levy ends with the rbrd.env image present in GAR.
- Re-levy on an already-inscribed depot dies with overwrite-fatal.
- Every cloud-submitting command pulls + diffs before submitting; drift
  produces a diff + recovery guidance + fatal; image absent → fatal.
- RBSDE updated for the inscribe step at end of levy; the spec covering
  the kindle-time check identified and updated.
- Handbook documents drift, missing image, and re-inscribe overwrite
  failure modes with recovery guidance.

### rbnRX-convention-document (₢BOAAL) [abandoned]

**[260519-1534] abandoned**

## Character

Design conversation requiring judgment. The convention is already in
use by code (rbnnh_ for nameplate hooks, rbndf_ for depot facts, rbndb_
for depot regime bespoke helpers); this pace decides where its formal
documentation lives and writes it there.

## Goal

Lift the self-contained mint record from rbndb_base.sh into the
canonical project documentation location for the rbn{R}{X}_ convention.

## Locked decisions

- Convention: `rbn{regime-letter}{kind-letter}_` is a regime-affiliated
  namespace that allows minting under a terminal regime prefix. Decode:
  `rb` (project) + `n` (extension marker, sidesteps terminal exclusivity
  on regime prefixes) + regime letter + kind letter. Existing uses to
  enumerate: rbnnh_ (nameplate hook), rbndf_ (depot fact image),
  rbndb_ (depot regime bespoke helpers).

## What to decide at mount

- Canonical doc home — candidates: CLAUDE.md prefix registry section,
  a buk*.md guide, BCG addendum, the acronym-selection minting study
  memo (Memos/memo-20260110-...), or a new dedicated location.
  Decide based on the convention's character: project-level naming
  rule (CLAUDE.md fits), BUK-pattern (buk*.md fits), or BCG-internal
  discipline (BCG fits).
- Cross-references: regardless of canonical home, CLAUDE.md's Project
  Prefix Registry or minting workflow should reference the convention
  so future minters discover it.

## Done

- Convention documented in the chosen location with letter-position
  decoding, existing uses enumerated, guidance for future application.
- rbndb_base.sh header trimmed to point at the canonical location
  rather than restating the convention.
- CLAUDE.md references the convention (at minimum a pointer; full
  prose may live elsewhere depending on the home decision).

**[260517-0907] rough**

## Character

Design conversation requiring judgment. The convention is already in
use by code (rbnnh_ for nameplate hooks, rbndf_ for depot facts, rbndb_
for depot regime bespoke helpers); this pace decides where its formal
documentation lives and writes it there.

## Goal

Lift the self-contained mint record from rbndb_base.sh into the
canonical project documentation location for the rbn{R}{X}_ convention.

## Locked decisions

- Convention: `rbn{regime-letter}{kind-letter}_` is a regime-affiliated
  namespace that allows minting under a terminal regime prefix. Decode:
  `rb` (project) + `n` (extension marker, sidesteps terminal exclusivity
  on regime prefixes) + regime letter + kind letter. Existing uses to
  enumerate: rbnnh_ (nameplate hook), rbndf_ (depot fact image),
  rbndb_ (depot regime bespoke helpers).

## What to decide at mount

- Canonical doc home — candidates: CLAUDE.md prefix registry section,
  a buk*.md guide, BCG addendum, the acronym-selection minting study
  memo (Memos/memo-20260110-...), or a new dedicated location.
  Decide based on the convention's character: project-level naming
  rule (CLAUDE.md fits), BUK-pattern (buk*.md fits), or BCG-internal
  discipline (BCG fits).
- Cross-references: regardless of canonical home, CLAUDE.md's Project
  Prefix Registry or minting workflow should reference the convention
  so future minters discover it.

## Done

- Convention documented in the chosen location with letter-position
  decoding, existing uses enumerated, guidance for future application.
- rbndb_base.sh header trimmed to point at the canonical location
  rather than restating the convention.
- CLAUDE.md references the convention (at minimum a pointer; full
  prose may live elsewhere depending on the home decision).

### cygwin-wsl-noninteractive-ssh-personas (₢BOAAN) [complete]

**[260517-1117] complete**

## Character
Substrate infrastructure — mechanical setup following an established
recipe. Predictable scope.

## Goal
Two parallel headless SSH accounts on rocket — `cygwin@rocket` and
`wsl@rocket` — each forced-command'd to one substrate, both
noninteractive. Enables Claude-driven theurge iteration from the
operator's Mac and gives a free WSL escape-hatch for substrate
comparison.

## Shape
Two applications of the worked example in
`Memos/memo-20260516-windows-headless-account-anatomy.md`. Differences
from brad@rocket: forced-command field in `authorized_keys` commits
each session to one substrate (cygwin → Cygwin bash; wsl → `wsl.exe
-- $SSH_ORIGINAL_COMMAND`). Forced-command quoting through
cmd.exe→wsl.exe is the sharp edge — empirical pass per the
windows-transport memo pattern (e.g.,
`Memos/memo-20260508-windows-transport-experiments.md`).

## Prereqs
WSL2 runtime present on rocket; no distro installed yet (2026-05-17).
Operator runs `wsl --install -d <distro>` from an admin Windows context
before `wsl@rocket` can carry useful commands.

## Done
`ssh cygwin@rocket "uname -o"` returns Cygwin output and exits
noninteractively. `ssh wsl@rocket "uname -o"` returns Linux output and
exits noninteractively. brad@rocket continues to work as the
interactive ad-hoc account — additive, not replacement.

**[260517-1010] rough**

## Character
Substrate infrastructure — mechanical setup following an established
recipe. Predictable scope.

## Goal
Two parallel headless SSH accounts on rocket — `cygwin@rocket` and
`wsl@rocket` — each forced-command'd to one substrate, both
noninteractive. Enables Claude-driven theurge iteration from the
operator's Mac and gives a free WSL escape-hatch for substrate
comparison.

## Shape
Two applications of the worked example in
`Memos/memo-20260516-windows-headless-account-anatomy.md`. Differences
from brad@rocket: forced-command field in `authorized_keys` commits
each session to one substrate (cygwin → Cygwin bash; wsl → `wsl.exe
-- $SSH_ORIGINAL_COMMAND`). Forced-command quoting through
cmd.exe→wsl.exe is the sharp edge — empirical pass per the
windows-transport memo pattern (e.g.,
`Memos/memo-20260508-windows-transport-experiments.md`).

## Prereqs
WSL2 runtime present on rocket; no distro installed yet (2026-05-17).
Operator runs `wsl --install -d <distro>` from an admin Windows context
before `wsl@rocket` can carry useful commands.

## Done
`ssh cygwin@rocket "uname -o"` returns Cygwin output and exits
noninteractively. `ssh wsl@rocket "uname -o"` returns Linux output and
exits noninteractively. brad@rocket continues to work as the
interactive ad-hoc account — additive, not replacement.

### cygwin-theurge-path-transmute (₢BOAAM) [complete]

**[260519-1854] complete**

## Character
Experimental — outcome uncertain. Three boundary classes, one chokepoint
each; if theurge clears but BUK-side bash stumbles surface as the next
blocker, stop and reassess rather than chase BUK fixes in this pace.

## Goal
Make `tt/rbtd-s.TestSuite.fast.sh` reach a verdict (pass or content-fail)
on a Cygwin shell against an `x86_64-pc-windows-gnu` theurge binary, by
systematizing POSIX↔Windows path conversion at theurge's bash/Rust
boundary. Currently the gauntlet aborts in `enrollment-validation` with
a substrate-level path mangling — Rust's `PathBuf::join` produces mixed
`/cygdrive/c/...\rbtd` paths Windows resolves unpredictably.

## Shape
New platform module under `Tools/rbk/rbtd/src/` with two helpers:
POSIX→native PathBuf for intake, native Path→POSIX String for outflow.
Cygwin detected at runtime (cache once); identity on Linux/macOS so
existing platforms unaffected. Apply at three boundary classes:

- env-var intake from Cygwin bash — single chokepoint
  `rbtdb_read_dispatch_dir` in `Tools/rbk/rbtd/src/main.rs`
- script-embedded paths handed to `Command::new("bash")` — discoverable
  via `grep -rn "source '{}'" Tools/rbk/rbtd/src/` and adjacent
  `format!()` interpolations of `path.display()`
- env vars on spawned tabtarget invocations — re-convert to POSIX
  before `Command::env`

Windows-native tool spawns (git, curl, docker) need no conversion.

## Test surface
`brad@rocket` Cygwin login (Memos/memo-20260516-windows-headless-account-anatomy.md).
Rust toolchain already installed there as `x86_64-pc-windows-gnu` via
`rustup-init.exe` from Cygwin shell.

**[260517-1005] rough**

## Character
Experimental — outcome uncertain. Three boundary classes, one chokepoint
each; if theurge clears but BUK-side bash stumbles surface as the next
blocker, stop and reassess rather than chase BUK fixes in this pace.

## Goal
Make `tt/rbtd-s.TestSuite.fast.sh` reach a verdict (pass or content-fail)
on a Cygwin shell against an `x86_64-pc-windows-gnu` theurge binary, by
systematizing POSIX↔Windows path conversion at theurge's bash/Rust
boundary. Currently the gauntlet aborts in `enrollment-validation` with
a substrate-level path mangling — Rust's `PathBuf::join` produces mixed
`/cygdrive/c/...\rbtd` paths Windows resolves unpredictably.

## Shape
New platform module under `Tools/rbk/rbtd/src/` with two helpers:
POSIX→native PathBuf for intake, native Path→POSIX String for outflow.
Cygwin detected at runtime (cache once); identity on Linux/macOS so
existing platforms unaffected. Apply at three boundary classes:

- env-var intake from Cygwin bash — single chokepoint
  `rbtdb_read_dispatch_dir` in `Tools/rbk/rbtd/src/main.rs`
- script-embedded paths handed to `Command::new("bash")` — discoverable
  via `grep -rn "source '{}'" Tools/rbk/rbtd/src/` and adjacent
  `format!()` interpolations of `path.display()`
- env vars on spawned tabtarget invocations — re-convert to POSIX
  before `Command::env`

Windows-native tool spawns (git, curl, docker) need no conversion.

## Test surface
`brad@rocket` Cygwin login (Memos/memo-20260516-windows-headless-account-anatomy.md).
Rust toolchain already installed there as `x86_64-pc-windows-gnu` via
`rustup-init.exe` from Cygwin shell.

### named-service-charge-predicate (₢BOAAO) [complete]

**[260519-1812] complete**

## Character
Mechanical implementation with discipline-reading prerequisite.

## Goal
Replace `rbob_charged_predicate` in `Tools/rbk/rbob_bottle.sh` with an
explicit per-service check: sentry, pentacle, and bottle must each be
individually `running` for the predicate to return 0.

## Locked
- **Read BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) before
  writing any bash.** The current predicate's `2>/dev/null` is itself
  a BCG violation being corrected here; the replacement must not
  introduce others.
- **Capture compose stderr to a file under `BURD_TEMP_DIR`** — do not
  redirect to `/dev/null`. Operators need to inspect daemon errors
  after a verify-active failure.

## Done
Predicate checks the three named services individually; stderr
captured to a temp file; behavioral contract unchanged from caller's
view (0 = charged, 1 = not, never dies). A moriah fixture trial
confirms the new predicate behaves correctly.

**[260519-1650] rough**

## Character
Mechanical implementation with discipline-reading prerequisite.

## Goal
Replace `rbob_charged_predicate` in `Tools/rbk/rbob_bottle.sh` with an
explicit per-service check: sentry, pentacle, and bottle must each be
individually `running` for the predicate to return 0.

## Locked
- **Read BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) before
  writing any bash.** The current predicate's `2>/dev/null` is itself
  a BCG violation being corrected here; the replacement must not
  introduce others.
- **Capture compose stderr to a file under `BURD_TEMP_DIR`** — do not
  redirect to `/dev/null`. Operators need to inspect daemon errors
  after a verify-active failure.

## Done
Predicate checks the three named services individually; stderr
captured to a temp file; behavioral contract unchanged from caller's
view (0 = charged, 1 = not, never dies). A moriah fixture trial
confirms the new predicate behaves correctly.

### cloud-build-timeout-naming-audit (₢BOAAQ) [complete]

**[260519-1837] complete**

## Character
Design conversation requiring judgment — enumerate, name, possibly
rename. Mint phase.

## Goal
Audit magic numbers governing Cloud Build dispatch await/poll/timeout
behavior across `rbgp_Payor.sh`, `rbfd_FoundryDirectorBuild.sh`,
`rbfv_FoundryVerify.sh`, and the cloud-side `curl --max-time` values in
`zrbgp_write_posture_check`. Mint a coherent naming scheme that encodes
policy in the name, not the value. Where call sites share semantics
(e.g., "Cloud Build terminal-state poll budget"), names should share too.

## Locked
- Read BCG before touching bash — kindle vs tinder placement turns on
  source-time dependency.
- Scope: build dispatch path only. Do not sweep unrelated magic numbers
  (test framework intervals, CLI rendering, regime values).

## Done
Every magic timeout/poll/interval in the build dispatch path is a named
constant whose identifier reflects the policy it encodes. The two
constants minted in ₢BOAAP (`ZRBGP_BUILD_POLL_CEILING`,
`ZRBGP_BUILD_POLL_INTERVAL`) are reviewed against the broader scheme and
renamed if the audit suggests better names. Any cross-module naming
agreement is documented in the heat's paddock or via spec annotation.

**[260519-1116] rough**

## Character
Design conversation requiring judgment — enumerate, name, possibly
rename. Mint phase.

## Goal
Audit magic numbers governing Cloud Build dispatch await/poll/timeout
behavior across `rbgp_Payor.sh`, `rbfd_FoundryDirectorBuild.sh`,
`rbfv_FoundryVerify.sh`, and the cloud-side `curl --max-time` values in
`zrbgp_write_posture_check`. Mint a coherent naming scheme that encodes
policy in the name, not the value. Where call sites share semantics
(e.g., "Cloud Build terminal-state poll budget"), names should share too.

## Locked
- Read BCG before touching bash — kindle vs tinder placement turns on
  source-time dependency.
- Scope: build dispatch path only. Do not sweep unrelated magic numbers
  (test framework intervals, CLI rendering, regime values).

## Done
Every magic timeout/poll/interval in the build dispatch path is a named
constant whose identifier reflects the policy it encodes. The two
constants minted in ₢BOAAP (`ZRBGP_BUILD_POLL_CEILING`,
`ZRBGP_BUILD_POLL_INTERVAL`) are reviewed against the broader scheme and
renamed if the audit suggests better names. Any cross-module naming
agreement is documented in the heat's paddock or via spec annotation.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 D levy-establish-probes
  2 A highcpu-build-substep-bench
  3 P probe-egress-assertion-repair
  4 B depot-regime-now
  5 C srjcl-jupyter-reverify
  6 F levy-pool-lro-await
  7 G theurge-buk-temp-discipline
  8 H seed-skip-orphan-boaah
  9 J rbrd-regime-split
  10 K rbrd-tripwire-wire
  11 N cygwin-wsl-noninteractive-ssh-personas
  12 M cygwin-theurge-path-transmute
  13 O named-service-charge-predicate
  14 Q cloud-build-timeout-naming-audit

DAPBCFGHJKNMOQ
x·x··x··xx···x rbgp_Payor.sh
x·x··x··xx···· RBSDE-depot_levy.adoc
·x······xx···x rbfd_FoundryDirectorBuild.sh
······xx···x·· lib.rs, main.rs, rbtdri_invocation.rs
····x···x···x· rbob_bottle.sh
····x···xx···· RBS0-SpecTop.adoc
··x·····xx···· rbk-claude-tabtarget-context.md, rbz_zipper.sh
x·······xx···· rbgc_Constants.sh
·········x···x rbfl_FoundryLedger.sh, rbfv_FoundryVerify.sh
········x··x·· rbtdrf_fast.rs
········xx···· RBSRT-RegimeDepot.adoc, rbfd_cli.sh, rbfl_cli.sh, rbfv_cli.sh, rbgp_cli.sh, rbrd_cli.sh
······xx······ rbtdth_helpers.rs, rbtdti_invocation.rs
·x······x····· rblm_cli.sh, rbrr.env
xx············ rbrv.env
·············x rbfc_FoundryCore.sh
···········x·· rbtdrx_platform.rs, rbtdtx_platform.rs
·········x···· rbhopw_payor_wrapper.sh, rbndb_base.sh, rbw-rdc.CheckDepotRegime.sh, rbw-rdi.InscribeDepotRegime.sh
········x····· RBSAJ-access_jwt_probe.adoc, RBSDI-depot_inscribe.adoc, RBSDK-director_invest.adoc, RBSDU-depot_unmake.adoc, RBSDY-director_yoke.adoc, RBSGM-governor_mantle.adoc, RBSIA-image_audit.adoc, RBSIR-image_rekon.adoc, RBSIW-image_wrest.adoc, RBSQB-quota_build.adoc, RBSRK-retriever_invest.adoc, RBSRR-RegimeRepo.adoc, RBSTB-trigger_build.adoc, README.md, rbbc_constants.sh, rbdc_DerivedConstants.sh, rbfc_cli.sh, rbfh_cli.sh, rbfk_cli.sh, rbfr_cli.sh, rbgd_DepotConstants.sh, rbgg_cli.sh, rbgv_AccessProbe.sh, rbho0_cli.sh, rbhocc_crash_course.sh, rbhoda_director_airgap.sh, rbhodb_director_bind.sh, rbhodf_director_first_build.sh, rbhodg_director_graft.sh, rbhp0_cli.sh, rbhpq_quota_build.sh, rbob_cli.sh, rbob_compose.yml, rbra_cli.sh, rbrd.env, rbrd_regime.sh, rbrn_cli.sh, rbro_cli.sh, rbrr_cli.sh, rbrr_regime.sh, rbrv_cli.sh, rbtdrk_canonical.rs, rbtdrp_pristine.rs, rbte_cli.sh, rbv_cli.sh, rbw-rdr.RenderDepotRegime.sh, rbw-rdv.ValidateDepotRegime.sh, rbyc_common.sh
·······x······ rbtdrc_crucible.rs, rbtdre_engine.rs, rbtdrg_log.rs
······x······· rbtdte_engine.rs, rbtdtk_canonical.rs, rbtdtl_calibrant.rs, rbtdtp_pristine.rs
····x········· .gitignore, RBSCC-crucible_charge.adoc, RBSCH-charge_hook.adoc, claude-netdiags.ipynb, compose.yml, ipycl-name-genie.ipynb, namegenie.py, netdiag.py, rbk-claude-acronyms.md, rbnnh_compose.yml, rbnnh_post_charge.sh
··x··········· memo.md, rbw-di.DepotInfo.sh
·x············ jjk-claude-context.md, memo-20260516-cloudbuild-machinetype-bench.md
x············· rbgl_GarLayout.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 87 commits)

  1 N cygwin-wsl-noninteractive-ssh-personas
  2 F levy-pool-lro-await
  3 O named-service-charge-predicate
  4 P probe-egress-assertion-repair
  5 Q cloud-build-timeout-naming-audit
  6 G theurge-buk-temp-discipline
  7 M cygwin-theurge-path-transmute
  8 J rbrd-regime-split
  9 K rbrd-tripwire-wire

123456789abcdefghijklmnopqrstuvwxyz
····x······························  N  1c
·············x·····················  F  1c
···············xx··················  O  2c
··················xx···············  P  2c
····················xx·············  Q  2c
······················x············  G  1c
························xx·········  M  2c
··························x·x······  J  2c
·····························xxxxx·  K  5c
```

## Steeplechase

### 2026-05-19 15:34 - Heat - T

rbnRX-convention-document

### 2026-05-19 15:31 - ₢BOAAK - W

RBRD depot-regime tripwire complete: drift now fails loud. rbndb_base.sh (new bespoke module under the rbn{R}{X}_ convention) exposes rbrd_inscribe and rbrd_check; zrbndb_kindle composes the FQN rbi_df/rbrd:tripwire once from RBRD+RBDC+RBGL. Producer: rbgp_depot_levy calls rbrd_inscribe at the tail of a successful levy (Payor OAuth token, FROM-scratch image carrying .rbk/rbrd.env, docker manifest-inspect existence guard fatal-on-present). Consumers: rbrd_check inlined at all seven foundry cloud-submit sites (3 in rbfd, 1 in rbfl, 3 in rbfv), each passing its director token; pull + cmp -s exact-byte-diff against local rbrd.env, fatal on drift/missing/auth-fail with recovery guidance. rbrd_cli.sh gained differential furnish (validate/render light, check/inscribe heavy) and exposes both ops via rbw-rdc/rbw-rdi tabtargets (param1 token channel). BCG-compliant throughout: line-2 SC2153 directive, local -r discipline, stderr-to-temp-file capture referenced in die messages, kindle-constant temp prefixes. Docs: RBSDE gained the inscribe step; RBSRT gained the rbrd_tripwire linked term (registered in RBS0 mapping section) documenting producer/consumer contract; rbgc_Constants.sh rbi_df comment block softened to enumerate rbrd:tripwire as a resident artifact; payor handbook documents the three failure modes (drift, missing, re-inscribe-refused) with recovery commands. Four commits (eb66b5f7, 7247681d, 7e5fbe66, + this wrap). Verification: fast suite 107/107 green across all chunks including handbook-render of the edited payor wrapper. NOT live-verified end-to-end: the heat's 96-core quota bars re-levying canest, so the fresh-inscribe path and live drift detection await the next depot regeneration. No local asciidoctor — spec edits validated structurally (linked-term three-part completeness) not rendered.

### 2026-05-19 15:31 - ₢BOAAK - n

Wire RBRD tripwire into the human handbook and three specs, completing pace BOAAK's documentation arc.

### 2026-05-19 12:55 - ₢BOAAK - n

Wire tripwire producer at end of levy + consumer checks at seven cloud-submitting sites. rbgp_Payor.sh's depot_levy now calls `rbrd_inscribe "${z_token}"` between zrbgp_depot_list_update and the buc_success completion banner — Payor's already-acquired OAuth token authorizes the FROM-scratch image push to rbi_df/rbrd:tripwire. rbgp_cli.sh furnish gains rbgl_GarLayout.sh + rbndb_base.sh sourcing plus zrbgl_kindle + zrbndb_kindle so the FQN composes correctly in the depot_levy context (rbndb depends on rbgc/rbgl/rbrd/rbdc sentinels — all already kindled by existing furnish lines). Each of the three foundry CLIs (rbfd_cli.sh, rbfl_cli.sh, rbfv_cli.sh) gains rbndb_base.sh source + zrbndb_kindle (rbgl already present in all three from prior work). The seven cloud-submit call sites get a single-line `rbrd_check "${z_token}"` inserted immediately before the `rbgu_http_json POST ${ZRBFC_GCB_PROJECT_BUILDS_URL}` invocation, using the same director-role bearer token the submission itself carries: zrbfd_enshrine_submit (line ~1045), rbfd_build (line ~1283), zrbfd_mirror_submit (line ~1610), zrbfl_inscribe_submit (line ~148), zrbfv_graft_metadata_submit (line ~403), zrbfv_about_submit (line ~566), zrbfv_vouch_submit (line ~802). The Payor pool-probe submits (zrbgp_pool_probe_submit, called twice during levy itself) are NOT instrumented — they run BEFORE rbrd_inscribe so there's no tripwire to check against. Verification: fast suite 107/107 green across two runs (one after Task 3, one after Task 4); the new dispatch chains source/kindle without errors. End-to-end inscribe/check verification is blocked by the heat's 96-core quota constraint — the canest depot cannot be re-levied to exercise the fresh-inscribe path, and live drift testing would require modifying production rbrd.env. Spec updates (RBSDE inscribe step, RBSRT kindle-time check) and handbook drift-failure-mode section follow in the next commit.

### 2026-05-19 12:50 - ₢BOAAK - n

Wire rbrd_cli.sh differential dispatch for tripwire commands, enroll rbw-rdc/rbw-rdi tabtargets, BCG-comply rbndb_base.sh, and soften rbgc_Constants.sh rbi_df comment block. rbrd_cli.sh's furnish now receives the command name as $1 and applies differential setup: rbrd_validate/rbrd_render stay lightweight (just RBRD enrollment); rbrd_check/rbrd_inscribe additionally source and kindle rbrr_regime, rbgc_Constants, rbgl_GarLayout, rbdc_DerivedConstants, and rbndb_base so the tripwire image FQN composes from validated regime state. rbz_zipper.sh gains two enrollments with channel=param1 (RBZ_INSCRIBE_DEPOT→rbw-rdi, RBZ_CHECK_DEPOT→rbw-rdc); both functions accept the bearer token from positional $1 or BUZ_FOLIO (the param1-channel dispatch path), so consuming CLIs can call rbrd_check $z_token in-process while operators can run `tt/rbw-rdc.sh $(gcloud auth print-access-token)` for manual drift verification. tt/rbw-rdc.CheckDepotRegime.sh and tt/rbw-rdi.InscribeDepotRegime.sh added as standard three-line launcher tabtargets. rbndb_base.sh rewritten for BCG compliance: line-2 shellcheck disable=SC2153 directive added, all assigned-once locals use local -r, temp file paths declared as kindle constants (ZRBNDB_INSCRIBE_PREFIX/ZRBNDB_CHECK_PREFIX) with per-op stderr files referenced in buc_die messages (`see ${z_*_stderr}` pattern from BCG line 542), 2>/dev/null suppressions all replaced with stderr→temp-file capture, docker create result captured via $(<temp_stdout_file) two-line pattern instead of inline $(cmd), diff exit-status explicitly handled (≤1 = expected, ≥2 = real error), docker rm cleanup failure downgraded to buc_warn with stderr file reference rather than `|| true` suppression. buc_doc_brief/buc_doc_param/buc_doc_shown added to rbrd_inscribe and rbrd_check so they participate in the CLI help system; module-level header documents the dual token-source pattern (positional $1 vs BUZ_FOLIO). rbgc_Constants.sh lines 175-189 updated: the comment block previously asserted RBRD shipped 'at a separate tag' outside rbi_df; revised to enumerate rbi_df's current artifacts (probe-tether:probe, probe-airgap:probe, rbrd:tripwire) and explain that rbrd:tripwire is host-inscribed by Payor at end of levy and consulted at every cloud submission, cross-referencing Tools/rbk/rbndb_base.sh. rbk-claude-tabtarget-context.md regenerated to surface the two new tabtargets under the Regime section. Verification: fast suite 107/107 green on both runs (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15, dockerfile-hygiene 9) — validate/render path unaffected by differential furnish, new dispatch routes don't break enrollment validation.

### 2026-05-19 12:37 - ₢BOAAK - n

Mint rbndb_base.sh under the rbn{R}{X}_ convention (depot regime bespoke implementation). Module exposes rbrd_inscribe and rbrd_check as public functions, with zrbndb_kindle composing the tripwire image FQN once (`${region}-docker.pkg.dev/${depot_project}/${gar_repo}/rbi_df/rbrd:tripwire`) from RBRD + RBDC + RBGL state. Internal zrbndb_docker_login authenticates the host docker client to GAR via `docker login -u oauth2accesstoken --password-stdin` with a bearer token; idempotent. rbrd_inscribe runs host-side at end of successful levy: pre-push existence guard via `docker manifest inspect` (present → fatal with unmake+relevy guidance), then builds a FROM-scratch image with COPY rbrd.env /rbrd.env and pushes. rbrd_check is role-agnostic and called inline at every cloud-submit site: manifest-inspect (absent → fatal), `docker pull`, `docker create`+`docker cp` to extract /rbrd.env, `cmp -s` for authoritative byte-match (diff -u for human display on mismatch), fatal on drift with restore/relevy recovery options. Module not yet wired — rbrd_cli.sh dispatch + rbw-rdc/rbw-rdi tabtargets, end-of-levy hook in rbgp_depot_levy, and inline checks at the seven foundry submit sites (zrbfd_enshrine_submit, rbfd_build, zrbfd_mirror_submit, zrbfl_inscribe_submit, zrbfv_graft_metadata_submit, zrbfv_about_submit, zrbfv_vouch_submit) follow in subsequent commits. RBSDE/RBSRT spec updates, rbgc_Constants.sh rbi_df comment-block softening, and handbook drift-failure-mode section also pending.

### 2026-05-19 12:22 - ₢BOAAJ - W

RBRD depot regime minted. Four depot-time-immutable settings (RBRR_CLOUD_PREFIX, RBRR_DEPOT_MONIKER, RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE) renamed to RBRD_* and relocated from rbrr.env to a new .rbk/rbrd.env. New rbrd_regime.sh + rbrd_cli.sh follow the sibling regime archetype uniformly; joint-length cap on (CLOUD_PREFIX + d- + DEPOT_MONIKER) moved to zrbrd_enforce. New RBSRT-RegimeDepot.adoc spec with linked-term definitions; RBS0 reorganized to host a 'Depot Regime (RBRD)' section between RBRR and RBGC, with rbrd_prefix glossary entry and group anchors. New rbw-rdr/rbw-rdv tabtargets enrolled via zipper. About 30 consumers migrated across .sh/.adoc/.rs: 18 CLI dispatchers now source/kindle/enforce both rbrr and rbrd; rblm_zero blanks both files in one commit with split case patterns and pre-confirmation surfaces both targets; rbob_compose.yml interpolates ${RBRD_GCP_REGION} and rbob_bottle.sh wires .rbk/rbrd.env into the compose --env-file list; test fixtures rbtdrp_pristine.rs, rbtdrk_canonical.rs, rbtdrf_fast.rs split RBRR/RBRD read/write paths and source/kindle/enforce both regimes in their bash heredocs. README appendix gains RBRD entry; crash-course handbook adds depot-regime validation step + 'd RBRD rbw-rdr rbw-rdv' to the regime table; RBYC_RBRD common yelp constant added. Stale 'future RBRD in rbi_df' comment block in rbgc_Constants.sh refreshed to reflect that RBRD ships as its own image at a separate tag. Tabtarget context regenerated. Validation: rbw-rdv + rbw-rrv both clean against the populated rbrr.env/rbrd.env pair; fast suite 107/107 green (enrollment 47, regime-validation 27, regime-smoke 9, handbook-render 15, dockerfile-hygiene 9); rbw-tf qualify-fast passes. Rebase conflict in rbtdrf_fast.rs with sibling pace ₢BOAAM (Cygwin path transmutation, ea2d04b1) resolved by keeping their rbtdrx_native_to_posix wrappers for path interpolations and merging my added rbrd source-line slots; const reference at line 1183 corrected from stale RBTDRF_VAR_RBRR_CLOUD_PREFIX to RBTDRF_VAR_RBRD_CLOUD_PREFIX. Pushed to main as fcd56168. Naming split only — tripwire enforcement (so a hand-edit of rbrd.env post-levy is not a silent no-op) is the next pace, per locked docket decision.

### 2026-05-19 12:41 - Heat - n

Augment substrate-landscape memo with Cygwin verification findings for ₢BOAAM. New H2 'Cygwin substrate, revisited 2026-05-19' retires the Unknown Tail prediction with empirical content from the rocket gauntlet trial: path-transmutation code is correct but its OSTYPE detection signal isn't an exported env var so the system silently no-ops; with OSTYPE forced, Command::new("bash") in rbtdrf_fast.rs:68 resolves via CreateProcessW's System32-before-PATH search to Windows's WSL stub instead of Cygwin's /usr/bin/bash, aborting cases with UTF-16LE 'no installed distributions' from wsl.exe. Section also documents the install prerequisites that made the trial possible (Rust 1.95 self-contained linker eliminates the MinGW system-install requirement, TMPDIR must be a Windows-form path because Schannel curl can't write to Cygwin /tmp paths, --default-host must not be passed because the rustup-init.sh wrapper inserts its own) and the SSH transport finding (cygwin@rocket's bash --login -c forced-command mangles argv-mode $var expansion; stdin-pipe form is reliable). Updates Live work captured in ₣BO entry for ₢BOAAM from outcome-uncertain to landed/verified with the two follow-on blockers named.

### 2026-05-19 12:14 - ₢BOAAJ - n

Mint RBRD depot regime: peel CLOUD_PREFIX, DEPOT_MONIKER, GCP_REGION, GCB_MACHINE_TYPE out of RBRR into rbrd_regime.sh + .rbk/rbrd.env, sibling-archetype rbrd_cli.sh, new RBSRT-RegimeDepot.adoc spec + RBS0 reorganization, rbw-rdr/rbw-rdv tabtargets via zipper, joint-length cap moved to zrbrd_enforce. Migrate ~30 consumers (CLI dispatchers source/kindle/enforce both regimes; rblm_zero blanks both files in one commit; compose.yml + rbob_bottle.sh wire rbrd.env into compose --env-file; test fixtures rbtdrp/rbtdrk/rbtdrf split read/write across the two files). README appendix + crash-course handbook surface RBRD; stale rbi_df comment in rbgc_Constants refreshed; tabtarget context regenerated. Naming split only — tripwire enforcement is the next pace.

### 2026-05-19 18:54 - ₢BOAAM - W

POSIX↔Windows path transmutation at theurge's bash/Rust boundary. New rbtdrx_platform module exposes rbtdrx_posix_to_native (Result<PathBuf>) for intake and rbtdrx_native_to_posix (String) for outflow, with OnceLock-cached Cygwin detection via OSTYPE=cygwin. Fast paths handle /cygdrive/X/... ↔ X:\... inline; bare POSIX paths fall back to shelling out to cygpath. On Linux/macOS both helpers are identity. Three boundaries wired: (1) rbtdb_read_dispatch_dir at main.rs:57 converts BURD_TEMP_DIR/BURD_OUTPUT_DIR on intake; (2) ten format!() sites in rbtdrf_fast.rs that embed `source '{}'` heredocs now interpolate rbtdrx_native_to_posix(&path) instead of path.display(); (3) rbtdri_invoke_impl converts burv_output/burv_temp before .env() so Cygwin-bash tabtargets receive POSIX BURV_*_DIR vars. New rbtdtx_platform.rs adds 18 unit tests exercising both identity (is_cygwin=false) and Cygwin (is_cygwin=true) branches via pub(crate) impl helpers — public API caches detection via OnceLock so tests target the explicit-parameter functions. Linux verification clean: fast suite 107/107 green, all 114 lib unit tests pass on rerun (one flaky rbtdti_invoke_passes_args 'text file busy' on first run is parallel-test infrastructure unrelated to this pace). Cygwin verification not performed under this pace: this curia host's SSH known_hosts didn't trust rocket so the actual x86_64-pc-windows-gnu Cygwin gauntlet trial was impossible. The docket's stated Done condition (`tt/rbtd-s.TestSuite.fast.sh` reaches a verdict on Cygwin) was therefore not exercised; a follow-on pace on a rocket-trusting host should build theurge as x86_64-pc-windows-gnu from Cygwin, run the fast suite, exercise the cygpath fallback for bare /home/... BURD_TEMP_DIR shapes, and apply the docket's stop-and-reassess discipline if theurge clears but BUK-side stumbles surface.

### 2026-05-19 18:53 - ₢BOAAM - n

Initial cut of POSIX↔Windows path transmutation at theurge's bash/Rust boundary. New rbtdrx_platform module exposes rbtdrx_posix_to_native (Result<PathBuf>) for intake and rbtdrx_native_to_posix (String) for outflow, with OnceLock-cached Cygwin detection via OSTYPE=cygwin. Fast paths handle /cygdrive/X/... ↔ X:\... inline; bare POSIX paths (/home/..., /tmp/...) fall back to shelling out to cygpath. On Linux/macOS both helpers are identity. Three boundaries wired: rbtdb_read_dispatch_dir at main.rs:57 converts BURD_TEMP_DIR/BURD_OUTPUT_DIR on intake; ten format!() sites in rbtdrf_fast.rs that embed `source '{}'` heredocs now interpolate rbtdrx_native_to_posix(&path) instead of path.display(); rbtdri_invoke_impl converts burv_output/burv_temp before .env() so Cygwin-bash tabtargets receive POSIX BURV_*_DIR vars. New rbtdtx_platform.rs adds 18 unit tests exercising both identity (is_cygwin=false) and Cygwin (is_cygwin=true) branches via pub(crate) impl helpers — the public API caches detection via OnceLock so tests target the explicit-parameter functions. Linux verification clean: fast suite 107/107 green, all 114 lib unit tests pass on rerun (one flaky rbtdti_invoke_passes_args 'text file busy' on first run is parallel-test infrastructure unrelated to this pace). Cygwin verification deferred: this host's SSH known_hosts doesn't trust rocket, so the actual x86_64-pc-windows-gnu/Cygwin gauntlet trial must happen from a host that does — pace will remount there to exercise the fast-path inline mapping and the cygpath fallback for bare /home/... BURD_TEMP_DIR shapes.

### 2026-05-19 11:32 - Heat - T

post-levy-quota-bump-flow

### 2026-05-19 11:26 - ₢BOAAG - W

Theurge BURV sandbox anchored under BUK dispatch-provided BURD_TEMP_DIR/BURD_OUTPUT_DIR (commit 00d99794): rbtdri_Context split into burv_temp_root + burv_output_root mirroring BUK's temp/output split, main.rs locals adopted rbtdb_ prefix (rbtdb_Roots, rbtdb_read_dispatch_dir, rbtdb_allocate_roots, rbtdb_run_suite, rbtdb_run_single, rbtdb_list_fixtures), crate root carries #![deny(warnings)] + #![allow(private_interfaces)]. Theurge dies immediately if dispatch envs absent — no fallback. Tests share the discipline via rbtdth_scratch_root() panic-on-unset helper; direct cargo test outside BUK no longer a supported workflow. Forensic artifacts (per-call HTTP captures from rbgu_http_json) now survive reboot and systemd-tmpfiles cleanup, matching standalone-workbench behavior. Follow-up cleanup pace ₢BOAAH (commit d7145e0c) swept 42 naked eprintln!/println! sites to rbtdrg_*_now! macros and centralized BURD_TEMP_DIR_KEY/BURD_OUTPUT_DIR_KEY constants in rbtdri_invocation.rs.

### 2026-05-19 18:37 - ₢BOAAQ - W

Cloud Build dispatch timeout/poll/interval naming audit landed. Minted ZRBFC_BUILD_POLL_* family in rbfc_FoundryCore.sh kindle: INTERVAL_SEC=5, RETRY_TOLERANCE=3, and seven CEILING_* per-build constants (INSCRIBE=120, ENSHRINE=50, CONJURE=960, MIRROR=100, ABOUT_VOUCH=100, ABOUT=50, VOUCH=50). Replaced sleep 5 at rbfc:264 and z_max_consecutive_failures=3 throughout zrbfc_wait_build_completion; the seven foundry callers (rbfl, rbfd×3, rbfv×3) now pass the named constant rather than a literal. Renamed BOAAP's pool-build constants: ZRBGP_BUILD_POLL_CEILING → ZRBGP_POOL_BUILD_POLL_CEILING and ZRBGP_BUILD_POLL_INTERVAL → ZRBGP_POOL_BUILD_POLL_INTERVAL_SEC (added _POOL_ to disambiguate diagnostic pool builds from payload-carrying foundry builds; added _SEC unit suffix for cross-family consistency). Kindled ZRBGP_POSTURE_REQUEST_MAX_TIME_SEC=10 and rewrote four zrbgp_write_posture_check lines from single to double quotes so host-side expansion fills the value into the cloud-destined script — BCG-compliant (no host-side heredoc). Cross-module naming agreement (rbgp _POOL_BUILD_POLL_ family vs rbfc BUILD_POLL_ family) carried in the two kindle-block comments per operator direction; no separate paddock/spec annotation. Fast suite 107/107 green; live rbw-di.DepotInfo against canest depot 100016 SUCCESS on both pools (tether 75s/15 polls, airgap 70s/14 polls), confirming the host-side curl --max-time interpolation reaches the cloud worker correctly and the renamed ZRBGP_POOL_BUILD_POLL_CEILING is read by the await primitive. Tier D (implicit Cloud Build wall-clock and queueTtl defaults on pool builds) deferred to a follow-on pace per operator scope discipline — pool builds currently rely on Cloud Build's 10-min default timeout and 3600s default queueTtl, neither named in code.

### 2026-05-19 18:37 - ₢BOAAQ - n

Cloud Build timeout naming audit: extract foundry wait_build_completion poll budgets to named ZRBFC_BUILD_POLL_CEILING_* constants (INSCRIBE/ENSHRINE/CONJURE/MIRROR/ABOUT_VOUCH/ABOUT/VOUCH) plus shared INTERVAL_SEC and RETRY_TOLERANCE kindled in rbfc, replacing magic-number call-site arguments and the inline z_max_consecutive_failures local. Rename rbgp's pool-build pair to ZRBGP_POOL_BUILD_POLL_{CEILING,INTERVAL_SEC} to disambiguate from foundry budgets, and lift the posture-check curl --max-time to ZRBGP_POSTURE_REQUEST_MAX_TIME_SEC interpolated into the cloud-side script.

### 2026-05-19 11:17 - ₢BOAAP - W

Posture-check redesign landed and verified. zrbgp_pool_probe_submit refactored: extracted zrbgp_pool_build_submit_await primitive (shared with new zrbgp_pool_posture_submit), zrbgp_write_posture_check helper writes cloud-side bash with honest curl idiom (no -f flag, SUCCESS terminal = posture correct, only honest network failures trip the assertion). Folkloric HTTP-400-quota-row-materialization tolerance branch removed; any non-2xx is now fatal with dumped diagnostic. New rbgp_depot_info verb + rbw-di.DepotInfo.sh tabtarget gives cheap iteration loop against existing depot — verified live against canest (tether 65s SUCCESS, airgap 75s SUCCESS). RBSDE-depot_levy.adoc and Memos/memo-20260517-cloudbuild-default-quota-wedge/memo.md updated to retire both 'designed to FAILURE' and 'submission-400 materializes quota row' framings. Levy fixture trial (docket's nominal Done criterion) deferred at operator direction — code/spec/memo deliberation complete. Follow-on pace ₢BOAAQ slated for the broader timeout-naming audit raised during the design conversation.

### 2026-05-19 11:16 - ₢BOAAP - n

Repair posture assertion: rewrite curl idiom (no -f, success = posture correct), drop folkloric 400-quota-row tolerance, extract zrbgp_pool_build_submit_await primitive (shared between levy probe and new depot-info posture check), add zrbgp_pool_posture_submit and rbgp_depot_info verb with rbw-di tabtarget. Verified live against canest: tether 65s SUCCESS, airgap 75s SUCCESS. Spec and wedge memo updated to retire the designed-to-FAILURE and quota-row-materialization framings.

### 2026-05-19 11:16 - Heat - S

cloud-build-timeout-naming-audit

### 2026-05-19 18:12 - ₢BOAAO - W

Replaced rbob_charged_predicate at Tools/rbk/rbob_bottle.sh:508 with an explicit per-service check loop over sentry, pentacle, and bottle. Each service's compose ps invocation captures stdout (container IDs) to BURD_TEMP_DIR/zrbob_charged_<service>_ids.txt and stderr to BURD_TEMP_DIR/zrbob_charged_<service>_stderr.txt — the prior 2>/dev/null suppression is gone, eliminating a BCG violation and giving operators forensic evidence after a verify-active failure. Loop short-circuits on first missing service (return 1) or compose-command failure (return 1); returns 0 only when all three services report a running container. BCG predicate contract preserved: no buc_die, no stdout/stderr, status-only. Comment trimmed to describe the named-service semantics without restating the obvious. Moriah three-state trial confirmed correct behavior: pre-charge cic exit 1 (short-circuit at sentry, clean files); after rbw-cC.Charge.moriah charge with all three healthy, cic exit 0 with all three ids files populated and stderr files empty; after rbw-cQ.Quench.moriah, cic exit 1. Fast qualify passed before and after the change.

### 2026-05-19 18:12 - ₢BOAAO - n

Tighten rbob_charged_predicate to require sentry, pentacle, and bottle each running individually, capturing compose stderr to BURD_TEMP_DIR for failure inspection.

### 2026-05-19 10:56 - Heat - T

machine-type-restore-e2-standard-2

### 2026-05-19 17:22 - ₢BOAAF - W

LRO-await for workerPools.create at both call sites in rbgp_Payor.sh (POST+case -> rbgu_http_json_lro_ok), matching the project-create / repo-create template in the same module. 409-as-success idempotency arms dropped as unreachable in practice. RBSDE-depot_levy 'Create Private Worker Pool' step restructured to plural [tether, airgap] iteration matching the existing 'Submit Per-Pool Probe Builds' idiom: {rbbc_call} -> {rbbc_submit}, adds {rbbc_await} returned operation, NOTE on egress posture difference and LRO-await rationale. Fast qualify passes. Quota denial and other terminal pool-create errors now surface at the create step rather than as a misleading 'workerpool not found' downstream. Live depot_stand_up fixture verification deferred to a subsequent bench pace per the paddock's 96-core quota constraint on the canest depot (cannot re-levy at default 2-cpu without forfeiting quota); the second 'Done' bullet (RBSDE create and probe steps internally consistent on pool count) is met by the shipped spec changes.

### 2026-05-19 16:57 - Heat - r

moved BOAAP to first

### 2026-05-19 16:54 - Heat - S

probe-egress-assertion-repair

### 2026-05-19 16:50 - Heat - S

named-service-charge-predicate

### 2026-05-18 18:26 - Heat - n

Update RBSDE-depot_levy spec to match the Submit Per-Pool Probe Builds repair landed in 8fc1ed3a9. Added a {rbbc_await} step between {rbbc_require} and {rbbc_show} carrying the polling specification (5s interval, 480-poll ceiling = 40 min, terminal-state tolerance for all six Cloud Build terminal states). Updated the {rbbc_show} description to include terminal status in the displayed block. Replaced the obsolete 'Submission is non-blocking; build runs asynchronously' NOTE with a NOTE documenting the await-and-serialize pattern: explicitly calls out that the loop's two iterations serialize because await blocks before the next submit, that this serialization is load-bearing, and that the prior non-awaiting variant caused the late-clearing-probe-collision wedge — pointer to Memos/memo-20260517-cloudbuild-default-quota-wedge for the full diagnosis. Retained the in-build-assertion-failure NOTE as a separate item since it remains true. Other NOTES (Google-hosted builder images, HTTP 400 quota-row materialization, rbi_df namespace flat-sibling layout) unchanged.

### 2026-05-18 18:03 - Heat - n

Repair: make zrbgp_pool_probe_submit synchronous on probe terminal state. Previous fire-and-forget pattern (per e14654739, with the explicit 'submitted — runs async' log line) caused the late-clearing-probe-collision wedge documented in Memos/memo-20260517-cloudbuild-default-quota-wedge — levy returned before either probe ran, the gauntlet's subsequent inscribe/kludge/sentry-conjure dispatches stole the airgap probe's worker-pool slot for ~19 minutes, and when the airgap probe finally cleared, the next tether conjure (jupyter) wedged in QUEUED until queueTtl expired. Repair: after successful (200/201) submission, poll the build via builds.get every 5s until terminal state (480-poll ceiling = 40 minutes). Any terminal state acceptable — probes are designed to FAILURE for egress posture assertion; what matters is that the levy waits before returning so concurrent dispatchers do not collide. Two call sites in rbgp_depot_levy now serialize naturally because bash is sequential: tether probe runs to terminal (~1 min) before airgap probe is submitted, so airgap gets a clean slot rather than queuing behind contention. Expected levy wall-clock increase: ~1-3 minutes (vs 60+ minute wedge under the bug). Header comment updated to remove 'Fire-and-forget' wording and point at the wedge memo. Did not modify RBSDE-depot_levy.adoc spec yet — that updates once this repair is empirically verified against today's Google environment via tP gauntlet.

### 2026-05-18 18:00 - Heat - n

Memo addendum: Theory 1 confirmed and proximate cause pinned to fire-and-forget probe submission. tP gauntlet on main with e14654739 reverted (revert commit 936a7607b) completed clean — 'Suite gauntlet complete (12 fixtures), Pristine qualification passed'. Trial tally now 2/2 passes with probes removed across two distinct commits, 2/2 wedges with probes active across two independent depots. Inspection of e14654739's zrbgp_pool_probe_submit confirms the 'runs async' log line literally documents the fire-and-forget pattern. Mechanism walked end-to-end: probes submit non-awaited → levy returns → gauntlet's subsequent inscribe/kludge/sentry-conjure dispatches steal the airgap probe's slot for ~19 minutes → airgap probe finally runs → at probe-clear moment, next tether conjure (jupyter) wedges in QUEUED and expires at queueTtl. Repair specified: synchronize the levy on probe terminal state via build-status polling after builds.create. All terminal states (SUCCESS/FAILURE/CANCELLED/TIMEOUT/EXPIRED/INTERNAL_ERROR) tolerated — probes are designed to FAILURE for egress assertion. Parallel-submit-then-await-both is preferred over serial because external-dispatcher contention is the real wedge cause, not probe-vs-probe contention. Cost analysis: levy grows from ~2 min to ~15 min, but that cost is already being paid as a 60+ min wedge today — repair moves cost to the right place.

### 2026-05-17 21:01 - Heat - n

Memo addendum: anchor test at ad3ea5e22 passed full tP gauntlet on a fresh canest depot at default 2-vCPU quota — overturning the original 'environmental change' framing. Diagnosis is now code regression in the 12-Tools/rbk/ commits between ad3ea5e22 (last known good) and HEAD. Inventory narrows the dispatch-path candidates to two: e14654739 (per-pool probe submission, +146 lines in rbgp_Payor.sh — strong correlation with the late-clearing-probe-collision pattern documented in the main body) and df0fb2651 (LRO-await for workerPools.create — timing change). Four theories ranked: (1) e14654739 alone (high confidence); (2) df0fb2651 alone (medium); (3) combined e14654739+df0fb2651 (medium, distinguishable only by experiment); (4) something else in the 10 residual commits (low, listed for humility). Suggested next experiment: revert e14654739 on a branch off main, run tP, observe.

### 2026-05-17 18:34 - Heat - n

Memo capturing the freshly-levied default-quota Cloud Build wedge. Evidence preserved before depot teardown: May 16 EXPIRED build object, May 17 live-reproduced QUEUED build object, both depots' tether+airgap pool configs, and complete build timelines on both depot projects. Memo corrects the prior bench memo's misdiagnosis ('jupyter needs 3 concurrent multi-arch children' — wrong; one conjure = one Cloud Build job = one e2-standard-2 worker), substantiates the real mechanism (status=EXPIRED at queueTtl=3600s, 97ms exec gap when worker eventually attempts pickup), identifies the late-clearing-probe-collision pattern reproducible across two independently-levied depots, and ranks mitigation paths. Open question to test next: is the wedge environmental (Google quota/scheduler change) or code-side regression — to be probed by re-running the gauntlet from an earlier commit known to pass.

### 2026-05-17 11:17 - ₢BOAAN - W

Stood up cygwin@rocket and wsl@rocket as noninteractive substrate-routing personas under hack rules. cygwin@rocket forced-command runs Cygwin bash directly with $SSH_ORIGINAL_COMMAND; wsl@rocket forced-command runs a runcmd.cmd wrapper that sets WSLENV=SSH_ORIGINAL_COMMAND/u and invokes wsl.exe with -d Ubuntu-24.04 -u root. WSL distros are per-Windows-user; the wsl headless account bootstrapped its own Ubuntu-24.04 registration via wsl --import (no admin elevation needed), populated from a wsl --export of bhyslop's WSL state. Pattern and findings memorialized in Memos/memo-20260516-windows-headless-account-anatomy.md 'Noninteractive forced-command variants' section. brad@rocket interactive persona untouched; formal garrison upgrade remains future work if/when bujn-winpc completes. Verified: ssh cygwin@rocket 'uname -o' returns Cygwin; ssh wsl@rocket 'uname -o' returns GNU/Linux.

### 2026-05-17 11:16 - Heat - n

Augment headless-account-anatomy memo with noninteractive substrate-routing personas — adds Noninteractive forced-command variants section covering cygwin@rocket and wsl@rocket worked examples (forced-command patterns, runcmd.cmd wrapper for WSL with WSLENV propagation), Per-user WSL state and provisioning without elevation (wsl --import bootstrap pattern as the no-rights-modification alternative to wsl --install), and operational notes on ACL re-lock after [IO.File]::WriteAllText, base64+EncodedCommand transit through cmd.exe/PowerShell, and wsl.exe UTF-16LE output quirk. Adds cross-reference from substrate-landscape memo into the augmented section.

### 2026-05-17 10:40 - Heat - n

Capture empirical findings on Cygwin vs WSL substrate viability for theurge; document Cygwin substrate boundary mismatch with three boundary classes, WSL substrate result, operational traps in cmd.exe→PowerShell→wsl.exe transit, neutral framing — both substrates documented without recommending either, both slated paces (cygwin path transmute and noninteractive personas) noted as live.

### 2026-05-17 10:10 - Heat - S

cygwin-wsl-noninteractive-ssh-personas

### 2026-05-17 10:05 - Heat - S

cygwin-theurge-path-transmute

### 2026-05-17 09:49 - Heat - n

Fix buc_require prompt/preamble interleaving race under Cygwin. Symptom (observed on first run via ssh brad@rocket): in rbw-MZ.MarshalZeroes, the 'Proceed with marshal zero?' prompt and its 'Type zero to confirm:' line appeared in the middle of the still-being-emitted vessel-hallmark listing — operator saw the section header, then the prompt, then the remainder of the listing (vessel paths, regime-fields section, preserved block, on-completion line) flushed afterward. Cause: buh_line writes to stderr (>&2) while buc_require wrote its prompt to /dev/tty directly; on a Cygwin pty bridged over SSH the two write paths to the same terminal device aren't serialized, and stderr gets pty-buffered while /dev/tty hits the device immediately, so the prompt overtook still-buffered buh_line output despite the sleep 1 preceding it. Repair: switch the two prompt printf writes from >/dev/tty to >&2 so they share the buh_line stream and natural in-order emission is preserved on every platform. Input read still uses </dev/tty so the confirmation survives stdin redirection (heredocs, pipes). Bonus side effect: the prompt now appears in the tabtarget transcript log (../logs-buk/) since the framework's stderr-tee captures it; previously /dev/tty writes bypassed the tee and left no record of what the operator confirmed. Tradeoff considered: someone redirecting stderr to a file (2>file) outside the BUK framework would lose the prompt — moot because the TabTarget Invocation Discipline forbids such piping and the framework already handles transcript capture. Alternatives rejected: (1) forcing an stderr flush before the /dev/tty write — bash has no portable fflush and the exec 2>&2 re-open trick doesn't serialize against a different fd; (2) routing buh_line through /dev/tty too — much larger change, would bypass framework transcript capture for all handbook prose. Affects every buc_require caller (marshal-zero, depot levy/unmake, payor establish/install/refresh, and other confirmation sites).

### 2026-05-17 17:28 - Heat - n

Align MZ's RBRR_GCB_MACHINE_TYPE default with bench memo's locked decision: e2-highcpu-32 -> e2-standard-2. MZ was the last code-level holdout; spec already documents e2-standard-2 as default. Necessary for the gauntlet to recreate the default-quota wedge observed in temp-20260516-223924's jupyter conjure (build sat in QUEUED indefinitely with quota=2 vCPU); reproducing the issue requires a fresh canest depot levied at e2-standard-2 rather than e2-highcpu-32.

### 2026-05-17 09:49 - Heat - n

Fix buc_require prompt/preamble interleaving race under Cygwin. Symptom (observed on first run via ssh brad@rocket): in rbw-MZ.MarshalZeroes, the 'Proceed with marshal zero?' prompt and its 'Type zero to confirm:' line appeared in the middle of the still-being-emitted vessel-hallmark listing — operator saw the section header, then the prompt, then the remainder of the listing (vessel paths, regime-fields section, preserved block, on-completion line) flushed afterward. Cause: buh_line writes to stderr (>&2) while buc_require wrote its prompt to /dev/tty directly; on a Cygwin pty bridged over SSH the two write paths to the same terminal device aren't serialized, and stderr gets pty-buffered while /dev/tty hits the device immediately, so the prompt overtook still-buffered buh_line output despite the sleep 1 preceding it. Repair: switch the two prompt printf writes from >/dev/tty to >&2 so they share the buh_line stream and natural in-order emission is preserved on every platform. Input read still uses </dev/tty so the confirmation survives stdin redirection (heredocs, pipes). Bonus side effect: the prompt now appears in the tabtarget transcript log (../logs-buk/) since the framework's stderr-tee captures it; previously /dev/tty writes bypassed the tee and left no record of what the operator confirmed. Tradeoff considered: someone redirecting stderr to a file (2>file) outside the BUK framework would lose the prompt — moot because the TabTarget Invocation Discipline forbids such piping and the framework already handles transcript capture. Alternatives rejected: (1) forcing an stderr flush before the /dev/tty write — bash has no portable fflush and the exec 2>&2 re-open trick doesn't serialize against a different fd; (2) routing buh_line through /dev/tty too — much larger change, would bypass framework transcript capture for all handbook prose. Affects every buc_require caller (marshal-zero, depot levy/unmake, payor establish/install/refresh, and other confirmation sites).

### 2026-05-17 09:25 - Heat - n

Refresh stale SETUP NEEDED message in bul_launcher. The message only mentioned BURS_LOG_DIR but BURS_USER (added in f1a32439) and BURS_TINCTURE (added in b2ffca63) had become required since — fresh clones followed the instructions then hit a cryptic validation error on second run. Block now enumerates all three required variables with brief rationales, recommends BURS_TINCTURE=a as a starter value (note: change it once you understand the disjointness purpose), and uses BUK-generic wording with no RB-specific terms. Output routed through buh_handbook primitives (buh_section/buh_line/buh_tt/buh_e) instead of bare echo; yawps the missing absolute path, the BURD_REGIME_FILE path, the relative BURC_STATION_FILE path, the three recommended additions, and the two referenced tabtargets via buh_tt (resolves buw-rcr/buw-rsr colophons to actual tt/*.sh paths at print time with OSC-8 hyperlinks when supported). Bootstrap-mirrors BURD_TABTARGET_DIR from BURC_TABTARGET_DIR (buyy_tt_yawp prerequisite; bud_dispatch is the canonical exporter but it doesn't run if SETUP NEEDED fires). Side effect: SETUP NEEDED now goes to stderr since buh_* writes to >&2, matching buc_die semantics.

### 2026-05-17 09:07 - ₢BOAAB - W

Planning-only pace. Design conversation settled the depot-regime shape: four depot-time settings (RBRR_CLOUD_PREFIX, RBRR_DEPOT_MONIKER, RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE) peel into a new RBRD regime; tripwire mechanism is a FROM-scratch image in rbi_df carrying rbrd.env, with host-side docker inscribe at Payor levy and host-side docker pull/extract/exact-byte-diff before every cloud submission. rbrd_check is role-agnostic taking a bearer token; consuming CLI furnishes source the bespoke module, kindle, call inline. Alternatives considered and rejected: ORAS host binary (violates narrow-deps), per-variable GCS bucket files (new IAM, new read mechanism, new write mechanism), per-variable GAR fact files (overcomplicated vs single image), Cloud-Build inscribe (needlessly complex for a small file). Comparison is exact byte-match rather than value-level so comment/whitespace edits also participate — sharper diagnostic than per-variable compare. Implementation slated as ₢BOAAJ (rbrd-regime-split, regime mint + spec + handbook + variable rename) and ₢BOAAK (rbrd-tripwire-wire, bespoke module + Payor host-side inscribe + role-agnostic check + inline call wiring at cloud-submission sites) following this pace. Note: originally slated as ₢BOAAI/AAJ in pre-rebase view; post-rebase reconciliation landed them at AAJ/AAK after ₢BOAAI was taken by origin's post-levy-quota-bump-flow slate.

### 2026-05-17 09:07 - Heat - S

rbnRX-convention-document

### 2026-05-17 09:04 - Heat - S

rbrd-tripwire-wire

### 2026-05-17 09:04 - Heat - S

rbrd-regime-split

### 2026-05-17 15:58 - ₢BOAAA - W

Three-phase Cloud Build machineType A/B (e2-highcpu-32 vs e2-standard-2). Phase 1: 11 dispatches on canest2bhl100011 (highcpu-32). Phase 2: rbrr.env flipped to standard-2, same warm pool, 11 dispatches — wall-clocks within 5-15s of Phase 1, empirically reconfirming ₢BLAAA's pool-time finding (rbrr.env machineType is read only by levy, not per-dispatch). Phase 3: new depot canest2bhl100012 levied at standard-2 via gauntlet's canonical-establish + onboarding-sequence fixtures; 11 dispatches on the real standard-2 pool. Memo at Memos/memo-20260516-cloudbuild-machinetype-bench.md records the 33-dispatch matrix, per-workload speedup ledger (CPU-bound conjures ~2x slower on standard-2, multi-arch FROM-cached ~1.7x, registry-I/O ~1.5x, vouch tool ~1.8x), profile groupings, and bench-surfaced findings. Phase 3 surfaced that Google's fresh-project Cloud Build vCPU quota has dropped from the historical 10 vCPU baseline to 2 vCPU — past gauntlets succeeded only because rbrr.env always carried highcpu-32 at levy (which Google grants 96 vCPU). Required manual quota bump on 100012 via Cloud Console (6 vCPU granted, sufficient to clear RBRR_GCB_MIN_CONCURRENT_BUILDS=3 × 2-vCPU). Pace ₢BOAAI slated to design tighter quota-bump integration into levy completion. canest2bhl100011 restored as active depot via cleanup commit; 100012 left alive for future reference, operator unmakes when expressly chosen. May 14 cerebro 2-cpu logs demoted from primary reference to cross-host sanity check. Ancillary yields: fc179e39f (airgap empty-anchor preflight split), f2073e4f4 (gazette H1-delimiter wrong-vs-right example), 25a54c77d (absolute-paths-to-working-trees docket anti-pattern), d7145e0c0 (RCG retrofit of Tools/rbk/rbtd/), 00d99794b (theurge BURV sandbox anchored under BUK dispatch envs).

### 2026-05-17 04:17 - ₢BOAAA - n

Write the Cloud Build machineType A/B memo. Records the 33-dispatch matrix (11 workloads × 3 phases — highcpu-32 baseline, rbrr.env-flipped warm-pool re-run, real standard-2 pool via second depot), per-workload speedup ledger (CPU-bound conjures ~2x slower on standard-2, registry-I/O ~1.5x, vouch tool ~1.8x), reconfirms ₢BLAAA's pool-time machineType finding empirically, demotes May 14 cerebro logs to cross-host sanity check, captures the bench-surfaced Google fresh-project quota-default-dropped-to-2-vCPU finding and the gauntlet's latent standard-2 levy precondition (motivating ₢BOAAI), and lists ancillary yields

### 2026-05-17 04:14 - ₢BOAAA - n

Phase 3 cleanup: revert rbrr.env from canest2bhl100012/e2-standard-2 back to canest2bhl100011/e2-highcpu-32 and restore vessel rbrv.env files to point at 100011's reliquary (r260515151530) and airgap's RBRV_IMAGE_1_ANCHOR to 100011's forge hallmark (c260516175159-r260516175203). The new canest2bhl100012 depot remains alive in the cloud for future reference; operator unmakes when expressly chosen

### 2026-05-17 00:34 - Heat - S

post-levy-quota-bump-flow

### 2026-05-16 20:51 - ₢BOAAA - n

Machine-type flip between Phase 1 and Phase 2 of the A/B: RBRR_GCB_MACHINE_TYPE e2-highcpu-32 → e2-standard-2. This is the locked-decision implementing change — the bench measures what was given up to make it

### 2026-05-16 18:23 - ₢BOAAA - n

Setup step 4 of machineType A/B: pin airgap RBRV_IMAGE_1_ANCHOR to the just-ordained forge hallmark (c260516175159-r260516175203) so airgap's conjure FROM resolves to forge's GAR image; completes setup before Phase 1 measurements

### 2026-05-16 17:51 - ₢BOAAA - n

Setup step 1 of machineType A/B: enshrine pins rust:slim-bookworm and writes forge RBRV_IMAGE_1_ANCHOR=rbi_es/rust-slim-bookworm-b8ecdb97c5:rust-slim-bookworm-b8ecdb97c5 so forge's conjure base is pulled from GAR rather than docker.io

### 2026-05-16 10:21 - ₢BOAAA - n

Split the airgap empty-anchor preflight branch in zrbfd_registry_preflight (rbfd_FoundryDirectorBuild.sh) so it discriminates RBSAE's two anchor-population paths. When RBRV_IMAGE_n_ORIGIN names a producer vessel in this repo (test -d on RBRR_VESSEL_DIR/origin), the diagnostic now identifies the anchor as a hallmark-pin and steers the operator to either the canonical handbook track (rbw-Oda) or the minimal manual sequence (ordain producer, capture hallmark from BURD_OUTPUT_DIR/rbf_fact_hallmark, write rbi_hm/<HALLMARK>/image:<HALLMARK> into the anchor, re-ordain). When origin is a public upstream (no matching vessel dir), the existing enshrine guidance survives unchanged. Surfaced mid-mount when this pace's first airgap ordain followed the misleading enshrine suggestion to a Cloud Build skopeo failure ('reading manifest latest in docker.io/library/rbev-bottle-ifrit-forge').

### 2026-05-16 10:03 - Heat - T

seed-skip-orphan-boaah

### 2026-05-16 10:03 - Heat - S

seed-skip-orphan-boaah

### 2026-05-16 09:54 - Heat - n

Add 'absolute paths to working trees' bullet to docket anti-patterns — captures the lesson surfaced when BOAAA's pinning to /Users/.../rbm_alpha_recipemuster forced cross-repo state divergence during this mount; rule prefers relative working-dir references, allows absolute paths only for external data roots and asks they be flagged as operator-mutable rather than identity

### 2026-05-16 09:38 - ₢BOAAH - n

Retrofit Tools/rbk/rbtd/ for three RCG sections — Output Discipline, Constant Discipline, String Boundary Discipline — that surfaced as cleanup debt from the prior pace's BURD-anchoring work. New rbtdrg_log module provides four-severity macros (trace/info/error/fatal, _now variants) modeled on APCK's apcrl_log; classifier 'g' chosen because 'l' was taken by rbtdrl_calibrant. 42 naked eprintln!/println! sites across main.rs, rbtdrc_crucible.rs, and rbtdre_engine.rs swept to rbtdrg_*_now! macros — library modules use crate::rbtdrg_*, binary uses rbtd::rbtdrg_*, sentinel comment on every output-producing file. fatal_now! returns ! so unreachable `return ExitCode::FAILURE` lines after fatal sites in main.rs are dropped; behavior unchanged (process exits with code 1 either way). RBTDRI_BURD_TEMP_DIR_KEY and RBTDRI_BURD_OUTPUT_DIR_KEY consts placed in rbtdri_invocation.rs next to the existing BURE_CONFIRM_KEY precedent; both main.rs and rbtdth_helpers.rs now reference the single definition rather than bare string literals. rbtdri_invoke_dir_name(n) helper added next to the production format site; test file's 11 hand-expanded 'invoke-NNNNN' literal sites in rbtdti_invocation.rs fold through the helper. Mid-work RCG audit caught and fixed three self-introduced violations: format-string duplication in rbtdrg_log (extracted zrbtdrg_format helper), caller names in rbtdri_invoke_dir_name doc comment (would rot), and time-bound phrasing in rbtdrg_log module doc. Build clean under #![deny(warnings)]; 94/94 unit tests pass; fast suite 47/47 enrollment-validation + 26/27 regime-validation — the single regime failure (rbtdrf_rv_rbrn_all_nameplates, RBRN_SENTRY_HALLMARK must not be empty) exercises the bash-direct nameplate-validation path with no contact with Rust code in this pace, same shape as the prior mount's noted rbrv_all_vessels state-drift failure. rbtdrc_crucible.rs is 2857 lines (pre-existing RCG file-size debt, not grown by this pace).

### 2026-05-16 09:23 - ₢BOAAA - n

Add wrong-vs-right example block to gazette wire format warning, surfacing the H1-as-notice-delimiter failure mode with a concrete pattern. Hit during this pace's paddock setup — `# Paddock: rbk-11-mvp-tactical` inside the body parsed as a new notice slug and failed with `unknown slug 'Paddock:'`. The existing abstract caveat was clear but didn't prevent the mistake; the example makes the failure mode visible.

### 2026-05-16 09:19 - Heat - d

paddock curried: encode environment + reference data after credential refresh and baseline log discovery

### 2026-05-16 09:11 - ₢BOAAG - W

Theurge's BURV sandbox now anchors under BUK dispatch-provided BURD_TEMP_DIR/BURD_OUTPUT_DIR instead of /tmp/rbtd-{pid}/. Per-call HTTP captures from rbgu_http_json (and theurge's case trace dirs) land under ../temp-buk/ and ../output-buk/, surviving reboot and systemd-tmpfiles cleanup — matching standalone workbench forensic behavior. Theurge fails immediately if dispatch envs are absent; no fallback path. Unit tests share the discipline via rbtdth_scratch_root() which panics when BURD_TEMP_DIR is unset, so direct cargo test outside BUK is no longer supported (uniform with production). rbtdri_Context split into separate burv_temp_root and burv_output_root mirroring BUK's split — per-invoke layout collapses from invoke-N/{output,temp} to one invoke-N under each root. main.rs locals minted with rbtdb_ prefix and crate root now carries the full RCG-required attribute set (deny warnings, allow non_camel_case_types, allow private_interfaces). Validation: 94/94 unit tests pass; fast suite 47/47 enrollment-validation + 25/26 regime-validation pass — the one regime-validation failure (rbtdrf_rv_rbrv_all_vessels) is pre-existing canest state with empty RBRV_RELIQUARY across all vessels, exercises bash-direct path not the modified BURV layer, requires rbw-dY yoke to clear. Three RCG findings surfaced during review and slated as cleanup pace BOAAH: output module retrofit replacing naked eprintln!/println!, env-var-name string-literal const extraction, and a shared helper for the invoke-{:05} naming pattern shared by production and tests.

### 2026-05-16 09:11 - ₢BOAAF - W

Pool-create now LRO-awaited; spec drift fixed. rbgp_Payor.sh: both workerPools.create call sites converted from POST+case to rbgu_http_json_lro_ok, modeled on the existing project-create and repo-create template in the same module. 409-as-success idempotency arms stripped — unreachable in practice since project-create at the same call path has no 409 handling and dies first on any second-levy attempt. Quota denial (e.g., the e2-highcpu-32 case that motivated this pace) now surfaces at pool-create rather than as a misleading 'workerpool not found' at the downstream per-pool probe submission. RBSDE-depot_levy spec: 'Create Private Worker Pool' restructured to plural, iterates [tether, airgap] with per-variant body (matching the existing 'Submit Per-Pool Probe Builds' idiom), {rbbc_call} -> {rbbc_submit}, adds {rbbc_await} returned operation, drops 409 clause, NOTE covers egress posture and LRO-await rationale. Fast qualify passes; live verification of the 'workerpool not found' clear deferred to next pace's bench cycle which exercises depot_stand_up against the canest depot.

### 2026-05-16 09:11 - ₢BOAAG - n

Anchor theurge's BURV sandbox under BUK dispatch-provided BURD_TEMP_DIR/BURD_OUTPUT_DIR rather than /tmp/rbtd-{pid}/, so per-call HTTP captures from rbgu_http_json survive reboot and systemd-tmpfiles cleanup. Theurge dies immediately if dispatch envs are absent — no fallback. Tests share the discipline via a new rbtdth_scratch_root() helper that also panics when BURD_TEMP_DIR is unset; direct cargo test outside BUK is no longer a supported workflow. rbtdri_Context split into separate burv_temp_root and burv_output_root to mirror BUK's temp/output split. main.rs locals carry rbtdb_ prefix (rbtdb_Roots, rbtdb_read_dispatch_dir, rbtdb_allocate_roots, rbtdb_run_suite, rbtdb_run_single, rbtdb_list_fixtures); crate root now carries RCG-required #![deny(warnings)] and #![allow(private_interfaces)].

### 2026-05-16 09:08 - Heat - S

rbtd-rcg-output-and-const-debt

### 2026-05-16 08:49 - ₢BOAAF - n

Make workerPools.create await its LRO terminal state in rbgp_Payor.sh by replacing both POST+case blocks with rbgu_http_json_lro_ok, matched against the project-create / repo-create template in the same module. Strict template-match: 409-as-success idempotency arms dropped (unreachable in practice — project-create at the same code path has no 409 handling and dies first on any second-levy attempt). Quota denial and other terminal pool-create errors now surface at the create step rather than as a misleading 'workerpool not found' at the downstream per-pool probe submission. RBSDE-depot_levy spec restructured to match: 'Create Private Worker Pool' becomes plural, iterates [tether, airgap] with per-variant body (matching the existing 'Submit Per-Pool Probe Builds' idiom), {rbbc_call} -> {rbbc_submit}, adds {rbbc_await} returned operation, drops the 409-idempotency clause, NOTE explains egress posture difference and the LRO-await rationale. Fast qualify passes; live verification deferred to the next bench pace which exercises depot_stand_up.

### 2026-05-16 08:37 - Heat - n

restore canest operational state to main rbrr.env after a pristine-lifecycle fixture run committed throwaway prlcbhl-/pristlbhl100011 state — cancbhl-/canrbhl- prefixes and RBRR_DEPOT_MONIKER=canest2bhl100011 restored from commit 4fe8f500; RBRR_GCB_MACHINE_TYPE=e2-highcpu-32 already intact. Heat-housekeeping precedes the highcpu-build-substep-bench pace so its mount agent sees the canest depot reflected in main rather than starting under a throwaway-state config that would point any rbw-fO/rbw-fV at a torn-down depot.

### 2026-05-16 15:29 - Heat - S

theurge-buk-temp-discipline

### 2026-05-16 15:28 - Heat - S

levy-pool-lro-await

### 2026-05-16 08:24 - Heat - S

machine-type-restore-e2-standard-2

### 2026-05-16 08:24 - Heat - T

highcpu-build-substep-bench

### 2026-05-15 08:09 - ₢BOAAC - W

srjcl jupyter notebook reverified end-to-end: ANTHROPIC_API_KEY transports from the launch shell through a bare-name compose environment passthrough into the bottle, sentry egress reaches api.anthropic.com, and the name-genie cell returns word-list content. Scope expanded from the original docket's 'surface the local port during charge' design beat into a general rbnnh_ per-nameplate customization mechanism: rbnnh_compose.yml Compose overlay fragment and rbnnh_post_charge.sh host-side hook, both auto-detected in .rbk/MONIKER/, hook runs as last charge step with full RBRN/RBRR exported and nonzero exit propagating. Spec wired through RBSCH subdoc with RBS0/RBSCC/RBK-acronym linkage. srjcl is the first nameplate to use the mechanism — its hook emits the JupyterLab URL and prints a yellow warning if the launch shell lacks ANTHROPIC_API_KEY. Legacy srjcl baked-in workspace files restored from history under .rbk/srjcl/workspace/. Two notebook fixes landed during reverification: retired the deprecated claude-3-5-sonnet-20240620 model ID (replaced with claude-sonnet-4-6) and dropped the assistant-message prefill (Claude 4.x rejects conversations not ending on a user turn), folding the 'Your words are:' priming into the trailing user message. Charge cleanup messaging refined to set operator expectations on the multi-minute implicit-quench wait. Two spooks surfaced and worth itching later: notebook-output commit size pressure (tripped 30KB warning twice), and lack of automated regression coverage for the notebook→Anthropic round trip.

### 2026-05-15 08:09 - ₢BOAAC - n

Capture successful srjcl name-genie notebook run after prefill fix: cell outputs now show the Anthropic API returning word lists instead of the prior 400/404 errors. srjcl appears to be working again end-to-end — ANTHROPIC_API_KEY transports from the launch shell through the bare-name compose passthrough into the bottle, network egress through the sentry reaches api.anthropic.com, and the model returns valid content.

### 2026-05-15 08:07 - ₢BOAAC - n

Drop assistant-message prefill from srjcl name-genie helper: claude-sonnet-4-6 rejects conversations that end on an assistant turn with 400 invalid_request_error ('This model does not support assistant message prefill. The conversation must end with a user message.'). The 'Your words are:' priming text is folded into the trailing user message so the model still gets the start-with-the-list signal without violating the new API constraint. Notebook captures the 400 trace from the prior attempt that motivated this fix.

### 2026-05-15 08:05 - ₢BOAAC - n

Update retired Anthropic model ID in srjcl name-genie notebook helper: claude-3-5-sonnet-20240620 was deprecated and now returns 404 from the API, replaced with claude-sonnet-4-6 (latest Sonnet, matches original tier intent). Also dropped the stale commented-out claude-3-haiku-20240307 alternative line which carried the same retirement rot. Notebook updated with the successful cell outputs from re-verification, confirming the rbnnh_ per-nameplate customization mechanism is working end-to-end: ANTHROPIC_API_KEY reaches the bottle via the bare-name compose environment passthrough, network egress to api.anthropic.com succeeds through the sentry, and the cell returns word-list content rather than the prior NotFoundError.

### 2026-05-15 07:56 - ₢BOAAC - n

Refine charge cleanup message to name the actual cause of the wait: the implicit quench of a preexisting crucible is what can take a couple of minutes, not the cleanup of stray state in general.

### 2026-05-15 07:53 - ₢BOAAC - n

Annotate charge cleanup step to set operator expectations on duration: docker compose down with --remove-orphans on a previously charged crucible can stall on container stop signals for a couple of minutes, and the bare 'Cleaning up any prior state' line gave no hint the wait was expected rather than a hang.

### 2026-05-15 07:41 - ₢BOAAC - n

Document the rbnnh_ per-nameplate customization mechanism. New RBSCH-charge_hook.adoc subdoc defines the two concepts (rbnnh_compose_fragment, rbnnh_post_charge_hook) with their detection, environment, and exit-status contract. RBS0 mapping section gains a Nameplate Customization comment block with both new quoins; body gains a Nameplate Customization Internals section between Foundry Internals and Trade Studies that anchors and includes RBSCH. RBSCC charge operation gains a new step describing optional hook invocation, references the new quoins, and states that nonzero hook exit propagates through charge. RBK acronyms file gains RBNNH and RBSCH entries pointing at the new prefix family and subdoc respectively.

### 2026-05-15 07:41 - ₢BOAAC - n

Add optional rbnnh_ per-nameplate customization mechanism: rbnnh_compose.yml Compose overlay and rbnnh_post_charge.sh post-charge hook auto-detected in .rbk/MONIKER/. Hook runs as last charge step with full RBRN/RBRR exported; nonzero hook exit propagates through charge. Renamed compose fragments across srjcl/tadmor/ccyolo/moriah from bare compose.yml. srjcl gets the first hook emitting JupyterLab URL and yellow ANTHROPIC_API_KEY warning when absent; srjcl rbnnh_compose.yml adds an ANTHROPIC_API_KEY bare-name environment passthrough so shell-exported key reaches the bottle with no on-disk credential file. Also restored legacy srjcl baked-in workspace files under .rbk/srjcl/workspace/, surfaced into the bottle via the new compose bind mount.

### 2026-05-15 15:36 - ₢BOAAD - W

Established per-pool probe submission in depot_levy: each pool gets a Cloud Build that materializes its concurrent-build quota row (HTTP 400 tolerated as the expected fresh-levy side-effect), asserts intended egress posture in-build (tether: public+Google reachable; airgap: public blocked, Google reachable), and pushes a FROM-scratch marker to a new GAR sibling category rbi_df (depot facts) — a flat sea of files alongside rbi_hm/rq/es, ignored by image enumerators, reserved for future RBRD-variable inhabitants. Updated RBSDE spec with a new axhos_step for the probe-submit contract. Updated uBuN paddock to replace the 'Manifest lives in GCS' locked decision with 'Manifest substrate undecided' — surfaces rbi_df as a viable alternative at mount-time, since the original GCS rationale predated the namespace existing. End-to-end validation: quota approval to 96 cores landed mid-pace; drove the full prerequisite chain (reliquary inscribe in 3m11s, yoke-all-vessels to stamp r260515151530, busybox conjure in 1m33s plus vouch in 0m59s) confirming the approved quota carries production-shape conjures against cancbhl-d-canest2bhl100011 — hallmark c260515152058-r260515152101 minted.

### 2026-05-15 15:36 - ₢BOAAD - n

yoke reliquary stamp r260515151530 into all nine vessel rbrv.env files, populating the empty RBRV_RELIQUARY slots for the fresh depot levy shakedown

### 2026-05-15 15:35 - ₢BOAAD - n

uBuN paddock: replace 'Manifest lives in GCS' locked decision with 'Manifest substrate undecided' — surface rbi_df (added in this pace) as a viable alternative at mount-time, since the original GCS rationale predated the namespace existing

### 2026-05-15 15:10 - ₢BOAAD - n

add per-pool probe submission to depot_levy: each pool gets a Cloud Build that materializes its quota row (HTTP 400 tolerated as the expected fresh-levy side-effect), asserts intended egress posture in-build, and pushes a FROM-scratch marker to a new GAR sibling category rbi_df (depot facts) — flat sea of files alongside rbi_hm/rq/es, ignored by image enumerators, reserved for future RBRD-variable inhabitants

### 2026-05-15 14:31 - Heat - S

levy-establish-probes

### 2026-05-15 13:14 - ₢BOAAA - n

restore site-specific RBRR fields and increment moniker to canest2bhl100011 for fresh depot levy under the 32-cpu gauntlet shakedown

### 2026-05-15 13:12 - ₢BOAAA - n

marshal-zero defaults RBRR_GCB_MACHINE_TYPE to e2-highcpu-32 — preserves the 32-cpu config across regime resets so the gauntlet shakedown stays exercised

### 2026-05-14 15:11 - Heat - S

srjcl-jupyter-reverify

### 2026-05-14 15:11 - Heat - S

depot-regime-now

### 2026-05-14 15:11 - Heat - S

gauntlet-32cpu-shakedown

### 2026-05-14 15:11 - Heat - f

racing

### 2026-05-14 15:11 - Heat - N

rbk-11-mvp-tactical

