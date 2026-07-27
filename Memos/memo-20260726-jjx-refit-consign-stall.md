# Incident: jjx_refit stalled between enfold and consign (second wedge, same day)

**Date:** 2026-07-26
**Reporter:** Claude Fable 5 (dispatched session, officium ☉260726-1056-pbzd)
**Context:** Billet `jjqb_200312_CAABz`, branch `personal/bhyslop/jjls_pace/CAABz`,
pace ₢Bz·CAABz (reliquary-cohort-one-package-collapse) worked/notched/landed.
`jjx_refit` invoked to clear the wrap staleness gate. The call never returned;
operator ruled it permanently stalled and directed kill + memo.

**Sibling incident:** `memo-20260726-jjx-record-mcp-hang-incident.md` (officium
☉260726-1051-uqs5, same day, different billet). Same outer signature — this memo
records where the signatures **differ**.

---

## Timeline (PDT; harness log times are Z = PDT+7)

| Clock | Event | Source |
|---|---|---|
| 22:20:11 | `jjx_refit` enters the `vvx mcp` server | harness MCP log `2026-07-27T04-11-16-103Z.jsonl`, entry `05:20:11.663Z` |
| 22:20:11 | officium `heartbeat` touched — the only touch, entry-only as in the sibling incident | mtime of `officia_scratch/260726-1056-pbzd/heartbeat` |
| 22:20:13 | **enfold complete**: merge commit `33acd8159 "enfold trunk"` created on the billet branch | `git log -1 --format=%ci 33acd8159` |
| 22:20:13 → 22:24:30 | silence — no response, no progress | harness log `still running (30s…240s)` |
| 22:24:30 | operator-directed kill (TaskStop → MCP abort, `failed after 258s: AbortError`) | harness log |

## What completed and what did not

Refit is glean (fetch origin) → enfold (merge `origin/main`, never rebase) →
consign (push the billet branch). Observed:

- **Glean + enfold: done, and fast.** Two seconds from call entry to the merge
  commit — including the networked fetch. Working tree clean after, the pace's
  edits intact through the merge (verified by grep of both rbgjl residue lines).
- **Consign: never happened.** `git ls-remote origin` shows no
  `refs/heads/personal/bhyslop/jjls_pace/CAABz` at kill time (checked twice,
  including after the kill). The remote has only the case-sibling `…/CAABZ`.
- **No live git child** during the stall window: `ps` showed the `vvx mcp`
  server (PID 25920) with no `git` subprocess and no `jjx_dispatch` worker.
  So the block was **in-process**, in the ~257 s between finishing the merge
  and (never) spawning the push — not a hung network push.

## How this differs from the jjx_record incident

The sibling incident's command **finished its work** and hung purely on the
return path (commit landed, then silence). This one hung **mid-sequence**:
step 2 of 3 complete, step 3 never began. Whatever wedges, it can bite
*between* engine steps, not only after the answer is composed. A
return-path-only theory (e.g. response serialization, MCP transport) does not
cover this one.

**Mechanism (settled post-memo by ₢CAAB4's code-read; supersedes this memo's
original "in-engine wait" phrasing):** not a wait — a **panic**. rmcp 0.16.0
spawns each handler task with no `catch_unwind`, and JJK panics by design on
any unclassified git failure; the request task **panicked and died** at an
in-engine failure between enfold and consign (plausibly a
case-collision-induced ref anomaly in consign-prep — see below), stranding the
call. A dead task, not a blocked one — which fits every observable here:
0.0% CPU, no live git child, no response ever, and the same signature on both
the mid-sequence and after-work stalls (the crossing point both paths share is
a panic site, not a lock).

Concurrency (jjx_open reported `Exsanguination: 167 active`; ~10 concurrent
dispatched sessions in `ps`) is context, not cause; the one-touch-at-entry
heartbeat carrying no phase information remains why none of this was visible
from durable files.

Also of note: **this session saw the same wedge signature three times** —
`jjx_record` (stalled ≥120 s; commit landed; task killed later, work verified
complete), `jjx_landing` (stalled 1800 s to harness idle-timeout; landing
commit `d4f7a29ca` landed anyway), and now `jjx_refit`. In the first two the
work completed and only the answer was lost; refit is the first observed
*partial* execution. `jjx_enroll` and `jjx_open` in the same officium returned
normally, though `jjx_enroll` — issued between the stalls — returned in
seconds. Stall onset is not per-command-type deterministic.

## Diagnosability note — the repair exists but was not aboard

Trunk commit `5000d8d6e` (₢CAAB2, wrapped 21:23 today) adds exactly the
narration this incident needed: GIT-OPEN/GIT-OUTCOME per git child into the
officium's `sectional.log`, bounded local-runner deadlines, vedette-retry
legibility. The stalled server's binary
(`rbm_alpha_recipemuster/Tools/vvk/bin/vvx`) was **built 13:07**, eight hours
before that wrap — so no `sectional.log` exists in this officium and the wedge
is again unlocalizable from durable files. Two implications:

1. This incident adds no evidence the ₢CAAB2 repair is insufficient — it was
   simply not running. Long-lived `vvx mcp` servers keep their launch-time
   binary; every dispatched session in `ps` predates the repair.
2. However, the repair narrates **git children**. This stall sat *between*
   git spawns — if the wedge is in a non-git wait (lock, blotter, studbook
   bookkeeping), GIT-OPEN/GIT-OUTCOME brackets will show "hung between
   OUTCOME and next OPEN" but not *where*. The sibling memo's per-phase
   progress recommendation (heartbeat re-touch / durable per-command trace)
   remains the missing piece for this class.

## State left behind (deliberate; nothing discarded)

- Billet branch local tip: `33acd8159` (enfold merge). Tree clean. The pace's
  landed work and the merge are both intact.
- **Refit is half-applied**: merged but unpushed. The wrap staleness gate
  compares against trunk's remote counterpart — whether a merged-but-unpushed
  billet passes is untested here.
- The obvious completion (`git push origin <branch>`) is **deliberately not
  taken**: pushing refspec `…/CAABz:…/CAABz` mints a new remote branch beside
  the existing `…/CAABZ` — the APFS case-collision documented this session
  (two live billets, ₢CAABz / ₢CAABZ, silently sharing one loose-ref file;
  fix slated as ₢B9·CAAB- billet-branch-case-armor). Publishing the second
  spelling is exactly the escalation held for an operator ruling.
- The `vvx mcp` server for this session (PID 25920) was left running; only the
  harness-side task was killed. Its next jjx call's behavior is unknown.

## Questions for the advising chat

1. Complete the consign by hand (`git push origin
   personal/bhyslop/jjls_pace/CAABz`), accepting the case-sibling remote
   branch — or resolve the CAABz/CAABZ fusion first and push once, under
   whatever name survives?
2. Is a merged-but-unpushed billet wrappable (does the staleness gate read the
   local merge base or the remote ref), or is the wrap blocked until consign?
3. Should the long-lived pre-repair `vvx mcp` servers be recycled now so the
   ₢CAAB2 narration is actually aboard for the next occurrence?

## Post-memo ruling and disposition (same night)

The advising chat answered, and the operator ruled:

- **Q1 — neither.** Do not push the billet ref under either spelling, and do
  not retry the transport (refit re-wedges deterministically; a wrap attempt
  is another transport call likely to strand). Instead the verified content
  commit was carried to main **off the transport**: provenance confirmed
  (`8347f3f40` is exactly the two-line collapse with the `₢CAABz:n` trailer;
  the landing commit is chalk-only), then cherry-picked onto a throwaway
  landing branch cut from the current `origin/main` tip. The billet's
  never-rebase rule does not bind a landing branch. Operator runs the push.
- **Q2 — mooted** by the cherry-pick path; wrap and ledger reconciliation
  deferred.
- **Q3 — reasonable but not needed** to protect the work; the cherry-pick
  bypasses the wedged server entirely.
- Consign, wrap, ledger reconciliation, and the CAABz/CAABZ fusion untangle
  all **wait until ₢CAAB4 (fail-loud) and ₢B9·CAAB- (case-armor) land**.
- Follow-on paces from tonight's incidents: ₢CAAB4 (panic containment /
  fail-loud transport), ₢CAAB5 (per-phase progress — this memo's missing
  piece), ₢B9·CAAB- (billet-branch case-armor).
