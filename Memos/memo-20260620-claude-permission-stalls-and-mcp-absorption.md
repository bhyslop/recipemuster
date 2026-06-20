# Claude Code Permission Stalls — Why the Matcher Fails, and the MCP-Absorption Strategy

**Date:** 2026-06-20
**Author:** Claude (Opus 4.8), with Brad
**Status:** provenance / reference capture — so we never re-dig this history again

## Why this memo exists

We have repeatedly rediscovered, from scratch, that Claude Code's Bash permission
matching is unreliable — and repeatedly re-derived that this is *the* reason Job
Jockey was converted to an MCP server. This memo is the durable record of the
problem and **all the supporting URLs**, so a future instance (or Brad) can read
one file instead of excavating retired heats and re-running web searches.

This is **provenance, not authority** (per the memo discipline): nothing here is
load-bearing. The decisions it points at live in the heat that carries the work,
and any fact that must stay true after this memo retires needs a spec home, not a
citation here.

## Where the record used to live (and why it was hard to refind)

- **The original symptom record** is the retired heat trophy
  `.claude/jjm/retired/jjh_b260209-r260310-jjk-cue-calibration.md`, pace
  `attempt-bash-permission-grant` (₢AYAAC), drafted repeatedly 2026-02-07 →
  2026-03-07. It documents the per-invocation "Do you want to proceed?" stall, a
  614-entry allow-list grown one click at a time, the Feb–Mar bug citations, and
  the verdict *"the permission system may simply be broken for Bash."* The pace was
  marked exploratory, *"success is not expected."*
- **The current-pile catalog** is a *separate* artifact: the beta clone's
  `Memos/memo-20260620-claude-permission-sediment-catalog.md` (a ~345KB generated
  dump of the live `settings.local.json` allow-lists across clones). That memo
  catalogs *what accumulated*; this memo records *why it accumulates and what to do*.
- **The MCP-conversion commit** is `4cdaa87b7` (2026-04-05): "all functionality
  (slash commands, ... settings permissions) superseded by MCP tool and Rust layer."

## The core finding

Claude Code has **two independent, still-open defect clusters** that together
guarantee the permission patch grows without bound:

### Cluster A — the allow-matcher is unreliable

Scoped Bash allow-patterns (`Bash(cmd:*)` / `Bash(cmd *)`) frequently fail to
match the command they should authorize, so the operator is re-prompted and the
exact invocation is appended to `permissions.allow` — the accumulation mechanism.

- [#18160](https://github.com/anthropics/claude-code/issues/18160) (OPEN) — `Bash(ls *)`-style patterns in global settings "sometimes ignored" (the original citation; opened Jan 2026, still surfacing).
- [#29616](https://github.com/anthropics/claude-code/issues/29616) — Bash wildcard permissions in `settings.local.json` not matching commands (newer dupe).
- [#27139](https://github.com/anthropics/claude-code/issues/27139) — broad wildcard permissions not respected, still prompted for individual actions (newer dupe).
- [#8581](https://github.com/anthropics/claude-code/issues/8581) — wildcards + "Always allow" break when the command has an **environment-variable prefix** (`VAR=x cmd ...`).
- [#13340](https://github.com/anthropics/claude-code/issues/13340) (OPEN) — **piped** commands don't match individual allow patterns.
- [#10467](https://github.com/anthropics/claude-code/issues/10467) — wildcard in the *middle* of a command fails to match.
- [#15921](https://github.com/anthropics/claude-code/issues/15921) (OPEN) — VSCode extension Bash permissions use a different code path than Read; config doesn't connect.
- [#6881](https://github.com/anthropics/claude-code/issues/6881) (OPEN) — recursive `/**` glob patterns broken for Read/Edit paths.
- [#28784](https://github.com/anthropics/claude-code/issues/28784) — `Bash(cd:*)` (and any prefix rule) allows arbitrary command execution via `&&` chaining — a *security* reason not to widen globs.
- [#20254](https://github.com/anthropics/claude-code/issues/20254) — [DOCS] Bash permission pattern limitations need stronger guidance.
- [#3428](https://github.com/anthropics/claude-code/issues/3428) (CLOSED) — Anthropic confirmed bare `"Bash"` is the **only** valid all-allow form; `"Bash:*"` and `"Bash(*)"` are **not** valid syntax.

**Matcher mechanics learned (from the official [permissions docs](https://code.claude.com/docs/en/permissions)):**
- `Bash(cmd:*)` is the documented prefix form, equivalent to `Bash(cmd *)`; `:*`
  works only at the *end* of a pattern. A trailing `*` is optional, so `Bash(git *)`
  also matches bare `git`.
- Claude Code **parses shell operators by design**: `Bash(safe:*)` deliberately will
  *not* authorize `safe && other`. This is the root mechanism — a prefix rule matches
  only a **bare, single, un-chained, un-piped, un-env-prefixed** invocation. Any `&&`,
  `|`, or leading `VAR=...` breaks the match → fresh prompt → new sediment entry. This
  is why the same logical command accretes in both `./tt/...` and absolute `/…/tt/...`
  forms, and why `BURE_CONFIRM=skip ./tt/foo.sh` re-prompts.

### Cluster B — the model doesn't choose native tools

Even the read-only inspection sediment (echo/sed/find/grep/cat/ls/wc) is evidence of
a *model-level* defect: Claude Code's system prompt **already** forbids Bash
`find`/`grep` in capitalized text and directs to native Glob/Grep — and the model
violates it anyway. Their very presence in the patch proves the native ops were not
chosen.

- [#39979](https://github.com/anthropics/claude-code/issues/39979) — [MODEL] Claude uses Bash (cat, grep, head) instead of dedicated tools **despite system-prompt prohibition**.
- [#21696](https://github.com/anthropics/claude-code/issues/21696) — frequently uses Bash cat/ls instead of dedicated Read/Glob.
- [#19649](https://github.com/anthropics/claude-code/issues/19649) — [MODEL] uses Bash sed/grep/etc when the use-case aligns with builtin tools.
- [#6971](https://github.com/anthropics/claude-code/issues/6971) — [DOCS] the explicit Bash restriction against find/grep is undocumented.

**Root cause (from the writeups):** LLM training data is saturated with bash
one-liners, so the model defaults to "what it knows from Stack Overflow," not what it
has available — **and the preference erodes after context compaction**, when the
dedicated-tool instruction gets compressed away. A behavioral fix ("just use native
tools") is therefore the weakest possible remedy: it re-promises exactly what the
system prompt already mandates and the whole fleet fails at.

## Why MCP escapes both clusters

An MCP tool call **is not a shell string** — there is no operator parsing, no
chaining ambiguity, no env-prefix breakage, nothing for the matcher to misfire on. A
single `mcp__vvx__jjx` (or sibling vvx tool) approval covers every invocation,
present and future. That is the structural reason the JJK MCP conversion ended the
stalls where every settings-glob attempt had failed.

## Strategy (in progress — see the carrying heat, not this memo, for the live shape)

Three complementary moves, being designed as of this date:

1. **vvx tabtarget runner** — a bounded MCP tool that execs `tt/*.sh` launchers
   (one approval; also encodes the tabtarget discipline: run from root, never pipe to
   tail/head, read `../logs-buk/`). Absorbs ~half the patch plus the cargo/docker
   long tail.
2. **jjx revision-substrate surveyor** — read-only inspection of the version-control
   substrate behind a **neutral interface vocabulary** but with **substrate-honest
   output** (it names the RCS and repo; it surfaces multi-repo topology rather than
   flattening it). Seeds JJK's multi-repo / planned-worktrees future. *Design note:
   neutral verb + honest payload — do NOT hide the underlying RCS from the LLM, which
   operates the substrate and needs it named; the LLM is the consumer, and disclosure
   beats abstraction for an operator.*
3. **PreToolUse hook** — the deterministic policy layer the model's behavior can't
   provide: deny Bash `grep`/`find`/`sed`-for-reading with a redirect to native tools
   (immune to compaction erosion, unlike a prompt sentence), and denylist the
   genuinely dangerous (rm -rf, sudo, git push/reset). Posture leaning: **hybrid** —
   keep the allowlist stance (Bash stays gated, not bare-`"Bash"`-allowed, given the
   security-sensitivity of this project) and let the hook + MCP tools carry the load,
   so the broken matcher is never relied upon.

## Cross-references

- Retired trophy (original symptom record): `.claude/jjm/retired/jjh_b260209-r260310-jjk-cue-calibration.md`
- Current-pile catalog (beta clone): `Memos/memo-20260620-claude-permission-sediment-catalog.md`
- MCP-conversion commit: `4cdaa87b7`
- Community hook pattern: [Claude Code Ignores Its Own Tools — 3 Hooks That Force It to Behave](https://dev.to/yurukusa/claude-code-ignores-its-own-tools-here-are-3-hooks-that-force-it-to-behave-mi1)
- Tools reference (native Read/Grep/Glob): https://code.claude.com/docs/en/tools-reference
