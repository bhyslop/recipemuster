#!/bin/bash
#
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
# Recipe Bottle Auth - RBRA/RBRO credential load and role token mint

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBA_SOURCED:-}" || buc_die "Module rba multiply sourced - check sourcing hierarchy"
ZRBA_SOURCED=1

######################################################################
# Internal Functions (zrba_*)

zrba_kindle() {
  test -z "${ZRBA_KINDLED:-}" || buc_die "Module rba already kindled"

  # Ensure dependency kindled first (rba mints tokens via rbgo)
  zrbgo_sentinel

  readonly ZRBA_KINDLED=1
}

zrba_sentinel() {
  test "${ZRBA_KINDLED:-}" = "1" || buc_die "Module rba not kindled - call zrba_kindle first"
}

######################################################################
# External / RBTOE Pattern Functions

# The credential accessor — the single place credential material is resolved.
# Keyed by identity (governor | director | retriever): maps the identity to its
# RBDC_<ROLE>_RBRA_FILE and mints through rbgo. No call site outside this
# function touches credential material (source-side grep-gated). The keyfile mint
# here is bridge scaffolding; the federated-token path will branch in this one
# function when it lands.
rba_token_capture() {
  zrba_sentinel

  local -r z_identity="${1:-}"

  local z_rbra_file
  case "${z_identity}" in
    governor)  z_rbra_file="${RBDC_GOVERNOR_RBRA_FILE:-}"  ;;
    director)  z_rbra_file="${RBDC_DIRECTOR_RBRA_FILE:-}"  ;;
    retriever) z_rbra_file="${RBDC_RETRIEVER_RBRA_FILE:-}" ;;
    *) buc_die "rba_token_capture: unknown identity '${z_identity}' (expected governor | director | retriever)" ;;
  esac

  test -n "${z_rbra_file}" || return 1

  # return $? not 1: an in-band rejection from the mint (credless guard) must
  # survive this accessor so the buc_die membrane upstream re-exits it precisely.
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || return $?

  test -n "${z_token}" || return 1
  echo    "${z_token}"
}

rba_extract_json_to_rbra() {
  zrba_sentinel

  local -r z_json_path="$1"
  local -r z_rbra_path="$2"
  local -r z_lifetime_sec="$3"
  local -r z_expected_project_id="${4:-}"

  test -f "${z_json_path}" || buc_die "Service account JSON not found: ${z_json_path}"

  buc_info "Extracting service account credentials from JSON"

  buc_log_args 'Extract fields'
  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_json_path}") \
                                        || buc_die "Failed to extract client_email"
  test -n "${z_client_email}"           || buc_die "Empty client_email in JSON"
  test    "${z_client_email}" != "null" || buc_die "Null client_email in JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_json_path}") \
                                       || buc_die "Failed to extract private_key"
  test -n "${z_private_key}"           || buc_die "Empty private_key in JSON"
  test    "${z_private_key}" != "null" || buc_die "Null private_key in JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_json_path}") \
                                      || buc_die "Failed to extract project_id"
  test -n "${z_project_id}"           || buc_die "Empty project_id in JSON"
  test    "${z_project_id}" != "null" || buc_die "Null project_id in JSON"

  if test -n "${z_expected_project_id}"; then
    buc_log_args "Verify project matches expected: ${z_expected_project_id}"
    test "${z_project_id}" = "${z_expected_project_id}" \
      || buc_die "Project mismatch: JSON has '${z_project_id}', expected '${z_expected_project_id}'"
  else
    buc_log_args "No project validation - accepting JSON project_id: ${z_project_id}"
  fi

  buc_log_args 'Write RBRA file'
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_oauth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "${z_lifetime_sec}"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"

  test -f "${z_rbra_path}" || buc_die "Failed to write RBRA file: ${z_rbra_path}"

  buc_warn "Consider deleting source JSON after verification: ${z_json_path}"
}

# RBTOE: RBRA Load Pattern
# Sources an RBRA file and validates required fields
rba_rbra_load() {
  zrba_sentinel

  local -r z_rbra_file="${1}"

  test -n "${z_rbra_file}" || buc_die "rba_rbra_load: RBRA file path required"
  test -f "${z_rbra_file}" || buc_die "rba_rbra_load: RBRA file not found: ${z_rbra_file}"

  buc_log_args "Loading and validating RBRA credentials from ${z_rbra_file}"

  # Source the RBRA file
  source "${z_rbra_file}" || buc_die "rba_rbra_load: failed to source RBRA file"

  # Validate required fields
  test -n "${RBRA_CLIENT_EMAIL:-}" || buc_die "rba_rbra_load: RBRA_CLIENT_EMAIL missing from ${z_rbra_file}"
  test -n "${RBRA_PRIVATE_KEY:-}" || buc_die "rba_rbra_load: RBRA_PRIVATE_KEY missing from ${z_rbra_file}"
  test -n "${RBRA_PROJECT_ID:-}" || buc_die "rba_rbra_load: RBRA_PROJECT_ID missing from ${z_rbra_file}"

  # Check for null values
  test "${RBRA_CLIENT_EMAIL}" != "null" || buc_die "rba_rbra_load: RBRA_CLIENT_EMAIL is null in ${z_rbra_file}"
  test "${RBRA_PRIVATE_KEY}" != "null" || buc_die "rba_rbra_load: RBRA_PRIVATE_KEY is null in ${z_rbra_file}"
  test "${RBRA_PROJECT_ID}" != "null" || buc_die "rba_rbra_load: RBRA_PROJECT_ID is null in ${z_rbra_file}"

  buc_log_args "RBRA validation successful: ${RBRA_CLIENT_EMAIL} in project ${RBRA_PROJECT_ID}"
}

# RBTOE: RBRO Load Pattern
# Thin wrapper: defensively sources rbro_regime.sh (callers don't need to know
# its path, which moved under AAD's payor/ subdirectory migration), then
# delegates to rbro_load. Parallels rba_rbra_load at the call-signature level
# even though that function carries its own validation; the uniform rba_*
# load-through-utility convention is the load-bearing reason this wrapper exists.
rba_rbro_load() {
  zrba_sentinel

  buc_log_args "Loading RBRO OAuth credentials"

  # Source regime module if not already loaded
  if test -z "${ZRBRO_SOURCED:-}"; then
    source "${BASH_SOURCE[0]%/*}/rbro_regime.sh"
  fi

  # Delegate to regime's canonical load
  rbro_load

  buc_log_args "RBRO validation successful"
}

# eof
