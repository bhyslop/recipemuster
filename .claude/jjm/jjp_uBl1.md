## Charter — repot the private working repo

Retire the crufted private development repo and found its successor rooted at the first public release,
so the private tree becomes provably "release + veiled overlay" and every future candidate squash against the public upstream is clean by construction.
The old repo is never deleted — it is archived read-only as the archaeology of record.

## Cinched (operator + design session, 260712)

Duplicate, never fork.
GitHub cannot make a private fork of a public repo (fork visibility is inherited and unchangeable),
and fork networks share an object graph whose objects are fetchable by SHA through the public parent — veiled content in a fork network would be reachable from the public repo.
The successor is an independent private repo whose only relationship to the public one is a configured remote (OPEN_SOURCE_UPSTREAM), exactly as today.
Root at the release, not staging: the promoted public main is published, immutable, and tree-hash-proven; staging is transient.
Keep the public release history as the successor's root (it is tiny — squashed candidates only) rather than an orphan commit:
it makes release-plus-overlay provable by tree hash and future squashes trivially clean.
The veiled overlay is the delivery strip run in reverse:
the rbk-prep-release Step 10 lists enumerate exactly what the private tree carries beyond the public one
(vov_veiled dirs, private kits, Memos, .claude and gallops, moorings node/user profiles, internal tabtargets);
land it as a small reviewable commit series on the release root.
Ancestry is grafted on demand, never baked in:
the successor's real history stays clean; deep per-file history is stitched locally with git replace —
fetch the old repo's final main into refs/archive/old-main, then git replace --graft the successor's founding commit onto it;
blame, log --follow, and bisect then traverse the boundary seamlessly, locally only.
Push refs/archive/* and refs/replace/* to the successor repo so any station opts in with one fetch;
replace refs do not propagate on normal clone — that is the safety.
Blame quality across the boundary is real: the release tree is essentially the old main's stripped tree and overlay files are byte-identical,
so attributions resolve into true old-repo commits, not a big-bang wall.
Deleted content needs no ancestry — nothing references it through the graft, so it never surfaces.
JJK state migration is deliberately unplanned here: Job Jockey is tectonically changing and the operator does not value the chalk-journal history;
plan JJK's carriage when this heat races, against JJK as it then stands.
This heat races only after the delivery heat (rbk-11-produce-release, ₣B0) closes — the founding point is the proven release;
do not re-point stations while delivery is mid-flight.
From the founding forward the old repo is frozen for delivered and veiled content — only JJK chalk may land (its carriage is the separately banked question);
the archive pace verifies the freeze held rather than discovering the state.
The founding proof is two-sided — release-plus-overlay equality, and a completeness sweep against the old tip:
work landed in the old repo after the candidate cut falls through the founding formula
(a post-cut delivered-file edit sits in neither the release root nor the overlay; a veiled path the strip lists never enumerated is dropped by the strip-in-reverse),
so the old tip is diffed against the successor tip over all paths and the difference must be exactly the strip lists' intentional exclusions —
anything else is carried as explicit commits atop the overlay or consciously ruled dropped, never silently lost.

## Shape

Found the successor (duplicate the released public main into a new independent private repo, land the veiled overlay series);
stitch and document the ancestry graft (archive refs pushed, replace recipe proven with a real blame traversal, recipe homed durably);
re-point the station fan-out (every fundus clone — rocket, beast, cerebro, and the local working clones — re-cloned or re-pointed; the beast-standup memo is the replay authority);
archive the old repo read-only on GitHub with a closing memo naming the successor.

## Done when

The successor private repo stands as release-root plus veiled overlay, tree-hash provable;
the ancestry graft is proven by a real cross-boundary blame and its recipe homed where a future session finds it;
every station points at the successor;
the old repo is archived read-only;
and daily work proceeds in the successor with the old repo untouched.