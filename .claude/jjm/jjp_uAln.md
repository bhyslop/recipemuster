# Paddock: jjk-v4-release-and-legacy-removal

## Context

Final V4 heat. Executes after ₣Ah (jjk-v4-vision) completes and V4 schema is stable. Single heat covering release, installation upgrade, and legacy removal.

## Phases

### Phase 1: Release V4
- Tag a V4 release
- Update installation documentation
- Verify V4 jjx builds and passes tests

### Phase 2: Upgrade Installations
- Upgrade work installation to V4 jjx
- V4 `jjdr_load` auto-migrates V3 gallops on first load
- Confirm all gallops files are now V4 format (`schema_version: 4`)
- Verify round-trip validation passes on migrated files

### Phase 3: Legacy Removal
- Re-enable strict round-trip byte-exact validation (remove V3 migration bypass in `jjdr_load`)
- Delete `jjrt_v3_types.rs` (frozen V3 type snapshot)
- Remove V3 schema reference section from JJS0
- Remove V3→V4 migration transforms from `jjdr_load`
- Update `jjrg_validate` to remove any V3-compat validation paths
- Clean up any `#[serde(alias = ...)]` attributes that served V3 compat

## Preconditions

- ₣Ah (jjk-v4-vision) must be complete
- All known gallops installations must be running V4 jjx
- All gallops files must have been loaded and saved at least once by V4 jjx (confirming migration)

## V3→V4 Migration Discipline

This heat inherits the migration discipline from ₣Ah's paddock. The key rule: every change that removes V3 compat code must be verified against the frozen V3 reference (`jjrt_v3_types.rs`) to ensure nothing is removed prematurely.

## Related Heats

- ₣Ah (jjk-v4-vision) — predecessor, must complete first
- ₣Am (jjk-v5-notional) — post-V4 ideas, independent

## References

- V4 migration discipline: ₣Ah paddock, "V3→V4 Migration Discipline" section
- V3 data model: `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc`
- Validation chain: `Tools/jjk/vov_veiled/src/jjri_io.rs` (`jjdr_load`, `jjdr_save`)
