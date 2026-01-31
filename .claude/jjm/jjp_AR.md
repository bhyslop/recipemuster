# Paddock: rbw-ark-vessel-vocabulary

## Context

Establish the ark/vessel vocabulary foundation across Recipe Bottle specifications. This work emerged from attempting to add a cosmology introduction to Getting Started and discovering the underlying vocabulary needed formalization.

## Key Conceptual Model

### The Layering

| Layer | Regime | Purpose | Assignment Files |
|-------|--------|---------|------------------|
| **Build Input** | RBRV (Vessel) | Defines *what* to build | `rbev-vessels/*/rbrv.env` |
| **Build Output** | Ark | Coherent result pair | `-image` + `-about` artifacts |
| **Runtime Config** | RBRN (Nameplate) | Defines *how* to deploy | `Tools/rbw/rbrn_*.env` |

### The Flow

```
Vessel (RBRV) → [conjure/build] → Ark → [deploy via Nameplate] → Containers
```

### Ark Definition

An **ark** is a precise built vessel — one of potentially several differing only by timestamp (ark_stamp). It comprises:
- `{moniker}:{ark_stamp}-image` — the container image
- `{moniker}:{ark_stamp}-about` — the build provenance/metadata

The ark is NOT a regime (no assignment file). It is a **bridging artifact** that:
- Is **produced** by building a vessel
- Is **stored** in the container registry
- Is **consumed/selected** by nameplates

### Vessel Tagging Convention

The sigil naming convention (`rbev-sentry-*`, `rbev-bottle-*`) embeds role tags. The vessel machinery treats them uniformly — the distinction is semantic/conventional, not enforced structurally.

### Modern MCM Patterns

AXLA and modern MCM documents use anchors WITHOUT `term_` prefix:
```asciidoc
// Modern (correct)
:rbrv_sigil:    <<rbrv_sigil,RBRV_SIGIL>>

// Old (RBS currently uses this)
:rbrn_moniker:  <<term_rbrn_moniker,RBRN_MONIKER>>
```

### AXLA Regime Vocabulary

AXLA provides motifs for regime concepts:
- `{axrg_regime}` — structured configuration system
- `{axrg_variable}` — named config element (schema level)
- `{axrg_assignment}` — specific value bound to variable
- `{axrg_prefix}` — namespace prefix
- `{axf_bash}` — .env file format (source-able)

Specs should reference these when discussing regimes for consistency.

### Toward RBSA

RBAGS and RBS will eventually combine into RBSA. This heat establishes vocabulary consistency as a preparatory step. For now, regime specs (RBRV, RBRN) are standalone AsciiDoc files that non-incidentally share mappings with their parent specs.

## References

- `lenses/RBAGS-AdminGoogleSpec.adoc` — Admin Google spec (ark definitions at lines 608-687)
- `lenses/RBS-Specification.adoc` — Makefile service spec (needs anchor modernization)
- `lenses/RBRN-RegimeNameplate.adoc` — Nameplate regime spec
- `lenses/CRR-ConfigRegimeRequirements.adoc` — Config regime definition (old, makefile-centric — being replaced by BURS)
- `Tools/buk/vov_veiled/BURS-BashUtilityRegimeSpec.adoc` — [TO CREATE] Modern bash regime spec
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` — Modern concept model patterns
- `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc` — Axial lexicon (regime/format motifs)
- `Tools/rbw/rbgc_Constants.sh` — GCP constants (needs ark suffix constants)
- `rbev-vessels/*/rbrv.env` — Vessel assignment files
- `Tools/rbw/rbrn_*.env` — Nameplate assignment files

## Heat Nature & Wrap Discipline

**This is a documentation-only heat** — all paces involve editing AsciiDoc specifications and AXLA-annotated definitions. No Rust code changes, no build/test requirements before wrapping.

When wrapping a pace, simply verify:
- Changes to lenses/ files are correct (review the edits)
- No accidental syntax errors in AsciiDoc

Then invoke `/jjc-pace-wrap` without pre-wrap build/test.

## Paddock Maintenance

**On pace wrap**: Review whether the paddock needs revision given what the pace accomplished. Specifically:
- New files created? Add to References section
- Conceptual understanding evolved? Update Key Conceptual Model
- Patterns established that future paces should follow? Document them

The paddock is living documentation — keep it current as the heat progresses.
