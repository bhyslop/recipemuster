# Heat Trophy: jjk-v4-1-svg-viewer-and-pane-labels

**Firemark:** ₣Bh
**Created:** 260617
**Retired:** 260624
**Status:** retired

## Paddock

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

## Viewer surface — a JJK master-UI primitive

The diagram viewer is a JOB JOCKEY surface, not an RBK one.
JJK is the operator's master UI across every project, so "show me this image" is a project-agnostic act; RBK is merely the first producer of images, and the viewing is generic.
This sits beside the emblem, which JJK already owns — both are surfaces of the JJK window system, with paneboard conducting the windows beneath (spawn / place / respawn) and JJK the operator-facing control plane on top.

The operator's entire memorized surface is ONE verb: unfurl — put an image on the viewer.
It follows the Upper-API two-layer split (see JJS0 Upper API): the vivid operator verb maps across a register gap to a deliberately boring lower tool, so the mapping cannot be pattern-guessed.
unfurl maps to a render tool on vvx — the non-sandboxed JJK server reads the image and pushes it to the viewer port over paneboard's wire, spawning the viewer if absent and failing soft otherwise.
The render tool carries an anew flag: a fresh look (fit-to-window) versus an iteration (retain the held zoom+pan).
The driving LLM sets anew from conversational intent — a new or different diagram, or an explicit fresh look, is anew; tweaking the diagram already up is not.
That is judgment the LLM legitimately holds, not a banned guess at the mapping; consistency comes from documenting the heuristic in the verb table, so every instance decides alike.

The render tool takes a light path plus an OPTIONAL dark path (the light/dark pair, see Transport).
The viewer cannot derive dark, and the tool stays generic — it never assumes RB's -dark naming, so the producer resolves both paths and passes them.
View manipulation lives entirely in viewer keystrokes, never in JJK vocabulary: f fits, d / l switch theme (and flip the viewer backing white<->dark, or the dark variant's light ink vanishes), zoom/pan stay scroll/drag.
The surface noun is left plain ("the viewer"); the operator never names it, so no quoin is minted now.

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
Starting content (soft): top = the work identity (a coronet on a pace, the heat firemark on a heat);
middle = the cwd basename, then — on a pace-mount only — the coronet-prefixed pace silks, then the firemark-prefixed heat silks, so a pace reads three middle lines and a heat two;
bottom = identity (mirrors top, so the identity reads from all four corners — paneboard paints top into both top corners and bottom into both bottom corners).
Each silks line leads with its own glyph-identity so the line says both what it is and which it is.
The work identity follows JJK's mount/groom semantics, gated so a transient groom cannot clobber the pace being worked:
a mount asserts its coronet (the resolved next-actionable one on a heat-mount, or the heat firemark when no actionable pace exists);
a groom fills the identity only when none is held or a firemark is, never demoting a coronet a mount has set, so a coronet once shown this officium holds for the officium's life.
The silks are resolved once at mount/groom and cached in the officium-resident saddle marker, so the per-engagement writer never re-reads the gallops.

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
The viewer conductor launches viewer instances and keeps them alive; because paneboard never reads the image bytes, the earlier multi-megabyte off-thread-read concern does not arise on the paneboard side.
Settled (2026-06-23 probe, faithful to pbmbs_sandbox's byte-identical policy): a child spawned DIRECTLY by paneboard (posix_spawn / exec / Command) INHERITS the (allow default)(deny network*) seatbelt sandbox — EPERM on both listen and connect, versus an unsandboxed control's ECONNREFUSED — so the viewer, which must listen on its advertised port, cannot be a direct paneboard child.
The viewer is therefore launched INDEPENDENTLY through launchd — NSWorkspace.openApplication / open of an .app bundle — and the launchd-parented viewer escapes the sandbox with full networking, while open itself runs fine under the sandbox (it reaches launchd over Mach IPC, which deny-network* does not touch).
Paneboard never forks the viewer, so the sandbox boundary is never the viewer's parent edge.
Placement is NOT the conductor's job (operator decision 2026-06-23): the switcher selects and the layout chords tile, so the viewer is a normal AX window the operator tiles like any other — the conductor only spawns it and keeps it alive, never moving or resizing it.
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

## Paces

### pb-guid-window-probe (₢BhAAA) [complete]

**[260622-0736] complete**

The go/no-go spike for the whole emblem overlay:
prove paneboard can resolve a live on-screen window to its iTerm session UUID via iTerm's own scripting API, under the Accessibility grant paneboard already holds — not Automation, which the operator declined for System Events.
This is the single resolver behind the typed window-reference scheme (paddock "Emblem and window reference").

See seed memo Phase 0a and its correlation-mechanism section.
Reuse the existing (pid,window_id) -> AX-rect path (pbmsa_alttab.rs get_window_rect); the yellow box is HighlightBorderWindow in pbmbo_observer.swift.

## Character
Throwaway spike — answer the questions, build nothing.

## Done when
- A paneboard probe resolves a live window to its iTerm session UUID, or proves it cannot.
- A one-line poke confirms the yellow box can carry drawn text at all (today it draws only an outline) — so the box-render pace does not discover this at mount.
- The fork is recorded as heat shape: resolver works -> emblems on list + box as designed; resolver fails -> emblems on the enriched list only, never the box, and the writer, render, and spec paces narrow accordingly.

### vvx-window-reference (₢BhAAB) [complete]

**[260622-0803] complete**

The first rbm-side slice, standalone and dependency-free:
vvx derives its own typed window reference (the iterm-session scheme: read ITERM_SESSION_ID, take the UUID after the colon) and resolves the fixed emblem-directory path it will write into.
No port-file discovery — emblems are a fixed-path file transport, not a socket (paddock "Transport").

See seed memo correlation-mechanism section (match the UUID, never the wNtNpN prefix).
Entry point: jjrm_mcp.rs near the existing Claude-Code-session-UUID reader (its sibling pattern); the env-read/parse/fallback pattern lives in jjrc_core.rs.

## Cinched
Match the UUID, not the position prefix (the prefix goes stale when a tab is dragged to another window).
The window reference is scheme-qualified (iterm-session/<uuid>); the bare UUID is never the key.
Fail-soft: no ITERM_SESSION_ID means not under iTerm, so vvx writes no emblem and skips silently.

## Done when
- vvx derives the correct scheme-qualified window reference from its own environment.
- vvx resolves the emblem-directory path and skips cleanly when it cannot, with the env-var inheritance confirmed by running the built vvx, not assumed from the proof.

### pb-standalone-image-viewer (₢BhAAC) [complete]

**[260622-0901] complete**

Build the standalone viewer and, with it, the walking skeleton for the viewer wire protocol:
an egui/iced window over a direct localhost-TCP socket, fresh/update verbs with retained zoom, format dispatch by magic-byte sniff (SVG via resvg, raster via the image crate).
A fresh standalone binary in ../pb_paneboard02 — not grafted onto the paneboard daemon.

See seed memo Phase 1 and its settled-decisions section (push-not-watch, the two verbs, format dispatch).
Sample payloads at diagrams/rbdg*.svg.

## Cinched
Paneboard conducts viewer instances; it does not contain them.
The viewer accepts direct connections (not only via paneboard) and advertises its own listen port in a port-file pushers read — the discovery the now-dropped listener pace would have owned, relocated to its rightful owner.
update retains the held zoom+pan and re-rasterizes (crisp for SVG, native-limited for raster).

## Done when
- A pushed SVG or raster renders in a window; fresh opens/replaces fit-to-window; update replaces content at the held zoom.
- The framing and verbs exist as working code, and the viewer writes its port-file on startup — the skeleton the protocol freeze ratifies.

### wire-protocol-freeze (₢BhAAD) [complete]

**[260622-1017] complete**

With the viewer skeleton proving the TCP path, write down and freeze the two contracts:
the emblem FILE format (the atomic per-window file) and the viewer fresh/update TCP wire.

See seed memo Phase 2 and the paddock "Transport" + "Emblem and window reference".

## Cinched
Emblem file: one atomic file per window (write-temp-then-rename), keyed by the scheme-qualified window reference; content is the ordered region set, each region {slot, lines, style}.
Region STRUCTURE freezes here; region CONTENT — which lines land in which band — stays deliberately soft and config-tunable, NOT frozen.
Viewer wire: localhost TCP, connect-per-message, best-effort / fail-soft, a JSON control line plus an optional length-prefixed payload tail.
The emblem-file format is frozen by design (its writer and readers land in the later rbm and render paces); the viewer wire is frozen against running skeleton code.

## Done when
- Both contracts are written in their homes (paneboard PoC spec; rbm formal landing tracked by the spec pace) and marked frozen.
- The emblem-file schema and the fresh/update wire are pinned; the spec pace transcribes the emblem schema, it does not re-decide it.

### pb-ipc-listener (₢BhAAJ) [abandoned]

**[260622-0552] abandoned**

Stand up paneboard's IPC listener:
the daemon accepts control frames on its existing CFRunLoop and writes its listen port to the discovery port-file vvx reads.
Shared infrastructure for both the overlay (register_label) and the conductor (fresh/update).

See the seed memo's protocol section (port-file discovery, framing) and the paddock's "Paneboard run-loop integration".
Entry point: the CFRunLoop in paneboard's event-tap module, where the event tap already attaches as a run-loop source.

## Cinched
The listener attaches as a run-loop source on the existing thread — never a background thread (paddock "Paneboard run-loop integration").
Control frames are read inline; the large-payload offload is the conductor's concern, not this pace.

## Done when
- Paneboard accepts a connection and parses a control frame on its run loop, with no new thread.
- The listen port is written to the discovery port-file, and a received register_label frame reaches the overlay code path.

### jjs0-overlay-concept-landing (₢BhAAE) [complete]

**[260622-1047] complete**

Land the rbm-side concepts formally in JJS0 as quoins:
the emblem (the displayed label), the typed window reference (how an emblem binds to a window), and the iterm-session scheme as the one resolver — and disambiguate the three overloaded "session" identities.

See seed memo spec-landing-zones section. Mint per MCM Lapidary and the Quoin Sub-Letter Discipline.

## Cinched
Forbid the bare word "session": it is already taken three ways — officium is defined as "a bounded agent session", "session: <uuid>" names the Claude Code session UUID in invitatory commits, and the new iTerm session id is the third. Lapidary forces three distinct words.
The window reference is an ephemeral, officium-scoped, non-fossil handle: follow the jjdt_legatio precedent (no glyph, no seed, not an Insignia), never the Insignia system.
Land under the jjdx_ executor-and-transport family (sub-letters o/r/p/f taken; w/s/c free); the legend is the Category-declarations COMMENT block in the mapping section, so each quoin is two coupled edits (comment line + attribute line) in a new transport-band subsection.

## Done when
- emblem, typed window reference, and the iterm-session scheme are minted as JJS0 quoins in a new transport-band subsection, with mapping-section entries and the legend comment updated, colliding with no existing neighborhood and never reusing "session".

### vvx-emblem-writer (₢BhAAF) [complete]

**[260622-1124] complete**

The core rbm-side emblem writer:
on each jjx engagement, vvx composes its window's emblem (identity, pace name, repo, working directory), reads per-region style from an rbm-side config, and atomically writes the emblem file for its window reference — best-effort, fail-soft.

Starting band content (soft, tweak freely): top = identity + pace name; middle = repo + working directory; bottom = reserved.

## Cinched
Write the emblem to a file (atomic temp-then-rename), keyed by the scheme-qualified window reference — never a socket send (paddock "Transport").
Identity beyond cwd/officium-id (coronet/firemark/pace name) is NOT on the jjx params; resolve it from the gallops/saddle, and handle the open verb — which has no saddled identity yet — as a degraded case (firemark or bare officium).
Style lives in a new .claude/jjm/*.json config read at write time (serde_json, the gallops-sibling convention), fail-soft to built-in paneboard defaults when the file or a field is absent; never compiled in.
Hook: the single per-command entry (fn jjx in jjrm_mcp.rs) after the model/officium gates; respect the crate's deny(warnings).

## Done when
- vvx writes a well-formed emblem file on jjx engagement, failing soft (no write, no surfaced error) when not under iTerm.
- Per-region style is read from the config with paneboard-default fallback.
- A jjx engagement returns its normal result with no added latency or error when the write path is disabled or fails — the no-paneboard non-regression, verified explicitly, including a refused/absent emblem directory.

### vvx-window-id-resolver (₢BhAAM) [complete]

**[260622-1232] complete**

The writer half of the emblem resolver — the prerequisite both paneboard emblem paces consume.
vvx resolves its own iTerm session to its containing window's CGWindowID by asking iTerm over AppleScript (osascript), and keys the emblem file by that window-id (`iterm-session/<window-id>.emblem`), so the sandboxed reader reads by the handle it already enumerates.
The session UUID is used only transiently as the AppleScript lookup key and is never persisted; the resolution is cached per vvx process (one window per session for its life).

See paddock "Resolver" and "Emblem and window reference".

## Cinched
Key = window-id direct: the emblem value is the CGWindowID, not the UUID, not a UUID-plus-index.
Operator confirmed one Claude session per iTerm window, so the multi-session-per-window clobber the UUID scheme would have guarded against does not arise.
The resolver lives in the writer, never the reader, whose sandbox forbids AppleEvents.
Fail-soft: not under iTerm, a malformed env value, or osascript denied/failed all write no emblem.

## Done when
- vvx writes `iterm-session/<window-id>.emblem`, the window-id resolved from the session UUID via iTerm AppleScript.
- The resolution is cached per process and degrades silently on any failure.
- The resolved window-id is confirmed equal to the CGWindowID the reader enumerates.

### pb-list-entry-emblems (₢BhAAK) [abandoned]

**[260623-0833] abandoned**

The first emblem render, on the cheapest surface:
paneboard puts each window's emblem on its alt-tab LIST entry — which already renders text — reusing the existing list-row drawing rather than the text-less box.
This validates the whole pipeline end to end (writer resolves and writes the file -> paneboard reads by window-id -> display) with almost no new drawing, and gives the all-windows-at-a-glance view that is the seed memo's actual goal.

See paddock "Overlay surface" and seed memo Phase 3.
Entry point: the list-row text drawing in pbmbo_observer.swift (OverlayContentView drawAltTabEntries).
paneboard reads `<emblem-root>/iterm-session/<window-id>.emblem`, keyed by the CGWindowID it already enumerates for each row; the writer resolved and wrote that file, so paneboard holds no resolver and never touches the sandbox question.

## Cinched
Text render only: the emblem's text lines, identity-led, drawn with the row's existing text rendering — no per-region pills or styling, which are the box-render pace's net-new drawing, not this one's.
File-gated: a row whose enumerated window-id has an emblem file shows it; one that does not shows today's plain entry (never regresses).
Read the emblem file at paint time — no stored frame state in paneboard.

## Done when
- Each alt-tab list entry whose window has an emblem file shows that emblem's text content, identity-led, read fresh from the file.
- A window with no emblem file shows exactly today's list entry.
- A malformed or half-written emblem file degrades to the plain entry, never a crash or a torn render.

### pb-box-emblems (₢BhAAG) [complete]

**[260623-0935] complete**

The emblem render — the sole surface (the list is the untouched switcher, not an emblem surface; see paddock "Overlay surface"):
paneboard paints the three-region emblem on the yellow highlight box it already draws around the selected window during alt-tab.
The box's window-positioning plumbing is reused; the text-and-pill rendering is net-new (the box draws only an outline today).
Paneboard reads the emblem file at paint time, keyed by the selected window's window-id.
The pipeline (writer -> file -> window-id -> read -> display) is already proven by the reverted list probe, so this pace is the box DRAWING, not the plumbing — with one gap to close: the box-draw path (the highlight-border FFI) carries only the window's rect today, so the selected window's window-id must additionally reach the draw site for the file read.

See seed memo Phase 3 and paddock "Overlay surface" + "Paneboard internals".

## Cinched
Net-new drawing: a content view doing three banded sub-rects of multi-line text on fixed black backing pills, branching cleanly on emblem-present vs absent — NOT an extension of the outline-only highlight content view.
Read at paint time (no stored frame state), so a freshly-written emblem always lands on the right window by construction — never stale or mis-bound.
No-emblem fallback: a window with no emblem file paints exactly today's plain box, so this never regresses the current alt-tab overlay.
Replicate the AX-to-Cocoa coordinate flip and the secondary-display setFrame trap from the existing box code, or the label lands off-screen on multi-display.

## Done when
- An emblem file paints the selected window's box with all populated regions, identity-led, per-region style from the file with built-in defaults.
- A window with no emblem file is unchanged from today.

### pb-viewer-conductor (₢BhAAH) [complete]

**[260623-1059] complete**

Layer paneboard over the viewer as its conductor and SOLE lifecycle owner:
paneboard spawns, AX-places, and respawns viewer instances, while the direct-socket viewer keeps working for the no-paneboard / Windows case.

See seed memo Phase 4 and paddock "Transport" + "Paneboard internals".

## Cinched
Spawn + place, never proxy: paneboard positions viewer windows via AX and does NOT proxy image bytes — pushers connect straight to the viewer's advertised port, so paneboard opens no socket and the network-deny posture stays intact.
Because paneboard never reads the bytes, there is no run-loop offload and no background-thread byte read here.

Paneboard is the sole spawner; the pusher never launches the viewer (a paneboard-owned sibling-repo binary), so the pusher's pace is untouched here — it still just reads the port-file, pushes, and fails soft.

Respawn on close: a gone viewer is brought back EMPTY — a declared state, not a bug — and the next push fills it (the viewer must render a graceful no-content state).
A respawn is a fresh process with a fresh port, and the pusher reads the port-file every push, so it picks up the new port with no pusher change.
Respawn is just spawn-again — no new sandbox surface, only the same child-inherits-network-deny question below.

Respawn rides the EXISTING alt-tab path — no new poll: paneboard re-ensures its viewer on the window-switch event it already runs, so a closed viewer reappears at the next alt-tab.
Prompt-on-close (a timer or window-close observer) is deferred, not built now.
No stay-closed dismiss gesture: a deliberately closed viewer returns; a stay-closed toggle, if ever wanted, is a separate later fork.

## Character
Layering, plus one Palisade check — little new risk beyond the spawn-inheritance question below.

## Done when
- Paneboard spawns a viewer instance and AX-places its window; the direct-socket path from the standalone viewer still works unchanged.
- A closed or dead viewer is respawned empty without operator action, and the next push fills it; the empty viewer renders gracefully.
- Verified: whether a paneboard-spawned child inherits the network-deny sandbox. If it does, the viewer is launched independently and paneboard only positions an already-running window — and that resolution is recorded.

### vvx-emblem-identity-derivation (₢BhAAP) [complete]

**[260623-1206] complete**

Derive the emblem work-identity from mount/groom semantics inside JJK, instead of persisting the verbatim jjx_orient halter lede.

Today the dispatcher saddles whatever single halter lede orient received (the emblem-refresh path in Tools/jjk/vov_veiled/src/jjrm_mcp.rs): a firemark lede saddles the firemark even though orient already resolves the next-actionable coronet, and groom (jjx_show) passes None and never sets the identity.
So firemark-vs-coronet is the agent's lede choice, not JJK's — the gap the paddock "Emblem and window reference" rule ("coronet when mounted on a pace, else the heat firemark") was meant to close.

## Cinched
The saddle marker is per-officium officium-resident state, not gallops — no schema change, so no reprieve episode.

## Character
Localized dispatcher change; the resolved next-pace coronet is already in hand at orient time.

## Done when
- Pace mount (orient on a coronet) saddles that coronet.
- Heat mount (orient on a firemark) saddles the resolved next-actionable pace's coronet, not the firemark.
- Groom (jjx_show on a heat) saddles the heat firemark.
- Verified live: heat-mount shows the next-pace coronet on the emblem; groom shows the firemark.

### viewer-unfurl-primitive (₢BhAAO) [complete]

**[260623-1255] complete**

The JJK side of the viewer surface — the one operator verb and its lower command — so RBK and any later producer have a primitive to consume.
See paddock "Viewer surface — a JJK master-UI primitive".

Three parts: the JJS0 vocabulary, the vvx implementation, and the verb-table entry.

## Cinched
JJS0 vocabulary: the upper verb (unfurl) and the lower tool (render) with its anew flag and the two-path light/dark signature.
Mint per MCM Lapidary and the jjdxw_ window family; the surface noun stays plain (no quoin) until JJS0 must reference it.
The upper/lower register gap is load-bearing: the operator verb is vivid, the tool name deliberately boring, never guessable from the verb (JJS0 Upper API anti-pattern-match discipline).
anew is a documented tool param the LLM sets from conversation per a verb-table heuristic — not a deterministic viewer-state rule, not an operator-typed flag.
render is generic: light path required, dark optional, no RB-specific -dark naming baked in.
vvx implementation: read the light (and optional dark) image and push over paneboard's existing wire, mapping anew->fresh / not-anew->update; spawn the viewer if absent; fail soft if the push cannot land.
Transient like the emblem — persists nothing to the gallops, so no schema change and no reprieve episode.

## Done when
- "unfurl <image>" routes through the verb table to the vvx render tool, which displays it (fresh or update per anew), spawning the viewer if needed and failing soft otherwise.
- JJS0 carries the verb, the tool, the anew param, and the verb-table heuristic; the surface noun is left plain.
- A generic single-image unfurl works with no dark variant.

### viewer-light-dark-pair (₢BhAAN) [complete]

**[260623-1336] complete**

The viewer side of the light/dark pair: hold both variants and let the operator switch, proofing exactly what the README <picture> blocks render in each mode.
The pair arrives via the JJK render primitive (light required, dark optional, see paddock "Viewer surface"); this pace is the viewer's DISPLAY of it plus its first keyboard handling.

See paddock "Viewer surface" + "Transport"; the viewer UI (pbgvu_ui.rs) and decode (pbgvd_decode.rs) in the paneboard checkout; the README <picture> blocks.

## Cinched
The viewer holds both decoded variants and switches with the held zoom+pan retained — re-rasterize the other at the held viewport, never a refit.
Theme switch must ALSO flip the viewer backing white<->dark: the dark variant is light ink on transparent, so on today's hard-coded white backing it would be near-invisible.
These are the viewer's FIRST keystrokes — it binds none today: d / l theme, f fit (a manual trigger for the existing fresh/resize fit logic); zoom/pan stay scroll/drag.
Recolor stays OUT of the viewer — it only displays the payloads it received; a single-variant push carries light alone and 'd' falls back to light.
The wire pair is additive (a second optional payload in one frame); do not pair via the instance id.

## Character
Viewer-side display plus the viewer's first keyboard handling. Consumes the JJK render primitive.

## Done when
- The viewer displays a pushed light/dark pair; d and l switch variant (retaining zoom+pan) and flip the backing accordingly; f fits; a small current-mode indicator shows which is up (default light).
- A single-variant push (no dark) renders unchanged, 'd' falling back to light.
- The frozen wire is revised and re-frozen for the optional second payload (poc spec + the pbgvw_ census).

### rbm-diagram-viewer-dogfood (₢BhAAI) [complete]

**[260623-1400] complete**

Recipe Bottle as the first consumer of the viewer primitive — by USE, not by automation.
No bespoke pusher and no tabtarget: the committed rbdg* SVGs are unfurled conversationally through the JJK primitive, and a human works the viewer on real content.
This is the viewer's acceptance gate — proving the primitive is good enough that RB needs nothing of its own.

See paddock "Viewer surface" and the JJS0 "Diagram Viewer" subsection for the primitive; the committed diagrams under diagrams/rbdg*.svg are the dogfood fodder.

## Character
A human dogfood pass plus a short affordance note. No code beyond documentation.

## Cinched
The tabtarget is dropped: a single viewer shows one image at a time, so a batch render-and-unfurl never fit; the edit→render→see loop is the already-deferred fixture-push fork, not built here.
The RB-specific -dark naming convention (foo.svg / foo-dark.svg) is resolved conversationally by the caller, never in the generic primitive.
The deliverable is guidance: a short operator-facing note that a committed diagram can be unfurled for review and the viewer worked by hand — where it lands is a mount-time choice (RBK docs near the diagrams, or the JJK Unfurl affordance).

## Done when
- A committed rbdg* diagram has been unfurled through the primitive and worked by hand — zoom, pan, and the light/dark toggle — with the result judged good, or any viewer gap captured.
- A short affordance note records that diagrams are reviewed by unfurling them, naming the -dark sibling convention.

### viewer-build-folds-into-paneboard-tabtargets (₢BhAAL) [complete]

**[260623-1412] complete**

Fold the standalone viewer crate into paneboard's build so the existing
paneboard build tabtargets build it — the viewer must never have its own build
script.

The viewer currently builds only via raw cargo against viewer/Cargo.toml; the
daemon builds via pbw_workbench.sh's bare cargo behind the pbw-p / pbw-t
tabtargets (poc/ crate).

## Cinched
No individual viewer build tabtarget (operator ruling, 2026-06-22): the viewer
builds inside pbw-p and pbw-t, alongside the daemon, never via a separate script.
Keep the two crates dependency-isolated — separate [dependencies], so the
sandboxed network-denied daemon never links egui/resvg/image.

## Done when
- pbw-p and pbw-t build both the daemon and the viewer crate.
- Those tabtargets still launch the daemon (a workspace makes a bare `cargo run`
  ambiguous across two packages, so the run leg must name the daemon package/bin).
- No standalone viewer build tabtarget or script exists.
- The viewer's run/launch path is out of scope here — this pace is the build only.

### pb-viewer-dies-with-conductor (₢BhAAQ) [complete]

**[260623-1508] complete**

Bind the diagram viewer's lifetime to its conductor so paneboard's death takes the viewer with it.
The viewer's launchd parentage is load-bearing — a paneboard child would inherit the network-deny sandbox and could not listen (paddock "Paneboard internals") — so the viewer outlives a dead conductor: an orphan left listening on its port, with a stale port-file naming a windowless process (observed live 2026-06-23, when a push "succeeded" to a viewer no operator could see).

See paddock "Paneboard internals" for the launchd/sandbox constraint and the conductor in pbmv_viewer.rs; paddock "Transport" for the standalone-viewer invariant; paddock "Cross-repo operation" for the git -C mechanics.

## Character
Paneboard-side conductor plus a viewer-side watch leg. Mechanical, but the standalone-viewer invariant constrains the design.

## Cinched
The viewer must NOT become a paneboard child — that inheritance is exactly what the launchd routing exists to avoid.
So the bind is explicit, not by process parentage: the conductor signals its own liveness to each viewer it spawns (its PID), and the viewer self-exits when that signal goes dead — crash-safe, covering SIGKILL and panic, not only a clean exit.
The watch must tolerate PID reuse — pair the PID with a conductor-owned liveness token (the singleton lock is the natural one), never a bare-PID poll.
The standalone viewer is preserved: launched with no conductor signal it never watches and stays independent (the standalone viewer must keep working for the no-paneboard case).
Defense-in-depth: the conductor also reaps any pre-existing orphan viewer at startup, before spawning a fresh one.

## Done when
- Killing paneboard — both a clean exit and a SIGKILL — makes its spawned viewer exit on its own shortly after.
- A viewer launched with no conductor signal is unaffected: it keeps running with no conductor.
- Starting paneboard reaps any orphan viewer left by a prior run before spawning a fresh one.

### viewer-dies-on-portfile-clear (₢BhAAS) [complete]

**[260624-1214] complete**

Bind the paneboard-spawned viewer's life to the canonical port file's existence, so paneboard cycling its lifecycle closes the viewer.
This is the dead-simple successor to the PID/lock leash, which the macOS seatbelt sandbox defeats.

The sandbox silently strips BOTH argv and env from a LaunchServices `open` launch (verified 2026-06-23 against paneboard's live `(allow default)(deny network*)` profile via sandbox-exec), so the sandboxed conductor cannot signal the viewer through the spawn at all.
The viewer must POLL a file, never be TOLD.
See paddock "Paneboard internals" for the launchd/sandbox launch path this constrains.

## Character
Mechanical; the protocol is fully settled below.

## Cinched
Protocol (operator-decided 2026-06-23):
- Paneboard clears the canonical port file at its own startup.
- The viewer polls that file every ~2s; a missing file means close.
- The viewer clears the file at its own startup, then writes its bound port once known.

Death triggers on the next paneboard STARTUP, not on paneboard's death; the kill-without-restart window is knowingly accepted as sufficient.
Every viewer obeys the one rule, so a hand-launched viewer is no longer kept independent while paneboard runs — this deliberately drops the earlier explicit-per-spawn-signal promise.
No flock, no PID stamp, no conductor→viewer launch signal.

The paneboard working tree currently holds the abandoned flock-leash code uncommitted (conductor.live setup, the viewer-side leash module pbgvl_leash.rs, the --pb-watch passing, the startup reap, the launch debounce); revert or simplify it down to this protocol as part of the pace.

## Done when
- Starting paneboard clears the port file, and any already-running viewer closes within ~2s.
- A viewer running with no paneboard keeps working (it maintains its own port file).

### viewer-port-latency-probe (₢BhAAR) [complete]

**[260624-1232] complete**

## Character
A focused diagnostic plus a recorded finding — empirical and low-stakes; the design decision it informs is deferred, not taken here.

The viewer's TCP port publishes late after a paneboard launch (observed at roughly twenty seconds), so an unfurl fired before it lands fails soft.
Two mechanisms could explain the gap, and they imply opposite fixes:
(a) paneboard services viewer lifecycle only in the alt-tab keypress path, not on its existing run-loop heartbeat, so publication waits for the operator's first tab;
(b) the delay is the viewer child's own egui/Metal cold-start — GPU shader compile plus glyph and font warmup — which no paneboard-side poll can shorten.
The clue that (a) is even possible: paneboard already runs a sub-second run-loop heartbeat (the event-tap health monitor), so if viewer servicing rode that tick, publication would already be prompt — see paddock "Paneboard internals" for the main-thread-only run-loop model.

Conduct the probe to settle (a) versus (b): timestamp the conductor's viewer-launch call against the viewer's port-bind.
Launch firing at startup but bind landing much later is (b) cold-start; launch itself firing only on the first tab is (a) event-gating; a mix is plausible.
Record the verdict where later design can read it — the seed memo (Memos/memo-20260617-paneboard-overlay-and-viewer.md) or the paddock — so the deferred decision rests on fact, not this chat's inference.

Carry forward the consolidation hypothesis the finding informs but does not build:
fold viewer-readiness servicing, and a future embedded resource monitor, onto one periodic run-loop tick — single-threaded, no separate startup thread — at a poll period (roughly 200ms to 1s) consonant with both snappy readiness and low-overhead resource sampling.

## Cinched
The probe is diagnostic only: it builds no poll and no resource monitor — those are later, decision-gated work that this finding informs.

## Done when
- The (a) event-gated versus (b) viewer-cold-start question is settled by a launch-versus-bind timestamp probe, with the verdict recorded where later steps can read it.
- The heartbeat-consolidation hypothesis — one periodic tick servicing viewer readiness and a future resource monitor — is captured alongside the verdict, framed as informed-but-deferred.

### emblem-coronet-sticks-per-officium (₢BhAAT) [complete]

**[260624-1341] complete**

## Character
Small and mechanical: one gating decision in the mount/groom dispatch plus its spec and paddock homes.
No gallops schema change — the saddle marker is officium-resident scratch, not a serialized gallops field — so no reprieve episode.

Mount and groom re-saddle unconditionally today, so the emblem identity flips on every transient groom;
a groom stamps the heat firemark over a working pace's coronet (the friction that seeded this pace).
The saddle marker's presence already records that a first mount-or-groom happened,
so gate the re-saddle on the identity already held instead of overwriting blind —
the orient and show branches of the MCP dispatcher (jjrm_mcp.rs) are the code home.

## Cinched
Coronet-sticks semantics (operator-decided 2026-06-23):
- A mount always (re)asserts its coronet.
- A groom writes the identity only when the slot is empty or holds a firemark — never demoting a coronet.
- Invariant: a coronet, once seen, holds for the officium's life; grooms fill or replace only a firemark.
Scope is per-officium — a /clear opens a fresh officium that re-decides on first engagement.
Reuse the existing saddle marker as the memory; mint no new file.

## Done when
- Grooming a heat while a pace coronet is saddled leaves the coronet on the emblem.
- Grooming first, then mounting a pace, upgrades the emblem to the coronet.
- JJS0 (and the saddle subdoc JJSCSD, where saddle/emblem identity semantics live) states the coronet-sticks rule.
- The heat paddock's Emblem section matches it, no longer asserting an unconditional firemark-on-groom.

### veiled-bhyslop-personal-extraction (₢BhAAU) [complete]

**[260624-1354] complete**

## Character
Design/judgment — a docs-architecture decision plus a careful move, not mechanical.

## Context
The CLAUDE.md "notch before test" placement (this session) surfaced that root CLAUDE.md conflates two orthogonal axes:
distribution (rbm-local vs distributed-to-consumers) and authorship (project doctrine vs personal disposition).
Personal dispositions currently sit in the project-doctrine file (root CLAUDE.md), where they read as project law rather than operator preference.
The veiled claude-<kit>-bhyslop.md layer is the personal-disposition home; claude-jjk-bhyslop.md is the worked precedent.

## Cinched
Kits may each carry their own veiled claude-<kit>-bhyslop.md (kit-scoped personal layer) — preferred over one rbm-level cross-cutting personal file.

## Consider / decide
- Which personal-disposition bits to extract from root CLAUDE.md. Candidates found this session: the "Always notch before you test" rule (added in 7e5fe6f2, now leading Rust Build Discipline), the Test Environments section (operator machines), and the Working Preferences block.
- The cross-cutting wrinkle: a rule spanning kits (notch-before-test governs both vow-t and rbw-ts) has no single kit owner — decide its home (the JJK file, since notch is JJK-native; another kit; or keep in root as a deliberate exception).

## Done when
- A kit-scoped veiled-bhyslop structure decision is recorded.
- The agreed personal bits are moved into the relevant veiled claude-<kit>-bhyslop.md files (or explicitly deferred).

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A pb-guid-window-probe
  2 B vvx-window-reference
  3 C pb-standalone-image-viewer
  4 D wire-protocol-freeze
  5 E jjs0-overlay-concept-landing
  6 F vvx-emblem-writer
  7 M vvx-window-id-resolver
  8 G pb-box-emblems
  9 H pb-viewer-conductor
  10 P vvx-emblem-identity-derivation
  11 O viewer-unfurl-primitive
  12 N viewer-light-dark-pair
  13 I rbm-diagram-viewer-dogfood
  14 L viewer-build-folds-into-paneboard-tabtargets
  15 Q pb-viewer-dies-with-conductor
  16 S viewer-dies-on-portfile-clear
  17 R viewer-port-latency-probe
  18 T emblem-coronet-sticks-per-officium
  19 U veiled-bhyslop-personal-extraction

ABCDEFMGHPONILQSRTU
·x·x·xxx·xxx·····x· jjrm_mcp.rs
····x·····x······x· JJS0_JobJockeySpec.adoc
············x·····x claude-jjk-bhyslop.md
··········xx······· claude-jjk-core.md
·x···x············· vorm_main.rs
x···············x·· memo-20260617-paneboard-overlay-and-viewer.md
··················x CLAUDE.md
·················x· JJSCSD-saddle.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 89 commits)

  1 H pb-viewer-conductor
  2 P vvx-emblem-identity-derivation
  3 O viewer-unfurl-primitive
  4 N viewer-light-dark-pair
  5 I rbm-diagram-viewer-dogfood
  6 L viewer-build-folds-into-paneboard-tabtargets
  7 Q pb-viewer-dies-with-conductor
  8 S viewer-dies-on-portfile-clear
  9 R viewer-port-latency-probe
  10 T emblem-coronet-sticks-per-officium
  11 U veiled-bhyslop-personal-extraction

123456789abcdefghijklmnopqrstuvwxyz
······x····························  H  1c
········xx·························  P  2c
··········xx·······················  O  2c
·················xx················  N  2c
···················xx··············  I  2c
······················x············  L  1c
························x··········  Q  1c
··························x········  S  1c
···························xx······  R  2c
······························x·x··  T  2c
·································xx  U  2c
```

## Steeplechase

### 2026-06-24 13:54 - ₢BhAAU - W

Extracted operator personal-dispositions from root CLAUDE.md into the veiled JJK layer. Moved the 'notch before test' rule and the Test Environments machine registry into claude-jjk-bhyslop.md as new sections H and I, plus a cross-ref from section B's fundus-host list; removed both from root CLAUDE.md (the Rust build targets stayed as project doctrine). Structure decision recorded: kit-scoped veiled claude-<kit>-bhyslop.md files are the personal-disposition home (honoring the cinch), JJK-only this pass — notch homed in JJK as the verb-owner of the cross-cutting test-commit rule, Test Environments homed in JJK as foray/fundus is its dominant consumer. Deferred: BUK and CMK bhyslop files until each accrues enough personal content to justify a file + @-include; the thin Working Preferences bullets (acronym-navigation, bash style, .adoc/.claudex formatting) left in root in place, their continued presence documenting the deferral.

### 2026-06-24 13:54 - ₢BhAAU - n

Relocate notch-before-test discipline and Test Environments registry from CLAUDE.md into the JJK veiled bhyslop file (§H, §I)

### 2026-06-24 13:41 - ₢BhAAT - W

Gate the groom saddle-write on coronet-sticks: a coronet a mount saddles holds for the officium's life; a groom now fills the saddled identity only when none is held or a firemark is, never demoting a working coronet to the heat firemark, while a mount stays unconditional. New zjjrm_standing_is_coronet discriminator (uses the jjrf_Coronet parser, no magic length literal) + a unit test covering coronet-sticks/firemark-fillable/absent/garbage. Rule stated once at JJS0 jjdxw_emblem with a cross-ref from the JJSCSD orient behavior; the heat paddock's Emblem section revised off its unconditional firemark-on-groom. Build + full kit suite green (404+27). Behavioral done-when verified by unit test + code review + the confirmed saddled-coronet baseline; live-paneboard groom dogfood deferred (the running MCP server is the pre-rebuild binary).

### 2026-06-24 13:40 - Heat - S

veiled-bhyslop-personal-extraction

### 2026-06-24 12:52 - ₢BhAAT - n

Gate the groom saddle-write on coronet-sticks: a coronet, once a mount saddles it, holds for the officium's life — a groom now fills the saddled identity only when none is held or a firemark is, never demoting a working coronet to the heat firemark; mount stays unconditional. New zjjrm_standing_is_coronet discriminator uses the jjrf_Coronet parser (no magic length literal) + a unit test covering coronet-sticks / firemark-fillable / absent / garbage. Rule stated once at JJS0 jjdxw_emblem with a cross-ref from the JJSCSD orient behavior.

### 2026-06-24 12:44 - Heat - d

paddock curried: BhAAT: revise Emblem section to coronet-sticks rule (no unconditional firemark-on-groom)

### 2026-06-24 12:32 - ₢BhAAR - W

Probe settled the viewer-port latency: (a) event-gating, not (b) cold-start. Timestamped pb_startup/launch_dispatch/viewer_main/viewer_bind on a shared file (launchd-detached viewer shares no console with paneboard). One cold run: once the launch fires, the port binds in ~1.0s total (0.89s launchd open -g routing + 0.14s eframe/Metal init->bind); the bind precedes first paint so GPU/font warmup never gates the port. The ~20s lag was idle-until-first-tab — ensure_viewer() is wired only to the alt-tab gesture, never to startup nor the existing 500ms health heartbeat. Verdict + the informed-but-deferred consolidation hypothesis (service viewer-readiness on that 500ms tick) recorded in Memos/memo-20260617-paneboard-overlay-and-viewer.md. Probe instrumentation fully reverted (zero PBPROBE, paneboard diff empty); built no poll/monitor per the cinch. Clean standing paneboard relaunched.

### 2026-06-24 12:32 - ₢BhAAR - n

Recorded the viewer-port latency finding in the paneboard memo (pace BhAAR): instrumented four wall-clock markers across a cold launchd-detached viewer launch (throwaway, fully reverted) and settled the ~20 s observed publish delay. Verdict: event-gating, decisively — once the launch fires the port binds in ~1.0 s total (launch→exec 0.89 s, eframe/Metal init→bind 0.14 s); the bind runs inside ViewerApp::new before first paint, so GPU/font warmup never gates it. The ~20 s was the idle-until-first-tab gap, since ensure_viewer() is wired only to the alt-tab gesture, not to startup or the existing 500 ms heartbeat. Captured the informed-but-deferred consolidation hypothesis (move readiness servicing onto the periodic run-loop tick) as the fact it should rest on, without building it.

### 2026-06-24 12:14 - ₢BhAAS - W

Replaced the sandbox-defeated flock leash with the cinched port-file poll close protocol: the viewer clears then publishes the canonical port-file at startup and polls it (~2s), closing when it goes missing; paneboard clears it at its own startup, so a paneboard restart retires any prior viewer. Reverted all abandoned flock-leash code (pbgvl_leash.rs, --pb-watch passing, conductor liveness lock, orphan reap, launch debounce). Committed to paneboard develop (3ddf263), 3 files. Verified: both crates build clean; standalone viewer survives + maintains its port-file; viewer self-closes ~2s after port-file removal; live harness loop confirmed paneboard startup-clear closes a running fresh-code viewer, and the standing relaunch retired a real launchd-orphaned viewer. Done-when both met.

### 2026-06-23 23:17 - Heat - S

emblem-coronet-sticks-per-officium

### 2026-06-23 15:08 - ₢BhAAQ - W

Explored the cinched PID/lock viewer leash — built the viewer-side flock+pid-stamp liveness probe (self-exit on a free lock or pid-stamp mismatch) and the conductor side (conductor.live lock setup, orphan reap, --pb-watch passing, launch debounce). Unit-proved the leash logic against a fake conductor: standalone-independent (no --pb-watch), SIGKILL self-exit, and pid-reuse defense all pass. Live integration then revealed the macOS seatbelt sandbox silently strips BOTH argv and env from a LaunchServices `open` launch (verified via sandbox-exec against paneboard's live (allow default)(deny network*) profile), so the sandboxed conductor cannot signal the viewer through the spawn — defeating the cinched explicit-signal design. Pivoted (operator decision) to a dead-simple port-file-existence protocol, cantled as successor pace BhAAS. The flock-leash code remains uncommitted in the paneboard working tree (../pb_paneboard02), to be reverted/simplified by BhAAS.

### 2026-06-23 15:05 - Heat - S

viewer-dies-on-portfile-clear

### 2026-06-23 14:12 - ₢BhAAL - W

Pace already satisfied by prior work; verified by code read. pbw-b and pbw-t each build the viewer crate (cargo build --manifest-path viewer/Cargo.toml), bundle it into PaneboardViewer.app via pbw_bundle_viewer, then build+run the daemon in poc/. Implemented as two dependency-isolated crates (no root workspace) rather than a cargo workspace, so the daemon's bare `cargo run` stays unambiguous and egui/resvg/image never enter the daemon's tree. No standalone viewer build script exists; pbw-p is retired, the live build pair is pbw-b/pbw-t. All Done-when criteria met.

### 2026-06-23 14:03 - Heat - S

viewer-port-latency-probe

### 2026-06-23 14:00 - ₢BhAAI - W

Dogfooded the JJK viewer primitive on real committed content: unfurled rbdgl_federation-login (light/dark pair) onto a clean pair-capable viewer and worked it by hand — fit, zoom, pan, and the d/l toggle at held zoom+pan — judged good, proving RB needs no viewer tooling of its own. Placed the affordance note as new §G in Tools/jjk/vov_veiled/claude-jjk-bhyslop.md (rbm-specific companion to the public Unfurl Protocol): rbm diagrams are reviewed by unfurling light + -dark sibling and working the viewer keys; the rbdgX_name.svg / -dark.svg convention and zrbtdrc_darken_svg recolor are referenced to the RBDG acronym entry, not restated. Live run also confirmed the dark-transport binary works once the vvx MCP server is refreshed.

### 2026-06-23 14:00 - ₢BhAAI - n

Add veiled JJK §G: diagram review via unfurl (rbm-specific)

### 2026-06-23 13:36 - ₢BhAAN - W

Viewer light/dark pair complete across both repos. Additive pbgvw_dark_len wire revision (re-frozen 2026-06-23): an optional dark payload is appended to the frame; its absence is byte-identical to the prior single-payload frame, never paired via pbgvw_id. The viewer holds both decoded variants and switches with d/l at the held zoom+pan (re-raster, never refit), flips the backing white<->dark (#0d1117), f fits, and a small top-left pill shows the live variant; a single-variant push falls back to light. vvx_render now actually transports the dark variant (the prior pace deferred it here), so unfurl with a dark path delivers a pair. Verified: viewer + kit compile clean; framing test pins both the single and paired control lines; 403 kit + 27 jjrm tests pass; live push confirmed dark=false (single) and dark=true (pair), and operator confirmed d/l/f live. Commits: rbm 24aa860 (jjrm_mcp.rs + Unfurl Protocol doc), paneboard f0db751 on develop (poc spec + 3 viewer src).

### 2026-06-23 13:30 - ₢BhAAN - n

Transport the dark variant in vvx_render: zjjrm_render_control emits the additive optional pbgvw_dark_len, zjjrm_push_viewer reads+appends the dark payload, the report names the landed pair, and the frozen-wire framing test pins both the single and paired control lines. Update the Unfurl Protocol doc — dark is now transported as the pair's second payload, not deferred.

### 2026-06-23 13:05 - Heat - d

batch: 1 reslate

### 2026-06-23 13:05 - Heat - T

rbm-diagram-viewer-dogfood

### 2026-06-23 13:04 - Heat - d

batch: 1 reslate

### 2026-06-23 13:04 - Heat - r

moved ₢BhAAN before ₢BhAAI

### 2026-06-23 12:57 - Heat - S

pb-viewer-dies-with-conductor

### 2026-06-23 12:55 - ₢BhAAO - W

Built the JJK viewer primitive (unfurl verb + vvx_render lower tool + verb table). JSS0 mints jjsuv_unfurl and jjdo_render with a Diagram Viewer framing subsection and a Viewer Operations subsection; jjrm_mcp.rs adds the vvx_render sibling MCP tool (no officium, no gallops) pushing over paneboard's frozen pbgvw_ wire (anew->fresh/update, light-only with dark accepted-but-deferred, fully fail-soft) plus a frozen-wire framing unit test; claude-jjk-core.md adds the unfurl verb-table row and an Unfurl Protocol carrying the anew heuristic. Deviations: render fails soft instead of spawning (conductor owns viewer launch); dark-payload transport deferred to the viewer-pair pace. Built clean, 403 kit tests pass, live-verified fresh/update/fail-soft against the running viewer.

### 2026-06-23 12:43 - ₢BhAAO - n

Build the JJK viewer primitive: mint jjsuv_unfurl (upper verb) + jjdo_render (lower tool) in JJS0 with a Diagram Viewer framing subsection and a Viewer Operations subsection; implement the vvx_render sibling MCP tool in jjrm_mcp.rs — port-file discovery + the frozen pbgvw_ wire push, anew->fresh/update, light-only transport with the dark path accepted-but-deferred, fully fail-soft — plus a frozen-wire framing unit test; add the unfurl verb-table row and an Unfurl Protocol carrying the anew heuristic. render fails soft rather than spawning (the conductor owns viewer launch); dark-payload transport is deferred to the wire-pair pace. Built clean; 403 kit tests pass; live-verified fresh/update/fail-soft against the running viewer.

### 2026-06-23 12:06 - ₢BhAAP - W

Emblem work-identity now derives from JJK mount/groom semantics, not the agent's halter lede: heat-mount saddles the resolved next-actionable coronet (reusing saddle's gazette Pace notice), pace-mount that coronet, groom the heat firemark. The saddle marker grew to a JSON record caching identity + pace/heat silks (officium-resident scratch, no gallops schema change, no reprieve). The emblem middle band was reworked from basename+full-cwd-path to basename / pace-silks / heat-silks (3 lines on a pace, 2 on a heat), each silks line glyph-prefixed with its own identity (coronet on the pace line, firemark on the heat line). All confined to jjrm_mcp.rs; 402 vow-t tests pass incl. exact compose-grammar assertions; live-verified at the emblem file and the alt-tab box render.

### 2026-06-23 12:06 - ₢BhAAP - n

Emblem middle band: cwd basename + glyph-prefixed pace/heat silks

### 2026-06-23 12:04 - Heat - d

paddock curried: refresh emblem starting-content to basename / glyph-prefixed pace+heat silks middle

### 2026-06-23 10:59 - ₢BhAAH - W

Layered paneboard as the SVG viewer's launchd-routed lifecycle owner. Palisade resolved + recorded: a direct child inherits paneboard's (deny network*) sandbox (EPERM on listen+connect vs an unsandboxed control's ECONNREFUSED), so the viewer is launched INDEPENDENTLY via launchd (open -g of an .app bundle) — launchd-parented, it escapes the sandbox and listens on its port; paneboard never forks it. Built: committed viewer/macos/Info.plist + pbw workbench bundle assembly (exports PBGV_VIEWER_APP); poc/src/pbmv_viewer.rs conductor wired to the alt-tab session-start event, relaunching the viewer if absent (rides the existing event, no new poll). Per operator decision (B), AUTO-PLACEMENT was dropped — the switcher selects, layout chords tile, so the viewer is a normal AX window the operator tiles like any other; Done-when #1's 'AX-places its window' intentionally not done. Standalone direct-socket path unchanged; empty viewer renders gracefully (existing code). Live-verified: closing the viewer + Command+Tab respawns it at default placement with no reshape. Paneboard committed 977c768 on develop; rbm paddock curried (sandbox resolution + B correction). Follow-up ₢BhAAP slated for the emblem identity-derivation gap.

### 2026-06-23 10:58 - Heat - S

vvx-emblem-identity-derivation

### 2026-06-23 10:43 - Heat - d

paddock curried: B decision: conductor spawns/relaunches only, no AX placement; viewer tiled via normal layout chords

### 2026-06-23 10:31 - Heat - d

batch: 2 reslate, 1 slate

### 2026-06-23 10:30 - Heat - d

paddock curried: Viewer reframed as a JJK master-UI surface: unfurl verb -> jjx_render(light,dark?,anew) on vvx, LLM-driven anew, viewer keystrokes f/d/l + backing-flip, surface noun deferred

### 2026-06-23 09:51 - Heat - d

paddock curried: record sandbox-inheritance resolution: launchd-route launch, viewer as .app bundle

### 2026-06-23 09:41 - Heat - d

paddock curried: Transport: cinch the viewer-wire optional light/dark pair (additive second payload, recolor single-homed in rbm)

### 2026-06-23 09:35 - ₢BhAAG - W

Alt-tab selection-box emblem render. A separate EmblemContentView (riding on top of the untouched outline-only border view) reads the pbge_ emblem file at paint time, keyed by the selected window's CGWindowID, and draws the regions on black backing pills; window_id is threaded through the highlight-border FFI (show/reposition) to reach the draw site. Absent/empty emblem paints exactly today's plain box (no regression). Refined to the operator-approved four-corner layout: identity in all four corners (pbge_top -> both top corners, pbge_bottom -> both bottom corners) at 84pt white, repo/path centered (pbge_middle) at 28pt yellow; per-region file style overrides the built-in per-placement defaults. Made the writer durable: vvx zjjrm_compose_emblem now emits the identity into pbge_bottom as well as pbge_top. paneboard develop 55a59a8; rbm main 7215e028; paddock + PoC spec soft-content notes updated. vow-t 400 pass, vow-b installed; box render verified live via alt-tab.

### 2026-06-23 09:32 - Heat - S

viewer-light-dark-pair

### 2026-06-23 09:32 - ₢BhAAG - n

vvx writes the work identity into pbge_bottom as well as pbge_top, so the emblem reads from all four corners of the alt-tab selection box. Paneboard side (sibling repo pb_paneboard02 commit 55a59a8 on develop) reads pbge_top into both top corners, pbge_bottom into both bottom corners, pbge_middle centered. paneboard PoC spec soft-content note updated to match; JJS0 jjdxw_emblem already content-agnostic. vow-t 400 pass, vow-b installed. Matched, tested reference point with paneboard 55a59a8 (box render verified live via alt-tab).

### 2026-06-23 09:29 - Heat - d

paddock curried: bottom region now mirrors top identity (four-corner read)

### 2026-06-23 09:07 - Heat - d

batch: 1 reslate

### 2026-06-23 08:40 - Heat - d

batch: 1 reslate

### 2026-06-23 08:40 - Heat - d

paddock curried: overlay surface rewrite: emblem renders on the selection box only; list is the untouched switcher; list probe reverted

### 2026-06-23 08:33 - Heat - T

pb-list-entry-emblems

### 2026-06-22 12:32 - ₢BhAAM - W

Built the vvx window-id resolver — the writer half both paneboard emblem paces depend on. vvx now resolves its own iTerm session to the containing window's CGWindowID by asking iTerm over osascript, and keys the emblem file by that window-id (iterm-session/<window-id>.emblem) so the sandboxed reader reads by the handle it already enumerates; the session UUID is transient-only (the AppleScript lookup key), never persisted, and resolution is cached per vvx process. Settled the open key-form grooming point to window-id direct after load-testing the simplification: the resolver is needed either way (paneboard can't bridge UUID->window inside its sandbox), and the operator's one-session-per-window posture removes the only thing UUID-keying would have bought (multi-session-per-window de-clobber). All in jjrm_mcp.rs (single-file), fully fail-soft. vow-t 400 pass incl. 2 new resolver-helper tests (uuid charset guard, AppleScript composition); vow-b clean. Live-verified the join two ways: osascript UUID->121, and 121 is a real iTerm CGWindowID in the on-screen window list; probe prints iterm-session/121.emblem. Paddock cinched (key decision recorded, two open-grooming flags retired, .json->.emblem staleness fixed); reslated BhAAK to the window-id keying + text-render-only scope.

### 2026-06-22 12:27 - Heat - d

batch: 1 reslate

### 2026-06-22 12:26 - Heat - d

paddock curried: cinch key=window-id direct; retire the two open-grooming flags; fix .json->.emblem staleness

### 2026-06-22 12:25 - ₢BhAAM - n

vvx resolves its iTerm session to the containing window's CGWindowID via osascript and keys the emblem by that window-id (iterm-session/<window-id>.emblem), so the sandboxed reader reads by the handle it already enumerates. UUID is now transient-only (the AppleScript lookup key), never persisted; resolution cached per process, fail-soft on any osascript failure. Settles the open key-form grooming point to window-id direct. 400 tests pass (2 new resolver-helper tests); probe live-verified iterm-session/121.emblem.

### 2026-06-22 12:24 - Heat - S

vvx-window-id-resolver

### 2026-06-22 11:24 - ₢BhAAF - W

Landed and live-verified the vvx emblem writer. vvx now writes its iTerm window's emblem file (frozen pbge_ grammar) on every jjx engagement: work identity from a new officium-resident saddle marker (orient persists its halter lede, every later command reads it back; open/pre-mount degrades to the bare officium handle), per-region style from a new .claude/jjm/jje_emblem.json read at write time (fail-soft per field to paneboard defaults, unknown fields ignored), atomic temp-then-rename to emblems/iterm-session/<uuid>.emblem, fully fail-soft (off-iTerm / no-HOME / refused-dir all silent no-ops). Single hook at the fn jjx dispatcher entry after the model/officium gates, plus the jjx_open degraded case. Verified end-to-end after the MCP restart: degraded-open paint (bare officium), mount->repaint to the full firemark, style-config leg with per-field fallback (color omitted when absent), no .tmp residue. Also corrected the vvx_emblem_probe path literal .json->.emblem via a shared jjrm_emblem_path helper. 398 kit tests pass incl. 6 new (composer grammar, region omit/style, glyph normalize, style fail-soft, atomic write, refused-dir non-regression); deny(warnings) clean. Deferred (out of scope, Done-when met without them): pace-name/silks enrichment in the top region, and content-band config tuning.

### 2026-06-22 11:12 - ₢BhAAF - n

Land the vvx emblem writer: on each jjx engagement vvx composes and atomically writes its iTerm window's emblem file in the frozen pbge_ grammar, fail-soft. Hook at the single fn jjx dispatcher entry (after the model/officium gates) plus the jjx_open degraded case. Identity resolves from a new officium-resident saddle marker — orient persists its halter lede, every later command reads it back, so record/list/close paint the mounted pace; pre-mount/open degrades to the bare officium handle. Per-region style read at write time from a new .claude/jjm/jje_emblem.json (serde, fail-soft to paneboard defaults per field, unknown fields ignored). Atomic temp-then-rename to emblems/iterm-session/<uuid>.emblem; not-under-iTerm / no-HOME / refused-dir all silent no-ops. Corrected the vvx_emblem_probe path literal .json->.emblem via a shared jjrm_emblem_path helper. Six new unit tests; vow-t 398 pass, vow-b clean under deny(warnings).

### 2026-06-22 10:47 - ₢BhAAE - W

Landed the three rbm-side overlay concepts in JJS0 as quoins under a new jjdxw_ (Window emblem overlay) transport band, sibling to Remote Dispatch under Design Principles. jjdxw_emblem (the displayed label, shared cross-repo with pbge_emblem), jjdxw_billet (the typed <scheme>/<value> window reference, modeled on the jjdt_legatio precedent — ephemeral, officium-scoped, explicitly NOT a jjdt_insignia), jjdxw_marque (the named window-identity scheme + resolver leg, iterm-session the sole marque, the two-axis OS×terminal port seam). Added the legend comment line, six mapping-section attribute entries (base + _s each), and the body definition subsection. A disambiguation NOTE keeps the three overloaded 'session' identities apart by word — agent session=officium, Claude Code UUID=invitatory wire field, iTerm identity=billet value under the iterm-session marque — and 'session' is never minted as a jjdxw_ quoin word (iterm-session survives only as a cited paneboard wire literal). billet/marque were grep-clean repo-wide; escutcheon and livery correctly avoided as taken/rejected.

### 2026-06-22 10:47 - ₢BhAAE - n

JJS0: land the Window Emblem Overlay subsection — three rbm-side concepts (jjdxw_ family) for the per-window work-identity label written by vvx and painted by paneboard.

### 2026-06-22 10:26 - Heat - d

paddock curried: testing harness: pbw-b replaces pbw-p; both build tabtargets now build the viewer crate too

### 2026-06-22 10:17 - ₢BhAAD - W

Froze the two ₣Bh contracts in paneboard's PoC spec and matched code to them. Viewer fresh/update TCP wire: re-keyed to the minted pbgvw_ sprue (Control serde renames, verb match, push() construction, module doc), cargo-check clean. Emblem file: frozen as a gazette-cousin three-level pbge_ grammar (pbge_emblem/pbge_pane/pbge_region, frozen pbge_location enum, optional color/size/stamp), one atomic file per window, disk-is-truth with a RAM cache and live-set read, manual pbw-c clear for stale sweep. Minted pbgvw_ (wire) and pbge_ (emblem) sprues — grep-clean repo-wide. Aligned vvx's emblem-path doc to .emblem + the pbge_ grammar. Committed paneboard 9e3e142 (develop) and rbm fff1ec6 (main). Spun out the JSSCGZ no-ordering correction as ₢BDAAi.

### 2026-06-22 10:14 - ₢BhAAD - n

Align the vvx emblem-writer path doc to the frozen contract: emblem file basename is now <value>.emblem (not .json), body is the pbge_ emblem grammar, with paneboard's PoC spec 'Emblem File Format' cited as the authority for both path literal and grammar.

### 2026-06-22 09:02 - Heat - S

viewer-build-folds-into-paneboard-tabtargets

### 2026-06-22 09:01 - ₢BhAAC - W

Built the standalone paneboard-viewer crate (egui/eframe + resvg + image) over a direct localhost-TCP socket: JSON-control-line + length-prefixed-payload framing, fresh/update verbs, ephemeral-port discovery via ~/.config/paneboard/viewer.port, content-sniff format dispatch (SVG via resvg tolerating a leading <?plantuml?> PI, raster via the image crate), white compositing backing, fresh=fit-to-window, update=swap-at-held-zoom with crisp SVG re-raster, zoom-out floored at fit, resize re-fit, and debounced re-raster so smooth-scroll stays fluid. A built-in `push` subcommand is the reference pusher/test driver. Verified live: SVG and PNG render, fresh/update/pan/zoom all work, port-file written on startup. Kept a separate crate so the egui/resvg/image tree stays out of the sandboxed network-denied daemon. Committed to paneboard develop (e9e0ce6, 5364297); rasterization-perf note (tiny-skia CPU/ARM soft spot + Vello/vello_svg upgrade path) added to paneboard-poc.md (21475ca). Build interface (workspace via pbw-p/pbw-t) and the socket-has-no-spoof-auth note for the freeze pace left as follow-ups.

### 2026-06-22 08:03 - ₢BhAAB - W

vvx derives its own emblem target from its environment. Added jjrm_iterm_window_ref (ITERM_SESSION_ID -> scheme-qualified iterm-session/<uuid>, wNtNpN position prefix discarded per cinch, fail-soft None when unset/malformed) and jjrm_emblem_root ($HOME/.config/paneboard/emblems, fail-soft None when HOME unset) in jjrm_mcp.rs, sibling to the Claude-Code session reader; consts JJRM_ITERM_SCHEME / JJRM_EMBLEM_ROOT_TAIL (latter cites paneboard PoC spec as literal authority). Pure zjjrm_parse_iterm_session split out for testing, 2 unit tests (UUID-keying+prefix-discard, fail-soft on malformed) — full suite green. Added hidden vvx_emblem_probe CLI diagnostic in vorm_main.rs exercising both. Done-when confirmed LIVE: built vvx inherits ITERM_SESSION_ID through the Claude-Code child chain (same chain the MCP server rides) and derives iterm-session/AA97D5ED-...-7365 -> .config/paneboard/emblems/iterm-session/<uuid>.json. Next pace's real writer is the MCP engagement path calling these two functions; the CLI probe is only the test harness. Name vvx_emblem_probe awaits operator blessing (keep/rename/un-hide/drop).

### 2026-06-22 08:03 - ₢BhAAB - n

Add emblem window-reference resolver primitives and a probe diagnostic

### 2026-06-22 07:44 - Heat - d

paddock curried: test-harness lifecycle back to agent-control (/clear-survival confirmed, singleton caveat); add remote-Claude-Code named fork (host boundary breaks writer/window co-location, graceful degradation, terminal-stream sub-feature if ever wanted)

### 2026-06-22 07:36 - ₢BhAAA - W

Go/no-go spike for the emblem overlay resolver. Proved the window->iTerm-session-UUID join is EXACT (iTerm's AppleScript window id == the CGWindowID paneboard already keys on via AX _AXUIElementGetWindow, confirmed by matching paneboard's live enumeration against iTerm's session list). Proved the AppleScript resolver CANNOT run inside paneboard: an in-process NSAppleScript call fails -10004 under the seatbelt sandbox (TCC/entitlement gate above the seatbelt layer, since (allow default) already permits appleevent-send; not seatbelt-profile-fixable) -> resolution relocates to the non-sandboxed writer (vvx self-scripts iTerm, resolves UUID->CGWindowID, keys the emblem by that handle; paneboard stays resolver-free and reads by the window-id it already enumerates). Proved the yellow selection box can carry drawn text. Recorded the fork outcome + writer-side reshape in the paddock, plus a direct-process-control test harness and a macOS platform-surface/portability section. Downstream paces (writer/render/spec) now want re-grooming against the writer-side shape.

### 2026-06-22 07:36 - ₢BhAAA - n

memo: mark paneboard overlay/viewer transport design superseded by ₣Bh grooming — labels now ride a file transport (paneboard hard-denies its own network), viewer bytes go direct to the standalone viewer over TCP; register_label wire verb and paneboard-side listener flagged historical, correlation/dead-ends/format-dispatch/push-not-watch still authoritative

### 2026-06-22 07:32 - Heat - d

paddock curried: resolver fork resolved: in-paneboard AppleScript blocked by sandbox (TCC/entitlement, -10004), box-text proven; resolution relocates to non-sandboxed writer (vvx keys emblem by CGWindowID, paneboard resolver-free); cascade updates to overlay-surface/emblem-ref/platform-surface; test-harness work-pattern (operator owns durable relaunch, agent kills-to-test)

### 2026-06-22 07:11 - Heat - d

paddock curried: add platform-surface/portability, direct-process-control test harness, interim resolver status (exact join proven, sandbox permission open)

### 2026-06-22 06:32 - Heat - f

racing

### 2026-06-22 06:04 - Heat - d

paddock curried: pin emblem root: $HOME/.config/paneboard/emblems/<scheme>/<value>.json, paneboard-owned, rbm mirrors with citation

### 2026-06-22 05:54 - Heat - d

batch: 9 reslate

### 2026-06-22 05:52 - Heat - S

pb-list-entry-emblems

### 2026-06-22 05:52 - Heat - T

pb-box-emblems

### 2026-06-22 05:52 - Heat - T

vvx-emblem-writer

### 2026-06-22 05:52 - Heat - T

vvx-window-reference

### 2026-06-22 05:52 - Heat - T

pb-ipc-listener

### 2026-06-22 05:52 - Heat - d

paddock curried: grooming reshape 260622: file-transport for emblems, typed window-reference, list-then-box surface, AAJ dropped

### 2026-06-21 09:00 - Heat - S

pb-ipc-listener

### 2026-06-21 09:00 - Heat - d

paddock curried: add Paneboard run-loop integration cinch (no new thread; payload offload only in conductor), grounded in event-tap code

### 2026-06-21 08:48 - Heat - S

rbm-diagram-push-regen-loop

### 2026-06-21 08:48 - Heat - S

pb-viewer-conductor

### 2026-06-21 08:48 - Heat - S

pb-overlay-render-and-mapping

### 2026-06-21 08:48 - Heat - S

vvx-label-frame-sender

### 2026-06-21 08:48 - Heat - S

jjs0-overlay-concept-landing

### 2026-06-21 08:47 - Heat - S

wire-protocol-freeze

### 2026-06-21 08:47 - Heat - S

pb-standalone-image-viewer

### 2026-06-21 08:47 - Heat - S

vvx-session-key-and-port-discovery

### 2026-06-21 08:47 - Heat - S

pb-guid-window-probe

### 2026-06-21 08:47 - Heat - d

paddock curried: record one-heat-spans-both-repos interleaved decision in Context

### 2026-06-21 08:30 - Heat - d

paddock curried: overlay reshape: three multi-line regions (top/middle/bottom) replace four-corners+center; uniform region struct; one atomic multi-region wire frame, not per-region

### 2026-06-20 10:09 - Heat - d

paddock curried: add pane-label overlay shape: 4-corner identity, center lines, tunable style on wire from rbm config

### 2026-06-20 09:52 - Heat - d

paddock curried: initial paddock: context, references, cross-repo operation

### 2026-06-17 15:48 - Heat - N

jjk-v4-1-svg-viewer-and-pane-labels

