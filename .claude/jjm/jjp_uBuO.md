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