## Context

Vocabulary debts identified during ₣Ax (bottle lifecycle vocabulary) work. Three categories: concept removal (ark, rubric inscribe), role/regime verb colorization, and image-level verb colorization. The generic verbs ("create," "delete," "reset," "retrieve," "inspect," "destroy," "duplicate") fail the same test as the crucible verbs that ₣Ax fixes — they're off-metaphor in a system with rich liturgical vocabulary.

## Debt 1: Remove "Ark" Concept

**Problem**: "Ark" (`rbtga_ark`) is a grouping noun for image+about+vouch artifacts sharing a consecration timestamp. It has three problems:

1. **Register mismatch** — biblical/nautical in a forge/Solomonic vocabulary. The odd word out.
2. **Collision with "vessel"** — both connote containers. Vessel is where you put things, ark is where things are, but the distinction isn't immediately apparent.
3. **Conceptual redundancy with "consecration"** — the consecration timestamp already IS the grouping key. Operations naturally speak in terms of consecrations, not arks: "tally consecrations," "ordain a consecration," "abjure a consecration." The ₣Ax tally/ordain verbs were coined without "ark" and read better for it.

**Evidence of drift**: The two newest operations (tally, ordain) avoid "ark" naturally. The tally output table headers say "Consecration | Mode | Platforms | Health" — not "Ark."

**Scope**: ~188 occurrences across 19 spec files, ~30 in rbf_Foundry.sh, 4 constants in rbgc_Constants.sh, 8 spec files named `RBS*-ark_*.adoc`, 8 operation quoins named `rbtgo_ark_*`, the entire `rbtga_` prefix category (5 terms), and the `rbtgog_ark` operation group.

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

## Decided: Role Authority & Regime Lifecycle Verbs (Group A)

Renaming the generic verbs used by role actors (Governor, Payor, Marshal) for authority conferral, infrastructure lifecycle, and regime operations. Register: feudal/military commission — the institutional governance layer above the forge/Solomonic operations.

The role nouns (Payor, Governor, Marshal, Director, Retriever, Mason) form a chartered institution: the Payor funds, the Governor authorizes, the Marshal maintains order, the Director commands operations, the Retriever serves, the Mason builds. The verbs should come from that same institutional register.

**Verb initial collision analysis** — letters taken by cloud/artifact verbs: A(abjure), B(bind), C(conjure), D(delete→being replaced), E(enshrine), G(graft), H(hail), I(inspect→being replaced), O(ordain), Q(quench), R(rack), S(summon), T(tally), V(vouch). Group A verbs must use letters from the remaining pool. Cross-group reuse (A vs B) is acceptable since the domains are distinct.

| Operation | Old verb | New verb | Letter | Actor | Register | Rationale |
|-----------|----------|----------|--------|-------|----------|-----------|
| Create role SA + IAM | create | **knight** | K | Governor | feudal | Confer knighthood — the Governor knights the Director/Retriever into service. Immediately understood, distinctive. |
| Delete role SA | delete | **forfeit** | F | Governor | feudal/legal | Authority seized back by decree. The Governor forfeits the Director — the office is forfeit. Strong pair with knight. |
| Create depot (GAR + bucket + pool + mason) | create | **levy** | L | Payor | feudal/military | Raise by sovereign authority. The Payor levies the Depot — the patron raises infrastructure by financial command. |
| Destroy depot | destroy | **unmake** | U | Payor | archaic | Reverse of creation. The Payor unmakes the Depot. More fundamental than "destroy" — the thing is un-made. Natural pair with levy. |
| Reset governor (destroy old SA, create fresh) | reset | **mantle** | M | Payor | feudal | Invest with the mantle of authority. The Payor mantles the Governor — the old mantle is cast off, a new one placed on the shoulders. Visual, feudal, captures destroy+recreate. |
| Reset regime to blank template | reset | **zero** | Z | Marshal | military | Zero the instrument before calibration. The Marshal zeroes the regime. Clean, precise, distinctive. |
| Duplicate repo for release testing | duplicate | **proof** | P | Marshal | publishing | A proof copy before the print run. The Marshal proofs the release. Exactly what this is — a test copy for qualification. |

Natural pairs:
- knight / forfeit (confer / seize authority)
- levy / unmake (raise / reverse infrastructure)
- zero / proof (prepare / test for release)

Confidence: HIGH.

## Decided: Image/Artifact-level Verbs (Group B)

Renaming the generic verbs for image-level operations. These parallel the consecration-level verbs but operate on individual artifacts rather than coherent consecration packages. Register: forge/Solomonic (same as consecration-level) but with verbs that convey the more surgical, single-artifact nature.

The consecration-level verbs are the standard: ordain, abjure, tally, vouch, summon, enshrine. Every consecration-level operation has a liturgical verb; every image-level operation had a generic one. This closes the gap.

Key insight: "retrieve" was contaminated by the Retriever role name — like naming the Director's primary operation "direct." The verb should not echo the role.

| Operation | Old verb | New verb | Letter | Actor | Register | Rationale |
|-----------|----------|----------|--------|-------|----------|-----------|
| Pull specific image by ref | retrieve | **wrest** | W | Retriever | feudal/physical | Seize by force, pull away. The Retriever wrests the image from the registry. Active, forceful, single-target. Distinct from summon (which pulls the consecration package). |
| Delete specific image tag | delete | **jettison** | J | Director | naval | Throw overboard to save the ship. The Director jettisons the image. Surgical discard of one artifact — vivid, captures "sacrifice one to preserve the rest." Distinct from abjure (which destroys the full consecration). |
| Examine trust posture | inspect | **plumb** | P | Retriever | mason/forge | Probe the depths with a plumb-bob. The Retriever plumbs the image. Forensic examination — measure the depth of trust (provenance, SBOM, cache delta, Dockerfile). The mason's precision instrument. |

Confidence: HIGH.

## Full Cloud Verb Registry (post ₣Ax + ₣Az)

**Consecration-level** (Solomonic/forge): abjure(A), enshrine(E), ordain(O), summon(S), tally(T), vouch(V)
**Image-level** (Solomonic/forge): jettison(J), plumb(P), wrest(W)
**Crucible** (forge): charge(C), enjoin(E), hail(H), observe(O), quench(Q), rack(R)
**Role/regime** (feudal/military): forfeit(F), knight(K), levy(L), mantle(M), proof(P), unmake(U), zero(Z)

17 verbs. No two share meaning. Each is non-software, memorable, and immediately evocative.

## References

- ₣Ax paddock (tally/ordain decisions, ark discussion, crucible colophon family)
- RBS0-SpecTop.adoc lines 75-81 (ark mappings), 960-969 (inscribe deferred), 1507-1534 (ark definition), 1720+ (rubric definition)
- rbf_Foundry.sh lines 848-870 (inscribe), 575+ (ark suffixes)
- rbz_zipper.sh (full colophon registry — all enrolled operations)
- 260330 conversation: ark assessment, inscribe discovery, verb election sessions