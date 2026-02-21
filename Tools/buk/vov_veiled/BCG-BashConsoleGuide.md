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

---

## Function Patterns

### Boilerplate Functions (one per module/CLI)

| Function             | Location       | First Line                                         | Can Source?       | Can use buc_step? | Purpose                                          |
|----------------------|----------------|----------------------------------------------------|-------------------|-------------------|--------------------------------------------------|
| `z«prefix»_kindle`   | Implementation | `test -z "${Z«PREFIX»_KINDLED:-}" \|\| buc_die`    | Yes (credentials) | No (use buc_log_*)| Define all kindle constants, set module state    |
| `z«prefix»_sentinel` | Implementation | `test "${Z«PREFIX»_KINDLED:-}" = "1" \|\| buc_die` | No                | No                | Guard all other functions                        |
| `z«prefix»_furnish`  | CLI only       | `buc_doc_env...`                                   | Yes (configs)     | No                | Document env vars, source configs, call kindle   |
| Module header        | Implementation | `test -z "${Z«PREFIX»_SOURCED:-}" \|\| buc_die`    | No                | N/A               | Prevent multiple inclusion                       |
| CLI header           | CLI            | `set -euo pipefail`                                | Yes (all deps)    | N/A               | Source all dependencies                          |

**Kindle constant**: Any variable — internal (`Z«PREFIX»_SCREAMING_NAME`) or public (`«PREFIX»_SCREAMING_NAME`) — defined exclusively within the kindle function. No other function — including enroll, setup, or helper functions — may assign to kindle constants. This ensures module state is fully determined at kindle time and visible in one place.

**Literal constant**: A public variable (`«PREFIX»_lower_name`) defined at module top level, immediately after the `Z«PREFIX»_SOURCED=1` guard. Literal constants must be pure string literals with **no variable expansion, no computation, and no runtime dependency**. They are available immediately after sourcing — no kindle required. Use `SCREAMING` case for the prefix (module identity) and `lower_snake` case for the name to visually distinguish from kindle constants. This enables sourcing chains where a literal constant from module A provides the path for sourcing module B's config in the same CLI header block.

**KINDLED must be last**: `Z«PREFIX»_KINDLED=1` must be the final statement in kindle. Since sentinel checks this variable, setting it last guarantees all kindle constants, roll arrays, and enroll calls are complete before the module is considered operational. Any function calling sentinel before kindle finishes will correctly fail.

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

### Template 2: CLI Entry Point

BUD variables are provided by dispatch. Validate and document only those used by the implementation.

```bash
# «shebang»
# «copyright»

set -euo pipefail

Z«PREFIX»_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${Z«PREFIX»_CLI_SCRIPT_DIR}/buc_command.sh"
source "${Z«PREFIX»_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${Z«PREFIX»_CLI_SCRIPT_DIR}/«prefix»_«name».sh"

z«prefix»_furnish() {
  
  # Document only the BUD variables actually used
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env "BURD_NOW_STAMP        " "Unique string between invocations"
  buc_doc_env "BURD_OUTPUT_DIR       " "Directory for command outputs"

  # Document module specific environment variables needed
  buc_doc_env "«PREFIX»_XXX         " "Module specific environment variable"
  buc_doc_env "«PREFIX»_YYY         " "Module specific environment variable"
  buc_doc_env "«PREFIX»_REGIME_FILE " "Module specific configuration file"

  # Source regime files, if any
  source "${«PREFIX»_REGIME_FILE}" || buc_die "Failed to source regime file"

  z«prefix»_kindle
}

buc_execute «prefix»_ "«module_description_phrase»" z«prefix»_furnish "$@"

# eof
```

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

# ✅ Reading lines with 'read'
while IFS= read -r z_line; do
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

### String Usage

```bash
# Single-line HERE-string allowed for piping variables
jq '.field' <<<"${z_json_data}"
grep "pattern" <<<"${z_content}"

# Multi-line HERE-strings and HERE-docs prohibited
```

## Special Function Definitions

### Special Function Patterns

This table defines scope: «prefix»_* is public, z«prefix»_* is internal.

| Type      | Pattern                        | First Line           | Returns                      | buc_step                  | Contract                            |
|-----------|--------------------------------|----------------------|------------------------------|---------------------------|-------------------------------------|
| Predicate | `[z]«prefix»_«name»_predicate` | `z«prefix»_sentinel` | 0=true, 1=false              | No (use buc_log_«source») | Never dies, status only             |
| Capture   | `[z]«prefix»_«name»_capture`   | `z«prefix»_sentinel` | stdout once at end or exit 1 | No (use buc_log_«source») | Clean error handling, single return |
| Enroll    | `[z]«prefix»_[«scope»_]enroll` | `z«prefix»_sentinel` | `z_«funcname»_«retval»` vars | No (use buc_log_«source») | Mutates rolls (parallel arrays) in kindle only; returns via variables |
| Recite    | `[z]«prefix»_«what»_recite`    | `z«prefix»_sentinel` | stdout or exit 1             | No (use buc_log_«source») | Read-only access to rolls; never mutates |
| Tsuite init | `z«tb»_«name»_tsuite_init`  | —                    | 0=proceed, non-zero=skip     | No                        | Parent layer; no kindle, no source  |
| Tsuite setup | `z«tb»_«name»_tsuite_setup` | —                   | — (side effects only)        | Optional                  | Suite subshell; kindle, source, configure |
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
- **_tsuite_init**: Suite precondition check in parent shell (return non-zero to skip)
- **_tsuite_setup**: Suite initialization inside `_tsuite` subshell (kindle, source, configure)
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
| Literal constant (public)    | `«PREFIX»_«name»`            | `RBCC_rbrr_file`             | Impl     | lower_snake (multi-word)      |
| Local parameters             | `z_«name»`                   | `z_vm_name`, `z_force_flag`  | Both     | snake_case (multi-word)       |
| Enroll functions             | `[z]«prefix»_[«scope»_]enroll` | `«prefix»_enroll`          | Impl     | kindle-only                   |
| Recite functions             | `[z]«prefix»_«what»_recite`  | `«prefix»_target_recite`     | Impl     | read-only, never mutates      |
| Roll arrays                  | `z_«prefix»_«name»_roll`     | `z_rbv_target_roll`          | Impl     | snake_case, kindle-only       |
| Enroll return vars           | `z_«funcname»_«retval»`      | `z_«prefix»_enroll_name`     | Impl     | snake_case (func name verbatim)|
| Testbench file               | `«prefix»tb_testbench.sh`    | `rbtb_testbench.sh`          | N/A      | fixed name                    |
| Test case file               | `«prefix»tc«xx»_«Name».sh`  | `rbtckk_KickTires.sh`        | N/A      | PascalCase name               |
| Suite init function          | `z«tb»_«name»_tsuite_init`  | `zrbtb_kick_tsuite_init`     | Testbench| snake_case, parent layer      |
| Suite setup function         | `z«tb»_«name»_tsuite_setup` | `zrbtb_kick_tsuite_setup`    | Testbench| snake_case, suite layer       |
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

**Counterpoint**: Test execution uses subshells for principled isolation — see **Test Execution Patterns**. Subshells are also present in `$()` command substitution (`_capture` functions) and status capture patterns (`( ... ) || z_status=$?`). The prohibition is specific to error-handling blocks where `exit`/`return` must reach the calling shell.

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

---

## Array Safety Under `set -u`

Under `set -u`, `"${array[@]}"` on an empty array triggers "unbound variable" in bash 3.2.

**Primary pattern: index iteration** — works safely on empty arrays, no guard needed:

```bash
# ✅ Safe: index iteration works on empty arrays under set -u
for z_i in "${!z_«prefix»_name_roll[@]}"; do
  echo "${z_«prefix»_name_roll[$z_i]}" || buc_die "Failed to echo"
done
```

**Acceptable alternative: guarded value iteration** — check array is non-empty before expanding. The `(( expr ))` arithmetic command returns exit 0 when expr is non-zero, exit 1 when zero — it works in bash 3.2 and is the idiomatic guard for array size checks:

```bash
# ✅ Acceptable: guard value iteration with size check
if (( ${#z_«prefix»_name_roll[@]} )); then
  for z_val in "${z_«prefix»_name_roll[@]}"; do
    echo "${z_val}" || buc_die "Failed to echo"
  done
fi
```

**Anti-pattern:**

```bash
# ❌ Unsafe: value iteration fails on empty arrays under set -u in bash 3.2
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
while IFS= read -r z_item; do
    complex_function "${z_item}"    # If this touches stdin, remaining items vanish
done < "${z_temp_file}"
```

### ✅ Pattern: Load-Then-Iterate

**Always** read file contents into an array first, then iterate. The file is fully consumed and closed before any loop body code runs.

```bash
# ✅ Load phase — file consumed and closed
local z_items=()
while IFS= read -r z_line; do
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
| CLI file header      | All dependencies           | Module loading                                       |
| CLI Furnish Function | Config files               | Environment setup                                    |
| Internal functions   | Credentials when necessary | Context-specific secrets (with documented rationale) |

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

### Using BVU Validators

```bash
# Validate environment variables
buv_env_string "VAR_NAME" 1 100 "default"
buv_env_bool   "FLAG_VAR" "0"
buv_env_ipv4   "IP_VAR"

# Validate parameters (dies on failure, no return value)
buv_val_xname "name" "${z_input_name}" 3 50
```

---

## Quick Reference Decision Matrix

| Situation           | Use This Pattern                                                 |
|---------------------|------------------------------------------------------------------|
| Module structure    | Always split: implementation + CLI                               |
| Need return value   | Check error handling decision tree                               |
| Simple validation   | `test ... \|\| buc_die`                                          |
| Complex validation  | `buv_val_*` functions                                            |
| User feedback       | `buc_step` for major milestones, `buc_log_«source»` for forensic trail    |
| Runtime information | `buc_log_«source»` for details, `buc_step` for milestones, never comments |
| Show commands       | `buc_code`                                                       |
| Non-fatal issue     | `buc_warn`                                                       |
| Fatal error         | `buc_die`                                                        |
| Success message     | `buc_success`                                                    |
| Variable expansion  | Always `"${var}"`                                                |
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
- [ ] CLI file sources all dependencies in header
- [ ] External code accesses module through CLI only (no direct `z*_kindle()` calls from outside)

### Required Functions
- [ ] `z«prefix»_kindle` - first line: `test -z "${Z«PREFIX»_KINDLED:-}" || buc_die`
- [ ] `z«prefix»_sentinel` - first line: `test "${Z«PREFIX»_KINDLED:-}" = "1" || buc_die`
- [ ] `z«prefix»_furnish` (CLI only) - documents env vars, sources configs, calls kindle
- [ ] All public functions start with sentinel check
- [ ] All internal helpers prefixed with `z«prefix»_`

### Variable Management
- [ ] All kindle constants (internal `Z«PREFIX»_SCREAMING` and public `«PREFIX»_SCREAMING`) defined exclusively in kindle
- [ ] No kindle constant assignments outside kindle function
- [ ] Literal constants (`«PREFIX»_lower_name`) are pure string literals with no `${}` expansion, placed after `SOURCED` guard
- [ ] All local variables use `z_` prefix
- [ ] All expansions use `"${var}"` pattern (braced, quoted)
- [ ] Parameters use `"${1:-}"` pattern for defensive programming
- [ ] Module state variable `Z«PREFIX»_KINDLED=1` is the last statement in kindle
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
- [ ] Error blocks use `{ ...; }` not `( ... )` — no `|| (... exit ...)` patterns

### Command Substitution Rules
- [ ] NO command substitution except `$(<file)` builtin and `_capture` functions
- [ ] Temp files used instead of complex command substitution
- [ ] `$(<file)` always followed by validation
- [ ] `_capture` functions properly named with suffix

### Loop Safety
- [ ] All while-read loops use load-then-iterate pattern
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
- [ ] Major operations use `buc_step` (always visible)

### Sourcing Restrictions
- [ ] Only CLI header sources dependencies
- [ ] Only `z«prefix»_furnish` sources config files
- [ ] Credential sourcing documented and never writes to disk
- [ ] No other sourcing anywhere

### Code Quality
- [ ] Prefer bash builtins over external tools (parameter expansion vs sed/awk)
- [ ] Use BCU utilities instead of raw bash (buc_die vs echo+exit)
- [ ] Use BVU validators for input validation
- [ ] Consistent error messages (specific and actionable)
- [ ] Proper temp file naming with module prefix
- [ ] No silent failures or ignored conditions
- [ ] Temp files never deleted in module code — preserved for forensics

### Enterprise Safety
- [ ] Crash-fast principle applied throughout
- [ ] No elegant-but-fragile pipeline patterns
- [ ] Secrets never written to disk (use `_capture` functions)
- [ ] Temp files make failures visible and debuggable
- [ ] Every potential error explicitly handled
- [ ] Abstraction layers used (BCU/BVU utilities)

---

## Test Execution Patterns

### Principled Subshell Use

BCG prohibits subshells for error handling (see **Subshell Exit vs Brace-Group**). Test execution is the principled counterpoint: subshells provide **isolation boundaries** that prevent state leakage between suites and cases.

In bash 3.2, there is no module system, no scope isolation, and no cleanup-on-exit guarantee. When suite A kindles modules and suite B kindles the same modules, the kindle guards (`Z«PREFIX»_KINDLED`) from suite A block suite B. Subshells solve this: all state — kindle guards, sourced functions, variables — dies at the `)` boundary.

This is discipline encoded as structure.

### Three-Layer Model

Test execution uses three layers separated by two subshell boundaries:

```
Parent Shell (runner layer)
 ├─ _tsuite_init — precondition check (return non-zero to skip)
 ├─ Suite iteration and status tracking
 └─ Reporting (pass/fail/skip counts)
     │
     └─ Suite Subshell (_tsuite boundary)
         ├─ _tsuite_setup — kindle, source, configure
         ├─ Suite temp dir creation
         └─ Case iteration
             │
             └─ Case Subshell (_tcase boundary)
                 ├─ Per-case BUT_TEMP_DIR and BUTE_BURV_ROOT
                 ├─ Assertions and verification
                 └─ Exit status communicated to suite
```

### Vocabulary

**`_tsuite`** — the suite isolation boundary:
- Runs in a subshell (state dies at boundary)
- Setup runs inside (visible to cases, dies with suite)
- Init/precondition runs outside (parent decides whether to enter)
- Communicates only exit status to the runner
- Kindle guards catch double-kindle within a suite; subshell boundary prevents cross-suite contamination

**`_tcase`** — the case isolation boundary:
- Runs in a subshell within the `_tsuite` subshell
- Inherits setup state, cannot mutate sibling state
- Communicates only exit status and stdio
- Each case gets isolated temp dir and BURV root

**`_tsuite_init`** — precondition function (parent layer):
- Runs in parent shell, outside the `_tsuite` boundary
- Returns 0 to proceed, non-zero to skip suite
- Must not kindle or source modules — state would persist to next suite
- Registered as second argument to `butr_suite_enroll`

**`_tsuite_setup`** — suite initialization function (suite layer):
- Runs inside the `_tsuite` subshell
- Kindles modules, sources dependencies, sets configuration
- State visible to all cases within this suite, dies at suite boundary
- Registered as third argument to `butr_suite_enroll`

### Allowed Operations by Layer

| Operation | Parent | `_tsuite` | `_tcase` |
|-----------|--------|-----------|----------|
| Init / precondition check | ✅ | — | — |
| `z«prefix»_kindle` | — | ✅ | — |
| `source` module files | — | ✅ | — |
| Setup configuration | — | ✅ | — |
| Case iteration | — | ✅ | — |
| Assertions (`buto_*_expect_*`) | — | — | ✅ |
| `zbuto_invoke` (capture) | — | — | ✅ |
| Tabtarget invocation | — | — | ✅ |
| Status tracking (pass/fail/skip) | ✅ | — | — |

### Communication Across Boundaries

| Boundary | Crosses | Does Not Cross |
|----------|---------|----------------|
| `_tsuite` → parent | Exit status (0=pass, non-zero=fail, 2=skip), stdout/stderr | Variables, kindle state, sourced functions |
| `_tcase` → `_tsuite` | Exit status (0=pass, non-zero=fail), stdout/stderr | Variable mutations, BURV roots, temp dirs |

### Naming Conventions

| Element | Pattern | Example |
|---------|---------|---------|
| Testbench file | `«prefix»tb_testbench.sh` | `rbtb_testbench.sh` |
| Test case file | `«prefix»tc«xx»_«Name».sh` | `rbtckk_KickTires.sh` |
| Init function | `z«tb»_«name»_tsuite_init` | `zrbtb_kick_tsuite_init` |
| Setup function | `z«tb»_«name»_tsuite_setup` | `zrbtb_kick_tsuite_setup` |
| Case function | `«tc»_«name»_tcase` | `rbtckk_false_tcase` |
| Suite enrollment | `butr_suite_enroll` | — |
| Case enrollment | `butr_case_enroll` | — |

### Compliance Checking

The naming conventions enable grep-based auditing:

```bash
# Find all suite init functions
grep -rn '_tsuite_init' Tools/

# Find all suite setup functions
grep -rn '_tsuite_setup' Tools/

# Find all case functions
grep -rn '_tcase' Tools/

# Verify enrollment matches naming — init/setup/case functions should
# appear both in enrollment calls and as function definitions
grep -rn 'butr_suite_enroll\|butr_case_enroll' Tools/
```

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
