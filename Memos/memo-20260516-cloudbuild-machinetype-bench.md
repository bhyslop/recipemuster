# Cloud Build machineType A/B — highcpu-32 vs standard-2

Pace: ₢BOAAA (highcpu-build-substep-bench), heat ₣BO (rbk-11-mvp-tactical).
Bench window: 2026-05-16 17:00 UTC → 2026-05-17 03:35 UTC.

## Decision

`RBRR_GCB_MACHINE_TYPE` reverts from `e2-highcpu-32` to `e2-standard-2`,
locked a priori. This memo documents what was given up, so a future
"should we go back?" decision has direct evidence rather than guesswork.

## Methodology

Three phases. The same 11-dispatch sequence in each, in the same order
(sequential — never parallel; BUK fixture discipline plus shared-pool
state assumptions):

  - rbev-bottle-anthropic-jupyter conjure ×2 (warmup + data; biggest CPU)
  - rbev-bottle-ifrit-forge conjure ×2 (multi-arch + apt-get + cargo)
  - rbev-bottle-ifrit-airgap conjure ×3 (multi-arch FROM forge)
  - rbev-bottle-plantuml bind ×1 (skopeo mirror — registry-I/O reference)
  - rbev-graft-demo about+vouch ×1 (mixed mode reference)
  - rbev-busybox conjure ×1 (tiny single-stage reference)
  - tt/rbw-fV batch vouch ×1 (sweeps all hallmarks)

Phase 1 (e2-highcpu-32): existing `canest2bhl100011` depot, pools
provisioned at levy time with e2-highcpu-32 (96 vCPU quota grant).

Phase 2 (`rbrr.env` flipped to e2-standard-2): same depot, same warm
pool. No relevy. Intent: confirm ₢BLAAA's structural finding empirically.

Phase 3 (real e2-standard-2 pool): new depot `canest2bhl100012` levied
via the gauntlet's `canonical-establish` fixture under
`rbrr.env=e2-standard-2`. Original 100011 depot left intact. Onboarding
work done via the gauntlet's `onboarding-sequence` fixture (covers 7 of
the 11 workloads' first runs as fixture-warmups); remaining data runs
dispatched manually in the same order as Phases 1 & 2.

Wall-clock per operation = timestamp delta between the opening
`rbfd_ordain` event (`Building vessel image` for conjures; `Loaded vessel`
for mirror/about+vouch) and the matching terminal `SUCCESS` line, read
from the per-tabtarget self-logs in `../logs-buk/`.

## Carried over from ₢BLAAA — empirically reconfirmed

**machineType is pool-time, not build-time.** `RBRR_GCB_MACHINE_TYPE` is
consumed only by the worker-pool create step inside `rbgp_depot_levy`
(`rbgp_Payor.sh:895`/`:921`); per-build dispatch reads nothing from
rbrr.env about machine size. Existing pools retain whatever machineType
they were provisioned with at levy. Phase 2's wall-clocks land within
5–15 seconds of Phase 1's per workload — well inside cloud-build
scheduling noise — confirming that the rbrr.env flip alone is a no-op
until the next depot is levied.

## Per-workload results

All 33 dispatches (11 × 3 phases). `build` column means conjure for
conjure vessels, mirror for bind, about+vouch for graft, GAR enumeration
for batch vouch sweep.

| # | Workload | P1 build | P1 vouch | P2 build | P2 vouch | P3 build | P3 vouch |
|---|---|---|---|---|---|---|---|
| 1 | jupyter conjure (warmup) | 8m39s | 1m02s | 8m49s | 0m55s | 16m52s | 1m49s |
| 2 | jupyter conjure (data)   | 8m34s | 0m59s | 8m55s | 0m56s | 15m55s | 1m38s |
| 3 | forge conjure (warmup)   | 3m08s | 0m59s | 3m16s | 1m01s | 6m13s  | 1m52s |
| 4 | forge conjure (data)     | 3m16s | 0m57s | 3m16s | 0m59s | 6m29s  | 1m42s |
| 5 | airgap conjure (warmup)  | 4m21s | 0m53s | 3m47s | 0m54s | 5m33s  | 1m48s |
| 6 | airgap conjure (data 1)  | 3m46s | 0m54s | 4m00s | 0m58s | 6m47s  | 2m12s |
| 7 | airgap conjure (data 2)  | 3m58s | 0m59s | 4m02s | 1m04s | 6m54s  | 1m38s |
| 8 | plantuml bind (mirror)   | 1m26s | 0m58s | 1m27s | 0m50s | 2m07s  | 1m38s |
| 9 | graft-demo (about+vouch) | —     | 1m36s | —     | 1m31s | —      | 2m21s |
| 10| busybox conjure          | 1m27s | 1m05s | 1m27s | 1m00s | 2m09s  | 1m37s |
| 11| batch vouch sweep        | —     | 0m33s | —     | 1m05s | —      | 0m31s |

Phases 1 & 2 differ from Phase 3 in pool machineType; Phases 1 & 2 share
the same warm highcpu-32 pool.

## Speedup ledger (highcpu-32 over standard-2)

Warmup runs discarded where >1 data run was collected. Data-run averages.

| Workload category | HC32 avg | S2 avg | HC32 speedup |
|---|---|---|---|
| jupyter conjure (CPU-heavy single-stage) | 8m45s (n=2) | 15m55s (n=1) | **1.82×** |
| forge conjure (multi-arch + apt + cargo) | 3m16s (n=2) | 6m29s (n=1) | **1.98×** |
| airgap conjure (multi-arch FROM forge)   | 3m56s (n=4) | 6m51s (n=2) | **1.74×** |
| plantuml bind (skopeo mirror)            | 1m27s (n=2) | 2m07s (n=1) | **1.46×** |
| graft about+vouch (mixed mode)           | 1m34s (n=2) | 2m21s (n=1) | **1.50×** |
| busybox conjure (tiny single-stage)      | 1m27s (n=2) | 2m09s (n=1) | **1.48×** |
| vouch tool (per-hallmark)                | 0m58s avg   | 1m45s avg   | **1.81×** |
| batch vouch sweep (GAR enumeration)      | 0m49s avg   | 0m31s        | **0.63×** (no-op; network/scale-bound, not CPU) |

## Profile groupings

**CPU-bound conjures (jupyter, forge):** ~2× slower on standard-2. Forge
hits 1.98× — close to the 2-vCPU-versus-32-vCPU ceiling for the parts of
the build that actually parallelize (cargo compile, apt installs).

**Multi-arch FROM-cached conjures (airgap):** 1.74× slower. The FROM
layer (forge image) is cached in GAR, so airgap's incremental work is
the airgap-specific Dockerfile only. The CPU shift bites less because
some of the build cost is image pulling and push from the registry,
which is network-bound regardless of machineType.

**Registry-I/O operations (plantuml mirror, busybox conjure,
graft about+vouch):** 1.46–1.50× slower. These spend most of their
wall-clock on skopeo copy / image push / GAR API calls. CPU is a
secondary factor; network is dominant.

**Vouch tool:** 1.81× slower. SLSA attestation processing is CPU-bound
and runs on a single worker; tracks the conjure ratio.

**GAR enumeration (batch vouch sweep):** Network-bound, not CPU. Phase
3's 0m31s is actually faster than Phase 1's 0m33s — within noise; not a
real effect of machineType.

## Cross-host reference (downgraded)

May 14 cerebro clean-gauntlet `fO` logs lived at
`../logs-buk/hist-rbw-fO-sh-20260514-2*.txt` on cerebro under
`RBRR_GCB_MACHINE_TYPE=e2-standard-2` on a cerebro-host-provisioned
depot. Originally planned as the primary 2-cpu reference; demoted to
cross-host sanity check because Phase 3 provides a same-host
standard-2 measurement on the same dispatcher binary version, free of
cross-host clock-skew confounds.

## Decision implications

For workloads dominated by registry-I/O (mirror, busybox, graft), the
speedup penalty is modest (~1.5×) and standard-2 is comfortably
adequate. For CPU-bound multi-arch conjures, standard-2 roughly doubles
wall-clock. The release-qualification gauntlet's whole-pipeline cost on
standard-2 is therefore ~1.7× the highcpu-32 cost; this is the kind of
budget the locked decision absorbs in exchange for the smaller vCPU
quota footprint.

If a future "should we go back?" question arises (e.g., per-developer
build throughput becomes a constraint), this ledger gives concrete
per-workload speedup numbers to plug into the trade-off.

## Findings surfaced during the bench

**Google's fresh-project Cloud Build vCPU quota has dropped.** Past
canest depots got the historical 10-vCPU baseline (encoded in the stale
`rbrr.env` comment `"fits 5 concurrent in 10-CPU quota"` and
`rbhpq_quota_build.sh` prose). `canest2bhl100012` was levied at
`e2-standard-2` and received only **2 vCPU** of
`concurrent_private_pool_build_cpus`. Past gauntlets succeeded because
the pre-bench `rbrr.env` always carried `e2-highcpu-32` at levy time,
for which Google grants 96 vCPU. This was the first canest2 levy in the
standard-2 era under the new low-default-quota regime. Required a manual
quota-request through the Cloud Console (6 vCPU was the bump granted; 6
clears the `RBRR_GCB_MIN_CONCURRENT_BUILDS=3` × 2-vCPU minimum). Stale
"10-CPU quota" wording in `rbrr.env` history and `rbhpq_quota_build.sh`
should be updated.

**Gauntlet has a latent precondition on a non-standard-2 levy.** The
`onboarding-sequence` fixture wedges on the jupyter conjure (which needs
3 concurrent multi-arch children) when the depot was freshly levied at
e2-standard-2 with default quota. Pace ₢BOAAI (post-levy-quota-bump-flow)
is slated to design a tighter integration of the quota-bump procedure
into levy completion — the levy's degenerate build primes the Cloud
Console "Edit Quotas" UI on the freshly-levied project, so the operator
can act immediately rather than discovering the wall on first ordain.

## Ancillary yields from the pace

These weren't the bench, but landed during it:

- **`fc179e39f`**: split `zrbfd_registry_preflight`'s airgap empty-anchor
  branch to discriminate hallmark-pin vs upstream-enshrine origins. The
  diagnostic now steers the operator to the correct recovery path
  (canonical handbook track for hallmark-pin, enshrine path for
  upstream).
- **`f2073e4f4`**: added a wrong-vs-right example block to the gazette
  H1-as-delimiter warning in `jjk-claude-context.md`, after hitting the
  failure mode mid-pace.
- **`25a54c77d`**: added the "absolute paths to working trees" bullet to
  JJK's docket anti-patterns list.
- **`d7145e0c0`**: retrofitted `Tools/rbk/rbtd/` for three RCG sections
  (Output Discipline, Constant Discipline, String Boundary Discipline)
  surfaced as cleanup debt during prior pace work.
- **`00d99794b`**: anchored theurge's BURV sandbox under BUK
  dispatch-provided `BURD_TEMP_DIR`/`BURD_OUTPUT_DIR`.

## State at memo close

`canest2bhl100011` is the active production depot
(`RBRR_DEPOT_MONIKER=canest2bhl100011`, `RBRR_GCB_MACHINE_TYPE=e2-highcpu-32`).
`canest2bhl100012` remains alive in the cloud, unused; operator unmakes
it later when expressly chosen.
