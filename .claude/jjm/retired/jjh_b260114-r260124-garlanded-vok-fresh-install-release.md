# Heat Trophy: garlanded-vok-fresh-install-release

**Firemark:** ₣AA
**Created:** 260114
**Retired:** 260124
**Status:** retired

> NOTE: VOS renamed to VOS0 (top-level spec '0' suffix convention). Filename references in this trophy are historical.

## Paddock

# Paddock: vok-fresh-install-release

## MVP Scope

Complete release/install/uninstall cycle. Single platform. Hardcoded CLAUDE.md templates (externalized post-MVP).

## Test Environment

- **Target repo**: /Users/bhyslop/projects/pb_paneboard02
- **Staging dir**: ../release-install-tarball/

**Iteration workflow** (from kit forge):
```bash
# 0. Notch to establish known state
/jjc-pace-notch

# 1. Build release (creates vvk-parcel-NNNN.tar.gz in kit forge)
tt/vow-R.Release.sh

# 2. Nuke staging (including hidden files), extract, delete ALL local tarballs
rm -rf ../release-install-tarball/.* ../release-install-tarball/* 2>/dev/null
tar -xzf vvk-parcel-*.tar.gz -C ../release-install-tarball/
rm -f vvk-parcel-*.tar.gz

# 3. Install to target (parameter is path to burc.env, NOT just target dir)
../release-install-tarball/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env

# 4. Uninstall from target
/Users/bhyslop/projects/pb_paneboard02/Tools/vvk/vvu_uninstall.sh
```

**Test discipline**: Continue as far as possible through steps 0-4. Diagnose any errors encountered. **STOP before editing kit forge code to fix** — report findings to human first.

## Parcel Structure (established)

```
vvk-parcel-{hallmark}/
├── vvi_install.sh          # Bootstrap: detect platform, invoke vvx
├── vvbf_brand.json         # {vvbh_hallmark, vvbd_date, vvbs_sha, vvbc_commit, vvbk_kits}
└── kits/{kit}/             # Kit assets (excludes vov_veiled/)
    └── bin/vvx-{platform}  # Binary (vvk only)
```

## Operation Contracts

**vvx_emplace** (Rust, with git commit):
- Parse burc.env → BURC_TOOLS_DIR, BURC_PROJECT_ROOT, BURC_MANAGED_KITS
- Validate exact match: parcel's vvbk_kits == BURC_MANAGED_KITS
- Nuclear cleanup: delete existing .vvk/ and kit directories
- Copy kits/* → ${BURC_TOOLS_DIR}/
- Route commands ({cipher}c-*.md) → .claude/commands/
- Freshen CLAUDE.md via voff_freshen()
- Copy brand → .vvk/vvbf_brand.json
- Git commit

**vvx_vacate** (Rust, with git commit):
- Parse burc.env (requires BURC_MANAGED_KITS for defense in depth)
- Read .vvk/vvbf_brand.json for kit list
- Remove routed commands/hooks
- Collapse CLAUDE.md via voff_collapse()
- Remove vvx binaries from Tools/vvk/bin/
- **Preserve** kit directories (unveiled content remains functional)
- Remove brand file and .vvk/
- Git commit

**Bash wrappers** (vvi_install.sh, vvu_uninstall.sh) are thin bootstraps that exec Rust.

## CLAUDE.md Managed Sections

**Reminder**: Content inside `<!-- MANAGED:{tag}:BEGIN/END -->` markers is overwritten on install.

**Template sources** (update these when modifying managed content):

| Section | MVP Source (₢AAAAF) | Post-MVP Source (₢AAABK) |
|---------|---------------------|--------------------------|
| BUK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/buk/vov_veiled/vocbumc_core.md` |
| CMK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/cmk/vov_veiled/voccmmc_core.md` |
| JJK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/jjk/vov_veiled/vocjjmc_core.md` |
| VVK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/vvk/vov_veiled/vocvvmc_core.md` |

Content outside markers is user content — survives reinstall.

## References

- RCG: Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- BCG: Tools/buk/vov_veiled/BCG-BashConsoleGuide.md
- VOS: Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc
- voff_freshen: Tools/vok/vof/src/voff_freshen.rs
- vofc_registry: Tools/vok/vof/src/vofc_registry.rs

## Steeplechase Summary

- **260113**: Heat created. Decisions: all-Rust, all-or-none install, git-aware with Claude recovery.
- **260115**: Archive-based asset model chosen over embedded binaries.
- **260117**: Release implemented (₢AAAAE). Hallmark 1000 allocated. Paddock condensed for MVP focus.
- **260118**: BURC_MANAGED_KITS added for kit inventory control. Exact match validation on emplace prevents binary capability leakage. Vacate updated to preserve kit directories (unveiled content is open-source). Install to pb tested successfully (hallmark 1005). Orphaned CMK from previous install cleaned up manually. VOS updated to match implementation.

## Next Steps

1. Test uninstall/reinstall cycle on pb
2. Verify pb bash utilities remain functional after vacate
3. Test reinstall after vacate

## Gallops Functional Test (post-install)

After successful install, validate JJK functionality in target repo:

```bash
# From target repo (pb_paneboard02)
cd /Users/bhyslop/projects/pb_paneboard02

# 1. Create a test heat
./Tools/vvk/bin/vvx jjx_nominate --silks test-install-validation

# 2. Slate a few short paces
echo "Verify JJK installed correctly" | ./Tools/vvk/bin/vvx jjx_slate <FIREMARK> --silks verify-install
echo "Check git commit history" | ./Tools/vvk/bin/vvx jjx_slate <FIREMARK> --silks check-commits
echo "Clean up test heat" | ./Tools/vvk/bin/vvx jjx_slate <FIREMARK> --silks cleanup

# 3. Read back - verify paces appear
./Tools/vvk/bin/vvx jjx_saddle <FIREMARK>

# 4. Check git log for JJK commits
git log --oneline -5
# Should see: nominate commit, slate commits

# 5. Cleanup: abandon heat (or leave for manual inspection)
```

**Success criteria**:
- [ ] jjx_nominate creates heat with correct firemark
- [ ] jjx_slate adds paces (each creates git commit)
- [ ] jjx_saddle returns correct JSON with paces
- [ ] Git history shows JJK-prefixed commit messages

## Paces

### jjd-routines-load-save (₢AAABU) [complete]

**[260119-0857] complete**

Added jjdr_ (Routines) category to JJD spec with jjdr_load and jjdr_save definitions. Updated all 7 write operations to use load/save sandwich pattern with explicit failure semantics.

**[260119-0848] rough**

Update JJD-GallopsData.adoc to add jjdr_ (Routines) category with two routines. This is SPEC WORK ONLY — no Rust implementation.

## Mapping section additions

Add to category declarations:
```
// jjdr_:  Routines (internal reusable procedures)
```

Add attribute references:
```
:jjdr_load:           <<jjdr_load,load routine>>
:jjdr_save:           <<jjdr_save,save routine>>
```

## Routine: jjdr_load

Voice as: `// ⟦axl_voices axo_routine⟧`

Behavior:
1. Read file as raw bytes
2. Deserialize bytes → Gallops structure
3. Reserialize Gallops → JSON string  
4. Compare reserialized bytes against original bytes (raw comparison)
5. If different: FATAL with diagnostic showing first difference location
6. Call {jjdv_validate} for semantic validation (timestamps, kebab-case, invariants)
7. If validation fails: FATAL with validation errors
8. Return ValidatedGallops (newtype wrapper ensuring validation occurred)

Document: jjdr_load is the ONLY way to obtain a validated Gallops from disk. This makes validation bypass architecturally impossible.

## Routine: jjdr_save

Voice as: `// ⟦axl_voices axo_routine⟧`

Behavior:
1. Serialize Gallops → canonical JSON string
2. Atomic write to temp file
3. Call {jjdr_load} on temp file to validate what was written
4. If load fails: delete temp, FATAL with error
5. Rename temp → final path

Document: jjdr_save reuses jjdr_load for validation, ensuring single validation code path for both directions.

## Update all write operations

Revise behavior sections of: jjdo_nominate, jjdo_slate, jjdo_rail, jjdo_draft, jjdo_tally, jjdo_furlough, jjdo_retire

Each write operation behavior becomes:
1. {jjdr_load} {jjda_file}; on failure, exit immediately with {jjdr_load} error status
2. (command-specific transformation steps)  
3. {jjdr_save} {jjdgr_gallops} → {jjda_file}

## Note on jjdv_validate

Reference existing jjrg_validate semantic validation. May need to add jjdv_ category for validation concerns, or reference under jjdz_ (serialization). Decide during editing.

**[260119-0831] rough**

Add jjdr_ (Routines) category to JJD with two routines enforcing round-trip validation:

## jjdr_load
- Read file as raw bytes
- Deserialize to Rust Gallops structure  
- Reserialize Rust structure to JSON string
- Compare reserialized string against original bytes (raw comparison, not normalized)
- If different: FATAL with diagnostic showing first difference location
- If identical: return Rust object model

## jjdr_save
- Accept Rust Gallops object
- Serialize to canonical JSON (sorted keys via BTreeMap, consistent formatting)
- Atomic write (temp file → rename)

## Update all write operations
Revise behavior sections of: jjdo_nominate, jjdo_slate, jjdo_rail, jjdo_draft, jjdo_tally, jjdo_furlough, jjdo_retire

Each write operation behavior becomes:
1. {jjdr_load} {jjda_file}; on failure, exit immediately with {jjdr_load} error status
2. (command-specific transformation steps)
3. {jjdr_save} {jjdgr_gallops} → {jjda_file}

## Mapping section additions
Add to JJD mapping section:
// jjdr_:  Routines (internal reusable procedures)
:jjdr_load:           <<jjdr_load,load routine>>
:jjdr_save:           <<jjdr_save,save routine>>

## Architectural constraint
Document that jjdr_load is the ONLY way to obtain a Gallops struct - no public constructor. This makes validation bypass impossible.

### jjr-io-routines-impl (₢AAABV) [complete]

**[260119-0912] complete**

File split to 5 modules (types, validate, io, ops, util) + ValidatedGallops newtype

**[260119-0900] bridled**

Implement jjdr_load and jjdr_save routines in Rust, including file split refactor.

## File Split

Split `jjrg_gallops.rs` (1,395 lines) into focused modules with unique prefix letters:

| New File | Contents |
|----------|----------|
| `jjrt_types.rs` | Gallops, Heat, Pace, Tack structs; HeatStatus, PaceState enums; serde derives |
| `jjrv_validate.rs` | jjrg_validate() and all zjjrg_is_* validator helpers |
| `jjri_io.rs` | jjdr_load, jjdr_save implementations; ValidatedGallops newtype |
| `jjro_ops.rs` | All operation methods (nominate, slate, tally, rail, draft, retire, furlough) |
| `jjru_util.rs` | zjjrg_increment_seed, jjrg_capture_commit_sha, jjrg_make_tack, stdin helpers |

Update `lib.rs` to re-export public API from new modules.

## ValidatedGallops Newtype

```rust
pub struct ValidatedGallops(jjrg_Gallops);

impl ValidatedGallops {
    // No public constructor - only jjdr_load can create
    pub fn inner(&self) -> &jjrg_Gallops { &self.0 }
    pub fn inner_mut(&mut self) -> &mut jjrg_Gallops { &self.0 }
    pub fn into_inner(self) -> jjrg_Gallops { self.0 }
    
    #[cfg(test)]
    pub fn test_wrap(g: jjrg_Gallops) -> Self { Self(g) }
}
```

## jjdr_load Implementation

```rust
pub fn jjdr_load(path: &Path) -> Result<ValidatedGallops, String> {
    let original_bytes = fs::read(path)?;
    let gallops: jjrg_Gallops = serde_json::from_slice(&original_bytes)?;
    let reserialized = serde_json::to_string_pretty(&gallops)?;
    if reserialized.as_bytes() \!= original_bytes {
        return Err(format\!("Round-trip validation failed at byte {}", 
            find_first_diff(&original_bytes, reserialized.as_bytes())));
    }
    gallops.jjrg_validate()?;
    Ok(ValidatedGallops(gallops))
}
```

## jjdr_save Implementation

```rust
pub fn jjdr_save(gallops: &jjrg_Gallops, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(gallops)?;
    let temp_path = path.with_extension(format\!("tmp.{}.json", std::process::id()));
    fs::write(&temp_path, &json)?;
    // Validate what we wrote by loading it back
    match jjdr_load(&temp_path) {
        Ok(_) => {
            fs::rename(&temp_path, path)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::remove_file(&temp_path);
            Err(format\!("Save validation failed: {}", e))
        }
    }
}
```

## CLI Updates

Update jjrx_cli.rs to use jjdr_load/jjdr_save instead of direct jjrg_load/jjrg_save calls.

## Test Updates

- Move tests to corresponding new modules
- Add round-trip validation tests
- Add ValidatedGallops newtype tests
- Ensure all existing tests pass after refactor

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: jjrt_types.rs, jjrv_validate.rs, jjri_io.rs, jjro_ops.rs, jjru_util.rs, jjrg_gallops.rs, lib.rs, jjrx_cli.rs (8 files)
Steps:
1. Create jjrt_types.rs: extract JJRG_UNKNOWN_COMMIT, enums (PaceState, HeatStatus), structs (Tack, Pace, Heat, Gallops), Args/Result structs
2. Create jjrv_validate.rs: extract zjjrg_is_* helpers and jjrg_validate() method (as standalone fn taking &jjrg_Gallops)
3. Create jjru_util.rs: extract zjjrg_increment_seed, jjrg_capture_commit_sha, jjrg_make_tack, jjrg_read_stdin, jjrg_read_stdin_optional
4. Create jjro_ops.rs: extract operation methods as standalone fns taking &mut jjrg_Gallops
5. Create jjri_io.rs: implement ValidatedGallops newtype, jjdr_load (with round-trip validation per spec), jjdr_save
6. Update jjrg_gallops.rs: remove extracted code, re-export from new modules for backwards compatibility
7. Update lib.rs: add new modules, update re-exports
8. Update jjrx_cli.rs: replace Gallops::jjrg_load with jjdr_load, jjrg_save with jjdr_save
Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260119-0848] rough**

Implement jjdr_load and jjdr_save routines in Rust, including file split refactor.

## File Split

Split `jjrg_gallops.rs` (1,395 lines) into focused modules with unique prefix letters:

| New File | Contents |
|----------|----------|
| `jjrt_types.rs` | Gallops, Heat, Pace, Tack structs; HeatStatus, PaceState enums; serde derives |
| `jjrv_validate.rs` | jjrg_validate() and all zjjrg_is_* validator helpers |
| `jjri_io.rs` | jjdr_load, jjdr_save implementations; ValidatedGallops newtype |
| `jjro_ops.rs` | All operation methods (nominate, slate, tally, rail, draft, retire, furlough) |
| `jjru_util.rs` | zjjrg_increment_seed, jjrg_capture_commit_sha, jjrg_make_tack, stdin helpers |

Update `lib.rs` to re-export public API from new modules.

## ValidatedGallops Newtype

```rust
pub struct ValidatedGallops(jjrg_Gallops);

impl ValidatedGallops {
    // No public constructor - only jjdr_load can create
    pub fn inner(&self) -> &jjrg_Gallops { &self.0 }
    pub fn inner_mut(&mut self) -> &mut jjrg_Gallops { &self.0 }
    pub fn into_inner(self) -> jjrg_Gallops { self.0 }
    
    #[cfg(test)]
    pub fn test_wrap(g: jjrg_Gallops) -> Self { Self(g) }
}
```

## jjdr_load Implementation

```rust
pub fn jjdr_load(path: &Path) -> Result<ValidatedGallops, String> {
    let original_bytes = fs::read(path)?;
    let gallops: jjrg_Gallops = serde_json::from_slice(&original_bytes)?;
    let reserialized = serde_json::to_string_pretty(&gallops)?;
    if reserialized.as_bytes() \!= original_bytes {
        return Err(format\!("Round-trip validation failed at byte {}", 
            find_first_diff(&original_bytes, reserialized.as_bytes())));
    }
    gallops.jjrg_validate()?;
    Ok(ValidatedGallops(gallops))
}
```

## jjdr_save Implementation

```rust
pub fn jjdr_save(gallops: &jjrg_Gallops, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(gallops)?;
    let temp_path = path.with_extension(format\!("tmp.{}.json", std::process::id()));
    fs::write(&temp_path, &json)?;
    // Validate what we wrote by loading it back
    match jjdr_load(&temp_path) {
        Ok(_) => {
            fs::rename(&temp_path, path)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::remove_file(&temp_path);
            Err(format\!("Save validation failed: {}", e))
        }
    }
}
```

## CLI Updates

Update jjrx_cli.rs to use jjdr_load/jjdr_save instead of direct jjrg_load/jjrg_save calls.

## Test Updates

- Move tests to corresponding new modules
- Add round-trip validation tests
- Add ValidatedGallops newtype tests
- Ensure all existing tests pass after refactor

### vos-claude-assets-spec (₢AAABQ) [complete]

**[260118-1450] complete**

VOS updated with claude/ parcel structure, release collection by cipher pattern, install routing from claude/* to .claude/*, and uninstall removal. All four acceptance criteria met.

**[260118-1447] bridled**

Update VOS to specify Claude config asset collection and routing.

## Scope

Add to VOS:
1. New `claude/` parcel root (parallel to `kits/`)
2. Release collection: for each kit's cipher, scan `.claude/commands/{cipher}c-*.md` and `.claude/hooks/{cipher}h-*.md`
3. Install routing: `claude/commands/*` → `.claude/commands/`, `claude/hooks/*` → `.claude/hooks/`
4. Uninstall removal: remove by cipher pattern from target `.claude/` directories

## Parcel Structure Addition

```
vvk-parcel-{hallmark}/
  vvi_install.sh
  vvbf_brand.json
  kits/                    # Kit assets (existing)
  claude/                  # Claude config assets (new)
    commands/
      jjc-heat-mount.md
      cma-normalize.md
      vvc-commit.md
    hooks/                 # Future
```

## Key Points

- Commands/hooks live at kit forge `.claude/` (single source of truth)
- Release collects by cipher prefix using existing DISTRIBUTABLE_KITS mapping
- No new lookup tables - use vofc_registry cipher→kit mapping
- Pattern: `{cipher.prefix()}{SUFFIX}*.md` where SUFFIX is "c-" or "h-"

## Files to Update

- Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc

## Acceptance

- VOS specifies claude/ parcel structure
- VOS specifies release collect behavior for commands/hooks
- VOS specifies install routing from claude/* to .claude/*
- VOS specifies uninstall removal by cipher pattern

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: VOS-VoxObscuraSpec.adoc (1 file)

NOTE: Spec-only — update VOS documentation. No Rust code changes.

Steps:
1. Update vose_parcel structure diagram: add claude/ directory with commands/ and hooks/ subdirectories
2. Update vosor_release behavior: add step for collecting commands/hooks from kit forge .claude/ by cipher pattern
3. Update vosoi_install behavior: change command routing from kits/{kit}/commands/ to claude/commands/
4. Verify vosou_uninstall: confirm cipher-pattern removal covers new routing (likely no change needed)
Verify: Read file to confirm all four acceptance criteria met

**[260118-1446] bridled**

Update VOS to specify Claude config asset collection and routing.

## Scope

Add to VOS:
1. New `claude/` parcel root (parallel to `kits/`)
2. Release collection: for each kit's cipher, scan `.claude/commands/{cipher}c-*.md` and `.claude/hooks/{cipher}h-*.md`
3. Install routing: `claude/commands/*` → `.claude/commands/`, `claude/hooks/*` → `.claude/hooks/`
4. Uninstall removal: remove by cipher pattern from target `.claude/` directories

## Parcel Structure Addition

```
vvk-parcel-{hallmark}/
  vvi_install.sh
  vvbf_brand.json
  kits/                    # Kit assets (existing)
  claude/                  # Claude config assets (new)
    commands/
      jjc-heat-mount.md
      cma-normalize.md
      vvc-commit.md
    hooks/                 # Future
```

## Key Points

- Commands/hooks live at kit forge `.claude/` (single source of truth)
- Release collects by cipher prefix using existing DISTRIBUTABLE_KITS mapping
- No new lookup tables - use vofc_registry cipher→kit mapping
- Pattern: `{cipher.prefix()}{SUFFIX}*.md` where SUFFIX is "c-" or "h-"

## Files to Update

- Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc

## Acceptance

- VOS specifies claude/ parcel structure
- VOS specifies release collect behavior for commands/hooks
- VOS specifies install routing from claude/* to .claude/*
- VOS specifies uninstall removal by cipher pattern

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: VOS-VoxObscuraSpec.adoc (1 file)
Steps:
1. Update vose_parcel structure diagram: add claude/ directory with commands/ and hooks/ subdirectories
2. Update vosor_release behavior: add step for collecting commands/hooks from kit forge .claude/ by cipher pattern
3. Update vosoi_install behavior: change command routing from kits/{kit}/commands/ to claude/commands/
4. Verify vosou_uninstall: confirm cipher-pattern removal covers new routing (likely no change needed)
Verify: Read file to confirm all four acceptance criteria met

**[260118-1441] rough**

Update VOS to specify Claude config asset collection and routing.

## Scope

Add to VOS:
1. New `claude/` parcel root (parallel to `kits/`)
2. Release collection: for each kit's cipher, scan `.claude/commands/{cipher}c-*.md` and `.claude/hooks/{cipher}h-*.md`
3. Install routing: `claude/commands/*` → `.claude/commands/`, `claude/hooks/*` → `.claude/hooks/`
4. Uninstall removal: remove by cipher pattern from target `.claude/` directories

## Parcel Structure Addition

```
vvk-parcel-{hallmark}/
  vvi_install.sh
  vvbf_brand.json
  kits/                    # Kit assets (existing)
  claude/                  # Claude config assets (new)
    commands/
      jjc-heat-mount.md
      cma-normalize.md
      vvc-commit.md
    hooks/                 # Future
```

## Key Points

- Commands/hooks live at kit forge `.claude/` (single source of truth)
- Release collects by cipher prefix using existing DISTRIBUTABLE_KITS mapping
- No new lookup tables - use vofc_registry cipher→kit mapping
- Pattern: `{cipher.prefix()}{SUFFIX}*.md` where SUFFIX is "c-" or "h-"

## Files to Update

- Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc

## Acceptance

- VOS specifies claude/ parcel structure
- VOS specifies release collect behavior for commands/hooks
- VOS specifies install routing from claude/* to .claude/*
- VOS specifies uninstall removal by cipher pattern

### implement-claude-assets-conveyance (₢AAABR) [complete]

**[260118-1504] complete**

Implemented Claude config asset conveyance - signet constants, release collection, emplace routing, vacate removal using shared constants. Build passes, 21 tests pass.

**[260118-1453] bridled**

Implement Claude config asset collection, routing, and removal in Rust.

## Scope

Implement the VOS-specified behavior for commands and hooks:
1. Release collect: scan `.claude/commands/{cipher}c-*.md` and `.claude/hooks/{cipher}h-*.md`
2. Install routing: copy `claude/*` from parcel to target `.claude/*`
3. Uninstall removal: delete by cipher pattern from target `.claude/*`

## Implementation Guidance

Follow RCG (Tools/vok/vov_veiled/RCG-RustCodingGuide.md) expressly.

**Constants**: Define suffix constants in vofc_registry.rs:
```rust
pub const VOFC_COMMAND_SIGNET_SUFFIX: &str = "c-";
pub const VOFC_HOOK_SIGNET_SUFFIX: &str = "h-";
```

**Pattern composition**: Use constants when building glob patterns:
```rust
let command_pattern = format!("{}{}*.md", cipher.prefix(), VOFC_COMMAND_SIGNET_SUFFIX);
// e.g., "jjc-*.md"
```

**Use constants** in both collection (release) and removal (uninstall) code paths.

## Files to Modify

- Tools/vok/vof/src/vofc_registry.rs (add constants)
- Tools/vok/vof/src/vofb_release.rs (or equivalent - add collection)
- Tools/vok/vof/src/vofe_emplace.rs (add routing)
- Tools/vok/vof/src/vofv_vacate.rs (add removal)

## Acceptance

- Release creates parcel with `claude/commands/` containing cipher-matched files
- Install copies `claude/commands/*` to target `.claude/commands/`
- Uninstall removes `{cipher}c-*.md` from target `.claude/commands/`
- Same pattern works for hooks (even if none exist yet)
- Build passes, tests pass

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vofc_registry.rs, vofr_release.rs, vofe_emplace.rs (3 files)

Steps:
1. Add VOFC_COMMAND_SIGNET_SUFFIX and VOFC_HOOK_SIGNET_SUFFIX constants to vofc_registry.rs
2. Update vofr_release.rs: scan kit forge .claude/commands/ and .claude/hooks/ for {cipher}{suffix}*.md, copy to parcel claude/commands/ and claude/hooks/
3. Update vofe_emplace.rs: route from parcel claude/* to target .claude/* (emplace), use constants for pattern matching (vacate already uses inline patterns — update to use constants)
4. Follow RCG expressly throughout
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260118-1442] rough**

Implement Claude config asset collection, routing, and removal in Rust.

## Scope

Implement the VOS-specified behavior for commands and hooks:
1. Release collect: scan `.claude/commands/{cipher}c-*.md` and `.claude/hooks/{cipher}h-*.md`
2. Install routing: copy `claude/*` from parcel to target `.claude/*`
3. Uninstall removal: delete by cipher pattern from target `.claude/*`

## Implementation Guidance

Follow RCG (Tools/vok/vov_veiled/RCG-RustCodingGuide.md) expressly.

**Constants**: Define suffix constants in vofc_registry.rs:
```rust
pub const VOFC_COMMAND_SIGNET_SUFFIX: &str = "c-";
pub const VOFC_HOOK_SIGNET_SUFFIX: &str = "h-";
```

**Pattern composition**: Use constants when building glob patterns:
```rust
let command_pattern = format!("{}{}*.md", cipher.prefix(), VOFC_COMMAND_SIGNET_SUFFIX);
// e.g., "jjc-*.md"
```

**Use constants** in both collection (release) and removal (uninstall) code paths.

## Files to Modify

- Tools/vok/vof/src/vofc_registry.rs (add constants)
- Tools/vok/vof/src/vofb_release.rs (or equivalent - add collection)
- Tools/vok/vof/src/vofe_emplace.rs (add routing)
- Tools/vok/vof/src/vofv_vacate.rs (add removal)

## Acceptance

- Release creates parcel with `claude/commands/` containing cipher-matched files
- Install copies `claude/commands/*` to target `.claude/commands/`
- Uninstall removes `{cipher}c-*.md` from target `.claude/commands/`
- Same pattern works for hooks (even if none exist yet)
- Build passes, tests pass

### vos-claudemd-freshening-spec (₢AAABA) [complete]

**[260117-1127] complete**

Specify CLAUDE.md freshening behavior in VOS.

## Deliverables

1. **Update vosoi_install procedure**:
   - Remove fatal on missing markers
   - Add: no markers → append section at end of file
   - Add: UNINSTALLED marker → expand to BEGIN/END at that location

2. **Update vosou_uninstall procedure**:
   - Change: delete content, replace BEGIN/END with single UNINSTALLED marker
   - Document: preserves user's section ordering for re-install

3. **Add vose_uninstalled_marker entity**:
   - Format: `<!-- MANAGED:{tag}:UNINSTALLED -->`
   - Semantics: placeholder preserving position

4. **Clarify ordering rules**:
   - Multiple sections per kit: order follows manifest declaration order
   - User reordering: preserved across install cycles (via marker positions)

5. **Document programmatic approach**:
   - Rust implementation, not Claude-assisted
   - Add note in VOS that this is deterministic text transformation

## References

- Current spec: VOS lines 1360-1372 (install), 1437-1444 (uninstall)
- Paddock CLAUDE.md Freshening section
- Pace ₢AAAAG depends on this spec work

**[260117-1121] bridled**

Specify CLAUDE.md freshening behavior in VOS.

## Deliverables

1. **Update vosoi_install procedure**:
   - Remove fatal on missing markers
   - Add: no markers → append section at end of file
   - Add: UNINSTALLED marker → expand to BEGIN/END at that location

2. **Update vosou_uninstall procedure**:
   - Change: delete content, replace BEGIN/END with single UNINSTALLED marker
   - Document: preserves user's section ordering for re-install

3. **Add vose_uninstalled_marker entity**:
   - Format: `<!-- MANAGED:{tag}:UNINSTALLED -->`
   - Semantics: placeholder preserving position

4. **Clarify ordering rules**:
   - Multiple sections per kit: order follows manifest declaration order
   - User reordering: preserved across install cycles (via marker positions)

5. **Document programmatic approach**:
   - Rust implementation, not Claude-assisted
   - Add note in VOS that this is deterministic text transformation

## References

- Current spec: VOS lines 1360-1372 (install), 1437-1444 (uninstall)
- Paddock CLAUDE.md Freshening section
- Pace ₢AAAAG depends on this spec work

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: Tools/vok/VOS-VoxObscuraSpec.adoc (1 file)
Steps:
1. Add vose_uninstalled_marker attribute to mapping section (near vose_marker)
2. Add entity definition for vose_uninstalled_marker after vose_marker definition
3. Update vosoi_install procedure: remove {vosc_fatal} for missing markers; add conditional for UNINSTALLED marker (expand to BEGIN/END at that location); add conditional for no markers (append section at end of file)
4. Update vosou_uninstall procedure: change from preserve markers to replace BEGIN/END with single UNINSTALLED marker
5. Add implementation note: programmatic Rust, deterministic text transformation
Verify: Visual review (no build for spec-only change)

**[260117-1115] rough**

Specify CLAUDE.md freshening behavior in VOS.

## Deliverables

1. **Update vosoi_install procedure**:
   - Remove fatal on missing markers
   - Add: no markers → append section at end of file
   - Add: UNINSTALLED marker → expand to BEGIN/END at that location

2. **Update vosou_uninstall procedure**:
   - Change: delete content, replace BEGIN/END with single UNINSTALLED marker
   - Document: preserves user's section ordering for re-install

3. **Add vose_uninstalled_marker entity**:
   - Format: `<!-- MANAGED:{tag}:UNINSTALLED -->`
   - Semantics: placeholder preserving position

4. **Clarify ordering rules**:
   - Multiple sections per kit: order follows manifest declaration order
   - User reordering: preserved across install cycles (via marker positions)

5. **Document programmatic approach**:
   - Rust implementation, not Claude-assisted
   - Add note in VOS that this is deterministic text transformation

## References

- Current spec: VOS lines 1360-1372 (install), 1437-1444 (uninstall)
- Paddock CLAUDE.md Freshening section
- Pace ₢AAAAG depends on this spec work

### vos-brand-hallmark-system (₢AAAA9) [complete]

**[260117-1102] complete**

Added brand/hallmark system to VOS spec: vose_brand_file, vose_registry, vost_hallmark types; amended vosor_release and vosoi_install operations.

**[260117-1048] rough**

Add brand/hallmark system to VOS specification.

NEW CONCEPTS:
1. vose_brand_file (axj_structure) — Runtime identity file at parcel root
   - vosem_hallmark (axj_field): vvbh_hallmark — 4-digit version identifier (string, starts at 1000)
   - vosem_brand_date (axj_field): vvbd_date — Minted timestamp (YYMMDD-HHMM)
   - vosem_brand_sha (axj_field): vvbs_sha — Content super-SHA
2. vose_registry (axj_structure) — Veiled registry mapping hallmark to {date, sha}
   - File: Tools/vok/vov_veiled/vovr_registry.json
   - Keys are hallmark values; entries use vvbd_date and vvbs_sha field names
3. vost_hallmark type — 4-digit string starting at 1000, increments for each unique content set

AMEND vosor_release:
- Compute order-independent super-SHA (sort files, hash path+content, combine)
- Exclude vvbf_brand_file.json from SHA computation
- Look up super-SHA in vovr_registry.json
- If new: allocate next hallmark, update registry, commit
- Write vvbf_brand_file.json to parcel root
- Parcel naming: vvk-parcel-{hallmark}.tar.gz (drop sigil from name)

AMEND vosoi_install:
- Read vvbf_brand_file.json from parcel
- Write hallmark to target manifest

ACCEPTANCE:
- VOS has complete brand file and registry entity definitions
- vosor_release behavior documents hallmark allocation flow
- vosoi_install behavior documents brand file consumption
- Field tags explicitly documented via axj_field_tag pattern

### jjk-test-compilation-fix (₢AAAAx) [complete]

**[260116-1438] complete**

Fixed 35 test compilation errors by applying RCG prefix conventions across jjrf_favor.rs, jjrn_notch.rs, jjrq_query.rs, and jjrg_gallops.rs. All 133 tests pass.

**[260116-1433] rough**

Fix 35 test compilation errors from incomplete RCG prefixing (₢AAAAY).

## Errors by category

1. **17 errors**: `.as_str()` on Firemark/Coronet → `.jjrf_as_str()`
2. **10 errors**: `.jjrf_as_str()` on HeatAction/ChalkMarker → `.jjrn_as_str()`
3. **7 errors**: `Pace` type not found → `jjrg_Pace`
4. **1 error**: Unresolved imports `Heat`, `Pace`, `Tack` → prefixed names

## Files

- Tools/jjk/veiled/src/jjrn_notch.rs (tests)
- Tools/jjk/veiled/src/jjrf_favor.rs (tests)
- Tools/jjk/veiled/src/jjrq_query.rs (tests)

## Verification

```bash
cargo test --manifest-path Tools/jjk/veiled/Cargo.toml
```

All 133+ tests must pass.

## Context

Pace ₢AAAAY prefixed declarations but tests still use old names. Tests are inline (not yet split to jjt*.rs files per RCG).

### fix-gallops-commit-scope (₢AAAAs) [complete]

**[260116-1450] complete**

Haiku agent applied jjx_retire pattern to slate/tally/rail/nominate. Fixed jjrf_display→jjrf_as_str for paddock paths. All 133 tests pass.

**[260116-1448] bridled**

Fix gallops commit scope: slate/tally/rail should only stage jjg_gallops.json + heat-specific paddock.

## Problem

jjx_slate, jjx_tally, jjx_rail use vvcc_commit which does git add -A, staging everything. This causes tangles when unrelated work is in progress.

## Solution

Switch from vvcc_commit to vvcm_commit (machine commit) with explicit file list:
- jjg_gallops.json (always)
- jjp_{firemark}.md (heat-specific paddock)

## Pattern

jjx_retire already does this correctly (lines 663-684 in jjrx_cli.rs).

## Files to change

- Tools/jjk/veiled/src/jjrx_cli.rs — 4 operations: slate, tally, rail, nominate

## Firemark derivation

- slate/rail: firemark from CLI arg
- tally: derive from coronet (first 2 chars after ₢)
- nominate: firemark from result

## JJD update

Not required — spec does not mention commit behavior.

*Direction:* Agent: haiku
Pattern: Copy jjx_retire pattern (lines 663-684) to slate, tally, rail, nominate
Files: jjrx_cli.rs only
Changes:
1. Replace vvcc_commit() calls with vvcm_CommitArgs + machine_commit()
2. Build explicit file list: gallops_path + paddock_path (derive from firemark)
3. Firemark derivation: slate/rail from CLI arg, tally from coronet[0:2], nominate from result
Verify: cargo test --manifest-path Tools/jjk/veiled/Cargo.toml

**[260116-1419] rough**

Fix gallops commit scope: slate/tally/rail should only stage jjg_gallops.json + heat-specific paddock.

## Problem

jjx_slate, jjx_tally, jjx_rail use vvcc_commit which does git add -A, staging everything. This causes tangles when unrelated work is in progress.

## Solution

Switch from vvcc_commit to vvcm_commit (machine commit) with explicit file list:
- jjg_gallops.json (always)
- jjp_{firemark}.md (heat-specific paddock)

## Pattern

jjx_retire already does this correctly (lines 663-684 in jjrx_cli.rs).

## Files to change

- Tools/jjk/veiled/src/jjrx_cli.rs — 4 operations: slate, tally, rail, nominate

## Firemark derivation

- slate/rail: firemark from CLI arg
- tally: derive from coronet (first 2 chars after ₢)
- nominate: firemark from result

## JJD update

Not required — spec does not mention commit behavior.

### notch-size-limit-flag (₢AAAAt) [complete]

**[260116-1457] complete**

Added --size-limit parameter to jjx_notch for legitimate large commits. Updated vvcc_CommitArgs with optional size_limit/warn_limit fields, propagated through JJK CLI, and documented in JJD spec and jjc-pace-notch.md.

**[260116-1450] bridled**

Add --size-limit parameter to jjx_notch for legitimate large commits.

## Rationale

Guard protects against accidental large commits (default 50KB). Legitimate large work (e.g., RCG renames across 7 files) needs an escape hatch. The pace spec serves as justification.

## Files to change

1. Tools/vvc/src/vvcc_commit.rs
   - Add optional size_limit/warn_limit fields to vvcc_CommitArgs
   - Propagate to guard check (None = use default)

2. Tools/jjk/veiled/src/jjrx_cli.rs
   - Add --size-limit to jjrx_NotchArgs
   - Pass to vvcc_CommitArgs

3. Tools/jjk/JJD-GallopsData.adoc
   - Document --size-limit argument for jjdo_notch
   - Note default limit (50KB) and guard behavior

4. .claude/commands/jjc-pace-notch.md
   - Document that --size-limit requires justification in pace spec

## Design

vvcc_CommitArgs gains:
```rust
pub size_limit: Option<u64>,  // None = use VVCC_SIZE_LIMIT (50KB)
pub warn_limit: Option<u64>,  // None = use VVCC_WARN_LIMIT (30KB)
```

*Direction:* Agent: sonnet. 1) Read vvcc_commit.rs, add to vvcc_CommitArgs: pub size_limit: Option<u64> and pub warn_limit: Option<u64>. Update guard call to use args.size_limit.unwrap_or(VVCC_SIZE_LIMIT) and similar for warn. 2) Read jjrx_cli.rs, add to jjrx_NotchArgs: #[arg(long)] size_limit: Option<u64>. Pass to vvcc_CommitArgs. 3) Read JJD-GallopsData.adoc, find jjdo_notch arguments, add --size-limit (optional) with description. 4) Read jjc-pace-notch.md, add note that --size-limit requires justification in pace spec. 5) Verify: cargo build --features jjk in Tools/vok.

**[260116-1419] rough**

Add --size-limit parameter to jjx_notch for legitimate large commits.

## Rationale

Guard protects against accidental large commits (default 50KB). Legitimate large work (e.g., RCG renames across 7 files) needs an escape hatch. The pace spec serves as justification.

## Files to change

1. Tools/vvc/src/vvcc_commit.rs
   - Add optional size_limit/warn_limit fields to vvcc_CommitArgs
   - Propagate to guard check (None = use default)

2. Tools/jjk/veiled/src/jjrx_cli.rs
   - Add --size-limit to jjrx_NotchArgs
   - Pass to vvcc_CommitArgs

3. Tools/jjk/JJD-GallopsData.adoc
   - Document --size-limit argument for jjdo_notch
   - Note default limit (50KB) and guard behavior

4. .claude/commands/jjc-pace-notch.md
   - Document that --size-limit requires justification in pace spec

## Design

vvcc_CommitArgs gains:
```rust
pub size_limit: Option<u64>,  // None = use VVCC_SIZE_LIMIT (50KB)
pub warn_limit: Option<u64>,  // None = use VVCC_WARN_LIMIT (30KB)
```

### rcg-copyright-templates (₢AAAAu) [complete]

**[260116-1514] complete**

Added File Templates section to RCG (proprietary + Apache 2.0). Applied proprietary 3-line header to all 14 Rust files via 14 parallel Haiku agents. Build verified.

**[260116-1424] bridled**

Add copyright templates to RCG and apply proprietary header to all Rust files.

## RCG changes (1 file, Sonnet)

Add "File Templates" section to Tools/vok/lenses/RCG-RustCodingGuide.md with two templates:

### Template 1: Proprietary
```rust
// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
```

### Template 2: Open Source (Apache 2.0)
```rust
// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
```

Placement note: Header at very top, before //\! doc comments or #\![...] attributes.

## Rust file updates (14 files, parallel Haiku agents)

Prepend proprietary 3-line header to each file. Mechanical, independent edits.

**Batch 1 (7 agents): Tools/jjk/veiled/src/**
- jjrc_core.rs, jjrf_favor.rs, jjrg_gallops.rs, jjrn_notch.rs
- jjrq_query.rs, jjrs_steeplechase.rs, jjrx_cli.rs, lib.rs

**Batch 2 (6 agents): Tools/vvc/src/ + Tools/vok/**
- vvcc_commit.rs, vvcg_guard.rs, vvcm_machine.rs, lib.rs
- build.rs, src/vorm_main.rs

## Execution strategy

1. Sonnet edits RCG (add File Templates section)
2. Launch 14 parallel Haiku agents (single message, 14 Task tool calls)
3. Each agent: Read file, prepend 3-line header, Write file
4. Verify: cargo build in Tools/vok

*Direction:* 1) Sonnet edits RCG: add File Templates section after Quick Reference. 2) Launch 14 parallel Haiku Task agents (single message, 14 Task tool calls). Each agent: read assigned Rust file, prepend 3-line proprietary header (// Copyright 2026 Scale Invariant, Inc. // All rights reserved. // SPDX-License-Identifier: LicenseRef-Proprietary), write file. Files: jjrc_core.rs, jjrf_favor.rs, jjrg_gallops.rs, jjrn_notch.rs, jjrq_query.rs, jjrs_steeplechase.rs, jjrx_cli.rs, lib.rs (jjk), vvcc_commit.rs, vvcg_guard.rs, vvcm_machine.rs, lib.rs (vvc), build.rs, vorm_main.rs. 3) Verify: cargo build --features jjk in Tools/vok.

**[260116-1424] rough**

Add copyright templates to RCG and apply proprietary header to all Rust files.

## RCG changes (1 file, Sonnet)

Add "File Templates" section to Tools/vok/lenses/RCG-RustCodingGuide.md with two templates:

### Template 1: Proprietary
```rust
// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
```

### Template 2: Open Source (Apache 2.0)
```rust
// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
```

Placement note: Header at very top, before //\! doc comments or #\![...] attributes.

## Rust file updates (14 files, parallel Haiku agents)

Prepend proprietary 3-line header to each file. Mechanical, independent edits.

**Batch 1 (7 agents): Tools/jjk/veiled/src/**
- jjrc_core.rs, jjrf_favor.rs, jjrg_gallops.rs, jjrn_notch.rs
- jjrq_query.rs, jjrs_steeplechase.rs, jjrx_cli.rs, lib.rs

**Batch 2 (6 agents): Tools/vvc/src/ + Tools/vok/**
- vvcc_commit.rs, vvcg_guard.rs, vvcm_machine.rs, lib.rs
- build.rs, src/vorm_main.rs

## Execution strategy

1. Sonnet edits RCG (add File Templates section)
2. Launch 14 parallel Haiku agents (single message, 14 Task tool calls)
3. Each agent: Read file, prepend 3-line header, Write file
4. Verify: cargo build in Tools/vok

**[260116-1420] rough**

Add copyright templates to RCG and apply proprietary header to all Rust files.

## RCG changes

Add "File Templates" section with two templates:

### Template 1: Proprietary
```rust
// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
```

### Template 2: Open Source (Apache 2.0)
```rust
// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// ... full Apache 2.0 text ...
```

Placement note: Header at very top, before //\! doc comments or #\![...] attributes.

## Rust files to update (14 files, proprietary header)

Tools/jjk/veiled/src/:
- jjrc_core.rs, jjrf_favor.rs, jjrg_gallops.rs, jjrn_notch.rs
- jjrq_query.rs, jjrs_steeplechase.rs, jjrx_cli.rs, lib.rs

Tools/vvc/src/:
- vvcc_commit.rs, vvcg_guard.rs, vvcm_machine.rs, lib.rs

Tools/vok/:
- build.rs, src/vorm_main.rs

## Files

- Tools/vok/lenses/RCG-RustCodingGuide.md (add section)
- 14 Rust source files (add proprietary header)

### rail-descriptive-commits (₢AAAAw) [complete]

**[260116-1515] complete**

Improved jjx_rail commit messages: move mode shows 'moved ₢XXXXX {position}', order mode shows 'order: coronet1, coronet2, ...'

**[260116-1501] bridled**

Improve jjx_rail commit messages to be descriptive instead of just "reordered".

## Move mode

Format: "moved {coronet} {position}"

Examples:
- "moved ₢AAAAC after ₢AAAAB"
- "moved ₢AAAAC before ₢AAAAD"
- "moved ₢AAAAC to first"
- "moved ₢AAAAC to last"

## Order mode

Format: "order: {coronet1}, {coronet2}, ..."

Just list the new order. No diff presentation needed.

## Files

- Tools/jjk/veiled/src/jjrx_cli.rs — zjjrx_run_rail function, around line 886

## Implementation

Replace hardcoded "reordered" with:
```rust
let subject = if let Some(ref moved) = args.r#move {
    // Move mode
    let target = if args.first { "to first".to_string() }
        else if args.last { "to last".to_string() }
        else if let Some(ref b) = args.before { format\!("before {}", b) }
        else if let Some(ref a) = args.after { format\!("after {}", a) }
        else { "???".to_string() };
    format\!("moved {} {}", moved, target)
} else {
    // Order mode - list new order
    format\!("order: {}", new_order.join(", "))
};
```

## Verification

- cargo build && cargo test
- Manual test: rail a pace, check commit message

*Direction:* Agent: haiku
Cardinality: 1 sequential
Files: jjrx_cli.rs (1 file)
Steps:
1. Read jjrx_cli.rs, find zjjrx_run_rail function (~line 868)
2. Before line 919 (let commit_args), insert subject computation using args.r#move/first/last/before/after for move mode, or new_order.join for order mode
3. Replace "reordered" on line 924 with &subject
Verify: cargo build --manifest-path Tools/vok/Cargo.toml --features jjk

**[260116-1433] rough**

Improve jjx_rail commit messages to be descriptive instead of just "reordered".

## Move mode

Format: "moved {coronet} {position}"

Examples:
- "moved ₢AAAAC after ₢AAAAB"
- "moved ₢AAAAC before ₢AAAAD"
- "moved ₢AAAAC to first"
- "moved ₢AAAAC to last"

## Order mode

Format: "order: {coronet1}, {coronet2}, ..."

Just list the new order. No diff presentation needed.

## Files

- Tools/jjk/veiled/src/jjrx_cli.rs — zjjrx_run_rail function, around line 886

## Implementation

Replace hardcoded "reordered" with:
```rust
let subject = if let Some(ref moved) = args.r#move {
    // Move mode
    let target = if args.first { "to first".to_string() }
        else if args.last { "to last".to_string() }
        else if let Some(ref b) = args.before { format\!("before {}", b) }
        else if let Some(ref a) = args.after { format\!("after {}", a) }
        else { "???".to_string() };
    format\!("moved {} {}", moved, target)
} else {
    // Order mode - list new order
    format\!("order: {}", new_order.join(", "))
};
```

## Verification

- cargo build && cargo test
- Manual test: rail a pace, check commit message

### parade-remaining-flag (₢AAAAy) [complete]

**[260116-1517] complete**

Added --remaining flag to jjx_parade to filter out complete/abandoned paces. Updated jjrx_cli.rs, jjrq_query.rs (with filtering in Overview/Order/Full formats), and JJD-GallopsData.adoc.

**[260116-1439] bridled**

Add --remaining flag to jjx_parade to filter out complete/abandoned paces.

## Files to change

1. **Tools/jjk/veiled/src/jjrx_cli.rs** (~line 155)
   - Add to zjjrx_ParadeArgs: `#[arg(long)] remaining: bool`
   - Pass through to LibParadeArgs

2. **Tools/jjk/veiled/src/jjrq_query.rs** (~line 178, 186)
   - Add to jjrq_ParadeArgs: `pub remaining: bool`
   - In jjrq_run_parade: skip paces where state is "complete" or "abandoned"

3. **Tools/jjk/JJD-GallopsData.adoc**
   - Document --remaining argument for jjdo_parade

## Implementation

In jjrq_run_parade pace iteration:
```rust
if args.remaining && (state == "complete" || state == "abandoned") {
    continue;
}
```

## Verification

- cargo build && cargo test
- ./tt/vvw-r.RunVVX.sh jjx_parade AA --format overview --remaining

*Direction:* Agent: sonnet. 1) Read jjrx_cli.rs, find zjjrx_ParadeArgs (~line 155), add: #[arg(long)] remaining: bool. 2) Read jjrq_query.rs, find jjrq_ParadeArgs (~line 178), add: pub remaining: bool. In jjrq_run_parade (~line 186), add filter: skip paces where tack.state is complete or abandoned when args.remaining is true. 3) Read JJD-GallopsData.adoc, find jjdo_parade arguments section, add --remaining documentation. 4) Verify: cargo build --features jjk in Tools/vok.

**[260116-1439] rough**

Add --remaining flag to jjx_parade to filter out complete/abandoned paces.

## Files to change

1. **Tools/jjk/veiled/src/jjrx_cli.rs** (~line 155)
   - Add to zjjrx_ParadeArgs: `#[arg(long)] remaining: bool`
   - Pass through to LibParadeArgs

2. **Tools/jjk/veiled/src/jjrq_query.rs** (~line 178, 186)
   - Add to jjrq_ParadeArgs: `pub remaining: bool`
   - In jjrq_run_parade: skip paces where state is "complete" or "abandoned"

3. **Tools/jjk/JJD-GallopsData.adoc**
   - Document --remaining argument for jjdo_parade

## Implementation

In jjrq_run_parade pace iteration:
```rust
if args.remaining && (state == "complete" || state == "abandoned") {
    continue;
}
```

## Verification

- cargo build && cargo test
- ./tt/vvw-r.RunVVX.sh jjx_parade AA --format overview --remaining

### notch-remove-message-bypass (₢AAAAn) [complete]

**[260116-1207] complete**

Removed --message bypass from jjx_notch. Commits now always use Claude-generated messages. Updated JJD spec, CLI args, and slash command.

**[260116-1203] rough**

Remove --message flag from jjx_notch. Commits should always have Claude-analyzed messages. Use chalk for explicit human messages (ceremony). Update: JJD spec, jjrn_notch.rs, jjrx_cli.rs (remove --message arg), jjc-pace-notch.md slash command.

### jjd-retire-spec-update (₢AAAAg) [complete]

**[260116-1108] complete**

Updated jjdo_retire spec: moved to Write Operations, changed output from JSON to markdown trophy format, added full lifecycle (lock, write trophy, remove from gallops, delete paddock, commit). Commit format deferred to unify-commit-format pace.

**[260116-1056] rough**

Update JJD spec: jjx_retire must remove heat from gallops.json, delete paddock file, and git commit - not just extract trophy data

### jjrc-commit-helper (₢AAAAk) [complete]

**[260116-1135] complete**

Added vvcm_machine.rs to VVC crate with machine commit infrastructure. Provides vvcm_CommitArgs and vvcm_commit() for programmatic operations: explicit file staging, custom guard limits, no Co-Authored-By. Placed in VVC as shared infrastructure rather than JJK-specific.

**[260116-1117] rough**

Add jjrc_commit helper to JJK Rust: reuses vvc lock, stages explicit file list, runs vvcg_guard with custom limit (200KB for retire), commits with provided message (no Claude). Used by jjx_retire and potentially other JJD write ops.

### implement-jjx-retire-full (₢AAAAi) [complete]

**[260116-1153] complete**

Implemented full retire operation in jjx_retire with --execute flag. Rust handles complete lifecycle: acquire lock, build trophy content, write to retired/, remove heat from gallops.json, delete paddock file, commit via vvcm_commit.

**[260116-1056] rough**

Implement full retire in jjx_retire Rust: extract trophy data, write trophy file to retired/, remove heat from gallops.json, delete paddock file, git commit with message 'Retire: ₣{firemark} {silks}'

### vvcc-optimize-claude-call (₢AAAAm) [complete]

**[260116-1155] complete**

Added --system-prompt, --model haiku, and --no-session-persistence flags to claude CLI invocation in zvvcc_generate_message_with_claude(). Build verified.

**[260116-1145] bridled**

Ready for autonomous execution

*Direction:* Haiku agent. Read Tools/vvc/src/vvcc_commit.rs, locate zvvcc_generate_message_with_claude() (~line 191), update Command::new("claude").args() to include: "--system-prompt", "Output only a conventional git commit message. No explanation or commentary.", "--model", "haiku", "--no-session-persistence". Verify cargo build -p vvc succeeds. No commit.

**[260116-1144] rough**

Optimize VVC Claude commit message generation for speed and cost.

## Changes

File: `Tools/vvc/src/vvcc_commit.rs`

Update `zvvcc_generate_message_with_claude()` (lines 191-217):

```rust
fn zvvcc_generate_message_with_claude(diff: &str) -> Result<String, String> {
    const SYSTEM: &str = "Output only a conventional git commit message. No explanation or commentary.";
    
    let prompt = format!("<diff>\n{}\n</diff>", diff);

    eprintln!("commit: invoking claude for commit message...");

    let output = Command::new("claude")
        .args([
            "--print",
            "--system-prompt", SYSTEM,
            "--model", "haiku",
            "--no-session-persistence",
            &prompt
        ])
        .output()
        .map_err(|e| format!("Failed to invoke claude: {}", e))?;
    // ... rest unchanged
}
```

## Rationale

- `--system-prompt`: Replaces ~4000 token default with ~15 tokens
- `--model haiku`: Faster and cheaper for simple generation task
- `--no-session-persistence`: Skips disk writes for ephemeral call

**[260116-1142] rough**

Optimize VVC Claude commit message generation: switch to haiku model, add custom system prompt, add --no-session-persistence flag

### fix-retire-slash-command (₢AAAAh) [complete]

**[260116-1200] complete**

Split /jjc-heat-retire into two commands: /jjc-heat-retire-dryrun (preview only, calls jjx_retire) and /jjc-heat-retire-FINAL (with confirmation, calls jjx_retire --execute). Old command now redirects to these.

**[260116-1058] rough**

Fix /jjc-heat-retire slash command: (1) Prompt user 'This will permanently retire heat ₣XX. Are you sure?' before proceeding, (2) Call jjx_retire (which should do all the work) instead of manually editing files

**[260116-1056] rough**

Fix /jjc-heat-retire slash command to call jjx_retire (which should do all the work) instead of manually editing files

### unify-commit-format (₢AAAAd) [complete]

**[260116-1237] complete**

Unify all JJ commit formats to colon-delimited coronet-based pattern.

## New Format

Standard: jjb:RBM:₢AAAAB: message
Chalk: jjb:RBM:₢AAAAB:WRAP: description
Heat-level: jjb:RBM:₣AB:SLATE: silks
Retire: jjb:RBM:₣AB:RETIRE: silks

## Files to Update

- jjrn_notch.rs: format_notch_prefix(), format_chalk_message()
- jjrs_steeplechase.rs: rein parsing patterns
- jjrx_cli.rs: tally, slate, rail, nominate, draft, retire commit messages
- jjrc_core.rs: retire commit formatting (uses jjrc_commit helper)
- JJD-GallopsData.adoc: commit format documentation
- jjc-pace-notch.md: update format docs
- vvc-commit.md: add JJ context detection/warning

## Depends On

installation-identifier (₢AAAAc) for runtime brand lookup

**[260116-1118] rough**

Unify all JJ commit formats to colon-delimited coronet-based pattern.

## New Format

Standard: jjb:RBM:₢AAAAB: message
Chalk: jjb:RBM:₢AAAAB:WRAP: description
Heat-level: jjb:RBM:₣AB:SLATE: silks
Retire: jjb:RBM:₣AB:RETIRE: silks

## Files to Update

- jjrn_notch.rs: format_notch_prefix(), format_chalk_message()
- jjrs_steeplechase.rs: rein parsing patterns
- jjrx_cli.rs: tally, slate, rail, nominate, draft, retire commit messages
- jjrc_core.rs: retire commit formatting (uses jjrc_commit helper)
- JJD-GallopsData.adoc: commit format documentation
- jjc-pace-notch.md: update format docs
- vvc-commit.md: add JJ context detection/warning

## Depends On

installation-identifier (₢AAAAc) for runtime brand lookup

**[260116-1044] rough**

Unify all JJ commit formats to colon-delimited coronet-based pattern.

## New Format

Standard: jjb:RBM:₢AAAAB: message
Chalk: jjb:RBM:₢AAAAB:WRAP: description
Heat-level: jjb:RBM:₣AB:SLATE: silks

## Files to Update

- jjrn_notch.rs: format_notch_prefix(), format_chalk_message()
- jjrs_steeplechase.rs: rein parsing patterns
- jjrx_cli.rs: tally, slate, rail, nominate, draft commit messages
- JJD-GallopsData.adoc: commit format documentation
- jjc-pace-notch.md: update format docs
- vvc-commit.md: add JJ context detection/warning

## Depends On

installation-identifier (₢AAAAc) for runtime brand lookup

### cleanup-orphan-rein (₢AAAAe) [complete]

**[260116-1308] complete**

Removed orphaned jjw-rn route and jju_rein function. Slash command creation deferred to post-install infrastructure.

**[260116-1306] rough**

Remove orphaned jjw-rn infrastructure (shell cleanup only).

## Delete

1. jjw_workbench.sh line 113: remove jjw-rn) case
2. jju_utility.sh: remove jju_rein function (~lines 227-239)

## Rationale

These are dead code paths - jjw-rn route exists but no tabtarget, jju_rein was a bash wrapper for jjx_rein.

## Deferred

Slash command /jjc-heat-rein creation deferred until install infrastructure provides brand identifier (see ₢AAAAc, ₢AAAAF).

**[260116-1045] rough**

Remove orphaned jjw-rn infrastructure and create replacement slash command.

## Delete

1. jjw_workbench.sh line 113: remove jjw-rn) case
2. jju_utility.sh: remove jju_rein function (~lines 227-239)

## Create

/jjc-heat-rein slash command:
- Arguments: firemark (required)
- Calls: ./tt/vvw-r.RunVVX.sh jjx_rein <FIREMARK>
- Displays: steeplechase history for heat (parsed JSON → readable format)

## Depends On

- installation-identifier (₢AAAAc): removes --brand requirement from jjx_rein
- unify-commit-format (₢AAAAd): rein parsing uses new format

**[260116-1044] rough**

Remove orphaned jjw-rn infrastructure.

## Delete

1. jjw_workbench.sh line 113: remove jjw-rn) case
2. jju_utility.sh: remove jju_rein function (~lines 227-239)

## Rationale

- jjw-rn route exists but no tabtarget file
- jju_rein was bash wrapper for jjx_rein
- Will be replaced by /jjc-heat-rein slash command (uses vvx directly)

### deprecate-jju-tabtargets (₢AAAAf) [complete]

**[260116-1347] complete**

Deprecated jju/jjt infrastructure: deleted tabtargets, removed workbench routes, cleaned up testbench. Added vvw-t.TestVVX.sh tabtarget and Rust Build Discipline to CLAUDE.md.

**[260116-1339] complete**

Deprecated JJW tabtargets and JJU utility functions. Deleted tt/jjw-*.sh and tt/jjt-*.sh launchers. Removed jju_utility.sh. Cleaned jjw_workbench.sh to only route arcanum commands. Slash command coverage verified (/jjc-heat-rein tracked separately in ₢AAAAo).

**[260116-1049] rough**

Deprecate jjw tabtargets and jju utility functions.

## Starting Point - Delete Tabtargets

- tt/jjw-hr.HeatRetire.sh
- tt/jjw-i.Info.sh
- tt/jjw-m.Muster.sh
- tt/jjw-pw.PaceWrap.sh

## Audit jju_utility.sh

1. List all jju_* functions
2. Check each for callers (grep across codebase)
3. Verify slash commands cover all use cases
4. Delete unused functions

## Clean Up Workbench

- Remove all jjw-* routes from jjw_workbench.sh
- Or delete jjw_workbench.sh entirely if empty

## Create Missing Slash Commands

- /jjc-heat-retire (for jjw-hr)
- /jjc-heat-rein (already in ₢AAAAe scope)

## Verify

- All JJ operations work via slash commands
- No dangling references to jju_* or jjw-*

### retire-heat-ab-test (₢AAAAj) [complete]

**[260116-1349] complete**

Heat AB retired via jjx_retire. Trophy written, heat removed from gallops, paddock deleted, commit created (ba845a0).

**[260116-1348] complete**

Test jjx_retire end-to-end by retiring ₣AB (axla-procedure-section-motifs). Validates: trophy file written, heat removed from gallops, paddock deleted, commit created.

**[260116-1110] rough**

Test jjx_retire end-to-end by retiring ₣AB (axla-procedure-section-motifs). Validates: trophy file written, heat removed from gallops, paddock deleted, commit created.

### vvc-rcg-compliance (₢AAAAZ) [complete]

**[260116-0950] complete**

VVC RCG compliance complete. Types: vvcg_GuardArgs, vvcc_CommitArgs, vvcc_CommitLock. Functions: vvcg_run, vvcc_run, vvcc_acquire, vvcc_commit. Internals: zvvcc_*, zvvcg_*. JJK call sites updated.

**[260116-0949] complete**

VVC RCG compliance complete: vvcg_GuardArgs, vvcg_run, vvcc_CommitArgs, vvcc_CommitLock, vvcc_run. Internal funcs zvvcc/zvvcg. Updated JJK call sites.

**[260116-0944] bridled**

Bring all VVC Rust code into RCG compliance.

## Scope

All files in Tools/vvc/src/:
- lib.rs (add boilerplate)
- vvcc_commit.rs
- vvcg_guard.rs

## Required Changes

1. **Crate boilerplate**: Add `#![allow(non_camel_case_types)]` to lib.rs

2. **Type prefixing**: All pub struct/enum get file prefix
   - `CommitArgs` → `vvcc_CommitArgs`
   - `CommitLock` → `vvcc_CommitLock`
   - `GuardArgs` → `vvcg_GuardArgs`

3. **Function prefixing**: All pub fn get file prefix
   - `run` (vvcc_commit) → `vvcc_run`
   - `run` (vvcg_guard) → `vvcg_run`

4. **Impl method prefixing**: All pub methods on impl blocks get file prefix
   - `CommitLock::acquire` → `vvcc_CommitLock::vvcc_acquire`
   - `CommitLock::commit` → `vvcc_CommitLock::vvcc_commit`

5. **Internal functions**: Any non-pub helpers get z prefix

6. **Update lib.rs re-exports** to use new names

7. **Update all call sites** in VVC and any VOK code that imports VVC

## Verification

Run `cargo build` and `cargo test` after changes. All must pass.

## Note

VVC is used by JJK. Coordinate with jjk-rcg-compliance pace — run VVC first since JJK imports from VVC.

*Direction:* Execute VVC RCG compliance per RCG guide. Prefix all types/functions/constants. Update vvcg_guard.rs and vvcc_commit.rs. Update lib.rs re-exports. Update call sites in Tools/vok/src/. Run cargo build and cargo test to verify.

**[260116-0941] rough**

Bring all VVC Rust code into RCG compliance.

## Scope

All files in Tools/vvc/src/:
- lib.rs (add boilerplate)
- vvcc_commit.rs
- vvcg_guard.rs

## Required Changes

1. **Crate boilerplate**: Add `#![allow(non_camel_case_types)]` to lib.rs

2. **Type prefixing**: All pub struct/enum get file prefix
   - `CommitArgs` → `vvcc_CommitArgs`
   - `CommitLock` → `vvcc_CommitLock`
   - `GuardArgs` → `vvcg_GuardArgs`

3. **Function prefixing**: All pub fn get file prefix
   - `run` (vvcc_commit) → `vvcc_run`
   - `run` (vvcg_guard) → `vvcg_run`

4. **Impl method prefixing**: All pub methods on impl blocks get file prefix
   - `CommitLock::acquire` → `vvcc_CommitLock::vvcc_acquire`
   - `CommitLock::commit` → `vvcc_CommitLock::vvcc_commit`

5. **Internal functions**: Any non-pub helpers get z prefix

6. **Update lib.rs re-exports** to use new names

7. **Update all call sites** in VVC and any VOK code that imports VVC

## Verification

Run `cargo build` and `cargo test` after changes. All must pass.

## Note

VVC is used by JJK. Coordinate with jjk-rcg-compliance pace — run VVC first since JJK imports from VVC.

### jjk-rcg-compliance (₢AAAAY) [complete]

**[260116-1418] complete**

Applied RCG prefixes to all JJK Rust declarations. Added #\![allow(non_camel_case_types)] to lib.rs. Prefixed public types, methods, and constants with file-specific prefixes (jjrg_, jjrf_, etc.). Fixed cross-file references with import aliases.

**[260116-1010] bridled**

JJK RCG Phase 1: Parallel declaration prefixing

## Approach

Launch 7 parallel Sonnet agents, one per source file. Each agent prefixes declarations DEFINED in its file only. No cross-file call site updates. No commits.

## Files and agents

1. jjrc_core.rs - prefix constants/functions with jjrc_
2. jjrf_favor.rs - prefix types/constants with jjrf_
3. jjrg_gallops.rs - prefix types/functions with jjrg_
4. jjrn_notch.rs - prefix types/functions with jjrn_
5. jjrq_query.rs - prefix types/functions with jjrq_
6. jjrs_steeplechase.rs - prefix types/functions with jjrs_
7. jjrx_cli.rs - prefix types/functions with jjrx_

## Per-agent instructions

For assigned file {prefix}_{name}.rs:
1. Add file to context
2. Prefix all pub struct/enum with {prefix}_ (e.g., Gallops -> jjrg_Gallops)
3. Prefix all pub fn with {prefix}_ (e.g., load -> jjrg_load)
4. Prefix all pub const with {PREFIX}_ (e.g., CHARSET -> JJRF_CHARSET)
5. Prefix impl methods with {prefix}_
6. Prefix private/internal items with z{prefix}_
7. Update call sites WITHIN this file only
8. Report: list of old_name -> new_name mappings

## Output

Each agent returns rename manifest. Do NOT commit. Phase 2 handles cross-file updates.

## Reference

RCG guide: Tools/vok/lenses/RCG-RustCodingGuide.md

*Direction:* Launch 7 parallel Sonnet Task agents (single message, 7 Task tool calls). Agent prompts: 1) jjrc_core.rs: Read RCG then file, prefix jjrc_/JJRC_/zjjrc_, internal calls only, return manifest, no commit. 2) jjrf_favor.rs: jjrf_/JJRF_/zjjrf_. 3) jjrg_gallops.rs: jjrg_/JJRG_/zjjrg_. 4) jjrn_notch.rs: jjrn_/JJRN_/zjjrn_. 5) jjrq_query.rs: jjrq_/JJRQ_/zjjrq_. 6) jjrs_steeplechase.rs: jjrs_/JJRS_/zjjrs_. 7) jjrx_cli.rs: jjrx_/JJRX_/zjjrx_. After all complete, collect manifests, proceed to Phase 2.

**[260116-1001] bridled**

JJK RCG Phase 1: Parallel declaration prefixing

## Approach

Launch 7 parallel Sonnet agents, one per source file. Each agent prefixes declarations DEFINED in its file only. No cross-file call site updates. No commits.

## Files and agents

1. jjrc_core.rs - prefix constants/functions with jjrc_
2. jjrf_favor.rs - prefix types/constants with jjrf_
3. jjrg_gallops.rs - prefix types/functions with jjrg_
4. jjrn_notch.rs - prefix types/functions with jjrn_
5. jjrq_query.rs - prefix types/functions with jjrq_
6. jjrs_steeplechase.rs - prefix types/functions with jjrs_
7. jjrx_cli.rs - prefix types/functions with jjrx_

## Per-agent instructions

For assigned file {prefix}_{name}.rs:
1. Add file to context
2. Prefix all pub struct/enum with {prefix}_ (e.g., Gallops -> jjrg_Gallops)
3. Prefix all pub fn with {prefix}_ (e.g., load -> jjrg_load)
4. Prefix all pub const with {PREFIX}_ (e.g., CHARSET -> JJRF_CHARSET)
5. Prefix impl methods with {prefix}_
6. Prefix private/internal items with z{prefix}_
7. Update call sites WITHIN this file only
8. Report: list of old_name -> new_name mappings

## Output

Each agent returns rename manifest. Do NOT commit. Phase 2 handles cross-file updates.

## Reference

RCG guide: Tools/vok/lenses/RCG-RustCodingGuide.md

*Direction:* Launch 7 parallel Sonnet Task agents. Each agent: read RCG guide, read assigned file, prefix all declarations per RCG, update internal call sites only, return rename manifest. Files: jjrc_core, jjrf_favor, jjrg_gallops, jjrn_notch, jjrq_query, jjrs_steeplechase, jjrx_cli. No commits. Collect manifests for Phase 2.

**[260116-1000] rough**

JJK RCG Phase 1: Parallel declaration prefixing

## Approach

Launch 7 parallel Sonnet agents, one per source file. Each agent prefixes declarations DEFINED in its file only. No cross-file call site updates. No commits.

## Files and agents

1. jjrc_core.rs - prefix constants/functions with jjrc_
2. jjrf_favor.rs - prefix types/constants with jjrf_
3. jjrg_gallops.rs - prefix types/functions with jjrg_
4. jjrn_notch.rs - prefix types/functions with jjrn_
5. jjrq_query.rs - prefix types/functions with jjrq_
6. jjrs_steeplechase.rs - prefix types/functions with jjrs_
7. jjrx_cli.rs - prefix types/functions with jjrx_

## Per-agent instructions

For assigned file {prefix}_{name}.rs:
1. Add file to context
2. Prefix all pub struct/enum with {prefix}_ (e.g., Gallops -> jjrg_Gallops)
3. Prefix all pub fn with {prefix}_ (e.g., load -> jjrg_load)
4. Prefix all pub const with {PREFIX}_ (e.g., CHARSET -> JJRF_CHARSET)
5. Prefix impl methods with {prefix}_
6. Prefix private/internal items with z{prefix}_
7. Update call sites WITHIN this file only
8. Report: list of old_name -> new_name mappings

## Output

Each agent returns rename manifest. Do NOT commit. Phase 2 handles cross-file updates.

## Reference

RCG guide: Tools/vok/lenses/RCG-RustCodingGuide.md

**[260116-0944] bridled**

Bring all JJK Rust code into RCG compliance.

## Scope

All files in Tools/jjk/veiled/src/:
- lib.rs (add boilerplate)
- jjrc_core.rs
- jjrf_favor.rs
- jjrg_gallops.rs
- jjrn_notch.rs
- jjrq_query.rs
- jjrs_steeplechase.rs
- jjrx_cli.rs

## Required Changes

1. **Crate boilerplate**: Add `#![allow(non_camel_case_types)]` to lib.rs

2. **Type prefixing**: All pub struct/enum get file prefix
   - `Gallops` → `jjrg_Gallops`
   - `Heat` → `jjrg_Heat`
   - `Pace` → `jjrg_Pace`
   - `Tack` → `jjrg_Tack`
   - `PaceState` → `jjrg_PaceState`
   - `HeatStatus` → `jjrg_HeatStatus`
   - `NominateArgs` → `jjrg_NominateArgs`
   - `NominateResult` → `jjrg_NominateResult`
   - `SlateArgs` → `jjrg_SlateArgs`
   - `SlateResult` → `jjrg_SlateResult`
   - `RailArgs` → `jjrg_RailArgs`
   - `TallyArgs` → `jjrg_TallyArgs`
   - `DraftArgs` → `jjrg_DraftArgs`
   - `DraftResult` → `jjrg_DraftResult`
   - `Firemark` → `jjrf_Firemark`
   - `Coronet` → `jjrf_Coronet`
   - `ChalkMarker` → `jjrn_ChalkMarker`
   - `ReinArgs` → `jjrs_ReinArgs`
   - `SteeplechaseEntry` → `jjrs_SteeplechaseEntry`
   - `MusterArgs` → `jjrq_MusterArgs`
   - `SaddleArgs` → `jjrq_SaddleArgs`
   - `ParadeFormat` → `jjrq_ParadeFormat`
   - `ParadeArgs` → `jjrq_ParadeArgs`
   - `RetireArgs` → `jjrq_RetireArgs`
   - `JjxCommands` → `jjrx_Commands`
   - `NotchArgs` → `jjrx_NotchArgs`
   - `ChalkArgs` → `jjrx_ChalkArgs`

3. **Function prefixing**: All pub fn get file prefix
   - `dispatch` → `jjrx_dispatch`
   - `is_jjk_command` → `jjrx_is_jjk_command`
   - `run_muster` → `jjrq_run_muster`
   - `run_saddle` → `jjrq_run_saddle`
   - `run_parade` → `jjrq_run_parade`
   - `run_retire` → `jjrq_run_retire`
   - `run` (jjrs) → `jjrs_run`
   - `format_notch_prefix` → `jjrn_format_notch_prefix`
   - `format_chalk_message` → `jjrn_format_chalk_message`
   - `validate_chalk_args` → `jjrn_validate_chalk_args`
   - `read_stdin` → `jjrg_read_stdin`
   - `read_stdin_optional` → `jjrg_read_stdin_optional`
   - `default_gallops_path` → `jjrc_default_gallops_path`
   - `timestamp_date` → `jjrc_timestamp_date`
   - `timestamp_full` → `jjrc_timestamp_full`

4. **Constant prefixing**: All pub const get FILE PREFIX (screaming)
   - `DEFAULT_GALLOPS_PATH` → `JJRC_DEFAULT_GALLOPS_PATH`
   - `CHARSET` → `JJRF_CHARSET`
   - `FIREMARK_PREFIX` → `JJRF_FIREMARK_PREFIX`
   - `CORONET_PREFIX` → `JJRF_CORONET_PREFIX`
   - `FIREMARK_MAX` → `JJRF_FIREMARK_MAX`
   - `CORONET_PACE_MAX` → `JJRF_CORONET_PACE_MAX`

5. **Impl method prefixing**: All pub methods on impl blocks get file prefix

6. **Internal functions**: Any non-pub helpers get z prefix (e.g., `fn validate()` → `fn zjjrg_validate()`)

7. **Update lib.rs re-exports** to use new names

8. **Update all call sites** across all files

## Verification

Run `cargo build` and `cargo test` after changes. All must pass.

## Test extraction

NOT in scope for this pace. Tests remain inline. Separate pace for test file extraction.

*Direction:* Execute JJK RCG compliance per RCG guide. Prefix all types/functions/constants per the tack list. Add crate boilerplate. Update all 7 source files. Update lib.rs re-exports. Update call sites. Run cargo build and cargo test to verify. Note: Run vvc-rcg-compliance first since JJK imports VVC.

**[260116-0941] rough**

Bring all JJK Rust code into RCG compliance.

## Scope

All files in Tools/jjk/veiled/src/:
- lib.rs (add boilerplate)
- jjrc_core.rs
- jjrf_favor.rs
- jjrg_gallops.rs
- jjrn_notch.rs
- jjrq_query.rs
- jjrs_steeplechase.rs
- jjrx_cli.rs

## Required Changes

1. **Crate boilerplate**: Add `#![allow(non_camel_case_types)]` to lib.rs

2. **Type prefixing**: All pub struct/enum get file prefix
   - `Gallops` → `jjrg_Gallops`
   - `Heat` → `jjrg_Heat`
   - `Pace` → `jjrg_Pace`
   - `Tack` → `jjrg_Tack`
   - `PaceState` → `jjrg_PaceState`
   - `HeatStatus` → `jjrg_HeatStatus`
   - `NominateArgs` → `jjrg_NominateArgs`
   - `NominateResult` → `jjrg_NominateResult`
   - `SlateArgs` → `jjrg_SlateArgs`
   - `SlateResult` → `jjrg_SlateResult`
   - `RailArgs` → `jjrg_RailArgs`
   - `TallyArgs` → `jjrg_TallyArgs`
   - `DraftArgs` → `jjrg_DraftArgs`
   - `DraftResult` → `jjrg_DraftResult`
   - `Firemark` → `jjrf_Firemark`
   - `Coronet` → `jjrf_Coronet`
   - `ChalkMarker` → `jjrn_ChalkMarker`
   - `ReinArgs` → `jjrs_ReinArgs`
   - `SteeplechaseEntry` → `jjrs_SteeplechaseEntry`
   - `MusterArgs` → `jjrq_MusterArgs`
   - `SaddleArgs` → `jjrq_SaddleArgs`
   - `ParadeFormat` → `jjrq_ParadeFormat`
   - `ParadeArgs` → `jjrq_ParadeArgs`
   - `RetireArgs` → `jjrq_RetireArgs`
   - `JjxCommands` → `jjrx_Commands`
   - `NotchArgs` → `jjrx_NotchArgs`
   - `ChalkArgs` → `jjrx_ChalkArgs`

3. **Function prefixing**: All pub fn get file prefix
   - `dispatch` → `jjrx_dispatch`
   - `is_jjk_command` → `jjrx_is_jjk_command`
   - `run_muster` → `jjrq_run_muster`
   - `run_saddle` → `jjrq_run_saddle`
   - `run_parade` → `jjrq_run_parade`
   - `run_retire` → `jjrq_run_retire`
   - `run` (jjrs) → `jjrs_run`
   - `format_notch_prefix` → `jjrn_format_notch_prefix`
   - `format_chalk_message` → `jjrn_format_chalk_message`
   - `validate_chalk_args` → `jjrn_validate_chalk_args`
   - `read_stdin` → `jjrg_read_stdin`
   - `read_stdin_optional` → `jjrg_read_stdin_optional`
   - `default_gallops_path` → `jjrc_default_gallops_path`
   - `timestamp_date` → `jjrc_timestamp_date`
   - `timestamp_full` → `jjrc_timestamp_full`

4. **Constant prefixing**: All pub const get FILE PREFIX (screaming)
   - `DEFAULT_GALLOPS_PATH` → `JJRC_DEFAULT_GALLOPS_PATH`
   - `CHARSET` → `JJRF_CHARSET`
   - `FIREMARK_PREFIX` → `JJRF_FIREMARK_PREFIX`
   - `CORONET_PREFIX` → `JJRF_CORONET_PREFIX`
   - `FIREMARK_MAX` → `JJRF_FIREMARK_MAX`
   - `CORONET_PACE_MAX` → `JJRF_CORONET_PACE_MAX`

5. **Impl method prefixing**: All pub methods on impl blocks get file prefix

6. **Internal functions**: Any non-pub helpers get z prefix (e.g., `fn validate()` → `fn zjjrg_validate()`)

7. **Update lib.rs re-exports** to use new names

8. **Update all call sites** across all files

## Verification

Run `cargo build` and `cargo test` after changes. All must pass.

## Test extraction

NOT in scope for this pace. Tests remain inline. Separate pace for test file extraction.

### fix-jjx-cli-command-name (₢AAAAX) [complete]

**[260116-0849] complete**

Fixed clap argument parsing in jjrx_cli.rs dispatch() by prepending synthetic 'jjx' binary name. Commands like 'vvx jjx_saddle AA' now parse correctly instead of treating the firemark as a subcommand.

**[260116-0826] rough**

Fix JJK CLI command group name: jjrx_cli.rs declares #[command(name = "jjx")] but VVX integration causes it to appear as jjx_nominate. Invocation should be 'vvx jjx <subcommand>' not 'vvx jjx_nominate <subcommand>'. Check external_subcommand registration in VVX main.

### jjd-rail-move-concepts (₢AAAAT) [complete]

**[260115-1554] complete**

Update JJD-GallopsData.adoc with rail move semantics concepts.

## Completed

1. Added `jjda_last` argument - move to end, rail-only
2. Added `jjda_move` argument - triggers move mode
3. Expanded `jjdo_rail` with dual-mode documentation (order mode + move mode)
4. Added validation errors table with 6 error conditions
5. Fixed asymmetric mutual exclusion (jjda_first no longer mentions jjda_last)
6. Clarified jjda_last is rail-specific

## Scope

JJD spec only. Implementation is separate pace (rail-move-semantics ₢AAAAK).

**[260115-1548] rough**

Update JJD-GallopsData.adoc with rail move semantics concepts.

## Purpose

Document the --move/--before/--after/--first/--last syntax in JJD before implementing in Rust.

## Sections to Update

1. **jjdo_rail** — Add move syntax documentation alongside existing full-order syntax
2. **Validation rules table** — Document error conditions
3. **Output format** — Document that move operations output in order format

## Scope

JJD spec only. Implementation is separate pace (rail-move-semantics ₢AAAAK).

### bud-cli-args-quoting-fix (₢AAAAP) [complete]

**[260115-1443] complete**

Fix argument quoting bug in bud_dispatch.sh that breaks multi-word arguments.

## The Bug

Line 177: `BUD_CLI_ARGS="$*"` — joins args into single string, loses boundaries
Lines 281/287/293: `$BUD_CLI_ARGS` unquoted — word splits on spaces

## Impact

Any multi-word argument through tabtarget dispatch gets broken:
- `--direction "has spaces"` becomes 4 separate args
- Affects ALL tabtargets, not just vvx

## The Fix

1. Store as array: `BUD_CLI_ARGS=("$@")`
2. Expand as array: `"${BUD_CLI_ARGS[@]}"`

## Files

- Tools/buk/bud_dispatch.sh

## REVIEW REMINDER

User did not expect this bug. Review before executing to confirm fix approach is correct and complete. May have broader implications for BUK infrastructure.

**[260115-1441] bridled**

Fix argument quoting bug in bud_dispatch.sh that breaks multi-word arguments.

## The Bug

Line 177: `BUD_CLI_ARGS="$*"` — joins args into single string, loses boundaries
Lines 281/287/293: `$BUD_CLI_ARGS` unquoted — word splits on spaces

## Impact

Any multi-word argument through tabtarget dispatch gets broken:
- `--direction "has spaces"` becomes 4 separate args
- Affects ALL tabtargets, not just vvx

## The Fix

1. Store as array: `BUD_CLI_ARGS=("$@")`
2. Expand as array: `"${BUD_CLI_ARGS[@]}"`

## Files

- Tools/buk/bud_dispatch.sh

## REVIEW REMINDER

User did not expect this bug. Review before executing to confirm fix approach is correct and complete. May have broader implications for BUK infrastructure.

*Direction:* Agent: haiku - Fix bud_dispatch.sh quoting

**[260115-1429] rough**

Fix argument quoting bug in bud_dispatch.sh that breaks multi-word arguments.

## The Bug

Line 177: `BUD_CLI_ARGS="$*"` — joins args into single string, loses boundaries
Lines 281/287/293: `$BUD_CLI_ARGS` unquoted — word splits on spaces

## Impact

Any multi-word argument through tabtarget dispatch gets broken:
- `--direction "has spaces"` becomes 4 separate args
- Affects ALL tabtargets, not just vvx

## The Fix

1. Store as array: `BUD_CLI_ARGS=("$@")`
2. Expand as array: `"${BUD_CLI_ARGS[@]}"`

## Files

- Tools/buk/bud_dispatch.sh

## REVIEW REMINDER

User did not expect this bug. Review before executing to confirm fix approach is correct and complete. May have broader implications for BUK infrastructure.

### bul-launcher-refactor (₢AAAAS) [complete]

**[260115-1518] complete**

Moved launcher infrastructure to Tools/buk/bul_launcher.sh. BURC exports consolidated into zburc_kindle() for module cohesion. All 10 launcher stubs updated and verified.

**[260115-1509] bridled**

Move launcher infrastructure to Tools/buk/ and consolidate exports.

## Changes

1. Move `.buk/launcher_common.sh` → `Tools/buk/bul_launcher.sh`
2. Move BURC variable exports into `zburc_kindle()`
3. Move BURS variable exports into `zburs_kindle()` 
4. Update `.buk/launcher.*.sh` stubs to source from new location
5. Verify tabtargets still function

## Rationale

- `bul_` prefix follows BUK naming (buc, bud, but, buv, buw → bul)
- Shared logic belongs in Tools/buk/, not hidden .buk/
- Exports belong in kindle functions that own the variables
- Discovered during arg quoting fix — natural time to address

## Files

- .buk/launcher_common.sh (delete after move)
- Tools/buk/bul_launcher.sh (new)
- Tools/buk/burc_regime.sh (add exports to kindle)
- Tools/buk/burs_regime.sh (add exports to kindle)
- .buk/launcher.*.sh (update source paths)

*Direction:* Agent: sonnet

## File Operations
1. Create Tools/buk/bul_launcher.sh from .buk/launcher_common.sh
2. Update .buk/launcher.*.sh stubs to use relative path: source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
3. Delete .buk/launcher_common.sh after verification

## Export Decisions
- BURC_TABTARGET_DIR, BURC_TOOLS_DIR → move to zburc_kindle() in burc_regime.sh
- BUD_REGIME_FILE, BUD_STATION_FILE → keep in bul_launcher.sh (dispatch context, set before kindle)

## Verification
- Run tt/vow-r.RunVVX.sh --help to verify tabtarget chain works
- Run one BUK tabtarget (e.g., tt/buw-tt-ll.ListLaunchers.sh) to verify full path

## BCG Notes
- bul_launcher.sh is NOT a full BCG module (no kindle/sentinel) - it's bootstrap infrastructure
- Add standard copyright header and guard against multiple inclusion

**[260115-1454] bridled**

Move launcher infrastructure to Tools/buk/ and consolidate exports.

## Changes

1. Move `.buk/launcher_common.sh` → `Tools/buk/bul_launcher.sh`
2. Move BURC variable exports into `zburc_kindle()`
3. Move BURS variable exports into `zburs_kindle()` 
4. Update `.buk/launcher.*.sh` stubs to source from new location
5. Verify tabtargets still function

## Rationale

- `bul_` prefix follows BUK naming (buc, bud, but, buv, buw → bul)
- Shared logic belongs in Tools/buk/, not hidden .buk/
- Exports belong in kindle functions that own the variables
- Discovered during arg quoting fix — natural time to address

## Files

- .buk/launcher_common.sh (delete after move)
- Tools/buk/bul_launcher.sh (new)
- Tools/buk/burc_regime.sh (add exports to kindle)
- Tools/buk/burs_regime.sh (add exports to kindle)
- .buk/launcher.*.sh (update source paths)

*Direction:* Agent: sonnet

## File Operations
1. Create Tools/buk/bul_launcher.sh from .buk/launcher_common.sh
2. Update .buk/launcher.*.sh stubs to use relative path: source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
3. Delete .buk/launcher_common.sh after verification

## Export Decisions
- BURC_TABTARGET_DIR, BURC_TOOLS_DIR → move to zburc_kindle() in burc_regime.sh
- BUD_REGIME_FILE, BUD_STATION_FILE → keep in bul_launcher.sh (dispatch context, set before kindle)

## Verification
- Run tt/vow-r.RunVVX.sh --help to verify tabtarget chain works
- Run one BUK tabtarget (e.g., tt/buw-tt-ll.ListLaunchers.sh) to verify full path

## BCG Notes
- bul_launcher.sh is NOT a full BCG module (no kindle/sentinel) - it's bootstrap infrastructure
- Add standard copyright header and guard against multiple inclusion

**[260115-1449] rough**

Move launcher infrastructure to Tools/buk/ and consolidate exports.

## Changes

1. Move `.buk/launcher_common.sh` → `Tools/buk/bul_launcher.sh`
2. Move BURC variable exports into `zburc_kindle()`
3. Move BURS variable exports into `zburs_kindle()` 
4. Update `.buk/launcher.*.sh` stubs to source from new location
5. Verify tabtargets still function

## Rationale

- `bul_` prefix follows BUK naming (buc, bud, but, buv, buw → bul)
- Shared logic belongs in Tools/buk/, not hidden .buk/
- Exports belong in kindle functions that own the variables
- Discovered during arg quoting fix — natural time to address

## Files

- .buk/launcher_common.sh (delete after move)
- Tools/buk/bul_launcher.sh (new)
- Tools/buk/burc_regime.sh (add exports to kindle)
- Tools/buk/burs_regime.sh (add exports to kindle)
- .buk/launcher.*.sh (update source paths)

### jjd-slate-position-concepts (₢AAAAR) [complete]

**[260115-1523] complete**

Added --before/--after/--first positioning flags to JJD-GallopsData.adoc. Updated jjx_slate operation with positioning arguments, mutual exclusivity rules, and insertion behavior.

**[260115-1443] bridled**

Update JJD-GallopsData.adoc with slate positioning concepts.

## Purpose

Document --before/--after/--first positioning flags for jjx_slate before implementation.

## Concepts to Add

**Slate Positioning** — New pace can be inserted at specific position rather than always appending.

- --before <coronet>: Insert before specified pace
- --after <coronet>: Insert after specified pace  
- --first: Insert at beginning of heat
- Default (no flag): Append to end (backwards compatible)

## Relationship to Rail

Rail reorders existing paces. Slate positioning inserts new pace at desired location. Different operations, same position vocabulary.

## Sections to Update

1. Operations section — Add positioning flags to jjx_slate entry
2. Glossary — If needed for new terms

## Reference

Implementation pace: ₢AAAAQ slate-position-flags

*Direction:* Agent: haiku - Update JJD-GallopsData.adoc: add slate positioning concepts to Operations section, add glossary if needed

**[260115-1432] rough**

Update JJD-GallopsData.adoc with slate positioning concepts.

## Purpose

Document --before/--after/--first positioning flags for jjx_slate before implementation.

## Concepts to Add

**Slate Positioning** — New pace can be inserted at specific position rather than always appending.

- --before <coronet>: Insert before specified pace
- --after <coronet>: Insert after specified pace  
- --first: Insert at beginning of heat
- Default (no flag): Append to end (backwards compatible)

## Relationship to Rail

Rail reorders existing paces. Slate positioning inserts new pace at desired location. Different operations, same position vocabulary.

## Sections to Update

1. Operations section — Add positioning flags to jjx_slate entry
2. Glossary — If needed for new terms

## Reference

Implementation pace: ₢AAAAQ slate-position-flags

### slate-position-flags (₢AAAAQ) [complete]

**[260115-1531] complete**

Implemented --before, --after, and --first positioning flags for jjx_slate per JJD-GallopsData.adoc spec. Added Clap mutual exclusivity, insertion logic, and 7 new unit tests. All 110 tests pass.

**[260115-1526] bridled**

Implement jjx_slate positioning flags per JJD-GallopsData.adoc.

## Authoritative Spec

Tools/jjk/JJD-GallopsData.adoc section `jjdo_slate` defines:
- Arguments: --before, --after, --first (mutually exclusive)
- Validation: target coronet must exist in heat
- Insertion behavior: prepend, before, after, or append (default)

## Implementation

1. Add Clap args with mutual exclusivity (same pattern as rail-move-semantics)
2. Validate target coronet exists when --before/--after provided
3. Update order array insertion logic per JJD behavior spec

## Files

Tools/vok/src (wherever jjx_slate is implemented)

## NOT in scope

- jjx_reslate positioning (position is rail's job)

*Direction:* Agent: sonnet - Implement per JJD-GallopsData.adoc jjdo_slate spec. Add Clap args for --before, --after, --first with mutual exclusivity. Update insertion logic in behavior.

**[260115-1444] bridled**

Add --before/--after positioning flags to jjx_slate.

## Current Behavior

`jjx_slate` always appends new pace to end of heat. Must follow with `jjx_rail` to reposition.

## Proposed Enhancement

```bash
vvx jjx_slate ₣AA --silks 'new-pace' --before ₢AAAAJ <<< "tack"
vvx jjx_slate ₣AA --silks 'new-pace' --after ₢AAAAK <<< "tack"
vvx jjx_slate ₣AA --silks 'new-pace' --first <<< "tack"
```

Without position flag: append to end (current behavior, backwards compatible).

## Implementation

Same Clap pattern as rail-move-semantics (₢AAAAK). Can share validation logic.

## Scope

- jjx_slate: YES — add position flags
- jjx_reslate: NO — reslate updates tack, doesn't change position. Position is jjx_rail's job.

## Files

Tools/vok/src (wherever jjx_slate is implemented)

*Direction:* Agent: sonnet - Add --before/--after/--first flags to jjx_slate in Tools/vok/src, follow Clap pattern from rail-move-semantics

**[260115-1431] rough**

Add --before/--after positioning flags to jjx_slate.

## Current Behavior

`jjx_slate` always appends new pace to end of heat. Must follow with `jjx_rail` to reposition.

## Proposed Enhancement

```bash
vvx jjx_slate ₣AA --silks 'new-pace' --before ₢AAAAJ <<< "tack"
vvx jjx_slate ₣AA --silks 'new-pace' --after ₢AAAAK <<< "tack"
vvx jjx_slate ₣AA --silks 'new-pace' --first <<< "tack"
```

Without position flag: append to end (current behavior, backwards compatible).

## Implementation

Same Clap pattern as rail-move-semantics (₢AAAAK). Can share validation logic.

## Scope

- jjx_slate: YES — add position flags
- jjx_reslate: NO — reslate updates tack, doesn't change position. Position is jjx_rail's job.

## Files

Tools/vok/src (wherever jjx_slate is implemented)

### vvw-workbench-tabtarget (₢AAAAC) [abandoned]

**[260115-1259] abandoned**

Superseded by consolidated slash-command-modernize pace. Rationale: avoid multiple touches across files in multiple paces; one pace handles vocabulary coherence + passthrough + new command names together.

**[260114-1108] rough**

Create VVW workbench with passthrough tabtarget for consolidated vvx permissions.

Files to create:
- Tools/vvk/vvw_workbench.sh - routes vvw-* colophons
- Tools/vvk/vvb_bash.sh - bash utilities (if needed)
- Tools/vvk/vvb_cli.sh - CLI routing

Tabtarget to create:
- tt/vvw-r.RunVVX.sh - passthrough to ./Tools/vvk/bin/vvx

IMPORTANT: Use buw-tt-cl (CreateLauncher) and buw-tt-cbn (CreateTabTargetBatchNolog) operations to create launcher and tabtarget infrastructure. Do NOT hand-write these files.

Update CLAUDE.md BUK Concepts section to add instructions directing Claude to use buw-tt-* operations when creating new tabtargets/launchers rather than hand-writing them.

ALSO: Revise ALL JJK slash commands to use the new ./tt/vvw-r.RunVVX.sh passthrough instead of bare 'vvx' calls. This consolidates permissions - one grant covers all vvx operations.

NOTE: By this point, vvx-push-rename pace will have renamed 'commit' to 'vvx_commit'. Use the NEW names in slash commands:
- vvx vvx_commit (not vvx commit)
- vvx vvx_push (new)

Files to update:
- .claude/commands/jjc-heat-saddle.md
- .claude/commands/jjc-heat-parade.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-heat-rail.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md (update to use vvx_commit)
- Tools/jjk/commands/*.md (source copies)

Reference existing patterns:
- Tools/vok/vow_workbench.sh for workbench structure
- tt/vow-r.RunVVX.sh for passthrough pattern (but point to vvk/bin/vvx not vok/target)

**[260114-1053] rough**

Create VVW workbench with passthrough tabtarget for consolidated vvx permissions.

Files to create:
- Tools/vvk/vvw_workbench.sh - routes vvw-* colophons
- Tools/vvk/vvb_bash.sh - bash utilities (if needed)
- Tools/vvk/vvb_cli.sh - CLI routing

Tabtarget to create:
- tt/vvw-r.RunVVX.sh - passthrough to ./Tools/vvk/bin/vvx

IMPORTANT: Use buw-tt-cl (CreateLauncher) and buw-tt-cbn (CreateTabTargetBatchNolog) operations to create launcher and tabtarget infrastructure. Do NOT hand-write these files.

Update CLAUDE.md BUK Concepts section to add instructions directing Claude to use buw-tt-* operations when creating new tabtargets/launchers rather than hand-writing them.

ALSO: Revise ALL JJK slash commands to use the new ./tt/vvw-r.RunVVX.sh passthrough instead of bare 'vvx' calls. This consolidates permissions - one grant covers all vvx operations. Files to update:
- .claude/commands/jjc-heat-saddle.md
- .claude/commands/jjc-heat-parade.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-heat-rail.md (NEW)
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md
- Tools/jjk/commands/*.md (source copies)

Reference existing patterns:
- Tools/vok/vow_workbench.sh for workbench structure
- tt/vow-r.RunVVX.sh for passthrough pattern (but point to vvk/bin/vvx not vok/target)

**[260114-1049] rough**

Create VVW workbench with passthrough tabtarget for consolidated vvx permissions.

Files to create:
- Tools/vvk/vvw_workbench.sh - routes vvw-* colophons
- Tools/vvk/vvb_bash.sh - bash utilities (if needed)
- Tools/vvk/vvb_cli.sh - CLI routing

Tabtarget to create:
- tt/vvw-r.RunVVX.sh - passthrough to ./Tools/vvk/bin/vvx

IMPORTANT: Use buw-tt-cl (CreateLauncher) and buw-tt-cbn (CreateTabTargetBatchNolog) operations to create launcher and tabtarget infrastructure. Do NOT hand-write these files.

Update CLAUDE.md BUK Concepts section to add instructions directing Claude to use buw-tt-* operations when creating new tabtargets/launchers rather than hand-writing them.

ALSO: Revise ALL JJK slash commands to use the new ./tt/vvw-r.RunVVX.sh passthrough instead of bare 'vvx' calls. This consolidates permissions - one grant covers all vvx operations. Files to update:
- .claude/commands/jjc-heat-saddle.md
- .claude/commands/jjc-heat-parade.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md
- Tools/jjk/commands/*.md (source copies)

Reference existing patterns:
- Tools/vok/vow_workbench.sh for workbench structure
- tt/vow-r.RunVVX.sh for passthrough pattern (but point to vvk/bin/vvx not vok/target)

**[260114-1046] rough**

Create VVW workbench with passthrough tabtarget for consolidated vvx permissions.

Files to create:
- Tools/vvk/vvw_workbench.sh - routes vvw-* colophons
- Tools/vvk/vvb_bash.sh - bash utilities (if needed)
- Tools/vvk/vvb_cli.sh - CLI routing

Tabtarget to create:
- tt/vvw-r.RunVVX.sh - passthrough to ./Tools/vvk/bin/vvx

IMPORTANT: Use buw-tt-cl (CreateLauncher) and buw-tt-cbn (CreateTabTargetBatchNolog) operations to create launcher and tabtarget infrastructure. Do NOT hand-write these files.

Also update CLAUDE.md BUK Concepts section to add instructions directing Claude to use buw-tt-* operations when creating new tabtargets/launchers rather than hand-writing them.

Reference existing patterns:
- Tools/vok/vow_workbench.sh for workbench structure
- tt/vow-r.RunVVX.sh for passthrough pattern (but point to vvk/bin/vvx not vok/target)

### install-arch-decision (₢AAAAI) [complete]

**[260115-1249] complete**

Resolved via paddock discussion 2026-01-15. Decision: archive-based asset model with plain text kit assets in kits/ directory, lean binaries with install logic only.

**[260115-1247] rough**

Document resolved install architecture decision.

This pace was resolved via paddock discussion (see Steeplechase 2026-01-15).

**Decision**: Archive-based asset model with plain text kit assets.

Key points captured in paddock Architecture section:
- Archive is the distribution unit (not self-contained binaries)
- Kit assets are plain text in `kits/` directory
- Lean binaries contain install logic only, no embedded content
- Any platform binary can perform full install

No implementation work needed — decision is documented. Mark complete when paddock is reviewed and confirmed accurate.

**[260114-1102] rough**

Resolve install architecture: static copy vs config-aware deployment.

Key question: Is install just 'copy files to fixed paths' or does it need to adapt to target repo configuration (burc.env)?

What might vary per target repo:
- Paths in slash commands (where is vvx binary?)
- CLAUDE.md structure/location
- Kit-specific settings from burc.env
- Tabtarget launcher paths

Options to evaluate:
1. Static install - target repos must conform to expected structure
2. Config-aware install (arcanum pattern in Rust) - reads burc.env, adapts content
3. Hybrid - most content static, slash commands get path templating

This decision affects:
- kit-asset-registry design (static content vs templates?)
- vvx-install-impl (copy vs transform?)
- Whether 'arcanums eliminated' holds or needs revision

Deliverable: Clear decision documented in paddock, possibly reflected in MCM concept model.

### vvx-push-rename (₢AAAAJ) [complete]

**[260115-1535] complete**

Renamed commit to vvx_commit, added vvx_push with same lock pattern (refs/vvg/locks/vvx) to prevent concurrent commit/push operations.

**[260115-1424] bridled**

Confirmed scope.

*Direction:* Rename-commit-to-vvx_commit-and-add-vvx_push-following-lock-pattern

**[260115-1412] rough**

Add vvx_push operation and rename commit to vvx_commit.

## Scope: Rust Only

This pace modifies Rust code. Slash command updates are handled by slash-command-modernize.

## Changes

1. **Rename subcommand**: `commit` → `vvx_commit`
2. **Add subcommand**: `vvx_push`

## vvx_push Behavior (Simple)

- Acquire lock (refs/vvg/locks/vvx)
- Run `git push` to origin/current-branch
- Release lock
- Report success/failure

No configuration flags. Simple push to origin. Add flexibility later if needed.

## Naming Rationale

vvx subcommands follow prefix discipline:
- `jjx_*` for JJ operations
- `vvx_*` for VVK core operations
- `guard` stays as-is (standalone utility)

## Files to Modify

- Tools/vok/src/vorm_main.rs (subcommand dispatch)
- Tools/vok/src/vorc_commit.rs (keep filename, command becomes vvx_commit)
- Add new file for vvx_push (e.g., vorc_push.rs or similar)

## NOT in Scope

Slash command updates — handled by slash-command-modernize (₢AAAAL).

**[260114-1107] rough**

Add vvx_push operation and rename commit to vvx_commit for naming clarity.

Changes:
1. Rename 'vvx commit' to 'vvx vvx_commit' (matches jjx_* naming pattern)
2. Add 'vvx vvx_push' operation - guarded push with lock

vvx_push responsibilities:
- Acquire lock (refs/vvg/locks/vvx)
- Run git push (with configurable remote/branch?)
- Release lock

Naming rationale: vvx subcommands should follow prefix discipline.
- jjx_* for JJ operations (jjx_muster, jjx_saddle, etc.)
- vvx_* for VVK core operations (vvx_commit, vvx_push)
- guard stays as-is (it's a standalone utility)

Files to update:
- Tools/vok/src/vorm_main.rs (subcommand dispatch)
- Tools/vok/src/vorc_commit.rs (or rename file?)
- All slash commands referencing 'vvx commit'
- vvc-commit.md slash command

### jjx-parade-variants (₢AAAAM) [complete]

**[260115-1545] complete**

Implemented --format flag with 4 modes (overview, order, detail, full) replacing JSON output with human-readable text. Updated JJD spec.

**[260115-1413] rough**

Add output format modes to jjx_parade. Text output only — no JSON.

## Design Philosophy

Slash commands tell Claude what it needs to know without leaking internal structure. All parade output is human-readable text, formatted for purpose.

## Flag

`--format <mode>` where mode is one of:

| Mode | Purpose | Output |
|------|---------|--------|
| `overview` | Quick status | One line per pace: `[state] silks (₢coronet)` |
| `order` | Dependency check | Numbered: `N. [state] silks (₢coronet)` |
| `detail` | Inspect one pace | Full tack text (requires --pace) |
| `full` | Planning context | Paddock + all paces with tack text |

## Default Behavior

Default to `full` if --format not specified. No JSON output mode.

## --pace Flag

Required with `--format detail`. Error without it: "--format detail requires --pace <coronet>"

## Output Examples

`--format overview`:
```
[abandoned] vvw-workbench-tabtarget (₢AAAAC)
[complete] install-arch-decision (₢AAAAI)
[rough] vvx-push-rename (₢AAAAJ)
```

`--format order`:
```
1. [abandoned] vvw-workbench-tabtarget (₢AAAAC)
2. [complete] install-arch-decision (₢AAAAI)
3. [rough] vvx-push-rename (₢AAAAJ)
```

`--format detail --pace ₢AAAAJ`:
```
Pace: vvx-push-rename (₢AAAAJ)
State: rough
Heat: ₣AA

Add vvx_push operation and rename commit to vvx_commit...
[full tack text]
```

Note: Always include sigils (₣, ₢) in output.

## Implementation

Rust file: Tools/vok/src (wherever jjx_parade is implemented)
Add Clap enum for format modes, match on mode to produce text output.

**[260115-1312] rough**

Add output format modes to jjx_parade for purpose-specific views.

## New Flag

`--format <mode>` where mode is one of:

| Mode | Purpose | Output |
|------|---------|--------|
| `overview` | Quick status | One line per pace: `[state] silks (coronet)` |
| `order` | Dependency check | Numbered list: `N. [state] silks (coronet)` |
| `detail` | Inspect one pace | Full tack text (requires `--pace <coronet>`) |
| `full` | Planning context | Paddock content + all paces with tack text |

## Output Format

Text output, not JSON. Each mode produces human-readable (and Claude-readable) text that requires no parsing.

Example `--format overview`:
```
[complete] install-arch-decision (₢AAAAI)
[rough] vok-concept-model (₢AAAAD)
[rough] rcg-establish (₢AAAAB)
...
```

Example `--format order`:
```
1. [complete] install-arch-decision (₢AAAAI)
2. [rough] vok-concept-model (₢AAAAD)
3. [rough] rcg-establish (₢AAAAB)
...
```

Example `--format detail --pace ₢AAAAD`:
```
Pace: vok-concept-model (₢AAAAD)
State: rough

Create MCM-style concept model for VOK release/install system...
[full tack text]
```

## Default Behavior

If `--format` not specified, default to current JSON behavior for backwards compatibility (or change default to `full`?).

## Implementation

Rust file: Tools/vok/src (wherever jjx_parade is implemented)

Add Clap enum for format modes, match on mode to produce appropriate output.

## Why This Matters

Slash commands will wrap these modes. Claude picks the right slash command by name/description, never sees raw JSON, never needs to parse output.

### rail-move-semantics (₢AAAAK) [complete]

**[260115-1603] complete**

Implemented rail move semantics: --move/--before/--after/--first/--last flags, mode detection, validation per JJD spec, 7 tests, updated /jjc-heat-rail docs.

**[260115-1413] rough**

Add move semantics to jjx_rail for easier pace reordering.

## Current Syntax (Retained)

List ALL coronets in new order:
```bash
vvx jjx_rail ₣AA ₢AAAAI ₢AAAAD ₢AAAAB ...
```

## New Syntax: Relative Move

```bash
vvx jjx_rail ₣AA --move ₢AAAAJ --before ₢AAAAC
vvx jjx_rail ₣AA --move ₢AAAAJ --after ₢AAAAB
vvx jjx_rail ₣AA --move ₢AAAAJ --first
vvx jjx_rail ₣AA --move ₢AAAAJ --last
```

## Validation Rules (Strict)

| Condition | Result |
|-----------|--------|
| `--move` without position flag | Error: "--move requires --before, --after, --first, or --last" |
| `--before` AND `--after` | Error: "Cannot specify both --before and --after" |
| `--move X --before X` | Error: "Cannot move pace before itself" |
| `--move X --after X` | Error: "Cannot move pace after itself" |
| Move to current position | No-op, success (already in position) |
| Unknown coronet | Error: "Pace not found: ₢XXXXX" |

## Implementation

- Add --move, --before, --after, --first, --last flags to Clap args
- If --move provided, compute new order from current + operation
- Validate as above
- Write new order to gallops
- Output new order (--format order style)

Rust file: Tools/vok/src (wherever jjx_rail is implemented)

## Slash Command

Update /jjc-heat-rail to document both syntaxes.

**[260114-1121] rough**

Add move semantics to jjx_rail for easier pace reordering.

Current: must list ALL coronets in new order
  vvx jjx_rail AA ₢AAAAI ₢AAAAD ₢AAAAB ₢AAAAJ ₢AAAAC ...

Proposed: relative move operations
  vvx jjx_rail AA --move ₢AAAAJ --before ₢AAAAC
  vvx jjx_rail AA --move ₢AAAAJ --after ₢AAAAB
  vvx jjx_rail AA --move ₢AAAAJ --first
  vvx jjx_rail AA --move ₢AAAAJ --last

Implementation:
- Add --move, --before, --after, --first, --last flags to Clap args
- If --move provided, compute new order from current + operation
- Validate result same as current validation
- Existing positional coronet list still works (backwards compatible)

Update /jjc-heat-rail slash command to document both syntaxes.

Rust file: Tools/vok/src (wherever jjx_rail is implemented - check JJK veiled)

### gallops-deterministic-serial (₢AAAAV) [complete]

**[260115-1622] complete**

JJD spec updated with Deterministic Serialization requirement. Rust changed HashMap to BTreeMap for heats and paces. Verified minimal diffs on subsequent rail operations.

**[260115-1612] rough**

Deterministic gallops serialization for minimal diffs.

## JJD Update
Add assertion to JJD-GallopsData.adoc: "Implementations MUST serialize paces keys in deterministic order to minimize diff churn."

## Rust Fix
Change paces storage from HashMap to BTreeMap in gallops structs. BTreeMap iterates in sorted key order, giving deterministic JSON output.

Files:
- Tools/jjk/JJD-GallopsData.adoc
- Tools/vok/src/jjx_*.rs (whichever defines Heat/Gallops structs)

### vvx-tabtarget (₢AAAAU) [complete]

**[260115-1732] complete**

BCG-compliant VVK tabtarget: vvb_bash.sh (platform detection), vvb_cli.sh, vvw_workbench.sh, launcher, tt/vvx-r.RunVVX.sh. Build updated for platform-specific install with codesign.

**[260115-1607] rough**

Create tabtarget for vvx invocation that can be distributed to target repos. Should be simple launcher that finds and runs the correct platform binary from Tools/vvk/bin/.

### slash-command-modernize (₢AAAAL) [complete]

**[260115-1746] complete**

Modernized 16 slash commands: 4 parade variants, tabtarget passthrough, vvx_commit naming, sigil conventions, Available Operations sections, guarded auto-commit for modifying commands.

**[260115-1607] rough**

Modernize all JJK/VVK slash commands in one pass.

Six concerns consolidated:

## 1. Vocabulary Coherence
Add "Available Operations" section to each slash command. Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants
Create parade slash commands wrapping --format modes:
- /jjc-parade-overview
- /jjc-parade-order
- /jjc-parade-detail
- /jjc-parade-full (rename from /jjc-heat-parade)

## 3. Passthrough Adoption
All vvx calls use the tabtarget from vvx-tabtarget pace (₢AAAAU).

## 4. New Command Names
Use vvx_commit, vvx_push from vvx-push-rename pace.

## 5. Sigil Convention
All examples use sigils: ₣AA for firemarks, ₢AAAAC for coronets.

## 6. Blocking Guarded Auto-Commit
Commands that modify gallops auto-commit using guarded infrastructure.

**Commands with auto-commit:**
- /jjc-pace-slate → "Slate: {silks} in ₣{heat}"
- /jjc-pace-reslate → "Reslate: {silks}"
- /jjc-pace-wrap → "Wrap: {silks}"
- /jjc-heat-rail → "Rail: reorder ₣{heat}"
- /jjc-heat-chalk → "Chalk: {marker} in ₣{heat}"

## Dependencies
- vvx-tabtarget (₢AAAAU) — must complete first
- vvx-push-rename (₢AAAAJ)
- jjx-parade-variants (₢AAAAM)
- rail-move-semantics (₢AAAAK)

**[260115-1414] rough**

Modernize all JJK/VVK slash commands in one pass.

Six concerns consolidated:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command. Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants

Create parade slash commands wrapping --format modes:
- `/jjc-parade-overview`
- `/jjc-parade-order`
- `/jjc-parade-detail`
- `/jjc-parade-full` (rename from /jjc-heat-parade)

## 3. Passthrough Adoption

All vvx calls use `./tt/vow-r.RunVVX.sh <subcommand> [args]`.

## 4. New Command Names

Use `vvx_commit`, `vvx_push` from vvx-push-rename pace.

## 5. Sigil Convention

All examples use sigils: `₣AA` for firemarks, `₢AAAAC` for coronets.

## 6. Blocking Guarded Auto-Commit

Commands that modify gallops auto-commit using guarded infrastructure.

**Implementation:** `./tt/vow-r.RunVVX.sh vvx_commit --message "..."`

**Failure handling:** Report error AND show operation result. The gallops modification succeeded; commit failure is separate. User can retry commit manually.

**Commands with auto-commit:**
- `/jjc-pace-slate` → "Slate: {silks} in ₣{heat}"
- `/jjc-pace-reslate` → "Reslate: {silks}"
- `/jjc-pace-wrap` → "Wrap: {silks}"
- `/jjc-heat-rail` → "Rail: reorder ₣{heat}"
- `/jjc-heat-chalk` → "Chalk: {marker} in ₣{heat}"

Note: `/jjc-heat-restring` is created by jjx-draft pace (₢AAAAN), not this pace.

## Files to Create

- .claude/commands/jjc-parade-overview.md
- .claude/commands/jjc-parade-order.md
- .claude/commands/jjc-parade-detail.md
- .claude/commands/jjc-parade-full.md

## Files to Update

All existing JJK/VVK slash commands in .claude/commands/ and Tools/jjk/commands/.

## Dependencies

- vvx-push-rename (₢AAAAJ)
- jjx-parade-variants (₢AAAAM)
- rail-move-semantics (₢AAAAK)

**[260115-1400] rough**

Modernize all JJK/VVK slash commands in one pass.

Six concerns consolidated to minimize file churn:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command showing sibling commands. Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants

Create purpose-specific parade commands:
- `/jjc-parade-overview` — silks, state, one-line per pace
- `/jjc-parade-order` — sequence with states
- `/jjc-parade-detail <pace>` — full tack for one pace
- `/jjc-parade-full` — paddock + all paces

Rename `/jjc-heat-parade` to `/jjc-parade-full` (terminal exclusivity).

## 3. Passthrough Adoption

Update all vvx calls to use `./tt/vow-r.RunVVX.sh <subcommand> [args]`.

## 4. New Command Names

Use names from vvx-push-rename pace: `vvx_commit`, `vvx_push`.

## 5. Sigil Convention

All examples use sigils: `₣AA` for firemarks, `₢AAAAC` for coronets.

## 6. Blocking Guarded Auto-Commit for Gallops Modifiers

Commands that modify gallops state auto-commit after success using GUARDED commit infrastructure (blocking, not background).

**Implementation:** Call `./tt/vow-r.RunVVX.sh vvx_commit --message "..."` (blocking).

**Commands with auto-commit:**
- `/jjc-pace-slate` → "Slate: {silks} in {heat}"
- `/jjc-pace-reslate` → "Reslate: {silks}"
- `/jjc-pace-wrap` → "Wrap: {silks}"
- `/jjc-heat-rail` → "Rail: reorder {heat}"
- `/jjc-heat-chalk` → "Chalk: {marker} in {heat}"
- `/jjc-heat-restring` → "Restring: {N} paces from {src} to {dest}"

**Why guarded:** Consistency. All commits through same infrastructure — locking, size check, Co-Authored-By trailer. Single point of control.

**Pattern:**
1. Execute gallops modification
2. On success, run blocking guarded commit with purpose-specific message
3. Report commit hash or failure
4. Continue with post-operation guidance

## Files to Create/Update

See previous tacks for full file list.

## Dependencies

- vvx-push-rename (₢AAAAJ) for vvx_commit name
- jjx-parade-variants (₢AAAAM) for parade output modes
- rail-move-semantics (₢AAAAK) for --move flag documentation

**[260115-1358] rough**

Modernize all JJK/VVK slash commands in one pass.

Six concerns consolidated to minimize file churn:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command showing sibling commands. Example:

```markdown
## Available Operations

**Planning:**
- `/jjc-pace-reslate <pace>` — refine pace specification
- `/jjc-pace-slate` — add new paces
- `/jjc-heat-rail` — reorder paces

**Viewing:**
- `/jjc-parade-overview` — silks, state, one-line per pace
- `/jjc-parade-order` — sequence with states
- `/jjc-parade-full` — paddock + all paces

Use slash commands via Skill tool. Do not call vvx directly.
```

Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants

Create purpose-specific parade commands:

| Command | Purpose | Output |
|---------|---------|--------|
| `/jjc-parade-overview` | Quick status | Silks, state, one-line summary per pace |
| `/jjc-parade-order` | Dependency check | Pace sequence with states |
| `/jjc-parade-detail <pace>` | Inspect one pace | Full tack text for specific pace |
| `/jjc-parade-full` | Planning context | Paddock + all paces |

Rename current `/jjc-heat-parade` to `/jjc-parade-full` (terminal exclusivity).

## 3. Passthrough Adoption

Update all vvx calls to use `./tt/vow-r.RunVVX.sh <subcommand> [args]`.

## 4. New Command Names

Use names from vvx-push-rename pace:
- `vvx_commit` (not `commit`)
- `vvx_push` (new)

## 5. Sigil Convention

All examples use sigils consistently:
- Firemarks: `₣AA` not `AA`
- Coronets: `₢AAAAC` not `AAAAC`

## 6. Blocking Auto-Commit for Gallops Modifiers

Commands that modify gallops state auto-commit after success (blocking, not background).

**Commands with auto-commit:**
- `/jjc-pace-slate` → commits "Slate: {silks} in {heat}"
- `/jjc-pace-reslate` → commits "Reslate: {silks}"
- `/jjc-pace-wrap` → commits "Wrap: {silks}"
- `/jjc-heat-rail` → commits "Rail: reorder {heat}"
- `/jjc-heat-chalk` → commits "Chalk: {marker} in {heat}"
- `/jjc-heat-restring` → commits "Restring: {N} paces from {src} to {dest}"

**Pattern:**
1. Execute gallops modification
2. On success, run blocking commit with purpose-specific message
3. Report commit hash or failure
4. Continue with any post-operation guidance

**Benefits:**
- Small, atomic commits with focused messages
- Immediate failure feedback
- Size guard catches actual problems, not accumulated work
- Clean git history

## Files to Create

- .claude/commands/jjc-parade-overview.md
- .claude/commands/jjc-parade-order.md
- .claude/commands/jjc-parade-detail.md
- .claude/commands/jjc-parade-full.md

## Files to Update

All JJK slash commands (see previous tack for full list).

## Dependencies

- vvx-push-rename (₢AAAAJ) for new command names
- jjx-parade-variants (₢AAAAM) for parade output modes
- rail-move-semantics (₢AAAAK) for --move flag documentation

**[260115-1350] rough**

Modernize all JJK/VVK slash commands in one pass.

Five concerns consolidated to minimize file churn:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command showing sibling commands. Example:

```markdown
## Available Operations

**Planning:**
- `/jjc-pace-reslate <pace>` — refine pace specification
- `/jjc-pace-slate` — add new paces
- `/jjc-heat-rail` — reorder paces

**Viewing:**
- `/jjc-parade-overview` — silks, state, one-line per pace
- `/jjc-parade-order` — sequence with states
- `/jjc-parade-full` — paddock + all paces

Use slash commands via Skill tool. Do not call vvx directly.
```

Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants

Create purpose-specific parade commands:

| Command | Purpose | Output |
|---------|---------|--------|
| `/jjc-parade-overview` | Quick status | Silks, state, one-line summary per pace |
| `/jjc-parade-order` | Dependency check | Pace sequence with states |
| `/jjc-parade-detail <pace>` | Inspect one pace | Full tack text for specific pace |
| `/jjc-parade-full` | Planning context | Paddock + all paces |

Rename current `/jjc-heat-parade` to `/jjc-parade-full` (terminal exclusivity).

Each variant formats output for its purpose — no JSON parsing needed by Claude.

## 3. Passthrough Adoption

Passthrough tabtarget exists: `./tt/vow-r.RunVVX.sh`

Update all vvx calls in slash commands to use:
```bash
./tt/vow-r.RunVVX.sh <subcommand> [args]
```

## 4. New Command Names

Use names from vvx-push-rename pace:
- `vvx_commit` (not `commit`)
- `vvx_push` (new)

## 5. Sigil Convention

All examples use sigils consistently:
- Firemarks: `₣AA` not `AA`
- Coronets: `₢AAAAC` not `AAAAC`

Examples in commands:
```bash
./tt/vow-r.RunVVX.sh jjx_rail ₣AA ₢AAAAC ₢AAAAI ...
./tt/vow-r.RunVVX.sh jjx_parade ₣AA
```

Makes identifiers visually distinct and self-documenting.

## Files to Create

- .claude/commands/jjc-parade-overview.md
- .claude/commands/jjc-parade-order.md
- .claude/commands/jjc-parade-detail.md
- .claude/commands/jjc-parade-full.md (rename from jjc-heat-parade.md)

## Files to Update

- .claude/commands/jjc-heat-groom.md
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-heat-rail.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md
- Tools/jjk/commands/*.md (source copies)

## Dependencies

- vvx-push-rename (₢AAAAJ) for new command names
- jjx-parade-variants (₢AAAAM) for parade output modes
- rail-move-semantics (₢AAAAK) for --move flag documentation

## Supersedes

₢AAAAC (vvw-workbench-tabtarget) — abandoned in favor of this consolidated approach.

**[260115-1310] rough**

Modernize all JJK/VVK slash commands in one pass.

Four concerns consolidated to minimize file churn:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command showing sibling commands. Example:

```markdown
## Available Operations

**Planning:**
- `/jjc-pace-reslate <pace>` — refine pace specification
- `/jjc-pace-slate` — add new paces
- `/jjc-heat-rail` — reorder paces

**Viewing:**
- `/jjc-parade-overview` — silks, state, one-line per pace
- `/jjc-parade-order` — sequence with states
- `/jjc-parade-full` — paddock + all paces

Use slash commands via Skill tool. Do not call vvx directly.
```

Commands reference each other — closed vocabulary, no vvx primitive exposure.

## 2. Parade Variants

Create purpose-specific parade commands:

| Command | Purpose | Output |
|---------|---------|--------|
| `/jjc-parade-overview` | Quick status | Silks, state, one-line summary per pace |
| `/jjc-parade-order` | Dependency check | Pace sequence with states |
| `/jjc-parade-detail <pace>` | Inspect one pace | Full tack text for specific pace |
| `/jjc-parade-full` | Planning context | Paddock + all paces |

Rename current `/jjc-heat-parade` to `/jjc-parade-full` (terminal exclusivity).

Each variant formats output for its purpose — no JSON parsing needed by Claude.

## 3. Passthrough Adoption

Passthrough tabtarget exists: `./tt/vow-r.RunVVX.sh`

Update all vvx calls in slash commands to use:
```bash
./tt/vow-r.RunVVX.sh <subcommand> [args]
```

## 4. New Command Names

Use names from vvx-push-rename pace:
- `vvx_commit` (not `commit`)
- `vvx_push` (new)

## Files to Create

- .claude/commands/jjc-parade-overview.md
- .claude/commands/jjc-parade-order.md
- .claude/commands/jjc-parade-detail.md
- .claude/commands/jjc-parade-full.md (rename from jjc-heat-parade.md)

## Files to Update

- .claude/commands/jjc-heat-groom.md
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-heat-rail.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md
- Tools/jjk/commands/*.md (source copies)

## Dependency

vvx-push-rename (₢AAAAJ) must complete first for new command names.

## Supersedes

₢AAAAC (vvw-workbench-tabtarget) — abandoned in favor of this consolidated approach.

**[260115-1259] rough**

Modernize all JJK/VVK slash commands in one pass.

Two concerns consolidated to minimize file churn:

## 1. Vocabulary Coherence

Add "Available Operations" section to each slash command showing sibling commands. Example for /jjc-heat-groom:

```markdown
## Available Operations

**Planning:**
- `/jjc-pace-reslate <pace>` — refine pace specification
- `/jjc-pace-slate` — add new paces
- `/jjc-heat-rail` — reorder paces

**Progression:**
- `/jjc-pace-prime <pace>` — arm for autonomous execution
- `/jjc-pace-wrap <pace>` — mark complete

**Viewing:**
- `/jjc-heat-parade` — full heat status
- `/jjc-heat-groom` — planning mode (you are here)

Use slash commands via Skill tool. Do not call vvx directly.
```

Commands reference each other — closed vocabulary at slash command level, no vvx primitive exposure.

## 2. Passthrough Adoption

Passthrough tabtarget already exists: `./tt/vow-r.RunVVX.sh`

Update all vvx calls in slash commands to use:
```bash
./tt/vow-r.RunVVX.sh <subcommand> [args]
```

Instead of bare `vvx <subcommand>`.

## 3. New Command Names

Use names from vvx-push-rename pace:
- `vvx_commit` (not `commit`)
- `vvx_push` (new)

## Files to Update

- .claude/commands/jjc-heat-groom.md
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-parade.md
- .claude/commands/jjc-heat-chalk.md
- .claude/commands/jjc-heat-rail.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-pace-wrap.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-pace-prime.md
- .claude/commands/vvc-commit.md
- Tools/jjk/commands/*.md (source copies)

## Dependency

vvx-push-rename (₢AAAAJ) must complete first for new command names.

## Supersedes

₢AAAAC (vvw-workbench-tabtarget) — abandoned in favor of this consolidated approach.

### slate-positioning-flags (₢AAAAW) [complete]

**[260115-1806] complete**

Added positioning flags (--before, --after, --first) to /jjc-pace-slate. Fixed remaining jjc-heat-parade references.

**[260115-1713] rough**

Expose slate positioning flags in /jjc-pace-slate slash command.

JJD supports --before, --after, --first for jjx_slate but the slash command doesn't expose them.

Update /jjc-pace-slate to:
1. Accept optional positioning in arguments (e.g., `--before AAAAC` or `--first`)
2. Pass flags through to vvx jjx_slate
3. Document the positioning options in usage examples

Depends on: slash-command-modernize (₢AAAAL) — should complete first so this pace builds on modernized command format.

### jjd-draft-concepts (₢AAAAO) [complete]

**[260116-0616] complete**

Added jjx_draft operation to JJD: mapping entries, argument definition, Coronet reassignment docs, operation spec with behavior and validation errors. Clarified drafted is not a state. Excluded restring (ceremony, not primitive).

**[260115-1417] rough**

Update JJD-GallopsData.adoc with draft operation concepts.

## Purpose

Establish authoritative definitions for draft and restring operations. Implementation paces (₢AAAAN jjx-draft) reference these definitions.

## Concepts to Define

### Draft

The operation of moving paces from one heat to another.

- Draft moves paces between heats
- Draft reassigns coronets (new firemark, new local ID)
- Draft preserves all tack history
- Draft does NOT change pace state (rough stays rough, complete stays complete)
- Draft is a primitive operation — mechanical, no ceremony

### Restring

The guided workflow for drafting paces with paddock ceremony.

- Restring calls draft primitive
- Restring guides paddock updates on both heats
- Restring adds steeplechase entries
- Restring warns about empty source heat
- Restring is a slash command ceremony, not a primitive

### Coronet Reassignment

When a pace is drafted:
- Old coronet becomes invalid
- New coronet assigned using destination heat seed
- Format: destination firemark + allocated local ID
- Tack history includes "Drafted from ₢{old} in ₣{source}"

## NOT a State

"Drafted" is NOT a pace state. Pace states remain: rough, primed, complete, abandoned.
Draft is an operation/event that moves paces. State is preserved through the move.

## Sections to Update in JJD

1. **Operations** — Add jjx_draft alongside jjx_rail, jjx_slate, etc.
2. **Coronet** — Document reassignment during draft
3. **Glossary** — Add draft, restring, coronet reassignment

## AXLA Annotations

- `{jjd_draft}` — the draft operation
- `{jjd_restring}` — the restring ceremony

## Scope

This pace updates JJD concept model only. Slash command design is separate (handled by jjx-draft pace).

**[260115-1341] rough**

Update JJD-GallopsData.adoc with draft operation concepts.

## New Concepts to Document

**Draft** — Moving paces from one heat to another, reassigning coronets while preserving tack history. The primitive operation.

**Restring** — The guided workflow for drafting paces, including paddock ceremony (reviewing/updating both paddocks, adding steeplechase entries).

**Coronet Reassignment** — When a pace moves heats, its coronet changes to reflect new firemark. Old coronet becomes invalid. Tack history transfers intact.

## Naming Discipline

- Primitive: `jjx_draft` — mechanical operation
- Slash command: `/jjc-heat-restring` — guided workflow with paddock ceremony

Different names because different scope. Restring adds significant ceremony around the primitive.

## Primitive: jjx_draft

```bash
vvx jjx_draft <dest-firemark> <source-firemark> <coronet> [<coronet>...]
```

- Both heats must exist
- Moves paces, reassigns coronets
- Preserves tack history
- Returns old→new coronet mapping

## Slash Command: /jjc-heat-restring

1. Calls jjx_draft
2. Guides source paddock review (remove context for restrung paces)
3. Guides destination paddock review (add context for arriving paces)
4. Adds steeplechase entries to both heats
5. Warns if source becomes empty (suggests retire, does not auto-act)

## Workflow

```bash
# Create destination heat if needed
/jjc-heat-nominate --silks "jjk-command-refinement"

# Restring paces with ceremony
/jjc-heat-restring <dest> <source> <paces...>
```

## Sections to Update in JJD

1. Operations section — Add jjx_draft
2. Coronet section — Document reassignment during draft
3. Pace lifecycle — Add "drafted" as transition
4. Glossary — Add draft, restring terms

## AXLA Annotations

- `{jjd_draft}` — the draft operation
- `{jjd_restring}` — the guided restring workflow

**[260115-1335] rough**

Update JJD-GallopsData.adoc with draft operation concepts.

## New Concepts to Document

**Draft** — Moving paces from one heat to another, reassigning coronets while preserving tack history.

**Coronet Reassignment** — When a pace moves heats, its coronet changes to reflect new firemark. Old coronet becomes invalid. Tack history transfers intact.

**Cross-Heat Operation** — Operations that span two heats (draft is the first). Requires both heats to exist.

## Sections to Add/Update

1. **Operations section** — Add jjx_draft alongside jjx_rail, jjx_slate, etc.

2. **Coronet section** — Document that coronets are heat-scoped and can be reassigned during draft.

3. **Pace lifecycle** — Add "drafted" as a transition (pace leaves heat, enters another).

## AXLA Annotations

Add appropriate annotations for new terms:
- `{jjd_draft}` — the draft operation
- `{jjd_coronet_reassignment}` — coronet change during draft

## Reference

- Current JJD: Tools/jjk/JJD-GallopsData.adoc
- MCM patterns: Tools/cmk/MCM-MetaConceptModel.adoc

## Why Before Implementation

Concept model guides implementation. Documenting draft semantics in JJD ensures the Rust implementation matches the conceptual design.

### jjx-draft (₢AAAAN) [complete]

**[260116-0631] complete**

Implemented jjx_draft primitive and /jjc-heat-restring slash command. Rust: DraftArgs, DraftResult, draft() method with coronet reassignment and tack history preservation. CLI: JjxDraftArgs with --to and positioning flags. Slash command guides paddock updates and steeplechase markers.

**[260115-1417] rough**

Implement jjx_draft primitive and /jjc-heat-restring slash command.

## Reference

Implements concepts defined in JJD-GallopsData.adoc (see jjd-draft-concepts pace ₢AAAAO):
- {jjd_draft} — pace movement operation
- {jjd_restring} — ceremony workflow
- Coronet reassignment semantics

## Primitive: jjx_draft

```bash
vvx jjx_draft ₣<dest> ₣<source> ₢<coronet> [₢<coronet>...]
```

### Behavior (per JJD)

1. Validate: both heats exist, all coronets exist in source
2. For each coronet (in order):
   - Remove pace from source heat
   - Allocate new coronet using destination heat seed
   - Copy all tacks to new pace
   - Add tack entry: "Drafted from ₢{old} in ₣{source}"
   - Preserve pace state (draft does NOT change state)
   - Append to destination heat
3. Return mapping: old coronet → new coronet

### Atomicity

All-or-nothing per JJD definition.

### Example

```bash
vvx jjx_draft ₣AB ₣AA ₢AAAAJ ₢AAAAM ₢AAAAL

₢AAAAJ → ₢ABAAA
₢AAAAM → ₢ABAAB
₢AAAAL → ₢ABAAC
```

## Slash Command: /jjc-heat-restring

Create `.claude/commands/jjc-heat-restring.md` implementing {jjd_restring} ceremony:

1. Call jjx_draft primitive
2. Guide source paddock review
3. Guide destination paddock review
4. Add steeplechase entries to both heats
5. Warn if source becomes empty
6. Auto-commit: "Restring: {N} paces ₣{src} → ₣{dest}"

## Implementation

Rust: Tools/vok/src (near jjx_rail)
Slash command: .claude/commands/jjc-heat-restring.md

## Dependency

Requires jjd-draft-concepts (₢AAAAO) complete first — definitions must exist before implementation.

**[260115-1414] rough**

Add jjx_draft primitive and /jjc-heat-restring slash command.

## Primitive: jjx_draft

```bash
vvx jjx_draft <dest-firemark> <source-firemark> <coronet> [<coronet>...]
```

### Behavior

1. Validate: both heats exist, all coronets exist in source
2. For each coronet (in order):
   - Remove pace from source heat
   - Allocate new coronet using destination heat seed (simplest approach)
   - Copy all tacks to new pace
   - Add tack entry: "Drafted from ₢{old} in ₣{source}"
   - Preserve pace state (rough/primed/complete/abandoned)
   - Append to destination heat
3. Return mapping: old coronet → new coronet

### Atomicity

All-or-nothing. If any operation fails:
- Abort entirely
- Leave both heats in original state
- Report error

### Example

```bash
vvx jjx_draft ₣AB ₣AA ₢AAAAJ ₢AAAAM ₢AAAAL

# Output:
₢AAAAJ → ₢ABAAA
₢AAAAM → ₢ABAAB
₢AAAAL → ₢ABAAC
```

## Slash Command: /jjc-heat-restring

Create `.claude/commands/jjc-heat-restring.md`:

```markdown
Restring paces from one heat to another with paddock ceremony.

Arguments: <dest> <source> <paces...>

1. Call jjx_draft primitive
2. Guide source paddock review (remove restrung context)
3. Guide destination paddock review (add arriving context)
4. Add steeplechase entries to both heats
5. Warn if source becomes empty (suggest retire)
6. Auto-commit with guarded commit: "Restring: {N} paces ₣{src} → ₣{dest}"
```

## Implementation

Rust: Tools/vok/src (near jjx_rail)
Slash command: .claude/commands/jjc-heat-restring.md

**[260115-1341] rough**

Add jjx_draft primitive for moving paces between heats.

## Command

```bash
vvx jjx_draft <dest-firemark> <source-firemark> <coronet> [<coronet>...]
```

## Behavior

1. For each specified coronet (in order):
   - Remove pace from source heat
   - Assign new coronet with destination firemark
   - Preserve all tack history
   - Append to destination heat in specified order

2. Return mapping: old coronet → new coronet

## Validation

- Destination heat must exist
- Source heat must exist
- All coronets must exist in source heat
- Paces can be any state (rough, primed, complete, abandoned)

## Example

```bash
vvx jjx_draft AB AA ₢AAAAJ ₢AAAAM ₢AAAAL

# Output:
₢AAAAJ → ₢ABAAA
₢AAAAM → ₢ABAAB
₢AAAAL → ₢ABAAC
```

## Slash Command

`/jjc-heat-restring` wraps this primitive with paddock ceremony:
1. Calls jjx_draft
2. Guides source paddock review (remove restrung context)
3. Guides destination paddock review (add arriving context)
4. Adds steeplechase entries to both heats
5. Warns if source becomes empty

Note: Different names (draft vs restring) because different scope — slash command adds significant ceremony.

## Implementation

Rust file: Tools/vok/src (near jjx_rail, similar pace manipulation)

**[260115-1335] rough**

Add jjx_draft primitive for moving paces between heats.

## Command

```bash
vvx jjx_draft <dest-firemark> <source-firemark> <coronet> [<coronet>...]
```

## Behavior

1. For each specified coronet (in order):
   - Remove pace from source heat
   - Assign new coronet with destination firemark
   - Preserve all tack history
   - Append to destination heat in specified order

2. Return mapping: old coronet → new coronet

## Validation

- Destination heat must exist
- Source heat must exist
- All coronets must exist in source heat
- Paces can be any state (rough, primed, complete, abandoned)

## Example

```bash
# Draft JJK paces from AA to AB
vvx jjx_draft AB AA ₢AAAAJ ₢AAAAM ₢AAAAL ₢AAAAK

# Output:
₢AAAAJ → ₢ABAAA
₢AAAAM → ₢ABAAB
₢AAAAL → ₢ABAAC
₢AAAAK → ₢ABAAD
```

## Slash Command

`/jjc-heat-draft` wraps this primitive and guides paddock maintenance:
1. Calls jjx_draft
2. Prompts to review/edit source paddock (remove drafted context)
3. Prompts to review/edit destination paddock (add relevant context)
4. Adds steeplechase entries to both heats recording the draft

## Implementation

Rust file: Tools/vok/src (near jjx_rail, similar pace manipulation)

### vok-concept-model (₢AAAAD) [complete]

**[260116-0913] complete**

VOS coverage is sufficient - no separate data model needed. VOS already defines Types, Entities, Places, Assets, Operations, and Key Premises for the release/install infrastructure.

**[260116-0844] rough**

Review VOS-VoxObscuraSpec.adoc coverage and identify remaining data model work.

VOS already covers: Parcel structure, Kit, Whisper/Conclave, ManagedSection/Marker, Sigil, KitForge/TargetRepo, Release/Install/Uninstall operations, Cipher registry.

Original tack requested concepts that are largely addressed. Remaining gaps to discuss:

1. **KitAsset entity** — VOS has Assets category (vosa_) but no explicit KitAsset with source_path/install_path/kit_id members. Is this needed? Current approach uses convention-based discovery (cipher prefix matching) rather than explicit registration.

2. **Manifest schema** — VOS mentions `.claude/vvx-manifest.json` but doesn't define its structure. Is a formal schema needed in VOS?

3. **Whisper discovery** — VOS shows Whisper builder API but not how kits are discovered/collected. Convention-based or explicit?

**Decision needed:** Does VOS need these additions, or is current level sufficient for implementation? The original pace title "vok-concept-model" suggested a separate data model doc (like JJD for Gallops), but VOS already serves that role.

**[260115-1248] rough**

Create MCM-style concept model for VOK release/install system with AXLA annotations.

Document: Tools/vok/lenses/VOKD-VoxObscuraData.adoc (mint appropriate name)

KEY DESIGN DECISIONS TO CAPTURE:

1. **Archive Structure**
   - Archive is the distribution unit
   - Contains lean binaries (install logic only) + plain text kit assets
   - Multi-platform: all binaries bundled, any can perform full install
   - `kits/` directory structure mirrors kit organization

2. **Kit Asset Definition**
   - KitAsset: source_path, install_path, kit membership (metadata only)
   - Content lives as plain text in archive, not embedded in binaries
   - Registry in Rust defines membership, not content

3. **Version/Release Identity**
   - Version numbering scheme (YYMMDD-HHMM)
   - Release naming: `vok-release-{version}.tar.gz`

4. **Kit CLAUDE.md Sections**
   - Template structure for each kits managed section
   - Marker format: `<\!-- MANAGED:{KIT}:BEGIN/END -->`
   - Templates in `kits/{kit}/CLAUDE.md.template`

5. **Two-Repo Relationship**
   - Kit Forge (source repo) vs Target Repo (consumer)
   - Archive bridges the two
   - What crosses: binaries, kit assets, CLAUDE.md templates

CONCEPTS TO DEFINE (with AXLA annotations):
- KitAsset, Archive, ReleaseManifest
- Kit, KitForge, TargetRepo
- ManagedSection, Marker, Template

Reference: JJD-GallopsData.adoc for MCM patterns.

**[260114-1102] rough**

Create MCM-style concept model for VOK release/install system with AXLA annotations.

Document: Tools/vok/lenses/VOKD-VoxObscuraData.adoc (mint appropriate name)

KEY DESIGN DECISIONS TO CAPTURE (not just vocabulary):

1. Kit Asset Definition
   - What constitutes a kit asset (source_path, install_path, content, permissions?)
   - Static vs templated content
   - How assets are declared in Rust (include_str\! pattern)

2. Version/Release Identity
   - Version numbering scheme (YYMMDD-HHMM? semver?)
   - Release naming conventions
   - How versions are embedded and tracked

3. Ledger Design
   - Release history tracking
   - What's recorded per release (timestamp, commit, assets, platforms)
   - Location: Tools/vok/vol_ledger.json?

4. Kit CLAUDE.md Sections
   - Template structure for each kit's managed section
   - What configuration each kit contributes
   - Marker format and freshening rules

5. Permissions Model
   - File permissions during install (executable bits, etc.)
   - Directory creation permissions

6. Two-Repo Relationship
   - Kit Forge (source) vs Target Repo (consumer)
   - What crosses the boundary, what stays veiled
   - burc.env role in target repos

CONCEPTS TO DEFINE (with AXLA annotations):
- KitAsset, ReleaseArchive, InstallManifest
- Veiled vs Public content
- Arcanum (if retained) vs static install
- Ledger, Sigil (version), Codex (tracking)

Reference: JJD-GallopsData.adoc for MCM patterns, Tools/cmk/MCM-MetaConceptModel.adoc for spec.

**[260114-1050] rough**

Create MCM-style concept model for VOK release/install system with AXLA annotations.

Document: Tools/vok/lenses/VOKD-VoxObscuraData.adoc (or similar - mint appropriate name)

Capture from session discussion:
- VOK describes a PROCESS (release/install) but has meaningful data structures worth formalizing
- Two-repo model: source repo (kit forge) vs target repo (consumer)
- Compilation model: knowledge compiles into arcanum emitters, doesn't persist as docs
- Voce Viva (vvx/VVK - user-facing) vs Vox Obscura (VOK - hidden infrastructure)

Key concepts to define (with AXLA annotations for type categorization):
- KitAsset: source_path, install_path, embedded content
- ReleaseArchive: structure of packaged release
- InstallManifest: what's installed where (if any external tracking)
- Veiled vs Public: content that never leaves source repo
- Kit Forge / Target Repo: the two-repo relationship
- Arcanum: install script that configures Claude environment
- Ledger: release record history

Operations to specify (parallel to JJD pattern):
- vvx release: tests → builds → packages archive
- vvx install: snapshot → extract → freshen CLAUDE.md → cleanup → commit
- Version tracking via git (no external manifest)

Scope decision: Focus on data model and operations. Less vocabulary reuse than JJD (Gallops) but the veiled/public distinction and two-repo model warrant formal treatment.

Reference: JJD-GallopsData.adoc for MCM patterns, Tools/cmk/MCM-MetaConceptModel.adoc for spec.

### rcg-establish (₢AAAAB) [complete]

**[260116-0935] complete**

RCG established at Tools/vok/lenses/RCG-RustCodingGuide.md. Covers: file naming ({cipher}r{classifier}), declaration prefixing, z-prefix internals, separate test files, crate boilerplate.

**[260114-1028] rough**

Establish RCG (Rust Coding Guide) for VOK/JJK Rust development. Core tenets: (1) Minting discipline - all files need unique prefixes following CLAUDE.md patterns; (2) Public functions/variables exported by a file must carry that file's prefix; (3) Test organization - study JJK test patterns, likely distinct files with naming like <prefix>rt_<submodule>.rs for Rust Tests. Reference CLAUDE.md Prefix Naming Discipline section. Model structure after BCG (Bash Console Guide) at Tools/buk/lenses/BCG-BashConsoleGuide.md. Note: RCG will be comparatively skimpy vs BCG - trusting more of Claude's inherent Rust idioms; focus only on project-specific conventions.

**[260114-1026] rough**

Establish RCG (Rust Coding Guide) for VOK/JJK Rust development. Core tenets: (1) Minting discipline - all files need unique prefixes following CLAUDE.md patterns; (2) Public functions/variables exported by a file must carry that file's prefix; (3) Test organization - study JJK test patterns, likely distinct files with naming like <prefix>rt_<submodule>.rs for Rust Tests. Reference CLAUDE.md Prefix Naming Discipline section. Model structure after BCG (Bash Console Guide) at Tools/buk/lenses/BCG-BashConsoleGuide.md.

**[260114-1025] rough**

Establish RCG (Rust Coding Guide) for VOK/JJK Rust development. Core tenets: (1) Minting discipline - all files need unique prefixes following CLAUDE.md patterns; (2) Public functions/variables exported by a file must carry that file's prefix; (3) Test organization - study JJK test patterns, likely distinct files with naming like <prefix>rt_<submodule>.rs for Rust Tests. Reference CLAUDE.md Prefix Naming Discipline section.

### saddle-output-field-rename (₢AAAAr) [complete]

**[260117-0804] complete**

Renamed jjx_saddle output fields from tack_text/tack_direction to spec/direction in Rust struct and tests, plus updated 4 slash command docs. All 133 tests pass.

**[260116-1508] bridled**

Rename saddle output fields: tack_text → spec, tack_direction → direction.

## Rationale

Consumers of jjx_saddle output do not need internal terminology. "spec" and "direction" are clearer for slash command prompts.

## Rust changes (jjrq_query.rs)

- Struct field: tack_text → spec
- Struct field: tack_direction → direction
- Update initialization, assignment, tests (~14 lines)

## Slash command changes

- jjc-heat-mount.md (6 references)
- jjc-heat-groom.md (4 references)
- jjc-pace-wrap.md (1 reference)
- jjc-pace-prime.md (1 reference)

## Verification

- cargo build && cargo test
- Spot-check jjx_saddle AA output has new field names

*Direction:* Agent: haiku. Cardinality: 1 sequential. Files: jjrq_query.rs, jjc-heat-mount.md, jjc-heat-groom.md, jjc-pace-wrap.md, jjc-pace-prime.md (5 files). Steps: 1) In jjrq_query.rs: rename tack_text → spec, tack_direction → direction (struct fields, initialization, assignment, test assertions). 2) Verify: cargo build --features jjk && cargo test --features jjk. 3) In each slash command: replace tack_text → spec, tack_direction → direction. Verify: ./tt/vvw-r.RunVVX.sh jjx_saddle AA (confirm new field names in output).

**[260116-1353] rough**

Rename saddle output fields: tack_text → spec, tack_direction → direction.

## Rationale

Consumers of jjx_saddle output do not need internal terminology. "spec" and "direction" are clearer for slash command prompts.

## Rust changes (jjrq_query.rs)

- Struct field: tack_text → spec
- Struct field: tack_direction → direction
- Update initialization, assignment, tests (~14 lines)

## Slash command changes

- jjc-heat-mount.md (6 references)
- jjc-heat-groom.md (4 references)
- jjc-pace-wrap.md (1 reference)
- jjc-pace-prime.md (1 reference)

## Verification

- cargo build && cargo test
- Spot-check jjx_saddle AA output has new field names

### jjk-test-file-separation (₢AAAAv) [complete]

**[260117-0810] complete**

Extracted inline tests from 6 jjr*.rs files to separate jjt*.rs test files per RCG pattern. Added 6 test module declarations to lib.rs. Made private structs pub(crate) for test access. All 133 tests pass.

**[260116-1534] bridled**

Extract inline tests to separate jjt*.rs files per RCG.

## RCG Pattern

Source: `{cipher}r{classifier}_{name}.rs`
Tests:  `{cipher}t{classifier}_{name}.rs`

Example: jjrg_gallops.rs → jjtg_gallops.rs

## Files to Split

1. jjrc_core.rs → jjtc_core.rs
2. jjrf_favor.rs → jjtf_favor.rs
3. jjrg_gallops.rs → jjtg_gallops.rs
4. jjrn_notch.rs → jjtn_notch.rs
5. jjrq_query.rs → jjtq_query.rs
6. jjrs_steeplechase.rs → jjts_steeplechase.rs

Note: jjrx_cli.rs has no inline tests currently.

## Per-file Process

1. Create new jjt{x}_{name}.rs file
2. Move #[cfg(test)] mod tests { ... } content to new file
3. Add `use super::{module}::*;` import
4. Rename test functions with jjt{x}_ prefix
5. Wire in lib.rs: `#[cfg(test)] mod jjt{x}_{name};`

## lib.rs Changes

```rust
pub mod jjrg_gallops;
#[cfg(test)]
mod jjtg_gallops;
```

## Verification

cargo test --manifest-path Tools/jjk/veiled/Cargo.toml

*Direction:* Agent: haiku
Cardinality: 6 parallel + sequential lib.rs update
Files: jjrc_core.rs, jjrf_favor.rs, jjrg_gallops.rs, jjrn_notch.rs, jjrq_query.rs, jjrs_steeplechase.rs, jjtc_core.rs, jjtf_favor.rs, jjtg_gallops.rs, jjtn_notch.rs, jjtq_query.rs, jjts_steeplechase.rs, lib.rs (13 files)
Steps:
1. Each agent (parallel): Extract #[cfg(test)] mod tests block from jjr{x}_{name}.rs to new jjt{x}_{name}.rs
2. Each agent (parallel): Add 'use super::jjr{x}_{name}::*;' import to test file
3. Each agent (parallel): Rename test functions from test_* to jjt{x}_*
4. Each agent (parallel): Remove #[cfg(test)] mod tests block from source file
5. Sequential: Add '#[cfg(test)] mod jjt{x}_{name};' lines to lib.rs for all 6 test modules
Verify: cargo test --manifest-path Tools/jjk/veiled/Cargo.toml

**[260116-1421] rough**

Extract inline tests to separate jjt*.rs files per RCG.

## RCG Pattern

Source: `{cipher}r{classifier}_{name}.rs`
Tests:  `{cipher}t{classifier}_{name}.rs`

Example: jjrg_gallops.rs → jjtg_gallops.rs

## Files to Split

1. jjrc_core.rs → jjtc_core.rs
2. jjrf_favor.rs → jjtf_favor.rs
3. jjrg_gallops.rs → jjtg_gallops.rs
4. jjrn_notch.rs → jjtn_notch.rs
5. jjrq_query.rs → jjtq_query.rs
6. jjrs_steeplechase.rs → jjts_steeplechase.rs

Note: jjrx_cli.rs has no inline tests currently.

## Per-file Process

1. Create new jjt{x}_{name}.rs file
2. Move #[cfg(test)] mod tests { ... } content to new file
3. Add `use super::{module}::*;` import
4. Rename test functions with jjt{x}_ prefix
5. Wire in lib.rs: `#[cfg(test)] mod jjt{x}_{name};`

## lib.rs Changes

```rust
pub mod jjrg_gallops;
#[cfg(test)]
mod jjtg_gallops;
```

## Verification

cargo test --manifest-path Tools/jjk/veiled/Cargo.toml

### prime-to-bridle-rename (₢AAAA0) [complete]

**[260117-0832] complete**

Renamed prime/primed vocabulary to bridle/bridled throughout codebase. Rust enum, JJD spec, CLAUDE.md, 9 slash commands, gallops JSON migration. Added serde alias for backwards compatibility.

**[260116-1530] bridled**

Rename 'prime/primed' vocabulary to 'bridle/bridled' throughout the codebase.

## Scope (from grep analysis)

**Rust files (4 files):**
- jjrg_gallops.rs: PaceState::Primed enum, validation logic, string literals, tests
- jjrx_cli.rs: state parsing, --direction help text
- jjrq_query.rs: saddle output, state display
- jjrn_notch.rs: chalk marker comment

**JJD-GallopsData.adoc:**
- Attribute: :jjdpe_primed: → :jjdpe_bridled:
- ~20 references to {jjdpe_primed}

**Slash commands (5 files):**
- jjc-pace-prime.md → rename to jjc-pace-bridle.md
- jjc-heat-mount.md: references to primed state and /jjc-pace-prime
- jjc-pace-reslate.md: references to prime command
- jjc-pace-slate.md: primeability reference
- jjc-heat-quarter.md: references to prime command (just created)

**CLAUDE.md:**
- Primeability Assessment section → Bridleability Assessment
- /jjc-pace-prime in skills list → /jjc-pace-bridle

**Exclude (unrelated):**
- rbgg_Governor.sh, rbgi_IAM.sh: 'cb_prime' is Cloud Build, not JJ
- Retired heat files: historical, leave as-is

## Approach

Mechanical find-replace with these transformations:
- PaceState::Primed → PaceState::Bridled
- "primed" → "bridled" (string literals)
- primed → bridled (prose, preserving case)
- prime → bridle (command names, prose)
- primeable → bridleable
- primeability → bridleability
- /jjc-pace-prime → /jjc-pace-bridle

Rename file: jjc-pace-prime.md → jjc-pace-bridle.md (git mv)

*Direction:* Agent: sonnet
Cardinality: 2 parallel + sequential build
Files: jjrg_gallops.rs, jjrx_cli.rs, jjrq_query.rs, jjrn_notch.rs, JJD-GallopsData.adoc, jjc-pace-prime.md, jjc-pace-bridle.md, jjc-heat-mount.md, jjc-pace-reslate.md, jjc-pace-slate.md, jjc-heat-quarter.md, CLAUDE.md (12 files)

Batch 1 (parallel):
- Agent A (sonnet): Rust files — replace PaceState::Primed→Bridled, "primed"→"bridled", update test function names
- Agent B (sonnet): Docs — JJD attribute jjdpe_primed→jjdpe_bridled, all {jjdpe_primed} refs, CLAUDE.md primeability→bridleability section

Sequential:
1. git mv .claude/commands/jjc-pace-prime.md .claude/commands/jjc-pace-bridle.md
2. Update all slash commands: /jjc-pace-prime→/jjc-pace-bridle, primed→bridled, primeable→bridleable
3. cargo build --manifest-path Tools/vok/Cargo.toml

Exclude: rbgg_Governor.sh, rbgi_IAM.sh (cb_prime is Cloud Build), retired heats (historical)

**[260116-1530] rough**

Rename 'prime/primed' vocabulary to 'bridle/bridled' throughout the codebase.

## Scope (from grep analysis)

**Rust files (4 files):**
- jjrg_gallops.rs: PaceState::Primed enum, validation logic, string literals, tests
- jjrx_cli.rs: state parsing, --direction help text
- jjrq_query.rs: saddle output, state display
- jjrn_notch.rs: chalk marker comment

**JJD-GallopsData.adoc:**
- Attribute: :jjdpe_primed: → :jjdpe_bridled:
- ~20 references to {jjdpe_primed}

**Slash commands (5 files):**
- jjc-pace-prime.md → rename to jjc-pace-bridle.md
- jjc-heat-mount.md: references to primed state and /jjc-pace-prime
- jjc-pace-reslate.md: references to prime command
- jjc-pace-slate.md: primeability reference
- jjc-heat-quarter.md: references to prime command (just created)

**CLAUDE.md:**
- Primeability Assessment section → Bridleability Assessment
- /jjc-pace-prime in skills list → /jjc-pace-bridle

**Exclude (unrelated):**
- rbgg_Governor.sh, rbgi_IAM.sh: 'cb_prime' is Cloud Build, not JJ
- Retired heat files: historical, leave as-is

## Approach

Mechanical find-replace with these transformations:
- PaceState::Primed → PaceState::Bridled
- "primed" → "bridled" (string literals)
- primed → bridled (prose, preserving case)
- prime → bridle (command names, prose)
- primeable → bridleable
- primeability → bridleability
- /jjc-pace-prime → /jjc-pace-bridle

Rename file: jjc-pace-prime.md → jjc-pace-bridle.md (git mv)

### common-crate-cipher-registry (₢AAAA1) [complete]

**[260117-0849] complete**

Created voi crate with voic_Cipher struct and 17 ciphers. Added validation functions. Updated VOS spec.

**[260117-0845] bridled**

Create common crate with cipher registry as Rust enums/structs.

## Context

- Reference: Memos/memo-20260110-acronym-selection-study.md (minting rules, project-wide acronyms)
- VOS has initial cipher concept to build from
- Must be in a common crate usable from multiple locations (JJK, VVK, etc.)
- This crate is for global naming logic consolidation, not cipher-specific

## Requirements

- Ciphers as Rust types (enums/structs), NOT strings scattered through code
- Validation functions for naming rules
- Naming violations must be compilation errors (type safety)
- Include all project ciphers: JJ, VV, BU, RB, GAD, MCM, AXL, CCC, HM, etc.

## Deliverables

1. Design common crate location and structure
2. Implement cipher enum with all known ciphers
3. Validation functions enforcing minting rules
4. Update existing call sites to use typed ciphers

## References

- Tools/vok/VOS-VoxObscuraSpec.adoc (initial cipher concepts)
- CLAUDE.md Prefix Naming Discipline section

*Direction:* Cardinality: 2 parallel + sequential build
Files: Tools/voi/Cargo.toml, Tools/voi/src/lib.rs, Tools/voi/src/voic_registry.rs, Tools/vok/Cargo.toml, Tools/vok/VOS-VoxObscuraSpec.adoc (5 files)
Steps:
1. Agent A (haiku): Create Tools/voi/ crate with Cargo.toml, src/lib.rs, src/voic_registry.rs; add voi dependency to Tools/vok/Cargo.toml
2. Agent B (sonnet): Update VOS Cipher Registry section — file naming (voic_registry.rs), crate structure (voi), qualified path access pattern, complete cipher list from minting memo
3. Sequential: cargo build --manifest-path Tools/vok/Cargo.toml
Verify: cargo build --manifest-path Tools/vok/Cargo.toml

**[260117-0805] rough**

Create common crate with cipher registry as Rust enums/structs.

## Context

- Reference: Memos/memo-20260110-acronym-selection-study.md (minting rules, project-wide acronyms)
- VOS has initial cipher concept to build from
- Must be in a common crate usable from multiple locations (JJK, VVK, etc.)
- This crate is for global naming logic consolidation, not cipher-specific

## Requirements

- Ciphers as Rust types (enums/structs), NOT strings scattered through code
- Validation functions for naming rules
- Naming violations must be compilation errors (type safety)
- Include all project ciphers: JJ, VV, BU, RB, GAD, MCM, AXL, CCC, HM, etc.

## Deliverables

1. Design common crate location and structure
2. Implement cipher enum with all known ciphers
3. Validation functions enforcing minting rules
4. Update existing call sites to use typed ciphers

## References

- Tools/vok/VOS-VoxObscuraSpec.adoc (initial cipher concepts)
- CLAUDE.md Prefix Naming Discipline section

### vos-install-procedures (₢AAAA2) [complete]

**[260117-0929] complete**

Added axe_bash_scripted to AXLA. Renamed axo_pattern→axo_routine across 4 files. Added VOS control terms, section headers, and 4 AXLA-compliant procedures.

**[260117-0924] bridled**

Add MCM/AXLA-style install procedures to VOS.

## Context

- Read MCM (Tools/cmk/MCM-MetaConceptModel.adoc) and AXLA (Tools/cmk/AXLA-Lexicon.adoc) for procedure patterns
- JJD-GallopsData.adoc is a healthy example of AXLA annotation usage
- RBAGS-AdminGoogleSpec.adoc also demonstrates AXLA patterns
- Key concept is "procedure" - exact AXLA terminology to be determined during execution

## Deliverables

Two MCM/AXLA-style procedures in VOS-VoxObscuraSpec.adoc:

1. **Install process (whole)** - The complete vvx install workflow from archive to target repo
2. **Per-kit process** - The kit-level installation performed as part of the whole install

## References

- Tools/vok/VOS-VoxObscuraSpec.adoc (target file)
- Tools/cmk/MCM-MetaConceptModel.adoc (procedure format)
- Tools/cmk/AXLA-Lexicon.adoc (annotations)
- Tools/jjk/JJD-GallopsData.adoc (healthy AXLA example)
- lenses/RBAGS-AdminGoogleSpec.adoc (another AXLA example)
- Paddock architecture section (install process details)

*Direction:* Agent: opus
Cardinality: 1 sequential
Files: AXLA-Lexicon.adoc, GADS-GoogleAsciidocDifferSpecification.adoc, RBAGS-AdminGoogleSpec.adoc, VOS-VoxObscuraSpec.adoc (4 files)
Steps:
1. Add axe_bash_scripted to AXLA (mapping + definition after axe_bash_unattended)
2. Rename axo_pattern to axo_routine in AXLA (5 changes: mapping entries, anchor, definition, hierarchy reference)
3. Update GADS annotations (3 changes: axo_pattern → axo_routine)
4. Update RBAGS annotations (10 changes: axo_pattern → axo_routine)
5. Add VOS mapping entries (vosok_kit, vosc_*, voss_*)
6. Add VOS Control Terms section after Key Premises (6 definitions: vosc_require, vosc_fatal, vosc_store, vosc_call, vosc_show, vosc_step)
7. Add VOS Section Headers section after Control Terms (4 definitions: voss_inputs, voss_behavior, voss_outputs, voss_completion)
8. Replace VOS Operations section with AXLA-compliant procedures (vosor_release, vosoi_install, vosok_kit, vosou_uninstall) using axo_command/axo_routine with axe_bash_scripted axd_transient
Verify: grep -r axo_pattern (should return only retired files)

**[260117-0805] rough**

Add MCM/AXLA-style install procedures to VOS.

## Context

- Read MCM (Tools/cmk/MCM-MetaConceptModel.adoc) and AXLA (Tools/cmk/AXLA-Lexicon.adoc) for procedure patterns
- JJD-GallopsData.adoc is a healthy example of AXLA annotation usage
- RBAGS-AdminGoogleSpec.adoc also demonstrates AXLA patterns
- Key concept is "procedure" - exact AXLA terminology to be determined during execution

## Deliverables

Two MCM/AXLA-style procedures in VOS-VoxObscuraSpec.adoc:

1. **Install process (whole)** - The complete vvx install workflow from archive to target repo
2. **Per-kit process** - The kit-level installation performed as part of the whole install

## References

- Tools/vok/VOS-VoxObscuraSpec.adoc (target file)
- Tools/cmk/MCM-MetaConceptModel.adoc (procedure format)
- Tools/cmk/AXLA-Lexicon.adoc (annotations)
- Tools/jjk/JJD-GallopsData.adoc (healthy AXLA example)
- lenses/RBAGS-AdminGoogleSpec.adoc (another AXLA example)
- Paddock architecture section (install process details)

### vos-uninstall-procedure (₢AAAA3) [complete]

**[260117-0931] complete**

Already completed as part of vos-install-procedures pace - vosou_uninstall procedure included with full AXLA compliance.

**[260117-0806] rough**

Add MCM/AXLA-style uninstall procedure to VOS.

## Context

- Read MCM (Tools/cmk/MCM-MetaConceptModel.adoc) and AXLA (Tools/cmk/AXLA-Lexicon.adoc) for procedure patterns
- JJD-GallopsData.adoc is a healthy example of AXLA annotation usage
- Key concept is "procedure" - exact AXLA terminology to be determined during execution
- Companion to vos-install-procedures pace

## Deliverables

One MCM/AXLA-style procedure in VOS-VoxObscuraSpec.adoc:

**Uninstall process** - Remove installed kit assets from target repo
- Git-aware (commit before removing)
- Handle CLAUDE.md managed section removal
- Clean up manifest
- Consider partial uninstall (single kit vs all kits)

## References

- Tools/vok/VOS-VoxObscuraSpec.adoc (target file)
- Tools/cmk/MCM-MetaConceptModel.adoc (procedure format)
- Tools/cmk/AXLA-Lexicon.adoc (annotations)
- Tools/jjk/JJD-GallopsData.adoc (healthy AXLA example)
- Paddock architecture section (install process details - inverse applies)

### incorporate-liturgy-vocabulary-vos (₢AAAA6) [complete]

**[260117-0947] complete**

Added liturgy vocabulary (vosl*) to VOS: cipher, signet, epithet, inscription, vesture plus 7 domain vestures. Migrated vost_cipher to voslc_cipher with alias. Updated key references to use liturgy terms.

**[260117-0934] rough**

Incorporate liturgy vocabulary (vosl*) and dispatch vocabulary (vosd*) into VOS.

ORIGIN: Vocabulary designed in chat session analyzing memo-20260110-acronym-selection-study.md. Extended the "Cipher" concept already in VOS into a complete naming system.

VOCABULARY EVOLUTION (why these names):
- "Nomenclature" rejected as too generic → chose "Liturgy" (prescribed ritual form)
- "Cartouche" → "Inscription" (avoid C-collision with Cipher, Canon)
- "Canon" → not needed yet (kit signet-rules deliberately unnamed)
- "Term" → "Epithet" (avoid MCM "linked term" conflict)
- "Form" → "Vesture" (too generic; vesture = clothing/dress for a domain)

DESIGN PATTERN: First-letter trick
- vosl = liturgy umbrella (non-terminal)
- voslc = cipher, vosls = signet, vosle = epithet, vosli = inscription, voslv = vesture
- Enables: `vosl*` greps all liturgy terms; `vosli` is fast and unambiguous

PREFIX TREE (terminal exclusivity verified):
```
vosl (non-terminal: liturgy)
├── voslc (terminal: cipher)
├── vosls (terminal: signet)
├── vosle (terminal: epithet)
├── vosli (terminal: inscription)
├── voslv (terminal: vesture)
└── vosld (non-terminal: domain)
    ├── vosldr (terminal: rust source)
    ├── vosldb (terminal: bash source)
    ├── voslda (terminal: asciidoc attribute)
    ├── vosldp (terminal: publication/spec)
    ├── vosldg (terminal: git ref)
    ├── voslds (terminal: slash command)
    └── vosldt (terminal: tabtarget)
```

CORE DEFINITIONS:
- Cipher: Project root (2-5 chars). The seed. Globally unique, lowercase.
- Signet: Complete prefix before separator. The identity stamp.
- Epithet: Descriptive word after separator. Human-readable portion.
- Inscription: Full artifact name (signet + separator + epithet + envelope).
- Vesture: Domain construction rules (signet_case, separator, epithet_case, envelope*).
  *Envelope deferred to ₢AAAA4

SUBSUMPTION:
- vost_cipher → voslc_cipher (cipher IS a liturgy concept)
- vost_sigil stays (version ID, not nomenclature)
- vosk_prefix_validation should reference liturgy concepts

IMPLEMENTATION STEPS:
1. Add category declarations to VOS mapping section (// vosl*: Liturgy, // vosld*: Domains)
2. Add linked term attributes (:vosli_inscription:, etc.)
3. Add [[anchor]] definitions with // ⟦axl_voices⟧ annotations
4. Decide: migrate vost_cipher → voslc_cipher, or alias?
5. Update vosk_prefix_validation to reference liturgy terms
6. Cross-reference dispatch vocabulary (₢AAAA5)

DELIBERATE DEFERRALS:
- Envelope component (₢AAAA4)
- Kit signet-building rules (unnamed until clearly needed)
- Additional domains: variables, functions, anchors, directories

**[260117-0931] rough**

Incorporate liturgy vocabulary (vosl*) and dispatch vocabulary (vosd*) into VOS.

Reference: Chat session where vocabulary was designed (this chat).

Core terms to add:
- voslc_cipher (subsumes vost_cipher)
- vosls_signet
- vosle_epithet  
- vosli_inscription
- voslv_vesture

Domain catalog:
- vosldr (rust), vosldb (bash), voslda (asciidoc attr)
- vosldp (publication), vosldg (git ref), voslds (slash cmd), vosldt (tabtarget)

Dispatch terms (cross-ref BUK):
- vosdc_colophon, vosdf_frontispiece, vosdi_imprint
- vosdm_formulary, vosdl_launcher

Implementation:
1. Add category declarations to mapping section
2. Add linked term attributes
3. Add definition sections with [[anchors]]
4. Migrate vost_cipher → voslc_cipher (update all references)
5. Cross-reference vosk_prefix_validation to liturgy concepts

### formalize-dispatch-vocabulary (₢AAAA5) [complete]

**[260117-0953] complete**

Added dispatch vocabulary (vosd*) to VOS: colophon, frontispiece, imprint, formulary, launcher. Cross-references liturgy terms for tabtarget inscription patterns.

**[260117-0934] rough**

Incorporate tabtarget/dispatch vocabulary into VOS under vosd* prefix.

DESIGN PATTERN: First-letter trick (like vosl*)
- vosd = dispatch umbrella (non-terminal)
- vosdc = colophon (c)
- vosdf = frontispiece (f)
- vosdi = imprint (i)
- vosdm = formulary (m for mapping)
- vosdl = launcher (l)

TERM CATEGORIES:
Types (string patterns):
- vosdc_colophon: Routing identifier (what formulary matches on)
- vosdf_frontispiece: Human-readable description  
- vosdi_imprint: Embedded parameter(s), target/instance specifier

Entities (structured objects):
- vosdm_formulary: Component that routes colophons to implementations
- vosdl_launcher: Bootstrap script that validates and delegates

RELATIONSHIP TO LITURGY:
- Dispatch vocabulary (vosd*) describes the TABTARGET UNIVERSE conceptually
- Domain vosldt describes the tabtarget DOMAIN VESTURE (construction rules)
- These are complementary, not redundant

CROSS-REFERENCE: BUK README documents implementation details; VOS defines formal linked terms with [[anchors]].

Tabtarget domain vesture (vosldt):
- Pattern: {colophon}.{frontispiece}[.{imprint}].sh
- signet_case: per-colophon (inherits from routed kit)
- separator: . (dot)
- epithet_case: PascalCase (frontispiece)
- envelope: .sh suffix

**[260117-0924] rough**

Incorporate tabtarget/dispatch vocabulary into VOS under vosd* prefix.

Terms to formalize:
- vosdc_colophon: Routing identifier (what formulary matches on)
- vosdf_frontispiece: Human-readable description  
- vosdi_imprint: Embedded parameter(s), target/instance specifier
- vosdm_formulary: Component that routes colophons to implementations
- vosdl_launcher: Bootstrap script that validates and delegates

These terms originate in BUK but belong in VOS as the formal nomenclature.

Cross-reference: BUK README documents implementation; VOS defines the linked terms.

Related domains in liturgy:
- vosldt already reserved for tabtarget domain vesture
- Tabtarget vesture: {colophon}.{frontispiece}[.{imprint}].sh

### rail-first-actionable-semantics (₢AAAA7) [complete]

**[260117-1014] complete**

Implemented first-actionable semantics: --first now positions pace before first rough/bridled pace instead of absolute position 0. Added tests for both standard case and all-complete fallback.

**[260117-0959] rough**

Change --first flag in jjx_rail to position pace at first actionable slot (before first rough/bridled pace) rather than absolute position 0.

RATIONALE: When users say 'move to first', they mean 'make it the next pace to work on.' Moving before 45 completed paces is never useful.

CHANGES:
1. Update jjrg_rail() in jjrg_gallops.rs to find first actionable position
2. Update tests in jjtg_gallops.rs (jjtg_rail_move_first)
3. Update JJD jjda_first definition to clarify semantics
4. Consider: should --last also mean 'last actionable'? Probably not - last overall is useful for deferral

ACCEPTANCE:
- --first places pace before first rough/bridled pace
- If all paces complete/abandoned, --first places at end (nothing actionable)
- --last unchanged (absolute last)
- Tests pass

### kit-asset-registry (₢AAAAA) [abandoned]

**[260117-1012] abandoned**

Registry concept did not materialize. Asset discovery, routing, and file-name validation are implementation details within vosor_release and vosok_kit operations, not a separate entity. VOS now clarifies: (1) vosk_prefix_validation applies to file names only, (2) vov_veiled/ exclusion rule is explicit, (3) vosk_default_routing defines copy-in-place behavior. Work absorbed into vvx-release-impl pace.

**[260116-0908] rough**

Implement convention-based kit asset discovery per VOS spec.

## Approach

VOS establishes convention-based discovery via cipher prefix matching (vosk_prefix_validation). No explicit per-file registration. The registry defines kit metadata and routing rules, not file enumeration.

## Kit Metadata (already in Whisper)

Each kit's Whisper declares:
- `kit_id`: Directory name (e.g., "jjk")
- `cipher`: Namespace prefix (e.g., JJ from voci_ciphers)
- `display_name`: Human-readable name

## Discovery Conventions (implement in Rust)

Source scanning:
- Root: `Tools/{kit_id}/`
- Exclude: `vov_veiled/` subdirectory
- Include: Everything else recursively

Install routing:
- Default: `${BURC_TOOLS_DIR}/{kit_id}/` preserving relative paths
- Commands (`{cipher}c-*.md`): Route to `.claude/commands/`
- Hooks, skills, subagents: Reserved patterns per VOS

## Validation (vosk_prefix_validation)

During release, validate all discovered assets:
- Commands: Must match `{cipher}c-*.md`
- Hooks: Must match `{cipher}h_*`
- Shell scripts: Public functions must use `{cipher}_` prefix
- Fail release if any asset violates prefix rules

## What This Replaces

Original tack proposed explicit KitAsset structs with source_path/install_path per file. VOS favors scanning + convention. Benefits:
- Less maintenance (no manifest updates when adding files)
- Automatic discovery of new assets
- Prefix validation catches naming mistakes
- Single source of truth (filesystem structure)

## Deliverables

1. Asset discovery function: scan kit directory, apply exclusions
2. Install routing logic: map source paths to target paths
3. Prefix validation: check all assets against cipher patterns
4. Integration with Conclave for release-time validation

**[260115-1247] rough**

Define KitAsset struct and registry pattern in Rust.

Each kit declares its assets with:
- `source_path`: Location in source repo (e.g., `Tools/buk/buc_command.sh`)
- `install_path`: Location in target repo (e.g., `Tools/buk/buc_command.sh`)
- Kit membership (which kit owns this asset)

The registry defines WHAT files belong to each kit and WHERE they install — but does NOT embed content. Content lives as plain text in the archive `kits/` directory.

Release process uses registry to:
1. Know which files to copy from source tree to archive
2. Organize files into `kits/{kit}/` structure

Install process uses registry to:
1. Know where to read each file from archive `kits/` directory  
2. Know where to write each file in target repo

No `include_str\!()` — registry is metadata only.

**[260114-0954] rough**

Define KitAsset struct and registry pattern in Rust. Each kit declares its assets with source_path, install_path, and embedded content via include_str\!(). No external manifest files - Rust structs own the knowledge of what files belong to each kit.

### veiled-whisper-spec (₢AAABE) [complete]

**[260117-1208] complete**

Update VOS spec to fully specify the Whisper mechanism before implementation.

CONTEXT:
VOS mentions Whisper (lines 780-799) with a builder API example but lacks:
- Complete API specification
- All builder methods and their semantics
- How whispers register with Conclave
- How managed sections are generated from templates
- Relationship between concept models and CLAUDE.md extraction

GOALS:
1. Fully specify Whisper builder API:
   - .cipher() - symbolic cipher reference
   - .display_name() - human-readable kit name
   - .managed_section(file, tag) - CLAUDE.md template registration
   - .concept_model(file) - concept model registration
   - .register(&mut conclave) - registration with Conclave

2. Specify Conclave responsibilities:
   - How it collects Whispers
   - Validation during release
   - Asset enumeration for install/uninstall

3. Specify managed section generation:
   - How vo{cipher}mc_*.md templates are authored
   - How they reference concept model content
   - Marker format and replacement semantics

4. Clarify veiled directory structure:
   - Rename from veiled/ to vov_veiled/ if needed
   - File naming conventions (vo{cipher}*)
   - What goes in veiled vs kit root

FILES: Tools/vok/VOS-VoxObscuraSpec.adoc

This pace is SPEC ONLY - no implementation. Must complete before veiled-directory-migration can be bridled.

**[260117-1145] rough**

Update VOS spec to fully specify the Whisper mechanism before implementation.

CONTEXT:
VOS mentions Whisper (lines 780-799) with a builder API example but lacks:
- Complete API specification
- All builder methods and their semantics
- How whispers register with Conclave
- How managed sections are generated from templates
- Relationship between concept models and CLAUDE.md extraction

GOALS:
1. Fully specify Whisper builder API:
   - .cipher() - symbolic cipher reference
   - .display_name() - human-readable kit name
   - .managed_section(file, tag) - CLAUDE.md template registration
   - .concept_model(file) - concept model registration
   - .register(&mut conclave) - registration with Conclave

2. Specify Conclave responsibilities:
   - How it collects Whispers
   - Validation during release
   - Asset enumeration for install/uninstall

3. Specify managed section generation:
   - How vo{cipher}mc_*.md templates are authored
   - How they reference concept model content
   - Marker format and replacement semantics

4. Clarify veiled directory structure:
   - Rename from veiled/ to vov_veiled/ if needed
   - File naming conventions (vo{cipher}*)
   - What goes in veiled vs kit root

FILES: Tools/vok/VOS-VoxObscuraSpec.adoc

This pace is SPEC ONLY - no implementation. Must complete before veiled-directory-migration can be bridled.

### veiled-directory-migration (₢AAABF) [complete]

**[260117-1220] complete**

Migrated concept models/lenses to vov_veiled/ across JJK, VOK, CMK, BUK, GAD. Updated Cargo.toml, build.rs, vob_build.sh paths. Updated CLAUDE.md mappings. Build verified.

**[260117-1145] rough**

Migrate concept models to veiled directories and create CLAUDE.md templates.

PREREQUISITE: veiled-whisper-spec (₢AAABE) must be complete first.

TASKS:

1. Rename/create veiled directories per VOS spec:
   - Tools/jjk/veiled/ → Tools/jjk/vov_veiled/ (or restructure)
   - Create Tools/vok/vov_veiled/
   - Create Tools/cmk/vov_veiled/
   - Create Tools/gad/vov_veiled/

2. Move concept model files:
   - JJD-GallopsData.adoc → Tools/jjk/vov_veiled/
   - VOS-VoxObscuraSpec.adoc → Tools/vok/vov_veiled/
   - MCM-MetaConceptModel.adoc → Tools/cmk/vov_veiled/
   - AXLA-Lexicon.adoc → Tools/cmk/vov_veiled/
   - GADS-*.adoc → Tools/gad/vov_veiled/

3. Update CLAUDE.md mappings:
   - All acronym paths in "File Acronym Mappings" section
   - Point to new vov_veiled/ locations

4. Extract CLAUDE.md managed section templates:
   - Analyze current CLAUDE.md for kit-specific content
   - Create vo{cipher}mc_*.md templates for each kit
   - JJK:vojjmc_*.md (Job Jockey configuration section)
   - CMK: vocmmc_*.md (Concept Model Kit section)
   - Identify marker boundaries for managed sections

5. Preserve Rust crate structure:
   - JJK veiled/src/ Rust code needs to remain buildable
   - May need vov_veiled/ alongside veiled/ or restructure

FILES: 
- Tools/*/vov_veiled/ (create)
- Tools/*/*.adoc (move)
- CLAUDE.md (update mappings)
- vo*mc_*.md templates (create)

### installation-identifier (₢AAAAc) [abandoned]

**[260117-1225] abandoned**

Superseded by VOS vose_brand_file specification

**[260116-1032] rough**

Replace hardcoded DEFAULT_BRAND with installation identifier set during parcel generation. Update jjrn_notch.rs and jjrs_steeplechase.rs to read from manifest.

### version-manifest (₢AAAAH) [abandoned]

**[260117-1225] abandoned**

Superseded by VOS vose_brand_file specification

**[260114-1058] rough**

Design and implement version manifest for tracking installed kits.

Proposed location: .claude/vvx-manifest.json

Schema (see paddock):
{
  "version": "260115-1430",    // Release version
  "installed": "260115-1823",  // Install timestamp
  "commit": "abc123def",       // Git commit of install
  "kits": ["jjk", "buk", "cmk", "vok"]
}

Used by:
- Diff analysis (find previous install commit)
- vvx --version to show installed version
- Future upgrade logic

Open decision: Confirm .claude/ as location vs Tools/vok/.

### claude-md-freshening (₢AAAAG) [complete]

**[260117-1244] complete**

Implemented voff_freshen.rs in VOF crate with voff_freshen(), voff_collapse(), and voff_parse_sections() functions. Also migrated VOI→VOF, renamed voic_→vofc_ prefixes, and updated CLAUDE.md template naming to voc{cipher}mc pattern.

**[260114-1058] rough**

Implement CLAUDE.md managed section freshening in Rust.

Marker format (see paddock):
<!-- MANAGED:{KIT}:BEGIN -->
...content from embedded template...
<!-- MANAGED:{KIT}:END -->

Rules to implement:
- Markers are authoritative - content between them replaced entirely
- User content outside markers preserved
- Order of managed sections follows kit installation order
- Missing markers - append section at end of file

Each kit has its CLAUDE.md section template embedded via include_str!().

This is a utility used by vvx-install-impl, may also be useful standalone for testing.

### distributable-kits-typed-registry (₢AAABH) [complete]

**[260117-1318] complete**

Typed kit registry implemented: vofc_Kit and vofc_AssetRoute structs, DISTRIBUTABLE_KITS now typed, kit_id() derived from cipher. Added String Boundary Discipline to RCG.

**[260117-1255] rough**

Implement typed kit registry in VOF crate to consolidate scattered kit knowledge.

## Goal

Replace string-based DISTRIBUTABLE_KITS with typed structs that capture:
- Kit identity (cipher reference, kit_id, display_name)
- Asset routing (what files go where during install)

## Implementation

Add to `vofc_registry.rs`:

```rust
pub struct vofc_Kit {
    pub cipher: &'static vofc_Cipher,
    pub id: &'static str,           // "jjk", "buk", etc.
    pub display_name: &'static str,
}

pub struct vofc_AssetRoute {
    pub source_pattern: &'static str,  // relative to kit dir
    pub target_path: &'static str,     // relative to target repo
    pub is_command: bool,              // routes to .claude/commands/
}

pub const DISTRIBUTABLE_KITS: &[vofc_Kit] = &[
    vofc_Kit { cipher: &BU, id: "buk", display_name: "Bash Utilities" },
    vofc_Kit { cipher: &CM, id: "cmk", display_name: "Concept Model" },
    vofc_Kit { cipher: &JJ, id: "jjk", display_name: "Job Jockey" },
    vofc_Kit { cipher: &VV, id: "vvk", display_name: "Voce Viva" },
];
```

## Asset Routing Rules (hardcoded for MVP)

Per-kit patterns:
- `commands/{cipher}c-*.md` → `.claude/commands/`
- `hooks/{cipher}h_*` → `.claude/hooks/`
- Everything else → `${BURC_TOOLS_DIR}/{kit}/` preserving structure
- Exclude `vov_veiled/` entirely

## Design Decisions

1. vofc_Kit references vofc_Cipher (typed, not string)
2. Kit id is explicit (not derived from cipher + "k")
3. Asset routes are data, not scattered logic
4. Whisper/Conclave can replace this later — structure is compatible

## Files

- Tools/vok/vof/src/vofc_registry.rs (extend)

## Verification

- cargo build succeeds
- cargo test succeeds
- Existing DISTRIBUTABLE_KITS usage still works (provide compatibility accessor)

**[260117-1221] rough**

Refactor kit registry to consolidate scattered hardcoded kit references.

## Current State — "Word Cancer"

Three locations independently hardcode kit knowledge:

1. **voic_registry.rs** (lines 162-167) — distributable kit list:
```rust
pub const DISTRIBUTABLE_KITS: &[&str] = &["buk", "cmk", "jjk", "vvk"];
```

2. **build.rs** (lines 16-18) — kit → Cargo.toml mapping:
```rust
let kits = [
    ("jjk", "../jjk/vov_veiled/Cargo.toml"),
];
```

3. **vob_build.sh** (lines 52-55) — kit feature detection:
```bash
if test -f "${BURC_TOOLS_DIR}/jjk/vov_veiled/Cargo.toml"; then
  ZVOB_FEATURE_LIST="${ZVOB_FEATURE_LIST:+${ZVOB_FEATURE_LIST},}jjk"
fi
```

Meanwhile the typed cipher registry exists with `BU`, `CM`, `JJ`, `VV` constants.

## Problem

- No compile-time guarantee that kit names correspond to registered ciphers
- Kit metadata scattered across multiple files
- Adding a new kit requires updating multiple locations
- Path knowledge duplicated (vov_veiled location)

## Design Questions (not resolved)

1. Kit directory = cipher + "k" suffix. Where does this mapping live?
   - Method on voic_Cipher: `fn kit_dir(&self) -> &str`?
   - Separate struct for "kit" vs "cipher"?

2. Should DISTRIBUTABLE_KITS be `&[voic_Cipher]` or a new `voic_Kit` type?

3. Not all ciphers are kits (e.g., MCM, AXL are vocabulary). How to distinguish?

4. Some kits share ciphers with non-kit uses (e.g., `vv` is both VVC crate and VVK kit).

5. Kit → Cargo.toml path: derive from convention or explicit registry?

6. How does bash (vob_build.sh) consume Rust-owned registry? Generated file?

## Not MVP

Raw strings work. This is type-safety cleanup for later.

## Status

Needs design thinking before implementation. Do not bridle without resolving design questions.

**[260117-1210] rough**

Refactor DISTRIBUTABLE_KITS to use typed voic_Cipher registry instead of raw strings.

## Current State

`DISTRIBUTABLE_KITS` (voic_registry.rs:162-167) uses raw strings:
```rust
pub const DISTRIBUTABLE_KITS: &[&str] = &["buk", "cmk", "jjk", "vvk"];
```

Meanwhile the typed cipher registry exists with `BU`, `CM`, `JJ`, `VV` constants.

## Problem

No compile-time guarantee that distributable kit names correspond to registered ciphers.

## Design Questions (not resolved)

1. Kit directory = cipher + "k" suffix. Where does this mapping live?
   - Method on voic_Cipher: `fn kit_dir(&self) -> &str`?
   - Separate struct for "kit" vs "cipher"?
   - Const fn that derives it?

2. Should DISTRIBUTABLE_KITS be `&[voic_Cipher]` or a new `voic_Kit` type?

3. Not all ciphers are kits (e.g., MCM, AXL are vocabulary, not kits). How to distinguish?

4. Some kits share ciphers with non-kit uses (e.g., `vv` is both VVC crate and VVK kit).

## Not MVP

Raw strings work. This is type-safety cleanup for later.

## Status

Needs design thinking before implementation. Do not bridle without resolving design questions.

### vvx-release-impl (₢AAAAE) [complete]

**[260117-1410] complete**

Implemented VVK parcel release: vob_parcel orchestrates tests, build, release_collect, release_brand, tarball. First hallmark 1000 allocated.

**[260117-1354] rough**

Implement VVK parcel release: tests, build, asset collection, branding, tarball.

## Overview

Release creates `vvk-parcel-{hallmark}.tar.gz` per VOS spec (vosor_release).
Bash orchestration calls Rust utilities for asset collection and branding.

## Platform Scope

**MVP: Single-platform only.** Release builds for the current platform only. Parcel contains one vvx binary matching the build host. Cross-platform parcels (all three binaries) require cross-compilation infrastructure — deferred to post-MVP.

## Layered Architecture

**Bash orchestration** (user-facing, handles git):
- `vob_parcel` function in vob_build.sh — orchestrates release
- `vvi_install.sh` — bootstrap script (source-controlled, copied to parcel root)

**Rust utilities** (mechanical work, no git):
- `release_collect` — enumerate and copy kit assets to staging
- `release_brand` — compute SHA, allocate hallmark, write brand file

## Files

### Existing to Modify

- **Tools/vok/src/vorm_main.rs** — add release_collect and release_brand subcommands
- **Tools/vok/vob_build.sh** — add vob_parcel function

### New to Create

1. **Tools/vvk/vvi_install.sh** — Static bootstrap script
   - Accepts one argument: path to target's burc.env file
   - Validates burc.env exists and is readable
   - Detects platform (darwin-arm64, darwin-x86_64, linux-x86_64)
   - Invokes `./kits/vvk/bin/vvx-{platform} vvx_emplace --parcel . --burc <burc.env>`
   - NOTE: This file lives in source at Tools/vvk/ but release copies it to PARCEL ROOT (not kits/vvk/)

2. **Tools/vok/vov_veiled/vovr_registry.json** — Initial empty registry
   ```json
   { "hallmarks": {} }
   ```

## Rust Subcommands (vorm_main.rs)

### release_collect --staging <dir>

Inputs: staging directory path

Behavior:
1. Iterate DISTRIBUTABLE_KITS from vofc_registry
2. For each kit, enumerate source files:
   - Source: `${BURC_TOOLS_DIR}/{kit_id}/`
   - Skip any path containing `vov_veiled/`
3. Copy files to staging preserving structure:
   - Default: `kits/{kit_id}/{relative_path}`
   - Commands (`{cipher}c-*.md`): `kits/{kit_id}/commands/`
4. Copy `Tools/vvk/vvi_install.sh` to staging root (NOT to kits/vvk/)
5. Output: JSON with file counts per kit

### release_brand --staging <dir>

Inputs: staging directory path

Behavior:
1. Compute super-SHA:
   - List all files in staging (excluding vvbf_brand.json)
   - Sort by path
   - Hash each file: SHA256(path + content)
   - Combine hashes → super-SHA
2. Load vovr_registry.json (create if missing)
3. Hallmark allocation:
   - If super-SHA exists: reuse existing hallmark
   - If new: allocate next sequential (max+1, starting at 1000)
4. Write vvbf_brand.json to staging root:
   ```json
   {
     "vvbh_hallmark": 1000,
     "vvbd_date": "260117-1430",
     "vvbs_sha": "abc123...",
     "vvbc_commit": "9a8f15ef...",
     "vvbk_kits": ["buk", "cmk", "jjk", "vvk"]
   }
   ```
5. If new hallmark: update vovr_registry.json, commit it
6. Output: hallmark value (for bash to use in tarball name)

## Bash Orchestration (vob_build.sh → vob_parcel)

New function `vob_parcel`:
1. STEP: Run tests — `cargo test --manifest-path ... --features jjk`
2. STEP: Build binary — call existing `vob_build` (current platform only)
3. STEP: Create staging — `${BUD_TEMP_DIR}/staging`
4. STEP: Collect assets — `vvx release_collect --staging $staging`
5. STEP: Copy platform binary — `cp vvx-{platform} $staging/kits/vvk/bin/`
6. STEP: Brand parcel — `vvx release_brand --staging $staging`
7. STEP: Create tarball — `tar -czf vvk-parcel-{hallmark}.tar.gz -C $staging .`
8. Output: path to tarball

## Parcel Structure (verification)

```
vvk-parcel-1000/
├── vvi_install.sh            # At root, NOT in kits/vvk/
├── vvbf_brand.json
└── kits/
    ├── buk/
    │   ├── buc_command.sh
    │   └── ...
    ├── cmk/
    │   └── ...
    ├── jjk/
    │   ├── jju_utility.sh
    │   └── commands/
    │       └── jjc-*.md
    └── vvk/
        ├── vvu_uninstall.sh  # Uninstall IS a kit asset
        └── bin/
            └── vvx-darwin-arm64  # Single platform for MVP
```

## Testing Strategy

1. **Unit tests** (Rust):
   - Super-SHA computation is deterministic
   - Hallmark allocation: reuse vs new
   - Brand file serialization

2. **Integration test**:
   - `tt/vow-R.Release.sh` produces valid tarball
   - Tarball contains expected structure
   - vvi_install.sh at root, vvu_uninstall.sh in kits/vvk/
   - Repeated release with same content reuses hallmark

## Prerequisites

- ₢AAABH (typed kit registry) — COMPLETE

**[260117-1328] rough**

Implement VVK parcel release: tests, build, asset collection, branding, tarball.

## Overview

Release creates `vvk-parcel-{hallmark}.tar.gz` per VOS spec (vosor_release).
Bash orchestration calls Rust utilities for asset collection and branding.

## Layered Architecture

**Bash orchestration** (user-facing, handles git):
- `vvi_install.sh` → calls `vvx_emplace`, manages git commits
- `vvu_uninstall.sh` → calls `vvx_vacate`, manages git commits

**Rust utilities** (mechanical work, no git):
- `release_collect` — enumerate and copy kit assets to staging
- `release_brand` — compute SHA, allocate hallmark, write brand file
- `vvx_emplace` — copy files from parcel to target, freshen CLAUDE.md
- `vvx_vacate` — remove files, collapse CLAUDE.md sections

## Files to Create

1. **Tools/vvk/vvi_install.sh** — Static bootstrap script
   - Detect platform (darwin-arm64, darwin-x86_64, linux-x86_64)
   - Invoke `./kits/vvk/bin/vvx-{platform} vvx_emplace --parcel . --target <repo>`
   - Handles pre/post git commits, diff analysis

2. **Tools/vok/vov_veiled/vovr_registry.json** — Initial empty registry
   ```json
   { "hallmarks": {} }
   ```

## Rust Subcommands (vorm_main.rs)

### release_collect --staging <dir>

Inputs: staging directory path
Behavior:
1. Iterate DISTRIBUTABLE_KITS from vofc_registry
2. For each kit, enumerate source files:
   - Source: `${BURC_TOOLS_DIR}/{kit_id}/`
   - Skip any path containing `vov_veiled/`
3. Copy files to staging preserving structure:
   - Default: `kits/{kit_id}/{relative_path}`
   - Commands (`{cipher}c-*.md`): `kits/{kit_id}/commands/`
4. Copy `vvi_install.sh` to staging root
5. Output: JSON with file counts per kit

### release_brand --staging <dir>

Inputs: staging directory path
Behavior:
1. Compute super-SHA:
   - List all files in staging (excluding vvbf_brand.json)
   - Sort by path
   - Hash each file: SHA256(path + content)
   - Combine hashes → super-SHA
2. Load vovr_registry.json (create if missing)
3. Hallmark allocation:
   - If super-SHA exists: reuse existing hallmark
   - If new: allocate next sequential (max+1, starting at 1000)
4. Write vvbf_brand.json to staging root:
   ```json
   {
     "vvbh_hallmark": 1000,
     "vvbd_date": "260117-1430",
     "vvbs_sha": "abc123...",
     "vvbc_commit": "9a8f15ef...",
     "vvbk_kits": ["buk", "cmk", "jjk", "vvk"]
   }
   ```
5. If new hallmark: update vovr_registry.json, commit it
6. Output: hallmark value (for bash to use in tarball name)

### vvx_emplace --parcel <dir> --target <repo>

(For pace ₢AAAAF — documented here for naming clarity)
Inputs: extracted parcel directory, target repo path
Behavior:
1. Read vvbf_brand.json from parcel
2. Copy kits/* to ${BURC_TOOLS_DIR}/
3. Route commands to .claude/commands/
4. Freshen CLAUDE.md using voff_freshen
5. Copy brand file to .vvk/vvbf_brand.json
Output: summary of installed files

### vvx_vacate --target <repo>

(For uninstall — documented here for naming clarity)
Inputs: target repo path
Behavior:
1. Read .vvk/vvbf_brand.json for kit list
2. Remove files by cipher pattern from .claude/commands/
3. Collapse CLAUDE.md sections using voff_collapse
4. Remove kit directories
5. Remove brand file
Output: summary of removed files

## Bash Orchestration (vob_build.sh → vob_parcel)

New function `vob_parcel`:
1. STEP: Run tests — `cargo test --manifest-path ... --features jjk`
2. STEP: Build binary — call existing `vob_build`
3. STEP: Create staging — `${BUD_TEMP_DIR}/staging`
4. STEP: Collect assets — `vvx release_collect --staging $staging`
5. STEP: Copy platform binary — `cp vvx-{platform} $staging/kits/vvk/bin/`
6. STEP: Brand parcel — `vvx release_brand --staging $staging`
7. STEP: Create tarball — `tar -czf vvk-parcel-{hallmark}.tar.gz -C $staging .`
8. Output: path to tarball

## Parcel Structure (verification)

```
vvk-parcel-1000/
├── vvi_install.sh
├── vvbf_brand.json
└── kits/
    ├── buk/
    │   ├── buc_command.sh
    │   └── ...
    ├── cmk/
    │   └── ...
    ├── jjk/
    │   ├── jju_utility.sh
    │   └── commands/
    │       └── jjc-*.md
    └── vvk/
        └── bin/
            └── vvx-darwin-arm64
```

## Testing Strategy

1. **Unit tests** (Rust):
   - Super-SHA computation is deterministic
   - Hallmark allocation: reuse vs new
   - Brand file serialization

2. **Integration test**:
   - `tt/vow-R.Release.sh` produces valid tarball
   - Tarball contains expected structure
   - Repeated release with same content reuses hallmark

## Scope

This pace implements release_collect and release_brand only.
vvx_emplace is pace ₢AAAAF.
vvx_vacate can be a separate pace.

## Prerequisites

- ₢AAABH (typed kit registry) — COMPLETE

**[260117-1324] rough**

Implement VVK parcel release: tests, build, asset collection, branding, tarball.

## Overview

Release creates `vvk-parcel-{hallmark}.tar.gz` per VOS spec (vosor_release).
Bash orchestration calls Rust utilities for asset collection and branding.

## Files to Create

1. **Tools/vvk/vvi_install.sh** — Static bootstrap script
   - Detect platform (darwin-arm64, darwin-x86_64, linux-x86_64)
   - Invoke `./kits/vvk/bin/vvx-{platform} install --target <repo>`
   - Simple, no dependencies

2. **Tools/vok/vov_veiled/vovr_registry.json** — Initial empty registry
   ```json
   { "hallmarks": {} }
   ```

## Rust Subcommands (vorm_main.rs)

### vvx release_collect --staging <dir>

Inputs: staging directory path
Behavior:
1. Iterate DISTRIBUTABLE_KITS from vofc_registry
2. For each kit, enumerate source files:
   - Source: `${BURC_TOOLS_DIR}/{kit_id}/`
   - Skip any path containing `vov_veiled/`
3. Copy files to staging preserving structure:
   - Default: `kits/{kit_id}/{relative_path}`
   - Commands (`{cipher}c-*.md`): `kits/{kit_id}/commands/`
4. Copy `vvi_install.sh` to staging root
5. Output: count of files collected per kit

### vvx release_brand --staging <dir>

Inputs: staging directory path
Behavior:
1. Compute super-SHA:
   - List all files in staging (excluding vvbf_brand.json)
   - Sort by path
   - Hash each file: SHA256(path + content)
   - Combine hashes → super-SHA
2. Load vovr_registry.json (or create if missing)
3. Hallmark allocation:
   - If super-SHA exists: reuse existing hallmark
   - If new: allocate next sequential (max+1, starting at 1000)
4. Write vvbf_brand.json to staging root:
   ```json
   {
     "vvbh_hallmark": 1000,
     "vvbd_date": "260117-1430",
     "vvbs_sha": "abc123...",
     "vvbc_commit": "9a8f15ef...",
     "vvbk_kits": ["buk", "cmk", "jjk", "vvk"]
   }
   ```
5. If new hallmark: update vovr_registry.json, commit it
6. Output: hallmark (for bash to use in tarball name)

## Bash Orchestration (vob_build.sh → vob_parcel)

New function `vob_parcel`:
1. STEP: Run tests — `cargo test --manifest-path ... --features jjk`
2. STEP: Build binary — call existing `vob_build`
3. STEP: Create staging — `mktemp -d` or `${BUD_TEMP_DIR}/staging`
4. STEP: Collect assets — `vvx release_collect --staging $staging`
5. STEP: Copy platform binary — `cp vvx-{platform} $staging/kits/vvk/bin/`
6. STEP: Brand parcel — `vvx release_brand --staging $staging`
7. STEP: Create tarball — `tar -czf vvk-parcel-{hallmark}.tar.gz -C $staging .`
8. Output: path to tarball

## Parcel Structure (verification)

```
vvk-parcel-1000/
├── vvi_install.sh
├── vvbf_brand.json
└── kits/
    ├── buk/
    │   ├── buc_command.sh
    │   └── ...
    ├── cmk/
    │   └── ...
    ├── jjk/
    │   ├── jju_utility.sh
    │   └── commands/
    │       └── jjc-*.md
    └── vvk/
        └── bin/
            └── vvx-darwin-arm64
```

## Testing Strategy

1. **Unit tests** (Rust):
   - Super-SHA computation is deterministic
   - Hallmark allocation: reuse vs new
   - Brand file serialization

2. **Integration test**:
   - `tt/vow-R.Release.sh` produces valid tarball
   - Tarball contains expected structure
   - Repeated release with same content reuses hallmark

## Prerequisites

- ₢AAABH (typed kit registry) — COMPLETE

## MVP Constraints

- Single platform binary (current platform only)
- Hardcoded kit list from DISTRIBUTABLE_KITS

**[260117-1256] rough**

Implement VVK parcel release via vow_workbench.sh orchestration + Rust utilities.

## Architecture Split

**Bash (vow_workbench.sh → vow_release):**
1. Run testbenches (cargo test for all crates)
2. Build vvx binary (cargo build --release)
3. Create staging directory
4. Call `vvx release_collect` to populate staging
5. Call `vvx release_brand` to write brand file
6. Create tarball: `vvk-parcel-{hallmark}.tar.gz`

**Rust (vvx subcommands):**

`vvx release_collect --staging <dir>`:
- Validate prefix compliance for all kit assets
- Enumerate files using vofc_Kit registry
- Copy files to staging dir with correct routing
- Copy `vvi_install.sh` to staging root
- Report what was collected

`vvx release_brand --staging <dir>`:
- Compute content super-SHA of staging dir
- Read/create vovr_registry.json
- Allocate hallmark (reuse if SHA exists, else next sequential from 1000)
- Write vvbf_brand.json to staging root
- Output hallmark for bash to use in tarball name

## Parcel Structure

```
vvk-parcel-{hallmark}/
├── vvi_install.sh        # Static file from source
├── vvbf_brand.json       # Generated by release_brand
└── kits/
    ├── buk/...
    ├── cmk/...
    ├── jjk/...
    └── vvk/
        └── bin/
            └── vvx-darwin-arm64  # MVP: current platform only
```

## MVP Constraints

- macOS only (darwin-arm64 or darwin-x86_64)
- Single platform binary in parcel
- Hardcoded kit list from vofc_registry (no Whisper yet)

## Prerequisites

- ₢AAABH (typed kit registry) — provides vofc_Kit for asset enumeration

## Files

- Tools/vok/vow_workbench.sh (add vow_release)
- Tools/vok/src/vorm_main.rs (add release_collect, release_brand subcommands)
- Tools/vok/vov_veiled/vovr_registry.json (created if missing)
- Tools/vvk/vvi_install.sh (static, must exist)

## Verification

- tt/vow-r.Release.sh produces vvk-parcel-{hallmark}.tar.gz
- Tarball contains expected structure
- Brand file has valid hallmark
- Repeated release with same content reuses hallmark

**[260115-1247] rough**

Implement vvx release subcommand in Rust.

Responsibilities:
1. Run all testbenches — fail release if tests fail
2. Build vvx for target platforms (darwin-arm64, darwin-x86_64, linux-x86_64)
3. Collect kit assets from source tree using KitAsset registry
4. Organize into archive structure:
   ```
   vok-release-YYMMDD-HHMM/
   ├── bin/
   │   ├── vvx-darwin-arm64
   │   ├── vvx-darwin-x86_64
   │   └── vvx-linux-x86_64
   └── kits/
       ├── buk/...
       ├── cmk/...
       ├── jjk/...
       └── vok/...
   ```
5. Package archive: `vok-release-YYMMDD-HHMM.tar.gz`

Key: Kit assets are COPIED as plain text files from source tree to archive `kits/` directory. Not embedded in binaries.

Depends on: kit-asset-registry pace for KitAsset structs defining what to collect.

**[260114-1057] rough**

Implement vvx release subcommand in Rust.

Responsibilities (see paddock Architecture section):
1. Run all testbenches - fail release if tests fail
2. Build vvx for target platforms (darwin-arm64, darwin-x86_64, linux-x86_64)
3. Collect kit assets via KitAsset registry (depends on kit-asset-registry pace)
4. Package archive: vok-release-YYMMDD-HHMM.tar.gz

Archive structure per paddock - lean package with binaries + install tabtarget.
Kit files embedded in vvx binary, not loose in archive.

Depends on: kit-asset-registry pace for KitAsset structs.

### vvx-install-impl (₢AAAAF) [complete]

**[260118-0730] complete**

Implemented vvx_emplace: 4 files (vofm_managed.rs, vofe_emplace.rs, lib.rs, vorm_main.rs). Hardcoded CLAUDE.md templates, burc parser, kit copier, command/hook router, freshen integration. Build passes, 25 tests pass.

**[260118-0726] bridled**

Implement vvx_emplace: the Rust utility that copies kit assets from parcel to target repo.

## Layered Architecture

**vvi_install.sh (bash orchestration):**
1. Parse args — path to target's burc.env file
2. Source burc.env, extract BURC_TOOLS_DIR and BURC_PROJECT_ROOT
3. Detect platform, select vvx binary from parcel
4. Verify target is git repo
5. Pre-install snapshot — `git commit` if working tree dirty
6. Call `vvx_emplace --parcel . --burc <burc.env>`
7. Post-install commit — `git commit -m "VVK install: {hallmark}"`
8. Diff analysis — compare pre/post, invoke Claude for recovery guidance

**vvx_emplace (Rust, this pace):**
Pure file operations, no git. Called by vvi_install.sh.

## vvx_emplace --parcel <dir> --burc <path>

Inputs:
- `--parcel` — extracted parcel directory (contains vvbf_brand.json, kits/)
- `--burc` — path to target's burc.env file

Behavior:

1. **Parse BURC**
   - Read and parse burc.env file
   - Extract BURC_TOOLS_DIR, BURC_PROJECT_ROOT
   - FATAL if file missing or variables undefined

2. **Read brand file**
   - Parse `{parcel}/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`

3. **Copy kit assets**
   - For each kit in `{parcel}/kits/`:
     - Copy directory to `${BURC_TOOLS_DIR}/{kit}/`
     - Preserve internal structure

4. **Route special assets**
   - Commands (`{cipher}c-*.md`): copy to `${BURC_PROJECT_ROOT}/.claude/commands/`
   - Hooks (`{cipher}h_*`): copy to `${BURC_PROJECT_ROOT}/.claude/hooks/`

5. **Freshen CLAUDE.md** (MVP: hardcoded templates)
   - Template content is hardcoded in Rust constants (vofm_managed.rs)
   - Each kit's CLAUDE.md section defined as const in vof crate
   - Call voff_freshen() with sections for installed kits
   - Write updated CLAUDE.md to BURC_PROJECT_ROOT
   - **Post-MVP**: ₢AAABK will externalize templates to parcel files

6. **Copy brand file**
   - Create `${BURC_PROJECT_ROOT}/.vvk/` directory if needed
   - Copy `{parcel}/vvbf_brand.json` to `.vvk/vvbf_brand.json`

Output: JSON summary
```json
{
  "hallmark": 1000,
  "kits_installed": ["buk", "cmk", "jjk", "vvk"],
  "files_copied": 47,
  "commands_routed": 12,
  "claude_sections_updated": ["BUK", "CMK", "JJK", "VVK"]
}
```

## MVP Shortcut: Hardcoded CLAUDE.md Templates

For MVP, each kit's CLAUDE.md content is defined in Rust:

```rust
// In vof crate: vofm_managed.rs
pub const BUK_MANAGED_SECTION: voff_ManagedSection = voff_ManagedSection {
    tag: "BUK",
    content: r#"
## Bash Utility Kit (BUK)
...content...
"#,
};

pub const JJK_MANAGED_SECTION: voff_ManagedSection = voff_ManagedSection {
    tag: "JJK",
    content: r#"
## Job Jockey Configuration
...content...
"#,
};
```

This means:
- Changing CLAUDE.md content requires Rust rebuild + new release
- Acceptable for MVP; ₢AAABK will externalize to template files
- Keep sections minimal — just essential config, not full docs

## Key Design Points

- **No git operations** — bash handles all commits
- **Idempotent** — can re-run safely (overwrites existing)
- **Platform-agnostic** — any vvx binary can emplace all kits
- **BURC-driven** — all paths derived from burc.env

## Prerequisites

- ₢AAAAE (vvx-release-impl) — provides parcel structure
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_freshen
- ₢AAABH (typed kit registry) — COMPLETE, provides vofc_Kit

## Files

- Tools/vok/src/vorm_main.rs — add vvx_emplace subcommand
- Tools/vok/vof/src/vofm_managed.rs — NEW: hardcoded CLAUDE.md templates

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vofm_managed.rs, vofe_emplace.rs, lib.rs, vorm_main.rs (4 files)
Steps:
1. Create vofm_managed.rs with minimal CLAUDE.md templates for BUK, CMK, JJK, VVK as voff_ManagedSection constants
2. Create vofe_emplace.rs with: burc.env parser, brand file reader, kit copier, command/hook router, freshen caller, brand copier
3. Update lib.rs to export new modules
4. Add vvx_emplace subcommand to vorm_main.rs with --parcel and --burc args
Verify: ./tt/vow-b.Build.sh

**[260117-1355] rough**

Implement vvx_emplace: the Rust utility that copies kit assets from parcel to target repo.

## Layered Architecture

**vvi_install.sh (bash orchestration):**
1. Parse args — path to target's burc.env file
2. Source burc.env, extract BURC_TOOLS_DIR and BURC_PROJECT_ROOT
3. Detect platform, select vvx binary from parcel
4. Verify target is git repo
5. Pre-install snapshot — `git commit` if working tree dirty
6. Call `vvx_emplace --parcel . --burc <burc.env>`
7. Post-install commit — `git commit -m "VVK install: {hallmark}"`
8. Diff analysis — compare pre/post, invoke Claude for recovery guidance

**vvx_emplace (Rust, this pace):**
Pure file operations, no git. Called by vvi_install.sh.

## vvx_emplace --parcel <dir> --burc <path>

Inputs:
- `--parcel` — extracted parcel directory (contains vvbf_brand.json, kits/)
- `--burc` — path to target's burc.env file

Behavior:

1. **Parse BURC**
   - Read and parse burc.env file
   - Extract BURC_TOOLS_DIR, BURC_PROJECT_ROOT
   - FATAL if file missing or variables undefined

2. **Read brand file**
   - Parse `{parcel}/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`

3. **Copy kit assets**
   - For each kit in `{parcel}/kits/`:
     - Copy directory to `${BURC_TOOLS_DIR}/{kit}/`
     - Preserve internal structure

4. **Route special assets**
   - Commands (`{cipher}c-*.md`): copy to `${BURC_PROJECT_ROOT}/.claude/commands/`
   - Hooks (`{cipher}h_*`): copy to `${BURC_PROJECT_ROOT}/.claude/hooks/`

5. **Freshen CLAUDE.md** (MVP: hardcoded templates)
   - Template content is hardcoded in Rust constants (vofm_managed.rs)
   - Each kit's CLAUDE.md section defined as const in vof crate
   - Call voff_freshen() with sections for installed kits
   - Write updated CLAUDE.md to BURC_PROJECT_ROOT
   - **Post-MVP**: ₢AAABK will externalize templates to parcel files

6. **Copy brand file**
   - Create `${BURC_PROJECT_ROOT}/.vvk/` directory if needed
   - Copy `{parcel}/vvbf_brand.json` to `.vvk/vvbf_brand.json`

Output: JSON summary
```json
{
  "hallmark": 1000,
  "kits_installed": ["buk", "cmk", "jjk", "vvk"],
  "files_copied": 47,
  "commands_routed": 12,
  "claude_sections_updated": ["BUK", "CMK", "JJK", "VVK"]
}
```

## MVP Shortcut: Hardcoded CLAUDE.md Templates

For MVP, each kit's CLAUDE.md content is defined in Rust:

```rust
// In vof crate: vofm_managed.rs
pub const BUK_MANAGED_SECTION: voff_ManagedSection = voff_ManagedSection {
    tag: "BUK",
    content: r#"
## Bash Utility Kit (BUK)
...content...
"#,
};

pub const JJK_MANAGED_SECTION: voff_ManagedSection = voff_ManagedSection {
    tag: "JJK",
    content: r#"
## Job Jockey Configuration
...content...
"#,
};
```

This means:
- Changing CLAUDE.md content requires Rust rebuild + new release
- Acceptable for MVP; ₢AAABK will externalize to template files
- Keep sections minimal — just essential config, not full docs

## Key Design Points

- **No git operations** — bash handles all commits
- **Idempotent** — can re-run safely (overwrites existing)
- **Platform-agnostic** — any vvx binary can emplace all kits
- **BURC-driven** — all paths derived from burc.env

## Prerequisites

- ₢AAAAE (vvx-release-impl) — provides parcel structure
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_freshen
- ₢AAABH (typed kit registry) — COMPLETE, provides vofc_Kit

## Files

- Tools/vok/src/vorm_main.rs — add vvx_emplace subcommand
- Tools/vok/vof/src/vofm_managed.rs — NEW: hardcoded CLAUDE.md templates

**[260117-1333] rough**

Implement vvx_emplace: the Rust utility that copies kit assets from parcel to target repo.

## Layered Architecture

**vvi_install.sh (bash orchestration):**
1. Parse args — target repo path
2. Detect platform, select vvx binary
3. Verify target is git repo with BURC
4. Pre-install snapshot — `git commit` if working tree dirty
5. Call `vvx_emplace --parcel . --target <repo>`
6. Post-install commit — `git commit -m "VVK install: {hallmark}"`
7. Diff analysis — compare pre/post, invoke Claude for recovery guidance

**vvx_emplace (Rust, this pace):**
Pure file operations, no git. Called by vvi_install.sh.

## vvx_emplace --parcel <dir> --target <repo>

Inputs:
- `--parcel` — extracted parcel directory (contains vvbf_brand.json, kits/)
- `--target` — target repository root

Behavior:

1. **Read brand file**
   - Parse `{parcel}/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`

2. **Validate target**
   - REQUIRE `.buk/burc.env` exists
   - Parse BURC_TOOLS_DIR

3. **Cleanup by prefix** (optional, controlled by flag)
   - For each kit cipher, remove existing files matching prefix from:
     - `${BURC_TOOLS_DIR}/{kit}/`
     - `.claude/commands/{cipher}c-*`
     - `.claude/hooks/{cipher}h_*`
   - Ensures clean slate before copying

4. **Copy kit assets**
   - For each kit in `{parcel}/kits/`:
     - Copy directory to `${BURC_TOOLS_DIR}/{kit}/`
     - Preserve internal structure

5. **Route special assets**
   - Commands (`{cipher}c-*.md`): copy to `.claude/commands/`
   - Hooks (`{cipher}h_*`): copy to `.claude/hooks/`

6. **Freshen CLAUDE.md** (MVP: hardcoded templates)
   - Template content is hardcoded in Rust (not read from parcel files)
   - Each kit's CLAUDE.md section defined in vofc_registry or adjacent module
   - Call voff_freshen() with sections for installed kits
   - Write updated CLAUDE.md
   - **Future**: Whisper/Conclave will externalize templates to vov_veiled files

7. **Copy brand file**
   - Copy `{parcel}/vvbf_brand.json` to `.vvk/vvbf_brand.json`
   - Create `.vvk/` directory if needed

Output: JSON summary
```json
{
  "hallmark": 1000,
  "kits_installed": ["buk", "cmk", "jjk", "vvk"],
  "files_copied": 47,
  "commands_routed": 12,
  "claude_sections_updated": ["BUK", "CMK", "JJK", "VVK"]
}
```

## MVP Shortcut: Hardcoded CLAUDE.md Templates

For MVP, each kit's CLAUDE.md content is defined in Rust:

```rust
// In vofc_registry.rs or new vofm_managed.rs
pub const BUK_CLAUDE_SECTION: &str = r#"
## Bash Utility Kit (BUK)
...content...
"#;

pub const JJK_CLAUDE_SECTION: &str = r#"
## Job Jockey Configuration
...content...
"#;
```

This means:
- Changing CLAUDE.md content requires Rust rebuild + new release
- Acceptable for MVP; Whisper/Conclave will externalize later
- Keep sections minimal — just essential config, not full docs

## Key Design Points

- **No git operations** — bash handles all commits
- **Idempotent** — can re-run safely (cleanup + copy)
- **Platform-agnostic** — any vvx binary can emplace all kits
- **Parcel-relative** — reads kit files from parcel, templates from binary

## Prerequisites

- ₢AAAAE (vvx-release-impl) — provides parcel structure
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_freshen
- ₢AAABH (typed kit registry) — COMPLETE, provides vofc_Kit

## Files

- Tools/vok/src/vorm_main.rs — add vvx_emplace subcommand
- Tools/vok/vof/src/vofm_managed.rs — NEW: hardcoded CLAUDE.md templates
- Tools/vvk/vvi_install.sh — bash orchestration (spec'd in ₢AAAAE)

**[260117-1331] rough**

Implement vvx_emplace: the Rust utility that copies kit assets from parcel to target repo.

## Layered Architecture

**vvi_install.sh (bash orchestration):**
1. Parse args — target repo path
2. Detect platform, select vvx binary
3. Verify target is git repo with BURC
4. Pre-install snapshot — `git commit` if working tree dirty
5. Call `vvx_emplace --parcel . --target <repo>`
6. Post-install commit — `git commit -m "VVK install: {hallmark}"`
7. Diff analysis — compare pre/post, invoke Claude for recovery guidance

**vvx_emplace (Rust, this pace):**
Pure file operations, no git. Called by vvi_install.sh.

## vvx_emplace --parcel <dir> --target <repo>

Inputs:
- `--parcel` — extracted parcel directory (contains vvbf_brand.json, kits/)
- `--target` — target repository root

Behavior:

1. **Read brand file**
   - Parse `{parcel}/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`

2. **Validate target**
   - REQUIRE `.buk/burc.env` exists
   - Parse BURC_TOOLS_DIR

3. **Cleanup by prefix** (optional, controlled by flag)
   - For each kit cipher, remove existing files matching prefix from:
     - `${BURC_TOOLS_DIR}/{kit}/`
     - `.claude/commands/{cipher}c-*`
     - `.claude/hooks/{cipher}h_*`
   - Ensures clean slate before copying

4. **Copy kit assets**
   - For each kit in `{parcel}/kits/`:
     - Copy directory to `${BURC_TOOLS_DIR}/{kit}/`
     - Preserve internal structure

5. **Route special assets**
   - Commands (`{cipher}c-*.md`): copy to `.claude/commands/`
   - Hooks (`{cipher}h_*`): copy to `.claude/hooks/`

6. **Freshen CLAUDE.md**
   - Read kit CLAUDE.md templates from `{parcel}/kits/{kit}/voc{cipher}mc_*.md`
   - Call voff_freshen() with collected sections
   - Write updated CLAUDE.md

7. **Copy brand file**
   - Copy `{parcel}/vvbf_brand.json` to `.vvk/vvbf_brand.json`
   - Create `.vvk/` directory if needed

Output: JSON summary
```json
{
  "hallmark": 1000,
  "kits_installed": ["buk", "cmk", "jjk", "vvk"],
  "files_copied": 47,
  "commands_routed": 12,
  "claude_sections_updated": ["BUK", "CMK", "JJK", "VVK"]
}
```

## Key Design Points

- **No git operations** — bash handles all commits
- **Idempotent** — can re-run safely (cleanup + copy)
- **Platform-agnostic** — any vvx binary can emplace all kits
- **Parcel-relative** — reads from parcel filesystem, not embedded content

## Open Question

**Cleanup strategy**: Current spec uses prefix-based cleanup before copy. Alternative: track installed files in manifest for precise diff. Prefix cleanup is simpler and aligns with vosk_prefix_validation premise.

## Prerequisites

- ₢AAAAE (vvx-release-impl) — provides parcel structure
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_freshen
- ₢AAABH (typed kit registry) — COMPLETE, provides vofc_Kit

## Files

- Tools/vok/src/vorm_main.rs — add vvx_emplace subcommand
- Tools/vvk/vvi_install.sh — already spec'd in ₢AAAAE, calls vvx_emplace

**[260115-1247] rough**

Implement vvx install subcommand in Rust.

Run from extracted archive directory:
```bash
./bin/vvx-darwin-arm64 install --target /path/to/repo
```

The 7-step process:
1. **Pre-install snapshot** — git commit if working tree dirty
2. **Copy kit assets** — Read from archive `kits/` directory, write to install_path locations in target
3. **Copy platform binaries** — Copy ALL sibling binaries from archive `bin/` to target `Tools/vvk/bin/`
4. **Freshen CLAUDE.md** — Managed section markers (depends on claude-md-freshening pace)
5. **Cleanup obsolete** — Remove files no longer in current release
6. **Post-install commit** — git commit with version and kit list
7. **Diff analysis** — Find previous install, diff, invoke Claude for recovery guidance

Key: Install reads kit assets from archive filesystem, NOT from embedded content. Binary locates its archive context via path relative to itself.

Install is platform-agnostic: any platform binary can install everything.

Depends on: kit-asset-registry, claude-md-freshening, version-manifest paces.

**[260114-1058] rough**

Implement vvx install subcommand in Rust.

The 6-step process (see paddock Architecture section):
1. Pre-install snapshot - git commit if working tree dirty
2. Extract assets - write embedded kit files to install_path locations
3. Freshen CLAUDE.md - managed section markers (depends on claude-md-freshening pace)
4. Cleanup obsolete - remove files no longer in current release
5. Post-install commit - git commit with version and kit list
6. Diff analysis - find previous install, diff, invoke Claude for recovery guidance

Git commit message formats:
- Pre: [vvx:pre-install] Snapshot before {version}
- Post: [vvx:install:{version}] {kit-list}

Depends on: kit-asset-registry, claude-md-freshening, version-manifest paces.

### jjk-rcg-phase2-callsites (₢AAAAa) [abandoned]

**[260116-1421] abandoned**

Superseded by ₢AAAAY which completed all RCG prefixing in one pass.

**[260116-1000] rough**

JJK RCG Phase 2: Cross-file call site updates

## Prerequisite

Phase 1 complete - all declarations prefixed, manifests collected.

## Approach

Using rename manifests from Phase 1, update all cross-file references:
- jjrx_cli.rs uses types from jjrg, jjrf, jjrq, jjrs, jjrn
- jjrq_query.rs uses types from jjrg, jjrf
- etc.

## Method

For each rename mapping (old -> new) from Phase 1:
- replace_all across all 7 source files
- Order: types first, then functions, then constants

## Verification

After all replacements, cargo build must pass (may have errors to fix).

## Output

All call sites updated. Ready for Phase 3 coordination.

### jjk-rcg-phase3-finalize (₢AAAAb) [abandoned]

**[260116-1421] abandoned**

Superseded by ₢AAAAY which completed all RCG prefixing in one pass.

**[260116-1000] rough**

JJK RCG Phase 3: Finalize and verify

## Prerequisite

Phase 2 complete - all call sites updated.

## Tasks

1. Add crate boilerplate: #![allow(non_camel_case_types)] to lib.rs
2. Update lib.rs re-exports to use new prefixed names
3. Run cargo build --features jjk - fix any errors
4. Run cargo test - verify tests pass
5. Commit all changes

## Test extraction (deferred)

Test file separation (jjt*.rs pattern) is NOT in scope. Deferred to separate pace.

## Verification

cargo build and cargo test both pass.

### rein-filter-by-identity (₢AAAAp) [complete]

**[260116-1334] complete**

Refactored jjx_rein to filter by firemark/coronet identity instead of brand. Removed --brand CLI arg. Updated grep pattern to ^jjb:[^:]+:(₣XX|₢XX). All 133 tests pass. Rein now works without brand parameter.

**[260116-1330] bridled**

Refactor jjx_rein to filter by firemark/coronet identity rather than brand.

## Changes

1. **Grep pattern**: Change from `^jjb:BRAND:(₣XX|₢XX)` to `^jjb:[^:]+:(₣XX|₢XX)` - match any brand, filter by identity
2. **CLI args**: Remove `--brand` argument from ReinArgs
3. **Internal calls**: Update run_retire() to not need brand for get_entries()

## Rationale

The firemark/coronet already uniquely identifies heat membership:
- ₢AAAAk → first 2 chars after prefix = AA = parent heat
- ₣AA → direct heat identity

Brand is orthogonal - it identifies which installation made the commit, not which heat it belongs to.

## Files

- Tools/jjk/veiled/src/jjrs_steeplechase.rs (grep pattern, ReinArgs)
- Tools/jjk/veiled/src/jjrx_cli.rs (CLI ReinArgs, run_retire)
- Tools/jjk/JJD-GallopsData.adoc (update jjdo_rein spec)

## Unblocks

- /jjc-heat-rein slash command (no longer needs installation-identifier)
- Cleaner retire flow

*Direction:* Agent: sonnet. Strategy: 1) Update grep pattern in jjrs_steeplechase.rs from ^jjb:BRAND: to ^jjb:[^:]+: to match any brand. 2) Remove brand field from ReinArgs struct in jjrs_steeplechase.rs. 3) Remove --brand arg from CLI ReinArgs in jjrx_cli.rs. 4) Update run_retire() to not pass brand to get_entries(). 5) Update JJD jjdo_rein spec removing --brand argument. 6) Build and run tests. Key files: jjrs_steeplechase.rs, jjrx_cli.rs, JJD-GallopsData.adoc

**[260116-1327] rough**

Refactor jjx_rein to filter by firemark/coronet identity rather than brand.

## Changes

1. **Grep pattern**: Change from `^jjb:BRAND:(₣XX|₢XX)` to `^jjb:[^:]+:(₣XX|₢XX)` - match any brand, filter by identity
2. **CLI args**: Remove `--brand` argument from ReinArgs
3. **Internal calls**: Update run_retire() to not need brand for get_entries()

## Rationale

The firemark/coronet already uniquely identifies heat membership:
- ₢AAAAk → first 2 chars after prefix = AA = parent heat
- ₣AA → direct heat identity

Brand is orthogonal - it identifies which installation made the commit, not which heat it belongs to.

## Files

- Tools/jjk/veiled/src/jjrs_steeplechase.rs (grep pattern, ReinArgs)
- Tools/jjk/veiled/src/jjrx_cli.rs (CLI ReinArgs, run_retire)
- Tools/jjk/JJD-GallopsData.adoc (update jjdo_rein spec)

## Unblocks

- /jjc-heat-rein slash command (no longer needs installation-identifier)
- Cleaner retire flow

### mcm-prefix-tree-declaration (₢AAAA-) [abandoned]

**[260117-1243] abandoned**

Design and implement MCM prefix-tree declaration pattern for organizing category declarations in mapping sections.

## Status: ABANDONED

Superseded by ₢AAABI (vof-prefix-tree-checker).

## Reason

MCM-based documentation approach rejected because:
- Requires Claude to parse/remember prefix trees each session (context window cost)
- No enforcement — hope-based compliance  
- Violations discovered late (during review) not early (at build time)

## Replacement

₢AAABI implements prefix tree registry in vof with `vvx check` utility for build-time enforcement. Single source of truth, zero context cost, CI integration.

**[260117-1048] rough**

Design and implement MCM prefix-tree declaration pattern for organizing category declarations in mapping sections.

PROBLEM:
Current flat comment lists (like VOS lines 7-39) dont capture prefix hierarchy. As vocabularies grow, this becomes unwieldy and error-prone. The tree structure is implicit.

GOALS:
1. Explicit hierarchical representation of prefix trees
2. Machine-parseable for VVX validation
3. Human-readable in source
4. Compatible with existing MCM mapping section conventions

DESIGN OPTIONS TO EVALUATE:

Option A - Indented comments:
```
// vosl:   Liturgy (non-terminal)
//   voslc_: Cipher
//   vosld:  Domain (non-terminal)
//     vosldr_: Rust
```

Option B - Tree annotation tags:
```
// prefix-tree::
//   vosl [Liturgy]
//     voslc_ [Cipher]
//     vosld [Domain]
//       vosldr_ [Rust]
// end-prefix-tree::
```

Option C - Grouping tags (simpler):
```
// group:: Liturgy (vosl)
// voslc_: Cipher
// vosls_: Signet
// end-group::
```

CONSIDERATIONS:
- Terminal vs non-terminal distinction (trailing _ vs no _)
- Validation: can VVX check tree matches actual attribute definitions?
- Migration path for existing documents
- Interaction with AXLA category patterns

IMPLEMENTATION:
1. Survey MCM-MetaConceptModel.adoc for current conventions
2. Design chosen syntax with examples
3. Update MCM spec with new pattern
4. Migrate VOS as proof-of-concept
5. Document in CMK README

FILES:
- Tools/cmk/MCM-MetaConceptModel.adoc (spec update)
- Tools/vok/VOS-VoxObscuraSpec.adoc (migration)
- Tools/cmk/AXLA-Lexicon.adoc (potential migration)

### voi-directory-relocation (₢AAABG) [abandoned]

**[260117-1241] abandoned**

Relocate Tools/voi/ to Tools/vok/voi/ per prefix naming discipline.

## Rationale

The `voi` crate uses `vo` prefix (VOK namespace) and is VOK's infrastructure crate. Per prefix naming discipline, it belongs under `Tools/vok/` not as a sibling.

## Tasks

1. Move `Tools/voi/` → `Tools/vok/voi/`
2. Update `Tools/vok/Cargo.toml`: change `voi = { path = "../voi" }` to `voi = { path = "voi" }`
3. Verify build: `tt/vow-b.Build.sh`
4. Update any documentation referencing `Tools/voi/`

## Precedent

JJK uses `Tools/jjk/veiled/` for its nested Rust crate.

## Not MVP

This is organizational cleanup, not blocking release/install functionality.

**[260117-1207] rough**

Relocate Tools/voi/ to Tools/vok/voi/ per prefix naming discipline.

## Rationale

The `voi` crate uses `vo` prefix (VOK namespace) and is VOK's infrastructure crate. Per prefix naming discipline, it belongs under `Tools/vok/` not as a sibling.

## Tasks

1. Move `Tools/voi/` → `Tools/vok/voi/`
2. Update `Tools/vok/Cargo.toml`: change `voi = { path = "../voi" }` to `voi = { path = "voi" }`
3. Verify build: `tt/vow-b.Build.sh`
4. Update any documentation referencing `Tools/voi/`

## Precedent

JJK uses `Tools/jjk/veiled/` for its nested Rust crate.

## Not MVP

This is organizational cleanup, not blocking release/install functionality.

### vvx-vacate-impl (₢AAABJ) [complete]

**[260118-0735] complete**

Implemented vvx_vacate: vofe_vacate() with pattern-based command/hook removal, voff_collapse integration, kit dir deletion, brand cleanup. Added vvu_uninstall.sh orchestration. Build passes, 25 tests pass.

**[260118-0732] bridled**

Implement vvx_vacate: the Rust utility that removes kit assets from target repo.

## Layered Architecture

**vvu_uninstall.sh (bash orchestration):**
1. Source `.buk/burc.env` from current directory
2. Verify .vvk/vvbf_brand.json exists (something is installed)
3. Pre-uninstall snapshot — `git commit` if working tree dirty
4. Call `vvx_vacate --burc .buk/burc.env`
5. Post-uninstall commit — `git commit -m "VVK uninstall"`

**vvx_vacate (Rust, this pace):**
Pure file operations, no git. Called by vvu_uninstall.sh.

## vvx_vacate --burc <path>

Inputs:
- `--burc` — path to target's burc.env file

Behavior:

1. **Parse BURC**
   - Read and parse burc.env file
   - Extract BURC_TOOLS_DIR, BURC_PROJECT_ROOT
   - FATAL if file missing or variables undefined

2. **Read brand file**
   - Parse `${BURC_PROJECT_ROOT}/.vvk/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`
   - Extract cipher for each kit (from vofc_registry)

3. **Remove commands**
   - For each kit cipher: delete `${BURC_PROJECT_ROOT}/.claude/commands/{cipher}c-*`

4. **Remove hooks**
   - For each kit cipher: delete `${BURC_PROJECT_ROOT}/.claude/hooks/{cipher}h_*`

5. **Collapse CLAUDE.md sections**
   - For each kit's managed section tag (from vofm_managed.rs constants):
     - Call voff_collapse() to replace BEGIN/END with UNINSTALLED marker
   - Preserves user's section ordering for future reinstall

6. **Remove kit directories**
   - For each kit in brand: delete `${BURC_TOOLS_DIR}/{kit}/`

7. **Remove brand file**
   - Delete `${BURC_PROJECT_ROOT}/.vvk/vvbf_brand.json`
   - Remove `.vvk/` directory if empty

Output: JSON summary
```json
{
  "kits_removed": ["buk", "cmk", "jjk", "vvk"],
  "files_deleted": 47,
  "commands_removed": 12,
  "claude_sections_collapsed": ["BUK", "CMK", "JJK", "VVK"]
}
```

## Key Design Points

- **No git operations** — bash handles all commits
- **Brand-driven** — only removes what brand file says was installed
- **Preserves CLAUDE.md structure** — collapses to UNINSTALLED markers, not deletion
- **Clean removal** — no orphaned files if prefix validation was enforced at install
- **BURC-driven** — all paths derived from burc.env

## MVP Shortcut: Hardcoded Section Tags

Like vvx_emplace, the managed section tags are hardcoded in Rust:
- BUK, CMK, JJK, VVK (matching vofm_managed.rs constants)
- Post-MVP: ₢AAABK will provide tag registry via template files

## Prerequisites

- ₢AAAAF (vvx_emplace) — parallel implementation, shares patterns
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_collapse

## Files

- Tools/vok/src/vorm_main.rs — add vvx_vacate subcommand
- Tools/vvk/vvu_uninstall.sh — NEW: bash orchestration script (this IS a kit asset, installs to target)

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vofe_emplace.rs, lib.rs, vorm_main.rs, vvu_uninstall.sh (4 files)
Steps:
1. Add vofe_VacateArgs, vofe_VacateResult types and vofe_vacate() function to vofe_emplace.rs
2. Implement: parse burc, read brand from .vvk/, remove commands/hooks by pattern, collapse CLAUDE.md sections, delete kit dirs, delete brand
3. Update lib.rs to export vacate types
4. Add vvx_vacate subcommand to vorm_main.rs with --burc arg
5. Create Tools/vvk/vvu_uninstall.sh bash orchestration script
Verify: ./tt/vow-b.Build.sh

**[260117-1355] rough**

Implement vvx_vacate: the Rust utility that removes kit assets from target repo.

## Layered Architecture

**vvu_uninstall.sh (bash orchestration):**
1. Source `.buk/burc.env` from current directory
2. Verify .vvk/vvbf_brand.json exists (something is installed)
3. Pre-uninstall snapshot — `git commit` if working tree dirty
4. Call `vvx_vacate --burc .buk/burc.env`
5. Post-uninstall commit — `git commit -m "VVK uninstall"`

**vvx_vacate (Rust, this pace):**
Pure file operations, no git. Called by vvu_uninstall.sh.

## vvx_vacate --burc <path>

Inputs:
- `--burc` — path to target's burc.env file

Behavior:

1. **Parse BURC**
   - Read and parse burc.env file
   - Extract BURC_TOOLS_DIR, BURC_PROJECT_ROOT
   - FATAL if file missing or variables undefined

2. **Read brand file**
   - Parse `${BURC_PROJECT_ROOT}/.vvk/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`
   - Extract cipher for each kit (from vofc_registry)

3. **Remove commands**
   - For each kit cipher: delete `${BURC_PROJECT_ROOT}/.claude/commands/{cipher}c-*`

4. **Remove hooks**
   - For each kit cipher: delete `${BURC_PROJECT_ROOT}/.claude/hooks/{cipher}h_*`

5. **Collapse CLAUDE.md sections**
   - For each kit's managed section tag (from vofm_managed.rs constants):
     - Call voff_collapse() to replace BEGIN/END with UNINSTALLED marker
   - Preserves user's section ordering for future reinstall

6. **Remove kit directories**
   - For each kit in brand: delete `${BURC_TOOLS_DIR}/{kit}/`

7. **Remove brand file**
   - Delete `${BURC_PROJECT_ROOT}/.vvk/vvbf_brand.json`
   - Remove `.vvk/` directory if empty

Output: JSON summary
```json
{
  "kits_removed": ["buk", "cmk", "jjk", "vvk"],
  "files_deleted": 47,
  "commands_removed": 12,
  "claude_sections_collapsed": ["BUK", "CMK", "JJK", "VVK"]
}
```

## Key Design Points

- **No git operations** — bash handles all commits
- **Brand-driven** — only removes what brand file says was installed
- **Preserves CLAUDE.md structure** — collapses to UNINSTALLED markers, not deletion
- **Clean removal** — no orphaned files if prefix validation was enforced at install
- **BURC-driven** — all paths derived from burc.env

## MVP Shortcut: Hardcoded Section Tags

Like vvx_emplace, the managed section tags are hardcoded in Rust:
- BUK, CMK, JJK, VVK (matching vofm_managed.rs constants)
- Post-MVP: ₢AAABK will provide tag registry via template files

## Prerequisites

- ₢AAAAF (vvx_emplace) — parallel implementation, shares patterns
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_collapse

## Files

- Tools/vok/src/vorm_main.rs — add vvx_vacate subcommand
- Tools/vvk/vvu_uninstall.sh — NEW: bash orchestration script (this IS a kit asset, installs to target)

**[260117-1333] rough**

Implement vvx_vacate: the Rust utility that removes kit assets from target repo.

## Layered Architecture

**vvu_uninstall.sh (bash orchestration):**
1. Verify current directory is git repo with BURC
2. Verify .vvk/vvbf_brand.json exists (something is installed)
3. Pre-uninstall snapshot — `git commit` if working tree dirty
4. Call `vvx_vacate --target .`
5. Post-uninstall commit — `git commit -m "VVK uninstall"`

**vvx_vacate (Rust, this pace):**
Pure file operations, no git. Called by vvu_uninstall.sh.

## vvx_vacate --target <repo>

Inputs:
- `--target` — target repository root (usually `.`)

Behavior:

1. **Read brand file**
   - Parse `.vvk/vvbf_brand.json`
   - Extract kit list from `vvbk_kits`
   - Extract cipher for each kit

2. **Remove commands**
   - For each kit cipher: delete `.claude/commands/{cipher}c-*`

3. **Remove hooks**
   - For each kit cipher: delete `.claude/hooks/{cipher}h_*`

4. **Collapse CLAUDE.md sections**
   - For each kit's managed section tag:
     - Call voff_collapse() to replace BEGIN/END with UNINSTALLED marker
   - Preserves user's section ordering for future reinstall

5. **Remove kit directories**
   - For each kit in brand: delete `${BURC_TOOLS_DIR}/{kit}/`

6. **Remove brand file**
   - Delete `.vvk/vvbf_brand.json`
   - Remove `.vvk/` directory if empty

Output: JSON summary
```json
{
  "kits_removed": ["buk", "cmk", "jjk", "vvk"],
  "files_deleted": 47,
  "commands_removed": 12,
  "claude_sections_collapsed": ["BUK", "CMK", "JJK", "VVK"]
}
```

## Key Design Points

- **No git operations** — bash handles all commits
- **Brand-driven** — only removes what brand file says was installed
- **Preserves CLAUDE.md structure** — collapses to UNINSTALLED markers, not deletion
- **Clean removal** — no orphaned files if prefix validation was enforced at install

## MVP Shortcut: Hardcoded Section Tags

Like vvx_emplace, the managed section tags are hardcoded in Rust:
- BUK, CMK, JJK, VVK (matching vofm_managed.rs constants)
- Future: Whisper/Conclave will provide tag registry

## Prerequisites

- ₢AAAAF (vvx_emplace) — parallel implementation, shares patterns
- ₢AAAAG (claude-md-freshening) — COMPLETE, provides voff_collapse

## Files

- Tools/vok/src/vorm_main.rs — add vvx_vacate subcommand
- Tools/vvk/vvu_uninstall.sh — NEW: bash orchestration script

### managed-section-templates (₢AAABK) [complete]

**[260118-1126] complete**

Externalized CLAUDE.md templates to vov_veiled/ files. Registry extended with managed_sections. Release/emplace/vacate updated to use template files.

**[260118-0737] bridled**

Externalize CLAUDE.md templates from hardcoded Rust to editable markdown files.

## Goal

Replace MVP hardcoding with template files in vov_veiled/, enabling CLAUDE.md content iteration without Rust rebuilds.

## Registry Extension (vofc_registry.rs)

Extend vofc_Kit with managed section declarations:

```rust
pub struct vofc_Kit {
    pub cipher: &'static vofc_Cipher,
    pub display_name: &'static str,
    pub managed_sections: &'static [vofc_ManagedSection],
}

pub struct vofc_ManagedSection {
    pub tag: &'static str,           // "JJK", "BUK"
    pub template_path: &'static str, // "vocjjmc_core.md" (relative to kit's vov_veiled)
}

pub const DISTRIBUTABLE_KITS: &[vofc_Kit] = &[
    vofc_Kit {
        cipher: &BU,
        display_name: "Bash Utilities",
        managed_sections: &[
            vofc_ManagedSection { tag: "BUK", template_path: "vocbumc_core.md" },
        ],
    },
    vofc_Kit {
        cipher: &JJ,
        display_name: "Job Jockey", 
        managed_sections: &[
            vofc_ManagedSection { tag: "JJK", template_path: "vocjjmc_core.md" },
        ],
    },
    // ... etc
];
```

## Template Files to Create

Each kit gets a template file in its vov_veiled/:

- Tools/buk/vov_veiled/vocbumc_core.md — BUK CLAUDE.md section
- Tools/cmk/vov_veiled/voccmmc_core.md — CMK CLAUDE.md section  
- Tools/jjk/vov_veiled/vocjjmc_core.md — JJK CLAUDE.md section
- Tools/vvk/vov_veiled/vocvvmc_core.md — VVK CLAUDE.md section

Content: Extract from current CLAUDE.md managed sections (the "## Job Jockey Configuration" etc.)

## Update release_collect

Modify to read templates:
1. For each kit in DISTRIBUTABLE_KITS:
2. For each managed_section in kit:
3. Read template from `{kit_dir}/vov_veiled/{template_path}`
4. Copy to parcel at `kits/{kit_id}/templates/{template_path}`

## Update vvx_emplace  

Modify to read from parcel:
1. For each kit being installed:
2. For each managed_section in kit's registry entry:
3. Read template from `{parcel}/kits/{kit_id}/templates/{template_path}`
4. Build voff_ManagedSection with tag + content
5. Call voff_freshen()

## Update vvx_vacate

Modify to use registry for tags:
1. For each kit in brand file:
2. Look up kit in DISTRIBUTABLE_KITS
3. Collect all managed_section.tag values
4. Call voff_collapse() with those tags

## Delete Hardcoded Shortcuts

Remove vofm_managed.rs (or never create it) — templates are the source of truth.

## Verification

1. Edit a template file
2. Run release (no Rust rebuild needed)
3. Install to test repo
4. Verify CLAUDE.md has updated content

## Prerequisites

- ₢AAAAE (release_collect exists to modify)
- ₢AAAAF (vvx_emplace exists to modify)
- ₢AAABJ (vvx_vacate exists to modify)

*Direction:* Agent: sonnet
Cardinality: 1 sequential
Files: vocbumc_core.md, vocvvmc_core.md, vofc_registry.rs, vofr_release.rs, vofe_emplace.rs, vofm_managed.rs, lib.rs (7 files)
Steps:
1. Create Tools/buk/vov_veiled/vocbumc_core.md with BUK template content
2. Create Tools/vvk/vov_veiled/ directory and vocvvmc_core.md with VVK template content
3. Extend vofc_Kit struct with managed_sections field, add vofc_ManagedSection struct
4. Update DISTRIBUTABLE_KITS to include managed_sections for each kit
5. Update vofr_release.rs release_collect to copy templates to parcel/kits/{kit}/templates/
6. Update vofe_emplace.rs to read templates from parcel instead of vofm_managed.rs
7. Update vofe_emplace.rs vacate to use registry for section tags
8. Remove vofm_managed.rs and its exports from lib.rs
Verify: ./tt/vow-b.Build.sh

**[260117-1337] rough**

Externalize CLAUDE.md templates from hardcoded Rust to editable markdown files.

## Goal

Replace MVP hardcoding with template files in vov_veiled/, enabling CLAUDE.md content iteration without Rust rebuilds.

## Registry Extension (vofc_registry.rs)

Extend vofc_Kit with managed section declarations:

```rust
pub struct vofc_Kit {
    pub cipher: &'static vofc_Cipher,
    pub display_name: &'static str,
    pub managed_sections: &'static [vofc_ManagedSection],
}

pub struct vofc_ManagedSection {
    pub tag: &'static str,           // "JJK", "BUK"
    pub template_path: &'static str, // "vocjjmc_core.md" (relative to kit's vov_veiled)
}

pub const DISTRIBUTABLE_KITS: &[vofc_Kit] = &[
    vofc_Kit {
        cipher: &BU,
        display_name: "Bash Utilities",
        managed_sections: &[
            vofc_ManagedSection { tag: "BUK", template_path: "vocbumc_core.md" },
        ],
    },
    vofc_Kit {
        cipher: &JJ,
        display_name: "Job Jockey", 
        managed_sections: &[
            vofc_ManagedSection { tag: "JJK", template_path: "vocjjmc_core.md" },
        ],
    },
    // ... etc
];
```

## Template Files to Create

Each kit gets a template file in its vov_veiled/:

- Tools/buk/vov_veiled/vocbumc_core.md — BUK CLAUDE.md section
- Tools/cmk/vov_veiled/voccmmc_core.md — CMK CLAUDE.md section  
- Tools/jjk/vov_veiled/vocjjmc_core.md — JJK CLAUDE.md section
- Tools/vvk/vov_veiled/vocvvmc_core.md — VVK CLAUDE.md section

Content: Extract from current CLAUDE.md managed sections (the "## Job Jockey Configuration" etc.)

## Update release_collect

Modify to read templates:
1. For each kit in DISTRIBUTABLE_KITS:
2. For each managed_section in kit:
3. Read template from `{kit_dir}/vov_veiled/{template_path}`
4. Copy to parcel at `kits/{kit_id}/templates/{template_path}`

## Update vvx_emplace  

Modify to read from parcel:
1. For each kit being installed:
2. For each managed_section in kit's registry entry:
3. Read template from `{parcel}/kits/{kit_id}/templates/{template_path}`
4. Build voff_ManagedSection with tag + content
5. Call voff_freshen()

## Update vvx_vacate

Modify to use registry for tags:
1. For each kit in brand file:
2. Look up kit in DISTRIBUTABLE_KITS
3. Collect all managed_section.tag values
4. Call voff_collapse() with those tags

## Delete Hardcoded Shortcuts

Remove vofm_managed.rs (or never create it) — templates are the source of truth.

## Verification

1. Edit a template file
2. Run release (no Rust rebuild needed)
3. Install to test repo
4. Verify CLAUDE.md has updated content

## Prerequisites

- ₢AAAAE (release_collect exists to modify)
- ₢AAAAF (vvx_emplace exists to modify)
- ₢AAABJ (vvx_vacate exists to modify)

### test-mvp-round-trip (₢AAABL) [complete]

**[260118-1552] complete**

Validated full release/install/uninstall/reinstall cycle on pb_paneboard02. Binary integrity verified, 23 commands routed, CLAUDE.md markers work. Issue noted: vvu_uninstall.sh requires CWD fix.

**[260117-1341] rough**

Validate MVP release/install/uninstall cycle on PaneBoard (neighbor project).

## Purpose

Human-guided validation that the 4 MVP paces work end-to-end on a real target repo before evolving architecture.

## Prerequisites

All must be complete:
- ₢AAAAE (release) — can create parcel
- ₢AAAAF (emplace) — can install to target
- ₢AAABJ (vacate) — can uninstall
- ₢AAABK (templates) — CLAUDE.md content is externalized

## Test Environment

- **Kit forge**: rbm_alpha_recipemuster (this repo)
- **Target repo**: PaneBoard (../pb_PaneBoard or similar)
- **Requirement**: PaneBoard must have BURC configured (.buk/burc.env)

## Test Script

### 1. Create Release

```bash
# From kit forge
tt/vow-R.Release.sh
# Expect: vvk-parcel-{hallmark}.tar.gz created
# Verify: tarball contains vvi_install.sh, vvbf_brand.json, kits/
```

### 2. Stage for Install

```bash
# Copy parcel to target repo area
cp vvk-parcel-*.tar.gz ../pb_PaneBoard/
cd ../pb_PaneBoard
tar -xzf vvk-parcel-*.tar.gz
cd vvk-parcel-*
```

### 3. Fresh Install

```bash
./vvi_install.sh ..
# Expect: Pre-install commit (if dirty), files copied, CLAUDE.md freshened, post-install commit
# Verify:
#   - Tools/buk/, Tools/jjk/, etc. exist
#   - .claude/commands/ has jjc-*.md files
#   - CLAUDE.md has <!-- MANAGED:JJK:BEGIN --> sections
#   - .vvk/vvbf_brand.json exists
```

### 4. Verify Functionality

```bash
# Test that installed tools work
cd ..
./Tools/vvk/bin/vvx --version
# Should work and show version
```

### 5. Uninstall

```bash
./Tools/vvk/vvu_uninstall.sh
# Expect: Files removed, CLAUDE.md sections collapsed, commit created
# Verify:
#   - Tools/buk/, Tools/jjk/ removed
#   - .claude/commands/jjc-*.md removed
#   - CLAUDE.md has <!-- MANAGED:JJK:UNINSTALLED --> markers
#   - .vvk/vvbf_brand.json removed
```

### 6. Reinstall (UNINSTALLED marker test)

```bash
cd vvk-parcel-*
./vvi_install.sh ..
# Expect: UNINSTALLED markers expand back to BEGIN/END at same positions
# Verify: User content between sections preserved, section order preserved
```

### 7. Cleanup

```bash
cd ..
rm -rf vvk-parcel-*
# Optionally uninstall again to leave PaneBoard clean
```

## Success Criteria

- [ ] Release creates valid parcel
- [ ] Install deploys all kits
- [ ] CLAUDE.md sections present and correct
- [ ] Installed tools functional
- [ ] Uninstall removes cleanly
- [ ] UNINSTALLED markers preserve positions
- [ ] Reinstall expands markers correctly

## Failure Handling

Document any failures as issues. May spawn fix paces before proceeding to whisper-conclave work.

## Completion

Mark complete when round-trip validated. No code changes in this pace — validation only.

## Steeplechase

### 2026-01-18 15:52 - ₢AAABL - W

Validated MVP round-trip on pb_paneboard02

### 2026-01-18 15:52 - Heat - T

test-mvp-round-trip

### 2026-01-18 15:18 - ₢AAABL - n

Fix claude asset collection path - use CWD not tools_dir

### 2026-01-18 15:06 - ₢AAABL - n

Pre-validation notch - claude asset conveyance implementation

### 2026-01-18 15:05 - ₢AAABL - A

Interactive validation: notch, release, stage, install to pb, test, uninstall, reinstall

### 2026-01-18 15:04 - ₢AAABR - W

Implemented Claude config asset conveyance - signet constants, release collection, emplace routing, vacate removal

### 2026-01-18 15:04 - Heat - T

implement-claude-assets-conveyance

### 2026-01-18 14:59 - ₢AAABR - F

Executing bridled pace via sonnet agent

### 2026-01-18 14:53 - Heat - T

implement-claude-assets-conveyance

### 2026-01-18 14:50 - ₢AAABQ - W

VOS updated with claude/ parcel structure, release collection by cipher pattern, install routing, and uninstall removal

### 2026-01-18 14:50 - Heat - T

vos-claude-assets-spec

### 2026-01-18 14:48 - ₢AAABQ - F

Executing bridled pace via sonnet agent

### 2026-01-18 14:47 - Heat - T

vos-claude-assets-spec

### 2026-01-18 14:46 - Heat - T

vos-claude-assets-spec

### 2026-01-18 14:45 - ₢AAABQ - A

Update VOS parcel/release/install/uninstall for claude/ asset dir

### 2026-01-18 14:42 - Heat - S

implement-claude-assets-conveyance

### 2026-01-18 14:41 - Heat - S

vos-claude-assets-spec

### 2026-01-18 14:23 - ₢AAABL - n

Add Gallops Functional Test section for JJK post-install validation

### 2026-01-18 14:22 - ₢AAABL - A

Sequential validation: release→stage→install→verify→uninstall→reinstall

### 2026-01-18 14:22 - ₢AAABL - A

Sequential validation: release→stage→install→verify→uninstall→reinstall

### 2026-01-18 13:28 - ₢AAABL - n

Update vvx operation contracts for BURC_MANAGED_KITS validation and git-aware commits

### 2026-01-18 13:27 - ₢AAABL - n

Add BURC_MANAGED_KITS requirement to VOS spec with defense-in-depth rationale

### 2026-01-18 13:24 - ₢AAABL - n

Clarify that vacated kit content remains functional outside Claude Code

### 2026-01-18 13:21 - ₢AAABL - n

Clarify VOS uninstall behavior: preserve kit directories while removing Claude integration

### 2026-01-18 13:18 - ₢AAABL - n

Preserve kit directories in VVK vacate operation; only remove vvx binaries

### 2026-01-18 13:11 - ₢AAABL - n

vovr: Register snapshot 1005

### 2026-01-18 13:05 - ₢AAABL - n

Add kit inventory management via BURC_MANAGED_KITS configuration

### 2026-01-18 12:55 - ₢AAABL - n

Add BURC_MANAGED_KITS configuration for kit release and install validation

### 2026-01-18 12:47 - ₢AAABL - n

Extract hash and commit capture functions; refactor platform detection

### 2026-01-18 12:36 - ₢AAABL - n

Document BURC_PROJECT_ROOT specification and update path resolution logic

### 2026-01-18 12:20 - ₢AAABL - n

Fix vvk iteration workflow: clarify tarball cleanup, install target parameter, and test steps

### 2026-01-18 12:13 - ₢AAABL - n

VVK emplace/vacate: move git operations to Rust, simplify bash bootstrap

### 2026-01-18 12:05 - ₢AAABL - A

Human-guided validation: execute 7-step test script interactively

### 2026-01-18 11:29 - ₢AAABL - A

Human-guided MVP round-trip validation on PaneBoard

### 2026-01-18 11:26 - ₢AAABK - W

Templates externalized to vov_veiled/; registry, release, emplace, vacate updated

### 2026-01-18 11:26 - Heat - T

managed-section-templates

### 2026-01-18 11:24 - ₢AAABK - F

Executing bridled pace via sonnet agent

### 2026-01-18 08:31 - Heat - n

Add BUD environment validation to vvx

### 2026-01-18 07:57 - Heat - S

paneboard-install-test

### 2026-01-18 07:48 - ₢AAABK - n

Externalize managed section templates to parcel; support autonomous execution via Task agents

### 2026-01-18 07:37 - ₢AAABK - F

Executing bridled pace

### 2026-01-18 07:37 - Heat - T

managed-section-templates

### 2026-01-18 07:35 - ₢AAABJ - W

vvx_vacate complete: pattern removal, voff_collapse, vvu_uninstall.sh orchestration

### 2026-01-18 07:35 - Heat - T

vvx-vacate-impl

### 2026-01-18 07:32 - ₢AAABJ - F

Executing bridled pace

### 2026-01-18 07:32 - Heat - T

vvx-vacate-impl

### 2026-01-18 07:30 - ₢AAAAF - W

vvx_emplace complete: 4 files, burc parser, kit copier, command/hook routing, freshen integration

### 2026-01-18 07:30 - Heat - T

vvx-install-impl

### 2026-01-18 07:26 - ₢AAAAF - F

Executing bridled pace

### 2026-01-18 07:26 - Heat - T

vvx-install-impl

### 2026-01-18 07:25 - ₢AAAAF - A

Four files: vofm_managed.rs (templates), vofe_emplace.rs (impl), vorm_main.rs (subcommand), lib.rs (exports)

### 2026-01-17 14:31 - ₢AAAAF - n

Clarify CLAUDE.md managed section sources with template mapping table

### 2026-01-17 14:30 - ₢AAAAF - n

Document CLAUDE.md managed sections and template update workflow

### 2026-01-17 14:27 - ₢AAAAF - n

Add notch to Quick Verbs table and document commit discipline for JJ

### 2026-01-17 14:11 - ₢AAAAE - W

Release implemented: vob_parcel, release_collect, release_brand. Hallmark 1000.

### 2026-01-17 14:10 - Heat - T

vvx-release-impl

### 2026-01-17 14:00 - ₢AAAAE - A

Sequential sonnet: vvi_install.sh, registry, release_collect, release_brand, vob_parcel, tabtarget

### 2026-01-17 13:55 - Heat - T

vvx-vacate-impl

### 2026-01-17 13:55 - Heat - T

vvx-install-impl

### 2026-01-17 13:54 - Heat - T

vvx-release-impl

### 2026-01-17 13:50 - Heat - S

tack-silks-and-commit-migration

### 2026-01-17 13:44 - Heat - r

moved ₢AAAAo to last

### 2026-01-17 13:44 - Heat - T

create-heat-rein-command

### 2026-01-17 13:44 - Heat - S

vos-implementation-reconciliation

### 2026-01-17 13:41 - Heat - S

whisper-conclave-lite

### 2026-01-17 13:41 - Heat - S

test-mvp-round-trip

### 2026-01-17 13:37 - Heat - S

managed-section-templates

### 2026-01-17 13:33 - Heat - S

vvx-vacate-impl

### 2026-01-17 13:33 - Heat - T

vvx-install-impl

### 2026-01-17 13:31 - Heat - T

vvx-install-impl

### 2026-01-17 13:28 - Heat - T

vvx-release-impl

### 2026-01-17 13:24 - Heat - T

vvx-release-impl

### 2026-01-17 13:18 - ₢AAABH - W

Typed kit registry: vofc_Kit, vofc_AssetRoute, kit_id() method, String Boundary Discipline in RCG

### 2026-01-17 13:18 - Heat - T

distributable-kits-typed-registry

### 2026-01-17 13:17 - ₢AAABH - n

Add kit registry types and string boundary discipline guidance

### 2026-01-17 12:58 - ₢AAABH - A

Type vofc_Kit and vofc_AssetRoute structs, redefine DISTRIBUTABLE_KITS, add compatibility accessor

### 2026-01-17 12:57 - Heat - r

moved AAABH to first

### 2026-01-17 12:56 - Heat - T

vvx-release-impl

### 2026-01-17 12:55 - Heat - T

distributable-kits-typed-registry

### 2026-01-17 12:44 - ₢AAAAG - W

Implemented voff_freshen.rs with freshen/collapse/parse. Migrated VOI→VOF, voic_→vofc_, template files to voc{cipher}mc pattern.

### 2026-01-17 12:44 - Heat - T

claude-md-freshening

### 2026-01-17 12:44 - Heat - T

vof-prefix-tree-checker

### 2026-01-17 12:43 - Heat - T

mcm-prefix-tree-declaration

### 2026-01-17 12:43 - Heat - S

vof-prefix-tree-checker

### 2026-01-17 12:41 - Heat - T

voi-directory-relocation

### 2026-01-17 12:25 - Heat - T

create-heat-rein-command

### 2026-01-17 12:25 - Heat - T

version-manifest

### 2026-01-17 12:25 - Heat - T

installation-identifier

### 2026-01-17 12:21 - Heat - T

distributable-kits-typed-registry

### 2026-01-17 12:20 - ₢AAABF - W

Migrated concept models to vov_veiled/ across 5 kits

### 2026-01-17 12:20 - Heat - T

veiled-directory-migration

### 2026-01-17 12:10 - Heat - S

distributable-kits-typed-registry

### 2026-01-17 12:08 - Heat - T

veiled-whisper-spec

### 2026-01-17 12:07 - Heat - S

voi-directory-relocation

### 2026-01-17 11:53 - Heat - S

Finalize install architecture in VOS and paddock

### 2026-01-17 11:49 - Heat - r

moved AAABF after AAABE

### 2026-01-17 11:49 - Heat - r

moved AAABE to first

### 2026-01-17 11:45 - Heat - S

veiled-directory-migration

### 2026-01-17 11:45 - Heat - S

veiled-whisper-spec

### 2026-01-17 11:33 - Heat - T

furlough-slash-mount

### 2026-01-17 11:29 - Heat - S

furlough-slash-mount

### 2026-01-17 11:29 - Heat - S

furlough-rust-impl

### 2026-01-17 11:29 - Heat - S

furlough-jjd-spec

### 2026-01-17 11:27 - ₢AAABA - n

Define uninstalled marker concept and refine managed section lifecycle in VOS spec

### 2026-01-17 11:27 - Heat - T

vos-claudemd-freshening-spec

### 2026-01-17 11:21 - Heat - T

vos-claudemd-freshening-spec

### 2026-01-17 11:15 - Heat - S

vos-claudemd-freshening-spec

### 2026-01-17 11:02 - ₢AAAA9 - W

Added brand/hallmark system to VOS spec

### 2026-01-17 11:02 - Heat - T

vos-brand-hallmark-system

### 2026-01-17 11:02 - Heat - T

liturgy-state-machine-vocabulary

### 2026-01-17 11:01 - ₢AAAA9 - n

Add hallmark version system and brand file to VVK release specification

### 2026-01-17 11:00 - Heat - S

liturgy-state-machine-vocabulary

### 2026-01-17 10:57 - ₢AAAA9 - A

Dispatch sonnet agent for VOS brand/hallmark spec additions

### 2026-01-17 10:48 - Heat - S

mcm-prefix-tree-declaration

### 2026-01-17 10:48 - Heat - S

vos-brand-hallmark-system

### 2026-01-17 10:24 - Heat - T

remove-envelope-from-inscription-structure

### 2026-01-17 10:23 - Heat - T

define-envelope-vesture-component

### 2026-01-17 10:20 - Heat - T

add-ensign-monogram-liturgy-terms

### 2026-01-17 10:14 - ₢AAAA7 - n

Clarify --first positioning logic to target first actionable pace, not absolute start

### 2026-01-17 10:14 - ₢AAAA7 - W

first-actionable semantics for --first, tests pass

### 2026-01-17 10:14 - Heat - T

rail-first-actionable-semantics

### 2026-01-17 10:12 - Heat - T

kit-asset-registry

### 2026-01-17 10:11 - ₢AAAA7 - A

first-actionable: find first rough/bridled, update rail+test+JJD

### 2026-01-17 10:09 - Heat - S

vos-liturgy-cleanup-batch

### 2026-01-17 10:05 - Heat - r

moved AAAA7 before AAAAA

### 2026-01-17 10:00 - ₢AAAA7 - n

Add JJK CLI syntax warnings and invocation pattern guidance

### 2026-01-17 09:59 - Heat - S

rail-first-actionable-semantics

### 2026-01-17 09:57 - Heat - r

moved AAAA4 to last

### 2026-01-17 09:53 - ₢AAAA5 - n

Add Dispatch vocabulary to VOS (tabtarget system components)

### 2026-01-17 09:53 - ₢AAAA5 - W

Added dispatch vocabulary vosd* to VOS with 5 terms

### 2026-01-17 09:53 - Heat - T

formalize-dispatch-vocabulary

### 2026-01-17 09:52 - ₢AAAA5 - A

Add dispatch vocabulary vosd* following liturgy pattern

### 2026-01-17 09:50 - ₢AAAA5 - n

Update jjc-heat-chalk and jjc-pace-wrap command specs to use identity types consistently

### 2026-01-17 09:48 - ₢AAAA6 - W

Added liturgy vocabulary vosl* to VOS with core terms and domain vestures

### 2026-01-17 09:47 - Heat - T

incorporate-liturgy-vocabulary-vos

### 2026-01-17 09:41 - ₢AAAA6 - A

Add liturgy vocabulary vosl* following MCM patterns

### 2026-01-17 09:40 - Heat - r

moved AAAA5 before AAAA4

### 2026-01-17 09:40 - Heat - r

moved AAAA6 before AAAA4

### 2026-01-17 09:38 - Heat - r

moved AAAA6 before AAAAA

### 2026-01-17 09:38 - Heat - r

moved AAAA5 before AAAAA

### 2026-01-17 09:38 - Heat - r

moved AAAA4 before AAAAA

### 2026-01-17 09:34 - Heat - T

incorporate-liturgy-vocabulary-vos

### 2026-01-17 09:34 - Heat - T

formalize-dispatch-vocabulary

### 2026-01-17 09:33 - Heat - T

define-envelope-vesture-component

### 2026-01-17 09:31 - Heat - S

incorporate-liturgy-vocabulary-vos

### 2026-01-17 09:31 - ₢AAAA3 - W

Completed as part of vos-install-procedures pace

### 2026-01-17 09:31 - Heat - T

vos-uninstall-procedure

### 2026-01-17 09:29 - ₢AAAA2 - W

AXLA: axe_bash_scripted + axo_routine rename. VOS: control terms, section headers, 4 procedures.

### 2026-01-17 09:29 - Heat - T

vos-install-procedures

### 2026-01-17 09:25 - ₢AAAA2 - F

Executing bridled pace: AXLA+VOS procedure updates

### 2026-01-17 09:24 - Heat - S

formalize-dispatch-vocabulary

### 2026-01-17 09:24 - Heat - T

vos-install-procedures

### 2026-01-17 09:24 - Heat - S

define-envelope-vesture-component

### 2026-01-17 08:49 - ₢AAAA1 - W

Created voi crate with voic_Cipher struct and 17 ciphers

### 2026-01-17 08:49 - Heat - T

common-crate-cipher-registry

### 2026-01-17 08:47 - ₢AAAA1 - F

Executing bridled pace

### 2026-01-17 08:45 - Heat - T

common-crate-cipher-registry

### 2026-01-17 08:32 - ₢AAAA0 - W

Renamed prime/primed to bridle/bridled throughout codebase

### 2026-01-17 08:32 - ₢AAAA0 - W

prime/primed→bridle/bridled vocabulary rename complete

### 2026-01-17 08:32 - Heat - T

prime-to-bridle-rename

### 2026-01-17 08:21 - ₢AAAA0 - F

Executing prime-to-bridle rename

### 2026-01-17 08:21 - ₢AAAA0 - n

Update JJ command reference and commit message generation prompt

### 2026-01-17 08:16 - ₢AAAA0 - n

Add struct field visibility guidance and test extraction checklist to RCG

### 2026-01-17 08:10 - ₢AAAAv - W

Extracted inline tests to separate jjt*.rs files per RCG

### 2026-01-17 08:10 - ₢AAAAv - W

Extracted 6 test files per RCG, all 133 tests pass

### 2026-01-17 08:10 - Heat - T

jjk-test-file-separation

### 2026-01-17 08:06 - Heat - S

vos-uninstall-procedure

### 2026-01-17 08:05 - Heat - S

vos-install-procedures

### 2026-01-17 08:05 - Heat - S

common-crate-cipher-registry

### 2026-01-17 08:04 - ₢AAAAv - F

Executing primed pace: extract tests to separate files

### 2026-01-17 08:04 - ₢AAAAr - W

Renamed tack_text/tack_direction to spec/direction in Rust and slash commands

### 2026-01-17 08:04 - ₢AAAAr - W

Renamed tack_text/tack_direction to spec/direction in Rust and 4 slash commands

### 2026-01-17 08:04 - Heat - T

saddle-output-field-rename

### 2026-01-17 07:59 - ₢AAAAr - F

Executing primed pace

### 2026-01-17 07:54 - Heat - r

moved AAAA0 after AAAAv

### 2026-01-17 07:54 - Heat - r

moved AAAAv after AAAAr

### 2026-01-17 07:54 - Heat - r

moved AAAAr after AAAAB

### 2026-01-16 15:38 - ₢AAAAv - n

Add jjc-heat-quarter command to evaluate and bridle primeable paces

### 2026-01-16 15:34 - Heat - T

jjk-test-file-separation

### 2026-01-16 15:30 - ₢AAAA0 - A

Primed: parallel Rust+docs, then sequential git mv and slash command updates

### 2026-01-16 15:30 - Heat - T

prime-to-bridle-rename

### 2026-01-16 15:30 - Heat - S

prime-to-bridle-rename

### 2026-01-16 15:21 - ₢AAAAy - n

``` Add parade --remaining filter, notch --size-limit override, and rail descriptive subjects

### 2026-01-16 15:17 - ₢AAAAy - W

Added --remaining flag to jjx_parade for filtering

### 2026-01-16 15:17 - Heat - T

parade-remaining-flag

### 2026-01-16 15:15 - ₢AAAAy - F

Executing primed pace

### 2026-01-16 15:15 - ₢AAAAw - W

Descriptive rail commit messages: move mode + order mode

### 2026-01-16 15:15 - Heat - T

rail-descriptive-commits

### 2026-01-16 15:14 - ₢AAAAw - F

Executing primed pace

### 2026-01-16 15:14 - ₢AAAAu - W

RCG templates + 14 Rust files with proprietary headers

### 2026-01-16 15:14 - Heat - T

rcg-copyright-templates

### 2026-01-16 15:12 - Heat - T

prime-merges-direction-into-spec

### 2026-01-16 15:11 - ₢AAAAu - F

Executing primed pace

### 2026-01-16 15:10 - Heat - T

prime-merges-direction-into-spec

### 2026-01-16 15:09 - Heat - S

prime-merges-direction-into-spec

### 2026-01-16 15:08 - Heat - T

saddle-output-field-rename

### 2026-01-16 15:01 - Heat - T

rail-descriptive-commits

### 2026-01-16 14:57 - ₢AAAAt - W

Added --size-limit to jjx_notch with vvcc_CommitArgs propagation and documentation

### 2026-01-16 14:57 - Heat - T

notch-size-limit-flag

### 2026-01-16 14:55 - ₢AAAAt - F

Executing primed pace

### 2026-01-16 14:50 - Heat - T

notch-size-limit-flag

### 2026-01-16 14:50 - ₢AAAAs - W

fix-gallops-commit-scope

### 2026-01-16 14:50 - ₢AAAAs - W

Haiku agent applied vvcm_commit pattern to slate/tally/rail/nominate. Fixed jjrf_display to jjrf_as_str for paddock paths.

### 2026-01-16 14:48 - Heat - T

fix-gallops-commit-scope

### 2026-01-16 14:39 - Heat - T

parade-remaining-flag

### 2026-01-16 14:39 - Heat - S

parade-remaining-flag

### 2026-01-16 14:38 - ₢AAAAx - W

Fixed 35 test compilation errors with RCG prefixes, all 133 tests pass

### 2026-01-16 14:38 - Heat - T

jjk-test-compilation-fix

### 2026-01-16 14:34 - ₢AAAAx - A

Systematic prefix fixes across 3 test files

### 2026-01-16 14:33 - Heat - S

jjk-test-compilation-fix

### 2026-01-16 14:33 - Heat - S

rail-descriptive-commits

### 2026-01-16 14:26 - Heat - r

reordered

### 2026-01-16 14:25 - Heat - r

reordered

### 2026-01-16 14:25 - Heat - r

reordered

### 2026-01-16 14:24 - Heat - T

rcg-copyright-templates

### 2026-01-16 14:24 - Heat - T

rcg-copyright-templates

### 2026-01-16 14:21 - Heat - S

jjk-test-file-separation

### 2026-01-16 14:21 - Heat - T

jjk-rcg-phase3-finalize

### 2026-01-16 14:21 - Heat - T

jjk-rcg-phase2-callsites

### 2026-01-16 14:20 - Heat - S

rcg-copyright-templates

### 2026-01-16 14:19 - Heat - S

notch-size-limit-flag

### 2026-01-16 14:19 - Heat - S

fix-gallops-commit-scope

### 2026-01-16 14:18 - ₢AAAAY - W

RCG prefixes applied to all JJK Rust declarations. 8 source files updated with file-specific prefixes, #![allow(non_camel_case_types)] added, cross-file references fixed.

### 2026-01-16 14:18 - Heat - T

jjk-rcg-compliance

### 2026-01-16 14:17 - ₢AAAAY - n

Apply RCG prefixes to JJK Rust declarations

### 2026-01-16 13:53 - Heat - S

saddle-output-field-rename

### 2026-01-16 13:49 - ₢AAAAY - F

Executing primed pace: 7 parallel agents for RCG prefixing

### 2026-01-16 13:49 - Heat - T

retire-heat-ab-test

### 2026-01-16 13:48 - Heat - T

retire-heat-ab-test

### 2026-01-16 13:47 - Heat - T

deprecate-jju-tabtargets

### 2026-01-16 13:47 - ₢AAAAj - W

jjx_retire --execute works: trophy written, paddock deleted, gallops updated, commit created

### 2026-01-16 13:46 - ₢AAAAf - n

Add Rust test suite execution via VVX tabtarget and workbench

### 2026-01-16 13:44 - ₢AAAAj - A

Test jjx_retire on AB: run retire, verify trophy/gallops/paddock/commit

### 2026-01-16 13:39 - ₢AAAAf - W

Deprecated JJW/JJT tabtargets, deleted jju_utility.sh, cleaned workbench to arcanum-only

### 2026-01-16 13:39 - Heat - T

deprecate-jju-tabtargets

### 2026-01-16 13:37 - ₢AAAAe - n

Strip Co-Authored-By trailer from Claude-generated commit messages

### 2026-01-16 13:34 - ₢AAAAp - W

Refactored rein to filter by identity not brand. Removed --brand CLI arg. 133 tests pass.

### 2026-01-16 13:34 - Heat - T

rein-filter-by-identity

### 2026-01-16 13:30 - Heat - T

rein-filter-by-identity

### 2026-01-16 13:30 - ₢AAAAf - n

Remove JJT testbench launcher and tabtargets

### 2026-01-16 13:28 - Heat - S

steeplechase-version-tracking

### 2026-01-16 13:27 - Heat - S

rein-filter-by-identity

### 2026-01-16 13:18 - ₢AAAAf - A

Delete tabtargets, audit jju functions, clean workbench, verify slash coverage

### 2026-01-16 13:16 - ₢AAAAe - n

Remove hardcoded co-author attribution from commit messages

### 2026-01-16 13:11 - ₢AAAAe - n

Document JJ operations guidance for slash commands vs direct calls

### 2026-01-16 13:09 - Heat - S

create-heat-rein-command

### 2026-01-16 13:08 - Heat - T

cleanup-orphan-rein

### 2026-01-16 13:07 - ₢AAAAe - n

Remove jju_rein function and jjw-rn command route

### 2026-01-16 13:06 - Heat - T

cleanup-orphan-rein

### 2026-01-16 13:01 - ₢AAAAe - n

Make notch action code required: standardize all commits with 'n' code

### 2026-01-16 12:54 - ₢AAAAe - notch

Surface dependency and sequencing concerns in rough pace analysis

### 2026-01-16 12:44 - ₢AAAAe - notch

Remove redundant gallops JSON prerequisites from JJ command docs

### 2026-01-16 12:37 - Heat - T

unify-commit-format

### 2026-01-16 12:37 - ₢AAAAd - W

Unified commit format to jjb:BRAND:IDENTITY[:ACTION]: pattern with coronet-based CLI

