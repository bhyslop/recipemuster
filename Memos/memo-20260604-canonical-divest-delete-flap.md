# Dogfight failure — IAM read-after-delete flap defeats `rbuh_poll_until_gone`

**Date**: 2026-06-04
**Invocation**: `tt/rbw-ts.TestSuite.dogfight.sh`
**Suite**: dogfight (`canonical-invest` → `dogfight` fixtures)
**Fix commit**: `2202ece57` (this finding's repair; failing run predates it)

## Phenomenon (one line)

A service account's GET read path flaps **404→200 across replicas after a
DELETE**, so a single observed 404 is not durable proof of deletion. This is
the inverse of the create-side flap already recorded in
`memo-20260514-pristine-gauntlet-iam-flap.md` (200→200→404 after create).

## Test outcome

| Fixture | Case | Result |
|---|---|---|
| canonical-invest | `rbtdrk_governor_mantle` | passed |
| canonical-invest | `rbtdrk_retriever_invest` | **failed** — `invest retriever exit 1` |

The suite halted at the second case; `rbtdrk_director_invest` and the dogfight
fixture never ran. The canonical-invest fixture is designed to be idempotent:
`rbtdrk_role_invest_impl` runs an explicit **divest before invest** (commit
`e0d8fb281`), 404-tolerant, so a standing-depot rerun clears the prior SA. That
divest **ran and succeeded** — the failure was downstream, in the invest's
existence preflight.

## Forensic artifacts (theurge trace, reaped-at-risk)

Trace dir: `../temp-buk/temp-20260604-112141-44242-735/rbtd/rbtdrk_retriever_invest/`

- Depot project: `cancbhm-d-canest3bhm100001`
- SA: `retriever-canest-ret@cancbhm-d-canest3bhm100001.iam.gserviceaccount.com`

**Divest** (`rbw-arD canest-ret`, 11:21:51) — transcript:

```
HTTP DELETE .../serviceAccounts/retriever-canest-ret@... returned code 200
Confirm deletion propagated before any same-name recreate
HTTP GET .../retriever-canest-ret@... returned code 200   # still present, waiting 3s
HTTP GET .../retriever-canest-ret@... returned code 200   # still present, waiting 3s
HTTP GET .../retriever-canest-ret@... returned code 404   # "gone after 6 seconds"  → returns success
```

**Invest** (`rbw-arI canest-ret`, 11:21:59, ~2s after the divest's lone 404) — transcript:

```
Preflight: confirm retriever-canest-ret does not already exist
HTTP GET .../retriever-canest-ret@... returned code 200   # flapped back to present
buc_die: Service account ... already exists — run the matching GovernorDivests verb first to re-key
```

## Root cause

`rbuh_poll_until_gone` (`rbuh_Http.sh`) declared the SA gone on the **first**
404. GCP IAM's SA read path is multi-replica eventually-consistent: a
post-DELETE GET flaps 200↔404 for seconds as replicas converge. The poll
hit a transient 404 from one replica and returned; the invest's preflight GET
~2s later hit a replica still showing the SA (200) and tripped the fail-loud
existence preflight (`rbgg_Governor.sh`, `zrbgg_create_service_account_with_key`).

Note the task's initial hypothesis ("SA left over from a prior run, never
divested") was **contradicted by the trace** — the divest ran and confirmed-gone
in this very invocation. The defect is the read-flap, not stale state.

## Fix

Honors `e0d8fb281`'s cinch (idempotency lives in the fixture-level divest;
invest stays fail-loud) — the repair is in the divest/poll membrane, preflight
untouched:

- `rbuh_poll_until_gone` now requires `RBGC_GONE_CONFIRM_STREAK` (=3)
  **consecutive** 404 observations before declaring durably gone; any
  intervening non-404 resets the streak. Total wait still bounded by
  `RBGC_MAX_CONSISTENCY_SEC`.
- New constant `RBGC_GONE_CONFIRM_STREAK` in `rbgc_Constants.sh` +
  `rbuh` kindle-sentinel validation.
- Spec drift repaired: RBSRD / RBSDD "Confirm Deletion Propagated" step now
  states the consecutive-404 requirement; RBSCIP gains the delete-side flap as
  a documented propagation behavior (this memo is its evidence row).

## Verification

`tt/rbw-ts.TestSuite.dogfight.sh` green twice consecutively (proves both the
immediate resolution and idempotency from a dirty post-success state).
