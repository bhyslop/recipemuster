# Memo — Freeze/Builder Object Pattern (reference artifact)

**Date:** 2026-06-20
**Status:** Reference only. Historical, proprietary, for shape not for porting.
**Artifact:** `codegen-ideas.py` (this directory)
**Relevant heat:** ₣Bg (Matricula)

## What this is

`codegen-ideas.py` is a single-file Python utility library (`gbu_*`, March 2019,
Scale Invariant proprietary) extracted from a prior code-generator project. It is kept
here as the canonical reference for the *object pattern the operator favours*:
**collections of immutables, accumulated during a mutable build phase, then frozen into
immutable structures that are safe to read.**

## The pattern, in its own terms

- `gbu_Freezable` — base class. An object lives mutable (a "builder"), then `fzc_freeze()`
  seals it; thereafter `__setattr__` refuses mutation. The freeze is *triggered on first
  outside read* — the instant anything depends on the interior, the interior locks.
  Pre-/post-freeze cascade lists freeze nested freezables in order.
- `gbu_Immutable` — set-only-in-`__init__` leaves.
- `gbu_IceList` / `gbu_IceDictionary` — collections that freeze on first query and refuse
  further mutation; optional per-element type checks; **the dictionary raises if a key is
  re-inserted with a *different* value** (duplicate-key-conflict detection).
- `gbu_makeFrozenList` / `…Dictionary` / `…Tuple` — build-then-freeze constructors.
- Supporting cast: `gbu_Token` (validated identifier), `gbu_Xan` (opaque id-keyed store),
  `gbu_GrepTag` (generator↔generated linkage), `gbu_Element`/`gbu_loop` (first/last/only
  codegen iteration), `_dieIf`/`_dieUnless`/`_dieNow` (fail-fast exception discipline).

## Why it is kept (relevance to ₣Bg)

The Matricula is a transient census: built by a scan (mutable accumulation), then read,
then discarded. That lifecycle *is* this freeze pattern — build mutable, freeze, query,
drop. Two of the library's invariants map straight onto the MVP violation classes:

- the IceDictionary's "same key, different value → raise" *is* the exact-collision check;
- the freeze boundary *is* the "census complete, now read-only" line.

In Rust the pattern becomes a compiler-enforced typestate (Builder → frozen struct) rather
than runtime `__setattr__` guards — cleaner, but the same shape.

## Provenance, not authority

Reference only. Python, proprietary, built for a different domain (codegen). Do not port
verbatim; mine it for the shape. Per the project memo convention this artifact is
provenance — durable design facts it inspires belong in a vo spec, not here.
