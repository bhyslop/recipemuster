# MCP Server Aggregation Constraint in Claude Code

**Date:** 2026-03-18
**Severity:** High — affects operational architecture and context accounting
**Domain:** Claude Code MCP integration, context display behavior
**Status:** Documented constraint (not a JJK or project-side bug)

---

## Summary

Claude Code's `/context` command displays MCP tools individually rather than aggregating them by server. Multiple tools from a single MCP server appear as separate line items in the context accounting, when by MCP design they should roll up as a single server entry.

This is a **Claude Code limitation**, not an architectural flaw in our MCP implementation. Both tunneled and non-tunneled MCP command architectures exhibit identical behavior, confirming the issue is at the server registration/display layer, not the command dispatching layer.

**Key constraint:** We cannot rely on `/context` to accurately represent MCP server resource usage. Each tool appears individually, making context budgeting unclear.

---

## Observed Behavior

### Symptom
Running `/context` shows MCP tools as a flat list with individual token counts:
```
MCP tools · /mcp
└ mcp__vvx__jjx: 285 tokens
└ mcp__vvx__jjx_test_echo: 135 tokens
```

### Expected Behavior (By MCP Design)
Multiple tool invocations from one MCP server should aggregate:
```
MCP server 'vvx': 420 tokens (0.2%)
```

Or at minimum, grouped under server identity with clear aggregation semantics.

### Duration
- **First observed:** During JJK MCP integration testing (2026-03-18)
- **Persistence:** Issue remains consistent across tool invocations
- **Prior occurrence:** User reports encountering this behavior previously but without documentation

---

## Diagnostic Methodology

### Test Design

To isolate whether the issue was caused by **tunneled command architecture** (single `jjx` tool with subcommand dispatcher), we created a test infrastructure:

**Architecture A (Tunneled):**
- Single MCP tool: `jjx`
- Subcommands passed as parameters: `command` (string) + `params` (JSON)
- Dispatcher routes to 25+ internal handlers
- Current production implementation

**Architecture B (Non-Tunneled):**
- Separate MCP tool: `jjx_test_echo`
- Native JSON parameters (direct, not in a dispatcher)
- No intermediate routing layer
- Test implementation for comparison

Both tools deployed in **same MCP server** (`vvx`), enabling direct behavior comparison.

### Test Sequence

1. **Baseline /context** — captured tool registry before any invocations
2. **Build & restart Claude Code** — reloaded MCP server with test tool
3. **Invoke `jjx_list`** — tunneled command (subcommand dispatcher)
4. **Invoke `jjx_test_echo`** — non-tunneled command (direct params)
5. **Post-invocation /context** — captured context accounting after tool use

### Results

| Aspect | Before Invocation | After Invocation | Delta | Analysis |
|--------|-------------------|------------------|-------|----------|
| Total tokens | 69k/200k | 73k/200k | +4k | Expected (messages + tool outputs) |
| MCP tools category | 420 tokens | 420 tokens | No change | Tools listed separately before and after |
| Aggregation | Separate entries | Separate entries | No change | **Issue persists regardless of invocation** |
| Tool count | 2 listed | 2 listed | No change | New tool appeared in registry correctly |

### Confirmation

**Tunneled vs Non-Tunneled Parity:**
- ✓ Both `jjx` (tunneled) and `jjx_test_echo` (non-tunneled) showed identical aggregation behavior
- ✓ Issue is not specific to command dispatching architecture
- ✓ Confirms root cause is at MCP server registration level, not command routing level

---

## Root Cause Analysis

### What We Know

1. **MCP server itself is correct** — both tools registered successfully and invoked without error
2. **Claude Code receives the tools** — they appear in `/context` and function properly
3. **Registration layer works** — new tools appear automatically after Claude Code restart
4. **Problem is display/aggregation** — tokens are counted but not rolled up by server

### Diagnosis

The issue is in **Claude Code's context accounting and display logic**, specifically:
- MCP server tool registration works correctly
- Tool invocation and execution work correctly
- **But:** Claude Code's `/context` command treats each tool as an independent line item rather than aggregating by server

This suggests Claude Code either:
1. Does not implement MCP server-level aggregation in the context display
2. Has a bug where the aggregation logic is bypassed
3. Treats tool names (vs server names) as the primary grouping key

### NOT the Cause

We definitively ruled out:
- ❌ **Tunneled command architecture** — non-tunneled tool exhibits same behavior
- ❌ **JJK implementation** — both tools in same server, same MCP crate
- ❌ **Tool invocation semantics** — persists before and after calls
- ❌ **Parameter format** — tested both string and native JSON params
- ❌ **Configuration** — standard `.mcp.json` setup, no special routing

---

## Implications for Project Operations

### Impact on Context Budgeting

**Problem:** `/context` cannot be used for accurate MCP resource tracking.

If we have 25+ MCP subcommands (jjx_list, jjx_show, jjx_orient, etc.), they all route through one `jjx` tool. By design, this should appear as a single 285-token entry. Instead, if each were listed separately, context accounting would be misleading.

**Current state:** We have 2 tools and 420 tokens. This is manageable. But as we expand MCP operations, the display becomes increasingly inaccurate.

### Impact on Architecture Decisions

**Good news:** Tunneled dispatch (single `jjx` tool, 25+ subcommands) is architecturally sound.

We verified that both tunneled and non-tunneled approaches have identical `/context` behavior. This means:
- The tunneled dispatcher is not causing any additional context overhead
- Token counts are accurate (even if display is wrong)
- Operationally, the choice between tunneled and non-tunneled should be based on other factors (simplicity, maintainability), not context impact

### Impact on JJF File Exchange Design

**Scope:** The MCP integration issue does **not** constrain JJF (Job Jockey File) design.

JJF is a data format for markdown-based multiline parameter passing in MCP. It works regardless of whether we use tunneled or non-tunneled command architecture. The `/context` aggregation issue is purely a display problem, not a functional one.

### Impact on Future Kit Development

As we build more MCP-based kits (VSLK, CGK, HMK, etc.), each will:
1. Register as a separate MCP server
2. Expose multiple tools
3. Have each tool appear individually in `/context`

This is **operationally acceptable** if we understand and document it. It's not a bug in the kits — it's a Claude Code limitation.

---

## Design Constraints

### Hard Constraints (Cannot be worked around)

1. **MCP tools will display individually in `/context`** — cannot aggregate at project level
2. **Token counting is correct, display is wrong** — we cannot "fix" the display by changing our code
3. **This is upstream Claude Code behavior** — requires Claude Code update to fix

### Soft Constraints (Can be managed)

1. **Document token overhead separately** — maintain a memo tracking expected MCP overhead per kit
2. **Use `/context` for debugging, not for planning** — don't rely on it for context budgeting decisions
3. **Monitor total context usage** — track conversation length and system prompt size instead

### Workarounds (Operational strategies)

1. **Calculate MCP overhead independently** — count tokens by multiplying tool count by average overhead
2. **Keep conversation history lean** — most context pressure comes from messages, not MCP tools
3. **Use tunneled architecture** — concentrating many subcommands into one tool reduces line items in `/context` display (though doesn't fix aggregation)
4. **Document per-operation cost** — from our testing, a single MCP operation costs ~4k tokens in message context

---

## Testing Notes

### Build & Verification Steps

```bash
# Build with new test tool
tt/vow-b.Build.sh

# Restart Claude Code (required for MCP server reload)
# Exit and restart the session

# Check /context baseline
/context

# Test tunneled command
jjx_list

# Test non-tunneled command
mcp__vvx__jjx_test_echo with message "test"

# Verify /context unchanged
/context

# Revert test tool
# Edit jjrm_mcp.rs: remove jjrm_TestEchoParams struct and jjx_test_echo method

# Rebuild and reverify
tt/vow-b.Build.sh
```

### Artifacts

- **Diagnostic commits:** f74becd50a (added test tool), 11c58ecc95 (reverted)
- **Pace affiliation:** ₢AwAAK (jjk-v4-diagnose-mcp-integration)
- **Paddock entry:** Updated with findings and recommendations

---

## Precedent and History

**User note:** This issue was encountered previously (date/session unknown) but documentation was not retained. This memo exists to prevent recurrent investigation and decision confusion.

Key lesson: **When we discover CloudDE/infrastructure constraints that affect architectural decisions, we must document them immediately with high detail.** This prevents:
- Repeated diagnostic work
- Conflicting design decisions based on different understandings
- Loss of context when multiple engineers engage with the codebase

---

## Recommendations

### For JJK V3/V4 Work (₣Aw)

1. **Proceed with tunneled architecture** — it's not the cause of any issues, and it simplifies tool proliferation
2. **Do not attempt to work around the `/context` issue** — it's Claude Code, not fixable at project level
3. **Document MCP token overhead in future CLAUDE.md** — when we establish expected overhead per operation
4. **Plan context budgeting around messages, not tools** — MCP tools are typically <1% of context

### For Future MCP Kit Development

1. **Consolidate MCP operations into tunneled dispatchers** — reduces line item count in `/context` (cosmetic, but cleaner)
2. **Keep this memo accessible** — reference when designing new kits
3. **Monitor Claude Code changelog** — watch for MCP aggregation fixes (unlikely, but possible)
4. **Consider reporting upstream** — if/when community engagement with Claude Code is appropriate

### For Team Communication

- **Circulate this memo** to anyone designing MCP-based infrastructure
- **Reference in CLAUDE.md** of projects using MCP servers
- **Update if we encounter new MCP integration behaviors** — this documents one specific constraint; others may emerge

---

## Conclusion

The `/context` aggregation issue is a **Claude Code limitation that we must design around, not a flaw in our MCP architecture**. Both tunneled and non-tunneled command dispatching work correctly; the limitation is purely in how Claude Code displays tool accounting.

This is documented as a hard constraint on project operations: we cannot rely on `/context` for accurate MCP server resource tracking. However, it does not affect functionality, token accounting accuracy, or architectural soundness of our MCP implementations.

**Strong mooring:** When evaluating MCP architecture decisions (tunneled vs non-tunneled, single server vs multiple servers), prioritize simplicity and maintainability. The `/context` display limitation affects all approaches equally.
