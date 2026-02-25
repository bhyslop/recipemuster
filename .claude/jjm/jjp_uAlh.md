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

## Slash Command Reduction (cchat-20260224 groom session)

V4 aggressively reduces slash commands. Most become CLAUDE.md verb table entries. Only protocol-heavy verbs justify slash commands.

**Delete entirely (concept eliminated):**
- bridle, quarter, braid, garland

**Demote to CLAUDE.md verb table:**
- slate, reslate, notch, rail, furlough, restring, retire-dryrun, retire-FINAL

**Notch simplification**: Branch model eliminates file-selection complexity. All changes on a pace branch belong to that pace. Notch becomes a simple verb table entry.

**Keep as slash command:**
- mount — protocol-heavy, may keep
- school — new, protocol-heavy (the jjx-analytical + LLM-conversational dance)
- prance — new, protocol-heavy

**Undecided:**
- groom — changing nature in V4, TBD

**Design principle**: Slash commands set behavioral protocols for complex multi-step interactions. Simple verb→jjx mappings go in the CLAUDE.md verb table (cheaper, no token tax).

## Still Open

- **School mechanics**: Q&A flow, chain detection, ready-vs-reined promotion. Pattern: slash command primes LLM, jjx does algorithmic analysis with directive output, LLM handles human Q&A.
- **Candidate rejection flow**: Back to ready or green? Human decides per-rejection?
- **Prance merge conflicts**: Rebase, present, or re-breeze?
- **Groom's fate**: Slash command or verb table entry in V4?

## References

- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- `Memos/memo-20260222-jjk-v4-vision.md` — Superseded original design seeds