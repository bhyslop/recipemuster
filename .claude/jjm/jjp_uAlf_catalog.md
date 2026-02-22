# Regime Second-Pass Transformation Catalog

Reference: Before-anchor commit `57b1ff99`.
Exemplars: RBRR (singleton), RBRN (manifold), RBRV (manifold, applied via ₢AfAAI).

---

## Part 1: Transformation Recipes

### T1: CLI Restructure — case dispatch to buc_execute + furnish

**Before:** CLI file has `case "${1:-}" in validate|render) ...` at bottom with manual dispatch.

**After:** CLI file uses `buc_execute prefix_ "Label" furnish_func "$@"` at bottom. A `zXXXX_furnish()` function handles all sourcing and kindling. Commands are plain functions matching `prefix_*`.

**Details:**
- Remove `ZXXX_CLI_SCRIPT_DIR` variable — use `BURD_BUK_DIR` and `BURD_TOOLS_DIR` instead
- Remove all `source` lines from file top (except `source "${BURD_BUK_DIR}/buc_command.sh"`)
- Move all sourcing into `furnish()`, which receives the command name as `$1`
- Remove manual `case` dispatch at bottom
- Add `buc_execute prefix_ "Label" furnish_func "$@"` as final statement
- All public command functions get `buc_doc_brief` + `buc_doc_shown || return 0` preamble

**Singleton example (RBRR):**
```bash
source "${BURD_BUK_DIR}/buc_command.sh"

zrbrr_furnish() {
  zbuv_kindle; zburd_kindle; zrbcc_kindle
  zrbrr_kindle; zrbrr_enforce
  zbupr_kindle
}

buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"
```

**Manifold example (RBRN):**
- Furnish receives command name as `$1` for differential loading
- Light deps (validate, render, list) always loaded
- Heavy deps (survey, audit) loaded conditionally via `case "${z_command}" in`
- If `BUZ_FOLIO` is set, load and kindle the specific instance

### T2: Eliminate _load/_load_moniker — inline source + kindle in furnish

**Before:** Regime module has `rbrr_load()` or `rbrn_load_moniker()` public functions that source + kindle + validate. CLI calls these.

**After:** Furnish does explicit `source "${file}"` + `zXXXX_kindle` + `zXXXX_enforce` inline. No _load convenience functions. Consumers that need the regime (e.g., rbob_cli.sh) also inline source+kindle+enforce in their own furnish.

**Rationale:** Makes sourcing hierarchy explicit. No hidden file-loading in function calls.

### T3: buv_ Enrollment — kindle registers variables, enforce validates

**Before:** `zrbrr_kindle()` sets defaults, detects unexpected vars via `compgen -v` loop, builds `ZRBRR_ROLLUP` string. `zrbrr_validate_fields()` calls `buv_env_TYPE` per variable.

**After:** `zrbrr_kindle()` sets defaults then calls enrollment functions. `zrbrr_enforce()` calls `buv_vet SCOPE` plus any custom post-vet checks.

**Enrollment sequence in kindle (after defaults):**
```bash
buv_regime_enroll SCOPE         # Set scope (required first)
buv_group_enroll  "Group Name"  # Start visual group
buv_TYPE_enroll   VARNAME  P1  P2  "description"  # Register variable
buv_scope_sentinel SCOPE PREFIX_   # Detect unexpected vars
```

**Available enrollment types:**
| Function | Params | Notes |
|----------|--------|-------|
| `buv_string_enroll` | VAR min max desc | min=0 means optional |
| `buv_xname_enroll` | VAR min max desc | `^[a-zA-Z][a-zA-Z0-9_-]*$` |
| `buv_gname_enroll` | VAR min max desc | `^[a-z][a-z0-9-]*[a-z0-9]$` |
| `buv_fqin_enroll` | VAR min max desc | Fully-qualified image name |
| `buv_bool_enroll` | VAR desc | 0 or 1 |
| `buv_enum_enroll` | VAR desc val1 val2... | Whitespace-separated choices |
| `buv_decimal_enroll` | VAR min max desc | Integer range |
| `buv_odref_enroll` | VAR desc | OCI digest reference |
| `buv_ipv4_enroll` | VAR desc | Dotted-quad IPv4 |
| `buv_port_enroll` | VAR desc | 1-65535 |
| `buv_list_string_enroll` | VAR min max desc | Space-separated list |
| `buv_list_ipv4_enroll` | VAR desc | Space-separated IPv4s |
| `buv_list_gname_enroll` | VAR min max desc | Space-separated gnames |
| `buv_list_cidr_enroll` | VAR desc | Space-separated CIDRs |
| `buv_list_domain_enroll` | VAR desc | Space-separated domains |

### T4: Gated Enrollment — conditional validation for mode-dependent variables

**Before:** `if [[ ${MODE} == value ]]; then buv_env_TYPE ...; fi` scattered in validate function.

**After:** `buv_gate_enroll GATE_VAR GATE_VAL` sets gate context. All subsequent enrolls in the current group are gated — skipped during vet/render when gate doesn't match.

**Example (RBRV vessel mode):**
```bash
buv_group_enroll "Binding Configuration"
buv_gate_enroll   RBRV_VESSEL_MODE  bind
buv_fqin_enroll   RBRV_BIND_IMAGE  1  512  "Source image"

buv_group_enroll "Conjuring Configuration"
buv_gate_enroll   RBRV_VESSEL_MODE  conjure
buv_string_enroll RBRV_CONJURE_DOCKERFILE  1  512  "Dockerfile path"
```

Gate context resets on each `buv_group_enroll`. Group registry carries gate for render section headers.

### T5: Replace validate/render commands with buv_report/buv_render

**Before:** `rbrr_validate()` takes file arg, sources file, calls `zrbrr_validate_fields()` which dies on first error. `rbrr_render()` takes file arg, manually constructs `rbcr_section_*` display.

**After:** `rbrr_validate()` calls `buv_report SCOPE "Label"` — rich per-variable PASS/FAIL/SKIP display. `rbrr_render()` calls `buv_render SCOPE "Label"` — walks enrollment rolls grouped by group via bupr_ presentation module. Neither takes file args — file is sourced in furnish.

**Singleton:** No file arg needed; furnish loads the one file.
**Manifold:** `BUZ_FOLIO` identifies the instance; furnish loads it. If `BUZ_FOLIO` empty, show list and die.

### T6: Unexpected variable detection via buv_scope_sentinel

**Before:** Each kindle manually builds `z_known` string, loops `compgen -v PREFIX_`, accumulates `ZXXX_UNEXPECTED` array. Validate dies if array non-empty.

**After:** Single call at end of kindle: `buv_scope_sentinel SCOPE PREFIX_`. Internally does the same compgen-vs-enrolled check and dies immediately if unexpected vars found.

**Removes:** `ZXXX_UNEXPECTED` array, `z_known` string, manual compgen loop from each regime module.

### T7: Rollup elimination — buv_docker_env replaces manual rollup

**Before:** Each kindle builds `ZXXX_ROLLUP` string by concatenating `VAR='${VAR}'` for every variable. Used for passing to scripts/containers.

**After:** `buv_docker_env SCOPE ARRAY_VAR` populates an array with `-e VAR=val` pairs derived from enrollment rolls. Manual rollup string deleted.

**Example:** `buv_docker_env RBRN ZRBRN_DOCKER_ENV` replaces 20 lines of manual rollup.

**Note:** RBRR still has a small manual `ZRBRR_DOCKER_ENV` for the one var it actually injects (DNS_SERVER). This is deliberate — not all enrolled vars need docker injection.

### T8: BCG violations — pattern/operator corrections

**8a: `[[ == ]]` to `test` or `[[ =~ ]]`**
- Replace `[[ ${X} == value ]]` with `test "${X}" = "value"` (string equality)
- Replace `[[ ${X} == pattern ]]` with `[[ "${X}" =~ pattern ]]` (regex match)
- Already applied to RBRR (commit 3b745da8), RBRN, RBRV

**8b: `grep -qE` to `[[ =~ ]]`**
- Replace `echo "${X}" | grep -qE 'pattern'` with `[[ "${X}" =~ pattern ]]`
- Applicable to RBRP (3 sites), RBRA (2 sites)

**8c: Unquoted array count**
- Replace `${#ZXXX_UNEXPECTED[@]}` with `"${#ZXXX_UNEXPECTED[@]}"` (quotes)
- Moot after T6 (scope_sentinel eliminates the array entirely)

**8d: `for z_x in $()` to array iteration**
- Replace `for z_m in $(rbrn_list)` with array: `local z_files=(...); for z_i in "${!z_files[@]}"`
- Replaces word-splitting pipe with direct glob expansion

### T9: buc_doc_* preamble on all public command functions

**Before:** Functions have no self-documentation.

**After:** Every public command function starts with:
```bash
func_name() {
  buc_doc_brief "One-line description"
  buc_doc_shown || return 0
  # ... actual implementation
}
```

Also for functions with env requirements:
```bash
func_name() {
  buc_doc_env "VAR_NAME" "Description"
  buc_doc_brief "One-line description"
  buc_doc_shown || return 0
}
```

### T10: buz channel declaration for manifold regime dispatch

**Before:** Workbench has explicit `case` arms that translate `BURD_TOKEN_3` (imprint) to `RBOB_MONIKER` for bottle operations. Zipper registered commands without channel awareness.

**After:** `buz_enroll` takes optional 5th arg: channel (`""`, `"imprint"`, `"param1"`).
- `"imprint"` → `BUZ_FOLIO` set from `BURD_TOKEN_3`
- `"param1"` → `BUZ_FOLIO` set from first positional arg (shifted)
- `buz_exec_lookup` decodes channel and injects `BUZ_FOLIO` into exec environment

**Workbench collapse:** All manual `case` dispatch arms removed. Workbench is now just qualification gate + `buz_exec_lookup`.

**For regime CLIs:** Use `BUZ_FOLIO` instead of `RBOB_MONIKER` or file-path args. Furnish reads `BUZ_FOLIO` to locate and load the specific instance.

### T11: `_list` to `_list_capture` — subshell-safe list functions

**Before:** `rbrn_list()` calls `echo` per item inside a loop, intended for pipe consumption (`for z in $(rbrn_list)`).

**After:** `rbrn_list_capture()` accumulates space-separated string, echoes once. Returns 1 if empty. Consumers call `z_result=$(xxx_list_capture)` then iterate `for z in ${z_result}`.

**BCG rationale:** Avoids subshell-per-item pattern, avoids word-splitting on pipe output.

### T12: buc_execute furnish receives command name

**Before (pre-exemplar):** `buc_execute` called furnish with no args.

**After:** `buc_execute` passes the resolved command name as `$1` to furnish. This enables differential furnish — loading heavy dependencies only for commands that need them (T1 manifold case).

**Change in buc_command.sh:** Line `[ -n "${env_func}" ] && "${env_func}"` became `[ -n "${env_func}" ] && "${env_func}" "${command}"`.

### T13: bupr_PresentationRegime replaces rbcr_render

**Before:** Render functions use `rbcr_section_begin`, `rbcr_section_item`, `rbcr_section_end` from `rbcr_render.sh` (RBW-specific rendering).

**After:** `buv_render` delegates to `bupr_section_begin`, `bupr_section_item`, `bupr_section_end` from `bupr_PresentationRegime.sh` (BUK-level, reusable). Render is fully data-driven from enrollment rolls. Manual per-variable `rbcr_section_item` calls eliminated.

---

## Part 2: Per-Regime Transformation Briefs

### RBRP — Payor Regime [Singleton]

**Current state:** `rbrp_regime.sh` (75 lines), `rbrp_cli.sh` (112 lines). Small regime — 4 variables, no rollup, no docker_env, no UNEXPECTED array.

**Files:** `Tools/rbw/rbrp_regime.sh`, `Tools/rbw/rbrp_cli.sh`

**What exists now:**
- CLI uses `ZRBRP_CLI_SCRIPT_DIR` + manual case dispatch (validate|render)
- Regime has `zrbrp_kindle()` that validates inline (no separate validate_fields)
- Has `rbrp_load()` public function (source + kindle)
- Render uses `rbcr_section_*` with 3 sections (Identity, Billing, OAuth)
- No `buc_doc_*` calls anywhere
- Depends on RBGC for `RBGC_GLOBAL_PAYOR_REGEX` constant

**Variables (4 total, 1 group):**
| Variable | Type | Req | Current validation |
|----------|------|-----|-------------------|
| `RBRP_PAYOR_PROJECT_ID` | string | req | Non-empty + `grep -qE "${RBGC_GLOBAL_PAYOR_REGEX}"` |
| `RBRP_BILLING_ACCOUNT_ID` | string | opt | `grep -qE '^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$'` |
| `RBRP_OAUTH_CLIENT_ID` | string | opt | `grep -qE '\.apps\.googleusercontent\.com$'` |
| `RBRP_OAUTH_REDIRECT_URI` | string | opt | Not currently validated (exists in render only) |

**Specific transformation notes:**

- **T1 (CLI restructure):** Standard singleton pattern. Source only `buc_command.sh` at top. Furnish sources buv, burd, rbcc, rbgc, rbrp_regime, bupr. No differential furnish needed (only 2 commands).
- **T2 (eliminate _load):** Delete `rbrp_load()` from regime. Furnish does `source "${RBCC_rbrp_file}"` + `zrbrp_kindle` + `zrbrp_enforce`.
- **T3 (enrollment):** 4 variables. Suggested groups: "Payor Project Identity" (RBRP_PAYOR_PROJECT_ID), "Billing Configuration" (RBRP_BILLING_ACCOUNT_ID), "OAuth Configuration" (RBRP_OAUTH_CLIENT_ID, RBRP_OAUTH_REDIRECT_URI). Use `buv_string_enroll` with min=0 for the optional ones.
- **T4 (gating):** Not needed. Optionals use min=0, not mode-gates.
- **T5 (buv_report/render):** Direct replacement. Validate calls `buv_report RBRP "Payor Regime"`. Render calls `buv_render RBRP "RBRP - Recipe Bottle Regime Payor"`.
- **T6 (scope_sentinel):** No UNEXPECTED array currently exists — this is new. Add `buv_scope_sentinel RBRP RBRP_` at end of kindle.
- **T7 (rollup):** No rollup exists. Nothing to delete.
- **T8b (grep -qE):** Three `printf | grep -qE` sites in `zrbrp_kindle()`. These become custom checks in `zrbrp_enforce()` using `[[ =~ ]]`. The RBGC_GLOBAL_PAYOR_REGEX check needs RBGC kindled before enforce — furnish must kindle rbgc before rbrp.
- **T8b detail — custom enforce checks:**
  ```bash
  zrbrp_enforce() {
    zrbrp_sentinel
    buv_vet RBRP
    # Custom format checks that go beyond buv_ type system
    zrbgc_sentinel
    [[ "${RBRP_PAYOR_PROJECT_ID}" =~ ${RBGC_GLOBAL_PAYOR_REGEX} ]] \
      || buc_die "RBRP_PAYOR_PROJECT_ID must match payor project pattern"
    if test -n "${RBRP_BILLING_ACCOUNT_ID}"; then
      [[ "${RBRP_BILLING_ACCOUNT_ID}" =~ ^[A-Z0-9]{6}-[A-Z0-9]{6}-[A-Z0-9]{6}$ ]] \
        || buc_die "RBRP_BILLING_ACCOUNT_ID must be 3 hex-6 groups (XXXXXX-XXXXXX-XXXXXX)"
    fi
    if test -n "${RBRP_OAUTH_CLIENT_ID}"; then
      [[ "${RBRP_OAUTH_CLIENT_ID}" =~ \.apps\.googleusercontent\.com$ ]] \
        || buc_die "RBRP_OAUTH_CLIENT_ID must end with .apps.googleusercontent.com"
    fi
  }
  ```
- **T9 (buc_doc_*):** Add to both commands (rbrp_validate, rbrp_render).
- **T10-T11:** Not applicable (singleton, no list).
- **T13 (bupr):** CLI currently sources `rbcr_render.sh` and uses `rbcr_section_*`. Delete those; `buv_render` handles it.

**Complexity:** Low. 4 variables, no rollup, no docker_env, no gating. The only wrinkle is the custom RBGC regex check in enforce.

**RBRP render currently shows a 4th variable (RBRP_OAUTH_REDIRECT_URI) that is NOT validated.** The enrollment must include it (buv_string_enroll with min=0) even though the current validator ignores it.

---

### RBRE — ECR Regime [Singleton]

**Current state:** `rbre_regime.sh` (93 lines), `rbre_cli.sh` (106 lines). Medium regime — 7 variables, has UNEXPECTED array and validate_fields.

**Files:** `Tools/rbw/rbre_regime.sh`, `Tools/rbw/rbre_cli.sh`

**What exists now:**
- CLI uses `ZRBRE_CLI_SCRIPT_DIR` + manual case dispatch (validate|render)
- Regime has `zrbre_kindle()` with defaults, UNEXPECTED detection, and `zrbre_validate_fields()`
- Has `rbre_load()` public function
- Render uses `rbcr_section_*` with 2 sections (ECR Identity, ECR Access)
- No `buc_doc_*` calls
- No rollup, no docker_env

**Variables (7 total):**
| Variable | Type | Req | Notes |
|----------|------|-----|-------|
| `RBRE_AWS_ACCOUNT_ID` | string | req | 12-digit AWS account |
| `RBRE_AWS_REGION` | string | req | e.g., us-east-1 |
| `RBRE_ECR_REGISTRY` | string | req | Full registry URL |
| `RBRE_ECR_REPOSITORY_PREFIX` | string | req | Repository prefix |
| `RBRE_AWS_ACCESS_KEY_ID` | string | req | |
| `RBRE_AWS_SECRET_ACCESS_KEY` | string | req | |
| `RBRE_AWS_SESSION_TOKEN` | string | opt | Optional STS token |

**Specific transformation notes:**

- **T1 (CLI restructure):** Standard singleton. Furnish sources buv, burd, rbcc, rbre_regime, bupr. Sources `"${RBCC_rbre_file}"` to load the env file.
- **T2 (eliminate _load):** Delete `rbre_load()`. Furnish inlines source + kindle + enforce.
- **T3 (enrollment):** 7 variables. Suggested groups: "ECR Identity" (RBRE_AWS_ACCOUNT_ID, RBRE_AWS_REGION, RBRE_ECR_REGISTRY, RBRE_ECR_REPOSITORY_PREFIX), "ECR Access" (RBRE_AWS_ACCESS_KEY_ID, RBRE_AWS_SECRET_ACCESS_KEY, RBRE_AWS_SESSION_TOKEN). All `buv_string_enroll`; session token with min=0.
- **T4 (gating):** Not needed. SESSION_TOKEN is optional (min=0), not gated.
- **T6 (scope_sentinel):** Has UNEXPECTED array currently. Replace with `buv_scope_sentinel RBRE RBRE_`.
- **T7 (rollup):** No rollup. Nothing to delete.
- **T8:** No `grep -qE` or `[[ == ]]` issues noted. Current validation uses `buv_env_string` only.
- **T9 (buc_doc_*):** Add to both commands.

**Complexity:** Low. Flat variable list, no custom format checks, no gating, no rollup. Straightforward mechanical transformation.

**Caution:** RBRE contains AWS secrets (ACCESS_KEY, SECRET_KEY). The render function should still display these. Enrollment doesn't change security posture — these are already shown by the current render.

---

### RBRS — Station Regime [Singleton]

**Current state:** `rbrs_regime.sh` (45 lines), `rbrs_cli.sh` (98 lines). Tiny regime — 3 variables.

**Files:** `Tools/rbw/rbrs_regime.sh`, `Tools/rbw/rbrs_cli.sh`

**What exists now:**
- CLI uses `ZRBRS_CLI_SCRIPT_DIR` + manual case dispatch
- Regime has `zrbrs_kindle()` — but validation is INSIDE kindle (not separate validate_fields)
- No `_load()` function
- Has UNEXPECTED array detection
- Render uses `rbcr_section_*` with 1 section (Station Paths)
- Default file is `../station-files/rbrs.env` (hardcoded path, NOT from RBCC)
- No `buc_doc_*` calls

**Variables (3 total):**
| Variable | Type | Req | Notes |
|----------|------|-----|-------|
| `RBRS_PODMAN_ROOT_DIR` | string | req | 1-64 chars |
| `RBRS_VMIMAGE_CACHE_DIR` | string | req | 1-64 chars |
| `RBRS_VM_PLATFORM` | string | req | 1-64 chars |

**Specific transformation notes:**

- **T1 (CLI restructure):** Standard singleton. **Special:** CLI currently hardcodes file path as `../station-files/rbrs.env`. This should use `RBCC_rbrs_file` (verify this constant exists in rbcc_Constants.sh). If not, the file path wiring needs to be confirmed.
- **T2 (eliminate _load):** No _load function exists. Furnish just sources the env file + kindle + enforce.
- **T3 (enrollment):** Only 3 variables. Single group "Station Paths". All `buv_string_enroll` with min=1, max=64.
- **T4 (gating):** Not needed.
- **T5:** `buv_report` / `buv_render` — trivial with 3 vars.
- **T6 (scope_sentinel):** Has UNEXPECTED array. Replace with `buv_scope_sentinel RBRS RBRS_`.
- **T7-T8:** No rollup, no grep, no `[[ == ]]`.
- **T3 note — validation is currently inside kindle:** The current `zrbrs_kindle()` calls `buv_env_string` directly, combining kindle and validate. Must split: kindle sets defaults + enrolls, enforce calls `buv_vet`.

**Complexity:** Very low. Smallest regime. The only question mark is the file path sourcing (RBCC constant vs hardcoded path).

---

### BURC — Configuration Regime [Singleton, BUK domain]

**Current state:** `burc_regime.sh` (90 lines), `burc_cli.sh` (133 lines). Medium regime — 11 variables (9 validated + 2 derived).

**Files:** `Tools/buk/burc_regime.sh`, `Tools/buk/burc_cli.sh`

**What exists now:**
- CLI uses `ZBURC_CLI_SCRIPT_DIR` + manual case dispatch
- Regime has `zburc_kindle()` with defaults, UNEXPECTED detection (compgen-based), and separate `zburc_validate_fields()`
- No `_load()` function (BURC is sourced from burc.env by burd at dispatch time)
- Has UNEXPECTED array
- CLI renders using `rbcr_section_*` (via bupr) with 5 groups
- Kindle exports 3 vars: BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR
- No `buc_doc_*` calls

**Variables (11 total, 3 exported):**
| Variable | Type | Req | Notes |
|----------|------|-----|-------|
| `BURC_STATION_FILE` | string | req | 1-512 |
| `BURC_TABTARGET_DIR` | string | req | 1-128, exported |
| `BURC_TABTARGET_DELIMITER` | string | req | 1-1 (single char) |
| `BURC_TOOLS_DIR` | string | req | 1-128, exported |
| `BURC_PROJECT_ROOT` | string | req | 1-512 |
| `BURC_MANAGED_KITS` | string | req | 1-512 |
| `BURC_TEMP_ROOT_DIR` | string | req | 1-512 |
| `BURC_OUTPUT_ROOT_DIR` | string | req | 1-512 |
| `BURC_LOG_LAST` | xname | req | 1-64 |
| `BURC_LOG_EXT` | xname | req | 1-16 |
| `BURC_BUK_DIR` | derived | n/a | `${BURC_TOOLS_DIR}/buk` (line 60) |

**Specific transformation notes:**

- **T1 (CLI restructure):** BUK-domain CLI. Source `"${BURD_BUK_DIR}/buc_command.sh"` at top. Furnish sources buv, burd, burc_regime, bupr. **Special:** BURC is the foundational configuration regime — it must NOT depend on RBCC or any RBW-domain module. Furnish kindles buv, burd, burc (from BURD environment), bupr.
- **T1 BUK-domain note:** BURC CLI currently sources `${ZBURC_CLI_SCRIPT_DIR}/burc_regime.sh` — after restructure this becomes `source "${BURD_BUK_DIR}/burc_regime.sh"` inside furnish. The BURC file itself is already sourced by burd at dispatch time, so `zburc_kindle` can fire immediately.
- **T2 (eliminate _load):** No _load exists. BURC values are already in environment from burd dispatch. Furnish just calls `zburc_kindle` + `zburc_enforce`.
- **T3 (enrollment):** 11 variables. Suggested groups: "Station Reference" (BURC_STATION_FILE), "Tabtarget Infrastructure" (BURC_TABTARGET_DIR, BURC_TABTARGET_DELIMITER), "Project Structure" (BURC_TOOLS_DIR, BURC_PROJECT_ROOT, BURC_MANAGED_KITS), "Build Output" (BURC_TEMP_ROOT_DIR, BURC_OUTPUT_ROOT_DIR), "Logging" (BURC_LOG_LAST, BURC_LOG_EXT).
- **T3 special — derived variable:** `BURC_BUK_DIR="${BURC_TOOLS_DIR}/buk"` is set in kindle but not enrolled (it's derived, not user-configured). Keep this assignment in kindle after enrollment but before scope_sentinel. Must NOT be enrolled (it's BURC_ prefixed but derived — scope_sentinel would flag it). **Solution:** Compute BURC_BUK_DIR before the scope_sentinel call but don't enroll it. Then add it to the known set by enrolling it as a string with min=0 and the value already set, OR handle it by renaming to ZBURC_BUK_DIR (private). **Decision needed from human.**
- **T4 (gating):** Not needed. All variables required.
- **T6 (scope_sentinel):** Has UNEXPECTED array. Replace. See BURC_BUK_DIR note above.
- **T7:** No rollup. But the export lines (BURC_TABTARGET_DIR, BURC_TOOLS_DIR, BURC_BUK_DIR) must be preserved in kindle — enrollment doesn't handle export.
- **T8:** No grep or `[[ == ]]` issues.

**Complexity:** Medium. The BURC_BUK_DIR derived-variable + export pattern needs a design decision. The exports must survive the restructure. Everything else is mechanical.

---

### BURS — Station Regime [Singleton, BUK domain]

**Current state:** `burs_regime.sh` (66 lines), `burs_cli.sh` (108 lines). Tiny regime — 1 variable.

**Files:** `Tools/buk/burs_regime.sh`, `Tools/buk/burs_cli.sh`

**What exists now:**
- CLI uses `ZBURS_CLI_SCRIPT_DIR` + manual case dispatch
- Regime has `zburs_kindle()` with UNEXPECTED detection and separate `zburs_validate_fields()`
- No `_load()` function
- Has UNEXPECTED array
- CLI renders with 1 section (Developer Logging)
- No `buc_doc_*` calls
- BURS file sourced by burd dispatch (same as BURC)

**Variables (1 total):**
| Variable | Type | Req | Notes |
|----------|------|-----|-------|
| `BURS_LOG_DIR` | string | req | 1-512 |

**Specific transformation notes:**

- **T1 (CLI restructure):** BUK-domain. Furnish sources buv, burd, burs_regime, bupr.
- **T2:** No _load. Values already in environment from burd.
- **T3 (enrollment):** Single variable, single group "Developer Logging". `buv_string_enroll BURS_LOG_DIR 1 512 "Directory for developer log output"`.
- **T6 (scope_sentinel):** Has UNEXPECTED array. Replace with `buv_scope_sentinel BURS BURS_`.
- **Everything else:** Nothing to gate, no rollup, no grep, no `[[ == ]]`.

**Complexity:** Very low. Smallest possible regime. Pure mechanical.

---

### BURE — Environment Regime [Singleton, BUK domain, AMBIENT]

**Current state:** `bure_regime.sh` (73 lines), `bure_cli.sh` (94 lines). Small regime — 3 variables, but uses `buv_opt_enum` and `buv_env_enum` (not just string).

**Files:** `Tools/buk/bure_regime.sh`, `Tools/buk/bure_cli.sh`

**What exists now:**
- CLI uses `ZBURE_CLI_SCRIPT_DIR` + manual case dispatch
- Regime has `zbure_kindle()` with UNEXPECTED detection and separate `zbure_validate_fields()`
- No `_load()` function — BURE is **ambient** (values from caller environment, not file-sourced)
- Has UNEXPECTED array
- CLI render header says "ambient (caller environment)" instead of showing a file
- No `buc_doc_*` calls

**Variables (3 total):**
| Variable | Type | Req | Current validation |
|----------|------|-----|--------------------|
| `BURE_COUNTDOWN` | enum | opt | `buv_opt_enum BURE_COUNTDOWN skip` |
| `BURE_VERBOSE` | enum | req | `buv_env_enum BURE_VERBOSE 0 1 2 3` |
| `BURE_COLOR` | string | req | `buv_env_string BURE_COLOR 1 4` |

**Specific transformation notes:**

- **T1 (CLI restructure):** BUK-domain ambient. Furnish sources buv, burd, bure_regime, bupr. **Special:** No env file to source — BURE variables come from caller environment. Furnish just calls `zbure_kindle` + `zbure_enforce`.
- **T2:** No _load. Ambient regime — nothing to source.
- **T3 (enrollment):**
  - `buv_enum_enroll BURE_VERBOSE "Verbosity level" 0 1 2 3` (required enum)
  - `buv_string_enroll BURE_COLOR 1 4 "Color mode"` — OR convert to `buv_enum_enroll BURE_COLOR "Color mode" auto on off` if those are the valid values. **Check current usage.**
  - `BURE_COUNTDOWN` is optional enum — currently uses `buv_opt_enum` which has no enrollment equivalent. **Design choice:** Use `buv_string_enroll BURE_COUNTDOWN 0 4 "Countdown override"` (min=0 makes it optional) and add custom enforce check `test -z "${BURE_COUNTDOWN}" || test "${BURE_COUNTDOWN}" = "skip"`.
- **T3 special — buv_opt_enum vs enrollment:** The enrollment system doesn't have a direct "optional enum" type. Options: (a) enroll as string with min=0 + custom enforce, or (b) enroll as enum with a gate on non-empty. Approach (a) is simpler.
- **T6 (scope_sentinel):** Has UNEXPECTED array. Replace.
- **T7-T8:** No rollup, no grep, no `[[ == ]]`.

**Complexity:** Low, but needs a small design decision on how to handle `buv_opt_enum` in the enrollment system. This is the only regime that uses it.

---

### RBRA — Authentication/Credential Regime [Manifold: governor, retriever, director]

**Current state:** `rbra_regime.sh` (92 lines), `rbra_cli.sh` (189 lines). Medium regime — 4 variables, manifold across 3 roles. Has rollup with REDACTED key. Has `grep -qE` for custom format validation.

**Files:** `Tools/rbw/rbra_regime.sh`, `Tools/rbw/rbra_cli.sh`

**What exists now:**
- CLI uses `ZRBRA_CLI_SCRIPT_DIR` + manual case dispatch (validate|render|list)
- Regime has `zrbra_kindle()` with UNEXPECTED array and rollup, separate `zrbra_validate_fields()`
- No `rbra_load()` — documented as intentionally absent (manifold, loaded per-role)
- Has **ZRBRA_ROLLUP** with `RBRA_PRIVATE_KEY='[REDACTED]'` — security-sensitive
- Has UNEXPECTED array
- CLI has `zrbra_cli_resolve_role()` mapping role names to RBRR file paths
- CLI has `rbra_list()` showing all 3 role files with existence status
- 2 `grep -qE` sites and 1 `grep -q` site for format validation
- No `buc_doc_*` calls
- Render has special `RBRA_PRIVATE_KEY` display (length + redacted status)

**Variables (4 total):**
| Variable | Type | Req | Current validation |
|----------|------|-----|--------------------|
| `RBRA_CLIENT_EMAIL` | string | req | `grep -qE '\.iam\.gserviceaccount\.com$'` |
| `RBRA_PRIVATE_KEY` | string | req | `grep -q 'BEGIN'` (PEM check) |
| `RBRA_PROJECT_ID` | string | req | 1-64 chars |
| `RBRA_TOKEN_LIFETIME_SEC` | decimal | req | 300-3600 |

**Manifold mechanics:**
- 3 instances: governor, retriever, director
- Each instance file path comes from RBRR: `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`
- CLI resolve: `zrbra_cli_resolve_role("governor")` → `"${RBRR_GOVERNOR_RBRA_FILE}"`
- Not nameplate-based (unlike RBRN) — fixed set of 3 roles, not glob-discovered

**Specific transformation notes:**

- **T1 (CLI restructure):** Manifold singleton-ish. Furnish sources buv, burd, rbcc, rbrr_regime, rbra_regime, bupr. Loads RBRR for role→file mapping. If `BUZ_FOLIO` is set, resolves to file path and sources.
- **T1 manifold note:** RBRA is manifold but NOT glob-discovered like RBRN/RBRV. The 3 roles are fixed (governor, retriever, director). `BUZ_FOLIO` carries the role name. Furnish maps folio→RBRR variable→file path.
  ```bash
  zrbra_furnish() {
    # ... source and kindle buv, burd, rbcc, rbrr, rbra_regime, bupr
    source "${RBCC_rbrr_file}"
    zrbrr_kindle; zrbrr_enforce
    if test -n "${BUZ_FOLIO:-}"; then
      local z_file
      z_file=$(zrbra_resolve_role "${BUZ_FOLIO}")
      source "${z_file}" || buc_die "Failed to source RBRA: ${z_file}"
      zrbra_kindle; zrbra_enforce
    fi
  }
  ```
- **T2 (eliminate _load):** No _load exists. But the role-resolution function `zrbra_cli_resolve_role()` must be preserved (renamed/refactored as needed). It's a case-dispatch from role name to RBRR variable — belongs in regime or CLI.
- **T3 (enrollment):** 4 variables. Single group "Service Account Credentials". `buv_string_enroll RBRA_CLIENT_EMAIL 1 256`, `buv_string_enroll RBRA_PRIVATE_KEY 1 4096`, `buv_string_enroll RBRA_PROJECT_ID 1 64`, `buv_decimal_enroll RBRA_TOKEN_LIFETIME_SEC 300 3600`.
- **T4 (gating):** Not needed. All variables required for all roles.
- **T6 (scope_sentinel):** Has UNEXPECTED array. Replace.
- **T7 (rollup elimination):** Has `ZRBRA_ROLLUP` with `[REDACTED]` for private key. **This is security-sensitive.** Check if ZRBRA_ROLLUP is consumed anywhere. If not, delete. If consumed, replace with `buv_docker_env RBRA ZRBRA_DOCKER_ENV` — but that would expose the private key in docker env, which may be intentional for credential injection.
- **T8b (grep -qE):** 3 sites in `zrbra_validate_fields()`. These become custom enforce checks:
  ```bash
  zrbra_enforce() {
    zrbra_sentinel
    buv_vet RBRA
    [[ "${RBRA_CLIENT_EMAIL}" =~ \.iam\.gserviceaccount\.com$ ]] \
      || buc_die "RBRA_CLIENT_EMAIL must end with .iam.gserviceaccount.com"
    [[ "${RBRA_PRIVATE_KEY}" =~ BEGIN ]] \
      || buc_die "RBRA_PRIVATE_KEY must contain PEM BEGIN marker"
  }
  ```
- **T9 (buc_doc_*):** Add to all 3 commands (rbra_validate, rbra_render, rbra_list).
- **T10 (buz channel):** RBRA commands in zipper currently use no channel — the role is `$2` in the manual case dispatch. After restructure, RBRA should use `"param1"` channel: `buz_enroll RBZ_RENDER_AUTH "rbw-rar" "${z_mod}" "rbra_render" "param1"`. BUZ_FOLIO carries the role name.
- **T11 (list):** `rbra_list()` iterates 3 fixed roles — not glob-discovered. Keep as explicit function with `buc_doc_brief`. No `_list_capture` needed since it's not used programmatically.
- **T13 (bupr):** Render has special PRIVATE_KEY handling (shows length + `[REDACTED]`). After enrollment, `buv_render` will show the raw value. **This is a problem.** Need to either: (a) add a `redacted` type to buv enrollment, (b) keep a custom render for RBRA, or (c) accept that the private key shows in render (it's already on-disk, render is local-only). **Decision needed from human.**

**Complexity:** Medium-high. The manifold mechanics are non-standard (fixed roles, not glob-discovered). The ROLLUP with REDACTED private key and the render privacy concern need human decisions. The `grep -qE` → `[[ =~ ]]` conversions are mechanical.

---

## Part 3: Regimes NOT in scope

- **RBRG (GitHub Regime):** Removed in ₢AfAAA.
- **RBRO (OAuth Regime):** Spec exists but no validator implementation. Separate pace to create from scratch.
- **BURD (Dispatch Runtime):** Runtime-only by design, no validator.

---

## Part 4: Open Design Decisions

These need human judgment before bridled paces can execute:

1. **BURC_BUK_DIR:** Derived variable with BURC_ prefix. Enroll as string? Rename to ZBURC_BUK_DIR? Keep and add to scope_sentinel known set?

2. **BURE_COUNTDOWN:** Optional enum has no enrollment equivalent. Use string(min=0) + custom enforce? Or is this a gap worth filling in buv_ enrollment?

3. **RBRA_PRIVATE_KEY render:** buv_render will display raw value. Accept (local-only tool)? Add redacted type? Keep custom render?

4. **RBRA_ROLLUP:** Is ZRBRA_ROLLUP consumed by anything? If not, delete. If yes, what replaces the REDACTED masking?

---

## Part 5: Execution Guidance

**Model recommendation per regime:**
- BURS: haiku (1 variable, pure mechanical)
- RBRS: haiku (3 variables, pure mechanical)
- RBRE: haiku (7 variables, no custom checks)
- RBRP: sonnet (custom format checks in enforce, RBGC dependency)
- BURC: sonnet (derived variable design decision, export preservation)
- BURE: sonnet (optional enum design decision)
- RBRA: sonnet (manifold role-mapping, security concerns, channel wiring)

**Parallelization:** BURS, RBRS, RBRE are fully independent and can run in parallel. RBRP, BURC, BURE, RBRA each need the design decisions above resolved first.

**Verification per pace:**
1. Run `./tt/rbw-trg.RegimeSmoke.sh` (regime smoke suite)
2. Run regime-specific validate and render tabtargets
3. Verify no regressions in dependent operations (bottle start, qualification)
