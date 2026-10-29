<!--
Copyright 2026 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

# TEMPLATE — the estate rules that govern your infield

Copy the block under "The text" into your studbook as
`estate/abuttal-infield.md`, fill every «...» substitution, and delete
everything else in this file. Guillemets mark the boundary between what this kit
says and what only you can say; a guillemet left standing is an unfinished file.

## Where it goes, and why it stands in two places

This text and its companion are the written customs every session on a station
loads before it loads anything of a project's. A door emplaces each of them as a
`CLAUDE.md` at the directory it governs — this one in your infield, the directory
every repository of your estate stands in, read out of your studbook's corpus;
its companion, BUK's own text and no estate's, in the directory above it, read
from the kit itself. The emplaced copy is byte-identical to the one the door
reads.

The corpus copy is the canon and the emplaced copy is the echo, which is what the
closing line below says. That line is written to read true in both, so carry it
across unchanged; while it stands in this kit it is neither copy and claims to be
neither.

The engine's working doors refuse to run until this text stands on your
studbook's corpus trunk, so this is founding work rather than tidying. Edit the
canon and re-run the door that emplaces it; never hand-edit an echo.

## What is yours to change, and what is not

Most of what follows is not a policy this kit is recommending — it is how the
engine finds its ground. The engine names no root: your infield is wherever the
launch stand's parent is, under whatever name you give it, and every tree in it
finds its peers beside it. So the text locates itself by the directory holding
it and names its peers as `../<name>`, and it names no absolute path — not the
home directory, not the infield's own place. The studbook clone's dirname and
its darkness, and the corpus road a dispatched session reads canon through, are
shared by every estate running this engine, and changing them in prose does not
change them in the engine.

What is yours is the one substitution: any further tree in your infield that
your estate bars, or opens on terms of its own. The studbook clone is named here
because the engine puts it there; a private store of your own, a client's tree,
a shared clone your own rules govern differently, a tree you keep read-only,
anything a session must ask before reaching — those are yours to name, and
nothing else will name them for you.

## The text

```markdown
# Infield — estate rules

The directory holding this file is an infield: the home of one estate. Every
repo and clone of the estate lives in a subdirectory of it, and each finds its
peers beside it as `../<name>`. The subtree is safe ground for reading, with one
exception named below; the infield itself is not a workspace.

If your working directory is the directory holding this file: no project
context is loaded and unrelated trees are exposed side by side. Do not begin
work and do not read files. Tell the user: "This session started at the
infield root — please restart in a specific project folder," and make no
further tool calls until redirected.

In every session below this directory: the peers beside your own tree may be
read when the task calls for it (shared clones, sibling repos). Cross-peer
writes are governed by the target project's own rules; where no rule covers
one, ask. Each project's own CLAUDE.md governs its work — this file sets estate
geography only and adds no task instructions.

The exception is the studbook clone, `../jjqs_studbook` from any tree in this
infield. It is deliberately dark, and it is not a peer you may read.

- Never read, list, or search it — not by file tools, not by shell, not by git
  reads against its refs. The bar is on the reach itself, whatever the clone
  happens to hold, and it holds however the reach is spelled.
- The bar is on the clone, and on nothing else. A session dispatched onto a
  kraal reaches the corpus through its own corpus road, which stands beside
  that session's working directory: a vulgate billet it may edit, or a
  read-only image it may only read. That road is the corpus for the session
  that holds it — a tree in its own right, read by file tools and by git
  alike, and never a route into the clone. What the bar forbids is the clone
  as a resting store, the side channel; it forbids nothing about the road.
- The engine's own machinery does reach the clone, by ref, as mechanism — that
  is how a corpus road is filled and how canon is rendered. It is the
  `equerry`'s act and never a session's: do not perform one by hand, and do not
  read the `equerry`'s doing so as licence to reach the clone yourself.
- Corpus edits — this estate text among them — go slate-first through a
  vulgate, never by hand into the clone.
- If a task appears to require the clone itself, stop and ask first — do not
  proceed on judgment.

«Any further tree in this infield that this estate bars, or opens on terms of
its own — one short block apiece, in the shape of the one above: name the tree
as a peer, `../<name>`, say what the rule covers and what it does not, and say
what to reach for instead. Delete this paragraph if there are none.»

Canonical copy: `estate/abuttal-infield.md` in the knowledge corpus, reached
through a session's own corpus road. Wherever this text stands outside the
corpus, that copy is an echo and edits belong in the canon.
```
