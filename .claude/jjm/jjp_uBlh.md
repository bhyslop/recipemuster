## Paddock: jjk-v4-1-svg-viewer-and-pane-labels

## Context

This heat is the rbm-side slice and coordination spine for a two-repo feature:
a paneboard-hosted overlay that labels each Claude Code window with its JJK pace,
and a standalone SVG/raster diagram viewer that paneboard conducts.
Provenance and the one proven mechanism (iTerm session-id correlation) are in the seed memo.

The repo split is load-bearing.
The viewer binary and the paneboard hub are paneboard-owned;
the rbm side is thin — vvx reads its iTerm session id, discovers paneboard's port-file,
and sends label/diagram messages best-effort, fail-soft.
Both ends share one wire protocol, frozen once the viewer's walking skeleton validates it;
the rbm-side concepts land formally in JJS0.
The whole feature is driven from one control console in rbm — see Cross-repo operation.

## References

- `Memos/memo-20260617-paneboard-overlay-and-viewer.md` — seed memo:
  design decisions, the proven session-id correlation mechanism, dead-ends, and the viewer-first sequencing spine.
- `../pb_paneboard02/poc/paneboard-poc.md` — paneboard's PoC spec (its requirements home);
  the IPC channel, session-id correlation, and viewer are all greenfield in it.
- `diagrams/rbdg*.svg` — sample PlantUML-rendered diagrams (federation login/setup/keyfile/seam, light+dark),
  the realistic viewer payload and test fodder; only a start.

## Cross-repo operation

This heat spans two repos, driven from one control console in rbm.
Paneboard is a sibling checkout at `../pb_paneboard02` (adjust if relocated).
This is cross-repo but local — not a foray/fundus remote.

Drive paneboard's tabtargets by sibling-relative path:
`../pb_paneboard02/tt/<name>.sh`.
They self-locate and chdir internally, so they run correctly from the rbm cwd
and do not corrupt it.
Discover with `ls ../pb_paneboard02/tt/`; the timed PoC is
`pbw-t.ProofOfConceptTimed.10.sh`.

JJK cannot commit paneboard code — notch commits into rbm's git only.
Commit paneboard work from this console with
`git -C ../pb_paneboard02 add <explicit files>` then `git -C ../pb_paneboard02 commit`.
Same additive, explicit-file-list discipline as notch;
the forbidden git commands (reset, restore, checkout-to-discard, clean, stash)
still apply — `-C` does not make them safe.
Respect paneboard's own branching, not rbm's.