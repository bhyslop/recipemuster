# Memo — Paneboard as window hub: pace overlay + diagram viewer

- **Date:** 2026-06-17
- **Participants:** Brad + Claude Opus 4.8 (1M context), in `rbm_beta_recipemuster`
- **Status:** Design settled; the one hard mechanism proven empirically today (see below); implementation not started.
- **Heat:** intentionally heat-agnostic. This is provenance plus a cross-repo coordination spine, not a docket — enroll heats/paces against it separately.
- **Scope:** spans two repos. See **Repo split** for who owns what.

## What this is

A coordination memo for a feature that lives mostly in the **paneboard** repo (`pb_paneboard02`)
but needs a thin **rbm-side** (vvx) counterpart and a shared wire protocol between them.
It captures the design we settled, the one mechanism we proved today (so it is not re-derived),
the dead-ends (so they are not retried), and — the part requested explicitly —
a **sequencing spine** for building the jjk↔paneboard protocol across both repos.

It is written to be picked up by a paneboard-side JJK instance that lacks this conversation's context:
JJK and Rust literacy assumed; rbm-specific mechanics and today's proof spelled out.

## The two features (one unifying idea)

Paneboard is, by necessity, a singleton — one global keyboard/event tap, one `/tmp/paneboard.lock`.
That singularity makes it the natural hub for two things:

1. **Pace overlay ("alt-tab help") — the higher-value feature.**
   When you alt-tab across many concurrent Claude Code windows, each window's overlay entry is
   labeled with *which JJK pace it is on and in which repo*, so parallel chats across parallel
   repos stop blurring together.
2. **Diagram viewer.**
   A small window displaying a pushed image (the PlantUML SVGs a Recipe Bottle crucible renders),
   updating in place, with paneboard managing the viewer window instances.

Both ride a single paneboard control channel. Build the **viewer first** — it is the lowest-risk slice
and stands up the transport the overlay reuses — then the overlay (see the sequencing spine).

## Settled decisions

- **Image viewer (SVG + raster), standalone, portable binary** — not absorbed into paneboard's macOS
  daemon. Format dispatch by magic-byte sniff: SVG (`<?xml`/`<svg`) → `resvg`; raster (PNG `\x89PNG`,
  JPEG `\xff\xd8`) → the `image` crate, or egui/iced's built-in loader. Both converge on one RGBA
  pixmap, so everything downstream (`fresh`/`update`, zoom, pan) is format-uniform. Shell is `egui` or
  `iced`; Windows-portable (localhost TCP, no macOS-only deps). Paneboard *conducts* viewer instances
  (spawns/places/proxies); it does not *contain* them.
- **Push, not watch.** The sender pushes image bytes over the socket; the viewer is a dumb display
  surface. Regeneration/when-to-refresh policy lives in the sender, not the viewer.
- **Two viewer verbs.** `fresh` = open/replace a window, fit-to-window, default view.
  `update` = replace a window's content while *retaining its current zoom+pan* (re-rasterized at the
  held zoom, so it stays crisp). Owning rasterization is what makes `update` possible — a webview
  could not give this cleanly. The crisp-on-retained-zoom payoff is **SVG-specific**: SVG re-rasterizes
  sharp at any held zoom, whereas a pushed raster (PNG) is fixed-resolution — `update` still preserves
  the viewport but cannot add detail beyond native.
- **Correlation is by session id, not title or focus** (proven today — next section).
- **Tier-0 freebie:** paneboard colors the alt-tab highlight differently when two windows share a
  title. Pure paneboard, no protocol, no jjx — ships disambiguation value on its own.

## The correlation mechanism (the heart) — proven 2026-06-17

**Problem:** a pace label must land on the *right* OS window, but the OS sees one iTerm application
owning all windows — there is no obvious per-window handle from outside.

**Mechanism:** every iTerm session carries `ITERM_SESSION_ID` (e.g. `w6t0p0:0C807AF8-…`), stamped at
session birth, stable for the session's life, inherited down the process tree. The vvx MCP server
(one per Claude Code session) inherits it. So:

1. vvx reads `ITERM_SESSION_ID` from its own environment (free — no tty, no escape codes).
2. On a jjx engagement, vvx best-effort opens paneboard's socket and sends `{session_key, officium, label}`.
3. Paneboard resolves `session_key` → the iTerm window (via iTerm's scripting API) → its AX window
   handle, and binds the label to it.

**Why this beats the alternatives — it is latency-immune.** The session id travels *in the message*,
not read from "what is focused now." A message arriving a full minute late (the LLM may not call jjx
until well into its reply) still carries the correct key; the worst case is the label appears a little
late, never wrong. Focus-at-call-time fails exactly here: focus drifts during that minute.

**Proof — preserve, do not re-derive.** `ps eww` on the live vvx processes, observed from two
different windows in two different repos:

- Each concurrent chat had its *own* vvx process carrying a *distinct* `ITERM_SESSION_ID`; the one
  matching a window's own `$ITERM_SESSION_ID` was that window's server. (5 concurrent sessions, 5
  distinct ids.)
- A freshly-launched session's vvx carried its id *immediately at spawn*; pre-existing sessions' ids
  were untouched by the new launch.
- Conclusion: per-session (not shared), inherits at birth, distinct per window, durable for the
  session's life. All three conditions the strategy needed are met.

**Implementation note:** match on the **UUID** after the colon, not the `wNtNpN` prefix. The prefix is
the session's *original position*, frozen in the env — it goes stale if a tab is dragged to another
window; the UUID does not.

## Dead-ends (do not retry)

- **Setting the window title by printing OSC escapes from a tool/command: dead.** Claude Code captures
  command stdout (the escape is not interpreted), and the process has no controlling terminal
  (`/dev/tty` → "device not configured"). vvx is equally detached.
- **jjx setting the title via AppleScript: abandoned.** It works, but Claude Code actively manages its
  own per-session title and clobbers anything jjx writes — a losing contention.
- **Focus-at-tool-call-time correlation: abandoned.** Tool-call time is decoupled from human-action
  time by LLM latency (up to ~a minute), so focus has usually drifted — a late bind is actively wrong.
  Superseded by session-id, above.
- **System Events / AX automation from a helper: gated.** Triggers a macOS Automation prompt the
  operator declines. iTerm-*direct* AppleScript works without that grant — paneboard's mapping should
  use iTerm's own API, not System Events.

## The protocol (draft — freeze after the walking skeleton validates it)

- **Transport:** localhost TCP (Windows-portable). Paneboard listens; vvx connects per-message,
  **best-effort, fail-soft** (connection refused = paneboard absent = silently skip).
- **Discovery:** paneboard writes its listen port to a known port-file; vvx reads it. (Port-file over a
  fixed port, to survive collisions.)
- **Framing:** a JSON control line terminated by `\n`, optionally followed by `len` raw payload bytes
  for payload-bearing verbs. The payload is self-describing (the viewer sniffs SVG vs raster), so
  PNG/JPEG ride the same path as SVG with no protocol change.
- **Verbs:**
  - `register_label` — `{session_key, officium, label:{repo, <short human-facing pace text>}}`.
    Overlay feature. Paneboard maps `session_key`→window and stores/paints the label. Re-sent on each
    jjx engagement (idempotent; keeps it fresh).
  - `fresh` / `update` — `{id}` + payload bytes. Viewer feature. Paneboard forwards to the viewer
    instance for `id`, spawning one if needed.
  - (later: `raise` / `close`.)
- **Two consumers, one channel:** `register_label` drives the overlay; `fresh`/`update` drive the
  viewer. The viewer must *also* accept direct connections (not only via paneboard) so the
  no-paneboard / Windows case keeps a bare viewer.

## Sequencing spine (the coordination core) — viewer-first

Ordered so the **viewer leads** — it is the lowest-risk slice, delivers standalone value, and stands
up the transport the overlay later reuses — while the overlay's one scary unknown is **spiked in
parallel** so its fork is not deferred. Each item tagged with its repo.

**Phase 0 — Parallel spikes (no commitment).**

- **0a [paneboard] — GUID→window probe (go/no-go for the overlay).** Prove paneboard can take a
  session UUID and resolve it to an AX window handle via iTerm's scripting API, under the permission
  model paneboard runs in (Accessibility — which paneboard already holds — vs Automation, which the
  operator declined for System Events).
  **Fork:** success → session-id overlay as designed; failure → fall back to reading Claude Code's
  existing per-session title (approximate, but free and portable). This forks the whole overlay, so
  run it early and in parallel even though the viewer leads.
- **0b [paneboard] — Tier-0 title-collision color.** Pure paneboard, no protocol. Ships disambiguation
  value immediately and warms up the overlay-rendering code path. Independent of everything.

**Phase 1 — Build the standalone viewer [paneboard, but self-contained].** `egui`/`iced` + `resvg`
(plus the `image` crate for raster) behind a *direct* socket: `fresh`/`update`, retain-zoom, format
sniff. This both delivers standalone diagram-viewing (push by hand — no paneboard hub, no jjx) **and is
the walking skeleton for the whole protocol** — it stands up the framing, the `fresh`/`update` verbs,
and port-file discovery as working code.

**Phase 2 — Freeze the protocol [shared artifact].** With the transport proven by the viewer, write the
wire contract down and freeze it. `fresh`/`update` are already exercised; this mainly nails down
`register_label`'s shape, and is informed by 0a (if mapping failed, its correlation field changes).

**Phase 3 — Overlay ("alt-tab help") [paneboard], on the proven channel.** vvx sends `register_label`
best-effort; paneboard maps `session_key`→window (from 0a) and paints the label on the alt-tab overlay.
Because the channel already exists, this is "add one verb + paneboard-internal rendering," not "build a
protocol and the rendering."

**Phase 4 — Paneboard as conductor [paneboard].** Paneboard proxies `fresh`/`update` to viewer instances
it spawns and places; the direct-socket viewer from Phase 1 keeps working for the no-paneboard / Windows
case. Layers on once both the viewer and the overlay channel exist.

The rbm side stays thin and lands late: vvx only ever *sends* — `register_label` in Phase 3, and the
diagram bytes whenever something pushes them. It never cares how paneboard renders.

## Repo split (who owns what)

- **rbm (`rbm_*recipemuster`) — thin.** vvx changes only: read
  `ITERM_SESSION_ID`, discover paneboard's port-file, and on each jjx engagement send `register_label`
  best-effort / fail-soft. The diagram-push side (`fresh`/`update`) is sent by whatever regenerates the
  SVGs — also rbm-side, but a separate, later concern (see Open risks: regen loop). This slice is the
  rbm JJK heat.
- **paneboard (`pb_paneboard02`) — the bulk.** The socket listener, the GUID→window mapping, the
  overlay rendering, the viewer binary, and the viewer-instance proxy/management. The paneboard JJK
  instance owns this and sequences it by the phases above.

## Open risks / verify-early (honest)

- **GUID→window mapping is UNVERIFIED.** Phase 0a, the single biggest unknown. Do it first; it forks
  the whole overlay.
- **Paneboard has no IPC surface today.** It is a keyboard-driven CFRunLoop daemon; the socket listener
  is net-new infrastructure (a background thread feeding the run loop). Not sized in this memo.
- **"Live" diagram updates need a regen loop that does not exist.** Today the SVGs change only on a
  pluml crucible fixture run. The viewer shows the latest *pushed* bytes; who pushes and when (a hook?
  the fixture? a manual command?) is a separate rbm-side feature. The viewer is useful without it
  (manual `fresh`).
- **Viewer raster zoom.** SVG re-rasterizes crisply on zoom/resize (vector source); a pushed raster
  (PNG/JPEG) is fixed-resolution and pixelates past native. Accepted tradeoff vs a webview's free
  vector zoom; flagged because diagrams are zoom-heavy.
- **Port-file discovery + lifecycle.** Define where the port-file lives and what vvx does on a
  stale/missing file (fail-soft).

## Spec landing zones (where documentation lands)

The two repos document at different formality levels; this records where each side's concept work goes
(surveyed 2026-06-17 — verify, may drift).

**rbm — formal, in JJS0** (`Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc`; an MCM concept model with a
quoin mapping section and a documented sub-letter legend). These features earn a full quoin treatment.
Existing neighborhoods to coordinate with, not collide:

- `jjdxo_` — officium / agent-session lifecycle.
- `jjdxr_` (curia/fundus) + `jjdt_legatio` + `jjsodp_` — the remote-dispatch / IPC-protocol layer; our
  control channel is adjacent.
- `jjsa_` (presentation aliases) + `jjdyr_` (table) — display-side neighbors.
- **New territory, clean to mint:** terminal-window / session-id correlation. Officium identity is
  temporal (`YYMMDD-NNNN`), not terminal-based, so nothing collides.
- **Word-selection hazard:** "session" is already overloaded — an officium *is* a session, the
  chat-capture work keys on the *Claude Code session UUID*, and we add the *iTerm session id*. Three
  distinct identities; MCM Word Selection forces distinct words rather than a reused "session."

**paneboard — informal, in `poc/paneboard-poc.md`** (~1,260 lines, feature-organized, "rigorous
developer notes" register — no quoins). New features slot in as new sections following its template
(Intent → aspects → Logging Contract → Edge Cases). State today:

- IPC / control-channel / session-id / cross-tool integration — **all greenfield.**
- Window tracking by `(pid, window_id)` already exists — the AX handle the GUID→window mapping must
  reach.
- Overlays exist but are single-instance / minimalist (the alt-tab popup with its yellow highlight; the
  tier-0 collision color extends exactly that).

## Entry points (paneboard repo, as surveyed — verify, may drift)

- `poc/src/pbmbe_eventtap.rs` — `run_quadrant_poc()`, the main CFRunLoop (where a socket-listener
  thread attaches).
- `poc/src/pbmba_ax.rs` — AX FFI (window enumeration / handles).
- `poc/src/pbmp_pane.rs` — `focus_window_by_id()` (window focus/position; where GUID→window resolution
  feeds).
- `poc/src/pbmbo_overlay.rs` — the alt-tab overlay FFI (where label text and the tier-0 collision color
  render).
