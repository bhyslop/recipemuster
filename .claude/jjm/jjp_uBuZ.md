## Boundary — shape, not mechanism (cinched)

This heat builds the **citizen tier** — the current keyfile, no-org operator-credential tier —
restructured so its capability layer is identical to the future federation tier's. The backing
stays keyfile; a no-org operator runs it unchanged. We do **not** touch federation here.
Explicitly OUT of scope: Workforce pool/provider setup, STS exchange, IdP device flow, and the
RBRD mode-enum activation — federation's own future work (RBSHR "Operator federation";
memo-20260527).

The payoff is structural, not nominal: the **capability layer** (capability-sets, grant/revoke,
the declared roster, the audit) becomes tier-blind, so a later federation switch reshapes nothing
in it — it swaps the identity-kind (citizen → federate) and the token mint (signed-JWT → STS) and
nothing else. That is what makes the switch non-disruptive.

Mechanism for everything below lives in `Memos/memo-20260605-citizen-capability-model.md`; this
paddock holds shape only.

NOTE — supersedes memo-20260527 on one point: it says operator verbs "keep their names, only
bodies differ" and keyfile is "one SA key file each per role." This heat converges the verb
*bodies* in shape and makes the citizen tier one SA *per person*. Update that memo when the model
is frozen.

## The citizen model — identity decoupled from capability

- **Citizen** = a keyfile-tier operator identity (governor/director/retriever), a depot-issued SA
  holding one RBRA key. Rank-neutral: rank is not a property of the citizen, it lives in the
  capability-sets the citizen holds. Multi-role is first-class — one citizen holds several
  capability-sets. (The federation tier's identity is a *federate*; the payor is the founder,
  outside both.)
- **Identity lifecycle is decoupled from capability lifecycle** — the spine. Adding/removing a
  citizen is independent of granting/revoking its capabilities; this is native to the GCP
  primitive (an SA may exist with zero bindings). Today's `invest` fuses the two; the model
  unfuses them.
- **Capability-set** = a named bundle of IAM grants, defined in code (lifted out of the invest
  bodies). Tier-blind.
- **The verbs dissolve** to add-citizen / remove-citizen / grant / revoke / rekey + read-roster,
  parameterized by (actor, capability-set, citizen). invest/divest/mantle/roster stop existing as
  distinct concepts.
- **Capability verbs are tier-blind; only identity verbs branch by tier** (and by the payor's
  OAuth backing). rekey is keyfile-only.
- **Grant/revoke operate only on named capability-sets, never raw bindings** — the contamination
  guard that keeps the roster auditable.
- Ontology: a depot has **citizens** (and later federates) — people who hold capabilities — and
  **holdings** (arks/hoards/bullions) — property.

## Intent vs enforcement — the declared roster (cinched)

Enforcement is server-side IAM, always. Intent — which citizen is meant to hold which capability —
is stored in a depot-resident **declared roster**, distinct from IAM. The audit diffs declared
intent against IAM; routine roster reads intent and is identical across tiers. Federation forces
the same roster anyway, so it is paid once for both. (Why it must be stored, and where it lives:
memo.)

## The audit — drift as signal, asymmetric healing (cinched)

The audit measures drift between declared intent and IAM. Healing is asymmetric, and the asymmetry
is the Pale recognition test:

- **Deficit** (IAM short of intent — our own grant failed) is ours → auto-converge, logged.
- **Surplus** (IAM beyond intent — touched outside our verbs) crossed the Pale → report; a human
  adjudicates. No verb auto-revokes; the destructive operation is always human-driven.

(Drift taxonomy, the intent-first bypass signal, definition-change versioning, and the orphan
marker: memo.)

## Authority topology — the one seam with policy content

"Who may grant which capability-set." Shape:

- Authority boundaries are IAM-real only where they fall on resource-scope splits (payor/governor
  holds because billing is a separate resource; governor/director does not).
- The governor's grant power **is** scopable by role at project scope (IAM `modifiedGrantsByRole`)
  but **not** at resource scope (repo/bucket/SA). So the resolution composes: IAM-limit the
  project-scoped grants, detective-audit the resource-scoped residual, and pull authority to the
  payor where a standing governor buys nothing.
- **Tier-dependent**: keyfile/solo → topology (no/minimal standing governor); federation/multi-admin
  → a condition-limited governor. The model allows the answer to differ (actor and capability-set
  are parameters).
- Surplus-detection's security value depends on the topology — do not substitute detective audit for
  structural closure. (modifiedGrantsByRole limits + the seam reasoning: memo.)

## Blast radius — larger resting target, finer response (cinched)

One citizen = one SA = one key carrying the union of held capabilities — forced by "one identity per
person," accepted as a tradeoff. Bounded to the citizen tier (federation has no key); most citizens
wear one hat; and decoupling buys finer incident response (revoke a capability without touching the
key; rekey without touching capabilities). (Reasoning: memo.)

## Inherited concern — governor teardown leak

`rbgp_governor_mantle` deletes outgoing `governor-*` SAs without revoking first, minting a
`deleted:…?uid=` tombstone per re-mantle. The convergence closes it for free (generic
revoke-before-delete + idempotent rekey-not-delete), reusing the rbk-08 revoke layer + Class-C
tolerance. Standalone fix if the governor is descoped:
memo-20260605-governor-mantle-tombstone-leak.md.

## What done looks like

Every cult-verb surface migrated to the citizen/capability vocabulary: workstation bash (role-keyed
selection collapses to one identity accessor), the governor/payor admin verbs (invest/divest/roster/
mantle → add/grant/revoke/remove + read, parameterized by capability-set), cloud-side bash wherever
it names roles, the .adoc specs that voice the cult verbs, the Rust theurge cases that drive them,
and the onboarding handbook tracks that teach them. "citizen" replaces the role-named identity
throughout.

The load-bearing federation-enabling core within that surface is narrow and shippable with no
behavior change: route the role/file-path-keyed token-accessor calls through one identity-keyed
accessor (memo-20260527 "single code seam"). The vocabulary convergence is the larger,
motivated-but-not-required remainder. Scoping/heat-chopping is deferred — recorded here as shape.

## Cinched decisions

- Enforcement is server-side IAM. No file or client-side check is ever the boundary.
- The identity is the **citizen** — rank-neutral, keyfile-backed (the federation tier's peer is a
  *federate*; the payor is the founder, outside both). Identity lifecycle is decoupled from
  capability lifecycle. Multi-role is first-class.
- The citizen tier is one SA (one key, union of capabilities) per person, not one per role —
  blast-radius tradeoff accepted.
- Capability-sets are named code; grant/revoke operate only on named sets, never raw bindings.
- The declared roster (intent) is part of the model; IAM is enforcement; the audit diffs them.
- Audit healing is asymmetric: auto-converge deficits, report surplus; no verb auto-revokes.
- Governor breadth is resolved per tier, not globally: keyfile pulls grant authority to the payor /
  minimizes the standing governor; federation may keep a condition-limited governor.
- The mode enum (keyfile vs federation) belongs in RBRD and is added only when federation lands —
  not in this heat.
- Migration grandfathers existing role-named SAs: read grants from IAM, register citizens in the
  declared roster, let legacy `director-…`/`retriever-…` names age out. No flag day.
- The convergence covers operator citizens; cloud-native identities (Mason, future Envoy) share the
  capability-set abstraction but not the citizen lifecycle.

## Retired cinches

- "Roster derives from IAM (an `rbk-identity` label), not a stored list — it cannot drift." Retired:
  under decoupling, deriving from IAM is lossy and makes the intended-vs-actual audit a tautology,
  and SA labels do not exist (SAs support tags, in Preview). Replaced by the declared roster +
  asymmetric audit. The true invariant it reached for — no enforcement state outside IAM — is
  preserved; the false corollary (no intent state may exist) is dropped.

## Open — resolve within the heat

- Where the declared roster physically lives (GCS object in the depot bucket vs GAR artifact) and
  its writer/etag discipline.
- Whether the keyfile tier drops the standing governor entirely or demotes it to non-privileged.
- The citizen orphan marker: SA tags (Preview) vs description sentinel.
- Operator add-citizen UX: add yields a powerless credential, a separate grant makes it useful —
  confirm the two-gesture flow is intended.

## Sources

The per-person design exploration in rbk-08-credential-repairs that birthed this heat;
memo-20260604-credential-churn-leak-and-propagation-races (the lifecycle split — the enabler);
memo-20260527-operator-credential-models (the two-tier plan and cult-verb shape — to be updated per
the divergence above); memo-20260605-governor-mantle-tombstone-leak (standalone governor-leak fix);
memo-20260605-citizen-capability-model (the mechanism this paddock points to); RBSHR "Operator
federation". This revision folds in the 260605 design conversation.