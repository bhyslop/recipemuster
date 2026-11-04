#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# Recipe Bottle Google OAuth - base64url primitives, curl-transient classifier, docker login

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGO_SOURCED:-}" || buc_die_now "Module rbgo multiply sourced - check sourcing hierarchy"
ZRBGO_SOURCED=1

######################################################################
# Internal Functions (zrbgo_*)

zrbgo_kindle() {
  test -z "${ZRBGO_KINDLED:-}" || buc_die_now "Module rbgo already kindled"

  # Validate required tools (rehomed from rbl_Locator.sh)
  command -v openssl >/dev/null || buc_die_now "openssl not found - required for JWT signing and encoding"
  command -v curl    >/dev/null || buc_die_now "curl not found - required for OAuth exchange"
  command -v jq      >/dev/null || buc_die_now "jq not found - required for JSON parsing"

  buc_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  buc_log_args "Check environment"
  zburd_sentinel

  # Module prefix for temp files
  readonly ZRBGO_PREFIX="${BURD_TEMP_DIR}/rbgo_"

  readonly ZRBGO_KINDLED=1
}

zrbgo_sentinel() {
  test "${ZRBGO_KINDLED:-}" = "1" || buc_die_now "Module rbgo not kindled - call zrbgo_kindle first"
}

######################################################################
# Base64 primitives — generic openssl wrappers.
#
# rbfd and rblds decode Cloud Build step outputs. The -A flag (suppress line
# wrapping) is load-bearing on every site and lives only here, so it cannot be
# silently dropped on one path.

rbgo_base64_decode_file_to_file() {
  zrbgo_sentinel

  local -r z_input="${1:-}"
  local -r z_output="${2:-}"

  buc_doc_brief "Decode a base64 file into another file"
  buc_doc_param "input"  "File holding the base64 text"
  buc_doc_param "output" "File the decoded bytes are written to"
  buc_doc_shown || return 0

  test -n "${z_input}"  || buc_die_now "rbgo_base64_decode_file_to_file: input required"
  test -n "${z_output}" || buc_die_now "rbgo_base64_decode_file_to_file: output required"
  test -f "${z_input}"  || buc_die_now "rbgo_base64_decode_file_to_file: input not found: ${z_input}"

  local -r z_stderr_file="${ZRBGO_PREFIX}decode_${z_output##*/}_stderr.txt"
  openssl enc -base64 -d -A < "${z_input}" > "${z_output}" 2>"${z_stderr_file}" \
    || buc_die_now "Failed to base64-decode ${z_input} into ${z_output} — see ${z_stderr_file}"
}

rbgo_base64_encode_file_capture() {
  zrbgo_sentinel

  local -r z_file="${1:-}"
  test -f "${z_file}" || return 1
  openssl enc -base64 -A < "${z_file}"
}

rbgo_curl_status_is_transient_predicate() {
  zrbgo_sentinel

  case "${1:-}" in
    7|28|35|56) return 0 ;;
    *)          return 1 ;;
  esac
}

######################################################################
# External Functions (rbgo_*)

# Authenticate the host docker client to a GAR registry with bounded retry on
# the moby/moby#44350 premature-timeout transient — RBr_5b8 (signature) /
# RBr_2f8 (membrane). docker login is the lone login/pull/push
# verb with no internal retry and a hardcoded 15s daemon->registry auth timeout;
# against a healthy-but-slow backend it fails where the endpoint is in fact fine.
# This mirrors the curl-transient tolerance in rbuh_json: retry only the
# surveyed signature, fail fast on everything else (real auth failures emit
# "unauthorized" and do not match), so clean-failure semantics are preserved.
# Args: token registry_host
rbgo_docker_login() {
  zrbgo_sentinel

  local -r zSECRET_token="${1:-}"
  local -r z_host="${2:-}"

  buc_doc_brief "Log the host docker client in to a GAR registry, retrying the surveyed daemon timeout"
  buc_doc_param "token"         "OAuth access token docker login reads on stdin"
  buc_doc_param "registry_host" "Registry host to authenticate against"
  buc_doc_shown || return 0

  test -n "${zSECRET_token}" || buc_die_now "rbgo_docker_login: token required"
  test -n "${z_host}"  || buc_die_now "rbgo_docker_login: registry host required"

  local z_attempt=0
  local z_stderr_file=""
  local z_login_out=""
  local z_rc=0
  local z_stderr_text=""

  while :; do
    z_attempt=$((z_attempt + 1))
    z_stderr_file="${ZRBGO_PREFIX}docker_login_${z_attempt}_stderr.txt"

    z_rc=0
    z_login_out=$(printf '%s' "${zSECRET_token}" | docker login -u oauth2accesstoken --password-stdin "https://${z_host}" 2>"${z_stderr_file}") || z_rc=$?
    buc_log_args "docker login attempt ${z_attempt}: login status ${z_rc}${z_login_out:+ (${z_login_out})}"

    test "${z_rc}" -ne 0 || break

    buc_log_pipe < "${z_stderr_file}"

    z_stderr_text=$(<"${z_stderr_file}") || buc_die_now "Failed to read: ${z_stderr_file}"

    [[ "${z_stderr_text}" == *"${RBGC_DOCKER_LOGIN_TRANSIENT_SIGNATURE}"* ]] \
      || buc_die_now "Docker login to ${z_host} failed — see ${z_stderr_file}"

    test "${z_attempt}" -lt "${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}" \
      || buc_die_now "Docker login to ${z_host} failed after ${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS} attempts (transient daemon->registry timeout, moby#44350) — see ${z_stderr_file}"

    buc_warn "Docker login transient (attempt ${z_attempt}/${RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS}, moby#44350 timeout) — retrying in ${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}s"
    sleep "${RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC}" || buc_die_now "Failed to pause before docker login retry ${z_attempt}"
  done
}

# Remove the docker client's stored credential for a registry once the push or
# pull that needed it has finished: the token lives an hour, the operation
# minutes, and docker's store is shared by every process on the station. Output
# goes to files under the module prefix. A failure is logged here naming the
# registry and returns 1: a caller on the success path dies on it, a caller on a
# path already dying warns and lets its original die stand.
# Args: registry_host
rbgo_docker_logout_predicate() {
  zrbgo_sentinel

  local -r z_host="${1:-}"

  buc_doc_brief "Log the host docker client out of a GAR registry"
  buc_doc_param "registry_host" "Registry host to remove the stored credential for"
  buc_doc_shown || return 0

  test -n "${z_host}" || return 1

  local -r z_stdout_file="${ZRBGO_PREFIX}docker_logout_stdout.txt"
  local -r z_stderr_file="${ZRBGO_PREFIX}docker_logout_stderr.txt"

  docker logout "${z_host}" > "${z_stdout_file}" 2>"${z_stderr_file}" || {
    buc_log_args "Docker logout from ${z_host} failed — see ${z_stderr_file}"
    return 1
  }
}

# eof
