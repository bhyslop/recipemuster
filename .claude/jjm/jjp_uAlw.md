# Paddock: jjk-v4-0-jjs0-axla-normalization

## Purpose

Build V3 schema infrastructure: AXLA annotation vocabulary upgrade and JJF (Job Jockey File) format for markdown-based MCP file exchange.

This heat is **strictly V3 schema**. No breaking changes, no data model rewrites.

## Scope

1. **JJF file exchange protocol** — design and implement markdown-based file I/O for multiline MCP parameters (docket, paddock content). Reduces MCP parameter formatting spooks. **DONE** (gazette entity: ₢AwAAO, ₢AwAAI, ₢AwAAJ).
2. **AXLA annotation migration** — migrate JJS0 from first-generation `axl_voices` annotations (transport-coupled: `axi_cli_subcommand`, `axa_cli_option`) to `axhe*` entity voicing convention (transport-agnostic, structural). JJSCGZ-gazette.adoc is the exemplar.
3. **Specification gaps** — formalize `jjx_close` and `jjx_paddock` operations (in code, missing from spec). Resolve Bridled/Tack V3-legacy status.
4. **Entity voicing convention** — the `axhe*` hierarchy (entity, field, method, parameter, output) replaces transport-specific motifs. Proven in JJSCGZ and the ₢AwAAN rename across 29 documents.

## Key Design Insight (2026-03-23)

The original plan assumed we'd mint new transport-specific AXLA motifs (`axi_mcp_tool`, `axa_mcp_parameter`) to replace the CLI-era annotations. This perpetuates the coupling problem — when transport changes, all annotations break again.

The `axhe*` entity voicing convention is transport-agnostic by design. `axhems_scoped_method` doesn't care if it's served by CLI, MCP, or any future transport. The migration target is `axhe*`, not new `axl_voices` motifs.

## Key Premise Discovered

**jjdk_sole_operator** — All concurrent MCP sessions belong to a single operator. Cross-user concurrency is out of scope. This premise collapsed handler complexity: every operation locks unconditionally, handler signature is `FnOnce(&mut Gallops) -> Result<String, String>`.

## Audit Findings (2026-03-23, in-context, no memo)

Full spec-vs-impl audit was performed in-context during ₢AwAAB mount. Key findings:

- **20 operations** annotated `axi_cli_subcommand` — all are MCP tools, CLI removed
- **13 arguments** annotated `axa_cli_option`/`axa_cli_flag` — all are MCP JSON params
- **Bridled** variant in code, not in current spec enum values (V3 Legacy only)
- **Tack record** only described in V3 Legacy section
- **3 operations** in code without spec: `jjx_close`, `jjx_paddock`, `jjx_revise_docket` (last one has taxonomy-level spec via jjsoprd)
- **16 MCP parameters** unspecified, **4 spec arguments** vestigial
- **Data model alignment is strong**: all records, members, enums match
- **Operation Taxonomy** (resolve_pace, prepend_tack, revise_docket) fully matches

## Migration Strategy

Incremental, piloted:
1. **₢AwAAB**: Pilot on Operation Taxonomy section (6 annotations) — establish mapping
2. **₢AwAAC**: Data model core (records, members, enums, types — ~25 annotations)
3. **₢AwAAD**: Operations + arguments (~50 annotations) — largest change
4. **₢AwAAE**: Spec gap closure (new operation specs, V3-legacy resolutions)
5. **₢AwAAF**: Verification and ₣Ah handoff

## Sequencing

Completes before ₣Ah resumes. ₣Ah is furloughed during this work.

## Heat constellation

| Heat | Silks | Role | Status |
|------|-------|------|--------|
| ₣Aw | jjk-v4-0-jjs0-axla-normalization | V3 infrastructure — annotations + JJF file exchange | Racing |
| ₣Ah | jjk-v4-1-school-breeze-founding | V4 schema transition (separate initiative) | Stabled (furloughed) |
| ₣An | jjk-v4-release-and-legacy-removal | V4 cleanup (separate initiative) | Stabled |
| ₣Am | jjk-v5-notional | Future parking lot | Stabled |