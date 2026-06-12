# Office-Federation Conversion — Decision Record

Date: 2026-06-12

TRIAGED 2026-06-12: owned by ₣BZ (its bedrock decision record) —
outside ₣BH triage scope; no disposition claimed here.

Status: **DECIDED** (operator commitment, 2026-06-12, after the 06-10→06-12
design conversation). Single-tier operator credential model: **workforce
federation with office-SA impersonation**. The keyfile **citizen tier is
canceled unbuilt**. This memo supersedes **D1** of
`memo-20260609-federation-canon.md` (two permanent tiers) and replaces the
canon's direct-resource-grant model with office impersonation. The ₣BZ paddock
was rewritten against this record the same day and is authoritative for heat
shape; this memo holds the decision and its derivation pointers.

---

## The decision

1. **One tier.** Operators authenticate via Workforce Identity Federation
   (org + domain + external IdP required — the prerequisite ladder is
   accepted). No keyfile tier ships, ever. The RBRD mode enum is not built.
2. **Office-SA impersonation, not direct grants.** Depot levy creates
   **office SAs** (governor / director / retriever — the capability-sets
   instantiated) and grants **all** resource bindings to them, once, behind a
   settle gate. A human is admitted to an office by one binding:
   `roles/iam.serviceAccountTokenCreator` on the office SA, granted to their
   `principal://` subject. Runtime: IdP device flow → STS → 
   `generateAccessToken` → short-lived office token. No per-user Google
   identity exists; no SA key exists anywhere.
3. **Bridge policy.** Today's keyfile machinery is untouched legacy riding
   under the accessor seam until federation personas pass the same suites;
   then the RBRA estate retires whole (cult verbs, key machinery, RBSRA) as
   its own movement. Scaffolding with a demolition condition, never a tier.

## Why (pointers, not restatement)

- **Flap topology.** Post-levy IAM mutation collapses to one binding type at
  one scope (the office SA's own policy), with one gate point. The SA-lifecycle
  flap family (ACCOUNT_STATE_INVALID, read-after-create/delete, DRS race,
  tombstones) is confined to levy, one-time, behind a read-back gate. The
  Class-C family survives only at admission ceremonies, never on hot paths.
  Taxonomy: `memo-20260604-credential-churn-leak-and-propagation-races`,
  `memo-20260611-heat-BH-class-c-setiampolicy-write-flap`, README
  "Eventual Consistency" appendix.
- **Vendor alignment.** Google's published preference ladder puts keys last
  with default org-policy enforcement against them since 2024-05; impersonation
  with short-lived credentials is the recommended composition. Evidence:
  `../rbm_beta_recipemuster/Memos/memo-20260611-google-impersonation-preference.md`.
- **Product-matrix dodge.** Workforce federated tokens carry a per-product
  support matrix; an impersonated office token is a plain SA token,
  universally accepted.
- **Blast radius.** A token is one office at a time; the multi-hat union-key
  problem dissolves. Tokens are uncapped and self-expiring; keys were capped
  (10/SA) and immortal.
- **Deletion ledger.** Canceled unbuilt: recut, verb-internal key delivery,
  assay-retirement mechanics, key-last enfranchise ordering, mode enum,
  two-tier test matrix, per-citizen SA lifecycle and its flap tolerances,
  migration machinery. Added: federation-setup verb, three thin REST legs,
  session cache, test IdP. The conversion removes more planned complexity
  than it introduces.

## Durable-secret census (the security claim, stated plainly)

Shipped system: **zero service account keys**; the sole durable secret is the
payor's own RBRO refresh token (founding human, kept cold). Conditional
residuals, both deliberate: the retriever reserve-key posture (documented
fallback, not built) and the test-rig synthetic-persona decision (open — the
one place a durable secret could creep back).

## Token custody and exfiltration posture

MVP: per-session filesystem scratch behind the accessor seam, 0600, ≤ session
cap. Upgrade path: OS keystores via container credential-helpers; the
anti-exfiltration layer is VPC-SC ingress (workforce-compatible), deferred
with named triggers. Device-bound CBA is currently unavailable to federated
principals. Evidence:
`../rbm_beta_recipemuster/Memos/memo-20260611-token-custody-context-enforcement.md`.

## Retriever differentiation (policy, not mechanism)

Retriever-office token lifetime at the 12 h ceiling (activates the canon's
deferred D5 capability-set-keyed lifetime); machine consumers pull via their
own workload identities, leaving the human chain entirely; reserve-key
posture documented, not built.

## Multi-IdP posture

One live provider per pool is the norm; adding a provider is a payor ceremony
with root-of-trust gravity (pool-scoped subject namespace — the pool's
security floor is its weakest provider). Dual-provider windows are for IdP
migration only, never an availability hedge.

## Supersession map

| Document | Disposition |
|---|---|
| `memo-20260609-federation-canon.md` | D1 retired (single tier); direct-grant model → office impersonation; D2/D3/D4/D6 survive; D5's capability-set refinement **activated** for retriever. Banner added. |
| ₣BZ paddock | Rewritten 2026-06-12 against this record (authoritative for heat shape). |
| `memo-20260605-citizen-capability-model.md` | Mechanism memo for the canceled tier; historical. Civic layer concepts (terrier, capability-sets, verb orderings) survive via the paddock. |
| Beta-repo evidence memos (2026-06-11, both) | Written to beta during skirmish lock; candidates to migrate here. |

## Open (resolved within ₣BZ)

- Vocabulary re-mint for the new model — the office noun, the impersonation
  act, login/session words — under MCM Word Selection (follow-on pace, mulled
  separately).
- Test-rig synthetic-persona credential decision (per-run click vs caged
  test-org secret).
- Test IdP election: free Entra tenant (real foreign IdP, device flow) and/or
  Keycloak-in-a-crucible (hermetic; JWKS-upload makes private issuers viable
  for programmatic flow).
- Spike verification items: audit `delegationInfo` shape, office-token
  lifetime ceiling and the extension org-policy constraint, AR/Cloud Build
  federated-token status, STS egress-rule shape under VPC-SC.
