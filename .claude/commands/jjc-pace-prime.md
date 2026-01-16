---
argument-hint: [coronet]
description: Prime a pace for autonomous execution
---

Study a rough pace and prepare it for autonomous execution by adding direction.

Arguments: $ARGUMENTS (optional Coronet; uses current pace if omitted)

## Prerequisites

- Gallops JSON must exist
- Pace must be in "rough" state (not already primed/complete/abandoned)
- Should have run `/jjc-heat-mount` first to establish context

## Step 1: Identify target pace

**If $ARGUMENTS contains a Coronet (e.g., `AAAAC` or `₢AAAAC`):**
- Extract Firemark from first 2 characters
- Use that Coronet directly

**If $ARGUMENTS is empty:**
- Use PACE_CORONET from current context
- If no context, error: "No pace context. Run /jjc-heat-mount first."

## Step 2: Verify pace state

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_saddle <FIREMARK>
```

Verify the target pace is in "rough" state. If not:
- If "primed": "Pace already primed. Run /jjc-heat-mount to execute."
- If "complete"/"abandoned": "Pace is closed. Select another pace."

## Step 3: Study the pace

Read and analyze:
1. The tack_text (pace specification)
2. Any files referenced in the spec
3. The paddock_content for broader context

## Step 4: Recommend execution strategy

Based on your analysis, recommend:

**Agent type:**
- `haiku` — Simple, mechanical tasks (formatting, renames, straightforward edits)
- `sonnet` — Standard development tasks (features, bug fixes, refactoring)
- `opus` — Complex architectural work, multi-file coordination, nuanced decisions

**Execution notes:**
- Whether parallel agents could be used
- Key files to read first
- Potential risks or decision points

Present recommendation to user and ask for approval or adjustments.

## Step 5: Write direction and transition to primed

Once user approves the strategy, construct direction text using this structured format:

```
Agent: <haiku|sonnet|opus>
Cardinality: <1 sequential | N parallel>
Files: <file1.rs, file2.rs, ...> (N files)
Steps:
1. <first action>
2. <second action>
...
Verify: <build/test command>
```

**Format rules:**
- **Agent**: Always specify model tier (haiku/sonnet/opus)
- **Cardinality**: "1 sequential" for single agent, "N parallel" for parallel Task agents
- **Files**: List ALL files touched, with count in parentheses
- **Steps**: Numbered, scannable actions
- **Verify**: Build or test command to confirm success

**Example (sequential):**
```
Agent: sonnet
Cardinality: 1 sequential
Files: vvcc_commit.rs, jjrx_cli.rs, JJD-GallopsData.adoc (3 files)
Steps:
1. Add size_limit field to vvcc_CommitArgs
2. Add --size-limit CLI arg to NotchArgs
3. Document in JJD
Verify: cargo build --features jjk
```

**Example (parallel):**
```
Agent: haiku
Cardinality: 14 parallel
Files: jjrc_core.rs, jjrf_favor.rs, ... (14 files)
Steps:
1. Each agent: read file, prepend copyright header, write file
Verify: cargo build --features jjk
```

Run:
```bash
./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --state primed --direction "<direction text>"
```

Note: `--direction` is a string argument, not stdin. The text field is inherited when stdin is empty.

## Step 6: Confirm primed

Report:
- "Pace <SILKS> is now primed"
- "Run /jjc-heat-mount to begin autonomous execution"
- Or: "Ready to execute now?" → if yes, proceed as /jjc-heat-mount would for primed pace

## Available Operations

- `/jjc-heat-mount` — Begin work on next pace
- `/jjc-heat-muster` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-nominate` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade-overview` — Heat summary
