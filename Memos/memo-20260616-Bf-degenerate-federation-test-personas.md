# Memo — Degenerate Federation for Test Personas (non-interactive assize acquisition)

- **Date:** 2026-06-16
- **Author:** Claude Opus 4.8, in conversation with Brad
- **Heat:** ₣Bf (compearance-headless / test-rig reconsideration)
- **Status:** finding recorded; **approach not yet resolved** — this memo retains the
  concept until the heat settles which degenerate shape (if any) to adopt.

> Memos are provenance, never authority. The durable facts here (the STS programmatic
> flow, the JWKS-programmatic-only constraint) belong in a spec once an approach is
> chosen; this memo is the staging ground, not the home.

## The question

For test purposes, can we automate away the human's interactivity step — the
device-flow *compearance* — by configuring the federation to be **degenerate**: a
collapsed IdP configuration that issues acceptable tokens without a human browser
approval?

## Short answer

**Yes.** GCP Workforce Identity Federation exposes a **programmatic** STS
token-exchange flow that accepts a third-party OIDC JWT and returns a Google
federated token with **no browser and no device flow**. STS validates only
**signature (against the provider's JWKS) + issuer + audience + subject claims**; it
does not care *how* the JWT was minted. This confirms — now against live GCP docs —
the spike's V5 paper-finding (`Memos/memo-20260612-federation-legs-spike-findings.md`).

**Honesty caveat:** this is **doc-confirmed + spike-paper-confirmed**, not yet
live-run in our own harness. The caged-JWT path below has not been exercised end to
end against our pool; treat it as a strong design lead, not a proven recipe.

## Mechanism — the programmatic flow

`POST https://sts.googleapis.com/v1/token` (RFC 8693 token exchange):

| Parameter | Value |
|---|---|
| `grant_type` | `urn:ietf:params:oauth:grant-type:token-exchange` |
| `subject_token_type` | `urn:ietf:params:oauth:token-type:id_token` (OIDC; SAML alt: `…:token-type:saml2`) |
| `audience` | `//iam.googleapis.com/locations/global/workforcePools/POOL/providers/PROVIDER` |
| `subject_token` | the external OIDC JWT |
| `scope` | `https://www.googleapis.com/auth/cloud-platform` |
| `options` | `{"userProject": "<depot>"}` (quota/billing project) |

Returns a short-lived (~1 h) Google access token scoped to the workforce pool.

Doc-confirmed facts that make the degenerate path work:

- Locally **uploaded OIDC JWKS "can only be used in programmatic flow"** — i.e. the
  no-console path. Console / implicit / authorization-code sign-in **cannot** ride
  uploaded JWKS. This is precisely the asymmetry the spike's V5 flagged.
- The flow is documented as working on **"headless machines"** via file-sourced,
  URL-sourced, and executable-sourced credentials — fully non-interactive.
- **No workforce-vs-workload constraint** blocks automated acquisition; the
  programmatic STS endpoint is identical in shape.

## Two degenerate shapes

### 1. Caged self-signed JWT (most degenerate — "R4's ghost")

- Hold **one signing keypair**. Configure the workforce OIDC provider with that JWKS
  (uploaded to the provider).
- The harness mints a JWT carrying the right `iss`/`aud`/`sub`/`exp` and posts it
  straight to STS.
- **No IdP server, no Keycloak, no human** — just a keypair and a signature GCP
  checks.
- **Cost:** you own the signing key — it *is* a durable test secret. This is the one
  place a secret creeps back into a heat whose premise is "zero durable secrets
  beyond the payor's RBRO." Acceptable only if quarantined to the test org and
  unreachable from any production path.

### 2. Real test IdP with a non-interactive grant (Keycloak)

- Stand up Keycloak; configure the workforce provider to trust its issuer.
- Obtain the `id_token` via ROPC / password grant, or client-credentials /
  JWT-bearer (federated client auth is supported as of Keycloak 26.x).
- More faithful ("a real IdP issued this"); heavier (a running IdP); the password
  grant is deprecated under OAuth 2.1.

## What degenerate testing CAN and CANNOT prove

**CAN automate** (everything downstream of the federated token is identical
regardless of how the token was minted):

- the **don** / Leg-3 impersonation;
- the **admission-verb suite** (brevet / unseat / attaint / rehearse) — these are
  *governor-wielded*, so they require a **donned governor mantle**, which requires a
  **federated token**. Without a degenerate source they need a human at *every* run.
  **This is where the test-rig decision pays its way**, beyond the proof paces;
- the **autonomous founding proofs** (already payor-driven, no human);
- the machine-verifiable half of the **audit-attribution** read.

**CANNOT substitute for — by construction:**

- the **live device-flow proof**: its entire reason for being is the human browser
  click (RFC 8628, Leg 1). Degenerate it away and you delete the assurance it exists
  to give.
- **Leg-1 reachability** proof.

→ Keep exactly **one thin interactive pace** for the device-flow leg; degenerate
everything downstream of the federated token.

## Where this homes / the trade to resolve

- This is the **Test rig** open decision already named in the federation paddock:
  *"per-run human click vs a test-org-only secret (Keycloak password grant or caged
  token — R4's ghost); decide at movement-4 slating, deliberately."* Movement 4 has
  largely landed → the decision is now **due**, not deferred.
- **Cousin to, but distinct from, the production-headless question**
  (compearance-headless-reconsider, ₢BfAAA): same RFC 8628 async mechanism, different
  home. The production question asks whether *real* compearance must gate on a TTY;
  this asks how *synthetic personas* acquire assizes once keys retire.
- **The crux is the durable-secret trade.** Shape 1 reintroduces a signing key into
  a key-free design. The resolution is not "which is technically possible" (both are)
  but "is a test-org-only signing key an acceptable, quarantined exception, or does
  the per-run human click stay." That judgment is the heat's to settle.

## Standards references

- **RFC 8628** — OAuth 2.0 Device Authorization Grant. Asynchronous by design: the
  device polls while the human authenticates on a *separate* device — which is why
  "human present" need not mean "controlling terminal present."
- **RFC 8693** — OAuth 2.0 Token Exchange (the STS `grant_type`).

## Sources (URLs)

GCP Workforce Identity Federation:
- Obtain short-lived tokens — https://docs.cloud.google.com/iam/docs/workforce-obtaining-short-lived-credentials
- Configure Workforce Identity Federation — https://docs.cloud.google.com/iam/docs/configuring-workforce-identity-federation
- Overview — https://docs.cloud.google.com/iam/docs/workforce-identity-federation
- Troubleshoot — https://docs.cloud.google.com/iam/docs/troubleshooting-workforce-identity-federation

Keycloak:
- JWT Authorization Grant — https://www.keycloak.org/securing-apps/jwt-authorization-grant
- Federated client authentication (2026) — https://www.keycloak.org/2026/01/federated-client-authentication
- Token exchange — https://www.keycloak.org/securing-apps/token-exchange
- Password grant walkthrough — https://dulanjanwijesekara.medium.com/keycloak-generate-access-token-using-password-grant-and-oauth-2-0-credentials-eb082e7903f3

IETF standards:
- RFC 8628 (Device Authorization Grant) — https://datatracker.ietf.org/doc/html/rfc8628
- RFC 8693 (OAuth 2.0 Token Exchange) — https://datatracker.ietf.org/doc/html/rfc8693

## Cross-references

- Spike V5 (paper-confirmed Keycloak-in-a-crucible programmatic flow):
  `Memos/memo-20260612-federation-legs-spike-findings.md`
- The federation paddock's **Test rig** section — the open decision this finding
  feeds.
