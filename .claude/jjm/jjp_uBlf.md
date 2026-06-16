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

## Idea — entente, the configured-federation noun (keystone vocabulary)

Tension: the civic vocabulary names every actor and container in the federation story — the manor that founds, the depot that runs, the citizen who signs in, the mantle they don, the assize they open — but has no noun for the configured trust itself.
A configured federation is referred to mechanically (the workforce pool plus provider plus attribute mapping) or with the overloaded word federation, which carries both the concept and the instance at once.

Why it is the keystone: the healthy payor-governor model is only sayable as a relationship between named nouns, and three ideas below lean on it — multiple federations names instances of it, the governor-selects idea affiliates a depot with one, and the payor federation-setup guide configures one.
With the noun the model collapses to four speakable lines: the payor founds one entente (affiance, org-scope); the payor sanctions a set of them as eligible (the bound); the governor affiliates a depot with a sanctioned entente (depot-scope); a depot draws its citizens from the entente it is affiliated with.

Chosen noun (working lean): entente — the standing trust between two powers, in the diplomatic register the manorial/legal asterism already spans.
It pluralizes cleanly, which the multiple-federations idea needs: a manor holds several ententes as a sovereign holds several, where the marriage-register nouns (troth, espousal) stay monogamous and break.
The winnowing constraint was the colophon namespace: the structural ideal was a con-/com- binding-together word (concordat, covenant), but that whole Latin family starts with c, and Crucible owns the rbw-c colophon family — so the configured-federation tabtargets would collide.
Entente is grep-clean and takes a free rbw-e colophon family.
Noted tradeoff carried to the mint: entente connotes a loose understanding, which undersells the hard cryptographic trust beneath; weighed and accepted.
This sharpens RBS0 even in the single-federation case, so it is flagged for Fable's ₣BZ heat-review as a candidate RBS0 civic quoin; the mint is Fable's, owing the asterism check and terminal exclusivity (the grep gate lands clean), plus a fair-faced cold-probe before it hardens, and a check of whether affiance stays the founding verb or the entente earns a founding verb in its own register (you do not betroth an entente).

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

Detail and sources: the degenerate-federation test-personas memo records the live-doc confirmation of the programmatic STS flow (RFC 8693 token exchange; uploaded JWKS usable in the programmatic flow only), the two degenerate shapes (caged self-signed JWT vs a real test IdP on a non-interactive grant), the can-and-cannot-prove boundary, and the GCP / Keycloak / RFC URLs.
Honesty caveat carried in the memo: the mechanism is doc-confirmed and spike-paper-confirmed, not yet live-run in our own harness.

## Idea — the governor's role in federation

Frame under consideration: the payor as the IT department — founds federations and bounds what is permissible — and governors as more-trusted regional stewards who operate within those bounds, still subordinate to the payor's citizen-list and federation-set choices.

Two sub-questions, opposite verdicts on one scope test (creation is org-scoped; selection is depot-scoped):

- Configure or found a federation — declined.
  Founding trust is org-wide: a new provider is a root of trust over every depot under the manor.
  That belongs to the org-owner (payor), not a depot-scoped steward.
  This one is settled: federation founding is a payor job.

- Select which federation a depot is affiliated with — open, and this is the healthy model to aim for.
  Stated in the keystone noun above: the payor founds the configured federations and sanctions which are eligible; the governor affiliates its own depot with one of the sanctioned ones.
  Selection among already-founded, payor-sanctioned trusts affects only the one depot, so it respects the cinch that cross-depot administration does not exist, and it mirrors admission — the governor already decides who is admitted; this adds which trust the depot draws from.
  Depends on the multiple-federations idea.

The bound — the sanctioned set: which configured federations are eligible for affiliation is a payor-controlled, manor-level artifact, an eligibility the payor sets and governors only read.
This is the IT-department catalog of approved trusts.
Guard: a degenerate or test federation is never eligible for a production depot, so a governor cannot repoint production at a weak or caged pool.

The load-bearing distinction for how the hierarchy is enforced is authentication versus authorization:
- Authentication is the crypto layer, and it already exists — the workforce provider holds the identity provider's public keys (JWKS) while the identity provider holds the private signing key.
  A token is trusted iff signed by that private key; this answers only whether the token is genuinely from the trusted identity provider.
- Authorization is where the payor-governor hierarchy lives, and it is IAM, not the signing key — may this governor admit this principal, and may this governor affiliate this depot with this federation, are IAM questions.
  Founding a pool is an org-level permission the payor holds and a governor structurally lacks; bounding a governor's affiliation to the sanctioned set is an IAM-condition or a provider attribute-condition.

A bespoke payor keypair issuing signed grants would re-introduce a durable private secret — against the zero-keys premise — and duplicate what IAM already enforces; flagged as the path to avoid.
The exact GCP condition mechanism that bounds affiliation to the sanctioned set wants verification before this graduates.

## Idea — headless compearance (restrung in from the federation heat)

Restrung in: reconsider whether compearance must gate on a controlling terminal, or whether a headless-but-human-reachable caller can open an assize by surfacing the device-flow prompt out-of-band and polling to completion.

It keeps the human-present premise — a human still authenticates each assize — and only relaxes human-present from terminal-present.
Distinct from the degenerate-test-federation idea, which removes the human entirely.

Touches a cinched federation-heat premise (human-present, and the headless fail-fast membrane), so it graduates only after Fable's federation-heat review, and may surface a paddock amendment rather than land purely in code.

## Sources

The office-federation heat ₣BZ is the parent; these ideas are its deliberate deferrals.
Federation mechanism and the identity-provider-side console finding: the federation-legs spike findings memo.
The pace-design and divergence record for the parent heat: its pace-design memo and its movement-4 review-findings memo.
Degenerate-test-federation mechanism, sources, and the can-and-cannot-prove boundary: the degenerate-federation test-personas memo.