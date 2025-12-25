---
description: Prepare branch for upstream PR contribution
---

You are preparing a PR branch for upstream contribution, stripping proprietary content.

**Configuration:**
- Lenses directory: lenses
- Kit directory: Tools/cmk
- Kit path: Tools/cmk/README.md
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Execute these steps:**

0. **Request permissions upfront:**
   - Ask user for permission to execute all git operations needed
   - Get approval before proceeding

1. **Verify develop is clean and pushed:**
   - Check `git status` on develop branch
   - Push any uncommitted changes to origin/develop

2. **Sync main with upstream:**
   - `git checkout main`
   - `git fetch OPEN_SOURCE_UPSTREAM`
   - `git pull OPEN_SOURCE_UPSTREAM main`
   - If pull fails, **ABORT** and ask user to resolve
   - `git push origin main`

3. **Auto-detect next candidate branch:**
   - Find max batch from upstream: `git ls-remote --heads OPEN_SOURCE_UPSTREAM | grep 'candidate-'`
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
   - `git rm -rf --ignore-unmatch lenses/`
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
