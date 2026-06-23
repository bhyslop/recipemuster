## Paddock: jjk-v4-1-svg-viewer-and-pane-labels

## Context

This heat carries the whole two-repo feature as one interleaved pace stream:
a paneboard-hosted overlay that labels each Claude Code window with an emblem naming its JJK pace,
and a standalone SVG/raster diagram viewer that paneboard conducts.
Provenance and the one proven mechanism (iTerm session-id correlation) are in the seed memo;
a 2026-06-22 grooming scrub ground-truthed every docket against paneboard, vvx, JJS0, and the diagram pipeline, and reshaped the transport accordingly.

The repo split is load-bearing for code ownership, not for pace ownership.
The viewer binary and the paneboard hub are paneboard-owned;
the rbm side is thin but is NOT "vvx changes only":
it spans vvx (the emblem writer) plus a style-config file plus an RBK-side diagram push — the diagram push lives in the rbtd crate, a different binary from vvx.
Both repos' paces nonetheless live in this single heat and advance interleaved,
with paneboard-side paces committed via git -C (see Cross-repo operation).

## Transport — two channels, split along the sandbox boundary

Paneboard hard-applies a seatbelt sandbox that denies all network and refuses to run without it.
That posture is load-bearing — its keystroke tap can see everything typed, and no-network is what makes exfiltration categorically impossible — so it is kept intact, never relaxed.
The two halves of this feature have different endpoints, so they take different transports:

- Emblems (the labels) ride a FILE transport.
  Each vvx atomically writes its own window's emblem to a file under the fixed emblem root (see Emblem and window reference) that paneboard reads at paint time.
  No socket into paneboard, so the network deny is untouched.
  The emblem file IS the atomic per-window frame — the earlier "one message per window, never split" cinch, realized as one file per window.
- Viewer image bytes ride localhost TCP terminating at the VIEWER, a separate non-sandboxed binary, never at paneboard.
  The pusher connects directly to the viewer's advertised port.

Consequence: paneboard needs no listening socket at all, and the standalone viewer keeps working for the no-paneboard / Windows case.
The seatbelt loopback-only carve-out is verified feasible and held as the fallback if a future need ever forces bytes through paneboard itself.

The viewer wire additionally carries an optional light/dark PAIR per image — cinched here so the decision is not lost before it is built.
The README ships each diagram as two committed SVGs (a light render plus a dark recolor produced by rbm's pluml case), and the viewer cannot derive one from the other: the recolor palette is PlantUML-skin-specific and stays single-homed in rbm, never compiled into the format-agnostic viewer, so both variants must travel over the wire.
The pair rides ADDITIVELY — a second optional payload within one frame, its absence meaning today's single-payload frame — so the change is backward-compatible and the existing single-push path is untouched; do not pair via the instance id, which is the reserved per-instance handle, not an image id.
The viewer holds both and toggles between them ('d'/'l') with the held zoom+pan retained, an in-tool proof of exactly what the README <picture> blocks render in each mode.
Building it revises and re-freezes the viewer wire contract (the poc spec's protocol section).

## Emblem and window reference

An emblem is the displayed label: an ordered set of stacked regions (top / middle / bottom), each region a list of lines plus optional style, on fixed black backing pills.
The session identity is the primary glance datum — coronet when mounted on a pace, else the heat firemark, full identity, never abbreviated.

An emblem binds to a window through a TYPED window reference, not a bare session id.
The key is scheme-qualified — $HOME/.config/paneboard/emblems/<scheme>/<value>.emblem — so emblems generalize to other window types later.
The root is a fixed, paneboard-owned per-user path, matching paneboard's existing ~/.config/paneboard/ config home: a by-convention rendezvous needing no handshake, with paneboard's PoC spec as its authority and rbm mirroring the literal with a citation.
The writer mkdir's the tree and fails soft; the reader treats an absent or empty tree as no emblem (never world-writable /tmp, so the overlay cannot be spoofed by another local user).
The one resolver (UUID -> window) lives in the WRITER, not paneboard (see Resolver for why); vvx resolves and keys the emblem by the window handle paneboard already holds (the CGWindowID), and paneboard reads by that handle.
The exact key is decided: the window-id directly (the CGWindowID), not the UUID and not a UUID-plus-index — see Resolver for the recycling defense and why the operator's one-session-per-window posture makes it safe.
Emblems on non-Claude-Code windows are a named fork, not designed now: adopt the typed namespace, build only the one resolver.
Remote Claude Code sessions are a second named fork, not solved here: when Claude Code runs on another host (e.g. cerebro over SSH) shown in a LOCAL iTerm window, the writer (vvx) runs remote and has none of the local handles (no AppleScript, no CGWindowID, no ITERM_SESSION_ID since SSH does not forward it, no local emblem dir), so it can neither resolve nor write the emblem.
This is inherent to the writer-and-window co-location the whole design assumes — the abandoned loopback socket failed it too (loopback is not cross-host, and a cross-host listener is exactly what the sandbox forbids).
Such a window still appears in paneboard's switcher list, just without an emblem on its box (absent emblem = no emblem), no worse than today.
The only host-crossing channel is the terminal stream itself (OSC / iTerm badge), a different and lower-fidelity mechanism that renders through iTerm rather than paneboard's overlay; if remote labeling is ever wanted, it is that separate sub-feature, not a paneboard extension.

Style is optional per region (a font size and a color), sourced from an rbm-side config vvx reads at write time, never compiled in — edit the config, rewrite on any engagement, see it on the next alt-tab; paneboard supplies built-in defaults for any absent field.
Region STRUCTURE (slot + lines + style) is frozen; region CONTENT — which lines land in which band — is deliberately soft and config-tunable.
Starting content (soft): top = identity + pace name; middle = repo + working directory; bottom = identity (mirrors top, so the identity reads from all four corners — paneboard paints top into both top corners and bottom into both bottom corners).

## Overlay surface — the selection box

Paneboard draws two things during alt-tab: a LIST of all windows (lower half of the screen — the window switcher, renders text today) and a yellow OUTLINE BOX around the selected window (renders only an outline today).
The emblem renders on the BOX, never in the list.
The list's row text is the switcher's navigation handle — the operator picks a window by its title and app — so writing emblem content there overwrites the very datum used to switch; the list is left exactly as it is.
The box is empty real estate (an outline only) that wants an in-place label, drawn on the window the operator is selecting.

A throwaway list-surface probe (2026-06-23) proved the whole pipeline end to end — writer -> file -> window-id FFI -> paint-time read -> display all work — by briefly using the list as the cheapest possible reader; the probe is fully reverted, its only residue the de-risking it bought.
The "list first, then the box" sequencing it embodied was a misconception: the list is not an emblem surface, and the box is not a richer SECOND surface but the SOLE one.

The box looks up the emblem file at paint time by the window-id paneboard already holds for the selected window (the handle the resolver produced) — no in-paneboard resolver (see Resolver).
It draws the three regions as banded sub-rects of multi-line text on fixed black backing pills, branching cleanly on emblem-present vs absent.
Today only the selected window's box carries the emblem, so the operator reads each window's identity on its box as the highlight lands on it while tabbing; labelling EVERY window's box at once (N label windows simultaneously) is a named fork, not designed now — and it is the path back to the seed memo's "all windows at a glance" goal if that is ever wanted.

## Resolver — verdict: it moves off the sandboxed reader to the non-sandboxed writer (2026-06-22)

The spike settled the go/no-go.
The window -> iTerm-session-UUID mechanism is iTerm's own scripting API (AppleEvents), and the join is EXACT: iTerm's AppleScript window id is the very CGWindowID paneboard already keys on (its AX _AXUIElementGetWindow value), confirmed by matching paneboard's live window enumeration against iTerm's session list — every window's id identical across both.

But that resolver CANNOT run inside paneboard.
An in-process NSAppleScript call, under paneboard's real seatbelt sandbox, fails with AppleEvents privilege violation -10004 — identical to the same call under sandbox-exec, so the result is faithful, not a harness artifact.
The block is above the seatbelt layer: (allow default) already permits appleevent-send, so this is a TCC/entitlement gate (an unentitled sandboxed process is denied AppleEvents), which no seatbelt-profile text opens.
A profile carve-out therefore does NOT help, and is abandoned.

The resolution moves to the WRITER instead.
vvx is not sandboxed and IS an iTerm descendant, so the same AppleScript self-scripts there with no prompt (proven via a bare osascript run).
So vvx resolves its own UUID -> CGWindowID and keys the emblem by that window handle; paneboard reads the emblem by the window-id it already enumerates and holds NO resolver, no AppleScript, and never touches the sandbox question.
The file transport is the sandbox-crossing membrane, exactly as designed.

Window-handle key stability — decided: key by window-id direct (the CGWindowID), the UUID kept only as the transient AppleScript lookup, never persisted.
A CGWindowID is stable for a window's life but can be recycled after a window closes, and changes if a tab is dragged to a new window; the recycling defense is the write-stamp plus paneboard's live-set intersection plus vvx rewriting on each engagement, with manual cleanup between runs as the backstop.
The multi-session-per-window clobber that UUID-keying would have guarded against does not arise: the operator runs one Claude session per iTerm window (and does not drag tabs between windows), so the window-id is a faithful per-window key.

Box-text is PROVEN: the yellow selection box can carry drawn text (a one-line NSString.draw poke rendered white-on-black on the box), so the box-render pace is de-risked.

## Paneboard internals (grounded in current code)

The overlay and AX render path is main-thread-only: every paint is marshalled to the main run loop via CFRunLoopPerformBlock + CFRunLoopWakeUp, and the yellow box is repositioned per tab-press.
Reading a tiny emblem file at paint time rides that same path — a microsecond file read, no new thread, no run-loop source.
The viewer conductor launches and AX-places viewer windows; because paneboard never reads the image bytes, the earlier multi-megabyte off-thread-read concern does not arise on the paneboard side.
Settled (2026-06-23 probe, faithful to pbmbs_sandbox's byte-identical policy): a child spawned DIRECTLY by paneboard (posix_spawn / exec / Command) INHERITS the (allow default)(deny network*) seatbelt sandbox — EPERM on both listen and connect, versus an unsandboxed control's ECONNREFUSED — so the viewer, which must listen on its advertised port, cannot be a direct paneboard child.
The viewer is therefore launched INDEPENDENTLY through launchd — NSWorkspace.openApplication / open of an .app bundle — and the launchd-parented viewer escapes the sandbox with full networking, while open itself runs fine under the sandbox (it reaches launchd over Mach IPC, which deny-network* does not touch).
Paneboard only AX-positions the already-running, launchd-owned window; it never forks the viewer, so the sandbox boundary is never the viewer's parent edge.
Consequence on the viewer side: it must ship as a minimal .app bundle (Info.plist + Contents/MacOS/paneboard-viewer), since launchd cannot open a bare binary — a small packaging addition that also makes the standalone viewer a proper double-clickable app.

## Platform surface — macOS pins and the portability seam

Paneboard has never been ported off macOS, but cross-OS portability is a preserved direction, so this section keeps tabs on what is macOS-bound.

Paneboard's core is already macOS-native and predates this heat: the CGEventTap keyboard intercept, the Accessibility API (window enumeration, rects, focus, the CGWindowID identity), the CFRunLoop, the AppKit overlay rendering (the Swift shim), NSWorkspace app identity, NSPasteboard, and the seatbelt sandbox.
This heat adds essentially ONE new platform dependency: the window -> session-key RESOLVER (AppleEvents to iTerm's scripting API, macOS AND iTerm specific) — which the 2026-06-22 spike relocated from paneboard to the non-sandboxed writer (vvx, see Resolver) because paneboard's sandbox blocks AppleEvents; relocating it actually IMPROVES portability, since the seam now lives in the cross-platform writer and paneboard holds no resolver.
The box-text the emblem feature draws is not new lock-in — it rides the AppKit overlay paneboard already has.

The resolver is the per-platform-AND-per-terminal SEAM, now housed in the writer; portability survives iff one boundary holds:
the resolver leg is the only platform-specific part of the writer, and everything else stays neutral — the emblem CONTENT, the JSON format, the typed scheme namespace, and the standalone viewer.
The window handle the resolver emits is platform-specific by nature (CGWindowID on macOS, an HWND-equivalent elsewhere), so it lives behind the typed scheme, never as a bare assumption in the reader.
Then a port is: write one new resolver leg in the writer and register one new scheme; paneboard's emblem lookup is already generic over the handle, though paneboard's overlay/AX/event-tap core remains the separate, pre-existing macOS port problem.

The lock is two-axis, which the typed scheme namespace already anticipates: OS (macOS / Windows / Linux) times terminal (iTerm / Terminal.app / Windows Terminal / ...).
iterm-session is one cell; even a macOS switch from iTerm to Terminal.app would need a different resolver.
The Accessibility-API window access is the hardest port target — Win32 has direct equivalents, but Wayland deliberately restricts cross-window introspection.

## References

- Memos/memo-20260617-paneboard-overlay-and-viewer.md — seed memo: design, the proven session-id correlation, dead-ends, sequencing spine.
- ../pb_paneboard02/poc/paneboard-poc.md — paneboard's PoC spec (its requirements home).
- diagrams/rbdg*.svg — sample diagrams: the viewer payload and test fodder (~17-22 KB each).

## Cross-repo operation

This heat spans two repos, driven from one control console in rbm.
Paneboard is a sibling checkout at ../pb_paneboard02 (adjust if relocated).
This is cross-repo but local — not a foray/fundus remote.

Drive paneboard's tabtargets by sibling-relative path: ../pb_paneboard02/tt/<name>.sh.
They self-locate and chdir internally, so they run correctly from the rbm cwd and do not corrupt it.

JJK cannot commit paneboard code — notch commits into rbm's git only.
Commit paneboard work from this console with git -C ../pb_paneboard02 add <explicit files> then git -C ../pb_paneboard02 commit.
Same additive, explicit-file-list discipline as notch; the forbidden git commands still apply — -C does not make them safe.
Respect paneboard's own branching, not rbm's.

## Testing harness — direct process control

Paneboard is a singleton (an exclusive file-lock at /tmp/paneboard.lock, released automatically when the process exits).
So a trial means displacing the running instance, then restoring it.
The proven loop, drivable entirely from this console:

- Kill the running paneboard-poc process (the lock releases on death — no stale-file hazard).
- Run the timed tabtarget ../pb_paneboard02/tt/pbw-t.ProofOfConceptTimed.10.sh, which builds BOTH crates (the viewer and the PoC), runs ~10s, and self-exits via its own auto-exit timer — so a trial cannot run away with the operator's alt-tab.
- Relaunch the standing instance in the background (../pb_paneboard02/tt/pbw-b.BuildProof.sh, which likewise builds both crates then launches).

Both build tabtargets (pbw-b standing, pbw-t timed) build the viewer alongside the PoC, so the harness exercises the viewer build too; pbw-p is retired.

For a throwaway source probe, edit between the kill and the timed run, then revert after (grep a unique marker to prove a full revert; git diff the touched files should be empty).
build.rs recompiles the Swift shim on change, so a Swift-side poke is picked up.
A poke that must be SEEN (e.g. box rendering) needs the operator to alt-tab during the live window; a poke that only emits to stdout does not.

Lifecycle ownership: the AGENT drives it in full — kill to test, relaunch after.
A background instance launched from the agent session SURVIVES the operator's /clear (a context reset, not a process restart), so it rides through pace transitions; it dies only at full Claude Code session end, when the next mount relaunches it (or the operator does).
Caveat: paneboard is a singleton, so only one agent session should drive its lifecycle at a time.