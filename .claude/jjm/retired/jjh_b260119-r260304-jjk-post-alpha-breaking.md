# Heat Trophy: jjk-post-alpha-breaking

**Firemark:** ₣AG
**Created:** 260119
**Retired:** 260304
**Status:** retired

## Paddock

# Paddock: jjk-post-alpha-breaking

## Context

Breaking schema changes for JJK post-alpha cleanup. These paces intentionally break
backward compatibility with prior gallops formats and should only race after all legacy
gallops files have been migrated.

## References

- jjrt_types.rs — Gallops and Heat struct definitions
- jjri_io.rs — load/save path
- jjro_ops.rs — nominate, retire, furlough operations

## Paces

### express-pace-state (₢AGAAA) [abandoned]

**[260304-1503] abandoned**

Add 'express' pace state for heavy human intervention. Prevents Claude from incorrectly assuming autonomous solvability.

**Problem:** Claude sometimes presumes it can solve things when it can't. Current states (rough, bridled, complete, abandoned) don't signal 'needs significant human guidance.'

**Proposed state:** express (or similar name TBD)
- Signals: 'human must drive this, Claude assists'
- Opposite of bridled: bridled = autonomous, express = human-led

**Schema change:** Add to PaceState enum in jjrg_gallops.rs

**Workflow implications to muse:**
- mount: How does express affect pace selection? Skip for autonomous mount?
- bridle: Can't bridle an express pace (mutual exclusion)
- saddle: Include express paces in output? With flag?
- groom: Surface express paces prominently
- State transitions: rough → express? express → complete?

**Open questions:**
- Name: express, guided, manual, assisted?
- Can a pace transition bridled ↔ express? Or only from rough?
- Does mount skip express paces entirely, or show them with warning?

**[260119-0941] rough**

Add 'express' pace state for heavy human intervention. Prevents Claude from incorrectly assuming autonomous solvability.

**Problem:** Claude sometimes presumes it can solve things when it can't. Current states (rough, bridled, complete, abandoned) don't signal 'needs significant human guidance.'

**Proposed state:** express (or similar name TBD)
- Signals: 'human must drive this, Claude assists'
- Opposite of bridled: bridled = autonomous, express = human-led

**Schema change:** Add to PaceState enum in jjrg_gallops.rs

**Workflow implications to muse:**
- mount: How does express affect pace selection? Skip for autonomous mount?
- bridle: Can't bridle an express pace (mutual exclusion)
- saddle: Include express paces in output? With flag?
- groom: Surface express paces prominently
- State transitions: rough → express? express → complete?

**Open questions:**
- Name: express, guided, manual, assisted?
- Can a pace transition bridled ↔ express? Or only from rough?
- Does mount skip express paces entirely, or show them with warning?

### json-schema-commit-to-basis (₢AGAAF) [abandoned]

**[260304-1503] abandoned**

Change JSON schema field from "commit" to "basis" in tack records.

## Context

The Rust code already uses `basis` as the field name (with `#[serde(rename = "commit")]` for backwards compatibility). This pace completes the rename by updating the JSON schema itself.

## Changes

1. Remove `#[serde(rename = "commit")]` from `jjrt_types.rs` — field serializes as `"basis"`
2. Migrate existing gallops JSON files: `s/"commit":/"basis":/g`
3. Update any documentation referencing the JSON schema

## Breaking

This changes the JSON schema. All existing gallops files must be migrated.

## Acceptance

- JSON files use `"basis"` key instead of `"commit"`
- Round-trip validation passes
- All tests pass

**[260127-1133] rough**

Change JSON schema field from "commit" to "basis" in tack records.

## Context

The Rust code already uses `basis` as the field name (with `#[serde(rename = "commit")]` for backwards compatibility). This pace completes the rename by updating the JSON schema itself.

## Changes

1. Remove `#[serde(rename = "commit")]` from `jjrt_types.rs` — field serializes as `"basis"`
2. Migrate existing gallops JSON files: `s/"commit":/"basis":/g`
3. Update any documentation referencing the JSON schema

## Breaking

This changes the JSON schema. All existing gallops files must be migrated.

## Acceptance

- JSON files use `"basis"` key instead of `"commit"`
- Round-trip validation passes
- All tests pass

### json-schema-text-to-docket (₢AGAAK) [abandoned]

**[260304-1503] abandoned**

Rename JSON schema field "text" to "docket" in tack records.

Companion to json-schema-commit-to-basis (₢AGAAF). Same pattern:
1. Remove serde rename attribute (or change field name) in jjrt_types.rs
2. Migrate existing gallops JSON: s/"text":/"docket":/g
3. Update documentation referencing the JSON schema

Breaking: Changes JSON schema. All existing gallops files must be migrated.

Acceptance: JSON files use "docket" key. Round-trip validation passes. All tests pass.

**[260206-2234] rough**

Rename JSON schema field "text" to "docket" in tack records.

Companion to json-schema-commit-to-basis (₢AGAAF). Same pattern:
1. Remove serde rename attribute (or change field name) in jjrt_types.rs
2. Migrate existing gallops JSON: s/"text":/"docket":/g
3. Update documentation referencing the JSON schema

Breaking: Changes JSON schema. All existing gallops files must be migrated.

Acceptance: JSON files use "docket" key. Round-trip validation passes. All tests pass.

### json-schema-direction-to-warrant (₢AGAAL) [abandoned]

**[260304-1503] abandoned**

Rename JSON schema field "direction" to "warrant" in tack records.

Companion to json-schema-text-to-docket (₢AGAAK). Same pattern:
1. Remove serde rename attribute (or change field name) in jjrt_types.rs
2. Migrate existing gallops JSON: s/"direction":/"warrant":/g
3. Update documentation referencing the JSON schema

Breaking: Changes JSON schema. All existing gallops files must be migrated.

Acceptance: JSON files use "warrant" key. Round-trip validation passes. All tests pass.

**[260206-2234] rough**

Rename JSON schema field "direction" to "warrant" in tack records.

Companion to json-schema-text-to-docket (₢AGAAK). Same pattern:
1. Remove serde rename attribute (or change field name) in jjrt_types.rs
2. Migrate existing gallops JSON: s/"direction":/"warrant":/g
3. Update documentation referencing the JSON schema

Breaking: Changes JSON schema. All existing gallops files must be migrated.

Acceptance: JSON files use "warrant" key. Round-trip validation passes. All tests pass.

### consider-heat-base-commit (₢AGAAI) [abandoned]

**[260206-0931] abandoned**

Superseded by ₢AHAAU pace-commit-timeline — commit-indexed bitmap provides the temporal view that base_commit was meant to enable.

**[260206-0854] rough**

Consider adding a 'base commit' field to the heat data model — a git SHA recorded at heat creation time that captures the codebase state when the heat was constructed.

## Problem

There is currently no way to programmatically determine what the codebase looked like "before" a heat's changes began. This matters for:
- Diffing: `git diff <base_commit>..HEAD` to see all changes a heat has made
- Rein/steeplechase: Understanding cumulative impact of a heat's paces
- Rollback analysis: Knowing what "clean" looked like before heat work started
- Heat retirement: Summarizing total delta introduced by the heat

## Proposed Schema Change

Add `base_commit` (optional string, git SHA) to Heat record in gallops.json:

```json
{
  "silks": "my-heat",
  "status": "racing",
  "base_commit": "a1b2c3d4e5f6...",  // NEW: SHA at heat creation
  ...
}
```

## Behavior

- `jjx_nominate`: Capture `git rev-parse HEAD` at heat creation, store as `base_commit`
- `jjx_parade`: Display base commit when present
- `jjx_rein`: Could use base commit as diff anchor for steeplechase summaries
- Migration: Existing heats get `base_commit: null` (not retroactively derivable with certainty)

## Design Questions

- Should this be mandatory for new heats or optional?
- Is a single SHA sufficient, or should we record branch name too?
- Should this interact with furlough (re-baseline when resuming)?
- Relationship to tack basis fields: tack records already track per-pace commit SHAs — how does heat-level base commit complement or overlap with per-pace basis?

## Acceptance

- Design decision documented on whether to proceed and with what schema
- If proceeding: schema change specified, migration path defined

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-03-04 15:03 - Heat - T

json-schema-direction-to-warrant

### 2026-03-04 15:03 - Heat - T

json-schema-text-to-docket

### 2026-03-04 15:03 - Heat - T

json-schema-commit-to-basis

### 2026-03-04 15:03 - Heat - T

express-pace-state

### 2026-02-22 10:41 - Heat - T

blaze-file-registry

### 2026-02-22 09:44 - Heat - T

blaze-file-registry

### 2026-02-22 09:44 - Heat - T

file-token-registry

### 2026-02-22 08:55 - Heat - S

file-token-registry

### 2026-02-21 13:23 - Heat - T

harden-paddock-encoding

### 2026-02-21 13:01 - Heat - S

harden-paddock-encoding

### 2026-02-10 11:24 - Heat - S

pace-snip-operation

### 2026-02-08 15:45 - Heat - S

permit-uppercase-silks

### 2026-02-08 13:18 - Heat - S

add-intent-field

### 2026-02-06 22:34 - Heat - S

json-schema-direction-to-warrant

### 2026-02-06 22:34 - Heat - S

json-schema-text-to-docket

### 2026-02-06 09:31 - Heat - T

consider-heat-base-commit

### 2026-02-06 09:02 - Heat - T

heat-file-touch-visualization

### 2026-02-06 08:54 - Heat - S

heat-file-touch-visualization

### 2026-02-06 08:54 - Heat - S

consider-heat-base-commit

### 2026-01-30 08:10 - Heat - r

moved AGAAH before AGAAE

### 2026-01-30 08:10 - Heat - S

wrap-completion-assessment

### 2026-01-28 06:38 - Heat - S

heat-build-discipline

### 2026-01-27 11:33 - Heat - S

json-schema-commit-to-basis

### 2026-01-24 10:31 - Heat - S

add-heat-order-vector

### 2026-01-24 07:31 - Heat - S

prewrap-test-fields

### 2026-01-23 17:15 - Heat - d

Restring: 1 pace from ₣AF (parked for prioritization)

### 2026-01-23 17:13 - Heat - D

AFAAP → ₢AGAAC

