# Itches

## rbags-retriever-spec
Specify rbtgo_retriever_create and rbtgo_image_retrieve in RBAGS.

## rbags-validation-run
End-to-end test of remote build with real infrastructure.

## rbags-payor-spec-review
Verify rbtgo_payor_establish/refresh spec matches rbgm_ManualProcedures.sh.

## rbags-api-audit
Verify remaining RBAGS operations against GCP REST API docs (depot_create, depot_destroy, retriever_create).

## axo-relevel
The axo_ (Axial Operation) category has accumulated disparate concepts:

1. **Operation execution types** (original intent): command, guide, pattern, sequence
2. **Identity concepts**: role, identity, actor
3. **Configuration infrastructure**: regime, slot, assignment
4. **Dependencies**: dependency

### Proposed First Step
Sort all axo_ attribute mappings into coherent subgroups to understand the natural clustering before deciding on category splits.

### Options to Consider
- Keep axo_ as broad "operational infrastructure"
- Split identity concepts to new prefix (axi_ is taken for Interface)
- Move regime/slot to axr_ (structural parallel with record/member)
- New category for configuration concepts

### Context
Emerged during RBAGS AXL voicing heat, session 2025-12-23.

## crg-to-cmk
Consider adding CRG (Configuration Regime Requirements) to concept-model-kit.md as a sibling pattern to MCM. CRG is a meta-specification for defining configuration regimes - reusable across projects, not Recipe Bottle specific.

Reference: lenses/crg-CRR-ConfigRegimeRequirements.adoc

## rbgp-create-governor
Create `rbgp_create_governor()` in `Tools/rbw/rbgp_Payor.sh` following RBAGS spec lines 579-653.

### Implementation Guide

**Critical reference:** BCG (`../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`) for bash style, error handling, and control flow patterns.

**Placement:** Add function after `rbgp_depot_list()` (around line 1012), before `rbgp_payor_oauth_refresh()`.

**Pattern:** Follow `rbgp_depot_create` (zrbgp_sentinel, OAuth auth via zrbgp_authenticate_capture).

**Helpers available:** rbgu_http_json, rbgi_add_project_iam_role, rbgo_rbra_generate_from_key.

### Steps per Spec

1. Validate RBRR_DEPOT_PROJECT_ID exists and != RBRP_PAYOR_PROJECT_ID
2. Create SA via iam.serviceAccounts.create in depot project
3. Poll/verify SA accessible via iam.serviceAccounts.get (3-5s intervals, 30s max)
4. Check no USER_MANAGED keys exist via serviceAccounts.keys.list
5. Grant roles/owner via projects.setIamPolicy (policy version 3)
6. Create key via serviceAccounts.keys.create and generate RBRA file

### API Verification

All 5 GCP REST APIs verified against spec (2025-12-25):
- iam.serviceAccounts.create/get
- serviceAccounts.keys.list/create
- projects.setIamPolicy (v3)

No discrepancies found.

### Success Criteria

Function exists, follows spec steps, uses correct auth pattern, adheres to BCG.

### Context

Extracted from heat jjh-b251225-rbags-manual-proc-spec during scope refinement, 2025-12-25. Spec is complete and API-verified; ready for implementation when prioritized.

## cmk-rust-normalizer
Replace LLM-based MCM normalizer with a deterministic Rust CLI tool.

### Motivation

The sonnet-based cmsa-normalizer agent has proven unreliable:
- Skips Phase 2, incorrectly claims "already well-formatted"
- Uses column 31 instead of 30 (violates MCM "multiples of 10" rule)
- Mis-sorts alphabetically (put STOP before CAUTION)
- Broke inline backtick content until explicit rules added
- Non-deterministic - different results each run

MCM normalization is fundamentally mechanical/deterministic - exactly what traditional programs excel at.

### Rust Advantages

- **Deterministic**: Same input → same output, every time
- **Testable**: Unit tests for each rule, regression tests for edge cases
- **Zero runtime**: Static binary, no Python venv complexity
- **Stable**: Edition 2021 + Cargo.lock = compiles unchanged for years
- **Fast**: Milliseconds vs API round-trips

### Architecture

**Data structure**: `Vec<String>` with "build new output" pattern (don't mutate in place)

```rust
let mut output = Vec::new();
for line in input_lines {
    if needs_splitting(&line) {
        for fragment in split_around_terms(&line) {
            output.push(fragment);
        }
    } else {
        output.push(line);
    }
}
```

**Multi-pass structure**:
```rust
enum Block {
    MappingSection(Vec<CategoryGroup>),
    CodeFence(Vec<String>),      // Opaque, pass through
    Prose(Vec<String>),          // Apply term isolation
    SectionHeader(String),       // Opaque
}
```

**Dependencies**: Pure std library preferred (zero external deps). Regex crate unnecessary - patterns are simple, and context tracking (backticks, code fences) requires state machines anyway.

### Phase 1: Text Normalization

1. Parse file into blocks (mapping, code fence, prose, headers)
2. For prose blocks, find `{term}` references outside opaque contexts
3. Opaque contexts: backticks, code fences, section headers, list markers, table cells
4. Insert line breaks before/after terms
5. Serialize back to lines

### Phase 2: Mapping Section Normalization

1. Parse `:attr:` lines into `MappingEntry { attr, anchor, display }`
2. Group by category comment headers
3. Sort each group alphabetically by display text
4. Align `<<` to smallest multiple of 10 that fits longest attr in group
5. Serialize with consistent spacing

### CLI Interface

```bash
mcm-normalize <file.adoc>           # normalize in place
mcm-normalize --check <file.adoc>   # exit 1 if changes needed
mcm-normalize --diff <file.adoc>    # show what would change
```

### Success Criteria

- Processes RBAGS correctly (matches manually normalized version)
- Handles MCM spec document correctly
- Unit tests for each opaque context type
- Zero dependencies (pure std)

### Context

Emerged from failed cmsa-normalizer attempts during RBAGS normalization, 2025-12-25. Even sonnet model made systematic errors on this mechanical task.

### LLM Agents Remain Useful For

- Promotion/demotion analysis (semantic understanding)
- Suggesting missing term links (context awareness)
- Validation with repair suggestions

## rbtgo-image-retrieve
Design and implement the image retrieval operation - currently has neither spec nor implementation.

### Context

Identified during RBAGS audit (heat jjh-b251225-rbags-manual-proc-spec, 2025-12-25) as one of two missing implementations in the Director-triggered remote build flow.

### Prerequisites

Before implementation:
1. Specify rbtgo_image_retrieve in RBAGS following completeness criteria
2. Verify API calls against GCP REST documentation

### Open Questions

- Which GCP API retrieves container images from Artifact Registry?
- What authentication pattern - Governor RBRA or Director token?
- Output format - tarball, OCI manifest, or streaming pull?
- Destination - local file, pipe to podman, or registry mirror?

### Related

- `rbtgo_trigger_build` - triggers the build that creates images
- `rbtgo_image_delete` - removes images (has implementation in rbf_Foundry.sh)
