---
description: Muster, then roster RBK and JJK heats as two space-padded plain-text blocks
---

Muster the heats and roster the `rbk-*` and `jjk-*` families as two plain-text
blocks. Reproduces the standing lineup view.

## Step 1: Ensure officium
If no officium is open this session, call `jjx_open` **alone** (never co-batched)
and capture the ☉-id. Otherwise reuse the open one.

## Step 2: Muster
Call `jjx_list` (no status filter — all heats) with the officium and your
verbatim model id.

## Step 3: Partition + sort
- **RBK heats**: rows whose silks begin `rbk-`.
- **JJK heats**: rows whose silks begin `jjk-`.
- Drop every other heat (`rbw-`, `vok-`, `apck-`, …). Do not coerce near-misses;
  if you drop something that looks like it should belong, flag it in one line.
- Sort each block alphabetically by silks (plain string order).

## Step 4: Render
Two fenced plain-text blocks, one headed `RBK heats`, one headed `JJK heats`.
Space-padded columns — **no markdown table pipes or border characters.**
Columns, in order: firemark, silks, status, done/total (e.g. `46/57`).

- Always print full firemarks with the ₣ glyph; never abbreviate (display discipline).
- Left-justify firemark and silks; status and progress follow.

Report nothing else beyond the two blocks (plus any one-line exclusion flag from Step 3).
