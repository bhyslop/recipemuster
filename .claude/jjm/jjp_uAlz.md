## Context

Two concept-removal debts identified during ₣Ax (bottle lifecycle vocabulary) work. Both are deletion/simplification — removing concepts that don't carry their weight. Verb renames discovered in the same conversation live in ₣Ax where the execution happens.

## Sequencing: Depends on ₣Ax

₣Ax (verb colorization) will mint some new quoins using existing `rbtgo_ark_*` naming convention (e.g., `rbtgo_ark_ordain`, `rbtgo_ark_plumb`). This heat's ark removal must account for those ₣Ax-minted names in addition to the pre-existing ark references inventoried below.

## Debt 1: Remove "Ark" Concept

**Problem**: "Ark" (`rbtga_ark`) is a grouping noun for image+about+vouch artifacts sharing a consecration timestamp. It has three problems:

1. **Register mismatch** — biblical/nautical in a forge/Solomonic vocabulary. The odd word out.
2. **Collision with "vessel"** — both connote containers. Vessel is where you put things, ark is where things are, but the distinction isn't immediately apparent.
3. **Conceptual redundancy with "consecration"** — the consecration timestamp already IS the grouping key. Operations naturally speak in terms of consecrations, not arks: "tally consecrations," "ordain a consecration," "abjure a consecration." The ₣Ax tally/ordain verbs were coined without "ark" and read better for it.

**Evidence of drift**: The two newest operations (tally, ordain) avoid "ark" naturally. The tally output table headers say "Consecration | Mode | Platforms | Health" — not "Ark."

**Scope**: ~188 occurrences across 19 spec files, ~30 in rbf_Foundry.sh, 4 constants in rbgc_Constants.sh, 8 spec files named `RBS*-ark_*.adoc`, 8 operation quoins named `rbtgo_ark_*`, the entire `rbtga_` prefix category (5 terms), and the `rbtgog_ark` operation group. Plus any `rbtgo_ark_*` quoins minted by ₣Ax.

**Design questions for implementation**:
- What replaces the grouping noun? Options: (a) "consecration" absorbs it — "consecration image," "consecration about," "consecration vouch"; (b) drop the grouping layer entirely — artifacts are just "image artifact," "about artifact," "vouch artifact" scoped by their consecration context; (c) a new forge-register term
- Does `rbtga_` collapse into another category or get a new prefix?
- Sub-artifact compound names: "Ark Image Artifact" → "Consecration Image"? Just "Image Artifact"?
- Operation names: `rbtgo_ark_conjure` → `rbtgo_conjure`? `rbtgo_vessel_conjure`?
- Spec file names: `RBSAA-ark_abjure.adoc` → `RBSAA-abjure.adoc`?
- Shell constants: `RBGC_ARK_SUFFIX_*` → `RBGC_CONSEC_SUFFIX_*`? Or just `RBGC_SUFFIX_*`?

## Debt 2: Remove Rubric Inscribe

**Problem**: `rbf_inscribe()` / `rbw-DI` / `rbtgo_rubric_inscribe` is a dead operation. The spec says "DEFERRED: Inscribe will become reliquary generation (₣Av)" and "Inscribe currently dies with a stub message." The include (`RBSRI-rubric_inscribe.adoc`) is commented out. But the operation is still enrolled in the zipper, has a live function with Cloud Build submission code, and has a tabtarget.

**Live artifacts to remove**:
- `Tools/rbk/rbf_Foundry.sh`: `rbf_inscribe()` and `zrbf_inscribe_submit()` functions, `ZRBF_INSCRIBE_*` variables
- `Tools/rbk/rbz_zipper.sh`: `buz_enroll RBZ_INSCRIBE_RELIQUARY "rbw-DI"`
- `tt/rbw-DI.DirectorInscribesReliquary.sh`: tabtarget
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`: `rbtgo_rubric_inscribe` quoin mapping, anchor, deferred section
- `Tools/rbk/vov_veiled/RBSRI-rubric_inscribe.adoc`: spec file (if present, already commented-out include)
- Consumer docs referencing `rbw-DI`

**Also evaluate**: Is the broader "rubric" concept (`rbtgr_rubric`, `rbtgr_build_json`, `rbtgr_provenance`) still load-bearing after ₣Av eliminated triggers? The mapping comment says "Rubric concept simplified (₣Av) — triggers/rubric repo eliminated, term retained for build definition." Determine whether "rubric" still earns its existence or should be simplified further.

## References

- ₣Ax paddock (verb decisions, ark forward dependency note)
- RBS0-SpecTop.adoc lines 75-81 (ark mappings), 960-969 (inscribe deferred), 1507-1534 (ark definition), 1720+ (rubric definition)
- rbf_Foundry.sh lines 848-870 (inscribe), 575+ (ark suffixes)
- 260330 conversation: ark assessment, inscribe discovery