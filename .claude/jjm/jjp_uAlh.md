# Paddock: jjk-v4-vision

## Current Design

**Authoritative**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline. Decisions from cchat-20260224-gaits-and-breezes (Opus).

Key decisions: no leg layer, branches instead of worktrees, composable gaits in gallops registry, three-phase pipeline (school → breeze → prance), warrant as LLM-to-LLM communication, bridled/quarter/chalk eliminated.

## Open Items — Prioritized

**Decide now** (architectural, blocks implementation paces):
1. Tack schema changes — foundation, every command touches tack fields
2. School mechanics — centerpiece new command, pipeline viability
3. Candidate rejection flow — state machine shape
4. Breeze dependency analysis — branch strategy
5. Prance merge conflict handling — candidate serialization

**Work out later** (mechanical, falls from above):
6. Gait CRUD commands — minting exercise
7. Mount rendering — presentation
8. Gait switching command — tiny UI call
9. Orient changes — follows from state machine
10. Migration from V3 — blocked by schema decisions

## Prior Design Seeds

**Superseded**: `Memos/memo-20260222-jjk-v4-vision.md` — Original design seeds from cchat-20260222-gallops-at-dawn (Sonnet + Opus, two sessions). The new memo documents which elements were retained, refined, or retired.

## References

- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — Current V3 data model (what V4 replaces)