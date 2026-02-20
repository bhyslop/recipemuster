# Heat Trophy: jjk-features-polish

**Firemark:** ₣AE
**Created:** 260117
**Retired:** 260124
**Status:** retired

> NOTE: JJSA renamed to JJS0 (top-level spec '0' suffix convention). Filename references in this trophy are historical.

## Paddock

# Paddock: jjk-features-polish

## Context

JJK quality-of-life improvements and data model evolution. These paces improve the daily experience of working with Job Jockey and prepare for future capabilities.

**Themes:**

1. **Better context on mount** — Show recent steeplechase history when starting work
2. **CLI polish** — Fix argument handling, add conveniences
3. **Data model evolution** — Enable pace renaming, capture commit SHAs for recovery

## Architecture

Five paces in rough dependency order:

1. **₢AEAAA — create-heat-rein-command** — `/jjc-heat-rein` slash command + saddle recent work enhancement
2. **₢AEAAB — parade-pace-silks-lookup** — Fix `--pace` arg: coronet normalization + silks fallback
3. **₢AEAAC — steeplechase-version-tracking** — Add brand field to steeplechase entries
4. **₢AEAAD — prime-merges-direction-into-spec** — Merge direction into tack_text, deprecate separate field
5. **₢AEAAE — tack-silks-and-commit-migration** — Move silks to Tack (enables rename), add commit SHA

## References

- Tools/jjk/vov_veiled/JJSA-GallopsData.adoc — JJSA spec (data model changes)
- Tools/vok/vov_veiled/RCG-RustCodingGuide.md — Rust naming conventions
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs — Pace/Tack structs
- Tools/jjk/vov_veiled/src/jjrq_query.rs — Query operations (saddle, parade)
- Tools/jjk/vov_veiled/src/jjrs_steeplechase.rs — Steeplechase parsing
- .claude/commands/jjc-heat-mount.md — Mount command (context display)

## Key Constraints

1. **Backwards compatibility** — Lazy migration for data model changes
2. **Serde aliases** — Accept old field names on read, write new format
3. **RCG compliance** — New functions use appropriate prefixes

## Steeplechase

### 2026-01-17 - Heat Created

Restrung 5 paces from ₣AA (vok-fresh-install-release) to separate JJK polish from MVP delivery.

## Paces

### vvc-guard-binary-file-size (₢AEAAF) [complete]

**[260118-1110] complete**

Refactored vvtg_guard.rs tests to eliminate set_current_dir() race conditions by passing repo_dir to test functions and using get_test_base() helper for temp directory fallback. All 4 tests now pass in parallel.

**[260118-1009] bridled**

Fix parallel test race condition in vvtg_guard.rs tests.

Two tests (vvtg_deleted_file_size, vvtg_regression_tarball) use std::env::set_current_dir() which is process-wide, causing race conditions when tests run in parallel.

Options:
1. Add --test-threads=1 to vob_test cargo invocation (quick fix)
2. Refactor tests to pass working directory to vvcg_run instead of using set_current_dir (proper fix)

All 9 tests pass with --test-threads=1.

Prior work completed:
- Build infrastructure refactored (vof_features.sh, vob_test, vow-t tabtarget) - committed 926bc81e
- Path canonicalization for BUD_TEMP_DIR/BUD_OUTPUT_DIR in vvce_env.rs - committed 42f60d58
- Test helper canonicalizes temp path - committed 42f60d58
- Tools/temp-buk debris deleted

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vvtg_guard.rs (new), vvcg_guard.rs, lib.rs (3 files)
Steps:
1. Make zvvcg_get_diff_size and zvvcg_StagedFile pub(crate) in vvcg_guard.rs for testability
2. Create vvtg_guard.rs with tests:
   - vvtg_text_file_size — text file reports actual blob size
   - vvtg_binary_file_size — binary file reports actual blob size, not diff output
   - vvtg_deleted_file_size — deleted file returns 0
   - vvtg_large_binary_blocked — integration test using vvce_env().temp_dir: create git repo, stage 100KB binary, verify guard returns 1
3. Add #[cfg(test)] mod vvtg_guard; to lib.rs after vvcg_guard declaration
4. Run /jjc-pace-wrap AEAAF on success
Verify: tt/vvw-t.TestVVX.sh

**[260118-0842] bridled**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing Requirements

### Unit Tests (vvct_guard.rs)

Create test file with cases:
1. Text file — verify diff size matches expectations
2. Binary file detection — verify binary files report actual blob size, not diff output size
3. Large binary rejection — verify guard blocks files over limit
4. Mixed staging — text + binary, verify total includes actual binary size

### Integration Test (Rust, in vvct_guard.rs)

Use `vvce_env().temp_dir` from `Tools/vvc/src/vvce_env.rs` for test repo location:

```rust
use crate::vvce_env;

#[test]
fn test_guard_blocks_large_binary() {
    let temp = vvce_env().temp_dir.join("guard-test");
    // Create git repo in temp
    // Stage large binary file (>50KB)
    // Run guard with default limit
    // Assert exit code 1 (BLOCKED)
}
```

Benefits of using vvce_env():
- BUK tabtarget manages directory lifecycle
- Consistent location across test runs
- Fails fast if invoked outside tabtarget context

### Regression Test

Reproduce the actual failure:
1. Create a tarball similar to vvk-parcel-1000.tar.gz (~2MB)
2. Stage it in test repo under vvce_env().temp_dir
3. Run guard with default 50KB limit
4. Verify BLOCKED (exit code 1)

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

## Acceptance Criteria

- [ ] Guard correctly reports actual size for binary files
- [ ] Unit tests cover text, binary, and mixed cases
- [ ] Integration test uses vvce_env().temp_dir for test repos
- [ ] Regression test proves the original failure would now be caught

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vvtg_guard.rs (new), vvcg_guard.rs, lib.rs (3 files)
Steps:
1. Make zvvcg_get_diff_size and zvvcg_StagedFile pub(crate) in vvcg_guard.rs for testability
2. Create vvtg_guard.rs with tests:
   - vvtg_text_file_size — text file reports actual blob size
   - vvtg_binary_file_size — binary file reports actual blob size, not diff output
   - vvtg_deleted_file_size — deleted file returns 0
   - vvtg_large_binary_blocked — integration test using vvce_env().temp_dir: create git repo, stage 100KB binary, verify guard returns 1
3. Add #[cfg(test)] mod vvtg_guard; to lib.rs after vvcg_guard declaration
4. Run /jjc-pace-wrap AEAAF on success
Verify: tt/vvw-t.TestVVX.sh

**[260118-0841] rough**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing Requirements

### Unit Tests (vvct_guard.rs)

Create test file with cases:
1. Text file — verify diff size matches expectations
2. Binary file detection — verify binary files report actual blob size, not diff output size
3. Large binary rejection — verify guard blocks files over limit
4. Mixed staging — text + binary, verify total includes actual binary size

### Integration Test (Rust, in vvct_guard.rs)

Use `vvce_env().temp_dir` from `Tools/vvc/src/vvce_env.rs` for test repo location:

```rust
use crate::vvce_env;

#[test]
fn test_guard_blocks_large_binary() {
    let temp = vvce_env().temp_dir.join("guard-test");
    // Create git repo in temp
    // Stage large binary file (>50KB)
    // Run guard with default limit
    // Assert exit code 1 (BLOCKED)
}
```

Benefits of using vvce_env():
- BUK tabtarget manages directory lifecycle
- Consistent location across test runs
- Fails fast if invoked outside tabtarget context

### Regression Test

Reproduce the actual failure:
1. Create a tarball similar to vvk-parcel-1000.tar.gz (~2MB)
2. Stage it in test repo under vvce_env().temp_dir
3. Run guard with default 50KB limit
4. Verify BLOCKED (exit code 1)

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

## Acceptance Criteria

- [ ] Guard correctly reports actual size for binary files
- [ ] Unit tests cover text, binary, and mixed cases
- [ ] Integration test uses vvce_env().temp_dir for test repos
- [ ] Regression test proves the original failure would now be caught

**[260118-0837] bridled**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing Requirements

### Unit Tests (vvct_guard.rs)

Create test file with cases:
1. Text file — verify diff size matches expectations
2. Binary file detection — verify binary files report actual blob size, not diff output size
3. Large binary rejection — verify guard blocks files over limit
4. Mixed staging — text + binary, verify total includes actual binary size

### Integration Test (Rust, in vvct_guard.rs)

Use `vvce_env().temp_dir` from `Tools/vvc/src/vvce_env.rs` for test repo location:

```rust
use crate::vvce_env;

#[test]
fn test_guard_blocks_large_binary() {
    let temp = vvce_env().temp_dir.join("guard-test");
    // Create git repo in temp
    // Stage large binary file (>50KB)
    // Run guard with default limit
    // Assert exit code 1 (BLOCKED)
}
```

Benefits of using vvce_env():
- BUK tabtarget manages directory lifecycle
- Consistent location across test runs
- Fails fast if invoked outside tabtarget context

### Regression Test

Reproduce the actual failure:
1. Create a tarball similar to vvk-parcel-1000.tar.gz (~2MB)
2. Stage it in test repo under vvce_env().temp_dir
3. Run guard with default 50KB limit
4. Verify BLOCKED (exit code 1)

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

## Acceptance Criteria

- [ ] Guard correctly reports actual size for binary files
- [ ] Unit tests cover text, binary, and mixed cases
- [ ] Integration test uses vvce_env().temp_dir for test repos
- [ ] Regression test proves the original failure would now be caught

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vvcg_guard.rs, vvct_guard.rs, lib.rs (3 files)
Steps:
1. Fix zvvcg_get_diff_size() to use git ls-files --cached -s to get blob SHA, then git cat-file -s for actual size
2. Handle edge cases: deleted files (size 0), new files, modified files
3. Create vvct_guard.rs with tests: text file size, binary file actual size, large binary rejection, mixed staging
4. Add integration test using vvce_env().temp_dir that creates temp git repo, stages large binary, verifies BLOCKED
5. Update lib.rs to include vvct_guard module (cfg test)
6. Run /jjc-pace-wrap AEAAF on successful completion
Verify: tt/vvw-t.TestVVX.sh

**[260118-0834] rough**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing Requirements

### Unit Tests (vvct_guard.rs)

Create test file with cases:
1. Text file — verify diff size matches expectations
2. Binary file detection — verify binary files report actual blob size, not diff output size
3. Large binary rejection — verify guard blocks files over limit
4. Mixed staging — text + binary, verify total includes actual binary size

### Integration Test (Rust, in vvct_guard.rs)

Use `vvce_env().temp_dir` from `Tools/vvc/src/vvce_env.rs` for test repo location:

```rust
use crate::vvce_env;

#[test]
fn test_guard_blocks_large_binary() {
    let temp = vvce_env().temp_dir.join("guard-test");
    // Create git repo in temp
    // Stage large binary file (>50KB)
    // Run guard with default limit
    // Assert exit code 1 (BLOCKED)
}
```

Benefits of using vvce_env():
- BUK tabtarget manages directory lifecycle
- Consistent location across test runs
- Fails fast if invoked outside tabtarget context

### Regression Test

Reproduce the actual failure:
1. Create a tarball similar to vvk-parcel-1000.tar.gz (~2MB)
2. Stage it in test repo under vvce_env().temp_dir
3. Run guard with default 50KB limit
4. Verify BLOCKED (exit code 1)

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

## Acceptance Criteria

- [ ] Guard correctly reports actual size for binary files
- [ ] Unit tests cover text, binary, and mixed cases
- [ ] Integration test uses vvce_env().temp_dir for test repos
- [ ] Regression test proves the original failure would now be caught

**[260118-0805] rough**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing Requirements

### Unit Tests (vvct_guard.rs)

Create test file with cases:
1. Text file — verify diff size matches expectations
2. Binary file detection — verify binary files report actual blob size, not diff output size
3. Large binary rejection — verify guard blocks files over limit
4. Mixed staging — text + binary, verify total includes actual binary size

### Integration Test Script

Create `tt/vvc-t.TestGuard.sh` or equivalent that:
1. Creates a temp git repo
2. Stages a small text file (should pass)
3. Stages a large binary file (>50KB) (should block)
4. Stages a binary file just under limit (should pass)
5. Verifies guard exit codes match expectations

### Regression Test

Reproduce the actual failure:
1. Create a tarball similar to vvk-parcel-1000.tar.gz (~2MB)
2. Stage it
3. Run guard with default 50KB limit
4. Verify BLOCKED (exit code 1)

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

## Acceptance Criteria

- [ ] Guard correctly reports actual size for binary files
- [ ] Unit tests cover text, binary, and mixed cases
- [ ] Integration test demonstrates end-to-end rejection
- [ ] Regression test proves the original failure would now be caught

**[260117-1424] rough**

Fix VVC guard to properly measure binary file sizes.

## The Bug

`zvvcg_get_diff_size()` in vvcg_guard.rs measures git diff output length, not actual file size:

```rust
let output = Command::new("git")
    .args(["diff", "--cached", "--", path])
    ...
Ok(output.stdout.len() as u64)  // counts diff output bytes, not file size
```

For binary files, git diff outputs:
```
Binary files /dev/null and b/vvk-parcel-1000.tar.gz differ
```

That is ~60 bytes regardless of actual file size. A 2MB tarball slips through a 50KB guard.

## The Fix

For new/modified files, get actual staged content size:
- Use `git ls-files --cached -s` to get blob SHA
- Use `git cat-file -s <sha>` to get blob size
- Or use `git diff --cached --numstat` which shows actual byte counts

## Files

- Tools/vvc/src/vvcg_guard.rs

## Testing

1. Stage a large binary file (>50KB)
2. Run vvx_commit with default limit
3. Verify guard rejects it

## Prevention

This bug allowed vvk-parcel-1000.tar.gz (2MB) to be committed. Guard should have caught it.

### tack-silks-and-commit-migration (₢AEAAE) [complete]

**[260118-1334] complete**

Update JJD-GallopsData.adoc for tack structural migration (accepts legacy, writes canonical).

## Changes

### 1. Update mapping section

Add new attributes:
```
:jjdkm_silks:  <<jjdkm_silks,silks>>
:jjdkm_commit: <<jjdkm_commit,commit>>
```

Remove or deprecate:
```
:jjdpm_silks: ...  // Move to Tack section
```

### 2. Move jjdpm_silks from Pace to Tack

**Pace record (jjdpr_pace):**
- Remove jjdpm_silks member documentation
- Add note: "Silks moved to Tack; current silks = tacks[0].silks"

**Tack record (jjdkr_tack):**
- Add jjdkm_silks as required member
- Definition: "Display name at this point in time. Enables rename history. Current pace silks derived from tacks[0].silks."

### 3. Add jjdkm_commit to Tack

**Tack record (jjdkr_tack):**
- Add jjdkm_commit as required member
- Type: 7-character hex string
- Definition: "Commit SHA at tack creation. Captures repo state when plan was written. Value '0000000' indicates unknown (migrated data)."

### 4. Add migration section

Add under Serialization (or new Migration section):

**Legacy Format Acceptance:**

During migration period, implementations MUST accept legacy format on read:
- Pace with `silks` field → copy to all tacks missing silks, discard from pace
- Tack without `silks` → inherit from pace (error if pace also missing silks)
- Tack without `commit` → use "0000000"

Implementations MUST write only canonical format:
- Pace without `silks` field
- Tack with `silks` (required)
- Tack with `commit` (required, "0000000" if unknown)

### 5. Update query operation outputs

**jjdo_saddle:**
- `pace_silks` derived from `tacks[0].silks` (not pace.silks)

**jjdo_parade:**
- Pace silks derived from `tacks[0].silks`

**jjdo_muster:**
- No change (shows heat silks, not pace silks)

### 6. Search and update all jjdpm_silks references

Any place that references pace silks derivation should now say "tacks[0].silks".

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Scope

Spec only. No Rust changes in this pace.

**[260118-1324] rough**

Update JJD-GallopsData.adoc for tack structural migration (accepts legacy, writes canonical).

## Changes

### 1. Update mapping section

Add new attributes:
```
:jjdkm_silks:  <<jjdkm_silks,silks>>
:jjdkm_commit: <<jjdkm_commit,commit>>
```

Remove or deprecate:
```
:jjdpm_silks: ...  // Move to Tack section
```

### 2. Move jjdpm_silks from Pace to Tack

**Pace record (jjdpr_pace):**
- Remove jjdpm_silks member documentation
- Add note: "Silks moved to Tack; current silks = tacks[0].silks"

**Tack record (jjdkr_tack):**
- Add jjdkm_silks as required member
- Definition: "Display name at this point in time. Enables rename history. Current pace silks derived from tacks[0].silks."

### 3. Add jjdkm_commit to Tack

**Tack record (jjdkr_tack):**
- Add jjdkm_commit as required member
- Type: 7-character hex string
- Definition: "Commit SHA at tack creation. Captures repo state when plan was written. Value '0000000' indicates unknown (migrated data)."

### 4. Add migration section

Add under Serialization (or new Migration section):

**Legacy Format Acceptance:**

During migration period, implementations MUST accept legacy format on read:
- Pace with `silks` field → copy to all tacks missing silks, discard from pace
- Tack without `silks` → inherit from pace (error if pace also missing silks)
- Tack without `commit` → use "0000000"

Implementations MUST write only canonical format:
- Pace without `silks` field
- Tack with `silks` (required)
- Tack with `commit` (required, "0000000" if unknown)

### 5. Update query operation outputs

**jjdo_saddle:**
- `pace_silks` derived from `tacks[0].silks` (not pace.silks)

**jjdo_parade:**
- Pace silks derived from `tacks[0].silks`

**jjdo_muster:**
- No change (shows heat silks, not pace silks)

### 6. Search and update all jjdpm_silks references

Any place that references pace silks derivation should now say "tacks[0].silks".

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Scope

Spec only. No Rust changes in this pace.

**[260118-1313] rough**

Update JJD-GallopsData.adoc for tack structural migration (accepts legacy, writes canonical).

## Changes

### 1. Move jjdpm_silks from Pace to Tack

**Pace record:**
- Remove jjdpm_silks member documentation
- Note: "Silks moved to Tack; see jjdkm_silks"

**Tack record:**
- Add jjdkm_silks as required member
- Definition: "Display name at this point in time. Enables rename history."

### 2. Add jjdkm_commit to Tack

**Tack record:**
- Add jjdkm_commit as required member
- Type: 7-character hex string
- Definition: "Commit SHA at tack creation. Value '0000000' indicates unknown (migrated data)."

### 3. Add migration note

Add section under Serialization or new Migration section:

**Legacy Format Acceptance:**
During migration period, implementations MUST accept legacy format on read:
- Pace with `silks` field → copy to all tacks, discard from pace
- Tack without `silks` → inherit from pace
- Tack without `commit` → use "0000000"

Implementations MUST write only canonical format:
- Pace without `silks` field
- Tack with `silks` (required)
- Tack with `commit` (required, "0000000" if unknown)

### 4. Update jjdpm_silks → jjdkm_silks references

Search document for references to pace silks derivation (e.g., in jjdo_parade, jjdo_saddle output).
Update to: "Current silks = tacks[0].silks"

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Scope

Spec only. No Rust changes in this pace.

**[260117-1410] rough**

Drafted from ₢AAABO in ₣AA.

Migrate silks from Pace to Tack and add commit SHA tracking.

## Changes

### 1. Move silks to Tack (required field)

**Current:**
```rust
struct jjrg_Pace {
    silks: String,
    tacks: Vec<jjrg_Tack>,
}
```

**New:**
```rust
struct jjrg_Pace {
    tacks: Vec<jjrg_Tack>,  // silks removed
}

struct jjrg_Tack {
    ts: String,
    state: PaceState,
    text: String,
    silks: String,          // NEW: required
    direction: Option<String>,
}
```

Current silks = `tacks[0].silks` (same pattern as state derivation).

### 2. Add commit SHA to Tack (optional field)

```rust
struct jjrg_Tack {
    // ... existing fields ...
    commit: Option<String>,  // NEW: HEAD SHA at tack write time
}
```

Capture via `git rev-parse HEAD` at tack creation.

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — spec updates
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs — struct changes
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — tack creation sites
- All slash commands that reference silks lookup

## Migration

Lazy migration on read:
- If old format (silks on Pace): copy to each tack, remove from Pace on write
- If tack missing commit: leave as None (historical data)

## Enables

- Pace renaming via `/jjc-pace-reslate` with --silks flag
- Point-in-time file recovery via `git show <commit>:path`

**[260117-1350] rough**

Migrate silks from Pace to Tack and add commit SHA tracking.

## Changes

### 1. Move silks to Tack (required field)

**Current:**
```rust
struct jjrg_Pace {
    silks: String,
    tacks: Vec<jjrg_Tack>,
}
```

**New:**
```rust
struct jjrg_Pace {
    tacks: Vec<jjrg_Tack>,  // silks removed
}

struct jjrg_Tack {
    ts: String,
    state: PaceState,
    text: String,
    silks: String,          // NEW: required
    direction: Option<String>,
}
```

Current silks = `tacks[0].silks` (same pattern as state derivation).

### 2. Add commit SHA to Tack (optional field)

```rust
struct jjrg_Tack {
    // ... existing fields ...
    commit: Option<String>,  // NEW: HEAD SHA at tack write time
}
```

Capture via `git rev-parse HEAD` at tack creation.

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — spec updates
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs — struct changes
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — tack creation sites
- All slash commands that reference silks lookup

## Migration

Lazy migration on read:
- If old format (silks on Pace): copy to each tack, remove from Pace on write
- If tack missing commit: leave as None (historical data)

## Enables

- Pace renaming via `/jjc-pace-reslate` with --silks flag
- Point-in-time file recovery via `git show <commit>:path`

### tack-struct-rust-migration (₢AEAAI) [complete]

**[260118-1417] complete**

Migrated silks from Pace to Tack level with custom deserializer for legacy format. Added commit field (0000000 default). Created jjrg_make_tack constructor. Extended RCG with Constant, Constructor, Comment, and File Size disciplines.

**[260118-1405] rough**

Migration test note

**[260118-1324] rough**

Implement Rust serde for tack structural migration.

## Changes

### 1. Update Tack struct (jjrg_gallops.rs)

```rust
pub struct Tack {
    pub ts: String,
    pub state: PaceState,
    pub text: String,
    pub silks: String,              // NEW - required
    pub commit: String,             // NEW - required, "0000000" if unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}
```

### 2. Update Pace struct (jjrg_gallops.rs)

```rust
pub struct Pace {
    // silks: String,  // REMOVED - now on Tack
    pub tacks: Vec<Tack>,
}
```

Note: Pace struct has NO silks field. Migration happens during deserialization.

### 3. Custom deserialization for Pace

Implement custom Deserialize that:

1. Deserialize raw JSON object
2. Check for `silks` field on pace (legacy format indicator)
3. Deserialize `tacks` array
4. For each tack:
   - If tack missing `silks`: copy from pace.silks (legacy migration)
   - If tack missing `commit`: use "0000000" (legacy migration)
5. Return Pace { tacks } — no silks field on struct

Error if: tack missing silks AND pace missing silks (malformed data).

### 4. Serialization

Standard derive works. Pace has no silks field to serialize.
Tack always has silks and commit (required fields).

### 5. Update tack creation sites

**jjx_slate** (new pace with first tack):
- `silks`: from --silks argument (required for new pace)
- `commit`: capture via `git rev-parse --short=7 HEAD`

**jjx_tally** (add tack to existing pace):
- `silks`: inherit from `tacks[0].silks` (current silks)
- `commit`: capture via `git rev-parse --short=7 HEAD`

**jjx_draft** (move pace, add note tack):
- `silks`: inherit from source `tacks[0].silks`
- `commit`: capture via `git rev-parse --short=7 HEAD`

### 6. Helper function for commit capture

```rust
fn capture_commit_sha() -> String {
    // Run: git rev-parse --short=7 HEAD
    // Return 7-char hex, or "0000000" on error
}
```

### 7. Update silks accessors

Any code that accessed `pace.silks` must now use `pace.tacks[0].silks`.
Search for `.silks` usage in query operations (saddle, parade, etc.).

## Verification

```bash
tt/vow-b.Build.sh
./tt/vvw-r.RunVVX.sh jjx_validate
./tt/vvw-r.RunVVX.sh jjx_muster
./tt/vvw-r.RunVVX.sh jjx_parade AE --format order
```

## Files

- Tools/jjk/vov_veiled/src/jjrg_gallops.rs (struct changes, custom serde)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (tack creation sites)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (silks accessors)

## Scope

Migration only. No new features (rename flag, etc.).

**[260118-1314] rough**

Implement Rust serde for tack structural migration.

## Changes

### 1. Update Tack struct (jjrg_gallops.rs)

```rust
pub struct Tack {
    pub ts: String,
    pub state: PaceState,
    pub text: String,
    pub silks: String,              // NEW - required
    pub commit: String,             // NEW - required, "0000000" if unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}
```

### 2. Update Pace struct (jjrg_gallops.rs)

```rust
pub struct Pace {
    // silks: String,  // REMOVED - now on Tack
    pub tacks: Vec<Tack>,
}
```

### 3. Custom deserialization

Implement custom Deserialize for Pace that:
- If `silks` field exists on Pace: copy to all tacks missing silks, then discard
- If Tack missing `silks`: inherit from Pace (error if Pace also missing)
- If Tack missing `commit`: use "0000000"

### 4. Serialization

Standard derive should work - just ensure:
- Pace never serializes `silks`
- Tack always serializes `silks` and `commit`

### 5. Update all tack creation sites

Find all places that create Tack instances:
- jjx_slate (new pace → new tack)
- jjx_tally (add tack)
- jjx_draft (copy with note)

Each must now provide:
- `silks`: inherit from previous tack[0].silks or pace.silks (during migration)
- `commit`: capture via `git rev-parse --short=7 HEAD`

## Verification

```bash
tt/vow-b.Build.sh
./tt/vvw-r.RunVVX.sh jjx_validate
./tt/vvw-r.RunVVX.sh jjx_muster  # Should work with migrated data
```

## Files

- Tools/jjk/vov_veiled/src/jjrg_gallops.rs (struct changes, custom serde)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (tack creation sites)

## Scope

Migration only. No new features (rename flag, etc.).

### tack-struct-jjd-cleanup (₢AEAAJ) [complete]

**[260118-1441] complete**

Removed Legacy Format Acceptance section and simplified jjdkm_commit definition. Verification confirmed zero legacy/migration references remain.

**[260118-1419] bridled**

Remove legacy format acceptance from JJD spec.

## Context

After tack-struct-rust-migration (₢AEAAI) completes, legacy format
no longer exists in the data. The spec should reflect this.

## Changes

### 1. Remove Legacy Format Acceptance section

Delete lines 602-614 (the entire "=== Legacy Format Acceptance" section).

### 2. Simplify jjdkm_commit definition

Line 511 currently says:
  Value `0000000` indicates unknown (migrated data).

Change to:
  Value `0000000` indicates commit was unavailable at creation time.

(Removes "migrated" reference while keeping the sentinel documented.)

## Verification

Search for: "legacy", "migration", "migrated"
Expected: zero matches

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Scope

Spec cleanup only. Bridleable.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/JJD-GallopsData.adoc (1 file)
Steps:
1. Delete === Legacy Format Acceptance section (from === Legacy Format Acceptance through blank line before == Arguments)
2. Change jjdkm_commit definition from indicates unknown (migrated data) to indicates commit was unavailable at creation time
3. Grep verify: no matches for legacy, migration, migrated
Verify: grep -i legacy|migrat Tools/jjk/vov_veiled/JJD-GallopsData.adoc (expect no output)

**[260118-1349] rough**

Remove legacy format acceptance from JJD spec.

## Context

After tack-struct-rust-migration (₢AEAAI) completes, legacy format
no longer exists in the data. The spec should reflect this.

## Changes

### 1. Remove Legacy Format Acceptance section

Delete lines 602-614 (the entire "=== Legacy Format Acceptance" section).

### 2. Simplify jjdkm_commit definition

Line 511 currently says:
  Value `0000000` indicates unknown (migrated data).

Change to:
  Value `0000000` indicates commit was unavailable at creation time.

(Removes "migrated" reference while keeping the sentinel documented.)

## Verification

Search for: "legacy", "migration", "migrated"
Expected: zero matches

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Scope

Spec cleanup only. Bridleable.

**[260118-1316] rough**

Remove legacy format acceptance from JJD spec.

## Context

After tack-struct-rust-migration (₢AEAAI) completes and gallops.json is migrated, the legacy format no longer exists in the data. The spec should reflect this.

## Changes

### 1. Remove migration note

Delete the "Legacy Format Acceptance" section added in ₢AEAAE.

### 2. Simplify member documentation

- jjdkm_silks: Remove any "inherited from pace" language
- jjdkm_commit: Remove "0000000 for migrated data" emphasis (keep format spec)

### 3. Verify no legacy references remain

Search for:
- "legacy"
- "migration"
- "Pace.silks" or "pace silks" (should only reference tack now)

## Files

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Depends On

- ₢AEAAE (JJD migration spec)
- ₢AEAAI (Rust migration impl) — must complete first so data is migrated

## Scope

Spec cleanup only. No Rust changes.

### tack-struct-rust-cleanup (₢AEAAK) [complete]

**[260118-1448] complete**

Replaced custom Deserialize impl for jjrg_Pace with standard derive, removed 65 lines of legacy migration code and updated doc comments. All tests pass.

**[260118-1444] bridled**

Remove legacy format deserialization from Rust.

## Context

After tack-struct-jjd-cleanup (₢AEAAJ) confirms the spec no longer permits legacy format, the Rust code can drop that support.

## Changes

### 1. Simplify Pace deserialization

Remove custom Deserialize impl that handled:
- Pace with `silks` field
- Tacks without `silks`
- Tacks without `commit`

Replace with standard derive:
```rust
#[derive(Deserialize, Serialize)]
pub struct Pace {
    pub tacks: Vec<Tack>,
}
```

### 2. Simplify Tack deserialization

Standard derive should now work:
```rust
#[derive(Deserialize, Serialize)]
pub struct Tack {
    pub ts: String,
    pub state: PaceState,
    pub text: String,
    pub silks: String,
    pub commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}
```

### 3. Remove migration helper code

Delete any:
- `migrate_pace()` functions
- `ensure_tack_silks()` helpers
- Comments about "legacy format"

## Verification

```bash
tt/vow-b.Build.sh
./tt/vvw-r.RunVVX.sh jjx_validate
./tt/vvw-r.RunVVX.sh jjx_muster
```

## Files

- Tools/jjk/vov_veiled/src/jjrg_gallops.rs

## Depends On

- ₢AEAAI (Rust migration impl)
- ₢AEAAJ (JJD cleanup) — confirms legacy is gone

## Scope

Code cleanup only. No new features.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/jjk/vov_veiled/src/jjrg_gallops.rs (1 file)
Steps:
1. Add Deserialize to jjrg_Pace derive macro (existing Serialize only)
2. Delete the custom impl<'de> Deserialize<'de> for jjrg_Pace block (65 lines)
3. Remove legacy migration mention from JJRG_UNKNOWN_COMMIT doc comment
4. Remove legacy migration note from jjrg_Pace doc comment
5. Build: tt/vow-b.Build.sh
6. Test: tt/vow-t.Test.sh
7. Validate: ./tt/vvw-r.RunVVX.sh jjx_validate
8. Muster: ./tt/vvw-r.RunVVX.sh jjx_muster
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh && ./tt/vvw-r.RunVVX.sh jjx_validate

**[260118-1320] rough**

Remove legacy format deserialization from Rust.

## Context

After tack-struct-jjd-cleanup (₢AEAAJ) confirms the spec no longer permits legacy format, the Rust code can drop that support.

## Changes

### 1. Simplify Pace deserialization

Remove custom Deserialize impl that handled:
- Pace with `silks` field
- Tacks without `silks`
- Tacks without `commit`

Replace with standard derive:
```rust
#[derive(Deserialize, Serialize)]
pub struct Pace {
    pub tacks: Vec<Tack>,
}
```

### 2. Simplify Tack deserialization

Standard derive should now work:
```rust
#[derive(Deserialize, Serialize)]
pub struct Tack {
    pub ts: String,
    pub state: PaceState,
    pub text: String,
    pub silks: String,
    pub commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}
```

### 3. Remove migration helper code

Delete any:
- `migrate_pace()` functions
- `ensure_tack_silks()` helpers
- Comments about "legacy format"

## Verification

```bash
tt/vow-b.Build.sh
./tt/vvw-r.RunVVX.sh jjx_validate
./tt/vvw-r.RunVVX.sh jjx_muster
```

## Files

- Tools/jjk/vov_veiled/src/jjrg_gallops.rs

## Depends On

- ₢AEAAI (Rust migration impl)
- ₢AEAAJ (JJD cleanup) — confirms legacy is gone

## Scope

Code cleanup only. No new features.

### create-heat-rein-command (₢AEAAA) [complete]

**[260118-1458] complete**

Created /jjc-heat-rein slash command, added recent_work to saddle output with commit SHA, updated /jjc-heat-mount to display recent work. JJD spec and tests updated.

**[260117-1410] rough**

Drafted from ₢AAAAo in ₣AA.

Create /jjc-heat-rein slash command AND enhance saddle with recent work context.

## Part 1: Slash Command

Create .claude/commands/jjc-heat-rein.md:
- Arguments: firemark (required), --limit (optional, default 20)
- Calls: ./tt/vvw-r.RunVVX.sh jjx_rein <FIREMARK> --limit <N>
- Formats JSON output as human-readable steeplechase history
- Shows: timestamp, pace silks (if pace-level), action type, subject

## Part 2: Saddle Enhancement

Modify jjx_saddle to include recent steeplechase entries in output.

### JJD Spec Update

Add to jjdo_saddle output:
- `recent_work`: array of last N steeplechase entries (default 10)

### Rust Implementation

In jjrq_query.rs saddle function:
1. After getting pace info, call jjrs_get_entries() with limit 10
2. Include entries in SaddleResult struct
3. Serialize in JSON output

### Slash Command Update

Update /jjc-heat-mount to display recent work section:
- Show 5-10 recent entries before presenting the pace
- Helps Claude orient to what was just accomplished
- Format: "Recent work on this heat:" followed by entries

## Rationale

Steeplechase history provides crucial context for Claude when starting work:
- What was just completed
- Pattern of recent activity
- Continuity across sessions

## Files

- .claude/commands/jjc-heat-rein.md (new)
- .claude/commands/jjc-heat-mount.md (update)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (saddle output spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (saddle implementation)

**[260117-1344] rough**

Create /jjc-heat-rein slash command AND enhance saddle with recent work context.

## Part 1: Slash Command

Create .claude/commands/jjc-heat-rein.md:
- Arguments: firemark (required), --limit (optional, default 20)
- Calls: ./tt/vvw-r.RunVVX.sh jjx_rein <FIREMARK> --limit <N>
- Formats JSON output as human-readable steeplechase history
- Shows: timestamp, pace silks (if pace-level), action type, subject

## Part 2: Saddle Enhancement

Modify jjx_saddle to include recent steeplechase entries in output.

### JJD Spec Update

Add to jjdo_saddle output:
- `recent_work`: array of last N steeplechase entries (default 10)

### Rust Implementation

In jjrq_query.rs saddle function:
1. After getting pace info, call jjrs_get_entries() with limit 10
2. Include entries in SaddleResult struct
3. Serialize in JSON output

### Slash Command Update

Update /jjc-heat-mount to display recent work section:
- Show 5-10 recent entries before presenting the pace
- Helps Claude orient to what was just accomplished
- Format: "Recent work on this heat:" followed by entries

## Rationale

Steeplechase history provides crucial context for Claude when starting work:
- What was just completed
- Pattern of recent activity
- Continuity across sessions

## Files

- .claude/commands/jjc-heat-rein.md (new)
- .claude/commands/jjc-heat-mount.md (update)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (saddle output spec)
- Tools/jjk/vov_veiled/src/jjrq_query.rs (saddle implementation)

**[260117-1225] abandoned**

Blocked by stale installation-identifier pace

**[260116-1309] rough**

Create /jjc-heat-rein slash command.

## Prerequisites

- ₢AAAAc (installation-identifier) complete — jjx_rein no longer requires --brand

## Implementation

Create .claude/commands/jjc-heat-rein.md:
- Arguments: firemark (required)
- Calls: ./tt/vvw-r.RunVVX.sh jjx_rein <FIREMARK>
- Parses JSON output into human-readable steeplechase history

## Reference

See JJD jjdo_rein spec for jjx_rein output format and behavior.

### parade-pace-silks-lookup (₢AEAAB) [complete]

**[260118-1510] complete**

Added zjjrq_resolve_pace() helper with Coronet normalization and silks fallback for parade --pace. Updated JJD spec to document the behavior.

**[260118-1505] bridled**

Drafted from ₢AAAAl in ₣AA.

Fix jjx_parade --pace argument handling:

1. **Coronet normalization**: Add Coronet::parse() + .display() before lookup (matches jjx_rail/jjx_tally pattern). Fixes: `--pace AAAAk` currently fails because map keys are `₢AAAAk`.

2. **Silks fallback**: If coronet lookup fails, iterate heat.paces to find pace by silks match. Allows `--pace jjrc-commit-helper` as convenience.

3. **JJD update**: Document the silks fallback behavior in jjx_parade's --pace argument description.

Files: Tools/jjk/veiled/src/jjrq_query.rs (implementation), Tools/jjk/JJD-GallopsData.adoc (spec)

*Direction:* Agent: sonnet
Cardinality: 2 parallel + sequential build
Files: jjrq_query.rs, JJD-GallopsData.adoc (2 files)
Steps:
1. Agent A (sonnet): In jjrq_query.rs, import Coronet from jjrf_favor. Add zjjrq_resolve_pace() helper that: (a) tries Coronet::jjrf_parse() + .jjrf_display() for lookup, (b) if parse fails or lookup fails, iterates heat.paces to find by tacks[0].silks match. Apply helper in Detail format branch replacing direct heat.paces.get().
2. Agent B (sonnet): In JJD-GallopsData.adoc, update jjdo_parade --pace argument description to document: accepts coronet with or without prefix, falls back to silks match if coronet not found.
3. Sequential: tt/vow-b.Build.sh && tt/vow-t.Test.sh
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260117-1410] rough**

Drafted from ₢AAAAl in ₣AA.

Fix jjx_parade --pace argument handling:

1. **Coronet normalization**: Add Coronet::parse() + .display() before lookup (matches jjx_rail/jjx_tally pattern). Fixes: `--pace AAAAk` currently fails because map keys are `₢AAAAk`.

2. **Silks fallback**: If coronet lookup fails, iterate heat.paces to find pace by silks match. Allows `--pace jjrc-commit-helper` as convenience.

3. **JJD update**: Document the silks fallback behavior in jjx_parade's --pace argument description.

Files: Tools/jjk/veiled/src/jjrq_query.rs (implementation), Tools/jjk/JJD-GallopsData.adoc (spec)

**[260116-1125] rough**

Fix jjx_parade --pace argument handling:

1. **Coronet normalization**: Add Coronet::parse() + .display() before lookup (matches jjx_rail/jjx_tally pattern). Fixes: `--pace AAAAk` currently fails because map keys are `₢AAAAk`.

2. **Silks fallback**: If coronet lookup fails, iterate heat.paces to find pace by silks match. Allows `--pace jjrc-commit-helper` as convenience.

3. **JJD update**: Document the silks fallback behavior in jjx_parade's --pace argument description.

Files: Tools/jjk/veiled/src/jjrq_query.rs (implementation), Tools/jjk/JJD-GallopsData.adoc (spec)

**[260116-1125] rough**

Add silks lookup fallback to jjx_parade --pace: if coronet lookup fails, try matching against pace silks in the heat. Allows --pace jjrc-commit-helper as convenience.

### steeplechase-version-tracking (₢AEAAC) [complete]

**[260118-2009] complete**

Added hallmark version tracking to JJ commit messages with brand file / registry+git fallback; fixed registry lookup to read nested hallmarks object

**[260118-1959] bridled**

Add hallmark to JJ commit messages for VVK version tracking.

## Commit Format Change

Current: `jjb:BRAND:IDENTITY[:ACTION]: message`
New: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`

Example (installed): `jjb:RBM:1009:₢AEAAC:n: Fix the bug`
Example (dev): `jjb:RBM:1009-abc1234:₢AEAAC:n: Fix the bug`

## Hallmark Source Logic

1. Try `.vvk/vvbf_brand.json` → if exists, use `vvbh_hallmark` (4 digits)
2. If missing (Kit Forge) → read `Tools/vok/vov_veiled/vovr_registry.json`, find max hallmark, get `git rev-parse --short HEAD`, format as `{hallmark}-{commit}`

## Changes

### 1. jjrn_notch.rs - Read hallmark at commit time

Add function zjjrn_get_hallmark() that:
- Tries to read `.vvk/vvbf_brand.json` and extract `vvbh_hallmark`
- If missing, reads `Tools/vok/vov_veiled/vovr_registry.json`, finds max hallmark key
- If missing, runs `git rev-parse --short HEAD` for 7-char commit
- Returns `NNNN` (installed) or `NNNN-xxxxxxx` (dev)

Update all format functions to include hallmark:
- jjrn_format_notch_prefix
- jjrn_format_chalk_message
- jjrn_format_heat_message
- jjrn_format_heat_discussion

### 2. jjrs_steeplechase.rs - Parse hallmark from commits

Add `hallmark: Option<String>` to jjrs_SteeplechaseEntry.

Update zjjrs_parse_new_format() to parse hallmark between brand and identity:
- After brand, check next segment
- If starts with ₢ or ₣ → no hallmark (old format), parse as identity
- Otherwise → hallmark field, then parse identity
- Pattern: `\d{4}` or `\d{4}-[a-f0-9]{7}`

### 3. JJD-GallopsData.adoc - Document new format

Update "Commit Message Patterns" section:
- New format: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`
- Add HALLMARK bullet: "Version identifier. Format: NNNN (installed) or NNNN-xxxxxxx (dev)"
- Document source logic (brand file vs registry + git HEAD)

Update rein output documentation:
- Add hallmark field to JSON structure example

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs
- Tools/jjk/vov_veiled/src/jjrs_steeplechase.rs
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Verification

- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Create test commit in Kit Forge, verify hallmark is NNNN-xxxxxxx format
- Verify jjx_rein output includes hallmark field

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrn_notch.rs, jjrs_steeplechase.rs, JJD-GallopsData.adoc (3 files)
Steps:
1. Add zjjrn_get_hallmark() that tries .vvk/vvbf_brand.json first, falls back to registry max + git HEAD
2. Update jjrn_format_notch_prefix, jjrn_format_chalk_message, jjrn_format_heat_message, jjrn_format_heat_discussion to include hallmark
3. Add hallmark: Option<String> to jjrs_SteeplechaseEntry
4. Update zjjrs_parse_new_format(): after brand, if next segment starts with identity prefix (₢/₣) treat as old format, else parse as hallmark then identity
5. Update JJD Commit Message Patterns: new format, HALLMARK bullet with NNNN vs NNNN-xxxxxxx, source logic
6. Update JJD rein output JSON to include hallmark field
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260118-1959] rough**

Add hallmark to JJ commit messages for VVK version tracking.

## Commit Format Change

Current: `jjb:BRAND:IDENTITY[:ACTION]: message`
New: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`

Example (installed): `jjb:RBM:1009:₢AEAAC:n: Fix the bug`
Example (dev): `jjb:RBM:1009-abc1234:₢AEAAC:n: Fix the bug`

## Hallmark Source Logic

1. Try `.vvk/vvbf_brand.json` → if exists, use `vvbh_hallmark` (4 digits)
2. If missing (Kit Forge) → read `Tools/vok/vov_veiled/vovr_registry.json`, find max hallmark, get `git rev-parse --short HEAD`, format as `{hallmark}-{commit}`

## Changes

### 1. jjrn_notch.rs - Read hallmark at commit time

Add function zjjrn_get_hallmark() that:
- Tries to read `.vvk/vvbf_brand.json` and extract `vvbh_hallmark`
- If missing, reads `Tools/vok/vov_veiled/vovr_registry.json`, finds max hallmark key
- If missing, runs `git rev-parse --short HEAD` for 7-char commit
- Returns `NNNN` (installed) or `NNNN-xxxxxxx` (dev)

Update all format functions to include hallmark:
- jjrn_format_notch_prefix
- jjrn_format_chalk_message
- jjrn_format_heat_message
- jjrn_format_heat_discussion

### 2. jjrs_steeplechase.rs - Parse hallmark from commits

Add `hallmark: Option<String>` to jjrs_SteeplechaseEntry.

Update zjjrs_parse_new_format() to parse hallmark between brand and identity:
- After brand, check next segment
- If starts with ₢ or ₣ → no hallmark (old format), parse as identity
- Otherwise → hallmark field, then parse identity
- Pattern: `\d{4}` or `\d{4}-[a-f0-9]{7}`

### 3. JJD-GallopsData.adoc - Document new format

Update "Commit Message Patterns" section:
- New format: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`
- Add HALLMARK bullet: "Version identifier. Format: NNNN (installed) or NNNN-xxxxxxx (dev)"
- Document source logic (brand file vs registry + git HEAD)

Update rein output documentation:
- Add hallmark field to JSON structure example

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs
- Tools/jjk/vov_veiled/src/jjrs_steeplechase.rs
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc

## Verification

- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Create test commit in Kit Forge, verify hallmark is NNNN-xxxxxxx format
- Verify jjx_rein output includes hallmark field

**[260118-1949] bridled**

Add hallmark to JJ commit messages for VVK version tracking.

## Commit Format Change

Current: `jjb:BRAND:IDENTITY[:ACTION]: message`
New: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`

Example: `jjb:RBM:1005:₢AEAAC:n: Fix the bug`

## Changes

### 1. jjrn_notch.rs - Read hallmark at commit time

Add function to read `.vvk/vvbf_brand.json` and extract `vvbh_hallmark` field.
Fatal error if brand file missing (broken VVK install).
Include hallmark in all commit message formatting functions.

### 2. jjrs_steeplechase.rs - Parse hallmark from commits

Update `zjjrs_parse_new_format()` to extract hallmark field after brand.
Add `hallmark: Option<String>` to `jjrs_SteeplechaseEntry`.
Old commits without hallmark parse as None (no backwards compat needed).

### 3. JJD-GallopsData.adoc - Document new format

Update "Commit Message Patterns" section with new format.
Add hallmark field to rein output documentation.
Document fatal error on missing brand file.

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs (read brand file, format commits)
- Tools/jjk/vov_veiled/src/jjrs_steeplechase.rs (parse hallmark, SteeplechaseEntry)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (spec update)

## Verification

- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Create test commit, verify hallmark appears in jjx_rein output

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrn_notch.rs, jjrs_steeplechase.rs, JJD-GallopsData.adoc (3 files)
Steps:
1. Add zjjrn_read_hallmark() to read .vvk/vvbf_brand.json, extract vvbh_hallmark, fatal if missing
2. Update jjrn_format_notch_prefix, jjrn_format_chalk_message, jjrn_format_heat_message, jjrn_format_heat_discussion to include hallmark
3. Add hallmark: Option<String> to jjrs_SteeplechaseEntry
4. Update zjjrs_parse_new_format() to parse hallmark between brand and identity, None for old format
5. Update JJD "Commit Message Patterns" section with new format
6. Document hallmark field in rein output
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260118-1518] rough**

Add hallmark to JJ commit messages for VVK version tracking.

## Commit Format Change

Current: `jjb:BRAND:IDENTITY[:ACTION]: message`
New: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`

Example: `jjb:RBM:1005:₢AEAAC:n: Fix the bug`

## Changes

### 1. jjrn_notch.rs - Read hallmark at commit time

Add function to read `.vvk/vvbf_brand.json` and extract `vvbh_hallmark` field.
Fatal error if brand file missing (broken VVK install).
Include hallmark in all commit message formatting functions.

### 2. jjrs_steeplechase.rs - Parse hallmark from commits

Update `zjjrs_parse_new_format()` to extract hallmark field after brand.
Add `hallmark: Option<String>` to `jjrs_SteeplechaseEntry`.
Old commits without hallmark parse as None (no backwards compat needed).

### 3. JJD-GallopsData.adoc - Document new format

Update "Commit Message Patterns" section with new format.
Add hallmark field to rein output documentation.
Document fatal error on missing brand file.

## Files

- Tools/jjk/vov_veiled/src/jjrn_notch.rs (read brand file, format commits)
- Tools/jjk/vov_veiled/src/jjrs_steeplechase.rs (parse hallmark, SteeplechaseEntry)
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc (spec update)

## Verification

- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Create test commit, verify hallmark appears in jjx_rein output

**[260117-1410] rough**

Drafted from ₢AAAAq in ₣AA.

Add version/brand observability to steeplechase entries and trophy output.

## Changes

1. **SteeplechaseEntry**: Add `brand` field (parsed from commit, informational only)
2. **Trophy rendering**: Show version transitions across heat history
3. **Potential**: Include VVX version if embedded in commits

## Steeplechase Entry Enhancement

```rust
pub struct SteeplechaseEntry {
    pub timestamp: String,
    pub coronet: Option<String>,
    pub action: Option<String>,
    pub subject: String,
    pub brand: Option<String>,  // NEW: parsed from jjb:BRAND:...
}
```

## Trophy Version Section

In the Steeplechase section of trophy, group or annotate by version:
- Show when brand/version changed during heat
- Example: "Commits 1-15: RBM/JJK-v1, Commits 16-30: RBM/JJK-v2"

## Files

- Tools/jjk/veiled/src/jjrs_steeplechase.rs (SteeplechaseEntry, parsing)
- Tools/jjk/veiled/src/jjrg_gallops.rs (trophy rendering)
- Tools/jjk/JJD-GallopsData.adoc (document brand field)

## Depends On

- rein-filter-by-identity (₢AAAAp) - brand parsing happens alongside identity filtering

**[260116-1328] rough**

Add version/brand observability to steeplechase entries and trophy output.

## Changes

1. **SteeplechaseEntry**: Add `brand` field (parsed from commit, informational only)
2. **Trophy rendering**: Show version transitions across heat history
3. **Potential**: Include VVX version if embedded in commits

## Steeplechase Entry Enhancement

```rust
pub struct SteeplechaseEntry {
    pub timestamp: String,
    pub coronet: Option<String>,
    pub action: Option<String>,
    pub subject: String,
    pub brand: Option<String>,  // NEW: parsed from jjb:BRAND:...
}
```

## Trophy Version Section

In the Steeplechase section of trophy, group or annotate by version:
- Show when brand/version changed during heat
- Example: "Commits 1-15: RBM/JJK-v1, Commits 16-30: RBM/JJK-v2"

## Files

- Tools/jjk/veiled/src/jjrs_steeplechase.rs (SteeplechaseEntry, parsing)
- Tools/jjk/veiled/src/jjrg_gallops.rs (trophy rendering)
- Tools/jjk/JJD-GallopsData.adoc (document brand field)

## Depends On

- rein-filter-by-identity (₢AAAAp) - brand parsing happens alongside identity filtering

### tally-silks-argument (₢AEAAL) [complete]

**[260118-2021] complete**

Added --silks argument to jjx_tally for pace rename capability; updated TallyArgs, CLI, and JJD documentation

**[260118-2016] bridled**

Add `--silks` to `jjx_tally` for pace rename capability.

## Changes

### 1. JJD-GallopsData.adoc - Update tally arguments

Add to jjdo_tally Arguments section:
```
// ⟦axd_optional⟧
* {jjda_silks}
— if provided, new Tack uses this value; otherwise inherits from previous Tack
```

### 2. jjrg_gallops.rs - Add silks to TallyArgs

Add `silks: Option<String>` to `jjrg_TallyArgs` struct.

Update `jjrg_tally()` to use provided silks or inherit:
```rust
let new_silks = args.silks.unwrap_or_else(|| current_tack.silks.clone());
```

### 3. jjrm_main.rs - Wire up CLI argument

Add `--silks` / `-s` argument to tally subcommand, pass to TallyArgs.

## Files
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs
- Tools/jjk/vov_veiled/src/jjrm_main.rs

## Verification
- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Manual: `echo "test" | ./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --silks new-name`

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrg_gallops.rs, jjrx_cli.rs, JJD-GallopsData.adoc (3 files)
Steps:
1. Add silks: Option<String> to jjrg_TallyArgs struct
2. Update jjrg_tally(): replace current_tack.silks.clone() with args.silks.unwrap_or_else(|| current_tack.silks.clone())
3. Add #[arg(long, short = 's')] silks: Option<String> to zjjrx_TallyArgs struct
4. Pass silks: args.silks in LibTallyArgs construction in zjjrx_run_tally()
5. Update JJD jjdo_tally Arguments section with silks documentation
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260118-2014] rough**

Add `--silks` to `jjx_tally` for pace rename capability.

## Changes

### 1. JJD-GallopsData.adoc - Update tally arguments

Add to jjdo_tally Arguments section:
```
// ⟦axd_optional⟧
* {jjda_silks}
— if provided, new Tack uses this value; otherwise inherits from previous Tack
```

### 2. jjrg_gallops.rs - Add silks to TallyArgs

Add `silks: Option<String>` to `jjrg_TallyArgs` struct.

Update `jjrg_tally()` to use provided silks or inherit:
```rust
let new_silks = args.silks.unwrap_or_else(|| current_tack.silks.clone());
```

### 3. jjrm_main.rs - Wire up CLI argument

Add `--silks` / `-s` argument to tally subcommand, pass to TallyArgs.

## Files
- Tools/jjk/vov_veiled/JJD-GallopsData.adoc
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs
- Tools/jjk/vov_veiled/src/jjrm_main.rs

## Verification
- tt/vow-b.Build.sh && tt/vow-t.Test.sh
- Manual: `echo "test" | ./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --silks new-name`

### mount-name-check (₢AEAAM) [complete]

**[260118-2025] complete**

Added Step 3.5 name assessment to jjc-heat-mount.md with 3-option prompt (Rename/Continue/Stop) for silks validation

**[260118-2017] bridled**

Add name assessment to mount for rough and bridled paces, with 3-option prompt.

## Behavior

After displaying pace context (Step 3), before branching on state (Step 4):

**Step 3.5: Name assessment**

Assess whether the pace silks fits the spec:
- Read the spec content
- Consider if the kebab-case name accurately reflects the work
- If name fits: proceed silently
- If mismatch detected: present 3 options

```
⚠ Name check: "current-silks" may not fit.
  Spec is about [brief summary of actual work].
  Suggested: "better-name"
  
  [R] Rename to "better-name" (default)
  [C] Continue with current name  
  [S] Stop

  Choice [R]:
```

**On R (or Enter)**: 
- Run `./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --silks "better-name"`
- Report: "Renamed to better-name"
- Continue with mount

**On C**: Continue with current name, no action.

**On S**: Stop mount, suggest `/jjc-pace-reslate` to refine scope.

## Applies to
- Rough paces
- Bridled paces

Both should have correct names before work begins.

## Files
- .claude/commands/jjc-heat-mount.md

## Verification
- Mount a pace with mismatched name, verify prompt appears
- Test all 3 options (R, C, S)
- Mount a pace with fitting name, verify no prompt

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: .claude/commands/jjc-heat-mount.md (1 file)
Steps:
1. Read current jjc-heat-mount.md
2. Insert new '## Step 3.5: Name assessment' section after '## Step 3: Display context', before '## Step 4: Branch on state'
3. Content from pace spec: assessment logic, 3-option AskUserQuestion prompt (R=rename default, C=continue, S=stop), actions for each
4. On R: call jjx_tally with --silks, report rename, continue mount
5. On C: proceed silently
6. On S: stop mount, suggest /jjc-pace-reslate
Verify: Read file, confirm Step 3.5 exists between Steps 3 and 4

**[260118-2015] rough**

Add name assessment to mount for rough and bridled paces, with 3-option prompt.

## Behavior

After displaying pace context (Step 3), before branching on state (Step 4):

**Step 3.5: Name assessment**

Assess whether the pace silks fits the spec:
- Read the spec content
- Consider if the kebab-case name accurately reflects the work
- If name fits: proceed silently
- If mismatch detected: present 3 options

```
⚠ Name check: "current-silks" may not fit.
  Spec is about [brief summary of actual work].
  Suggested: "better-name"
  
  [R] Rename to "better-name" (default)
  [C] Continue with current name  
  [S] Stop

  Choice [R]:
```

**On R (or Enter)**: 
- Run `./tt/vvw-r.RunVVX.sh jjx_tally <CORONET> --silks "better-name"`
- Report: "Renamed to better-name"
- Continue with mount

**On C**: Continue with current name, no action.

**On S**: Stop mount, suggest `/jjc-pace-reslate` to refine scope.

## Applies to
- Rough paces
- Bridled paces

Both should have correct names before work begins.

## Files
- .claude/commands/jjc-heat-mount.md

## Verification
- Mount a pace with mismatched name, verify prompt appears
- Test all 3 options (R, C, S)
- Mount a pace with fitting name, verify no prompt

### reslate-name-check (₢AEAAN) [complete]

**[260118-2028] complete**

Added name assessment to jjc-pace-reslate.md with gestalt comparison and 3-option prompt for conditional rename

**[260118-2019] bridled**

Add name assessment to reslate after spec refinement.

## Behavior

After spec is refined, before committing:

**Name assessment step**

Assess whether the pace silks still fits the refined spec:
- Compare old spec gestalt to new spec gestalt
- If name still fits: proceed silently
- If gestalt has shifted: suggest new name with 3-option prompt

```
⚠ Name check: "old-silks" may not fit refined spec.
  Was: [old focus]
  Now: [new focus]
  Suggested: "better-name"
  
  [R] Rename to "better-name" (default)
  [C] Continue with current name  
  [S] Stop (abort reslate)

  Choice [R]:
```

**On R (or Enter)**: Include `--silks "new-name"` in the jjx_tally call.

**On C**: Proceed with current silks (tally without --silks).

**On S**: Abort the reslate entirely, no changes made.

## Files
- .claude/commands/jjc-pace-reslate.md

## Verification
- Reslate a pace with significant scope change, verify rename suggestion
- Reslate a pace with minor refinement, verify no prompt
- Test all 3 options (R, C, S)

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: .claude/commands/jjc-pace-reslate.md (1 file)
Steps:
1. Read current reslate command
2. Restructure Step 3: before tally, fetch current spec via jjx_saddle or parade --pace
3. Compare old spec gestalt to new text - if name no longer fits, present 3-option prompt (R=rename default, C=continue, S=stop/abort)
4. Conditional tally execution:
   - On R: echo NEW_TEXT | jjx_tally CORONET --silks new-name
   - On C: echo NEW_TEXT | jjx_tally CORONET
   - On S: abort reslate, no changes made
5. Update Step 4 reporting to mention rename if it occurred
Verify: Read file, confirm name assessment happens before tally call

**[260118-2015] rough**

Add name assessment to reslate after spec refinement.

## Behavior

After spec is refined, before committing:

**Name assessment step**

Assess whether the pace silks still fits the refined spec:
- Compare old spec gestalt to new spec gestalt
- If name still fits: proceed silently
- If gestalt has shifted: suggest new name with 3-option prompt

```
⚠ Name check: "old-silks" may not fit refined spec.
  Was: [old focus]
  Now: [new focus]
  Suggested: "better-name"
  
  [R] Rename to "better-name" (default)
  [C] Continue with current name  
  [S] Stop (abort reslate)

  Choice [R]:
```

**On R (or Enter)**: Include `--silks "new-name"` in the jjx_tally call.

**On C**: Proceed with current silks (tally without --silks).

**On S**: Abort the reslate entirely, no changes made.

## Files
- .claude/commands/jjc-pace-reslate.md

## Verification
- Reslate a pace with significant scope change, verify rename suggestion
- Reslate a pace with minor refinement, verify no prompt
- Test all 3 options (R, C, S)

### prime-merges-direction-into-spec (₢AEAAD) [abandoned]

**[260118-1950] abandoned**

Drafted from ₢AAAAz in ₣AA.

Refactor /jjc-pace-prime to merge direction into spec instead of using separate tack_direction field.

## Current behavior

Prime writes direction to tack_direction field. Mount reads both tack_text and tack_direction for primed paces.

## Proposed behavior

Prime appends direction content to tack_text under a '## Direction' heading. The tack_direction field becomes unused for new paces.

## Benefits

- Single source of truth - what agent sees is in one place
- No risk of direction referencing spec content agent doesn't see
- Simpler mental model

## Changes

1. **jjc-pace-prime.md**: Change jjx_tally call to append direction to tack_text (via --text with merged content) instead of using --direction flag

2. **jjc-heat-mount.md**: 
   - In the "If pace_state is primed" section, remove the two lines added in commit 3ad5957c that reference "tack_text (the spec) and tack_direction (execution guidance)"
   - Revert to simpler "Execute per the spec autonomously" since direction is now in spec
   - Keep tack_direction fallback for legacy primed paces (backward compatibility)

3. **JJD-GallopsData.adoc**: Document that tack_direction is deprecated for new paces; direction should be appended to tack_text under ## Direction heading

4. **CLAUDE.md Job Jockey Configuration section**: Update any references to direction field if present

## Migration

Existing primed paces with tack_direction continue to work (mount checks tack_direction as fallback). New paces use merged approach.

## Verification

- Prime a test pace, verify direction appears in tack_text under ## Direction
- Mount primed pace, verify execution succeeds
- Mount legacy primed pace (with tack_direction), verify backward compatibility

**[260117-1410] rough**

Drafted from ₢AAAAz in ₣AA.

Refactor /jjc-pace-prime to merge direction into spec instead of using separate tack_direction field.

## Current behavior

Prime writes direction to tack_direction field. Mount reads both tack_text and tack_direction for primed paces.

## Proposed behavior

Prime appends direction content to tack_text under a '## Direction' heading. The tack_direction field becomes unused for new paces.

## Benefits

- Single source of truth - what agent sees is in one place
- No risk of direction referencing spec content agent doesn't see
- Simpler mental model

## Changes

1. **jjc-pace-prime.md**: Change jjx_tally call to append direction to tack_text (via --text with merged content) instead of using --direction flag

2. **jjc-heat-mount.md**: 
   - In the "If pace_state is primed" section, remove the two lines added in commit 3ad5957c that reference "tack_text (the spec) and tack_direction (execution guidance)"
   - Revert to simpler "Execute per the spec autonomously" since direction is now in spec
   - Keep tack_direction fallback for legacy primed paces (backward compatibility)

3. **JJD-GallopsData.adoc**: Document that tack_direction is deprecated for new paces; direction should be appended to tack_text under ## Direction heading

4. **CLAUDE.md Job Jockey Configuration section**: Update any references to direction field if present

## Migration

Existing primed paces with tack_direction continue to work (mount checks tack_direction as fallback). New paces use merged approach.

## Verification

- Prime a test pace, verify direction appears in tack_text under ## Direction
- Mount primed pace, verify execution succeeds
- Mount legacy primed pace (with tack_direction), verify backward compatibility

**[260116-1512] rough**

Refactor /jjc-pace-prime to merge direction into spec instead of using separate tack_direction field.

## Current behavior

Prime writes direction to tack_direction field. Mount reads both tack_text and tack_direction for primed paces.

## Proposed behavior

Prime appends direction content to tack_text under a '## Direction' heading. The tack_direction field becomes unused for new paces.

## Benefits

- Single source of truth - what agent sees is in one place
- No risk of direction referencing spec content agent doesn't see
- Simpler mental model

## Changes

1. **jjc-pace-prime.md**: Change jjx_tally call to append direction to tack_text (via --text with merged content) instead of using --direction flag

2. **jjc-heat-mount.md**: 
   - In the "If pace_state is primed" section, remove the two lines added in commit 3ad5957c that reference "tack_text (the spec) and tack_direction (execution guidance)"
   - Revert to simpler "Execute per the spec autonomously" since direction is now in spec
   - Keep tack_direction fallback for legacy primed paces (backward compatibility)

3. **JJD-GallopsData.adoc**: Document that tack_direction is deprecated for new paces; direction should be appended to tack_text under ## Direction heading

4. **CLAUDE.md Job Jockey Configuration section**: Update any references to direction field if present

## Migration

Existing primed paces with tack_direction continue to work (mount checks tack_direction as fallback). New paces use merged approach.

## Verification

- Prime a test pace, verify direction appears in tack_text under ## Direction
- Mount primed pace, verify execution succeeds
- Mount legacy primed pace (with tack_direction), verify backward compatibility

**[260116-1510] rough**

Refactor /jjc-pace-prime to merge direction into spec instead of using separate tack_direction field.

## Current behavior

Prime writes direction to tack_direction field. Mount reads both tack_text and tack_direction for primed paces.

## Proposed behavior

Prime appends direction content to tack_text under a '## Direction' heading. The tack_direction field becomes unused for new paces.

## Benefits

- Single source of truth - what agent sees is in one place
- No risk of direction referencing spec content agent doesn't see
- Simpler mental model

## Changes

1. **jjc-pace-prime.md**: Change jjx_tally call to append direction to tack_text (via --text with merged content) instead of using --direction flag

2. **jjc-heat-mount.md**: 
   - Remove lines 94-95 added in commit 3ad5957c that reference tack_direction for primed paces
   - Revert to simpler "Execute per the spec autonomously" since direction is now in spec
   - Keep tack_direction fallback for legacy primed paces (backward compatibility)

3. **JJD-GallopsData.adoc**: Document that tack_direction is deprecated for new paces; direction should be appended to tack_text under ## Direction heading

4. **CLAUDE.md Job Jockey Configuration section**: Update any references to direction field if present

## Migration

Existing primed paces with tack_direction continue to work (mount checks tack_direction as fallback). New paces use merged approach.

## Verification

- Prime a test pace, verify direction appears in tack_text under ## Direction
- Mount primed pace, verify execution succeeds
- Mount legacy primed pace (with tack_direction), verify backward compatibility

**[260116-1509] rough**

Refactor /jjc-pace-prime to merge direction into spec instead of using separate tack_direction field.

## Current behavior

Prime writes direction to tack_direction field. Mount reads both tack_text and tack_direction for primed paces.

## Proposed behavior

Prime appends direction content to tack_text under a '## Direction' heading. The tack_direction field becomes unused for new paces.

## Benefits

- Single source of truth - what agent sees is in one place
- No risk of direction referencing spec content agent doesn't see
- Simpler mental model

## Changes

1. **jjc-pace-prime.md**: Change jjx_tally call to append direction to --text instead of using --direction
2. **jjc-heat-mount.md**: Remove tack_direction references for primed paces (direction is in spec)
3. **JJD-GallopsData.adoc**: Document that tack_direction is deprecated for new paces

## Migration

Existing primed paces with tack_direction continue to work (mount reads both). New paces use merged approach.

## Verification

- Prime a test pace, verify direction appears in tack_text
- Mount primed pace, verify execution succeeds

### accept-unprefixed-identities (₢AEAAG) [complete]

**[260118-2015] complete**

Verified unprefixed identity parsing already implemented in jjrf_favor.rs; no changes needed

**[260118-1105] bridled**

JJD specifies input flexibility for Firemark and Coronet types:
- 'Input may omit the ₣ prefix; output always includes it.' (Firemark, line 251)
- 'Input may omit the ₢ prefix; output always includes it.' (Coronet, line 264)
- 'CLI commands accept identities with or without prefix. Length determines type: 2 base64 chars = firemark, 5 base64 chars = coronet.' (lines 585-586)

Currently the Rust implementation requires the unicode prefix. For example:
- `jjx_parade --format detail --pace ADAAL AD` fails with 'Pace ADAAL not found'
- `jjx_parade --format detail --pace ₢ADAAL AD` succeeds

Update identity parsing throughout jjx to accept bare base64 strings:
- 2 characters → Firemark (prepend ₣ internally)
- 5 characters → Coronet (prepend ₢ internally)
- Already-prefixed inputs continue to work

This reduces stumbles when slash commands or users omit the unicode prefix.

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrt_types.rs or equivalent types module (1-2 files)
Steps:
1. Find Firemark/Coronet type definitions and their FromStr implementations
2. Update parsing: if input is 2 chars without ₣, prepend ₣; if 5 chars without ₢, prepend ₢
3. Existing prefixed inputs continue to work unchanged
4. Test with both prefixed and unprefixed inputs via CLI
Verify: tt/vvw-t.TestVVX.sh && ./tt/vvw-r.RunVVX.sh jjx_parade --format detail --pace AEAAG AE

**[260118-1100] rough**

JJD specifies input flexibility for Firemark and Coronet types:
- 'Input may omit the ₣ prefix; output always includes it.' (Firemark, line 251)
- 'Input may omit the ₢ prefix; output always includes it.' (Coronet, line 264)
- 'CLI commands accept identities with or without prefix. Length determines type: 2 base64 chars = firemark, 5 base64 chars = coronet.' (lines 585-586)

Currently the Rust implementation requires the unicode prefix. For example:
- `jjx_parade --format detail --pace ADAAL AD` fails with 'Pace ADAAL not found'
- `jjx_parade --format detail --pace ₢ADAAL AD` succeeds

Update identity parsing throughout jjx to accept bare base64 strings:
- 2 characters → Firemark (prepend ₣ internally)
- 5 characters → Coronet (prepend ₢ internally)
- Already-prefixed inputs continue to work

This reduces stumbles when slash commands or users omit the unicode prefix.

### consider-pace-locking (₢AEAAH) [abandoned]

**[260118-1946] abandoned**

Consider adding pessimistic locking to prevent concurrent session collisions on paces.

## Problem

Two Claude Code sessions can mount the same pace simultaneously. One completes and wraps while the other churns or attempts execution on stale state. Observed 260118 when parallel sessions caused Task agent display loop.

## Proposed Solution

Add "mounted" state for pessimistic locking:
- rough/bridled → mounted (on mount, if not locked)
- mounted → complete (wrap succeeds)
- mounted → rough/bridled (failure or explicit dismount)

Lock metadata in tack:
```json
{
  "state": "mounted",
  "lock": {
    "session": "abc123",
    "acquired": "260118-1145"
  }
}
```

Mount fails if pace already mounted by different session.

## Escape Hatch

Need `/jjc-pace-dismount --force` for breaking stale locks (crashed sessions).
Options: manual break, timestamp expiry, or both.

## Open Question

What constitutes a "session"? Claude Code doesn't expose session ID. May need to generate one.

## Constraint

**Must begin with JJD spec update.** State machine is growing complex; need specification clarity before implementation.

## Verb: Consider

This pace requires re-justification before execution. Behavioral workaround exists (don't run parallel sessions on same heat). Evaluate whether the complexity is worth the protection.

**[260118-1120] rough**

Consider adding pessimistic locking to prevent concurrent session collisions on paces.

## Problem

Two Claude Code sessions can mount the same pace simultaneously. One completes and wraps while the other churns or attempts execution on stale state. Observed 260118 when parallel sessions caused Task agent display loop.

## Proposed Solution

Add "mounted" state for pessimistic locking:
- rough/bridled → mounted (on mount, if not locked)
- mounted → complete (wrap succeeds)
- mounted → rough/bridled (failure or explicit dismount)

Lock metadata in tack:
```json
{
  "state": "mounted",
  "lock": {
    "session": "abc123",
    "acquired": "260118-1145"
  }
}
```

Mount fails if pace already mounted by different session.

## Escape Hatch

Need `/jjc-pace-dismount --force` for breaking stale locks (crashed sessions).
Options: manual break, timestamp expiry, or both.

## Open Question

What constitutes a "session"? Claude Code doesn't expose session ID. May need to generate one.

## Constraint

**Must begin with JJD spec update.** State machine is growing complex; need specification clarity before implementation.

## Verb: Consider

This pace requires re-justification before execution. Behavioral workaround exists (don't run parallel sessions on same heat). Evaluate whether the complexity is worth the protection.

## Steeplechase

### 2026-01-18 20:00 - ₢AEAAC - F

Executing bridled pace via sonnet agent

### 2026-01-18 19:59 - Heat - T

steeplechase-version-tracking

### 2026-01-18 19:59 - Heat - T

steeplechase-version-tracking

### 2026-01-18 19:51 - ₢AEAAC - F

Executing bridled pace via sonnet agent

### 2026-01-18 19:50 - Heat - T

prime-merges-direction-into-spec

### 2026-01-18 19:49 - Heat - T

steeplechase-version-tracking

### 2026-01-18 19:46 - Heat - T

consider-pace-locking

### 2026-01-18 15:18 - Heat - T

steeplechase-version-tracking

### 2026-01-18 15:10 - ₢AEAAB - W

Added zjjrq_resolve_pace() with Coronet normalization + silks fallback for --pace

### 2026-01-18 15:10 - Heat - T

parade-pace-silks-lookup

### 2026-01-18 15:07 - ₢AEAAB - F

Executing bridled pace via 2 parallel sonnet agents

### 2026-01-18 15:05 - Heat - T

parade-pace-silks-lookup

### 2026-01-18 15:03 - ₢AEAAB - A

Add Coronet parse normalization and silks fallback to parade --pace

### 2026-01-18 14:58 - ₢AEAAA - W

Created /jjc-heat-rein, added recent_work with commit SHA to saddle, updated mount display

### 2026-01-18 14:58 - Heat - T

create-heat-rein-command

### 2026-01-18 14:51 - ₢AEAAA - A

Create rein slash command, add recent_work to saddle, update mount display

### 2026-01-18 14:48 - ₢AEAAK - W

Replaced custom Deserialize impl with derive, removed 65 lines legacy code

### 2026-01-18 14:48 - Heat - T

tack-struct-rust-cleanup

### 2026-01-18 14:46 - ₢AEAAK - F

Executing bridled pace via sonnet agent

### 2026-01-18 14:44 - Heat - T

tack-struct-rust-cleanup

### 2026-01-18 14:42 - ₢AEAAK - A

Simplify Pace/Tack deserialize: replace custom impl with derive, remove legacy comments

### 2026-01-18 14:41 - ₢AEAAJ - W

Removed Legacy Format Acceptance section and simplified jjdkm_commit definition. Verification confirmed zero legacy/migration references remain.

### 2026-01-18 14:41 - Heat - T

tack-struct-jjd-cleanup

### 2026-01-18 14:24 - ₢AEAAJ - F

Executing bridled pace via sonnet agent

### 2026-01-18 14:19 - Heat - T

tack-struct-jjd-cleanup

### 2026-01-18 14:17 - ₢AEAAI - W

Migrated silks from Pace to Tack level with custom deserializer. Added commit field. Created jjrg_make_tack constructor. Extended RCG with 4 new disciplines.

### 2026-01-18 14:17 - Heat - T

tack-struct-rust-migration

### 2026-01-18 14:17 - ₢AEAAI - n

RCG improvements and CLAUDE.md fix

### 2026-01-18 14:06 - ₢AEAAI - n

Fix remaining silks accessor and update test file for migration

### 2026-01-18 14:03 - ₢AEAAI - A

Migration implementation complete

### 2026-01-18 14:00 - ₢AEAAI - n

Migrate silks and commit from Pace to Tack level

### 2026-01-18 13:49 - Heat - T

tack-struct-jjd-cleanup

### 2026-01-18 13:48 - ₢AEAAI - A

Sequential migration: 1) update Tack struct with silks+commit, 2) custom Pace deserialize for legacy, 3) remove Pace.silks, 4) update creation sites, 5) fix accessors

### 2026-01-18 13:41 - ₢AEAAI - n

Enforce explicit user confirmation for pace wrapping in JJ workflow

### 2026-01-18 13:34 - Heat - T

tack-silks-and-commit-migration

### 2026-01-18 13:34 - ₢AEAAE - W

JJD migration spec complete: moved silks to Tack, added commit field, added legacy format acceptance section

### 2026-01-18 13:34 - ₢AEAAE - n

Move pace silks from pace record to tack record with legacy format acceptance

### 2026-01-18 13:29 - ₢AEAAE - A

JJD migration spec: move silks to Tack, add commit field, add migration section

### 2026-01-18 13:24 - Heat - T

tack-struct-rust-migration

### 2026-01-18 13:24 - Heat - T

tack-silks-and-commit-migration

### 2026-01-18 13:20 - Heat - r

moved AEAAK after AEAAJ

### 2026-01-18 13:20 - Heat - r

moved AEAAJ after AEAAI

### 2026-01-18 13:20 - Heat - r

moved AEAAI after AEAAE

### 2026-01-18 13:20 - Heat - r

moved AEAAE after AEAAF

### 2026-01-18 13:20 - Heat - S

tack-struct-rust-cleanup

### 2026-01-18 13:16 - Heat - S

tack-struct-jjd-cleanup

### 2026-01-18 13:14 - Heat - S

tack-struct-rust-migration

### 2026-01-18 13:13 - Heat - T

tack-silks-and-commit-migration

### 2026-01-18 11:20 - Heat - S

consider-pace-locking

### 2026-01-18 11:10 - ₢AEAAF - W

Refactored vvtg_guard.rs tests to eliminate set_current_dir() race conditions by passing repo_dir to test functions and using get_test_base() helper for temp directory fallback. All 4 tests now pass in parallel.

### 2026-01-18 11:10 - Heat - T

vvc-guard-binary-file-size

### 2026-01-18 11:07 - ₢AEAAF - F

Executing bridled pace via sonnet agent

### 2026-01-18 11:05 - Heat - T

accept-unprefixed-identities

### 2026-01-18 11:00 - Heat - S

accept-unprefixed-identities

### 2026-01-18 10:47 - ₢AEAAF - n

Add repo_dir parameter to guard functions for explicit git directory control

### 2026-01-18 10:38 - ₢AEAAF - n

Refactor diff size calculation to implement cost model

### 2026-01-18 10:33 - Heat - S

simplify guard limit parameters, remove layered overrides

### 2026-01-18 10:16 - Heat - S

preserve gallops database before guard fix

### 2026-01-18 10:07 - Heat - S

canonicalize-bud-paths-for-test-compatibility

### 2026-01-18 09:54 - Heat - S

vof-vvb-build-infrastructure-refactor

### 2026-01-18 08:43 - ₢AEAAF - F

Executing bridled pace via sonnet agent

### 2026-01-18 08:42 - Heat - T

vvc-guard-binary-file-size

### 2026-01-18 08:41 - Heat - T

vvc-guard-binary-file-size

### 2026-01-18 08:39 - ₢AEAAF - F

Executing bridled pace via sonnet agent

### 2026-01-18 08:37 - Heat - T

vvc-guard-binary-file-size

### 2026-01-18 08:35 - ₢AEAAF - A

Fix blob size via ls-files+cat-file, add vvct_guard.rs tests

### 2026-01-18 08:34 - Heat - T

vvc-guard-binary-file-size

### 2026-01-18 08:05 - Heat - T

vvc-guard-binary-file-size

### 2026-01-17 14:24 - Heat - S

vvc-guard-binary-file-size

### 2026-01-17 14:10 - Heat - d

Restring: 5 paces from ₣AA (JJK features & polish)

### 2026-01-17 14:10 - Heat - D

₢AAABO → ₢AEAAE

### 2026-01-17 14:10 - Heat - D

₢AAAAz → ₢AEAAD

### 2026-01-17 14:10 - Heat - D

₢AAAAq → ₢AEAAC

### 2026-01-17 14:10 - Heat - D

₢AAAAl → ₢AEAAB

### 2026-01-17 14:10 - Heat - D

₢AAAAo → ₢AEAAA

### 2026-01-17 14:10 - Heat - N

jjk-features-polish

