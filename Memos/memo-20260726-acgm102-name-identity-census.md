# ACGm_102 name-identity census — corpus-wide

*Pace ₢CAABw (name-identity-census), heat rbk-23-win-acg-incorporate (₣Bz). Detect-only per ACGm_102 — nothing mutated by this pace. Repairs land as separate reviewed moves.*

## Method

The mechanical census (ACGm_102, *Name-identity's symbol link (landed)*):

1. Grep every `//axvo_procedure` / `//axvo_method` voicing carrying the `axd_inlaid` dimension.
2. Read the **inlay** — the first backtick token of the definition body — as the declared implementing symbol.
3. Read the **display-text** from the quoin's mapping-section entry (`:attr: <<anchor,Display Text>>`).
4. Flag display ≠ inlay; grep the inlay in source to confirm it resolves.

Run across `Tools/*/vov_veiled/*.adoc`. Extraction script archived at scratchpad `census2.py`.

Corpus tallies:

| Spec | Inlaid voicings | display ≠ inlay |
|------|----------------:|----------------:|
| `RBS0-SpecTop.adoc` | 64 | 9 |
| `JJS0_JobJockeySpec.adoc` | 43 | 41 |
| `VOS0-VoxObscuraSpec.adoc` | 5 | 5 |
| `AXLA-Lexicon.adoc` | 8 | 0 (dimension-definition examples, not operation quoins) |

## The census-first reading

The raw disagreement counts are misleading until read through ACGm_102's *census-before-exception* triage. The three specs sit at two different display conventions, and the convention — not the raw count — decides whether a disagreement is a defect:

- **RBS0 — display = implementing symbol.** 55 of 64 voicings display their function symbol verbatim (`rbtgo_ark_summon` → `rbfr_summon`), so display and inlay agree by construction. The 9 disagreements are drift *away from RBS0's own convention*: an ad-hoc English/Title-Case phrase where the symbol belongs.
- **JJS0 / VOS0 — display = operator surface, inlay = internal symbol.** Every `jjdo_*` quoin displays its operator-facing MCP command (`jjx_create`) while the inlay records the internal dispatcher (`jjrx_run_nominate`); VOS0 displays the ceremony name (`Release`) over the internal symbol (`vob_release`). Their near-100% "disagreement" rate is the signature of a deliberate three-layer identity (operator command / operation quoin / implementing symbol), not of pervasive drift.

The distinction is the whole point of ACGm_102's triage clause: a display/inlay disagreement is first a census question (*does the display name the operator surface?*), and only a linkage defect when the display names **neither** the operator surface nor the symbol — an ad-hoc phrase.

*Fable-ruling correction (260726):* "census-sound" is the **gate verdict, not conformance**. Read strictly, ACGm_102's authority ("a procedure's function name equals its quoin's display-text") has a single correct convention — **display = implementing symbol** — and the triage clause resolves the sound-census case explicitly ("the shared core is the implementing symbol — inlay and display alike"). The operator surface has its own census layer (the colophon quoins); the operation quoin is not where the surface is displayed. So JJS0/VOS0 are the **nonconforming side** as the rule stands — not cleared, but *pending a convention ruling*. What makes them more than drift: `jjx_create` is itself a stable machine name (the wire command), so JJS0 embodies a genuine three-layer identity the rule as written never contemplated.

## RBS0 findings (9)

### Group A — repair candidates: display drifted to an ad-hoc phrase; census sound, single symbol exists

Each names neither the colophon frontispiece nor the symbol. RBS0's convention is display = symbol; the reviewed repair is display-text → the inlay symbol (the ACGm_102 worked-case pattern, as ark-inspect was repaired to `zrbfc_plumb_core`).

| Anchor | Current display | Symbol (confirmed in source) | Notes |
|--------|-----------------|------------------------------|-------|
| `rbtgo_image_rekon` (L1810) | `Image Rekon` | `rbfl_rekon_hallmark` (`rbfln_inventory.sh:137`) | Body already states "One procedure (`rbfl_rekon_hallmark`)" — census explicitly unitary. |
| `rbtgo_image_audit` (L1823) | `Image Audit` | `rbfl_audit_hallmarks` (`rbfln_inventory.sh:211`) | Flat catalog, single symbol. |
| `rbsc_charge` (L2304) | `Sessile Charge Rule` | `rbob_charge` (`rbob_bottle.sh:511`) | Operator-facing (`rbw-cC` Charge); phrase matches neither colophon nor symbol. |
| `rbsfh_dockerfile_hygiene` (L6040) `[internal]` | `Dockerfile Hygiene` | `rbfh_dockerfile_check` (`rbfh_hygiene.sh:54`) | Lower priority — `axd_internal`, no operator surface. |
| `rbtoe_depot_list_update` (L5867) `[internal]` | `Depot List Update Pattern` | `zrbgp_depot_list_update` (`rbgp_payor.sh:334`) | Lower priority — `axd_internal`, private helper. |

### Group B — census-sound exceptions: no defect, report-only

The four hierophant ceremonies (`rbth_essai` L2205, `rbth_docimasy` L2222, `rbth_ostend` L2239, `rbth_harbinger` L2256). Display is the civic verb (`Essai`); inlay is the wire-subcommand transport (`hierophant essai`) — the surveyed no-single-symbol specimen ACGm_102 names explicitly. The display names the operator-surface verb (the `essai` subcommand); the inlay carries the full CLI transport. Census-sound; no function symbol to repair to. Leave as-is.

## JJS0 / VOS0 — convention divergence (out of RBK scope; route to CMK/ACG)

The 46 JJS0/VOS0 disagreements are the deliberate display=operator-surface convention. Read strictly against ACGm_102 (above), they are the nonconforming side — but the nonconformance is interesting, not mere drift: `jjx_create` is itself a machine name, so JJS0 embodies a three-layer identity the rule never anticipated. The open question is therefore a **spec-authoring ruling for the ACG owner**, not an RBK repair (rbk-23 is RBK; the discovery recipe homes on `RBS*.adoc`; JJS0/VOS0 are the JJK/VOK kits' own specs):

> Should ACGm_102 be amended to bless "display = surface symbol where the surface token is itself a machine name" (blessing JJS0/VOS0), or should JJS0/VOS0 be repaired toward display = dispatcher symbol?

Route as an itch or a CMK pace with that crux stated; take **no** JJS0/VOS0 action under rbk-23. Per the fable ruling, this question does **not** gate the RBS0 Group-A repairs — whichever way it lands, display = symbol cannot become wrong *for RBS0*, whose convention already is display = symbol.

## Recommended disposition (per fable ruling, 260726)

- **Group A (RBS0):** slate **one** reviewed repair pace covering all five sites, display-text → symbol — homogeneous one-line mapping-section edits with a shared verification recipe; per-site paces would be ceremony without load. The rule draws no operator-facing/`axd_internal` distinction, so do not split; order the three operator-facing ones first within the pace. **Caution to bake into the docket:** `rbsc_charge`'s display "Sessile Charge Rule" smells like the quoin may double as a rule-name in running prose — the repair must grep the attribute's usage sites and confirm `rbob_charge` reads sensibly at every reference before landing (a mechanical swap that garbles a prose sentence is the one failure mode). Repair now; not gated on the Q2 ruling.
- **Group B (RBS0):** no action; documented as the surveyed no-single-symbol exception.
- **JJS0 / VOS0:** itch/route the convention-ruling crux above to CMK/ACG; no repair under this heat.
