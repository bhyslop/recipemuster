# Itches

## rbags-retriever-spec
Specify rbtgo_retriever_create and rbtgo_image_retrieve in RBAGS.

## rbags-validation-run
End-to-end test of remote build with real infrastructure.

## rbags-payor-spec-review
Verify rbtgo_payor_establish/refresh spec matches rbgm_ManualProcedures.sh.

## rbags-payor-guide-links
Improve link transparency in payor_establish guide: either display full URL under link text or make link text descriptive/project-specific rather than generic. Consider BUC configuration option to control whether links render as clickable or show expanded text.

### Problem

Links in documentation guides (e.g., payor_establish) are often generic and hide the actual URL. This reduces transparency and makes it hard to verify where links point.

### Options

1. **Full URL underneath**: Display as `[link text](full-url-shown-below)` with URL visible below link
2. **Descriptive text**: Use specific link text like `[GCP IAM Service Account Documentation for payor-project](url)` instead of generic `[documentation](url)`
3. **Configurable rendering**: Add BUC option to choose between clickable and expanded formats per document

### Context

Identified while reviewing payor_establish guide during RBAGS manual procedure specification work.

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

## bvu-bcg-sequencing
Refactor BVU validators to align with BCG module architecture: ensure validation order properly interweaves with kindle initialization sequencing across dependent modules.

### Context

BVU validators (buv_validation.sh) use pre-BCG architecture with wrapper-based parameter indirection and embedded error handling. BCG defines modern module structure with kindle/sentinel boilerplate and clear initialization sequencing.

The challenge is subtle: validation can't happen in isolation—it must coordinate with *when* modules kindle and *which* validators are available at each stage. A module's kindle needs validators already present, but those validators themselves may kindle. The initialization sequence is interdependent.

### Scope

- Map current BVU validator dependencies and kindle requirements
- Identify sequence constraints that affect validation availability
- Refactor validators to integrate with BCG patterns while respecting kindle ordering
- Ensure backward compatibility during transition

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

## rbgm-establish-install-overlap
Remove step 10 from payor_establish guide - it duplicates payor_install procedure.

### Problem

The `rbgm_payor_establish()` procedure ends with step 10 "Install OAuth Credentials" which overlaps with the separate `rbgp_payor_install` operation. This creates:

1. **Duplication**: Same content in two places to maintain
2. **Ambiguity**: Uses internal function name `rbgp_payor_install` rather than tabtarget `rbw-PI.PayorInstall.sh`
3. **Scope creep**: Establishment procedure should end at "download JSON" - installation is a separate operation

### Proposed Fix

1. Remove step 10 from `rbgm_payor_establish()`
2. End with clear handoff: "Proceed to payor_install operation with downloaded JSON file"
3. Reference tabtarget name in handoff, not internal function name

### Context

Identified during cloud-first-light heat, 2025-12-28.

## rbgp-billing-quota-detection
Improve detection and reporting of Cloud Billing quota limit errors in depot_create.

### Problem

When a billing account reaches its quota limit on linked projects, the Cloud Billing API returns HTTP 400 `FAILED_PRECONDITION` with error details buried in a `google.rpc.QuotaFailure` structure. Current error reporting only shows the generic "Precondition check failed" message, making it hard to diagnose the actual cause (quota exceeded vs. other precondition failures).

### Expected Scenario

During depot_create debugging sessions where multiple test depots are created without deletion, the billing account quota limit is reached. This is normal and expected in development—we need developers to quickly recognize and understand the cause rather than debugging a vague "precondition failed" error.

### Proposed Solution

1. Parse the Cloud Billing API error response for `QuotaFailure` details
2. When detected, display a clear diagnostic message: "Cloud billing quota exceeded for account [ACCOUNT_ID]. Clean up unused depot projects or request quota increase."
3. Include link to GCP quota increase support page
4. Exit with a specific error code or flag that distinguishes quota errors from other failures

### Implementation Hints

- Check `rbgu_depot_billing_link_u_resp.json` structure: `error.details[0]["@type"]` equals `"type.googleapis.com/google.rpc.QuotaFailure"`
- Extract account ID from `violations[0].subject` field
- Current code location: `rbgp_Payor.sh` in the depot_create function

### Context

Identified during cloud-first-light heat debugging, 2025-12-28. Test depot creation repeatedly hits billing quota as expected during iterative debugging—better error reporting will save debugging time.

## buc-info-default-visibility
`buc_info()` requires `BUC_VERBOSE >= 1` to print, which is confusing and unexpected. Most expect info-level logging to always display like `warn()` or `die()`. Consider making `buc_info()` always print by default, or improve documentation about this non-standard behavior.

## rbrn-volume-mounts-cleanup
The pluml nameplate has broken/placeholder volume mount configuration with "OUCH NOT ACTUALLY PROPER" comments:
- `RBRN_VOLUME_MOUNTS` points to wrong directory (`RBM-environments-srjcl` instead of pluml)
- `RBRN_UPLINK_ALLOWED_*` values are placeholders for a service that shouldn't need uplink access

Fix these or make volume mounts properly optional in the nameplate validation.

### Context
Identified during dockerize-bashize-proto-bottle heat planning, 2025-12-29.
