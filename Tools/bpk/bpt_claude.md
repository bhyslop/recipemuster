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

# TEMPLATE — the include block a repository's CLAUDE.md declares

Copy the block under "The block" into your own repository's `CLAUDE.md`, keep
the lines whose kits you installed, drop the rest, and delete everything else in
this file. Guillemets mark the boundary between what this kit says and what only
you can say; a guillemet left standing is an unfinished file.

No door writes your `CLAUDE.md`, at install or ever. It is yours, and this is the
one part of it the kits have anything to say about.

## What an include line is

A declared include is a line whose whole content is `@` followed by one
path with no whitespace in it, written relative to the repository root:

```
@Tools/buk/buai0/buais_buk.md
```

Four rules, each of which a session's context composition enforces:

- An `@` inside running prose is not an include and is left alone, so this
  document can name one without declaring it.
- An `@path` inside a fenced code block is not an include either — the block
  above shows the form rather than declaring it.
- The path stays inside your own repository. An absolute path, or one climbing
  through `..`, is refused rather than followed.
- Anything after a whitespace on the line makes it ordinary prose, so a line
  carrying a comment beside its path declares nothing.

## The salutation

Declare no include for it. The dispatch places the salutation into each
session's composed context itself, at the session's tier, so it reaches every
dispatched session whatever your CLAUDE.md says. A session opened in your
repository without a dispatch has none.

## The block

```
@Tools/cmk/cmai0/cmais_roe.md
@Tools/cmk/cmai0/cmais_cmk.md
@Tools/cmk/cmai0/cmais_minting.md

@Tools/jjk/jjai0/jjais_jjk.md

@Tools/buk/buai0/buais_buk.md

@Tools/vvk/vvai0/vvais_vvk.md
```

One line per context file of a kit this repository receives by parcel, as
`BURC_MANAGED_KITS` declares it. A kit you did not install has no such file on disk, and a declared include that resolves
nowhere refuses the dispatch rather than being skipped — so the list of lines and
the list of kits move together.

Add your own repository's context files to the block the same way, and say in
prose around the block what your repository is for: a session reads this document
whole, and the includes are only the part the kits contribute.
