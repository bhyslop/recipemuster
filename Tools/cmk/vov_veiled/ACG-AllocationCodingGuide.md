<!--
Copyright 2026 Scale Invariant, Inc.
All rights reserved.
SPDX-License-Identifier: LicenseRef-Proprietary
-->

# Allocation Coding Guide (ACG) — Reference the Home

## Purpose

ACG codifies one discipline: **don't recreate inline what already has a named
home — reference the home.** It is the language-neutral, allocation-facing
sibling to BCG (host bash), RCG (host Rust), WSG (Windows transport), and CBG
(cloud step). Those govern *how* code in one environment is written; ACG governs
*where a fact or a name is allowed to live*, and what source must do when a home
already exists: point at it, never restate it.

ACG is the **source-side complement to MCM**. MCM builds the named homes — quoins
in specs, constants in code; ACG governs source's obligation to reference rather
than recreate them. That complement is why ACG lives in the CMK kit beside MCM
and AXLA rather than in any single-language kit.

ACG is **v1**: it blesses and names emergent practice and states the spine. It
does **not** mandate universals ("every verb must announce"); those are
candidates, confirmed move-by-move, not law. There is no qualify-time enforcement
in v1. Every move-type below is licensed precisely — most are detect-and-report,
not repair.

## How this document is organized — frame, then catalog

ACG separates two kinds of knowledge, because they earn their form differently:

- **The Frame** (prose, no IDs) — the spine, the three homes, the v1 posture, the
  move discipline. Systematic and interderivable; internalized, not cited by
  number.
- **The Move Catalog** (numbered `ACGm_`) — discrete, verifier-backed move-types,
  each of which something *points at*: a constant's breadcrumb comment, a
  detect-and-report verb, a conformance-engine row, a review flagging a
  violation. An ID earns its existence only when a citer will exist.

This split is itself the load-bearing test (BCG Core Philosophy): catalog the
discrete moves; systematize the discipline behind them.

## The spine — reference the home

The governing idea, stated once:

> Don't recreate inline what has a named home. **Values → constants; concepts →
> quoin-refs.** MCM builds the homes; ACG governs source's duty to reference
> rather than recreate.

This is the **allocation instance of Load-Bearing Complexity** (CLAUDE.md). A
recreated value or name is a non-load-bearing duplicate: its existence adds drift
risk without adding correctness, and the drift it invites is exactly the gap
between intent and behavior the principle forbids. BCG's and RCG's Constant
Discipline and Interface Contamination are the per-language instances of this;
ACG is the general statement those instantiate, plus the one thing a
single-language guide cannot state — *where prose with no code-home actually
goes.*

## The three homes — allocate by when-read

A fact has exactly one right home, fixed by *when it is read*:

| When read | Home | Carries |
|-----------|------|---------|
| **Design-time** | the spec (MCM quoin) | what a thing *is* and why it is shaped so — the conceptual model |
| **Edit-time** | the source comment | operational mechanics only — language idiom, and the Palisade (foreign-boundary) characterization a maintainer needs *at the code* |
| **Execution-time** | the runtime announcement | what is happening now — intent printed as the operation runs |

Most conceptual comments are **temporal misallocation**: design-time or
execution-time knowledge dumped into the edit-time medium, where it rots for lack
of a forcing function. The spec is re-read when the design is questioned; the
announcement is re-read every run; an edit-time comment restating either is read
by no one with a reason to correct it when it drifts.

Two edit-time/execution-time source-doc forms are distinct and **both blessed**:

- **Contract header** — a bounded edit-time comment stating a function's input
  contract and invariants. Read when editing the function.
- **Intent announcement** — an execution-time printout naming what the operation
  is doing. Read when running it.

Failure-path option disclosure (the missing-parameter-shows-the-options
practice, live in `rbfc_require_vessel_sigil`) is blessed alongside happy-path
announcement: an error that lists the valid forms is an execution-time
announcement of the contract, not contamination.

**The execution-time home is plural off the terminal.** "Announcement" reads as
a single printed stream because a CLI gives you one. A routine whose product
does not go to a terminal — an MCP command, a library call, a daemon — splits
it: the **product** (the value returned to the caller), the **trace**
(diagnostic breadcrumbs), and the **error** travel different media, bound per
project. Naming the full output-role roster is future work; one invariant earns
its place now: **trace has no re-read-every-run forcing function, so it rots like
an edit-time comment — never make it the sole home of design-time knowledge.**

## The move discipline — one litmus, three rules

Every cleanup move obeys one litmus:

> **Mutate only where a *wrong* move is cheaply caught; everywhere else the verb
> is detect-and-report, never repair.**

Code-side moves — the value resolves through a named constant, and shellcheck
plus the fast suite verify it — may mutate now. Spec- and document-side moves are
detect-and-report until the lexer/linter that would catch a wrong move exists.

Three rules ride that litmus:

1. **Triage is part of every move, not an afterthought.** Exclude the
   false-positive class first; declare the authoritative side before any
   alignment; gate scope to settled lanes — never terms a hot heat is still
   moving. A sweep run against vocabulary two heats are still relocating only
   re-creates the drift it meant to kill.
2. **A checker proves itself against a known answer before its output is
   trusted.** The conformance fixture (next section) is the canonical home for
   such checks — ACG eating its own dog food: a named home, referenced not
   recreated.
3. **Concept linkage is name-identity.** The implementing symbol *is* the link: a
   function name equals its quoin's display-text, a tabtarget filename is the
   quoin's sprue, a scoped method is its inlay. A source-to-spec comment is the
   *exception*, reserved for where no single symbol can carry the link.

## The named home — the conformance fixture

Rule 2 and move-types ACGm_102/103 all defer to one concrete home: the
**`conformance`** fast-tier theurge fixture
(`Tools/rbk/rbtd/src/rbtdrn_conformance.rs`), registered in the `fast` suite. It
is to evicted vocabulary what BUK's `bug_require_clean_tree` is to the clean-tree
gate — the single named place the check lives, so no caller improvises its own.

The fixture is a data-driven engine, not a word list. It reads rows of
`{kill_stem, keep_contexts}` and scans `Tools/` and `tt/` with identifier-boundary
awareness, flagging a stem as a bare token while sparing it inside a kept
identifier (`Identifier`) or under an exempt path (`PathPrefix`). A row with empty
`keep_contexts` is a *pure corpse* — the term must appear nowhere. The engine
proves itself against known inputs before its verdict on the live tree is trusted
(move discipline, rule 2), so it ships proven with zero production rows.

**The rule: retire a term → add a row there. Never improvise a grep.** Population
is deliberately deferred — each term gains its row behind its own cutover, owned
by the heat retiring it — never against vocabulary a hot heat is still moving
(move discipline, rule 1).

## The Move Catalog

Each move-type states five things: **Detect** (the rule that finds a site),
**Authority** (the ground truth a site is judged against), **Licensing**,
**Verifier**, and a countable **Done**. Headers tag the licensing: **🔧
mutate-now** (a wrong move is cheaply caught — repair in place) or **🔍
detect-only** (read and report; no repair until a verifier exists). Numbered from
101 to leave room for insertion; once a move has a citer it is never renumbered.

### 🔧 ACGm_101: magic-string → constant

- **Detect:** a literal value — a path fragment, a filename, a repeated number or
  string — that *constructs or addresses* something, where a named home exists or
  should. Not human-facing prose.
- **Authority:** the value resolves through a single named constant home (e.g.
  the `RBCC_rbr*_file` family in `rbcc_Constants.sh`). The constant is ground
  truth; every literal is a copy that can drift.
- **Licensing:** mutate-now — the constant either resolves or the verifier fails,
  so a wrong move dies loud and immediately.
- **Verifier:** bash — `tt/rbw-tl.Shellcheck.sh` + `tt/rbw-ts.TestSuite.fast.sh`
  green; Rust — `tt/rbw-tb.Build.sh` + `tt/rbw-tt.Test.sh` green.
- **Done:** every genuine construction site resolves through the constant; the
  constant carries a breadcrumb to its spec quoin.
- **Triage:** naming a file *for a human* in a message is not a magic string —
  `doc_params`, `buc_info`/`buh_line` messages, and comments keep the literal.
  Exclude that class before sweeping.
- **Worked example:** the `rbrv.env` filename sweep is the first worked
  application of this move.

### 🔍 ACGm_102: name-identity

- **Detect:** an operation quoin (voiced `//axvo_procedure` or `//axvo_method`)
  whose display-text and implementing symbol disagree.
- **Authority:** the implementing symbol is the link (move discipline, rule 3) —
  a procedure's function name equals its quoin's display-text; a source-to-spec
  comment is the exception where no single symbol can carry it.
- **Licensing:** detect-only — document-side; no mutation until the lexer/linter
  exists.
- **Verifier:** none mechanized in v1 — read and reported by hand. The
  conformance fixture is the future verifier.
- **Done:** a report of operation-quoin / symbol disagreements; nothing mutated.
- **Formal dependency:** full mechanical decidability waits on an AXLA arity
  change — an explicit symbol lookahead on `axvo_procedure` / `axvo_method`
  (one `mcm_inlay`, the prefix-disciplined source symbol; mirroring
  `axvo_tabtarget`'s literal lookahead). That change carries a migration across
  every live operation voicing, so it is deferred to its own pace (see *The
  AXLA/MCM interface*). Until it lands, the move is read against the existing
  `axvo_*` voicings — which already mark *which* quoins are operations — plus the
  implementation symbol named at an operation's detail site (`axhems_scoped_method`).

### 🔍 ACGm_103: typed-parameter lookahead

- **Detect:** a parameter whose type is resolvable by bounded lookahead but is
  not yet carried as a type quoin-ref.
- **Authority:** the lookahead scanning rule (below) — the parameter's type is
  the attribute-reference the marker reads.
- **Licensing:** detect-only — document-side.
- **Verifier:** none in v1; the future lexer/linter.
- **Done:** a report of parameters whose type is derivable but unhomed; nothing
  mutated.

## The AXLA/MCM interface

### The lookahead scanning rule

When counting a marker's lookahead in an MCM document, scan the following text and
**skip prose lines and blank lines; count only attribute-references (`{…}` linked
terms) and backtick sprues.** A dimension that counts "the Nth linked term" (e.g.
`axd_grouped`) counts attribute-references only — a backtick inlay or sprue is a
distinct token type and does not shift that count.

### Name-identity's symbol-link dependency (deferred)

ACGm_102 becomes mechanically decidable only when an operation voicing carries its
implementing symbol explicitly. The surveyed shape: add a one-`mcm_inlay`
lookahead to `axvo_procedure` / `axvo_method` reading the literal source symbol
(the MCM-correct token for a prefix-disciplined name, matching how
`axhems_scoped_method` already reads an implementation method name). It is an
*amendment* to two established voicings with many live consumers, not a fresh
mint, so it carries a real migration and is held for its own deliberate pace
rather than folded into guide authoring. Recorded here as ACGm_102's named
dependency; raised and deferred, not dropped.

## Related Guides

- **BCG** — host bash. Constant Discipline and Interface Contamination are the
  bash instances of the spine.
- **RCG** — host Rust. Constant Discipline (Beyond Strings), String Boundary
  Discipline, and Interface Contamination are the Rust instances.
- **WSG**, **CBG** — foreign-environment siblings (Windows transport, cloud step).
- **MCM** — builds the named homes ACG references; the design-time home.
- **AXLA** — the motif vocabulary; the lookahead scanning rule and the
  name-identity symbol-link dependency live against it.
- **CLAUDE.md "Prefix Naming Discipline" (mint)** — governs the naming homes
  themselves.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| ACG | Allocation Coding Guide (this document) |
| The spine | "Reference the home" — values → constants, concepts → quoin-refs |
| Home | The one right location for a fact, fixed by when it is read (design / edit / execution time) |
| Move-type | A catalogued `ACGm_` cleanup move: detect-rule, authority, licensing, verifier, countable Done |
| mutate-now | Licensing: a wrong move is cheaply caught, so the verb may repair in place |
| detect-only | Licensing: the verb reads and reports; no repair until a verifier exists |
| Name-identity | Concept linkage where the implementing symbol *is* the link to its quoin |
