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

## Idea — foedus, the configured-federation noun (keystone vocabulary)

Tension: the civic vocabulary names every actor and container in the federation story — the manor that founds, the depot that runs, the citizen who signs in, the mantle they don, the assize they open — but has no noun for the configured trust itself.
A configured federation is referred to mechanically (the workforce pool plus provider plus attribute mapping) or with the overloaded word federation, which carries both the concept and the instance at once.

Why it is the keystone: the healthy payor-governor model is only sayable as a relationship between named nouns, and three ideas below lean on it — multiple federations names instances of it, the governor-selects idea affiliates a depot with one, and the payor federation-setup guide configures one.
With the noun the model collapses to four speakable lines: the payor founds one foedus (affiance, org-scope); the payor sanctions a set of them as eligible (the bound); the governor affiliates a depot with a sanctioned foedus (depot-scope); a depot draws its citizens from the foedus it is affiliated with.

Chosen noun (working lean): foedus — the Latin treaty between a sovereign and an external people (the foederati), and the literal root of the word federation.
It names the standing trust structurally — a pact with an external authority — and it is tightly mnemonic, being the etymon of the very concept it names; it pluralizes (foedera) for the multiple-federations idea.
The noun is a folio, not a colophon: a named foedus is data passed as a parameter to operations that stay homed by actor — the payor founds and dissolves a foedus (affiance and jilt in the payor file), the governor affiliates a depot with one.
So the noun claims no colophon family, which dissolves the earlier worry that a configured-federation tabtarget family would collide with Crucible's rbw-c — the worry that had ruled out the con-/com- binding-together words (concordat, covenant); foedus is preferred over those for its tighter tie to the word federation itself.
Noted tradeoff carried to the mint: foedus is Latin and opaque, so a fresh operator may not parse it cold; the fair-faced cold-probe is the load-bearing gate before it hardens.
This sharpens RBS0 even in the single-federation case, so it is flagged for Fable's ₣BZ heat-review as a candidate RBS0 civic quoin in the rbtf_ federation-civics category; the mint is Fable's, owing the asterism check, terminal exclusivity, and that cold-probe (the grep gate lands clean).
The founding and un-founding verbs (affiance, jilt) are betrothal-register and monogamous while a foedus is a treaty that pluralizes; whether the verbs migrate to match the noun or the registers coexist is the verb-register question for Fable's mint — you do not betroth a foedus.

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

## Idea — freeholds: durable reused test installations

Tension: a federation test-bed is expensive to churn.
A workforce pool soft-deletes for 30 days and counts against the 100-per-org cap the whole time, and a soft-deleted id cannot be re-created, only undeleted — the same churn-quota pain depots already carry (why the team runs skirmish over gauntlet).
Recreating the trust and its citizens every run does not scale.

Current lean: a freehold — a durable, deliberately-kept test installation reused day-to-day, set against the ephemeral create→destroy lifecycle fixture (the freehold/leasehold contrast).
It cross-cuts depot and foedus, and the two are intertwined: a muniment binds a foedus principal to a depot mantle, so the standing-citizen roster is the join between them — the manor-wide roll is the foedus view, the per-polity slice the depot view — and post-impersonation a depot's mantles are donnable only through a foedus.

Split across heats: the foedus-freehold (establish-if-absent + verify, reusing one durable pool via undelete, quota-flat) graduates to a ₣BZ pace, since its machinery — the terrier, brevet, and rehearse — lands there.
The depot-freehold stays shape here: its nature under impersonation stature, its per-polity roster slice, and the canonical→freehold rename of the existing depot test infrastructure (blast radius across the gauntlet/skirmish/dogfight/onboarding suites), all gated on ₣BZ settling the citizen relationship.

Composes with the ideas above: a freehold would be a named instance (the multiple-federations idea) and, for unattended runs, a degenerate federation; reuse-one-pool needs the affiance undelete-on-DELETED fix the workforce-pool-constraints memo records (affiance today treats a soft-deleted pool as present and skips create, leaving a dead trust reported live).

Release-cadence refresh: when the quota-touching lifecycle does run (say at releases), it also refreshes the freehold — jilt then re-establish, ordered after the lifecycle's own create→jilt passes so cleanup is proven on a throwaway before it touches the durable pool.
Buys isolation from stale freeholds.

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

## Idea — heed: rename the compear sign-in verb (fair-faced remint)

Tension: compear is an ashlar — the operator meets it in error output ("assize lapsed, compear") — yet it was minted as an obscure Scots-law term (to appear in answer to a summons).
That inverts word-husbandry: the high-traffic, operator-facing sign-in verb is exactly the slot that wants the plainest, most guessable word, and compear spent its whole budget on rarity, so it never connects to "sign in."
Distinct from the headless-compearance idea above, which is about the act's behavior — TTY gating — not its name.

Current lean: heed — the grep-isolated twin of answer (both name responding to a summons), at zero repo hits where answer lands at 83.
It sits on the fair-faced floor by construction — rare enough to grep clean, warm enough to guess on first contact: the system summons you, you heed it by signing in, and the assize opens.
Runner-up proffer (the present-yourself frame, also grep-clean) reads slightly more as offering a thing than showing up; heed is the better fit.

Carried to the remint: this is a Fable remint with an eviction sweep, not a paddock swap — compear is live (rbtf_compear, the credential accessor, the "assize lapsed, compear" error), so all forms move together (compear -> heed, compearing/compearance -> heeding; heed needs no -ance form, heeding covers every standing usage).
Register note for the mint: heed is plain Old English where its siblings (affiance, compear, brevet) are Latinate — intentional, since husbandry gives the high-traffic error-output ashlar the plainest word, so it should not be re-elevated toward the Latinate family on register grounds.

## Idea — rename the assize live-window noun (fair-faced remint)

Tension: assize names the live sign-in window and is an ashlar — the operator meets it in cap prose and in the "assize lapsed, compear" error text — yet it fails the fair-faced bar on two axes at once.
Register: the leading "ass-" sits at the head of an operator- and customer-visible word, an embedded-substring wart that undercuts the polish corporate adoption needs.
Memorability: assize is an obscure medieval-legal term (a court sitting that is also a fixed measure), so beyond the substring it never connects to "the live sign-in window" on first contact.
The double-miss — bad substring and unguessable — lands in the one slot that least tolerates either, which on its own is the case for retiring the word regardless of its replacement.
Sibling to the heed/compear remint above: the same word-husbandry inversion (a high-traffic operator-facing ashlar minted for rarity), here compounded by the register wart.

Current lean: retire assize; the replacement is Fable's mint with an eviction sweep — the word is live (RBRF_SESSION_DURATION as the cap, the zrba_assize_* identifiers and cache, the "assize opened / assize lapsed" surface), so all forms move together.
Load-bearing fork carried to the mint: whether the concept keeps an operator-facing noun at all, or demotes to hearting with the error text going plain prose ("your sign-in expired — sign in again").
Lean on the fork: the concept is recurring and precise enough to earn a noun (asterism coherence, cap prose like "the window is 15 min–12 h"), so keep a noun but the error text may soften — and either way the new word must clear register and memorability, which assize cannot.
No word is picked here, per the heat's vocabulary-waits-on-Fable posture.

## Idea — a verb to clear the live assize (force a fresh compearance)

Tension: nothing discards a cached assize today — the rba accessor exposes only path/read/live/write, and no tabtarget clears the federation token.
While an assize is live, every rbw-acf cache-hits and reuses the same federated token until natural expiry; the only way to force a fresh compearance now is to delete the cache file by hand.
Demonstrated live (260617): after one compearance, repeated rbw-acf runs reused the single cached token byte-for-byte across the whole assize window — there was no on-demand path to re-mint.
The gap is a capability (delete the per-assize cache file at the tmpfs path) plus a name; the capability is trivial and the name is the open part.

Current lean: add the verb and a tabtarget.
Naming is coupled to the assize-rename idea above — it clears the live-window thing, so it keys off whatever that noun becomes and wants the same vocabulary pass.
Disposition left open: whether it rides the Fable vocabulary mint or lands sooner under a provisional name is undecided — too soon to call any Bf pace Fable-only.

## Idea — one-click compearance via verification_uri_complete (IdP-agnostic)

Tension: compearance makes the operator read a URL and type a short user_code, yet RFC 8628 defines verification_uri_complete — the verification URL with the code pre-embedded, often QR-rendered — for exactly this friction.
The win is modest: it removes typing the code, not the approve-in-browser step the human-present premise requires, and it matters most for keyboardless devices, less on a dev terminal.

Current lean: teach Leg 1 to prefer verification_uri_complete (optionally QR it) whenever the IdP returns it, with graceful fallback to today's URL+code when absent.
IdP-agnostic by construction — it stays dark for an IdP that omits the field and lights up for any conformant one — so it fits the regime's IdP-agnostic design with no provider-specific hack.

Entra finding (260617, empirical, not just docs): the live devicecode response from the standing trust omits verification_uri_complete entirely, and it is not a knob — no scope, param, or app-registration toggle turns it on; Microsoft's endpoint structurally lacks it.
Declined fallback recorded: synthesizing a code-carrying URL ourselves (.../device?otc=<code>) is unsupported, Palisade-fragile, and rides the device-code-phishing pattern Microsoft's filters flag — and tried live, the prefill did not populate, so even the hack does not work.
Revisit trigger: a new or migrated IdP that returns the field, or Microsoft adding it.

## Sources

The office-federation heat ₣BZ is the parent; these ideas are its deliberate deferrals.
Federation mechanism and the identity-provider-side console finding: the federation-legs spike findings memo.
The pace-design and divergence record for the parent heat: its pace-design memo and its movement-4 review-findings memo.
Degenerate-test-federation mechanism, sources, and the can-and-cannot-prove boundary: the degenerate-federation test-personas memo.
Workforce-pool quota and soft-delete constraints (the freehold idea's load-bearing facts): Memos/memo-20260617-BZ-workforce-pool-constraints.md.