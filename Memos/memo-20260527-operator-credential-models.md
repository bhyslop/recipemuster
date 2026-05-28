# Operator Credential Models — Keyfile and Workforce Federation

Date: 2026-05-27

Status: **Committed plan — support two operator-credential models as permanent,
per-depot-selected tiers.** The keyfile tier is the shipping default and needs no
organization. The federation tier is future build; this memo fixes its shape and
the architecture that lets both live in the codebase while never coexisting in a
single depot. It is the source document for the RBSHR "Operator federation"
roadmap entry.

---

## The plan

RBK supports two operator-credential models, permanently, as a free/paid split:

- **Keyfile** — service-account key files on disk (RBRA). No GCP organization
  required. The free, default tier; ships today.
- **Federation** — Workforce Identity Federation against an external IdP. No
  secret on disk. Requires a GCP organization. The paid tier.

A single per-depot **mode enum** selects which model is in force. One depot runs
exactly one model — the two never coexist within an installation — but the
codebase carries both forever. The boundary between the tiers is the organization
requirement, which is also the natural paywall.

---

## Two models, two tiers

| | **Keyfile** (free) | **Federation** (paid) |
|---|---|---|
| Operator secret on disk | SA key (RBRA) | none |
| GCP organization | not required | required |
| Identity proof | possession of the key | fresh IdP sign-in |
| Revocation | rotate / delete the key | disable at the IdP |
| Roles (gov/dir/ret) | one SA key file each | IAM grants on the operator's principal |

Both models terminate in the same artifact: a short-lived Google access token.
Everything downstream of the token — every depot API call — is identical and
model-blind.

---

## The organization requirement is the tier boundary

Federation's trust setting — the **workforce pool** — can only be created at the
**organization** level; a plain project cannot hold one. So federation is
structurally org-only, and that requirement is the line between free and paid.

An organization is obtainable at no cost via **Cloud Identity Free**, which
creates the org once the owner **verifies ownership of a DNS domain**. So the paid
tier's real prerequisite is "the operator controls a domain," not "the operator
pays Google." The keyfile tier exists to serve operators who cannot or will not
verify a domain.

---

## Workforce vs Workload — two federation products

GCP has two federation products answering one question: *what kind of identity is
authenticating?*

| | **Workforce** Identity Federation | **Workload** Identity Federation |
|---|---|---|
| For | **Humans** (console, gcloud, API) | **Machines** (CI, apps, cross-cloud) |
| Pool scope | **Organization-level** | Project-level |
| RBK use | **Operator federation** (this memo) | The **envoy** — GCP→AWS egress |

RBK uses both, for different actors; they coexist but never chain for a single
actor. The org-level pool scope is why operator federation is org-only.

---

## How federation authenticates

The flow is two legs, both inside RBK's existing curl/openssl/jq stack. gcloud
touches **setup only** (pool/provider creation, itself REST- and console-doable),
**never runtime**.

**Setup (once, Payor-side).** The Payor — as org admin — creates an org-level
workforce pool plus a provider trusting an external IdP (any OIDC or SAML 2.0
issuer). An **attribute mapping** pins `google.subject` to an *immutable* IdP
claim (e.g. `sub`/`oid`, ≤127 bytes); a mutable claim invites spoofing. Operators
receive depot capabilities as IAM grants on a **federated principal**:
`principal://iam.googleapis.com/locations/global/workforcePools/POOL/subject/SUBJECT`.
Identity proofing lives entirely in the IdP; RBK only grants capabilities to an
already-known principal, and the depot remains the home of the role-to-identity
roster.

**Leg 1 — IdP auth (outside Google).** Obtain an OIDC **ID token (JWT)** from the
IdP. The headless realization is the OAuth 2.0 Device Authorization Grant
(RFC 8628):

1. The tool POSTs the IdP's device-authorization endpoint → `user_code`,
   `verification_uri`, `device_code`.
2. The tool prints "go to `<uri>` and enter `WXYZ-1234`." The human authorizes in
   **any** browser, on any device — proving themselves to *their own* IdP. Google
   is not involved.
3. The tool polls the IdP's token endpoint until it returns the ID token.

**Leg 2 — STS exchange (pure REST against `sts.googleapis.com`):**

```
POST https://sts.googleapis.com/v1/token        # send NO Authorization header
  grant_type         = urn:ietf:params:oauth:grant-type:token-exchange   # RFC 8693
  audience           = //iam.googleapis.com/locations/global/workforcePools/POOL/providers/PROVIDER
  subject_token      = <IdP ID token from Leg 1>
  subject_token_type = urn:ietf:params:oauth:token-type:id_token
  → returns a short-lived Google access_token
```

Then bearer that token (`Authorization: Bearer …`) against any GCP API. Leg 2 has
no signing — it just POSTs a token RBK already holds.

**Session cap.** A Workforce session is configurable **15 min – 12 h, default
1 h** — a hard GCP ceiling. Long-running operation comes from re-running Leg 2,
and, where unattended, from a refresh token (test rig only) — never from a single
long session.

---

## IdP landscape

Federation trusts any **OIDC or SAML 2.0** issuer — Entra ID, Okta, AD FS, Ping,
Auth0, self-hosted Keycloak/Dex, or GCP Identity Platform as an in-house IdP. In
practice each operator org uses its existing IdP; RBK only points the pool at it.
The headless device flow (RFC 8628) is supported across Entra, Okta, Auth0,
Keycloak, and Google, so Leg 1 is curl-friendly on essentially every realistic
IdP. The only install where RBK itself chooses the IdP is the project's test org.

---

## Refresh-token policy

A persisted refresh token is the one durable secret federation can reintroduce.
Confine it to exactly one place:

- **Shipped human operators: no persisted refresh token.** Device flow per
  session; reauthentication on the operator org's own cadence.
- **Synthetic-human test rig (the project's own test org only): one persisted
  refresh token per firewalled machine** — the single acknowledged durable-secret
  residual, re-exchanged headless within the session cap. Issuing it requires the
  **`offline_access` scope**; without that scope most IdPs return no refresh
  token.
- **Payor: keeps its own OAuth refresh token** — the bootstrap human identity,
  outside the operator mode enum.

A persisted refresh token anywhere in the shipped path would recreate the
on-disk-secret problem federation exists to remove. Keep it caged in the test rig.

---

## Where configuration lives

Three homes, by scope:

- **Depot regime (RBRD)** holds the **mode enum** and, in federation mode, the
  org-wide federation facts: **pool, provider, audience**. These are identical for
  every operator of the depot. RBRD is tripwire-protected (`rbw-rdc` checks local
  `rbrd.env` against the GAR-inscribed copy), so the mode is **tamper-evident**: a
  depot cannot be silently downgraded from federation to keyfile without a
  deliberate re-inscription. Placing the enum here also makes one-mode-per-depot a
  structural fact rather than a per-machine convention — operators cannot
  independently straddle modes.
- **Station scope (or local)** holds only the per-operator **credential source** —
  how *this* machine obtains its IdP token (an interactive session cache, or, for
  the test rig, an executable backed by the caged refresh token).
- **RBRA files exist only in keyfile mode** — one per role
  (governor/director/retriever). In federation mode there are none.

There is no separate per-operator credential file: org-wide facts live in RBRD,
the one machine-local pointer lives at station scope, and nothing in between needs
to persist. A leaked station credential-source pointer is inert — it holds no
secret.

---

## Operator-admin verbs

The verbs — **mantle** (Payor→Governor), **invest** (Governor→Director/Retriever),
**divest**, **roster** — keep their names, actors, and directions in both models;
operator vocabulary does not change with the tier. Only their bodies differ:
keyfile mints an SA and downloads a key; federation adds or removes an IAM binding
on the operator's `principal://` subject.

Federation adds exactly one verb keyfile has no equivalent of: a one-time
**Payor-side federation-setup** verb that configures the workforce pool and IdP
trust. The test-rig "log a machine in for a month" is a separate **test-only
enrollment** verb, never in the shipped register.

---

## One consumer path, real and synthetic

Real and synthetic operators walk the **same consumer path** — read config,
resolve a token, call GCP, never knowing what is behind the token. They differ
only in the credential source: a real human's resolves against an interactive
device-flow session; the synthetic human's resolves against the caged refresh
token. This sameness is required, not convenient: CI must exercise the same code
path a real operator does, or it validates the wrong thing.

---

## The single code seam

Both models meet at one function: a **role-keyed token accessor** ("give me a
token as director"), with the mode enum branching inside it —

- **keyfile** → resolve role → RBRA file → signed-JWT exchange;
- **federation** → device flow → STS exchange.

Downstream is already model-blind: every GCP call takes a bare token string and
adds `Authorization: Bearer`, with no knowledge of its origin.

Today the keyfile token function is keyed by **file path**, and ~30 call sites
pass an explicit RBRA file — so the keyfile concept leaks into every consumer. The
enabling step, shippable now with **no behavior change**, is to collapse those
call sites behind the role-keyed accessor. After that refactor, federation is a
second branch inside one function; the call sites and the HTTP layer never change
again.

**Sequencing: refactor to the role accessor first (keyfile-only, today), then add
the federation branch.**

---

## Open at implementation time

- RBRD field names and any new mints for the mode enum and federation facts.
- Concrete credential-source wiring for the interactive human (device-flow output
  → what the token resolver consumes).
- The chosen IdP(s) for the test org and the federation-setup verb's exact API
  calls.

---

## Sources

- [Workforce Identity Federation — overview](https://docs.cloud.google.com/iam/docs/workforce-identity-federation)
- [Configure Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/configuring-workforce-identity-federation) — pools, providers, attribute mapping, session duration (15 min – 12 h, default 1 h)
- [Obtain short-lived tokens for Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/workforce-obtaining-short-lived-credentials) — STS token-exchange, file- and executable-sourced credentials
- [Using Workforce Identity Federation with API-based web applications](https://cloud.google.com/blog/products/identity-security/using-workforce-identity-federation-with-api-based-web-applications/) — gcloud-free runtime; IdP-auth then raw STS POST
- [Cloud Identity editions](https://docs.cloud.google.com/identity/docs/editions) and [set up Cloud Identity as admin](https://docs.cloud.google.com/identity/docs/how-to/set-up-cloud-identity-admin) — free org via domain verification
- [RFC 8628 — OAuth 2.0 Device Authorization Grant](https://datatracker.ietf.org/doc/html/rfc8628)
