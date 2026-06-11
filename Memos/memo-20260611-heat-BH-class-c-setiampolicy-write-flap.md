# Class-C flap on the resource-scope setIamPolicy WRITE — the lean-write reasoning falsified

Date: 2026-06-11 (~03:49, skirmish ladder attempt 2)
Status: finding with full trace evidence, no verdict — for the ₣BH terminal memo triage.
Sibling evidence row for `memo-20260604-credential-churn-leak-and-propagation-races`
(its 2026-06-05 addendum documents the same fixture edge on the READ surface).

## Phenomenon (one line)

After `governor_mantle` re-mantles the governor, `director_divest`'s repo-scope
**getIamPolicy read tolerance converges (200) but the immediately following
setIamPolicy WRITE still draws 403** — read-visibility and write-permission
propagate on different clocks, and the write path carries no Class-C tolerance.

## Trace (temp-20260611-034822-15282-34, invoke-00001)

- Project-scope and SA-scope revokes: all 200, clean.
- Repo getIamPolicy: 403 at 0s, 3s, 9s, 21s — each logged
  "caller-empowerment propagating", per fix 1a66443e — then **200 at 41s**.
- Repo setIamPolicy (next call): **403**,
  "Permission 'artifactregistry.repositories.setIamPolicy' denied … (or it may not exist)"
  → `buc_die` in `rbgg_divest_director` → case fail, suite halt.

## Relation to the recorded fix — what is new

Fix 1a66443e (churn memo addendum) gave the resource-scope revoke READS the
Class-C (403, empty-glob) tolerance and stated: "the setIamPolicy write loops
stay lean." The implicit premise: the read's bounded wait absorbs the caller's
empowerment propagation, so by write time the caller is converged.
Tonight falsifies that premise: a read served 200 (one replica/check converged)
while the write check ~1s later was still stale. The addendum's own lesson —
Class C attaches to the *caller*, not to the direction of the policy edit —
already predicted writes were exposed; the tolerance simply was not extended.

Same-night context distinguishing this from the suite's other failures:
attempt 1 was Payor RAPT expiry (operator reauth, not code); attempts 3+ hit the
stale `rbf_fact_reliquary` test reader (fixed, commit 2274b2ac2). This finding
is attempt 2 only. The 22:30 canonical-invest smoke and attempt 3 both passed
this exact sequence — the flap is stochastic, observed once in three runs.

## Repair question for triage (not decided here)

Extend Class-C tolerance to the resource-scope setIamPolicy write when the
caller is recently empowered — with care: the write loop deliberately treats
409 as fatal (single-writer invariant) and a blanket 403-retry on writes could
mask real denials. Candidate shapes: (a) bounded 403 retry on the write
mirroring the read's backoff; (b) after a read recovers from Class-C 403s,
require a confirm streak (the RBGC_GONE_CONFIRM_STREAK precedent — consecutive
clean reads before proceeding to the write); (c) a mantle-side read-back gate
(sibling of the invest-side actAs gate, ₢BHAAk) so the wait confines to mantle
rather than spreading tolerance across divest surfaces.

Also relevant: skirmish leads with this fixture, so one stochastic flap costs a
whole suite run — the suite-head Payor probe banked at triage shares that shape.
