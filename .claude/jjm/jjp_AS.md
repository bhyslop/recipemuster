# Paddock: rbw-spec-axla-coherence

## Context

Bring Recipe Bottle specification ecosystem into coherent alignment with AXLA patterns. This heat establishes the structural foundation, culminating in RBSA — a single unified spec replacing both RBAGS and RBS.

## Decision: RBSA Consolidation (₢ASAAD)

**Decision**: Merge RBAGS-AdminGoogleSpec.adoc and RBS-Specification.adoc into a single RBSA-SpecTop.adoc. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes.

**Three-tier document structure:**

| Tier | Focus | Content source |
|------|-------|---------------|
| **Tier 1: Why and What** | Security architecture (significance-first) | RBS: trust challenge, bottle pattern, security properties |
| **Tier 2: How** | Operations in temporal order | RBAGS: cloud operations (by role). RBS: local operations (bottle lifecycle) — rewritten in AXLA style |
| **Tier 3: Reference** | Lookup material | Both: regimes, types, voicings, patterns, definitions, script internals, trade studies |

**Key sub-decisions:**

- **RBAGS patterns govern** — AXLA annotations, not CRR/cmk vocabulary, wherever applicable
- **cmk_* vocabulary is dead** — Console Makefile Discipline fully superseded by bash model (BUK)
- **Operations bifurcated from script internals** — Bottle lifecycle operations (sentry start, bottle start/stop) expressed as RBAGS-style procedures in Tier 2. Detailed script internals (iptables, DNS/dnsmasq, socat, eBPF) go to Tier 3 reference.
- **CRR not modified in this heat** — CRR-ConfigRegimeRequirements.adoc becomes orphaned when RBSA eliminates crg_* vocabulary; retirement is future work.

## Decision: Google Type Voicing Prefix (₢ASAAD, refining ₢ASAAG)

**Decision**: Google-specific types use `rbgt_*` prefix, NOT `rbst_gcp_*`.

**Rationale**: `rbst_gcp_*` is double-prefixing — a namespace hack. `rbgt_` is clean, sits naturally under `rbg` (already "RB Google" in the code namespace), and avoids the token/readability cost of compound prefixes.

**Two type voicing families in RBSA:**

| Prefix | Voices | Purpose |
|--------|--------|---------|
| `rbst_*` | `axtu_*` (universal AXLA types) | Universal types: xname, path, string, etc. |
| `rbgt_*` | `axtg_*` (Google AXLA types) | Google types: project_id, region, service_account, billing_account |

**New `rbgt_*` types** (in RBSA Type Voicings section):

| rbgt_ type | voices | RB constraint |
|---|---|---|
| `rbgt_project_id` | `axtg_project_id` | GCP project identifier in RB context |
| `rbgt_region` | `axtg_region` | GCP region in RB context |
| `rbgt_service_account` | `axtg_service_account` | Service account email in RB context |
| `rbgt_billing_account` | `axtg_billing_account` | Billing account identifier in RB context |

**Updated pattern for regime variable annotations:**
```asciidoc
[[rbrr_depot_project_id]]
// ⟦axl_voices axrg_variable rbgt_project_id⟧
{rbrr_depot_project_id}::
The GCP project where all Recipe Bottle resources are created.
```

**Updated pattern for type voicing definitions:**
```asciidoc
[[rbgt_project_id]]
// ⟦axl_voices axtg_project_id⟧
{rbgt_project_id}::
GCP project identifier. 6-30 lowercase letters, digits, and hyphens;
must start with letter, end with letter or digit.
```

**Note**: This supersedes the ₢ASAAG paddock entry that proposed `rbst_gcp_*`. The ₢ASAAG decision that Google types MUST get project-specific voicings still stands — only the prefix changed.

## Decision: Google Type Voicing Requirement (₢ASAAG)

**Decision**: Google-specific types (`axtg_*`) MUST get project-specific subspecializations, same as universal types.

**Rationale** (unchanged from original ₢ASAAG):

1. **`axtg_*` terms are annotation-only** — they appear exclusively in `// ⟦...⟧` Strachey bracket comments. Never AsciiDoc attribute references, never render in document body.

2. **Project type voicings are full document citizens** — `:rbst_*:` and `:rbgt_*:` attribute references with `<<anchor,Display>>` targets, render in tables and body text, participate in AsciiDoc's referential structure.

3. **Uniform type surface** — every regime variable table row uses `rbst_*` or `rbgt_*` in the Type column. No raw AXLA terms in tables. At a glance you can see which variables are Google-specific (`rbgt_*`) vs universal (`rbst_*`).

4. **Two-layer voicing is the design** — the type definition carries the `axtg_*` voicing in its own annotation. The regime variable voices the project type. The `axtg_*` knowledge is centralized in the type definition.

5. **Project-wide analysis** — querying "all variables typed as GCP project ID" is a grep for `rbgt_project_id`, not a comment-parsing exercise.

## Pattern Established in ₣AR (RBRV exemplar)

The RBRV regime subdocument (RBSRV) was cleaned up as an exemplar. Key patterns to follow:

### Type Voicing Pattern

1. **Parent doc defines type voicings** in "Type Voicings" section:
   - `rbst_*` voices universal AXLA types (`axtu_*`)
   - `rbgt_*` voices Google AXLA types (`axtg_*`)

2. **Regime variable voicings** reference subspecialized types:
   ```asciidoc
   [[rbrv_sigil]]
   // ⟦axl_voices axrg_variable rbst_xname⟧
   {rbrv_sigil}::
   Vessel identifier; basis for directory and ark naming.
   ```

3. **Subdocs use type voicings** in tables (they resolve via parent mappings).

4. **Subdocs have NO anchors** — parent doc owns all `[[anchor]]` definitions.

### Subdocument Structure

- **Overview**: Brief context, explain any bifurcations (e.g., binding vs conjuring)
- **Feature Groups**: Tables with Type column using `rbst_*` or `rbgt_*` references
- **NO "Core Term Definitions"** — parent doc owns these
- **Use NOTE blocks** for group-level applicability rules (not repeated per row)

## Architectural Insight: Annotation-First Structure (₢ASAAG discussion)

During ₢ASAAG we considered whether regimes need `axrs_*` section motifs (like `axs_*` for procedures). Decision: **NO**.

### Annotations ARE the structure

`axvr_*` and `axhr*_` annotations are **self-describing**. A linter can reconstruct the full regime hierarchy from annotations alone — no section headings needed to carry structural metadata. This contrasts with `axs_*` section motifs which embed structure in document headings.

### Deliberate duelling representations

The `axvr_*`/`axhr*_` annotation approach and the `axs_*` section approach are intentionally coexisting. Regimes use annotations exclusively. If the annotation-first pattern proves more efficient and consistent, it may eventually supersede section motifs for other domains too. **Do not create regime section vocabulary** — keep regimes as a clean testbed for the annotation-first approach.

### Parent vs subdoc: complementary layers, not unified

| Layer | Location | Annotations | Purpose |
|---|---|---|---|
| **Schema** | Parent (RBSA) | `axvr_*` | Anchors, types, cardinality, groups. One-sentence definitions. |
| **Operational** | Subdoc (RBSRV, RBRN) | `axhr*_` | Usage context, typical values, gate explanations, interactions. |

These stay separate:
- Parent definitions are terse type references — the "what"
- Subdoc expansions are operational narrative — the "how"
- `include::` connects them under the same heading
- ₢ASAAF reinforces this: move detail OUT of parent INTO subdoc

### ax_ terms are comment-only by design

All AXLA terms (`axt_*`, `axtu_*`, `axtg_*`, `axs_*`, etc.) appear exclusively in `// ⟦...⟧` Strachey bracket annotations or `//axvr_*`/`//axhr*_` prefix-discriminated annotations. They are **never** AsciiDoc attribute references (`:term: <<anchor,Display>>`). This is deliberate — the specs are too dense for AXLA terms to compete with project terms in body text. Project-specific voicings (`rbst_*`, `rbgt_*`, `rbrv_*`, etc.) are the only terms that render.

## Downstream Work

**CRITICAL: After completing this heat**, plan a follow-up heat to **refine AXLA regime vocabulary itself** based on what we learn during:
- RBSA consolidation experience
- Creating BURS and BURC specs
- Studying all RB regimes (BUD_, RBRN, RBRR, etc.)

AXLA will need refinement to capture patterns we discover. That work belongs in a future heat (not in ₣AS). ₢ASAAA (expand-axla-regime-section) was abandoned from this heat for re-slating there.

Also plan: CRR-ConfigRegimeRequirements.adoc retirement (orphaned once RBSA eliminates crg_* vocabulary).

## References

- AXLA: `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc`
- CRR: `lenses/CRR-ConfigRegimeRequirements.adoc` (future retirement)
- RBS: `lenses/RBS-Specification.adoc` (retiring into RBSA)
- RBAGS: `lenses/RBAGS-AdminGoogleSpec.adoc` (retiring into RBSA)
- RBSA: `lenses/RBSA-SpecTop.adoc` (new, created by ₢ASAAH)
- RBRN: `lenses/RBRN-RegimeNameplate.adoc`
- BUS: `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc`
