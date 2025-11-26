# Job Jockey Kit

## What is Job Jockey?

Job Jockey (JJ) is a lightweight system for managing project initiatives through conversation with Claude Code. It helps you track bounded efforts, remember what's next, and keep a backlog of ideas without drowning in ceremony or context bloat.

Think of it as a project notebook specifically designed for human-AI collaboration:
- **Efforts** are your current work (3-50 chat sessions worth)
- **Steps** track what's done and what's next within an effort
- **Itches** capture future ideas without losing focus

The system is ephemeral by design: documents have clear lifecycles, completed work gets archived, and context stays lean. Everything is markdown, lives in git, and can move between computers with you.

This document (the Job Jockey Kit) is the complete reference and installer for the system.

## Installation Variables

During installation, Claude replaces these markers in the generated command files:

- `«JJC_FILESYSTEM_RELATIVE_PATH»` → Your chosen path for JJ files (relative to CLAUDE.md)
  - Example: `.claude/jji/` (co-located in same repo)
  - Example: `../project-admin/.claude/jji/` (separate admin repo)
- `«JJC_SEPARATE_REPO»` → `yes` or `no` (determines git command structure)
- `«JJC_KIT_PATH»` → Path to this Kit file (for /jja-doctor validation)
- `«JJC_EFFORT_PREFIX»` → Effort file prefix (typically `jje-`)

These markers appear throughout this document in templates and will be hardcoded with actual values during installation. You never need to type the guillemets (« ») yourself.

## Core Concepts

### Effort
A bounded initiative spanning 3-50 chat sessions. Has a clear goal, context section, and list of steps. Lives as a dated file like `jje-b251108-buk-portability.md` (active) or `jje-b251108-r251126-buk-portability.md` (retired).

### Step
A discrete action within the current effort. Appears as checklist items in effort documents. Pending steps can have detailed descriptions. Completed steps get condensed to brief summaries to save context.

Each step has a **mode**:
- **Manual**: Human drives, model assists. Minimal spec needed.
- **Delegated**: Model drives from spec, human monitors. Requires clear objective, bounded scope, success criteria, and failure behavior.

Steps default to `manual` when created. Use `/jja-step-refine` to prepare a step for delegation or to clarify a manual step.

### Itch
A potential future effort or consideration. The spark/urge that might become an effort someday. Lives in either Future (worthy of doing) or Shelved (respectfully set aside for now).

## How It Works

### Day-to-Day Usage

You work on an effort by talking with Claude Code. As you make progress:
- Claude uses `/jja-effort-next` to show current effort and next step(s), asking for clarification if needed
- You work on the step together
- Claude uses `/jja-step-wrap` to summarize and mark it complete
- New steps emerge and get added with `/jja-step-add`

When new ideas come up that don't belong in current effort, Claude uses `/jja-itch-locate` and `/jja-itch-move` to file them away in Future or Shelved.

When an effort completes, Claude uses `/jja-effort-retire` to move it to `retired/` with a datestamp and start a new one.

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
- "See /jja- commands for Job Jockey services"

This reminds the user of available tooling without being intrusive.

## File Structure

All Job Jockey documents use the `jj` prefix with category-specific third letters:

### `jje-bYYMMDD-description.md` and `jje-bYYMMDD-rYYMMDD-description.md` (Job Jockey Effort)
Main context document for an effort.
- **Active**: Named with begin date and description (e.g., `jje-b251108-buk-portability.md`)
- **Retired**: Begin date preserved, retire date added (e.g., `jje-b251108-r251126-buk-portability.md`)
- Lifecycle: Active (`current/`) → Retired (`retired/` with r-date added)
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/current/` (active) or `«JJC_FILESYSTEM_RELATIVE_PATH»/retired/` (completed)
- Contains context section and steps

### `jjf-future.md` (Job Jockey Future)
Itches for worthy future efforts.
- Items graduate from here to new `jje-` files
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/`

### `jjs-shelved.md` (Job Jockey Shelved)
Itches respectfully set aside.
- Not rejected, but deferred for foreseeable future
- May include brief context on why shelved
- Located in: `«JJC_FILESYSTEM_RELATIVE_PATH»/`

### `job-jockey-kit.md` (this document)
The complete reference and installer. Defines structure, naming, and conventions.
- Used during installation
- Referenced by `/jja-doctor` for validation
- Location tracked in CLAUDE.md configuration

## Directory Structure

Typical installation uses `.claude/jji/` subdirectory:

```
.claude/
  jji/                    # Job Jockey Installation directory
    jjf-future.md         # Future effort itches
    jjs-shelved.md        # Shelved itches
    current/
      jje-b251108-buk-portability.md      # Active effort (began Nov 8)
      jje-b251023-gad-implementation.md   # Another active effort (began Oct 23)
    retired/
      jje-b251001-r251015-regime-management.md    # Completed effort (began Oct 1, retired Oct 15)
```

And in the CLAUDE.md repo:
```
.claude/
  commands/
    jja-effort-next.md
    jja-effort-retire.md
    jja-step-find.md
    jja-step-left.md
    jja-step-add.md
    jja-step-refine.md
    jja-step-delegate.md
    jja-step-wrap.md
    jja-itch-locate.md
    jja-itch-move.md
    jja-doctor.md
```

## Workflows

### Starting a New Effort
1. Create `jje-bYYMMDD-description.md` in `«JJC_FILESYSTEM_RELATIVE_PATH»/current/` (use today's date)
2. Include Context section with stable background information
3. Include Steps section with initial checklist items
4. Archive previous effort to `retired/` (if applicable)

### Selecting Current Effort
When starting a session or the user calls `/jja-effort-next`, Claude checks `«JJC_FILESYSTEM_RELATIVE_PATH»/current/`:
- **0 efforts**: No active work, ask if user wants to start one or promote an itch
- **1 effort**: Show effort and next step(s), ask for clarification if next step is unclear
- **2+ efforts**: Ask user which effort to work on, then show that effort with next step(s)

### Working on an Effort
1. Use `/jja-effort-next` to see current effort and next step(s)
2. Work on it conversationally with Claude
3. Use `/jja-step-wrap` when complete (Claude summarizes)
4. Use `/jja-effort-next` again to see what's next
5. Repeat until effort is complete

### Completing an Effort
1. Verify all steps are complete or explicitly discarded
2. Use `/jja-effort-retire` to move and rename effort file:
   - Adds retire date (`rYYMMDD`) to filename, preserving begin date
   - Moves from `current/jje-bYYMMDD-description.md` → `retired/jje-bYYMMDD-rYYMMDD-description.md`
   - Commits the archival

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
  - `b` prefix = begin date (when effort started)
  - `r` prefix = retire date (when effort completed)
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

Job Jockey Actions (JJA) are Claude Code commands for managing the system.

### Effort Actions

#### `/jja-effort-next`
Show the current effort and its next step(s), with optional clarification prompts.

**Behavior**:
- Checks `«JJC_FILESYSTEM_RELATIVE_PATH»/current/` for active efforts
- **0 efforts**: Announces no active work, asks if user wants to start an effort or promote an itch
- **1 effort**: Displays:
  - Effort name and brief gesture/summary
  - Next incomplete step with description
  - If multiple next steps or unclear priority: asks for clarification ("Which step should we focus on next?")
- **2+ efforts**: Asks user which effort to work on, then displays that effort with next step(s)

**Example output**:
```
Current effort: **BUK Utility Rename**
Next step: Update buc_command.sh internal functions
  Rename zbcu_* functions to zbuc_*
  (11 internal functions total)

Ready to start?
```

#### `/jja-effort-retire`
Move completed effort to retired directory with retire date added to filename.

**Behavior**:
- Verifies current effort exists in `current/`
- Checks that all steps are marked complete (or explicitly discarded)
- Adds retire date to filename: `jje-bYYMMDD-description.md` → `jje-bYYMMDD-rYYMMDD-description.md`
- Moves file to `«JJC_FILESYSTEM_RELATIVE_PATH»/retired/`
- Commits the retirement

**Example**:
- Before: `.claude/jji/current/jje-b251108-buk-rename.md`
- After: `.claude/jji/retired/jje-b251108-r251126-buk-rename.md`

### Itch Actions

#### `/jja-itch-locate`
Find an itch by keyword or fuzzy match across both `jjf-future.md` and `jjs-shelved.md`.

**Usage**: User provides search term, Claude searches both files and reports matches with context.

#### `/jja-itch-move`
Move an itch between future, shelved, or promote to a new effort.

**Usage**: After locating an itch, move it to:
- `jjf-future.md` (worthy of doing)
- `jjs-shelved.md` (setting aside)
- New `jje-*.md` file (promoting to effort, will get datestamp on retirement)

### Step Actions

#### `/jja-step-find`
Show the next incomplete step from the current effort.

**Behavior**: Displays the title, mode, and description of the first unchecked step.

#### `/jja-step-left`
Show terse list of all remaining steps in the current effort, with mode.

**Output format**:
```
Remaining steps (3):
1. [manual] Audit BUK portability
2. [manual] Create test harness
3. [delegated] Document migration guide
```

#### `/jja-step-add`
Add a new step to the current effort with intelligent positioning.

**Behavior**:
- Claude analyzes the effort context and existing steps
- Proposes a new step with title, optional description, and position
- New steps default to `mode: manual`
- Explains reasoning for the placement
- Waits for user approval or amendment before updating file

**Example**:
```
I propose adding step '**Test BCU fixes**' after 'Audit BUK portability'
because we'll need to validate each fix before moving to BDU.
Should I add it there?
```

#### `/jja-step-wrap`
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

#### `/jja-step-refine`
Refine a step's specification through adaptive interview. Can set or change step mode.

**Behavior**:
- Reads current step spec (may be sparse or already detailed)
- Conducts adaptive interview based on current state:
  - If sparse: builds spec from scratch
  - If exists: asks "what needs to change?" and focuses on delta
- Interview determines mode (`manual` or `delegated`)
- For `delegated` mode, ensures spec includes:
  - Clear objective
  - Bounded scope
  - Success criteria
  - Failure behavior
  - Model hint (haiku-ok / needs-sonnet / needs-opus)
- Final check for `delegated`: reads spec as fresh model would, verifies clarity
- Updates step in effort file with refined spec
- Can be run multiple times (iterative refinement)

**Final clarity check** (for delegated steps):
```
Reading this spec as a model with no prior context:
- Objective: ✓ clear / ✗ ambiguous because...
- Scope: ✓ bounded / ✗ unclear because...
- Success: ✓ measurable / ✗ vague because...
- Stuck: ✓ know when to stop / ✗ might spin because...
```

If any check fails, interview continues until spec passes.

#### `/jja-step-delegate`
Execute a delegated step. Validates health before proceeding.

**Behavior**:
- Verifies step mode is `delegated`
- Verifies spec passes health checks (objective, scope, success, failure defined)
- If unhealthy: refuses with specific guidance ("Run /jja-step-refine first")
- If healthy: presents step spec to model for execution
- Model executes from spec alone (no refinement context)
- On completion or failure: reports outcome

**Refusal cases**:
- Step is `manual`: "This step is manual - work on it conversationally"
- Step is `delegated` but unhealthy: "This step needs refinement - [specific gap]"

### Effort Document Structure

Effort files (`jje-bYYMMDD-description.md` when active, `jje-bYYMMDD-rYYMMDD-description.md` when retired) contain two main sections:

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
   - Can be in the same repo as CLAUDE.md (e.g., `Tools/jjk/job-jockey-kit.md`)
   - Can be in a separate admin/documentation repo

2. **Run the installation conversation** with Claude Code:
   - Open the repository containing CLAUDE.md
   - Say: "Read job-jockey-kit.md and let's install Job Jockey"
   - Claude will ask configuration questions

3. **Configuration questions**:
   - **JJ files path**: Where should JJ files live, relative to CLAUDE.md?
     - Example: `.claude/jji/` (co-located, note the 'jji' subdirectory)
     - Example: `../project-admin/.claude/jji/` (separate repo)
     - Convention: Use `jji/` subdirectory to avoid confusion with other `jj*` patterns
   - **Separate repo**: Are JJ files in a different git repository? (yes/no)
   - **Kit path**: Claude will use the path where it found this file as the canonical location

4. **Claude will then** (idempotent - safe to run multiple times):
   - Delete any existing `jja-*.md` command files from `.claude/commands/`
   - Generate all command files in `.claude/commands/jja-*.md` with hardcoded paths
     - All `«JJC_FILESYSTEM_RELATIVE_PATH»`, `«JJC_SEPARATE_REPO»`, `«JJC_KIT_PATH»` variables are replaced with actual values
     - Git commands include full paths and repo navigation if needed
     - Commit messages are fully specified per action
     - No runtime variable parsing required
   - Replace (not append) the `## Job Jockey Configuration` section in CLAUDE.md
   - Initialize JJ file structure at the configured path:
     - Create `jjf-future.md` (if not exists, preserve if exists)
     - Create `jjs-shelved.md` (if not exists, preserve if exists)
     - Create `current/` directory (if not exists)
     - Create `retired/` directory (if not exists)
   - Note any existing effort files found
   - Commit the changes

5. **Installation completes**. CLAUDE.md will contain:
```markdown
## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Effort**: Bounded initiative (3-50 sessions), has steps
- **Step**: Discrete action within an effort; mode is `manual` (human drives) or `delegated` (model drives from spec)
- **Itch**: Future idea, lives in Future or Shelved

- JJ files path: `../project-admin/.claude/jji/`
- JJ Kit path: `Tools/jjk/job-jockey-kit.md`
- Separate repo: `yes`
- Installed: `2025-11-08`

**Available commands:**
- `/jja-effort-next` - Show current effort and next step(s)
- `/jja-effort-retire` - Move completed effort to retired with datestamp
- `/jja-step-find` - Show next incomplete step (with mode)
- `/jja-step-left` - List all remaining steps (with mode)
- `/jja-step-add` - Add a new step (defaults to manual)
- `/jja-step-refine` - Refine step spec, set mode (manual or delegated)
- `/jja-step-delegate` - Execute a delegated step
- `/jja-step-wrap` - Mark step complete
- `/jja-itch-locate` - Find an itch by keyword
- `/jja-itch-move` - Move or promote an itch
- `/jja-doctor` - Validate Job Jockey setup

**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available.
```

### Validation

After installation completes and you restart your Claude Code session, you can use `/jja-doctor` to verify:
- Kit file exists at configured path
- JJ files directory exists
- Expected files are present
- If separate repo, it's a valid git repository
- Commands exist and reference correct paths

**Important**: Do not attempt to run `/jja-doctor` in the same chat session where installation occurred. The commands are not available until you restart Claude Code, as they are only loaded when the session initializes.

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
git add .claude/jji/current/jje-b251108-buk-portability.md
git commit -m "JJA: step-wrap - Completed audit of BUK portability"
cd - > /dev/null
```

**When co-located** (hardcoded during install):
```bash
git add .claude/jji/current/jje-b251108-buk-portability.md
git commit -m "JJA: step-wrap - Completed audit of BUK portability"
```

Each action specifies its own commit message pattern.

### Command Templates

The following templates are used during installation. Variables (`«JJC_*»`) are replaced with configured values.

#### `/jja-step-refine` Template

```markdown
You are helping refine a step's specification in the current Job Jockey effort.

Configuration:
- JJ files path: «JJC_FILESYSTEM_RELATIVE_PATH»
- Kit path: «JJC_KIT_PATH»

Steps:

1. Check for current effort in «JJC_FILESYSTEM_RELATIVE_PATH»current/
   - If no effort: announce "No active effort" and stop
   - If multiple: ask which one

2. Ask which step to refine (or infer from context)

3. Read the current step spec and assess its state:
   - Is mode defined? (manual/delegated/unset)
   - Is spec sparse or detailed?

4. Conduct adaptive interview:

   If spec is sparse/new:
   - "Is this a manual step (you drive) or should we prepare it for delegation (model drives)?"
   - If manual: confirm and done
   - If delegated: continue to step 5

   If spec exists:
   - Show current spec summary
   - "What needs to change?"
   - Focus on the delta

5. For delegated mode, ensure spec covers:
   - Objective: What specifically to achieve?
   - Scope: What files/systems to touch or avoid?
   - Success: How do we know it's done?
   - Failure: What to do if stuck? (stop/report/retry)
   - Model hint: haiku-ok / needs-sonnet / needs-opus

   Ask only for missing elements.

6. Final clarity check (delegated only):
   Read the spec as if you have no prior context. Assess:
   - Objective: clear or ambiguous?
   - Scope: bounded or unclear?
   - Success: measurable or vague?
   - Stuck: know when to stop or might spin?

   If any check fails, explain why and ask clarifying question.
   Loop until all checks pass.

7. Update the step in the effort file with refined spec

8. Commit: "JJA: step-refine - [step title] now [manual|delegated]"

9. Report what was updated

Error handling: If paths wrong or files missing, announce issue and stop.
```

#### `/jja-step-delegate` Template

```markdown
You are executing a delegated step from the current Job Jockey effort.

Configuration:
- JJ files path: «JJC_FILESYSTEM_RELATIVE_PATH»
- Kit path: «JJC_KIT_PATH»

Steps:

1. Check for current effort in «JJC_FILESYSTEM_RELATIVE_PATH»current/
   - If no effort: announce "No active effort" and stop

2. Identify the step to delegate (from context or ask)

3. Validate the step:
   - Is mode `delegated`?
     - If `manual`: refuse with "This step is manual - work on it conversationally"
     - If unset: refuse with "Run /jja-step-refine first to set mode"
   - Is spec healthy? Check for:
     - Objective defined
     - Scope bounded
     - Success criteria clear
     - Failure behavior specified
   - If unhealthy: refuse with "This step needs refinement - [specific gap]"

4. If valid, present the step spec clearly:
   ```
   Executing delegated step: **[title]**

   Objective: [objective]
   Scope: [scope]
   Success: [criteria]
   On failure: [behavior]
   ```

5. Execute the step based solely on the spec
   - Work from the spec, not from refinement conversation context
   - Stay within defined scope
   - Stop when success criteria met OR failure condition hit

6. Report outcome:
   - Success: what was accomplished, evidence of success criteria
   - Failure: what was attempted, why stopped, what's needed

7. Do NOT auto-complete the step. User decides via /jja-step-wrap

Error handling: If paths wrong or files missing, announce issue and stop.
```

### Example Command Structure (Before Installation)

Template in this kit with variables:

```markdown
You are helping mark a step complete in the current Job Jockey effort.

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
5. Commit: "JJA: step-wrap - [brief description]"
6. Report what was done
```

### Example Command Structure (After Installation)

Generated `.claude/commands/jja-step-wrap.md` with hardcoded values:

```markdown
You are helping mark a step complete in the current Job Jockey effort.

Configuration:
- JJ files path: ../project-admin/.claude/jji/
- Separate repo: yes
- Kit path: Tools/jjk/job-jockey-kit.md

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
   git commit -m "JJA: step-wrap - [brief description]"
   cd - > /dev/null
6. Report what was done

Error handling: If files missing or paths wrong, announce issue and stop.
```

**Important**: After installation completes, restart your Claude Code session for the new commands to become available.

## Future Enhancements

- Automated prompts for itch triage
- Cross-effort learning/pattern extraction
- Template generation for new efforts
- Integration with project-specific workflows
- Enhanced effort metadata (estimated duration, tags, dependencies)

---

*This is a living document. Refine as the system evolves.*
