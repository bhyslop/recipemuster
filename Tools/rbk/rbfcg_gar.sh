#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Foundry Core - GAR-REST cluster (guard-free, sourced by rbfck_):
# enumerate GAR packages and anchors, extract FROM-scratch artifacts, and resolve
# a hallmark's vessel from its vouch ark - all via the Registry API, no docker.

set -euo pipefail

######################################################################
# GAR REST (zrbfcg_* / rbfcg_*)

# Internal: enumerate every <element>/<basename> pair under <subtree-root>/ via GAR REST.
# Writes "<element> <basename>" pairs (one per line, sorted) to ZRBFC0_PACKAGE_LIST_FILE.
# An empty file is a valid result (no elements).
# Args: token, subtree_root (e.g., RBGL_HALLMARKS_ROOT)
# Pagination: pageSize=1000 — deferred until evidence of >1000 packages.
zrbfcg_list_packages_capture() {
  zrbfc0_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_subtree_root="${2:?Subtree root required}"

  local -r z_list_url="${ZRBFC0_GAR_API_BASE}/${ZRBFC0_GAR_PACKAGE_BASE}/packages?pageSize=1000"
  local -r z_list_infix="rbfc_list_packages"
  local -r z_subtree="${z_subtree_root}/"

  rbuh_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbuh_require_ok "List GAR packages" "${z_list_infix}"

  local -r z_resp_file="${ZRBUH_PREFIX}${z_list_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_raw_file="${BURD_TEMP_DIR}/rbfc_package_list_raw.txt"

  # GAR returns package names URL-encoded (slashes as %2F). Decode, strip the
  # API path prefix, filter to <subtree>/<element>/<basename> shape
  # (length == 2 after split excludes anything deeper or shallower), emit as
  # space-separated pair lines.
  jq -r --arg subtree "${z_subtree}" '
    .packages[]?.name
    | sub("^.*/packages/"; "")
    | gsub("%2F"; "/")
    | select(startswith($subtree))
    | ltrimstr($subtree)
    | split("/")
    | select(length == 2)
    | "\(.[0]) \(.[1])"
  ' "${z_resp_file}" > "${z_raw_file}" \
    || return 1

  local -r z_sorted_file="${BURD_TEMP_DIR}/rbfc_package_list_sorted.txt"
  sort "${z_raw_file}" > "${z_sorted_file}" \
    || return 1

  # Per-package tags.list filter: skip packages with zero live tags. RBr_w9k
  : > "${ZRBFC0_PACKAGE_LIST_FILE}" || return 1
  local z_pair_lines=()
  local z_pair_line=""
  while IFS= read -r z_pair_line || test -n "${z_pair_line}"; do
    z_pair_lines+=("${z_pair_line}")
  done < "${z_sorted_file}"

  local z_element=""
  local z_basename=""
  local z_pkg_name=""
  local z_pkg_encoded=""
  local z_tag_infix=""
  local z_tag_count=""
  local z_tags_url=""
  local z_tag_idx=0
  local z_i=0
  for z_i in "${!z_pair_lines[@]}"; do
    IFS=' ' read -r z_element z_basename <<< "${z_pair_lines[${z_i}]}" || return 1
    test -n "${z_element}" || continue
    z_pkg_name="${z_subtree}${z_element}/${z_basename}"
    z_pkg_encoded="${z_pkg_name//\//%2F}"
    z_tag_infix=$(printf 'rbfc_tags_%04d' "${z_tag_idx}") || return 1
    z_tag_idx=$((z_tag_idx + 1))
    z_tags_url="${ZRBFC0_GAR_API_BASE}/${ZRBFC0_GAR_PACKAGE_BASE}/packages/${z_pkg_encoded}/tags?pageSize=1"
    rbuh_json "GET" "${z_tags_url}" "${z_token}" "${z_tag_infix}"
    rbuh_require_ok "List tags for ${z_pkg_name}" "${z_tag_infix}"
    z_tag_count=$(rbuh_json_field_capture "${z_tag_infix}" '(.tags // []) | length') \
      || return 1
    test "${z_tag_count}" -gt 0 || continue
    echo "${z_element} ${z_basename}" >> "${ZRBFC0_PACKAGE_LIST_FILE}" || return 1
  done
}

# Internal: enumerate every <anchor> directly under <subtree-root>/ via GAR REST.
# For 1-deep categories (e.g., Lodes) where the package itself is
# named <anchor> with no further basename. Writes "<anchor>" lines (one per
# line, sorted) to ZRBFC0_PACKAGE_LIST_FILE. An empty file is a valid result.
# Args: token, subtree_root (e.g., RBGL_LODES_ROOT)
# Pagination: pageSize=1000 — deferred until evidence of >1000 packages.
zrbfcg_list_anchors_capture() {
  zrbfc0_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_subtree_root="${2:?Subtree root required}"

  local -r z_list_url="${ZRBFC0_GAR_API_BASE}/${ZRBFC0_GAR_PACKAGE_BASE}/packages?pageSize=1000"
  local -r z_list_infix="rbfc_list_anchors"
  local -r z_subtree="${z_subtree_root}/"

  rbuh_json "GET" "${z_list_url}" "${z_token}" "${z_list_infix}"
  rbuh_require_ok "List GAR anchor packages" "${z_list_infix}"

  local -r z_resp_file="${ZRBUH_PREFIX}${z_list_infix}${ZRBUH_POSTFIX_JSON}"
  local -r z_raw_file="${BURD_TEMP_DIR}/rbfc_anchor_list_raw.txt"

  # Filter to <subtree>/<anchor> shape (length == 1 after split excludes
  # anything deeper) and emit just the anchor name.
  jq -r --arg subtree "${z_subtree}" '
    .packages[]?.name
    | sub("^.*/packages/"; "")
    | gsub("%2F"; "/")
    | select(startswith($subtree))
    | ltrimstr($subtree)
    | split("/")
    | select(length == 1)
    | .[0]
  ' "${z_resp_file}" > "${z_raw_file}" \
    || return 1

  sort "${z_raw_file}" > "${ZRBFC0_PACKAGE_LIST_FILE}" \
    || return 1
}

# Internal: write the HTTP code a manifest HEAD answers for package:tag in GAR to
# out_file — 200 when the artifact stands. Callers that branch on presence read
# the file and test it; a transport failure dies here rather than reading as
# absence.
# Args: token package tag out_file
zrbfcg_manifest_code_fetch() {
  zrbfc0_sentinel

  local -r z_token="${1:-}"
  local -r z_package="${2:-}"
  local -r z_tag="${3:-}"
  local -r z_out="${4:-}"
  test -n "${z_token}"   || buc_die_now "zrbfcg_manifest_code_fetch: token required"
  test -n "${z_package}" || buc_die_now "zrbfcg_manifest_code_fetch: package required"
  test -n "${z_tag}"     || buc_die_now "zrbfcg_manifest_code_fetch: tag required"
  test -n "${z_out}"     || buc_die_now "zrbfcg_manifest_code_fetch: out required"

  local -r z_response="${z_out}_response.txt"
  local -r z_stderr="${z_out}_stderr.txt"
  local z_curl_status=0
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC0_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_response}" \
    "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/manifests/${z_tag}" \
    > "${z_out}" 2>"${z_stderr}" || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "HEAD request failed for ${z_package}:${z_tag} (curl exit ${z_curl_status}) — see ${z_stderr}"
}

# Internal: extract files from a FROM-scratch artifact in GAR to a local directory.
# Handles multi-platform manifest lists by picking the first platform (content is
# identical for architecture-independent artifacts like -about and -vouch).
# Args: token package tag extract_dir [absent_message]
# Dies when the artifact is absent (with absent_message where the caller gives one) or on any infrastructure failure (curl, jq,
# tar); a caller that branches on presence asks zrbfcg_manifest_code_fetch first.
zrbfcg_gar_extract_artifact() {
  zrbfc0_sentinel

  local -r z_token="${1:-}"
  local -r z_package="${2:-}"
  local -r z_tag="${3:-}"
  local -r z_extract_dir="${4:-}"
  local -r z_absent_message="${5:-}"
  test -n "${z_token}" || buc_die_now "zrbfcg_gar_extract_artifact: token required"
  test -n "${z_package}" || buc_die_now "zrbfcg_gar_extract_artifact: package required"
  test -n "${z_tag}" || buc_die_now "zrbfcg_gar_extract_artifact: tag required"
  test -n "${z_extract_dir}" || buc_die_now "zrbfcg_gar_extract_artifact: extract_dir required"

  local -r z_safe_pkg="${z_package//\//_}"
  local -r z_prefix="${BURD_TEMP_DIR}/gar_${z_safe_pkg}_${z_tag}_"

  # HEAD check — does the artifact exist?
  zrbfcg_manifest_code_fetch "${z_token}" "${z_package}" "${z_tag}" "${z_prefix}head_code.txt"
  local z_http_code
  z_http_code=$(<"${z_prefix}head_code.txt") || buc_die_now "Failed to read: ${z_prefix}head_code.txt"
  if test "${z_http_code}" != "200"; then
    test -z "${z_absent_message}" || buc_die_now "${z_absent_message}"
    buc_die_now "Not found in GAR: ${z_package}:${z_tag} (HTTP ${z_http_code})"
  fi
  local z_curl_status=0

  # GET manifest (may be manifest list/index or single-platform manifest)
  local -r z_manifest="${z_prefix}manifest.json"
  local -r z_manifest_stderr="${z_prefix}manifest_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC0_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/manifests/${z_tag}" \
    > "${z_manifest}" 2>"${z_manifest_stderr}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "GET manifest failed for ${z_package}:${z_tag} (curl exit ${z_curl_status}) — see ${z_manifest_stderr}"

  # Resolve to a single-platform manifest
  local z_single_manifest="${z_manifest}"
  local -r z_media_type_file="${z_prefix}media_type.txt"
  local -r z_media_type_stderr="${z_prefix}media_type_stderr.txt"
  jq -r '.mediaType // empty' "${z_manifest}" > "${z_media_type_file}" 2>"${z_media_type_stderr}" \
    || buc_die_now "Failed to read manifest mediaType for ${z_package}:${z_tag} — see ${z_media_type_stderr}"
  local z_media_type
  z_media_type=$(<"${z_media_type_file}") || buc_die_now "Failed to read: ${z_media_type_file}"

  case "${z_media_type}" in
    *manifest.list*|*image.index*)
      # Multi-platform manifest list — pick first platform's manifest
      local -r z_digest_file="${z_prefix}platform_digest.txt"
      local -r z_digest_stderr="${z_prefix}platform_digest_stderr.txt"
      jq -r '.manifests[0].digest // empty' "${z_manifest}" \
        > "${z_digest_file}" 2>"${z_digest_stderr}" \
        || buc_die_now "Failed to extract platform digest from manifest list — see ${z_digest_stderr}"
      local z_platform_digest
      z_platform_digest=$(<"${z_digest_file}") || buc_die_now "Failed to read: ${z_digest_file}"
      test -n "${z_platform_digest}" || buc_die_now "Empty platform digest in manifest list"

      local -r z_plat_manifest="${z_prefix}plat_manifest.json"
      local -r z_plat_stderr="${z_prefix}plat_manifest_stderr.txt"
      curl -sL \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
        -H "Authorization: Bearer ${z_token}" \
        -H "Accept: application/vnd.docker.distribution.manifest.v2+json,application/vnd.oci.image.manifest.v1+json" \
        "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/manifests/${z_platform_digest}" \
        > "${z_plat_manifest}" 2>"${z_plat_stderr}" \
        || z_curl_status=$?
      test "${z_curl_status}" -eq 0 \
        || buc_die_now "GET platform manifest failed for ${z_package} (curl exit ${z_curl_status}) — see ${z_plat_stderr}"
      z_single_manifest="${z_plat_manifest}"
      ;;
  esac

  # Extract all layer digests. Buildkit FROM-scratch images emit one layer per
  # COPY instruction (rbgja04's about-image stacks six: sbom, build_info,
  # recipe.txt, buildkit_metadata.json, cache_before.json, cache_after.json),
  # so a single-layer extract loses everything past the first COPY.
  local -r z_layers_file="${z_prefix}layer_digests.txt"
  local -r z_layers_stderr="${z_prefix}layer_digests_stderr.txt"
  jq -r '.layers[].digest' "${z_single_manifest}" \
    > "${z_layers_file}" 2>"${z_layers_stderr}" \
    || buc_die_now "Failed to extract layer digests from manifest — see ${z_layers_stderr}"
  test -s "${z_layers_file}" \
    || buc_die_now "No layer digests in manifest for ${z_package}:${z_tag}"

  mkdir -p "${z_extract_dir}" || buc_die_now "Failed to create extraction directory: ${z_extract_dir}"

  # Load-then-iterate: file fully consumed and closed before curl/tar run, so
  # no child process can silently consume the loop's remaining input.
  local z_layer_digests=()
  local z_layer_line=""
  while IFS= read -r z_layer_line || test -n "${z_layer_line}"; do
    z_layer_digests+=("${z_layer_line}")
  done < "${z_layers_file}"

  # Fetch and extract each layer in manifest order. FROM-scratch COPY layers
  # add disjoint files, so accumulation needs no whiteout handling.
  local z_layer_digest=""
  local z_blob_file=""
  local z_blob_stderr=""
  local z_tar_stderr=""
  local z_layer_idx=0
  for z_layer_idx in "${!z_layer_digests[@]}"; do
    z_layer_digest="${z_layer_digests[${z_layer_idx}]}"
    test -n "${z_layer_digest}" || continue
    z_blob_file="${z_prefix}blob_${z_layer_idx}.tar.gz"
    z_blob_stderr="${z_prefix}blob_${z_layer_idx}_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/blobs/${z_layer_digest}" \
      > "${z_blob_file}" 2>"${z_blob_stderr}" \
      || z_curl_status=$?
    test "${z_curl_status}" -eq 0 \
      || buc_die_now "GET blob failed for ${z_package} layer ${z_layer_digest} (curl exit ${z_curl_status}) — see ${z_blob_stderr}"

    z_tar_stderr="${z_prefix}tar_${z_layer_idx}_stderr.txt"
    tar -xzf "${z_blob_file}" -C "${z_extract_dir}" 2>"${z_tar_stderr}" \
      || buc_die_now "Failed to extract layer ${z_layer_digest} for ${z_package}:${z_tag} — see ${z_tar_stderr}"
  done

  return 0
}

# Internal: fetch an image's config blob (the .config.Labels carrier) via the
# Registry API — manifest -> .config.digest -> blob — no docker, no gcloud. Used by
# plumb to read the rbi_resolved_base_n labels back from the signed attest image.
# Resolves a multi-platform index to its first platform; image labels are identical
# across platforms (RBr_b4e), so any platform's config carries them.
# Args: token package tag out_config_file
# Writes the config JSON to out_config_file. Dies when the image is absent or on
# any infrastructure failure (curl, jq); a caller for whom absence is graceful
# (e.g. a pre-resolved-base hallmark) asks zrbfcg_manifest_code_fetch first.
zrbfcg_image_config_fetch() {
  zrbfc0_sentinel

  local -r z_token="${1:-}"
  local -r z_package="${2:-}"
  local -r z_tag="${3:-}"
  local -r z_out="${4:-}"
  test -n "${z_token}" || buc_die_now "zrbfcg_image_config_fetch: token required"
  test -n "${z_package}" || buc_die_now "zrbfcg_image_config_fetch: package required"
  test -n "${z_tag}" || buc_die_now "zrbfcg_image_config_fetch: tag required"
  test -n "${z_out}" || buc_die_now "zrbfcg_image_config_fetch: out required"

  local -r z_safe_pkg="${z_package//\//_}"
  local -r z_prefix="${BURD_TEMP_DIR}/cfg_${z_safe_pkg}_${z_tag}_"

  # HEAD check — does the image exist?
  zrbfcg_manifest_code_fetch "${z_token}" "${z_package}" "${z_tag}" "${z_prefix}head_code.txt"
  local z_http_code
  z_http_code=$(<"${z_prefix}head_code.txt") || buc_die_now "Failed to read: ${z_prefix}head_code.txt"
  test "${z_http_code}" = "200" \
    || buc_die_now "Not found in GAR: ${z_package}:${z_tag} (HTTP ${z_http_code})"
  local z_curl_status=0

  # GET manifest (may be index or single-platform)
  local -r z_manifest="${z_prefix}manifest.json"
  local -r z_manifest_stderr="${z_prefix}manifest_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBFC0_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/manifests/${z_tag}" \
    > "${z_manifest}" 2>"${z_manifest_stderr}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "GET manifest failed for ${z_package}:${z_tag} (curl exit ${z_curl_status}) — see ${z_manifest_stderr}"

  # Resolve to a single-platform manifest. The attest per-platform tag is already
  # single-platform, but the defensive index-resolve keeps the helper general.
  local z_single_manifest="${z_manifest}"
  local -r z_media_type_file="${z_prefix}media_type.txt"
  local -r z_media_type_stderr="${z_prefix}media_type_stderr.txt"
  jq -r '.mediaType // empty' "${z_manifest}" > "${z_media_type_file}" 2>"${z_media_type_stderr}" \
    || buc_die_now "Failed to read manifest mediaType for ${z_package}:${z_tag} — see ${z_media_type_stderr}"
  local z_media_type
  z_media_type=$(<"${z_media_type_file}") || buc_die_now "Failed to read: ${z_media_type_file}"
  case "${z_media_type}" in
    *manifest.list*|*image.index*)
      local -r z_digest_file="${z_prefix}platform_digest.txt"
      local -r z_digest_stderr="${z_prefix}platform_digest_stderr.txt"
      jq -r '.manifests[0].digest // empty' "${z_manifest}" > "${z_digest_file}" 2>"${z_digest_stderr}" \
        || buc_die_now "Failed to extract platform digest from manifest list — see ${z_digest_stderr}"
      local z_platform_digest
      z_platform_digest=$(<"${z_digest_file}") || buc_die_now "Failed to read: ${z_digest_file}"
      test -n "${z_platform_digest}" || buc_die_now "Empty platform digest in manifest list"
      local -r z_plat_manifest="${z_prefix}plat_manifest.json"
      local -r z_plat_stderr="${z_prefix}plat_manifest_stderr.txt"
      curl -sL \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
        -H "Authorization: Bearer ${z_token}" \
        -H "Accept: application/vnd.docker.distribution.manifest.v2+json,application/vnd.oci.image.manifest.v1+json" \
        "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/manifests/${z_platform_digest}" \
        > "${z_plat_manifest}" 2>"${z_plat_stderr}" \
        || z_curl_status=$?
      test "${z_curl_status}" -eq 0 \
        || buc_die_now "GET platform manifest failed for ${z_package} (curl exit ${z_curl_status}) — see ${z_plat_stderr}"
      z_single_manifest="${z_plat_manifest}"
      ;;
  esac

  # Extract the config blob digest and fetch the config JSON (carries .config.Labels).
  local -r z_config_digest_file="${z_prefix}config_digest.txt"
  local -r z_config_digest_stderr="${z_prefix}config_digest_stderr.txt"
  jq -r '.config.digest // empty' "${z_single_manifest}" > "${z_config_digest_file}" 2>"${z_config_digest_stderr}" \
    || buc_die_now "Failed to extract config digest from manifest for ${z_package}:${z_tag} — see ${z_config_digest_stderr}"
  local z_config_digest
  z_config_digest=$(<"${z_config_digest_file}") || buc_die_now "Failed to read: ${z_config_digest_file}"
  test -n "${z_config_digest}" || buc_die_now "No config digest in manifest for ${z_package}:${z_tag}"

  local -r z_config_stderr="${z_prefix}config_stderr.txt"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBFC0_REGISTRY_API_BASE}/${z_package}/blobs/${z_config_digest}" \
    > "${z_out}" 2>"${z_config_stderr}" \
    || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || buc_die_now "GET config blob failed for ${z_package}:${z_tag} (curl exit ${z_curl_status}) — see ${z_config_stderr}"

  return 0
}

# Resolve a hallmark's vessel by reading the vessel field from its vouch ark.
# Authenticates as Retriever, fetches the vouch ark, extracts vouch_summary.json,
# and emits the .vessel field. Single home for hallmark→vessel lookup; callers
# that need vessel identity for operation prose or logging route through here.
# Args: hallmark
# Emits: vessel sigil (e.g., "rbev_busybox")
rbfcg_vessel_for_hallmark_capture() {
  zrbfc0_sentinel

  local -r z_hallmark="${1:?Hallmark required}"

  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_retriever}") \
    || return 1

  local -r z_vouch_pkg="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}"
  local -r z_scratch="${BURD_TEMP_DIR}/rbfc_vessel_for_hallmark"
  rm -rf "${z_scratch}" || return 1

  zrbfcg_gar_extract_artifact "${z_token}" "${z_vouch_pkg}" "${z_hallmark}" "${z_scratch}"

  test -f "${z_scratch}/vouch_summary.json" \
    || return 1

  local -r z_vessel_file="${BURD_TEMP_DIR}/rbfc_vessel_for_hallmark_value.txt"
  jq -r '.vessel // empty' "${z_scratch}/vouch_summary.json" > "${z_vessel_file}" \
    || return 1
  local z_vessel
  z_vessel=$(<"${z_vessel_file}") || return 1
  test -n "${z_vessel}" || return 1

  printf '%s\n' "${z_vessel}" || return 1
}

# eof
