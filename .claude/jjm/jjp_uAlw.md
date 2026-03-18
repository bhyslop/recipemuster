# Paddock: jjk-v4-0-jjs0-axla-normalization

## Purpose

Build V3 schema infrastructure: AXLA annotation vocabulary upgrade and JJF (Job Jockey File) format for markdown-based MCP file exchange.

This heat is **strictly V3 schema**. No breaking changes, no data model rewrites. Annotation-only normalization on existing definitions, plus new infrastructure (JJF parsing/emitting routines) that works with current types.

## Scope

1. **JJF file exchange protocol** — design and implement markdown-based file I/O for multiline MCP parameters (docket, paddock content). Reduces MCP parameter formatting spooks.
2. **AXLA annotation upgrade** — fix first-generation patterns (`//axl_voices axi_cli_subcommand` for all operations) to RBS0-style specificity (correct interface motifs, dimension modifiers).
3. **Specification gaps** — formalize `jjx_curry` operation (currently in code, missing from spec).
4. **AXLA motif additions** — identify and propose upstream additions to AXLA lexicon (axa_cli_flag, axi_mcp_tool, transport/dimension applicability to MCP context).

All work uses V3 data model and schema.

## Sequencing

Completes before ₣Ah resumes. ₣Ah is furloughed during this work.

## Heat constellation

| Heat | Silks | Role | Status |
|------|-------|------|--------|
| ₣Aw | jjk-v4-0-jjs0-axla-normalization | V3 infrastructure — annotations + JJF file exchange | Racing |
| ₣Ah | jjk-v4-1-school-breeze-founding | V4 schema transition (separate initiative) | Stabled (furloughed) |
| ₣An | jjk-v4-release-and-legacy-removal | V4 cleanup (separate initiative) | Stabled |
| ₣Am | jjk-v5-notional | Future parking lot | Stabled |