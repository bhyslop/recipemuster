## Shape

This heat exists because RBK's credential and resource model was designed against an **org-less** GCP account (the original gmail payor) and breaks under a **secure-by-default organization**. Google enforces a security-baseline org-policy bundle automatically on every organization created on/after 2024-05-03; scaleinvariant.org is such an org, while org-less accounts and older orgs never see it. Moving the payor under the org is what activated the friction — a structural mismatch, not incidental breakage. The depot *infrastructure* (project, pools, builds, GAR, bucket) levies cleanly under the org; the conflict is confined to the operational-role credential layer.

## Constraint bundle vs RBK

The secure-by-default managed constraints and their RBK bearing:

- `iam.managed.disableServiceAccountKeyCreation` — confirmed blocker. RBRA credentials are downloadable SA keys; governor mantle and retriever/director invest each generate one, refused HTTP 400 "Key creation is not allowed."
- `iam.allowedPolicyMemberDomains` — probable. Restricts IAM-grant members to org-domain identities; RBK grants to SAs and Google-managed agents need auditing.
- `iam.managed.disableServiceAccountKeyUpload` — related family; RBK creates rather than uploads, likely clear.
- `iam.automaticIamGrantsForDefaultServiceAccounts` — verify RBK never relies on default-SA Editor.
- `storage.uniformBucketLevelAccess` — depot bucket already created under it; confirm no per-object ACL use.
- `essentialcontacts.managed.allowedContactDomains`, `compute.managed.restrictProtocolForwardingCreationForTypes` — not exercised by RBK; low risk.

## The strategic fork (undecided — why this heat is stabled)

1. Override path — project-scoped org-policy exceptions for the biting constraints. Unblocks fast. Costs: needs Organization Policy Administrator (owner insufficient); managed constraints' project-level override of an inherited org policy is reportedly stubborn (managed-vs-legacy double-lock). Leaves RBK dependent on long-lived SA keys, against the platform's direction.

2. Identity / keyless path — re-express retriever and director as Google identities so no downloadable SA key is ever created. This *dissolves* the confirmed blocker rather than overriding it: the constraint forbids key creation, and this path creates none. A full design already exists in `Memos/memo-20260427-google-native-human-auth.md` (Approach C, recommended): browser OAuth reusing the existing payor machinery, then `iamcredentials …:generateAccessToken` impersonation; the human-to-role binding is a Google email on the SA's IAM policy (`roles/iam.serviceAccountTokenCreator`, member `user:...`). The SA principal, role taxonomy, and audit identity are preserved; distribution dissolves; revocation is one IAM-binding deletion. The memo also rejects the gcloud-SDK path (Approach B) for RBK's minimal-stack (curl/openssl/jq) reasons.

   Gap the memo does not close: it covers human-driven operations only and explicitly leaves headless paths (Cloud Build itself, automated fixtures, Ifrit scenarios, the gauntlet) on keyfiles-or-workload-identity. Under a secure-by-default org those automated paths stay blocked, so this heat must add a **workload-identity-federation** story for them. So ₣BS scope = memo Approach C (human-driven roles) + a WIF answer (headless/automated), not the memo alone.

## Constraints

- Bash paces read **BCG** (Bash Console Guide) first. `rbgp_Payor.sh` is a complex BCG-compliant module; do not write bash against it without the guide.

## What done looks like

A decision on override vs identity-keyless (plausibly override-to-unblock now, keyless as the durable target); an audit verdict on each bundle constraint against RBK flows; a workload-identity-federation answer for headless paths; and, for the chosen path, the concrete changes to credential install, regime files, and the RBS* depot / SA-invest specs. Until that decision lands, this heat holds shape only.