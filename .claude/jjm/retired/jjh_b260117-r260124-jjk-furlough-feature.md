# Heat Trophy: jjk-furlough-feature

**Firemark:** ₣AC
**Created:** 260117
**Retired:** 260124
**Status:** retired

> NOTE: JJSA renamed to JJS0 (top-level spec '0' suffix convention). Filename references in this trophy are historical.

## Paddock

# Paddock: jjk-furlough-feature

## Context

Add heat lifecycle management: the ability to pause heats ("stabled") and resume them ("racing"), plus heat renaming.

**Problem:** With multiple concurrent heats, there's no way to temporarily set aside a heat while focusing on another. The current binary (current/retired) doesn't support "I'll come back to this later."

**Solution:** Introduce `jjx_furlough` command with:
- `--stabled` / `--racing` flags to toggle heat status
- `--silks` flag to rename heats
- Lazy migration from "current" → "racing" terminology

**Behavior changes:**
- `mount` filters to racing heats only (execution focus)
- `groom`, `slate`, `reslate`, `restring` show all heats (planning can happen on stabled heats)
- `saddle` fails on stabled heats (can't execute work on paused heat)

## Architecture

Three-pace arc following standard JJK feature pattern:
1. **Spec** (₢ACAAA) — Update JJSA-GallopsData.adoc with new concepts
2. **Impl** (₢ACAAB) — Rust implementation in jjrg/jjrx/jjrq modules
3. **Commands** (₢ACAAC) — Slash command + updates to existing commands

## References

- Tools/jjk/vov_veiled/JJSA-GallopsData.adoc — JJSA spec (target for pace 1)
- Tools/vok/vov_veiled/RCG-RustCodingGuide.md — Rust naming conventions for new code
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs — Heat/Pace structs, HeatStatus enum
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — CLI command dispatch
- Tools/jjk/vov_veiled/src/jjrq_query.rs — Query operations (saddle, muster)

## Key Constraints

1. **Lazy migration** — Accept "current" on read, write "racing". No batch migration needed.
2. **Serde compatibility** — Use `#[serde(alias = "current")]` for backwards compat.
3. **RCG compliance** — New functions use `jjrg_`, `jjrx_`, `jjrq_` prefixes per module.

## Steeplechase

### 2026-01-17 - Heat Created

Restrung 3 furlough paces from ₣AA (vok-fresh-install-release) to focus that heat on MVP delivery.

## Paces

### muster-remaining-sort (₢ACAAE) [complete]

**[260118-2139] complete**

Muster now sorts racing heats first and shows Remaining/Total columns. JJD spec and jjrq_query.rs implementation updated.

**[260118-2136] rough**

Update jjx_muster output format:

1. **Sort order**: Racing heats first, then stabled (then retired if any)

2. **Replace Paces column** with two columns:
   - **Remaining**: Actionable paces count (rough + bridled)
   - **Total**: Total pace count

Output format (TSV):
```
FIREMARK<TAB>SILKS<TAB>STATUS<TAB>REMAINING<TAB>TOTAL
```

Files:
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (update jjdo_muster spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (update jjrq_run_muster implementation)

**[260118-2134] bridled**

Update jjx_muster output format:

1. **Sort order**: Racing heats first, then stabled (then retired if any)

2. **Replace Paces column** with two columns:
   - **Remaining**: Actionable paces count (rough + bridled)
   - **Total**: Total pace count

Output format (TSV):
```
FIREMARK<TAB>SILKS<TAB>STATUS<TAB>REMAINING<TAB>TOTAL
```

Files:
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (update jjdo_muster spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (update jjrq_run_muster implementation)

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc, Tools/jjk/vov_veiled/src/jjrq_query.rs (2 files)
Steps:
1. Read jjrq_query.rs to find jjrq_run_muster function
2. Update JJD-GallopsData.adoc jjdo_muster spec: add sort order (racing first) and new columns (Remaining, Total)
3. Update jjrq_run_muster:
   - Add sort: racing heats first, then stabled, then retired
   - Replace single paces count with two: remaining (rough + bridled) and total
   - Adjust output format to match new TSV spec
4. Build: cargo build --manifest-path Tools/vok/Cargo.toml
5. Test: ./tt/vvw-r.RunVVX.sh jjx_muster
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260118-2131] bridled**

Update jjx_muster output format:

1. **Sort order**: Racing heats first, then stabled (then retired if any)

2. **Replace Paces column** with two columns:
   - **Remaining**: Actionable paces count (rough + bridled)
   - **Total**: Total pace count

Output format (TSV):
```
FIREMARK<TAB>SILKS<TAB>STATUS<TAB>REMAINING<TAB>TOTAL
```

Files:
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (update jjdo_muster spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (update jjrq_run_muster implementation)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc, Tools/jjk/vov_veiled/src/jjrq_query.rs (2 files)
Steps:
1. Read jjrq_query.rs to find jjrq_run_muster function
2. Update JJD-GallopsData.adoc jjdo_muster spec: add sort order (racing first) and new columns (Remaining, Total)
3. Update jjrq_run_muster:
   - Add sort: racing heats first, then stabled, then retired
   - Replace single paces count with two: remaining (rough + bridled) and total
   - Adjust output format to match new TSV spec
4. Build: cargo build --manifest-path Tools/vok/Cargo.toml
5. Test: ./tt/vvw-r.RunVVX.sh jjx_muster
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260118-2111] rough**

Update jjx_muster output format:

1. **Sort order**: Racing heats first, then stabled (then retired if any)

2. **Replace Paces column** with two columns:
   - **Remaining**: Actionable paces count (rough + bridled)
   - **Total**: Total pace count

Output format (TSV):
```
FIREMARK<TAB>SILKS<TAB>STATUS<TAB>REMAINING<TAB>TOTAL
```

Files:
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (update jjdo_muster spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (update jjrq_run_muster implementation)

### furlough-jjd-spec (₢ACAAA) [complete]

**[260118-2043] complete**

Updated JJD-GallopsData.adoc with furlough concepts: added racing/stabled enum values, jjdo_furlough operation spec, updated saddle to error on stabled heats, removed muster --status filter.

**[260118-2039] bridled**

Update JJD-GallopsData.adoc specification with final-form furlough concepts (no migration language):

1. Add HeatStatus enum values:
   - jjdhe_racing: Heat is actively being worked
   - jjdhe_stabled: Heat is paused, not actively worked
   - (jjdhe_retired remains unchanged)

2. Add jjdo_furlough operation:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required
   - Errors if already in target state
   - Errors if heat is retired (terminal)

3. Update jjdo_saddle:
   - Add: Fails with error if heat status is stabled

4. Update jjdo_muster:
   - Remove --status filter (show all heats, no filtering)

NOTE: The spec describes the target state. Implementation may temporarily accept legacy values during migration (see ₢ACAAB, ₢ACAAD).

Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc (1 file)
Steps:
1. Add jjdhe_racing and jjdhe_stabled to mapping section (after jjdhe_retired line)
2. Add jjdhe_racing and jjdhe_stabled anchor definitions in Status Values section (after jjdhe_retired definition, before === {jjdpr_pace})
3. Update jjdhm_status definition to list racing/stabled/retired values
4. Add jjdo_furlough to operations mapping section (after jjdo_draft)
5. Add [[jjdo_furlough]] operation spec as new section under Write Operations (after jjdo_retire)
6. Update [[jjdo_saddle]] behavior to add step: error if heat status is stabled
7. Update [[jjdo_muster]] to remove --status argument and filtering behavior
Verify: File reads correctly with no AsciiDoc syntax errors

**[260118-2025] rough**

Update JJD-GallopsData.adoc specification with final-form furlough concepts (no migration language):

1. Add HeatStatus enum values:
   - jjdhe_racing: Heat is actively being worked
   - jjdhe_stabled: Heat is paused, not actively worked
   - (jjdhe_retired remains unchanged)

2. Add jjdo_furlough operation:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required
   - Errors if already in target state
   - Errors if heat is retired (terminal)

3. Update jjdo_saddle:
   - Add: Fails with error if heat status is stabled

4. Update jjdo_muster:
   - Remove --status filter (show all heats, no filtering)

NOTE: The spec describes the target state. Implementation may temporarily accept legacy values during migration (see ₢ACAAB, ₢ACAAD).

Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc

**[260117-1406] rough**

Drafted from ₢AAABB in ₣AA.

Update JJD-GallopsData.adoc specification:

1. Add new status enum value:
   - jjdhe_stabled: Heat is paused, not actively worked

2. Rename existing enum value (lazy migration):
   - jjdhe_current → jjdhe_racing (accept "current" on read, write "racing")

3. Add jjdo_furlough operation:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required
   - Errors if already in target state
   - Errors if heat is retired (terminal)

4. Update jjdo_saddle:
   - Add: Fails with error if heat status is stabled

5. Update jjdo_muster:
   - Remove --status filter (show all heats, no filtering)

Files: Tools/jjk/JJD-GallopsData.adoc

**[260117-1129] rough**

Update JJD-GallopsData.adoc specification:

1. Add new status enum value:
   - jjdhe_stabled: Heat is paused, not actively worked

2. Rename existing enum value (lazy migration):
   - jjdhe_current → jjdhe_racing (accept "current" on read, write "racing")

3. Add jjdo_furlough operation:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required
   - Errors if already in target state
   - Errors if heat is retired (terminal)

4. Update jjdo_saddle:
   - Add: Fails with error if heat status is stabled

5. Update jjdo_muster:
   - Remove --status filter (show all heats, no filtering)

Files: Tools/jjk/JJD-GallopsData.adoc

### furlough-rust-impl (₢ACAAB) [complete]

**[260118-2054] complete**

Implemented furlough feature in Rust: added Racing/Stabled to HeatStatus enum with serde alias for migration, implemented jjx_furlough command, updated saddle to error on stabled heats, removed muster --status filter. All 134 tests pass.

**[260118-2025] rough**

Implement furlough feature in Rust:

1. Update HeatStatus enum in jjrg_gallops.rs:
   - Add HeatStatus::Racing variant
   - Add HeatStatus::Stabled variant
   - TEMPORARY: Add #[serde(alias = "current")] to Racing for migration
   - Serialize always writes "racing" (never "current")
   - NOTE: The "current" alias will be removed in ₢ACAAD after migration completes

2. Implement jjx_furlough command in jjrx_cli.rs:
   - Args: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required (validate)
   - Check heat exists and is not retired
   - If --racing/--stabled: verify not already in target state, update status
   - If --silks: update heat silks
   - Atomic write

3. Update jjx_saddle in jjrq_query.rs:
   - Check heat status before returning saddle context
   - If stabled: return error "Heat is stabled, cannot saddle"

4. Update jjx_muster in jjrq_query.rs:
   - Remove --status filter argument
   - Always return all heats (racing + stabled)

Files: Tools/jjk/vov_veiled/src/jjrg_gallops.rs, jjrx_cli.rs, jjrq_query.rs

**[260117-1406] rough**

Drafted from ₢AAABC in ₣AA.

Implement furlough feature in Rust:

1. Lazy migration in jjrg_gallops.rs:
   - Deserialize: accept both "current" and "racing" as valid HeatStatus
   - Serialize: always write "racing" (never "current")
   - Add HeatStatus::Stabled variant

2. Implement jjx_furlough command in jjrx_cli.rs:
   - Args: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required (validate)
   - Check heat exists and is not retired
   - If --racing/--stabled: verify not already in target state, update status
   - If --silks: update heat silks
   - Atomic write

3. Update jjx_saddle in jjrq_query.rs:
   - Check heat status before returning saddle context
   - If stabled: return error "Heat is stabled, cannot saddle"

4. Update jjx_muster in jjrq_query.rs:
   - Remove --status filter argument
   - Always return all heats (racing + stabled)

Files: Tools/jjk/veiled/src/jjrg_gallops.rs, jjrx_cli.rs, jjrq_query.rs

**[260117-1129] rough**

Implement furlough feature in Rust:

1. Lazy migration in jjrg_gallops.rs:
   - Deserialize: accept both "current" and "racing" as valid HeatStatus
   - Serialize: always write "racing" (never "current")
   - Add HeatStatus::Stabled variant

2. Implement jjx_furlough command in jjrx_cli.rs:
   - Args: <firemark> [--racing | --stabled] [--silks <new-name>]
   - At least one option required (validate)
   - Check heat exists and is not retired
   - If --racing/--stabled: verify not already in target state, update status
   - If --silks: update heat silks
   - Atomic write

3. Update jjx_saddle in jjrq_query.rs:
   - Check heat status before returning saddle context
   - If stabled: return error "Heat is stabled, cannot saddle"

4. Update jjx_muster in jjrq_query.rs:
   - Remove --status filter argument
   - Always return all heats (racing + stabled)

Files: Tools/jjk/veiled/src/jjrg_gallops.rs, jjrx_cli.rs, jjrq_query.rs

### furlough-slash-mount (₢ACAAC) [complete]

**[260118-2107] complete**

Created jjc-heat-furlough slash command, updated mount to filter racing heats with 0-heats guidance, updated groom/slate/reslate/restring to show all heats, updated vocjjmc_core.md with furlough command and Quick Verbs.

**[260118-2101] bridled**

Create slash command and update mount/groom behavior:

1. Create /jjc-heat-furlough slash command:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - Document all options with examples
   - Call: ./tt/vvw-r.RunVVX.sh jjx_furlough <firemark> [options]
   - Report new status and/or new silks on success

2. Update /jjc-heat-mount:
   - Filter to racing heats only (--status racing)
   - If exactly 1 racing heat: auto-proceed without prompting
   - If 0 racing heats: error suggesting check stabled heats or use furlough
   - If 2+ racing heats: prompt for selection (existing behavior)

3. Update /jjc-heat-groom:
   - REMOVE --status filter (show all heats)
   - Allow grooming stabled heats
   - Mention in output if heat is stabled

4. Update /jjc-pace-slate, /jjc-pace-reslate, /jjc-heat-restring:
   - REMOVE --status filter (show all heats)
   - Can operate on stabled heats (planning ahead)

5. Update vocjjmc_core.md template (CLAUDE.md section for JJK installs):
   - Add to command table: "Pause/resume heat | /jjc-heat-furlough"
   - Add to Quick Verbs: "furlough | /jjc-heat-furlough"
   - Update Concepts to mention stabled/racing status

PATTERN: Execution (mount) filters to racing; Planning (groom, slate, reslate, restring) shows all.

Files: 
- .claude/commands/jjc-heat-furlough.md (new)
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-groom.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-heat-restring.md
- Tools/jjk/vov_veiled/vocjjmc_core.md

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjc-heat-furlough.md (new), jjc-heat-mount.md, jjc-heat-groom.md, jjc-pace-slate.md, jjc-pace-reslate.md, jjc-heat-restring.md, vocjjmc_core.md (7 files)
Steps:
1. Create .claude/commands/jjc-heat-furlough.md following jjc-heat-nominate pattern: frontmatter, description, args, steps calling jjx_furlough
2. Update jjc-heat-mount.md Step 1: change muster call to filter racing only, add 0-racing-heats error guidance
3. Update jjc-heat-groom.md Step 1: remove --status current from muster call, note stabled heats in display
4. Update jjc-pace-slate.md Step 2: remove --status current from muster call
5. Update jjc-pace-reslate.md: remove --status current from muster call (if present)
6. Update jjc-heat-restring.md Step 2: remove --status current from muster call
7. Update vocjjmc_core.md: add furlough to command table, Quick Verbs, update Concepts with racing/stabled
Verify: Manual review of markdown syntax

**[260118-2039] rough**

Create slash command and update mount/groom behavior:

1. Create /jjc-heat-furlough slash command:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - Document all options with examples
   - Call: ./tt/vvw-r.RunVVX.sh jjx_furlough <firemark> [options]
   - Report new status and/or new silks on success

2. Update /jjc-heat-mount:
   - Filter to racing heats only (--status racing)
   - If exactly 1 racing heat: auto-proceed without prompting
   - If 0 racing heats: error suggesting check stabled heats or use furlough
   - If 2+ racing heats: prompt for selection (existing behavior)

3. Update /jjc-heat-groom:
   - REMOVE --status filter (show all heats)
   - Allow grooming stabled heats
   - Mention in output if heat is stabled

4. Update /jjc-pace-slate, /jjc-pace-reslate, /jjc-heat-restring:
   - REMOVE --status filter (show all heats)
   - Can operate on stabled heats (planning ahead)

5. Update vocjjmc_core.md template (CLAUDE.md section for JJK installs):
   - Add to command table: "Pause/resume heat | /jjc-heat-furlough"
   - Add to Quick Verbs: "furlough | /jjc-heat-furlough"
   - Update Concepts to mention stabled/racing status

PATTERN: Execution (mount) filters to racing; Planning (groom, slate, reslate, restring) shows all.

Files: 
- .claude/commands/jjc-heat-furlough.md (new)
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-groom.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-heat-restring.md
- Tools/jjk/vov_veiled/vocjjmc_core.md

**[260117-1406] rough**

Drafted from ₢AAABD in ₣AA.

Create slash command and update mount/groom behavior:

1. Create /jjc-heat-furlough slash command:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - Document all options with examples
   - Call: ./tt/vvw-r.RunVVX.sh jjx_furlough <firemark> [options]
   - Report new status and/or new silks on success
   - Auto-commit via vvx_commit

2. Update /jjc-heat-mount:
   - Filter to racing heats only (--status racing)
   - If exactly 1 racing heat: auto-proceed without prompting
   - If 0 racing heats: error suggesting check stabled heats or use furlough
   - If 2+ racing heats: prompt for selection (existing behavior)

3. Update /jjc-heat-groom:
   - REMOVE --status filter (show all heats)
   - Allow grooming stabled heats
   - Mention in output if heat is stabled

4. Update /jjc-pace-slate:
   - REMOVE --status filter (show all heats)
   - Can add paces to stabled heats (planning ahead)

5. Update /jjc-pace-reslate:
   - REMOVE --status filter (show all heats)
   - Can refine paces in stabled heats

6. Update /jjc-heat-restring:
   - REMOVE --status filter (show all heats)
   - Can move paces to/from stabled heats

PATTERN: Execution (mount) filters to racing; Planning (groom, slate, reslate, restring) shows all.

Files: .claude/commands/jjc-heat-furlough.md (new), jjc-heat-mount.md, jjc-heat-groom.md, jjc-pace-slate.md, jjc-pace-reslate.md, jjc-heat-restring.md

**[260117-1133] rough**

Create slash command and update mount/groom behavior:

1. Create /jjc-heat-furlough slash command:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - Document all options with examples
   - Call: ./tt/vvw-r.RunVVX.sh jjx_furlough <firemark> [options]
   - Report new status and/or new silks on success
   - Auto-commit via vvx_commit

2. Update /jjc-heat-mount:
   - Filter to racing heats only (--status racing)
   - If exactly 1 racing heat: auto-proceed without prompting
   - If 0 racing heats: error suggesting check stabled heats or use furlough
   - If 2+ racing heats: prompt for selection (existing behavior)

3. Update /jjc-heat-groom:
   - REMOVE --status filter (show all heats)
   - Allow grooming stabled heats
   - Mention in output if heat is stabled

4. Update /jjc-pace-slate:
   - REMOVE --status filter (show all heats)
   - Can add paces to stabled heats (planning ahead)

5. Update /jjc-pace-reslate:
   - REMOVE --status filter (show all heats)
   - Can refine paces in stabled heats

6. Update /jjc-heat-restring:
   - REMOVE --status filter (show all heats)
   - Can move paces to/from stabled heats

PATTERN: Execution (mount) filters to racing; Planning (groom, slate, reslate, restring) shows all.

Files: .claude/commands/jjc-heat-furlough.md (new), jjc-heat-mount.md, jjc-heat-groom.md, jjc-pace-slate.md, jjc-pace-reslate.md, jjc-heat-restring.md

**[260117-1129] rough**

Create slash command and update mount/groom behavior:

1. Create /jjc-heat-furlough slash command:
   - Arguments: <firemark> [--racing | --stabled] [--silks <new-name>]
   - Document all options with examples
   - Call: ./tt/vvw-r.RunVVX.sh jjx_furlough <firemark> [options]
   - Report new status and/or new silks on success
   - Auto-commit via vvx_commit

2. Update /jjc-heat-mount:
   - When checking current heats, filter to racing only (exclude stabled)
   - If exactly 1 racing heat: auto-proceed without prompting
   - If 0 racing heats: error with suggestion to check stabled heats
   - If 2+ racing heats: prompt for selection (existing behavior)

3. Update /jjc-heat-groom:
   - Allow grooming stabled heats (no status check)
   - Mention in output if heat is stabled

Files: .claude/commands/jjc-heat-furlough.md (new), jjc-heat-mount.md, jjc-heat-groom.md

### furlough-remove-current-alias (₢ACAAD) [complete]

**[260118-2142] complete**

Removed serde alias for 'current' - schema now strictly requires 'racing' status value.

**[260118-2035] bridled**

Remove #[serde(alias = "current")] from HeatStatus deserialization in jjrg_gallops.rs. Before removing, verify all heat JSON files in current/ and retired/ directories use "racing" (not "current"). This hardens the schema to match the final-form JJD spec.

Files: Tools/jjk/vov_veiled/src/jjrg_gallops.rs

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/src/jjrg_gallops.rs (1 file)
Steps:
1. Grep for "current" in .claude/jjm/current/*.json and .claude/jjm/retired/*.json
2. If any "current" values found, abort with error listing files needing migration
3. If no "current" found, remove the #[serde(alias = "current")] attribute from HeatStatus::Racing
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260118-2025] rough**

Remove #[serde(alias = "current")] from HeatStatus deserialization in jjrg_gallops.rs. Before removing, verify all heat JSON files in current/ and retired/ directories use "racing" (not "current"). This hardens the schema to match the final-form JJD spec.

Files: Tools/jjk/vov_veiled/src/jjrg_gallops.rs

## Steeplechase

### 2026-01-17 14:06 - Heat - d

Restring: 3 paces from ₣AA (furlough feature)

### 2026-01-17 14:06 - Heat - D

₢AAABD → ₢ACAAC

### 2026-01-17 14:06 - Heat - D

₢AAABC → ₢ACAAB

### 2026-01-17 14:06 - Heat - D

₢AAABB → ₢ACAAA

### 2026-01-17 14:05 - Heat - N

jjk-furlough-feature

