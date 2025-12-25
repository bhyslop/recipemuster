# Job Jockey Kit

## What is Job Jockey?

Job Jockey (JJ) is a lightweight system for managing project initiatives through conversation with Claude Code. It helps you track bounded heats, remember what's next, and keep a backlog of ideas without drowning in ceremony or context bloat.

Think of it as a project notebook specifically designed for human-AI collaboration:
- **Heats** are your current work (3-50 chat sessions worth)
- **Paces** track what's done and what's next within a heat
- **Itches** capture future ideas without losing focus
- **Scars** record closed ideas with lessons learned

The system is ephemeral by design: documents have clear lifecycles, completed work gets archived, and context stays lean. Everything is markdown, lives in git, and can move between computers with you.

## Naming Prefixes

All Job Jockey artifacts use the `jj` prefix with category-specific third letters:

| Prefix | Category | Purpose |
|--------|----------|---------|
| `jjm/` | Memory | State directory (`.claude/jjm/`) |
| `jjh_` | Heat | Bounded initiative files |
| `jji_` | Itch | Future work aggregate |
| `jjs_` | Scar | Closed work aggregate |
| `jja_` | Action | Slash commands |
| `jjk_` | sKill | (Future) Skill definitions |
| `jjg_` | aGent | (Future) Agent definitions |
| `jjl_` | Log | (Future) Action logs |

## Core Concepts

### Heat
A bounded initiative with **coherent goals that are clear and present**. Spans 3-50 chat sessions. Has a goal, context section, and list of paces. Lives as a dated file like `jjh_b251108-buk-portability.md`.

Heat location indicates state:
- `current/` — actively working
- `retired/` — completed (retire date added to filename: `jjh_b251108-r251126-buk-portability.md`)

A heat must be timely. If work is well-specified but not timely, it remains an itch until the time is right.

### Pace
A discrete action within the current heat. Appears in heat documents as structured sections. Pending paces can have detailed descriptions. Completed paces get condensed to brief summaries to save context.

Each pace has a **mode**:
- **Manual**: Human drives, model assists. Minimal spec needed.
- **Delegated**: Model drives from spec, human monitors. Requires clear objective, bounded scope, success criteria, and failure behavior.

Paces default to `manual` when created. Use `/jja-pace-refine` to prepare a pace for delegation or to clarify a manual pace.

### Itch
A potential future heat or consideration. Can range from a brief spark to a fully-articulated specification. The key attribute is **not now** — regardless of detail level, it's not timely for current work.

All itches live in a single aggregate file (`jji_itch.md`). No individual itch files. No itch sections in heat documents.

### Scar
An itch that has been **closed with lessons learned**. Not deleted (we learned something), but won't be revisited. Different from "shelved" which implies "maybe later" — a scar is deliberately closed.

All scars live in a single aggregate file (`jjs_scar.md`).

## How It Works

### Day-to-Day Usage

You work on a heat by talking with Claude Code. As you make progress:
- Claude uses `/jja-heat-resume` to show current heat and next pace, asking for clarification if needed
- You work on the pace together
- Claude uses `/jja-pace-wrap` to summarize and mark it complete
- New paces emerge and get added with `/jja-pace-add`

When new ideas come up that don't belong in current heat, Claude uses `/jja-itch-find` and `/jja-itch-move` to file them away.

When a heat completes, Claude uses `/jja-heat-retire` to move it to `retired/` with a datestamp.

### Interaction Pattern

The system is **conversational and collaborative**:
- Claude proposes actions ("I'll mark this pace done and summarize it as...")
- You approve or amend ("yes" / "change it to..." / "no, actually...")
- Changes commit to git automatically after approval
- You maintain control, Claude does the bookkeeping

### Context Management

The system is designed to minimize context usage:
- Completed paces become one-line summaries
- Only current heat is in regular context
- Itches and scars stay out of context unless needed
- Full history preserved in git, not in active documents

## File Structure

### `jjh_bYYMMDD-description.md` (Job Jockey Heat)
Main context document for a heat.
- **Active**: Named with begin date and description (e.g., `jjh_b251108-buk-portability.md`)
- **Retired**: Begin date preserved, retire date added (e.g., `jjh_b251108-r251126-buk-portability.md`)
- Located in: `.claude/jjm/current/` (active) or `.claude/jjm/retired/` (completed)

### `jji_itch.md` (Job Jockey Itches)
**All** itches live here — the single source of future work.
- Brief sparks or detailed specifications
- Items graduate from here to new heat files
- Located in: `.claude/jjm/`

### `jjs_scar.md` (Job Jockey Scars)
Closed itches with lessons learned.
- Not rejected, but deliberately closed
- Includes context on why closed and what was learned
- Located in: `.claude/jjm/`

## Directory Structure

JJ memory lives at `.claude/jjm/` relative to CLAUDE.md. Commands live at `.claude/commands/`.

```
my-project/                 # Launch Claude Code here
  CLAUDE.md
  .claude/
    commands/
      jja-heat-resume.md
      jja-heat-retire.md
      jja-pace-find.md
      jja-pace-left.md
      jja-pace-add.md
      jja-pace-refine.md
      jja-pace-delegate.md
      jja-pace-wrap.md
      jja-sync.md
      jja-itch-list.md
      jja-itch-find.md
      jja-itch-move.md
    jjm/
      jji_itch.md
      jjs_scar.md
      current/
        jjh_b251108-feature-x.md
      retired/
        jjh_b251001-r251015-feature-y.md
  src/
  ...
```

## Itch Format

All itches live in `jji_itch.md`. Each itch is a section with a descriptive header (no "Itch:" prefix — keeps entries clean for moving to scar file):

```markdown
# Itches

## governor-implementation
Create rbgp_create_governor for depot setup flow. Depends on understanding
the full depot lifecycle. Could be haiku-delegatable once spec is clear.

## image-retrieve-design
Design rbtgo_image_retrieve operation from scratch. No existing implementation
to extract from. Needs architectural decision on caching strategy.

## quick-idea
Brief spark about improving error messages.
```

When moving to scars, the section moves as-is with added closure context:

```markdown
# Scars

## governor-implementation
Create rbgp_create_governor for depot setup flow...

**Closed**: Superseded by rbgg_create_depot which handles governor internally.
Learned: governor is an implementation detail, not a user-facing concept.
```

## Workflows

### Starting a New Heat
1. Create `jjh_bYYMMDD-description.md` in `.claude/jjm/current/` (use today's date)
2. Include Context section with stable background information
3. Include Paces section with initial checklist items
4. Archive previous heat to `retired/` (if applicable)

### Selecting Current Heat
When starting a session or the user calls `/jja-heat-resume`, Claude checks `.claude/jjm/current/`:
- **0 heats**: No active work. Ask if user wants to start a new heat or promote an itch.
- **1 heat**: Show heat and current pace
- **2+ heats**: Ask user which heat to work on

### Working on a Heat
1. Use `/jja-heat-resume` to see current heat and current pace
2. Work on it conversationally with Claude
3. Use `/jja-pace-wrap` when complete
4. Use `/jja-heat-resume` again to see what's next
5. Repeat until heat is complete

### Completing a Heat
1. Verify all paces are complete or explicitly discarded
2. Use `/jja-heat-retire` to move and rename heat file:
   - Adds retire date (`rYYMMDD`) to filename, preserving begin date
   - Moves from `current/` → `retired/`
   - Commits the archival

### Itch Triage
When a new idea emerges:
1. **Does it block current heat completion?** → Add as pace to current heat
2. **Is it future work worth capturing?** → Add to `jji_itch.md`
3. **Is it something we're deliberately closing?** → Add to `jjs_scar.md` with reason

## Format Conventions

- **All documents**: Markdown (`.md`)
- **Dates**: YYMMDD format (e.g., 251108 for 2025-11-08)
  - `b` prefix = begin date (when heat started)
  - `r` prefix = retire date (when heat completed)
- **Descriptions**: Lowercase with hyphens (e.g., `buk-portability`)
- **Pace titles**: Bold (e.g., `**Audit BUK portability**`)
- **Completed summaries**: Brief, factual, no line numbers (they go stale)

## Heat Document Structure

Heat files contain these sections:

```markdown
# Heat: [Name]

## Context
[Stable background info. Can grow as insights emerge during heat work.]

## Done
1. First completed pace title
2. Second completed pace title
...

## Current
**Current pace title**
[Working notes for this pace only, if needed]

## Remaining
- Future pace title
- Another future pace
...
```

### Section Details

**Context**: Stable information that grows as architectural insights emerge. Goals, constraints, decisions, background.

**Done**: Numbered list of completed pace titles only. Number = completion order (useful for commit references). No verbose summaries - git commits carry that detail.

**Current**: The one pace being worked. May include working notes. Gets numbered and moved to Done when complete.

**Remaining**: Unnumbered queue of future paces. Order can change freely. First item becomes Current when current pace completes.

## Design Principles

1. **Ephemeral by design**: Documents have clear lifecycles, completed work gets archived
2. **Conversational**: Claude proposes, you approve or amend, Claude executes
3. **Context-conscious**: Minimize active context, maximize git history
4. **Model-primary**: Claude reads/writes frequently, human adjusts occasionally
5. **Clear naming**: Prefixes make purpose immediately obvious
6. **Git-friendly**: Preserve history, commit after approval (one commit per action)
7. **Minimal ceremony**: Easy to use, hard to misuse
8. **Aggregate itches**: All itches in one file, all scars in one file — no sprawl
9. **Portable**: Works across computers via relative paths
10. **Do No Harm**: If paths are misconfigured or files missing, announce issue and stop — don't guess or auto-fix

## Installation

Job Jockey is installed via the workbench script:

```bash
# Install
./Tools/jjk/jjw_workbench.sh jjk-i

# Uninstall (preserves .claude/jjm/ state)
./Tools/jjk/jjw_workbench.sh jjk-u
```

The workbench:
- Creates `.claude/commands/jja-*.md` command files
- Creates `.claude/jjm/` directory structure
- Patches CLAUDE.md with configuration section

Configuration is via environment variables:
- `ZJJW_TARGET_DIR` - Target repo directory (default: `.`)
- `ZJJW_KIT_PATH` - Path to this kit file (default: `Tools/jjk/README.md`)

**Important**: Restart Claude Code after installation for new commands to become available.

## Available Commands

- `/jja-heat-resume` - Resume current heat, show current pace
- `/jja-heat-retire` - Move completed heat to retired with datestamp
- `/jja-pace-find` - Show current pace (with mode)
- `/jja-pace-left` - List all remaining paces (with mode)
- `/jja-pace-add` - Add a new pace (defaults to manual)
- `/jja-pace-refine` - Refine pace spec, set mode (manual or delegated)
- `/jja-pace-delegate` - Execute a delegated pace
- `/jja-pace-wrap` - Mark pace complete
- `/jja-sync` - Commit and push JJ state and target repo
- `/jja-itch-list` - List all itches and scars
- `/jja-itch-find` - Find an itch by keyword
- `/jja-itch-move` - Move itch to scar or promote to heat

## Future Directions

### Heat Action Logs
Current doctrine keeps only short summaries in the Done section. However, valuable detail is lost that could improve Job Jockey itself. Consider:
- **jjl_bYYMMDD-description.log** - Detailed action log per heat
- Captures: delegated pace specs, execution traces, failure modes, recovery attempts
- Lives alongside heat file, retired together
- Enables retrospective analysis: "what worked, what didn't, what should change"

### Specialized Agents
Create purpose-built agents for delegation, not just model hints:
- **Model-tier agents**: haiku-worker, sonnet-worker, opus-worker with appropriate context budgets
- **Pace-type agents**: mechanical-edit, codebase-explore, test-runner, doc-writer
- **Delegation router**: analyzes pace spec, selects optimal agent, handles handoff
- Success criteria: right agent for right task, minimal token waste, clear failure escalation

### Skill Articulation
Before delegation can succeed, skills must be identified and well-described:
- **Skill inventory**: catalog of capabilities available for delegation (edit, search, test, generate, validate, etc.)
- **Skill cards**: each skill has preconditions, inputs, outputs, failure modes, model requirements
- **Heat planning**: match heat goals to available skills, identify gaps early
- **Pace preparation**: `/jja-pace-refine` could suggest "this pace needs skills X, Y" and verify they exist
- **Skill gaps**: surface when a pace requires a skill not yet articulated → triggers skill development

### Delegation Intelligence
Improve the refine→delegate flow:
- Learn from action logs which pace patterns succeed/fail per agent type
- Auto-suggest agent selection based on pace characteristics
- Detect scope creep or unbounded work before it spins
- Graceful escalation: haiku fails → sonnet retry → opus rescue → human
- Match pace requirements to skill inventory before attempting delegation

---

*Command implementations live in the workbench. This document is the conceptual reference.*
