# Org-Affiliated Credential Reorientation — Session Findings & Design Direction

Date: 2026-05-22

Status: **Exploration, not a decision.** Captured from a live design
conversation while first-running the ₣BM skirmish suite under a newly
org-affiliated Payor. Feeds ₢BSAAF (org-affiliated-account-reorientation).
Extends and reframes `memo-20260427-google-native-human-auth.md`.

---

## Why this memo exists

The project's active Payor was moved from a consumer Google account
(`bhyslop@gmail.com`, no organization) to an org-affiliated Workspace identity
(`bhyslop@scaleinvariant.org`, organization `247899326218`). What looked like a
routine account switch surfaced a foundational truth: **the project's credential
model was built on a consumer-account assumption that organizations invalidate.**
This memo records what we proved, the reframe it forced, and the design
direction it points to — so the thinking can be resumed deliberately rather than
re-derived.

---

## 1. The foundational realization

The RBRA pattern — mint a service-account key file, place it on disk, let
whatever holds it act as that role (Governor / Director / Retriever) — is the
bedrock the project stands on. It works beautifully for a solo developer on a
consumer account, and it required *zero* cloud-side identity machinery to
bootstrap. That was the right call to get started.

But it is a **consumer-account** assumption. Serious organizations treat
downloadable, long-lived service-account keys as a security liability (they
never expire, they can leak, possession alone grants power). So secure-by-default
Workspace orgs **forbid creating them**. A project whose credential model
*requires* minting SA keys is therefore not merely "frictionful" in such orgs —
it is **structurally rejected** unless the operator deliberately weakens org
policy.

Conclusion the conversation reached, plainly: **durable RBRA keyfiles are
anathema to serious orgs.** Keeping them as the *primary* mechanism is not a path
to acceptance in organizations the project does not itself administer. RBRA is
the consumer / bootstrap mode, not the org-native mode.

---

## 2. What an organization actually imposes (two independent policies)

Empirically confirmed this session. These are **two separate dials**, not one
intertwined problem:

| Policy | What it does | Where it bit us |
|--------|--------------|-----------------|
| **Cloud session / reauthentication control** | Periodically invalidates the session and forces the human to re-prove identity | Payor OAuth refresh token returned `invalid_rapt` after ~22h |
| **`iam.disableServiceAccountKeyCreation`** | Forbids creating downloadable SA keys | Governor mantle created the SA, granted it owner, then was refused the key |

- **The reauth cadence is org-configurable**, not an OAuth law. A consumer
  account has no admin imposing it, which is why the gmail Payor's refresh token
  is effectively immortal and *has literally never needed reauthorization*. A
  Workspace admin can set the cadence anywhere from ~1h to "never expires."
- **The key-creation block is the structural one.** It is enforced by default on
  the org, inherited by every project under it.

---

## 3. Session evidence (the override experiment)

We unblocked the standing depot by hand, to learn the shape of the problem:

- Setting a **project-scoped override** clearing `disableServiceAccountKeyCreation`
  works — key creation succeeded ~60–90s after the policy write, and the full
  `canonical-invest` fixture then passed (governor mantle + retriever + director
  invests, real keys minted).
- But the override **requires `roles/orgpolicy.policyAdmin`, which is
  organization/folder-scoped only** — gcloud rejected a project-level grant
  outright (`Role ... is not supported for this resource`). So whoever performs
  the override on an org depot must wield **org-wide** Organization Policy
  Administrator authority.
- **This is a survival hack for an org you administer, not a strategy for orgs
  you do not.** The orgs the project wants to win over are precisely the ones
  that will not let you turn that policy off.
- Operational notes: the override is durable (persists until reverted); minted SA
  keys are long-lived (no reauth issue once created); the override is per-project
  (a fresh depot levy re-hits the wall); the mantle is **not idempotent** on the
  key-creation-failure path (it deletes and recreates the SA, grants owner, then
  fails — leaving a keyless-but-owner SA). The retriever/director invests are
  likewise not idempotent on rerun (HTTP 409 if the SA already exists). The
  idempotency gap is captured separately as a theurge fix (₢BMAAG).

---

## 4. The reframe: reauth is a compass, not an obstacle

The central turn of the conversation. The reauth requirement is not noise the
org throws in the way — it is **the org telling you what it considers a
trustworthy credential**:

> Trust must trace back to a human who recently proved they are present, and
> nothing durable should sit on disk.

A downloaded SA key violates both halves (durable, and detached from any present
human). Reauth is the org continuously re-grounding the entire trust chain in a
live person. If you follow that compass instead of routing around it, you land
exactly where serious orgs want to be — and where central authorize/deauthorize
of roles becomes natural:

- **No durable secret exists to steal.** The human holds a refresh token the org
  can age out; the roles hold nothing on disk.
- **Authorization is one IAM binding**, set centrally by an admin. Nothing to ship.
- **Deauthorization is one binding deletion** — instant, central, total. No key
  to hunt down, no rotation, no trusting old copies are gone.
- **Audit names the human** (`delegationInfo`: "Alice acted as director at
  14:03"), not the invisible-keyfile "the director did something."

The thing that was annoying us all afternoon is, structurally, the doorway to
the model the project actually wants.

---

## 5. The target model

**Human OAuth as the universal front door; roles stay service accounts, reached
by impersonation.** Critically, this is *not* "recast Director/Retriever to be
OAuth identities like Payor." The SA principals are preserved (their roles,
depot-scoped identity, audit trail, the RBSCIG/RBGI taxonomy all survive). What
changes is the *front door*: instead of holding a role's downloaded keyfile, a
human authenticates as themselves and asks Google to let them **act as** the role
SA for a short window (`generateAccessToken` on `iamcredentials.googleapis.com`).

The load-bearing distinction — the one that prevents recreating the RBRA mess:

- **The credential proves identity, not authority.** It says "you are Alice."
- **Authority lives centrally in IAM bindings**, granted/revoked in one place by
  an admin. The server decides, at action time, whether Alice may act in the role
  the action requires.
- If a credential's *contents* ever decide what the holder may do, RBRA has been
  rebuilt. If the credential only says *who* the holder is and the server
  consults central grants, it has not.

This is also the architecture the 2026-04-27 memo proposed (Approach C). The
org-policy finding adds a trigger that memo never anticipated: it is no longer a
future *convenience*, it is the *only* path that survives on org accounts the
project does not administer.

---

## 6. Recasting RBRA (the vessel survives; the contents change)

The credential file at a regime path survives as a concept. Its contents change,
and the natural template already exists: `RBRO` (the Payor) holds
`RBRO_CLIENT_SECRET` + `RBRO_REFRESH_TOKEN`, not an SA key.

The knot to untie: **you cannot mint a refresh token *per role*, because a
service account cannot do the browser dance — only a human can.** The browser
flow only ever produces *your* (human) refresh token. So the recast is *not*
"each RBRA holds its own long-lived secret." It collapses to:

- **One human identity credential** (RBRO-shaped — the operator's refresh token),
  written by the authentication step.
- **Each role's RBRA becomes a secretless pointer**: "to act as Director,
  impersonate `director-…@…`." No key, no token — just the SA's address and the
  instruction to impersonate it.
- Authority to use the pointer is the IAM binding; revocation is its deletion.

This is *better* than a one-for-one recast: it reduces the number of secrets to
one, rather than re-housing the same count in a new form.

---

## 7. The right axis: human-driven vs unattended (not org vs no-org)

A tempting but wrong split is "no-org mode" vs "with-org mode." It is wrong
because **org-ness is not the project's to control** — it is a property of each
user's account, so branching on it means defending behavior the project cannot
predict.

The defensible axis is the *workload*, which the project does control:

- **Human-driven** (a person at a workstation, even a headless one): can always
  do the browser/device authentication and periodic reauth. Impersonation fits,
  on any org, **without needing to administer that org** — granting impersonation
  rights is an IAM binding the depot's own Governor controls, not an org-policy
  override requiring org-admin.
- **Unattended** (Cloud Build, CI with nobody present): cannot reauth, so it
  needs a *machine* identity (workload identity / build SA) — keyless, and
  org-sanctioned — regardless of org.

Same two modes whether the user is on gmail or a Workspace never seen before.
Fewer permutations, keyed on something stable and owned.

---

## 8. The synthetic-human CI complication (the genuinely hard corner)

The clean axis above has one exception specific to this project: **its
cross-platform CI must behave *as if a human* operates it** (Windows / macOS /
Linux), because the point is to validate the human-operator experience. A machine
identity would exercise a code path no real human walks. So this CI is
**unattended *and* must wear a human face** — a "synthetic human" the clean axis
does not allow for.

Resolution, with honest costs:

- It runs on an org **the project administers** (its own test org), so the
  session policy can be set long / never-expiring there, and the synthetic-human
  credential can be emplaced before a run and survive it untouched. **Contained
  to the project's own yard; never part of the shipped model.**
- "Very long lived" is a privilege grantable only on orgs you administer.
  End-users on strict orgs get *their* org's cadence, full stop.
- **Residual cost:** a long-lived human refresh token sitting on a CI box before
  a run is still a durable secret — a milder, revocable one, but durable. This is
  the one place the project does not fully escape "a long-lived secret on a
  machine." Acceptable for the test rig; not held up as the org-native ideal.

Practical near-term mitigation (independent of any rework): the reauth window
only governs the *human Payor* OAuth; the long, slow parts of a run (cloud
builds via the Director SA, crucibles) use long-lived SA credentials and have no
reauth window. So a fresh authentication (or a cheap preflight probe that
attempts a token refresh) **right before** a run covers the human-touch prefix,
and the long tail rides credentials immune to the cadence.

---

## 9. Authentication mechanics: OOB is deprecated; Device flow is the successor

- The "paste this URL in a browser, authenticate, paste the code back" step is the
  **OAuth 2.0 Authorization Code grant via the out-of-band (OOB) redirect**
  (`urn:ietf:wg:oauth:2.0:oob`). The pasted value is the *authorization code*.
  This is what `rbw-gPI` does today.
- **Google has deprecated OOB.** Building further on it is building on sand.
- The modern, non-deprecated, headless-friendly successor is the **OAuth 2.0
  Device Authorization Grant (RFC 8628)** — the "go to this URL, enter this code"
  smart-TV flow. The browser need not be on the box; it need only exist somewhere
  the operator can reach.
- Crucially, the device flow **authenticates a human and produces a token tied to
  that human's identity** — the same identity model as OOB, just a better
  delivery channel. It introduces no new credential type and no authority-bearing
  artifact. Authority still lives in central IAM. It does **not** recreate the
  RBRA mess.

---

## 10. Acceptance in orgs the project does not control

The acid test. In the target model:

- A real human user on *any* org authenticates via the device flow, reauthing
  when their org's policy demands it (a human is present — fine).
- The admin of *their* org grants/revokes the person's roles centrally; the
  project ships no keys.
- The project never needs to weaken the user's org policy, because impersonation
  creates no SA keys — it sidesteps `disableServiceAccountKeyCreation` entirely.

This is why the reorientation matters for the project's success: it is the
difference between "works only where I am the org admin" and "works in any
serious org out of the box."

---

## 11. Open questions (to decide deliberately, feeding ₢BSAAF)

- **Session-policy as a lever vs a constraint.** For the project's own test org,
  what should the Cloud session control be set to, and how is that documented as
  test-rig-only rather than model behavior? (Verify the current
  `scaleinvariant.org` setting — it lives in the Workspace Admin console.)
- **One human credential vs anything per-role.** Confirm the single-identity +
  impersonation-pointer shape, and what (if anything) the per-role RBRA file
  still holds once it carries no secret.
- **Does Payor stay distinct**, or does it become "a human granted the payor
  role" under the same model?
- **Coexistence & migration.** How RBRA-keyfile mode and impersonation mode
  coexist during transition (the 2026-04-27 memo's mode field), and whether
  keyfile mode is retained only for the consumer/bootstrap case.
- **The synthetic-human CI emplacement** mechanics, and where the residual
  long-lived-secret risk is documented and bounded.
- **Probe / handbook / regime surfaces** that assume keyfiles
  (RBSAJ/RBSAO/RBSAV, onboarding tracks, `rbw-gPR`'s stale "6-month idle" text).
- **The gauntlet coupling.** Any pace that levies a fresh depot (the full
  gauntlet) re-hits the key wall on the new project; skirmish (standing depot)
  does not. Regression during burn-down should lean on skirmish.

---

## 12. The one-line version

The org didn't break a setting — it invalidated the assumption the credential
model was built on. RBRA keyfiles are the consumer/bootstrap mode. The org-native
mode is: a human proves identity (device flow), the project holds no role
secrets, roles are reached by impersonation, and authority is granted and revoked
centrally in IAM. Reauth is not the obstacle; it is the compass that forces the
model to be honest.

---

## 13. Resolutions (decided)

Decided answers to §11, banked as they settle. Implementation heat TBD.

### R1 — Payor stays distinct; the three lower tiers unify (resolves §11 "Does Payor stay distinct?")

> **Superseded in part by R2:** R1's impersonation / `tokenCreator`-on-role-SA
> mechanism is replaced by direct human capability grants. R1's durable
> conclusions survive — Payor as ownership root, the erased credential axis, central
> revoke, org/no-org consistency; only "humans impersonate role SAs" is dropped.

The keyless reorientation erases the *credential* axis that today's specs use to
set Payor apart (RBSGS: "the Payor stands apart"; OAuth vs RBRA keyfile). Once
Governor/Director/Retriever are reached by impersonation, all four tiers share the
human-OAuth front door — OAuth-vs-keyfile distinguishes nothing.

What remains distinguishing Payor is non-credential and durable:

- **Bootstrap root** — Payor's authority is project ownership, with no API path to
  bootstrap (RBSPE); the impersonation chain must bottom out on an authority that
  is not itself an impersonation grant. Payor is that floor.
- **Scope** — multi-depot + billing, vs Governor's single depot.

Collapse statement: **Governor/Director/Retriever become one primitive — "a human
granted `roles/iam.serviceAccountTokenCreator` on a role SA by the tier above."
Payor is the axiom the recursion bottoms out on, authorized by ownership.**

Verb redefinition implied:

- **mantle** (Payor→Governor): project-owner-OAuth human grants `user:X`
  token-creator on the governor SA (was: create SA + mint downloadable key).
- **invest** (Governor→Dir/Ret): human impersonating the governor SA (which keeps
  owner-on-depot, RBSGM) grants `user:Y` token-creator on the dir/ret SA.

Role SA principals persist (§5 "SA principals preserved"); only the human reach
changes from keyfile to impersonation.

Org/no-org consistent: project ownership is the root in both worlds. Membership
eligibility differs only via `iam.allowedPolicyMemberDomains` (an org may restrict
designees to its own domain).

### R2 — Roles are direct human capability grants; SAs survive only as runtime identities (revises R1's mechanism)

A role is a named set of IAM capabilities granted DIRECTLY to a human (`user:…`) on
the depot. Humans call APIs as themselves, carrying the capabilities the org
granted. No human ever impersonates a per-role service account.

What survives from R1 unchanged: Payor is the ownership-rooted bootstrap floor; the
OAuth-vs-keyfile credential axis is erased (all humans use one browser/device
login); revocation is central (delete the human's grant); audit names the human —
now directly, not via `delegationInfo`; org/no-org consistent via project ownership
as the universal root.

Verified separation that makes this sound (`rbfd_FoundryDirectorBuild.sh:812`,
RBSDE-depot_levy):

- Builds never ran as director. They run as **mason**, a per-depot runtime SA whose
  email is derived from depot config (`RBGD_MASON_EMAIL`). Director only submits
  builds and pushes the build-context pouch.
- Every director permission is `user:`-grantable — `cloudbuild.builds.editor`,
  `viewer`, `cloudbuild.workerPoolUser`, `serviceAccountUser` on mason (actAs),
  `artifactregistry.repoAdmin`. Retriever = `artifactregistry.reader` alone.
- Mason is impersonated only by Google's Cloud Build service agent
  (machine-to-machine), never by a human.

Consequences:

- Service accounts retreat to pure runtime plumbing (mason), addressed via
  depot-derived config, never held or worn by a human.
- Zero per-role files. The only on-disk human credential is the one OAuth login.
  The role-SA addressing/naming question — and the Governor timestamp — evaporates:
  there is no per-role SA a human names. This is the maximal form of §6 dissolution.
- Verb redefinition: mantle (Payor→Governor) = Payor grants a human the Governor
  capability-set (project `setIamPolicy`). invest (Governor→Dir/Ret) = Governor
  grants a human the Director/Retriever capability-set. Both are IAM grants naming
  `user:…`, no SA minting.
- Trade-off accepted: standing privilege rather than just-in-time elevation. Bounded
  by org reauth cadence; strictly better than today's durable keyfiles; JIT is
  addable later without re-architecture.

Resolves §11 "one human credential vs anything per-role": one login, nothing
per-role at all.

### R3 — Three credential models are tiers on one axis; MVP ships RBRA; the keyless successor is deferred and unchosen

The option-map (not a committed ladder). Three ways an actor proves identity to
reach depot capabilities, along one axis:

1. **RBRA keyfile** — consumer / bootstrap. Works only where SA-key creation is
   permitted (no-org, or an org that allows it). Already built. **MVP ships this.**
2. **Direct Google grants (R2)** — org baseline. Keyless. Users need *a*
   Google account (a free gmail suffices; they do not join the owner's org). Natural
   fit for Google-Workspace orgs, whose IdP already is Google.
3. **Federation** — org-only (federation pools are org-level). Users authenticate via
   an external OIDC/SAML IdP; no Google account; keyless. Opt-in / candidate paid
   tier. IdP is commoditized by *standards*, not vendor: GCP federation accepts any
   compliant OIDC/SAML issuer; self-hostable (Dex/Keycloak) or GCP Identity Platform
   as an in-house IdP. Abstraction boundary: "any OIDC/SAML issuer with stable
   subject claims."

These are tiers, not alternatives. **The real cost is multi-mode coexistence**, not
any single tier — so the judicious frame is "which one keyless successor eventually
*replaces* RBRA," run one mode at a time, not an accreting stack.

Decision: MVP ships RBRA (no rework). The keyless successor is **deferred and
unchosen** — do not commit build order while the complexity budget is tight. Bank
the map, choose the tier when org adoption actually demands keyless.

Note: this federation is *ingress* identity (operators reaching the depot), distinct
from the roadmap's envoy entry, which uses federation for *egress* (GCP→AWS
cross-post). Same mechanism family, opposite direction — keep them unconflated.

### R4 — Synthetic-human CI emplacement and test-org session policy (resolves §11 "synthetic-human CI emplacement" and "session-policy as a lever vs constraint")

> **SUPERSEDED (2026-06-09) by `Memos/memo-20260609-federation-canon.md` D2.** The operator
> decided a human is always present to start runs, so the federation tier persists **no refresh
> token** — the caged test-rig residual below is **not built**. The tradeoff (no unattended
> scheduled federation runs) and its revisit-trigger live in the canon. R4's reasoning is retained
> as derivation of record.

The two test-rig questions resolve together. A federated operator (Workforce
Identity Federation) authenticates by device flow; the resulting Google session
is capped at **12 hours** — a hard GCP limit, even on an org you own. So
month-scale unattended test machines are *not* a long session: they are a
**persisted IdP refresh token** that the machine silently re-exchanges every
≤12 h, headless. The human touch happens once, when the token is minted.

Decision:

- **Shipped human roles persist no refresh token** — device flow per session,
  reauth on the operator-org's own cadence. This is the org-native ideal.
- **The project's own test rig** (firewalled machines, project-administered test
  org) gets a **test-only enrollment** that persists one refresh token per
  machine — the single acknowledged durable-secret residual (§8). Set the test
  org's Workforce session to its 12 h maximum; the month-span comes from the
  refresh token, not the session.
- **Not security-breaking:** a federated/OAuth refresh token is not an SA key
  (org-legal, no policy override needed), is centrally revocable (disable the
  principal at the IdP — kills every machine at once), is capability-scoped, and
  is physically contained behind the firewall.
- This is a **federation-era** mechanism. Under MVP/keyfile the same durable
  test-credential need is met by the SA key itself (where key creation is
  permitted). Two eras, one need.

Session policy is therefore a lever pulled only on the org you administer (the
test rig), documented as test-rig-only because end users always get *their* org's
cadence — never a project-set value.

### R5 — Gauntlet coupling: regression leans on skirmish (resolves §11 "gauntlet coupling")

Independent of credential tier. A full-gauntlet run levies a fresh depot, which
re-hits `disableServiceAccountKeyCreation` on a secure-by-default org **even in
RBRA/keyfile mode**. Skirmish (standing depot) does not re-levy and does not hit
the wall.

Decision: burn-down and routine regression **lean on skirmish**; full-gauntlet
fresh-depot levies run only on an account that permits SA-key creation (no-org,
or an org with the override). This is an operational testing posture, live for
MVP — not contingent on any keyless work.

### §11 disposition (closure)

All seven §11 questions are now decided or deliberately retired:

| §11 question | Disposition |
|---|---|
| One human credential vs per-role | Resolved — R2 |
| Does Payor stay distinct | Resolved — R1 / R2 |
| Coexistence & migration | Resolved — R3 (mode-enum migration; see the credential-models memo) |
| Synthetic-human CI emplacement | Resolved — R4 |
| Session-policy lever vs constraint | Resolved — R4 |
| Gauntlet coupling | Resolved — R5 |
| Probe / handbook / regime keyfile surfaces (RBSAJ/RBSAO/RBSAV, onboarding, `rbw-gPR`) | **Retired** — R3 ships RBRA for MVP, so these surfaces stay correct as-is; no MVP rework. Re-decided if/when the keyless tier is chosen; the revisit-trigger lives in the RBSHR "Operator federation" roadmap entry. |

The federation tier's mechanism — and the credential mode-enum and cult-verb
shape — are recorded separately in
`memo-20260527-operator-credential-models.md`.
