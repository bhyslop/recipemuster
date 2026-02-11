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
  test -d          "${z_dirpath}"               || buc_die "Required directory not found: ${z_dirpath}"
  test -z "$(ls -A "${z_dirpath}" 2>/dev/null)" || buc_die "Directory must be empty: ${z_dirpath}"
}

# Generic environment variable wrapper
buv_env_wrapper() {
  local z_func_name="${1:-}"
  local z_varname="${2:-}"
  eval "local z_val=\${${z_varname}:-}" || buc_die "Variable '${z_varname}' is not defined"
  shift 2

  "${z_func_name}" "${z_varname}" "${z_val}" "$@"
}

# Generic optional wrapper - returns empty if value is empty
buv_opt_wrapper() {
  local z_func_name="${1:-}"
  local z_varname="${2:-}"
  eval "local z_val=\${${z_varname}:-}" || buc_die "Variable '${z_varname}' is not defined"

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
    echo "${z_val}"
    return 0
  fi

  # Otherwise must not be empty
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"

  # Check length constraints if max provided
  if test -n "${z_max}"; then
    test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
    test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"
  fi

  echo "${z_val}"
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
    echo "${z_val}"
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

  echo "${z_val}"
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
    echo "${z_val}"
    return 0
  fi

  # Non-empty and length window
  test -n "${z_val}" || buc_die "${z_varname} must not be empty"
  test "${#z_val}" -ge "${z_min}" || buc_die "${z_varname} must be at least ${z_min} chars, got '${z_val}' (${#z_val})"
  test "${#z_val}" -le "${z_max}" || buc_die "${z_varname} must be no more than ${z_max} chars, got '${z_val}' (${#z_val})"

  # Pattern: ^[a-z][a-z0-9-]*[a-z0-9]$
  echo "${z_val}" | grep -qE '^[a-z][a-z0-9-]*[a-z0-9]$' || \
    buc_die "${z_varname} must match ^[a-z][a-z0-9-]*[a-z0-9]$ (lowercase letters, digits, hyphens; start with a letter; end with letter/digit), got '${z_val}'"

  echo "${z_val}"
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
    echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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

  echo "${z_val}"
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
buv_opt_range()              { buv_opt_wrapper "buv_val_decimal"          "$@"; }
buv_opt_ipv4()               { buv_opt_wrapper "buv_val_ipv4"             "$@"; }
buv_opt_cidr()               { buv_opt_wrapper "buv_val_cidr"             "$@"; }
buv_opt_domain()             { buv_opt_wrapper "buv_val_domain"           "$@"; }
buv_opt_port()               { buv_opt_wrapper "buv_val_port"             "$@"; }

# eof

