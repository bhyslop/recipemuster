---
description: Emit two parallel-dispatch waves for heats ₣B_ and ₣CA — wave 1 now, wave 2 after
---

Interim stand-in for the parallel-wave advisor door (that door builds
post-inversion; this command retires when it lands). Read-only: this command
**never saddles and never mutates state** — it emits a dispatch pick and nothing
else. Acting on the pick stays the operator's move.

Scope is fixed and hardcoded: heats **₣B_** and **₣CA** only. There is no
heat-list argument.

## Step 1: Ensure officium
If no officium is open this session, call `jjx_open` **alone** (never co-batched)
and capture the ☉-id. Otherwise reuse the open one.

## Step 2: Pull fresh groom material through the JJ interface
Never cached state, never raw storage. Write two halter notices to the
officium's `gazette_in.md` (path from `jjx_open`):

```
# jjezs_halter ₣B_
# jjezs_halter ₣CA
```

Then call `jjx_show {"remaining": true}` with the officium and your verbatim
model id, and **read `gazette_out.md`** for the two paddocks and every remaining
pace docket. The paddocks carry the cinches the prune depends on; read them, not
just the dockets.

## Step 3: Assemble candidates from heat order
Single-operator workflow runs each heat's paces in heat order, so the only pace
of a heat that can be dispatched *now* is its **next actionable pace** — the
first remaining (non-abandoned, non-bridled-out) pace in that heat's order. The
candidate set is therefore at most two paces: ₣B_'s next actionable pace and
₣CA's next actionable pace.

## Step 4: The cross-heat prune (the whole judgment)
Point at the parallel-wave advisor door pace's doctrine (₢B_·CAACb) — it is the
authority; the criteria below are its working restatement, not a re-derivation.
Two candidate paces are **contaminated** (cannot run concurrently) if any holds:

- **Wrap-time convergence overlap** — their wraps would collide or one wrap's
  convergence would land atop the other's.
- **Semantic contamination** — one pace's landing invalidates the other's
  premise (its docket assumes a state the sibling's land would change).
- **Studbook same-document seam, at file grain** — two paces whose spec accords
  touch the **same studbook-resident document** are contaminated. Billet
  isolation removes the work-repo concurrent-edit hazard, so **work-repo file
  overlap is not a contamination source** — only the shared studbook store is,
  and while the barebones spec-commit posture stands the grain is the file.

If the two candidates are **not** contaminated → both ride wave 1.
If they **are** → the higher-heat-order / less-blocking one rides wave 1 and the
other drops to wave 2.

## Step 5: Compose the two waves
- **Wave 1** — the candidate paces that survive the prune, dispatchable now in
  parallel.
- **Wave 2** — dispatched only after **every** wave-1 pace completes. It holds:
  the pace after each wave-1 pace in that heat's own order (it waits on its
  heat-mate), plus any candidate the prune bumped from wave 1.

Wave 2 is the near sketch, not an exhaustive plan — one pace per heat is enough.

## Step 6: Render — two waves, nothing else
Emit exactly two labeled waves. For **each** pace name:
- its **live-qualified coronet** — `₢` + heat firemark + `·` + body
  (e.g. `₢B_·CAAC8`); never a bare body, never abbreviated (display discipline);
- its **silks**;
- its **bridle tier** — `bridled opus` / `bridled sonnet` / `bridled haiku`
  (with the effort word if one is set), or `rough` if unbridled. This is a soft
  warning for how the operator monitors the session, not load-bearing.

One line of contamination rationale per pace is permitted where it explains a
wave placement; **no other commentary** — no preamble, no summary, no
next-steps. The two waves are the whole output.
