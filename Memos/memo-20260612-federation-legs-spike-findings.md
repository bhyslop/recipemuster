# Federation Legs Spike — Findings (₢BZAAA)

Date: 2026-06-12

Status: Evidence record for the ₣BZ federation-legs spike. The three-leg
office-federation runtime chain is **proven end-to-end live** against the
canonical standing depot, in pure curl/jq (no gcloud at runtime). Setup
ceremony used payor-token REST (curl) throughout; the only console work was
Microsoft-side (Entra tenant + app registration).

---

## The proven chain (the headline)

A federated principal minted an office-SA token from a clean shell and called
a depot API with it:

1. **Leg 1 — Entra device flow (RFC 8628).** POST to
   `login.microsoftonline.com/{tenant}/oauth2/v2.0/devicecode`, human approved
   in browser, poll returned the OIDC ID token. Scope `openid profile email` —
   **no `offline_access`, and the response carried no refresh token**
   (D2 verified live: `has_refresh_token: false`).
2. **Leg 2 — STS exchange (RFC 8693).** POST `sts.googleapis.com/v1/token`,
   no auth header, audience = the provider resource name → Google federated
   access token, `expires_in 3598`. **No billing/quota project was required
   at this leg.**
3. **Leg 3 — `generateAccessToken`.** POST
   `iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{office-SA}:generateAccessToken`
   bearing the federated token **plus `x-goog-user-project: {depot}`**
   (see Finding F2) → office token, `expireTime` exactly +1 h.
4. **Bearer call.** `artifactregistry.googleapis.com/v1/.../repositories`
   listed the depot's GAR repo with the office token — **no quota-project
   header needed** (an impersonated office token is a plain SA token; the
   product support matrix and the quota-project ceremony are both dodged
   downstream, as the architecture premised).

Test fixture (standing test trust candidate, per the docket cinch):

- Org `scaleinvariant.org` = `organizations/247899326218`
- Pool `locations/global/workforcePools/spike-office-test`, session 3600s
- Provider `spike-entra`: issuer
  `https://login.microsoftonline.com/beb3e682-4589-4228-bcf9-27d48c125dbd/v2.0`,
  clientId `6d54c87a-e6c4-47d0-81e6-b91fe470ee1c`,
  mapping `google.subject = assertion.oid`, webSsoConfig ID_TOKEN /
  ONLY_ID_TOKEN_CLAIMS
- Office SA `spike-office-test@cancbhm-d-canest3bhm100001.iam.gserviceaccount.com`
  with `roles/artifactregistry.reader` on the depot project
- Admission binding: `roles/iam.serviceAccountTokenCreator` on the office SA
  for `principal://iam.googleapis.com/locations/global/workforcePools/spike-office-test/subject/9657166c-8a2d-4f5d-bcd1-ef481ee31f3e`
  (the Entra user's immutable `oid`) — at spike end this is the **only**
  binding on the SA policy, the one-binding admission shape working as cinched
- Entra side: free tenant `beb3e682-…`, app registration `spike-office-test`
  (single tenant, public client flows enabled, no secret, no redirect URI)

## Findings that feed the founding/admission ceremonies

- **F1 — Payor needs `roles/iam.workforcePoolAdmin` at org level.** The
  payor's standing org roles (`organizationAdmin`, `orgpolicy.policyAdmin`)
  do **not** include workforce-pool permissions; pool list/create 403'd until
  the payor self-granted it (`organizationAdmin` suffices to self-grant; the
  grant propagated in ~10 s). Establish-federation must seat this role.
- **F2 — Leg 3 requires a quota project; admission must grant
  `roles/serviceusage.serviceUsageConsumer`.** A workforce federated token
  carries no API-consumer project; bare `generateAccessToken` fails
  `403 PERMISSION_DENIED: "Method doesn't allow unregistered callers"` —
  a Palisade signature worth recognizing on sight (it reads like an API-key
  complaint, not an IAM denial, and no propagation wait fixes it). Fix:
  `x-goog-user-project: {depot}` header on Leg 3 + grant the federated
  principal `serviceUsageConsumer` on the depot. With both in place the call
  succeeded on the first attempt. `iamcredentials.googleapis.com` was already
  ENABLED on the depot (levy-time state, no action needed).
  Admission ceremony therefore writes **two** bindings, not one:
  tokenCreator on the office SA + serviceUsageConsumer on the depot project.
  The paddock's "admission is one binding" needs this amendment — flagged,
  not absorbed.
- **F3 — STS Leg 2 needs nothing extra.** No userProject, no auth header.
  The canon's Leg 2 description stands exactly as written.

## Verification items (docket "Done when" list)

- **V1 — Office-token lifetime ceiling: 1 h default and max; 12 h only via
  org policy.** Paper: the lifetime-extension constraint is
  `constraints/iam.allowServiceAccountCredentialLifetimeExtension` (list
  constraint naming the SA). Live: requesting `lifetime: 43200s` was refused
  `400 INVALID_ARGUMENT` with the constraint named in the error text. The
  retriever-office 12 h posture rides exactly this org-policy listing.
  Note the distinction: the workforce **session** duration (pool
  `sessionDuration`, 15 min–12 h) caps the federated token; the **office**
  token is independently capped at 1 h/12 h-by-policy. The two ceilings
  compose; an office token CAN outlive the workforce session that minted it.
- **V2 — AR / Cloud Build workforce federated-token support matrix:** both
  **GA, "No known limitations"** (Identity federation: products and
  limitations table). Adjacent wrinkle recorded: an AR troubleshooting note
  describes federated-user 400s on pushes/pulls tied to attribute mappings in
  the AR URL — moot under office impersonation but evidence for why the
  matrix-dodge premise earns its keep.
- **V3 — Audit-log shape under impersonation:**
  - `GenerateAccessToken` is audit-logged **always-on** — the depot had no
    `auditConfigs` at all, yet every mint produced a `data_access` entry.
    Corollary: `iamcredentials.googleapis.com` **rejects service-level audit
    config** (`400: does not … support service level configuration of Google
    Cloud audit logging`) because its token-mint logging is not optional.
  - **Surprise, flagged:** the always-on mint entry attributes
    `authenticationInfo.principalEmail` to the **office SA itself**, with a
    `loggableShortLivedCredential` token prefix — the federated caller's
    `principalSubject` is **absent** from the mint record, contradicting the
    documented workforce example-log shape and weakening the
    "every use is logged with subject" custody claim for the mint hop
    specifically. The caller IP and user agent are present.
  - **Downstream attribution observed, fully intact.** With AR audit logs
    enabled, the office-token `ListRepositories` call logged:
    `principalEmail` = the office SA, and
    `serviceAccountDelegationInfo[].principalSubject` =
    `principal://…/workforcePools/spike-office-test/subject/{oid}` — the
    human, by immutable IdP claim. Every office-token use is attributable to
    the federate at the using service's log. Two conditions for that trail:
    (a) the service's audit logs must be enabled on the depot (opt-in,
    per-service — a **levy-ceremony decision**), and (b) the right log class:
    `ListRepositories` is **ADMIN_READ**, not DATA_READ — the first
    DATA_READ-only config produced nothing for it. Artifact pulls are
    DATA_READ; enable both for retriever-trail coverage.
  - Net audit posture: the mint hop names the SA + caller IP but not the
    human; the use hop names both SA and human. Attribution lives downstream.
- **V4 — STS egress-rule shape under VPC-SC (paper):** the workforce pool is
  org-level and cannot sit inside a perimeter, so STS calls whose audience is
  the pool need an **egress rule with `identityType: ANY_IDENTITY`** (the
  token endpoint is unauthenticated) to `sts.googleapis.com`, method `*`.
  `iamcredentials` is GA under VPC-SC with no known limitations; workforce
  principals are nameable in ingress/egress rules.
- **V5 — Keycloak-uploaded-JWKS viability (paper): viable for the hermetic
  test case.** Workforce OIDC providers accept `jwksJson` precisely for
  issuers that are not publicly reachable (Google then never dials the
  issuer; the issuer URI is a string match needing only `https://` shape).
  Documented restriction: uploaded JWKS serve **only the programmatic flow**
  (direct STS `/token` exchange — exactly Leg 2); console federated sign-in
  cannot use them. A Keycloak-in-a-crucible standing test IdP is therefore
  legitimate for suite personas; console access would need the real-IdP
  trust instead.

## Session-cache shape — elected (MVP)

What is cached: **the STS federated token only** (plus its expiry and the
subject), in one per-operator-session scratch file, 0600 in a 0700 dir,
tmpfs-preferred (`$XDG_RUNTIME_DIR` on Linux, `$TMPDIR` on macOS), written
atomically (temp + rename). Never a regime, never `BURD_OUTPUT_DIR`
(two-run clobber horizon), never `BURD_TEMP_DIR` (per-invocation lifetime is
too short — the cache must span tabtarget processes within one session).

- Office tokens are **not** cached: each verb-run mints its office token from
  the cached federated token (one cheap REST call, F2 header included) and
  holds it in process memory only. This keeps the higher-authority artifact
  ephemeral and makes one-office-per-token blast radius automatic.
- Accessor behavior: cache hit and unexpired → use; miss/expired and a TTY is
  present → run Leg 1 inline (print `verification_uri` + `user_code`),
  then Legs 2 and cache; miss/expired and **no TTY → fail loud** (the
  headless fail-fast membrane, canon D3/D2).
- The Entra ID token is never persisted at all — consumed by Leg 2 in-process.

## Incidental observations

- The depot's `artifactregistry.reader` binding carries three
  `deleted:serviceAccount:…?uid=` tombstones from prior retriever churn —
  the credential-churn memo's tombstone pattern observed live in the standing
  depot.
- Entra onboarding snag for solo evaluators: a bare personal Microsoft
  account cannot register apps ("outside of a directory has been
  deprecated"); the M365 Developer Program path is effectively closed to solo
  signups, and the working path is a free Azure signup (card required, $0)
  which provisions a real tenant. ~15 min of ceremony, one-time. Tenant and
  app registration themselves are free.
- `cloud.google.com` doc URLs 301 to `docs.cloud.google.com` and the docs
  pages truncate under naive fetch; the support-matrix table needed a raw
  HTML pull to read the AR/Cloud Build rows.
- Tooling self-lesson (cost ~30 min): piping REST responses through
  `jq '.field'` masks auth failures — an expired payor token returned 401
  error objects that the filters rendered as `null`/`[]`, which read as
  "config vanished" and "no log entries." Spike scripts should branch on
  `.error` explicitly (`if .error then {ERR: …} else … end`) before
  extracting. The 1 h payor token expiring mid-spike is the trigger case.

## Cloud-resource ledger (all additive, spike-created)

| Resource | Disposition |
|---|---|
| Org binding: payor → `iam.workforcePoolAdmin` | Keep (founding ceremony needs it) or revoke post-spike — operator call |
| Pool `spike-office-test` + provider `spike-entra` | Candidate standing test trust (docket cinch) |
| SA `spike-office-test@…` + its tokenCreator binding | Spike fixture; delete or keep with the pool |
| Depot bindings: SA → `artifactregistry.reader`, principal → `serviceUsageConsumer` | Spike fixtures; same disposition as the SA |
| Depot auditConfig: AR `ADMIN_READ` + `DATA_READ` | Enabled mid-spike for V3; harmless to keep, trivial to drop |
| Entra: free tenant + `spike-office-test` app | Microsoft-side, operator-owned |

No existing depot SAs, bindings, or regimes were modified. The only
non-additive touch is the depot auditConfig row above.
