## Boundary — shape, not mechanism (cinched)

This heat adopts the federation *shape* in keyfile backing — without building the
federation *mechanism*. The backing stays keyfile (service-account keys); a no-org
operator runs it unchanged. Explicitly OUT of scope: Workforce pool/provider setup,
STS token exchange, IdP device flow, and the RBRD mode-enum activation — those belong
to the federation tier's own future work (RBSHR "Operator federation"; memo-20260527).

The payoff is sharper than "the vocabulary stays the same." The decoupling below makes
keyfile's *capability layer structurally identical* to federation's — same gesture, same
GCP IAM mechanism — so a later federation switch reshapes nothing in that layer; it adds
one identity-backing and one token path. This is structural isomorphism, not nominal
vocabulary stability, and it is the real reason the switch is non-disruptive.

NOTE — supersedes the source memo on one point. memo-20260527 says the operator verbs
"keep their names, only their bodies differ," and that keyfile holds "one SA key file each
*per role*." This heat goes further: the verb *bodies* converge in shape (not just names),
and keyfile becomes one SA *per person*, not per role. memo-20260527 is to be updated to
record this divergence when the model is frozen.

## The target model — identity decoupled from capability

The spine: **an identity's lifecycle is independent of its capability set**, exactly as in
federation. This is native to the GCP primitive — a service account can exist with zero IAM
bindings, and bindings are added/removed per-binding. Today's `invest` *fuses*
identity-creation with capability-granting; the model unfuses them. (Tractability holds: the
token layer is already identity-based — a minted token asserts only "I am this SA" and Google
enforces that SA's bindings server-side — so what is being removed is a *selection
convention*, not enforcement.)

- **Identity** = one principal per person, named for the person (no `director-` prefix). In
  keyfile backing it is a service account holding one RBRA key; in federation a `principal://`
  subject. Multi-role is first-class: one identity holds several capability sets at once.
- **Capability** = a *named set* of IAM grants (the role's binding list, lifted out of the
  invest bodies into code). The same set applies identically to an SA member (keyfile) or a
  `principal://` member (federation) — that sameness is the convergence.
- **The verbs dissolve** into a generic register parameterized by (actor, capability-set,
  identity): **add-person / remove-person / grant / revoke / rekey** + **read-roster**.
  `invest` = add-person + grant. `mantle` = add-person + grant(governor-set), run by the payor.
  `divest` = revoke + remove-person. `roster` = read the declared roster. Mantle stops existing
  as a distinct concept — it is the generic add+grant with the governor set and the payor as
  actor.
- **Capability verbs are backing-blind; only identity verbs branch** — and they branch *three*
  ways, not two. grant/revoke never branch (both tiers are an IAM binding on a pre-existing
  identity). add-person/remove-person/rekey branch on identity-backing: SA-key (keyfile),
  IdP-principal (federation), or OAuth-human (payor). rekey is keyfile-only (federation owns
  rotation at the IdP). The capability layer sits above all three, identical.
- **Grant/revoke operate only on named capability-sets, never raw bindings** — the
  interface-contamination guard that keeps the declared roster set-valued and auditable; a
  raw-binding grant would collapse the audit back into lossy reverse-mapping.

## The declared roster — intent ground-truth, distinct from enforcement

Enforcement is server-side IAM and nothing else. But *intent* — who is meant to hold which
capability — must be stored, for two reasons the decoupled model forces:

- Without a name prefix, deriving the roster from IAM is lossy reverse-mapping (a director's
  set is several bindings smeared across project + Mason SA + repo + bucket; partial and
  ambiguous matches become representable).
- You cannot audit IAM against itself. "Intent = whatever IAM says" makes intended-vs-actual a
  tautology that always passes, discarding the very drift signal the audit exists to find.

So a **depot-resident declared roster** records intent (`alice := {director, retriever}`); IAM
records enforcement; capability-set *code* defines what each set contains. Federation *forces*
this roster anyway — a `principal://` subject carries no role marker at all, and the memo
already concedes "the depot remains the home of the roster" — so adopting it in keyfile is paid
once for both tiers and makes **routine roster identical and O(1) across tiers**: read the
declared roster; touch IAM only when auditing drift. The intended-roster is therefore part of
the model, not an optional fork.

## The audit — drift as signal, healing asymmetric across the Pale

The audit diffs declared intent against IAM. Drift is not binary; the state space:
*deficit-whole* (a declared set wholly absent), *deficit-fragment* (a set partly present — the
half-failed multi-scope grant), *surplus-complete* (a whole undeclared set present), and
*surplus-fragment* (bindings belonging to no whole set — tampering, or a definition that
shrank).

Healing is **asymmetric**, and the asymmetry is the Pale recognition test applied:

- **Deficit is ours** — our own grant failed to complete. Auto-converge up to intent (re-run
  the idempotent grant), logged, never silent.
- **Surplus crossed the Pale** — IAM was touched outside our verbs. Characterize, report, and
  let a human adjudicate (legitimize → declare it, or reject → revoke it). Never auto-revoke (a
  stale roster would deprovision a working operator) and never auto-backfill intent (that
  re-enters the audit-against-itself tautology). No verb auto-revokes; the destructive
  operation is always human-driven.

Consequences: because grant writes intent *first*, our tooling cannot produce a surplus, so a
surplus is a clean "someone bypassed the verbs" signal. A capability-set definition change
decomposes into deficit-on-additions (auto-heal) + surplus-on-removals (report) — no special
versioning machinery. The audit's domain is roster-identities × set-defined bindings;
non-roster principals are out of scope.

**Orphan hazard (keyfile only).** With no name prefix and (apparently) no SA labels, nothing on
an SA marks it RBK-managed, so a failed add-person can strand a live key for an untracked SA.
Repair: a faint sentinel in the SA `description`, used *only* for an orphan-sweep — the roster
owns capability, the description earns exactly that one job. Federation has no key to leak and a
stray `principal://` binding is visible in the policy, so this hazard is keyfile-specific.

## Authority topology — the one seam with policy content

"Who may grant which capability-set" is the last real policy seam. Intended: payor →
governor-set; governor → director/retriever-sets.

- **Authority boundaries are real only where they fall on resource-scope splits.** The
  payor/governor wall holds *because billing is a separate resource* that governor's
  `roles/owner` excludes. The governor/director wall has no such split — both live in project
  IAM and governor has owner over it — so "governor may grant only director/retriever" is
  honor-system.
- **IAM cannot scope grants by role** (verify): Conditions cannot read a setIamPolicy payload,
  and Deny matches permission/resource/principal, not payload — so a "hard IAM wall scoping the
  governor's grants" is largely infeasible.
- **The model's own cinch decides the fork.** "Honor-system gates a malicious operator
  bypasses" condemns the "keep governor broad, catch unions by audit" horn — that audit is
  detective-only against the very authority it watches.
- **Resolution is topological and tier-dependent.** Since a standing owner cannot be scoped,
  ask whether it should exist. The governor exists for *delegation* (a non-payor admin). In the
  keyfile tier (no-org, solo/tiny — payor and admin are the same human) a standing owner-holding
  SA buys nothing and is pure attack surface → pull grant authority up to the payor; drop or
  demote the standing governor. In the federation tier (org, multi-admin plausible, no
  exfiltrable key) a delegated governor is tolerable. The answer legitimately differs by tier,
  and the model allows it since actor and capability-set are already parameters. (Decoupling also
  unlocks an *ephemeral* governor — grant the governor-set for one op, then revoke — if a keyfile
  delegated governor is ever wanted.)

**The seam between audit and topology:** the security value of surplus-detection depends on the
topology. A broad standing governor makes surplus-detection the *only* escalation tripwire (and,
per the cinch, inadequate against malice); grant authority at the payor closes the escalation
path *structurally* and demotes the audit to honest-mistake hygiene. Do not substitute detective
audit for structural closure.

## Blast radius — larger resting target, finer response

One SA per person carrying the union of granted capabilities widens the keyfile resting target
(a leaked key carries more than a per-role key did). Accept this explicitly. It is bounded: the
widening lands only in the free keyfile tier, where the operator already holds an on-disk secret;
federation has no key at all. And the decoupling *improves incident response* in a way the fused
model cannot — revoke a capability without touching the identity/key, or rekey without touching
capabilities, where the old model's only lever was delete-and-recreate.

## Inherited concern — governor teardown leak

`rbgp_governor_mantle` deletes outgoing `governor-*` SAs without first revoking their grants,
minting a `deleted:…?uid=` project tombstone per re-mantle (several ghosts and counting on the
canonical depot). The convergence closes it for free: the generic revoke-before-delete covers the
governor like any identity, and an idempotent governor (rekey, not delete+recreate) drops the
delete entirely. Must reuse the rbk-08 revoke layer + Class-C propagation tolerance, not reinvent
them. If this heat defers or descopes the governor, the standalone fix (lean project-scope
revoke-before-delete in the mantle delete loop) is captured in
memo-20260605-governor-mantle-tombstone-leak.md.

## What done looks like

Every cult-verb surface migrated to the converged identity/capability vocabulary: workstation
bash (the role-keyed selection collapses to one identity accessor), the governor/payor admin
verbs (invest/divest/roster/mantle become add/grant/revoke/remove + read parameterized by
capability-set), cloud-side bash wherever it names roles, the .adoc specs that voice the cult
verbs, the Rust theurge cases that drive them, and the onboarding handbook tracks that teach them.

The load-bearing federation-enabling core within that surface is narrow and shippable with no
behavior change: route the role/file-path-keyed token-accessor calls through one identity-keyed
accessor (memo-20260527 "single code seam"). The vocabulary convergence is the larger,
motivated-but-not-required remainder. Scoping/heat-chopping is deferred — recorded here as shape
only.

## Cinched decisions

- Enforcement is server-side IAM. No file or client-side check is ever the boundary.
- Identity lifecycle is decoupled from capability lifecycle (the spine). Multi-role is
  first-class: one identity, several capability-sets.
- Keyfile is one SA per person (union key), not one per role — blast-radius tradeoff accepted
  (bounded; finer incident response in return).
- Capability-sets are named code (the IAM-role lists lifted out of the invest bodies); grant and
  revoke operate only on named sets, never raw bindings.
- The declared roster (intent) is part of the model; IAM is enforcement; the audit diffs them.
- Audit healing is asymmetric: auto-converge deficits (ours), report surplus (Pale); no verb
  auto-revokes.
- Governor breadth is resolved per tier, not globally: keyfile pulls grant authority to the payor
  / minimizes the standing governor; federation may keep a delegated governor.
- The mode enum (keyfile vs federation) belongs in RBRD and is added only when federation lands —
  not in this heat.
- Migration grandfathers existing role-named SAs: read grants from IAM, register them in the
  declared roster, let legacy `director-…`/`retriever-…` names age out. No flag day.
- The convergence covers *operator* identities; cloud-native identities (Mason, later Envoy)
  share the capability-set abstraction but not the person-shaped identity lifecycle.

## Retired cinches

- "Roster derives from IAM (an `rbk-identity` label), not a stored list — it cannot drift."
  Retired: under decoupling, deriving from IAM is lossy and makes the intended-vs-actual audit a
  tautology, and SA labels appear not to exist (verify). Replaced by the declared roster +
  asymmetric audit above. The *true* invariant it reached for — no enforcement state outside IAM —
  is preserved; only the false corollary (no intent state may exist) is dropped.

## Open — resolve within the heat

- **Where the declared roster physically lives** (GCS object in the depot bucket, GAR artifact,
  …) and its writer/etag discipline. It is mutable, high-churn depot state — unlike
  tripwire-protected RBRD.
- **Whether keyfile drops the standing governor entirely or demotes it** to non-privileged.
- **Verify the two Pale facts before building on them**: SA labels absent; IAM Conditions/Deny
  cannot scope a grant by the roles in its payload.
- **Operator add-person UX**: add-person yields a powerless credential; a separate grant makes it
  useful — confirm the two-gesture flow is the intended operator experience.

## Sources

The per-person design exploration in rbk-08-credential-repairs that birthed this heat;
memo-20260604-credential-churn-leak-and-propagation-races (the H1/H2 lifecycle split — the
enabler); memo-20260527-operator-credential-models (the two-tier plan and cult-verb shape — to be
updated per the divergence noted above); memo-20260605-governor-mantle-tombstone-leak (standalone
governor-leak fix); RBSHR "Operator federation". This revision also folds in the four-turn design
conversation of 260605 (identity/capability decoupling, the declared roster, asymmetric audit,
authority topology).