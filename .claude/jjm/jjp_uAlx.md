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

**charge** (6, c) — Load the crucible and bring to operating temperature. Forge-native.
**quench** (6, q) — Extinguish the crucible. Forge-native, the original elected stop verb.
**enjoin** (6, e) — Formally command ephemeral work through agile envelope. Completion connotation.

Verb behavior matrix:
- charge sessile: stand up sentry + pentacle + bottle
- charge agile: stand up sentry + pentacle only
- quench sessile: tear down bottle + pentacle + sentry
- quench agile: tear down pentacle + sentry
- enjoin sessile: FAIL (wrong mode)
- enjoin agile not charged: FAIL (not charged)
- enjoin agile charged: dispatch ephemeral bottle

Operator mental model:
- Sessile: charge ... service runs ... quench
- Agile: charge ... enjoin enjoin enjoin ... quench

Confidence: HIGH.

## Decided: Service Mode Names

**agile** and **sessile** retained. Load-bearing distinction, unique words. Will become nameplate enum (RBRN_SERVICE_MODE).

Confidence: HIGH.

## Decided: Diagnostic Verbs

**rack** (4, r) — bottle (the demon). The instrument of compulsion — compel the demon to reveal its state. Adversarial, commanding.
**hail** (4, h) — sentry (the ally). Call out to the guard. Respectful, military.
**observe** (7, o) — networks (the perimeter). Observational. Retained from current vocabulary.

Pentacle interactive access eliminated — scaffolding with no diagnostic use case.

Confidence: HIGH.

## Decided: "Bottle Service" → Crucible

The tandem container assembly (sentry + pentacle + bottle) is now named **crucible**. The vessel where dangerous materials are subjected to extreme conditions and transformed. Universally understood, precisely correct for the security-containment metaphor.

Confidence: HIGH.

## Decided: Censer → Pentacle

The privileged container establishing network namespace and routing is now named **pentacle**. From the Solomonic tradition: the inscribed disc establishing the magician's authority over the contained space, compelling the demon to obey the rules.

The three-container trio:
- **Sentry** — guards the perimeter (eBPF, iptables, dnsmasq)
- **Pentacle** — establishes authority over the space (network namespace, routing)
- **Bottle** — holds the demon (application container)

Confidence: HIGH.

## Decided: Crucible Colophon Family

All crucible operations grouped under `rbw-c*`. Uppercase second letter = lifecycle, lowercase = operational/diagnostic.

| Colophon | Operation | Type |
|----------|-----------|------|
| `rbw-cC` | Charge | lifecycle |
| `rbw-cQ` | Quench | lifecycle |
| `rbw-ce` | Enjoin | operational |
| `rbw-ch` | Hail sentry | diagnostic |
| `rbw-cr` | Rack bottle | diagnostic |
| `rbw-co` | Observe networks | diagnostic |

Retiring: `rbw-s` (Start), `rbw-z` (Stop), `rbw-B` (ConnectBottle), `rbw-C` (ConnectCenser), `rbw-S` (ConnectSentry), `rbw-o` (ObserveNetworks).

Tabtarget examples:
- `tt/rbw-cC.Charge.tadmor.sh`
- `tt/rbw-cQ.Quench.tadmor.sh`
- `tt/rbw-ce.Enjoin.tadmor.sh`
- `tt/rbw-ch.Hail.tadmor.sh`
- `tt/rbw-cr.Rack.tadmor.sh`
- `tt/rbw-co.Observe.tadmor.sh`

Confidence: HIGH.

## Decided: Quoin Prefix — rbsc_

All crucible-related quoins use flat `rbsc_` prefix (RB Spec, Crucible category). Follows existing `rbs*` convention (`rbst_` types, `rbsi_` ifrit, `rbsk_` constraints). Term count (~10) doesn't warrant subcategory descent.

New quoins:
- `rbsc_crucible` — the tandem container assembly
- `rbsc_pentacle` — namespace/routing container (replaces `at_censer_container`)
- `rbsc_charge` — lifecycle: stand up crucible
- `rbsc_quench` — lifecycle: tear down crucible
- `rbsc_enjoin` — operational: dispatch ephemeral bottle (agile)
- `rbsc_hail` — diagnostic: sentry
- `rbsc_rack` — diagnostic: bottle
- `rbsc_observe` — diagnostic: networks
- `rbsc_agile` — service mode: ephemeral dispatch pattern
- `rbsc_sessile` — service mode: persistent service pattern

Retiring: `opbs_bottle_start`, `opbr_bottle_run`, `opss_sentry_start`.
Migrating display text: `at_bottle_service` → `rbsc_crucible`, `at_censer_container` → `rbsc_pentacle`, `at_agile_service` → `rbsc_agile`, `at_sessile_service` → `rbsc_sessile`.

Note: Full `at_*` retirement is a separate future heat. This heat only mints new `rbsc_` terms and retires the `op*_` terms they directly replace.

Confidence: HIGH.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh
- Prior conversations: 260327 session, 260329 vocabulary election session