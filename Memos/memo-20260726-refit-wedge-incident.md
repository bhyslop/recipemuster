# Incident report: `jjx_refit` wedge and manual recovery — 2026-07-26

Written during the wrap of pace `₢B2·CAAAd` (RBK reliquary-pinning hardening),
after `jjx_refit` twice wedged for ~5 minutes and the operating agent
mishandled it. This is a dated incident record — provenance, not authority.

## Summary

At wrap time, `jjx_refit` was invoked to bring the billet current with trunk.
It ran past the 120s tool ceiling and kept running, with **no progress output**,
twice. The agent misread the silence as a deadlock and killed the running MCP
operation with `TaskStop` — which interrupted a real merge-in-progress and left
the billet in a conflicted half-merged working tree. Recovery was done by
executing refit's steps **by hand** (fetch → merge → resolve conflict → commit →
push), which succeeded and left the billet current with trunk.

**No commit was ever at risk or lost** — every commit was already pushed to
`origin` throughout, so nothing local-only ever existed to destroy.

Two root causes:
1. `jjx_refit` has no progress/timing observability, so its slow path is
   indistinguishable from a hang.
2. The agent should never have killed a running git-mutating operation. A slow
   operation is not a hung one.

## Sequence

1. Session start: a refit advisory fired (billet behind trunk); an initial
   `jjx_refit` **succeeded** (merge `55a525d19`).
2. During the pace, trunk advanced to `aebe1bb1c` (`₢Bz·BzAAD`, an ACGm_105
   word-cancer comment sweep that touched `rbfca_assembly.sh`, `rbfcg_gar.sh`,
   `rbfd_director.sh`, three RBS* adoc, and three JJK stile/farrier files). At
   wrap the operator directed "refit if needed and wrap."
3. `jjx_refit` (attempt A) ran >120s and backgrounded. A read-only git check
   showed a quiescent tree, so the agent concluded "deadlocked" and `TaskStop`'d
   it. This interrupted the merge: 9 staged trunk files + one conflict
   (`rbfca_assembly.sh`), and — misleadingly — no `.git/MERGE_HEAD`.
4. The agent refused to wrap (would have committed conflict markers, since
   `jjx_close` stages every dirty file) and refused to discard (additive-only),
   and surfaced the state to the operator.
5. Operator reset the working tree to clean HEAD (safe: all commits proven
   pushed). `jjx_refit` (attempt B) re-run; it too wedged >120s and was killed
   on operator direction, leaving the identical half-merge.
6. Operator directed manual execution of refit's steps. From clean HEAD:
   `git fetch origin` → `git merge origin/main` (**instant**; conflicted on
   `rbfca_assembly.sh`) → conflict resolved → 2-parent merge commit `e2802c4b2`
   → shellcheck (240 clean) + theurge build green → `git push`.
7. Billet now current with trunk (ahead 12, behind 0), tree clean.

## The conflict (for the record)

The single conflict was in the pinning function itself. The merge-base had one
`zrbfc_resolve_tool_images()`. This branch's lineage (commit `69d1e8c47`) had
**split** it into `zrbfc_resolve_tool_images_from(touchmark)` (the pinning entry
point) plus a thin `zrbfc_resolve_tool_images()` wrapper that reads
`RBRV_RELIQUARY` and delegates. Trunk's BzAAD sweep had only **reworded the
comment** on the old monolithic function. Resolution: keep this branch's side —
it already subsumes trunk's change (the wrapper preserves the `RBRV_RELIQUARY`
behaviour), and trunk's comment described a code shape that no longer exists.
The resolved file is byte-identical to HEAD; no trunk content of value was lost.

## Root cause and the observability gap

- The manual `git merge` was instant, so the ~5 minutes was **not** the merge.
  It was JJK-refit-specific: `glean` (fetch) and `consign` (push) both ride the
  bounded-retry "vedette" (`ZJJRFG_VEDETTE_DEADLINE` = 30s, backoffs `[2s, 5s]`
  → ~97s worst case per remote op), plus whatever JJK-server overhead the refit
  wrapper carries (officium / studbook / lock work). **The exact time sink was
  never definitively identified — because there is no timing or progress logging
  in the refit path to reveal it.** That absence is the core finding.
- Production JJK is almost uninstrumented: 6 `eprintln!` (all in `jjrm_mcp.rs`,
  command-level dispatch only) and the only real timing (`Instant::now` /
  `elapsed()`) lives in the **test** file `jjtfg_plaingit.rs`, not the live
  path. The git-execution boundary (`jjrfg_plaingit`'s `zjjrfg_run_git` /
  `_bounded` / `_remote`) emits nothing about which step it is on or its
  duration.
- `zjjrfg_run_git` (`jjrfg_plaingit.rs`, the non-bounded runner) uses `.output()`
  with **no deadline** — an unbounded hang risk for the many local git calls
  (`rev-parse`, `merge-base`, `status`, the `merge --ff-only` in `advance`).
- Per the `jjrfg_plaingit` module header, a merge conflict at `enfold` "panics
  and leaves conflict markers standing" — a conflict is *meant* to fail fast, so
  the slowness is elsewhere (fetch / push / overhead), not the conflict.

## The worktree `MERGE_HEAD` gotcha

The agent twice checked `test -f .git/MERGE_HEAD`, saw it absent, and wrongly
concluded no merge was in progress (hence "the half-merge can't be completed as
a proper 2-parent merge"). But the billet is a git **worktree** — `.git` is a
file pointing at `rbm_alpha_recipemuster/.git/worktrees/jjqb_200221_CAAAd`, so
`MERGE_HEAD` lives in the worktree's gitdir, not `.git/MERGE_HEAD`. The correct,
worktree-safe check is `git rev-parse -q --verify MERGE_HEAD`. The interrupted
refit had most likely left a completable merge; the filesystem misread hid it.

## Lessons

1. **Never kill a running JJ or git-mutating operation mid-flight.** Wait, or
   surface and ask — killing it is what manufactured the conflicted state, twice.
2. **Check merge state via git, not the filesystem** (`git rev-parse MERGE_HEAD`),
   which is worktree-correct.
3. **The additive-only discipline held and mattered.** The agent ran no
   forbidden command; the operator owned each `reset --hard HEAD`, which — with
   all commits pushed — moved the branch pointer zero distance and discarded only
   re-fetchable trunk content.

## Recommendation (currently unhomed)

Instrument `jjx_refit` / the `jjrfg_plaingit` git-execution boundary: emit the
running step (`glean` / `enfold` / `consign`) and its elapsed time, so a slow
refit is diagnosable instead of indistinguishable from a hang; and consider
bounding the unbounded `zjjrfg_run_git`. This is JJK tooling
(`jjrrf_refit` / `jjrfg_plaingit`) and belongs in a JJK heat. A scout on
2026-07-26 found **no existing pace** for this work — the operator's recollection
of a prior logging discussion did not resolve to a slated pace.
