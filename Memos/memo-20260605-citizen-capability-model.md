# Citizen Tier — Identity / Capability Mechanics

Date: 2026-06-05

Status: Design captured during the 260605 grooming conversation for heat ₣BZ
(`rbk-14-citizen-model`). The *shape* lives in the ₣BZ paddock; this memo holds the
*mechanism* the paddock points to. Nothing here is built yet. Amended 260605 after two review
passes (a single-reviewer pass, then an ultracode multi-agent pass — see
memo-20260605-citizen-model-ultracode-process.md).

## Scope and vocabulary

This heat builds the **citizen tier** — the current, keyfile, no-org operator-credential
tier — restructured so its capability layer is identical to the future federation tier's, so
that federation plugs in later by swapping identity-kind and token mint and nothing else.

- **Citizen** — a keyfile-tier operator identity: a depot-issued service account holding one
  RBRA key. Rank-neutral (governor, director, retriever are all citizens); the home of the
  identity, not the role. Possession of the key is the identity. (Mints over RBSHR's prose use of
  "citizen" for a depot *artifact* — those are reworded to "holding"; minting rule: semantic
  uniqueness across all persistent names.)
- **Federate** (future, not this heat) — a federation-tier operator identity: an external-IdP
  `principal://` subject the depot recognizes by treaty. A federate holds no key. (This is the
  *Workforce*-federated operator subject; not the envoy's *Workload* "federates-to-AWS" egress,
  which RBSHR Envoy and memo-20260527 Workforce-vs-Workload already separate.)
- **Payor** — the founding human (OAuth refresh token), outside both tiers; neither citizen nor
  federate (memo-20260527).
- **Capability-set** — a named bundle of IAM grants, defined in code (the role's binding list,
  lifted out of today's invest bodies). Tier-blind.
- **Declared ledger** — depot-resident data mapping identity → {capability-sets held}: the stored
  *intent*. Tier-blind. Distinct from the existing `roster` cult-verb (RBSDR), which reads the
  *actual* discovered IAM state — the audit is precisely the diff between the actual-state roster
  and the declared ledger. (Short form below: "the ledger.")
- **Holdings** — the depot's artifacts (arks/hoards/bullions). Property, not identities. The
  depot's civic ontology — citizens/federates (people) + holdings (property) — covers *operator*
  identities; cloud-native system identities (Mason, future Envoy) sit outside it, reusing the
  capability-set abstraction without the citizen lifecycle (whether they deserve their own
  ontological category is deferred).

The convergence: the **capability layer** (capability-sets, grant/revoke, the declared ledger,
the audit) is tier-blind and shared. The **identity** differs by tier (citizen vs federate) —
that difference is the one real seam. "Structural isomorphism" means the capability layer is
identical across tiers *modulo the opaque principal handle the identity layer supplies*
(`serviceAccount:…` vs `principal://…`); only identity-kind, that handle, and the token mint
(signed-JWT vs STS) differ.

## Verb dissolution

Today's role-keyed verbs decompose into a generic register parameterized by
(actor, capability-set, identity):

| Today | Decomposes to | Notes |
|---|---|---|
| invest (director/retriever) | add-citizen + grant(set) | governor is actor |
| mantle (governor) | add-citizen + grant(governor-set) | payor is actor; not special *once the governor is idempotent* (see below) |
| divest | revoke(all held) + remove-citizen | ledger-withdraw-first (see Identity verbs) |
| roster (verb) | read the declared ledger | the actual-reading verb; the audit diffs it against IAM |
| — | grant(set) / revoke(set) | the capability axis, tier-blind |
| — | rekey | keyfile-only identity-backing maintenance |

Actor and capability-set are parameters, not verb variants. Federation reuses the capability
half (grant/revoke/ledger-read) verbatim and swaps the identity half (add-federate, no rekey).

**mantle is not special only once the governor is idempotent** (stable name, add-once + rekey). If
the governor stays datestamped delete+recreate (today's RBSGM), mantle decomposes to
*divest + add-citizen + grant* and remains a replace verb — see Governor teardown leak.

## Identity verbs branch; capability verbs do not

- **Capability verbs (grant/revoke)** never branch on tier — both are an IAM binding on a
  pre-existing identity.
- **Identity verbs** are tier-specific (this is the seam):
  - *citizen* (this heat):
    - **add** = mint SA + key + deliver RBRA.
    - **rekey** = rotate the key, bindings untouched. Availability-safe order: create-new →
      deliver RBRA → verify → delete-old (never delete-before-create — a failed create would leave
      a live citizen keyless). GCP caps an SA at **10 keys** (hard, non-adjustable), so a rekey
      that fails to reclaim the old key accumulates toward the ceiling; reclaim stale keys, and let
      the orphan/audit sweep flag `>1` USER_MANAGED key on a citizen as a stranded key to reclaim.
    - **remove** = **withdraw ledger intent (delete the citizen's ledger rows) FIRST**, then revoke
      IAM, then delete the SA. Invariant: *no IAM revoke without a prior ledger withdrawal* — the
      teardown dual of grant-writes-intent-first. (Ledger-first means a crashed teardown lands as a
      transient *surplus* — reported, safe — not an auto-converging *deficit* that would resurrect
      the just-removed citizen, acute because RBSDD anticipates immediate same-folio re-invest.)
  - *federate* (future): add ≈ first grant (identity owned by the IdP, not minted here); no rekey
    (the IdP owns rotation); remove = de-register + revoke-all (the principal is not ours to
    delete).
  - *payor*: install / refresh a human OAuth refresh token.
- Token mint also branches (citizen: signed-JWT; federate: STS) and sits below everything.

## The declared ledger

Why intent must be stored (the retired "derive from IAM" cinch was self-defeating once the role
prefix leaves the identity name):

1. Without a name prefix, deriving the ledger from IAM is lossy reverse-mapping — a director set
   is several bindings smeared across project + Mason SA + repo + bucket; partial and ambiguous
   matches become representable.
2. You cannot audit IAM against itself — "intent = whatever IAM says" makes intended-vs-actual a
   tautology that always passes, discarding the drift signal.

Federation forces the same ledger anyway (a federated principal carries no role marker;
memo-20260527: "the depot remains the home of the roster"), so the ledger is paid once for both
tiers and makes the routine ledger read identical and O(1) across tiers.

- **Code vs data.** Capability-set *definitions* are code; the ledger is *data* (memberships).
  The audit compares: for each (identity, declared-set), are the code-defined bindings present in
  IAM?
- **Physical home (open) — a security boundary, not just storage.** The home must place
  ledger-write authority at least on par with grant authority (see the audit invariant below).
  This **disqualifies the obvious choice**: a GCS object in the depot's *build* bucket fails the
  invariant, because the director already holds `roles/storage.objectCreator` there (RBSDK) — a
  director could overwrite the ledger and self-escalate via auto-converge. The ledger home's
  write-ACL must be reachable by **no capability-set** — held only by governor/payor. Viable
  options: a separate access-restricted bucket/object, or a GAR artifact, each with a
  governor/payor-only write ACL. Decide in-heat. **Cinch: ledger-write authority is never a
  capability-set member.**
- **Writers (topology-conditional).** The ledger writer for an entry is the holder of grant
  authority for that entry's capability-sets under the active topology.
  - Federation/multi-admin: the governor writes director/retriever entries; the payor writes the
    governor entry.
  - Keyfile/solo: the payor writes director/retriever entries directly; a demoted or absent
    governor must NOT write entries it lacks grant authority for.

## The audit

Drift taxonomy (for one identity; D = union of declared sets' bindings, A = actual RBK-relevant
bindings):

| State | Meaning | Likely cause | Heal |
|---|---|---|---|
| D = A | clean | — | — |
| deficit-whole | a declared set wholly absent in A | grant never ran / errant revoke | auto-converge |
| deficit-fragment | a declared set partly present | half-failed multi-scope grant | auto-converge |
| surplus-complete | a whole undeclared named set in A | out-of-band grant | report |
| surplus-fragment | bindings belonging to no whole set | tampering / shrunk definition / half-failed revoke | report (but see below) |

**Half-failed revoke is ours, not adversarial.** A multi-scope revoke (a director set spans
project + Mason + repo + bucket) that fails partway — exactly the Class-C 403-flap memo-20260604
documents — leaves a fragment of a no-longer-declared set: structurally a surplus, but *ours*. The
divest/remove verb retries its own revoke idempotently (Class-C tolerance) so most flaps resolve
(~81s) without stranding a fragment; for a fully abandoned mid-divest, a recorded "revoking" intent
marker lets the audit recognize an interrupted authorized revoke as ours and finish it
idempotently, distinct from out-of-band surplus.

**Asymmetric healing = the Pale recognition test:**

- **Deficit is ours** (our own grant failed to complete) → auto-converge up to D (re-run the
  idempotent grant), logged, never silent.
- **Surplus crossed the Pale** (IAM touched outside our verbs) → characterize, report, let a human
  adjudicate (legitimize → declare it, or reject → revoke it). Never auto-backfill intent
  (re-enters the tautology). **The *audit* never auto-revokes** — surplus resolution is
  human-driven. (Divest/remove *do* revoke; that is an explicit operator teardown, not an audit
  heal.)

**Invariant the asymmetry rests on.** Auto-converging deficits treats the declared ledger as
authoritative intent, so writing the ledger *upward* is itself a grant. The auto-heal is safe only
if ledger-write authority is at least as protected as the grant verbs (setIamPolicy); otherwise it
is a softer escalation path than setIamPolicy. The ledger's physical home (above) must satisfy that
ACL constraint — a security boundary, not a storage convenience.

**Two audit axes.** The identity-first axis (above) walks `ledger identities × set-defined
bindings`. It **cannot see** escalation onto a *fresh, non-ledger* principal — e.g. a leaked
director, via repo-scope `repoAdmin`/`setIamPolicy`, grants a role to a brand-new SA the ledger
never names. So the audit also runs a **member-first axis**: on every resource where an RBK
principal can reach `setIamPolicy` (the AR repo, the build bucket, individual SAs), enumerate the
*actual* members and flag any RBK-relevant binding whose member is neither a ledger identity nor an
allow-listed system identity (Mason, payor, governor, future Envoy — allow-listed to avoid false
positives). The repoAdmin residual (Authority section) is covered **only once this axis exists**.

Consequences:

- Because grant writes intent *first*, our tooling cannot produce a surplus *absent a definition
  change* — so a surplus is normally a clean "someone bypassed the verbs" signal. The one
  tooling-side surplus source is a capability-set definition *shrink* (next bullet), which strands a
  binding our own earlier grant placed — which is why surplus is reported, not auto-revoked,
  regardless of origin.
- **A definition change is not all routine.** A *shrink* decomposes to surplus-on-removals (report
  for human-confirmed revoke). An *expansion* (adding a binding to a set's definition) turns every
  holder into a deficit the audit would auto-grant — silently relocating the privilege-expansion
  trust boundary to "who can merge a one-line code change." So distinguish a per-identity deficit
  against an *unchanged* definition (a genuine failed grant — safe to auto-heal) from a deficit
  appearing across *all* holders of a set at once (the signature of an expansion — gate behind the
  same human adjudication that surplus-legitimization requires). **"Who may edit capability-set
  definitions" is a first-class authority, equal to grant / ledger-write.**
- **Concurrency.** Two writers hit IAM: the grant verb and the audit's deficit auto-converge.
  Auto-converge re-reads the ledger under etag immediately before each grant (not from a
  start-of-audit snapshot), so a concurrent revoke wins. Both teardown sides (revoke, definition
  shrink) write the ledger first so a crash lands as report-only surplus, never an auto-converging
  deficit that resurrects what was just removed.
- **Audit domain.** Identity-first: ledger identities × set-defined bindings. Member-first: actual
  members of setIamPolicy-reachable resource policies. A non-ledger, non-system member is *flagged*,
  not ignored.

## Orphan hazard and the identity marker (citizen tier only)

With no name prefix and no SA labels, nothing intrinsic marks an SA as RBK-managed — so a failed
add-citizen can strand a live key for an untracked SA (a credential leak you cannot find).

The orphan sweep is **integrity-advisory, not authoritative.** The description sentinel lives in
`serviceAccounts.update` — the same `serviceAccountAdmin` authority that mints citizens — so it can
be stripped, or absent after a crash. Marker *absence must not be trusted.* The **authoritative**
orphan signal is the ledger cross-check the capability audit already performs: a USER_MANAGED key on
an SA absent from the declared ledger is an orphan, marker or not. Marker options, used only as
corroborating hints:

- **SA tags** (tag bindings): key-value, listable, condition-usable, with a separable (stronger)
  tag-binding ACL — preferred once they leave **Preview**.
- **SA `description` sentinel** (`rbk:…`): stable fallback for the sweep; the ledger owns
  capability, the description earns exactly this one hint.

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
  at *project* scope: the project-scoped director/retriever grants bear no setIamPolicy. The
  repo-scoped `roles/artifactregistry.repoAdmin` *does* carry
  `artifactregistry.repositories.setIamPolicy` — that is the resource-scope residual covered by the
  audit's **member-first axis** (the identity-first axis alone cannot see a grant to a fresh
  principal), and modifiedGrantsByRole does not reach repo scope anyway.
- **Composed resolution.**
  - Project-scoped grants → IAM-limit on the governor (preventive, server-side).
  - Resource-scoped grants (repo/bucket/SA) → not IAM-limitable → detective audit (member-first
    axis) covers the residual.
  - Topology → pull grant authority to the payor where a standing governor buys nothing.
- **Tier-dependent.** Keyfile/solo (payor == admin) → topology: a standing owner-holding citizen
  buys nothing and is pure attack surface; drop or demote it. Federation/multi-admin (no
  exfiltrable key) → a condition-limited governor is the right preventive tool.
- **Audit ↔ topology seam.** Surplus-detection's security value depends on topology: a broad
  standing governor makes surplus-detection the only escalation tripwire (detective-only, weak vs
  malice); authority at the payor closes escalation structurally and demotes the audit to
  honest-mistake hygiene. Do not substitute detective audit for structural closure.
- **Ephemeral governor** (decoupling-enabled option): grant the governor-set for one op, then
  revoke. Caveat: because capability-revoke is decoupled from key-destruction, this strips the
  *binding* but leaves the on-disk key — to shrink the keyfile attack surface you must couple
  key-destruction (rekey-to-nothing or SA delete) to the revoke; and during grant→use→revoke plus
  revoke lag (~81s, memo-20260604) the on-disk key carries full owner. Relevant only if a keyfile
  delegated governor is ever wanted.

## Blast radius — full reasoning

One citizen per person = one SA = one key carrying the **union** of granted capabilities. This is
**forced** by "one identity per person": an SA's key always wields the SA's full bindings; you
cannot scope one key to a subset. The only escape — separate SAs per role — is the old model,
which discards the citizen goal. So accepting multi-role on one citizen *is* accepting the union
key; they are one decision.

- Cost: larger resting target (a leaked key carries more).
- Bounded: the *resting-key* form of this exposure is citizen-tier-only (a file to steal);
  federation has no key at rest. The union-of-capabilities-on-one-identity property is itself
  tier-blind — a federate granted director+retriever wields both too, exposed through a hijacked
  live session rather than a file at rest. Most citizens wear one hat (one-per-person ==
  one-per-role).
- Upside: finer incident response — revoke a capability without touching the citizen/key, or rekey
  without touching capabilities. The old model's only lever was delete-and-recreate.

## Governor teardown leak (folds in)

`rbgp_governor_mantle` deletes outgoing `governor-*` SAs without first revoking their grants,
minting a `deleted:…?uid=` tombstone per re-mantle. The leak closes cheaply: add the generic
revoke-before-delete (reusing the rbk-08 revoke layer + Class-C propagation tolerance) before the
existing delete — no tombstone, no naming change. The further idempotent-governor refinement (rekey
instead of delete+recreate, dropping the delete entirely) is *not* free: it needs a *stable*
governor name in place of today's datestamped `governor-YYYYMMDDHHmm` (which exists precisely to
dodge the delete→recreate flap), plus a one-time migration of existing datestamped governors.
Standalone leak fix (if the heat descopes the governor):
memo-20260605-governor-mantle-tombstone-leak.md.

## Citizen naming migration

The model's foundation is the SA name becoming **identity-only** (`alice`, not `director-alice`) —
the precondition for one-citizen-one-key-union and for the lossy-IAM-derivation argument. (The
governor's analogous rename is in Governor teardown leak; this is the director/retriever dual.)
Surfaces:

- **RBSDK / RBSRK** — SA-name composition drops the role prefix (was `${RBCC_account_director}-<identity>`).
- **RBSDD** — email synthesis becomes identity-only.
- **RBSDR** — the prefix-filter roster is retired in favor of the declared-ledger read.
- **RBSRA / RBSRR** — the three role-keyed `rbra.env` paths collapse toward one identity-keyed path.

**Existing-SA cutover.** GCP cannot rename an SA, so legacy `director-*`/`retriever-*` SAs migrate by
delete-old + invest-new, re-entering the divest revoke-before-delete + the invest create/propagation
race (the memo-20260604 churn) — it must run through the rbk-08 revoke layer + Class-C tolerance.

**One-time bootstrap** (distinct from the *retired* standing "derive from IAM"): recover each legacy
citizen's role — and thus its capability-set — from the role-prefixed SA **name** (the RBSDR roster
regex), reading bindings only to seed/confirm; thereafter the ledger stands independently. Stamp the
orphan marker onto each grandfathered SA so the sweep operates on a marker-uniform domain (a
back-stamp prerequisite — until then sweep coverage of the legacy population is incomplete).

Whether *this heat* performs the migration is under the open scoping flag (the paddock).

## Verified facts (checked 260605)

- **Service accounts do not support resource labels.** They support **tags** (tag bindings),
  currently in Preview. So the orphan marker uses tags (Preview) or the description sentinel, not a
  label.
- **IAM can limit which roles a principal may grant** via `modifiedGrantsByRole`, but only on
  project/folder/org policies and ≤10 roles — not on resource-level policies.
- **A service account is capped at 10 keys** (hard, non-adjustable) — bounds the rekey lifecycle.
- A service account **can exist with zero IAM bindings** (the decoupling primitive).

Sources:
- Set limits on granting roles — https://docs.cloud.google.com/iam/docs/setting-limits-on-granting-roles
- Tags for service accounts — https://docs.cloud.google.com/iam/docs/service-accounts-tags
- Service accounts overview — https://docs.cloud.google.com/iam/docs/service-account-overview

## memo-20260527 divergence (record when the model is frozen)

memo-20260527 states the operator verbs "keep their names, only their bodies differ," and that
keyfile holds "one SA key file each per role." This heat supersedes both: the verb *bodies*
converge in shape (not just names), and the citizen tier is one SA *per person*, not per role.

A third divergence the paddock leans on by name: 20260527's "single code seam" is a *role-keyed*
token accessor ("a token as director"), shippable with no behavior change over today's per-role
SAs. Under one-SA-per-person the accessor becomes *identity-keyed* ("a token as alice", carrying the
union) — a generalization of that seam, but *not* itself no-behavior-change, since it presupposes
the per-person migration. (Whether the heat's "load-bearing core" is the role-keyed no-behavior
stepping-stone or the identity-keyed end state is an open scoping question — see the paddock.)

A deferral-honoring breadcrumb has been added to memo-20260527's Status block now; the full
divergence note folds in when the model is frozen.
