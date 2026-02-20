# Heat Trophy: jjk-post-alpha-polish

**Firemark:** ₣AF
**Created:** 260119
**Retired:** 260124
**Status:** retired

> NOTE: JJSA renamed to JJS0 (top-level spec '0' suffix convention). Filename references in this trophy are historical.

## Paddock

# Paddock: jjk-post-alpha-polish

## Context

Schema-stable JJK improvements. Bug fixes, CLI ergonomics, documentation, and behavior refinements that do NOT alter JSON structure in jjm/ files.

**CONSTRAINT: NO SCHEMA CHANGES.** If a fix requires modifying JSON field names, types, or adding/removing fields, it belongs in ₣AG (jjk-post-alpha-breaking) instead.

## References

- JJSA: Tools/jjk/vov_veiled/JJSA-GallopsData.adoc
- RCG: Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- VOS: Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc

## Paces

### strip-tally-chatter (₢AFAA7) [complete]

**[260124-1745] complete**

Strip verbose process chatter from jjx_tally output.

## Rationale

Now that tally is trusted and has good error reporting, the step-by-step narration consumes tokens without benefit.

## Current output (10+ lines)

```
jjx_tally: starting
jjx_tally: stdin read (N bytes)
jjx_tally: gallops loaded
jjx_tally: calling jjrg_tally
jjx_tally: tally succeeded, persisting
vvcm_commit: staged N file(s)
guard: OK - staged content N bytes (limit: 50000)
vvcm_commit: committed <hash>
jjx_tally: committed <hash>
jjx_tally: creating B (bridle) commit
jjx_tally: B commit created <hash>
jjx_tally: complete, releasing lock
```

## Target output (1-2 lines)

Standard tally:
```
committed <hash>
```

Bridle tally (with B marker):
```
committed <hash>
B commit <hash>
```

## Lines to eliminate

- `jjx_tally: starting`
- `jjx_tally: stdin read (N bytes)`
- `jjx_tally: gallops loaded`
- `jjx_tally: calling jjrg_tally`
- `jjx_tally: tally succeeded, persisting`
- `jjx_tally: committed <hash>` (redundant with vvcm_commit)
- `jjx_tally: creating B (bridle) commit`
- `jjx_tally: complete, releasing lock`
- `vvcm_commit: staged N file(s)`
- `guard: OK - ...`

## Lines to keep

- `vvcm_commit: committed <hash>` → simplify to `committed <hash>`
- `jjx_tally: B commit created <hash>` → simplify to `B commit <hash>`

## Files

jjrx_cli.rs (tally entry point), vvcc_commit.rs (vvcm_commit output)

**[260124-1740] bridled**

Strip verbose process chatter from jjx_tally output.

## Rationale

Now that tally is trusted and has good error reporting, the step-by-step narration consumes tokens without benefit.

## Current output (10+ lines)

```
jjx_tally: starting
jjx_tally: stdin read (N bytes)
jjx_tally: gallops loaded
jjx_tally: calling jjrg_tally
jjx_tally: tally succeeded, persisting
vvcm_commit: staged N file(s)
guard: OK - staged content N bytes (limit: 50000)
vvcm_commit: committed <hash>
jjx_tally: committed <hash>
jjx_tally: creating B (bridle) commit
jjx_tally: B commit created <hash>
jjx_tally: complete, releasing lock
```

## Target output (1-2 lines)

Standard tally:
```
committed <hash>
```

Bridle tally (with B marker):
```
committed <hash>
B commit <hash>
```

## Lines to eliminate

- `jjx_tally: starting`
- `jjx_tally: stdin read (N bytes)`
- `jjx_tally: gallops loaded`
- `jjx_tally: calling jjrg_tally`
- `jjx_tally: tally succeeded, persisting`
- `jjx_tally: committed <hash>` (redundant with vvcm_commit)
- `jjx_tally: creating B (bridle) commit`
- `jjx_tally: complete, releasing lock`
- `vvcm_commit: staged N file(s)`
- `guard: OK - ...`

## Lines to keep

- `vvcm_commit: committed <hash>` → simplify to `committed <hash>`
- `jjx_tally: B commit created <hash>` → simplify to `B commit <hash>`

## Files

jjrx_cli.rs (tally entry point), vvcc_commit.rs (vvcm_commit output)

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrx_cli.rs, vvcc_commit.rs (2 files) | Steps: 1. In jjrx_cli.rs remove all eprintln with starting, stdin read, gallops loaded, calling jjrg_tally, tally succeeded, committed hash, creating B commit, complete releasing lock 2. In jjrx_cli.rs change B commit created line to just B commit hash 3. In vvcc_commit.rs remove staged N files and guard OK lines, change committed line to just committed hash | Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260124-1739] rough**

Strip verbose process chatter from jjx_tally output.

## Rationale

Now that tally is trusted and has good error reporting, the step-by-step narration consumes tokens without benefit.

## Current output (10+ lines)

```
jjx_tally: starting
jjx_tally: stdin read (N bytes)
jjx_tally: gallops loaded
jjx_tally: calling jjrg_tally
jjx_tally: tally succeeded, persisting
vvcm_commit: staged N file(s)
guard: OK - staged content N bytes (limit: 50000)
vvcm_commit: committed <hash>
jjx_tally: committed <hash>
jjx_tally: creating B (bridle) commit
jjx_tally: B commit created <hash>
jjx_tally: complete, releasing lock
```

## Target output (1-2 lines)

Standard tally:
```
committed <hash>
```

Bridle tally (with B marker):
```
committed <hash>
B commit <hash>
```

## Lines to eliminate

- `jjx_tally: starting`
- `jjx_tally: stdin read (N bytes)`
- `jjx_tally: gallops loaded`
- `jjx_tally: calling jjrg_tally`
- `jjx_tally: tally succeeded, persisting`
- `jjx_tally: committed <hash>` (redundant with vvcm_commit)
- `jjx_tally: creating B (bridle) commit`
- `jjx_tally: complete, releasing lock`
- `vvcm_commit: staged N file(s)`
- `guard: OK - ...`

## Lines to keep

- `vvcm_commit: committed <hash>` → simplify to `committed <hash>`
- `jjx_tally: B commit created <hash>` → simplify to `B commit <hash>`

## Files

jjrx_cli.rs (tally entry point), vvcc_commit.rs (vvcm_commit output)

### jjrp-column-table-module (₢AFAAx) [complete]

**[260124-1003] complete**

Design and implement jjrp_print.rs column table module following RCG principles.

**Goal:** Eliminate magic numbers/strings from column-formatted output.

**JJSA Update (voices entity):**
Add new entity definition to JJSA-GallopsData.adoc mapping section:
- `jjdyr_table` → Column Table entity (axl_voices axo_entity)
- `jjdym_column` → Column member (axl_voices axr_member)
- `jjdym_header` → Header string (axl_voices axr_member axd_required)
- `jjdym_align` → Alignment (axl_voices axr_member axd_required)
- `jjdye_left` / `jjdye_right` → Alignment enum values

**Rust Implementation (jjrp_print.rs):**

Constants:
- `JJRP_COLUMN_GAP: usize = 2` — gap between columns
- `JJRP_MIN_WIDTH: usize = 5` — minimum column width

Struct `jjrp_Column`:
- `header: &'static str` — header text (single source of truth)
- `min_width: usize` — defaults to header.len()
- `align: jjrp_Align` — Left or Right

Struct `jjrp_Table`:
- Constructor: `jjrp_new(columns: Vec<jjrp_Column>)`
- `jjrp_measure(&mut self, row: &[&str])` — update widths from data
- `jjrp_print_header(&self)` — print header row
- `jjrp_print_separator(&self)` — print computed-width separator
- `jjrp_print_row(&self, values: &[&str])` — print data row

**Identity Display Discipline:**
When columns contain Firemark or Coronet identities:
- ALWAYS include the currency prefix (₣ or ₢)
- Header should reflect this: "Fire" → "₣Fire" or just "Firemark"

**Files:** JJSA-GallopsData.adoc, jjrp_print.rs (new), lib.rs (wire module)

**Test:** cargo build, cargo test

**[260124-0957] bridled**

Design and implement jjrp_print.rs column table module following RCG principles.

**Goal:** Eliminate magic numbers/strings from column-formatted output.

**JJSA Update (voices entity):**
Add new entity definition to JJSA-GallopsData.adoc mapping section:
- `jjdyr_table` → Column Table entity (axl_voices axo_entity)
- `jjdym_column` → Column member (axl_voices axr_member)
- `jjdym_header` → Header string (axl_voices axr_member axd_required)
- `jjdym_align` → Alignment (axl_voices axr_member axd_required)
- `jjdye_left` / `jjdye_right` → Alignment enum values

**Rust Implementation (jjrp_print.rs):**

Constants:
- `JJRP_COLUMN_GAP: usize = 2` — gap between columns
- `JJRP_MIN_WIDTH: usize = 5` — minimum column width

Struct `jjrp_Column`:
- `header: &'static str` — header text (single source of truth)
- `min_width: usize` — defaults to header.len()
- `align: jjrp_Align` — Left or Right

Struct `jjrp_Table`:
- Constructor: `jjrp_new(columns: Vec<jjrp_Column>)`
- `jjrp_measure(&mut self, row: &[&str])` — update widths from data
- `jjrp_print_header(&self)` — print header row
- `jjrp_print_separator(&self)` — print computed-width separator
- `jjrp_print_row(&self, values: &[&str])` — print data row

**Identity Display Discipline:**
When columns contain Firemark or Coronet identities:
- ALWAYS include the currency prefix (₣ or ₢)
- Header should reflect this: "Fire" → "₣Fire" or just "Firemark"

**Files:** JJSA-GallopsData.adoc, jjrp_print.rs (new), lib.rs (wire module)

**Test:** cargo build, cargo test

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrp_print.rs (new), lib.rs, JJSA-GallopsData.adoc (3 files) | Steps: 1. Create jjrp_print.rs with JJRP_COLUMN_GAP and JJRP_MIN_WIDTH constants, jjrp_Align enum Left/Right, jjrp_Column struct with header min_width align fields, jjrp_Table struct with jjrp_new jjrp_measure jjrp_print_header jjrp_print_separator jjrp_print_row methods 2. Add pub mod jjrp_print to lib.rs after jjrq_query 3. Add jjdy* attribute references to JJSA mapping section for Column Table entity and alignment enum values with axl_voices annotations | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-0955] rough**

Design and implement jjrp_print.rs column table module following RCG principles.

**Goal:** Eliminate magic numbers/strings from column-formatted output.

**JJSA Update (voices entity):**
Add new entity definition to JJSA-GallopsData.adoc mapping section:
- `jjdyr_table` → Column Table entity (axl_voices axo_entity)
- `jjdym_column` → Column member (axl_voices axr_member)
- `jjdym_header` → Header string (axl_voices axr_member axd_required)
- `jjdym_align` → Alignment (axl_voices axr_member axd_required)
- `jjdye_left` / `jjdye_right` → Alignment enum values

**Rust Implementation (jjrp_print.rs):**

Constants:
- `JJRP_COLUMN_GAP: usize = 2` — gap between columns
- `JJRP_MIN_WIDTH: usize = 5` — minimum column width

Struct `jjrp_Column`:
- `header: &'static str` — header text (single source of truth)
- `min_width: usize` — defaults to header.len()
- `align: jjrp_Align` — Left or Right

Struct `jjrp_Table`:
- Constructor: `jjrp_new(columns: Vec<jjrp_Column>)`
- `jjrp_measure(&mut self, row: &[&str])` — update widths from data
- `jjrp_print_header(&self)` — print header row
- `jjrp_print_separator(&self)` — print computed-width separator
- `jjrp_print_row(&self, values: &[&str])` — print data row

**Identity Display Discipline:**
When columns contain Firemark or Coronet identities:
- ALWAYS include the currency prefix (₣ or ₢)
- Header should reflect this: "Fire" → "₣Fire" or just "Firemark"

**Files:** JJSA-GallopsData.adoc, jjrp_print.rs (new), lib.rs (wire module)

**Test:** cargo build, cargo test

### jjrp-incorporate-existing (₢AFAAy) [complete]

**[260124-1007] complete**

Refactor existing column-formatted output to use jjrp_Table.

**Locations to update:**

1. **jjrq_run_muster** (jjrq_query.rs:79-115)
   - Headers: Fire, Silks, Status, Done, Total
   - Replace max_silks_len calculation with jjrp_Table.jjrp_measure()
   - Replace manual println! with jjrp_print_row()
   - Identity display: Fire column shows ₣XX format

2. **jjrq_run_parade remaining view** (jjrq_query.rs:445-467)
   - Headers: No, State, Pace, Coronet
   - Replace max_silks_len with jjrp_Table
   - Identity display: Coronet column shows ₢XXXXX format

3. **jjrq_run_parade full view** (jjrq_query.rs:478-505)
   - Headers: No, State, Pace, Coronet
   - Same refactoring as remaining view
   - Identity display: Coronet column shows ₢XXXXX format

**Pattern:**
```rust
let mut table = jjrp_Table::jjrp_new(vec![
    jjrp_Column { header: "₣Fire", ... },
    jjrp_Column { header: "Silks", ... },
]);
for item in items { table.jjrp_measure(&[...]); }
table.jjrp_print_header();
table.jjrp_print_separator();
for item in items { table.jjrp_print_row(&[...]); }
```

**Verify:**
- No magic numbers (20, 8, 10) remain in these functions
- No duplicate string literals for headers
- ₣/₢ prefixes always displayed

**Files:** jjrq_query.rs
**Depends on:** ₢AFAAx (jjrp-column-table-module)

**[260124-1002] bridled**

Refactor existing column-formatted output to use jjrp_Table.

**Locations to update:**

1. **jjrq_run_muster** (jjrq_query.rs:79-115)
   - Headers: Fire, Silks, Status, Done, Total
   - Replace max_silks_len calculation with jjrp_Table.jjrp_measure()
   - Replace manual println! with jjrp_print_row()
   - Identity display: Fire column shows ₣XX format

2. **jjrq_run_parade remaining view** (jjrq_query.rs:445-467)
   - Headers: No, State, Pace, Coronet
   - Replace max_silks_len with jjrp_Table
   - Identity display: Coronet column shows ₢XXXXX format

3. **jjrq_run_parade full view** (jjrq_query.rs:478-505)
   - Headers: No, State, Pace, Coronet
   - Same refactoring as remaining view
   - Identity display: Coronet column shows ₢XXXXX format

**Pattern:**
```rust
let mut table = jjrp_Table::jjrp_new(vec![
    jjrp_Column { header: "₣Fire", ... },
    jjrp_Column { header: "Silks", ... },
]);
for item in items { table.jjrp_measure(&[...]); }
table.jjrp_print_header();
table.jjrp_print_separator();
for item in items { table.jjrp_print_row(&[...]); }
```

**Verify:**
- No magic numbers (20, 8, 10) remain in these functions
- No duplicate string literals for headers
- ₣/₢ prefixes always displayed

**Files:** jjrq_query.rs
**Depends on:** ₢AFAAx (jjrp-column-table-module)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrq_query.rs (1 file) | Steps: 1. Import jjrp_print module 2. Refactor jjrq_run_muster to use jjrp_Table with headers Fire, Silks, Status, Done, Total 3. Refactor jjrq_run_parade remaining view to use jjrp_Table with headers No, State, Pace, Coronet 4. Refactor jjrq_run_parade full view same pattern 5. Ensure currency prefixes always displayed in identity columns | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-0955] rough**

Refactor existing column-formatted output to use jjrp_Table.

**Locations to update:**

1. **jjrq_run_muster** (jjrq_query.rs:79-115)
   - Headers: Fire, Silks, Status, Done, Total
   - Replace max_silks_len calculation with jjrp_Table.jjrp_measure()
   - Replace manual println! with jjrp_print_row()
   - Identity display: Fire column shows ₣XX format

2. **jjrq_run_parade remaining view** (jjrq_query.rs:445-467)
   - Headers: No, State, Pace, Coronet
   - Replace max_silks_len with jjrp_Table
   - Identity display: Coronet column shows ₢XXXXX format

3. **jjrq_run_parade full view** (jjrq_query.rs:478-505)
   - Headers: No, State, Pace, Coronet
   - Same refactoring as remaining view
   - Identity display: Coronet column shows ₢XXXXX format

**Pattern:**
```rust
let mut table = jjrp_Table::jjrp_new(vec![
    jjrp_Column { header: "₣Fire", ... },
    jjrp_Column { header: "Silks", ... },
]);
for item in items { table.jjrp_measure(&[...]); }
table.jjrp_print_header();
table.jjrp_print_separator();
for item in items { table.jjrp_print_row(&[...]); }
```

**Verify:**
- No magic numbers (20, 8, 10) remain in these functions
- No duplicate string literals for headers
- ₣/₢ prefixes always displayed

**Files:** jjrq_query.rs
**Depends on:** ₢AFAAx (jjrp-column-table-module)

### optional-firemark-target (₢AFAAn) [complete]

**[260124-0847] complete**

Make TARGET argument optional in jjx_parade and jjx_saddle. When omitted, auto-select the first racing heat.

**Behavior:**
- If TARGET provided: use it (current behavior)
- If TARGET omitted: resolve to first racing heat from gallops
- If no racing heats exist: return error "No racing heats found"

**Changes:**

1. **jjrq_query.rs**: Add helper function `jjrq_resolve_default_heat(gallops: &Gallops) -> Result<Firemark>` that returns first racing heat or error

2. **jjrx_cli.rs**: Change `<TARGET>` to `[TARGET]` (optional) in ParadeArgs and SaddleArgs

3. **jjrq_run_parade / jjrq_run_saddle**: If target is None, call resolve helper to get default

4. **JJSA spec updates**: Document optional target behavior in JJSCPD-parade.adoc and JJSCSD-saddle.adoc

**Why:** Eliminates muster round-trip in slash commands like /jjc-heat-groom. The "first racing heat" logic (most recently furloughed to racing) already exists in muster ordering.

**Files:** jjrq_query.rs, jjrx_cli.rs, JJSCPD-parade.adoc, JJSCSD-saddle.adoc

**[260124-0830] bridled**

Make TARGET argument optional in jjx_parade and jjx_saddle. When omitted, auto-select the first racing heat.

**Behavior:**
- If TARGET provided: use it (current behavior)
- If TARGET omitted: resolve to first racing heat from gallops
- If no racing heats exist: return error "No racing heats found"

**Changes:**

1. **jjrq_query.rs**: Add helper function `jjrq_resolve_default_heat(gallops: &Gallops) -> Result<Firemark>` that returns first racing heat or error

2. **jjrx_cli.rs**: Change `<TARGET>` to `[TARGET]` (optional) in ParadeArgs and SaddleArgs

3. **jjrq_run_parade / jjrq_run_saddle**: If target is None, call resolve helper to get default

4. **JJSA spec updates**: Document optional target behavior in JJSCPD-parade.adoc and JJSCSD-saddle.adoc

**Why:** Eliminates muster round-trip in slash commands like /jjc-heat-groom. The "first racing heat" logic (most recently furloughed to racing) already exists in muster ordering.

**Files:** jjrq_query.rs, jjrx_cli.rs, JJSCPD-parade.adoc, JJSCSD-saddle.adoc

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrq_query.rs, jjrx_cli.rs, JJSCPD-parade.adoc, JJSCSD-saddle.adoc (4 files) | Steps: 1. Add jjrq_resolve_default_heat helper in jjrq_query.rs after imports that iterates gallops.heats finding first racing heat or returns error No racing heats found 2. In jjrx_cli.rs change zjjrx_SaddleArgs.firemark and zjjrx_ParadeArgs.target from String to Option of String using clap optional positional 3. Update zjjrx_run_saddle to resolve None firemark via helper before calling lib 4. Update zjjrx_run_parade to resolve None target for heat view via helper 5. Update JJSCSD-saddle.adoc TARGET line to show optional with default behavior 6. Update JJSCPD-parade.adoc TARGET line to show optional with default behavior | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-0821] rough**

Make TARGET argument optional in jjx_parade and jjx_saddle. When omitted, auto-select the first racing heat.

**Behavior:**
- If TARGET provided: use it (current behavior)
- If TARGET omitted: resolve to first racing heat from gallops
- If no racing heats exist: return error "No racing heats found"

**Changes:**

1. **jjrq_query.rs**: Add helper function `jjrq_resolve_default_heat(gallops: &Gallops) -> Result<Firemark>` that returns first racing heat or error

2. **jjrx_cli.rs**: Change `<TARGET>` to `[TARGET]` (optional) in ParadeArgs and SaddleArgs

3. **jjrq_run_parade / jjrq_run_saddle**: If target is None, call resolve helper to get default

4. **JJSA spec updates**: Document optional target behavior in JJSCPD-parade.adoc and JJSCSD-saddle.adoc

**Why:** Eliminates muster round-trip in slash commands like /jjc-heat-groom. The "first racing heat" logic (most recently furloughed to racing) already exists in muster ordering.

**Files:** jjrq_query.rs, jjrx_cli.rs, JJSCPD-parade.adoc, JJSCSD-saddle.adoc

### parade-column-format (₢AFAAf) [complete]

**[260124-0933] complete**

Change jjx_parade --remaining output from plain text to display-ready markdown. No new flag — this is the new default format.

**Current output (plain text):**
```
# Progress: 21 complete, 4 abandoned, 13 remaining (7 rough, 6 bridled)
1. [rough] slash-cmd-heredoc-stdin (₢AFAAj)
2. [bridled] muster-box-table-output (₢AFAAe)
```

**New output (markdown):**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Modify jjrq_run_parade: when --remaining flag is set, output markdown format
- No new CLI flag — just change the output format
- "Next up" = first pace that is not complete/abandoned

**JJSA update:**
- Update JJSCPD-parade.adoc stdout section to document new markdown format for --remaining

**Files:** jjrq_query.rs, JJSCPD-parade.adoc

**[260124-0920] bridled**

Change jjx_parade --remaining output from plain text to display-ready markdown. No new flag — this is the new default format.

**Current output (plain text):**
```
# Progress: 21 complete, 4 abandoned, 13 remaining (7 rough, 6 bridled)
1. [rough] slash-cmd-heredoc-stdin (₢AFAAj)
2. [bridled] muster-box-table-output (₢AFAAe)
```

**New output (markdown):**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Modify jjrq_run_parade: when --remaining flag is set, output markdown format
- No new CLI flag — just change the output format
- "Next up" = first pace that is not complete/abandoned

**JJSA update:**
- Update JJSCPD-parade.adoc stdout section to document new markdown format for --remaining

**Files:** jjrq_query.rs, JJSCPD-parade.adoc

*Direction:* Cardinality: 2 parallel + sequential build
Files: jjrq_query.rs, JJSCPD-parade.adoc (2 files)
Steps:
1. Agent A (haiku): In jjrq_run_parade, when args.remaining is true, output markdown format: H2 header with heat silks/firemark/status, Progress line with bold labels and pipe separators, Remaining paces as numbered markdown list with state/silks/coronet, Next up callout identifying first non-complete/abandoned pace
2. Agent B (sonnet): In JJSCPD-parade.adoc, update stdout section to document markdown format for --remaining output
3. Sequential: tt/vow-b.Build.sh and tt/vow-t.Test.sh
Verify: tt/vow-b.Build.sh

**[260124-0828] bridled**

Change jjx_parade --remaining output from plain text to display-ready markdown. No new flag — this is the new default format.

**Current output (plain text):**
```
# Progress: 21 complete, 4 abandoned, 13 remaining (7 rough, 6 bridled)
1. [rough] slash-cmd-heredoc-stdin (₢AFAAj)
2. [bridled] muster-box-table-output (₢AFAAe)
```

**New output (markdown):**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Modify jjrq_run_parade: when --remaining flag is set, output markdown format
- No new CLI flag — just change the output format
- "Next up" = first pace that is not complete/abandoned

**JJSA update:**
- Update JJSCPD-parade.adoc stdout section to document new markdown format for --remaining

**Files:** jjrq_query.rs, JJSCPD-parade.adoc

*Direction:* Cardinality: 2 parallel + sequential build
Files: jjrq_query.rs, JJSCPD-parade.adoc (2 files)
Steps:
1. Agent A (haiku): In jjrq_run_parade, when args.remaining is true, output markdown format: H2 header with heat silks/firemark/status, Progress line with bold labels and pipe separators, Remaining paces as numbered markdown list with state/silks/coronet, Next up callout identifying first non-complete/abandoned pace
2. Agent B (sonnet): In JJSCPD-parade.adoc, update stdout section to document markdown format for --remaining output
3. Sequential: tt/vow-b.Build.sh and tt/vow-t.Test.sh
Verify: tt/vow-b.Build.sh

**[260124-0822] rough**

Change jjx_parade --remaining output from plain text to display-ready markdown. No new flag — this is the new default format.

**Current output (plain text):**
```
# Progress: 21 complete, 4 abandoned, 13 remaining (7 rough, 6 bridled)
1. [rough] slash-cmd-heredoc-stdin (₢AFAAj)
2. [bridled] muster-box-table-output (₢AFAAe)
```

**New output (markdown):**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Modify jjrq_run_parade: when --remaining flag is set, output markdown format
- No new CLI flag — just change the output format
- "Next up" = first pace that is not complete/abandoned

**JJSA update:**
- Update JJSCPD-parade.adoc stdout section to document new markdown format for --remaining

**Files:** jjrq_query.rs, JJSCPD-parade.adoc

**[260124-0743] bridled**

Add --markdown flag to jjx_parade --remaining that outputs ready-to-display markdown for /jjc-heat-groom.

**Output format:**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Add `--markdown` flag to parade command
- When combined with `--remaining`, output the format above
- State indicators: `[rough]`, `[bridled]`, `[complete]`, `[abandoned]`
- Next up = first pace that is not complete/abandoned

**Files:** jjx_parade.rs (add flag handling and markdown formatter)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrq_query.rs, jjrx_cli.rs (2 files)
Steps:
1. Add markdown field to jjrq_ParadeArgs struct
2. Add --markdown flag to ParadeArgs in jjrx_cli.rs
3. In jjrq_run_parade, when args.remaining and args.markdown are both true, output markdown format instead of plain list
4. Markdown format: H2 header with heat silks/firemark/status, Progress line with stats, Remaining paces as numbered markdown list, Next up callout identifying first non-complete/abandoned pace
Verify: tt/vow-b.Build.sh

**[260124-0742] rough**

Add --markdown flag to jjx_parade --remaining that outputs ready-to-display markdown for /jjc-heat-groom.

**Output format:**
```markdown
## Heat: {silks} (₣{firemark}) [{status}]

**Progress:** {complete} complete | {abandoned} abandoned | {remaining} remaining ({rough} rough, {bridled} bridled)

**Remaining paces:**
1. [{state}] {silks} (₢{coronet})
2. [{state}] {silks} (₢{coronet})
...

**Next up:** {silks} (₢{coronet}) [{state}]
```

**Implementation:**
- Add `--markdown` flag to parade command
- When combined with `--remaining`, output the format above
- State indicators: `[rough]`, `[bridled]`, `[complete]`, `[abandoned]`
- Next up = first pace that is not complete/abandoned

**Files:** jjx_parade.rs (add flag handling and markdown formatter)

### vos-commit-message-format (₢AFAAm) [complete]

**[260124-1010] complete**

Update VOS (Vox Obscura Spec) with VVX commit message format section.

**Add section: Commit Message Architecture**

Define the vvb commit format for VOK operations:
```
vvb:HALLMARK::ACTION: message
```

**Fields:**
- `vvb` — Voce Viva brand prefix (literal)
- `HALLMARK` — Version identifier (same source as JJ: .vvk/vvbf_brand.json or kit forge registry + HEAD)
- `` — Empty identity field (VOK ops don't have heat/pace nesting)
- `ACTION` — Single-letter code

**Action codes:**
- `A` — Allocate: registry hallmark allocation
- `R` — Release: parcel creation

**Cross-reference:** Note shared hallmark concept with JJSA jjb: commits.

**Files:** VOS-VoxObscuraSpec.adoc

**[260124-0814] bridled**

Update VOS (Vox Obscura Spec) with VVX commit message format section.

**Add section: Commit Message Architecture**

Define the vvb commit format for VOK operations:
```
vvb:HALLMARK::ACTION: message
```

**Fields:**
- `vvb` — Voce Viva brand prefix (literal)
- `HALLMARK` — Version identifier (same source as JJ: .vvk/vvbf_brand.json or kit forge registry + HEAD)
- `` — Empty identity field (VOK ops don't have heat/pace nesting)
- `ACTION` — Single-letter code

**Action codes:**
- `A` — Allocate: registry hallmark allocation
- `R` — Release: parcel creation

**Cross-reference:** Note shared hallmark concept with JJSA jjb: commits.

**Files:** VOS-VoxObscuraSpec.adoc

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: VOS-VoxObscuraSpec.adoc (1 file)
Steps:
1. Add mapping attributes in mapping section after existing vosc_ entries: vosca_vvb, voscf_format, voscaa_allocate, voscar_release
2. Add new section after Operations section titled Commit Message Architecture
3. Define format: vvb:HALLMARK::ACTION: message
4. Document fields: vvb is Voce Viva brand prefix literal, HALLMARK uses same source as JJ from .vvk/vvbf_brand.json or kit forge registry plus HEAD, empty field denotes no identity nesting, ACTION is single letter code
5. Define action codes: A for Allocate hallmark in registry, R for Release parcel creation
6. Add cross-reference note that hallmark concept is shared with JJSA jjb: commits
Verify: Review that section follows MCM attribute patterns used elsewhere in VOS

**[260124-0813] rough**

Update VOS (Vox Obscura Spec) with VVX commit message format section.

**Add section: Commit Message Architecture**

Define the vvb commit format for VOK operations:
```
vvb:HALLMARK::ACTION: message
```

**Fields:**
- `vvb` — Voce Viva brand prefix (literal)
- `HALLMARK` — Version identifier (same source as JJ: .vvk/vvbf_brand.json or kit forge registry + HEAD)
- `` — Empty identity field (VOK ops don't have heat/pace nesting)
- `ACTION` — Single-letter code

**Action codes:**
- `A` — Allocate: registry hallmark allocation
- `R` — Release: parcel creation

**Cross-reference:** Note shared hallmark concept with JJSA jjb: commits.

**Files:** VOS-VoxObscuraSpec.adoc

### saddle-column-format (₢AFAAg) [complete]

**[260124-1019] complete**

Change jjx_saddle to plain text AND update jjc-heat-mount to parse it.

**Coordinated change** - both must change together to avoid breaking mount.

**Depends on:** ₢AFAAx (jjrp-column-table-module), ₢AFAAy (jjrp-incorporate-existing)

**New saddle output format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Paddock-content:
[paddock markdown here, indented 2 spaces]

Next: pace-silks (₢XXXXX) [rough|bridled]

Spec:
[spec text, indented 2 spaces]

Direction:
[direction text if bridled, section omitted if rough]

Recent-work:
[column-formatted table using jjrp_Table]
```

**Recent-work column formatting:**
Use jjrp_Table for column-aligned output with headers:
- Timestamp (left-aligned)
- Commit (left-aligned)
- [A] action marker (left-aligned)
- Identity (₢XXXXX or ₣XX, left-aligned, always with prefix)
- Subject (left-aligned)

**Identity Display Discipline:**
- Heat line: always show ₣XX format
- Next line: always show ₢XXXXX format  
- Recent-work: always show ₢XXXXX or ₣XX format (never strip prefix)

**Files to change:**

1. **jjrq_query.rs**: Replace JSON serialization in jjrq_run_saddle with println statements. Import jjrp module. Use jjrp_Table for Recent-work section.

2. **jjc-heat-mount.md**: Update Step 2 to parse plain text format instead of JSON. Field extraction by label prefix.

3. **JJSCSD-saddle.adoc**: Update stdout section to show plain text format.

**Test:** After changes, `/jjc-heat-mount AF` should still work correctly.

**Files:** jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc

**[260124-1007] bridled**

Change jjx_saddle to plain text AND update jjc-heat-mount to parse it.

**Coordinated change** - both must change together to avoid breaking mount.

**Depends on:** ₢AFAAx (jjrp-column-table-module), ₢AFAAy (jjrp-incorporate-existing)

**New saddle output format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Paddock-content:
[paddock markdown here, indented 2 spaces]

Next: pace-silks (₢XXXXX) [rough|bridled]

Spec:
[spec text, indented 2 spaces]

Direction:
[direction text if bridled, section omitted if rough]

Recent-work:
[column-formatted table using jjrp_Table]
```

**Recent-work column formatting:**
Use jjrp_Table for column-aligned output with headers:
- Timestamp (left-aligned)
- Commit (left-aligned)
- [A] action marker (left-aligned)
- Identity (₢XXXXX or ₣XX, left-aligned, always with prefix)
- Subject (left-aligned)

**Identity Display Discipline:**
- Heat line: always show ₣XX format
- Next line: always show ₢XXXXX format  
- Recent-work: always show ₢XXXXX or ₣XX format (never strip prefix)

**Files to change:**

1. **jjrq_query.rs**: Replace JSON serialization in jjrq_run_saddle with println statements. Import jjrp module. Use jjrp_Table for Recent-work section.

2. **jjc-heat-mount.md**: Update Step 2 to parse plain text format instead of JSON. Field extraction by label prefix.

3. **JJSCSD-saddle.adoc**: Update stdout section to show plain text format.

**Test:** After changes, `/jjc-heat-mount AF` should still work correctly.

**Files:** jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc (3 files) | Steps: 1. In jjrq_run_saddle replace JSON serialization with println using labels Heat: Paddock: Paddock-content: Next: Spec: Direction: Recent-work: with 2-space indentation for multiline content 2. For Recent-work use jjrp_Table with columns Timestamp Commit Action Identity Subject 3. Update jjc-heat-mount.md Step 2 to parse plain text by label prefix instead of JSON 4. Update JJSCSD-saddle.adoc stdout section with new plain text format | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-0956] rough**

Change jjx_saddle to plain text AND update jjc-heat-mount to parse it.

**Coordinated change** - both must change together to avoid breaking mount.

**Depends on:** ₢AFAAx (jjrp-column-table-module), ₢AFAAy (jjrp-incorporate-existing)

**New saddle output format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Paddock-content:
[paddock markdown here, indented 2 spaces]

Next: pace-silks (₢XXXXX) [rough|bridled]

Spec:
[spec text, indented 2 spaces]

Direction:
[direction text if bridled, section omitted if rough]

Recent-work:
[column-formatted table using jjrp_Table]
```

**Recent-work column formatting:**
Use jjrp_Table for column-aligned output with headers:
- Timestamp (left-aligned)
- Commit (left-aligned)
- [A] action marker (left-aligned)
- Identity (₢XXXXX or ₣XX, left-aligned, always with prefix)
- Subject (left-aligned)

**Identity Display Discipline:**
- Heat line: always show ₣XX format
- Next line: always show ₢XXXXX format  
- Recent-work: always show ₢XXXXX or ₣XX format (never strip prefix)

**Files to change:**

1. **jjrq_query.rs**: Replace JSON serialization in jjrq_run_saddle with println statements. Import jjrp module. Use jjrp_Table for Recent-work section.

2. **jjc-heat-mount.md**: Update Step 2 to parse plain text format instead of JSON. Field extraction by label prefix.

3. **JJSCSD-saddle.adoc**: Update stdout section to show plain text format.

**Test:** After changes, `/jjc-heat-mount AF` should still work correctly.

**Files:** jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc

**[260124-0946] bridled**

Change jjx_saddle to plain text AND update jjc-heat-mount to parse it.

**Coordinated change** - both must change together to avoid breaking mount.

**New saddle output format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Paddock-content:
[paddock markdown here, indented 2 spaces]

Next: pace-silks (₢XXXXX) [rough|bridled]

Spec:
[spec text, indented 2 spaces]

Direction:
[direction text if bridled, section omitted if rough]

Recent-work:
YYYY-MM-DD HH:MM  commit  [A]  ₢XXXXX  subject
YYYY-MM-DD HH:MM  commit  [n]  ₣XX     subject
```

**Files to change:**

1. **jjrq_query.rs**: Replace JSON serialization in jjrq_run_saddle with println statements. Use consistent field labels (Heat:, Paddock:, Next:, Spec:, Direction:, Recent-work:).

2. **jjc-heat-mount.md**: Update Step 2 to parse plain text format instead of JSON. Field extraction by label prefix.

3. **JJSCSD-saddle.adoc**: Update stdout section to show plain text format.

**Test:** After changes, `/jjc-heat-mount AF` should still work correctly.

**Files:** jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc (3 files) | Steps: 1. In jjrq_run_saddle replace JSON serialization with println using labels Heat: Paddock: Paddock-content: Next: Spec: Direction: Recent-work: 2. Update jjc-heat-mount.md Step 2 to show plain text format and field extraction by label prefix 3. Update JJSCSD-saddle.adoc stdout section with new format | Verify: tt/vow-b.Build.sh

**[260124-0945] rough**

Change jjx_saddle to plain text AND update jjc-heat-mount to parse it.

**Coordinated change** - both must change together to avoid breaking mount.

**New saddle output format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Paddock-content:
[paddock markdown here, indented 2 spaces]

Next: pace-silks (₢XXXXX) [rough|bridled]

Spec:
[spec text, indented 2 spaces]

Direction:
[direction text if bridled, section omitted if rough]

Recent-work:
YYYY-MM-DD HH:MM  commit  [A]  ₢XXXXX  subject
YYYY-MM-DD HH:MM  commit  [n]  ₣XX     subject
```

**Files to change:**

1. **jjrq_query.rs**: Replace JSON serialization in jjrq_run_saddle with println statements. Use consistent field labels (Heat:, Paddock:, Next:, Spec:, Direction:, Recent-work:).

2. **jjc-heat-mount.md**: Update Step 2 to parse plain text format instead of JSON. Field extraction by label prefix.

3. **JJSCSD-saddle.adoc**: Update stdout section to show plain text format.

**Test:** After changes, `/jjc-heat-mount AF` should still work correctly.

**Files:** jjrq_query.rs, jjc-heat-mount.md, JJSCSD-saddle.adoc

**[260124-0937] bridled**

Change jjx_saddle to output column-aligned plain text instead of JSON.

**Current:** JSON output requiring agent parsing (wastes tokens)

**New format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Next: parade-column-format (₢AFAAf) [bridled]

Spec:
[spec text here]

Direction:
[direction text if bridled, omit section if rough]
```

**Implementation:**
- Modify jjrq_run_saddle in jjrq_query.rs
- Replace JSON with plain text format
- No flags - single output format

**Files:** jjrq_query.rs, JJSCSD-saddle.adoc

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrq_query.rs, JJSCSD-saddle.adoc (2 files) | Steps: 1. Replace JSON serialization in jjrq_run_saddle with println statements matching spec format 2. Remove or comment zjjrq_SaddleOutput struct 3. Update JJSCSD-saddle.adoc stdout section to show plain text format | Verify: tt/vow-b.Build.sh

**[260124-0932] rough**

Change jjx_saddle to output column-aligned plain text instead of JSON.

**Current:** JSON output requiring agent parsing (wastes tokens)

**New format:**
```
Heat: jjk-post-alpha-polish (₣AF) [racing]
Paddock: .claude/jjm/jjp_AF.md

Next: parade-column-format (₢AFAAf) [bridled]

Spec:
[spec text here]

Direction:
[direction text if bridled, omit section if rough]
```

**Implementation:**
- Modify jjrq_run_saddle in jjrq_query.rs
- Replace JSON with plain text format
- No flags - single output format

**Files:** jjrq_query.rs, JJSCSD-saddle.adoc

**[260124-0828] bridled**

Change jjx_saddle default output from JSON to display-ready markdown. Add --json flag for programmatic access.

**Current output (JSON):**
```json
{
  "heat_silks": "...",
  "paddock_content": "...",
  "pace_coronet": "₢XXXXX",
  ...
}
```

**New default output (markdown):**
```markdown
## Pace: {silks} (₢{coronet}) [{state}]

**Heat:** {heat_silks} (₣{firemark})

### Specification
{spec_content}

### Direction
{direction_content or omit section if rough}

### Paddock
{paddock_content or 'No paddock file.'}

### Steeplechase History
{recent_work formatted or 'No prior attempts.'}
```

**Implementation:**
- Change default output to markdown format
- Add `--json` flag to get current JSON output (for programmatic use)
- Update /jjc-heat-mount slash command to use `--json` flag

**JJSA update:**
- Update JJSCSD-saddle.adoc: document markdown as default stdout, add --json flag option

**Files:** jjrq_query.rs, jjrx_cli.rs, JJSCSD-saddle.adoc

*Direction:* Cardinality: 2 parallel + sequential build
Files: jjrq_query.rs, jjrx_cli.rs, JJSCSD-saddle.adoc (3 files)
Steps:
1. Agent A (haiku): Add json field to jjrq_SaddleArgs struct, add --json flag to SaddleArgs in jjrx_cli.rs. In jjrq_run_saddle, output markdown by default. When args.json is true, output current JSON format instead.
2. Agent B (sonnet): In JJSCSD-saddle.adoc, update stdout section: markdown is now default output, add --json flag documentation for programmatic access returning original JSON format
3. Sequential: tt/vow-b.Build.sh and tt/vow-t.Test.sh
Verify: tt/vow-b.Build.sh

**[260124-0822] rough**

Change jjx_saddle default output from JSON to display-ready markdown. Add --json flag for programmatic access.

**Current output (JSON):**
```json
{
  "heat_silks": "...",
  "paddock_content": "...",
  "pace_coronet": "₢XXXXX",
  ...
}
```

**New default output (markdown):**
```markdown
## Pace: {silks} (₢{coronet}) [{state}]

**Heat:** {heat_silks} (₣{firemark})

### Specification
{spec_content}

### Direction
{direction_content or omit section if rough}

### Paddock
{paddock_content or 'No paddock file.'}

### Steeplechase History
{recent_work formatted or 'No prior attempts.'}
```

**Implementation:**
- Change default output to markdown format
- Add `--json` flag to get current JSON output (for programmatic use)
- Update /jjc-heat-mount slash command to use `--json` flag

**JJSA update:**
- Update JJSCSD-saddle.adoc: document markdown as default stdout, add --json flag option

**Files:** jjrq_query.rs, jjrx_cli.rs, JJSCSD-saddle.adoc

**[260124-0744] bridled**

Add --markdown flag to jjx_saddle that outputs ready-to-display markdown for /jjc-heat-mount.

**Output format:**
```markdown
## Pace: {silks} (₢{coronet}) [{state}]

**Heat:** {heat_silks} (₣{firemark})

### Specification
{spec_content}

### Paddock
{paddock_content or 'No paddock file.'}

### Steeplechase History
{rein_output or 'No prior attempts.'}
```

**Implementation:**
- Add `--markdown` flag to saddle command
- Include spec content inline (not just path)
- Include paddock content if file exists
- Include steeplechase history (reuse rein logic or call rein internally)
- State indicators: `[rough]`, `[bridled]`

**Files:** jjx_saddle.rs (add flag handling, read spec/paddock files, integrate rein output)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrq_query.rs, jjrx_cli.rs (2 files)
Steps:
1. Add markdown field to jjrq_SaddleArgs struct
2. Add --markdown flag to SaddleArgs in jjrx_cli.rs
3. In jjrq_run_saddle, when args.markdown is true, output markdown format instead of JSON
4. Markdown format: H2 header with pace silks/coronet/state, Heat line with heat silks/firemark, H3 Specification with spec content, H3 Paddock with paddock content or No paddock file message, H3 Steeplechase History with recent_work entries or No prior attempts message
Verify: tt/vow-b.Build.sh

**[260124-0742] rough**

Add --markdown flag to jjx_saddle that outputs ready-to-display markdown for /jjc-heat-mount.

**Output format:**
```markdown
## Pace: {silks} (₢{coronet}) [{state}]

**Heat:** {heat_silks} (₣{firemark})

### Specification
{spec_content}

### Paddock
{paddock_content or 'No paddock file.'}

### Steeplechase History
{rein_output or 'No prior attempts.'}
```

**Implementation:**
- Add `--markdown` flag to saddle command
- Include spec content inline (not just path)
- Include paddock content if file exists
- Include steeplechase history (reuse rein logic or call rein internally)
- State indicators: `[rough]`, `[bridled]`

**Files:** jjx_saddle.rs (add flag handling, read spec/paddock files, integrate rein output)

### fix-nominate-test (₢AFAAa) [complete]

**[260123-1628] complete**

Fix test jjtg_nominate_creates_heat which expects Racing but gets Stabled.

The test at src/jjtg_gallops.rs:432 was written before nominate defaulted to stabled. Update the test assertion to expect Stabled instead of Racing, matching the current behavior from the nominate-defaults-stabled pace.

**File:** Tools/jjk/vov_veiled/src/jjtg_gallops.rs (line ~432)
**Change:** `assert_eq\!(heat.status, HeatStatus::Racing)` → `assert_eq\!(heat.status, HeatStatus::Stabled)`

**[260123-1627] rough**

Fix test jjtg_nominate_creates_heat which expects Racing but gets Stabled.

The test at src/jjtg_gallops.rs:432 was written before nominate defaulted to stabled. Update the test assertion to expect Stabled instead of Racing, matching the current behavior from the nominate-defaults-stabled pace.

**File:** Tools/jjk/vov_veiled/src/jjtg_gallops.rs (line ~432)
**Change:** `assert_eq\!(heat.status, HeatStatus::Racing)` → `assert_eq\!(heat.status, HeatStatus::Stabled)`

### split-jjd-subfiles (₢AFAAW) [complete]

**[260123-1430] complete**

Split JJD-GallopsData.adoc into JJSA (main) + subfiles:

**Pattern:**
- JJSA-GallopsData.adoc: Main file with mapping section, all [[anchors]], linked term definitions
- JJSCxx-command.adoc: One per CLI command (nominate, slate, rail, draft, tally, furlough, retire, validate, muster, saddle, scout, parade, rein, notch, chalk)
- JJSRxx-routine.adoc: One per routine (load, save, persist, wrap)

**File naming:**
- Commands: JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc
- Routines: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc

**Main file keeps:**
- Mapping section with all :jjd*: attributes
- All [[anchor]] definitions with linked term headers
- include:: directives to pull in subfile content

**Subfiles contain:**
- Only the procedure body (Arguments, Behavior, Stdout, Exit Status sections)
- No anchors, no linked terms

**Constraint:** Subfile acronyms never start with A (reserved for JJSA).

**[260123-1418] bridled**

Split JJD-GallopsData.adoc into JJSA (main) + subfiles:

**Pattern:**
- JJSA-GallopsData.adoc: Main file with mapping section, all [[anchors]], linked term definitions
- JJSCxx-command.adoc: One per CLI command (nominate, slate, rail, draft, tally, furlough, retire, validate, muster, saddle, scout, parade, rein, notch, chalk)
- JJSRxx-routine.adoc: One per routine (load, save, persist, wrap)

**File naming:**
- Commands: JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc
- Routines: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc

**Main file keeps:**
- Mapping section with all :jjd*: attributes
- All [[anchor]] definitions with linked term headers
- include:: directives to pull in subfile content

**Subfiles contain:**
- Only the procedure body (Arguments, Behavior, Stdout, Exit Status sections)
- No anchors, no linked terms

**Constraint:** Subfile acronyms never start with A (reserved for JJSA).

*Direction:* Phase 1: 19 haiku agents in parallel
Agent: haiku
Cardinality: 19 parallel
Files: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc, JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc (19 files)
Steps:
1. Each agent reads JJD-GallopsData.adoc
2. Extracts procedure body for its assigned operation or routine
3. Procedure body is everything AFTER the section header line, BEFORE the next anchor or section
4. Do NOT include the anchor line or the section header line
5. Write to Tools/jjk/vov_veiled/JJSxxx-name.adoc

Phase 2: 1 sonnet agent sequential
Agent: sonnet
Files: JJD-GallopsData.adoc renamed to JJSA-GallopsData.adoc, CLAUDE.md (2 files)
Steps:
1. Rename JJD-GallopsData.adoc to JJSA-GallopsData.adoc
2. For each extracted section, replace procedure body with include directive
3. Update CLAUDE.md: change JJD mapping to JJSA, add all 19 new acronym mappings

Phase 3: 1 sonnet review
Agent: sonnet
Steps:
1. Verify all 19 subfiles exist
2. Verify JJSA has 19 include directives
3. Verify no orphaned procedure content in JJSA
4. Verify all anchors and linked terms remain in JJSA

Verify: ls Tools/jjk/vov_veiled/JJS*.adoc and grep include JJSA-GallopsData.adoc

**[260123-1407] bridled**

Split JJD-GallopsData.adoc into JJSA (main) + subfiles:

**Pattern:**
- JJSA-GallopsData.adoc: Main file with mapping section, all [[anchors]], linked term definitions
- JJSCxx-command.adoc: One per CLI command (nominate, slate, rail, draft, tally, furlough, retire, validate, muster, saddle, scout, parade, rein, notch, chalk)
- JJSRxx-routine.adoc: One per routine (load, save, persist, wrap)

**File naming:**
- Commands: JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc
- Routines: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc

**Main file keeps:**
- Mapping section with all :jjd*: attributes
- All [[anchor]] definitions with linked term headers
- include:: directives to pull in subfile content

**Subfiles contain:**
- Only the procedure body (Arguments, Behavior, Stdout, Exit Status sections)
- No anchors, no linked terms

**Constraint:** Subfile acronyms never start with A (reserved for JJSA).

*Direction:* Phase 1: 19 haiku agents in parallel
Agent: haiku
Cardinality: 19 parallel
Files: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc, JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc (19 files)
Steps:
1. Each agent reads JJD-GallopsData.adoc
2. Extracts procedure body for its assigned operation or routine
3. Procedure body is everything AFTER the section header line, BEFORE the next anchor or section
4. Do NOT include the anchor line or the section header line
5. Write to Tools/jjk/vov_veiled/JJSxxx-name.adoc

Phase 2: 1 sonnet agent sequential
Agent: sonnet
Files: JJD-GallopsData.adoc renamed to JJSA-GallopsData.adoc, CLAUDE.md (2 files)
Steps:
1. Rename JJD-GallopsData.adoc to JJSA-GallopsData.adoc
2. For each extracted section, replace procedure body with include directive
3. Update CLAUDE.md: change JJD mapping to JJSA, add all 19 new acronym mappings

Phase 3: 1 sonnet review
Agent: sonnet
Steps:
1. Verify all 19 subfiles exist
2. Verify JJSA has 19 include directives
3. Verify no orphaned procedure content in JJSA
4. Verify all anchors and linked terms remain in JJSA

Verify: ls Tools/jjk/vov_veiled/JJS*.adoc and grep include JJSA-GallopsData.adoc

**[260123-1403] rough**

Split JJD-GallopsData.adoc into JJSA (main) + subfiles:

**Pattern:**
- JJSA-GallopsData.adoc: Main file with mapping section, all [[anchors]], linked term definitions
- JJSCxx-command.adoc: One per CLI command (nominate, slate, rail, draft, tally, furlough, retire, validate, muster, saddle, scout, parade, rein, notch, chalk)
- JJSRxx-routine.adoc: One per routine (load, save, persist, wrap)

**File naming:**
- Commands: JJSCNO-nominate.adoc, JJSCSL-slate.adoc, JJSCRL-rail.adoc, JJSCDR-draft.adoc, JJSCTL-tally.adoc, JJSCFU-furlough.adoc, JJSCRT-retire.adoc, JJSCVL-validate.adoc, JJSCMU-muster.adoc, JJSCSD-saddle.adoc, JJSCSC-scout.adoc, JJSCPD-parade.adoc, JJSCRN-rein.adoc, JJSCNC-notch.adoc, JJSCCH-chalk.adoc
- Routines: JJSRLD-load.adoc, JJSRSV-save.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc

**Main file keeps:**
- Mapping section with all :jjd*: attributes
- All [[anchor]] definitions with linked term headers
- include:: directives to pull in subfile content

**Subfiles contain:**
- Only the procedure body (Arguments, Behavior, Stdout, Exit Status sections)
- No anchors, no linked terms

**Constraint:** Subfile acronyms never start with A (reserved for JJSA).

### jjd-to-jjsa-references (₢AFAAX) [complete]

**[260123-1700] complete**

After JJD is renamed to JJSA, update all remaining references.

**At execution time, rescan for JJD references:**
1. Run: jjx_scout JJD --actionable
2. For each pace spec with JJD reference: reslate to replace JJD with JJSA
3. Edit paddock files (jjp_*.md) to update References sections
4. Edit other files: CLAUDE.md, jjc-pace-bridle.md, jji_itch.md, VOS-VoxObscuraSpec.adoc, VLS-VoxLiturgicalSpec.adoc

**Do NOT update:**
- Completed pace specs in gallops.json (historical record)
- Retired heat trophies in .claude/jjm/retired/
- RBM-history/ or Study/ directories (false positives)

**Constraint:** This pace depends on split-jjd-subfiles completing first.

**[260123-1656] complete**

After JJD is renamed to JJSA, update all remaining references.

**At execution time, rescan for JJD references:**
1. Run: jjx_scout JJD --actionable
2. For each pace spec with JJD reference: reslate to replace JJD with JJSA
3. Edit paddock files (jjp_*.md) to update References sections
4. Edit other files: CLAUDE.md, jjc-pace-bridle.md, jji_itch.md, VOS-VoxObscuraSpec.adoc, VLS-VoxLiturgicalSpec.adoc

**Do NOT update:**
- Completed pace specs in gallops.json (historical record)
- Retired heat trophies in .claude/jjm/retired/
- RBM-history/ or Study/ directories (false positives)

**Constraint:** This pace depends on split-jjd-subfiles completing first.

**[260123-1412] rough**

After JJD is renamed to JJSA, update all remaining references.

**At execution time, rescan for JJD references:**
1. Run: jjx_scout JJD --actionable
2. For each pace spec with JJD reference: reslate to replace JJD with JJSA
3. Edit paddock files (jjp_*.md) to update References sections
4. Edit other files: CLAUDE.md, jjc-pace-bridle.md, jji_itch.md, VOS-VoxObscuraSpec.adoc, VLS-VoxLiturgicalSpec.adoc

**Do NOT update:**
- Completed pace specs in gallops.json (historical record)
- Retired heat trophies in .claude/jjm/retired/
- RBM-history/ or Study/ directories (false positives)

**Constraint:** This pace depends on split-jjd-subfiles completing first.

### mount-requires-approval (₢AFAAQ) [complete]

**[260123-1203] complete**

Modify the /jjc-heat-mount slash command to NOT automatically begin work on a bridled pace. Instead, when mount detects the next pace is bridled, it should display the pace details and ask for explicit user approval before spawning the agent. This ensures human oversight of every autonomous execution commitment.

**[260123-1159] bridled**

Modify the /jjc-heat-mount slash command to NOT automatically begin work on a bridled pace. Instead, when mount detects the next pace is bridled, it should display the pace details and ask for explicit user approval before spawning the agent. This ensures human oversight of every autonomous execution commitment.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: .claude/commands/jjc-heat-mount.md (1 file)
Steps:
1. Read .claude/commands/jjc-heat-mount.md
2. Find Step 4 bridled branch (the section starting with: If pace_state is bridled)
3. Before the line that spawns a Task agent, add a confirmation prompt
4. The prompt should show: pace silks, coronet, spec summary, and direction summary
5. Ask user to confirm with options like: Proceed with autonomous execution / Stop and work interactively / Abort
6. Only spawn the Task agent if user confirms proceed
7. If user chooses interactive, suggest running mount again after unbridling
Verify: Read the file and confirm the approval gate is in place before agent spawn

**[260123-1140] rough**

Modify the /jjc-heat-mount slash command to NOT automatically begin work on a bridled pace. Instead, when mount detects the next pace is bridled, it should display the pace details and ask for explicit user approval before spawning the agent. This ensures human oversight of every autonomous execution commitment.

### reslate-resets-bridled-to-rough (₢AFAAT) [complete]

**[260123-1205] complete**

Bug: Reslating a bridled pace inherits stale direction.

**Problem:**
When jjx_tally receives new spec text for a bridled pace without explicit --state, it inherits the bridled state with the old direction. The new spec and old direction are now mismatched.

**Solution:**
In jjx_tally, detect when:
1. Pace is currently bridled
2. New spec text is provided (stdin not empty)
3. No explicit --state argument passed

In this case, auto-reset to rough state (clear direction). This enforces the invariant: "new spec invalidates old direction."

**Files:**
- Tools/jjk/vov_veiled/src/jjrg_tally.rs (behavior change)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (document the invariant)

**Acceptance:**
- Reslating a bridled pace without --state resets to rough
- Reslating a bridled pace with explicit --state bridled --direction keeps it bridled (intentional re-bridle)
- Reslating a rough pace still inherits rough (no change to current behavior)

**[260123-1158] bridled**

Bug: Reslating a bridled pace inherits stale direction.

**Problem:**
When jjx_tally receives new spec text for a bridled pace without explicit --state, it inherits the bridled state with the old direction. The new spec and old direction are now mismatched.

**Solution:**
In jjx_tally, detect when:
1. Pace is currently bridled
2. New spec text is provided (stdin not empty)
3. No explicit --state argument passed

In this case, auto-reset to rough state (clear direction). This enforces the invariant: "new spec invalidates old direction."

**Files:**
- Tools/jjk/vov_veiled/src/jjrg_tally.rs (behavior change)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (document the invariant)

**Acceptance:**
- Reslating a bridled pace without --state resets to rough
- Reslating a bridled pace with explicit --state bridled --direction keeps it bridled (intentional re-bridle)
- Reslating a rough pace still inherits rough (no change to current behavior)

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/src/jjro_ops.rs, Tools/jjk/vov_veiled/JJD-GallopsData.adoc (2 files)
Steps:
1. In jjro_ops.rs, find the jjrg_tally function
2. Locate the match arm for state inherited and was bridled (the None, jjrg_PaceState::Bridled case)
3. Modify this arm: if args.text is Some (new spec provided), force state to rough and direction to None instead of inheriting bridled state
4. This requires restructuring the state determination to check for the new-text-invalidates-bridled condition before the current logic
5. In JJD-GallopsData.adoc, find the jjdo_tally algorithm section around line 1799
6. Add a new step after Determine new state: If state would inherit bridled AND stdin provided new text, reset state to rough (spec change invalidates direction)
7. Update the direction determination step to reflect this invariant
Verify: ./tt/vow-b.Build.sh

**[260123-1156] rough**

Bug: Reslating a bridled pace inherits stale direction.

**Problem:**
When jjx_tally receives new spec text for a bridled pace without explicit --state, it inherits the bridled state with the old direction. The new spec and old direction are now mismatched.

**Solution:**
In jjx_tally, detect when:
1. Pace is currently bridled
2. New spec text is provided (stdin not empty)
3. No explicit --state argument passed

In this case, auto-reset to rough state (clear direction). This enforces the invariant: "new spec invalidates old direction."

**Files:**
- Tools/jjk/vov_veiled/src/jjrg_tally.rs (behavior change)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (document the invariant)

**Acceptance:**
- Reslating a bridled pace without --state resets to rough
- Reslating a bridled pace with explicit --state bridled --direction keeps it bridled (intentional re-bridle)
- Reslating a rough pace still inherits rough (no change to current behavior)

**[260123-1153] bridled**

Bug: /jjc-pace-reslate inherits stale direction when reslating a bridled pace.

**Problem:**
When reslating a bridled pace, jjx_tally inherits the old direction even though the spec text changed. This causes the pace to have mismatched spec (new) and direction (old/stale).

**Solution:**
In /jjc-pace-reslate (Step 4), detect if the pace is currently bridled. If so, explicitly pass `--state rough` to reset to rough state, forcing re-evaluation via /jjc-pace-bridle.

**Change:**
In `.claude/commands/jjc-pace-reslate.md`, Step 4 should:
1. Check current pace state before calling jjx_tally
2. If state is "bridled", add `--state rough` to the jjx_tally call
3. Report: "Note: Pace was bridled; reset to rough. Re-run /jjc-pace-bridle to set new direction."

**Files:**
- .claude/commands/jjc-pace-reslate.md

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: .claude/commands/jjc-pace-reslate.md (1 file)
Steps:
1. Read .claude/commands/jjc-pace-reslate.md
2. In Step 4, before the jjx_tally calls, add logic to detect if pace is bridled
3. Add a paragraph after the Step 4 heading that explains: fetch current pace state, if bridled then add --state rough to the jjx_tally call
4. Update both jjx_tally command examples (rename case and non-rename case) to show the --state rough variant
5. Add a note after the commands: If pace was bridled, report to user that it was reset to rough and suggest re-running /jjc-pace-bridle
Verify: Read the file to confirm changes are present

**[260123-1152] rough**

Bug: /jjc-pace-reslate inherits stale direction when reslating a bridled pace.

**Problem:**
When reslating a bridled pace, jjx_tally inherits the old direction even though the spec text changed. This causes the pace to have mismatched spec (new) and direction (old/stale).

**Solution:**
In /jjc-pace-reslate (Step 4), detect if the pace is currently bridled. If so, explicitly pass `--state rough` to reset to rough state, forcing re-evaluation via /jjc-pace-bridle.

**Change:**
In `.claude/commands/jjc-pace-reslate.md`, Step 4 should:
1. Check current pace state before calling jjx_tally
2. If state is "bridled", add `--state rough` to the jjx_tally call
3. Report: "Note: Pace was bridled; reset to rough. Re-run /jjc-pace-bridle to set new direction."

**Files:**
- .claude/commands/jjc-pace-reslate.md

### muster-status-filter (₢AFAAD) [complete]

**[260123-1211] complete**

Fix: /jjc-heat-mount calls `jjx_muster --status racing` but jjx_muster doesn't support --status flag.

**Solution:** Option 2 — Remove --status from slash command, filter client-side.

**Rationale:**
- YAGNI: Only jjc-heat-mount needs this filtering
- JJD spec defines {jjda_status} but doesn't assign it to muster's arguments
- Bash filtering on TSV is trivial

**Change:**
In `.claude/commands/jjc-heat-mount.md`, replace:
```
./tt/vvw-r.RunVVX.sh jjx_muster --status racing
```
with:
```
./tt/vvw-r.RunVVX.sh jjx_muster | grep $'\t'racing$'\t'
```

**Files:**
- .claude/commands/jjc-heat-mount.md (line ~21)

**[260123-1150] bridled**

Fix: /jjc-heat-mount calls `jjx_muster --status racing` but jjx_muster doesn't support --status flag.

**Solution:** Option 2 — Remove --status from slash command, filter client-side.

**Rationale:**
- YAGNI: Only jjc-heat-mount needs this filtering
- JJD spec defines {jjda_status} but doesn't assign it to muster's arguments
- Bash filtering on TSV is trivial

**Change:**
In `.claude/commands/jjc-heat-mount.md`, replace:
```
./tt/vvw-r.RunVVX.sh jjx_muster --status racing
```
with:
```
./tt/vvw-r.RunVVX.sh jjx_muster | grep $'\t'racing$'\t'
```

**Files:**
- .claude/commands/jjc-heat-mount.md (line ~21)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: .claude/commands/jjc-heat-mount.md (1 file)
Steps:
1. Read .claude/commands/jjc-heat-mount.md
2. Find the line with jjx_muster --status racing
3. Replace with: jjx_muster followed by pipe to grep for tab-racing-tab pattern
4. The grep pattern filters TSV output where the status column equals racing
Verify: Read the file and confirm the change

**[260123-1149] bridled**

Fix: /jjc-heat-mount calls `jjx_muster --status racing` but jjx_muster doesn't support --status flag.

**Solution:** Option 2 — Remove --status from slash command, filter client-side.

**Rationale:**
- YAGNI: Only jjc-heat-mount needs this filtering
- JJD spec defines {jjda_status} but doesn't assign it to muster's arguments
- Bash filtering on TSV is trivial

**Change:**
In `.claude/commands/jjc-heat-mount.md`, replace:
```
./tt/vvw-r.RunVVX.sh jjx_muster --status racing
```
with:
```
./tt/vvw-r.RunVVX.sh jjx_muster | grep $'\t'racing$'\t'
```

**Files:**
- .claude/commands/jjc-heat-mount.md (line ~21)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrx_cli.rs, jjrq_query.rs (2 files)
Steps:
1. Add status: Option<String> to zjjrx_MusterArgs with #[arg(long)]
2. Add status field to jjrq_MusterArgs in jjrq_query.rs
3. In jjrq_run_muster, filter heats if status is Some (match racing/stabled)
4. Pass status from CLI args to lib args in zjjrx_run_muster
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260123-1040] bridled**

Fix: /jjc-heat-mount calls `jjx_muster --status racing` but jjx_muster doesn't support --status flag.

**Bug:** Slash command assumes CLI filtering that doesn't exist.

**Options:**
1. Add --status flag to jjx_muster (filter server-side)
2. Remove --status from slash command, filter client-side

**Preference:** Option 1 — filtering in Rust is cleaner, reduces output parsing in slash command.

**Files:**
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (add --status to MusterArgs)
- .claude/commands/jjc-heat-mount.md (verify usage after fix)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrx_cli.rs, jjrq_query.rs (2 files)
Steps:
1. Add status: Option<String> to zjjrx_MusterArgs with #[arg(long)]
2. Add status field to jjrq_MusterArgs in jjrq_query.rs
3. In jjrq_run_muster, filter heats if status is Some (match racing/stabled)
4. Pass status from CLI args to lib args in zjjrx_run_muster
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260119-0939] rough**

Fix: /jjc-heat-mount calls `jjx_muster --status racing` but jjx_muster doesn't support --status flag.

**Bug:** Slash command assumes CLI filtering that doesn't exist.

**Options:**
1. Add --status flag to jjx_muster (filter server-side)
2. Remove --status from slash command, filter client-side

**Preference:** Option 1 — filtering in Rust is cleaner, reduces output parsing in slash command.

**Files:**
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (add --status to MusterArgs)
- .claude/commands/jjc-heat-mount.md (verify usage after fix)

### fix-hallmark-and-rein-no-rbm (₢AFAAG) [complete]

**[260123-1032] complete**

Removed vestigial BRAND field from commit format. New format: jjb:HALLMARK:IDENTITY:ACTION: message. Rein now correctly finds steeplechase entries.

**[260120-1749] rough**

The git commit string still has 'rbm' in it and I think that is why rein is not working.  Remove that and validate rein again.

**[260120-1748] rough**

The git commit string still has 'rbm' in it and I think that is why rein is not working.  Remove that and vvalidate rein again.

### gallops-commit-routine (₢AFAAB) [complete]

**[260123-1047] complete**

Implemented jjri_persist routine centralizing gallops+paddock commit logic; refactored 5 commands to use it; documented in JJD spec.

**[260123-1040] bridled**

Formalize a JJD routine for committing gallops+paddock. Audit all jjx commands for consistent use.

**Goal:** Single pattern for 'commit gallops+paddock' — no ad-hoc implementations scattered across commands.

**Tasks:**
1. Define routine in JJD spec (name TBD: jjdr_commit_state? jjdr_persist?)
2. Audit each jjx command:
   - slate — currently commits both
   - tally — currently commits both
   - nominate — check
   - rail — check
   - draft — check
   - furlough — check
   - wrap (via tally?) — check
   - bridle (via tally?) — check
3. Refactor any ad-hoc implementations to use the new routine
4. Ensure chalk remains marker-only (no state commit)

**Constraint:** Schema-stable — this is implementation refactoring, not data model change.

*Direction:* Agent: sonnet
Cardinality: 2 parallel + sequential build
Files: jjri_io.rs, jjrx_cli.rs, lib.rs, JJD-GallopsData.adoc (4 files)
Steps:
1. Agent A (sonnet): Implement jjri_persist routine in jjri_io.rs that saves gallops and commits gallops+paddock. Refactor zjjrx_run_nominate, zjjrx_run_slate, zjjrx_run_rail, zjjrx_run_tally, zjjrx_run_draft, zjjrx_run_furlough in jjrx_cli.rs to use new routine. Normalize draft to use machine_commit pattern. Add re-export in lib.rs.
2. Agent B (sonnet): Document jjdr_persist routine in JJD-GallopsData.adoc following existing routine patterns (jjdr_load, jjdr_save). Add to mapping section and Routines section.
3. Sequential: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk
Verify: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk
Notes:
- retire has extra trophy file - keep its custom commit logic, do not use jjri_persist
- chalk creates empty commits only - already correct, no changes needed
- jjri_persist signature: fn jjri_persist(lock: &vvcc_CommitLock, gallops: &jjrg_Gallops, file: &Path, firemark: &Firemark, message: String, size_limit: u64) -> Result<String, String>

**[260119-0924] rough**

Formalize a JJD routine for committing gallops+paddock. Audit all jjx commands for consistent use.

**Goal:** Single pattern for 'commit gallops+paddock' — no ad-hoc implementations scattered across commands.

**Tasks:**
1. Define routine in JJD spec (name TBD: jjdr_commit_state? jjdr_persist?)
2. Audit each jjx command:
   - slate — currently commits both
   - tally — currently commits both
   - nominate — check
   - rail — check
   - draft — check
   - furlough — check
   - wrap (via tally?) — check
   - bridle (via tally?) — check
3. Refactor any ad-hoc implementations to use the new routine
4. Ensure chalk remains marker-only (no state commit)

**Constraint:** Schema-stable — this is implementation refactoring, not data model change.

### wrap-notch-self-sufficiency (₢AFAAA) [complete]

**[260123-1133] complete**

Consolidate wrap and notch into self-contained vvx commands. Remove chalk from user-facing vocabulary.

## jjx_wrap (new command)

Syntax: `jjx_wrap <CORONET> [--size-limit N]`

Behavior:
1. Acquire commit lock
2. Stage all changes (git add -A)
3. Size guard check (default 50KB, configurable via --size-limit)
   - If over limit: release lock, exit 2
4. Generate commit message via Claude
5. Commit (work + gallops + paddock)
6. Transition pace state to complete
7. Create W chalk marker with outcome summary
8. Release lock, return commit hash

Exit codes: 0=success, 1=general error, 2=size guard exceeded

## jjx_notch (enhanced)

Syntax: `jjx_notch <IDENTITY> <file1> [file2...]`

- IDENTITY: coronet (5-char, pace-affiliated) or firemark (2-char, heat-only)
- File list required; empty list = exit 1
- Non-existent file in list = exit 1 (fail fast)
- Warn to stderr if uncommitted files exist outside list (see warning format)
- Create chalk marker, then commit specified files only

Warning format:
```
warning: uncommitted changes outside file list:
  modified: path/to/file.rs
```

## Slash commands

- `/jjc-pace-wrap` — thin wrapper; on exit 2, advise --size-limit flag
- `/jjc-pace-notch` — updated to require file arguments
- `/jjc-heat-chalk` — DELETE

## JJD documentation

- Add jjdr_wrap routine
- Update jjdr_notch routine (file list, heat-only)
- Remove chalk from user-facing commands section

## Files

- jjrx_cli.rs — implement jjx_wrap, enhance jjx_notch
- JJD-GallopsData.adoc — document routines
- jjc-pace-wrap.md — update
- jjc-pace-notch.md — update
- jjc-heat-chalk.md — delete

**[260123-1114] bridled**

Consolidate wrap and notch into self-contained vvx commands. Remove chalk from user-facing vocabulary.

## jjx_wrap (new command)

Syntax: `jjx_wrap <CORONET> [--size-limit N]`

Behavior:
1. Acquire commit lock
2. Stage all changes (git add -A)
3. Size guard check (default 50KB, configurable via --size-limit)
   - If over limit: release lock, exit 2
4. Generate commit message via Claude
5. Commit (work + gallops + paddock)
6. Transition pace state to complete
7. Create W chalk marker with outcome summary
8. Release lock, return commit hash

Exit codes: 0=success, 1=general error, 2=size guard exceeded

## jjx_notch (enhanced)

Syntax: `jjx_notch <IDENTITY> <file1> [file2...]`

- IDENTITY: coronet (5-char, pace-affiliated) or firemark (2-char, heat-only)
- File list required; empty list = exit 1
- Non-existent file in list = exit 1 (fail fast)
- Warn to stderr if uncommitted files exist outside list (see warning format)
- Create chalk marker, then commit specified files only

Warning format:
```
warning: uncommitted changes outside file list:
  modified: path/to/file.rs
```

## Slash commands

- `/jjc-pace-wrap` — thin wrapper; on exit 2, advise --size-limit flag
- `/jjc-pace-notch` — updated to require file arguments
- `/jjc-heat-chalk` — DELETE

## JJD documentation

- Add jjdr_wrap routine
- Update jjdr_notch routine (file list, heat-only)
- Remove chalk from user-facing commands section

## Files

- jjrx_cli.rs — implement jjx_wrap, enhance jjx_notch
- JJD-GallopsData.adoc — document routines
- jjc-pace-wrap.md — update
- jjc-pace-notch.md — update
- jjc-heat-chalk.md — delete

*Direction:* Agent: sonnet
Cardinality: 2 parallel + sequential build
Files: jjrx_cli.rs, JJD-GallopsData.adoc, jjc-pace-wrap.md, jjc-pace-notch.md, jjc-heat-chalk.md (5 files)
Steps:
1. Agent A (sonnet) edits Tools/jjk/vov_veiled/src/jjrx_cli.rs:
   - Add jjx_wrap command: new WrapArgs struct with coronet and optional size_limit fields
   - Implement zjjrx_run_wrap: acquire lock, git add -A, size guard (exit 2 if exceeded), generate message via Claude, commit, transition pace to complete via jjrg_tally, create W chalk marker, return hash
   - Add Wrap variant to jjrx_JjxCommands enum and dispatch in jjrx_run
   - Enhance jjrx_NotchArgs: add files field as Vec of String for positional args
   - Enhance zjjrx_run_notch: require non-empty files list (exit 1 if empty), check each file exists (exit 1 if not), warn to stderr about uncommitted files outside list, stage only specified files, then commit
   - Support both Coronet (5-char) and Firemark (2-char) as identity for notch
2. Agent B (sonnet) edits documentation:
   - Tools/jjk/vov_veiled/JJD-GallopsData.adoc: Add jjdr_wrap routine (new command behavior), update jjdr_notch routine (file list requirement, heat-only support, warning format)
   - .claude/commands/jjc-pace-wrap.md: Simplify to thin wrapper calling jjx_wrap, advise --size-limit on exit 2
   - .claude/commands/jjc-pace-notch.md: Update to require file arguments, document heat-only usage with Firemark
   - Delete .claude/commands/jjc-heat-chalk.md
3. Sequential: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk
Verify: cargo build succeeds, jjc-heat-chalk.md deleted
Notes:
- Exit codes for wrap: 0 success, 1 general error, 2 size guard exceeded
- Warning format for notch: warning: uncommitted changes outside file list: followed by modified: filepath lines
- Notch with empty file list must exit 1 immediately
- Notch with non-existent file must exit 1 immediately

**[260123-1111] rough**

Consolidate wrap and notch into self-contained vvx commands. Remove chalk from user-facing vocabulary.

## jjx_wrap (new command)

Syntax: `jjx_wrap <CORONET> [--size-limit N]`

Behavior:
1. Acquire commit lock
2. Stage all changes (git add -A)
3. Size guard check (default 50KB, configurable via --size-limit)
   - If over limit: release lock, exit 2
4. Generate commit message via Claude
5. Commit (work + gallops + paddock)
6. Transition pace state to complete
7. Create W chalk marker with outcome summary
8. Release lock, return commit hash

Exit codes: 0=success, 1=general error, 2=size guard exceeded

## jjx_notch (enhanced)

Syntax: `jjx_notch <IDENTITY> <file1> [file2...]`

- IDENTITY: coronet (5-char, pace-affiliated) or firemark (2-char, heat-only)
- File list required; empty list = exit 1
- Non-existent file in list = exit 1 (fail fast)
- Warn to stderr if uncommitted files exist outside list (see warning format)
- Create chalk marker, then commit specified files only

Warning format:
```
warning: uncommitted changes outside file list:
  modified: path/to/file.rs
```

## Slash commands

- `/jjc-pace-wrap` — thin wrapper; on exit 2, advise --size-limit flag
- `/jjc-pace-notch` — updated to require file arguments
- `/jjc-heat-chalk` — DELETE

## JJD documentation

- Add jjdr_wrap routine
- Update jjdr_notch routine (file list, heat-only)
- Remove chalk from user-facing commands section

## Files

- jjrx_cli.rs — implement jjx_wrap, enhance jjx_notch
- JJD-GallopsData.adoc — document routines
- jjc-pace-wrap.md — update
- jjc-pace-notch.md — update
- jjc-heat-chalk.md — delete

**[260119-0917] rough**

Consolidate wrap and notch into self-contained vvx commands. Remove chalk from user-facing vocabulary.

**Wrap changes:**
- jjx_wrap internally: chalk start → git add all (size-guarded) → chalk complete
- Returns distinct error code on size guard failure
- Slash command becomes thin wrapper, advises --size-limit on failure

**Notch changes:**
- Requires explicit file list argument (declare intent)
- Chalks first, then commits
- Warns if uncommitted files exist outside declared list

**Chalk removal:**
- Delete /jjc-heat-chalk slash command
- All chalk calls become internal to wrap/notch

**Open questions (resolve on mount):**
- Can notch affiliate with heat-only (no pace)? Needed for between-pace housekeeping.
- Should this split into multiple paces once requirements clarify?
- What's the exact CLI syntax for file list? Positional args? --files?

### add-macos-sandbox (₢AFAAE) [abandoned]

**[260123-1228] abandoned**

Create Tools/vok/src/vovs_sandbox.rs following the pattern in pb_paneboard02/poc/src/pbmbs_sandbox.rs:
1. Copy the implementation, adapting comments ("vvx" instead of "PaneBoard")
2. Keep the same Seatbelt policy: (version 1)(allow default)(deny network*)
3. In vorm_main.rs:
   - Add `mod vovs_sandbox;` near the top with other mods
   - Add `#[cfg(target_os = "macos")] vovs_sandbox::drop_network_access();` as first line of main()
4. Build with: tt/vow-b.Build.sh

**[260123-1114] bridled**

Create Tools/vok/src/vovs_sandbox.rs following the pattern in pb_paneboard02/poc/src/pbmbs_sandbox.rs:
1. Copy the implementation, adapting comments ("vvx" instead of "PaneBoard")
2. Keep the same Seatbelt policy: (version 1)(allow default)(deny network*)
3. In vorm_main.rs:
   - Add `mod vovs_sandbox;` near the top with other mods
   - Add `#[cfg(target_os = "macos")] vovs_sandbox::drop_network_access();` as first line of main()
4. Build with: tt/vow-b.Build.sh

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: vovs_sandbox.rs (create), vorm_main.rs (edit) — 2 files
Steps:
1. Read pb_paneboard02/poc/src/pbmbs_sandbox.rs as pattern source
2. Create Tools/vok/src/vovs_sandbox.rs with same implementation, change PaneBoard to vvx in comments
3. In Tools/vok/src/vorm_main.rs, add mod vovs_sandbox near other mod declarations
4. In vorm_main.rs main(), add sandbox call as first statement with cfg(target_os = macos) gate
Verify: tt/vow-b.Build.sh

**[260123-1114] rough**

Create Tools/vok/src/vovs_sandbox.rs following the pattern in pb_paneboard02/poc/src/pbmbs_sandbox.rs:
1. Copy the implementation, adapting comments ("vvx" instead of "PaneBoard")
2. Keep the same Seatbelt policy: (version 1)(allow default)(deny network*)
3. In vorm_main.rs:
   - Add `mod vovs_sandbox;` near the top with other mods
   - Add `#[cfg(target_os = "macos")] vovs_sandbox::drop_network_access();` as first line of main()
4. Build with: tt/vow-b.Build.sh

**[260120-1745] rough**

Look at paneboard's implementation of sandboxing for macos preventing ALL network access and implement this in the vvx application.

### verify-tests-block-release (₢AFAAF) [complete]

**[260123-1237] complete**

Verify that release correctly fails when tests fail.

## Goal

Confirm the test gate in vob_release() actually blocks release on test failure.

## Steps

1. Add a deliberately failing test to Tools/vok (e.g., in a test module: `assert!(false, "deliberate failure");`)
2. Run release: `./tt/vow-r.Release.sh`
3. Confirm release fails with "Tests failed" message
4. Remove the deliberate failure
5. Run release again to confirm it succeeds

## Acceptance

- Release aborts when tests fail (exit non-zero, "Tests failed" in output)
- Release succeeds when tests pass
- No permanent changes to codebase

## Files

- Tools/vok/src/*.rs (temporary test addition)
- tt/vow-r.Release.sh (invoke only, no edit)

**[260123-1126] bridled**

Verify that release correctly fails when tests fail.

## Goal

Confirm the test gate in vob_release() actually blocks release on test failure.

## Steps

1. Add a deliberately failing test to Tools/vok (e.g., in a test module: `assert!(false, "deliberate failure");`)
2. Run release: `./tt/vow-r.Release.sh`
3. Confirm release fails with "Tests failed" message
4. Remove the deliberate failure
5. Run release again to confirm it succeeds

## Acceptance

- Release aborts when tests fail (exit non-zero, "Tests failed" in output)
- Release succeeds when tests pass
- No permanent changes to codebase

## Files

- Tools/vok/src/*.rs (temporary test addition)
- tt/vow-r.Release.sh (invoke only, no edit)

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: Tools/vok/src/vorm_main.rs (1 file, temporary edit)
Steps:
1. Read Tools/vok/src/vorm_main.rs to find the test module section
2. Add a failing test function: #[test] fn test_deliberate_failure() { panic!("deliberate failure for release gate verification"); }
3. Run ./tt/vow-r.Release.sh and capture output
4. Verify output contains "Tests failed" and command exited non-zero
5. Remove the deliberate failure test from vorm_main.rs
6. Run cargo build to confirm code is clean
Verify: vorm_main.rs has no deliberate_failure test, cargo build succeeds
Notes:
- Do NOT run release a second time after removing the test
- Goal is only to confirm test failures block release
- Report the exact error message observed

**[260123-1125] rough**

Verify that release correctly fails when tests fail.

## Goal

Confirm the test gate in vob_release() actually blocks release on test failure.

## Steps

1. Add a deliberately failing test to Tools/vok (e.g., in a test module: `assert!(false, "deliberate failure");`)
2. Run release: `./tt/vow-r.Release.sh`
3. Confirm release fails with "Tests failed" message
4. Remove the deliberate failure
5. Run release again to confirm it succeeds

## Acceptance

- Release aborts when tests fail (exit non-zero, "Tests failed" in output)
- Release succeeds when tests pass
- No permanent changes to codebase

## Files

- Tools/vok/src/*.rs (temporary test addition)
- tt/vow-r.Release.sh (invoke only, no edit)

**[260120-1746] rough**

A release of vvk needs to run all tests before forming the tarball; I did not see tests run before last release

### muster-completed-defined-columns (₢AFAAH) [complete]

**[260123-1242] complete**

Redefine muster pace columns using state classification predicates.

## Goal

Change muster output from "remaining/total" to "completed/defined" semantics, where:
- `<completed>` = paces that are done
- `<defined>` = paces that count toward scope (excludes abandoned)

Reading: "3/5" means "3 of 5 defined paces are complete" — intuitive progress.

## JJD Spec Changes

### 1. Add State Classification Predicates section (after State Values)

Define two predicates that each state satisfies:

**jjdpe_defined** — "Pace contributes to heat's scope"
- rough: true
- bridled: true
- complete: true
- abandoned: false

**jjdpe_resolved** — "Pace requires no further action"
- rough: false
- bridled: false
- complete: true
- abandoned: true

Note: Future states (primeable, murky, blocked, etc.) will declare their predicate values when added. Muster logic queries predicates, not state names.

### 2. Update jjdo_muster Stdout section

Change from:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><remaining><TAB><total>`
Where:
* `<remaining>` — count of actionable paces (rough or bridled)
* `<total>` — total pace count
```

To:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><completed><TAB><defined>`
Where:
* `<completed>` — count of paces where jjdpe_defined ∧ jjdpe_resolved (i.e., complete)
* `<defined>` — count of paces where jjdpe_defined (excludes abandoned)
```

### 3. Update jjdo_muster Behavior section

Change step about counting paces to use new semantics.

## Implementation Changes (jjrq_query.rs)

In `jjrq_run_muster`:

1. Change `remaining_count` to `completed_count`:
   - Count paces where state == Complete

2. Change `total_pace_count` to `defined_count`:
   - Count paces where state != Abandoned

3. Output order unchanged: `completed_count, defined_count`

## Constraint

Schema-stable — no JSON structure changes. This is display/query logic only.

**[260123-1104] bridled**

Redefine muster pace columns using state classification predicates.

## Goal

Change muster output from "remaining/total" to "completed/defined" semantics, where:
- `<completed>` = paces that are done
- `<defined>` = paces that count toward scope (excludes abandoned)

Reading: "3/5" means "3 of 5 defined paces are complete" — intuitive progress.

## JJD Spec Changes

### 1. Add State Classification Predicates section (after State Values)

Define two predicates that each state satisfies:

**jjdpe_defined** — "Pace contributes to heat's scope"
- rough: true
- bridled: true
- complete: true
- abandoned: false

**jjdpe_resolved** — "Pace requires no further action"
- rough: false
- bridled: false
- complete: true
- abandoned: true

Note: Future states (primeable, murky, blocked, etc.) will declare their predicate values when added. Muster logic queries predicates, not state names.

### 2. Update jjdo_muster Stdout section

Change from:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><remaining><TAB><total>`
Where:
* `<remaining>` — count of actionable paces (rough or bridled)
* `<total>` — total pace count
```

To:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><completed><TAB><defined>`
Where:
* `<completed>` — count of paces where jjdpe_defined ∧ jjdpe_resolved (i.e., complete)
* `<defined>` — count of paces where jjdpe_defined (excludes abandoned)
```

### 3. Update jjdo_muster Behavior section

Change step about counting paces to use new semantics.

## Implementation Changes (jjrq_query.rs)

In `jjrq_run_muster`:

1. Change `remaining_count` to `completed_count`:
   - Count paces where state == Complete

2. Change `total_pace_count` to `defined_count`:
   - Count paces where state != Abandoned

3. Output order unchanged: `completed_count, defined_count`

## Constraint

Schema-stable — no JSON structure changes. This is display/query logic only.

*Direction:* Agent: sonnet
Cardinality: 2 parallel + sequential build
Files: JJD-GallopsData.adoc, jjrq_query.rs (2 files)
Steps:
1. Agent A (sonnet) edits JJD-GallopsData.adoc. Add jjdpe_defined and jjdpe_resolved to mapping section. Add State Classification Predicates subsection after State Values with table for rough/bridled/complete/abandoned. Update jjdo_muster Stdout to use completed/defined columns instead of remaining/total. Update Behavior step 3.
2. Agent B (haiku) edits jjrq_query.rs. In jjrq_run_muster change remaining_count to completed_count (count Complete only). Change total_pace_count to defined_count (exclude Abandoned). Update println output.
3. Sequential build.
Verify: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk

**[260123-1052] rough**

Redefine muster pace columns using state classification predicates.

## Goal

Change muster output from "remaining/total" to "completed/defined" semantics, where:
- `<completed>` = paces that are done
- `<defined>` = paces that count toward scope (excludes abandoned)

Reading: "3/5" means "3 of 5 defined paces are complete" — intuitive progress.

## JJD Spec Changes

### 1. Add State Classification Predicates section (after State Values)

Define two predicates that each state satisfies:

**jjdpe_defined** — "Pace contributes to heat's scope"
- rough: true
- bridled: true
- complete: true
- abandoned: false

**jjdpe_resolved** — "Pace requires no further action"
- rough: false
- bridled: false
- complete: true
- abandoned: true

Note: Future states (primeable, murky, blocked, etc.) will declare their predicate values when added. Muster logic queries predicates, not state names.

### 2. Update jjdo_muster Stdout section

Change from:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><remaining><TAB><total>`
Where:
* `<remaining>` — count of actionable paces (rough or bridled)
* `<total>` — total pace count
```

To:
```
`{jjdt_firemark}<TAB>{jjdt_silks}<TAB>{jjdhm_status}<TAB><completed><TAB><defined>`
Where:
* `<completed>` — count of paces where jjdpe_defined ∧ jjdpe_resolved (i.e., complete)
* `<defined>` — count of paces where jjdpe_defined (excludes abandoned)
```

### 3. Update jjdo_muster Behavior section

Change step about counting paces to use new semantics.

## Implementation Changes (jjrq_query.rs)

In `jjrq_run_muster`:

1. Change `remaining_count` to `completed_count`:
   - Count paces where state == Complete

2. Change `total_pace_count` to `defined_count`:
   - Count paces where state != Abandoned

3. Output order unchanged: `completed_count, defined_count`

## Constraint

Schema-stable — no JSON structure changes. This is display/query logic only.

**[260120-1756] rough**

Fix muster command to correctly display pace completion status. Currently showing 0/5 for ₣AC when all 5 paces are complete. Verify column semantics (Completed/Total) and ensure query returns accurate completion counts.

### wrap-recommends-clear-mount (₢AFAAU) [complete]

**[260123-1246] complete**

Add post-wrap guidance to jjx_wrap Rust output and simplify the slash command.

**Rust changes (jjx_wrap in jjrx_cli.rs):**
After successful wrap, output to stderr:
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
```
- Extract firemark from the coronet (already parsed)
- Place after the existing success output, before returning 0

**Slash command changes (.claude/commands/jjc-pace-wrap.md):**
- Remove Step 3 entirely (the "Advance to next pace" section that calls jjx_saddle)
- The Rust output now provides the guidance, reducing Claude round trips

No schema changes. Build verification: tt/vow-b.Build.sh

**[260123-1236] bridled**

Add post-wrap guidance to jjx_wrap Rust output and simplify the slash command.

**Rust changes (jjx_wrap in jjrx_cli.rs):**
After successful wrap, output to stderr:
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
```
- Extract firemark from the coronet (already parsed)
- Place after the existing success output, before returning 0

**Slash command changes (.claude/commands/jjc-pace-wrap.md):**
- Remove Step 3 entirely (the "Advance to next pace" section that calls jjx_saddle)
- The Rust output now provides the guidance, reducing Claude round trips

No schema changes. Build verification: tt/vow-b.Build.sh

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/src/jjrx_cli.rs, .claude/commands/jjc-pace-wrap.md (2 files)
Steps:
1. In jjrx_cli.rs zjjrx_run_wrap function, after the successful println of commit_hash (around line 1566), add:
   - let fm = coronet.jjrf_parent_firemark();
   - eprintln!();
   - eprintln!("Recommended: /clear then /jjc-heat-mount {}", fm.jjrf_as_str());
2. In jjc-pace-wrap.md, delete Step 3 entirely (the Advance to next pace section, lines 38-53)
Verify: tt/vow-b.Build.sh

**[260123-1235] rough**

Add post-wrap guidance to jjx_wrap Rust output and simplify the slash command.

**Rust changes (jjx_wrap in jjrx_cli.rs):**
After successful wrap, output to stderr:
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
```
- Extract firemark from the coronet (already parsed)
- Place after the existing success output, before returning 0

**Slash command changes (.claude/commands/jjc-pace-wrap.md):**
- Remove Step 3 entirely (the "Advance to next pace" section that calls jjx_saddle)
- The Rust output now provides the guidance, reducing Claude round trips

No schema changes. Build verification: tt/vow-b.Build.sh

### implement-scout-search (₢AFAAI) [complete]

**[260123-1253] complete**

Implement `jjx_scout` command and `/jjc-scout` slash command for regex search across heats and paces.

**Search behavior:**
- Case insensitive (always)
- Full regex pattern support
- Searches: pace silks, specs, directions, paddock content
- Does NOT search: steeplechase/chalk descriptions

**Filtering:**
- All heats by default (racing, stabled, retired)
- `--actionable` flag limits to rough/bridled paces only

**Output format:**
```
₣AF jjk-post-alpha-polish
  ₢AFAAI [rough] implement-scout-search
    spec: ...keyword... for searching across heats
₣AA garlanded-vok-fresh-install
  ₢AAABC [complete] related-pace-name
    paddock: ...keyword... mentioned in context
```

One entry per matching pace, grouped by heat. Shows:
- Heat firemark and silks (group header)
- Coronet, state, pace silks (pace line)
- Field name and excerpt with match context (match line)

Multiple matches in same pace: show first match only (keep output scannable).

**CLI signature:**
```
jjx_scout <PATTERN> [--actionable]
```

No schema changes required — queries existing gallops data via jjdr_load.

**[260123-1219] bridled**

Implement `jjx_scout` command and `/jjc-scout` slash command for regex search across heats and paces.

**Search behavior:**
- Case insensitive (always)
- Full regex pattern support
- Searches: pace silks, specs, directions, paddock content
- Does NOT search: steeplechase/chalk descriptions

**Filtering:**
- All heats by default (racing, stabled, retired)
- `--actionable` flag limits to rough/bridled paces only

**Output format:**
```
₣AF jjk-post-alpha-polish
  ₢AFAAI [rough] implement-scout-search
    spec: ...keyword... for searching across heats
₣AA garlanded-vok-fresh-install
  ₢AAABC [complete] related-pace-name
    paddock: ...keyword... mentioned in context
```

One entry per matching pace, grouped by heat. Shows:
- Heat firemark and silks (group header)
- Coronet, state, pace silks (pace line)
- Field name and excerpt with match context (match line)

Multiple matches in same pace: show first match only (keep output scannable).

**CLI signature:**
```
jjx_scout <PATTERN> [--actionable]
```

No schema changes required — queries existing gallops data via jjdr_load.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/src/jjrq_query.rs, Tools/jjk/vov_veiled/src/jjrx_cli.rs, Tools/jjk/vov_veiled/JJD-GallopsData.adoc, .claude/commands/jjc-scout.md (4 files)
Steps:
1. In jjrq_query.rs: Add jjrq_ScoutArgs struct with file, pattern, actionable fields
2. In jjrq_query.rs: Add jjrq_run_scout function that loads gallops, iterates heats/paces, regex searches silks/spec/direction/paddock, outputs grouped format
3. In jjrx_cli.rs: Add Scout variant to jjrx_JjxCommands enum with jjx_scout name
4. In jjrx_cli.rs: Add zjjrx_ScoutArgs struct with pattern positional and --actionable flag
5. In jjrx_cli.rs: Add zjjrx_run_scout that calls jjrq_run_scout
6. In jjrx_cli.rs: Add Scout dispatch in jjrx_dispatch match
7. In JJD-GallopsData.adoc: Add jjdo_scout operation definition following existing operation patterns (Arguments, Stdout, Exit Status, Behavior sections)
8. Create .claude/commands/jjc-scout.md slash command that invokes jjx_scout
Verify: tt/vow-b.Build.sh

**[260123-1216] rough**

Implement `jjx_scout` command and `/jjc-scout` slash command for regex search across heats and paces.

**Search behavior:**
- Case insensitive (always)
- Full regex pattern support
- Searches: pace silks, specs, directions, paddock content
- Does NOT search: steeplechase/chalk descriptions

**Filtering:**
- All heats by default (racing, stabled, retired)
- `--actionable` flag limits to rough/bridled paces only

**Output format:**
```
₣AF jjk-post-alpha-polish
  ₢AFAAI [rough] implement-scout-search
    spec: ...keyword... for searching across heats
₣AA garlanded-vok-fresh-install
  ₢AAABC [complete] related-pace-name
    paddock: ...keyword... mentioned in context
```

One entry per matching pace, grouped by heat. Shows:
- Heat firemark and silks (group header)
- Coronet, state, pace silks (pace line)
- Field name and excerpt with match context (match line)

Multiple matches in same pace: show first match only (keep output scannable).

**CLI signature:**
```
jjx_scout <PATTERN> [--actionable]
```

No schema changes required — queries existing gallops data via jjdr_load.

**[260120-1805] rough**

Implement jjx_scout command and /jjc-pace-scout slash command for searching across heats and paces by keyword. Should search pace silks, specs, and directions for matches. No schema changes required — queries existing gallops data. Return matching paces with their heat context. Consider: case sensitivity, regex support, output format (which fields to show), whether to search paddock content too.

### consolidate-parade-slash-commands (₢AFAAS) [complete]

**[260123-1301] complete**

Consolidate parade views: simplify Rust CLI, JJD spec, and slash commands.

## Rust CLI Changes

**Current:** `jjx_parade <FIREMARK> --format <mode> [--pace <coronet>] [--remaining]`

**New:** `jjx_parade <TARGET> [--full] [--remaining]`

TARGET detection:
- 2 chars / ₣XX → firemark (heat view)
- 5 chars / ₢XXXXX → coronet (pace view)

Behavior:
- `jjx_parade AF` → numbered list of paces in heat
- `jjx_parade AF --full` → paddock + all specs for heat
- `jjx_parade AFAAB` → full spec/direction for one pace

**Delete from jjrx_cli.rs:**
- `zjjrx_ParadeFormatArg` enum
- `--format` argument
- `--pace` argument

**Delete from jjrq_query.rs:**
- `jjrq_ParadeFormat` enum
- Format matching logic

**Replace with:**
- `jjrq_ParadeArgs { file, target: String, full: bool, remaining: bool }`
- Target parsing: `Firemark::jjrf_parse` vs `Coronet::jjrc_parse`
- Branch on target type for output

## JJD Spec Changes

**Update jjdo_parade:**

Arguments:
- `<TARGET>` — Firemark (heat) or Coronet (pace)
- `--full` — Show paddock and full specs (heat mode only)
- `--remaining` — Exclude complete/abandoned paces

Behavior by target type:
- Heat + no flags → numbered pace list
- Heat + --full → paddock + all pace specs
- Coronet → full tack detail for that pace

**Delete:**
- ParadeFormat type definition
- References to --format and --pace flags

## Slash Command Changes

**Create:** `.claude/commands/jjc-parade.md`

**Delete:**
- `jjc-parade-overview.md`
- `jjc-parade-order.md`
- `jjc-parade-detail.md`
- `jjc-parade-full.md`

**New slash command logic:**
- No args → list of racing heat
- `full` keyword → full view
- `full <firemark>` → full view of specific heat
- `<firemark>` → list of specific heat
- `<coronet>` → pace detail

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs (CLI args, dispatch)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (parade logic)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (jjdo_parade spec)
- .claude/commands/jjc-parade.md (new consolidated command)
- .claude/commands/jjc-parade-overview.md (delete)
- .claude/commands/jjc-parade-order.md (delete)
- .claude/commands/jjc-parade-detail.md (delete)
- .claude/commands/jjc-parade-full.md (delete)

## Acceptance

- `jjx_parade AF` shows numbered list
- `jjx_parade AF --full` shows paddock + specs
- `jjx_parade AFAAB` shows pace detail
- `/jjc-parade` works with smart defaults
- Old slash commands deleted
- JJD spec updated
- No --format or --pace flags remain

**[260123-1249] bridled**

Consolidate parade views: simplify Rust CLI, JJD spec, and slash commands.

## Rust CLI Changes

**Current:** `jjx_parade <FIREMARK> --format <mode> [--pace <coronet>] [--remaining]`

**New:** `jjx_parade <TARGET> [--full] [--remaining]`

TARGET detection:
- 2 chars / ₣XX → firemark (heat view)
- 5 chars / ₢XXXXX → coronet (pace view)

Behavior:
- `jjx_parade AF` → numbered list of paces in heat
- `jjx_parade AF --full` → paddock + all specs for heat
- `jjx_parade AFAAB` → full spec/direction for one pace

**Delete from jjrx_cli.rs:**
- `zjjrx_ParadeFormatArg` enum
- `--format` argument
- `--pace` argument

**Delete from jjrq_query.rs:**
- `jjrq_ParadeFormat` enum
- Format matching logic

**Replace with:**
- `jjrq_ParadeArgs { file, target: String, full: bool, remaining: bool }`
- Target parsing: `Firemark::jjrf_parse` vs `Coronet::jjrc_parse`
- Branch on target type for output

## JJD Spec Changes

**Update jjdo_parade:**

Arguments:
- `<TARGET>` — Firemark (heat) or Coronet (pace)
- `--full` — Show paddock and full specs (heat mode only)
- `--remaining` — Exclude complete/abandoned paces

Behavior by target type:
- Heat + no flags → numbered pace list
- Heat + --full → paddock + all pace specs
- Coronet → full tack detail for that pace

**Delete:**
- ParadeFormat type definition
- References to --format and --pace flags

## Slash Command Changes

**Create:** `.claude/commands/jjc-parade.md`

**Delete:**
- `jjc-parade-overview.md`
- `jjc-parade-order.md`
- `jjc-parade-detail.md`
- `jjc-parade-full.md`

**New slash command logic:**
- No args → list of racing heat
- `full` keyword → full view
- `full <firemark>` → full view of specific heat
- `<firemark>` → list of specific heat
- `<coronet>` → pace detail

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs (CLI args, dispatch)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (parade logic)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (jjdo_parade spec)
- .claude/commands/jjc-parade.md (new consolidated command)
- .claude/commands/jjc-parade-overview.md (delete)
- .claude/commands/jjc-parade-order.md (delete)
- .claude/commands/jjc-parade-detail.md (delete)
- .claude/commands/jjc-parade-full.md (delete)

## Acceptance

- `jjx_parade AF` shows numbered list
- `jjx_parade AF --full` shows paddock + specs
- `jjx_parade AFAAB` shows pace detail
- `/jjc-parade` works with smart defaults
- Old slash commands deleted
- JJD spec updated
- No --format or --pace flags remain

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrq_query.rs, jjrx_cli.rs, JJD-GallopsData.adoc, jjc-parade.md, jjc-parade-overview.md, jjc-parade-order.md, jjc-parade-detail.md, jjc-parade-full.md (8 files)
Steps:
1. In jjrq_query.rs: Delete jjrq_ParadeFormat enum. Change jjrq_ParadeArgs to have target String, full bool, remaining bool. Rewrite jjrq_run_parade to parse target as firemark or coronet, branch on type for output logic.
2. In jjrx_cli.rs: Delete zjjrx_ParadeFormatArg enum. Change zjjrx_ParadeArgs to positional target, optional --full flag, optional --remaining flag. Remove --format and --pace args. Update zjjrx_run_parade to build new args struct.
3. Run tt/vow-b.Build.sh to verify Rust compiles.
4. In JJD-GallopsData.adoc: Update jjdo_parade section - change Arguments to TARGET plus --full and --remaining, update Behavior to describe target type detection and output modes, delete ParadeFormat references.
5. Create .claude/commands/jjc-parade.md with frontmatter description Display heat or pace info, argument-hint target or full, and body with argument parsing logic for no-args, full, firemark, coronet cases.
6. Delete jjc-parade-overview.md, jjc-parade-order.md, jjc-parade-detail.md, jjc-parade-full.md.
Verify: tt/vow-b.Build.sh succeeds, jjx_parade AF outputs numbered list

**[260123-1249] rough**

Consolidate parade views: simplify Rust CLI, JJD spec, and slash commands.

## Rust CLI Changes

**Current:** `jjx_parade <FIREMARK> --format <mode> [--pace <coronet>] [--remaining]`

**New:** `jjx_parade <TARGET> [--full] [--remaining]`

TARGET detection:
- 2 chars / ₣XX → firemark (heat view)
- 5 chars / ₢XXXXX → coronet (pace view)

Behavior:
- `jjx_parade AF` → numbered list of paces in heat
- `jjx_parade AF --full` → paddock + all specs for heat
- `jjx_parade AFAAB` → full spec/direction for one pace

**Delete from jjrx_cli.rs:**
- `zjjrx_ParadeFormatArg` enum
- `--format` argument
- `--pace` argument

**Delete from jjrq_query.rs:**
- `jjrq_ParadeFormat` enum
- Format matching logic

**Replace with:**
- `jjrq_ParadeArgs { file, target: String, full: bool, remaining: bool }`
- Target parsing: `Firemark::jjrf_parse` vs `Coronet::jjrc_parse`
- Branch on target type for output

## JJD Spec Changes

**Update jjdo_parade:**

Arguments:
- `<TARGET>` — Firemark (heat) or Coronet (pace)
- `--full` — Show paddock and full specs (heat mode only)
- `--remaining` — Exclude complete/abandoned paces

Behavior by target type:
- Heat + no flags → numbered pace list
- Heat + --full → paddock + all pace specs
- Coronet → full tack detail for that pace

**Delete:**
- ParadeFormat type definition
- References to --format and --pace flags

## Slash Command Changes

**Create:** `.claude/commands/jjc-parade.md`

**Delete:**
- `jjc-parade-overview.md`
- `jjc-parade-order.md`
- `jjc-parade-detail.md`
- `jjc-parade-full.md`

**New slash command logic:**
- No args → list of racing heat
- `full` keyword → full view
- `full <firemark>` → full view of specific heat
- `<firemark>` → list of specific heat
- `<coronet>` → pace detail

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs (CLI args, dispatch)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (parade logic)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (jjdo_parade spec)
- .claude/commands/jjc-parade.md (new consolidated command)
- .claude/commands/jjc-parade-overview.md (delete)
- .claude/commands/jjc-parade-order.md (delete)
- .claude/commands/jjc-parade-detail.md (delete)
- .claude/commands/jjc-parade-full.md (delete)

## Acceptance

- `jjx_parade AF` shows numbered list
- `jjx_parade AF --full` shows paddock + specs
- `jjx_parade AFAAB` shows pace detail
- `/jjc-parade` works with smart defaults
- Old slash commands deleted
- JJD spec updated
- No --format or --pace flags remain

**[260123-1150] rough**

Consider consolidating the four parade slash commands into one.

## Current State

Four separate slash commands:
- /jjc-parade-overview — one line per pace
- /jjc-parade-order — numbered list
- /jjc-parade-detail — full tack for one pace
- /jjc-parade-full — paddock + all paces

This clutters the slash command namespace and adds cognitive load.

## Design Questions

1. **Single command with arguments?**
   `/jjc-parade [overview|order|detail|full] [pace]`
   
2. **Smart defaults?**
   - No args → overview (most common use)
   - With pace arg → detail for that pace
   - `--full` flag → full output

3. **Rust changes needed?**
   - jjx_parade already has --format flag
   - Slash command could just expose this better
   - Or: simplify Rust to have better defaults

4. **What about --remaining flag?**
   - Currently on jjx_parade
   - Should consolidated command default to remaining?

## Considerations

- Fewer slash commands = less clutter
- But: discoverability suffers if too much hidden behind flags
- Balance: one command with clear, memorable argument patterns

## Outcome Options

A. Keep separate commands (decide current state is fine)
B. Single /jjc-parade with positional mode argument
C. Single /jjc-parade with smart defaults based on args
D. Something else discovered during analysis

## Files (if implementing B or C)

- .claude/commands/jjc-parade.md (new, consolidated)
- .claude/commands/jjc-parade-overview.md (delete)
- .claude/commands/jjc-parade-order.md (delete)
- .claude/commands/jjc-parade-detail.md (delete)
- .claude/commands/jjc-parade-full.md (delete)
- Possibly: Tools/jjk/vov_veiled/src/jjrx_cli.rs (if Rust changes help)
- JJD-GallopsData.adoc (update jjdo_parade if behavior changes)

### parade-coronet-history (₢AFAAR) [complete]

**[260123-1308] complete**

Extend jjx_parade to accept coronet (pace identity) in addition to firemark (heat identity). When given a coronet, display the pace's full tack history.

## Behavior

**Detection logic:**
- 2 chars → firemark (existing behavior)
- 5 chars starting with valid firemark → coronet (new behavior)

**Coronet mode output:**
```
Pace: feature-name (₢XXXXX)
Heat: ₣XX

[0] complete (abc123f)
    Silks: feature-name
    
    Final implementation complete.

[1] bridled (def456a)
    Silks: feature-name
    Direction: Agent: sonnet...
    
    Ready for autonomous execution.

[2] rough (789abc0)
    Silks: initial-name
    
    Initial spec text here...
```

## Files

- Tools/jjk/vov_veiled/src/jjrq_query.rs — add coronet detection, tack history renderer
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — update jjdo_parade to document coronet mode

## Notes

- No new slash command needed — existing /jjc-parade-detail can document coronet usage
- Relates to ₢AFAAS (consolidate-parade-slash-commands) — may inform that consolidation

**[260123-1253] bridled**

Extend jjx_parade to accept coronet (pace identity) in addition to firemark (heat identity). When given a coronet, display the pace's full tack history.

## Behavior

**Detection logic:**
- 2 chars → firemark (existing behavior)
- 5 chars starting with valid firemark → coronet (new behavior)

**Coronet mode output:**
```
Pace: feature-name (₢XXXXX)
Heat: ₣XX

[0] complete (abc123f)
    Silks: feature-name
    
    Final implementation complete.

[1] bridled (def456a)
    Silks: feature-name
    Direction: Agent: sonnet...
    
    Ready for autonomous execution.

[2] rough (789abc0)
    Silks: initial-name
    
    Initial spec text here...
```

## Files

- Tools/jjk/vov_veiled/src/jjrq_query.rs — add coronet detection, tack history renderer
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — update jjdo_parade to document coronet mode

## Notes

- No new slash command needed — existing /jjc-parade-detail can document coronet usage
- Relates to ₢AFAAS (consolidate-parade-slash-commands) — may inform that consolidation

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrq_query.rs, JJD-GallopsData.adoc (2 files)
Steps:
1. In jjrq_query.rs jjrq_run_parade coronet branch: iterate all tacks in reverse order showing oldest first, output each with index, state, commit hash, silks, direction if present, and spec text
2. In JJD-GallopsData.adoc jjdo_parade section: document coronet mode output format showing tack history with example
Verify: tt/vow-b.Build.sh succeeds

**[260123-1219] rough**

Extend jjx_parade to accept coronet (pace identity) in addition to firemark (heat identity). When given a coronet, display the pace's full tack history.

## Behavior

**Detection logic:**
- 2 chars → firemark (existing behavior)
- 5 chars starting with valid firemark → coronet (new behavior)

**Coronet mode output:**
```
Pace: feature-name (₢XXXXX)
Heat: ₣XX

[0] complete (abc123f)
    Silks: feature-name
    
    Final implementation complete.

[1] bridled (def456a)
    Silks: feature-name
    Direction: Agent: sonnet...
    
    Ready for autonomous execution.

[2] rough (789abc0)
    Silks: initial-name
    
    Initial spec text here...
```

## Files

- Tools/jjk/vov_veiled/src/jjrq_query.rs — add coronet detection, tack history renderer
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — update jjdo_parade to document coronet mode

## Notes

- No new slash command needed — existing /jjc-parade-detail can document coronet usage
- Relates to ₢AFAAS (consolidate-parade-slash-commands) — may inform that consolidation

**[260123-1148] rough**

Add command to display full tack history for a pace, enabling postmortem analysis of plan evolution.

## Use Case

After completing a pace (or during debugging), view the full history of tacks:
- How the spec evolved
- State transitions (rough → bridled → complete)
- Direction changes
- Silks renames
- Timestamps and commit references

## Command: jjx_tacks

Syntax: `jjx_tacks <CORONET>`

Output: Formatted display of all tacks, newest first (matching JSON order).

```
Pace: feature-name (₢XXXXX)
Heat: ₣XX

[0] 260123-1130 complete
    Silks: feature-name
    Commit: abc123f
    
    Final implementation complete.

[1] 260123-1000 bridled
    Silks: feature-name  
    Commit: def456a
    Direction: Agent: sonnet...
    
    Ready for autonomous execution.

[2] 260122-1500 rough
    Silks: initial-name
    Commit: 789abc0
    
    Initial spec text here...
```

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs (add Tacks command, TacksArgs struct)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (document jjdo_tacks operation)

## JJD Updates Required

Add [[jjdo_tacks]] operation documentation:
- Arguments (coronet positional)
- Output format specification  
- Behavior steps (load gallops, find pace, iterate tacks)
- Exit status

### nominate-defaults-stabled (₢AFAAJ) [complete]

**[260123-1310] complete**

Change jjx_nominate default: new heats start as 'stabled' not 'racing'. No heat should auto-start running — user must explicitly furlough to racing when ready to execute. Update default in Rust impl and any slash command docs that mention initial state.

**[260123-1042] bridled**

Change jjx_nominate default: new heats start as 'stabled' not 'racing'. No heat should auto-start running — user must explicitly furlough to racing when ready to execute. Update default in Rust impl and any slash command docs that mention initial state.

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjro_ops.rs (1 file)
Steps:
1. In jjrg_nominate function, change line with 'status: jjrg_HeatStatus::Racing' to 'status: jjrg_HeatStatus::Stabled'
Verify: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk

**[260120-1820] rough**

Change jjx_nominate default: new heats start as 'stabled' not 'racing'. No heat should auto-start running — user must explicitly furlough to racing when ready to execute. Update default in Rust impl and any slash command docs that mention initial state.

### nominate-created-from-env (₢AFAAL) [complete]

**[260123-1935] complete**

Remove --created flag from jjx_nominate; derive date automatically.

**Priority:** BUD_NOW_STAMP env var (if set) → system clock fallback.

**Rationale:** When running via BUK infrastructure, BUD_NOW_STAMP provides session-consistent timestamp. Outside BUK, system clock works fine.

**Implementation:**
- Add `jjrc_timestamp_from_env()` to jjrc_core.rs:
  - Check BUD_NOW_STAMP env var (format: YYYYMMDD-HHMMSS-PID-RANDOM)
  - If set, extract YYYYMMDD and convert to YYMMDD (drop century)
  - If unset, call existing jjrc_timestamp_date() for system clock
- Remove `created` field from zjjrx_NominateArgs in jjrx_cli.rs
- In zjjrx_run_nominate, call jjrc_timestamp_from_env() instead of using args.created
- In jjc-heat-nominate.md, remove --created flag and date command entirely

**Files:**
- Tools/jjk/vov_veiled/src/jjrc_core.rs (add jjrc_timestamp_from_env)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (remove created from NominateArgs, use new fn)
- .claude/commands/jjc-heat-nominate.md (simplify invocation)

**Constraint:** Schema-stable — heat created field format unchanged (YYMMDD), just sourced differently.

**[260123-1929] bridled**

Remove --created flag from jjx_nominate; derive date automatically.

**Priority:** BUD_NOW_STAMP env var (if set) → system clock fallback.

**Rationale:** When running via BUK infrastructure, BUD_NOW_STAMP provides session-consistent timestamp. Outside BUK, system clock works fine.

**Implementation:**
- Add `jjrc_timestamp_from_env()` to jjrc_core.rs:
  - Check BUD_NOW_STAMP env var (format: YYYYMMDD-HHMMSS-PID-RANDOM)
  - If set, extract YYYYMMDD and convert to YYMMDD (drop century)
  - If unset, call existing jjrc_timestamp_date() for system clock
- Remove `created` field from zjjrx_NominateArgs in jjrx_cli.rs
- In zjjrx_run_nominate, call jjrc_timestamp_from_env() instead of using args.created
- In jjc-heat-nominate.md, remove --created flag and date command entirely

**Files:**
- Tools/jjk/vov_veiled/src/jjrc_core.rs (add jjrc_timestamp_from_env)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (remove created from NominateArgs, use new fn)
- .claude/commands/jjc-heat-nominate.md (simplify invocation)

**Constraint:** Schema-stable — heat created field format unchanged (YYMMDD), just sourced differently.

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrc_core.rs, jjrx_cli.rs, jjc-heat-nominate.md (3 files)
Steps:
1. Add jjrc_timestamp_from_env() to jjrc_core.rs that checks BUD_NOW_STAMP env var, extracts YYYYMMDD, converts to YYMMDD; falls back to jjrc_timestamp_date()
2. Remove created field from zjjrx_NominateArgs in jjrx_cli.rs
3. In zjjrx_run_nominate, call jjrc_timestamp_from_env() instead of using args.created
4. In jjc-heat-nominate.md, remove Step 2 (date command), remove --created from the jjx_nominate invocation
Verify: tt/vow-b.Build.sh

**[260123-1725] rough**

Remove --created flag from jjx_nominate; derive date automatically.

**Priority:** BUD_NOW_STAMP env var (if set) → system clock fallback.

**Rationale:** When running via BUK infrastructure, BUD_NOW_STAMP provides session-consistent timestamp. Outside BUK, system clock works fine.

**Implementation:**
- Add `jjrc_timestamp_from_env()` to jjrc_core.rs:
  - Check BUD_NOW_STAMP env var (format: YYYYMMDD-HHMMSS-PID-RANDOM)
  - If set, extract YYYYMMDD and convert to YYMMDD (drop century)
  - If unset, call existing jjrc_timestamp_date() for system clock
- Remove `created` field from zjjrx_NominateArgs in jjrx_cli.rs
- In zjjrx_run_nominate, call jjrc_timestamp_from_env() instead of using args.created
- In jjc-heat-nominate.md, remove --created flag and date command entirely

**Files:**
- Tools/jjk/vov_veiled/src/jjrc_core.rs (add jjrc_timestamp_from_env)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (remove created from NominateArgs, use new fn)
- .claude/commands/jjc-heat-nominate.md (simplify invocation)

**Constraint:** Schema-stable — heat created field format unchanged (YYMMDD), just sourced differently.

**[260123-1043] bridled**

Make --created optional in jjx_nominate by defaulting to system clock.

**Current:** Slash command calls `date +%y%m%d` then passes `--created "YYMMDD"`.

**After:** If --created omitted, Rust uses system clock to generate YYMMDD format.

**Files:**
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (make created optional in NominateArgs)
- Tools/jjk/vov_veiled/src/jjx_nominate.rs (default to chrono::Local::now())
- .claude/commands/jjc-heat-nominate.md (remove date command, simplify invocation)

**Constraint:** Schema-stable — created field format unchanged, just sourced differently.

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrx_cli.rs, jjc-heat-nominate.md (2 files)
Steps:
1. In zjjrx_NominateArgs, change 'created: String' to 'created: Option<String>'
2. In zjjrx_run_nominate, add 'use crate::jjrc_core::jjrc_timestamp_date;' import
3. In zjjrx_run_nominate, change 'created: args.created' to 'created: args.created.unwrap_or_else(jjrc_timestamp_date)'
4. In .claude/commands/jjc-heat-nominate.md, remove the date command and make --created optional in the invocation example
Verify: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk

**[260122-1622] rough**

Make --created optional in jjx_nominate by defaulting to system clock.

**Current:** Slash command calls `date +%y%m%d` then passes `--created "YYMMDD"`.

**After:** If --created omitted, Rust uses system clock to generate YYMMDD format.

**Files:**
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (make created optional in NominateArgs)
- Tools/jjk/vov_veiled/src/jjx_nominate.rs (default to chrono::Local::now())
- .claude/commands/jjc-heat-nominate.md (remove date command, simplify invocation)

**Constraint:** Schema-stable — created field format unchanged, just sourced differently.

### investigate-hallmark-format-discrepancy (₢AFAAO) [complete]

**[260123-1942] complete**

Investigate why heat-only commits show hallmark as `1010` while pace commits show `1010-xxxxxxx` in Kit Forge.

## Observed

- Pace commit: `jjb:1010-c75754c9:₢AFAAB:n: ...`
- Heat-only commit: `jjb:1010:₣AF:n: ...`

Both should use Kit Forge format (`NNNN-xxxxxxx`) since no `.vvk/vvbf_brand.json` exists.

## Investigation Steps

1. Add debug logging to zjjrn_get_hallmark() in jjrn_notch.rs to trace which branch executes
2. Check if fs::read_to_string succeeds on a non-existent file (should fail, but verify)
3. Verify the brand file path is correct (`.vvk/vvbf_brand.json` vs `./vvk/...` vs absolute path)
4. Test both notch and heat-level commit paths to see hallmark output
5. Check if any symlinks or mount points could cause the brand file to appear/disappear

## Expected Outcome

- Identify root cause of format inconsistency
- Fix if bug found, or document if environmental quirk

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs (zjjrn_get_hallmark function)

**[260123-1135] rough**

Investigate why heat-only commits show hallmark as `1010` while pace commits show `1010-xxxxxxx` in Kit Forge.

## Observed

- Pace commit: `jjb:1010-c75754c9:₢AFAAB:n: ...`
- Heat-only commit: `jjb:1010:₣AF:n: ...`

Both should use Kit Forge format (`NNNN-xxxxxxx`) since no `.vvk/vvbf_brand.json` exists.

## Investigation Steps

1. Add debug logging to zjjrn_get_hallmark() in jjrn_notch.rs to trace which branch executes
2. Check if fs::read_to_string succeeds on a non-existent file (should fail, but verify)
3. Verify the brand file path is correct (`.vvk/vvbf_brand.json` vs `./vvk/...` vs absolute path)
4. Test both notch and heat-level commit paths to see hallmark output
5. Check if any symlinks or mount points could cause the brand file to appear/disappear

## Expected Outcome

- Identify root cause of format inconsistency
- Fix if bug found, or document if environmental quirk

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs (zjjrn_get_hallmark function)

### heat-json-order-auto-groom-mount-choice (₢AFAAb) [complete]

**[260124-0734] complete**

Switch heats from BTreeMap to IndexMap for insertion-order preservation. Nominate unchanged (appends at end). Furlough-to-racing removes and re-inserts heat at front of heats object. Muster iterates in JSON order. Groom/saddle slash commands pick first racing heat when no firemark provided. Result: furlough controls which heat is active; new heats queue behind existing work.

**[260123-1947] bridled**

Switch heats from BTreeMap to IndexMap for insertion-order preservation. Nominate unchanged (appends at end). Furlough-to-racing removes and re-inserts heat at front of heats object. Muster iterates in JSON order. Groom/saddle slash commands pick first racing heat when no firemark provided. Result: furlough controls which heat is active; new heats queue behind existing work.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Cargo.toml, jjrt_types.rs, jjro_ops.rs, jjtg_gallops.rs, jjrx_cli.rs, jjtq_query.rs, jjc-heat-groom.md, jjc-heat-mount.md (8 files)
Steps:
1. Add indexmap = { version = 1, features = serde } to Tools/jjk/vov_veiled/Cargo.toml
2. In jjrt_types.rs: add use indexmap::IndexMap, change heats: BTreeMap to heats: IndexMap
3. In jjro_ops.rs: add IndexMap import, modify jjrg_furlough to remove/shift_insert at index 0 when racing
4. In jjtg_gallops.rs: update test helpers to use IndexMap
5. Check jjrx_cli.rs and jjtq_query.rs for any needed import changes
6. Update jjc-heat-groom.md: when 2+ heats and no arg, pick first racing heat
7. Update jjc-heat-mount.md: when 2+ racing heats and no arg, pick first one
Verify: tt/vow-b.Build.sh

**[260123-1719] rough**

Switch heats from BTreeMap to IndexMap for insertion-order preservation. Nominate unchanged (appends at end). Furlough-to-racing removes and re-inserts heat at front of heats object. Muster iterates in JSON order. Groom/saddle slash commands pick first racing heat when no firemark provided. Result: furlough controls which heat is active; new heats queue behind existing work.

**[260123-1717] rough**

Switch heats from BTreeMap to IndexMap for insertion-order preservation. Nominate inserts new heats at front. Furlough-to-racing removes and re-inserts at front. Muster iterates in JSON order. Groom/saddle slash commands pick first racing heat when no firemark provided. Result: active heat is always first, eliminating round-trips for heat selection.

### slash-cmd-heredoc-stdin (₢AFAAj) [complete]

**[260124-0957] complete**

Update slash commands to use heredoc pattern with context-appropriate delimiters per CLAUDE.md guidance.

## CLAUDE.md Guidance (lines 167-173)

> When generating heredocs for stdin content, the delimiter must not appear alone on any line within the content.
> - **Check content first**: If content includes `EOF`, use a different delimiter
> - **Safe alternatives**: `SPEC`, `CONTENT`, `DOC`, `PACESPEC`, `SLASHCMD`

## Files to Update

| File | Lines | Change | Delimiter |
|------|-------|--------|-----------|
| jjc-pace-slate.md | 58 | echo to cat heredoc | `PACESPEC` |
| jjc-pace-reslate.md | 92, 97 | echo to cat heredoc | `PACESPEC` |
| jjc-heat-restring.md | 113 | Add explicit heredoc pattern | `PACESPEC` |

## Pattern

Replace:
```bash
echo "<TEXT>" | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
```

With:
```bash
cat <<'PACESPEC' | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
<content>
PACESPEC
```

## Rationale

1. Heredoc with quoted delimiter is shell-safe for arbitrary content containing $variables, `backticks`, <angles>, pipes, etc.
2. `EOF` is too generic — appears in code examples and documentation
3. `PACESPEC` is project-specific and unlikely to appear in pace specifications

## Bridleable

Yes - mechanical text replacement, 3 files, clear pattern, single approach.

**[260124-0825] bridled**

Update slash commands to use heredoc pattern with context-appropriate delimiters per CLAUDE.md guidance.

## CLAUDE.md Guidance (lines 167-173)

> When generating heredocs for stdin content, the delimiter must not appear alone on any line within the content.
> - **Check content first**: If content includes `EOF`, use a different delimiter
> - **Safe alternatives**: `SPEC`, `CONTENT`, `DOC`, `PACESPEC`, `SLASHCMD`

## Files to Update

| File | Lines | Change | Delimiter |
|------|-------|--------|-----------|
| jjc-pace-slate.md | 58 | echo to cat heredoc | `PACESPEC` |
| jjc-pace-reslate.md | 92, 97 | echo to cat heredoc | `PACESPEC` |
| jjc-heat-restring.md | 113 | Add explicit heredoc pattern | `PACESPEC` |

## Pattern

Replace:
```bash
echo "<TEXT>" | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
```

With:
```bash
cat <<'PACESPEC' | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
<content>
PACESPEC
```

## Rationale

1. Heredoc with quoted delimiter is shell-safe for arbitrary content containing $variables, `backticks`, <angles>, pipes, etc.
2. `EOF` is too generic — appears in code examples and documentation
3. `PACESPEC` is project-specific and unlikely to appear in pace specifications

## Bridleable

Yes - mechanical text replacement, 3 files, clear pattern, single approach.

*Direction:* Agent: haiku | Files: jjc-pace-slate.md, jjc-pace-reslate.md, jjc-heat-restring.md (3 files) | Steps: 1. In jjc-pace-slate.md line 58 replace echo with cat heredoc using PACESPEC delimiter 2. In jjc-pace-reslate.md lines 92 and 97 replace echo with cat heredoc 3. In jjc-heat-restring.md add heredoc pattern | Verify: grep PACESPEC in all three files

**[260124-0812] rough**

Update slash commands to use heredoc pattern with context-appropriate delimiters per CLAUDE.md guidance.

## CLAUDE.md Guidance (lines 167-173)

> When generating heredocs for stdin content, the delimiter must not appear alone on any line within the content.
> - **Check content first**: If content includes `EOF`, use a different delimiter
> - **Safe alternatives**: `SPEC`, `CONTENT`, `DOC`, `PACESPEC`, `SLASHCMD`

## Files to Update

| File | Lines | Change | Delimiter |
|------|-------|--------|-----------|
| jjc-pace-slate.md | 58 | echo to cat heredoc | `PACESPEC` |
| jjc-pace-reslate.md | 92, 97 | echo to cat heredoc | `PACESPEC` |
| jjc-heat-restring.md | 113 | Add explicit heredoc pattern | `PACESPEC` |

## Pattern

Replace:
```bash
echo "<TEXT>" | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
```

With:
```bash
cat <<'PACESPEC' | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
<content>
PACESPEC
```

## Rationale

1. Heredoc with quoted delimiter is shell-safe for arbitrary content containing $variables, `backticks`, <angles>, pipes, etc.
2. `EOF` is too generic — appears in code examples and documentation
3. `PACESPEC` is project-specific and unlikely to appear in pace specifications

## Bridleable

Yes - mechanical text replacement, 3 files, clear pattern, single approach.

**[260124-0749] bridled**

Update slash commands to use heredoc pattern for stdin content.

## Files to Update

| File | Lines | Change |
|------|-------|--------|
| jjc-pace-slate.md | 58 | echo to cat heredoc |
| jjc-pace-reslate.md | 92, 97 | echo to cat heredoc |
| jjc-heat-restring.md | 113 | Add explicit heredoc pattern |

## Pattern

Replace:
```bash
echo "<TEXT>" | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
```

With:
```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
<content>
EOF
```

## Rationale

Heredoc with quoted delimiter is shell-safe for arbitrary content containing $variables, `backticks`, <angles>, pipes, etc. The echo pattern breaks on these characters.

**Delimiter constraint:** The delimiter (e.g., EOF) must not appear alone on a line within the content.

## Bridleable

Yes - mechanical text replacement, 3 files, clear pattern.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjc-pace-slate.md, jjc-pace-reslate.md, jjc-heat-restring.md (3 files)
Steps:
1. In jjc-pace-slate.md Step 3: Replace echo pattern with cat heredoc using quoted delimiter
2. In jjc-pace-reslate.md Step 4: Replace both echo patterns with cat heredoc using quoted delimiter
3. In jjc-heat-restring.md Step 6d: Add explicit cat heredoc pattern where jjx_tally stdin is mentioned
Verify: Manual review

**[260124-0743] rough**

Update slash commands to use heredoc pattern for stdin content.

## Files to Update

| File | Lines | Change |
|------|-------|--------|
| jjc-pace-slate.md | 58 | echo to cat heredoc |
| jjc-pace-reslate.md | 92, 97 | echo to cat heredoc |
| jjc-heat-restring.md | 113 | Add explicit heredoc pattern |

## Pattern

Replace:
```bash
echo "<TEXT>" | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
```

With:
```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_<cmd> ...
<content>
EOF
```

## Rationale

Heredoc with quoted delimiter is shell-safe for arbitrary content containing $variables, `backticks`, <angles>, pipes, etc. The echo pattern breaks on these characters.

**Delimiter constraint:** The delimiter (e.g., EOF) must not appear alone on a line within the content.

## Bridleable

Yes - mechanical text replacement, 3 files, clear pattern.

### muster-column-format (₢AFAAe) [abandoned]

**[260124-0947] abandoned**

Duplicate of completed parade-column-format work (₢AFAAf). Muster already outputs column-aligned plain text.

**[260124-0932] rough**

ALREADY DONE - jjx_muster now outputs column-aligned plain text.

This pace was completed as part of ₢AFAAf (parade-column-format) work.

**Current output:**
```
Fire   Silks                                Status   Done  Total
----------------------------------------------------------------
₣AF    jjk-post-alpha-polish                racing     22     42
₣AH    jjk-commission-haiku-pilot           racing      0      1
```

**Remaining work:** None - mark as complete or abandon as duplicate.

**[260124-0724] bridled**

Modify jjx_muster output to generate a proper box-drawing table for racing heats. Use Unicode box-drawing characters (┌ ─ ┬ ┐ │ ├ ┼ ┤ └ ┴ ┘) to create a formatted table with columns: Firemark, Silks, Status, Progress. Racing heats get the table; stabled heats get a single summary line. Include the 'Which heat would you like to groom?' prompt at the end.

*Direction:* Cardinality: 2 parallel + sequential build
Files: jjrq_query.rs, JJSCMU-muster.adoc (2 files)
Steps:
1. Agent A (sonnet): In jjrq_query.rs, modify jjrq_run_muster to output box-drawing table for racing heats, summary line for stabled heats, and groom prompt at end
2. Agent B (sonnet): In JJSCMU-muster.adoc, update stdout section to document new box-table format for racing heats, summary format for stabled heats, and trailing prompt
3. Sequential: tt/vow-b.Build.sh and tt/vow-t.Test.sh
Box chars: corner-TL corner-TR corner-BL corner-BR, horizontal, vertical, T-junctions, cross
Columns: Firemark, Silks, Status, Progress (completed/defined)

**[260124-0720] bridled**

Modify jjx_muster output to generate a proper box-drawing table for racing heats. Use Unicode box-drawing characters (┌ ─ ┬ ┐ │ ├ ┼ ┤ └ ┴ ┘) to create a formatted table with columns: Firemark, Silks, Status, Progress. Racing heats get the table; stabled heats get a single summary line. Include the 'Which heat would you like to groom?' prompt at the end.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrq_query.rs (1 file)
Steps:
1. Add helper function for box-drawing table generation
2. Modify jjrq_run_muster to collect racing vs stabled heats
3. Render racing heats as box table with dynamic column widths
4. Render stabled heats as single summary line
5. Append groom prompt
Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-0718] rough**

Modify jjx_muster output to generate a proper box-drawing table for racing heats. Use Unicode box-drawing characters (┌ ─ ┬ ┐ │ ├ ┼ ┤ └ ┴ ┘) to create a formatted table with columns: Firemark, Silks, Status, Progress. Racing heats get the table; stabled heats get a single summary line. Include the 'Which heat would you like to groom?' prompt at the end.

### implement-jjx-curry (₢AFAAd) [complete]

**[260124-1025] complete**

Implement `jjx_curry` primitive for paddock maintenance with getter/setter semantics.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution. Also provides simple paddock viewing without saddle JSON overhead.

## CLI Signature

```
jjx_curry <firemark> [--<verb>] [--note "reason"]
```

## Behavior

| Stdin | Verb | Result |
|-------|------|--------|
| empty | ignored | Display paddock content (getter) |
| content | required | Update paddock + chalk entry (setter) |

**Getter mode:** No stdin → print paddock to stdout, exit 0. Verb is ignored if provided.

**Setter mode:** Stdin present → verb is required. Update paddock file, append chalk entry, exit 0.

## Verbs (setter mode only)

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Slash Command Pattern

Getter (display paddock):
```bash
./tt/vvw-r.RunVVX.sh jjx_curry AF
```

Setter (update paddock):
```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_curry AF --refine --note "added context"
# Paddock: heat-name

New paddock content here.
EOF
```

## Chalk Entry (setter only)

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Check if stdin is empty (is_atty or read with timeout)
- If empty stdin: read paddock file, print to stdout, exit
- If stdin has content: require verb flag, write paddock, chalk entry
- Error if setter mode with no verb

## Files

jjrx_cli.rs, jjro_ops.rs

**[260124-0843] bridled**

Implement `jjx_curry` primitive for paddock maintenance with getter/setter semantics.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution. Also provides simple paddock viewing without saddle JSON overhead.

## CLI Signature

```
jjx_curry <firemark> [--<verb>] [--note "reason"]
```

## Behavior

| Stdin | Verb | Result |
|-------|------|--------|
| empty | ignored | Display paddock content (getter) |
| content | required | Update paddock + chalk entry (setter) |

**Getter mode:** No stdin → print paddock to stdout, exit 0. Verb is ignored if provided.

**Setter mode:** Stdin present → verb is required. Update paddock file, append chalk entry, exit 0.

## Verbs (setter mode only)

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Slash Command Pattern

Getter (display paddock):
```bash
./tt/vvw-r.RunVVX.sh jjx_curry AF
```

Setter (update paddock):
```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_curry AF --refine --note "added context"
# Paddock: heat-name

New paddock content here.
EOF
```

## Chalk Entry (setter only)

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Check if stdin is empty (is_atty or read with timeout)
- If empty stdin: read paddock file, print to stdout, exit
- If stdin has content: require verb flag, write paddock, chalk entry
- Error if setter mode with no verb

## Files

jjrx_cli.rs, jjro_ops.rs

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrx_cli.rs, jjro_ops.rs (2 files) | Steps: 1. Add CurryArgs with optional verb enum and optional note to jjrx_cli.rs 2. Add jjrg_curry in jjro_ops.rs that checks stdin via atty crate 3. If stdin empty print paddock content to stdout and return 4. If stdin present require verb flag else error and write paddock then call chalk | Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260124-0843] rough**

Implement `jjx_curry` primitive for paddock maintenance with getter/setter semantics.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution. Also provides simple paddock viewing without saddle JSON overhead.

## CLI Signature

```
jjx_curry <firemark> [--<verb>] [--note "reason"]
```

## Behavior

| Stdin | Verb | Result |
|-------|------|--------|
| empty | ignored | Display paddock content (getter) |
| content | required | Update paddock + chalk entry (setter) |

**Getter mode:** No stdin → print paddock to stdout, exit 0. Verb is ignored if provided.

**Setter mode:** Stdin present → verb is required. Update paddock file, append chalk entry, exit 0.

## Verbs (setter mode only)

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Slash Command Pattern

Getter (display paddock):
```bash
./tt/vvw-r.RunVVX.sh jjx_curry AF
```

Setter (update paddock):
```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_curry AF --refine --note "added context"
# Paddock: heat-name

New paddock content here.
EOF
```

## Chalk Entry (setter only)

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Check if stdin is empty (is_atty or read with timeout)
- If empty stdin: read paddock file, print to stdout, exit
- If stdin has content: require verb flag, write paddock, chalk entry
- Error if setter mode with no verb

## Files

jjrx_cli.rs, jjro_ops.rs

**[260124-0739] bridled**

Implement `jjx_curry` primitive for paddock maintenance with chalk tracking.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution.

## CLI Signature

```
jjx_curry <firemark> --<verb> [--note "reason"]
```

Content via stdin (required, non-empty).

## Verbs

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Slash Command Pattern

Use heredoc with quoted delimiter for shell-safe content:

```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_curry <FIREMARK> --<verb> [--note "reason"]
<new paddock content - any characters allowed>
EOF
```

## Chalk Entry

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Read new paddock content from stdin
- Error if stdin is empty (paddocks are never empty)
- Overwrite paddock file
- Append chalk entry to heat steeplechase
- Exit 0 on success, non-zero with message on failure

## Bridleable

Yes. Mechanical implementation following existing jjx_* patterns.
- Pattern: jjx_chalk for steeplechase, jjx_tally for state updates
- Files: jjrx_cli.rs, jjro_ops.rs
- Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrx_cli.rs, jjro_ops.rs (2 files)
Steps:
1. Add CurryArgs struct and jjx_curry subcommand to jjrx_cli.rs following jjx_chalk pattern
2. Add jjrg_curry function to jjro_ops.rs: read stdin, validate non-empty, write paddock file, call chalk
3. Wire up CLI to call jjrg_curry with firemark, verb, and optional note
4. Verb enum: Refine, Level, Muck - format chalk description accordingly
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260124-0738] rough**

Implement `jjx_curry` primitive for paddock maintenance with chalk tracking.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution.

## CLI Signature

```
jjx_curry <firemark> --<verb> [--note "reason"]
```

Content via stdin (required, non-empty).

## Verbs

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Slash Command Pattern

Use heredoc with quoted delimiter for shell-safe content:

```bash
cat <<'EOF' | ./tt/vvw-r.RunVVX.sh jjx_curry <FIREMARK> --<verb> [--note "reason"]
<new paddock content - any characters allowed>
EOF
```

## Chalk Entry

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Read new paddock content from stdin
- Error if stdin is empty (paddocks are never empty)
- Overwrite paddock file
- Append chalk entry to heat steeplechase
- Exit 0 on success, non-zero with message on failure

## Bridleable

Yes. Mechanical implementation following existing jjx_* patterns.
- Pattern: jjx_chalk for steeplechase, jjx_tally for state updates
- Files: jjrx_cli.rs, jjro_ops.rs
- Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260123-2005] rough**

Implement `jjx_curry` primitive for paddock maintenance with chalk tracking.

## Purpose

Enable chalked paddock modifications so retrospectives can see context evolution.

## CLI Signature

```
jjx_curry <firemark> --<verb> [--note "reason"]
```

Content via stdin.

## Verbs

| Verb | Use case |
|------|----------|
| `--refine` | Manual context update during work |
| `--level` | Braid absorbing context from another heat |
| `--muck` | Intentional reduction, removing stale material |

## Chalk Entry

Format: `paddock curried (<verb>)[: <note>]`
Example: `paddock curried (level): from ₣AB`

## Implementation Notes

- Read new paddock content from stdin
- Overwrite paddock file
- Append chalk entry to heat steeplechase
- Exit 0 on success, non-zero with message on failure

### fix-muster-status-filter (₢AFAAN) [abandoned]

**[260123-1040] abandoned**

The jjc-heat-mount.md slash command on line 21 uses `jjx_muster --status racing` but jjx_muster does not accept a --status argument.

Fix options:
1. Add --status filtering to jjx_muster in Rust (preferred - cleaner interface)
2. Update slash command to filter TSV output after the fact

The Rust implementation should add an optional --status flag that filters heats by status (racing/stabled).

**[260123-1039] rough**

The jjc-heat-mount.md slash command on line 21 uses `jjx_muster --status racing` but jjx_muster does not accept a --status argument.

Fix options:
1. Add --status filtering to jjx_muster in Rust (preferred - cleaner interface)
2. Update slash command to filter TSV output after the fact

The Rust implementation should add an optional --status flag that filters heats by status (racing/stabled).

### common-mount-recommendation (₢AFAAo) [complete]

**[260124-1039] complete**

Extract mount recommendation to shared Rust helper and emit from both wrap and tally.

**Goal:** Consistent post-operation advice from Rust code, removing need for slash commands to render this.

**Output format:**
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
Reminder: Use /jjc-pace-notch for commits
```

**Changes:**

1. **jjrx_cli.rs**: Add helper function `zjjrx_emit_recommendations(firemark: &Firemark)` that prints both lines to stderr

2. **zjjrx_run_wrap**: Replace inline eprintln with call to the new helper

3. **zjjrx_run_tally**: When state is bridled, call the helper after successful persist

4. **jjc-pace-bridle.md**: Remove Step 6 mount advice - Rust handles it now

5. **jjc-pace-wrap.md**: Remove "pass through Recommended lines" guidance - automatic now

**Files:** jjrx_cli.rs, .claude/commands/jjc-pace-bridle.md, .claude/commands/jjc-pace-wrap.md

**[260124-1004] bridled**

Extract mount recommendation to shared Rust helper and emit from both wrap and tally.

**Goal:** Consistent post-operation advice from Rust code, removing need for slash commands to render this.

**Output format:**
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
Reminder: Use /jjc-pace-notch for commits
```

**Changes:**

1. **jjrx_cli.rs**: Add helper function `zjjrx_emit_recommendations(firemark: &Firemark)` that prints both lines to stderr

2. **zjjrx_run_wrap**: Replace inline eprintln with call to the new helper

3. **zjjrx_run_tally**: When state is bridled, call the helper after successful persist

4. **jjc-pace-bridle.md**: Remove Step 6 mount advice - Rust handles it now

5. **jjc-pace-wrap.md**: Remove "pass through Recommended lines" guidance - automatic now

**Files:** jjrx_cli.rs, .claude/commands/jjc-pace-bridle.md, .claude/commands/jjc-pace-wrap.md

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrx_cli.rs, jjc-pace-bridle.md, jjc-pace-wrap.md (3 files) | Steps: 1. Add zjjrx_emit_recommendations helper function that prints mount recommendation and notch reminder to stderr 2. Replace inline eprintln in zjjrx_run_wrap with helper call 3. Add helper call in zjjrx_run_tally after successful bridled persist 4. Remove Step 6 mount advice from jjc-pace-bridle.md 5. Remove pass-through guidance from jjc-pace-wrap.md | Verify: tt/vow-b.Build.sh

**[260124-0941] rough**

Extract mount recommendation to shared Rust helper and emit from both wrap and tally.

**Goal:** Consistent post-operation advice from Rust code, removing need for slash commands to render this.

**Output format:**
```
Recommended: /clear then /jjc-heat-mount <FIREMARK>
Reminder: Use /jjc-pace-notch for commits
```

**Changes:**

1. **jjrx_cli.rs**: Add helper function `zjjrx_emit_recommendations(firemark: &Firemark)` that prints both lines to stderr

2. **zjjrx_run_wrap**: Replace inline eprintln with call to the new helper

3. **zjjrx_run_tally**: When state is bridled, call the helper after successful persist

4. **jjc-pace-bridle.md**: Remove Step 6 mount advice - Rust handles it now

5. **jjc-pace-wrap.md**: Remove "pass through Recommended lines" guidance - automatic now

**Files:** jjrx_cli.rs, .claude/commands/jjc-pace-bridle.md, .claude/commands/jjc-pace-wrap.md

**[260124-0939] bridled**

Extract mount recommendation to shared Rust helper and emit from both wrap and tally.

**Goal:** Consistent "Recommended: /clear then /jjc-heat-mount" advice from Rust code, removing need for slash commands to render this.

**Changes:**

1. **jjrx_cli.rs**: Add helper function `zjjrx_emit_mount_recommendation(firemark: &Firemark)` that prints the recommendation to stderr

2. **zjjrx_run_wrap**: Replace inline eprintln with call to the new helper

3. **zjjrx_run_tally**: When state is bridled, call the helper after successful persist (in the Ok branch after "tally succeeded")

4. **jjc-pace-bridle.md**: Remove Step 6 mount advice (lines about "If this pace is next in order"). The Rust output handles it now.

5. **jjc-pace-wrap.md**: Remove "pass through Recommended lines" guidance since it's automatic now.

**Files:** jjrx_cli.rs, .claude/commands/jjc-pace-bridle.md, .claude/commands/jjc-pace-wrap.md

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrx_cli.rs, jjc-pace-bridle.md, jjc-pace-wrap.md (3 files) | Steps: 1. Add zjjrx_emit_mount_recommendation helper function in jjrx_cli.rs 2. Replace inline eprintln in zjjrx_run_wrap with helper call 3. Add helper call in zjjrx_run_tally after successful bridled persist 4. Remove Step 6 mount advice from jjc-pace-bridle.md 5. Remove pass-through guidance from jjc-pace-wrap.md | Verify: tt/vow-b.Build.sh

**[260124-0838] rough**

Extract mount recommendation to shared Rust helper and emit from both wrap and tally.

**Goal:** Consistent "Recommended: /clear then /jjc-heat-mount" advice from Rust code, removing need for slash commands to render this.

**Changes:**

1. **jjrx_cli.rs**: Add helper function `zjjrx_emit_mount_recommendation(firemark: &Firemark)` that prints the recommendation to stderr

2. **zjjrx_run_wrap**: Replace inline eprintln with call to the new helper

3. **zjjrx_run_tally**: When state is bridled, call the helper after successful persist (in the Ok branch after "tally succeeded")

4. **jjc-pace-bridle.md**: Remove Step 6 mount advice (lines about "If this pace is next in order"). The Rust output handles it now.

5. **jjc-pace-wrap.md**: Remove "pass through Recommended lines" guidance since it's automatic now.

**Files:** jjrx_cli.rs, .claude/commands/jjc-pace-bridle.md, .claude/commands/jjc-pace-wrap.md

### vob-release-conformance (₢AFAAC) [complete]

**[260124-1045] complete**

Fix vob_release() conformance with VOS spec.

**Prerequisite:** ₢AFAAm (vos-commit-message-format) must complete first.

**Changes:**

1. **Parcel output location:**
   - Create `.jjk/parcels/` directory if not exists
   - Output parcel to `.jjk/parcels/{kit}-{hallmark}.tar.gz`
   - Add `.jjk/parcels/` to `.gitignore`

2. **Registry commit:**
   - After allocating new hallmark in registry, commit using `vvx_commit`
   - Message format: `vvb:HALLMARK::A: allocate hallmark XXXX for {kit}`
   - Uses the newly allocated hallmark in the message

**Files:** vob_release.rs (or equivalent), .gitignore

**[260124-1041] bridled**

Fix vob_release() conformance with VOS spec.

**Prerequisite:** ₢AFAAm (vos-commit-message-format) must complete first.

**Changes:**

1. **Parcel output location:**
   - Create `.jjk/parcels/` directory if not exists
   - Output parcel to `.jjk/parcels/{kit}-{hallmark}.tar.gz`
   - Add `.jjk/parcels/` to `.gitignore`

2. **Registry commit:**
   - After allocating new hallmark in registry, commit using `vvx_commit`
   - Message format: `vvb:HALLMARK::A: allocate hallmark XXXX for {kit}`
   - Uses the newly allocated hallmark in the message

**Files:** vob_release.rs (or equivalent), .gitignore

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vok/vob_build.sh, .gitignore (2 files) | Steps: 1. In vob_release change parcel output from project root to .jjk/parcels/ directory, create dir if needed 2. Add .jjk/parcels/ to .gitignore 3. After release_brand succeeds, parse stderr for allocated new hallmark to detect is_new 4. If is_new, commit registry using vvx_commit with format vvb:HALLMARK::A: allocate hallmark for BURC_MANAGED_KITS | Verify: tt/vow-b.Build.sh

**[260124-0813] rough**

Fix vob_release() conformance with VOS spec.

**Prerequisite:** ₢AFAAm (vos-commit-message-format) must complete first.

**Changes:**

1. **Parcel output location:**
   - Create `.jjk/parcels/` directory if not exists
   - Output parcel to `.jjk/parcels/{kit}-{hallmark}.tar.gz`
   - Add `.jjk/parcels/` to `.gitignore`

2. **Registry commit:**
   - After allocating new hallmark in registry, commit using `vvx_commit`
   - Message format: `vvb:HALLMARK::A: allocate hallmark XXXX for {kit}`
   - Uses the newly allocated hallmark in the message

**Files:** vob_release.rs (or equivalent), .gitignore

**[260119-0930] rough**

Fix vob_release() conformance with VOS spec:
1. Registry commit: VOS line 917 requires 'commit registry change' after allocating new hallmark - implementation updates file but doesn't commit
2. Parcel output location: BURC_PROJECT_ROOT is misapplied - parcel goes to parent dir instead of kit forge root; VOS needs explicit spec text for output directory

Requires human decisions on:
- Where exactly should parcel be output (kit forge root? explicit --output flag?)
- Commit message format for registry commits
- Whether to use vvx_commit or direct git

### rein-token-efficiency (₢AFAAV) [complete]

**[260124-1050] complete**

Rethink /jjc-heat-rein output to reduce token usage.

Current issue: jjx_rein returns JSON which Claude then reformats. This consumes tokens twice (raw JSON + formatted output).

Questions to resolve:
- Should jjx_rein output human-readable text directly?
- Or should the slash command use a more compact summary?
- What information is actually useful from rein? (recent context vs full history)
- Consider: rein exists to show steeplechase history — is formatted text better than structured JSON for this use case?

**[260123-2013] bridled**

Rethink /jjc-heat-rein output to reduce token usage.

Current issue: jjx_rein returns JSON which Claude then reformats. This consumes tokens twice (raw JSON + formatted output).

Questions to resolve:
- Should jjx_rein output human-readable text directly?
- Or should the slash command use a more compact summary?
- What information is actually useful from rein? (recent context vs full history)
- Consider: rein exists to show steeplechase history — is formatted text better than structured JSON for this use case?

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrs_steeplechase.rs, jjc-heat-rein.md (2 files)
Steps:
1. In jjrs_run, replace serde_json::to_string_pretty with formatted text loop
2. Output format: TIMESTAMP  COMMIT  [ACTION] CORONET  SUBJECT
3. Use fixed-width columns: timestamp (16), commit (8), action (3), coronet (7)
4. Simplify slash command to run and display output directly
Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260123-1318] rough**

Rethink /jjc-heat-rein output to reduce token usage.

Current issue: jjx_rein returns JSON which Claude then reformats. This consumes tokens twice (raw JSON + formatted output).

Questions to resolve:
- Should jjx_rein output human-readable text directly?
- Or should the slash command use a more compact summary?
- What information is actually useful from rein? (recent context vs full history)
- Consider: rein exists to show steeplechase history — is formatted text better than structured JSON for this use case?

### rein-column-format (₢AFAAw) [complete]

**[260124-1059] complete**

Change jjx_rein to output column-aligned plain text instead of JSON.

**Current:** JSON array requiring agent to parse and format (wastes tokens)

**New format:**
```
Timestamp          Commit    Act  Affil     Subject
-----------------------------------------------------------------
2026-01-24 09:20   f4824e77  [T]  ₣AF       Tally: parade-remaining-markdown
2026-01-24 08:49   857d982c  [F]  ₢AFAAf    Executing bridled pace...
2026-01-24 08:47   a6b75159  [W]  ₢AFAAn    pace complete
```

**Affil column logic:**
- If entry has coronet: show ₢XXXXX (pace-level)
- Otherwise: show ₣XX (heat-level, from the target firemark)

**Implementation:**
- Modify jjrq_run_rein in jjrq_query.rs
- Output header row with column names
- Output separator line of dashes
- For each entry: timestamp, commit (8 char), action in brackets, affil (coronet or firemark), subject
- Columns: fixed width, left-aligned

**Files:** jjrq_query.rs, JJSCRN-rein.adoc (update stdout docs)

**[260124-0942] bridled**

Change jjx_rein to output column-aligned plain text instead of JSON.

**Current:** JSON array requiring agent to parse and format (wastes tokens)

**New format:**
```
Timestamp          Commit    Act  Affil     Subject
-----------------------------------------------------------------
2026-01-24 09:20   f4824e77  [T]  ₣AF       Tally: parade-remaining-markdown
2026-01-24 08:49   857d982c  [F]  ₢AFAAf    Executing bridled pace...
2026-01-24 08:47   a6b75159  [W]  ₢AFAAn    pace complete
```

**Affil column logic:**
- If entry has coronet: show ₢XXXXX (pace-level)
- Otherwise: show ₣XX (heat-level, from the target firemark)

**Implementation:**
- Modify jjrq_run_rein in jjrq_query.rs
- Output header row with column names
- Output separator line of dashes
- For each entry: timestamp, commit (8 char), action in brackets, affil (coronet or firemark), subject
- Columns: fixed width, left-aligned

**Files:** jjrq_query.rs, JJSCRN-rein.adoc (update stdout docs)

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrq_query.rs, JJSCRN-rein.adoc (2 files) | Steps: 1. Modify jjrs_get_entries or jjrq_run_rein to output column-aligned text 2. Add header row with Timestamp, Commit, Act, Affil, Subject columns 3. Add separator line of dashes 4. Format entries with fixed-width columns, affil shows coronet if present else firemark 5. Update JJSCRN-rein.adoc stdout section | Verify: tt/vow-b.Build.sh

**[260124-0929] rough**

Change jjx_rein to output column-aligned plain text instead of JSON.

**Current:** JSON array requiring agent to parse and format (wastes tokens)

**New format:**
```
Timestamp          Commit    Act  Affil     Subject
-----------------------------------------------------------------
2026-01-24 09:20   f4824e77  [T]  ₣AF       Tally: parade-remaining-markdown
2026-01-24 08:49   857d982c  [F]  ₢AFAAf    Executing bridled pace...
2026-01-24 08:47   a6b75159  [W]  ₢AFAAn    pace complete
```

**Affil column logic:**
- If entry has coronet: show ₢XXXXX (pace-level)
- Otherwise: show ₣XX (heat-level, from the target firemark)

**Implementation:**
- Modify jjrq_run_rein in jjrq_query.rs
- Output header row with column names
- Output separator line of dashes
- For each entry: timestamp, commit (8 char), action in brackets, affil (coronet or firemark), subject
- Columns: fixed width, left-aligned

**Files:** jjrq_query.rs, JJSCRN-rein.adoc (update stdout docs)

### test-trophy-operation (₢AFAAk) [complete]

**[260124-1624] complete**

Test the heat retirement (trophy) operation end-to-end and repair any issues found.

## Scope

1. Create a test heat with a few paces (some complete, some abandoned)
2. Run /jjc-heat-retire on it
3. Verify trophy file is created correctly in .claude/jjm/retired/
4. Verify steeplechase entries are preserved
5. Verify heat is removed from active gallops
6. Fix any bugs discovered during testing

## Test Heat Setup

- Nominate a throwaway heat (silks: test-trophy-validation)
- Add 2-3 paces with minimal specs
- Complete one pace, abandon another
- Add a chalk marker
- Then retire it

## Expected Trophy Contents

Per JJSA spec:
- Header: firemark, silks, duration, pace counts
- Paddock: full text preserved
- Paces table: all paces with final states
- Steeplechase: git history for heat

## Not Bridleable

Exploratory testing with potential bug fixes - requires human judgment on what constitutes correct behavior and how to fix issues.

**[260124-0751] rough**

Test the heat retirement (trophy) operation end-to-end and repair any issues found.

## Scope

1. Create a test heat with a few paces (some complete, some abandoned)
2. Run /jjc-heat-retire on it
3. Verify trophy file is created correctly in .claude/jjm/retired/
4. Verify steeplechase entries are preserved
5. Verify heat is removed from active gallops
6. Fix any bugs discovered during testing

## Test Heat Setup

- Nominate a throwaway heat (silks: test-trophy-validation)
- Add 2-3 paces with minimal specs
- Complete one pace, abandon another
- Add a chalk marker
- Then retire it

## Expected Trophy Contents

Per JJSA spec:
- Header: firemark, silks, duration, pace counts
- Paddock: full text preserved
- Paces table: all paces with final states
- Steeplechase: git history for heat

## Not Bridleable

Exploratory testing with potential bug fixes - requires human judgment on what constitutes correct behavior and how to fix issues.

### implement-garland-ceremony (₢AFAAK) [abandoned]

**[260124-0918] abandoned**

Superseded by split paces: garland-silks-parser (₢AFAAp), garland-primitive (₢AFAAq), garland-spec (₢AFAAr), garland-slash-cmd (₢AFAAs)

**[260124-0915] rough**

Implement jjx_garland primitive and /jjc-heat-garland slash command.

## Concept

When a heat has run many paces, garland transfers remaining work to a fresh continuation heat while preserving the original for retrospective.

## jjx_garland Primitive

**Command:** `jjx_garland <FIREMARK>`

**Behavior:**
1. No threshold validation — execute when called
2. Rename source heat: add `garlanded-` prefix and sequence suffix
   - `jjk-post-alpha-polish` → `garlanded-jjk-post-alpha-polish-01`
   - `jjk-post-alpha-polish-01` → `garlanded-jjk-post-alpha-polish-02`
3. Add steeplechase marker: "Garlanded at pace {N} — magnificent service"
4. Furlough source to stabled
5. Nominate new heat with incremented suffix (starts racing)
   - New heat: `jjk-post-alpha-polish-02` (or `-03`, etc.)
6. Copy paddock verbatim to new heat
7. Draft actionable paces (rough + bridled) to new heat
8. Retain complete + abandoned paces with source

**Output (JSON):**
```json
{
  "old_firemark": "AF",
  "old_silks": "garlanded-jjk-post-alpha-polish-01",
  "new_firemark": "AH",
  "new_silks": "jjk-post-alpha-polish-02",
  "paces_transferred": 15,
  "paces_retained": 25
}
```

## /jjc-heat-garland Slash Command

1. Confirmation with context reminder:
   "Garland ₣AF? This transfers N paces to a new heat. Context-heavy operation — consider /clear first if session is long. Proceed? [y/n]"
2. Call jjx_garland
3. Report brief summary
4. Suggest: "Run /jjc-heat-groom <new-firemark> to review paddock for the new heat."

## JJSA Spec

Create `JJSCGL-garland.adoc` with full MCM-style specification. Draft content provided below.

### JJSCGL-garland.adoc Content

```asciidoc
Transfer remaining
{jjdpr_pace_s}
from a
{jjdhr_heat}
to a fresh continuation
{jjdhr_heat},
preserving the original for retrospective.
This is a ceremony operation that combines multiple primitives.

The source
{jjdhr_heat}
receives a `garlanded-` prefix and sequence suffix in its
{jjdhm_silks},
then is furloughed to
{jjdhe_stabled}.
A new
{jjdhr_heat}
is nominated with incremented suffix, starts
{jjdhe_racing},
and receives all actionable
{jjdpr_pace_s}
({jjdpe_rough}
and
{jjdpe_bridled}).
{jjdpe_complete}
and
{jjdpe_abandoned}
{jjdpr_pace_s}
remain with the garlanded source.

{jjds_arguments}

// ⟦axd_optional axd_defaulted⟧
* {jjda_file}

// ⟦axd_required⟧
* {jjdt_firemark}
(positional) —
{jjdhr_heat}
to garland

{jjds_stdout} JSON object:

[source,json]
----
{
  "old_firemark": "AF",
  "old_silks": "garlanded-jjk-post-alpha-polish-01",
  "new_firemark": "AH",
  "new_silks": "jjk-post-alpha-polish-02",
  "paces_transferred": 15,
  "paces_retained": 25
}
----

{jjds_exit_uniform} 0 success, non-zero error.

{jjds_behavior}

. {jjdr_load}
{jjda_file};
on failure, exit immediately with
{jjdr_load}
error status
. Verify source
{jjdhr_heat}
exists with given
{jjdt_firemark}
. Determine sequence number:
.. If source
{jjdhm_silks}
has no `-NN` suffix: sequence = 1
.. If source
{jjdhm_silks}
ends with `-NN`: sequence = NN
. Compute new
{jjdhm_silks}
for source:
`garlanded-` + base silks (without any `-NN` suffix) + `-` + sequence (zero-padded 2 digits)
. Compute new
{jjdhm_silks}
for continuation:
base silks (without `-NN` suffix) + `-` + (sequence + 1) (zero-padded 2 digits)
. Add
{jjdkr_steeplechase}
marker to source:
`Garlanded at pace {complete_count} — magnificent service`
. Update source
{jjdhm_silks}
to garlanded name
. Update source
{jjdhm_status}
to
{jjdhe_stabled}
. Nominate continuation
{jjdhr_heat}
with computed
{jjdhm_silks},
status
{jjdhe_racing}
. Copy source
{jjdhm_paddock}
to continuation
{jjdhr_heat}
. Partition source
{jjdpr_pace_s}:
.. Actionable:
{jjdpe_rough}
or
{jjdpe_bridled}
→ transfer
.. Retained:
{jjdpe_complete}
or
{jjdpe_abandoned}
→ stay with source
. For each actionable
{jjdpr_pace}
in
{jjdhm_order}:
.. Draft to continuation using
{jjdr_draft}
logic (new
{jjdt_coronet},
preserve
{jjdkr_tack}
history)
. {jjdr_save}
{jjdgr_gallops}
to
{jjda_file}
. Output JSON to stdout

*Validation errors:*

[cols="2,3"]
|===
| Condition | Error

| Source {jjdhr_heat} not found
| "Heat {firemark} not found"

| Source has no actionable {jjdpr_pace_s}
| "Heat {firemark} has no actionable paces to transfer"
|===
```

## Files

- `Tools/jjk/vov_veiled/src/jjrq_query.rs` — add jjrq_run_garland function
- `Tools/jjk/vov_veiled/src/jjrx_cli.rs` — add GarlandArgs and wire to subcommand
- `Tools/jjk/vov_veiled/JJSCGL-garland.adoc` — JJSA spec (create new)
- `.claude/commands/jjc-heat-garland.md` — slash command (create new)

## Implementation Notes

- Reuse jjdr_draft logic for pace transfer (coronet minting, tack history)
- Silks parsing: regex for `-(\d{2})$` suffix detection
- Steeplechase marker uses existing chalk mechanism internally

**[260121-1925] rough**

Implement /jjc-heat-garland ceremony for honoring a heat that has run many paces.

## Concept

When a heat has completed 20+ paces, it deserves celebration and the remaining work should transfer to a fresh heat. "Garland" acknowledges magnificent service while preserving the heat for later retrospective.

## Ceremony behavior

1. Add steeplechase marker: "Garlanded at pace N — magnificent service"
2. Furlough heat to stabled (preserves for later retrospective)
3. Nominate fresh heat (prompt for new silks, or derive from old)
4. Draft remaining paces to new heat
5. Open new paddock for rewriting — context has evolved

## Design questions to resolve

- What is slash command vs jjx_ primitive boundary?
- Should there be a jjx_garland primitive, or is this pure ceremony using existing primitives (chalk, furlough, nominate, draft)?
- Threshold for warning (20 completed) — where does this live?
- Warning placement: in groom? in muster? both?

## Related

- jjx_draft already moves paces between heats
- /jjc-heat-restring is precedent for ceremony using draft primitive
- Furlough already handles stabling

### fix-jjd-reslate-extraction (₢AFAAY) [complete]

**[260123-1929] complete**

Fix remaining JJD→JJSA references in pace specs.

Paces needing reslate:
- ₢ADAAC liturgy-state-machine-vocabulary
- ₢AIAAA whisper-conclave-lite

The tabtarget wrapper outputs log lines to stdout which breaks jq piping. Options:
1. Fix the tabtarget to send logs to stderr
2. Use grep to extract JSON portion before piping to jq
3. Have jjx_peek write to a temp file

Once extraction works, the reslate pattern is:
```bash
jjx_peek <CORONET> | jq -r .spec | sed "s/JJD/JJSA/g" | jjx_tally <CORONET>
```

Note: ₢AFAAP was already fixed via direct jq on gallops.json.

**[260123-1453] rough**

Fix remaining JJD→JJSA references in pace specs.

Paces needing reslate:
- ₢ADAAC liturgy-state-machine-vocabulary
- ₢AIAAA whisper-conclave-lite

The tabtarget wrapper outputs log lines to stdout which breaks jq piping. Options:
1. Fix the tabtarget to send logs to stderr
2. Use grep to extract JSON portion before piping to jq
3. Have jjx_peek write to a temp file

Once extraction works, the reslate pattern is:
```bash
jjx_peek <CORONET> | jq -r .spec | sed "s/JJD/JJSA/g" | jjx_tally <CORONET>
```

Note: ₢AFAAP was already fixed via direct jq on gallops.json.

### implement-text-emitter-commands (₢AFAAZ) [complete]

**[260123-1629] complete**

Replace jjx_order/jjx_peek with purpose-built text emitters.

## Problem

JSON output commands require jq parsing, are token-heavy, and break when tabtarget logs pollute stdout.

## Solution

Implement simple text emitters:
- `jjx_get_pace_spec <CORONET>` — raw spec text only
- `jjx_get_pace_silks <CORONET>` — silks string only  
- `jjx_get_heat_coronets <FIREMARK> [--remaining]` — one coronet per line

## Benefits

- No jq needed
- Pipes cleanly: `jjx_get_pace_spec ADAAC | sed "s/JJD/JJSA/g" | jjx_tally ADAAC`
- Token-minimal
- Unix-composable

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs — add new commands, remove order/peek
- Tools/jjk/vov_veiled/src/jjrq_query.rs — add implementations, remove order/peek
- .claude/commands/jjc-heat-rail.md — update to use jjx_get_heat_coronets
- .claude/commands/jjc-heat-quarter.md — update to use new commands
- .claude/commands/jjc-pace-reslate.md — update to use jjx_get_pace_spec
- .claude/commands/jjc-heat-restring.md — update to use jjx_get_heat_coronets

## Constraint

Also fix tabtarget log pollution (logs should go to stderr, not stdout).

**[260123-1458] rough**

Replace jjx_order/jjx_peek with purpose-built text emitters.

## Problem

JSON output commands require jq parsing, are token-heavy, and break when tabtarget logs pollute stdout.

## Solution

Implement simple text emitters:
- `jjx_get_pace_spec <CORONET>` — raw spec text only
- `jjx_get_pace_silks <CORONET>` — silks string only  
- `jjx_get_heat_coronets <FIREMARK> [--remaining]` — one coronet per line

## Benefits

- No jq needed
- Pipes cleanly: `jjx_get_pace_spec ADAAC | sed "s/JJD/JJSA/g" | jjx_tally ADAAC`
- Token-minimal
- Unix-composable

## Files

- Tools/jjk/vov_veiled/src/jjrx_cli.rs — add new commands, remove order/peek
- Tools/jjk/vov_veiled/src/jjrq_query.rs — add implementations, remove order/peek
- .claude/commands/jjc-heat-rail.md — update to use jjx_get_heat_coronets
- .claude/commands/jjc-heat-quarter.md — update to use new commands
- .claude/commands/jjc-pace-reslate.md — update to use jjx_get_pace_spec
- .claude/commands/jjc-heat-restring.md — update to use jjx_get_heat_coronets

## Constraint

Also fix tabtarget log pollution (logs should go to stderr, not stdout).

### test-heredoc (₢AFAAh) [abandoned]

**[260124-0748] abandoned**

Simple content with backticks: `hello`

**[260124-0743] rough**

Simple content with backticks: `hello`

### test-heredoc-2 (₢AFAAi) [abandoned]

**[260124-0748] abandoned**

Content with code fence:

```bash
echo "TEXT" | command
```

And backticks: `hello` and `world`

**[260124-0743] rough**

Content with code fence:

```bash
echo "TEXT" | command
```

And backticks: `hello` and `world`

### bud-absolute-paths (₢AFAAl) [complete]

**[260124-1014] complete**

Make BUD_TEMP_DIR, BUD_OUTPUT_DIR, and BUD_TRANSCRIPT absolute paths in zbud_setup() to prevent breakage when dispatched scripts change directories. Use PWD builtin (not subshell) per BCG patterns.

**[260124-0806] bridled**

Make BUD_TEMP_DIR, BUD_OUTPUT_DIR, and BUD_TRANSCRIPT absolute paths in zbud_setup() to prevent breakage when dispatched scripts change directories. Use PWD builtin (not subshell) per BCG patterns.

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: bud_dispatch.sh (1 file)
Steps:
1. After BUD_TEMP_DIR assignment at line 78, add case statement to prepend PWD if path is not absolute
2. After BUD_OUTPUT_DIR assignment at line 97, add same case statement pattern
3. BUD_TRANSCRIPT derives from BUD_TEMP_DIR so needs no change
Pattern:
case "${VAR}" in
  /*) ;;
  *)  VAR="${PWD}/${VAR}" ;;
esac
Verify: Run tabtarget and confirm BUD_TEMP_DIR and BUD_OUTPUT_DIR are absolute in output

**[260124-0802] rough**

Make BUD_TEMP_DIR, BUD_OUTPUT_DIR, and BUD_TRANSCRIPT absolute paths in zbud_setup() to prevent breakage when dispatched scripts change directories. Use PWD builtin (not subshell) per BCG patterns.

### simplify-saddle-recent-work (₢AFAAz) [complete]

**[260124-1641] complete**

Simplify the Recent-work table in jjx_saddle output.

## Changes

1. **Filter entries**: Only show action codes `n` (notch), `A` (approach), `d` (discussion). Exclude lifecycle noise like `W` (wrap), `F` (fly), `S` (slate), etc.

2. **Remove columns**: Drop Timestamp and [A] (action code) columns.

3. **Keep columns**: Commit, Identity, Subject (3 columns).

## Location

`Tools/jjk/vov_veiled/src/jjrq_query.rs:288-337`

## Before

```
Recent-work:
Timestamp         Commit    [A]    Identity  Subject
----------------  --------  -----  --------  --------
2026-01-24 10:19  a6d71428  [n]    ₢AFAAg    Remove dead struct...
2026-01-24 10:14  b136fc79  [F]    ₢AFAAg    Executing bridled pace...
```

## After

```
Recent-work:
Commit    Identity  Subject
--------  --------  --------
a6d71428  ₢AFAAg    Remove dead struct...
```

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260124-1627] bridled**

Simplify the Recent-work table in jjx_saddle output.

## Changes

1. **Filter entries**: Only show action codes `n` (notch), `A` (approach), `d` (discussion). Exclude lifecycle noise like `W` (wrap), `F` (fly), `S` (slate), etc.

2. **Remove columns**: Drop Timestamp and [A] (action code) columns.

3. **Keep columns**: Commit, Identity, Subject (3 columns).

## Location

`Tools/jjk/vov_veiled/src/jjrq_query.rs:288-337`

## Before

```
Recent-work:
Timestamp         Commit    [A]    Identity  Subject
----------------  --------  -----  --------  --------
2026-01-24 10:19  a6d71428  [n]    ₢AFAAg    Remove dead struct...
2026-01-24 10:14  b136fc79  [F]    ₢AFAAg    Executing bridled pace...
```

## After

```
Recent-work:
Commit    Identity  Subject
--------  --------  --------
a6d71428  ₢AFAAg    Remove dead struct...
```

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrq_query.rs (1 file) | Steps: 1. Filter recent_work to keep only entries where action is n, A, or d 2. Change table columns from 5 to 3: Commit, Identity, Subject 3. Update jjrp_measure call to pass 3 values 4. Update jjrp_print_row call to pass 3 values | Verify: tt/vow-b.Build.sh

**[260124-1028] rough**

Simplify the Recent-work table in jjx_saddle output.

## Changes

1. **Filter entries**: Only show action codes `n` (notch), `A` (approach), `d` (discussion). Exclude lifecycle noise like `W` (wrap), `F` (fly), `S` (slate), etc.

2. **Remove columns**: Drop Timestamp and [A] (action code) columns.

3. **Keep columns**: Commit, Identity, Subject (3 columns).

## Location

`Tools/jjk/vov_veiled/src/jjrq_query.rs:288-337`

## Before

```
Recent-work:
Timestamp         Commit    [A]    Identity  Subject
----------------  --------  -----  --------  --------
2026-01-24 10:19  a6d71428  [n]    ₢AFAAg    Remove dead struct...
2026-01-24 10:14  b136fc79  [F]    ₢AFAAg    Executing bridled pace...
```

## After

```
Recent-work:
Commit    Identity  Subject
--------  --------  --------
a6d71428  ₢AFAAg    Remove dead struct...
```

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

### trophy-alpha-jjsa-commit-types (₢AFAA0) [complete]

**[260124-1647] complete**

Update JJSA spec with new commit types and parameter for trophy data collection.

## Additions to JJSA

### Commit Types Section

Add to "Commit Message Patterns" section:

**B (Bridle)** — Records bridling decision before autonomous execution
- Created when pace transitions to bridled state
- Subject: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
- Body contains full direction text

**L (Landing)** — Records agent return after autonomous execution
- Created when spawned agent completes (success or failure)
- Subject: `jjb:HALLMARK:₢XXXXX:L: {agent} {landed|crashed} | {steps_completed}/{steps_total} | verify: {pass|fail}`
- Body contains files touched, duration, error if crashed

### Arguments Section

Add `--intent` argument for jjx_notch:
- Optional string parameter
- If provided, used as commit subject (opus-supplied intent)
- If omitted, falls back to haiku diff description

## Files

- JJSA-GallopsData.adoc

**[260124-1643] bridled**

Update JJSA spec with new commit types and parameter for trophy data collection.

## Additions to JJSA

### Commit Types Section

Add to "Commit Message Patterns" section:

**B (Bridle)** — Records bridling decision before autonomous execution
- Created when pace transitions to bridled state
- Subject: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
- Body contains full direction text

**L (Landing)** — Records agent return after autonomous execution
- Created when spawned agent completes (success or failure)
- Subject: `jjb:HALLMARK:₢XXXXX:L: {agent} {landed|crashed} | {steps_completed}/{steps_total} | verify: {pass|fail}`
- Body contains files touched, duration, error if crashed

### Arguments Section

Add `--intent` argument for jjx_notch:
- Optional string parameter
- If provided, used as commit subject (opus-supplied intent)
- If omitted, falls back to haiku diff description

## Files

- JJSA-GallopsData.adoc

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: JJSA-GallopsData.adoc (1 file) | Steps: 1. Read existing Commit Message Patterns section 2. Add B Bridle marker definition after existing markers 3. Add L Landing marker definition 4. Add --intent argument to jjx_notch Arguments section | Verify: visual review of adoc formatting

**[260124-1149] rough**

Update JJSA spec with new commit types and parameter for trophy data collection.

## Additions to JJSA

### Commit Types Section

Add to "Commit Message Patterns" section:

**B (Bridle)** — Records bridling decision before autonomous execution
- Created when pace transitions to bridled state
- Subject: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
- Body contains full direction text

**L (Landing)** — Records agent return after autonomous execution
- Created when spawned agent completes (success or failure)
- Subject: `jjb:HALLMARK:₢XXXXX:L: {agent} {landed|crashed} | {steps_completed}/{steps_total} | verify: {pass|fail}`
- Body contains files touched, duration, error if crashed

### Arguments Section

Add `--intent` argument for jjx_notch:
- Optional string parameter
- If provided, used as commit subject (opus-supplied intent)
- If omitted, falls back to haiku diff description

## Files

- JJSA-GallopsData.adoc

### trophy-alpha-bridle-marker (₢AFAA1) [complete]

**[260124-1709] complete**

Implement B commit when pace is bridled.

## Trigger

When `/jjc-pace-bridle` transitions a pace to bridled state.

## Implementation

In the bridle flow (jjx_tally with state=bridled):
1. After tack is added to gallops
2. Create chalk-style commit with B marker
3. Subject format: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
4. Body: full direction text from the tack

## Changes

- jjrn_notch.rs: Add ChalkMarker::Bridle or similar
- jjro_ops.rs: Create B commit in tally-bridled path
- Update /jjc-pace-bridle slash command if needed

## Depends on

trophy-alpha-jjsa-commit-types (spec must be defined first)

**[260124-1649] bridled**

Implement B commit when pace is bridled.

## Trigger

When `/jjc-pace-bridle` transitions a pace to bridled state.

## Implementation

In the bridle flow (jjx_tally with state=bridled):
1. After tack is added to gallops
2. Create chalk-style commit with B marker
3. Subject format: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
4. Body: full direction text from the tack

## Changes

- jjrn_notch.rs: Add ChalkMarker::Bridle or similar
- jjro_ops.rs: Create B commit in tally-bridled path
- Update /jjc-pace-bridle slash command if needed

## Depends on

trophy-alpha-jjsa-commit-types (spec must be defined first)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrn_notch.rs, jjro_ops.rs (2 files) | Steps: 1. Read existing ChalkMarker enum in jjrn_notch.rs 2. Add Bridle variant following existing pattern 3. Add format_bridle_message function or extend existing chalk formatting 4. In jjro_ops.rs find tally logic for state=bridled 5. After tack added, create B commit with subject jjb:HALLMARK:CORONET:B: agent pipe file_count files pipe silks and body containing direction text | Verify: tt/vow-b.Build.sh

**[260124-1150] rough**

Implement B commit when pace is bridled.

## Trigger

When `/jjc-pace-bridle` transitions a pace to bridled state.

## Implementation

In the bridle flow (jjx_tally with state=bridled):
1. After tack is added to gallops
2. Create chalk-style commit with B marker
3. Subject format: `jjb:HALLMARK:₢XXXXX:B: {agent} | {file_count} files | {silks}`
4. Body: full direction text from the tack

## Changes

- jjrn_notch.rs: Add ChalkMarker::Bridle or similar
- jjro_ops.rs: Create B commit in tally-bridled path
- Update /jjc-pace-bridle slash command if needed

## Depends on

trophy-alpha-jjsa-commit-types (spec must be defined first)

### trophy-alpha-landing-marker (₢AFAA2) [complete]

**[260124-1732] complete**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Changes

### 1. jjrn_notch.rs
- Add `Landing` variant to ChalkMarker enum
- Add `jjrn_format_landing_message(coronet, agent)` function
- Returns: `jjb:HALLMARK:₢XXXXX:L: {agent} landed`

### 2. jjrx_cli.rs
- Add `jjx_landing` subcommand
- Args: `<CORONET> <AGENT>`
- Body via stdin (agent completion report)
- Creates L commit with formatted subject and stdin as body

### 3. JJSA-GallopsData.adoc
- Document `jjx_landing` command in Commands section
- Usage: `echo "agent output" | jjx_landing <CORONET> <AGENT>`

### 4. /jjc-heat-mount.md
- After Task agent returns in bridled flow
- Before asking "Ready to wrap?"
- Invoke: `echo "{agent_output}" | jjx_landing <CORONET> <AGENT>`

## Outcome

Always "landed" for now - body tells the actual story.

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

**[260124-1721] bridled**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Changes

### 1. jjrn_notch.rs
- Add `Landing` variant to ChalkMarker enum
- Add `jjrn_format_landing_message(coronet, agent)` function
- Returns: `jjb:HALLMARK:₢XXXXX:L: {agent} landed`

### 2. jjrx_cli.rs
- Add `jjx_landing` subcommand
- Args: `<CORONET> <AGENT>`
- Body via stdin (agent completion report)
- Creates L commit with formatted subject and stdin as body

### 3. JJSA-GallopsData.adoc
- Document `jjx_landing` command in Commands section
- Usage: `echo "agent output" | jjx_landing <CORONET> <AGENT>`

### 4. /jjc-heat-mount.md
- After Task agent returns in bridled flow
- Before asking "Ready to wrap?"
- Invoke: `echo "{agent_output}" | jjx_landing <CORONET> <AGENT>`

## Outcome

Always "landed" for now - body tells the actual story.

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrn_notch.rs, jjrx_cli.rs, JJSA-GallopsData.adoc, jjc-heat-mount.md (4 files) | Steps: 1. Add Landing variant to ChalkMarker in jjrn_notch.rs following Bridle pattern 2. Add jjrn_format_landing_message function returning jjb:HALLMARK:CORONET:L: agent landed 3. Add jjx_landing subcommand in jjrx_cli.rs taking CORONET AGENT args and body via stdin 4. Document jjx_landing in JJSA Commands section 5. Update jjc-heat-mount.md bridled flow to invoke jjx_landing after Task returns | Verify: tt/vow-b.Build.sh

**[260124-1720] rough**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Changes

### 1. jjrn_notch.rs
- Add `Landing` variant to ChalkMarker enum
- Add `jjrn_format_landing_message(coronet, agent)` function
- Returns: `jjb:HALLMARK:₢XXXXX:L: {agent} landed`

### 2. jjrx_cli.rs
- Add `jjx_landing` subcommand
- Args: `<CORONET> <AGENT>`
- Body via stdin (agent completion report)
- Creates L commit with formatted subject and stdin as body

### 3. JJSA-GallopsData.adoc
- Document `jjx_landing` command in Commands section
- Usage: `echo "agent output" | jjx_landing <CORONET> <AGENT>`

### 4. /jjc-heat-mount.md
- After Task agent returns in bridled flow
- Before asking "Ready to wrap?"
- Invoke: `echo "{agent_output}" | jjx_landing <CORONET> <AGENT>`

## Outcome

Always "landed" for now - body tells the actual story.

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

**[260124-1719] rough**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Implementation

After Task agent returns, before asking "Ready to wrap?":
1. Create L marker commit
2. Subject: `jjb:HALLMARK:₢XXXXX:L: {agent} landed`
3. Body: agent's completion report (the Task tool result)

## Changes

- jjrn_notch.rs: Add Landing variant to ChalkMarker, add jjrn_format_landing_message(coronet, agent) function
- /jjc-heat-mount: After Task returns, create L commit with agent output as body

## Outcome

For now, always use "landed" - the body contains the actual outcome details. "Crashed" reserved for Task tool failures (rare).

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

**[260124-1718] rough**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Implementation

After Task agent returns, before asking "Ready to wrap?":
1. Determine outcome: landed (success) or crashed (error)
2. Create L marker commit
3. Subject: `jjb:HALLMARK:₢XXXXX:L: {agent} {landed|crashed}`
4. Body: agent's completion report (raw text)

## Changes

- jjrn_notch.rs: Add Landing variant to ChalkMarker, add jjrn_format_landing_message function
- /jjc-heat-mount: After Task returns, create L commit with agent output

## Outcome Detection

- **landed**: Task tool returns successfully with agent output
- **crashed**: Task tool returns error or agent reports failure

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

**[260124-1150] rough**

Implement L commit when agent returns from autonomous execution.

## Trigger

When a spawned agent completes execution in `/jjc-heat-mount` bridled flow.

## Implementation

Before asking user "Ready to wrap?":
1. Collect execution metadata from agent result
2. Create chalk-style commit with L marker
3. Subject format: `jjb:HALLMARK:₢XXXXX:L: {agent} {landed|crashed} | {steps}/{total} | verify: {pass|fail}`
4. Body: files touched, duration, error message if crashed

## Data to Capture

- Agent tier (haiku/sonnet/opus)
- Steps completed vs total (parse from direction)
- Verification result (did build/test pass)
- Files actually touched (from git status or agent report)
- Duration (from timestamps)

## Changes

- jjrn_notch.rs: Add Landing marker type
- Update /jjc-heat-mount to create L commit after agent returns
- May need agent to report structured result

## Depends on

trophy-alpha-bridle-marker (B must exist to have flights to land)

### trophy-alpha-notch-intent (₢AFAA3) [complete]

**[260124-1737] complete**

Add --intent parameter to jjx_notch for opus-supplied commit messages.

## Purpose

Allow opus driver to provide top-down intent ("what we meant to do") instead of relying on haiku's bottom-up diff description.

## CLI Change

```
jjx_notch <CORONET> [--intent "message"] <files...>
```

- If --intent provided: use as commit subject
- If omitted: fall back to current haiku behavior

## Implementation

- jjrx_cli.rs: Add --intent Option<String> to notch args
- jjro_ops.rs: In notch logic, check for intent before calling haiku
- Format: `jjb:HALLMARK:₢XXXXX:n: {intent}` (same prefix, user message)

## Slash Command Update

Update /jjc-pace-notch:
- Prompt opus for one-line intent summary
- Pass to jjx_notch --intent

## Backward Compatible

Omitting --intent preserves current haiku-generated messages.

**[260124-1713] bridled**

Add --intent parameter to jjx_notch for opus-supplied commit messages.

## Purpose

Allow opus driver to provide top-down intent ("what we meant to do") instead of relying on haiku's bottom-up diff description.

## CLI Change

```
jjx_notch <CORONET> [--intent "message"] <files...>
```

- If --intent provided: use as commit subject
- If omitted: fall back to current haiku behavior

## Implementation

- jjrx_cli.rs: Add --intent Option<String> to notch args
- jjro_ops.rs: In notch logic, check for intent before calling haiku
- Format: `jjb:HALLMARK:₢XXXXX:n: {intent}` (same prefix, user message)

## Slash Command Update

Update /jjc-pace-notch:
- Prompt opus for one-line intent summary
- Pass to jjx_notch --intent

## Backward Compatible

Omitting --intent preserves current haiku-generated messages.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrx_cli.rs, jjro_ops.rs, jjc-pace-notch.md (3 files) | Steps: 1. Add --intent Option String to NotchArgs in jjrx_cli.rs with clap annotation 2. In jjro_ops.rs notch logic use intent as subject when provided, else fall back to haiku 3. Update .claude/commands/jjc-pace-notch.md to prompt opus for one-line intent and pass --intent to invocation | Verify: tt/vow-b.Build.sh

**[260124-1643] bridled**

Add --intent parameter to jjx_notch for opus-supplied commit messages.

## Purpose

Allow opus driver to provide top-down intent ("what we meant to do") instead of relying on haiku's bottom-up diff description.

## CLI Change

```
jjx_notch <CORONET> [--intent "message"] <files...>
```

- If --intent provided: use as commit subject
- If omitted: fall back to current haiku behavior

## Implementation

- jjrx_cli.rs: Add --intent Option<String> to notch args
- jjro_ops.rs: In notch logic, check for intent before calling haiku
- Format: `jjb:HALLMARK:₢XXXXX:n: {intent}` (same prefix, user message)

## Slash Command Update

Update /jjc-pace-notch:
- Prompt opus for one-line intent summary
- Pass to jjx_notch --intent

## Backward Compatible

Omitting --intent preserves current haiku-generated messages.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrx_cli.rs, jjro_ops.rs (2 files) | Steps: 1. Add --intent Option String to NotchArgs struct in jjrx_cli.rs 2. Add clap arg annotation for --intent 3. In jjro_ops.rs notch logic, check if intent provided and use as subject instead of haiku call 4. Preserve existing haiku fallback when intent is None | Verify: tt/vow-b.Build.sh

**[260124-1150] rough**

Add --intent parameter to jjx_notch for opus-supplied commit messages.

## Purpose

Allow opus driver to provide top-down intent ("what we meant to do") instead of relying on haiku's bottom-up diff description.

## CLI Change

```
jjx_notch <CORONET> [--intent "message"] <files...>
```

- If --intent provided: use as commit subject
- If omitted: fall back to current haiku behavior

## Implementation

- jjrx_cli.rs: Add --intent Option<String> to notch args
- jjro_ops.rs: In notch logic, check for intent before calling haiku
- Format: `jjb:HALLMARK:₢XXXXX:n: {intent}` (same prefix, user message)

## Slash Command Update

Update /jjc-pace-notch:
- Prompt opus for one-line intent summary
- Pass to jjx_notch --intent

## Backward Compatible

Omitting --intent preserves current haiku-generated messages.

### trophy-alpha-jjsa-session-marker (₢AFAA4) [complete]

**[260124-1741] complete**

Define S (Session) marker in JJSA for session start tracking.

## New Marker Type

Add to "Commit Message Patterns" section:

**S (Session)** — Records session start with model versions and environment
- Created automatically by saddle when 1+ hour gap detected in steeplechase
- Subject: `jjb:HALLMARK:₣XX:S: YYMMDD-HHMM session`
- Body contains model IDs and machine info

## Session Detection

A new session is detected when:
- jjx_saddle runs
- Most recent commit in steeplechase is >1 hour old (or no commits exist)

## Commit Format

```
jjb:HALLMARK:₣XX:S: 260124-1630 session

haiku: claude-3-5-haiku-20241022
sonnet: claude-sonnet-4-20250514
opus: claude-opus-4-5-20251101
host: macbook-pro.local
platform: darwin-arm64
```

## Model ID Collection

Uses ephemeral subagents created via --agents JSON flag:
- Minimal system prompt: "Report only your exact model ID string. Nothing else."
- No tools granted
- Token-cheap: ~10 token prompt, ~15 token response per probe

## Semantics

- Timestamp in subject (YYMMDD-HHMM) indicates session start time
- Model IDs are full strings (e.g., claude-opus-4-5-20251101)
- Host/platform provide machine context for multi-device workflows

## Files

- JJSA-GallopsData.adoc (add S marker to commit patterns)

**[260124-1643] bridled**

Define S (Session) marker in JJSA for session start tracking.

## New Marker Type

Add to "Commit Message Patterns" section:

**S (Session)** — Records session start with model versions and environment
- Created automatically by saddle when 1+ hour gap detected in steeplechase
- Subject: `jjb:HALLMARK:₣XX:S: YYMMDD-HHMM session`
- Body contains model IDs and machine info

## Session Detection

A new session is detected when:
- jjx_saddle runs
- Most recent commit in steeplechase is >1 hour old (or no commits exist)

## Commit Format

```
jjb:HALLMARK:₣XX:S: 260124-1630 session

haiku: claude-3-5-haiku-20241022
sonnet: claude-sonnet-4-20250514
opus: claude-opus-4-5-20251101
host: macbook-pro.local
platform: darwin-arm64
```

## Model ID Collection

Uses ephemeral subagents created via --agents JSON flag:
- Minimal system prompt: "Report only your exact model ID string. Nothing else."
- No tools granted
- Token-cheap: ~10 token prompt, ~15 token response per probe

## Semantics

- Timestamp in subject (YYMMDD-HHMM) indicates session start time
- Model IDs are full strings (e.g., claude-opus-4-5-20251101)
- Host/platform provide machine context for multi-device workflows

## Files

- JJSA-GallopsData.adoc (add S marker to commit patterns)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: JJSA-GallopsData.adoc (1 file) | Steps: 1. Read existing Commit Message Patterns section 2. Add S Session marker definition following existing marker format 3. Document session detection trigger 1 hour gap 4. Document commit body format with model IDs and host/platform | Verify: visual review of adoc formatting

**[260124-1640] rough**

Define S (Session) marker in JJSA for session start tracking.

## New Marker Type

Add to "Commit Message Patterns" section:

**S (Session)** — Records session start with model versions and environment
- Created automatically by saddle when 1+ hour gap detected in steeplechase
- Subject: `jjb:HALLMARK:₣XX:S: YYMMDD-HHMM session`
- Body contains model IDs and machine info

## Session Detection

A new session is detected when:
- jjx_saddle runs
- Most recent commit in steeplechase is >1 hour old (or no commits exist)

## Commit Format

```
jjb:HALLMARK:₣XX:S: 260124-1630 session

haiku: claude-3-5-haiku-20241022
sonnet: claude-sonnet-4-20250514
opus: claude-opus-4-5-20251101
host: macbook-pro.local
platform: darwin-arm64
```

## Model ID Collection

Uses ephemeral subagents created via --agents JSON flag:
- Minimal system prompt: "Report only your exact model ID string. Nothing else."
- No tools granted
- Token-cheap: ~10 token prompt, ~15 token response per probe

## Semantics

- Timestamp in subject (YYMMDD-HHMM) indicates session start time
- Model IDs are full strings (e.g., claude-opus-4-5-20251101)
- Host/platform provide machine context for multi-device workflows

## Files

- JJSA-GallopsData.adoc (add S marker to commit patterns)

**[260124-1207] rough**

Define V (Version) marker in JJSA for daily model version tracking.

## New Marker Type

Add to "Commit Message Patterns" section:

**V (Version)** — Records Claude model versions in use for the day
- Created automatically by saddle on first invocation of the day
- One per day per heat (or global?)
- Subject: `jjb:HALLMARK:₣XX:V: YYMMDD models`
- Body contains model ID strings for each tier

## Commit Format

```
jjb:HALLMARK:₣XX:V: 260124 models

haiku: claude-3-5-haiku-20241022
sonnet: claude-sonnet-4-20250514
opus: claude-opus-4-5-20251101
```

## Semantics

- Marker date in subject (YYMMDD) indicates which day's models
- Body uses simple `tier: model-id` format, one per line
- Model IDs are the full strings (e.g., claude-opus-4-5-20251101)
- Captured once per day, first saddle triggers collection

## Trophy Integration

- V markers included in steeplechase section
- Enables retrospective: "which model versions worked on this heat"

## Files

- JJSA-GallopsData.adoc (add V marker to commit patterns)

### trophy-alpha-session-probe (₢AFAA5) [complete]

**[260124-1755] complete**

Implement session detection and ephemeral agent probe spawning.

## Trigger

In jjx_saddle, before returning output:
1. Parse steeplechase for most recent commit timestamp
2. Compare to current time
3. If gap > 1 hour (or no commits): new session detected

## Session Start Flow

When new session detected:

1. **Spawn 3 ephemeral probe agents** via --agents JSON flag:
   ```json
   {
     "model-probe": {
       "description": "Report model ID",
       "prompt": "Report only your exact model ID string. Nothing else.",
       "tools": [],
       "model": "<tier>"
     }
   }
   ```
   Run for haiku, sonnet, opus (can be parallel)

2. **Collect machine info**:
   - Hostname (from environment or uname)
   - Platform (darwin-arm64, linux-x86_64, etc.)

3. **Create S marker commit**:
   - Use existing chalk infrastructure
   - Format per JJSA spec

## Implementation Options

**Option A: Rust subprocess**
- saddle shells out to spawn claude with --agents flag
- Parse JSON response from each probe
- Requires claude CLI available in PATH

**Option B: Slash command layer**
- saddle returns `needs_session_check: true` flag
- /jjc-heat-mount or /jjc-heat-groom handles probe spawning
- More natural for Task tool usage

Recommend Option B: keeps Rust code simpler, leverages Claude Code's native agent spawning.

## Files

- jjrq_query.rs (session gap detection logic)
- jjro_ops.rs (S marker creation)
- Slash commands (probe spawning if Option B)

## Depends on

trophy-alpha-jjsa-session-marker (spec must be defined first)

**[260124-1747] rough**

Implement session detection and ephemeral agent probe spawning.

## Trigger

In jjx_saddle, before returning output:
1. Parse steeplechase for most recent commit timestamp
2. Compare to current time
3. If gap > 1 hour (or no commits): new session detected

## Session Start Flow

When new session detected:

1. **Spawn 3 ephemeral probe agents** via --agents JSON flag:
   ```json
   {
     "model-probe": {
       "description": "Report model ID",
       "prompt": "Report only your exact model ID string. Nothing else.",
       "tools": [],
       "model": "<tier>"
     }
   }
   ```
   Run for haiku, sonnet, opus (can be parallel)

2. **Collect machine info**:
   - Hostname (from environment or uname)
   - Platform (darwin-arm64, linux-x86_64, etc.)

3. **Create S marker commit**:
   - Use existing chalk infrastructure
   - Format per JJSA spec

## Implementation Options

**Option A: Rust subprocess**
- saddle shells out to spawn claude with --agents flag
- Parse JSON response from each probe
- Requires claude CLI available in PATH

**Option B: Slash command layer**
- saddle returns `needs_session_check: true` flag
- /jjc-heat-mount or /jjc-heat-groom handles probe spawning
- More natural for Task tool usage

Recommend Option B: keeps Rust code simpler, leverages Claude Code's native agent spawning.

## Files

- jjrq_query.rs (session gap detection logic)
- jjro_ops.rs (S marker creation)
- Slash commands (probe spawning if Option B)

## Depends on

trophy-alpha-jjsa-session-marker (spec must be defined first)

**[260124-1641] rough**

Implement session detection and ephemeral agent probe spawning.

## Trigger

In jjx_saddle, before returning output:
1. Parse steeplechase for most recent commit timestamp
2. Compare to current time
3. If gap > 1 hour (or no commits): new session detected

## Session Start Flow

When new session detected:

1. **Spawn 3 ephemeral probe agents** via --agents JSON flag:
   ```json
   {
     "model-probe": {
       "description": "Report model ID",
       "prompt": "Report only your exact model ID string. Nothing else.",
       "tools": [],
       "model": "<tier>"
     }
   }
   ```
   Run for haiku, sonnet, opus (can be parallel)

2. **Collect machine info**:
   - Hostname (from environment or uname)
   - Platform (darwin-arm64, linux-x86_64, etc.)

3. **Create S marker commit**:
   - Use existing chalk infrastructure
   - Format per JJSA spec

## Implementation Options

**Option A: Rust subprocess**
- saddle shells out to spawn claude with --agents flag
- Parse JSON response from each probe
- Requires claude CLI available in PATH

**Option B: Slash command layer**
- saddle returns `needs_session_check: true` flag
- /jjc-heat-mount or /jjc-heat-groom handles probe spawning
- More natural for Task tool usage

Recommend Option B: keeps Rust code simpler, leverages Claude Code's native agent spawning.

## Files

- jjrq_query.rs (session gap detection logic)
- jjro_ops.rs (S marker creation)
- Slash commands (probe spawning if Option B)

## Depends on

trophy-alpha-jjsa-session-marker (spec must be defined first)

**[260124-1207] rough**

Implement automatic model version collection in saddle.

## Trigger

In jjx_saddle, before returning saddle output:
1. Check steeplechase for V marker with today's date
2. If found, skip collection
3. If not found, spawn 3 agents and create marker

## Collection Logic

Spawn 3 minimal Task agents (can be parallel):
- haiku agent: "Report your exact model ID string"
- sonnet agent: "Report your exact model ID string"  
- opus agent: "Report your exact model ID string"

Each agent returns its model ID (e.g., "claude-opus-4-5-20251101").

## Marker Creation

After collecting all 3 responses:
1. Format commit body with tier: model-id lines
2. Create V marker commit via existing chalk infrastructure
3. Use heat firemark from saddle context

## Implementation Notes

- Agent spawning from Rust: may need to shell out to vvx or use subprocess
- Or: saddle returns a "needs_version_check" flag and slash command handles spawning
- Fully autonomous: no user prompts

## Error Handling

- If any agent fails to respond, log warning but continue
- Partial results still useful (record what we got)

## Files

- jjro_ops.rs (saddle logic)
- Possibly jjrx_cli.rs (if spawning handled at CLI layer)

## Depends on

trophy-alpha-jjsa-version-marker (spec must be defined first)

### parallel-split-commands (₢AFABE) [complete]

**[260124-1810] complete**

Split jjx commands into per-command files using parallel haiku agents.

## Deliverable

Create per-command files following JJSA 2-letter code pattern:

| Code | Command | New file |
|------|---------|----------|
| CH | chalk | jjrch_chalk.rs |
| CU | curry | jjrcu_curry.rs |
| DR | draft | jjrdr_draft.rs |
| FU | furlough | jjrfu_furlough.rs |
| GL | garland | jjrgl_garland.rs |
| GC | get_coronets | jjrgc_get_coronets.rs |
| GS | get_spec | jjrgs_get_spec.rs |
| LD | landing | jjrld_landing.rs |
| MU | muster | jjrmu_muster.rs |
| NC | notch | jjrnc_notch.rs |
| NO | nominate | jjrno_nominate.rs |
| PD | parade | jjrpd_parade.rs |
| RL | rail | jjrrl_rail.rs |
| RN | rein | jjrrn_rein.rs |
| RS | restring | jjrrs_restring.rs |
| RT | retire | jjrrt_retire.rs |
| SD | saddle | jjrsd_saddle.rs |
| SC | scout | jjrsc_scout.rs |
| SL | slate | jjrsl_slate.rs |
| TL | tally | jjrtl_tally.rs |
| VL | validate | jjrvl_validate.rs |
| WP | wrap | jjrwp_wrap.rs |

## Per-agent task

Each haiku agent extracts ONE command:
1. Read jjrx_cli.rs - find Args struct for this command
2. Read jjro_ops.rs or jjrq_query.rs - find handler function
3. Create jjrxx_command.rs with:
   - Args struct
   - Handler function  
   - Necessary imports from jjrg_gallops, jjru_util, etc.
4. Mark handler as pub

## Parallel execution

22 agents, each creates one file. No file conflicts - all write to different destinations.

## Expected state after

- 22 new jjrxx_*.rs files exist
- Old aggregate files unchanged (read-only during this pace)
- Build will NOT pass yet - dispatch not rewired

**[260124-1805] bridled**

Split jjx commands into per-command files using parallel haiku agents.

## Deliverable

Create per-command files following JJSA 2-letter code pattern:

| Code | Command | New file |
|------|---------|----------|
| CH | chalk | jjrch_chalk.rs |
| CU | curry | jjrcu_curry.rs |
| DR | draft | jjrdr_draft.rs |
| FU | furlough | jjrfu_furlough.rs |
| GL | garland | jjrgl_garland.rs |
| GC | get_coronets | jjrgc_get_coronets.rs |
| GS | get_spec | jjrgs_get_spec.rs |
| LD | landing | jjrld_landing.rs |
| MU | muster | jjrmu_muster.rs |
| NC | notch | jjrnc_notch.rs |
| NO | nominate | jjrno_nominate.rs |
| PD | parade | jjrpd_parade.rs |
| RL | rail | jjrrl_rail.rs |
| RN | rein | jjrrn_rein.rs |
| RS | restring | jjrrs_restring.rs |
| RT | retire | jjrrt_retire.rs |
| SD | saddle | jjrsd_saddle.rs |
| SC | scout | jjrsc_scout.rs |
| SL | slate | jjrsl_slate.rs |
| TL | tally | jjrtl_tally.rs |
| VL | validate | jjrvl_validate.rs |
| WP | wrap | jjrwp_wrap.rs |

## Per-agent task

Each haiku agent extracts ONE command:
1. Read jjrx_cli.rs - find Args struct for this command
2. Read jjro_ops.rs or jjrq_query.rs - find handler function
3. Create jjrxx_command.rs with:
   - Args struct
   - Handler function  
   - Necessary imports from jjrg_gallops, jjru_util, etc.
4. Mark handler as pub

## Parallel execution

22 agents, each creates one file. No file conflicts - all write to different destinations.

## Expected state after

- 22 new jjrxx_*.rs files exist
- Old aggregate files unchanged (read-only during this pace)
- Build will NOT pass yet - dispatch not rewired

*Direction:* Agent: haiku | Cardinality: 22 parallel | Files: Create jjrch_chalk.rs jjrcu_curry.rs jjrdr_draft.rs jjrfu_furlough.rs jjrgl_garland.rs jjrgc_get_coronets.rs jjrgs_get_spec.rs jjrld_landing.rs jjrmu_muster.rs jjrnc_notch.rs jjrno_nominate.rs jjrpd_parade.rs jjrrl_rail.rs jjrrn_rein.rs jjrrs_restring.rs jjrrt_retire.rs jjrsd_saddle.rs jjrsc_scout.rs jjrsl_slate.rs jjrtl_tally.rs jjrvl_validate.rs jjrwp_wrap.rs in src dir | Steps: 1. Each agent reads jjrx_cli.rs to find its command Args struct and handler function 2. Extracts to new jjrxx_command.rs with necessary imports from jjrg_gallops jjru_util jjrf_favor etc 3. Does NOT modify lib.rs - files are staged only 4. Adds copyright header | Verify: All 22 files exist in Tools/jjk/vov_veiled/src | Rollback: git reset --hard a7f133ec

**[260124-1800] rough**

Split jjx commands into per-command files using parallel haiku agents.

## Deliverable

Create per-command files following JJSA 2-letter code pattern:

| Code | Command | New file |
|------|---------|----------|
| CH | chalk | jjrch_chalk.rs |
| CU | curry | jjrcu_curry.rs |
| DR | draft | jjrdr_draft.rs |
| FU | furlough | jjrfu_furlough.rs |
| GL | garland | jjrgl_garland.rs |
| GC | get_coronets | jjrgc_get_coronets.rs |
| GS | get_spec | jjrgs_get_spec.rs |
| LD | landing | jjrld_landing.rs |
| MU | muster | jjrmu_muster.rs |
| NC | notch | jjrnc_notch.rs |
| NO | nominate | jjrno_nominate.rs |
| PD | parade | jjrpd_parade.rs |
| RL | rail | jjrrl_rail.rs |
| RN | rein | jjrrn_rein.rs |
| RS | restring | jjrrs_restring.rs |
| RT | retire | jjrrt_retire.rs |
| SD | saddle | jjrsd_saddle.rs |
| SC | scout | jjrsc_scout.rs |
| SL | slate | jjrsl_slate.rs |
| TL | tally | jjrtl_tally.rs |
| VL | validate | jjrvl_validate.rs |
| WP | wrap | jjrwp_wrap.rs |

## Per-agent task

Each haiku agent extracts ONE command:
1. Read jjrx_cli.rs - find Args struct for this command
2. Read jjro_ops.rs or jjrq_query.rs - find handler function
3. Create jjrxx_command.rs with:
   - Args struct
   - Handler function  
   - Necessary imports from jjrg_gallops, jjru_util, etc.
4. Mark handler as pub

## Parallel execution

22 agents, each creates one file. No file conflicts - all write to different destinations.

## Expected state after

- 22 new jjrxx_*.rs files exist
- Old aggregate files unchanged (read-only during this pace)
- Build will NOT pass yet - dispatch not rewired

### rewire-command-dispatch (₢AFABF) [complete]

**[260124-1817] complete**

Rewire CLI dispatch to use new per-command files.

## Depends on

parallel-split-commands (₢ assigned at slate time)

## Deliverable

Update lib.rs and CLI dispatch:
1. Add `pub mod jjrxx_command;` for each new file
2. Update Subcommand enum to import Args from new locations
3. Update match dispatch to call handlers from new locations
4. Build passes
5. Tests pass

## Sequential

This pace touches shared files (lib.rs, jjrx_cli.rs) - must be single agent.

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

All 135 existing tests must pass.

**[260124-1800] rough**

Rewire CLI dispatch to use new per-command files.

## Depends on

parallel-split-commands (₢ assigned at slate time)

## Deliverable

Update lib.rs and CLI dispatch:
1. Add `pub mod jjrxx_command;` for each new file
2. Update Subcommand enum to import Args from new locations
3. Update match dispatch to call handlers from new locations
4. Build passes
5. Tests pass

## Sequential

This pace touches shared files (lib.rs, jjrx_cli.rs) - must be single agent.

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

All 135 existing tests must pass.

### trim-cli-dead-code (₢AFABG) [complete]

**[260124-1827] complete**

Remove dead code from CLI modules after rewire.

## Context

The rewire-command-dispatch pace moved Args and handlers to per-command modules (jjrxx_*.rs). However:
- jjro_ops.rs contains active operations used via jjrg_gallops re-exports
- jjrq_query.rs contains query functions still used by jjrsd_saddle.rs
- These files CANNOT be deleted

## Deliverable

1. Remove unused re-exports from lib.rs:
   - Check if `jjrq_MusterArgs`, `jjrq_SaddleArgs`, `jjrq_ParadeArgs` are used externally
   - If not, remove the `pub use jjrq_query::...` line

2. Check jjrx_cli.rs for any remaining dead code patterns

3. Run `cargo fix --lib -p jjk` to auto-fix unused import warnings

## NOT in scope

- jjro_ops.rs stays (active operations)
- jjrq_query.rs stays (active queries)
- jjrg_gallops.rs stays (re-export facade)

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

Clean build with no warnings, all tests pass.

**[260124-1822] bridled**

Remove dead code from CLI modules after rewire.

## Context

The rewire-command-dispatch pace moved Args and handlers to per-command modules (jjrxx_*.rs). However:
- jjro_ops.rs contains active operations used via jjrg_gallops re-exports
- jjrq_query.rs contains query functions still used by jjrsd_saddle.rs
- These files CANNOT be deleted

## Deliverable

1. Remove unused re-exports from lib.rs:
   - Check if `jjrq_MusterArgs`, `jjrq_SaddleArgs`, `jjrq_ParadeArgs` are used externally
   - If not, remove the `pub use jjrq_query::...` line

2. Check jjrx_cli.rs for any remaining dead code patterns

3. Run `cargo fix --lib -p jjk` to auto-fix unused import warnings

## NOT in scope

- jjro_ops.rs stays (active operations)
- jjrq_query.rs stays (active queries)
- jjrg_gallops.rs stays (re-export facade)

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

Clean build with no warnings, all tests pass.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: lib.rs (1 file, plus cargo fix touches multiple) | Steps: 1. Remove line 75 pub use jjrq_query from lib.rs 2. Run cargo fix --lib -p jjk --manifest-path Tools/vok/Cargo.toml --allow-dirty to auto-fix unused imports 3. Run tt/vow-b.Build.sh and tt/vow-t.Test.sh | Verify: tt/vow-b.Build.sh with zero warnings

**[260124-1820] rough**

Remove dead code from CLI modules after rewire.

## Context

The rewire-command-dispatch pace moved Args and handlers to per-command modules (jjrxx_*.rs). However:
- jjro_ops.rs contains active operations used via jjrg_gallops re-exports
- jjrq_query.rs contains query functions still used by jjrsd_saddle.rs
- These files CANNOT be deleted

## Deliverable

1. Remove unused re-exports from lib.rs:
   - Check if `jjrq_MusterArgs`, `jjrq_SaddleArgs`, `jjrq_ParadeArgs` are used externally
   - If not, remove the `pub use jjrq_query::...` line

2. Check jjrx_cli.rs for any remaining dead code patterns

3. Run `cargo fix --lib -p jjk` to auto-fix unused import warnings

## NOT in scope

- jjro_ops.rs stays (active operations)
- jjrq_query.rs stays (active queries)
- jjrg_gallops.rs stays (re-export facade)

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

Clean build with no warnings, all tests pass.

**[260124-1801] rough**

Delete old aggregate files after successful rewire.

## Depends on

rewire-command-dispatch (build + tests passing)

## Deliverable

Remove obsolete files:
- jjrx_cli.rs (CLI parsing - now distributed)
- jjro_ops.rs (operations - now distributed)
- jjrq_query.rs (queries - now distributed)

## Keep

Infrastructure files remain:
- jjrg_gallops.rs (data structures)
- jjru_util.rs (shared utilities)
- jjrf_favor.rs (encoding)
- jjri_io.rs (file I/O)
- jjrn_notch.rs (commit formatting - shared)
- jjrs_steeplechase.rs (log parsing - shared)
- jjrp_print.rs (display utilities)
- jjrt_types.rs (type definitions)
- jjrv_validate.rs (validation - may stay shared or move)

## Verification

```
tt/vow-b.Build.sh && tt/vow-t.Test.sh
```

Clean build, all tests pass, no dead code warnings.

### audit-module-overlap (₢AFABI) [complete]

**[260124-1831] complete**

Audit JJK module structure for overlapping implementations after CLI rewire.

## Context

The rewire-command-dispatch work moved Args and handlers to per-command modules (jjrxx_*.rs), but the old aggregate files still exist with potentially duplicated code.

## Audit checklist

1. **jjrq_query.rs** — Map what's live vs dead:
   - jjrq_resolve_default_heat (USED by jjrsd_saddle.rs)
   - jjrq_MusterArgs, jjrq_run_muster (DEAD - replaced by jjrmu_*)
   - jjrq_SaddleArgs, jjrq_run_saddle (DEAD - replaced by jjrsd_*)
   - jjrq_ParadeArgs, jjrq_run_parade (DEAD - replaced by jjrpd_*)
   - Other helper functions?

2. **jjro_ops.rs** — Map what's live vs dead:
   - Operations called via jjrg_gallops.rs re-exports
   - Any functions that could move to per-command modules?

3. **jjrg_gallops.rs** — Understand the re-export facade:
   - Is this indirection necessary?
   - Could per-command modules import directly?

## Deliverable

Document findings and slate follow-up paces:
- prune-dead-query-code (remove dead jjrq_* functions)
- Any other cleanup needed for "one file per operation"

## NOT in scope

Actual code changes - this is analysis only.

**[260124-1823] rough**

Audit JJK module structure for overlapping implementations after CLI rewire.

## Context

The rewire-command-dispatch work moved Args and handlers to per-command modules (jjrxx_*.rs), but the old aggregate files still exist with potentially duplicated code.

## Audit checklist

1. **jjrq_query.rs** — Map what's live vs dead:
   - jjrq_resolve_default_heat (USED by jjrsd_saddle.rs)
   - jjrq_MusterArgs, jjrq_run_muster (DEAD - replaced by jjrmu_*)
   - jjrq_SaddleArgs, jjrq_run_saddle (DEAD - replaced by jjrsd_*)
   - jjrq_ParadeArgs, jjrq_run_parade (DEAD - replaced by jjrpd_*)
   - Other helper functions?

2. **jjro_ops.rs** — Map what's live vs dead:
   - Operations called via jjrg_gallops.rs re-exports
   - Any functions that could move to per-command modules?

3. **jjrg_gallops.rs** — Understand the re-export facade:
   - Is this indirection necessary?
   - Could per-command modules import directly?

## Deliverable

Document findings and slate follow-up paces:
- prune-dead-query-code (remove dead jjrq_* functions)
- Any other cleanup needed for "one file per operation"

## NOT in scope

Actual code changes - this is analysis only.

### implement-heat-braid (₢AFAAM) [complete]

**[260124-1807] complete**

Implement `/jjc-heat-braid` slash command based on completed design.

## Deliverable

Create `.claude/commands/jjc-heat-braid.md` implementing the braid ceremony.

## Design Summary (from prior iterations)

**Core model:** Intersection, not migration. Braid finds overlapping paces between two heats and consolidates them. Both heats may continue with remaining work.

**CLI:** `/jjc-heat-braid <firemark-A> <firemark-B>` (order does not imply direction)

**Pace classifications:**
- Already done: steeplechase shows completion → abandon with citation
- Overlap: semantic match in other heat → reslate keeper, abandon other
- Distinct: fits its heat, no overlap → leave in place
- Soggy: ill-formed spec → flag for human decision

**Tiered architecture:**
- Haiku pass: correlate paces across heats + check steeplechase for completion evidence
- Opus pass: classify paces, recommend outcomes, orchestrate ceremony

**Ceremony flow:**
1. Load both heats, assess gestalt compatibility
2. Haiku correlation pass
3. Opus classifies each pace
4. Present summary (overlaps, soggy, already-done counts)
5. Walk through each finding with user approval
6. Paddock maintenance via jjx_curry --level
7. Offer silks rename if gestalt shifted
8. If heat now empty, offer to retire
9. Summary with chalk references

**Primitives (all exist):**
- jjx_tally --state abandoned (abandon paces)
- jjx_tally with stdin (reslate paces)
- jjx_rein (steeplechase queries)
- jjx_parade --full (full pace specs)
- jjx_curry --level (paddock maintenance)

**Edge cases:** No intersection → report and exit. All soggy → recommend grooming. One heat empty → offer retire. Same pace matches multiple → present as group.

## Implementation Pattern

Follow existing `/jjc-heat-*` slash commands for structure. Use Task tool with haiku subagent for correlation pass, then orchestrate ceremony interactively.

## Files

.claude/commands/jjc-heat-braid.md

**[260124-1737] bridled**

Implement `/jjc-heat-braid` slash command based on completed design.

## Deliverable

Create `.claude/commands/jjc-heat-braid.md` implementing the braid ceremony.

## Design Summary (from prior iterations)

**Core model:** Intersection, not migration. Braid finds overlapping paces between two heats and consolidates them. Both heats may continue with remaining work.

**CLI:** `/jjc-heat-braid <firemark-A> <firemark-B>` (order does not imply direction)

**Pace classifications:**
- Already done: steeplechase shows completion → abandon with citation
- Overlap: semantic match in other heat → reslate keeper, abandon other
- Distinct: fits its heat, no overlap → leave in place
- Soggy: ill-formed spec → flag for human decision

**Tiered architecture:**
- Haiku pass: correlate paces across heats + check steeplechase for completion evidence
- Opus pass: classify paces, recommend outcomes, orchestrate ceremony

**Ceremony flow:**
1. Load both heats, assess gestalt compatibility
2. Haiku correlation pass
3. Opus classifies each pace
4. Present summary (overlaps, soggy, already-done counts)
5. Walk through each finding with user approval
6. Paddock maintenance via jjx_curry --level
7. Offer silks rename if gestalt shifted
8. If heat now empty, offer to retire
9. Summary with chalk references

**Primitives (all exist):**
- jjx_tally --state abandoned (abandon paces)
- jjx_tally with stdin (reslate paces)
- jjx_rein (steeplechase queries)
- jjx_parade --full (full pace specs)
- jjx_curry --level (paddock maintenance)

**Edge cases:** No intersection → report and exit. All soggy → recommend grooming. One heat empty → offer retire. Same pace matches multiple → present as group.

## Implementation Pattern

Follow existing `/jjc-heat-*` slash commands for structure. Use Task tool with haiku subagent for correlation pass, then orchestrate ceremony interactively.

## Files

.claude/commands/jjc-heat-braid.md

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: .claude/commands/jjc-heat-braid.md (1 file) | Steps: 1. Read existing jjc-heat-restring.md and jjc-heat-groom.md for slash command patterns 2. Create jjc-heat-braid.md implementing ceremony flow with Haiku correlation pass and Opus judgment pass 3. Include argument parsing, gestalt compatibility check, per-pace walkthrough, paddock maintenance via jjx_curry, and retire offer | Verify: Review command structure matches existing patterns

**[260124-1737] rough**

Implement `/jjc-heat-braid` slash command based on completed design.

## Deliverable

Create `.claude/commands/jjc-heat-braid.md` implementing the braid ceremony.

## Design Summary (from prior iterations)

**Core model:** Intersection, not migration. Braid finds overlapping paces between two heats and consolidates them. Both heats may continue with remaining work.

**CLI:** `/jjc-heat-braid <firemark-A> <firemark-B>` (order does not imply direction)

**Pace classifications:**
- Already done: steeplechase shows completion → abandon with citation
- Overlap: semantic match in other heat → reslate keeper, abandon other
- Distinct: fits its heat, no overlap → leave in place
- Soggy: ill-formed spec → flag for human decision

**Tiered architecture:**
- Haiku pass: correlate paces across heats + check steeplechase for completion evidence
- Opus pass: classify paces, recommend outcomes, orchestrate ceremony

**Ceremony flow:**
1. Load both heats, assess gestalt compatibility
2. Haiku correlation pass
3. Opus classifies each pace
4. Present summary (overlaps, soggy, already-done counts)
5. Walk through each finding with user approval
6. Paddock maintenance via jjx_curry --level
7. Offer silks rename if gestalt shifted
8. If heat now empty, offer to retire
9. Summary with chalk references

**Primitives (all exist):**
- jjx_tally --state abandoned (abandon paces)
- jjx_tally with stdin (reslate paces)
- jjx_rein (steeplechase queries)
- jjx_parade --full (full pace specs)
- jjx_curry --level (paddock maintenance)

**Edge cases:** No intersection → report and exit. All soggy → recommend grooming. One heat empty → offer retire. Same pace matches multiple → present as group.

## Implementation Pattern

Follow existing `/jjc-heat-*` slash commands for structure. Use Task tool with haiku subagent for correlation pass, then orchestrate ceremony interactively.

## Files

.claude/commands/jjc-heat-braid.md

**[260124-0847] rough**

Design the `/jjc-heat-braid` ceremony for consolidating overlapping paces across heats.

**Depends on:** ₢AFAAd (jjx_curry primitive)

## Core Model: Intersection, Not Migration

Braid finds the **intersection** between two heats — paces with overlapping intent — and consolidates them. This is symmetric analysis with asymmetric action:

- Analyze both heats equally to find overlapping paces
- Consolidate overlaps into one heat (user chooses which keeps the merged pace)
- Both heats may continue with their remaining non-overlapping work

**Key insight:** Two legitimate initiatives can share some paces. Braid untangles the overlap without destroying either initiative. Unlike migration ("subsume stale into fresh"), braid respects that both heats may represent valid ongoing work.

**Key constraint:** Braid should result in fewer total paces. If no overlap exists, braid reports "no intersection found" and exits. For wholesale pace movement without consolidation, use restring instead.

## Real Scenario

Day 1: Create heat A, slate paces, make progress.
Day 2: Fresh inspiration → create heat B with some paces that overlap A's intent.
Now: Two heats with partial duplication.

Braid finds the intersection, consolidates it into one heat, and leaves both heats with their non-overlapping work intact.

## Pace Classifications

For each pace in both heats, Opus assigns one classification:

| Classification | Evidence | Recommendation |
|----------------|----------|----------------|
| **Already done** | Steeplechase shows completion elsewhere | Abandon with citation |
| **Overlap** | Semantic match with pace in other heat | Consolidate: enrich one, abandon other |
| **Distinct** | Fits its heat's gestalt, no overlap | Leave in place |
| **Soggy** | Ill-formed spec, vague intent | Flag for human decision |

### Soggy Detection Heuristics

Soggy paces are ill-formed — slated in a burst of enthusiasm without proper specification:

- Very short spec lacking detail
- Vague language: "improve", "look into", "consider", "maybe", "explore"
- No clear deliverable or acceptance criteria
- Aspirational positioning (often late in queue)
- Never refined (single steeplechase entry)
- Unbridleable due to ambiguity, not complexity

Opus flags soggy paces for human decision: abandon, or flesh out properly via reslate.

## Tiered Agent Architecture

**Haiku pass** (retrieval/correlation):
- Batch all paces from both heats + steeplechase history
- Find textual/semantic similarities between pace specs
- Find evidence of completion in steeplechase
- Output: correlation matrix (pace pairs with similarity scores) + completion evidence

**Opus pass** (judgment/ceremony):
- Reviews Haiku's correlations
- Applies soggy heuristics
- Assigns classifications with reasoning
- Orchestrates interactive ceremony

## Ceremony Flow

```
1. User: /jjc-heat-braid <firemark-A> <firemark-B>

2. Load both heats
   - Read paddocks, assess gestalt compatibility
   - If completely divergent: "These heats have no apparent overlap. Continue anyway?"

3. Haiku pass
   - Correlate all actionable paces across both heats
   - Check steeplechase for completion evidence

4. Opus classifies each pace
   - Already done / Overlap / Distinct / Soggy
   - For overlaps: identify the pair and recommend which heat keeps merged pace

5. Present summary
   - "Found N overlapping pace pairs, M soggy paces, K already-done"
   - "Heats have X and Y distinct paces respectively"

6. Walk through each finding (user approves/overrides):

   For overlaps:
   - Show both pace specs side by side
   - "Recommend consolidating into ₣B. Proceed?"
   - If yes: reslate keeper with enriched spec, abandon other

   For already-done:
   - Show pace + steeplechase evidence
   - "This appears complete. Abandon with citation?"

   For soggy:
   - Show pace + reasoning
   - "This pace looks underspecified. Abandon, or flesh out now?"
   - If flesh out: interactive reslate

   Distinct paces: no action needed, just noted in summary

7. Paddock maintenance (if any consolidation occurred)
   - For heat that received merged paces: jjx_curry --level
   - Opus drafts paddock additions for absorbed context
   - User approves paddock changes

8. Summary
   - Paces consolidated: N
   - Paces abandoned (done): M
   - Paces abandoned (soggy): K
   - Both heats continue with remaining work (or offer retire if one is now empty)
```

## CLI Signature

```
/jjc-heat-braid <firemark-A> <firemark-B>
```

Order doesn't imply direction. Braid analyzes the intersection; user decides per-overlap which heat keeps the merged pace.

## Primitives Required

**Existing:**
- `jjx_tally --state abandoned` — abandon paces
- `jjx_tally` with stdin — reslate paces
- `jjx_rein` — steeplechase queries
- `jjx_parade --full` — full pace specs

**New (₢AFAAd):**
- `jjx_curry <firemark> --level [--note]` — paddock maintenance

## Edge Cases

- **No intersection found:** Report and exit gracefully
- **All paces soggy:** Surface this as a signal the heat needs grooming, not braiding
- **One heat empty after braid:** Offer to retire
- **Same pace matches multiple:** Present as group, user decides consolidation target
- **Circular overlap (A₁↔B₁, A₂↔B₂ but A₁ relates to B₂):** Haiku surfaces the cluster, Opus presents holistically

## Slash Command Implementation Notes

When implementing `/jjc-heat-braid`, enshrine these principles:

1. **Intersection, not migration:** Never assume one heat is "primary" or "destination." Both heats are peers being analyzed for overlap.

2. **Conservative on abandonment:** Only abandon with positive evidence (steeplechase completion) or explicit user confirmation (soggy). "No match" does NOT mean "abandon."

3. **Human decides consolidation target:** For each overlap, Opus recommends but user chooses which heat keeps the merged pace.

4. **Soggy is a service:** Surfacing ill-formed paces is a cleanup opportunity, not a judgment. Offer to flesh out, not just discard.

5. **Both heats may continue:** The default outcome is two healthier heats, not one merged heat. Full consolidation is a special case, not the goal.

## Deliverable

Slash command `/jjc-heat-braid` implementing this ceremony with Haiku correlation pass and Opus judgment pass.

**[260123-2006] rough**

Design the `/jjc-heat-braid` ceremony for consolidating paces across heats.

**Depends on:** ₢AFAAd (jjx_curry primitive)

## Concept

Braid analyzes two heats (source → destination) and recommends how to consolidate similar/overlapping paces. Key gestalt: braid must result in pace reduction, otherwise use restring.

## Real Scenario

Day 1: Create heat, work on it, make progress.
Day 2: Fresh inspiration → create new heat with new paces, some overlapping day 1 work.
Now: Two heats with duplicated intent, stale paddock in old heat, fresher context in new.

Braid cleans up this mess by consolidating the stale heat into the fresh one.

## Resolved Decisions

- **Direction:** stale → fresh (newer heat has better paddock/framing)
- **Scope:** Multi-pace analysis (user does not know which paces overlap)
- **Match detection:** Opus semantic analysis of pace specs
- **Paddock levelling:** Opus picks relevant parts from both paddocks
- **Silks proposals:** Only if Opus judges gestalt shifted
- **Consolidation method:** Reslate destination pace (enrich spec), abandon source pace
- **Interactive:** Walk through recommendations, user approves each
- **No-match handling:** Inform user of incongruity, no forced action
- **No auto-retire:** Don't automatically retire source heat (but offer if empty)

## Tiered Agent Architecture

**Haiku pass** (retrieval/correlation):
- Batch all source paces + steeplechase history (all heats) in one call
- Find evidence that source paces might already be done elsewhere
- Output: per-pace list of potentially matching commits/entries

**Opus pass** (judgment/ceremony):
- Reviews Haiku's correlation evidence
- Makes consolidate/move/abandon recommendations
- Orchestrates the interactive ceremony

## Per-Pace Outcomes

1. **Already done elsewhere:** Evidence in steeplechase → abandon with citation
2. **Match in destination:** Similar pace exists → reslate destination, abandon source
3. **No match, still relevant:** → move wholesale via draft
4. **No match, obsolete:** → abandon in source

## Ceremony Flow

```
1. User: /jjc-heat-braid <stale-firemark> <fresh-firemark>
2. Opus reads both paddocks, assesses consonance
   - If divergent: "These look like different initiatives. Continue?"
3. Haiku pass: correlate all source paces against steeplechase history
4. For each source pace:
   - Present Haiku's evidence (if any)
   - Opus recommends outcome
   - User approves or overrides
5. Paddock maintenance:
   - Opus drafts merged paddock for destination
   - Shows diff, user approves
   - jjx_curry <fresh> --level --note "from ₣<stale>"
   - If source has remaining paces: jjx_curry <stale> --muck
6. Silks proposals (if Opus judges gestalt shifted)
7. If source heat now empty: offer to retire
8. Summary with chalk references
```

## Primitives

**Existing (sufficient):**
- `jjx_tally --state abandoned` — abandon source paces
- `jjx_tally` with stdin — reslate destination paces
- `jjx_draft` — move paces wholesale
- `jjx_furlough --silks` — rename heat silks
- `jjx_rein` — steeplechase queries (loop per heat)

**New primitive (₢AFAAd):**
- `jjx_curry <firemark> --level|--muck [--note]` — paddock maintenance with chalk

## CLI Signature

```
/jjc-heat-braid <stale-firemark> <fresh-firemark>
```

## Deliverable

Design document specifying:
- Paddock maintenance approach using jjx_curry
- Interactive workflow with Opus prompts
- Error cases and edge conditions

**[260123-1325] rough**

Design the `/jjc-heat-braid` ceremony for consolidating paces across heats.

## Concept

Braid analyzes two heats (source → destination) and recommends how to consolidate similar/overlapping paces. Key gestalt: braid must result in pace reduction, otherwise use restring.

## Real Scenario

Day 1: Create heat, work on it, make progress.
Day 2: Fresh inspiration → create new heat with new paces, some overlapping day 1's work.
Now: Two heats with duplicated intent, stale paddock in old heat, fresher context in new.

Braid cleans up this mess by consolidating the stale heat into the fresh one.

## Established Decisions

- **Direction:** stale → fresh (newer heat has better paddock/framing)
- **Scope:** Multi-pace analysis (user doesn't know which paces overlap)
- **Consolidation method:** Reslate destination pace (enrich spec), abandon source pace
- **Interactive:** Walk through recommendations, user approves each
- **No-match handling:** Inform user of incongruity, no forced action
- **No auto-retire:** Don't automatically retire source heat (but offer if empty)

## Tiered Agent Architecture

**Haiku pass** (retrieval/correlation):
- Batch all source paces + steeplechase history (all heats) in one call
- Find evidence that source paces might already be done elsewhere
- Output: per-pace list of potentially matching commits/entries

**Opus pass** (judgment/ceremony):
- Reviews Haiku's correlation evidence
- Makes consolidate/move/abandon recommendations
- Orchestrates the interactive ceremony

## Per-Pace Outcomes

1. **Already done elsewhere:** Evidence in steeplechase → abandon with citation
2. **Match in destination:** Similar pace exists → reslate destination, abandon source
3. **No match, still relevant:** → move wholesale via draft
4. **No match, obsolete:** → abandon in source

## Ceremony Flow

```
1. User: /jjc-heat-braid <stale-firemark> <fresh-firemark>
2. Opus reads both paddocks, assesses consonance
   - If divergent: "These look like different initiatives. Continue?"
3. Haiku pass: correlate all source paces against history
4. For each source pace:
   - Present Haiku's evidence (if any)
   - Opus recommends outcome
   - User approves or overrides
5. Ceremony ending:
   - Paddock levelling for destination (absorb relevant context)
   - Paddock trim for source (if paces remain)
   - Silks proposals for BOTH heats
6. If source heat now empty: offer to retire
7. Summary
```

## Primitives

**Existing (sufficient):**
- `jjx_tally --state abandoned` — abandon source paces
- `jjx_tally` with stdin — reslate destination paces
- `jjx_draft` — move paces wholesale
- `jjx_furlough --silks` — rename heat silks
- `jjx_rein` — steeplechase queries (loop per heat)

**New primitive needed:**
- `jjx_scribe <firemark>` — update paddock content from stdin

## CLI Signature

```
/jjc-heat-braid <stale-firemark> <fresh-firemark>
```

## Open Questions

1. How to detect "match in destination"? Opus semantic analysis, or user hints?
2. Paddock levelling: full replacement or additive merge?
3. Silks proposals: mandatory offer or only if gestalt shifted?

## Deliverable

Design document specifying:
- Final answers to open questions
- Paddock maintenance approach
- Interactive workflow with opus prompts
- Error cases and edge conditions

**[260123-0850] rough**

Design the `jjc-heat-braid` operation for consolidating paces across heats.

## Concept

Braid analyzes two heats (source → destination) and recommends how to consolidate similar/overlapping paces. Key gestalt: braid must result in pace reduction, otherwise use restring.

## Established Decisions

- **Agent tier:** Opus (requires judgment on paddock consonance)
- **Inputs:** Two heats — source and destination
- **Consolidation method:** Reslate destination pace (enrich spec), abandon source pace
- **Interactive:** Walk through recommendations, user approves each
- **No-match handling:** Inform user of incongruity, no forced action
- **No auto-retire:** Don't automatically retire source heat

## Open Questions

### 1. Single-pace vs multi-pace input
Should braid:
- (A) Take two heats and analyze ALL actionable paces for consolidation opportunities?
- (B) Take a specific source pace and find its best destination match?

Option B might be simpler — user identifies candidate, braid finds the match and executes.

### 2. Relationship to restring
Braid needs paddock maintenance like restring does. Options:
- (A) Braid calls restring internally for wholesale moves
- (B) Braid duplicates restring logic (problematic — we've found this causes drift)
- (C) Extract shared paddock-maintenance into lower-level utility that both use

Need to audit restring's paddock handling before deciding.

### 3. Workflow sketch (pending Q1 resolution)

If multi-pace (A):
```
1. Compare paddocks → assess consonance
2. If low consonance → warn, offer abort
3. For each source pace:
   - Find destination matches (or none)
   - Present: braid into X / move wholesale / no match
   - User approves → execute
4. Summary
```

If single-pace (B):
```
1. User specifies source pace
2. Opus analyzes destination heat for best match
3. Present recommendation with rationale
4. User approves → reslate + abandon
```

### 4. CLI signature (pending Q1)

Multi-pace: `jjc-heat-braid <source-firemark> <dest-firemark>`
Single-pace: `jjc-heat-braid <source-coronet> <dest-firemark>`

## Deliverable

Design document specifying:
- Final answers to open questions
- CLI signature and argument handling
- Paddock maintenance approach (and restring relationship)
- Interactive workflow with opus prompts
- Error cases and edge conditions

### remove-verbose-lock-messages (₢AFAA6) [complete]

**[260124-1728] complete**

Remove verbose lock acquired/released messages from VVC, VOK, and JJK.

## Rationale

These messages consume tokens and provide no value in the success path. The error case ("Another commit in progress - lock held") already explains the context.

## Changes

1. **Tools/vvc/src/vvcc_commit.rs**
   - Delete line 121: `eprintln!("commit: lock acquired");`
   - Delete line 133: `eprintln!("commit: lock released");`

2. **Tools/vok/src/vorm_main.rs**
   - Delete line 305: `eprintln!("push: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

3. **Tools/jjk/vov_veiled/src/jjrx_cli.rs**
   - Delete line 1256: `eprintln!("jjx_tally: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260124-1719] bridled**

Remove verbose lock acquired/released messages from VVC, VOK, and JJK.

## Rationale

These messages consume tokens and provide no value in the success path. The error case ("Another commit in progress - lock held") already explains the context.

## Changes

1. **Tools/vvc/src/vvcc_commit.rs**
   - Delete line 121: `eprintln!("commit: lock acquired");`
   - Delete line 133: `eprintln!("commit: lock released");`

2. **Tools/vok/src/vorm_main.rs**
   - Delete line 305: `eprintln!("push: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

3. **Tools/jjk/vov_veiled/src/jjrx_cli.rs**
   - Delete line 1256: `eprintln!("jjx_tally: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: vvcc_commit.rs, vorm_main.rs, jjrx_cli.rs (3 files) | Steps: 1. Delete lock acquired and lock released eprintln calls in vvcc_commit.rs 2. Delete lock acquired eprintln in vorm_main.rs and any lock released if present 3. Delete lock acquired eprintln in jjrx_cli.rs and any lock released if present | Verify: tt/vow-b.Build.sh

**[260124-1718] rough**

Remove verbose lock acquired/released messages from VVC, VOK, and JJK.

## Rationale

These messages consume tokens and provide no value in the success path. The error case ("Another commit in progress - lock held") already explains the context.

## Changes

1. **Tools/vvc/src/vvcc_commit.rs**
   - Delete line 121: `eprintln!("commit: lock acquired");`
   - Delete line 133: `eprintln!("commit: lock released");`

2. **Tools/vok/src/vorm_main.rs**
   - Delete line 305: `eprintln!("push: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

3. **Tools/jjk/vov_veiled/src/jjrx_cli.rs**
   - Delete line 1256: `eprintln!("jjx_tally: lock acquired");`
   - Check for corresponding "lock released" message and delete if present

## Verify

tt/vow-b.Build.sh && tt/vow-t.Test.sh

### test-jjx-curry (₢AFAA8) [complete]

**[260124-1839] complete**

Develop tests for jjx_curry (paddock getter/setter with chalk).

## Scope

1. Review jjx_curry implementation in jjrcu_curry.rs
2. Identify testable behaviors: getter mode, setter mode, verb handling, chalk entries
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_curry happy paths and error cases.

**[260124-1833] bridled**

Develop tests for jjx_curry (paddock getter/setter with chalk).

## Scope

1. Review jjx_curry implementation in jjrcu_curry.rs
2. Identify testable behaviors: getter mode, setter mode, verb handling, chalk entries
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_curry happy paths and error cases.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjtcu_curry.rs new, lib.rs add test mod (2 files to edit), jjrcu_curry.rs, jjro_ops.rs, jjtg_gallops.rs (3 files to read) | Steps: 1. Read jjrcu_curry.rs and jjro_ops.rs jjrg_curry function to understand implementation 2. Read jjtg_gallops.rs for test helper patterns 3. Create jjtcu_curry.rs with tests for CurryVerb as_str, verb validation logic, getter mode with mock files 4. Add test mod jjtcu_curry to lib.rs 5. Document in test file comment that setter commit behavior is not tested due to vvc dependency | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_curry (paddock getter/setter with chalk).

## Scope

1. Review jjx_curry implementation in jjrcu_curry.rs
2. Identify testable behaviors: getter mode, setter mode, verb handling, chalk entries
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_curry happy paths and error cases.

**[260124-1744] rough**

Develop tests for jjx_curry (paddock getter/setter with chalk).

## Scope

1. Review jjx_curry implementation in jjro_ops.rs
2. Identify testable behaviors: getter mode, setter mode, verb handling, chalk entries
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_curry happy paths and error cases.

### test-jjx-garland (₢AFAA9) [complete]

**[260124-1849] complete**

Develop tests for jjx_garland (celebrate heat completion, create continuation).

## Scope

1. Review jjx_garland implementation in jjrgl_garland.rs
2. Identify testable behaviors: completion detection, continuation heat creation, silks handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_garland happy paths and error cases.

**[260124-1834] bridled**

Develop tests for jjx_garland (celebrate heat completion, create continuation).

## Scope

1. Review jjx_garland implementation in jjrgl_garland.rs
2. Identify testable behaviors: completion detection, continuation heat creation, silks handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_garland happy paths and error cases.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjtgl_garland.rs new, lib.rs add test mod (2 files to edit), jjrgl_garland.rs, jjro_ops.rs jjrg_garland function, jjrq_query.rs silks helpers, jjtg_gallops.rs test patterns (4 files to read) | Steps: 1. Read jjrg_garland in jjro_ops.rs to understand operation logic 2. Read jjrq_query.rs for silks parsing helpers 3. Read jjtg_gallops.rs for test helper patterns with temp dirs 4. Create jjtgl_garland.rs with tests for heat not found, no actionable paces, successful garland with pace counts and silks transformation 5. Add test mod jjtgl_garland to lib.rs 6. Document that CLI commit behavior is not tested | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_garland (celebrate heat completion, create continuation).

## Scope

1. Review jjx_garland implementation in jjrgl_garland.rs
2. Identify testable behaviors: completion detection, continuation heat creation, silks handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_garland happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_garland (celebrate heat completion, create continuation).

## Scope

1. Review jjx_garland implementation in jjro_ops.rs
2. Identify testable behaviors: completion detection, continuation heat creation, silks handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_garland happy paths and error cases.

### test-jjx-restring (₢AFAA-) [complete]

**[260124-1855] complete**

Develop tests for jjx_restring (bulk pace transfer between heats).

## Scope

1. Review jjx_restring implementation in jjrrs_restring.rs
2. Identify testable behaviors: pace selection, transfer mechanics, order preservation
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_restring happy paths and error cases.

**[260124-1835] bridled**

Develop tests for jjx_restring (bulk pace transfer between heats).

## Scope

1. Review jjx_restring implementation in jjrrs_restring.rs
2. Identify testable behaviors: pace selection, transfer mechanics, order preservation
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_restring happy paths and error cases.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjtrs_restring.rs new, lib.rs add test mod (2 files to edit), jjro_ops.rs jjrg_restring function, jjtg_gallops.rs test patterns (2 files to read) | Steps: 1. Read jjrg_restring in jjro_ops.rs to understand validation and draft mechanics 2. Read jjtg_gallops.rs for test helper patterns 3. Create jjtrs_restring.rs with tests for same heat error, heat not found, empty coronets, coronet not in source, coronet wrong heat identity, successful restring with order preservation 4. Add test mod jjtrs_restring to lib.rs | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_restring (bulk pace transfer between heats).

## Scope

1. Review jjx_restring implementation in jjrrs_restring.rs
2. Identify testable behaviors: pace selection, transfer mechanics, order preservation
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_restring happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_restring (bulk pace transfer between heats).

## Scope

1. Review jjx_restring implementation in jjro_ops.rs
2. Identify testable behaviors: pace selection, transfer mechanics, order preservation
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_restring happy paths and error cases.

### test-jjx-landing (₢AFAA_) [complete]

**[260124-1902] complete**

Develop tests for jjx_landing (agent completion commits).

## Scope

1. Review jjx_landing implementation in jjrld_landing.rs
2. Identify testable behaviors: commit message formatting, coronet marker handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_landing happy paths and error cases.

**[260124-1835] bridled**

Develop tests for jjx_landing (agent completion commits).

## Scope

1. Review jjx_landing implementation in jjrld_landing.rs
2. Identify testable behaviors: commit message formatting, coronet marker handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_landing happy paths and error cases.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjtn_notch.rs add tests (1 file to edit), jjrn_notch.rs jjrn_format_landing_message, jjrld_landing.rs (2 files to read) | Steps: 1. Read jjrn_format_landing_message in jjrn_notch.rs 2. Read existing tests in jjtn_notch.rs to understand patterns 3. Add tests for jjrn_format_landing_message with various coronets and agent strings 4. Document that commit behavior is not tested due to vvc dependency | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_landing (agent completion commits).

## Scope

1. Review jjx_landing implementation in jjrld_landing.rs
2. Identify testable behaviors: commit message formatting, coronet marker handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_landing happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_landing (agent completion commits).

## Scope

1. Review jjx_landing implementation in jjro_ops.rs and jjrn_notch.rs
2. Identify testable behaviors: commit message formatting, coronet marker handling
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_landing happy paths and error cases.

### test-jjx-scout (₢AFABA) [complete]

**[260124-1902] complete**

Develop tests for jjx_scout (regex search across heats/paces).

## Scope

1. Review jjx_scout implementation in jjrsc_scout.rs
2. Identify testable behaviors: regex matching, multi-heat search, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_scout happy paths and error cases.

**[260124-1836] bridled**

Develop tests for jjx_scout (regex search across heats/paces).

## Scope

1. Review jjx_scout implementation in jjrsc_scout.rs
2. Identify testable behaviors: regex matching, multi-heat search, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_scout happy paths and error cases.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjtsc_scout.rs new, lib.rs add test mod (2 files to edit), jjrsc_scout.rs (1 file to read) | Steps: 1. Read jjrsc_scout.rs to understand helper functions 2. Create jjtsc_scout.rs with tests for zjrsc_pace_state_str all states, zjrsc_extract_match_context with various match positions and window sizes 3. Add test mod jjtsc_scout to lib.rs | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_scout (regex search across heats/paces).

## Scope

1. Review jjx_scout implementation in jjrsc_scout.rs
2. Identify testable behaviors: regex matching, multi-heat search, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_scout happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_scout (regex search across heats/paces).

## Scope

1. Review jjx_scout implementation in jjrq_query.rs
2. Identify testable behaviors: regex matching, multi-heat search, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_scout happy paths and error cases.

### test-jjx-furlough (₢AFABB) [complete]

**[260124-1907] complete**

Develop tests for jjx_furlough (change heat status/rename).

## Scope

1. Review jjx_furlough implementation in jjrfu_furlough.rs
2. Identify testable behaviors: status transitions (racing/stabled), silks rename, heat reordering
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_furlough happy paths and error cases.

**[260124-1837] bridled**

Develop tests for jjx_furlough (change heat status/rename).

## Scope

1. Review jjx_furlough implementation in jjrfu_furlough.rs
2. Identify testable behaviors: status transitions (racing/stabled), silks rename, heat reordering
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_furlough happy paths and error cases.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjtfu_furlough.rs new, lib.rs add test mod (2 files to edit), jjro_ops.rs jjrg_furlough function, jjtg_gallops.rs test patterns (2 files to read) | Steps: 1. Read jjrg_furlough in jjro_ops.rs 2. Read jjtg_gallops.rs for test helper patterns 3. Create jjtfu_furlough.rs with tests for no options error, both racing and stabled error, invalid silks, heat not found, retired heat error, already racing error, already stabled error, successful status change with reordering, successful silks rename 4. Add test mod jjtfu_furlough to lib.rs | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_furlough (change heat status/rename).

## Scope

1. Review jjx_furlough implementation in jjrfu_furlough.rs
2. Identify testable behaviors: status transitions (racing/stabled), silks rename, heat reordering
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_furlough happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_furlough (change heat status/rename).

## Scope

1. Review jjx_furlough implementation in jjro_ops.rs
2. Identify testable behaviors: status transitions (racing/stabled), silks rename, heat reordering
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_furlough happy paths and error cases.

### test-jjx-rein (₢AFABC) [complete]

**[260124-1914] complete**

Develop tests for jjx_rein (steeplechase queries).

## Scope

1. Review jjx_rein implementation in jjrrn_rein.rs
2. Identify testable behaviors: log parsing, filtering, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_rein happy paths and error cases.

**[260124-1908] complete**

Develop tests for jjx_rein (steeplechase queries).

## Scope

1. Review jjx_rein implementation in jjrrn_rein.rs
2. Identify testable behaviors: log parsing, filtering, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_rein happy paths and error cases.

**[260124-1853] bridled**

Develop tests for jjx_rein (steeplechase queries).

## Scope

1. Review jjx_rein implementation in jjrrn_rein.rs
2. Identify testable behaviors: log parsing, filtering, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_rein happy paths and error cases.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjtrn_rein.rs (1 file to edit), jjts_steeplechase.rs jjrs_steeplechase.rs jjrrn_rein.rs (3 files to read) | Steps: 1. Read jjrs_steeplechase.rs and jjts_steeplechase.rs to understand existing test patterns and coverage gaps 2. Read jjrrn_rein.rs to understand CLI wrapper 3. Add tests to jjtrn_rein.rs covering edge cases for timestamp parsing short strings and empty input, malformed log lines with wrong field counts, log lines with non-JJ commits, ReinArgs struct construction, and ensure module imports work correctly | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_rein (steeplechase queries).

## Scope

1. Review jjx_rein implementation in jjrrn_rein.rs
2. Identify testable behaviors: log parsing, filtering, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_rein happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_rein (steeplechase queries).

## Scope

1. Review jjx_rein implementation in jjrs_steeplechase.rs
2. Identify testable behaviors: log parsing, filtering, output formatting
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_rein happy paths and error cases.

### test-jjx-parade (₢AFABD) [complete]

**[260124-1914] complete**

Develop tests for jjx_parade (display heat/pace info).

## Scope

1. Review jjx_parade implementation in jjrpd_parade.rs
2. Identify testable behaviors: heat display, pace display, --full flag, --remaining flag
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_parade happy paths and error cases.

**[260124-1855] bridled**

Develop tests for jjx_parade (display heat/pace info).

## Scope

1. Review jjx_parade implementation in jjrpd_parade.rs
2. Identify testable behaviors: heat display, pace display, --full flag, --remaining flag
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_parade happy paths and error cases.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjtpd_parade.rs (1 file to edit), jjrpd_parade.rs jjtg_gallops.rs (2 files to read) | Steps: 1. Read jjtg_gallops.rs for test helper patterns make_valid_gallops make_valid_heat make_valid_pace 2. Add tests for zjjrpd_pace_state_str helper covering all four states 3. Add tests for zjjrpd_resolve_default_heat with racing heat present and no racing heats 4. Add test for target length validation error message 5. Focus on internal functions since CLI output capture is not available | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260124-1828] rough**

Develop tests for jjx_parade (display heat/pace info).

## Scope

1. Review jjx_parade implementation in jjrpd_parade.rs
2. Identify testable behaviors: heat display, pace display, --full flag, --remaining flag
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_parade happy paths and error cases.

**[260124-1745] rough**

Develop tests for jjx_parade (display heat/pace info).

## Scope

1. Review jjx_parade implementation in jjrq_query.rs
2. Identify testable behaviors: heat display, pace display, --full flag, --remaining flag
3. Write unit tests in new or existing jjt*.rs file
4. If implementation has issues or spec is ambiguous, document and propose fixes

## Spec Review Mandate

While planning tests, actively look for:
- Edge cases not handled
- Error messages that are unclear
- Behavior that contradicts documentation
- Missing validation

Push back on the specification if issues found — do not silently work around problems.

## Deliverable

Test functions covering jjx_parade happy paths and error cases.

### claudemd-quick-verb-audit (₢AFAAc) [complete]

**[260124-1918] complete**

Audit and update CLAUDE.md quick verb table for completeness.

## Tasks

1. Add `scout` → `/jjc-scout` to the quick verbs table
2. Survey all `/jjc-*` slash commands in `.claude/commands/`
3. Identify any missing from the quick verbs table that should be there
4. Update the table with any missing entries

## Criteria for Quick Verbs

Quick verbs are single-word shortcuts that invoke common operations. Good candidates:
- Frequently used commands
- Single intuitive verb (e.g., "mount", "wrap", "scout")
- Not compound operations (e.g., "heat-retire" stays as full command)

## Files

- CLAUDE.md — Quick Verbs table in JJK section
- .claude/commands/jjc-*.md — All JJK slash commands

**[260123-1933] rough**

Audit and update CLAUDE.md quick verb table for completeness.

## Tasks

1. Add `scout` → `/jjc-scout` to the quick verbs table
2. Survey all `/jjc-*` slash commands in `.claude/commands/`
3. Identify any missing from the quick verbs table that should be there
4. Update the table with any missing entries

## Criteria for Quick Verbs

Quick verbs are single-word shortcuts that invoke common operations. Good candidates:
- Frequently used commands
- Single intuitive verb (e.g., "mount", "wrap", "scout")
- Not compound operations (e.g., "heat-retire" stays as full command)

## Files

- CLAUDE.md — Quick Verbs table in JJK section
- .claude/commands/jjc-*.md — All JJK slash commands

### update-guide-copyrights (₢AFABH) [complete]

**[260124-1821] complete**

Update RCG and BCG copyright dates to 2026.

## Files

- Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md

## Deliverable

Change copyright year references from 2025 (or earlier) to 2026 in both guide files.

**[260124-1813] bridled**

Update RCG and BCG copyright dates to 2026.

## Files

- Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md

## Deliverable

Change copyright year references from 2025 (or earlier) to 2026 in both guide files.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: Tools/buk/vov_veiled/BCG-BashConsoleGuide.md (1 file) | Steps: 1. Read BCG file 2. Replace Copyright 2025 with Copyright 2026 in template example around line 102 3. Write file | Verify: File contains only 2026 copyright dates

**[260124-1810] rough**

Update RCG and BCG copyright dates to 2026.

## Files

- Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md

## Deliverable

Change copyright year references from 2025 (or earlier) to 2026 in both guide files.

## Steeplechase

### 2026-01-24 19:18 - ₢AFAAc - W

pace complete

### 2026-01-24 19:17 - ₢AFAAc - n

Add missing Quick Verbs entries to JJ command reference table

### 2026-01-24 19:15 - ₢AFAAc - A

Add scout, parade, rail, reslate, rein to quick verbs table

### 2026-01-24 19:14 - ₢AFABD - W

pace complete

### 2026-01-24 19:14 - ₢AFABC - W

pace complete

### 2026-01-24 19:14 - ₢AFABC - n

Add unit tests for jjx_parade command covering pace state display, default heat resolution, and target length validation

### 2026-01-24 19:08 - ₢AFABC - W

pace complete

### 2026-01-24 19:07 - ₢AFABB - W

pace complete

### 2026-01-24 19:07 - ₢AFABB - n

Add unit tests for jjx_furlough and jjx_rein commands covering error cases, status changes, silks validation, and steeplechase parsing edge cases

### 2026-01-24 19:06 - ₢AFABB - L

haiku landed

### 2026-01-24 19:03 - ₢AFABB - F

Executing bridled pace via haiku agent

### 2026-01-24 19:02 - ₢AFAA_ - W

pace complete

### 2026-01-24 19:02 - ₢AFABA - W

pace complete

### 2026-01-24 19:02 - ₢AFABA - n

Add unit tests for jjx_scout helper functions and landing message formatting

### 2026-01-24 18:57 - ₢AFAA_ - F

Executing bridled pace via haiku agent

### 2026-01-24 18:55 - ₢AFAA- - W

pace complete

### 2026-01-24 18:55 - ₢AFAA- - n

Add comprehensive test suite for jjx_restring command

### 2026-01-24 18:55 - ₢AFAA- - L

sonnet landed

### 2026-01-24 18:55 - ₢AFABD - B

tally | test-jjx-parade

### 2026-01-24 18:55 - Heat - T

test-jjx-parade

### 2026-01-24 18:53 - ₢AFABC - B

tally | test-jjx-rein

### 2026-01-24 18:53 - Heat - T

test-jjx-rein

### 2026-01-24 18:51 - ₢AFAA- - F

Executing bridled pace via sonnet agent

### 2026-01-24 18:49 - ₢AFAA9 - W

pace complete

### 2026-01-24 18:49 - ₢AFAA9 - n

Add test modules for garland, restring, scout, furlough, rein, and parade commands

### 2026-01-24 18:48 - ₢AFAA9 - L

sonnet landed

### 2026-01-24 18:43 - Heat - d

Prep test modules in lib.rs for parallel execution

### 2026-01-24 18:39 - ₢AFAA9 - F

Executing bridled pace via sonnet agent

### 2026-01-24 18:39 - ₢AFAA8 - W

pace complete

### 2026-01-24 18:39 - ₢AFAA8 - n

Add jjx_curry unit tests for CurryVerb and verb validation logic

### 2026-01-24 18:37 - ₢AFABB - B

tally | test-jjx-furlough

### 2026-01-24 18:37 - Heat - T

test-jjx-furlough

### 2026-01-24 18:36 - ₢AFABA - B

tally | test-jjx-scout

### 2026-01-24 18:36 - Heat - T

test-jjx-scout

### 2026-01-24 18:35 - ₢AFAA_ - B

tally | test-jjx-landing

### 2026-01-24 18:35 - Heat - T

test-jjx-landing

### 2026-01-24 18:35 - ₢AFAA- - B

tally | test-jjx-restring

### 2026-01-24 18:35 - Heat - T

test-jjx-restring

### 2026-01-24 18:34 - ₢AFAA9 - B

tally | test-jjx-garland

### 2026-01-24 18:34 - Heat - T

test-jjx-garland

### 2026-01-24 18:34 - ₢AFAA8 - F

Executing bridled pace via sonnet agent

### 2026-01-24 18:33 - ₢AFAA8 - B

tally | test-jjx-curry

### 2026-01-24 18:33 - Heat - T

test-jjx-curry

### 2026-01-24 18:32 - ₢AFAA8 - A

Unit tests for jjx_curry getter/setter modes

### 2026-01-24 18:31 - ₢AFABI - W

pace complete

### 2026-01-24 18:29 - ₢AFABI - A

Audit module overlap: jjrq_query.rs, jjro_ops.rs, jjrg_gallops.rs

### 2026-01-24 18:28 - Heat - T

test-jjx-parade

### 2026-01-24 18:28 - Heat - T

test-jjx-rein

### 2026-01-24 18:28 - Heat - T

test-jjx-furlough

### 2026-01-24 18:28 - Heat - T

test-jjx-scout

### 2026-01-24 18:28 - Heat - T

test-jjx-landing

### 2026-01-24 18:28 - Heat - T

test-jjx-restring

### 2026-01-24 18:28 - Heat - T

test-jjx-garland

### 2026-01-24 18:28 - Heat - T

test-jjx-curry

### 2026-01-24 18:27 - ₢AFABG - W

pace complete

### 2026-01-24 18:27 - ₢AFABG - n

Remove unused imports across JJK Rust modules

### 2026-01-24 18:26 - ₢AFABG - L

haiku landed

### 2026-01-24 18:24 - ₢AFABG - F

Executing bridled pace via haiku agent

### 2026-01-24 18:23 - Heat - S

audit-module-overlap

### 2026-01-24 18:22 - ₢AFABG - B

tally | trim-cli-dead-code

### 2026-01-24 18:22 - Heat - T

trim-cli-dead-code

### 2026-01-24 18:21 - ₢AFABH - W

pace complete

### 2026-01-24 18:20 - Heat - T

delete-aggregate-files

### 2026-01-24 18:18 - ₢AFABF - W

pace complete

### 2026-01-24 18:17 - ₢AFABF - n

Refactor JJK CLI dispatch to use per-command modules with Args and handlers

### 2026-01-24 18:14 - ₢AFABH - F

Executing bridled pace via haiku agent (out of order)

### 2026-01-24 18:13 - ₢AFABH - B

tally | update-guide-copyrights

### 2026-01-24 18:13 - Heat - T

update-guide-copyrights

### 2026-01-24 18:12 - ₢AFABF - A

Rewiring CLI dispatch to per-command modules

### 2026-01-24 18:10 - Heat - S

update-guide-copyrights

### 2026-01-24 18:10 - ₢AFABE - W

pace complete

### 2026-01-24 18:10 - ₢AFABE - n

Add JJK Rust command modules for all jjx_* subcommands

### 2026-01-24 18:07 - ₢AFAAM - W

pace complete

### 2026-01-24 18:07 - ₢AFAAM - n

Add /jjc-heat-braid command for consolidating overlapping paces between heats

### 2026-01-24 18:06 - ₢AFABE - F

Executing bridled pace via haiku agents (22 parallel)

### 2026-01-24 18:05 - ₢AFABE - B

tally | parallel-split-commands

### 2026-01-24 18:05 - Heat - T

parallel-split-commands

### 2026-01-24 18:03 - ₢AFAAM - F

Executing bridled pace via sonnet agent

### 2026-01-24 18:01 - Heat - r

moved AFABG after AFABF

### 2026-01-24 18:01 - Heat - r

moved AFABF after AFABE

### 2026-01-24 18:01 - Heat - r

moved AFABE to first

### 2026-01-24 18:01 - Heat - S

delete-aggregate-files

### 2026-01-24 18:00 - Heat - S

rewire-command-dispatch

### 2026-01-24 18:00 - Heat - S

parallel-split-commands

### 2026-01-24 17:57 - Heat - r

moved AFAAc to last

### 2026-01-24 17:55 - ₢AFAA5 - W

pace complete

### 2026-01-24 17:54 - ₢AFAA5 - n

Implement session probe: gap detection in saddle, s marker in chalk, slash command step 2.5

### 2026-01-24 17:53 - Heat - s

260124-1753 session

### 2026-01-24 17:48 - ₢AFAA5 - A

Implementing session probe: Rust gap detection + slash command probe spawning

### 2026-01-24 17:47 - Heat - T

trophy-alpha-session-detect

### 2026-01-24 17:45 - ₢AFAA7 - W

pace complete

### 2026-01-24 17:45 - ₢AFAA7 - n

Remove debug eprintln statements from jjx_tally, vvcc_commit, and vvcg_guard

### 2026-01-24 17:45 - Heat - S

test-jjx-parade

### 2026-01-24 17:45 - Heat - S

test-jjx-rein

### 2026-01-24 17:45 - Heat - S

test-jjx-furlough

### 2026-01-24 17:45 - Heat - S

test-jjx-scout

### 2026-01-24 17:45 - Heat - S

test-jjx-landing

### 2026-01-24 17:45 - Heat - S

test-jjx-restring

### 2026-01-24 17:45 - Heat - S

test-jjx-garland

### 2026-01-24 17:44 - Heat - S

test-jjx-curry

### 2026-01-24 17:44 - ₢AFAA7 - L

haiku landed

### 2026-01-24 17:42 - ₢AFAA7 - F

Executing bridled pace via haiku agent

### 2026-01-24 17:41 - ₢AFAA4 - W

pace complete

### 2026-01-24 17:41 - ₢AFAA4 - n

Add session marker specification to steeplechase protocol

### 2026-01-24 17:40 - ₢AFAA4 - L

sonnet landed

### 2026-01-24 17:40 - ₢AFAA7 - B

tally | strip-tally-chatter

### 2026-01-24 17:40 - Heat - T

strip-tally-chatter

### 2026-01-24 17:39 - Heat - S

strip-tally-chatter

### 2026-01-24 17:39 - ₢AFAA4 - F

Executing bridled pace via sonnet agent

### 2026-01-24 17:37 - ₢AFAA3 - W

pace complete

### 2026-01-24 17:37 - ₢AFAA3 - n

Add --intent flag to jjx_notch for user-provided commit messages

### 2026-01-24 17:37 - ₢AFAAM - B

tally | implement-heat-braid

### 2026-01-24 17:37 - Heat - T

implement-heat-braid

### 2026-01-24 17:37 - ₢AFAA3 - L

sonnet landed

### 2026-01-24 17:37 - Heat - T

design-heat-braid

### 2026-01-24 17:35 - ₢AFAA3 - F

Executing bridled pace via sonnet agent

### 2026-01-24 17:32 - ₢AFAA2 - W

pace complete

### 2026-01-24 17:31 - ₢AFAA2 - n

Add jjx_landing command for recording agent completion commits

### 2026-01-24 17:28 - ₢AFAA6 - W

pace complete

### 2026-01-24 17:25 - ₢AFAA6 - n

Add jjrn_format_landing_message for landing commit formatting

### 2026-01-24 17:24 - ₢AFAA6 - n

Add landing command to create empty landing commits with coronet markers

### 2026-01-24 17:23 - ₢AFAA6 - n

Add jjx_landing command for agent execution landing; remove debug lock messages

### 2026-01-24 17:21 - ₢AFAA6 - F

Executing bridled pace via haiku agent

### 2026-01-24 17:21 - ₢AFAA2 - F

Executing bridled pace via sonnet agent

### 2026-01-24 17:21 - ₢AFAA2 - B

tally | trophy-alpha-landing-marker

### 2026-01-24 17:21 - Heat - T

trophy-alpha-landing-marker

### 2026-01-24 17:20 - Heat - T

trophy-alpha-landing-marker

### 2026-01-24 17:19 - ₢AFAA6 - B

tally | remove-verbose-lock-messages

### 2026-01-24 17:19 - Heat - T

remove-verbose-lock-messages

### 2026-01-24 17:19 - Heat - T

trophy-alpha-landing-marker

### 2026-01-24 17:18 - Heat - S

remove-verbose-lock-messages

### 2026-01-24 17:18 - Heat - n

Simplify landing marker format: remove step counts and verification status, use agent completion report

### 2026-01-24 17:18 - Heat - T

trophy-alpha-landing-marker

### 2026-01-24 17:15 - Heat - n

Remove file_count from bridle marker format

### 2026-01-24 17:13 - ₢AFAA3 - B

tally | 0 files | trophy-alpha-notch-intent

### 2026-01-24 17:13 - Heat - T

trophy-alpha-notch-intent

### 2026-01-24 17:09 - ₢AFAA1 - W

pace complete

### 2026-01-24 17:09 - ₢AFAA1 - n

Add B (bridle) chalk marker and auto-commit on pace bridle transition

### 2026-01-24 16:50 - ₢AFAA1 - F

Executing bridled pace via sonnet agent

### 2026-01-24 16:49 - Heat - T

trophy-alpha-bridle-marker

### 2026-01-24 16:47 - ₢AFAA0 - W

pace complete

### 2026-01-24 16:47 - ₢AFAA0 - n

jjb:1010-7dcc0874:₢AFAA0:W: Add --intent option to notch and document bridle/landing markers

### 2026-01-24 16:44 - ₢AFAA0 - F

Executing bridled pace via sonnet agent

### 2026-01-24 16:43 - Heat - T

trophy-alpha-jjsa-session-marker

### 2026-01-24 16:43 - Heat - T

trophy-alpha-notch-intent

### 2026-01-24 16:43 - Heat - T

trophy-alpha-jjsa-commit-types

### 2026-01-24 16:41 - ₢AFAAz - W

pace complete

### 2026-01-24 16:41 - Heat - T

trophy-alpha-version-collect

### 2026-01-24 16:40 - Heat - T

trophy-alpha-jjsa-version-marker

### 2026-01-24 16:29 - ₢AFAAz - n

Filter recent-work table to show only notch, approach, and discussion entries; remove timestamp and action code columns

### 2026-01-24 16:28 - ₢AFAAz - F

Executing bridled pace via haiku agent

### 2026-01-24 16:27 - Heat - T

simplify-saddle-recent-work

### 2026-01-24 16:24 - Heat - r

moved AFAAc after AFAAM

### 2026-01-24 16:24 - Heat - r

moved AFAAM to last

### 2026-01-24 16:24 - ₢AFAAk - W

pace complete

### 2026-01-24 12:07 - Heat - S

trophy-alpha-version-collect

### 2026-01-24 12:07 - Heat - S

trophy-alpha-jjsa-version-marker

### 2026-01-24 11:50 - Heat - S

trophy-alpha-notch-intent

### 2026-01-24 11:50 - Heat - S

trophy-alpha-landing-marker

### 2026-01-24 11:50 - Heat - S

trophy-alpha-bridle-marker

### 2026-01-24 11:49 - Heat - S

trophy-alpha-jjsa-commit-types

### 2026-01-24 11:24 - ₢AFAAk - n

Update jjh_ trophy filename format to include created timestamp prefix

### 2026-01-24 11:21 - ₢AFAAk - A

End-to-end test: nominate test heat, add paces, manipulate states, retire, verify trophy

### 2026-01-24 10:59 - ₢AFAAw - W

pace complete

### 2026-01-24 10:57 - ₢AFAAw - n

Update JJSCRN-rein behavior section to reflect column-aligned output

### 2026-01-24 10:56 - ₢AFAAw - n

Convert jjx_rein output from JSON to column-aligned plain text table

### 2026-01-24 10:55 - ₢AFAAw - F

Executing bridled pace via haiku agent

### 2026-01-24 10:54 - Heat - r

moved AFAAw to first

### 2026-01-24 10:52 - ₢AFAAk - A

End-to-end test: nominate test heat, add paces, manipulate states, retire, verify trophy

### 2026-01-24 10:50 - ₢AFAAV - W

pace complete

### 2026-01-24 10:50 - ₢AFAAV - n

Update jjx_rein to output human-readable formatted text instead of JSON

### 2026-01-24 10:47 - ₢AFAAV - F

Executing bridled pace via haiku agent

### 2026-01-24 10:45 - ₢AFAAC - W

pace complete

### 2026-01-24 10:45 - ₢AFAAC - n

Add registry auto-commit on new hallmark allocation and relocate parcels to .jjk/parcels/

### 2026-01-24 10:43 - ₢AFAAC - F

Executing bridled pace via sonnet agent

### 2026-01-24 10:41 - Heat - T

vob-release-conformance

### 2026-01-24 10:39 - ₢AFAAo - W

pace complete

### 2026-01-24 10:39 - ₢AFAAo - n

Fix parade table separator using box-drawing character to prevent markdown dash interpretation

### 2026-01-24 10:37 - Heat - n

Clarify notch commit discipline: pace-affiliated vs heat-affiliated cases

### 2026-01-24 10:28 - Heat - S

simplify-saddle-recent-work

### 2026-01-24 10:25 - Heat - T

implement-jjx-curry

### 2026-01-24 10:25 - ₢AFAAd - W

pace complete

### 2026-01-24 10:20 - ₢AFAAd - F

Executing bridled pace via sonnet agent

### 2026-01-24 10:19 - ₢AFAAg - W

pace complete

### 2026-01-24 10:19 - ₢AFAAg - n

Remove dead zjjrq_SaddleOutput struct and obsolete JSON serialization tests

### 2026-01-24 10:14 - ₢AFAAg - F

Executing bridled pace via sonnet agent

### 2026-01-24 10:14 - ₢AFAAl - W

pace complete

### 2026-01-24 10:12 - ₢AFAAl - n

Normalize BUD_TEMP_DIR and BUD_OUTPUT_DIR to absolute paths

### 2026-01-24 10:11 - ₢AFAAl - F

Executing bridled pace via haiku agent

### 2026-01-24 10:10 - ₢AFAAm - W

pace complete

### 2026-01-24 10:09 - ₢AFAAm - n

Add commit message architecture section to VOS-VoxObscuraSpec

### 2026-01-24 10:08 - ₢AFAAm - F

Executing bridled pace via sonnet agent

### 2026-01-24 10:07 - Heat - r

moved AFAAm to first

### 2026-01-24 10:07 - Heat - T

saddle-column-format

### 2026-01-24 10:07 - ₢AFAAy - W

pace complete

### 2026-01-24 10:07 - ₢AFAAy - n

Migrate muster and parade output to jjrp_Table column formatter

### 2026-01-24 10:04 - Heat - T

common-mount-recommendation

### 2026-01-24 10:04 - ₢AFAAy - F

Executing bridled pace via sonnet agent

### 2026-01-24 10:03 - ₢AFAAx - W

pace complete

### 2026-01-24 10:03 - ₢AFAAx - n

Add jjrp_print column table module for structured output formatting

### 2026-01-24 10:02 - Heat - T

jjrp-incorporate-existing

### 2026-01-24 10:01 - Heat - n

Simplify jjc-heat-groom command: remove Firemark selection logic, defer to jjx_parade defaults

### 2026-01-24 09:59 - ₢AFAAx - F

Executing bridled pace via sonnet agent

### 2026-01-24 09:57 - Heat - T

jjrp-column-table-module

### 2026-01-24 09:57 - ₢AFAAj - W

pace complete

### 2026-01-24 09:57 - ₢AFAAj - n

Standardize jjx stdin patterns to heredoc across slash commands

### 2026-01-24 09:56 - Heat - T

saddle-column-format

### 2026-01-24 09:55 - Heat - S

jjrp-incorporate-existing

### 2026-01-24 09:55 - Heat - S

jjrp-column-table-module

### 2026-01-24 09:47 - Heat - T

muster-column-format

### 2026-01-24 09:46 - Heat - T

saddle-column-format

### 2026-01-24 09:45 - Heat - T

saddle-column-format

### 2026-01-24 09:42 - Heat - T

rein-column-format

### 2026-01-24 09:42 - Heat - T

garland-silks-parser

### 2026-01-24 09:42 - Heat - r

moved AFAAc to last

### 2026-01-24 09:41 - Heat - n

Add commit discipline reminder to JJ command documentation

### 2026-01-24 09:41 - Heat - T

common-mount-recommendation

### 2026-01-24 09:41 - Heat - T

rust-restring-spec

### 2026-01-24 09:39 - Heat - T

common-mount-recommendation

### 2026-01-24 09:37 - Heat - T

saddle-column-format

### 2026-01-24 09:33 - ₢AFAAf - W

pace complete

### 2026-01-24 09:33 - ₢AFAAf - n

jjx_muster and jjx_parade output column-aligned tables with headers

### 2026-01-24 09:32 - Heat - T

muster-box-table-output

### 2026-01-24 09:32 - Heat - T

saddle-markdown

### 2026-01-24 09:29 - Heat - S

rein-column-format

### 2026-01-24 09:20 - Heat - T

parade-remaining-markdown

### 2026-01-24 09:19 - Heat - S

rust-restring-slash

### 2026-01-24 09:19 - Heat - S

rust-restring-impl

### 2026-01-24 09:18 - Heat - S

rust-restring-spec

### 2026-01-24 09:18 - Heat - T

implement-garland-ceremony

### 2026-01-24 09:18 - Heat - S

garland-slash-cmd

### 2026-01-24 09:18 - Heat - S

garland-spec

### 2026-01-24 09:17 - Heat - S

garland-primitive

### 2026-01-24 09:16 - Heat - S

garland-silks-parser

### 2026-01-24 09:15 - Heat - T

implement-garland-ceremony

### 2026-01-24 08:49 - ₢AFAAf - F

Executing bridled pace via parallel haiku+sonnet agents

### 2026-01-24 08:47 - ₢AFAAn - W

pace complete

### 2026-01-24 08:47 - ₢AFAAn - n

Make target argument optional for parade and saddle commands, defaulting to first racing heat

### 2026-01-24 08:47 - Heat - T

design-heat-braid

### 2026-01-24 08:44 - ₢AFAAn - F

Executing bridled pace via sonnet agent

### 2026-01-24 08:43 - Heat - T

implement-jjx-curry

### 2026-01-24 08:43 - Heat - T

implement-jjx-curry

### 2026-01-24 08:38 - Heat - S

common-mount-recommendation

### 2026-01-24 08:30 - Heat - T

optional-firemark-target

### 2026-01-24 08:28 - Heat - T

saddle-markdown

### 2026-01-24 08:28 - Heat - T

parade-remaining-markdown

### 2026-01-24 08:28 - Heat - r

moved AFAAg after AFAAf

### 2026-01-24 08:28 - Heat - r

moved AFAAf after AFAAn

### 2026-01-24 08:27 - Heat - n

Simplify direction format to single-line shell-safe string for pace bridling

### 2026-01-24 08:25 - Heat - T

slash-cmd-heredoc-stdin

### 2026-01-24 08:22 - Heat - T

saddle-markdown

### 2026-01-24 08:22 - Heat - T

parade-remaining-markdown

### 2026-01-24 08:21 - Heat - r

moved AFAAM after AFAAl

### 2026-01-24 08:21 - Heat - S

optional-firemark-target

### 2026-01-24 08:14 - Heat - T

vos-commit-message-format

### 2026-01-24 08:13 - Heat - T

vob-release-conformance

### 2026-01-24 08:13 - Heat - S

vos-commit-message-format

### 2026-01-24 08:12 - Heat - T

slash-cmd-heredoc-stdin

### 2026-01-24 08:06 - Heat - T

bud-absolute-paths

### 2026-01-24 08:02 - Heat - S

bud-absolute-paths

### 2026-01-24 07:51 - Heat - r

moved AFAAk before AFAAK

### 2026-01-24 07:51 - Heat - S

test-trophy-operation

### 2026-01-24 07:49 - Heat - T

slash-cmd-heredoc-stdin

### 2026-01-24 07:48 - Heat - T

test-heredoc-2

### 2026-01-24 07:48 - Heat - T

test-heredoc

### 2026-01-24 07:48 - ₢AFAAj - n

Document heredoc delimiter selection conventions for stdin patterns

### 2026-01-24 07:44 - Heat - T

saddle-markdown

### 2026-01-24 07:44 - Heat - r

moved AFAAj to first

### 2026-01-24 07:43 - Heat - T

parade-remaining-markdown

### 2026-01-24 07:43 - Heat - S

slash-cmd-heredoc-stdin

### 2026-01-24 07:43 - Heat - S

test-heredoc-2

### 2026-01-24 07:43 - Heat - S

test-heredoc

### 2026-01-24 07:42 - Heat - S

saddle-markdown

### 2026-01-24 07:42 - Heat - S

parade-remaining-markdown

### 2026-01-24 07:39 - Heat - T

implement-jjx-curry

### 2026-01-24 07:38 - Heat - T

implement-jjx-curry

### 2026-01-24 07:34 - ₢AFAAb - W

pace complete

### 2026-01-24 07:34 - ₢AFAAb - n

Switch heats map from BTreeMap to IndexMap for stable ordering; furlough --racing moves heat to front; mount/groom auto-select first racing heat

### 2026-01-24 07:28 - ₢AFAAb - F

Executing bridled pace via sonnet agent

### 2026-01-24 07:25 - Heat - r

moved AFAAb to first

### 2026-01-24 07:24 - Heat - T

muster-box-table-output

### 2026-01-24 07:23 - Heat - r

moved AFAAe to first

### 2026-01-24 07:20 - Heat - T

muster-box-table-output

### 2026-01-24 07:18 - Heat - S

muster-box-table-output

### 2026-01-23 20:16 - Heat - n

Document JJK CLI pitfalls and parade/saddle command patterns

### 2026-01-23 20:13 - Heat - T

rein-token-efficiency

### 2026-01-23 20:06 - Heat - T

design-heat-braid

### 2026-01-23 20:05 - Heat - S

implement-jjx-curry

### 2026-01-23 19:47 - Heat - T

heat-json-order-auto-groom-mount-choice

### 2026-01-23 19:42 - ₢AFAAO - W

pace complete

### 2026-01-23 19:42 - ₢AFAAO - n

Add pre-wrap build/test verification to jjc-pace-wrap and CLAUDE.md

### 2026-01-23 19:35 - ₢AFAAL - W

pace complete

### 2026-01-23 19:35 - ₢AFAAL - n

Remove --created arg from jjx_nominate; derive date from BUD_NOW_STAMP env var or system clock

### 2026-01-23 19:33 - ₢AFAAL - F

Executing bridled pace via haiku agent

### 2026-01-23 19:33 - Heat - S

claudemd-quick-verb-audit

### 2026-01-23 19:31 - Heat - r

moved AFAAO after AFAAL

### 2026-01-23 19:29 - Heat - T

nominate-created-from-env

### 2026-01-23 19:29 - ₢AFAAY - W

pace complete

### 2026-01-23 17:25 - Heat - T

nominate-default-created-date

### 2026-01-23 17:19 - Heat - T

heat-json-order-auto-groom-mount-choice

### 2026-01-23 17:17 - Heat - S

heat-json-order-auto-groom-mount-choice

### 2026-01-23 17:00 - ₢AFAAP - A

JJSA updates + env var support + integration test harness

### 2026-01-23 17:00 - ₢AFAAX - W

pace complete

### 2026-01-23 17:00 - ₢AFAAX - n

Add progress stats line to parade --remaining output

### 2026-01-23 16:56 - ₢AFAAX - W

pace complete

### 2026-01-23 16:56 - ₢AFAAX - n

Streamline groom command to show remaining paces by default instead of full parade output

### 2026-01-23 16:55 - ₢AFAAX - n

Add multi-session discipline guidelines to JJ configuration

### 2026-01-23 16:42 - ₢AFAAX - n

Fix acronym references: JJD → JJSA in JJ heat/pace documentation

### 2026-01-23 16:42 - Heat - n

Refactor logging suppression and update VVX launcher configuration

### 2026-01-23 16:38 - ₢AFAAX - A

Update 2 pace specs via reslate, 3 files via edit; keep silks as-is

### 2026-01-23 16:29 - ₢AFAAZ - W

pace complete

### 2026-01-23 16:28 - ₢AFAAa - W

pace complete

### 2026-01-23 16:28 - ₢AFAAa - n

Fix heat nomination test to verify correct initial status

### 2026-01-23 16:28 - ₢AFAAZ - n

Add tests for GetSpec and GetCoronets query operations

### 2026-01-23 16:27 - Heat - S

fix-nominate-test

### 2026-01-23 16:24 - ₢AFAAZ - n

Add jjx_get_spec and jjx_get_coronets commands for querying pace data

### 2026-01-23 14:59 - ₢AFAAX - n

Update JJK acronym from JJD to JJSA in documentation and references

### 2026-01-23 14:58 - Heat - S

implement-text-emitter-commands

### 2026-01-23 14:53 - Heat - S

fix-jjd-reslate-extraction

### 2026-01-23 14:38 - Heat - T

jjk-integration-test-harness

### 2026-01-23 14:35 - Heat - T

jjk-integration-test-harness

### 2026-01-23 14:31 - ₢AFAAX - A

Update JJD→JJSA: paddock refs (AF, AD, AG, AH), itch, bridle.md, VOS, VLS; skip gallops.json completed specs

### 2026-01-23 14:30 - ₢AFAAW - W

pace complete

### 2026-01-23 14:30 - ₢AFAAW - n

Split JJD-GallopsData into JJSA main file with includes for operations and routines, update CLAUDE.md mappings, simplify jjc-heat-groom to use --full flag, and consolidate parade command references

### 2026-01-23 14:18 - Heat - T

split-jjd-subfiles

### 2026-01-23 14:15 - ₢AFAAW - F

Executing bridled pace via haiku/sonnet agents

### 2026-01-23 14:12 - Heat - S

jjd-to-jjsa-references

### 2026-01-23 14:07 - Heat - T

split-jjd-subfiles

### 2026-01-23 14:03 - Heat - S

split-jjd-subfiles

### 2026-01-23 13:44 - Heat - r

moved AFAAP to first

### 2026-01-23 13:44 - Heat - r

moved AFAAK to last

### 2026-01-23 13:25 - Heat - T

design-heat-braid

### 2026-01-23 13:18 - Heat - S

rein-token-efficiency

### 2026-01-23 13:14 - Heat - n

Add reporting step to jjc-pace-wrap command documentation

### 2026-01-23 13:10 - ₢AFAAJ - W

pace complete

### 2026-01-23 13:10 - ₢AFAAJ - n

Change default heat status from Racing to Stabled on nomination

### 2026-01-23 13:09 - ₢AFAAJ - F

Executing bridled pace via haiku agent

### 2026-01-23 13:08 - ₢AFAAR - W

pace complete

### 2026-01-23 13:08 - ₢AFAAR - n

Add regex dependency and enhance parade detail to show full tack history with chronological ordering and commit hashes

### 2026-01-23 13:03 - ₢AFAAR - F

Executing bridled pace via sonnet agent

### 2026-01-23 13:01 - ₢AFAAS - W

pace complete

### 2026-01-23 13:01 - ₢AFAAS - n

Consolidate parade commands into unified jjx_parade with target-type detection

### 2026-01-23 12:55 - ₢AFAAS - F

Executing bridled pace via sonnet agent

### 2026-01-23 12:53 - Heat - T

parade-coronet-history

### 2026-01-23 12:53 - ₢AFAAI - W

pace complete

### 2026-01-23 12:53 - ₢AFAAI - n

Add jjx_scout command for regex search across heats and paces

### 2026-01-23 12:51 - Heat - r

moved AFAAR after AFAAS

### 2026-01-23 12:51 - Heat - r

moved AFAAS after AFAAI

### 2026-01-23 12:49 - Heat - T

consolidate-parade-slash-commands

### 2026-01-23 12:49 - Heat - T

consolidate-parade-slash-commands

### 2026-01-23 12:47 - ₢AFAAI - F

Executing bridled pace via sonnet agent

### 2026-01-23 12:46 - ₢AFAAU - W

pace complete

### 2026-01-23 12:46 - ₢AFAAU - n

Remove auto-advance from wrap command, add mount recommendation to CLI output

### 2026-01-23 12:44 - ₢AFAAU - F

Executing bridled pace via haiku agent

### 2026-01-23 12:43 - Heat - r

moved AFAAU to first

### 2026-01-23 12:42 - ₢AFAAH - W

pace complete

### 2026-01-23 12:42 - ₢AFAAH - n

Add pace state classification predicates (defined/resolved) and update muster to show completed/defined counts

### 2026-01-23 12:38 - ₢AFAAH - F

Executing bridled pace via sonnet+haiku agents

### 2026-01-23 12:37 - ₢AFAAF - W

pace complete

### 2026-01-23 12:37 - Heat - n

Support verification-only paces in wrap by making commit optional when no staged changes exist

### 2026-01-23 12:36 - Heat - T

wrap-recommends-clear-mount

### 2026-01-23 12:35 - Heat - S

wrap-recommends-clear-mount

### 2026-01-23 12:30 - ₢AFAAF - F

Executing bridled pace via haiku agent

### 2026-01-23 12:29 - Heat - n

Clarify `jjda_first` insertion behavior to account for non-actionable paces

### 2026-01-23 12:28 - Heat - T

add-macos-sandbox

### 2026-01-23 12:22 - ₢AFAAE - F

Executing bridled pace via haiku agent

### 2026-01-23 12:19 - Heat - T

implement-scout-search

### 2026-01-23 12:19 - Heat - T

pace-tack-history-dump

### 2026-01-23 12:16 - Heat - T

implement-scout-search

### 2026-01-23 12:11 - ₢AFAAD - W

pace complete

### 2026-01-23 12:11 - ₢AFAAD - n

jjb:1010-f86d02cf:₢AFAAD:W: Fix jjx_muster racing heat filter to use grep instead of unsupported --status flag

### 2026-01-23 12:06 - ₢AFAAD - F

Executing bridled pace via haiku agent

### 2026-01-23 12:05 - ₢AFAAT - W

pace complete

### 2026-01-23 12:05 - ₢AFAAT - n

Auto-reset bridled pace to rough when reslate provides new spec text

### 2026-01-23 12:04 - ₢AFAAT - F

Executing bridled pace via sonnet agent

### 2026-01-23 12:03 - ₢AFAAQ - W

pace complete

### 2026-01-23 12:03 - ₢AFAAQ - n

Add approval prompt before autonomous bridled pace execution and add --status filter to muster command

### 2026-01-23 12:01 - ₢AFAAQ - F

Executing bridled pace via sonnet agent

### 2026-01-23 11:59 - Heat - r

moved AFAAQ to first

### 2026-01-23 11:59 - Heat - T

mount-requires-approval

### 2026-01-23 11:58 - Heat - T

reslate-resets-bridled-to-rough

### 2026-01-23 11:56 - Heat - T

reslate-resets-bridled-to-rough

### 2026-01-23 11:53 - Heat - T

reslate-resets-bridled-to-rough

### 2026-01-23 11:53 - Heat - r

moved AFAAD after AFAAT

### 2026-01-23 11:52 - ₢AFAAT - A

Add state check in Step 4: if bridled, pass --state rough

### 2026-01-23 11:52 - Heat - S

reslate-resets-bridled-to-rough

### 2026-01-23 11:50 - Heat - T

muster-status-filter

### 2026-01-23 11:50 - Heat - S

consolidate-parade-slash-commands

### 2026-01-23 11:49 - Heat - T

muster-status-filter

### 2026-01-23 11:48 - Heat - S

pace-tack-history-dump

### 2026-01-23 11:40 - Heat - S

mount-requires-approval

### 2026-01-23 11:40 - Heat - S

jjk-integration-test-harness

### 2026-01-23 11:37 - ₢AFAAD - F

Executing bridled pace via haiku agent

### 2026-01-23 11:35 - Heat - S

investigate-hallmark-format-discrepancy

### 2026-01-23 11:33 - ₢AFAAA - W

pace complete

### 2026-01-23 11:33 - ₢AFAAA - n

Add jjx_wrap routine and update jjx_notch to require explicit file list with heat-only commit support

### 2026-01-23 11:26 - Heat - T

verify-tests-block-release

### 2026-01-23 11:26 - Heat - n

Update jjc-pace-notch command to require explicit file list and support heat-only commits

### 2026-01-23 11:25 - ₢AFAAA - n

Simplify jjc-pace-wrap: integrate commit step and remove redundant outcome handling

### 2026-01-23 11:25 - Heat - T

make-tests-block-release

### 2026-01-23 11:18 - Heat - r

moved AFAAC to last

### 2026-01-23 11:16 - ₢AFAAA - F

Executing bridled pace via 2 parallel sonnet agents

### 2026-01-23 11:14 - Heat - T

add-macos-sandbox

### 2026-01-23 11:14 - Heat - T

wrap-notch-self-sufficiency

### 2026-01-23 11:14 - Heat - T

add-macos-sandbox

### 2026-01-23 11:11 - Heat - T

wrap-notch-self-sufficiency

### 2026-01-23 11:08 - ₢AFAAB - n

Clarify pace bridle direction format: remove angle brackets, add shell-safety guidance

### 2026-01-23 11:04 - Heat - T

muster-completed-defined-columns

### 2026-01-23 10:52 - Heat - T

repair-muster-pace-columns

### 2026-01-23 10:47 - ₢AFAAB - W

Implemented jjri_persist routine centralizing gallops+paddock commit logic; refactored 5 commands to use it; documented in JJD spec.

### 2026-01-23 10:47 - Heat - T

gallops-commit-routine

### 2026-01-23 10:47 - ₢AFAAB - n

Introduce jjri_persist routine for centralized gallops commit logic

### 2026-01-23 10:43 - Heat - T

nominate-default-created-date

### 2026-01-23 10:42 - Heat - T

nominate-defaults-stabled

### 2026-01-23 10:42 - ₢AFAAB - F

Executing bridled pace via 2x sonnet agents

### 2026-01-23 10:40 - Heat - T

fix-muster-status-filter

### 2026-01-23 10:40 - Heat - T

muster-status-filter

### 2026-01-23 10:40 - Heat - T

gallops-commit-routine

### 2026-01-23 10:39 - Heat - S

fix-muster-status-filter

### 2026-01-23 10:35 - ₢AFAAB - n

Simplify wrap command by removing saddle call and redundant state checks

### 2026-01-23 10:32 - ₢AFAAG - W

Removed BRAND field; commit format now jjb:HALLMARK:IDENTITY:ACTION:; rein validated

### 2026-01-23 10:32 - Heat - T

fix-hallmark-and-rein-no-rbm

### 2026-01-23 10:32 - ₢AFAAG - n

Refactor commit format: remove BRAND field, make ACTION required for all commits

### 2026-01-23 10:30 - ₢AFAAG - A

Test commit 3: approach marker

### 2026-01-23 10:30 - ₢AFAAG - d

Test commit 2: another sample entry

### 2026-01-23 10:30 - ₢AFAAG - d

Test commit 1: validating new format without BRAND

