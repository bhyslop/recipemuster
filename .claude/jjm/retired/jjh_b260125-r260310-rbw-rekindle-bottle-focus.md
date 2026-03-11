# Heat Trophy: rbw-rekindle-bottle-focus

**Firemark:** ₣AP
**Created:** 260125
**Retired:** 260310
**Status:** retired

## Paddock

# Paddock: rbw-rekindle-bottle-focus

## Build Requirements

This heat focuses on RBW (bash/GCP tooling). No Rust builds required — skip `tt/vow-b.Build.sh` and `tt/vow-t.Test.sh` for pre-wrap verification.

## Context

Rekindle the Recipe Bottle infrastructure after a period of dormancy. Verify GCP resources are still active, Docker lifecycle works, and test suites pass.

## References

- `Tools/rbw/` — RBW bash tooling
- `tt/rbw-*.sh` — RBW tabtargets

## Paces

### fix-sbom-arm64-test-vessel (₢APAAp) [complete]

**[260216-1059] complete**

Fix SBOM generation step failing on arm64-only test vessels in GCB.

## Problem

The ark-lifecycle test fails at step 6 (rbgjb08-sbom-and-summary.sh) with:
`no matching manifest for linux/amd64 in the manifest list entries`

The test vessel `trbim-macos` builds arm64-only, but the SBOM step runs `docker pull` on a linux/amd64 GCB worker, which cannot pull a single-arch arm64 image. This predates the skopeo-to-crane migration and likely broke when the test vessel was narrowed to arm64-only.

## Questions to resolve

1. **Can we do SBOM for arm64 builds?** Syft (the SBOM tool) should work on any OCI image regardless of platform — but `docker pull` on an amd64 host can only pull amd64 images. The fix may be to use `crane pull` or `syft` directly against the registry (no docker pull needed) rather than pulling through Docker.

2. **Should trbim-macos be multi-platform?** A multi-platform test vessel (amd64+arm64) would make the SBOM step work naturally but doubles build time. Alternatively, keep it single-arch and fix the SBOM step to handle any platform.

3. **Do we need platform-specific test vessels?** E.g., `trbim-amd64` and `trbim-arm64` with ark-lifecycle running whichever matches the GCB worker arch. Simpler per-vessel but more vessels to maintain.

## Work

1. Read rbgjb08-sbom-and-summary.sh to understand exactly how it pulls and analyzes
2. Check if syft can analyze images directly from registry without docker pull
3. Decide: fix SBOM step to be platform-agnostic, OR make test vessel multi-platform, OR add platform-specific test vessels
4. Implement the chosen approach
5. Re-run ark-lifecycle to confirm full pass

## Acceptance

- ark-lifecycle test passes end-to-end (all 6 steps)
- SBOM generation works for arm64 images built on amd64 GCB workers
- Clear decision documented on test vessel platform strategy

**[260216-1038] rough**

Fix SBOM generation step failing on arm64-only test vessels in GCB.

## Problem

The ark-lifecycle test fails at step 6 (rbgjb08-sbom-and-summary.sh) with:
`no matching manifest for linux/amd64 in the manifest list entries`

The test vessel `trbim-macos` builds arm64-only, but the SBOM step runs `docker pull` on a linux/amd64 GCB worker, which cannot pull a single-arch arm64 image. This predates the skopeo-to-crane migration and likely broke when the test vessel was narrowed to arm64-only.

## Questions to resolve

1. **Can we do SBOM for arm64 builds?** Syft (the SBOM tool) should work on any OCI image regardless of platform — but `docker pull` on an amd64 host can only pull amd64 images. The fix may be to use `crane pull` or `syft` directly against the registry (no docker pull needed) rather than pulling through Docker.

2. **Should trbim-macos be multi-platform?** A multi-platform test vessel (amd64+arm64) would make the SBOM step work naturally but doubles build time. Alternatively, keep it single-arch and fix the SBOM step to handle any platform.

3. **Do we need platform-specific test vessels?** E.g., `trbim-amd64` and `trbim-arm64` with ark-lifecycle running whichever matches the GCB worker arch. Simpler per-vessel but more vessels to maintain.

## Work

1. Read rbgjb08-sbom-and-summary.sh to understand exactly how it pulls and analyzes
2. Check if syft can analyze images directly from registry without docker pull
3. Decide: fix SBOM step to be platform-agnostic, OR make test vessel multi-platform, OR add platform-specific test vessels
4. Implement the chosen approach
5. Re-run ark-lifecycle to confirm full pass

## Acceptance

- ark-lifecycle test passes end-to-end (all 6 steps)
- SBOM generation works for arm64 images built on amd64 GCB workers
- Clear decision documented on test vessel platform strategy

### remove-gcrane-pin-add-crane-freshening (₢APAAq) [complete]

**[260217-0643] complete**

Remove unused RBRR_GCB_GCRANE_IMAGE_REF and add crane tarball freshening.

## Context

The gcrane container image (gcr.io/go-containerregistry/gcrane) is distroless with no shell. After the skopeo-to-crane migration (₢APAAo), step 07 runs in alpine and installs crane from RBRR_CRANE_TAR_GZ. The gcrane image pin has no remaining consumers.

## Work — Remove RBRR_GCB_GCRANE_IMAGE_REF

1. rbrr.env — delete the gcrane comment and variable lines
2. Tools/rbw/rbrr_cli.sh — remove from regime display and refresh list
3. Tools/rbw/rbrr_regime.sh — remove default, rollup, env check, validation
4. lenses/RBS0-SpecTop.adoc — remove attribute reference, anchor, and definition
5. lenses/RBSRR-RegimeRepo.adoc — remove reference
6. Tools/rbw/rbf_Foundry.sh — remove the test-n check for RBRR_GCB_GCRANE_IMAGE_REF at line 131
7. Tools/rbw/rbgjm_mirror.json — this uses the gcrane image; decide if mirror.json should also switch to alpine+crane or if mirror.json is dormant

## Work — Add crane tarball freshening to refresh script

RBRR_CRANE_TAR_GZ is currently a static URL (v0.20.3). The refresh script should discover the latest go-containerregistry release from GitHub API and update the URL. Pattern: query GitHub releases API for google/go-containerregistry, find latest stable tag, construct tarball URL.

## Acceptance

- Zero references to RBRR_GCB_GCRANE_IMAGE_REF in codebase (outside gallops/retired)
- rbrr_refresh_gcb_pins updates RBRR_CRANE_TAR_GZ to latest stable release
- qualify-all passes

**[260216-1039] rough**

Remove unused RBRR_GCB_GCRANE_IMAGE_REF and add crane tarball freshening.

## Context

The gcrane container image (gcr.io/go-containerregistry/gcrane) is distroless with no shell. After the skopeo-to-crane migration (₢APAAo), step 07 runs in alpine and installs crane from RBRR_CRANE_TAR_GZ. The gcrane image pin has no remaining consumers.

## Work — Remove RBRR_GCB_GCRANE_IMAGE_REF

1. rbrr.env — delete the gcrane comment and variable lines
2. Tools/rbw/rbrr_cli.sh — remove from regime display and refresh list
3. Tools/rbw/rbrr_regime.sh — remove default, rollup, env check, validation
4. lenses/RBS0-SpecTop.adoc — remove attribute reference, anchor, and definition
5. lenses/RBSRR-RegimeRepo.adoc — remove reference
6. Tools/rbw/rbf_Foundry.sh — remove the test-n check for RBRR_GCB_GCRANE_IMAGE_REF at line 131
7. Tools/rbw/rbgjm_mirror.json — this uses the gcrane image; decide if mirror.json should also switch to alpine+crane or if mirror.json is dormant

## Work — Add crane tarball freshening to refresh script

RBRR_CRANE_TAR_GZ is currently a static URL (v0.20.3). The refresh script should discover the latest go-containerregistry release from GitHub API and update the URL. Pattern: query GitHub releases API for google/go-containerregistry, find latest stable tag, construct tarball URL.

## Acceptance

- Zero references to RBRR_GCB_GCRANE_IMAGE_REF in codebase (outside gallops/retired)
- rbrr_refresh_gcb_pins updates RBRR_CRANE_TAR_GZ to latest stable release
- qualify-all passes

### consolidate-regime-load-primitives (₢APAAg) [complete]

**[260210-1019] complete**

Consolidate nameplate and repo regime loading into canonical functions in their respective _regime.sh files.

## Functions to Create

### In `rbrn_regime.sh`

1. **`rbrn_load <moniker>`** — Canonical entry point: construct path to `rbrn_<moniker>.env`, verify exists, source, kindle, validate. Single call replaces the 4-6 line pattern repeated across callers.

2. **`rbrn_list`** — Enumerate concrete nameplate monikers by globbing `rbrn_*.env` and stripping prefix/suffix. No loaded regime prerequisite — this is a regime primitive for "what instances exist?"

### In `rbrr_regime.sh`

3. **`rbrr_load`** — Source `rbrr_RecipeBottleRegimeRepo.sh`, kindle. Encapsulates path construction and existence check. Not all callers need RBRR (e.g., rbob_cli doesn't always), so this stays separate from rbrn_load.

## Callsites to Refactor

### rbrn_load consolidation (4 sites):
- `rbt_testbench.sh:54-62` — construct path, exists, source, kindle, validate
- `rbob_cli.sh:108-112` — construct path, exists, source, kindle, validate
- `rbrn_cli.sh:47-48` — source + kindle (validate cmd)
- `rbrn_cli.sh:78-80` — source + kindle (render cmd)

### rbrr_load consolidation (8 sites):
- `rbt_testbench.sh:68-71` — construct path, exists, source, kindle
- `rbob_cli.sh:115-118` — construct path, exists, source, kindle
- `rbf_cli.sh:49-52` — via RBL, exists, source, kindle
- `rbgg_cli.sh:43-46` — via RBL, exists, source
- `rbgd_DepotConstants.sh:33-37` — via RBL, source, kindle, validate
- `rbrv_cli.sh:47-50` — conditional source (validate)
- `rbrv_cli.sh:85-88` — conditional source (render)
- `trbim_suite.sh:27` — source RBRR

### rbrn_list consolidation (1 site):
- `rbw_workbench.sh:123-132` — inline glob with strip logic

## Notes

- rbrn_cli.sh callers receive the file path externally; rbrn_load takes a moniker. Decide whether rbrn_load is moniker-only or also accepts a path, or whether rbrn_cli uses a lower-level internal.
- Some RBRR callers go through RBL (rbl_Locator.sh) for path resolution. Decide whether rbrr_load encapsulates RBL or sits alongside it.
- Mechanical refactoring — pattern is clear from existing code.

**[260210-0952] bridled**

Consolidate nameplate and repo regime loading into canonical functions in their respective _regime.sh files.

## Functions to Create

### In `rbrn_regime.sh`

1. **`rbrn_load <moniker>`** — Canonical entry point: construct path to `rbrn_<moniker>.env`, verify exists, source, kindle, validate. Single call replaces the 4-6 line pattern repeated across callers.

2. **`rbrn_list`** — Enumerate concrete nameplate monikers by globbing `rbrn_*.env` and stripping prefix/suffix. No loaded regime prerequisite — this is a regime primitive for "what instances exist?"

### In `rbrr_regime.sh`

3. **`rbrr_load`** — Source `rbrr_RecipeBottleRegimeRepo.sh`, kindle. Encapsulates path construction and existence check. Not all callers need RBRR (e.g., rbob_cli doesn't always), so this stays separate from rbrn_load.

## Callsites to Refactor

### rbrn_load consolidation (4 sites):
- `rbt_testbench.sh:54-62` — construct path, exists, source, kindle, validate
- `rbob_cli.sh:108-112` — construct path, exists, source, kindle, validate
- `rbrn_cli.sh:47-48` — source + kindle (validate cmd)
- `rbrn_cli.sh:78-80` — source + kindle (render cmd)

### rbrr_load consolidation (8 sites):
- `rbt_testbench.sh:68-71` — construct path, exists, source, kindle
- `rbob_cli.sh:115-118` — construct path, exists, source, kindle
- `rbf_cli.sh:49-52` — via RBL, exists, source, kindle
- `rbgg_cli.sh:43-46` — via RBL, exists, source
- `rbgd_DepotConstants.sh:33-37` — via RBL, source, kindle, validate
- `rbrv_cli.sh:47-50` — conditional source (validate)
- `rbrv_cli.sh:85-88` — conditional source (render)
- `trbim_suite.sh:27` — source RBRR

### rbrn_list consolidation (1 site):
- `rbw_workbench.sh:123-132` — inline glob with strip logic

## Notes

- rbrn_cli.sh callers receive the file path externally; rbrn_load takes a moniker. Decide whether rbrn_load is moniker-only or also accepts a path, or whether rbrn_cli uses a lower-level internal.
- Some RBRR callers go through RBL (rbl_Locator.sh) for path resolution. Decide whether rbrr_load encapsulates RBL or sits alongside it.
- Mechanical refactoring — pattern is clear from existing code.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: rbrn_regime.sh, rbrr_regime.sh, rbt_testbench.sh, rbob_cli.sh, rbrn_cli.sh, rbf_cli.sh, rbgg_cli.sh, rbgd_DepotConstants.sh, rbrv_cli.sh, trbim_suite.sh, rbw_workbench.sh, rbgm_ManualProcedures.sh (12 files) | Steps: 1. Add rbrn_load and rbrn_list public functions to rbrn_regime.sh using BURC_TOOLS_DIR relative paths from guaranteed project root CWD per BUK launcher convention -- no BASH_SOURCE dirname tricks 2. Add rbrr_load public function to rbrr_regime.sh using project-root-relative path to rbrr_RecipeBottleRegimeRepo.sh 3. Refactor 4 rbrn callsites to use rbrn_load: rbt_testbench.sh, rbob_cli.sh, rbrn_cli.sh validate and render 4. Refactor 8 rbrr callsites to use rbrr_load: rbt_testbench.sh, rbob_cli.sh, rbf_cli.sh, rbgg_cli.sh, rbgd_DepotConstants.sh, rbrv_cli.sh validate and render, trbim_suite.sh 5. Refactor rbw_workbench.sh nameplate enumeration inline glob to use rbrn_list | Notes: rbrn_cli.sh receives file path externally not moniker so it may need a lower-level rbrn_load_file variant or the cli adapts to extract moniker from path. Some rbrr callers use RBL for path resolution -- rbrr_load should not depend on RBL but callers currently using RBL can switch to rbrr_load directly. The kindle-once guard means rbrn_load is single-use per process which is fine for current callers. | Verify: No Rust build needed per paddock -- bash only heat

**[260210-0946] rough**

Consolidate nameplate and repo regime loading into canonical functions in their respective _regime.sh files.

## Functions to Create

### In `rbrn_regime.sh`

1. **`rbrn_load <moniker>`** — Canonical entry point: construct path to `rbrn_<moniker>.env`, verify exists, source, kindle, validate. Single call replaces the 4-6 line pattern repeated across callers.

2. **`rbrn_list`** — Enumerate concrete nameplate monikers by globbing `rbrn_*.env` and stripping prefix/suffix. No loaded regime prerequisite — this is a regime primitive for "what instances exist?"

### In `rbrr_regime.sh`

3. **`rbrr_load`** — Source `rbrr_RecipeBottleRegimeRepo.sh`, kindle. Encapsulates path construction and existence check. Not all callers need RBRR (e.g., rbob_cli doesn't always), so this stays separate from rbrn_load.

## Callsites to Refactor

### rbrn_load consolidation (4 sites):
- `rbt_testbench.sh:54-62` — construct path, exists, source, kindle, validate
- `rbob_cli.sh:108-112` — construct path, exists, source, kindle, validate
- `rbrn_cli.sh:47-48` — source + kindle (validate cmd)
- `rbrn_cli.sh:78-80` — source + kindle (render cmd)

### rbrr_load consolidation (8 sites):
- `rbt_testbench.sh:68-71` — construct path, exists, source, kindle
- `rbob_cli.sh:115-118` — construct path, exists, source, kindle
- `rbf_cli.sh:49-52` — via RBL, exists, source, kindle
- `rbgg_cli.sh:43-46` — via RBL, exists, source
- `rbgd_DepotConstants.sh:33-37` — via RBL, source, kindle, validate
- `rbrv_cli.sh:47-50` — conditional source (validate)
- `rbrv_cli.sh:85-88` — conditional source (render)
- `trbim_suite.sh:27` — source RBRR

### rbrn_list consolidation (1 site):
- `rbw_workbench.sh:123-132` — inline glob with strip logic

## Notes

- rbrn_cli.sh callers receive the file path externally; rbrn_load takes a moniker. Decide whether rbrn_load is moniker-only or also accepts a path, or whether rbrn_cli uses a lower-level internal.
- Some RBRR callers go through RBL (rbl_Locator.sh) for path resolution. Decide whether rbrr_load encapsulates RBL or sits alongside it.
- Mechanical refactoring — pattern is clear from existing code.

### create-rbcr-regime-render (₢APAAh) [complete]

**[260212-0934] complete**

Create shared regime render module `rbcr_render.sh` and refactor rbrn_cli.sh and rbrv_cli.sh to use it.

## Context

Current render code in rbrn_cli.sh and rbrv_cli.sh uses raw printf with hardcoded widths, duplicated color codes, and no awareness of terminal size or field gating. Design discussion established a shared utility approach: formatting mechanics are shared, editorial decisions (section ordering, grouping) stay manual per regime CLI.

## Architecture

New BCG module `Tools/rbw/rbcr_render.sh` with `zrbcr_kindle`:

### Kindle Constants
- ZRBCR_TERM_COLS — from BURD_TERM_COLS (terminal width ferried by dispatch)
- ZRBCR_LAYOUT — "single" (wide terminal, >=120 cols) or "double" (narrow, <120 cols)

### Public Functions

**rbcr_section_begin TITLE [GATE_VAR GATE_VALUE]**
- Prints section header
- Optional gate: evaluates `${!GATE_VAR}` against GATE_VALUE via indirect expansion
- If gate satisfied or ungated: sets ZRBCR_SECTION_ACTIVE=1, resets suppressed accumulator
- If gate not satisfied: prints one dimmed collapsed reminder line with gate state, sets ZRBCR_SECTION_ACTIVE=0

**rbcr_section_end**
- If section was active and had suppressed fields: prints one dimmed summary line listing suppressed var names and gate reason
- If section was inactive: no-op (collapsed reminder already printed by section_begin)
- Resets section state

**rbcr_line VARNAME TYPE REQ_STATUS DESCRIPTION**
- VARNAME: unquoted regime variable name (e.g., RBRN_ENTRY_MODE)
- TYPE: unquoted type badge (xname, string, fqin, port, ipv4, decimal, enum, cidr_list, domain_list)
- REQ_STATUS: unquoted — req, opt, or cond
- DESCRIPTION: quoted human prose
- If ZRBCR_SECTION_ACTIVE=0: early return (section collapsed)
- If REQ_STATUS=cond and section gate was not satisfied: appends to suppressed accumulator, returns
- Reads ZRBCR_LAYOUT to choose format:
  - single: VARNAME value [type] [req] description — one line
  - double: VARNAME value on line 1, type+req+description on line 2

### Gate Mechanism
- Entirely generic — uses bash indirect expansion ${!var}, no regime-specific knowledge
- rbcr_section_begin IS the enroll: evaluates gate, stores result in module state
- rbcr_line reads module state, no gate parameters needed on individual lines
- All parameters unquoted except description (controlled constants, always [A-Z0-9_])

## Work

### 1. Add BURD_TERM_COLS to dispatch
Add terminal width detection to BURD constants (burd_dispatch.sh or launcher). Value: `tput cols 2>/dev/null || echo 80`.

### 2. Create rbcr_render.sh
New BCG module in Tools/rbw/ with zrbcr_kindle, rbcr_section_begin, rbcr_section_end, rbcr_line. Two layout modes keyed on terminal width.

### 3. Refactor rbrn_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Service Identity (ungated)
- Container Image Configuration (ungated)
- Entry Service Configuration (gated: RBRN_ENTRY_MODE enabled)
- Enclave Network Configuration (ungated)
- Uplink Core (ungated)
- Uplink DNS Allowlist (gated: RBRN_UPLINK_DNS_MODE allowlist)
- Uplink Access Allowlist (gated: RBRN_UPLINK_ACCESS_MODE allowlist)
- Volume Mount Configuration (ungated)
- Unexpected variables block at end (reads ZRBRN_UNEXPECTED from kindle, unchanged)

### 4. Refactor rbrv_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Vessel Identity (ungated)
- Binding Configuration (ungated — or potentially gated if mode field added later)
- Conjuring Configuration (ungated)
- Unexpected variables block at end

### 5. Remove duplicate render_field functions
Delete zrbrn_render_field and zrbrv_render_field — replaced by shared rbcr_line.

### 6. Verify both renders
Run all four tabtargets (rnr, rnv, rvr, rvv) with real data. Confirm: gated sections collapse correctly, layouts adapt to terminal width, validation still works.

## Conventions
- All rbcr function parameters unquoted except description strings
- Gate variable names are bare (RBRN_ENTRY_MODE not "RBRN_ENTRY_MODE")
- Module prefix: rbcr_ (public), zrbcr_ (private)
- Kindle constant prefix: ZRBCR_

**[260212-0825] rough**

Create shared regime render module `rbcr_render.sh` and refactor rbrn_cli.sh and rbrv_cli.sh to use it.

## Context

Current render code in rbrn_cli.sh and rbrv_cli.sh uses raw printf with hardcoded widths, duplicated color codes, and no awareness of terminal size or field gating. Design discussion established a shared utility approach: formatting mechanics are shared, editorial decisions (section ordering, grouping) stay manual per regime CLI.

## Architecture

New BCG module `Tools/rbw/rbcr_render.sh` with `zrbcr_kindle`:

### Kindle Constants
- ZRBCR_TERM_COLS — from BURD_TERM_COLS (terminal width ferried by dispatch)
- ZRBCR_LAYOUT — "single" (wide terminal, >=120 cols) or "double" (narrow, <120 cols)

### Public Functions

**rbcr_section_begin TITLE [GATE_VAR GATE_VALUE]**
- Prints section header
- Optional gate: evaluates `${!GATE_VAR}` against GATE_VALUE via indirect expansion
- If gate satisfied or ungated: sets ZRBCR_SECTION_ACTIVE=1, resets suppressed accumulator
- If gate not satisfied: prints one dimmed collapsed reminder line with gate state, sets ZRBCR_SECTION_ACTIVE=0

**rbcr_section_end**
- If section was active and had suppressed fields: prints one dimmed summary line listing suppressed var names and gate reason
- If section was inactive: no-op (collapsed reminder already printed by section_begin)
- Resets section state

**rbcr_line VARNAME TYPE REQ_STATUS DESCRIPTION**
- VARNAME: unquoted regime variable name (e.g., RBRN_ENTRY_MODE)
- TYPE: unquoted type badge (xname, string, fqin, port, ipv4, decimal, enum, cidr_list, domain_list)
- REQ_STATUS: unquoted — req, opt, or cond
- DESCRIPTION: quoted human prose
- If ZRBCR_SECTION_ACTIVE=0: early return (section collapsed)
- If REQ_STATUS=cond and section gate was not satisfied: appends to suppressed accumulator, returns
- Reads ZRBCR_LAYOUT to choose format:
  - single: VARNAME value [type] [req] description — one line
  - double: VARNAME value on line 1, type+req+description on line 2

### Gate Mechanism
- Entirely generic — uses bash indirect expansion ${!var}, no regime-specific knowledge
- rbcr_section_begin IS the enroll: evaluates gate, stores result in module state
- rbcr_line reads module state, no gate parameters needed on individual lines
- All parameters unquoted except description (controlled constants, always [A-Z0-9_])

## Work

### 1. Add BURD_TERM_COLS to dispatch
Add terminal width detection to BURD constants (burd_dispatch.sh or launcher). Value: `tput cols 2>/dev/null || echo 80`.

### 2. Create rbcr_render.sh
New BCG module in Tools/rbw/ with zrbcr_kindle, rbcr_section_begin, rbcr_section_end, rbcr_line. Two layout modes keyed on terminal width.

### 3. Refactor rbrn_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Service Identity (ungated)
- Container Image Configuration (ungated)
- Entry Service Configuration (gated: RBRN_ENTRY_MODE enabled)
- Enclave Network Configuration (ungated)
- Uplink Core (ungated)
- Uplink DNS Allowlist (gated: RBRN_UPLINK_DNS_MODE allowlist)
- Uplink Access Allowlist (gated: RBRN_UPLINK_ACCESS_MODE allowlist)
- Volume Mount Configuration (ungated)
- Unexpected variables block at end (reads ZRBRN_UNEXPECTED from kindle, unchanged)

### 4. Refactor rbrv_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Vessel Identity (ungated)
- Binding Configuration (ungated — or potentially gated if mode field added later)
- Conjuring Configuration (ungated)
- Unexpected variables block at end

### 5. Remove duplicate render_field functions
Delete zrbrn_render_field and zrbrv_render_field — replaced by shared rbcr_line.

### 6. Verify both renders
Run all four tabtargets (rnr, rnv, rvr, rvv) with real data. Confirm: gated sections collapse correctly, layouts adapt to terminal width, validation still works.

## Conventions
- All rbcr function parameters unquoted except description strings
- Gate variable names are bare (RBRN_ENTRY_MODE not "RBRN_ENTRY_MODE")
- Module prefix: rbcr_ (public), zrbcr_ (private)
- Kindle constant prefix: ZRBCR_

**[260212-0825] rough**

Create shared regime render module `rbcr_render.sh` and refactor rbrn_cli.sh and rbrv_cli.sh to use it.

## Context

Current render code in rbrn_cli.sh and rbrv_cli.sh uses raw printf with hardcoded widths, duplicated color codes, and no awareness of terminal size or field gating. Design discussion established a shared utility approach: formatting mechanics are shared, editorial decisions (section ordering, grouping) stay manual per regime CLI.

## Architecture

New BCG module `Tools/rbw/rbcr_render.sh` with `zrbcr_kindle`:

### Kindle Constants
- ZRBCR_TERM_COLS — from BURD_TERM_COLS (terminal width ferried by dispatch)
- ZRBCR_LAYOUT — "single" (wide terminal, >=120 cols) or "double" (narrow, <120 cols)

### Public Functions

**rbcr_section_begin TITLE [GATE_VAR GATE_VALUE]**
- Prints section header
- Optional gate: evaluates `${!GATE_VAR}` against GATE_VALUE via indirect expansion
- If gate satisfied or ungated: sets ZRBCR_SECTION_ACTIVE=1, resets suppressed accumulator
- If gate not satisfied: prints one dimmed collapsed reminder line with gate state, sets ZRBCR_SECTION_ACTIVE=0

**rbcr_section_end**
- If section was active and had suppressed fields: prints one dimmed summary line listing suppressed var names and gate reason
- If section was inactive: no-op (collapsed reminder already printed by section_begin)
- Resets section state

**rbcr_line VARNAME TYPE REQ_STATUS DESCRIPTION**
- VARNAME: unquoted regime variable name (e.g., RBRN_ENTRY_MODE)
- TYPE: unquoted type badge (xname, string, fqin, port, ipv4, decimal, enum, cidr_list, domain_list)
- REQ_STATUS: unquoted — req, opt, or cond
- DESCRIPTION: quoted human prose
- If ZRBCR_SECTION_ACTIVE=0: early return (section collapsed)
- If REQ_STATUS=cond and section gate was not satisfied: appends to suppressed accumulator, returns
- Reads ZRBCR_LAYOUT to choose format:
  - single: VARNAME value [type] [req] description — one line
  - double: VARNAME value on line 1, type+req+description on line 2

### Gate Mechanism
- Entirely generic — uses bash indirect expansion ${!var}, no regime-specific knowledge
- rbcr_section_begin IS the enroll: evaluates gate, stores result in module state
- rbcr_line reads module state, no gate parameters needed on individual lines
- All parameters unquoted except description (controlled constants, always [A-Z0-9_])

## Work

### 1. Add BURD_TERM_COLS to dispatch
Add terminal width detection to BURD constants (burd_dispatch.sh or launcher). Value: `tput cols 2>/dev/null || echo 80`.

### 2. Create rbcr_render.sh
New BCG module in Tools/rbw/ with zrbcr_kindle, rbcr_section_begin, rbcr_section_end, rbcr_line. Two layout modes keyed on terminal width.

### 3. Refactor rbrn_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Service Identity (ungated)
- Container Image Configuration (ungated)
- Entry Service Configuration (gated: RBRN_ENTRY_MODE enabled)
- Enclave Network Configuration (ungated)
- Uplink Core (ungated)
- Uplink DNS Allowlist (gated: RBRN_UPLINK_DNS_MODE allowlist)
- Uplink Access Allowlist (gated: RBRN_UPLINK_ACCESS_MODE allowlist)
- Volume Mount Configuration (ungated)
- Unexpected variables block at end (reads ZRBRN_UNEXPECTED from kindle, unchanged)

### 4. Refactor rbrv_cli.sh render
Replace raw printf rendering with rbcr calls. Section decomposition:
- Core Vessel Identity (ungated)
- Binding Configuration (ungated — or potentially gated if mode field added later)
- Conjuring Configuration (ungated)
- Unexpected variables block at end

### 5. Remove duplicate render_field functions
Delete zrbrn_render_field and zrbrv_render_field — replaced by shared rbcr_line.

### 6. Verify both renders
Run all four tabtargets (rnr, rnv, rvr, rvv) with real data. Confirm: gated sections collapse correctly, layouts adapt to terminal width, validation still works.

## Conventions
- All rbcr function parameters unquoted except description strings
- Gate variable names are bare (RBRN_ENTRY_MODE not "RBRN_ENTRY_MODE")
- Module prefix: rbcr_ (public), zrbcr_ (private)
- Kindle constant prefix: ZRBCR_

**[260210-1114] rough**

Review and refine rbrn_cli.sh and rbrv_cli.sh render output formatting.

## Context

Sections 1-4 of the original regime-load-cleanup docket are complete and committed. This pace now focuses solely on the render appearance review that was deferred.

## Work

1. Run rbrn_cli.sh render with a real nameplate (e.g., nsproto) and review output formatting
2. Run rbrv_cli.sh render with a real vessel and review output formatting
3. Collect specific appearance nudges: alignment, color usage, field grouping, whitespace, header style
4. Apply agreed refinements

## Notes

- The render/validate architecture was established in ₢ATAAG (refactor-rbrv-rbrn-render-validate)
- This is cosmetic polish, not structural changes
- User input required to identify what needs nudging

**[260210-1114] rough**

Review and refine rbrn_cli.sh and rbrv_cli.sh render output formatting.

## Context

Sections 1-4 of the original regime-load-cleanup docket are complete and committed. This pace now focuses solely on the render appearance review that was deferred.

## Work

1. Run rbrn_cli.sh render with a real nameplate (e.g., nsproto) and review output formatting
2. Run rbrv_cli.sh render with a real vessel and review output formatting
3. Collect specific appearance nudges: alignment, color usage, field grouping, whitespace, header style
4. Apply agreed refinements

## Notes

- The render/validate architecture was established in ₢ATAAG (refactor-rbrv-rbrn-render-validate)
- This is cosmetic polish, not structural changes
- User input required to identify what needs nudging

**[260210-1054] rough**

Clean up regime load primitives from APAAg: eliminate magic strings, fix bugs, create RBCC constants module, and refactor missed callsites.

## Completed

### 1. Created RBCC Constants Module
New BCG module `rbcc_Constants.sh` in `Tools/rbw/` with zrbcc_kindle() providing:
- RBCC_KIT_DIR, RBCC_RBRN_PREFIX, RBCC_RBRN_EXT, RBCC_RBRR_FILE
- BURC_TOOLS_DIR guard (dies if not set)

### 2. Fixed BURC_PROJECT_ROOT Bug
- rbrr_regime.sh: rbrr_load uses RBCC_RBRR_FILE instead of broken BURC_PROJECT_ROOT path
- rbrv_cli.sh: both conditional checks use RBCC_RBRR_FILE; added missing source rbrr_regime.sh

### 3. Replaced Magic Strings in rbrn_regime.sh
- rbrn_load_moniker: path construction uses RBCC_KIT_DIR/RBCC_RBRN_PREFIX/RBCC_RBRN_EXT
- rbrn_list: glob pattern and prefix/extension stripping use RBCC constants

### 4. Refactored Missed Callsites
- rbgm_cli.sh: RBL-based RBRR loading replaced with rbrr_load (RBL kept for RBRP)
- rbgp_cli.sh: same refactor as rbgm

### Additional fixes from review
- rbf_cli.sh: added missing source rbrr_regime.sh (APAAg bug)
- rbgg_cli.sh, rbf_cli.sh: fixed sourcing order (rbcc before rbgd)
- rbgd_DepotConstants.sh: removed rbrr_load from zrbgd_kindle (double-kindle bug)
- rbrr_regime.sh: updated stale comment
- All callers of rbrn_load/rbrr_load: wired rbcc source + zrbcc_kindle

## Deferred

### 5. RBL Locator Retirement → ₢ATAAK (consolidate-rbrp-retire-rbl)
### 6. Makefile References — bare string acceptable, Makefile being deleted soon
### 7. Render Appearance Nudges — no nudges requested, closed

**[260210-1019] rough**

Clean up regime load primitives from APAAg: eliminate magic strings, fix bugs, create RBCC constants module, and refactor missed callsites.

## 1. Create RBCC Constants Module

New BCG module `rbcc_Constants.sh` in `Tools/rbw/` following `rbgc_Constants.sh` pattern:

```
zrbcc_kindle():
  RBCC_KIT_DIR="${BURC_TOOLS_DIR}/rbw"          # kit directory (rbw until renamed rbk)
  RBCC_RBRN_PREFIX="rbrn_"                      # nameplate file prefix
  RBCC_RBRN_EXT=".env"                          # nameplate file extension
  RBCC_RBRR_FILE="rbrr_RecipeBottleRegimeRepo.sh"  # RBRR assignment at project root
```

No CLI needed (library/utility module). Must be kindled before rbrn_load/rbrr_load are called.

## 2. Fix BURC_PROJECT_ROOT Bug in rbrr_load

`rbrr_regime.sh:116` uses `${BURC_PROJECT_ROOT}/rbrr_RecipeBottleRegimeRepo.sh` but BURC_PROJECT_ROOT=.. is relative to burc.env location, not CWD. From bash CWD (project root), this resolves to parent directory. Fix: use `${RBCC_RBRR_FILE}` (bare relative from project root).

Same bug exists in `rbrv_cli.sh:47,84` conditional checks.

## 3. Replace Magic Strings in rbrn_regime.sh

In `rbrn_load` (line 203) and `rbrn_list` (line 228):
- Replace `"${BURC_TOOLS_DIR}/rbw/rbrn_"` with `"${RBCC_KIT_DIR}/${RBCC_RBRN_PREFIX}"`
- Replace `.env` with `${RBCC_RBRN_EXT}`
- Replace prefix strip `rbrn_` with `${RBCC_RBRN_PREFIX}`

## 4. Refactor Missed Callsites

The APAAg agent missed these rbrr callsites still using old RBL pattern:
- `rbgm_cli.sh:42-43` — validate + source via RBL_RBRR_FILE
- `rbgp_cli.sh:42-43` — validate + source via RBL_RBRR_FILE

Switch to `rbrr_load()`.

## 5. Assess RBL Locator Retirement

After all callers migrate to rbrr_load, check if rbl_Locator.sh has remaining consumers. If RBL_RBRR_FILE and RBL_RBRP_FILE are its only exports and all callers have migrated, flag for retirement.

## 6. Makefile References

`rbw.workbench.mk:35,42` uses bare `rbrr_RecipeBottleRegimeRepo.sh` string. These cant call bash functions. Assess whether Makefile should reference RBCC_RBRR_FILE or if bare string is acceptable in Makefile context.

## 7. Render Appearance Nudges

Review `rbrn_cli.sh` render and validate output formatting. Collect specific nudge requests from user during execution.

## Notes

- RBCC kindle must be added to furnish functions of all CLIs that call rbrn_load or rbrr_load
- The kindle graph change means sourcing rbcc_Constants.sh in CLI headers alongside other dependencies
- BCG rule: file paths ALL defined in kindle as module variables

### multi-nameplate-utilities (₢APAAf) [complete]

**[260212-1133] complete**

Design and implement multi-nameplate utilities — cross-cutting operations that act across all known nameplates.

## Prerequisite

Depends on ₢APAAg (consolidate-regime-load-primitives) which provides `rbrn_list` and `rbrn_load <moniker>`.

## Core Capabilities

### 1. Cross-Nameplate Validation

Scan all nameplates and detect configuration conflicts:

- **Port uniqueness**: Verify `RBRN_ENTRY_PORT_WORKSTATION` values are distinct across all nameplates. Two nameplates claiming port 8001 would cause socat bind failure at runtime — catch it statically.
- **Subnet non-overlap**: Verify `RBRN_ENCLAVE_BASE_IP`/`RBRN_ENCLAVE_NETMASK` ranges don't overlap. Current convention uses third-octet staggering (10.242.0.0, 10.242.1.0, 10.242.2.0) but nothing enforces this when adding a fourth nameplate.
- **Enclave IP uniqueness**: Verify sentry and bottle IPs don't collide across nameplates (currently .2/.3 within each /24, but validate).

Currently NO cross-nameplate validation exists anywhere in the codebase. Failures surface only at container runtime.

### 2. Bulk Ark Summon

"Make local match what all nameplates say" — iterate `rbrn_list`, load each, summon all declared consecrations. Courtesy download so that starting any bottle service doesn't block on ark retrieval.

### 3. Multi-Nameplate Status View

Show all nameplates with their key configuration at a glance:
- Moniker, workstation port, enclave port, subnet, sentry IP, bottle IP
- Ark consecration state (present locally or not)
- Entry mode (enabled/disabled)

### Current Port/IP Allocations (for reference)

| Moniker | WS Port | Enclave Port | Subnet | Sentry IP | Bottle IP |
|---------|---------|--------------|--------|-----------|-----------|
| nsproto | 8890 | 8888 | 10.242.0.0/24 | 10.242.0.2 | 10.242.0.3 |
| pluml | 8001 | 8080 | 10.242.1.0/24 | 10.242.1.2 | 10.242.1.3 |
| srjcl | 7999 | 8000 | 10.242.2.0/24 | 10.242.2.2 | 10.242.2.3 |

## Design Questions

1. Where do these utilities live? Options: new `rbnm_` module (nameplate-multi?), or extend `rbrn_cli.sh` with cross-nameplate subcommands.
2. Should validation run automatically on `rbrn_load` (catch conflicts early) or be an explicit command?
3. Bulk summon: iterate serially or parallel? Serial is simpler; parallel risks auth token contention.
4. What tabtargets expose these? Consider `rbw-nv.ValidateNameplates.sh`, `rbw-ns.NameplateStatus.sh`, `rbw-na.SummonAllArks.sh`.

## Notes

- This is a design pace — needs human judgment on module placement, CLI surface, and which validations are hard errors vs warnings.
- Does NOT change existing ark APIs (summon/beseech/abjure signatures stay as-is). This pace adds cross-cutting operations that use the existing per-nameplate ark functions.

**[260210-0947] rough**

Design and implement multi-nameplate utilities — cross-cutting operations that act across all known nameplates.

## Prerequisite

Depends on ₢APAAg (consolidate-regime-load-primitives) which provides `rbrn_list` and `rbrn_load <moniker>`.

## Core Capabilities

### 1. Cross-Nameplate Validation

Scan all nameplates and detect configuration conflicts:

- **Port uniqueness**: Verify `RBRN_ENTRY_PORT_WORKSTATION` values are distinct across all nameplates. Two nameplates claiming port 8001 would cause socat bind failure at runtime — catch it statically.
- **Subnet non-overlap**: Verify `RBRN_ENCLAVE_BASE_IP`/`RBRN_ENCLAVE_NETMASK` ranges don't overlap. Current convention uses third-octet staggering (10.242.0.0, 10.242.1.0, 10.242.2.0) but nothing enforces this when adding a fourth nameplate.
- **Enclave IP uniqueness**: Verify sentry and bottle IPs don't collide across nameplates (currently .2/.3 within each /24, but validate).

Currently NO cross-nameplate validation exists anywhere in the codebase. Failures surface only at container runtime.

### 2. Bulk Ark Summon

"Make local match what all nameplates say" — iterate `rbrn_list`, load each, summon all declared consecrations. Courtesy download so that starting any bottle service doesn't block on ark retrieval.

### 3. Multi-Nameplate Status View

Show all nameplates with their key configuration at a glance:
- Moniker, workstation port, enclave port, subnet, sentry IP, bottle IP
- Ark consecration state (present locally or not)
- Entry mode (enabled/disabled)

### Current Port/IP Allocations (for reference)

| Moniker | WS Port | Enclave Port | Subnet | Sentry IP | Bottle IP |
|---------|---------|--------------|--------|-----------|-----------|
| nsproto | 8890 | 8888 | 10.242.0.0/24 | 10.242.0.2 | 10.242.0.3 |
| pluml | 8001 | 8080 | 10.242.1.0/24 | 10.242.1.2 | 10.242.1.3 |
| srjcl | 7999 | 8000 | 10.242.2.0/24 | 10.242.2.2 | 10.242.2.3 |

## Design Questions

1. Where do these utilities live? Options: new `rbnm_` module (nameplate-multi?), or extend `rbrn_cli.sh` with cross-nameplate subcommands.
2. Should validation run automatically on `rbrn_load` (catch conflicts early) or be an explicit command?
3. Bulk summon: iterate serially or parallel? Serial is simpler; parallel risks auth token contention.
4. What tabtargets expose these? Consider `rbw-nv.ValidateNameplates.sh`, `rbw-ns.NameplateStatus.sh`, `rbw-na.SummonAllArks.sh`.

## Notes

- This is a design pace — needs human judgment on module placement, CLI surface, and which validations are hard errors vs warnings.
- Does NOT change existing ark APIs (summon/beseech/abjure signatures stay as-is). This pace adds cross-cutting operations that use the existing per-nameplate ark functions.

**[260210-0947] rough**

Design and implement multi-nameplate utilities — cross-cutting operations that act across all known nameplates.

## Prerequisite

Depends on ₢APAAg (consolidate-regime-load-primitives) which provides `rbrn_list` and `rbrn_load <moniker>`.

## Core Capabilities

### 1. Cross-Nameplate Validation

Scan all nameplates and detect configuration conflicts:

- **Port uniqueness**: Verify `RBRN_ENTRY_PORT_WORKSTATION` values are distinct across all nameplates. Two nameplates claiming port 8001 would cause socat bind failure at runtime — catch it statically.
- **Subnet non-overlap**: Verify `RBRN_ENCLAVE_BASE_IP`/`RBRN_ENCLAVE_NETMASK` ranges don't overlap. Current convention uses third-octet staggering (10.242.0.0, 10.242.1.0, 10.242.2.0) but nothing enforces this when adding a fourth nameplate.
- **Enclave IP uniqueness**: Verify sentry and bottle IPs don't collide across nameplates (currently .2/.3 within each /24, but validate).

Currently NO cross-nameplate validation exists anywhere in the codebase. Failures surface only at container runtime.

### 2. Bulk Ark Summon

"Make local match what all nameplates say" — iterate `rbrn_list`, load each, summon all declared consecrations. Courtesy download so that starting any bottle service doesn't block on ark retrieval.

### 3. Multi-Nameplate Status View

Show all nameplates with their key configuration at a glance:
- Moniker, workstation port, enclave port, subnet, sentry IP, bottle IP
- Ark consecration state (present locally or not)
- Entry mode (enabled/disabled)

### Current Port/IP Allocations (for reference)

| Moniker | WS Port | Enclave Port | Subnet | Sentry IP | Bottle IP |
|---------|---------|--------------|--------|-----------|-----------|
| nsproto | 8890 | 8888 | 10.242.0.0/24 | 10.242.0.2 | 10.242.0.3 |
| pluml | 8001 | 8080 | 10.242.1.0/24 | 10.242.1.2 | 10.242.1.3 |
| srjcl | 7999 | 8000 | 10.242.2.0/24 | 10.242.2.2 | 10.242.2.3 |

## Design Questions

1. Where do these utilities live? Options: new `rbnm_` module (nameplate-multi?), or extend `rbrn_cli.sh` with cross-nameplate subcommands.
2. Should validation run automatically on `rbrn_load` (catch conflicts early) or be an explicit command?
3. Bulk summon: iterate serially or parallel? Serial is simpler; parallel risks auth token contention.
4. What tabtargets expose these? Consider `rbw-nv.ValidateNameplates.sh`, `rbw-ns.NameplateStatus.sh`, `rbw-na.SummonAllArks.sh`.

## Notes

- This is a design pace — needs human judgment on module placement, CLI surface, and which validations are hard errors vs warnings.
- Does NOT change existing ark APIs (summon/beseech/abjure signatures stay as-is). This pace adds cross-cutting operations that use the existing per-nameplate ark functions.

**[260210-0302] rough**

Redesign the ark function API surface (summon, beseech, abjure) for nameplate-centric workflow.

## Primary Consideration: Consolidate Nameplate Loading

Three independent copies of nameplate loading exist today:

| Location | Function | Chain |
|----------|----------|-------|
| `rbt_testbench.sh:50` | `rbt_load_nameplate()` | source env → `zrbrn_kindle` → `zrbrn_validate_fields` → source RBRR → `zrbrr_kindle` → kindle RBOB |
| `rbob_cli.sh:107` | inline in `zrbob_furnish()` | source env → `zrbrn_kindle` → `zrbrn_validate_fields` (no RBRR) |
| `rbw_workbench.sh:118` | inline in routing | lists nameplates, delegates to `rbrn_cli.sh` (no loading) |

Before redesigning the ark API, extract canonical load functions into the regime files:

1. **`rbrn_load <moniker>`** in `rbrn_regime.sh` — source `rbrn_<moniker>.env`, kindle, validate. Single entry point for "give me a loaded nameplate."
2. **`rbrr_load`** in `rbrr_regime.sh` — source RBRR config, kindle. Separated because not all callers need RBRR (rbob_cli doesn't).

Then refactor all three call sites to use these canonical functions. This gives the ark functions a clean `rbrn_load` to call when they need nameplate context.

## Problem

Current API requires explicit consecration arguments for summon and abjure, but the natural workflow is nameplate-driven: the nameplate declares which consecrations are active, and operations should follow from that.

## Design Questions to Resolve

1. **Summon**: Should take a nameplate (or work from regime context) and pull whatever consecrations are declared there — no explicit consecration arg. "Make local match what nameplate says." Consider: summon both vessels at once vs per-vessel.

2. **Beseech**: Becomes the discovery/selection tool. Keep beseech as a pure registry listing tool. Consider a separate "refresh" concept for nameplate-aware comparison later.

3. **Abjure**: Stays by-consecration (deleting a specific version is inherently explicit), BUT must add a hard guard: refuse to delete any consecration currently referenced by a nameplate. No override flag — to delete a referenced consecration, update the nameplate first. Scan surface: `Tools/rbw/rbrn_*.env` files.

4. **Tabtarget implications**: Current `rbw-as.SummonArk.sh` takes `<vessel> <consecration>` — this signature changes. Consider whether vessel arg is even needed if nameplate provides both.

## Bugs Found During Investigation (already fixed)

- beseech vessel filter didn't strip path prefix (fixed: `${var##*/}`)
- `curl -X HEAD` vs `curl --head` in summon/abjure caused exit code 18 (fixed)

## Scope

- Extract `rbrn_load` and `rbrr_load` into regime files
- Refactor testbench, rbob_cli, workbench to use canonical load functions
- Design new API signatures for summon, beseech, abjure
- Update tabtarget arguments to match
- Update spec docs (RBSAS, RBSAB, RBSAA) to reflect new contracts
- Implement and test

## Notes

- This is a design pace — needs human judgment on API shape and regime function placement
- Asymmetric by nature: summon is nameplate-driven, abjure is consecration-driven with nameplate guard
- Regime consolidation is prerequisite infrastructure that unblocks the API redesign

**[260209-1727] rough**

Redesign the ark function API surface (summon, beseech, abjure) for nameplate-centric workflow.

## Problem

Current API requires explicit consecration arguments for summon and abjure, but the natural workflow is nameplate-driven: the nameplate declares which consecrations are active, and operations should follow from that.

## Design Questions to Resolve

1. **Summon**: Should take a nameplate (or work from regime context) and pull whatever consecrations are declared there — no explicit consecration arg. "Make local match what nameplate says." Consider: summon both vessels at once vs per-vessel.

2. **Beseech**: Becomes the discovery/selection tool. Should it output copy-pasteable nameplate lines? Interactive "install this consecration" mode? Both?

3. **Abjure**: Stays by-consecration (deleting a specific version is inherently explicit), BUT must add a safeguard: refuse to delete any consecration currently referenced by a nameplate. Scan surface: `Tools/rbw/rbrn_*.env` files.

4. **Tabtarget implications**: Current `rbw-as.SummonArk.sh` takes `<vessel> <consecration>` — this signature changes. Consider whether vessel arg is even needed if nameplate provides both.

## Bugs Found During Investigation (already fixed)

- beseech vessel filter didn't strip path prefix (fixed: `${var##*/}`)
- `curl -X HEAD` vs `curl --head` in summon/abjure caused exit code 18 (fixed)

## Scope

- Design the new API signatures for all three functions
- Update tabtarget arguments to match
- Update spec docs (RBSAS, RBSAB, RBSAA) to reflect new contracts
- Implement and test

## Notes

- This is a design pace — needs human judgment on API shape
- Asymmetric by nature: summon is nameplate-driven, abjure is consecration-driven with nameplate guard

### rbsa-local-ops-axla-parity (₢APAAV) [abandoned]

**[260208-1514] abandoned**

Bring RBSA Local Operations section to GCP-operations parity for AXLA voicing.

## Background (resolved from exploration)

Audit findings from mount session:
- All 33 RBS subfiles already exist and are include::'d into RBS0-SpecTop.adoc
- GCP operation sections in RBSA have two-layer AXLA pattern (parent voicing + subfile sections)
- Local Operations section in RBSA has NEITHER layer
- Subfile mappings stay in parent (established pattern, no merge needed)
- Naming pattern established (RBSXX-name.adoc)

## Scope

Add AXLA annotations to bring Local Operations to the same standard as GCP Operations:

### Layer 1: Parent voicing annotations in RBS0-SpecTop.adoc

Add `// ⟦axl_voices ...⟧` between anchor and heading for each Local Operation:
- opss_sentry_start, mkr_network_create, mkr_sentry_run, mkr_network_connect
- opbs_bottle_start, mkr_bottle_cleanup, mkr_bottle_launch
- opbr_bottle_run, mkr_bottle_create, mkr_command_exec
- ops_rbv_check, ops_rbv_mirror
- scr_security_config, scr_iptables_init, scr_port_setup, scr_access_setup, scr_dns_step
- mkc_interface_check

Voicing types to assign:
- Top-level ops (opss_, opbs_, opbr_): axo_command or axo_sequence with appropriate axe_ and axd_
- Sub-sequences (mkr_*): axo_sequence (they're ordered steps within a parent)
- Script phases (scr_*): axo_command axe_bash_scripted axd_transient
- Utility (mkc_*): likely axo_routine

### Layer 2: Section annotations in deployment subfiles

Add `// ⟦axs_*⟧` markers to each deployment subfile that lacks them:
- RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL, RBSBR, RBSBC, RBSCE
- RBSVC, RBSVM
- RBSSC, RBSIP, RBSPT, RBSAX, RBSDS

Pattern to follow (from GCP subfiles):
- // ⟦axs_inputs⟧ or // ⟦axs_inputs axd_none⟧
- // ⟦axs_behavior⟧
- // ⟦axs_outputs⟧ or // ⟦axs_outputs axd_none⟧
- // ⟦axs_completion⟧

## Not in scope
- Rewriting stub subfiles (RBSBS 1-line, RBSBR 1-line) — content quality is a separate pace
- Deprecating RBS-Specification.adoc — separate decision
- Vocabulary divergence (cmk_recipe_line vs "steps") — separate pace
- Adding mapping sections to subfiles — established pattern keeps them in parent

## Acceptance criteria
- Every Local Operation anchor in RBS0-SpecTop.adoc has a voicing annotation
- Every deployment subfile has axs_* section markers
- Pattern matches what GCP operations already do

**[260208-1506] rough**

Bring RBSA Local Operations section to GCP-operations parity for AXLA voicing.

## Background (resolved from exploration)

Audit findings from mount session:
- All 33 RBS subfiles already exist and are include::'d into RBS0-SpecTop.adoc
- GCP operation sections in RBSA have two-layer AXLA pattern (parent voicing + subfile sections)
- Local Operations section in RBSA has NEITHER layer
- Subfile mappings stay in parent (established pattern, no merge needed)
- Naming pattern established (RBSXX-name.adoc)

## Scope

Add AXLA annotations to bring Local Operations to the same standard as GCP Operations:

### Layer 1: Parent voicing annotations in RBS0-SpecTop.adoc

Add `// ⟦axl_voices ...⟧` between anchor and heading for each Local Operation:
- opss_sentry_start, mkr_network_create, mkr_sentry_run, mkr_network_connect
- opbs_bottle_start, mkr_bottle_cleanup, mkr_bottle_launch
- opbr_bottle_run, mkr_bottle_create, mkr_command_exec
- ops_rbv_check, ops_rbv_mirror
- scr_security_config, scr_iptables_init, scr_port_setup, scr_access_setup, scr_dns_step
- mkc_interface_check

Voicing types to assign:
- Top-level ops (opss_, opbs_, opbr_): axo_command or axo_sequence with appropriate axe_ and axd_
- Sub-sequences (mkr_*): axo_sequence (they're ordered steps within a parent)
- Script phases (scr_*): axo_command axe_bash_scripted axd_transient
- Utility (mkc_*): likely axo_routine

### Layer 2: Section annotations in deployment subfiles

Add `// ⟦axs_*⟧` markers to each deployment subfile that lacks them:
- RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL, RBSBR, RBSBC, RBSCE
- RBSVC, RBSVM
- RBSSC, RBSIP, RBSPT, RBSAX, RBSDS

Pattern to follow (from GCP subfiles):
- // ⟦axs_inputs⟧ or // ⟦axs_inputs axd_none⟧
- // ⟦axs_behavior⟧
- // ⟦axs_outputs⟧ or // ⟦axs_outputs axd_none⟧
- // ⟦axs_completion⟧

## Not in scope
- Rewriting stub subfiles (RBSBS 1-line, RBSBR 1-line) — content quality is a separate pace
- Deprecating RBS-Specification.adoc — separate decision
- Vocabulary divergence (cmk_recipe_line vs "steps") — separate pace
- Adding mapping sections to subfiles — established pattern keeps them in parent

## Acceptance criteria
- Every Local Operation anchor in RBS0-SpecTop.adoc has a voicing annotation
- Every deployment subfile has axs_* section markers
- Pattern matches what GCP operations already do

**[260208-1506] rough**

Bring RBSA Local Operations section to GCP-operations parity for AXLA voicing.

## Background (resolved from exploration)

Audit findings from mount session:
- All 33 RBS subfiles already exist and are include::'d into RBS0-SpecTop.adoc
- GCP operation sections in RBSA have two-layer AXLA pattern (parent voicing + subfile sections)
- Local Operations section in RBSA has NEITHER layer
- Subfile mappings stay in parent (established pattern, no merge needed)
- Naming pattern established (RBSXX-name.adoc)

## Scope

Add AXLA annotations to bring Local Operations to the same standard as GCP Operations:

### Layer 1: Parent voicing annotations in RBS0-SpecTop.adoc

Add `// ⟦axl_voices ...⟧` between anchor and heading for each Local Operation:
- opss_sentry_start, mkr_network_create, mkr_sentry_run, mkr_network_connect
- opbs_bottle_start, mkr_bottle_cleanup, mkr_bottle_launch
- opbr_bottle_run, mkr_bottle_create, mkr_command_exec
- ops_rbv_check, ops_rbv_mirror
- scr_security_config, scr_iptables_init, scr_port_setup, scr_access_setup, scr_dns_step
- mkc_interface_check

Voicing types to assign:
- Top-level ops (opss_, opbs_, opbr_): axo_command or axo_sequence with appropriate axe_ and axd_
- Sub-sequences (mkr_*): axo_sequence (they're ordered steps within a parent)
- Script phases (scr_*): axo_command axe_bash_scripted axd_transient
- Utility (mkc_*): likely axo_routine

### Layer 2: Section annotations in deployment subfiles

Add `// ⟦axs_*⟧` markers to each deployment subfile that lacks them:
- RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL, RBSBR, RBSBC, RBSCE
- RBSVC, RBSVM
- RBSSC, RBSIP, RBSPT, RBSAX, RBSDS

Pattern to follow (from GCP subfiles):
- // ⟦axs_inputs⟧ or // ⟦axs_inputs axd_none⟧
- // ⟦axs_behavior⟧
- // ⟦axs_outputs⟧ or // ⟦axs_outputs axd_none⟧
- // ⟦axs_completion⟧

## Not in scope
- Rewriting stub subfiles (RBSBS 1-line, RBSBR 1-line) — content quality is a separate pace
- Deprecating RBS-Specification.adoc — separate decision
- Vocabulary divergence (cmk_recipe_line vs "steps") — separate pace
- Adding mapping sections to subfiles — established pattern keeps them in parent

## Acceptance criteria
- Every Local Operation anchor in RBS0-SpecTop.adoc has a voicing annotation
- Every deployment subfile has axs_* section markers
- Pattern matches what GCP operations already do

**[260130-0804] rough**

Drafted from ₢ARAAF in ₣AR.

Explore partitioning RBS procedural specifications into AXLA-consistent subsections.

## Purpose
Preparatory step toward RBSA (eventual RBAGS + RBS consolidation). Make RBS procedure documentation follow AXLA patterns like RBAGS subfiles do.

## Exploration
1. Read AXLA thoroughly — understand axs_* section motifs, axo_procedure patterns
2. Review how RBAGS partitions content into subfiles (RBSAA, RBSOB, etc.)
3. Identify RBS procedure sections that could become standalone subfiles

## Implementation approach
- Partition procedural specs into subfiles following RBAGS naming pattern
- Add AXLA-consistent voicings (axs_inputs, axs_behavior, axs_outputs, axs_completion)
- Use annotations: // ⟦axl_voices axo_command axd_transient⟧

## Concerns to address
(Future implementer: address these before proceeding)
- How do subfile mappings merge with parent document mappings?
- Should subfiles be include::'d or kept as reference documents?
- What naming pattern for RBS subfiles? (RBSGS pattern vs new pattern)

## Naming discipline
Subfiles MUST follow naming practices established by RBAGS subfiles. Review existing pattern before creating new files.

**[260130-0740] rough**

Explore partitioning RBS procedural specifications into AXLA-consistent subsections.

## Purpose
Preparatory step toward RBSA (eventual RBAGS + RBS consolidation). Make RBS procedure documentation follow AXLA patterns like RBAGS subfiles do.

## Exploration
1. Read AXLA thoroughly — understand axs_* section motifs, axo_procedure patterns
2. Review how RBAGS partitions content into subfiles (RBSAA, RBSOB, etc.)
3. Identify RBS procedure sections that could become standalone subfiles

## Implementation approach
- Partition procedural specs into subfiles following RBAGS naming pattern
- Add AXLA-consistent voicings (axs_inputs, axs_behavior, axs_outputs, axs_completion)
- Use annotations: // ⟦axl_voices axo_command axd_transient⟧

## Concerns to address
(Future implementer: address these before proceeding)
- How do subfile mappings merge with parent document mappings?
- Should subfiles be include::'d or kept as reference documents?
- What naming pattern for RBS subfiles? (RBSGS pattern vs new pattern)

## Naming discipline
Subfiles MUST follow naming practices established by RBAGS subfiles. Review existing pattern before creating new files.

### create-rbsco-cosmology-overview (₢APAAU) [complete]

**[260208-1522] complete**

Create RBSCO-CosmologyIntro.adoc — the nosebleed-terse "what Recipe Bottle is" overview.

## Purpose
Standalone subdoc in `lenses/` serving as the index.html frontmatter source for the
whole Recipe Bottle project (scaleinv.github.io/recipebottle). This is the public face:
elevator-pitch cosmology, not a procedural walkthrough (that's RBSGS).

## Ancestry
Absorbs and supersedes `index.adoc` (removed from working tree, last committed 84323c31
on 2025-08-18). Full git lineage: 31 commits from 2642121c through 84323c31.
Recover latest content via `git show 84323c31:index.adoc`.

## Content to include
- **Executive vision**: Containing untrusted workloads safely
- **The metaphor**: Vessel, Ark (-image + -about), Sentry, Bottle, Censer
- **Two-part architecture vision**:
  - Part One: Image Management (GCB/GAR, SBOMs, metadata artifacts, multi-arch builds)
  - Part Two: Bottle Service Orchestration (Sentry + Censer + Bottle trio)
- **Trust model**: Verify the ark's -about before deploying the -image
- **Architecture diagram reference** (rbm-abstract-drawio.svg)
- **Significant Events timeline** (preserve commented-out entries for history)
- **Comment marking**: Note that this section is the index.html frontmatter source

## Mapping discipline
Use RBS0-SpecTop.adoc's existing `at_`/`rbtga_` mapping section — no duplicate definitions.
Add Censer Container terms to RBSA mappings if not already present.
Reference existing `[[term_*]]` anchors; define new ones only for concepts not yet in RBSA.

## What this is NOT
- Not a procedural guide (that's RBSGS)
- Not independently compilable yet (future tooling will handle `include::` extraction)
- Not a duplicate of RBSA definitions — references them

## File
`lenses/RBSCO-CosmologyIntro.adoc`

**[260208-1453] rough**

Create RBSCO-CosmologyIntro.adoc — the nosebleed-terse "what Recipe Bottle is" overview.

## Purpose
Standalone subdoc in `lenses/` serving as the index.html frontmatter source for the
whole Recipe Bottle project (scaleinv.github.io/recipebottle). This is the public face:
elevator-pitch cosmology, not a procedural walkthrough (that's RBSGS).

## Ancestry
Absorbs and supersedes `index.adoc` (removed from working tree, last committed 84323c31
on 2025-08-18). Full git lineage: 31 commits from 2642121c through 84323c31.
Recover latest content via `git show 84323c31:index.adoc`.

## Content to include
- **Executive vision**: Containing untrusted workloads safely
- **The metaphor**: Vessel, Ark (-image + -about), Sentry, Bottle, Censer
- **Two-part architecture vision**:
  - Part One: Image Management (GCB/GAR, SBOMs, metadata artifacts, multi-arch builds)
  - Part Two: Bottle Service Orchestration (Sentry + Censer + Bottle trio)
- **Trust model**: Verify the ark's -about before deploying the -image
- **Architecture diagram reference** (rbm-abstract-drawio.svg)
- **Significant Events timeline** (preserve commented-out entries for history)
- **Comment marking**: Note that this section is the index.html frontmatter source

## Mapping discipline
Use RBS0-SpecTop.adoc's existing `at_`/`rbtga_` mapping section — no duplicate definitions.
Add Censer Container terms to RBSA mappings if not already present.
Reference existing `[[term_*]]` anchors; define new ones only for concepts not yet in RBSA.

## What this is NOT
- Not a procedural guide (that's RBSGS)
- Not independently compilable yet (future tooling will handle `include::` extraction)
- Not a duplicate of RBSA definitions — references them

## File
`lenses/RBSCO-CosmologyIntro.adoc`

**[260208-1453] rough**

Create RBSCO-CosmologyIntro.adoc — the nosebleed-terse "what Recipe Bottle is" overview.

## Purpose
Standalone subdoc in `lenses/` serving as the index.html frontmatter source for the
whole Recipe Bottle project (scaleinv.github.io/recipebottle). This is the public face:
elevator-pitch cosmology, not a procedural walkthrough (that's RBSGS).

## Ancestry
Absorbs and supersedes `index.adoc` (removed from working tree, last committed 84323c31
on 2025-08-18). Full git lineage: 31 commits from 2642121c through 84323c31.
Recover latest content via `git show 84323c31:index.adoc`.

## Content to include
- **Executive vision**: Containing untrusted workloads safely
- **The metaphor**: Vessel, Ark (-image + -about), Sentry, Bottle, Censer
- **Two-part architecture vision**:
  - Part One: Image Management (GCB/GAR, SBOMs, metadata artifacts, multi-arch builds)
  - Part Two: Bottle Service Orchestration (Sentry + Censer + Bottle trio)
- **Trust model**: Verify the ark's -about before deploying the -image
- **Architecture diagram reference** (rbm-abstract-drawio.svg)
- **Significant Events timeline** (preserve commented-out entries for history)
- **Comment marking**: Note that this section is the index.html frontmatter source

## Mapping discipline
Use RBS0-SpecTop.adoc's existing `at_`/`rbtga_` mapping section — no duplicate definitions.
Add Censer Container terms to RBSA mappings if not already present.
Reference existing `[[term_*]]` anchors; define new ones only for concepts not yet in RBSA.

## What this is NOT
- Not a procedural guide (that's RBSGS)
- Not independently compilable yet (future tooling will handle `include::` extraction)
- Not a duplicate of RBSA definitions — references them

## File
`lenses/RBSCO-CosmologyIntro.adoc`

**[260128-2119] rough**

Introduce the Recipe Bottle metaphor and trust model in Getting Started.

## Context
The Getting Started guide is procedural but lacks conceptual grounding in the metaphor.

## Content to add
- **Why "Recipe Bottle"**: Containing potentially dangerous workloads (demons) safely
- **Vessel**: Mystical artifact designed to contain demons
- **Ark**: A specific manifestation of a vessel, producing:
  - `-image`: the deployable prison
  - `-about`: provenance record for trust verification
- **Sentry**: Guardian watching the containment
- **Bottle**: The prison holding the demon
- **Censer**: Bottle configured to see only the sentry's smoke (routing through sentry)
- **Trust model**: Verify the ark's -about before deploying the -image

## Location
`lenses/RBSGS-GettingStarted.adoc` — add as opening section before "Depots and Roles"

## Relationship
This narrative introduction complements the formal vocabulary definitions in RBAGS.

### test-image-retrieve (₢APAAK) [complete]

**[260128-0720] complete**

Test the rbf_retrieve workflow by pulling an existing image from GAR to local Docker. Use rbf_list to identify an available image, then run tt/rbw-r.RetrieveImage.sh to pull it. Verify the image appears locally with docker images. This validates the Retriever credential flow and GAR authentication before running test suites.

**[260128-0710] rough**

Test the rbf_retrieve workflow by pulling an existing image from GAR to local Docker. Use rbf_list to identify an available image, then run tt/rbw-r.RetrieveImage.sh to pull it. Verify the image appears locally with docker images. This validates the Retriever credential flow and GAR authentication before running test suites.

### remove-gcb-jq-image-ref (₢APAAF) [complete]

**[260125-1417] complete**

Remove RBRR_GCB_JQ_IMAGE_REF and RBRR_GCB_SYFT_IMAGE_REF from all files. These undocumented variables block container start with validation errors. The GCB metadata step installs jq at runtime anyway. Remove: variable definitions in rbrr_RecipeBottleRegimeRepo.sh, validation calls in rbrr_regime.sh, substitution passing in rbf_Foundry.sh, and usage in rbgjb10-assemble-metadata.sh comments. Leave gcrane and oras refs (they're properly pinned and may be in use).

**[260125-1413] bridled**

Remove RBRR_GCB_JQ_IMAGE_REF and RBRR_GCB_SYFT_IMAGE_REF from all files. These undocumented variables block container start with validation errors. The GCB metadata step installs jq at runtime anyway. Remove: variable definitions in rbrr_RecipeBottleRegimeRepo.sh, validation calls in rbrr_regime.sh, substitution passing in rbf_Foundry.sh, and usage in rbgjb10-assemble-metadata.sh comments. Leave gcrane and oras refs (they're properly pinned and may be in use).

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: rbrr_RecipeBottleRegimeRepo.sh, Tools/rbw/rbrr_regime.sh, Tools/rbw/rbf_Foundry.sh (3 files) | Steps: 1. Delete RBRR_GCB_JQ_IMAGE_REF and RBRR_GCB_SYFT_IMAGE_REF definitions plus their comment block from rbrr_RecipeBottleRegimeRepo.sh 2. Delete the two buv_env_odref validation lines for JQ and SYFT from rbrr_regime.sh 3. In rbf_Foundry.sh delete the two validation test lines, delete the two jq arg lines for zjq_jq_ref and zjq_syft_ref, and delete the two substitution output lines for _RBGY_JQ_REF and _RBGY_SYFT_REF | Verify: source each file in bash to check syntax

**[260125-1410] rough**

Remove RBRR_GCB_JQ_IMAGE_REF and RBRR_GCB_SYFT_IMAGE_REF from all files. These undocumented variables block container start with validation errors. The GCB metadata step installs jq at runtime anyway. Remove: variable definitions in rbrr_RecipeBottleRegimeRepo.sh, validation calls in rbrr_regime.sh, substitution passing in rbf_Foundry.sh, and usage in rbgjb10-assemble-metadata.sh comments. Leave gcrane and oras refs (they're properly pinned and may be in use).

### verify-depot-and-docker (₢APAAA) [complete]

**[260128-0631] complete**

Verify GCP depot is active and Docker lifecycle works. First, list images in Artifact Registry to confirm depot access and see what's available. Then start nsproto nameplate (start/stop cycle) to confirm containers spin up correctly. If depot has lapsed or auth expired, this pace surfaces that early. Tabtargets: tt/rbw-il.ImageList.sh, tt/rbw-s.Start.nsproto.sh, tt/rbw-z.Stop.nsproto.sh

**[260125-1400] rough**

Verify GCP depot is active and Docker lifecycle works. First, list images in Artifact Registry to confirm depot access and see what's available. Then start nsproto nameplate (start/stop cycle) to confirm containers spin up correctly. If depot has lapsed or auth expired, this pace surfaces that early. Tabtargets: tt/rbw-il.ImageList.sh, tt/rbw-s.Start.nsproto.sh, tt/rbw-z.Stop.nsproto.sh

**[260125-0837] rough**

Verify GCP depot is active and Docker lifecycle works. Start nsproto nameplate (start/stop cycle) to confirm containers spin up correctly. If depot has lapsed, this pace surfaces that early. Tabtargets: tt/rbw-s.Start.nsproto.sh, tt/rbw-z.Stop.nsproto.sh

### verify-cloud-build-pipeline (₢APAAE) [complete]

**[260128-0652] complete**

Trigger a full cloud build to verify the OCI Layout Bridge pipeline still works. Uses rbf_Foundry.sh with stitcher to generate Cloud Build JSON, pushes multi-platform image to Artifact Registry via Skopeo. Confirms mason SA, depot, and build infrastructure are functional.

**[260125-0837] rough**

Trigger a full cloud build to verify the OCI Layout Bridge pipeline still works. Uses rbf_Foundry.sh with stitcher to generate Cloud Build JSON, pushes multi-platform image to Artifact Registry via Skopeo. Confirms mason SA, depot, and build infrastructure are functional.

### improve-image-list-output (₢APAAG) [complete]

**[260128-0707] complete**

Improve rbf_list output to show all available images with tags. Current behavior shows only moniker count. New behavior: list all images with full reference (registry/project/repo/moniker:tag) and metadata indicator [meta] when -meta companion exists. Output serves as reference for available images; actual nameplate config will use short monikers once GAR resolution logic is implemented (see ₢APAAI).

**[260128-0704] bridled**

Improve rbf_list output to show all available images with tags. Current behavior shows only moniker count. New behavior: list all images with full reference (registry/project/repo/moniker:tag) and metadata indicator [meta] when -meta companion exists. Output serves as reference for available images; actual nameplate config will use short monikers once GAR resolution logic is implemented (see ₢APAAI).

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/rbw/rbf_Foundry.sh (1 file) | Steps: 1. Read rbf_list function around line 712 2. Rewrite no-moniker branch to fetch all packages, build moniker list, identify -meta companions, fetch tags for each primary image, display full registry/project/repo/moniker:tag references with [meta] indicator when companion exists 3. Update moniker-specific branch to use same full reference format | Verify: Manual run of rbf_list tabtarget

**[260128-0702] rough**

Improve rbf_list output to show all available images with tags. Current behavior shows only moniker count. New behavior: list all images with full reference (registry/project/repo/moniker:tag) and metadata indicator [meta] when -meta companion exists. Output serves as reference for available images; actual nameplate config will use short monikers once GAR resolution logic is implemented (see ₢APAAI).

**[260128-0702] rough**

Improve rbf_list output to show all available images with tags. Current behavior shows only moniker count. New behavior: list all images with full reference (registry/project/repo/moniker:tag) and metadata indicator [meta] when -meta companion exists. Output serves as reference for available images; actual nameplate config will use short monikers once GAR resolution logic is implemented (see ₢APAAI).

**[260128-0630] rough**

Study why only one busybox image exists in the depot (rbev-busybox) when images should have datestamp names. Investigate: (1) How images are being named during builds, (2) Whether old images are being overwritten vs retained, (3) The expected naming convention and where it's defined. Goal: understand the naming pattern and whether this is expected behavior or a configuration issue.

### rename-repo-path-to-moniker (₢APAAI) [complete]

**[260128-0739] complete**

Rename RBRN_SENTRY_REPO_PATH and RBRN_BOTTLE_REPO_PATH to RBRN_SENTRY_MONIKER and RBRN_BOTTLE_MONIKER across all documents and code. Delete legacy .mk nameplate files.

## Documents to Update

1. **lenses/RBRN-RegimeNameplate.adoc** — Rename variables in spec table, update purpose from "Full repository path" to "Image moniker"
2. **lenses/RBS-Specification.adoc** — Update attribute references (lines 157-158) and term definitions (lines 1463-1475)

## Bash Code to Update

3. **Tools/rbw/rbrn_regime.sh** — Lines 55-56 (validation), 103-104 (rollup), 129-130 (docker env)
4. **Tools/rbw/rbrn_cli.sh** — Lines 72-73, 129, 133 (display output)

## Nameplate .env Files to Update (3 files)

5. **Tools/rbw/rbrn_nsproto.env** — Rename variables
6. **Tools/rbw/rbrn_pluml.env** — Rename variables
7. **Tools/rbw/rbrn_srjcl.env** — Rename variables

## Legacy Files to Delete (8 files)

8. **RBM-nameplates/nameplate.*.mk** (7 files) — Legacy Make format
9. **Tools/rbw/rbrn.nameplate.mk** — Make rollup helper

Values stay the same (e.g., sentry_ubuntu_large), only variable names change.

**[260128-0734] bridled**

Rename RBRN_SENTRY_REPO_PATH and RBRN_BOTTLE_REPO_PATH to RBRN_SENTRY_MONIKER and RBRN_BOTTLE_MONIKER across all documents and code. Delete legacy .mk nameplate files.

## Documents to Update

1. **lenses/RBRN-RegimeNameplate.adoc** — Rename variables in spec table, update purpose from "Full repository path" to "Image moniker"
2. **lenses/RBS-Specification.adoc** — Update attribute references (lines 157-158) and term definitions (lines 1463-1475)

## Bash Code to Update

3. **Tools/rbw/rbrn_regime.sh** — Lines 55-56 (validation), 103-104 (rollup), 129-130 (docker env)
4. **Tools/rbw/rbrn_cli.sh** — Lines 72-73, 129, 133 (display output)

## Nameplate .env Files to Update (3 files)

5. **Tools/rbw/rbrn_nsproto.env** — Rename variables
6. **Tools/rbw/rbrn_pluml.env** — Rename variables
7. **Tools/rbw/rbrn_srjcl.env** — Rename variables

## Legacy Files to Delete (8 files)

8. **RBM-nameplates/nameplate.*.mk** (7 files) — Legacy Make format
9. **Tools/rbw/rbrn.nameplate.mk** — Make rollup helper

Values stay the same (e.g., sentry_ubuntu_large), only variable names change.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: lenses/RBRN-RegimeNameplate.adoc, lenses/RBS-Specification.adoc, Tools/rbw/rbrn_regime.sh, Tools/rbw/rbrn_cli.sh, Tools/rbw/rbrn_nsproto.env, Tools/rbw/rbrn_pluml.env, Tools/rbw/rbrn_srjcl.env, plus delete RBM-nameplates/nameplate.*.mk and Tools/rbw/rbrn.nameplate.mk (7 edits + 8 deletes) | Steps: 1. Replace RBRN_SENTRY_REPO_PATH with RBRN_SENTRY_MONIKER in all edit files 2. Replace RBRN_BOTTLE_REPO_PATH with RBRN_BOTTLE_MONIKER in all edit files 3. Update doc prose Full repository path to Image moniker in RBRN-RegimeNameplate.adoc 4. Delete 8 legacy .mk files 5. Commit with /jjc-pace-notch | Verify: source Tools/rbw/rbrn_nsproto.env and echo RBRN_SENTRY_MONIKER

**[260128-0732] rough**

Rename RBRN_SENTRY_REPO_PATH and RBRN_BOTTLE_REPO_PATH to RBRN_SENTRY_MONIKER and RBRN_BOTTLE_MONIKER across all documents and code. Delete legacy .mk nameplate files.

## Documents to Update

1. **lenses/RBRN-RegimeNameplate.adoc** — Rename variables in spec table, update purpose from "Full repository path" to "Image moniker"
2. **lenses/RBS-Specification.adoc** — Update attribute references (lines 157-158) and term definitions (lines 1463-1475)

## Bash Code to Update

3. **Tools/rbw/rbrn_regime.sh** — Lines 55-56 (validation), 103-104 (rollup), 129-130 (docker env)
4. **Tools/rbw/rbrn_cli.sh** — Lines 72-73, 129, 133 (display output)

## Nameplate .env Files to Update (3 files)

5. **Tools/rbw/rbrn_nsproto.env** — Rename variables
6. **Tools/rbw/rbrn_pluml.env** — Rename variables
7. **Tools/rbw/rbrn_srjcl.env** — Rename variables

## Legacy Files to Delete (8 files)

8. **RBM-nameplates/nameplate.*.mk** (7 files) — Legacy Make format
9. **Tools/rbw/rbrn.nameplate.mk** — Make rollup helper

Values stay the same (e.g., sentry_ubuntu_large), only variable names change.

**[260128-0701] rough**

Add GAR image resolution logic to nameplate image handling. Currently nameplates use RBRN_*_REPO_PATH directly with docker pull. Implement resolution that detects GAR vs local images and prepends registry path from regime variables (RBRR_GAR_REPOSITORY, RBGD_GAR_LOCATION, RBGD_GAR_PROJECT_ID) for GAR images. This keeps nameplates DRY - they specify just the moniker, regime provides the registry context. Key file: Tools/rbw/rbob_bottle.sh (z_image construction at lines 166, 224, 275).

### implement-gar-image-resolution (₢APAAL) [complete]

**[260128-0740] complete**

Implement GAR image path construction in rbob_bottle.sh. After the MONIKER rename (pace APAAI), rbob needs to construct full GAR image paths.

## Implementation

In **Tools/rbw/rbob_bottle.sh**, update z_image construction at lines 166, 224, 275:

From:
```bash
local z_image="${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

To:
```bash
local z_image="${RBGD_GAR_LOCATION}-docker.pkg.dev/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

Pattern from rbi_Image.sh lines 45-46 shows the registry path construction.

## Prerequisite

Depends on ₢APAAI (rename-repo-path-to-moniker) completing first — the variables must be renamed before this implementation makes sense.

**[260128-0735] bridled**

Implement GAR image path construction in rbob_bottle.sh. After the MONIKER rename (pace APAAI), rbob needs to construct full GAR image paths.

## Implementation

In **Tools/rbw/rbob_bottle.sh**, update z_image construction at lines 166, 224, 275:

From:
```bash
local z_image="${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

To:
```bash
local z_image="${RBGD_GAR_LOCATION}-docker.pkg.dev/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

Pattern from rbi_Image.sh lines 45-46 shows the registry path construction.

## Prerequisite

Depends on ₢APAAI (rename-repo-path-to-moniker) completing first — the variables must be renamed before this implementation makes sense.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: Tools/rbw/rbob_bottle.sh (1 file) | Steps: 1. In zrbob_launch_sentry replace z_image assignment with GAR path using RBGD_GAR_LOCATION-docker.pkg.dev/RBGD_GAR_PROJECT_ID/RBRR_GAR_REPOSITORY/RBRN_SENTRY_MONIKER:tag 2. In zrbob_launch_censer same pattern with SENTRY moniker 3. In zrbob_launch_bottle same pattern with RBRN_BOTTLE_MONIKER 4. Commit with /jjc-pace-notch | Verify: grep docker.pkg.dev Tools/rbw/rbob_bottle.sh shows 3 matches

**[260128-0732] rough**

Implement GAR image path construction in rbob_bottle.sh. After the MONIKER rename (pace APAAI), rbob needs to construct full GAR image paths.

## Implementation

In **Tools/rbw/rbob_bottle.sh**, update z_image construction at lines 166, 224, 275:

From:
```bash
local z_image="${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

To:
```bash
local z_image="${RBGD_GAR_LOCATION}-docker.pkg.dev/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_MONIKER}:${RBRN_SENTRY_IMAGE_TAG}"
```

Pattern from rbi_Image.sh lines 45-46 shows the registry path construction.

## Prerequisite

Depends on ₢APAAI (rename-repo-path-to-moniker) completing first — the variables must be renamed before this implementation makes sense.

### fix-cloud-build-console-url (₢APAAM) [complete]

**[260128-2027] complete**

Fix Cloud Build console URL to include region

The `ZRBF_CLOUD_QUERY_BASE` URL in `Tools/rbw/rbf_Foundry.sh` generates malformed Cloud Console links.

## Current
```
https://console.cloud.google.com/cloud-build/builds/${build_id}?project=${project}
```

## Should be
```
https://console.cloud.google.com/cloud-build/builds;region=${region}/${build_id}?project=${project}
```

## Fix
Update `zrbf_kindle()` to include region in `ZRBF_CLOUD_QUERY_BASE`, using `RBGD_GCB_REGION`.

**[260128-2023] bridled**

Fix Cloud Build console URL to include region

The `ZRBF_CLOUD_QUERY_BASE` URL in `Tools/rbw/rbf_Foundry.sh` generates malformed Cloud Console links.

## Current
```
https://console.cloud.google.com/cloud-build/builds/${build_id}?project=${project}
```

## Should be
```
https://console.cloud.google.com/cloud-build/builds;region=${region}/${build_id}?project=${project}
```

## Fix
Update `zrbf_kindle()` to include region in `ZRBF_CLOUD_QUERY_BASE`, using `RBGD_GCB_REGION`.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: Tools/rbw/rbf_Foundry.sh (1 file) | Steps: 1. Update z_console_url assignment in rbf_build to insert ;region=RBGD_GCB_REGION after ZRBF_CLOUD_QUERY_BASE | Verify: N/A bash tooling

**[260128-0815] rough**

Fix Cloud Build console URL to include region

The `ZRBF_CLOUD_QUERY_BASE` URL in `Tools/rbw/rbf_Foundry.sh` generates malformed Cloud Console links.

## Current
```
https://console.cloud.google.com/cloud-build/builds/${build_id}?project=${project}
```

## Should be
```
https://console.cloud.google.com/cloud-build/builds;region=${region}/${build_id}?project=${project}
```

## Fix
Update `zrbf_kindle()` to include region in `ZRBF_CLOUD_QUERY_BASE`, using `RBGD_GCB_REGION`.

### audit-ark-vessel-vocabulary-in-rbsa (₢APAAP) [complete]

**[260208-1528] complete**

Audit and complete existing Ark/Vessel vocabulary in RBS0-SpecTop.adoc.

## Background

RBAGS-AdminGoogleSpec.adoc was renamed to RBS0-SpecTop.adoc (commit 41473401).
The `rbtga_*` Ark terms (ark, consecration, ark_image, ark_about) already exist at lines 1011-1054.
Vessel vocabulary exists via `rbrv_*` terms in RBSRV-RegimeVessel.adoc (included by RBSA).

## Audit checklist

1. Verify all Ark/Vessel terms from the original docket are covered:
   - Vessel (moniker/sigil) — check `rbrv_sigil` definition
   - Ark (timestamp manifestation) — check `rbtga_ark` definition
   - Ark `-image` / `-about` pair — check `rbtga_ark_image`, `rbtga_ark_about`
   - Consecration — check `rbtga_consecration`
2. Check for stale "image tag" references that should use Ark terminology
3. Verify Roles (Sentry, Bottle, Censer) are defined in `at_*` architectural terms
4. Confirm cross-references between Vessel regime (rbrv_*) and Ark definitions (rbtga_*) are coherent

## Location
`lenses/RBS0-SpecTop.adoc`

## Outcome
Either confirm vocabulary is complete (pace can wrap with no changes) or identify specific gaps to fill.

**[260208-1526] rough**

Audit and complete existing Ark/Vessel vocabulary in RBS0-SpecTop.adoc.

## Background

RBAGS-AdminGoogleSpec.adoc was renamed to RBS0-SpecTop.adoc (commit 41473401).
The `rbtga_*` Ark terms (ark, consecration, ark_image, ark_about) already exist at lines 1011-1054.
Vessel vocabulary exists via `rbrv_*` terms in RBSRV-RegimeVessel.adoc (included by RBSA).

## Audit checklist

1. Verify all Ark/Vessel terms from the original docket are covered:
   - Vessel (moniker/sigil) — check `rbrv_sigil` definition
   - Ark (timestamp manifestation) — check `rbtga_ark` definition
   - Ark `-image` / `-about` pair — check `rbtga_ark_image`, `rbtga_ark_about`
   - Consecration — check `rbtga_consecration`
2. Check for stale "image tag" references that should use Ark terminology
3. Verify Roles (Sentry, Bottle, Censer) are defined in `at_*` architectural terms
4. Confirm cross-references between Vessel regime (rbrv_*) and Ark definitions (rbtga_*) are coherent

## Location
`lenses/RBS0-SpecTop.adoc`

## Outcome
Either confirm vocabulary is complete (pace can wrap with no changes) or identify specific gaps to fill.

**[260208-1526] rough**

Audit and complete existing Ark/Vessel vocabulary in RBS0-SpecTop.adoc.

## Background

RBAGS-AdminGoogleSpec.adoc was renamed to RBS0-SpecTop.adoc (commit 41473401).
The `rbtga_*` Ark terms (ark, consecration, ark_image, ark_about) already exist at lines 1011-1054.
Vessel vocabulary exists via `rbrv_*` terms in RBSRV-RegimeVessel.adoc (included by RBSA).

## Audit checklist

1. Verify all Ark/Vessel terms from the original docket are covered:
   - Vessel (moniker/sigil) — check `rbrv_sigil` definition
   - Ark (timestamp manifestation) — check `rbtga_ark` definition
   - Ark `-image` / `-about` pair — check `rbtga_ark_image`, `rbtga_ark_about`
   - Consecration — check `rbtga_consecration`
2. Check for stale "image tag" references that should use Ark terminology
3. Verify Roles (Sentry, Bottle, Censer) are defined in `at_*` architectural terms
4. Confirm cross-references between Vessel regime (rbrv_*) and Ark definitions (rbtga_*) are coherent

## Location
`lenses/RBS0-SpecTop.adoc`

## Outcome
Either confirm vocabulary is complete (pace can wrap with no changes) or identify specific gaps to fill.

**[260128-2116] rough**

Define the Ark concept formally in RBAGS specification.

## Vocabulary to define

- **Vessel** (moniker/sigil): mystical artifact design for containing demons
- **Ark** (timestamp): specific manifestation of a vessel, producing two artifacts:
  - `-image`: the deployable prison
  - `-about`: provenance record for trust verification
- **Roles**: Sentry (guardian), Bottle (prison), Censer (bottle seeing only sentry's smoke)

## Location
`lenses/RBAGS-AdminGoogleSpec.adoc`

## Changes
- Add glossary/vocabulary section if not present
- Define Ark as the fundamental unit of deployment trust
- Explain the -image/-about artifact pair
- Update any existing references to "image tag" to use Ark terminology

### update-rbs-image-tag-to-vessel-consecration (₢APAAQ) [complete]

**[260208-1532] complete**

Update stale rbrn_*_image_tag references in RBS-Specification.adoc to use vessel/consecration vocabulary.

## Background

The env files (rbrn_*.env) already use RBRN_SENTRY_VESSEL / RBRN_SENTRY_CONSECRATION / RBRN_BOTTLE_VESSEL / RBRN_BOTTLE_CONSECRATION. But RBS-Specification.adoc still has the old image_tag pattern.

## Stale references in RBS-Specification.adoc

Mappings (lines 162-163):
- `:rbrn_sentry_image_tag:` → should reference vessel+consecration
- `:rbrn_bottle_image_tag:` → should reference vessel+consecration

Usage sites (lines 699, 838, 1069, 1202):
- References to `{rbrn_bottle_image_tag}` and `{rbrn_sentry_image_tag}`
- These describe how image references are constructed — update to vessel:consecration pattern

Definitions (lines 1486-1493):
- `[[rbrn_sentry_image_tag]]` and `[[rbrn_bottle_image_tag]]` anchors + definitions

## Approach

The image_tag concept is replaced by the vessel+consecration pair. Each usage site needs analysis:
- Some sites may simply replace `image_tag` with `vessel:consecration` expression
- Mapping/definition sections should remove old terms and ensure vessel/consecration terms from RBSA are used
- Check that RBSA already defines rbrn_sentry_vessel, rbrn_sentry_consecration, rbrn_bottle_vessel, rbrn_bottle_consecration (it does, confirmed in prior audit)

## Verification
Grep for `image_tag` in lenses/ to ensure complete removal.

**[260208-1530] rough**

Update stale rbrn_*_image_tag references in RBS-Specification.adoc to use vessel/consecration vocabulary.

## Background

The env files (rbrn_*.env) already use RBRN_SENTRY_VESSEL / RBRN_SENTRY_CONSECRATION / RBRN_BOTTLE_VESSEL / RBRN_BOTTLE_CONSECRATION. But RBS-Specification.adoc still has the old image_tag pattern.

## Stale references in RBS-Specification.adoc

Mappings (lines 162-163):
- `:rbrn_sentry_image_tag:` → should reference vessel+consecration
- `:rbrn_bottle_image_tag:` → should reference vessel+consecration

Usage sites (lines 699, 838, 1069, 1202):
- References to `{rbrn_bottle_image_tag}` and `{rbrn_sentry_image_tag}`
- These describe how image references are constructed — update to vessel:consecration pattern

Definitions (lines 1486-1493):
- `[[rbrn_sentry_image_tag]]` and `[[rbrn_bottle_image_tag]]` anchors + definitions

## Approach

The image_tag concept is replaced by the vessel+consecration pair. Each usage site needs analysis:
- Some sites may simply replace `image_tag` with `vessel:consecration` expression
- Mapping/definition sections should remove old terms and ensure vessel/consecration terms from RBSA are used
- Check that RBSA already defines rbrn_sentry_vessel, rbrn_sentry_consecration, rbrn_bottle_vessel, rbrn_bottle_consecration (it does, confirmed in prior audit)

## Verification
Grep for `image_tag` in lenses/ to ensure complete removal.

**[260208-1530] rough**

Update stale rbrn_*_image_tag references in RBS-Specification.adoc to use vessel/consecration vocabulary.

## Background

The env files (rbrn_*.env) already use RBRN_SENTRY_VESSEL / RBRN_SENTRY_CONSECRATION / RBRN_BOTTLE_VESSEL / RBRN_BOTTLE_CONSECRATION. But RBS-Specification.adoc still has the old image_tag pattern.

## Stale references in RBS-Specification.adoc

Mappings (lines 162-163):
- `:rbrn_sentry_image_tag:` → should reference vessel+consecration
- `:rbrn_bottle_image_tag:` → should reference vessel+consecration

Usage sites (lines 699, 838, 1069, 1202):
- References to `{rbrn_bottle_image_tag}` and `{rbrn_sentry_image_tag}`
- These describe how image references are constructed — update to vessel:consecration pattern

Definitions (lines 1486-1493):
- `[[rbrn_sentry_image_tag]]` and `[[rbrn_bottle_image_tag]]` anchors + definitions

## Approach

The image_tag concept is replaced by the vessel+consecration pair. Each usage site needs analysis:
- Some sites may simply replace `image_tag` with `vessel:consecration` expression
- Mapping/definition sections should remove old terms and ensure vessel/consecration terms from RBSA are used
- Check that RBSA already defines rbrn_sentry_vessel, rbrn_sentry_consecration, rbrn_bottle_vessel, rbrn_bottle_consecration (it does, confirmed in prior audit)

## Verification
Grep for `image_tag` in lenses/ to ensure complete removal.

**[260128-2116] rough**

Rename nameplate fields from IMAGE_TAG to ARK.

## Changes
All nameplate files in `Tools/rbw/rbrn_*.env`:
- `RBRN_SENTRY_IMAGE_TAG` → `RBRN_SENTRY_ARK`
- `RBRN_BOTTLE_IMAGE_TAG` → `RBRN_BOTTLE_ARK`

## Files
- `Tools/rbw/rbrn_nsproto.env`
- `Tools/rbw/rbrn_pluml.env` (if exists)
- `Tools/rbw/rbrn_srjcl.env` (if exists)
- Any code reading these fields

## Verification
Grep for IMAGE_TAG to ensure complete rename.

### plumb-ark-constants-to-gcb (₢APAAR) [complete]

**[260208-1534] complete**

Plumb RBGC ark constants to Cloud Build via substitution variables.

## Background

Local scripts now use RBGC constants for registry host suffix and ark artifact suffixes. Cloud Build scripts (rbgjb) still hardcode these values. This pace unifies them.

## New substitution variables

Add to `rbf_Foundry.sh` substitutions block:
```
_RBGY_GAR_HOST_SUFFIX:   ${RBGC_GAR_HOST_SUFFIX}    # -docker.pkg.dev
_RBGY_ARK_SUFFIX_IMAGE:  ${RBGC_ARK_SUFFIX_IMAGE}   # -image
_RBGY_ARK_SUFFIX_ABOUT:  ${RBGC_ARK_SUFFIX_ABOUT}   # -about
```

## Files to update

1. **Tools/rbw/rbf_Foundry.sh** — Add three substitutions to the jq build config block (~line 609)

2. **Tools/rbw/rbgjb/rbgjb03-docker-login-gar.sh** — Use `${_RBGY_GAR_HOST_SUFFIX}` instead of hardcoded `-docker.pkg.dev`

3. **Tools/rbw/rbgjb/rbgjb06-build-and-export.sh** — Use substitution variables for host suffix and `-img` → `${_RBGY_ARK_SUFFIX_IMAGE}`

4. **Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh** — Same: host suffix and image suffix

5. **Tools/rbw/rbgjb/rbgjb09-build-and-push-metadata.sh** — Use host suffix and `-meta` → `${_RBGY_ARK_SUFFIX_ABOUT}`

## Verification

- Build succeeds with new substitution variables
- Artifacts land with correct `-image` and `-about` suffixes
- No hardcoded `-docker.pkg.dev`, `-img`, or `-meta` remain in rbgjb scripts

## Breaking change

Existing GAR artifacts with `-img`/`-meta` suffixes become unretrievable. Accepted — burn the bridges.

**[260201-2021] rough**

Plumb RBGC ark constants to Cloud Build via substitution variables.

## Background

Local scripts now use RBGC constants for registry host suffix and ark artifact suffixes. Cloud Build scripts (rbgjb) still hardcode these values. This pace unifies them.

## New substitution variables

Add to `rbf_Foundry.sh` substitutions block:
```
_RBGY_GAR_HOST_SUFFIX:   ${RBGC_GAR_HOST_SUFFIX}    # -docker.pkg.dev
_RBGY_ARK_SUFFIX_IMAGE:  ${RBGC_ARK_SUFFIX_IMAGE}   # -image
_RBGY_ARK_SUFFIX_ABOUT:  ${RBGC_ARK_SUFFIX_ABOUT}   # -about
```

## Files to update

1. **Tools/rbw/rbf_Foundry.sh** — Add three substitutions to the jq build config block (~line 609)

2. **Tools/rbw/rbgjb/rbgjb03-docker-login-gar.sh** — Use `${_RBGY_GAR_HOST_SUFFIX}` instead of hardcoded `-docker.pkg.dev`

3. **Tools/rbw/rbgjb/rbgjb06-build-and-export.sh** — Use substitution variables for host suffix and `-img` → `${_RBGY_ARK_SUFFIX_IMAGE}`

4. **Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh** — Same: host suffix and image suffix

5. **Tools/rbw/rbgjb/rbgjb09-build-and-push-metadata.sh** — Use host suffix and `-meta` → `${_RBGY_ARK_SUFFIX_ABOUT}`

## Verification

- Build succeeds with new substitution variables
- Artifacts land with correct `-image` and `-about` suffixes
- No hardcoded `-docker.pkg.dev`, `-img`, or `-meta` remain in rbgjb scripts

## Breaking change

Existing GAR artifacts with `-img`/`-meta` suffixes become unretrievable. Accepted — burn the bridges.

**[260128-2116] rough**

Rename build artifact suffixes from -img/-meta to -image/-about.

## Locations
- `Tools/rbw/rbgjb/rbgjb09-build-and-push-metadata.sh` — produces -meta, change to -about
- `Tools/rbw/rbf_Foundry.sh` — references to -img and -meta throughout
- Any other rbgjb scripts referencing these suffixes

## Changes
- `-img` → `-image`
- `-meta` → `-about`

## Impact
Existing GAR artifacts will become unretrievable. This is accepted — burn the bridges.

### simplify-rbf-list-raw-images (₢APAAS) [complete]

**[260208-1630] complete**

Simplify rbf_list to show raw container images without ark interpretation.

## Current state
rbf_list has stale `-meta` companion package logic that checks for separate `moniker-meta` packages. This predates the ark model where `-image` and `-about` are tag suffixes on the same moniker.

## Code changes
- Remove all `-meta` companion detection logic (the separate package lookup)
- Show every tag as a flat line — no grouping, no annotations
- `rbf_list` (no args): list all vessels with tag counts
- `rbf_list <vessel>`: list all tags for that vessel, one per line

## Spec changes
- Review/update RBSA mapping and definition for the image list operation if one exists
- If no operation lens exists for rbf_list, create one (e.g., RBSIL-image_list.adoc)
- Ensure spec describes plain image-level listing, no ark semantics

## Design principle
rbf_list is the plain, low-level "what's in GAR" command. No ark awareness. Ark-level viewing is rbf_beseech (separate pace).

## Location
- `Tools/rbw/rbf_Foundry.sh` — rbf_list function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — operation lens if needed

## Verification
- No `-meta` references remain in rbf_list
- Output shows raw tags without interpretation
- RBSA terms consistent with implementation

**[260208-1621] bridled**

Simplify rbf_list to show raw container images without ark interpretation.

## Current state
rbf_list has stale `-meta` companion package logic that checks for separate `moniker-meta` packages. This predates the ark model where `-image` and `-about` are tag suffixes on the same moniker.

## Code changes
- Remove all `-meta` companion detection logic (the separate package lookup)
- Show every tag as a flat line — no grouping, no annotations
- `rbf_list` (no args): list all vessels with tag counts
- `rbf_list <vessel>`: list all tags for that vessel, one per line

## Spec changes
- Review/update RBSA mapping and definition for the image list operation if one exists
- If no operation lens exists for rbf_list, create one (e.g., RBSIL-image_list.adoc)
- Ensure spec describes plain image-level listing, no ark semantics

## Design principle
rbf_list is the plain, low-level "what's in GAR" command. No ark awareness. Ark-level viewing is rbf_beseech (separate pace).

## Location
- `Tools/rbw/rbf_Foundry.sh` — rbf_list function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — operation lens if needed

## Verification
- No `-meta` references remain in rbf_list
- Output shows raw tags without interpretation
- RBSA terms consistent with implementation

*Direction:* Agent: sonnet | Cardinality: 2 parallel then verify | Files: Tools/rbw/rbf_Foundry.sh, lenses/RBSIL-image_list.adoc, lenses/RBS0-SpecTop.adoc (3 files) | Steps: 1. Agent A rewrites rbf_list function in rbf_Foundry.sh -- remove all -meta companion detection logic including meta_moniker, meta_file, z_has_meta, meta_map, meta_candidate, primary_list filtering, and meta annotations -- with-moniker path just lists tags one per line with full registry reference -- no-arg path lists all packages with tag counts, no filtering 2. Agent B creates RBSIL-image_list.adoc following RBSID-image_delete.adoc pattern describing plain image listing with no ark semantics, and adds rbtgo_image_list mapping to RBS0-SpecTop.adoc near the existing rbtgo_image_delete line | Verify: grep -c meta Tools/rbw/rbf_Foundry.sh should show only non-rbf_list occurrences like assemble-metadata build steps

**[260208-1600] rough**

Simplify rbf_list to show raw container images without ark interpretation.

## Current state
rbf_list has stale `-meta` companion package logic that checks for separate `moniker-meta` packages. This predates the ark model where `-image` and `-about` are tag suffixes on the same moniker.

## Code changes
- Remove all `-meta` companion detection logic (the separate package lookup)
- Show every tag as a flat line — no grouping, no annotations
- `rbf_list` (no args): list all vessels with tag counts
- `rbf_list <vessel>`: list all tags for that vessel, one per line

## Spec changes
- Review/update RBSA mapping and definition for the image list operation if one exists
- If no operation lens exists for rbf_list, create one (e.g., RBSIL-image_list.adoc)
- Ensure spec describes plain image-level listing, no ark semantics

## Design principle
rbf_list is the plain, low-level "what's in GAR" command. No ark awareness. Ark-level viewing is rbf_beseech (separate pace).

## Location
- `Tools/rbw/rbf_Foundry.sh` — rbf_list function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — operation lens if needed

## Verification
- No `-meta` references remain in rbf_list
- Output shows raw tags without interpretation
- RBSA terms consistent with implementation

**[260208-1558] rough**

Simplify rbf_list to show raw container images without ark interpretation.

## Current state
rbf_list has stale `-meta` companion package logic that checks for separate `moniker-meta` packages. This predates the ark model where `-image` and `-about` are tag suffixes on the same moniker.

## Changes
- Remove all `-meta` companion detection logic (the separate package lookup)
- Show every tag as a flat line — no grouping, no annotations
- `rbf_list` (no args): list all monikers with tag counts
- `rbf_list <vessel>`: list all tags for that vessel, one per line

## Design principle
rbf_list is the plain, low-level "what's in GAR" command. No ark awareness. Ark-level viewing is rbf_augur (separate pace).

## Location
`Tools/rbw/rbf_Foundry.sh` — rbf_list function

## Verification
- No `-meta` references remain in rbf_list
- Output shows raw tags without interpretation

**[260208-1558] rough**

Simplify rbf_list to show raw container images without ark interpretation.

## Current state
rbf_list has stale `-meta` companion package logic that checks for separate `moniker-meta` packages. This predates the ark model where `-image` and `-about` are tag suffixes on the same moniker.

## Changes
- Remove all `-meta` companion detection logic (the separate package lookup)
- Show every tag as a flat line — no grouping, no annotations
- `rbf_list` (no args): list all monikers with tag counts
- `rbf_list <vessel>`: list all tags for that vessel, one per line

## Design principle
rbf_list is the plain, low-level "what's in GAR" command. No ark awareness. Ark-level viewing is rbf_augur (separate pace).

## Location
`Tools/rbw/rbf_Foundry.sh` — rbf_list function

## Verification
- No `-meta` references remain in rbf_list
- Output shows raw tags without interpretation

**[260128-2116] rough**

Update ImageList and Retrieve commands for ark vocabulary.

## ImageList changes
- Output should show ark timestamps clearly
- Indicate -image vs -about artifacts distinctly (not both as [meta])
- Consider showing just the ark (base timestamp) with artifact type annotations

## Retrieve changes  
- Accept ark (base timestamp) and auto-append -image suffix
- Or accept explicit full reference with -image suffix
- Update user-facing messages to use ark terminology

## Location
`Tools/rbw/rbf_Foundry.sh` — rbf_list and rbf_retrieve functions

### implement-rbf-delete-single-tag (₢APAAT) [complete]

**[260209-0542] complete**

Implement rbf_delete for plain container image deletion.

## Concept
rbf_delete is the plain, low-level counterpart to rbf_list. It deletes a single container image tag from GAR. No ark awareness — it operates on raw image references.

## Interface
`rbf_delete <vessel>:<tag>` — deletes one tag from the registry

## Code changes
- New rbf_delete function in rbf_Foundry.sh
- Use Docker Registry HTTP API V2 DELETE manifests (same pattern as rbf_abjure)
- Authenticate as Director
- Confirm before deletion unless --force flag
- Delete exactly one tag — no companion/pair logic

## Spec changes
- Add RBSA mapping and definition for rbtgo_image_delete
- Create operation lens RBSID-image_delete.adoc (or update if exists)
- Spec describes plain single-tag deletion, distinct from ark-level rbf_abjure

## Design principle
Ark-level deletion (removing both -image and -about as a unit) is rbf_abjure. rbf_delete is for surgical cleanup of individual tags.

## Location
- `Tools/rbw/rbf_Foundry.sh` — new rbf_delete function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/RBSID-image_delete.adoc` — operation lens

## Verification
- Can delete a single tag without affecting its ark partner
- Confirmation prompt works, --force bypasses it
- RBSA terms consistent with implementation

**[260208-1603] rough**

Implement rbf_delete for plain container image deletion.

## Concept
rbf_delete is the plain, low-level counterpart to rbf_list. It deletes a single container image tag from GAR. No ark awareness — it operates on raw image references.

## Interface
`rbf_delete <vessel>:<tag>` — deletes one tag from the registry

## Code changes
- New rbf_delete function in rbf_Foundry.sh
- Use Docker Registry HTTP API V2 DELETE manifests (same pattern as rbf_abjure)
- Authenticate as Director
- Confirm before deletion unless --force flag
- Delete exactly one tag — no companion/pair logic

## Spec changes
- Add RBSA mapping and definition for rbtgo_image_delete
- Create operation lens RBSID-image_delete.adoc (or update if exists)
- Spec describes plain single-tag deletion, distinct from ark-level rbf_abjure

## Design principle
Ark-level deletion (removing both -image and -about as a unit) is rbf_abjure. rbf_delete is for surgical cleanup of individual tags.

## Location
- `Tools/rbw/rbf_Foundry.sh` — new rbf_delete function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/RBSID-image_delete.adoc` — operation lens

## Verification
- Can delete a single tag without affecting its ark partner
- Confirmation prompt works, --force bypasses it
- RBSA terms consistent with implementation

**[260208-1558] rough**

Implement rbf_delete for plain container image deletion.

## Concept
rbf_delete is the plain, low-level counterpart to rbf_list. It deletes a single container image tag from GAR. No ark awareness — it operates on raw image references.

## Interface
`rbf_delete <vessel>:<tag>` — deletes one tag from the registry

## Implementation
- Use Docker Registry HTTP API V2 DELETE manifests (same pattern as rbf_abjure)
- Authenticate as Director
- Confirm before deletion unless --force flag
- Delete exactly one tag — no companion/pair logic

## Design principle
Ark-level deletion (removing both -image and -about as a unit) is rbf_abjure. rbf_delete is for surgical cleanup of individual tags.

## Location
`Tools/rbw/rbf_Foundry.sh` — new rbf_delete function

## Verification
- Can delete a single tag without affecting its ark partner
- Confirmation prompt works, --force bypasses it

**[260208-1558] rough**

Implement rbf_delete for plain container image deletion.

## Concept
rbf_delete is the plain, low-level counterpart to rbf_list. It deletes a single container image tag from GAR. No ark awareness — it operates on raw image references.

## Interface
`rbf_delete <vessel>:<tag>` — deletes one tag from the registry

## Implementation
- Use Docker Registry HTTP API V2 DELETE manifests (same pattern as rbf_abjure)
- Authenticate as Director
- Confirm before deletion unless --force flag
- Delete exactly one tag — no companion/pair logic

## Design principle
Ark-level deletion (removing both -image and -about as a unit) is rbf_abjure. rbf_delete is for surgical cleanup of individual tags.

## Location
`Tools/rbw/rbf_Foundry.sh` — new rbf_delete function

## Verification
- Can delete a single tag without affecting its ark partner
- Confirmation prompt works, --force bypasses it

**[260128-2116] rough**

Add capability to delete images from GAR by ark.

## Functionality
New command/tabtarget to delete both -image and -about artifacts for a given ark.

## Interface
`tt/rbw-id.ImageDelete.sh moniker:ark` — deletes both artifacts

## Implementation
- Use GAR API or gcloud to delete specific tags
- Should delete both -image and -about for the specified ark
- Confirm before deletion or require --force flag

## Purpose
Clean up old artifacts after vocabulary migration (burn the bridges).

### implement-rbf-beseech-ark-view (₢APAAW) [complete]

**[260209-0615] complete**

Implement rbf_beseech — ark-level registry view.

## Concept
rbf_beseech petitions the registry to reveal its consecrated arks. It correlates `-image` and `-about` tags by shared consecration timestamp, presenting arks as logical units.

## Interface
- `rbf_beseech` (no args): show all arks across all vessels
- `rbf_beseech <vessel>`: show arks for one vessel

## Output format
Flat list, one line per ark (consecration), with completeness columns:
```
VESSEL                        CONSECRATION          -image  -about
rbev-sentry-ubuntu-large      20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250710T191022Z      ✓       ✗
```

## Code changes
- New rbf_beseech function in rbf_Foundry.sh
- Fetch all tags per vessel (reuse rbf_list's API calls)
- Parse tags: strip `-image` and `-about` suffixes to extract base consecration
- Group by vessel+consecration, track which artifacts exist
- Sort by vessel then consecration (newest first)

## Spec changes
- Add RBSA mapping and definition for rbtgo_ark_beseech
- Create operation lens (e.g., RBSAB-ark_beseech.adoc)
- Spec describes ark-level enumeration, correlating artifact pairs by consecration

## Design principle
This is the ark-level counterpart to rbf_list (plain images). Pairs with rbf_abjure (ark-level delete) and rbf_consecrate (ark-level build).

## Location
- `Tools/rbw/rbf_Foundry.sh` — new rbf_beseech function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — new operation lens

## Verification
- Shows arks grouped by consecration
- Correctly detects orphaned artifacts (only -image or only -about)
- No `-meta` references
- RBSA terms consistent with implementation

**[260208-1603] rough**

Implement rbf_beseech — ark-level registry view.

## Concept
rbf_beseech petitions the registry to reveal its consecrated arks. It correlates `-image` and `-about` tags by shared consecration timestamp, presenting arks as logical units.

## Interface
- `rbf_beseech` (no args): show all arks across all vessels
- `rbf_beseech <vessel>`: show arks for one vessel

## Output format
Flat list, one line per ark (consecration), with completeness columns:
```
VESSEL                        CONSECRATION          -image  -about
rbev-sentry-ubuntu-large      20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250710T191022Z      ✓       ✗
```

## Code changes
- New rbf_beseech function in rbf_Foundry.sh
- Fetch all tags per vessel (reuse rbf_list's API calls)
- Parse tags: strip `-image` and `-about` suffixes to extract base consecration
- Group by vessel+consecration, track which artifacts exist
- Sort by vessel then consecration (newest first)

## Spec changes
- Add RBSA mapping and definition for rbtgo_ark_beseech
- Create operation lens (e.g., RBSAB-ark_beseech.adoc)
- Spec describes ark-level enumeration, correlating artifact pairs by consecration

## Design principle
This is the ark-level counterpart to rbf_list (plain images). Pairs with rbf_abjure (ark-level delete) and rbf_consecrate (ark-level build).

## Location
- `Tools/rbw/rbf_Foundry.sh` — new rbf_beseech function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — new operation lens

## Verification
- Shows arks grouped by consecration
- Correctly detects orphaned artifacts (only -image or only -about)
- No `-meta` references
- RBSA terms consistent with implementation

**[260208-1603] rough**

Implement rbf_beseech — ark-level registry view.

## Concept
rbf_beseech petitions the registry to reveal its consecrated arks. It correlates `-image` and `-about` tags by shared consecration timestamp, presenting arks as logical units.

## Interface
- `rbf_beseech` (no args): show all arks across all vessels
- `rbf_beseech <vessel>`: show arks for one vessel

## Output format
Flat list, one line per ark (consecration), with completeness columns:
```
VESSEL                        CONSECRATION          -image  -about
rbev-sentry-ubuntu-large      20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250710T191022Z      ✓       ✗
```

## Code changes
- New rbf_beseech function in rbf_Foundry.sh
- Fetch all tags per vessel (reuse rbf_list's API calls)
- Parse tags: strip `-image` and `-about` suffixes to extract base consecration
- Group by vessel+consecration, track which artifacts exist
- Sort by vessel then consecration (newest first)

## Spec changes
- Add RBSA mapping and definition for rbtgo_ark_beseech
- Create operation lens (e.g., RBSAB-ark_beseech.adoc)
- Spec describes ark-level enumeration, correlating artifact pairs by consecration

## Design principle
This is the ark-level counterpart to rbf_list (plain images). Pairs with rbf_abjure (ark-level delete) and rbf_consecrate (ark-level build).

## Location
- `Tools/rbw/rbf_Foundry.sh` — new rbf_beseech function
- `lenses/RBS0-SpecTop.adoc` — mapping/definition
- `lenses/` — new operation lens

## Verification
- Shows arks grouped by consecration
- Correctly detects orphaned artifacts (only -image or only -about)
- No `-meta` references
- RBSA terms consistent with implementation

**[260208-1559] rough**

Implement rbf_augur — ark-level registry view.

## Concept
rbf_augur discerns the state of consecrated arks. It correlates `-image` and `-about` tags by shared consecration timestamp, presenting arks as logical units.

## Interface
- `rbf_augur` (no args): show all arks across all vessels
- `rbf_augur <vessel>`: show arks for one vessel

## Output format
Flat list, one line per ark (consecration), with completeness columns:
```
VESSEL                        CONSECRATION          -image  -about
rbev-sentry-ubuntu-large      20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250715T032145Z      ✓       ✓
rbev-bottle-ubuntu-test       20250710T191022Z      ✓       ✗
```

## Implementation
- Fetch all tags per vessel (reuse rbf_list's API calls)
- Parse tags: strip `-image` and `-about` suffixes to extract base consecration
- Group by vessel+consecration, track which artifacts exist
- Sort by vessel then consecration (newest first)

## Design principle
This is the ark-level counterpart to rbf_list (plain images). Pairs with rbf_abjure (ark-level delete) and rbf_consecrate (ark-level build).

## Location
`Tools/rbw/rbf_Foundry.sh` — new rbf_augur function

## Spec
May need a new operation lens (RBSAU or similar) and RBSA mapping for rbtgo_ark_augur.

## Verification
- Shows arks grouped by consecration
- Correctly detects orphaned artifacts (only -image or only -about)
- No `-meta` references

### align-rbf-retrieve-with-locator-vocab (₢APAAX) [complete]

**[260209-0627] complete**

Align rbf_retrieve with updated vocabulary and specification patterns.

## Context
rbf_list and rbf_delete have been updated to use rbst_locator (moniker:tag format).
rbf_retrieve still uses the old interface and spec terminology.

## Code changes
- Update rbf_retrieve in rbf_Foundry.sh to accept rbst_locator input
- Simplify parameter parsing (no digest support needed — locator is moniker:tag)
- Align parameter naming with rbf_delete pattern

## Spec changes
- Update RBSIR-image_retrieve.adoc to use {rbst_locator} vocabulary
- Align with RBSID-image_delete.adoc pattern

## Verification
- rbf_retrieve accepts locator from rbf_list output (copy-paste workflow)
- Spec uses {rbst_locator} consistently
- Tabtarget and coordinator unchanged (pass-through)

**[260209-0558] rough**

Align rbf_retrieve with updated vocabulary and specification patterns.

## Context
rbf_list and rbf_delete have been updated to use rbst_locator (moniker:tag format).
rbf_retrieve still uses the old interface and spec terminology.

## Code changes
- Update rbf_retrieve in rbf_Foundry.sh to accept rbst_locator input
- Simplify parameter parsing (no digest support needed — locator is moniker:tag)
- Align parameter naming with rbf_delete pattern

## Spec changes
- Update RBSIR-image_retrieve.adoc to use {rbst_locator} vocabulary
- Align with RBSID-image_delete.adoc pattern

## Verification
- rbf_retrieve accepts locator from rbf_list output (copy-paste workflow)
- Spec uses {rbst_locator} consistently
- Tabtarget and coordinator unchanged (pass-through)

### formalize-conjure-in-rbsa (₢APAAY) [complete]

**[260209-0632] complete**

Formalize the conjure operation in RBSA vocabulary.

## Context
rbf_build is already the conjuring operation (uses conjuring config: RBRV_CONJURE_DOCKERFILE, etc.)
but has no rbtgo_ark_conjure mapping in RBSA. The ark verb table should be complete:
abjure (delete), beseech (list), conjure (build), summon (retrieve).

## Code changes
None — rbf_build remains the implementation function.

## Spec changes
- Add rbtgo_ark_conjure mapping in RBS0-SpecTop.adoc (pointing to rbf_build)
- Add definition section for rbtgo_ark_conjure
- Optionally create RBSAC-ark_conjure.adoc that references/includes existing RBSTB spec

## Verification
- RBSA has complete ark verb table: abjure, beseech, conjure, summon
- No code changes, spec-only pace

**[260209-0606] rough**

Formalize the conjure operation in RBSA vocabulary.

## Context
rbf_build is already the conjuring operation (uses conjuring config: RBRV_CONJURE_DOCKERFILE, etc.)
but has no rbtgo_ark_conjure mapping in RBSA. The ark verb table should be complete:
abjure (delete), beseech (list), conjure (build), summon (retrieve).

## Code changes
None — rbf_build remains the implementation function.

## Spec changes
- Add rbtgo_ark_conjure mapping in RBS0-SpecTop.adoc (pointing to rbf_build)
- Add definition section for rbtgo_ark_conjure
- Optionally create RBSAC-ark_conjure.adoc that references/includes existing RBSTB spec

## Verification
- RBSA has complete ark verb table: abjure, beseech, conjure, summon
- No code changes, spec-only pace

### wire-ark-colophon-group (₢APAAZ) [complete]

**[260209-0748] complete**

Wire all ark operations under rbw-a* colophon group and implement rbf_summon.

## Colophon changes

### New ark group (rbw-a*)
- rbw-aA.AbjureArk.sh   → rbf_abjure   (Director)  — relocated from rbw-fA
- rbw-ab.BeseechArk.sh  → rbf_beseech  (Director)  — new wiring
- rbw-aC.ConjureArk.sh  → rbf_build    (Director)  — replaces rbw-fB
- rbw-as.SummonArk.sh   → rbf_summon   (Retriever) — new function + wiring

### Governor absorbs service account ops
- rbw-Gl.ListServiceAccounts.sh    → rbgg_list_service_accounts   (was rbw-al)
- rbw-GS.DeleteServiceAccount.sh   → rbgg_delete_service_account  (was rbw-aDS)

### Image group rename
- rbw-iD.DeleteImage.sh → rbf_delete (was rbw-fD)

### Removals (dead old-dispatch tabtargets)
- rbw-fA.AbjureArk.sh (relocated to rbw-aA)
- rbw-fB.BuildVessel.sh (replaced by rbw-aC)
- rbw-fD.DeleteImage.sh (relocated to rbw-iD)
- rbw-fS.BuildStudyDEBUG.sh (dead study debug)
- rbw-al.ListServiceAccounts.sh (relocated to rbw-Gl)
- rbw-aDS.DeleteServiceAccount.sh (relocated to rbw-GS)
- rbw-a.PodmanStart.sh (dead)
- rbw-aCD.CreateDirectorAccount.sh (dead, superseded by rbw-GD)
- rbw-aCR.CreateReaderAccount.sh (dead, superseded by rbw-GR)
- rbw-aIA.InitializeAdminAccount.sh (dead)
- rbw-aID.DELETE_ALL.sh (dead)
- rbw-aPO.ObliterateProject.sh (dead)
- rbw-aPr.RestoreProject.sh (dead)

## Code changes
- New rbf_summon function in rbf_Foundry.sh
  - Accept <vessel> <consecration>
  - Authenticate as Retriever (fallback Director, same as rbf_retrieve)
  - Construct both ark tags using RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT
  - HEAD check both artifacts (same as rbf_abjure pattern)
  - Docker login + docker pull for each existing artifact
  - Report which artifacts were retrieved, warn on orphans

## Coordinator changes (rbk_Coordinator.sh)
- Remove: rbw-fA, rbw-fB, rbw-fD, rbw-al, rbw-aDS
- Add: rbw-aA, rbw-ab, rbw-aC, rbw-as, rbw-Gl, rbw-GS, rbw-iD

## Spec changes
- Add :rbtgo_ark_summon: mapping in RBS0-SpecTop.adoc
- Add [[rbtgo_ark_summon]] section in Retriever Operations
- Create RBSAS-ark_summon.adoc lens following ark_abjure/ark_conjure pattern

## Verification
- All four ark tabtargets (aA, ab, aC, as) route correctly through coordinator
- Relocated governor and image tabtargets work
- Dead tabtargets removed
- rbf_summon pulls both artifacts for a given vessel+consecration
- RBSA terms consistent with implementation

**[260209-0709] rough**

Wire all ark operations under rbw-a* colophon group and implement rbf_summon.

## Colophon changes

### New ark group (rbw-a*)
- rbw-aA.AbjureArk.sh   → rbf_abjure   (Director)  — relocated from rbw-fA
- rbw-ab.BeseechArk.sh  → rbf_beseech  (Director)  — new wiring
- rbw-aC.ConjureArk.sh  → rbf_build    (Director)  — replaces rbw-fB
- rbw-as.SummonArk.sh   → rbf_summon   (Retriever) — new function + wiring

### Governor absorbs service account ops
- rbw-Gl.ListServiceAccounts.sh    → rbgg_list_service_accounts   (was rbw-al)
- rbw-GS.DeleteServiceAccount.sh   → rbgg_delete_service_account  (was rbw-aDS)

### Image group rename
- rbw-iD.DeleteImage.sh → rbf_delete (was rbw-fD)

### Removals (dead old-dispatch tabtargets)
- rbw-fA.AbjureArk.sh (relocated to rbw-aA)
- rbw-fB.BuildVessel.sh (replaced by rbw-aC)
- rbw-fD.DeleteImage.sh (relocated to rbw-iD)
- rbw-fS.BuildStudyDEBUG.sh (dead study debug)
- rbw-al.ListServiceAccounts.sh (relocated to rbw-Gl)
- rbw-aDS.DeleteServiceAccount.sh (relocated to rbw-GS)
- rbw-a.PodmanStart.sh (dead)
- rbw-aCD.CreateDirectorAccount.sh (dead, superseded by rbw-GD)
- rbw-aCR.CreateReaderAccount.sh (dead, superseded by rbw-GR)
- rbw-aIA.InitializeAdminAccount.sh (dead)
- rbw-aID.DELETE_ALL.sh (dead)
- rbw-aPO.ObliterateProject.sh (dead)
- rbw-aPr.RestoreProject.sh (dead)

## Code changes
- New rbf_summon function in rbf_Foundry.sh
  - Accept <vessel> <consecration>
  - Authenticate as Retriever (fallback Director, same as rbf_retrieve)
  - Construct both ark tags using RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT
  - HEAD check both artifacts (same as rbf_abjure pattern)
  - Docker login + docker pull for each existing artifact
  - Report which artifacts were retrieved, warn on orphans

## Coordinator changes (rbk_Coordinator.sh)
- Remove: rbw-fA, rbw-fB, rbw-fD, rbw-al, rbw-aDS
- Add: rbw-aA, rbw-ab, rbw-aC, rbw-as, rbw-Gl, rbw-GS, rbw-iD

## Spec changes
- Add :rbtgo_ark_summon: mapping in RBS0-SpecTop.adoc
- Add [[rbtgo_ark_summon]] section in Retriever Operations
- Create RBSAS-ark_summon.adoc lens following ark_abjure/ark_conjure pattern

## Verification
- All four ark tabtargets (aA, ab, aC, as) route correctly through coordinator
- Relocated governor and image tabtargets work
- Dead tabtargets removed
- rbf_summon pulls both artifacts for a given vessel+consecration
- RBSA terms consistent with implementation

**[260209-0709] rough**

Wire all ark operations under rbw-a* colophon group and implement rbf_summon.

## Colophon changes

### New ark group (rbw-a*)
- rbw-aA.AbjureArk.sh   → rbf_abjure   (Director)  — relocated from rbw-fA
- rbw-ab.BeseechArk.sh  → rbf_beseech  (Director)  — new wiring
- rbw-aC.ConjureArk.sh  → rbf_build    (Director)  — replaces rbw-fB
- rbw-as.SummonArk.sh   → rbf_summon   (Retriever) — new function + wiring

### Governor absorbs service account ops
- rbw-Gl.ListServiceAccounts.sh    → rbgg_list_service_accounts   (was rbw-al)
- rbw-GS.DeleteServiceAccount.sh   → rbgg_delete_service_account  (was rbw-aDS)

### Image group rename
- rbw-iD.DeleteImage.sh → rbf_delete (was rbw-fD)

### Removals (dead old-dispatch tabtargets)
- rbw-fA.AbjureArk.sh (relocated to rbw-aA)
- rbw-fB.BuildVessel.sh (replaced by rbw-aC)
- rbw-fD.DeleteImage.sh (relocated to rbw-iD)
- rbw-fS.BuildStudyDEBUG.sh (dead study debug)
- rbw-al.ListServiceAccounts.sh (relocated to rbw-Gl)
- rbw-aDS.DeleteServiceAccount.sh (relocated to rbw-GS)
- rbw-a.PodmanStart.sh (dead)
- rbw-aCD.CreateDirectorAccount.sh (dead, superseded by rbw-GD)
- rbw-aCR.CreateReaderAccount.sh (dead, superseded by rbw-GR)
- rbw-aIA.InitializeAdminAccount.sh (dead)
- rbw-aID.DELETE_ALL.sh (dead)
- rbw-aPO.ObliterateProject.sh (dead)
- rbw-aPr.RestoreProject.sh (dead)

## Code changes
- New rbf_summon function in rbf_Foundry.sh
  - Accept <vessel> <consecration>
  - Authenticate as Retriever (fallback Director, same as rbf_retrieve)
  - Construct both ark tags using RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT
  - HEAD check both artifacts (same as rbf_abjure pattern)
  - Docker login + docker pull for each existing artifact
  - Report which artifacts were retrieved, warn on orphans

## Coordinator changes (rbk_Coordinator.sh)
- Remove: rbw-fA, rbw-fB, rbw-fD, rbw-al, rbw-aDS
- Add: rbw-aA, rbw-ab, rbw-aC, rbw-as, rbw-Gl, rbw-GS, rbw-iD

## Spec changes
- Add :rbtgo_ark_summon: mapping in RBS0-SpecTop.adoc
- Add [[rbtgo_ark_summon]] section in Retriever Operations
- Create RBSAS-ark_summon.adoc lens following ark_abjure/ark_conjure pattern

## Verification
- All four ark tabtargets (aA, ab, aC, as) route correctly through coordinator
- Relocated governor and image tabtargets work
- Dead tabtargets removed
- rbf_summon pulls both artifacts for a given vessel+consecration
- RBSA terms consistent with implementation

**[260209-0607] rough**

Implement rbf_summon — ark-level image retrieval.

## Concept
rbf_summon is the ark-level counterpart to rbf_retrieve. Given a vessel and
consecration, it pulls both -image and -about artifacts as a coherent unit.
Pairs with rbf_abjure (ark-level delete) and rbf_conjure/rbf_build (ark-level build).

## Interface
rbf_summon <vessel> <consecration>

## Code changes
- New rbf_summon function in rbf_Foundry.sh
- Authenticate as Retriever (fallback to Director, same as rbf_retrieve)
- Construct both ark tags using RBGC_ARK_SUFFIX_IMAGE and RBGC_ARK_SUFFIX_ABOUT
- Verify both artifacts exist (HEAD check, like rbf_abjure)
- Pull both via docker pull
- Report which artifacts were retrieved

## Spec changes
- Add rbtgo_ark_summon mapping and definition in RBS0-SpecTop.adoc
- Create operation lens RBSAS-ark_summon.adoc
- Coordinator wiring and tabtarget

## Verification
- Pulls both -image and -about for a given vessel+consecration
- Reports if either artifact is missing (orphaned ark)
- Uses Retriever credentials when available
- RBSA terms consistent with implementation

### quadruple-build-poll-limit (₢APAAO) [complete]

**[260209-0755] complete**

Increase Cloud Build polling limit from 240 to 960 attempts.

## Location
`Tools/rbw/rbf_Foundry.sh:498`:
```bash
local z_max_attempts=240  # 20 minutes with 5 second intervals
```

## Change
```bash
local z_max_attempts=960  # 80 minutes with 5 second intervals
```

Update comment at line 628 accordingly.

**[260128-2042] bridled**

Increase Cloud Build polling limit from 240 to 960 attempts.

## Location
`Tools/rbw/rbf_Foundry.sh:498`:
```bash
local z_max_attempts=240  # 20 minutes with 5 second intervals
```

## Change
```bash
local z_max_attempts=960  # 80 minutes with 5 second intervals
```

Update comment at line 628 accordingly.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: Tools/rbw/rbf_Foundry.sh (1 file) | Steps: 1. Change z_max_attempts from 240 to 960 and update comment to 80 minutes 2. Update comment near zrbf_wait_build_completion call to reflect 5s x 960 = 80m | Verify: none required - bash script

**[260128-2039] rough**

Increase Cloud Build polling limit from 240 to 960 attempts.

## Location
`Tools/rbw/rbf_Foundry.sh:498`:
```bash
local z_max_attempts=240  # 20 minutes with 5 second intervals
```

## Change
```bash
local z_max_attempts=960  # 80 minutes with 5 second intervals
```

Update comment at line 628 accordingly.

### introduce-zipper-retire-formulary (₢APAAa) [complete]

**[260209-0807] complete**

Introduce zipper colophon registry and retire formulary concept.

## Concept

Two vocabulary changes to BUK dispatch infrastructure:
1. **Introduce zipper**: a BCG-compliant module that kindles array constants
   mapping colophons to their implementing modules and commands. Testbenches
   use symbolic constants instead of hardcoded colophon strings.
2. **Retire formulary**: the abstract superclass concept unifying workbench/
   testbench/coordinator adds naming cost without behavioral value. Replace
   with direct references to the concrete types (workbench, testbench,
   coordinator). Zipper takes over the data-mapping role formulary implied.

## New modules

### buz_zipper.sh (Tools/buk/)

BCG-compliant module, sourced only by testbenches and workbenches.

- zbuz_kindle() / zbuz_sentinel() — standard BCG lifecycle
- Three internal parallel arrays: zbuz_colophons[], zbuz_modules[], zbuz_commands[]
- buz_create_capture(colophon, module, command) — appends to arrays, prints index
- buz_get_colophon(idx), buz_get_module(idx), buz_get_command(idx) — getters

### rbz_zipper.sh (Tools/rbw/)

- rbz_kindle() registers all RBW coordinator-wired tuples:
    RBZ_ABJURE_ARK=$(buz_create_capture "rbw-aA" "rbf_Foundry" "rbf_abjure")
    RBZ_BESEECH_ARK=$(buz_create_capture "rbw-ab" "rbf_Foundry" "rbf_beseech")
    RBZ_CONJURE_ARK=$(buz_create_capture "rbw-aC" "rbf_Foundry" "rbf_build")
    RBZ_SUMMON_ARK=$(buz_create_capture "rbw-as" "rbf_Foundry" "rbf_summon")
    RBZ_DELETE_IMAGE=$(buz_create_capture "rbw-iD" "rbf_Foundry" "rbf_delete")
    RBZ_LIST_IMAGES=$(buz_create_capture "rbw-il" "rbf_Foundry" "rbf_list")
    RBZ_RETRIEVE_IMAGE=$(buz_create_capture "rbw-ir" "rbf_Foundry" "rbf_retrieve")
    ... (all coordinator-wired operations including Payor, Governor, Admin)

## Formulary removal

### Tools/buk/README.md (~30 references)
- Remove Formulary section and definition
- Remove formulary type table row
- Replace "formulary" in launcher/dispatch descriptions with "workbench"
  or "testbench" as context requires
- Add Zipper section alongside Workbench/Testbench documentation

### CLAUDE.md (3 references)
- BUK Concepts table: remove Formulary row, add Zipper row
- Tabtarget pattern description: remove formulary reference
- buw_workbench.sh description: remove "formulary" label

## Scope boundary

Baby step: constants per tuple only. No testbench consumption of zipper
in this pace — that comes in the testing paces that follow.

## Verification

- buz_zipper.sh passes shellcheck
- rbz_zipper.sh passes shellcheck
- rbz_kindle() registers all coordinator-wired colophons
- Constants are integers (array indices)
- Getters return correct values for each constant
- Zero occurrences of "formulary" in CLAUDE.md after edit
- BUK README coherent without formulary concept

**[260209-0747] rough**

Introduce zipper colophon registry and retire formulary concept.

## Concept

Two vocabulary changes to BUK dispatch infrastructure:
1. **Introduce zipper**: a BCG-compliant module that kindles array constants
   mapping colophons to their implementing modules and commands. Testbenches
   use symbolic constants instead of hardcoded colophon strings.
2. **Retire formulary**: the abstract superclass concept unifying workbench/
   testbench/coordinator adds naming cost without behavioral value. Replace
   with direct references to the concrete types (workbench, testbench,
   coordinator). Zipper takes over the data-mapping role formulary implied.

## New modules

### buz_zipper.sh (Tools/buk/)

BCG-compliant module, sourced only by testbenches and workbenches.

- zbuz_kindle() / zbuz_sentinel() — standard BCG lifecycle
- Three internal parallel arrays: zbuz_colophons[], zbuz_modules[], zbuz_commands[]
- buz_create_capture(colophon, module, command) — appends to arrays, prints index
- buz_get_colophon(idx), buz_get_module(idx), buz_get_command(idx) — getters

### rbz_zipper.sh (Tools/rbw/)

- rbz_kindle() registers all RBW coordinator-wired tuples:
    RBZ_ABJURE_ARK=$(buz_create_capture "rbw-aA" "rbf_Foundry" "rbf_abjure")
    RBZ_BESEECH_ARK=$(buz_create_capture "rbw-ab" "rbf_Foundry" "rbf_beseech")
    RBZ_CONJURE_ARK=$(buz_create_capture "rbw-aC" "rbf_Foundry" "rbf_build")
    RBZ_SUMMON_ARK=$(buz_create_capture "rbw-as" "rbf_Foundry" "rbf_summon")
    RBZ_DELETE_IMAGE=$(buz_create_capture "rbw-iD" "rbf_Foundry" "rbf_delete")
    RBZ_LIST_IMAGES=$(buz_create_capture "rbw-il" "rbf_Foundry" "rbf_list")
    RBZ_RETRIEVE_IMAGE=$(buz_create_capture "rbw-ir" "rbf_Foundry" "rbf_retrieve")
    ... (all coordinator-wired operations including Payor, Governor, Admin)

## Formulary removal

### Tools/buk/README.md (~30 references)
- Remove Formulary section and definition
- Remove formulary type table row
- Replace "formulary" in launcher/dispatch descriptions with "workbench"
  or "testbench" as context requires
- Add Zipper section alongside Workbench/Testbench documentation

### CLAUDE.md (3 references)
- BUK Concepts table: remove Formulary row, add Zipper row
- Tabtarget pattern description: remove formulary reference
- buw_workbench.sh description: remove "formulary" label

## Scope boundary

Baby step: constants per tuple only. No testbench consumption of zipper
in this pace — that comes in the testing paces that follow.

## Verification

- buz_zipper.sh passes shellcheck
- rbz_zipper.sh passes shellcheck
- rbz_kindle() registers all coordinator-wired colophons
- Constants are integers (array indices)
- Getters return correct values for each constant
- Zero occurrences of "formulary" in CLAUDE.md after edit
- BUK README coherent without formulary concept

**[260209-0747] rough**

Introduce zipper colophon registry and retire formulary concept.

## Concept

Two vocabulary changes to BUK dispatch infrastructure:
1. **Introduce zipper**: a BCG-compliant module that kindles array constants
   mapping colophons to their implementing modules and commands. Testbenches
   use symbolic constants instead of hardcoded colophon strings.
2. **Retire formulary**: the abstract superclass concept unifying workbench/
   testbench/coordinator adds naming cost without behavioral value. Replace
   with direct references to the concrete types (workbench, testbench,
   coordinator). Zipper takes over the data-mapping role formulary implied.

## New modules

### buz_zipper.sh (Tools/buk/)

BCG-compliant module, sourced only by testbenches and workbenches.

- zbuz_kindle() / zbuz_sentinel() — standard BCG lifecycle
- Three internal parallel arrays: zbuz_colophons[], zbuz_modules[], zbuz_commands[]
- buz_create_capture(colophon, module, command) — appends to arrays, prints index
- buz_get_colophon(idx), buz_get_module(idx), buz_get_command(idx) — getters

### rbz_zipper.sh (Tools/rbw/)

- rbz_kindle() registers all RBW coordinator-wired tuples:
    RBZ_ABJURE_ARK=$(buz_create_capture "rbw-aA" "rbf_Foundry" "rbf_abjure")
    RBZ_BESEECH_ARK=$(buz_create_capture "rbw-ab" "rbf_Foundry" "rbf_beseech")
    RBZ_CONJURE_ARK=$(buz_create_capture "rbw-aC" "rbf_Foundry" "rbf_build")
    RBZ_SUMMON_ARK=$(buz_create_capture "rbw-as" "rbf_Foundry" "rbf_summon")
    RBZ_DELETE_IMAGE=$(buz_create_capture "rbw-iD" "rbf_Foundry" "rbf_delete")
    RBZ_LIST_IMAGES=$(buz_create_capture "rbw-il" "rbf_Foundry" "rbf_list")
    RBZ_RETRIEVE_IMAGE=$(buz_create_capture "rbw-ir" "rbf_Foundry" "rbf_retrieve")
    ... (all coordinator-wired operations including Payor, Governor, Admin)

## Formulary removal

### Tools/buk/README.md (~30 references)
- Remove Formulary section and definition
- Remove formulary type table row
- Replace "formulary" in launcher/dispatch descriptions with "workbench"
  or "testbench" as context requires
- Add Zipper section alongside Workbench/Testbench documentation

### CLAUDE.md (3 references)
- BUK Concepts table: remove Formulary row, add Zipper row
- Tabtarget pattern description: remove formulary reference
- buw_workbench.sh description: remove "formulary" label

## Scope boundary

Baby step: constants per tuple only. No testbench consumption of zipper
in this pace — that comes in the testing paces that follow.

## Verification

- buz_zipper.sh passes shellcheck
- rbz_zipper.sh passes shellcheck
- rbz_kindle() registers all coordinator-wired colophons
- Constants are integers (array indices)
- Getters return correct values for each constant
- Zero occurrences of "formulary" in CLAUDE.md after edit
- BUK README coherent without formulary concept

**[260209-0739] rough**

Introduce zipper pattern: a colophon↔module↔command registry for testbench isolation.

## Concept

A "zipper" is a BCG-compliant module that kindles array constants mapping
colophons to their implementing modules and commands. Testbenches and workbenches
use symbolic constants instead of hardcoded colophon strings, so colophon renames
(like the rbw-fB→rbw-aC migration) require zero test edits.

Two layers:
- **buz_zipper.sh** (BUK framework): array infrastructure + create/get API
- **rbz_zipper.sh** (RBW kit registrations): constants per tuple

## Layer 1: buz_zipper.sh (Tools/buk/)

BCG-compliant module, but unusually only sourced by testbenches and workbenches,
not by other BCG modules.

- zbuz_kindle() / zbuz_sentinel() — standard BCG lifecycle
- Three internal parallel arrays: zbuz_colophons[], zbuz_modules[], zbuz_commands[]
- buz_create_capture(colophon, module, command) — appends to arrays, prints index
- buz_get_colophon(idx), buz_get_module(idx), buz_get_command(idx) — getters

## Layer 2: rbz_zipper.sh (Tools/rbw/)

- rbz_kindle() registers all RBW tuples and assigns constants:
    RBZ_ABJURE_ARK=$(buz_create_capture "rbw-aA" "rbf_Foundry" "rbf_abjure")
    RBZ_BESEECH_ARK=$(buz_create_capture "rbw-ab" "rbf_Foundry" "rbf_beseech")
    RBZ_CONJURE_ARK=$(buz_create_capture "rbw-aC" "rbf_Foundry" "rbf_build")
    RBZ_SUMMON_ARK=$(buz_create_capture "rbw-as" "rbf_Foundry" "rbf_summon")
    RBZ_DELETE_IMAGE=$(buz_create_capture "rbw-iD" "rbf_Foundry" "rbf_delete")
    RBZ_LIST_IMAGES=$(buz_create_capture "rbw-il" "rbf_Foundry" "rbf_list")
    RBZ_RETRIEVE_IMAGE=$(buz_create_capture "rbw-ir" "rbf_Foundry" "rbf_retrieve")
    ... (all coordinator-wired operations)

## Baby step scope

This pace creates buz_zipper.sh + rbz_zipper.sh with constants per tuple only.
No testbench consumption yet — that comes in the testing paces that follow.

## Documentation

Add zipper as a formulary type in Tools/buk/README.md alongside workbench/testbench.
Note the buf_ prefix reservation already exists for "formulary" — consider whether
buz_ (zipper-specific) or buf_ (generic formulary) is more appropriate.

## Verification

- buz_zipper.sh passes shellcheck
- rbz_zipper.sh passes shellcheck
- rbz_kindle() registers all coordinator-wired colophons
- Constants are integers (array indices)
- Getters return correct values for each constant

### bcg-register-pattern-zipper-fix (₢APAAb) [complete]

**[260209-1128] complete**

Add _register function type to BCG and apply to buz_zipper.sh to fix subshell array loss.

## Problem

buz_create_capture() has side effects (array mutation) AND returns a value via echo.
When called in $() from rbz_kindle, the subshell discards array modifications.
Registry arrays (zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
are always empty after kindle. The dispatch workaround (on-demand colophon resolution)
works but leaves dead getters and an inconsistent design.

## BCG Change: _register function type

Add a new special function type to BCG-BashConsoleGuide.md:

### Contract
- Mutates shared state AND returns value(s)
- Return via `z1z_<prefix>_<term>` variables (NOT echo, NOT Z-prefixed kindle constants)
- `z1z_` prefix signals: rare bootstrap return channel, not a kindle constant
- Must NOT be called inside $() — side effects would be lost
- May use buc_die internally (unlike _capture)
- Should rarely be used — only when a function must both mutate and return

### BCG sections to update
- Special Function Patterns table (add _register row)
- Naming Convention Patterns table (add z1z_ row)
- Module Maturity Checklist (add _register check)
- Quick Reference Decision Matrix (add _register guidance)

## Code changes

### buz_zipper.sh
- Rename buz_create_capture → buz_register
- Set z1z_buz_colophon (colophon string) after registration
- Remove dead index-based getters OR verify they work with populated arrays
- Decide: dispatch accepts colophon string (current) or index (restored)?

### rbz_zipper.sh
- Update all callers:
  buz_register "rbw-il" "rbf_Foundry" "rbf_list"
  RBZ_LIST_IMAGES="${z1z_buz_colophon}"

## Verification
- BCG doc updated with _register pattern
- shellcheck passes on buz_zipper.sh, rbz_zipper.sh
- Dispatch exercise still passes (tt/rbtg-de.DispatchExercise.sh)
- Registry arrays populated after kindle (verify with a quick test)

**[260209-1120] rough**

Add _register function type to BCG and apply to buz_zipper.sh to fix subshell array loss.

## Problem

buz_create_capture() has side effects (array mutation) AND returns a value via echo.
When called in $() from rbz_kindle, the subshell discards array modifications.
Registry arrays (zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
are always empty after kindle. The dispatch workaround (on-demand colophon resolution)
works but leaves dead getters and an inconsistent design.

## BCG Change: _register function type

Add a new special function type to BCG-BashConsoleGuide.md:

### Contract
- Mutates shared state AND returns value(s)
- Return via `z1z_<prefix>_<term>` variables (NOT echo, NOT Z-prefixed kindle constants)
- `z1z_` prefix signals: rare bootstrap return channel, not a kindle constant
- Must NOT be called inside $() — side effects would be lost
- May use buc_die internally (unlike _capture)
- Should rarely be used — only when a function must both mutate and return

### BCG sections to update
- Special Function Patterns table (add _register row)
- Naming Convention Patterns table (add z1z_ row)
- Module Maturity Checklist (add _register check)
- Quick Reference Decision Matrix (add _register guidance)

## Code changes

### buz_zipper.sh
- Rename buz_create_capture → buz_register
- Set z1z_buz_colophon (colophon string) after registration
- Remove dead index-based getters OR verify they work with populated arrays
- Decide: dispatch accepts colophon string (current) or index (restored)?

### rbz_zipper.sh
- Update all callers:
  buz_register "rbw-il" "rbf_Foundry" "rbf_list"
  RBZ_LIST_IMAGES="${z1z_buz_colophon}"

## Verification
- BCG doc updated with _register pattern
- shellcheck passes on buz_zipper.sh, rbz_zipper.sh
- Dispatch exercise still passes (tt/rbtg-de.DispatchExercise.sh)
- Registry arrays populated after kindle (verify with a quick test)

**[260209-1120] rough**

Add _register function type to BCG and apply to buz_zipper.sh to fix subshell array loss.

## Problem

buz_create_capture() has side effects (array mutation) AND returns a value via echo.
When called in $() from rbz_kindle, the subshell discards array modifications.
Registry arrays (zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
are always empty after kindle. The dispatch workaround (on-demand colophon resolution)
works but leaves dead getters and an inconsistent design.

## BCG Change: _register function type

Add a new special function type to BCG-BashConsoleGuide.md:

### Contract
- Mutates shared state AND returns value(s)
- Return via `z1z_<prefix>_<term>` variables (NOT echo, NOT Z-prefixed kindle constants)
- `z1z_` prefix signals: rare bootstrap return channel, not a kindle constant
- Must NOT be called inside $() — side effects would be lost
- May use buc_die internally (unlike _capture)
- Should rarely be used — only when a function must both mutate and return

### BCG sections to update
- Special Function Patterns table (add _register row)
- Naming Convention Patterns table (add z1z_ row)
- Module Maturity Checklist (add _register check)
- Quick Reference Decision Matrix (add _register guidance)

## Code changes

### buz_zipper.sh
- Rename buz_create_capture → buz_register
- Set z1z_buz_colophon (colophon string) after registration
- Remove dead index-based getters OR verify they work with populated arrays
- Decide: dispatch accepts colophon string (current) or index (restored)?

### rbz_zipper.sh
- Update all callers:
  buz_register "rbw-il" "rbf_Foundry" "rbf_list"
  RBZ_LIST_IMAGES="${z1z_buz_colophon}"

## Verification
- BCG doc updated with _register pattern
- shellcheck passes on buz_zipper.sh, rbz_zipper.sh
- Dispatch exercise still passes (tt/rbtg-de.DispatchExercise.sh)
- Registry arrays populated after kindle (verify with a quick test)

**[260209-1027] rough**

Add dispatch infrastructure to buz_zipper.sh, introduce BURV virtual regime for testbench isolation, and create fast dispatch exercise testbench.

## BURV Virtual Regime (Bash Utility Regime Verification)

Environment-only override regime. No file — testbench exports BURV_* env vars, bul_launcher.sh applies them over corresponding BURC_* values.

### bul_launcher.sh changes
After sourcing burc.env, apply verification overrides:
- `BURC_OUTPUT_ROOT_DIR="${BURV_OUTPUT_ROOT_DIR:-${BURC_OUTPUT_ROOT_DIR}}"`
- `BURC_TEMP_ROOT_DIR="${BURV_TEMP_ROOT_DIR:-${BURC_TEMP_ROOT_DIR}}"`

This prevents inner dispatches (tabtargets under test) from colliding with the testbench's own output/temp/log dirs.

## buz_zipper.sh updates

### 4th parallel array: tabtargets
- Add `zbuz_tabtargets=()` array to kindle
- Add `buz_get_tabtarget(idx)` getter
- Update `buz_create_capture` to resolve tabtarget via bash glob (builtin, no subprocess):
  ```
  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)
  ```
- Die at kindle time if 0 matches (missing tabtarget) or >1 match (ambiguous colophon)
- Bash 3.2 compliant: no-match glob returns literal, check with `test -e`

### Dispatch + evidence infrastructure
- `buz_init_evidence()` — create evidence root dir under testbench temp
- Step result parallel arrays: `zbuz_step_colophon_idx[]`, `zbuz_step_exit_status[]`, `zbuz_step_output_dir[]`
- `buz_dispatch_capture(colophon_idx, ...)` — invoke tabtarget via BURV-isolated environment, harvest overridden output dir to evidence subdir via `cp -r`, record in step arrays, return step index
- `buz_dispatch_expect_ok(colophon_idx, ...)` — dispatch_capture + buc_die on non-zero exit
- `buz_dispatch_expect_fail(colophon_idx, ...)` — dispatch_capture + buc_die on zero exit
- Step getters: `buz_get_step_exit(step_idx)`, `buz_get_step_output(step_idx)`

## New file: Tools/rbw/rbtg_testbench.sh

BCG-compliant testbench (Recipe Bottle Testbench Google).

### rbtg_case_dispatch_exercise

Fast dispatch exercise — validates plumbing without cloud builds:

| Step | Zipper Constant | Check After |
|------|----------------|-------------|
| 1 | RBZ_LIST_IMAGES | Exit 0, evidence dir populated |

Sources buz_zipper.sh + rbz_zipper.sh, kindles both.
Validates BURV isolation: inner dispatch uses overridden output dir, testbench output dir untouched.

## Wiring

- Tabtarget: `tt/rbtg-de.DispatchExercise.<imprint>.sh` (or similar)
- Launcher: `.buk/launcher.rbtg_testbench.sh`

## Verification

- buz_zipper.sh passes shellcheck
- rbtg_testbench.sh passes shellcheck
- bul_launcher.sh passes shellcheck
- Kindle validates all RBZ colophons resolve to exactly one tabtarget
- Dispatch exercise passes — BURV isolation confirmed

## Bash 3.2 compliance

All new code must work with bash 3.2 (macOS default). No bash 4+ features.

**[260209-1027] rough**

Add dispatch infrastructure to buz_zipper.sh, introduce BURV virtual regime for testbench isolation, and create fast dispatch exercise testbench.

## BURV Virtual Regime (Bash Utility Regime Verification)

Environment-only override regime. No file — testbench exports BURV_* env vars, bul_launcher.sh applies them over corresponding BURC_* values.

### bul_launcher.sh changes
After sourcing burc.env, apply verification overrides:
- `BURC_OUTPUT_ROOT_DIR="${BURV_OUTPUT_ROOT_DIR:-${BURC_OUTPUT_ROOT_DIR}}"`
- `BURC_TEMP_ROOT_DIR="${BURV_TEMP_ROOT_DIR:-${BURC_TEMP_ROOT_DIR}}"`

This prevents inner dispatches (tabtargets under test) from colliding with the testbench's own output/temp/log dirs.

## buz_zipper.sh updates

### 4th parallel array: tabtargets
- Add `zbuz_tabtargets=()` array to kindle
- Add `buz_get_tabtarget(idx)` getter
- Update `buz_create_capture` to resolve tabtarget via bash glob (builtin, no subprocess):
  ```
  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)
  ```
- Die at kindle time if 0 matches (missing tabtarget) or >1 match (ambiguous colophon)
- Bash 3.2 compliant: no-match glob returns literal, check with `test -e`

### Dispatch + evidence infrastructure
- `buz_init_evidence()` — create evidence root dir under testbench temp
- Step result parallel arrays: `zbuz_step_colophon_idx[]`, `zbuz_step_exit_status[]`, `zbuz_step_output_dir[]`
- `buz_dispatch_capture(colophon_idx, ...)` — invoke tabtarget via BURV-isolated environment, harvest overridden output dir to evidence subdir via `cp -r`, record in step arrays, return step index
- `buz_dispatch_expect_ok(colophon_idx, ...)` — dispatch_capture + buc_die on non-zero exit
- `buz_dispatch_expect_fail(colophon_idx, ...)` — dispatch_capture + buc_die on zero exit
- Step getters: `buz_get_step_exit(step_idx)`, `buz_get_step_output(step_idx)`

## New file: Tools/rbw/rbtg_testbench.sh

BCG-compliant testbench (Recipe Bottle Testbench Google).

### rbtg_case_dispatch_exercise

Fast dispatch exercise — validates plumbing without cloud builds:

| Step | Zipper Constant | Check After |
|------|----------------|-------------|
| 1 | RBZ_LIST_IMAGES | Exit 0, evidence dir populated |

Sources buz_zipper.sh + rbz_zipper.sh, kindles both.
Validates BURV isolation: inner dispatch uses overridden output dir, testbench output dir untouched.

## Wiring

- Tabtarget: `tt/rbtg-de.DispatchExercise.<imprint>.sh` (or similar)
- Launcher: `.buk/launcher.rbtg_testbench.sh`

## Verification

- buz_zipper.sh passes shellcheck
- rbtg_testbench.sh passes shellcheck
- bul_launcher.sh passes shellcheck
- Kindle validates all RBZ colophons resolve to exactly one tabtarget
- Dispatch exercise passes — BURV isolation confirmed

## Bash 3.2 compliance

All new code must work with bash 3.2 (macOS default). No bash 4+ features.

**[260209-0840] rough**

Add dispatch infrastructure to buz_zipper.sh and create first integration testbench.

## buz_zipper.sh updates

### 4th parallel array: tabtargets
- Add `zbuz_tabtargets=()` array to kindle
- Add `buz_get_tabtarget(idx)` getter
- Update `buz_create_capture` to resolve tabtarget via bash glob (builtin, no subprocess):
  ```
  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)
  ```
- Die at kindle time if 0 matches (missing tabtarget) or >1 match (ambiguous colophon)
- Bash 3.2 compliant: no-match glob returns literal, check with `test -e`

### Dispatch + evidence infrastructure
- `buz_init_evidence()` — create evidence root dir under testbench temp
- Step result parallel arrays: `zbuz_step_colophon_idx[]`, `zbuz_step_exit_status[]`, `zbuz_step_output_dir[]`
- `buz_dispatch_capture(colophon_idx, ...)` — invoke tabtarget (from resolved path, no subprocess lookup), harvest `${BURC_OUTPUT_ROOT_DIR}/current` to evidence subdir via `cp -r`, record in step arrays, return step index
- `buz_dispatch_expect_ok(colophon_idx, ...)` — dispatch_capture + buc_die on non-zero exit
- `buz_dispatch_expect_fail(colophon_idx, ...)` — dispatch_capture + buc_die on zero exit
- Step getters: `buz_get_step_exit(step_idx)`, `buz_get_step_output(step_idx)`

## New file: Tools/rbw/rbtg_testbench.sh

BCG-compliant testbench (Recipe Bottle Testbench Google).

### rbtg_case_ark_lifecycle

Full external-boundary integration test using zipper dispatch:

| Step | Zipper Constant | Check After |
|------|----------------|-------------|
| 1 | RBZ_LIST_IMAGES | Exit 0, capture baseline |
| 2 | RBZ_CONJURE_ARK | Exit 0, FQIN file in evidence → harvest FQIN |
| 3 | RBZ_LIST_IMAGES | Exit 0, count = baseline + 1 |
| 4 | RBZ_RETRIEVE_IMAGE | Exit 0, artifact in evidence (using harvested tag) |
| 5 | RBZ_DELETE_IMAGE | Exit 0 |
| 6 | RBZ_LIST_IMAGES | Exit 0, count = baseline |

Sources buz_zipper.sh + rbz_zipper.sh, kindles both.
Uses `trbim_dockerfile.recipe` as test recipe.

## Wiring

- Tabtarget: `tt/rbtg-al.ArkLifecycle.<imprint>.sh` (or similar)
- Launcher: `.buk/launcher.rbtg_testbench.sh`

## Verification

- buz_zipper.sh passes shellcheck
- rbtg_testbench.sh passes shellcheck
- Kindle validates all RBZ colophons resolve to exactly one tabtarget
- Full ark lifecycle passes against live GCP environment

## Bash 3.2 compliance

All new code must work with bash 3.2 (macOS default). No bash 4+ features.

### rename-bud-to-burd (₢APAAc) [complete]

**[260209-1141] complete**

Rename all BUD_ items to BURD_ (Bash Utility Regime Dispatch) across the codebase.

## Scope

Mechanical rename in these files:
- Tools/buk/bud_dispatch.sh — all BUD_* variables → BURD_*, zbud_* → zburd_*, rename file to burd_dispatch.sh
- Tools/buk/bul_launcher.sh — BUD_REGIME_FILE, BUD_COORDINATOR_SCRIPT, BUD_STATION_FILE → BURD_*
- Tools/buk/burc_regime.sh — any BUD_ references
- Tools/buk/burc_cli.sh — any BUD_ references
- Tools/buk/buc_command.sh — any BUD_ references
- All tabtargets in tt/ — BUD_NO_LOG, BUD_INTERACTIVE etc.
- Tools/buk/README.md — documentation references
- Tools/buk/burc_specification.md — specification references
- Any other files referencing BUD_ or zbud_

## Rules
- BUD_ → BURD_ (public dispatch regime variables)
- zbud_ → zburd_ (private dispatch functions)
- BUD_VERBOSE, BUD_NO_LOG, BUD_INTERACTIVE etc. all become BURD_*
- zbud_die → zburd_die, zbud_show → zburd_show etc.
- BURC_* unchanged (file-based config regime)
- BURS_* unchanged (secrets regime)
- BURV_* unchanged (verification regime — new in prior pace)

## Strategy
Recommend parallel haiku agents — files are independent. Partition by file or file group:
- Agent 1: bud_dispatch.sh (heaviest — bulk of references + file rename)
- Agent 2: bul_launcher.sh + burc_regime.sh + burc_cli.sh + buc_command.sh
- Agent 3: All tt/*.sh tabtargets
- Agent 4: README.md + burc_specification.md + any other docs
Verify with grep that no BUD_ or zbud_ references remain after all agents complete.

## Verification
- `grep -r 'BUD_' Tools/buk/` returns zero matches (only BURD_)
- `grep -r 'zbud_' Tools/buk/` returns zero matches (only zburd_)
- `grep -r 'BUD_' tt/` returns zero matches
- All tabtargets still execute (smoke test a few)
- shellcheck passes on modified files

**[260209-1027] rough**

Rename all BUD_ items to BURD_ (Bash Utility Regime Dispatch) across the codebase.

## Scope

Mechanical rename in these files:
- Tools/buk/bud_dispatch.sh — all BUD_* variables → BURD_*, zbud_* → zburd_*, rename file to burd_dispatch.sh
- Tools/buk/bul_launcher.sh — BUD_REGIME_FILE, BUD_COORDINATOR_SCRIPT, BUD_STATION_FILE → BURD_*
- Tools/buk/burc_regime.sh — any BUD_ references
- Tools/buk/burc_cli.sh — any BUD_ references
- Tools/buk/buc_command.sh — any BUD_ references
- All tabtargets in tt/ — BUD_NO_LOG, BUD_INTERACTIVE etc.
- Tools/buk/README.md — documentation references
- Tools/buk/burc_specification.md — specification references
- Any other files referencing BUD_ or zbud_

## Rules
- BUD_ → BURD_ (public dispatch regime variables)
- zbud_ → zburd_ (private dispatch functions)
- BUD_VERBOSE, BUD_NO_LOG, BUD_INTERACTIVE etc. all become BURD_*
- zbud_die → zburd_die, zbud_show → zburd_show etc.
- BURC_* unchanged (file-based config regime)
- BURS_* unchanged (secrets regime)
- BURV_* unchanged (verification regime — new in prior pace)

## Strategy
Recommend parallel haiku agents — files are independent. Partition by file or file group:
- Agent 1: bud_dispatch.sh (heaviest — bulk of references + file rename)
- Agent 2: bul_launcher.sh + burc_regime.sh + burc_cli.sh + buc_command.sh
- Agent 3: All tt/*.sh tabtargets
- Agent 4: README.md + burc_specification.md + any other docs
Verify with grep that no BUD_ or zbud_ references remain after all agents complete.

## Verification
- `grep -r 'BUD_' Tools/buk/` returns zero matches (only BURD_)
- `grep -r 'zbud_' Tools/buk/` returns zero matches (only zburd_)
- `grep -r 'BUD_' tt/` returns zero matches
- All tabtargets still execute (smoke test a few)
- shellcheck passes on modified files

### ark-lifecycle-cloud-testbench (₢APAAd) [complete]

**[260209-1622] complete**

Full external-boundary integration test for ark lifecycle using zipper dispatch.

## Prerequisites
- Dispatch infrastructure and BURV regime from zipper-dispatch-burv-testbench pace
- BURD_ rename complete

## rbtg_case_ark_lifecycle

Add to Tools/rbw/rbtg_testbench.sh:

| Step | Zipper Constant | Check After |
|------|----------------|-------------|
| 1 | RBZ_LIST_IMAGES | Exit 0, capture baseline count |
| 2 | RBZ_CONJURE_ARK | Exit 0, FQIN file in evidence → harvest FQIN |
| 3 | RBZ_LIST_IMAGES | Exit 0, count = baseline + 1 |
| 4 | RBZ_RETRIEVE_IMAGE | Exit 0, artifact in evidence (using harvested tag) |
| 5 | RBZ_DELETE_IMAGE | Exit 0 |
| 6 | RBZ_LIST_IMAGES | Exit 0, count = baseline |

Sources buz_zipper.sh + rbz_zipper.sh, kindles both.
Uses trbim_dockerfile.recipe as test recipe.

## Wiring
- Tabtarget: tt/rbtg-al.ArkLifecycle.<imprint>.sh

## Verification
- Full ark lifecycle passes against live GCP environment
- Evidence dirs contain expected artifacts at each step
- Baseline count restored after delete
- BURV isolation holds throughout multi-step sequence

**[260209-1027] rough**

Full external-boundary integration test for ark lifecycle using zipper dispatch.

## Prerequisites
- Dispatch infrastructure and BURV regime from zipper-dispatch-burv-testbench pace
- BURD_ rename complete

## rbtg_case_ark_lifecycle

Add to Tools/rbw/rbtg_testbench.sh:

| Step | Zipper Constant | Check After |
|------|----------------|-------------|
| 1 | RBZ_LIST_IMAGES | Exit 0, capture baseline count |
| 2 | RBZ_CONJURE_ARK | Exit 0, FQIN file in evidence → harvest FQIN |
| 3 | RBZ_LIST_IMAGES | Exit 0, count = baseline + 1 |
| 4 | RBZ_RETRIEVE_IMAGE | Exit 0, artifact in evidence (using harvested tag) |
| 5 | RBZ_DELETE_IMAGE | Exit 0 |
| 6 | RBZ_LIST_IMAGES | Exit 0, count = baseline |

Sources buz_zipper.sh + rbz_zipper.sh, kindles both.
Uses trbim_dockerfile.recipe as test recipe.

## Wiring
- Tabtarget: tt/rbtg-al.ArkLifecycle.<imprint>.sh

## Verification
- Full ark lifecycle passes against live GCP environment
- Evidence dirs contain expected artifacts at each step
- Baseline count restored after delete
- BURV isolation holds throughout multi-step sequence

### build-and-stage-nsproto-images (₢APAAJ) [complete]

**[260212-1137] complete**

Conjure fresh RB Arks for nsproto vessels and set nameplate consecration references.

## Context

The nsproto nameplate (`Tools/rbw/rbrn_nsproto.env`) has empty `RBRN_SENTRY_CONSECRATION` and `RBRN_BOTTLE_CONSECRATION` fields. Existing GAR images use pre-ark tag format (`-img`/`-meta`). The test suite needs arks with current format (`-image`/`-about`) and the nameplate ark reference variables populated.

Vessels:
- `rbev-sentry-ubuntu-large` → nameplate field `RBRN_SENTRY_CONSECRATION`
- `rbev-bottle-ubuntu-test` → nameplate field `RBRN_BOTTLE_CONSECRATION`

## Steps

1. **Conjure sentry ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-sentry-ubuntu-large`
   Creates paired Ark Image Artifact and Ark About Artifact with shared Consecration timestamp.
2. **Conjure bottle ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-bottle-ubuntu-test`
3. **Beseech to identify consecrations** — `tt/rbw-ab.BeseechArk.sh` to see ark pairs and extract consecration timestamps
4. **Set nameplate ark references** — Edit `Tools/rbw/rbrn_nsproto.env`:
   - `RBRN_SENTRY_CONSECRATION=<sentry consecration>`
   - `RBRN_BOTTLE_CONSECRATION=<bottle consecration>`
5. **Summon arks locally** — `tt/rbw-as.SummonArk.sh <vessel> <consecration>` for both vessels. Pulls both -image and -about artifacts to local container runtime.
6. **Verify** — `docker images` shows sentry and bottle images available locally

## Notes

- Git must be clean+pushed before each conjure (rbf_build enforces this)
- Both vessels use multi-platform with `allow` binfmt — arm64 builds via QEMU may be slow for ubuntu-based images
- Consecration format per spec: `YYYYMMDDTHHMMSSZ`

**[260209-1632] rough**

Conjure fresh RB Arks for nsproto vessels and set nameplate consecration references.

## Context

The nsproto nameplate (`Tools/rbw/rbrn_nsproto.env`) has empty `RBRN_SENTRY_CONSECRATION` and `RBRN_BOTTLE_CONSECRATION` fields. Existing GAR images use pre-ark tag format (`-img`/`-meta`). The test suite needs arks with current format (`-image`/`-about`) and the nameplate ark reference variables populated.

Vessels:
- `rbev-sentry-ubuntu-large` → nameplate field `RBRN_SENTRY_CONSECRATION`
- `rbev-bottle-ubuntu-test` → nameplate field `RBRN_BOTTLE_CONSECRATION`

## Steps

1. **Conjure sentry ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-sentry-ubuntu-large`
   Creates paired Ark Image Artifact and Ark About Artifact with shared Consecration timestamp.
2. **Conjure bottle ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-bottle-ubuntu-test`
3. **Beseech to identify consecrations** — `tt/rbw-ab.BeseechArk.sh` to see ark pairs and extract consecration timestamps
4. **Set nameplate ark references** — Edit `Tools/rbw/rbrn_nsproto.env`:
   - `RBRN_SENTRY_CONSECRATION=<sentry consecration>`
   - `RBRN_BOTTLE_CONSECRATION=<bottle consecration>`
5. **Summon arks locally** — `tt/rbw-as.SummonArk.sh <vessel> <consecration>` for both vessels. Pulls both -image and -about artifacts to local container runtime.
6. **Verify** — `docker images` shows sentry and bottle images available locally

## Notes

- Git must be clean+pushed before each conjure (rbf_build enforces this)
- Both vessels use multi-platform with `allow` binfmt — arm64 builds via QEMU may be slow for ubuntu-based images
- Consecration format per spec: `YYYYMMDDTHHMMSSZ`

**[260209-1628] rough**

Conjure fresh nsproto vessel arks and stage them for local test suite execution.

## Context

Existing GAR images use old tag format (-img/-meta) from pre-ark vocabulary. Need fresh builds with current ark format (-image/-about) and consecration timestamps.

Vessels to conjure:
- `rbev-sentry-ubuntu-large` — sentry container for network isolation
- `rbev-bottle-ubuntu-test` — bottle container with test services

Both need arm64 builds (macOS development). Cloud Build handles this natively (~90s for busybox-class, longer for ubuntu-based).

## Steps

1. **Conjure sentry ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-sentry-ubuntu-large`
2. **Conjure bottle ark** — `tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-bottle-ubuntu-test`
3. **List to get consecration timestamps** — `tt/rbw-il.ImageList.sh` and identify new -image/-about tags
4. **Update nameplate** — Set `RBRN_SENTRY_CONSECRATION` and `RBRN_BOTTLE_CONSECRATION` in `Tools/rbw/rbrn_nsproto.env`
5. **Summon arks locally** — `tt/rbw-as.SummonArk.sh <vessel> <consecration>` for both
6. **Verify** — `docker images` shows both sentry and bottle images available locally

## Key Commands

- Conjure: `tt/rbw-aC.ConjureArk.sh rbev-vessels/<vessel>`
- List: `tt/rbw-il.ImageList.sh`
- Summon: `tt/rbw-as.SummonArk.sh <vessel> <consecration>`
- Nameplate: `Tools/rbw/rbrn_nsproto.env`

## Notes

- Vessel binfmt_policy is `allow` with multi-platform — Cloud Build may be slow for cross-arch (arm64 via QEMU). Consider single-platform vessels if build time is painful.
- Git must be clean+pushed before conjure (rbf_build enforces this)
- Consecration format: `YYYYMMDDTHHMMSSZ`

**[260209-0711] rough**

Build nsproto vessel images and stage them for local testing.

## Completed
- Created vessels: rbev-sentry-ubuntu-large, rbev-bottle-ubuntu-test
- Updated nameplates (nsproto, pluml, srjcl) with new monikers
- Deleted local build infrastructure (RBM-recipes/, LocalBuild tabtargets)
- Built sentry image successfully: rbev-sentry-ubuntu-large in GAR

## Remaining
1. **Rebuild bottle image** — Previous build cancelled (slow arm64 via QEMU). Options:
   - Retry with patience (~20-30 min for arm64)
   - Or modify vessel to amd64-only for faster builds
2. **Verify images in registry** — Run tt/rbw-il.ImageList.sh to get locators for both monikers
3. **Update nameplate** — Set RBRN_SENTRY_CONSECRATION and RBRN_BOTTLE_CONSECRATION in Tools/rbw/rbrn_nsproto.env
4. **Summon images** — Use tt/rbw-as.SummonArk.sh <vessel> <consecration> to pull from GAR to local Docker
5. **Verify** — Confirm images available locally with docker images

## Key Commands
- Conjure: tt/rbw-aC.ConjureArk.sh rbev-vessels/rbev-bottle-ubuntu-test
- List: tt/rbw-il.ImageList.sh (outputs rbst_locator values)
- Summon: tt/rbw-as.SummonArk.sh <vessel> <consecration> (ark-level retrieve)
- Retrieve: tt/rbw-ir.RetrieveImage.sh <locator> (plain single-tag retrieve)
- Nameplate: Tools/rbw/rbrn_nsproto.env

**[260209-0608] rough**

Build nsproto vessel images and stage them for local testing.

## Completed
- Created vessels: rbev-sentry-ubuntu-large, rbev-bottle-ubuntu-test
- Updated nameplates (nsproto, pluml, srjcl) with new monikers
- Deleted local build infrastructure (RBM-recipes/, LocalBuild tabtargets)
- Built sentry image successfully: rbev-sentry-ubuntu-large in GAR

## Remaining
1. **Rebuild bottle image** — Previous build cancelled (slow arm64 via QEMU). Options:
   - Retry with patience (~20-30 min for arm64)
   - Or modify vessel to amd64-only for faster builds
2. **Verify images in registry** — Run tt/rbw-il.ImageList.sh to get locators for both monikers
3. **Update nameplate** — Set RBRN_SENTRY_CONSECRATION and RBRN_BOTTLE_CONSECRATION in Tools/rbw/rbrn_nsproto.env
4. **Summon images** — Use rbf_summon (or rbf_retrieve with locator) to pull from GAR to local Docker
5. **Verify** — Confirm images available locally with docker images

## Key Commands
- Build: tt/rbw-fB.BuildVessel.sh rbev-vessels/rbev-bottle-ubuntu-test
- List: tt/rbw-il.ImageList.sh (outputs rbst_locator values)
- Summon: rbf_summon <vessel> <consecration> (ark-level retrieve)
- Retrieve: tt/rbw-ir.RetrieveImage.sh <locator> (plain single-tag retrieve)
- Nameplate: Tools/rbw/rbrn_nsproto.env

**[260128-0830] rough**

Build nsproto vessel images and stage them for local testing.

## Completed
- Created vessels: `rbev-sentry-ubuntu-large`, `rbev-bottle-ubuntu-test`
- Updated nameplates (nsproto, pluml, srjcl) with new monikers
- Deleted local build infrastructure (RBM-recipes/, LocalBuild tabtargets)
- Built sentry image successfully: `rbev-sentry-ubuntu-large` in GAR

## Remaining
1. **Rebuild bottle image** — Previous build cancelled (slow arm64 via QEMU). Options:
   - Retry with patience (~20-30 min for arm64)
   - Or modify vessel to amd64-only for faster builds
2. **Get new tags** — Run `tt/rbw-il.ImageList.sh` for both monikers
3. **Update nameplate** — Set RBRN_*_IMAGE_TAG values in `Tools/rbw/rbrn_nsproto.env`
4. **Retrieve images** — Pull from GAR to local Docker via `tt/rbw-ir.RetrieveImage.sh`
5. **Verify** — Confirm images available locally with `docker images`

## Key Commands
- Build: `tt/rbw-fB.BuildVessel.sh rbev-vessels/rbev-bottle-ubuntu-test`
- List: `tt/rbw-il.ImageList.sh rbev-sentry-ubuntu-large` (and bottle)
- Retrieve: `tt/rbw-ir.RetrieveImage.sh <moniker>:<tag>`

**[260128-0738] rough**

Build nsproto vessel images and stage them for local testing.

## Steps

1. **Build images** — Run rbf_build for nsproto sentry and bottle images via Cloud Build
2. **Get new tags** — Run rbf_list to get the newly created image tags
3. **Update nameplate** — Edit Tools/rbw/rbrn_nsproto.env with new RBRN_SENTRY_IMAGE_TAG and RBRN_BOTTLE_IMAGE_TAG values
4. **Retrieve images** — Run rbf_retrieve (via tt/rbw-ir.RetrieveImage.sh) to pull images from GAR to local Docker
5. **Verify** — Confirm images are available locally with docker images

## Key Commands

- Build: tt/rbw-tb.TriggerBuild.sh (or rbf_build directly)
- List: tt/rbw-il.ListImages.sh
- Retrieve: tt/rbw-ir.RetrieveImage.sh
- Nameplate: Tools/rbw/rbrn_nsproto.env

## Success Criteria

- Images exist in GAR with new tags
- Nameplate updated with matching tags
- Images pulled to local Docker daemon
- Ready for test suite execution

**[260128-0709] rough**

Build nsproto vessel images via Cloud Build and verify they appear in Artifact Registry. Run rbf_build for the nsproto vessel directory. After successful build, run rbf_list to confirm images with tags are present. This ensures test suite has current images to test against.

### run-nsproto-test-suite (₢APAAB) [complete]

**[260212-1140] complete**

Run all 22 nsproto security tests. These validate the censer network isolation model: DNS filtering, TCP 443 restrictions, package blocking, ICMP behavior. Tabtarget: tt/rbw-to.TestBottleService.nsproto.sh

**[260125-0837] rough**

Run all 22 nsproto security tests. These validate the censer network isolation model: DNS filtering, TCP 443 restrictions, package blocking, ICMP behavior. Tabtarget: tt/rbw-to.TestBottleService.nsproto.sh

### run-srjcl-test-suite (₢APAAC) [complete]

**[260212-1203] complete**

Run all 3 srjcl Jupyter tests. These validate the Jupyter notebook service: container running, HTTP connectivity from host, WebSocket kernel communication. Tabtarget: tt/rbw-to.TestBottleService.srjcl.sh

**[260125-0837] rough**

Run all 3 srjcl Jupyter tests. These validate the Jupyter notebook service: container running, HTTP connectivity from host, WebSocket kernel communication. Tabtarget: tt/rbw-to.TestBottleService.srjcl.sh

### run-pluml-test-suite (₢APAAD) [complete]

**[260212-1206] complete**

Run all 5 pluml PlantUML tests. These validate the PlantUML rendering service: text rendering, local diagram generation, HTTP headers, invalid hash handling, malformed diagram rejection. Tabtarget: tt/rbw-to.TestBottleService.pluml.sh

**[260125-0837] rough**

Run all 5 pluml PlantUML tests. These validate the PlantUML rendering service: text rendering, local diagram generation, HTTP headers, invalid hash handling, malformed diagram rejection. Tabtarget: tt/rbw-to.TestBottleService.pluml.sh

### pin-gcb-tool-versions (₢APAAH) [complete]

**[260212-1327] complete**

Pin all GCB step builder images to digest refs in RBRR, and build a refresh tabtarget to maintain them.

## Images to pin (add to existing RBRR_GCB_*_IMAGE_REF group, lines 49-57)

Six unpinned images, all used in GCB build steps:
1. gcr.io/cloud-builders/gcloud — steps 01, 02 (tag derivation, auth token)
2. gcr.io/cloud-builders/docker — steps 03, 04, 06, 08, 09 (docker/buildx ops)
3. quay.io/skopeo/stable — step 07 (OCI push to GAR)
4. alpine — step 10 (metadata JSON assembly)
5. anchore/syft:latest — step 08, hardcoded in rbgjb08-sbom-and-summary.sh:12
6. tonistiigi/binfmt — step 04, hardcoded in rbgjb04-qemu-binfmt.sh:8

Two already pinned (gcrane, oras) — refresh command should also cover these.

## Refresh tabtarget: tt/rbw-gp.RefreshGcbPins.sh

Build a tabtarget that:
- Uses `docker manifest inspect` to resolve each image's current tag to a @sha256: digest
- Updates RBRR_GCB_*_IMAGE_REF lines in rbrr_RecipeBottleRegimeRepo.sh
- Updates vintage comments with current date
- Prints diff: old digest vs new for each image

Run it once to populate initial pins.

## Wiring

After pins exist in RBRR:
- rbf_Foundry.sh step defs (lines 156-166): replace hardcoded image strings with RBRR variables
- rbgjb08-sbom-and-summary.sh: replace SYFT_IMAGE="anchore/syft:latest" with RBRR variable
- rbgjb04-qemu-binfmt.sh: replace tonistiigi/binfmt with RBRR variable

## Variable naming pattern

Follow existing: RBRR_GCB_{TOOL}_IMAGE_REF (e.g., RBRR_GCB_GCLOUD_IMAGE_REF)

## Files touched

- rbrr_RecipeBottleRegimeRepo.sh (new pin variables)
- rbrr_regime.sh (broach/validate the new variables)
- rbrr_cli.sh (render the new variables)
- rbf_Foundry.sh (wire variables into step defs)
- rbgjb04-qemu-binfmt.sh (wire variable)
- rbgjb08-sbom-and-summary.sh (wire variable)
- New: tt/rbw-gp.RefreshGcbPins.sh + workbench routing

**[260212-1249] rough**

Pin all GCB step builder images to digest refs in RBRR, and build a refresh tabtarget to maintain them.

## Images to pin (add to existing RBRR_GCB_*_IMAGE_REF group, lines 49-57)

Six unpinned images, all used in GCB build steps:
1. gcr.io/cloud-builders/gcloud — steps 01, 02 (tag derivation, auth token)
2. gcr.io/cloud-builders/docker — steps 03, 04, 06, 08, 09 (docker/buildx ops)
3. quay.io/skopeo/stable — step 07 (OCI push to GAR)
4. alpine — step 10 (metadata JSON assembly)
5. anchore/syft:latest — step 08, hardcoded in rbgjb08-sbom-and-summary.sh:12
6. tonistiigi/binfmt — step 04, hardcoded in rbgjb04-qemu-binfmt.sh:8

Two already pinned (gcrane, oras) — refresh command should also cover these.

## Refresh tabtarget: tt/rbw-gp.RefreshGcbPins.sh

Build a tabtarget that:
- Uses `docker manifest inspect` to resolve each image's current tag to a @sha256: digest
- Updates RBRR_GCB_*_IMAGE_REF lines in rbrr_RecipeBottleRegimeRepo.sh
- Updates vintage comments with current date
- Prints diff: old digest vs new for each image

Run it once to populate initial pins.

## Wiring

After pins exist in RBRR:
- rbf_Foundry.sh step defs (lines 156-166): replace hardcoded image strings with RBRR variables
- rbgjb08-sbom-and-summary.sh: replace SYFT_IMAGE="anchore/syft:latest" with RBRR variable
- rbgjb04-qemu-binfmt.sh: replace tonistiigi/binfmt with RBRR variable

## Variable naming pattern

Follow existing: RBRR_GCB_{TOOL}_IMAGE_REF (e.g., RBRR_GCB_GCLOUD_IMAGE_REF)

## Files touched

- rbrr_RecipeBottleRegimeRepo.sh (new pin variables)
- rbrr_regime.sh (broach/validate the new variables)
- rbrr_cli.sh (render the new variables)
- rbf_Foundry.sh (wire variables into step defs)
- rbgjb04-qemu-binfmt.sh (wire variable)
- rbgjb08-sbom-and-summary.sh (wire variable)
- New: tt/rbw-gp.RefreshGcbPins.sh + workbench routing

**[260128-0652] rough**

Pin Cloud Build tool image versions to avoid "latest" drift. Currently hardcoded: (1) anchore/syft:latest in rbgjb08-sbom-and-summary.sh, (2) alpine (implicit latest) in rbf_Foundry.sh step definitions. Convert to locked versions and consider making them configurable via RBRR or RBRV regime variables.

### fix-buc-link-ansi-escapes (₢APAAN) [complete]

**[260212-0637] complete**

Fix buc_link ANSI escape sequence rendering.

## Problem
The `buc_link` function outputs raw escape codes instead of interpreted ANSI:
```
Click to  \033[34m\033[4mOpen build in Cloud Console\033[0m
```

## Location
- Called from: `Tools/rbw/rbf_Foundry.sh:473`
- Implementation: likely in `Tools/buk/buc_command.sh`

## Fix
Ensure escape sequences are interpreted - either:
- Use `echo -e` instead of `echo`
- Use `printf` with proper formatting
- Use `$'...'` quoting for literal escapes

**[260128-2035] rough**

Fix buc_link ANSI escape sequence rendering.

## Problem
The `buc_link` function outputs raw escape codes instead of interpreted ANSI:
```
Click to  \033[34m\033[4mOpen build in Cloud Console\033[0m
```

## Location
- Called from: `Tools/rbw/rbf_Foundry.sh:473`
- Implementation: likely in `Tools/buk/buc_command.sh`

## Fix
Ensure escape sequences are interpreted - either:
- Use `echo -e` instead of `echo`
- Use `printf` with proper formatting
- Use `$'...'` quoting for literal escapes

### refine-gcb-quota-procedure (₢APAAe) [complete]

**[260213-0942] complete**

## Context

Prior work on this pace diagnosed GCB concurrent build serialization. The root cause was the Concurrent Build CPUs quota (10) vs machine type (E2_HIGHCPU_8 = 8 vCPU), not the build count quota. The machine type was changed to UNSPECIFIED (2 vCPU, 5 concurrent) in rbrr_RecipeBottleRegimeRepo.sh. A new RBSQB spec, rbgm_quota_build function, and rbw-QB tabtarget were created.

## Remaining Work

The current RBSQB spec and rbgm_quota_build function mix two concerns that should be cleanly separated:

1. **Human Console procedure** (RBSQB scope): How to navigate to IAM & Admin > Quotas & System Limits, verify CPU quota headroom, and optionally request a quota increase. This is the manual procedure analogous to RBSPE.

2. **Regime configuration** (not RBSQB scope): The RBRR_GCB_MACHINE_TYPE setting. This is a regime variable, not a Console procedure. The spec should reference it for context but not prescribe changing it.

## Steps

1. Rerun a cloud build to verify UNSPECIFIED machine type works correctly (reminder: do this first before any spec edits)
2. Revise RBSQB-quota_build.adoc to focus purely on the Console verification/quota-increase procedure
3. Revise rbgm_quota_build() function to match — show quota check and optional increase, reference machine type for context only
4. Update RBS0-SpecTop.adoc cross-references if needed

## Key Files
- `lenses/RBSQB-quota_build.adoc` — procedure spec (primary edit target)
- `Tools/rbw/rbgm_ManualProcedures.sh` — rbgm_quota_build function
- `lenses/RBS0-SpecTop.adoc` — Concurrent Session Safety NOTE, operation section
- `rbrr_RecipeBottleRegimeRepo.sh` — regime config (already changed, verify only)

**[260212-1418] rough**

## Context

Prior work on this pace diagnosed GCB concurrent build serialization. The root cause was the Concurrent Build CPUs quota (10) vs machine type (E2_HIGHCPU_8 = 8 vCPU), not the build count quota. The machine type was changed to UNSPECIFIED (2 vCPU, 5 concurrent) in rbrr_RecipeBottleRegimeRepo.sh. A new RBSQB spec, rbgm_quota_build function, and rbw-QB tabtarget were created.

## Remaining Work

The current RBSQB spec and rbgm_quota_build function mix two concerns that should be cleanly separated:

1. **Human Console procedure** (RBSQB scope): How to navigate to IAM & Admin > Quotas & System Limits, verify CPU quota headroom, and optionally request a quota increase. This is the manual procedure analogous to RBSPE.

2. **Regime configuration** (not RBSQB scope): The RBRR_GCB_MACHINE_TYPE setting. This is a regime variable, not a Console procedure. The spec should reference it for context but not prescribe changing it.

## Steps

1. Rerun a cloud build to verify UNSPECIFIED machine type works correctly (reminder: do this first before any spec edits)
2. Revise RBSQB-quota_build.adoc to focus purely on the Console verification/quota-increase procedure
3. Revise rbgm_quota_build() function to match — show quota check and optional increase, reference machine type for context only
4. Update RBS0-SpecTop.adoc cross-references if needed

## Key Files
- `lenses/RBSQB-quota_build.adoc` — procedure spec (primary edit target)
- `Tools/rbw/rbgm_ManualProcedures.sh` — rbgm_quota_build function
- `lenses/RBS0-SpecTop.adoc` — Concurrent Session Safety NOTE, operation section
- `rbrr_RecipeBottleRegimeRepo.sh` — regime config (already changed, verify only)

**[260212-1418] rough**

## Context

Prior work on this pace diagnosed GCB concurrent build serialization. The root cause was the Concurrent Build CPUs quota (10) vs machine type (E2_HIGHCPU_8 = 8 vCPU), not the build count quota. The machine type was changed to UNSPECIFIED (2 vCPU, 5 concurrent) in rbrr_RecipeBottleRegimeRepo.sh. A new RBSQB spec, rbgm_quota_build function, and rbw-QB tabtarget were created.

## Remaining Work

The current RBSQB spec and rbgm_quota_build function mix two concerns that should be cleanly separated:

1. **Human Console procedure** (RBSQB scope): How to navigate to IAM & Admin > Quotas & System Limits, verify CPU quota headroom, and optionally request a quota increase. This is the manual procedure analogous to RBSPE.

2. **Regime configuration** (not RBSQB scope): The RBRR_GCB_MACHINE_TYPE setting. This is a regime variable, not a Console procedure. The spec should reference it for context but not prescribe changing it.

## Steps

1. Rerun a cloud build to verify UNSPECIFIED machine type works correctly (reminder: do this first before any spec edits)
2. Revise RBSQB-quota_build.adoc to focus purely on the Console verification/quota-increase procedure
3. Revise rbgm_quota_build() function to match — show quota check and optional increase, reference machine type for context only
4. Update RBS0-SpecTop.adoc cross-references if needed

## Key Files
- `lenses/RBSQB-quota_build.adoc` — procedure spec (primary edit target)
- `Tools/rbw/rbgm_ManualProcedures.sh` — rbgm_quota_build function
- `lenses/RBS0-SpecTop.adoc` — Concurrent Session Safety NOTE, operation section
- `rbrr_RecipeBottleRegimeRepo.sh` — regime config (already changed, verify only)

**[260209-1709] rough**

## Problem Discovery

When dispatching two Cloud Build jobs from separate terminal sessions against the same GCP depot project, the second build remains in QUEUED status while the first is WORKING. Only after the first completes (SUCCESS) does the second transition from QUEUED to WORKING. The serialization occurs in the cloud, not at the CLI level.

## Root Cause

Google Cloud Build's **default pool** enforces a concurrent build limit of **1 build per project per region**. The Recipe Bottle system uses the default pool (no private worker pool configured) with `E2_HIGHCPU_8` machine type, as set in `rbrr_RecipeBottleRegimeRepo.sh`:
```
RBRR_GCB_MACHINE_TYPE=E2_HIGHCPU_8
```

No `workerPool` field is present in the build submission JSON constructed in `rbf_Foundry.sh` (around line 438).

## Architectural Note

The RBSA spec's "Concurrent Session Safety" section (RBS0-SpecTop.adoc, lines 563-593) explicitly documents that concurrent builds are supported and produce independent images. The CLI-level isolation is correct — unique timestamps, isolated temp dirs, no collision. The bottleneck is purely the GCP quota on the default Cloud Build pool.

## Recommended Repair Options (in order of preference)

1. **Request GCP quota increase** — Increase the concurrent build limit for the default pool via GCP Console (IAM & Admin > Quotas & System Limits, filter for `cloudbuild.googleapis.com`). Zero code changes. Google typically grants small increases (5-10) without friction. This is the right first move.

2. **Private worker pool** — Create a Cloud Build private pool with explicit concurrency configuration. Requires:
   - New regime variable for pool name/config
   - Adding `workerPool` field to the build request JSON in `rbf_Foundry.sh`
   - Pool creation/teardown procedures (likely new RBSA operations)
   - More infrastructure to manage, but gives direct concurrency control

3. **Document the limitation** — At minimum, add a NOTE to the Concurrent Session Safety section in RBS0-SpecTop.adoc clarifying that while the CLI supports concurrent invocation, the default Cloud Build pool quota may serialize actual execution.

## Key Files
- `rbrr_RecipeBottleRegimeRepo.sh` — regime config with `RBRR_GCB_MACHINE_TYPE`
- `Tools/rbw/rbf_Foundry.sh` — build submission (line ~438, no workerPool field)
- `lenses/RBS0-SpecTop.adoc` — Concurrent Session Safety section (lines 563-593)
- `lenses/RBSTB-trigger_build.adoc` — trigger_build spec (step 9: submit, step 10: poll)

### ark-lifecycle-container-smoke (₢APAAi) [abandoned]

**[260213-0953] abandoned**

Add a container smoke test step to the ark-lifecycle test suite.

## Context

The ark-lifecycle test (rbtcal_ArkLifecycle.sh) exercises image build/push/pull
through the foundry but never starts a container from the resulting image. This
leaves a gap: we prove the image exists in the registry but not that it runs.

## Work

1. After the existing ark-lifecycle image retrieval step, add a new test step
   that starts a container from the retrieved image using the bottle
   infrastructure (rbob_start or equivalent).
2. Interact with the running container at least once — e.g., exec a command,
   check a health endpoint, or verify a known file exists inside the container.
3. Stop and clean up the container (rbob_stop or equivalent).
4. Use the existing rbtb_exec_bottle / rbtb_exec_sentry helpers from
   rbtb_testbench.sh if applicable, or the BUT framework test primitives.

## Constraints

- The test vessel used is trbim-macos (test image for macOS).
- Must work within the existing BUT framework case registration pattern.
- Container start/stop should be idempotent (clean up even on failure).

## Acceptance

- ark-lifecycle suite includes a case that starts a container from the built image
- At least one interaction proves the container is functional
- Container is cleaned up after the test

**[260212-0932] rough**

Add a container smoke test step to the ark-lifecycle test suite.

## Context

The ark-lifecycle test (rbtcal_ArkLifecycle.sh) exercises image build/push/pull
through the foundry but never starts a container from the resulting image. This
leaves a gap: we prove the image exists in the registry but not that it runs.

## Work

1. After the existing ark-lifecycle image retrieval step, add a new test step
   that starts a container from the retrieved image using the bottle
   infrastructure (rbob_start or equivalent).
2. Interact with the running container at least once — e.g., exec a command,
   check a health endpoint, or verify a known file exists inside the container.
3. Stop and clean up the container (rbob_stop or equivalent).
4. Use the existing rbtb_exec_bottle / rbtb_exec_sentry helpers from
   rbtb_testbench.sh if applicable, or the BUT framework test primitives.

## Constraints

- The test vessel used is trbim-macos (test image for macOS).
- Must work within the existing BUT framework case registration pattern.
- Container start/stop should be idempotent (clean up even on failure).

## Acceptance

- ark-lifecycle suite includes a case that starts a container from the built image
- At least one interaction proves the container is functional
- Container is cleaned up after the test

### retire-makefile-workbench (₢APAAj) [complete]

**[260216-0754] complete**

Retire the makefile workbench infrastructure and clean up rbk_Coordinator remnants.

## Context

`rbk_Coordinator.sh` was already deleted in prior work. The `rbk` prefix is now clear for kit use. However, `rbw.workbench.mk` and its entire MBD dispatch system still exist, serving 17 old tabtargets. Podman VM functionality is mothballed, LMCI is out of scope, and all other functionality has modern BUK equivalents. The whole makefile stack can go.

## Deletions (24 files)

### Makefile infrastructure (5 files)
- `Tools/rbw/rbw.workbench.mk`
- `Tools/mbc.console.mk`
- `Tools/mbd.dispatch.sh`
- `Tools/mbd.utils.sh`
- `mbv.variables.sh`

### LMCI tools (2 files)
- `Tools/lmci/bundle.sh`
- `Tools/lmci/strip.sh`

### Old tabtargets (17 files)
- `tt/rbw-N.NUKE_PODMAN_MACHINE.sh` (Podman VM mothballed)
- `tt/rbw-c.CheckLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-m.MirrorLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-f.FetchChosenPodmanVMImage.sh` (Podman VM mothballed)
- `tt/rbw-i.InitChosenPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-vc.PodmanVmCheckLatest.sh` (duplicate of rbw-c)
- `tt/rbw-hv.HelpPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-hw.HelpResetWSL.sh` (WSL/Windows dead)
- `tt/rbw-D.DigestPodmanHtml.sh` (Cygwin one-off dead)
- `tt/rbw-II.ImageInfoGHCR.sh` (orphan — no makefile rule)
- `tt/rbw-b.BuildWithRecipe.sh` (orphan — no makefile rule)
- `tt/rbw-d.DeleteImage.sh` (orphan — no makefile rule)
- `tt/rbw-hi.HelpImageManagement.sh` (superseded by rbw-him)
- `tt/rbw-s.Start.nsdemo.sh` (old moniker, modern equivalents exist)
- `tt/ttc.CreateTabtarget.sh` (superseded by BUK buw-tt-c*)
- `tt/ttx.FixTabtargetExecutability.sh` (superseded by BUK)
- `tt/lmci-b.BundleFilesForLLM.sh` (LMCI out of scope)
- `tt/lmci-s.StripWhitespaceFiles.sh` (LMCI out of scope)

## Live code fix (1 file)
- `Tools/ccck/cccw_workbench.sh:115` — replace `rbk_show` with `echo`

## Doc reference (assess)
- `lenses/CRR-ConfigRegimeRequirements.adoc:427` — example showing `rbw.workbench.mk` in variable syntax demo. Update or annotate as historical.

## Verification
- No files at deleted paths
- `grep -r mbd.dispatch` returns only gallops/heat docs
- `grep -r rbw.workbench.mk` returns only gallops/heat docs/CRR (if kept as historical)
- `grep -r rbk_` returns only gallops/heat docs
- `cccw_workbench.sh` has no `rbk_show` reference

**[260216-0750] rough**

Retire the makefile workbench infrastructure and clean up rbk_Coordinator remnants.

## Context

`rbk_Coordinator.sh` was already deleted in prior work. The `rbk` prefix is now clear for kit use. However, `rbw.workbench.mk` and its entire MBD dispatch system still exist, serving 17 old tabtargets. Podman VM functionality is mothballed, LMCI is out of scope, and all other functionality has modern BUK equivalents. The whole makefile stack can go.

## Deletions (24 files)

### Makefile infrastructure (5 files)
- `Tools/rbw/rbw.workbench.mk`
- `Tools/mbc.console.mk`
- `Tools/mbd.dispatch.sh`
- `Tools/mbd.utils.sh`
- `mbv.variables.sh`

### LMCI tools (2 files)
- `Tools/lmci/bundle.sh`
- `Tools/lmci/strip.sh`

### Old tabtargets (17 files)
- `tt/rbw-N.NUKE_PODMAN_MACHINE.sh` (Podman VM mothballed)
- `tt/rbw-c.CheckLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-m.MirrorLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-f.FetchChosenPodmanVMImage.sh` (Podman VM mothballed)
- `tt/rbw-i.InitChosenPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-vc.PodmanVmCheckLatest.sh` (duplicate of rbw-c)
- `tt/rbw-hv.HelpPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-hw.HelpResetWSL.sh` (WSL/Windows dead)
- `tt/rbw-D.DigestPodmanHtml.sh` (Cygwin one-off dead)
- `tt/rbw-II.ImageInfoGHCR.sh` (orphan — no makefile rule)
- `tt/rbw-b.BuildWithRecipe.sh` (orphan — no makefile rule)
- `tt/rbw-d.DeleteImage.sh` (orphan — no makefile rule)
- `tt/rbw-hi.HelpImageManagement.sh` (superseded by rbw-him)
- `tt/rbw-s.Start.nsdemo.sh` (old moniker, modern equivalents exist)
- `tt/ttc.CreateTabtarget.sh` (superseded by BUK buw-tt-c*)
- `tt/ttx.FixTabtargetExecutability.sh` (superseded by BUK)
- `tt/lmci-b.BundleFilesForLLM.sh` (LMCI out of scope)
- `tt/lmci-s.StripWhitespaceFiles.sh` (LMCI out of scope)

## Live code fix (1 file)
- `Tools/ccck/cccw_workbench.sh:115` — replace `rbk_show` with `echo`

## Doc reference (assess)
- `lenses/CRR-ConfigRegimeRequirements.adoc:427` — example showing `rbw.workbench.mk` in variable syntax demo. Update or annotate as historical.

## Verification
- No files at deleted paths
- `grep -r mbd.dispatch` returns only gallops/heat docs
- `grep -r rbw.workbench.mk` returns only gallops/heat docs/CRR (if kept as historical)
- `grep -r rbk_` returns only gallops/heat docs
- `cccw_workbench.sh` has no `rbk_show` reference

**[260216-0750] rough**

Retire the makefile workbench infrastructure and clean up rbk_Coordinator remnants.

## Context

`rbk_Coordinator.sh` was already deleted in prior work. The `rbk` prefix is now clear for kit use. However, `rbw.workbench.mk` and its entire MBD dispatch system still exist, serving 17 old tabtargets. Podman VM functionality is mothballed, LMCI is out of scope, and all other functionality has modern BUK equivalents. The whole makefile stack can go.

## Deletions (24 files)

### Makefile infrastructure (5 files)
- `Tools/rbw/rbw.workbench.mk`
- `Tools/mbc.console.mk`
- `Tools/mbd.dispatch.sh`
- `Tools/mbd.utils.sh`
- `mbv.variables.sh`

### LMCI tools (2 files)
- `Tools/lmci/bundle.sh`
- `Tools/lmci/strip.sh`

### Old tabtargets (17 files)
- `tt/rbw-N.NUKE_PODMAN_MACHINE.sh` (Podman VM mothballed)
- `tt/rbw-c.CheckLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-m.MirrorLatestPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-f.FetchChosenPodmanVMImage.sh` (Podman VM mothballed)
- `tt/rbw-i.InitChosenPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-vc.PodmanVmCheckLatest.sh` (duplicate of rbw-c)
- `tt/rbw-hv.HelpPodmanVM.sh` (Podman VM mothballed)
- `tt/rbw-hw.HelpResetWSL.sh` (WSL/Windows dead)
- `tt/rbw-D.DigestPodmanHtml.sh` (Cygwin one-off dead)
- `tt/rbw-II.ImageInfoGHCR.sh` (orphan — no makefile rule)
- `tt/rbw-b.BuildWithRecipe.sh` (orphan — no makefile rule)
- `tt/rbw-d.DeleteImage.sh` (orphan — no makefile rule)
- `tt/rbw-hi.HelpImageManagement.sh` (superseded by rbw-him)
- `tt/rbw-s.Start.nsdemo.sh` (old moniker, modern equivalents exist)
- `tt/ttc.CreateTabtarget.sh` (superseded by BUK buw-tt-c*)
- `tt/ttx.FixTabtargetExecutability.sh` (superseded by BUK)
- `tt/lmci-b.BundleFilesForLLM.sh` (LMCI out of scope)
- `tt/lmci-s.StripWhitespaceFiles.sh` (LMCI out of scope)

## Live code fix (1 file)
- `Tools/ccck/cccw_workbench.sh:115` — replace `rbk_show` with `echo`

## Doc reference (assess)
- `lenses/CRR-ConfigRegimeRequirements.adoc:427` — example showing `rbw.workbench.mk` in variable syntax demo. Update or annotate as historical.

## Verification
- No files at deleted paths
- `grep -r mbd.dispatch` returns only gallops/heat docs
- `grep -r rbw.workbench.mk` returns only gallops/heat docs/CRR (if kept as historical)
- `grep -r rbk_` returns only gallops/heat docs
- `cccw_workbench.sh` has no `rbk_show` reference

**[260213-0809] rough**

Drafted from ₢AUAAC in ₣AU.

Delete `Tools/rbw/rbk_Coordinator.sh` by absorbing any still-useful content into `rbw_workbench.sh`.

## Rationale

The `rbk` prefix must be freed for use as a kit directory (`Tools/rbk/`). Terminal exclusivity forbids `rbk` naming both a file prefix and a kit directory.

Also confirm `rbw.workbench.mk` is gone and clean up any dangling references.

## Tasks
1. Read `rbk_Coordinator.sh` and `rbw_workbench.sh` — identify what in Coordinator is still useful
2. Move useful content into `rbw_workbench.sh`
3. Delete `rbk_Coordinator.sh`
4. Grep for any `rbk_` references (imports, calls) and update to `rbw_` equivalents
5. Confirm `rbw.workbench.mk` is absent; grep for references and remove any
6. Update CLAUDE.md acronym mapping (remove RBK → rbk_Coordinator.sh entry)

## Verification
- No file at `Tools/rbw/rbk_Coordinator.sh`
- No `rbk_` references in codebase (except future `Tools/rbk/` kit)
- No `rbw.workbench.mk` references
- `rbw_workbench.sh` has any salvaged functionality

**[260209-0552] rough**

Delete `Tools/rbw/rbk_Coordinator.sh` by absorbing any still-useful content into `rbw_workbench.sh`.

## Rationale

The `rbk` prefix must be freed for use as a kit directory (`Tools/rbk/`). Terminal exclusivity forbids `rbk` naming both a file prefix and a kit directory.

Also confirm `rbw.workbench.mk` is gone and clean up any dangling references.

## Tasks
1. Read `rbk_Coordinator.sh` and `rbw_workbench.sh` — identify what in Coordinator is still useful
2. Move useful content into `rbw_workbench.sh`
3. Delete `rbk_Coordinator.sh`
4. Grep for any `rbk_` references (imports, calls) and update to `rbw_` equivalents
5. Confirm `rbw.workbench.mk` is absent; grep for references and remove any
6. Update CLAUDE.md acronym mapping (remove RBK → rbk_Coordinator.sh entry)

## Verification
- No file at `Tools/rbw/rbk_Coordinator.sh`
- No `rbk_` references in codebase (except future `Tools/rbk/` kit)
- No `rbw.workbench.mk` references
- `rbw_workbench.sh` has any salvaged functionality

### implement-qualify-all-system (₢APAAk) [complete]

**[260216-0841] complete**

## Context

Implement a multi-layer qualification system that validates tabtarget health,
colophon registry consistency, and nameplate resource conflicts. Runs as a
hard gate before cloud builds, service starts, and test suites.

## Architecture (Decided)

```
rbw-qa.QualifyAll.sh  (tabtarget, orchestrator)
  └─ rbq_cli.sh  →  rbq_qualify_all()
       ├─ buv_qualify_tabtargets()    [BUK — structural checks]
       ├─ rbq_qualify_colophons()     [RBW — zipper cross-check]
       └─ rbrn_preflight()            [RBW — nameplate conflicts, Option B]
```

**Layering principle**: BUK owns structural tabtarget validation (shebang,
BURD_LAUNCHER export, exec pattern, launcher existence). RBW's rbq orchestrates
all checks without owning domain-specific logic. `rbrn_preflight()` stays in
`rbrn_regime.sh` (Option B — rbq calls it, doesn't own it).

## Design Decisions Made

- **Prefix**: `rbq` ("qualify") — terminal, no conflicts
- **Home**: RBW space (must import rbz zipper; BUK can't depend on RBW)
- **Tabtarget colophon**: `rbw-qa` with frontispiece `QualifyAll`
- **BUK validation**: New `buv_qualify_tabtargets()` in `buv_validation.sh`
  owns structural checks (BUK conventions validated by BUK)
- **Nameplate preflight**: Option B — rbq calls `rbrn_preflight()`, doesn't
  move or duplicate it
- **Gating**: Die always on any failure (hard stop)
- **No aliases**: Reject duplicate colophon→command mappings

## BUK Layer: buv_qualify_tabtargets()

Structural checks BUK owns:
- Valid shebang line
- `BURD_LAUNCHER` export present and references existing file
- exec pattern follows BUK convention
- Colophon extracted from filename matches at least one `buz_register` entry

## RBW Layer: rbq_qualify_colophons()

RBW-specific checks:
- Colophon cross-check against `rbz_zipper.sh` registrations
- Module file existence for each registered colophon
- No duplicate colophon→command mappings in workbench case tables

## Gates (die on failure)

| Gate Location | When |
|---------------|------|
| `rbw-iB.BuildImageRemotely.sh` | Before cloud build |
| `rbw-s.Start.*.sh` | Before service start |
| Testbench (test case) | Before test-all runs |

Gate mechanism: call rbq validation function early; `buc_die` on failure.
Testbench gate: add test case that invokes the qualify CLI and asserts exit 0.

## Steps

1. Create `buv_qualify_tabtargets()` in `Tools/buk/buv_validation.sh`
   - Structural checks for all tabtargets in tt/
   - Colophon extraction and buz registry lookup
2. Create `rbq_Qualify.sh` module with kindle/sentinel pattern
   - `rbq_qualify_all()` orchestrator calling buv + colophon + preflight
3. Create `rbq_cli.sh` with furnish sourcing both zippers
4. Register `rbw-qa` colophon in `rbz_zipper.sh`
5. Create `rbw-qa.QualifyAll.sh` tabtarget
6. Add gates to `rbw-iB`, `rbw-s.Start.*` scripts
7. Add testbench test case for qualify
8. Update CLAUDE.md acronym mappings (RBQ)

## Key Files
- `Tools/buk/buv_validation.sh` — BUK validation (add buv_qualify_tabtargets)
- `Tools/rbw/rbq_Qualify.sh` — NEW: RBW qualify orchestrator
- `Tools/rbw/rbq_cli.sh` — NEW: RBW qualify CLI
- `Tools/rbw/rbz_zipper.sh` — RBW colophon registry (add rbw-qa)
- `Tools/rbw/rbw_workbench.sh` — RBW workbench routing
- `Tools/rbw/rbrn_regime.sh` — contains rbrn_preflight() (called by rbq)
- `Tools/rbw/rbrn_cli.sh` — contains rbrn_audit() (reference pattern)
- `Tools/buk/buz_zipper.sh` — BUK colophon registry
- `tt/*.sh` — all tabtargets to validate + new rbw-qa tabtarget

**[260216-0816] rough**

## Context

Implement a multi-layer qualification system that validates tabtarget health,
colophon registry consistency, and nameplate resource conflicts. Runs as a
hard gate before cloud builds, service starts, and test suites.

## Architecture (Decided)

```
rbw-qa.QualifyAll.sh  (tabtarget, orchestrator)
  └─ rbq_cli.sh  →  rbq_qualify_all()
       ├─ buv_qualify_tabtargets()    [BUK — structural checks]
       ├─ rbq_qualify_colophons()     [RBW — zipper cross-check]
       └─ rbrn_preflight()            [RBW — nameplate conflicts, Option B]
```

**Layering principle**: BUK owns structural tabtarget validation (shebang,
BURD_LAUNCHER export, exec pattern, launcher existence). RBW's rbq orchestrates
all checks without owning domain-specific logic. `rbrn_preflight()` stays in
`rbrn_regime.sh` (Option B — rbq calls it, doesn't own it).

## Design Decisions Made

- **Prefix**: `rbq` ("qualify") — terminal, no conflicts
- **Home**: RBW space (must import rbz zipper; BUK can't depend on RBW)
- **Tabtarget colophon**: `rbw-qa` with frontispiece `QualifyAll`
- **BUK validation**: New `buv_qualify_tabtargets()` in `buv_validation.sh`
  owns structural checks (BUK conventions validated by BUK)
- **Nameplate preflight**: Option B — rbq calls `rbrn_preflight()`, doesn't
  move or duplicate it
- **Gating**: Die always on any failure (hard stop)
- **No aliases**: Reject duplicate colophon→command mappings

## BUK Layer: buv_qualify_tabtargets()

Structural checks BUK owns:
- Valid shebang line
- `BURD_LAUNCHER` export present and references existing file
- exec pattern follows BUK convention
- Colophon extracted from filename matches at least one `buz_register` entry

## RBW Layer: rbq_qualify_colophons()

RBW-specific checks:
- Colophon cross-check against `rbz_zipper.sh` registrations
- Module file existence for each registered colophon
- No duplicate colophon→command mappings in workbench case tables

## Gates (die on failure)

| Gate Location | When |
|---------------|------|
| `rbw-iB.BuildImageRemotely.sh` | Before cloud build |
| `rbw-s.Start.*.sh` | Before service start |
| Testbench (test case) | Before test-all runs |

Gate mechanism: call rbq validation function early; `buc_die` on failure.
Testbench gate: add test case that invokes the qualify CLI and asserts exit 0.

## Steps

1. Create `buv_qualify_tabtargets()` in `Tools/buk/buv_validation.sh`
   - Structural checks for all tabtargets in tt/
   - Colophon extraction and buz registry lookup
2. Create `rbq_Qualify.sh` module with kindle/sentinel pattern
   - `rbq_qualify_all()` orchestrator calling buv + colophon + preflight
3. Create `rbq_cli.sh` with furnish sourcing both zippers
4. Register `rbw-qa` colophon in `rbz_zipper.sh`
5. Create `rbw-qa.QualifyAll.sh` tabtarget
6. Add gates to `rbw-iB`, `rbw-s.Start.*` scripts
7. Add testbench test case for qualify
8. Update CLAUDE.md acronym mappings (RBQ)

## Key Files
- `Tools/buk/buv_validation.sh` — BUK validation (add buv_qualify_tabtargets)
- `Tools/rbw/rbq_Qualify.sh` — NEW: RBW qualify orchestrator
- `Tools/rbw/rbq_cli.sh` — NEW: RBW qualify CLI
- `Tools/rbw/rbz_zipper.sh` — RBW colophon registry (add rbw-qa)
- `Tools/rbw/rbw_workbench.sh` — RBW workbench routing
- `Tools/rbw/rbrn_regime.sh` — contains rbrn_preflight() (called by rbq)
- `Tools/rbw/rbrn_cli.sh` — contains rbrn_audit() (reference pattern)
- `Tools/buk/buz_zipper.sh` — BUK colophon registry
- `tt/*.sh` — all tabtargets to validate + new rbw-qa tabtarget

**[260216-0816] rough**

## Context

Implement a multi-layer qualification system that validates tabtarget health,
colophon registry consistency, and nameplate resource conflicts. Runs as a
hard gate before cloud builds, service starts, and test suites.

## Architecture (Decided)

```
rbw-qa.QualifyAll.sh  (tabtarget, orchestrator)
  └─ rbq_cli.sh  →  rbq_qualify_all()
       ├─ buv_qualify_tabtargets()    [BUK — structural checks]
       ├─ rbq_qualify_colophons()     [RBW — zipper cross-check]
       └─ rbrn_preflight()            [RBW — nameplate conflicts, Option B]
```

**Layering principle**: BUK owns structural tabtarget validation (shebang,
BURD_LAUNCHER export, exec pattern, launcher existence). RBW's rbq orchestrates
all checks without owning domain-specific logic. `rbrn_preflight()` stays in
`rbrn_regime.sh` (Option B — rbq calls it, doesn't own it).

## Design Decisions Made

- **Prefix**: `rbq` ("qualify") — terminal, no conflicts
- **Home**: RBW space (must import rbz zipper; BUK can't depend on RBW)
- **Tabtarget colophon**: `rbw-qa` with frontispiece `QualifyAll`
- **BUK validation**: New `buv_qualify_tabtargets()` in `buv_validation.sh`
  owns structural checks (BUK conventions validated by BUK)
- **Nameplate preflight**: Option B — rbq calls `rbrn_preflight()`, doesn't
  move or duplicate it
- **Gating**: Die always on any failure (hard stop)
- **No aliases**: Reject duplicate colophon→command mappings

## BUK Layer: buv_qualify_tabtargets()

Structural checks BUK owns:
- Valid shebang line
- `BURD_LAUNCHER` export present and references existing file
- exec pattern follows BUK convention
- Colophon extracted from filename matches at least one `buz_register` entry

## RBW Layer: rbq_qualify_colophons()

RBW-specific checks:
- Colophon cross-check against `rbz_zipper.sh` registrations
- Module file existence for each registered colophon
- No duplicate colophon→command mappings in workbench case tables

## Gates (die on failure)

| Gate Location | When |
|---------------|------|
| `rbw-iB.BuildImageRemotely.sh` | Before cloud build |
| `rbw-s.Start.*.sh` | Before service start |
| Testbench (test case) | Before test-all runs |

Gate mechanism: call rbq validation function early; `buc_die` on failure.
Testbench gate: add test case that invokes the qualify CLI and asserts exit 0.

## Steps

1. Create `buv_qualify_tabtargets()` in `Tools/buk/buv_validation.sh`
   - Structural checks for all tabtargets in tt/
   - Colophon extraction and buz registry lookup
2. Create `rbq_Qualify.sh` module with kindle/sentinel pattern
   - `rbq_qualify_all()` orchestrator calling buv + colophon + preflight
3. Create `rbq_cli.sh` with furnish sourcing both zippers
4. Register `rbw-qa` colophon in `rbz_zipper.sh`
5. Create `rbw-qa.QualifyAll.sh` tabtarget
6. Add gates to `rbw-iB`, `rbw-s.Start.*` scripts
7. Add testbench test case for qualify
8. Update CLAUDE.md acronym mappings (RBQ)

## Key Files
- `Tools/buk/buv_validation.sh` — BUK validation (add buv_qualify_tabtargets)
- `Tools/rbw/rbq_Qualify.sh` — NEW: RBW qualify orchestrator
- `Tools/rbw/rbq_cli.sh` — NEW: RBW qualify CLI
- `Tools/rbw/rbz_zipper.sh` — RBW colophon registry (add rbw-qa)
- `Tools/rbw/rbw_workbench.sh` — RBW workbench routing
- `Tools/rbw/rbrn_regime.sh` — contains rbrn_preflight() (called by rbq)
- `Tools/rbw/rbrn_cli.sh` — contains rbrn_audit() (reference pattern)
- `Tools/buk/buz_zipper.sh` — BUK colophon registry
- `tt/*.sh` — all tabtargets to validate + new rbw-qa tabtarget

**[260213-1003] rough**

## Context

The project needs a systematic validation module (`rbq_Qualify.sh`) that checks tabtarget health and colophon registry consistency. This was designed in conversation alongside the GCB quota check work.

## Design Decisions Made

- **Prefix**: `rbq` ("qualify") — terminal, no conflicts
- **Home**: RBW space (must import rbz zipper; BUK can't depend on RBW)
- **Zipper import**: Sources both `rbz` (RBW) and `buz` (BUK) to get authoritative colophon registries
- **Tabtarget structural checks**: Validate exports, launcher references, file existence (note: tabtargets may have multiple exports for interactive/nonlogging config)
- **Colophon cross-check**: Extract colophon from tt/ filename, verify against zipper tables
- **No aliases**: Reject duplicate colophon→command mappings in workbench case tables
- **Attachment point**: TBD — must design where rbq checks are invoked (standalone tabtarget? part of audit? both?)

## Nameplate Cross-Integrity Consideration

`rbrn_preflight()` in `rbrn_regime.sh` currently performs cross-nameplate
validation: port uniqueness, IP uniqueness, subnet non-overlap. It is called
from `rbrn_audit()` (dispatched by `rbw-nv`).

This is a different flavor of "qualify" — resource conflict detection across
nameplates rather than structural/registry consistency. Decide during
implementation:

- **Option A**: Move `rbrn_preflight` wholesale into `rbq` — rbq becomes the
  single home for all preflight validation
- **Option B**: Keep `rbrn_preflight` in `rbrn_regime.sh` but have `rbq` call
  it — rbq orchestrates all checks without owning nameplate-specific logic
- **Option C**: Leave them independent — `rbw-nv` stays as-is, `rbq` only
  handles tabtarget/colophon concerns

Consider: does unifying under rbq reduce confusion about "which preflight
command do I run?" or does it create unwanted coupling?

## Steps

1. Design the attachment point: standalone `rbw-Q.Qualify.sh` tabtarget, integration with existing audit, or both
2. Decide on nameplate preflight relationship (Option A/B/C above)
3. Create `rbq_Qualify.sh` module with kindle/sentinel pattern
4. Create `rbq_cli.sh` with furnish sourcing both zippers
5. Implement tabtarget structural validation (shebang, exports, launcher existence, line structure)
6. Implement colophon cross-check against zipper registrations
7. Wire tabtarget and test
8. Update CLAUDE.md acronym mappings

## Key Files
- `Tools/rbw/rbz_zipper.sh` — RBW colophon registry
- `Tools/buk/buz_zipper.sh` — BUK colophon registry
- `Tools/buk/buw_workbench.sh` — BUK workbench routing
- `Tools/rbw/rbw_workbench.sh` — RBW workbench routing
- `Tools/rbw/rbrn_regime.sh` — contains `rbrn_preflight()` (cross-nameplate checker)
- `Tools/rbw/rbrn_cli.sh` — contains `rbrn_audit()` (current dispatch for preflight)
- `tt/*.sh` — all tabtargets to validate

**[260213-0924] rough**

## Context

The project needs a systematic validation module (`rbq_Qualify.sh`) that checks tabtarget health and colophon registry consistency. This was designed in conversation alongside the GCB quota check work.

## Design Decisions Made

- **Prefix**: `rbq` ("qualify") — terminal, no conflicts
- **Home**: RBW space (must import rbz zipper; BUK can't depend on RBW)
- **Zipper import**: Sources both `rbz` (RBW) and `buz` (BUK) to get authoritative colophon registries
- **Tabtarget structural checks**: Validate exports, launcher references, file existence (note: tabtargets may have multiple exports for interactive/nonlogging config)
- **Colophon cross-check**: Extract colophon from tt/ filename, verify against zipper tables
- **No aliases**: Reject duplicate colophon→command mappings in workbench case tables
- **Attachment point**: TBD — must design where rbq checks are invoked (standalone tabtarget? part of audit? both?)

## Steps

1. Design the attachment point: standalone `rbw-Q.Qualify.sh` tabtarget, integration with existing audit, or both
2. Create `rbq_Qualify.sh` module with kindle/sentinel pattern
3. Create `rbq_cli.sh` with furnish sourcing both zippers
4. Implement tabtarget structural validation (shebang, exports, launcher existence, line structure)
5. Implement colophon cross-check against zipper registrations
6. Wire tabtarget and test
7. Update CLAUDE.md acronym mappings

## Key Files
- `Tools/rbw/rbz_zipper.sh` — RBW colophon registry
- `Tools/buk/buz_zipper.sh` — BUK colophon registry
- `Tools/buk/buw_workbench.sh` — BUK workbench routing
- `Tools/rbw/rbw_workbench.sh` — RBW workbench routing
- `tt/*.sh` — all tabtargets to validate

### zipper-registration-cleanup (₢APAAn) [complete]

**[260216-0924] complete**

## Context

The qualify-all system (₢APAAk) is built and running. Bottle colophon
registrations have been added to achieve a clean qualify pass. This pace
now completes the zipper cleanup: FM-001 enroll migration and magic-string
elimination.

## Part 1: Qualify pass (DONE)

Five bottle colophons (`rbw-s`, `rbw-S`, `rbw-C`, `rbw-B`, `rbw-o`)
registered in `rbz_zipper.sh`. Workbench explicit case arms retained for
imprint→moniker translation. `./tt/rbw-qa.QualifyAll.sh` exits 0.

## Part 2: FM-001 enroll migration

Migrate `buz_register` → `buz_enroll` per BCG FM-001:

- **Definition** (`buz_zipper.sh`): Rename function, add `printf -v` for
  caller-specified variable name as first arg. Update return channel from
  shared `z_buz_register_colophon` to `printf -v` into caller's variable.
  Rename arrays to `_roll` convention.
- **Consumer** (`rbz_zipper.sh`): Convert all 51 two-line call pairs to
  single-line `buz_enroll RBZ_CONSTANT "colophon" "module" "command"`.
- **Consumer** (`rbtb_testbench.sh`): Convert 1 call site.

## Part 3: Local z_mod deduplication

Within each group in `rbz_zipper.sh`, introduce `local z_mod="filename.sh"`
to eliminate repeated CLI filename strings. Each group already has a comment
header — the local goes right after it.

## Acceptance Criteria

- `./tt/rbw-qa.QualifyAll.sh` exits 0 (clean pass preserved)
- Zero `buz_register` calls remain (fully migrated to `buz_enroll`)
- Zero `z_buz_register_colophon` references remain
- Each CLI filename string appears at most once per group (via `local z_mod`)
- Zipper arrays use `_roll` naming convention
- All existing tests pass

## Key Files
- `Tools/buk/buz_zipper.sh` — enroll function definition (migrate)
- `Tools/rbw/rbz_zipper.sh` — primary consumer (51 call sites)
- `Tools/rbw/rbtb_testbench.sh` — secondary consumer (1 call site)
- `Tools/rbw/rbq_Qualify.sh` — qualify orchestrator (verify still passes)

## Read First
- BCG FM-001 section (fading memory)
- BCG enroll function pattern (printf -v, _roll arrays)
- buz_zipper.sh current implementation

**[260216-0908] rough**

## Context

The qualify-all system (₢APAAk) is built and running. Bottle colophon
registrations have been added to achieve a clean qualify pass. This pace
now completes the zipper cleanup: FM-001 enroll migration and magic-string
elimination.

## Part 1: Qualify pass (DONE)

Five bottle colophons (`rbw-s`, `rbw-S`, `rbw-C`, `rbw-B`, `rbw-o`)
registered in `rbz_zipper.sh`. Workbench explicit case arms retained for
imprint→moniker translation. `./tt/rbw-qa.QualifyAll.sh` exits 0.

## Part 2: FM-001 enroll migration

Migrate `buz_register` → `buz_enroll` per BCG FM-001:

- **Definition** (`buz_zipper.sh`): Rename function, add `printf -v` for
  caller-specified variable name as first arg. Update return channel from
  shared `z_buz_register_colophon` to `printf -v` into caller's variable.
  Rename arrays to `_roll` convention.
- **Consumer** (`rbz_zipper.sh`): Convert all 51 two-line call pairs to
  single-line `buz_enroll RBZ_CONSTANT "colophon" "module" "command"`.
- **Consumer** (`rbtb_testbench.sh`): Convert 1 call site.

## Part 3: Local z_mod deduplication

Within each group in `rbz_zipper.sh`, introduce `local z_mod="filename.sh"`
to eliminate repeated CLI filename strings. Each group already has a comment
header — the local goes right after it.

## Acceptance Criteria

- `./tt/rbw-qa.QualifyAll.sh` exits 0 (clean pass preserved)
- Zero `buz_register` calls remain (fully migrated to `buz_enroll`)
- Zero `z_buz_register_colophon` references remain
- Each CLI filename string appears at most once per group (via `local z_mod`)
- Zipper arrays use `_roll` naming convention
- All existing tests pass

## Key Files
- `Tools/buk/buz_zipper.sh` — enroll function definition (migrate)
- `Tools/rbw/rbz_zipper.sh` — primary consumer (51 call sites)
- `Tools/rbw/rbtb_testbench.sh` — secondary consumer (1 call site)
- `Tools/rbw/rbq_Qualify.sh` — qualify orchestrator (verify still passes)

## Read First
- BCG FM-001 section (fading memory)
- BCG enroll function pattern (printf -v, _roll arrays)
- buz_zipper.sh current implementation

**[260216-0908] rough**

## Context

The qualify-all system (₢APAAk) is built and running. Bottle colophon
registrations have been added to achieve a clean qualify pass. This pace
now completes the zipper cleanup: FM-001 enroll migration and magic-string
elimination.

## Part 1: Qualify pass (DONE)

Five bottle colophons (`rbw-s`, `rbw-S`, `rbw-C`, `rbw-B`, `rbw-o`)
registered in `rbz_zipper.sh`. Workbench explicit case arms retained for
imprint→moniker translation. `./tt/rbw-qa.QualifyAll.sh` exits 0.

## Part 2: FM-001 enroll migration

Migrate `buz_register` → `buz_enroll` per BCG FM-001:

- **Definition** (`buz_zipper.sh`): Rename function, add `printf -v` for
  caller-specified variable name as first arg. Update return channel from
  shared `z_buz_register_colophon` to `printf -v` into caller's variable.
  Rename arrays to `_roll` convention.
- **Consumer** (`rbz_zipper.sh`): Convert all 51 two-line call pairs to
  single-line `buz_enroll RBZ_CONSTANT "colophon" "module" "command"`.
- **Consumer** (`rbtb_testbench.sh`): Convert 1 call site.

## Part 3: Local z_mod deduplication

Within each group in `rbz_zipper.sh`, introduce `local z_mod="filename.sh"`
to eliminate repeated CLI filename strings. Each group already has a comment
header — the local goes right after it.

## Acceptance Criteria

- `./tt/rbw-qa.QualifyAll.sh` exits 0 (clean pass preserved)
- Zero `buz_register` calls remain (fully migrated to `buz_enroll`)
- Zero `z_buz_register_colophon` references remain
- Each CLI filename string appears at most once per group (via `local z_mod`)
- Zipper arrays use `_roll` naming convention
- All existing tests pass

## Key Files
- `Tools/buk/buz_zipper.sh` — enroll function definition (migrate)
- `Tools/rbw/rbz_zipper.sh` — primary consumer (51 call sites)
- `Tools/rbw/rbtb_testbench.sh` — secondary consumer (1 call site)
- `Tools/rbw/rbq_Qualify.sh` — qualify orchestrator (verify still passes)

## Read First
- BCG FM-001 section (fading memory)
- BCG enroll function pattern (printf -v, _roll arrays)
- buz_zipper.sh current implementation

**[260216-0840] rough**

## Context

The qualify-all system (₢APAAk) is built and running. It correctly identifies
structural and registration issues. This pace addresses the findings to achieve
a clean pass.

## Current Failures

### Colophon registration (12 failures)

Bottle operation colophons (`rbw-s`, `rbw-z`, `rbw-S`, `rbw-C`, `rbw-B`, `rbw-o`)
are routed by explicit workbench case arms, not through zipper dispatch. They
have tabtargets using the RBW launcher but no `buz_register` entries.

The workbench routes these specially because it translates the moniker imprint
(`BURD_TOKEN_3` → `RBOB_MONIKER`) before dispatching to `rbob_cli.sh`. This
translation step doesn't fit the simple zipper `colophon→module→command` pattern.

**Options to resolve:**
- Register them in the zipper AND keep the explicit case arm for imprint translation
- Extend zipper dispatch to support imprint translation
- Another architectural approach (discuss during implementation)

The user explicitly rejected exemption/skip — these must be properly handled.

### butctt.TestTarget.sh

Currently passes structural check (accepted `exit` as dispatch pattern) but is
a test fixture, not a real dispatch target. May need architectural review of
whether test fixtures belong in `tt/`.

## Acceptance Criteria

- `./tt/rbw-qa.QualifyAll.sh` exits 0 (clean pass)
- All RBW tabtargets have registered colophons
- No exemptions or skips in the qualify logic
- Testbench qualify-all suite passes

## Key Files
- `Tools/rbw/rbq_Qualify.sh` — qualify orchestrator
- `Tools/rbw/rbz_zipper.sh` — colophon registry (needs bottle registrations)
- `Tools/rbw/rbw_workbench.sh` — workbench routing (bottle case arms)
- `Tools/buk/buv_validation.sh` — structural validation
- `Tools/buk/buz_zipper.sh` — zipper infrastructure
- `tt/butctt.TestTarget.sh` — test fixture to resolve

## Read First
- BCG (always before writing bash)
- rbw_workbench.sh bottle routing logic
- buz_zipper.sh zbuz_exec_lookup pattern

### design-gcrane-skopeo-replacement (₢APAAl) [complete]

**[260216-0951] complete**

Design how gcrane can replace skopeo in the GCB pipeline.

## Context

quay.io aggressively garbage-collects image digests (within hours), making pinned skopeo refs unreliable. gcrane (gcr.io, Google-hosted) is already a GCB dependency and performs similar OCI registry copy operations.

## Work

1. Identify the specific GCB step(s) that use skopeo — what exact skopeo subcommand and flags
2. Find the gcrane equivalent command(s)
3. Verify gcrane supports the same OCI artifact types (manifests, indexes, etc.)
4. Draft the replacement cloudbuild step configuration
5. Validate that no other skopeo-specific features are relied upon

## Acceptance

- Clear mapping from skopeo command(s) to gcrane equivalent(s)
- Draft replacement GCB step ready for implementation
- Confirmation that gcrane handles all artifact types we push

**[260213-0959] rough**

Design how gcrane can replace skopeo in the GCB pipeline.

## Context

quay.io aggressively garbage-collects image digests (within hours), making pinned skopeo refs unreliable. gcrane (gcr.io, Google-hosted) is already a GCB dependency and performs similar OCI registry copy operations.

## Work

1. Identify the specific GCB step(s) that use skopeo — what exact skopeo subcommand and flags
2. Find the gcrane equivalent command(s)
3. Verify gcrane supports the same OCI artifact types (manifests, indexes, etc.)
4. Draft the replacement cloudbuild step configuration
5. Validate that no other skopeo-specific features are relied upon

## Acceptance

- Clear mapping from skopeo command(s) to gcrane equivalent(s)
- Draft replacement GCB step ready for implementation
- Confirmation that gcrane handles all artifact types we push

### replace-skopeo-with-crane (₢APAAo) [complete]

**[260216-1037] complete**

Replace skopeo with crane in GCB pipeline push step.

## Context

Pace ₢APAAl designed the replacement. This pace implements it. The skopeo tool image is pinned from quay.io which aggressively GCs digests. Crane is already available via RBRR_GCB_GCRANE_IMAGE_REF (gcr.io-hosted). No new image pin needed — we delete RBRR_GCB_SKOPEO_IMAGE_REF and reuse the existing gcrane pin.

## Work — Agent A (bash/env files)

1. Rename `Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh` → `rbgjb07-push-with-crane.sh`
2. Rewrite the script: untar OCI archive, `crane push DIR URI --index`. No manual token fetch (gcloud auth from step 03 handles it). ~20 lines replaces ~82.
3. `Tools/rbw/rbf_Foundry.sh:165` — change `RBRR_GCB_SKOPEO_IMAGE_REF` to `RBRR_GCB_GCRANE_IMAGE_REF`, update script name to `rbgjb07-push-with-crane.sh`, update step ID to `push-with-crane`
4. `rbrr.env:64-65` — delete the two SKOPEO lines (comment + variable)
5. `Tools/rbw/rbrr_cli.sh:109` — remove RBRR_GCB_SKOPEO_IMAGE_REF regime display item
6. `Tools/rbw/rbrr_cli.sh:204` — remove SKOPEO from refresh list
7. `Tools/rbw/rbrr_regime.sh:56` — remove SKOPEO default assignment
8. `Tools/rbw/rbrr_regime.sh:95` — remove SKOPEO from rollup
9. `Tools/rbw/rbrr_regime.sh:156` — remove SKOPEO validation
10. `Tools/rbw/rbrr_regime.sh:62` — remove SKOPEO from env presence check (the long line)
11. `Tools/rbw/rbv_PodmanVM.sh:196` — fix stale log: "skopeo installed" → "crane installed"

## Work — Agent B (adoc spec files)

12. `lenses/RBS0-SpecTop.adoc:257` — remove `:rbrr_gcb_skopeo_image_ref:` attribute reference
13. `lenses/RBS0-SpecTop.adoc:1838-1840` — remove `[[rbrr_gcb_skopeo_image_ref]]` anchor and definition
14. `lenses/RBSOB-oci_layout_bridge.adoc` — update push phase from skopeo to crane throughout; update code blocks and rationale
15. `lenses/RBSRR-RegimeRepo.adoc:182` — remove `{rbrr_gcb_skopeo_image_ref}` reference
16. `lenses/RBWMBX-BuildxMultiPlatformAuth.adoc` — update skopeo references to crane in code blocks and discussion (lines 232, 238, 250, 312, 314, 473)

## Draft replacement script (rbgjb07-push-with-crane.sh)

```bash
#!/bin/bash
# RBGJB Step 07: Push OCI layout to GAR using crane
# Builder: gcr.io/go-containerregistry/gcrane (via RBRR_GCB_GCRANE_IMAGE_REF)
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
#
# OCI Layout Bridge Phase 2: Push the multi-platform OCI archive from /workspace/oci-layout.tar
# to Artifact Registry using crane. Auth provided by gcloud auth configure-docker (step 03).

set -euo pipefail

test -n "${_RBGY_GAR_LOCATION}"   || { echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1; }
test -n "${_RBGY_GAR_PROJECT}"    || { echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1; }
test -n "${_RBGY_GAR_REPOSITORY}" || { echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1; }
test -n "${_RBGY_MONIKER}"        || { echo "_RBGY_MONIKER missing"        >&2; exit 1; }
test -s .tag_base                  || { echo "tag base not derived"         >&2; exit 1; }
test -f /workspace/oci-layout.tar  || { echo "OCI archive not found"        >&2; exit 1; }

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

mkdir -p /workspace/oci-layout
tar xf /workspace/oci-layout.tar -C /workspace/oci-layout

echo "Pushing OCI layout to ${IMAGE_URI}..."
crane push /workspace/oci-layout "${IMAGE_URI}" --index

echo "${IMAGE_URI}" > .image_uri
echo "Done. Image available at: ${IMAGE_URI}"
```

## Acceptance

- `RBRR_GCB_SKOPEO_IMAGE_REF` fully removed from env, regime, cli, and specs
- rbgjb07 renamed and rewritten to use crane
- rbf_Foundry.sh references updated gcrane image ref
- All four adoc files updated to reflect crane instead of skopeo
- Stale log message in rbv_PodmanVM.sh fixed
- `./tt/rbw-qa.QualifyAll.sh` passes (or explain failures unrelated to this change)

## Parallelism

Agent A (bash/env) and Agent B (adoc specs) touch zero overlapping files. Safe for parallel execution.

**[260216-0953] bridled**

Replace skopeo with crane in GCB pipeline push step.

## Context

Pace ₢APAAl designed the replacement. This pace implements it. The skopeo tool image is pinned from quay.io which aggressively GCs digests. Crane is already available via RBRR_GCB_GCRANE_IMAGE_REF (gcr.io-hosted). No new image pin needed — we delete RBRR_GCB_SKOPEO_IMAGE_REF and reuse the existing gcrane pin.

## Work — Agent A (bash/env files)

1. Rename `Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh` → `rbgjb07-push-with-crane.sh`
2. Rewrite the script: untar OCI archive, `crane push DIR URI --index`. No manual token fetch (gcloud auth from step 03 handles it). ~20 lines replaces ~82.
3. `Tools/rbw/rbf_Foundry.sh:165` — change `RBRR_GCB_SKOPEO_IMAGE_REF` to `RBRR_GCB_GCRANE_IMAGE_REF`, update script name to `rbgjb07-push-with-crane.sh`, update step ID to `push-with-crane`
4. `rbrr.env:64-65` — delete the two SKOPEO lines (comment + variable)
5. `Tools/rbw/rbrr_cli.sh:109` — remove RBRR_GCB_SKOPEO_IMAGE_REF regime display item
6. `Tools/rbw/rbrr_cli.sh:204` — remove SKOPEO from refresh list
7. `Tools/rbw/rbrr_regime.sh:56` — remove SKOPEO default assignment
8. `Tools/rbw/rbrr_regime.sh:95` — remove SKOPEO from rollup
9. `Tools/rbw/rbrr_regime.sh:156` — remove SKOPEO validation
10. `Tools/rbw/rbrr_regime.sh:62` — remove SKOPEO from env presence check (the long line)
11. `Tools/rbw/rbv_PodmanVM.sh:196` — fix stale log: "skopeo installed" → "crane installed"

## Work — Agent B (adoc spec files)

12. `lenses/RBS0-SpecTop.adoc:257` — remove `:rbrr_gcb_skopeo_image_ref:` attribute reference
13. `lenses/RBS0-SpecTop.adoc:1838-1840` — remove `[[rbrr_gcb_skopeo_image_ref]]` anchor and definition
14. `lenses/RBSOB-oci_layout_bridge.adoc` — update push phase from skopeo to crane throughout; update code blocks and rationale
15. `lenses/RBSRR-RegimeRepo.adoc:182` — remove `{rbrr_gcb_skopeo_image_ref}` reference
16. `lenses/RBWMBX-BuildxMultiPlatformAuth.adoc` — update skopeo references to crane in code blocks and discussion (lines 232, 238, 250, 312, 314, 473)

## Draft replacement script (rbgjb07-push-with-crane.sh)

```bash
#!/bin/bash
# RBGJB Step 07: Push OCI layout to GAR using crane
# Builder: gcr.io/go-containerregistry/gcrane (via RBRR_GCB_GCRANE_IMAGE_REF)
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
#
# OCI Layout Bridge Phase 2: Push the multi-platform OCI archive from /workspace/oci-layout.tar
# to Artifact Registry using crane. Auth provided by gcloud auth configure-docker (step 03).

set -euo pipefail

test -n "${_RBGY_GAR_LOCATION}"   || { echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1; }
test -n "${_RBGY_GAR_PROJECT}"    || { echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1; }
test -n "${_RBGY_GAR_REPOSITORY}" || { echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1; }
test -n "${_RBGY_MONIKER}"        || { echo "_RBGY_MONIKER missing"        >&2; exit 1; }
test -s .tag_base                  || { echo "tag base not derived"         >&2; exit 1; }
test -f /workspace/oci-layout.tar  || { echo "OCI archive not found"        >&2; exit 1; }

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

mkdir -p /workspace/oci-layout
tar xf /workspace/oci-layout.tar -C /workspace/oci-layout

echo "Pushing OCI layout to ${IMAGE_URI}..."
crane push /workspace/oci-layout "${IMAGE_URI}" --index

echo "${IMAGE_URI}" > .image_uri
echo "Done. Image available at: ${IMAGE_URI}"
```

## Acceptance

- `RBRR_GCB_SKOPEO_IMAGE_REF` fully removed from env, regime, cli, and specs
- rbgjb07 renamed and rewritten to use crane
- rbf_Foundry.sh references updated gcrane image ref
- All four adoc files updated to reflect crane instead of skopeo
- Stale log message in rbv_PodmanVM.sh fixed
- `./tt/rbw-qa.QualifyAll.sh` passes (or explain failures unrelated to this change)

## Parallelism

Agent A (bash/env) and Agent B (adoc specs) touch zero overlapping files. Safe for parallel execution.

*Direction:* Agent: sonnet+sonnet | Cardinality: 2 parallel then notch | Files: rbgjb07-push-with-skopeo.sh, rbf_Foundry.sh, rbrr.env, rbrr_cli.sh, rbrr_regime.sh, rbv_PodmanVM.sh, RBS0-SpecTop.adoc, RBSOB-oci_layout_bridge.adoc, RBSRR-RegimeRepo.adoc, RBWMBX-BuildxMultiPlatformAuth.adoc (10 files) | Steps: 1. Agent A sonnet: git mv rename rbgjb07, rewrite script to use crane push --index, update rbf_Foundry.sh ref, delete SKOPEO from rbrr.env+rbrr_cli.sh+rbrr_regime.sh, fix rbv_PodmanVM.sh log 2. Agent B sonnet: remove skopeo attribute+anchor+definition from RBS0-SpecTop, remove ref from RBSRR-RegimeRepo, update RBSOB push phase to crane, update RBWMBX references to crane | Verify: ./tt/rbw-qa.QualifyAll.sh

**[260216-0950] rough**

Replace skopeo with crane in GCB pipeline push step.

## Context

Pace ₢APAAl designed the replacement. This pace implements it. The skopeo tool image is pinned from quay.io which aggressively GCs digests. Crane is already available via RBRR_GCB_GCRANE_IMAGE_REF (gcr.io-hosted). No new image pin needed — we delete RBRR_GCB_SKOPEO_IMAGE_REF and reuse the existing gcrane pin.

## Work — Agent A (bash/env files)

1. Rename `Tools/rbw/rbgjb/rbgjb07-push-with-skopeo.sh` → `rbgjb07-push-with-crane.sh`
2. Rewrite the script: untar OCI archive, `crane push DIR URI --index`. No manual token fetch (gcloud auth from step 03 handles it). ~20 lines replaces ~82.
3. `Tools/rbw/rbf_Foundry.sh:165` — change `RBRR_GCB_SKOPEO_IMAGE_REF` to `RBRR_GCB_GCRANE_IMAGE_REF`, update script name to `rbgjb07-push-with-crane.sh`, update step ID to `push-with-crane`
4. `rbrr.env:64-65` — delete the two SKOPEO lines (comment + variable)
5. `Tools/rbw/rbrr_cli.sh:109` — remove RBRR_GCB_SKOPEO_IMAGE_REF regime display item
6. `Tools/rbw/rbrr_cli.sh:204` — remove SKOPEO from refresh list
7. `Tools/rbw/rbrr_regime.sh:56` — remove SKOPEO default assignment
8. `Tools/rbw/rbrr_regime.sh:95` — remove SKOPEO from rollup
9. `Tools/rbw/rbrr_regime.sh:156` — remove SKOPEO validation
10. `Tools/rbw/rbrr_regime.sh:62` — remove SKOPEO from env presence check (the long line)
11. `Tools/rbw/rbv_PodmanVM.sh:196` — fix stale log: "skopeo installed" → "crane installed"

## Work — Agent B (adoc spec files)

12. `lenses/RBS0-SpecTop.adoc:257` — remove `:rbrr_gcb_skopeo_image_ref:` attribute reference
13. `lenses/RBS0-SpecTop.adoc:1838-1840` — remove `[[rbrr_gcb_skopeo_image_ref]]` anchor and definition
14. `lenses/RBSOB-oci_layout_bridge.adoc` — update push phase from skopeo to crane throughout; update code blocks and rationale
15. `lenses/RBSRR-RegimeRepo.adoc:182` — remove `{rbrr_gcb_skopeo_image_ref}` reference
16. `lenses/RBWMBX-BuildxMultiPlatformAuth.adoc` — update skopeo references to crane in code blocks and discussion (lines 232, 238, 250, 312, 314, 473)

## Draft replacement script (rbgjb07-push-with-crane.sh)

```bash
#!/bin/bash
# RBGJB Step 07: Push OCI layout to GAR using crane
# Builder: gcr.io/go-containerregistry/gcrane (via RBRR_GCB_GCRANE_IMAGE_REF)
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY, _RBGY_MONIKER
#
# OCI Layout Bridge Phase 2: Push the multi-platform OCI archive from /workspace/oci-layout.tar
# to Artifact Registry using crane. Auth provided by gcloud auth configure-docker (step 03).

set -euo pipefail

test -n "${_RBGY_GAR_LOCATION}"   || { echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1; }
test -n "${_RBGY_GAR_PROJECT}"    || { echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1; }
test -n "${_RBGY_GAR_REPOSITORY}" || { echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1; }
test -n "${_RBGY_MONIKER}"        || { echo "_RBGY_MONIKER missing"        >&2; exit 1; }
test -s .tag_base                  || { echo "tag base not derived"         >&2; exit 1; }
test -f /workspace/oci-layout.tar  || { echo "OCI archive not found"        >&2; exit 1; }

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

mkdir -p /workspace/oci-layout
tar xf /workspace/oci-layout.tar -C /workspace/oci-layout

echo "Pushing OCI layout to ${IMAGE_URI}..."
crane push /workspace/oci-layout "${IMAGE_URI}" --index

echo "${IMAGE_URI}" > .image_uri
echo "Done. Image available at: ${IMAGE_URI}"
```

## Acceptance

- `RBRR_GCB_SKOPEO_IMAGE_REF` fully removed from env, regime, cli, and specs
- rbgjb07 renamed and rewritten to use crane
- rbf_Foundry.sh references updated gcrane image ref
- All four adoc files updated to reflect crane instead of skopeo
- Stale log message in rbv_PodmanVM.sh fixed
- `./tt/rbw-qa.QualifyAll.sh` passes (or explain failures unrelated to this change)

## Parallelism

Agent A (bash/env) and Agent B (adoc specs) touch zero overlapping files. Safe for parallel execution.

### select-gcb-roadmap-mvp-tier (₢APAAr) [complete]

**[260222-1514] complete**

Select which RBSCB Cloud Build Roadmap tier(s) to target for initial MVP release.

## Context

RBSCB-CloudBuildRoadmap.adoc defines progressive hardening tiers (0-5) for the GCB pipeline.
Tier 0 (current baseline) is already in place. The question is which tier(s) must be
complete before the first public release of Recipe Bottle.

## Considerations

- Tier 1 (baseline hardening) is small and independent — base image validation, --provenance flag, log hygiene
- Tier 2 (trigger migration) gives free Google SLSA provenance but is a moderate refactor
- Tiers 3-5 are significant infrastructure investment with GCB lock-in implications
- Multi-architecture strategy (binfmt canary) affects how deep to invest in GCB-specific tiers
- The decision should balance "good enough for alpha" vs "security debt we accept knowingly"

## Outcome

A decision recorded in the RBSCB Decision Log specifying:
- Which tiers are MVP-required
- Which tiers are post-MVP
- Any tier ordering changes based on the decision

**[260216-1144] rough**

Select which RBSCB Cloud Build Roadmap tier(s) to target for initial MVP release.

## Context

RBSCB-CloudBuildRoadmap.adoc defines progressive hardening tiers (0-5) for the GCB pipeline.
Tier 0 (current baseline) is already in place. The question is which tier(s) must be
complete before the first public release of Recipe Bottle.

## Considerations

- Tier 1 (baseline hardening) is small and independent — base image validation, --provenance flag, log hygiene
- Tier 2 (trigger migration) gives free Google SLSA provenance but is a moderate refactor
- Tiers 3-5 are significant infrastructure investment with GCB lock-in implications
- Multi-architecture strategy (binfmt canary) affects how deep to invest in GCB-specific tiers
- The decision should balance "good enough for alpha" vs "security debt we accept knowingly"

## Outcome

A decision recorded in the RBSCB Decision Log specifying:
- Which tiers are MVP-required
- Which tiers are post-MVP
- Any tier ordering changes based on the decision

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 p fix-sbom-arm64-test-vessel
  2 q remove-gcrane-pin-add-crane-freshening
  3 g consolidate-regime-load-primitives
  4 h create-rbcr-regime-render
  5 f multi-nameplate-utilities
  6 V rbsa-local-ops-axla-parity
  7 U create-rbsco-cosmology-overview
  8 K test-image-retrieve
  9 F remove-gcb-jq-image-ref
  10 A verify-depot-and-docker
  11 E verify-cloud-build-pipeline
  12 G improve-image-list-output
  13 I rename-repo-path-to-moniker
  14 L implement-gar-image-resolution
  15 M fix-cloud-build-console-url
  16 P audit-ark-vessel-vocabulary-in-rbsa
  17 Q update-rbs-image-tag-to-vessel-consecration
  18 R plumb-ark-constants-to-gcb
  19 S simplify-rbf-list-raw-images
  20 T implement-rbf-delete-single-tag
  21 W implement-rbf-beseech-ark-view
  22 X align-rbf-retrieve-with-locator-vocab
  23 Y formalize-conjure-in-rbsa
  24 Z wire-ark-colophon-group
  25 O quadruple-build-poll-limit
  26 a introduce-zipper-retire-formulary
  27 b bcg-register-pattern-zipper-fix
  28 c rename-bud-to-burd
  29 d ark-lifecycle-cloud-testbench
  30 J build-and-stage-nsproto-images
  31 B run-nsproto-test-suite
  32 C run-srjcl-test-suite
  33 D run-pluml-test-suite
  34 H pin-gcb-tool-versions
  35 N fix-buc-link-ansi-escapes
  36 e refine-gcb-quota-procedure
  37 i ark-lifecycle-container-smoke
  38 j retire-makefile-workbench
  39 k implement-qualify-all-system
  40 n zipper-registration-cleanup
  41 l design-gcrane-skopeo-replacement
  42 o replace-skopeo-with-crane
  43 r select-gcb-roadmap-mvp-tier

pqghfVUKFAEGILMPQRSTWXYZOabcdJBCDHNeijknlor
xx······x·xx··x··xxxxx·xx··x·x···x·x·····x· rbf_Foundry.sh
·x··x·x···········xxx·xx···x··x····x·····x· RBSA-SpecTop.adoc
·xxx····x·····················x··x·x·····x· rbrr_regime.sh
··xxx······················x·····x····x···x rbw_workbench.sh
··xxx·······x······················x······x rbrn_cli.sh
·························xx········x··xx··· rbz_zipper.sh
·x·······························x·x·····xx rbrr_cli.sh
·························x·x·······x··x···· CLAUDE.md
·························xxx···········x··· buz_zipper.sh
·······x···············x···x·······x······· rbk_Coordinator.sh
··xxx·······x······························ rbrn_regime.sh
·x····························x····x·····x· RBSRR-RegimeRepo.adoc
······································xx··x rbtb_testbench.sh
···························x·········x····x cccw_workbench.sh
··························xx···x··········· BCG-BashConsoleGuide.md
··························xxx·············· rbtg_testbench.sh
·························xxx··············· README.md
············x···x····················x····· RBS-Specification.adoc
········x························x·x······· rbrr_RecipeBottleRegimeRepo.sh
···x······················xx··············· bul_launcher.sh
··xx······································x rbrv_cli.sh
··xx·······························x······· rbgd_DepotConstants.sh
··xx·······················x··············· rbf_cli.sh, rbgg_cli.sh, rbt_testbench.sh, trbim_suite.sh
·x·························x··············x buw_workbench.sh
·x·························x······x········ buc_command.sh
x·········x······················x········· rbgjb08-sbom-and-summary.sh
······································xx··· rbq_Qualify.sh
····························xx············· Dockerfile, rbrv.env
···························x··············x burc_cli.sh, burs_cli.sh, jjw_workbench.sh, vow_workbench.sh, vslw_workbench.sh, vvw_workbench.sh
···························x·········x····· rbw.workbench.mk
···························x·······x······· memo-20260209-regime-inventory.md, rbgm_ManualProcedures.sh
··························x·x·············· rbtg-de.DispatchExercise.sh
·······················x·····x············· rbw-fB.BuildVessel.sh
·······················x···x··············· rbw-GS.DeleteServiceAccount.sh, rbw-Gl.ListServiceAccounts.sh, rbw-aA.AbjureArk.sh, rbw-aC.ConjureArk.sh, rbw-ab.BeseechArk.sh, rbw-as.SummonArk.sh, rbw-iD.DeleteImage.sh
····················x······x··············· vob_build.sh
··················xx······················· RBSIL-image_list.adoc
·················x·······················x· rbgjb07-push-with-skopeo.sh
············x················x············· rbrn_nsproto.env, rbrn_pluml.env, rbrn_srjcl.env
··········xx······························· RBSTB-trigger_build.adoc
······x·········x·························· RBSBC-bottle_create.adoc, RBSBL-bottle_launch.adoc
····x········x····························· rbob_bottle.sh
····x·······x······························ RBRN-RegimeNameplate.adoc
···x·······················x··············· rbgm_cli.sh
··xx······································· rbob_cli.sh
·x········································x bure_cli.sh
·x·······································x· rbrr.env
·x····································x···· buv_validation.sh
x········································x· RBSOB-oci_layout_bridge.adoc
··········································x RBSCB-CloudBuildRoadmap.adoc, burd_regime.sh, butcrg_RegimeSmoke.sh, rbra_cli.sh, rbrp_cli.sh, rbrs_cli.sh
·········································x· RBWMBX-BuildxMultiPlatformAuth.adoc, rbgjb07-push-with-crane.sh, rbv_PodmanVM.sh
·······································x··· JJSCRL-rail.adoc, jjro_ops.rs, jjrrl_rail.rs
······································x···· rbq_cli.sh, rbtcqa_QualifyAll.sh, rbw-qa.QualifyAll.sh
·····································x····· lmci-b.BundleFilesForLLM.sh, lmci-s.StripWhitespaceFiles.sh, mbc.console.mk, mbd.dispatch.sh, mbd.utils.sh, mbv.variables.sh, rbw-D.DigestPodmanHtml.sh, rbw-II.ImageInfoGHCR.sh, rbw-N.NUKE_PODMAN_MACHINE.sh, rbw-b.BuildWithRecipe.sh, rbw-c.CheckLatestPodmanVM.sh, rbw-d.DeleteImage.sh, rbw-f.FetchChosenPodmanVMImage.sh, rbw-hi.HelpImageManagement.sh, rbw-hv.HelpPodmanVM.sh, rbw-hw.HelpResetWSL.sh, rbw-i.InitChosenPodmanVM.sh, rbw-m.MirrorLatestPodmanVM.sh, rbw-s.Start.nsdemo.sh, rbw-vc.PodmanVmCheckLatest.sh, ttc.CreateTabtarget.sh, ttx.FixTabtargetExecutability.sh
···································x······· RBSQB-quota_build.adoc, rbgc_Constants.sh, rbrr.validator.sh, rbw-QB.QuotaBuild.sh
·································x········· bupr_PresentationRegime.sh, rbgjb04-qemu-binfmt.sh, rbw-rrg.RefreshGcbPins.sh
·····························x············· bottle_anthropic_jupyter.recipe, bottle_deftextpro.recipe, bottle_plantuml.recipe, bottle_python_networking.recipe, bottle_rust.recipe, bottle_sci_viz_jupyter.recipe, bottle_textpro.recipe, bottle_ubuntu_test.recipe, rbtest_python_networking.recipe, rbw-lB.LocalBuild.bottle_ubuntu_test.sh, rbw-lB.LocalBuild.sentry_ubuntu_large.sh, rbw-lB.LocalBuild.test_busybox.sh, sentry_alpine_large.recipe, sentry_alpine_small.recipe, sentry_debian_large.recipe, sentry_debian_small.recipe, sentry_ubuntu_large.recipe, sentry_ubuntu_small.recipe, test_busybox.recipe, test_sftrr.recipe
····························x·············· rbtg-al.TEST_ONLY.trbim-macos.sh
···························x··············· RBSDI-director_create.adoc, bug_guide.sh, burd_dispatch.sh, buut_cli.sh, buut_tabtarget.sh, buw-rgi-burc.InfoBurcRegime.sh, buw-rgi-burs.InfoBursRegime.sh, buw-rgr-burc.RenderBurcRegime.sh, buw-rgr-burs.RenderBursRegime.sh, buw-rgv-burc.ValidateBurcRegime.sh, buw-rgv-burs.ValidateBursRegime.sh, buw-tt-cbl.CreateTabTargetBatchLogging.sh, buw-tt-cbn.CreateTabTargetBatchNolog.sh, buw-tt-cil.CreateTabTargetInteractiveLogging.sh, buw-tt-cin.CreateTabTargetInteractiveNolog.sh, buw-tt-cl.CreateLauncher.sh, buw-tt-ll.ListLaunchers.sh, ccck-s.ConnectShell.sh, cmw_workbench.sh, gadcf.LaunchFactoryInContainer.sh, gadi-i.Inspect.sh, jja_arcanum.sh, jjrc_core.rs, memo-20260110-acronym-selection-study.md, memo-20260209-diptych-format-study.md, rbga_ArtifactRegistry.sh, rbga_cli.sh, rbgb_Buckets.sh, rbgb_cli.sh, rbgg_Governor.sh, rbgi_IAM.sh, rbgo_OAuth.sh, rbgp_Payor.sh, rbgu_Utility.sh, rbha_GithubActions.sh, rbhcr_GithubContainerRegistry.sh, rbhh_GithubHost.sh, rbhr_GithubRemote.sh, rbhr_cli.sh, rbi_Image.sh, rbw-B.ConnectBottle.nsproto.sh, rbw-B.ConnectBottle.srjcl.sh, rbw-C.ConnectCenser.nsproto.sh, rbw-GD.GovernorDirectorCreate.sh, rbw-GR.GovernorRetrieverCreate.sh, rbw-PC.PayorDepotCreate.sh, rbw-PD.PayorDepotDestroy.sh, rbw-PG.PayorGovernorReset.sh, rbw-PI.PayorInstall.sh, rbw-S.ConnectSentry.nsproto.sh, rbw-S.ConnectSentry.pluml.sh, rbw-S.ConnectSentry.srjcl.sh, rbw-hga.HelpGoogleAdmin.sh, rbw-him.HelpImageManagement.sh, rbw-iB.BuildImageRemotely.sh, rbw-il.ImageList.sh, rbw-l.ListCurrentRegistryImages.sh, rbw-ld.ListDepots.sh, rbw-o.ObserveNetworks.nsproto.sh, rbw-o.ObserveNetworks.pluml.sh, rbw-o.ObserveNetworks.srjcl.sh, rbw-ps.ShowPayorEstablishment.sh, rbw-s.Start.nsproto.sh, rbw-s.Start.pluml.sh, rbw-s.Start.srjcl.sh, rbw-z.Stop.nsproto.sh, rbw-z.Stop.pluml.sh, rbw-z.Stop.srjcl.sh, rgbs_ServiceAccounts.sh, rgbs_cli.sh, vob_cli.sh, vocbumc_core.md, vow-F.Freshen.sh, vow-P.Parcel.sh, vow-R.Release.sh, vow-b.Build.sh, vow-c.Clean.sh, vow-t.Test.sh, vvce_env.rs, vvcp_probe.rs, vvtg_guard.rs, vvw-r.RunVVX.sh
··························x················ bud_dispatch.sh, launcher.rbtg_testbench.sh
·······················x··················· RBSAS-ark_summon.adoc, rbw-a.PodmanStart.sh, rbw-aCD.CreateDirectorAccount.sh, rbw-aCR.CreateReaderAccount.sh, rbw-aDS.DeleteServiceAccount.sh, rbw-aIA.InitializeAdminAccount.sh, rbw-aID.DELETE_ALL.sh, rbw-aPO.ObliterateProject.sh, rbw-aPr.RestoreProject.sh, rbw-al.ListServiceAccounts.sh, rbw-fA.AbjureArk.sh, rbw-fD.DeleteImage.sh, rbw-fS.BuildStudyDEBUG.sh
······················x···················· RBSAC-ark_conjure.adoc
·····················x····················· RBSIR-image_retrieve.adoc
····················x······················ .gitignore, .gitkeep, RBSAB-ark_beseech.adoc
···················x······················· RBSAA-ark_abjure.adoc, RBSID-image_delete.adoc
·················x························· rbgjb03-docker-login-gar.sh, rbgjb06-build-and-export.sh, rbgjb09-build-and-push-metadata.sh
·········x································· vovr_registry.json
·······x··································· rbw-ir.RetrieveImage.sh, rbw-r.RetrieveImage.sh
······x···································· RBSBK-bottle_cleanup.adoc, RBSBR-bottle_run.adoc, RBSBS-bottle_start.adoc, RBSCE-command_exec.adoc, RBSCO-CosmologyIntro.adoc, RBSNC-network_create.adoc, RBSNX-network_connect.adoc, RBSSS-sentry_start.adoc, RBSVC-rbv_check.adoc, RBSVM-rbv_mirror.adoc
····x······································ AXLA-Lexicon.adoc, RBSRV-RegimeVessel.adoc, rbw-ni.NameplateInfo.sh, rbw-nv.ValidateNameplates.sh
···x······································· rbcc_Constants.sh, rbcr_render.sh, rbgp_cli.sh
·x········································· BUSA-BashUtilitiesSpec.adoc, bure_regime.sh, buw-rer.RenderEnvironmentRegime.sh, buw-rev.ValidateEnvironmentRegime.sh, rbgjm_mirror.json

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 312 commits)

  1 n zipper-registration-cleanup
  2 l design-gcrane-skopeo-replacement
  3 o replace-skopeo-with-crane
  4 p fix-sbom-arm64-test-vessel
  5 q remove-gcrane-pin-add-crane-freshening
  6 r select-gcb-roadmap-mvp-tier
  7 m unknown

123456789abcdefghijklmnopqrstuvwxyz
··xxx······························  n  3c
······x·x··························  l  2c
··········xxxx·xxx·················  o  7c
····················xxx············  p  3c
························xx·xx······  q  4c
·······························xxx·  r  3c
··································x  m  1c
```

## Steeplechase

### 2026-02-24 07:13 - ₢APAAm - A

Add runtime probe to infra-dependent suites; inconclusive on failure; TCC diagnostic on macOS

### 2026-02-22 15:14 - ₢APAAr - W

Resolved: Tier 2 MVP-required, Tiers 3-5 post-MVP, decisions recorded in RBSCB Decision Log by ₢AiAAA

### 2026-02-22 15:14 - ₢APAAr - n

Enrollment+scope_sentinel+enforce in burd_regime, zburd_enforce added to all scrubbed CLI furnish functions

### 2026-02-22 13:22 - ₢APAAr - A

Decision pace: present tier tradeoffs, get user MVP boundary decision, record in RBSCB Decision Log

### 2026-02-22 13:21 - Heat - r

moved APAAr to first

### 2026-02-18 08:18 - Heat - n

Replace grep with bash [[ =~ ]] regex in rbrr_regime validation (BCG compliance)

### 2026-02-17 06:43 - ₢APAAq - W

Remove RBRR_GCB_GCRANE_IMAGE_REF from 6 files, delete dormant rbgjm_mirror.json, add crane tarball freshening to refresh_gcb_pins

### 2026-02-17 06:43 - ₢APAAq - n

Add BURE ambient environment regime with countdown skip override, enum validator, and CLI/tabtargets

### 2026-02-17 06:32 - Heat - n

Add cloud build roadmap lens

### 2026-02-17 06:31 - ₢APAAq - n

Remove RBRR_GCB_GCRANE_IMAGE_REF from all files, delete dormant rbgjm_mirror.json, add crane tarball freshening to refresh_gcb_pins

### 2026-02-17 06:26 - ₢APAAq - A

Remove gcrane refs from 6 files, delete mirror.json, add crane tarball freshening to refresh

### 2026-02-16 11:44 - Heat - S

select-gcb-roadmap-mvp-tier

### 2026-02-16 10:59 - ₢APAAp - W

Switched SBOM step from docker-pull to oci-dir: scheme; ark-lifecycle passes end-to-end including arm64 vessel on amd64 GCB worker

### 2026-02-16 10:48 - ₢APAAp - n

Switch SBOM step from docker-pull to oci-dir: scheme, fix stale skopeo comment, update RBSOB trade study

### 2026-02-16 10:41 - ₢APAAp - A

Switch syft from docker-pull to oci-dir:/workspace/oci-layout; remove docker socket mount; fix stale skopeo comment in foundry

### 2026-02-16 10:39 - Heat - S

remove-gcrane-pin-add-crane-freshening

### 2026-02-16 10:38 - Heat - S

fix-sbom-arm64-test-vessel

### 2026-02-16 10:37 - ₢APAAo - W

Crane push to GAR verified working in GCB. SBOM step failure is pre-existing (single-arch arm64 vessel on amd64 worker). Remaining items slated as follow-on paces.

### 2026-02-16 10:29 - ₢APAAo - n

Switch step 07 to alpine+crane: gcrane image is distroless (no shell), install crane from RBRR_CRANE_TAR_GZ tarball, add _RBGY_CRANE_TAR_GZ substitution

### 2026-02-16 10:25 - ₢APAAo - n

Fix GCB stitch entrypoints: bare sh/bash to /bin/sh and /bin/bash for distroless containers

### 2026-02-16 10:16 - Heat - T

suite-infrastructure-preconditions

### 2026-02-16 09:58 - ₢APAAo - n

Replace skopeo with crane in GCB pipeline: rename rbgjb07, rewrite push step, remove RBRR_GCB_SKOPEO_IMAGE_REF, update 4 adoc specs

### 2026-02-16 09:57 - ₢APAAo - L

sonnet landed

### 2026-02-16 09:53 - ₢APAAo - F

Executing bridled pace via 2 parallel sonnet agents

### 2026-02-16 09:53 - ₢APAAo - B

arm | replace-skopeo-with-crane

### 2026-02-16 09:53 - Heat - T

replace-skopeo-with-crane

### 2026-02-16 09:51 - ₢APAAl - W

Designed crane replacement for skopeo in GCB pipeline: command mapping, draft replacement script, full blast radius across 11 files, parallelization strategy

### 2026-02-16 09:50 - Heat - S

replace-skopeo-with-crane

### 2026-02-16 09:33 - ₢APAAl - A

Research gcrane OCI archive push capabilities; map auth model; draft replacement GCB step

### 2026-02-16 09:31 - Heat - n

Clarify officium definition in CLAUDE.md: not sessions, concurrent git-activity streams

### 2026-02-16 09:24 - ₢APAAn - W

FM-001 zipper migration: buz_register to buz_blazon with printf-v output params, _roll array naming, local z_mod deduplication, 5 bottle colophon registrations, qualify-all clean pass

### 2026-02-16 09:24 - ₢APAAn - n

Remove jjx_reorder order mode, keeping only move mode for single-pace relocation

### 2026-02-16 09:24 - ₢APAAn - n

FM-001 zipper migration: buz_register→buz_blazon with printf-v, _roll arrays, local z_mod deduplication, bottle colophon registrations

### 2026-02-16 09:08 - Heat - T

zipper-registration-cleanup

### 2026-02-16 09:08 - Heat - T

achieve-clean-qualify-pass

### 2026-02-16 08:44 - ₢APAAn - A

Register 5 bottle colophons in rbz_zipper.sh; keep explicit workbench case arms

### 2026-02-16 08:41 - Heat - r

moved APAAn to first

### 2026-02-16 08:41 - ₢APAAk - W

Built qualify-all system: buv structural checks, rbq orchestrator/CLI, zipper registration, workbench gates, testbench suite

### 2026-02-16 08:40 - Heat - S

achieve-clean-qualify-pass

### 2026-02-16 08:39 - ₢APAAk - n

Implement qualify-all system: buv structural checks, rbq orchestrator, rbq CLI, zipper registration, tabtarget, workbench gates for rbw-s/rbw-iB, testbench suite, CLAUDE.md mapping

### 2026-02-16 08:19 - ₢APAAk - A

Multi-layer: buv structural + rbq orchestrator + subprocess gates + testbench suite

### 2026-02-16 08:16 - Heat - T

implement-qualify-all-system

### 2026-02-16 08:16 - Heat - T

create-rbq-qualify-module

### 2026-02-16 07:54 - ₢APAAj - W

Retired makefile workbench infrastructure: deleted rbw.workbench.mk, mbd/mbc dispatch, mbv.variables.sh, 17 old tabtargets, 2 lmci tabtargets; fixed rbk_show in cccw; LMCI tools kept without launchers

### 2026-02-16 07:54 - ₢APAAj - n

Delete RBS-Specification.adoc (RBAGS already gone), update CLAUDE.md mappings, verify no broken references

### 2026-02-16 07:53 - ₢APAAj - n

Retire makefile workbench: delete rbw.workbench.mk, mbd dispatch, 17 old tabtargets; fix rbk_show in cccw

### 2026-02-16 07:50 - Heat - T

retire-makefile-workbench

### 2026-02-16 07:50 - Heat - T

delete-rbk-coordinator

### 2026-02-16 07:35 - ₢APAAj - A

Fix rbk_show in cccw, leave rbw.workbench.mk intact (active MBD system)

### 2026-02-16 06:12 - Heat - S

suite-infrastructure-preconditions

### 2026-02-13 11:37 - ₢APAAj - A

Cleanup residual rbk_ and rbw.workbench.mk references: fix cccw rbk_show, assess mbv.variables.sh dead ref, check CRR doc

### 2026-02-13 10:03 - Heat - T

create-rbq-qualify-module

### 2026-02-13 09:59 - Heat - S

design-gcrane-skopeo-replacement

### 2026-02-13 09:53 - Heat - T

ark-lifecycle-container-smoke

### 2026-02-13 09:46 - ₢APAAi - A

Add 2 smoke steps (start+verify container) between retrieve and delete in rbtcal_lifecycle, 6-step→8-step

### 2026-02-13 09:42 - ₢APAAe - W

Programmatic GCB quota check added to audit and build preflight; RBSQB manual procedure refinement deferred to separate pace

### 2026-02-13 09:24 - Heat - S

create-rbq-qualify-module

### 2026-02-13 09:10 - ₢APAAe - n

Add programmatic GCB quota check to audit and build preflight (UNTESTED — parallel editing prevents integration test)

### 2026-02-13 08:13 - ₢APAAe - A

Separate Console procedure from regime config: keep machine-type as context, remove prescriptive Options A/B, focus on quota check and increase

### 2026-02-13 08:09 - Heat - D

restring 1 paces from ₣AU

### 2026-02-13 07:02 - Heat - n

Fix digest pinning: use real registry digest via --verbose manifest inspect, switch step-def delimiter to pipe, BCG-comply both functions with per-item temp files

### 2026-02-13 06:46 - Heat - n

Fix IFS delimiter collision: sha256 digests in pinned image refs contain colons that broke colon-delimited step definitions

### 2026-02-13 06:30 - ₢APAAe - A

Refocus RBSQB+rbgm on Console quota check; demote machine-type to context reference

### 2026-02-12 14:18 - Heat - T

refine-gcb-quota-procedure

### 2026-02-12 14:18 - Heat - T

increase-gcb-concurrent-build-quota

### 2026-02-12 14:07 - ₢APAAe - n

GCB concurrent build fix: CPU quota was bottleneck (8 of 10 CPUs), not build count; downgrade machine type to UNSPECIFIED (2 vCPU, 5 concurrent); new RBSQB spec, rbgm_quota_build guide function, rbw-QB tabtarget wiring

### 2026-02-12 13:54 - Heat - n

Fully qualify Docker Hub image refs (alpine, syft, binfmt) to pass buv_val_odref validation

### 2026-02-12 13:34 - ₢APAAe - A

Document GCB default pool concurrency limit in RBSA spec; guide user on manual quota increase

### 2026-02-12 13:27 - ₢APAAH - W

GCB image pin infrastructure: RBRR variables, BURD-routed refresh with oras auto-discovery, wired into rbf_Foundry and rbgjb scripts

### 2026-02-12 13:27 - ₢APAAH - n

Narrow-terminal regime field rendering: 3-line layout with name+req+type, value, description on separate lines

### 2026-02-12 13:27 - ₢APAAH - n

Add rate-limit guidance printouts to refresh command

### 2026-02-12 13:25 - ₢APAAH - n

BCG-compliant oras tag discovery via GHCR API with jq; BURD-routed tabtarget replaces standalone script

### 2026-02-12 13:13 - ₢APAAH - n

Run GCB pin refresh: all 8 images resolved to digests; fix oras tag to v1.2.2 (no latest tag published)

### 2026-02-12 13:06 - ₢APAAH - n

Add RBRR GCB image pin variables, BURD-routed refresh command, wire pins into rbf_Foundry and rbgjb step scripts

### 2026-02-12 12:51 - ₢APAAH - A

Parallel 3x sonnet: A=tabtarget+RBRR seeds, B=foundry+rbgjb wiring, C=regime plumbing

### 2026-02-12 12:49 - Heat - T

pin-gcb-tool-versions

### 2026-02-12 12:06 - ₢APAAD - W

Pluml PlantUML test suite passed: 5/5 cases

### 2026-02-12 12:04 - ₢APAAD - A

Start pluml, run 5 PlantUML tests, stop pluml

### 2026-02-12 12:03 - ₢APAAC - W

Srjcl Jupyter test suite passed (3/3); simplified BCG load-then-iterate rule to unconditional

### 2026-02-12 12:02 - ₢APAAC - n

Simplify BCG load-then-iterate rule: remove conditional exceptions, require pattern unconditionally for all while-read loops

### 2026-02-12 11:58 - ₢APAAC - A

Run srjcl-jupyter test suite via tt/rbtb-sj tabtarget; diagnose failures

### 2026-02-12 11:56 - Heat - n

Fix stdin consumption bug in butd_dispatch: while-read loops let docker exec -i consume case list; apply load-then-iterate pattern; add BCG anti-pattern entry

### 2026-02-12 11:40 - ₢APAAB - W

nsproto security test suite passed: 4/4 cases

### 2026-02-12 11:40 - ₢APAAB - n

Split rbrr_regime.sh kindle into broach/validate phases, add RBRR voicings to RBSA, create RBSRR regime repo subdoc

### 2026-02-12 11:37 - ₢APAAJ - W

All nameplate arks already present locally — confirmed via rbw-ni survey showing ok for all 6 sentry+bottle images

### 2026-02-12 11:33 - ₢APAAf - W

Implemented cross-nameplate survey/audit/preflight with tabtargets rbw-ni and rbw-nv, rbob_start integration, and per-nameplate IP subnet membership validation

### 2026-02-12 11:33 - ₢APAAf - n

Add cross-nameplate validation: preflight conflict detection (ports, subnets, IPs), survey/audit commands, subnet membership checks, MCM normalization across AXLA/RBSA/RBRN/RBSRV

### 2026-02-12 10:54 - ₢APAAf - A

Validation+info in rbrn_regime/cli, bulk summon separate, rbob_start integration

### 2026-02-12 09:34 - ₢APAAh - W

Created rbcr_render.sh shared renderer with section gating, terminal-adaptive layouts, blue badges, green since-clauses; refactored rbrn_cli and rbrv_cli to use it; RBRV gating deferred to RBRV_VESSEL_MODE implementation

### 2026-02-12 09:32 - Heat - S

ark-lifecycle-container-smoke

### 2026-02-12 09:16 - ₢APAAh - n

Rename rbcr_line to rbcr_section_item, add rbcr_item for sectionless fields, change since-clause color to green, normalize indentation

### 2026-02-12 09:03 - ₢APAAh - n

Cosmetic refinements: blue for req/type badges and since-clauses, remove ==== header lines, column-align gated section titles with (since VAR=val)

### 2026-02-12 08:44 - ₢APAAh - n

Create shared rbcr_render.sh module with section gating, terminal-adaptive layouts, type/req badges; refactor rbrn_cli and rbrv_cli to use shared renderer; add BURD_TERM_COLS via stty in launcher

### 2026-02-12 08:25 - Heat - T

create-rbcr-regime-render

### 2026-02-12 08:25 - Heat - T

regime-render-polish

### 2026-02-12 07:25 - ₢APAAh - A

Interactive render review: run rbrn/rbrv renders, collect user cosmetic nudges, apply refinements

### 2026-02-12 06:37 - ₢APAAN - W

Fixed zbuc_hyperlink ANSI escape rendering using dollar-quote syntax; also improved BURD_REGIME_FILE sentinel name

### 2026-02-12 06:27 - ₢APAAN - n

Fix zbuc_hyperlink ANSI escapes (dollar-quote) and improve BURD_REGIME_FILE sentinel name

### 2026-02-10 12:57 - ₢APAAh - n

Source rbcc_Constants and rbrn_regime in rbw_workbench so rbrn_list is available for no-arg nameplate listing

### 2026-02-10 12:49 - ₢APAAh - A

Interactive render review: run rbrn/rbrv renders, collect user cosmetic feedback, apply refinements

### 2026-02-10 11:14 - Heat - T

regime-render-polish

### 2026-02-10 11:14 - Heat - T

regime-load-cleanup

### 2026-02-10 10:54 - ₢APAAh - n

Create RBCC constants module, fix BURC_PROJECT_ROOT bug, replace magic strings, refactor missed callsites, wire RBCC kindle into all callers

### 2026-02-10 10:54 - Heat - T

regime-load-cleanup

### 2026-02-10 10:19 - Heat - S

regime-load-cleanup

### 2026-02-10 10:19 - ₢APAAg - W

Created rbrn_load, rbrn_load_file, rbrn_list in rbrn_regime.sh and rbrr_load in rbrr_regime.sh. Refactored 11 callsite files. Known followup: magic strings, BURC_PROJECT_ROOT bug, missed callsites rbgm_cli.sh and rbgp_cli.sh.

### 2026-02-10 09:57 - ₢APAAg - L

sonnet landed

### 2026-02-10 09:57 - ₢APAAg - n

Consolidate regime load primitives: rbrn_load, rbrn_list, rbrr_load

### 2026-02-10 09:53 - ₢APAAg - F

Executing bridled pace via sonnet agent

### 2026-02-10 09:52 - ₢APAAg - B

arm | consolidate-regime-load-primitives

### 2026-02-10 09:52 - Heat - T

consolidate-regime-load-primitives

### 2026-02-10 09:47 - Heat - T

multi-nameplate-utilities

### 2026-02-10 09:47 - Heat - T

redesign-ark-summon-abjure-api

### 2026-02-10 09:46 - Heat - S

consolidate-regime-load-primitives

### 2026-02-10 03:02 - Heat - T

redesign-ark-summon-abjure-api

### 2026-02-09 17:27 - Heat - S

redesign-ark-summon-abjure-api

### 2026-02-09 17:27 - ₢APAAJ - n

Fix curl HEAD requests in summon/abjure (use --head not -X HEAD), set nsproto nameplate consecrations from successful cloud builds

### 2026-02-09 17:19 - Heat - n

Fix ark function family to accept vessel directory paths: strip path prefix in beseech, summon, abjure so rbev-vessels/name works like bare name

### 2026-02-09 17:09 - Heat - S

increase-gcb-concurrent-build-quota

### 2026-02-09 16:32 - Heat - T

build-and-stage-nsproto-images

### 2026-02-09 16:28 - Heat - T

build-and-stage-nsproto-images

### 2026-02-09 16:22 - ₢APAAd - W

pace complete

### 2026-02-09 16:08 - ₢APAAd - n

Add ark lifecycle testbench: trbim-macos vessel, rbtg-al route, 6-step dispatch test case. Fix rbtg-de tabtarget to canonical form. Step 1 (list baseline) passes; step 2 blocked by git-clean gate (expected pre-commit).

### 2026-02-09 15:16 - ₢APAAd - A

6-step ark lifecycle test case following existing dispatch exercise pattern

### 2026-02-09 11:41 - ₢APAAc - W

pace complete

### 2026-02-09 11:41 - ₢APAAc - n

Pure mechanical BUD_->BURD_ zbud_->zburd_ rename across all dependent files, tabtargets, docs, and Rust sources. Add diptych format study memo.

### 2026-02-09 11:29 - ₢APAAc - A

Parallel haiku x4: bud_dispatch.sh rename+refactor, dependent buk files, tt/ tabtargets, docs. Pure mechanical BUD_->BURD_ zbud_->zburd_ rename.

### 2026-02-09 11:28 - ₢APAAb - W

pace complete

### 2026-02-09 11:28 - ₢APAAb - n

Add _register function type to BCG, rename buz_create_capture to buz_register with z1z_ return vars, update rbz callers, remove dead getters

### 2026-02-09 11:21 - ₢APAAb - A

BCG _register pattern doc + buz_register refactor + rbz caller update + README update

### 2026-02-09 11:20 - Heat - T

bcg-register-pattern-zipper-fix

### 2026-02-09 11:20 - Heat - T

zipper-dispatch-burv-testbench

### 2026-02-09 11:19 - ₢APAAb - n

Add dispatch infrastructure to buz_zipper.sh, BURV isolation in bul_launcher.sh+bud_dispatch.sh, create rbtg testbench with dispatch exercise

### 2026-02-09 10:29 - ₢APAAb - A

Add 4th parallel array (tabtargets) + dispatch/evidence to buz_zipper.sh, BURV overrides in bul_launcher.sh, new rbtg_testbench.sh with dispatch exercise

### 2026-02-09 10:27 - Heat - S

ark-lifecycle-cloud-testbench

### 2026-02-09 10:27 - Heat - S

rename-bud-to-burd

### 2026-02-09 10:27 - Heat - T

zipper-dispatch-burv-testbench

### 2026-02-09 10:27 - Heat - T

zipper-dispatch-ark-lifecycle-testbench

### 2026-02-09 10:00 - Heat - r

moved APAAb to first

### 2026-02-09 08:40 - Heat - S

zipper-dispatch-ark-lifecycle-testbench

### 2026-02-09 08:07 - ₢APAAa - W

pace complete

### 2026-02-09 08:07 - ₢APAAa - n

Create buz_zipper.sh + rbz_zipper.sh, retire formulary from BUK README and CLAUDE.md

### 2026-02-09 07:57 - ₢APAAa - A

Create buz_zipper.sh + rbz_zipper.sh, retire formulary from BUK README and CLAUDE.md

### 2026-02-09 07:55 - ₢APAAO - W

pace complete

### 2026-02-09 07:48 - ₢APAAZ - W

pace complete

### 2026-02-09 07:48 - Heat - T

introduce-zipper-retire-formulary

### 2026-02-09 07:47 - Heat - T

introduce-zipper-colophon-registry

### 2026-02-09 07:39 - Heat - S

introduce-zipper-colophon-registry

### 2026-02-09 07:25 - ₢APAAZ - n

Wire ark colophon group (rbw-a*), implement rbf_summon, relocate service account ops to rbw-G*, rename rbw-fD to rbw-iD, remove 13 dead tabtargets

### 2026-02-09 07:11 - Heat - T

build-and-stage-nsproto-images

### 2026-02-09 07:09 - Heat - T

wire-ark-colophon-group

### 2026-02-09 07:09 - Heat - T

implement-rbf-summon-ark-retrieve

### 2026-02-09 06:36 - ₢APAAZ - A

Implement rbf_summon following rbf_abjure/rbf_retrieve patterns: function in rbf_Foundry.sh, coordinator wiring rbw-fS, tabtarget, RBSA mapping, RBSAS lens

### 2026-02-09 06:32 - ₢APAAY - W

pace complete

### 2026-02-09 06:32 - ₢APAAY - n

Add rbtgo_ark_conjure mapping+section in RBSA, create RBSAC-ark_conjure.adoc lens, update ark definition to use conjure verb

### 2026-02-09 06:29 - ₢APAAY - F

Executing via haiku agent

### 2026-02-09 06:28 - ₢APAAY - A

Add rbtgo_ark_conjure mapping+section in RBSA, create RBSAC lens, update ark definition

### 2026-02-09 06:27 - ₢APAAX - W

pace complete

### 2026-02-09 06:27 - ₢APAAX - n

Align rbf_retrieve with locator vocab: accept moniker:tag only, drop digest support, mirror rbf_delete pattern in code and spec

### 2026-02-09 06:18 - ₢APAAX - F

Executing bridled pace via haiku agent

### 2026-02-09 06:18 - ₢APAAX - A

Align rbf_retrieve param to locator (moniker:tag), drop digest, mirror rbf_delete pattern in code and spec

### 2026-02-09 06:15 - ₢APAAW - W

pace complete

### 2026-02-09 06:15 - ₢APAAW - n

Add rbf_beseech function for ark enumeration, move JJK parcels gitignore to .jjk/, fix vob_build parcels_dir path

### 2026-02-09 06:11 - ₢APAAW - F

Executing via sonnet agent

### 2026-02-09 06:10 - ₢APAAW - A

3-part: rbf_beseech function (reusing rbf_list pattern), RBSA mapping+section, RBSAB operation lens

### 2026-02-09 06:08 - Heat - T

build-and-stage-nsproto-images

### 2026-02-09 06:07 - Heat - S

implement-rbf-summon-ark-retrieve

### 2026-02-09 06:06 - Heat - S

formalize-conjure-in-rbsa

### 2026-02-09 05:58 - Heat - S

align-rbf-retrieve-with-locator-vocab

### 2026-02-09 05:43 - ₢APAAW - A

Implement rbf_beseech: fetch tags, parse ark suffixes from constants, group by consecration, tabular output

### 2026-02-09 05:42 - ₢APAAT - W

pace complete

### 2026-02-09 05:42 - ₢APAAT - n

Rewrite rbf_delete for locator input with buc_require confirmation, update rbf_abjure to buc_require, update both specs

### 2026-02-09 05:26 - ₢APAAT - n

Add rbst_locator type, simplify rbf_list to single-mode locator output with Repository header

### 2026-02-09 05:20 - ₢APAAO - n

Quadruple build poll limit from 240 to 960 attempts (20min to 80min)

### 2026-02-09 05:18 - ₢APAAO - F

Executing bridled pace via haiku agent

### 2026-02-09 05:04 - ₢APAAT - A

Align rbf_delete impl (add confirmation/force) and RBSID spec (two-arg, tag-direct delete, confirmation step)

### 2026-02-08 16:30 - ₢APAAS - W

pace complete

### 2026-02-08 16:30 - ₢APAAS - n

Remove -meta companion logic from rbf_list, flatten output to raw tags, create RBSIL lens

### 2026-02-08 16:27 - ₢APAAS - L

sonnet landed

### 2026-02-08 16:24 - ₢APAAS - F

Executing bridled pace via sonnet agents (2 parallel)

### 2026-02-08 16:21 - ₢APAAS - B

arm | simplify-rbf-list-raw-images

### 2026-02-08 16:21 - Heat - T

simplify-rbf-list-raw-images

### 2026-02-08 16:20 - ₢APAAS - A

Remove -meta companion logic from rbf_list, flatten output to raw tags, create RBSIL lens

### 2026-02-08 16:03 - Heat - T

implement-rbf-beseech-ark-view

### 2026-02-08 16:03 - Heat - T

implement-rbf-augur-ark-view

### 2026-02-08 16:03 - Heat - T

implement-rbf-delete-single-tag

### 2026-02-08 16:00 - Heat - T

simplify-rbf-list-raw-images

### 2026-02-08 15:59 - Heat - S

implement-rbf-augur-ark-view

### 2026-02-08 15:58 - Heat - T

implement-rbf-delete-single-tag

### 2026-02-08 15:58 - Heat - T

add-image-delete-by-ark

### 2026-02-08 15:58 - Heat - T

simplify-rbf-list-raw-images

### 2026-02-08 15:58 - Heat - T

update-imagelist-retrieve-for-ark

### 2026-02-08 15:39 - ₢APAAS - A

Rewrite rbf_list to parse ark tag suffixes instead of -meta companions; update rbf_retrieve for ark input

### 2026-02-08 15:34 - ₢APAAR - W

pace complete

### 2026-02-08 15:34 - ₢APAAR - n

Plumb RBGC ark constants to Cloud Build via substitution variables, replacing hardcoded -docker.pkg.dev, -img, and -meta in rbgjb scripts

### 2026-02-08 15:32 - ₢APAAQ - W

pace complete

### 2026-02-08 15:32 - ₢APAAQ - n

Replace stale rbrn_*_image_tag references with vessel/consecration vocabulary across RBS spec and bottle operation lenses

### 2026-02-08 15:30 - ₢APAAQ - A

Remove rbrn_*_image_tag from RBS-Spec, replace with vessel+consecration references from RBSA

### 2026-02-08 15:30 - Heat - T

update-rbs-image-tag-to-vessel-consecration

### 2026-02-08 15:30 - Heat - T

rename-nameplate-fields-to-ark

### 2026-02-08 15:28 - ₢APAAP - W

pace complete

### 2026-02-08 15:26 - ₢APAAP - A

Direct haiku audit of Ark/Vessel vocabulary completeness in RBSA

### 2026-02-08 15:26 - Heat - T

audit-ark-vessel-vocabulary-in-rbsa

### 2026-02-08 15:26 - Heat - T

define-ark-vocabulary-in-rbags

### 2026-02-08 15:22 - ₢APAAU - W

pace complete

### 2026-02-08 15:22 - ₢APAAU - n

Rewrite RBS operation lenses to MCM behavioral notation with structured steps, annotations, and completion semantics

### 2026-02-08 15:22 - ₢APAAU - n

Create RBSCO-CosmologyIntro.adoc from index.adoc ancestry; add at_build_service and at_runtime terms to RBSA; strip local mappings/definitions in favor of RBSA consumers

### 2026-02-08 15:14 - Heat - T

rbsa-local-ops-axla-parity

### 2026-02-08 15:06 - Heat - T

rbsa-local-ops-axla-parity

### 2026-02-08 15:06 - Heat - T

explode-rbs-procedures-axla-voicings

### 2026-02-08 14:58 - ₢APAAV - A

Explore AXLA voicings for RBS subfiles; compare existing RBSAA/RBAGS patterns against RBS parent

### 2026-02-08 14:53 - Heat - T

create-rbsco-cosmology-overview

### 2026-02-08 14:53 - Heat - T

add-recipe-bottle-cosmology-intro

### 2026-02-01 20:21 - Heat - T

rename-artifact-suffixes-to-ark

### 2026-01-30 08:05 - Heat - r

moved APAAV to first

### 2026-01-30 08:04 - Heat - D

restring 1 paces from ₣AR

### 2026-01-30 06:49 - ₢APAAU - A

Add metaphor intro section before Depots and Roles; use existing RBAGS vocab for ark/vessel; narrative prose for demon/bottle/sentry/censer concepts

### 2026-01-30 06:48 - Heat - s

260130-0648 session

### 2026-01-28 21:39 - Heat - n

add RBSAA-ark_abjure spec and ark vocabulary to RBAGS

### 2026-01-28 21:19 - Heat - S

add-recipe-bottle-cosmology-intro

### 2026-01-28 21:16 - Heat - r

moved APAAT before APAAO

### 2026-01-28 21:16 - Heat - r

moved APAAS before APAAO

### 2026-01-28 21:16 - Heat - r

moved APAAR before APAAO

### 2026-01-28 21:16 - Heat - r

moved APAAQ before APAAO

### 2026-01-28 21:16 - Heat - r

moved APAAP before APAAO

### 2026-01-28 21:16 - Heat - S

add-image-delete-by-ark

### 2026-01-28 21:16 - Heat - S

update-imagelist-retrieve-for-ark

### 2026-01-28 21:16 - Heat - S

rename-artifact-suffixes-to-ark

### 2026-01-28 21:16 - Heat - S

rename-nameplate-fields-to-ark

### 2026-01-28 21:16 - Heat - S

define-ark-vocabulary-in-rbags

### 2026-01-28 20:42 - ₢APAAO - B

tally | quadruple-build-poll-limit

### 2026-01-28 20:42 - Heat - T

quadruple-build-poll-limit

### 2026-01-28 20:40 - Heat - r

moved APAAO before APAAJ

### 2026-01-28 20:39 - Heat - S

quadruple-build-poll-limit

### 2026-01-28 20:35 - Heat - S

fix-buc-link-ansi-escapes

### 2026-01-28 20:32 - ₢APAAJ - n

Regenerate BuildVessel tabtarget via CreateTabTargetBatchLogging

### 2026-01-28 20:31 - ₢APAAJ - n

Enable logging for BuildVessel tabtarget

### 2026-01-28 20:27 - ₢APAAM - W

pace complete

### 2026-01-28 20:27 - ₢APAAM - n

Fix Cloud Build console URL to include region parameter

### 2026-01-28 20:24 - ₢APAAM - F

Executing bridled pace via haiku agent

### 2026-01-28 20:23 - ₢APAAM - B

tally | fix-cloud-build-console-url

### 2026-01-28 20:23 - Heat - T

fix-cloud-build-console-url

### 2026-01-28 20:22 - Heat - r

moved APAAM to first

### 2026-01-28 08:30 - Heat - T

build-and-stage-nsproto-images

### 2026-01-28 08:15 - Heat - S

fix-cloud-build-console-url

### 2026-01-28 08:09 - Heat - n

Document concurrent session safety invariant for rbf_build

### 2026-01-28 07:58 - ₢APAAJ - n

Delete local build infrastructure (recipes and tabtargets)

### 2026-01-28 07:58 - ₢APAAJ - n

Create GAR vessels for sentry and bottle; update nameplates to use new monikers

### 2026-01-28 07:53 - ₢APAAI - n

Add rbrv-vessel-role-attribute itch for image role branding

### 2026-01-28 07:40 - ₢APAAL - W

pace complete

### 2026-01-28 07:40 - ₢APAAL - n

Update container image references to use GAR registry paths

### 2026-01-28 07:39 - ₢APAAL - F

Executing bridled pace via haiku agent

### 2026-01-28 07:39 - ₢APAAI - W

pace complete

### 2026-01-28 07:38 - Heat - T

build-nsproto-images

### 2026-01-28 07:36 - ₢APAAI - n

Rename RBRN_*_REPO_PATH variables to RBRN_*_MONIKER; delete legacy .mk nameplate files

### 2026-01-28 07:36 - Heat - r

moved APAAL after APAAI

### 2026-01-28 07:35 - ₢APAAL - B

tally | implement-gar-image-resolution

### 2026-01-28 07:35 - Heat - T

implement-gar-image-resolution

### 2026-01-28 07:35 - ₢APAAI - F

Executing bridled pace via haiku agent

### 2026-01-28 07:34 - ₢APAAI - B

tally | rename-repo-path-to-moniker

### 2026-01-28 07:34 - Heat - T

rename-repo-path-to-moniker

### 2026-01-28 07:32 - Heat - S

implement-gar-image-resolution

### 2026-01-28 07:32 - Heat - T

nameplate-gar-resolution

### 2026-01-28 07:20 - ₢APAAI - A

Add zrbob_resolve_image helper, update z_image construction at lines 166, 224, 275

### 2026-01-28 07:20 - ₢APAAK - W

pace complete

### 2026-01-28 07:19 - ₢APAAK - n

Rename rbw-r to rbw-ir for parity with rbw-il image list

### 2026-01-28 07:15 - ₢APAAK - n

Add coordinator route for rbf_retrieve, enabling tt/rbw-r.RetrieveImage.sh tabtarget

### 2026-01-28 07:12 - ₢APAAK - A

Run rbf_list, pick image, retrieve to local Docker, verify

### 2026-01-28 07:10 - Heat - S

test-image-retrieve

### 2026-01-28 07:09 - Heat - S

build-nsproto-images

### 2026-01-28 07:08 - Heat - r

moved APAAI after APAAG

### 2026-01-28 07:07 - ₢APAAG - W

pace complete

### 2026-01-28 07:07 - ₢APAAG - n

Improve image list output to show full references with meta companion indicators

### 2026-01-28 07:07 - ₢APAAG - L

sonnet landed

### 2026-01-28 07:05 - ₢APAAG - F

Executing bridled pace via sonnet agent

### 2026-01-28 07:04 - ₢APAAG - B

tally | improve-image-list-output

### 2026-01-28 07:04 - Heat - T

improve-image-list-output

### 2026-01-28 07:02 - Heat - T

investigate-image-naming

### 2026-01-28 07:02 - Heat - T

investigate-image-naming

### 2026-01-28 07:01 - Heat - S

nameplate-gar-resolution

### 2026-01-28 06:52 - ₢APAAE - W

pace complete

### 2026-01-28 06:52 - ₢APAAE - n

Simplify build context: remove cloudbuild.yaml copy, inline build steps in API request

### 2026-01-28 06:52 - Heat - S

pin-gcb-tool-versions

### 2026-01-28 06:46 - ₢APAAE - n

Fix step definition parsing: use alpine without tag

### 2026-01-28 06:46 - ₢APAAE - n

Fix build pipeline: hardcode syft image, use alpine for metadata assembly

### 2026-01-28 06:38 - ₢APAAE - A

Investigate foundry tabtargets, trigger cloud build, verify in Artifact Registry

### 2026-01-28 06:37 - Heat - n

Add paddock build requirements for RBW heats; update notch to synthesize intent from conversation

### 2026-01-28 06:31 - ₢APAAA - W

pace complete

### 2026-01-28 06:31 - ₢APAAA - n

Add release 1011 to vovr_registry.json

### 2026-01-28 06:30 - Heat - S

investigate-image-naming

### 2026-01-28 06:24 - ₢APAAA - A

Verify depot access via ImageList, then start/stop nsproto container

### 2026-01-28 06:23 - Heat - s

260128-0623 session

### 2026-01-27 11:58 - Heat - s

260127-1158 session

### 2026-01-25 14:18 - Heat - s

260125-1417 session session

### 2026-01-25 14:17 - ₢APAAF - W

pace complete

### 2026-01-25 14:17 - ₢APAAF - n

Remove unused GCB tool image refs (jq, syft) from regime config

### 2026-01-25 14:14 - ₢APAAF - F

Executing bridled pace via haiku agent

### 2026-01-25 14:13 - ₢APAAF - B

tally | remove-gcb-jq-image-ref

### 2026-01-25 14:13 - Heat - T

remove-gcb-jq-image-ref

### 2026-01-25 14:12 - ₢APAAF - A

Remove jq/syft image refs: 4 files, delete definitions/validations/substitutions

### 2026-01-25 14:11 - Heat - r

moved APAAF to first

### 2026-01-25 14:10 - Heat - S

remove-gcb-jq-image-ref

### 2026-01-25 14:01 - ₢APAAA - A

List images, start/stop nsproto containers

### 2026-01-25 14:00 - Heat - T

verify-depot-and-docker

### 2026-01-25 14:00 - Heat - r

moved APAAE after APAAA

### 2026-01-25 13:50 - Heat - f

racing

### 2026-01-25 08:37 - Heat - S

verify-cloud-build-pipeline

### 2026-01-25 08:37 - Heat - S

run-pluml-test-suite

### 2026-01-25 08:37 - Heat - S

run-srjcl-test-suite

### 2026-01-25 08:37 - Heat - S

run-nsproto-test-suite

### 2026-01-25 08:37 - Heat - S

verify-depot-and-docker

### 2026-01-25 08:36 - Heat - N

rbw-rekindle-bottle-focus

