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

### Proposed Solution: `z_locale` Wrapper Pattern

Each file/module defines local wrappers following BCG `z_*` conventions:

```bash
z_locale="${0##*/}"

# Local convenience wrappers
z_step()    { buc_step "${z_locale}" "$@"; }
z_info()    { buc_info "${z_locale}" "$@"; }
z_warn()    { buc_warn "${z_locale}" "$@"; }
z_die()     { buc_die "${z_locale}" "$@"; }
z_success() { buc_success "${z_locale}" "$@"; }
```

Then call sites become: `z_step "message"` with correct file attribution in output.

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
