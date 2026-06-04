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
# Recipe Bottle Foundry Ledger - delete cluster (guard-free, sourced by rbflk_):
# jettison a single image tag by locator, or abjure a whole hallmark subtree
# (Director credentials).

set -euo pipefail

######################################################################
# Delete (rbfl_*)

rbfl_jettison() {
  zrbfl_sentinel

  local z_locator="${BUZ_FOLIO:-}"

  # Documentation block
  buc_doc_brief "Jettison an image tag from the registry by locator"
  buc_doc_param "locator" "Image locator in package-path:tag format (e.g. rbi_hm/H/image:H)"
  buc_doc_shown || return 0

  # Validate locator parameter
  test -n "${z_locator}" || buc_die "Locator parameter required (package-path:tag)"

  # Parse locator into package path and tag
  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected package-path:tag" ;;
  esac
  local z_pkg_path="${z_locator%:*}"
  local z_tag="${z_locator##*:}"
  test -n "${z_pkg_path}" || buc_die "Package path is empty in locator"
  test -n "${z_tag}" || buc_die "Tag is empty in locator"

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_require "Will jettison: ${z_locator}" "yes"

  buc_step "Jettisoning: ${z_locator}"

  # Jettison by tag reference
  local z_status_file="${ZRBFL_DELETE_PREFIX}status.txt"
  local z_response_file="${ZRBFL_DELETE_PREFIX}response.json"
  local z_stderr_file="${ZRBFL_DELETE_PREFIX}stderr.txt"

  rbuh_request "DELETE"                                                  \
                    "${ZRBFC_REGISTRY_API_BASE}/${z_pkg_path}/manifests/${z_tag}" \
                    "${z_token}"                                              \
                    "${z_response_file}" "${z_status_file}" "${z_stderr_file}" \
    || buc_die "DELETE request failed — see ${z_stderr_file}"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || buc_die "HTTP status code is empty"

  if test "${z_http_code}" != "202" && test "${z_http_code}" != "204"; then
    local z_body="empty"
    if test -f "${z_response_file}"; then z_body=$(<"${z_response_file}"); fi
    buc_warn "Response body: ${z_body}"
    buc_die "Jettison failed with HTTP ${z_http_code}"
  fi

  buc_success "Jettisoned or nonexistent: ${z_locator}"
}

rbfl_abjure() {
  zrbfl_sentinel

  local z_hallmark="${BUZ_FOLIO:-}"

  # Documentation block
  buc_doc_brief "Abjure a hallmark — delete all GAR packages under rbi_hm/<hallmark>/"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  test -n "${z_hallmark}" || buc_die "Hallmark parameter required"

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Enumerate packages under rbi_hm/<hallmark>/ via GAR REST API.
  # Each immediate child of the subtree is one ark (image, vouch, pouch,
  # about, attest, diags). Iterating discovered children rather than a
  # hardcoded suffix list naturally tolerates graft's missing pouch.
  local -r z_subtree="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/"
  buc_step "Enumerating packages under ${z_subtree}"

  local -r z_list_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000"
  local -r z_list_infix="rbfl_abjure_list"

  rbuh_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbuh_require_ok "List packages for abjure" "${z_list_infix}"

  # GAR returns package names URL-encoded in the resource name (slashes as
  # %2F); decode and prefix-match to the hallmark subtree.
  local -r z_resp_file="${ZRBUH_PREFIX}${z_list_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_pkg_file="${ZRBFL_DELETE_PREFIX}packages.txt"

  jq -r --arg subtree "${z_subtree}" '
    .packages[]?.name
    | sub("^.*/packages/"; "")
    | gsub("%2F"; "/")
    | select(startswith($subtree))
  ' "${z_resp_file}" > "${z_pkg_file}" \
    || buc_die "Failed to extract package names for hallmark subtree"

  if ! test -s "${z_pkg_file}"; then
    buc_die "No packages found under ${z_subtree} — hallmark not present in registry"
  fi

  local z_count=0
  local z_count_line=""
  while IFS= read -r z_count_line || test -n "${z_count_line}"; do
    z_count=$((z_count + 1))
  done < "${z_pkg_file}"

  local z_confirm_msg="Will abjure ${z_count} packages under ${z_subtree}:"
  local z_pkg_path=""
  while IFS= read -r z_pkg_path || test -n "${z_pkg_path}"; do
    z_confirm_msg="${z_confirm_msg}\n  - ${z_pkg_path}"
  done < "${z_pkg_file}"
  buc_require "${z_confirm_msg}" "yes"

  # Delete each package via GAR REST API.
  # DELETE returns a long-running operation; trust 200 as accepted (matches
  # the prior fire-and-forget semantics with 202/204 on the v2 manifest API).
  local z_pkg_path=""
  local z_del_idx=0
  while IFS= read -r z_pkg_path || test -n "${z_pkg_path}"; do
    buc_step "Deleting package: ${z_pkg_path}"

    local z_pkg_encoded="${z_pkg_path//\//%2F}"
    local z_del_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}"
    local z_del_infix="rbfl_abjure_del_${z_del_idx}"

    rbuh_json "DELETE" "${z_del_url}" "${z_token}" "${z_del_infix}"
    rbuh_require_ok "Delete package ${z_pkg_path}" "${z_del_infix}"

    buc_info "Deleted: ${z_pkg_path}"
    z_del_idx=$((z_del_idx + 1))
  done < "${z_pkg_file}"

  echo ""
  buc_success "Hallmark abjured: ${z_hallmark} (${z_count} packages)"
}

# eof
