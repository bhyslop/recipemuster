---
argument-hint: <firemark-A> <firemark-B>
description: Find and consolidate overlapping paces between two heats
---

Braid two heats by finding overlapping paces and consolidating them. This is an intersection operation, not migration — both heats may continue with remaining work.

Arguments: $ARGUMENTS (format: `<firemark-A> <firemark-B>`)

## Core Model

**Intersection, not migration:** Braid identifies overlapping work between two heats and consolidates duplicate effort. Order of arguments does not imply direction.

**Pace classifications:**
- **Already done**: Steeplechase shows completion evidence → abandon with citation
- **Overlap**: Semantic match found in other heat → reslate keeper, abandon duplicate
- **Distinct**: Fits its heat, no overlap → leave in place
- **Soggy**: Ill-formed spec that prevents classification → flag for human decision

**Tiered architecture:**
- **Haiku pass**: Correlate paces across heats, check steeplechase for completion
- **Opus pass**: Classify paces, recommend outcomes, orchestrate ceremony

## Prerequisites

- Gallops JSON must exist
- Both Firemarks must exist
- Both heats should be in racing state (warn if stabled)

## Step 1: Parse arguments

Extract from $ARGUMENTS:
- First positional: Firemark A
- Second positional: Firemark B

**If fewer than 2 arguments:**
- Error: "Usage: /jjc-heat-braid <firemark-A> <firemark-B>"
- Example: `/jjc-heat-braid ₣AF ₣AG`

**If arguments identical:**
- Error: "Cannot braid a heat with itself. Provide two different Firemarks."

## Step 2: Load both heats

Run parade for both heats:
```bash
PARADE_A=$(./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK_A> --remaining)
PARADE_B=$(./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK_B> --remaining)
```

Extract from each:
- Heat silks (from header)
- Progress stats (complete, abandoned, remaining counts)
- List of remaining pace Coronets

**If either heat not found:**
- Error: "Heat not found: ₣{FIREMARK}" and stop

**If either heat is stabled:**
- Warn: "⚠ Heat ₣{FIREMARK} ({SILKS}) is stabled. Consider `/jjc-heat-furlough --racing` before braiding."
- Ask user to confirm proceeding or abort

**If both heats have no remaining paces:**
- Report: "Both heats have no remaining paces. Nothing to braid."
- Stop

## Step 3: Assess gestalt compatibility

Read paddock content for both heats:
```bash
PADDOCK_A=$(./tt/vvw-r.RunVVX.sh jjx_paddock <FIREMARK_A>)
PADDOCK_B=$(./tt/vvw-r.RunVVX.sh jjx_paddock <FIREMARK_B>)
```

Analyze both heat gestalts:
- What is the coherent goal of heat A?
- What is the coherent goal of heat B?
- Do they share problem space or are they orthogonal?

Present gestalt assessment to user:
```
Heat A (₣{FA}): {silks-A}
  Goal: {coherent goal summary A}
  Remaining: {N} paces

Heat B (₣{FB}): {silks-B}
  Goal: {coherent goal summary B}
  Remaining: {M} paces

Gestalt overlap: {HIGH | MODERATE | LOW | NONE}
  {1-2 sentence explanation of why}
```

**If gestalt overlap is NONE:**
- Warn: "These heats appear orthogonal. Braiding may not find meaningful overlap."
- Ask user to confirm proceeding or abort

## Step 4: Haiku correlation pass

Fetch full dockets for all remaining paces in both heats:
```bash
# For each coronet in PARADE_A and PARADE_B
DOCKET=$(./tt/vvw-r.RunVVX.sh jjx_get_brief <CORONET>)
```

Load steeplechase history for both heats:
```bash
REIN_A=$(./tt/vvw-r.RunVVX.sh jjx_log <FIREMARK_A> --limit 100)
REIN_B=$(./tt/vvw-r.RunVVX.sh jjx_log <FIREMARK_B> --limit 100)
```

**Task for Haiku:** For each pace in heat A and each pace in heat B:
1. **Completion check**: Does steeplechase show this work already done in either heat?
   - Look for commit messages, chalk entries, wrap events mentioning similar work
   - If found, mark as "already_done" with citation (commit hash or chalk text)

2. **Correlation analysis**: Does this pace pair represent the same work?
   - Compare semantic intent, not just text similarity
   - Assign correlation score: STRONG | MODERATE | WEAK | NONE
   - Provide 1-sentence rationale

**Haiku output format** (structured data for Opus pass):
```
Pace ₢{A-coronet} ({A-silks}) vs ₢{B-coronet} ({B-silks}):
  Completion: {DONE_IN_A | DONE_IN_B | INCOMPLETE}
  Citation: {commit hash or chalk text if done}
  Correlation: {STRONG | MODERATE | WEAK | NONE}
  Rationale: {1-sentence explanation}
```

Run Haiku pass in a single block analyzing all pace pairs.

## Step 5: Opus classification pass

**Task for Opus:** Using Haiku correlation data, classify each pace into one of four categories:

**Already done:**
- Steeplechase shows completion evidence
- Action: Abandon with citation

**Overlap:**
- STRONG or MODERATE correlation with pace in other heat
- Action: Pick keeper (better docket, more recent tack, or first by order), reslate keeper with merged context, abandon duplicate

**Distinct:**
- WEAK or NONE correlation with all paces in other heat
- Fits its heat's gestalt
- Action: Leave in place

**Soggy:**
- Docket too vague to assess correlation
- Action: Flag for human grooming

**Opus judgment calls:**
- When correlation is MODERATE, use gestalt fit and docket quality to decide overlap vs distinct
- When multiple paces in one heat overlap with single pace in other, group them
- Consider dependency chains (if pace A overlaps but pace B depends on A's output)

**Opus output format:**
```
Heat A (₣{FA}) paces:
  ₢{coronet} ({silks}):
    Class: {ALREADY_DONE | OVERLAP | DISTINCT | SOGGY}
    Action: {specific action}
    Reason: {1-2 sentence explanation}

Heat B (₣{FB}) paces:
  ₢{coronet} ({silks}):
    Class: {ALREADY_DONE | OVERLAP | DISTINCT | SOGGY}
    Action: {specific action}
    Reason: {1-2 sentence explanation}
```

## Step 6: Present summary

Display classification counts:
```
Braid summary for ₣{FA} ({silks-A}) × ₣{FB} ({silks-B}):

  Already done: {N} paces ({X} from A, {Y} from B)
  Overlaps: {M} pace pairs
  Distinct: {P} paces ({Q} from A, {R} from B)
  Soggy: {S} paces ({T} from A, {U} from B)
```

**If all distinct (no overlaps, no already-done, no soggy):**
- Report: "No intersection found. These heats have orthogonal work."
- Stop

**If all soggy:**
- Report: "All paces are soggy (ill-formed dockets). Recommend `/jjc-heat-groom` for both heats before braiding."
- Stop

## Step 7: Walk through findings with user approval

For each category with findings, walk through in this order:

### 7a: Already done paces

For each already-done pace:
```
₢{coronet} ({silks}) — Already done
  Evidence: {citation from steeplechase}

  Abandon with citation? [Y/n]:
```

**On user approval (Y or Enter):**
```bash
./tt/vvw-r.RunVVX.sh jjx_drop <CORONET> abandoned
```

Add chalk entry recording the abandonment reason:
```bash
cat <<'CHALKNOTE' | ./tt/vvw-r.RunVVX.sh jjx_mark <FIREMARK> --note -
Braid: Abandoned ₢{coronet} ({silks}) — already done per {citation}
CHALKNOTE
```

**On user decline (n):**
- Skip abandonment
- Mark pace for manual review

### 7b: Overlapping paces

For each overlap pair (or group):
```
Overlap: ₢{A-coronet} ({A-silks}) ≈ ₢{B-coronet} ({B-silks})
  Correlation: {STRONG | MODERATE}
  Reason: {rationale}

  Keeper: ₢{keeper-coronet} ({keeper-silks})
  Duplicate: ₢{dup-coronet} ({dup-silks})

  Consolidate? [Y/n]:
```

**On user approval (Y or Enter):**

1. **Merge context into keeper:**
   - Read keeper docket: `./tt/vvw-r.RunVVX.sh jjx_get_brief <KEEPER_CORONET>`
   - Read duplicate docket: `./tt/vvw-r.RunVVX.sh jjx_get_brief <DUP_CORONET>`
   - Synthesize merged docket preserving best details from both
   - Apply reslate to keeper:
   ```bash
   cat <<'DOCKET' | ./tt/vvw-r.RunVVX.sh jjx_revise_docket <KEEPER_CORONET>
   [Merged from ₢{dup-coronet} during braid]

   {synthesized merged docket}
   DOCKET
   ```

2. **Abandon duplicate:**
   ```bash
   ./tt/vvw-r.RunVVX.sh jjx_drop <DUP_CORONET> abandoned
   ```

3. **Chalk both heats:**
   ```bash
   cat <<'CHALKNOTE' | ./tt/vvw-r.RunVVX.sh jjx_mark <KEEPER_FIREMARK> --note -
   Braid: Merged ₢{dup-coronet} from ₣{dup-firemark} into ₢{keeper-coronet}
   CHALKNOTE

   cat <<'CHALKNOTE' | ./tt/vvw-r.RunVVX.sh jjx_mark <DUP_FIREMARK> --note -
   Braid: Abandoned ₢{dup-coronet} (overlap with ₢{keeper-coronet} in ₣{keeper-firemark})
   CHALKNOTE
   ```

**On user decline (n):**
- Skip consolidation
- Leave both paces distinct

### 7c: Soggy paces

For each soggy pace:
```
₢{coronet} ({silks}) — Soggy spec
  Reason: {why spec is ill-formed}

  Flag for grooming? [Y/n]:
```

**On user approval (Y or Enter):**
```bash
cat <<'CHALKNOTE' | ./tt/vvw-r.RunVVX.sh jjx_mark <FIREMARK> --note -
Braid: Flagged ₢{coronet} ({silks}) for grooming — {reason}
CHALKNOTE
```

Report: "Recommend `/jjc-heat-groom ₣{firemark}` to refine soggy paces."

**On user decline (n):**
- Skip flagging

### 7d: Distinct paces

Report distinct pace counts but do not walk through:
```
{N} distinct paces remain in ₣{FA}
{M} distinct paces remain in ₣{FB}
These paces fit their respective heats and have no overlap.
```

## Step 8: Paddock maintenance

After processing all findings, run paddock leveling for both heats:
```bash
./tt/vvw-r.RunVVX.sh jjx_paddock <FIREMARK_A> --level
./tt/vvw-r.RunVVX.sh jjx_paddock <FIREMARK_B> --level
```

This removes obsolete references to abandoned paces and adjusts paddock content to match new heat state.

## Step 9: Gestalt reassessment

After consolidation, reassess each heat's gestalt:

For each heat, check:
- Did the braid change the heat's focus?
- Do the remaining paces still fit under the original silks?

**If gestalt shifted:**
- Suggest new silks based on refined focus
- Present 3-option prompt:

```
⚠ Heat ₣{firemark} gestalt check: "{old-silks}" may not fit after braid.
  Was: {old focus summary}
  Now: {new focus summary}
  Suggested: "{new-silks}"

  [R] Rename to "{new-silks}" (default)
  [C] Continue with current name
  [S] Skip rename for now

  Choice [R]:
```

**On user response:**
- **R** (or Enter): Rename heat silks (note: this requires adding heat rename primitive — flag for now)
- **C**: Keep current silks
- **S**: Skip rename

**Note:** Heat rename is not yet implemented in jjx. For now, record suggested rename in chalk:
```bash
cat <<'CHALKNOTE' | ./tt/vvw-r.RunVVX.sh jjx_mark <FIREMARK> --note -
Braid: Suggested rename "{old-silks}" → "{new-silks}" (gestalt shifted)
CHALKNOTE
```

## Step 10: Empty heat handling

After braid, check if either heat is now empty (all paces complete/abandoned):

For each heat:
```bash
REMAINING=$(./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK> --remaining | tail -n +2 | wc -l)
```

**If REMAINING is 0:**
```
Heat ₣{firemark} ({silks}) is now empty (all paces complete or abandoned).

Retire heat? [Y/n]:
```

**On user approval (Y or Enter):**
- Invoke `/jjc-heat-retire <firemark>` (delegates to retire ceremony)

**On user decline (n):**
- Report: "Heat remains racing. Use `/jjc-heat-retire ₣{firemark}` when ready."

## Step 11: Summary with references

Display final summary:
```
Braid complete for ₣{FA} ({silks-A}) × ₣{FB} ({silks-B})

Actions taken:
  - Abandoned: {N} already-done paces
  - Consolidated: {M} overlap pairs
  - Flagged: {S} soggy paces
  - Distinct: {P} paces remain (unchanged)

Heat A (₣{FA}): {X} paces remaining
Heat B (₣{FB}): {Y} paces remaining

Steeplechase references:
  ₣{FA}: See `/jjc-heat-rein {FA}` for braid chalk entries
  ₣{FB}: See `/jjc-heat-rein {FB}` for braid chalk entries

Next steps:
  - `/jjc-heat-mount ₣{FA}` — Continue heat A
  - `/jjc-heat-mount ₣{FB}` — Continue heat B
  - `/jjc-heat-groom ₣{X}` — Refine soggy paces (if any)
```

## Error handling

Common errors:
- "Heat not found" — invalid Firemark
- "Cannot braid a heat with itself" — identical Firemarks
- "No remaining paces" — nothing to braid

## Available Operations

**Commits:** Always use `/jjc-pace-notch` — never vvx_commit directly.

- `/jjc-heat-mount` — Begin work on a heat
- `/jjc-heat-groom` — Review and refine heat
- `/jjc-heat-rein` — View steeplechase history
- `/jjc-heat-retire` — Retire completed heat
- `/jjc-parade` — Heat or pace details
