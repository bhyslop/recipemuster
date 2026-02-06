---
argument-hint: [firemark]
description: Mount up and execute next pace
---

Mount a heat: identify the next actionable pace and begin execution.

Use this command when you're ready to work - it finds the next rough or bridled pace and drives toward completing it.

Arguments: $ARGUMENTS (optional Firemark or silks to select specific heat)

## Prerequisites

## Step 1: Saddle up

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_saddle $ARGUMENTS
```

If $ARGUMENTS is empty, saddle auto-selects the first racing heat.
If $ARGUMENTS contains a Firemark (e.g., `AA` or `₣AA`), saddle uses that heat.

Parse plain text output by label prefix:
- Racing-heats table at top of output → list of racing heats with firemarks
- Line starting with `Heat:` → extract heat silks, firemark, and status
- Line starting with `Paddock:` → extract paddock file path
- Section `Paddock-content:` (ends at blank line) → extract paddock content (2-space indented)
- Line starting with `Next:` → extract pace silks, coronet, and state (if present)
- Section `Spec:` (ends at blank line) → extract spec text (2-space indented)
- Section `Direction:` (ends at blank line) → extract direction text if bridled (2-space indented)
- Section `Recent-work:` → column-formatted table with headers

If `Next:` line is absent, no actionable pace exists.

## Step 2: Display context

Show:
- **Racing heats table** (from saddle output top) — show all racing heats so user sees the full context
- Heat silks and Firemark (of the selected heat)
- Brief paddock summary (from paddock_content)
- **Recent work** (from `recent_work` array, last 5-10 entries):
  ```
  Recent work on this heat:
  YYYY-MM-DD HH:MM  abc123ef  [W] ₢XXXXX  Wrap description
  YYYY-MM-DD HH:MM  def456ab  [n] ₢XXXXX  Commit message
  ...
  ```
- Current pace silks and state (if present)
- Spec (the pace specification)

## Step 3: Name assessment

Before branching on state, assess whether the pace silks fits the spec:

**Assessment:**
- Read the spec content
- Consider if the kebab-case name accurately reflects the work
- If name fits: proceed silently to Step 3.5
- If mismatch detected: present 3-option prompt

**If mismatch detected:**

```
⚠ Name check: "{current_silks}" may not fit.
  Spec is about: [brief summary of actual work]
  Suggested: "{better_name}"

  [R] Rename to "{better_name}" (default)
  [C] Continue with current name
  [S] Stop

  Choice [R]:
```

**On R (or Enter):**
- Run: `./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --silks "{better_name}"`
- Report: `"Renamed to {better_name}"`
- Update pace_silks in context to reflect new name
- Continue to Step 3.5

**On C:**
- Proceed silently to Step 3.5 with current name

**On S:**
- Report: "Mount stopped at Step 3.5"
- Suggest: "Consider using `/jjc-pace-reslate` to refine the pace scope and silks"
- Stop mount

## Step 3.5: Branch on state

**If no actionable pace:**
- Report "All paces complete or abandoned"
- Suggest `/jjc-pace-slate` to add a new pace
- Suggest `/jjc-heat-groom` to review the heat
- Stop

**If pace_state is "rough":**
- Analyze the spec to understand the work
- If spec mentions dependencies, blockers, or sequencing concerns:
  - Surface these to the user as questions before proceeding
  - Do NOT investigate gallops data to validate the system's pace selection
  - If sequencing appears wrong, suggest `/jjc-heat-groom` to reorder or refine
- Read any files referenced in the pace spec
- Propose a concrete approach (2-4 bullets)
- Assess execution strategy:
  - **Bridleability**: Apply CLAUDE.md criteria (mechanical, pattern exists, no forks, bounded). If all four hold, note "This pace is bridleable" and mention `/jjc-pace-bridle` as an option.
  - **Parallelization**: Would multiple agents help? Consider:
    - File independence (same-file edits conflict)
    - Task decomposability (can work be split into independent units?)
    - Overhead vs benefit (parallel setup cost vs sequential simplicity)
  - **Model tier**: Recommend haiku (mechanical), sonnet (standard dev), or opus (architectural) based on complexity.
  - State recommendation explicitly: e.g., "Sequential haiku — single file, mechanical pattern" or "Parallel sonnet×2 — independent modules"
- Create chalk APPROACH marker: `./tt/vvw-r.RunVVX.sh jjx_chalk <PACE_CORONET> --marker A --description "<approach summary>"`
- Ask: "Ready to proceed, or would you prefer to `/jjc-pace-bridle` for autonomous execution later?"
- On approval: Begin work directly

**If pace_state is "bridled":**
- The pace has explicit direction in the direction field
- Parse direction to extract `Agent:` line (haiku/sonnet/opus)
- **Display pace details and request approval:**
  ```
  Bridled pace ready for autonomous execution:

  Pace: {pace_silks} (₢{CORONET})
  Agent: {agent_tier}

  Spec:
  {spec}

  Direction:
  {direction}

  [P] Proceed with autonomous execution (default)
  [I] Stop and work interactively
  [A] Abort mount

  Choice [P]:
  ```
- **On P (or Enter):**
  - Create chalk FLY marker: `./tt/vvw-r.RunVVX.sh jjx_chalk <PACE_CORONET> --marker F --description "Executing bridled pace via {agent} agent"`
  - **Spawn a Task agent** to execute the pace:
    - `model`: the extracted agent tier (haiku/sonnet/opus)
    - `subagent_type`: "general-purpose"
    - `prompt`: Combine spec + direction + wrap discipline (see below)
  - The spec contains the "what" (requirements, acceptance criteria); direction contains the "how" (steps, verification)
  - **Include this wrap discipline section in the agent prompt:**
    ```
    ## Wrap Discipline

    DO NOT run /jjc-pace-wrap or mark the pace complete. When work is done:
    1. Run build + tests as specified in direction
    2. Report completion status and test results
    3. STOP — the calling agent will confirm wrap with the user
    ```
  - Wait for agent completion and report outcome to user
  - Create landing commit: `echo "{agent_output}" | ./tt/vvw-r.RunVVX.sh jjx_landing <PACE_CORONET> {agent}`
  - **Do NOT auto-wrap.** Ask user: "Ready to wrap ₢<CORONET>?" and wait for confirmation before running `/jjc-pace-wrap`
- **On I:**
  - Report: "Switching to interactive mode for ₢{CORONET}"
  - Suggest: "Use `/jjc-pace-reslate` to unbridle and refine, or work directly on the pace"
  - Stop mount
- **On A:**
  - Report: "Mount aborted at Step 4"
  - Stop mount

## Context preservation

Store for use by other commands:
- Current FIREMARK (for heat-level operations)
- Current PACE_CORONET (primary identifier for pace-level operations)
- Current PACE_SILKS (for display)

## Available Operations

**Commits:** Always use `/jjc-pace-notch` — never vvx_commit directly.

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade-overview` — Heat summary
