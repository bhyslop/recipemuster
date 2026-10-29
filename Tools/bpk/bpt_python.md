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

================================================================================
TEMPLATE — the Python authoring discipline a repository writes code under.

Copy this whole file into your own corpus under your guides directory, naming it
by your own document-naming scheme, then delete this template block — the
licence block above it stays. Fill every «...» substitution. Guillemets mark the
boundary between what this kit says and what only you can say; a guillemet left
standing is an unfinished file.

Four substitutions stand in the document, and each is a decision this kit cannot
make for you: where your prefix naming rules are homed, whether a type checker
runs, whether a formatter and linter run, and which sibling guides this one sits
beside. Three of the four are tooling rulings; leaving one unfilled leaves the
guide unable to say what it is for.

This document is a starting point and expects to be tuned. From the moment you
copy it, it is yours: this kit's copy is replaced whole by the next parcel and
reaches your copy never.
================================================================================
-->

# Python Coding Guide — Pattern Reference

## Purpose

This guide codifies the estate's Python authoring discipline. It is a
first-class-language guide rather than a foreign-environment one: the
interpreter is yours to configure, so nothing here works around someone else's
tooling — it is all house convention.

It is deliberately minimal. Trust Claude's inherent Python idioms for error
handling, data design, and general style; this guide covers only the
project-specific conventions that override or extend standard Python practice.
The two overrides that define it: **minting discipline** applied to a language
that thinks it already has a privacy convention, and **annotations as the
declared contract** in a language that will run happily without one.

Examples below use the illustrative prefix `spqr` — not a minted name; real
modules substitute their own minted prefix.

## Core Philosophy

**Minting is the primary concern.** All identifiers follow the estate's prefix
naming rules: every persistent name starts with a registered project prefix, and
it does so for grep-ability and provenance. `grep spqr_` is the census of a
module's names, and that axiom holds at every use site, not only at
declarations. A minted name is therefore never synthesized: no `setattr`, no
`globals()[…]` assignment, no dynamic class or attribute built from fragments,
because such a name exists at no site any grep can reach — the declaration-side
half of the same axiom.

> «Name here where your estate's prefix naming rules are homed, and where its
> registry of project prefixes stands. This guide instantiates those rules in
> Python and does not restate them; a reader who cannot reach them cannot mint.»

**The signature is the contract.** An unannotated Python function is a
verdict-less boundary — the caller learns its shape by reading the body or by
crashing. Every `def` declares its full contract in types (see Type Annotation
Discipline). This is the Python instantiation of the crash-fast,
no-silent-failures philosophy the rest of the estate's code is written under: a
boundary either states its contract or forces its caller to discover it the hard
way, and the second is not a style preference to weigh but a defect.

**Privacy is spelled the house way.** Python's leading-underscore convention is
deliberately not used. Privacy is the `z` prefix, uniform with bash and Rust, and
the module is the privacy unit.

## File Naming

Python source files follow the same default every other persistent name follows:
`{prefix}_{word}.py`, lowercase, prefer one word (`spqr_assay.py`). Test files
follow the same minting rules; configure the test runner's discovery pattern to
the house names rather than bending names to the tool's default.

A toolchain-fixed filename holds no prefix of its own, so the package's prefix
stands in its place. `__main__.py`, `conftest.py` and their kin are named by the
interpreter or the runner rather than by us and cannot carry a prefix at all, so
declarations in them wear the package's. A filename we chose has no share in that
relaxation, however little the name suits what it holds.

## Declaration Naming

**All public declarations in a module start with that module's prefix; private
ones are `z`-prefixed.** Non-negotiable, as in every other language the estate
writes.

**The rule's subject is the module, and it ends at a `def`'s colon.** A constant,
a nested function or a class declared inside a function body is exported by
nothing and reachable from nowhere else, so no grep for it can land wrong and no
discussion can be ambiguous about which one is meant; such a declaration wears
whatever name reads best where it stands. This is a statement of where the law's
subject ends, never a carve-out granted to Python idiom.

For a module with prefix `spqr`:

| Declaration | Public Pattern | Example | Private Pattern | Example |
|-------------|---------------|---------|-----------------|---------|
| Function | `{prefix}_snake_name` | `spqr_load()` | `z{prefix}_snake_name` | `zspqr_validate()` |
| Class | `{prefix}_PascalName` | `spqr_Gauge` | `z{prefix}_PascalName` | `zspqr_Output` |
| Constant | `{PREFIX}_SCREAMING` | `SPQR_DEFAULT_PATH` | `Z{PREFIX}_SCREAMING` | `ZSPQR_MAX_DEPTH` |
| Type alias | `type {prefix}_AliasName` | `type spqr_Result = ...` | `type z{prefix}_AliasName` | `type zspqr_Row = ...` |

- **Methods on prefixed classes carry the prefix** (`gauge.spqr_save(path)`) —
  provenance is immediate at the call site, and "call `spqr_save`" is
  unambiguous in discussion.
- **Protocol (dunder) methods are exempt.** `__init__`, `__repr__`, `__enter__`
  and kin are the language's wire format — foreign names keep their foreign form,
  exactly as foreign JSON schemas keep foreign keys.
- **Test functions wear their own test module's prefix**, as every other
  declaration in it does; the runner's discovery pattern is configured to that
  shape rather than the names bent to the tool's default `test_`.
- **Local variables use normal names, no prefix, no `z`.** Python's function
  scope makes them unambiguous, as Rust's block scope does; only bash's flat
  scope needed more.
- The prefixed-PascalCase class shape diverges from PEP 8's CapWords on purpose,
  exactly as the estate's Rust suppresses the analogous lint to bless the same
  divergence there.

## Privacy: the z prefix, not the underscore

Python convention marks privates `_name`; this estate spells them
`z{prefix}_name` instead, and recovers what the underscore provided
declaratively:

- **`__all__` is mandatory in every module**, listing exactly the public
  (`{prefix}_`) names. It is the module's declared public manifest — the same
  statement the underscore made by spelling, made once and greppably.
- **Star imports are banned** (`__all__` is a manifest, not an invitation).
- **The module is the privacy unit.** A `z` name is internal to its module's
  conceptual boundary; nothing outside it calls one — except tests.
- **Tests call `z` names freely.** No lint ceremony, no protected-access
  suppressions — the testability that motivated the choice.

Named honestly, what the underscore bought that `z` does not: static
enforcement. No checker polices `z`-privacy — it is convention-only, exactly as
it is in bash. The declared `__all__` recovers star-import hygiene and doc-tool
visibility; the enforcement gap is accepted.

## Import Discipline

The estate's import and alias rules carry over from its other languages whole,
because the census argument is language-independent:

- **One item per line, alphabetically ordered**, in parenthesized
  `from module import (...)` lists — change-proportional diffs, deterministic
  insertion points.
- **No `import … as` / `from … import … as` — anywhere, for anything.** A rename
  gives the body a second name for a thing that already has exactly one, and
  every aliased use site is invisible to the census grep. Foreign packages keep
  their declared names (`import numpy`, never `as np`); when two foreign names
  collide, import the frequent one and write the rare one fully-qualified at its
  use sites.

## Type Annotation Discipline

**Every `def` is fully annotated** — every parameter and the return, `-> None`
spelled explicitly. Public and private alike: uniform rules hold where judged
ones drift.

**Modern spelling only**, which the interpreter floor guarantees is available:

- Builtin generics: `list[str]`, `dict[str, int]` — never `typing.List`.
- Unions: `str | None` — never `Optional`, never `Union`.
- Aliases: PEP 695 `type spqr_Result = dict[str, spqr_Gauge]`.

**Enforcement is a ruling this estate owes, and it is stated here rather than
left implicit.**

> «Say whether a type checker runs, and which — pyright or mypy in strict mode is
> the usual pair. Until one runs, annotations are review-enforced, and an
> unchecked annotation that has drifted from its body is a defect in review
> exactly as a wrong comment would be; say that plainly rather than leaving the
> reader to guess which state the estate is in. Annotations written under a
> deferral are the contract the eventual checker verifies, so the deferral costs
> nothing at the keyboard.»

## Interpreter Floor

**Python 3.12 is the floor.** It completes the modern typing spelling above
(PEP 695) and holds security support into 2028. The floor is a floor: code must
run on 3.12 and may not require newer features.

The interpreter a project actually runs is never the station's, so no system
interpreter's version bears on this floor. It is the exact patch version that
project's own declarations name, at or above the floor.

## Environment Discipline

**Code never assumes an ambient interpreter or site-packages.** The interpreter
and its dependencies are guaranteed by the entry path that launches the module —
never by the code itself:

- No runtime `pip install`, no `sys.path` surgery, no in-code venv activation.
- A module's imports are satisfied by the environment its entry point
  guarantees, or they crash-fast at import time — which is the correct failure.

That environment's lifecycle — convergence, refresh, pinning — belongs to
whatever tooling stands up the entry path, and is stated in that tooling's own
documentation rather than here; where this estate installed the kennel kit, the
kennel's own README states it. This guide governs only what the Python author may
assume: nothing the entry path did not provide.

## Formatting and Linting

> «Say whether a formatter and a linter run, and which. Where they do not, say so
> and say that the conventions above are review-enforced instead — an unstated
> deferral reads to a fresh session as an oversight, and it will try to install
> one. This section is the placeholder the eventual tooling decision replaces.»

## Related

> «The sibling guides this one sits beside in your corpus — your other language
> guides, and the document your prefix naming rules are homed in. This guide
> names none, because the estate it was written in is not the estate it now
> stands in; the names here should be yours.»
