# Federation Canon — Operator Workforce-Identity Federation, Final Shape

Date: 2026-06-09

Status: **CANON, superseded in part (2026-06-12)** by
`memo-20260612-office-federation-conversion.md`: **D1 is retired** — the keyfile/citizen
tier is canceled unbuilt and federation is the **single** operator tier (no mode enum);
the direct-resource-grant model below is replaced by **office-SA impersonation**
(capability bindings frozen on levy-minted office SAs; operators hold one
`serviceAccountTokenCreator` binding per office and mint office tokens via
`generateAccessToken`). D2, D3, D4, D6 survive unchanged; D5's deferred
capability-set-keyed lifetime is **activated** (retriever at the 12 h ceiling). The
login mechanics (Setup, Leg 1, Leg 2, the accessor seam, the orchestration pattern)
remain authoritative, with a third leg (`generateAccessToken`) appended after Leg 2.

Original status: the single authoritative home for the **federation** operator-credential
tier (the paid, organization-required tier — keyless, no operator secret on disk). This document
supersedes `memo-20260527-operator-credential-models.md` (folded in whole) and **R4** of
`memo-20260522-org-affiliated-credential-reorientation.md` (the test-rig refresh-token residual —
retired by D2 below).

The **citizen / keyfile tier** (the free, no-org tier being *built now* under heat ₣BZ) is **not**
restated here — it lives in `memo-20260605-citizen-capability-model.md` + the ₣BZ paddock. This canon
points at it; the two tiers share a capability layer and an accessor seam but never coexist in one
depot.

---

## How to read this

Federation lets a depot operator reach GCP **without any secret on disk** — they prove identity by a
browser sign-in to their organization's own IdP, and the tool exchanges that proof for a short-lived
Google token. It is the heavyweight successor to the keyfile tier, gated by one structural fact: the
trust object (a *workforce pool*) is organization-level, so federation requires a GCP org — which is
the free/paid line.

**If you read only one thing, read "The decisions" below.** The single decision that shapes
everything (D2) is that *a human is always present to start a run* — which lets the tier hold **zero
durable secrets, anywhere**, shipped or test-rig.

Where the keyfile tier lives: `memo-20260605` + ₣BZ paddock. Where the reasoning behind these
decisions lives: `memo-20260522` (R1–R5). This canon **states** decisions; it points to the
derivation rather than repeating it.

---

## The decisions (the record)

The cinched calls. D2, D4–D6 are this design line's; the rest are carried forward from the source
memos and confirmed.

- **D1 — Two permanent tiers, one per-depot mode enum.** Keyfile (free, no-org) and federation
  (paid, org-required) both ship forever; a depot runs exactly one, selected by a mode enum homed in
  RBRD (tamper-evident — see Regime homes). The organization requirement is the paywall.

- **D2 — A human is always present to start a run; therefore the federation tier holds NO persisted
  refresh token, anywhere.** (Operator decision, 2026-06-09.) The browser sign-in happens at run
  kickoff (and again if a run outlives the session cap). This **retires R4's "single acknowledged
  durable-secret residual"** — the caged test-rig refresh token is not built. The federation tier
  becomes **fully secret-free**: no SA key (it has none by definition), no refresh token, only an
  ephemeral in-process access token. *Tradeoff, stated honestly:* truly **unattended scheduled**
  federation runs (cron at 3am, nobody present) are **not supported** — they would require the
  refresh token D2 declines. Revisit-trigger in Open/deferred.

- **D3 — One identity-keyed accessor; downstream model-blind.** Every GCP call resolves a token via a
  single accessor keyed by *identity* ("a token as alice"), which branches *inside* by tier. Every
  call site adds `Authorization: Bearer <token>` and knows nothing of the token's origin. The
  accessor treats its **credential source as opaque and pluggable** — it must not assume a fresh
  per-call mint (the one constraint ₣BZ's keyfile-only accessor must honor to keep federation cheap).

- **D4 — Regime homes obey the XOR authorship rule.** Every `RBRx.env` is **either** human-edited
  **or** infrastructure-maintained — never both. Identity selector → human-edited RBRS field;
  durable credential material → full-auto RBRA (keyfile only); mode + federation facts + lifetime
  policy → tamper-evident RBRD; ephemeral access token → not a regime at all. (See Regime homes.)

- **D5 — Token lifetime is Governor-administered, in tamper-evident RBRD.** Not a per-station human
  knob. Bounded by the GCP ceilings (Palisade limits: 1h keyfile, 12h federation) — neither Governor
  nor Payor can exceed them. With D2 there is no month-scale "Payor escalation"; long runs that
  outlive the cap get a fresh human click.

- **D6 — Federation is fully secret-free; the tier contrast is sharp.** Keyfile = exactly **one**
  durable secret (the SA key). Federation = **zero**. This is the security payoff of the org
  requirement and of D2 together.

---

## What federation is — the two tiers

| | **Keyfile** (free) | **Federation** (paid) |
|---|---|---|
| Operator secret on disk | SA key (RBRA) | **none** |
| GCP organization | not required | **required** |
| Identity proof | possession of the key | fresh IdP sign-in (browser) |
| Revocation | rotate / delete the key | disable the principal at the IdP |
| Capabilities | grants on the citizen's SA | grants on the federated `principal://` subject |
| Token mint | signed-JWT exchange | STS token-exchange |
| Durable secrets held by RBK | one (the key) | **zero** (D2) |

Both tiers terminate in the **same** artifact — a short-lived Google access token — and everything
downstream is identical and model-blind. They differ only in identity-kind (citizen ↔ federate) and
token mint (signed-JWT ↔ STS).

**Why org-only.** Federation's trust object, the **workforce pool**, can only be created at the
**organization** level; a plain project cannot hold one. A qualifying org is *free* via Cloud
Identity once the owner verifies control of a DNS domain — so the real prerequisite is "the operator
controls a domain," not "the operator pays Google." The keyfile tier exists for operators who cannot
or will not verify a domain.

**Workforce vs Workload — do not conflate.** Federation here is **Workforce** Identity Federation
(human identities, org-level pool). It is distinct from the **envoy's** egress federation, which is
**Workload** Identity Federation (machine identities, GCP→AWS delivery). Same mechanism family,
opposite direction and actor; they never chain for one actor.

---

## Setup — Payor / org-admin, one-time

The Payor, acting as org admin, configures trust once. `gcloud`/REST touches **setup only** — never
runtime. (Visual companion: `diagrams/rbdgs_federation-setup.puml`.)

1. Create an org-level **workforce pool**.
2. Create a **provider** in the pool trusting an external IdP — any compliant **OIDC or SAML 2.0**
   issuer (Entra, Okta, AD FS, Ping, Auth0, self-hosted Keycloak/Dex, or GCP Identity Platform as an
   in-house IdP).
3. Set an **attribute mapping** pinning `google.subject` to an *immutable* IdP claim (`sub`/`oid`,
   ≤127 bytes). A *mutable* claim invites spoofing.
4. Grant depot capability to the federated principal:
   `principal://iam.googleapis.com/locations/global/workforcePools/POOL/subject/SUBJECT`.

Identity proofing lives **entirely** in the IdP. RBK grants capability only to an already-known
principal; the role-to-identity roster (the **Terrier**, in citizen-model terms — see
`memo-20260605`) remains RBK-homed. (Homing reshaped by the ₣BZ paddock, 260609 evening,
which is authoritative: per-depot ledger files in a *Manor bucket*, payor-created at levy,
governor-writeable own-depot-only — no longer depot-resident.) Recording grant *intent* in the
Terrier parallels the IAM grant (intent-first); it is **not** consulted at login (see The
accessor).

---

## Login — operator, per session, human present

Two legs, both inside RBK's existing curl/openssl/jq stack. (Visual companion:
`diagrams/rbdgl_federation-login.puml`.)

**Leg 1 — IdP auth, the OAuth 2.0 Device Authorization Grant (RFC 8628).** Headless-friendly: the
browser need not be on the box.

1. The tool POSTs the IdP's device-authorization endpoint → `user_code`, `verification_uri`,
   `device_code`, `interval`.
2. The tool prints **"go to `<uri>`, enter `WXYZ-1234`."** The human authorizes in **any** browser,
   on any device, proving themselves to *their own* IdP. **Google is not involved in Leg 1.**
3. The tool polls the IdP's token endpoint until the human approves → returns the **OIDC ID token
   (JWT)**.

   - **Clickable / QR upgrade.** Where the IdP returns the optional `verification_uri_complete`
     (the URI with the `user_code` embedded — Entra/Okta/Auth0/Keycloak/Zitadel all do), the tool may
     print a clickable link or QR instead of "enter this code." This removes the *typing*, not the
     *approval*.
   - **The human approval is load-bearing and cannot be encoded away.** Even with the complete URI,
     RFC 8628 still has the server confirm the `user_code` matches the device (anti-phishing), and
     the approval *is* the identity proof — it is what replaces "possession of the key." Removing it
     would turn federation back into a stored-credential model.
   - **No `offline_access` scope is requested** (D2) — so the IdP returns **no refresh token**.

**Leg 2 — STS exchange (pure REST against `sts.googleapis.com`, RFC 8693):**

```
POST https://sts.googleapis.com/v1/token        # send NO Authorization header
  grant_type         = urn:ietf:params:oauth:grant-type:token-exchange
  audience           = //iam.googleapis.com/locations/global/workforcePools/POOL/providers/PROVIDER
  subject_token      = <IdP ID token from Leg 1>
  subject_token_type = urn:ietf:params:oauth:token-type:id_token
  → short-lived Google access_token
```

Leg 2 has no signing — it POSTs a token RBK already holds. Then bearer the access token against any
GCP API.

**Session cap.** A Workforce session is configurable **15 min – 12 h, default 1 h** — a hard GCP
ceiling. A run that outlives its session does **not** extend the token; the human re-clicks Leg 1.
Under D2 there is no headless re-mint, by design.

---

## The Claude Code orchestration pattern (secret-free kickoff)

A non-trusted orchestrator (e.g. a Claude Code session driving a test run) can start an
**authenticated** run without ever holding a secret:

- The orchestrator runs the tabtarget that initiates Leg 1 and surfaces only the **`user_code` +
  `verification_uri`** — neither is a secret (single-use, useless without the human's own IdP auth).
- The **human clicks in their own browser** and approves. Nothing is pasted into the orchestrator.
- The resulting **access token lives inside the spawned tabtarget process**, not in the orchestrator's
  conversation context.

So the operator's mental model holds: *a Claude Code session kicks off an authenticated run on one
secret-free human click.* This is the device flow's central virtue — the requesting agent is
decoupled from the authenticating browser.

---

## The accessor — the single seam

(Visual companion: `diagrams/rbdgm_federation-seam.puml`.) One function resolves "give me a token as
`<identity>`," branching inside by the depot's mode:

- **keyfile** → resolve identity → RBRA key → **signed-JWT** exchange → access token.
- **federation** → device flow (Leg 1) → STS exchange (Leg 2) → access token.

Downstream is already model-blind. The branch point is the accessor's **credential source**, which is
*pluggable* (D3): for federation it drives the device flow; for keyfile it mints from the key. The
accessor must never assume a per-call mint — it caches the live session's token (≤ session cap) so
the many tabtargets of one run reuse it without re-prompting. **It must fail loud, never silently
prompt, if no live session exists in a context where a human cannot click** (the headless
fail-fast membrane).

---

## Regime homes & the XOR authorship rule (D4)

What a workstation holds, decomposed so each file is *either* human-edited *or* infra-maintained:

| What | Home | Authorship | Notes |
|---|---|---|---|
| **Identity selector** ("this home is alice") | RBRS field (`RBRS_CITIZEN`) | **human** | inert, singular per home (one home = one operator), stable |
| **Durable credential material** | RBRA | **infra (full-auto)** | *keyfile only.* Delivered by add/rekey; operator never edits. Federation has none. |
| **Mode enum + federation facts** (pool, provider, audience) + **lifetime policy** | RBRD | **infra (Payor/Governor-inscribed)** | tripwire-protected ⇒ tamper-evident; one mode per depot |
| **Ephemeral access token** | — | — | **not a regime** — runtime scratch (process/keychain, ≤ session cap) |

Consequences worth stating:

- **RBRA goes full-auto** in the citizen model — its sole authors are the `add`/`rekey` verbs. Any
  human-tunable that rides it today (the `RBRA_TOKEN_LIFETIME_SEC` knob is the suspect) **must be
  evicted** to keep RBRA cleanly infra-owned — and its new home is the Governor-administered lifetime
  policy in RBRD (D5), not a station field. The XOR rule *forces* this eviction.
- **Shared workstation is out of scope by operator rule:** distinct operators must have distinct
  homes/filesystems. So a regime's scope (a home) is single-citizen, and RBRA is a true per-home
  singleton — the role-indexed manifold of today collapses to one.

---

## Token lifetime policy (D5)

Lifetime is **Governor-administered** and lives in tamper-evident **RBRD**, so a compromised station
cannot widen its own token window. It is bounded by GCP's hard ceilings — **1 h** (keyfile signed-JWT)
and **12 h** (federation Workforce session) — which are Palisade limits no role can exceed.

- **Where it earns its keep:** in *federation* the access-token window is the *only* exposure, so the
  knob is load-bearing. In *keyfile* the SA key on disk dwarfs the token as the real risk, so a tight
  TTL buys little — implement it for uniformity (via RBRD, honored at mint), but do not over-invest.
- **No month-scale escalation:** under D2 there is no refresh token, so the earlier-explored
  "special Payor operation to greatly extend lifetime for CI/CD" is **retired**. Long runs get a human
  re-click, not a longer token.
- *(Deferred refinement, not built:* lifetime keyed by **capability-set** — a low-blast-radius
  retriever token allowed a longer window than a high-blast-radius governor token. Ties to the
  blast-radius cinch; hold, don't build.*)*

---

## Relationship to the citizen (keyfile) tier

Federation is the future **swap**, not a rewrite: the citizen model's capability layer
(capability-sets, grant/revoke, the Terrier, the audit) is built **tier-blind**, so the
federation switch reshapes nothing in it — it changes only **identity-kind** (citizen → *federate*)
and **token mint** (signed-JWT → STS). Identity verbs branch by tier; capability verbs do not. The
*federate* identity is owned by the IdP (no SA minted, no key, no rekey — the IdP owns rotation;
remove = de-register + revoke-all). Full mechanism: `memo-20260605-citizen-capability-model.md` + ₣BZ
paddock.

---

## What this supersedes / points to (the map)

| Document | Disposition |
|---|---|
| `memo-20260527-operator-credential-models.md` | **Folded here in whole.** Now a superseded-banner pointer. |
| `memo-20260522` **R4** (test-rig refresh token) | **Superseded by D2** (no refresh token; fully secret-free). |
| `memo-20260522` R1–R3, R5 + §7–§8 | **Derivation of record** — the human-vs-unattended axis and resolutions. Point-to, don't restate. |
| `memo-20260605-citizen-capability-model.md` + ₣BZ paddock | **The citizen/keyfile tier — the active build.** Separate timeline; not absorbed here. |
| `memo-20260427-google-native-human-auth.md` | Impersonation mechanism detail; its human-auth *framing* was already superseded (20260522 R2). |
| RBSHR "Operator federation" roadmap entry | This canon is its detail; the entry now points here. |

---

## Open / deferred (with revisit-triggers)

- **Unattended scheduled federation runs** — *deliberately unsupported* per D2 (no refresh token).
  *Revisit-trigger:* a concrete need for cron-fired federation CI with no human present; reintroducing
  R4's caged, firewall-bound, centrally-revocable refresh token is the known path, to be re-decided
  then.
- **The federation-setup verb's exact API calls** (pool/provider/attribute-mapping creation) and the
  chosen IdP(s) for the project's own test org.
- **Concrete credential-source wiring** for the interactive human (device-flow output → what the
  accessor's token resolver consumes) and the headless **fail-fast** behavior when no human can click.
- **RBRD field names** for the mode enum, federation facts, and the Governor lifetime policy.
- **Diagram Source: lines** — `diagrams/rbdg{l,s,k,m}_*.puml` still cite `memo-20260527`; repoint to
  this canon and re-render on the next pluml fixture run (container work, deferred from this edit).

---

## Sources

- `memo-20260527-operator-credential-models.md` (folded), `memo-20260522-org-affiliated-credential-reorientation.md` (R1–R5 derivation), `memo-20260605-citizen-capability-model.md` (citizen tier).
- Diagrams: `diagrams/rbdgl_federation-login.puml`, `rbdgs_federation-setup.puml`, `rbdgk_keyfile-login.puml`, `rbdgm_federation-seam.puml`.
- [Workforce Identity Federation — overview](https://docs.cloud.google.com/iam/docs/workforce-identity-federation) · [Configure Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/configuring-workforce-identity-federation) · [Obtain short-lived tokens](https://docs.cloud.google.com/iam/docs/workforce-obtaining-short-lived-credentials).
- [RFC 8628 — Device Authorization Grant](https://datatracker.ietf.org/doc/html/rfc8628) · [RFC 8693 — Token Exchange](https://datatracker.ietf.org/doc/html/rfc8693).
- [Cloud Identity editions](https://docs.cloud.google.com/identity/docs/editions) (free org via domain verification).
