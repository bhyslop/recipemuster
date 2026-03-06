# Paddock: jjk-v3-recast-mcp

## Mission

Promote all vvx commands (jjx_* and vvx_*) to MCP stdio server transport. Claude Code accesses vvx exclusively via MCP — the bash invocation path (`./tt/vvw-r.RunVVX.sh jjx_*`) is eliminated for Claude. The CLI entry point remains for hooks, tabtargets, and manual use.

## Why separate heat

MCP transport is a foundation change benefiting all kits (VVK, JJK, future kits), not specific to V4 schema evolution. JJK is under active concurrent load across multiple officium right now — permission friction from heredoc-based bash invocation is a daily pain point.

## Design decisions (from ₢AoAAA evaluation, refined in chat)

- **Claude uses MCP exclusively**: Claude Code never invokes vvx via Bash tool. MCP is the sole invocation path for Claude.
- **CLI preserved for non-Claude contexts**: Hooks, tabtargets, manual debugging, and future shell-context tools can still call the binary directly. Two front doors, same handlers.
- **Stateless per tool call**: Each MCP tool call does full lock → read → modify → write → unlock cycle. No in-memory state held between calls. This is critical because each Claude session spawns a long-lived MCP server process — unlike short-lived CLI processes, a long-lived server that cached gallops between calls would see stale state from other sessions' writes.
- **Existing commands**: All current jjx_* and vvx_* commands become MCP tools with typed schemas derived from existing clap Args structs.
- **Stdin commands become structured params**: The 7 stdin-reading commands (enroll, revise_docket, arm, transfer, close, paddock, landing) gain string parameters instead. No more heredoc escaping or delimiter conflicts.
- **Registration via kit install**: vvx_emplace adds mcpServers config to .claude/settings.local.json.

## Concurrency discipline

Multiple Claude sessions = multiple long-lived MCP server processes, one per session. The git-ref locking model (`vvg_lock_acquire`) already serializes concurrent access. This works because:
- Locks are per-tool-call, not per-session
- No in-memory state survives between tool calls
- Lock orphaning risk is identical to CLI model (git-ref locks survive process death; `vvx_unlock` is the recovery path)

This heat must prove this under real multi-officium concurrent load, not just assert it.

## CLAUDE.md impact

After MCP:
- Verb table no longer needs `./tt/vvw-r.RunVVX.sh jjx_*` invocation patterns — Claude sees MCP tool schemas natively
- Heredoc guidance section becomes unnecessary (structured params replace stdin pipes)
- CLI Command Reference may simplify to parameter documentation rather than shell syntax

## Prove-under-load discipline

This heat is not done until multi-officium concurrent sessions are running on MCP transport. The transport must be proven under the same concurrent pressure that the bash transport handles today.

## V4 ripple

On completing this heat, groom ₣Ah (jjk-v4-1-school-breeze-founding) to incorporate MCP transport assumptions into remaining paces. Specifics deferred until MCP details are settled — don't predict what changes until the foundation is proven.

## References

- ₢AoAAA docket: MCP evaluation with 7 questions resolved
- `Tools/vok/src/vorm_main.rs`: vvx entry point (`dispatch_external` seam)
- `Tools/jjk/vov_veiled/src/jjrx_cli.rs`: jjx command enum and dispatch
- Chat context: concurrency concerns about long-lived MCP server processes, decision to preserve CLI for non-Claude contexts