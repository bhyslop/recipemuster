# Heat Trophy: jjk-v3-recast-mcp

**Firemark:** ₣Ao
**Created:** 260306
**Retired:** 260306
**Status:** retired

## Paddock

# Paddock: jjk-v3-recast-mcp

## Mission

Promote all vvx commands (jjx_* and vvx_*) to MCP stdio server transport. Claude Code accesses vvx exclusively via MCP. The CLI path for jjx_* commands is removed — MCP tools are NOT also available via command line.

Infrastructure commands (emplace, unlock) remain CLI-only.

## Why separate heat

MCP transport is a foundation change benefiting all kits (VVK, JJK, future kits), not specific to V4 schema evolution. JJK is under active concurrent load across multiple officium right now — permission friction from heredoc-based bash invocation is a daily pain point.

## Design decisions

- **MCP-only for jjx_***: Claude Code calls MCP tools via JSON-RPC. The `vvx jjx` CLI subcommand group is removed. No dual invocation paths. No backwards compatibility.
- **`vvx mcp` subcommand**: Top-level peer to `vvx jjx` (which it replaces). Starts a long-lived stdio MCP server per Claude session.
- **Stateless per tool call**: Each MCP tool call does full lock → load → transform → save → unlock. No in-memory state between calls. Critical for multi-session correctness.
- **rmcp crate**: Official Rust MCP SDK, version pinned exactly. MCP spec version 2025-11-25.
- **Separate type definitions**: MCP tool params defined via rmcp `#[tool]` macro. No shared structs with clap. No crossing the streams.
- **Stdin eliminated**: The 7 stdin-reading commands gain structured string parameters. No stdin path exists.
- **Registration via kit install**: vvx_emplace adds mcpServers config to .claude/settings.local.json.

## Vocabulary

- **CLI subcommand**: shell invocation (`vvx jjx create`). Being removed for jjx_*.
- **MCP tool**: JSON-RPC tool call within an MCP session (`jjx_create` with structured params). The sole invocation path for Claude.
- **Handler**: shared Rust function implementing the operation. Transport-agnostic.

## Concurrency discipline

Multiple Claude sessions = multiple long-lived MCP server processes, one per session. The git-ref locking model (`vvg_lock_acquire`) already serializes concurrent access. This works because:
- Locks are per-tool-call, not per-session
- No in-memory state survives between tool calls
- Lock orphaning risk is identical to previous model (git-ref locks survive process death; `vvx_unlock` is the recovery path)

## CLAUDE.md impact

After MCP:
- Verb table maps to MCP tool names, not bash invocations
- Heredoc guidance section removed (structured params replace stdin)
- CLI Command Reference replaced with MCP tool reference
- All semantic content preserved (protocols, discipline rules)

## V4 ripple

On completing this heat, groom ₣Ah (jjk-v4-1-school-breeze-founding) to incorporate MCP transport assumptions into remaining paces.

## References

- ₢AoAAA docket: MCP evaluation with GO decision
- `Tools/vok/src/vorm_main.rs`: vvx entry point (`dispatch_external` seam)
- `Tools/jjk/vov_veiled/src/jjrx_cli.rs`: jjx command enum and dispatch
- rmcp: https://github.com/modelcontextprotocol/rust-sdk
- MCP spec 2025-11-25: https://modelcontextprotocol.io/specification/2025-11-25

## Paces

### consider-mcp-transport-for-jjx (₢AoAAA) [complete]

**[260306-1021] complete**

Evaluate MCP (Model Context Protocol) as transport for vvx commands. **Decision: GO.**

## Context

Currently Claude invokes jjx/vvx via Bash tool calls with heredoc stdin pipes. This causes permission friction — Claude Code's Bash permission matcher sees each unique heredoc as a new command string. The bash layer is purely transport (args + stdin); Rust already owns locking, JSON transformation, and git operations.

## Key Insight

MCP stdio transport replaces bash as the invocation layer with structured JSON-RPC tool calls. The Rust core logic stays untouched — only the entry point changes.

Current:  Claude → Bash(args + stdin pipe) → Rust(lock + transform + git)
MCP:      Claude → MCP tool call(structured params) → Rust(lock + transform + git)

## Decided (from evaluation + chat)

1. **Scope**: vvx gains `--mcp` mode (or similar) for MCP stdio server. JSON-RPC dispatch loop, tool registration from existing clap Args, parameter mapping. All jjx_* and vvx_* commands registered as MCP tools.
2. **Registration**: `.claude/settings.local.json` mcpServers config, added by vvx_emplace during kit install.
3. **Multi-officium**: Multiple sessions = multiple MCP server processes. Git-ref locking serializes access. Each tool call is stateless (full lock/unlock cycle). Proven safe — same locking model as CLI, no cross-call state.
4. **Stdin commands**: The 7 stdin-reading commands become structured string parameters. Eliminates heredoc escaping complexity entirely.
5. **CLI preserved**: CLI entry point stays for hooks, tabtargets, manual use. Claude uses MCP exclusively. Two front doors, same handlers.
6. **JJS0 stale section**: Crash-Safe Architecture section (lines 248-265) still describes bash-owned locking. Update as part of this work.
7. **No incremental path**: Go straight to MCP. No file-based input stopgap.

## Acceptance

This pace is the evaluation. It produced the GO recommendation. Ready to wrap — implementation paces follow.

**[260306-0951] rough**

Evaluate MCP (Model Context Protocol) as transport for vvx commands. **Decision: GO.**

## Context

Currently Claude invokes jjx/vvx via Bash tool calls with heredoc stdin pipes. This causes permission friction — Claude Code's Bash permission matcher sees each unique heredoc as a new command string. The bash layer is purely transport (args + stdin); Rust already owns locking, JSON transformation, and git operations.

## Key Insight

MCP stdio transport replaces bash as the invocation layer with structured JSON-RPC tool calls. The Rust core logic stays untouched — only the entry point changes.

Current:  Claude → Bash(args + stdin pipe) → Rust(lock + transform + git)
MCP:      Claude → MCP tool call(structured params) → Rust(lock + transform + git)

## Decided (from evaluation + chat)

1. **Scope**: vvx gains `--mcp` mode (or similar) for MCP stdio server. JSON-RPC dispatch loop, tool registration from existing clap Args, parameter mapping. All jjx_* and vvx_* commands registered as MCP tools.
2. **Registration**: `.claude/settings.local.json` mcpServers config, added by vvx_emplace during kit install.
3. **Multi-officium**: Multiple sessions = multiple MCP server processes. Git-ref locking serializes access. Each tool call is stateless (full lock/unlock cycle). Proven safe — same locking model as CLI, no cross-call state.
4. **Stdin commands**: The 7 stdin-reading commands become structured string parameters. Eliminates heredoc escaping complexity entirely.
5. **CLI preserved**: CLI entry point stays for hooks, tabtargets, manual use. Claude uses MCP exclusively. Two front doors, same handlers.
6. **JJS0 stale section**: Crash-Safe Architecture section (lines 248-265) still describes bash-owned locking. Update as part of this work.
7. **No incremental path**: Go straight to MCP. No file-based input stopgap.

## Acceptance

This pace is the evaluation. It produced the GO recommendation. Ready to wrap — implementation paces follow.

**[260306-0938] rough**

Drafted from ₢AhAAH in ₣Ah.

Evaluate MCP (Model Context Protocol) as a transport replacement for the current bash-stdin invocation pattern used by jjx commands.

## Context

Currently Claude invokes jjx via Bash tool calls with heredoc stdin pipes. This causes permission friction — Claude Code's Bash permission matcher sees each unique heredoc as a new command string. The current bash layer is purely transport (args + stdin); Rust already owns locking, JSON transformation, and git operations (see persist/wrap routines in JJS0).

## Key Insight

MCP stdio transport would replace bash as the invocation layer with structured JSON-RPC tool calls. The Rust core logic stays untouched — only the entry point changes.

Current:  Claude → Bash(args + stdin pipe) → Rust(lock + transform + git)
MCP:      Claude → MCP tool call(structured params) → Rust(lock + transform + git)

## Questions to Resolve

1. **Scope**: What exactly needs to change in vvx to serve as MCP stdio server? JSON-RPC dispatch loop, tool registration, parameter mapping.
2. **Registration**: How does Claude Code discover and spawn the MCP server? `.claude/settings.json` mcpServers config.
3. **Multi-officium**: Multiple Claude sessions = multiple MCP server instances. Locking already handles this (vvg_lock_acquire), but verify no conflicts.
4. **Stdin commands**: The 7 commands taking stdin (enroll, revise_docket, arm, transfer, close, paddock, landing) become structured string parameters. Any edge cases?
5. **Backward compatibility**: Can vvx serve both CLI and MCP modes? (e.g., `vvx --mcp` for stdio server mode, default for CLI)
6. **JJS0 stale section**: The Crash-Safe Architecture section (lines 248-265) still describes bash-owned locking. Update regardless of MCP decision.
7. **Incremental path**: Could we do file-based input (--input-file) as a quick fix AND MCP as the proper solution?

## Acceptance

A design document or paddock update with clear go/no-go recommendation and implementation sketch if go.

**[260306-0922] rough**

Evaluate MCP (Model Context Protocol) as a transport replacement for the current bash-stdin invocation pattern used by jjx commands.

## Context

Currently Claude invokes jjx via Bash tool calls with heredoc stdin pipes. This causes permission friction — Claude Code's Bash permission matcher sees each unique heredoc as a new command string. The current bash layer is purely transport (args + stdin); Rust already owns locking, JSON transformation, and git operations (see persist/wrap routines in JJS0).

## Key Insight

MCP stdio transport would replace bash as the invocation layer with structured JSON-RPC tool calls. The Rust core logic stays untouched — only the entry point changes.

Current:  Claude → Bash(args + stdin pipe) → Rust(lock + transform + git)
MCP:      Claude → MCP tool call(structured params) → Rust(lock + transform + git)

## Questions to Resolve

1. **Scope**: What exactly needs to change in vvx to serve as MCP stdio server? JSON-RPC dispatch loop, tool registration, parameter mapping.
2. **Registration**: How does Claude Code discover and spawn the MCP server? `.claude/settings.json` mcpServers config.
3. **Multi-officium**: Multiple Claude sessions = multiple MCP server instances. Locking already handles this (vvg_lock_acquire), but verify no conflicts.
4. **Stdin commands**: The 7 commands taking stdin (enroll, revise_docket, arm, transfer, close, paddock, landing) become structured string parameters. Any edge cases?
5. **Backward compatibility**: Can vvx serve both CLI and MCP modes? (e.g., `vvx --mcp` for stdio server mode, default for CLI)
6. **JJS0 stale section**: The Crash-Safe Architecture section (lines 248-265) still describes bash-owned locking. Update regardless of MCP decision.
7. **Incremental path**: Could we do file-based input (--input-file) as a quick fix AND MCP as the proper solution?

## Acceptance

A design document or paddock update with clear go/no-go recommendation and implementation sketch if go.

### jjs0-mcp-transport-spec (₢AoAAB) [complete]

**[260306-1037] complete**

Update JJS0-GallopsData.adoc to specify MCP as the transport layer for Claude, replacing the CLI path.

## Vocabulary to establish

JJS0 must define three terms used throughout all subsequent paces and documentation:

- **CLI subcommand**: a shell invocation (`vvx jjx create --silks "my-heat"`). Being removed for jjx_* commands.
- **MCP tool**: a JSON-RPC tool call within an MCP session (`jjx_create` with structured params). The sole invocation path for Claude.
- **Handler**: the shared Rust function that implements the operation. Transport-agnostic.

The `vvx mcp` subcommand starts a stdio MCP server. Within that server, all jjx_* and vvx_* operations are registered as MCP tools. The CLI subcommand path (`vvx jjx *`) is removed — MCP tools are NOT also available via CLI.

Infrastructure commands (emplace, unlock) remain CLI-only.

## Sections to update

1. **New section: MCP Transport** — Add under Design Principles. Describes `vvx mcp` as stdio JSON-RPC server, stateless per-call model (lock→load→transform→save→unlock), rmcp crate as implementation, MCP spec version 2025-11-25. Establishes vocabulary above.

2. **Crash-Safe Architecture** (lines 248-265) — Replace bash-only locking description. Remove bash code example. Describe transport-agnostic invariant: every operation performs lock→load→transform→save→unlock. MCP server and (historical) CLI both implement this invariant. No in-memory state between calls.

3. **Operations preamble** (lines 1143-1153) — Remove "Bash handles locking" framing. Operations are MCP tools invoked via JSON-RPC. Handlers are transport-agnostic Rust functions.

4. **CLI section** (lines 1089-1105) — Revise to describe `vvx mcp` as the primary entry point. Note that jjx_* CLI subcommands are removed; MCP tools replace them. Infrastructure CLI commands remain.

5. **Stdin-reading operations** — The 7 commands (enroll, revise_docket, arm, transfer, close, paddock, landing) take structured string parameters. No stdin path — MCP is the only invocation. Document the parameter names for each.

6. **{jjda_file} argument** (lines 895-906) — Remove from per-operation argument lists. MCP server resolves the gallops path internally. Not exposed as a tool parameter.

7. **Upper API section** — Small touch: acknowledge that the LLM invokes MCP tools directly rather than mapping verbs to CLI invocations. The verb table in CLAUDE.md still bridges vocabulary, but the invocation mechanism is MCP, not bash.

## Not changing
- Types, Records, Serialization, Commit Message Architecture — all transport-agnostic.

## Acceptance
- JJS0 establishes CLI subcommand / MCP tool / handler vocabulary
- New MCP Transport section under Design Principles
- Crash-Safe Architecture reflects stateless per-call MCP model
- jjx_* CLI path documented as removed
- Stdin→structured params documented for 7 affected operations
- {jjda_file} removed from operation argument lists

**[260306-1019] rough**

Update JJS0-GallopsData.adoc to specify MCP as the transport layer for Claude, replacing the CLI path.

## Vocabulary to establish

JJS0 must define three terms used throughout all subsequent paces and documentation:

- **CLI subcommand**: a shell invocation (`vvx jjx create --silks "my-heat"`). Being removed for jjx_* commands.
- **MCP tool**: a JSON-RPC tool call within an MCP session (`jjx_create` with structured params). The sole invocation path for Claude.
- **Handler**: the shared Rust function that implements the operation. Transport-agnostic.

The `vvx mcp` subcommand starts a stdio MCP server. Within that server, all jjx_* and vvx_* operations are registered as MCP tools. The CLI subcommand path (`vvx jjx *`) is removed — MCP tools are NOT also available via CLI.

Infrastructure commands (emplace, unlock) remain CLI-only.

## Sections to update

1. **New section: MCP Transport** — Add under Design Principles. Describes `vvx mcp` as stdio JSON-RPC server, stateless per-call model (lock→load→transform→save→unlock), rmcp crate as implementation, MCP spec version 2025-11-25. Establishes vocabulary above.

2. **Crash-Safe Architecture** (lines 248-265) — Replace bash-only locking description. Remove bash code example. Describe transport-agnostic invariant: every operation performs lock→load→transform→save→unlock. MCP server and (historical) CLI both implement this invariant. No in-memory state between calls.

3. **Operations preamble** (lines 1143-1153) — Remove "Bash handles locking" framing. Operations are MCP tools invoked via JSON-RPC. Handlers are transport-agnostic Rust functions.

4. **CLI section** (lines 1089-1105) — Revise to describe `vvx mcp` as the primary entry point. Note that jjx_* CLI subcommands are removed; MCP tools replace them. Infrastructure CLI commands remain.

5. **Stdin-reading operations** — The 7 commands (enroll, revise_docket, arm, transfer, close, paddock, landing) take structured string parameters. No stdin path — MCP is the only invocation. Document the parameter names for each.

6. **{jjda_file} argument** (lines 895-906) — Remove from per-operation argument lists. MCP server resolves the gallops path internally. Not exposed as a tool parameter.

7. **Upper API section** — Small touch: acknowledge that the LLM invokes MCP tools directly rather than mapping verbs to CLI invocations. The verb table in CLAUDE.md still bridges vocabulary, but the invocation mechanism is MCP, not bash.

## Not changing
- Types, Records, Serialization, Commit Message Architecture — all transport-agnostic.

## Acceptance
- JJS0 establishes CLI subcommand / MCP tool / handler vocabulary
- New MCP Transport section under Design Principles
- Crash-Safe Architecture reflects stateless per-call MCP model
- jjx_* CLI path documented as removed
- Stdin→structured params documented for 7 affected operations
- {jjda_file} removed from operation argument lists

**[260306-1002] rough**

Update JJS0-GallopsData.adoc to specify MCP as a transport layer alongside CLI.

## Sections to update

1. **Crash-Safe Architecture** (lines 248-265) — Replace bash-only locking description with dual-transport model. MCP server performs lock→read→modify→write→unlock per tool call (stateless). CLI path unchanged. Remove bash code example showing lock_acquire/lock_release pattern; replace with transport-agnostic description.

2. **Operations preamble** (lines 1143-1153) — Revise "Bash handles locking" to describe two transports: MCP (Claude's exclusive path, structured JSON-RPC params) and CLI (hooks, tabtargets, manual use, same binary). Both share the same Rust handlers.

3. **CLI section** (lines 1089-1105) — Add MCP transport description alongside existing CLI description. vvx gains `--mcp` mode for stdio JSON-RPC server. Tool registration derives from existing clap Args structs.

4. **Stdin-reading operations** — Document that the 7 stdin commands (enroll, revise_docket, arm, transfer, close, paddock, landing) accept structured string parameters under MCP transport while retaining stdin for CLI. This is a transport concern, not a schema change.

5. **{jjda_file} argument** (lines 895-906) — Note that MCP transport defaults this internally; not exposed as a tool parameter. CLI retains the flag.

## Not changing
- Types, Records, Serialization, Upper API verbs, Commit Message Architecture — all transport-agnostic.

## Acceptance
- JJS0 accurately describes both MCP and CLI transports
- Crash-Safe Architecture reflects stateless per-call MCP model
- Stdin→params documented for affected operations

### jjx-stdin-to-params (₢AoAAD) [complete]

**[260306-1052] complete**

Refactor the 7 stdin-reading jjx handlers to accept text as function parameters.

## Context

With jjx_* commands moving to MCP-only (no CLI path), these handlers no longer need to read from process stdin. Each gains a string parameter passed by the MCP tool definition.

## Handlers affected

1. jjx_enroll — docket text
2. jjx_revise_docket — new docket text
3. jjx_close — wrap summary
4. jjx_arm — warrant text
5. jjx_transfer — JSON array of coronets (string)
6. jjx_paddock — paddock content (when setting)
7. jjx_landing — agent completion report

## Approach

- Each handler function signature changes: stdin read replaced with a String parameter
- The MCP tool definition (from ₢AoAAC) passes the structured string parameter to the handler
- Remove all stdin-reading code from these handlers
- No dual-path logic — stdin path is gone
- Handler refactoring is testable with unit tests before MCP wiring exists

## Naming

MCP tool parameter names for the text fields. Use presentation vocabulary that Claude already knows:
- `docket` for enroll and revise_docket
- `summary` for close
- `warrant` for arm
- `coronets` for transfer (JSON array as string, per JJS0 spec)
- `content` for paddock and landing

## Acceptance
- All 7 handlers accept text via function parameter, not stdin
- No stdin-reading code remains in these handlers
- MCP tool definitions use the named parameters above

**[260306-1037] rough**

Refactor the 7 stdin-reading jjx handlers to accept text as function parameters.

## Context

With jjx_* commands moving to MCP-only (no CLI path), these handlers no longer need to read from process stdin. Each gains a string parameter passed by the MCP tool definition.

## Handlers affected

1. jjx_enroll — docket text
2. jjx_revise_docket — new docket text
3. jjx_close — wrap summary
4. jjx_arm — warrant text
5. jjx_transfer — JSON array of coronets (string)
6. jjx_paddock — paddock content (when setting)
7. jjx_landing — agent completion report

## Approach

- Each handler function signature changes: stdin read replaced with a String parameter
- The MCP tool definition (from ₢AoAAC) passes the structured string parameter to the handler
- Remove all stdin-reading code from these handlers
- No dual-path logic — stdin path is gone
- Handler refactoring is testable with unit tests before MCP wiring exists

## Naming

MCP tool parameter names for the text fields. Use presentation vocabulary that Claude already knows:
- `docket` for enroll and revise_docket
- `summary` for close
- `warrant` for arm
- `coronets` for transfer (JSON array as string, per JJS0 spec)
- `content` for paddock and landing

## Acceptance
- All 7 handlers accept text via function parameter, not stdin
- No stdin-reading code remains in these handlers
- MCP tool definitions use the named parameters above

**[260306-1019] rough**

Refactor the 7 stdin-reading jjx handlers to accept text as function parameters.

## Context

With jjx_* commands moving to MCP-only (no CLI path), these handlers no longer need to read from process stdin. Each gains a string parameter passed by the MCP tool definition.

## Handlers affected

1. jjx_enroll — docket text
2. jjx_revise_docket — new docket text
3. jjx_close — wrap summary
4. jjx_arm — warrant text
5. jjx_transfer — coronet list (newline-separated string)
6. jjx_paddock — paddock content (when setting)
7. jjx_landing — agent completion report

## Approach

- Each handler function signature changes: stdin read replaced with a String parameter
- The MCP tool definition (from ₢AoAAC) passes the structured string parameter to the handler
- Remove all stdin-reading code from these handlers
- No dual-path logic, no "param present → use it; absent → read stdin" — stdin path is gone

## Naming

MCP tool parameter names for the text fields. Use presentation vocabulary that Claude already knows:
- `docket` for enroll and revise_docket
- `summary` for close
- `warrant` for arm
- `coronets` for transfer
- `content` for paddock and landing

## Acceptance
- All 7 handlers accept text via function parameter, not stdin
- No stdin-reading code remains in these handlers
- MCP tool definitions use the named parameters above

**[260306-1002] rough**

Convert the 7 stdin-reading jjx commands to accept structured string parameters under MCP transport.

## Commands affected
1. jjx_enroll — docket text
2. jjx_revise_docket — new docket text
3. jjx_arm — warrant text
4. jjx_transfer — coronet list
5. jjx_close — wrap summary
6. jjx_paddock — paddock content (when setting)
7. jjx_landing — agent completion report

## Approach

- Each command gains an optional string parameter (e.g., `--docket-text`, `--content`, or similar)
- When invoked via MCP, the structured parameter is used
- When invoked via CLI, stdin is read as before (parameter not provided)
- The command handler unifies both paths: parameter present → use it; absent → read stdin
- No behavioral changes — same validation, same processing

## Acceptance
- All 7 commands work via MCP with structured string params
- All 7 commands still work via CLI with stdin pipes
- No regressions in existing CLI workflows

### vvx-mcp-stdio-server (₢AoAAC) [complete]

**[260306-1240] complete**

Implement MCP stdio server mode in vvx.

## Design

`vvx mcp` is a top-level subcommand (peer to `vvx jjx`). It starts a long-lived stdio JSON-RPC server implementing MCP spec version 2025-11-25. Within the server session, all jjx_* and vvx_* operations are registered as MCP tools with typed schemas.

The jjx_* CLI subcommand path (`vvx jjx create`, etc.) is removed. MCP tools are the sole invocation path for these operations. Infrastructure CLI commands (emplace, unlock) remain as clap subcommands.

## Implementation

- **Crate**: `rmcp = { version = "=0.16.0", features = ["server"] }` (official MCP Rust SDK). Pin exact version.
- **Entry point**: `vvx mcp` subcommand added via clap. Enters `rmcp` stdio server loop.
- **Tool definitions**: Each jjx_* and vvx_* operation gets an rmcp `#[tool]` definition with typed parameters and descriptions. These are independent from the (now-removed) clap subcommand definitions — no shared structs between clap and rmcp.
- **Dispatch**: MCP tool handler calls the same Rust handler functions that CLI subcommands called. Handlers are transport-agnostic.
- **Stateless**: Each tool call performs full lock→load→transform→save→unlock. No in-memory gallops state between calls.
- **Gallops path**: MCP server resolves `.claude/jjm/jjg_gallops.json` internally. Not exposed as a tool parameter.
- **Errors**: Map handler errors to JSON-RPC error responses.

## CLI removal

- Remove `vvx jjx` subcommand group from clap dispatch
- Remove tabtargets that invoked jjx_* via CLI (e.g., `tt/vvw-r.RunVVX.sh jjx_*` patterns)
- Remove jjw_workbench.sh dispatch for jjx commands

## Dependencies
- JJS0 spec update (₢AoAAB) should be done first to guide implementation

## Acceptance
- `vvx mcp` starts stdio MCP server
- All jjx_* operations callable as MCP tools with typed parameters
- `vvx jjx` CLI subcommand path removed
- Tool schemas include parameter types and descriptions
- Stateless per-call (no cached gallops between calls)
- All dependency crate versions pinned exactly

**[260306-1019] rough**

Implement MCP stdio server mode in vvx.

## Design

`vvx mcp` is a top-level subcommand (peer to `vvx jjx`). It starts a long-lived stdio JSON-RPC server implementing MCP spec version 2025-11-25. Within the server session, all jjx_* and vvx_* operations are registered as MCP tools with typed schemas.

The jjx_* CLI subcommand path (`vvx jjx create`, etc.) is removed. MCP tools are the sole invocation path for these operations. Infrastructure CLI commands (emplace, unlock) remain as clap subcommands.

## Implementation

- **Crate**: `rmcp = { version = "=0.16.0", features = ["server"] }` (official MCP Rust SDK). Pin exact version.
- **Entry point**: `vvx mcp` subcommand added via clap. Enters `rmcp` stdio server loop.
- **Tool definitions**: Each jjx_* and vvx_* operation gets an rmcp `#[tool]` definition with typed parameters and descriptions. These are independent from the (now-removed) clap subcommand definitions — no shared structs between clap and rmcp.
- **Dispatch**: MCP tool handler calls the same Rust handler functions that CLI subcommands called. Handlers are transport-agnostic.
- **Stateless**: Each tool call performs full lock→load→transform→save→unlock. No in-memory gallops state between calls.
- **Gallops path**: MCP server resolves `.claude/jjm/jjg_gallops.json` internally. Not exposed as a tool parameter.
- **Errors**: Map handler errors to JSON-RPC error responses.

## CLI removal

- Remove `vvx jjx` subcommand group from clap dispatch
- Remove tabtargets that invoked jjx_* via CLI (e.g., `tt/vvw-r.RunVVX.sh jjx_*` patterns)
- Remove jjw_workbench.sh dispatch for jjx commands

## Dependencies
- JJS0 spec update (₢AoAAB) should be done first to guide implementation

## Acceptance
- `vvx mcp` starts stdio MCP server
- All jjx_* operations callable as MCP tools with typed parameters
- `vvx jjx` CLI subcommand path removed
- Tool schemas include parameter types and descriptions
- Stateless per-call (no cached gallops between calls)
- All dependency crate versions pinned exactly

**[260306-1002] rough**

Implement MCP stdio server mode in vvx.

## Scope

- Add `--mcp` flag (or subcommand) to vvx that enters JSON-RPC 2.0 stdio server loop
- Register all jjx_* and vvx_* commands as MCP tools with typed schemas
- Derive tool schemas from existing clap Args structs (tool name, parameter types, descriptions)
- JSON-RPC dispatch loop: read request from stdin, route to existing command handler, write response to stdout
- Each tool call is stateless: full lock→read→modify→write→unlock cycle per invocation
- Error responses use JSON-RPC error codes

## Key design points

- Reuse existing `dispatch_external` seam in `vorm_main.rs`
- Existing command handlers return structured data; MCP wraps in JSON-RPC response envelope
- The 7 stdin-reading commands gain string parameters instead of reading process stdin
- No in-memory state between tool calls — critical for long-lived server process correctness

## Dependencies
- JJS0 spec update (₢AoAAB) should be done first to guide implementation

## Acceptance
- `vvx --mcp` starts stdio server
- All jjx_* and vvx_* commands callable as MCP tools
- Tool schemas include parameter types and descriptions
- Stateless per-call (no cached gallops between calls)

### vvx-emplace-mcp-config (₢AoAAE) [complete]

**[260306-1240] complete**

Update vvx_emplace (kit install) to register MCP server in Claude Code settings.

## Scope

- vvx_emplace writes mcpServers entry to `.claude/settings.local.json`
- Handle existing settings file (merge, don't overwrite other keys)
- vvx_uninstall removes the mcpServers entry

## Binary path resolution

- **Installed kit**: binary at path from `.vvk/vvbf_brand.json`
- **Kit Forge (dev)**: binary at `Tools/vok/target/debug/vvx`
- Emplace must detect which mode and use the correct path
- Verify: does Claude Code accept relative paths in mcpServers, or must they be absolute?

## Config shape (verify against Claude Code's actual schema)

```json
{
  "mcpServers": {
    "vvx": {
      "command": "<resolved-binary-path>",
      "args": ["mcp"]
    }
  }
}
```

Note: `args` is `["mcp"]` (subcommand), not `["--mcp"]` (flag).

## Server name

Use `"vvx"` as the server name. settings.local.json is per-project, so no cross-project collision.

## Acceptance
- After `vvx_emplace`, Claude Code sees vvx MCP tools natively
- After uninstall, the mcpServers entry is removed
- Existing settings.local.json content preserved
- Correct binary path for both installed and Kit Forge modes

**[260306-1038] rough**

Update vvx_emplace (kit install) to register MCP server in Claude Code settings.

## Scope

- vvx_emplace writes mcpServers entry to `.claude/settings.local.json`
- Handle existing settings file (merge, don't overwrite other keys)
- vvx_uninstall removes the mcpServers entry

## Binary path resolution

- **Installed kit**: binary at path from `.vvk/vvbf_brand.json`
- **Kit Forge (dev)**: binary at `Tools/vok/target/debug/vvx`
- Emplace must detect which mode and use the correct path
- Verify: does Claude Code accept relative paths in mcpServers, or must they be absolute?

## Config shape (verify against Claude Code's actual schema)

```json
{
  "mcpServers": {
    "vvx": {
      "command": "<resolved-binary-path>",
      "args": ["mcp"]
    }
  }
}
```

Note: `args` is `["mcp"]` (subcommand), not `["--mcp"]` (flag).

## Server name

Use `"vvx"` as the server name. settings.local.json is per-project, so no cross-project collision.

## Acceptance
- After `vvx_emplace`, Claude Code sees vvx MCP tools natively
- After uninstall, the mcpServers entry is removed
- Existing settings.local.json content preserved
- Correct binary path for both installed and Kit Forge modes

**[260306-1019] rough**

Update vvx_emplace (kit install) to register MCP server in Claude Code settings.

## Scope

- vvx_emplace writes mcpServers entry to `.claude/settings.local.json`
- Handle existing settings file (merge, don't overwrite other keys)
- vvx_uninstall removes the mcpServers entry

## Binary path resolution

- **Installed kit**: binary at path from `.vvk/vvbf_brand.json`
- **Kit Forge (dev)**: binary at `Tools/vok/target/debug/vvx`
- Emplace must detect which mode and use the correct path
- Verify: does Claude Code accept relative paths in mcpServers, or must they be absolute?

## Config shape (verify against Claude Code's actual schema)

```json
{
  "mcpServers": {
    "vvx": {
      "command": "<resolved-binary-path>",
      "args": ["mcp"]
    }
  }
}
```

Note: `args` is `["mcp"]` (subcommand), not `["--mcp"]` (flag).

## Server name

Use `"vvx"` as the server name. settings.local.json is per-project, so no cross-project collision.

## Cleanup

- Remove jjx_* tabtargets from `tt/` that used the CLI path
- Remove or update jjw_workbench.sh references to jjx CLI dispatch

## Acceptance
- After `vvx_emplace`, Claude Code sees vvx MCP tools natively
- After uninstall, the mcpServers entry is removed
- Existing settings.local.json content preserved
- Correct binary path for both installed and Kit Forge modes

**[260306-1002] rough**

Update vvx_emplace (kit install) to register MCP server in Claude Code settings.

## Scope

- vvx_emplace writes mcpServers entry to `.claude/settings.local.json`
- Entry specifies: command path to vvx binary, `--mcp` flag, working directory
- Handle existing settings file (merge, don't overwrite)
- vvx_uninstall removes the mcpServers entry

## Config shape (approximate)
```json
{
  "mcpServers": {
    "vvx": {
      "command": "./Tools/vok/target/debug/vvx",
      "args": ["--mcp"]
    }
  }
}
```

## Acceptance
- After `vvx_emplace`, Claude Code sees vvx MCP tools natively
- After uninstall, the mcpServers entry is removed
- Existing settings.local.json content preserved

### mcp-single-dispatcher-tool (₢AoAAI) [complete]

**[260306-1255] complete**

Replace 25 individual MCP tools with a single `jjx` dispatcher tool.

## Motivation

Each jjx command is currently a separate MCP tool. This means Claude Code must ToolSearch before first use of each command in a conversation, and the tool list is cluttered with 25 entries. Since CLAUDE.md already teaches the command vocabulary, per-tool schemas add friction without proportional value.

## Design

Single `#[tool]` function with parameters:
- `command`: String — the jjx command name (e.g., `list`, `record`, `show`)
- `params`: String — JSON-encoded parameters for the command

The dispatcher deserializes `params` into the appropriate per-command param struct and calls the existing handler. All handler functions and their Args types are unchanged.

## Scope

- Rewrite `jjrm_mcp.rs` to expose one tool (`jjx`) instead of 25
- Remove per-tool `#[tool]` functions
- Keep all `jjrm_*Params` structs (used for serde deserialization)
- Keep all handler dispatch logic
- Update `.mcp.json` or registration if tool name changes

## Acceptance

- `jjx` appears as single MCP tool in Claude Code
- All 25 commands work via the single dispatcher
- ToolSearch loads one tool instead of 25
- Build and tests pass

**[260306-1249] rough**

Replace 25 individual MCP tools with a single `jjx` dispatcher tool.

## Motivation

Each jjx command is currently a separate MCP tool. This means Claude Code must ToolSearch before first use of each command in a conversation, and the tool list is cluttered with 25 entries. Since CLAUDE.md already teaches the command vocabulary, per-tool schemas add friction without proportional value.

## Design

Single `#[tool]` function with parameters:
- `command`: String — the jjx command name (e.g., `list`, `record`, `show`)
- `params`: String — JSON-encoded parameters for the command

The dispatcher deserializes `params` into the appropriate per-command param struct and calls the existing handler. All handler functions and their Args types are unchanged.

## Scope

- Rewrite `jjrm_mcp.rs` to expose one tool (`jjx`) instead of 25
- Remove per-tool `#[tool]` functions
- Keep all `jjrm_*Params` structs (used for serde deserialization)
- Keep all handler dispatch logic
- Update `.mcp.json` or registration if tool name changes

## Acceptance

- `jjx` appears as single MCP tool in Claude Code
- All 25 commands work via the single dispatcher
- ToolSearch loads one tool instead of 25
- Build and tests pass

### remove-cli-dispatch-path (₢AoAAG) [complete]

**[260306-1251] complete**

Remove the CLI dispatch path for jjx_* commands from dispatch_external in vorm_main.rs.

## Context

CLI dispatch was temporarily restored during ₢AoAAC to avoid bootstrap deadlock (can't jjx_record without CLI while MCP isn't registered). Once ₢AoAAE registers MCP in settings and MCP tools are verified working from Claude Code, the CLI path is redundant.

## Scope

- Remove jjk delegation from dispatch_external in vorm_main.rs (the 4-line #[cfg(feature = "jjk")] block)
- Remove jjrx_dispatch and jjrx_is_jjk_command re-exports from jjk lib.rs (dead code after removal)
- Optionally remove jjrx_cli.rs entirely if nothing else references it
- Remove any tabtargets that invoke jjx_* via CLI (tt/vvw-r.RunVVX.sh jjx_* patterns)

## Gate

Do NOT execute this pace until MCP tools are confirmed working from Claude Code via ₢AoAAE registration. Verify with at least one MCP tool call (e.g., jjx_list) before removing CLI.

## Acceptance

- vvx jjx_* returns "unknown command"
- All jjx operations still work via MCP
- No dead code warnings from removed dispatch path

**[260306-1137] rough**

Remove the CLI dispatch path for jjx_* commands from dispatch_external in vorm_main.rs.

## Context

CLI dispatch was temporarily restored during ₢AoAAC to avoid bootstrap deadlock (can't jjx_record without CLI while MCP isn't registered). Once ₢AoAAE registers MCP in settings and MCP tools are verified working from Claude Code, the CLI path is redundant.

## Scope

- Remove jjk delegation from dispatch_external in vorm_main.rs (the 4-line #[cfg(feature = "jjk")] block)
- Remove jjrx_dispatch and jjrx_is_jjk_command re-exports from jjk lib.rs (dead code after removal)
- Optionally remove jjrx_cli.rs entirely if nothing else references it
- Remove any tabtargets that invoke jjx_* via CLI (tt/vvw-r.RunVVX.sh jjx_* patterns)

## Gate

Do NOT execute this pace until MCP tools are confirmed working from Claude Code via ₢AoAAE registration. Verify with at least one MCP tool call (e.g., jjx_list) before removing CLI.

## Acceptance

- vvx jjx_* returns "unknown command"
- All jjx operations still work via MCP
- No dead code warnings from removed dispatch path

### claudemd-mcp-migration (₢AoAAF) [complete]

**[260306-1337] complete**

Update CLAUDE.md JJK managed section to reflect MCP-native usage via single `jjx` dispatcher tool.

## Context

With jjx commands served exclusively via the single `mcp__vvx__jjx` MCP tool (command + params dispatch), the CLAUDE.md JJK section needs rewriting. The single-tool design means Claude does NOT see per-command schemas — the CLAUDE.md reference is the PRIMARY source of per-command parameter knowledge.

## Changes

- **CLI Syntax section**: Remove entirely. Replace with MCP tool usage section explaining the single-tool dispatch pattern: `command` string selects the operation, `params` JSON object provides arguments.
- **Verb table**: Keep the table. Command column references MCP command names (e.g., slate → `enroll` command). Remove all `./tt/vvw-r.RunVVX.sh jjx_*` patterns.
- **CLI Command Reference**: Replace with MCP params reference. Document the `params` JSON shape for each command. Field names may differ from old CLI (e.g., `identity` for coronet/firemark, `content` for stdin text). Check `jjrm_mcp.rs` for exact field names.
- **Heredoc guidance**: Remove from JJK section. (Root CLAUDE.md heredoc section stays — still needed for non-JJK heredocs.)
- **Composition Recipes**: Remove `&&` chaining examples. Sequential MCP tool calls replace bash composition.
- **Commit Discipline**: Update examples from bash invocations to MCP tool call descriptions. Keep all semantic content (additive-only, explicit file list, multi-officium discipline).
- **Mount/Groom/Wrap protocols**: Update to reflect MCP tool calls. Preserve protocol logic.
- **Wrap Discipline**: Replace `echo "summary" | jjx_close CORONET` with MCP close command + summary param.
- **Forbidden Git Commands**: Unchanged — transport-agnostic.
- **Build & Run Discipline**: Unchanged — cargo/tabtarget commands, not jjx.
- **Diagnose Before Escalating**: Keep, adapt examples to MCP context.

## Implementation approach

1. Read `jjrm_mcp.rs` to inventory exact param field names for all commands
2. Rewrite the JJK managed section
3. Also update `Tools/jjk/vov_veiled/vocjjmc_core.md` (per the HTML comment above the managed section)

## Acceptance
- CLAUDE.md JJK section contains zero bash invocation patterns for jjx
- Single-tool dispatch pattern documented clearly
- Per-command params reference with exact JSON field names
- Heredoc guidance removed from JJK section
- All semantic content preserved (protocols, discipline rules, vocabulary)
- vocjjmc_core.md updated in sync

**[260306-1302] rough**

Update CLAUDE.md JJK managed section to reflect MCP-native usage via single `jjx` dispatcher tool.

## Context

With jjx commands served exclusively via the single `mcp__vvx__jjx` MCP tool (command + params dispatch), the CLAUDE.md JJK section needs rewriting. The single-tool design means Claude does NOT see per-command schemas — the CLAUDE.md reference is the PRIMARY source of per-command parameter knowledge.

## Changes

- **CLI Syntax section**: Remove entirely. Replace with MCP tool usage section explaining the single-tool dispatch pattern: `command` string selects the operation, `params` JSON object provides arguments.
- **Verb table**: Keep the table. Command column references MCP command names (e.g., slate → `enroll` command). Remove all `./tt/vvw-r.RunVVX.sh jjx_*` patterns.
- **CLI Command Reference**: Replace with MCP params reference. Document the `params` JSON shape for each command. Field names may differ from old CLI (e.g., `identity` for coronet/firemark, `content` for stdin text). Check `jjrm_mcp.rs` for exact field names.
- **Heredoc guidance**: Remove from JJK section. (Root CLAUDE.md heredoc section stays — still needed for non-JJK heredocs.)
- **Composition Recipes**: Remove `&&` chaining examples. Sequential MCP tool calls replace bash composition.
- **Commit Discipline**: Update examples from bash invocations to MCP tool call descriptions. Keep all semantic content (additive-only, explicit file list, multi-officium discipline).
- **Mount/Groom/Wrap protocols**: Update to reflect MCP tool calls. Preserve protocol logic.
- **Wrap Discipline**: Replace `echo "summary" | jjx_close CORONET` with MCP close command + summary param.
- **Forbidden Git Commands**: Unchanged — transport-agnostic.
- **Build & Run Discipline**: Unchanged — cargo/tabtarget commands, not jjx.
- **Diagnose Before Escalating**: Keep, adapt examples to MCP context.

## Implementation approach

1. Read `jjrm_mcp.rs` to inventory exact param field names for all commands
2. Rewrite the JJK managed section
3. Also update `Tools/jjk/vov_veiled/vocjjmc_core.md` (per the HTML comment above the managed section)

## Acceptance
- CLAUDE.md JJK section contains zero bash invocation patterns for jjx
- Single-tool dispatch pattern documented clearly
- Per-command params reference with exact JSON field names
- Heredoc guidance removed from JJK section
- All semantic content preserved (protocols, discipline rules, vocabulary)
- vocjjmc_core.md updated in sync

**[260306-1020] rough**

Update CLAUDE.md JJK managed section to reflect MCP-native usage.

## Context

With jjx_* commands served exclusively via MCP, the CLAUDE.md JJK section no longer needs bash invocation patterns, heredoc guidance, or CLI syntax documentation. Claude calls MCP tools directly with typed parameters.

## Changes

- **Verb table**: Keep the table but change the "Command" column. Verbs map to MCP tool names (e.g., slate → `jjx_enroll` MCP tool), not bash invocations. Remove `./tt/vvw-r.RunVVX.sh jjx_*` patterns entirely.
- **CLI Command Reference section**: Replace with MCP tool reference. Claude sees tool schemas natively, so this may reduce to just parameter documentation for the text-blob parameters (docket, warrant, summary, content, coronets) that aren't self-evident from the schema.
- **Heredoc guidance**: Remove entirely. Structured params eliminate delimiter conflicts.
- **Heredoc Delimiter Selection**: This is in the root CLAUDE.md, not JJK-managed. Leave it alone (still needed for non-JJK heredocs).
- **Commit Discipline**: Update — jjx_record invoked as MCP tool, not via bash. Remove bash invocation examples. Keep semantic content (additive-only, explicit file list, multi-officium discipline).
- **Mount/Groom/Wrap protocols**: Update to reflect MCP tool calls. Preserve protocol logic.
- **Forbidden Git Commands**: Unchanged — transport-agnostic.
- **Build & Run Discipline**: Unchanged — these are cargo/tabtarget commands, not jjx.

## Dependencies
- MCP server working (₢AoAAC)
- Emplace config done (₢AoAAE)
- Should be last pace — only update docs after transport is proven working

## Acceptance
- CLAUDE.md JJK section contains zero bash invocation patterns for jjx
- Heredoc guidance removed from JJK section
- Verb table maps to MCP tool names
- Claude can follow the updated section to use MCP tools directly
- All semantic content preserved (protocols, discipline rules, vocabulary)

**[260306-1003] rough**

Update CLAUDE.md JJK managed section to reflect MCP-native usage.

## Changes

- Remove bash invocation patterns (`./tt/vvw-r.RunVVX.sh jjx_*`) from verb table and examples
- Remove heredoc guidance section (structured params eliminate delimiter conflicts)
- CLI Command Reference: simplify to parameter documentation rather than shell syntax
- Update Commit Discipline section: jjx_record invoked via MCP, not bash
- Update Mount/Groom/Wrap protocols to reflect MCP tool calls
- Preserve all semantic content (verb mappings, protocols, discipline rules)

## Dependencies
- MCP server working (₢AoAAC)
- Emplace config done (₢AoAAE)
- Should be last implementation pace — only update docs after transport is proven

## Acceptance
- CLAUDE.md JJK section contains no bash invocation patterns for jjx
- Heredoc guidance removed
- Claude can follow the updated section to use MCP tools directly

### investigate-jjs0-crash-safe-staleness (₢AoAAH) [complete]

**[260306-1952] complete**

Investigate whether JJS0 Crash-Safe Architecture section (currently lines ~248-265) accurately reflects the implemented locking and git commit architecture after MCP transport changes.

## Context

As of cchat-20260306, the section says "Bash handles locking via `git update-ref`" and shows a bash lock_acquire/lock_release pattern. But the actual persist routine (JJSRPS) and wrap routine (JJSRWP) show Rust owns locking via `vvg_lock_acquire`, git staging (`git add`), and git commit — all inside the Rust binary.

After ₣Ao's MCP work completes, the bash layer may be further reduced or eliminated, making this section even more stale — or the MCP changes may have already addressed it.

## Action

Read the section, compare against implemented routines, and either:
- Correct if still stale
- Confirm if already fixed by prior paces in ₣Ao
- Note what remains inaccurate

Do NOT assume the fix is "just update the text" — the MCP transport changes may have restructured the architecture in ways that require rethinking the section entirely.

## Additional: Dead-code cleanup in vvcp_probe.rs

Remove the 9 dead-code items in `Tools/vvc/src/vvcp_probe.rs` — constants (VVCP_RAW_HAIKU_FILE, VVCP_ELEMENT_*), VVCP_BURD_TEMP_DIR_VAR, and functions (write_raw_output, extract_xml_element) plus their associated tests (test_extract_xml_element_*). These are leftovers from the old XML-based opus probe approach that the new raw-output probes no longer use. They produce 9 compiler warnings on every build.

**[260306-1947] rough**

Investigate whether JJS0 Crash-Safe Architecture section (currently lines ~248-265) accurately reflects the implemented locking and git commit architecture after MCP transport changes.

## Context

As of cchat-20260306, the section says "Bash handles locking via `git update-ref`" and shows a bash lock_acquire/lock_release pattern. But the actual persist routine (JJSRPS) and wrap routine (JJSRWP) show Rust owns locking via `vvg_lock_acquire`, git staging (`git add`), and git commit — all inside the Rust binary.

After ₣Ao's MCP work completes, the bash layer may be further reduced or eliminated, making this section even more stale — or the MCP changes may have already addressed it.

## Action

Read the section, compare against implemented routines, and either:
- Correct if still stale
- Confirm if already fixed by prior paces in ₣Ao
- Note what remains inaccurate

Do NOT assume the fix is "just update the text" — the MCP transport changes may have restructured the architecture in ways that require rethinking the section entirely.

## Additional: Dead-code cleanup in vvcp_probe.rs

Remove the 9 dead-code items in `Tools/vvc/src/vvcp_probe.rs` — constants (VVCP_RAW_HAIKU_FILE, VVCP_ELEMENT_*), VVCP_BURD_TEMP_DIR_VAR, and functions (write_raw_output, extract_xml_element) plus their associated tests (test_extract_xml_element_*). These are leftovers from the old XML-based opus probe approach that the new raw-output probes no longer use. They produce 9 compiler warnings on every build.

**[260306-1153] rough**

Investigate whether JJS0 Crash-Safe Architecture section (currently lines ~248-265) accurately reflects the implemented locking and git commit architecture after MCP transport changes.

## Context

As of cchat-20260306, the section says "Bash handles locking via `git update-ref`" and shows a bash lock_acquire/lock_release pattern. But the actual persist routine (JJSRPS) and wrap routine (JJSRWP) show Rust owns locking via `vvg_lock_acquire`, git staging (`git add`), and git commit — all inside the Rust binary.

After ₣Ao's MCP work completes, the bash layer may be further reduced or eliminated, making this section even more stale — or the MCP changes may have already addressed it.

## Action

Read the section, compare against implemented routines, and either:
- Correct if still stale
- Confirm if already fixed by prior paces in ₣Ao
- Note what remains inaccurate

Do NOT assume the fix is "just update the text" — the MCP transport changes may have restructured the architecture in ways that require rethinking the section entirely.

### fix-mcp-subprocess-stdin-inheritance (₢AoAAJ) [complete]

**[260306-1953] complete**

Fix MCP transport deadlock caused by child processes inheriting the MCP server's stdin fd.

Root cause: Command::new("git") and Command::new("claude") spawn subprocesses that inherit stdin from the parent MCP server process. Since MCP uses stdin/stdout for JSON-RPC transport, child processes reading from the shared stdin corrupt or deadlock the protocol.

Fix: Create vvce_git_command() factory in vvce_env.rs that sets stdin(Stdio::null()) on all git subprocess calls. Apply same fix to vvce_claude_command(). Replace all 28 Command::new("git") call sites across vvc and jjk crates with the factory. Remove probe disable workaround — all three model probes (haiku/sonnet/opus) now run successfully in parallel during invitatory.

**[260306-1943] rough**

Fix MCP transport deadlock caused by child processes inheriting the MCP server's stdin fd.

Root cause: Command::new("git") and Command::new("claude") spawn subprocesses that inherit stdin from the parent MCP server process. Since MCP uses stdin/stdout for JSON-RPC transport, child processes reading from the shared stdin corrupt or deadlock the protocol.

Fix: Create vvce_git_command() factory in vvce_env.rs that sets stdin(Stdio::null()) on all git subprocess calls. Apply same fix to vvce_claude_command(). Replace all 28 Command::new("git") call sites across vvc and jjk crates with the factory. Remove probe disable workaround — all three model probes (haiku/sonnet/opus) now run successfully in parallel during invitatory.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A consider-mcp-transport-for-jjx
  2 B jjs0-mcp-transport-spec
  3 D jjx-stdin-to-params
  4 C vvx-mcp-stdio-server
  5 E vvx-emplace-mcp-config
  6 I mcp-single-dispatcher-tool
  7 G remove-cli-dispatch-path
  8 F claudemd-mcp-migration
  9 H investigate-jjs0-crash-safe-staleness
  10 J fix-mcp-subprocess-stdin-inheritance

ABDCEIGFHJ
···x··x··x lib.rs
···x·x·x·· jjrm_mcp.rs
··xx·····x jjrtl_tally.rs, jjrwp_wrap.rs
··xx··x··· jjrx_cli.rs
········xx vvcp_probe.rs
···x·····x jjrmu_muster.rs, jjrnc_notch.rs, jjrrt_retire.rs, jjrs_steeplechase.rs, jjrsd_saddle.rs
···x··x··· vorm_main.rs
···x·x···· Cargo.lock
··xx······ jjrcu_curry.rs, jjrld_landing.rs, jjrrs_restring.rs, jjrsl_slate.rs
·········x jjrq_query.rs, jjru_util.rs, vvcc_commit.rs, vvcc_format.rs, vvce_env.rs, vvcg_guard.rs, vvcm_machine.rs, vvtg_guard.rs
·······x·· BCG-BashConsoleGuide.md, CLAUDE.md, RCG-RustCodingGuide.md, vocjjmc_core.md
······x··· rbrr.env
···x······ .gitignore, Cargo.toml, jjrch_chalk.rs, jjrdr_draft.rs, jjrfu_furlough.rs, jjrgc_get_coronets.rs, jjrgl_garland.rs, jjrgs_get_spec.rs, jjrno_nominate.rs, jjrp_print.rs, jjrpd_parade.rs, jjrrl_rail.rs, jjrrn_rein.rs, jjrsc_scout.rs, jjrvl_validate.rs
·x········ JJS0-GallopsData.adoc, JJSCDR-draft.adoc, JJSCFU-furlough.adoc, JJSCGC-get-coronets.adoc, JJSCGL-garland.adoc, JJSCGS-get-spec.adoc, JJSCMU-muster.adoc, JJSCNO-nominate.adoc, JJSCPD-parade.adoc, JJSCRL-rail.adoc, JJSCRT-retire.adoc, JJSCSC-scout.adoc, JJSCSD-saddle.adoc, JJSCTL-tally.adoc, JJSCVL-validate.adoc, JJSRLD-load.adoc, JJSRPS-persist.adoc, JJSRSV-save.adoc
x········· RBS0-SpecTop.adoc, rbf_Foundry.sh, rbgc_Constants.sh, rbgjb03-build-and-load.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 59 commits)

  1 D jjx-stdin-to-params
  2 C vvx-mcp-stdio-server
  3 E vvx-emplace-mcp-config
  4 G remove-cli-dispatch-path
  5 I mcp-single-dispatcher-tool
  6 F claudemd-mcp-migration
  7 J fix-mcp-subprocess-stdin-inheritance
  8 H investigate-jjs0-crash-safe-staleness

123456789abcdefghijklmnopqrstuvwxyz
··xxx······························  D  3c
·····xxxx··xx······················  C  6c
·············x·····················  E  1c
··············x··xx················  G  3c
···················xxx·············  I  3c
······················x·xxx········  F  4c
····························xx····x  J  3c
·······························xxx·  H  3c
```

## Steeplechase

### 2026-03-06 19:53 - ₢AoAAJ - W

Fixed MCP transport deadlock: created vvce_git_command factory with stdin(Stdio::null()), applied same fix to vvce_claude_command, replaced all 28 Command::new("git") call sites across vvc and jjk with factory, re-enabled all three model probes. Follow-up pace cleaned up 9 dead-code items from old XML probe approach.

### 2026-03-06 19:52 - ₢AoAAH - W

Two tasks: (1) Audit JJS0 Crash-Safe Architecture section against actual Rust locking/commit implementation, correct if stale. (2) Remove 9 dead-code items from vvcp_probe.rs (old XML probe leftovers). Build to confirm clean.

### 2026-03-06 19:52 - ₢AoAAH - n

Remove 9 dead-code items from vvcp_probe.rs (old XML probe leftovers)

### 2026-03-06 19:48 - ₢AoAAH - A

Two tasks: (1) Audit JJS0 Crash-Safe Architecture section against actual Rust locking/commit implementation, correct if stale. (2) Remove 9 dead-code items from vvcp_probe.rs (old XML probe leftovers). Build to confirm clean.

### 2026-03-06 19:47 - Heat - T

investigate-jjs0-crash-safe-staleness

### 2026-03-06 19:45 - ₢AoAAJ - n

Restore VVCP_OFFICIUM_GAP_SECS to 3600, clean up experiment comments, fix stale doc comment

### 2026-03-06 19:44 - ₢AoAAJ - n

Fix MCP transport deadlock: add vvce_git_command factory with stdin(Stdio::null()), add stdin null to vvce_claude_command, replace all 28 Command::new("git") call sites across vvc and jjk with factory, re-enable all three model probes

### 2026-03-06 19:43 - Heat - S

fix-mcp-subprocess-stdin-inheritance

### 2026-03-06 13:37 - ₢AoAAF - W

Rewrote CLAUDE.md JJK section for MCP-native usage with canonical jjx_* command names, added Interface Contamination Discipline to RCG and BCG, removed strip_prefix tolerance from MCP dispatcher

### 2026-03-06 13:33 - ₢AoAAF - n

Rewrite CLAUDE.md JJK section for MCP-native usage with canonical jjx_* command names, remove strip_prefix tolerance from dispatcher, add Interface Contamination Discipline to RCG and BCG

### 2026-03-06 13:04 - ₢AoAAF - A

Read current CLAUDE.md + jjrm_mcp.rs + vocjjmc_core.md, fix remaining bash patterns, verify params match code, ensure acceptance criteria met

### 2026-03-06 13:02 - Heat - T

claudemd-mcp-migration

### 2026-03-06 12:58 - ₢AoAAF - A

Rewrite CLAUDE.md JJK section: verb table to MCP tool names, remove bash/heredoc patterns, update protocols for MCP calls, preserve semantics

### 2026-03-06 12:55 - ₢AoAAI - W

Rewrite jjrm_mcp.rs: single jjx tool with command+params dispatch, replacing 25 individual tools

### 2026-03-06 12:55 - ₢AoAAI - n

Rewrite jjrm_mcp.rs: single jjx tool with command+params dispatch, replacing 25 individual tools

### 2026-03-06 12:51 - ₢AoAAI - A

Rewrite jjrm_mcp.rs: single jjx tool with command+params dispatch, replacing 25 individual tools

### 2026-03-06 12:51 - ₢AoAAG - W

Removed CLI dispatch path for jjx_* commands: deleted jjrx_cli.rs, removed jjk delegation from dispatch_external in vorm_main.rs, removed re-exports from lib.rs. CLI now returns 'unknown command' for jjx_*, all operations work via MCP only.

### 2026-03-06 12:51 - ₢AoAAG - n

Remove jjk CLI dispatch from vorm_main.rs, dead exports from lib.rs, assess jjrx_cli.rs removal, audit tt/ for CLI jjx_* tabtargets

### 2026-03-06 12:50 - Heat - r

moved AoAAI before AoAAG

### 2026-03-06 12:49 - Heat - S

mcp-single-dispatcher-tool

### 2026-03-06 12:41 - ₢AoAAG - A

Remove jjk CLI dispatch from vorm_main.rs, dead exports from lib.rs, assess jjrx_cli.rs removal, audit tt/ for CLI jjx_* tabtargets

### 2026-03-06 12:40 - ₢AoAAE - W

MCP server registered via .mcp.json, Claude Code discovers and calls all 25 tools successfully

### 2026-03-06 12:40 - ₢AoAAC - W

MCP stdio server implemented with 25 tools, stdout capture replaced with buf-return pattern, all tools verified working from Claude Code

### 2026-03-06 12:40 - ₢AoAAC - n

Replace stdout capture (libc pipe/dup2) with return-value plumbing: all handlers return (i32, String), MCP layer uses jjrm_result(), CLI uses emit! macro, jjrp_Table writes to buffer, drop libc dependency

### 2026-03-06 11:53 - Heat - S

investigate-jjs0-crash-safe-staleness

### 2026-03-06 11:37 - Heat - S

remove-cli-dispatch-path

### 2026-03-06 11:35 - ₢AoAAC - n

Fix async capture deadlock (add reader thread), remove validate file param, add SAFETY comments, fix stale doc comment

### 2026-03-06 11:28 - ₢AoAAC - n

Restore CLI dispatch for jjx commands during MCP bootstrap transition

### 2026-03-06 11:28 - ₢AoAAC - n

Implement MCP stdio server with 25 jjx tools, restore CLI dispatch for bootstrap transition

### 2026-03-06 10:53 - ₢AoAAC - A

Add rmcp dep, create MCP tool defs for all jjx ops, add vvx-mcp subcommand, remove vvx-jjx CLI path

### 2026-03-06 10:52 - ₢AoAAD - W

Refactored 7 stdin-reading jjx handlers to accept text as function parameters, moved stdin reads to CLI dispatch layer

### 2026-03-06 10:52 - ₢AoAAD - n

Refactor 7 stdin-reading handlers: change fn signatures to accept String params, remove stdin reads, update CLI call sites to bridge

### 2026-03-06 10:43 - ₢AoAAD - A

Refactor 7 stdin-reading handlers: change fn signatures to accept String params, remove stdin reads, update CLI call sites to bridge

### 2026-03-06 10:38 - Heat - T

vvx-emplace-mcp-config

### 2026-03-06 10:37 - Heat - T

jjx-stdin-to-params

### 2026-03-06 10:37 - Heat - r

moved AoAAD before AoAAC

### 2026-03-06 10:37 - ₢AoAAB - W

JJS0 spec updated for MCP transport: new MCP Transport section, vocabulary, Crash-Safe Architecture revised, CLI removal documented, stdin-to-params across 22 spec files, review fixes applied

### 2026-03-06 10:35 - ₢AoAAB - n

Fix review issues: Exclusive JSON Ownership references handler not cli, Presentation Vocabulary says display not CLI

### 2026-03-06 10:33 - ₢AoAAB - n

JJS0 MCP transport spec: new MCP Transport section, revised Crash-Safe Architecture, CLI removal, stdin-to-params, jjda_file removal across all operation specs

### 2026-03-06 10:23 - ₢AoAAB - A

Sequential spec edits: mapping terms, Crash-Safe revision, new MCP Transport section, CLI/Operations/Upper API revisions, stdin-to-params for affected operations

### 2026-03-06 10:21 - Heat - d

paddock curried

### 2026-03-06 10:21 - ₢AoAAA - W

Evaluation complete. Decision: GO for MCP-only transport. Implementation paces slated.

### 2026-03-06 10:21 - ₢AoAAA - n

Add vouch artifact lifecycle and dual-tag single-platform builds

### 2026-03-06 10:20 - Heat - T

claudemd-mcp-migration

### 2026-03-06 10:19 - Heat - T

vvx-emplace-mcp-config

### 2026-03-06 10:19 - Heat - T

jjx-stdin-to-params

### 2026-03-06 10:19 - Heat - T

vvx-mcp-stdio-server

### 2026-03-06 10:19 - Heat - T

jjs0-mcp-transport-spec

### 2026-03-06 10:03 - Heat - S

claudemd-mcp-migration

### 2026-03-06 10:02 - Heat - S

vvx-emplace-mcp-config

### 2026-03-06 10:02 - Heat - S

jjx-stdin-to-params

### 2026-03-06 10:02 - Heat - S

vvx-mcp-stdio-server

### 2026-03-06 10:02 - Heat - S

jjs0-mcp-transport-spec

### 2026-03-06 09:51 - Heat - T

consider-mcp-transport-for-jjx

### 2026-03-06 09:51 - Heat - d

paddock curried

### 2026-03-06 09:39 - Heat - d

paddock curried

### 2026-03-06 09:38 - Heat - D

AhAAH → ₢AoAAA

### 2026-03-06 09:38 - Heat - f

racing

### 2026-03-06 09:38 - Heat - N

jjk-v3-recast-mcp

