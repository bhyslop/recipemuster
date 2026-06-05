## Boundary — shape, not mechanism (cinched)

This heat builds the **citizen tier** — the current keyfile, no-org operator-credential tier —
restructured so its capability layer is identical to the future federation tier's. The backing
stays keyfile; a no-org operator runs it unchanged. We do **not** touch federation here.
OUT of scope: Workforce pool/provider setup, STS exchange, IdP device flow, RBRD mode-enum
activation — federation's own future work (RBSHR "Operator federation"; memo-20260527).

The payoff is structural, not nominal: the **capability layer** (capability-sets, grant/revoke,
the declared ledger, the audit) becomes tier-blind, so a later federation switch reshapes nothing
in it — it swaps identity-kind (citizen → federate) and token mint (signed-JWT → STS) and nothing
else.

Mechanism for everything below lives in `Memos/memo-20260605-citizen-capability-model.md`; this
paddock holds shape only.

NOTE — supersedes memo-20260527: it says operator verbs "keep their names, only bodies differ" and
keyfile is "one SA key file each per role." This heat converges the verb *bodies* in shape, makes
the citizen tier one SA *per person*, and generalizes 20260527's role-keyed token seam to an
identity-keyed one. (A deferral-honoring breadcrumb is on memo-20260527 now; full note when frozen.)

## The citizen model — identity decoupled from capability

- **Citizen** = a keyfile-tier operator identity (governor/director/retriever), rank-neutral: rank
  is not a property of the citizen, it lives in the capability-sets it holds. Multi-role is
  first-class — one citizen holds several capability-sets. (Federation peer = *federate*; payor =
  founder, outside both. Definitions: memo.)
- **Identity lifecycle is decoupled from capability lifecycle** — the spine. Adding/removing a
  citizen is independent of granting/revoking its capabilities. Today's `invest` fuses the two;
  the model unfuses them. (Why this is achievable: memo.)
- **Capability-set** = a named bundle of IAM grants, defined in code (lifted out of the invest
  bodies). Tier-blind.
- **The verbs dissolve** to add-citizen / remove-citizen / grant / revoke / rekey + read-ledger,
  parameterized by (actor, capability-set, citizen). invest/divest/mantle/roster stop existing as
  distinct concepts.
- **Capability verbs are tier-blind; only identity verbs branch by tier** (and by the payor's
  OAuth backing). rekey is keyfile-only.
- **Grant/revoke operate only on named capability-sets, never raw bindings** — the contamination
  guard that keeps the ledger auditable.
- Ontology: the depot's civic ontology covers **operator** identities — citizens (and later
  federates), people who hold capabilities — and **holdings** (arks/hoards/bullions), property.
  Cloud-native system identities (Mason, future Envoy) sit outside it. ("citizen" supersedes
  RBSHR's prose use of it for an artifact, now reworded to "holding.")

## Intent vs enforcement — the declared ledger (cinched)

Enforcement is server-side IAM, always. Intent — which citizen is meant to hold which capability —
is stored in a depot-resident **declared ledger**, distinct from IAM. The audit is the diff between
**actual IAM state** and the declared ledger; the routine ledger read is identical across tiers.
Federation forces the same ledger anyway, so it is paid once for both. (Today's name-regex `roster`
verb is retired — no prefix to filter on. Why it must be stored, and where it lives: memo.)

## The audit — drift as signal, asymmetric healing (cinched)

The audit measures drift between declared intent (ledger) and IAM. Healing is asymmetric, and the
asymmetry is the Pale recognition test:

- **Deficit** (IAM short of intent — our own grant failed) is ours → auto-converge, logged.
- **Surplus** (IAM beyond intent — touched outside our verbs) crossed the Pale → report; a human
  adjudicates. The audit never auto-revokes; surplus resolution is human-driven.

Both capability directions write intent first — grant writes the ledger before adding bindings,
revoke withdraws it before removing them — so a crash lands as a safe report-only surplus, never an
auto-converging deficit. Auto-converge is safe only where ledger-write authority is at least as
protected as grant (else a softer escalation path; see Open). And the audit reads **both
by-identity and by-resource-member**, so a grant to an unrostered principal cannot hide. (Drift
taxonomy, the member-first axis, definition-change gating, concurrency, and the orphan marker:
memo.)

## Authority topology — the one seam with policy content

"Who may grant which capability-set." Shape:

- Authority boundaries are IAM-real only where they fall on resource-scope splits (payor/governor
  holds because billing is a separate resource; governor/director does not).
- Grant authority is preventively scopable at **project** scope but only **detectively** at
  **resource** scope — so the resolution composes: IAM-limit the project-scoped grants,
  detective-audit the resource-scoped residual, and pull authority to the payor where a standing
  governor buys nothing.
- **Tier-dependent**: keyfile/solo → topology (no/minimal standing governor); federation/multi-admin
  → a condition-limited governor. The model allows the answer to differ (actor and capability-set
  are parameters).
- Surplus-detection's security value depends on the topology — do not substitute detective audit for
  structural closure. (The IAM attribute, its scope/role limits, and the seam reasoning: memo.)

## Blast radius — larger resting target, finer response (cinched)

One citizen = one SA = one key carrying the union of held capabilities — forced by "one identity per
person," accepted as a tradeoff. The resting-key form of this exposure is citizen-tier-only
(federation has no key at rest; the union property itself is tier-blind); most citizens wear one
hat; and decoupling buys finer incident response (revoke a capability without touching the key;
rekey without touching capabilities). (Reasoning: memo.)

## Inherited concern — governor teardown leak

Governor teardown sits inside this heat's blast radius: today's mantle leaks a project tombstone
per re-mantle. The convergence closes it (cheap revoke-before-delete; an idempotent governor is a
further, not-free refinement). Mechanism + standalone fallback if descoped:
memo + memo-20260605-governor-mantle-tombstone-leak.md.

## What done looks like

Every cult-verb surface migrated to the citizen/capability vocabulary: workstation bash (role-keyed
selection collapses to one identity accessor), the governor/payor admin verbs (the admin verbs
collapse to the dissolved set), cloud-side bash naming roles, the .adoc specs voicing the cult
verbs, the Rust theurge cases, the onboarding handbook. "citizen" replaces the role-named identity
throughout, and the SA name itself becomes identity-only (the naming migration: memo).

The load-bearing federation-enabling core is narrow: route the role/file-path-keyed token-accessor
calls through one identity-keyed accessor (generalizing memo-20260527's "single code seam"). The
vocabulary convergence is the larger, motivated-but-not-required remainder. Scoping/heat-chopping is
deferred — recorded here as shape.

## Cinched decisions

- Enforcement is server-side IAM. No file or client-side check is ever the boundary.
- The identity is the **citizen** — rank-neutral, keyfile-backed (federation peer *federate*; payor
  founder outside both). Identity lifecycle decoupled from capability lifecycle. Multi-role
  first-class.
- The citizen tier is one SA (one key, union of capabilities) per person, not one per role —
  blast-radius tradeoff accepted.
- Capability-sets are named code; grant/revoke operate only on named sets, never raw bindings.
- The declared ledger (intent) is part of the model; IAM is enforcement; the audit diffs them.
- Audit healing is asymmetric: auto-converge deficits, report surplus; the audit never auto-revokes.
- Ledger-write authority is never a capability-set member; the ledger writer for an entry is the
  grant-authority holder under the active topology.
- Governor breadth is resolved per tier, not globally: keyfile pulls grant authority to the payor /
  minimizes the standing governor; federation may keep a condition-limited governor.
- The mode enum (keyfile vs federation) belongs in RBRD and is added only when federation lands —
  not in this heat.
- Migration grandfathers existing role-named SAs: recover each citizen's role from the role-prefixed
  SA name (a one-time bootstrap, distinct from the retired standing "derive from IAM"), register
  citizens in the declared ledger, let legacy names age out. No flag day.
- The convergence covers operator citizens; cloud-native identities (Mason, future Envoy) share the
  capability-set abstraction but not the citizen lifecycle.

## Retired cinches

- "Roster derives from IAM (an `rbk-identity` label), not a stored list — it cannot drift." Retired:
  under decoupling, deriving from IAM is lossy and makes the intended-vs-actual audit a tautology,
  and SA labels do not exist (SAs support tags, in Preview). Replaced by the declared ledger +
  asymmetric audit. The true invariant it reached for — no enforcement state outside IAM — is
  preserved; the false corollary (no intent state may exist) is dropped.

## Open — resolve within the heat

- Where the declared ledger physically lives (a separate access-restricted object vs a GAR
  artifact) — explicitly **not** the build bucket (the director holds objectCreator there); the
  write-ACL must be held only by governor/payor (the ledger-write invariant; see The audit).
- Whether the keyfile tier drops the standing governor entirely or demotes it to non-privileged —
  this choice gates whether project-scope grant-limiting is in-heat.
- The citizen orphan marker: SA tags (Preview) vs description sentinel (both advisory; the
  authoritative orphan signal is a key on an SA absent from the ledger — see memo).
- Whether to offer a combined add+grant convenience wrapper for the common one-hat citizen (the
  two-gesture flow itself follows from the decoupling).

## Sources

rbk-08-credential-repairs; memo-20260604-credential-churn-leak-and-propagation-races (the lifecycle
split — the enabler); memo-20260527-operator-credential-models (the two-tier plan and cult-verb
shape — breadcrumbed, full update when frozen); memo-20260605-governor-mantle-tombstone-leak
(standalone governor-leak fix); memo-20260605-citizen-capability-model (the mechanism this paddock
points to); memo-20260605-citizen-model-ultracode-process (how this revision was produced);
RBSHR "Operator federation". This revision folds in the 260605 design conversation and two review
passes (single-reviewer + ultracode multi-agent).