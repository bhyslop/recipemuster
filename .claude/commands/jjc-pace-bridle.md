---
argument-hint: [coronet]
description: Bridle a pace for autonomous execution
---

Study a rough pace and prepare it for autonomous execution by adding warrant.

Arguments: $ARGUMENTS (optional Coronet; uses current pace if omitted)

## Prerequisites

- Gallops JSON must exist
- Pace must be in "rough" state (not already bridled/complete/abandoned)
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
./tt/vvw-r.RunVVX.sh jjx_orient <FIREMARK>
```

Verify the target pace is in "rough" state. If not:
- If "bridled": "Pace already bridled. Run /jjc-heat-mount to execute."
- If "complete"/"abandoned": "Pace is closed. Select another pace."

## Step 3: Study the pace

Read and analyze:
1. The docket (pace specification)
2. Any files referenced in the docket
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

## Step 5: Write warrant and transition to bridled

Once user approves the strategy, construct warrant text as a **single-line string**.

**CRITICAL: Single-line format required.**
The warrant is passed via stdin to jjx_arm. Use this format:

```
Agent: {tier} | Cardinality: {card} | Files: {list} ({N} files) | Steps: 1. {first} 2. {second} 3. {third} | Verify: {cmd}
```

**Format rules:**
- **Single line**: All content on one line, fields separated by ` | `
- **Agent**: Always specify model tier (haiku/sonnet/opus)
- **Cardinality**: "1 sequential" for single agent, "N parallel" for parallel Task agents
- **Files**: List ALL files touched, with count in parentheses
- **Steps**: Numbered inline (1. action 2. action 3. action)
- **Verify**: Build or test command to confirm success
- **No line numbers**: Never reference line numbers — they change. Use pattern references instead (function names, string literals, structural markers)
- **Shell-safe text**: Avoid characters that shells interpret: `<` `>` `&` `$` `` ` `` `(` `)`. Write prose instead of angle-bracket placeholders

**Parallelization principle**: Documentation edits can run in parallel with code edits — they don't affect build.

**Doc agent tier**: Use sonnet or opus for documentation edits, not haiku. Docs require judgment.

**Example (sequential):**
```
Agent: sonnet | Cardinality: 1 sequential | Files: vvcc_commit.rs, jjrx_cli.rs, JJSA-GallopsData.adoc (3 files) | Steps: 1. Add size_limit field to vvcc_CommitArgs 2. Add --size-limit CLI arg to NotchArgs 3. Document in JJSA | Verify: tt/vow-b.Build.sh
```

**Example (parallel):**
```
Agent: haiku | Cardinality: 14 parallel | Files: jjrc_core.rs, jjrf_favor.rs, ... (14 files) | Steps: 1. Each agent reads file, prepends copyright header, writes file | Verify: tt/vow-b.Build.sh
```

**Example (mixed — code + docs in parallel):**
```
Agent: haiku+sonnet | Cardinality: 2 parallel then build | Files: jjrx_cli.rs, jjrq_query.rs, JJSA-GallopsData.adoc (3 files) | Steps: 1. Agent A haiku adds --remaining to ParadeArgs 2. Agent B sonnet documents --remaining in JJSA 3. Sequential build | Verify: tt/vvw-b.Build.sh
```

Run:
```bash
cat <<'WARRANT' | ./tt/vvw-r.RunVVX.sh jjx_arm <CORONET>
<warrant text>
WARRANT
```

Note: The warrant is passed via stdin.

## Step 6: Confirm bridled

Report:
- "Pace **<SILKS>** (₢<CORONET>) is now **bridled**"
- If this pace is next in order: "`/jjc-heat-mount` to execute"
- If this pace is not next: "This pace is not next. Next pace is **<SILKS>** (₢<CORONET>)"

Do not offer to execute directly. Execution belongs to `/jjc-heat-mount`.

## Available Operations

**Commits:** Always use `/jjc-pace-notch` — never vvx_commit directly.

- `/jjc-heat-mount` — Begin work on next pace
- `jjx_list` — List all heats
- `/jjc-heat-groom` — Review and refine heat
- `jjx_create` — Create new heat
- `/jjc-pace-slate` — Add a new pace
- `/jjc-pace-notch` — JJ-aware git commit
- `/jjc-parade-overview` — Heat summary
