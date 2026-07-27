# Incident report: billet-branch case-fold ref fusion on an APFS clone — 2026-07-27

Written in the `rbm_alpha_recipemuster` clone during a trunk catch-up, with Job
Jockey deliberately unavailable (operator direction). Every finding below is
git-level and was verified by command; every JJ-state question is left open by
construction, for a billet where `jjx_*` is reachable. This is a dated incident
record — provenance, not authority.

## Summary

This clone's `origin` remote-tracking namespace holds **four pairs of
billet branches whose names differ only in the final character's case**. GitHub
is case-sensitive and stores eight distinct branches with eight distinct tips.
This clone sits on APFS with `core.ignorecase=true`, so each pair collapses onto
a **single loose-ref file**, and `git rev-parse` returns the same SHA for both
spellings.

Two consequences, one cosmetic and one not:

1. **Every `git fetch` reports four spurious "forced update" lines.** Git sees
   the file for `…/CAABQ` holding `…/CAABq`'s value, "corrects" it, and then the
   lowercase write clobbers it back. The pairs ping-pong on every fetch, forever.
2. **Four of the eight remote tips are unaddressable by ref name on this clone.**
   In every pair the *lowercase* value wins the fold, so the uppercase branch's
   tip cannot be reached as `origin/personal/bhyslop/jjls_pace/CAABQ` — that name
   resolves to the lowercase tip instead.

**No content is at risk.** All eight tips exist as objects locally, and every
one is subsumed by `main` (proof below). The fix for the *cause* — `8370a84c3`,
case-armoring the coronet segment of billet branch names — is now in local
`main`. It governs newly composed names only; it does not rename the eight
branches already on GitHub, so the fusion persists until those are cleared.

## The four pairs

Remote truth (from `git ls-remote --heads origin`, case-preserving over the
wire) against what this clone resolves. All under
`refs/heads/personal/bhyslop/jjls_pace/` remotely,
`refs/remotes/origin/personal/bhyslop/jjls_pace/` locally.

| Pair | Remote upper | Remote lower | Local resolves **both** to | File on disk |
|---|---|---|---|---|
| `CAABQ`/`CAABq` | `c9b0021e2` | `6fa0dc891` | `6fa0dc891` | `CAABQ` |
| `CAABT`/`CAABt` | `0c4b06d75` | `98ec973bf` | `98ec973bf` | `CAABT` |
| `CAABW`/`CAABw` | `6faf70b1c` | `a895c2029` | `a895c2029` | `CAABW` |
| `CAABZ`/`CAABz` | `6ad7c8496` | `514d835dc` | `514d835dc` | `CAABZ` |

The surviving *filename* is uppercase in all four (created first, historically);
the surviving *value* is the lowercase tip (written last by the fetch, since git
updates the uppercase ref before creating the lowercase one).

Tip shapes, for orientation:

| Tip | Marker | Subject |
|---|---|---|
| `c9b0021e2` | `₢CAABQ:L:` | `claude-haiku-4-5-20251001 landed` |
| `6fa0dc891` | — | `enfold trunk` |
| `0c4b06d75` | `₢CAABT:n:` | `Evict the reserved word 'spine'` |
| `98ec973bf` | — | `Merge remote-tracking branch 'origin/main' into personal/…` |
| `6faf70b1c` | — | `enfold trunk` |
| `a895c2029` | — | `enfold trunk` |
| `6ad7c8496` | `₢CAABZ:L:` | `claude-haiku-4-5-20251001 landed` |
| `514d835dc` | — | `enfold trunk` |

Case-distinct coronets are real and in use, not an accident of one bad push:
`origin/main` carries independent `:W:` wrap commits citing both `₢CAABQ`
(`ec662762b`) and `₢CAABw` (`971b9f27f`). The coronet alphabet is
case-sensitive by design; the filesystem is what cannot hold it.

## Blast radius: exactly these four pairs

- **Remote-tracking namespace**: four folds, enumerated above. Checked by folding
  all 95 remote head names to lowercase and counting duplicates.
- **Local head namespace (93 heads, 46 of them under
  `personal/bhyslop/jjls_pace/`)**: **no fold**, checked on full ref paths.
- Cross-namespace folds are impossible — `refs/heads/…` and
  `refs/remotes/origin/…` are different directories.

## Nothing is owed: the subsumption proof

Two facts compose:

1. **Within every pair, the uppercase tip is an ancestor of the lowercase tip**
   (`git merge-base --is-ancestor`, 4/4). The lowercase branch is the uppercase
   branch plus an `enfold trunk` merge — one line of work, two names.
2. **Each lowercase tip's tree is reproducible from `main`.** For each,
   `diff(merge-base(main, tip), tip)` is empty, except `CAABw`, whose single
   differing path is `Memos/memo-20260726-acgm102-name-identity-census.md` —
   verified **byte-identical** to `main`'s copy. Every merge base is an ancestor
   of `main`.

Therefore no net content on any of the eight tips is absent from `main`.

**`git cherry` is the wrong instrument here and will mislead a future reader.**
It reports 0–4 "unlanded" patches per tip, because it compares patch-ids and
JJ's `enfold trunk` merges have none that match. Whether a billet's individual
commits ever appear on trunk as commits varies in this corpus (`origin/main`
carries `:n:` notch commits such as `9be92be3d` for `₢CAABz`, so at least
sometimes they do) — the mechanism was not run to ground and is not needed.
**Tree comparison is decisive; patch-id comparison is not.**

## The open JJ-state question — the gate on any cleanup

Five of the eight coronets have a `:W:` wrap commit on `origin/main`; **three do
not**:

| Coronet | Wrap on `origin/main` | Wrap's cited billet parent |
|---|---|---|
| `₢CAABQ` | `ec662762b` | `3df23a658` — *not* an ancestor of the branch tip |
| `₢CAABq` | **none found** | — |
| `₢CAABT` | `36cf9f519` | `0c4b06d75` — matches the branch tip |
| `₢CAABt` | **none found** | — |
| `₢CAABW` | `bc8c74a67` (pre-dates the catch-up merge base) | `6faf70b1c` — matches |
| `₢CAABw` | `971b9f27f` | `a895c2029` — matches |
| `₢CAABZ` | `3f356ab21` | `1d7841631` — *not* an ancestor of the branch tip |
| `₢CAABz` | **none found** | — |

So `₢CAABq`, `₢CAABt`, `₢CAABz` may be **open paces**, whose billet branches are
live JJ working state rather than residue. And for `₢CAABQ` and `₢CAABZ`, the
wrap on trunk cites a billet parent **disjoint** from what the branch now holds,
while the branch tip is itself an `:L:` designee landing — consistent with a
second, unreviewed landing under a reused branch name, but not proven.

Git cannot distinguish "open pace" from "residue under a reused name." Only JJ
can. `8370a84c3`'s own text also states that the legacy bare-length parse arm
reads pre-armoring billets **until they age out** (the no-migration cinch) — so
JJ deliberately expects these branches to persist and stay readable. Deleting
them ages four of them out by hand, ahead of that schedule.

**Resolve with JJ before deleting anything remote.** Minimum checks: are
`₢CAABq`, `₢CAABt`, `₢CAABz` open? Do `₢CAABQ`/`₢CAABZ` have unreviewed
landings? Does the gallops/blotter state or the muck-reap machinery reference
any of these billets?

## Proposed resolution, once the gate clears

The fold exists **only because both spellings exist on GitHub**. Local-only
palliatives do not hold: `git pack-refs --all` would let case-distinct names
coexist in `packed-refs`, but the next fetch touching either writes a loose ref
and re-fuses. The durable fix is remote-side, in this order:

1. `git push` the catch-up merge (see below) — pure catch-up, no risk, can go now.
2. Delete the settled billet branches on GitHub (`git push origin --delete`).
   Objects remain in this clone, so it is recoverable.
3. `git fetch --prune` — clears the local remote-tracking refs and, with them,
   the four spurious forced-update lines.

## The catch-up merge, for the record

Local `main` was `ahead 2, behind 34` of `origin/main` from merge base
`93d4b89bb`. Both local-only commits were **empty** officium markers
(`8a774aca7`, `c4de7ad04`; `git diff --stat 93d4b89bb main` empty), so the
divergence was bookkeeping, not content.

Merged (`ef747e9ec`), not rebased: the markers' subjects encode their parent SHA
(`jjb:1019-93d4b89bb`, `jjb:1019-8a774aca7`), which a rebase would re-parent and
thereby falsify. Precedent exists (`0569d44b5` is a plain trunk merge). Our side
contributed no paths, so the merge was one-sided and conflict-free; `git diff
main origin/main` is now empty. State: `ahead 3, behind 0`, tree clean, not yet
pushed.

## Method notes — two wrong turns worth not repeating

1. **`git diff main...tip` does not answer "is anything owed."** Three-dot is
   `diff(merge-base, tip)`: it shows the tip's own changes whether or not `main`
   also has them. Read naively it reported "1 / 29 / 14 paths NOT on main" for
   the uppercase tips — pure artifact of an older merge base. Compare trees
   against a main-resident merge base instead.
2. **Fold-checking on prefix-stripped ref names manufactures phantom
   collisions.** Stripping `personal/bhyslop/jjls_pace/` made
   `refs/heads/CAABA` and `refs/heads/personal/…/CAABa` look fused. They are in
   different directories and cannot collide. Six false positives came from this;
   all six evaporated when the check was redone on full ref paths. **Fold-check
   on the full ref path, which is what the filesystem sees.**

## Environment

`core.ignorecase=true`; volume is APFS, case-insensitive. Station-local — a
case-sensitive clone of the same repo would show all eight refs correctly and
none of these symptoms.
