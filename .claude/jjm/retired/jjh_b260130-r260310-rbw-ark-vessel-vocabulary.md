# Heat Trophy: rbw-ark-vessel-vocabulary

**Firemark:** ₣AR
**Created:** 260130
**Retired:** 260310
**Status:** retired

## Paddock

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

An **ark** is a precise built vessel — one of potentially several differing only by **consecration** (timestamp marking when the vessel was consecrated into existence). It comprises:
- `{vessel}:{consecration}-image` — the container image
- `{vessel}:{consecration}-about` — the build provenance/metadata

**Terminology:**
- **Vessel** — what to build (the design/template, e.g., `rbev-sentry-ubuntu-large`)
- **Consecration** — which build (the timestamp, e.g., `20240115T123456Z`)
- **Ark** — the coherent result pair (vessel:consecration → image + about artifacts)

The ark is NOT a regime (no assignment file). It is a **bridging artifact** that:
- Is **produced** by conjuring/building a vessel
- Is **stored** in the container registry (under `RBRR_GAR_REPOSITORY`)
- Is **consumed/selected** by nameplates via `RBRN_*_VESSEL` and `RBRN_*_CONSECRATION` variables

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

### Type Voicing Pattern for Subdocuments

**Problem**: Included subdocs (like RBSRV) need type annotations, but AXLA types (`axtu_xname`, etc.) don't resolve because parent doc lacks mappings.

**Solution**: Define Recipe Bottle type voicings (`rbst_*`) that subspecialize AXLA motifs:

1. **RBAGS defines type voicings** in "Type Voicings" section:
   ```asciidoc
   [[rbst_xname]]
   // ⟦axl_voices axtu_xname⟧
   {rbst_xname}::
   Cross-context safe identifier. 1-64 characters...
   ```

2. **RBAGS mapping section** includes `rbst_*` mappings so subdocs can reference them.

3. **Regime variable voicings** reference the subspecialized type:
   ```asciidoc
   [[rbrv_sigil]]
   // ⟦axl_voices axrg_variable rbst_xname⟧
   {rbrv_sigil}::
   Vessel identifier...
   ```

4. **Subdocs use `rbst_*` types** in tables and prose (they resolve via parent mappings).

5. **Subdocs have NO anchors** — parent doc owns all `[[anchor]]` definitions. Subdocs expand with detail (tables, constraints) but never re-define terms.

**Voicing chain**: AXLA motif → rbst_* type → regime variable

**Dimensions**: For now, use prose ("Optional", "list of..."). Future pace (₢ARAAK) will extend AXLA to properly handle regime variable dimensions.

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
- `Tools/rbw/rbgc_Constants.sh` — GCP constants (has `RBGC_ARK_SUFFIX_IMAGE`, `RBGC_ARK_SUFFIX_ABOUT`, `RBGC_GAR_HOST_SUFFIX`)
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

## Paces

### rbrn-restate-boolean-enums (₢ARAAR) [complete]

**[260206-0941] complete**

Restate RBRN boolean variable pairs as enumerations.

## Problem

RBRN uses boolean pairs (ENABLED + GLOBAL) that are really 3-valued enumerations
with an impossible state. This makes axhr gating awkward (compound stacked gates)
and the regime itself less precise.

## Changes

### Uplink DNS: 2 booleans → 1 enum
- Remove: RBRN_UPLINK_DNS_ENABLED, RBRN_UPLINK_DNS_GLOBAL
- Add: RBRN_UPLINK_DNS_MODE = disabled | global | allowlist
- RBRN_UPLINK_ALLOWED_DOMAINS: required when dns_mode=allowlist (unchanged)

### Uplink Access: 2 booleans → 1 enum  
- Remove: RBRN_UPLINK_ACCESS_ENABLED, RBRN_UPLINK_ACCESS_GLOBAL
- Add: RBRN_UPLINK_ACCESS_MODE = disabled | global | allowlist
- RBRN_UPLINK_ALLOWED_CIDRS: required when access_mode=allowlist (unchanged)

### Entry: 1 boolean → 1 enum
- Remove: RBRN_ENTRY_ENABLED (crg_atom_bool)
- Add: RBRN_ENTRY_MODE = disabled | enabled
- Gated variables unchanged

## Files to update
1. lenses/RBRN-RegimeNameplate.adoc — the spec itself
2. Tools/rbw/rbrn_*.env — all nameplate assignment files (update variable names and values)
3. Tools/rbw/ bash scripts that read these variables — search for RBRN_ENTRY_ENABLED,
   RBRN_UPLINK_DNS_ENABLED, RBRN_UPLINK_DNS_GLOBAL, RBRN_UPLINK_ACCESS_ENABLED,
   RBRN_UPLINK_ACCESS_GLOBAL and update to new enum-based logic

## Verification
- All rbrn_*.env files use new variable names with valid enum values
- Bash scripts that consume RBRN variables handle new enum values correctly
- No references to removed variable names remain in codebase

**[260206-0930] rough**

Restate RBRN boolean variable pairs as enumerations.

## Problem

RBRN uses boolean pairs (ENABLED + GLOBAL) that are really 3-valued enumerations
with an impossible state. This makes axhr gating awkward (compound stacked gates)
and the regime itself less precise.

## Changes

### Uplink DNS: 2 booleans → 1 enum
- Remove: RBRN_UPLINK_DNS_ENABLED, RBRN_UPLINK_DNS_GLOBAL
- Add: RBRN_UPLINK_DNS_MODE = disabled | global | allowlist
- RBRN_UPLINK_ALLOWED_DOMAINS: required when dns_mode=allowlist (unchanged)

### Uplink Access: 2 booleans → 1 enum  
- Remove: RBRN_UPLINK_ACCESS_ENABLED, RBRN_UPLINK_ACCESS_GLOBAL
- Add: RBRN_UPLINK_ACCESS_MODE = disabled | global | allowlist
- RBRN_UPLINK_ALLOWED_CIDRS: required when access_mode=allowlist (unchanged)

### Entry: 1 boolean → 1 enum
- Remove: RBRN_ENTRY_ENABLED (crg_atom_bool)
- Add: RBRN_ENTRY_MODE = disabled | enabled
- Gated variables unchanged

## Files to update
1. lenses/RBRN-RegimeNameplate.adoc — the spec itself
2. Tools/rbw/rbrn_*.env — all nameplate assignment files (update variable names and values)
3. Tools/rbw/ bash scripts that read these variables — search for RBRN_ENTRY_ENABLED,
   RBRN_UPLINK_DNS_ENABLED, RBRN_UPLINK_DNS_GLOBAL, RBRN_UPLINK_ACCESS_ENABLED,
   RBRN_UPLINK_ACCESS_GLOBAL and update to new enum-based logic

## Verification
- All rbrn_*.env files use new variable names with valid enum values
- Bash scripts that consume RBRN variables handle new enum values correctly
- No references to removed variable names remain in codebase

### rbrn-axhr-voicing (₢ARAAS) [complete]

**[260206-2017] complete**

Apply axvr_*/axhr*_ regime annotation patterns to RBRN.

## Purpose

Document RBRN using the same voicing and hierarchy patterns deployed for RBRV in ₢ARAAP.
This pace depends on ₢ARAAR (rbrn-restate-boolean-enums) completing first so the
variable structure is finalized.

## Changes

### RBAGS parent document — mint rbrn_* voicings
- rbrn_regime (axvr_regime axf_bash)
- rbrn_moniker, rbrn_description, rbrn_runtime (axvr_variable)
- rbrn_sentry_vessel, rbrn_sentry_consecration, rbrn_bottle_vessel, rbrn_bottle_consecration
- rbrn_entry_mode + enum values (axvr_variable axt_enumeration)
- rbrn_uplink_dns_mode, rbrn_uplink_access_mode + enum values
- rbrn_uplink_port_min, rbrn_uplink_allowed_cidrs, rbrn_uplink_allowed_domains
- rbrn_enclave_* variables
- rbrn_volume_mounts
- Group voicings: rbrn_group_ark_reference, rbrn_group_entry, rbrn_group_access_allowlist,
  rbrn_group_dns_allowlist (axvr_group, conditional where gated)
- rbst_* type voicings as needed for RBRN's types

### RBRN subdocument — apply axhr hierarchy markers
- axhrb_regime → rbrn_regime
- Ungrouped variables via axhrv_variable (moniker, description, runtime, enclave vars, 
  uplink ungrouped vars, volume_mounts)
- axhrgb_group for ark_reference (unconditional), entry (conditional), 
  access_allowlist (conditional), dns_allowlist (conditional)
- axhrgc_gate for conditional groups (entry_mode=enabled, access_mode=allowlist, dns_mode=allowlist)
- axhrgv_variable for grouped variables

## Inputs
- Assessment memo: Memos/memo-20260206-rbrn-regime-fit-assessment.md
- RBSRV exemplar (completed in ₢ARAAP)
- AXLA with axvr_*/axhr*_ terms (from ₢ARAAN)

## Verification
- All rbrn_* attribute references resolve
- axhr hierarchy in RBRN subdoc matches axvr voicings in RBAGS parent
- Cross-document validation rules from AXLA are satisfied

**[260206-0930] rough**

Apply axvr_*/axhr*_ regime annotation patterns to RBRN.

## Purpose

Document RBRN using the same voicing and hierarchy patterns deployed for RBRV in ₢ARAAP.
This pace depends on ₢ARAAR (rbrn-restate-boolean-enums) completing first so the
variable structure is finalized.

## Changes

### RBAGS parent document — mint rbrn_* voicings
- rbrn_regime (axvr_regime axf_bash)
- rbrn_moniker, rbrn_description, rbrn_runtime (axvr_variable)
- rbrn_sentry_vessel, rbrn_sentry_consecration, rbrn_bottle_vessel, rbrn_bottle_consecration
- rbrn_entry_mode + enum values (axvr_variable axt_enumeration)
- rbrn_uplink_dns_mode, rbrn_uplink_access_mode + enum values
- rbrn_uplink_port_min, rbrn_uplink_allowed_cidrs, rbrn_uplink_allowed_domains
- rbrn_enclave_* variables
- rbrn_volume_mounts
- Group voicings: rbrn_group_ark_reference, rbrn_group_entry, rbrn_group_access_allowlist,
  rbrn_group_dns_allowlist (axvr_group, conditional where gated)
- rbst_* type voicings as needed for RBRN's types

### RBRN subdocument — apply axhr hierarchy markers
- axhrb_regime → rbrn_regime
- Ungrouped variables via axhrv_variable (moniker, description, runtime, enclave vars, 
  uplink ungrouped vars, volume_mounts)
- axhrgb_group for ark_reference (unconditional), entry (conditional), 
  access_allowlist (conditional), dns_allowlist (conditional)
- axhrgc_gate for conditional groups (entry_mode=enabled, access_mode=allowlist, dns_mode=allowlist)
- axhrgv_variable for grouped variables

## Inputs
- Assessment memo: Memos/memo-20260206-rbrn-regime-fit-assessment.md
- RBSRV exemplar (completed in ₢ARAAP)
- AXLA with axvr_*/axhr*_ terms (from ₢ARAAN)

## Verification
- All rbrn_* attribute references resolve
- axhr hierarchy in RBRN subdoc matches axvr voicings in RBAGS parent
- Cross-document validation rules from AXLA are satisfied

### find-rbrv-cli-kindled-usage (₢ARAAI) [complete]

**[260131-1222] complete**

Search codebase for RBRV_CLI_KINDLED constant usage to verify if it's misnamed and belongs to RBRV or another subsystem. Determine correct naming and scope.

**[260131-1212] rough**

Search codebase for RBRV_CLI_KINDLED constant usage to verify if it's misnamed and belongs to RBRV or another subsystem. Determine correct naming and scope.

### add-ark-suffix-constants-rbgc (₢ARAAA) [complete]

**[260131-1202] complete**

Add RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT constants to rbgc_Constants.sh.

## Purpose
Single point of maintenance for ark artifact suffixes used across the codebase.

## Implementation
- Add constants in the zrbgc_kindle() function (Tools/rbw/rbgc_Constants.sh around line 120)
- Values: "-image" and "-about"

## Follow-on work
Check these files for hardcoded suffixes that should eventually use these constants:
- Tools/rbw/rbob_bottle.sh — bottle operations
- Tools/rbw/rbi_Image.sh — image operations  
- Tools/rbw/rbf_Foundry.sh — build operations
Note any usage gaps for later paces; do NOT update these files in this pace.

## Verification
- Build succeeds (no syntax errors)
- Constants are accessible after kindle

**[260130-0802] rough**

Add RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT constants to rbgc_Constants.sh.

## Purpose
Single point of maintenance for ark artifact suffixes used across the codebase.

## Implementation
- Add constants in the zrbgc_kindle() function (Tools/rbw/rbgc_Constants.sh around line 120)
- Values: "-image" and "-about"

## Follow-on work
Check these files for hardcoded suffixes that should eventually use these constants:
- Tools/rbw/rbob_bottle.sh — bottle operations
- Tools/rbw/rbi_Image.sh — image operations  
- Tools/rbw/rbf_Foundry.sh — build operations
Note any usage gaps for later paces; do NOT update these files in this pace.

## Verification
- Build succeeds (no syntax errors)
- Constants are accessible after kindle

**[260130-0740] rough**

Add RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT constants to rbgc_Constants.sh.

## Purpose
Single point of maintenance for ark artifact suffixes used across the codebase.

## Implementation
- Add constants in the zrbgc_kindle() function
- Values: "-image" and "-about"

## Follow-on work
- These constants need to be plumbed into the cloud build script as parameters (future pace)
- Check bottle service download scripts (rbob_bottle.sh, rbi_Image.sh) for hardcoded suffixes that should use these constants
- Note any usage gaps for later paces

## Verification
- Build succeeds
- Constants are exported and accessible after kindle

### add-rbrv-vocab-to-rbags (₢ARAAB) [complete]

**[260131-1218] complete**

Add vessel regime variables to RBAGS mapping section using modern MCM patterns.

## Purpose
Enable {rbrv_sigil}, {rbrv_conjure_dockerfile}, etc. as linked terms in RBAGS for discussing ark production.

## Implementation
- Add mapping entries using modern anchor pattern (NO term_ prefix):
  :rbrv_sigil:              <<rbrv_sigil,RBRV_SIGIL>>
- Add definition section for RBRV variables
- Use AXLA motif references (axrg_variable, axf_bash) in annotations

## Exploration required
- Read documents included by RBAGS (grep for include:: in RBAGS):
  - RBSAA-ark_abjure.adoc
  - RBSOB-oci_layout_bridge.adoc
  - Others as found
- Identify which should reference vessel vocabulary
- Document findings for incorporation in those files (separate paces if needed)

## Note
Do NOT add RBGC_ARK_SUFFIX_* constants to RBAGS — it's dense enough already. Those constants are implementation detail.

## Pattern reference
See AXLA-Lexicon.adoc for axrg_* regime motifs and annotation patterns.

**[260130-0803] rough**

Add vessel regime variables to RBAGS mapping section using modern MCM patterns.

## Purpose
Enable {rbrv_sigil}, {rbrv_conjure_dockerfile}, etc. as linked terms in RBAGS for discussing ark production.

## Implementation
- Add mapping entries using modern anchor pattern (NO term_ prefix):
  :rbrv_sigil:              <<rbrv_sigil,RBRV_SIGIL>>
- Add definition section for RBRV variables
- Use AXLA motif references (axrg_variable, axf_bash) in annotations

## Exploration required
- Read documents included by RBAGS (grep for include:: in RBAGS):
  - RBSAA-ark_abjure.adoc
  - RBSOB-oci_layout_bridge.adoc
  - Others as found
- Identify which should reference vessel vocabulary
- Document findings for incorporation in those files (separate paces if needed)

## Note
Do NOT add RBGC_ARK_SUFFIX_* constants to RBAGS — it's dense enough already. Those constants are implementation detail.

## Pattern reference
See AXLA-Lexicon.adoc for axrg_* regime motifs and annotation patterns.

**[260130-0740] rough**

Add vessel regime variables to RBAGS mapping section using modern MCM patterns.

## Purpose
Enable {rbrv_sigil}, {rbrv_conjure_dockerfile}, etc. as linked terms in RBAGS for discussing ark production.

## Implementation
- Add mapping entries using modern anchor pattern (NO term_ prefix):
  :rbrv_sigil:              <<rbrv_sigil,RBRV_SIGIL>>
- Add definition section for RBRV variables
- Use AXLA motif references (axrg_variable, axf_bash) in annotations

## Exploration required
- Read documents included by RBAGS (RBSAA, RBSOB, etc.)
- Identify which should use vessel vocabulary
- Document findings for incorporation in those files (separate paces if needed)

## Pattern reference
See AXLA-Lexicon.adoc for axrg_* regime motifs and annotation patterns.

### create-rbrv-regime-spec (₢ARAAC) [complete]

**[260131-1247] complete**

Create RBRV-RegimeVessel.adoc formal specification for vessel configuration.

## Purpose
Document the vessel regime variables (RBRV_SIGIL, RBRV_CONJURE_*, etc.) in a standalone AsciiDoc spec.

## Approach
- Create as standalone file: lenses/RBRV-RegimeVessel.adoc
- Follow RBRN-RegimeNameplate.adoc pattern for structure
- Use modern MCM anchor patterns (no term_ prefix)
- Reference AXLA motifs (axrg_regime, axrg_variable, axf_bash)

## Critical requirement: Identical attribute mappings
The attribute mappings in RBRV-RegimeVessel.adoc MUST be identical to those added to RBAGS in pace ARAAB. Same anchors, same terms, same display text. This is intentional duplication — both documents describe exactly the same concepts.

Future heat will determine how to consolidate these (include::, AsciiDoc tags, or other mechanism). For now, keep them in sync manually.

## Variables to document
- RBRV_SIGIL — vessel identifier (anchors ark identity)
- RBRV_DESCRIPTION — human-readable description
- RBRV_CONJURE_DOCKERFILE — path to Dockerfile
- RBRV_CONJURE_BLDCONTEXT — build context directory
- RBRV_CONJURE_PLATFORMS — target platforms (space-separated)
- RBRV_CONJURE_BINFMT_POLICY — binfmt policy for cross-platform builds

**[260130-0803] rough**

Create RBRV-RegimeVessel.adoc formal specification for vessel configuration.

## Purpose
Document the vessel regime variables (RBRV_SIGIL, RBRV_CONJURE_*, etc.) in a standalone AsciiDoc spec.

## Approach
- Create as standalone file: lenses/RBRV-RegimeVessel.adoc
- Follow RBRN-RegimeNameplate.adoc pattern for structure
- Use modern MCM anchor patterns (no term_ prefix)
- Reference AXLA motifs (axrg_regime, axrg_variable, axf_bash)

## Critical requirement: Identical attribute mappings
The attribute mappings in RBRV-RegimeVessel.adoc MUST be identical to those added to RBAGS in pace ARAAB. Same anchors, same terms, same display text. This is intentional duplication — both documents describe exactly the same concepts.

Future heat will determine how to consolidate these (include::, AsciiDoc tags, or other mechanism). For now, keep them in sync manually.

## Variables to document
- RBRV_SIGIL — vessel identifier (anchors ark identity)
- RBRV_DESCRIPTION — human-readable description
- RBRV_CONJURE_DOCKERFILE — path to Dockerfile
- RBRV_CONJURE_BLDCONTEXT — build context directory
- RBRV_CONJURE_PLATFORMS — target platforms (space-separated)
- RBRV_CONJURE_BINFMT_POLICY — binfmt policy for cross-platform builds

**[260130-0740] rough**

Create RBRV-RegimeVessel.adoc formal specification for vessel configuration.

## Purpose
Document the vessel regime variables (RBRV_SIGIL, RBRV_CONJURE_*, etc.) in a standalone AsciiDoc spec.

## Approach
- Create as standalone file in lenses/ directory
- Follow RBRN-RegimeNameplate.adoc pattern for structure
- Use modern MCM anchor patterns (no term_ prefix)
- Reference AXLA motifs (axrg_regime, axrg_variable, axf_bash)
- Share mappings with RBAGS non-incidentally (same anchors, same terms)

## Design note
Regime specs and overarching specs (RBS, RBAGS) describing the same concepts is not fully resolved. For now: standalone files that happen to share mappings because they describe exactly the same thing. Future wisdom will guide grafting them together.

## Variables to document
- RBRV_SIGIL — vessel identifier
- RBRV_DESCRIPTION — human-readable description
- RBRV_CONJURE_DOCKERFILE — path to Dockerfile
- RBRV_CONJURE_BLDCONTEXT — build context directory
- RBRV_CONJURE_PLATFORMS — target platforms
- RBRV_CONJURE_BINFMT_POLICY — binfmt policy

### resolve-axla-detail-doc-pattern (₢ARAAJ) [complete]

**[260131-2357] complete**

Resolve AXLA type reference pattern for included detail documents.

## Context
RBSRV-RegimeVessel.adoc uses `{axtu_xname}`, `{axrg_variable}` etc. in definition text and table cells, but these don't resolve because RBAGS lacks AXLA mappings. This is a design gap.

## Questions to resolve
1. Should detail docs (like RBSRV) reference AXLA types at all?
2. If yes: add AXLA mappings to parent doc, or create new "subconstraint" pattern?
3. If no: use plain text and accept loss of semantic linkage?

## Acceptance criteria
- Decision documented in paddock
- RBSRV fixed to match chosen pattern
- Pattern guidance added for future regime detail specs

**[260131-1255] rough**

Resolve AXLA type reference pattern for included detail documents.

## Context
RBSRV-RegimeVessel.adoc uses `{axtu_xname}`, `{axrg_variable}` etc. in definition text and table cells, but these don't resolve because RBAGS lacks AXLA mappings. This is a design gap.

## Questions to resolve
1. Should detail docs (like RBSRV) reference AXLA types at all?
2. If yes: add AXLA mappings to parent doc, or create new "subconstraint" pattern?
3. If no: use plain text and accept loss of semantic linkage?

## Acceptance criteria
- Decision documented in paddock
- RBSRV fixed to match chosen pattern
- Pattern guidance added for future regime detail specs

### evolve-rbrn-ark-reference (₢ARAAD) [complete]

**[260201-2031] complete**

Replace RBRN image tag pairs with single ark reference pattern.

## Current state
Nameplate uses separate variables:
- RBRN_SENTRY_MONIKER + RBRN_SENTRY_IMAGE_TAG
- RBRN_BOTTLE_MONIKER + RBRN_BOTTLE_IMAGE_TAG

## Target state
Single ark reference:
- RBRN_SENTRY_ARK = moniker:ark_stamp
- RBRN_BOTTLE_ARK = moniker:ark_stamp

Runtime code appends -image or -about suffix using RBGC constants.

## Breaking change — burn the bridges
This is explicitly a breaking change. There is NO backwards compatibility path. Old nameplate files will not work. This is intentional — we embrace the new nameplate nature fully.

## Files to update
- lenses/RBRN-RegimeNameplate.adoc — update spec to define new variables, remove old
- Tools/rbw/rbrn_*.env — update all assignment files to new pattern
- Grep for RBRN_.*_IMAGE_TAG and RBRN_.*_MONIKER to find consuming code

## Verification
- Nameplate validation still works (if validation exists)
- Document any code that needs updating for follow-on paces

**[260130-0803] rough**

Replace RBRN image tag pairs with single ark reference pattern.

## Current state
Nameplate uses separate variables:
- RBRN_SENTRY_MONIKER + RBRN_SENTRY_IMAGE_TAG
- RBRN_BOTTLE_MONIKER + RBRN_BOTTLE_IMAGE_TAG

## Target state
Single ark reference:
- RBRN_SENTRY_ARK = moniker:ark_stamp
- RBRN_BOTTLE_ARK = moniker:ark_stamp

Runtime code appends -image or -about suffix using RBGC constants.

## Breaking change — burn the bridges
This is explicitly a breaking change. There is NO backwards compatibility path. Old nameplate files will not work. This is intentional — we embrace the new nameplate nature fully.

## Files to update
- lenses/RBRN-RegimeNameplate.adoc — update spec to define new variables, remove old
- Tools/rbw/rbrn_*.env — update all assignment files to new pattern
- Grep for RBRN_.*_IMAGE_TAG and RBRN_.*_MONIKER to find consuming code

## Verification
- Nameplate validation still works (if validation exists)
- Document any code that needs updating for follow-on paces

**[260130-0740] rough**

Replace RBRN image tag pairs with single ark reference pattern.

## Current state
Nameplate uses separate variables:
- RBRN_SENTRY_MONIKER + RBRN_SENTRY_IMAGE_TAG
- RBRN_BOTTLE_MONIKER + RBRN_BOTTLE_IMAGE_TAG

## Target state
Single ark reference:
- RBRN_SENTRY_ARK = moniker:ark_stamp
- RBRN_BOTTLE_ARK = moniker:ark_stamp

Runtime code appends -image or -about suffix using RBGC constants.

## Files to update
- lenses/RBRN-RegimeNameplate.adoc — spec
- Tools/rbw/rbrn_*.env — assignment files
- Any code consuming these variables

## Verification
- Nameplate validation still works
- Bottle service startup resolves ark to image correctly

### modernize-rbs-anchors (₢ARAAE) [complete]

**[260201-2033] complete**

Update RBS-Specification.adoc to remove term_ prefix from anchors.

## Current state
RBS uses old anchor pattern:
  :rbrn_moniker:  <<term_rbrn_moniker,RBRN_MONIKER>>

## Target state
Modern MCM pattern (matches AXLA):
  :rbrn_moniker:  <<rbrn_moniker,RBRN_MONIKER>>

## Scope
- Update all mapping entries to remove term_ prefix from anchor references
- Update all anchor definitions [[term_xyz]] → [[xyz]]
- Verify all internal cross-references still resolve

## Cross-document references
No concern here — when RBS was originally written, there were no cross-document references to its anchors. The term_ prefix was local convention only.

## Verification
- Document renders correctly (if tooling available)
- All links resolve (no broken anchors) — grep for [[term_ to ensure none remain
- Pattern matches MCM-MetaConceptModel.adoc and AXLA-Lexicon.adoc

**[260130-0803] rough**

Update RBS-Specification.adoc to remove term_ prefix from anchors.

## Current state
RBS uses old anchor pattern:
  :rbrn_moniker:  <<term_rbrn_moniker,RBRN_MONIKER>>

## Target state
Modern MCM pattern (matches AXLA):
  :rbrn_moniker:  <<rbrn_moniker,RBRN_MONIKER>>

## Scope
- Update all mapping entries to remove term_ prefix from anchor references
- Update all anchor definitions [[term_xyz]] → [[xyz]]
- Verify all internal cross-references still resolve

## Cross-document references
No concern here — when RBS was originally written, there were no cross-document references to its anchors. The term_ prefix was local convention only.

## Verification
- Document renders correctly (if tooling available)
- All links resolve (no broken anchors) — grep for [[term_ to ensure none remain
- Pattern matches MCM-MetaConceptModel.adoc and AXLA-Lexicon.adoc

**[260130-0740] rough**

Update RBS-Specification.adoc to remove term_ prefix from anchors.

## Current state
RBS uses old anchor pattern:
  :rbrn_moniker:  <<term_rbrn_moniker,RBRN_MONIKER>>

## Target state
Modern MCM pattern (matches AXLA):
  :rbrn_moniker:  <<rbrn_moniker,RBRN_MONIKER>>

## Scope
- Update all mapping entries to remove term_ prefix
- Update all anchor definitions [[term_xyz]] → [[xyz]]
- Verify all internal cross-references still resolve

## Verification
- Document renders correctly
- All links resolve (no broken anchors)
- Pattern matches MCM-MetaConceptModel.adoc and AXLA-Lexicon.adoc

### add-cosmology-intro-getting-started (₢ARAAG) [complete]

**[260201-2042] complete**

Introduce Recipe Bottle metaphor and trust model in Getting Started guide.

## Purpose
Provide conceptual grounding before the procedural content. Help readers understand WHY before HOW.

## Location
lenses/RBSGS-GettingStarted.adoc — add as opening section before "Depots and Roles"

## Content approach
This is narrative introduction — complement procedural content that follows. Use linked terms from RBAGS vocabulary.

## Content to cover
- Why "Recipe Bottle" — containing potentially dangerous workloads safely
- Vessel → Ark → Nameplate flow (the build-to-deploy lifecycle)
- Trust model: verify the ark's -about before deploying the -image
- Role tags in vessel naming (sentry, bottle) — semantic convention not type system

## Term usage — decide during pace execution
The intro material should NOT use specific regime variables directly (e.g., {rbrv_sigil}). Instead, it should identify concepts like "vessels" exist as configuration.

Before writing:
1. Review RBAGS mapping section to see what terms exist
2. Review RBSGS current content for tone and level
3. Decide which linked terms are appropriate for conceptual intro vs. procedural detail
4. Note that RBAGS has "regime prefixes" section — review for vocabulary guidance

The future implementer will make term decisions after reading relevant documents with fresh eyes.

## Dependencies
This pace depends on vocabulary being established by earlier paces in this heat (ARAAB, ARAAC).

**[260130-0804] rough**

Introduce Recipe Bottle metaphor and trust model in Getting Started guide.

## Purpose
Provide conceptual grounding before the procedural content. Help readers understand WHY before HOW.

## Location
lenses/RBSGS-GettingStarted.adoc — add as opening section before "Depots and Roles"

## Content approach
This is narrative introduction — complement procedural content that follows. Use linked terms from RBAGS vocabulary.

## Content to cover
- Why "Recipe Bottle" — containing potentially dangerous workloads safely
- Vessel → Ark → Nameplate flow (the build-to-deploy lifecycle)
- Trust model: verify the ark's -about before deploying the -image
- Role tags in vessel naming (sentry, bottle) — semantic convention not type system

## Term usage — decide during pace execution
The intro material should NOT use specific regime variables directly (e.g., {rbrv_sigil}). Instead, it should identify concepts like "vessels" exist as configuration.

Before writing:
1. Review RBAGS mapping section to see what terms exist
2. Review RBSGS current content for tone and level
3. Decide which linked terms are appropriate for conceptual intro vs. procedural detail
4. Note that RBAGS has "regime prefixes" section — review for vocabulary guidance

The future implementer will make term decisions after reading relevant documents with fresh eyes.

## Dependencies
This pace depends on vocabulary being established by earlier paces in this heat (ARAAB, ARAAC).

**[260130-0741] rough**

Introduce Recipe Bottle metaphor and trust model in Getting Started guide.

## Purpose
Provide conceptual grounding before the procedural content. Help readers understand WHY before HOW.

## Location
lenses/RBSGS-GettingStarted.adoc — add as opening section before "Depots and Roles"

## Content to cover
- Why "Recipe Bottle" — containing potentially dangerous workloads safely
- Vessel → Ark → Nameplate flow (reference the vocabulary now established)
- Trust model: verify the ark's -about before deploying the -image
- Role tags in vessel naming (sentry, bottle) — semantic convention not type system

## Dependencies
This pace depends on vocabulary being established by earlier paces in this heat.

## Style
Narrative introduction — complement the procedural content that follows. Use linked terms from RBAGS vocabulary.

### create-burs-regime-spec (₢ARAAH) [abandoned]

**[260201-2059] abandoned**

Superseded by ₣AT regime consolidation work:
- ₢ATAAB (rename-bus-to-busa) 
- ₢ATAAC (expand-busa-regime-vocabulary)

The vision evolved: instead of standalone BURS spec, we expand BUSA to be the complete BUK concept model with integrated regime vocabulary.

**[260130-0804] rough**

Create BURS-BashUtilityRegimeSpec.adoc — modernized regime definition for bash config systems.

## Purpose
Replace the old CRR-ConfigRegimeRequirements.adoc (makefile-centric) with a modern bash-focused regime specification. Full excision of makefile patterns — that era is gone.

## Location
Tools/buk/vov_veiled/BURS-BashUtilityRegimeSpec.adoc (veiled, travels with BUK)

## Content approach
- Read MCM-MetaConceptModel.adoc for document patterns
- Read AXLA-Lexicon.adoc for regime/format motifs (axrg_*, axf_*)
- Study BURC as the most recent regime style — use as pattern

## Key changes from CRR
- Remove all makefile patterns (include, :=, etc.)
- Focus on bash .env assignment files ({axf_bash})
- Add AXLA voicings for regime concepts
- Modern MCM anchor patterns (no term_ prefix)
- Reference axrg_regime, axrg_variable, axrg_assignment, axrg_prefix motifs

## Scope control
This is the formal specification document only. It does NOT include:
- Updating existing regime files to match
- Creating validator/renderer scripts
- Migrating other regimes

Those are future work (potentially separate heat).

## Verification
- Document follows MCM patterns
- AXLA motifs are correctly referenced in annotations
- Can serve as authoritative reference for regime authors

### axla-regime-dimension-voicings (₢ARAAK) [complete]

**[260203-1832] complete**

Extend AXLA to properly handle regime variable dimensions.

## Context
RBRV_CONJURE_PLATFORMS is a single assignment containing a space-delimited list. This differs from "repeated" (multiple separate assignments). Current AXLA has `axd_repeated` but regime variables need:
- `axd_optional` — variable may be absent from regime (works for conjure/bind bifurcation)
- `axd_list` (new?) — single assignment containing delimited list of values

## Design Musing (from ₢ARAAJ discussion)

The semantic gap: storage is a string, but interpretation is a list. AXLA needs to express this duality.

**Option A: New dimension `axd_list`**
```
// ⟦axl_voices axtu_string axd_list⟧
```
Extends dimension vocabulary. Parallels `axd_optional`, `axd_repeated`. Says "single assignment, multiple values." Cleanest option — orthogonal to type.

**Option B: Compose with `axt_array`**
```
// ⟦axl_voices axt_array axtu_string⟧
```
Uses existing AXLA motif. Awkward — two type motifs feels wrong.

**Option C: Format modifier**
```
// ⟦axl_voices axtu_string axf_delimited⟧
```
`axf_` already exists for formats. Add `axf_delimited` or `axf_space_list`.

**Current leaning:** Option A (`axd_list`) — think carefully during this pace.

## Work required
1. Review AXLA dimension terms (`axd_*`) for regime variable applicability
2. Evaluate options A/B/C above; determine best pattern
3. Add appropriate voicings/motifs to AXLA
4. Update RBAGS type voicings to use the new dimension patterns
5. Update RBSRV to use proper dimension references instead of prose

## Acceptance criteria
- AXLA has clear guidance on regime variable dimensions
- `optional` and `list` patterns are properly distinguished from `repeated`
- RBSRV dimensions render correctly via RBAGS mappings

**[260131-2247] rough**

Extend AXLA to properly handle regime variable dimensions.

## Context
RBRV_CONJURE_PLATFORMS is a single assignment containing a space-delimited list. This differs from "repeated" (multiple separate assignments). Current AXLA has `axd_repeated` but regime variables need:
- `axd_optional` — variable may be absent from regime (works for conjure/bind bifurcation)
- `axd_list` (new?) — single assignment containing delimited list of values

## Design Musing (from ₢ARAAJ discussion)

The semantic gap: storage is a string, but interpretation is a list. AXLA needs to express this duality.

**Option A: New dimension `axd_list`**
```
// ⟦axl_voices axtu_string axd_list⟧
```
Extends dimension vocabulary. Parallels `axd_optional`, `axd_repeated`. Says "single assignment, multiple values." Cleanest option — orthogonal to type.

**Option B: Compose with `axt_array`**
```
// ⟦axl_voices axt_array axtu_string⟧
```
Uses existing AXLA motif. Awkward — two type motifs feels wrong.

**Option C: Format modifier**
```
// ⟦axl_voices axtu_string axf_delimited⟧
```
`axf_` already exists for formats. Add `axf_delimited` or `axf_space_list`.

**Current leaning:** Option A (`axd_list`) — think carefully during this pace.

## Work required
1. Review AXLA dimension terms (`axd_*`) for regime variable applicability
2. Evaluate options A/B/C above; determine best pattern
3. Add appropriate voicings/motifs to AXLA
4. Update RBAGS type voicings to use the new dimension patterns
5. Update RBSRV to use proper dimension references instead of prose

## Acceptance criteria
- AXLA has clear guidance on regime variable dimensions
- `optional` and `list` patterns are properly distinguished from `repeated`
- RBSRV dimensions render correctly via RBAGS mappings

**[260131-2231] rough**

Extend AXLA to properly handle regime variable dimensions.

## Context
RBRV_CONJURE_PLATFORMS is a single assignment containing a space-delimited list. This differs from "repeated" (multiple separate assignments). Current AXLA has `axd_repeated` but regime variables need:
- `axd_optional` — variable may be absent from regime (works for conjure/bind bifurcation)
- `axd_list` (new?) — single assignment containing delimited list of values

## Work required
1. Review AXLA dimension terms (`axd_*`) for regime variable applicability
2. Determine if `axd_list` or similar is needed for multi-valued single assignments
3. Add appropriate voicings/motifs to AXLA
4. Update RBAGS type voicings to use the new dimension patterns
5. Update RBSRV to use proper dimension references instead of prose

## Acceptance criteria
- AXLA has clear guidance on regime variable dimensions
- `optional` and `list` patterns are properly distinguished from `repeated`
- RBSRV dimensions render correctly via RBAGS mappings

### simplify-rbst-definitions (₢ARAAL) [complete]

**[260203-1837] complete**

Simplify rbst_* type definitions to avoid duplicating AXLA.

## Context
The rbst_* type voicings in RBAGS currently repeat constraints that are already defined in AXLA motifs (e.g., axtu_xname already says "must start with letter, may contain..."). The voicing should only add RB-specific subspecialization, not repeat the base motif.

## Work required
1. Review each rbst_* definition in RBAGS Type Voicings section
2. Remove prose that duplicates AXLA motif definitions
3. Keep only RB-specific constraints (e.g., "1-64 characters" for sigils, specific enum values)
4. Ensure definitions still read coherently

## Acceptance criteria
- rbst_* definitions are concise
- No duplication of AXLA motif prose
- RB-specific constraints are preserved

**[260131-2235] rough**

Simplify rbst_* type definitions to avoid duplicating AXLA.

## Context
The rbst_* type voicings in RBAGS currently repeat constraints that are already defined in AXLA motifs (e.g., axtu_xname already says "must start with letter, may contain..."). The voicing should only add RB-specific subspecialization, not repeat the base motif.

## Work required
1. Review each rbst_* definition in RBAGS Type Voicings section
2. Remove prose that duplicates AXLA motif definitions
3. Keep only RB-specific constraints (e.g., "1-64 characters" for sigils, specific enum values)
4. Ensure definitions still read coherently

## Acceptance criteria
- rbst_* definitions are concise
- No duplication of AXLA motif prose
- RB-specific constraints are preserved

### axla-regime-annotations (₢ARAAN) [complete]

**[260206-0901] complete**

Add AXLA terms for regime definition-site voicing (axvr_*) and regime subdoc hierarchy markers (axhr*_).

## Background

MCM now supports prefix-discriminated annotations: comment lines starting with `//ax` (no
space after `//`) are recognized as AXLA annotations. This replaces Strachey bracket syntax
for the new regime annotation families. MCM change already committed.

This pace adds two new annotation grammar families using the prefix-discriminated form:
- axvr_* (definition-site, anchor->annotation->definition pattern)
- axhr*_ (standalone hierarchy markers with attrref lookahead)

## Surface Syntax

Definition-site (between anchor and definition in parent doc):
```
[[rbrv_sigil]]
//axvr_variable axd_required rbst_xname
{rbrv_sigil}::
```

Standalone hierarchy (in subdoc, not between anchor and definition):
```
//axhrb_regime
{rbrv_regime}

//axhrgv_variable
{rbrv_sigil}
```

## axvr_* Definition-Site Terms

New voicing annotations for regime structures in parent documents. All share the constraint
that anchor must match first attrref in definition.

### axvr_regime
- Voices a regime definition
- Dimensions: format (axf_bash, axf_json, etc.)
- No additional lookahead constraints beyond standard voicing

### axvr_variable
- Voices a regime variable definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Ties each variable unequivocally to its regime
- Dimensions: axd_required/axd_optional, type motif

### axvr_group
- Voices a regime group definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Optional dimension: axd_conditional (group has activation gate in subdoc)

## axhr*_ Standalone Hierarchy Terms

Standalone markers using prefix-discriminated form. These appear in regime subdocuments,
NOT between anchor and definition. Each marker reads ahead N attribute references from
following text.

### Markers and lookahead arities

| Marker           | Lookahead | Reads                                          |
|------------------|-----------|-------------------------------------------------|
| axhrb_regime     | 1 attrref | The regime this subdoc describes                |
| axhrv_variable   | 1 attrref | An ungrouped regime variable                    |
| axhrgb_group     | 1 attrref | A group (must match axvr_group in parent)       |
| axhrgc_gate      | 2 attrrefs| Enumerated variable + enum value of that type   |
| axhrgv_variable  | 1 attrref | A variable within current group                 |

### Nesting rules
- Higher-level marker implicitly closes previous (no end markers needed)
- axhrgb_group, axhrgc_gate, axhrgv_variable must appear within axhrb_regime scope
- axhrgc_gate and axhrgv_variable must appear within axhrgb_group scope
- Multiple axhrgc_gate within one group = AND semantics

### Cross-document validation
- axhrb_regime attrref must match a regime defined with axvr_regime in parent
- axhrgb_group attrref must match a group defined with axvr_group under that regime
- axhrgc_gate first attrref must be enumerated variable (axt_enumeration), second must
  be enum value (axt_enum_value) of that specific enumeration type
- axhrgc_gate variable may be from a different regime (cross-regime gating legal)

## Compliance Rules

- axvr_group with axd_conditional requires corresponding axhrgb_group in subdoc to have
  at least one axhrgc_gate
- axvr_group without axd_conditional: corresponding axhrgb_group must have no axhrgc_gate
- Every axvr_variable under a regime should appear as either axhrv_variable or
  axhrgv_variable in the subdoc

## Scope Note

This is a focused experiment in prefix-discriminated annotations distinct from existing
axl_voices Strachey bracket annotations. Existing axl_voices patterns are NOT modified.
The prefix after `//` serves as the grammar selector — `axvr_` and `axhr` each define
their own parsing rules. If this pattern validates over regime usage, the prefix-discriminated
approach may eventually replace Strachey brackets for axl_voices as well.

## Inputs
AXLA-Lexicon.adoc, MCM (prefix-discriminated annotation form already added)

## Output
New AXLA section with axvr_* and axhr*_ terms, compliance rules, examples using //ax form.

**[260206-0849] rough**

Add AXLA terms for regime definition-site voicing (axvr_*) and regime subdoc hierarchy markers (axhr*_).

## Background

MCM now supports prefix-discriminated annotations: comment lines starting with `//ax` (no
space after `//`) are recognized as AXLA annotations. This replaces Strachey bracket syntax
for the new regime annotation families. MCM change already committed.

This pace adds two new annotation grammar families using the prefix-discriminated form:
- axvr_* (definition-site, anchor->annotation->definition pattern)
- axhr*_ (standalone hierarchy markers with attrref lookahead)

## Surface Syntax

Definition-site (between anchor and definition in parent doc):
```
[[rbrv_sigil]]
//axvr_variable axd_required rbst_xname
{rbrv_sigil}::
```

Standalone hierarchy (in subdoc, not between anchor and definition):
```
//axhrb_regime
{rbrv_regime}

//axhrgv_variable
{rbrv_sigil}
```

## axvr_* Definition-Site Terms

New voicing annotations for regime structures in parent documents. All share the constraint
that anchor must match first attrref in definition.

### axvr_regime
- Voices a regime definition
- Dimensions: format (axf_bash, axf_json, etc.)
- No additional lookahead constraints beyond standard voicing

### axvr_variable
- Voices a regime variable definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Ties each variable unequivocally to its regime
- Dimensions: axd_required/axd_optional, type motif

### axvr_group
- Voices a regime group definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Optional dimension: axd_conditional (group has activation gate in subdoc)

## axhr*_ Standalone Hierarchy Terms

Standalone markers using prefix-discriminated form. These appear in regime subdocuments,
NOT between anchor and definition. Each marker reads ahead N attribute references from
following text.

### Markers and lookahead arities

| Marker           | Lookahead | Reads                                          |
|------------------|-----------|-------------------------------------------------|
| axhrb_regime     | 1 attrref | The regime this subdoc describes                |
| axhrv_variable   | 1 attrref | An ungrouped regime variable                    |
| axhrgb_group     | 1 attrref | A group (must match axvr_group in parent)       |
| axhrgc_gate      | 2 attrrefs| Enumerated variable + enum value of that type   |
| axhrgv_variable  | 1 attrref | A variable within current group                 |

### Nesting rules
- Higher-level marker implicitly closes previous (no end markers needed)
- axhrgb_group, axhrgc_gate, axhrgv_variable must appear within axhrb_regime scope
- axhrgc_gate and axhrgv_variable must appear within axhrgb_group scope
- Multiple axhrgc_gate within one group = AND semantics

### Cross-document validation
- axhrb_regime attrref must match a regime defined with axvr_regime in parent
- axhrgb_group attrref must match a group defined with axvr_group under that regime
- axhrgc_gate first attrref must be enumerated variable (axt_enumeration), second must
  be enum value (axt_enum_value) of that specific enumeration type
- axhrgc_gate variable may be from a different regime (cross-regime gating legal)

## Compliance Rules

- axvr_group with axd_conditional requires corresponding axhrgb_group in subdoc to have
  at least one axhrgc_gate
- axvr_group without axd_conditional: corresponding axhrgb_group must have no axhrgc_gate
- Every axvr_variable under a regime should appear as either axhrv_variable or
  axhrgv_variable in the subdoc

## Scope Note

This is a focused experiment in prefix-discriminated annotations distinct from existing
axl_voices Strachey bracket annotations. Existing axl_voices patterns are NOT modified.
The prefix after `//` serves as the grammar selector — `axvr_` and `axhr` each define
their own parsing rules. If this pattern validates over regime usage, the prefix-discriminated
approach may eventually replace Strachey brackets for axl_voices as well.

## Inputs
AXLA-Lexicon.adoc, MCM (prefix-discriminated annotation form already added)

## Output
New AXLA section with axvr_* and axhr*_ terms, compliance rules, examples using //ax form.

**[260204-0824] rough**

Add AXLA terms for regime definition-site voicing (axvr_*) and regime subdoc hierarchy markers (axhr*_).

## Background

Strachey bracket annotations currently only support axl_voices as the grammar. This pace
adds two new annotation grammar families scoped to regime documentation as an experiment:
- axvr_* (definition-site, anchor->annotation->definition pattern)
- axhr*_ (standalone hierarchy markers with attrref lookahead)

## axvr_* Definition-Site Terms

New voicing annotations for regime structures in parent documents. All share the constraint
that anchor must match first attrref in definition.

### axvr_regime
- Voices a regime definition
- Dimensions: format (axf_bash, axf_json, etc.)
- No additional lookahead constraints beyond standard voicing

### axvr_variable
- Voices a regime variable definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Ties each variable unequivocally to its regime
- Dimensions: axd_required/axd_optional, type motif

### axvr_group
- Voices a regime group definition
- Second attrref in definition text must be the parent regime (voices axrg_regime)
- Optional dimension: axd_conditional (group has activation gate in subdoc)

## axhr*_ Standalone Hierarchy Terms

New annotation category: standalone markers that read ahead N attribute references from
following text. These appear in regime subdocuments, NOT between anchor and definition.

### Markers and lookahead arities

| Marker           | Lookahead | Reads                                          |
|------------------|-----------|-------------------------------------------------|
| axhrb_regime     | 1 attrref | The regime this subdoc describes                |
| axhrv_variable   | 1 attrref | An ungrouped regime variable                    |
| axhrgb_group     | 1 attrref | A group (must match axvr_group in parent)       |
| axhrgc_gate      | 2 attrrefs| Enumerated variable + enum value of that type   |
| axhrgv_variable  | 1 attrref | A variable within current group                 |

### Nesting rules
- Higher-level marker implicitly closes previous (no end markers needed)
- axhrgb_group, axhrgc_gate, axhrgv_variable must appear within axhrb_regime scope
- axhrgc_gate and axhrgv_variable must appear within axhrgb_group scope
- Multiple axhrgc_gate within one group = AND semantics

### Cross-document validation
- axhrb_regime attrref must match a regime defined with axvr_regime in parent
- axhrgb_group attrref must match a group defined with axvr_group under that regime
- axhrgc_gate first attrref must be enumerated variable (axt_enumeration), second must
  be enum value (axt_enum_value) of that specific enumeration type
- axhrgc_gate variable may be from a different regime (cross-regime gating legal)

## Compliance Rules

- axvr_group with axd_conditional requires corresponding axhrgb_group in subdoc to have
  at least one axhrgc_gate
- axvr_group without axd_conditional: corresponding axhrgb_group must have no axhrgc_gate
- Every axvr_variable under a regime should appear as either axhrv_variable or
  axhrgv_variable in the subdoc

## Scope Note

This is a focused experiment in structural annotations distinct from axl_voices.
Existing axl_voices patterns are NOT modified. If this pattern validates over regime
usage, the grammar-selector concept (first word in brackets defines parsing rules)
may generalize.

## Inputs
AXLA-Lexicon.adoc, MCM (for form expectations)

## Output
New AXLA section with axvr_* and axhr*_ terms, compliance rules, examples.

**[260203-1932] rough**

Design AXLA patterns for regime specification subdocuments using bare voicing metapattern.

## Problem

RBAGS is massive. Subdocs like RBSRV should carry real detail, but currently they're thin
tables restating what the parent already says. Need a pattern where:
- Parent carries schema (anchors, types, AXLA annotations, terse definitions)
- Subdoc carries validation topology (groups, conditions, constraints)

## Design Decisions (settled)

1. Parent/subdoc split: parent owns all [[anchors]] and attribute mappings with AXLA
   annotations. Subdoc owns validation detail structured by bare voicings.

2. No mutex concept needed: add RBRV_VESSEL_MODE enumeration (bind/conjure) to make
   both RBRV groups conditional. Mutex semantics emerge from single-valued enum.

3. Not every variable needs a group. Groups are for variable subsets with shared
   activation conditions.

4. Bare voicing metapattern: parent context (axrg_regime, axo_procedure) licenses a
   specific vocabulary of bare voicings in included subdocs. Linter validates grammar
   per context type. Already working for procedures (axs_inputs, axs_behavior, etc.).

5. Two-level bare voicing structure for regime subdocs:
   - Section level: axs_group (marks a variable group)
   - Within group: axg_conditional (activation condition), axg_required (required vars),
     axg_optional (optional vars)

## Remaining Design Work (interactive — not bridleable)

1. Prefix conventions for context-licensed bare voicings. Need naming that signals
   which context a bare voicing belongs to and what nesting is legal. The axg_ prefix
   is provisional.

2. Exact AXLA term definitions for: axs_group, axg_conditional, axg_required, axg_optional.
   Consider whether axg_conditional prose must name gate variable + activation value.

3. Whether axd_dependent (variable-level conditionality, e.g. RBRN_UPLINK_ALLOWED_CIDRS
   depends on ACCESS_ENABLED=1 AND ACCESS_GLOBAL=0) is needed alongside group-level
   conditionality, or if group-level is sufficient.

4. Document the bare voicing metapattern itself in AXLA — the concept that parent
   context licenses subdoc grammar.

## Apply Pattern

5. Add RBRV_VESSEL_MODE enumeration to RBAGS vessel regime section.

6. Restructure RBSRV using bare voicing pattern: axs_group sections with
   axg_conditional/axg_required/axg_optional markers.

7. Validate pattern against RBRN's structures (Entry conditional on ENTRY_ENABLED,
   Enclave unconditional, Uplink compound conditions).

## Inputs
RBSRV, RBAGS (lines 1467-1519), RBRN, AXLA-Lexicon.adoc, MCM, CRR

## Output
Updated AXLA with regime bare voicing terms, restructured RBSRV as exemplar,
RBRV_VESSEL_MODE added, clear precedent for future regime subdocs.

**[260202-2001] rough**

Design the AXLA pattern for regime specification subdocuments.

Problem: RBAGS is massive. Subdocs like RBSRV should hold details while RBAGS holds gestalts.
Current approach (all mappings/anchors in parent) makes subdoc "thin gruel".

Explore and decide:
1. What content belongs in parent (RBAGS) vs subdoc (RBSRV)?
   - Attribute reference mappings location
   - Anchor definitions location
   - Table specifications location
   - Term definitions location

2. Subgroup AXLA representation:
   - RBRV has mutually-exclusive subgroups: BIND_* vs CONJURE_*
   - RBRN has conditional subgroups: Entry/Enclave/Uplink (required when enable flag set)
   - Each subgroup likely needs its own attribute reference and anchor
   - Define AXLA vocabulary for these patterns (axd_mutual_exclusive? axd_conditional_group?)

3. Consult RBRN as reference - how does it handle subgroups currently?

4. Update AXLA-Lexicon.adoc with new regime subgroup patterns

5. Apply chosen pattern to RBSRV and its RBAGS integration

Inputs: RBSRV, RBAGS (lines 1467-1519), RBRN, AXLA-Lexicon.adoc, CRR
Output: Revised RBSRV structure, updated AXLA patterns, clear precedent for future regime subdocs

### apply-regime-annotations-rbsrv (₢ARAAP) [complete]

**[260206-0911] complete**

Apply axvr_* and axhr*_ annotation patterns to RBAGS and RBSRV.

## Work Items

1. Mint RBRV_VESSEL_MODE as enumerated regime variable in RBAGS:
   - Add axvr_variable annotation
   - Define axt_enumeration type
   - Mint enum value linked terms: rbrv_vessel_mode_bind, rbrv_vessel_mode_conjure
   - Each enum value gets anchor, attrref, and axt_enum_value voicing

2. Mint regime group linked terms in RBAGS:
   - rbrv_group_binding with axvr_group annotation (axd_conditional)
   - rbrv_group_conjuring with axvr_group annotation (axd_conditional)
   - Each group definition's second attrref is {rbrv_regime}

3. Retrofit existing RBRV variable definitions in RBAGS:
   - Add axvr_variable annotations to rbrv_sigil, rbrv_description, etc.
   - Each variable definition's second attrref ties it to {rbrv_regime}

4. Restructure RBSRV subdoc using axhr*_ hierarchy markers:
   - axhrb_regime followed by {rbrv_regime}
   - axhrv_variable for ungrouped variables (sigil, description, vessel_mode)
   - axhrgb_group for each group, with axhrgc_gate referencing vessel_mode enum values
   - axhrgv_variable for grouped variables with axd_required/axd_optional

5. Verify cross-document consistency:
   - All groups in subdoc match axvr_group definitions in parent
   - All variables accounted for (either axhrv_variable or axhrgv_variable)
   - Gate attrrefs are valid enum variable + enum value pairs

## Inputs
AXLA (with new axvr_/axhr*_ terms from previous pace), RBAGS, RBSRV

## Output
RBAGS with axvr_* annotated regime/variable/group definitions.
RBSRV restructured as exemplar regime subdoc using axhr*_ hierarchy.

**[260204-0824] rough**

Apply axvr_* and axhr*_ annotation patterns to RBAGS and RBSRV.

## Work Items

1. Mint RBRV_VESSEL_MODE as enumerated regime variable in RBAGS:
   - Add axvr_variable annotation
   - Define axt_enumeration type
   - Mint enum value linked terms: rbrv_vessel_mode_bind, rbrv_vessel_mode_conjure
   - Each enum value gets anchor, attrref, and axt_enum_value voicing

2. Mint regime group linked terms in RBAGS:
   - rbrv_group_binding with axvr_group annotation (axd_conditional)
   - rbrv_group_conjuring with axvr_group annotation (axd_conditional)
   - Each group definition's second attrref is {rbrv_regime}

3. Retrofit existing RBRV variable definitions in RBAGS:
   - Add axvr_variable annotations to rbrv_sigil, rbrv_description, etc.
   - Each variable definition's second attrref ties it to {rbrv_regime}

4. Restructure RBSRV subdoc using axhr*_ hierarchy markers:
   - axhrb_regime followed by {rbrv_regime}
   - axhrv_variable for ungrouped variables (sigil, description, vessel_mode)
   - axhrgb_group for each group, with axhrgc_gate referencing vessel_mode enum values
   - axhrgv_variable for grouped variables with axd_required/axd_optional

5. Verify cross-document consistency:
   - All groups in subdoc match axvr_group definitions in parent
   - All variables accounted for (either axhrv_variable or axhrgv_variable)
   - Gate attrrefs are valid enum variable + enum value pairs

## Inputs
AXLA (with new axvr_/axhr*_ terms from previous pace), RBAGS, RBSRV

## Output
RBAGS with axvr_* annotated regime/variable/group definitions.
RBSRV restructured as exemplar regime subdoc using axhr*_ hierarchy.

### assess-rbrn-regime-fit (₢ARAAQ) [complete]

**[260206-0931] complete**

Assess whether the axvr_*/axhr*_ regime annotation patterns work for RBRN.

## Purpose

RBRN (RegimeNameplate) has different structural patterns from RBRV:
- Simple conditional groups (Entry gated by ENTRY_ENABLED)
- Unconditional groups (Enclave, Core Identity, Ark Reference)
- Compound conditional variables (UPLINK_ALLOWED_CIDRS requires ACCESS_ENABLED=1 AND ACCESS_GLOBAL=0)

This pace validates the annotation patterns against these structures without necessarily
applying changes.

## Assessment Questions

1. Do RBRN's unconditional groups (Enclave, Core) work with axhrgb_group without axhrgc_gate?
2. Does the stacked axhrgc_gate pattern (AND semantics) handle RBRN's compound conditions?
3. Is RBRN_ENTRY_ENABLED already an enumeration, or does it need to become one for gates?
   Boolean variables as gates: does axhrgc_gate require axt_enumeration or can axt_boolean work?
4. Are there RBRN patterns that expose gaps in the axhr*_ design?
5. Would RBRN benefit from axd_conditional on variable-level (not just group-level)?

## Inputs
RBRN, AXLA (with new terms), RBSRV (as completed exemplar)

## Output
Assessment document: what works, what needs adjustment, recommendations for RBRN application.

**[260204-0825] rough**

Assess whether the axvr_*/axhr*_ regime annotation patterns work for RBRN.

## Purpose

RBRN (RegimeNameplate) has different structural patterns from RBRV:
- Simple conditional groups (Entry gated by ENTRY_ENABLED)
- Unconditional groups (Enclave, Core Identity, Ark Reference)
- Compound conditional variables (UPLINK_ALLOWED_CIDRS requires ACCESS_ENABLED=1 AND ACCESS_GLOBAL=0)

This pace validates the annotation patterns against these structures without necessarily
applying changes.

## Assessment Questions

1. Do RBRN's unconditional groups (Enclave, Core) work with axhrgb_group without axhrgc_gate?
2. Does the stacked axhrgc_gate pattern (AND semantics) handle RBRN's compound conditions?
3. Is RBRN_ENTRY_ENABLED already an enumeration, or does it need to become one for gates?
   Boolean variables as gates: does axhrgc_gate require axt_enumeration or can axt_boolean work?
4. Are there RBRN patterns that expose gaps in the axhr*_ design?
5. Would RBRN benefit from axd_conditional on variable-level (not just group-level)?

## Inputs
RBRN, AXLA (with new terms), RBSRV (as completed exemplar)

## Output
Assessment document: what works, what needs adjustment, recommendations for RBRN application.

### gar-delete-ark-cleanup (₢ARAAM) [complete]

**[260206-2036] complete**

Review GAR delete operation now that ARKs exist. Delete may need to function on a whole ark rather than just a subimage. Evaluate whether image_delete should be ark-aware or if a separate ark_delete operation is needed. Consider implications for RBSID spec and rbga_ArtifactRegistry.sh implementation.

**[260201-2123] rough**

Review GAR delete operation now that ARKs exist. Delete may need to function on a whole ark rather than just a subimage. Evaluate whether image_delete should be ark-aware or if a separate ark_delete operation is needed. Consider implications for RBSID spec and rbga_ArtifactRegistry.sh implementation.

### implement-rbf-abjure (₢ARAAT) [complete]

**[260206-2040] complete**

Implement rbf_abjure() in rbf_Foundry.sh following the RBSAA-ark_abjure.adoc spec. Takes vessel and consecration arguments, deletes both -image and -about artifacts as a coherent ark unit. Follow the existing rbf_delete() pattern for HTTP/auth mechanics but operate on paired tags. Add coordinator routing (rbw-fA or similar) in rbk_Coordinator.sh and a tabtarget launcher. Handle orphaned artifacts (one exists, other missing) with warnings per spec. Include --force flag to skip confirmation prompt.

**[260206-2036] bridled**

Implement rbf_abjure() in rbf_Foundry.sh following the RBSAA-ark_abjure.adoc spec. Takes vessel and consecration arguments, deletes both -image and -about artifacts as a coherent ark unit. Follow the existing rbf_delete() pattern for HTTP/auth mechanics but operate on paired tags. Add coordinator routing (rbw-fA or similar) in rbk_Coordinator.sh and a tabtarget launcher. Handle orphaned artifacts (one exists, other missing) with warnings per spec. Include --force flag to skip confirmation prompt.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/rbw/rbf_Foundry.sh, Tools/rbw/rbk_Coordinator.sh, tt/rbw-fA.AbjureArk.sh (3 files) | Steps: 1. Read rbf_delete in rbf_Foundry.sh and RBSAA-ark_abjure.adoc spec as patterns 2. Add rbf_abjure function after rbf_delete -- takes vessel and consecration args, constructs both tags using RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT, HEAD-checks each tag for existence, warns on orphaned artifacts, prompts for confirmation unless --force, deletes each existing artifact via DELETE, reports results 3. Add rbw-fA routing in rbk_Coordinator.sh case statement in the Foundry commands section pointing to rbf_cli.sh rbf_abjure 4. Create tt/rbw-fA.AbjureArk.sh tabtarget following exact pattern of tt/rbw-fD.DeleteImage.sh 5. Use ZRBF_DELETE_PREFIX for temp files and ZRBF_REGISTRY_API_BASE for endpoints -- add abjure-specific temp file vars in zrbf_kindle if needed | Verify: bash -n Tools/rbw/rbf_Foundry.sh and bash -n Tools/rbw/rbk_Coordinator.sh

**[260206-2035] rough**

Implement rbf_abjure() in rbf_Foundry.sh following the RBSAA-ark_abjure.adoc spec. Takes vessel and consecration arguments, deletes both -image and -about artifacts as a coherent ark unit. Follow the existing rbf_delete() pattern for HTTP/auth mechanics but operate on paired tags. Add coordinator routing (rbw-fA or similar) in rbk_Coordinator.sh and a tabtarget launcher. Handle orphaned artifacts (one exists, other missing) with warnings per spec. Include --force flag to skip confirmation prompt.

### claude-md-git-discipline (₢ARAAO) [complete]

**[260206-2055] complete**

Strengthen CLAUDE.md git discipline section to explicitly forbid ALL git reset variants.

Problem: Claude ran `git reset HEAD <file>` claiming it was "safe" because it only unstages.
While technically true, this violates the spirit of additive-only discipline and the command
is dangerously close to destructive variants.

Tasks:
1. Review current git discipline section in CLAUDE.md
2. Explicitly enumerate forbidden commands including ALL reset forms:
   - git reset (all variants, with or without --hard, with or without file paths)
   - git restore (when used to discard changes)
   - Existing list: git checkout <file>, git clean, git stash
3. Consider positive framing: what TO do when staging goes wrong (just run jjx_notch, ask user)
4. Make the prohibition memorable and unambiguous

The goal is preventing future Claude sessions from reasoning their way around the intent.

**[260202-2012] rough**

Strengthen CLAUDE.md git discipline section to explicitly forbid ALL git reset variants.

Problem: Claude ran `git reset HEAD <file>` claiming it was "safe" because it only unstages.
While technically true, this violates the spirit of additive-only discipline and the command
is dangerously close to destructive variants.

Tasks:
1. Review current git discipline section in CLAUDE.md
2. Explicitly enumerate forbidden commands including ALL reset forms:
   - git reset (all variants, with or without --hard, with or without file paths)
   - git restore (when used to discard changes)
   - Existing list: git checkout <file>, git clean, git stash
3. Consider positive framing: what TO do when staging goes wrong (just run jjx_notch, ask user)
4. Make the prohibition memorable and unambiguous

The goal is preventing future Claude sessions from reasoning their way around the intent.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 R rbrn-restate-boolean-enums
  2 S rbrn-axhr-voicing
  3 I find-rbrv-cli-kindled-usage
  4 A add-ark-suffix-constants-rbgc
  5 B add-rbrv-vocab-to-rbags
  6 C create-rbrv-regime-spec
  7 J resolve-axla-detail-doc-pattern
  8 D evolve-rbrn-ark-reference
  9 E modernize-rbs-anchors
  10 G add-cosmology-intro-getting-started
  11 H create-burs-regime-spec
  12 K axla-regime-dimension-voicings
  13 L simplify-rbst-definitions
  14 N axla-regime-annotations
  15 P apply-regime-annotations-rbsrv
  16 Q assess-rbrn-regime-fit
  17 M gar-delete-ark-cleanup
  18 T implement-rbf-abjure
  19 O claude-md-git-discipline

RSIABCJDEGHKLNPQMTO
·x··xxxx····x·x···· RBAGS-AdminGoogleSpec.adoc
·····xx·······x···· RBSRV-RegimeVessel.adoc
xx·····x··········· RBRN-RegimeNameplate.adoc
·············xx···· AXLA-Lexicon.adoc
·······x·········x· rbf_Foundry.sh
x·······x·········· RBS-Specification.adoc
x······x··········· rbob_bottle.sh, rbrn_cli.sh, rbrn_nsproto.env, rbrn_pluml.env, rbrn_regime.sh, rbrn_srjcl.env
··················x CLAUDE.md, vocjjmc_core.md
·················x· rbk_Coordinator.sh, rbw-fA.AbjureArk.sh
················x·· RBSID-image_delete.adoc
···············x··· memo-20260206-rbrn-regime-fit-assessment.md
·············x····· MCM-MetaConceptModel.adoc
·········x········· RBSGS-GettingStarted.adoc
·······x··········· RBSAA-ark_abjure.adoc, rbi_Image.sh
····x·············· RBSTB-trigger_build.adoc
···x··············· rbgc_Constants.sh
··x················ jjc-pace-wrap.md
x·················· rbss.sentry.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 102 commits)

  1 N axla-regime-annotations
  2 P apply-regime-annotations-rbsrv
  3 Q assess-rbrn-regime-fit
  4 R rbrn-restate-boolean-enums
  5 S rbrn-axhr-voicing
  6 M gar-delete-ark-cleanup
  7 T implement-rbf-abjure
  8 O claude-md-git-discipline

123456789abcdefghijklmnopqrstuvwxyz
···xxx·····························  N  3c
······xxx··························  P  3c
·········x··xx·····················  Q  3c
··············xxx··················  R  3c
·················x·xx··············  S  3c
·····················xx···x········  M  3c
·························x·xxx·····  T  4c
······························xxx··  O  3c
```

## Steeplechase

### 2026-02-07 07:45 - Heat - f

stabled

### 2026-02-07 07:39 - Heat - s

260207-0739 session

### 2026-02-06 20:55 - ₢ARAAO - W

pace complete

### 2026-02-06 20:55 - ₢ARAAO - n

Expand forbidden git commands list, add positive framing, make unambiguous

### 2026-02-06 20:46 - ₢ARAAO - A

Expand forbidden git commands list, add positive framing, make unambiguous

### 2026-02-06 20:40 - ₢ARAAT - W

pace complete

### 2026-02-06 20:40 - ₢ARAAT - n

Add rbf_abjure function to delete ark artifacts (-image and -about) as coherent unit with existence verification, orphan detection, and confirmation prompt

### 2026-02-06 20:37 - ₢ARAAT - F

Executing bridled pace via sonnet agent

### 2026-02-06 20:36 - ₢ARAAM - W

pace complete

### 2026-02-06 20:36 - ₢ARAAT - B

tally | implement-rbf-abjure

### 2026-02-06 20:36 - Heat - T

implement-rbf-abjure

### 2026-02-06 20:35 - Heat - S

implement-rbf-abjure

### 2026-02-06 20:34 - ₢ARAAM - n

Add clarification about deleting entire artifacts via ark_abjure

### 2026-02-06 20:23 - ₢ARAAM - A

Review RBSID/RBSAA relationship; add cross-reference note to RBSID; verify RBSAA ark vocabulary consistency; confirm rbga is out of scope (repo-level)

### 2026-02-06 20:17 - ₢ARAAS - W

pace complete

### 2026-02-06 20:17 - ₢ARAAS - n

Mint rbrn_* voicings in RBAGS (mapping+definitions), rewrite RBRN with axhr hierarchy markers, add RBAGS include; follows RBRV/RBSRV exemplar; entry/access/dns groups gated on 3-enum modes from ARAAR

### 2026-02-06 20:15 - Heat - s

260206-2015 session

### 2026-02-06 09:43 - ₢ARAAS - A

Mint rbrn_* voicings in RBAGS (mapping+definitions), rewrite RBRN with axhr hierarchy markers, add RBAGS include; follows RBRV/RBSRV exemplar; entry/access/dns groups gated on 3-enum modes from ARAAR

### 2026-02-06 09:41 - ₢ARAAR - W

pace complete

### 2026-02-06 09:41 - ₢ARAAR - n

Restate 5 RBRN booleans as 3 enums (ENTRY_MODE, UPLINK_DNS_MODE, UPLINK_ACCESS_MODE) across spec, env files, and bash scripts

### 2026-02-06 09:33 - ₢ARAAR - A

Restate 5 RBRN booleans as 3 enums (ENTRY_MODE, UPLINK_DNS_MODE, UPLINK_ACCESS_MODE) across spec, env files, and bash scripts

### 2026-02-06 09:31 - ₢ARAAQ - W

pace complete

### 2026-02-06 09:31 - ₢ARAAQ - n

Assess axhr regime fit for RBRN against 5 spec questions, produce assessment memo

### 2026-02-06 09:30 - Heat - S

rbrn-axhr-voicing

### 2026-02-06 09:30 - Heat - S

rbrn-restate-boolean-enums

### 2026-02-06 09:16 - ₢ARAAQ - A

Read RBRN+AXLA+RBSRV, assess axhr pattern fit against 5 spec questions, produce assessment doc

### 2026-02-06 09:11 - ₢ARAAP - W

pace complete

### 2026-02-06 09:11 - ₢ARAAP - n

Restructure RBRV vessel regime with axhr hierarchy, vessel_mode enum, and binding/conjuring groups

### 2026-02-06 09:02 - ₢ARAAP - A

Sequential edits: RBAGS (mint vessel_mode enum, groups, retrofit vars) then RBSRV (restructure with axhr hierarchy)

### 2026-02-06 09:01 - ₢ARAAN - W

pace complete

### 2026-02-06 09:01 - ₢ARAAN - n

Add axvr_* definition-site and axhr*_ hierarchy terms to AXLA with compliance rules

### 2026-02-06 08:50 - ₢ARAAN - A

Add axvr_* definition-site and axhr*_ hierarchy terms to AXLA with compliance rules

### 2026-02-06 08:49 - Heat - T

axla-regime-annotations

### 2026-02-06 08:32 - Heat - s

260206-0832 session

### 2026-02-04 08:25 - Heat - S

assess-rbrn-regime-fit

### 2026-02-04 08:24 - Heat - S

apply-regime-annotations-rbsrv

### 2026-02-04 08:24 - Heat - T

regime-subdoc-axla-patterns

### 2026-02-03 19:32 - Heat - T

regime-subdoc-axla-patterns

### 2026-02-03 18:45 - ₢ARAAN - A

Reading AXLA, RBRN, RBAGS, RBSRV to design subdoc pattern

### 2026-02-03 18:45 - Heat - r

moved ARAAN to first

### 2026-02-03 18:43 - ₢ARAAM - A

Evaluate RBSID vs RBSAA relationship and recommend cleanup path

### 2026-02-03 18:37 - ₢ARAAL - W

pace complete

### 2026-02-03 18:37 - ₢ARAAL - n

Remove AXLA-duplicated prose from rbst_* definitions, keep RB-specific constraints only

### 2026-02-03 18:33 - ₢ARAAL - A

Remove AXLA-duplicated prose from rbst_* definitions, keep RB-specific constraints only

### 2026-02-03 18:32 - ₢ARAAK - W

pace complete

### 2026-02-03 18:31 - Heat - r

moved ARAAO to last

### 2026-02-03 18:30 - Heat - s

260203-1830 session

### 2026-02-02 20:13 - Heat - n

AXLA: add axd_list dimension and regime variable qualification rules

### 2026-02-02 20:12 - Heat - S

claude-md-git-discipline

### 2026-02-02 20:03 - Heat - n

RBSRV: convert assignment variables to attribute references

### 2026-02-02 20:01 - Heat - S

regime-subdoc-axla-patterns

### 2026-02-01 21:23 - Heat - S

gar-delete-ark-cleanup

### 2026-02-01 21:01 - ₢ARAAK - A

Option A: add axd_list dimension to AXLA

### 2026-02-01 20:59 - Heat - T

create-burs-regime-spec

### 2026-02-01 20:43 - ₢ARAAH - A

Create BURS-BashUtilityRegimeSpec.adoc following MCM patterns with AXLA voicings

### 2026-02-01 20:42 - ₢ARAAG - W

pace complete

### 2026-02-01 20:42 - ₢ARAAG - n

Write cosmology intro section: cover vessel-ark relationship, multiple arks per vessel, pedigree/attestation concepts

### 2026-02-01 20:34 - ₢ARAAG - A

Write cosmology intro section: cover vessel-ark-nameplate flow, trust model, naming convention

### 2026-02-01 20:33 - ₢ARAAE - W

pace complete

### 2026-02-01 20:33 - ₢ARAAE - n

Mechanical sed replacement: remove term_ prefix from anchors and refs

### 2026-02-01 20:32 - ₢ARAAE - A

Mechanical sed replacement: remove term_ prefix from anchors and refs

### 2026-02-01 20:31 - ₢ARAAD - W

pace complete

### 2026-02-01 20:31 - ₢ARAAD - n

Replace moniker+tag pairs with vessel+consecration ark reference pattern

### 2026-02-01 19:48 - ₢ARAAD - A

Replace moniker+tag pairs with single ark reference pattern

### 2026-02-01 19:47 - Heat - s

260201-1947 session

### 2026-01-31 23:57 - ₢ARAAJ - W

pace complete

### 2026-01-31 23:16 - ₢ARAAJ - n

Establish rbst_* type voicing pattern: add Type Voicings section to RBAGS, update RBSRV as exemplar subdocument with clean structure

### 2026-01-31 22:47 - Heat - T

axla-regime-dimension-voicings

### 2026-01-31 22:35 - Heat - S

simplify-rbst-definitions

### 2026-01-31 22:31 - Heat - S

axla-regime-dimension-voicings

### 2026-01-31 21:58 - Heat - s

260131-2158 session

### 2026-01-31 12:56 - Heat - r

moved ARAAJ to first

### 2026-01-31 12:55 - Heat - S

resolve-axla-detail-doc-pattern

### 2026-01-31 12:47 - ₢ARAAC - W

pace complete

### 2026-01-31 12:47 - ₢ARAAC - n

Extract RBRV regime variables to detail spec: create RBSRV-RegimeVessel.adoc, replace RBAGS definitions with brief gestalt descriptions and mappings

### 2026-01-31 12:22 - ₢ARAAC - A

Review RBRN pattern → Adapt structure for vessel variables → Create RBRV spec with MCM anchors → Verify RBAGS attribute synchronization

### 2026-01-31 12:22 - ₢ARAAI - W

pace complete

### 2026-01-31 12:22 - ₢ARAAI - n

Verify RBRV_CLI_KINDLED naming: analyze kindle pattern usage in module, check if suffix is correct

### 2026-01-31 12:21 - ₢ARAAI - A

Verify RBRV_CLI_KINDLED naming: analyze kindle pattern usage in module, check if suffix is correct

### 2026-01-31 12:18 - ₢ARAAB - W

pace complete

### 2026-01-31 12:18 - ₢ARAAB - n

jjb:1011-ARAAB: Document RBRV Vessel Configuration variables and wrap discipline

### 2026-01-31 12:12 - Heat - S

find-rbrv-cli-kindled-usage

### 2026-01-31 12:02 - ₢ARAAA - W

pace complete

### 2026-01-31 12:02 - ₢ARAAA - n

Establish heat ₣AS context: Recipe Bottle-AXLA alignment foundation

### 2026-01-31 12:00 - ₢ARAAA - n

Add ark artifact suffix constants to RBGC

### 2026-01-31 12:00 - Heat - s

260131-1200 session

### 2026-01-30 08:04 - Heat - S

create-burs-regime-spec

### 2026-01-30 08:04 - Heat - T

add-cosmology-intro-getting-started

### 2026-01-30 08:03 - Heat - T

modernize-rbs-anchors

### 2026-01-30 08:03 - Heat - T

evolve-rbrn-ark-reference

### 2026-01-30 08:03 - Heat - T

create-rbrv-regime-spec

### 2026-01-30 08:03 - Heat - T

add-rbrv-vocab-to-rbags

### 2026-01-30 08:02 - Heat - T

add-ark-suffix-constants-rbgc

### 2026-01-30 07:45 - Heat - f

racing

### 2026-01-30 07:41 - Heat - S

add-cosmology-intro-getting-started

### 2026-01-30 07:40 - Heat - S

explode-rbs-procedures-axla-voicings

### 2026-01-30 07:40 - Heat - S

modernize-rbs-anchors

### 2026-01-30 07:40 - Heat - S

evolve-rbrn-ark-reference

### 2026-01-30 07:40 - Heat - S

create-rbrv-regime-spec

### 2026-01-30 07:40 - Heat - S

add-rbrv-vocab-to-rbags

### 2026-01-30 07:40 - Heat - S

add-ark-suffix-constants-rbgc

### 2026-01-30 07:38 - Heat - N

rbw-ark-vessel-vocabulary

