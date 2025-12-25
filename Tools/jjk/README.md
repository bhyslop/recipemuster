# Job Jockey Kit

## What is Job Jockey?

Job Jockey (JJ) is a lightweight system for managing project initiatives through conversation with Claude Code. It helps you track bounded heats, remember what's next, and keep a backlog of ideas without drowning in ceremony or context bloat.

Think of it as a project notebook specifically designed for human-AI collaboration:
- **Heats** are your current work (3-50 chat sessions worth)
- **Paces** track what's done and what's next within a heat
- **Itches** capture future ideas without losing focus

The system is ephemeral by design: documents have clear lifecycles, completed work gets archived, and context stays lean. Everything is markdown, lives in git, and can move between computers with you.

## Core Concepts

### Heat
A bounded initiative spanning 3-50 chat sessions. Has a clear goal, context section, and list of paces. Lives as a dated file like `jjh-b251108-buk-portability.md`.

Heat location indicates state:
- `current/` — actively working
- `pending/` — detailed but parked (blocked or deferred)
- `retired/` — completed (retire date added to filename: `jjh-b251108-r251126-buk-portability.md`)

Move to `current/` via prose when ready to work. Park in `pending/` via prose when blocked or deferring.

### Pace
A discrete action within the current heat. Appears as checklist items in heat documents. Pending paces can have detailed descriptions. Completed paces get condensed to brief summaries to save context.

Each pace has a **mode**:
- **Manual**: Human drives, model assists. Minimal spec needed.
- **Delegated**: Model drives from spec, human monitors. Requires clear objective, bounded scope, success criteria, and failure behavior.

Paces default to `manual` when created. Use `/jja-pace-refine` to prepare a pace for delegation or to clarify a manual pace.

### Itch
A potential future heat or consideration. The spark/urge that might become a heat someday. Lives in either Future (worthy of doing) or Shelved (respectfully set aside for now).

## How It Works

### Day-to-Day Usage

You work on a heat by talking with Claude Code. As you make progress:
- Claude uses `/jja-heat-resume` to show current heat and next pace, asking for clarification if needed
- You work on the pace together
- Claude uses `/jja-pace-wrap` to summarize and mark it complete
- New paces emerge and get added with `/jja-pace-add`

When new ideas come up that don't belong in current heat, Claude uses `/jja-itch-find` and `/jja-itch-move` to file them away in Future or Shelved.

When a heat completes, Claude uses `/jja-heat-retire` to move it to `retired/` with a datestamp and start a new one.

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
- Future/Shelved itches stay out of context unless needed
- Full history preserved in git, not in active documents

## File Structure

All Job Jockey documents use the `jj` prefix with category-specific third letters:

### `jjh-bYYMMDD-description.md` and `jjh-bYYMMDD-rYYMMDD-description.md` (Job Jockey Heat)
Main context document for a heat.
- **Active**: Named with begin date and description (e.g., `jjh-b251108-buk-portability.md`)
- **Retired**: Begin date preserved, retire date added (e.g., `jjh-b251108-r251126-buk-portability.md`)
- Located in: `.claude/jji/current/` (active), `.claude/jji/pending/` (parked), or `.claude/jji/retired/` (completed)

### `jjf-future.md` (Job Jockey Future)
Itches for worthy future heats.
- Items graduate from here to new `jjh-` files
- Located in: `.claude/jji/`

### `jjs-shelved.md` (Job Jockey Shelved)
Itches respectfully set aside.
- Not rejected, but deferred for foreseeable future
- May include brief context on why shelved
- Located in: `.claude/jji/`

## Directory Structure

JJ files always live at `.claude/jji/` relative to CLAUDE.md. Commands live at `.claude/commands/`.

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
    jji/
      jjf-future.md
      jjs-shelved.md
      current/
        jjh-b251108-feature-x.md
      pending/
        jjh-b251101-blocked-work.md
      retired/
        jjh-b251001-r251015-feature-y.md
  src/
  ...
```

## Workflows

### Starting a New Heat
1. Create `jjh-bYYMMDD-description.md` in `.claude/jji/current/` (use today's date)
2. Include Context section with stable background information
3. Include Paces section with initial checklist items
4. Archive previous heat to `retired/` (if applicable)

### Selecting Current Heat
When starting a session or the user calls `/jja-heat-resume`, Claude checks `.claude/jji/current/`:
- **0 heats**: No active work. If pending heats exist, mention them. Ask if user wants to start a new heat, activate a pending heat, or promote an itch.
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
When a new itch emerges:
1. **Does it block current heat completion?** → Add as pace to current heat
2. **Is it worthy but not now?** → Add to `jjf-future.md`
3. **Interesting but setting aside?** → Add to `jjs-shelved.md`

## Format Conventions

- **All documents**: Markdown (`.md`)
- **Paces**: Checklist format with `- [ ]` and `- [x]`
- **Dates**: YYMMDD format (e.g., 251108 for 2025-11-08)
  - `b` prefix = begin date (when heat started)
  - `r` prefix = retire date (when heat completed)
- **Descriptions**: Lowercase with hyphens (e.g., `buk-portability`)
- **Pace titles**: Bold (e.g., `**Audit BUK portability**`)
- **Completed summaries**: Brief, factual (e.g., `Found 12 issues, documented in notes.md`)

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

## Itches
- itch-name: Brief description
```

### Section Details

**Context**: Stable information that grows as architectural insights emerge. Goals, constraints, decisions, background.

**Done**: Numbered list of completed pace titles only. Number = completion order (useful for commit references). No verbose summaries - git commits carry that detail.

**Current**: The one pace being worked. May include working notes. Gets numbered and moved to Done when complete.

**Remaining**: Unnumbered queue of future paces. Order can change freely. First item becomes Current when current pace completes.

**Itches**: Future work spawned during the heat. May become new heats later.

## Design Principles

1. **Ephemeral by design**: Documents have clear lifecycles, completed work gets archived
2. **Conversational**: Claude proposes, you approve or amend, Claude executes
3. **Context-conscious**: Minimize active context, maximize git history
4. **Model-primary**: Claude reads/writes frequently, human adjusts occasionally
5. **Clear naming**: Prefixes make purpose immediately obvious
6. **Git-friendly**: Preserve history, commit after approval (one commit per action)
7. **Minimal ceremony**: Easy to use, hard to misuse
8. **Respectful**: Itches are "shelved" not "rejected"
9. **Portable**: Works across computers via relative paths
10. **Do No Harm**: If paths are misconfigured or files missing, announce issue and stop - don't guess or auto-fix

## Installation

Job Jockey is installed via the workbench script:

```bash
# Install
./Tools/jjk/jjw_workbench.sh jjk-i

# Uninstall (preserves .claude/jji/ state)
./Tools/jjk/jjw_workbench.sh jjk-u
```

The workbench:
- Creates `.claude/commands/jja-*.md` command files
- Creates `.claude/jji/` directory structure
- Patches CLAUDE.md with configuration section

Configuration is via environment variables:
- `ZJJW_TARGET_DIR` - Target repo directory (default: `.`)
- `ZJJW_KIT_PATH` - Path to this kit file (default: `Tools/jjk/job-jockey-kit.md`)

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
- `/jja-itch-list` - List all itches (future and shelved)
- `/jja-itch-find` - Find an itch by keyword
- `/jja-itch-move` - Move or promote an itch

---

*Command implementations live in the workbench. This document is the conceptual reference.*
