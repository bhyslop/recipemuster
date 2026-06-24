## Character

Junk-drawer of small pre-MVP loose ends —
the residual engineering that accreted under the release heats
once the release-qualification machinery shipped.
As the project approaches MVP the remaining issues trend small and self-contained;
this heat is their shared home,
with no pretense of a single goal beyond settling them before release.

Two loose bands, ordered robustness ahead of hygiene:

- Robustness that smooths a clean release run —
  a credential-flap tolerance,
  an OAuth terminal-failure fail-fast,
  and a build-poll queue/execution clock split.
- Hygiene and cleanup —
  a dormant station regime relocated off the MVP surface,
  a shellcheck-coverage widening past the `Tools/` root,
  two stale Cloud Build spec references,
  a network-diagnostic tool repair,
  and a superseded build-bucket scrub.

Paces stand alone;
order is loose priority, not a dependency chain.
The two lowest-value tail paces (the network-diagnostic repair and the stale spec references)
are itch-candidates if the heat needs thinning rather than finishing.

## Coupling to watch

The credential-flap tolerance and the OAuth fail-fast both touch `rbgo_get_token_capture`;
the fail-fast pace's own docket asks whether it is the same landing as the flap-tolerance touchpoint,
so mount those two adjacent or fold them together.

## Chaining-fact discipline — the shape the cluster cinched

One coherent sub-initiative inside the junk-drawer: the verbs that pass a value forward between tabtargets are split by role, and the split is load-bearing for safety.

Chain HEADS — the build and capture verbs (conjure, ensconce, conclave, underpin, immure, kludge, mirror) — only ever WRITE a fact; a head never reads a prior fact.
Conjure violated this by embedding the base-anchor election, which is being extracted into a standalone link (feoff) so conjure becomes pure output and builds only from committed config; that also closes a live provenance hole.

Chain LINKS read one chained value and write one durable config field.
Only three verbs write durable config from a resolved value — feoff (the base anchor), anoint (the graft image), yoke (the reliquary) — and they are the only sanctioned members of the durable-leak surface.

The leak-elimination invariant is no-relay: the express-or-chain resolver is depth-1 and terminally consumed, never forwarded.
The git clean-tree gate is an ergonomic backstop only — never the safety mechanism.
Instance-binding (stamping a producing-vessel identity into the fact) was deliberately weighed and DROPPED; the safety is operator-trust plus loud-on-typecheck output plus the commit-review gate.

Read-side chain consumers (augur, summon, plumb; rekon and vouch are not yet chained) resolve the same express-or-chain fact but write no durable config.
A broken resolve now rejects with the named chaining band (BUBC_band_chain) — reused, NOT a new band: a read and a durable-leak write never share one tabtarget, so 105 stays unambiguous per tabtarget without a second code.
This reverses the earlier loud-die cinch: the read/write distinction no longer lives in the exit code but in the EFFECT — a wrong read corrupts only a transient action, never a regime file.
They must still never be extended to write durable config without joining the durable-leak surface and its no-relay discipline.

The read side carries two requirements the durable-leak verbs' coverage does not vouch for it, and they need two different nets — one runtime, one static — because the exit code cannot tell them apart.
First, furnish: any CLI whose source closure reaches a buf_* fact caller must source buf_fact, or the helper is undefined and the verb dies command-not-found at the resolve.
This is furnish-what-you-LOAD, not just furnish-what-you-dispatch: the static check caught rbfh and rbfv, which only transitively load rbfcp_plumb through the rbfc0_core 0-trick.
Second, coverage: the reveille net is named-band exit-code cases (folio-less drives asserting the band — the resolve-logic net) PLUS a static furnish-invariant case (the transitive source-closure scan).
The furnish gap needs the static case because a command-not-found exits the SAME band 105 as a real broken chain, so a runtime exit-code case cannot distinguish it; an end-to-end cloud fixture (dogfight) proves the pull against real artifacts.
A coding error like the furnish gap is flushed out statically and NEVER given a named runtime band — naming command-not-found (127) would mix tiers, dressing a defect as a deliberate rejection.

Verification of this discipline is homed under theurge — it drives the real verbs through the full tabtarget exec path and asserts the named chain-rejection band, regime-poison the type specimen.
The band now fires at both the durable-leak LINKS (feoff/yoke via buc_reject) and the read consumers (summon/plumb/augur), all RBK; the BUK footing resolver and decoder still return a bare 1.
It is deliberately NOT homed in the BUK bash self-test: the band fires only at the RBK consumer, and the self-test sources no RBK — so it can prove the footing primitives' return-1 shape but never the band itself.
The footing primitives keep their existing BUK self-test cases; the band, wrong-kind, furnish-invariant, and fact-intact matrix is theurge's.

## Provenance

Restrung from ₣BB and ₣BU.
₣BB built and shipped the release-qualification machinery — its goal is complete;
these paces were ordinary engineering that had collected under it, never release ceremony.
₣BU holds the actual release runs and the RELEASE.md revision,
parked stabled as paddock-memory until release time —
that work is not here and is not reopened by this heat.
Both source heats keep the git history of any dropped dockets.

## References

- ₣BB — shipped the release-qualification machinery; holds the design record of what was built
- ₣BU — the deferred release runs and doc revision (stabled until release time)