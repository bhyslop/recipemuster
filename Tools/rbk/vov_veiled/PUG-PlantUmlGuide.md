<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# PlantUml Guide (PUG) — Diagram Source & Render Discipline

## Purpose

PUG codifies the discipline for the project's PlantUML diagrams: how the
committed `.puml` sources are authored, and how they are rendered to the
committed `.svg` siblings linked from `README.md`. It is a foreign-environment
sibling to CBG (Cloud Build step bodies) and WSG (ssh-to-Windows transport),
sharing the BCG/RCG philosophy — crash-fast, no silent failures, load-bearing
complexity — but its mechanics are dominated by one fact: **the renderer is not
ours.** PlantUML sits at the Palisade (CMK Rules of Engagement). We cannot edit
how it decodes our input; we can only conduct ourselves precisely at the
membrane where our text crosses into it.

## How this document is organized — two genres on purpose

Like CBG, PUG separates two kinds of knowledge:

- **Authored Disciplines** (prose, no IDs) — patterns for what *we* control: the
  `.puml` sources and the render path. Internalized, not cited.
- **Cited Rules** (numbered `PU*_`) — discrete facts about the PlantUML Palisade
  we do *not* control, each of which something points at (a render comment, a
  memo, a review flag, a gate). An ID earns its existence only when a citer will.

## Core Philosophy

**PlantUML is at the Palisade.** The renderer — here the PlantUML server in the
`pluml` crucible — is a neighbor we cannot legislate. Per the CMK ROE:
characterize the foreign behavior precisely, contain dependence at one membrane,
absorb only the surveyed signature, log the bend with a removal condition.

**Encode past the neighbor; don't trust its decode.** The strongest membrane
removes the foreign decision entirely rather than depending on the neighbor to
make it correctly. HTTP charset negotiation is such a decision: the robust render
path encodes UTF-8 source into charset-independent ASCII *before* transit
(PUr_101), so no charset is ever negotiated on the wire.

**The render is one membrane, and byte-stable.** Diagrams render through exactly
one path — the `pluml` crucible render case (`rbtdrc_pluml_render_diagrams`). No
ad-hoc rendering. Identical source yields byte-identical SVG, so source and SVG
commit in lockstep and a clean tree stays clean; a diff in a committed SVG means
a diff in its source, never render nondeterminism.

## Authored Disciplines (prose — internalize, don't cite)

### Sources are UTF-8; non-ASCII is deliberate

`.puml` sources are UTF-8. Every multibyte glyph (em dash `—`, typographic
quotes, arrows) is a hostage to the render transport (PUr_101). Prefer plain
ASCII in diagram text where it reads as well; when a multibyte glyph earns its
place, know it rides entirely on the render path getting its bytes across intact.

### Render through the charset-independent transport

Render via PlantUML's deflate-encoded URL form, not raw-text POST that leaves the
body charset to the server. The encoding (UTF-8 → deflate → URL-safe ASCII; see
`plantuml.com/text-encoding`) is *independent of HTTP charset negotiation* — that
is the whole point: it carries exact UTF-8 bytes through a neighbor that
otherwise mis-decodes them. See PUr_101 for the failure this avoids and the one
fallback.

### Enforcement is mechanical, not advisory

A guide a future instance can forget is the weak form of "forbid by constraint."
The intended mechanical arm (**not yet built**) is a gate that rejects any
committed SVG carrying the mojibake signature `&#226;&#8364;` — the ISO-8859-1
misread of a UTF-8 multibyte lead. The render case already fails loud on a
non-SVG / syntax-error response; the charset gate extends that to
corrupt-but-well-formed output. Until it exists, this prose is the only fence.

## Cited Rules (numbered — each has a citer)

### PUr_ — PlantUML render transport (the server we don't control)

**PUr_101 — raw-text POST body decodes as ISO-8859-1 absent a charset.**
The PlantUML server's text endpoint (`POST /svg/uml`) decodes the request body
as the servlet default **ISO-8859-1** when the `Content-Type` carries no
`charset`. UTF-8 multibyte input is then read byte-by-byte as Latin-1 and frozen
into the SVG as per-byte numeric entities — a UTF-8 em dash (`E2 80 94`) becomes
`&#226;&#8364;&#8221;` (`â€"`). We send correct UTF-8 bytes; the corruption is the
server's decode. Independently reproduced upstream (PlantUML forum: `Queen's` →
`Queenâ€™s` on POST while the same source renders cleanly via GET, even though the
server reports "Default Encoding: UTF-8").

- **Membrane (robust):** render through the deflate-encoded URL transport
  (UTF-8 → deflate → URL-safe ASCII; `plantuml.com/text-encoding`), which is
  *independent of HTTP charset negotiation*. This is the Palisade-correct fix —
  it encodes past the neighbor's decode rather than depending on it.
- **Fallback (cheaper, unverified):** set `Content-Type: text/plain;
  charset=utf-8` on the existing POST (`rbtdrc_pluml_render_diagrams` →
  `rbtdrc_curl_post_stdin`, `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`). Servlet
  request-charset *may* be honored, but the upstream forum reproduces the bug
  without confirming a header fix, and it still rests on the server's decode path
  — so it must be live-verified (charge `pluml`, re-render, diff the SVG) before
  trust. Apply at the call site, never the shared POST helper, whose other
  callers must not be perturbed.
- **Removal condition:** the server's POST body decode defaults to UTF-8 — then
  the encode-past transport is no longer load-bearing for correctness (still fine
  to keep for URL-safety).

---

This document is a seed, founded on PUr_101 — the em-dash mojibake in the
federation-tier diagrams. Further PlantUML disciplines accrete here as found.
