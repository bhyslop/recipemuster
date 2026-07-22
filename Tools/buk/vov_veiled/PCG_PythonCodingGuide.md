<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Python Coding Guide (PCG) — Pattern Reference

*Guide-family member — see GMG (Guide Meta-Guide) for the family's shared authoring conventions.*

## Purpose

PCG codifies the project's Python authoring discipline. It is the Python
sibling of RCG (Rust) and BCG (bash) — a first-class-language guide, not a
foreign-environment one: the interpreter is ours to configure, so there is no
Palisade here, only house convention.

Like RCG, PCG is deliberately minimal. Trust Claude's inherent Python idioms
for error handling, data design, and general style; this guide covers only the
project-specific conventions that override or extend standard Python practice.
The two overrides that define it: **minting discipline** applied to a language
that thinks it already has a privacy convention, and **annotations as the
declared contract** in a language that will run happily without one.

Examples below use the illustrative prefix `spqr` — not a minted name; real
modules substitute their own minted prefix.

## Core Philosophy

**Minting is the primary concern.** All identifiers follow the project's
prefix naming rules (claude-cmk-minting.md "Prefix Naming Discipline") for
grep-ability and provenance: `grep spqr_` is the census of a module's names,
and that axiom holds at every use site, not only at declarations.

**The signature is the contract.** An unannotated Python function is a
verdict-less boundary — the caller learns its shape by reading the body or by
crashing. Every `def` declares its full contract in types (see Type Annotation
Discipline). This is the Python instantiation of the family's crash-fast,
no-silent-failures philosophy (BCG headwater — cited, not restated).

**Privacy is spelled the house way.** Python's leading-underscore convention
is deliberately not used. Privacy is the `z` prefix, uniform with bash and
Rust, and the module is the privacy unit.

## File Naming

Python source files follow the Primary Universe default: `{prefix}_{word}.py`,
lowercase, prefer one word (`spqr_assay.py`). Test files follow the same
minting rules; configure the test runner's discovery pattern to the house
names rather than bending names to the tool's default.

## Declaration Naming

**All public declarations in a module start with that module's prefix; private
ones are `z`-prefixed.** Non-negotiable, same as RCG.

For a module with prefix `spqr`:

| Declaration | Public Pattern | Example | Private Pattern | Example |
|-------------|---------------|---------|-----------------|---------|
| Function | `{prefix}_snake_name` | `spqr_load()` | `z{prefix}_snake_name` | `zspqr_validate()` |
| Class | `{prefix}_PascalName` | `spqr_Gauge` | `z{prefix}_PascalName` | `zspqr_Output` |
| Constant | `{PREFIX}_SCREAMING` | `SPQR_DEFAULT_PATH` | `Z{PREFIX}_SCREAMING` | `ZSPQR_MAX_DEPTH` |
| Type alias | `type {prefix}_AliasName` | `type spqr_Result = ...` | `type z{prefix}_AliasName` | `type zspqr_Row = ...` |

- **Methods on prefixed classes carry the prefix** (`gauge.spqr_save(path)`) —
  same rationale as RCG's impl methods: provenance is immediate at the call
  site, and "call `spqr_save`" is unambiguous in discussion.
- **Protocol (dunder) methods are exempt.** `__init__`, `__repr__`,
  `__enter__` and kin are the language's wire format — foreign names keep
  their foreign form, exactly as foreign JSON schemas keep foreign keys.
- **Local variables use normal names, no prefix, no `z`.** Python's function
  scope makes them unambiguous, as Rust's block scope does; only bash's flat
  scope needed more.
- The prefixed-PascalCase class shape diverges from PEP 8's CapWords on
  purpose, exactly as RCG's `#![allow(non_camel_case_types)]` blesses the same
  divergence in Rust.

## Privacy: the z prefix, not the underscore

Python convention marks privates `_name`; this project spells them
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
enforcement. No checker polices `z`-privacy — it is convention-only, exactly
as it is in bash. The declared `__all__` recovers star-import hygiene and
doc-tool visibility; the enforcement gap is accepted.

## Import Discipline

The RCG import and alias rules carry over whole, because the census argument
is language-independent:

- **One item per line, alphabetically ordered**, in parenthesized
  `from module import (...)` lists — change-proportional diffs, deterministic
  insertion points.
- **No `import … as` / `from … import … as` — anywhere, for anything.** A
  rename gives the body a second name for a thing that already has exactly
  one, and every aliased use site is invisible to the census grep. Foreign
  packages keep their declared names (`import numpy`, never `as np`); when two
  foreign names collide, import the frequent one and write the rare one
  fully-qualified at its use sites.

## Type Annotation Discipline

**Every `def` is fully annotated** — every parameter and the return, `-> None`
spelled explicitly. Public and private alike: uniform rules hold where judged
ones drift.

**Modern spelling only**, which the interpreter floor guarantees is available:

- Builtin generics: `list[str]`, `dict[str, int]` — never `typing.List`.
- Unions: `str | None` — never `Optional`, never `Union`.
- Aliases: PEP 695 `type spqr_Result = dict[str, spqr_Gauge]`.

**Enforcement is deferred, and that deferral is named.** No type checker runs
today (an operator ruling, alongside the formatter/linter deferral below).
Until one lands, annotations are review-enforced, and an unchecked annotation
that drifts from the body is a defect in review exactly as a wrong comment
would be. Checker adoption (pyright or mypy, strict) is the standing
follow-up that retires this paragraph; annotations written now are the
contract it will verify then.

## Interpreter Floor

**Python 3.12 is the floor.** It is the Ubuntu 24.04 LTS system interpreter
(the project's Linux test hosts), it completes the modern typing spelling
above (PEP 695), and it holds security support into 2028. The floor is a
floor: code must run on 3.12 and may not require newer features.

## Environment Discipline

**Code never assumes an ambient interpreter or site-packages.** The
interpreter and its dependencies are guaranteed by the entry path — a
tabtarget dispatching through the launcher system into a project-managed
virtual environment — never by the code itself:

- No runtime `pip install`, no `sys.path` surgery, no in-code venv activation.
- A module's imports are satisfied by the environment its entry point
  guarantees, or they crash-fast at import time — which is the correct
  failure.

The venv lifecycle itself — creation, refresh, pinning — is BUK's to own,
beside the tabtarget/launcher machinery that guarantees it (module pending;
not yet minted). PCG governs only what the Python author may assume: nothing
the entry path did not provide.

## Formatting and Linting

None for now — an operator ruling with considerations beyond this guide;
conventions here are review-enforced (as RCG's import layout is, absent a
stable rustfmt mode). This section is the placeholder the eventual tooling
decision replaces.

## Related Guides

- **RCG** — the closest sibling: the same minting table, z-prefix uniformity,
  alias ban, and interface-contamination philosophy, in Rust.
- **BCG** — the headwater philosophy (crash-fast, no silent failures,
  load-bearing complexity) and the origin of the `z` convention.
- **GMG** — the guide-family conventions this document is written under.
- **claude-cmk-minting.md** — the minting doctrine every naming rule here
  instantiates.

---

This document is a seed, drafted alongside the minting factor-out that freed
its acronym. No Cited Rules yet — Python has shown no foreign signature worth
a numbered ID; the section accretes when a citer exists.
