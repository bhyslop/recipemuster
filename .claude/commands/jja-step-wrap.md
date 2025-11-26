# Mark a Job Jockey Step Complete

You are helping mark a step complete in the current Job Jockey effort.

Configuration:
- JJ files path: `.claude/jji/`
- Separate repo: `no`
- Kit path: `Tools/jjk/job-jockey-kit.md`

## Process

1. **Ask which step to mark done** (or infer from context)

2. **Summarize the step completion** based on chat context
   - Keep the summary brief and factual
   - Include evidence of success if applicable

3. **Show proposed summary** and ask for approval or amendments

4. **Update the effort file** in `.claude/jji/current/`
   - Move step from Pending to Completed section
   - Replace description with brief summary

5. **Commit** the change:
   ```bash
   git add .claude/jji/current/jje-*.md
   git commit -m "JJA: step-wrap - [brief description]"
   ```

6. **Report** what was done

## Error Handling

If files missing or paths wrong, announce issue and stop.
