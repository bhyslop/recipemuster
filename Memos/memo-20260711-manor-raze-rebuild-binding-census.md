# Manor raze-and-rebuild: binding-survival census

Pace ₢BsAAE (rbk-10-rebaseline-all). The manor was razed and rebuilt on the real
installation through operator-facing verbs alone, watching what a pool-id change
kills and which verb re-grants each grant. Old pool `spike-office-test` (soft-deleted
2026-07-11 ~21:21Z, ~30-day purge window, org 247899326218); new pool `rb-manor-260711`
via the one cinched regime edit (`RBRW_WORKFORCE_POOL_ID`, commit af50c53a5).

## Before: the complete pool-referencing binding set

Probed via payor-credentialed `getIamPolicy` over the six policies RB touches
(depot project, payor project, terrier bucket, three mantle SAs). Exactly four
bindings referenced the pool, all for the single freehold subject
`9657166c-8a2d-4f5d-bcd1-ef481ee31f3e`, every one owned by an admission verb:

| Binding | Home | Owning verb |
|---------|------|-------------|
| `roles/iam.serviceAccountTokenCreator` → `principal://…/spike-office-test/subject/9657…` | governor mantle SA | gird |
| same | director mantle SA | brevet |
| same | retriever mantle SA | brevet |
| `roles/serviceusage.serviceUsageConsumer` → same principal | depot project | brevet (first admission; attaint sweeps) |

Zero strays: payor project and terrier bucket carried no pool reference.
(Method bound: the sweep covered those six policies, not an org-wide asset search.)

## What the raze killed, and how it presented

- **The standing sitting died at the credential layer, not the binding layer.**
  A sitting with ~11.5h nominal runway drew `HTTP 401 UNAUTHENTICATED` at Leg 3
  (don, `generateAccessToken`) — the STS token itself is invalid the moment its
  minting pool goes DELETED. The request never reached IAM binding evaluation.
- **Cache-alone espy still reported LIVE** (~11h29m runway) post-raze — correct
  for its charter (no network), and a census point: the cache cannot see a raze.
  Avow, by contrast, correctly detected no live sitting and opened fresh.
- **The four bindings were not deleted — they tombstoned.** GCP rewrote each
  member in place to `deleted:principal://…/spike-office-test/…` (inert forever:
  a deleted pool's id cannot be resurrected as the same principal).

## The rebuild: which verb re-granted what

| Verb | Grant work |
|------|-----------|
| instaurate (`rbw-mI`) | pool `rb-manor-260711` only — no principal grants; terrier bucket idempotent-confirmed (it rode through the raze untouched, payor-project grain) |
| affiance ×2 (`rbw-mA` entrada, `rbw-qjK` keycloak facility) | providers `spike-entra`, `keycloak` under the new pool — no principal grants |
| gird (`rbw-mG`) | governor tokenCreator + depot serviceUsageConsumer, new-pool principal; muniment already present (idempotent) |
| brevet ×2 (`rbw-pB` director, retriever) | each mantle SA's tokenCreator, new-pool principal; SUC idempotent-ensured; muniments already present |

Post-rebuild probe: every policy carries a matched pair — the old binding as a
`deleted:` tombstone beside the fresh `rb-manor-260711` binding its verb re-granted.

## The flag: tombstones are unowned residue

The docket asked for any pool-referencing binding no admission verb owns. Answer:
**the four `deleted:principal://` tombstones.** Brevet only adds; unseat and
attaint remove by live-member match; escheat (`rbw-mE`) is terrier-scoped, not
IAM-scoped. Nothing in the verb set will ever clean them. They are inert by
construction, but each raze-rebuild generation deposits four more on the same
policies. Disposition is an operator call (itch candidate; not slated here).

## Muniment layer: record and access are independent, as doctrine says

The terrier roll was byte-identical before and after (three muniments,
subject/mantle/provider — no pool id in the `rbgft_` fields), and every
re-admission found its muniment already present. The roll proves record;
`rbw-am` proves access — both were exercised separately, both green.

## Access recovery proof

Fresh avow (device flow) → sitting live against the rebuilt trust; all three
mantle dons green with AR reach (HTTP 200); attribution trail shows each use-hop
carrying the freehold subject via the **new** pool's principal form
(21:31:48Z governor, 21:32:47Z director, 21:33:09Z retriever).

## Incidental observations

- **IAM propagation, reconfirmed:** the director don failed 28s after its brevet
  with the admission-deficit message, then succeeded on retry ~1 min later; the
  governor (girded ~2.5 min before its don) and retriever (~2 min) donned clean
  first try. Matches RBSCIP and memo-20260711-admission-don-propagation-incident.
- **Facility rot found en route:** fdkyclk's nameplate had both hallmarks vacant,
  blocking the keycloak facility charge; repaired by kludging both vessels
  (commits 78c2f9846, e66da1d62).
- **Clean-tree gates bit twice, correctly:** affiance refused the uncommitted
  pool-id edit (the seated provider must answer to committed config), and the
  bottle kludge refused the sentry kludge's uncommitted hallmark drive.
