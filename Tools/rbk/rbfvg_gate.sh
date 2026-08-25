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
# Recipe Bottle Foundry Verify - vouch gate body (guard-free cluster, sourced
# by rbfv0_verify): rbfv_vouch_gate — the read-only admission check that HEADs
# a hallmark's vouch package manifest in GAR and refuses an image that carries
# none. Submits no Cloud Build and owns no support of its own; it reads no
# ZRBFV_ kindle constant, taking its probe scratch file from Foundry Core.

set -euo pipefail

######################################################################
# External Functions (rbfv_*)

rbfv_vouch_gate() {
  zrbfv_sentinel

  local -r z_vessel="${1:-}"
  local -r z_hallmark="${2:-}"

  test -n "${z_vessel}"       || buc_die_now "rbfv_vouch_gate: vessel required"
  test -n "${z_hallmark}" || buc_die_now "rbfv_vouch_gate: hallmark required"

  # Vouch package = rbi_hm/<H>/vouch, tag = <H> (hallmark-as-tag).
  local -r z_vouch_tag="${z_hallmark}"
  buc_step "Vouch gate: checking ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}:${z_vouch_tag}"

  local z_token
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "rbfv_vouch_gate: failed to get Director OAuth token"

  local z_vouch_http_code
  local z_curl_status=0
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -o /dev/null \
    -w "%{http_code}" \
    "${ZRBFC_REGISTRY_API_BASE}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}/manifests/${z_vouch_tag}" \
    > "${ZRBFC_SCRATCH_FILE}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "rbfv_vouch_gate: HEAD request failed for ${z_vessel}:${z_vouch_tag} (curl exit ${z_curl_status})"
  z_vouch_http_code=$(<"${ZRBFC_SCRATCH_FILE}")

  if test "${z_vouch_http_code}" != "200"; then
    buc_die_now "Hallmark not vouched: ${z_hallmark} (HTTP ${z_vouch_http_code} — refusing to use unvouched image)"
  fi

  buc_info "Vouch verified: ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}:${z_vouch_tag}"
}

# eof
