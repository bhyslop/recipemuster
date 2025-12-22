---
description: Regenerate command files from kit
---

You are regenerating Concept Model Kit command files from the kit source.

**Configuration:**
- Lenses directory: lenses/
- Kit path: Tools/cmk/concept-model-kit.md
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Process:**

1. Read the kit file at Tools/cmk/concept-model-kit.md
2. Extract all command templates from the "Command Templates" section
3. Replace all `«CML_LENSES_DIR»` with `lenses/`
4. Replace all `«CML_KIT_PATH»` with `Tools/cmk/concept-model-kit.md`
5. Replace all `«CML_OPEN_SOURCE_GIT_UPSTREAM»` with `OPEN_SOURCE_UPSTREAM`
6. Delete existing cma-*.md files in .claude/commands/
7. Write updated command files
8. Report changes made

**Use this command when:**
- The kit has been updated and commands need regeneration
- Configuration values have changed
- Commands appear to be out of sync with the kit
