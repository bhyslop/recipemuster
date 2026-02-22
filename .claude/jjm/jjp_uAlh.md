# Paddock: jjk-v4-vision

## Context

Next-generation Job Jockey architecture. Major breaking redesign introducing a middle
layer ("leg") between heats and paces, git worktree isolation, tiered model dispatch,
proactive autonomous agents, and JJ as context assembler.

This heat holds no paces yet — it exists as a landing zone for continued design
discussion before implementation planning begins.

## Origin Chat

**cchat-20260222-gallops-at-dawn** — Two-session design conversation (2026-02-21 evening
through 2026-02-22 morning). Sonnet drove session one; Opus drove session two.
Model switch was undetected mid-conversation.

## References

- `Memos/memo-20260222-jjk-v4-vision.md` — Full design seeds from cchat-20260222-gallops-at-dawn
- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — Current V3 data model (what V4 replaces)

## Key Design Seeds

1. **Leg** as collaborative episode (heat > leg > pace)
2. **Chalk retired** — ceremony that didn't pay off
3. **Pace state simplification** — decompose "bridled" into state + orchestration + dispatch
4. **Git worktrees** — per-leg (collaborative) and per-pace (proactive candidates)
5. **Gallops stays global** — never forked into worktrees
6. **Tiered dispatch** — haiku scout, sonnet implement, opus decide
7. **Action templates** — pre-defined leg patterns with context specifications
8. **Proactive agents** — background scanning for unattended bridled work
9. **No daemon** — git-mediated communication, scan at natural transition points
10. **Precise context assembly** — JJ provides targeted context per pace, not kitchen sink
