# Google-Native Human Authentication — Design Exploration

Date: 2026-04-27

## Problem

Today's Recipe Bottle credential model distributes service-account JSON
key files (the RBRA pattern) for Governor, Director, and Retriever
roles.  For a single-developer or small-team project this works fine.
For wider adoption, two frictions emerge:

1. **Distribution logistics.**  Whoever holds Payor (or Governor) must
   generate an SA key, ship it via secure channel to each user, and
   the user must place it on disk at the regime-defined path.
   Multiply across multiple Depots, multiple roles, multiple users.

2. **Revocation hygiene.**  SA keys do not expire.  Revoking access
   means rotating the key (and re-issuing to legitimate users), then
   trusting that old copies are gone.  There is no central "kill this
   person's access" button.

The natural ask from a future external user: "I have a Google account,
why can't I just log into Chrome and use this?"

## Approaches Evaluated

### A. Status quo (RBRA keyfile distribution)

Current state.  Payor generates SA key → ships to user → user places at
regime-defined path → all subsequent operations use the key file
directly via the existing JWT-bearer flow in RBGO.

- **Pros:** works with no cloud-side identity management; user needs
  nothing Google-specific installed.
- **Cons:** all the distribution and revocation problems above.

### B. gcloud SDK + Application Default Credentials + impersonation

User installs gcloud SDK, runs `gcloud auth login`, gets
`roles/iam.serviceAccountTokenCreator` on the target SA, uses
`--impersonate-service-account` (or sets
`auth/impersonate_service_account` config) to act as the SA.

- **Pros:** Google's recommended path.  Well-documented.  Token
  caching and credential discovery are handled by the SDK.
- **Cons:** requires gcloud SDK install — large dependency surface
  relative to Recipe Bottle's current minimal stack (curl, openssl,
  jq).  Project policy has consistently rejected gcloud as a
  prerequisite for the same surface-area reasons.

Verdict: dismissed.

### C. REST-only browser OAuth + impersonation (recommended)

Replicates the existing Payor OAuth pattern (RBGP/RBGO) for a new
"user impersonation" OAuth client.  User authenticates in a browser
once; refresh token cached locally.  When an operation needs to act as
Governor/Director/Retriever, mint a user access token from the
refresh token, then call

```
POST https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{sa-email}:generateAccessToken
Authorization: Bearer <user access token>
```

to obtain a short-lived SA access token.  Use that SA token in the
`Authorization: Bearer` header for the actual API call.

- **Pros:**
  - No new dependency: reuses the existing curl/openssl/jq stack.
  - SA principal preserved — RBSCIG contracts, RBGI machinery, role
    taxonomy, audit identity all unchanged.
  - Distribution dissolves: nothing to ship; access is granted by IAM
    binding (server-side state).
  - Revocation: one IAM binding deletion, or the user can revoke
    OAuth consent at myaccount.google.com.  No artifact to chase.
  - Audit logs gain `delegationInfo`: "user X impersonated SA Y at
    time Z" — better forensics than today's invisible-human keyfile
    model.
- **Cons:**
  - Project must register a new OAuth client (app identity for the
    impersonation flow); analogous to today's Payor OAuth client.
  - Two extra REST calls per operation (token refresh + impersonation
    exchange), both cacheable for the token's lifetime.
  - Coexistence with RBRA: probes (RBSAJ, RBSAO, RBSAV) and any
    credential-mode-aware machinery must handle both modes.

## Mechanism Detail

### Authentication flow

1. **Once per user (sign-in).**  User runs a tabtarget — provisional
   shape: an OnboardingCredentialBrowser entry — which opens a
   localhost callback listener and launches the system browser to
   Google's OAuth consent endpoint.  User signs in with their Google
   account, consents to the `cloud-platform` scope, the authorization
   code returns to localhost, and the tabtarget exchanges it for a
   refresh token via `oauth2.googleapis.com/token`.  Refresh token
   cached locally.
2. **Once per user (admin-side grant).**  Governor (or whoever
   administers the Depot's SA roster) grants the user
   `roles/iam.serviceAccountTokenCreator` on each SA the user is
   authorized to act as.  This is a new Governor verb operating on
   SA IAM policy — sibling to mantle/charter/knight/forfeit.
3. **Per operation.**  Tabtarget refreshes the user access token (POST
   to `oauth2.googleapis.com/token` with `grant_type=refresh_token`)
   if the cached one has expired, then calls `generateAccessToken` on
   the impersonation endpoint to obtain an SA access token (TTL ≤ 1
   hour, configurable).  Use SA token in `Authorization: Bearer` for
   the API call.

### Storage shape

Two persistent items, parallel to today's Payor OAuth:

| Artifact | Origin | Sensitivity |
|---|---|---|
| OAuth client ID + secret | Project owner registers once at console.cloud.google.com, ships in repo or installed via existing JSON-ingest pattern (analogous to RBSPI) | App identity, not user-secret |
| User refresh token | Generated by browser consent, lands locally after first sign-in | Long-lived, sensitive |

Two transient caches (re-mintable from the refresh token):

- User access token (~1 hour)
- SA access token from `generateAccessToken` (≤ 1 hour, configurable)

Storage location: open question (see below).  In all cases: 0600
permissions, gitignored.

### IAM binding model

Service accounts in GCP are dual-natured: identities (principals that
act) and resources (with their own IAM policies).  The grant lives in
IAM policy on the SA itself:

```
Resource: governor@<depot>.iam.gserviceaccount.com
  Binding: roles/iam.serviceAccountTokenCreator
  Member:  user:alice@example.com
```

Multi-Depot, multi-role, multi-user is a sparse three-axis matrix
populated by IAM bindings on SAs across Depots:

```
                  Depot D1              Depot D2
alice@...     Governor, Director    Director
bob@...       Director              Retriever
carol@...     —                     Governor, Director, Retriever
```

Grants do not span Depots — each Depot is its own boundary.  Discovery
from the user side: enumerate SAs in a Depot, check each SA's IAM
policy for the user's email — no central registry needed because the
policies on the SAs are the registry.

### Administrative home

Naturally extends Governor.  Today Governor mantles itself, charters
Retriever, knights Director, forfeits SAs — managing the SA roster
within its Depot.  Adding a verb for "grant user X impersonation
rights on SA Y" sits in the same module operating on the same kind
of object (SA IAM policy).  A complementary verb removes the binding.

## Coexistence with RBRA

Additive in the near term, not a replacement:

- Headless paths (Cloud Build itself, automated fixtures, Ifrit-style
  scenarios) still need keyfiles or workload identity.
- Existing users with working RBRA setups do not need to migrate.
- A regime mode field selects credential mode per role (provisional:
  `RBRA_MODE=keyfile|impersonation`); the token-mint helper branches
  on mode at the lowest layer, leaving downstream callers unchanged.
- Credential probes (RBSAJ, RBSAO, RBSAV) gain mode dispatch.

Eventual deprecation of the keyfile mode for human-driven operations
is a separate decision, triggered by adoption signal — not part of
this work.

## Threat Model Summary

| Concern | RBRA keyfile (today) | OAuth + impersonation |
|---|---|---|
| Artifact on disk | SA private key (long-lived) | User refresh token (revocable) |
| Workstation compromise | SA fully compromised until rotation | User can revoke OAuth consent; admin can drop IAM binding |
| Revocation | Hunt all key copies, rotate | Single IAM binding deletion |
| Audit attribution | "SA did X" — human invisible | "User X impersonated SA at time Z" |
| Required user state | JSON file at known path | Refresh token at known path + IAM binding admin-side |

The proposed mechanism is strictly stronger on every axis the existing
RBRA model addresses, with the caveat that it requires a one-time
admin grant per (user, SA) pair rather than a one-time keyfile handoff.

## Open Questions

- **Storage location.**  Regime-tree (per-checkout, gitignored, dies
  with `git clean -fdx`) or `$HOME`-rooted (per-user, multiplexes
  across checkouts, survives clean operations)?  The Payor refresh
  token's current home is the obvious starting point.
- **OAuth client scope.**  One project-wide OAuth client (registered
  once at the project level, used by all installations) or per-Manor
  (each Manor registers its own)?  Tradeoff: shared client simplifies
  onboarding but ties all users to the project's OAuth consent
  screen and ToS.
- **Onboarding handbook track.**  Parallel RBHO* track
  (OnboardingCredentialBrowser) alongside the existing keyfile
  tracks, or a unified track that branches on user choice?
- **Probe coexistence.**  Should existing probes auto-detect mode
  from regime, or should there be separate probes per mode?
- **Mode field naming and placement.**  Does the mode live on the
  RBRA file itself, on the regime that points to it, or as a
  separate per-role regime field?  Affects how the probe stack
  loads.

## Relationship to Existing Roadmap Items

No direct dependencies.  Indirectly relates to "Credential confinement
to crucible" (Crucible Conduit Architecture) in spirit — both move
credentials toward shorter-lived, more bounded forms — but the
mechanisms are independent.  Credential confinement is about where
credentials live within the workstation/crucible split; this work is
about whether the workstation-side credentials are static keys or
browser-minted tokens.

## Trigger to Revisit

Either:

- First concrete external-user request for browser-based onboarding.
- First time keyfile distribution materially blocks adoption (rather
  than being theoretically frictionful).

## Notes on Conversation Origin

Captured from a design conversation on 2026-04-27 between the project
owner and Claude.  The conversation traced the constraint chain:
"users won't want to distribute RBRA files" → impersonation pattern →
"does this require gcloud?" → REST-only flow against
`iamcredentials.googleapis.com` → "where does the security state
live?" → storage shape → "how does the user tie to roles?" → IAM
bindings on SAs as resources.  The key turn was recognizing that the
Payor OAuth machinery already proves the harder primitive (browser
auth without gcloud); adding human-to-SA impersonation is a small
extension on top, not a new dependency layer.
