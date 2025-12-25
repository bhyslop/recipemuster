---
description: Validate Concept Model Kit installation
---

You are validating the Concept Model Kit installation.

**Configuration:**
- Kit path: Tools/cmk/README.md
- Lenses directory: lenses
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Checks to perform:**

1. **Kit file:**
   - Verify `Tools/cmk/README.md` exists and is readable
   - Report: ✓ Kit file found / ✗ Kit file not found

2. **Lenses directory:**
   - Verify `lenses` exists
   - List .adoc files found
   - Report: ✓ Lenses directory with N documents / ✗ Missing

3. **Command files:**
   - Check for all expected files in `.claude/commands/`:
     - cma-normalize.md
     - cma-render.md
     - cma-validate.md
     - cma-prep-pr.md
     - cma-doctor.md
   - Report: ✓ All 5 commands present / ✗ Missing: [list]

4. **Subagent files:**
   - Check for all expected files in `.claude/agents/`:
     - cmsa-normalizer.md
   - Report: ✓ All 1 subagents present / ✗ Missing: [list]

5. **Git remote** (for prep-pr):
   - Check if `OPEN_SOURCE_UPSTREAM` remote exists
   - Report: ✓ Remote configured / ⚠ Remote not found (prep-pr may fail)

6. **CLAUDE.md integration:**
   - Verify CMK configuration section exists
   - Report: ✓ CLAUDE.md configured / ✗ Missing configuration

**Output format:**
```
Concept Model Kit Health Check
==============================
Kit:        ✓ Found at Tools/cmk/README.md
Lenses:     ✓ lenses/ with N documents
Commands:   ✓ All 5 commands installed
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
