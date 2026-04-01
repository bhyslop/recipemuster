# Heat Trophy: rbk-mvp-5-verb-colorization

**Firemark:** ₣Ax
**Created:** 260327
**Retired:** 260331
**Status:** retired

## Paddock

## Context

Recipe Bottle's operations use generic verbs where liturgical ones belong. This heat addresses the full vocabulary gap across four domains: crucible lifecycle verbs (charge/quench/enjoin), crucible diagnostic verbs (rack/hail/scry), consecration operation verbs (tally/ordain), role/regime authority verbs (knight/forfeit/levy/unmake/mantle/zero/proof), image-level artifact verbs (wrest/jettison/plumb), plus concept renames (censer→pentacle, bottle service→crucible) and MCM identity tier vocabulary.

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
**scry** (4, s) — networks (the perimeter). Solomonic divination — see through the veil to the hidden topology.

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
| `rbw-cs` | Scry networks | diagnostic |

Retiring: `rbw-s` (Start), `rbw-z` (Stop), `rbw-B` (ConnectBottle), `rbw-C` (ConnectCenser), `rbw-S` (ConnectSentry), `rbw-o` (ObserveNetworks).

Tabtarget examples:
- `tt/rbw-cC.Charge.tadmor.sh`
- `tt/rbw-cQ.Quench.tadmor.sh`
- `tt/rbw-ce.Enjoin.tadmor.sh`
- `tt/rbw-ch.Hail.tadmor.sh`
- `tt/rbw-cr.Rack.tadmor.sh`
- `tt/rbw-cs.Scry.tadmor.sh`

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
- `rbsc_scry` — diagnostic: networks
- `rbsc_agile` — service mode: ephemeral dispatch pattern
- `rbsc_sessile` — service mode: persistent service pattern

Retiring: `opbs_bottle_start`, `opbr_bottle_run`, `opss_sentry_start`.
Migrating display text: `at_bottle_service` → `rbsc_crucible`, `at_censer_container` → `rbsc_pentacle`, `at_agile_service` → `rbsc_agile`, `at_sessile_service` → `rbsc_sessile`.

Note: Full `at_*` retirement is a separate future heat. This heat only mints new `rbsc_` terms and retires the `op*_` terms they directly replace.

Confidence: HIGH.

## Full Cloud Verb Registry (post this heat)

**Consecration-level** (Solomonic/forge): abjure(A), enshrine(E), ordain(O), summon(S), tally(T), vouch(V)
**Image-level** (Solomonic/forge): jettison(J), plumb(P), wrest(W)
**Crucible** (forge): charge(C), enjoin(E), hail(H), quench(Q), rack(R), scry(S)
**Role/regime** (feudal/military): forfeit(F), knight(K), levy(L), mantle(M), proof(P), unmake(U), zero(Z)

## Forward Dependency: ₣Az (ark concept removal)

This heat follows existing `rbtgo_ark_*` naming convention for quoins (e.g., `rbtgo_ark_ordain`, `rbtgo_ark_plumb`). ₣Az will later remove the "ark" concept and revise these names. No pre-optimization — follow convention now, let ₣Az handle the sweep.

## References

- RBS0-SpecTop.adoc, RBRN-RegimeNameplate.adoc, RBSBS/RBSBR/RBSBK specs
- MCM-MetaConceptModel.adoc, AXLA-Lexicon.adoc
- rbob_bottle.sh, rbf_Foundry.sh, rbz_zipper.sh (full colophon registry)
- rbgg_cli.sh (Governor), rbgp_cli.sh (Payor), rblm_cli.sh (Marshal)
- Prior conversations: 260327 session, 260329 vocabulary election session, 260330 tally/ordain + Group A/B election sessions, 260331 observe→scry election

## Paces

### subdocument-acronym-rename (₢AxAAQ) [complete]

**[260331-0955] complete**

## Character
Pure mechanical file rename — no content changes. Maximizes git rename detection for ancestry tracking (`git log --follow` traces through each rename). Must complete before any verb-rename pace edits subdocument content.

## Scope
Rename 12 RBS0 subdocument files: update acronym to match new operation vocabulary. Update all references in CLAUDE.md, RBS0 include directives, and any other filename references.

## Rename Table

| Old | New | Old Filename | New Filename |
|-----|-----|--------------|--------------|
| RBSCK | RBSCL | RBSCK-consecration_check.adoc | RBSCL-consecration_tally.adoc |
| RBSDC | RBSDE | RBSDC-depot_create.adoc | RBSDE-depot_levy.adoc |
| RBSBS | RBSCC | RBSBS-bottle_start.adoc | RBSCC-crucible_charge.adoc |
| RBSBR | RBSCN | RBSBR-bottle_run.adoc | RBSCN-crucible_enjoin.adoc |
| RBSDI | RBSDK | RBSDI-director_create.adoc | RBSDK-director_knight.adoc |
| RBSRC | RBSRK | RBSRC-retriever_create.adoc | RBSRK-retriever_knight.adoc |
| RBSSD | RBSSF | RBSSD-sa_delete.adoc | RBSSF-sa_forfeit.adoc |
| RBSDD | RBSDU | RBSDD-depot_destroy.adoc | RBSDU-depot_unmake.adoc |
| RBSGR | RBSGM | RBSGR-governor_reset.adoc | RBSGM-governor_mantle.adoc |
| RBSIR | RBSIW | RBSIR-image_retrieve.adoc | RBSIW-image_wrest.adoc |
| RBSID | RBSIJ | RBSID-image_delete.adoc | RBSIJ-image_jettison.adoc |
| RBSAI | RBSAP | RBSAI-ark_inspect.adoc | RBSAP-ark_plumb.adoc |

## Near Acronym Rationale (4 blocked slots)
- RBSCL: C(onsecration) ta**L**ly — CT blocked by RBSCTD child
- RBSDE: D(epot) l**E**vy — DL blocked by RBSDL (depot_list)
- RBSCC: **C**(rucible) **C**(harge) — domain shift, double-letter precedent (RBSSS, RBSDD)
- RBSCN: C(rucible) e**N**join — CE blocked by RBSCE (command_exec)

## Reference Updates
- `CLAUDE.md`: Update all 12 acronym → file mappings in File Acronym Mappings section
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: Update include directives and inline filename references
- Grep repo for old filenames to catch stragglers (other spec cross-references, dockets, etc.)

## Approach
- `git mv` all 12 files — NO content changes (pure rename maximizes git rename detection)
- Update CLAUDE.md and RBS0 references
- Grep sweep for remaining old-name references
- Single commit containing all renames + reference updates

**[260330-1803] rough**

## Character
Pure mechanical file rename — no content changes. Maximizes git rename detection for ancestry tracking (`git log --follow` traces through each rename). Must complete before any verb-rename pace edits subdocument content.

## Scope
Rename 12 RBS0 subdocument files: update acronym to match new operation vocabulary. Update all references in CLAUDE.md, RBS0 include directives, and any other filename references.

## Rename Table

| Old | New | Old Filename | New Filename |
|-----|-----|--------------|--------------|
| RBSCK | RBSCL | RBSCK-consecration_check.adoc | RBSCL-consecration_tally.adoc |
| RBSDC | RBSDE | RBSDC-depot_create.adoc | RBSDE-depot_levy.adoc |
| RBSBS | RBSCC | RBSBS-bottle_start.adoc | RBSCC-crucible_charge.adoc |
| RBSBR | RBSCN | RBSBR-bottle_run.adoc | RBSCN-crucible_enjoin.adoc |
| RBSDI | RBSDK | RBSDI-director_create.adoc | RBSDK-director_knight.adoc |
| RBSRC | RBSRK | RBSRC-retriever_create.adoc | RBSRK-retriever_knight.adoc |
| RBSSD | RBSSF | RBSSD-sa_delete.adoc | RBSSF-sa_forfeit.adoc |
| RBSDD | RBSDU | RBSDD-depot_destroy.adoc | RBSDU-depot_unmake.adoc |
| RBSGR | RBSGM | RBSGR-governor_reset.adoc | RBSGM-governor_mantle.adoc |
| RBSIR | RBSIW | RBSIR-image_retrieve.adoc | RBSIW-image_wrest.adoc |
| RBSID | RBSIJ | RBSID-image_delete.adoc | RBSIJ-image_jettison.adoc |
| RBSAI | RBSAP | RBSAI-ark_inspect.adoc | RBSAP-ark_plumb.adoc |

## Near Acronym Rationale (4 blocked slots)
- RBSCL: C(onsecration) ta**L**ly — CT blocked by RBSCTD child
- RBSDE: D(epot) l**E**vy — DL blocked by RBSDL (depot_list)
- RBSCC: **C**(rucible) **C**(harge) — domain shift, double-letter precedent (RBSSS, RBSDD)
- RBSCN: C(rucible) e**N**join — CE blocked by RBSCE (command_exec)

## Reference Updates
- `CLAUDE.md`: Update all 12 acronym → file mappings in File Acronym Mappings section
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: Update include directives and inline filename references
- Grep repo for old filenames to catch stragglers (other spec cross-references, dockets, etc.)

## Approach
- `git mv` all 12 files — NO content changes (pure rename maximizes git rename detection)
- Update CLAUDE.md and RBS0 references
- Grep sweep for remaining old-name references
- Single commit containing all renames + reference updates

### consecration-checks-to-tally (₢AxAAK) [complete]

**[260331-1001] complete**

## Character
Same vertical shape as the charge/quench/enjoin rename paces. Mechanical find-and-replace across shell, dispatch, tabtarget, spec, and consumer docs. The read-only nature of this operation means no runtime risk — renaming perception doesn't break perception.

## Scope
Rename **checks → tally** for the Director's read-only registry consecration audit operation.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_check_consecrations()` → `rbf_tally()`
- Update the `# Consecration Check (rbw-Dc)` section comment → `# Consecration Tally (rbw-Dt)`
- Update `buc_doc_brief` string inside the function

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CHECK_CONSECRATIONS "rbw-Dc"` → `RBZ_TALLY_CONSECRATIONS "rbw-Dt"`

## Tabtarget layer
- Delete: `tt/rbw-Dc.DirectorChecksConsecrations.sh`
- Create: `tt/rbw-Dt.DirectorTalliesConsecrations.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`:
  - Quoin mapping: `rbtgo_consecration_check` → `rbtgo_consecration_tally`
  - Anchor: `[[rbtgo_consecration_check]]` → `[[rbtgo_consecration_tally]]`
  - Section heading and body text referencing "check" → "tally"
- `Tools/rbk/vov_veiled/RBSCK-consecration_check.adoc`: update internal references (consider file rename to RBSCK-consecration_tally.adoc — but RBSCK acronym is already allocated, so filename stays; update only content)
- All spec files that reference `{rbtgo_consecration_check}` — grep and update

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update `rbw-Dc` row
- `Tools/rbk/vov_veiled/README.consumer.md`: update all `rbw-Dc` / `DirectorChecksConsecrations` references

## Consecration Check Fact Map
- `RBS0-SpecTop.adoc` defines `rbcc_fact_consec_infix` — the `rbcc_` prefix references "consecration check." Evaluate whether to rename to `rbct_` (consecration tally) or leave as-is since it's a stable internal identifier.

## Approach
- Shell + zipper first (make routing work)
- Tabtarget second (make entry point work)
- Spec + consumer docs last
- Grep for any remaining `check_consecration` / `ChecksConsecration` / `rbw-Dc` stragglers

**[260330-1423] rough**

## Character
Same vertical shape as the charge/quench/enjoin rename paces. Mechanical find-and-replace across shell, dispatch, tabtarget, spec, and consumer docs. The read-only nature of this operation means no runtime risk — renaming perception doesn't break perception.

## Scope
Rename **checks → tally** for the Director's read-only registry consecration audit operation.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_check_consecrations()` → `rbf_tally()`
- Update the `# Consecration Check (rbw-Dc)` section comment → `# Consecration Tally (rbw-Dt)`
- Update `buc_doc_brief` string inside the function

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CHECK_CONSECRATIONS "rbw-Dc"` → `RBZ_TALLY_CONSECRATIONS "rbw-Dt"`

## Tabtarget layer
- Delete: `tt/rbw-Dc.DirectorChecksConsecrations.sh`
- Create: `tt/rbw-Dt.DirectorTalliesConsecrations.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`:
  - Quoin mapping: `rbtgo_consecration_check` → `rbtgo_consecration_tally`
  - Anchor: `[[rbtgo_consecration_check]]` → `[[rbtgo_consecration_tally]]`
  - Section heading and body text referencing "check" → "tally"
- `Tools/rbk/vov_veiled/RBSCK-consecration_check.adoc`: update internal references (consider file rename to RBSCK-consecration_tally.adoc — but RBSCK acronym is already allocated, so filename stays; update only content)
- All spec files that reference `{rbtgo_consecration_check}` — grep and update

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update `rbw-Dc` row
- `Tools/rbk/vov_veiled/README.consumer.md`: update all `rbw-Dc` / `DirectorChecksConsecrations` references

## Consecration Check Fact Map
- `RBS0-SpecTop.adoc` defines `rbcc_fact_consec_infix` — the `rbcc_` prefix references "consecration check." Evaluate whether to rename to `rbct_` (consecration tally) or leave as-is since it's a stable internal identifier.

## Approach
- Shell + zipper first (make routing work)
- Tabtarget second (make entry point work)
- Spec + consumer docs last
- Grep for any remaining `check_consecration` / `ChecksConsecration` / `rbw-Dc` stragglers

### consecration-creates-to-ordain (₢AxAAL) [complete]

**[260331-1007] complete**

## Character
Same vertical shape as the tally pace but slightly more complex — the function being renamed (`rbf_create`) is a dispatcher with mode branching and metadata chaining. The rename is mechanical but the function is load-bearing: it's the primary entry point for all consecration creation across all vessel modes.

## Scope
Rename **creates → ordain** for the Director's master consecration ceremony dispatch operation.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_create()` → `rbf_ordain()`
- Update `buc_doc_brief` string inside the function
- Update any internal references/comments that say "create" in the consecration context
- Check for callers of `rbf_create` outside the zipper dispatch path

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_CONSECRATION "rbw-DC"` → `RBZ_ORDAIN_CONSECRATION "rbw-DO"`

## Tabtarget layer
- Delete: `tt/rbw-DC.DirectorCreatesConsecration.sh`
- Create: `tt/rbw-DO.DirectorOrdainsConsecration.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`:
  - Mint new quoin: `rbtgo_ark_ordain` → `<<rbtgo_ark_ordain,rbf_ordain>>` (the dispatcher currently has no quoin — this is a new minting)
  - Add anchor `[[rbtgo_ark_ordain]]` with section heading and definition
  - Update any prose that references "creating a consecration" to use ordain language
- Spec files that reference the create operation (RBSAC-ark_conjure.adoc, RBSAG-ark_graft.adoc may reference the dispatcher)

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update `rbw-DC` row
- `Tools/rbk/vov_veiled/README.consumer.md`: update all `rbw-DC` / `DirectorCreatesConsecration` references

## Workbench
- `Tools/rbk/rbw_workbench.sh`: line 22 references `rbw-DC` in a qualification gate comment — update to `rbw-DO`

## Approach
- Shell + zipper first (make routing work)
- Tabtarget second (make entry point work)
- Spec: mint the new quoin, then update references
- Consumer docs last
- Grep for any remaining `rbf_create` / `CreatesConsecration` / `rbw-DC` / `RBZ_CREATE_CONSECRATION` stragglers

**[260330-1423] rough**

## Character
Same vertical shape as the tally pace but slightly more complex — the function being renamed (`rbf_create`) is a dispatcher with mode branching and metadata chaining. The rename is mechanical but the function is load-bearing: it's the primary entry point for all consecration creation across all vessel modes.

## Scope
Rename **creates → ordain** for the Director's master consecration ceremony dispatch operation.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_create()` → `rbf_ordain()`
- Update `buc_doc_brief` string inside the function
- Update any internal references/comments that say "create" in the consecration context
- Check for callers of `rbf_create` outside the zipper dispatch path

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_CONSECRATION "rbw-DC"` → `RBZ_ORDAIN_CONSECRATION "rbw-DO"`

## Tabtarget layer
- Delete: `tt/rbw-DC.DirectorCreatesConsecration.sh`
- Create: `tt/rbw-DO.DirectorOrdainsConsecration.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`:
  - Mint new quoin: `rbtgo_ark_ordain` → `<<rbtgo_ark_ordain,rbf_ordain>>` (the dispatcher currently has no quoin — this is a new minting)
  - Add anchor `[[rbtgo_ark_ordain]]` with section heading and definition
  - Update any prose that references "creating a consecration" to use ordain language
- Spec files that reference the create operation (RBSAC-ark_conjure.adoc, RBSAG-ark_graft.adoc may reference the dispatcher)

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update `rbw-DC` row
- `Tools/rbk/vov_veiled/README.consumer.md`: update all `rbw-DC` / `DirectorCreatesConsecration` references

## Workbench
- `Tools/rbk/rbw_workbench.sh`: line 22 references `rbw-DC` in a qualification gate comment — update to `rbw-DO`

## Approach
- Shell + zipper first (make routing work)
- Tabtarget second (make entry point work)
- Spec: mint the new quoin, then update references
- Consumer docs last
- Grep for any remaining `rbf_create` / `CreatesConsecration` / `rbw-DC` / `RBZ_CREATE_CONSECRATION` stragglers

### mcm-rename-quoin-inlay-sprue (₢AxAAB) [abandoned]

**[260328-0645] abandoned**

## Character
Mechanical rename with precision — small file count, grep-and-replace with careful attention to anchors, attributes, and definition text.

## Context
The MCM identity tier vocabulary (lemma/graven/intaglio) is being refreshed. Lemma has mathematical connotations, graven and intaglio lack the cohesion of the new craft-register trio. All three new words come from the mason/engraver/foundry world.

## Renames

lemma/lemmata → quoin/quoins (cornerstone that locks the structure)
graven → inlay/inlays (set permanently into a surface, recognized but not catalogued)
intaglio → sprue/sprues (the literal channel from the mould — wire-level token)

## File inventory (4 files, ~31 occurrences)

Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc — 18 hits (definitions, attribute references, anchors, all body references)
Tools/cmk/vov_veiled/AXLA-Lexicon.adoc — 8 hits (lexicon entries)
Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — 1 hit (single lemma reference)
.claude/jjm/jjp_uAlh.md — 4 hits (paddock historical context)

## Mechanical steps

1. MCM-MetaConceptModel.adoc: rename all mcm_lemma/mcm_graven/mcm_intaglio attributes, anchors, references, definition text, and plural forms
2. AXLA-Lexicon.adoc: rename lexicon entries and cross-references
3. RBS0-SpecTop.adoc: rename single lemma reference
4. jjp_uAlh.md: rename paddock references
5. Verify: grep for any surviving lemma/graven/intaglio references

## References
- MCM-MetaConceptModel.adoc lines 958-1005 — current definitions
- AXLA-Lexicon.adoc — lexicon entries

**[260327-1745] rough**

## Character
Mechanical rename with precision — small file count, grep-and-replace with careful attention to anchors, attributes, and definition text.

## Context
The MCM identity tier vocabulary (lemma/graven/intaglio) is being refreshed. Lemma has mathematical connotations, graven and intaglio lack the cohesion of the new craft-register trio. All three new words come from the mason/engraver/foundry world.

## Renames

lemma/lemmata → quoin/quoins (cornerstone that locks the structure)
graven → inlay/inlays (set permanently into a surface, recognized but not catalogued)
intaglio → sprue/sprues (the literal channel from the mould — wire-level token)

## File inventory (4 files, ~31 occurrences)

Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc — 18 hits (definitions, attribute references, anchors, all body references)
Tools/cmk/vov_veiled/AXLA-Lexicon.adoc — 8 hits (lexicon entries)
Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — 1 hit (single lemma reference)
.claude/jjm/jjp_uAlh.md — 4 hits (paddock historical context)

## Mechanical steps

1. MCM-MetaConceptModel.adoc: rename all mcm_lemma/mcm_graven/mcm_intaglio attributes, anchors, references, definition text, and plural forms
2. AXLA-Lexicon.adoc: rename lexicon entries and cross-references
3. RBS0-SpecTop.adoc: rename single lemma reference
4. jjp_uAlh.md: rename paddock references
5. Verify: grep for any surviving lemma/graven/intaglio references

## References
- MCM-MetaConceptModel.adoc lines 958-1005 — current definitions
- AXLA-Lexicon.adoc — lexicon entries

### spec-broach-quench-decant-vocabulary (₢AxAAA) [abandoned]

**[260328-0645] abandoned**

## Character
Spec-first vocabulary design — precise quoin minting with naming judgment, then mechanical spec updates.

## Context
Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. The agile/sessile service type distinction is load-bearing but was never surfaced in the nameplate or given proper verb support.

## New Verbs

broach (b for begin): Open a bottle service for use. Polymorphic on service mode.
quench (q for quit): Extinguish a bottle service. Polymorphic on service mode.
decant (d for dispatch): Pour ephemeral work through a broached agile envelope. Fails for sessile.

## Verb Behavior Matrix

broach sessile: stand up sentry + censer + bottle (full service)
broach agile: stand up sentry + censer only (envelope)
quench sessile: tear down bottle + censer + sentry
quench agile: tear down censer + sentry
decant sessile: FAIL (wrong mode)
decant agile not broached: FAIL (not broached)
decant agile broached: dispatch ephemeral bottle into running envelope

## Operator Mental Model

Sessile: broach ... service runs ... quench
Agile: broach ... decant decant decant ... quench

## Quoin Inventory

### New operation quoins (prefix rbtlo + first letter + _word)

rbtlob_broach — Broach Rule — open a bottle service for use; polymorphic on service mode
rbtloq_quench — Quench Rule — extinguish a bottle service; polymorphic on service mode
rbtlod_decant — Decant Rule — dispatch ephemeral bottle through a broached agile envelope; fails for sessile

### New RBRN regime variable quoins

rbrn_service_mode — RBRN_SERVICE_MODE — service lifecycle mode for the bottle service instance (enumeration)
rbrn_service_mode_sessile — long-running service: broach starts all three containers, quench stops all three
rbrn_service_mode_agile — dispatch envelope: broach starts sentry+censer, decant dispatches ephemeral bottles, quench tears down envelope

### Quoins to retire

opbs_bottle_start → superseded by rbtlob_broach
opbr_bottle_run → superseded by rbtlod_decant
opss_sentry_start → demoted to internal sequence within broach

### Quoins to update (definition text only, keep legacy prefix)

at_sessile_service — reference broach/quench, reference rbrn_service_mode_sessile
at_agile_service — reference broach/quench/decant, reference rbrn_service_mode_agile
at_bottle_service — weave in service mode concept

## Spec Subdocument Changes

RBSBS-bottle_start.adoc → rename/rewrite for broach
RBSBR-bottle_run.adoc → rename/rewrite for decant
New subdocument for quench
RBSBK-bottle_cleanup.adoc → now internal to quench

## Scope

Spec-only. Implementation changes to rbob_bottle.sh, tabtargets, and workbench are a separate pace.

## References

RBS0-SpecTop.adoc — main spec, local operations section (lines 273-284 mapping, 1179-1252 operations)
RBRN-RegimeNameplate.adoc — nameplate spec
RBSBS-bottle_start.adoc — current sessile start
RBSBR-bottle_run.adoc — current agile run
RBSBK-bottle_cleanup.adoc — cleanup sequence
RBSSS-sentry_start.adoc — sentry start (demoting to internal)

**[260327-1805] rough**

## Character
Spec-first vocabulary design — precise quoin minting with naming judgment, then mechanical spec updates.

## Context
Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. The agile/sessile service type distinction is load-bearing but was never surfaced in the nameplate or given proper verb support.

## New Verbs

broach (b for begin): Open a bottle service for use. Polymorphic on service mode.
quench (q for quit): Extinguish a bottle service. Polymorphic on service mode.
decant (d for dispatch): Pour ephemeral work through a broached agile envelope. Fails for sessile.

## Verb Behavior Matrix

broach sessile: stand up sentry + censer + bottle (full service)
broach agile: stand up sentry + censer only (envelope)
quench sessile: tear down bottle + censer + sentry
quench agile: tear down censer + sentry
decant sessile: FAIL (wrong mode)
decant agile not broached: FAIL (not broached)
decant agile broached: dispatch ephemeral bottle into running envelope

## Operator Mental Model

Sessile: broach ... service runs ... quench
Agile: broach ... decant decant decant ... quench

## Quoin Inventory

### New operation quoins (prefix rbtlo + first letter + _word)

rbtlob_broach — Broach Rule — open a bottle service for use; polymorphic on service mode
rbtloq_quench — Quench Rule — extinguish a bottle service; polymorphic on service mode
rbtlod_decant — Decant Rule — dispatch ephemeral bottle through a broached agile envelope; fails for sessile

### New RBRN regime variable quoins

rbrn_service_mode — RBRN_SERVICE_MODE — service lifecycle mode for the bottle service instance (enumeration)
rbrn_service_mode_sessile — long-running service: broach starts all three containers, quench stops all three
rbrn_service_mode_agile — dispatch envelope: broach starts sentry+censer, decant dispatches ephemeral bottles, quench tears down envelope

### Quoins to retire

opbs_bottle_start → superseded by rbtlob_broach
opbr_bottle_run → superseded by rbtlod_decant
opss_sentry_start → demoted to internal sequence within broach

### Quoins to update (definition text only, keep legacy prefix)

at_sessile_service — reference broach/quench, reference rbrn_service_mode_sessile
at_agile_service — reference broach/quench/decant, reference rbrn_service_mode_agile
at_bottle_service — weave in service mode concept

## Spec Subdocument Changes

RBSBS-bottle_start.adoc → rename/rewrite for broach
RBSBR-bottle_run.adoc → rename/rewrite for decant
New subdocument for quench
RBSBK-bottle_cleanup.adoc → now internal to quench

## Scope

Spec-only. Implementation changes to rbob_bottle.sh, tabtargets, and workbench are a separate pace.

## References

RBS0-SpecTop.adoc — main spec, local operations section (lines 273-284 mapping, 1179-1252 operations)
RBRN-RegimeNameplate.adoc — nameplate spec
RBSBS-bottle_start.adoc — current sessile start
RBSBR-bottle_run.adoc — current agile run
RBSBK-bottle_cleanup.adoc — cleanup sequence
RBSSS-sentry_start.adoc — sentry start (demoting to internal)

**[260327-1805] rough**

## Character
Spec-first vocabulary design — precise quoin minting with naming judgment, then mechanical spec updates.

## Context
Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. The agile/sessile service type distinction is load-bearing but was never surfaced in the nameplate or given proper verb support.

## New Verbs

broach (b for begin): Open a bottle service for use. Polymorphic on service mode.
quench (q for quit): Extinguish a bottle service. Polymorphic on service mode.
decant (d for dispatch): Pour ephemeral work through a broached agile envelope. Fails for sessile.

## Verb Behavior Matrix

broach sessile: stand up sentry + censer + bottle (full service)
broach agile: stand up sentry + censer only (envelope)
quench sessile: tear down bottle + censer + sentry
quench agile: tear down censer + sentry
decant sessile: FAIL (wrong mode)
decant agile not broached: FAIL (not broached)
decant agile broached: dispatch ephemeral bottle into running envelope

## Operator Mental Model

Sessile: broach ... service runs ... quench
Agile: broach ... decant decant decant ... quench

## Quoin Inventory

### New operation quoins (prefix rbtlo + first letter + _word)

rbtlob_broach — Broach Rule — open a bottle service for use; polymorphic on service mode
rbtloq_quench — Quench Rule — extinguish a bottle service; polymorphic on service mode
rbtlod_decant — Decant Rule — dispatch ephemeral bottle through a broached agile envelope; fails for sessile

### New RBRN regime variable quoins

rbrn_service_mode — RBRN_SERVICE_MODE — service lifecycle mode for the bottle service instance (enumeration)
rbrn_service_mode_sessile — long-running service: broach starts all three containers, quench stops all three
rbrn_service_mode_agile — dispatch envelope: broach starts sentry+censer, decant dispatches ephemeral bottles, quench tears down envelope

### Quoins to retire

opbs_bottle_start → superseded by rbtlob_broach
opbr_bottle_run → superseded by rbtlod_decant
opss_sentry_start → demoted to internal sequence within broach

### Quoins to update (definition text only, keep legacy prefix)

at_sessile_service — reference broach/quench, reference rbrn_service_mode_sessile
at_agile_service — reference broach/quench/decant, reference rbrn_service_mode_agile
at_bottle_service — weave in service mode concept

## Spec Subdocument Changes

RBSBS-bottle_start.adoc → rename/rewrite for broach
RBSBR-bottle_run.adoc → rename/rewrite for decant
New subdocument for quench
RBSBK-bottle_cleanup.adoc → now internal to quench

## Scope

Spec-only. Implementation changes to rbob_bottle.sh, tabtargets, and workbench are a separate pace.

## References

RBS0-SpecTop.adoc — main spec, local operations section (lines 273-284 mapping, 1179-1252 operations)
RBRN-RegimeNameplate.adoc — nameplate spec
RBSBS-bottle_start.adoc — current sessile start
RBSBR-bottle_run.adoc — current agile run
RBSBK-bottle_cleanup.adoc — cleanup sequence
RBSSS-sentry_start.adoc — sentry start (demoting to internal)

**[260327-1726] rough**

## Character
Design conversation turned specification — architectural vocabulary with naming judgment.

## Context
Recipe Bottle's cloud operations have rich liturgical verbs (conjure, abjure, summon, graft, vouch, enshrine, inscribe) but local bottle service lifecycle uses generic words (start, run, cleanup) with no verb for stop. The agile/sessile service type distinction is load-bearing but was never surfaced in the nameplate or given proper verb support.

## Discovery
- `bottle_start` stands up sentry+censer+bottle (all persistent) — sessile pattern
- `bottle_run` dispatches an ephemeral bottle into an already-running sentry+censer envelope — agile pattern
- The deferred third pattern (persistent envelope, ephemeral workloads) was actually implemented as `bottle_run` all along
- No operation exists for stopping/tearing down a service
- agile/sessile are retained as service type names — load-bearing distinction, unique words

## New Vocabulary

broach (b for begin): Sessile — stand up sentry+censer+bottle. Agile — stand up sentry+censer envelope.
quench (q for quit): Sessile — tear down bottle+censer+sentry. Agile — tear down censer+sentry.
assay (a for agile): Sessile — fail (wrong mode). Agile — dispatch ephemeral bottle into running envelope.

## Assay failure modes
- Sessile service: Cannot assay a sessile service
- Agile but not broached: Service not broached, broach first then assay

## Deliverables

### A. Nameplate mode field
Add agile/sessile mode declaration to RBRN nameplate spec. Every nameplate must declare its service type. Single source of truth for broach/quench/assay dispatch.

### B. RBS0 linked terms
Mint new linked terms and operation definitions. Retire opbs_bottle_start, opbr_bottle_run. Add broach, quench, assay operations. Update at_sessile_service and at_agile_service display terms.

### C. Spec subdocuments
Rename/rewrite RBSBS-bottle_start.adoc and RBSBR-bottle_run.adoc. Create new subdocument for quench. Update RBSBK-bottle_cleanup.adoc role (now internal to quench).

### D. Operator mental model
Sessile: broach ... service runs ... quench
Agile: broach ... assay assay assay ... quench

## Prefix decisions needed
The op prefix pattern needs new suffixes. Current: opbs_ (bottle start), opbr_ (bottle run), opss_ (sentry start). Propose new allocations following terminal exclusivity.

## References
- RBS0-SpecTop.adoc — main spec, local operations section
- RBRN-RegimeNameplate.adoc — nameplate spec
- RBSBS-bottle_start.adoc, RBSBR-bottle_run.adoc, RBSBK-bottle_cleanup.adoc
- rbob_bottle.sh — implementation

### mcm-tier-rename (₢AxAAC) [complete]

**[260331-1012] complete**

## Character
Mechanical find-and-replace with careful attention to plural forms and surrounding context. The terms appear in definition structures (MCM linked terms), so each replacement must preserve the three-part structure: attribute reference, replacement text, and definition.

## Scope
Rename the MCM identity tier vocabulary across 4 files (~31 occurrences):
- **lemma/lemmata → quoin/quoins** — Full catalogue entry: mapping + anchor + definition
- **graven → inlay/inlays** — Prefix-named, subdocument-scoped, no catalogue entry
- **intaglio → sprue/sprues** — Wire-level token in backticks

## Files
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` (~18 occurrences)
- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` (~8 occurrences)
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` (~1 occurrence)
- `.claude/jjm/jjp_uAlh.md` (~4 occurrences, retired heat memo)

## Approach
- Search each file for all three old terms (including plural forms: lemmata, gravens if any)
- Replace with new terms, preserving AsciiDoc structure
- Verify no dangling cross-references after replacement

**[260330-0800] rough**

## Character
Mechanical find-and-replace with careful attention to plural forms and surrounding context. The terms appear in definition structures (MCM linked terms), so each replacement must preserve the three-part structure: attribute reference, replacement text, and definition.

## Scope
Rename the MCM identity tier vocabulary across 4 files (~31 occurrences):
- **lemma/lemmata → quoin/quoins** — Full catalogue entry: mapping + anchor + definition
- **graven → inlay/inlays** — Prefix-named, subdocument-scoped, no catalogue entry
- **intaglio → sprue/sprues** — Wire-level token in backticks

## Files
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` (~18 occurrences)
- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` (~8 occurrences)
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` (~1 occurrence)
- `.claude/jjm/jjp_uAlh.md` (~4 occurrences, retired heat memo)

## Approach
- Search each file for all three old terms (including plural forms: lemmata, gravens if any)
- Replace with new terms, preserving AsciiDoc structure
- Verify no dangling cross-references after replacement

### censer-to-pentacle (₢AxAAD) [complete]

**[260331-1022] complete**

## Character
Wide but mechanical rename. The word "censer" appears ~121 times across ~14 files. Most are straightforward text substitution, but the compose file and shell variable names require careful attention to avoid breaking runtime behavior.

## Scope
Rename **censer → pentacle** everywhere: container names, function names, variable names, AsciiDoc references, compose service definitions, tabtarget content, and public-facing assets.

## Files (by risk)
**Critical (runtime):**
- `.rbk/rbob_compose.yml` — service name, container name, network aliases
- `Tools/rbk/rbob_bottle.sh` — ZRBOB_CENSER variable, rbob_connect_censer function
- `Tools/rbk/rboc_censer.sh` — entire module (consider renaming file to rboc_pentacle.sh)
- `rbev-vessels/rbev-sentry-debian-slim/rboc_censer.sh` — vessel copy

**High (dispatch):**
- `Tools/rbk/rbz_zipper.sh` — buz_enroll registration for rbw-C colophon

**Medium (spec/docs):**
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — ~10 occurrences
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`
- `Tools/rbk/rboo_observe.sh`
- `Tools/rbk/rbob_cli.sh`
- `Tools/rbk/rbtb_testbench.sh`
- `Tools/rbk/rbts/rbtcns_TadmorSecurity.sh`

**Public-facing:**
- `index.html` — 3 "Censer" references
- `rbm-abstract-drawio.svg` — check for any censer references

**Low (tracking/context):**
- `.claude/jjm/jjg_gallops.json` — will update via normal jjx operations
- `.claude/settings.local.json`

## Approach
- Start with compose and shell (runtime correctness first)
- Then spec and docs
- Then public-facing assets
- Skip gallops.json (updates organically) and retired memos (sweep pace verifies)
- Verify `tt/rbw-C.ConnectCenser.*` tabtargets still work (they'll be retired in a later pace, but shouldn't break here)

**[260330-0807] rough**

## Character
Wide but mechanical rename. The word "censer" appears ~121 times across ~14 files. Most are straightforward text substitution, but the compose file and shell variable names require careful attention to avoid breaking runtime behavior.

## Scope
Rename **censer → pentacle** everywhere: container names, function names, variable names, AsciiDoc references, compose service definitions, tabtarget content, and public-facing assets.

## Files (by risk)
**Critical (runtime):**
- `.rbk/rbob_compose.yml` — service name, container name, network aliases
- `Tools/rbk/rbob_bottle.sh` — ZRBOB_CENSER variable, rbob_connect_censer function
- `Tools/rbk/rboc_censer.sh` — entire module (consider renaming file to rboc_pentacle.sh)
- `rbev-vessels/rbev-sentry-debian-slim/rboc_censer.sh` — vessel copy

**High (dispatch):**
- `Tools/rbk/rbz_zipper.sh` — buz_enroll registration for rbw-C colophon

**Medium (spec/docs):**
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — ~10 occurrences
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`
- `Tools/rbk/rboo_observe.sh`
- `Tools/rbk/rbob_cli.sh`
- `Tools/rbk/rbtb_testbench.sh`
- `Tools/rbk/rbts/rbtcns_TadmorSecurity.sh`

**Public-facing:**
- `index.html` — 3 "Censer" references
- `rbm-abstract-drawio.svg` — check for any censer references

**Low (tracking/context):**
- `.claude/jjm/jjg_gallops.json` — will update via normal jjx operations
- `.claude/settings.local.json`

## Approach
- Start with compose and shell (runtime correctness first)
- Then spec and docs
- Then public-facing assets
- Skip gallops.json (updates organically) and retired memos (sweep pace verifies)
- Verify `tt/rbw-C.ConnectCenser.*` tabtargets still work (they'll be retired in a later pace, but shouldn't break here)

**[260330-0800] rough**

## Character
Wide but mechanical rename. The word "censer" appears ~121 times across ~14 files. Most are straightforward text substitution, but the compose file and shell variable names require careful attention to avoid breaking runtime behavior.

## Scope
Rename **censer → pentacle** everywhere: container names, function names, variable names, AsciiDoc references, compose service definitions, and tabtarget content.

## Files (by risk)
**Critical (runtime):**
- `.rbk/rbob_compose.yml` — service name, container name, network aliases
- `Tools/rbk/rbob_bottle.sh` — ZRBOB_CENSER variable, rbob_connect_censer function
- `Tools/rbk/rboc_censer.sh` — entire module (consider renaming file to rboc_pentacle.sh)
- `rbev-vessels/rbev-sentry-debian-slim/rboc_censer.sh` — vessel copy

**High (dispatch):**
- `Tools/rbk/rbz_zipper.sh` — buz_enroll registration for rbw-C colophon

**Medium (spec/docs):**
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — ~10 occurrences
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`
- `Tools/rbk/rboo_observe.sh`
- `Tools/rbk/rbob_cli.sh`
- `Tools/rbk/rbtb_testbench.sh`
- `Tools/rbk/rbts/rbtcns_TadmorSecurity.sh`

**Low (tracking/context):**
- `.claude/jjm/jjg_gallops.json` — will update via normal jjx operations
- `.claude/settings.local.json`

## Approach
- Start with compose and shell (runtime correctness first)
- Then spec and docs
- Skip gallops.json (updates organically) and retired memos (sweep pace)
- Verify `tt/rbw-C.ConnectCenser.*` tabtargets still work (they'll be retired in a later pace, but shouldn't break here)

### bottle-service-to-crucible (₢AxAAE) [complete]

**[260331-1033] complete**

## Character
Conceptual rename in specification, documentation, and public-facing assets. Less mechanical than the censer rename — "bottle service" is a two-word phrase that appears in varied grammatical contexts ("the bottle service", "bottle service containers", "a bottle service lifecycle"). Requires reading each occurrence in context.

## Scope
Rename the **bottle service** concept (the tandem sentry+pentacle+bottle assembly) to **crucible** in specs, documentation, and public-facing assets. This is the conceptual container, not any individual container or operation.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary spec, highest occurrence count
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — cosmology introduction
- `Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc` — will reference crucible concept
- `Tools/rbk/vov_veiled/RBSBR-bottle_run.adoc`
- `Tools/rbk/vov_veiled/RBSBL-bottle_launch.adoc`
- `Tools/rbk/vov_veiled/RBSSS-sentry_start.adoc`
- Shell code comments where "bottle service" appears as a concept description
- `index.html` — 3 "Bottle Service", 2 "bottle_service"
- `rbm-abstract-drawio.svg` — 3 "Bottle Service"

## Approach
- Search for "bottle.service" (regex) to catch all grammatical forms
- Replace with "crucible" where it refers to the assembly concept
- Do NOT rename things that refer to an individual bottle container — only the composite
- Update AsciiDoc linked terms: `at_bottle_service` → `rbsc_crucible` (but full quoin minting is a later pace)
- Update public-facing assets alongside spec work

**[260330-0807] rough**

## Character
Conceptual rename in specification, documentation, and public-facing assets. Less mechanical than the censer rename — "bottle service" is a two-word phrase that appears in varied grammatical contexts ("the bottle service", "bottle service containers", "a bottle service lifecycle"). Requires reading each occurrence in context.

## Scope
Rename the **bottle service** concept (the tandem sentry+pentacle+bottle assembly) to **crucible** in specs, documentation, and public-facing assets. This is the conceptual container, not any individual container or operation.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary spec, highest occurrence count
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — cosmology introduction
- `Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc` — will reference crucible concept
- `Tools/rbk/vov_veiled/RBSBR-bottle_run.adoc`
- `Tools/rbk/vov_veiled/RBSBL-bottle_launch.adoc`
- `Tools/rbk/vov_veiled/RBSSS-sentry_start.adoc`
- Shell code comments where "bottle service" appears as a concept description
- `index.html` — 3 "Bottle Service", 2 "bottle_service"
- `rbm-abstract-drawio.svg` — 3 "Bottle Service"

## Approach
- Search for "bottle.service" (regex) to catch all grammatical forms
- Replace with "crucible" where it refers to the assembly concept
- Do NOT rename things that refer to an individual bottle container — only the composite
- Update AsciiDoc linked terms: `at_bottle_service` → `rbsc_crucible` (but full quoin minting is a later pace)
- Update public-facing assets alongside spec work

**[260330-0801] rough**

## Character
Conceptual rename in specification and documentation. Less mechanical than the censer rename — "bottle service" is a two-word phrase that appears in varied grammatical contexts ("the bottle service", "bottle service containers", "a bottle service lifecycle"). Requires reading each occurrence in context.

## Scope
Rename the **bottle service** concept (the tandem sentry+pentacle+bottle assembly) to **crucible** in specs and documentation. This is the conceptual container, not any individual container or operation.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary spec, highest occurrence count
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — cosmology introduction
- `Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc` — will reference crucible concept
- `Tools/rbk/vov_veiled/RBSBR-bottle_run.adoc`
- `Tools/rbk/vov_veiled/RBSBL-bottle_launch.adoc`
- `Tools/rbk/vov_veiled/RBSSS-sentry_start.adoc`
- Shell code comments where "bottle service" appears as a concept description

## Approach
- Search for "bottle.service" (regex) to catch all grammatical forms
- Replace with "crucible" where it refers to the assembly concept
- Do NOT rename things that refer to an individual bottle container — only the composite
- Update AsciiDoc linked terms: `at_bottle_service` → `rbsc_crucible` (but full quoin minting is a later pace)

### lifecycle-verbs-charge-quench (₢AxAAF) [complete]

**[260331-1039] complete**

## Character
The most structurally complex pace. Renames two operations (start→charge, stop→quench) across the full vertical: shell functions, zipper dispatch, colophon routing, tabtarget files, and spec. Must leave the system runnable after completion.

## Scope
Replace **start → charge** and **stop → quench** for crucible lifecycle operations.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_start()` → `rbob_charge()`, `rbob_stop()` → `rbob_quench()`
- Internal function references and comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_START "rbw-s"` → `RBZ_CRUCIBLE_CHARGE "rbw-cC"`
  - `RBZ_BOTTLE_STOP "rbw-z"` → `RBZ_CRUCIBLE_QUENCH "rbw-cQ"`

## Tabtarget layer
- Delete: `tt/rbw-s.Start.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-z.Stop.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cC.Charge.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cQ.Quench.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc`: update to reference charge verb (consider file rename to RBSBS?)

## Approach
- Shell + zipper first (make routing work)
- Tabtargets second (make entry points work)
- Spec last (documentation catches up)
- Verify at least one charge/quench cycle works on tadmor if environment allows

**[260330-0801] rough**

## Character
The most structurally complex pace. Renames two operations (start→charge, stop→quench) across the full vertical: shell functions, zipper dispatch, colophon routing, tabtarget files, and spec. Must leave the system runnable after completion.

## Scope
Replace **start → charge** and **stop → quench** for crucible lifecycle operations.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_start()` → `rbob_charge()`, `rbob_stop()` → `rbob_quench()`
- Internal function references and comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_START "rbw-s"` → `RBZ_CRUCIBLE_CHARGE "rbw-cC"`
  - `RBZ_BOTTLE_STOP "rbw-z"` → `RBZ_CRUCIBLE_QUENCH "rbw-cQ"`

## Tabtarget layer
- Delete: `tt/rbw-s.Start.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-z.Stop.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cC.Charge.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cQ.Quench.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc`: update to reference charge verb (consider file rename to RBSBS?)

## Approach
- Shell + zipper first (make routing work)
- Tabtargets second (make entry points work)
- Spec last (documentation catches up)
- Verify at least one charge/quench cycle works on tadmor if environment allows

### operational-verb-enjoin (₢AxAAG) [complete]

**[260331-1044] complete**

## Character
Same vertical shape as the charge/quench pace but narrower — only one operation. Replaces the agile dispatch command (bottle_run → enjoin).

## Scope
Replace **run → enjoin** for agile crucible dispatch.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_run()` → `rbob_enjoin()` (or equivalent current function name for bottle_run)
- Update internal references and comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - Find the bottle_run registration → `RBZ_CRUCIBLE_ENJOIN "rbw-ce"`

## Tabtarget layer
- Identify and delete existing run tabtargets (likely `tt/rbw-r.Run.*` or similar)
- Create: `tt/rbw-ce.Enjoin.{tadmor,srjcl,pluml}.sh` (only for nameplates that support agile mode)

## Spec layer
- `Tools/rbk/vov_veiled/RBSBR-bottle_run.adoc`: update to reference enjoin verb
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions

## Approach
- Same order as charge/quench: shell → dispatch → tabtargets → spec
- Note: enjoin on a sessile nameplate should fail — verify the mode guard exists or add it

**[260330-0801] rough**

## Character
Same vertical shape as the charge/quench pace but narrower — only one operation. Replaces the agile dispatch command (bottle_run → enjoin).

## Scope
Replace **run → enjoin** for agile crucible dispatch.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_run()` → `rbob_enjoin()` (or equivalent current function name for bottle_run)
- Update internal references and comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - Find the bottle_run registration → `RBZ_CRUCIBLE_ENJOIN "rbw-ce"`

## Tabtarget layer
- Identify and delete existing run tabtargets (likely `tt/rbw-r.Run.*` or similar)
- Create: `tt/rbw-ce.Enjoin.{tadmor,srjcl,pluml}.sh` (only for nameplates that support agile mode)

## Spec layer
- `Tools/rbk/vov_veiled/RBSBR-bottle_run.adoc`: update to reference enjoin verb
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions

## Approach
- Same order as charge/quench: shell → dispatch → tabtargets → spec
- Note: enjoin on a sessile nameplate should fail — verify the mode guard exists or add it

### diagnostic-verbs-rack-hail-scry (₢AxAAH) [complete]

**[260331-1050] complete**

## Character
Three parallel renames with identical vertical shape. Could be done as three sub-steps or all at once since they don't interact.

## Scope
Replace diagnostic/interactive commands:
- **ConnectBottle → Rack** (rbw-B → rbw-cr) — compel the demon to reveal state
- **ConnectSentry → Hail** (rbw-S → rbw-ch) — call out to the guard
- **ObserveNetworks → Scry** (rbw-o → rbw-cs) — Solomonic divination, see through the veil to the hidden topology

Also: **ConnectCenser is eliminated** (rbw-C) — pentacle interactive access has no diagnostic use case. Delete without replacement.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_connect_bottle()` → `rbob_rack()`, `rbob_connect_sentry()` → `rbob_hail()`
- `Tools/rbk/rboo_observe.sh`: rename to `rboo_scry.sh`, rename functions accordingly
- Delete `rbob_connect_censer()` — no replacement

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_CONNECT "rbw-B"` → `RBZ_CRUCIBLE_RACK "rbw-cr"`
  - `RBZ_BOTTLE_SENTRY "rbw-S"` → `RBZ_CRUCIBLE_HAIL "rbw-ch"`
  - `RBZ_BOTTLE_OBSERVE "rbw-o"` → `RBZ_CRUCIBLE_SCRY "rbw-cs"`
  - Delete `RBZ_BOTTLE_CENSER "rbw-C"` registration

## Tabtarget layer
- Delete: `tt/rbw-B.ConnectBottle.{tadmor,srjcl}.sh`
- Delete: `tt/rbw-C.ConnectCenser.tadmor.sh`
- Delete: `tt/rbw-S.ConnectSentry.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-o.ObserveNetworks.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cr.Rack.{tadmor,srjcl}.sh`
- Create: `tt/rbw-ch.Hail.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cs.Scry.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSCE-command_exec.adoc`: update references

## Approach
- Do all three renames together — they share no state
- Shell → dispatch → tabtargets → spec

**[260331-0735] rough**

## Character
Three parallel renames with identical vertical shape. Could be done as three sub-steps or all at once since they don't interact.

## Scope
Replace diagnostic/interactive commands:
- **ConnectBottle → Rack** (rbw-B → rbw-cr) — compel the demon to reveal state
- **ConnectSentry → Hail** (rbw-S → rbw-ch) — call out to the guard
- **ObserveNetworks → Scry** (rbw-o → rbw-cs) — Solomonic divination, see through the veil to the hidden topology

Also: **ConnectCenser is eliminated** (rbw-C) — pentacle interactive access has no diagnostic use case. Delete without replacement.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_connect_bottle()` → `rbob_rack()`, `rbob_connect_sentry()` → `rbob_hail()`
- `Tools/rbk/rboo_observe.sh`: rename to `rboo_scry.sh`, rename functions accordingly
- Delete `rbob_connect_censer()` — no replacement

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_CONNECT "rbw-B"` → `RBZ_CRUCIBLE_RACK "rbw-cr"`
  - `RBZ_BOTTLE_SENTRY "rbw-S"` → `RBZ_CRUCIBLE_HAIL "rbw-ch"`
  - `RBZ_BOTTLE_OBSERVE "rbw-o"` → `RBZ_CRUCIBLE_SCRY "rbw-cs"`
  - Delete `RBZ_BOTTLE_CENSER "rbw-C"` registration

## Tabtarget layer
- Delete: `tt/rbw-B.ConnectBottle.{tadmor,srjcl}.sh`
- Delete: `tt/rbw-C.ConnectCenser.tadmor.sh`
- Delete: `tt/rbw-S.ConnectSentry.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-o.ObserveNetworks.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cr.Rack.{tadmor,srjcl}.sh`
- Create: `tt/rbw-ch.Hail.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cs.Scry.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSCE-command_exec.adoc`: update references

## Approach
- Do all three renames together — they share no state
- Shell → dispatch → tabtargets → spec

**[260331-0735] rough**

## Character
Three parallel renames with identical vertical shape. Could be done as three sub-steps or all at once since they don't interact.

## Scope
Replace diagnostic/interactive commands:
- **ConnectBottle → Rack** (rbw-B → rbw-cr) — compel the demon to reveal state
- **ConnectSentry → Hail** (rbw-S → rbw-ch) — call out to the guard
- **ObserveNetworks → Scry** (rbw-o → rbw-cs) — Solomonic divination, see through the veil to the hidden topology

Also: **ConnectCenser is eliminated** (rbw-C) — pentacle interactive access has no diagnostic use case. Delete without replacement.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_connect_bottle()` → `rbob_rack()`, `rbob_connect_sentry()` → `rbob_hail()`
- `Tools/rbk/rboo_observe.sh`: rename to `rboo_scry.sh`, rename functions accordingly
- Delete `rbob_connect_censer()` — no replacement

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_CONNECT "rbw-B"` → `RBZ_CRUCIBLE_RACK "rbw-cr"`
  - `RBZ_BOTTLE_SENTRY "rbw-S"` → `RBZ_CRUCIBLE_HAIL "rbw-ch"`
  - `RBZ_BOTTLE_OBSERVE "rbw-o"` → `RBZ_CRUCIBLE_SCRY "rbw-cs"`
  - Delete `RBZ_BOTTLE_CENSER "rbw-C"` registration

## Tabtarget layer
- Delete: `tt/rbw-B.ConnectBottle.{tadmor,srjcl}.sh`
- Delete: `tt/rbw-C.ConnectCenser.tadmor.sh`
- Delete: `tt/rbw-S.ConnectSentry.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-o.ObserveNetworks.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cr.Rack.{tadmor,srjcl}.sh`
- Create: `tt/rbw-ch.Hail.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cs.Scry.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSCE-command_exec.adoc`: update references

## Approach
- Do all three renames together — they share no state
- Shell → dispatch → tabtargets → spec

**[260330-0801] rough**

## Character
Three parallel renames with identical vertical shape. Could be done as three sub-steps or all at once since they don't interact.

## Scope
Replace diagnostic/interactive commands:
- **ConnectBottle → Rack** (rbw-B → rbw-cr) — compel the demon to reveal state
- **ConnectSentry → Hail** (rbw-S → rbw-ch) — call out to the guard
- **ObserveNetworks → Observe** (rbw-o → rbw-co) — retained verb, new colophon

Also: **ConnectCenser is eliminated** (rbw-C) — pentacle interactive access has no diagnostic use case. Delete without replacement.

## Shell layer
- `Tools/rbk/rbob_bottle.sh`: rename `rbob_connect_bottle()` → `rbob_rack()`, `rbob_connect_sentry()` → `rbob_hail()`
- `Tools/rbk/rboo_observe.sh`: function rename if needed, or keep internal name and just re-route
- Delete `rbob_connect_censer()` — no replacement

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll registrations
  - `RBZ_BOTTLE_CONNECT "rbw-B"` → `RBZ_CRUCIBLE_RACK "rbw-cr"`
  - `RBZ_BOTTLE_SENTRY "rbw-S"` → `RBZ_CRUCIBLE_HAIL "rbw-ch"`
  - `RBZ_BOTTLE_OBSERVE "rbw-o"` → `RBZ_CRUCIBLE_OBSERVE "rbw-co"`
  - Delete `RBZ_BOTTLE_CENSER "rbw-C"` registration

## Tabtarget layer
- Delete: `tt/rbw-B.ConnectBottle.{tadmor,srjcl}.sh`
- Delete: `tt/rbw-C.ConnectCenser.tadmor.sh`
- Delete: `tt/rbw-S.ConnectSentry.{tadmor,srjcl,pluml}.sh`
- Delete: `tt/rbw-o.ObserveNetworks.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-cr.Rack.{tadmor,srjcl}.sh`
- Create: `tt/rbw-ch.Hail.{tadmor,srjcl,pluml}.sh`
- Create: `tt/rbw-co.Observe.{tadmor,srjcl,pluml}.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update operation definitions
- `Tools/rbk/vov_veiled/RBSCE-command_exec.adoc`: update references

## Approach
- Do all three renames together — they share no state
- Shell → dispatch → tabtargets → spec

### quoin-minting-rbsc-terms (₢AxAAI) [complete]

**[260331-1107] complete**

## Character
Spec-focused, requires MCM fluency. Minting new AsciiDoc linked terms and retiring old ones. Each quoin needs the three-part structure: attribute mapping, anchor definition, and definition text.

## Scope
Mint the **rbsc_** quoin family in RBS0-SpecTop.adoc and retire superseded terms.

## New quoins to mint
- `rbsc_crucible` — the tandem container assembly (sentry + pentacle + bottle)
- `rbsc_pentacle` — namespace/routing container (replaces `at_censer_container`)
- `rbsc_charge` — lifecycle: stand up crucible
- `rbsc_quench` — lifecycle: tear down crucible
- `rbsc_enjoin` — operational: dispatch ephemeral bottle (agile)
- `rbsc_hail` — diagnostic: sentry
- `rbsc_rack` — diagnostic: bottle
- `rbsc_scry` — diagnostic: networks
- `rbsc_agile` — service mode: ephemeral dispatch pattern
- `rbsc_sessile` — service mode: persistent service pattern

## Terms to retire
- `opbs_bottle_start` → replaced by `rbsc_charge`
- `opbr_bottle_run` → replaced by `rbsc_enjoin`
- `opss_sentry_start` → evaluate: does sentry_start survive as a distinct operation, or is it subsumed by charge?

## Terms to migrate (display text only)
- `at_bottle_service` → `rbsc_crucible`
- `at_censer_container` → `rbsc_pentacle`
- `at_agile_service` → `rbsc_agile`
- `at_sessile_service` → `rbsc_sessile`

Note: Full `at_*` retirement is out of scope per paddock — only migrate the display text references that directly overlap with new rbsc_ terms.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary target for all minting and retirement
- Other spec files that reference the retired `op*_` terms (RBSBS, RBSBR, RBSBL, RBSBK, RBSCE, RBSNC, RBSSS, RBSSR, RBSNX)

## Approach
- Mint all rbsc_ terms in the mapping and definition sections of RBS0
- Update cross-references in operation spec files
- Retire op*_ anchor definitions
- Verify no dangling references remain

**[260331-0735] rough**

## Character
Spec-focused, requires MCM fluency. Minting new AsciiDoc linked terms and retiring old ones. Each quoin needs the three-part structure: attribute mapping, anchor definition, and definition text.

## Scope
Mint the **rbsc_** quoin family in RBS0-SpecTop.adoc and retire superseded terms.

## New quoins to mint
- `rbsc_crucible` — the tandem container assembly (sentry + pentacle + bottle)
- `rbsc_pentacle` — namespace/routing container (replaces `at_censer_container`)
- `rbsc_charge` — lifecycle: stand up crucible
- `rbsc_quench` — lifecycle: tear down crucible
- `rbsc_enjoin` — operational: dispatch ephemeral bottle (agile)
- `rbsc_hail` — diagnostic: sentry
- `rbsc_rack` — diagnostic: bottle
- `rbsc_scry` — diagnostic: networks
- `rbsc_agile` — service mode: ephemeral dispatch pattern
- `rbsc_sessile` — service mode: persistent service pattern

## Terms to retire
- `opbs_bottle_start` → replaced by `rbsc_charge`
- `opbr_bottle_run` → replaced by `rbsc_enjoin`
- `opss_sentry_start` → evaluate: does sentry_start survive as a distinct operation, or is it subsumed by charge?

## Terms to migrate (display text only)
- `at_bottle_service` → `rbsc_crucible`
- `at_censer_container` → `rbsc_pentacle`
- `at_agile_service` → `rbsc_agile`
- `at_sessile_service` → `rbsc_sessile`

Note: Full `at_*` retirement is out of scope per paddock — only migrate the display text references that directly overlap with new rbsc_ terms.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary target for all minting and retirement
- Other spec files that reference the retired `op*_` terms (RBSBS, RBSBR, RBSBL, RBSBK, RBSCE, RBSNC, RBSSS, RBSSR, RBSNX)

## Approach
- Mint all rbsc_ terms in the mapping and definition sections of RBS0
- Update cross-references in operation spec files
- Retire op*_ anchor definitions
- Verify no dangling references remain

**[260330-0802] rough**

## Character
Spec-focused, requires MCM fluency. Minting new AsciiDoc linked terms and retiring old ones. Each quoin needs the three-part structure: attribute mapping, anchor definition, and definition text.

## Scope
Mint the **rbsc_** quoin family in RBS0-SpecTop.adoc and retire superseded terms.

## New quoins to mint
- `rbsc_crucible` — the tandem container assembly (sentry + pentacle + bottle)
- `rbsc_pentacle` — namespace/routing container (replaces `at_censer_container`)
- `rbsc_charge` — lifecycle: stand up crucible
- `rbsc_quench` — lifecycle: tear down crucible
- `rbsc_enjoin` — operational: dispatch ephemeral bottle (agile)
- `rbsc_hail` — diagnostic: sentry
- `rbsc_rack` — diagnostic: bottle
- `rbsc_observe` — diagnostic: networks
- `rbsc_agile` — service mode: ephemeral dispatch pattern
- `rbsc_sessile` — service mode: persistent service pattern

## Terms to retire
- `opbs_bottle_start` → replaced by `rbsc_charge`
- `opbr_bottle_run` → replaced by `rbsc_enjoin`
- `opss_sentry_start` → evaluate: does sentry_start survive as a distinct operation, or is it subsumed by charge?

## Terms to migrate (display text only)
- `at_bottle_service` → `rbsc_crucible`
- `at_censer_container` → `rbsc_pentacle`
- `at_agile_service` → `rbsc_agile`
- `at_sessile_service` → `rbsc_sessile`

Note: Full `at_*` retirement is out of scope per paddock — only migrate the display text references that directly overlap with new rbsc_ terms.

## Files
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — primary target for all minting and retirement
- Other spec files that reference the retired `op*_` terms (RBSBS, RBSBR, RBSBL, RBSBK, RBSCE, RBSNC, RBSSS, RBSSR, RBSNX)

## Approach
- Mint all rbsc_ terms in the mapping and definition sections of RBS0
- Update cross-references in operation spec files
- Retire op*_ anchor definitions
- Verify no dangling references remain

### governor-verbs-knight-forfeit (₢AxAAM) [complete]

**[260331-1129] complete**

## Character
Two parallel renames in the Governor domain. Same vertical shape as the consecration verb paces. The Governor module (rbgg_cli.sh, rbgg_ functions) is self-contained — these renames don't interact with other modules.

## Scope
Replace **create → knight** and **delete → forfeit** for Governor role identity operations.

## Shell layer
- `Tools/rbk/rbgg_cli.sh`: rename `rbgg_create_director` → `rbgg_knight_director`, `rbgg_create_retriever` → `rbgg_knight_retriever`
- `Tools/rbk/rbgg_cli.sh`: rename `rbgg_delete_service_account` → `rbgg_forfeit_service_account`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_RETRIEVER "rbw-GR"` → `RBZ_KNIGHT_RETRIEVER "rbw-GK"` (K for knight)
  - `RBZ_CREATE_DIRECTOR "rbw-GD"` → `RBZ_KNIGHT_DIRECTOR "rbw-GN"` (N to avoid K/D collision? — evaluate colophon letters)
  - `RBZ_DELETE_SERVICE_ACCOUNT "rbw-GS"` → `RBZ_FORFEIT_SERVICE_ACCOUNT "rbw-GF"` (F for forfeit)

## Tabtarget layer
- Delete old `tt/rbw-G*.Governor*` tabtargets for create/delete operations
- Create new tabtargets with updated colophons and frontispieces

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_director_create` → `rbtgo_director_knight`
  - `rbtgo_retriever_create` → `rbtgo_retriever_knight`
  - `rbtgo_sa_delete` → `rbtgo_sa_forfeit`
- Update section headings and body text
- Spec files: RBSDI-director_create.adoc, RBSRC-retriever_create.adoc, RBSSD-sa_delete.adoc — update internal references (evaluate file renames)

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update Governor operation rows
- `Tools/rbk/vov_veiled/README.consumer.md`: update all references

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `create_director`, `create_retriever`, `delete_service_account`, `rbw-GR`, `rbw-GD`, `rbw-GS` stragglers

**[260330-1611] rough**

## Character
Two parallel renames in the Governor domain. Same vertical shape as the consecration verb paces. The Governor module (rbgg_cli.sh, rbgg_ functions) is self-contained — these renames don't interact with other modules.

## Scope
Replace **create → knight** and **delete → forfeit** for Governor role identity operations.

## Shell layer
- `Tools/rbk/rbgg_cli.sh`: rename `rbgg_create_director` → `rbgg_knight_director`, `rbgg_create_retriever` → `rbgg_knight_retriever`
- `Tools/rbk/rbgg_cli.sh`: rename `rbgg_delete_service_account` → `rbgg_forfeit_service_account`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_RETRIEVER "rbw-GR"` → `RBZ_KNIGHT_RETRIEVER "rbw-GK"` (K for knight)
  - `RBZ_CREATE_DIRECTOR "rbw-GD"` → `RBZ_KNIGHT_DIRECTOR "rbw-GN"` (N to avoid K/D collision? — evaluate colophon letters)
  - `RBZ_DELETE_SERVICE_ACCOUNT "rbw-GS"` → `RBZ_FORFEIT_SERVICE_ACCOUNT "rbw-GF"` (F for forfeit)

## Tabtarget layer
- Delete old `tt/rbw-G*.Governor*` tabtargets for create/delete operations
- Create new tabtargets with updated colophons and frontispieces

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_director_create` → `rbtgo_director_knight`
  - `rbtgo_retriever_create` → `rbtgo_retriever_knight`
  - `rbtgo_sa_delete` → `rbtgo_sa_forfeit`
- Update section headings and body text
- Spec files: RBSDI-director_create.adoc, RBSRC-retriever_create.adoc, RBSSD-sa_delete.adoc — update internal references (evaluate file renames)

## Consumer docs
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`: update Governor operation rows
- `Tools/rbk/vov_veiled/README.consumer.md`: update all references

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `create_director`, `create_retriever`, `delete_service_account`, `rbw-GR`, `rbw-GD`, `rbw-GS` stragglers

### payor-verbs-levy-unmake-mantle (₢AxAAN) [complete]

**[260331-1144] complete**

## Character
Three renames in the Payor domain. Levy/unmake are a natural pair (depot create/destroy); mantle is the governor reset (credential rotation). Two modules: rbgp_cli.sh (depot + governor reset) and rbgm_cli.sh (if any manual procedures reference these).

## Scope
Replace **create → levy**, **destroy → unmake**, and **reset → mantle** for Payor operations.

## Shell layer
- `Tools/rbk/rbgp_cli.sh`: rename `rbgp_depot_create` → `rbgp_depot_levy`, `rbgp_depot_destroy` → `rbgp_depot_unmake`, `rbgp_governor_reset` → `rbgp_governor_mantle`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_DEPOT "rbw-PC"` → `RBZ_LEVY_DEPOT "rbw-PL"` (L for levy)
  - `RBZ_DESTROY_DEPOT "rbw-PD"` → `RBZ_UNMAKE_DEPOT "rbw-PU"` (U for unmake)
  - `RBZ_GOVERNOR_RESET "rbw-PG"` → `RBZ_MANTLE_GOVERNOR "rbw-PM"` (M for mantle)

## Tabtarget layer
- Delete old `tt/rbw-P*.Payor*` tabtargets for create/destroy/reset
- Create new tabtargets with updated colophons and frontispieces

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_depot_create` → `rbtgo_depot_levy`
  - `rbtgo_depot_destroy` → `rbtgo_depot_unmake`
  - `rbtgo_governor_reset` → `rbtgo_governor_mantle`
- Spec files: RBSDC-depot_create.adoc, RBSDD-depot_destroy.adoc, RBSGR-governor_reset.adoc — update internal references

## Consumer docs
- Update Payor operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `depot_create`, `depot_destroy`, `governor_reset`, `rbw-PC`, `rbw-PD`, `rbw-PG` stragglers

**[260330-1611] rough**

## Character
Three renames in the Payor domain. Levy/unmake are a natural pair (depot create/destroy); mantle is the governor reset (credential rotation). Two modules: rbgp_cli.sh (depot + governor reset) and rbgm_cli.sh (if any manual procedures reference these).

## Scope
Replace **create → levy**, **destroy → unmake**, and **reset → mantle** for Payor operations.

## Shell layer
- `Tools/rbk/rbgp_cli.sh`: rename `rbgp_depot_create` → `rbgp_depot_levy`, `rbgp_depot_destroy` → `rbgp_depot_unmake`, `rbgp_governor_reset` → `rbgp_governor_mantle`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_CREATE_DEPOT "rbw-PC"` → `RBZ_LEVY_DEPOT "rbw-PL"` (L for levy)
  - `RBZ_DESTROY_DEPOT "rbw-PD"` → `RBZ_UNMAKE_DEPOT "rbw-PU"` (U for unmake)
  - `RBZ_GOVERNOR_RESET "rbw-PG"` → `RBZ_MANTLE_GOVERNOR "rbw-PM"` (M for mantle)

## Tabtarget layer
- Delete old `tt/rbw-P*.Payor*` tabtargets for create/destroy/reset
- Create new tabtargets with updated colophons and frontispieces

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_depot_create` → `rbtgo_depot_levy`
  - `rbtgo_depot_destroy` → `rbtgo_depot_unmake`
  - `rbtgo_governor_reset` → `rbtgo_governor_mantle`
- Spec files: RBSDC-depot_create.adoc, RBSDD-depot_destroy.adoc, RBSGR-governor_reset.adoc — update internal references

## Consumer docs
- Update Payor operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `depot_create`, `depot_destroy`, `governor_reset`, `rbw-PC`, `rbw-PD`, `rbw-PG` stragglers

### marshal-verbs-zero-proof (₢AxAAO) [complete]

**[260331-1404] complete**

## Character
Two renames in the Marshal domain. Self-contained module (rblm_cli.sh). Zero/proof are a natural pair: prepare (zero the regime) then test (proof the release).

## Scope
Replace **reset → zero** and **duplicate → proof** for Marshal regime lifecycle operations.

## Shell layer
- `Tools/rbk/rblm_cli.sh`: rename `rblm_reset` → `rblm_zero`, `rblm_duplicate` → `rblm_proof`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_MARSHAL_RESET "rbw-MR"` → `RBZ_MARSHAL_ZERO "rbw-MZ"` (Z for zero)
  - `RBZ_MARSHAL_DUPLICATE "rbw-MD"` → `RBZ_MARSHAL_PROOF "rbw-MP"` (P for proof)

## Tabtarget layer
- Delete: `tt/rbw-MR.MarshalReset*.sh`, `tt/rbw-MD.MarshalDuplicate*.sh` (or current frontispieces)
- Create: `tt/rbw-MZ.MarshalZeroes*.sh`, `tt/rbw-MP.MarshalProofs*.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update any quoin mappings for marshal operations
- Any spec files referencing marshal operations

## Consumer docs
- Update Marshal operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `rblm_reset`, `rblm_duplicate`, `rbw-MR`, `rbw-MD` stragglers

**[260330-1611] rough**

## Character
Two renames in the Marshal domain. Self-contained module (rblm_cli.sh). Zero/proof are a natural pair: prepare (zero the regime) then test (proof the release).

## Scope
Replace **reset → zero** and **duplicate → proof** for Marshal regime lifecycle operations.

## Shell layer
- `Tools/rbk/rblm_cli.sh`: rename `rblm_reset` → `rblm_zero`, `rblm_duplicate` → `rblm_proof`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_MARSHAL_RESET "rbw-MR"` → `RBZ_MARSHAL_ZERO "rbw-MZ"` (Z for zero)
  - `RBZ_MARSHAL_DUPLICATE "rbw-MD"` → `RBZ_MARSHAL_PROOF "rbw-MP"` (P for proof)

## Tabtarget layer
- Delete: `tt/rbw-MR.MarshalReset*.sh`, `tt/rbw-MD.MarshalDuplicate*.sh` (or current frontispieces)
- Create: `tt/rbw-MZ.MarshalZeroes*.sh`, `tt/rbw-MP.MarshalProofs*.sh`

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update any quoin mappings for marshal operations
- Any spec files referencing marshal operations

## Consumer docs
- Update Marshal operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `rblm_reset`, `rblm_duplicate`, `rbw-MR`, `rbw-MD` stragglers

### image-verbs-wrest-jettison-plumb (₢AxAAP) [complete]

**[260331-1419] complete**

## Character
Three renames for image-level artifact operations. All live in the Foundry module (rbf_Foundry.sh / rbf_cli.sh) and share the Retriever/Director colophon families. These parallel the consecration-level verbs (summon/abjure/inspect) but at the individual artifact level.

## Scope
Replace **retrieve → wrest**, **delete → jettison**, and **inspect → plumb** for image-level operations.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_retrieve` → `rbf_wrest`, `rbf_delete` → `rbf_jettison`, `rbf_inspect_full` → `rbf_plumb_full`, `rbf_inspect_compact` → `rbf_plumb_compact`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_RETRIEVE_IMAGE "rbw-Rr"` → `RBZ_WREST_IMAGE "rbw-Rw"` (w for wrest)
  - `RBZ_DELETE_IMAGE "rbw-DD"` → `RBZ_JETTISON_IMAGE "rbw-DJ"` (J for jettison)
  - `RBZ_INSPECT_FULL "rbw-RiF"` → `RBZ_PLUMB_FULL "rbw-RpF"` (p for plumb)
  - `RBZ_INSPECT_COMPACT "rbw-Ric"` → `RBZ_PLUMB_COMPACT "rbw-Rpc"` (p for plumb)

## Tabtarget layer
- Delete old tabtargets for retrieve/delete/inspect
- Create new tabtargets with updated colophons and frontispieces
- Note: inspect has two modes (full/compact) with separate colophons — both rename

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_image_retrieve` or equivalent → `rbtgo_image_wrest`
  - `rbtgo_image_delete` or equivalent → `rbtgo_image_jettison`
  - `rbtgo_ark_inspect` → `rbtgo_image_plumb` (or equivalent — evaluate naming)
  - Update `rbtc_inspect_full` / `rbtc_inspect_compact` colophon quoins
- Spec files: RBSIR-image_retrieve.adoc, RBSID-image_delete.adoc, RBSAI-ark_inspect.adoc — update internal references

## Consumer docs
- Update Retriever and Director image operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `rbf_retrieve`, `rbf_delete`, `rbf_inspect`, `rbw-Rr`, `rbw-DD`, `rbw-RiF`, `rbw-Ric` stragglers

**[260330-1611] rough**

## Character
Three renames for image-level artifact operations. All live in the Foundry module (rbf_Foundry.sh / rbf_cli.sh) and share the Retriever/Director colophon families. These parallel the consecration-level verbs (summon/abjure/inspect) but at the individual artifact level.

## Scope
Replace **retrieve → wrest**, **delete → jettison**, and **inspect → plumb** for image-level operations.

## Shell layer
- `Tools/rbk/rbf_Foundry.sh`: rename `rbf_retrieve` → `rbf_wrest`, `rbf_delete` → `rbf_jettison`, `rbf_inspect_full` → `rbf_plumb_full`, `rbf_inspect_compact` → `rbf_plumb_compact`
- Update `buc_doc_brief` strings and internal comments

## Dispatch layer
- `Tools/rbk/rbz_zipper.sh`: update buz_enroll
  - `RBZ_RETRIEVE_IMAGE "rbw-Rr"` → `RBZ_WREST_IMAGE "rbw-Rw"` (w for wrest)
  - `RBZ_DELETE_IMAGE "rbw-DD"` → `RBZ_JETTISON_IMAGE "rbw-DJ"` (J for jettison)
  - `RBZ_INSPECT_FULL "rbw-RiF"` → `RBZ_PLUMB_FULL "rbw-RpF"` (p for plumb)
  - `RBZ_INSPECT_COMPACT "rbw-Ric"` → `RBZ_PLUMB_COMPACT "rbw-Rpc"` (p for plumb)

## Tabtarget layer
- Delete old tabtargets for retrieve/delete/inspect
- Create new tabtargets with updated colophons and frontispieces
- Note: inspect has two modes (full/compact) with separate colophons — both rename

## Spec layer
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: update quoin mappings and anchors
  - `rbtgo_image_retrieve` or equivalent → `rbtgo_image_wrest`
  - `rbtgo_image_delete` or equivalent → `rbtgo_image_jettison`
  - `rbtgo_ark_inspect` → `rbtgo_image_plumb` (or equivalent — evaluate naming)
  - Update `rbtc_inspect_full` / `rbtc_inspect_compact` colophon quoins
- Spec files: RBSIR-image_retrieve.adoc, RBSID-image_delete.adoc, RBSAI-ark_inspect.adoc — update internal references

## Consumer docs
- Update Retriever and Director image operation rows in CLAUDE.consumer.md and README.consumer.md

## Approach
- Shell + zipper first
- Tabtargets second
- Spec + consumer docs last
- Grep for remaining `rbf_retrieve`, `rbf_delete`, `rbf_inspect`, `rbw-Rr`, `rbw-DD`, `rbw-RiF`, `rbw-Ric` stragglers

### documentation-sweep (₢AxAAJ) [complete]

**[260331-1432] complete**

## Character
Verification pass. Low effort, high confidence. Confirms all prior paces achieved complete coverage.

## Scope
Comprehensive grep for any surviving old-vocabulary references. No primary renames — if anything is found, it's a straggler that should have been caught by an earlier pace.

## Search terms

Crucible/pentacle renames:
censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s., rbw-z., rbw-B., rbw-C., rbw-S., rbw-o., opbs_, opbr_, opss_

Consecration operation renames:
DirectorChecksConsecrations, DirectorCreatesConsecration, rbf_check_consecrations, rbf_create, rbw-Dc., rbw-DC., rbtgo_consecration_check, RBZ_CHECK_CONSECRATIONS, RBZ_CREATE_CONSECRATION

Governor verb renames:
rbgg_create_director, rbgg_create_retriever, rbgg_delete_service_account, RBZ_CREATE_RETRIEVER, RBZ_CREATE_DIRECTOR, RBZ_DELETE_SERVICE_ACCOUNT, rbw-GR., rbw-GD., rbw-GS.

Payor verb renames:
rbgp_depot_create, rbgp_depot_destroy, rbgp_governor_reset, RBZ_CREATE_DEPOT, RBZ_DESTROY_DEPOT, RBZ_GOVERNOR_RESET, rbw-PC., rbw-PD., rbw-PG.

Marshal verb renames:
rblm_reset, rblm_duplicate, RBZ_MARSHAL_RESET, RBZ_MARSHAL_DUPLICATE, rbw-MR., rbw-MD.

Image verb renames:
rbf_retrieve, rbf_delete, rbf_inspect, RBZ_RETRIEVE_IMAGE, RBZ_DELETE_IMAGE, RBZ_INSPECT_FULL, RBZ_INSPECT_COMPACT, rbw-Rr., rbw-DD., rbw-RiF., rbw-Ric.

## Disposition of findings
- **Active code/spec/docs**: fix immediately (straggler from earlier pace)
- **Retired heat memos**: evaluate case-by-case — update if the old term is used as a current reference; leave if it's genuinely historical narrative
- **gallops.json / settings.local.json**: update if stale references cause confusion
- If zero findings: the heat is clean. Celebrate briefly.

**[260330-1612] rough**

## Character
Verification pass. Low effort, high confidence. Confirms all prior paces achieved complete coverage.

## Scope
Comprehensive grep for any surviving old-vocabulary references. No primary renames — if anything is found, it's a straggler that should have been caught by an earlier pace.

## Search terms

Crucible/pentacle renames:
censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s., rbw-z., rbw-B., rbw-C., rbw-S., rbw-o., opbs_, opbr_, opss_

Consecration operation renames:
DirectorChecksConsecrations, DirectorCreatesConsecration, rbf_check_consecrations, rbf_create, rbw-Dc., rbw-DC., rbtgo_consecration_check, RBZ_CHECK_CONSECRATIONS, RBZ_CREATE_CONSECRATION

Governor verb renames:
rbgg_create_director, rbgg_create_retriever, rbgg_delete_service_account, RBZ_CREATE_RETRIEVER, RBZ_CREATE_DIRECTOR, RBZ_DELETE_SERVICE_ACCOUNT, rbw-GR., rbw-GD., rbw-GS.

Payor verb renames:
rbgp_depot_create, rbgp_depot_destroy, rbgp_governor_reset, RBZ_CREATE_DEPOT, RBZ_DESTROY_DEPOT, RBZ_GOVERNOR_RESET, rbw-PC., rbw-PD., rbw-PG.

Marshal verb renames:
rblm_reset, rblm_duplicate, RBZ_MARSHAL_RESET, RBZ_MARSHAL_DUPLICATE, rbw-MR., rbw-MD.

Image verb renames:
rbf_retrieve, rbf_delete, rbf_inspect, RBZ_RETRIEVE_IMAGE, RBZ_DELETE_IMAGE, RBZ_INSPECT_FULL, RBZ_INSPECT_COMPACT, rbw-Rr., rbw-DD., rbw-RiF., rbw-Ric.

## Disposition of findings
- **Active code/spec/docs**: fix immediately (straggler from earlier pace)
- **Retired heat memos**: evaluate case-by-case — update if the old term is used as a current reference; leave if it's genuinely historical narrative
- **gallops.json / settings.local.json**: update if stale references cause confusion
- If zero findings: the heat is clean. Celebrate briefly.

**[260330-1424] rough**

## Character
Verification pass. Low effort, high confidence. Confirms all prior paces achieved complete coverage.

## Scope
Comprehensive grep for any surviving old-vocabulary references. No primary renames — if anything is found, it's a straggler that should have been caught by an earlier pace.

## Search terms
censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s., rbw-z., rbw-B., rbw-C., rbw-S., rbw-o., opbs_, opbr_, opss_, DirectorChecksConsecrations, DirectorCreatesConsecration, rbf_check_consecrations, rbf_create, rbw-Dc., rbw-DC., rbtgo_consecration_check, RBZ_CHECK_CONSECRATIONS, RBZ_CREATE_CONSECRATION

## Disposition of findings
- **Active code/spec/docs**: fix immediately (straggler from earlier pace)
- **Retired heat memos**: evaluate case-by-case — update if the old term is used as a current reference; leave if it's genuinely historical narrative ("we renamed censer to pentacle")
- **gallops.json / settings.local.json**: update if stale references cause confusion
- If zero findings: the heat is clean. Celebrate briefly.

**[260330-0807] rough**

## Character
Verification pass. Low effort, high confidence. Confirms all prior paces achieved complete coverage.

## Scope
Comprehensive grep for any surviving old-vocabulary references. No primary renames — if anything is found, it's a straggler that should have been caught by an earlier pace.

## Search terms
censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s., rbw-z., rbw-B., rbw-C., rbw-S., rbw-o., opbs_, opbr_, opss_

## Disposition of findings
- **Active code/spec/docs**: fix immediately (straggler from earlier pace)
- **Retired heat memos**: evaluate case-by-case — update if the old term is used as a current reference; leave if it's genuinely historical narrative ("we renamed censer to pentacle")
- **gallops.json / settings.local.json**: update if stale references cause confusion
- If zero findings: the heat is clean. Celebrate briefly.

**[260330-0806] rough**

## Character
Janitorial. Low risk, wide surface. Updating references in documentation, examples, context files, diagrams, and historical records to use the new vocabulary consistently.

## Scope
Sweep all remaining files that still reference old vocabulary after the prior paces complete. This catches anything the vertical slices didn't touch.

## File categories

**Public-facing assets:**
- `index.html` — 3 "Bottle Service", 3 "Censer", 2 "bottle_service"
- `rbm-abstract-drawio.svg` — 3 "Bottle Service" (draw.io SVG, edit directly)

**BUK documentation (examples using old tabtarget names):**
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — tabtarget examples
- `Tools/buk/README.md` — BUK context examples
- `Tools/buk/buk-claude-context.md` — Claude context examples

**Project context:**
- `CLAUDE.md` — file acronym mappings, tabtarget examples
- `.claude/settings.local.json` — permission patterns referencing old colophons

**Consumer-facing docs:**
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`
- `Tools/rbk/vov_veiled/README.consumer.md`

**Historical (retired heat memos):**
- `.claude/jjm/retired/` — multiple retired heats reference old vocabulary
- Decision: update or leave as historical record? (Discuss during execution)

**Work tracking:**
- `.claude/jjm/jjg_gallops.json` — should update organically via jjx operations
- Paddock content for ₣Ax itself — already uses new vocabulary

## Approach
- Grep for all remaining instances of: censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s, rbw-z, rbw-B, rbw-C, rbw-S, rbw-o, opbs_, opbr_, opss_
- Update each occurrence in context
- For retired memos: lean toward updating for searchability, but flag any where the old term is part of the historical narrative

**[260330-0802] rough**

## Character
Janitorial. Low risk, wide surface. Updating references in documentation, examples, context files, and historical records to use the new vocabulary consistently.

## Scope
Sweep all remaining files that still reference old vocabulary after the prior paces complete. This catches anything the vertical slices didn't touch.

## File categories

**BUK documentation (examples using old tabtarget names):**
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — tabtarget examples
- `Tools/buk/README.md` — BUK context examples
- `Tools/buk/buk-claude-context.md` — Claude context examples

**Project context:**
- `CLAUDE.md` — file acronym mappings, tabtarget examples
- `.claude/settings.local.json` — permission patterns referencing old colophons

**Consumer-facing docs:**
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md`
- `Tools/rbk/vov_veiled/README.consumer.md`

**Historical (retired heat memos):**
- `.claude/jjm/retired/` — multiple retired heats reference old vocabulary
- Decision: update or leave as historical record? (Discuss during execution)

**Work tracking:**
- `.claude/jjm/jjg_gallops.json` — should update organically via jjx operations
- Paddock content for ₣Ax itself — already uses new vocabulary

## Approach
- Grep for all remaining instances of: censer, bottle_start, bottle_run, bottle_service, ConnectBottle, ConnectCenser, ConnectSentry, ObserveNetworks, rbw-s, rbw-z, rbw-B, rbw-C, rbw-S, rbw-o, opbs_, opbr_, opss_
- Update each occurrence in context
- For retired memos: lean toward updating for searchability, but flag any where the old term is part of the historical narrative

### queued-build-advisory-and-quota-guide-repair (₢AxAAS) [complete]

**[260331-1505] complete**

## Character
Interactive — requires user screenshots and hands-on depot testing to validate the quota lifecycle sequence. Not mechanical; the procedure must match real Console behavior at each depot age.

## Context
Private pool builds sit in QUEUED for 7-13 polls (~40-70s) during normal cold-start scale-from-zero. When two builds compete for the 2 vCPU quota, the second serializes (observed: 130-288 polls, 10-26 min). Historical log analysis in `_logs_buk/hist-rbw-DC-sh-*` confirms the pattern across 68 builds from 2026-03-03 to 2026-03-14.

## Deliverable 1: Polling advisory in zrbf_wait_build_completion
In `Tools/rbk/rbf_Foundry.sh` (the QUEUED/WORKING while loop), add a one-time advisory when QUEUED persists past ~20 polls (~110s). Use `buc_warn` + `buc_tabtarget "${RBZ_QUOTA_BUILD}"`. Fire once (guard variable). Message should convey: "Build queued longer than normal — another build may be holding the private pool."

## Deliverable 2: Quota guide lifecycle repair
The guide in `rbgm_ManualProcedures.sh` (rbgm_quota_build function) and spec `RBSQB-quota_build.adoc` were overcorrected in commit 13fd330f to say Console Edit Quotas "does NOT work" for this metric. The real lifecycle is:
1. **Fresh depot**: 2 vCPU limit, Console Edit Quotas panel not yet available
2. **Established depot** (after some build activity): Console Edit Quotas becomes available, values above 2 require provider approval

Key discovery: certain quotas do not become adjustable until after the depot has been used a while. The revision history before 13fd330f likely has a more accurate description of the Console flow — use it as reference for what to restore, then layer on the fresh-vs-established distinction.

This deliverable is interactive: user will provide screenshots of the Console at different depot lifecycle stages to inform the procedure.

## Threshold rationale
20 polls x ~5.5s/poll = 110s. Normal cold-start ceiling is ~13 polls (~70s). 20 polls provides clear separation from cold-start noise while catching contention early enough to be useful.

**[260331-1427] rough**

## Character
Interactive — requires user screenshots and hands-on depot testing to validate the quota lifecycle sequence. Not mechanical; the procedure must match real Console behavior at each depot age.

## Context
Private pool builds sit in QUEUED for 7-13 polls (~40-70s) during normal cold-start scale-from-zero. When two builds compete for the 2 vCPU quota, the second serializes (observed: 130-288 polls, 10-26 min). Historical log analysis in `_logs_buk/hist-rbw-DC-sh-*` confirms the pattern across 68 builds from 2026-03-03 to 2026-03-14.

## Deliverable 1: Polling advisory in zrbf_wait_build_completion
In `Tools/rbk/rbf_Foundry.sh` (the QUEUED/WORKING while loop), add a one-time advisory when QUEUED persists past ~20 polls (~110s). Use `buc_warn` + `buc_tabtarget "${RBZ_QUOTA_BUILD}"`. Fire once (guard variable). Message should convey: "Build queued longer than normal — another build may be holding the private pool."

## Deliverable 2: Quota guide lifecycle repair
The guide in `rbgm_ManualProcedures.sh` (rbgm_quota_build function) and spec `RBSQB-quota_build.adoc` were overcorrected in commit 13fd330f to say Console Edit Quotas "does NOT work" for this metric. The real lifecycle is:
1. **Fresh depot**: 2 vCPU limit, Console Edit Quotas panel not yet available
2. **Established depot** (after some build activity): Console Edit Quotas becomes available, values above 2 require provider approval

Key discovery: certain quotas do not become adjustable until after the depot has been used a while. The revision history before 13fd330f likely has a more accurate description of the Console flow — use it as reference for what to restore, then layer on the fresh-vs-established distinction.

This deliverable is interactive: user will provide screenshots of the Console at different depot lifecycle stages to inform the procedure.

## Threshold rationale
20 polls x ~5.5s/poll = 110s. Normal cold-start ceiling is ~13 polls (~70s). 20 polls provides clear separation from cold-start noise while catching contention early enough to be useful.

**[260331-1427] rough**

Drafted from ₢AUAAf in ₣AU.

## Character
Mechanical with one judgment call (poll threshold).

## Context
Private pool builds sit in QUEUED for 7-13 polls (~40-70s) during normal cold-start scale-from-zero. When two builds compete for the 2 vCPU quota, the second serializes (observed: 130-288 polls, 10-26 min). Historical log analysis in `_logs_buk/hist-rbw-DC-sh-*` confirms the pattern across 68 builds from 2026-03-03 to 2026-03-14.

## Deliverable 1: Polling advisory in zrbf_wait_build_completion
In `Tools/rbk/rbf_Foundry.sh` line ~607 (the QUEUED/WORKING while loop), add a one-time advisory when QUEUED persists past ~20 polls (~110s). Use `buc_warn` + `buc_tabtarget "${RBZ_QUOTA_BUILD}"`. Fire once (guard variable). Message should convey: "Build queued longer than normal — another build may be holding the private pool."

## Deliverable 2: Quota guide lifecycle repair
The guide in `rbgm_ManualProcedures.sh` (rbgm_quota_build function) and spec `RBSQB-quota_build.adoc` were overcorrected in commit 13fd330f to say Console Edit Quotas "does NOT work" for this metric. Screenshot evidence shows it DOES work on established depots — the edit panel appears with "A value above 2 will require approval from your service provider." The lifecycle is:
1. Fresh depot: 2 vCPU limit, Console Edit Quotas not available
2. Established depot (after some build activity): Console Edit Quotas becomes available, values above 2 require provider approval

Repair the guide to document both paths: Console flow (when available) and support ticket (when Console flow isn't available yet). Git history before 13fd330f has the earlier version that described Console Edit Quotas as working — use it as reference for what to restore, then layer on the fresh-vs-established distinction.

## Threshold rationale
20 polls × ~5.5s/poll ≈ 110s. Normal cold-start ceiling is ~13 polls (~70s). 20 polls provides clear separation from cold-start noise while catching contention early enough to be useful.

**[260314-1756] rough**

## Character
Mechanical with one judgment call (poll threshold).

## Context
Private pool builds sit in QUEUED for 7-13 polls (~40-70s) during normal cold-start scale-from-zero. When two builds compete for the 2 vCPU quota, the second serializes (observed: 130-288 polls, 10-26 min). Historical log analysis in `_logs_buk/hist-rbw-DC-sh-*` confirms the pattern across 68 builds from 2026-03-03 to 2026-03-14.

## Deliverable 1: Polling advisory in zrbf_wait_build_completion
In `Tools/rbk/rbf_Foundry.sh` line ~607 (the QUEUED/WORKING while loop), add a one-time advisory when QUEUED persists past ~20 polls (~110s). Use `buc_warn` + `buc_tabtarget "${RBZ_QUOTA_BUILD}"`. Fire once (guard variable). Message should convey: "Build queued longer than normal — another build may be holding the private pool."

## Deliverable 2: Quota guide lifecycle repair
The guide in `rbgm_ManualProcedures.sh` (rbgm_quota_build function) and spec `RBSQB-quota_build.adoc` were overcorrected in commit 13fd330f to say Console Edit Quotas "does NOT work" for this metric. Screenshot evidence shows it DOES work on established depots — the edit panel appears with "A value above 2 will require approval from your service provider." The lifecycle is:
1. Fresh depot: 2 vCPU limit, Console Edit Quotas not available
2. Established depot (after some build activity): Console Edit Quotas becomes available, values above 2 require provider approval

Repair the guide to document both paths: Console flow (when available) and support ticket (when Console flow isn't available yet). Git history before 13fd330f has the earlier version that described Console Edit Quotas as working — use it as reference for what to restore, then layer on the fresh-vs-established distinction.

## Threshold rationale
20 polls × ~5.5s/poll ≈ 110s. Normal cold-start ceiling is ~13 polls (~70s). 20 polls provides clear separation from cold-start noise while catching contention early enough to be useful.

### verb-rename-integration-test (₢AxAAR) [complete]

**[260331-1718] complete**

## Character

End-to-end verification that all verb renames across ₣Ax land correctly. Not mechanical — requires judgment about what constitutes adequate coverage given the breadth of changes (crucible lifecycle, diagnostics, consecration ops, role authority, image-level ops, marshal ops).

## Scope

Exercise every renamed tabtarget and verify dispatch reaches the correct function. Specific coverage:

- **Crucible**: charge/quench/enjoin/hail/rack/scry colophons dispatch correctly
- **Consecration**: tally/ordain colophons dispatch correctly
- **Governor**: knight/forfeit/charter colophons dispatch correctly
- **Payor**: levy/unmake/mantle colophons dispatch correctly
- **Marshal**: zero/proof colophons dispatch correctly
- **Image**: wrest/jettison/plumb colophons dispatch correctly
- **Qualification**: rbw-Qf fast qualification passes (tabtarget/colophon health)

## Approach

- Start with `rbw-Qf` fast qualification — catches any broken tabtarget/colophon/nameplate wiring
- Then exercise `buc_doc_brief` on each renamed command to verify dispatch without side effects
- Report any stragglers found during testing

**[260331-1422] rough**

## Character

End-to-end verification that all verb renames across ₣Ax land correctly. Not mechanical — requires judgment about what constitutes adequate coverage given the breadth of changes (crucible lifecycle, diagnostics, consecration ops, role authority, image-level ops, marshal ops).

## Scope

Exercise every renamed tabtarget and verify dispatch reaches the correct function. Specific coverage:

- **Crucible**: charge/quench/enjoin/hail/rack/scry colophons dispatch correctly
- **Consecration**: tally/ordain colophons dispatch correctly
- **Governor**: knight/forfeit/charter colophons dispatch correctly
- **Payor**: levy/unmake/mantle colophons dispatch correctly
- **Marshal**: zero/proof colophons dispatch correctly
- **Image**: wrest/jettison/plumb colophons dispatch correctly
- **Qualification**: rbw-Qf fast qualification passes (tabtarget/colophon health)

## Approach

- Start with `rbw-Qf` fast qualification — catches any broken tabtarget/colophon/nameplate wiring
- Then exercise `buc_doc_brief` on each renamed command to verify dispatch without side effects
- Report any stragglers found during testing

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 Q subdocument-acronym-rename
  2 K consecration-checks-to-tally
  3 L consecration-creates-to-ordain
  4 C mcm-tier-rename
  5 D censer-to-pentacle
  6 E bottle-service-to-crucible
  7 F lifecycle-verbs-charge-quench
  8 G operational-verb-enjoin
  9 H diagnostic-verbs-rack-hail-scry
  10 I quoin-minting-rbsc-terms
  11 M governor-verbs-knight-forfeit
  12 N payor-verbs-levy-unmake-mantle
  13 O marshal-verbs-zero-proof
  14 P image-verbs-wrest-jettison-plumb
  15 J documentation-sweep
  16 S queued-build-advisory-and-quota-guide-repair
  17 R verb-rename-integration-test

QKLCDEFGHIMNOPJSR
xxx·xxxx·xxx·x·x· RBS0-SpecTop.adoc
·xx·x·x·x·xxxx··· rbz_zipper.sh
·xx···x····x·xxxx rbf_Foundry.sh
·xx·x·x·x·xx·x··· CLAUDE.consumer.md, README.consumer.md
····xxx·x········ rbob_bottle.sh
··x···x····x···x· rbgm_ManualProcedures.sh
··········xx·x··· RBSGS-GettingStarted.adoc
······x·x·····x·· README.md
····x··x·x······· RBSCN-crucible_enjoin.adoc
····x·x·········x rbtb_testbench.sh
····xx·······x··· index.html
····xx··x········ rbob_cli.sh
·xx··········x··· rbtctm_ThreeMode.sh
x···x·······x···· CLAUDE.md
············x··x· rblm_cli.sh
···········x···x· RBSQB-quota_build.adoc
··········xx····· RBSDK-director_knight.adoc, rbgg_Governor.sh
······x·x········ buk-claude-context.md
····x····x······· RBSBC-bottle_create.adoc, RBSCC-crucible_charge.adoc, RBSCE-command_exec.adoc
····xx··········· RBSCO-CosmologyIntro.adoc, RBSSS-sentry_start.adoc, rboo_observe.sh
···x·······x····· AXLA-Lexicon.adoc
··x·············x RBSAC-ark_conjure.adoc
··x···x·········· rbw_workbench.sh
·x···········x··· RBSCB-CloudBuildPosture.adoc
·x····x·········· rbcc_Constants.sh
················x pluml.rbrn.env, rbt_test_srjcl.py, rbtcsj_SrjclJupyter.sh, rbw-ts.TestSuite.crucible.sh, srjcl.rbrn.env, tadmor.rbrn.env
···············x· rbrr.env
·············x··· RBSAP-ark_plumb.adoc, RBSIJ-image_jettison.adoc, RBSIW-image_wrest.adoc
············x···· rbk-prep-release.md, rbw-MD.MarshalDuplicate.sh, rbw-MP.MarshalProofs.sh, rbw-MR.MarshalReset.sh, rbw-MZ.MarshalZeroes.sh
···········x····· RBSCIG-IamGrantContracts.adoc, RBSCTD-CloudBuildTriggerDispatch.adoc, RBSDE-depot_levy.adoc, RBSDN-depot_initialize.adoc, RBSDU-depot_unmake.adoc, RBSGD-gdc_establish.adoc, RBSGM-governor_mantle.adoc, RBSPI-payor_install.adoc, RBSRI-rubric_inscribe.adoc, RBSRR-RegimeRepo.adoc, rbgp_Payor.sh, rbgp_cli.sh
··········x······ RBSRK-retriever_knight.adoc, RBSSF-sa_forfeit.adoc
·········x······· RBSBK-bottle_cleanup.adoc
········x········ BUS0-BashUtilitiesSpec.adoc
······x·········· BCG-BashConsoleGuide.md, RBSAK-ark_kludge.adoc
·····x··········· RBRN-RegimeNameplate.adoc, RBSHR-HorizonRoadmap.adoc, bul_launcher.sh, rbm-abstract-drawio.svg, rbrn_regime.sh
····x············ Dockerfile, RBSBL-bottle_launch.adoc, RBSNC-network_create.adoc, rbjp_pentacle.sh, rbo.observe.sh, rbob_compose.yml, rbrv.env, rbtcns_TadmorSecurity.sh
···x············· MCM-MetaConceptModel.adoc
··x·············· RBSAG-ark_graft.adoc, rbw-DC.DirectorCreatesConsecration.sh, rbw-DO.DirectorOrdainsConsecration.sh
·x··············· RBSCL-consecration_tally.adoc, rbw-Dc.DirectorChecksConsecrations.sh, rbw-Dt.DirectorTalliesConsecrations.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 82 commits)

  1 E bottle-service-to-crucible
  2 F lifecycle-verbs-charge-quench
  3 G operational-verb-enjoin
  4 H diagnostic-verbs-rack-hail-scry
  5 I quoin-minting-rbsc-terms
  6 M governor-verbs-knight-forfeit
  7 N payor-verbs-levy-unmake-mantle
  8 O marshal-verbs-zero-proof
  9 P image-verbs-wrest-jettison-plumb
  10 J documentation-sweep
  11 S queued-build-advisory-and-quota-guide-repair
  12 R verb-rename-integration-test

123456789abcdefghijklmnopqrstuvwxyz
xx·································  E  2c
··xx·······························  F  2c
····xx·····························  G  2c
······xx···························  H  2c
········xx·························  I  2c
··········xx·······················  M  2c
············xx·····················  N  2c
··············xx···················  O  2c
················xx·················  P  2c
····················xx·············  J  2c
······················xxxx·········  S  4c
··························xxxxxxxxx  R  9c
```

## Steeplechase

### 2026-03-31 17:18 - ₢AxAAR - W

Integration testing of verb renames plus significant infrastructure improvements discovered along the way. Registry preflight added to rbf_build (reliquary canary + enshrine HEAD checks with educational text and copy-paste tabtargets). Mirror pool routing bug fixed (bind vessels hardcoded to airgap instead of honoring RBRV_EGRESS_MODE). Restored deleted srjcl WebSocket kernel test (collateral damage from AU RBM-tests cleanup). Created crucible test suite — runs all fixtures on existing consecrations without three-mode rebuild, scoped to retriever credentials only. Updated all three nameplate consecrations with fresh builds. First-ever cross-platform validation: full crucible suite (115 cases) passed on both arm64 (local Apple Silicon) and amd64 (cerebro Linux), proving the QEMU-emulated multi-platform build pipeline works end-to-end.

### 2026-03-31 17:14 - ₢AxAAR - n

Remove regime-credentials from crucible suite — station completeness audit needs all credentials (governor, director, retriever, payor), crucible only needs retriever

### 2026-03-31 17:05 - ₢AxAAR - n

Remove access-probe from crucible suite — it tests all 4 role credentials which is a service concern, not a crucible concern

### 2026-03-31 16:42 - ₢AxAAR - n

Add crucible test suite: runs all fixtures on existing consecrations without three-mode rebuild. Includes tadmor-security, srjcl-jupyter, pluml-diagram, regime-smoke, regime-credentials, enrollment-validation, regime-validation.

### 2026-03-31 16:39 - ₢AxAAR - n

Restore srjcl WebSocket kernel test deleted as collateral in ₣AU RBM-tests cleanup (30a8e69a). Relocated to Tools/rbk/rbts/, fixed path reference from RBTB_SCRIPT_DIR to RBTB_RBTS_DIR. All 3 srjcl cases now pass.

### 2026-03-31 16:07 - ₢AxAAR - n

Update all three nameplate consecrations with freshly built images from today's ordain runs: sentry, ifrit, jupyter, plantuml

### 2026-03-31 15:51 - ₢AxAAR - n

Fix mirror pool routing bug: zrbf_mirror_submit hardcoded RBDC_POOL_AIRGAP instead of honoring RBRV_EGRESS_MODE, causing bind vessels with tether egress (e.g., plantuml) to fail with i/o timeout on air-gapped pool

### 2026-03-31 15:42 - ₢AxAAR - n

Expand enshrine preflight into full registry preflight: add reliquary canary check (Layer 1) before enshrine check (Layer 2), educational buc_bare text explaining reliquary air-gap role and enshrine content-pinning, copy-paste tabtargets for both enshrine and ordain on failure

### 2026-03-31 15:22 - ₢AxAAR - n

Add enshrine preflight check to rbf_build: HEAD-checks anchored base images in GAR before expensive Cloud Build submission, fails fast with directive to run enshrine first. Updated RBSAC spec with new Verify Enshrined Base Images step.

### 2026-03-31 15:05 - ₢AxAAS - W

Overhauled quota guide with Console-verified UI steps (filter by concurrent_private, three-dot menu, Edit quota, request description, Submit), corrected metric display name to Concurrent Build CPUs (Private Pool), added 20-poll QUEUED advisory in zrbf_wait_build_completion, demoted quota preflight from fatal gate to advisory warning with fresh-depot guidance, restored RBRR_GCB_MIN_CONCURRENT_BUILDS default to 3, removed dead mode parameter and machine type tangent section

### 2026-03-31 15:05 - ₢AxAAS - n

Add QUEUED polling advisory at 20-poll threshold in zrbf_wait_build_completion (fires once per build wait), update live RBRR_GCB_MIN_CONCURRENT_BUILDS from 1 to 3

### 2026-03-31 15:03 - ₢AxAAS - n

Interactive quota guide overhaul: precise Console UI steps (filter by concurrent_private, three-dot menu, Edit quota, request description, Next, Submit), correct metric display name, remove machine type tangent, restore RBRR_GCB_MIN_CONCURRENT_BUILDS default to 3, demote quota preflight from fatal gate to advisory warning with fresh-depot guidance, strip dead mode parameter

### 2026-03-31 14:39 - ₢AxAAS - n

Restore pre-overcorrection quota guide: revert 13fd330f claims that Console Edit Quotas does NOT work and that metric is a non-adjustable system limit. Restore original metric name, quota language, and Edit Quotas as last-resort option. Preserve current tabtarget/function names from verb-colorization.

### 2026-03-31 14:32 - ₢AxAAJ - W

Comprehensive grep sweep for all old-vocabulary references across 7 rename categories. Found and fixed stragglers in 4 files: 5 private zrbf_inspect_* functions renamed to zrbf_plumb_* in rbf_Foundry.sh (missed in pace P), rbf_create→rbf_ordain in racing heat ₣At paddock, old tabtarget/function names in jji_itch.md, rbf_delete→rbf_jettison in JJK README. All retired heat memos, gallops.json, scars, and historical study/memo files left untouched as genuinely historical narrative.

### 2026-03-31 14:32 - ₢AxAAJ - n

Documentation sweep: renamed 5 private zrbf_inspect_* functions to zrbf_plumb_* in Foundry (straggler from pace P), updated display strings, fixed stale rbf_create→rbf_ordain in ₣At paddock, updated old tabtarget/function names in itch and JJK README

### 2026-03-31 14:27 - Heat - D

AUAAf → ₢AxAAS

### 2026-03-31 14:22 - Heat - S

verb-rename-integration-test

### 2026-03-31 14:19 - ₢AxAAP - W

Renamed image-level artifact operations to liturgical verbs: delete→jettison (rbw-DD→rbw-DJ), retrieve→wrest (rbw-Rr→rbw-Rw), inspect→plumb (rbw-RiF→rbw-RpF, rbw-Ric→rbw-Rpc). Shell functions, zipper constants+colophons, git mv 4 tabtargets, spec quoins (rbtgo_image_delete, rbtgo_image_retrieve, rbtgo_ark_inspect, rbtc_plumb_full, rbtc_plumb_compact), subdocs (RBSIJ, RBSIW, RBSAP, RBSGS, RBSCB), consumer docs, index.html, and test (rbtctm_ThreeMode) all updated. Straggler grep clean.

### 2026-03-31 14:19 - ₢AxAAP - n

Renamed Foundry operations to liturgical verbs: delete→jettison (rbf_delete→rbf_jettison, rbw-DD→rbw-DJ), retrieve→wrest (rbf_retrieve→rbf_wrest, rbw-Rr→rbw-Rw), inspect→plumb (rbf_inspect_full/compact→rbf_plumb_full/compact, rbw-RiF/Ric→rbw-RpF/Rpc). Shell functions, zipper constants+colophons, specs (RBS0, RBSAP, RBSIJ, RBSIW, RBSCB, RBSGS), consumer docs, test fixtures, and public index.html all updated.

### 2026-03-31 14:04 - ₢AxAAO - W

Renamed Marshal operations to liturgical verbs: reset→zero (rbw-MR→rbw-MZ), duplicate→proof (rbw-MD→rbw-MP). Shell functions, zipper constants+colophons, git mv 2 tabtargets, rbk-prep-release slash command, and CLAUDE.md all updated. Straggler grep clean.

### 2026-03-31 14:04 - ₢AxAAO - n

Renamed Marshal operations: reset→zero (rbw-MR→rbw-MZ), duplicate→proof (rbw-MD→rbw-MP). Shell functions, zipper constants+colophons, git mv 2 tabtargets, prep-release procedure, and CLAUDE.md RBLM description all updated.

### 2026-03-31 11:44 - ₢AxAAN - W

Renamed Payor domain operations: create_depot→levy_depot (rbw-PC→rbw-PL), destroy_depot→unmake_depot (rbw-PD→rbw-PU), governor_reset→governor_mantle (rbw-PG→rbw-PM). Shell functions, zipper constants+colophons, git mv 3 tabtargets, spec quoins (rbtgo_depot_levy, rbtgo_depot_unmake, rbtgo_governor_mantle), subdocs (RBSDE, RBSDU, RBSGM, RBSGS, RBSDN, RBSCIG, RBSRR, RBSRI, RBSQB, RBSPI, RBSCTD, RBSDK, RBSGD), lexicon, consumer docs all updated. Straggler grep clean.

### 2026-03-31 11:44 - ₢AxAAN - n

Rename Payor operations to liturgical verbs: depot_create→depot_levy, depot_destroy→depot_unmake, governor_reset→governor_mantle. Shell functions, zipper constants+colophons, spec quoins (rbtgo_depot_levy, rbtgo_depot_unmake, rbtgo_governor_mantle), subdocs (RBSDE, RBSDU, RBSGM, RBSGD, RBSDN, RBSGS, RBSPI, RBSQB, RBSRI, RBSRR, RBSDK, RBSCIG, RBSCTD), consumer docs, and lexicon examples all updated.

### 2026-03-31 11:29 - ₢AxAAM - W

Renamed Governor role identity operations: create_director→knight_director (rbw-GD→rbw-GK), create_retriever→charter_retriever (rbw-GR→rbw-GC), delete_service_account→forfeit_service_account (rbw-GS→rbw-GF). Knight confers authority to act (director); charter grants rights to access (retriever). Shell functions, zipper constants+colophons, git mv 3 tabtargets, spec quoins (rbtgo_director_knight, rbtgo_retriever_charter, rbtgo_sa_forfeit), subdocs (RBSDK, RBSRK, RBSSF, RBSGS), consumer docs all updated. Straggler grep clean.

### 2026-03-31 11:29 - ₢AxAAM - n

Rename Governor SA operations to liturgical verbs: charter retriever, knight director, forfeit service account

### 2026-03-31 11:07 - ₢AxAAI - W

Minted rbsc_ quoin family in RBS0: 10 new linked terms (rbsc_charge, rbsc_enjoin, rbsc_quench, rbsc_hail, rbsc_rack, rbsc_scry, rbsc_pentacle, rbsc_agile, rbsc_sessile + existing rbsc_crucible). Retired opbs_bottle_start and opbr_bottle_run via clean anchor swap (Option A). Added 7 definition blocks (pentacle, sessile, agile, quench, hail, rack, scry). Updated at_sessile_service/at_agile_service defs to reference new terms. Updated 5 subdocs (RBSCC, RBSCN, RBSBK, RBSBC, RBSCE). Left opss_sentry_start untouched (Option C — internal procedure, deferred).

### 2026-03-31 11:07 - ₢AxAAI - n

Promote crucible operations to rbsc_ linked terms: rename opbs_bottle_start→rbsc_charge and opbr_bottle_run→rbsc_enjoin, define pentacle/sessile/agile/quench/hail/rack/scry as first-class spec vocabulary

### 2026-03-31 10:50 - ₢AxAAH - W

Renamed diagnostic/interactive commands: ConnectBottle→Rack (rbw-B→rbw-cr), ConnectSentry→Hail (rbw-S→rbw-ch), ObserveNetworks→Scry (rbw-o→rbw-cs), eliminated ConnectCenser (rbw-C, no diagnostic use case). Shell: rbob_connect_bottle→rbob_rack, rbob_connect_sentry→rbob_hail, rbob_observe→rbob_scry, deleted rbob_connect_pentacle. Zipper: RBZ_CRUCIBLE_HAIL/RACK/SCRY with colophons rbw-ch/cr/cs. git mv 8 tabtargets + git rm ConnectCenser. Consumer docs, BUK docs (BUS0, README, buk-claude-context), current heat examples aligned. Straggler grep clean.

### 2026-03-31 10:50 - ₢AxAAH - n

Rename crucible interactive-shell operations to thematic vocabulary (hail/rack/scry), unify colophons under rbw-c* prefix, remove pentacle connect, and update all BUK/RBK documentation examples to match

### 2026-03-31 10:44 - ₢AxAAG - W

Renamed bottle run→enjoin in spec layer: display text 'Agile Bottle Run Rule'→'Agile Enjoin Rule', definition prose 'Creates and executes'→'Enjoins', RBSCN subdoc prose updated. No shell/dispatch/tabtarget changes needed — runtime code for agile dispatch doesn't exist yet. Attribute name opbr_bottle_run retained (consistent with charge/quench pattern).

### 2026-03-31 10:44 - ₢AxAAG - n

Align opbr_bottle_run display text and prose to enjoin vocabulary across spec top and crucible enjoin subdoc

### 2026-03-31 10:39 - ₢AxAAF - W

Renamed start→charge and stop→quench across all layers: rbob_start→rbob_charge, rbob_stop→rbob_quench (shell), RBZ_BOTTLE_START→RBZ_CRUCIBLE_CHARGE rbw-cC, RBZ_BOTTLE_STOP→RBZ_CRUCIBLE_QUENCH rbw-cQ (zipper+workbench), git mv 6 tabtargets (3 Start→Charge + 3 Stop→Quench for tadmor/srjcl/pluml), spec linked term display text + definition prose, consumer docs, BUK docs (BCG guide, buk-claude-context, README examples), rbf_Foundry, rbcc_Constants, rbgm_ManualProcedures, rbtb_testbench, RBSAK subdoc. Straggler grep clean.

### 2026-03-31 10:39 - ₢AxAAF - n

Renamed bottle start/stop→crucible charge/quench across runtime and docs. Shell: rbob_start→rbob_charge, rbob_stop→rbob_quench. Zipper: RBZ_BOTTLE_START→RBZ_CRUCIBLE_CHARGE, RBZ_BOTTLE_STOP→RBZ_CRUCIBLE_QUENCH with colophons rbw-s→rbw-cC, rbw-z→rbw-cQ. Spec: opbs_bottle_start display text and sessile/agile prose updated. Consumer docs, BCG examples, BUK context, testbench baste calls, workbench qualification gate, and foundry kludge comments aligned.

### 2026-03-31 10:33 - ₢AxAAE - W

Renamed bottle service→crucible as composite assembly concept. Spec: at_bottle_service→rbsc_crucible linked term (mapping+anchor+definition), at_agile_service/at_sessile_service display text updated. Subdocs: RBSCO, RBSSS, RBRN, RBSHR updated (RBSBL/RBSCN/RBSBC correctly retained 'bottle service definition' for Compose service entry). Shell: rbob_bottle.sh/rbob_cli.sh/rboo_observe.sh comments and user messages. Regime: rbrn_regime.sh validation string. Public: index.html (headings+anchors+prose), rbm-abstract-drawio.svg (3 label replacements). Straggler grep clean.

### 2026-03-31 10:33 - ₢AxAAE - n

Renamed "bottle service" → "crucible" across runtime (rbob_bottle, rbob_cli, rboo_observe), spec (RBS0 linked terms at_bottle_service→rbsc_crucible, RBRN regime descriptions, RBSCO, RBSHR, RBSSS), public index.html, and abstract SVG diagram. Replaced stty+awk terminal width detection with tput cols in bul_launcher.

### 2026-03-31 10:22 - ₢AxAAD - W

Renamed censer→pentacle across 25 files + 1 git mv. Runtime: compose service/container/network_mode, rbob_bottle.sh (ZRBOB_CENSER→ZRBOB_PENTACLE, rbob_connect_censer→rbob_connect_pentacle), rbjc_censer.sh→rbjp_pentacle.sh (file + prefix + healthcheck sentinel rbjch→rbjph), Dockerfile COPY/comments. Dispatch: zipper RBZ_BOTTLE_CENSER→RBZ_BOTTLE_PENTACLE. Spec: at_censer_container→at_pentacle_container quoin + 8 subdocs. Consumer docs, ifrit CLAUDE.md, index.html, testbench/fixture, legacy observe, vessel description. Straggler grep clean.

### 2026-03-31 10:22 - ₢AxAAD - n

Renamed censer→pentacle across all layers: compose service/container/healthcheck, RBOB bottle orchestration (connect, validate, info), observe scripts, testbench exec helpers, zipper enrollment, sentry Dockerfile (COPY + comments), pentacle init script (RBJP prefix, log lines, health file rbjph_healthy), spec definitions (RBS0 linked term + prose), subdocs (RBSBC/RBSBL/RBSCC/RBSCE/RBSCN/RBSCO/RBSNC/RBSSS), consumer docs, public index.html, vessel env, and CLAUDE.md RBJ children list. Straggler grep clean.

### 2026-03-31 10:12 - ₢AxAAC - W

Renamed MCM identity tier vocabulary across 4 files: lemma/lemmata→quoin/quoins, graven→inlay/inlays, intaglio→sprue/sprues. Updated MCM attribute mappings (mcm_lemma→mcm_quoin, mcm_graven→mcm_inlay, mcm_intaglio→mcm_sprue), anchor definitions with revised prose, AXLA references (axr_member, axhems_scoped_method, hierarchy table), and retired heat paddock memo. Straggler grep clean except historical gallops JSON.

### 2026-03-31 10:12 - ₢AxAAC - n

Renamed MCM token identity tier: lemma→quoin, graven→inlay, intaglio→sprue across MCM spec, AXLA lexicon, and heat paddock

### 2026-03-31 10:07 - ₢AxAAL - W

Renamed creates→ordain across all layers: rbf_create→rbf_ordain, RBZ_CREATE_CONSECRATION→RBZ_ORDAIN_CONSECRATION, rbw-DC→rbw-DO tabtarget (git mv), minted rbtgo_ark_ordain quoin in RBS0, updated subdocs (RBSAC/RBSAG now use linked term instead of backticked function name), consumer docs, manual procedures, workbench qualification gate, and test fixture. Straggler grep clean.

### 2026-03-31 10:06 - ₢AxAAL - n

Renamed creates→ordain across all layers: rbf_create→rbf_ordain, RBZ_CREATE_CONSECRATION→RBZ_ORDAIN_CONSECRATION, rbw-DC→rbw-DO tabtarget (git mv), minted rbtgo_ark_ordain quoin in RBS0+RBSCL, updated subdocs (RBSAC/RBSAG to use linked term), consumer docs, manual procedures, workbench qualification gate, and test fixture

### 2026-03-31 10:01 - ₢AxAAK - W

Renamed checks→tally across all layers: shell (rbf_check_consecrations→rbf_tally), zipper (RBZ_CHECK_CONSECRATIONS→RBZ_TALLY_CONSECRATIONS, rbw-Dc→rbw-Dt), tabtarget (git mv), spec quoin (rbtgo_consecration_check→rbtgo_consecration_tally in RBS0 + RBSCL + RBSCB), consumer docs (CLAUDE.consumer.md + README.consumer.md), constants comment, and test fixture. rbcc_ prefix retained as Constants module prefix. Straggler grep clean.

### 2026-03-31 10:01 - ₢AxAAK - n

Renamed checks→tally across all layers: rbf_check_consecrations→rbf_tally, RBZ_CHECK_CONSECRATIONS→RBZ_TALLY_CONSECRATIONS, rbw-Dc→rbw-Dt tabtarget, rbtgo_consecration_check→rbtgo_consecration_tally quoin, updated spec subdocs, consumer docs, constants comment, and test fixture

### 2026-03-31 09:55 - ₢AxAAQ - W

Renamed 12 RBS0 subdocument files to new verb-colorized acronyms via git mv (pure renames for ancestry tracking), updated CLAUDE.md mappings with re-sorted order and added missing RBSAE entry, updated all 12 RBS0 include directives, grep-verified no straggler references

### 2026-03-31 09:55 - ₢AxAAQ - n

Renamed 12 RBS0 subdocument files to match new operation vocabulary, updated CLAUDE.md mappings (re-sorted + added missing RBSAE entry) and RBS0 include directives

### 2026-03-31 07:35 - Heat - T

diagnostic-verbs-rack-hail-scry

### 2026-03-31 07:34 - Heat - d

paddock curried: observe→scry election for network diagnostic verb

### 2026-03-30 18:03 - Heat - S

subdocument-acronym-rename

### 2026-03-30 16:16 - Heat - d

paddock curried: added ark forward dependency note, no content changes

### 2026-03-30 16:15 - Heat - f

silks=rbk-mvp-5-verb-colorization

### 2026-03-30 16:11 - Heat - S

image-verbs-wrest-jettison-plumb

### 2026-03-30 16:11 - Heat - S

marshal-verbs-zero-proof

### 2026-03-30 16:11 - Heat - S

payor-verbs-levy-unmake-mantle

### 2026-03-30 16:11 - Heat - S

governor-verbs-knight-forfeit

### 2026-03-30 16:09 - Heat - d

paddock curried: added Group A (role/regime) and Group B (image-level) verb decisions, full verb registry

### 2026-03-30 14:23 - Heat - S

consecration-creates-to-ordain

### 2026-03-30 14:23 - Heat - S

consecration-checks-to-tally

### 2026-03-30 14:23 - Heat - d

paddock curried: added Decided: Consecration Operation Verbs (tally + ordain)

### 2026-03-30 08:02 - Heat - S

documentation-sweep

### 2026-03-30 08:02 - Heat - S

quoin-minting-rbsc-terms

### 2026-03-30 08:01 - Heat - S

diagnostic-verbs-rack-hail-observe

### 2026-03-30 08:01 - Heat - S

operational-verb-enjoin

### 2026-03-30 08:01 - Heat - S

lifecycle-verbs-charge-quench

### 2026-03-30 08:01 - Heat - S

bottle-service-to-crucible

### 2026-03-30 08:00 - Heat - S

censer-to-pentacle

### 2026-03-30 08:00 - Heat - S

mcm-tier-rename

### 2026-03-29 10:16 - Heat - f

racing

### 2026-03-29 10:15 - Heat - f

stabled

### 2026-03-29 10:10 - Heat - d

paddock curried: elect rbsc_ flat prefix for crucible quoins, resolve all open items

### 2026-03-29 09:45 - Heat - d

paddock curried: finalize colophon family rbw-c*, promote observe to HIGH, trim paddock to change-focused

### 2026-03-29 09:34 - Heat - d

paddock curried: elect charge/quench/enjoin lifecycle verbs, hail/rack diagnostic verbs, colophon migration plan

### 2026-03-29 09:12 - Heat - d

paddock curried: elect crucible (bottle-service) and pentacle (censer) renames

### 2026-03-29 08:40 - Heat - f

racing

### 2026-03-28 07:16 - Heat - f

silks=rbk-mvp-5-bottle-lifecycle-vocabulary

### 2026-03-28 06:49 - Heat - n

Fix gazette protocol documentation: redirect enroll/redocket to gazette path, clarify H1 wire format delimiter, add failure recovery guidance

### 2026-03-28 06:45 - Heat - T

mcm-rename-quoin-inlay-sprue

### 2026-03-28 06:45 - Heat - T

spec-broach-quench-decant-vocabulary

### 2026-03-28 06:45 - Heat - d

paddock curried

### 2026-03-27 18:05 - Heat - T

spec-broach-quench-decant-vocabulary

### 2026-03-27 17:45 - Heat - S

mcm-rename-quoin-inlay-sprue

### 2026-03-27 17:26 - Heat - S

spec-broach-quench-assay-vocabulary

### 2026-03-27 17:05 - Heat - N

rbk-mvp-4-bottle-lifecycle-vocabulary

