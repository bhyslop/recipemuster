---
description: Regenerate command files from kit
---

You are regenerating Concept Model Kit command files from the kit source.

**Configuration:**
- Lenses directory: lenses/
- Kit path: ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Steps:**

1. **Read kit file** at ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md

2. **Extract current configuration** from CLAUDE.md CMK section:
   - Lenses directory
   - Upstream remote name

3. **Delete existing commands:**
   - Remove all `.claude/commands/cma-*.md` files

4. **Generate fresh commands:**
   - Extract each template from kit
   - Replace all `«CML_*»` variables with configured values:
     - `«CML_LENSES_DIR»` → lenses/
     - `«CML_KIT_PATH»` → ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md
     - `«CML_OPEN_SOURCE_GIT_UPSTREAM»` → OPEN_SOURCE_UPSTREAM
   - Write to `.claude/commands/cma-*.md`

5. **Commit the refresh:**
   ```bash
   git add .claude/commands/cma-*.md
   git commit -m "CMA: refresh - Regenerated commands from kit"
   ```

6. **Report:**
   ```
   Refreshed 7 command files from kit.
   Configuration unchanged.
   Committed to repository.
   ```

**Note:** This preserves existing configuration. To change configuration, re-run full installation.
