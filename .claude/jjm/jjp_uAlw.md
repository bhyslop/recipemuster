# Paddock: jjk-v4-0-jjs0-axla-normalization

## Purpose

Bring JJS0 to the latest AXLA voicing styles before V4 design work begins in ₣Ah. This is a **form** change — how the spec is captured — not a specification change. The goal is clean annotation vocabulary so V4 design paces in ₣Ah annotate new sections correctly from birth.

## Context

RBS0 levelled up to rich `//ax` annotation vocabulary:
- `//axvo_method axd_transient axd_grouped` for operations
- `//axvr_regime axf_bash axrd_file_sourced` for regime definitions
- `//axpof_fact` / `//axpot_tally` for operation outputs
- `//axhr*` hierarchy markers

JJS0 is still on first-generation patterns:
- `//axl_voices axi_cli_subcommand` for ALL operations (semantically wrong — they're MCP tools)
- 89 annotations total vs RBS0's 283
- Zero `axvo_`, `axvr_`, `axpo_`, `axhr_` annotations (except 4 routines)

## Known AXLA gaps to resolve

1. `axa_cli_flag` — used in JJS0, undefined in AXLA
2. `axi_mcp_tool` — needed for MCP tool operations
3. Transport dimension on operations
4. Operation dimension vocabulary applicability to MCP context
5. Category-level voicing pattern

## Sequencing

This heat completes before ₣Ah resumes. ₣Ah is furloughed during this work.

## Heat constellation update

| Heat | Silks | Role | Status |
|------|-------|------|--------|
| ₣Aw | jjk-v4-0-jjs0-axla-normalization | Spec form normalization — annotation vocabulary upgrade | Racing |
| ₣Ah | jjk-v4-1-school-breeze-founding | Development — schema transition, breaking changes | Stabled (furloughed for ₣Aw) |
| ₣An | jjk-v4-release-and-legacy-removal | Cleanup — upgrade installs, remove V3 compat | Stabled |
| ₣Am | jjk-v5-notional | Parking lot — post-V4 ideas | Stabled |