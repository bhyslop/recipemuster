---
description: Prepare branch for upstream PR contribution
---

You are preparing a PR branch for upstream contribution, stripping proprietary content.

**Configuration:**
- Lenses directory: lenses/
- Kit path: ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md
- Upstream remote: OPEN_SOURCE_UPSTREAM

**Execute these steps:**

0. **Request permissions upfront:**
   - Ask user for permission to execute all git operations needed:
     - `git status`, `git push origin develop`
     - `git checkout main`, `git fetch OPEN_SOURCE_UPSTREAM`, `git pull OPEN_SOURCE_UPSTREAM main`
     - `git push origin main`
     - `git ls-remote --heads OPEN_SOURCE_UPSTREAM`, `git branch -a`
     - `git checkout -b candidate-NNN-R main`
     - `git log main..develop`, `git diff main..develop --stat`
     - `git merge --squash develop`
     - `git rm -rf` commands to remove proprietary content
     - `git commit`, `git ls-files` verification
   - Get approval before proceeding

1. **Verify develop is clean and pushed:**
   - Check `git status` on develop branch
   - Ensure working tree is clean
   - Push any uncommitted changes to origin/develop

2. **Sync main with upstream:**
   - `git checkout main`
   - `git fetch OPEN_SOURCE_UPSTREAM`
   - `git pull OPEN_SOURCE_UPSTREAM main`
   - If pull fails (non-fast-forward or conflicts), **ABORT** and ask user to resolve
   - `git push origin main` (should be fast-forward)

3. **Auto-detect next candidate branch:**
   - Find max batch from upstream: `git ls-remote --heads OPEN_SOURCE_UPSTREAM | grep 'candidate-'`
   - Find max batch from local branches
   - **If batch exists upstream** → new batch (previous merged): create `candidate-{MAX+1}-1`
   - **If batch NOT upstream** → redo (pending/damaged): create `candidate-{MAX}-{REV+1}`
   - Tell user which branch and why

4. **Create candidate branch:**
   - `git checkout -b candidate-NNN-R main`

5. **Squash merge develop:**
   - Show commits: `git log main..develop --oneline`
   - Show summary: `git diff main..develop --stat`
   - Execute: `git merge --squash develop`

6. **Strip proprietary content:**
   - `git rm -rf --ignore-unmatch .claude/`
   - `git rm -rf --ignore-unmatch lenses/`
   - Verify removal: `git ls-files | grep -E '(\.claude/|lenses/)'` should return nothing
   - If files remain, **ERROR** and stop

7. **Generate commit:**
   - Analyze `git log main..develop --format="%s%n%b"`
   - Filter out commits that only touch stripped files
   - Analyze staged changes with `git diff --cached --stat`
   - Draft consolidated commit message summarizing features/fixes
   - Show proposed message to user
   - Create commit: `git commit -m "message"`
   - **Do NOT include attribution footer or AI mention**

8. **Final review:**
   - Show `git log -1 --stat`
   - Show `git diff main..HEAD --stat`
   - Display instructions:
     ```
     To amend the commit message before pushing:
     git commit --amend

     To push to origin when ready:
     git push -u origin candidate-NNN-R

     To return to development work:
     git checkout develop
     ```
   - **STOP** - user reviews and pushes manually

**Important:**
- Be methodical and show output at each step
- Stop immediately if any errors occur
- User maintains control over final push
