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
