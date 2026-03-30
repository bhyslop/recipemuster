## Context

Recipe Bottle's operations use generic verbs where liturgical ones belong. This heat addresses the full vocabulary gap across four domains: crucible lifecycle verbs (charge/quench/enjoin), crucible diagnostic verbs (rack/hail/observe), consecration operation verbs (tally/ordain), role/regime authority verbs (knight/forfeit/levy/unmake/mantle/zero/proof), image-level artifact verbs (wrest/jettison/plumb), plus concept renames (censer→pentacle, bottle service→crucible) and MCM identity tier vocabulary.

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

## Decided: Consecration Operation Verbs

Renaming the two remaining off-metaphor generic verbs in the Director colophon family (`rbw-D*`). Verb initials must not collide with any existing cloud verb: A(abjure), B(bind), C(conjure), D(delete), E(enshrine), G(graft), I(inscribe/inspect), S(summon), V(vouch).

**tally** (5, T) — Count, verify, and classify consecrations by health state. Accounting/forge register: tallying the yield. Read-only registry audit via GAR Docker Registry API.
**ordain** (6, O) — The Director ordains the consecration; the forge executes conjure/bind/graft. Solomonic/liturgical: the presiding authority initiating the ceremony, delegating specific rites. Master ceremony dispatch.

| Old | New | Colophon | Function | Frontispiece | Quoin |
|-----|-----|----------|----------|-------------|-------|
| checks | tally | `rbw-Dc` → `rbw-Dt` | `rbf_check_consecrations` → `rbf_tally` | `DirectorChecksConsecrations` → `DirectorTalliesConsecrations` | `rbtgo_consecration_check` → `rbtgo_consecration_tally` |
| creates | ordain | `rbw-DC` → `rbw-DO` | `rbf_create` → `rbf_ordain` | `DirectorCreatesConsecration` → `DirectorOrdainsConsecration` | (none) → `rbtgo_ark_ordain` |

Zipper: `RBZ_CHECK_CONSECRATIONS` → `RBZ_TALLY_CONSECRATIONS`, `RBZ_CREATE_CONSECRATION` → `RBZ_ORDAIN_CONSECRATION`.

Note: `rbtgo_ark_ordain` and other `rbtgo_ark_*` quoins follow existing naming convention. ₣Az (ark concept removal) will revise these names when it executes — no pre-optimization here.

Confidence: HIGH.

## Decided: Role Authority & Regime Lifecycle Verbs (Group A)

Renaming the generic verbs used by role actors (Governor, Payor, Marshal) for authority conferral, infrastructure lifecycle, and regime operations. Register: feudal/military commission — the institutional governance layer above the forge/Solomonic operations.

The role nouns (Payor, Governor, Marshal, Director, Retriever, Mason) form a chartered institution: the Payor funds, the Governor authorizes, the Marshal maintains order, the Director commands operations, the Retriever serves, the Mason builds. The verbs come from that same institutional register.

| Operation | Old verb | New verb | Letter | Actor | Register | Rationale |
|-----------|----------|----------|--------|-------|----------|-----------|
| Create role SA + IAM | create | **knight** | K | Governor | feudal | Confer knighthood — the Governor knights the Director/Retriever into service. |
| Delete role SA | delete | **forfeit** | F | Governor | feudal/legal | Authority seized back by decree. The office is forfeit. Pair with knight. |
| Create depot (GAR + bucket + pool + mason) | create | **levy** | L | Payor | feudal/military | Raise by sovereign authority. The patron raises infrastructure by financial command. |
| Destroy depot | destroy | **unmake** | U | Payor | archaic | Reverse of creation. More fundamental than "destroy." Pair with levy. |
| Reset governor (destroy old SA, create fresh) | reset | **mantle** | M | Payor | feudal | Invest with the mantle of authority. Old mantle cast off, new one bestowed. Captures destroy+recreate. |
| Reset regime to blank template | reset | **zero** | Z | Marshal | military | Zero the instrument before calibration. |
| Duplicate repo for release testing | duplicate | **proof** | P | Marshal | publishing | A proof copy before the print run. |

Natural pairs: knight/forfeit (confer/seize), levy/unmake (raise/reverse), zero/proof (prepare/test).

Confidence: HIGH.

## Decided: Image/Artifact-level Verbs (Group B)

Renaming the generic verbs for image-level operations. These parallel the consecration-level verbs but operate on individual artifacts rather than coherent consecration packages. Register: forge/Solomonic, with verbs conveying surgical single-artifact nature.

Key insight: "retrieve" was contaminated by the Retriever role name — like naming the Director's primary operation "direct." The verb should not echo the role.

| Operation | Old verb | New verb | Letter | Actor | Register | Rationale |
|-----------|----------|----------|--------|-------|----------|-----------|
| Pull specific image by ref | retrieve | **wrest** | W | Retriever | feudal/physical | Seize by force, pull away. Distinct from summon (consecration package). |
| Delete specific image tag | delete | **jettison** | J | Director | naval | Throw overboard to save the ship. Surgical discard. Distinct from abjure (full consecration). |
| Examine trust posture | inspect | **plumb** | P | Retriever | mason/forge | Probe the depths with a plumb-bob. Forensic examination of trust. |

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

## Full Cloud Verb Registry (post this heat)

**Consecration-level** (Solomonic/forge): abjure(A), enshrine(E), ordain(O), summon(S), tally(T), vouch(V)
**Image-level** (Solomonic/forge): jettison(J), plumb(P), wrest(W)
**Crucible** (forge): charge(C), enjoin(E), hail(H), observe(O), quench(Q), rack(R)
**Role/regime** (feudal/military): forfeit(F), knight(K), levy(L), mantle(M), proof(P), unmake(U), zero(Z)

## Forward Dependency: ₣Az (ark concept removal)

This heat follows existing `rbtgo_ark_*` naming convention for quoins (e.g., `rbtgo_ark_ordain`, `rbtgo_ark_plumb`). ₣Az will later remove the "ark" concept and revise these names. No pre-optimization — follow convention now, let ₣Az handle the sweep.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh, rbf_Foundry.sh, rbz_zipper.sh (full colophon registry)
- rbgg_cli.sh (Governor), rbgp_cli.sh (Payor), rblm_cli.sh (Marshal)
- Prior conversations: 260327 session, 260329 vocabulary election session, 260330 tally/ordain + Group A/B election sessions