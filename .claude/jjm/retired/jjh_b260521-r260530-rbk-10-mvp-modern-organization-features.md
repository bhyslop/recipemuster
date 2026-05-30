# Heat Trophy: rbk-10-mvp-modern-organization-features

**Firemark:** ₣BS
**Created:** 260521
**Retired:** 260530
**Status:** retired

## Paddock

## Shape

This heat exists because RBK's credential and resource model was designed against an **org-less** GCP account (the original gmail payor) and breaks under a **secure-by-default organization**. Google enforces a security-baseline org-policy bundle automatically on every organization created on/after 2024-05-03; scaleinvariant.org is such an org, while org-less accounts and older orgs never see it. Moving the payor under the org is what activated the friction — a structural mismatch, not incidental breakage. The depot *infrastructure* (project, pools, builds, GAR, bucket) levies cleanly under the org; the conflict is confined to the operational-role credential layer.

## Constraint bundle vs RBK

The secure-by-default managed constraints and their RBK bearing:

- `iam.managed.disableServiceAccountKeyCreation` — confirmed blocker. RBRA credentials are downloadable SA keys; governor mantle and retriever/director invest each generate one, refused HTTP 400 "Key creation is not allowed."
- `iam.allowedPolicyMemberDomains` — probable. Restricts IAM-grant members to org-domain identities; RBK grants to SAs and Google-managed agents need auditing.
- `iam.managed.disableServiceAccountKeyUpload` — related family; RBK creates rather than uploads, likely clear.
- `iam.automaticIamGrantsForDefaultServiceAccounts` — verify RBK never relies on default-SA Editor.
- `storage.uniformBucketLevelAccess` — depot bucket already created under it; confirm no per-object ACL use.
- `essentialcontacts.managed.allowedContactDomains`, `compute.managed.restrictProtocolForwardingCreationForTypes` — not exercised by RBK; low risk.

## The strategic fork (undecided — why this heat is stabled)

1. Override path — project-scoped org-policy exceptions for the biting constraints. Unblocks fast. Costs: needs Organization Policy Administrator (owner insufficient); managed constraints' project-level override of an inherited org policy is reportedly stubborn (managed-vs-legacy double-lock). Leaves RBK dependent on long-lived SA keys, against the platform's direction.

2. Identity / keyless path — re-express retriever and director as Google identities so no downloadable SA key is ever created. This *dissolves* the confirmed blocker rather than overriding it: the constraint forbids key creation, and this path creates none. A full design already exists in `Memos/memo-20260427-google-native-human-auth.md` (Approach C, recommended): browser OAuth reusing the existing payor machinery, then `iamcredentials …:generateAccessToken` impersonation; the human-to-role binding is a Google email on the SA's IAM policy (`roles/iam.serviceAccountTokenCreator`, member `user:...`). The SA principal, role taxonomy, and audit identity are preserved; distribution dissolves; revocation is one IAM-binding deletion. The memo also rejects the gcloud-SDK path (Approach B) for RBK's minimal-stack (curl/openssl/jq) reasons.

   Gap the memo does not close: it covers human-driven operations only and explicitly leaves headless paths (Cloud Build itself, automated fixtures, Ifrit scenarios, the gauntlet) on keyfiles-or-workload-identity. Under a secure-by-default org those automated paths stay blocked, so this heat must add a **workload-identity-federation** story for them. So ₣BS scope = memo Approach C (human-driven roles) + a WIF answer (headless/automated), not the memo alone.

## Constraints

- Bash paces read **BCG** (Bash Console Guide) first. `rbgp_Payor.sh` is a complex BCG-compliant module; do not write bash against it without the guide.

## What done looks like

A decision on override vs identity-keyless (plausibly override-to-unblock now, keyless as the durable target); an audit verdict on each bundle constraint against RBK flows; a workload-identity-federation answer for headless paths; and, for the chosen path, the concrete changes to credential install, regime files, and the RBS* depot / SA-invest specs. Until that decision lands, this heat holds shape only.

## Paces

### org-affiliated-account-reorientation (₢BSAAF) [complete]

**[260527-0813] complete**

## Character

Interactive design conversation requiring operator judgment — do NOT implement.
Product is a *decision record*: a decided answer to every §11 open question,
banked durably, independent of which heat implements them. The operator has not
committed this heat to implementation — decisions may seed a future heat. The
pace wraps when §11 is fully decided, not when downstream paces are cut.

## Substrate

- memo-20260522-org-affiliated-credential-reorientation.md — §11 open questions
  and the accumulating Resolutions section (decisions land there, not in this docket).
- memo-20260427-google-native-human-auth.md — impersonation mechanism detail.

## Goal

Work §11 with the operator; record each decided answer as a Resolution in the
2026-05-22 memo. Output: §11 fully decided and recorded.

## Constraint

Produces decisions, not code. Decisions live in the memo (durable design
substance), pointed at — never restated — from paddock or docket.

**[260522-1249] rough**

## Character

Interactive design conversation requiring operator judgment — do NOT implement.
Product is a *decision record*: a decided answer to every §11 open question,
banked durably, independent of which heat implements them. The operator has not
committed this heat to implementation — decisions may seed a future heat. The
pace wraps when §11 is fully decided, not when downstream paces are cut.

## Substrate

- memo-20260522-org-affiliated-credential-reorientation.md — §11 open questions
  and the accumulating Resolutions section (decisions land there, not in this docket).
- memo-20260427-google-native-human-auth.md — impersonation mechanism detail.

## Goal

Work §11 with the operator; record each decided answer as a Resolution in the
2026-05-22 memo. Output: §11 fully decided and recorded.

## Constraint

Produces decisions, not code. Decisions live in the memo (durable design
substance), pointed at — never restated — from paddock or docket.

**[260522-0933] rough**

## Character

Interactive design conversation requiring operator judgment — do NOT implement.
A strategic reorientation that reshapes this heat's tactical paces before they
are cut; its product is an agreed direction, not code.

## Trigger

The active Payor moved from a consumer Google account (no org) to an
org-affiliated Workspace identity. That is not a config tweak — it invalidated
the assumption the project's credential model was built on (freely-minted
service-account keyfiles). Surfaced live during the ₣BM skirmish first-run.

## Substrate — read both before reshaping anything

- `Memos/memo-20260522-org-affiliated-credential-reorientation.md` — the spine of
  this heat: why org-affiliation makes impersonation necessary rather than a
  future convenience, the reauth-as-compass reframe, the RBRA → identity-pointer
  recast, the human-driven-vs-unattended axis, the synthetic-human CI corner, the
  OOB → Device-Authorization-Grant mechanics, the session evidence, and a
  consolidated open-questions list (§11). Start here.
- `Memos/memo-20260427-google-native-human-auth.md` — the impersonation mechanism
  substrate (generateAccessToken, the IAM-binding model, storage shape, threat
  model). The 2026-05-22 memo supersedes its "future convenience" framing but not
  its mechanism detail.

Cite both; do not re-paraphrase them here — keeping the design substance in the
memos avoids a second, diverging copy that drifts.

## Goal

Work through the 2026-05-22 memo's open questions (§11) with the operator, decide
the design direction, and reshape this heat's downstream tactical paces and the
paddock to match. Output: an agreed direction written back into the paddock.

## Constraint

Produces understanding and a reshaped plan, not code. Implementation belongs to
the downstream paces, governed by this pace's conclusions.

**[260522-0812] rough**

## Character

Interactive design conversation requiring operator judgment — do NOT
implement. A strategic reorientation that should reshape this heat's tactical
paces before they are cut; its product is an agreed direction, not code.

## Trigger

Surfaced live while first-running the ₣BM skirmish suite. The project's active
Payor moved from a **consumer** Google account (`bhyslop@gmail.com`, no org) to
an **org-affiliated** Workspace identity (`bhyslop@scaleinvariant.org`, org
`247899326218`). The operator's read: this is a major use-style change, not a
one-off config tweak, and the project must be reoriented to treat
org-affiliated accounts as a first-class mode.

## What we proved this session (the durable reminder)

Org-affiliated identities inherit secure-by-default posture the prior consumer
account never had — two distinct behaviors, two distinct layers:

- **Reauth cadence.** The org's session/reauth policy refuses the payor OAuth
  refresh token after a short window (a ~22h-old token returned `invalid_rapt`;
  `rbw-gPI` re-auth cleared it). `rbw-gPR` is stale here — its text assumes the
  consumer "6-month idle expiry" model, which no longer describes reality.
- **Key-creation block.** `iam.disableServiceAccountKeyCreation` is `enforced`
  by default on the standing depot, so governor/retriever/director SA-key
  creation will be refused — the wall the mantle would hit next.
- The payor **lacks** `orgpolicy.policy.set` but **holds**
  `resourcemanager.organizations.setIamPolicy` (so it could self-grant
  policyAdmin). The standing depot is ACTIVE under the org; no levy needed.

## The memo this fold pulls in

`Memos/memo-20260427-google-native-human-auth.md` — a 2026-04-27 design
exploration proposing the project move off static RBRA service-account keyfiles
(Governor / Director / Retriever) toward formal Google identities via
**REST-only browser OAuth + SA impersonation** (Approach C; gcloud-SDK Approach
B dismissed for dependency surface; keyfile Approach A retained as coexisting).
Mechanism: user authenticates once in a browser, refresh token cached locally;
per operation, mint a user access token then call `generateAccessToken` on
`iamcredentials.googleapis.com` to obtain a short-lived SA access token. The
SA principal is preserved — RBSCIG contracts, RBGI machinery, role taxonomy,
audit identity all unchanged. Distribution dissolves (access becomes an IAM
binding); revocation becomes a single binding deletion; audit logs gain
`delegationInfo`. Read it in full before reshaping downstream paces — its
Mechanism Detail, Storage shape, IAM binding model, Coexistence, Threat Model,
and Open Questions sections are the design substrate and should NOT be
re-paraphrased here.

## How the org finding reframes the memo (the key turn)

The memo framed keyfile→impersonation as an **adoption/convenience** play — its
two named frictions are distribution logistics and revocation hygiene, and its
"Trigger to Revisit" is demand-driven (first external-user request, or first
time keyfile distribution *materially* blocks adoption). The org-policy finding
is a **third trigger the memo never anticipated**: on an org-affiliated depot,
`iam.disableServiceAccountKeyCreation` is enforced *by default*, so the RBRA
keyfile pattern is not "theoretically frictionful" — it is **structurally
unavailable** unless the operator deliberately weakens org policy. That flips
the posture: impersonation moves from future nice-to-have toward the
**org-native path**, while keyfile mode becomes the thing requiring a policy
override to keep alive.

## Issues to work through (none resolved — captured for a fresh head)

1. **Is impersonation load-bearing for the org mode, or still optional?** The
   memo's coexistence section keeps keyfiles for headless paths (Cloud Build,
   automated fixtures, Ifrit-style scenarios). Those also run inside the
   depot/org. Open: does the key-creation block hit them too, or do they have a
   different escape (workload identity, build-time SA)? The answer decides
   whether impersonation is "the human path" or "the only path."

2. **The memo predates the reauth-cadence finding and collides with it.**
   Approach C is literally "replicate the Payor OAuth pattern" and leans *harder*
   on OAuth refresh tokens — yet this session proved the org's session policy
   *refuses* aged refresh tokens (`invalid_rapt` at ~22h). The very primitive the
   memo calls "already proven" is the one the org now constrains. The
   impersonation flow inherits the same reauth cadence; the memo's "refresh token
   cached locally, mint on demand" story needs that caveat.

3. **`rbw-gPR` (PayorRefresh) is stale.** Its "6-month idle expiry" text
   describes the consumer model, not the org's short reauth window. Whatever
   direction we land, this surface needs rewriting — and it interacts with #2,
   since impersonation onboarding would carry the same reauth assumption forward
   if copied verbatim.

4. **Org-policy override ownership.** The payor holds
   `resourcemanager.organizations.setIamPolicy` (could self-grant policyAdmin)
   but lacks `orgpolicy.policy.set`. If keyfile mode is to survive on org
   depots, *someone* must own relaxing `disableServiceAccountKeyCreation` — and
   where that override is surfaced, owned, and documented (establish / levy /
   onboarding) is part of the org-mode design, not a side note.

5. **Memo's own Open Questions remain open and feed downstream paces:** storage
   location (regime-tree vs `$HOME`), OAuth client scope (project-wide vs
   per-Manor), onboarding handbook track shape (parallel RBHO* vs unified
   branching), probe coexistence / mode dispatch (RBSAJ, RBSAO, RBSAV), and mode
   field naming/placement. These live in the memo; this pace decides which
   become tactical paces in this heat.

## Goal

Decide, honestly and with the operator, how org-affiliated accounts become a
supported use style: where reauth cadence and the key-creation override are
surfaced, owned, and documented across establish / levy / onboarding; whether
the keyfile→impersonation move (per the 2026-04-27 memo) is now load-bearing for
the org mode rather than a future convenience; and whether the tactical
org-policy paces already slated in this heat are the right shape or need
recasting in light of the above. Output: an agreed design direction written back
into the paddock, with the downstream paces reshaped to match.

## Constraint

Produces understanding and a reshaped plan, not code. Implementation belongs to
the downstream paces, governed by this pace's conclusions.

**[260522-0756] rough**

## Character

Interactive design conversation requiring operator judgment — do NOT
implement. A strategic reorientation that should reshape this heat's tactical
paces before they are cut; its product is an agreed direction, not code.

## Trigger

Surfaced live while first-running the ₣BM skirmish suite. The project's active
Payor moved from a **consumer** Google account (`bhyslop@gmail.com`, no org) to
an **org-affiliated** Workspace identity (`bhyslop@scaleinvariant.org`, org
`247899326218`). The operator's read: this is a major use-style change, not a
one-off config tweak, and the project must be reoriented to treat
org-affiliated accounts as a first-class mode.

## What we proved this session (the durable reminder)

Org-affiliated identities inherit secure-by-default posture the prior consumer
account never had — two distinct behaviors, two distinct layers:

- **Reauth cadence.** The org's session/reauth policy refuses the payor OAuth
  refresh token after a short window (a ~22h-old token returned `invalid_rapt`;
  `rbw-gPI` re-auth cleared it). `rbw-gPR` is stale here — its text assumes the
  consumer "6-month idle expiry" model, which no longer describes reality.
- **Key-creation block.** `iam.disableServiceAccountKeyCreation` is `enforced`
  by default on the standing depot, so governor/retriever/director SA-key
  creation will be refused — the wall the mantle would hit next.
- The payor **lacks** `orgpolicy.policy.set` but **holds**
  `resourcemanager.organizations.setIamPolicy` (so it could self-grant
  policyAdmin). The standing depot is ACTIVE under the org; no levy needed.

## Goal

Decide, honestly and with the operator, how org-affiliated accounts become a
supported use style: where reauth cadence and the key-creation override are
surfaced, owned, and documented across establish / levy / onboarding — and
whether the tactical org-policy paces already slated in this heat are the right
shape or need recasting in light of the above. Output: an agreed design
direction written back into the paddock, with the downstream paces reshaped to
match.

## Constraint

Produces understanding and a reshaped plan, not code. Implementation belongs to
the downstream paces, governed by this pace's conclusions.

### delete-dead-rbrp-parent-fields (₢BSAAA) [complete]

**[260527-1019] complete**

## Character
Mechanical cleanup; confirm dead-ness, then delete.

`RBRP_PARENT_TYPE` and `RBRP_PARENT_ID` are defined in `rbrp.env` and string-enrolled in `rbrp_regime.sh`, but nothing reads them — depot project creation is parentless and GCP auto-parents under the org. `RBSDE-depot_levy.adoc`'s create-project step describes a `parent:` it never actually sets — a spec/code divergence.

Done: both fields removed from `rbrp.env` and the rbrp regime validator; the RBSDE create-project step rewritten to describe the parentless create + GCP auto-parenting reality. Before deleting, `grep RBRP_PARENT` across the tree confirms zero non-definition readers.

**[260521-1606] rough**

## Character
Mechanical cleanup; confirm dead-ness, then delete.

`RBRP_PARENT_TYPE` and `RBRP_PARENT_ID` are defined in `rbrp.env` and string-enrolled in `rbrp_regime.sh`, but nothing reads them — depot project creation is parentless and GCP auto-parents under the org. `RBSDE-depot_levy.adoc`'s create-project step describes a `parent:` it never actually sets — a spec/code divergence.

Done: both fields removed from `rbrp.env` and the rbrp regime validator; the RBSDE create-project step rewritten to describe the parentless create + GCP auto-parenting reality. Before deleting, `grep RBRP_PARENT` across the tree confirms zero non-definition readers.

### org-detection-from-live-parent (₢BSAAB) [abandoned]

**[260523-1505] abandoned**

## Character
Small helper; the reliable signal is the live API, never config.

The levy preflight and the override step both need to know whether the depot project is under an organization (and which one). The only trustworthy source is the project's live `parent` from the Resource Manager API — never a regime field (the just-deleted `RBRP_PARENT_TYPE` proved config diverges from reality: it read `none` while GCP auto-parented under the org).

Done: a helper the levy uses, post-create, to classify "under org (capture org id)" vs "no org" from the created project's actual parent. Consumed by the preflight and override paces that follow.

**[260521-1606] rough**

## Character
Small helper; the reliable signal is the live API, never config.

The levy preflight and the override step both need to know whether the depot project is under an organization (and which one). The only trustworthy source is the project's live `parent` from the Resource Manager API — never a regime field (the just-deleted `RBRP_PARENT_TYPE` proved config diverges from reality: it read `none` while GCP auto-parented under the org).

Done: a helper the levy uses, post-create, to classify "under org (capture org id)" vs "no org" from the created project's actual parent. Consumed by the preflight and override paces that follow.

### establish-grants-payor-policy-authority (₢BSAAC) [abandoned]

**[260523-1505] abandoned**

## Character
Handbook + spec; documents a manual one-time operator step. Org-affiliated payors only.

Under a secure-by-default org, the payor needs `roles/orgpolicy.policyAdmin` to set the per-project key-creation override the levy performs. Org owners do NOT hold this by default — it must be granted deliberately. No-org payors never need it.

Done: the payor-establish handbook (`RBHPE` / `rbhpe_establish.sh`) gains a step under its existing org-affiliation (advanced) branch directing the operator to grant their identity Organization Policy Administrator on the org; `RBSPE-payor_establish.adoc` updated to match. The no-org path is untouched.

**[260521-1606] rough**

## Character
Handbook + spec; documents a manual one-time operator step. Org-affiliated payors only.

Under a secure-by-default org, the payor needs `roles/orgpolicy.policyAdmin` to set the per-project key-creation override the levy performs. Org owners do NOT hold this by default — it must be granted deliberately. No-org payors never need it.

Done: the payor-establish handbook (`RBHPE` / `rbhpe_establish.sh`) gains a step under its existing org-affiliation (advanced) branch directing the operator to grant their identity Organization Policy Administrator on the org; `RBSPE-payor_establish.adoc` updated to match. The no-org path is untouched.

### levy-preflight-confirms-policy-authority (₢BSAAD) [abandoned]

**[260523-1505] abandoned**

## Character
Spec + impl; fail-fast guard in the levy's cheapest-first ordering. Org-affiliated payors only.

If the depot is org-affiliated, the levy must confirm the payor holds org-policy-set authority BEFORE creating any resource — otherwise it half-builds a depot it cannot finish (the exact failure hit this session: project created, governor key generation refused). Org-ness comes from the live-parent helper; the permission via `testIamPermissions` on the org.

Done: `RBSDE-depot_levy.adoc` gains a preflight step (after authenticate-as-payor, before create-project) that, for org-affiliated payors, requires the set-policy permission and otherwise dies with a clear message pointing at the establish-time grant. No-org payors skip it.

**[260521-1606] rough**

## Character
Spec + impl; fail-fast guard in the levy's cheapest-first ordering. Org-affiliated payors only.

If the depot is org-affiliated, the levy must confirm the payor holds org-policy-set authority BEFORE creating any resource — otherwise it half-builds a depot it cannot finish (the exact failure hit this session: project created, governor key generation refused). Org-ness comes from the live-parent helper; the permission via `testIamPermissions` on the org.

Done: `RBSDE-depot_levy.adoc` gains a preflight step (after authenticate-as-payor, before create-project) that, for org-affiliated payors, requires the set-policy permission and otherwise dies with a clear message pointing at the establish-time grant. No-org payors skip it.

### levy-sets-project-scoped-key-override (₢BSAAE) [abandoned]

**[260523-1505] abandoned**

## Character
Spec + impl; the payoff step. The propagation wait is load-bearing. Org-affiliated payors only.

For org-affiliated depots, after the project exists the levy sets a project-scoped override clearing `iam.disableServiceAccountKeyCreation`, so governor/retriever/director key creation succeeds. Verified this session: a v1 `setOrgPolicy` `enforced:false` at project scope works, and key creation unblocks roughly a minute later. The step MUST wait out that propagation (mirroring the levy's existing IAM-propagation gate) before any SA-key-dependent step, or the subsequent governor mantle races the lag and fails.

Done: `RBSDE-depot_levy.adoc` gains the override step (org branch only) with an explicit propagation wait/verify before proceeding. No-org branch unaffected. Pairs with the establish grant and the preflight that guards it.

**[260521-1606] rough**

## Character
Spec + impl; the payoff step. The propagation wait is load-bearing. Org-affiliated payors only.

For org-affiliated depots, after the project exists the levy sets a project-scoped override clearing `iam.disableServiceAccountKeyCreation`, so governor/retriever/director key creation succeeds. Verified this session: a v1 `setOrgPolicy` `enforced:false` at project scope works, and key creation unblocks roughly a minute later. The step MUST wait out that propagation (mirroring the levy's existing IAM-propagation gate) before any SA-key-dependent step, or the subsequent governor mantle races the lag and fails.

Done: `RBSDE-depot_levy.adoc` gains the override step (org branch only) with an explicit propagation wait/verify before proceeding. No-org branch unaffected. Pairs with the establish grant and the preflight that guards it.

### explode-rbgu-grab-bag (₢BSAAG) [complete]

**[260530-0912] complete**

## Character
Intricate but mechanical — a behavior-preserving move/rename/resite sweep, plus one operator-approved scope addition: deletion of two vestigial pre-gateway clis (see OPEN ITEM — now RESOLVED). No new logic. The map below was derived under Opus 4.7, then critically re-reviewed and operator-approved under Opus 4.8; two structural changes resulted (rbuj merged into rbuh; transient predicate homed in rbgo). This revised map is the authoritative spec — do NOT re-derive or re-split. Detail exceeds normal docket discipline by explicit operator allowance because this pace executes imminently. Re-verify the as-discovered site lists with the grep recipes at mount (file paths are durable; line numbers are not — none are cited here).

## Goal
Dissolve `rbgu_Utility.sh` (the "Google Utility" grab-bag) into focused modules whose prefix matches what the code actually is, dropping the now-redundant infix. `rbgu` disappears entirely (`grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing).

## Revised frozen map (3 new modules; rbgi + rbgo absorb the rest; deletions below)

rbuh — NEW, Utility/HTTP + the JSON/string + temp-file machinery it produces and consumes (11 fns). Owns the shared temp-file constants. This is the rbuh+rbuj MERGE: the prior map split them, but they are mutually dependent (rbuh's require-ok reads JSON via the json-field capture; 3 of the 4 JSON fns read rbuh's temp-file constants) and always co-sourced/co-kindled — splitting was non-load-bearing. One module:
- rbgu_http_request            -> rbuh_request
- rbgu_http_json               -> rbuh_json
- rbgu_http_require_ok         -> rbuh_require_ok
- rbgu_http_code_capture       -> rbuh_code_capture
- rbgu_poll_until_ok           -> rbuh_poll_until_ok
- rbgu_poll_until_gone         -> rbuh_poll_until_gone
- rbgu_json_valid_predicate    -> rbuh_json_valid_predicate
- rbgu_json_field_capture      -> rbuh_json_field_capture
- rbgu_jq_file_to_file_ok      -> rbuh_jq_file_to_file_ok
- rbgu_urlencode_capture       -> rbuh_urlencode_capture
- (the transient predicate does NOT go here — see rbgo below)

Note: the original `rbgu_http_json`/`rbgu_http_request` define BOTH the retry-looping JSON helper and the BCG-conformant single-shot primitive — both move to rbuh as named.

rbge — NEW, GCP REST / LRO over rbuh (4 fns):
- rbgu_http_json_lro_ok        -> rbge_lro_ok
- rbgu_newly_created_delay     -> rbge_newly_created_delay
- rbgu_api_enable              -> rbge_api_enable
- rbgu_error_message_capture   -> rbge_error_message_capture

rba — NEW, Auth (mode-neutral; depends on rbgo, touches NO temp-file machinery — verified) (5 fns):
- rbgu_get_governor_token_capture -> rba_get_governor_token_capture
- rbgu_authenticate_role_capture  -> rba_authenticate_role_capture
- rbgu_rbra_load                  -> rba_rbra_load
- rbgu_rbro_load                  -> rba_rbro_load
- rbgu_extract_json_to_rbra       -> rba_extract_json_to_rbra

rbgi — EXISTING (IAM), gains 3 fns:
- rbgu_jq_add_member_to_role_capture -> rbgi_jq_add_member_to_role_capture
- rbgu_provision_service_agent       -> rbgi_provision_service_agent
- rbgu_sa_email_capture              -> rbgi_sa_email_capture

rbgo — EXISTING (OAuth), gains 1 STATELESS fn (the transient-curl-exit predicate):
- rbgu_curl_status_is_transient_predicate -> rbgo_curl_status_is_transient_predicate
  Rationale (operator-approved): this is the one stateless, sentinel-free function, consumed by BOTH rbgo's own curl-retry path AND rbuh's retry loop. rbgo is the lowest module that performs curl and is a kindle-dependency of rbuh, so it is the lowest common ancestor that can hold a function with the dependency arrow pointing only downward. It sits beside rbgo's existing stateless cross-module helpers (the rbgo_base64_* family, explicitly documented "safe to call from any module regardless of kindle order"). rbgc was rejected: it is constants-only, no home for methods.

DELETE (operator-approved — do NOT relocate):
- rbgu_write_vanilla_json — dead code, no callers anywhere.
- ZRBGU_EMPTY_JSON — dead code, written in kindle, never read (every other module owns its own).
- rbgb_cli.sh — vestigial pre-gateway cli (see OPEN ITEM RESOLVED). Delete the whole file.
- rbga_cli.sh — vestigial pre-gateway cli (see OPEN ITEM RESOLVED). Delete the whole file.

## Shared temp-file machinery — rbuh owns it
The infix protocol: rbuh_json writes `${PREFIX}${infix}${POSTFIX}`; the capture, require-ok, and the moved rbgi/rbge functions read those files back by infix. rbuh is the owner:
- ZRBGU_PREFIX -> ZRBUH_PREFIX; ZRBGU_POSTFIX_JSON -> ZRBUH_POSTFIX_JSON; ZRBGU_POSTFIX_CODE -> ZRBUH_POSTFIX_CODE (defined in zrbuh_kindle).
- rbge and the moved rbgi functions reference ZRBUH_* -> their kindles assert zrbuh_sentinel.

## External constant-leak sites (NOT module-internal)
Five files reach `${ZRBGU_PREFIX}${infix}${ZRBGU_POSTFIX_JSON}` directly; rewrite to ZRBUH_* and ensure rbuh is kindled/sourced on those paths: rbgb_Buckets.sh, rbgg_Governor.sh, rbfl_FoundryLedger.sh, rbfc_FoundryCore.sh, rbgp_Payor.sh. (Recipe: `grep 'ZRBGU_' Tools/rbk`.)

## Cross-module internal call rewrites
Moved functions call each other; rename every internal reference per the map. Notably: rbuh_require_ok calls rbuh_json_field_capture (now INTRA-module after the merge); rbge_* call rbuh_*; rbge_api_enable calls rbge_lro_ok + rbge_error_message_capture; rbgi_provision_service_agent calls rbuh_*. rbuh's retry loop and rbgo's own curl path both call rbgo_curl_status_is_transient_predicate (DOWNWARD into rbgo). rba functions call rbgo_get_token_capture (unchanged); rba_rbro_load sources rbro_regime.sh via `${BASH_SOURCE[0]%/*}` (same dir — resolves correctly from rba's file).

## Kindle / sentinel / sourcing wiring
- Kindle DAG flattens to: rbgc -> rbgo -> rbuh -> rbge. rbuh asserts rbgc + rbgo sentinels and validates the RBGC eventual-consistency consistency vars. rbge asserts rbuh. rbgi (existing) asserts rbuh (was rbgu). rba asserts rbgo only. rbgo gains the predicate but its dependency set is UNCHANGED (still rbgc + burd) — the predicate is stateless, no new state. Kindles are single-shot (buc_die on re-kindle); dependents assert via sentinel, they do not kindle deps.
- Sourcing: the 8 furnish-bearing clis that source rbgu_Utility.sh replace it with rbuh, rbge, rba in dependency order (3 lines, not 4 — rbuj is gone). rbgo is already sourced wherever rbgu was (rbgu depended on it), so the predicate resolves with no new source line. (Recipe: `grep -l 'rbgu_Utility.sh' Tools/rbk` — expect 8 surviving cli .sh files plus the acronyms doc, after the 2 vestigial clis are deleted; pre-deletion the grep shows 10.)
- Kindle sites: each `zrbgu_kindle` call -> the three new kindles in dep order (rbuh first, then rbge; rba where auth is used). Sentinel sites: each module-internal `zrbgu_sentinel` -> the new sentinel(s) that site actually depends on (rbuh for temp-file/HTTP/JSON use).
- rbgi sourcing is already covered: the only callers of the moved rbgi functions are rbgp_Payor (provision_service_agent, sa_email) and rbgg_Governor (jq_add_member) — both clis already source rbgi_IAM.sh.

## OPEN ITEM — RESOLVED (operator-approved disposition: delete)
Investigated and closed. rbgb_cli.sh and rbga_cli.sh are vestigial PRE-GATEWAY relics, not merely kindle-less:
- Unreachable: zero references in Tools/ or tt/ — no tabtarget, launcher, or workbench routes to either. Only hits are historical commit-affiliation records in retired JJ heat journals.
- Gateway violation (BCG "CLI as Module Gateway" + Template 2): they top-level-source via `${BASH_SOURCE[0]%/*}`, have NO furnish function, NO kindle calls, and end in `burd_dispatch "$@"` — the pre-BCG dispatch verb. The 8 working clis use `buc_execute «prefix»_ "…" z«prefix»_furnish "$@"`, sourcing + kindling inside furnish. rbgb/rbga predate that pattern.
- Non-functional as written: rbgb_Buckets.sh / rbga_ArtifactRegistry.sh assert zrbgu_sentinel; with nothing kindled, any real dispatch buc_die's at the first sentinel.
Disposition: DELETE both files outright in this pace (operator-approved scope addition). Do NOT source-swap them, do NOT rewrite them into modern gateway form. Their command modules (rbgb_Buckets.sh, rbga_ArtifactRegistry.sh) REMAIN — they are sourced/used by other clis and carry the ZRBGU_ leak-site rewrites above; only the two `_cli.sh` wrappers are deleted.

## Constraints
- Behavior-preserving (for the move/rename/resite core): move + rename + resite only. Temp-file string VALUES are behaviorally irrelevant (ephemeral, written+read within one run via the same constant) — renaming identifiers is safe. The two cli deletions are the one operator-approved exception to behavior-preservation (the files are dead — deleting them removes no live behavior).
- Library-only: no tabtarget / colophon changes. (The deleted clis have no tabtarget routing — confirmed — so this holds.)
- Bash discipline: read BCG before writing modules. New modules mirror rbgu's BCG structure (license header, line-2 `# shellcheck disable=SC2153  # kindle chain - per BCG` for cross-module Z*_ consumers, set -euo pipefail, ZRB**_SOURCED guard, zrb**_kindle + zrb**_sentinel, sectioned bodies). Per BCG "CLI as Module Gateway," kindle graphs are owned by the cli furnish/dispatch path — do not leak kindle outside. rbgp_Payor.sh is complex BCG (highest rbgu_ ref count) — do not edit blind.
- Mint: rbuh confirmed child of family rbu (rbuj retired — never created). rba, rbge terminal. All free.

## Docs to update (in scope)
RBS0-SpecTop.adoc, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc reference rbgu_ symbols; rbk-claude-acronyms.md has the RBGU entry. Update all four: replace renamed symbols, retire the RBGU acronym entry, add rbuh / rba / rbge entries (NOT rbuj), and note rbgo gains the transient predicate. Also correct rbgc_Constants.sh's comment reference to rbgu_http_json (-> rbuh_json) — comment-only, not behavior.

## Execution shape (advisory, not locked)
OPEN ITEM already resolved — go straight to execution. Single parallel agent wave over DISJOINT files (no two agents touch one file). One agent per new module (rbuh, rbge, rba — extract + rename + kindle/sentinel + assigned machinery); one agent owns rbgi_IAM.sh end-to-end (gains 3 fns + rewrites its own rbgu_ call sites); one agent adds the predicate to rbgo + fixes rbgo's self-call; remaining agents partition the pure-caller .sh files + the 4 docs. The two cli deletions (rbgb_cli.sh, rbga_cli.sh) are trivial — orchestrator does them directly. The orchestrator keeps the sourcing/kindle-chain wiring for itself (the linchpin) and verifies. Disjoint files mean no worktree isolation needed.

## What done looks like
rbgu_Utility.sh is gone; the two vestigial clis (rbgb_cli.sh, rbga_cli.sh) are gone; `grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing; every new name resolves to exactly one definition; all callers, sourcing, and the kindle chain updated; the four docs updated; dead code deleted. Verification: qualify (`tt/rbw-tr` — shellcheck + deny-warnings) clean, and the service-tier suite (`tt/rbw-ts.TestSuite.service.sh`) passes — it exercises rba's token path. Operator has confirmed live GCP credentials are available; the service tier is in-scope for the executing agent, not deferred.

## Discovery
- `grep 'rbgu_' Tools/rbk` — call sites and sourcing.
- `grep 'ZRBGU_' Tools/rbk` — temp-file constant leak sites.
- `grep -rn 'zrbgu_kindle\|zrbgu_sentinel' Tools/rbk` — kindle vs sentinel topology (the OPEN ITEM — now resolved — lived here).
- `grep -rln 'rbgb_cli\|rbga_cli' Tools/ tt/` — confirm the two clis remain unrouted before deleting.
- Source under dissolution: Tools/rbk/rbgu_Utility.sh.

**[260529-1752] rough**

## Character
Intricate but mechanical — a behavior-preserving move/rename/resite sweep, plus one operator-approved scope addition: deletion of two vestigial pre-gateway clis (see OPEN ITEM — now RESOLVED). No new logic. The map below was derived under Opus 4.7, then critically re-reviewed and operator-approved under Opus 4.8; two structural changes resulted (rbuj merged into rbuh; transient predicate homed in rbgo). This revised map is the authoritative spec — do NOT re-derive or re-split. Detail exceeds normal docket discipline by explicit operator allowance because this pace executes imminently. Re-verify the as-discovered site lists with the grep recipes at mount (file paths are durable; line numbers are not — none are cited here).

## Goal
Dissolve `rbgu_Utility.sh` (the "Google Utility" grab-bag) into focused modules whose prefix matches what the code actually is, dropping the now-redundant infix. `rbgu` disappears entirely (`grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing).

## Revised frozen map (3 new modules; rbgi + rbgo absorb the rest; deletions below)

rbuh — NEW, Utility/HTTP + the JSON/string + temp-file machinery it produces and consumes (11 fns). Owns the shared temp-file constants. This is the rbuh+rbuj MERGE: the prior map split them, but they are mutually dependent (rbuh's require-ok reads JSON via the json-field capture; 3 of the 4 JSON fns read rbuh's temp-file constants) and always co-sourced/co-kindled — splitting was non-load-bearing. One module:
- rbgu_http_request            -> rbuh_request
- rbgu_http_json               -> rbuh_json
- rbgu_http_require_ok         -> rbuh_require_ok
- rbgu_http_code_capture       -> rbuh_code_capture
- rbgu_poll_until_ok           -> rbuh_poll_until_ok
- rbgu_poll_until_gone         -> rbuh_poll_until_gone
- rbgu_json_valid_predicate    -> rbuh_json_valid_predicate
- rbgu_json_field_capture      -> rbuh_json_field_capture
- rbgu_jq_file_to_file_ok      -> rbuh_jq_file_to_file_ok
- rbgu_urlencode_capture       -> rbuh_urlencode_capture
- (the transient predicate does NOT go here — see rbgo below)

Note: the original `rbgu_http_json`/`rbgu_http_request` define BOTH the retry-looping JSON helper and the BCG-conformant single-shot primitive — both move to rbuh as named.

rbge — NEW, GCP REST / LRO over rbuh (4 fns):
- rbgu_http_json_lro_ok        -> rbge_lro_ok
- rbgu_newly_created_delay     -> rbge_newly_created_delay
- rbgu_api_enable              -> rbge_api_enable
- rbgu_error_message_capture   -> rbge_error_message_capture

rba — NEW, Auth (mode-neutral; depends on rbgo, touches NO temp-file machinery — verified) (5 fns):
- rbgu_get_governor_token_capture -> rba_get_governor_token_capture
- rbgu_authenticate_role_capture  -> rba_authenticate_role_capture
- rbgu_rbra_load                  -> rba_rbra_load
- rbgu_rbro_load                  -> rba_rbro_load
- rbgu_extract_json_to_rbra       -> rba_extract_json_to_rbra

rbgi — EXISTING (IAM), gains 3 fns:
- rbgu_jq_add_member_to_role_capture -> rbgi_jq_add_member_to_role_capture
- rbgu_provision_service_agent       -> rbgi_provision_service_agent
- rbgu_sa_email_capture              -> rbgi_sa_email_capture

rbgo — EXISTING (OAuth), gains 1 STATELESS fn (the transient-curl-exit predicate):
- rbgu_curl_status_is_transient_predicate -> rbgo_curl_status_is_transient_predicate
  Rationale (operator-approved): this is the one stateless, sentinel-free function, consumed by BOTH rbgo's own curl-retry path AND rbuh's retry loop. rbgo is the lowest module that performs curl and is a kindle-dependency of rbuh, so it is the lowest common ancestor that can hold a function with the dependency arrow pointing only downward. It sits beside rbgo's existing stateless cross-module helpers (the rbgo_base64_* family, explicitly documented "safe to call from any module regardless of kindle order"). rbgc was rejected: it is constants-only, no home for methods.

DELETE (operator-approved — do NOT relocate):
- rbgu_write_vanilla_json — dead code, no callers anywhere.
- ZRBGU_EMPTY_JSON — dead code, written in kindle, never read (every other module owns its own).
- rbgb_cli.sh — vestigial pre-gateway cli (see OPEN ITEM RESOLVED). Delete the whole file.
- rbga_cli.sh — vestigial pre-gateway cli (see OPEN ITEM RESOLVED). Delete the whole file.

## Shared temp-file machinery — rbuh owns it
The infix protocol: rbuh_json writes `${PREFIX}${infix}${POSTFIX}`; the capture, require-ok, and the moved rbgi/rbge functions read those files back by infix. rbuh is the owner:
- ZRBGU_PREFIX -> ZRBUH_PREFIX; ZRBGU_POSTFIX_JSON -> ZRBUH_POSTFIX_JSON; ZRBGU_POSTFIX_CODE -> ZRBUH_POSTFIX_CODE (defined in zrbuh_kindle).
- rbge and the moved rbgi functions reference ZRBUH_* -> their kindles assert zrbuh_sentinel.

## External constant-leak sites (NOT module-internal)
Five files reach `${ZRBGU_PREFIX}${infix}${ZRBGU_POSTFIX_JSON}` directly; rewrite to ZRBUH_* and ensure rbuh is kindled/sourced on those paths: rbgb_Buckets.sh, rbgg_Governor.sh, rbfl_FoundryLedger.sh, rbfc_FoundryCore.sh, rbgp_Payor.sh. (Recipe: `grep 'ZRBGU_' Tools/rbk`.)

## Cross-module internal call rewrites
Moved functions call each other; rename every internal reference per the map. Notably: rbuh_require_ok calls rbuh_json_field_capture (now INTRA-module after the merge); rbge_* call rbuh_*; rbge_api_enable calls rbge_lro_ok + rbge_error_message_capture; rbgi_provision_service_agent calls rbuh_*. rbuh's retry loop and rbgo's own curl path both call rbgo_curl_status_is_transient_predicate (DOWNWARD into rbgo). rba functions call rbgo_get_token_capture (unchanged); rba_rbro_load sources rbro_regime.sh via `${BASH_SOURCE[0]%/*}` (same dir — resolves correctly from rba's file).

## Kindle / sentinel / sourcing wiring
- Kindle DAG flattens to: rbgc -> rbgo -> rbuh -> rbge. rbuh asserts rbgc + rbgo sentinels and validates the RBGC eventual-consistency consistency vars. rbge asserts rbuh. rbgi (existing) asserts rbuh (was rbgu). rba asserts rbgo only. rbgo gains the predicate but its dependency set is UNCHANGED (still rbgc + burd) — the predicate is stateless, no new state. Kindles are single-shot (buc_die on re-kindle); dependents assert via sentinel, they do not kindle deps.
- Sourcing: the 8 furnish-bearing clis that source rbgu_Utility.sh replace it with rbuh, rbge, rba in dependency order (3 lines, not 4 — rbuj is gone). rbgo is already sourced wherever rbgu was (rbgu depended on it), so the predicate resolves with no new source line. (Recipe: `grep -l 'rbgu_Utility.sh' Tools/rbk` — expect 8 surviving cli .sh files plus the acronyms doc, after the 2 vestigial clis are deleted; pre-deletion the grep shows 10.)
- Kindle sites: each `zrbgu_kindle` call -> the three new kindles in dep order (rbuh first, then rbge; rba where auth is used). Sentinel sites: each module-internal `zrbgu_sentinel` -> the new sentinel(s) that site actually depends on (rbuh for temp-file/HTTP/JSON use).
- rbgi sourcing is already covered: the only callers of the moved rbgi functions are rbgp_Payor (provision_service_agent, sa_email) and rbgg_Governor (jq_add_member) — both clis already source rbgi_IAM.sh.

## OPEN ITEM — RESOLVED (operator-approved disposition: delete)
Investigated and closed. rbgb_cli.sh and rbga_cli.sh are vestigial PRE-GATEWAY relics, not merely kindle-less:
- Unreachable: zero references in Tools/ or tt/ — no tabtarget, launcher, or workbench routes to either. Only hits are historical commit-affiliation records in retired JJ heat journals.
- Gateway violation (BCG "CLI as Module Gateway" + Template 2): they top-level-source via `${BASH_SOURCE[0]%/*}`, have NO furnish function, NO kindle calls, and end in `burd_dispatch "$@"` — the pre-BCG dispatch verb. The 8 working clis use `buc_execute «prefix»_ "…" z«prefix»_furnish "$@"`, sourcing + kindling inside furnish. rbgb/rbga predate that pattern.
- Non-functional as written: rbgb_Buckets.sh / rbga_ArtifactRegistry.sh assert zrbgu_sentinel; with nothing kindled, any real dispatch buc_die's at the first sentinel.
Disposition: DELETE both files outright in this pace (operator-approved scope addition). Do NOT source-swap them, do NOT rewrite them into modern gateway form. Their command modules (rbgb_Buckets.sh, rbga_ArtifactRegistry.sh) REMAIN — they are sourced/used by other clis and carry the ZRBGU_ leak-site rewrites above; only the two `_cli.sh` wrappers are deleted.

## Constraints
- Behavior-preserving (for the move/rename/resite core): move + rename + resite only. Temp-file string VALUES are behaviorally irrelevant (ephemeral, written+read within one run via the same constant) — renaming identifiers is safe. The two cli deletions are the one operator-approved exception to behavior-preservation (the files are dead — deleting them removes no live behavior).
- Library-only: no tabtarget / colophon changes. (The deleted clis have no tabtarget routing — confirmed — so this holds.)
- Bash discipline: read BCG before writing modules. New modules mirror rbgu's BCG structure (license header, line-2 `# shellcheck disable=SC2153  # kindle chain - per BCG` for cross-module Z*_ consumers, set -euo pipefail, ZRB**_SOURCED guard, zrb**_kindle + zrb**_sentinel, sectioned bodies). Per BCG "CLI as Module Gateway," kindle graphs are owned by the cli furnish/dispatch path — do not leak kindle outside. rbgp_Payor.sh is complex BCG (highest rbgu_ ref count) — do not edit blind.
- Mint: rbuh confirmed child of family rbu (rbuj retired — never created). rba, rbge terminal. All free.

## Docs to update (in scope)
RBS0-SpecTop.adoc, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc reference rbgu_ symbols; rbk-claude-acronyms.md has the RBGU entry. Update all four: replace renamed symbols, retire the RBGU acronym entry, add rbuh / rba / rbge entries (NOT rbuj), and note rbgo gains the transient predicate. Also correct rbgc_Constants.sh's comment reference to rbgu_http_json (-> rbuh_json) — comment-only, not behavior.

## Execution shape (advisory, not locked)
OPEN ITEM already resolved — go straight to execution. Single parallel agent wave over DISJOINT files (no two agents touch one file). One agent per new module (rbuh, rbge, rba — extract + rename + kindle/sentinel + assigned machinery); one agent owns rbgi_IAM.sh end-to-end (gains 3 fns + rewrites its own rbgu_ call sites); one agent adds the predicate to rbgo + fixes rbgo's self-call; remaining agents partition the pure-caller .sh files + the 4 docs. The two cli deletions (rbgb_cli.sh, rbga_cli.sh) are trivial — orchestrator does them directly. The orchestrator keeps the sourcing/kindle-chain wiring for itself (the linchpin) and verifies. Disjoint files mean no worktree isolation needed.

## What done looks like
rbgu_Utility.sh is gone; the two vestigial clis (rbgb_cli.sh, rbga_cli.sh) are gone; `grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing; every new name resolves to exactly one definition; all callers, sourcing, and the kindle chain updated; the four docs updated; dead code deleted. Verification: qualify (`tt/rbw-tr` — shellcheck + deny-warnings) clean, and the service-tier suite (`tt/rbw-ts.TestSuite.service.sh`) passes — it exercises rba's token path. Operator has confirmed live GCP credentials are available; the service tier is in-scope for the executing agent, not deferred.

## Discovery
- `grep 'rbgu_' Tools/rbk` — call sites and sourcing.
- `grep 'ZRBGU_' Tools/rbk` — temp-file constant leak sites.
- `grep -rn 'zrbgu_kindle\|zrbgu_sentinel' Tools/rbk` — kindle vs sentinel topology (the OPEN ITEM — now resolved — lived here).
- `grep -rln 'rbgb_cli\|rbga_cli' Tools/ tt/` — confirm the two clis remain unrouted before deleting.
- Source under dissolution: Tools/rbk/rbgu_Utility.sh.

**[260529-0552] rough**

## Character
Intricate but mechanical — a behavior-preserving move/rename/resite sweep. No new logic. The map below was derived under Opus 4.7, then critically re-reviewed and operator-approved under Opus 4.8; two structural changes resulted (rbuj merged into rbuh; transient predicate homed in rbgo). This revised map is the authoritative spec — do NOT re-derive or re-split. Detail exceeds normal docket discipline by explicit operator allowance because this pace executes imminently. Re-verify the as-discovered site lists with the grep recipes at mount (file paths are durable; line numbers are not — none are cited here).

## Goal
Dissolve `rbgu_Utility.sh` (the "Google Utility" grab-bag) into focused modules whose prefix matches what the code actually is, dropping the now-redundant infix. `rbgu` disappears entirely (`grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing).

## Revised frozen map (3 new modules; rbgi + rbgo absorb the rest; 2 deletions)

rbuh — NEW, Utility/HTTP + the JSON/string + temp-file machinery it produces and consumes (11 fns). Owns the shared temp-file constants. This is the rbuh+rbuj MERGE: the prior map split them, but they are mutually dependent (rbuh's require-ok reads JSON via the json-field capture; 3 of the 4 JSON fns read rbuh's temp-file constants) and always co-sourced/co-kindled — splitting was non-load-bearing. One module:
- rbgu_http_request            -> rbuh_request
- rbgu_http_json               -> rbuh_json
- rbgu_http_require_ok         -> rbuh_require_ok
- rbgu_http_code_capture       -> rbuh_code_capture
- rbgu_poll_until_ok           -> rbuh_poll_until_ok
- rbgu_poll_until_gone         -> rbuh_poll_until_gone
- rbgu_json_valid_predicate    -> rbuh_json_valid_predicate
- rbgu_json_field_capture      -> rbuh_json_field_capture
- rbgu_jq_file_to_file_ok      -> rbuh_jq_file_to_file_ok
- rbgu_urlencode_capture       -> rbuh_urlencode_capture
- (the transient predicate does NOT go here — see rbgo below)

Note: the original `rbgu_http_json`/`rbgu_http_request` define BOTH the retry-looping JSON helper and the BCG-conformant single-shot primitive — both move to rbuh as named.

rbge — NEW, GCP REST / LRO over rbuh (4 fns):
- rbgu_http_json_lro_ok        -> rbge_lro_ok
- rbgu_newly_created_delay     -> rbge_newly_created_delay
- rbgu_api_enable              -> rbge_api_enable
- rbgu_error_message_capture   -> rbge_error_message_capture

rba — NEW, Auth (mode-neutral; depends on rbgo, touches NO temp-file machinery — verified) (5 fns):
- rbgu_get_governor_token_capture -> rba_get_governor_token_capture
- rbgu_authenticate_role_capture  -> rba_authenticate_role_capture
- rbgu_rbra_load                  -> rba_rbra_load
- rbgu_rbro_load                  -> rba_rbro_load
- rbgu_extract_json_to_rbra       -> rba_extract_json_to_rbra

rbgi — EXISTING (IAM), gains 3 fns:
- rbgu_jq_add_member_to_role_capture -> rbgi_jq_add_member_to_role_capture
- rbgu_provision_service_agent       -> rbgi_provision_service_agent
- rbgu_sa_email_capture              -> rbgi_sa_email_capture

rbgo — EXISTING (OAuth), gains 1 STATELESS fn (the transient-curl-exit predicate):
- rbgu_curl_status_is_transient_predicate -> rbgo_curl_status_is_transient_predicate
  Rationale (operator-approved): this is the one stateless, sentinel-free function, consumed by BOTH rbgo's own curl-retry path AND rbuh's retry loop. rbgo is the lowest module that performs curl and is a kindle-dependency of rbuh, so it is the lowest common ancestor that can hold a function with the dependency arrow pointing only downward. It sits beside rbgo's existing stateless cross-module helpers (the rbgo_base64_* family, explicitly documented "safe to call from any module regardless of kindle order"). rbgc was rejected: it is constants-only, no home for methods.

DELETE (dead code, operator-approved — do NOT relocate):
- rbgu_write_vanilla_json — no callers anywhere.
- ZRBGU_EMPTY_JSON — written in kindle, never read (every other module owns its own).

## Shared temp-file machinery — rbuh owns it
The infix protocol: rbuh_json writes `${PREFIX}${infix}${POSTFIX}`; the capture, require-ok, and the moved rbgi/rbge functions read those files back by infix. rbuh is the owner:
- ZRBGU_PREFIX -> ZRBUH_PREFIX; ZRBGU_POSTFIX_JSON -> ZRBUH_POSTFIX_JSON; ZRBGU_POSTFIX_CODE -> ZRBUH_POSTFIX_CODE (defined in zrbuh_kindle).
- rbge and the moved rbgi functions reference ZRBUH_* -> their kindles assert zrbuh_sentinel.

## External constant-leak sites (NOT module-internal)
Five files reach `${ZRBGU_PREFIX}${infix}${ZRBGU_POSTFIX_JSON}` directly; rewrite to ZRBUH_* and ensure rbuh is kindled/sourced on those paths: rbgb_Buckets.sh, rbgg_Governor.sh, rbfl_FoundryLedger.sh, rbfc_FoundryCore.sh, rbgp_Payor.sh. (Recipe: `grep 'ZRBGU_' Tools/rbk`.)

## Cross-module internal call rewrites
Moved functions call each other; rename every internal reference per the map. Notably: rbuh_require_ok calls rbuh_json_field_capture (now INTRA-module after the merge); rbge_* call rbuh_*; rbge_api_enable calls rbge_lro_ok + rbge_error_message_capture; rbgi_provision_service_agent calls rbuh_*. rbuh's retry loop and rbgo's own curl path both call rbgo_curl_status_is_transient_predicate (DOWNWARD into rbgo). rba functions call rbgo_get_token_capture (unchanged); rba_rbro_load sources rbro_regime.sh via `${BASH_SOURCE[0]%/*}` (same dir — resolves correctly from rba's file).

## Kindle / sentinel / sourcing wiring
- Kindle DAG flattens to: rbgc -> rbgo -> rbuh -> rbge. rbuh asserts rbgc + rbgo sentinels and validates the RBGC eventual-consistency consistency vars. rbge asserts rbuh. rbgi (existing) asserts rbuh (was rbgu). rba asserts rbgo only. rbgo gains the predicate but its dependency set is UNCHANGED (still rbgc + burd) — the predicate is stateless, no new state. Kindles are single-shot (buc_die on re-kindle); dependents assert via sentinel, they do not kindle deps.
- Sourcing: the cli files that source rbgu_Utility.sh replace it with rbuh, rbge, rba in dependency order (3 lines, not 4 — rbuj is gone). rbgo is already sourced wherever rbgu was (rbgu depended on it), so the predicate resolves with no new source line. (Recipe: `grep -l 'rbgu_Utility.sh' Tools/rbk` — expect 10 cli .sh files plus the acronyms doc.)
- Kindle sites: each `zrbgu_kindle` call -> the three new kindles in dep order (rbuh first, then rbge; rba where auth is used). Sentinel sites: each module-internal `zrbgu_sentinel` -> the new sentinel(s) that site actually depends on (rbuh for temp-file/HTTP/JSON use).
- rbgi sourcing is already covered: the only callers of the moved rbgi functions are rbgp_Payor (provision_service_agent, sa_email) and rbgg_Governor (jq_add_member) — both clis already source rbgi_IAM.sh.

## OPEN ITEM — resolve FIRST, before any rewiring (do NOT guess)
rbgb_cli and rbga_cli source rbgu but contain ZERO kindle calls of any kind, yet zrbgb_kindle / zrbga_kindle assert zrbgu_sentinel and their modules' command functions assert their own sentinels. As written, invoking these two clis directly would buc_die at the first sentinel — nothing kindles the chain. Determine how (or whether) these entry points are actually reached and kindled before rewiring their source lines. This may surface a vestigial-cli cleanup that is arguably OUT of this behavior-preserving pace's scope — if so, flag to operator rather than absorbing it. (Recipe: trace `burd_dispatch` invocation + any tabtarget routing to rbgb/rbga; compare against the 8 clis that DO call zrbgu_kindle.)

## Constraints
- Behavior-preserving: move + rename + resite only. Temp-file string VALUES are behaviorally irrelevant (ephemeral, written+read within one run via the same constant) — renaming identifiers is safe.
- Library-only: no tabtarget / colophon changes.
- Bash discipline: read BCG before writing modules. New modules mirror rbgu's BCG structure (license header, line-2 `# shellcheck disable=SC2153  # kindle chain - per BCG` for cross-module Z*_ consumers, set -euo pipefail, ZRB**_SOURCED guard, zrb**_kindle + zrb**_sentinel, sectioned bodies). Per BCG "CLI as Module Gateway," kindle graphs are owned by the cli furnish/dispatch path — do not leak kindle outside. rbgp_Payor.sh is complex BCG (highest rbgu_ ref count) — do not edit blind.
- Mint: rbuh confirmed child of family rbu (rbuj retired — never created). rba, rbge terminal. All free.

## Docs to update (in scope)
RBS0-SpecTop.adoc, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc reference rbgu_ symbols; rbk-claude-acronyms.md has the RBGU entry. Update all four: replace renamed symbols, retire the RBGU acronym entry, add rbuh / rba / rbge entries (NOT rbuj), and note rbgo gains the transient predicate. Also correct rbgc_Constants.sh's comment reference to rbgu_http_json (-> rbuh_json) — comment-only, not behavior.

## Execution shape (advisory, not locked)
After the OPEN ITEM is resolved: single parallel agent wave over DISJOINT files (no two agents touch one file). One agent per new module (rbuh, rbge, rba — extract + rename + kindle/sentinel + assigned machinery); one agent owns rbgi_IAM.sh end-to-end (gains 3 fns + rewrites its own rbgu_ call sites); one agent adds the predicate to rbgo + fixes rbgo's self-call; remaining agents partition the pure-caller .sh files + the 4 docs. The orchestrator keeps the sourcing/kindle-chain wiring for itself (the linchpin) and verifies. Disjoint files mean no worktree isolation needed.

## What done looks like
rbgu_Utility.sh is gone; `grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing; every new name resolves to exactly one definition; all callers, sourcing, and the kindle chain updated; the four docs updated; dead code deleted. Verification: qualify (`tt/rbw-tr` — shellcheck + deny-warnings) clean, and the service-tier suite (`tt/rbw-ts.TestSuite.service.sh`) passes — it exercises rba's token path. Operator has confirmed live GCP credentials are available; the service tier is in-scope for the executing agent, not deferred.

## Discovery
- `grep 'rbgu_' Tools/rbk` — call sites and sourcing.
- `grep 'ZRBGU_' Tools/rbk` — temp-file constant leak sites.
- `grep -rn 'zrbgu_kindle\|zrbgu_sentinel' Tools/rbk` — kindle vs sentinel topology (the OPEN ITEM lives here).
- Source under dissolution: Tools/rbk/rbgu_Utility.sh.

**[260529-0535] rough**

## Character
Intricate but mechanical — a behavior-preserving move/rename/resite sweep. No new logic. The frozen map below was derived and operator-approved in a prior session; it is the authoritative spec. Detail exceeds normal docket discipline by explicit operator allowance because this pace is to be executed imminently. Re-verify the as-discovered site lists with the grep recipe at mount (file paths are durable; any line numbers are not).

## Goal
Dissolve `rbgu_Utility.sh` (the "Google Utility" grab-bag) into focused modules whose prefix matches what the code actually is, dropping the now-redundant infix. `rbgu` disappears entirely.

## Frozen rename map (23 functions relocate; 1 deleted)

rbuh — new, Utility/HTTP (owns the shared temp-file machinery), 7 fns:
- rbgu_http_request -> rbuh_request
- rbgu_http_json -> rbuh_json
- rbgu_http_require_ok -> rbuh_require_ok
- rbgu_http_code_capture -> rbuh_code_capture
- rbgu_curl_status_is_transient_predicate -> rbuh_transient_predicate
- rbgu_poll_until_ok -> rbuh_poll_until_ok
- rbgu_poll_until_gone -> rbuh_poll_until_gone

rbuj — new, Utility/JSON+string, 4 fns:
- rbgu_json_valid_predicate -> rbuj_valid_predicate
- rbgu_json_field_capture -> rbuj_field_capture
- rbgu_jq_file_to_file_ok -> rbuj_file_to_file_ok
- rbgu_urlencode_capture -> rbuj_urlencode_capture

rba — new, Auth (mode-neutral; depends on rbgo, NOT on the temp-file layer), 5 fns:
- rbgu_get_governor_token_capture -> rba_get_governor_token_capture
- rbgu_authenticate_role_capture -> rba_authenticate_role_capture
- rbgu_rbra_load -> rba_rbra_load
- rbgu_rbro_load -> rba_rbro_load
- rbgu_extract_json_to_rbra -> rba_extract_json_to_rbra

rbge — new, GCP REST over rbuh, 4 fns:
- rbgu_http_json_lro_ok -> rbge_lro_ok
- rbgu_newly_created_delay -> rbge_newly_created_delay
- rbgu_api_enable -> rbge_api_enable
- rbgu_error_message_capture -> rbge_error_message_capture

rbgi — EXISTING (IAM), gains 3 fns:
- rbgu_jq_add_member_to_role_capture -> rbgi_jq_add_member_to_role_capture
- rbgu_provision_service_agent -> rbgi_provision_service_agent
- rbgu_sa_email_capture -> rbgi_sa_email_capture

DELETE (dead code, operator-approved — do NOT relocate):
- rbgu_write_vanilla_json — no callers anywhere.
- ZRBGU_EMPTY_JSON — written in kindle, never read (every other module owns its own).

## Shared temp-file machinery — rbuh owns it
The infix protocol couples four modules: `rbuh_json` writes `${PREFIX}${infix}${POSTFIX}`; the capture, error, and IAM-jq functions read those files back by infix. rbuh is the owner:
- ZRBGU_PREFIX -> ZRBUH_PREFIX; ZRBGU_POSTFIX_JSON -> ZRBUH_POSTFIX_JSON; ZRBGU_POSTFIX_CODE -> ZRBUH_POSTFIX_CODE (defined in zrbuh_kindle).
- rbuj, rbge, and the moved rbgi functions reference ZRBUH_* -> their kindles assert zrbuh_sentinel.

## External constant-leak sites (NOT module-internal)
Five files reach `${ZRBGU_PREFIX}${infix}${ZRBGU_POSTFIX_JSON}` directly; rewrite to ZRBUH_* and ensure rbuh is kindled there: rbgb_Buckets.sh, rbgg_Governor.sh, rbfl_FoundryLedger.sh, rbfc_FoundryCore.sh, rbgp_Payor.sh. (Recipe: `grep 'ZRBGU_' Tools/rbk`.)

## Cross-module internal call rewrites
Moved functions call each other; rename every internal reference per the map. Notably: rbuh_require_ok calls rbuj_field_capture; rbge_* call rbuh_*/rbuj_* (and rbge_api_enable calls rbge_lro_ok/rbge_error_message_capture); rbgi_provision_service_agent calls rbuh_*/rbuj_*. rba functions call rbgo_get_token_capture (unchanged) and rba_rbro_load sources rbro_regime.sh via BASH_SOURCE (same dir — unaffected).

## Kindle / sentinel / sourcing wiring
- Kindle DAG: rbuh (asserts rbgc + rbgo sentinels, validates RBGC consistency vars) -> rbuj -> rbge; existing rbgi also asserts rbuh + rbuj; rba asserts rbgo only. Kindles are single-shot (buc_die on re-kindle) — dependents assert via sentinel, they do not kindle deps.
- Sourcing: 10 cli files source rbgu_Utility.sh — replace with sourcing rbuh, rbuj, rbge, rba in dependency order: rbgb_cli, rbfv_cli, rbgv_cli, rbfl_cli, rbgp_cli, rbfk_cli, rbfd_cli, rbga_cli, rbgg_cli, rbh0/rbhp0_cli. (Recipe: `grep -l 'rbgu_Utility.sh' Tools/rbk`.)
- Kindle sites: each `zrbgu_kindle` -> the four new kindles in dep order (rbuh first). Sentinel sites: each module-internal `zrbgu_sentinel` -> the new sentinel(s) that site actually depends on (rbuh for temp-file/HTTP use; add rbuj if it uses json captures).
- rbgi sourcing is already covered: the only callers of the moved rbgi functions are rbgp_Payor (provision_service_agent, sa_email) and rbgg_Governor (jq_add_member) — both clis (rbgp_cli, rbgg_cli) already source rbgi_IAM.sh.
- OPEN ITEM to trace at mount: rbgb_cli and rbga_cli source rbgu but contain no `zrbgu_kindle`, yet rbgb_Buckets / rbga_ArtifactRegistry assert zrbgu_sentinel. Find where rbgu is kindled for those paths before rewiring; do not guess.

## Constraints
- Behavior-preserving: move + rename + resite only. Temp-file string VALUES are behaviorally irrelevant (ephemeral, written+read within one run via the same constant) — renaming identifiers is safe.
- Library-only: no tabtarget / colophon changes.
- Bash discipline: read BCG before writing modules. New modules mirror rbgu's BCG structure (license header, set -euo pipefail, ZRB**_SOURCED multiple-inclusion guard, zrb**_kindle + zrb**_sentinel, sectioned bodies). rbgp_Payor.sh is complex BCG — do not edit blind.
- Mint: rbuh/rbuj confirmed children of family rbu; rba and rbge terminal. All four free.

## Docs to update (in scope)
RBS0-SpecTop.adoc, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc reference rbgu_ symbols; rbk-claude-acronyms.md has the rbgu entry. Update all four: replace renamed symbols, retire the RBGU acronym entry, add rbuh/rbuj/rba/rbge entries.

## Execution shape (advisory, not locked)
Single parallel agent wave over DISJOINT files (no two agents touch one file): one agent per new module (extract + rename + kindle/sentinel + assigned machinery); one agent owns rbgi_IAM.sh end-to-end (gains 3 fns + rewrites its own rbgu_ call sites); remaining agents partition the ~15 pure-caller .sh files + the 4 docs. The orchestrator keeps the sourcing/kindle-chain wiring for itself (the linchpin), then verifies. Disjoint files mean no worktree isolation needed.

## What done looks like
rbgu_Utility.sh is gone; `grep 'rbgu_\|ZRBGU_' Tools/rbk` returns nothing; every new name resolves to exactly one definition; all callers, sourcing, and the kindle chain updated; the four docs updated; dead code deleted. Verification: qualify (`tt/rbw-tr` — shellcheck + deny-warnings) clean, and the service-tier suite (`tt/rbw-ts.TestSuite.service.sh`) passes — it exercises rba's token path. Operator has confirmed live GCP credentials are available this session, so the service tier is in-scope for the executing agent, not deferred.

## Discovery
- `grep 'rbgu_' Tools/rbk` — call sites and sourcing.
- `grep 'ZRBGU_' Tools/rbk` — temp-file constant leak sites.
- Source under dissolution: Tools/rbk/rbgu_Utility.sh.

**[260528-1000] rough**

## Character
Intricate but mechanical — a behavior-preserving move/rename sweep.

## Goal
Dissolve `rbgu_Utility.sh` (the "Google Utility" grab-bag) into focused modules whose prefix matches what the code actually is. No behavior change.

## Locked mapping
Drop the now-redundant infix (`rbgu_http_request` → `rbuh_request`):

- **rbuh** (new — Recipe Bottle Utility / HTTP): http_request, http_json, http_require_ok, http_code, curl-transient predicate, poll_until_ok, poll_until_gone
- **rbuj** (new — Utility / JSON+string): json_valid, json_field, jq_file_to_file, urlencode
- **rba** (new — Auth / RBK credential domain, mode-neutral): get_governor_token, authenticate_role, rbra_load, rbro_load, extract_json_to_rbra
- **rbge** (new — GCP REST, GCP conventions over rbuh): http_json_lro_ok, newly_created_delay, api_enable, error_message
- **rbgi** (existing — IAM): write_vanilla_json, jq_add_member_to_role, provision_service_agent, sa_email

`rbgu` dissolves entirely.

## Constraints
- Behavior-preserving: move + rename + resite only; no new logic.
- OUT of scope: the role-keyed token accessor and the keyfile/federation fork — those belong to the gcp-hardening heat, not here. `rba` just holds the moved functions.
- Each new module carries its own kindle/sentinel per BCG; relocate the `ZRBGU_` temp-file machinery to its new owner(s).
- Library-only — no tabtarget/colophon changes.
- Mint: rbuh/rbuj/rba/rbge confirmed free. `rbu` is a family (children rbuh/rbuj); `rba` and `rbge` are terminal.

## Discovery
- `grep "rbgu_" Tools/rbk` for call sites and sourcing wiring.

## What done looks like
`rbgu_Utility.sh` is gone; every former function lives in its mapped module under the new name; all callers and sourcing updated; deny-warnings + shellcheck (qualify) clean; the service-tier suite passes (exercises `rba`'s token path).

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 F org-affiliated-account-reorientation
  2 A delete-dead-rbrp-parent-fields
  3 G explode-rbgu-grab-bag

FAG
··x RBS0-SpecTop.adoc, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc, rba_Auth.sh, rbfc_FoundryCore.sh, rbfd_FoundryDirectorBuild.sh, rbfd_cli.sh, rbfk_cli.sh, rbfl_FoundryLedger.sh, rbfl_cli.sh, rbfv_FoundryVerify.sh, rbfv_cli.sh, rbga_ArtifactRegistry.sh, rbga_cli.sh, rbgb_Buckets.sh, rbgb_cli.sh, rbgc_Constants.sh, rbge_Rest.sh, rbgg_Governor.sh, rbgg_cli.sh, rbgi_IAM.sh, rbgo_OAuth.sh, rbgp_Payor.sh, rbgp_cli.sh, rbgu_Utility.sh, rbgv_AccessProbe.sh, rbgv_cli.sh, rbhp0_cli.sh, rbk-claude-acronyms.md, rbuh_Http.sh
·x· RBSDE-depot_levy.adoc, rbrp.env, rbrp_regime.sh
x·· RBSHR-HorizonRoadmap.adoc, memo-20260522-org-affiliated-credential-reorientation.md, memo-20260527-workforce-federation-feasibility.md

Commit swim lanes (x = commit affiliated with pace):

  1 F org-affiliated-account-reorientation
  2 A delete-dead-rbrp-parent-fields
  3 G explode-rbgu-grab-bag

123456789abcdefghijklmnopqrst
·············xxxxx····xx·····  F  7c
························xx···  A  2c
···························xx  G  2c
```

## Steeplechase

### 2026-05-30 09:12 - ₢BSAAG - W

Dissolved rbgu_Utility.sh grab-bag into prefix-honest modules: new rbuh_Http (HTTP/JSON/temp-file machinery + owns ZRBUH_ temp constants), rbge_Rest (GCP REST/LRO + api-enable over rbuh), rba_Auth (RBRA/RBRO load + role token mint). rbgi_IAM absorbed 3 IAM utility fns and now asserts zrbuh+zrbge (rbgi→rbge edge: it calls rbge_error_message_capture); rbgo_OAuth absorbed the stateless transient-curl predicate and fixed its self-call. Rewired source+kindle across 8 clis (rbgo→rbuh→rbge, rba; rbge before rbgi). Deleted dead code (rbgu_write_vanilla_json, ZRBGU_EMPTY_JSON) and the 2 vestigial pre-gateway clis (rbga_cli, rbgb_cli). Operator-approved scope addition: fixed latent bug where rbgg_restore_project called the undefined rbgu_http_is_ok — now a new boolean rbuh_code_ok_predicate. Updated RBS0, RBSCIG, RBSCIP, RBK acronyms. Verified: grep rbgu_/ZRBGU_ in Tools/rbk returns nothing; bash -n clean on all 22 changed files; every new symbol resolves to one definition; rbw-tq fast-qualify green (shellcheck 180 files clean); service-tier suite 8 fixtures 119 passed 0 failed against live GCP — incl. access-probe JWT/OAuth token mint and the hallmark-lifecycle + batch-vouch cloud-build paths (rba + rbge LRO polling + rbuh). Executed as a 9-agent disjoint-file wave with orchestrator-owned cli/kindle wiring. Code committed 77ec3764. Note: docket named rbw-tr but that runs the full container-tier complete suite; ran the docket's described intent (rbw-tq lint gate + service tier) — full rbw-tr complete-suite qualify not run.

### 2026-05-30 08:53 - ₢BSAAG - n

Dissolve rbgu_Utility.sh grab-bag into prefix-honest modules. New: rbuh_Http (HTTP/JSON/temp-file machinery, owns the shared ZRBUH_ temp constants), rbge_Rest (GCP REST/LRO + api-enable over rbuh), rba_Auth (RBRA/RBRO load + role token mint). rbgi_IAM absorbs the 3 IAM utility fns (jq_add_member, provision_service_agent, sa_email) and now asserts zrbuh+zrbge in kindle (rbgi calls rbge_error_message_capture); rbgo_OAuth absorbs the stateless transient-curl-exit predicate and fixes its own self-call. All callers, sourcing, and kindle chains rewired across 8 clis (rbgo->rbuh->rbge, rba; rbge before rbgi). Deleted dead code (rbgu_write_vanilla_json, ZRBGU_EMPTY_JSON) and the 2 vestigial pre-gateway clis (rbga_cli.sh, rbgb_cli.sh). Fixed a latent pre-existing bug: rbgg_restore_project called the undefined rbgu_http_is_ok; replaced with a new boolean rbuh_code_ok_predicate (operator-approved scope addition). Updated RBS0, RBSCIG, RBSCIP, and the RBK acronyms doc. grep 'rbgu_|ZRBGU_' Tools/rbk now returns nothing; bash -n clean on all changed files; every new symbol resolves to exactly one definition; rbw-tq fast-qualify green (shellcheck 180 files clean).

### 2026-05-28 10:00 - Heat - S

explode-rbgu-grab-bag

### 2026-05-27 10:19 - ₢BSAAA - W

Delete dead RBRP_PARENT_TYPE/RBRP_PARENT_ID fields and close the spec/code divergence. The two fields were defined in rbrp.env and string-enrolled in rbrp_regime.sh but had zero readers — depot project creation is parentless and GCP auto-parents under the org. Removed both from rbrp.env (and their comment) and from the regime validator's Payor Project Identity group. Rewrote the RBSDE-depot_levy create-project step: replaced the phantom 'parent: {rbrp_parent_type}s/{rbrp_parent_id} (if parent_type != none)' line — whose attributes were undefined references used only at that one site — with a NOTE that the create is parentless and GCP places the project automatically (under the payor org when one exists, org-less otherwise). Verified: grep RBRP_PARENT/rbrp_parent across the tree returns zero matches; rbw-rpv payor regime validation passes on all four remaining fields.

### 2026-05-27 10:19 - ₢BSAAA - n

Removes the unused RBRP_PARENT_TYPE and RBRP_PARENT_ID config knobs; depot project creation is parentless, letting GCP auto-place new projects under the payor's org when one exists.

### 2026-05-27 08:13 - ₢BSAAF - W

Decided and recorded all of memo-20260522 §11. Banked R4 (synthetic-human CI + test-org session policy) and R5 (gauntlet coupling leans on skirmish), and a §11 disposition table marking all seven questions resolved or deliberately retired (only the keyfile-surfaces question retires — no MVP rework, R3 ships RBRA). Consolidated the federation tier into the new memo-20260527-workforce-federation-feasibility.md: Workforce-vs-Workload distinction, curl-native two-leg STS flow (gcloud-free), device flow + 12h session cap, three-bucket refresh-token policy, and a labeled sketch (RBRA->RBRI obsolescence, regime-scoped keyfile|federation mode enum, one shared secretless RBRI type for real and synthetic humans differing only in credential_source, cult-verb invariant). Improved the RBSHR Operator federation roadmap entry with the curl-native clause and references to the new memo + GCP docs. Decisions only; keyless tier remains deferred and unchosen per R3.

### 2026-05-27 08:11 - ₢BSAAF - n

Close out memo-20260522 §11 and consolidate the federation tier. Banked R4 (synthetic-human CI + test-org session policy: shipped roles persist no refresh token; the project's test rig persists one quarantined refresh token per firewalled machine, 12h session cap with month-span from the token; federation-era mechanism, keyfile-era uses the SA key itself) and R5 (gauntlet coupling: regression leans on skirmish; fresh-depot gauntlet levies hit the SA-key wall under secure-by-default orgs even in keyfile mode). Added a §11 disposition table marking all seven questions resolved or deliberately retired — only the probe/handbook/regime keyfile-surfaces question retires (no MVP rework since R3 ships RBRA). Created memo-20260527-workforce-federation-feasibility.md: Workforce-vs-Workload product distinction, the curl-native two-leg STS token-exchange flow (gcloud-free, correcting an earlier worry), device flow as Leg 1 with the 12h Workforce session cap, the three-bucket refresh-token policy, and a labeled sketch of RBRA->RBRI obsolescence, a regime-scoped credential mode enum (keyfile|federation), one shared RBRI type serving both real and synthetic humans (differing only in credential_source, secretless in both), and the cult-verb invariant (mantle/invest/divest/roster names survive, bodies detox from key-mint to IAM grant on the principal, one Payor-side federation-setup verb added). Improved the RBSHR Operator federation roadmap entry with a curl-native clause and extended _Reference_ to the new memo plus primary GCP docs.

### 2026-05-23 15:05 - Heat - T

org-detection-from-live-parent

### 2026-05-23 15:05 - Heat - T

establish-grants-payor-policy-authority

### 2026-05-23 15:05 - Heat - T

levy-preflight-confirms-policy-authority

### 2026-05-23 15:05 - Heat - T

levy-sets-project-scoped-key-override

### 2026-05-23 14:24 - ₢BSAAF - n

Replace the stale 'Browser-OAuth human authentication' roadmap entry with 'Operator federation'. The old entry described the impersonation/generateAccessToken mechanism (Approach C from the 2026-04-27 memo) that R1/R2 rejected; its topic — how humans authenticate to roles — is now decided near-term by R2 (direct capability grants), so it left roadmap scope entirely. The new entry captures the one genuinely-future item: operator federation, the heavyweight keyless tier for participants who cannot/will not hold Google accounts (external OIDC/SAML IdP, federated principal:// subjects, org-only). One orienting clause positions it as the far tier without restating the direct-Google baseline; depot framed as home of the participant roster while the IdP owns identity proofing; deconflicted from the envoy's egress (GCP-to-AWS) federation. Linked-term attributes ({rbtgi_depot}, {at_operator}, {rbtr_payor}) match neighbor entries. References the 2026-05-22 memo for the full tier ladder and deliberate deferral; notes the 2026-04-27 memo's human-auth framing is superseded.

### 2026-05-23 14:05 - ₢BSAAF - n

Scrub chat-local 'Model Y' label from the memo (3 occurrences in R1/R2/R3). The label was coined in conversation and has no definition outside it — a dangling reference once the chat ends. Each occurrence already carried its plain-English meaning ('direct human capability grants' / 'Direct Google grants'), so the parentheticals were simply deleted; prose stands on its own. R1/R2/R3 refs retained (self-defined as section headers). 'Model X' never entered the memo.

### 2026-05-23 13:59 - ₢BSAAF - n

Bank R3: three credential models are tiers on one axis (RBRA keyfile = consumer/bootstrap, MVP ships this; direct Google grants = Model Y org baseline; federation = org-only OIDC/SAML, no Google account, candidate paid tier). The real cost is multi-mode coexistence, not any single tier — frame as 'which one keyless successor replaces RBRA', one mode at a time. Decision: MVP ships RBRA, keyless successor deferred and unchosen while complexity budget is tight; bank the map not a committed ladder. IdP is commoditized by OIDC/SAML standards (self-hostable or GCP Identity Platform). Note ingress federation (operators reaching depot) is distinct from the roadmap envoy's egress federation (GCP->AWS).

### 2026-05-22 13:51 - ₢BSAAF - n

Bank R2 (Model Y) and amend R1. Decision: roles become IAM capability-sets granted directly to humans (user:), not per-role service accounts reached by impersonation. Verified the build path already separates submit (director, human-holdable: builds.editor/viewer/workerPoolUser/actAs-mason/repoAdmin) from runtime identity (mason, a per-depot SA impersonated only by Google's Cloud Build service agent, never a human) — so SAs retreat to pure runtime plumbing and every role permission is user-grantable. Consequences banked: zero per-role files (one OAuth login only), the role-SA addressing/naming question and Governor timestamp evaporate (maximal §6 dissolution), mantle/invest redefined as direct capability grants, standing-privilege trade-off accepted (bounded by reauth, JIT addable later). R1 annotated as superseded-in-part: its impersonation/tokenCreator mechanism is replaced while its durable conclusions (Payor ownership root, erased credential axis, central revoke, org/no-org consistency) survive. Resolves §11 'one human credential vs per-role': one login, nothing per-role.

### 2026-05-22 12:49 - ₢BSAAF - n

Add §13 Resolutions to the org-credential reorientation memo and bank R1: Payor stays distinct (ownership-rooted bootstrap tier + multi-depot/billing scope) while Governor/Director/Retriever collapse into one impersonation-grant primitive. Records that the keyless move erases the OAuth-vs-keyfile credential axis the specs use to set Payor apart (RBSGS), the implied mantle/invest verb redefinition from key-minting to token-creator bindings, SA-principal preservation, and org/no-org consistency via project ownership as the universal root. Resolves §11 'Does Payor stay distinct?'.

### 2026-05-22 09:45 - Heat - f

racing

### 2026-05-22 09:31 - Heat - n

Add design memo capturing the org-affiliated credential reorientation surfaced during the BM skirmish first-run. Records: the org invalidates the RBRA-keyfile assumption (durable SA keys anathema to serious orgs; disableServiceAccountKeyCreation enforced by default); the two independent org dials (reauth cadence + key-creation block); session evidence (override works but needs org-level policyAdmin, not project-scopable; ~1min propagation; mantle/invest non-idempotency); the reauth-as-compass reframe; the target model (human OAuth as universal front door, roles stay SAs reached by impersonation, identity-in-token vs authority-in-central-IAM); the RBRA recast to a single human credential + secretless impersonation pointers; the human-driven-vs-unattended axis (not org-vs-no-org); the synthetic-human CI corner and its contained cost; OOB deprecation and the Device Authorization Grant successor; and open questions feeding BSAAF. Extends and reframes memo-20260427-google-native-human-auth.

### 2026-05-22 07:56 - Heat - S

org-affiliated-account-reorientation

### 2026-05-21 16:23 - Heat - d

paddock curried: add BCG-before-bash constraint

### 2026-05-21 16:07 - Heat - S

levy-sets-project-scoped-key-override

### 2026-05-21 16:06 - Heat - S

levy-preflight-confirms-policy-authority

### 2026-05-21 16:06 - Heat - S

establish-grants-payor-policy-authority

### 2026-05-21 16:06 - Heat - S

org-detection-from-live-parent

### 2026-05-21 16:06 - Heat - S

delete-dead-rbrp-parent-fields

### 2026-05-21 15:13 - Heat - d

paddock curried: anchor keyless path on memo-20260427-google-native-human-auth; add headless/WIF gap

### 2026-05-21 11:40 - Heat - d

paddock curried: initial shape — secure-by-default org constraint bundle, override vs keyless fork

### 2026-05-21 11:39 - Heat - f

stabled

### 2026-05-21 11:39 - Heat - N

rbk-10-mvp-modern-organization-features

