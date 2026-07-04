# ACCOUNT_STATE_INVALID — Re-invested Director SA, Cloud Build Poll + Vouch Token Mint

**Date:** 2026-05-27
**Observed in:** dogfight suite (`tt/rbw-ts.TestSuite.dogfight.sh` = `canonical-invest` + `dogfight`), run during ₣BM ₢BMAAK dogfight verification. Reproduced on **two independent hosts**.
**Disposition:** Transient — **2 failures / 3 runs**. Root cause is GCP service-account account-state eventual-consistency after `divest + re-invest`. Repair slated in ₣BB. This memo is the durable record (trace dirs are ephemeral `temp-buk/`).

## Summary

`canonical-invest` divests and re-invests the **director** SA, then `dogfight`'s `ordain` (rbw-fO, conjure-mode busybox) immediately leans on that same SA across a multi-minute cloud build *and* a post-build vouch. GCP's view of a freshly re-created SA's **account state** is still settling, so an auth call that lands in the unsettled window is rejected with **HTTP 401 / `UNAUTHENTICATED` / `reason: ACCOUNT_STATE_INVALID`** — even though the OAuth token itself is well-formed and the SA's IAM grants are present. Which call fails depends only on timing; the build itself is healthy.

This is a **new signature** distinct from the two prior IAM-propagation memos, and it lands at touchpoints their repairs never covered.

## Trigger conditions

1. `canonical-invest` re-mantles governor, divest/re-invests retriever + director (standing-depot credential refresh).
2. `dogfight` `ordain` mints a director OAuth token, submits a conjure Cloud Build, **polls build status** to completion (`rbfc_FoundryCore.sh`), then **vouches** the result (`rbfv_FoundryVerify.sh`, which mints a director token via `rbgo_OAuth.sh`).
3. The director SA was re-created seconds-to-minutes earlier; its account state flaps valid→invalid→valid across the build window.

`canonical-invest`'s director gate only probes Artifact Registry `packages.list` (HTTP 200) — that does **not** guarantee cloudbuild `GetBuild` or OAuth-token-mint stability for the same SA.

## Failure signature — two manifestations, one root

### Host A — cerebro (CONFIRMED: build-status poll)

Run `hist-rbw-ts-dogfight-20260527-005646-234127-457` (wall 6m6s, exit 1).
- Conjure build `83f5af85-9361-48f0-b7ee-dd9f362f5567`: `QUEUED` (polls 1–8) → `WORKING` (polls 9–27) — healthy.
- Polls 28, 29, 30 → HTTP 401. `rbfc_FoundryCore.sh:317` warns `1/3 .. 3/3 consecutive`; `:319` `buc_die "HTTP errors after 3 consecutive failures"`.
- `ordain` exits 1; the build was still progressing.

Error body (`rbfc_poll_response_28.json`, identical for 29/30):
```json
{ "error": {
    "code": 401, "status": "UNAUTHENTICATED",
    "message": "Request had invalid authentication credentials...",
    "details": [{ "reason": "ACCOUNT_STATE_INVALID", "domain": "googleapis.com",
      "metadata": { "service": "cloudbuild.googleapis.com",
        "method": "google.devtools.cloudbuild.v1.CloudBuild.GetBuild",
        "email": "director-canest-dir@cancbhm-d-canest3bhm100001.iam.gserviceaccount.com" } }] } }
```

### Host B — macOS (CORROBORATING: post-build vouch token mint)

Run `hist-rbw-ts-dogfight-20260526-180105-43629-216` (exit 1).
- Conjure build reached `SUCCESS` (poll 37, wall 2m11s) — the build *succeeded*.
- A single 401 `ACCOUNT_STATE_INVALID` appeared at poll 36 and was **absorbed** (`1/3 consecutive`), so the poll path recovered.
- Failure then surfaced in the vouch phase: `rbfv_FoundryVerify.sh:623` `buc_die "[rbfd_ordain] Failed to get Director OAuth token"`.
- **Caveat:** the exact OAuth error body for this vouch-phase mint failure was not captured (no `error`-bearing `oauth_response.json` in the trace). Same SA, same window, strongly suspected same `ACCOUNT_STATE_INVALID` family — **confirm during repair**.

### Run 3 — cerebro (PASSED)

Run `hist-rbw-ts-dogfight-20260527-010526-242392-626` (wall 9m39s) — clean. The SA state had settled by build time. Confirms transience.

## Root cause

A freshly **divest+re-invested** director SA is, for a window of minutes, in an `ACCOUNT_STATE_INVALID` condition at GCP's auth front door for *some* services even after its IAM grants and OAuth-token mint succeed elsewhere. Any director-authenticated call landing in that window is rejected 401. The build poll and the vouch token-mint are the two long-lived director-auth touchpoints in `ordain` and are therefore the exposed surfaces.

## Why prior tolerance work does not cover this

| Prior | Signature it absorbs | Path | Why it misses this |
|---|---|---|---|
| `memo-20260513` → ₢BBABF | `403 getIamPolicy` (caller recently empowered) | `rbgi_IAM.sh` grant sites | Grant-time, IAM-policy scope, 403 — not 401 on a runtime call |
| `memo-20260514` → ₢BBAAn/₢BBAAp | SA read-after-create `404`; OAuth `invalid_grant`+"Invalid JWT Signature" | SA-create probe; `rbgo_OAuth.sh` mint | ₢BBAAp's consumer retry matches **only** `invalid_grant`+"Invalid JWT Signature"; it **explicitly deferred** "other retry-worthy OAuth error patterns... capture as itch." `ACCOUNT_STATE_INVALID` is exactly that deferred itch. |

`rbfc_FoundryCore.sh:314–321` treats *any* `.error.code` in a poll response as a hard failure, fatal after `RETRY_TOLERANCE` (3) consecutive, with the poll token minted once up front (`:261`). It has no transient discrimination at all.

## Recommended repair (two touchpoints, one root)

1. **`rbfc_FoundryCore.sh` build-status poll (`:314–321`):** discriminate a transient `401`/`UNAUTHENTICATED` + `ACCOUNT_STATE_INVALID` from a hard error; absorb it with backoff against a bounded budget (mirroring the access-probe's time-bound model and `RBGC_PROPAGATION_*` from ₢BBABF) rather than counting it toward the 3-consecutive fatal. A still-`WORKING` build must survive a transient auth flap.
2. **`rbgo_OAuth.sh` token mint:** extend the consumer-side retry error-class (added by ₢BBAAp) to also absorb `ACCOUNT_STATE_INVALID`. This realizes ₢BBAAp's explicitly-deferred itch and covers the `rbfv_FoundryVerify.sh:623` vouch path (Host B).

Both bounded by a propagation budget consistent with the existing `RBGC_PROPAGATION_*` / access-probe profiles — the bound is the discriminator (real settling completes within budget; real denial waits the budget and fails clean), the same model ₢BBABF locked for the 403 class.

Repair-time investigation items:
- Confirm Host B's vouch failure body is `ACCOUNT_STATE_INVALID` (Host B caveat above).
- Decide whether `canonical-invest`'s director gate should additionally probe `GetBuild` / token-mint stability (not just AR `packages.list`) so the flap is bounded *before* `ordain` rather than tolerated *within* it.

## Cross-references

- `Memos/memo-20260513-iam-propagation-race-director-invest-gar.md` — 403 grant-scope race (sibling).
- `Memos/memo-20260514-pristine-gauntlet-iam-flap.md` — SA read-after-create flap (sibling).
- ₢BBAAp `oauth-token-mint-retry-on-signature-race` — the consumer-retry mechanism this extends; deferred "other OAuth error patterns" itch realized here.
- ₢BBABF `iam-grant-propagation-tolerance-share` — the `RBGC_PROPAGATION_*` time-bound model to reuse.
- ₢BMAAG `idempotent-canonical-invests` — re-invest timing / SA deletion propagation latency (same re-invest origin).
