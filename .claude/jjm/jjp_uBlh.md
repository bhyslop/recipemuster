## Paddock: jjk-v4-1-svg-viewer-and-pane-labels

## Context

This heat carries the whole two-repo feature as one interleaved pace stream:
a paneboard-hosted overlay that labels each Claude Code window with its JJK pace,
and a standalone SVG/raster diagram viewer that paneboard conducts.
Provenance and the one proven mechanism (iTerm session-id correlation) are in the seed memo.

The repo split is load-bearing for code ownership, not for pace ownership.
The viewer binary and the paneboard hub are paneboard-owned;
the rbm side is thin — vvx reads its iTerm session id, discovers paneboard's port-file,
and sends label/diagram messages best-effort, fail-soft.
Both repos' paces nonetheless live in this single heat and advance interleaved —
the two ends are too tightly coupled across the shared wire to sequence as separate heats —
with paneboard-side paces committed via `git -C` (see Cross-repo operation).
Both ends share one wire protocol, frozen once the viewer's walking skeleton validates it;
the rbm-side concepts land formally in JJS0.
The whole feature is driven from one control console in rbm — see Cross-repo operation.

## Pane-label overlay shape

The pane label renders on the yellow highlight box paneboard already draws around a window during alt-tab.

The overlay is three stacked regions — top, middle, bottom — replacing the earlier four-corners-plus-center scheme.
Cinched decisions:

- Every region is uniform and multi-line: an ordered list of lines plus an optional style.
  The earlier asymmetry — a scalar identity in the corners against an array of lines in the center — is gone;
  each region is now the same shape, so the model carries one region concept rather than two.
- Each region sits on a fixed black backing pill; the backing is not tunable.
- The session identity is the primary glance datum — coronet when mounted on a pace, else the heat firemark, full identity, never abbreviated.
- vvx sends one atomic frame per window: the session key plus the full set of regions in a single message, each region carrying its slot (top/middle/bottom), its lines, and its optional style.
  The overlay is never split into one-message-per-region.
  The transport is best-effort and fail-soft, so a multi-message overlay could tear — a stale band left beside a fresh one, with no way for the operator to tell which —
  the session key would repeat needlessly across the messages,
  and the producer recomputes the whole label on each engagement anyway.
  The slot enumeration rides the region, not the message boundary.
- Style is optional per region — a font size and a color — and paneboard falls back to built-in defaults when any field is absent.
- Style values are sourced from an rbm-side config that vvx reads at send time, never compiled into the binary;
  tuning is edit the config, run any jjx engagement to re-send, see the change on the next alt-tab — no rebuild, no paneboard restart.

Open, not yet cinched: which lines land in which band.
The identity leads the top region by default;
the repo, the working directory, and the pace-or-heat silks are the other high-value lines awaiting assignment.
This is the next layout musing.

Named fork, not designed now: a second independent producer — something other than vvx pushing into one band on its own clock — would justify genuine one-message-per-region framing.
vvx is the sole label producer today, so the overlay stays a single atomic frame;
the fork is recorded only so it is not reflexively foreclosed.

Accepted tradeoff: presentation lives partly on the wire rather than solely in paneboard — bought deliberately for iteration velocity, and free to retract into paneboard defaults if the sizes ever stabilize.

This richer in-place overlay presumes the session-id-to-window mapping succeeds (the seed memo's still-unverified spike).
If that mapping fails, the same data degrades to a richer alt-tab list entry rather than stacked regions.

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