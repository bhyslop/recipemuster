# JJK V4 Vision: gallops-at-dawn-260222

Design seeds from a two-session conversation (2026-02-21 evening through 2026-02-22 morning).
Sonnet drove the first session; Opus drove the second. The model switch was undetected,
which became evidence for one of the conversation's own theses.

## The Leg: New Middle Layer

**Core insight**: Paces are too small for the real unit of collaborative work. Heats are too large.
A new layer — the **leg** — sits between them.

- **Heat**: bounded initiative (unchanged)
- **Leg**: collaborative episode containing multiple paces, with explicit human/LLM handoff points
- **Pace**: atomic step (reclaimed as the smallest unit — edit these files, run this build, etc.)

The endurance racing metaphor: legs have mandatory vet checks (human review) between them.
The human assesses before the next leg begins. The leg goal holds narrative continuity
even when individual paces get reslated mid-leg.

### Leg Lifecycle

A leg is nominated with a goal and rough initial paces — knowing paces will evolve
during execution. Upfront-perfect-planning is fiction; the leg accommodates iterative discovery.

### Richer Mount

Mount becomes leg-scoped: shows the leg goal at top, done paces as brief compact summaries
(what was done, what commits), current pace in full detail. Done work provides context
without demanding equal attention.

## Chalk Retired

Chalking (APPROACH markers) hasn't paid off. The warrant already IS the proposed approach.
Chalk added ceremony to a moment that doesn't exist as a distinct handoff.
Drop cleanly in V4.

## Pace State Machine Rework

V3 "bridled" conflated four concerns:
1. Specification is clear enough to act on
2. Warrant has been written
3. Autonomous execution is authorized
4. Eligible for proactive pickup

V4 decomposes these across layers:
- **Pace state**: just progress (pending → active → done / dropped)
- **Leg**: carries orchestration (attended/unattended, worktree, merge strategy)
- **Dispatch metadata**: execution guidance (model tier, brief, prior pace output)

## Git Worktrees for Isolation

Worktrees replace the advisory file locking concept entirely.

### Two Worktree Patterns

| | Leg worktree | Pace worktree |
|---|---|---|
| **When** | Active collaborative work | Background proactive work |
| **Scope** | Multiple paces share it | One pace, one worktree |
| **Human** | In the loop between paces | Reviews finished candidate |
| **Merge** | Within the leg (merge pace) | Separate review decision |
| **Failure** | Handle during leg, full context | Discard worktree, zero cost |

### Zero or One Worktree Per Leg

Not every leg needs a worktree. Solo leg on a quiet repo → work on main.
The worktree is a tool reached for when concurrent work demands it.

### Merge as a Pace Within the Leg

Merge back to main happens WITHIN the leg while context is still hot.
The agent and human who did the work understand intent — that's when to resolve conflicts,
not later. The leg wraps on main, clean. Worktree is scaffolding removed before completion.

### Parallel Legs Within a Heat

Two legs in the same heat can run in parallel with separate worktrees.
Merge ordering is a human judgment call. More likely to touch related files
than cross-heat parallelism, but worktrees handle the isolation.

## Gallops Stays Global

**Critical architectural constraint**: gallops.json is coordination state, not code.
It must NEVER fork into worktrees.

- JJX always reads/writes the main worktree's `.claude/jjm/jjg_gallops.json`
- `git update-ref` locking works because `.git` is shared across worktrees
- V4 separates commit streams: code commits on worktree branches, gallops commits on main
- Steeplechase markers link them by coronet/firemark identity
- Merge-pace reunites them in main's history

## Proactive Agents

An orchestrator scans for work eligible for background autonomous execution.

### Guards (all must be true):
1. Heat must be `unattended` (human permission — possibly leg-level, not heat-level)
2. Heat must be `racing`
3. Pace must be ready for autonomous execution
4. No conflicting work in flight

### Mechanism:
- Per-pace worktrees for proactive work (candidates, not committed results)
- Human reviews finished worktrees and decides what merges
- Discarding bad results is free — nothing touched main

### No Daemon:
Git repo is the communication channel. Proactive scanning happens at natural transition
points (mount-time checks, explicit "go find background work" gestures, or at most
cron-triggered). The complexity budget is better spent on the leg model.

## Tiered Model Dispatch

Different paces have different intelligence requirements:

- **Scout paces** (haiku): identify relevant files, rough scope
- **Implementation paces** (sonnet): substantive code work
- **Decision paces** (opus): hard architectural choices, complex debugging
- **Verification paces** (haiku): run builds, run tests

### Action Templates

Pre-defined leg patterns encoding accumulated wisdom about how work usually unfolds:
- `scout → implement → verify` (haiku → sonnet → haiku)
- `diagnose → fix → test` (sonnet → sonnet → haiku)
- `design → implement → review` (opus → sonnet → human)

Templates define both workflow AND context specification — each pace type knows
exactly what context it needs.

## JJ as Context Assembler

**Reframe**: JJ is not just a task tracker. It's the system that assembles the right
context for each pace type.

### Precise Context vs Kitchen Sink

Instead of loading all of CLAUDE.md for every interaction, JJ could provide targeted
context per pace:
- Scout pace: leg goal, file tree, nothing else
- Implement pace: scout output, relevant file contents, coding conventions
- Verify pace: what changed, how to check, what success looks like

Tight precise contexts let lighter models punch above their weight.
The "200 lines of examples beats 600 lines of rules" finding supports this —
activate rather than instruct.

## Leg Identity

Legs probably need their own identity type (like firemarks for heats, coronets for paces).
The orchestrator needs to reference legs; humans need to say "I'm working on this leg."
Vocabulary and encoding TBD.

## Open Questions

- Horse racing vocabulary for new pace states and leg states
- Leg identity encoding (₤? Something else?)
- How does `orient` work with legs? Does it present "which leg?" or auto-select?
- Can the proactive orchestrator suggest splitting remaining paces into parallel legs?
- Optimal boundary between leg-level and pace-level metadata
- Interaction between action templates and the existing warrant/docket model

## Meta-Observations

From the conversation itself, not just the technical content:

- **Model handoff as evidence**: Sonnet-to-Opus switch was undetected, validating
  that the Claude model family shares deep enough structure for seamless handoffs
- **Convergence is real**: Long conversations accumulate shared understanding that
  genuinely refines the invocation. RAG-style context dropping destroys this
  ("Maude" vs Claude — same model, different entity due to context mismanagement)
- **The collaboration structure is a first-class engineering problem**: Job Jockey
  is a theory of human-AI collaboration expressed as software
- **Kindness as discipline**: Shapes the collaboration differently than efficiency alone
