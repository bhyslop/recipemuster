# Heat Trophy: jjk-heat-priority-ordering

**Firemark:** ₣Ag
**Created:** 260221
**Retired:** 260222
**Status:** retired

## Paddock

# Paddock: jjk-heat-priority-ordering (₣Ag) / jjk-post-alpha-breaking (₣AG)
# NOTE: macOS collision — both heats share this file. Fix pending in ₣Ag pace 1.

## Context: ₣Ag — jjk-heat-priority-ordering

Two related problems:

1. **Paddock filename encoding (urgent)**: macOS HFS+/APFS is case-insensitive.
   Firemark `AG` and `Ag` both resolve to `jjp_AG.md` on disk, causing silent
   paddock corruption. Fix: encode each firemark character with a `u`/`l` case
   prefix (e.g. `AG` → `jjp_uAuG.md`, `Ag` → `jjp_uAlg.md`). JJK detects
   legacy names on load and fatals with exact repair instructions.

2. **Heat priority ordering**: heats derive display/selection order from IndexMap
   insertion order. Furloughing is disruptive because there is no explicit priority
   list. Fix: add `heat_order: Vec<Firemark>` to `jjrg_Gallops` with
   `#[serde(default)]` + lazy populate on load. Furlough changes status only,
   never position in heat_order.

## References

- jjri_io.rs — load-time legacy detection + paddock path computation
- jjro_ops.rs — nominate (new paddock creation), retire (paddock cleanup)
- jjrt_types.rs — Gallops struct (heat_order field)
- jjrq_query.rs / jjrmu_muster.rs / jjrsd_saddle.rs — muster/saddle consumers

## Paces

### jjs0-paddock-encoding-update (₢AgAAC) [complete]

**[260221-1325] complete**

Update JJS0 specification to reflect the ul-prefix paddock filename encoding
scheme introduced in this heat.

## Changes needed

1. **Paddock filename encoding** — Document the `ul`-prefix scheme:
   - Each firemark character prefixed with `u` (uppercase) or `l` (lowercase)
   - Examples: `AG` → `jjp_uAuG.md`, `Ag` → `jjp_uAlg.md`
   - Rationale: macOS HFS+/APFS is case-insensitive by default; raw firemark
     names collide silently when both upper and lowercase variants are active

2. **paddock_file ownership** — Document that `paddock_file` in `jjrg_Heat` is
   computed by `jjdr_load` from the firemark and never stored meaningfully.
   The stored value in JSON is overwritten on load; correct value written back
   on next save. External tools must not rely on the stored value.

3. **Legacy detection behavior** — Document the existence-check + fatal pattern:
   on load, if any encoded paddock file is absent, JJK fatals with `mv`
   instructions covering all missing files. No gallops.json editing required.

4. **Migration note** — Any project importing a new JJK binary will immediately
   see the fatal if legacy raw-firemark paddock files are present. The error
   message is fully self-contained.

## Files

JJS0-GallopsData.adoc (lenses/ or jjk/vov_veiled/)

**[260221-1325] rough**

Update JJS0 specification to reflect the ul-prefix paddock filename encoding
scheme introduced in this heat.

## Changes needed

1. **Paddock filename encoding** — Document the `ul`-prefix scheme:
   - Each firemark character prefixed with `u` (uppercase) or `l` (lowercase)
   - Examples: `AG` → `jjp_uAuG.md`, `Ag` → `jjp_uAlg.md`
   - Rationale: macOS HFS+/APFS is case-insensitive by default; raw firemark
     names collide silently when both upper and lowercase variants are active

2. **paddock_file ownership** — Document that `paddock_file` in `jjrg_Heat` is
   computed by `jjdr_load` from the firemark and never stored meaningfully.
   The stored value in JSON is overwritten on load; correct value written back
   on next save. External tools must not rely on the stored value.

3. **Legacy detection behavior** — Document the existence-check + fatal pattern:
   on load, if any encoded paddock file is absent, JJK fatals with `mv`
   instructions covering all missing files. No gallops.json editing required.

4. **Migration note** — Any project importing a new JJK binary will immediately
   see the fatal if legacy raw-firemark paddock files are present. The error
   message is fully self-contained.

## Files

JJS0-GallopsData.adoc (lenses/ or jjk/vov_veiled/)

### paddock-filename-encoding (₢AgAAB) [complete]

**[260221-1332] complete**

Implement case-safe paddock filename encoding for macOS compatibility.

## Problem

macOS HFS+/APFS is case-insensitive by default. Paddock files named `jjp_{firemark}.md`
silently collide when firemark pairs like `AG`/`Ag` are both active. The collision
causes one heat's paddock to overwrite the other's with no error.

## Encoding Scheme

Prefix each firemark character with `u` (uppercase) or `l` (lowercase):

- `AG` → `jjp_uAuG.md`
- `Ag` → `jjp_uAlg.md`
- `ag` → `jjp_lalg.md`

The `u`/`l` prefix chars are always lowercase, so case-folding never collapses
two distinct encoded strings to the same value.

## Detection and Recovery

On gallops load, iterate all heats and compare each `paddock_file` value against
the expected encoded name. If any mismatch is found, **fatal immediately** with:

```
ERROR: Legacy paddock names detected. Repair required:

  mv .claude/jjm/jjp_AG.md .claude/jjm/jjp_uAuG.md
  # update gallops.json: heat "AG" paddock_file -> "jjp_uAuG.md"
```

Report ALL mismatches at once (do not stop at first). No built-in migration — the
error message IS the repair instruction for the Claude Code instance.

## New Paddock Creation

Update `jjx_nominate` (and anywhere else paddock files are created) to use the
encoded name from day one. No legacy names ever written after this change.

## Files

jjri_io.rs (load-time detection), jjrno_nominate.rs or jjro_ops.rs (new paddock creation),
jjrrt_retire.rs (paddock cleanup on retire)

**[260221-1258] rough**

Implement case-safe paddock filename encoding for macOS compatibility.

## Problem

macOS HFS+/APFS is case-insensitive by default. Paddock files named `jjp_{firemark}.md`
silently collide when firemark pairs like `AG`/`Ag` are both active. The collision
causes one heat's paddock to overwrite the other's with no error.

## Encoding Scheme

Prefix each firemark character with `u` (uppercase) or `l` (lowercase):

- `AG` → `jjp_uAuG.md`
- `Ag` → `jjp_uAlg.md`
- `ag` → `jjp_lalg.md`

The `u`/`l` prefix chars are always lowercase, so case-folding never collapses
two distinct encoded strings to the same value.

## Detection and Recovery

On gallops load, iterate all heats and compare each `paddock_file` value against
the expected encoded name. If any mismatch is found, **fatal immediately** with:

```
ERROR: Legacy paddock names detected. Repair required:

  mv .claude/jjm/jjp_AG.md .claude/jjm/jjp_uAuG.md
  # update gallops.json: heat "AG" paddock_file -> "jjp_uAuG.md"
```

Report ALL mismatches at once (do not stop at first). No built-in migration — the
error message IS the repair instruction for the Claude Code instance.

## New Paddock Creation

Update `jjx_nominate` (and anywhere else paddock files are created) to use the
encoded name from day one. No legacy names ever written after this change.

## Files

jjri_io.rs (load-time detection), jjrno_nominate.rs or jjro_ops.rs (new paddock creation),
jjrrt_retire.rs (paddock cleanup on retire)

### heat-order-btreemap-migration (₢AgAAA) [complete]

**[260221-1818] complete**

Add explicit heat ordering to Gallops with BTreeMap switch and seamless cross-project migration.

## Problem

Two related deficiencies:

1. Heats derive display/selection order from IndexMap insertion order. Furlough reorders the
   IndexMap (shift_remove + shift_insert(0)) causing massive gallops.json diffs on every
   status change. No explicit priority list exists.

2. JJS0 mandates BTreeMap for heats to ensure deterministic serialization. Currently violated.

Additionally, other projects using JJK will install the new binary and must migrate
automatically from old format (no heat_order, IndexMap key order) to new format
(heat_order present, BTreeMap sorted order) — without manual steps and without
the stepping-stone commits this project accumulates during development.

## Solution

Combined single breaking-change: add heat_order + switch to BTreeMap + conditional
round-trip migration. Other projects get old-format → new-format in one shot.

## Key Design: Conditional Round-Trip Skip

The load routine detects old-format files by heat_order being absent (empty after
serde default). For these files, the round-trip check is skipped — because BTreeMap
will reserialize heats in sorted order, which won't match the original furlough-shuffled
bytes. For new-format files (heat_order non-empty), the strict round-trip check applies
as before. This is a permanent condition in jjdr_load, not a temporary disable.

## Schema Change

```json
{
  "next_heat_seed": "Am",
  "heat_order": ["₣AD", "₣AG", ...],   // NEW: sorted on first migration
  "heats": { ... }                       // keys now in BTreeMap sorted order
}
```

## Migration Behavior (any project, any state)

1. Load old gallops.json: heat_order absent → serde default → empty vec
2. Round-trip check: SKIPPED (heat_order empty = migration mode)
3. Deserialize heats into BTreeMap → keys in sorted order
4. Populate heat_order from heats.keys() (sorted)
5. Paddock recomputation and existence check (unchanged)
6. Semantic validation
7. First operation saves → new format committed; subsequent loads use strict round-trip

## Behavior Changes

- jjx_nominate: append new firemark to heat_order
- jjx_archive (retire): remove firemark from heat_order; use BTreeMap remove()
- jjx_alter (furlough): status-only change; drop shift_remove/shift_insert entirely
- jjx_continue (garland): after nominate (which appends), move new firemark to heat_order[0]
- jjx_list (muster): iterate heat_order for display order; drop status sort
- jjrq_resolve_default_heat (saddle): iterate heat_order for first racing heat

## Production Files

- jjrt_types.rs — add heat_order field (serde default + skip_serializing_if = Vec::is_empty);
  change heats: IndexMap → BTreeMap; remove indexmap import, add BTreeMap import
- jjri_io.rs — conditional round-trip skip + heat_order migration in jjdr_load
- jjro_ops.rs — nominate/retire/furlough/garland changes; remove indexmap import
- jjrmu_muster.rs — iterate heat_order, drop status sort
- jjrq_query.rs — resolve_default_heat uses heat_order

NOT needed: jjrsd_saddle.rs — its heats.iter() for the racing-heats display table
sorts explicitly anyway; no behavioral change required.

## Test Files

All test files that construct jjrg_Gallops struct literals need:
- heats: IndexMap::new() → heats: BTreeMap::new()
- heat_order: vec![] added to struct literal
- indexmap import removed, BTreeMap import added

Files: jjrno_nominate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs,
       jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs

### Test changes beyond struct literals

jjtgl_garland.rs:173 — assert heat_order[0] instead of heats.keys().next()
(garland now puts new heat first via heat_order, not IndexMap position)

jjtfu_furlough.rs + jjtg_gallops.rs — FURLOUGH ORDERING TESTS MUST BE DELETED.
Any test asserting that furlough moves a heat to heats.keys().next() or position 0
in the IndexMap is testing behavior that no longer exists. Find and delete these
assertions; do not attempt to rewrite them against heat_order (furlough intentionally
does NOT change heat_order position).

jjrno_nominate.rs — ADD assertion that heat_order contains the new firemark after
nominate. Currently no such assertion exists.

jjtg_gallops.rs — similarly audit for any furlough-reordering assertions and delete.

**[260221-1735] rough**

Add explicit heat ordering to Gallops with BTreeMap switch and seamless cross-project migration.

## Problem

Two related deficiencies:

1. Heats derive display/selection order from IndexMap insertion order. Furlough reorders the
   IndexMap (shift_remove + shift_insert(0)) causing massive gallops.json diffs on every
   status change. No explicit priority list exists.

2. JJS0 mandates BTreeMap for heats to ensure deterministic serialization. Currently violated.

Additionally, other projects using JJK will install the new binary and must migrate
automatically from old format (no heat_order, IndexMap key order) to new format
(heat_order present, BTreeMap sorted order) — without manual steps and without
the stepping-stone commits this project accumulates during development.

## Solution

Combined single breaking-change: add heat_order + switch to BTreeMap + conditional
round-trip migration. Other projects get old-format → new-format in one shot.

## Key Design: Conditional Round-Trip Skip

The load routine detects old-format files by heat_order being absent (empty after
serde default). For these files, the round-trip check is skipped — because BTreeMap
will reserialize heats in sorted order, which won't match the original furlough-shuffled
bytes. For new-format files (heat_order non-empty), the strict round-trip check applies
as before. This is a permanent condition in jjdr_load, not a temporary disable.

## Schema Change

```json
{
  "next_heat_seed": "Am",
  "heat_order": ["₣AD", "₣AG", ...],   // NEW: sorted on first migration
  "heats": { ... }                       // keys now in BTreeMap sorted order
}
```

## Migration Behavior (any project, any state)

1. Load old gallops.json: heat_order absent → serde default → empty vec
2. Round-trip check: SKIPPED (heat_order empty = migration mode)
3. Deserialize heats into BTreeMap → keys in sorted order
4. Populate heat_order from heats.keys() (sorted)
5. Paddock recomputation and existence check (unchanged)
6. Semantic validation
7. First operation saves → new format committed; subsequent loads use strict round-trip

## Behavior Changes

- jjx_nominate: append new firemark to heat_order
- jjx_archive (retire): remove firemark from heat_order; use BTreeMap remove()
- jjx_alter (furlough): status-only change; drop shift_remove/shift_insert entirely
- jjx_continue (garland): after nominate (which appends), move new firemark to heat_order[0]
- jjx_list (muster): iterate heat_order for display order; drop status sort
- jjrq_resolve_default_heat (saddle): iterate heat_order for first racing heat

## Production Files

- jjrt_types.rs — add heat_order field (serde default + skip_serializing_if = Vec::is_empty);
  change heats: IndexMap → BTreeMap; remove indexmap import, add BTreeMap import
- jjri_io.rs — conditional round-trip skip + heat_order migration in jjdr_load
- jjro_ops.rs — nominate/retire/furlough/garland changes; remove indexmap import
- jjrmu_muster.rs — iterate heat_order, drop status sort
- jjrq_query.rs — resolve_default_heat uses heat_order

NOT needed: jjrsd_saddle.rs — its heats.iter() for the racing-heats display table
sorts explicitly anyway; no behavioral change required.

## Test Files

All test files that construct jjrg_Gallops struct literals need:
- heats: IndexMap::new() → heats: BTreeMap::new()
- heat_order: vec![] added to struct literal
- indexmap import removed, BTreeMap import added

Files: jjrno_nominate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs,
       jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs

### Test changes beyond struct literals

jjtgl_garland.rs:173 — assert heat_order[0] instead of heats.keys().next()
(garland now puts new heat first via heat_order, not IndexMap position)

jjtfu_furlough.rs + jjtg_gallops.rs — FURLOUGH ORDERING TESTS MUST BE DELETED.
Any test asserting that furlough moves a heat to heats.keys().next() or position 0
in the IndexMap is testing behavior that no longer exists. Find and delete these
assertions; do not attempt to rewrite them against heat_order (furlough intentionally
does NOT change heat_order position).

jjrno_nominate.rs — ADD assertion that heat_order contains the new firemark after
nominate. Currently no such assertion exists.

jjtg_gallops.rs — similarly audit for any furlough-reordering assertions and delete.

**[260221-1728] rough**

Add explicit heat ordering to Gallops with BTreeMap switch and seamless cross-project migration.

## Problem

Two related deficiencies:

1. Heats derive display/selection order from IndexMap insertion order. Furlough reorders the
   IndexMap (shift_remove + shift_insert(0)) causing massive gallops.json diffs on every
   status change. No explicit priority list exists.

2. JJS0 mandates BTreeMap for heats to ensure deterministic serialization. Currently violated.

Additionally, other projects using JJK will install the new binary and must migrate
automatically from old format (no heat_order, IndexMap key order) to new format
(heat_order present, BTreeMap sorted order) — without manual steps and without
the stepping-stone commits this project accumulates during development.

## Solution

Combined single breaking-change: add heat_order + switch to BTreeMap + conditional
round-trip migration. Other projects get old-format → new-format in one shot.

## Key Design: Conditional Round-Trip Skip

The load routine detects old-format files by heat_order being absent (empty after
serde default). For these files, the round-trip check is skipped — because BTreeMap
will reserialize heats in sorted order, which won't match the original furlough-shuffled
bytes. For new-format files (heat_order non-empty), the strict round-trip check applies
as before.

This is a permanent condition in jjdr_load, not a temporary disable.

## Schema Change

```json
{
  "next_heat_seed": "Am",
  "heat_order": ["₣AD", "₣AG", ...],   // NEW: sorted on first migration
  "heats": { ... }                       // keys now in BTreeMap sorted order
}
```

## Migration Behavior (any project, any state)

1. Load old gallops.json: heat_order absent → serde default → empty vec
2. Round-trip check: SKIPPED (heat_order empty = migration mode)
3. Deserialize heats into BTreeMap → keys in sorted order
4. Populate heat_order from heats.keys() (sorted)
5. Paddock recomputation and existence check (unchanged)
6. Semantic validation
7. First operation saves → new format committed; subsequent loads use strict round-trip

## Behavior Changes

- jjx_nominate: append new firemark to heat_order
- jjx_archive (retire): remove firemark from heat_order; use BTreeMap remove()
- jjx_alter (furlough): status-only change; drop shift_remove/shift_insert entirely
- jjx_continue (garland): after nominate (which appends), move new firemark to heat_order[0]
- jjx_list (muster): iterate heat_order for display order; drop status sort
- jjrq_resolve_default_heat (saddle): iterate heat_order for first racing heat

## Files

Production:
- jjrt_types.rs — add heat_order field (serde default + skip_serializing_if); heats: BTreeMap
- jjri_io.rs — conditional round-trip skip + heat_order migration in jjdr_load
- jjro_ops.rs — nominate/retire/furlough/garland changes
- jjrmu_muster.rs — iterate heat_order, drop status sort
- jjrq_query.rs — resolve_default_heat uses heat_order

Tests (struct literal updates + ordering assertion fixes):
- jjrno_nominate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs,
  jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs
- jjtgl_garland.rs:173 specifically: assert heat_order[0] instead of heats.keys().next()

**[260221-1726] rough**

Add explicit heat ordering to Gallops with BTreeMap switch and seamless cross-project migration.

## Problem

Two related deficiencies:

1. Heats derive display/selection order from IndexMap insertion order. Furlough reorders the
   IndexMap (shift_remove + shift_insert(0)) causing massive gallops.json diffs on every
   status change. No explicit priority list exists.

2. JJS0 mandates BTreeMap for heats to ensure deterministic serialization. Currently violated.

Additionally, other projects using JJK will install the new binary and must migrate
automatically from old format (no heat_order, IndexMap key order) to new format
(heat_order present, BTreeMap sorted order) — without manual steps and without
the stepping-stone commits this project accumulates during development.

## Solution

Combined single breaking-change: add heat_order + switch to BTreeMap + conditional
round-trip migration. Other projects get old-format → new-format in one shot.

## Key Design: Conditional Round-Trip Skip

The load routine detects old-format files by heat_order being absent (empty after
serde default). For these files, the round-trip check is skipped — because BTreeMap
will reserialize heats in sorted order, which won't match the original furlough-shuffled
bytes. For new-format files (heat_order non-empty), the strict round-trip check applies
as before.

This is a permanent condition in jjdr_load, not a temporary disable.

## Schema Change

```json
{
  "next_heat_seed": "Am",
  "heat_order": ["₣AD", "₣AG", ...],   // NEW: sorted on first migration
  "heats": { ... }                       // keys now in BTreeMap sorted order
}
```

## Migration Behavior (any project, any state)

1. Load old gallops.json: heat_order absent → serde default → empty vec
2. Round-trip check: SKIPPED (heat_order empty = migration mode)
3. Deserialize heats into BTreeMap → keys in sorted order
4. Populate heat_order from heats.keys() (sorted)
5. Paddock recomputation and existence check (unchanged)
6. Semantic validation
7. First operation saves → new format committed; subsequent loads use strict round-trip

## Behavior Changes

- jjx_nominate: append new firemark to heat_order
- jjx_archive (retire): remove firemark from heat_order; use BTreeMap remove()
- jjx_alter (furlough): status-only change; drop shift_remove/shift_insert entirely
- jjx_continue (garland): after nominate (which appends), move new firemark to heat_order[0]
- jjx_list (muster): iterate heat_order for display order; drop status sort
- jjrq_resolve_default_heat (saddle): iterate heat_order for first racing heat

## Files

Production:
- jjrt_types.rs — add heat_order field (serde default + skip_serializing_if); heats: BTreeMap
- jjri_io.rs — conditional round-trip skip + heat_order migration in jjdr_load
- jjro_ops.rs — nominate/retire/furlough/garland changes
- jjrmu_muster.rs — iterate heat_order, drop status sort
- jjrq_query.rs — resolve_default_heat uses heat_order

Tests (struct literal updates + ordering assertion fixes):
- jjrno_nominate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs,
  jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs
- jjtgl_garland.rs:173 specifically: assert heat_order[0] instead of heats.keys().next()

**[260221-1227] rough**

Drafted from ₢AGAAE in ₣AG.

Add explicit heat ordering to Gallops while keeping BTreeMap for deterministic serialization.

## Problem

Heats currently derive order from BTreeMap key sorting. Several commands depend on "heat order" semantics:
- `jjx_muster`: display order
- `jjx_saddle`: "first racing heat" selection when firemark omitted
- Future: heat priority for concurrent work

BTreeMap gives alphabetical firemark order, not semantic priority order.

## Solution

Add `heat_order: Vec<Firemark>` to Gallops record, parallel to how `Heat.order` tracks pace order.

## Schema Change

```json
{
  "next_heat_seed": "AM",
  "heat_order": ["AF", "AH", "AI", ...],  // NEW FIELD
  "heats": { ... }
}
```

## Behavior

- `jjx_nominate`: append new firemark to `heat_order`
- `jjx_retire`: remove firemark from `heat_order`
- `jjx_muster`: iterate `heat_order` instead of BTreeMap keys
- `jjx_saddle` (no firemark): use first racing heat from `heat_order`
- Future: `jjx_rail` for heats (reorder heat_order)

## Migration

Existing gallops.json needs `heat_order` populated from current BTreeMap keys on first load (or explicit migration).

## Files

jjrg_gallops.rs (Gallops struct), jjro_ops.rs (nominate, retire), jjrq_query.rs (muster, saddle), JJSA spec

**[260124-1031] rough**

Add explicit heat ordering to Gallops while keeping BTreeMap for deterministic serialization.

## Problem

Heats currently derive order from BTreeMap key sorting. Several commands depend on "heat order" semantics:
- `jjx_muster`: display order
- `jjx_saddle`: "first racing heat" selection when firemark omitted
- Future: heat priority for concurrent work

BTreeMap gives alphabetical firemark order, not semantic priority order.

## Solution

Add `heat_order: Vec<Firemark>` to Gallops record, parallel to how `Heat.order` tracks pace order.

## Schema Change

```json
{
  "next_heat_seed": "AM",
  "heat_order": ["AF", "AH", "AI", ...],  // NEW FIELD
  "heats": { ... }
}
```

## Behavior

- `jjx_nominate`: append new firemark to `heat_order`
- `jjx_retire`: remove firemark from `heat_order`
- `jjx_muster`: iterate `heat_order` instead of BTreeMap keys
- `jjx_saddle` (no firemark): use first racing heat from `heat_order`
- Future: `jjx_rail` for heats (reorder heat_order)

## Migration

Existing gallops.json needs `heat_order` populated from current BTreeMap keys on first load (or explicit migration).

## Files

jjrg_gallops.rs (Gallops struct), jjro_ops.rs (nominate, retire), jjrq_query.rs (muster, saddle), JJSA spec

### jjk-test-raii-tempdir (₢AgAAD) [complete]

**[260222-0734] complete**

Replace manual std::env::temp_dir() pre/post cleanup in JJK Rust tests with a RAII
Drop-based TempDir guard that auto-cleans on panic.

Current pattern (in jjtgl_garland.rs, jjtg_gallops.rs, jjrno_nominate.rs, etc.):
  let temp_dir = std::env::temp_dir().join("jjk_test_name");
  let _ = std::fs::remove_dir_all(&temp_dir);  // pre-clean
  std::fs::create_dir_all(&temp_dir).unwrap();
  // ... test body ...
  let _ = std::fs::remove_dir_all(&temp_dir);  // post-clean (skipped on panic!)

Problem: post-clean never runs if the test panics, leaving stale directories in /tmp.

Desired: a jjk-internal RAII struct (no new crate dependency) that removes itself on Drop:
  struct JjkTestDir(PathBuf);
  impl Drop for JjkTestDir { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

Live in a new test utility module (e.g., jjtu_testdir.rs with #[cfg(test)]).
Update all existing tests that use the manual pattern.

**[260221-1828] rough**

Replace manual std::env::temp_dir() pre/post cleanup in JJK Rust tests with a RAII
Drop-based TempDir guard that auto-cleans on panic.

Current pattern (in jjtgl_garland.rs, jjtg_gallops.rs, jjrno_nominate.rs, etc.):
  let temp_dir = std::env::temp_dir().join("jjk_test_name");
  let _ = std::fs::remove_dir_all(&temp_dir);  // pre-clean
  std::fs::create_dir_all(&temp_dir).unwrap();
  // ... test body ...
  let _ = std::fs::remove_dir_all(&temp_dir);  // post-clean (skipped on panic!)

Problem: post-clean never runs if the test panics, leaving stale directories in /tmp.

Desired: a jjk-internal RAII struct (no new crate dependency) that removes itself on Drop:
  struct JjkTestDir(PathBuf);
  impl Drop for JjkTestDir { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

Live in a new test utility module (e.g., jjtu_testdir.rs with #[cfg(test)]).
Update all existing tests that use the manual pattern.

### jjk-test-raii-tempdir (₢AgAAE) [complete]

**[260222-0739] complete**

Replace manual std::env::temp_dir() pre/post cleanup in JJK Rust tests with a RAII
Drop-based TempDir guard that auto-cleans on panic.

Current pattern (in jjtgl_garland.rs, jjtg_gallops.rs, jjrno_nominate.rs, etc.):
  let temp_dir = std::env::temp_dir().join("jjk_test_name");
  let _ = std::fs::remove_dir_all(&temp_dir);  // pre-clean
  std::fs::create_dir_all(&temp_dir).unwrap();
  // ... test body ...
  let _ = std::fs::remove_dir_all(&temp_dir);  // post-clean (skipped on panic!)

Problem: post-clean never runs if the test panics, leaving stale directories in /tmp.

Desired: a jjk-internal RAII struct (no new crate dependency) that removes itself on Drop:
  struct JjkTestDir(PathBuf);
  impl Drop for JjkTestDir { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

Live in a new test utility module (e.g., jjtu_testdir.rs with #[cfg(test)]).
Update all existing tests that use the manual pattern.

**[260221-1830] rough**

Replace manual std::env::temp_dir() pre/post cleanup in JJK Rust tests with a RAII
Drop-based TempDir guard that auto-cleans on panic.

Current pattern (in jjtgl_garland.rs, jjtg_gallops.rs, jjrno_nominate.rs, etc.):
  let temp_dir = std::env::temp_dir().join("jjk_test_name");
  let _ = std::fs::remove_dir_all(&temp_dir);  // pre-clean
  std::fs::create_dir_all(&temp_dir).unwrap();
  // ... test body ...
  let _ = std::fs::remove_dir_all(&temp_dir);  // post-clean (skipped on panic!)

Problem: post-clean never runs if the test panics, leaving stale directories in /tmp.

Desired: a jjk-internal RAII struct (no new crate dependency) that removes itself on Drop:
  struct JjkTestDir(PathBuf);
  impl Drop for JjkTestDir { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

Live in a new test utility module (e.g., jjtu_testdir.rs with #[cfg(test)]).
Update all existing tests that use the manual pattern.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 C jjs0-paddock-encoding-update
  2 B paddock-filename-encoding
  3 A heat-order-btreemap-migration
  4 D jjk-test-raii-tempdir
  5 E jjk-test-raii-tempdir

CBADE
··xx· jjrno_nominate.rs, jjtg_gallops.rs, jjtgl_garland.rs
x·x·· JJS0-GallopsData.adoc
···x· jjtu_testdir.rs, lib.rs
··x·· BCG-BashConsoleGuide.md, Cargo.lock, Cargo.toml, JJSCFU-furlough.adoc, JJSCGL-garland.adoc, JJSCMU-muster.adoc, JJSCNO-nominate.adoc, JJSCRT-retire.adoc, JJSCSD-saddle.adoc, JJSRLD-load.adoc, jjri_io.rs, jjrmu_muster.rs, jjro_ops.rs, jjrq_query.rs, jjrsd_saddle.rs, jjrt_types.rs, jjtfu_furlough.rs, jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs, rbcnc_cli.sh, rbcnx_cli.sh, rbrn_cli.sh, rbz_zipper.sh

Commit swim lanes (x = commit affiliated with pace):

  1 B paddock-filename-encoding
  2 C jjs0-paddock-encoding-update
  3 A heat-order-btreemap-migration
  4 D jjk-test-raii-tempdir
  5 E jjk-test-raii-tempdir
  6 * heat-level

123456789abcdefghijklmno
·····x···x··············  B  2c
·······xx···············  C  2c
··········x···xxxxx·····  A  6c
····················xxx·  D  3c
·······················x  E  1c
xxxxx·x····xxx·····x····  *  10c
```

## Steeplechase

### 2026-02-22 07:39 - ₢AgAAE - W

Duplicate of AgAAD — RAII TempDir work already complete

### 2026-02-22 07:34 - ₢AgAAD - W

Added RAII JjkTestDir guard in jjtu_testdir.rs; replaced 10 manual temp_dir patterns across 3 test files

### 2026-02-22 07:34 - ₢AgAAD - n

RAII JjkTestDir struct in cfg(test) module; mechanical find-and-replace across test files

### 2026-02-22 07:25 - ₢AgAAD - A

RAII JjkTestDir struct in cfg(test) module; mechanical find-and-replace across test files

### 2026-02-21 18:30 - Heat - S

jjk-test-raii-tempdir

### 2026-02-21 18:18 - ₢AgAAA - n

jjb:1011-AgAAA:₢AgAAA:n: Split rbrn_cli.sh into rbcnc_cli.sh (light furnish) and rbcnx_cli.sh (heavy furnish); document two-tier CLI pattern in BCG; update rbz_zipper.sh references

### 2026-02-21 18:16 - ₢AgAAA - n

Fix nominate test to use std::env::temp_dir() instead of Path::new(".") to avoid polluting repo

### 2026-02-21 18:12 - ₢AgAAA - n

Update Cargo.lock files after indexmap dependency removal

### 2026-02-21 18:12 - ₢AgAAA - n

Switch heats IndexMap→BTreeMap; add heat_order for explicit priority ordering with conditional round-trip migration; update 5 prod files, 7 test files, 8 spec files; remove indexmap dep; fix saddle racing-heats to respect heat_order

### 2026-02-21 17:40 - ₢AgAAA - A

Switch heats IndexMap→BTreeMap; conditional round-trip skip for migration; fix furlough/retire/garland ops; update 7 test files (IndexMap→BTreeMap, delete furlough-ordering tests, add nominate heat_order assertion)

### 2026-02-21 17:35 - Heat - T

heat-order-btreemap-migration

### 2026-02-21 17:28 - Heat - T

heat-order-btreemap-migration

### 2026-02-21 17:26 - Heat - T

add-heat-order-vector

### 2026-02-21 17:03 - ₢AgAAA - A

Add heat_order: Vec<String> to jjrg_Gallops with serde(default). Migration on load. Update nominate/retire/muster/saddle consumers. 5 files: jjrt_types, jjri_io, jjro_ops, jjrmu_muster, jjrq_query.

### 2026-02-21 13:32 - ₢AgAAB - W

Implement ul-prefix paddock filename encoding: jjri_paddock_path helper, jjdr_load recompute+existence-check, fix garland/draft/nominate/persist; rename all 22 paddocks; update JJS0 spec

### 2026-02-21 13:25 - ₢AgAAC - W

Updated paddock_file definition: ul-prefix encoding, computed-on-load behavior, existence-check fatal

### 2026-02-21 13:25 - ₢AgAAC - n

jjb:AgAAC: update JJS0 paddock_file encoding docs: describe ul-encoding format, examples, load-time recomputation, and fatal-on-missing behavior

### 2026-02-21 13:25 - Heat - S

jjs0-paddock-encoding-update

### 2026-02-21 13:02 - ₢AgAAB - A

Add zjjri_encode_paddock_path() helper; update jjri_io.rs:143 + jjro_ops.rs:38 to use it; add load-time legacy detection loop in jjdr_load reporting all mismatches with exact mv commands

### 2026-02-21 13:01 - Heat - n

Slate paddock-filename-encoding as pace 1; restore shared paddock to old naming pending Rust fix

### 2026-02-21 12:57 - Heat - n

Repair AG/Ag paddock collision: rename legacy jjp_AG.md to ul-encoded filenames, update gallops paddock_file refs

### 2026-02-21 12:27 - Heat - D

restring 1 paces from ₣AG

### 2026-02-21 12:26 - Heat - f

racing

### 2026-02-21 12:26 - Heat - N

jjk-heat-priority-ordering

