---
description: Emit two parallel-dispatch waves for heat ₣B_ — wave 1 now, wave 2 after
---

Interim stand-in for the parallel-wave advisor door (that door builds
post-inversion; this command retires when it lands). Read-only: this command
**never saddles and never mutates state** — it emits a dispatch pick and nothing
else. Acting on the pick stays the operator's move.

Scope is fixed and hardcoded: heat **₣B_** only. There is no heat-list
argument.

The pick is governed by **blast radius**, not heat-order seriality. A single
operator runs one heat's paces in heat order, but this command exists to feed
*parallel chats*, where heat order stops constraining concurrency and only
genuine inter-pace dependency does. So a heat is not a serial lane: two or more
of its remaining paces may ride the same wave when nothing real gates them
against each other.

## Step 1: Ensure officium
If no officium is open this session, call `jjx_open` **alone** (never co-batched)
and capture the ☉-id. Otherwise reuse the open one.

## Step 2: Pull fresh groom material through the JJ interface
Never cached state, never raw storage. Write a halter notice to the officium's
`gazette_in.md` (path from `jjx_open`):

```
# jjezs_halter ₣B_
```

Then call `jjx_show {"remaining": true}` with the officium and your verbatim
model id, and **read `gazette_out.md`** for the paddock and every remaining
pace docket. The paddock carries the cinches the dependency read *and* the prune
depend on; read it, not just the dockets.

## Step 3: Read each remaining pace's blast radius
Consider **every** remaining (non-abandoned, non-bridled-out) pace in the heat
— not just the front of the heat. For each, read two things from its docket and
the heat's paddock cinches:

- **Hard dependencies** — a predecessor pace whose land this pace's premise
  assumes. Heat order is *not* itself a dependency: only a genuine inter-pace
  dependency gates a pace — a docket that names a predecessor coronet, or a
  paddock cinch that sequences one pace after another ("runs only after X
  lands", "sequenced, never concurrent"). Two paces of one heat with no
  dependency between them are both live.
- **Touched stores and documents** — which stores its land writes (a billet, the
  studbook) and, in the studbook, which document(s). This is the raw material
  for the contamination prune below.

A pace is a **wave-1 candidate** when every hard dependency it names has already
landed — none of them sits among the remaining paces. There may be several
candidates.

## Step 4: The contamination prune (the whole cross-candidate judgment)
Point at the parallel-wave advisor door pace's doctrine (₢B_·CAACb) — it is the
authority; the criteria below are its working restatement, not a re-derivation.
Two candidates are **contaminated** (cannot ride the same wave) if any holds:

- **Wrap-time convergence overlap** — their wraps would collide or one wrap's
  convergence would land atop the other's.
- **Semantic contamination** — one pace's landing invalidates the other's
  premise (its docket assumes a state the sibling's land would change), *or*
  both paces own the **same mutation over a shared surface** — both repoint the
  same citations, both rewrite the same rules, both edit the same span. Billet
  isolation makes *disjoint* concurrent edits to a shared work-repo file
  trivially mergeable, and disjoint overlap alone is never contamination — but
  it does **not** make two sessions owning the same change coherent. The test is
  the merge: a mechanical rebase is trivial and clears; a merge that demands
  reconciliation judgment is an interaction product. Sequence them.
- **Studbook same-document seam, at file grain** — two paces whose spec accords
  touch the **same studbook-resident document** are contaminated: their wraps
  converge on the shared studbook store. Different studbook documents are not —
  additive git merges them. While the barebones spec-commit posture stands the
  grain is the file.

Run this test **pairwise across the full candidate set**. Do not stop at file
identity: ask whether the two landings own the *same change*, and whether their
merge is a rebase or a reconciliation.

## Step 5: Compose the two waves
- **Wave 1** — a maximal set of candidates that are mutually uncontaminated.
  Start from every candidate; wherever a pair is contaminated, hold back the one
  whose deferral costs less — the pace that unblocks the fewer downstream paces,
  ties broken toward the later position in its heat's order — so its sibling
  rides now. Every survivor is dispatchable immediately, in parallel.
- **Wave 2** — dispatched only after **every** wave-1 pace completes. It holds:
  for each wave-1 pace, the next pace its land unblocks (a heat-mate that named
  it as a hard dependency, or the next actionable pace in that heat), plus any
  candidate the prune bumped from wave 1.

Wave 2 is the near sketch, not an exhaustive plan — a pace or two is enough.

## Step 6: Render — two waves, nothing else
Emit exactly two labeled waves. Each wave is a **single flat list** — the
parallelism frontier does not run along heat order, so grouping paces by
their position in the heat would reimpose the very seriality this command
rejects. Order each wave by blast radius: the paces that unblock the most
downstream work first. For **each** pace name:
- its **live-qualified coronet** — `₢` + heat firemark + `·` + body
  (e.g. `₢B_·CAAC8`); never a bare body, never abbreviated (display discipline);
- its **silks**;
- its **bridle tier** — `bridled opus` / `bridled sonnet` / `bridled haiku`
  (with the effort word if one is set), or `rough` if unbridled. This is a soft
  warning for how the operator monitors the session, not load-bearing.

One line of contamination or dependency rationale per pace is permitted where it
explains a wave placement; **no other commentary** — no preamble, no summary, no
next-steps. The two waves are the whole output.
