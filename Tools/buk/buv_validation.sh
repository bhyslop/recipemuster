#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
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
# Bash Validation Utility Library
# Compatible with Bash 3.2 (e.g., macOS default shell)

# Multiple inclusion guard
test -n "${ZBUV_INCLUDED:-}" && return 0
ZBUV_INCLUDED=1

# Source the console utility library
ZBUV_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZBUV_SCRIPT_DIR}/buc_command.sh"

buv_file_exists() {
  local z_filepath="${1:-}"
  test -f "${z_filepath}" || buc_die "Required file not found: ${z_filepath}"
}

buv_dir_exists() {
  local z_dirpath="${1:-}"
  test -d "${z_dirpath}" || buc_die "Required directory not found: ${z_dirpath}"
}

buv_dir_empty() {
  local z_dirpath="${1:-}"
  test -d "${z_dirpath}" || buc_die "Required directory not found: ${z_dirpath}"
  local z_check_file
  z_check_file=$(mktemp)
  find "${z_dirpath}" -maxdepth 1 -mindepth 1 -print -quit > "${z_check_file}"
  test ! -s "${z_check_file}" || { rm -f "${z_check_file}"; buc_die "Directory must be empty: ${z_dirpath}"; }
  rm -f "${z_check_file}"
}

# Generic environment variable wrapper
buv_env_wrapper() {
  local z_func_name="${1:-}"
  local z_varname="${2:-}"
  echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' || buc_die "Invalid variable name: '${z_varname}'"
  eval "local z_val=\${${z_varname}:-}"
  shift 2

  "${z_func_name}" "${z_varname}" "${z_val}" "$@"
}

# Generic optional wrapper - returns empty if value is empty
buv_opt_wrapper() {
  local z_func_name="${1:-}"
  local z_varname="${2:-}"
  echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' || buc_die "Invalid variable name: '${z_varname}'"
  eval "local z_val=\${${z_varname}:-}"

  # Empty is always valid for optional
  test -z "${z_val}" && return 0

  shift 2
  "${z_func_name}" "${z_varname}" "${z_val}" "$@"
}

# String validator with optional length constraints
buv_val_string() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_min="${3:-}"
  local z_max="${4:-}"
  local z_default="${5-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_min}"     || buc_die "min parameter is required for varname '${z_varname}'"
  test -n "${z_max}"     || buc_die "max parameter is required for varname '${z_varname}'"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  # Allow empty if min=0
  if test "${z_min}" = "0" && test -z "${z_val}"; then
    return 0
  fi

  # Otherwise must not be empty
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  # Check length constraints if max provided
  if test -n "${z_max}"; then
    test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
    test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"
  fi
}

# Cross-context name validator (system-safe identifier)
buv_val_xname() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_min="${3:-}"
  local z_max="${4:-}"
  local z_default="${5-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_min}"     || buc_die "min parameter is required for varname '${z_varname}'"
  test -n "${z_max}"     || buc_die "max parameter is required for varname '${z_varname}'"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  # Allow empty if min=0
  if test "${z_min}" = "0" && test -z "${z_val}"; then
    return 0
  fi

  # Otherwise must not be empty
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  # Check length constraints
  test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
  test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"

  # Must start with letter and contain only allowed chars
  echo "${z_val}" | grep -qE '^[a-zA-Z][a-zA-Z0-9_-]*$' || \
    buc_die "${z_varname} must start with letter and contain only letters, numbers, underscore, hyphen, got '${z_val}'"
}

# Google-style resource identifier (lowercase, digits, hyphens)
# Must start with a letter, end with letter/digit.
# Examples: GCP project IDs, GAR repo IDs.
buv_val_gname() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_min="${3:-}"
  local z_max="${4:-}"
  local z_default="${5-}"

  # Required params
  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_min}"     || buc_die "min parameter is required for varname '${z_varname}'"
  test -n "${z_max}"     || buc_die "max parameter is required for varname '${z_varname}'"

  # Defaulting
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  # Allow empty if min=0
  if test "${z_min}" = "0" && test -z "${z_val}"; then
    return 0
  fi

  # Non-empty and length window
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
  test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"

  # Pattern: ^[a-z][a-z0-9-]*[a-z0-9]$
  echo "${z_val}" | grep -qE '^[a-z][a-z0-9-]*[a-z0-9]$' || \
    buc_die "${z_varname} must match ^[a-z][a-z0-9-]*[a-z0-9]$ (lowercase letters, digits, hyphens; start with a letter; end with letter/digit), got '${z_val}'"
}

# Fully Qualified Image Name component validator
buv_val_fqin() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_min="${3:-}"
  local z_max="${4:-}"
  local z_default="${5-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_min}"     || buc_die "min parameter is required for varname '${z_varname}'"
  test -n "${z_max}"     || buc_die "max parameter is required for varname '${z_varname}'"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  # Allow empty if min=0
  if test "${z_min}" = "0" && test -z "${z_val}"; then
    return 0
  fi

  # Otherwise must not be empty
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  # Check length constraints
  test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
  test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"

  # Allow letters, numbers, dots, hyphens, underscores, forward slashes, colons
  echo "${z_val}" | grep -qE '^[a-zA-Z0-9][a-zA-Z0-9:._/-]*$' || \
    buc_die "${z_varname} must start with letter/number and contain only letters, numbers, colons, dots, underscores, hyphens, forward slashes, got '${z_val}'"
}

# Boolean validator
buv_val_bool() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  test "${z_val}" = "0" || test "${z_val}" = "1" || buc_die "${z_varname} must be 0 or 1, got: '${z_val}'"
}

# Enumeration validator — value must be one of the listed choices
buv_val_enum() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  shift 2

  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  local z_allowed
  for z_allowed in "$@"; do
    test "${z_val}" = "${z_allowed}" && return 0
  done
  buc_die "${z_varname} must be one of: $*, got '${z_val}'"
}

# Decimal range validator
buv_val_decimal() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_min="${3:-}"
  local z_max="${4:-}"
  local z_default="${5-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"
  test -n "${z_min}"     || buc_die "min parameter is required for varname '${z_varname}'"
  test -n "${z_max}"     || buc_die "max parameter is required for varname '${z_varname}'"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  test "${z_val}" -ge "${z_min}" && test "${z_val}" -le "${z_max}" || buc_die "${z_varname} value '${z_val}' must be between ${z_min} and ${z_max}"
}

# IPv4 validator
buv_val_ipv4() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  echo "${z_val}" | grep -qE '^([0-9]{1,3}\.){3}[0-9]{1,3}$' || buc_die "${z_varname} has invalid IPv4 format: '${z_val}'"
}

# CIDR validator
buv_val_cidr() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  echo "${z_val}" | grep -qE '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$' || buc_die "${z_varname} has invalid CIDR format: '${z_val}'"
}

# Domain validator
buv_val_domain() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  echo "${z_val}" | grep -qE '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$' || buc_die "${z_varname} has invalid domain format: '${z_val}'"
}

# Port validator
buv_val_port() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Use default if value is empty and default provided
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  test "${z_val}" -ge 1 && test "${z_val}" -le 65535 || buc_die "${z_varname} value '${z_val}' must be between 1 and 65535"
}

# OCI/Docker image reference that MUST be digest-pinned
# Accepts:
#   - Any registry host: letters, digits, dots, hyphens; optional :port
#   - Repository path: one or more slash-separated lowercase segments [a-z0-9._-]
#   - Mandatory digest: @sha256:<64 lowercase hex>
#
# Examples:
#   docker.io/stedolan/jq@sha256:...
#   ghcr.io/anchore/syft@sha256:...
#   gcr.io/go-containerregistry/gcrane@sha256:...
#   us-central1-docker.pkg.dev/my-proj/my-repo/tool@sha256:...
buv_val_odref() {
  local z_varname="${1:-}"
  local z_val="${2:-}"
  local z_default="${3-}"

  test -n "${z_varname}" || buc_die "varname parameter is required"

  # Defaulting when allowed by caller
  if test -z "${z_val}" && test -n "${z_default}"; then
    z_val="${z_default}"
  fi

  # Must not be empty here (use buv_opt_odref for optional)
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  # Enforce digest-pinned image ref:
  #   host[:port]/repo(/subrepo)@sha256:64hex
  #   - host: [a-z0-9.-]+ with optional :port
  #   - each repo segment: [a-z0-9._-]+ (lowercase)
  #   - digest algo fixed to sha256 with 64 lowercase hex chars
  local z_re='^[a-z0-9.-]+(:[0-9]{2,5})?/([a-z0-9._-]+/)*[a-z0-9._-]+@sha256:[0-9a-f]{64}$'
  echo "${z_val}" | grep -Eq "${z_re}" || buc_die "${z_varname} has invalid image reference format (require host[:port]/repo@sha256:<64hex>), got '${z_val}'"
}

# List validators
buv_val_list_ipv4() {
  local z_varname="${1:-}"
  local z_val="${2:-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  test -z "${z_val}" && return 0

  local z_item_num=0
  local z_item
  for z_item in ${z_val}; do
    z_item_num=$((z_item_num + 1))
    echo "${z_item}" | grep -qE '^([0-9]{1,3}\.){3}[0-9]{1,3}$' || buc_die "${z_varname} item #${z_item_num} has invalid IPv4 format: '${z_item}'"
  done
}

buv_val_list_cidr() {
  local z_varname="${1:-}"
  local z_val="${2:-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  test -z "${z_val}" && return 0

  local z_item_num=0
  local z_item
  for z_item in ${z_val}; do
    z_item_num=$((z_item_num + 1))
    echo "${z_item}" | grep -qE '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$' || buc_die "${z_varname} item #${z_item_num} has invalid CIDR format: '${z_item}'"
  done
}

buv_val_list_domain() {
  local z_varname="${1:-}"
  local z_val="${2:-}"

  # Validate required parameters
  test -n "${z_varname}" || buc_die "varname parameter is required"

  test -z "${z_val}" && return 0

  local z_item_num=0
  local z_item
  for z_item in ${z_val}; do
    z_item_num=$((z_item_num + 1))
    echo "${z_item}" | grep -qE '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$' || buc_die "${z_varname} item #${z_item_num} has invalid domain format: '${z_item}'"
  done
}

# Environment variable validators
buv_env_string()             { buv_env_wrapper "buv_val_string"           "$@"; }
buv_env_xname()              { buv_env_wrapper "buv_val_xname"            "$@"; }
buv_env_gname()              { buv_env_wrapper "buv_val_gname"            "$@"; }
buv_env_fqin()               { buv_env_wrapper "buv_val_fqin"             "$@"; }
buv_env_bool()               { buv_env_wrapper "buv_val_bool"             "$@"; }
buv_env_enum()               { buv_env_wrapper "buv_val_enum"             "$@"; }
buv_env_decimal()            { buv_env_wrapper "buv_val_decimal"          "$@"; }
buv_env_ipv4()               { buv_env_wrapper "buv_val_ipv4"             "$@"; }
buv_env_cidr()               { buv_env_wrapper "buv_val_cidr"             "$@"; }
buv_env_domain()             { buv_env_wrapper "buv_val_domain"           "$@"; }
buv_env_port()               { buv_env_wrapper "buv_val_port"             "$@"; }
buv_env_odref()              { buv_env_wrapper "buv_val_odref"            "$@"; }

# Environment list validators
buv_env_list_ipv4()          { buv_env_wrapper "buv_val_list_ipv4"        "$@"; }
buv_env_list_cidr()          { buv_env_wrapper "buv_val_list_cidr"        "$@"; }
buv_env_list_domain()        { buv_env_wrapper "buv_val_list_domain"      "$@"; }

# Optional validators
buv_opt_bool()               { buv_opt_wrapper "buv_val_bool"             "$@"; }
buv_opt_enum()               { buv_opt_wrapper "buv_val_enum"             "$@"; }
buv_opt_range()              { buv_opt_wrapper "buv_val_decimal"          "$@"; }
buv_opt_ipv4()               { buv_opt_wrapper "buv_val_ipv4"             "$@"; }
buv_opt_cidr()               { buv_opt_wrapper "buv_val_cidr"             "$@"; }
buv_opt_domain()             { buv_opt_wrapper "buv_val_domain"           "$@"; }
buv_opt_port()               { buv_opt_wrapper "buv_val_port"             "$@"; }


# ---------------------------------------------------------------------------
# Enrollment infrastructure
# ---------------------------------------------------------------------------

zbuv_kindle() {
  test -z "${ZBUV_KINDLED:-}" || buc_die "Module buv already kindled"

  # Enrollment rolls (7 parallel arrays)
  z_buv_scope_roll=()
  z_buv_varname_roll=()
  z_buv_type_roll=()
  z_buv_gate_var_roll=()
  z_buv_gate_val_roll=()
  z_buv_p1_roll=()
  z_buv_p2_roll=()

  ZBUV_KINDLED=1
}

zbuv_sentinel() {
  test "${ZBUV_KINDLED:-}" = "1" || buc_die "Module buv not kindled - call zbuv_kindle first"
}

# Internal enrollment helper — all public enroll functions delegate here
# Usage: zbuv_enroll SCOPE VARNAME TYPE GATE_VAR GATE_VAL P1 P2
zbuv_enroll() {
  zbuv_sentinel

  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_type="${3:-}"
  local z_gate_var="${4:-}"
  local z_gate_val="${5:-}"
  local z_p1="${6:-}"
  local z_p2="${7:-}"

  test -n "${z_scope}"   || buc_die "zbuv_enroll: scope required"
  echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' || buc_die "zbuv_enroll: invalid variable name: '${z_varname}'"

  z_buv_scope_roll+=("${z_scope}")
  z_buv_varname_roll+=("${z_varname}")
  z_buv_type_roll+=("${z_type}")
  z_buv_gate_var_roll+=("${z_gate_var}")
  z_buv_gate_val_roll+=("${z_gate_val}")
  z_buv_p1_roll+=("${z_p1}")
  z_buv_p2_roll+=("${z_p2}")
}

# Public enrollment functions — scalar types

buv_string_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "string" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_xname_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "xname" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_gname_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "gname" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_fqin_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "fqin" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_bool_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "bool" "${z_gate_var}" "${z_gate_val}" "" ""
}

buv_enum_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  shift 4
  zbuv_enroll "${z_scope}" "${z_varname}" "enum" "${z_gate_var}" "${z_gate_val}" "$*" ""
}

buv_decimal_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "decimal" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_odref_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "odref" "${z_gate_var}" "${z_gate_val}" "" ""
}

buv_ipv4_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "ipv4" "${z_gate_var}" "${z_gate_val}" "" ""
}

# Public enrollment functions — list types

buv_list_string_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "list_string" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

buv_list_ipv4_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "list_ipv4" "${z_gate_var}" "${z_gate_val}" "" ""
}

buv_list_gname_enroll() {
  local z_scope="${1:-}"
  local z_varname="${2:-}"
  local z_gate_var="${3:-}"
  local z_gate_val="${4:-}"
  local z_p1="${5:-}"
  local z_p2="${6:-}"
  zbuv_enroll "${z_scope}" "${z_varname}" "list_gname" "${z_gate_var}" "${z_gate_val}" "${z_p1}" "${z_p2}"
}

# Internal check predicate — validates a single enrolled variable by roll index.
# Returns 0 on pass (or gated-out), 1 on fail.
# Sets ZBUV_CHECK_ERROR with detail on fail (or "gated-out" when gate doesn't match).
zbuv_check_predicate() {
  zbuv_sentinel

  local z_idx="${1:-}"
  local z_varname="${z_buv_varname_roll[$z_idx]}"
  local z_type="${z_buv_type_roll[$z_idx]}"
  local z_gate_var="${z_buv_gate_var_roll[$z_idx]}"
  local z_gate_val="${z_buv_gate_val_roll[$z_idx]}"
  local z_p1="${z_buv_p1_roll[$z_idx]}"
  local z_p2="${z_buv_p2_roll[$z_idx]}"

  ZBUV_CHECK_ERROR=""

  # Gating check — if gated and gate doesn't match, skip (pass)
  if test -n "${z_gate_var}"; then
    local z_gate_actual="${!z_gate_var:-}"
    if test "${z_gate_actual}" != "${z_gate_val}"; then
      ZBUV_CHECK_ERROR="gated-out"
      return 0
    fi
  fi

  local z_val="${!z_varname:-}"

  case "${z_type}" in

    string)
      if test "${z_p1}" = "0" && test -z "${z_val}"; then
        return 0
      fi
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${#z_val}" -lt "${z_p1}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be at least ${z_p1} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      if test "${#z_val}" -gt "${z_p2}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be no more than ${z_p2} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      ;;

    xname)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${#z_val}" -lt "${z_p1}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be at least ${z_p1} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      if test "${#z_val}" -gt "${z_p2}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be no more than ${z_p2} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      echo "${z_val}" | grep -qE '^[a-zA-Z][a-zA-Z0-9_-]*$' || {
        ZBUV_CHECK_ERROR="${z_varname} must start with letter and contain only letters, numbers, underscore, hyphen, got '${z_val}'"
        return 1
      }
      ;;

    gname)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${#z_val}" -lt "${z_p1}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be at least ${z_p1} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      if test "${#z_val}" -gt "${z_p2}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be no more than ${z_p2} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      echo "${z_val}" | grep -qE '^[a-z][a-z0-9-]*[a-z0-9]$' || {
        ZBUV_CHECK_ERROR="${z_varname} must match ^[a-z][a-z0-9-]*[a-z0-9]$ (lowercase letters, digits, hyphens; start with a letter; end with letter/digit), got '${z_val}'"
        return 1
      }
      ;;

    fqin)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${#z_val}" -lt "${z_p1}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be at least ${z_p1} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      if test "${#z_val}" -gt "${z_p2}"; then
        ZBUV_CHECK_ERROR="${z_varname} must be no more than ${z_p2} chars, got '${z_val}' (${#z_val})"
        return 1
      fi
      echo "${z_val}" | grep -qE '^[a-zA-Z0-9][a-zA-Z0-9:._/-]*$' || {
        ZBUV_CHECK_ERROR="${z_varname} must start with letter/number and contain only letters, numbers, colons, dots, underscores, hyphens, forward slashes, got '${z_val}'"
        return 1
      }
      ;;

    bool)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${z_val}" != "0" && test "${z_val}" != "1"; then
        ZBUV_CHECK_ERROR="${z_varname} must be 0 or 1, got: '${z_val}'"
        return 1
      fi
      ;;

    enum)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      local z_choice
      local z_found=0
      for z_choice in ${z_p1}; do
        if test "${z_val}" = "${z_choice}"; then
          z_found=1
          break
        fi
      done
      if test "${z_found}" = "0"; then
        ZBUV_CHECK_ERROR="${z_varname} must be one of: ${z_p1}, got '${z_val}'"
        return 1
      fi
      ;;

    decimal)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      if test "${z_val}" -ge "${z_p1}" && test "${z_val}" -le "${z_p2}"; then
        return 0
      fi
      ZBUV_CHECK_ERROR="${z_varname} value '${z_val}' must be between ${z_p1} and ${z_p2}"
      return 1
      ;;

    odref)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      local z_re='^[a-z0-9.-]+(:[0-9]{2,5})?/([a-z0-9._-]+/)*[a-z0-9._-]+@sha256:[0-9a-f]{64}$'
      echo "${z_val}" | grep -Eq "${z_re}" || {
        ZBUV_CHECK_ERROR="${z_varname} has invalid image reference format (require host[:port]/repo@sha256:<64hex>), got '${z_val}'"
        return 1
      }
      ;;

    ipv4)
      if test -z "${z_val}"; then
        ZBUV_CHECK_ERROR="${z_varname} must not be empty"
        return 1
      fi
      echo "${z_val}" | grep -qE '^([0-9]{1,3}\.){3}[0-9]{1,3}$' || {
        ZBUV_CHECK_ERROR="${z_varname} has invalid IPv4 format: '${z_val}'"
        return 1
      }
      ;;

    list_string)
      local z_item
      local z_item_num=0
      for z_item in ${z_val}; do
        z_item_num=$((z_item_num + 1))
        if test "${#z_item}" -lt "${z_p1}"; then
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} must be at least ${z_p1} chars, got '${z_item}' (${#z_item})"
          return 1
        fi
        if test "${#z_item}" -gt "${z_p2}"; then
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} must be no more than ${z_p2} chars, got '${z_item}' (${#z_item})"
          return 1
        fi
      done
      ;;

    list_ipv4)
      local z_item
      local z_item_num=0
      for z_item in ${z_val}; do
        z_item_num=$((z_item_num + 1))
        echo "${z_item}" | grep -qE '^([0-9]{1,3}\.){3}[0-9]{1,3}$' || {
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} has invalid IPv4 format: '${z_item}'"
          return 1
        }
      done
      ;;

    list_gname)
      local z_item
      local z_item_num=0
      for z_item in ${z_val}; do
        z_item_num=$((z_item_num + 1))
        if test "${#z_item}" -lt "${z_p1}"; then
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} must be at least ${z_p1} chars, got '${z_item}' (${#z_item})"
          return 1
        fi
        if test "${#z_item}" -gt "${z_p2}"; then
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} must be no more than ${z_p2} chars, got '${z_item}' (${#z_item})"
          return 1
        fi
        echo "${z_item}" | grep -qE '^[a-z][a-z0-9-]*[a-z0-9]$' || {
          ZBUV_CHECK_ERROR="${z_varname} item #${z_item_num} must match ^[a-z][a-z0-9-]*[a-z0-9]$, got '${z_item}'"
          return 1
        }
      done
      ;;

    *)
      ZBUV_CHECK_ERROR="unknown type: ${z_type}"
      return 1
      ;;

  esac
}

# buv_vet SCOPE — iterate all enrolled vars in scope; die on first failure
buv_vet() {
  zbuv_sentinel

  local z_scope="${1:-}"
  test -n "${z_scope}" || buc_die "buv_vet: scope required"

  local z_i
  for z_i in "${!z_buv_scope_roll[@]}"; do
    test "${z_buv_scope_roll[$z_i]}" = "${z_scope}" || continue
    zbuv_check_predicate "${z_i}" || buc_die "${z_buv_varname_roll[$z_i]}: ${ZBUV_CHECK_ERROR}"
  done
}

# buv_report SCOPE "Label" — rich per-variable display; returns non-zero if any failed
buv_report() {
  zbuv_sentinel

  local z_scope="${1:-}"
  local z_label="${2:-}"
  test -n "${z_scope}" || buc_die "buv_report: scope required"
  test -n "${z_label}" || buc_die "buv_report: label required"

  local z_any_failed=0

  buc_step "${z_label}"

  local z_i
  for z_i in "${!z_buv_scope_roll[@]}"; do
    test "${z_buv_scope_roll[$z_i]}" = "${z_scope}" || continue

    local z_varname="${z_buv_varname_roll[$z_i]}"
    local z_type="${z_buv_type_roll[$z_i]}"
    local z_val="${!z_varname:-}"

    if zbuv_check_predicate "${z_i}"; then
      if test "${ZBUV_CHECK_ERROR}" = "gated-out"; then
        buc_step "  SKIP  ${z_varname} (gated)"
      else
        buc_step "  PASS  ${z_varname}=${z_val} [${z_type}]"
      fi
    else
      buc_step "  FAIL  ${z_varname}=${z_val} [${z_type}]: ${ZBUV_CHECK_ERROR}"
      z_any_failed=1
    fi
  done

  return "${z_any_failed}"
}

# eof

