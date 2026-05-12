# Heat Trophy: rbk-12-mvp-regime-scrub

**Firemark:** ₣A7
**Created:** 260409
**Retired:** 260512
**Status:** retired

## Paddock

# Paddock: rbk-mvp-3-regime-scrub

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### buv-render-add-file-path (₢A7AAB) [complete]

**[260512-1344] complete**

## Character

Mechanical multi-file edit landing concern #1 from triage ₢A7AAA. No new design choices — all the design conversation lives in ₢A7AAA's discussion record. Intricate but mechanical.

## Decisions

- `buv_render` signature grows optional 3rd param: `SCOPE LABEL [FILE_PATH]`
- Display: `  File: <path>` in gray (BUC_gray), indented two spaces under the existing bold-white title line; empty/omitted FILE_PATH skips the line
- Path passed exactly as-is — no realpath, no relative conversion, no prefix stripping
- Manifolds (folio-scoped: BURN, BURP, RBRN, RBRV, RBRA) recompute the path inside the render command, mirroring the derivation already in their furnish block. Do NOT promote the furnish-local to a global — would conflict with scope-sentinel rejection and BCG kindle-constant rules
- Out of scope: RBRO inline render (does its own secret-masking, bypasses `buv_render`; queue separately if desired); ₢A7AAA concerns #2–#8 (still in triage)

## Sites

Discover: `grep -n "buv_render" Tools/{buk,rbk}/*_cli.sh`

Eleven sites total:
- 5 singletons (BURC, BURS, RBRR, RBRP, RBRS) — pass the existing global from each furnish's source line as 3rd arg
- 1 ambient (BURE) — pass `""`
- 5 manifolds (BURN, BURP, RBRN, RBRV, RBRA) — recompute path locally in render command mirroring their furnish derivation

Plus `Tools/buk/buv_validation.sh:792` (`buv_render` signature + body) and BUS0 spec.

## Done when

All 11 render tabtargets show the `File:` line under their title (BURE shows none); `tt/buw-st.BukSelfTest.sh` green; BUS0 `buv_render` contract reflects new signature.

**[260512-1340] complete**

## Character

Mechanical multi-file edit landing concern #1 from triage ₢A7AAA. No new design choices — all the design conversation lives in ₢A7AAA's discussion record. Intricate but mechanical.

## Decisions

- `buv_render` signature grows optional 3rd param: `SCOPE LABEL [FILE_PATH]`
- Display: `  File: <path>` in gray (BUC_gray), indented two spaces under the existing bold-white title line; empty/omitted FILE_PATH skips the line
- Path passed exactly as-is — no realpath, no relative conversion, no prefix stripping
- Manifolds (folio-scoped: BURN, BURP, RBRN, RBRV, RBRA) recompute the path inside the render command, mirroring the derivation already in their furnish block. Do NOT promote the furnish-local to a global — would conflict with scope-sentinel rejection and BCG kindle-constant rules
- Out of scope: RBRO inline render (does its own secret-masking, bypasses `buv_render`; queue separately if desired); ₢A7AAA concerns #2–#8 (still in triage)

## Sites

Discover: `grep -n "buv_render" Tools/{buk,rbk}/*_cli.sh`

Eleven sites total:
- 5 singletons (BURC, BURS, RBRR, RBRP, RBRS) — pass the existing global from each furnish's source line as 3rd arg
- 1 ambient (BURE) — pass `""`
- 5 manifolds (BURN, BURP, RBRN, RBRV, RBRA) — recompute path locally in render command mirroring their furnish derivation

Plus `Tools/buk/buv_validation.sh:792` (`buv_render` signature + body) and BUS0 spec.

## Done when

All 11 render tabtargets show the `File:` line under their title (BURE shows none); `tt/buw-st.BukSelfTest.sh` green; BUS0 `buv_render` contract reflects new signature.

**[260511-1113] rough**

## Character

Mechanical multi-file edit landing concern #1 from triage ₢A7AAA. No new design choices — all the design conversation lives in ₢A7AAA's discussion record. Intricate but mechanical.

## Decisions

- `buv_render` signature grows optional 3rd param: `SCOPE LABEL [FILE_PATH]`
- Display: `  File: <path>` in gray (BUC_gray), indented two spaces under the existing bold-white title line; empty/omitted FILE_PATH skips the line
- Path passed exactly as-is — no realpath, no relative conversion, no prefix stripping
- Manifolds (folio-scoped: BURN, BURP, RBRN, RBRV, RBRA) recompute the path inside the render command, mirroring the derivation already in their furnish block. Do NOT promote the furnish-local to a global — would conflict with scope-sentinel rejection and BCG kindle-constant rules
- Out of scope: RBRO inline render (does its own secret-masking, bypasses `buv_render`; queue separately if desired); ₢A7AAA concerns #2–#8 (still in triage)

## Sites

Discover: `grep -n "buv_render" Tools/{buk,rbk}/*_cli.sh`

Eleven sites total:
- 5 singletons (BURC, BURS, RBRR, RBRP, RBRS) — pass the existing global from each furnish's source line as 3rd arg
- 1 ambient (BURE) — pass `""`
- 5 manifolds (BURN, BURP, RBRN, RBRV, RBRA) — recompute path locally in render command mirroring their furnish derivation

Plus `Tools/buk/buv_validation.sh:792` (`buv_render` signature + body) and BUS0 spec.

## Done when

All 11 render tabtargets show the `File:` line under their title (BURE shows none); `tt/buw-st.BukSelfTest.sh` green; BUS0 `buv_render` contract reflects new signature.

### burc-render-review-triage (₢A7AAA) [complete]

**[260511-1157] complete**

## Character

Discussion pace. Each bullet below is a concern raised during a `tt/buw-rcr.RenderConfigRegime.sh` review session. **Disposition for each must be discussed individually before any editing happens.** Do not batch-implement. When this pace is mounted, walk the list with the user one concern at a time, confirm direction, then either split into implementation paces or notch inline as decisions land.

## Context

Triggered `tt/buw-rcr.RenderConfigRegime.sh` against the project's `.buk/burc.env`. Output rendered 11 enrolled BURC variables across groups: Station Reference, Tabtarget Infrastructure, Project Structure, Build Output, Logging. User reviewed the rendered output and raised 8 concerns. **Concern #1 (file-path line under title) was triaged and slated as ₢A7AAB (`buv-render-add-file-path`), now first in heat queue.** 7 remain in discussion below. Original numbering #2–#8 preserved for traceability with ₢A7AAB's docket reference.

Key files touched by any future implementation:
- `Tools/buk/burc_regime.sh` — enrollment definitions (lines 40-64)
- `Tools/buk/buv_validation.sh` — `buv_render()` at line 778
- `Tools/buk/burc_cli.sh:41` — call site for `buv_render`
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — BURC spec (lines 672-697)
- `.buk/burc.env` — assignment file (content changes only)

## The seven remaining concerns

### 2. BURC_TABTARGET_DIR wording

Current: `"Directory containing launcher scripts"`
Requested: `"Project dir containing launcher scripts"`

Literal swap at `burc_regime.sh:46`. Mirror to BUS0.

**Discuss:** Confirm "Project dir" phrasing; confirm BUS0 mirror scope.

### 3. BURC_TOOLS_DIR wording

Current: `"Directory containing tool scripts"`
Requested: `"Project dir containing tool scripts"`

Literal swap at `burc_regime.sh:50`. Mirror to BUS0.

**Discuss:** Same as #2 — confirm phrasing and mirror.

### 4. BURC_PROJECT_ROOT confusingly defined

Current: `"Path from burc.env to project root"`

Confusing because it describes the direction rather than what the variable IS, and "project root" is ambiguous with other "root" nouns in BUK.

**Claude's preliminary proposal:**
> `"Repo root, expressed as a relative path from the BURC assignment file's directory (typically '..')"`

**Discuss:** Does "repo root" vs "project root" matter? Is the example value helpful or clutter?

### 5. BURC_MANAGED_KITS: remove vvx reference

Current description: `"Comma-separated kit list for vvx"`
Current value: `buk,cmk,jjk,vvk`
User wants: value reduced to `rbk,buk`, description reworded to not reference the vvx binary.

**Claude's preliminary proposals (pick one or draft new):**
- A. `"Kits whose source this project owns and maintains"`
- B. `"Project-owned kits whose Tools/ subdirectories are the managed surface"`
- C. `"Kit inventory declared by this project"`

**Discuss — TWO sub-decisions:**
- Description wording (A / B / C / other)
- Content change from `buk,cmk,jjk,vvk` → `rbk,buk`: implications for VOF (`vof_features.sh`, `vob_build.sh`) and vvx release/install flows since they consume this variable. This project may or may not be a VOK forge context; need to confirm that reducing the list won't break vvx build discipline.

### 6. BURC_TEMP_ROOT_DIR underspecified

Current: `"Root directory for temporary files"`
BUS0 (line 681-682) is equally thin: `"Root directory for BUK temporary files"`.

Actual semantics from `bud_dispatch.sh:100`: each dispatch creates `temp-${BURD_NOW_STAMP}` under this root.

**Claude's preliminary proposal:**
> `"Parent directory under which each dispatch creates a per-invocation scratch subdir (temp-<stamp>). Used for ephemeral files produced during a single tabtarget run; not cleaned automatically."`

**Discuss:** Level of detail for render tooltip vs BUS0 spec. Should render summary be short (one line) and defer to BUS0 for full semantics? Cleanup behavior worth calling out?

### 7. BURC_OUTPUT_ROOT_DIR underspecified

Current: `"Root directory for command output"`

Actual semantics from `bud_dispatch.sh:112`: `BURD_OUTPUT_DIR="${BURC_OUTPUT_ROOT_DIR}/current"` — fixed `current/` subdir cleared and recreated per dispatch. Remote callers (jjx_fetch via `jjrlg_legatio.rs:336`) depend on this path.

**Claude's preliminary proposal:**
> `"Parent directory containing 'current/', the fixed output subdirectory cleared and recreated on each dispatch. Holds facts and artifacts from the most recent invocation only; remote callers (e.g. jjx_fetch) read from here."`

**Discuss:** Same detail-level question as #6. Worth mentioning the `current/` convention explicitly? Worth naming the remote fetch dependency?

### 8. BURC_BUK_DIR "Derived: BUK directory" is confusing

Yes, it IS actually derived — `burc_regime.sh:36`:

```bash
readonly BURC_BUK_DIR="${BURC_TOOLS_DIR:-}/buk"
```

Set at kindle time before `buv_vet`, not user-configurable. Two sub-issues:

**8a. Description doesn't say from what.**
Proposal: `"BUK kit directory, computed as \${BURC_TOOLS_DIR}/buk at kindle time (not user-set)"`

**8b. Structural: derived variables are mixed in with user-set ones.**
Currently BURC_BUK_DIR sits in the "Logging" group which is semantically wrong. Options:
- Move to a new "Derived" group at the bottom of the enrollment.
- Add a visual marker in the render output to distinguish derived from user-configurable.
- Both.

**Discuss:** Scope — is this a burc_regime.sh enrollment change only, or does it touch the render output format (new visual marker in `buv_render`)? Should other regimes (BURD, BURV, BURS) also grow a "Derived" group convention?

## Disposition protocol

When mounting this pace:

1. Read this docket top-to-bottom with the user.
2. For each remaining concern (#2 through #8):
   - Restate the concern.
   - Present the preliminary proposal(s).
   - Ask user for disposition: (a) accept as-is, (b) revise wording/approach, (c) split into implementation pace, (d) defer to itch, (e) drop.
3. Do NOT edit any files during the discussion phase.
4. Once all 7 remaining have landed dispositions, decide whether the surviving items become:
   - A single omnibus implementation pace (if small wording-only edits).
   - Multiple paces grouped by concern (wording sweep / structural).
   - A mix with some items moved to itch.
5. Wrap this discussion pace, then slate any implementation pace(s) not already slated.

## Not in scope for THIS pace

- Implementation of any edit. This is discussion-only.
- Sweeping other BURx regimes (BURS, BURD, BURV) unless a concern explicitly generalizes during discussion.
- Changing `.buk/burc.env` content (concern #5's value change) without first confirming VOK/vvx implications.
- The file-path display concern (#1) — owned by ₢A7AAB.

**[260511-1122] rough**

## Character

Discussion pace. Each bullet below is a concern raised during a `tt/buw-rcr.RenderConfigRegime.sh` review session. **Disposition for each must be discussed individually before any editing happens.** Do not batch-implement. When this pace is mounted, walk the list with the user one concern at a time, confirm direction, then either split into implementation paces or notch inline as decisions land.

## Context

Triggered `tt/buw-rcr.RenderConfigRegime.sh` against the project's `.buk/burc.env`. Output rendered 11 enrolled BURC variables across groups: Station Reference, Tabtarget Infrastructure, Project Structure, Build Output, Logging. User reviewed the rendered output and raised 8 concerns. **Concern #1 (file-path line under title) was triaged and slated as ₢A7AAB (`buv-render-add-file-path`), now first in heat queue.** 7 remain in discussion below. Original numbering #2–#8 preserved for traceability with ₢A7AAB's docket reference.

Key files touched by any future implementation:
- `Tools/buk/burc_regime.sh` — enrollment definitions (lines 40-64)
- `Tools/buk/buv_validation.sh` — `buv_render()` at line 778
- `Tools/buk/burc_cli.sh:41` — call site for `buv_render`
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — BURC spec (lines 672-697)
- `.buk/burc.env` — assignment file (content changes only)

## The seven remaining concerns

### 2. BURC_TABTARGET_DIR wording

Current: `"Directory containing launcher scripts"`
Requested: `"Project dir containing launcher scripts"`

Literal swap at `burc_regime.sh:46`. Mirror to BUS0.

**Discuss:** Confirm "Project dir" phrasing; confirm BUS0 mirror scope.

### 3. BURC_TOOLS_DIR wording

Current: `"Directory containing tool scripts"`
Requested: `"Project dir containing tool scripts"`

Literal swap at `burc_regime.sh:50`. Mirror to BUS0.

**Discuss:** Same as #2 — confirm phrasing and mirror.

### 4. BURC_PROJECT_ROOT confusingly defined

Current: `"Path from burc.env to project root"`

Confusing because it describes the direction rather than what the variable IS, and "project root" is ambiguous with other "root" nouns in BUK.

**Claude's preliminary proposal:**
> `"Repo root, expressed as a relative path from the BURC assignment file's directory (typically '..')"`

**Discuss:** Does "repo root" vs "project root" matter? Is the example value helpful or clutter?

### 5. BURC_MANAGED_KITS: remove vvx reference

Current description: `"Comma-separated kit list for vvx"`
Current value: `buk,cmk,jjk,vvk`
User wants: value reduced to `rbk,buk`, description reworded to not reference the vvx binary.

**Claude's preliminary proposals (pick one or draft new):**
- A. `"Kits whose source this project owns and maintains"`
- B. `"Project-owned kits whose Tools/ subdirectories are the managed surface"`
- C. `"Kit inventory declared by this project"`

**Discuss — TWO sub-decisions:**
- Description wording (A / B / C / other)
- Content change from `buk,cmk,jjk,vvk` → `rbk,buk`: implications for VOF (`vof_features.sh`, `vob_build.sh`) and vvx release/install flows since they consume this variable. This project may or may not be a VOK forge context; need to confirm that reducing the list won't break vvx build discipline.

### 6. BURC_TEMP_ROOT_DIR underspecified

Current: `"Root directory for temporary files"`
BUS0 (line 681-682) is equally thin: `"Root directory for BUK temporary files"`.

Actual semantics from `bud_dispatch.sh:100`: each dispatch creates `temp-${BURD_NOW_STAMP}` under this root.

**Claude's preliminary proposal:**
> `"Parent directory under which each dispatch creates a per-invocation scratch subdir (temp-<stamp>). Used for ephemeral files produced during a single tabtarget run; not cleaned automatically."`

**Discuss:** Level of detail for render tooltip vs BUS0 spec. Should render summary be short (one line) and defer to BUS0 for full semantics? Cleanup behavior worth calling out?

### 7. BURC_OUTPUT_ROOT_DIR underspecified

Current: `"Root directory for command output"`

Actual semantics from `bud_dispatch.sh:112`: `BURD_OUTPUT_DIR="${BURC_OUTPUT_ROOT_DIR}/current"` — fixed `current/` subdir cleared and recreated per dispatch. Remote callers (jjx_fetch via `jjrlg_legatio.rs:336`) depend on this path.

**Claude's preliminary proposal:**
> `"Parent directory containing 'current/', the fixed output subdirectory cleared and recreated on each dispatch. Holds facts and artifacts from the most recent invocation only; remote callers (e.g. jjx_fetch) read from here."`

**Discuss:** Same detail-level question as #6. Worth mentioning the `current/` convention explicitly? Worth naming the remote fetch dependency?

### 8. BURC_BUK_DIR "Derived: BUK directory" is confusing

Yes, it IS actually derived — `burc_regime.sh:36`:

```bash
readonly BURC_BUK_DIR="${BURC_TOOLS_DIR:-}/buk"
```

Set at kindle time before `buv_vet`, not user-configurable. Two sub-issues:

**8a. Description doesn't say from what.**
Proposal: `"BUK kit directory, computed as \${BURC_TOOLS_DIR}/buk at kindle time (not user-set)"`

**8b. Structural: derived variables are mixed in with user-set ones.**
Currently BURC_BUK_DIR sits in the "Logging" group which is semantically wrong. Options:
- Move to a new "Derived" group at the bottom of the enrollment.
- Add a visual marker in the render output to distinguish derived from user-configurable.
- Both.

**Discuss:** Scope — is this a burc_regime.sh enrollment change only, or does it touch the render output format (new visual marker in `buv_render`)? Should other regimes (BURD, BURV, BURS) also grow a "Derived" group convention?

## Disposition protocol

When mounting this pace:

1. Read this docket top-to-bottom with the user.
2. For each remaining concern (#2 through #8):
   - Restate the concern.
   - Present the preliminary proposal(s).
   - Ask user for disposition: (a) accept as-is, (b) revise wording/approach, (c) split into implementation pace, (d) defer to itch, (e) drop.
3. Do NOT edit any files during the discussion phase.
4. Once all 7 remaining have landed dispositions, decide whether the surviving items become:
   - A single omnibus implementation pace (if small wording-only edits).
   - Multiple paces grouped by concern (wording sweep / structural).
   - A mix with some items moved to itch.
5. Wrap this discussion pace, then slate any implementation pace(s) not already slated.

## Not in scope for THIS pace

- Implementation of any edit. This is discussion-only.
- Sweeping other BURx regimes (BURS, BURD, BURV) unless a concern explicitly generalizes during discussion.
- Changing `.buk/burc.env` content (concern #5's value change) without first confirming VOK/vvx implications.
- The file-path display concern (#1) — owned by ₢A7AAB.

**[260409-1432] rough**

## Character

Discussion pace. Each bullet below is a concern raised during a `tt/buw-rcr.RenderConfigRegime.sh` review session. **Disposition for each must be discussed individually before any editing happens.** Do not batch-implement. When this pace is mounted, walk the list with the user one concern at a time, confirm direction, then either split into implementation paces or notch inline as decisions land.

## Context

Triggered `tt/buw-rcr.RenderConfigRegime.sh` against the project's `.buk/burc.env`. Output rendered 11 enrolled BURC variables across groups: Station Reference, Tabtarget Infrastructure, Project Structure, Build Output, Logging. User reviewed the rendered output and raised 8 concerns. Claude drafted preliminary proposals for each but no implementation has been done.

Key files touched by any future implementation:
- `Tools/buk/burc_regime.sh` — enrollment definitions (lines 40-64)
- `Tools/buk/buv_validation.sh` — `buv_render()` at line 778
- `Tools/buk/burc_cli.sh:41` — call site for `buv_render`
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — BURC spec (lines 672-697)
- `.buk/burc.env` — assignment file (content changes only)

## The eight concerns

### 1. File path line after title

Render output should show the project-relative path of the BURC assignment file as a second line under the title. Example:

```
BURC - Bash Utility Configuration Regime
  File in your repo at .buk/burc.env
```

**Claude's preliminary proposal:** Extend `buv_render()` signature to `buv_render SCOPE LABEL FILE_PATH`, update the `burc_cli.sh:41` call site to pass a project-relative form of `BURD_REGIME_FILE` (either passed in or derived by stripping `BURC_PROJECT_ROOT`).

**Discuss:** How to compute project-relative — new param vs derive from env vars? Should this generalize to all `buv_render` callers (not just BURC)?

### 2. BURC_TABTARGET_DIR wording

Current: `"Directory containing launcher scripts"`
Requested: `"Project dir containing launcher scripts"`

Literal swap at `burc_regime.sh:46`. Mirror to BUS0.

**Discuss:** Confirm "Project dir" phrasing; confirm BUS0 mirror scope.

### 3. BURC_TOOLS_DIR wording

Current: `"Directory containing tool scripts"`
Requested: `"Project dir containing tool scripts"`

Literal swap at `burc_regime.sh:50`. Mirror to BUS0.

**Discuss:** Same as #2 — confirm phrasing and mirror.

### 4. BURC_PROJECT_ROOT confusingly defined

Current: `"Path from burc.env to project root"`

Confusing because it describes the direction rather than what the variable IS, and "project root" is ambiguous with other "root" nouns in BUK.

**Claude's preliminary proposal:**
> `"Repo root, expressed as a relative path from the BURC assignment file's directory (typically '..')"`

**Discuss:** Does "repo root" vs "project root" matter? Is the example value helpful or clutter?

### 5. BURC_MANAGED_KITS: remove vvx reference

Current description: `"Comma-separated kit list for vvx"`
Current value: `buk,cmk,jjk,vvk`
User wants: value reduced to `rbk,buk`, description reworded to not reference the vvx binary.

**Claude's preliminary proposals (pick one or draft new):**
- A. `"Kits whose source this project owns and maintains"`
- B. `"Project-owned kits whose Tools/ subdirectories are the managed surface"`
- C. `"Kit inventory declared by this project"`

**Discuss — TWO sub-decisions:**
- Description wording (A / B / C / other)
- Content change from `buk,cmk,jjk,vvk` → `rbk,buk`: implications for VOF (`vof_features.sh`, `vob_build.sh`) and vvx release/install flows since they consume this variable. This project may or may not be a VOK forge context; need to confirm that reducing the list won't break vvx build discipline.

### 6. BURC_TEMP_ROOT_DIR underspecified

Current: `"Root directory for temporary files"`
BUS0 (line 681-682) is equally thin: `"Root directory for BUK temporary files"`.

Actual semantics from `bud_dispatch.sh:100`: each dispatch creates `temp-${BURD_NOW_STAMP}` under this root.

**Claude's preliminary proposal:**
> `"Parent directory under which each dispatch creates a per-invocation scratch subdir (temp-<stamp>). Used for ephemeral files produced during a single tabtarget run; not cleaned automatically."`

**Discuss:** Level of detail for render tooltip vs BUS0 spec. Should render summary be short (one line) and defer to BUS0 for full semantics? Cleanup behavior worth calling out?

### 7. BURC_OUTPUT_ROOT_DIR underspecified

Current: `"Root directory for command output"`

Actual semantics from `bud_dispatch.sh:112`: `BURD_OUTPUT_DIR="${BURC_OUTPUT_ROOT_DIR}/current"` — fixed `current/` subdir cleared and recreated per dispatch. Remote callers (jjx_fetch via `jjrlg_legatio.rs:336`) depend on this path.

**Claude's preliminary proposal:**
> `"Parent directory containing 'current/', the fixed output subdirectory cleared and recreated on each dispatch. Holds facts and artifacts from the most recent invocation only; remote callers (e.g. jjx_fetch) read from here."`

**Discuss:** Same detail-level question as #6. Worth mentioning the `current/` convention explicitly? Worth naming the remote fetch dependency?

### 8. BURC_BUK_DIR "Derived: BUK directory" is confusing

Yes, it IS actually derived — `burc_regime.sh:36`:

```bash
readonly BURC_BUK_DIR="${BURC_TOOLS_DIR:-}/buk"
```

Set at kindle time before `buv_vet`, not user-configurable. Two sub-issues:

**8a. Description doesn't say from what.**
Proposal: `"BUK kit directory, computed as \${BURC_TOOLS_DIR}/buk at kindle time (not user-set)"`

**8b. Structural: derived variables are mixed in with user-set ones.**
Currently BURC_BUK_DIR sits in the "Logging" group which is semantically wrong. Options:
- Move to a new "Derived" group at the bottom of the enrollment.
- Add a visual marker in the render output to distinguish derived from user-configurable.
- Both.

**Discuss:** Scope — is this a burc_regime.sh enrollment change only, or does it touch the render output format (new visual marker in `buv_render`)? Should other regimes (BURD, BURV, BURS) also grow a "Derived" group convention?

## Disposition protocol

When mounting this pace:

1. Read this docket top-to-bottom with the user.
2. For each concern (1 through 8):
   - Restate the concern.
   - Present the preliminary proposal(s).
   - Ask user for disposition: (a) accept as-is, (b) revise wording/approach, (c) split into implementation pace, (d) defer to itch, (e) drop.
3. Do NOT edit any files during the discussion phase.
4. Once all 8 have landed dispositions, decide whether the surviving items become:
   - A single omnibus implementation pace (if small wording-only edits).
   - Multiple paces grouped by concern (wording sweep / render infra / structural).
   - A mix with some items moved to itch.
5. Wrap this discussion pace, then slate the implementation pace(s).

## Not in scope for THIS pace

- Implementation of any edit. This is discussion-only.
- Sweeping other BURx regimes (BURS, BURD, BURV) unless a concern explicitly generalizes during discussion.
- Changing `.buk/burc.env` content (concern #5's value change) without first confirming VOK/vvx implications.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 B buv-render-add-file-path
  2 A burc-render-review-triage

BA
xx BUS0-BashUtilitiesSpec.adoc
·x burc_regime.sh
x· burc_cli.sh, bure_cli.sh, burn_cli.sh, burp_cli.sh, burs_cli.sh, buv_validation.sh, rbra_cli.sh, rbrn_cli.sh, rbrp_cli.sh, rbrr_cli.sh, rbrs_cli.sh, rbrv_cli.sh

Commit swim lanes (x = commit affiliated with pace):

  1 A burc-render-review-triage
  2 B buv-render-add-file-path

123456789abcd
····xxxxx····  A  5c
··········xxx  B  3c
```

## Steeplechase

### 2026-05-12 13:44 - ₢A7AAB - W

Extended buv_render with optional FILE_PATH 3rd arg; emits indented gray 'File: <path>' line under bold-white title when non-empty. Wired all 11 render sites: 5 singletons pass each furnish's source-file global, BURE passes empty string, 5 manifolds recompute path locally mirroring furnish derivation. BUS0 bukro_render quoin description refreshed to mention source-file provenance. BukSelfTest green (5 fixtures, 28 cases); spot-checks confirmed singleton/ambient/manifold scopes all behave correctly.

### 2026-05-12 13:40 - ₢A7AAB - W

Extended buv_render signature with optional FILE_PATH 3rd arg; emits indented gray 'File: <path>' line under the bold-white title when non-empty. Wired all 11 render sites: 5 singletons pass each furnish's source-file global, BURE passes empty string, 5 manifolds recompute path locally mirroring furnish derivation. BUS0 bukro_render quoin description refreshed to mention source-file provenance. BukSelfTest green (5 fixtures, 28 cases); spot-checks confirmed singleton (BURC), ambient (BURE no line), manifold (RBRN tadmor) all behave correctly.

### 2026-05-12 13:40 - ₢A7AAB - n

Extend buv_render with optional FILE_PATH 3rd arg; emit indented 'File: <path>' line in BUC_gray under the bold-white title when non-empty (empty/omitted skips line). Wire all 11 render sites: 5 singletons (BURC, BURS, RBRR, RBRP, RBRS) pass each furnish's existing source-file global as 3rd arg; 1 ambient (BURE) passes explicit empty string; 5 manifolds (BURN, BURP, RBRN, RBRV, RBRA) recompute path locally in their render command mirroring the furnish derivation (do NOT promote furnish-local to global — would conflict with scope-sentinel rejection and BCG kindle-constant rules). BUS0 bukro_render quoin description refreshed to mention source-file provenance — closest spec location to a 'buv_render contract'. Gates: tt/buw-st.BukSelfTest.sh green (5 fixtures, 28 cases). Spot-checks confirmed all three scope categories: singleton (buw-rcr displays absolute .buk/burc.env path), ambient (buw-rer omits File line), manifold (rbw-rnr tadmor displays .rbk/tadmor/rbrn.env). Out of scope: RBRO inline render (does own secret-masking, bypasses buv_render); concerns #2-#8 from triage A7AAA.

### 2026-05-11 15:40 - Heat - f

silks=rbk-12-mvp-regime-scrub

### 2026-05-11 11:57 - ₢A7AAA - W

Triage of 8 BURC render concerns surfaced by tt/buw-rcr.RenderConfigRegime.sh review. Concern #1 (file-path line under title) — design conversation landed an optional 3rd-param approach for buv_render; slated as implementation pace ₢A7AAB. Concerns #2 (BURC_TABTARGET_DIR), #3 (BURC_TOOLS_DIR), #4 (BURC_PROJECT_ROOT), #6 (BURC_TEMP_ROOT_DIR), #7 (BURC_OUTPUT_ROOT_DIR) — description rewordings implemented inline in both burc_regime.sh and BUS0 spec (commits 670eb9ba, 2b03954, 35167ca, a2fdab9). Concern #5 (BURC_MANAGED_KITS) — dropped after study showed the wording change would obfuscate accurate vvx-specific semantics and the proposed value change buk,cmk,jjk,vvk → rbk,buk would break vvx Cargo-feature compilation plus emplace parcel exact-match validation. Concern #8 (BURC_BUK_DIR derived-var-mixed-with-user-set-vars structural question) — dropped.

### 2026-05-11 11:54 - ₢A7AAA - n

Mirror BURC description rewordings from burc_regime.sh into BUS0 spec to keep spec aligned with implementation. Covers concerns #2 (TABTARGET_DIR), #3 (TOOLS_DIR), #4 (PROJECT_ROOT), #6 (TEMP_ROOT_DIR), #7 (OUTPUT_ROOT_DIR) from triage ₢A7AAA. BUS0 wording slightly more formal than regime descriptions (uses full 'directory' rather than 'dir', and BUS0's linked-term references preserved where present)

### 2026-05-11 11:48 - ₢A7AAA - n

Expand BURC_TEMP_ROOT_DIR and BURC_OUTPUT_ROOT_DIR descriptions from generic 'Root directory for X' to capture actual semantics — TEMP names the per-dispatch scratch subdir pattern (temp-<stamp>); OUTPUT names the fixed 'current/' subdirectory cleared per dispatch. Terse forms aligned with BURC's established style; full semantics belong in BUS0 spec (concerns #6 and #7 from triage ₢A7AAA)

### 2026-05-11 11:47 - ₢A7AAA - n

Reword BURC_PROJECT_ROOT description from direction-style 'Path from burc.env to project root' to identity-style 'Repo root, expressed relative to burc.env' — disambiguates 'project root' (which collided with TEMP_ROOT_DIR/OUTPUT_ROOT_DIR's 'root') by using 'repo root', and names what the variable IS rather than describing the journey (concern #4 from triage ₢A7AAA)

### 2026-05-11 11:43 - ₢A7AAA - n

Reword BURC_TABTARGET_DIR and BURC_TOOLS_DIR descriptions from 'Directory containing' to 'Project dir containing' to clarify these are project-internal paths (concerns #2 and #3 from triage ₢A7AAA)

### 2026-05-11 11:13 - Heat - S

buv-render-add-file-path

### 2026-04-09 14:32 - Heat - S

burc-render-review-triage

### 2026-04-09 13:53 - Heat - f

racing

### 2026-04-09 13:53 - Heat - N

rbk-mvp-3-regime-scrub

