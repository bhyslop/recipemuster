## Context

Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. This heat addresses the full vocabulary gap: service lifecycle verbs, diagnostic verbs, service type naming, the MCM identity tier vocabulary, renaming "bottle service" to crucible, and renaming "censer" to pentacle.

## Discovery: Architecture is Sound

The agile/sessile distinction is load-bearing and already implemented correctly:
- `bottle_start` stands up sentry+pentacle+bottle (all persistent) — sessile pattern
- `bottle_run` dispatches an ephemeral bottle into an already-running sentry+pentacle — agile pattern
- The nameplate should declare service mode (agile/sessile) as single source of truth
- Verbs become polymorphic on mode: same verb, different behavior per mode

## Decided: MCM Identity Tier Rename

Replacing the three-tier vocabulary in MCM. All from mason/engraver/foundry register:

lemma/lemmata to **quoin**/quoins — Cornerstone that locks the structure. Full catalogue: mapping + anchor + definition.
graven to **inlay**/inlays — Set permanently into a surface. Prefix-named, subdocument-scoped, no catalogue entry.
intaglio to **sprue**/sprues — The literal channel from the mould. Wire-level token in backticks.

File inventory (4 files, ~31 occurrences): MCM-MetaConceptModel.adoc (18), AXLA-Lexicon.adoc (8), RBS0-SpecTop.adoc (1), jjp_uAlh.md (4).

Confidence: HIGH. "Coining vocabulary by defining quoins" — the spelling prevents collision with "coin."

## Decided: Lifecycle Verbs

**broach** (6, b for begin) — Open service for use, polymorphic on mode.
**quench** (6, q for quit) — Extinguish service, polymorphic on mode.
**decant** (6, d for dispatch) — Pour ephemeral work through agile envelope.

Verb behavior matrix:
- broach sessile: stand up sentry + pentacle + bottle
- broach agile: stand up sentry + pentacle only
- quench sessile: tear down bottle + pentacle + sentry
- quench agile: tear down pentacle + sentry
- decant sessile: FAIL (wrong mode)
- decant agile not broached: FAIL (not broached)
- decant agile broached: dispatch ephemeral bottle

Operator mental model:
- Sessile: broach ... service runs ... quench
- Agile: broach ... decant decant decant ... quench

Confidence: HIGH.

Note: Forge-native alternatives (kindle/quench/cast) are thematically coherent with crucible. Broach/quench/decant remain elected but forge verbs are a recognized option if thematic unity is prioritized later.

## Decided: Service Mode Names

**agile** and **sessile** retained. Load-bearing distinction, unique words. Will become nameplate enum (RBRN_SERVICE_MODE).

Confidence: HIGH.

## Decided: Diagnostic Verbs

**arraign** (7) — bottle (the demon), commanding/adversarial tone.
**consult** (7) — sentry (the ally), respectful/mutual tone.
**surveil** (7) — networks (the perimeter), observational.

Pentacle interactive access eliminated — scaffolding with no diagnostic use case. Diagnostic verbs are parameter-driven (moniker as argument), not imprint-driven.

Confidence: HIGH on arraign/consult. MEDIUM on surveil.

## Decided: "Bottle Service" → Crucible

The tandem container assembly (sentry + pentacle + bottle) is now named **crucible**. The vessel where dangerous materials are subjected to extreme conditions and transformed. Universally understood, precisely correct for the security-containment metaphor.

Forge vocabulary (kindle/quench/cast) is natively coherent with crucible — quench is native to both the forge register and the current lifecycle verbs. The existing verbs (broach/quench/decant) work well alongside crucible.

Colophon root: rbw-c (crucible). Uppercase terminal for lifecycle bookends, lowercase for operational/diagnostic.

Confidence: HIGH.

## Decided: Censer → Pentacle

The privileged container establishing network namespace and routing is now named **pentacle**. From the Solomonic tradition: the inscribed disc establishing the magician's authority over the contained space, compelling the demon to obey the rules. Maps precisely to function — pentacle defines the namespace rules everything else operates within.

The three-container trio:
- **Sentry** — guards the perimeter (eBPF, iptables, dnsmasq)
- **Pentacle** — establishes authority over the space (network namespace, routing)
- **Bottle** — holds the demon (application container)

Maps directly to Solomonic practice: outer protection, inscribed authority, sealed vessel.

Confidence: HIGH.

## Open: Quoin Prefix Allocation

New quoins: rbtlob_broach, rbtloq_quench, rbtlod_decant, rbrn_service_mode (+_sessile, +_agile).
Retire: opbs_bottle_start, opbr_bottle_run, opss_sentry_start.
Rename: at_bottle_service → at_crucible, at_censer_container → at_pentacle_container.
New: at_crucible — the whole assembly.

## Open: Colophon Root and Tabtarget Migration

Root: rbw-c (crucible). Uppercase terminal for lifecycle bookends (Broach, Quench), lowercase for operational/diagnostic. Migration plan covers 7 existing tabtarget families plus new Decant.

## Open: Forge Verb Alignment

With crucible elected, the forge register offers thematically native alternatives to current lifecycle verbs:
- kindle (start) vs broach (open)
- quench (stop) — same in both registers
- cast (dispatch) vs decant (pour)

Decision deferred — broach/quench/decant are already decided at HIGH confidence. Revisit if thematic unity with crucible becomes a priority.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh
- Prior conversations: 260327 session, 260329 crucible/pentacle election