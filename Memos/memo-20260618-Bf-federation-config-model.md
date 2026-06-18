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

## RBS0 subdoc plan — detailed spec-authoring reference

This expands the summary above into the reference the future spec-first pace consults when
it cuts. It is spec-authoring guidance, not paddock or docket content. Two design forks gate
it (homed in the ₣Bf front-of-heat design pace): the programmatic JWKS-source shape (S1) and
the single-test-manor topology — settle those first.

### Marker-scheme note (verified against rbrv_regime.sh / RBSRV)

Model the mechanism discriminator on the vessel regime; point at RBSRV as the pattern home
rather than inventing a marker recipe. Three facts to carry:

- The `//axhrgv_variable` marker means *grouped*, not *gated*: RBSRV's RBRV_RELIQUARY and
  RBRV_EGRESS_MODE sit in their own groups yet apply to every mode. Do not partition
  core-vs-gated by that marker.
- The discriminator is enrolled flat (`buv_enum_enroll`) and its enum values are referenced
  inline in the discriminator's definition body — not separately enrolled quoins.
- The serialized on-the-wire enum tokens are a separate namespace (vessel's are
  `rbnve_bind` / `rbnve_conjure` / `rbnve_graft`). The mechanism discriminator needs a new
  `rbnve_`-parallel wire-value family for its literal tokens — a primary-universe mint
  (Fable's), owing the grep gate.

### S1 — RBSRF (RegimeFederation), UPDATE

- Regime NOTE: the trust divides into a vendor-agnostic core and a single
  acquisition-mechanism gate; the Google-side provider config is the same structure for
  every OIDC IdP, so vendor identity is not a regime field; a programmatic trust's private
  signing key never enters the regime (only the public JWKS, a public identifier); adding a
  real vendor is a guide plus values. State the singleton as the model's *scope* (one active
  federation), not as a settled property.
- The mechanism discriminator: an always-present core field selecting how a citizen acquires
  a federated assize; values interactive | programmatic (placeholders, Fable's). Enrolled
  flat, values inline, per the RBSRV precedent.
- Re-home the interactive-only fields (device endpoint, token endpoint, device scope) into an
  interactive group. Fold the existing "non-Entra needs only new values, no code change"
  rationale into the group prose — that per-field justification is the conviction in
  miniature and would otherwise read orphaned under a gate.
- The JWKS source as the programmatic arm's field — but resolve first (design fork) whether
  it is a programmatic-only uploaded field or a core field with two sub-modes (uploaded
  self-held vs issuer-discovered). A real non-interactive IdP (Keycloak-style) has no
  uploaded JWKS and does not fit a single uploaded-JWKS field. Contract-blocking.
- Issuer field: under the programmatic mechanism the issuer is a self-declared identifier
  matched against the JWT `iss`, not a resolvable /.well-known endpoint — its validation
  degrades from "resolvable https URI" to "well-formed iss-matching string." A first-contact
  trip.
- Human-present premise is mechanism-conditional: the openid-required / offline_access-
  forbidden rule and its human-present rationale belong to the interactive mechanism,
  deliberately voided by the programmatic one (which deletes the human by construction).
- Per-vendor-guide contract line: a vendor guide must yield the vendor-agnostic core values;
  the guide is not a regime and not a tabtarget.

### S2 — RBSMA (manor_affiance), UPDATE

- Mechanism-conditional provider body at provider-create: the interactive branch builds the
  existing issuerUri + webSsoConfig body; the programmatic branch builds the uploaded-JWKS
  body and omits webSsoConfig (a locally-uploaded OIDC JWKS can only be used in the
  programmatic flow, so the two are mutually exclusive — the load-bearing correctness fact).
- Rewrite the existing line-128 NOTE to make webSsoConfig interactive-only rather than
  unconditional.
- The uploaded-JWKS REST field name is a placeholder — confirm it at impl. Doc-confirmed only
  at the gcloud `--jwk-json-path` flag level, not at the REST field name; do not anchor on a
  guessed key.
- Affiance undelete-on-DELETED: on a 200 with state DELETED, undelete rather than skip (the
  workforce-pool-constraints memo gap — RBSMA's soft-delete NOTE anticipates it; the impl
  does not yet honor it).
- Completion clause: under the programmatic mechanism the follow-up is a programmatic STS
  exchange, not a compearance.
- Idempotent-ensure, drift-deferral, scope-boundary NOTEs are mechanism-invariant and stay.

### S3 — NEW subdoc: caged-federation establishment (marshal-only)

- WHO: marshal / test-rig lifecycle — tagged as inheriting the premise-touching payor-health
  fork (Fable-gated), not asserted as a settled role allocation.
- Ordered operation: mint keypair → derive public JWKS → write core regime facts with
  mechanism=programmatic → publish public JWKS to the regime → hand off to affiance's
  programmatic branch (reference S2, do not duplicate the provider-create body).
- The durable-secret quarantine contract (the keystone): private key marshal-fenced, never
  the committed regime or any production path; public JWKS commits.
- First-contact trips (client-id=aud, issuer format, exp skew) — reference the
  degenerate-personas memo.
- Honesty: doc-confirmed to --jwk-json-path, not yet harness-proven end-to-end; this bash is
  what first proves it.
- Must not: enumerate a vendor, require a guide/console, or pre-bake function names / paths /
  algorithm.

### S4 — NEW subdoc: programmatic token acquisition (self-mint → STS)

- Recommend stand-alone, mirroring the spec's existing affiance / compear provision-vs-runtime
  split.
- Precondition: a caged trust established (S3) and affianced (S2 programmatic branch); the
  marshal-fenced private key reachable only to the accessor.
- Mint step: construct an OIDC JWT (iss matches regime issuer, aud matches client-id, sub
  matches the subject claim, fresh exp within skew), sign with the fenced key.
- The STS exchange reuses the existing second leg wholesale — it is mechanism-invariant; this
  subdoc owns only the self-mint of the subject token and ends at the federated token,
  pointing at the existing don path.
- The can/cannot-prove boundary (personas memo): can automate the don, the admission-verb
  suite, the autonomous founding proofs, the audit-attribution read; cannot substitute for
  the live device-flow proof or first-leg reachability.

### S5 — per-vendor setup-guide contract (home, as a lean)

- The guides are handbook (rbh) content, one per vendor (Entra first), authored per HCG.
- Current lean: the binding contract is a single line in RBSRF, not a standalone subdoc — a
  one-sentence contract does not earn a subdoc.
- The guide-family colophon / acronym mint is Fable's.

### S6 — quoin / sub-letter implications (MCM, no mint — hand to Fable)

- Regime category: the mechanism discriminator is a flat enum-enroll core field with inline
  values (RBSRV precedent), not a nested-quoin scheme; the public-JWKS source and group
  labels are grouped fields; the serialized enum-value tokens are a new wire-value family
  parallel to vessel's `rbnve_`, needing its own mint and grep gate.
- Federation civic category: the caged-establishment verb is a civic verb quoin sibling to
  affiance; the programmatic-acquisition verb is a civic verb quoin sibling to compear; the
  marshal-fenced-home noun's category (civic vs test-rig) is Fable's.
- Two new subdoc acronyms (caged-establishment, programmatic-accessor) — Fable's.
- MCM checks before minting: within-domain Y monosemy; grep gate clean repo-wide;
  trodden-word screen on the value words (interactive / programmatic are at risk); asterism
  fit (the federation family is diplomatic / civic — compear, affiance, brevet, attaint,
  citizen, mantle).

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
