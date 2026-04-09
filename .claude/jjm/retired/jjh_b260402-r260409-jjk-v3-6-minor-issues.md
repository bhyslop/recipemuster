# Heat Trophy: jjk-v3-6-minor-issues

**Firemark:** ₣A2
**Created:** 260402
**Retired:** 260409
**Status:** retired

## Paddock

# Paddock: jjk-v3-6-minor-issues

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### gazette-fenced-code-block-awareness (₢A2AAH) [complete]

**[260403-1851] complete**

Drafted from ₢AmAAJ in ₣Am.

## Character
Surgical parser fix — small change, high value. The gazette wire format parser currently treats every `#`-prefixed line as a notice boundary, even inside fenced code blocks.

## Problem
Gazette content frequently contains bash/code examples with `#` comments at column 0. The line-oriented parser can't distinguish a wire format notice header (`# jjezs_slate ...`) from a code comment inside triple-backtick fences. This causes silent content corruption or parse errors like `unknown slug 'output:'`.

## Observed failure
A docket containing a bash code block with `# output: hash *filename` was parsed as a notice with invalid slug `output:`, failing the enroll command.

## Fix
Track fenced code block state in the gazette parser. When inside a ``` fence, skip `#`-line notice detection. The parser already reads line-by-line — adding a toggle for "inside fence" is minimal.

Pseudocode:
- Maintain a boolean `in_fence`
- On any line matching `^````, toggle `in_fence`
- Only parse `# slug` headers when `in_fence` is false

## Acceptance
- Gazette content containing `#` inside fenced code blocks parses correctly
- Existing gazette operations unaffected
- Add a test case with a code block containing `# comment` lines

**[260403-1847] rough**

Drafted from ₢AmAAJ in ₣Am.

## Character
Surgical parser fix — small change, high value. The gazette wire format parser currently treats every `#`-prefixed line as a notice boundary, even inside fenced code blocks.

## Problem
Gazette content frequently contains bash/code examples with `#` comments at column 0. The line-oriented parser can't distinguish a wire format notice header (`# jjezs_slate ...`) from a code comment inside triple-backtick fences. This causes silent content corruption or parse errors like `unknown slug 'output:'`.

## Observed failure
A docket containing a bash code block with `# output: hash *filename` was parsed as a notice with invalid slug `output:`, failing the enroll command.

## Fix
Track fenced code block state in the gazette parser. When inside a ``` fence, skip `#`-line notice detection. The parser already reads line-by-line — adding a toggle for "inside fence" is minimal.

Pseudocode:
- Maintain a boolean `in_fence`
- On any line matching `^````, toggle `in_fence`
- Only parse `# slug` headers when `in_fence` is false

## Acceptance
- Gazette content containing `#` inside fenced code blocks parses correctly
- Existing gazette operations unaffected
- Add a test case with a code block containing `# comment` lines

**[260331-1050] rough**

## Character
Surgical parser fix — small change, high value. The gazette wire format parser currently treats every `#`-prefixed line as a notice boundary, even inside fenced code blocks.

## Problem
Gazette content frequently contains bash/code examples with `#` comments at column 0. The line-oriented parser can't distinguish a wire format notice header (`# jjezs_slate ...`) from a code comment inside triple-backtick fences. This causes silent content corruption or parse errors like `unknown slug 'output:'`.

## Observed failure
A docket containing a bash code block with `# output: hash *filename` was parsed as a notice with invalid slug `output:`, failing the enroll command.

## Fix
Track fenced code block state in the gazette parser. When inside a ``` fence, skip `#`-line notice detection. The parser already reads line-by-line — adding a toggle for "inside fence" is minimal.

Pseudocode:
- Maintain a boolean `in_fence`
- On any line matching `^````, toggle `in_fence`
- Only parse `# slug` headers when `in_fence` is false

## Acceptance
- Gazette content containing `#` inside fenced code blocks parses correctly
- Existing gazette operations unaffected
- Add a test case with a code block containing `# comment` lines

### rename-jjk-hallmark-to-brand (₢A2AAE) [complete]

**[260403-1013] complete**

## Character
Mechanical rename across two kits (JJK + VOK), Rust, bash, specs, and JSON wire formats.

## Docket
Rename "hallmark" to "brand" across VVK/JJK/VOK. Aligns spec vocabulary with existing `vvbf_brand.json` file name. Frees "hallmark" for RBK usage.

### JSON key renames
```
vovr_registry.json:  "hallmarks"      →  "vovr_brands"
vvbf_brand.json:     "vvbh_hallmark"  →  "vvbf_brand"
```

### Scope
- VOK: `VOS0` spec, `vofr_release.rs`, `vofe_emplace.rs`, `vorm_main.rs`, `vob_build.sh`, `vovr_registry.json`
- JJK: `jjrs_steeplechase.rs`, `jjrn_notch.rs`, `jjrnc_notch.rs`, `jjrm_mcp.rs`, tests, specs (JJS0, JJSCCH, JJSCNC, JJSCLD, JJSRPS, JJSCRN)
- External: `pb_paneboard02/.vvk/vvbf_brand.json`

### Verification
- `tt/vow-b.Build.sh` succeeds
- `tt/vow-t.Test.sh` passes
- No remaining "hallmark" references (grep, excluding git history)

**[260403-0956] rough**

## Character
Mechanical rename across two kits (JJK + VOK), Rust, bash, specs, and JSON wire formats.

## Docket
Rename "hallmark" to "brand" across VVK/JJK/VOK. Aligns spec vocabulary with existing `vvbf_brand.json` file name. Frees "hallmark" for RBK usage.

### JSON key renames
```
vovr_registry.json:  "hallmarks"      →  "vovr_brands"
vvbf_brand.json:     "vvbh_hallmark"  →  "vvbf_brand"
```

### Scope
- VOK: `VOS0` spec, `vofr_release.rs`, `vofe_emplace.rs`, `vorm_main.rs`, `vob_build.sh`, `vovr_registry.json`
- JJK: `jjrs_steeplechase.rs`, `jjrn_notch.rs`, `jjrnc_notch.rs`, `jjrm_mcp.rs`, tests, specs (JJS0, JJSCCH, JJSCNC, JJSCLD, JJSRPS, JJSCRN)
- External: `pb_paneboard02/.vvk/vvbf_brand.json`

### Verification
- `tt/vow-b.Build.sh` succeeds
- `tt/vow-t.Test.sh` passes
- No remaining "hallmark" references (grep, excluding git history)

**[260403-0956] rough**

## Character
Mechanical rename across two kits (JJK + VOK), Rust code, bash, specs, and JSON wire formats. Larger surface than initially scoped but each change is straightforward.

## Docket
Rename the VVK/JJK concept "hallmark" to "brand" to free the word for RBK's Consecration → Hallmark rename. The source file is already `vvbf_brand.json` — this aligns the spec vocabulary with the existing file name.

### JSON key renames (persistent wire format)
```
vovr_registry.json:  "hallmarks"      →  "vovr_brands"
vvbf_brand.json:     "vvbh_hallmark"  →  "vvbf_brand"
```

### Scope
- **VOK spec**: `VOS0-VoxObscuraSpec.adoc` — quoins `vost_hallmark`, `vosem_hallmark`, all references
- **VOK Rust**: `vofr_release.rs` (~40 refs), `vofe_emplace.rs` (~15 refs), `vorm_main.rs` (~5 refs)
- **VOK bash**: `vob_build.sh` (~15 refs)
- **VOK data**: `vovr_registry.json` — top-level key rename
- **JJK Rust**: `jjrs_steeplechase.rs`, `jjrn_notch.rs`, `jjrnc_notch.rs`, `jjrm_mcp.rs`
- **JJK tests**: `jjts_steeplechase.rs`, `jjtn_notch.rs`
- **JJK specs**: JJS0, JJSCCH, JJSCNC, JJSCLD, JJSRPS, JJSCRN
- **External**: `pb_paneboard02/.vvk/vvbf_brand.json` — key rename (separate repo)

### Exclusions
- Git history — old commits retain `jjb:NNNN:...` format unchanged
- The wire format numbers are unchanged — only the name for the concept changes

### Verification
- `tt/vow-b.Build.sh` succeeds
- `tt/vow-t.Test.sh` passes
- No remaining references to "hallmark" in JJK/VOK code (grep, excluding git history)

### Acceptance
- VOS0 and JJS0 use "brand" consistently
- All Rust/bash code uses "brand"
- JSON keys use prefixed form (`vovr_brands`, `vvbf_brand`)
- Word "hallmark" freed for RBK usage

**[260403-0949] rough**

## Character
Mechanical rename across spec and Rust code. Straightforward find-replace with attention to the commit wire format documentation.

## Docket
Rename the JJK concept "hallmark" to "brand" to free the word for RBK's Consecration → Hallmark rename. The source file is already `vvbf_brand.json` — this aligns the spec vocabulary with the existing file name.

### Scope
- JJS0 spec: all references to "hallmark" in commit format documentation, field descriptions
- Rust code: constants, variable names, string literals, comments referencing hallmark
- JJK claude-context (`jjk-claude-context.md`): any references to hallmark in the commit format docs
- VVK code: if `vvbf_brand.json` parsing code references "hallmark" internally

### Exclusions
- Git history — old commits retain `jjb:NNNN:...` format, the numeric value is unchanged
- The wire format itself doesn't change — the number is the same, only the name for it changes in spec/code

### Work items
- Update JJS0 spec: hallmark → brand in all definitions and examples
- Update Rust code: rename hallmark references to brand
- Update jjk-claude-context.md if needed
- Build and test

### Verification
- `tt/vow-b.Build.sh` succeeds
- `tt/vow-t.Test.sh` passes
- No remaining references to "hallmark" in JJK/VVK code (grep, excluding git history)

### Acceptance
- JJS0 uses "brand" consistently
- All Rust code uses "brand" for the version identifier
- Word "hallmark" freed for RBK usage

### jjk-disk-space-guard (₢A2AAA) [complete]

**[260402-0739] complete**

## Character

Mechanical integration work with a design edge — the `sysinfo` crate does the heavy lifting, but the guard must be wired into three JJK command paths cleanly without disrupting existing flow. The error message is load-bearing UX: it must tell the user exactly what's wrong and how to fix it.

## Objective

Add a cross-platform disk space check that blocks `jjx_open`, `jjx_orient`, and `jjx_show` (the commands backing open, mount, and groom) when any filesystem with capacity > 10 GB is >= 85% full. Hard error — no override mechanism. User must free space in a separate terminal before retrying.

## Approach

- Add `sysinfo` crate dependency to JJK's `Cargo.toml`
- New module: `jjrdk_diskcheck.rs` — self-contained disk space guard
- Implement using `sysinfo::Disks::new_with_refreshed_list()`
- For each disk: skip if `total_space < 10 GB`; compute `used_pct = (total - available) / total * 100`; collect violations at >= 85%
- **APFS dedup**: Multiple APFS volumes sharing a container report identical (total, available) pairs. Deduplicate violations by (total_space, available_space) tuple — show one diagnostic line per unique pair, listing representative mount point
- No platform-specific code — `sysinfo` handles Windows/macOS/Linux uniformly

## Integration points in `jjrm_mcp.rs`

- `jjx_open`: line ~765, before officium creation (bypasses normal validation path)
- `jjx_orient` and `jjx_show`: after officium validation (~line 782), before command routing (~line 820). Guard these two commands specifically, not the entire dispatch

## Error message format

```
DISK SPACE CRITICAL — Job Jockey refusing to proceed.

  /System/Volumes/Data: 86.2% full (28 GB free of 228 GB)

Free space before retrying. Quick wins:
  docker container prune       # remove stopped containers
  docker image prune            # remove dangling images
  docker system prune -a        # remove ALL unused images (will need re-pull)
```

Multiple violations shown as separate indented lines. Human-readable byte units (GB/TB).

## Acceptance

- `jjx_open` refuses with clear diagnostic when any qualifying disk >= 85% full
- `jjx_orient` and `jjx_show` refuse similarly
- Works on macOS (APFS), Linux (ext4/xfs), Windows (NTFS) without conditional compilation
- APFS duplicate volumes are deduplicated in output — one line per unique (total, available) pair
- Error message includes actionable Docker cleanup commands
- No override — hard block, no env var escape hatch
- Other jjx commands (record, close, enroll, etc.) are not gated
- Testing: manual verification against current overlarge system (currently at 86%)

**[260402-0701] rough**

## Character

Mechanical integration work with a design edge — the `sysinfo` crate does the heavy lifting, but the guard must be wired into three JJK command paths cleanly without disrupting existing flow. The error message is load-bearing UX: it must tell the user exactly what's wrong and how to fix it.

## Objective

Add a cross-platform disk space check that blocks `jjx_open`, `jjx_orient`, and `jjx_show` (the commands backing open, mount, and groom) when any filesystem with capacity > 10 GB is >= 85% full. Hard error — no override mechanism. User must free space in a separate terminal before retrying.

## Approach

- Add `sysinfo` crate dependency to JJK's `Cargo.toml`
- New module: `jjrdk_diskcheck.rs` — self-contained disk space guard
- Implement using `sysinfo::Disks::new_with_refreshed_list()`
- For each disk: skip if `total_space < 10 GB`; compute `used_pct = (total - available) / total * 100`; collect violations at >= 85%
- **APFS dedup**: Multiple APFS volumes sharing a container report identical (total, available) pairs. Deduplicate violations by (total_space, available_space) tuple — show one diagnostic line per unique pair, listing representative mount point
- No platform-specific code — `sysinfo` handles Windows/macOS/Linux uniformly

## Integration points in `jjrm_mcp.rs`

- `jjx_open`: line ~765, before officium creation (bypasses normal validation path)
- `jjx_orient` and `jjx_show`: after officium validation (~line 782), before command routing (~line 820). Guard these two commands specifically, not the entire dispatch

## Error message format

```
DISK SPACE CRITICAL — Job Jockey refusing to proceed.

  /System/Volumes/Data: 86.2% full (28 GB free of 228 GB)

Free space before retrying. Quick wins:
  docker container prune       # remove stopped containers
  docker image prune            # remove dangling images
  docker system prune -a        # remove ALL unused images (will need re-pull)
```

Multiple violations shown as separate indented lines. Human-readable byte units (GB/TB).

## Acceptance

- `jjx_open` refuses with clear diagnostic when any qualifying disk >= 85% full
- `jjx_orient` and `jjx_show` refuse similarly
- Works on macOS (APFS), Linux (ext4/xfs), Windows (NTFS) without conditional compilation
- APFS duplicate volumes are deduplicated in output — one line per unique (total, available) pair
- Error message includes actionable Docker cleanup commands
- No override — hard block, no env var escape hatch
- Other jjx commands (record, close, enroll, etc.) are not gated
- Testing: manual verification against current overlarge system (currently at 86%)

**[260402-0633] rough**

## Character

Mechanical integration work with a design edge — the `sysinfo` crate does the heavy lifting, but the guard must be wired into three JJK command paths cleanly without disrupting existing flow.

## Objective

Add a cross-platform disk space check that blocks `jjx_open`, `jjx_orient`, and `jjx_show` (the commands backing open, mount, and groom) when any filesystem with capacity > 10 GB is >= 85% full. Hard error, not a warning — user must free space before proceeding.

## Approach

- Add `sysinfo` crate dependency to the VVK/JJK Rust workspace
- Implement `check_disk_space()` using `sysinfo::Disks::new_with_refreshed_list()`
- For each disk: skip if `total_space < 10 GB`; compute `used_pct = (total - available) / total * 100`; collect violations at >= 85%
- On violation: print diagnostic (mount point, percentage, free bytes in human units), return hard error
- Gate the three command entry points: `jjx_open`, `jjx_orient`, `jjx_show`
- No platform-specific code — `sysinfo` handles Windows/macOS/Linux uniformly
- APFS shared-container redundancy is acceptable (multiple volumes may fire; noise not incorrectness)

## Acceptance

- `jjx_open` refuses with clear diagnostic when any qualifying disk is >= 85% full
- `jjx_orient` and `jjx_show` refuse similarly
- Works on macOS (APFS), Linux (ext4/xfs), Windows (NTFS) without conditional compilation
- Other jjx commands (record, close, enroll, etc.) are not gated — only session-start and orientation commands

### jjk-opus-model-gate (₢A2AAB) [complete]

**[260402-0810] complete**

## Character

Mechanical plumbing — add a string param to the MCP schema, thread it through dispatch, check a substring. The design decision (verbatim string, opus-only gate, per-call not per-session) is already made. Straightforward sonnet work.

## Objective

Add a `model` parameter to every `jjx_*` MCP call that carries the agent's verbatim model ID string (e.g., `claude-opus-4-6[1m]`). Gate all commands behind an opus-only check for now. Log the string for version audit.

## Design Decisions (settled)

- **Verbatim model string**: Agent passes exactly what its system prompt says (e.g., `claude-opus-4-6[1m]`). No tier mapping on the agent side.
- **Per-call, not per-session**: The `model` param is a sibling to `command`, `params`, and `officium` on the MCP tool schema. Reason: future JJK will delegate commands to different model tiers within a session.
- **Opus-only gate for now**: Server extracts tier by case-insensitive substring match for `opus`/`sonnet`/`haiku`. All commands require `opus`. Relax per-command later when evidence warrants.
- **No override mechanism**: Hard block, like the disk space guard.

## Approach

1. Add `model` field to `jjrm_JjxParams` struct (sibling to `command`, `params`, `officium`)
2. Server-side tier extraction: case-insensitive search for `opus`, `sonnet`, `haiku` in the model string. Unknown → treat as unauthorized.
3. Gate check early in dispatch — after officium validation, before command routing. All commands gated for now (including `jjx_open`).
4. Error message format:
   ```
   MODEL GATE — this command requires opus.

     Received model: claude-sonnet-4-5-20250514
     Extracted tier: sonnet

   Job Jockey commands currently require an opus-tier model.
   ```
5. Log model string: include in officium record and/or commit metadata where practical.
6. Update CLAUDE.md managed insert: instruct agents to pass their model ID string on every jjx call.

## Files

- `Tools/jjk/vov_veiled/src/jjrm_mcp.rs` — param struct, dispatch gate
- `Tools/vvk/vov_veiled/vvk-claude-context.md` or CLAUDE.md — agent cue to pass model string

## Acceptance

- `model` param accepted on every MCP call
- Missing or non-opus model string → hard error with clear diagnostic
- Verbatim model string logged (officium or commit)
- CLAUDE.md cue tells agents to pass their model ID
- Builds clean, existing tests pass

**[260402-0751] rough**

## Character

Mechanical plumbing — add a string param to the MCP schema, thread it through dispatch, check a substring. The design decision (verbatim string, opus-only gate, per-call not per-session) is already made. Straightforward sonnet work.

## Objective

Add a `model` parameter to every `jjx_*` MCP call that carries the agent's verbatim model ID string (e.g., `claude-opus-4-6[1m]`). Gate all commands behind an opus-only check for now. Log the string for version audit.

## Design Decisions (settled)

- **Verbatim model string**: Agent passes exactly what its system prompt says (e.g., `claude-opus-4-6[1m]`). No tier mapping on the agent side.
- **Per-call, not per-session**: The `model` param is a sibling to `command`, `params`, and `officium` on the MCP tool schema. Reason: future JJK will delegate commands to different model tiers within a session.
- **Opus-only gate for now**: Server extracts tier by case-insensitive substring match for `opus`/`sonnet`/`haiku`. All commands require `opus`. Relax per-command later when evidence warrants.
- **No override mechanism**: Hard block, like the disk space guard.

## Approach

1. Add `model` field to `jjrm_JjxParams` struct (sibling to `command`, `params`, `officium`)
2. Server-side tier extraction: case-insensitive search for `opus`, `sonnet`, `haiku` in the model string. Unknown → treat as unauthorized.
3. Gate check early in dispatch — after officium validation, before command routing. All commands gated for now (including `jjx_open`).
4. Error message format:
   ```
   MODEL GATE — this command requires opus.

     Received model: claude-sonnet-4-5-20250514
     Extracted tier: sonnet

   Job Jockey commands currently require an opus-tier model.
   ```
5. Log model string: include in officium record and/or commit metadata where practical.
6. Update CLAUDE.md managed insert: instruct agents to pass their model ID string on every jjx call.

## Files

- `Tools/jjk/vov_veiled/src/jjrm_mcp.rs` — param struct, dispatch gate
- `Tools/vvk/vov_veiled/vvk-claude-context.md` or CLAUDE.md — agent cue to pass model string

## Acceptance

- `model` param accepted on every MCP call
- Missing or non-opus model string → hard error with clear diagnostic
- Verbatim model string logged (officium or commit)
- CLAUDE.md cue tells agents to pass their model ID
- Builds clean, existing tests pass

**[260402-0751] rough**

## Character

Mechanical plumbing — add a string param to the MCP schema, thread it through dispatch, check a substring. The design decision (verbatim string, opus-only gate, per-call not per-session) is already made. Straightforward sonnet work.

## Objective

Add a `model` parameter to every `jjx_*` MCP call that carries the agent's verbatim model ID string (e.g., `claude-opus-4-6[1m]`). Gate all commands behind an opus-only check for now. Log the string for version audit.

## Design Decisions (settled)

- **Verbatim model string**: Agent passes exactly what its system prompt says (e.g., `claude-opus-4-6[1m]`). No tier mapping on the agent side.
- **Per-call, not per-session**: The `model` param is a sibling to `command`, `params`, and `officium` on the MCP tool schema. Reason: future JJK will delegate commands to different model tiers within a session.
- **Opus-only gate for now**: Server extracts tier by case-insensitive substring match for `opus`/`sonnet`/`haiku`. All commands require `opus`. Relax per-command later when evidence warrants.
- **No override mechanism**: Hard block, like the disk space guard.

## Approach

1. Add `model` field to `jjrm_JjxParams` struct (sibling to `command`, `params`, `officium`)
2. Server-side tier extraction: case-insensitive search for `opus`, `sonnet`, `haiku` in the model string. Unknown → treat as unauthorized.
3. Gate check early in dispatch — after officium validation, before command routing. All commands gated for now (including `jjx_open`).
4. Error message format:
   ```
   MODEL GATE — this command requires opus.

     Received model: claude-sonnet-4-5-20250514
     Extracted tier: sonnet

   Job Jockey commands currently require an opus-tier model.
   ```
5. Log model string: include in officium record and/or commit metadata where practical.
6. Update CLAUDE.md managed insert: instruct agents to pass their model ID string on every jjx call.

## Files

- `Tools/jjk/vov_veiled/src/jjrm_mcp.rs` — param struct, dispatch gate
- `Tools/vvk/vov_veiled/vvk-claude-context.md` or CLAUDE.md — agent cue to pass model string

## Acceptance

- `model` param accepted on every MCP call
- Missing or non-opus model string → hard error with clear diagnostic
- Verbatim model string logged (officium or commit)
- CLAUDE.md cue tells agents to pass their model ID
- Builds clean, existing tests pass

**[260402-0733] rough**

Drafted from ₢AhAAJ in ₣Ah.

Drafted from ₢AYAAA in ₣AY.

Add a test/probe command to VVX that validates the --model convention for model-aware CLI output.

## Goal

Build a `jjx_probe` (or similar) command that:
1. Accepts `--model opus|sonnet|haiku` flag
2. Emits the same semantic content but formatted differently per model tier:
   - **opus**: Rich, detailed output (tables, context, suggestions)
   - **sonnet**: Balanced output (structured but concise)
   - **haiku**: Minimal, machine-parseable output (bare facts, no decoration)
3. Returns a known test payload so we can grade whether the calling model correctly interprets it

## Experiment Design

The probe command should emit a structured scenario that requires the model to:
- Parse specific fields from the output
- Make a decision based on parsed data
- Report what it understood

This lets us test: "Given CLAUDE.md says `always pass --model`, does the model do it? And does jjx output tuned for that model tier actually improve comprehension?"

## CLAUDE.md Cue

Add to JJK managed insert:
```
When calling jjx_* commands, pass --model opus|sonnet|haiku matching your model tier.
```

Then observe across sessions whether models comply and whether tier-tuned output helps.

## Files
- Tools/jjk/vov_veiled/src/ (new probe command)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (register command)
- CLAUDE.md (add --model cue to JJK insert)

## Success Criteria
- Command builds and runs
- Three distinct output modes produce visibly different formatting
- Can be invoked in a test session to observe model behavior

**[260307-1155] rough**

Drafted from ₢AYAAA in ₣AY.

Add a test/probe command to VVX that validates the --model convention for model-aware CLI output.

## Goal

Build a `jjx_probe` (or similar) command that:
1. Accepts `--model opus|sonnet|haiku` flag
2. Emits the same semantic content but formatted differently per model tier:
   - **opus**: Rich, detailed output (tables, context, suggestions)
   - **sonnet**: Balanced output (structured but concise)
   - **haiku**: Minimal, machine-parseable output (bare facts, no decoration)
3. Returns a known test payload so we can grade whether the calling model correctly interprets it

## Experiment Design

The probe command should emit a structured scenario that requires the model to:
- Parse specific fields from the output
- Make a decision based on parsed data
- Report what it understood

This lets us test: "Given CLAUDE.md says `always pass --model`, does the model do it? And does jjx output tuned for that model tier actually improve comprehension?"

## CLAUDE.md Cue

Add to JJK managed insert:
```
When calling jjx_* commands, pass --model opus|sonnet|haiku matching your model tier.
```

Then observe across sessions whether models comply and whether tier-tuned output helps.

## Files
- Tools/jjk/vov_veiled/src/ (new probe command)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (register command)
- CLAUDE.md (add --model cue to JJK insert)

## Success Criteria
- Command builds and runs
- Three distinct output modes produce visibly different formatting
- Can be invoked in a test session to observe model behavior

**[260209-0708] rough**

Add a test/probe command to VVX that validates the --model convention for model-aware CLI output.

## Goal

Build a `jjx_probe` (or similar) command that:
1. Accepts `--model opus|sonnet|haiku` flag
2. Emits the same semantic content but formatted differently per model tier:
   - **opus**: Rich, detailed output (tables, context, suggestions)
   - **sonnet**: Balanced output (structured but concise)
   - **haiku**: Minimal, machine-parseable output (bare facts, no decoration)
3. Returns a known test payload so we can grade whether the calling model correctly interprets it

## Experiment Design

The probe command should emit a structured scenario that requires the model to:
- Parse specific fields from the output
- Make a decision based on parsed data
- Report what it understood

This lets us test: "Given CLAUDE.md says `always pass --model`, does the model do it? And does jjx output tuned for that model tier actually improve comprehension?"

## CLAUDE.md Cue

Add to JJK managed insert:
```
When calling jjx_* commands, pass --model opus|sonnet|haiku matching your model tier.
```

Then observe across sessions whether models comply and whether tier-tuned output helps.

## Files
- Tools/jjk/vov_veiled/src/ (new probe command)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (register command)
- CLAUDE.md (add --model cue to JJK insert)

## Success Criteria
- Command builds and runs
- Three distinct output modes produce visibly different formatting
- Can be invoked in a test session to observe model behavior

### size-guard-rename-awareness (₢A2AAC) [complete]

**[260403-1016] complete**

Drafted from ₢AhAAR in ₣Ah.

The jjx_record size guard counts full file content of staged additions without accounting for paired deletions in git renames. File moves (git detects as R status) are effectively zero new bytes, but the guard treats them as full-size additions, forcing unnecessary size_limit overrides.

## Problem

When jjx_record stages files that are renames/moves, git stores them efficiently (same blob, new path). But the size guard sums the byte count of all staged additions, triggering the limit even though the commit adds no new content.

## Solution

In the size guard calculation, detect rename pairs (git diff --cached --name-status shows R100 or similar) and exclude the destination file size from the total, or only count the net new bytes.

## Acceptance

- File moves/renames do not trigger size guard
- Genuinely large new content still triggers the guard
- Warning threshold still fires for large commits regardless

**[260402-0733] rough**

Drafted from ₢AhAAR in ₣Ah.

The jjx_record size guard counts full file content of staged additions without accounting for paired deletions in git renames. File moves (git detects as R status) are effectively zero new bytes, but the guard treats them as full-size additions, forcing unnecessary size_limit overrides.

## Problem

When jjx_record stages files that are renames/moves, git stores them efficiently (same blob, new path). But the size guard sums the byte count of all staged additions, triggering the limit even though the commit adds no new content.

## Solution

In the size guard calculation, detect rename pairs (git diff --cached --name-status shows R100 or similar) and exclude the destination file size from the total, or only count the net new bytes.

## Acceptance

- File moves/renames do not trigger size guard
- Genuinely large new content still triggers the guard
- Warning threshold still fires for large commits regardless

**[260313-1238] rough**

The jjx_record size guard counts full file content of staged additions without accounting for paired deletions in git renames. File moves (git detects as R status) are effectively zero new bytes, but the guard treats them as full-size additions, forcing unnecessary size_limit overrides.

## Problem

When jjx_record stages files that are renames/moves, git stores them efficiently (same blob, new path). But the size guard sums the byte count of all staged additions, triggering the limit even though the commit adds no new content.

## Solution

In the size guard calculation, detect rename pairs (git diff --cached --name-status shows R100 or similar) and exclude the destination file size from the total, or only count the net new bytes.

## Acceptance

- File moves/renames do not trigger size guard
- Genuinely large new content still triggers the guard
- Warning threshold still fires for large commits regardless

### redocket-emits-diff-output (₢A2AAD) [complete]

**[260403-1026] complete**

## Character
Small focused Rust change — add diff generation to an existing command's output path.

## Docket
`jjx_redocket` currently returns only "Revised N pace(s)". Add a diff of the old vs new docket content to the command output so the agent can immediately verify the reslate without a follow-up `jjx_show`.

### Output format
After the existing "Revised N pace(s)" line, emit a clearly labeled diff per revised pace:

```
Revised 2 pace(s)

--- ₢A1AAe reslate diff ---
- No remaining references to "consecration" (case-insensitive grep)
+ No remaining references to "consecration" (case-insensitive grep, excluding git history)
+ - `buz_group` descriptions in `rbz_zipper.sh` if any reference consecration

--- ₢A1AAb reslate diff ---
...
```

### Design notes
- Unified diff format (- old, + new) — familiar, compact
- Label each diff block with the coronet for multi-pace reslates
- The diff is between the previous revision's docket and the new one
- Keep it in stdout alongside the existing summary — no gazette output needed

### Verification
- `tt/rbtd-b.Build.sh` succeeds
- `tt/rbtd-t.Test.sh` passes
- Manual test: reslate a pace, verify diff appears and is accurate

### Acceptance
- `jjx_redocket` output includes readable diff per revised pace
- Existing "Revised N pace(s)" summary still present

**[260403-0940] rough**

## Character
Small focused Rust change — add diff generation to an existing command's output path.

## Docket
`jjx_redocket` currently returns only "Revised N pace(s)". Add a diff of the old vs new docket content to the command output so the agent can immediately verify the reslate without a follow-up `jjx_show`.

### Output format
After the existing "Revised N pace(s)" line, emit a clearly labeled diff per revised pace:

```
Revised 2 pace(s)

--- ₢A1AAe reslate diff ---
- No remaining references to "consecration" (case-insensitive grep)
+ No remaining references to "consecration" (case-insensitive grep, excluding git history)
+ - `buz_group` descriptions in `rbz_zipper.sh` if any reference consecration

--- ₢A1AAb reslate diff ---
...
```

### Design notes
- Unified diff format (- old, + new) — familiar, compact
- Label each diff block with the coronet for multi-pace reslates
- The diff is between the previous revision's docket and the new one
- Keep it in stdout alongside the existing summary — no gazette output needed

### Verification
- `tt/rbtd-b.Build.sh` succeeds
- `tt/rbtd-t.Test.sh` passes
- Manual test: reslate a pace, verify diff appears and is accurate

### Acceptance
- `jjx_redocket` output includes readable diff per revised pace
- Existing "Revised N pace(s)" summary still present

### buf-dispatch-fact-files (₢A2AAG) [abandoned]

**[260403-1854] abandoned**

## Character
Infrastructure with design judgment — the `buf_` prefix establishes a cross-project equivalence contract, not just constants. Requires care in deciding what crosses the firewall. Implementation itself is mechanical once the contract is clear.

## Docket

Establish `buf_` as BUK's cross-project equivalence contract: a stable interface that promises identical shape across independently-evolving BUK installations with different configuration.

### Concept

Two projects may have different `burc.env`, different station files, different `BURS_LOG_DIR` layouts. But both emit the same `buf_` surface. A consumer (like JJK relay) doesn't need the projects to be configured identically — it needs them to make the same promises.

### Deliverables

**New file `Tools/buk/buf_facts.sh`** — tinder constants defining the equivalence vocabulary:
- `BUF_fact_now_stamp` — fact-file name for dispatch timestamp
- `BUF_fact_log_hist` — fact-file name for historical log path

**New query script** (exact file TBD, likely `buf_` prefixed) — the attestation:
- Sources `burc.env` → chains station file → sources `buf_facts.sh`
- Emits tinder-constant-style key=value output:
  ```
  buf_output_dir=/absolute/path/to/output-buk/current
  buf_log_dir=/absolute/path/to/logs-buk
  buf_fact_now_stamp=buf_fact_now_stamp
  buf_fact_log_hist=buf_fact_log_hist
  ```
- This IS the firewall: consumers see `buf_` names only, never `BURC_*` or `BURS_*`
- Wire format is flat key=value, directly parseable by non-bash consumers (Rust `split_once('=')`)

**`Tools/buk/bud_dispatch.sh` changes:**
- Source `buf_facts.sh`
- After `BURD_OUTPUT_DIR` is created, write both facts:
  - `echo "${BURD_NOW_STAMP}" > "${BURD_OUTPUT_DIR}/${BUF_fact_now_stamp}"`
  - `echo "${BURD_LOG_HIST}" > "${BURD_OUTPUT_DIR}/${BUF_fact_log_hist}"`

**BUS0 updates:**
- Add `buf_` category declaration: BUK Fidelity (cross-project equivalence contract)
- Add quoins for the tinder constants and the attestation concept
- Link to existing `busff_fact_file` pattern

### Design Principles

- **Tinder constants** (not kindle) — values are pure string literals, no expansion
- **One prefix, one firewall** — `buf_` is both the tinder constants and the query output contract
- **Equivalence, not identity** — different configs, same promises
- **Only cross-boundary concepts enter `buf_`** — internal BUK details stay behind the firewall

### Not in Scope
- Relay consumption (₢A2AAF territory)
- Additional fact-files beyond now_stamp and log_hist (extend later as needed)
- Kindle ceremony (none — tinder only for constants, query script is standalone)

**[260403-1229] rough**

## Character
Infrastructure with design judgment — the `buf_` prefix establishes a cross-project equivalence contract, not just constants. Requires care in deciding what crosses the firewall. Implementation itself is mechanical once the contract is clear.

## Docket

Establish `buf_` as BUK's cross-project equivalence contract: a stable interface that promises identical shape across independently-evolving BUK installations with different configuration.

### Concept

Two projects may have different `burc.env`, different station files, different `BURS_LOG_DIR` layouts. But both emit the same `buf_` surface. A consumer (like JJK relay) doesn't need the projects to be configured identically — it needs them to make the same promises.

### Deliverables

**New file `Tools/buk/buf_facts.sh`** — tinder constants defining the equivalence vocabulary:
- `BUF_fact_now_stamp` — fact-file name for dispatch timestamp
- `BUF_fact_log_hist` — fact-file name for historical log path

**New query script** (exact file TBD, likely `buf_` prefixed) — the attestation:
- Sources `burc.env` → chains station file → sources `buf_facts.sh`
- Emits tinder-constant-style key=value output:
  ```
  buf_output_dir=/absolute/path/to/output-buk/current
  buf_log_dir=/absolute/path/to/logs-buk
  buf_fact_now_stamp=buf_fact_now_stamp
  buf_fact_log_hist=buf_fact_log_hist
  ```
- This IS the firewall: consumers see `buf_` names only, never `BURC_*` or `BURS_*`
- Wire format is flat key=value, directly parseable by non-bash consumers (Rust `split_once('=')`)

**`Tools/buk/bud_dispatch.sh` changes:**
- Source `buf_facts.sh`
- After `BURD_OUTPUT_DIR` is created, write both facts:
  - `echo "${BURD_NOW_STAMP}" > "${BURD_OUTPUT_DIR}/${BUF_fact_now_stamp}"`
  - `echo "${BURD_LOG_HIST}" > "${BURD_OUTPUT_DIR}/${BUF_fact_log_hist}"`

**BUS0 updates:**
- Add `buf_` category declaration: BUK Fidelity (cross-project equivalence contract)
- Add quoins for the tinder constants and the attestation concept
- Link to existing `busff_fact_file` pattern

### Design Principles

- **Tinder constants** (not kindle) — values are pure string literals, no expansion
- **One prefix, one firewall** — `buf_` is both the tinder constants and the query output contract
- **Equivalence, not identity** — different configs, same promises
- **Only cross-boundary concepts enter `buf_`** — internal BUK details stay behind the firewall

### Not in Scope
- Relay consumption (₢A2AAF territory)
- Additional fact-files beyond now_stamp and log_hist (extend later as needed)
- Kindle ceremony (none — tinder only for constants, query script is standalone)

**[260403-1216] rough**

## Character
Mechanical BUK infrastructure — new tinder constants file, two writes in dispatch, BUS0 quoin updates. Clear pattern to follow from existing fact-file usage.

## Docket

Add self-describing dispatch metadata to `BURD_OUTPUT_DIR` via the fact-file pattern.

### Deliverables

**New file `Tools/buk/buf_facts.sh`** — tinder constants (pure string literals, no kindle):
- `BUF_fact_now_stamp` — filename for the dispatch timestamp fact
- `BUF_fact_log_hist` — filename for the historical log path fact

**`Tools/buk/bud_dispatch.sh` changes:**
- Source `buf_facts.sh` (after SOURCED guard, before dispatch runs)
- After `BURD_OUTPUT_DIR` is created (after line 125), write both facts:
  - `echo "${BURD_NOW_STAMP}" > "${BURD_OUTPUT_DIR}/${BUF_fact_now_stamp}"`
  - `echo "${BURD_LOG_HIST}" > "${BURD_OUTPUT_DIR}/${BUF_fact_log_hist}"`

**BUS0 updates:**
- Add `buf_` category declaration in the mapping section comment block
- Add tinder constant quoins for `BUF_fact_now_stamp` and `BUF_fact_log_hist`
- Link them to the existing `busff_fact_file` pattern

### Design Notes

- Tinder constants (not kindle) because values are pure string literals with no variable expansion
- `buf_` prefix is open in the BUK namespace — "f for facts"
- Output dir is already created and cleared by dispatch before command execution — fact writes happen in the same setup window
- Any consumer (tests, relay, tooling) can source `buf_facts.sh` independently to know the filenames without pulling in dispatch

### Not in Scope
- Relay consumption (₢A2AAF territory)
- Additional fact-files beyond now_stamp and log_hist
- Kindle ceremony (none needed — tinder only)

### remote-execution-bind-send (₢A2AAF) [abandoned]

**[260403-1854] abandoned**

## Character
Design-first infrastructure — API shape matters more than implementation. Resolved design, pending JJS0 quoin pass before mounting.

## Docket

Add remote command execution as a JJK primitive: four commands for configuring a remote target, executing commands, running tabtargets, and fetching output.

### Commands

| Command | Params | Returns |
|---------|--------|---------|
| `jjx_bind` | `{host, user, directory}` | Confirmation (SSH probe verifies connectivity) |
| `jjx_send` | `{command, timeout}` | Exit code + merged output inline |
| `jjx_relay` | `{tabtarget, timeout}` | Exit code + manifest of fetchable refs |
| `jjx_fetch` | `{ref}` | File content |

### Resolved Design Decisions

**Param shape (jjx_bind):** Flat `{host, user, directory}`. Explicit, validatable, matches jjx style.

**Output streams:** Merged stdout+stderr. SSH inherently merges on the client side. One stream — what a human would see. For `jjx_send`, returned inline. For `jjx_relay`, written to transcript file in officium.

**Timeout:** Required `timeout` parameter in seconds on both `jjx_send` and `jjx_relay`. No optionals, no defaults. Caller states their expectation.

**Connectivity check:** `jjx_bind` performs SSH probe at bind time. Fail fast.

**Working directory:** Always the bound directory. No override param. Remote command runs inside `cd <directory> && <command>`, formed as a single argv element via Rust `Command` API — no local shell quoting issues.

**Error reporting:** Binary success/fail. Exit code + captured output. No distinction between SSH failure, command failure, or timeout. Output tells you what went wrong.

**Tabtarget field:** Full `tt/` path with space-delimited arguments in one string. Caller writes exactly what they'd type at a terminal. Remote shell parses naturally.

### Relay Remote Discovery Protocol

Before executing the remote tabtarget, `jjx_relay`:

1. Reads `.buk/burc.env` on the remote side to get `BURC_STATION_FILE` and `BURC_OUTPUT_ROOT_DIR`
2. Chains to the station file (path from `BURC_STATION_FILE` relative to project root) to get `BURS_LOG_DIR`
3. Executes the tabtarget
4. Fetches fact-files from `${BURC_OUTPUT_ROOT_DIR}/current/` — including `BUF_fact_now_stamp` and `BUF_fact_log_hist` (provided by ₢A2AAG)
5. Returns manifest with exit code, fact-file contents, and fetchable refs to log files

**Dependency:** ₢A2AAG (buf-dispatch-fact-files) provides the self-describing output dir that relay consumes.

### Officium Integration

**Bind state:** `jjx_bind` writes remote config to a file in the officium directory. `jjx_send`/`jjx_relay` read it. Latest bind wins. Absolution cleans up automatically.

**Relay output:** `jjx_relay` stashes transcript and log files in the officium directory. `jjx_fetch` retrieves by reference. Stash cleared on next `jjx_relay` call. Available until then.

### Implementation Notes

SSH invocation via Rust `std::process::Command` — argv-level argument passing eliminates local shell quoting entirely. The remote shell interprets the command string as-is.

### Before Mounting

Requires a JJS0 quoin pass: mint quoin names, attribute references, and linked terms for the four new commands and their concepts. The vocabulary shapes the implementation API surface.

### Not in Scope
- Theurge integration
- The comprehension agent itself (₣A3 territory)
- Remote file transfer (scp/rsync)
- Concurrent remote targets
- Separate stdout/stderr capture

## References
- ₢A2AAG — buf-dispatch-fact-files (prerequisite: self-describing output dir)
- JJS0 — vocabulary isolation, quoin model (pending pass)
- ₣A3 paddock — onboarding comprehension testing (first consumer)
- Officium lifecycle — ephemeral state patterns
- BUS0 `busff_fact_file` — the fact-file pattern relay consumes

**[260403-1217] rough**

## Character
Design-first infrastructure — API shape matters more than implementation. Resolved design, pending JJS0 quoin pass before mounting.

## Docket

Add remote command execution as a JJK primitive: four commands for configuring a remote target, executing commands, running tabtargets, and fetching output.

### Commands

| Command | Params | Returns |
|---------|--------|---------|
| `jjx_bind` | `{host, user, directory}` | Confirmation (SSH probe verifies connectivity) |
| `jjx_send` | `{command, timeout}` | Exit code + merged output inline |
| `jjx_relay` | `{tabtarget, timeout}` | Exit code + manifest of fetchable refs |
| `jjx_fetch` | `{ref}` | File content |

### Resolved Design Decisions

**Param shape (jjx_bind):** Flat `{host, user, directory}`. Explicit, validatable, matches jjx style.

**Output streams:** Merged stdout+stderr. SSH inherently merges on the client side. One stream — what a human would see. For `jjx_send`, returned inline. For `jjx_relay`, written to transcript file in officium.

**Timeout:** Required `timeout` parameter in seconds on both `jjx_send` and `jjx_relay`. No optionals, no defaults. Caller states their expectation.

**Connectivity check:** `jjx_bind` performs SSH probe at bind time. Fail fast.

**Working directory:** Always the bound directory. No override param. Remote command runs inside `cd <directory> && <command>`, formed as a single argv element via Rust `Command` API — no local shell quoting issues.

**Error reporting:** Binary success/fail. Exit code + captured output. No distinction between SSH failure, command failure, or timeout. Output tells you what went wrong.

**Tabtarget field:** Full `tt/` path with space-delimited arguments in one string. Caller writes exactly what they'd type at a terminal. Remote shell parses naturally.

### Relay Remote Discovery Protocol

Before executing the remote tabtarget, `jjx_relay`:

1. Reads `.buk/burc.env` on the remote side to get `BURC_STATION_FILE` and `BURC_OUTPUT_ROOT_DIR`
2. Chains to the station file (path from `BURC_STATION_FILE` relative to project root) to get `BURS_LOG_DIR`
3. Executes the tabtarget
4. Fetches fact-files from `${BURC_OUTPUT_ROOT_DIR}/current/` — including `BUF_fact_now_stamp` and `BUF_fact_log_hist` (provided by ₢A2AAG)
5. Returns manifest with exit code, fact-file contents, and fetchable refs to log files

**Dependency:** ₢A2AAG (buf-dispatch-fact-files) provides the self-describing output dir that relay consumes.

### Officium Integration

**Bind state:** `jjx_bind` writes remote config to a file in the officium directory. `jjx_send`/`jjx_relay` read it. Latest bind wins. Absolution cleans up automatically.

**Relay output:** `jjx_relay` stashes transcript and log files in the officium directory. `jjx_fetch` retrieves by reference. Stash cleared on next `jjx_relay` call. Available until then.

### Implementation Notes

SSH invocation via Rust `std::process::Command` — argv-level argument passing eliminates local shell quoting entirely. The remote shell interprets the command string as-is.

### Before Mounting

Requires a JJS0 quoin pass: mint quoin names, attribute references, and linked terms for the four new commands and their concepts. The vocabulary shapes the implementation API surface.

### Not in Scope
- Theurge integration
- The comprehension agent itself (₣A3 territory)
- Remote file transfer (scp/rsync)
- Concurrent remote targets
- Separate stdout/stderr capture

## References
- ₢A2AAG — buf-dispatch-fact-files (prerequisite: self-describing output dir)
- JJS0 — vocabulary isolation, quoin model (pending pass)
- ₣A3 paddock — onboarding comprehension testing (first consumer)
- Officium lifecycle — ephemeral state patterns
- BUS0 `busff_fact_file` — the fact-file pattern relay consumes

**[260403-1143] rough**

## Character
Design-first infrastructure — API shape matters more than implementation. Resolved design, pending JJS0 quoin pass before mounting.

## Docket

Add remote command execution as a JJK primitive: four commands for configuring a remote target, executing commands, running tabtargets, and fetching output.

### Commands

| Command | Params | Returns |
|---------|--------|---------|
| `jjx_bind` | `{host, user, directory}` | Confirmation (SSH probe verifies connectivity) |
| `jjx_send` | `{command, timeout}` | Exit code + merged output inline |
| `jjx_relay` | `{tabtarget, timeout}` | Exit code + manifest of fetchable refs |
| `jjx_fetch` | `{ref}` | File content |

### Resolved Design Decisions

**Param shape (jjx_bind):** Flat `{host, user, directory}`. Explicit, validatable, matches jjx style.

**Output streams:** Merged stdout+stderr. SSH inherently merges on the client side. One stream — what a human would see. For `jjx_send`, returned inline. For `jjx_relay`, written to transcript file in officium.

**Timeout:** Required `timeout` parameter in seconds on both `jjx_send` and `jjx_relay`. No optionals, no defaults. Caller states their expectation.

**Connectivity check:** `jjx_bind` performs SSH probe at bind time. Fail fast.

**Working directory:** Always the bound directory. No override param. Remote command runs inside `cd <directory> && <command>`, formed as a single argv element via Rust `Command` API — no local shell quoting issues.

**Error reporting:** Binary success/fail. Exit code + captured output. No distinction between SSH failure, command failure, or timeout. Output tells you what went wrong.

**Tabtarget field:** Full `tt/` path with space-delimited arguments in one string. Caller writes exactly what they'd type at a terminal. Remote shell parses naturally.

### Officium Integration

**Bind state:** `jjx_bind` writes remote config to a file in the officium directory. `jjx_send`/`jjx_relay` read it. Latest bind wins. Absolution cleans up automatically.

**Relay output:** `jjx_relay` stashes transcript and log files in the officium directory. `jjx_fetch` retrieves by reference. Stash cleared on next `jjx_relay` call. Available until then.

### Implementation Notes

SSH invocation via Rust `std::process::Command` — argv-level argument passing eliminates local shell quoting entirely. The remote shell interprets the command string as-is.

### Before Mounting

Requires a JJS0 quoin pass: mint quoin names, attribute references, and linked terms for the four new commands and their concepts. The vocabulary shapes the implementation API surface.

### Not in Scope
- Theurge integration
- The comprehension agent itself (₣A3 territory)
- Remote file transfer (scp/rsync)
- Concurrent remote targets
- Separate stdout/stderr capture

## References
- JJS0 — vocabulary isolation, quoin model (pending pass)
- ₣A3 paddock — onboarding comprehension testing (first consumer)
- Officium lifecycle — ephemeral state patterns

**[260403-1122] rough**

## Character
Design-first infrastructure — API shape matters more than implementation. The feature is general but the first consumer (onboarding comprehension testing) will validate the design.

## Docket

Add remote command execution as a JJK primitive: configure a remote target, then execute commands on it.

### Vocabulary

| Layer | Configure | Execute |
|-------|-----------|---------|
| Upper verb (equestrian) | tether | lunge |
| jjx_ operation (boring) | jjx_bind | jjx_send |

### Semantics

**jjx_bind** — store remote target configuration (host, user, directory) in officium. Latest call wins — no concurrent remotes. Subsequent `jjx_send` calls use whatever was last bound.

**jjx_send** — execute a command string on the bound remote, return stdout. Stateless per invocation — no session, no channel, no connection persistence. Transport is SSH but the caller doesn't know or care.

### Officium integration

Remote config lives in the officium directory alongside gazette files. Lifecycle is automatic — when the officium is absolved, the config goes with it. No new cleanup story needed.

### Open design questions

- Param shape for `jjx_bind`: flat `{host, user, directory}` or a single connection string?
- Does `jjx_send` return stderr separately or merged with stdout?
- Timeout behavior — what happens when a remote command hangs?
- Does `jjx_bind` verify connectivity (SSH probe) or is that the caller's problem?
- Should `jjx_send` accept an optional working directory override, or always use the bound directory?
- Error reporting: SSH failure vs command failure vs timeout — how to distinguish?

### Motivation

Onboarding comprehension testing (₣A3) needs to run tabtargets on a remote machine with controlled credential state. The comprehension agent should think in terms of "run this command" not "SSH to this user at this host." JJK absorbs the transport, the agent sees only `jjx_send`.

Also general: release qualification on clean machines, cross-platform testing, any "fresh checkout" validation.

### Not in scope for this pace
- Theurge integration (theurge stays container-focused)
- The comprehension agent itself (₣A3 territory)
- Remote file transfer (scp/rsync) — may be needed later but start with exec only
- Concurrent remote targets

## References
- JJS0 — vocabulary isolation principle (upper verbs vs jjx_ operations)
- ₣A3 paddock — onboarding comprehension testing motivation
- Officium lifecycle — `jjx_open`, `jjx_absolve`

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 H gazette-fenced-code-block-awareness
  2 E rename-jjk-hallmark-to-brand
  3 A jjk-disk-space-guard
  4 B jjk-opus-model-gate
  5 C size-guard-rename-awareness
  6 D redocket-emits-diff-output

HEABCD
·xxx·x jjrm_mcp.rs
·xx··· lib.rs
·····x jjrtl_tally.rs, rbw-DA.DirectorAbjuresConsecration.sh, rbw-DE.DirectorEnshrinesVessel.sh, rbw-DI.DirectorInscribesReliquary.sh, rbw-DJ.DirectorJettisonsImage.sh, rbw-DO.DirectorOrdainsConsecration.sh, rbw-DV.DirectorVouchesConsecrations.sh, rbw-Dt.DirectorTalliesConsecrations.sh, rbw-GC.GovernorChartersRetriever.sh, rbw-GF.GovernorForfeitsServiceAccount.sh, rbw-GK.GovernorKnightsDirector.sh, rbw-Gl.GovernorListsServiceAccounts.sh, rbw-LK.LocalKludge.sh, rbw-PL.PayorLeviesDepot.sh, rbw-PM.PayorMantlesGovernor.sh, rbw-PU.PayorUnmakesDepot.sh, rbw-Pl.PayorListsDepots.sh, rbw-QR.QualifyRelease.sh, rbw-Qf.QualifyFast.sh, rbw-RpF.RetrieverPlumbsFull.sh, rbw-Rpc.RetrieverPlumbsCompact.sh, rbw-Rs.RetrieverSummonsConsecration.sh, rbw-Rw.RetrieverWrestsImage.sh, rbw-Tk.KludgeCycle.tadmor.sh, rbw-To.OrdainCycle.tadmor.sh, rbw-aC.GovernorChartersRetriever.sh, rbw-aF.GovernorForfeitsServiceAccount.sh, rbw-aK.GovernorKnightsDirector.sh, rbw-aL.GovernorListsServiceAccounts.sh, rbw-aM.PayorMantlesGovernor.sh, rbw-ca.CrucibleActive.sh, rbw-cic.CrucibleIsCharged.sh, rbw-dE.DirectorEnshrinesVessel.sh, rbw-dI.DirectorInscribesReliquary.sh, rbw-dL.PayorLeviesDepot.sh, rbw-dU.PayorUnmakesDepot.sh, rbw-dl.PayorListsDepots.sh, rbw-hA.DirectorAbjuresHallmark.sh, rbw-hO.DirectorOrdainsHallmark.sh, rbw-hV.DirectorVouchesHallmarks.sh, rbw-hk.LocalKludge.sh, rbw-hpc.RetrieverPlumbsCompact.sh, rbw-hpf.RetrieverPlumbsFull.sh, rbw-hs.RetrieverSummonsHallmark.sh, rbw-ht.DirectorTalliesHallmarks.sh, rbw-iJ.DirectorJettisonsImage.sh, rbw-iw.RetrieverWrestsImage.sh, rbw-tK.KludgeCycle.tadmor.sh, rbw-tO.OrdainCycle.tadmor.sh, rbw-tf.QualifyFast.sh, rbw-tr.QualifyRelease.sh
····x· vvcg_guard.rs, vvtg_guard.rs
···x·· jjk-claude-context.md
··x··· Cargo.lock, Cargo.toml, jjrdk_diskcheck.rs
·x···· JJS0_JobJockeySpec.adoc, JJSCCH-chalk.adoc, JJSCLD-landing.adoc, JJSCNC-notch.adoc, JJSCRN-rein.adoc, JJSRPS-persist.adoc, VOS0-VoxObscuraSpec.adoc, jjrn_notch.rs, jjrnc_notch.rs, jjrs_steeplechase.rs, jjtn_notch.rs, jjtrn_rein.rs, jjts_steeplechase.rs, vob_build.sh, vofe_emplace.rs, vofr_release.rs, vorm_main.rs, vovr_registry.json, vvcc_format.rs, vvcp_probe.rs
x····· jjrz_gazette.rs, jjtz_gazette.rs

Commit swim lanes (x = commit affiliated with pace):

  1 A jjk-disk-space-guard
  2 B jjk-opus-model-gate
  3 C size-guard-rename-awareness
  4 E rename-jjk-hallmark-to-brand
  5 D redocket-emits-diff-output
  6 H gazette-fenced-code-block-awareness

123456789abcdefghijklmnopqrs
···xx·x····x················  A  4c
········xx··················  B  2c
··········x·······x·········  C  2c
···············xxx··········  E  3c
···················xx·······  D  2c
························xx··  H  2c
```

## Steeplechase

### 2026-04-03 18:54 - Heat - T

remote-execution-bind-send

### 2026-04-03 18:54 - Heat - T

buf-dispatch-fact-files

### 2026-04-03 18:51 - ₢A2AAH - W

Added fenced code block awareness to gazette parser. Track ``` toggle state in jjrz_parse; skip notice boundary detection inside fences. Prevents bare # lines in code examples from being misinterpreted as gazette headers. 4 new tests covering hash-in-fence, slug-like content in fence, notice-after-fence, and round-trip. 322 tests pass.

### 2026-04-03 18:49 - ₢A2AAH - n

Add fenced code block awareness to gazette parser: track ``` state and skip notice boundary detection inside fences. 4 new tests, 322 total pass.

### 2026-04-03 18:47 - Heat - D

AmAAJ → ₢A2AAH

### 2026-04-03 12:16 - Heat - S

buf-dispatch-fact-files

### 2026-04-03 11:22 - Heat - S

remote-execution-bind-send

### 2026-04-03 10:26 - ₢A2AAD - W

Added LCS-based line diff output to jjx_redocket so reslate results show old vs new docket changes per pace. 8 unit tests for the diff function, 318 tests pass.

### 2026-04-03 10:24 - ₢A2AAD - n

Add diff output to jjx_redocket: LCS-based line diff of old vs new docket emitted per revised pace, with 8 unit tests. 318 tests pass.

### 2026-04-03 10:16 - ₢A2AAC - W

Verified size guard rename awareness implementation: -M flag detects renames, exact renames cost 0, edited renames cost only the diff. Six tests cover all cost model paths including rename-specific cases. 310 tests pass.

### 2026-04-03 10:13 - ₢A2AAE - W

Renamed hallmark to brand across VVK/VVC/JJK/VOK. Updated Rust code, bash, specs (JJS0, VOS0), registry JSON key (hallmarks→vovr_brands), brand file key (vvbh_hallmark→vvbf_brand), and pb_paneboard02 brand file. 333 tests pass. Verified brand lookup end-to-end: new binary correctly reads vovr_brands from registry.

### 2026-04-03 10:09 - ₢A2AAE - n

Rename hallmark to brand in JJK and VOK specs. All quoin anchors, attributes, and references updated. 333 tests pass.

### 2026-04-03 10:04 - ₢A2AAE - n

Rename hallmark to brand in VOK/VVC/JJK Rust code, registry JSON key, and build script. 310 tests pass, build green. Specs and pb brand file still pending.

### 2026-04-03 09:49 - Heat - S

rename-jjk-hallmark-to-brand

### 2026-04-03 09:40 - Heat - S

redocket-emits-diff-output

### 2026-04-02 09:12 - Heat - n

Unify annotation syntax: remove Strachey bracket form, prefix-discriminated form is now sole annotation mechanism, with real motif examples in code templates and deduplicated context file

### 2026-04-02 08:57 - ₢A2AAA - n

Include Cargo.lock update for sysinfo crate addition (missed in disk space guard commit)

### 2026-04-02 08:37 - ₢A2AAC - n

Size guard rename awareness: batch rename detection via -M flag so exact renames cost 0 and edited renames cost only the diff, with tests for both cases

### 2026-04-02 08:10 - ₢A2AAB - W

Added model gate: bare String model field on jjrm_JjxParams (required by serde), case-insensitive tier extraction (opus/sonnet/haiku/unknown), opus-only gate checked before all dispatch including jjx_open, stderr model logging for audit, updated jjk-claude-context.md to document the fourth parameter. Verified gate blocks sonnet and haiku agents with formatted diagnostic.

### 2026-04-02 08:09 - ₢A2AAB - n

Add model gate: bare String model field on JjxParams, tier extraction, opus-only gate before all dispatch

### 2026-04-02 07:51 - Heat - T

jjk-opus-model-gate

### 2026-04-02 07:39 - ₢A2AAA - W

Added cross-platform disk space guard using sysinfo crate. New jjrdk_diskcheck module blocks jjx_open/orient/show when any 10GB+ disk is >=85% full, with APFS volume deduplication and actionable Docker cleanup hints in error output.

### 2026-04-02 07:33 - Heat - D

restring 2 paces from ₣Ah

### 2026-04-02 07:13 - ₢A2AAA - n

Restore 85% threshold, make disk survey permanent in both Ok and Err paths, change return type to Result<String, String>

### 2026-04-02 07:05 - ₢A2AAA - n

Add disk space guard: new jjrdk_diskcheck module with sysinfo crate, wired into jjx_open/orient/show to hard-block when any 10GB+ disk is >=85% full, with APFS dedup and actionable Docker cleanup hints

### 2026-04-02 07:01 - Heat - f

racing

### 2026-04-02 06:33 - Heat - S

jjk-disk-space-guard

### 2026-04-02 06:21 - Heat - N

jjk-v3-6-minor-issues

