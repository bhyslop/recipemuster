<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Bash Console Guide (BCG) - Pattern Reference

## Core Philosophy

This pattern emerged from environments where hidden failures cascade into production outages. When bash scripts manage critical infrastructure, every potential failure point needs explicit handling at its location, making post-mortem analysis straightforward.

The aesthetic of elegant bash pipelines trades debuggability for brevity. A failing command in `$(echo | sed | awk)` leaves no trace. Temp files make each transformation step visible, testable, and recoverable.

Traps and signal handlers create action-at-a-distance effects that obscure control flow. When error handling lives at the point of failure rather than in distant handlers, code review and maintenance become predictable.

Enterprise bash isn't about looking clever; it's about being boringly, reliably correct. These patterns optimize for the 3am debugging session, not the initial writing experience. Each external process spawn is a potential failure point, so bash builtins are preferred despite verbosity.

The patterns reject bash 4+ features to maintain compatibility with older enterprise systems still running bash 3.2, particularly older RHEL/CentOS or macOS deployments.

### Load-Bearing Complexity

Every element in a system — every function extraction, every pattern variant, every structural distinction — must carry weight. An element is **load-bearing** when its removal would create a gap between intent and behavior. When similar things differ, ask whether the difference is load-bearing: if yes, document why; if no, homogenize.

This is the general decision criterion that tools like Zeroes Theory instantiate. Zeroes Theory asks "how many zeroes did this add to state space?" — load-bearing complexity asks the prior question: "does this element earn its existence at all?"

Concrete examples from this codebase:

- **Four separate `rbgi_add_*_iam` functions** look like copy-paste, but each wraps a different GCP API with different failure modes. The variation is load-bearing — extracting a "generic IAM helper" would hide which API failed at 3am.
- **Three copy-pasted Secret Manager IAM blocks** are NOT load-bearing variation — same API, same failure class, same recovery. Extract a helper.
- **Not adding three-part structure to a simple `api_enable` spec definition** — the structure would add ceremony without carrying meaning. Non-load-bearing elements increase cognitive cost without increasing correctness.

Non-load-bearing elements are the silent budget overruns of a codebase. Each one is small; their aggregate is what makes systems incomprehensible.

## Module Architecture

Every module has an implementation file. CLI entry points are only present if module requires standalone execution.

```bash
«prefix»_«name».sh (implementation - REQUIRED, prefer single lowercase word, allow snake_case)
«prefix»_cli.sh (executable entry point - OPTIONAL, omit for library/utility modules)
```

### CLI as Module Gateway

**Rule**: Code outside the BCG module system must never call `z*_kindle()` directly. All external access to BCG modules goes through CLI scripts.

**Pattern**:
```
External Code → CLI script → furnish() kindles → module functions
```

**Why this matters**:
- Kindle sequences are implementation details that may change
- The CLI's `furnish()` owns the kindle graph (which modules to kindle, in what order)
- External code remains stable when module dependencies change
- CLIs provide the documented, stable API for module functionality

**Anti-pattern**:
```bash
# ❌ External script kindling directly
source "${TOOLS_DIR}/rbob_bottle.sh"
zrbob_kindle                          # VIOLATION: kindle leaked outside CLI
rbob_start
```

**Correct pattern**:
```bash
# ✅ External script delegates to CLI
exec "${TOOLS_DIR}/rbob_cli.sh" rbob_start "$@"
```

**What counts as "external code"**:
- Entry point scripts (workbenches, testbenches, coordinators)
- TabTargets and launchers
- Any dispatcher or router that invokes module functionality
- Scripts that are not themselves BCG modules

**What can call kindle**:
- A module's own CLI (in `furnish()`)
- Another CLI's `furnish()` (when CLI A depends on module B, A's furnish kindles B)
- Test harnesses that explicitly isolate module behavior

### Module-CLI Prefix Unity

**Convention**: A module and its CLI share a prefix (`rbrn_regime.sh` + `rbrn_cli.sh`).

When commands within a single CLI have different dependency weights (e.g., `validate` needs
only the regime file, `audit` needs GCP/OAuth), use **differential furnish**: the furnish
function receives the command name as `$1` and conditionally sources/kindles heavy
dependencies based on which command is being invoked.

### Regime Module Archetype

A **regime** is a sourced `.env` file containing typed configuration fields — the contract between human-authored configuration and the modules that consume it. When a module's purpose is validating a regime, it follows the regime archetype:

| Component | Location | Purpose |
|-----------|----------|---------|
| `buv_*_enroll` calls | kindle | Declare field contracts (type, range, description) |
| `buv_scope_sentinel` | kindle (last, before KINDLED) | Catch undeclared `PREFIX_*` variables |
| `z«prefix»_enforce` | regime module | `buv_vet SCOPE` + custom format checks |
| `readonly` lock | after enforce | Lock enrolled variables against mutation |
| `*_validate` command | CLI | `buv_report` — explained per-field display |
| `*_render` command | CLI | `buv_render` — diagnostic dump |

Furnish sequence: kindle → enforce → lock (ironclad gate before any command runs). After enforce succeeds, lock all enrolled `PREFIX_*` variables with `readonly` to prevent accidental mutation by downstream code. Any derived state (e.g., docker env arrays) must be built from validated values after enforce, before or during the lock step.

**Singleton** regimes source config unconditionally in furnish. **Manifold** regimes (multiple instances, e.g. nameplates) source conditionally when a folio identifies the instance.

Enrollment types and gating API: see `buv_validation.sh`.

---

## Function Patterns

### Boilerplate Functions (one per module/CLI)

| Function             | Location       | First Line                                         | Can Source?       | Can use buc_step? | Purpose                                          |
|----------------------|----------------|----------------------------------------------------|-------------------|-------------------|--------------------------------------------------|
| `z«prefix»_kindle`   | Implementation | `test -z "${Z«PREFIX»_KINDLED:-}" \|\| buc_die`    | Yes (credentials) | No (use buc_log_*)| Define all kindle constants, set module state    |
| `z«prefix»_sentinel` | Implementation | `test "${Z«PREFIX»_KINDLED:-}" = "1" \|\| buc_die` | No                | No                | Guard all other functions                        |
| `z«prefix»_enforce`  | Implementation | `z«prefix»_sentinel`                               | No                | No                | Validate sourced config (regime archetype only)  |
| `z«prefix»_furnish`  | CLI only       | `buc_doc_env...`                                   | Yes (all deps)    | No                | Document env vars, source all deps, kindle       |
| Module header        | Implementation | `test -z "${Z«PREFIX»_SOURCED:-}" \|\| buc_die`    | No                | N/A               | Prevent multiple inclusion                       |
| CLI header           | CLI            | `set -euo pipefail`                                | Yes (buc_command)  | N/A               | Source buc_command.sh only                       |

**Kindle constant**: Any variable — internal (`Z«PREFIX»_SCREAMING_NAME`) or public (`«PREFIX»_SCREAMING_NAME`) — defined exclusively within the kindle function with `readonly`. No other function — including enroll, setup, or helper functions — may assign to kindle constants. This ensures module state is fully determined at kindle time and visible in one place.

**Mutable kindle state**: A variable initialized in kindle but intentionally mutated after kindle returns (counters, accumulators, registry rolls, builder state). Uses **lowercase** `z_«prefix»_name` to visually distinguish from `readonly` kindle constants. Never apply `readonly` to mutable kindle state.

**Literal constant**: A public variable (`«PREFIX»_lower_name`) defined at module top level, immediately after the `Z«PREFIX»_SOURCED=1` guard. Literal constants must be pure string literals with **no variable expansion, no computation, and no runtime dependency**. They are available immediately after sourcing — no kindle required. Use `SCREAMING` case for the prefix (module identity) and `lower_snake` case for the name to visually distinguish from kindle constants. This enables sourcing chains where a literal constant from module A provides the path for sourcing module B's config in the furnish function.

**KINDLED must be last**: `readonly Z«PREFIX»_KINDLED=1` must be the final statement in kindle. Since sentinel checks this variable, setting it last guarantees all kindle constants, roll arrays, and enroll calls are complete before the module is considered operational. Any function calling sentinel before kindle finishes will correctly fail. The `readonly` makes re-kindling a loud error.

### Regular Functions (output via printouts/temp files)

This table defines scope: «prefix»_* is public, z«prefix»_* is internal.

| Type            | Pattern              | First Line           | Parameters | buc_step | Documentation        |
|-----------------|----------------------|----------------------|------------|----------|----------------------|
| Public          | `«prefix»_«command»` | `z«prefix»_sentinel` | `"${1:-}"` | Yes      | Required (buc_doc_*) |
| Internal helper | `z«prefix»_«name»`   | `z«prefix»_sentinel` | `"${1:-}"` | Yes      | Recommended (buc_doc_*) |

---

## File Templates

### Template 1: Shebang and Copyright

```bash
#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
```

### Dispatch-Provided Directory Variables

Three BURD variables carry directory paths across the exec boundary into CLI processes. These are derived from BURC configuration and exported by bud_dispatch.sh during zbud_setup():

| Variable | Source | Value |
|----------|--------|-------|
| `BURD_TOOLS_DIR` | `BURC_TOOLS_DIR` | Project tools root directory |
| `BURD_BUK_DIR` | Derived from `BURC_TOOLS_DIR` | BUK subdirectory (`$BURC_TOOLS_DIR/buk`) |
| `BURD_TABTARGET_DIR` | `BURC_TABTARGET_DIR` | Tabtarget directory |

These variables are available in exec'd CLI processes without requiring the CLI to re-source BURC. CLIs that need stable paths to tools or utilities should use these variables rather than reconstructing paths from `BASH_SOURCE`.

### Template 2: CLI Entry Point

BUD variables are provided by dispatch. The CLI header sources only `buc_command.sh`; all other dependencies are sourced inside the furnish function. This enables doc-mode help display without runtime env vars — furnish gates after `buc_doc_env` calls before any sourcing that might fail.

```bash
# «shebang»
# «copyright»

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

z«prefix»_furnish() {

  # Document only the BUD variables actually used
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "«PREFIX»_REGIME_FILE " "Module specific configuration file"
  buc_doc_env_done || return 0

  # Source all dependencies
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/«prefix»_«name».sh"

  # Source regime files, if any
  source "${«PREFIX»_REGIME_FILE}" || buc_die "Failed to source regime file"

  z«prefix»_kindle
  z«prefix»_enforce  # Regime archetype: ironclad gate after kindle (omit if no regime)
}

buc_execute «prefix»_ "«module_description_phrase»" z«prefix»_furnish "$@"

# eof
```

### Differential Furnish (conditional dependency sourcing)

The furnish function receives the **command name** as `$1`, enabling conditional sourcing based on which command will execute. This allows heavy dependencies (e.g., GCP client libraries, OAuth flows) to be loaded only for commands that need them.

**Pattern**: Light dependencies always loaded; heavy dependencies conditionally sourced per command.

```bash
z«prefix»_furnish() {
  local z_command="${1:-}"

  # Document environment variables
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "«PREFIX»_REGIME_FILE " "Module configuration file"
  buc_doc_env_done || return 0

  # Always source light dependencies
  source "${«PREFIX»_REGIME_FILE}" || buc_die "Failed to source regime"
  source "${BURD_BUK_DIR}/buv_validation.sh" || buc_die "Failed to source validation"

  # Conditionally source heavy dependencies based on command
  case "${z_command}" in
    «prefix»_survey|«prefix»_audit)
      # These commands need GCP and OAuth infrastructure
      source "${BURD_TOOLS_DIR}/some/heavy_dep.sh" || buc_die "Failed to source heavy deps"
      zheavy_kindle
      ;;
    «prefix»_validate|«prefix»_render)
      # These commands work with config only
      ;;
  esac

  z«prefix»_kindle
}
```

**Key points:**
- The furnish function receives the command name (e.g., "«prefix»_validate") as `$1`
- Existing furnish functions that don't use `$1` are unaffected (extra arg ignored in bash)
- Light, always-needed dependencies are sourced unconditionally
- Heavy dependencies are sourced only for the commands that need them (avoiding startup overhead)
- The `case` statement can be omitted if a module has no conditional dependencies

**Benefits:**
- Faster startup for lightweight commands (validate, render, list)
- Heavy infrastructure (GCP SDKs, OAuth flows) loaded only when actually needed
- Reduced memory footprint for simple operations
- Clear visibility of which commands depend on what infrastructure

### Template 3: Implementation Module

```bash
# «shebang»
# «copyright»

set -euo pipefail

# Multiple inclusion detection
test -z "${Z«PREFIX»_SOURCED:-}" || buc_die "Module «prefix» multiply sourced - check sourcing hierarchy"
Z«PREFIX»_SOURCED=1

# Literal constants (pure string literals, no variable expansion — available at source time)
# «PREFIX»_some_path="fixed-filename.env"
# «PREFIX»_some_prefix="prefix_"

######################################################################
# Internal Functions (z«prefix»_*)

z«prefix»_kindle() {
  test -z "${Z«PREFIX»_KINDLED:-}" || buc_die "Module «prefix» already kindled"

  # Validate only the BURD variables actually used
  buv_dir_exists "${BURD_TEMP_DIR}"   # If using temp files
  buv_dir_exists "${BURD_OUTPUT_DIR}"  # If producing outputs
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP is unset"  # If using timestamps

  # Validate module specific environment
  buv_file_exists "${«PREFIX»_REGIME_FILE}"  # If present

  # Source credentials into environment only - never write to disk
  source "${REGIME_CRED_FILE}" || buc_die "Failed to source credentials"

  # Kindle constant (public — safe to read from other modules; never a secret)
  «PREFIX»_PUBLIC_SELECTED_NAME="some-specific-name"

  # Kindle constants (internal)
  Z«PREFIX»_TEMP_FILE="${BURD_TEMP_DIR}/«prefix»_temp.txt"  # Non-secret data only
  Z«PREFIX»_RESULT_FILE="${BURD_TEMP_DIR}/«prefix»_«command»_«step_number»_result.json"
  Z«PREFIX»_CONFIG_PREFIX="${BURD_TEMP_DIR}/«prefix»_«command»_«step_number»_config_"     # rest of file name appended on use
  Z«PREFIX»_MANIFEST_PREFIX="${BURD_TEMP_DIR}/«prefix»_«command»_«step_number»_manifest_"   # rest of file name appended on use

  # Temporary files (if using BURD_OUTPUT_DIR)
  Z«PREFIX»_OUTPUT_RESULT="${BURD_OUTPUT_DIR}/result.txt"

  Z«PREFIX»_KINDLED=1
}

z«prefix»_sentinel() {
  test "${Z«PREFIX»_KINDLED:-}" = "1" || buc_die "Module «prefix» not kindled - call z«prefix»_kindle first"
}

# Helper functions follow

######################################################################
# External Functions («prefix»_*)

«prefix»_«command»() {
  z«prefix»_sentinel

  local z_param1="${1:-}"
  local z_param2="${2:-}"

  # Documentation block
  buc_doc_brief "«one_line_description»"
  buc_doc_param "«required_arg»" "«one_line_description»"
  buc_doc_oparm "«optional_arg»" "«one_line_description»"
  buc_doc_shown || return 0

  # Implementation
  test -n "${z_param1}" || buc_die "Parameter 'param1' is required"

  buc_step "«user_visible_milestone»"
  buc_log_args "«detailed_progress_information»"
  [command] || buc_die "«failure_explanation»"

  buc_success "«success_message»"
}
```

**Kindle constant — anti-pattern:**

```bash
# ❌ Kindle constant defined outside kindle
z«prefix»_setup() {
  Z«PREFIX»_CACHED_PATH="${BURD_TEMP_DIR}/cache"   # VIOLATION: must be in kindle
}

# ✅ Kindle constants defined in kindle, used elsewhere
z«prefix»_kindle() {
  Z«PREFIX»_CACHED_PATH="${BURD_TEMP_DIR}/cache"
  Z«PREFIX»_KINDLED=1
}
```

**Literal vs kindle constant — choosing correctly:**

```bash
# ✅ Literal constant: pure string, no variable expansion, available at source time
«PREFIX»_config_file="config.env"        # lower_snake name — no kindle needed

# ✅ Kindle constant: depends on runtime state, requires kindle
«PREFIX»_CONFIG_DIR="${BURD_TEMP_DIR}/config"  # SCREAMING name — kindle required

# ❌ Wrong: variable expansion in literal constant position
«PREFIX»_config_dir="${BURD_TEMP_DIR}/config"  # VIOLATION: uses ${}, must be in kindle
```

## Variable Handling (General Rules)

### Expansion Requirements

**Always use braced, quoted expansion:**
```bash
"${z_var}"              # Required pattern - prevents word splitting and glob expansion
"${Z_MODULE_VAR}"       # Module variables from kindle
"${1:-}"                # Parameters with defaults
```

**Never use:**
```bash
$var                    # Unquoted - word splitting risk
"$var"                  # Unbraced - boundary ambiguity
${var}                  # Unquoted - glob expansion risk
```

**Quote inside parameter expansion operators:**
```bash
# ✅ Inner expansion quoted separately — matches literally
"${z_path%"${z_suffix}"}"       # Strip suffix
"${z_path#"${z_prefix}"}"       # Strip prefix

# ❌ Inner expansion unquoted — matches as a glob pattern
"${z_path%${z_suffix}}"         # z_suffix with * or ? chars matches as glob!
"${z_path#${z_prefix}}"         # Same risk
```

When using `%`, `%%`, `#`, `##` operators with a variable as the pattern, the inner expansion must be quoted separately. Without inner quotes, the pattern is glob-expanded — a suffix containing `*` or `?` silently matches more than intended.

### Assignment Patterns

```bash
# Direct assignment
local z_message="Processing item"

# ❌ NEVER declare multiple variables on one line
local z_name z_value z_result  # Hidden initialization failures!

# ✅ ALWAYS one declaration per line with explicit initialization
local z_name=""
local z_value="default"
local z_result=0

# ❌ NEVER use local -i — silently coerces non-integer values to 0
local -i z_count="${z_input}"  # "abc" becomes 0 — violates no-silent-failures!

# ✅ Use plain local with explicit validation
local z_count="${z_input}"
test "${z_count}" -ge 0 2>/dev/null || buc_die "z_count must be non-negative integer, got: ${z_count}"

# ✅ Two-line pattern for captured values (explicit and debuggable)
local z_token
z_token=$(zauth_get_token_capture) || buc_die "Failed to capture token"

# ❌ NEVER combine declaration with capture
local z_token=$(zauth_get_token_capture)  # Hides exit status!

# From file using builtin $(<file) - NOT command substitution, a bash builtin
local z_content=$(<"${Z«PREFIX»_TEMP_FILE}")
test -n "${z_content}" || buc_die "Failed to read or empty: ${Z«PREFIX»_TEMP_FILE}"

# Never split declaration from $(<file) assignment
# ❌ local z_content
# ❌ z_content=$(<"${Z«PREFIX»_FILE}")  # Unnecessary verbosity

# Use Capture functions for secrets that must never touch disk
local z_token
z_token=$(zauth_get_token_capture) || buc_die "Failed to capture token"

# Temp files INSTEAD of command substitution holds intermediate results
echo "${z_result}" > "${Z«PREFIX»_RESULT_FILE}"
complex_command > "${Z«PREFIX»_TEMP1}" || buc_die "Complex command failed"
process < "${Z«PREFIX»_TEMP1}" > "${Z«PREFIX»_TEMP2}" || buc_die "Process failed"
local z_result=$(<"${Z«PREFIX»_TEMP2}")
test -n "${z_result}" || buc_die "Failed to read or empty: ${Z«PREFIX»_TEMP2}"
```

### Prefer Bash Builtins Over External Tools

```bash
# ✅ String manipulation with parameter expansion
"${z_path##*/}"                    # Instead of: basename
"${z_path%/*}"                     # Instead of: dirname
"${z_string//old/new}"             # Instead of: sed 's/old/new/g'
"${z_string#prefix}"               # Instead of: sed 's/^prefix//'
"${z_string%suffix}"               # Instead of: sed 's/suffix$//'

# ✅ Reading lines with 'read' — guard handles missing trailing newline
while IFS= read -r z_line || test -n "${z_line}"; do
  echo "Processing: ${z_line}" || buc_die "Failed to echo line"
done < "${Z_MODULE_INPUT_FILE}"

# ✅ Arithmetic with $((...))
z_result=$((z_value * 2 + 10))    # Instead of: expr or bc
```

### Temp Files Instead of Command Substitution

```bash
# ❌ Anti-pattern: nested command substitution
z_result=$(echo $(complex_command) | process)  # Hidden failures!

# ✅ Correct: temp files make failures visible afterward
complex_command > "${Z_MODULE_TEMP1}" || buc_die "Complex command failed"
process < "${Z_MODULE_TEMP1}" > "${Z_MODULE_TEMP2}" || buc_die "Process failed"
read -r z_result < "${Z_MODULE_TEMP2}" || buc_die "No result"

# ⚠️ EXCEPTION: Secrets use _capture functions, never temp files
```

**Temp file lifecycle**: Temp files under `BURD_TEMP_DIR` are **preserved after execution** for forensic debugging. Never delete temp files in module code — their persistence is intentional. Cleanup is handled by infrastructure outside BCG's scope.

### Stderr Capture — Never Suppress

Never use `2>/dev/null` on external commands. Redirect stderr to a temp file so forensic evidence is preserved when commands fail. Include the stderr file path in the `buc_die` message so the user can inspect it.

```bash
# ❌ Anti-pattern: stderr suppressed — failure cause invisible
curl -sS "${z_url}" -o "${z_output_file}" 2>/dev/null \
  || buc_die "Failed to fetch ${z_url}"

# ✅ Correct: stderr captured to temp file, referenced in die message
curl -sS "${z_url}" -o "${z_output_file}" 2>"${z_stderr_file}" \
  || buc_die "Failed to fetch ${z_url} — see ${z_stderr_file}"
```

**Stderr file naming**: Use a kindle `_PREFIX` constant (grouped with other temp file prefixes for collision visibility) and append a discriminator at the usage site.

**In loops**, use an auto-incrementing integer for uniqueness rather than trusting loop data values to be unique:

```bash
# In kindle — grouped with sibling temp file prefixes
Z«PREFIX»_«COMMAND»_PREFIX="${BURD_TEMP_DIR}/«prefix»_«command»_"

# In command function — integer discriminator ensures uniqueness
local z_index=0
while IFS='|' read -r z_varname z_image z_tag || test -n "${z_varname}"; do
  local z_manifest_file="${Z«PREFIX»_«COMMAND»_PREFIX}${z_index}_manifest.json"
  local z_stderr_file="${Z«PREFIX»_«COMMAND»_PREFIX}${z_index}_inspect_stderr.txt"

  docker manifest inspect "${z_image}:${z_tag}" \
    > "${z_manifest_file}" 2>"${z_stderr_file}" \
    || buc_die "Failed to inspect ${z_image}:${z_tag} — see ${z_stderr_file}"

  z_index=$((z_index + 1))
done
```

### String Usage

```bash
# Single-line HERE-string allowed for piping variables
jq '.field' <<<"${z_json_data}"
grep "pattern" <<<"${z_content}"

# Multi-line HERE-strings and HERE-docs prohibited
```

### Readonly Patterns

Readonly enforcement makes mutation bugs loud at the point of violation rather than silent cascades downstream. Three patterns at different scopes:

#### `local -r` — Default for Local Variables

Use `local -r` for every local variable that is assigned once and never modified. This is the majority of locals in a typical BCG function.

```bash
# ✅ Default: readonly locals
local -r z_filepath="${1:-}"
local -r z_varname="${z_buv_varname_roll[$z_idx]}"
local -r z_full_ref="${z_image}@${z_digest}"
local -r z_content=$(<"${z_file}")    # $(<file) is a builtin — safe on one line
```

**Four exceptions** where `local` (mutable) is required:

```bash
# Exception 1: Loop counters
local z_index=0
z_index=$((z_index + 1))

# Exception 2: Per-iteration synthesized variables (reassigned each pass)
local z_manifest_file=""
for z_spec in "${z_specs[@]}"; do
  z_manifest_file="${Z_PREFIX}${z_index}_manifest.json"
  ...
done

# Exception 3: Two-line capture pattern (readonly would lock empty value)
local z_token
z_token=$(zauth_get_token_capture) || buc_die "Failed to capture token"

# Exception 4: Accumulators and state-tracking flags
local z_any_failed=0
local z_current_group=""
```

**Why not `local -i`?** `local -i` silently coerces non-integers to 0 — it hides bugs. `local -r` does the opposite — it surfaces mutation bugs loudly. Same flag syntax, opposite safety properties.

#### `readonly` — Every Kindle Constant

Every variable assigned in a kindle function is a constant. Apply `readonly` to every assignment — whether the value is a literal, computed from other variables, or composed from prior assignments:

```bash
z«prefix»_kindle() {
  test -z "${Z«PREFIX»_KINDLED:-}" || buc_die "Module «prefix» already kindled"

  # Literal constants
  readonly Z«PREFIX»_API_VERSION="v2"
  readonly Z«PREFIX»_MAX_RETRIES=3

  # Computed from already-set values
  readonly Z«PREFIX»_REGISTRY_HOST="${RBGD_LOCATION}${RBGC_HOST_SUFFIX}"
  readonly Z«PREFIX»_TEMP_PREFIX="${BURD_TEMP_DIR}/«prefix»_"

  # Composed from prior kindle assignments
  readonly Z«PREFIX»_ACCEPT_TYPES="${Z«PREFIX»_TYPE_A},${Z«PREFIX»_TYPE_B}"

  readonly Z«PREFIX»_KINDLED=1
}
```

If a kindle function uses intermediate variables to build up a result (e.g., composing array elements), the final assignment is still `readonly`. Intermediates that are truly local to the build-up should use `local -r`.

**The KINDLED sentinel** is a kindle constant — it gets `readonly` like every other kindle assignment. The `test -z` guard at the top of kindle fires before any reassignment attempt, so `readonly` is belt-and-suspenders: the guard gives a friendly error message, `readonly` is the backstop.

#### Mutable Kindle State

Some kindle functions initialize mutable state alongside constants — counters, accumulators, registry rolls, and builder state that are modified after kindle returns. These variables **must not** be `readonly`, but they must be visually distinguishable from kindle constants.

**Convention**: Mutable kindle state uses **lowercase** `z_«prefix»_name`. Constants use **UPPERCASE** `readonly Z«PREFIX»_NAME`. Case alone signals mutability.

```bash
z«prefix»_kindle() {
  test -z "${Z«PREFIX»_KINDLED:-}" || buc_die "Module «prefix» already kindled"

  # ✅ Kindle constants — UPPERCASE, readonly
  readonly Z«PREFIX»_TEMP_PREFIX="${BURD_TEMP_DIR}/«prefix»_"

  # ✅ Mutable kindle state — lowercase, NO readonly
  z_«prefix»_file_index=0              # counter incremented post-kindle
  z_«prefix»_name_roll=()              # registry populated by enroll calls
  z_«prefix»_current_scope=""           # builder state set during enrollment

  readonly Z«PREFIX»_KINDLED=1
}
```

**Rules**:
- Mutable kindle state follows the same `z_` internal prefix as roll arrays
- Never apply `readonly` to mutable kindle state
- If a variable is initialized in kindle and never mutated afterward, it is a constant — use `readonly Z«PREFIX»_NAME`
- The `_roll` suffix is conventional for parallel-array registries but not required for all mutable state (counters, flags, builder state use descriptive names)

**Test reset**: When tests need fresh enrollment state, provide an internal reset function that clears mutable kindle state without touching `KINDLED`. Re-kindling a module is never correct — the sentinel is `readonly`.

```bash
# ✅ Test support: reset mutable state, module stays kindled
z«prefix»_reset_enrollment() {
  z«prefix»_sentinel
  z_«prefix»_name_roll=()
  z_«prefix»_current_scope=""
  # ... clear all mutable kindle state ...
}
```

**The re-source trap** — `readonly` belongs in code, never in sourceable `.env` files:
```bash
# ❌ NEVER in sourceable .env files
# If a .env file contains `readonly FOO=bar`, sourcing it a second time
# (e.g., regime reload, test re-initialization) dies with:
#   bash: FOO: readonly variable
```

#### `readonly VAR` — Lock After Enforce

For regime variables loaded from `.env` files, lock them *after* validation succeeds. This is the regime archetype's lock step (see Regime Module Archetype above):

```bash
z«prefix»_enforce() {
  z«prefix»_sentinel
  buv_vet «SCOPE»
  # ... custom format checks ...
}

# In the lock step (called after enforce succeeds):
# Lock all enrolled variables — any downstream mutation is now a loud error
readonly REGI_FIELD_ONE REGI_FIELD_TWO REGI_FIELD_THREE

# Derived state built from validated values:
Z«PREFIX»_DOCKER_ENV=("-e" "REGI_FIELD_ONE=${REGI_FIELD_ONE}")
```

**Ordering matters**: Build derived state (docker env arrays, rollup strings) *after* enforce but *before* or *during* the lock call. Once variables are readonly, derived state captures validated, immutable values.

## Special Function Definitions

### Special Function Patterns

This table defines scope: «prefix»_* is public, z«prefix»_* is internal.

| Type      | Pattern                        | First Line           | Returns                      | buc_step                  | Contract                            |
|-----------|--------------------------------|----------------------|------------------------------|---------------------------|-------------------------------------|
| Predicate | `[z]«prefix»_«name»_predicate` | `z«prefix»_sentinel` | 0=true, 1=false              | No (use buc_log_«source») | Never dies, status only             |
| Capture   | `[z]«prefix»_«name»_capture`   | `z«prefix»_sentinel` | stdout once at end or exit 1 | No (use buc_log_«source») | Clean error handling, single return |
| Enroll    | `[z]«prefix»_[«scope»_]enroll` | `z«prefix»_sentinel` | `z_«funcname»_«retval»` vars | No (use buc_log_«source») | Mutates rolls (parallel arrays) in kindle only; returns via variables |
| Recite    | `[z]«prefix»_«what»_recite`    | `z«prefix»_sentinel` | stdout or exit 1             | No (use buc_log_«source») | Read-only access to rolls; never mutates |
| Litmus predicate | `z«tb»_«name»_litmus_predicate` | —              | 0=proceed, 1=skip            | No                        | Reusable, composable; never dies, no output |
| Fixture baste | `z«tb»_«name»_baste`         | —                   | — (side effects only)        | Optional                  | Fixture subshell; kindle, source, configure |
| Tcase     | `«tc»_«name»_tcase`           | —                    | — (exit status to suite)     | No                        | Case subshell; assertions, verification |

---

#### Capture Functions (controlled command substitution)

**Purpose: Return a single processed value through stdout with clean error handling.**

Capture functions encapsulate command substitution to ensure predictable error handling and single-point-of-return semantics.

```bash
z«prefix»_«name»_capture() {
  z«prefix»_sentinel

  # Process and validate internally
  local z_result
  z_result=$(some_command | process) || return 1

  # Validate without buc_die
  test -n "${z_result}" || return 1

  # Single output at end
  echo "${z_result}"
}

# Caller uses two-line pattern
local z_value
z_value=$(z«prefix»_«name»_capture) || buc_die "Failed to capture value"
```

**Use capture functions for:**
- Secrets that must never touch disk
- Values requiring command substitution or pipeline processing
- Any computed value where clean error handling matters

**Contract:**
- Returns stdout once at end or exits with status 1
- Never uses `buc_die` internally
- No side effects beyond the returned value
- May source credential files when necessary for security
- May use `buc_log_«source»` for forensic trail (writes to transcript file only)
- Never writes secrets to disk
- Never writes to stderr

#### Predicate Functions (status only, no output)

Predicate functions are used to make decisions, not indicating error.

```bash
z«prefix»_file_valid_predicate() {
  z«prefix»_sentinel

  local z_file="${1:-}"

  # Return status only - no output, no dies
  test -f "${z_file}" || return 1
  grep -q "required_pattern" "${z_file}" || return 1

  return 0
}

# Caller uses in conditionals
z«prefix»_file_valid_predicate "${z_config}" || buc_warn "Config invalid, using defaults"
```

#### Enroll Functions (kindle-only registry population)

**Purpose**: Populate parallel-array registries (rolls) during module initialization. Enroll functions solve the subshell problem: when a function both mutates shared state and returns a value, calling it in `$()` loses the mutations.

**The parallel-array registry pattern (rolls)**:
- Kindle initializes N empty arrays (rolls) of the same length
- Enroll validates inputs and appends atomically to all rolls
- Recite functions (see below) provide read-only access after kindle completes
- Variable naming: `z_«prefix»_«name»_roll` for all roll arrays (internal state, initialized in kindle)

**Example** (using synthetic notation — «reg» owns the rolls, «con» consumes them):

```bash
# Module «reg» — owns the rolls
z«reg»_kindle() {
  test -z "${Z«REG»_KINDLED:-}" || buc_die "already kindled"

  z_«reg»_name_roll=()
  z_«reg»_target_roll=()
  z_«reg»_handler_roll=()

  Z«REG»_KINDLED=1   # MUST be last — sentinel guards all subsequent calls
}

# Enroll function — called by consuming module's kindle, NOT «reg»'s own kindle
«reg»_enroll() {
  z«reg»_sentinel   # Safe: «reg» is already kindled when consumer calls this

  local z_name="${1:-}"
  local z_target="${2:-}"
  local z_handler="${3:-}"
  test -n "${z_name}" || buc_die "enroll: name required"

  # Registration-time validation
  declare -F "${z_handler}" >/dev/null || buc_die "enroll: handler not found: ${z_handler}"

  z_«reg»_name_roll+=("${z_name}")
  z_«reg»_target_roll+=("${z_target}")
  z_«reg»_handler_roll+=("${z_handler}")

  # Return via variable — NOT echo
  z_«reg»_enroll_name="${z_name}"
}

# Module «con» — consumes the registry
z«con»_kindle() {
  test -z "${Z«CON»_KINDLED:-}" || buc_die "already kindled"

  # «reg» must be kindled first (rolls initialized, sentinel passes)
  «reg»_enroll "alpha" "/path/alpha" "handle_alpha"
  Z«CON»_DEFAULT="${z_«reg»_enroll_name}"

  «reg»_enroll "bravo" "/path/bravo" "handle_bravo"

  Z«CON»_KINDLED=1   # MUST be last
}
```

**Key constraints**:
- Enroll functions may ONLY be called within kindle
- Rolls are initialized in kindle, populated by enroll in kindle, then immutable after kindle completes
- Enroll functions work exclusively with rolls (parallel arrays) — no other shared state mutation
- Registration-time validation: check invariants at enroll time, not at recite/execution time
- Return values use `z_«funcname»_«retval»` convention (function name embedded verbatim for traceability)
- Must NOT be called inside `$()` — side effects would be lost
- May use `buc_die` internally (unlike `_capture`)

**Two-level registries**: When entities have parent-child relationships (e.g., groups containing items), use two flat registries with a foreign-key column rather than per-parent dynamic arrays. This avoids `eval` and stays bash 3.2 safe.

**Contract summary**:
- Mutates rolls (parallel arrays) only
- Returns value(s) via `z_«funcname»_«retval»` variables (NOT echo)
- Called exclusively within kindle
- Should rarely be used — only when a function must both mutate and return

#### Recite Functions (read-only roll access)

**Purpose: Provide read-only access to roll arrays populated by enroll functions.**

Recite functions are the only access path to rolls after kindle completes. They must never mutate roll arrays.

```bash
«prefix»_«what»_recite() {
  z«prefix»_sentinel

  local z_key="${1:-}"
  test -n "${z_key}" || return 1

  local z_i=0
  for z_i in "${!z_«prefix»_name_roll[@]}"; do
    test "${z_«prefix»_name_roll[$z_i]}" = "${z_key}" || continue
    echo "${z_«prefix»_target_roll[$z_i]}" || return 1
    return 0
  done
  return 1
}

# Caller uses two-line capture pattern
local z_target
z_target=$(«prefix»_target_recite "alpha") || buc_die "not found"
```

**Contract:**
- Read-only: MUST NOT mutate roll arrays or any shared state
- Returns via echo (like capture functions) — safe to call in `$()`
- Returns 1 on not-found (like `_capture`) — never uses `buc_die` internally
- Caller decides error handling: `|| buc_die` or conditional
- Usable anywhere after kindle completes (not restricted to kindle)

---

## When to Use Special Functions

- **_capture**: Need a value that requires command substitution or clean error handling
- **_predicate**: Need true/false for conditional logic without dying
- **_enroll**: Populate parallel-array registries (rolls) during kindle with validated inputs and return values
- **_recite**: Read-only access to roll arrays populated by enroll functions
- **_litmus_predicate**: Fixture precondition check in parent shell (0=proceed, 1=skip); reusable, composable
- **_baste**: Fixture preparation inside fixture subshell (kindle, source, configure)
- **_tcase**: Test case verification function inside `_tcase` subshell
- **Neither**: Use temp files for multi-step pipelines, direct `|| buc_die` for simple failures

## Naming Convention Patterns

### ✅ Correct Patterns

| Element                      | Pattern                      | Example                      | Location | Name/ Case Constraints        |
|------------------------------|------------------------------|------------------------------|----------|-------------------------------|
| Module prefix                | `[a-z]{2,4}_`                | `rbv_`, `buc_`, `auth_`      | Both     | single char group             |
| Implementation file          | `«prefix»_«name».sh`         | `rbv_podman.sh`              | N/A      | lowercase (prefer 1 word, allow snake_case) |
| CLI file                     | `«prefix»_cli.sh`            | `rbv_cli.sh`                 | N/A      | snake_case (fixed)            |
| Public functions             | `«prefix»_«command»`         | `rbv_init`, `rbv_start`      | Impl     | snake_case (usually one word) |
| Internal functions           | `z«prefix»_«name»`           | `zrbv_validate_pat`          | Impl     | snake_case (often multi word) |
| Furnish function             | `z«prefix»_furnish`          | `zrbv_furnish`               | CLI      | fixed name                    |
| Kindle function              | `z«prefix»_kindle`           | `zrbv_kindle`                | Impl     | fixed name                    |
| Sentinel function            | `z«prefix»_sentinel`         | `zrbv_sentinel`              | Impl     | fixed name                    |
| Predicate Internal functions | `z«prefix»_«name»_predicate` | `zrbv_file_exists_predicate` | Impl     | snake_case                    |
| Capture Internal functions   | `z«prefix»_«name»_capture`   | `zrbv_get_token_capture`     | Impl     | snake_case                    |
| Capture Public functions     | `«prefix»_«name»_capture`    | `rbv_get_token_capture`      | Impl     | snake_case                    |
| Kindle constant (internal)   | `Z«PREFIX»_«NAME»`           | `ZRBV_TEMP_FILE`             | Impl     | SCREAMING_SNAKE (multi-word)  |
| Kindle constant (public)     | `«PREFIX»_«NAME»`            | `RBV_REGIME_FILE`            | Both     | SCREAMING_SNAKE (multi-word)  |
| Literal constant (public)    | `«PREFIX»_«name»`            | `RBBC_rbrr_file`             | Impl     | lower_snake (multi-word)      |
| Local parameters             | `z_«name»`                   | `z_vm_name`, `z_force_flag`  | Both     | snake_case (multi-word)       |
| Enroll functions             | `[z]«prefix»_[«scope»_]enroll` | `«prefix»_enroll`          | Impl     | kindle-only                   |
| Recite functions             | `[z]«prefix»_«what»_recite`  | `«prefix»_target_recite`     | Impl     | read-only, never mutates      |
| Roll arrays                  | `z_«prefix»_«name»_roll`     | `z_rbv_target_roll`          | Impl     | snake_case, kindle-only       |
| Enroll return vars           | `z_«funcname»_«retval»`      | `z_«prefix»_enroll_name`     | Impl     | snake_case (func name verbatim)|
| Testbench file               | `«prefix»tb_testbench.sh`    | `rbtb_testbench.sh`          | N/A      | fixed name                    |
| Test case file               | `«prefix»tc«xx»_«Name».sh`  | `rbtckk_KickTires.sh`        | N/A      | PascalCase name               |
| Litmus predicate             | `z«tb»_«name»_litmus_predicate` | `zrbtb_container_runtime_litmus_predicate` | Testbench| snake_case, parent layer |
| Fixture baste function       | `z«tb»_«name»_baste`        | `zrbtb_ark_baste`            | Testbench| snake_case, fixture layer     |
| Case function                | `«tc»_«name»_tcase`         | `rbtckk_false_tcase`         | Test case| snake_case, case layer        |

---

## Runtime Visibility vs Comments

### ✅ Use Runtime Visibility

```bash
# ✅ Use Runtime Visibility
buc_step "Fetching manifest from registry"          # User milestones
buc_log_args "Processing platform linux/amd64"      # Variable details
jq '.response' file.json | buc_log_pipe            # Pipeline output

# When to use each source:
buc_log_args - Simple messages, variable values, progress markers
buc_log_pipe - Command output, JSON responses, multi-line data
```

### ❌ Never Use Comments for Runtime Info

```bash
# Building JWT header         ❌ Wrong
buc_log_args "Building JWT header"  ✅ Correct

# Process each manifest       ❌ Wrong
buc_step "Processing manifests" ✅ Correct
```

Comments are ONLY for code clarification that doesn't matter at runtime.

---

## Test vs Bracket Expressions

### ✅ Use test Command instead of `[[ xxx ]]/fi` where possible

```bash
# Error, warning, and optional reporting
test -f "${z_file}" || buc_die "File not found"
test "${z_var}" = "expected" || buc_warn "Unexpected value"
test -n "${z_param}" || buc_step "Running with zero length parameter"
```

### ✅ Exception: Pattern Matching Only

```bash
# Only acceptable use of [[ ]] - pattern matching requires it
[[ "${z_filename}" =~ \.tar\.gz$ ]] && z_extension="tar.gz"
```

### ❌ Anti-Patterns

```bash
# Avoid [[ ]] for simple tests
[[ -f "${z_file}" ]]           # Use: test -f "${z_file}"
[[ "${z_var}" == "value" ]]    # Use: test "${z_var}" = "value"
[[ -n "${z_param}" ]]          # Use: test -n "${z_param}"

# Never use test with command substitution that might expand to empty
# test with ZERO arguments returns TRUE (exit 0) — silent pass landmine
test $(echo "${z_val}" | grep -E '^pattern$')  # ❌ Succeeds when grep matches nothing!

# ✅ Use exit-status predicates instead
echo "${z_val}" | grep -qE '^pattern$' || buc_die "Invalid format"

# ✅ Or use case statement for pattern matching
case "${z_val}" in
  *pattern*) buc_log_args "Found pattern" ;;
  *) buc_die "Pattern not found" ;;
esac
```

**Note**: Use `=` not `==` in test expressions for POSIX compliance.

### ❌ Subshell Exit vs Brace-Group

```bash
# ❌ exit in subshell kills only the subshell — script continues
some_cmd || (echo "ERROR" >&2 && exit 1)

# ✅ Brace-group stays in same process — exit/return works correctly
some_cmd || { echo "ERROR" >&2; exit 1; }
```

**Rule**: Error blocks must use `{ ...; }` not `( ... )` when intending to exit or return. The `(subshell)` creates a new process — `exit` terminates only that subshell, not the calling script.

### ✅ Legitimate Subshell Uses

The prohibition above is specific to error-handling blocks where `exit`/`return` must reach the calling shell. Subshells are legitimate in these contexts:

- **Test execution** — principled isolation between suites and cases; see **Test Execution Patterns**
- **`$()` command substitution** — inside `_capture` functions only; see **Capture Functions**
- **Isolation subshells** — environment containment with exit-status propagation (below)

#### Isolation Subshells (environment containment)

When a sequence of commands mutates the shell environment (sourcing files, `cd`, venv activation, `export`), a subshell prevents those mutations from leaking into subsequent commands. The exit status propagates across the `)` boundary, so the caller can detect and handle failures.

**Rarely needed.** Most code operates within a single kindled environment. Use isolation subshells only when you must execute commands that change the environment and those changes must not persist.

**Two variants** — same principle, different caller handling:

```bash
# Variant 1: Error propagation — die on any internal failure
(
  source "${z_config_file}" || buc_die "Failed to source: ${z_config_file}"
  «prefix»_operation || buc_die "Operation failed"
) || buc_die "Isolation subshell failed"

# Variant 2: Status capture — caller inspects and decides
local z_status=0
(
  source "${z_config_file}" || buc_die "Failed to source: ${z_config_file}"
  «prefix»_operation || buc_die "Operation failed"
) || z_status=$?
```

Inside the subshell, `buc_die` calls `exit` which terminates only the subshell — producing a non-zero exit status that the outer `|| buc_die` or `|| z_status=$?` catches. Failure propagates through exit status, not through `exit` reaching the parent.

**Constraints:**
- Every command inside must have explicit `|| buc_die` — `set -e` is suppressed inside `( ... ) ||` (as with all BCG code, never rely on it)
- The outer boundary must have `|| buc_die` or `|| z_status=$?` — never leave the subshell exit status unchecked
- Sourcing inside the subshell is permitted (this is a primary use case — isolation prevents sourced variables from leaking)
- Keep subshell bodies short and focused — if the body grows complex, delegate to a CLI script via `exec`
- `$( ... ) || buc_die` is legitimate when you need both isolation and output capture — `$()` is a subshell, so isolation holds and the two-line capture pattern applies

**Use cases:**
- Iterating config files that define overlapping variables (e.g., sourcing each nameplate `.env` in a loop)
- Running commands that require `cd` to a different directory
- Activating a virtual environment for a single operation
- Any sequence where environment mutations must not outlive the block

**Anti-pattern — no internal error handling:**

```bash
# ❌ Failures inside subshell are silent — source or echo can fail undetected
(
  source "${z_file}"
  echo "${SOME_VAR}"
) || buc_die "Subshell failed"

# ✅ Every command explicitly handled
(
  source "${z_file}" || buc_die "Failed to source: ${z_file}"
  echo "${SOME_VAR}" || buc_die "Failed to echo"
) || buc_die "Subshell failed"
```

---

## Prohibited Constructs

### ❌ HERE Documents (Heredocs)

```bash
# Never use - breaks indentation
cat <<EOF > "${z_file}"
content here
must be unindented
EOF

# Use temp files or echo sequences instead
echo "content here" > "${z_file}"
echo "stays indented" >> "${z_file}"
```

### ❌ Bash 4+ Features

```bash
# Avoid these (bash 3.2 compatibility)
declare -A z_array              # Associative arrays
z_files=(**/*.txt)              # ** globbing
readarray -t z_lines            # readarray/mapfile
```

### ❌ Bare File Truncation

```bash
# ❌ Looks like a line where the command was accidentally deleted
> "${z_file}"

# ✅ Explicit no-op makes truncation intent unambiguous
: > "${z_file}"
```

Bare `> file` without a command is valid bash but reads like an accident at 3am. The `: >` form explicitly signals "I am creating or truncating this file." Shellcheck enforces this via SC2188.

**Prefer immutable temp files.** BCG temp files are forensic artifacts — preserved for post-mortem debugging. If you find yourself truncating a temp file, the design likely needs a unique filename per step instead of reusing and clearing. Truncation is appropriate only for log files and other non-forensic outputs.

### ❌ Inline Shellcheck Directives

```bash
# ❌ Never suppress shellcheck inline
# shellcheck disable=SC2086
some_command ${z_unquoted}

# ❌ Never use source path hints
# shellcheck source=/dev/null
source "${z_file}"
```

All BCG-structural shellcheck suppressions live in `busc_shellcheckrc`. No inline `# shellcheck` directives of any kind. If a shellcheck code fires and it's BCG-structural, add it to `busc_shellcheckrc` with rationale. If it's a genuine finding, fix the code.

### ❌ Commit-Message Comments

A **commit-message comment** describes the *change event* rather than the *code*. It narrates what was added, when, or why it was inserted — information that belongs in `git log`, not in the file. The tell: the comment would make more sense as a commit summary than as documentation for a future reader.

```bash
# ❌ Commit-message comment: narrates the insertion
# Added qualification subsystem for shellcheck support
buw-qsc) exec "${z_buq_cli}" buq_shellcheck "$@" ;;

# ❌ Commit-message comment: documents the change, not the code
# New helper for retry logic (see PR #142)
zrbf_retry_with_backoff() {

# ✅ No comment needed — the code is self-evident
buw-qsc) exec "${z_buq_cli}" buq_shellcheck "$@" ;;

# ✅ Comment describes the code, not the change
# Retry with exponential backoff; max delay capped at 60s
zrbf_retry_with_backoff() {
```

**Litmus test**: Would this comment be better as a commit message? If yes, delete it from the code — it's already captured in git history.

This is a characteristic LLM failure mode. Models narrate their own work process into code, writing for the person reviewing the diff rather than the person reading the file next year. The resulting comments assume a supervisory reader with conversation context — an audience that does not exist at maintenance time.

---

## Interface Contamination Discipline

**Interface contamination** is any change that expands the accepted input space, invocation forms, or alternative paths through an interface without explicit specification requirement.

### Zeroes Theory

Every tolerance, alias, fallback, or alternative path multiplies the enumerated state space. The litmus test: **"How many zeroes got added to the enumerated state space because of this choice?"** If the answer isn't zero, the change requires explicit justification.

### Rules

1. **One canonical form**: Every command, argument, and parameter has exactly one accepted form. Do not accept alternatives.
2. **No tolerances**: Do not add case folding, prefix stripping, alias acceptance, or alternative parse paths. If the canonical form is `--racing`, do not also accept `-r`, `racing`, or `RACING`.
3. **No undocumented defaults**: Do not add `${1:-default}` patterns that silently fill missing required arguments. Required means required.
4. **No silent normalization**: Do not silently transform input. If input doesn't match the canonical form, `buc_die` with a clear message stating the expected form.
5. **Documentation uses canonical form**: All documentation, examples, and `buc_doc_*` output must show the single canonical invocation — never a tolerated alternative.

### Smell Test

If a command can be invoked two different ways for the same result, the interface is contaminated. If `"${1:-}"` silently provides a default for what should be a mandatory argument, the interface is contaminated.

---

## `set -e` is Not Sufficient

**The POSIX suppression rule**: `set -e` is suppressed inside `if`, `while`, `||`, `&&` test expressions, and this propagates through the **entire call tree** of the tested command.

```bash
# ❌ set -e suppressed for entire call tree of some_function
if some_function; then ...

# Even if some_function calls other_function that fails,
# set -e will not terminate — suppression propagates
```

**BCG rule: Only `_predicate` functions may appear in `if`/`while` conditions.** All other functions must be invoked as simple commands with explicit `|| buc_die` / `|| return`. This completely prevents the suppression hazard.

```bash
# ✅ Predicate in conditional — designed for this, never dies, status only
if z«prefix»_ready_predicate; then ...

# ✅ Regular function — explicit error handling, not inside conditional
some_function || buc_die "..."

# ❌ Regular function in conditional — set -e suppressed for entire call tree
if some_function; then ...
```

**Why BCG's `|| buc_die` discipline works**: It doesn't rely on `set -e` — explicit error handling after every command means suppression doesn't matter. The `|| z_status=$?` capture pattern intentionally relies on this suppression behavior.

**Loops**: `set -e` does not reliably catch failures inside `for`/`while` loop bodies in bash 3.2. Every command in a loop body must have explicit error handling.

The error handling suffix depends on the function type:

| Function type    | On failure use       | Why                                             |
|------------------|----------------------|-------------------------------------------------|
| Regular/enroll   | `\|\| buc_die`       | Enroll validates invariants; violations are fatal |
| Predicate        | `\|\| return 1`      | Never dies, status only                          |
| Capture/recite   | `\|\| return 1`      | Never dies, caller decides                       |
| Flow control     | `\|\| continue`      | Intentional skip to next iteration               |

### ❌ Test-and-control-flow with `&&`

```bash
# ❌ Relies on set -e exemption for &&/|| lists — fragile
test "${z_val}" = "done" && break
test -z "${z_val}" && return 0
[[ "${z_line}" == *"pattern"* ]] && continue

# ✅ Invert the test, use || — same behavior, no exemption reliance
test "${z_val}" != "done" || break
test -n "${z_val}" || return 0
[[ "${z_line}" != *"pattern"* ]] || continue
```

**BCG rule: Never `test ... && break/continue/return`.** Always invert the test and use `||`. The `||` form is already the BCG standard for control flow after a test; the `&&` form relies on humans remembering that bash exempts `&&`/`||` lists from `set -e`, which is a language-lawyer trap.

### ❌ `A && B || C` as Pseudo-Ternary

```bash
# ❌ C runs both when A fails AND when B fails — not equivalent to if/then/else
curl "${z_url}" && jq '.field' "${z_file}" || buc_die "Failed"

# ✅ Explicit if/then/else — C runs only when A fails
if curl "${z_url}"; then
  jq '.field' "${z_file}" || buc_die "jq failed"
else
  buc_die "curl failed"
fi
```

**BCG rule: Never `A && B || C`.** This is not if-then-else. When `A` succeeds but `B` fails, `C` runs — which is almost never the intended behavior. Use explicit `if`/`then`/`else` for conditional execution with a fallback.

---

## Array Safety Under `set -u`

Under `set -u`, `"${array[@]}"` on an empty array triggers "unbound variable" in bash 3.2. Two patterns are safe; choose by whether you need the index.

**Index iteration** — use when you need the index (parallel arrays, kindle rolls). Inherently safe on empty arrays, no guard needed:

```bash
for z_i in "${!z_«prefix»_name_roll[@]}"; do
  echo "${z_«prefix»_name_roll[$z_i]}" || buc_die "Failed to echo"
done
```

**Guarded value iteration** — use when you only need values. The `(( ))` guard prevents expansion of the empty array:

```bash
if (( ${#z_«prefix»_name_roll[@]} )); then
  for z_val in "${z_«prefix»_name_roll[@]}"; do
    echo "${z_val}" || buc_die "Failed to echo"
  done
fi
```

**Anti-pattern** — unguarded value iteration on a possibly-empty array:

```bash
# ❌ Crashes under set -u in bash 3.2 when array is empty
for z_val in "${z_«prefix»_name_roll[@]}"; do
  echo "${z_val}" || buc_die "Failed to echo"
done
```

---

## Stdin Consumption in While-Read Loops

### The Problem

When `while read ... done < file` feeds a file to stdin (FD 0), **any child process that reads stdin silently consumes the loop's remaining input**. This includes `docker exec -i`, `ssh`, `read` without explicit FD, `cat` without arguments, and any command that inherits stdin.

The failure is **silent and partial**: the loop processes some iterations correctly, then stops early with no error. This is one of the hardest bash bugs to diagnose because the loop appears to work — it just processes fewer items than expected.

### ❌ Anti-Pattern: Open-FD Loop Body

```bash
# ❌ File held open on stdin for entire loop — child processes can consume it
while IFS= read -r z_item || test -n "${z_item}"; do
    complex_function "${z_item}"    # If this touches stdin, remaining items vanish
done < "${z_temp_file}"
```

### ✅ Pattern: Load-Then-Iterate

**Always** read file contents into an array first, then iterate. The file is fully consumed and closed before any loop body code runs.

```bash
# ✅ Load phase — file consumed and closed
local z_items=()
while IFS= read -r z_line || test -n "${z_line}"; do
    z_items+=("${z_line}")
done < "${z_temp_file}"
rm -f "${z_temp_file}"

# ✅ Iterate phase — stdin is free, no FD held open
local z_i
for z_i in "${!z_items[@]}"; do
    test -n "${z_items[$z_i]}" || continue
    complex_function "${z_items[$z_i]}"
done
```

---

## Sourcing Rules

Sourcing is restricted because it breaks error handling. Only three locations may source files:

| Location             | Can Source                 | Purpose                                              |
|----------------------|----------------------------|------------------------------------------------------|
| CLI file header      | `buc_command.sh` only      | Bootstrap command infrastructure                     |
| CLI Furnish Function | All deps + config files    | Module loading, environment setup                    |
| Internal functions   | Credentials when necessary | Context-specific secrets (with documented rationale) |

The CLI header sources only `buc_command.sh` (via `BURD_BUK_DIR`). All other module and config sourcing happens inside the furnish function, after the `buc_doc_env` / `buc_doc_env_done` gate. This enables doc-mode help display without runtime dependencies.

## Eval Policy

`eval` is **forbidden** except for validated variable-name dereference. Require `^[A-Za-z_][A-Za-z0-9_]*$` regex validation before any eval.

**Prefer indirect expansion for reading** — `${!name}` works in bash 3.2:

```bash
# ✅ Indirect expansion for reading
local z_val="${!z_varname}"
```

**Use `printf -v` for assignment to indirect variable** — after name validation:

```bash
# ✅ printf -v for assignment — no quoting hazards from values
echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' \
  || buc_die "Invalid variable name: ${z_varname}"
printf -v "${z_varname}" '%s' "${z_new_value}" || buc_die "printf -v failed"
```

**Anti-patterns:**

```bash
# ❌ eval for assignment — quoting hazards from values with quotes/newlines/backslashes
eval "${z_varname}=\${z_new_value}"

# ❌ Unvalidated eval — injection risk
eval "local z_val=\${${z_varname}:-}"
```

## Output and Messaging Patterns

### Message Hierarchy

buc_step "Major user-visible operation"     # White, stderr, milestones
buc_log_args "Detail from variables"        # Transcript only, from arguments
command | buc_log_pipe                      # Transcript only, from pipeline
buc_warn "Non-fatal warning"                # Yellow, stderr
buc_die "Fatal error message"               # Red, stderr, exits

### When to use buc_step vs buc_log_«source»:

**Use buc_step in:**
- Public command functions (rbv_start, rbv_stop)
- Top-level user-initiated operations
- Major state transitions visible to user

**Use buc_log_«source» in:**
- Internal helper functions (zrbgo_*)
- Capture functions (*_capture) - keeps stdout clean
- Predicate functions (*_predicate) - no user output
- Forensic/debugging information
- Sub-operations within a larger step

### ✅ Code Display

```bash
buc_code "command-to-run --with-args"       # Cyan, for showing commands
```

### ❌ Anti-Patterns

```bash
# Never use raw echo for user messages
echo "Starting operation..."

# Never use comments for runtime information
# Processing items    ❌
buc_log_args "Processing items"  ✅

# Never mix stdout and stderr purposes
echo "Result: ${z_value}"          # If this is return data
echo "Progress update..." >&2      # If this is progress info
```

---

## Integration Patterns

### Using BCU Utilities

```bash
# Always prefer BCU functions over raw commands
buc_step    # Instead of echo
buc_die     # Instead of echo + exit
buc_warn    # Instead of echo >&2
```

---

## Quick Reference Decision Matrix

| Situation           | Use This Pattern                                                 |
|---------------------|------------------------------------------------------------------|
| Module structure    | Always split: implementation + CLI                               |
| Need return value   | Check error handling decision tree                               |
| Simple validation   | `test ... \|\| buc_die`                                          |
| Config validation   | Regime archetype: enrollment → enforce → report                  |
| User feedback       | `buc_step` for major milestones, `buc_log_«source»` for forensic trail    |
| Runtime information | `buc_log_«source»` for details, `buc_step` for milestones, never comments |
| Show commands       | `buc_code`                                                       |
| Non-fatal issue     | `buc_warn`                                                       |
| Fatal error         | `buc_die`                                                        |
| Success message     | `buc_success`                                                    |
| Variable expansion  | Always `"${var}"`                                                |
| Control flow after test | `test ... \|\| break/continue/return` (never `&&`)           |
| Conditional tests   | `test` not `[[ ]]`                                               |
| Pattern matching    | `[[ var =~ pattern ]]` only                                      |
| Secret extraction   | `_capture` function, never temp files                            |
| Roll population + return | `_enroll` function (kindle only), `z_«funcname»_«retval»` vars |
| Roll read access    | `_recite` function                                               |
| True/false check    | `_predicate` function                                            |
| Runtime information | `buc_log_«source»`/`buc_step`, never comments                             |
| File paths          | Kindle constants — defined in kindle only                        |
| File reading        | Single-line `$(<file)` with validation                           |
| Function structure  | See Function Patterns tables                                     |
| Error handling      | See Error Handling Decision Tree                                 |
| Forensic detail (vars)  | `buc_log_args "message ${z_var}"` |
| Forensic detail (output)| `command \| buc_log_pipe`         |

---

## Module Maturity Checklist

### Module Structure
- [ ] Implementation file «prefix»_«name».sh exists
- [ ] CLI file «prefix»_cli.sh exists (skip if library/utility module with no direct user commands)
- [ ] Implementation has multiple inclusion detection guard (`Z«PREFIX»_SOURCED`)
- [ ] CLI starts with `set -euo pipefail`
- [ ] Implementation file sources nothing (except within kindle function)
- [ ] CLI header sources only `buc_command.sh`; all other deps sourced in furnish
- [ ] External code accesses module through CLI only (no direct `z*_kindle()` calls from outside)

### Required Functions
- [ ] `z«prefix»_kindle` - first line: `test -z "${Z«PREFIX»_KINDLED:-}" || buc_die`
- [ ] `z«prefix»_sentinel` - first line: `test "${Z«PREFIX»_KINDLED:-}" = "1" || buc_die`
- [ ] `z«prefix»_furnish` (CLI only) - documents env vars, sources all deps, calls kindle; gates with `buc_doc_env_done`
- [ ] `z«prefix»_enforce` (regime archetype only) - `buv_vet` + custom format checks after kindle
- [ ] All public functions start with sentinel check
- [ ] All internal helpers prefixed with `z«prefix»_`

### Variable Management
- [ ] All kindle constants (internal `Z«PREFIX»_SCREAMING` and public `«PREFIX»_SCREAMING`) defined exclusively in kindle with `readonly`
- [ ] No kindle constant assignments outside kindle function
- [ ] Mutable kindle state (counters, rolls, builder state) uses lowercase `z_«prefix»_name` — no `readonly`
- [ ] Literal constants (`«PREFIX»_lower_name`) are pure string literals with no `${}` expansion, placed after `SOURCED` guard
- [ ] All local variables use `z_` prefix
- [ ] All expansions use `"${var}"` pattern (braced, quoted)
- [ ] Parameters use `"${1:-}"` pattern for defensive programming
- [ ] Module state variable `readonly Z«PREFIX»_KINDLED=1` is the last statement in kindle
- [ ] No bare `$var` or unbraced `"$var"` expansions
- [ ] No `local -i` — use plain local with explicit validation
- [ ] No raw `eval` for value assignment — use `printf -v` after name validation; `${!name}` for reading

### Error Handling
- [ ] Every command that can fail has `|| buc_die` (`|| buc_warn` only with a human-authored comment granting permission and explaining why non-fatal is safe)
- [ ] `_predicate` functions return 0/1, never die, no output
- [ ] `_capture` functions output once at end or exit 1, no stderr
- [ ] `_enroll` functions set `z_«funcname»_«retval»` return vars, never echo; callers never use `$()` (when applicable)
- [ ] `_enroll` functions called only within kindle (when applicable)
- [ ] `_recite` functions never mutate roll arrays (when applicable)
- [ ] Roll arrays use `z_«prefix»_«name»_roll` naming convention (when applicable)
- [ ] Two-line pattern for capturing: `z_var=$(func_capture) || buc_die`
- [ ] File reads validated: `test -n "${z_content}" || buc_die`
- [ ] Pipelines either: occur inside a `_capture` function, write to temp files with explicit status checks, or explicitly inspect `${PIPESTATUS[@]}`
- [ ] No `test $(command)` — use `grep -q` or `case` for validation
- [ ] Only `_predicate` functions in `if`/`while` conditions — no regular/enroll functions in conditionals
- [ ] No `test ... && break/continue/return` — invert test and use `||`
- [ ] Error blocks use `{ ...; }` not `( ... )` — no `|| (... exit ...)` patterns
- [ ] Isolation subshells (`( ... ) || buc_die`) have `|| buc_die` on every internal command and on the outer boundary

### Command Substitution Rules
- [ ] NO command substitution except `$(<file)` builtin and `_capture` functions
- [ ] Temp files used instead of complex command substitution
- [ ] `$(<file)` always followed by validation
- [ ] `_capture` functions properly named with suffix

### Loop Safety
- [ ] All while-read loops use load-then-iterate pattern
- [ ] All `while read` conditions include trailing-newline guard: `|| test -n "${z_var}"`
- [ ] No file descriptor held open across function calls that may touch stdin

### Bash Compatibility
- [ ] No bash 4+ features (associative arrays, `**`, readarray)
- [ ] Use `test` not `[[ ]]` except for pattern matching `=~`
- [ ] Use `=` not `==` in test expressions
- [ ] No HERE documents (heredocs)
- [ ] Here-strings (`<<<`) single-line piping only; no multi-line
- [ ] Target bash 3.2 minimum
- [ ] Array iteration uses index pattern for `set -u` safety

### Naming Conventions
- [ ] Module prefix: 2-4 lowercase letters + underscore
- [ ] Public functions: `«prefix»_«command»` (snake_case)
- [ ] Internal functions: `z«prefix»_«name»` (snake_case)
- [ ] Internal constants: `Z«PREFIX»_«NAME»` (SCREAMING_SNAKE)
- [ ] Public constants: `«PREFIX»_«NAME»` (SCREAMING_SNAKE)
- [ ] Special functions: `*_predicate`, `*_capture`, `*_enroll`, `*_recite` suffixes

### Documentation & Visibility
- [ ] Public functions documented with `buc_doc_*` blocks
- [ ] Internal functions documented with `buc_doc_*` blocks (recommended)
- [ ] `buc_doc_shown || return 0` after documentation in functions that have it
- [ ] Runtime info via `buc_log_«source»` (forensic) and `buc_step` (milestones), never comments
- [ ] Details use `buc_log_«source»` (transcript.txt)
- [ ] Comments only for code clarification, not runtime info
- [ ] No commit-message comments (narrating the change, not the code)
- [ ] Major operations use `buc_step` (always visible)

### Sourcing Restrictions
- [ ] CLI header sources only `buc_command.sh`
- [ ] `z«prefix»_furnish` sources all other dependencies and config files
- [ ] Furnish gates with `buc_doc_env_done || return 0` before any sourcing
- [ ] Credential sourcing documented and never writes to disk
- [ ] No other sourcing anywhere

### Interface Contamination Discipline
- [ ] Commands accept exactly one canonical form for each argument — no aliases
- [ ] No case folding, prefix stripping, or normalization of inputs
- [ ] No `${1:-default}` for required parameters — missing required args cause `buc_die`
- [ ] `buc_doc_*` output shows the single canonical invocation form
- [ ] Zeroes theory applied: no unexplained state space expansion

### Code Quality
- [ ] Prefer bash builtins over external tools (parameter expansion vs sed/awk)
- [ ] Use BCU utilities instead of raw bash (buc_die vs echo+exit)
- [ ] Use enrollment for config validation, `buv_dir_exists`/`buv_file_exists` for path checks
- [ ] Consistent error messages (specific and actionable)
- [ ] Proper temp file naming with module prefix
- [ ] No silent failures or ignored conditions
- [ ] No `2>/dev/null` — stderr redirected to temp file, path included in `buc_die` message
- [ ] Temp files never deleted in module code — preserved for forensics

### Enterprise Safety
- [ ] Crash-fast principle applied throughout
- [ ] No elegant-but-fragile pipeline patterns
- [ ] Secrets never written to disk (use `_capture` functions)
- [ ] Temp files make failures visible and debuggable
- [ ] Every potential error explicitly handled
- [ ] Abstraction layers used (BCU utilities, enrollment infrastructure)

### Regime Archetype (when applicable)
- [ ] Kindle calls `buv_*_enroll` for each config field
- [ ] Kindle calls `buv_scope_sentinel` after all enrollments, before `KINDLED=1`
- [ ] `z«prefix»_enforce` calls `buv_vet SCOPE` then custom format checks
- [ ] Furnish calls kindle → enforce (ironclad gate)
- [ ] CLI provides `*_validate` command using `buv_report`
- [ ] CLI provides `*_render` command using `buv_render`

---

## Test Execution Patterns

### Principled Subshell Use

BCG prohibits subshells for error handling (see **Subshell Exit vs Brace-Group**). Test execution is the principled counterpoint: subshells provide **isolation boundaries** that prevent state leakage between fixtures and cases.

In bash 3.2, there is no module system, no scope isolation, and no cleanup-on-exit guarantee. When fixture A kindles modules and fixture B kindles the same modules, the kindle guards (`Z«PREFIX»_KINDLED`) from fixture A block fixture B. Subshells solve this: all state — kindle guards, sourced functions, variables — dies at the `)` boundary.

This is discipline encoded as structure.

### Three-Layer Model

Test execution uses three layers separated by two subshell boundaries:

```
Parent Shell (runner layer)
 ├─ _litmus_predicate — precondition check (0=proceed, 1=skip)
 ├─ Fixture iteration and status tracking
 └─ Reporting (pass/fail/skip counts)
     │
     └─ Fixture Subshell (fixture boundary)
         ├─ _baste — kindle, source, configure
         ├─ Fixture temp dir creation
         └─ Case iteration
             │
             └─ Case Subshell (_tcase boundary)
                 ├─ Per-case BUT_TEMP_DIR and BUTE_BURV_ROOT
                 ├─ Assertions and verification
                 └─ Exit status communicated to fixture
```

### Vocabulary

**Fixture boundary** — the fixture isolation boundary:
- Runs in a subshell (state dies at boundary)
- Baste runs inside (visible to cases, dies with fixture)
- Litmus runs outside (parent decides whether to enter)
- Communicates only exit status to the runner
- Kindle guards catch double-kindle within a fixture; subshell boundary prevents cross-fixture contamination

**`_tcase`** — the case isolation boundary:
- Runs in a subshell within the fixture subshell
- Inherits baste state, cannot mutate sibling state
- Communicates only exit status and stdio
- Each case gets isolated temp dir and BURV root

**`_litmus_predicate`** — fixture precondition (parent layer):
- Runs in parent shell, outside the fixture boundary
- Returns 0 to proceed, 1 to skip fixture
- Must not kindle or source modules — state would persist to next fixture
- Reusable: multiple fixtures share one litmus; composable: one litmus calls others
- Registered as second argument to `butr_fixture_enroll`

**`_baste`** — fixture preparation function (fixture layer):
- Runs inside the fixture subshell
- Kindles modules, sources dependencies, sets configuration
- State visible to all cases within this fixture, dies at fixture boundary
- Registered as third argument to `butr_fixture_enroll`

### Allowed Operations by Layer

| Operation | Parent | Fixture | `_tcase` |
|-----------|--------|---------|----------|
| Litmus predicate check | ✅ | — | — |
| `z«prefix»_kindle` | — | ✅ | — |
| `source` module files | — | ✅ | — |
| Baste configuration | — | ✅ | — |
| Case iteration | — | ✅ | — |
| Assertions (`buto_*_expect_*`) | — | — | ✅ |
| `zbuto_invoke` (capture) | — | — | ✅ |
| Tabtarget invocation | — | — | ✅ |
| Status tracking (pass/fail/skip) | ✅ | — | — |

### Communication Across Boundaries

| Boundary | Crosses | Does Not Cross |
|----------|---------|----------------|
| Fixture → parent | Exit status (0=pass, non-zero=fail, 2=skip), stdout/stderr | Variables, kindle state, sourced functions |
| `_tcase` → fixture | Exit status (0=pass, non-zero=fail), stdout/stderr | Variable mutations, BURV roots, temp dirs |

### Naming Conventions

| Element | Pattern | Example |
|---------|---------|---------|
| Testbench file | `«prefix»tb_testbench.sh` | `rbtb_testbench.sh` |
| Test case file | `«prefix»tc«xx»_«Name».sh` | `rbtckk_KickTires.sh` |
| Litmus predicate | `z«tb»_«name»_litmus_predicate` | `zrbtb_container_runtime_litmus_predicate` |
| Baste function | `z«tb»_«name»_baste` | `zrbtb_ark_baste` |
| Case function | `«tc»_«name»_tcase` | `rbtckk_false_tcase` |
| Fixture enrollment | `butr_fixture_enroll` | — |
| Suite enrollment | `butr_suite_enroll` | Sets sweep suite context |
| Case enrollment | `butr_case_enroll` | — |

### Compliance Checking

The naming conventions enable grep-based auditing:

```bash
# Find all litmus predicates
grep -rn '_litmus_predicate' Tools/

# Find all baste functions
grep -rn '_baste' Tools/

# Find all case functions
grep -rn '_tcase' Tools/

# Verify enrollment matches naming — litmus/baste/case functions should
# appear both in enrollment calls and as function definitions
grep -rn 'butr_fixture_enroll\|butr_case_enroll' Tools/
```

---

## Shellcheck Integration

BCG uses [shellcheck](https://www.shellcheck.net) for static analysis of bash source with a curated suppress list that eliminates false positives caused by BCG's module architecture.

### Architecture

| Artifact | Location | Purpose |
|----------|----------|---------|
| `busc_shellcheckrc` | `Tools/buk/` | BCG-structural suppressions with rationale |
| `buq_cli.sh` | `Tools/buk/` | BUK qualification CLI hosting shellcheck command |
| `buw-qsc` | Workbench route | Invokes `buq_shellcheck` |
| `buw-qsc.QualifyShellCheck.sh` | `tt/` | Tabtarget for shellcheck qualification |

### Suppressed Codes (BCG-structural)

These codes are false positives caused by BCG's dynamic sourcing, cross-module constants, indirect dispatch, and template-generation patterns:

| Code | Reason |
|------|--------|
| SC1090 | Can't follow non-constant source — BCG furnish pattern |
| SC1091 | Not following sourced file — same root cause |
| SC2034 | Variable appears unused — cross-module kindle/literal constants |
| SC2154 | Variable not assigned — inverse of SC2034 |
| SC2155 | Declare and assign separately — BCG blesses single-line `$(<file)` |
| SC2153 | Possible misspelling — cross-module prefix similarity |
| SC2329 | Function never invoked — indirect invocation via source/dispatch |
| SC2016 | Expressions in single quotes — template-generation patterns echo single-quoted strings containing `${}` into generated files (tabtargets, launchers, Dockerfiles) |
| SC2254 | Unquoted case pattern — BCG glob-matching patterns intentionally use unquoted variables in case statements |
| SC2059 | Variable in printf format string — BCG table-formatting patterns use `local -r` format constants shared across header, separator, and data rows |

### Policy

1. **No inline directives.** All structural suppressions live in `busc_shellcheckrc`. See **Prohibited Constructs § Inline Shellcheck Directives**.
2. **Genuine findings must be fixed.** If shellcheck flags code that isn't BCG-structural, the code has a real issue.
3. **New structural codes** discovered during adoption should be added to `busc_shellcheckrc` with rationale — never suppressed inline.

---

## Fading Memory — Superseded Conventions

This section documents conventions that have been replaced. When encountering legacy code using these patterns, transform to the current convention. Do not bulk-rename across the codebase — migrate opportunistically when touching affected files.

### FM-001: _register → _enroll

**Superseded pattern:**
- Function suffix: `_register`
- Return variables: `z1z_«prefix»_«term»`
- Array naming: no `_roll` suffix convention
- Accessor naming: ad-hoc `_get_*` functions

**Recognition — legacy code looks like:**
- Functions named `*_register` that populate parallel arrays
- Variables starting with `z1z_` used as return channels
- Parallel arrays without `_roll` suffix (e.g., `zbuz_colophons` instead of `z_buz_colophon_roll`)
- Accessor functions named `*_get_*` without `_recite` suffix

**Current convention:**
- Function suffix: `_enroll` (kindle-only)
- Return variables: `z_«funcname»_«retval»` (function name embedded verbatim)
- Array naming: `z_«prefix»_«name»_roll`
- Accessor suffix: `_recite` (read-only, never mutates)

**Known legacy sites:**
- `buz_register` in `Tools/buk/buz_zipper.sh`
- `butr_register` in `Tools/buk/butr_registry.sh`

**Migration:** When touching a file that uses the old pattern, transform to new. Enroll calls must move into kindle. Return variables must use `z_«funcname»_«retval»`. Arrays must gain `_roll` suffix using `z_«prefix»_` convention. Accessors must gain `_recite` suffix.
