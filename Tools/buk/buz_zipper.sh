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
# BUK Zipper - Colophon registry via parallel arrays with regime-selection channels

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBUZ_SOURCED:-}" || buc_die "Module buz multiply sourced - check sourcing hierarchy"
ZBUZ_SOURCED=1

######################################################################
# Internal kindle boilerplate

zbuz_kindle() {
  test -z "${ZBUZ_KINDLED:-}" || buc_die "buz already kindled"

  # Registry rolls (populated by buz_enroll in consumer kindle, same-process only)
  z_buz_colophon_roll=()
  z_buz_module_roll=()
  z_buz_command_roll=()
  z_buz_channel_roll=()

  readonly ZBUZ_KINDLED=1
}

######################################################################
# Internal sentinel

zbuz_sentinel() {
  test "${ZBUZ_KINDLED:-}" = "1" || buc_die "Module buz not kindled - call zbuz_kindle first"
}

######################################################################
# Internal helpers

# zbuz_resolve_tabtarget_capture() - Resolve colophon to tabtarget path
# Args: colophon
# Returns: tabtarget path or exit 1
zbuz_resolve_tabtarget_capture() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || return 1

  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || return 1

  # Allow multiple matches (imprinted colophons share a colophon prefix)
  # Return first match as representative
  echo "${z_matches[0]}"
}

######################################################################
# Group declaration (no-op category annotation for colophon taxonomy)

# buz_group() - Declare a colophon group category
# Args: constant_name, prefix, description
# Currently a no-op — documents the group taxonomy without affecting dispatch.
buz_group() {
  zbuz_sentinel
  :
}

######################################################################
# Public enroll (kindle-only registry population)

# buz_enroll() - Register colophon tuple in parallel rolls
# Args: varname, colophon, module, command [, channel]
# Optional channel: "" (default), "imprint", or "param1"
# Assigns colophon string to caller's variable via printf -v
# Side effects: populates registry rolls (must be called in same process, NOT inside $())
buz_enroll() {
  zbuz_sentinel

  local z_varname="${1:-}"
  local z_colophon="${2:-}"
  local z_module="${3:-}"
  local z_command="${4:-}"
  local z_channel="${5:-}"
  test -n "${z_varname}"  || buc_die "buz_enroll: varname required"
  test -n "${z_colophon}" || buc_die "buz_enroll: colophon required"
  test -n "${z_module}"   || buc_die "buz_enroll: module required"
  test -n "${z_command}"  || buc_die "buz_enroll: command required"

  # Validate channel value
  case "${z_channel}" in
    ""|"imprint"|"param1") ;;
    *) buc_die "buz_enroll: invalid channel: ${z_channel}" ;;
  esac

  # Validate variable name
  [[ "${z_varname}" =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]] \
    || buc_die "buz_enroll: invalid variable name: ${z_varname}"

  # Roll population (only persists in same-process context, lost in $() subshell)
  z_buz_colophon_roll+=("${z_colophon}")
  z_buz_module_roll+=("${z_module}")
  z_buz_command_roll+=("${z_command}")
  z_buz_channel_roll+=("${z_channel}")

  # Assign colophon to caller's variable
  printf -v "${z_varname}" '%s' "${z_colophon}" || buc_die "buz_enroll: printf -v failed for ${z_varname}"
}

######################################################################
# Healthcheck (opt-in tabtarget validation — called by consumer, not by BUK)

# buz_healthcheck() - Validate that all enrolled colophons have tabtargets on disk
# Dies on first missing tabtarget. Call after enrollment is complete.
buz_healthcheck() {
  zbuz_sentinel

  local z_i=""
  for z_i in "${!z_buz_colophon_roll[@]}"; do
    zbuz_resolve_tabtarget_capture "${z_buz_colophon_roll[z_i]}" >/dev/null \
      || buc_die "buz_healthcheck: no tabtarget for colophon '${z_buz_colophon_roll[z_i]}' in ${BURC_TABTARGET_DIR}/"
  done
}

######################################################################
# Lookup dispatch

# buz_exec_lookup() - Resolve colophon via registry and exec
# Args: colophon, base_dir [, extra args passed through to exec]
# Execs: BUZ_FOLIO=<folio> ${base_dir}/${module} ${command} [extra args]
# Dies if colophon not found
buz_exec_lookup() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  local z_base_dir="${2:-}"
  test -n "${z_colophon}" || buc_die "buz_exec_lookup: colophon required"
  test -n "${z_base_dir}" || buc_die "buz_exec_lookup: base_dir required"
  shift 2

  # Find colophon in registry
  local z_found=""
  local z_i=""
  for z_i in "${!z_buz_colophon_roll[@]}"; do
    test "${z_buz_colophon_roll[z_i]}" = "${z_colophon}" || continue
    z_found="${z_i}"
    break
  done
  test -n "${z_found}" || buc_die "buz_exec_lookup: colophon not found: ${z_colophon}"

  # Decode folio from channel
  local z_folio=""
  local z_args=("$@")
  case "${z_buz_channel_roll[z_found]}" in
    "") ;;
    "imprint")
      z_folio="${BURD_TOKEN_3}"
      ;;
    "param1")
      if (( ${#z_args[@]} )); then
        z_folio="${z_args[0]}"
        z_args=("${z_args[@]:1}")
      fi
      ;;
    *)
      buc_die "buz_exec_lookup: unknown channel: ${z_buz_channel_roll[z_found]}"
      ;;
  esac

  # Dispatch with folio in exec environment
  BUZ_FOLIO="${z_folio}" exec "${z_base_dir}/${z_buz_module_roll[z_found]}" "${z_buz_command_roll[z_found]}" ${z_args[@]+"${z_args[@]}"}
}

# eof
