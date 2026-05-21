# Heat Trophy: rbk-12-mvp-moorings-cutover

**Firemark:** ₣BK
**Created:** 260513
**Retired:** 260520
**Status:** retired

## Paddock

## Shape

Cutover of consumer-config + tabtarget path indirection per AAJ findings (₣A_/₢A_AAJ docket). Ten paces, hard cutover, branch-merge expected.

The load-bearing mechanism is `tt/z-launcher.sh` — a trampoline that resolves moorings location, normalizes cwd to repo root, and dispatches to a named launcher. Every other change in this heat orbits that mechanism: the filesystem rename flips its hardcoded path literal; constant pools become downstream consumers of trampoline-established execution context; vocabulary sweeps make the new layout visible to the human reader.

## Locked decisions

- **Hard cutover**, no bridge period. Branch-merge atomic.
- **`rbcc_Constants.sh` is the RBK-side fact locale.** Existing file, extended — not a new file.
- **`bubc_constants.sh` is the BUK-side fact locale.** Established 2026-05-10, ready.
- **Trampoline hardcodes the moorings path literal.** Bash constant pools (RBCC, BUBC) are downstream consumers of trampoline-established execution context, not its inputs.
- **z-launcher.sh chdirs to repo root** before dispatching to the named launcher. Workbenches start with deterministic cwd regardless of user invocation directory.
- **Tabtarget dispatch sprue.** Tabtargets pass a minted moorings-launcher *sprue* to `tt/z-launcher.sh`, never a bare workbench-id — a bare id would conflate the hyphenated colophon universe with launcher dispatch. Form `{owner}ml_{launcher-id}`: RBK-authored launchers take `rbml_` (`rbml_rbw`, `rbml_rbtw`); the BUK launcher infrastructure that hosts every other kit takes `buml_` (`buml_buw`, `buml_jjw`, `buml_cmw`, `buml_vow`, `buml_vvw`, `buml_vslw`, `buml_apcw`, `buml_study`). The `rbml_`/`buml_` prefix is ownership-semantic, **not** a location selector — every launcher file co-locates in `rbmm_moorings/rbml_launchers/` after the rename (`.buk/` is deleted), so the trampoline recovers the launcher-id by stripping the `*ml_` prefix (`${1#*ml_}`) and dispatch stays a single literal across both families. The off-pattern `launcher_nolog.rbw_workbench.sh` takes its own sprue.
- **`rbm*_` prefix family**: `rbmm_` (umbrella), `rbml_` (launchers), `rbmn_` (nodes), `rbmu_` (users), `rbmv_` (vessels). `rbmn_` and `rbmu_` are the BURN/BURP regime pair — sibling consumer-authored profile subtrees under `.buk/`, defined side-by-side in BUBC; `rbmn_` joins the family rather than splitting off.
- **`buml_` prefix family**: BUK moorings launchers, parallel to `rbml_`. Registered as the dispatch-sprue namespace for every non-RBK launcher. A kit that later earns its own moorings migrates its sprue from `buml_` to `{kit}ml_`.
- **AAJ stays in ₣A_.** Nomination happened in agent conversation; AAJ's reslated form becomes "update A_ paddock with rbm*_ findings + verify ₣BK nominated + wrap."

## Discipline

- **Per-pace fast-suite smoke.** Each pace wrap runs `tt/rbtd-s.TestSuite.fast.sh`; must pass before notch. 75 cases, no GCP deps, ~2-3 minutes. Catches breakage at the pace that caused it.
- **`git mv` for file moves** so history follows.
- **Single notch-commit per pace** where the work permits (`jjx_record` handles staging).
- **Parallel-internal dispatch** where the docket calls for it (paces 5 and 8): spawn agents on disjoint file subtrees within the same pace.

## Out of scope

- Extending moorings to other kits (CMK / JJK / VVK / VOK).
- Eliminating per-nameplate `compose.yml` files (they're consumer-authored).
- Redesigning launcher infrastructure beyond introducing `z-launcher.sh`.
- Per-customer migration of consumer-side trees outside this repo.

## Sources

- ₣A_ / ₢A_AAJ docket — design rationale, renames table, BCG codification commitment
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — sections 21-30 (load-bearing complexity), 44-77 (CLI as module gateway), 171-181 (dispatch-provided BURD_*)
- `Tools/rbk/rbcc_Constants.sh` — RBK fact locale (extend, don't replace)
- `Tools/buk/bubc_constants.sh` — BUK fact locale (ready)

## Paces

### bubc-rbmn-prefix-reconciliation (₢BKAAA) [complete]

**[260519-1542] complete**

## Character

Naming-family decision. Opus-grade judgment, no code change.

## Goal

Resolve whether `rbmn_` joins the `rbm*_` moorings family or splits off.

## Context

`Tools/buk/bubc_constants.sh` defines `BUBC_rbmn_nodes_subdir="rbmn_nodes"`, using the `rbmn_` prefix. AAJ docket's `rbm*_` family table lists only `rbmm_`, `rbml_`, `rbmu_`, `rbmv_` — `rbmn_` is absent. Either AAJ's table is now stale (and `rbmn_` joins the family), or `rbmn_` belongs to a different concept warranting its own family.

## Work

- Read `Tools/buk/bubc_constants.sh` to see how `BUBC_rbmn_nodes_subdir` is used.
- Grep the kit for `rbmn_` references to understand the concept.
- Decide: joins `rbm*_` family or splits off.
- If joins: update the heat paddock and pace 9's CLAUDE.md registration scope.
- If splits: document the new family's gestalt in the wrap commit message and consider whether it also needs CLAUDE.md registration.

Pace 1 outcome shapes the constants minted in pace 3 (rbcc-establish-fact-locale) — wrap before pace 3 mounts.

## Sources

- `Tools/buk/bubc_constants.sh` — site of divergence
- ₣A_/₢A_AAJ docket — `rbm*_` family table

**[260513-1257] rough**

## Character

Naming-family decision. Opus-grade judgment, no code change.

## Goal

Resolve whether `rbmn_` joins the `rbm*_` moorings family or splits off.

## Context

`Tools/buk/bubc_constants.sh` defines `BUBC_rbmn_nodes_subdir="rbmn_nodes"`, using the `rbmn_` prefix. AAJ docket's `rbm*_` family table lists only `rbmm_`, `rbml_`, `rbmu_`, `rbmv_` — `rbmn_` is absent. Either AAJ's table is now stale (and `rbmn_` joins the family), or `rbmn_` belongs to a different concept warranting its own family.

## Work

- Read `Tools/buk/bubc_constants.sh` to see how `BUBC_rbmn_nodes_subdir` is used.
- Grep the kit for `rbmn_` references to understand the concept.
- Decide: joins `rbm*_` family or splits off.
- If joins: update the heat paddock and pace 9's CLAUDE.md registration scope.
- If splits: document the new family's gestalt in the wrap commit message and consider whether it also needs CLAUDE.md registration.

Pace 1 outcome shapes the constants minted in pace 3 (rbcc-establish-fact-locale) — wrap before pace 3 mounts.

## Sources

- `Tools/buk/bubc_constants.sh` — site of divergence
- ₣A_/₢A_AAJ docket — `rbm*_` family table

### z-launcher-trampoline-introduce (₢BKAAB) [complete]

**[260520-0134] complete**

## Character

Load-bearing mechanism. Workbench cwd audit gates the mass rewire — silent breakage vector if skipped.

## Goal

`tt/z-launcher.sh` exists, every tabtarget routes through it via its minted dispatch sprue, and no workbench broke from the cwd semantic change.

## Scope

The trampoline and cwd-normalization are **universal** — they apply to every kit's tabtargets and launchers (rbw, buw, jjw, vvw, apcw, cmw, rbtw, vow, vslw, study), not a rbk/buk subset. The moorings *relocation* in later paces is RBK/BUK-only, but the launcher directory all kits dispatch through is shared and moves with it. The cwd audit must therefore cover every `*w_workbench.sh`, not just the RBK/BUK ones.

## Work

### Workbench cwd audit (gates the rewire)

z-launcher.sh chdirs to repo root before dispatching the named launcher. Today, workbenches start with cwd = wherever-user-invoked. Any workbench code assuming user-invocation-cwd breaks silently after the rewire.

Survey before rewiring:

- `grep` every `*w_workbench.sh` and dispatched module for `$PWD`, `cd "${0%/*}"`, and unqualified relative paths.
- Note each site; classify as cwd-stable (fine), cwd-sensitive-fixable (fix in this pace), or cwd-sensitive-requires-rework (surface as a blocker, do not proceed).
- **CLI-furnish cwd-reliance belongs to ₢BKAAC, not here.** ₢BKAAC refactors CLI furnishes to self-locate via `BASH_SOURCE`. If a cwd-sensitive site is a CLI furnish, note it for ₢BKAAC rather than fixing it in this pace.

### Dispatch sprue (paddock locked decision)

Tabtargets pass a minted moorings-launcher *sprue* to the trampoline, never a bare workbench-id. Form `{owner}ml_{launcher-id}`. The full mapping from today's launcher-id to sprue:

- `rbw` → `rbml_rbw`
- `rbtw` → `rbml_rbtw`
- `buw` → `buml_buw`
- `jjw` → `buml_jjw`
- `cmw` → `buml_cmw`
- `vow` → `buml_vow`
- `vvw` → `buml_vvw`
- `vslw` → `buml_vslw`
- `apcw` → `buml_apcw`
- `study` → `buml_study`

### Trampoline introduction

- Create `tt/z-launcher.sh`:
  - Resolves its own dir via `${BASH_SOURCE[0]%/*}`.
  - chdirs to repo root.
  - Receives the sprue as `${1}`; recovers the launcher-id by stripping the ownership prefix (`${1#*ml_}`). Hardcodes the moorings-launchers path as a literal. Initial value points at `../.buk/launcher.${1#*ml_}_workbench.sh` (transitional — ₢BKAAD flips this literal to the new layout; the strip survives the flip unchanged).
  - `exec`s the resolved launcher with the remaining positional args forwarded (`"${@:2}"`).
  - Carries a comment block enumerating the valid sprues and the `*ml_`-strip contract, so the dispatch token universe is self-documenting (CLAUDE.md registration is ₢BKAAI).
  - Fails loud (BCG die discipline) if the resolved launcher path does not exist — this catches a mistyped sprue in the rewire rather than dispatching silently to nothing.
- **Off-pattern launcher:** `.buk/launcher_nolog.rbw_workbench.sh` does not fit the `launcher.${launcher-id}_workbench.sh` shape. Decide its handling explicitly so it yields a *valid sprue* — a distinct sprue whose stripped form resolves to the nolog file, a distinct trampoline arg, or folding its no-log behavior into a flag. Do not let it fall through `${1#*ml_}` substitution silently.

### Tabtarget rewire

- Rewrite every `tt/*.sh` (excluding `z-launcher.sh` itself) from the current `${BURD_LAUNCHER}` pattern to `exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <sprue> "${@}"`.
- Derive each tabtarget's sprue from its **existing `BURD_LAUNCHER` launcher-id** via the mapping above — NOT from the colophon prefix. Colophon ≠ launcher-id in at least two families: `rbtd-*` tabtargets dispatch `rbtw` (→ `rbml_rbtw`) and `vslk-*` tabtargets dispatch `vslw` (→ `buml_vslw`). Read the current `BURD_LAUNCHER` line of each tabtarget to get the truth.

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Spot-check sampling of tabtargets executes correctly from at least three different cwd locations.

## Sources

- ₣A_/₢A_AAJ docket — "Trampoline design", "Execution-context normalization"
- ₣BK paddock — "Tabtarget dispatch sprue" locked decision (authoritative sprue list)
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` sections 171-181 — dispatch-provided `BURD_*`, the pattern this extends
- Any `*w_workbench.sh` under `Tools/` — audit surface

**[260519-1616] rough**

## Character

Load-bearing mechanism. Workbench cwd audit gates the mass rewire — silent breakage vector if skipped.

## Goal

`tt/z-launcher.sh` exists, every tabtarget routes through it via its minted dispatch sprue, and no workbench broke from the cwd semantic change.

## Scope

The trampoline and cwd-normalization are **universal** — they apply to every kit's tabtargets and launchers (rbw, buw, jjw, vvw, apcw, cmw, rbtw, vow, vslw, study), not a rbk/buk subset. The moorings *relocation* in later paces is RBK/BUK-only, but the launcher directory all kits dispatch through is shared and moves with it. The cwd audit must therefore cover every `*w_workbench.sh`, not just the RBK/BUK ones.

## Work

### Workbench cwd audit (gates the rewire)

z-launcher.sh chdirs to repo root before dispatching the named launcher. Today, workbenches start with cwd = wherever-user-invoked. Any workbench code assuming user-invocation-cwd breaks silently after the rewire.

Survey before rewiring:

- `grep` every `*w_workbench.sh` and dispatched module for `$PWD`, `cd "${0%/*}"`, and unqualified relative paths.
- Note each site; classify as cwd-stable (fine), cwd-sensitive-fixable (fix in this pace), or cwd-sensitive-requires-rework (surface as a blocker, do not proceed).
- **CLI-furnish cwd-reliance belongs to ₢BKAAC, not here.** ₢BKAAC refactors CLI furnishes to self-locate via `BASH_SOURCE`. If a cwd-sensitive site is a CLI furnish, note it for ₢BKAAC rather than fixing it in this pace.

### Dispatch sprue (paddock locked decision)

Tabtargets pass a minted moorings-launcher *sprue* to the trampoline, never a bare workbench-id. Form `{owner}ml_{launcher-id}`. The full mapping from today's launcher-id to sprue:

- `rbw` → `rbml_rbw`
- `rbtw` → `rbml_rbtw`
- `buw` → `buml_buw`
- `jjw` → `buml_jjw`
- `cmw` → `buml_cmw`
- `vow` → `buml_vow`
- `vvw` → `buml_vvw`
- `vslw` → `buml_vslw`
- `apcw` → `buml_apcw`
- `study` → `buml_study`

### Trampoline introduction

- Create `tt/z-launcher.sh`:
  - Resolves its own dir via `${BASH_SOURCE[0]%/*}`.
  - chdirs to repo root.
  - Receives the sprue as `${1}`; recovers the launcher-id by stripping the ownership prefix (`${1#*ml_}`). Hardcodes the moorings-launchers path as a literal. Initial value points at `../.buk/launcher.${1#*ml_}_workbench.sh` (transitional — ₢BKAAD flips this literal to the new layout; the strip survives the flip unchanged).
  - `exec`s the resolved launcher with the remaining positional args forwarded (`"${@:2}"`).
  - Carries a comment block enumerating the valid sprues and the `*ml_`-strip contract, so the dispatch token universe is self-documenting (CLAUDE.md registration is ₢BKAAI).
  - Fails loud (BCG die discipline) if the resolved launcher path does not exist — this catches a mistyped sprue in the rewire rather than dispatching silently to nothing.
- **Off-pattern launcher:** `.buk/launcher_nolog.rbw_workbench.sh` does not fit the `launcher.${launcher-id}_workbench.sh` shape. Decide its handling explicitly so it yields a *valid sprue* — a distinct sprue whose stripped form resolves to the nolog file, a distinct trampoline arg, or folding its no-log behavior into a flag. Do not let it fall through `${1#*ml_}` substitution silently.

### Tabtarget rewire

- Rewrite every `tt/*.sh` (excluding `z-launcher.sh` itself) from the current `${BURD_LAUNCHER}` pattern to `exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <sprue> "${@}"`.
- Derive each tabtarget's sprue from its **existing `BURD_LAUNCHER` launcher-id** via the mapping above — NOT from the colophon prefix. Colophon ≠ launcher-id in at least two families: `rbtd-*` tabtargets dispatch `rbtw` (→ `rbml_rbtw`) and `vslk-*` tabtargets dispatch `vslw` (→ `buml_vslw`). Read the current `BURD_LAUNCHER` line of each tabtarget to get the truth.

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Spot-check sampling of tabtargets executes correctly from at least three different cwd locations.

## Sources

- ₣A_/₢A_AAJ docket — "Trampoline design", "Execution-context normalization"
- ₣BK paddock — "Tabtarget dispatch sprue" locked decision (authoritative sprue list)
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` sections 171-181 — dispatch-provided `BURD_*`, the pattern this extends
- Any `*w_workbench.sh` under `Tools/` — audit surface

**[260519-1559] rough**

## Character

Load-bearing mechanism. Workbench cwd audit gates the mass rewire — silent breakage vector if skipped.

## Goal

`tt/z-launcher.sh` exists, every tabtarget routes through it, and no workbench broke from the cwd semantic change.

## Scope

The trampoline and cwd-normalization are **universal** — they apply to every kit's tabtargets and launchers (rbw, buw, jjw, vvw, apcw, cmw, rbtw, vow, vslw, study), not a rbk/buk subset. The moorings *relocation* in later paces is RBK/BUK-only, but the launcher directory all kits dispatch through is shared and moves with it. The cwd audit must therefore cover every `*w_workbench.sh`, not just the RBK/BUK ones.

## Work

### Workbench cwd audit (gates the rewire)

z-launcher.sh chdirs to repo root before dispatching the named launcher. Today, workbenches start with cwd = wherever-user-invoked. Any workbench code assuming user-invocation-cwd breaks silently after the rewire.

Survey before rewiring:

- `grep` every `*w_workbench.sh` and dispatched module for `$PWD`, `cd "${0%/*}"`, and unqualified relative paths.
- Note each site; classify as cwd-stable (fine), cwd-sensitive-fixable (fix in this pace), or cwd-sensitive-requires-rework (surface as a blocker, do not proceed).
- **CLI-furnish cwd-reliance belongs to ₢BKAAC, not here.** AAC refactors CLI furnishes to self-locate via `BASH_SOURCE`. If a cwd-sensitive site is a CLI furnish, note it for AAC rather than fixing it in this pace.

### Trampoline introduction

- Create `tt/z-launcher.sh`:
  - Resolves its own dir via `${BASH_SOURCE[0]%/*}`.
  - chdirs to repo root.
  - Hardcodes the moorings-launchers path as a literal string. Initial value points at `../.buk/launcher.${1}_workbench.sh` (transitional — ₢BKAAD flips this literal to the new layout).
  - `exec`s the named launcher with positional args forwarded.
- **Off-pattern launcher:** `.buk/launcher_nolog.rbw_workbench.sh` does not fit the `launcher.${1}_workbench.sh` shape. Decide its handling explicitly — a second workbench-id form, a distinct trampoline arg, or folding its no-log behavior into a flag. Do not let it fall through `${1}` substitution silently.

### Tabtarget rewire

- Rewrite every `tt/*.sh` (excluding `z-launcher.sh` itself) from the current `${BURD_LAUNCHER}` pattern to `exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <workbench-id> "${@}"`.
- The workbench-id is the existing colophon prefix's workbench identifier (e.g., `rbw`, `buw`, `jjw`).

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Spot-check sampling of tabtargets executes correctly from at least three different cwd locations.

## Sources

- ₣A_/₢A_AAJ docket — "Trampoline design", "Execution-context normalization"
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` sections 171-181 — dispatch-provided `BURD_*`, the pattern this extends
- Any `*w_workbench.sh` under `Tools/` — audit surface

**[260513-1257] rough**

## Character

Load-bearing mechanism. Workbench cwd audit gates the mass rewire — silent breakage vector if skipped.

## Goal

`tt/z-launcher.sh` exists, every tabtarget routes through it, and no workbench broke from the cwd semantic change.

## Work

### Workbench cwd audit (gates the rewire)

z-launcher.sh chdirs to repo root before dispatching the named launcher. Today, workbenches start with cwd = wherever-user-invoked. Any workbench code assuming user-invocation-cwd breaks silently after the rewire.

Survey before rewiring:

- `grep` every `*w_workbench.sh` and dispatched module for `$PWD`, `cd "${0%/*}"`, and unqualified relative paths.
- Note each site; classify as cwd-stable (fine), cwd-sensitive-fixable (fix in this pace), or cwd-sensitive-requires-rework (surface as a blocker, do not proceed).

### Trampoline introduction

- Create `tt/z-launcher.sh`:
  - Resolves its own dir via `${BASH_SOURCE[0]%/*}`.
  - chdirs to repo root.
  - Hardcodes the moorings-launchers path as a literal string. Initial value points at `../.buk/launcher.${1}_workbench.sh` (transitional — pace 4 flips this literal to the new layout).
  - `exec`s the named launcher with positional args forwarded.

### Tabtarget rewire

- Rewrite every `tt/*.sh` from the current `${BURD_LAUNCHER}` pattern to `exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <workbench-id> "${@}"`.
- The workbench-id is the existing colophon prefix's workbench identifier (e.g., `rbw`, `buw`, `jjw`).

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Spot-check sampling of tabtargets executes correctly from at least three different cwd locations.

## Sources

- ₣A_/₢A_AAJ docket — "Trampoline design", "Execution-context normalization"
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` sections 171-181 — dispatch-provided `BURD_*`, the pattern this extends
- Any `*w_workbench.sh` under `Tools/` — audit surface

### rbcc-establish-fact-locale (₢BKAAC) [complete]

**[260520-0808] complete**

## Character

Absorb RBBC into RBCC using the kit-self-location pattern already proven in `Tools/rbk/rbtd/rbte_cli.sh`. Mechanical retarget across ~20 CLI furnishes, light judgment on transitional name handling.

## Goal

`.buk/rbbc_constants.sh` is deleted. RBCC is the sole RBK fact locale. Every CLI furnish self-locates via `${BASH_SOURCE[0]%/*}`. New moorings constants exist in RBCC as inventory for later paces to consume.

## Context

RBBC's sole role is bootstrapping the path to RBCC inside CLI furnishes — a workaround for what `BASH_SOURCE[0]` answers natively. `rbte_cli.sh` already lives without RBBC; the other CLIs are laggards. Post-absorption, RBBC has no remaining role.

## Work

### Mint moorings constants in RBCC (inventory, unread at wrap)

Add source-time literals per AAJ Renames table. Subdir constants: `RBCC_moorings_dir`, `RBCC_launchers_subdir`, `RBCC_users_subdir`, `RBCC_nodes_subdir`, `RBCC_vessels_subdir`. File constants, moorings-prefixed: `RBCC_rbrr_file`, `RBCC_rbrp_file`, `RBCC_rbrm_file`, `RBCC_rbrd_file`. `RBCC_KIT_DIR` becomes a source-time `${BASH_SOURCE[0]%/*}` derivation.

`rbmn_nodes` and `rbmu_users` are the BURN/BURP pair per ₢BKAAA — both get subdir constants. The constant set must cover the full inventory ₢BKAAD moves: confirm against `git ls-files .buk .rbk rbev-vessels` so no tracked config file (e.g. `rbje_compose_probe.env`) lacks a destination home.

These constants are new names that no kit code reads yet — they are inventory for ₢BKAAD (value-flip in lockstep with filesystem move) and ₢BKAAE (literal sweep).

### Retarget CLI furnishes

Discovery: `grep -l 'rbbc_constants.sh' Tools/rbk/`. Each match converts from the RBBC-bootstrap pattern to the `rbte_cli.sh` self-location pattern. The bootstrap source line drops; `z_rbk_kit_dir` derives from BASH_SOURCE.

### Preserve RBBC_* names as transitional aliases in RBCC

Non-furnish kit code (rbob_bottle, rbrn_regime, rblm_cli, rbh0/*, rbte_engine, etc.) still reads `RBBC_dot_dir`, `RBBC_rbrr_file`, etc. Emit these as RBCC-side aliases pointing at their CURRENT values (`.rbk/...`) — ₢BKAAD flips them when the filesystem moves; ₢BKAAE sweeps the names. The alias set must cover every `RBBC_*` name kit code currently reads — `grep -rn 'RBBC_' Tools/` to enumerate before deciding the set.

### Delete RBBC and drop RBCC's source line

Delete `.buk/rbbc_constants.sh`. Drop RBCC's BURD_CONFIG_DIR precondition + rbbc source line.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes (existing kit code reads aliases unchanged).
- `grep -rn 'rbbc_constants' Tools/ .buk/` returns no surviving sources.
- `rbte_cli.sh` is no longer the only file using the BASH_SOURCE self-location pattern.

## Flagged for wrap commit message

`Tools/buk/buts/butcrg_RegimeCredentials.sh` reads `${RBBC_rbrr_file}` — BUK testbench reaching into RBK paths via the shared stub. Post-absorption needs an explicit RBCC source in the BUK furnish chain (with one `"rbk"` literal in BUK code, defensible because the fixture tests RBK regimes) or relocation to RBK testbench. Out of scope here; surface for operator review.

## Sources

- `Tools/rbk/rbtd/rbte_cli.sh` — proven self-location canon
- `Tools/rbk/rbq_cli.sh` — exemplar of the laggard pattern
- `Tools/rbk/rbcc_Constants.sh` — destination fact locale
- ₣A_/₢A_AAJ docket — Renames table, Symbolic-constant discipline

**[260519-1559] rough**

## Character

Absorb RBBC into RBCC using the kit-self-location pattern already proven in `Tools/rbk/rbtd/rbte_cli.sh`. Mechanical retarget across ~20 CLI furnishes, light judgment on transitional name handling.

## Goal

`.buk/rbbc_constants.sh` is deleted. RBCC is the sole RBK fact locale. Every CLI furnish self-locates via `${BASH_SOURCE[0]%/*}`. New moorings constants exist in RBCC as inventory for later paces to consume.

## Context

RBBC's sole role is bootstrapping the path to RBCC inside CLI furnishes — a workaround for what `BASH_SOURCE[0]` answers natively. `rbte_cli.sh` already lives without RBBC; the other CLIs are laggards. Post-absorption, RBBC has no remaining role.

## Work

### Mint moorings constants in RBCC (inventory, unread at wrap)

Add source-time literals per AAJ Renames table. Subdir constants: `RBCC_moorings_dir`, `RBCC_launchers_subdir`, `RBCC_users_subdir`, `RBCC_nodes_subdir`, `RBCC_vessels_subdir`. File constants, moorings-prefixed: `RBCC_rbrr_file`, `RBCC_rbrp_file`, `RBCC_rbrm_file`, `RBCC_rbrd_file`. `RBCC_KIT_DIR` becomes a source-time `${BASH_SOURCE[0]%/*}` derivation.

`rbmn_nodes` and `rbmu_users` are the BURN/BURP pair per ₢BKAAA — both get subdir constants. The constant set must cover the full inventory ₢BKAAD moves: confirm against `git ls-files .buk .rbk rbev-vessels` so no tracked config file (e.g. `rbje_compose_probe.env`) lacks a destination home.

These constants are new names that no kit code reads yet — they are inventory for ₢BKAAD (value-flip in lockstep with filesystem move) and ₢BKAAE (literal sweep).

### Retarget CLI furnishes

Discovery: `grep -l 'rbbc_constants.sh' Tools/rbk/`. Each match converts from the RBBC-bootstrap pattern to the `rbte_cli.sh` self-location pattern. The bootstrap source line drops; `z_rbk_kit_dir` derives from BASH_SOURCE.

### Preserve RBBC_* names as transitional aliases in RBCC

Non-furnish kit code (rbob_bottle, rbrn_regime, rblm_cli, rbh0/*, rbte_engine, etc.) still reads `RBBC_dot_dir`, `RBBC_rbrr_file`, etc. Emit these as RBCC-side aliases pointing at their CURRENT values (`.rbk/...`) — ₢BKAAD flips them when the filesystem moves; ₢BKAAE sweeps the names. The alias set must cover every `RBBC_*` name kit code currently reads — `grep -rn 'RBBC_' Tools/` to enumerate before deciding the set.

### Delete RBBC and drop RBCC's source line

Delete `.buk/rbbc_constants.sh`. Drop RBCC's BURD_CONFIG_DIR precondition + rbbc source line.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes (existing kit code reads aliases unchanged).
- `grep -rn 'rbbc_constants' Tools/ .buk/` returns no surviving sources.
- `rbte_cli.sh` is no longer the only file using the BASH_SOURCE self-location pattern.

## Flagged for wrap commit message

`Tools/buk/buts/butcrg_RegimeCredentials.sh` reads `${RBBC_rbrr_file}` — BUK testbench reaching into RBK paths via the shared stub. Post-absorption needs an explicit RBCC source in the BUK furnish chain (with one `"rbk"` literal in BUK code, defensible because the fixture tests RBK regimes) or relocation to RBK testbench. Out of scope here; surface for operator review.

## Sources

- `Tools/rbk/rbtd/rbte_cli.sh` — proven self-location canon
- `Tools/rbk/rbq_cli.sh` — exemplar of the laggard pattern
- `Tools/rbk/rbcc_Constants.sh` — destination fact locale
- ₣A_/₢A_AAJ docket — Renames table, Symbolic-constant discipline

**[260514-1204] rough**

## Character

Absorb RBBC into RBCC using the kit-self-location pattern already proven in `Tools/rbk/rbtd/rbte_cli.sh`. Mechanical retarget across ~20 CLI furnishes, light judgment on transitional name handling.

## Goal

`.buk/rbbc_constants.sh` is deleted. RBCC is the sole RBK fact locale. Every CLI furnish self-locates via `${BASH_SOURCE[0]%/*}`. New moorings constants exist in RBCC as inventory for later paces to consume.

## Context

RBBC's sole role is bootstrapping the path to RBCC inside CLI furnishes — a workaround for what `BASH_SOURCE[0]` answers natively. `rbte_cli.sh` already lives without RBBC; the other CLIs are laggards. Post-absorption, RBBC has no remaining role.

## Work

### Mint moorings constants in RBCC (inventory, unread at wrap)

Add source-time literals per AAJ Renames table — `RBCC_moorings_dir`, `RBCC_launchers_subdir`, `RBCC_users_subdir`, `RBCC_vessels_subdir`, plus moorings-prefixed `RBCC_rbrr_file` / `RBCC_rbrp_file` / `RBCC_rbrm_file`. `RBCC_KIT_DIR` becomes a source-time `${BASH_SOURCE[0]%/*}` derivation. Include any constants ₢BKAAA's `rbmn_` reconciliation surfaces.

These constants are new names that no kit code reads yet — they are inventory for ₢BKAAD (value-flip in lockstep with filesystem move) and ₢BKAAE (literal sweep).

### Retarget CLI furnishes

Discovery: `grep -l 'rbbc_constants.sh' Tools/rbk/`. Each match converts from the RBBC-bootstrap pattern to the `rbte_cli.sh:27` self-location pattern. The bootstrap source line drops; `z_rbk_kit_dir` derives from BASH_SOURCE.

### Preserve RBBC_* names as transitional aliases in RBCC

Non-furnish kit code (rbob_bottle, rbrn_regime, rblm_cli, rbh0/*, rbte_engine, etc.) still reads `RBBC_dot_dir`, `RBBC_rbrr_file`, etc. Emit these as RBCC-side aliases pointing at their CURRENT values (`.rbk/...`) — `₢BKAAD` flips them when the filesystem moves; `₢BKAAE` sweeps the names.

### Delete RBBC and drop RBCC's source line

Delete `.buk/rbbc_constants.sh`. Drop RBCC's BURD_CONFIG_DIR precondition + rbbc source (current lines 27-29).

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes (existing kit code reads aliases unchanged).
- `grep -rn 'rbbc_constants' Tools/ .buk/` returns no surviving sources.
- `rbte_cli.sh` is no longer the only file using the BASH_SOURCE self-location pattern.

## Flagged for wrap commit message

`Tools/buk/buts/butcrg_RegimeCredentials.sh` reads `${RBBC_rbrr_file}` — BUK testbench reaching into RBK paths via the shared stub. Post-absorption needs an explicit RBCC source in the BUK furnish chain (with one `"rbk"` literal in BUK code, defensible because the fixture tests RBK regimes) or relocation to RBK testbench. Out of scope here; surface for operator review.

## Sources

- `Tools/rbk/rbtd/rbte_cli.sh` — proven self-location canon
- `Tools/rbk/rbq_cli.sh` — exemplar of the laggard pattern
- `Tools/rbk/rbcc_Constants.sh` — destination fact locale
- ₣A_/₢A_AAJ docket — Renames table, Symbolic-constant discipline

**[260513-1258] rough**

## Character

Migrate consumer-side bootstrap into kit-side fact locale. Light judgment per item; sensitive to behavioral change.

## Goal

`Tools/rbk/rbcc_Constants.sh` is the sole RBK fact locale. Consumer-side `.buk/rbbc_constants.sh` source line is gone; moorings path constants are minted.

## Work

### rbbc consumer-content audit

The current `rbcc_Constants.sh` sources `${BURD_CONFIG_DIR}/rbbc_constants.sh` at source-time. Before eliminating that line, read what `rbbc_constants.sh` currently provides:

- Inspect `.buk/rbbc_constants.sh` in this tree.
- Classify each constant: kit-default (safe to absorb), consumer-customized (behavioral change to absorb — flag and decide), or obsolete (delete).
- Eliminating real consumer overrides without surfacing the change is a behavioral regression. If overrides exist, document them in the wrap commit message.

### Absorb survivors

- Move surviving constants from `rbbc_constants.sh` into `rbcc_Constants.sh` directly, in the appropriate section.
- Remove the `source "${BURD_CONFIG_DIR}/rbbc_constants.sh"` line and its `BURD_CONFIG_DIR` precondition check from `rbcc_Constants.sh`.
- Delete `.buk/rbbc_constants.sh` (file itself; pace 4 handles the directory move).

### Mint moorings constants

Add to `rbcc_Constants.sh` (in the literal-constants section near `RBCC_rbrn_file`):

- `RBCC_moorings_dir` — value per AAJ table (`rbmm_moorings`)
- `RBCC_launchers_subdir` — value per AAJ table (`rbml_launchers`)
- `RBCC_users_subdir` — match BUBC parallel (`rbmu_users`)
- `RBCC_vessels_subdir` — value per AAJ table (`rbmv_vessels`)
- Any additional moorings paths surfaced by pace 1's `rbmn_` reconciliation.

Match the BUBC naming pattern where the concept is parallel (BUBC_rbmu_users_subdir already exists with value `rbmu_users` — RBCC should hold the same value, the symbol on each side).

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- No call-site changes in this pace — that's pace 5.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — RBK fact locale (extend, don't replace)
- `Tools/buk/bubc_constants.sh` — parallel BUK locale, naming reference
- `.buk/rbbc_constants.sh` — consumer-side file being eliminated
- ₣A_/₢A_AAJ docket — Renames table, Symbolic-constant discipline

### moorings-filesystem-rename (₢BKAAD) [complete]

**[260520-1051] complete**

## Character

Hard cutover, highest-risk pace. Filesystem rename + every path resolver + consumer-config content + test fixtures, one atomic commit. Recovery is `git revert`.

## Goal

`rbmm_moorings/` holds the new layout; `.buk/` and `.rbk/` and `rbev-vessels/` are gone. All tabtargets execute; fast suite passes.

## Locked (mount-conversation findings, operator-confirmed)

- **`rbje_compose_probe.env` is kit machinery**, classified with `rbob_compose.yml`. Both relocate to `Tools/rbk/` THIS pace (pulled forward from the ₢BKAAI carve-out so `rbob_bottle.sh` charge stays green across the `RBBC_dot_dir` flip). `rbob_bottle.sh` rewires its compose+probe reads from `${RBBC_dot_dir}/...` to `${RBCC_KIT_DIR}/...`. ₢BKAAI is thereby reduced to `rbmP_laterSecurityAudit.md` only.
- **Magic-string discipline (RCG Constant Discipline / BCG tinder).** The `RBBC_dot_dir` flip reaches only consumers that read THROUGH the alias (~40 RBK sites, free). Every site that hardcoded a raw `.buk`/`.rbk`/`rbev-vessels` literal is stranded and must be repaired — by deriving from the single source of truth, not by re-literaling, except where bootstrap forbids it.
- **Bootstrap-anchor exception.** `bul_launcher.sh` config-dir literal is irreducible (must locate config before sourcing anything); it IS the anchor, holding `rbmm_moorings`. `BUBC_moorings_dir` mirrors it for non-bootstrap consumers. The cross-kit `burc.env` readers (BUK `buut_cli`, VOK `vob_*`, VSLK `vslw`, VVK `vvi/vvu`) are each independent bootstrap entry points → each holds the literal.
- **`jjrlg_legatio.rs` stays on `.buk`** (resolves config on a remote fundus that may be unmigrated — cross-repo, out of scope). FOLLOW-UP: foray against a post-cutover fundus needs jjrlg updated; capture as itch/pace.
- **Launcher home moves two levels deep** (`rbmm_moorings/rbml_launchers/`), so `bul_launcher` project-root derivation gains a level. Trampoline literal flip + launcher move + this derivation fix are inseparable.

## Work (discovery recipes — sites drift)

- Moves: `git ls-files .buk .rbk rbev-vessels` is the manifest; carve `rbob_compose.yml`+`rbje_compose_probe.env` to `Tools/rbk/`.
- Launcher-path resolvers: `grep -rn 'launcher\.\${.*}_workbench\|/launcher\.' tt/ Tools/buk Tools/rbk`.
- burc.env anchors: `grep -rn '\.buk/burc\.env' Tools/` (skip jjrlg).
- Alias flip: `RBBC_dot_dir` in `rbcc_Constants.sh`; mirror `BUBC_moorings_dir` in `bubc_constants.sh`.
- Content: `RBRR_VESSEL_DIR` (live `rbrr.env` + `rblm_cli.sh` template).
- Fixtures: `grep -rn '\.rbk\|rbev-vessels' Tools/rbk/rbtd/src` — flip path consts; rebuild theurge (`tt/rbtd-b`) before fast suite.
- Accuracy strings: residual `.buk`/`.rbk` in burn/burp/burs/burd/rbndb display+comments.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes (after `tt/rbtd-b.Build.sh`).
- One tabtarget per family executes: rbw, buw, jjw, vvw (+ vow/vslw, touched here).
- `grep -rn '\.buk\b\|\.rbk\b' Tools/ tt/` shows only the deliberate jjrlg remote-fundus reads.

**[260520-0835] rough**

## Character

Hard cutover, highest-risk pace. Filesystem rename + every path resolver + consumer-config content + test fixtures, one atomic commit. Recovery is `git revert`.

## Goal

`rbmm_moorings/` holds the new layout; `.buk/` and `.rbk/` and `rbev-vessels/` are gone. All tabtargets execute; fast suite passes.

## Locked (mount-conversation findings, operator-confirmed)

- **`rbje_compose_probe.env` is kit machinery**, classified with `rbob_compose.yml`. Both relocate to `Tools/rbk/` THIS pace (pulled forward from the ₢BKAAI carve-out so `rbob_bottle.sh` charge stays green across the `RBBC_dot_dir` flip). `rbob_bottle.sh` rewires its compose+probe reads from `${RBBC_dot_dir}/...` to `${RBCC_KIT_DIR}/...`. ₢BKAAI is thereby reduced to `rbmP_laterSecurityAudit.md` only.
- **Magic-string discipline (RCG Constant Discipline / BCG tinder).** The `RBBC_dot_dir` flip reaches only consumers that read THROUGH the alias (~40 RBK sites, free). Every site that hardcoded a raw `.buk`/`.rbk`/`rbev-vessels` literal is stranded and must be repaired — by deriving from the single source of truth, not by re-literaling, except where bootstrap forbids it.
- **Bootstrap-anchor exception.** `bul_launcher.sh` config-dir literal is irreducible (must locate config before sourcing anything); it IS the anchor, holding `rbmm_moorings`. `BUBC_moorings_dir` mirrors it for non-bootstrap consumers. The cross-kit `burc.env` readers (BUK `buut_cli`, VOK `vob_*`, VSLK `vslw`, VVK `vvi/vvu`) are each independent bootstrap entry points → each holds the literal.
- **`jjrlg_legatio.rs` stays on `.buk`** (resolves config on a remote fundus that may be unmigrated — cross-repo, out of scope). FOLLOW-UP: foray against a post-cutover fundus needs jjrlg updated; capture as itch/pace.
- **Launcher home moves two levels deep** (`rbmm_moorings/rbml_launchers/`), so `bul_launcher` project-root derivation gains a level. Trampoline literal flip + launcher move + this derivation fix are inseparable.

## Work (discovery recipes — sites drift)

- Moves: `git ls-files .buk .rbk rbev-vessels` is the manifest; carve `rbob_compose.yml`+`rbje_compose_probe.env` to `Tools/rbk/`.
- Launcher-path resolvers: `grep -rn 'launcher\.\${.*}_workbench\|/launcher\.' tt/ Tools/buk Tools/rbk`.
- burc.env anchors: `grep -rn '\.buk/burc\.env' Tools/` (skip jjrlg).
- Alias flip: `RBBC_dot_dir` in `rbcc_Constants.sh`; mirror `BUBC_moorings_dir` in `bubc_constants.sh`.
- Content: `RBRR_VESSEL_DIR` (live `rbrr.env` + `rblm_cli.sh` template).
- Fixtures: `grep -rn '\.rbk\|rbev-vessels' Tools/rbk/rbtd/src` — flip path consts; rebuild theurge (`tt/rbtd-b`) before fast suite.
- Accuracy strings: residual `.buk`/`.rbk` in burn/burp/burs/burd/rbndb display+comments.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes (after `tt/rbtd-b.Build.sh`).
- One tabtarget per family executes: rbw, buw, jjw, vvw (+ vow/vslw, touched here).
- `grep -rn '\.buk\b\|\.rbk\b' Tools/ tt/` shows only the deliberate jjrlg remote-fundus reads.

**[260519-1559] rough**

## Character

Hard cutover. The atomic move: filesystem rename + trampoline literal flip + RBCC alias value-flip + test-fixture path updates, all in one commit.

## Goal

`rbmm_moorings/` exists with the new layout. Old `.buk/`, `.rbk/`, and `rbev-vessels/` directories are gone. Trampoline literal points at the new launcher path. `RBBC_*` aliases in RBCC now resolve to moorings paths. All tabtargets still execute.

## Work

### Inventory check (do this first)

The renames table below must account for every tracked file under the three old roots. Before moving, run `git ls-files .buk .rbk rbev-vessels` and reconcile against the table — any tracked file not covered is a finding to resolve here, not leave stranded. Two carve-outs:

- `.rbk/rbje_compose_probe.env` — classify before moving: consumer config (→ moorings) or kit machinery (→ `Tools/rbk/`, like `rbob_compose.yml` does via ₢BKAAI). It is deliberately NOT in the table below pending that call.
- `.rbk/rbob_compose.yml` and `Tools/rbk/rbmP_laterSecurityAudit.md` are ₢BKAAI — NOT this pace.

### File moves (use `git mv` so history follows)

- `.buk/launcher.<wb>_workbench.sh` (all kits) → `rbmm_moorings/rbml_launchers/launcher.<wb>_workbench.sh`. Includes the off-pattern `launcher_nolog.rbw_workbench.sh`, moved to match whatever dispatch form ₢BKAAB settled on. (Drop the `_workbench` segment only if ₢BKAAB's trampoline literal expects it dropped — verify against that literal, not this table.)
- `.buk/rbmu_users/` → `rbmm_moorings/rbmu_users/`
- `.buk/rbmn_nodes/` → `rbmm_moorings/rbmn_nodes/` (BURN profiles — the `rbmn_` nodes member; absent from the original table)
- `.buk/burc.env` → `rbmm_moorings/burc.env`
- `.rbk/rbrr.env` → `rbmm_moorings/rbrr.env`
- `.rbk/rbrp.env` → `rbmm_moorings/rbrp.env`
- `.rbk/rbrd.env` → `rbmm_moorings/rbrd.env`
- `.rbk/<nameplate>/` → `rbmm_moorings/<nameplate>/` (each nameplate dir with its `rbrn.env`, `rbnnh_*`, and any `workspace/`)
- `rbev-vessels/` → `rbmm_moorings/rbmv_vessels/`

### Trampoline literal flip

Edit `tt/z-launcher.sh` to point at the new launcher path per ₢BKAAB's exact form. Single literal edit.

### RBCC alias value-flip

₢BKAAC left transitional `RBBC_*` aliases in RBCC pointing at `.rbk/...`. Flip the root value so aliases now resolve to moorings paths:

- `RBBC_dot_dir`: `.rbk` → `rbmm_moorings`
- Composed aliases (`RBBC_rbrr_file`, `RBBC_rbrp_file`, `RBBC_rbrm_file`, `RBBC_rbrd_file`) auto-flip via composition.

Single-line edit in RBCC, atomically bound to the filesystem move — both must land together so existing kit code reads paths matching the new filesystem.

### Test-fixture path updates

Any test fixture that creates `.buk/`, `.rbk/`, or `rbev-vessels/` paths in its setup must be updated in this same pace so the test suite still functions:

- Sweep `Tools/rbk/rbtd/` and test infrastructure for setup code creating these paths.
- Update fixture setup to create `rbmm_moorings/`-shaped trees.
- Mechanical but bulky.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- A representative tabtarget from each colophon family (rbw, buw, jjw, vvw) executes correctly.

## Risk

Highest-risk pace in the heat. Recovery is `git revert` of the wrap commit. Keep the commit pure rename + literal + alias-flip + fixture-paths.

## Sources

- ₣A_/₢A_AAJ docket — Renames table, Final layout
- `tt/z-launcher.sh` (created in ₢BKAAB) — literal to flip
- `Tools/rbk/rbcc_Constants.sh` — RBBC aliases established by ₢BKAAC
- `Tools/rbk/rbtd/` — test fixture surface

**[260514-1206] rough**

## Character

Hard cutover. The atomic move: filesystem rename + trampoline literal flip + RBCC alias value-flip + test-fixture path updates, all in one commit.

## Goal

`rbmm_moorings/` exists with the new layout. Old `.buk/`, `.rbk/`, and `rbev-vessels/` directories are gone. Trampoline literal points at the new launcher path. `RBBC_*` aliases in RBCC now resolve to moorings paths. All tabtargets still execute.

## Work

### File moves (use `git mv` so history follows)

Per AAJ Renames table:

- `.buk/launcher.*.sh` → `rbmm_moorings/rbml_launchers/launcher.*.sh` (drop the `_workbench` part per AAJ docket if applicable; preserve otherwise — verify against docket text)
- `.buk/users/` → `rbmm_moorings/rbmu_users/`
- `.buk/burc.env` → `rbmm_moorings/burc.env`
- `.rbk/rbrr.env` → `rbmm_moorings/rbrr.env`
- `.rbk/rbrp.env` → `rbmm_moorings/rbrp.env`
- `.rbk/<nameplate>/` → `rbmm_moorings/<nameplate>/` (each nameplate dir with its `rbrn.env` and `compose.yml`)
- `rbev-vessels/` → `rbmm_moorings/rbmv_vessels/`

`.rbk/rbob_compose.yml` and `Tools/rbk/rbmP_laterSecurityAudit.md` are ₢BKAAI (kit-cleanup) — NOT this pace.

### Trampoline literal flip

Edit `tt/z-launcher.sh` to point at the new launcher path per ₢BKAAB's exact form. Single literal edit.

### RBCC alias value-flip

₢BKAAC left transitional `RBBC_*` aliases in RBCC pointing at `.rbk/...`. Flip the root value so aliases now resolve to moorings paths:

- `RBBC_dot_dir`: `.rbk` → `rbmm_moorings`
- Composed aliases (`RBBC_rbrr_file`, `RBBC_rbrp_file`, `RBBC_rbrm_file`) auto-flip via composition.

Single-line edit in RBCC, atomically bound to the filesystem move — both must land together so existing kit code reads paths matching the new filesystem.

### Test-fixture path updates

Any test fixture that creates `.buk/`, `.rbk/`, or `rbev-vessels/` paths in its setup must be updated in this same pace so the test suite still functions:

- Sweep `Tools/rbk/rbtd/` and test infrastructure for setup code creating these paths.
- Update fixture setup to create `rbmm_moorings/`-shaped trees.
- Mechanical but bulky.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- A representative tabtarget from each colophon family (rbw, buw, jjw, vvw) executes correctly.

## Risk

Highest-risk pace in the heat. Recovery is `git revert` of the wrap commit. Keep the commit pure rename + literal + alias-flip + fixture-paths.

## Sources

- ₣A_/₢A_AAJ docket — Renames table, Final layout
- `tt/z-launcher.sh` (created in ₢BKAAB) — literal to flip
- `Tools/rbk/rbcc_Constants.sh` — RBBC aliases established by ₢BKAAC
- `Tools/rbk/rbtd/` — test fixture surface

**[260513-1258] rough**

## Character

Hard cutover. The atomic move: filesystem rename + trampoline literal flip + test-fixture path updates, all in one commit.

## Goal

`rbmm_moorings/` exists with the new layout. Old `.buk/`, `.rbk/`, and `rbev-vessels/` directories are gone. Trampoline literal points at the new launcher path. All tabtargets still execute.

## Work

### File moves (use `git mv` so history follows)

Per AAJ Renames table:

- `.buk/launcher.*.sh` → `rbmm_moorings/rbml_launchers/launcher.*.sh` (drop the `_workbench` part per AAJ docket if applicable; preserve otherwise — verify against docket text)
- `.buk/users/` → `rbmm_moorings/rbmu_users/`
- `.buk/burc.env` → `rbmm_moorings/burc.env`
- `.rbk/rbrr.env` → `rbmm_moorings/rbrr.env`
- `.rbk/rbrp.env` → `rbmm_moorings/rbrp.env`
- `.rbk/<nameplate>/` → `rbmm_moorings/<nameplate>/` (each nameplate dir with its `rbrn.env` and `compose.yml`)
- `rbev-vessels/` → `rbmm_moorings/rbmv_vessels/`

`.rbk/rbob_compose.yml` and `Tools/rbk/rbmP_laterSecurityAudit.md` are pace 9 (kit-cleanup) — NOT this pace.

### Trampoline literal flip

Edit `tt/z-launcher.sh` to point at `../rbmm_moorings/rbml_launchers/launcher.${id}.sh` (or whatever exact form pace 2 settled on). Single literal edit.

### Test-fixture path updates

Any test fixture that creates `.buk/`, `.rbk/`, or `rbev-vessels/` paths in its setup must be updated in this same pace so the test suite still functions:

- Sweep `Tools/rbk/rbtd/` and test infrastructure for setup code creating these paths.
- Update fixture setup to create `rbmm_moorings/`-shaped trees.
- This is mechanical but bulky.

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- A representative tabtarget from each colophon family (rbw, buw, jjw, vvw) executes correctly.

## Risk

This is the highest-risk pace in the heat. Recovery is `git revert` of the wrap commit. Avoid mixing this pace's work with anything else — keep the commit pure rename + literal + fixture-paths.

## Sources

- ₣A_/₢A_AAJ docket — Renames table, Final layout
- `tt/z-launcher.sh` (created in ₢BKAAB) — literal to flip
- `Tools/rbk/rbtd/` — test fixture surface

### kit-literal-sweep (₢BKAAE) [complete]

**[260520-1118] complete**

## Character

Mechanical with light context judgment. Parallel-internal: two agents, one per kit subtree. Sweeps both literals and the transitional `RBBC_*` alias names; retires aliases at the end.

## Goal

No `.buk`, `.rbk`, or `rbev-vessels` literals survive in kit code under `Tools/{rbk,buk}/`. No `RBBC_*` references survive anywhere in kit code. RBCC's transitional aliases are removed. Every reference goes through a named RBCC or BUBC constant.

## Work

### Parallel dispatch

Spawn two sonnet agents working on disjoint trees:

- **rbk-agent** — sweep `Tools/rbk/**/*.sh` AND `Tools/rbk/rbtd/src/**/*.rs` (Rust files embed shell strings referencing kit vars — see `rbtdrf_fast.rs` for the canonical example):
  - Replace `.buk`, `.rbk`, `rbev-vessels` literals with RBCC constants minted in ₢BKAAC.
  - Replace `RBBC_*` references with their RBCC equivalents (`RBBC_dot_dir` → `RBCC_moorings_dir`, `RBBC_rbrr_file` → `RBCC_rbrr_file`, etc.).
  - Decide per-site which RBCC constant fits (the moorings dir alone, a subpath joined from constants, or a more specific named constant).
- **buk-agent** — sweep `Tools/buk/**/*.sh`:
  - Replace `.buk`, `.rbk`, `rbev-vessels` literals with BUBC constants from `bubc_constants.sh`. Mint additional BUBC constants if a needed concept is absent.

Both agents work on disjoint trees — no merge conflicts. Each agent's wrap criterion is its own grep returning empty for both classes of survivors.

### Selection discipline

- If a site uses `.buk/X` where X is a stable subpath, prefer a single composed reference (`"${RBCC_moorings_dir}/X"`) over minting a one-off constant.
- If a site uses `.buk/X` where X is a kit-defined identifier already constant-bound, compose via both constants (`"${RBCC_moorings_dir}/${RBCC_launchers_subdir}"`).
- If a literal recurs three or more times for the same concept, mint a constant rather than scattering composed references.

### RBCC alias retirement

After both agents wrap, remove the transitional `RBBC_*` aliases from RBCC (added in ₢BKAAC). If any `RBBC_*` references genuinely survive outside the swept subtrees, surface in the wrap commit — operator decides whether to keep the aliases or expand sweep scope.

### Survivors

After both agents wrap:
- `grep -rE '\.(buk|rbk)|rbev-vessels' Tools/{rbk,buk}/` returns only constant-definition sites and documented exceptions.
- `grep -rn 'RBBC_' Tools/` returns nothing (or only documented exceptions).

If literals or `RBBC_*` names genuinely survive after the sweep, that's the docket's open RBK-locale question answering itself empirically: surface the count and pattern in the wrap commit message — ₢BKAAI may need to register a finding.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Both grep families return expected sites only.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — RBCC constants minted in ₢BKAAC, alias-retire site
- `Tools/buk/bubc_constants.sh` — BUBC constants
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — canonical example of Rust-embedded shell referencing kit vars
- ₣A_/₢A_AAJ docket — Symbolic-constant discipline section

**[260514-1206] rough**

## Character

Mechanical with light context judgment. Parallel-internal: two agents, one per kit subtree. Sweeps both literals and the transitional `RBBC_*` alias names; retires aliases at the end.

## Goal

No `.buk`, `.rbk`, or `rbev-vessels` literals survive in kit code under `Tools/{rbk,buk}/`. No `RBBC_*` references survive anywhere in kit code. RBCC's transitional aliases are removed. Every reference goes through a named RBCC or BUBC constant.

## Work

### Parallel dispatch

Spawn two sonnet agents working on disjoint trees:

- **rbk-agent** — sweep `Tools/rbk/**/*.sh` AND `Tools/rbk/rbtd/src/**/*.rs` (Rust files embed shell strings referencing kit vars — see `rbtdrf_fast.rs` for the canonical example):
  - Replace `.buk`, `.rbk`, `rbev-vessels` literals with RBCC constants minted in ₢BKAAC.
  - Replace `RBBC_*` references with their RBCC equivalents (`RBBC_dot_dir` → `RBCC_moorings_dir`, `RBBC_rbrr_file` → `RBCC_rbrr_file`, etc.).
  - Decide per-site which RBCC constant fits (the moorings dir alone, a subpath joined from constants, or a more specific named constant).
- **buk-agent** — sweep `Tools/buk/**/*.sh`:
  - Replace `.buk`, `.rbk`, `rbev-vessels` literals with BUBC constants from `bubc_constants.sh`. Mint additional BUBC constants if a needed concept is absent.

Both agents work on disjoint trees — no merge conflicts. Each agent's wrap criterion is its own grep returning empty for both classes of survivors.

### Selection discipline

- If a site uses `.buk/X` where X is a stable subpath, prefer a single composed reference (`"${RBCC_moorings_dir}/X"`) over minting a one-off constant.
- If a site uses `.buk/X` where X is a kit-defined identifier already constant-bound, compose via both constants (`"${RBCC_moorings_dir}/${RBCC_launchers_subdir}"`).
- If a literal recurs three or more times for the same concept, mint a constant rather than scattering composed references.

### RBCC alias retirement

After both agents wrap, remove the transitional `RBBC_*` aliases from RBCC (added in ₢BKAAC). If any `RBBC_*` references genuinely survive outside the swept subtrees, surface in the wrap commit — operator decides whether to keep the aliases or expand sweep scope.

### Survivors

After both agents wrap:
- `grep -rE '\.(buk|rbk)|rbev-vessels' Tools/{rbk,buk}/` returns only constant-definition sites and documented exceptions.
- `grep -rn 'RBBC_' Tools/` returns nothing (or only documented exceptions).

If literals or `RBBC_*` names genuinely survive after the sweep, that's the docket's open RBK-locale question answering itself empirically: surface the count and pattern in the wrap commit message — ₢BKAAI may need to register a finding.

## Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Both grep families return expected sites only.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — RBCC constants minted in ₢BKAAC, alias-retire site
- `Tools/buk/bubc_constants.sh` — BUBC constants
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — canonical example of Rust-embedded shell referencing kit vars
- ₣A_/₢A_AAJ docket — Symbolic-constant discipline section

**[260513-1258] rough**

## Character

Mechanical with light context judgment. Parallel-internal: two agents, one per kit subtree.

## Goal

No string literals matching `.buk`, `.rbk`, or `rbev-vessels` survive in kit code under `Tools/{rbk,buk}/*.sh`. Every reference goes through a named constant from RBCC or BUBC.

## Work

### Parallel dispatch

Spawn two sonnet agents:

- **rbk-agent** — sweep `Tools/rbk/**/*.sh`, replace literals with RBCC constants minted in ₢BKAAC. Decide per-site which RBCC constant fits (the moorings dir alone, a subpath joined from constants, or a more specific named constant if the locale calls for one).
- **buk-agent** — sweep `Tools/buk/**/*.sh`, replace literals with BUBC constants from `bubc_constants.sh`. Mint additional BUBC constants if a needed concept is absent.

Both agents work on disjoint file trees — no merge conflicts. Each agent's wrap criterion is its own grep returning empty for the literals.

### Selection discipline

- If a site uses `.buk/X` where X is a stable subpath, prefer a single composed reference (`"${RBCC_moorings_dir}/X"`) over minting a one-off constant.
- If a site uses `.buk/X` where X is a kit-defined identifier already constant-bound, compose via both constants (`"${RBCC_moorings_dir}/${RBCC_launchers_subdir}"`).
- If a literal recurs three or more times for the same concept, mint a constant rather than scattering composed references.

### Survivors

After both agents wrap, `grep -rE '\.(buk|rbk)|rbev-vessels' Tools/{rbk,buk}/*.sh` should return only the constant-definition sites themselves and any documented exceptions.

If RBK literals genuinely survive after the sweep, that's the docket's open RBK-locale question answering itself empirically: surface the count and pattern in the wrap commit message — pace 9 may need to register a finding.

### Wrap criteria

- `tt/rbtd-s.TestSuite.fast.sh` passes.
- Literal grep returns expected sites only.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — RBCC constants minted in ₢BKAAC
- `Tools/buk/bubc_constants.sh` — BUBC constants
- ₣A_/₢A_AAJ docket — Symbolic-constant discipline section

### marshal-zero-and-proof-regen (₢BKAAF) [complete]

**[260520-1124] complete**

## Character

Update Marshal so the lifecycle of new consumers + release proof recognizes the new layout.

## Goal

`tt/rbw-MZ.MarshalZeroes` produces a clean blank tree with `rbmm_moorings/` (not `.buk/`/`.rbk/`). `tt/rbw-MP.MarshalProofs` validates against new layout and completes cleanly.

## Work

### Zero template regen

- Read `Tools/rbk/rblm_cli.sh` (Marshal Lifecycle).
- Find where it generates `.buk/` and `.rbk/` directory structures for fresh consumers.
- Update generation to produce the `rbmm_moorings/` layout per AAJ Final layout section.
- Reference RBCC constants from ₢BKAAC where they exist; literal path fragments fed back through constants.

### Proof workflow validation

- The Marshal proof repo workflow tests a real release into a blank tree.
- If proof creates or expects old paths in its scaffolding, update.
- Run `tt/rbw-MP.MarshalProofs.sh` end-to-end; must complete.

### Why this is one pace, not two

Zero template and proof workflow are tightly coupled — Marshal proof exercises what Marshal zero produces. Splitting risks a green proof that doesn't actually exercise the new layout. Single pace keeps them in lockstep.

### Wrap criteria

- `tt/rbw-MZ.MarshalZeroes.sh` produces an `rbmm_moorings/` tree.
- `tt/rbw-MP.MarshalProofs.sh` completes.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Risk note

Bugs here have downstream blast radius into ₣BB (rbk-16-mvp-release-qualification, 48/57 done in muster) — verify proof workflow before wrap.

## Sources

- `Tools/rbk/rblm_cli.sh` — Marshal CLI
- `tt/rbw-MZ.MarshalZeroes.sh`, `tt/rbw-MP.MarshalProofs.sh` — entry points
- ₣A_/₢A_AAJ docket — Final layout section

**[260514-1359] rough**

## Character

Update Marshal so the lifecycle of new consumers + release proof recognizes the new layout.

## Goal

`tt/rbw-MZ.MarshalZeroes` produces a clean blank tree with `rbmm_moorings/` (not `.buk/`/`.rbk/`). `tt/rbw-MP.MarshalProofs` validates against new layout and completes cleanly.

## Work

### Zero template regen

- Read `Tools/rbk/rblm_cli.sh` (Marshal Lifecycle).
- Find where it generates `.buk/` and `.rbk/` directory structures for fresh consumers.
- Update generation to produce the `rbmm_moorings/` layout per AAJ Final layout section.
- Reference RBCC constants from ₢BKAAC where they exist; literal path fragments fed back through constants.

### Proof workflow validation

- The Marshal proof repo workflow tests a real release into a blank tree.
- If proof creates or expects old paths in its scaffolding, update.
- Run `tt/rbw-MP.MarshalProofs.sh` end-to-end; must complete.

### Why this is one pace, not two

Zero template and proof workflow are tightly coupled — Marshal proof exercises what Marshal zero produces. Splitting risks a green proof that doesn't actually exercise the new layout. Single pace keeps them in lockstep.

### Wrap criteria

- `tt/rbw-MZ.MarshalZeroes.sh` produces an `rbmm_moorings/` tree.
- `tt/rbw-MP.MarshalProofs.sh` completes.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Risk note

Bugs here have downstream blast radius into ₣BB (rbk-16-mvp-release-qualification, 48/57 done in muster) — verify proof workflow before wrap.

## Sources

- `Tools/rbk/rblm_cli.sh` — Marshal CLI
- `tt/rbw-MZ.MarshalZeroes.sh`, `tt/rbw-MP.MarshalProofs.sh` — entry points
- ₣A_/₢A_AAJ docket — Final layout section

**[260513-1259] rough**

## Character

Update Marshal so the lifecycle of new consumers + release proof recognizes the new layout.

## Goal

`tt/rbw-MZ.MarshalZeroes` produces a clean blank tree with `rbmm_moorings/` (not `.buk/`/`.rbk/`). `tt/rbw-MP.MarshalProofs` validates against new layout and completes cleanly.

## Work

### Zero template regen

- Read `Tools/rbk/rblm_cli.sh` (Marshal Lifecycle).
- Find where it generates `.buk/` and `.rbk/` directory structures for fresh consumers.
- Update generation to produce the `rbmm_moorings/` layout per AAJ Final layout section.
- Reference RBCC constants from ₢BKAAC where they exist; literal path fragments fed back through constants.

### Proof workflow validation

- The Marshal proof repo workflow tests a real release into a blank tree.
- If proof creates or expects old paths in its scaffolding, update.
- Run `tt/rbw-MP.MarshalProofs.sh` end-to-end; must complete.

### Why this is one pace, not two

Zero template and proof workflow are tightly coupled — Marshal proof exercises what Marshal zero produces. Splitting risks a green proof that doesn't actually exercise the new layout. Single pace keeps them in lockstep.

### Wrap criteria

- `tt/rbw-MZ.MarshalZeroes.sh` produces an `rbmm_moorings/` tree.
- `tt/rbw-MP.MarshalProofs.sh` completes.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Risk note

Bugs here have downstream blast radius into ₣BB (rbk-15-mvp-release-qualification, 48/57 done in muster) — verify proof workflow before wrap.

## Sources

- `Tools/rbk/rblm_cli.sh` — Marshal CLI
- `tt/rbw-MZ.MarshalZeroes.sh`, `tt/rbw-MP.MarshalProofs.sh` — entry points
- ₣A_/₢A_AAJ docket — Final layout section

### bcg-tabtarget-path-indirection-section (₢BKAAG) [complete]

**[260520-1131] complete**

## Character

Codification. Prose largely drafted in AAJ docket; integrate into BCG's voice and structure.

## Goal

BCG carries a "Tabtarget Path Indirection" subsection peer to "Dispatch-Provided Directory Variables." Future readers learn the trampoline pattern from BCG rather than tribal knowledge.

## Work

- Locate the "Dispatch-Provided Directory Variables" section in `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- Add a peer subsection titled "Tabtarget Path Indirection."
- Content based on AAJ docket's BCG codification section. Adapt for BCG's voice; align with BCG's CLI-as-Module-Gateway principle (sections 44-77).
- Key teaching points:
  - Single tabtarget (`tt/z-launcher.sh`) resolves moorings location.
  - Trampoline normalizes cwd to repo root before launcher dispatch.
  - All other tabtargets invoke the trampoline with a minted moorings-launcher *sprue* (`{owner}ml_{launcher-id}` — `rbml_*` for RBK-authored launchers, `buml_*` for the BUK launcher infra hosting other kits), never a bare colophon. The trampoline strips the `*ml_` prefix to recover the launcher-id.
  - The sprue keeps launcher dispatch tokens in the underscore-shaped Primary Universe, distinct from the hyphenated colophon universe — a name collision the bare-workbench-id form would have invited.
  - Moorings location is single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer).
  - Workbenches start with deterministic cwd regardless of user invocation directory.

### Wrap criteria

- BCG renders cleanly.
- Cross-references to related BCG sections (load-bearing complexity, CLI as module gateway, dispatch-provided BURD_*) are coherent.
- `tt/rbtd-s.TestSuite.fast.sh` passes (defensive — should be a no-op for doc-only changes).

## Sources

- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — target file
- ₣BK paddock — "Tabtarget dispatch sprue" / "`buml_` prefix family" locked decisions
- ₣A_/₢A_AAJ docket — BCG codification section, prose draft

**[260519-1616] rough**

## Character

Codification. Prose largely drafted in AAJ docket; integrate into BCG's voice and structure.

## Goal

BCG carries a "Tabtarget Path Indirection" subsection peer to "Dispatch-Provided Directory Variables." Future readers learn the trampoline pattern from BCG rather than tribal knowledge.

## Work

- Locate the "Dispatch-Provided Directory Variables" section in `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- Add a peer subsection titled "Tabtarget Path Indirection."
- Content based on AAJ docket's BCG codification section. Adapt for BCG's voice; align with BCG's CLI-as-Module-Gateway principle (sections 44-77).
- Key teaching points:
  - Single tabtarget (`tt/z-launcher.sh`) resolves moorings location.
  - Trampoline normalizes cwd to repo root before launcher dispatch.
  - All other tabtargets invoke the trampoline with a minted moorings-launcher *sprue* (`{owner}ml_{launcher-id}` — `rbml_*` for RBK-authored launchers, `buml_*` for the BUK launcher infra hosting other kits), never a bare colophon. The trampoline strips the `*ml_` prefix to recover the launcher-id.
  - The sprue keeps launcher dispatch tokens in the underscore-shaped Primary Universe, distinct from the hyphenated colophon universe — a name collision the bare-workbench-id form would have invited.
  - Moorings location is single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer).
  - Workbenches start with deterministic cwd regardless of user invocation directory.

### Wrap criteria

- BCG renders cleanly.
- Cross-references to related BCG sections (load-bearing complexity, CLI as module gateway, dispatch-provided BURD_*) are coherent.
- `tt/rbtd-s.TestSuite.fast.sh` passes (defensive — should be a no-op for doc-only changes).

## Sources

- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — target file
- ₣BK paddock — "Tabtarget dispatch sprue" / "`buml_` prefix family" locked decisions
- ₣A_/₢A_AAJ docket — BCG codification section, prose draft

**[260513-1259] rough**

## Character

Codification. Prose largely drafted in AAJ docket; integrate into BCG's voice and structure.

## Goal

BCG carries a "Tabtarget Path Indirection" subsection peer to "Dispatch-Provided Directory Variables." Future readers learn the trampoline pattern from BCG rather than tribal knowledge.

## Work

- Locate the "Dispatch-Provided Directory Variables" section in `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`.
- Add a peer subsection titled "Tabtarget Path Indirection."
- Content based on AAJ docket's BCG codification section. Adapt for BCG's voice; align with BCG's CLI-as-Module-Gateway principle (sections 44-77).
- Key teaching points:
  - Single tabtarget (`tt/z-launcher.sh`) resolves moorings location.
  - Trampoline normalizes cwd to repo root before launcher dispatch.
  - All other tabtargets invoke trampoline with a workbench identifier.
  - Moorings location is single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer).
  - Workbenches start with deterministic cwd regardless of user invocation directory.

### Wrap criteria

- BCG renders cleanly.
- Cross-references to related BCG sections (load-bearing complexity, CLI as module gateway, dispatch-provided BURD_*) are coherent.
- `tt/rbtd-s.TestSuite.fast.sh` passes (defensive — should be a no-op for doc-only changes).

## Sources

- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — target file
- ₣A_/₢A_AAJ docket — BCG codification section, prose draft

### docs-vocabulary-sweep (₢BKAAH) [complete]

**[260520-1259] complete**

## Character

Mechanical with linked-term subtlety. Parallel-internal: two agents, one per doc subtree.

## Goal

No document teaches `.buk/`, `.rbk/`, or `rbev-vessels/` to a reader. AsciiDoc linked terms reflect the new vocabulary coherently.

## Work

### Parallel dispatch

Spawn two sonnet agents working on disjoint doc subtrees:

- **handbook-agent** — onboarding tracks and human-facing handbook content:
  - `Tools/rbk/rbh0/` (rbho/rbhp/rbhw families — onboarding CLIs)
  - Any handbook display strings, probe outputs, learner-facing examples
  - Tabtargets surfaced in onboarding flows (`rbw-O*` series)

- **specs-agent** — specifications and meta-docs:
  - `Tools/rbk/vov_veiled/RBS*.adoc` family (start with RBS0, RBRN, RBSRR, RBSRP, RBSRV)
  - `README.md`, `CLAUDE.md`
  - Consumer CLAUDE: `Tools/rbk/vov_veiled/CLAUDE.consumer.md`
  - AsciiDoc linked terms — attribute references, anchors, replacement text — must stay coherent. A linked term whose definition mentions the old path needs both the definition text AND any anchor/attribute name updated if those embed the old vocabulary.

### Substitution rules (both agents)

- `.buk/` → `rbmm_moorings/` (subdirs determine specific final path)
- `.rbk/` → `rbmm_moorings/` (subdirs determine specific final path)
- `rbev-vessels/` → `rbmm_moorings/rbmv_vessels/`
- `.buk/users/` → `rbmm_moorings/rbmu_users/`
- `.buk/launcher.*` → `rbmm_moorings/rbml_launchers/launcher.*`

Where prose teaches the directory structure as a concept (not just a path), update both the path and any prose that describes the old structure (e.g., "consumer config under `.buk/`" → "consumer config under `rbmm_moorings/`").

### Coherence

Two agents on disjoint trees, but both applying the same substitution rules. Substitution rule consistency is the docket's contract — if either agent invents a different substitution, both wraps need to align before this pace wraps.

### Wrap criteria

- `grep -rE '\.(buk|rbk)/|rbev-vessels' Tools/rbk/vov_veiled/ Tools/rbk/rbh0/ README.md CLAUDE.md` returns empty (or only documented exceptions).
- Spot-render an onboarding track (`tt/rbw-Occ.OnboardingConfigureEnvironment.sh` etc.) and read through; vocabulary reads natural.
- AsciiDoc linked terms resolve.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Sources

- `Tools/rbk/rbh0/` — handbook surface
- `Tools/rbk/vov_veiled/RBS*.adoc` — spec surface
- ₣A_/₢A_AAJ docket — Handbook and specs section

**[260513-1259] rough**

## Character

Mechanical with linked-term subtlety. Parallel-internal: two agents, one per doc subtree.

## Goal

No document teaches `.buk/`, `.rbk/`, or `rbev-vessels/` to a reader. AsciiDoc linked terms reflect the new vocabulary coherently.

## Work

### Parallel dispatch

Spawn two sonnet agents working on disjoint doc subtrees:

- **handbook-agent** — onboarding tracks and human-facing handbook content:
  - `Tools/rbk/rbh0/` (rbho/rbhp/rbhw families — onboarding CLIs)
  - Any handbook display strings, probe outputs, learner-facing examples
  - Tabtargets surfaced in onboarding flows (`rbw-O*` series)

- **specs-agent** — specifications and meta-docs:
  - `Tools/rbk/vov_veiled/RBS*.adoc` family (start with RBS0, RBRN, RBSRR, RBSRP, RBSRV)
  - `README.md`, `CLAUDE.md`
  - Consumer CLAUDE: `Tools/rbk/vov_veiled/CLAUDE.consumer.md`
  - AsciiDoc linked terms — attribute references, anchors, replacement text — must stay coherent. A linked term whose definition mentions the old path needs both the definition text AND any anchor/attribute name updated if those embed the old vocabulary.

### Substitution rules (both agents)

- `.buk/` → `rbmm_moorings/` (subdirs determine specific final path)
- `.rbk/` → `rbmm_moorings/` (subdirs determine specific final path)
- `rbev-vessels/` → `rbmm_moorings/rbmv_vessels/`
- `.buk/users/` → `rbmm_moorings/rbmu_users/`
- `.buk/launcher.*` → `rbmm_moorings/rbml_launchers/launcher.*`

Where prose teaches the directory structure as a concept (not just a path), update both the path and any prose that describes the old structure (e.g., "consumer config under `.buk/`" → "consumer config under `rbmm_moorings/`").

### Coherence

Two agents on disjoint trees, but both applying the same substitution rules. Substitution rule consistency is the docket's contract — if either agent invents a different substitution, both wraps need to align before this pace wraps.

### Wrap criteria

- `grep -rE '\.(buk|rbk)/|rbev-vessels' Tools/rbk/vov_veiled/ Tools/rbk/rbh0/ README.md CLAUDE.md` returns empty (or only documented exceptions).
- Spot-render an onboarding track (`tt/rbw-Occ.OnboardingConfigureEnvironment.sh` etc.) and read through; vocabulary reads natural.
- AsciiDoc linked terms resolve.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Sources

- `Tools/rbk/rbh0/` — handbook surface
- `Tools/rbk/vov_veiled/RBS*.adoc` — spec surface
- ₣A_/₢A_AAJ docket — Handbook and specs section

### kit-cleanup (₢BKAAI) [complete]

**[260520-1304] complete**

## Character

Tidy. Mechanical file moves + CLAUDE.md updates.

## Goal

Stranded items relocated to their proper homes. Acronym registry reflects the new `rbm*_` and `buml_` families and the dispatch-sprue scheme.

## Work

### File relocations (use `git mv`)

- `Tools/rbk/rbmP_laterSecurityAudit.md` → `Memos/`. Orphan memo; reclaims `rbm*` namespace for the moorings family.
- `.rbk/rbob_compose.yml` → `Tools/rbk/rbob_compose.yml`. Kit machinery (crucible topology), not consumer-tunable. (If the rename pace already moved this, verify and skip.)

### CLAUDE.md acronym registration

Register the moorings naming in the repo-root `CLAUDE.md` File Acronym Mappings. Under the RBK subdirectory section, the `rbm*_` family:

- `rbmm_` — Moorings umbrella (the directory itself)
- `rbml_` — Moorings launchers (the shared launcher directory; holds every kit's launcher file)
- `rbmn_` — Moorings nodes (remote BURN profiles)
- `rbmu_` — Moorings users (remote profiles)
- `rbmv_` — Moorings vessels

Alongside (BUK-side, near the `bubc_` neighborhood), the `buml_` family:

- `buml_` — BUK moorings launchers; the dispatch-sprue namespace for every non-RBK kit's launcher.

Register the **dispatch-sprue scheme** itself where the tabtarget/colophon discipline is described: sprues take the form `{owner}ml_{launcher-id}` (`rbml_*` RBK-authored, `buml_*` BUK-hosted), are the tabtarget→trampoline dispatch tokens, and live in the underscore-shaped Primary Universe distinct from hyphenated colophons.

### Wrap criteria

- Both files at new locations; old locations gone.
- CLAUDE.md acronym table includes the `rbm*_` family, the `buml_` family, and the sprue scheme.
- No references to old locations survive in code (handled by ₢BKAAE) or docs (handled by ₢BKAAH); if any do, they're bugs to file, not bugs to fix in this pace.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Sources

- `Tools/rbk/rbmP_laterSecurityAudit.md` — orphan memo
- `.rbk/rbob_compose.yml` (or its moorings location if the rename pace staged it there)
- `CLAUDE.md` (repo root) — acronym registry
- ₣BK paddock — `rbm*_` / `buml_` family + dispatch-sprue locked decisions
- ₣A_/₢A_AAJ docket — Cleanup section

**[260519-1616] rough**

## Character

Tidy. Mechanical file moves + CLAUDE.md updates.

## Goal

Stranded items relocated to their proper homes. Acronym registry reflects the new `rbm*_` and `buml_` families and the dispatch-sprue scheme.

## Work

### File relocations (use `git mv`)

- `Tools/rbk/rbmP_laterSecurityAudit.md` → `Memos/`. Orphan memo; reclaims `rbm*` namespace for the moorings family.
- `.rbk/rbob_compose.yml` → `Tools/rbk/rbob_compose.yml`. Kit machinery (crucible topology), not consumer-tunable. (If the rename pace already moved this, verify and skip.)

### CLAUDE.md acronym registration

Register the moorings naming in the repo-root `CLAUDE.md` File Acronym Mappings. Under the RBK subdirectory section, the `rbm*_` family:

- `rbmm_` — Moorings umbrella (the directory itself)
- `rbml_` — Moorings launchers (the shared launcher directory; holds every kit's launcher file)
- `rbmn_` — Moorings nodes (remote BURN profiles)
- `rbmu_` — Moorings users (remote profiles)
- `rbmv_` — Moorings vessels

Alongside (BUK-side, near the `bubc_` neighborhood), the `buml_` family:

- `buml_` — BUK moorings launchers; the dispatch-sprue namespace for every non-RBK kit's launcher.

Register the **dispatch-sprue scheme** itself where the tabtarget/colophon discipline is described: sprues take the form `{owner}ml_{launcher-id}` (`rbml_*` RBK-authored, `buml_*` BUK-hosted), are the tabtarget→trampoline dispatch tokens, and live in the underscore-shaped Primary Universe distinct from hyphenated colophons.

### Wrap criteria

- Both files at new locations; old locations gone.
- CLAUDE.md acronym table includes the `rbm*_` family, the `buml_` family, and the sprue scheme.
- No references to old locations survive in code (handled by ₢BKAAE) or docs (handled by ₢BKAAH); if any do, they're bugs to file, not bugs to fix in this pace.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Sources

- `Tools/rbk/rbmP_laterSecurityAudit.md` — orphan memo
- `.rbk/rbob_compose.yml` (or its moorings location if the rename pace staged it there)
- `CLAUDE.md` (repo root) — acronym registry
- ₣BK paddock — `rbm*_` / `buml_` family + dispatch-sprue locked decisions
- ₣A_/₢A_AAJ docket — Cleanup section

**[260513-1259] rough**

## Character

Tidy. Mechanical file moves + CLAUDE.md updates.

## Goal

Stranded items relocated to their proper homes. Acronym registry reflects the new `rbm*_` family.

## Work

### File relocations (use `git mv`)

- `Tools/rbk/rbmP_laterSecurityAudit.md` → `Memos/`. Orphan memo; reclaims `rbm*` namespace for the moorings family.
- `.rbk/rbob_compose.yml` → `Tools/rbk/rbob_compose.yml`. Kit machinery (crucible topology), not consumer-tunable. (If pace 4 already moved this as part of the rename, verify and skip.)

### CLAUDE.md acronym registration

Register the `rbm*_` family in `/Users/bhyslop/projects/rbm_alpha_recipemuster/CLAUDE.md` File Acronym Mappings under the RBK subdirectory section:

- `rbmm_` — Moorings umbrella (the directory itself)
- `rbml_` — Moorings launchers
- `rbmu_` — Moorings users (remote profiles)
- `rbmv_` — Moorings vessels
- Plus `rbmn_` per ₢BKAAA outcome if it joined the family

### Wrap criteria

- Both files at new locations; old locations gone.
- CLAUDE.md acronym table includes the family.
- No references to old locations survive in code (handled by ₢BKAAE) or docs (handled by ₢BKAAH); if any do, they're bugs to file, not bugs to fix in this pace.
- `tt/rbtd-s.TestSuite.fast.sh` passes.

## Sources

- `Tools/rbk/rbmP_laterSecurityAudit.md` — orphan memo
- `.rbk/rbob_compose.yml` (or `rbmm_moorings/rbob_compose.yml` if pace 4 staged it under moorings)
- `CLAUDE.md` — acronym registry
- ₣A_/₢A_AAJ docket — Cleanup section

### moorings-acceptance (₢BKAAJ) [complete]

**[260520-1937] complete**

## Character

Two-platform gauntlet exit. Operator-driven; failure triage is opus-grade — a failure here is a real cutover bug, not exploration.

## Goal

`tt/rbw-tP.QualifyPristine.sh` passes on macOS and linux against the new layout.

## Work

Run sequentially, never concurrent:

- macOS branch off the ₣BK heat branch: `tt/rbw-tP.QualifyPristine.sh`.
- linux branch off the ₣BK heat branch: `tt/rbw-tP.QualifyPristine.sh`.

The gauntlet runs canonical-establish (depot regen) + qualification in one pass. Per-platform branches keep canonical-depot commits isolated.

### Triage on failure

- **Grammar bug** (literal that escaped the sweep, wrong constant) — fix inline, re-run.
- **Contract bug** (cwd assumption that escaped the audit, layout assumption in untested code path) — fix inline if mechanical, spin off a pace if architectural.
- **Genuine architectural surprise** — stop, surface, decide. The cutover is wide enough that something subtle could surface only under the full gauntlet.

### Wrap criteria

- Both platform runs green.
- Any inline fixes are themselves committed as `jjx_record` entries against this pace.

## Sources

- `Tools/rbk/rbq_Qualify.sh` — pristine entry point
- `Tools/rbk/rbtd/rbte_engine.sh` — gauntlet suite contents
- `Tools/rbk/rbtd/src/rbtdrk_canonical.rs` — canonical-establish performs the depot regen
- ₣A_/₢A_AAP — template pace pattern for two-platform acceptance

**[260513-1300] rough**

## Character

Two-platform gauntlet exit. Operator-driven; failure triage is opus-grade — a failure here is a real cutover bug, not exploration.

## Goal

`tt/rbw-tP.QualifyPristine.sh` passes on macOS and linux against the new layout.

## Work

Run sequentially, never concurrent:

- macOS branch off the ₣BK heat branch: `tt/rbw-tP.QualifyPristine.sh`.
- linux branch off the ₣BK heat branch: `tt/rbw-tP.QualifyPristine.sh`.

The gauntlet runs canonical-establish (depot regen) + qualification in one pass. Per-platform branches keep canonical-depot commits isolated.

### Triage on failure

- **Grammar bug** (literal that escaped the sweep, wrong constant) — fix inline, re-run.
- **Contract bug** (cwd assumption that escaped the audit, layout assumption in untested code path) — fix inline if mechanical, spin off a pace if architectural.
- **Genuine architectural surprise** — stop, surface, decide. The cutover is wide enough that something subtle could surface only under the full gauntlet.

### Wrap criteria

- Both platform runs green.
- Any inline fixes are themselves committed as `jjx_record` entries against this pace.

## Sources

- `Tools/rbk/rbq_Qualify.sh` — pristine entry point
- `Tools/rbk/rbtd/rbte_engine.sh` — gauntlet suite contents
- `Tools/rbk/rbtd/src/rbtdrk_canonical.rs` — canonical-establish performs the depot regen
- ₣A_/₢A_AAP — template pace pattern for two-platform acceptance

### charge-subnet-keyed-reclaim (₢BKAAK) [complete]

**[260520-1951] complete**

## Character

Architectural simplification — a clear target shape, but the invariant reasoning must survive the edit. Standard dev.

## Goal

Collapse `rbob_charge`'s two-stage prior-state cleanup (own-project compose-down + foreign-collision refusal) into a single subnet-keyed reclaim: charge frees the nameplate's subnet by tearing down whatever occupies it, regardless of prefix. No own-vs-foreign branch, no project-label string-parsing to infer lineage.

## Why this is safe to simplify

A crucible's subnet derives from the nameplate (`RBRN_ENCLAVE_BASE_IP`), not the prefix — so two same-nameplate crucibles can never coexist on one daemon. The foreign-collision branch therefore guarded an impossible state; its only real effect was to *refuse* instead of *reclaim* when a leftover carried a different prefix. That is exactly the orphan that stalled the macOS ₢BKAAJ gauntlet: `canrbhl-srjcl_{enclave,transit}` networks from a run killed mid-srjcl ~5 days prior (depot `canest2bhl100009`), whose quench never executed. Reclaiming the slot is just Stage 1's existing force-takeover extended to all prefixes. Match on the subnet (a structural fact), not on inferred name lineage.

## Sources

- `Tools/rbk/rbob_bottle.sh` — `zrbob_detect_nameplate_collision`, `zrbob_detect_subnet_conflict`, and the `rbob_charge` cleanup step that calls both

**[260520-1935] rough**

## Character

Architectural simplification — a clear target shape, but the invariant reasoning must survive the edit. Standard dev.

## Goal

Collapse `rbob_charge`'s two-stage prior-state cleanup (own-project compose-down + foreign-collision refusal) into a single subnet-keyed reclaim: charge frees the nameplate's subnet by tearing down whatever occupies it, regardless of prefix. No own-vs-foreign branch, no project-label string-parsing to infer lineage.

## Why this is safe to simplify

A crucible's subnet derives from the nameplate (`RBRN_ENCLAVE_BASE_IP`), not the prefix — so two same-nameplate crucibles can never coexist on one daemon. The foreign-collision branch therefore guarded an impossible state; its only real effect was to *refuse* instead of *reclaim* when a leftover carried a different prefix. That is exactly the orphan that stalled the macOS ₢BKAAJ gauntlet: `canrbhl-srjcl_{enclave,transit}` networks from a run killed mid-srjcl ~5 days prior (depot `canest2bhl100009`), whose quench never executed. Reclaiming the slot is just Stage 1's existing force-takeover extended to all prefixes. Match on the subnet (a structural fact), not on inferred name lineage.

## Sources

- `Tools/rbk/rbob_bottle.sh` — `zrbob_detect_nameplate_collision`, `zrbob_detect_subnet_conflict`, and the `rbob_charge` cleanup step that calls both

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A bubc-rbmn-prefix-reconciliation
  2 B z-launcher-trampoline-introduce
  3 C rbcc-establish-fact-locale
  4 D moorings-filesystem-rename
  5 E kit-literal-sweep
  6 F marshal-zero-and-proof-regen
  7 G bcg-tabtarget-path-indirection-section
  8 H docs-vocabulary-sweep
  9 I kit-cleanup
  10 J moorings-acceptance
  11 K charge-subnet-keyed-reclaim

ABCDEFGHIJK
···xx·····x rbob_bottle.sh
··xxx······ rbcc_Constants.sh, rblm_cli.sh
····x····x· rbndb_base.sh
····x·x···· BCG-BashConsoleGuide.md
···xx······ rbtdrf_fast.rs
··x·x······ rbfc_cli.sh, rbfd_cli.sh, rbfh_cli.sh, rbfk_cli.sh, rbfl_cli.sh, rbfr_cli.sh, rbfv_cli.sh, rbgg_cli.sh, rbgp_cli.sh, rbho0_cli.sh, rbhp0_cli.sh, rbob_cli.sh, rbra_cli.sh, rbrd_cli.sh, rbrn_cli.sh, rbro_cli.sh, rbrp_cli.sh, rbrr_cli.sh, rbrv_cli.sh, rbv_cli.sh
··xx······· rbtdrp_pristine.rs
·x·x······· bul_launcher.sh, buq_qualify.sh, buut_tabtarget.sh, rbq_Qualify.sh, z-launcher.sh
··········x RBSCC-crucible_charge.adoc
·······x··· CLAUDE.consumer.md, CLAUDE.md, RBRN-RegimeNameplate.adoc, RBS0-SpecTop.adoc, RBSAE-ark_enshrine.adoc, RBSAG-ark_graft.adoc, RBSAO-access_oauth_probe.adoc, RBSAV-ark_vouch.adoc, RBSDE-depot_levy.adoc, RBSDI-depot_inscribe.adoc, RBSDU-depot_unmake.adoc, RBSDY-director_yoke.adoc, RBSHR-HorizonRoadmap.adoc, RBSIP-ifrit_pentester.adoc, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc, RBSPV-PodmanVmSupplyChain.adoc, RBSRM-RegimeMachine.adoc, RBSRP-RegimePayor.adoc, RBSRR-RegimeRepo.adoc, RBSRT-RegimeDepot.adoc, RBSRV-RegimeVessel.adoc, RBSTB-trigger_build.adoc, README.md
····x······ burd_regime.sh, burn_cli.sh, burn_regime.sh, burp_cli.sh, burp_regime.sh, burs_regime.sh, butcrg_RegimeCredentials.sh, rbfc_FoundryCore.sh, rbgc_Constants.sh, rbhob_base.sh, rbhocc_crash_course.sh, rbhoda_director_airgap.sh, rbhodb_director_bind.sh, rbhodf_director_first_build.sh, rbhodg_director_graft.sh, rbhofc_first_crucible.sh, rbhots_tadmor_security.sh, rbhpb_base.sh, rbrn_regime.sh, rbte_cli.sh, rbte_engine.sh, rbv_PodmanVM.sh
···x······· bubc_constants.sh, buut_cli.sh, buw-SI.StationInit.sh, dockerfile-ifrit-setcap.patch, launcher.apcw_workbench.sh, launcher.buw_workbench.sh, launcher.cmw_workbench.sh, launcher.jjw_workbench.sh, launcher.rbtw_workbench.sh, launcher.rbw_workbench.sh, launcher.study_workbench.sh, launcher.vow_workbench.sh, launcher.vslw_workbench.sh, launcher.vvw_workbench.sh, lib.rs, rbob_compose.yml, rbrn.env, rbrr.env, rbtdrc_crucible.rs, rbtdrk_canonical.rs, rbtdro_onboarding.rs, rbtdtk_canonical.rs, rbtdtp_pristine.rs, vob_build.sh, vob_cli.sh, vslw_workbench.sh, vvi_install.sh, vvu_uninstall.sh
··x········ rbbc_constants.sh, rbhw0_cli.sh, rbq_cli.sh, rbrs_cli.sh
·x········· apcw-D.Deploy.sh, apcw-b.Build.sh, apcw-ba.BatchAssay.sh, apcw-cb.ContainerBuild.sh, apcw-ci.ContainerStatus.sh, apcw-cs.ContainerStart.sh, apcw-cx.ContainerStop.sh, apcw-dr.DictionaryRefresh.sh, apcw-fl.FixtureLoad.geriatric.sh, apcw-fl.FixtureLoad.progress.sh, apcw-nsa.NeuralStanfordAssay.sh, apcw-nsi.NeuralStanfordInstall.sh, apcw-r.Run.sh, apcw-t.Test.sh, bul_nolog_launcher.sh, buw-hj0.HandbookJurisdictionTop.sh, buw-hjl.HandbookJurisdictionLinux.sh, buw-hjm.HandbookJurisdictionMacos.sh, buw-hjw.HandbookJurisdictionWindows.sh, buw-jpCL.CaparisonLinux.sh, buw-jpCM.CaparisonMacos.sh, buw-jpCW.CaparisonWindows.sh, buw-jpGb.GarrisonBash.sh, buw-jpGc.GarrisonCygwin.sh, buw-jpGw.GarrisonWsl.sh, buw-jpS.PrivilegedSsh.sh, buw-jwc.WorkloadCommandFile.sh, buw-jwk.WorkloadKnock.sh, buw-jws.WorkloadInteractiveSession.sh, buw-qsc.QualifyShellCheck.sh, buw-rcr.RenderConfigRegime.sh, buw-rcv.ValidateConfigRegime.sh, buw-rer.RenderEnvironmentRegime.sh, buw-rev.ValidateEnvironmentRegime.sh, buw-rnl.ListNodeRegime.sh, buw-rnr.RenderNodeRegime.sh, buw-rnv.ValidateNodeRegime.sh, buw-rpl.ListPrivilegeRegime.sh, buw-rpr.RenderPrivilegeRegime.sh, buw-rpv.ValidatePrivilegeRegime.sh, buw-rsr.RenderStationRegime.sh, buw-rsv.ValidateStationRegime.sh, buw-st.BukSelfTest.sh, buw-tt-cbl.CreateTabTargetBatchLogging.sh, buw-tt-cbn.CreateTabTargetBatchNolog.sh, buw-tt-cil.CreateTabTargetInteractiveLogging.sh, buw-tt-cin.CreateTabTargetInteractiveNolog.sh, buw-tt-cl.CreateLauncher.sh, buw-tt-ll.ListLaunchers.sh, buw-xd.Delay.sh, jjw-tfP1.ProvisionPhase1.sh, jjw-tfP2.ProvisionPhase2.cerebro.sh, jjw-tfP2.ProvisionPhase2.localhost.sh, jjw-tfS.TestFundusSingle.localhost.sh, jjw-tfs.TestFundusScenario.cerebro.sh, jjw-tfs.TestFundusScenario.localhost.sh, launcher_nolog.rbw_workbench.sh, rbtd-ap.AccessProbe.director.sh, rbtd-ap.AccessProbe.governor.sh, rbtd-ap.AccessProbe.payor.sh, rbtd-ap.AccessProbe.retriever.sh, rbtd-b.Build.sh, rbtd-r.FixtureRun.access-probe.sh, rbtd-r.FixtureRun.batch-vouch.sh, rbtd-r.FixtureRun.canonical-establish.sh, rbtd-r.FixtureRun.dockerfile-hygiene.sh, rbtd-r.FixtureRun.enrollment-validation.sh, rbtd-r.FixtureRun.hallmark-lifecycle.sh, rbtd-r.FixtureRun.handbook-render.sh, rbtd-r.FixtureRun.moriah.sh, rbtd-r.FixtureRun.onboarding-sequence.sh, rbtd-r.FixtureRun.pluml.sh, rbtd-r.FixtureRun.pristine-lifecycle.sh, rbtd-r.FixtureRun.regime-smoke.sh, rbtd-r.FixtureRun.regime-validation.sh, rbtd-r.FixtureRun.srjcl.sh, rbtd-r.FixtureRun.tadmor.sh, rbtd-s.FixtureCase.sh, rbtd-s.TestSuite.complete.sh, rbtd-s.TestSuite.crucible.sh, rbtd-s.TestSuite.fast.sh, rbtd-s.TestSuite.gauntlet.sh, rbtd-s.TestSuite.service.sh, rbtd-t.Test.sh, rbw-HWdc.DockerContextDiscipline.sh, rbw-HWdd.DockerDesktop.sh, rbw-Ic.IfritClient.moriah.sh, rbw-Ic.IfritClient.tadmor.sh, rbw-Is.IfritSortie.moriah.sh, rbw-Is.IfritSortie.tadmor.sh, rbw-MG.MarshalGenerate.sh, rbw-MP.MarshalProofs.sh, rbw-MZ.MarshalZeroes.sh, rbw-Occ.OnboardingConfigureEnvironment.sh, rbw-Ocd.OnboardingCredentialDirector.sh, rbw-Ocr.OnboardingCredentialRetriever.sh, rbw-Oda.OnboardingDirectorAirgap.sh, rbw-Odb.OnboardingDirectorBind.sh, rbw-Odf.OnboardingDirectorFirstBuild.sh, rbw-Odg.OnboardingDirectorGraft.sh, rbw-Ofc.OnboardingFirstCrucible.sh, rbw-Og.OnboardingGovernor.sh, rbw-Op.OnboardingPayor.sh, rbw-Ots.OnboardingTadmorSecurity.sh, rbw-aM.PayorMantlesGovernor.sh, rbw-adD.GovernorDivestsDirector.sh, rbw-adI.GovernorInvestsDirector.sh, rbw-adr.GovernorRostersDirectors.sh, rbw-arD.GovernorDivestsRetriever.sh, rbw-arI.GovernorInvestsRetriever.sh, rbw-arr.GovernorRostersRetrievers.sh, rbw-cC.Charge.ccyolo.sh, rbw-cC.Charge.moriah.sh, rbw-cC.Charge.pluml.sh, rbw-cC.Charge.srjcl.sh, rbw-cC.Charge.tadmor.sh, rbw-cKB.KludgeBottle.sh, rbw-cKS.KludgeSentry.sh, rbw-cQ.Quench.ccyolo.sh, rbw-cQ.Quench.moriah.sh, rbw-cQ.Quench.pluml.sh, rbw-cQ.Quench.srjcl.sh, rbw-cQ.Quench.tadmor.sh, rbw-cS.SshTo.ccyolo.sh, rbw-cS.SshTo.moriah.sh, rbw-cb.Bark.moriah.sh, rbw-cb.Bark.pluml.sh, rbw-cb.Bark.srjcl.sh, rbw-cb.Bark.tadmor.sh, rbw-cf.Fiat.moriah.sh, rbw-cf.Fiat.pluml.sh, rbw-cf.Fiat.srjcl.sh, rbw-cf.Fiat.tadmor.sh, rbw-ch.Hail.sh, rbw-cic.CrucibleIsCharged.sh, rbw-cr.Rack.sh, rbw-cs.Scry.sh, rbw-cw.Writ.moriah.sh, rbw-cw.Writ.pluml.sh, rbw-cw.Writ.srjcl.sh, rbw-cw.Writ.tadmor.sh, rbw-dE.DirectorEnshrinesVessel.sh, rbw-dI.DirectorInscribesReliquary.sh, rbw-dL.PayorLeviesDepot.sh, rbw-dU.PayorUnmakesDepot.sh, rbw-dY.DirectorYokesReliquaryAllVessels.sh, rbw-di.DepotInfo.sh, rbw-dl.PayorListsDepots.sh, rbw-fA.DirectorAbjuresHallmark.sh, rbw-fO.DirectorOrdainsHallmark.sh, rbw-fV.DirectorVouchesHallmarks.sh, rbw-fhc.HygieneCheck.sh, rbw-fhv.HygieneCheckVessel.sh, rbw-fk.LocalKludge.sh, rbw-fpc.RetrieverPlumbsCompact.sh, rbw-fpf.RetrieverPlumbsFull.sh, rbw-fs.RetrieverSummonsHallmark.sh, rbw-ft.RetrieverTalliesHallmarks.sh, rbw-gPE.PayorEstablish.sh, rbw-gPI.PayorInstall.sh, rbw-gPR.PayorRefresh.sh, rbw-gq.QuotaBuild.sh, rbw-h0.HandbookTOP.sh, rbw-hw.HandbookWindows.sh, rbw-iJe.DirectorJettisonsEnshrinement.sh, rbw-iJh.DirectorJettisonsHallmarkImage.sh, rbw-iJr.DirectorJettisonsReliquaryImage.sh, rbw-iae.DirectorAuditsEnshrinements.sh, rbw-iah.DirectorAuditsHallmarks.sh, rbw-iar.DirectorAuditsReliquaries.sh, rbw-irh.DirectorRekonsHallmark.sh, rbw-irr.DirectorRekonsReliquary.sh, rbw-iwe.DirectorWrestsEnshrinedImage.sh, rbw-iwh.DirectorWrestsHallmarkImage.sh, rbw-iwr.DirectorWrestsReliquaryImage.sh, rbw-ni.NameplateInfo.sh, rbw-nv.ValidateNameplates.sh, rbw-o.ONBOARDING.sh, rbw-ral.ListAuthRegimes.sh, rbw-rar.RenderAuthRegime.sh, rbw-rav.ValidateAuthRegime.sh, rbw-rdc.CheckDepotRegime.sh, rbw-rdi.InscribeDepotRegime.sh, rbw-rdr.RenderDepotRegime.sh, rbw-rdv.ValidateDepotRegime.sh, rbw-rnl.ListNameplateRegime.sh, rbw-rnr.RenderNameplateRegime.sh, rbw-rnv.ValidateNameplateRegime.sh, rbw-ror.RenderOauthRegime.sh, rbw-rov.ValidateOauthRegime.sh, rbw-rpr.RenderPayorRegime.sh, rbw-rpv.ValidatePayorRegime.sh, rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh, rbw-rsr.RenderStationRegime.sh, rbw-rsv.ValidateStationRegime.sh, rbw-rvl.ListVesselRegime.sh, rbw-rvr.RenderVesselRegime.sh, rbw-rvv.ValidateVesselRegime.sh, rbw-tK.KludgeCycle.tadmor.sh, rbw-tO.OrdainCycle.tadmor.sh, rbw-tP.QualifyPristine.sh, rbw-tf.QualifyFast.sh, rbw-tr.QualifyRelease.sh, study-mpt.Run.FULL.sh, study-mpt.Run.api-FULL.sh, study-mpt.Run.smoke.sh, vow-F.Freshen.sh, vow-P.Parcel.sh, vow-R.Release.sh, vow-b.Build.sh, vow-c.Clean.sh, vow-r.RunVVX.sh, vow-t.Test.sh, vslk-i.InstallSlickEditProject.sh, vvw-r.RunVVX.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 44 commits)

  1 A bubc-rbmn-prefix-reconciliation
  2 B z-launcher-trampoline-introduce
  3 C rbcc-establish-fact-locale
  4 D moorings-filesystem-rename
  5 E kit-literal-sweep
  6 F marshal-zero-and-proof-regen
  7 G bcg-tabtarget-path-indirection-section
  8 H docs-vocabulary-sweep
  9 I kit-cleanup
  10 J moorings-acceptance
  11 K charge-subnet-keyed-reclaim

123456789abcdefghijklmnopqrstuvwxyz
·····x·····························  A  1c
·······xx··························  B  2c
·········xx························  C  2c
···········xxxxx···················  D  5c
················xx·················  E  2c
··················x················  F  1c
···················xx··············  G  2c
······················x·xx·········  H  3c
···························x·······  I  1c
····························x··x···  J  2c
································x·x  K  2c
```

## Steeplechase

### 2026-05-20 19:51 - ₢BKAAK - W

Collapsed rbob_charge's two-stage charge-invariant guard (nameplate-collision refusal + subnet-conflict refusal, both buc_die) into a single zrbob_reclaim_subnet: match the nameplate's subnet (structural fact) and tear down whatever occupies it — force-remove attached containers then the network — regardless of prefix or creation method. Deleted the impossible-state nameplate-collision branch. RBSCC spec step rewritten 'Detect Nameplate Collisions' -> 'Reclaim Nameplate Subnet'. BCG-compliant (stderr to forensic temp files, distinct temps per op, load-then-iterate, guarded array iteration, inverted-test control flow); shellcheck clean. Real-charge acceptance passed: staged a foreign-prefixed orphan network with a running container on tadmor's 10.242.0.0/24 (the exact case old code refused, reproducing the BKAAJ srjcl stall), charged tadmor — reclaim fired, orphan fully removed, all three containers Healthy, quenched clean.

### 2026-05-20 19:48 - Heat - n

Regime state: re-kludged tadmor sentry hallmark (k260520194735-cd9dbdd73) driven into rbrn.env to satisfy the charge clean-tree gate for the BKAAK subnet-reclaim acceptance test. Ephemeral local kludge state, not pace logic.

### 2026-05-20 19:47 - ₢BKAAK - n

Collapse rbob_charge's two-stage charge-invariant guard into a single subnet-keyed reclaim. Previously charge ran zrbob_detect_nameplate_collision (label/prefix-parsing refusal on any foreign compose network bearing the moniker) then zrbob_detect_subnet_conflict (refusal on any network occupying the nameplate's subnet) — both buc_die, refusing to charge until the operator manually removed the offending network. That coarseness stalled the macOS BKAAJ gauntlet on a stale canrbhl-srjcl orphan whose quench never ran. Replace both with zrbob_reclaim_subnet: enumerate networks, match the nameplate's subnet (RBRN_ENCLAVE_BASE_IP/NETMASK, a structural fact), and tear down whatever occupies it — force-remove attached containers then the network — regardless of prefix or creation method. Safe because a crucible's subnet derives from the nameplate not the prefix, so two same-nameplate crucibles can never coexist on one daemon; the nameplate-collision branch guarded an impossible state and its only real effect was to refuse instead of reclaim. Reclaim is Stage-1's force-takeover extended to all prefixes. BCG-compliant: stderr to forensic temp files (no 2>/dev/null), distinct temp files per operation, load-then-iterate, guarded array iteration, inverted-test control flow; shellcheck clean against busc_shellcheckrc. RBSCC spec: 'Detect Nameplate Collisions' step rewritten to 'Reclaim Nameplate Subnet' (rbbc_warn reclaim action + rbbc_fatal only if removal fails), line-7 summary 'subnet conflict detection' -> 'subnet reclaim'.

### 2026-05-20 19:37 - ₢BKAAJ - W

macOS branch of the two-platform acceptance: tt/rbw-tP.QualifyPristine.sh green across all 12 fixtures (236 cases) against the new rbmm_moorings layout — enrollment-validation 47, pristine-lifecycle 5, canonical-establish 4, onboarding-sequence 8, regime-validation 27, regime-smoke 9, dockerfile-hygiene 9, hallmark-lifecycle 1, tadmor 59, moriah 59, srjcl 3, pluml 5. Zero cutover defects surfaced; no code changes needed (the linux RBRD-tripwire repair already landed on main). One environmental stumble: the srjcl charge correctly refused on a stale-orphan collision — canrbhl-srjcl_{enclave,transit} networks from a run killed mid-srjcl ~5 days prior (depot canest2bhl100009), whose quench never ran. Cleaned the orphans; srjcl (3) and pluml (5) then validated as individual rbtd-r fixture runs against the still-live depot rather than inside a single end-to-end gauntlet pass, per operator choice. Per-platform regime-state commits (MarshalZero + ~16 fixture commits) isolated on branch bhyslop-macmini-pym-BKAAJ, pushed to origin for cerebro debug; main untouched. Completes the two-platform exit (linux on cerebro-BKAAJ + macOS both green). Charge's foreign-collision coarseness captured as follow-on pace BKAAK.

### 2026-05-20 19:35 - Heat - S

charge-subnet-keyed-reclaim

### 2026-05-20 22:40 - Heat - n

Linux pristine gauntlet (tt/rbw-tP) passed on cerebro against the new rbmm_moorings layout — all 12 fixtures green: enrollment-validation 47, pristine-lifecycle 5, canonical-establish 4, onboarding-sequence 8, regime-validation 27, regime-smoke 9, dockerfile-hygiene 9, hallmark-lifecycle 1, tadmor 59, moriah 59, srjcl 3, pluml 5. Merge back the one repair the run required: the RBRD tripwire drift-detector (rbrd_check) extracts /rbrd.env from the FROM-scratch tripwire image via docker create + docker cp + docker rm, but the Linux docker daemon rejects `docker create` on a command-less image ('no command specified'); pass a never-executed placeholder command (/rbrd.env) since the container is only cp'd then rm'd. Pre-existing defect from heat BO's tripwire feature (BOAAK), first surfaced by the full BK acceptance gauntlet. Per-platform regime-state commits from the run remain isolated on branch cerebro-BKAAJ; only this repair lands on main. macOS branch of the BKAAJ acceptance pace still pending.

### 2026-05-20 20:48 - ₢BKAAJ - n

Fix RBRD tripwire drift-detector failing the pristine gauntlet on linux at the inscribe-reliquary cloud-submit site (onboarding-sequence/rbtdro_onboarding_inscribe_reliquary). rbndb_base.sh's rbrd_check extracts /rbrd.env from the tripwire image by docker create + docker cp + docker rm; the tripwire is built FROM scratch with no CMD/ENTRYPOINT, and the Linux docker daemon rejects `docker create` on a command-less image with 'no command specified'. The container is never started, so pass a never-executed placeholder command (/rbrd.env) to satisfy the daemon's create-time requirement. Pre-existing bug from heat BO's tripwire feature (rbndb_base.sh line authored in 074f842b3/0bf76fe6e, BOAAK), surfaced for the first time by the full ₣BK acceptance gauntlet; fixed inline and affiliated to the moorings-acceptance pace per operator direction.

### 2026-05-20 13:04 - ₢BKAAI - W

Cleanup tail of the moorings cutover. Deleted the stranded Tools/rbk/rbmP_laterSecurityAudit.md (a 2026-03-10 advisory 'provisioner' security wishlist superseded by the Payor/Governor decomposition; squatted the rbm* prefix the moorings family now wants) — convergently, the ₣BM sprue-migration officium removed it first, so the deletion landed via commit 4e351f17 and the namespace is reclaimed. Registered the minted-but-undocumented prefix allocations in their owning namespace homes (deviating from the docket's literal repo-root CLAUDE.md target, per CLAUDE.md's own rule that expanded prefix trees live in the kit includes): rbm*_ family (rbmm/rbml/rbmn/rbmu/rbmv) in the RBK acronyms include, buml_ sprue namespace in the BUK include, and a one-line sprue pointer in repo-root CLAUDE.md's Tabtarget Universe Pattern. All entries are allocation/terminal-exclusivity records pointing at BCG 'Tabtarget Path Indirection' (added by BKAAG) — no restatement of the sprue rationale. rbob_compose.yml relocation skipped (already moved during the BKAAD rename). fast suite 107/107. Committed as 95821b9c against ₣BK.

### 2026-05-20 13:02 - Heat - n

Reclaim rbm* namespace and register the moorings prefix families (cleanup tail of the moorings cutover, pace BKAAI). Delete the stranded Tools/rbk/rbmP_laterSecurityAudit.md — a 2026-03-10 advisory security wishlist for a single high-privilege 'provisioner' SA that the Payor/Governor decomposition has since superseded; nothing referenced it, and it squatted the rbm* prefix now wanted by the moorings family. Register the minted-but-undocumented allocations in their owning namespace homes rather than the docket's literal repo-root CLAUDE.md target: rbm*_ family (rbmm/rbml/rbmn/rbmu/rbmv) in the RBK acronyms include, buml_ sprue namespace in the BUK include — both per CLAUDE.md's own rule that expanded prefix trees live in the kit includes. Repo-root CLAUDE.md gets only a one-line sprue pointer in the Tabtarget Universe Pattern section. All entries are allocation/terminal-exclusivity records pointing at BCG 'Tabtarget Path Indirection' (added by BKAAG) for the sprue rationale — no restatement. rbob_compose.yml relocation skipped (already moved during the BKAAD rename). README.md/CLAUDE.consumer.md left untouched — BKAAH's deferred work. fast suite 107/107 pass.

### 2026-05-20 12:59 - ₢BKAAH - W

Moorings vocabulary cutover across the documentation surface. Spec (.adoc): no document teaches .buk/.rbk/rbev-vessels — instead of a new path-quoin family, consolidated via MCM into the existing regime quoins. Established rbsld_moorings as a single full quoin (the one place the literal moorings dir name appears in the assembled spec, mirroring bash RBCC_moorings_dir); each regime file location lives in that regime's definition (rbrr/rbrd/rbrm/rbrp at moorings root, rbrv under {rbrr_vessel_dir} by sigil, rbrn under nameplate subdir by moniker); all operational prose references the regime concept quoin, so a regime filename basename survives only in its six authoritative definition location lines. Markdown (README.md, CLAUDE.consumer.md): directory displays collapsed to the single rbmm_moorings/ umbrella. Commits e650b27 (spec consolidation) + 8ab31a79 (markdown + naked-filename elimination). Fast suite green (no-op, doc-only). Deferred: concretizing rbXX_regime quoins to render as the path itself; FUTURE/RBSPV left literal (unassembled). Design note: the journey explored a dedicated rbsrf_/rbsld_ path-quoin family and per-file basename quoins before the operator steered to regime-quoin consolidation — fewer quoins, the file fact homed where axrd_file_sourced already lives.

### 2026-05-20 12:58 - ₢BKAAH - n

Markdown umbrella rework + naked-regime-filename elimination. (1) README.md and CLAUDE.consumer.md directory displays collapse the three former sibling roots (.buk/.rbk/rbev-vessels) into one rbmm_moorings/ umbrella: burc.env + regime .env at root, nameplate subdirs each with rbrn.env, rbml_launchers/, rbmv_vessels/ with its vessel tree re-indented a level deeper; inline vessel path to rbmm_moorings/rbmv_vessels/. (2) Replace every naked regime filename in operational prose with its regime concept quoin ({rbrr/rbrd/rbrm/rbrn/rbrv/rbrp_regime}) per operator directive. Sites: RBSDY yoke steps + the Rewrite-the-{rbrv_regime} step label (referenced twice), RBSAV/RBSAG sourcing, RBSAE enshrine write, RBSDI reliquary set, RBSRV enumeration, RBSTB trigger input, RBSDU recovery, RBSRT tripwire (embedded {rbrd_regime} assignment file vs local copy, drops in-image /rbrd.env), and rbrp payor sites (RBSAO/RBSPI x2/RBSPR/RBS0). Added rbrp_regime a File-sourced location line and fixed RBSRP stale project-root location. Basename literal now survives only in the six regime-definition location lines (their authoritative single home). Deferred: concretizing rbXX_regime to render as the path. FUTURE/RBSPV left literal (unassembled cruft).

### 2026-05-20 12:36 - Heat - n

Soften axrg_carrier path-locus wording to not prescribe representation. The original clause mandated that a singleton carrier's locus 'is named by one quoin' — but the RBS0 cutover (commit e650b27a, BKAAH) correctly rejected per-file carrier quoins on Load-Bearing Complexity grounds (a thin distinction over the regime concept) and instead composes each carrier's path in prose from the rbsld_moorings root quoin plus a literal basename, inside the regime definition. The motif should describe the concept (a file-sourced carrier has a path locus; singleton resolves to one path, manifold to one templated path per instance) and leave quoin-vs-prose representation to per-spec judgment. RBS0 voicings reviewed and confirmed correct (rbsld_moorings axtu_path; regimes retain axvr_regime axf_bash axrd_file_sourced; rbrr_vessel_dir axvr_variable rbst_path; no dangling refs) — no RBS0 edits needed.

### 2026-05-20 12:33 - ₢BKAAH - n

Spec-surface moorings vocabulary cutover via MCM consolidation, not a separate path-quoin family. Establish rbsld_moorings as a single full MCM quoin (anchor + axtu_path voicing + definition in a new Consumer Filesystem Layout subsection; mapping reftext rbmm_moorings/) — the one place the literal moorings directory name appears in the assembled spec, mirroring bash fact-locale RBCC_moorings_dir/BUBC_moorings_dir. Consolidate each regime file's location into that regime's existing legacy quoin definition (rbrr/rbrd/rbrm/rbrn_regime, all already annotated axrd_file_sourced): each def gains a File-sourced line composing {rbsld_moorings} + literal basename leaf; rbrn composes the per-nameplate {rbrn_moniker} subdir; rbrv (manifold) locates under the existing {rbrr_vessel_dir} quoin by {rbrv_sigil}. Rewire prose: the rbrd tripwire/levy sites (RBSRT, RBSDE) and locating sentences (RBSRR, RBSRM, RBRN, RBSRV, RBSHR) reference the regime concept ({rbXX_regime} assignment file) rather than repeating paths; RBSIP vessel-family + build-context paths use {rbrr_vessel_dir}, moorings-regime-files use {rbsld_moorings}. No per-file quoins minted (rejected as thin distinction vs the regime concept; the file/concept split is load-bearing only in the tripwire byte-comparison, served by the regime quoin). CLAUDE.md node-profile path and unassembled FUTURE/RBSPV updated to literal rbmm_moorings paths (markdown/unassembled — MCM does not apply). Assembled spec now carries zero .buk/.rbk/rbev-vessels literals and the moorings literal exactly once. README.md and CLAUDE.consumer.md (markdown directory displays) deferred to a following commit.

### 2026-05-20 12:20 - Heat - n

Mint axrg_carrier motif in AXLA; trim axrg_assignment to a single binding. The stale axrg_assignment wording conflated two layers — one variable-value binding vs. the artifact holding one instance's complete assignment set. Split them: axrg_assignment is now narrowly the binding; new axrg_carrier names the carrier, with provenance-polymorphic form (axrd_file_sourced manifests as a file at a path; axrd_constructed as a defining module sourced into consumers, e.g. RBDC/RBGC virtual regimes; axrd_env_inherited as a process scope). Carrier cardinality ties to existing axrd_singleton/axrd_manifold (one carrier vs. one-per-instance), and the path-locus-iff-file-sourced rule is stated as prose. Reworded the provenance-dimensions preamble and mapping comment to qualify the carrier rather than the assignment. RBS0 deliberately untouched — voicings (rbsld_* -> axtu_path, rbsrf_* -> axrg_carrier axrd_file_sourced) land separately.

### 2026-05-20 11:31 - ₢BKAAG - W

Codify the tabtarget-trampoline pattern in BCG. Added a 'Tabtarget Path Indirection' subsection under ## File Templates, placed immediately before 'Dispatch-Provided Directory Variables' so the guide reads in execution order: trampoline establishes context (resolves moorings, normalizes cwd to repo root) -> dispatch provides BURD_* -> CLI entry point. Content grounded against live tt/z-launcher.sh: the one-line tabtarget exec shape; the trampoline's two responsibilities; the sprue ({owner}ml_{launcher-id}) with a worked rbml_/buml_ table, the ownership-semantic-not-location-selector framing, and ${1#*ml_} launcher-id recovery; the load-bearing rationale for a sprue over a bare workbench-id (closes the colophon/launcher namespace-collision class, tied to the Load-Bearing Complexity test); moorings as single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer); deterministic workbench cwd. Cross-references render in BCG house style: the guide uses no markdown anchor links anywhere, so my initial [text](#anchor) draft was reverted to prose name-references (the Load-Bearing Complexity test, instance of CLI as Module Gateway, the Dispatch-Provided Directory Variables described next). Defensive fast suite green 107/107 (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15, dockerfile-hygiene 9) — no-op for the doc-only change as expected.

### 2026-05-20 11:31 - ₢BKAAG - n

Add Tabtarget Path Indirection section to BCG

### 2026-05-20 11:24 - ₢BKAAF - W

Marshal zero/proof verified moorings-correct by inspection; no code changes required. The mechanical path migration was already completed by ₢BKAAD (filesystem rename) + ₢BKAAE (literal sweep): rblm_cli.sh carries zero .buk/.rbk/rbev-vessels literals and routes every path through RBCC constants. rblm_zero does not scaffold a fresh tree — it zeroes the live regime in place; its single layout-shaped emission (line 139) already writes RBRR_VESSEL_DIR=rbmm_moorings/rbmv_vessels. rblm_proof is layout-agnostic (clones repo + copies out-of-repo station-files), no legacy literals. Confirmed live regime already on new layout (RBRR_VESSEL_DIR=rbmm_moorings/rbmv_vessels). Gate: fast suite 107/107 (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15, dockerfile-hygiene 9). MP end-to-end DEFERRED by operator decision: rblm_zero is a destructive release-ceremony reset (deletes credentials, blanks depot identity, auto-commits) gated on clean+pushed HEAD — its genuine exercise belongs in a proof clone, not the live working repo. Finding: the docket's 'find where Marshal generates .buk/.rbk structures' framing was off — Marshal zeroes the live regime rather than scaffolding; the true MZ exercise is inside an MP proof clone, which is why the docket couples the two.

### 2026-05-20 11:18 - ₢BKAAE - W

Kit-literal + RBBC sweep: route all kit references through named RBCC/BUBC constants, retire transitional RBBC_* aliases. Two parallel sonnet agents on disjoint trees: RBK (Tools/rbk/**/*.sh + rbtd/src/**/*.rs) swapped ~115 RBBC_* refs to RBCC equivalents and corrected 7 stale .rbk/rbev-vessels literals in comments/operator messages to rbmm_moorings/rbmv_vessels (via constants where live; RBBC_kit_subdir site resolved to RBCC_KIT_DIR, no new constant); BUK (Tools/buk/**/*.sh) replaced 12 stale .buk/ literals with BUBC constants (none minted). Parent resolved two cross-boundary sites: butcrg_RegimeCredentials.sh RBBC_rbrr_file -> RBCC_rbrr_file (already RBK-coupled via zrbrr_*/RBDC_*, follows live pool), and BCG naming-convention example off the deleted symbol. Retired RBBC_* alias block from rbcc_Constants.sh. Caught a real kindle-ordering bug: the buk-agent referenced ${BUBC_moorings_dir} in zburd_kindle, but BURD is kindled from 9 sites and only some furnish BUBC first -> unbound variable; fix is zburd_kindle now kindles its BUBC dependency via source "${BURD_BUK_DIR}/bubc_constants.sh" (dispatch anchor already used to locate burd_regime.sh, idempotent via BUBC inclusion guard) rather than retreating to a literal. Census clean: grep RBBC_ Tools/ returns only a retirement-narrating comment; .buk/.rbk/rbev-vessels literal grep empty across kit code. Gate: fast suite 107/107. Finding for follow-up: butcrg_RegimeCredentials.sh is a BUK-namespaced test reaching into RBK internals with an implicit (un-sourced) dependency -- architectural question whether it belongs in RBK's tree or BUK needs a stable cross-kit probe interface.

### 2026-05-20 11:18 - ₢BKAAE - n

Retire RBBC_* transitional aliases in favor of RBCC_* across BUK and RBK (literal sweep, pace BKAAE). Delete the RBBC_* alias block from rbcc_Constants.sh now that every consumer reads the moorings-relative RBCC_* names directly: the furnish config-source lines (rbob/rbra/rbfc/rbfd/rbfh/rbfk/rbfl/rbfr/rbfv/rbgg/rbgp/rbro/rbrr/rbrv/rbrd/rbrp clis, rbh0/* onboarding, rbte probe), the rbob_bottle compose env-file paths (RBRR/RBRD now via RBCC_rbrr_file/RBCC_rbrd_file; project-directory + nameplate/fragment/hook paths via RBCC_moorings_dir), rblm zero/generate, rbndb tripwire, rbrn fleet survey + list/preflight, and the rbtdrf_fast theurge probe heredoc. Switch butcrg test cases to RBCC_rbrr_file. Refresh user-facing path literals from legacy .buk/.rbk to rbmm_moorings/rbmu_users/rbmn_nodes in burd/burn/burp/burs descriptions and warnings, plus stale doc comments (rbgc, rbndb, rbfc, rbv); kindle bubc_constants.sh in zburd_kindle so BURD enroll descriptions can interpolate BUBC_moorings_dir. Update BCG tinder-constant example RBBC_rbrr_file → RBCC_rbrr_file.

### 2026-05-20 10:51 - ₢BKAAD - W

Moorings filesystem cutover complete and validated end-to-end. Three legacy roots (.buk/.rbk/rbev-vessels) moved into rbmm_moorings/ with the launcher subtree at rbml_launchers/ and vessels at rbmv_vessels/; kit-machinery compose files (rbob_compose.yml + rbje_compose_probe.env) relocated to Tools/rbk/. Every path resolver rewired or made to derive from a single source: z-launcher trampoline flipped; launcher stubs + bul_launcher switched to trust the dispatch-established repo-root PWD (deleted brittle directory-depth counting), with bul_launcher's config-dir literal the sole bootstrap anchor and BUBC mirrors for non-bootstrap consumers; RBBC_dot_dir flipped so ~40 RBK consumers derive for free; rbob_bottle compose/probe reads moved to RBCC_KIT_DIR; cross-kit burc.env readers derive from the enrolled BURD_REGIME_FILE symbol or hold named-constant anchors; RBRR_VESSEL_DIR content + rblm template flipped. Theurge test fixtures single-sourced via rbtd_moorings_dir!/rbtd_vessels_dir! macros (concat!-composed path consts, zero new dependency, zero repetition). Magic-string discipline enforced throughout per RCG/BCG — no re-literaling. One charge-path regression caught by tadmor charge and fixed (compose --project-directory pinned to moorings root). Validation: fast 107/107, theurge unit 114/114, buw self-test 28/28, tadmor charge + 59-case adversarial security suite + clean quench, family tabtargets rbw/rbtw/buw/vvw/vow. Deleted dead ifrit setcap patch. Follow-ups noted: jjrlg_legatio remote-fundus .buk reads (cross-repo, deferred); jjw/vslw tabtargets dispatch via the proven launcher mechanism but were not run (external deps); rbnnh_compose.yml fragment header comment still references .rbk (consumer file, stale comment only). Raw file moves landed under a concurrent officium's self-committing jjx sweep rather than ₢BKAAD.

### 2026-05-20 10:34 - ₢BKAAD - n

Fix charge-path regression from relocating rbob_compose.yml to Tools/rbk: pin compose --project-directory to the moorings root. Docker compose derives the project directory from the first -f file's location; with the base compose now kit-resident (absolute Tools/rbk path), the project dir shifted to Tools/rbk, breaking the nameplate fragment's relative paths (env_file: tadmor/rbrn.env resolved to Tools/rbk/tadmor/rbrn.env; volume mounts ../ likewise). The rbnnh_compose.yml fragment authors paths relative to the consumer-config root, so --project-directory must be set explicitly to RBBC_dot_dir (rbmm_moorings). Caught by tadmor charge during cutover validation.

### 2026-05-20 10:27 - ₢BKAAD - n

Charge-path validation: kludge tadmor bottle (rbev-bottle-ifrit-tether) — drove RBRN_BOTTLE_HALLMARK=k260520095350-c48853e0 into the moorings nameplate. Both tadmor vessels now locally kludged; committing the nameplate so charge's reproducibility gate is satisfied. (Charge also ran fast qualification green, validating buq_qualify launcher-path derivation against the moorings layout.)

### 2026-05-20 09:52 - ₢BKAAD - n

Charge-path validation: kludge tadmor sentry (rbev-sentry-deb-tether) — confirms RBRR_VESSEL_DIR resolves to rbmm_moorings/rbmv_vessels and the kludge drives RBRN_SENTRY_HALLMARK into the moorings nameplate dir rbmm_moorings/tadmor/rbrn.env. Hallmark k260520095006-4bded6b6. Committing to clear the tree for the bottle kludge's provenance gate.

### 2026-05-20 09:48 - ₢BKAAD - n

Moorings cutover: path-resolver rewires + magic-string single-sourcing (filesystem moves themselves landed under a concurrent officium's self-committing jjx sweep). z-launcher trampoline flipped to rbmm_moorings/rbml_launchers (both the exec path and BURD_LAUNCHER export). Launcher stubs + bul_launcher now trust the dispatch-established repo-root PWD instead of counting directory hops — deleted the brittle ../.. depth that broke when launchers moved two levels deep; bul_launcher:37 is the sole config-root bootstrap anchor (cannot derive — must locate config before sourcing), with BUBC_moorings_dir/BUBC_launchers_subdir mirroring it for non-bootstrap consumers. RBBC_dot_dir flipped .rbk->rbmm_moorings (~40 consumers derive for free). rbob_bottle compose-base + quoting-probe rewired from RBBC_dot_dir to RBCC_KIT_DIR (both relocated to Tools/rbk as kit machinery, pulled forward from AAI so charge stays green across the flip). Cross-kit burc.env readers derive from the enrolled BURD_REGIME_FILE symbol where dispatched (buut_cli, vob_cli, vob_build, vslw); standalone installers (vvi/vvu) and the pre-BUK buw-SI bootstrap hold named-constant anchors. RBRR_VESSEL_DIR content + rblm zero-template flipped to rbmm_moorings/rbmv_vessels. buq/rbq launcher-path qualifiers derive from BUBC/RBCC. Theurge test fixtures: introduced rbtd_moorings_dir!/rbtd_vessels_dir! macros in lib.rs so every path const composes via concat! from one literal (concat! eagerly expands the macro token where it rejects a const ident) — zero new dependency, zero textual repetition, structural drift-safety; reverted the interim assertion scaffolding. Deleted the dead/superseded ifrit setcap patch. Verified: fast suite 107/107, theurge unit 114/114, buw self-test 28/28.

### 2026-05-20 08:08 - ₢BKAAC - W

Absorb RBBC into RBCC; RBCC is now the sole RBK fact locale. Deleted .buk/rbbc_constants.sh. RBCC_KIT_DIR is now a source-time ${BASH_SOURCE[0]%/*} derivation (BCG kit-self-location, canon rbte_cli.sh), and zrbcc_kindle drops its now-unused BURC_TOOLS_DIR precondition. Minted moorings inventory constants holding FUTURE moorings-relative values (RBCC_moorings_dir + rbml_/rbmu_/rbmn_/rbmv_ subdir constants; moorings-prefixed RBCC_rbrr/rbrp/rbrm/rbrd_file) — unread by kit code, inventory for AAD value-flip and AAE literal sweep. Emitted RBBC_* transitional aliases (kit_subdir, dot_dir, rbrr/rbrd/rbrm/rbrp_file) at current .rbk/ values, covering every RBBC_* name kit code reads. Retargeted 24 CLI furnishes from the RBBC-bootstrap pattern to BASH_SOURCE self-location: ${BASH_SOURCE[0]%/*} for the 20 Tools/rbk/*_cli.sh, ${BASH_SOURCE[0]%/*}/.. for the rbh0 trio and vov_veiled/rbv_cli; pruned stale BURD_TOOLS_DIR/BURD_CONFIG_DIR doc_env lines where no longer read (rblm and rbhw0 retain BURD_TOOLS_DIR). Updated rbtdrp_pristine.rs comment (its .rbk/ path constants flip in AAD). Fast suite 107/107. grep rbbc_constants clean. Flagged for follow-up: Tools/buk/buts/butcrg_RegimeCredentials.sh reads ${RBBC_rbrr_file} via the shared stub — keeps working through the alias, but post-absorption wants an explicit RBCC source in the BUK furnish chain or relocation to the RBK testbench (out of scope here).

### 2026-05-20 08:08 - ₢BKAAC - n

Replace .buk/rbbc_constants.sh bootstrap file with self-locating kit-dir pattern in RBK furnishes

### 2026-05-20 01:34 - ₢BKAAB - W

Introduced tt/z-launcher.sh trampoline: normalizes cwd to repo root and dispatches via minted {owner}ml_{launcher-id} sprues, forwarding tabtarget basename + args unchanged. Collapsed the nolog launcher into a BURD_NO_LOG guard in bul_launcher.sh (deleted bul_nolog_launcher.sh + .buk/launcher_nolog stub); the 13 former nolog tabtargets keep BURD_NO_LOG=1 and use rbml_rbw. Rewired all 214 tabtargets to the trampoline (sprue derived from each file's actual BURD_LAUNCHER; flag exports preserved; standalone buw-SI untouched). Updated buq_qualify.sh shape contract (sprue extraction + launcher-existence check, min 2 lines) with z-launcher.sh exemption, and buut_tabtarget.sh generator to emit the trampoline pattern. Fixed pre-existing stale launcher-stub generator (bul_launcher.sh/bul_launch, was launcher_common.sh/bud_launch). cwd audit gate clean (cwd already repo-root via launcher; no rework). Verified: fast suite 107/107, QualifyFast pass, multi-cwd spot-checks across rbml_/buml_ families and logging+nolog paths.

### 2026-05-20 01:34 - ₢BKAAB - n

Route all tabtargets through a universal tt/z-launcher.sh trampoline that resolves a minted {owner}ml_ sprue token to its moorings launcher and normalizes cwd to repo root. Collapse the separate nolog launcher: BURD_NO_LOG now gates the BURS station load inside bul_launcher rather than selecting a distinct launcher stub. Update the tabtarget generator (buut) and qualifier (buq) to emit and validate the sprue form, and exempt z-launcher.sh from tabtarget qualification.

### 2026-05-19 16:14 - Heat - d

paddock curried: sprue scheme locked + buml_ family registered

### 2026-05-19 15:42 - ₢BKAAA - W

rbmn_ joins the rbm*_ moorings family as the nodes member. Investigation: rbmn_ has one definition site (BUBC_rbmn_nodes_subdir="rbmn_nodes") sitting side-by-side with BUBC_rbmu_users_subdir; the two are the BURN/BURP regime pair — BURN viceroyalty profiles (.buk/rbmn_nodes/) and BURP investiture profiles (.buk/rbmu_users/), sibling consumer-authored profile subtrees. The AAJ family table (rbmm_/rbml_/rbmu_/rbmv_) was simply incomplete, not deliberately excluding rbmn_. Splitting it off would invent a one-member family whose only distinction from rbmu_ is the BURN/BURP axis they already jointly express. Paddock line 16 updated to enumerate rbmn_ (nodes) with the sibling rationale. Pace 9 CLAUDE.md scope needs no docket edit — its conditional branch already covers rbmn_. Pace 3 surfaces no new RBCC constant; rbmn_ is simply known in-family. No code change.

### 2026-05-19 15:41 - Heat - d

paddock curried: ₢BKAAA: rbmn_ joins rbm*_ family as nodes member

### 2026-05-13 13:00 - Heat - S

moorings-acceptance

### 2026-05-13 12:59 - Heat - S

kit-cleanup

### 2026-05-13 12:59 - Heat - S

docs-vocabulary-sweep

### 2026-05-13 12:59 - Heat - S

bcg-tabtarget-path-indirection-section

### 2026-05-13 12:59 - Heat - S

marshal-zero-and-proof-regen

### 2026-05-13 12:58 - Heat - S

kit-literal-sweep

### 2026-05-13 12:58 - Heat - S

moorings-filesystem-rename

### 2026-05-13 12:58 - Heat - S

rbcc-establish-fact-locale

### 2026-05-13 12:57 - Heat - S

z-launcher-trampoline-introduce

### 2026-05-13 12:57 - Heat - S

bubc-rbmn-prefix-reconciliation

### 2026-05-13 12:57 - Heat - d

paddock curried: initial paddock at heat nomination

### 2026-05-13 12:56 - Heat - f

racing

### 2026-05-13 12:56 - Heat - N

rbk-12-mvp-moorings-cutover

