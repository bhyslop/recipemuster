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

- ~~Should `axtg_*` (Google-specific types) get `rbst_*` treatment, or are they already specific enough?~~ **RESOLVED** in ₢ASAAG — see Decision below.
- ~~How do we handle regimes with mixed universal/Google types (RBRR, RBRA)?~~ **RESOLVED** — same `rbst_*` pattern for both; see Decision below.

## Decision: Google Type Voicing Pattern (₢ASAAG)

**Decision**: Google-specific types (`axtg_*`) MUST get `rbst_*` subspecializations, same as universal types.

**Option chosen**: B — require project-specific `rbst_*` type voicings for all regime variable types, including Google types.

**Rationale**:

1. **`axtg_*` terms are annotation-only** — they appear exclusively in `// ⟦...⟧` Strachey bracket comments. They are never AsciiDoc attribute references, never render in document body, and have no cross-reference integrity within AsciiDoc.

2. **`rbst_*` terms are full document citizens** — they are `:rbst_*:` attribute references with `<<anchor,Display>>` targets, render in tables and body text, and participate in AsciiDoc's referential structure. A linter can validate that every regime variable's Type column resolves to a real `rbst_*` definition.

3. **Uniform type surface** — every regime variable table row uses `rbst_*` in the Type column, regardless of whether the underlying AXLA motif is universal (`axtu_*`) or Google-specific (`axtg_*`). No special cases.

4. **Two-layer voicing is the design** — the `rbst_*` type definition carries the `axtg_*` voicing in its own annotation. The regime variable voices `rbst_*`. The `axtg_*` knowledge is centralized in the type definition, not scattered across every variable that uses the type.

5. **Project-wide analysis** — querying "all variables typed as GCP project ID" is a grep for `rbst_gcp_project_id`, not a comment-parsing exercise across annotation syntax.

**New `rbst_*` types to add** (in RBAGS Type Voicings section):

| rbst_ type | voices | RB constraint |
|---|---|---|
| `rbst_gcp_project_id` | `axtg_project_id` | GCP project identifier in RB context |
| `rbst_gcp_region` | `axtg_region` | GCP region in RB context |
| `rbst_gcp_service_account` | `axtg_service_account` | Service account email in RB context |
| `rbst_gcp_billing_account` | `axtg_billing_account` | Billing account identifier in RB context |

**Pattern for regime variable annotations** (after this decision):
```asciidoc
[[rbrr_depot_project_id]]
// ⟦axl_voices axrg_variable rbst_gcp_project_id⟧
{rbrr_depot_project_id}::
The GCP project where all Recipe Bottle resources are created.
```

**Pattern for type voicing definitions** (in RBAGS Type Voicings section):
```asciidoc
[[rbst_gcp_project_id]]
// ⟦axl_voices axtg_project_id⟧
{rbst_gcp_project_id}::
GCP project identifier. 6-30 lowercase letters, digits, and hyphens;
must start with letter, end with letter or digit.
```

## Architectural Insight: Annotation-First Structure (₢ASAAG discussion)

During ₢ASAAG we considered whether regimes need `axrs_*` section motifs (like `axs_*` for procedures). Decision: **NO**.

### Annotations ARE the structure

`axvr_*` and `axhr*_` annotations are **self-describing**. A linter can reconstruct the full regime hierarchy from annotations alone — no section headings needed to carry structural metadata. This contrasts with `axs_*` section motifs which embed structure in document headings.

### Deliberate duelling representations

The `axvr_*`/`axhr*_` annotation approach and the `axs_*` section approach are intentionally coexisting. Regimes use annotations exclusively. If the annotation-first pattern proves more efficient and consistent, it may eventually supersede section motifs for other domains too. **Do not create regime section vocabulary** — keep regimes as a clean testbed for the annotation-first approach.

### Parent vs subdoc: complementary layers, not unified

| Layer | Location | Annotations | Purpose |
|---|---|---|---|
| **Schema** | Parent (RBAGS) | `axvr_*` | Anchors, types, cardinality, groups. One-sentence definitions. |
| **Operational** | Subdoc (RBSRV, RBRN) | `axhr*_` | Usage context, typical values, gate explanations, interactions. |

These stay separate:
- Parent definitions are terse type references — the "what"
- Subdoc expansions are operational narrative — the "how"
- `include::` connects them under the same heading
- ₢ASAAF reinforces this: move detail OUT of parent INTO subdoc

### ax_ terms are comment-only by design

All AXLA terms (`axt_*`, `axtu_*`, `axtg_*`, `axs_*`, etc.) appear exclusively in `// ⟦...⟧` Strachey bracket annotations or `//axvr_*`/`//axhr*_` prefix-discriminated annotations. They are **never** AsciiDoc attribute references (`:term: <<anchor,Display>>`). This is deliberate — the specs are too dense for AXLA terms to compete with project terms in body text. Project-specific voicings (`rbst_*`, `rbrv_*`, etc.) are the only terms that render.

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
