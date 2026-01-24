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
