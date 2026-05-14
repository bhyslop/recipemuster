# Pristine gauntlet failure — IAM read-after-create flap on `rbtdrp_sa_cycle`

**Date**: 2026-05-14
**Run start**: 2026-05-14 17:00:17 local
**Invocation**: `tt/rbw-tP.QualifyPristine.sh` (gauntlet test suite)

## Branch state at run time

- Branch: `cerebro-BB-002`
- HEAD: `d545c7661d1cf7a03bc5ac4c17efd23bcc09e655` — *pristine-lifecycle fixture: set RBRR_DEPOT_MONIKER=pristlbhl100009*
- Merge-base against `main`: `5838194f4aed46f1d385866a33dd8cc53337394d` — *jjb:1011-ae38563d::i: OFFICIUM 260513-1040*
- Commits ahead of `main` (3):
  - `d545c7661` pristine-lifecycle fixture: set RBRR_DEPOT_MONIKER=pristlbhl100009
  - `db1ff74f2` pristine-lifecycle fixture: install throwaway RBRR prefixes (prlcbhl-/prlrbhl-)
  - `cd93504d7` Marshal Zero — release qualification reset
- Working tree at run time: clean.

## Test outcome

`tt/rbw-tP.QualifyPristine.sh` → `rbtd-s.TestSuite.gauntlet` → fixtures run:

| Fixture | Cases | Result |
|---|---|---|
| enrollment-validation | 47 | 47 passed |
| pristine-lifecycle | 3 | 2 passed, 1 **failed** |

Failed case: **`rbtdrp_sa_cycle`** (the third pristine-lifecycle case). Marshal-zero attestation and depot stand-up both passed before it.

## Forensic artifacts

- Top-of-tree logs (host filesystem):
  - `../logs-buk/hist-rbw-tP-sh-20260514-170017-1358543-227.txt` (QualifyPristine)
  - `../logs-buk/hist-rbtd-s-gauntlet-20260514-170017-1358579-122.txt` (gauntlet driver)
  - `../logs-buk/hist-rbw-arI-sh-20260514-170249-1364416-563.txt` (the failing `GovernorInvestsRetriever` invocation)
- Theurge trace dir: `/tmp/rbtd-1359516/`
  - `rbtdrp_sa_cycle/trace.txt` — `FAILED: rbtdrp_sa_cycle / invest retriever exit 1`
  - `rbtdrp_sa_cycle/moniker.txt` — `pristlbhl100009`
  - `rbtdrp_sa_cycle/governor-sa-email.txt` — `governor-202605141702@prlcbhl-d-pristlbhl100009.iam.gserviceaccount.com`
  - `rbtdrp_sa_cycle/invest-retriever-{stdout,stderr}.txt` (stderr empty)
  - `burv/invoke-00004/` — the BURD invocation that wrapped the failing tabtarget
    - `output/current/burx.env`: `BURX_EXIT_STATUS=1`, began 17:02:49, ended 17:02:52 (3 s wall)
    - `temp/temp-20260514-170249-1364416-563/transcript.txt` — full call trace

## Precise failure point (from transcript)

Identity in play:
- Depot project: `prlcbhl-d-pristlbhl100009`
- Retriever SA being invested: `retriever-pristl-ret@prlcbhl-d-pristlbhl100009.iam.gserviceaccount.com` (uid `104464931844201861787`)

Sequence of HTTP calls inside `rbgg_invest_retriever` → `zrbgg_create_service_account_with_key`:

| # | Call | Endpoint | Result |
|---|---|---|---|
| 1 | Create SA | `POST iam.googleapis.com/v1/projects/.../serviceAccounts` | **200** (uid returned) |
| 2 | SA propagation probe (by email) | `GET .../serviceAccounts/{email}` | **200** — `"SA propagation (by email) ready after 0 seconds"` |
| 3 | Keys subresource propagation probe | `GET .../serviceAccounts/{email}/keys` | **200** — `"Keys subresource propagation ready after 0 seconds"` |
| 4 | Real List keys | `GET .../serviceAccounts/{email}/keys` | **404** — `Service account ... does not exist.` → `buc_die` |

So the read-after-create view at Google's IAM front door went **200 → 200 → 404** within microseconds. The existing `rbgu_poll_until_ok` helper accepted the first 200 as "ready," but IAM's visibility was still flapping non-monotonically. That makes this *not* a "missing propagation wait" — it's a "propagation wait accepts a single transient 200" bug.

stderr is empty; the failure surfaces only via stdout because the path through `rbgu_http_require_ok` → `buc_die` writes its error line to the standard log channel.

## Locale where a fix could go

Two candidate sites, narrow → broad:

1. **Narrow (symptom-local)** — `Tools/rbk/rbgg_Governor.sh:198-208`, the Preflight block immediately after the `Keys subresource propagation` probe. Wrap the `List keys` call (`rbgu_http_json` + `rbgu_http_require_ok`) in a small retry/back-off so a single transient 404 here doesn't abort the whole investment. Fixes this specific failure but doesn't help anywhere else the same pattern bites.

2. **Structural (root-cause)** — `Tools/rbk/rbgu_Utility.sh:570-596`, the `rbgu_poll_until_ok` helper. It returns ready on the *first* 200 (`if test "${z_code}" = "200"; then ... return 0`). Make readiness require **N consecutive 200s** (double-tap, or even triple-tap) against IAM's flapping window — same helper, same callers, but now resistant to the observed `200 → 200 → 404` pattern. This is the higher-leverage fix; `rbgu_poll_until_ok` is the propagation primitive used wherever the codebase says "wait until this Google resource is visible," so hardening it benefits every caller.

Existing related concept doc: `RBSCIP-IamPropagation.adoc` (governs how the codebase treats IAM propagation). Worth re-reading before picking option 2, since the helper change is a contract change for every caller.

## Reproducibility note

This run was against a freshly-minted pristine depot (`prlcbhl-d-pristlbhl100009`, governor minted at `202605141702`). Pristine runs create a brand-new GCP project per cycle, which is exactly the regime that exposes wide IAM propagation windows. A naive re-run is *likely* to pass; the bug is in the helper's tolerance of IAM's view, not in this branch's logic.
