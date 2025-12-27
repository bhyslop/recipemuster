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
| `jjc_` | Chase | (Future) Steeplechase performance logs |
| `jja_` | Action | Slash commands |
| `jjk_` | sKill | (Future) Skill definitions |
| `jjg_` | aGent | (Future) Agent definitions |

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

Paces default to `manual` when created. Refinement happens naturally in the workflow: when transitioning to a new pace, Claude analyzes it and proposes an approach. For delegation, say "delegate this" and Claude will formalize the spec.

### Itch
A potential future heat or consideration. Can range from a brief spark to a fully-articulated specification. The key attribute is **not now** — regardless of detail level, it's not timely for current work.

All itches live in a single aggregate file (`jji_itch.md`). No individual itch files. No itch sections in heat documents.

### Scar
An itch that has been **closed with lessons learned**. Not deleted (we learned something), but won't be revisited. Different from "shelved" which implies "maybe later" — a scar is deliberately closed.

All scars live in a single aggregate file (`jjs_scar.md`).

## How It Works

### Day-to-Day Usage

You work on a heat by talking with Claude Code. As you make progress:
- At session start, use `/jja-heat-resume` to establish context and see proposed approach
- Work on the pace together
- Use `/jja-pace-wrap` to mark complete - Claude automatically analyzes next pace and proposes approach
- Use `/jja-sync` to commit/push - Claude then proposes approach for current pace
- New paces emerge and get added with `/jja-pace-add`

Note: After pace-wrap or sync, you do NOT need heat-resume - those commands flow directly into the next pace.

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

### `jjc_bYYMMDD-description.md` (Job Jockey Steeplechase)
Performance log capturing how each heat actually ran.
- Created lazily when first entry is logged
- Naming matches the heat file (same begin date and silks)
- Located in: `.claude/jjm/current/` during active work
- On retirement: contents merged into heat file under `## Steeplechase` section

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
        jjc_b251108-feature-x.md
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

**Closed**: Implemented as rbgp_create_governor in Payor module.
Learned: Governor creation is a Payor operation since Payor owns depot lifecycle.
```

## Steeplechase Format

The steeplechase file (`jjc_bYYMMDD-description.md`) captures three types of entries:

### APPROACH Entry
Logged when analyzing a pace and proposing how to approach it:
```markdown
---
### 2025-12-25 14:30 - specify-image-delete - APPROACH
**Mode**: manual
**Proposed approach**:
- Read rbf_delete implementation to extract step sequence
- Apply completeness criteria from RBAGS pattern
- Document in same format as rbtgo_director_create
---
```

### WRAP Entry
Logged when marking a pace complete:
```markdown
---
### 2025-12-25 16:45 - specify-image-delete - WRAP
**Mode**: manual
**Outcome**: Documented rbtgo_image_delete with 5-step sequence extracted from rbf_delete
---
```

### DELEGATE Entry
Logged when executing a delegated pace:
```markdown
---
### 2025-12-25 15:00 - update-config-refs - DELEGATE
**Spec**:
- Objective: Update all config references from old to new format
- Scope: src/config/*.ts files only
- Success: All references updated, tests pass
- On failure: Report files that couldn't be updated

**Execution trace**:
- Read 12 config files
- Modified 8 files with reference updates
- Ran test suite

**Result**: success
Updated 23 references across 8 files, tests passing.

**Modified files**:
- /Users/name/project/src/config/auth.ts
- /Users/name/project/src/config/api.ts
---
```

On heat retirement, the entire steeplechase is appended to the heat file under a `## Steeplechase` section, creating a complete archive of how the heat ran.

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
1. Use `/jja-heat-resume` at session start - Claude shows context and proposes approach
2. Approve approach or adjust, then work on the pace
3. Use `/jja-pace-wrap` when complete - Claude analyzes next pace and proposes approach
4. Approve and continue (no need for heat-resume between paces)
5. Use `/jja-sync` periodically - also proposes approach for current pace
6. Repeat until heat is complete

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

## Remaining
- **Current pace title** ← First item is implicitly current (bold to highlight)
  [Working notes for this pace only, if needed]
- Next pace title
- Another future pace
...
```

### Section Details

**Context**: Stable information that grows as architectural insights emerge. Goals, constraints, decisions, background.

**Done**: Numbered list of completed pace titles only. Number = completion order (useful for commit references). No verbose summaries - git commits carry that detail.

**Remaining**: Unnumbered queue of paces. **First item is implicitly current** (bold it to highlight, may include working notes). Rest are future paces in priority order. Order can change freely. When current pace completes, move it to Done and first remaining item becomes current.

**Design rationale**: Eliminates dedicated Current section to reduce document thrash. As paces move from Remaining → Done, fewer section movements mean cleaner diffs and less context distraction.

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

- `/jja-heat-resume` - Resume heat at session start, analyze pace, propose approach
- `/jja-heat-retire` - Move completed heat to retired with datestamp
- `/jja-pace-find` - Show current pace (with mode)
- `/jja-pace-left` - List all remaining paces (with mode)
- `/jja-pace-add` - Add a new pace (defaults to manual)
- `/jja-pace-delegate` - Execute a delegated pace
- `/jja-pace-wrap` - Mark pace complete, analyze next pace, propose approach
- `/jja-sync` - Commit and push, then analyze current pace, propose approach
- `/jja-itch-list` - List all itches and scars
- `/jja-itch-find` - Find an itch by keyword
- `/jja-itch-move` - Move itch to scar or promote to heat

## Terminology

### Silks
The kebab-case identifier that uniquely names JJ artifacts:
- **Silks** are the unique names for itches, scars, heats, and steeplechases (e.g., `governor-implementation`, `buk-portability`)
- Every itch has silks; silks carry to scars when closed
- Heats have silks (the description part of filename)
- Steeplechases inherit the heat's silks
- Usage: "What's the silks on that itch?" / "The heat silks are `rbags-specification`"

## Future Directions

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
- **Pace analysis**: when proposing approach, suggest "this pace needs skills X, Y" and verify they exist
- **Skill gaps**: surface when a pace requires a skill not yet articulated → triggers skill development

### Delegation Intelligence
Improve the analyze→delegate flow:
- Learn from action logs which pace patterns succeed/fail per agent type
- Auto-suggest agent selection based on pace characteristics
- Detect scope creep or unbounded work before it spins
- Graceful escalation: haiku fails → sonnet retry → opus rescue → human
- Match pace requirements to skill inventory before attempting delegation

### Instance Versioning via Content Addressing
Add a deterministic version designation for JJ installations to connect retrospective study to improvement:
- **Approach**: Compute shasum (SHA-256) of JJ kit files: `jjw_workbench.sh` + `README.md`
- **Installation process**: During `jjw_workbench.sh jjk-i`, compute content hash and generate brand version (e.g., `jj-a3f7d2e`)
- **Storage**: Write JJ brand to `.claude/jjm/jj_brand.txt` and optionally to CLAUDE.md JJ configuration
- **Embedding**: Bake brand into heat/steeplechase files and git commits (via `/pace-commit` and other JJ commands)
- **Benefits**:
  - Deterministic: same kit files always produce same version, no manual versioning needed
  - Lean context: short hash is easy to reference in prose and commit messages
  - Traceable: enables retrospective filtering ("heats using jj-a3f7d2e"), correlates code changes to JJ design iterations
  - Self-documenting: hash is cryptographic proof of which exact JJ version produced the pattern
- **Enables**: "these heats used JJ version a3f7d2e, these used b8c2f1a, compare outcomes and identify design improvements"
- **Fallback**: If hash computation unavailable, fall back to timestamp-based version (e.g., `jj-2512271430`)

### Heat Document Efficiency
Reduce thrash in heat files during active work:
- Current structure causes frequent moves between Done/Current/Remaining sections
- Consider: more stable pace representation that reduces edits
- Investigate: what's the minimum mutable surface for tracking progress?
- Goal: cleaner diffs, less context churn, easier retrospectives

### Formal Pace Numbering
Remove pace numbers from human-visible artifacts:
- Numbers appearing in heat files/code are brittle and go stale
- Pace ordering is implicit in document position (first unnumbered item = current/next)
- Recommendation: assign stable pace IDs on creation (e.g., `p001`, `p002`) for internal tracking only
- **Heat template revision**: Remove all `1.`, `2.`, etc. prefixes from Remaining section; keep paces as unnumbered list
- **Steeplechase entries**: Reference pace by ID+silks (e.g., `p001 - setup-config`) instead of numbers
- Keep numbers internal to JJ machinery, never in prose or human-facing docs

### Silk Design Guidance
Make silks short and memorable for human cognition:
- **Silks**: Kebab-case identifiers for heats, paces, itches, scars (e.g., `cloud-foundation-stabilize`, `fix-unbound-variable`)
- **Target**: 2-4 words, under 30 characters, easy to recall and type
- **Rationale**: Silks appear frequently in speech, commit messages, and steeplechase entries. Short + catchy reduces cognitive load and typos.
- **Anti-patterns**: Avoid long descriptive names, avoid acronyms unless widely recognized in project, avoid generic names (e.g., `misc-fixes`, `stuff`)
- **Mnemonic quality**: Good silks create mental hooks (e.g., `image-registry-listing` immediately evokes the feature; `gad-perf-analysis` links to GAD tool)
- **Workshop**: When creating a new heat/itch/pace, generate 3-5 candidate silks and pick the one that "sticks" best

### Git Commit Integration
Enrich heat-related commits with structured metadata:
- Include heat silks and pace silks in commit messages
- Consider: steeplechase entries could live in extended commit messages
- Add intervention level indicator (manual heavy / manual light / delegated)
- Format: `[heat:silks][pace:silks][mode:manual|delegated]` prefix
- Enables: filter git log by heat, reconstruct execution timeline

### Dedicated Commit Subagent
Create a specialized subagent for JJ-initiated git commits:
- **Purpose**: Execute commits with JJ-aware system prompt that injects heat/pace context without Claude Code self-promotion
- **System prompt**: Include heat silks, pace silks, and context; omit standard "Generated with Claude Code" footer
- **Invocation**: JJ commands (wrap, sync) delegate to commit-subagent instead of using standard Bash tool
- **Benefits**: Cleaner commit messages focused on work content, not tool attribution; consistent JJ metadata in git history
- **Configuration**: Project-level setting in CLAUDE.md: `jj_commit_subagent: enabled | disabled` (default: enabled if available)
- **Fallback**: If subagent unavailable, use standard Bash commits with manual formatting

### Pace-Level Commits with /pace-commit
Create a new slash command for committing work-in-progress within a pace:
- **Purpose**: Allow commits during pace work (not just at pace completion) with full JJ context
- **Command**: `/pace-commit` delegates to commit-subagent with:
  - Heat silks (e.g., `cloud-foundation-stabilize`)
  - Pace silks (e.g., `fix-unbound-variable`)
  - Job Jockey brand (JJ installation version/identity) as metadata
  - Structured commit format: `[jj:brand][heat:silks][pace:silks] Commit message`
- **Job Jockey Brand**: Version/identity of JJ installation (e.g., `jj-v1` or timestamp-based identifier like `jj-2512271430`)
  - Enables retrospective analysis: "which JJ version produced this pattern?"
  - Links commits back to JJ design iteration and heat execution metadata
  - Stored in `.claude/jjm/jj_brand.txt` or CLAUDE.md JJ configuration
- **Benefits**: Granular commit history within paces, cleaner git log filtering by JJ context, ability to study which JJ versions succeeded/failed
- **Example**: `[jj:v1][heat:cloud-foundation-stabilize][pace:fix-unbound-variable] Remove unset variable in rbgm line 102`

### Configurable Autocommit
Project-level control over automatic git commits:
- Some projects want commits per pace wrap
- Some want manual commit control
- Some want no JJ-initiated commits at all
- Configuration in CLAUDE.md JJ section: `autocommit: per-pace | per-sync | never`
- Default behavior should match current (commits on wrap/sync)

---

*Command implementations live in the workbench. This document is the conceptual reference.*
