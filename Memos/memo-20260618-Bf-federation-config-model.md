# Memo — Federation Configuration Model (vendor-agnostic core, acquisition-mechanism gate)

- **Date:** 2026-06-18
- **Author:** Claude Opus 4.8, in conversation with Brad
- **Heat:** ₣Bf (federation-evolution holding paddock)
- **Status:** conviction recorded — informs the ₣Bf paddock and the RBS0 subdocs this
  heat must provide. Not yet built; gated on ₣BZ completing the single-federation
  implementation. Vocabulary for any new quoin/subdoc named below waits on Fable.

> Memos are provenance, never authority. The durable facts here — the vendor-invariance
> of the GCP-side provider config, and the acquisition-mechanism axis — belong in RBSRF,
> RBSMA, and the new subdocs named at the foot of this memo once the heat builds. This
> memo is the reasoning trail, not the spec home.

## What prompted this

The degenerate test federation (a caged self-signed JWT, to cut the human out of
headless test runs — see the degenerate-federation test-personas memo) forced a
modeling question: how do we represent a federation that is real-Entra in production
but caged in test?

The first framing was a binary — make the federation regime (rbrf) a **variant**
regime (a discriminated `kind` field, fields gated per kind) or a **superset** regime
(one flat schema = the union of both shapes). Brad rejected the binary: it silently
assumed the schema's job is to *enumerate providers* (Entra, Keycloak, Okta, …), and we
were reasoning from exactly two of them. That objection is correct, and the docs below
settle why.

## The deciding finding — the GCP-side provider config is vendor-invariant

Google's Workforce Identity Federation provider configuration is the **same structure
for every OIDC IdP** — only the values differ:

- *"Configuration differs only in the values supplied, not structural organization.
  Whether configuring Microsoft Entra, Okta, or generic OIDC providers, the same flags
  apply with identical semantics."* — Manage workforce providers.
- *"Google's workforce pool provider accepts the same parameter structure regardless of
  IdP — only the values (issuer URL, client credentials, claim names) vary."* —
  Configure WIF with Okta.
- The genuinely vendor-specific work is **in the IdP's own admin console** (app
  registration, claims, scopes) — human, foreign-console work, not a Google-side knob.

The full provider field set (all vendor-invariant in *shape*): `--issuer-uri`,
`--client-id` (must equal the JWT `aud`), `--client-secret-value` (code flow only),
`--attribute-mapping`, `--attribute-condition`, `--jwk-json-path` (uploaded JWKS),
`--web-sso-response-type`, `--web-sso-assertion-claims-behavior`,
`--web-sso-additional-scopes`.

## The reframe — three kinds of variation, only one belongs in the schema

| What varies | Belongs in | Why |
|---|---|---|
| IdP **console setup** (register app, configure claims/scopes) | a **guide**, per vendor | foreign-console human work; not a regime or a tabtarget |
| **opaque values** (issuer URL, client-id, subject claim — `assertion.oid` for Entra, `assertion.sub` elsewhere) | **values in one vendor-agnostic core** | already rbrf.env's stated design ("a non-Entra IdP needs no code change — only new values"); the docs prove it generalizes |
| **token-acquisition mechanism** (interactive device-flow vs programmatic self-supplied JWT) | the **one real schema discriminator** | the only thing that changes the required-field shape *and* the code path in affiance and the accessor |

The discriminator is **mechanism, not vendor.** Vendors are open-ended but collapse
onto a closed, tiny set of acquisition mechanisms; a new vendor never adds a mechanism,
it slots into one.

## The four models scored

| Model | Verdict | Reason |
|---|---|---|
| 1 — variant (discriminated kind) **on vendor** | wrong axis | the kind it discriminates (vendor) is not what varies in the GCP-side schema |
| 2 — superset (union schema) **on vendor** | wrong axis + dead fields | same axis error; also every file carries fields meaningless to its IdP |
| 3 — one regime *type* per vendor | reject | GCP-side config is vendor-invariant, so per-vendor regimes are ~identical copies of one core *and* still cannot hold the console work (that is a guide) |
| 4 — shrink rbrf to IdP-independent + per-vendor guides/bash | **adopt** | continuation of rbrf.env's existing decision; the only residue (mechanism) is a small gate |

The gated-variant *machinery* (`buv_enum_enroll` + `buv_gate_enroll`) survives from
models 1/2 — repurposed onto **mechanism**, where it is load-bearing.

## The landing model

**rbrf = a vendor-agnostic trust core + an acquisition-mechanism gate.**

- *Core (always present):* org/pool/provider id, session-duration, client-id,
  attribute-mapping, issuer (or JWKS source).
- *Gate = interactive:* device-authorization + token endpoints, device scope.
- *Gate = programmatic:* uploaded **public** JWKS (the caged case).
- *Vendor identity: not in the regime at all.* Each real vendor gets a setup **guide**
  (its console) and supplies **values**. Adding a vendor = a guide + values: no schema
  change, no code change.

The caged degenerate federation is then not a vendor at all — it is
`mechanism = programmatic` + a self-held keypair + an uploaded public JWKS. No vendor,
no guide, no console; just the programmatic gate and a marshal tool that mints the
keypair and writes the core facts.

## Machinery grounding

The variant machinery is precedented in-tree: the vessel regime (`rbrv_regime.sh:44`)
enrolls `RBRV_VESSEL_MODE {bind,conjure,graft}` via `buv_enum_enroll`, then gates each
mode's fields behind `buv_gate_enroll RBRV_VESSEL_MODE <value>` (lines 55–77). The
federation mechanism gate is the same pattern on a 2-value (later possibly 3)
discriminator, kept on a **singleton** regime — one active federation per manor, no
family. (Whether the manor ever holds *several* federations at once is the separate
multiple-federations question; it is orthogonal to this config model and stays
Fable-gated.)

## Invariant preserved — "ships committed, no secrets"

rbrf's premise is that every value is a public identifier, so it ships committed. The
model preserves this: the programmatic gate carries the **public** JWKS (public keys
commit fine); the **private** signing key never enters rbrf — it lives in the
marshal-only fenced home. The discriminator splits public config (rbrf) from the one
durable secret (marshal-local) along exactly the right seam.

## Boundary — OIDC only

Vendor-invariance holds for **OIDC**. The one place a genuine *per-protocol* fork would
reappear is **OIDC vs SAML** (a structurally different provider) — but the manor is
OIDC-only, so it is out of scope. If a SAML IdP ever entered, that would be a real
variant: by *protocol*, still not by vendor.

## RBS0 subdocs this heat must provide

(Named by purpose; acronym mints are Fable's.)

1. **RBSRF (RegimeFederation) — UPDATE.** Recast the regime as vendor-agnostic-core +
   acquisition-mechanism gate. Introduce the mechanism discriminator as a quoin
   (interactive | programmatic). Reclassify device endpoints + device scope as
   interactive-gated; the uploaded-JWKS source as programmatic-gated; the core as
   always-present. State explicitly that vendor identity is *not* a regime field.
2. **RBSMA (manor_affiance) — UPDATE.** Affiance branches on the mechanism: interactive
   builds `issuerUri` + `webSsoConfig`; programmatic builds `jwksJson` and omits
   `webSsoConfig` (the uploaded-JWKS-is-programmatic-only constraint). The provider body
   becomes mechanism-conditional.
3. **NEW subdoc — caged federation establishment (marshal-only).** The BCG bash that
   stands up a programmatic caged trust end-to-end: generate keypair, derive public
   JWKS, create pool + provider via `--jwk-json-path`, set `client-id = aud`, write the
   attribute mapping. Owns the durable-secret quarantine contract (private key fenced;
   public JWKS committed).
4. **NEW subdoc — programmatic token acquisition (self-mint → STS).** The runtime
   sibling of the device-flow accessor: for `mechanism = programmatic`, mint/obtain a
   JWT and exchange at STS (no device flow). May fold into #3 or stand alone; conceptual
   sibling of the compearance accessor.
5. **Per-vendor setup-guide contract.** Each real vendor (Entra first, the live one)
   gets a setup guide (the rbw-gPF idea, generalized to one-per-vendor). The guides are
   handbook (rbh) content; their *contract* — "a guide must yield the vendor-agnostic
   core facts" — is a line in RBSRF.

## Sources

- Manage workforce identity pool providers — https://docs.cloud.google.com/iam/docs/manage-workforce-identity-pools-providers
- Configure WIF with Okta — https://docs.cloud.google.com/iam/docs/workforce-sign-in-okta
- Configure Workforce Identity Federation — https://docs.cloud.google.com/iam/docs/configuring-workforce-identity-federation
- Obtain short-lived tokens (Workforce) — https://docs.cloud.google.com/iam/docs/workforce-obtaining-short-lived-credentials

## Cross-references

- Degenerate-federation test-personas memo (the caged mechanism, the can/cannot-prove
  boundary): `Memos/memo-20260616-Bf-degenerate-federation-test-personas.md`
- Workforce-pool quota / soft-delete constraints (the freehold facts):
  `Memos/memo-20260617-BZ-workforce-pool-constraints.md`
- Federation-legs spike findings (the V5 programmatic-STS paper-finding):
  `Memos/memo-20260612-federation-legs-spike-findings.md`
- Spec homes to evolve: RBSRF (`RBSRF-RegimeFederation.adoc`), RBSMA
  (`RBSMA-manor_affiance.adoc`).
