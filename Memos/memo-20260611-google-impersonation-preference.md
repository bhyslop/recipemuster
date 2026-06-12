# Google's Stated Preference for Service Account Impersonation over Keys — Evidence Record

Date: 2026-06-11

TRIAGED 2026-06-12: owned by ₣BZ (office-federation evidence, cited by its paddock) —
outside ₣BH triage scope; no disposition claimed here.

Status: Evidence memo. Substantiates the claim "impersonation is Google's own
recommended pattern," made during the 2026-06-11 federation-architecture
conversation (the office-SA / IAM-frozen-at-levy proposal, alpha-repo heat ₣BZ
context). Written to the beta repo because the alpha working tree was
skirmish-locked at the time.

---

## The claim, made precise

Google does not prefer impersonation *absolutely* — it prefers **eliminating
durable credentials**, and publishes an explicit preference ordering. Within
that ordering:

1. **Attached service accounts** — for code running on GCP resources (VMs, GKE,
   Cloud Build). No credential exists at all.
2. **Workload/Workforce Identity Federation** — for code and humans outside
   GCP. No Google secret exists; trust rides an external token.
3. **Service account impersonation with short-lived credentials** — for an
   authenticated principal that needs a service account's authority.
4. **Service account keys — explicit last resort.**

So the precise form of the claim: **Google prefers impersonation over keys,
strongly, with last-resort framing on keys and default org-policy enforcement
against them since May 2024.** An architecture in which a federated human
impersonates a standing, keyless service account composes mechanisms 2 and 3 —
both recommended — and touches mechanism 4 not at all.

---

## Evidence

### E1 — IAM best practices: keys are last resort, impersonation listed first

[Best practices for using service accounts securely](https://docs.cloud.google.com/iam/docs/best-practices-service-accounts):

- "avoid using service account keys whenever possible"
- "Use service account keys only when there is no viable alternative"
- "authenticating with a service account key introduces a non-repudiation
  threat"
- Lists "impersonate a service account with user credentials" first among
  alternatives, ahead of attached SAs and federation, with keys last.
- Recommends the `Disable service account key creation` org policy outright.

### E2 — Impersonation doc: the direct security comparison

[Use service account impersonation](https://docs.cloud.google.com/docs/authentication/use-service-account-impersonation):

> "Service account impersonation is more secure than using a service account
> key because service account impersonation requires a prior authenticated
> identity, and the credentials that are created by using impersonation do not
> persist."

> "authenticating with a service account key requires no prior authentication,
> and the persistent key is a high risk credential if exposed"

Mechanism reference: [Service account impersonation (IAM)](https://cloud.google.com/iam/docs/service-account-impersonation)
— `generateAccessToken` on `iamcredentials.googleapis.com`, gated by
`roles/iam.serviceAccountTokenCreator`.

### E3 — Key-management best practices: avoid the artifact entirely

[Best practices for managing service account keys](https://docs.cloud.google.com/iam/docs/best-practices-for-managing-service-account-keys):

> "The best way to mitigate these threats is to avoid user-managed service
> account keys and to use other methods to authenticate service accounts
> whenever possible."

> "For situations where you can't use more secure alternatives to service
> account keys, this guide presents best practices…" (the entire guide is
> framed as fallback conduct)

Decision guidance: [authentication decision tree](https://cloud.google.com/docs/authentication#auth-decision-tree).

### E4 — Google's own threat data (blog, Feb 2024)

["Want your cloud to be more secure? Stop using service account keys"](https://cloud.google.com/blog/products/identity-security/want-your-cloud-to-be-more-secure-stop-using-service-account-keys)
(Luis Urena, Google Cloud, 2024-02-23), citing Threat Horizons reports:

- "More than 69% of cloud compromises were caused by credential issues"
  (Q3 2023 Threat Horizons).
- "Nearly 65% of alerts across organizations were related to risky use of
  service accounts."
- 68% of service accounts carried overly permissive roles; in 42% of
  leaked-key incidents, project owners took no corrective action after Google
  contacted them.

### E5 — Preference expressed as default enforcement (the strongest form)

["Introducing stronger default Org Policies for our customers"](https://cloud.google.com/blog/products/identity-security/introducing-stronger-default-org-policies-for-our-customers)
(Google Cloud, 2024-03-20) and
[Disable/enable service account keys](https://docs.cloud.google.com/iam/docs/keys-disable-enable):

- Organizations created **on or after 2024-05-03** receive
  `iam.disableServiceAccountKeyCreation` **enforced by default** ("to decrease
  the risk of exposed service account credentials"), alongside
  `iam.disableServiceAccountKeyUpload` and domain-restricted sharing.
- A vendor's preference is most credible where it ships as the default: new
  GCP orgs cannot mint SA keys without a deliberate policy exception. This is
  also the constraint the alpha repo hit empirically on its own org
  (memo-20260522, alpha repo: the override experiment).

### E6 — The federation tie-in

[Workforce Identity Federation](https://docs.cloud.google.com/iam/docs/workforce-identity-federation)
principals are org-confined; granting one permission to impersonate a service
account is the documented composition for reaching anything an SA can reach.
Federated-token direct access has a per-product support matrix with
limitations
([Identity federation: products and limitations](https://docs.cloud.google.com/iam/docs/federated-identity-supported-services));
an impersonated SA token is a plain SA token and bypasses that matrix
entirely.

---

## Implication for the office-SA architecture

The proposal under discussion (all role/office SAs created and fully bound at
depot levy; humans admitted by being granted impersonation on an office;
keys never minted) sits squarely inside Google's recommended region:

- Standing SAs are not disfavored — **keys** are. Office SAs hold no keys.
- Federated human → STS → `generateAccessToken` composes two recommended
  mechanisms (E2, E6).
- A keyfile front door, if retained for any tier, is by Google's own framing
  the documented last resort (E1, E3) and is structurally rejected by
  secure-by-default orgs (E5) — aligning any free/paid tier split with
  Google's own decision tree rather than against it.

## Open verification items (spike-grade, not settled here)

- Artifact Registry and Cloud Build status in the workforce federated-token
  support matrix (page truncated during research; impersonation moots it but
  verify).
- Audit-log shape under impersonation: `delegationInfo` content naming the
  human principal.
- Office-token lifetime ceiling (1h default; 12h via
  `iam.allowServiceAccountCredentialLifetimeExtension` — confirm).
- Whether `authenticationInfo.serviceAccountKeyName` provides per-key
  attribution, if a keyfile front door is ever retained.

## Sources

- https://docs.cloud.google.com/iam/docs/best-practices-service-accounts
- https://docs.cloud.google.com/docs/authentication/use-service-account-impersonation
- https://cloud.google.com/iam/docs/service-account-impersonation
- https://docs.cloud.google.com/iam/docs/best-practices-for-managing-service-account-keys
- https://cloud.google.com/docs/authentication#auth-decision-tree
- https://cloud.google.com/blog/products/identity-security/want-your-cloud-to-be-more-secure-stop-using-service-account-keys
- https://cloud.google.com/blog/products/identity-security/introducing-stronger-default-org-policies-for-our-customers
- https://docs.cloud.google.com/iam/docs/keys-disable-enable
- https://docs.cloud.google.com/iam/docs/workforce-identity-federation
- https://docs.cloud.google.com/iam/docs/federated-identity-supported-services
