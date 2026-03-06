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