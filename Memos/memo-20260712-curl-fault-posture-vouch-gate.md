# Incident: curl faults aborting echelon, and the undeclared fault posture of the vouch about-gate

Date: 2026-07-12
Context: heat ₣Bs (rbk-10-rebaseline-all), pace ₢BsAAe (baseline-drift-reproof), station: operator's macOS primary.
Tree: `e2d1a6cc3`.

## What happened

The baseline re-proof needed one clean `echelon` run. It took four attempts. Three
aborted; the fourth landed green (27 fixtures, 256 passed, 0 failed).

The three aborts had **two distinct causes**, and conflating them is the trap this
memo exists to prevent.

| Attempt | Died at | Cause |
|---|---|---|
| 1 | fixture 24 (`srjcl`) | Real defect: colophon-census roster over-declaration. Repaired at `e2d1a6cc3`. Not a curl matter. |
| 2 | fixture 14 (`hallmark-lifecycle`) | `rbfv_verify.sh` about-gate: `curl` exit 28 (timeout), no tolerance, `buc_die`. |
| 3 | fixture 16 (`reliquary-lifecycle`) | `rbfcb_host.sh` build-status poll: three consecutive curl failures exhausted its tolerance. |

## Root cause of the curl aborts: the station's link, not the tree

Attempt 3's transcript is unambiguous. The Banish Cloud Build **submitted
successfully** (HTTP 200), then the status poll died:

```
poll 2: rc=28  Failed to connect to cloudbuild.googleapis.com port 443 after 10004 ms: Timeout was reached
poll 3: rc=7   Failed to connect to cloudbuild.googleapis.com port 443 after 2 ms: Couldn't connect to server
poll 4: rc=7   Failed to connect to cloudbuild.googleapis.com port 443 after 2 ms: Couldn't connect to server
```

A connect failure in **2 ms** is not a slow neighbor — it is no route to the network.
The operator independently confirmed the station's network was troubled during this
window. A probe taken minutes later returned 5/5 connects at ~11 ms.

`rbfcb_host.sh` behaved **correctly**: it carries the transient membrane, tolerated
the failures, and died only after three *consecutive* ones. It lost a genuine outage.
There is no defect there.

Attempt 2's exit 28 most likely shares this cause. It cannot be proven — a 60 s
`RBCC_CURL_MAX_TIME_SEC` ceiling was exceeded on a GAR manifest HEAD, and a slow
neighbor and a dying link are indistinguishable from that evidence alone. **This
ambiguity is the central fact of the incident.**

## The finding: the about-gate's fault posture is undeclared

`rbfv_verify.sh:627` (and the sibling at `:171`) gates vouch on the about artifact:

```
curl --head -s ... || z_curl_status=$?
test "${z_curl_status}" -eq 0 \
  || buc_die "HEAD request failed for about artifact (curl exit ${z_curl_status}) ..."
```

It dies on **any** nonzero curl exit, including 28 — which the project's own
`rbgo_curl_status_is_transient_predicate` (`rbgo_oauth.sh:89`) classifies as transient
(`7|28|35|56`), and which every peer HTTP path retries.

What this is **not**: it is not a curl-containment violation. The containment scanner
(`rbtdrn_conformance.rs`, case `rbtdrn_curl_containment`) governs *exit-code band
hygiene* — curl 8.6.0+ mints exit codes 100/101 that collide with the precision band,
so a curl exit may never be handed to `buc_die` directly; the mandated shape is capture
into `z_curl_status` then classify. **rbfv complies with that**, which is why the scan
is green. Containment and tolerance are different concerns at different layers.

What this **is**: a call site whose fault posture nobody declared. It hand-rolls curl
rather than riding `rbuh`'s HTTP membrane (plausibly because `rbuh`/`rbgu_http_json` is
JSON-oriented and this is a HEAD carrying a manifest `Accept`), and in doing so it
silently inherits fail-fast without a word about whether that was chosen.

## Doctrine survey

Retry/tolerance doctrine in this codebase is real but **stratified**, and the strata are
easy to mistake for one another:

| Layer | Home | Rule |
|---|---|---|
| curl exit codes | `rbgo_curl_status_is_transient_predicate`; `RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS=3` | The *surveyed transient curl band*: `7\|28\|35\|56`. |
| Host HTTP | `rbuh` / `rbgu_http_json` | Retries the surveyed band. |
| Build-status poll | `rbfcb_host.sh` | Tolerates failures; dies on 3 *consecutive*. |
| HTTP status | RBSCIG | 429/5xx transient retry inside IAM SET loops. |
| Eventual consistency | RBSCIP | Bounded blind waits — **explicitly reserved to post-grant IAM sites**. |
| Federation path | RBS0 `rbtf_don` | **Single-attempt fail-fast.** |
| Exit-code hygiene | `rbtdrn_conformance` curl-containment scan | Capture-then-classify; never `curl \|\| buc_die`. |

The governing posture is stated at RBS0 `rbtf_don`, and it is the **opposite** of what a
reflex reading assumes:

> Fault posture: the don and the acquisition legs behind it are single-attempt fail-fast;
> only the device-flow poll loops, and only on the surveyed transient curl band and the
> RFC 8628 pending states... No account-state flap tolerance rides the federation path,
> **by decision rather than omission**... tolerance returns only on **observed** evidence.

Three things follow. **Fail-fast is the declared default**, not an oversight to be
corrected. **Tolerance is granted only against a surveyed signature**, never as general
robustness. And the project's idiom is to *say so* when non-tolerance is deliberate — the
phrase "by decision rather than omission" exists precisely to mark that.

RBSAV (`RBSAV-ark_vouch.adoc:59-63`) specifies the gate itself — "Gate on About
Existence... HEAD manifests... HTTP 200 response" — but is **silent on its fault
posture**. So the code's fail-fast is, on the present record, an omission rather than a
decision. A future reader cannot tell which, and the next person to hit a timeout will
"helpfully" add a retry.

## Recommendation

**Do not add retry tolerance to the about-gate on this incident's evidence.** Three
reasons, in order of weight:

1. **The evidence is of the wrong thing.** Today's proven fault is a *local link
   outage*. The neighbor did not misbehave; the station's network died. Doctrine grants
   tolerance only on observed evidence of the *foreign* service faulting, and the
   Palisade rule is to absorb only the surveyed signature and fail fast on everything
   else. Papering a security-relevant vouch gate over the operator's wifi is precisely
   the bend the discipline forbids.
2. **The one arguably-GAR-side datum is unresolvable.** Attempt 2's exit 28 cannot be
   separated from the same link trouble. An unresolved observation is not a survey.
3. **A bespoke retry in `rbfv` would be the wrong shape anyway.** Tolerance for the
   surveyed band already has a home (`rbuh`). A second, hand-rolled implementation at
   one call site is how three divergent retry policies became four.

**Do instead — declare the posture, do not change the behavior.** Home the about-gate's
fault posture in RBSAV beside the gate it already specifies (§ *Gate on About
Existence*), stating that the gate is single-attempt fail-fast and why, and cite it at
`rbfv_verify.sh:627` / `:171` with an `RBr_` rivet. This converts an omission into a
decision, costs no behavior change, and inoculates the site against a future
well-meaning retry. If the posture is instead judged *wrong* on review, the same spec
edit is where that ruling lands.

**The trigger that would change the answer.** If the about-gate faults on the surveyed
band again **on a demonstrably healthy link**, that is a survey, and it earns a membrane
— in `rbuh` (extended to carry HEAD), not in `rbfv`. This memo is the first data point;
it exists so the second one is recognizable as a pattern rather than re-diagnosed from
scratch.

## Residue

The aborted banish submitted its Cloud Build (HTTP 200) before its poll died, so the
delete likely completed server-side — but a stale `rbi_ld/r260712110829` Lode package
may remain in GAR. A `rbw-ld` (divine) sweep would confirm.

## Durable facts owed a spec home

Per the memo discipline: nothing here is authority. The one fact that must survive this
memo's retirement is **the about-gate's fault posture**, and its home is RBSAV — not
this file.
