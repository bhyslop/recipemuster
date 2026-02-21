# Paddock: jjk-heat-priority-ordering

## Context

Two related problems addressed here:

1. **Paddock filename encoding (urgent)**: macOS HFS+/APFS is case-insensitive by default.
   Firemark `AG` and `Ag` both resolve to `jjp_AG.md` on disk, causing silent paddock
   corruption. Fix: encode each firemark character with a `u`/`l` case prefix
   (e.g. `AG` → `jjp_uAuG.md`, `Ag` → `jjp_uAlg.md`). JJK detects legacy names on
   load and fatals with exact repair instructions (no built-in migration machinery).

2. **Heat priority ordering**: heats currently derive display/selection order from
   IndexMap insertion order (creation order). Furloughing is disruptive because there
   is no explicit priority list — `jjx_saddle` picks "first racing heat" by insertion
   order with no user control. Fix: add `heat_order: Vec<Firemark>` to `jjrg_Gallops`,
   parallel to `Heat.order` for paces. Field uses `#[serde(default)]` + lazy populate
   on load for tacit backward compat. Furlough changes status only, never position.

## References

- jjrt_types.rs — Gallops struct (heat_order field)
- jjri_io.rs — load path (lazy populate migration)
- jjro_ops.rs — nominate (append to heat_order), retire (remove from heat_order)
- jjrq_query.rs — muster (iterate heat_order), saddle (first racing in heat_order)
- jjrmu_muster.rs — muster display
- jjrsd_saddle.rs — saddle selection
- jjrg_gallops.rs (Gallops struct re-export)
