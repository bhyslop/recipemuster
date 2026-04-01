# Heat Trophy: jjk-v4-0-jjs0-axla-normalization

**Firemark:** ₣Aw
**Created:** 260317
**Retired:** 260331
**Status:** retired

## Paddock

## Purpose

Build V3 schema infrastructure: AXLA annotation vocabulary upgrade, JJF file exchange protocol, and officium lifecycle for gazette I/O isolation.

This heat is **strictly V3 schema**. No breaking changes, no data model rewrites.

## Completed Work

1. **JJF file exchange protocol** — markdown-based file I/O for multiline MCP parameters. **DONE** (₢AwAAO, ₢AwAAI, ₢AwAAJ).
2. **AXLA annotation migration** — `axhe*` entity voicing convention replaces transport-coupled `axl_voices`. **DONE** (₢AwAAB–₢AwAAF).
3. **Specification gaps** — formalized `jjx_close`, `jjx_paddock`, V3-legacy resolution. **DONE** (₢AwAAE).
4. **Officium lifecycle spec** — officium entity, invitatory/compline procedures, gazette exchange path. **DONE** (₢AwAAP). Subsequently redesigned (see below).

## Officium Redesign (2026-03-27)

### The Thrash Incident

₢AwAAQ originally implemented `jjdxo_invitatory` at MCP server startup (`jjrm_serve_stdio`). Claude Code desktop spawns/kills MCP server processes at unpredictable frequency — hundreds per minute during health checks and reconnection. This caused catastrophic feedback: each startup created a directory + git commit + probe (spawning parallel claude invocations), which slowed the system, triggering more restarts. Result: 1300+ officia directories, kernel load average 230+, required hard reboot.

Emergency fix landed on ₣Ah: lazy invitatory on first jjx command with 1-hour gap guard via `vvc::vvcp_invitatory()`.

### Design Pivot: The Chat Is The Session

The fundamental error: assuming MCP server process lifetime maps to "a session." It doesn't — the MCP transport layer restarts servers independently of chat sessions. The server process is disposable infrastructure, not a session boundary.

**New model**: The agent (chat) is the stable identity anchor. `jjx_open` is an explicit agent-initiated operation called once per chat. The MCP server is stateless — officium ID is a routing parameter passed on every jjx call. Server restarts are invisible.

### Key Design Decisions

- **☉ (U+2609 SUN)** — unicode verification prefix for officium identity. Evokes the Divine Office's daily cycle. Rule: ☉ in params/display, stripped for directory name (parallels ₣/₢).
- **Identity format**: `YYMMDD-NNNN` — datestamp + autonumber (enumerate existing dirs, pick next unused starting at 1000). No persistent counter file.
- **Heartbeat liveness** — every jjx dispatch touches a sentinel file in the officium directory. No coupling to process trees or Claude Code internals.
- **Exsanguination** — at `jjx_open`, scan heartbeat mtimes, reap stale directories (generous threshold). Replaces compline.
- **No compline** — sessions just stop. Staleness detected by heartbeat absence. Self-healing: if officium dir is missing, agent calls `jjx_open` again.
- **Gitignored officia/** — `.claude/jjm/officia/` is ephemeral exchange infrastructure, not tracked by git. Eliminates git noise and makes wrong exsanguination benign.
- **Daily probe** — `.probe_date` datestamp file; `vvcp_probe` runs once per calendar day, not per chat.

## Gazette Directional Split Spook (2026-03-27)

### The Spook

During ₣Av ₢AvAAV work: `jjx_orient` wrote paddock+docket to `gazette.md`. Agent then called `jjx_enroll` without overwriting the gazette. Server consumed the stale orient output as enroll input, choking on `# paddock` slug where `# slate` was expected. Recoverable but cost a round-trip.

### Root Cause

Single `gazette.md` serves as both server output channel (orient, show, paddock getter) and agent input channel (enroll, redocket, paddock setter). Stale output from a getter is indistinguishable from fresh input for a setter at the file layer.

### Fix: ₢AwAAb

Split into `gazette_in.md` (agent→server) and `gazette_out.md` (server→agent). Universal entry rule: every jjx call reads+deletes gazette_in and deletes gazette_out before dispatch. Single-MCP-call lifetime. Implementation verified 2026-03-28 across all scenarios (orient, show, paddock get/set, enroll, reslate, wrong-command discard).

## Prior Context (retained)

### Key Design Insight (2026-03-23)

The `axhe*` entity voicing convention is transport-agnostic by design. `axhems_scoped_method` doesn't care if it's served by CLI, MCP, or any future transport.

### Key Premise

**jjdk_sole_operator** — All concurrent MCP sessions belong to a single operator. The commit lock serializes their mutations; the officium exchange directory isolates their gazette file I/O.

### Heat Constellation

| Heat | Silks | Role | Status |
|------|-------|------|--------|
| ₣Aw | jjk-v4-0-jjs0-axla-normalization | V3 infrastructure — annotations + officium + gazette | Racing |
| ₣Ah | jjk-v4-1-school-breeze-founding | V4 schema transition | Racing (has emergency invitatory fix) |
| ₣An | jjk-v4-release-and-legacy-removal | V4 cleanup | Stabled |
| ₣Am | jjk-v5-notional | Future parking lot | Stabled |

## Paces

### design-entity-voicing-with-jjf-exemplar (₢AwAAM) [complete]

**[260321-1327] complete**

Design and implement AXLA entity voicing annotations — a third annotation family alongside regime (axvr_*/axhr*_) and operation (axvo_*/axho*_) — that enables subdocument-scoped class specification. Use the JJF (Job Jockey File) entity as the founding exemplar.

## Character

Design conversation requiring judgment. The AXLA voicing shapes and the JJF entity spec must co-evolve — the voicings become real only through contact with the exemplar, and the exemplar needs the voicings to express its constraints properly.

## Deliverables

1. **New AXLA voicing definitions in AXLA-Lexicon.adoc** — entity hierarchy markers enabling two-layer subdocument structure:
   - Layer 1: field markers (struct data members with type/dimension voicings) and method markers (open method scope with lifecycle dimensions)
   - Layer 2: parameter and output markers nested within method scope (multiple inputs/outputs per method)
   - Entity definition-site voicing for parent documents (the missing axv?_entity that axvo_method already cross-references)
   - Compliance rules for entity voicing completeness and hierarchy consistency

2. **JJF entity subdocument (JJSCJF-jjf-entity.adoc)** — first exemplar of the entity voicing pattern, specifying:
   - Struct fields (the internal representation of a parsed JJF document)
   - Constructor (parse: raw markdown → JJF instance)
   - Traversal methods (navigate tags, extract sections by tag type)
   - Builder methods (programmatically construct JJF documents)
   - Emitter method (JJF instance → formatted markdown, with round-trip guarantee)
   - All constrained via the new AXLA hierarchy markers, no per-method JJS0 linked terms

3. **Single JJS0 linked term** for the JJF entity itself — the only elevation to the parent document. All methods, fields, and parameters remain subdocument-scoped, demonstrating the token-efficient pattern for entity specification.

## Key Design Principle

A well-constrained entity is its own documentation. Operation procedures that reference the entity invoke its full contract — individual methods do not need independent addressability from the parent spec. This is the OO pattern: the type constrains all usage.

## Depends on

Nothing — this pace pioneers the pattern that subsequent normalization and implementation paces build on.

**[260319-1859] rough**

Design and implement AXLA entity voicing annotations — a third annotation family alongside regime (axvr_*/axhr*_) and operation (axvo_*/axho*_) — that enables subdocument-scoped class specification. Use the JJF (Job Jockey File) entity as the founding exemplar.

## Character

Design conversation requiring judgment. The AXLA voicing shapes and the JJF entity spec must co-evolve — the voicings become real only through contact with the exemplar, and the exemplar needs the voicings to express its constraints properly.

## Deliverables

1. **New AXLA voicing definitions in AXLA-Lexicon.adoc** — entity hierarchy markers enabling two-layer subdocument structure:
   - Layer 1: field markers (struct data members with type/dimension voicings) and method markers (open method scope with lifecycle dimensions)
   - Layer 2: parameter and output markers nested within method scope (multiple inputs/outputs per method)
   - Entity definition-site voicing for parent documents (the missing axv?_entity that axvo_method already cross-references)
   - Compliance rules for entity voicing completeness and hierarchy consistency

2. **JJF entity subdocument (JJSCJF-jjf-entity.adoc)** — first exemplar of the entity voicing pattern, specifying:
   - Struct fields (the internal representation of a parsed JJF document)
   - Constructor (parse: raw markdown → JJF instance)
   - Traversal methods (navigate tags, extract sections by tag type)
   - Builder methods (programmatically construct JJF documents)
   - Emitter method (JJF instance → formatted markdown, with round-trip guarantee)
   - All constrained via the new AXLA hierarchy markers, no per-method JJS0 linked terms

3. **Single JJS0 linked term** for the JJF entity itself — the only elevation to the parent document. All methods, fields, and parameters remain subdocument-scoped, demonstrating the token-efficient pattern for entity specification.

## Key Design Principle

A well-constrained entity is its own documentation. Operation procedures that reference the entity invoke its full contract — individual methods do not need independent addressability from the parent spec. This is the OO pattern: the type constrains all usage.

## Depends on

Nothing — this pace pioneers the pattern that subsequent normalization and implementation paces build on.

### implement-gazette-rust (₢AwAAO) [complete]

**[260323-0944] complete**

Implement the Gazette entity in Rust per the JJSCGZ-gazette.adoc specification.

## Scope

Create `jjrz_gazette.rs` implementing:

1. **Gazette struct** — two-level map (slug → lede → content) with vocabulary and frozen state
2. **jjrz_parse** — construct frozen gazette from markdown, collect all diagnostics as Vec<String> for LLM-legible error reporting
3. **jjrz_build** — construct unfrozen gazette with vocabulary
4. **jjrz_add** — add notice, fatal if frozen/bad slug/duplicate (slug,lede) key
5. **jjrz_query_by_slug** — retrieve entries by slug, triggers freeze-on-disclosure
6. **jjrz_query_all** — retrieve all notices, triggers freeze-on-disclosure
7. **jjrz_emit** — produce markdown, triggers freeze-on-disclosure
8. **Slug constants** — jjezs_slate, jjezs_reslate, jjezs_paddock, jjezs_pace as Rust enum

## Invariants

- Freeze-on-disclosure: any method that reveals notice map contents freezes permanently
- No ordering semantic
- Unique (slug, lede) keys, fatal on duplicate
- Round-trip: parse(vocab, emit(g)) produces identical notice map

## Testing

Unit tests for:
- Parse: valid markdown, unknown slugs, malformed headers, duplicate keys, near-match suggestions
- Build/add/freeze: vocabulary enforcement, freeze-on-disclosure, duplicate key fatality
- Round-trip: parse(emit(x)) == x
- Directionality: slug direction constants match spec

## Character

Straightforward Rust implementation from a tight spec. The JJSCGZ subdocument is authoritative.

**[260321-1328] rough**

Implement the Gazette entity in Rust per the JJSCGZ-gazette.adoc specification.

## Scope

Create `jjrz_gazette.rs` implementing:

1. **Gazette struct** — two-level map (slug → lede → content) with vocabulary and frozen state
2. **jjrz_parse** — construct frozen gazette from markdown, collect all diagnostics as Vec<String> for LLM-legible error reporting
3. **jjrz_build** — construct unfrozen gazette with vocabulary
4. **jjrz_add** — add notice, fatal if frozen/bad slug/duplicate (slug,lede) key
5. **jjrz_query_by_slug** — retrieve entries by slug, triggers freeze-on-disclosure
6. **jjrz_query_all** — retrieve all notices, triggers freeze-on-disclosure
7. **jjrz_emit** — produce markdown, triggers freeze-on-disclosure
8. **Slug constants** — jjezs_slate, jjezs_reslate, jjezs_paddock, jjezs_pace as Rust enum

## Invariants

- Freeze-on-disclosure: any method that reveals notice map contents freezes permanently
- No ordering semantic
- Unique (slug, lede) keys, fatal on duplicate
- Round-trip: parse(vocab, emit(g)) produces identical notice map

## Testing

Unit tests for:
- Parse: valid markdown, unknown slugs, malformed headers, duplicate keys, near-match suggestions
- Build/add/freeze: vocabulary enforcement, freeze-on-disclosure, duplicate key fatality
- Round-trip: parse(emit(x)) == x
- Directionality: slug direction constants match spec

## Character

Straightforward Rust implementation from a tight spec. The JJSCGZ subdocument is authoritative.

### rename-axho-markers-to-new-convention (₢AwAAN) [complete]

**[260323-1002] complete**

Rename existing operation hierarchy markers (axhop_*, axhoo_*) to match the entity voicing naming convention established in ₢AwAAM.

## Renames

- `axhop_parameter` (abstract) → eliminated (branch enforces)
- `axhop_parameter_from_type` → `axhopt_typed_parameter`
- `axhop_parameter_from_arg` → `axhopa_arg_parameter`
- `axhoo_output` (abstract) → eliminated (branch enforces)
- `axhoo_output_of_type` → `axhoot_typed_output`

## New markers

- `axhopm_motif_parameter` (0-arity)
- `axhoom_motif_output` (0-arity)

## Character

Mechanical but requires care — grep all specs for existing usages and update consistently. Check both AXLA definitions and all consuming documents.

**[260320-1739] rough**

Rename existing operation hierarchy markers (axhop_*, axhoo_*) to match the entity voicing naming convention established in ₢AwAAM.

## Renames

- `axhop_parameter` (abstract) → eliminated (branch enforces)
- `axhop_parameter_from_type` → `axhopt_typed_parameter`
- `axhop_parameter_from_arg` → `axhopa_arg_parameter`
- `axhoo_output` (abstract) → eliminated (branch enforces)
- `axhoo_output_of_type` → `axhoot_typed_output`

## New markers

- `axhopm_motif_parameter` (0-arity)
- `axhoom_motif_output` (0-arity)

## Character

Mechanical but requires care — grep all specs for existing usages and update consistently. Check both AXLA definitions and all consuming documents.

### jjk-v4-diagnose-mcp-integration (₢AwAAK) [complete]

**[260318-1612] complete**

## Character

Diagnostic and methodical. Test whether the MCP integration issue reported in /context output is a real bug or a configuration problem with the tunneled command approach.

## Context

User reported that each MCP operation appears as a separate line item in /context output, rather than being aggregated by server. Research shows this should NOT happen — MCP operations should aggregate by server per design.

User implemented a single MCP server using a tunneled command line approach (single MCP command taking a string argument). Before proceeding with the jjk-v4 normalization work, we need to determine if:

1. The tunneling approach causes the /context display problem
2. Adding a second MCP command (non-tunneled) follows correct design and fixes the issue
3. The root cause is user configuration error or an actual Claude Code bug

Finding may affect how other MCP operations in subsequent paces are structured.

## Research Summary (from 2026-03-18 session)

### Expected Behavior (By Design)
- MCP operations SHOULD be aggregated by server in /context output
- Multiple tool invocations from one server roll up to: "MCP server 'name': uses X% of context"
- Individual tool calls are NOT separately enumerated
- Token accounting is per-server rollup, not per-operation

### Project Architecture
- Single MCP server configured: `mcp__vvx__jjx`
- Multiple commands underneath (jjx_list, jjx_show, jjx_record, etc.)
- All operations should aggregate into ONE /context line item

### Potential Root Causes
1. Tunneled command architecture (string-based parameter passing) causes side effects on MCP server registration
2. Claude Code bug where operation invocations are listed instead of aggregated
3. User configuration error in settings.json or .claude.json
4. Display artifact where MCP operations are aggregated but visually prominent and push context info off-screen

## Task

1. **Create second MCP command** — add a new, non-tunneled MCP command alongside the existing tunneled one. This command should be simpler (e.g., take structured JSON parameters directly, no string tunnel). Document the implementation pattern.

2. **Test /context output** — run /context command and document:
   - How many MCP server entries appear
   - How many operation rows/line items appear
   - Whether non-tunneled command behaves differently from tunneled command
   - Token breakdown and proportions

3. **Compare behaviors** — analyze whether adding a non-tunneled command:
   - Fixes the individual-operation-rows issue
   - Introduces new issues
   - Confirms the tunneling approach is the root cause

4. **Document findings** — update heat paddock with:
   - Implementation pattern for the new non-tunneled command
   - /context output screenshots or detailed description
   - Root cause diagnosis (configuration error, bug, design issue)
   - Recommendation for subsequent paces

5. **Assess impact on ₣Aw** — determine if findings affect how JJF file exchange should be implemented. If MCP integration has limitations, JJF operations may need workarounds.

## Produces

- Working second MCP command integrated into `mcp__vvx__jjx` or as separate server
- Detailed /context output analysis with findings
- Decision: proceed with AwAAH as-is, or adjust implementation strategy
- Updated ₣Aw paddock with MCP integration diagnostics

## Sequencing

This pace runs FIRST (before AwAAB) to unblock the rest of the heat. Finding may ripple to how file exchange operations are designed/tested.

**[260318-1559] rough**

## Character

Diagnostic and methodical. Test whether the MCP integration issue reported in /context output is a real bug or a configuration problem with the tunneled command approach.

## Context

User reported that each MCP operation appears as a separate line item in /context output, rather than being aggregated by server. Research shows this should NOT happen — MCP operations should aggregate by server per design.

User implemented a single MCP server using a tunneled command line approach (single MCP command taking a string argument). Before proceeding with the jjk-v4 normalization work, we need to determine if:

1. The tunneling approach causes the /context display problem
2. Adding a second MCP command (non-tunneled) follows correct design and fixes the issue
3. The root cause is user configuration error or an actual Claude Code bug

Finding may affect how other MCP operations in subsequent paces are structured.

## Research Summary (from 2026-03-18 session)

### Expected Behavior (By Design)
- MCP operations SHOULD be aggregated by server in /context output
- Multiple tool invocations from one server roll up to: "MCP server 'name': uses X% of context"
- Individual tool calls are NOT separately enumerated
- Token accounting is per-server rollup, not per-operation

### Project Architecture
- Single MCP server configured: `mcp__vvx__jjx`
- Multiple commands underneath (jjx_list, jjx_show, jjx_record, etc.)
- All operations should aggregate into ONE /context line item

### Potential Root Causes
1. Tunneled command architecture (string-based parameter passing) causes side effects on MCP server registration
2. Claude Code bug where operation invocations are listed instead of aggregated
3. User configuration error in settings.json or .claude.json
4. Display artifact where MCP operations are aggregated but visually prominent and push context info off-screen

## Task

1. **Create second MCP command** — add a new, non-tunneled MCP command alongside the existing tunneled one. This command should be simpler (e.g., take structured JSON parameters directly, no string tunnel). Document the implementation pattern.

2. **Test /context output** — run /context command and document:
   - How many MCP server entries appear
   - How many operation rows/line items appear
   - Whether non-tunneled command behaves differently from tunneled command
   - Token breakdown and proportions

3. **Compare behaviors** — analyze whether adding a non-tunneled command:
   - Fixes the individual-operation-rows issue
   - Introduces new issues
   - Confirms the tunneling approach is the root cause

4. **Document findings** — update heat paddock with:
   - Implementation pattern for the new non-tunneled command
   - /context output screenshots or detailed description
   - Root cause diagnosis (configuration error, bug, design issue)
   - Recommendation for subsequent paces

5. **Assess impact on ₣Aw** — determine if findings affect how JJF file exchange should be implemented. If MCP integration has limitations, JJF operations may need workarounds.

## Produces

- Working second MCP command integrated into `mcp__vvx__jjx` or as separate server
- Detailed /context output analysis with findings
- Decision: proceed with AwAAH as-is, or adjust implementation strategy
- Updated ₣Aw paddock with MCP integration diagnostics

## Sequencing

This pace runs FIRST (before AwAAB) to unblock the rest of the heat. Finding may ripple to how file exchange operations are designed/tested.

### mark-volatile-v4-sections (₢AwAAA) [abandoned]

**[260318-1538] abandoned**

Survey JJS0 sections and subdocuments that ₣Ah V4 design paces will deeply rewrite. Add visible AsciiDoc comments (e.g., `// V4-VOLATILE: This section will be rewritten by ₢AhAAA design-v4-data-model`) at the top of each volatile section/subdocument.

## Purpose

Prevent future normalization paces from investing effort on doomed text, and prevent future agents from pattern-matching against pre-normalization forms in sections that will be replaced.

## Sections likely volatile

- V3 Pace data model (tack array, pace members, pace state enum) — rewritten by ₢AhAAA/₢AhAAD
- V3 validation rules referencing tack/bridle/direction — rewritten alongside type changes
- Any remaining V3-specific operation details that reference tack access patterns

## Sections likely stable

- Identity encoding (firemark/coronet)
- Equestrian vocabulary
- Deterministic serialization
- Transport (MCP/handler architecture)
- Routines (load/save/persist/wrap)
- Upper API verbs and slash commands
- Most operations (command surface persists, internal details change)

## Produces

- V4-VOLATILE markers on every section/subdocument destined for V4 rewrite
- Brief note in paddock listing what was marked and why

**[260317-1830] rough**

Survey JJS0 sections and subdocuments that ₣Ah V4 design paces will deeply rewrite. Add visible AsciiDoc comments (e.g., `// V4-VOLATILE: This section will be rewritten by ₢AhAAA design-v4-data-model`) at the top of each volatile section/subdocument.

## Purpose

Prevent future normalization paces from investing effort on doomed text, and prevent future agents from pattern-matching against pre-normalization forms in sections that will be replaced.

## Sections likely volatile

- V3 Pace data model (tack array, pace members, pace state enum) — rewritten by ₢AhAAA/₢AhAAD
- V3 validation rules referencing tack/bridle/direction — rewritten alongside type changes
- Any remaining V3-specific operation details that reference tack access patterns

## Sections likely stable

- Identity encoding (firemark/coronet)
- Equestrian vocabulary
- Deterministic serialization
- Transport (MCP/handler architecture)
- Routines (load/save/persist/wrap)
- Upper API verbs and slash commands
- Most operations (command surface persists, internal details change)

## Produces

- V4-VOLATILE markers on every section/subdocument destined for V4 rewrite
- Brief note in paddock listing what was marked and why

### spec-paddock-with-axla-voicing (₢AwAAH) [complete]

**[260318-2135] complete**

Simplify dispatch lifecycle per jjdk_sole_operator premise.

## What's Done

1. **jjdk_sole_operator premise nucleated in JJS0** — all concurrent sessions are single operator, lock
unconditionally
2. **jjsohr_handler_result eliminated from spec** — handler returns output text, not mutation discriminant
3. **jjsodp_command_lifecycle rewritten** — unconditional lock→load→call→persist→return, heat/pace
affiliation for firemark derivation
4. **Code: HandlerResult/CommitInfo deleted** — two structs gone
5. **Code: dispatch split into dispatch_heat/dispatch_pace** — firemark parsed by dispatcher, not handler
6. **Code: jjrtl_run_revise_docket simplified** — 19 lines to 8, returns Result<String, String>
7. **All 264 tests pass**

## What Remains

- Migrate relabel and drop handlers to dispatch_pace (same file, easy wins)
- Migrate all other mutating handlers incrementally
- Remove per-handler lock/load/persist scaffolding as each migrates

**[260318-2133] rough**

Simplify dispatch lifecycle per jjdk_sole_operator premise.

## What's Done

1. **jjdk_sole_operator premise nucleated in JJS0** — all concurrent sessions are single operator, lock
unconditionally
2. **jjsohr_handler_result eliminated from spec** — handler returns output text, not mutation discriminant
3. **jjsodp_command_lifecycle rewritten** — unconditional lock→load→call→persist→return, heat/pace
affiliation for firemark derivation
4. **Code: HandlerResult/CommitInfo deleted** — two structs gone
5. **Code: dispatch split into dispatch_heat/dispatch_pace** — firemark parsed by dispatcher, not handler
6. **Code: jjrtl_run_revise_docket simplified** — 19 lines to 8, returns Result<String, String>
7. **All 264 tests pass**

## What Remains

- Migrate relabel and drop handlers to dispatch_pace (same file, easy wins)
- Migrate all other mutating handlers incrementally
- Remove per-handler lock/load/persist scaffolding as each migrates

**[260318-2054] rough**

Nucleate jjsg/jjso prefix tree in JJS0 using revise_docket as the full-stack exemplar.

## Anchor Refactor — COMPLETED

Decomposed `jjrg_tally` grab-bag into spec-governed substeps. Five AXLA-voiced terms nucleated. Dispatch lifecycle added.

## What's Done

1. **Five Operation Taxonomy terms in JJS0** — group (jjsogmc), procedure (jjsoprd), 3 methods (jjsgmrd, jjsgmrp, jjsgmpt) with correct AXLA voicings
2. **Dispatch protocol in JJS0** — jjsodp_command_lifecycle (lock→load→call→persist frame) and jjsohr_handler_result (return contract)
3. **Rust primitives** — resolve_pace (shared read), prepend_tack (shared write, takes &PaceContext ADT)
4. **Composed method** — jjrg_revise_docket: pure state transform, takes basis+ts from caller, no git dependency
5. **Dispatch lifecycle impl** — jjrm_dispatch() in jjrm_mcp.rs owns lock/load/persist for revise_docket exemplar
6. **Handler refactored** — jjrtl_run_revise_docket takes &mut Gallops, returns jjrm_HandlerResult
7. **Tests** — 3 passing: resolve_pace context, docket update, empty text rejection. Zero disk I/O.
8. **Bridled auto-reset stripped** — dead code removed from spec and implementation

## What Remains — Active Design Discussion

### HandlerResult structure concerns

The user flagged that jjrm_HandlerResult + jjrm_CommitInfo feels heavy as an "apparition" — two structs, three fields. Needs discussion:
- Should these be one struct?
- Is the jjrm_ prefix wrong? (These are operation-level concepts, not MCP-module-specific)
- Does the naming create module coupling that doesn't match the spec's jjsohr_ prefix?

This is the NEXT conversation to have before proceeding.

### Remaining migration (future paces)

- Migrate relabel and drop handlers to dispatch lifecycle pattern
- Migrate other handlers (slate, draft, garland, restring, wrap) incrementally
- Pull jjrg_make_tack ambient I/O capture up to procedure boundary per exemplar pattern
- Promote jjrf_Firemark into PaceContext to eliminate final coronet re-parse

## Produces (when complete)

- Tight spec↔implementation correspondence across all layers
- Dispatch lifecycle as single implementation point
- Handlers as pure procedures receiving &mut Gallops
- Methods as pure state transforms
- Exemplar pattern for migrating all remaining operations

**[260318-2006] rough**

Nucleate jjsg/jjso prefix tree in JJS0 using revise_docket as the full-stack exemplar.

## Anchor Refactor

Decompose `jjrg_tally` (the grab-bag) into spec-governed substeps. The core structural move: split one flat operation into a **procedure** (external MCP command) that calls a **composed method**, which itself composes **two shared primitive methods**.

Today: `jjdo_revise_docket` → `jjrtl_run_revise_docket` → `jjrg_tally` (grab-bag, also serves relabel and drop)
After: `jjsoprd_revise_docket` → `jjsgmrd_revise_docket` → `jjsgmrp_resolve_pace` + policy + `jjsgmpt_prepend_tack`

The spec governs the decomposition. Future operations (relabel, drop) compose the same primitives with different policy in the middle.

## Prior Work (this pace)

- AXLA slot ordering clarified (committed): axvo_procedure, axvo_method, axvo_group now explicitly declare attribute reference slot sequence for future mechanical linting. See Tools/cmk/vov_veiled/AXLA-Lexicon.adoc lines ~1994-2060.
- Pace reslated from original paddock/curry focus after discovering curry barely touches Gallops (paddock path is derived from firemark alone).
- Pace reslated again to reflect five-term nucleation shape discovered through design conversation.

## Key Design Decisions

### Procedure vs Method
- External MCP commands = **procedures** (`//axvo_procedure`). They exist because users need commands.
- Internal Gallops operations = **methods** (`//axvo_method`). They exist because Gallops exists.
- The procedure/method distinction captures the boundary; `axd_internal` is not needed.

### Prefix Tree

New categories under `jjs` (specification domain):

```
jjs
├── jjsg    gallops (specification)
│   └── jjsgm*  methods (entity-affiliated operations on Gallops)
├── jjso    operations (MCP command surface)
│   ├── jjsop*  procedures (external MCP commands)
│   └── jjsog*  groups (operation collections)
└── jjsu    upper API (existing, unchanged)
```

Do NOT declare `jjsg_` or `jjso_` as categories — that would make jjsg/jjso terminals (minting violation). Only declare leaf categories: `jjsgm*`, `jjsop*`, `jjsog*`.

### Five Nucleation Terms

| Term | Voice | Role |
|------|-------|------|
| `jjsogmc_mcp_commands` | `//axvo_group` | Collection of MCP procedures |
| `jjsoprd_revise_docket` | `//axvo_procedure axd_transient axd_grouped` | External MCP command |
| `jjsgmrd_revise_docket` | `//axvo_method axd_transient` | Composed method: resolve → policy → prepend |
| `jjsgmrp_resolve_pace` | `//axvo_method axd_transient` | Shared read: coronet → pace context |
| `jjsgmpt_prepend_tack` | `//axvo_method axd_transient` | Shared write: tack → pace history |

### Voicing Patterns

Annotations are MCM prefix-discriminated form (`//axvo_...`), placed between anchor and definition per mcm_form_deflist / mcm_form_section.

- `//axvo_procedure`: first attr ref = procedure being defined. `axd_grouped` requires second attr ref = group term. Lifecycle dimension required.
- `//axvo_method`: first attr ref = method being defined. Second attr ref = affiliated entity (`{jjdgr_gallops}`). Lifecycle dimension required.
- `//axvo_group`: first attr ref = group being defined. Second attr ref = affiliated entity.

### Gallops Entity

The affiliated entity for group and methods is `{jjdgr_gallops}` from the existing `jjd` data model tree. Do NOT create a duplicate entity in the `jjs` tree — cross-reference the existing one.

## Scope

### 1. JJS0 Mapping Section

Add category declarations (in header comments):
```
// jjsgm*: Gallops methods (entity-affiliated operations)
// jjsop*: Operation procedures (MCP-served)
// jjsog*: Operation groups
```

Add mapping entries for all five terms.

### 2. JJS0 Definition Sites

Add a new section (peer to existing Routines and Operations sections) for the nucleation terms.

**Group** — first ref is group, second ref is affiliated entity:
```
[[jjsogmc_mcp_commands]]
//axvo_group
{jjsogmc_mcp_commands}::
The collection of user-visible operations exposed via the jjx MCP tool dispatcher, affiliated with {jjdgr_gallops}.
```

**Procedure** — first ref is procedure, second ref is group:
```
[[jjsoprd_revise_docket]]
//axvo_procedure axd_transient axd_grouped
{jjsoprd_revise_docket}::
Update a pace's docket text, member of {jjsogmc_mcp_commands}.
(Full spec with Arguments, Behavior, Exit Status sections)
```

**Composed method** — first ref is method, second ref is affiliated entity. Behavior section names substeps:
```
[[jjsgmrd_revise_docket]]
//axvo_method axd_transient
{jjsgmrd_revise_docket}::
Pure state transform on {jjdgr_gallops} that updates a pace's docket text.
Composes {jjsgmrp_resolve_pace}, applies bridled auto-reset policy, then {jjsgmpt_prepend_tack}.
```

**Resolve pace** — shared read primitive:
```
[[jjsgmrp_resolve_pace]]
//axvo_method axd_transient
{jjsgmrp_resolve_pace}::
Navigate {jjdgr_gallops} from a {jjdt_coronet} to the target {jjdpr_pace} and its current tack state.
Returns pace context: parsed coronet, firemark, current state/text/silks/direction.
```

**Prepend tack** — shared write primitive:
```
[[jjsgmpt_prepend_tack]]
//axvo_method axd_transient
{jjsgmpt_prepend_tack}::
Insert a new tack at position zero of a {jjdpr_pace_p} tack history within {jjdgr_gallops}.
```

Leave existing `jjdo_revise_docket` (old pattern) undisturbed pending retirement decision.

### 3. Rust Implementation

**Shared primitive: resolve_pace**
- New method on `jjrg_Gallops`: `jjrg_resolve_pace(&self, coronet: &str) -> Result<PaceContext, String>`
- Returns struct with parsed coronet, firemark key, current state/text/silks/direction (cloned from current tack)
- Extracts steps 1-4 of current `jjrg_tally`

**Shared primitive: prepend_tack**
- New method on `jjrg_Gallops`: `jjrg_prepend_tack(&mut self, coronet: &str, tack: jjrg_Tack) -> Result<(), String>`
- Parses coronet, finds pace, inserts tack at position 0
- Extracts step 9 of current `jjrg_tally`

**Composed method: revise_docket**
- New method on `jjrg_Gallops`: `jjrg_revise_docket(&mut self, coronet: &str, docket: &str) -> Result<PaceContext, String>`
- Calls `resolve_pace`, applies bridled auto-reset policy, creates tack, calls `prepend_tack`
- Returns PaceContext so handler has firemark/silks for commit message (eliminates duplicate coronet parse in handler)

**Handler update**
- Update `jjrtl_run_revise_docket` in `jjrtl_tally.rs` to call `gallops.jjrg_revise_docket()` instead of constructing TallyArgs
- Handler becomes: lock → load → call method → persist (clean separation)

**`jjrg_tally` stays** for relabel/drop — they get their own extraction in future paces.

### 4. Test

- Construct `jjrg_Gallops` from stack immediates (struct literals with hand-built tacks — no `jjrg_make_tack`, no git, no disk)
- Call `jjrg_resolve_pace` — assert correct context returned
- Call `jjrg_revise_docket` — assert docket updated, tack prepended, bridled auto-reset works
- Demonstrates zero-disk-I/O testing pattern enabled by the method extraction

### 5. Build and Verify

- `tt/vow-b.Build.sh`
- `tt/vow-t.Test.sh`

## Produces

- Five nucleation terms in JJS0 with correct AXLA voicings (group, procedure, 3 methods)
- New jjsg/jjso prefix tree categories
- Two shared Gallops primitives (resolve_pace, prepend_tack)
- One composed Gallops method (revise_docket) that demonstrates the composition pattern
- Handler simplified to: lock → load → method → persist
- Tests validating the primitives and composition without disk I/O
- Exemplar pattern for migrating remaining operations (relabel, drop) in future paces

## Character

Precise and structural. This is recovery work — decomposing a grab-bag that earned its decomposition. The spec governs the decomposition: every Rust method maps to a spec term, every composition is named in behavior sections. The more daylight we close between spec and implementation, the less state-space froth we carry forward.

**[260318-1940] rough**

Nucleate jjsg/jjso prefix tree in JJS0 using revise_docket as the full-stack exemplar.

## Prior Work (this pace)

- AXLA slot ordering clarified (committed): axvo_procedure, axvo_method, axvo_group now explicitly declare attribute reference slot sequence for future mechanical linting. See Tools/cmk/vov_veiled/AXLA-Lexicon.adoc lines ~1994-2060.
- Pace reslated from original paddock/curry focus after discovering curry barely touches Gallops (paddock path is derived from firemark alone).

## Key Design Decisions (established in conversation)

### Procedure vs Method (replaces old routine concept)
- `axo_routine` / `axvo_routine` were REMOVED from AXLA. No routine concept exists.
- External MCP commands = **procedures** (`//axvo_procedure`). They exist because users need commands.
- Internal Gallops operations = **methods** (`//axvo_method`). They exist because Gallops exists.
- `axd_internal` dimension is available but NOT needed here — the procedure/method distinction already captures the boundary.

### Prefix Tree

New categories under `jjs` (specification domain), parallel to existing `jjsu` (upper API):

```
jjs
├── jjsg    gallops (specification)
│   └── jjsgm*  methods (entity-affiliated operations on Gallops)
├── jjso    operations (MCP command surface)
│   ├── jjsop*  procedures (external MCP commands)
│   └── jjsog*  groups (operation collections)
└── jjsu    upper API (existing, unchanged)
```

NOTE: Do NOT declare `jjsg_` or `jjso_` as categories — that would make jjsg/jjso terminals (minting violation). Only declare leaf categories: `jjsgm*`, `jjsop*`, `jjsog*`.

Prefixes use acronym identification per minting preference:
- `jjsgmXX_name` where XX identifies the specific method
- `jjsopXX_name` where XX identifies the specific procedure
- `jjsogXX_name` where XX identifies the specific group

### Voicing Patterns

Annotations are MCM prefix-discriminated form (`//axvo_...`), placed between anchor and definition per mcm_form_deflist / mcm_form_section.

`//axvo_procedure` requires first attribute reference = procedure being defined. `axd_grouped` dimension requires second attribute reference = a term voiced `//axvo_group`. Lifecycle dimension is REQUIRED (exactly one of axd_transient, axd_longrunning, axd_periodic). Revise_docket is `axd_transient`.

`//axvo_method` requires first attribute reference = method being defined. Second attribute reference = affiliated entity (voiced `//axo_entity` or equivalent). Lifecycle dimension also required — `axd_transient` for revise_docket.

`//axvo_group` requires first attribute reference = group being defined. Second attribute reference = affiliated entity.

### Gallops Entity

The affiliated entity for both group and methods is `{jjdgr_gallops}` from the existing `jjd` data model tree. Do NOT create a duplicate entity in the `jjs` tree — cross-reference the existing one.

## Scope

### 1. JJS0 Mapping Section

Add category declarations (in header comments):
```
// jjsgm*: Gallops methods (entity-affiliated operations)
// jjsop*: Operation procedures (MCP-served)
// jjsog*: Operation groups
```

Add mapping entries:
```
:jjsogmc_mcp_commands:       <<jjsogmc_mcp_commands,MCP Commands>>
:jjsoprd_revise_docket:      <<jjsoprd_revise_docket,Revise Docket>>
:jjsgmrd_revise_docket:      <<jjsgmrd_revise_docket,Revise Docket Method>>
```

### 2. JJS0 Definition Sites

Add a new section (peer to existing Routines and Operations sections) for the nucleation terms.

Group definition — first ref is group, second ref is affiliated entity:
```
[[jjsogmc_mcp_commands]]
//axvo_group
{jjsogmc_mcp_commands}::
The collection of user-visible operations exposed via the jjx MCP tool dispatcher, affiliated with {jjdgr_gallops}.
```

Procedure definition — first ref is procedure, second ref is group (required by axd_grouped):
```
[[jjsoprd_revise_docket]]
//axvo_procedure axd_transient axd_grouped
{jjsoprd_revise_docket}::
Update a pace's docket text, member of {jjsogmc_mcp_commands}.
(Full spec with Arguments, Behavior, Exit Status sections)
```

Method definition — first ref is method, second ref is affiliated entity:
```
[[jjsgmrd_revise_docket]]
//axvo_method axd_transient
{jjsgmrd_revise_docket}::
Pure state transform on {jjdgr_gallops} that updates a pace's docket text.
(Spec the mutation: parse coronet, find heat/pace, handle bridled auto-reset, create new tack, prepend)
```

Leave existing `jjdo_revise_docket` (old pattern, `//axl_voices axi_cli_subcommand`) undisturbed pending retirement decision.

### 3. Rust Method Extraction

Rust naming stays `jjr*` (separate from JJS0 `jjs*` naming).

- Extract focused method from multi-purpose `jjrg_tally` in `jjro_ops.rs` (line ~297)
- New method on `jjrg_Gallops` (the inner type in `jjrt_types.rs`, NOT `jjdr_ValidatedGallops`): takes `(&mut self, coronet: &str, docket: &str) -> Result<(), String>`
- Add to existing impl block in `jjrg_gallops.rs` (lines 50-108) alongside other thin wrappers
- Extracts the subset of tally logic for docket update: parse coronet, find pace, handle bridled auto-reset to rough, create new tack with updated text, prepend
- `jjrg_tally` stays for relabel/drop — they get their own extraction in future paces
- Update handler `jjrtl_run_revise_docket` in `jjrtl_tally.rs` to call focused method instead of constructing TallyArgs

### 4. Test

- Add unit test constructing `jjrg_Gallops` from stack immediates (the inner struct, not the ValidatedGallops wrapper)
- Call the focused revise_docket method directly on the constructed Gallops
- Assert docket is updated, tack is prepended
- Demonstrates zero-disk-I/O testing pattern

### 5. Build and Verify

- `tt/vow-b.Build.sh`
- `tt/vow-t.Test.sh`

## Produces

- Three nucleation terms in JJS0 with correct AXLA voicings (group, procedure, method)
- New jjsg/jjso prefix tree categories
- A focused Gallops method replacing multi-purpose tally dispatch for revise_docket
- A test demonstrating stack-immediate Gallops construction
- Exemplar pattern for migrating remaining operations in future paces

## Character

Precise and structural. Spec work is architectural (voicing discipline, prefix minting). Code work is mechanical extraction. Test work validates the design (methods testable without disk I/O). This is nucleation — plant the correct structure, let future work converge toward it.

**[260318-1934] rough**

Nucleate jjsg/jjso prefix tree in JJS0 using revise_docket as the full-stack exemplar.

## Prior Work (this pace)

- AXLA slot ordering clarified (committed): axvo_procedure, axvo_method, axvo_group now explicitly declare attribute reference slot sequence for future mechanical linting. See Tools/cmk/vov_veiled/AXLA-Lexicon.adoc lines ~1994-2060.
- Pace reslated from original paddock/curry focus after discovering curry barely touches Gallops (paddock path is derived from firemark alone).

## Key Design Decisions (established in conversation)

### Procedure vs Method (replaces old routine concept)
- `axo_routine` / `axvo_routine` were REMOVED from AXLA. No routine concept exists.
- External MCP commands = **procedures** (`//axvo_procedure`). They exist because users need commands.
- Internal Gallops operations = **methods** (`//axvo_method`). They exist because Gallops exists.
- `axd_internal` dimension is available but NOT needed here — the procedure/method distinction already captures the boundary.

### Prefix Tree

New categories under `jjs` (specification domain), parallel to existing `jjsu` (upper API):

```
jjs
├── jjsg    gallops (specification)
│   └── jjsgm*  methods (entity-affiliated operations on Gallops)
├── jjso    operations (MCP command surface)
│   ├── jjsop*  procedures (external MCP commands)
│   └── jjsog*  groups (operation collections)
└── jjsu    upper API (existing, unchanged)
```

NOTE: Do NOT declare `jjsg_` or `jjso_` as categories — that would make jjsg/jjso terminals (minting violation). Only declare leaf categories: `jjsgm*`, `jjsop*`, `jjsog*`.

Prefixes use acronym identification per minting preference:
- `jjsgmXX_name` where XX identifies the specific method
- `jjsopXX_name` where XX identifies the specific procedure
- `jjsogXX_name` where XX identifies the specific group

### Voicing Patterns

Annotations are MCM prefix-discriminated form (`//axvo_...`), placed between anchor and definition per mcm_form_deflist / mcm_form_section.

`//axvo_procedure` requires first attribute reference = procedure being defined. `axd_grouped` dimension requires second attribute reference = a term voiced `//axvo_group`.

`//axvo_method` requires first attribute reference = method being defined. Second attribute reference = affiliated entity (voiced `//axo_entity` or equivalent).

`//axvo_group` requires first attribute reference = group being defined. Second attribute reference = affiliated entity.

### Gallops Entity

The affiliated entity for both group and methods is `{jjdgr_gallops}` from the existing `jjd` data model tree. Do NOT create a duplicate entity in the `jjs` tree — cross-reference the existing one.

## Scope

### 1. JJS0 Mapping Section

Add category declarations (in header comments):
```
// jjsgm*: Gallops methods (entity-affiliated operations)
// jjsop*: Operation procedures (MCP-served)
// jjsog*: Operation groups
```

Add mapping entries:
```
:jjsogmc_mcp_commands:       <<jjsogmc_mcp_commands,MCP Commands>>
:jjsoprd_revise_docket:      <<jjsoprd_revise_docket,Revise Docket>>
:jjsgmrd_revise_docket:      <<jjsgmrd_revise_docket,Revise Docket Method>>
```

### 2. JJS0 Definition Sites

Add a new section (peer to existing Routines and Operations sections) for the nucleation terms:

Group definition:
```
[[jjsogmc_mcp_commands]]
//axvo_group
{jjsogmc_mcp_commands}::
The collection of user-visible operations exposed via the jjx MCP tool dispatcher, affiliated with {jjdgr_gallops}.
```

Procedure definition:
```
[[jjsoprd_revise_docket]]
//axvo_procedure axd_grouped
{jjsoprd_revise_docket}::
Update a pace's docket text, member of {jjsogmc_mcp_commands}.
(Full spec with Arguments, Behavior, Exit Status sections)
```

Method definition:
```
[[jjsgmrd_revise_docket]]
//axvo_method
{jjsgmrd_revise_docket}::
Pure state transform on {jjdgr_gallops} that updates a pace's docket text.
(Spec the mutation: parse coronet, find heat/pace, handle bridled auto-reset, create new tack, prepend)
```

Leave existing `jjdo_revise_docket` (old pattern, `//axl_voices axi_cli_subcommand`) undisturbed.

### 3. Rust Method Extraction

Rust naming stays `jjr*` (separate from JJS0 `jjs*` naming).

- Extract focused method from multi-purpose `jjrg_tally` in `jjro_ops.rs` (line ~297)
- New method on `jjrg_Gallops` impl: takes `(&mut self, coronet: &str, docket: &str) -> Result<(), String>`
- Extracts the subset of tally logic for docket update: parse coronet, find pace, handle bridled auto-reset to rough, create new tack with updated text, prepend
- `jjrg_tally` stays for relabel/drop — they get their own extraction in future paces
- Update handler `jjrtl_run_revise_docket` in `jjrtl_tally.rs` to call focused method instead of constructing TallyArgs

### 4. Test

- Add unit test constructing Gallops from stack immediates using `jjdr_ValidatedGallops::test_wrap()`
- Call the focused revise_docket method
- Assert docket is updated, tack is prepended
- Demonstrates zero-disk-I/O testing pattern

### 5. Build and Verify

- `tt/vow-b.Build.sh`
- `tt/vow-t.Test.sh`

## Produces

- Three nucleation terms in JJS0 with correct AXLA voicings (group, procedure, method)
- New jjsg/jjso prefix tree categories
- A focused Gallops method replacing multi-purpose tally dispatch for revise_docket
- A test demonstrating stack-immediate Gallops construction
- Exemplar pattern for migrating remaining operations in future paces

## Character

Precise and structural. Spec work is architectural (voicing discipline, prefix minting). Code work is mechanical extraction. Test work validates the design (methods testable without disk I/O). This is nucleation — plant the correct structure, let future work converge toward it.

**[260318-1927] rough**

Nucleate the jjsg/jjso prefix tree in JJS0 using revise_docket as the full-stack exemplar.

## Scope

1. **AXLA clarification** — slot ordering for axvo_procedure, axvo_method, axvo_group (DONE)

2. **JJS0 prefix nucleation** — add jjsg_ (gallops methods), jjso_ (operations: procedures + groups) category declarations and mapping entries:
   - `jjsog_mcp_commands` — `//axvo_group` affiliated with `{jjdgr_gallops}`
   - `jjsop_revise_docket` — `//axvo_procedure axd_grouped` referencing `{jjsog_mcp_commands}`
   - `jjsgm_revise_docket` — `//axvo_method` affiliated with `{jjdgr_gallops}`

3. **Rust method extraction** — extract focused `jjsgm_revise_docket(&mut self, coronet, docket)` from multi-purpose `jjrg_tally`, update handler to call it

4. **Tests** — add unit test constructing Gallops from stack immediates, calling the method, asserting docket update

5. **Build and verify** — tt/vow-b.Build.sh, tt/vow-t.Test.sh

## Produces

- Three nucleation terms in JJS0 with correct AXLA voicings
- A focused Gallops method replacing multi-purpose tally dispatch for this operation
- A test demonstrating stack-immediate Gallops construction
- Exemplar pattern for migrating remaining operations in future paces

## Character

Precise and structural. The spec work is architectural (voicing discipline, prefix minting). The code work is mechanical extraction with clear boundaries. The test work validates the design decision (methods testable without disk I/O).

**[260318-1807] rough**

Formalize and tightly constrain the paddock operation using AXLA concepts (procedure vs routine).

## Scope

Use AXLA voicing discipline to clarify the architectural boundary between user-visible operation and internal implementation.

1. **Spec the procedure layer** — `[[jjdo_paddock]]` is a {axo_procedure}: standalone, user-visible MCP command
   - Voiced as `//axvo_procedure` at definition site
   - Parameters: firemark (required), content (optional), note (optional)
   - Behavior: getter mode (no content → display paddock); setter mode (with content → update paddock)
   - Completion semantics: exit code 0/nonzero, output buffer
   - Constraints: procedure owns the decision logic, error handling at the procedure boundary

2. **Spec the routine layer** — `[[jjdr_curry]]` is an internal {axo_routine}: implementation detail that executes the procedure
   - Voiced as `//axvo_routine axd_internal` at definition site
   - Input: firemark, content (optional), note (optional), output buffer
   - Responsibility: file I/O, JSON mutation, lock acquisition
   - Constraint: routine is called ONLY by the procedure; no external callers
   - Naming: decide whether to keep curry name or rename to jjrpd_paddock to match procedure name

3. **AXLA voicing as constraint** — the annotations enforce architectural discipline:
   - {axvo_procedure} means: this is the public contract, don't change signature without versioning
   - {axvo_routine} means: this is internal plumbing, can be refactored as long as the procedure's contract holds
   - {axd_internal} means: not reusable elsewhere; if another operation needs this logic, refactor to a separate routine

4. **Update CLAUDE.md** — reference both the procedure and routine in verb table, clarifying the layering to users.

## Produces

- `[[jjdo_paddock]]` procedure specification with `//axvo_procedure` voicing
- `[[jjdr_curry]]` routine specification with `//axvo_routine axd_internal` voicing (or renamed jjrpd_curry if module is renamed)
- Clear documentation of get/set modes and parameter semantics
- Explicit architectural boundary: procedure (contract) vs routine (implementation)
- This becomes the constraint baseline that subsequent file exchange design (AwAAG) respects

**[260318-1557] rough**

Formalize and tightly constrain the paddock operation using AXLA concepts (procedure vs routine).

## Scope

Use AXLA voicing discipline to clarify the architectural boundary between user-visible operation and internal implementation.

1. **Spec the procedure layer** — `[[jjdo_paddock]]` is a {axo_procedure}: standalone, user-visible MCP command
   - Voiced as `//axvo_procedure` at definition site
   - Parameters: firemark (required), content (optional), note (optional)
   - Behavior: getter mode (no content → display paddock); setter mode (with content → update paddock)
   - Completion semantics: exit code 0/nonzero, output buffer
   - Constraints: procedure owns the decision logic, error handling at the procedure boundary

2. **Spec the routine layer** — `[[jjdr_curry]]` is an internal {axo_routine}: implementation detail that executes the procedure
   - Voiced as `//axvo_routine axd_internal` at definition site
   - Input: firemark, content (optional), note (optional), output buffer
   - Responsibility: file I/O, JSON mutation, lock acquisition
   - Constraint: routine is called ONLY by the procedure; no external callers
   - Naming: decide whether to keep curry name or rename to jjrpd_paddock to match procedure name

3. **AXLA voicing as constraint** — the annotations enforce architectural discipline:
   - {axvo_procedure} means: this is the public contract, don't change signature without versioning
   - {axvo_routine} means: this is internal plumbing, can be refactored as long as the procedure's contract holds
   - {axd_internal} means: not reusable elsewhere; if another operation needs this logic, refactor to a separate routine

4. **Update CLAUDE.md** — reference both the procedure and routine in verb table, clarifying the layering to users.

## Produces

- `[[jjdo_paddock]]` procedure specification with `//axvo_procedure` voicing
- `[[jjdr_curry]]` routine specification with `//axvo_routine axd_internal` voicing (or renamed jjrpd_curry if module is renamed)
- Clear documentation of get/set modes and parameter semantics
- Explicit architectural boundary: procedure (contract) vs routine (implementation)
- This becomes the constraint baseline that subsequent file exchange design (AwAAG) respects

**[260318-1546] rough**

Formalize and annotate the current jjx_paddock operation with modern AXLA style.

## Scope

1. **Clarify the paddock operation** — spec `[[jjdo_paddock]]` documenting:
   - Current behavior: getter mode (no content param) displays paddock; setter mode (with content param) updates paddock
   - Parameters: firemark (required), content (optional), note (optional)
   - Dual-mode operation signature and behavior
   - Implementation is internally called "curry" (resolve naming, decide whether to rename module to match)

2. **AXLA annotations** — apply modern voicing to the operation spec:
   - Interface motif: appropriate MCP tool annotation (not axi_cli_subcommand)
   - Dimension modifiers: transient, attended, or grouped as applicable
   - Routine annotations if curry is specified as a named `[[jjdr_*]]` routine

3. **Update CLAUDE.md** — if internal naming changes, update verb table and references accordingly.

## Produces

- `[[jjdo_paddock]]` operation specification (currently missing from JJS0)
- Modern AXLA annotations on the operation
- Clear documentation of get/set modes and parameter semantics
- This becomes the baseline that ₢AwAAG will design file exchange modifications against

## Notes

This pace establishes the current paddock arrangement in clear, modern form. Subsequent paces (AwAAG, AwAAI, AwAAJ) will design and implement file exchange modifications to this operation.

**[260318-1535] rough**

Implement and test the JJF parsing and emitting routines specified in ₢AwAAG.

## Scope

Build `jjrpf_parse_jjf` and `jjremit_emit_jjf` Rust routines per JJS0 specification. Test coverage includes:

1. **Parser tests** — parse all 5 tag types (jjfids_slate, jjfidr_reslate, jjfip_paddocks, jjfip_paddocks output, jjfids_slate output)
2. **Emitter tests** — emit all 5 tag types with correct formatting
3. **Round-trip tests** — parse(emit(x)) == x for all tag types
4. **Error cases** — invalid tags, missing required labels, malformed sections, empty content, whitespace edge cases
5. **Edge cases** — tags at EOF, consecutive tags without blank lines, subsections in content, multiline labels

## Produces

- `jjrpf_parse_jjf.rs` — parser routine implementation
- `jjremit_emit_jjf.rs` — emitter routine implementation
- `jjtpf_parse_jjf.rs` — comprehensive unit test module (all cases listed above)
- Both routines used by subsequent input/output implementation paces

## Depends on

₢AwAAG (design-v4-mcp-file-exchange) — needs JJF spec and procedure definitions

### retire-jjdo-revise-docket (₢AwAAL) [complete]

**[260323-1004] complete**

Delete `jjdo_revise_docket` from JJS0. Remove its mapping entry, anchor, definition site, and voicing. Update all references within JJS0 to use `{jjsoprd_revise_docket}` instead. This sets the precedent: each migrated operation deletes its old `jjdo_*` term, gradually emptying the legacy Operations section as the new jjsop/jjsgm tree fills.

**[260318-1941] rough**

Delete `jjdo_revise_docket` from JJS0. Remove its mapping entry, anchor, definition site, and voicing. Update all references within JJS0 to use `{jjsoprd_revise_docket}` instead. This sets the precedent: each migrated operation deletes its old `jjdo_*` term, gradually emptying the legacy Operations section as the new jjsop/jjsgm tree fills.

### design-v4-mcp-file-exchange (₢AwAAG) [abandoned]

**[260321-1333] abandoned**

Design and specify the Job Jockey File (JJF) format for markdown-based file exchange with MCP tools.

## Scope

1. **JJF format definition** — document the tagged markdown structure (top-level sections with `# <<jjfi-tag>> [label]`), tag semantics, label requirements per tag type, subsection handling rules.

2. **Parsing procedure** — specify `[[jjdr_parse_jjf]]` routine with AXLA annotations:
   - Input: raw markdown file content
   - Output: structured tag→content mapping
   - Error handling: fail-fast on invalid tags, missing required labels, malformed sections
   - Semantics: which tags are allowed for which operations

3. **Emitting procedure** — specify `[[jjdr_emit_jjf]]` routine with AXLA annotations:
   - Input: structured tag→content mapping
   - Output: properly formatted markdown file content
   - Round-trip guarantee: parse(emit(x)) == x

4. **Design modifications to operations** — for jjx_enroll, jjx_revise_docket, and jjx_paddock (now formalized in ₢AwAAH), design how file exchange input will work. For jjx_orient, jjx_show --detail, and jjx_paddock getter (now formalized in ₢AwAAH), design how file exchange output will work.

5. **Update CLAUDE.md verb table** — document file exchange behavior for all 6 affected operations and their corresponding JJF input/output tags.

## JJF Tags

Input tags (3 total):
- `jjfids_slate <<silks>>` — pace creation docket
- `jjfidr_reslate <<coronet>>` — pace docket update
- `jjfip_paddocks <<firemark>>` — heat paddock update

Output tags (2 total, same tag names, context from directory):
- `jjfip_paddocks <<firemark>>` — paddock content from queries
- `jjfids_slate <<silks>>` — pace docket from queries

## Produces

- JJF format section in JJS0 with tagged markdown examples
- `[[jjdr_parse_jjf]]` and `[[jjdr_emit_jjf]]` routine specifications with AXLA annotations
- Design documentation for how file exchange modifies jjx_enroll, jjx_revise_docket, jjx_paddock, jjx_orient, jjx_show, jjx_paddock (output)
- Updated CLAUDE.md verb table documenting file exchange for all 6 operations

## Depends on

₢AwAAH (clarify-and-annotate-paddocks) — establishes the baseline paddock operation that file exchange will modify

**[260318-1546] rough**

Design and specify the Job Jockey File (JJF) format for markdown-based file exchange with MCP tools.

## Scope

1. **JJF format definition** — document the tagged markdown structure (top-level sections with `# <<jjfi-tag>> [label]`), tag semantics, label requirements per tag type, subsection handling rules.

2. **Parsing procedure** — specify `[[jjdr_parse_jjf]]` routine with AXLA annotations:
   - Input: raw markdown file content
   - Output: structured tag→content mapping
   - Error handling: fail-fast on invalid tags, missing required labels, malformed sections
   - Semantics: which tags are allowed for which operations

3. **Emitting procedure** — specify `[[jjdr_emit_jjf]]` routine with AXLA annotations:
   - Input: structured tag→content mapping
   - Output: properly formatted markdown file content
   - Round-trip guarantee: parse(emit(x)) == x

4. **Design modifications to operations** — for jjx_enroll, jjx_revise_docket, and jjx_paddock (now formalized in ₢AwAAH), design how file exchange input will work. For jjx_orient, jjx_show --detail, and jjx_paddock getter (now formalized in ₢AwAAH), design how file exchange output will work.

5. **Update CLAUDE.md verb table** — document file exchange behavior for all 6 affected operations and their corresponding JJF input/output tags.

## JJF Tags

Input tags (3 total):
- `jjfids_slate <<silks>>` — pace creation docket
- `jjfidr_reslate <<coronet>>` — pace docket update
- `jjfip_paddocks <<firemark>>` — heat paddock update

Output tags (2 total, same tag names, context from directory):
- `jjfip_paddocks <<firemark>>` — paddock content from queries
- `jjfids_slate <<silks>>` — pace docket from queries

## Produces

- JJF format section in JJS0 with tagged markdown examples
- `[[jjdr_parse_jjf]]` and `[[jjdr_emit_jjf]]` routine specifications with AXLA annotations
- Design documentation for how file exchange modifies jjx_enroll, jjx_revise_docket, jjx_paddock, jjx_orient, jjx_show, jjx_paddock (output)
- Updated CLAUDE.md verb table documenting file exchange for all 6 operations

## Depends on

₢AwAAH (clarify-and-annotate-paddocks) — establishes the baseline paddock operation that file exchange will modify

**[260318-1535] rough**

Design and specify the Job Jockey File (JJF) format for markdown-based file exchange with MCP tools.

## Scope

1. **JJF format definition** — document the tagged markdown structure (top-level sections with `# <<jjfi-tag>> [label]`), tag semantics, label requirements per tag type, subsection handling rules.

2. **Parsing procedure** — specify `[[jjdr_parse_jjf]]` routine with AXLA annotations:
   - Input: raw markdown file content
   - Output: structured tag→content mapping
   - Error handling: fail-fast on invalid tags, missing required labels, malformed sections
   - Semantics: which tags are allowed for which operations

3. **Emitting procedure** — specify `[[jjdr_emit_jjf]]` routine with AXLA annotations:
   - Input: structured tag→content mapping
   - Output: properly formatted markdown file content
   - Round-trip guarantee: parse(emit(x)) == x

4. **Formalize jjx_curry** — add `[[jjdo_curry]]` operation spec defining getter/setter modes and file exchange behavior.

5. **Update CLAUDE.md verb table** — document file exchange behavior for all 6 affected operations: jjx_enroll, jjx_revise_docket, jjx_curry, jjx_orient, jjx_show, and their corresponding JJF input/output tags.

## JJF Tags

Input tags (5 total):
- `jjfids_slate <<silks>>` — pace creation docket
- `jjfidr_reslate <<coronet>>` — pace docket update
- `jjfip_paddocks <<firemark>>` — heat paddock update

Output tags (2 total, same tag names, context from directory):
- `jjfip_paddocks <<firemark>>` — paddock content from queries
- `jjfids_slate <<silks>>` — pace docket from queries

## Produces

- JJF format section in JJS0 with tagged markdown examples
- `[[jjdr_parse_jjf]]` and `[[jjdr_emit_jjf]]` routine specifications with AXLA annotations
- `[[jjdo_curry]]` operation specification (currently missing from spec)
- Updated CLAUDE.md verb table documenting file exchange for all 6 operations

**[260318-1505] rough**

Drafted from ₢AhAAW in ₣Ah.

Define the MCP file exchange protocol in JJS0: how jjx communicates prose-length content (dockets, paddocks, warrants, provender) via filesystem rather than inline JSON string params.

Scope:

1. **Exchange directory announcement** -- jjx announces two exchange directories (in/ and out/) via MCP instructions field at server initialization. Paths available to client for the session lifetime.
2. **Input discipline** -- client writes exactly one file to in/ before invoking a command that needs prose input. jjx validates exactly one file present (fail otherwise), reads it, deletes it. Commands not needing prose input find in/ empty.
3. **Output discipline** -- every jjx invocation deletes any file in out/ first, unconditionally. If the command produces prose output, jjx writes one file to out/. Client reads before next invocation or loses it.
4. **Inline param coexistence** -- short scalar params (coronet, firemark, silks, boolean flags) remain inline MCP params. File exchange is for prose-length markdown content only.
5. **Command surface impact** -- identify which existing and V4 commands gain file exchange semantics: paddock content/note, revise_docket docket, enroll docket, show/orient output, provender emission.
6. **JJS0 section** -- formal specification of the protocol as infrastructure used by all subsequent command specs.

Design constraints:
- One file at a time per direction, no naming convention needed
- jjx owns lifecycle (delete on read for input, delete on entry for output)
- Motivated by MCP JSON serialization bug (Claude Code issue 5504) but the pattern is independently good architecture -- prose documents deserve document treatment
- Exchange directories are ephemeral per session

Depends on: none (infrastructure pattern, no schema dependencies)

Produces:
- JJS0 section: MCP File Exchange Protocol

Not bridleable: Design conversation -- command surface impact assessment requires judgment about which params qualify as prose-length.

**[260317-2116] rough**

Define the MCP file exchange protocol in JJS0: how jjx communicates prose-length content (dockets, paddocks, warrants, provender) via filesystem rather than inline JSON string params.

Scope:

1. **Exchange directory announcement** -- jjx announces two exchange directories (in/ and out/) via MCP instructions field at server initialization. Paths available to client for the session lifetime.
2. **Input discipline** -- client writes exactly one file to in/ before invoking a command that needs prose input. jjx validates exactly one file present (fail otherwise), reads it, deletes it. Commands not needing prose input find in/ empty.
3. **Output discipline** -- every jjx invocation deletes any file in out/ first, unconditionally. If the command produces prose output, jjx writes one file to out/. Client reads before next invocation or loses it.
4. **Inline param coexistence** -- short scalar params (coronet, firemark, silks, boolean flags) remain inline MCP params. File exchange is for prose-length markdown content only.
5. **Command surface impact** -- identify which existing and V4 commands gain file exchange semantics: paddock content/note, revise_docket docket, enroll docket, show/orient output, provender emission.
6. **JJS0 section** -- formal specification of the protocol as infrastructure used by all subsequent command specs.

Design constraints:
- One file at a time per direction, no naming convention needed
- jjx owns lifecycle (delete on read for input, delete on entry for output)
- Motivated by MCP JSON serialization bug (Claude Code issue 5504) but the pattern is independently good architecture -- prose documents deserve document treatment
- Exchange directories are ephemeral per session

Depends on: none (infrastructure pattern, no schema dependencies)

Produces:
- JJS0 section: MCP File Exchange Protocol

Not bridleable: Design conversation -- command surface impact assessment requires judgment about which params qualify as prose-length.

### implement-jjf-input-operations (₢AwAAI) [complete]

**[260323-1033] complete**

Implement gazette-based input for write operations: jjx_enroll (slate), jjx_revise_docket (reslate), and jjx_paddock setter.

## Scope

Wire gazette parsing into three input-consuming operations:

1. **jjx_enroll** — accept gazette with {jjezs_slate} slug. Parse input, extract silks from lede, docket from content, invoke enroll logic.

2. **jjx_revise_docket** — accept gazette with {jjezs_reslate} slug. Parse input, extract coronet from lede, docket from content, invoke revise logic. Support mass reslate: multiple {jjezs_reslate} notices in one gazette, each with a different coronet lede.

3. **jjx_paddock setter** — accept gazette with {jjezs_paddock} slug. Parse input, extract firemark from lede, content from body, invoke paddock write logic.

## Testing

For each operation: valid input, error cases (unknown slugs, missing ledes, malformed content), gazette diagnostic quality for LLM consumption.

## Depends on

₢AwAAO (implement-gazette-rust) — needs working jjrz_parse

**[260321-1328] rough**

Implement gazette-based input for write operations: jjx_enroll (slate), jjx_revise_docket (reslate), and jjx_paddock setter.

## Scope

Wire gazette parsing into three input-consuming operations:

1. **jjx_enroll** — accept gazette with {jjezs_slate} slug. Parse input, extract silks from lede, docket from content, invoke enroll logic.

2. **jjx_revise_docket** — accept gazette with {jjezs_reslate} slug. Parse input, extract coronet from lede, docket from content, invoke revise logic. Support mass reslate: multiple {jjezs_reslate} notices in one gazette, each with a different coronet lede.

3. **jjx_paddock setter** — accept gazette with {jjezs_paddock} slug. Parse input, extract firemark from lede, content from body, invoke paddock write logic.

## Testing

For each operation: valid input, error cases (unknown slugs, missing ledes, malformed content), gazette diagnostic quality for LLM consumption.

## Depends on

₢AwAAO (implement-gazette-rust) — needs working jjrz_parse

**[260318-1535] rough**

Implement file exchange for input operations: jjx_enroll (slate), jjx_revise_docket (reslate), and jjx_curry (paddock setter).

## Scope

Wire JJF parsing into three input-consuming operations:

1. **jjx_enroll** — accept `jjfids_slate` file input in place of docket string parameter. Read from exchange in/, parse jjfids_slate tag, extract silks from label, docket from content, invoke enroll logic.

2. **jjx_revise_docket** — accept `jjfidr_reslate` file input in place of docket string parameter. Read from exchange in/, parse jjfidr_reslate tag, extract coronet from label, docket from content, invoke revise logic.

3. **jjx_curry setter** — accept `jjfip_paddocks` file input in place of content parameter. Read from exchange in/, parse jjfip_paddocks tag, extract firemark from label, content from body, invoke curry write logic.

## Testing

For each operation: write unit tests exercising valid input, error cases (missing labels, invalid tags, malformed content), file lifecycle (delete after read).

## Produces

- Updated enroll, revise_docket, curry implementations using jjrpf_parse_jjf from ₢AwAAH
- Unit test coverage for each operation's file exchange path
- Verified file exchange lifecycle (in/ cleanup, error handling)

## Depends on

₢AwAAH (unit-test-jjf-format) — needs working JJF parser routines

### implement-jjf-output-operations (₢AwAAJ) [complete]

**[260323-1806] complete**

Implement gazette-based output for read operations: jjx_orient (mount), jjx_show (parade), and jjx_paddock getter.

## Scope

Wire gazette building and emitting into three output-producing operations:

1. **jjx_orient** — build gazette with {jjezs_paddock} (firemark lede, paddock content) and {jjezs_pace} (coronet lede, docket content). Emit as markdown in response.

2. **jjx_show --detail** — build gazette with {jjezs_paddock} and {jjezs_pace} when displaying heat with detail flag. Same slug/lede pattern as orient.

3. **jjx_paddock getter** — build gazette with {jjezs_paddock} (firemark lede, paddock content). Emit as markdown.

## Testing

For each operation: verify emitted gazette parses cleanly (round-trip), verify slug/lede values match expected identities.

## Depends on

₢AwAAO (implement-gazette-rust) — needs working jjrz_build, jjrz_add, jjrz_emit

**[260321-1328] rough**

Implement gazette-based output for read operations: jjx_orient (mount), jjx_show (parade), and jjx_paddock getter.

## Scope

Wire gazette building and emitting into three output-producing operations:

1. **jjx_orient** — build gazette with {jjezs_paddock} (firemark lede, paddock content) and {jjezs_pace} (coronet lede, docket content). Emit as markdown in response.

2. **jjx_show --detail** — build gazette with {jjezs_paddock} and {jjezs_pace} when displaying heat with detail flag. Same slug/lede pattern as orient.

3. **jjx_paddock getter** — build gazette with {jjezs_paddock} (firemark lede, paddock content). Emit as markdown.

## Testing

For each operation: verify emitted gazette parses cleanly (round-trip), verify slug/lede values match expected identities.

## Depends on

₢AwAAO (implement-gazette-rust) — needs working jjrz_build, jjrz_add, jjrz_emit

**[260318-1535] rough**

Implement file exchange for output operations: jjx_orient (mount), jjx_show (parade), and jjx_curry (paddock getter).

## Scope

Wire JJF emitting into three output-producing operations:

1. **jjx_orient** — emit `jjfip_paddocks` and `jjfids_slate` tags to exchange out/. Write paddock content under jjfip_paddocks tag (labeled with firemark), write pace docket under jjfids_slate tag (labeled with silks). Delete prior out/ content on invocation.

2. **jjx_show --detail** — emit `jjfip_paddocks` and `jjfids_slate` tags to exchange out/ when displaying heat with detail flag. Same labeling as orient.

3. **jjx_curry getter** — emit `jjfip_paddocks` tag to exchange out/ when invoked without content parameter. Write paddock content under tag (labeled with firemark).

## Testing

For each operation: verify emitted files are valid JJF (parseable by jjremit_emit_jjf), verify labels match expected values, verify file lifecycle (prior content deleted, new content present until next invocation).

## Produces

- Updated orient, show (--detail), curry (getter) implementations using jjremit_emit_jjf from ₢AwAAH
- Unit test coverage for each operation's file exchange output path
- Verified file exchange lifecycle (out/ cleanup before write, idempotent re-invocation)

## Depends on

₢AwAAH (unit-test-jjf-format) — needs working JJF emitter routines

### axhe-taxonomy-pilot (₢AwAAB) [complete]

**[260323-1900] complete**

Replace transport-coupled `axi_cli_*` annotations with transport-agnostic `axvo_*` equivalents in JJS0.

## Scope

**Layer 1 — Operations (20 instances):**
Replace `//axl_voices axi_cli_subcommand` with `//axvo_procedure axd_transient axd_grouped` on all operation definitions in the Operations section (L1267+). Mechanical — the target annotation is already proven in the Operation Taxonomy for `jjsoprd_revise_docket`.

**Layer 3 — Infrastructure (2 instances):**
Replace `//axl_voices axi_cli_program` (L1104) and `//axl_voices axi_cli_command_group` (L1110) with appropriate transport-agnostic voicing.

**Layer 2 — Arguments (13 instances): design only, no code change.**
The `axa_cli_option`/`axa_cli_flag` annotations carry dead CLI metadata (`Long:`, `Short:`). Document the design question: what voices a shared argument at its definition site? Capture options in commit message or paddock for subsequent pace.

## Out of scope

Operation Taxonomy `axvo_*` annotations — confirmed correct and transport-agnostic. They stay.

## Character

Mostly mechanical (Layer 1), with a small design element (Layer 2 assessment). Fast-moving.

## Produces

- 20 operations migrated from `axi_cli_subcommand` to `axvo_procedure`
- 2 infrastructure terms migrated
- Design note on Layer 2 argument voicing for subsequent pace
- Confirmation that Operation Taxonomy annotations are stable

**[260323-1851] rough**

Replace transport-coupled `axi_cli_*` annotations with transport-agnostic `axvo_*` equivalents in JJS0.

## Scope

**Layer 1 — Operations (20 instances):**
Replace `//axl_voices axi_cli_subcommand` with `//axvo_procedure axd_transient axd_grouped` on all operation definitions in the Operations section (L1267+). Mechanical — the target annotation is already proven in the Operation Taxonomy for `jjsoprd_revise_docket`.

**Layer 3 — Infrastructure (2 instances):**
Replace `//axl_voices axi_cli_program` (L1104) and `//axl_voices axi_cli_command_group` (L1110) with appropriate transport-agnostic voicing.

**Layer 2 — Arguments (13 instances): design only, no code change.**
The `axa_cli_option`/`axa_cli_flag` annotations carry dead CLI metadata (`Long:`, `Short:`). Document the design question: what voices a shared argument at its definition site? Capture options in commit message or paddock for subsequent pace.

## Out of scope

Operation Taxonomy `axvo_*` annotations — confirmed correct and transport-agnostic. They stay.

## Character

Mostly mechanical (Layer 1), with a small design element (Layer 2 assessment). Fast-moving.

## Produces

- 20 operations migrated from `axi_cli_subcommand` to `axvo_procedure`
- 2 infrastructure terms migrated
- Design note on Layer 2 argument voicing for subsequent pace
- Confirmation that Operation Taxonomy annotations are stable

**[260323-1835] rough**

Pilot the `axhe*` entity voicing migration on JJS0's Operation Taxonomy section.

## Approach

Use JJSCGZ-gazette.adoc as the Rosetta Stone — it already speaks `axhe*` natively. Map the 6 `axvo_*` annotations in the Operation Taxonomy section (L1160-1249) to `axhe*` equivalents:

- `axvo_group` (jjsogmc_mcp_commands) → appropriate `axhe*` marker
- `axvo_procedure axd_transient axd_grouped` (jjsoprd_revise_docket) → `axhe*` equivalent
- `axvo_method axd_transient` (resolve_pace, prepend_tack, revise_docket) → `axhe*` equivalent
- `axvo_procedure axd_internal` (load, save, persist, wrap routines) → `axhe*` equivalent

## Character

Design conversation at the boundary — mapping between two vocabularies requires judgment about which structural roles align. Small scope (6 annotations) limits blast radius.

## Produces

- Updated JJS0 Operation Taxonomy section with `axhe*` annotations
- Mapping table (old → new) captured in commit message or paddock for subsequent paces
- Discovery of any `axhe*` motifs that need extension

**[260323-1834] rough**

Compare JJS0 spec annotations against actual Rust implementation to establish a baseline before normalization.

## Scope

For each annotated definition in JJS0, check whether the implementation (jjrt_types.rs, jjr*_.rs command files, jjrv_validate.rs) matches the spec's claims. Focus on:

1. **Type annotations**: Do `//axl_voices axr_record_json` and `//axl_voices axr_member` on Gallops/Heat/Pace match actual serde-derived Rust structs?
2. **Operation annotations**: Do `//axl_voices axi_cli_subcommand` annotations match what the MCP tool registration actually does?
3. **Enum annotations**: Do `//axl_voices axt_enum_value` markers cover all variants in Rust enums?
4. **Argument annotations**: Do `//axl_voices axa_cli_option` / `axa_cli_flag` match actual MCP tool parameter schemas?

## Character

Audit discipline — careful, mechanical comparison. Not design work. Read spec, read code, note discrepancies.

## Produces

- **Memo**: `Memos/memo-YYYYMMDD-jjs0-impl-vs-spec-baseline.md` documenting:
  - Total annotation count and categorized breakdown
  - Each divergence: spec says X, code does Y
  - Each gap: spec has annotation but code has no corresponding construct (or vice versa)
  - Each match: confirmed alignment
- Memo filename written into ₣Aw paddock so the trailer reassessment pace can find it
- This baseline informs all subsequent normalization paces AND the trailer reassessment

**[260317-1835] rough**

Compare JJS0 spec annotations against actual Rust implementation to establish a baseline before normalization.

## Scope

For each annotated definition in JJS0, check whether the implementation (jjrt_types.rs, jjr*_.rs command files, jjrv_validate.rs) matches the spec's claims. Focus on:

1. **Type annotations**: Do `//axl_voices axr_record_json` and `//axl_voices axr_member` on Gallops/Heat/Pace match actual serde-derived Rust structs?
2. **Operation annotations**: Do `//axl_voices axi_cli_subcommand` annotations match what the MCP tool registration actually does?
3. **Enum annotations**: Do `//axl_voices axt_enum_value` markers cover all variants in Rust enums?
4. **Argument annotations**: Do `//axl_voices axa_cli_option` / `axa_cli_flag` match actual MCP tool parameter schemas?

## Character

Audit discipline — careful, mechanical comparison. Not design work. Read spec, read code, note discrepancies.

## Produces

- **Memo**: `Memos/memo-YYYYMMDD-jjs0-impl-vs-spec-baseline.md` documenting:
  - Total annotation count and categorized breakdown
  - Each divergence: spec says X, code does Y
  - Each gap: spec has annotation but code has no corresponding construct (or vice versa)
  - Each match: confirmed alignment
- Memo filename written into ₣Aw paddock so the trailer reassessment pace can find it
- This baseline informs all subsequent normalization paces AND the trailer reassessment

**[260317-1831] rough**

Compare JJS0 spec annotations against actual Rust implementation to establish a baseline before normalization.

## Scope

For each annotated definition in JJS0, check whether the implementation (jjrt_types.rs, jjr*_.rs command files, jjrv_validate.rs) matches the spec's claims. Focus on:

1. **Type annotations**: Do `//axl_voices axr_record_json` and `//axl_voices axr_member` on Gallops/Heat/Pace match actual serde-derived Rust structs?
2. **Operation annotations**: Do `//axl_voices axi_cli_subcommand` annotations match what the MCP tool registration actually does?
3. **Enum annotations**: Do `//axl_voices axt_enum_value` markers cover all variants in Rust enums?
4. **Argument annotations**: Do `//axl_voices axa_cli_option` / `axa_cli_flag` match actual MCP tool parameter schemas?

## Character

Audit discipline — careful, mechanical comparison. Not design work. Read spec, read code, note discrepancies.

## Produces

- Paddock note summarizing: what matches, what diverges, what's missing from spec, what's missing from code
- This baseline informs all subsequent normalization paces

### axhe-data-model-migration (₢AwAAC) [complete]

**[260324-0840] complete**

Migrate JJS0 data model annotations to `axhe*` entity voicing convention.

## Scope

Apply the vocabulary mapping established in ₢AwAAB (taxonomy pilot) to JJS0's structural core:

1. **Records**: Gallops, Heat, Pace — `axr_record_json` → `axheb_entity` or equivalent
2. **Members**: All `axr_member` annotations on Gallops/Heat/Pace fields → `axheft_typed_field` or `axhefm_motif_field`
3. **Enum values**: HeatStatus (3), PaceState (3), Column Table alignment (2) — `axt_enum_value` → appropriate `axhe*`
4. **Scalar types**: Firemark, Coronet, Silks, Slug — `axt_string`/`axt_enumeration` → `axhe*` equivalents
5. **Column Table**: entity + members + enums as a unit

## Character

Mechanical with the taxonomy pilot as guide. ~25 annotations to migrate.

## Depends on

₢AwAAB (axhe-taxonomy-pilot) — need the mapping vocabulary

**[260323-1835] rough**

Migrate JJS0 data model annotations to `axhe*` entity voicing convention.

## Scope

Apply the vocabulary mapping established in ₢AwAAB (taxonomy pilot) to JJS0's structural core:

1. **Records**: Gallops, Heat, Pace — `axr_record_json` → `axheb_entity` or equivalent
2. **Members**: All `axr_member` annotations on Gallops/Heat/Pace fields → `axheft_typed_field` or `axhefm_motif_field`
3. **Enum values**: HeatStatus (3), PaceState (3), Column Table alignment (2) — `axt_enum_value` → appropriate `axhe*`
4. **Scalar types**: Firemark, Coronet, Silks, Slug — `axt_string`/`axt_enumeration` → `axhe*` equivalents
5. **Column Table**: entity + members + enums as a unit

## Character

Mechanical with the taxonomy pilot as guide. ~25 annotations to migrate.

## Depends on

₢AwAAB (axhe-taxonomy-pilot) — need the mapping vocabulary

**[260323-1834] rough**

Audit JJS0 against AXLA for motif gaps and propose additions.

## Known gaps (from initial analysis)

1. **axa_cli_flag** — JJS0 uses this (lines 925, 942) but AXLA doesn't define it. Needs upstream addition.
2. **axi_mcp_tool** — JJS0 operations are MCP tools, not CLI subcommands. AXLA has axi_cli_subcommand but nothing for MCP tool interface. Need a motif.
3. **Transport dimension on operations** — AXLA defines axe_mcp_transport as a motif but JJS0 operations don't carry it. Should MCP-served operations annotate their transport?
4. **Operation dimension vocabulary** — RBS0 uses axd_transient, axd_attended, axd_grouped. Which of these apply to JJS0's MCP tool operations? Are they all transient? Which are attended?
5. **Category-level voicing** — JJS0 category prefixes (jjdgr_, jjdgm_) declare intent in comments but don't formally voice AXLA motifs. Is per-definition annotation sufficient or does AXLA need category-level voicing?

## Character

Design conversation — judgment required on what AXLA should add vs what JJS0 should adapt to existing motifs.

## Produces

- Concrete AXLA additions (new definitions in AXLA-Lexicon.adoc mapping section and definition body)
- Decision record on each gap: add to AXLA, reuse existing motif, or accept the gap
- Updated JJS0 mapping section category comments if category-level voicing pattern is adopted

**[260317-1831] rough**

Audit JJS0 against AXLA for motif gaps and propose additions.

## Known gaps (from initial analysis)

1. **axa_cli_flag** — JJS0 uses this (lines 925, 942) but AXLA doesn't define it. Needs upstream addition.
2. **axi_mcp_tool** — JJS0 operations are MCP tools, not CLI subcommands. AXLA has axi_cli_subcommand but nothing for MCP tool interface. Need a motif.
3. **Transport dimension on operations** — AXLA defines axe_mcp_transport as a motif but JJS0 operations don't carry it. Should MCP-served operations annotate their transport?
4. **Operation dimension vocabulary** — RBS0 uses axd_transient, axd_attended, axd_grouped. Which of these apply to JJS0's MCP tool operations? Are they all transient? Which are attended?
5. **Category-level voicing** — JJS0 category prefixes (jjdgr_, jjdgm_) declare intent in comments but don't formally voice AXLA motifs. Is per-definition annotation sufficient or does AXLA need category-level voicing?

## Character

Design conversation — judgment required on what AXLA should add vs what JJS0 should adapt to existing motifs.

## Produces

- Concrete AXLA additions (new definitions in AXLA-Lexicon.adoc mapping section and definition body)
- Decision record on each gap: add to AXLA, reuse existing motif, or accept the gap
- Updated JJS0 mapping section category comments if category-level voicing pattern is adopted

### axhe-arguments-and-remaining-voicing (₢AwAAD) [complete]

**[260324-1928] complete**

Migrate remaining transport-coupled annotations in JJS0. Two categories only — Upper API Verbs (axi_cc_claudemd_verb) deliberately retained.

## Scope

**1. Arguments (13 instances, L1046-1200):**
Replace `axa_cli_option`/`axa_cli_flag` with transport-agnostic voicing. Strip dead CLI metadata (`Long:`, `Short:`). Design question: AXLA has no `axvo_argument` — either mint one, use dimension-only annotations, or find existing motif that fits shared argument definitions. Note: `jjda_file` (L1048) already has `axd_internal` — this dimension should survive migration.

**2. Section Headers (5 definitions, L1201-1225):**
3 annotated: `axa_argument_list` (L1203), `axa_exit_uniform` (L1212), `axa_exit_enumerated` (L1217). 2 bare (no annotation): `jjds_stdout` (L1208), `jjds_behavior` (L1222). The `axa_` prefix is from the CLI era. Evaluate: rename prefix, add annotations to bare ones, or leave. These are structural formatting annotations for spec prose, not transport-coupled.

## Deliberately Retained

**Upper API Verbs (15 instances, L450-539):**
14x `axi_cc_claudemd_verb` + 1x `axi_cc_claudemd_section`. These voice JJ verb/section entries in the CLAUDE.md interface — the mapping layer from simple user verbs to precise MCP calls. Evaluated and retained: the CLAUDE.md interface is a real, load-bearing semantic layer, not dead transport metadata.

## Exemplars

**Operation Taxonomy (₢AwAAB):** `//axvo_procedure axd_transient` — transport-agnostic behavioral voicing.
**Data Model (₢AwAAC):** `//axl_voices axr_member axd_required` — structural voicing with dimensions.

## Character

Mostly mechanical with one design decision: argument voicing motif. Section headers are minor prefix cleanup. Smaller scope than originally planned — verb retention eliminated the hardest design question.

**[260324-1901] rough**

Migrate remaining transport-coupled annotations in JJS0. Two categories only — Upper API Verbs (axi_cc_claudemd_verb) deliberately retained.

## Scope

**1. Arguments (13 instances, L1046-1200):**
Replace `axa_cli_option`/`axa_cli_flag` with transport-agnostic voicing. Strip dead CLI metadata (`Long:`, `Short:`). Design question: AXLA has no `axvo_argument` — either mint one, use dimension-only annotations, or find existing motif that fits shared argument definitions. Note: `jjda_file` (L1048) already has `axd_internal` — this dimension should survive migration.

**2. Section Headers (5 definitions, L1201-1225):**
3 annotated: `axa_argument_list` (L1203), `axa_exit_uniform` (L1212), `axa_exit_enumerated` (L1217). 2 bare (no annotation): `jjds_stdout` (L1208), `jjds_behavior` (L1222). The `axa_` prefix is from the CLI era. Evaluate: rename prefix, add annotations to bare ones, or leave. These are structural formatting annotations for spec prose, not transport-coupled.

## Deliberately Retained

**Upper API Verbs (15 instances, L450-539):**
14x `axi_cc_claudemd_verb` + 1x `axi_cc_claudemd_section`. These voice JJ verb/section entries in the CLAUDE.md interface — the mapping layer from simple user verbs to precise MCP calls. Evaluated and retained: the CLAUDE.md interface is a real, load-bearing semantic layer, not dead transport metadata.

## Exemplars

**Operation Taxonomy (₢AwAAB):** `//axvo_procedure axd_transient` — transport-agnostic behavioral voicing.
**Data Model (₢AwAAC):** `//axl_voices axr_member axd_required` — structural voicing with dimensions.

## Character

Mostly mechanical with one design decision: argument voicing motif. Section headers are minor prefix cleanup. Smaller scope than originally planned — verb retention eliminated the hardest design question.

**[260324-0846] rough**

Migrate remaining transport-coupled and first-generation annotations in JJS0.

## Exemplars

Two completed paces established the patterns this work follows:

**Operation Taxonomy (₢AwAAB, L1285-1375):** Transport-agnostic voicing for behavioral constructs.
- `//axvo_procedure axd_transient axd_grouped` — MCP-served procedures
- `//axvo_method axd_transient` — internal Gallops methods
- `//axvo_group` — operation collection
- No CLI metadata; arguments listed inline in definition prose.

**Data Model (₢AwAAC, L555-952):** Structural voicing for data constructs.
- `//axl_voices axr_record_json` — JSON records
- `//axl_voices axr_member axd_required` (+ `axd_optional`, `axd_repeated`, type dims) — members
- `//axl_voices axt_enum_value` — enum values
- `//axl_voices axo_entity` — non-JSON entities
- Pattern: `axl_voices` + structural motif + zero or more dimensions.

## Scope

Three categories, each needing a vocabulary decision before mechanical application:

**1. Arguments (13 instances, L1046-1200):**
Replace `axa_cli_option`/`axa_cli_flag` with transport-agnostic voicing. Strip dead CLI metadata (`Long:`, `Short:`). Design question: AXLA has no `axvo_argument` — either mint one, use dimension-only annotations, or find existing motif that fits shared argument definitions. Note: `jjda_file` (L1048) already has `axd_internal` — this dimension should survive migration.

**2. Upper API Verbs (15 instances, L450-539):**
14x `axi_cc_claudemd_verb` + 1x `axi_cc_claudemd_section` (L548). These voice JJ verb/section entries in the CLAUDE.md interface. Transport-specific to Claude Code's CLAUDE.md convention. Evaluate: keep as-is (they ARE interface-specific), replace with transport-agnostic equivalent, or retire. Key question: is "delivered via CLAUDE.md" a transport concern or a presentation concern?

**3. Section Headers (5 definitions, L1201-1225):**
3 annotated: `axa_argument_list` (L1203), `axa_exit_uniform` (L1212), `axa_exit_enumerated` (L1217). 2 bare (no annotation): `jjds_stdout` (L1208), `jjds_behavior` (L1222). The `axa_` prefix is from the CLI era. Evaluate: rename prefix, add annotations to bare ones, or leave. These are structural formatting annotations for spec prose, not transport-coupled.

## Character

Design-heavy. Each category needs a vocabulary decision before mechanical application. Arguments are the highest-value target (dead metadata removal). CLAUDE.md verbs are a genuine transport-specific concept that may be correct as-is. Start by reading the exemplar sections to internalize the pattern, then design each category's voicing.

## Depends on

₢AwAAB (done) — established the axvo_procedure pattern
₢AwAAC (done) — established the data model voicing pattern

**[260324-0845] rough**

Migrate remaining transport-coupled and first-generation annotations in JJS0.

## Exemplars

Two completed paces established the patterns this work follows:

**Operation Taxonomy (₢AwAAB, L1285-1375):** Transport-agnostic voicing for behavioral constructs.
- `//axvo_procedure axd_transient axd_grouped` — MCP-served procedures
- `//axvo_method axd_transient` — internal Gallops methods
- `//axvo_group` — operation collection
- No CLI metadata; arguments listed inline in definition prose.

**Data Model (₢AwAAC, L555-952):** Structural voicing for data constructs.
- `//axl_voices axr_record_json` — JSON records
- `//axl_voices axr_member axd_required` (+ `axd_optional`, `axd_repeated`, type dims) — members
- `//axl_voices axt_enum_value` — enum values
- `//axl_voices axo_entity` — non-JSON entities
- Pattern: `axl_voices` + structural motif + zero or more dimensions.

## Scope

Three categories, each needing a vocabulary decision before mechanical application:

**1. Arguments (13 instances, L1046-1200):**
Replace `axa_cli_option`/`axa_cli_flag` with transport-agnostic voicing. Strip dead CLI metadata (`Long:`, `Short:`). Design question: AXLA has no `axvo_argument` — either mint one, use dimension-only annotations, or find existing motif that fits shared argument definitions. Note: `jjda_file` (L1048) already has `axd_internal` — this dimension should survive migration.

**2. Upper API Verbs (15 instances, L450-539):**
14x `axi_cc_claudemd_verb` + 1x `axi_cc_claudemd_section` (L548). These voice JJ verb/section entries in the CLAUDE.md interface. Transport-specific to Claude Code's CLAUDE.md convention. Evaluate: keep as-is (they ARE interface-specific), replace with transport-agnostic equivalent, or retire. Key question: is "delivered via CLAUDE.md" a transport concern or a presentation concern?

**3. Section Headers (5 definitions, L1201-1225):**
3 annotated: `axa_argument_list` (L1203), `axa_exit_uniform` (L1212), `axa_exit_enumerated` (L1217). 2 bare (no annotation): `jjds_stdout` (L1208), `jjds_behavior` (L1222). The `axa_` prefix is from the CLI era. Evaluate: rename prefix, add annotations to bare ones, or leave. These are structural formatting annotations for spec prose, not transport-coupled.

## Character

Design-heavy. Each category needs a vocabulary decision before mechanical application. Arguments are the highest-value target (dead metadata removal). CLAUDE.md verbs are a genuine transport-specific concept that may be correct as-is. Start by reading the exemplar sections to internalize the pattern, then design each category's voicing.

## Depends on

₢AwAAB (done) — established the axvo_procedure pattern
₢AwAAC (done) — established the data model voicing pattern

**[260323-1857] rough**

Migrate remaining transport-coupled and first-generation annotations in JJS0.

## Scope

Operations (20) and infrastructure (3) already done in ₢AwAAB. This pace covers the three remaining categories:

**1. Arguments (13 instances, L924-1067):**
Replace `axa_cli_option`/`axa_cli_flag` with transport-agnostic voicing. Strip dead CLI metadata (`Long:`, `Short:`). Design question: AXLA has no `axvo_argument` — either mint one, use dimension-only annotations, or find existing motif that fits shared argument definitions.

**2. CLAUDE.md upper API (15 instances, L440-532):**
14x `axi_cc_claudemd_verb` + 1x `axi_cc_claudemd_section`. These voice JJ verb/section entries in the CLAUDE.md interface. Transport-specific to Claude Code’s CLAUDE.md convention. Evaluate: keep as-is (they ARE interface-specific), replace with transport-agnostic equivalent, or retire.

**3. Section headers (3 instances, L1079-1093):**
`axa_argument_list`, `axa_exit_uniform`, `axa_exit_enumerated`. Structural formatting annotations for spec prose. Not transport-coupled but carry the `axa_` prefix from the CLI era. Evaluate: rename prefix or leave.

## Character

Design-heavy. Each category needs a vocabulary decision before mechanical application. Arguments are the highest-value target (dead metadata removal). CLAUDE.md verbs are a genuine transport-specific concept that may be correct as-is.

## Depends on

₢AwAAB (done) — established the axvo_procedure pattern

**[260323-1835] rough**

Migrate JJS0 operation and argument annotations to `axhe*` entity voicing convention.

## Scope

1. **20 operations** (`jjdo_*`): Replace `//axl_voices axi_cli_subcommand` with transport-agnostic `axhe*` annotations. These are structural operations, not CLI subcommands.
2. **13 arguments** (`jjda_*`): Replace `axa_cli_option`/`axa_cli_flag` with `axhe*` parameter annotations. Short-form CLI syntax (`-s`, `-p`) is vestigial — document or remove.
3. **Transport/CLI definitions**: `jjdx_vvx` (axi_cli_program), `jjdx_cli` (axi_cli_command_group), `jjdx_mcp` (axi_cli_subcommand) — these ARE transport-specific by nature, migrate thoughtfully.
4. **Upper API verbs** (14): `axi_cc_claudemd_verb` — evaluate whether these belong in `axhe*` or remain in the context-artifact vocabulary.
5. **Section headers** (3): `axa_argument_list`, `axa_exit_uniform`, `axa_exit_enumerated` — structural formatting annotations.

## Character

Largest migration pace (~50 annotations). Mechanical core with judgment edges on transport-specific definitions and upper API verbs.

## Depends on

₢AwAAB (taxonomy pilot) and ₢AwAAC (data model) — vocabulary and confidence from earlier paces

**[260323-1834] rough**

Replace first-generation annotations on JJS0 operations with RBS0-style voicing annotations.

## Current state

All JJS0 operations (create, enroll, reorder, show, orient, etc.) carry:
```
//axl_voices axi_cli_subcommand
```
This is semantically wrong (they're MCP tools) and lacks dimensions.

## Target state

Each operation carries the appropriate annotation from ₢AwAAC's AXLA gap resolution:
- Correct interface motif (whatever replaces axi_cli_subcommand for MCP tools)
- Dimension modifiers matching RBS0 patterns (transient, attended where applicable, grouped if operation groups are formalized)

## Scope

- All `[[jjdo_*]]` anchored definitions in JJS0
- All `[[jjdr_*]]` routine definitions (these already have `//axvo_procedure axd_internal` — verify correctness)
- Operation group headers if ₢AwAAC formalizes them as linked terms
- Skip V4-VOLATILE marked sections

## Character

Mechanical with judgment edges — most operations get the same annotation pattern, but attended/unattended distinction requires understanding each operation.

## Depends on

- ₢AwAAC (surface-axla-gaps) — need resolved motifs before applying them

**[260317-1831] rough**

Replace first-generation annotations on JJS0 operations with RBS0-style voicing annotations.

## Current state

All JJS0 operations (create, enroll, reorder, show, orient, etc.) carry:
```
//axl_voices axi_cli_subcommand
```
This is semantically wrong (they're MCP tools) and lacks dimensions.

## Target state

Each operation carries the appropriate annotation from ₢AwAAC's AXLA gap resolution:
- Correct interface motif (whatever replaces axi_cli_subcommand for MCP tools)
- Dimension modifiers matching RBS0 patterns (transient, attended where applicable, grouped if operation groups are formalized)

## Scope

- All `[[jjdo_*]]` anchored definitions in JJS0
- All `[[jjdr_*]]` routine definitions (these already have `//axvo_procedure axd_internal` — verify correctness)
- Operation group headers if ₢AwAAC formalizes them as linked terms
- Skip V4-VOLATILE marked sections

## Character

Mechanical with judgment edges — most operations get the same annotation pattern, but attended/unattended distinction requires understanding each operation.

## Depends on

- ₢AwAAC (surface-axla-gaps) — need resolved motifs before applying them

### spec-gap-closure (₢AwAAE) [complete]

**[260324-2003] complete**

Close specification gaps in JJS0, using `axhe*` annotations from the start.

## Scope

1. **Add `jjdo_close` operation**: Code dispatches `jjx_close` to wrap handler (jjrm_mcp.rs:565-571). Params: coronet, summary, size_limit. No spec entry exists. Write full operation definition with `axhe*` annotations.
2. **Add `jjdo_paddock` operation**: Code dispatches `jjx_paddock` to curry handler (jjrm_mcp.rs:596-622). Params: firemark, content, note, input (gazette). No spec entry exists. Write full operation definition.
3. **Resolve Bridled state**: `jjrg_PaceState::Bridled` exists in code but only in V3 Legacy spec section. Either declare current or mark for removal in ₣An.
4. **Tack record**: Only described in V3 Legacy (L1522-1528). Either add current-spec definition or acknowledge V3 Legacy as authoritative for Tack.
5. **Vestigial arguments**: `jjda_state`, `jjda_pace`, `jjda_created`, `jjda_direction` — no MCP exposure. Mark deprecated or remove.
6. **Unspecified MCP parameters**: 16 parameters (detail, remaining, rough, pattern, etc.) have no `jjda_` definitions. Add specs or document why not needed.

## Character

Mixed — spec writing (operations) + design decisions (bridled, tack, vestigial args). Judgment-heavy.

**[260323-1835] rough**

Close specification gaps in JJS0, using `axhe*` annotations from the start.

## Scope

1. **Add `jjdo_close` operation**: Code dispatches `jjx_close` to wrap handler (jjrm_mcp.rs:565-571). Params: coronet, summary, size_limit. No spec entry exists. Write full operation definition with `axhe*` annotations.
2. **Add `jjdo_paddock` operation**: Code dispatches `jjx_paddock` to curry handler (jjrm_mcp.rs:596-622). Params: firemark, content, note, input (gazette). No spec entry exists. Write full operation definition.
3. **Resolve Bridled state**: `jjrg_PaceState::Bridled` exists in code but only in V3 Legacy spec section. Either declare current or mark for removal in ₣An.
4. **Tack record**: Only described in V3 Legacy (L1522-1528). Either add current-spec definition or acknowledge V3 Legacy as authoritative for Tack.
5. **Vestigial arguments**: `jjda_state`, `jjda_pace`, `jjda_created`, `jjda_direction` — no MCP exposure. Mark deprecated or remove.
6. **Unspecified MCP parameters**: 16 parameters (detail, remaining, rough, pattern, etc.) have no `jjda_` definitions. Add specs or document why not needed.

## Character

Mixed — spec writing (operations) + design decisions (bridled, tack, vestigial args). Judgment-heavy.

**[260323-1834] rough**

Update record/member/enum/type annotations on JJS0 data model sections that survive V4.

## Scope

Annotate definitions that are NOT marked V4-VOLATILE:

1. **Gallops record and members** — verify/complete axr_record_json and axr_member annotations
2. **Heat record and members** — same
3. **Identity types** (firemark, coronet, silks) — verify axt_string annotations, add dimension modifiers
4. **Heat status enum** (racing/stabled/retired) — verify axt_enum_value annotations
5. **Column Table** record/members/enum — these are JJS0-specific display infrastructure, annotate appropriately
6. **Argument definitions** (jjda_*) — verify axa_cli_option / axa_cli_flag annotations match resolved AXLA vocabulary
7. **Upper API verbs and slash commands** — verify axi_cc_claudemd_verb and axi_cc_slash_command annotations

## Character

Mechanical — systematic pass through non-volatile definitions applying consistent annotation patterns.

## Depends on

- ₢AwAAA (mark-volatile) — need to know what to skip
- ₢AwAAC (surface-axla-gaps) — need resolved motif vocabulary

**[260317-1831] rough**

Update record/member/enum/type annotations on JJS0 data model sections that survive V4.

## Scope

Annotate definitions that are NOT marked V4-VOLATILE:

1. **Gallops record and members** — verify/complete axr_record_json and axr_member annotations
2. **Heat record and members** — same
3. **Identity types** (firemark, coronet, silks) — verify axt_string annotations, add dimension modifiers
4. **Heat status enum** (racing/stabled/retired) — verify axt_enum_value annotations
5. **Column Table** record/members/enum — these are JJS0-specific display infrastructure, annotate appropriately
6. **Argument definitions** (jjda_*) — verify axa_cli_option / axa_cli_flag annotations match resolved AXLA vocabulary
7. **Upper API verbs and slash commands** — verify axi_cc_claudemd_verb and axi_cc_slash_command annotations

## Character

Mechanical — systematic pass through non-volatile definitions applying consistent annotation patterns.

## Depends on

- ₢AwAAA (mark-volatile) — need to know what to skip
- ₢AwAAC (surface-axla-gaps) — need resolved motif vocabulary

### impl-vs-spec-reassessment (₢AwAAF) [complete]

**[260324-2003] complete**

Light verification of `axhe*` migration and handoff to ₣Ah.

## Scope

1. Walk JJS0 annotations and confirm `axhe*` convention is consistently applied to operations and arguments
2. Verify no orphaned `axi_cli_subcommand` or `axa_cli_option` annotations remain. Note: `axl_voices` is deliberately retained on upper API verbs (`axi_cc_claudemd_verb`), entities (`axo_entity`), and enum values (`axt_enum_value`) — these are not migration targets
3. Confirm new operation specs (close, paddock) are well-formed and their includes resolve
4. Update ₣Ah paddock with migration summary and any V4-relevant findings
5. Capture any `axhe*` vocabulary extensions made during this heat for AXLA upstream

## Character

Verification and documentation — light touch, not a full re-audit. The incremental approach (pilot → data model → operations) should catch errors as they happen.

## Produces

- Clean handoff summary for ₣Ah paddock
- List of any AXLA upstream proposals generated by the migration

**[260324-1945] rough**

Light verification of `axhe*` migration and handoff to ₣Ah.

## Scope

1. Walk JJS0 annotations and confirm `axhe*` convention is consistently applied to operations and arguments
2. Verify no orphaned `axi_cli_subcommand` or `axa_cli_option` annotations remain. Note: `axl_voices` is deliberately retained on upper API verbs (`axi_cc_claudemd_verb`), entities (`axo_entity`), and enum values (`axt_enum_value`) — these are not migration targets
3. Confirm new operation specs (close, paddock) are well-formed and their includes resolve
4. Update ₣Ah paddock with migration summary and any V4-relevant findings
5. Capture any `axhe*` vocabulary extensions made during this heat for AXLA upstream

## Character

Verification and documentation — light touch, not a full re-audit. The incremental approach (pilot → data model → operations) should catch errors as they happen.

## Produces

- Clean handoff summary for ₣Ah paddock
- List of any AXLA upstream proposals generated by the migration

**[260323-1836] rough**

Light verification of `axhe*` migration and handoff to ₣Ah.

## Scope

1. Walk JJS0 annotations and confirm `axhe*` convention is consistently applied
2. Verify no orphaned `axl_voices` annotations remain on migrated definitions
3. Confirm new operation specs (close, paddock) are well-formed
4. Update ₣Ah paddock with migration summary and any V4-relevant findings
5. Capture any `axhe*` vocabulary extensions made during this heat for AXLA upstream

## Character

Verification and documentation — light touch, not a full re-audit. The incremental approach (pilot → data model → operations) should catch errors as they happen.

## Produces

- Clean handoff summary for ₣Ah paddock
- List of any AXLA upstream proposals generated by the migration

**[260317-1836] rough**

Re-run the ₢AwAAB audit after normalization is complete. Confirm divergence count decreased and categorize what remains.

## Inputs

- Baseline memo from ₢AwAAB (filename in ₣Aw paddock)
- Normalized JJS0 (output of ₢AwAAD and ₢AwAAE)

## Scope

Same audit as ₢AwAAB: walk every annotated JJS0 definition against Rust implementation. For each item in the baseline memo, classify as:

1. **Resolved** — normalization fixed the divergence
2. **Deferred to ₣Ah** — in a V4-VOLATILE section, will be addressed by V4 design paces
3. **New gap** — normalization introduced a divergence (should be zero; if nonzero, fix before wrapping)
4. **Persistent** — was divergent before, still divergent, not V4-VOLATILE (needs explanation)

## Character

Audit discipline — same posture as ₢AwAAB. Mechanical comparison with categorization judgment.

## Acceptance

- Zero "new gap" items
- Zero "persistent" items without documented justification
- Updated memo or addendum documenting the reassessment results
- Clean handoff summary for ₣Ah paddock

## Depends on

- ₢AwAAD (normalize-operation-annotations)
- ₢AwAAE (normalize-data-model-annotations)

### spec-officium-lifecycle (₢AwAAP) [complete]

**[260325-0810] complete**

Spec the officium lifecycle in JJS0: officium as a bounded MCP session with unique identity, directory layout (.claude/jjm/officia/<id>/), invitatory (internal MCP startup: ID generation, directory creation, model inventory, dynamic instructions), compline (internal MCP shutdown: closing commit, directory cleanup), chapter and absolve as tool operations. Define the gazette file exchange path (fixed filename within per-officium directory). Update linked terms and premises as needed.

**[260325-0749] rough**

Spec the officium lifecycle in JJS0: officium as a bounded MCP session with unique identity, directory layout (.claude/jjm/officia/<id>/), invitatory (internal MCP startup: ID generation, directory creation, model inventory, dynamic instructions), compline (internal MCP shutdown: closing commit, directory cleanup), chapter and absolve as tool operations. Define the gazette file exchange path (fixed filename within per-officium directory). Update linked terms and premises as needed.

### claudemd-firemark-coronet-case-sensitivity (₢AwAAa) [complete]

**[260327-1647] complete**

Drafted from ₢AvAAU in ₣Av.

## Character
Spook fix — prevent agent mishandling of case-sensitive identities.

## Goal
Update CLAUDE.md Job Jockey Configuration section to explicitly state that firemarks and coronets are case-sensitive. `AV` ≠ `Av` ≠ `av`. The orient error when passing wrong case is confusing and wastes a tool call.

## Scope
- CLAUDE.md: Add case-sensitivity warning near the Identities vs Display Names section
- Consider adding examples showing correct case matters

**[260327-0837] rough**

Drafted from ₢AvAAU in ₣Av.

## Character
Spook fix — prevent agent mishandling of case-sensitive identities.

## Goal
Update CLAUDE.md Job Jockey Configuration section to explicitly state that firemarks and coronets are case-sensitive. `AV` ≠ `Av` ≠ `av`. The orient error when passing wrong case is confusing and wastes a tool call.

## Scope
- CLAUDE.md: Add case-sensitivity warning near the Identities vs Display Names section
- Consider adding examples showing correct case matters

**[260327-0754] rough**

## Character
Spook fix — prevent agent mishandling of case-sensitive identities.

## Goal
Update CLAUDE.md Job Jockey Configuration section to explicitly state that firemarks and coronets are case-sensitive. `AV` ≠ `Av` ≠ `av`. The orient error when passing wrong case is confusing and wastes a tool call.

## Scope
- CLAUDE.md: Add case-sensitivity warning near the Identities vs Display Names section
- Consider adding examples showing correct case matters

### spec-agent-initiated-officium (₢AwAAY) [complete]

**[260327-1701] complete**

Update JJS0 spec to reflect agent-initiated officium lifecycle BEFORE implementation.

Design rationale: The original officium model (invitatory at MCP server startup) caused a catastrophic thrash incident — Claude Code desktop spawns/kills MCP servers at unpredictable frequency (hundreds per minute), creating runaway directory and commit accumulation. The redesign decouples officium from server process lifetime entirely. The agent (chat session) is the stable identity anchor, not the server process.

1. Revise jjdxo_officium entity: identity format YYMMDD-NNNN (autonumber, no persistent counter — enumerate existing dirs for today, pick next unused starting at 1000). Unicode verification prefix ☉ (U+2609 SUN, evoking the Divine Office's daily cycle). Rule: ☉ appears in params and display, stripped for directory name (parallels ₣/₢ convention). Directory layout: gazette.md + heartbeat sentinel. Entire .claude/jjm/officia/ tree is gitignored.
2. Revise jjdxo_invitatory: now triggered by explicit jjx_open operation (not MCP server startup). Agent calls once per chat. Probe runs once per day via .claude/jjm/officia/.probe_date datestamp file. Invitatory commit (action code 'i') is empty commit with probe body.
3. Retire jjdxo_compline: no clean shutdown procedure. Staleness detected by heartbeat mtime absence. Cleanup via exsanguination at next jjx_open. Retire JJRNM_COMPLINE marker constant ('o') or mark as reserved-unused.
4. Define exsanguination: at jjx_open, scan existing officia heartbeat mtimes, reap directories exceeding staleness threshold (generous, ~4-6 hours). Exsanguination runs BEFORE creating the new officium directory.
5. Spec jjx_open as a new operation: takes no params, returns officium ID string (e.g. ☉260327-1000). Define identity, behavior, output format.
6. Spec officium param: required on all jjx operations except jjx_open. Dispatcher validates directory exists and touches heartbeat on every call.
7. Update steeplechase commit patterns if affected.

This pace is spec-only — no code changes. Implementation follows in ₢AwAAQ.

**[260327-0829] rough**

Update JJS0 spec to reflect agent-initiated officium lifecycle BEFORE implementation.

Design rationale: The original officium model (invitatory at MCP server startup) caused a catastrophic thrash incident — Claude Code desktop spawns/kills MCP servers at unpredictable frequency (hundreds per minute), creating runaway directory and commit accumulation. The redesign decouples officium from server process lifetime entirely. The agent (chat session) is the stable identity anchor, not the server process.

1. Revise jjdxo_officium entity: identity format YYMMDD-NNNN (autonumber, no persistent counter — enumerate existing dirs for today, pick next unused starting at 1000). Unicode verification prefix ☉ (U+2609 SUN, evoking the Divine Office's daily cycle). Rule: ☉ appears in params and display, stripped for directory name (parallels ₣/₢ convention). Directory layout: gazette.md + heartbeat sentinel. Entire .claude/jjm/officia/ tree is gitignored.
2. Revise jjdxo_invitatory: now triggered by explicit jjx_open operation (not MCP server startup). Agent calls once per chat. Probe runs once per day via .claude/jjm/officia/.probe_date datestamp file. Invitatory commit (action code 'i') is empty commit with probe body.
3. Retire jjdxo_compline: no clean shutdown procedure. Staleness detected by heartbeat mtime absence. Cleanup via exsanguination at next jjx_open. Retire JJRNM_COMPLINE marker constant ('o') or mark as reserved-unused.
4. Define exsanguination: at jjx_open, scan existing officia heartbeat mtimes, reap directories exceeding staleness threshold (generous, ~4-6 hours). Exsanguination runs BEFORE creating the new officium directory.
5. Spec jjx_open as a new operation: takes no params, returns officium ID string (e.g. ☉260327-1000). Define identity, behavior, output format.
6. Spec officium param: required on all jjx operations except jjx_open. Dispatcher validates directory exists and touches heartbeat on every call.
7. Update steeplechase commit patterns if affected.

This pace is spec-only — no code changes. Implementation follows in ₢AwAAQ.

**[260327-0822] rough**

Update JJS0 spec to reflect agent-initiated officium lifecycle BEFORE implementation.

1. Revise jjdxo_officium entity: identity format YYMMDD-NNNN (autonumber, no persistent counter — enumerate existing dirs, pick next unused starting at 1000). Directory layout: gazette.md + heartbeat sentinel. Gitignored (.claude/jjm/officia/).
2. Revise jjdxo_invitatory: now triggered by explicit jjx_open operation (not MCP server startup). Agent calls once per chat. Probe runs once per day via .probe_date datestamp file. Invitatory commit (action code 'i') is empty commit with probe body.
3. Remove or retire jjdxo_compline: no clean shutdown procedure. Staleness detected by heartbeat mtime absence. Cleanup via exsanguination at next jjx_open.
4. Define exsanguination: at jjx_open, scan existing officia heartbeat mtimes, reap directories exceeding staleness threshold.
5. Spec jjx_open as a new operation with identity, behavior, and output.
6. Spec officium param: required on all jjx operations except jjx_open. Dispatcher validates directory exists and touches heartbeat.
7. Define unicode verification prefix for officium identity.
8. Update steeplechase commit patterns if affected.

This pace is spec-only — no code changes. Implementation follows in •AwAAQ.

### implement-jjx-open-and-officium-param (₢AwAAQ) [complete]

**[260327-1845] complete**

Implement jjx_open and officium param threading per spec from ₢AwAAY.

1. Add .claude/jjm/officia/ to .gitignore.
2. Implement jjx_open (takes no params):
   a. Exsanguinate first: scan existing officia heartbeat mtimes, remove directories exceeding staleness threshold.
   b. Generate officium ID: enumerate .claude/jjm/officia/ dirs matching today's YYMMDD-*, pick max+1 or 1000 if none.
   c. Create exchange directory .claude/jjm/officia/<id>/ with empty gazette.md and heartbeat file.
   d. Run vvcp_probe if .claude/jjm/officia/.probe_date doesn't match today (write datestamp after).
   e. Create invitatory commit (action code 'i', empty commit with probe body).
   f. Return ☉<id> (officium ID with unicode prefix).
3. Add officium param to jjrm_JjxParams. Every jjx command except jjx_open requires officium — dispatcher strips ☉ prefix, validates directory exists, touches heartbeat on every call. Derive exchange path from bare ID.
4. Remove lazy invitatory (AtomicBool + vvcp_invitatory) from jjx dispatcher — jjx_open replaces it. Leave vvcp_invitatory in VVC untouched (other kits may use it).
5. Retire or remove JJRNM_COMPLINE constant per spec decision in ₢AwAAY.

**[260327-0830] rough**

Implement jjx_open and officium param threading per spec from ₢AwAAY.

1. Add .claude/jjm/officia/ to .gitignore.
2. Implement jjx_open (takes no params):
   a. Exsanguinate first: scan existing officia heartbeat mtimes, remove directories exceeding staleness threshold.
   b. Generate officium ID: enumerate .claude/jjm/officia/ dirs matching today's YYMMDD-*, pick max+1 or 1000 if none.
   c. Create exchange directory .claude/jjm/officia/<id>/ with empty gazette.md and heartbeat file.
   d. Run vvcp_probe if .claude/jjm/officia/.probe_date doesn't match today (write datestamp after).
   e. Create invitatory commit (action code 'i', empty commit with probe body).
   f. Return ☉<id> (officium ID with unicode prefix).
3. Add officium param to jjrm_JjxParams. Every jjx command except jjx_open requires officium — dispatcher strips ☉ prefix, validates directory exists, touches heartbeat on every call. Derive exchange path from bare ID.
4. Remove lazy invitatory (AtomicBool + vvcp_invitatory) from jjx dispatcher — jjx_open replaces it. Leave vvcp_invitatory in VVC untouched (other kits may use it).
5. Retire or remove JJRNM_COMPLINE constant per spec decision in ₢AwAAY.

**[260327-0818] rough**

Implement jjx_open and officium param threading.

1. Add .claude/jjm/officia/ to .gitignore
2. Implement jjx_open: generate officium ID (YYMMDD-NNNN format — enumerate existing dirs for today, pick max+1 or 1000), create exchange directory with empty gazette.md and heartbeat file, run vvcp_probe if .probe_date doesn't match today (write datestamp after), create invitatory commit (action code 'i', empty commit with probe body), exsanguinate stale officia (scan heartbeat mtimes, generous threshold ~4-6 hours), return officium ID.
3. Add officium param to jjrm_JjxParams. Every jjx command except jjx_open requires officium — validate directory exists, touch heartbeat on every dispatch. Derive exchange path from officium ID.
4. Remove lazy invitatory (AtomicBool + vvcp_invitatory) from jjx dispatcher — jjx_open replaces it.
5. Define unicode prefix for officium identity (display/params only, not in directory name).
6. Rename pace silks to match new scope.

**[260327-0818] rough**

Implement jjx_open and officium param threading.

1. Add .claude/jjm/officia/ to .gitignore
2. Implement jjx_open: generate officium ID (YYMMDD-NNNN format — enumerate existing dirs for today, pick max+1 or 1000), create exchange directory with empty gazette.md and heartbeat file, run vvcp_probe if .probe_date doesn't match today (write datestamp after), create invitatory commit (action code 'i', empty commit with probe body), exsanguinate stale officia (scan heartbeat mtimes, generous threshold ~4-6 hours), return officium ID.
3. Add officium param to jjrm_JjxParams. Every jjx command except jjx_open requires officium — validate directory exists, touch heartbeat on every dispatch. Derive exchange path from officium ID.
4. Remove lazy invitatory (AtomicBool + vvcp_invitatory) from jjx dispatcher — jjx_open replaces it.
5. Define unicode prefix for officium identity (display/params only, not in directory name).
6. Rename pace silks to match new scope.

**[260325-0800] rough**

Implement jjdxo_invitatory in jjrm_serve_stdio: generate officium ID (YYMMDD-HHMM-XXXX), create .claude/jjm/officia/<id>/ directory, create invitatory commit (action code 'i') with model inventory in body via vvcp_probe, populate ServerInfo.instructions with exchange directory path. Add officium state (ID, exchange path) to jjrm_McpServer struct. This is a new JJK invitatory — leave VOS vvcp_invitatory in vvcp_probe.rs untouched (other kits may use it). Remove lazy invitatory calls from orient and muster.

**[260325-0749] rough**

Implement invitatory at MCP startup in jjrm_serve_stdio: generate officium ID (timestamp+short-token), create per-officium directory (.claude/jjm/officia/<id>/), create invitatory commit with model inventory in body, populate ServerInfo instructions with officium ID and gazette exchange path. Move invitatory from lazy-fire (orient/muster) to deterministic MCP-startup.

### claudemd-officium-protocol (₢AwAAU) [complete]

**[260327-1917] complete**

Update CLAUDE.md JJ configuration section to document officium protocol, AND enforce officium param as required on all non-open commands.

1. Document jjx_open: must be called once at start of each chat, returns officium ID (☉YYMMDD-NNNN format). Add to MCP Command Reference table.
2. Document officium param: required on all subsequent jjx commands — pass the returned ☉-prefixed ID every time.
3. Document officium ID format: ☉ (U+2609 SUN) unicode prefix + YYMMDD-NNNN datestamp-sequence. Prefix required in params, stripped for directory name.
4. Self-healing: if any jjx command fails with officium-not-found, call jjx_open again to recover.
5. Forward-looking note: gazette file exchange via officium directory will be documented after ₢AwAAV lands. For now, gazette I/O continues via existing inline 'input' param.
6. Update Quick Verbs if needed.
7. **Enforce officium param in code**: change jjrm_mcp.rs dispatcher to reject all non-open commands when officium is missing. Currently optional (backward compatible) — make it required. One-line change: return error when p.officium is None for non-open commands.

User will stop all other sessions before testing. This pace must complete before user testing of the officium plumbing.

**[260327-1850] rough**

Update CLAUDE.md JJ configuration section to document officium protocol, AND enforce officium param as required on all non-open commands.

1. Document jjx_open: must be called once at start of each chat, returns officium ID (☉YYMMDD-NNNN format). Add to MCP Command Reference table.
2. Document officium param: required on all subsequent jjx commands — pass the returned ☉-prefixed ID every time.
3. Document officium ID format: ☉ (U+2609 SUN) unicode prefix + YYMMDD-NNNN datestamp-sequence. Prefix required in params, stripped for directory name.
4. Self-healing: if any jjx command fails with officium-not-found, call jjx_open again to recover.
5. Forward-looking note: gazette file exchange via officium directory will be documented after ₢AwAAV lands. For now, gazette I/O continues via existing inline 'input' param.
6. Update Quick Verbs if needed.
7. **Enforce officium param in code**: change jjrm_mcp.rs dispatcher to reject all non-open commands when officium is missing. Currently optional (backward compatible) — make it required. One-line change: return error when p.officium is None for non-open commands.

User will stop all other sessions before testing. This pace must complete before user testing of the officium plumbing.

**[260327-0830] rough**

Update CLAUDE.md JJ configuration section to document officium protocol for phase 1 testing.

1. Document jjx_open: must be called once at start of each chat, returns officium ID (☉YYMMDD-NNNN format). Add to MCP Command Reference table.
2. Document officium param: required on all subsequent jjx commands — pass the returned ☉-prefixed ID every time.
3. Document officium ID format: ☉ (U+2609 SUN) unicode prefix + YYMMDD-NNNN datestamp-sequence. Prefix required in params, stripped for directory name.
4. Self-healing: if any jjx command fails with officium-not-found, call jjx_open again to recover.
5. Forward-looking note: gazette file exchange via officium directory will be documented after ₢AwAAV lands. For now, gazette I/O continues via existing inline 'input' param.
6. Update Quick Verbs if needed.

This pace must complete before user testing of the officium plumbing.

**[260327-0818] rough**

Update CLAUDE.md JJ configuration section to document officium protocol.

1. jjx_open must be called once at start of each chat — returns officium ID
2. All subsequent jjx commands require officium param — pass the returned ID every time
3. Gazette exchange: officium directory contains gazette.md for file I/O between agent and server
4. Agent-side protocol: Write tool to gazette.md path for input operations, Read tool from gazette.md path for output operations
5. If gazette I/O fails (officium dir missing), call jjx_open again — self-healing
6. Add jjx_open to the MCP Command Reference table
7. Update Quick Verbs if needed

This pace must complete before user testing of the officium plumbing.

**[260327-0818] rough**

Update CLAUDE.md JJ configuration section to document officium protocol.

1. jjx_open must be called once at start of each chat — returns officium ID
2. All subsequent jjx commands require officium param — pass the returned ID every time
3. Gazette exchange: officium directory contains gazette.md for file I/O between agent and server
4. Agent-side protocol: Write tool to gazette.md path for input operations, Read tool from gazette.md path for output operations
5. If gazette I/O fails (officium dir missing), call jjx_open again — self-healing
6. Add jjx_open to the MCP Command Reference table
7. Update Quick Verbs if needed

This pace must complete before user testing of the officium plumbing.

**[260325-0749] rough**

Update CLAUDE.md JJ configuration section to document gazette file exchange protocol: officium directory path communicated at MCP startup via instructions, gazette wire format (# slug lede / content), fixed filename within officium directory, how to write gazette input (Write tool to path, then call operation), how to read gazette output (Read tool from path after operation). This must land before the gazette migration pace so agents know the protocol.

### implement-compline-mcp-shutdown (₢AwAAR) [abandoned]

**[260327-0818] abandoned**

Implement jjdxo_compline in jjrm_serve_stdio: after service.waiting().await returns, attempt 'officium over' commit (action code 'o', empty commit, best-effort), then remove the exchange directory. Thread officium ID from invitatory through server state to compline — do not re-derive. Handle signal-kill gracefully: if compline doesn't run, the stale directory remains for absolve.

**[260325-0800] rough**

Implement jjdxo_compline in jjrm_serve_stdio: after service.waiting().await returns, attempt 'officium over' commit (action code 'o', empty commit, best-effort), then remove the exchange directory. Thread officium ID from invitatory through server state to compline — do not re-derive. Handle signal-kill gracefully: if compline doesn't run, the stale directory remains for absolve.

**[260325-0749] rough**

Implement compline at MCP shutdown: after service.waiting().await returns in jjrm_serve_stdio, attempt 'officium over' closing commit, remove per-officium directory. Handle crash case gracefully (directory may remain for absolve to clean).

### spec-chapter-absolve-operations (₢AwAAZ) [complete]

**[260327-1922] complete**

Spec jjx_chapter and jjx_absolve operations in JJS0 BEFORE implementation.

1. Define jjdo_chapter operation: scan .claude/jjm/officia/ directories, report officium identity (☉-prefixed), creation time, heartbeat mtime (last activity), staleness status. Mark caller's own officium distinctly in output (identified by officium param). Tabular output. Bypasses Gallops command lifecycle lock (infrastructure, not heat-affiliated). Takes officium param.
2. Define jjdo_absolve operation: remove officia directories exceeding staleness threshold. Report what was removed. Refuses to remove caller's own officium (identified by officium param). Bypasses Gallops lock. Takes officium param.
3. Add both to operation registry with appropriate axhe* voicing annotations.
4. Clarify relationship between absolve and jjx_open exsanguination: absolve is explicit/diagnostic (user-invoked, reports results), exsanguination is automatic/silent (runs within jjx_open, no output).

This pace is spec-only — no code changes. Implementation follows in ₢AwAAS.

**[260327-0830] rough**

Spec jjx_chapter and jjx_absolve operations in JJS0 BEFORE implementation.

1. Define jjdo_chapter operation: scan .claude/jjm/officia/ directories, report officium identity (☉-prefixed), creation time, heartbeat mtime (last activity), staleness status. Mark caller's own officium distinctly in output (identified by officium param). Tabular output. Bypasses Gallops command lifecycle lock (infrastructure, not heat-affiliated). Takes officium param.
2. Define jjdo_absolve operation: remove officia directories exceeding staleness threshold. Report what was removed. Refuses to remove caller's own officium (identified by officium param). Bypasses Gallops lock. Takes officium param.
3. Add both to operation registry with appropriate axhe* voicing annotations.
4. Clarify relationship between absolve and jjx_open exsanguination: absolve is explicit/diagnostic (user-invoked, reports results), exsanguination is automatic/silent (runs within jjx_open, no output).

This pace is spec-only — no code changes. Implementation follows in ₢AwAAS.

**[260327-0822] rough**

Spec jjx_chapter and jjx_absolve operations in JJS0 BEFORE implementation.

1. Define jjdo_chapter operation: scan .claude/jjm/officia/ directories, report officium identity, creation time, heartbeat mtime (last activity), staleness status. Tabular output. Bypasses Gallops command lifecycle lock (infrastructure, not heat-affiliated). Takes officium param (to mark caller's own session).
2. Define jjdo_absolve operation: remove officia directories exceeding staleness threshold. Report what was removed. Refuses to remove caller's own officium (identified by officium param). Bypasses Gallops lock.
3. Add both to operation registry with appropriate axhe* voicing annotations.
4. Clarify relationship between absolve and jjx_open exsanguination (absolve is explicit/diagnostic, exsanguination is automatic/silent).

This pace is spec-only — no code changes. Implementation follows in •AwAAS.

### implement-chapter-and-absolve (₢AwAAS) [complete]

**[260327-1930] complete**

Implement jjx_chapter and jjx_absolve as diagnostic operations.

jjx_chapter: scan .claude/jjm/officia/ directories, report officium identity, creation time, heartbeat mtime (last activity), age, and liveness status in tabular format. Liveness determined by heartbeat mtime recency. These operations do not touch Gallops — they bypass the command lifecycle lock.

jjx_absolve: remove officia directories whose heartbeat exceeds the staleness threshold. Report what was removed. Refuse to remove the caller's own officium (identified by the officium param). Add MCP dispatch entries and param structs for both operations.

**[260327-0819] rough**

Implement jjx_chapter and jjx_absolve as diagnostic operations.

jjx_chapter: scan .claude/jjm/officia/ directories, report officium identity, creation time, heartbeat mtime (last activity), age, and liveness status in tabular format. Liveness determined by heartbeat mtime recency. These operations do not touch Gallops — they bypass the command lifecycle lock.

jjx_absolve: remove officia directories whose heartbeat exceeds the staleness threshold. Report what was removed. Refuse to remove the caller's own officium (identified by the officium param). Add MCP dispatch entries and param structs for both operations.

**[260327-0819] rough**

Implement jjx_chapter and jjx_absolve as diagnostic operations.

jjx_chapter: scan .claude/jjm/officia/ directories, report officium identity, creation time, heartbeat mtime (last activity), age, and liveness status in tabular format. Liveness determined by heartbeat mtime recency. These operations do not touch Gallops — they bypass the command lifecycle lock.

jjx_absolve: remove officia directories whose heartbeat exceeds the staleness threshold. Report what was removed. Refuse to remove the caller's own officium (identified by the officium param). Add MCP dispatch entries and param structs for both operations.

**[260325-0801] rough**

Implement jjx_chapter and jjx_absolve. Chapter: scan .claude/jjm/officia/ directories, report identity, age, liveness status in tabular format. Absolve: remove directories whose owning process is gone, refuse to remove own directory. Liveness mechanism: write a PID file (or officium metadata JSON) at invitatory; chapter/absolve check if PID is still alive. Add MCP dispatch entries, param structs. These operations do not touch Gallops — they bypass the command lifecycle lock.

**[260325-0749] rough**

Implement jjx_chapter (list active officia: directory scan of .claude/jjm/officia/, show ID, creation time, whether current process or stale) and jjx_absolve (clear stale officia: remove directories for sessions that are no longer running). Add MCP tool dispatch, param structs, and handler modules.

### wire-gazette-to-officium-exchange (₢AwAAV) [complete]

**[260327-1952] complete**

Migrate gazette I/O to officium exchange directory.

Exchange path derived from the officium param on each jjx call (not server state). Dispatcher resolves officium ID to .claude/jjm/officia/<id>/gazette.md and passes path to handlers.

Output side: orient, show, paddock getter write gazette.md in officium dir instead of appending gazette markdown to MCP response text.

Input side: enroll, redocket, paddock setter read gazette.md from officium dir instead of inline 'input' param. Remove 'input' field from affected MCP param structs.

The exchange_path threading from dispatcher to handlers is the key structural change. JJK must remain fully functional throughout — test each migrated operation.

**[260327-0819] rough**

Migrate gazette I/O to officium exchange directory.

Exchange path derived from the officium param on each jjx call (not server state). Dispatcher resolves officium ID to .claude/jjm/officia/<id>/gazette.md and passes path to handlers.

Output side: orient, show, paddock getter write gazette.md in officium dir instead of appending gazette markdown to MCP response text.

Input side: enroll, redocket, paddock setter read gazette.md from officium dir instead of inline 'input' param. Remove 'input' field from affected MCP param structs.

The exchange_path threading from dispatcher to handlers is the key structural change. JJK must remain fully functional throughout — test each migrated operation.

**[260327-0819] rough**

Migrate gazette I/O to officium exchange directory.

Exchange path derived from the officium param on each jjx call (not server state). Dispatcher resolves officium ID to .claude/jjm/officia/<id>/gazette.md and passes path to handlers.

Output side: orient, show, paddock getter write gazette.md in officium dir instead of appending gazette markdown to MCP response text.

Input side: enroll, redocket, paddock setter read gazette.md from officium dir instead of inline 'input' param. Remove 'input' field from affected MCP param structs.

The exchange_path threading from dispatcher to handlers is the key structural change. JJK must remain fully functional throughout — test each migrated operation.

**[260325-0801] rough**

Migrate gazette I/O to officium exchange directory. Server side: jjrm_McpServer carries exchange_path as state; dispatch code passes it to output handlers (orient, show, paddock getter) which write gazette.md instead of appending gazette markdown to MCP response. Input side: handlers for enroll, redocket, paddock setter read gazette.md from exchange_path instead of inline 'input' param. Remove 'input' field from MCP param structs. The exchange_path threading from server state to handlers is the key structural change.

**[260325-0749] rough**

Migrate gazette I/O to use officium directory: input operations (enroll, redocket, paddock setter) read gazette from officium directory file instead of inline input param. Output operations (orient, show detail, paddock getter) write gazette to officium directory file instead of appending to MCP response text. Remove inline input params from MCP param structs. The gazette file becomes the exchange medium, not MCP JSON strings.

### remove-legacy-duplicate-output (₢AwAAW) [complete]

**[260327-1955] complete**

Remove legacy duplicate output from orient, show detail, and paddock getter. After gazette migration (₢AwAAV), these operations write structured content (paddock, docket, pace entries) to gazette.md. Remove the inline legacy emission: 'Paddock-content:' indented block from orient, '## Paddock' section from show detail, raw paddock dump from paddock getter. Unique tabular output (racing-heats table, recent-work table, file-touch bitmap, swim lanes) remains in the MCP response — only the duplicated gazette content is removed.

Also update CLAUDE.md to reflect the new output shape: agents now read gazette content from the exchange file, not from inline MCP response text.

**[260327-0830] rough**

Remove legacy duplicate output from orient, show detail, and paddock getter. After gazette migration (₢AwAAV), these operations write structured content (paddock, docket, pace entries) to gazette.md. Remove the inline legacy emission: 'Paddock-content:' indented block from orient, '## Paddock' section from show detail, raw paddock dump from paddock getter. Unique tabular output (racing-heats table, recent-work table, file-touch bitmap, swim lanes) remains in the MCP response — only the duplicated gazette content is removed.

Also update CLAUDE.md to reflect the new output shape: agents now read gazette content from the exchange file, not from inline MCP response text.

**[260325-0801] rough**

Remove legacy duplicate output from orient, show detail, and paddock getter. After gazette migration, these operations write structured content (paddock, docket, pace entries) to gazette.md. Remove the inline legacy emission: 'Paddock-content:' indented block from orient, '## Paddock' section from show detail, raw paddock dump from paddock getter. Unique tabular output (racing-heats table, recent-work table, file-touch bitmap, swim lanes) remains in the MCP response — only the duplicated gazette content is removed.

**[260325-0749] rough**

Remove legacy duplicate output from orient, show detail, and paddock getter. Currently each emits content twice: once in legacy format (indented Paddock-content:, ## Paddock, raw text) then again in gazette wire format. After gazette migration, the gazette file IS the structured output — remove the legacy inline emission. Operations return concise status text only; structured content lives in the gazette file.

### jjs0-officium-lifecycle-spec-update (₢AwAAX) [complete]

**[260327-2004] complete**

Final JJS0 spec verification and reconciliation pass.

The foundational spec work was done in ₢AwAAY (officium lifecycle) and ₢AwAAZ (chapter/absolve). This pace verifies spec-vs-implementation alignment after all code paces (Q, S, V, W) are complete.

1. Reconcile any drift between spec (written before implementation) and actual built behavior. Update spec where implementation legitimately diverged.
2. Verify gazette exchange spec references officium param as path source and matches the wiring in ₢AwAAV.
3. Verify JJSCGZ subdocument aligns with implemented file exchange protocol.
4. Ensure operation specs for jjx_open, jjx_chapter, jjx_absolve match their implementations.
5. Review all officium-related linked terms for consistency.
6. Remove any residual references to legacy inline MCP parameter style for multiline content.

**[260327-0831] rough**

Final JJS0 spec verification and reconciliation pass.

The foundational spec work was done in ₢AwAAY (officium lifecycle) and ₢AwAAZ (chapter/absolve). This pace verifies spec-vs-implementation alignment after all code paces (Q, S, V, W) are complete.

1. Reconcile any drift between spec (written before implementation) and actual built behavior. Update spec where implementation legitimately diverged.
2. Verify gazette exchange spec references officium param as path source and matches the wiring in ₢AwAAV.
3. Verify JJSCGZ subdocument aligns with implemented file exchange protocol.
4. Ensure operation specs for jjx_open, jjx_chapter, jjx_absolve match their implementations.
5. Review all officium-related linked terms for consistency.
6. Remove any residual references to legacy inline MCP parameter style for multiline content.

**[260327-0819] rough**

Update JJS0 spec to reflect agent-initiated officium lifecycle.

1. Update jjdxo_officium entity: identity format YYMMDD-NNNN (not YYMMDD-HHMM-XXXX), agent-initiated via jjx_open (not MCP server startup), heartbeat liveness (not PID), exsanguination on open (not compline cleanup)
2. Update jjdxo_invitatory: triggered by jjx_open operation, probe once per day via datestamp file, officium param required on all subsequent calls
3. Remove or revise jjdxo_compline: no clean shutdown — staleness detected by heartbeat absence, cleanup via exsanguination at next open
4. Update jjdo_chapter and jjdo_absolve specs for heartbeat-based liveness
5. Ensure gazette exchange spec references officium param as path source
6. Verify JJSCGZ subdocument aligns with implemented file exchange protocol

**[260327-0819] rough**

Update JJS0 spec to reflect agent-initiated officium lifecycle.

1. Update jjdxo_officium entity: identity format YYMMDD-NNNN (not YYMMDD-HHMM-XXXX), agent-initiated via jjx_open (not MCP server startup), heartbeat liveness (not PID), exsanguination on open (not compline cleanup)
2. Update jjdxo_invitatory: triggered by jjx_open operation, probe once per day via datestamp file, officium param required on all subsequent calls
3. Remove or revise jjdxo_compline: no clean shutdown — staleness detected by heartbeat absence, cleanup via exsanguination at next open
4. Update jjdo_chapter and jjdo_absolve specs for heartbeat-based liveness
5. Ensure gazette exchange spec references officium param as path source
6. Verify JJSCGZ subdocument aligns with implemented file exchange protocol

**[260325-0749] rough**

Update JJS0 spec to reflect gazette file I/O as the canonical exchange mechanism. Remove any residual references to legacy inline MCP parameter style for multiline content. Ensure operation specs reference gazette slugs and officium directory for input/output. Verify JJSCGZ subdocument aligns with implemented file exchange protocol. This is the spec catching up to the implementation migration.

### test-officium-lifecycle (₢AwAAT) [complete]

**[260327-2118] complete**

Test officium lifecycle end-to-end with the agent-initiated model.

1. Verify jjx_open creates directory, gazette.md, heartbeat, and invitatory commit.
2. Verify jjx_open returns ☉-prefixed officium ID.
3. Verify every jjx command rejects missing/invalid officium param.
4. Verify heartbeat mtime updates on each jjx call.
5. Verify exsanguination at jjx_open reaps stale directories but preserves active ones.
6. Verify probe runs once per day (datestamp file check).
7. Verify concurrent officia coexist (two directories, isolated gazette I/O).
8. Verify self-healing: delete officium dir, next jjx call fails, jjx_open recovers.
9. Verify jjx_chapter lists active officia with correct liveness status and marks caller's own.
10. Verify jjx_absolve reaps stale dirs, refuses to remove caller's own.
11. Verify gazette read/write flow through officium exchange path.

**[260327-0831] rough**

Test officium lifecycle end-to-end with the agent-initiated model.

1. Verify jjx_open creates directory, gazette.md, heartbeat, and invitatory commit.
2. Verify jjx_open returns ☉-prefixed officium ID.
3. Verify every jjx command rejects missing/invalid officium param.
4. Verify heartbeat mtime updates on each jjx call.
5. Verify exsanguination at jjx_open reaps stale directories but preserves active ones.
6. Verify probe runs once per day (datestamp file check).
7. Verify concurrent officia coexist (two directories, isolated gazette I/O).
8. Verify self-healing: delete officium dir, next jjx call fails, jjx_open recovers.
9. Verify jjx_chapter lists active officia with correct liveness status and marks caller's own.
10. Verify jjx_absolve reaps stale dirs, refuses to remove caller's own.
11. Verify gazette read/write flow through officium exchange path.

**[260327-0820] rough**

Test officium lifecycle end-to-end with the agent-initiated model.

1. Verify jjx_open creates directory, gazette.md, heartbeat, and invitatory commit
2. Verify every jjx command rejects missing officium param
3. Verify heartbeat mtime updates on each jjx call
4. Verify exsanguination at jjx_open reaps stale directories but preserves active ones
5. Verify probe runs once per day (datestamp file check)
6. Verify concurrent officia coexist (two directories, isolated gazette I/O)
7. Verify self-healing: delete officium dir, next jjx call fails, jjx_open recovers
8. Verify gazette read/write flow through officium exchange path

**[260327-0820] rough**

Test officium lifecycle end-to-end with the agent-initiated model.

1. Verify jjx_open creates directory, gazette.md, heartbeat, and invitatory commit
2. Verify every jjx command rejects missing officium param
3. Verify heartbeat mtime updates on each jjx call
4. Verify exsanguination at jjx_open reaps stale directories but preserves active ones
5. Verify probe runs once per day (datestamp file check)
6. Verify concurrent officia coexist (two directories, isolated gazette I/O)
7. Verify self-healing: delete officium dir, next jjx call fails, jjx_open recovers
8. Verify gazette read/write flow through officium exchange path

**[260325-0749] rough**

Test officium lifecycle end-to-end: verify invitatory creates directory and commit at MCP startup, compline removes directory and creates closing commit, chapter lists active officia correctly, absolve reaps stale directories. Test concurrent session isolation (two officia directories coexist). Test crash recovery (stale directory survives for absolve).

### gazette-split-input-output (₢AwAAb) [complete]

**[260328-0600] complete**

## Character
Spook fix — operational experience revealed a directional ambiguity in the gazette file exchange. Spec + implementation + documentation, narrow scope.

## Spook
₣Av chat (₢AvAAV): `jjx_orient` wrote paddock+docket to `gazette.md`. Agent then called `jjx_enroll` without overwriting the gazette. Server consumed stale orient output as enroll input. Single-file gazette couldn't distinguish directions.

## Design

Split `gazette.md` into two directional files:
- **`gazette_in.md`** — Agent → Server. Functionally a parameter.
- **`gazette_out.md`** — Server → Agent. Functionally a return value.

### Universal Entry Rule

Every jjx call reads+deletes `gazette_in.md` and deletes `gazette_out.md` before dispatch. Single-MCP-call lifetime — gazette content is a parameter or return value, not persistent state.

## Status
Implementation complete and verified. All test scenarios pass.

**[260328-0558] rough**

## Character
Spook fix — operational experience revealed a directional ambiguity in the gazette file exchange. Spec + implementation + documentation, narrow scope.

## Spook
₣Av chat (₢AvAAV): `jjx_orient` wrote paddock+docket to `gazette.md`. Agent then called `jjx_enroll` without overwriting the gazette. Server consumed stale orient output as enroll input. Single-file gazette couldn't distinguish directions.

## Design

Split `gazette.md` into two directional files:
- **`gazette_in.md`** — Agent → Server. Functionally a parameter.
- **`gazette_out.md`** — Server → Agent. Functionally a return value.

### Universal Entry Rule

Every jjx call reads+deletes `gazette_in.md` and deletes `gazette_out.md` before dispatch. Single-MCP-call lifetime — gazette content is a parameter or return value, not persistent state.

## Status
Implementation complete and verified. All test scenarios pass.

**[260328-0545] rough**

## Character
Spook fix — operational experience revealed a directional ambiguity in the gazette file exchange. Spec + implementation + documentation, narrow scope.

## Spook
₣Av chat (₢AvAAV): `jjx_orient` wrote paddock+docket to `gazette.md`. Agent then called `jjx_enroll` without overwriting the gazette. Server consumed the stale orient output as enroll input, failing on `# paddock` slug (expected `# slate`). Recoverable but cost a round-trip.

Root cause: a single `gazette.md` serves both directions. Stale output from a getter is indistinguishable from fresh input for a setter at the file layer.

## Design

Split `gazette.md` into two directional files:
- **`gazette_in.md`** — Agent → Server. Functionally a parameter: exists only for the single MCP call that consumes it.
- **`gazette_out.md`** — Server → Agent. Functionally a return value: exists only until the next MCP call.

### Universal Entry Rule

Every jjx MCP call, unconditionally, before dispatch:
1. Read `gazette_in.md` into memory (if present)
2. Delete `gazette_in.md`
3. Delete `gazette_out.md`

No exceptions, no branching. After step 3, the officium directory is always clean. Gazette content has single-MCP-call lifetime — it is a parameter or a return value, not persistent state.

### Setter Commands (enroll, redocket, paddock set)
- Use in-memory gazette content from entry rule step 1
- If content was absent: fail with "no input gazette"

### Getter Commands (orient, show, paddock get)
- After processing: write fresh `gazette_out.md`
- Always writing to an absent file (entry rule step 3 guarantees this)

### Non-gazette Commands (list, log, alter, record, close, ...)
- In-memory gazette content (if any was read) is silently discarded
- No `gazette_out.md` written

### Error Cases
- **Agent writes gazette_in, calls wrong command**: input is read, deleted, discarded. Agent re-writes. This is correct — calling the wrong command is an agent error, and the agent has the content in conversation context.
- **Agent writes gazette_in, setter fails validation**: input is already deleted (entry rule). Agent re-writes and retries. Cost is one file write, not creative work.
- **Stale gazette_out from prior getter**: impossible — deleted on entry of every subsequent call.

### Agent Workflow: Read-Modify-Write Paddock
Getter writes paddock to `gazette_out.md`. Agent renames `gazette_out.md` → `gazette_in.md`, edits content, calls paddock setter. Clean cycle — no copy-paste between files.

### Bidirectional Slug (paddock)
The `paddock` slug is marked bidirectional in JJSCGZ. This means it appears in `gazette_in.md` when writing and `gazette_out.md` when reading — not that a single operation touches both files. Each operation is unidirectional.

## Scope

### JJSCGZ-gazette.adoc
- Add file-level I/O protocol section: two files, universal entry rule, single-call lifetime
- Update slug direction annotations to reference `gazette_in.md` / `gazette_out.md`
- Remove any language implying file persistence across calls

### JJS0_JobJockeySpec.adoc
- Line 151: update `:jjdxo_gazette:` attribute (split or generalize)
- Line 383: officium directory layout `gazette.md` → `gazette_in.md` + `gazette_out.md`
- Update prose referencing gazette exchange path

### Rust: jjrm_mcp.rs (primary implementation target)
- Line 47: `GAZETTE_FILE` → `GAZETTE_IN_FILE` + `GAZETTE_OUT_FILE`
- Line 445: `zjjrm_gazette_path()` → two path functions or parameterize
- New: universal entry rule implementation before dispatch (read+delete in, delete out)
- Lines 786–1051: refactor per-command gazette I/O:
  - Setters: use in-memory content from entry rule (no file reads)
  - Getters: write to `gazette_out.md` (no file reads)
  - Eliminate current "write empty string" cleanup calls (entry rule handles it)

### Rust: jjrz_gazette.rs / jjtz_gazette.rs
- No changes expected — gazette data structure (parse, build, emit, query) is file-agnostic

### CLAUDE.md
- Line 523: rewrite gazette exchange description with universal entry rule
- Line 534: two paths, simplified protocol (no "read existing file first" caveat)
- Update setter table: `gazette.md` → `gazette_in.md`
- Add rename workflow hint for read-modify-write paddock

### Specs referencing gazette (check for path references)
- JJSCSL-slate.adoc, JJSCCU-curry.adoc — update if they reference `gazette.md` path

## Verification
- Every jjx call leaves officium directory clean of gazette files after entry rule
- Setter commands fail cleanly with "no input gazette" when `gazette_in.md` is absent
- Getter commands always produce fresh `gazette_out.md` (never overwriting — file is always absent)
- Stale gazette content never survives across any MCP boundary
- All existing workflows work: orient→mount, slate, reslate, paddock get/set
- Rename workflow works: paddock get → rename out→in → edit → paddock set

**[260327-2154] rough**

## Character
Spook fix — operational experience revealed a directional ambiguity in the gazette file exchange. Spec + implementation + documentation, narrow scope.

## Spook
₣Av chat (₢AvAAV): `jjx_orient` wrote paddock+docket to `gazette.md`. Agent then called `jjx_enroll` without overwriting the gazette. Server consumed the stale orient output as enroll input, failing on `# paddock` slug (expected `# slate`). The error was recoverable but cost a round-trip and required the agent to reason about stale file contents.

Root cause: a single `gazette.md` file serves as both the server's output channel (orient, show, paddock getter) and the agent's input channel (enroll, redocket, paddock setter). Stale output is indistinguishable from fresh input at the file layer.

## Fix
Split `gazette.md` into two directional files:

- **`gazette_in.md`** (Agent → Server): agent writes before setter commands. Server reads and **deletes after consuming**. Absence when a setter command runs → clear "no input gazette" error.
- **`gazette_out.md`** (Server → Agent): server writes after getter commands. Overwrites any prior content. Stale output is harmless — cannot be confused with input.

## Scope

### JJSCGZ-gazette.adoc
- Add file-level I/O protocol section describing the two files and their lifecycles
- Existing slug direction annotations (`input`, `output`, `bidirectional`) already map: input slugs appear in `gazette_in.md`, output slugs appear in `gazette_out.md`, bidirectional slugs appear in whichever file matches the operation's direction

### JJS0_JobJockeySpec.adoc
- Update officium directory layout to show `gazette_in.md` + `gazette_out.md` instead of `gazette.md`
- Update any prose referencing the gazette exchange path

### Rust implementation
- Change gazette file read/write paths in the I/O layer
- Add delete-after-consume for input file reads
- Gazette data structure (parse, build, emit, query) is unchanged

### CLAUDE.md
- Update gazette wire format section: paths, protocol, setter/getter table
- Simplify: "write gazette_in.md then call" / "call then read gazette_out.md" — no more "read existing file first" caveat

## Verification
- Setter commands fail cleanly with "no input gazette" when `gazette_in.md` is absent
- Getter commands always produce fresh `gazette_out.md`
- Stale `gazette_out.md` from a prior getter never interferes with a subsequent setter
- All existing JJK operations work: orient→mount, slate, reslate, paddock get/set

### gazette-split-test-pace (₢AwAAc) [abandoned]

**[260328-0558] abandoned**

## Character
Temporary test pace to verify gazette_in.md enroll path. Drop immediately after verification.

**[260328-0557] rough**

## Character
Temporary test pace to verify gazette_in.md enroll path. Drop immediately after verification.

### align-wire-slugs-to-jjezs-minted-names (₢AwAAd) [complete]

**[260328-0801] complete**

## Character

Mechanical but precise — four string constants drive all wire format output, but the change ripples through tests, specs, and CLAUDE.md examples. Attention to completeness over creativity.

## Docket

Align gazette wire format slug tokens to their spec-minted `jjezs_*` identifiers. Currently the wire uses bare words (`slate`, `reslate`, `paddock`, `pace`); the spec defines these as `jjezs_slate`, `jjezs_reslate`, `jjezs_paddock`, `jjezs_pace`. Every persistent identifier earns its prefix.

Update sites:
- `jjrz_gazette.rs` — four `JJRZ_SLUG_*` string constants
- `jjtz_gazette.rs` — all test gazette content using bare slugs
- `JJSCGZ-gazette.adoc` — wire format examples if any use bare slugs
- `CLAUDE.md` — JJK context section gazette examples

**[260328-0745] rough**

## Character

Mechanical but precise — four string constants drive all wire format output, but the change ripples through tests, specs, and CLAUDE.md examples. Attention to completeness over creativity.

## Docket

Align gazette wire format slug tokens to their spec-minted `jjezs_*` identifiers. Currently the wire uses bare words (`slate`, `reslate`, `paddock`, `pace`); the spec defines these as `jjezs_slate`, `jjezs_reslate`, `jjezs_paddock`, `jjezs_pace`. Every persistent identifier earns its prefix.

Update sites:
- `jjrz_gazette.rs` — four `JJRZ_SLUG_*` string constants
- `jjtz_gazette.rs` — all test gazette content using bare slugs
- `JJSCGZ-gazette.adoc` — wire format examples if any use bare slugs
- `CLAUDE.md` — JJK context section gazette examples

### test-wire-verify (₢AwAAe) [abandoned]

**[260328-0759] abandoned**

## Character
Mechanical verification — confirm gazette wire format slug prefixes.

## Docket
This is a test pace to verify gazette_out.md header slug format.

**[260328-0759] rough**

## Character
Mechanical verification — confirm gazette wire format slug prefixes.

## Docket
This is a test pace to verify gazette_out.md header slug format.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 M design-entity-voicing-with-jjf-exemplar
  2 O implement-gazette-rust
  3 N rename-axho-markers-to-new-convention
  4 K jjk-v4-diagnose-mcp-integration
  5 H spec-paddock-with-axla-voicing
  6 L retire-jjdo-revise-docket
  7 I implement-jjf-input-operations
  8 J implement-jjf-output-operations
  9 B axhe-taxonomy-pilot
  10 C axhe-data-model-migration
  11 D axhe-arguments-and-remaining-voicing
  12 E spec-gap-closure
  13 F impl-vs-spec-reassessment
  14 P spec-officium-lifecycle
  15 a claudemd-firemark-coronet-case-sensitivity
  16 Y spec-agent-initiated-officium
  17 Q implement-jjx-open-and-officium-param
  18 U claudemd-officium-protocol
  19 Z spec-chapter-absolve-operations
  20 S implement-chapter-and-absolve
  21 V wire-gazette-to-officium-exchange
  22 W remove-legacy-duplicate-output
  23 X jjs0-officium-lifecycle-spec-update
  24 T test-officium-lifecycle
  25 b gazette-split-input-output
  26 d align-wire-slugs-to-jjezs-minted-names

MONKHLIJBCDEFPaYQUZSVWXTbd
·········xxx··x··x··xx··x· CLAUDE.md
·········xxx·x·x··x···x·x· JJS0_JobJockeySpec.adoc
···xx·x·········xx·xx···x· jjrm_mcp.rs
x···xx··xx················ JJS0-GallopsData.adoc
·······x·····x······xx···· jjrsd_saddle.rs
·x····xx·················x jjrz_gazette.rs, jjtz_gazette.rs
x·x·x····x················ AXLA-Lexicon.adoc
···········x··········x·x· JJSCCU-curry.adoc
·······x············xx···· jjrcu_curry.rs, jjrpd_parade.rs
··x······x···x············ VOS0-VoxObscuraSpec.adoc
·x···········x············ lib.rs
x·······················x· JJSCGZ-gazette.adoc
x········x················ MCM-MetaConceptModel.adoc
·························x RCG-RustCodingGuide.md, jjk-claude-context.md
······················x··· JJSCSL-slate.adoc
················x········· .gitignore, jjrnm_markers.rs
·············x············ Cargo.lock, Cargo.toml, jjrmu_muster.rs, vorm_main.rs, vvcp_probe.rs
···········x·············· JJSCWP-wrap.adoc
··········x··············· JJSCLD-landing.adoc, JJSCRT-retire.adoc
·········x················ JJSCPD-parade.adoc, JJSCSD-saddle.adoc
····x····················· jjrg_gallops.rs, jjrt_types.rs, jjrtl_tally.rs, jjru_util.rs
···x······················ memo-20260318-mcp-server-aggregation-constraint.md
··x······················· RBSAA-ark_abjure.adoc, RBSAC-ark_conjure.adoc, RBSAG-ark_graft.adoc, RBSAI-ark_inspect.adoc, RBSAS-ark_summon.adoc, RBSAV-ark_vouch.adoc, RBSCK-consecration_check.adoc, RBSDC-depot_create.adoc, RBSDD-depot_destroy.adoc, RBSDI-director_create.adoc, RBSDL-depot_list.adoc, RBSDV-director_vouch.adoc, RBSGR-governor_reset.adoc, RBSID-image_delete.adoc, RBSIR-image_retrieve.adoc, RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, RBSQB-quota_build.adoc, RBSRC-retriever_create.adoc, RBSRI-rubric_inscribe.adoc, RBSSD-sa_delete.adoc, RBSSL-sa_list.adoc, RBSTB-trigger_build.adoc, VOSRC-commit.adoc, VOSRG-guard.adoc, VOSRI-init.adoc, VOSRL-lock.adoc, VOSRP-probe.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 143 commits)

  1 a claudemd-firemark-coronet-case-sensitivity
  2 Y spec-agent-initiated-officium
  3 Q implement-jjx-open-and-officium-param
  4 U claudemd-officium-protocol
  5 Z spec-chapter-absolve-operations
  6 S implement-chapter-and-absolve
  7 V wire-gazette-to-officium-exchange
  8 W remove-legacy-duplicate-output
  9 X jjs0-officium-lifecycle-spec-update
  10 T test-officium-lifecycle
  11 b gazette-split-input-output
  12 d align-wire-slugs-to-jjezs-minted-names

123456789abcdefghijklmnopqrstuvwxyz
·xx································  a  2c
···xx······························  Y  2c
·····xxx···························  Q  3c
········xx·························  U  2c
··········xx·······················  Z  2c
············xx·····················  S  2c
··············xx···················  V  2c
················xx·················  W  2c
··················xx···············  X  2c
····················x··············  T  1c
·······················x····x······  b  2c
·································xx  d  2c
```

## Steeplechase

### 2026-03-28 08:01 - ₢AwAAd - W

Aligned gazette wire format slugs to spec-minted jjezs_* identifiers (slate→jjezs_slate, reslate→jjezs_reslate, paddock→jjezs_paddock, pace→jjezs_pace). Removed non-load-bearing Levenshtein near-match machinery from parse error path. Promoted slug constants to pub(crate) and eliminated magic strings in tests. Updated CLAUDE.md gazette examples.

### 2026-03-28 08:01 - ₢AwAAd - n

Align gazette wire slugs to jjezs_ prefix and remove non-spec fuzzy matching

### 2026-03-28 07:59 - Heat - T

test-wire-verify

### 2026-03-28 07:59 - Heat - S

test-wire-verify

### 2026-03-28 07:45 - Heat - S

align-wire-slugs-to-jjezs-minted-names

### 2026-03-28 07:09 - Heat - n

Comprehensive gazette protocol documentation audit: fix param signatures (remove gazette-routed params from enroll/redocket/paddock), document gazette_out.md output format per getter command, add mass reslate capability, explain paddock note param, correct H1 delimiter warning for multi-notice case, complete wire format table with positioning params, fix redundant redocket example, update Groom Protocol to read gazette_out.md

### 2026-03-28 06:00 - ₢AwAAb - W

Split gazette into directional gazette_in.md (agent→server) and gazette_out.md (server→agent). Universal entry rule: every jjx call reads+deletes gazette_in and deletes gazette_out before dispatch. Single-MCP-call lifetime — gazette content is a parameter or return value, not persistent state.

### 2026-03-28 06:00 - Heat - d

paddock curried: update paddock fix description to reflect implemented universal entry rule

### 2026-03-28 05:58 - Heat - T

gazette-split-test-pace

### 2026-03-28 05:57 - Heat - S

gazette-split-test-pace

### 2026-03-28 05:56 - Heat - d

paddock curried: test gazette split paddock round-trip

### 2026-03-28 05:53 - ₢AwAAb - n

Split gazette into directional gazette_in.md (agent→server) and gazette_out.md (server→agent). Universal entry rule: every jjx call reads+deletes gazette_in and deletes gazette_out before dispatch. Single-MCP-call lifetime — gazette content is a parameter or return value, not persistent state.

### 2026-03-27 21:55 - Heat - d

paddock curried

### 2026-03-27 21:54 - Heat - S

gazette-split-input-output

### 2026-03-27 21:18 - ₢AwAAT - W

End-to-end officium lifecycle testing: all 11 test points pass — open/create, ID format, officium enforcement, heartbeat liveness, exsanguination, daily probe, concurrent coexistence, self-healing recovery, chapter diagnostics, absolve protection, gazette file exchange.

### 2026-03-27 20:04 - ₢AwAAX - W

Reconcile JJS0 spec with implementation: add gazette.md to open step, add age/fallback/unknown to chapter, add no-heartbeat and error reporting to absolve, add gazette I/O wiring section to dispatch protocol, remove legacy input param references from JJS0/curry/slate specs.

### 2026-03-27 20:04 - ₢AwAAX - n

Spec gazette I/O wiring: add dispatcher lifecycle section for output/input operations, update curry/slate/redocket to document gazette file exchange, remove obsolete inline input param, improve chapter and absolve edge-case handling

### 2026-03-27 19:55 - ₢AwAAW - W

Remove legacy duplicate gazette content from MCP responses: Paddock-content block from orient, ## Paddock section from show detail, raw paddock dump from paddock getter. Gazette file is now the sole source for paddock and pace docket content. Update CLAUDE.md Mount Protocol to reflect gazette file reading.

### 2026-03-27 19:55 - ₢AwAAW - n

₢AwAAW: Move paddock content from command output to gazette-only channel in curry, parade, and saddle; update mount protocol to read paddock from gazette file

### 2026-03-27 19:52 - ₢AwAAV - W

Wired gazette I/O to officium exchange: output handlers (orient/show/paddock-get) add notices to shared gazette object written by dispatcher to officium gazette.md; input operations (enroll/redocket/paddock-set) read and consume gazette.md; removed inline input param from all three setter commands.

### 2026-03-27 19:50 - ₢AwAAV - n

Wire gazette I/O to officium exchange: output handlers add to shared gazette object written by dispatcher, input operations read and consume gazette.md, remove inline input param from enroll/redocket/paddock

### 2026-03-27 19:30 - ₢AwAAS - W

Implement jjx_chapter and jjx_absolve: officia diagnostic operations with tabular status report and stale directory reaping

### 2026-03-27 19:26 - ₢AwAAS - n

Implement jjx_chapter and jjx_absolve: officia diagnostic operations with tabular status report and stale directory reaping

### 2026-03-27 19:22 - ₢AwAAZ - W

Expanded jjdo_chapter and jjdo_absolve stubs to full spec definitions in JJS0: arguments sections with officium param, numbered behavior steps, stdout format, Gallops bypass statements, and NOTE clarifying absolve (explicit/diagnostic) vs exsanguination (automatic/silent).

### 2026-03-27 19:22 - ₢AwAAZ - n

Refine census and absolve spec sections: add officium param, caller-identity marking, structured behavior steps, and clarify absolve vs exsanguination distinction

### 2026-03-27 19:17 - ₢AwAAU - W

Verified all 7 docket items landed: CLAUDE.md documents officium protocol (jjx_open, ☉ identity format, required param, self-healing, gazette forward note, Quick Verbs unchanged), and jjrm_mcp.rs enforces officium param on all non-open commands with error on None.

### 2026-03-27 18:59 - ₢AwAAU - n

Document officium protocol in CLAUDE.md and enforce officium param as required on all non-open jjx commands

### 2026-03-27 18:45 - ₢AwAAQ - W

Implemented jjx_open command and officium param threading. jjx_open performs exsanguination of stale officia, generates YYMMDD-NNNN identity with atomic create_dir race protection, creates exchange directory with gazette.md and heartbeat, runs daily probe, and creates jjb-branded invitatory commit. Dispatcher validates officium param and touches heartbeat on every call. Removed lazy invitatory AtomicBool. Added officia/ to gitignore. Retired COMPLINE marker constant.

### 2026-03-27 18:39 - ₢AwAAQ - n

Atomic exchange directory creation: create_dir with retry loop eliminates same-second race condition in ID generation

### 2026-03-27 17:24 - ₢AwAAQ - n

Implement jjx_open and officium param threading: exsanguination, YYMMDD-NNNN ID generation, exchange directory with heartbeat, daily probe, invitatory commit. Remove lazy invitatory AtomicBool. Add officium validation envelope to dispatcher.

### 2026-03-27 17:01 - ₢AwAAY - W

Revised JJS0 officium lifecycle spec for agent-initiated model. Replaced MCP-process-coupled officium with explicit jjx_open operation, YYMMDD-NNNN identity with ☉ prefix, heartbeat-based liveness detection, exsanguination reaping, retired compline. Restructured dispatch protocol into two-layer design: officium envelope (all commands) and gallops lifecycle (state-touching commands). Updated steeplechase commit patterns, officium operations (open/chapter/absolve), and marked s-session pattern superseded.

### 2026-03-27 16:59 - ₢AwAAY - n

Revise JJS0 officium lifecycle spec: agent-initiated model with jjx_open, heartbeat liveness, exsanguination, retired compline, two-layer dispatch protocol

### 2026-03-27 16:47 - ₢AwAAa - W

Added case-sensitivity warning for firemarks and coronets to CLAUDE.md Job Jockey Configuration section, immediately after the Identities vs Display Names block.

### 2026-03-27 16:47 - ₢AwAAa - n

Add case-sensitivity warning for firemark/coronet identities in CLAUDE.md

### 2026-03-27 08:37 - Heat - r

moved AwAAa to first

### 2026-03-27 08:37 - Heat - D

restring 1 paces from ₣Av

### 2026-03-27 08:34 - Heat - d

paddock curried

### 2026-03-27 08:22 - Heat - S

spec-chapter-absolve-operations

### 2026-03-27 08:22 - Heat - S

spec-agent-initiated-officium

### 2026-03-27 08:20 - Heat - r

moved AwAAT to last

### 2026-03-27 08:20 - Heat - T

test-officium-lifecycle

### 2026-03-27 08:19 - Heat - T

jjs0-officium-lifecycle-spec-update

### 2026-03-27 08:19 - Heat - T

wire-gazette-to-officium-exchange

### 2026-03-27 08:19 - Heat - T

implement-chapter-and-absolve

### 2026-03-27 08:19 - Heat - r

moved AwAAU after AwAAQ

### 2026-03-27 08:18 - Heat - T

claudemd-officium-protocol

### 2026-03-27 08:18 - Heat - T

implement-jjx-open-and-officium-param

### 2026-03-27 08:18 - Heat - T

implement-compline-mcp-shutdown

### 2026-03-25 08:10 - ₢AwAAP - W

Spec officium lifecycle in JJS0: officium entity with identity (YYMMDD-HHMM-XXXX), exchange directory (.claude/jjm/officia/<id>/gazette.md), invitatory/compline internal procedures, chapter/absolve operations. Removed VOS officium entirely: deleted vvcp_invitatory, gap-detection, chrono dep, vvx_invitatory CLI, lazy invitatory calls from orient/muster. VOS0 forwards officium definitions to JJS0. vvcp_probe retained as shared utility.

### 2026-03-25 08:09 - ₢AwAAP - n

Include Cargo.lock update from chrono dependency removal

### 2026-03-25 08:09 - ₢AwAAP - n

Remove VOS officium lifecycle: delete vvcp_invitatory, zvvcp_needs_officium, officium constants, chrono dep, vvx_invitatory CLI command, lazy invitatory calls from orient/muster. Retain vvcp_probe utility. VOS0 forwards officium/invitatory definitions to JJS0.

### 2026-03-25 08:00 - ₢AwAAP - n

Spec officium lifecycle in JJS0: officium entity with identity/directory/exchange, invitatory and compline internal procedures, chapter and absolve operations, gazette file exchange path, updated steeplechase commit patterns and sole_operator premise

### 2026-03-25 07:49 - Heat - S

repair-jjs0-gazette-commitment

### 2026-03-25 07:49 - Heat - S

remove-legacy-duplicate-output

### 2026-03-25 07:49 - Heat - S

migrate-gazette-io-to-officium-directory

### 2026-03-25 07:49 - Heat - S

claudemd-gazette-exchange-protocol

### 2026-03-25 07:49 - Heat - S

test-officium-lifecycle

### 2026-03-25 07:49 - Heat - S

implement-chapter-and-absolve

### 2026-03-25 07:49 - Heat - S

implement-compline-mcp-shutdown

### 2026-03-25 07:49 - Heat - S

implement-invitatory-mcp-startup

### 2026-03-25 07:49 - Heat - S

spec-officium-lifecycle

### 2026-03-24 20:03 - ₢AwAAF - W

Verified axhe migration consistency (zero orphaned CLI annotations), confirmed new operation spec includes resolve, updated ₣Ah paddock with migration handoff summary and V4-relevant findings

### 2026-03-24 20:03 - ₢AwAAE - W

Close JJS0 spec gaps: add jjdo_close and jjdo_paddock operations with subdocs, resolve Bridled as V3-legacy deprecated state, mark 4 vestigial arguments as internal, document unspecified MCP parameter design decision

### 2026-03-24 19:54 - ₢AwAAF - n

Verify axhe migration, update ₣Ah paddock with migration handoff summary, fix stale JJS0-GallopsData.adoc reference

### 2026-03-24 19:40 - ₢AwAAE - n

Close JJS0 spec gaps: add jjdo_close and jjdo_paddock operations with subdocs, resolve Bridled as V3-legacy deprecated state, mark 4 vestigial arguments as internal, document unspecified MCP parameter design decision

### 2026-03-24 19:28 - ₢AwAAD - W

Migrated 13 argument annotations from axa_cli_option/axa_cli_flag to transport-agnostic axa_keyword, stripped dead Long:/Short: CLI metadata. Cross-source audit found and fixed 6 discrepancies between CLAUDE.md, JJS0 specs, and Rust code (phantom archive execute param, landing content optionality, retire dry-run removal, paddock/redocket param optionality). Renamed 3 compound jjx commands to single words: jjx_redocket, jjx_brief, jjx_coronets. Upper API verb annotations (axi_cc_claudemd_verb) deliberately retained as load-bearing CLAUDE.md interface mapping.

### 2026-03-24 19:19 - ₢AwAAD - n

Sync CLAUDE.md and spec subdocs with Rust code: remove phantom execute param from archive, fix paddock/revise_docket optionality, fix landing content optional, collapse retire spec dry-run/execute split

### 2026-03-24 19:13 - ₢AwAAD - n

Replace 13 axa_cli_option/axa_cli_flag annotations with transport-agnostic axa_keyword, strip dead Long:/Short: CLI metadata. Upper API verbs (axi_cc_claudemd_verb) deliberately retained. Section headers left as-is.

### 2026-03-24 08:46 - Heat - T

axhe-arguments-and-remaining-voicing

### 2026-03-24 08:40 - ₢AwAAC - W

Design conversation discovered axr_*/axt_* annotations are already transport-agnostic — no migration needed. Instead: invented mcm_intaglio (wire-level token identity tier alongside lemma and graven), added axt_map with 2-arity (key type + value type) to AXLA, defined axr_member 1-arity for intaglio. Completed JJS0 data model: Tack record (jjdcr_tack, 6 members), missing collection members (heats map, paces map, tacks array), schema_version. Renamed JJS0-GallopsData.adoc to JJS0_JobJockeySpec.adoc. Key design finding: records (wire contract) and entities (behavioral contract) are distinct gestalts — not a carboat.

### 2026-03-24 08:37 - ₢AwAAC - n

Rename JJS0-GallopsData.adoc to JJS0_JobJockeySpec.adoc. Update all active source references in CLAUDE.md, JJSCPD, JJSCSD, VOS0.

### 2026-03-24 08:35 - ₢AwAAC - n

Complete JJS0 data model: add Tack record (jjdcr_tack, 6 members), missing collection members (heats map on Gallops, paces map on Heat, tacks array on Pace), schema_version on Gallops. New axt_map 2-arity pattern used for heats and paces dictionaries. Prefix jjdc_ allocated for Tack.

### 2026-03-24 08:29 - ₢AwAAC - n

Add mcm_intaglio (wire-level token identity tier) to MCM. Add axt_map (2-arity: key type + value type) to AXLA. Define axr_member 1-arity for mcm_intaglio (JSON key). Three-tier identity model: lemma (catalogued), graven (prefix-disciplined), intaglio (wire literal).

### 2026-03-23 19:00 - ₢AwAAB - W

Replaced 22 transport-coupled axi_cli_* annotations with transport-agnostic equivalents in JJS0: 20 operations → axvo_procedure axd_transient, 1 MCP transport → axvo_procedure axd_longrunning, 2 infrastructure → axl_voices axo_entity. Key design discovery: axvo_* and axhe* are complementary (definition-site vs subdocument), not competing — Operation Taxonomy annotations confirmed correct. Dropped axd_grouped (AXLA lookahead would require boilerplate). Reslated AwAAD to remove completed scope. Layer 2 argument voicing deferred.

### 2026-03-23 18:55 - ₢AwAAB - n

Replace 22 transport-coupled axi_cli_* annotations with transport-agnostic equivalents: 20 operations → axvo_procedure axd_transient, 1 MCP transport → axvo_procedure axd_longrunning, 2 infrastructure → axl_voices axo_entity. Drop axd_grouped (AXLA lookahead would require boilerplate). Layer 2 argument voicing deferred to subsequent pace.

### 2026-03-23 18:36 - Heat - d

paddock curried

### 2026-03-23 18:34 - Heat - T

spec-gap-closure

### 2026-03-23 18:34 - Heat - T

axhe-operations-migration

### 2026-03-23 18:34 - Heat - T

axhe-data-model-migration

### 2026-03-23 18:34 - Heat - T

axhe-taxonomy-pilot

### 2026-03-23 18:06 - ₢AwAAJ - W

Wired gazette-based output into three read operations. Added jjrz_build_read_output helper that constructs paddock+pace gazette notices. Orient emits firemark-lede paddock and coronet-lede pace. Show --detail emits paddock plus all displayed paces. Paddock getter emits paddock notice. 6 new round-trip tests including preamble-tolerance verification, 52 gazette tests total.

### 2026-03-23 18:02 - ₢AwAAJ - n

Wire gazette-based output into three read operations (orient, show --detail, paddock getter). Add jjrz_build_read_output helper for paddock+pace gazette construction. 6 new output round-trip tests, 52 gazette tests total, 316 all tests pass.

### 2026-03-23 10:33 - ₢AwAAI - W

Wire gazette-based input into three write operations. Add jjrz_parse_slate_input, jjrz_parse_reslate_input, jjrz_parse_paddock_input parsing functions. Update MCP params (EnrollParams, ReviseDocketParams, PaddockParams) with optional input field for gazette alternative. Mass reslate support via multiple notices in single lock/load/persist cycle. 14 new tests, 46 total gazette tests pass.

### 2026-03-23 10:13 - ₢AwAAI - n

Wire gazette-based input into three write operations. Add jjrz_parse_slate_input, jjrz_parse_reslate_input, jjrz_parse_paddock_input parsing functions. Update MCP params (EnrollParams, ReviseDocketParams, PaddockParams) with optional input field for gazette alternative. Mass reslate support via multiple notices in single lock/load/persist cycle. 14 new tests, 46 total gazette tests pass.

### 2026-03-23 10:04 - ₢AwAAL - W

Retired jjdo_revise_docket from JJS0: removed mapping entry, deleted definition block (anchor, voicing, arguments, behavior), migrated two body references to {jjsoprd_revise_docket}. Zero remaining references. Establishes precedent for incremental jjdo_* retirement.

### 2026-03-23 10:04 - ₢AwAAL - n

Retire jjdo_revise_docket from JJS0: remove mapping entry, delete 19-line definition block, migrate two references to jjsoprd_revise_docket. Sets precedent for jjdo_* term retirement as operations migrate to jjsop/jjsgm tree.

### 2026-03-23 10:02 - ₢AwAAN - W

Renamed operation hierarchy markers to entity voicing convention across AXLA and 29 consuming documents. Three concrete renames (axhop_parameter_from_type→axhopt_typed_parameter, axhop_parameter_from_arg→axhopa_arg_parameter, axhoo_output_of_type→axhoot_typed_output), eliminated two abstract parents, added two new motif markers (axhopm_motif_parameter, axhoom_motif_output). Bare //axhoo_output in VOK specs migrated to //axhoom_motif_output.

### 2026-03-23 09:53 - ₢AwAAN - n

Rename operation hierarchy markers to entity voicing convention: axhop_parameter_from_type→axhopt_typed_parameter, axhop_parameter_from_arg→axhopa_arg_parameter, axhoo_output_of_type→axhoot_typed_output. Eliminate abstract parents (axhop_parameter, axhoo_output). Add new motif markers (axhopm_motif_parameter, axhoom_motif_output). Updated AXLA definitions, summary table, cross-references, and 29 consuming documents across RBK and VOK.

### 2026-03-23 09:44 - ₢AwAAO - W

Implemented Gazette entity in Rust per JJSCGZ-gazette.adoc. Created jjrz_gazette.rs with jjrz_Slug enum (string boundary consts, direction metadata), jjrz_Gazette struct (Cell<bool> freeze-on-disclosure, BTreeMap two-level notice map), six spec methods (parse/build/add/query_by_slug/query_all/emit), near-match Levenshtein diagnostics, and # boundary detection preserving ##+ markdown headers in content. Created jjtz_gazette.rs with 32 tests covering parse, lifecycle, round-trip, directionality, and internal helpers. All tests pass.

### 2026-03-23 09:42 - ₢AwAAO - n

Implement Gazette entity in Rust per JJSCGZ spec: jjrz_Slug enum with string boundary consts and direction metadata, jjrz_Gazette struct with Cell<bool> freeze-on-disclosure, parse/build/add/query/emit methods, near-match diagnostics via Levenshtein distance, notice boundary detection preserving ##+ markdown headers. 32 tests covering parse valid/error cases, build/add/freeze lifecycle, round-trip guarantee, directionality, and internal helpers.

### 2026-03-21 13:33 - Heat - T

design-v4-mcp-file-exchange

### 2026-03-21 13:28 - Heat - S

implement-gazette-rust

### 2026-03-21 13:27 - ₢AwAAM - W

Designed and specified AXLA entity voicing family (axve_entity + 10 axhe*_ hierarchy markers with typed/arg/motif variant naming convention, elevated/scoped method distinction). Created Gazette entity (jjsz_gazette) as founding exemplar — two-level map with slug/lede/notice vocabulary, freeze-on-disclosure invariant, jjezs_* enum values, jjdt_slug type, jjrz_* graven method names, wire format spec. Defined mcm_lemma and mcm_graven as Diptych-era identity concepts in MCM. Slated follow-up pace for axhop_*/axhoo_* rename.

### 2026-03-21 12:49 - ₢AwAAM - n

Define mcm_lemma and mcm_graven as Diptych-era identity concepts in MCM, update AXLA axhems_scoped_method to read a graven name via 1-arity lookahead

### 2026-03-21 11:58 - ₢AwAAM - n

Add jjrz_ prefix to all gazette methods and implementation module reference in entity intro

### 2026-03-21 11:51 - ₢AwAAM - n

Tighten gazette subdocument — consolidate invariants, promote wire format, add jjdt_slug type to JJS0, elevate jjezs_* enum values with mapping entries, merge directionality into slug definitions

### 2026-03-21 09:01 - ₢AwAAM - n

Add Gazette entity (jjsz_gazette) as founding exemplar of entity voicing — slug/lede/notice vocabulary, two-level map structure, freeze-on-disclosure invariant, fully scoped methods (parse, build, add, query, emit)

### 2026-03-20 18:56 - ₢AwAAM - n

Add Gazette entity (jjsz_gazette) as founding exemplar of entity voicing — slug/lede/notice vocabulary, two-level map structure, freeze-on-disclosure invariant, fully scoped methods (parse, build, add, query, emit)

### 2026-03-20 17:44 - ₢AwAAM - n

Add entity voicing family to AXLA — axve_entity definition-site voicing, 10 axhe*_ hierarchy markers with two-layer structure (field/method at Layer 1, parameter/output at Layer 2), elevated/scoped method distinction, typed/arg/motif variant naming convention

### 2026-03-20 17:39 - Heat - S

rename-axho-markers-to-new-convention

### 2026-03-19 18:59 - Heat - S

design-entity-voicing-with-jjf-exemplar

### 2026-03-18 21:35 - ₢AwAAH - W

Nucleated Operation Taxonomy in JJS0 with spec-governed dispatch lifecycle. Discovered jjdk_sole_operator premise — all sessions single operator, lock unconditionally — which eliminated jjsohr_handler_result and collapsed handler signature to Result<String, String>. Implemented dispatch_heat/dispatch_pace split, migrated revise_docket as exemplar. Verified round-trip through new MCP binary.

### 2026-03-18 21:35 - ₢AwAAH - n

Simplify dispatch lifecycle — remove jjrm_HandlerResult, make persist unconditional, split dispatch into heat/pace variants per jjdk_sole_operator premise

### 2026-03-18 21:21 - ₢AwAAH - n

Simplify Operation Taxonomy per jjdk_sole_operator premise — remove jjsohr_handler_result, make dispatch lifecycle unconditional, describe heat/pace affiliation for firemark derivation

### 2026-03-18 20:54 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 20:54 - ₢AwAAH - n

Add dispatch lifecycle (jjsodp_command_lifecycle) and handler result (jjsohr_handler_result) to spec and implement for revise_docket exemplar — dispatcher owns lock/load/persist, handler receives &mut Gallops

### 2026-03-18 20:33 - ₢AwAAH - n

Eliminate redundant coronet parse in prepend_tack — now takes &PaceContext (the parsed-once ADT) instead of &str

### 2026-03-18 20:24 - ₢AwAAH - n

Make jjrg_revise_docket pure — basis and ts captured at procedure boundary, method constructs tack directly without git. Strip bridled auto-reset. Existing jjrg_make_tack unchanged for incremental migration.

### 2026-03-18 20:14 - ₢AwAAH - n

Nucleate Operation Taxonomy in JJS0 with five AXLA-voiced terms (group, procedure, 3 methods) and implement spec-governed decomposition of jjrg_tally — resolve_pace and prepend_tack as shared primitives, revise_docket as composed method, simplified handler

### 2026-03-18 20:06 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 19:41 - Heat - S

retire-jjdo-revise-docket

### 2026-03-18 19:40 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 19:34 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 19:27 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 18:43 - ₢AwAAH - n

Clarify axvo_procedure, axvo_method, and axvo_group slot ordering for future mechanical linting — each primary motif now explicitly declares its attribute reference sequence

### 2026-03-18 18:07 - Heat - T

spec-paddock-with-axla-voicing

### 2026-03-18 16:12 - ₢AwAAK - W

Diagnosed MCP server aggregation issue in Claude Code /context display. Confirmed tunneled dispatcher architecture is sound. Created comprehensive operational memo documenting constraint, implications, and design recommendations. Heat ₣Aw unblocked to proceed with AXLA normalization and JJF file exchange work.

### 2026-03-18 16:12 - ₢AwAAK - n

Document MCP server aggregation constraint — findings, implications, and design recommendations for future kit work

### 2026-03-18 16:09 - ₢AwAAK - n

Revert test MCP command — diagnostic complete, findings documented in paddock

### 2026-03-18 16:08 - ₢AwAAK - n

Add jjx_test_echo tool for MCP aggregation diagnostics — confirms /context issue is Claude Code bug, not JJK architecture

### 2026-03-18 15:59 - Heat - S

jjk-v4-diagnose-mcp-integration

### 2026-03-18 15:57 - Heat - T

unit-test-jjf-format

### 2026-03-18 15:46 - Heat - r

moved AwAAH to first

### 2026-03-18 15:46 - Heat - T

design-v4-mcp-file-exchange

### 2026-03-18 15:46 - Heat - T

unit-test-jjf-format

### 2026-03-18 15:38 - Heat - d

paddock curried

### 2026-03-18 15:38 - Heat - T

mark-volatile-v4-sections

### 2026-03-18 15:35 - Heat - S

implement-jjf-output-operations

### 2026-03-18 15:35 - Heat - S

implement-jjf-input-operations

### 2026-03-18 15:35 - Heat - S

unit-test-jjf-format

### 2026-03-18 15:35 - Heat - T

design-v4-mcp-file-exchange

### 2026-03-18 15:05 - Heat - r

moved AwAAG after AwAAA

### 2026-03-18 15:05 - Heat - D

restring 1 paces from ₣Ah

### 2026-03-17 18:42 - Heat - f

racing

### 2026-03-17 18:36 - Heat - S

impl-vs-spec-reassessment

### 2026-03-17 18:35 - Heat - T

impl-vs-spec-baseline

### 2026-03-17 18:31 - Heat - d

paddock curried

### 2026-03-17 18:31 - Heat - S

normalize-data-model-annotations

### 2026-03-17 18:31 - Heat - S

normalize-operation-annotations

### 2026-03-17 18:31 - Heat - S

surface-axla-gaps

### 2026-03-17 18:31 - Heat - S

impl-vs-spec-baseline

### 2026-03-17 18:30 - Heat - S

mark-volatile-v4-sections

### 2026-03-17 18:30 - Heat - N

jjk-v4-0-jjs0-axla-normalization

