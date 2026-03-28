## Context

Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. This heat addresses the full vocabulary gap: service lifecycle verbs, diagnostic verbs, service type naming, the MCM identity tier vocabulary, and potentially renaming "bottle service" itself.

## Discovery: Architecture is Sound

The agile/sessile distinction is load-bearing and already implemented correctly:
- `bottle_start` stands up sentry+censer+bottle (all persistent) — sessile pattern
- `bottle_run` dispatches an ephemeral bottle into an already-running sentry+censer — agile pattern
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
- broach sessile: stand up sentry + censer + bottle
- broach agile: stand up sentry + censer only
- quench sessile: tear down bottle + censer + sentry
- quench agile: tear down censer + sentry
- decant sessile: FAIL (wrong mode)
- decant agile not broached: FAIL (not broached)
- decant agile broached: dispatch ephemeral bottle

Operator mental model:
- Sessile: broach ... service runs ... quench
- Agile: broach ... decant decant decant ... quench

Confidence: HIGH.

## Decided: Service Mode Names

**agile** and **sessile** retained. Load-bearing distinction, unique words. Will become nameplate enum (RBRN_SERVICE_MODE).

Confidence: HIGH.

## Decided: Diagnostic Verbs

**arraign** (7) — bottle (the demon), commanding/adversarial tone.
**consult** (7) — sentry (the ally), respectful/mutual tone.
**surveil** (7) — networks (the perimeter), observational.

Censer interactive access eliminated — scaffolding with no diagnostic use case. Diagnostic verbs are parameter-driven (moniker as argument), not imprint-driven.

Confidence: HIGH on arraign/consult. MEDIUM on surveil.

## Open: "Bottle Service" Rename

The tandem container assembly (sentry + censer + bottle) is currently called "bottle service." Problems: nightclub joke, names whole after one part, "bs" colophons.

Finalists:

**vault** (5) — Universally understood, zero learning curve. Verb compatibility strong ("broach the vault"). Colophon rbw-v all clean. Concerns: common word (less distinctive), V shared with Vessel.

**citadel** (7) — Fortified area WITHIN a larger system, architecturally precise. More character, matches elevated register. Concerns: "quench the citadel" slightly unnatural, C shared with Censer, cc colophon with Consult.

Eliminated: dungeon (D/Depot collision), alembic (obscure), crucible (cc), keep (small/static), bastion/fortress/bunker (vault/citadel stronger).

Decision needed: vault wins usability, citadel wins character.

## Open: Quoin Prefix Allocation

New quoins (modern convention): rbtlob_broach, rbtloq_quench, rbtlod_decant, rbrn_service_mode (+_sessile, +_agile).
Retire: opbs_bottle_start, opbr_bottle_run, opss_sentry_start.
Update text only: at_sessile_service, at_agile_service, at_bottle_service.
Depends on bottle-service rename decision.

## Open: Colophon Root and Tabtarget Migration

Root depends on rename: rbw-v (vault) or rbw-c (citadel). Uppercase terminal for lifecycle bookends (Broach, Quench), lowercase for operational/diagnostic. Migration plan covers 7 existing tabtarget families plus new Decant.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh
- Prior conversation: 260327 session