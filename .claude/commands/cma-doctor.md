---
description: Validate Concept Model Kit installation
---

You are validating the Concept Model Kit installation.

**Configuration:**
- Lenses directory: lenses/
- Kit path: ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Checks:**

1. **Kit file:**
   - Verify ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md exists and is readable
   - Report: ✓ Kit file found / ✗ Kit file not found

2. **Lenses directory:**
   - Verify lenses/ exists
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
     - cma-refresh.md
   - Report: ✓ All 7 commands present / ✗ Missing: [list]

4. **Git remote** (for prep-pr):
   - Check if OPEN_SOURCE_UPSTREAM remote exists
   - Run: `git remote -v | grep OPEN_SOURCE_UPSTREAM`
   - Report: ✓ Remote configured / ⚠ Remote not found (prep-pr may fail)

5. **CLAUDE.md integration:**
   - Verify CMK configuration section exists in CLAUDE.md
   - Report: ✓ CLAUDE.md configured / ✗ Missing configuration

**Summary:**
```
Concept Model Kit Health Check
==============================
Kit:        ✓ Found at ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md
Lenses:     ✓ lenses/ with N documents
Commands:   ✓ All 7 commands installed
Remote:     ✓ OPEN_SOURCE_UPSTREAM configured
CLAUDE.md:  ✓ Configured

Status: HEALTHY
```

Or if issues:
```
Status: NEEDS ATTENTION
- Missing command: cma-render.md
- Remote OPEN_SOURCE_UPSTREAM not configured
```

**Error handling:** Report all issues found, don't stop at first error.
