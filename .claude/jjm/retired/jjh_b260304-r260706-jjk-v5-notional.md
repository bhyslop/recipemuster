# Heat Trophy: jjk-v5-notional

**Firemark:** ₣Am
**Created:** 260304
**Retired:** 260706
**Status:** retired

## Paddock

# Paddock: jjk-v5-notional

## Context

Parking lot for post-V4 JJK improvement ideas. Paces here were relocated from ₣AG (jjk-post-alpha-breaking, retired 260304) during V4 planning triage. They represent genuine needs but are not in scope for the V4 schema transition.

These paces may have V3-era assumptions in their dockets. Before promoting any pace to an active heat, review and update for V4 compatibility.

## Status

Stabled indefinitely. Not a work queue — a curated idea backlog. Paces may be:
- Promoted to a new heat when the time is right
- Further refined as V4 implementation reveals new constraints
- Abandoned if V4 renders them unnecessary

## Origin

All paces relocated from ₣AG (jjk-post-alpha-breaking) on 260304 during ₣Ah groom session. ₣AG was retired after redistribution.

## Related Heats

- ₣Ah (jjk-v4-vision) — the V4 development heat that triggered this triage
- ₣An (jjk-v4-release-and-legacy-removal) — V4 cleanup heat

## References

- Retired trophy: `.claude/jjm/retired/jjh_b260119-r260304-jjk-post-alpha-breaking.md`
- V4 paddock: ₣Ah paddock (jjk-v4-vision)

## Paces

### consider-itches-in-gallops (₢AmAAA) [rough]

**[260304-1503] rough**

Drafted from ₢AGAAB in ₣AG.

Consider moving itches into the gallops data structure. The gallops model is working well for heats/paces, and itches would benefit from the same structured treatment (queryable, linkable to heats they spawn, managed via jjx_* commands). This is a breaking change with schema, migration, and command implications. When working on this heat, evaluate whether this problem warrants its own dedicated heat — it probably does. Note: a search feature is in the queue and may inform itch schema decisions.

### jjk-integration-test-harness (₢AmAAB) [abandoned]

**[260507-1006] abandoned**

Drafted from ₢AGAAC in ₣AG.

Drafted from ₢AFAAP in ₣AF.

Add integration test infrastructure for JJK with JJSA specification updates.

## JJSA Updates

Add to JJS0-GallopsData.adoc:

### Test Mode section (after Design Principles)

Document environment variables for test mode:
- JJTF_TEST_MESSAGE: Skip Claude invocation, use this value as commit message
- JJTF_TEST_HALLMARK: Override hallmark for tests without installed kit

### Invariants section

Document properties that MUST hold and that tests verify:
- Coronet first 2 chars = parent Firemark
- tacks[0].state = current pace state
- order keys = paces keys (bijection)
- bridled state requires non-empty direction
- Commit messages parseable by jjx_rein regex

### Commit Message Grammar

Formalize the jjb:HALLMARK:IDENTITY:ACTION: format as a proper grammar for test verification.

## Rust Implementation

### Environment variable support

In jjrn_notch.rs or new jjtf_testmode.rs:
- Check JJTF_TEST_MESSAGE env var before invoking Claude
- Check JJTF_TEST_HALLMARK env var in hallmark lookup
- Keep production path unchanged when vars not set

### Integration test harness

Create Tools/jjk/vov_veiled/tests/integration/:
- mod.rs: Test utilities (temp git repo setup, cleanup)
- lifecycle.rs: nominate → slate → bridle → wrap
- notch.rs: File list validation, warnings, firemark mode

Tests run with: cargo test --features integration-tests

## Files

- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (or new jjtf_testmode.rs)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (use test mode in wrap/notch)
- Tools/jjk/vov_veiled/tests/integration/mod.rs
- Tools/jjk/vov_veiled/tests/integration/lifecycle.rs
- Tools/jjk/vov_veiled/Cargo.toml (integration-tests feature)

### prewrap-test-fields (₢AmAAC) [abandoned]

**[260507-0921] abandoned**

Drafted from ₢AGAAD in ₣AG.

Consider adding new heat-level fields for 'prewrap tests' - commands that must succeed before a pace wrap can proceed. Candidate fields: build_command and test_command. These would enforce the pre-wrap verification discipline (build + test) at the data model level rather than relying on convention.

### wrap-completion-assessment (₢AmAAD) [abandoned]

**[260507-1006] abandoned**

Drafted from ₢AGAAH in ₣AG.

When wrapping a pace, assess whether the work created materials that downstream paces need to consume or whether their specifications need adjustment.

**Problem:** A pace's output (code changes, schema updates, documentation, data migrations) often creates artifacts that future paces in the same heat depend on. Currently, developers must manually review remaining paces and update specs. This is error-prone and easy to forget, leading to downstream paces with stale or incomplete specifications.

**Proposed workflow:**
- Define materials/artifacts produced by each pace (outputs field in tack)
- At wrap time, surface remaining paces that might consume or be affected by these materials
- Allow developer to review and update downstream pace specs before proceeding
- Link materials to dependent paces for future reference

**Design questions:**
- Where does materials tracking live? (tack metadata field? pace spec section?)
- Should this be enforced at wrap, suggested, or optional?
- How to represent dependencies between paces in same heat?
- Integration with bridle: should unresolved downstream specs prevent bridling?

**Implementation approach:**
- Option A: Extend wrap command with post-wrap assessment prompt
- Option B: Create separate 'assess' subcommand to run after wrap
- Option C: Add 'materials' and 'dependents' fields to pace spec itself

**Acceptance:**
- Developer can identify which remaining paces are affected by current pace's work
- Pace specs referencing upstream materials are discoverable
- Developer can update downstream specs and paddock before completing wrap
- Materials produced are documented for audit trail

**Related:** prewrap-test-fields (₢AGAAD) - this is the post-wrap counterpart, checking *what this pace produces* rather than *whether it's ready to ship*

### heat-build-discipline (₢AmAAE) [abandoned]

**[260507-0921] abandoned**

Drafted from ₢AGAAG in ₣AG.

Extract hardcoded Rust build/test commands from the wrap slash command and make build discipline configurable per-heat.

Current state: The wrap command (and CLAUDE.md Pre-wrap Verification section) hardcodes `tt/vow-b.Build.sh && tt/vow-t.Test.sh` as the pre-wrap verification step.

Goal: Allow heats to specify their own build discipline (or none), so non-Rust heats don't require Rust builds, and different projects can have appropriate verification.

Design decision needed: Where should build discipline live?
- Option A: Paddock structure (new field in paddock file)
- Option B: Heat fields in gallops.json (new schema field)
- Option C: Heat-level config file

Implementation:
1. Choose storage location
2. Define schema for build discipline specification
3. Update wrap command to read discipline from heat context
4. Update CLAUDE.md to reference dynamic discipline rather than hardcoded commands
5. Migrate existing heats if needed

### add-intent-field (₢AmAAF) [abandoned]

**[260507-1006] abandoned**

Drafted from ₢AGAAM in ₣AG.

Add an auto-generated intent field to pace records.

The intent statement should be a prose summary (1-2 sentences) extracted from the pace docket, maintaining it automatically as part of slate/reslate operations.

Display intent:
- In non-detailed pace listings (alongside silks)
- In wrap completion summary (showing what was just finished)

This gives paces more semantic richness than silks alone, helping runners understand purpose at a glance.

### blaze-file-registry (₢AmAAI) [abandoned]

**[260507-1006] abandoned**

Drafted from ₢AGAAQ in ₣AG.

Blaze File Registry — heat-scoped file tracking with stable ₿ tokens for rename resilience.

## NAMING CAVEAT — Resolve at pace start

"Blaze" collides with existing `buz_blazon()` in BUK zipper infrastructure. The words
are close enough to cause confusion. At pace start, decide one of:
- Rename this feature (not "blaze") — find a different equestrian/₿ term
- Rename `buz_blazon` to something else (it means "register a colophon tuple")
- Accept the collision if the domains are sufficiently separate

The ₿ sigil (Bitcoin sign, U+20BF) is chosen regardless — it's the sigil we want.
The feature name that maps to ₿ is the open question.

## Problem

Files get renamed during paces, silently rotting references in downstream pace dockets
and paddocks. A pace that says "modify jjrt_types.rs" breaks when an earlier pace renames
it. This is a recurring friction point across all heats.

## Concept

Each heat maintains a **blaze registry** (name TBD, see caveat): a table mapping stable
4-hex-digit tokens (sigil ₿, e.g. `₿A3F2`) to current filenames, with rename history.
Dockets and paddocks reference files by ₿ token. JJK expands tokens to current filenames
on read, and tokenizes filenames on write.

## Sigil and Token Format

- **Sigil**: ₿ (Bitcoin sign, U+20BF)
- **Token**: 4 hex characters, heat-scoped (65K tokens per heat)
- **Pattern**: `₿XXXX` where X is [0-9a-f]
- Fits existing sigil family: ₣ (firemark), ₢ (coronet), ₿ (file token)

## Registry Schema

New field in gallops Heat struct:

```
"blazes": {
  "a3f2": {
    "current": "Tools/jjk/vov_veiled/src/jjrt_types.rs",
    "alternates": ["Tools/jjk/vov_veiled/src/jjrg_gallops.rs"]
  }
}
```

Heat-scoped, not gallops-wide. Avoids unbounded history accumulation and simplifies
restring operations (target heat adopts relevant entries for transferred paces).

## Tokenization Flow

### Write path (paddock edits, slate, reslate)

Paddock and docket text is piped through JJK via stdin (e.g. `jjx_paddock`, `jjx_enroll`,
`jjx_revise_docket`). JJK scans the text for filename-like strings, matches against:

1. Existing registry entries (by current path or alternates)
2. Files on the current filesystem

If a match is found: replace with ₿ token in stored text.
If no match and the string looks like a filename: **error out** with message:

```
₿ error: "path/to/unknown_file.rs" not found on filesystem or in registry.
If you intended to reference a file that doesn't exist yet, create it first, then retry.
Affected heat: ₣XX
```

This forces the invoking Claude instance to create the file before referencing it,
keeping the registry grounded in real filesystem state. No auto-touch.

### Read path (mount, show, parade)

JJK expands ₿ tokens to current filenames. Three rendering cases:

| Registry state             | Rendered as                                    |
|----------------------------|------------------------------------------------|
| File exists, no rename     | `current/path.rs`                              |
| File renamed recently      | `new/path.rs` *(was: old/path.rs)*             |
| File deleted               | `(??? FILE DELETED: old/path.rs)` + warning    |

"Recently" = renamed since this pace's docket was last written.

## Git Rename Detection

At notch time, JJK already knows committed files. Additionally run:

```
git diff --find-renames --name-status <basis>..HEAD
```

`R085 old/path.rs new/path.rs` tells JJK a rename occurred with 85% similarity.

- **Above confidence threshold**: auto-update registry (old name to alternates,
  new name to current)
- **Below threshold**: error out with advisory message for the invoker to clarify

This makes rename tracking mostly automatic with no human input needed.

## Registry Lifecycle

### Initialization

- **New heats**: registry created at nomination time (empty, populated as paces
  are slated)
- **Existing heats**: `jjx_resurvey` command scans all dockets and paddock text,
  matches filename-like strings against filesystem, builds initial registry,
  replaces raw filenames with ₿ tokens

### Restring handling

When paces transfer between heats via restring:
- Target heat adopts ₿ entries referenced by transferred paces
- Token values may need remapping if collisions occur (different file, same hex)
- Source heat's registry unchanged

## Commands

| Operation | Command | Description |
|-----------|---------|-------------|
| Initialize/rebuild registry | `jjx_resurvey` | Scan dockets/paddock, build ₿ table |
| Show registry | `jjx_atlas` | Display heat's ₿ table |
| Manual rename entry | `jjx_remap` | Manually update a ₿ entry's current path |

Forward references: create the file first, then reference it. JJK errors if the
file doesn't exist — no auto-touch.

## Design Decisions Settled

- Heat-scoped registry (not gallops-wide)
- ₿ sigil with 4-hex-digit tokens
- Renames only for v1 (no file splits/merges)
- Git rename detection for automatic registry updates at notch time
- Fail-on-unknown rather than auto-touch for forward references
- Paddock/docket writes go through JJK gate (already the case)
- Basename matching sufficient given project uniqueness; full path when ambiguous
- Error out on ambiguous situations; remediable by reslate or re-paddock

## Design Questions Deferred to Implementation

- **Feature name**: resolve blaze/blazon collision (see caveat)
- **Cross-pace conflict detection**: When pace A renames a file referenced by pace B,
  when/how to surface the conflict (mount time? notch time? both?). Explore at pace
  start — expect automatic to be generally sufficient, with JJK erroring on ambiguous
  cases.
- **Rename confidence threshold**: exact git similarity cutoff for auto-update vs error
- **"Recently renamed" window**: how to define for footnote rendering

## Files (expected, names pending feature-name resolution)

- `jjrt_types.rs` — Heat struct: add registry field
- `jjri_io.rs` — load/save: serialize/deserialize registry
- New module — registry operations: tokenize, expand, resurvey, remap
- `jjrx_cli.rs` — new subcommands: `jjx_resurvey`, `jjx_atlas`, `jjx_remap`
- `jjrn_notch.rs` — git rename detection integration at notch time
- Mount/show rendering paths — token expansion with footnotes
- `JJS0-GallopsData.adoc` — spec updates for registry

## Acceptance

- Heat-level ₿ registry created at nomination
- Docket/paddock writes tokenize filenames via ₿XXXX
- Mount/show expands tokens to current filenames with rename footnotes
- Deleted files render as `(??? FILE DELETED)` warnings
- Git rename detection auto-updates registry at notch time
- `jjx_resurvey` initializes registry for existing heats
- `jjx_atlas` displays registry
- Unknown filenames error with helpful message advising file creation
- Restring transfers ₿ entries to target heat

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-05-07 10:06 - Heat - T

blaze-file-registry

### 2026-05-07 10:06 - Heat - T

wrap-completion-assessment

### 2026-05-07 10:06 - Heat - T

add-intent-field

### 2026-05-07 10:06 - Heat - T

jjk-integration-test-harness

### 2026-05-07 10:01 - Heat - T

chivvy-and-cantle-action-verbs

### 2026-05-07 09:21 - Heat - T

heat-build-discipline

### 2026-05-07 09:21 - Heat - T

prewrap-test-fields

### 2026-05-06 12:21 - Heat - D

restring 1 paces from ₣A-

### 2026-04-26 06:32 - Heat - S

jjx-paddock-honor-size-limit-param

### 2026-04-20 10:53 - Heat - S

size-guard-review-not-split

### 2026-04-18 09:32 - Heat - S

capture-session-and-officium-in-git-trailer

### 2026-04-12 09:27 - Heat - S

clarify-furlough-verb-in-claude-context

### 2026-03-31 10:50 - Heat - S

gazette-fenced-code-block-awareness

### 2026-03-04 15:04 - Heat - D

AGAAP → ₢AmAAH

### 2026-03-04 15:04 - Heat - D

AGAAO → ₢AmAAG

### 2026-03-04 15:03 - Heat - D

AGAAM → ₢AmAAF

### 2026-03-04 15:03 - Heat - D

AGAAG → ₢AmAAE

### 2026-03-04 15:03 - Heat - D

AGAAH → ₢AmAAD

### 2026-03-04 15:03 - Heat - D

AGAAD → ₢AmAAC

### 2026-03-04 15:03 - Heat - D

AGAAC → ₢AmAAB

### 2026-03-04 15:03 - Heat - D

AGAAB → ₢AmAAA

### 2026-03-04 15:03 - Heat - f

stabled

### 2026-03-04 15:02 - Heat - N

jjk-v5-notional

