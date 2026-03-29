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

**rack** (4, k) — bottle (the demon). The instrument of compulsion — compel the demon to reveal its state. Adversarial, commanding.
**hail** (4, h) — sentry (the ally). Call out to the guard. Respectful, military.
**surveil** (7, o) — networks (the perimeter). Observational. Reuses existing ObserveNetworks colophon root.

Pentacle interactive access eliminated — scaffolding with no diagnostic use case.

Confidence: HIGH on rack/hail. MEDIUM on surveil.

## Decided: "Bottle Service" → Crucible

The tandem container assembly (sentry + pentacle + bottle) is now named **crucible**. The vessel where dangerous materials are subjected to extreme conditions and transformed. Universally understood, precisely correct for the security-containment metaphor.

Confidence: HIGH.

## Decided: Censer → Pentacle

The privileged container establishing network namespace and routing is now named **pentacle**. From the Solomonic tradition: the inscribed disc establishing the magician's authority over the contained space, compelling the demon to obey the rules. Maps precisely to function — pentacle defines the namespace rules everything else operates within.

The three-container trio:
- **Sentry** — guards the perimeter (eBPF, iptables, dnsmasq)
- **Pentacle** — establishes authority over the space (network namespace, routing)
- **Bottle** — holds the demon (application container)

Maps directly to Solomonic practice: outer protection, inscribed authority, sealed vessel.

Confidence: HIGH.

## Decided: Colophon Migration

Standalone verb-roots, consistent with existing `s`/`z`/`o` pattern. All six roots are collision-free.

| New | Verb | Replaces | Moniker? |
|-----|------|----------|----------|
| `rbw-c` | Charge | `rbw-s` (Start) | yes |
| `rbw-q` | Quench | `rbw-z` (Stop) | yes |
| `rbw-e` | Enjoin | (new) | yes |
| `rbw-h` | Hail sentry | `rbw-S` (ConnectSentry) | yes |
| `rbw-k` | Rack bottle | `rbw-B` (ConnectBottle) | yes |
| `rbw-o` | Surveil networks | `rbw-o` (ObserveNetworks) | yes |

Retiring: `rbw-s` (Start), `rbw-z` (Stop), `rbw-B` (ConnectBottle), `rbw-C` (ConnectCenser), `rbw-S` (ConnectSentry).

Tabtarget examples:
- `tt/rbw-c.Charge.tadmor.sh` (was `tt/rbw-s.Start.tadmor.sh`)
- `tt/rbw-q.Quench.tadmor.sh` (was `tt/rbw-z.Stop.tadmor.sh`)
- `tt/rbw-e.Enjoin.tadmor.sh` (new — agile only)
- `tt/rbw-h.Hail.tadmor.sh` (was `tt/rbw-S.ConnectSentry.tadmor.sh`)
- `tt/rbw-k.Rack.tadmor.sh` (was `tt/rbw-B.ConnectBottle.tadmor.sh`)
- `tt/rbw-o.Surveil.tadmor.sh` (was `tt/rbw-o.ObserveNetworks.tadmor.sh`)

Confidence: HIGH.

## Open: Quoin Prefix Allocation

New quoins: rbtloc_charge, rbtloq_quench, rbtloe_enjoin, rbrn_service_mode (+_sessile, +_agile).
Retire: opbs_bottle_start, opbr_bottle_run, opss_sentry_start.
Rename: at_bottle_service → at_crucible, at_censer_container → at_pentacle_container.
New: at_crucible — the whole assembly.

## Open: Surveil Confidence

Surveil remains at MEDIUM. Alternatives considered but not resolved. The colophon reuse of `rbw-o` is clean regardless of verb name.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh
- Prior conversations: 260327 session, 260329 crucible/pentacle/charge/quench/enjoin/hail/rack election