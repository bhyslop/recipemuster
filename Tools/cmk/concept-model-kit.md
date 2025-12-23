# Concept Model Kit

## What is Concept Model Kit?

Concept Model Kit (CMK) is a system for creating, editing, and rendering MCM-conformant concept model documents. It provides tooling for human-directed, LLM-executed curation of technical specifications that use linked terminology and precise relationship mapping.

The kit supports:
- **Editing**: Whitespace normalization, mapping section cleanup, link validation
- **Rendering**: Transform concept models to ClaudeMark format for LLM consumption
- **Workflow**: Prepare contributions for upstream repositories (stripping proprietary content)

This document (the Concept Model Kit) is the complete reference and installer for the system.

## Installation Variables

During installation, Claude replaces these markers in generated command files:

- `«CML_LENSES_DIR»` → Directory containing concept model documents
  - Default: `lenses/`
  - Example: `docs/specs/` or `specifications/`
- `«CML_KIT_PATH»` → Path to this Kit file
  - Example: `tools/cmk/concept-model-kit.md`
  - Example: `../shared-tools/cmk/concept-model-kit.md`
- `«CML_KIT_DIR»` → Directory containing the kit (derived from KIT_PATH)
  - Example: `tools/cmk/`
  - Used for locating kit reference documents (MCM, AXL)
- `«CML_OPEN_SOURCE_GIT_UPSTREAM»` → Git remote name for upstream repository
  - Default: `OPEN_SOURCE_UPSTREAM`

These markers appear throughout this document in templates and will be hardcoded with actual values during installation.

## Core Concepts

### Document Structure

Concept model documents follow the Meta Concept Model (MCM) specification. Key elements:

**Linked Term**: `{category_term}`
A reference to defined vocabulary. The curly braces invoke an AsciiDoc attribute that expands to display text with an anchor link.

**Attribute Reference**: `:category_term: <<anchor,Display Text>>`
Defined in the mapping section. Maps a category-prefixed attribute name to an anchor target and display text.

**Anchor**: `[[anchor_name]]`
A definition target that linked terms point to. Appears on its own line before the definition.

**Annotation**: `// ⟦content⟧`
A Strachey-bracket comment between anchor and definition. Used for type categorization when a referencing system (like AXL) is in context. Ignored during first-order document reading.

### Category Prefixes

Each linked term uses a category prefix that classifies it by conceptual role:
- Prefixes are declared in the mapping section header
- Common pattern: `mcm_` for Meta Concept Model terms
- Variant suffixes: `_s` (plural), `_p` (possessive), `_ed` (past), `_ing` (progressive)

### Whitespace Rules

MCM documents follow specific whitespace patterns for clean diff tracking:

1. **One sentence per line** - Each sentence ends with a line break
2. **Linked terms isolated** - Line breaks before and after standalone `{term}` references in prose (exception: terms at start of bullet items stay on the marker line, as AsciiDoc requires `* content` syntax)
3. **Term sequences together** - Multiple terms in sequence stay on one line, with breaks before first and after last
4. **Blank lines between paragraphs only** - Never within paragraphs or around list items
5. **Mapping section alignment** - The `<<` aligns to columns that are multiples of 10

### Definition Form Patterns

Three patterns for defining terms:

**Definition List Form** (`mcm_form_deflist`):
```asciidoc
[[term_anchor]]
// ⟦...⟧
{term_reference}::
Definition text following sentence-per-line rules.
```

**Bullet Form** (`mcm_form_bullet`):
```asciidoc
* [[field_anchor]]
// ⟦...⟧
{field_reference} defines the specific meaning within parent structure.
```

**Section Form** (`mcm_form_section`):
```asciidoc
[[sequence_anchor]]
// ⟦...⟧
=== {sequence_reference}

The sequence executes these steps:
```

## File Structure

### Kit Files (copied to repo during install)

After installation, the kit directory exists locally at `«CML_KIT_PATH»`:

- `concept-model-kit.md` - This file: installer, reference, and command templates
- `mcm-MCM-MetaConceptModel.adoc` - Meta Concept Model specification
- `axl-AXLA-Lexicon.adoc` - Axial Lexicon (optional, for AXL-annotated documents)
- `axl-AXMCM-ClaudeMarkConceptMemo.md` - ClaudeMark format reference

### Project Files (created at install)

- `«CML_LENSES_DIR»/` - Project-specific concept model documents (MCM/AXL specs remain in kit directory)
- `.claude/commands/cma-*.md` - Generated command files
- `.claude/agents/cmsa-*.md` - Generated subagent files

### ClaudeMark Output

Rendered ClaudeMark files are placed in `«CML_LENSES_DIR»/` with `-claudemark.md` suffix:
- `my-spec.adoc` → `my-spec-claudemark.md`

## Actions

Concept Model Actions (CMA) are Claude Code slash commands for working with concept model documents.

### Editing Actions

#### `/cma-normalize`
Apply whitespace normalization (ancestry enhancement) to concept model documents.

**Usage**: `/cma-normalize [file-path | all]`

**Model**: `haiku` (mechanical, rule-based task)

**Behavior**:
- Applies MCM whitespace rules to specified file or all files in lenses directory
- One sentence per line
- Linked terms on separate lines (unless in sequence)
- Blank lines only between paragraphs
- Preserves code block contents unchanged

#### `/cma-scrub`
Clean up mapping section formatting.

**Usage**: `/cma-scrub [file-path | all]`

**Model**: `haiku` (mechanical, rule-based task)

**Behavior**:
- Aligns `<<` to columns that are multiples of 10
- Ensures consistent variant patterns
- Alphabetizes entries by replacement text
- Validates category declarations

### Rendering Actions

#### `/cma-render`
Transform concept model document to ClaudeMark format.

**Usage**: `/cma-render [source-file] [full | --terms term1,term2 | --section "Section Name"]`

**Model**: `sonnet` (with validation before presenting)

**Behavior**:
- Transforms AsciiDoc to ClaudeMark using guillimet syntax
- `full`: Include all terms
- `--terms`: Include only specified terms and their definitions
- `--section`: Include only terms from specified section
- Output placed in lenses directory with `-claudemark.md` suffix
- Reviews output for semantic fidelity before presenting

### Validation Actions

#### `/cma-validate`
Validate links, anchors, and optionally annotations.

**Usage**: `/cma-validate [file-path | all]`

**Model**: Default (inherit)

**Behavior**:
- Verifies all `{term}` references have corresponding `:term:` definitions
- Verifies all `:term:` definitions have corresponding `[[anchor]]` targets
- Checks for orphaned anchors
- If AXL is in context: validates annotation format and motif references

### Workflow Actions

#### `/cma-prep-pr`
Prepare a branch for upstream contribution, stripping proprietary content.

**Usage**: `/cma-prep-pr`

**Model**: Default (inherit)

**Behavior**:
- Syncs main with upstream (`«CML_OPEN_SOURCE_GIT_UPSTREAM»`)
- Auto-detects next candidate branch number
- Squash merges develop to candidate branch
- Strips: `.claude/` directory, `«CML_LENSES_DIR»/` directory
- Generates consolidated commit message
- Stops for user review before push

#### `/cma-doctor`
Validate Concept Model Kit installation.

**Usage**: `/cma-doctor`

**Behavior**:
- Verifies kit file exists at configured path
- Verifies lenses directory exists
- Checks all command files present
- Reports any issues found

## Installation

### Prerequisites
- A git repository with a `CLAUDE.md` file
- Claude Code access to the repository

### Bootstrap Process

1. **Obtain kit source**: Access this kit file from any location:
   - Clone/checkout a repo containing the kit
   - Download from a release
   - Access from a sibling repo (e.g., `../shared-tools/cmk/`)

2. **Run the installation conversation** with Claude Code:
   - Say: "Read [path-to-kit]/concept-model-kit.md and install Concept Model Kit"
   - Installation is fully idempotent - safe to run any time

3. **Configuration handling** (smart detection):
   - Claude checks CLAUDE.md for existing `## Concept Model Kit Configuration` section
   - **If found**: Shows current config and asks "Keep this configuration? [Y/n]"
     - If yes: skips questions, regenerates commands with existing config
     - If no: asks configuration questions as if fresh install
   - **If not found**: Asks configuration questions:
     - **Kit destination**: Where to copy the kit in this repo? (Default: `Tools/cmk/`)
     - **Lenses directory**: Where should concept model documents live? (Default: `lenses/`)
     - **Upstream remote**: Git remote name for upstream repository? (Default: `OPEN_SOURCE_UPSTREAM`)

4. **Claude will then**:
   - **Copy the kit directory** into the target repo at the configured destination
     - This creates a local, versioned copy of the kit
     - The kit path in CLAUDE.md points to this LOCAL copy
   - Delete any existing `cma-*.md` command files from `.claude/commands/`
   - Delete any existing `cmsa-*.md` subagent files from `.claude/agents/`
   - Generate all command files with hardcoded paths (pointing to local kit)
   - Generate all subagent files with hardcoded paths
   - Create lenses directory if it doesn't exist (for project-specific concept models)
   - Replace the `## Concept Model Kit Configuration` section in CLAUDE.md
   - Commit the changes

**Important**: The kit is COPIED into your repo, not referenced externally. This ensures:
- Each repo has its own versioned kit copy
- No cross-repo dependencies
- Kit updates are explicit (re-run install from updated source)

**Re-running install**: To update the kit after upstream changes, run installation again pointing to the new source. It will detect existing config but replace the kit files with the new version.

5. **Installation completes**. CLAUDE.md will contain:

```markdown
## Concept Model Kit Configuration

Concept Model Kit (CMK) is installed for managing concept model documents.

**Configuration:**
- Lenses directory: `lenses/`
- Kit path: `Tools/cmk/concept-model-kit.md`
- Upstream remote: `OPEN_SOURCE_UPSTREAM`

**Concept Model Patterns:**
- **Linked Terms**: `{category_term}` - references defined vocabulary
- **Attribute References**: `:category_term: <<anchor,Display Text>>` - in mapping section
- **Anchors**: `[[anchor_name]]` - definition targets
- **Annotations**: `// ⟦content⟧` - Strachey brackets for type categorization

**Available commands:**
- `/cma-normalize` - Apply whitespace rules (haiku)
- `/cma-scrub` - Clean mapping section formatting (haiku)
- `/cma-render` - Transform to ClaudeMark (sonnet)
- `/cma-validate` - Check links and annotations
- `/cma-prep-pr` - Prepare upstream contribution
- `/cma-doctor` - Validate installation

**Subagents:**
- `cmsa-normalizer` - Haiku-enforced whitespace normalization

For full MCM specification, see `Tools/cmk/mcm-MCM-MetaConceptModel.adoc`.

**Important**: Restart Claude Code session after installation for new commands and subagents to become available.
```

## Command Templates

The following templates are used during installation. Variables (`«CML_*»`) are replaced with configured values.

### `/cma-normalize` Template

```markdown
---
description: Apply whitespace normalization to concept model documents
argument-hint: [file-path | all]
---

Invoke the cmsa-normalizer subagent to apply MCM whitespace normalization.

**Target:** $ARGUMENTS

Use the Task tool with:
- `subagent_type`: "cmsa-normalizer"
- `prompt`: Include the target from arguments and configuration below

**Configuration to pass:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»

**File Resolution** (include in prompt):
When a filename is provided (not "all"):
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc
```

### `/cma-scrub` Template

```markdown
---
description: Clean up mapping section formatting
argument-hint: [file-path | all]
model: haiku
---

You are cleaning up the mapping section of concept model documents.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»

**Target:** $ARGUMENTS (use "all" for all .adoc files in lenses directory)

**File Resolution:**
When a filename is provided (not "all"):
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc

**Mapping Section Rules:**

1. **Column alignment**: The `<<` of each definition aligns to columns that are multiples of 10
   - Minimum column: 30 (for short attribute names)
   - Adjust all entries together when one requires more space

2. **Ordering**: Entries alphabetized by the replacement text (what appears in `<<anchor,This Text>>`)

3. **Category declarations**: Comment block at top declares all category prefixes used

4. **Variant consistency**: Related variants grouped together:
   ```
   :excm_term:                   <<excm_term,Term>>
   :excm_term_s:                 <<excm_term,Terms>>
   :excm_term_p:                 <<excm_term,Term's>>
   ```

5. **Section markers**: Preserve `// tag::mapping-section[]` and `// end::mapping-section[]`

**Process:**
1. Find the mapping section (between tag markers or at document start)
2. Parse all attribute definitions
3. Recalculate alignment column
4. Sort by replacement text
5. Show diff for approval
6. Write updated file after approval

**Error handling:** If mapping section not found, report and stop.
```

### `/cma-render` Template

```markdown
---
description: Transform concept model to ClaudeMark format
argument-hint: [source-file] [full | --terms t1,t2 | --section "Name"]
model: sonnet
---

You are transforming a concept model document to ClaudeMark format for LLM consumption.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»

**Source:** $1
**Mode:** $2 (default: full)

**File Resolution:**
When resolving the source file:
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc

**ClaudeMark Syntax:**
- Term definitions: `### Term Name «term-id»`
- Term references: `«term-id»`
- Prose flows naturally without AsciiDoc markup
- Code literals: Standard backticks preserved

**Transformation Rules:**

1. **Extract term definitions**: For each `[[anchor]]` with definition, create:
   ```markdown
   ### Display Text «anchor»
   Definition prose with «other-term» references.
   ```

2. **Convert references**: `{category_term}` becomes `«anchor»` (using the anchor from the attribute definition)

3. **Subsetting modes**:
   - `full`: Include all terms
   - `--terms t1,t2`: Include only listed terms (by anchor name)
   - `--section "Name"`: Include only terms from that section

4. **Strip annotations**: `// ⟦...⟧` comments do not appear in output (they inform transformation, not output)

5. **Preserve structure**: Section hierarchy maps to heading levels

**Output:** Write to `«CML_LENSES_DIR»/[source-basename]-claudemark.md`

**Validation before presenting:**
- Verify all term references resolve
- Check that semantic meaning is preserved
- Report any transformation issues

**Error handling:** If source not found, report and stop.
```

### `/cma-validate` Template

```markdown
---
description: Validate concept model links and annotations
argument-hint: [file-path | all]
---

You are validating the structural integrity of concept model documents.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»

**Target:** $ARGUMENTS (use "all" for all .adoc files in lenses directory)

**File Resolution:**
When a filename is provided (not "all"):
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc

**Validation Checks:**

1. **Reference completeness**:
   - Every `{term}` used in prose has a `:term:` definition in mapping section
   - Report: "Reference to undefined term: {missing_term}"

2. **Definition completeness**:
   - Every `:term:` definition has a `[[anchor]]` target somewhere in document
   - Report: "Definition without anchor: :term:"

3. **Anchor orphans**:
   - Every `[[anchor]]` has at least one reference via `{term}` or `<<anchor,...>>`
   - Report: "Orphaned anchor: [[unused_anchor]]" (warning, not error)

4. **Annotation format** (if AXL context detected):
   - Strachey brackets properly formed: `// ⟦...⟧`
   - Content is space-separated terms
   - First term is relationship (e.g., `axl_voices`)
   - Report: "Malformed annotation at line N"

**Output format:**
```
Validating: filename.adoc
- Errors: N
  - Reference to undefined term: {foo}
  - Definition without anchor: :bar:
- Warnings: M
  - Orphaned anchor: [[baz]]
- Annotations: P checked, Q issues
```

**Summary:** Report total errors/warnings across all files.

**Error handling:** Continue validation even if some files have issues.
```

### `/cma-prep-pr` Template

```markdown
---
description: Prepare branch for upstream PR contribution
---

You are preparing a PR branch for upstream contribution, stripping proprietary content.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»
- Upstream remote: «CML_OPEN_SOURCE_GIT_UPSTREAM»

**Execute these steps:**

0. **Request permissions upfront:**
   - Ask user for permission to execute all git operations needed
   - Get approval before proceeding

1. **Verify develop is clean and pushed:**
   - Check `git status` on develop branch
   - Push any uncommitted changes to origin/develop

2. **Sync main with upstream:**
   - `git checkout main`
   - `git fetch «CML_OPEN_SOURCE_GIT_UPSTREAM»`
   - `git pull «CML_OPEN_SOURCE_GIT_UPSTREAM» main`
   - If pull fails, **ABORT** and ask user to resolve
   - `git push origin main`

3. **Auto-detect next candidate branch:**
   - Find max batch from upstream: `git ls-remote --heads «CML_OPEN_SOURCE_GIT_UPSTREAM» | grep 'candidate-'`
   - Find max batch from local branches
   - If batch exists upstream → new batch (previous merged)
   - If batch not upstream → redo (increment revision)
   - Tell user which branch and why

4. **Create candidate branch:**
   - `git checkout -b candidate-NNN-R main`

5. **Squash merge develop:**
   - Show commits: `git log main..develop --oneline`
   - Execute: `git merge --squash develop`

6. **Strip proprietary content:**
   - `git rm -rf --ignore-unmatch .claude/`
   - `git rm -rf --ignore-unmatch «CML_LENSES_DIR»/`
   - Verify removal with `git ls-files`

7. **Generate commit:**
   - Analyze changes for consolidated message
   - Filter out commits that only touch stripped files
   - Create commit (no attribution footer)

8. **Final review:**
   - Show `git log -1 --stat`
   - Display push instructions
   - **STOP** - user reviews and pushes manually

**Important:**
- Be methodical, show output at each step
- Stop immediately on errors
- User maintains control over final push
```

### `/cma-doctor` Template

```markdown
---
description: Validate Concept Model Kit installation
---

You are validating the Concept Model Kit installation.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»
- Upstream remote: «CML_OPEN_SOURCE_GIT_UPSTREAM»

**Checks:**

1. **Kit file:**
   - Verify «CML_KIT_PATH» exists and is readable
   - Report: ✓ Kit file found / ✗ Kit file not found

2. **Lenses directory:**
   - Verify «CML_LENSES_DIR» exists
   - List .adoc files found
   - Report: ✓ Lenses directory with N documents / ✗ Missing

3. **Command files:**
   - Check for all expected files in `.claude/commands/`:
     - cma-normalize.md
     - cma-scrub.md
     - cma-render.md
     - cma-validate.md
     - cma-prep-pr.md
     - cma-doctor.md
   - Report: ✓ All 6 commands present / ✗ Missing: [list]

4. **Subagent files:**
   - Check for all expected files in `.claude/agents/`:
     - cmsa-normalizer.md
   - Report: ✓ All 1 subagents present / ✗ Missing: [list]

5. **Git remote** (for prep-pr):
   - Check if «CML_OPEN_SOURCE_GIT_UPSTREAM» remote exists
   - Report: ✓ Remote configured / ⚠ Remote not found (prep-pr may fail)

6. **CLAUDE.md integration:**
   - Verify CMK configuration section exists
   - Report: ✓ CLAUDE.md configured / ✗ Missing configuration

**Summary:**
```
Concept Model Kit Health Check
==============================
Kit:        ✓ Found at tools/cmk/concept-model-kit.md
Lenses:     ✓ lenses/ with 3 documents
Commands:   ✓ All 6 commands installed
Subagents:  ✓ All 1 subagents installed
Remote:     ✓ OPEN_SOURCE_UPSTREAM configured
CLAUDE.md:  ✓ Configured

Status: HEALTHY
```

Or if issues:
```
Status: NEEDS ATTENTION
- Missing command: cma-render.md
- Missing subagent: cmsa-normalizer.md
- Remote OPEN_SOURCE_UPSTREAM not configured
```
```

## Subagent Templates

The following subagent templates are used during installation. Variables (`«CML_*»`) are replaced with configured values. Subagents are written to `.claude/agents/`.

### `cmsa-normalizer` Template

```markdown
---
name: cmsa-normalizer
description: Whitespace normalization for concept model documents. Enforces MCM ancestry enhancement rules.
model: haiku
tools: Read, Edit, Grep, Glob
---

You are applying MCM whitespace normalization (ancestry enhancement) to concept model documents.

**Configuration:**
- Lenses directory: «CML_LENSES_DIR»
- Kit directory: «CML_KIT_DIR»
- Kit path: «CML_KIT_PATH»

**CRITICAL CONSTRAINT - Whitespace Only:**
This operation adjusts ONLY line breaks and blank lines. You must NOT change any words, punctuation, or sentence structure. The document content must be identical before and after - only the placement of newlines changes.

**DO NOT:**
- Reword or rephrase any text
- Split compound sentences into multiple sentences
- Join sentences together
- Add new text or delete existing text
- Change punctuation
- "Improve" clarity or readability through rewording
- Fix grammar or spelling (report issues but do not fix)

**Whitespace Rules to Apply:**

1. **One sentence per line**: Break at EXISTING sentence boundaries (periods, question marks, exclamation points followed by space and capital letter). Do not create new sentences by restructuring.

2. **Linked terms isolated**: When a `{term_reference}` appears standalone in prose:
   - Line break before the term
   - Line break after the term
   - **Exception**: Terms at start of bullet items stay on the marker line (AsciiDoc requires `* content` syntax)
   - Example A (mid-sentence) - BEFORE:
     ```
     The system uses {excm_processor} to handle requests.
     ```
   - Example A - AFTER:
     ```
     The system uses
     {excm_processor}
     to handle requests.
     ```
   - Example B (start of sentence) - BEFORE:
     ```
     Previous sentence ends here.
     {excm_term} starts a new sentence.
     ```
   - Example B - AFTER:
     ```
     Previous sentence ends here.
     {excm_term}
     starts a new sentence.
     ```

3. **Term sequences together**: Multiple terms in sequence stay on one line:
   - Line break before the first term
   - Line break after the last term
   - Terms separated by commas/and stay together
   - Example:
     ```
     The interface supports
     {excm_drag}, {excm_drop}, and {excm_scroll}
     operations.
     ```

4. **Blank lines between paragraphs only**:
   - One blank line between paragraphs
   - No blank lines within paragraphs
   - No blank lines around list items (except before bulleted lists in Task Lens sections)
   - No blank lines around code blocks

5. **Preserve code blocks**: Content inside `----` fences is opaque. Do not modify.

6. **Punctuation stays attached**: Periods, commas stay with their text, not on separate lines.

**Edit Tool Warning:** When using the Edit tool, `{term}` references must remain as single braces. Do NOT escape or double them. The Edit tool takes literal strings - write `{mcm_term}` not `{{mcm_term}}`.

**Process:**
1. Read the target file(s)
2. **Search phase**: Use Grep to find all `\{[a-z_]+\}` patterns outside code blocks. This creates your checklist of terms to verify.
3. **Check each term**: For every term found, verify it has:
   - Line break immediately before (or is after bullet marker `* `)
   - Line break immediately after (or punctuation like `,` then line break)
   - Skip terms inside `----` code fences
4. Fix all violations found
5. **Verify**: Search again to confirm no inline terms remain (terms with text on same line before AND after)
6. Present summary of changes made

**Self-check**: Verify that removing all newlines from both versions produces identical text. If not, you have made unauthorized content changes - revert and try again.

**Error handling:** If file not found or not .adoc, report and stop.
```

## Design Principles

1. **Context-efficient**: Commands carry only the knowledge they need
2. **Source of truth**: Kit file is canonical; commands are derived
3. **Explicit invocation**: User controls when operations run
4. **Model-appropriate**: Haiku for mechanical tasks, Sonnet for judgment
5. **Git-friendly**: Clear commits, user controls push
6. **Proprietary-aware**: prep-pr strips internal content cleanly

---

*This is a living document. Refine as the system evolves.*
