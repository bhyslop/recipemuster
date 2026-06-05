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
# Recipe Bottle Lode - lifecycle REST (guard-free cluster, sourced by rbld0_Lode):
#   divine — enumerate Lodes / inspect one Lode's members (read-only)
#   banish — delete a whole Lode (Director credentials)

set -euo pipefail

######################################################################
# External Functions (rbld_*)

rbld_divine() {
  zrbld_sentinel

  local -r z_touchmark="${BUZ_FOLIO:-}"

  buc_doc_brief "Divine Lodes — enumerate all Lodes, or inspect one Lode's members (read-only)"
  buc_doc_oparm "touchmark" "Lode stamp to inspect (e.g., b260602120000); omit to enumerate all Lodes"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  if test -z "${z_touchmark}"; then
    buc_step "Enumerating Lodes under ${RBGL_LODES_ROOT}/"
    zrbfc_list_anchors_capture "${z_token}" "${RBGL_LODES_ROOT}"

    if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
      buc_info "No Lodes found under ${RBGL_LODES_ROOT}/"
      buc_success "Divine complete — 0 Lodes"
      return 0
    fi

    # Kind-letter legend, printed once so rows carry no repeated per-row column.
    # A touchmark's leading letter is its kind (b260602075327 -> bole); the
    # reader decodes the prefix from this key. One entry per implemented kind.
    local -r z_kind_fmt="    %-3s %-10s %s\n"
    echo ""
    printf "  Kinds (touchmark prefix):\n"
    printf "${z_kind_fmt}" "${RBGC_LODE_KIND_BOLE}" "bole" "upstream OCI image, consumed as a FROM line"

    # Load the touchmark list fully before iterating: the per-Lode tags fetch
    # spawns curl (via rbuh), and a child touching stdin would consume the
    # loop's remaining input. Load-then-iterate keeps that FD closed.
    local z_touchmarks=()
    local z_touch=""
    while IFS= read -r z_touch || test -n "${z_touch}"; do
      test -n "${z_touch}" || continue
      z_touchmarks+=("${z_touch}")
    done < "${ZRBFC_PACKAGE_LIST_FILE}"

    local -r z_row_fmt="  %-15s %s\n"
    echo ""
    printf "${z_row_fmt}" "TOUCHMARK" "IMAGE"
    printf "${z_row_fmt}" "---------------" "--------------------------------------"

    local z_idx=0
    local z_pkg=""
    local z_pkg_encoded=""
    local z_tags_url=""
    local z_enum_infix=""
    local z_resp_file=""
    local z_image_file=""
    local z_image=""
    for z_idx in "${!z_touchmarks[@]}"; do
      z_touch="${z_touchmarks[$z_idx]}"

      # One tags-list per Lode. IMAGE is the unsprued fingerprint tag
      # <sanitized-origin>-<sha10>; it is located via the sha10 taken from the
      # rbi_sha256-<hex> member tag, so Director semantic names (also unsprued)
      # cannot masquerade as the fingerprint. Per-Lode infix preserves each
      # response for forensics.
      z_pkg="${RBGL_LODES_ROOT}/${z_touch}"
      z_pkg_encoded="${z_pkg//\//%2F}"
      z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1000"
      z_enum_infix="rbld_divine_enum_${z_idx}"
      rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_enum_infix}"
      rbuh_require_ok "List tags for Lode ${z_touch}" "${z_enum_infix}"
      z_resp_file="${ZRBUH_PREFIX}${z_enum_infix}${ZRBUH_POSTFIX_JSON}"

      z_image_file="${ZRBLD_DIVINE_PREFIX}enum_${z_idx}_image.txt"
      jq -r --arg dp "${RBGC_LODE_TAG_DIGEST_PREFIX}" '
        [.tags[]?.name | sub(".*/tags/"; "")] as $names
        | ([$names[] | select(startswith($dp)) | ltrimstr($dp)[0:10]][0]) as $sha10
        | ([$names[] | select((startswith("rbi_") | not) and ($sha10 != null) and endswith("-" + $sha10))][0]) // "(no fingerprint)"
      ' "${z_resp_file}" > "${z_image_file}" \
        || buc_die "Failed to extract fingerprint for Lode ${z_touch}"
      z_image=$(<"${z_image_file}")
      test -n "${z_image}" || buc_die "Empty fingerprint extraction for Lode ${z_touch}"

      printf "${z_row_fmt}" "${z_touch}" "${z_image}"
    done
    echo ""
    buc_info "Total Lodes: ${#z_touchmarks[@]}"
    buc_success "Divine complete"
    return 0
  fi

  # Inspect depth: list the member tags on one Lode package.
  buc_step "Inspecting Lode ${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg="${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg_encoded="${z_pkg//\//%2F}"
  local -r z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1000"
  local -r z_tags_infix="rbld_divine_tags"

  rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_tags_infix}"
  rbuh_require_ok "List tags for Lode ${z_touchmark}" "${z_tags_infix}"

  local -r z_resp_file="${ZRBUH_PREFIX}${z_tags_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_tags_file="${ZRBLD_DIVINE_PREFIX}tags.txt"
  jq -r '.tags[]?.name | sub(".*/tags/"; "")' "${z_resp_file}" > "${z_tags_file}" \
    || buc_die "Failed to extract member tags for Lode ${z_touchmark}"

  if ! test -s "${z_tags_file}"; then
    buc_die "No member tags found under ${z_pkg} — Lode not present in registry"
  fi

  echo ""
  printf "  %s\n" "MEMBER-TAG"
  printf "  %s\n" "------------------------------"
  local z_count=0
  local z_tag=""
  while IFS= read -r z_tag || test -n "${z_tag}"; do
    test -n "${z_tag}" || continue
    printf "  %s\n" "${z_tag}"
    z_count=$((z_count + 1))
  done < "${z_tags_file}"
  echo ""
  buc_info "Total members: ${z_count}"
  buc_success "Divine complete — Lode ${z_touchmark}"
}

rbld_banish() {
  zrbld_sentinel

  local -r z_touchmark="${BUZ_FOLIO:-}"

  buc_doc_brief "Banish a Lode — delete the whole rbi_ld/<touchmark> GAR package"
  buc_doc_param "touchmark" "Lode stamp to delete (e.g., b260602120000)"
  buc_doc_shown || return 0

  test -n "${z_touchmark}" || buc_die "Touchmark parameter required"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  local -r z_pkg="${RBGL_LODES_ROOT}/${z_touchmark}"
  local -r z_pkg_encoded="${z_pkg//\//%2F}"

  # Verify presence before delete so banish reports a clean not-found.
  buc_step "Verifying Lode present: ${z_pkg}"
  local -r z_tags_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1"
  local -r z_probe_infix="rbld_banish_probe"
  rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_probe_infix}"
  rbuh_require_ok "Probe Lode ${z_touchmark}" "${z_probe_infix}"

  local z_tag_count=""
  z_tag_count=$(rbuh_json_field_capture "${z_probe_infix}" '(.tags // []) | length') \
    || buc_die "Failed to count tags for ${z_pkg}"
  test "${z_tag_count}" -gt 0 \
    || buc_die "No Lode found at ${z_pkg} — nothing to banish"

  buc_require "Will banish the whole Lode ${z_pkg} (single packages delete)" "yes"

  # Single packages delete removes the package and all its member versions/tags
  # atomically. DELETE returns a long-running operation; trust 200 as accepted.
  buc_step "Deleting Lode package: ${z_pkg}"
  local -r z_del_url="${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}"
  local -r z_del_infix="rbld_banish_del"
  rbuh_json "DELETE" "${z_del_url}" "${z_token}" "${z_del_infix}"
  rbuh_require_ok "Delete Lode package ${z_pkg}" "${z_del_infix}"

  echo ""
  buc_success "Lode banished: ${z_touchmark}"
}

# eof
