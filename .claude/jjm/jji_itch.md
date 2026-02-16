# Itches

## burc-acronym-config-collision
Rename BURC to avoid "Configuration" confusion with Config Regimes.

### Problem

BURC = "Bash Utility Regime Configuration" but we also have "Config Regimes" (CRG/CRR) as a distinct concept. The word "Configuration" appears in both, creating confusion about which "configuration" is being discussed.

### Scope

1. Inventory all existing regime prefixes across the project to find unique name
2. Choose new acronym that doesn't collide with existing regime terminology
3. Rename BURC throughout codebase (files, variables, documentation)

### Known Regime Prefixes (verify when converting to pace)

Need to audit: BURC, BURS, RBRR, RBRN, RBRV, RBRP, RBRA, RBRO, CRG, and any others.

### Context

Identified 2026-01-07 during code review. "Bash Utility Regime Configuration" vs "Config Regime Requirements" - both use "Configuration/Config" causing terminology collision.

## rbm-prerelease-regime-standardization
Bring all Config Regimes to a common standard before first RBM delivery.

### Problem

Config Regimes have evolved organically across the project. Each regime has different levels of completeness:
- Some have specs, others don't
- Some follow BCG kindle/sentinel, others use older patterns
- Validation coverage varies
- No renderers exist
- Naming and file organization inconsistent

Before first delivery, all regimes need consistent quality.

### Known Regimes (verify when commissioning)

| Prefix | Name | Purpose |
|--------|------|---------|
| BURC | Bash Utility Regime Configuration | Project-level BUK config |
| BURS | Bash Utility Regime Station | Developer/machine-level BUK config |
| RBRR | Recipe Bottle Regime Repo | Repository-level config |
| RBRN | Recipe Bottle Regime Nameplate | Per-nameplate config |
| RBRV | Recipe Bottle Regime Vessel | Vessel config |
| RBRP | Recipe Bottle Regime Payor | Payor identification |
| RBRA | Recipe Bottle Regime Admin | Admin/auth config |
| RBRO | Recipe Bottle Regime OAuth | OAuth credentials |

**Note**: This list requires exhaustive study when work begins. Other regimes may exist.

### Scope

For each regime, ensure:
1. Specification document exists and is complete
2. Assignment file follows standard format
3. Validator exists and covers all variables
4. Kindle/sentinel pattern follows BCG
5. Renderer exists for human-readable display
6. Naming conventions consistent across all regimes

### Context

Pre-release quality gate. Regime infrastructure is load-bearing for RBM operations — inconsistency creates maintenance burden and onboarding friction.

## rbm-llm-documentation-consolidation
Consolidate scattered LLM-facing documentation into consistent lenses directories.

### Problem

Documentation that LLMs need for context is scattered across multiple repos with stale cross-repo paths. This creates:
- Fragile references that break when files move
- Duplication between documents (e.g., BPA duplicated README content)
- Inconsistent format and organization
- Context that may be missing or inaccessible to LLMs in different repos

### Documents Requiring Consolidation Review

**From BPA external references (now deleted):**

| Document | Current Location | Content |
|----------|------------------|---------|
| `lens-console-makefile-reqs.md` | cnmp lenses | TabTarget concept |
| `lens-mbc-MakefileBashConsole-cmodel.adoc` | cnmp lenses | MBC implementation |
| `crg-CRR-ConfigRegimeRequirements.adoc` | recipebottle-admin | Authoritative Config Regime definition |
| `rbw-RBRN-RegimeNameplate.adoc` | recipebottle-admin | Example regime specification |
| `crgv.validate.sh` | brm_recipebottle | Validation functions for regime types |
| `crgr.render.sh` | brm_recipebottle | Rendering functions for regime display |
| `axl-AXLA-Lexicon.adoc` | cnmp lenses | Regime definition vocabulary |

**Note**: This list requires exhaustive audit when work begins. Other scattered documentation likely exists.

### Scope

1. Audit all cross-repo documentation references
2. Decide which documents belong where (single source of truth)
3. Migrate or consolidate as needed
4. Update CLAUDE.md mappings to reflect new locations
5. Remove stale references and duplicates

### Target State

Each repo has a `lenses/` or `Tools/*/lenses/` directory containing:
- Documents that LLMs need for that repo's context
- No cross-repo path dependencies
- Consistent format (decide: adoc vs md)

### Context

Identified during BPA cleanup — BPA was redundant with README and contained only stale external references. Those references point to real documents that need consolidation work.

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

## rbal-central-bottle-allocation
Centralize port and enclave network allocation across all nameplates to enable concurrent bottle operation.

### Problem

Running multiple bottles simultaneously requires:
1. **Unique ENTRY_PORT_WORKSTATION** - host port conflicts if two bottles claim same port
2. **Non-overlapping ENCLAVE_BASE_IP** - internal network conflicts even more serious

Currently each nameplate independently specifies these values. Developer must manually check all existing nameplates when adding a new one, and there's no enforcement mechanism.

### Solution Direction: Dynamic Allocation Regime

Create `rbal.env` - a "dynamic regime" that is:
- **Managed by tooling** - `rbw-commission`/`rbw-decommission` modify it
- **Checked into git** - changes visible in PRs, part of repo state
- **Authoritative at runtime** - bottles look up their assignments here

Format (bash-sourceable .env):
```bash
# rbal.env - Dynamic Bottle Allocation Regime
# Managed by rbw-commission/rbw-decommission - do not edit manually

# nsproto - commissioned 2025-01-15
RBAL_nsproto_ENTRY_PORT=8001
RBAL_nsproto_ENCLAVE_SLOT=0

# pluml - commissioned 2025-01-20
RBAL_pluml_ENTRY_PORT=8002
RBAL_pluml_ENCLAVE_SLOT=1
```

Where `ENCLAVE_SLOT` derives actual IPs: `10.242.${SLOT}.0/24`, `10.242.${SLOT}.2` (sentry), `10.242.${SLOT}.3` (bottle).

### Nameplate Changes

Move out of nameplate (derived from allocation):
- `RBRN_ENTRY_PORT_WORKSTATION`
- `RBRN_ENCLAVE_BASE_IP`, `RBRN_ENCLAVE_SENTRY_IP`, `RBRN_ENCLAVE_BOTTLE_IP`

Keep in nameplate (policy/intent):
- `RBRN_ENTRY_PORT_COUNT` (0, 1, or maybe 2) - declares need, not value
- `RBRN_ENTRY_PORT_ENCLAVE` - internal port the bottle listens on
- `RBRN_ENCLAVE_NETMASK` - probably always 24, but keep for flexibility

### Commissioning Model

**No separate commission step.** Instead:
1. Every rbw command checks current nameplate inventory against cached commission state
2. Cache lives in station filesystem (not repo) - workstation-local state
3. When mismatch detected (new nameplate, removed nameplate): alarm bells, all commands fail
4. User must explicitly run reconciliation to embrace new commissioning
5. Reconciliation updates both `rbal.env` (repo) and station cache

This means adding a nameplate triggers enforcement automatically on next command.

### Running Bottles Concern

If allocation changes while bottles are running, they're using stale port/network bindings. Options:
- Store allocation hash at bottle start, check on operations
- Marker file indicating "derived state changed"
- Simple documentation: "stop all bottles before reconciliation"
- Reconciliation refuses to proceed if any bottles detected running

Probably start simple (refuse if running) and add sophistication only if needed.

### Decommissioning Question

When a nameplate is removed, what happens to its allocation slot?
- **Preserve forever**: Slot 3 stays reserved even if that moniker is gone (simple, wastes slots)
- **Allow reuse**: Next commission gets the freed slot (complex, could cause confusion if old config lingers)
- **Explicit release**: Decommission marks slot available, but doesn't auto-assign (middle ground)

### Open Questions

1. Exact location of station cache file?
2. What constitutes "bottles running" - check for containers by moniker pattern?
3. Should `RBRN_ENTRY_PORT_COUNT=2` be supported, or just 0/1?
4. Port allocation strategy - sequential from base, or some other scheme?
5. Integration with existing rbw commands - where does the check hook in?

### Context

Long-percolating idea, refined during itch discussion 2025-12-29. Core insight: enclave network allocation is actually the more serious case than ports.

## buk-claude-install
Add Claude Code installation support to BUK, separate from workbench operations.

### Motivation

Some projects using BUK want Claude Code integration; others want to hide Claude sophistication entirely. The Claude setup should be:
- **Separate from workbench** - not bundled into general BUK usage
- **Optional** - projects can use BUK without any Claude awareness
- **Station-aware** - configures permissions for station-specific directories

BUK README already documents the directories well (temp, output, logs, transcript). This itch is purely about Claude Code integration.

### Scope

A distinct installer (not workbench) that:
1. Adds `Read` permissions to `.claude/settings.local.json` for station directories:
   - `BURC_TEMP_ROOT_DIR` (working/scratch)
   - `BURC_OUTPUT_ROOT_DIR` (outputs)
   - `BURS_LOG_DIR` (logs including transcript files)
2. Derives paths from `burc.env` and `burs.env` so permissions match actual configuration
3. Can be skipped entirely in projects that don't want Claude integration

### Naming

Needs a name distinct from "workbench" - perhaps:
- `buk-claude-setup`
- `buci` (BUK Claude Install)
- `buk-ai-install`

### Reference

BUK README "Future Directions" already mentions:
- "Hidden Configuration Workbench" - internal workbench for Claude-specific config
- "Standards Installation & Awareness" - BCG integration with CLAUDE.md

This itch is the concrete first step toward those visions.

### Context

Identified 2025-12-29. Key insight: Claude integration should be opt-in and separable from core BUK.

## buc-context-refactor
Replace global `buc_context` pattern with per-file wrapper functions using `z_locale` variable.

### Problem with Current `buc_context`

The current global context pattern has a fundamental flaw: when a workbench or module dispatches to other files, the context becomes misleading.

**Example:**
- `vslw_workbench.sh` sets `buc_context "${0##*/}"` globally
- Workbench then sources helpers or calls other scripts
- All output displays `vslw_workbench.sh` as source, even when the actual work happens in helper functions or dispatched subcommands
- Debugging output shows wrong file, making it hard to trace where messages originated

The context is determined at entry point, not at the point where output is produced. This violates BCG principle: "Every potential error explicitly handled" at its location.

### Proposed Solution: `z_locale` Wrapper Pattern with Optional Degradation

Each file/module defines local wrappers following BCG `z_*` conventions.

**Pattern with degradation (for portable scripts):**

```bash
z_locale="${0##*/}"

# Detect BUK availability and set up wrappers
if type buc_step >/dev/null 2>&1; then
    # Full BUK available
    z_step() { buc_step "${z_locale}" "$@"; }
    z_info() { buc_info "${z_locale}" "$@"; }
    z_warn() { buc_warn "${z_locale}" "$@"; }
    z_die()  { buc_die "${z_locale}" "$@"; }
else
    # Graceful degradation - minimal functionality
    z_step() { echo "[STEP] $*"; }
    z_info() { echo "[INFO] $*"; }
    z_warn() { echo "[WARN] $*" >&2; }
    z_die()  { echo "[ERROR] $*" >&2; exit 1; }
fi
```

**Pattern without degradation (for internal tooling):**

```bash
z_locale="${0##*/}"

# Fail fast if BUK not available
type buc_step >/dev/null 2>&1 || {
    echo "ERROR: This script requires BUK (Bash Utility Kit)" >&2
    exit 1
}

z_step() { buc_step "${z_locale}" "$@"; }
z_info() { buc_info "${z_locale}" "$@"; }
z_warn() { buc_warn "${z_locale}" "$@"; }
z_die()  { buc_die "${z_locale}" "$@"; }
```

Then call sites become: `z_step "message"` with correct file attribution in output.

**Key advantage of degradation pattern:** Many scripts need nothing but these four operations - no other BUK functions. The degradation pattern reduces dependency to zero for scripts that only use basic output/error handling, making them portable and shareable without requiring full BUK installation.

### Advantages

1. **Accurate attribution** - Each line shows where it actually came from
2. **No global pollution** - Context doesn't leak across dispatch chains
3. **Explicit ownership** - File decides what to call itself
4. **BCG compliant** - Follows `z_*` local variable pattern and "output at source" principle

### Implementation

1. Update `buc_step`, `buc_info`, `buc_warn`, `buc_success`, `buc_die` signatures to accept context as first argument
2. Update `zbuc_print` to use context from argument (keeping gray color)
3. Add `z_locale` boilerplate to every file using BUC
4. Replace all `buc_step "msg"` calls with `z_step "msg"` throughout codebase
5. Remove `buc_context()` function entirely

### Scope

- All workbenches (vslw, buw, cccw, cmw, jjw, rbw)
- All BCG-compliant modules (rbw/ suite, jjk/, cmk/, etc.)
- Likely ~hundreds of call sites

### Trade-offs

- More verbose call sites (worth the accuracy)
- Per-file boilerplate (4 lines, follows standard conventions)
- Large refactor across codebase (but mechanical and low-risk)

### BCG Documentation Update

BCG extensively documents BUC functions and message hierarchy (lines 500-585). When this refactor completes, BCG needs a new section documenting the wrapper pattern:

- Location for pattern: Add after "Integration Patterns" section (lines 546+)
- Content: Document `z_locale` boilerplate and wrapper functions
- Include: Rationale for per-file context over global `buc_context()`
- Cross-reference: Link to BCG's `z_*` naming conventions (line 395)
- Add to decision matrix: When/why to use wrappers vs raw `buc_*` calls

This ensures new code following BCG patterns uses the correct context approach automatically.

### Update: buc_context Works Well for Single-Entry Scripts

Further testing (2025-12-30) revealed that `buc_context` actually works quite well for the common case:

**What works well:**
- Multi-call bash scripts (tabtargets) that set `buc_context "${0##*/}"` at the top
- Each script gets correct attribution in output
- The function at entry point owns all printouts - this is actually good behavior
- Most RBW/BUW tabtargets follow this pattern and work correctly

**Where the problem actually occurs:**
- Only when workbenches source helpers or dispatch to other files mid-execution
- Specifically: sourced utility functions that produce output inherit the caller's context

**Revised assessment:**
The original problem description overstated the issue. The global context pattern is working as designed for the primary use case. The wrapper pattern may still be valuable for complex multi-file dispatch scenarios, but is lower priority than originally thought.

Consider keeping `buc_context` as-is and only addressing the edge cases where sourced files need their own attribution.

### Context

Emerged 2025-12-30 while testing context prefixes on workbenches. Realized global state creates misleading output when dispatching between files.

## rbsdi-instance-constraints
Document INPUT_INSTANCE parameter constraints for director_create and retriever_create operations.

### Missing Documentation

The RBSDI and RBSRC specs require `«INPUT_INSTANCE»` as first argument but don't specify:
1. **Valid values** - What strings are acceptable? Alphanumeric only? Hyphens allowed?
2. **Relationship to depot** - Must it match depot name? Can differ?
3. **Character constraints** - Length limits? GCP service account naming rules apply?
4. **Multiple instances** - Can one depot have multiple directors/retrievers with different instance names?

### Current Usage Pattern

Heat cloud-first-light uses depot name "proto" for keeper depot. The natural instance value would be "proto" to match, creating:
- `rbwd-proto@rbwg-d-proto-251230080456.iam.gserviceaccount.com`

But spec doesn't mandate this relationship.

### Recommended Clarification

Add to RBSDI/RBSRC a normative statement like:
> `«INPUT_INSTANCE»` identifies this service account instance. Typically matches the depot name for single-instance deployments. Must be lowercase alphanumeric with optional hyphens, 1-20 characters.

### Context

Identified during cloud-first-light heat, 2025-12-30, when tabtarget failed with "Instance name required" error.

## rbsdi-sa-prefix-mismatch
Fix RBSDI/RBSRC specs to use actual code prefixes for service account naming.

### Problem

Spec says `rb-director-«INPUT_INSTANCE»` but code uses `rbwd-«INPUT_INSTANCE»`. Same mismatch exists for retriever (`rb-retriever-` vs `rbwr-`).

### Recommendation

Fix the specs to match the code. Rationale:
1. **Consistent 4-char pattern**: Code uses `rbwm` (mason), `rbwr` (retriever), `rbwd` (director)
2. **Deployed infrastructure**: Changing code could orphan existing service accounts
3. **Length efficiency**: `rbwd-proto` (10 chars) vs `rb-director-proto` (17 chars)

### Spec Changes Required

**RBSDI** line 14: `rb-director-«INPUT_INSTANCE»` → `rbwd-«INPUT_INSTANCE»`
**RBSRC**: `rb-retriever-«INPUT_INSTANCE»` → `rbwr-«INPUT_INSTANCE»`

### Context

Identified during cloud-first-light heat, 2025-12-30, while diagnosing director_create tabtarget.

## rbw-tabtarget-nameplate-pattern
Rename RBW tabtargets to use naming patterns that indicate nameplate requirements.

### Problem

Current RBW tabtarget names don't distinguish between:
- Operations that require a nameplate argument (bottle-specific)
- Operations that work at regime/payor level (no nameplate needed)

This makes it hard to:
1. Know which operations need a nameplate before running them
2. Automate validation of tabtarget invocations
3. Provide helpful tab-completion or documentation

### Proposed Solution

Establish naming convention in tabtarget filenames that indicates nameplate requirement:
- `rbw-n«X».«Name».sh` - Requires nameplate (n = nameplate)
- `rbw-«X».«Name».sh` - No nameplate required (current pattern)

Or alternatively, use a different letter code position to indicate the distinction.

### Benefits

- Pattern-based detection enables tooling to validate arguments
- Self-documenting: filename tells you what's needed
- Enables smarter tab-completion and help systems
- Reduces "forgot the nameplate" errors

### Scope

Audit all RBW tabtargets in `tt/rbw-*.sh` and categorize by nameplate requirement, then rename accordingly.

### Context

Identified 2025-12-30 during VSLK workspace reorganization discussion.

## buw-shellcheck-tabtarget
Create a tabtarget to run shellcheck on all bash files in the repository.

### Motivation

Currently no automated way to lint all bash scripts. Manual shellcheck runs are tedious and easy to skip.

### Proposed Implementation

Create `tt/buw-sc.ShellCheck.sh` that:
1. Finds all `*.sh` files in the repo
2. Runs shellcheck on each
3. Reports summary of errors/warnings
4. Exits non-zero if any errors found

### Considerations

- Exclude vendored/external scripts if any
- Consider shellcheck directives file (`.shellcheckrc`) for project-wide settings
- May want separate modes: quick (errors only) vs full (all warnings)
- Integration with CI if/when that exists

### Context

Identified 2025-12-30 during BCG naming cleanup work.

## rbw-billing-visibility
Create tabtarget to display billing info and current costs for depot projects.

### Motivation

During development, depot projects accumulate costs (Cloud Build, Artifact Registry, storage, etc.). Currently no easy way to:
1. See current month's costs for a depot
2. Check if costs are accumulating unexpectedly
3. Quick-link to billing console for detailed breakdown

### Proposed Implementation

Create `tt/rbw-lC.ListCosts.sh` or similar that:
1. Lists all depot projects under the payor billing account
2. For each, shows current month cost (or links to billing)
3. Optionally shows cost trend or alerts for unusual spend

### Options

**Option A: Direct API**
- Use Cloud Billing API to fetch cost data
- Requires billing.viewer permissions
- Shows costs inline in terminal

**Option B: Console Links**
- Generate direct URLs to GCP Billing console for each depot
- Lower permission requirements
- User clicks to see details

**Option C: Hybrid**
- Show summary via API if permissions allow
- Fall back to console links otherwise

### GCP Resources

- Cloud Billing API: `cloudbilling.googleapis.com`
- Billing reports: `console.cloud.google.com/billing/[ACCOUNT_ID]/reports`
- Project billing: `console.cloud.google.com/billing/linkedaccount?project=[PROJECT_ID]`

### Context

Identified 2025-12-30 during depot create/destroy testing where billing quota was hit.

## rbp-planner-digest
Digest `/Users/bhyslop/projects/recipebottle-admin/rbw-RBP-planner.adoc` for heat development. This planning document likely contains important guidance for structuring and executing Recipe Bottle work that should inform how heats are designed and executed.

## podman-regime-support
Add podman support through environment variable-based regime file selection.

### Approach

Create a mechanism to use an environment variable to choose different base regime files, enabling easy reconfiguration between Docker and Podman environments without manually editing configuration files.

### Implementation Steps

1. Design environment variable pattern for selecting base regime (e.g., `RBW_CONTAINER_ENGINE=podman`)
2. Create podman-specific base regime files alongside existing Docker ones
3. Update regime loading logic to switch between regime files based on environment variable
4. Crank all test cases and suites to verify both Docker and Podman configurations work correctly

### Benefits

- Easy switching between container engines without manual config editing
- Clean separation of Docker vs Podman configuration
- Supports testing both engines in same repository
- Enables developer choice of container runtime

## jj-pace-history-transcript
Add pace history transcript at top of heat files to track all paces created during the heat.

### Problem

During heat execution, paces are regularly abandoned, rewritten, or renamed. Currently there's no easy way to see:
- What pace names have already been used
- Which paces were abandoned vs completed
- The evolution of work during the heat

This makes pace naming harder and loses context about what approaches were tried.

### Proposed Solution

Maintain a "Pace Transcript" section at the top of each heat file that lists all paces ever created with their final status:

```markdown
## Pace Transcript

- `initial-setup` - ✓ Finished
- `fix-docker-auth` - ✗ Abandoned (approach didn't work)
- `buildx-multiplatform` - ✗ Abandoned (switched to different strategy)
- `direct-push-test` - ⧖ Pending
- `cloud-build-integration` - ⧖ Active
```

### Benefits

1. **Naming guidance** - Quick check if a pace name is already taken
2. **Context preservation** - See what's been tried and why it was abandoned
3. **Progress visibility** - Clear record of work evolution throughout heat
4. **Session continuity** - Easy to resume after breaks by reviewing pace history

### Implementation

Update JJ heat management skills (`/jja-pace-new`, `/jja-pace-wrap`, `/jja-pace-arm`) to:
1. Maintain the transcript section at heat file top
2. Add new pace to transcript when created (status: Pending)
3. Update status when pace is wrapped (Finished) or explicitly abandoned
4. Preserve abandoned pace entries rather than deleting them

### Context

Identified during active heat work where pace iteration and abandonment is common, 2025-12-31.

## tabtarget-simplify-and-batch-create
Standardize tabtarget contents and enable batch creation/scrubbing.

### Problem

Tabtarget files have inconsistent structure and patterns. Creating or updating multiple tabtargets requires multiple operations, making it tedious to maintain consistency across a workbench's tabtargets.

### Proposed Solution

1. **Standardize tabtarget contents** - Define simple, consistent structure for all tabtarget files
2. **Variable-args tabtarget creator** - Update the tabtarget creator to accept:
   - First arg: launcher name (required)
   - Remaining args: any number of tabtarget specifications to create
3. **Batch operations** - Enable Claude to create/scrub all tabtargets with one command line

### Example Usage

```bash
# Create multiple tabtargets for rbw workbench in one command
tt/buw-tc.CreateTabTarget.sh rbw \
    "PC:PayorDepotCreate" \
    "PD:PayorDepotDestroy" \
    "lD:ListDepots" \
    "GD:GovernorDirectorCreate" \
    "GR:GovernorRetrieverCreate"
```

### Benefits

- **Consistency** - All tabtargets follow same structure automatically
- **Efficiency** - Create/update many tabtargets in one operation
- **Maintainability** - Easy to scrub all tabtargets when patterns change
- **Documentation** - Single command shows all tabtargets for a workbench

### Context

Identified 2025-12-31 during tabtarget maintenance work.

## jj-persistent-heat-selection
Store currently selected heat in uncommitted local file for seamless heat switching.

### Problem

Currently, heat selection isn't persistent across sessions. When running `/jja-heat-saddle` or other heat operations, you need to specify which heat you're working on each time, or the system needs to infer it from context.

### Proposed Solution

1. **Local state file** - Store currently selected heat in an uncommitted local file (e.g., `.claude/jjm/.current_heat` - gitignored)
2. **Heat selection commands** - Create tabtargets or slash commands to choose/switch the active heat:
   - `/jja-heat-select <heat-name>` or `tt/jjk-hs.HeatSelect.sh <heat-name>`
   - Lists available heats if no argument provided
3. **Implicit heat usage** - Update `/jja-heat-saddle` and other heat operations to use the selected heat from local file if no heat is explicitly specified

### Example Workflow

```bash
# Select a heat (persists across sessions)
/jja-heat-select cloud-first-light

# Later sessions just use the selected heat automatically
/jja-heat-saddle  # uses cloud-first-light from .current_heat
/jja-pace-new     # operates on cloud-first-light
/jja-pace-wrap    # operates on cloud-first-light

# Switch to different heat
/jja-heat-select dockerize-bashize-proto-bottle
```

### Benefits

- **Session continuity** - Heat selection persists across Claude Code sessions
- **Reduced friction** - No need to specify heat name repeatedly
- **Explicit switching** - Clear command to change active heat
- **Backward compatible** - Can still specify heat explicitly when needed

### Implementation Details

- File location: `.claude/jjm/.current_heat` (single line containing heat name)
- Add to `.gitignore` - this is workstation-local state, not repo state
- Validate heat exists when reading from file (handle case where heat was deleted/retired)
- Fall back to inferring heat or prompting if file missing/invalid

### Context

Identified 2025-12-31 during discussion about JJ workflow improvements.

## bcg-three-model-review-heat
Create heat to compare all three Claude models (Opus, Sonnet, Haiku) reviewing bash scripts against BCG standards.

### Objective

Assess how well each Claude incarnation (Opus 4.5, Sonnet 4.5, Haiku 4.0) performs BCG (Bash Console Guide) compliance reviews on project bash scripts, then compare their findings and effectiveness.

### Methodology

1. **Pre-screening with shellcheck** - Run shellcheck first to establish baseline mechanical issues (fairness - don't penalize models for catching things shellcheck already finds)
2. **Independent reviews** - Have each model review the same bash files against BCG standards:
   - Opus 4.5 review
   - Sonnet 4.5 review
   - Haiku 4.0 review
3. **Compare findings** - Analyze differences in:
   - What issues each model identified
   - False positives (flagged non-issues)
   - Missed issues (false negatives)
   - Quality of suggested fixes
   - Understanding of BCG patterns and principles
4. **Meta-assessment** - Evaluate each model's performance on:
   - Accuracy (correct identification of BCG violations)
   - Completeness (thoroughness of review)
   - Practicality (usefulness of recommendations)
   - Cost-effectiveness (quality per token/dollar)

### Scope Options

- **Full project** - Review all bash scripts in Tools/
- **Representative sample** - Select diverse scripts (BUK utils, RBW operations, workbenches, tabtargets)
- **Targeted files** - Focus on scripts that likely have BCG issues

### Deliverables

- Review reports from each model (stored in heat directory)
- Comparison matrix showing what each model caught
- Assessment memo on relative strengths/weaknesses of each model for BCG review work
- Recommendations for which model to use for different review scenarios

### Benefits

- **Improve codebase** - Get comprehensive BCG review of bash scripts
- **Model selection guidance** - Learn which model is best for bash review work
- **BCG refinement** - Identify ambiguities in BCG that cause model disagreement
- **Quality baseline** - Establish expected review quality for future automated checks

### Context

Identified 2025-12-31. All three Claude models (Opus, Sonnet, Haiku) are available via model parameter in Task/agent tools, making this experiment practical.

## workbench-testbench-autodoc
Make workbench and testbench facilities self-documenting from the command line.

### Problem

Tabtarget-roots map to bash functions, but there's no easy way to discover:
1. **What tabtargets are available** - Without looking at filesystem or reading code
2. **What parameters they accept** - Which have tokens, what those tokens mean
3. **Context-specific navigation** - Given a nameplate, what commands work with it? Given a command, what nameplates are available?

Currently you need to:
- Browse `tt/` directory to see what exists
- Read tabtarget source to understand parameters
- Grep through code to find parameter relationships

### Proposed Solution

Add self-documentation capabilities to workbench/testbench:

**1. Tabtarget listing with descriptions**
```bash
tt/buw-ll.ListLaunchers.sh          # Show all available tabtargets
tt/rbw-ll.ListLaunchers.sh rbw       # Show RBW-specific tabtargets
```

**2. Parameter help for token-based tabtargets**
```bash
tt/rbw-GD.GovernorDirectorCreate.sh --help
# Output:
# Creates a Director service account in the Governor project
# Usage: tt/rbw-GD.GovernorDirectorCreate.sh <NAMEPLATE>
# Example: tt/rbw-GD.GovernorDirectorCreate.sh proto
```

**3. Context-aware discovery**
```bash
# Given a nameplate, show available commands
tt/rbw-nc.NameplateCommands.sh proto
# Output: PC, PD, GD, GR, fB, fD, etc.

# Given a command pattern, show compatible nameplates
tt/rbw-cn.CommandNameplates.sh GD
# Output: proto, pluml, nsproto, etc.
```

**4. Function mapping documentation**
```bash
# Show which bash function each tabtarget invokes
tt/rbw-fm.FunctionMap.sh
# Output:
# tt/rbw-PC.PayorDepotCreate.sh → rbgp_depot_create()
# tt/rbw-GD.GovernorDirectorCreate.sh → rbgg_director_create()
```

### Implementation Hints

- Tabtargets could embed structured comments for parsing:
  ```bash
  # @description: Creates a Director service account
  # @usage: <NAMEPLATE>
  # @function: rbgg_director_create
  ```
- Or maintain central registry/manifest that maps tabtargets to metadata
- Help flags could be handled at workbench layer (intercept `--help` before dispatch)
- Context discovery might query regime files or scan nameplate definitions

### Benefits

- **Discoverability** - Find available operations without filesystem browsing
- **Self-documentation** - Understand parameters without reading source
- **Context navigation** - Explore nameplate/command relationships interactively
- **Onboarding** - New developers can explore tooling from command line
- **Tooling foundation** - Enables tab completion, validation, automated help generation

### Context

Identified 2025-12-31 during discussion about making workbench/testbench facilities more discoverable and user-friendly.

## cloud-build-manifest
Track successful cloud build metadata in local JSON manifest.

### Motivation

After running cloud builds, useful information is scattered or lost:
- What was the latest successful build image name/tag?
- How long did the build take (for duration expectations)?
- When was the last successful build for a given nameplate?
- Has build duration changed significantly (performance regression detection)?

Currently this requires:
- Querying GCP APIs to find latest images
- No historical record of build durations
- Manual tracking of what's been built

### Proposed Solution

Maintain a local JSON manifest (`.claude/rbw_build_manifest.json` or similar) that records successful cloud build metadata:

```json
{
  "builds": [
    {
      "nameplate": "proto",
      "timestamp": "2025-12-30T08:04:56Z",
      "image_name": "us-west1-docker.pkg.dev/rbwg-d-proto-251230080456/rbwd-proto/bottle:latest",
      "build_duration_seconds": 127,
      "build_id": "abc123-def456-...",
      "depot_project": "rbwg-d-proto-251230080456"
    },
    {
      "nameplate": "proto",
      "timestamp": "2025-12-29T14:22:13Z",
      "image_name": "us-west1-docker.pkg.dev/rbwg-d-proto-251229142213/rbwd-proto/bottle:latest",
      "build_duration_seconds": 132,
      "build_id": "xyz789-uvw012-...",
      "depot_project": "rbwg-d-proto-251229142213"
    }
  ]
}
```

**Operations:**
- `rbf_build()` appends entry on successful build
- Query utilities to find latest build for nameplate
- Calculate average/median build duration for expectations
- Detect anomalies (build took 2x normal duration)

### Benefits

- **Quick reference** - Latest image name without API queries
- **Duration expectations** - Know how long builds should take
- **Performance tracking** - Detect build slowdowns over time
- **History** - See build patterns across development sessions
- **Offline access** - Build metadata available without GCP connectivity

### Implementation with jq

Use jq for all manifest operations:

```bash
# Append new build record
jq --arg nameplate "$nameplate" \
   --arg timestamp "$timestamp" \
   --arg image "$image_name" \
   --argjson duration "$duration" \
   '.builds += [{nameplate: $nameplate, timestamp: $timestamp, image_name: $image, build_duration_seconds: $duration}]' \
   .claude/rbw_build_manifest.json > /tmp/manifest.json && mv /tmp/manifest.json .claude/rbw_build_manifest.json

# Get latest build for nameplate
jq -r --arg np "$nameplate" '.builds | map(select(.nameplate == $np)) | sort_by(.timestamp) | .[-1]' .claude/rbw_build_manifest.json

# Calculate average build duration for nameplate
jq --arg np "$nameplate" '[.builds[] | select(.nameplate == $np) | .build_duration_seconds] | add / length' .claude/rbw_build_manifest.json
```

### Scope Considerations

- **Local only** - Not checked into git (add to .gitignore), station-specific state
- **Per-nameplate tracking** - Multiple nameplates tracked independently
- **Build success only** - Only record successful builds, failures don't pollute data
- **Optional cleanup** - Maybe prune old entries (keep last N per nameplate)

### Context

Identified 2025-12-31. jq for the win - perfect tool for manipulating build metadata JSON.

## payor-install-rbrp-check
Add RBRP value validation and status display to payor_install completion.

### Problem

After payor_install completes, user is shown a list of "Configuration required in rbrp.env" values but no indication of whether current values are correct. User must manually compare values.

### Proposed Improvement

At completion, display checklist with guide colors showing:
- Which RBRP values already match the OAuth JSON (green checkmark)
- Which RBRP values need manual update (yellow warning)
- Clear todo list of what remains to configure

### Research Needed

- Is there an API way to validate billing account ID is correct/accessible?
- Can we verify billing account is linked to payor project?

### Example Output

```
Configuration Status:
  ✓ RBRP_PAYOR_PROJECT_ID=rbwg-p-251228075220 (matches)
  ✓ RBRP_OAUTH_CLIENT_ID=297222692580-... (matches)
  ⚠ RBRP_BILLING_ACCOUNT_ID - verify manually in Cloud Console
```

### Context

Identified 2026-01-01 during exercise-payor-refresh pace in cloud-first-light heat.

## utility-dependency-hunt
Conduct granular audit of all external utility dependencies across Recipe Bottle bash scripts.

### Objective

Identify every external command/utility that Recipe Bottle bash scripts depend on, at granular level - not just package names like "coreutils" but specific commands like `cat`, `grep`, `sort`, `jq`, etc.

### Why Granular Matters

- **Minimal containers** - Know exactly what to install in minimal base images
- **Portability** - Understand which utilities might differ between Linux/macOS/BSD
- **Documentation** - Clear prerequisites for new developers
- **Installation scripts** - Can generate precise dependency lists
- **Version requirements** - Some utilities need specific versions for certain flags

### Methodology

1. **Extract all external commands** - Parse all `.sh` files to find:
   - Direct command invocations (`cat`, `grep`, `jq`, etc.)
   - Commands in backticks or `$(...)`
   - Pipe chains
   - Commands passed to `bash -c` or similar
2. **Categorize by source**:
   - POSIX standard (available everywhere)
   - GNU coreutils (specific utilities: `cat`, `sort`, `head`, `tail`, `cut`, `tr`, `wc`, etc.)
   - External packages (`jq`, `yq`, `docker`, `podman`, `gcloud`, `gh`, etc.)
   - Bash built-ins (document for clarity even though always available)
3. **Note usage patterns**:
   - Which flags/options are used (some are GNU-specific)
   - Which scripts use which utilities
   - Critical path vs. optional utilities
4. **Create dependency manifest** - Document with:
   - Full list of utilities by category
   - Installation commands for each platform (apt, brew, etc.)
   - Version requirements if any
   - Alternative utilities where applicable

### Example Output Format

```markdown
## Core POSIX Utilities
- `cat` - used in: 47 scripts
- `grep` - used in: 63 scripts (WARNING: uses -P flag, requires GNU grep)
- `sort` - used in: 12 scripts

## GNU Coreutils (Linux standard, install on macOS)
- `timeout` (gtimeout on macOS) - used in: rbgjb scripts
- `base64` - used in: OAuth scripts
- `date` - used in: timestamp generation (uses GNU-specific +%s%N)

## External Packages
- `jq` 1.6+ - JSON processing - used in: all GCP API scripts
- `yq` - YAML processing - used in: buildx scripts
- `docker` or `podman` - container runtime - critical path
- `gcloud` - GCP CLI - used in: all RBW operations
- `gh` - GitHub CLI - used in: PR creation
- `shellcheck` - linting - development only

## Platform-Specific Notes
- macOS requires `brew install coreutils` for GNU timeout
- Linux native has all GNU coreutils
```

### Benefits

- **Clear prerequisites** - Know exactly what to install
- **Container optimization** - Minimal image builds with precise dependencies
- **Portability planning** - Identify GNU-specific vs POSIX patterns
- **Onboarding docs** - Precise setup instructions for new developers
- **Abstraction opportunities** - Identify where wrappers could handle platform differences

### Tools to Build/Use

Could create a utility scanner script that:
1. Parses all bash files with regex/grep patterns
2. Extracts command names from various contexts
3. Filters out functions (internal) vs utilities (external)
4. Generates categorized report

### Context

Identified 2025-12-31. Especially relevant for container builds and cross-platform portability (macOS development, Linux production).

## rbw-rbob-function-review
Review whether workbench functions should move to RBOB module.

### Functions to Review

- `rbw_cmd_local_build()` - builds recipes locally with docker. Currently in workbench but could be `rbob_local_build` since it's container image creation related to bottle operation.
- Future connect/stop implementations - already planned to go to RBOB, but verify pattern is consistent.

### Questions

1. Is local recipe build part of "bottle orchestration" (RBOB) or general "workbench" utility?
2. Should RBOB handle all container image operations, or just runtime lifecycle?
3. What's the clean separation between workbench (routing/config loading) and RBOB (container operations)?

### Context

Emerged during RBOB BCG modernization pace, 2025-12-30. Deferred to keep pace focused on BCG compliance.

## rbgm-guide-tabtarget-awareness
Guides reference commands by function name but users invoke tabtargets. Need coordination mechanism.

### Problem

`rbgm_payor_refresh()` guide text says "run `rbgp_payor_install`" - but users should run `rbw-PI.PayorInstall.sh`. The guide doesn't know the tabtarget name, and if tabtargets get renamed, guides break silently.

### Two Related Issues

**1. Guide tabtarget naming convention**
Tabtargets that emit guides (read-only, display info) vs tabtargets that execute operations (state-changing). Should guides use lowercase letter codes?
- `rbw-PR.PayorRefresh.sh` (current) → `rbw-pr.PayorRefresh.sh` (guide)
- `rbw-PE.PayorEstablishment.sh` (current) → `rbw-pe.PayorEstablishment.sh` (guide)
- Makes read-only nature visible in filename

**2. Cross-reference discovery**
How does a guide function learn the tabtarget for a referenced command?
- Option A: Registry/lookup table mapping function → tabtarget
- Option B: Convention-based derivation (function name pattern → tabtarget pattern)
- Option C: Guide functions receive tabtarget context as parameter
- Option D: BUC helper that queries coordinator routing table

### Affected Guides

At minimum, review all `rbgm_*` functions in `rbgm_ManualProcedures.sh` for cross-references that use function names instead of tabtargets.

### Context

Identified 2025-01-01 during retrospective heat ideation.

## rbw-podman-vm-migration
Migrate podman VM lifecycle from Makefile to bash, enabling RBRN_RUNTIME=podman support.

### Goal

Complete the bash migration by implementing podman VM machinery, making the architecture fully runtime-agnostic. After this heat, users can choose Docker (no VM) or Podman (with VM) via nameplate configuration.

### Scope: Makefile Rules to Migrate

**VM Lifecycle** (rbp.podman.mk):
- `rbp_podman_machine_start_rule` (lines 181-201) - VM startup, version logging
- `rbp_podman_machine_stop_rule` (lines 203-206) - VM shutdown
- `rbp_podman_machine_nuke_rule` (lines 208-213) - VM removal
- `rbp_check_connection` (lines 215-218) - connection validation

**VM Image Management** (rbp.podman.mk, marked DEFERRED/BUGGY):
- `rbp_stash_check_rule` (lines 62-107) - VM image acquisition and validation
- `rbp_stash_update_rule` (lines 138-179) - Pin to controlled version in registry

**Makefile Wrappers** (rbw.workbench.mk):
- `rbw-a.%` (line 79) - Start VM and login to registry
- `rbw-z.%` (line 82) - Stop VM
- `rbw-Z.%` (line 85) - Nuke VM
- `rbw-c.%`, `rbw-m.%`, `rbw-f.%`, `rbw-i.%`, `rbw-N.%`, `rbw-e.%` (lines 98-116) - VM management operations

**Variables to Abstract**:
- `RBM_MACHINE`, `RBM_CONNECTION` - machine name and connection params (rbp.podman.mk:25-26)
- `zRBM_PODMAN_SSH_CMD`, `zRBM_PODMAN_SHELL_CMD` - SSH execution (rbp.podman.mk:37-38)
- `zRBM_EXPORT_ENV` - environment rollup for VM exec (rbp.podman.mk:29-34)

### Architecture Decisions

**Module Structure**:
- Create `rbpv_PodmanVM.sh` (Podman VM lifecycle) following BCG kindle/sentinel pattern
- Add `rbpv_cli.sh` for direct invocation (info, start, stop, nuke commands)
- Extend `rbw_workbench.sh` to route VM commands when `RBRN_RUNTIME=podman`

**Configuration**:
- Add VM parameters to `rbrr_regime.sh` (machine name, connection, image pinning)
- Or create separate `rbrp.env` (Podman regime) if VM config is independent of repo
- Document in RBRR spec (or new spec if separate regime)

**Runtime Abstraction**:
- `rbw_runtime_cmd()` already exists - extend for VM-aware podman invocation
- VM lifecycle becomes prerequisite for podman runtime operations
- Docker operations skip VM lifecycle entirely (no-op)

### Key Technical Challenges

**Bridge Observation**:
- `rbo.observe.sh` uses `podman machine ssh` to capture bridge traffic
- No Docker equivalent - Docker Desktop doesn't expose VM bridge
- Options:
  1. Make bridge capture podman-only (document limitation)
  2. Create diagnostic container with network_mode:host on Docker
  3. Accept bridge observation unavailable on Docker runtime

**VM Image Pinning**:
- Current stash machinery is marked BUGGY/DEFERRED
- Do we implement controlled version pinning, or defer to future heat?
- Minimum viable: hardcode recent stable image, document manual update process

**Connection Validation**:
- `rbp_check_connection` validates VM is reachable before operations
- Integrate into `rbob_start()` and other operations when runtime=podman
- Graceful error if VM isn't running (suggest `rbw-vm-start`)

### Testing Strategy

**Vertical Slice on nsproto**:
- Update `rbrn_nsproto.env`: `RBRN_RUNTIME=podman`
- Implement VM lifecycle in bash
- Migrate one tabtarget at a time (start, connect, test, stop)
- Validate identical behavior to Docker

**Full Validation**:
- All three nameplates (nsproto, srjcl, pluml) pass tests with podman runtime
- Document runtime differences (--internal network flag, ICMP behavior)
- Ensure runtime can be switched per nameplate (mixed environment)

### Deferred to Future

**Batch Operations**:
- Parallel test execution (`rbw-tb.%`, `rbw-ta.%`) still deferred
- Focus on getting podman runtime working, not batch automation

**VM Image Registry**:
- Full stash/pin machinery may be deferred again if too complex
- Document manual image update process as interim solution

### Success Criteria

1. All three nameplates work with `RBRN_RUNTIME=podman`
2. All tests pass (22 nsproto, 3 srjcl, 5 pluml)
3. VM lifecycle managed via bash (start, stop, nuke, connection check)
4. No Makefile rules remaining (except possibly deferred stash/pin)
5. Documentation updated in RBRR or new spec

### Context

Deferred from dockerize-bashize-proto-bottle heat (jjh_b251229), completed 2025-12-31. Docker runtime fully validated; podman runtime is natural next step to complete the architecture vision.

## vvr-mint-analyzer
Prefix inventory and validation for the mint naming discipline — any persistent identifier anywhere.

### Motivation

The mint naming discipline (CLAUDE.md "Prefix Naming Discipline") enforces terminal exclusivity and prefix registration. Currently, violations are discovered manually during code review. A deterministic analyzer would:
- Inventory all prefix usage across **all namespaces** (code, git refs, slash commands, etc.)
- Detect terminal exclusivity violations (`rbg` used as both name AND parent)
- Find orphan prefixes (used but not registered)
- Identify stale registry entries (registered but unused)
- Validate kit suffix conventions (`*a_`=Arcanum, `*c-`=Command, etc.)

### Architecture: vvr Subcommand

```
vvr mint scan [--config <path>]    → JSON inventory to stdout
vvr mint check [--config <path>]   → exit 0 (clean) or 1 (violations found)
vvr mint tree [--config <path>]    → prefix tree visualization
```

The analyzer is a pure filter: reads files, outputs structured data. No mutations.

### What It Extracts

| Source | Pattern | Example |
|--------|---------|---------|
| Filenames | `prefix_Word.ext` | `rbga_ArtifactRegistry.sh` |
| Filenames | `ACRONYM-Words.ext` | `RBAGS-AdminGoogleSpec.adoc` |
| Bash functions | `prefix_name()` | `buc_log_args()` |
| Bash functions | `zprefix_name()` | `zbuc_color()` |
| Variables | `PREFIX_NAME` | `BURC_PROJECT_ROOT` |
| AsciiDoc attrs | `:prefix_term:` | `:rbw_depot:` |
| AsciiDoc anchors | `[[prefix_term]]` | `[[rbw_depot]]` |
| Directories | `Tools/prefix/` | `Tools/buk/` |
| Git refs | `refs/{prefix}/...` | `refs/vvg/locks/*` |
| Slash commands | `/{prefix}-{noun}` | `/vvc-commit` |
| Command files | `.claude/commands/{cmd}.md` | `vvc-commit.md` |

This is not exhaustive. The principle: **any persistent name anywhere is in scope**.

### Exclusion Config

Some paths contain historical/aspirational content outside the mint universe:

```json
{
  "exclude": [
    ".claude/jjm/retired/",
    ".claude/jjm/jjz_scar.md"
  ],
  "warn_only": [
    ".claude/jjm/jji_itch.md"
  ],
  "ignore_patterns": [
    "*.silks"
  ]
}
```

**Exclusion rationale:**
- **Trophies** (retired/) — frozen historical record
- **Scars** (jjz_scar.md) — declined itches, lessons learned
- **Itches** (jji_itch.md) — aspirational, may reference speculative names
- **Silks** — human traction only, expiring, not in mint universe
- **Favors** — generated IDs (₣Kb), not human-minted prefixes
- **Git history** — not scanned (working tree only)

### Output Format

```json
{
  "tokens": [
    {"token": "rbga", "file": "Tools/rbw/rbga_ArtifactRegistry.sh", "line": 1, "category": "filename"},
    {"token": "buc_log_args", "file": "Tools/buk/buc_command.sh", "line": 47, "category": "function"}
  ],
  "prefix_tree": {
    "rb": {"children": ["rbg", "rbi", "rbo", "rbw"], "terminal": false},
    "rbg": {"children": ["rbga", "rbgb", "rbgc"], "terminal": false},
    "rbga": {"children": [], "terminal": true}
  },
  "violations": [
    {"type": "terminal_exclusivity", "prefix": "foo", "message": "foo is both a name and has children"}
  ]
}
```

### Validation Categories

| Check | Description |
|-------|-------------|
| Terminal exclusivity | Prefix used as both name AND parent |
| Orphan prefix | Used but not registered in project registry |
| Stale registry | Registered but unused in codebase |
| Kit suffix violation | Kit prefix (`vo*`, `vv*`, `jj*`, `cg*`) uses wrong reserved suffix |

**Domain-aware validation:** Kit suffix rules (`*a_`=Arcanum, `*c-`=Command, `*w_`=Workbench, etc.) apply only to kit infrastructure prefixes. AsciiDoc concept attributes (`:prefix_term:`) follow MCM semantic categories, not kit suffixes. The analyzer must know which domain a prefix belongs to.

### Implementation Notes

- Pure Rust, minimal deps (regex, serde_json, clap)
- Pattern matching via regex, not full parser (sufficient for mint patterns)
- Config file optional — sensible defaults for JJ paths
- Integrates with VOK as `vvx mint` subcommand (invokes `vvr mint`)

### Success Criteria

- Correctly inventories this repo's mint universe
- Detects intentionally introduced violations in test fixtures
- Exclusion config respected (trophies don't trigger violations)
- Output parseable by both humans and tooling

### Context

Identified 2026-01-11 during Project Prefix Registry update. As codebase grows, manual mint discipline enforcement becomes error-prone.

**Refined 2026-01-11** after VV naming scrub session revealed that mint scope extends beyond "code" to git refs, slash commands, command files, target repo paths, and other persistent identifiers. Added extended namespace extraction, validation categories, and domain-aware suffix checking.

## axla-relational-voicing
Evaluate AXLA voicings for relational table concepts.

### Context

As JJSA defines structured data (Gallops JSON with heats, paces, tacks), consider whether AXLA should provide voicings to express database integrity concepts (foreign keys, referential integrity, cardinality, normalization).

### Deliverables

1. Survey existing relational concepts in JJSA and other specs
2. Evaluate whether explicit voicings would add clarity or are unnecessary
3. If worthwhile, propose specific voicings following MCM patterns

### Success Criteria

Clear decision documented; if yes, AXLA updated accordingly.

### Origin

Deferred from JJR Gallops Core heat (Phase 5 - Future), 2026-01-14.

## jjr-studbook-core
Replace jq-based studbook operations with a pure Rust CLI filter.

### Motivation

Writing "systems programming caliber" bash requires fighting Claude's training data - the internet is full of bad bash. The BCG patterns, temp file conventions, and discipline systems exist to counter-train the model toward reliability. This energy could go to features if the fragile jq pipelines were replaced with type-safe Rust.

The current `jju_utility.sh` has ~50 jq invocations. Each is a place where bad training data could have introduced subtle bugs. Rust replaces all of them with code where the compiler enforces correctness.

### Architecture: jjb (Bash) + jjr (Rust)

**Rename `jju` → `jjb`** to make the split legible:
- **jjb** = bash orchestration (locking, git, file ops)
- **jjr** = rust data integrity (pure transforms)

```
┌─────────────────────────────────────────────────────┐
│  jjb (Bash, née jju_utility.sh)                     │
│  ├─ Locking: git update-ref (refs/jj/locks/*)       │
│  ├─ Git ops: chalk, rein, notch (steeplechase)      │
│  └─ Orchestration: lock → jjr → mv → unlock         │
│                                                     │
│     ┌─────────────────────────────────────────┐     │
│     │  jjr (Rust) - Pure Filter               │     │
│     │  ├─ favor encode/decode                 │     │
│     │  ├─ studbook validate (stdin → exit)    │     │
│     │  ├─ studbook transform (stdin → stdout) │     │
│     │  └─ ledger query/update                 │     │
│     │                                         │     │
│     │  NO git. NO file writes. NO locking.    │     │
│     └─────────────────────────────────────────┘     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Why This Split

**Locking in Bash via `git update-ref`** (recommendation from architecture review):
- Git already solves distributed coordination for multiple agents
- `git update-ref --stdin` provides atomic transactions
- Crash recovery: if jjr crashes, bash detects and releases lock
- Platform-portable: Git handles macOS/Linux/Windows
- Temporally stable: Git CLI more stable than git2 crate bindings

**jjr as pure filter** (stdin → stdout):
- No side effects - if jjr crashes, no partial writes
- Testable in isolation - just data in, validated data out
- Simple implementation - serde_json + validation + clap
- ~300 lines of Rust, no external deps beyond serde

### Usage Pattern

```bash
jjb_tally() {
  jjb_lock_acquire "studbook"

  jjr studbook tally "${z_favor}" "${z_state}" \
    < "${ZJJB_STUDBOOK_FILE}" \
    > "${BUD_TEMP_DIR}/studbook_new.json" \
    || { jjb_lock_release "studbook"; buc_die "jjr failed"; }

  mv "${BUD_TEMP_DIR}/studbook_new.json" "${ZJJB_STUDBOOK_FILE}"
  jjb_lock_release "studbook"
}
```

### jjr Command Surface

```
jjr favor encode <heat> <pace>     →  KbAAB (5 chars, no ₣)
jjr favor decode <favor>           →  10\t42 (tab-delimited)

jjr studbook validate              →  exit 0 or 1 (stdin)
jjr studbook tally <favor> <state> →  transformed JSON (stdin→stdout)
jjr studbook slate <favor> <desc>  →  transformed JSON + new pace favor
jjr studbook heat-exists <favor>   →  exit 0 or 1 (stdin)

jjr ledger hash <file1> <file2>    →  12-char hash
jjr ledger lookup <hash>           →  brand number or exit 1 (stdin)
jjr ledger register <hash>         →  transformed JSON + brand (stdin→stdout)
```

### Key Design Decisions

1. **JSON stays JSON** - git-diffable, human-readable history preserved
2. **Name is `jjr` not `jj`** - avoids collision with Jujutsu VCS, clear provenance
3. **No git2 crate** - all git operations stay in bash
4. **No file I/O in Rust** - bash handles file read/write/atomic-mv
5. **Locking is bash's job** - git update-ref for multi-agent coordination

### What jjr Replaces

Every jq pipeline in jju_utility.sh:
- `zjju_studbook_validate()` - complex 8-step jq validation
- `zjju_favor_encode/decode()` - base64-ish arithmetic
- `jju_saddle()`, `jju_slate()`, `jju_tally()`, etc. - JSON transforms

### What Bash Keeps

- Git operations (chalk, rein, notch, retire steeplechase queries)
- Locking via `git update-ref --stdin`
- File orchestration (read → jjr → atomic mv)
- Error handling and user output
- All the stuff bash is actually good at

### Implementation Notes

- Rust binary: ~300 lines, deps: serde, serde_json, clap
- No runtime deps (static binary)
- Existing test suite validates behavior before/after migration
- Can migrate incrementally: one jq pipeline at a time

### Context

Emerged from conversation 2026-01-05 about energy cost of getting Claude to write reliable bash. Architecture refined with input on distributed locking: let Git own coordination, let Rust own data integrity. The compiler is tireless; humans are not.

## jj-chalking-clues-trophy-analysis
Instrument slash commands with chalk markers, analyze patterns at heat retirement to build templates.

### Concept

Add structured chalk markers during pace execution that capture workflow patterns. When a heat retires (trophy), analyze the chalk trail + git commits to discover common patterns and generate heat/pace templates.

### Chalk Vocabulary (during work)

- `APPROACH` - starting a pace (already exists)
- `PATTERN:slash-command-update` - doing slash command work
- `PATTERN:rust-impl` - Rust implementation
- `PATTERN:bash-infra` - bash/tabtarget infrastructure
- `PATTERN:mcm-doc` - MCM concept model work
- `BLOCKER` - hit an obstacle
- `PIVOT` - changed approach mid-pace

### Trophy Analysis (at retirement)

When heat retires via `/jjc-heat-retire`:
1. Query steeplechase (git log with `[jj:*]` markers)
2. Extract chalk markers per pace
3. Correlate with:
   - Pace silks patterns (e.g., `*-impl`, `*-decision`)
   - Number of commits per pace
   - Reslate frequency
   - Blocker/pivot occurrence
4. Generate statistics: "rust-impl paces average 3 commits, slash-command-update averages 1"

### Template Generation

Identify common pace sequences across heats:
- "Most heats start with a decision pace"
- "MCM doc paces typically precede impl paces"
- "Bash infra often follows design decisions"

Generate suggested templates:
```markdown
## Template: rust-feature-heat
Suggested paces:
1. {feature}-arch-decision (decision)
2. {feature}-concept-model (MCM doc, if complex)
3. {prefix}-coding-guide (if new domain)
4. {feature}-impl (rust-impl)
5. {feature}-slash-commands (slash-command-update)
```

### Benefits

- **Self-improving system** - JJ learns from usage patterns
- **Onboarding** - new users get suggested pace structures
- **Consistency** - common patterns become templates
- **Meta-insights** - understand what pace types take most effort

### Prerequisites

- Consistent chalk vocabulary (needs definition)
- Trophy analysis tooling (rust or bash)
- Template storage location (`.claude/jjm/templates/`?)

### Why "Too Soon"

This requires:
1. More heats completed to have pattern data
2. Chalk vocabulary stabilized
3. Trophy/retirement workflow solidified
4. Analysis tooling built

Better to accumulate experience first, then instrument.

### Context

Emerged 2026-01-14 during VOK heat planning session. Recognized that we're manually discovering patterns (decision paces first, reordering by dependency) that could eventually be codified.

## jj-pace-duplicate-state
Consider adding a 'duplicate' state for paces that were superseded by other paces.

### Problem

Sometimes a pace's work is completed by a different pace (e.g., `deprecate-jju-tabtargets` had `/jjc-heat-rein` creation in scope, but that's tracked separately as `create-heat-rein-command`). Currently we mark such paces as 'complete' even though the work was done elsewhere, or 'abandoned' which implies the work won't happen.

### Proposed Solution

Add a `duplicate` state (or perhaps `superseded`) that indicates:
- The pace's intent is addressed
- But the work was done under a different pace's identity
- Distinct from `abandoned` (work won't happen) and `complete` (work done here)

### Benefits

- **Accurate history** - steeplechase shows why pace closed without work
- **Cleaner trophies** - can distinguish "we did this elsewhere" from "we gave up"
- **Better metrics** - don't inflate completion counts with duplicates

### Alternatives

1. Keep using `abandoned` with a note in tack text
2. Use `complete` and note "superseded by ₢XXXXX" in tack
3. Add `duplicate` as true state in JJSA spec

### Context

Identified 2026-01-16 while wrapping `deprecate-jju-tabtargets` which had `/jjc-heat-rein` creation listed, but that work is tracked as a separate pace.

## jjc-heat-groom-execution-strategy
Add execution strategy assessment to `/jjc-heat-groom` output.

### Problem

`/jjc-heat-mount` now includes execution strategy assessment (primeability, parallelization, model tier) when proposing an approach for rough paces. But `/jjc-heat-groom` reviews the full heat and could provide this analysis for ALL paces at once, enabling better session planning.

### Proposed Enhancement

When groom displays paces, include a summary table:

| # | Pace | Primeable | Model | Blocks | Parallelizable with |
|---|------|-----------|-------|--------|---------------------|
| 1 | fix-foo | ✓ | haiku | — | 2, 3 |
| 2 | add-bar | ✓ | sonnet | — | 1 |
| 3 | design-baz | ✗ | opus | 4 | 1 |
| 4 | impl-baz | ✓ | sonnet | — | — |

### Assessment Criteria

**Primeability** (from CLAUDE.md):
- Mechanical, Pattern exists, No forks, Bounded → all four = primeable

**Parallelization** (cross-pace, not within-pace):
- File independence across paces
- Dependency ordering (does pace B need pace A's output?)
- Can run concurrently without conflicts

**Model tier**:
- haiku: mechanical pattern application
- sonnet: standard development
- opus: architectural decisions

### Benefits

- **Session planning**: "Prime 1 and 2, run parallel, then I'll handle opus, then prime 4"
- **Batching**: Group haiku tasks for efficient parallel execution
- **Visibility**: See which paces block others before starting work

### Context

Identified 2026-01-16. Enhancement to `/jjc-heat-mount` was just added; this extends the same analysis to heat-level review.

## gallops-key-findings-persistence
Maintain key findings in gallops as durable context that survives session boundaries.

### Problem

During heat execution, critical discoveries emerge that:
1. Inform current and future heat work
2. Document facts that can be forgotten over time
3. Need visibility for a while but aren't permanent paddock content
4. Require follow-up (approval, feedback, upstream fixes)

**Examples:**
- Compile bugs worked around that need upstream approval or feedback
- API quirks discovered during integration
- Version-specific behaviors that affect approach
- Workarounds for tooling issues
- Decisions made under time pressure that need revisiting

Currently these get lost in:
- Chat session context (evaporates)
- Steeplechase (execution log, not findings)
- Paddock (stable info, not transient findings)

### Proposed Solution

Add a "Findings" concept to gallops — structured key-value entries that:
- Persist across sessions within a heat
- Surface in mount/groom commands for visibility
- Can be queried ("what findings exist for this heat?")
- Have optional expiry or follow-up dates
- Migrate to paddock or scar when resolved

### Structure

```json
{
  "findings": {
    "rust-edition-workaround": {
      "summary": "Using edition 2021 due to async-trait incompatibility with 2024",
      "detail": "async-trait 0.1.83 fails to compile under edition 2024. Workaround: pin edition 2021 until crate updates.",
      "needs_followup": true,
      "followup_type": "upstream_fix",
      "added": "2026-01-23",
      "context_pace": "₢AfVBk"
    }
  }
}
```

### Commands

- `/jjc-finding-note` — add a finding to current heat
- `/jjc-finding-list` — show all active findings
- `/jjc-finding-resolve` — mark finding as resolved (migrates to paddock or closes)

### Benefits

- **Durable context** — survives session boundaries
- **Visibility** — mount/groom can surface active findings
- **Follow-up tracking** — know what needs upstream attention
- **Historical record** — findings don't disappear when resolved

### Context

Identified 2026-01-23. Compile bugs, API quirks, and workarounds discovered during development often get lost between sessions. Need structured way to capture and surface these.

## jjx-wrap-atomic-command
Combine notch + tally + chalk into a single `jjx_wrap` command for atomic pace completion.

### Problem

Wrap currently requires three separate jjx invocations:
1. `jjx_notch` — commit pending changes
2. `jjx_tally --state complete` — transition pace state
3. `jjx_chalk --marker W` — record wrap marker

Each is a separate git commit, making wrap heavyweight and fragmented. The slash command must orchestrate these steps, interpret intermediate results, and handle failures at each stage.

### Proposed Solution

Create `jjx_wrap <CORONET>` that:
1. Stages and commits pending changes (if any) with notch prefix
2. Transitions pace to complete state
3. Records wrap chalk marker
4. All in a single atomic operation (one or two commits max)

### CLI Interface

```bash
echo "<outcome summary>" | ./tt/vvw-r.RunVVX.sh jjx_wrap <CORONET>
```

Or with explicit outcome:
```bash
./tt/vvw-r.RunVVX.sh jjx_wrap <CORONET> --outcome "<summary>"
```

### Behavior

- If pending changes exist: notch commit + wrap commit (gallops + chalk)
- If no changes: single wrap commit only
- Outcome summary used for both tally text and chalk description
- Returns structured result: `{"notch_hash": "abc123" | null, "wrap_hash": "def456"}`

### Benefits

- **Simpler slash command** — `/jjc-pace-wrap` becomes thin wrapper
- **Fewer git commits** — gallops update + chalk in one commit
- **Atomic** — can't have partial wrap state
- **Consistent** — notch always happens when needed

### Context

Identified 2026-01-18 when wrap was missing automatic notch. Adding Step 3.5 to slash command works but highlighted that wrap is doing too much orchestration that belongs in Rust.

## rbrv-vessel-role-attribute
Add vessel role attribute to RBRV regime for image role branding.

### Problem

Images in GAR are undifferentiated blobs. Nothing in the image or metadata declares "I'm designed to be a sentry" vs "I'm designed to be a bottle." The nameplate assigns role via variable name (`RBRN_SENTRY_MONIKER`, `RBRN_BOTTLE_MONIKER`), but the vessel itself has no role awareness.

### Proposed Solution

Add `RBRV_VESSEL_ROLE` to vessel regime (`rbev-vessels/*/rbrv.env`):
- `sentry` — designed for sentry role
- `bottle` — designed for bottle role
- (No 'both' option — vessels should be purpose-built)

The `-meta` artifact would carry this role through to GAR, enabling:
- `rbw-il` list output grouped by role
- Validation that nameplate doesn't assign sentry image to bottle role
- Role-based image filtering in queries

### Open Questions

**Censer/sentry duality**: Currently censer uses the sentry image (same image, different container role). This concept doesn't cleanly address that duality. Options:
- Censer is an "internal role" not reflected in RBRV (it's a runtime configuration, not an image type)
- Add `censer` as a distinct role (but then we're building separate images for what's currently shared)
- Document that sentry images are also valid for censer role by definition

**Security implications**: Role branding could enable:
- Image signing that embeds permitted roles
- Registry policies that restrict which images can be pulled for which purposes
- Audit trails showing role-image mapping over time

These are speculative — may not be worth the complexity.

### Report Format Vision

With role attribute, `rbw-il` could show:

```
RBRR Registry Context:
  RBGD_GAR_LOCATION:    us-central1
  RBGD_GAR_PROJECT_ID:  rbwg-d-proto-251230080456
  RBRR_GAR_REPOSITORY:  rbw-proto-repository

RBRV_VESSEL_ROLE    RBRN_*_MONIKER         RBRN_*_IMAGE_TAG        AVAILABLE
────────────────    ────────────────────   ─────────────────────   ──────────
sentry              sentry_ubuntu_large    (not set)               20260128T... ✓
bottle              bottle_ubuntu_test     (not set)               (none)
(unknown)           rbev-busybox           —                       20260128T... ✓
```

### Context

Identified 2026-01-28 during `rbw-il` output improvement discussion. The list shows images but can't map them to nameplate roles without explicit vessel role metadata.

## axla-xref-macro-migration
Convert AXLA attribute entries from shorthand cross-references to xref macro form.

### Problem

All AXLA attribute entries currently use the shorthand cross-reference form:
```asciidoc
:axl_motif: <<axl_motif,Motif>>
```

The shorthand `<<target,text>>` treats the display text as plain text — no inline formatting is possible.

### Proposed Change

Convert to the xref macro form:
```asciidoc
:axl_motif: xref:axl_motif[Motif]
```

The macro form's square brackets allow inline formatting in the display text (bold, italic, monospace, etc.), enabling richer rendered output for specifications that consume AXLA terms.

### Scope

- All `:attr: <<anchor,text>>` lines in AXLA-Lexicon.adoc (~240 attribute entries)
- Mechanical transformation: `<<X,Y>>` → `xref:X[Y]`
- Verify no downstream documents break (grep for `{axl_*}`, `{axc_*}`, etc. in consuming specs)
- Update MCM spec if it prescribes the shorthand form

### Benefits

- Enables formatted display text (e.g., `xref:axc_fatal[*FATAL*]` for bold)
- Opens door to richer typographic conventions in rendered specs
- xref macro is the "long form" — more explicit about what it does

### Risks

- Large diff touching every attribute entry
- Must verify all AsciiDoc processors in use handle xref macro in attribute values
- Consuming documents should be unaffected (attribute substitution happens before xref resolution)

### Context

Identified 2026-02-04 during AXLA editing session. Shorthand form prevents any formatting of replacement text.

## rbgm-payor-establish-readme
Locate or create user-friendly README for the payor establishment workflow.

### Problem

Documentation exists in `lenses/RBSPE-payor_establish.adoc` and `lenses/RBSPR-payor_refresh.adoc` (AsciiDoc format), but no user-friendly README or getting-started guide links to it from `Tools/rbw/`.

### Context

Migrated from orphaned heat `jjh_b251226-bash-tooling-cleanup.md` (item #2), 2026-02-16.

## rbags-image-list-operation
Define and scope an image registry listing operation for the Director role.

### Problem

No operation exists to enumerate available images in the repository. Need to establish operation name (candidate: something under rbw or rbgd), define scope, parameters, and integration points.

### Context

Migrated from orphaned heat `jjh_b251226-bash-tooling-cleanup.md` (item #8), 2026-02-16. Originally marked `mode: manual`.

## rbw-local-build-push
Build images locally then push to Artifact Registry for users with Director credentials.

### Problem

Current image creation flow is cloud-only:
1. Director submits build to Cloud Build
2. Mason (cloud-only) executes build within GCB
3. Mason pushes to Artifact Registry

This works but has friction points:
- Cloud Build queue adds latency during iteration
- Multi-platform buildx is awkward in Cloud Build (see RBWMBX memo)
- Simple rebuilds incur Cloud Build costs
- No offline-first workflow option

### Proposed Feature

Allow users with Director credentials to:
1. Build locally (docker/podman/buildx)
2. Push directly to Artifact Registry
3. Gate on: "has Director RBRA file" (repository write access)

### Design Questions

**Permission granularity**: Director already has repository write permissions. The question is whether "can submit Cloud Build" should gate "can push locally", or whether repository write is the real gate. These are logically distinct capabilities.

**Provenance gap**: Cloud Build creates metadata artifacts (SBOM, build transcript). Local builds would lack this audit trail. Options:
- Accept the gap for dev workflows
- Generate equivalent metadata locally
- Tag local builds distinctly (e.g., `:local-*` suffix)

**Role design**: Is this "Director can also push locally" or a new lighter-weight role (LocalBuilder) with only repository write, no Cloud Build submission?

**Tagging discipline**: Cloud Build uses deterministic tags. Local pushes need discipline to avoid stomping production tags. Perhaps enforce `:local-*` or `:dev-*` prefix for local builds.

### Use Cases

1. **Fast iteration** - Build locally, push, test in target environment without Cloud Build queue
2. **Multi-platform builds** - Use local buildx for arm64/amd64 manifests
3. **Offline-first** - Build during travel, push when connected
4. **Cost reduction** - Skip Cloud Build for trivial rebuilds during development

### Implementation Hints

- New tabtarget: `rbw-lP.LocalPush.sh` or similar
- Authenticate with Director RBRA, push via docker/podman CLI
- Validate RBRR_DIRECTOR_RBRA_FILE exists before attempting
- Enforce tag prefix policy (reject push to `:latest` or production tags)
- Consider metadata generation (build timestamp, git hash, local builder identity)

### Context

Identified 2026-01-28 during exploratory discussion. Motivating use case unclear — could be dev iteration speed, multi-platform builds, or cost savings. Worth clarifying before implementation.
