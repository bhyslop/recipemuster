# Venture Wrangler Policy

## Overview

The Venture Wrangler (VW) system manages ephemeral project documents with clear lifecycles. These documents are primarily written and read by Claude, with occasional human adjustment.

## Terminology

- **Effort**: A bounded initiative spanning 3-50 chat sessions
- **Step**: A discrete action within the current effort
- **Idea**: A potential future effort or consideration

## Document Types

### Primary Documents

All Venture Wrangler documents use the `vw` prefix with category-specific third letters:

#### `vwe-YYMMDD-description.md` (Venture Wrangler Effort)
- Main context document for an effort
- Named with creation date and brief description
- Example: `vwe-251107-buk-portability.md`
- Lifecycle: Active → Completed (moved to `completed/`)
- Located in: `.claude/tasks/`

#### `vw-current.md` (Current Effort Symlink)
- Symlink pointing to the active effort in `tasks/`
- Provides stable filename for tooling
- Located in: `.claude/`

#### `vws-steps.md` (Venture Wrangler Steps)
- Remaining steps in current effort
- Checklist format for tracking completion
- Cleared/minimal at effort completion
- Located in: `.claude/`

#### `vwf-future.md` (Venture Wrangler Future)
- Ideas for worthy future efforts
- Items graduate from here to new `vwe-` files
- Located in: `.claude/`

#### `vwx-shelved.md` (Venture Wrangler eXcluded/Shelved)
- Ideas respectfully set aside
- Not rejected, but deferred for foreseeable future
- May include brief context on why shelved
- Located in: `.claude/`

#### `vwp-policy.md` (Venture Wrangler Policy)
- This document
- Defines structure, naming, and conventions
- Located in: `.claude/`

## Directory Structure

```
.claude/
  vw-current.md           # Symlink to tasks/vwe-YYMMDD-*.md
  vws-steps.md            # Current effort steps
  vwf-future.md           # Future effort ideas
  vwx-shelved.md          # Shelved ideas
  vwp-policy.md           # This policy document
  tasks/
    vwe-251107-buk-portability.md
    vwe-251023-gad-implementation.md
    completed/
      vwe-251015-regime-management.md
```

## Workflows

### Starting a New Effort
1. Create `vwe-YYMMDD-description.md` in `.claude/tasks/`
2. Update `vw-current.md` symlink to point to new effort
3. Initialize/clear `vws-steps.md` with initial steps
4. Archive previous effort to `completed/`

### Completing an Effort
1. Verify all steps in `vws-steps.md` are complete or explicitly discarded
2. Move effort file to `.claude/tasks/completed/`
3. Update symlink to next effort (if any)

### Idea Triage (Quick Place Decision Tree)
When a new idea emerges:
1. **Does it block current effort completion?** → Add to `vws-steps.md`
2. **Is it worthy but not now?** → Add to `vwf-future.md`
3. **Interesting but setting aside?** → Add to `vwx-shelved.md`

### Archival
- All archiving is manual
- Git preserves history across moves (`git log --follow`)
- Use `git mv` to preserve history cleanly
- Future: May create Claude commands to assist

## Format Preferences

- **All documents**: Markdown (`.md`)
- **Steps**: Checklist format with `- [ ]` and `- [x]`
- **Dates**: YYMMDD format (e.g., 251107 for 2025-11-07)
- **Descriptions**: Lowercase with hyphens (e.g., `buk-portability`)

## Integration with CLAUDE.md

The main project `CLAUDE.md` should reference this system:
- Point to `.claude/vw-current.md` for active effort context
- Reference `vwp-policy.md` for system documentation
- Claude should check these files at session start when relevant

## Design Principles

1. **Ephemeral by design**: Documents have clear lifecycles
2. **Model-primary**: Claude reads/writes frequently, human adjusts occasionally
3. **Clear naming**: Prefixes make purpose immediately obvious
4. **Git-friendly**: Preserve history, support archival
5. **Minimal ceremony**: Easy to use, hard to misuse
6. **Respectful**: Ideas are "shelved" not "rejected"

## Future Enhancements

- Custom Claude commands for effort lifecycle management
- Automated prompts for idea triage
- Cross-effort learning/pattern extraction
- Template generation for new efforts

---

*This is a living document. Refine as the system evolves.*
