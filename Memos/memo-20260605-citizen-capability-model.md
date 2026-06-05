# Citizen Tier — Identity / Capability Mechanics

Date: 2026-06-05

Status: Design captured during the 260605 grooming conversation for heat ₣BZ
(`rbk-14-citizen-model`). The *shape* lives in the ₣BZ paddock; this memo holds the
*mechanism* the paddock points to. Nothing here is built yet.

## Scope and vocabulary

This heat builds the **citizen tier** — the current, keyfile, no-org operator-credential
tier — restructured so its capability layer is identical to the future federation tier's, so
that federation plugs in later by swapping identity-kind and token mint and nothing else.

- **Citizen** — a keyfile-tier operator identity: a depot-issued service account holding one
  RBRA key. Rank-neutral (governor, director, retriever are all citizens); the home of the
  identity, not the role. Possession of the key is the identity.
- **Federate** (future, not this heat) — a federation-tier operator identity: an external-IdP
  `principal://` subject the depot recognizes by treaty. A federate holds no key.
- **Payor** — the founding human (OAuth refresh token), outside both tiers; neither citizen nor
  federate (memo-20260527).
- **Capability-set** — a named bundle of IAM grants, defined in code (the role's binding list,
  lifted out of today's invest bodies). Tier-blind.
- **Declared roster** — depot-resident data mapping identity → {capability-sets held}.
  Tier-blind.
- **Holdings** — the depot's artifacts (arks/hoards/bullions). Property, not identities. The
  depot's civic ontology is citizens/federates (people) + holdings (property).

The convergence: the **capability layer** (capability-sets, grant/revoke, the declared roster,
the audit) is tier-blind and shared. The **identity** differs by tier (citizen vs federate) —
that difference is the one real seam. "Structural isomorphism" means the capability layer is
byte-identical across tiers; only identity-kind and token mint (signed-JWT vs STS) differ.

## Verb dissolution

Today's role-keyed verbs decompose into a generic register parameterized by
(actor, capability-set, identity):

| Today | Decomposes to | Notes |
|---|---|---|
| invest (director/retriever) | add-citizen + grant(set) | governor is actor |
| mantle (governor) | add-citizen + grant(governor-set) | payor is actor; mantle is not special |
| divest | revoke(all held) + remove-citizen | |
| roster | read the declared roster | |
| — | grant(set) / revoke(set) | the capability axis, tier-blind |
| — | rekey | keyfile-only identity-backing maintenance |

Actor and capability-set are parameters, not verb variants. Federation reuses the capability
half (grant/revoke/roster) verbatim and swaps the identity half (add-federate, no rekey).

## Identity verbs branch; capability verbs do not

- **Capability verbs (grant/revoke)** never branch on tier — both are an IAM binding on a
  pre-existing identity.
- **Identity verbs** are tier-specific (this is the seam):
  - *citizen* (this heat): add = mint SA + key + deliver RBRA; rekey = rotate the key, bindings
    untouched; remove = revoke-all then delete the SA.
  - *federate* (future): add ≈ first grant (identity owned by the IdP, not minted here); no rekey
    (the IdP owns rotation); remove = de-register + revoke-all (the principal is not ours to
    delete).
  - *payor*: install / refresh a human OAuth refresh token.
- Token mint also branches (citizen: signed-JWT; federate: STS) and sits below everything.

## The declared roster

Why intent must be stored (the retired "derive from IAM" cinch was self-defeating once the role
prefix leaves the identity name):

1. Without a name prefix, deriving the roster from IAM is lossy reverse-mapping — a director set
   is several bindings smeared across project + Mason SA + repo + bucket; partial and ambiguous
   matches become representable.
2. You cannot audit IAM against itself — "intent = whatever IAM says" makes intended-vs-actual a
   tautology that always passes, discarding the drift signal.

Federation forces the same roster anyway (a federated principal carries no role marker;
memo-20260527: "the depot remains the home of the roster"), so the roster is paid once for both
tiers and makes routine roster identical and O(1) across tiers.

- **Code vs data.** Capability-set *definitions* are code; the roster is *data* (memberships).
  The audit compares: for each (identity, declared-set), are the code-defined bindings present in
  IAM?
- **Physical home (open).** A GCS object in the depot bucket — mutable, high-churn, etag-guarded
  read-modify-write (like IAM policies themselves) — is the natural fit, unlike tripwire-protected
  RBRD. A GAR artifact is an alternative. Decide in-heat.
- **Writers.** The governor writes director/retriever entries; the payor writes the governor
  entry — matching the authority hierarchy.

## The audit

Drift taxonomy (for one identity; D = union of declared sets' bindings, A = actual RBK-relevant
bindings):

| State | Meaning | Likely cause | Heal |
|---|---|---|---|
| D = A | clean | — | — |
| deficit-whole | a declared set wholly absent in A | grant never ran / errant revoke | auto-converge |
| deficit-fragment | a declared set partly present | half-failed multi-scope grant | auto-converge |
| surplus-complete | a whole undeclared named set in A | out-of-band grant | report |
| surplus-fragment | bindings belonging to no whole set | tampering / shrunk definition | report |

**Asymmetric healing = the Pale recognition test:**

- **Deficit is ours** (our own grant failed to complete) → auto-converge up to D (re-run the
  idempotent grant), logged, never silent.
- **Surplus crossed the Pale** (IAM touched outside our verbs) → characterize, report, let a human
  adjudicate (legitimize → declare it, or reject → revoke it). Never auto-revoke (a stale roster
  would deprovision a working identity); never auto-backfill intent (re-enters the tautology). No
  verb auto-revokes; the destructive operation is always human-driven.

Consequences:

- Because grant writes intent *first*, our tooling cannot produce a surplus — so a surplus is a
  clean "someone bypassed the verbs" signal.
- A capability-set definition change decomposes into deficit-on-additions (auto-heal via
  roster-wide re-converge) + surplus-on-removals (report for human-confirmed revoke) — no special
  versioning machinery.
- Audit domain = roster-identities × set-defined bindings; non-roster principals are out of scope.

## Orphan hazard and the identity marker (citizen tier only)

With no name prefix and no SA labels, nothing marks an SA as RBK-managed — so a failed add-citizen
can strand a live key for an untracked SA (a credential leak you cannot find). Marker options:

- **SA tags** (tag bindings): key-value, listable, condition-usable — the proper metadata
  mechanism. Caveat: SA tags are **Preview** (may change).
- **SA `description` sentinel** (`rbk:…`): stable fallback, used only for the orphan-sweep; the
  roster owns capability, the description earns exactly this one job.

Federation has no key to leak and a stray principal binding is visible in the policy, so this
hazard is citizen-tier-specific.

## Authority — who may grant which capability-set

- **Resource-scope-split principle.** An authority boundary is IAM-real only where it coincides
  with a resource boundary. Payor/governor holds because billing is a separate resource that
  governor's `roles/owner` excludes. Governor/director does not — both live in project IAM and
  governor has owner over it — so "governor may grant only director/retriever" is, by itself,
  honor-system.
- **IAM can scope grants by role** (corrects the first paddock draft). The condition attribute
  `iam.googleapis.com/modifiedGrantsByRole` limits which roles a setIamPolicy-capable principal
  may grant/revoke:
  `api.getAttribute('iam.googleapis.com/modifiedGrantsByRole', []).hasOnly(['roles/x','roles/y'])`.
  Limits: works only on **project/folder/org** allow policies, **not** resource-level (AR repo,
  bucket, individual SA); ≤10 roles per condition; don't join `hasOnly` with `&&`/`||`; some
  services don't recognize the limit (request fails); the governor must NOT hold plain
  `roles/owner` (unconditional setIamPolicy) — give it a conditioned setIamPolicy instead.
  Google's own caveat — don't let a limited admin grant roles bearing setIamPolicy — is satisfied
  because director/retriever sets contain none.
- **Composed resolution.**
  - Project-scoped grants → IAM-limit on the governor (preventive, server-side).
  - Resource-scoped grants (repo/bucket/SA) → not IAM-limitable → detective audit covers the
    residual.
  - Topology → pull grant authority to the payor where a standing governor buys nothing.
- **Tier-dependent.** Keyfile/solo (payor == admin) → topology: a standing owner-holding citizen
  buys nothing and is pure attack surface; drop or demote it. Federation/multi-admin (no
  exfiltrable key) → a condition-limited governor is the right preventive tool.
- **Audit ↔ topology seam.** Surplus-detection's security value depends on topology: a broad
  standing governor makes surplus-detection the only escalation tripwire (detective-only, weak vs
  malice); authority at the payor closes escalation structurally and demotes the audit to
  honest-mistake hygiene. Do not substitute detective audit for structural closure.
- **Ephemeral governor** (decoupling-enabled option): grant the governor-set for one op, then
  revoke. Shrinks resting owner-exposure; relevant only if a keyfile delegated governor is ever
  wanted.

## Blast radius — full reasoning

One citizen per person = one SA = one key carrying the **union** of granted capabilities. This is
**forced** by "one identity per person": an SA's key always wields the SA's full bindings; you
cannot scope one key to a subset. The only escape — separate SAs per role — is the old model,
which discards the citizen goal. So accepting multi-role on one citizen *is* accepting the union
key; they are one decision.

- Cost: larger resting target (a leaked key carries more).
- Bounded: only in the citizen tier (a file to steal); federation has no key. Most citizens wear
  one hat (one-per-person == one-per-role).
- Upside: finer incident response — revoke a capability without touching the citizen/key, or rekey
  without touching capabilities. The old model's only lever was delete-and-recreate.

## Governor teardown leak (folds in)

`rbgp_governor_mantle` deletes outgoing `governor-*` SAs without first revoking their grants,
minting a `deleted:…?uid=` tombstone per re-mantle. The convergence closes it for free: the
generic revoke-before-delete covers the governor like any citizen, and an idempotent governor
(rekey, not delete+recreate) drops the delete entirely. Reuse the rbk-08 revoke layer + Class-C
propagation tolerance. Standalone fix (if the heat descopes the governor):
memo-20260605-governor-mantle-tombstone-leak.md.

## Verified facts (checked 260605)

- **Service accounts do not support resource labels.** They support **tags** (tag bindings),
  currently in Preview. So the orphan marker uses tags (Preview) or the description sentinel, not a
  label.
- **IAM can limit which roles a principal may grant** via `modifiedGrantsByRole`, but only on
  project/folder/org policies and ≤10 roles — not on resource-level policies.

Sources:
- Set limits on granting roles — https://docs.cloud.google.com/iam/docs/setting-limits-on-granting-roles
- Tags for service accounts — https://docs.cloud.google.com/iam/docs/service-accounts-tags
- Service accounts overview — https://docs.cloud.google.com/iam/docs/service-account-overview

## memo-20260527 divergence (record when the model is frozen)

memo-20260527 states the operator verbs "keep their names, only their bodies differ," and that
keyfile holds "one SA key file each per role." This heat supersedes both: the verb *bodies*
converge in shape (not just names), and the citizen tier is one SA *per person*, not per role.
Update memo-20260527 to record this when the model is frozen.
