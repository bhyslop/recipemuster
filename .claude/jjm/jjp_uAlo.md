# Paddock: jjk-v3-recast-mcp

## Mission

Promote existing jjx commands from bash-stdin invocation to MCP stdio server transport. Burn the bash bridge — no backward compatibility, no dual mode.

## Why separate heat

MCP transport is a foundation change benefiting all kits (VVK, JJK, future kits), not specific to V4 schema evolution. JJK is under active concurrent load across multiple officium right now — permission friction from heredoc-based bash invocation is a daily pain point.

## Design decisions (from ₢AoAAA evaluation)

- **No backward compat**: vvx becomes MCP stdio server only. CLI entry point removed.
- **Stateless per tool call**: Each MCP tool call does full lock → read → modify → write → unlock cycle. No in-memory state between calls.
- **Existing commands**: All 27 current jjx_* commands become MCP tools with typed schemas derived from existing clap Args structs.
- **Stdin commands become structured params**: The 7 stdin-reading commands (enroll, revise_docket, arm, transfer, close, paddock, landing) gain string parameters instead.
- **Registration via kit install**: vvx_emplace adds mcpServers config to .claude/settings.local.json.

## Prove-under-load discipline

This heat is not done until multi-officium concurrent sessions are running on MCP transport. The transport must be proven under the same concurrent pressure that the bash transport handles today.

## V4 ripple

On completing this heat, groom ₣Ah (jjk-v4-1-school-breeze-founding) to incorporate MCP transport assumptions into remaining paces. Specifics deferred until MCP details are settled — don't predict what changes until the foundation is proven.

## References

- ₢AoAAA docket: full evaluation with 7 questions resolved
- `Tools/vok/src/vorm_main.rs`: vvx entry point (dispatch_external seam)
- `Tools/jjk/vov_veiled/src/jjrx_cli.rs`: jjx command enum and dispatch