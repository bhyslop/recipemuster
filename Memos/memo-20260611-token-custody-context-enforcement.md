# Token Custody on the Workstation, and Server-Side Context Enforcement — Evidence Record

Date: 2026-06-11

TRIAGED 2026-06-12: owned by ₣BZ (office-federation evidence, cited by its paddock) —
outside ₣BH triage scope; no disposition claimed here.

Status: Evidence memo, companion to
`memo-20260611-google-impersonation-preference.md`. Substantiates the
token-custody layer model presented in the 2026-06-11 federation-architecture
conversation (alpha-repo heat ₣BZ context) and builds out the server-side
context-enforcement story. Contains one correction to that presentation,
flagged below.

---

## Question 1 — Podman credential helpers: yes

Podman consumes the same credential-helper ecosystem Docker does, via
`containers-auth.json`:

- The auth file supports a per-registry **`credHelpers`** map whose values name
  a `docker-credential-<suffix>` executable on `$PATH` — the same helper
  binaries Docker uses (`secretservice`, `pass`, `osxkeychain`, `wincred`,
  vendor helpers like `docker-credential-acr-env`).
  [containers-auth.json(5) man page](https://www.mankier.com/5/containers-auth.json) ·
  [podman-login docs](https://docs.podman.io/en/latest/markdown/podman-login.1.html)
- Podman supports `credHelpers` (per-registry) but **not** Docker's global
  `credsStore` setting — helpers must be named per registry.
  [Podman credential helpers walkthrough](https://linuskarlsson.se/blog/podman-credential-helpers/)
- Default auth path on Linux is `${XDG_RUNTIME_DIR}/containers/auth.json` —
  typically **tmpfs**, so podman's out-of-the-box token stash is already
  memory-backed and reaped at logout, unlike Docker's `~/.docker/config.json`.
  Fallback search order continues to `~/.config/containers/auth.json` and
  Docker's paths.

Implication: a GAR pull token can ride a keystore-backed helper identically
under podman and Docker; one helper election covers both runtimes.

---

## Question 2 — the custody layer model, substantiated

### L1 — Plaintext file in the user profile (industry default)

Major cloud CLIs (gcloud, aws, az) default to bearer material in plaintext
files/sqlite under the user profile, 0600. This is the floor, not a defect of
any one tool. (Common knowledge; no single citable page — verifiable by
inspecting `~/.config/gcloud/`, `~/.aws/`.)

### L2 — OS keystores via credential helpers

Per-platform system stores: macOS Keychain, Windows Credential Manager/DPAPI,
Linux Secret Service (gnome-keyring/KWallet) with the kernel keyring as the
headless fallback. The container ecosystem's
[credential-helpers](https://www.mankier.com/5/containers-auth.json) are the
in-domain precedent (Q1 above).

What L2 buys: no plaintext at rest; invisible to other users, backups, sync
tools, casual forensics. What it does not buy: processes running **as the
same user** can read the store, and the legitimate user can always extract
and exfiltrate. Keystores defeat theft-at-rest, not insiders or same-user
malware.

### L3 — Sender-constrained tokens (proof-of-possession)

Industry direction: bind the token to a key the thief's machine lacks — DPoP
(RFC 9449), mTLS-bound tokens (RFC 8705), binding keys held in hardware
(TPM / Secure Enclave). **GCP APIs do not accept DPoP**; Google's
embodiment of this layer is **certificate-based access (CBA)**:

- CBA requires a verified X.509 **device certificate** on every API request
  via mTLS — "both user credentials and the original device certificate"
  must be present, explicitly targeting credential theft.
  [CBA overview](https://cloud.google.com/beyondcorp-enterprise/docs/securing-resources-with-certificate-based-access)
- [Endpoint Verification](https://docs.cloud.google.com/access-context-manager/docs/cba-endpoint-verification-certs)
  can provision self-signed device certificates **without a PKI**; certs land
  in Keychain (macOS), the certificate store (Windows), filesystem (Linux).
- CLI support exists:
  `gcloud config set context_aware/use_client_certificate true` plus the
  `enterprise-certificate-proxy` component.
  [Enable CBA in client apps](https://cloud.google.com/beyondcorp-enterprise/docs/enable-cba-client-apps)
- Licensing caveat: CBA is a Chrome Enterprise Premium / BeyondCorp
  Enterprise feature — paid, per-user.

**Correction to the conversation's presentation (load-bearing):**
device-information access levels — the mechanism CBA enforcement rides — are
**not available under Workforce Identity Federation**:

> "Access levels based on device information are not available when using
> Workforce Identity Federation. You can still use request-context-based
> access levels with conditions on IP address, and time and date."
> — [Workforce Identity Federation docs](https://docs.cloud.google.com/iam/docs/workforce-identity-federation)

So for federated operators, the device-bound layer is currently **off the
table**; the available context signals are IP address and time/date. The
device-cert story applies to Google-identity principals (and to any future
where Google lifts the limitation — a named revisit trigger).

### L4 — Server-side perimeter enforcement (VPC Service Controls)

The layer that works regardless of workstation cooperation, and the only one
that defeats deliberate exfiltration:

- VPC-SC draws a service perimeter around the depot project's APIs; requests
  failing the perimeter's ingress rules are rejected **even with a valid
  token**. Google states the mitigation explicitly: default restrictions
  protect against "lost or stolen credentials from being used to exfiltrate
  information within Service Perimeters."
  [VPC Service Controls](https://cloud.google.com/security/vpc-service-controls)
- **Artifact Registry is a supported product** with documented perimeter
  guidance: [Protect repositories in a service perimeter](https://docs.cloud.google.com/artifact-registry/docs/securing-with-vpc-sc).
  An emailed retriever token used from outside the allowed IP access level
  dies at the perimeter, not at IAM.
- Ingress rules compose with Access Context Manager access levels
  (IP ranges, identities):
  [Context-aware access with ingress rules](https://cloud.google.com/vpc-service-controls/docs/context-aware-access)
- **Workforce-federation compatibility confirmed**: workforce pool users can
  be named in ingress/egress rules; products supporting both WIF and VPC-SC
  "operate as documented." One wrinkle: STS calls whose audience is the
  (org-level) workforce pool need an **egress rule**, since org-level
  resources can't sit inside a perimeter.
  [VPC-SC supported products and limitations](https://docs.cloud.google.com/vpc-service-controls/docs/supported-products)

### Adjacent finding (multi-IdP discussion)

[IAP with Workforce Identity Federation](https://docs.cloud.google.com/iap/docs/use-workforce-identity-federation)
permits only **one workforce pool with one provider** per application —
evidence that Google's own products assume the one-live-provider posture the
conversation elected as the cinch candidate.

---

## Synthesis for RBK

| Layer | Verdict for RBK |
|---|---|
| L1 file scratch, per-session, 0600 | MVP. Same threat class as today's RBRA, with an ≤12 h fuse. |
| L2 keystore behind the accessor seam; `credHelpers` for registry pulls (covers podman + Docker with one election) | First upgrade — cheap, transparent, kills the at-rest/accident class. Podman's tmpfs default on Linux is already most of the way there. |
| L3 device-bound (CBA) | **Blocked for federated principals** by the device-access-level limitation. Revisit trigger: Google lifts it, or a customer runs Google-identity principals. Licensed (Chrome Enterprise Premium) regardless. |
| L4 VPC-SC perimeter, IP-based ingress levels | The working anti-exfiltration layer under federation, server-side, workstation-blind. Org-grade ceremony; awkward for roaming solo operators (home IPs). Defer with the named trigger: first multi-operator org customer or first audit requirement. |

Honest boundary, restated: against a determined *authorized* insider,
workstation custody is theater — they can exfiltrate work product regardless.
L2 addresses accident and theft; L4 addresses replay-from-elsewhere; nothing
addresses the insider with legitimate access except audit (every use is
logged with subject and source IP) and short lifetimes.

## Open verification items

- Whether the workforce device-access-level limitation has a lift roadmap
  (re-check at federation implementation time).
- Exact CEP/BeyondCorp licensing unit required for CBA.
- The STS egress-rule shape for a depot perimeter with workforce login
  (touched in supported-products docs; needs a worked configuration).

## Sources

- https://www.mankier.com/5/containers-auth.json
- https://docs.podman.io/en/latest/markdown/podman-login.1.html
- https://linuskarlsson.se/blog/podman-credential-helpers/
- https://cloud.google.com/beyondcorp-enterprise/docs/securing-resources-with-certificate-based-access
- https://docs.cloud.google.com/access-context-manager/docs/cba-endpoint-verification-certs
- https://cloud.google.com/beyondcorp-enterprise/docs/enable-cba-client-apps
- https://docs.cloud.google.com/iam/docs/workforce-identity-federation
- https://cloud.google.com/security/vpc-service-controls
- https://docs.cloud.google.com/artifact-registry/docs/securing-with-vpc-sc
- https://cloud.google.com/vpc-service-controls/docs/context-aware-access
- https://docs.cloud.google.com/vpc-service-controls/docs/supported-products
- https://docs.cloud.google.com/iap/docs/use-workforce-identity-federation
