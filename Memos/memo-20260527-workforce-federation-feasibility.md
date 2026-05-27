# Workforce Identity Federation — Feasibility & Shape

Date: 2026-05-27

Status: **Feasibility and concept — not implementation, not a build commitment.**
The keyless credential successor is deferred and unchosen per
`memo-20260522-org-affiliated-credential-reorientation.md` (R3). This memo
consolidates what a federation tier *is*, proves it fits the project's
curl/openssl/jq stack, and records the natural shape of the credential file and
operator verbs as a **labeled sketch**. It supersedes the human-auth *framing* of
`memo-20260427-google-native-human-auth.md` (whose impersonation mechanism R2
already replaced) and is the source document for the RBSHR "Operator federation"
roadmap entry.

---

## Why this memo exists

R3 left the keyless successor "deferred and unchosen." But a session of design
conversation produced enough durable understanding — and corrected one prior
error — that re-deriving it later would be waste. The valuable nuggets: the
Google-side credential path is **curl-native** (no gcloud, contrary to an earlier
worry), the human and machine federation products are **distinct and not
chained**, and the credential file most naturally **detoxifies rather than
multiplies**. None of this commits a build; all of it should survive the chat.

---

## 1. Workforce vs Workload — two products, two actors, not either/or

These are different GCP products answering one question: *what kind of identity
is authenticating?*

| | **Workforce** Identity Federation | **Workload** Identity Federation |
|---|---|---|
| For | **Humans** (console, gcloud, API) | **Machines** (CI, apps, cross-cloud) |
| Pool scope | **Organization-level** | Project-level |
| RBK use | **Operator federation** — operators reaching the depot | The **envoy** — GCP→AWS egress |

RBK uses both, for different actors; they coexist but never chain for a single
actor. The tier discussed here is **Workforce** (operator access). The org-level
pool scope is structural — which is exactly why operator federation is *org-only*
and a no-org Payor cannot use it.

---

## 2. The mechanism

1. The org owner (the Payor, as org admin) creates an org-level **workforce
   pool** plus a **provider** that trusts an external IdP — any OIDC or SAML 2.0
   issuer (Entra ID, Okta, AD FS, self-hosted Dex/Keycloak, or GCP Identity
   Platform as an in-house IdP). An **attribute mapping** pins `google.subject`
   to an *immutable* IdP claim (e.g. `sub`/`oid`, ≤127 bytes); choosing a mutable
   claim invites spoofing.
2. Operators receive depot capabilities as grants on a **federated principal**:
   `principal://iam.googleapis.com/locations/global/workforcePools/POOL/subject/SUBJECT`.
   Same IAM-grant shape as R2's direct human grants — just a federated principal
   instead of a Google `user:`.
3. Identity proofing (who is an operator) lives entirely in the IdP. RBK never
   "registers a human with the IdP"; it only grants depot capabilities to an
   already-IdP-known principal. The depot remains the home of the
   role-to-identity roster; the IdP owns only proofing.

---

## 3. The two-leg flow is gcloud-free / curl-native

The flow decomposes into two legs, both inside RBK's existing curl/openssl/jq
stack. gcloud touches **setup only** (pool/provider creation, itself REST- and
console-doable), **never runtime**.

**Leg 1 — IdP auth (RBK ↔ external IdP, outside Google):** obtain an OIDC **ID
token (JWT)** from the IdP via whatever flow it offers. Headless device flow
(below) keeps this in curl.

**Leg 2 — STS exchange (RBK ↔ `sts.googleapis.com`, pure REST):**

```
POST https://sts.googleapis.com/v1/token        # send NO Authorization header
  grant_type         = urn:ietf:params:oauth:grant-type:token-exchange   # RFC 8693
  audience           = //iam.googleapis.com/locations/global/workforcePools/POOL/providers/PROVIDER
  subject_token      = <IdP ID token from Leg 1>
  subject_token_type = urn:ietf:params:oauth:token-type:id_token
  → returns a short-lived Google access_token
```

Then bearer that token (`Authorization: Bearer …`) against any GCP API — the same
calls RBK already makes. Leg 2 is *simpler* than today's JWT-SA path (RBSAJ),
which signs a JWT with openssl: token-exchange has no signing, it just POSTs a
token RBK already holds.

**Correction banked:** an earlier worry that federation reintroduces a gcloud
dependency was wrong. The runtime is fully curl-native; only the IdP-auth leg's
*availability* of a headless flow varies by issuer.

---

## 4. Device flow is Leg 1; the 12-hour cap

"Device flow" (OAuth 2.0 Device Authorization Grant, RFC 8628) is the concrete,
headless realization of Leg 1 — the curl-friendly successor to the deprecated OOB
flow (memo-20260522 §9):

1. Tool POSTs the IdP's device-authorization endpoint → `user_code`,
   `verification_uri`, `device_code`.
2. Tool prints "go to `<uri>` and enter `WXYZ-1234`." The human authorizes in
   **any** browser, on any device.
3. Tool polls the IdP's token endpoint with `device_code` until it returns the
   OIDC ID token — the `subject_token` for Leg 2.

Device flow is an **IdP capability** (Entra/Okta/Auth0/Google support it). If the
chosen IdP offers it, Leg 1 is fully headless; if it only offers browser-redirect
SSO, Leg 1 needs a browser — but that is the human proving identity to *their own*
IdP, intrinsic to federation, not a gcloud artifact.

**Session cap:** a Workforce session is configurable **15 min – 12 h, default
1 h** — a hard GCP ceiling even on an org you own. Long-running operation comes
from re-running Leg 2 (silently) and, where unattended, from a refresh token (§5)
— never from a single long session.

**No workload hop in the human path.** The human path is exactly Leg 1 → Leg 2 →
API. Workforce tokens *can* impersonate a service account (the old Approach C
shape), but R2 deliberately dropped that — direct grants, SAs retreat to mason
runtime plumbing. So the human chain is *shorter*, not longer; Workload
federation is the envoy's separate track and never chains here.

---

## 5. Refresh-token policy — the line between honest and synthetic

A persisted refresh token is the marker that distinguishes an honest human-driven
session from a quarantined synthetic-human one. Use it in exactly one place,
deliberately (this is memo-20260522 R4):

- **Shipped human roles: no persisted refresh token.** Device flow per session;
  reauth on the operator-org's own cadence. The org-native ideal.
- **Synthetic-human test rig (project's own test org only): one persisted refresh
  token per firewalled machine** — the single acknowledged durable-secret
  residual. Re-exchanged headless every ≤12 h; the month-span is the token's
  lifetime, set by the IdP the project controls.
- **Payor (RBRO): keeps its existing persisted refresh token** — bootstrap human
  identity, revocable OAuth (not an SA key). A candidate to align with device
  flow later; not load-bearing here.

A persisted refresh token anywhere in the *shipped* path would re-create the
durable-secret-on-disk problem this whole reorientation exists to escape. Keep it
caged in the test rig.

---

## 6. Labeled sketch — credential file, mode enum, and verbs

> **SKETCH — not decided.** The keyless tier is deferred and unchosen (R3). This
> records the most natural shape if/when it is built. Exact regime homes, mints,
> function signatures, and API wiring are mount-time decisions, not fixed here.

**RBRO (Payor) — unchanged.** Google-native OAuth, persisted refresh token, the
ownership-rooted identity that owns the org and bootstraps everything (including
configuring the workforce pool in federation mode). Federation does not touch it.

**A credential mode enum, at regime scope, not per-identity.** R3's "one mode at a
time, not an accreting stack" is realized by a single installation-level enum —

```
keyfile     — tier 1, SA-key file (today's RBRA, MVP)
federation  — tier 3, Workforce IdP identity
```

It governs the *operational* roles (governor / director / retriever) only; the
Payor sits outside it. A per-identity enum would invite the very coexistence cost
R3 rejects, so the enum lives in a regime file (station regime the likely home),
and each identity file's existence and shape are *derived* from it. The enum is
the **migration switch**: flip it, and per-role secret files give way to one
secretless per-operator descriptor. (Whether a single org may ever straddle modes
mid-migration is the softest assumption here; R3 says it should not.)

**RBRA → RBRI: planned obsolescence, not a coexisting peer.** Ship MVP on
keyfile/RBRA (zero user churn). The keyless successor introduces a distinct,
*secretless* file type so the two are named for their nature and never coexist at
runtime:

| Mode | File | Holds a secret? | Count |
|---|---|---|---|
| `keyfile` (MVP) | **RBRA**, per-role | Yes (SA key) | 3 (gov/dir/ret) |
| `federation` | **RBRI** (identity), per-operator | **No** (`external_account` config) | 1 |

Under R2's direct-grant model there is nothing role-specific to store locally: a
single `external_account` descriptor (pool, provider, audience, and a
`credential_source`) is all that persists, holding no secret. Which of
governor/director/retriever an operator may exercise is the set of IAM grants on
their `principal://` subject, held **depot-side** — not encoded in any file. This
supersedes memo-20260522 §6's "RBRA = impersonate-`director-SA` pointer": with
impersonation gone (R2), the pointer degenerates to "which IdP/pool do I
authenticate against," shared across every role the one operator holds.
(`RBRI` appears free in the `RBR*` regime tree; a formal mint happens at
implementation time.)

**One RBRI type serves both real and synthetic humans.** The schema is identical;
the only difference is the `credential_source` field:

| Consumer | `credential_source` resolves against… | Persisted secret? |
|---|---|---|
| Real human | a transient session cache, refreshed by interactive device flow | None |
| Synthetic-human | an executable/file backed by a persisted refresh token | the refresh token, **separate from RBRI** |

This sameness is the *point*, not a convenience: §8 requires the synthetic human
to walk the **same code path** as a real human, or CI validates the wrong thing.
A shared RBRI type and an identical **consumer** path (read RBRI → resolve token →
call GCP, never knowing what is behind it) guarantee that fidelity. The two
diverge only in the **enrollment** verb (shipped device-flow vs test-only
persisted-refresh), never in the file or the consumer. And RBRI stays **secretless
in both cases** — the synthetic-human's durable secret lives in a separate
quarantined artifact RBRI merely points at, so a leaked RBRI is inert.

**Cult verbs: taxonomy invariant, bodies detox.** Across the keyfile→federation
migration the verb register survives intact — **mantle** (Payor→Governor),
**invest** (Governor→Director/Retriever), **divest**, **roster** keep their names,
actors, and directions; users' vocabulary does not churn. Only the verb *bodies*
change: from "mint SA + download key into RBRA" to "add/remove an IAM binding on
the `principal://` subject." Federation implies exactly one register *addition*
that keyfile mode has no equivalent of: a one-time **Payor-side federation-setup**
verb (configure the workforce pool + IdP trust). The test-rig "log a machine in
for a month" is a separate **test-only enrollment** verb, never in the shipped
user register.

---

## 7. Deliberately left open

These are implementation-tier and stay unanswered until the tier is chosen:

- The exact regime home of the mode enum (station vs repo) and whether straddling
  modes mid-migration is ever permitted.
- Concrete `credential_source` wiring for the interactive-human case
  (device-flow output → what GCP consumes).
- The chosen IdP(s) and whether each offers headless device flow.
- Coexistence/migration regime-file mechanics and the federation-setup verb's
  signature and API calls.

---

## Sources

- [Workforce Identity Federation — overview](https://docs.cloud.google.com/iam/docs/workforce-identity-federation)
- [Configure Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/configuring-workforce-identity-federation) — pools, providers, attribute mapping, session duration (15 min – 12 h, default 1 h)
- [Obtain short-lived tokens for Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/workforce-obtaining-short-lived-credentials) — STS token-exchange, file- and executable-sourced credentials
- [Using Workforce Identity Federation with API-based web applications](https://cloud.google.com/blog/products/identity-security/using-workforce-identity-federation-with-api-based-web-applications/) — gcloud-free runtime; IdP-auth then raw STS POST
- [Identity federation: products and limitations](https://docs.cloud.google.com/iam/docs/federated-identity-supported-services) — federated-identity access channels and per-product limits

## Cross-references

- `memo-20260522-org-affiliated-credential-reorientation.md` — R1/R2/R3 (credential
  model, tier ladder, deferral), R4 (synthetic-human CI + session policy), R5
  (gauntlet coupling).
- `memo-20260427-google-native-human-auth.md` — impersonation mechanism detail;
  its human-auth *framing* is superseded by R2.
- RBSHR "Operator federation" roadmap entry — the terse horizon record pointing here.
