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

2. Identity / keyless path — re-express retriever and director as Google identities (OAuth user / workload identity federation / impersonation) rather than SA-key-bearing service accounts: short-lived credentials, no downloadable keys, natively secure-by-default compatible. A larger change touching the RBRA credential model end to end. An existing exploration in the project's JJ records already gestures at binding retriever and director directly to Google email identities; that idea is the seed of this path.

## What done looks like

A decision on override vs identity-keyless (plausibly override-to-unblock now, keyless as the durable target); an audit verdict on each bundle constraint against RBK flows; and, for the chosen path, the concrete changes to credential install, regime files, and the RBS* depot / SA-invest specs. Until that decision lands, this heat holds shape only.