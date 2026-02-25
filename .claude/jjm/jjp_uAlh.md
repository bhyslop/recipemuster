# Paddock: jjk-v4-vision

## Current Design

**Authoritative**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline.

Key decisions: no leg layer, branches instead of worktrees, composable gaits, three-phase pipeline (school → breeze → prance), warrant as LLM-to-LLM communication.

## Schema Decisions (cchat-20260224 groom session)

- **Tack eliminated**: Flat mutable fields on pace (state, silks, gaits, docket, warrant, chain). No append-only history.
- **Branch names derived**: `jj/{firemark}/{coronet}`, not stored. State determines validity.
- **Dependencies via school**: `chain` field (optional coronet) set by school, not breeze. Breeze is mechanical.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Zero actionable overlap with V3.
- **Reined state**: Interactive-required. School decides ready (autonomous) vs reined (human-in-loop).
- **Gaits registry**: New top-level gallops key, silks-keyed.
- **Markers eliminated**: Restore on need.
- **Field renames**: V3 `text` → `docket`, V3 `direction` → `warrant`.

## Still Open

- **School mechanics**: Q&A flow, chain detection, ready-vs-reined promotion
- **Candidate rejection flow**: Back to ready or green? Human decides per-rejection?
- **Prance merge conflicts**: Rebase, present, or re-breeze?
- **Work-out-later items**: Gait CRUD, mount rendering, gait switching, orient, migration

## References

- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- `Memos/memo-20260222-jjk-v4-vision.md` — Superseded original design seeds