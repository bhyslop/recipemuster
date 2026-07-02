# Incident Report — JJK gallops coronet collision `₢BfAA3` (2026-07-02)

**Date:** 2026-07-02
**Clones involved:** `rbm_alpha_recipemuster` (this instance, officium `☉260702-1001-punb`, Opus 4.8) and `rbm_beta_recipemuster` (Fable)
**Heat:** `₣Bf` (rbk-00-mvp-review-federation-build)
**Colliding coronet:** `₢BfAA3`
**Resolution commit:** `0ecd309c0` (merge, on `origin/main`)
**Severity:** blocked push convergence across clones; no data lost; resolved same day.

Provenance note: this memo is historical record. The durable JJK lessons in
§8 must migrate to a spec/itch home to become authoritative — a memo is a nudge,
never authority (see CLAUDE.md "Retired Memos").

## 1. Summary

Two officia working the same heat `₣Bf` in two clones independently allocated the
*same* next-free coronet `₢BfAA3` for two unrelated paces:

- **alpha:** `₢BfAA3` = `affiance-jilt-folio-recut` (the folio-addressing work, wrapped/complete this session).
- **beta:** `₢BfAA3` = `terrier-noun-sheaf-carve` (a fresh, rough, unstarted spec-relocation pace).

The collision surfaced only at `git pull --rebase` as a merge conflict inside
`.claude/jjm/jjg_gallops.json` — not a text merge, but a genuine *identity* collision:
one coronet key naming two distinct paces. Resolution took three coordinated moves:
(a) Fable, in beta, evacuated the colliding coronet via an exotic **heat-tombstone
transfer** and re-coroneted the terrier pace to `₢BfAA4`; (b) alpha converged via a
**merge** (not a rebase); (c) the residual gallops conflict was **hand-edited under
explicit one-off operator authorization**, because JJK offers no non-manual path for a
hard gallops conflict today. Everything verified green and pushed.

## 2. Root cause

**Distributed next-free coronet allocation across divergent clones.** A coronet is
`firemark + 3-char suffix`, allocated as "next free" from a per-heat seed
(`jjghn_next_pace_seed`). When two clones diverge from a common ancestor whose `₣Bf`
seed was at `AA3`, and each independently slates a new pace, **both deterministically
pick `AA3`** — there is no cross-clone reservation or per-officium namespacing. The
allocator is correct locally and collision-prone globally.

Contributing factors that turned a rare collision into a painful convergence:

- **Rebase re-fights gallops per commit.** `git pull --rebase` replays every
  gallops-touching commit (slate → notch → wrap), each re-conflicting on the JSON, with
  no tool able to finalize mid-rebase (see §7).
- **No gallops merge driver.** `.gitattributes` registers none, and `.git/config` has no
  `merge.*` driver, so git falls back to a line-based merge that cannot understand gallops
  structure.
- **`jjx_validate` cannot parse a marker-mangled file.** It reads the working-tree
  gallops; git conflict markers make it invalid JSON, so it reports `broken` rather than
  finalizing (see §7, §9).

## 3. Timeline

1. **alpha** completed `₢BfAA3 = affiance-jilt-folio-recut`, notched and wrapped it, and
   attempted `git pull --rebase && git push`.
2. First rebase hit a **code** conflict in `rbtdrv_patrol.rs` (the ₣Bl provider-grain
   fixture re-cut, landed on origin, colliding with alpha's mechanical folio-pass). This
   was resolved cleanly — took the provider-grain structure, re-applied the folio-pass —
   and pushed to that point. (Distinct from the coronet collision; recorded here only as
   context.)
3. Push rejected — origin had advanced (beta pushed `₢BfAAy` terrier-muniment work and
   the `terrier-noun-sheaf-carve` slate).
4. Second `git pull --rebase` hit the **gallops** conflict on alpha's `₢BfAA3` slate.
   Inspection revealed the *identity collision*: `₢BfAA3` = affiance (alpha) vs
   `₢BfAA3` = terrier-noun-sheaf-carve (beta).
5. alpha aborted the rebase, ran a **gallops repair** (`git rebase --abort` +
   `jjx_validate` clean), and prepared an incident prompt for Fable in beta.
6. **Fable** dispositioned the collision in beta (see §5) and pushed.
7. alpha's next `git pull --rebase` *still* conflicted on gallops (rebase re-fights it),
   so alpha aborted and switched to a **merge** (see §6). The residual gallops conflict was
   hand-resolved under operator authorization; `jjx_validate` clean; merge committed
   (`0ecd309c0`), verified, pushed.

## 4. Diagnosis steps (alpha)

- `git fetch` + `git log HEAD..origin/main` to see the incoming commits.
- Read the gallops conflict regions directly (operator-directed). Identified it as an
  *identity* collision, not a text merge: same `₢BfAA3` key, two different `jjgtn_silks`.
- `git diff --name-only <merge-base> origin/main` to scope which of alpha's files the
  incoming commits touched (only `acronyms.md`, `rbgp_payor.sh` — small).
- Checked `git config rerere.enabled` (off) and `.gitattributes` for a gallops merge
  driver (none).
- Confirmed the collision source: inspected the sibling clone `../rbm_beta_recipemuster` —
  its HEAD *was* `origin/main`'s tip, and it held the `terrier-noun-sheaf-carve` slate
  that minted the colliding `₢BfAA3`.

## 5. Fable's resolution — the heat-tombstone transfer (the "exotic" move)

The naive fix — `jjx_drop` the colliding pace — risks leaving an **abandoned/tombstone
marker at `₢BfAA3` inside `₣Bf`**, which could block alpha from re-adding `₢BfAA3` for its
own pace. Fable avoided that by *evacuating the coronet out of the heat's namespace
entirely*:

1. **Re-established** `terrier-noun-sheaf-carve` fresh in `₣Bf`, which took the next free
   coronet `₢BfAA4` (rough) — the real work now lives here.
   (commits `2598c280b` `₣Bf:S`, `e849aa7f6` / `f0b2389cf` `₣Bf:T`)
2. **Nominated a throwaway heat** `₣Bp` = `bfaa3-collision-tombstone`.
   (commit `3a42d41e0` `₣Bp:N`)
3. **Transferred the colliding pace `₢BfAA3 → ₢BpAAA`** — moving it into the tombstone
   heat. Because transfer re-keys a pace's coronet to the destination heat's firemark, this
   *removed `₢BfAA3` from `₣Bf`'s live namespace* (the pace becomes `₢BpAAA`, marked
   abandoned, noted "Drafted from ₢BfAA3 in ₣Bf").
   (commit `d80a8087e` `₣Bp:D  ₢BfAA3 → ₢BpAAA`)
4. **Retired** `₣Bp` (`jjx_archive`) — the tombstone heat and its evacuated corpse move to
   `.claude/jjm/retired/jjh_b260702-r260702-bfaa3-collision-tombstone.md`.
   (commit `35281478f` `₣Bp:R`)

Net effect on origin: `terrier-noun-sheaf-carve` lives at `₢BfAA4`; `₢BfAA3` is *free* in
`₣Bf`, with no blocking marker; the collision "corpse" is buried in a retired heat.

**Why it is clever:** a transfer-to-a-graveyard-heat evacuates a coronet *key* cleanly,
where a drop-in-place would leave a tombstone in the heat that still owns the key. It uses
the ordinary `nominate → transfer → retire` verbs to synthesize a "delete this coronet from
the heat" operation JJK does not offer directly.

## 6. Alpha's convergence

With `₢BfAA3` freed on origin, alpha still could not rebase (see §7). Instead:

1. `git merge origin/main` — code auto-merged (`acronyms.md`, `rbgp_payor.sh`,
   `rbtdrv_patrol.rs`); only the gallops conflicted, now under `MERGE_HEAD`.
2. **Hand-edited the gallops** (operator-authorized one-off): removed the conflict markers,
   retaining *both* paces — `₢BfAA3` = affiance (complete) and `₢BfAA4` = terrier (rough) —
   and reconciled `jjghn_next_pace_seed` to `AA5` (both AA3 and AA4 now allocated). Verified
   valid JSON.
3. `jjx_validate` → **clean/canonical** (the hand-merge happened to be canonical, so no
   rewrite).
4. Completed the merge commit `0ecd309c0` (2 parents), verified: theurge build (no regen
   drift), shellcheck, fast-qualify, theurge-unit (156), reveille (116) all green; eyeballed
   the `₣Bf` inventory (`jjx_coronets` + `jjx_brief` on both) — both coronets present, correct
   identities, no swap or dupe.
5. Pushed (fast-forward).

## 7. Why rebase failed and merge worked

`jjx_validate`'s "finalize any in-progress merge" behavior keys off git's `MERGE_HEAD`.

- **Rebase** never creates a `MERGE_HEAD`; it replays commits one at a time. Each of
  alpha's three gallops-touching commits (slate, notch, wrap) re-conflicts on the JSON, and
  there is no `jjx` hook to finalize a rebase step. The `git pull --rebase` habit therefore
  fights JJK's convergence model head-on for any gallops divergence.
- **Merge** converges the gallops *once*, under `MERGE_HEAD`, which is the state
  `jjx_validate` is documented to finalize.

**Rule of thumb:** when gallops state has diverged across clones, converge with a **merge**,
never a rebase — even though the repo otherwise keeps a linear `jjb:` history. The merge
commit is the accepted cost.

## 8. The manual-edit gap (durable JJK weakness)

Even with the identity collision resolved, the *textual* gallops merge had **no non-manual
path**:

- No git merge driver is registered for `jjg_gallops.json`.
- `jjx_validate` parses the working-tree file, so a marker-mangled (mid-conflict) gallops is
  invalid JSON and reports `broken` — it cannot resolve conflict markers itself.

Hand-editing the gallops directly violates the standing "never reach past the JJK interface
to raw storage" rule; it was done here **only** under an explicit, deliberate operator
override for this one incident.

### Recommended durable fixes (JJK infrastructure — candidates, not yet cinched)

1. **A gallops merge driver.** Register a custom git merge driver in `.gitattributes` for
   `.claude/jjm/jjg_gallops.json` that invokes a `jjx` routine to converge the two versions
   semantically (union paces, reconcile the seed, detect true identity collisions). This
   makes `git merge` auto-resolve gallops with no hand-edit.
2. **A `jjx` conflict-finalize that reads `HEAD` + `MERGE_HEAD` git objects** (not the
   marker-mangled working file), so `jjx_validate` (or a sibling) can finalize a merge whose
   working-tree gallops still carries markers.
3. **Reduce coronet-collision probability at the source** — the deeper root cause. Options
   to weigh: per-officium coronet sub-namespacing, a reservation/lease on the seed, or making
   the seed collision-detecting on convergence with an automatic re-coronet (Fable's tombstone
   maneuver, but built-in and mechanical).
4. **Codify the tombstone-transfer** as a first-class `jjx` "evict coronet" operation, so a
   collision disposition does not require hand-assembling `nominate → transfer → retire`.

## 9. Artifacts and references

- **Merge / resolution:** `0ecd309c0` (alpha, on `origin/main`).
- **alpha `₢BfAA3` (affiance-jilt-folio-recut):** notch `5b3d3629e` (orig `68b2bf520`),
  wrap `11f18b9e7` (orig `278b5f0e5`).
- **Fable's tombstone sequence (beta):** `2598c280b`, `e849aa7f6`, `f0b2389cf` (terrier →
  `₢BfAA4`); `3a42d41e0` (`₣Bp:N`); `d80a8087e` (`₣Bp:D  ₢BfAA3 → ₢BpAAA`); `35281478f`
  (`₣Bp:R`).
- **Retired tombstone heat file:** `.claude/jjm/retired/jjh_b260702-r260702-bfaa3-collision-tombstone.md`.
- **The gallops file at issue:** `.claude/jjm/jjg_gallops.json` (`jjghn_next_pace_seed`
  reconciled to `AA5`).
