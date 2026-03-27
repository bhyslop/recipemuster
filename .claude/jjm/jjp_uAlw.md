# Paddock: jjk-v4-0-jjs0-axla-normalization

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

## Remaining Work — Two Phases

### Phase 1: Officium Plumbing (spec → implement → doc → user test)

| Pace | Silks | Character |
|------|-------|-----------|
| ₢AwAAY | spec-agent-initiated-officium | Spec-only: JJS0 officium model |
| ₢AwAAQ | implement-jjx-open-and-officium-param | Infrastructure: jjx_open + param threading |
| ₢AwAAU | claudemd-officium-protocol | Documentation for agent behavior |

User tests officium plumbing in real chats before proceeding.

### Phase 2: Gazette Migration

| Pace | Silks | Character |
|------|-------|-----------|
| ₢AwAAZ | spec-chapter-absolve-operations | Spec-only: chapter/absolve |
| ₢AwAAS | implement-chapter-and-absolve | Diagnostic operations |
| ₢AwAAV | wire-gazette-to-officium-exchange | Handler wiring — the real payoff |
| ₢AwAAW | remove-legacy-duplicate-output | Strip inline gazette from MCP responses |
| ₢AwAAX | jjs0-officium-lifecycle-spec-update | Final spec verification pass |
| ₢AwAAT | test-officium-lifecycle | End-to-end testing |

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