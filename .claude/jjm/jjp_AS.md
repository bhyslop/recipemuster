# Paddock: rbw-spec-axla-coherence

## Context

Bring Recipe Bottle specification ecosystem into coherent alignment with AXLA patterns. This heat establishes the structural foundation.

## Pattern Established in ₣AR (RBRV exemplar)

The RBRV regime subdocument (RBSRV) was cleaned up as an exemplar. Key patterns to follow:

### Type Voicing Pattern

1. **Parent doc defines `rbst_*` type voicings** in "Type Voicings" section:
   - `rbst_xname` voices `axtu_xname` — cross-context safe identifier
   - `rbst_description` voices `axtu_string` — human-readable text
   - `rbst_image_ref` voices `axtu_string` — container image reference
   - `rbst_path` voices `axtu_path` — file system path
   - `rbst_platform_list` voices `axtu_string` — space-delimited platforms
   - `rbst_binfmt_policy` voices `axt_enumeration` — policy enum

2. **Regime variable voicings** reference subspecialized types:
   ```asciidoc
   [[rbrv_sigil]]
   // ⟦axl_voices axrg_variable rbst_xname⟧
   {rbrv_sigil}::
   Vessel identifier; basis for directory and ark naming.
   ```

3. **Subdocs use `rbst_*` types** in tables (they resolve via parent mappings).

4. **Subdocs have NO anchors** — parent doc owns all `[[anchor]]` definitions.

### Subdocument Structure

- **Overview**: Brief context, explain any bifurcations (e.g., binding vs conjuring)
- **Feature Groups**: Tables with Type column using `rbst_*` references
- **NO "Core Term Definitions"** — parent doc owns these
- **Use NOTE blocks** for group-level applicability rules (not repeated per row)

### Open Questions for This Heat

- Should `axtg_*` (Google-specific types) get `rbst_*` treatment, or are they already specific enough?
- How do we handle regimes with mixed universal/Google types (RBRR, RBRA)?

## Downstream Work

**CRITICAL: After completing this heat**, plan a follow-up heat to **refine AXLA regime vocabulary itself** based on what we learn during:
- Expanding axrg_* section
- Creating BURS and BURC specs
- Studying all RB regimes (BUD_, RBRN, RBRR, etc.)

AXLA will need refinement to capture patterns we discover. That work belongs in a future heat (not in ₣AS).

## References

- AXLA: `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc`
- CRR: `lenses/CRR-ConfigRegimeRequirements.adoc`
- RBS: `lenses/RBS-Specification.adoc`
- RBRN: `lenses/RBRN-RegimeNameplate.adoc`
- RBAGS: `lenses/RBAGS-AdminGoogleSpec.adoc`
- BUS: `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc`
