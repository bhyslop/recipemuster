# Memo — ₣Bf static negative-authorization test rig (headless multi-subject)

- **Date:** 2026-06-19
- **Author:** Claude Opus 4.8, in conversation with Brad
- **Heat:** ₣Bf (federation-evolution holding paddock)
- **Status:** design haul from the ₣BZ ₢BZAAY mount conversation; recorded for ₣Bf's
  degenerate-federation work. Not built; gated on ₣Bf's config-forks and Fable per the
  heat posture. Provenance, not authority.

> Memos are provenance, never authority. The durable facts here — the two negative-test
> boundaries, the never-churn doctrine, the degenerate-IdP spectrum — belong in the ₣Bf
> paddock (terse leans) and the RBS0 federation subdocs once the heat builds. This memo is
> the reasoning trail.

## What this records

A design conversation during the ₣BZ freehold-substrate mount (260619) surfaced the shape of
₣Bf's negative-authorization testing. It refines the existing degenerate-test-federation and
multiple-federations ideas with a *purpose* and a *doctrine*; the reasoning lives here, the
terse leans belong in the ₣Bf paddock at its next groom.

## Two boundaries, two test shapes

The negative tests sort into two kinds with different needs:

- **Payor boundary** ("a non-payor can't do payor things") is largely a
  CREDENTIAL-POSSESSION boundary, not a distinct-principal one. The payor IS the OAuth RBRO
  refresh token; every federated principal lacks it, and the mantle SAs are depot-scoped (no
  org-level `workforcePoolAdmin`, no project-create/billing). So this is testable with ONE
  identity — don a mantle, attempt a payor op, be denied by the mantle's scope. No second
  subject needed.
- **Inter-citizen / inter-mantle denial** ("a director-mantle citizen is denied governor
  work", "an un-brevetted citizen is denied everything", "B can't unseat A") needs two
  principals with DIFFERENT standing grants — distinct federated subjects. With one real
  Entra `oid` this is fiction; it needs the degenerate federation.

## Static, never-churn — the doctrine carries here

The static-admission doctrine settled for ₣BZ (`memo-20260615-BZ-pace-design.md`, 260619
section) applies identically in ₣Bf: provision differentiated standing citizens ONCE, test
negatives as never-granted standing states, never mutate IAM mid-test, assert revocation
correctness by IAM read-back rather than don-and-wait. The degenerate federation just
supplies the multiple standing SUBJECTS the differentiation needs.

## The degenerate-IdP spectrum (control vs realism)

Two shapes, both headless, both local, neither needing the federated pipeline to exist:

- **Self-signed caged JWT** — no IdP software at all: a keypair + openssl (the marshal
  already mints keys), public JWKS uploaded to the workforce provider, tokens signed locally
  and POSTed to STS. Maximum control, no container, no chicken-and-egg, least realism (we
  impersonate an IdP rather than run one).
- **Keycloak (self-hosted test IdP)** — a real conformant OIDC server, run as a container or
  a bare JVM, headless via a non-interactive grant (password / client-credentials), JWKS
  uploaded. More realistic token-endpoint + JWKS path; more moving parts. Spike V5
  paper-confirmed the uploaded-JWKS STS exchange (console sign-in cannot ride uploaded JWKS —
  programmatic flow only, which is the point).

## The container chicken-and-egg (and why it doesn't bite)

Concern: the federation authorizes the cloud container pipeline, so a test IdP running in a
container looks circular. It isn't:

- Keycloak-as-a-local-container needs only the local runtime, never the federated pipeline.
  The egg bites only if its image comes THROUGH the federated GAR/summon path — pull it from
  upstream (or kludge it) and there is no cycle. The dependency is one-way: the rig stands up
  the IdP locally, the IdP then feeds the federation-under-test.
- The self-signed shape sidesteps containers entirely.

## Disposition

₣Bf, refining the degenerate-test-federation idea's purpose. Gated on ₣Bf's front-of-heat
config-forks (the single-test-manor topology and the JWKS-source shape) and on Fable for any
premise-touching vocabulary. The two-identity ambition the ₣BZ conversation reached for is
the capability this enables.

## Sources

- ₣BZ pace-design memo, 260619 static-admission section: `Memos/memo-20260615-BZ-pace-design.md`
- Degenerate-federation test-personas memo (the two degenerate shapes, the can/cannot-prove
  boundary): `Memos/memo-20260616-Bf-degenerate-federation-test-personas.md`
- Federation configuration model (vendor-agnostic core + mechanism gate):
  `Memos/memo-20260618-Bf-federation-config-model.md`
- Federation-legs spike findings (spike V5 Keycloak-in-a-crucible, the standing-trust
  subject `oid`): `Memos/memo-20260612-federation-legs-spike-findings.md`
