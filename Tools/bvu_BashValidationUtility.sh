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
[[ -n "${ZBVU_INCLUDED:-}" ]] && return 0
ZBVU_INCLUDED=1

# Source the console utility library
ZBVU_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZBVU_SCRIPT_DIR}/bcu_BashCommandUtility.sh"

bvu_file_exists() {
  local filepath="$1"
  test -f "$filepath" || bcu_die "Required file not found: $filepath"
}

bvu_dir_exists() {
  local dirpath="$1"
  test -d "$dirpath" || bcu_die "Required directory not found: $dirpath"
}

bvu_dir_empty() {
  local dirpath="$1"
  test -d          "$dirpath"               || bcu_die "Required directory not found: $dirpath"
  test -z "$(ls -A "$dirpath" 2>/dev/null)" || bcu_die "Directory must be empty: $dirpath"
}

# Generic environment variable wrapper
bvu_env_wrapper() {
  local func_name=$1
  local varname=$2
  eval "local val=\${$varname:-}" || bcu_die "Variable '$varname' is not defined"
  shift 2

  ${func_name} "$varname" "$val" "$@"
}

# Generic optional wrapper - returns empty if value is empty
bvu_opt_wrapper() {
  local func_name=$1
  local varname=$2
  eval "local val=\${$varname:-}" || bcu_die "Variable '$varname' is not defined"

  # Empty is always valid for optional
  test -z "$val" && return 0

  shift 2
  ${func_name} "$varname" "$val" "$@"
}

# String validator with optional length constraints
bvu_val_string() {
  local varname=$1
  local val=$2
  local min=$3
  local max=$4
  local default=${5-}  # empty permitted

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"
  test -n "$min"     || bcu_die "min parameter is required for varname '$varname'"
  test -n "$max"     || bcu_die "max parameter is required for varname '$varname'"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  # Allow empty if min=0
  if [ "$min" = "0" -a -z "$val" ]; then
    echo "$val"
    return 0
  fi

  # Otherwise must not be empty
  test -n "$val" || bcu_die "$varname must not be empty"

  # Check length constraints if max provided
  if [ -n "$max" ]; then
    test ${#val} -ge $min || bcu_die "$varname must be at least $min chars, got '${val}' (${#val})"
    test ${#val} -le $max || bcu_die "$varname must be no more than $max chars, got '${val}' (${#val})"
  fi

  echo "$val"
}

# Cross-context name validator (system-safe identifier)
bvu_val_xname() {
  local varname=$1
  local val=$2
  local min=$3
  local max=$4
  local default=${5-}  # empty permitted

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"
  test -n "$min"     || bcu_die "min parameter is required for varname '$varname'"
  test -n "$max"     || bcu_die "max parameter is required for varname '$varname'"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  # Allow empty if min=0
  if [ "$min" = "0" -a -z "$val" ]; then
    echo "$val"
    return 0
  fi

  # Otherwise must not be empty
  test -n "$val" || bcu_die "$varname must not be empty"

  # Check length constraints
  test ${#val} -ge $min || bcu_die "$varname must be at least $min chars, got '${val}' (${#val})"
  test ${#val} -le $max || bcu_die "$varname must be no more than $max chars, got '${val}' (${#val})"

  # Must start with letter and contain only allowed chars
  test $(echo "$val" | grep -E '^[a-zA-Z][a-zA-Z0-9_-]*$') || \
    bcu_die "$varname must start with letter and contain only letters, numbers, underscore, hyphen, got '$val'"

  echo "$val"
}

# Google-style resource identifier (lowercase, digits, hyphens)
# Must start with a letter, end with letter/digit.
# Examples: GCP project IDs, GAR repo IDs.
bvu_val_gname() {
  local varname=$1
  local val=$2
  local min=$3
  local max=$4
  local default=${5-}  # empty permitted

  # Required params
  test -n "$varname" || bcu_die "varname parameter is required"
  test -n "$min"     || bcu_die "min parameter is required for varname '$varname'"
  test -n "$max"     || bcu_die "max parameter is required for varname '$varname'"

  # Defaulting
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  # Allow empty if min=0
  if [ "$min" = "0" -a -z "$val" ]; then
    echo "$val"
    return 0
  fi

  # Non-empty and length window
  test -n "$val" || bcu_die "$varname must not be empty"
  test ${#val} -ge $min || bcu_die "$varname must be at least $min chars, got '${val}' (${#val})"
  test ${#val} -le $max || bcu_die "$varname must be no more than $max chars, got '${val}' (${#val})"

  # Pattern: ^[a-z][a-z0-9-]*[a-z0-9]$
  test "$(echo "$val" | grep -E '^[a-z][a-z0-9-]*[a-z0-9]$')" || \
    bcu_die "$varname must match ^[a-z][a-z0-9-]*[a-z0-9]$ (lowercase letters, digits, hyphens; start with a letter; end with letter/digit), got '$val'"

  echo "$val"
}

# Fully Qualified Image Name component validator
bvu_val_fqin() {
  local varname=$1
  local val=$2
  local min=$3
  local max=$4
  local default=${5-}  # empty permitted

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"
  test -n "$min"     || bcu_die "min parameter is required for varname '$varname'"
  test -n "$max"     || bcu_die "max parameter is required for varname '$varname'"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  # Allow empty if min=0
  if [ "$min" = "0" -a -z "$val" ]; then
    echo "$val"
    return 0
  fi

  # Otherwise must not be empty
  test -n "$val" || bcu_die "$varname must not be empty"

  # Check length constraints
  test ${#val} -ge $min || bcu_die "$varname must be at least $min chars, got '${val}' (${#val})"
  test ${#val} -le $max || bcu_die "$varname must be no more than $max chars, got '${val}' (${#val})"

  # Allow letters, numbers, dots, hyphens, underscores, forward slashes, colons
  test $(echo "$val" | grep -E '^[a-zA-Z0-9][a-zA-Z0-9:._/-]*$') || \
    bcu_die "$varname must start with letter/number and contain only letters, numbers, colons, dots, underscores, hyphens, forward slashes, got '$val'"

  echo "$val"
}

# Boolean validator
bvu_val_bool() {
  local varname=$1
  local val=$2
  local default=$3

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test "$val" = "0" -o "$val" = "1" || bcu_die "$varname must be 0 or 1, got: '$val'"

  echo "$val"
}

# Decimal range validator
bvu_val_decimal() {
  local varname=$1
  local val=$2
  local min=$3
  local max=$4
  local default=$5

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"
  test -n "$min"     || bcu_die "min parameter is required for varname '$varname'"
  test -n "$max"     || bcu_die "max parameter is required for varname '$varname'"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test $val -ge $min -a $val -le $max || bcu_die "$varname value '$val' must be between $min and $max"

  echo "$val"
}

# IPv4 validator
bvu_val_ipv4() {
  local varname=$1
  local val=$2
  local default=${3-}  # empty permitted

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test $(echo $val | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || bcu_die "$varname has invalid IPv4 format: '$val'"

  echo "$val"
}

# CIDR validator
bvu_val_cidr() {
  local varname=$1
  local val=$2
  local default=$3

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test $(echo $val | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || bcu_die "$varname has invalid CIDR format: '$val'"

  echo "$val"
}

# Domain validator
bvu_val_domain() {
  local varname=$1
  local val=$2
  local default=$3

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test $(echo $val | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || bcu_die "$varname has invalid domain format: '$val'"

  echo "$val"
}

# Port validator
bvu_val_port() {
  local varname=$1
  local val=$2
  local default=$3

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  # Use default if value is empty and default provided
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  test -n "$val" || bcu_die "$varname must not be empty"
  test $val -ge 1 -a $val -le 65535 || bcu_die "$varname value '$val' must be between 1 and 65535"

  echo "$val"
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
bvu_val_odref() {
  local varname=$1
  local val=$2
  local default=${3-}  # empty permitted (only if caller wants to allow empty)

  test -n "$varname" || bcu_die "varname parameter is required"

  # Defaulting when allowed by caller
  if [ -z "$val" -a -n "$default" ]; then
    val="$default"
  fi

  # Must not be empty here (use bvu_opt_odref for optional)
  test -n "$val" || bcu_die "$varname must not be empty"

  # Enforce digest-pinned image ref:
  #   host[:port]/repo(/subrepo)@sha256:64hex
  #   - host: [a-z0-9.-]+ with optional :port
  #   - each repo segment: [a-z0-9._-]+ (lowercase)
  #   - digest algo fixed to sha256 with 64 lowercase hex chars
  local _re='^[a-z0-9.-]+(:[0-9]{2,5})?/([a-z0-9._-]+/)*[a-z0-9._-]+@sha256:[0-9a-f]{64}$'
  echo "$val" | grep -Eq "$_re" || bcu_die "$varname has invalid image reference format (require host[:port]/repo@sha256:<64hex>), got '$val'"

  echo "$val"
}

# List validators
bvu_val_list_ipv4() {
  local varname=$1
  local val=$2

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  test -z "$val" && return 0  # Empty lists allowed

  local item_num=0
  for item in $val; do
    item_num=$((item_num + 1))
    test $(echo $item | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || bcu_die "$varname item #$item_num has invalid IPv4 format: '$item'"
  done
}

bvu_val_list_cidr() {
  local varname=$1
  local val=$2

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  test -z "$val" && return 0  # Empty lists allowed

  local item_num=0
  for item in $val; do
    item_num=$((item_num + 1))
    test $(echo $item | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || bcu_die "$varname item #$item_num has invalid CIDR format: '$item'"
  done
}

bvu_val_list_domain() {
  local varname=$1
  local val=$2

  # Validate required parameters
  test -n "$varname" || bcu_die "varname parameter is required"

  test -z "$val" && return 0  # Empty lists allowed

  local item_num=0
  for item in $val; do
    item_num=$((item_num + 1))
    test $(echo $item | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || bcu_die "$varname item #$item_num has invalid domain format: '$item'"
  done
}

# Environment variable validators
bvu_env_string()             { bvu_env_wrapper "bvu_val_string"           "$@"; }
bvu_env_xname()              { bvu_env_wrapper "bvu_val_xname"            "$@"; }
bvu_env_gname()              { bvu_env_wrapper "bvu_val_gname"            "$@"; }
bvu_env_fqin()               { bvu_env_wrapper "bvu_val_fqin"             "$@"; }
bvu_env_bool()               { bvu_env_wrapper "bvu_val_bool"             "$@"; }
bvu_env_decimal()            { bvu_env_wrapper "bvu_val_decimal"          "$@"; }
bvu_env_ipv4()               { bvu_env_wrapper "bvu_val_ipv4"             "$@"; }
bvu_env_cidr()               { bvu_env_wrapper "bvu_val_cidr"             "$@"; }
bvu_env_domain()             { bvu_env_wrapper "bvu_val_domain"           "$@"; }
bvu_env_port()               { bvu_env_wrapper "bvu_val_port"             "$@"; }
bvu_env_odref()              { bvu_env_wrapper "bvu_val_odref"            "$@"; }

# Environment list validators
bvu_env_list_ipv4()          { bvu_env_wrapper "bvu_val_list_ipv4"        "$@"; }
bvu_env_list_cidr()          { bvu_env_wrapper "bvu_val_list_cidr"        "$@"; }
bvu_env_list_domain()        { bvu_env_wrapper "bvu_val_list_domain"      "$@"; }

# Optional validators
bvu_opt_bool()               { bvu_opt_wrapper "bvu_val_bool"             "$@"; }
bvu_opt_range()              { bvu_opt_wrapper "bvu_val_decimal"          "$@"; }
bvu_opt_ipv4()               { bvu_opt_wrapper "bvu_val_ipv4"             "$@"; }
bvu_opt_cidr()               { bvu_opt_wrapper "bvu_val_cidr"             "$@"; }
bvu_opt_domain()             { bvu_opt_wrapper "bvu_val_domain"           "$@"; }
bvu_opt_port()               { bvu_opt_wrapper "bvu_val_port"             "$@"; }

# eof

