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

## Emblem and window reference

An emblem is the displayed label: an ordered set of stacked regions (top / middle / bottom), each region a list of lines plus optional style, on fixed black backing pills.
The session identity is the primary glance datum — coronet when mounted on a pace, else the heat firemark, full identity, never abbreviated.

An emblem binds to a window through a TYPED window reference, not a bare session id.
The key is scheme-qualified — $HOME/.config/paneboard/emblems/<scheme>/<value>.json — so emblems generalize to other window types later.
The root is a fixed, paneboard-owned per-user path, matching paneboard's existing ~/.config/paneboard/ config home: a by-convention rendezvous needing no handshake, with paneboard's PoC spec as its authority and rbm mirroring the literal with a citation.
The writer mkdir's the tree and fails soft; the reader treats an absent or empty tree as no emblem (never world-writable /tmp, so the overlay cannot be spoofed by another local user).
The one resolver (UUID -> window) lives in the WRITER, not paneboard (see Resolver for why); vvx resolves and keys the emblem by the window handle paneboard already holds (the CGWindowID), and paneboard reads by that handle.
The exact key — the window-id directly, or the UUID with a writer-written window-id index — is a grooming open point (window-handle recycling, see Resolver).
Emblems on non-Claude-Code windows are a named fork, not designed now: adopt the typed namespace, build only the one resolver.
Remote Claude Code sessions are a second named fork, not solved here: when Claude Code runs on another host (e.g. cerebro over SSH) shown in a LOCAL iTerm window, the writer (vvx) runs remote and has none of the local handles (no AppleScript, no CGWindowID, no ITERM_SESSION_ID since SSH does not forward it, no local emblem dir), so it can neither resolve nor write the emblem.
This is inherent to the writer-and-window co-location the whole design assumes — the abandoned loopback socket failed it too (loopback is not cross-host, and a cross-host listener is exactly what the sandbox forbids).
Such a window still appears in paneboard's list, just unlabeled (absent emblem = no emblem), no worse than today.
The only host-crossing channel is the terminal stream itself (OSC / iTerm badge), a different and lower-fidelity mechanism that renders through iTerm rather than paneboard's overlay; if remote labeling is ever wanted, it is that separate sub-feature, not a paneboard extension.

Style is optional per region (a font size and a color), sourced from an rbm-side config vvx reads at write time, never compiled in — edit the config, rewrite on any engagement, see it on the next alt-tab; paneboard supplies built-in defaults for any absent field.
Region STRUCTURE (slot + lines + style) is frozen; region CONTENT — which lines land in which band — is deliberately soft and config-tunable.
Starting content (soft): top = identity + pace name; middle = repo + working directory; bottom = reserved.

## Overlay surface — list first, then the box

Paneboard draws two things during alt-tab: a list of all windows (lower half of the screen, renders text today) and a yellow outline box around the selected window (renders only an outline today).
Both can carry emblems, and both just look up the emblem file by the window-id paneboard already enumerates — neither needs an in-paneboard resolver (see Resolver).
Sequenced by rendering cost:

- The list entries gain emblems first — reusing the list's existing text rendering, showing every window at a glance (the memo's actual goal), validating the whole pipeline with almost no new drawing.
- The selected-window box gains emblems second — net-new text-and-pill rendering on the box, the richer in-place view; the box CAN carry drawn text (proven, see Resolver).
- Emblems on every window's box at once (N label windows) is a named fork, not designed now.

The earlier "list-only if the resolver fails" fallback is retired: with the resolver moved to the writer, paneboard never holds a resolver that could fail, so list and box stand or fall together on the same window-id lookup.

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

Open for grooming: window-handle key stability.
A CGWindowID is stable for a window's life but can be recycled after a window closes, and changes if a tab is dragged to a new window; vvx rewriting on each engagement plus cleanup of its prior key is the likely handling, but the exact scheme (key by window-id, or keep UUID-keying with a vvx-written window-id -> UUID index) is undecided.

Box-text is PROVEN: the yellow selection box can carry drawn text (a one-line NSString.draw poke rendered white-on-black on the box), so the box-render pace is de-risked.

## Paneboard internals (grounded in current code)

The overlay and AX render path is main-thread-only: every paint is marshalled to the main run loop via CFRunLoopPerformBlock + CFRunLoopWakeUp, and the yellow box is repositioned per tab-press.
Reading a tiny emblem file at paint time rides that same path — a microsecond file read, no new thread, no run-loop source.
The viewer conductor spawns and AX-places viewer windows; because paneboard never reads the image bytes, the earlier multi-megabyte off-thread-read concern does not arise on the paneboard side.
Open, to verify at conductor time: whether a paneboard-spawned child inherits the network-deny sandbox.
If it does, the viewer is launched independently and paneboard only positions its window.

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