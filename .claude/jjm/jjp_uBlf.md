## Charter — a holding paddock for federation-evolution ideas

This heat is a holding paddock for federation ideas deliberately deferred from ₣BZ.
Every entry below is a candidate for discussion, not a committed change.

Posture (cinched for this heat): no pace slated here is a decision.
A docket is a specification of needed change; an undecided idea is shape, so it lives in this paddock, not in a pace.
A pace is cut only when an idea graduates from discuss to do — at which point it earns a real docket.
Until then the heat sits stabled, often with zero paces.

Premise-touching ideas wait on Fable: any idea that would amend a cinched ₣BZ premise graduates only after Fable's ₣BZ heat-review has weighed it.
Keep this catalog shape-shaped — each idea a named tension plus a current lean, never a running discussion log.
Genuinely thin or orthogonal ideas belong in itches or the horizon roadmap, not here; this paddock is for ideas worth deliberating with heat context.

## Idea — payor federation-setup guide

Tension: the identity-provider-side preconditions — standing up the external OIDC tenant and app registration, then authoring the federation regime from its values — are foreign-console human work with no operator guide today.
The affiance ceremony proves only the Google-side, autonomous half of founding; the human precondition half has no home.

Current lean: a guide-family procedure (colophon rbw-gPF, slotting into the payor-guide rbw-gP* family), capturing the federation spike's identity-provider-side manual steps.
A guide, not a workbench command — the work is in a foreign console and cannot be driven by an API token, exactly as manor establishment (rbw-gPE) is a guide for the same reason.

Source: the federation spike found the only console work was identity-provider-side; everything Google-side was payor-token REST.

## Idea — multiple federations (a federation-regime family)

Tension: today one federation regime serves the whole manor — one workforce pool, one subject namespace.
A single pool makes every provider a full root of trust over the whole namespace, so the pool's security floor is its weakest provider, and there is no way to isolate two real trust domains, or to isolate a test trust from a production trust.

Current lean: let the federation regime become a family of named instances (the pattern the nameplate and per-identity auth regimes already use), with affiance keyed to a named instance and each instance its own pool.
Buys trust isolation between real identity providers, per-depot-group trust, and — the load-bearing one — test/production separation.

Enables the degenerate-test-federation idea and the governor-selects-federation idea below; both presuppose it.

## Idea — degenerate test federation

Tension: the federation heat runs its suites by compearing once at suite head — one human browser click per run.
That is fine for operator-driven runs (the federation heat does not need unattended CI), but those suites cannot run unattended.

Current lean (mechanism doc-confirmed): a caged self-signed JWT — hold a signing keypair, upload its public JWKS to a workforce provider, mint tokens locally, and POST straight to the STS token endpoint (the headless programmatic flow, no browser).
Removes the human from the autonomous suite surface.

Hard constraint: the caged signing key is a new durable secret and a root of trust over its pool.
It must therefore live in its own pool (depends on the federation-regime-family idea) and never share a pool with real citizens, and it must never stand in for the paces whose whole purpose is to prove the real human-click path.

Why deferred: it introduces a durable secret, against the federation heat's zero-keys premise; and that heat does not need unattended CI, so the cost buys nothing there.

## Idea — the governor's role in federation

Frame under consideration: the payor as the IT department — founds federations and bounds what is permissible — and governors as more-trusted regional stewards who operate within those bounds, still subordinate to the payor's citizen-list and federation-set choices.

Two sub-questions, opposite verdicts on one scope test (creation is org-scoped; selection is depot-scoped):

- Configure or found a federation — declined.
  Founding trust is org-wide: a new provider is a root of trust over every depot under the manor.
  That belongs to the org-owner (payor), not a depot-scoped steward.
  This one is settled: federation founding is a payor job.

- Select which federation a depot is affiliated with — open.
  Selection among already-founded trusts affects only the one depot, so it respects the cinch that cross-depot administration does not exist, and it mirrors admission (the governor already decides who is admitted; this adds which trust the depot draws from).
  Guard: the eligible set must be payor-bounded, so a governor cannot repoint a production depot at a weak — or degenerate — pool.
  Depends on the federation-regime-family idea.

Open mechanism sub-question: how is the hierarchy enforced rather than merely documented?
The lean is that this is IAM-native, not bespoke cryptography — founding pools is an org-level permission the payor holds and governors structurally lack, and bounding a governor's selection is an IAM-condition or provider attribute-condition.
The public-key-anchor-with-private-key-at-the-provider pattern is worth naming because it already is the model: the workforce provider holds the identity provider's public keys (JWKS) while the identity provider holds the private signing key.
A bespoke payor keypair would re-introduce a durable private secret — against the zero-keys premise — and duplicate what IAM already enforces; flagged as the path to avoid.
The exact GCP condition mechanism wants verification before this graduates.

## Idea — headless compearance (restrung in from the federation heat)

Restrung in: reconsider whether compearance must gate on a controlling terminal, or whether a headless-but-human-reachable caller can open an assize by surfacing the device-flow prompt out-of-band and polling to completion.

It keeps the human-present premise — a human still authenticates each assize — and only relaxes human-present from terminal-present.
Distinct from the degenerate-test-federation idea, which removes the human entirely.

Touches a cinched federation-heat premise (human-present, and the headless fail-fast membrane), so it graduates only after Fable's federation-heat review, and may surface a paddock amendment rather than land purely in code.

## Sources

The office-federation heat ₣BZ is the parent; these ideas are its deliberate deferrals.
Federation mechanism and the identity-provider-side console finding: the federation-legs spike findings memo.
The pace-design and divergence record for the parent heat: its pace-design memo and its movement-4 review-findings memo.