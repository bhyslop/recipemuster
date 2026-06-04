# Dogfight intermittent failure — build-status poll connect-timeout aborts a healthy ordain

**Date**: 2026-06-04
**Invocation**: `tt/rbw-ts.TestSuite.dogfight.sh` (run 1 of two; run 2 of the same path passed)
**Class**: Pale (vendor/network transient) — *not* a code regression
**Status**: characterized; repair recommended below, **not yet applied**

## Phenomenon (one line)

The dogfight fixture's `ordain` step (`rbw-fO` → `rbfd_ordain`) was killed not by
a build failure but by the *build-status poll* failing to **connect** to the
Cloud Build API ~3 times in a row, while the build itself was healthy and
progressing.

## Evidence (run 1 ordain log `../logs-buk/hist-rbw-fO-sh-20260604-114803-75985-294.txt`)

```
11:48:04 WARNING: Build quota insufficient: 4 vCPU quota / 2 vCPUs per build = 2 concurrent (need 3)   # non-fatal; build proceeded
11:48:07 #5 exporting config sha256:5345d5dd... done                                                    # build progressing
11:51:37 WARNING: Curl failed (rc=28; 1/3 consecutive) — rbfc_poll_stderr_36.txt
11:51:52 WARNING: Curl failed (rc=28; 2/3 consecutive) — rbfc_poll_stderr_37.txt
11:52:07 WARNING: Curl failed (rc=28; 3/3 consecutive) — rbfc_poll_stderr_38.txt
11:52:07 ERROR: [rbfd_ordain] Failed to get build status after 3 consecutive failures (last rc=28)
```

- `curl rc=28` = operation/connection timeout. The 15s cadence (37→52→07) =
  `RBCC_CURL_CONNECT_TIMEOUT_SEC` (10s) + `ZRBFC_BUILD_POLL_INTERVAL_SEC` (5s)
  per cycle — i.e. curl could not establish a connection to googleapis.com
  within 10s, three polls running.
- The build had been running ~3.5 min and was past `exporting config`; nothing
  indicates the build itself failed.
- **Run 2 ran the identical ordain path and passed** (`rbtdrd_build_run_lifecycle`,
  ~420s) — confirming a transient, station-side connectivity blip.

## Mechanism (`Tools/rbk/rbfcb_BuildHost.sh`, `zrbfc_wait_build_completion`)

The poll loop (lines ~60-121) GETs the build resource each
`ZRBFC_BUILD_POLL_INTERVAL_SEC`. On any curl failure it increments
`z_consecutive_failures`; a successful poll resets it to 0. When the counter
reaches `ZRBFC_BUILD_POLL_RETRY_TOLERANCE` it `buc_die`s. Same counter and
tolerance are shared across three distinct failure modes:

1. curl transport failure (rc≠0) — lines 80-87  ← **this incident (rc=28)**
2. empty response body — lines 89-95
3. HTTP `error.code` present in body — lines 97-104

Constants (`rbfc_FoundryCore.sh` / `rbcc_Constants.sh`):

| Constant | Value |
|---|---|
| `ZRBFC_BUILD_POLL_RETRY_TOLERANCE` | 3 |
| `ZRBFC_BUILD_POLL_INTERVAL_SEC` | 5 |
| `RBCC_CURL_CONNECT_TIMEOUT_SEC` | 10 |
| `RBCC_CURL_MAX_TIME_SEC` | 60 |

## Why it mis-fires

The tolerance of 3 is reasonable for modes 2 and 3 (an empty body or an
`error.code` suggests something genuinely wrong with the request/permissions —
fail fast). It is **mis-calibrated for mode 1**: a status-poll that cannot reach
the API tells us nothing about the build, which runs server-side independently.
A ~45s station-side network blip therefore abandons an otherwise-healthy
(and already paid-for, minutes-long) Cloud Build. Connectivity-to-the-poller is
not build-health.

## Recommended repair (not applied)

In order of preference; (1) alone likely suffices:

1. **Give the curl-transport failure path (mode 1) a larger, separate tolerance
   than the body-error paths.** Split `ZRBFC_BUILD_POLL_RETRY_TOLERANCE` into a
   small `*_BODY` tolerance (keep 3 for modes 2/3 — those want to fail fast) and
   a larger `*_TRANSPORT` tolerance for rc≠0 (e.g. 8–10 ≈ ~2 min of sustained
   unreachability at the 15s cadence). The reset-on-success behavior means the
   larger value costs nothing on the happy path and only matters during a real
   blip. This keeps the fail-fast intent for genuine API errors while making
   transport patience proportional to the value of an in-flight build.

2. **(Optional, narrower) Discriminate rc=28 specifically.** Treat timeout
   (rc=28) as transient with the larger budget, but keep hard transport failures
   (e.g. rc=6 DNS, rc=7 connect-refused) on the short tolerance — those are less
   likely to self-heal. More precise but more code; (1) already covers the
   observed case.

3. **(Defensive) On exhausting transport retries, do one final classification
   poll before dying** — if the build can be confirmed terminal-success on a
   later reachable poll, do not abort. Heaviest option; only worth it if the
   blip proves recurrent.

A bare tolerance bump (raise the single constant 3→~8) is the one-line stopgap,
but conflates the three modes — option (1) is the load-bearing fix: the three
failure classes have genuinely different "should we keep waiting?" answers.

## Suggested home

A future pace under ₣BY (rbk-08-credential-repairs) or the foundry heat, when an
operator decides the transient recurs often enough to be worth the split. Until
then this memo is the record; the failure is self-healing on rerun.
