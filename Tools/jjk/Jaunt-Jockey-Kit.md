# Jaunt Jockey Kit

## What is Jaunt Jockey?

Jaunt Jockey (JJ) is a lightweight system for managing project initiatives through conversation with Claude Code. It helps you track bounded efforts, remember what's next, and keep a backlog of ideas without drowning in ceremony or context bloat.

Think of it as a project notebook specifically designed for human-AI collaboration:
- **Efforts** are your current work (3-50 chat sessions worth)
- **Steps** track what's done and what's next within an effort
- **Itches** capture future ideas without losing focus

The system is ephemeral by design: documents have clear lifecycles, completed work gets archived, and context stays lean. Everything is markdown, lives in git, and can move between computers with you.

This document (the Jaunt Jockey Kit) is the complete reference and installer for the system.

## Installation Variables

During installation, Claude replaces these markers in the generated command files:

- `«JJC_FILESYSTEM_RELATIVE_PATH»` → Your chosen path for JJ files (relative to CLAUDE.md)
  - Example: `.claude/jji/` (co-located in same repo)
  - Example: `../project-admin/.claude/jji/` (separate admin repo)
- `«JJC_SEPARATE_REPO»` → `yes` or `no` (determines git command structure)
- `«JJC_KIT_PATH»` → Path to this Kit file (for /jja-doctor validation)

These markers appear throughout this document in templates and will be hardcoded with actual values during installation. You never need to type the guillemets (« ») yourself.

## Core Concepts

### Effort
A bounded initiative spanning 3-50 chat sessions. Has a clear goal, context section, and list of steps. Lives as a dated file like `jje-251108-buk-portability.md`.

### Step
A discrete action within the current effort. Appears as checklist items in effort documents. Pending steps can have detailed descriptions. Completed steps get condensed to brief summaries to save context.

### Itch
A potential future effort or consideration. The spark/urge that might become an effort someday. Lives in either Future (worthy of doing) or Shelved (respectfully set aside for now).

## How It Works

### Day-to-Day Usage

You work on an effort by talking with Claude Code. As you make progress:
- Claude uses `/jja-step-find` to remind you what's next
- You work on the step together
- Claude uses `/jja-step-done` to summarize and mark it complete
- New steps emerge and get added with `/jja-step-add`

When new ideas come up that don't belong in current effort, Claude uses `/jja-itch-locate` and `/jja-itch-move` to file them away in Future or Shelved.

When an effort completes, its file moves to `retired/` and you start a new one.

### Interaction Pattern

The system is **conversational and collaborative**:
- Claude proposes actions ("I'll mark this step done and summarize it as...")
- You approve or amend ("yes" / "change it to..." / "no, actually...")
- Changes commit to git automatically after approval
- You maintain control, Claude does the bookkeeping

### Context Management

The system is designed to minimize context usage:
- Completed steps become one-line summaries
- Only current effort is in regular context
- Future/Shelved itches stay out of context unless needed
- Full history preserved in git, not in active documents

### Announcing JJ Availability

When appropriate (session start, effort selection, user mentions next steps), Claude announces:
- The current effort being worked on
- "See /jja- commands for Jaunt Jockey services"

This reminds the user of available tooling without being intrusive.

## File Structure

All Jaunt Jockey documents use the `jj` prefix with category-specific third letters:

### `jje-YYMMDD-description.md` (Jaunt Jockey Effort)
Main context document for an effort.
- Named with creation date and brief description
- Example: `jje-251108-buk-portability.md`
- Lifecycle: Current → Retired (moved to `retired/`)
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/current/` (active) or `«JJC_FILESYSTEM_RELATIVE_PATH»/retired/` (completed)
- Contains context section and steps

### `jjf-future.md` (Jaunt Jockey Future)
Itches for worthy future efforts.
- Items graduate from here to new `jje-` files
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/`

### `jjs-shelved.md` (Jaunt Jockey Shelved)
Itches respectfully set aside.
- Not rejected, but deferred for foreseeable future
- May include brief context on why shelved
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/`

### `Jaunt-Jockey-Kit.md` (this document)
The complete reference and installer. Defines structure, naming, and conventions.
- Used during installation
- Referenced by `/jja-doctor` for validation
- Location tracked in CLAUDE.md configuration

## Directory Structure

Typical installation uses `.claude/jji/` subdirectory:

```
.claude/
  jji/                    # Jaunt Jockey Installation directory
    jjf-future.md         # Future effort itches
    jjs-shelved.md        # Shelved itches
    current/
      jje-251108-buk-portability.md      # Current effort
      jje-251023-gad-implementation.md   # Another current effort
    retired/
      jje-251015-regime-management.md    # Completed effort
```

And in the CLAUDE.md repo:
```
.claude/
  commands/
    jja-step-find.md
    jja-step-left.md
    jja-step-add.md
    jja-step-done.md
    jja-itch-locate.md
    jja-itch-move.md
    jja-doctor.md
```

## Workflows

### Starting a New Effort
1. Create `jje-YYMMDD-description.md` in `«JJC_FILESYSTEM_RELATIVE_PATH»/current/`
2. Include Context section with stable background information
3. Include Steps section with initial checklist items
4. Archive previous effort to `retired/` (if applicable)

### Selecting Current Effort
When starting a session, Claude checks `«JJC_FILESYSTEM_RELATIVE_PATH»/current/`:
- **0 efforts**: No active work, ask if user wants to start one or promote an itch
- **1 effort**: Announce presumption ("Working on [effort name], see /jja- commands for Jaunt Jockey services")
- **2+ efforts**: Ask user which effort to work on, then announce selection

### Working on an Effort
1. Use `/jja-step-find` to see next step
2. Work on it conversationally with Claude
3. Use `/jja-step-done` when complete (Claude summarizes)
4. Repeat

### Completing an Effort
1. Verify all steps are complete or explicitly discarded
2. Use `git mv` to move effort file to `«JJC_FILESYSTEM_RELATIVE_PATH»/retired/`
3. Commit the archival

### Itch Triage
When a new itch emerges:
1. **Does it block current effort completion?** → Add as step to current effort
2. **Is it worthy but not now?** → Add to `jjf-future.md`
3. **Interesting but setting aside?** → Add to `jjs-shelved.md`

Use `/jja-itch-locate` to search for similar itches before adding.
Use `/jja-itch-move` to promote, demote, or shelve itches.

## Format Conventions

- **All documents**: Markdown (`.md`)
- **Steps**: Checklist format with `- [ ]` and `- [x]`
- **Dates**: YYMMDD format (e.g., 251108 for 2025-11-08)
- **Descriptions**: Lowercase with hyphens (e.g., `buk-portability`)
- **Step titles**: Bold (e.g., `**Audit BUK portability**`)
- **Completed summaries**: Brief, factual (e.g., `Found 12 issues, documented in notes.md`)

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

## Actions

Jaunt Jockey Actions (JJA) are Claude Code commands for managing the system.

### Itch Actions

#### `/jja-itch-locate`
Find an itch by keyword or fuzzy match across both `jjf-future.md` and `jjs-shelved.md`.

**Usage**: User provides search term, Claude searches both files and reports matches with context.

#### `/jja-itch-move`
Move an itch between future, shelved, or promote to a new effort.

**Usage**: After locating an itch, move it to:
- `jjf-future.md` (worthy of doing)
- `jjs-shelved.md` (setting aside)
- New `jje-YYMMDD-*.md` file (promoting to effort)

### Step Actions

#### `/jja-step-find`
Show the next incomplete step from the current effort.

**Behavior**: Displays the title and description of the first unchecked step.

#### `/jja-step-left`
Show terse list of all remaining steps in the current effort.

**Output format**:
```
Remaining steps (3):
1. Audit BUK portability
2. Create test harness
3. Document migration guide
```

#### `/jja-step-add`
Add a new step to the current effort with intelligent positioning.

**Behavior**:
- Claude analyzes the effort context and existing steps
- Proposes a new step with title, optional description, and position
- Explains reasoning for the placement
- Waits for user approval or amendment before updating file

**Example**:
```
I propose adding step '**Test BCU fixes**' after 'Audit BUK portability'
because we'll need to validate each fix before moving to BDU.
Should I add it there?
```

#### `/jja-step-done`
Mark a step as complete with automatic summarization.

**Behavior**:
- Claude summarizes the step based on current chat context
- Updates the effort file, moving step to Completed section with summary
- Reports what was written
- User can approve or request amendments

**Example output**:
```
Updated step 'Audit BUK portability' →
'Found 12 issues: 8 in BCU, 3 in BDU, 1 in BTU. Documented in portability-notes.md'
```

### Effort Document Structure

Effort files (`jje-YYMMDD-description.md`) contain two main sections:

#### Context Section
Stable information about the effort that only changes when explicitly updated by the user. Contains:
- Goals and objectives
- Key constraints
- Important decisions
- Background information
- Links to related resources

This section provides Claude with consistent context across sessions without needing to reread the entire chat history.

**Note**: Concrete examples of effort files will be added as the system is used and patterns emerge.

#### Steps Section
Divided into Pending and Completed subsections.

**Pending steps format**:
```markdown
### Pending
- [ ] **Step title in bold**
  Optional description with as much detail as needed.
  Can span multiple lines for complex steps.
  May include links, code snippets, or detailed requirements.
```

**Completed steps format**:
```markdown
### Completed
- [x] **Step title** - Concise summary of what was accomplished
- [x] **Another step** - Brief factual outcome
```

Completed steps are kept brief to minimize context usage. Full history is preserved in git.

## Installation

### Prerequisites
- A git repository with a `CLAUDE.md` file
- Claude Code access to the repository

### Bootstrap Process

1. **Place this file** in a location accessible to your CLAUDE.md
   - Can be in the same repo as CLAUDE.md (e.g., `Tools/jjk/Jaunt-Jockey-Kit.md`)
   - Can be in a separate admin/documentation repo

2. **Run the installation conversation** with Claude Code:
   - Open the repository containing CLAUDE.md
   - Say: "Read Jaunt-Jockey-Kit.md and let's install Jaunt Jockey"
   - Claude will ask configuration questions

3. **Configuration questions**:
   - **JJ files path**: Where should JJ files live, relative to CLAUDE.md?
     - Example: `.claude/jji/` (co-located, note the 'jji' subdirectory)
     - Example: `../project-admin/.claude/jji/` (separate repo)
     - Convention: Use `jji/` subdirectory to avoid confusion with other `jj*` patterns
   - **Separate repo**: Are JJ files in a different git repository? (yes/no)
   - **Kit path**: Claude will use the path where it found this file as the canonical location

4. **Claude will then**:
   - Add/update a `## Jaunt Jockey Configuration` section in CLAUDE.md
   - Create command files in `.claude/commands/jja-*.md` with hardcoded paths
     - All `{JJC_PATH}`, `{JJC_SEPARATE_REPO}`, `{JJC_KIT_PATH}` variables are replaced with actual values
     - Git commands include full paths and repo navigation if needed
     - Commit messages are fully specified per action
     - No runtime variable parsing required
   - Initialize JJ file structure at the configured path:
     - Create `jjf-future.md` (empty, if not exists)
     - Create `jjs-shelved.md` (empty, if not exists)
     - Create `current/` directory (if not exists)
     - Create `retired/` directory (if not exists)
   - Note any existing effort files found
   - Commit the changes

5. **Result**: CLAUDE.md will contain:
```markdown
## Jaunt Jockey Configuration
- JJ files path: `../project-admin/.claude/jji/`
- JJ Kit path: `Tools/jjk/Jaunt-Jockey-Kit.md`
- Separate repo: `yes`
- Installed: `2025-11-08`

Available commands:
- /jja-step-find - Show next incomplete step
- /jja-step-left - List all remaining steps
- /jja-step-add - Add a new step to current effort
- /jja-step-done - Mark step complete
- /jja-itch-locate - Find an itch by keyword
- /jja-itch-move - Move or promote an itch
- /jja-doctor - Validate Jaunt Jockey setup

**Note**: New commands require restarting your Claude Code session to become available.
```

### Reinstalling

To update configuration or reinstall commands:
1. Say: "Read Jaunt-Jockey-Kit.md and reinstall Jaunt Jockey"
2. Claude will update configuration and regenerate commands
3. Existing JJ content files (efforts, itches) remain untouched

### Validation

After installation, use `/jja-doctor` to verify:
- Kit file exists at configured path
- JJ files directory exists
- Expected files are present
- If separate repo, it's a valid git repository
- Commands exist and reference correct paths

## CLAUDE.md Integration

After installation, update CLAUDE.md to reference JJ for session context:

```markdown
## Session Context
- Check active efforts in «JJC_FILESYSTEM_RELATIVE_PATH»/current/ when starting relevant work
- Announce effort selection and mention /jja- commands
- Use /jja-step-find to see next step
- Use /jja-step-left for overview of remaining work

**Note**: Restart Claude Code session after installation for new commands to become available.
```

## Technical Implementation Notes

### Command Files
All JJA commands are markdown files in `.claude/commands/` that instruct Claude what to do.

**During installation**, the kit is used as a template to generate commands with:
- All `«JJC_FILESYSTEM_RELATIVE_PATH»` markers replaced with actual relative paths (e.g., `.claude/jji/` or `../project-admin/.claude/jji/`)
- All `«JJC_SEPARATE_REPO»` conditionals resolved to actual git command sequences
- All `«JJC_KIT_PATH»` references replaced with actual path to this kit
- Commit message patterns hardcoded per action (prefix: "JJA:")
- Each action commits separately after approval

**Result**: Commands are fully baked and ready to execute without any runtime interpretation. This keeps chat context focused on work, not system management.

### Git Behavior Examples
**When separate repo** (hardcoded during install):
```bash
cd ../project-admin
git add .claude/jji/current/jje-251108-buk-portability.md
git commit -m "JJA: step-done - Completed audit of BUK portability"
cd - > /dev/null
```

**When co-located** (hardcoded during install):
```bash
git add .claude/jji/current/jje-251108-buk-portability.md
git commit -m "JJA: step-done - Completed audit of BUK portability"
```

Each action specifies its own commit message pattern.

### Example Command Structure (Before Installation)

Template in this kit with variables:

```markdown
You are helping mark a step complete in the current Jaunt Jockey effort.

Configuration:
- JJ files path: «JJC_FILESYSTEM_RELATIVE_PATH»
- Separate repo: «JJC_SEPARATE_REPO»
- Kit path: «JJC_KIT_PATH»

Steps:
1. Ask which step to mark done (or infer from context)
2. Summarize the step completion based on chat context
3. Show proposed summary and ask for approval
4. Update the effort file in «JJC_FILESYSTEM_RELATIVE_PATH»/current/
   - Move step from Pending to Completed
   - Replace description with brief summary
5. Commit: "JJA: step-done - [brief description]"
6. Report what was done
```

### Example Command Structure (After Installation)

Generated `.claude/commands/jja-step-done.md` with hardcoded values:

```markdown
You are helping mark a step complete in the current Jaunt Jockey effort.

Configuration:
- JJ files path: ../project-admin/.claude/jji/
- Separate repo: yes
- Kit path: Tools/jjk/Jaunt-Jockey-Kit.md

Steps:
1. Ask which step to mark done (or infer from context)
2. Summarize the step completion based on chat context
3. Show proposed summary and ask for approval
4. Update the effort file in ../project-admin/.claude/jji/current/
   - Move step from Pending to Completed
   - Replace description with brief summary
5. Commit with:
   cd ../project-admin
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-done - [brief description]"
   cd - > /dev/null
6. Report what was done

Error handling: If files missing or paths wrong, announce issue and stop.
```

**Important**: After installation completes, restart your Claude Code session for the new commands to become available.

## Future Enhancements

- Additional JJA commands for effort lifecycle management
- Automated prompts for itch triage
- Cross-effort learning/pattern extraction
- Template generation for new efforts
- Effort switching support (if multiple active efforts needed)
- Integration with project-specific workflows

---

*This is a living document. Refine as the system evolves.*
