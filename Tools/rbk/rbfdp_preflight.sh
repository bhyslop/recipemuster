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
# Recipe Bottle Foundry Director Build - registry preflight body (guard-free
# cluster, sourced by rbfd0_director): the host-side registry preflight chain,
# the one support shared across operations — build, mirror and graft each call
# zrbfd_registry_preflight after vessel load and authentication, and beneath it
# zrbfd_preflight_reliquary verifies the reliquary tool cohort against GAR.
# Owns no build-submission or step-composition machinery; reads the entry's
# ZRBFD_PREFLIGHT_PREFIX kindle constant for its probe files.

set -euo pipefail

######################################################################
# Internal Functions (zrbfd_*)

# Verify reliquary tool images exist in GAR.
# Args: token vessel_dir
zrbfd_preflight_reliquary() {
  zrbfd_sentinel

  local -r z_token="${1:-}"
  local -r z_vessel_dir="${2:-}"
  test -n "${z_token}"      || buc_die_now "zrbfd_preflight_reliquary: token required"
  test -n "${z_vessel_dir}" || buc_die_now "zrbfd_preflight_reliquary: vessel_dir required"

  local -r z_reliquary="${RBRV_RELIQUARY:-}"
  test -n "${z_reliquary}" || buc_die_now "RBRV_RELIQUARY required on every ordain-path vessel — yoke a reliquary touchmark via tt/${RBZ_YOKE_RELIQUARY}.sh before ordaining"

  buc_step "Verifying reliquary tool images exist in GAR"

  local -r z_canonical_tools=(
    "${RBGC_RELIQUARY_TOOL_GCLOUD}"
    "${RBGC_RELIQUARY_TOOL_DOCKER}"
    "${RBGC_RELIQUARY_TOOL_ALPINE}"
    "${RBGC_RELIQUARY_TOOL_SYFT}"
    "${RBGC_RELIQUARY_TOOL_BINFMT}"
    "${RBGC_RELIQUARY_TOOL_GCRANE}"
  )

  local z_missing=()
  local z_tool=""
  local z_pkg=""
  local z_tag=""
  local z_status_file=""
  local z_response_file=""
  local z_stderr_file=""
  local z_http_code=""

  for z_tool in "${z_canonical_tools[@]}"; do
    z_pkg="${RBGL_LODES_ROOT}/${z_reliquary}"
    z_tag="${RBGC_LODE_TAG_SPRUE}${z_tool}"
    z_status_file="${ZRBFD_PREFLIGHT_PREFIX}reliquary_${z_tool}_status.txt"
    z_response_file="${ZRBFD_PREFLIGHT_PREFIX}reliquary_${z_tool}_response.txt"
    z_stderr_file="${ZRBFD_PREFLIGHT_PREFIX}reliquary_${z_tool}_stderr.txt"

    local z_curl_status=0
    curl --head -sS \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
      -w "%{http_code}" \
      -o "${z_response_file}" \
      "${ZRBFC_REGISTRY_API_BASE}/${z_pkg}/manifests/${z_tag}" \
      > "${z_status_file}" 2>"${z_stderr_file}" \
      || z_curl_status=$?
    test "${z_curl_status}" -eq 0 \
      || buc_die_now "HEAD request failed for reliquary tool: ${z_pkg}:${z_tag} (curl exit ${z_curl_status}) — see ${z_stderr_file}"

    z_http_code=$(<"${z_status_file}")
    test -n "${z_http_code}" || buc_die_now "HTTP status code is empty for reliquary check: ${z_tool}"

    case "${z_http_code}" in
      200) buc_log_args "Reliquary tool present: ${z_tool}" ;;
      404) z_missing+=("${z_tool}") ;;
      *)   buc_die_now "Unexpected HTTP ${z_http_code} when checking reliquary tool: ${z_pkg}:${z_tag}" ;;
    esac
  done

  if test "${#z_missing[@]}" -eq 0; then
    buc_info "Reliquary verified: ${z_reliquary} (${#z_canonical_tools[@]}/${#z_canonical_tools[@]} tools present)"
    return 0
  fi

  buc_warn "Reliquary integrity check failed: ${z_reliquary} (${#z_missing[@]}/${#z_canonical_tools[@]} tools missing)"
  buc_bare "  The reliquary is a co-versioned set of builder tool images (gcloud, docker,"
  buc_bare "  syft, alpine, binfmt, gcrane) captured from upstream into your private GAR."
  buc_bare "  Air-gapped worker pools cannot pull from the public internet — the reliquary"
  buc_bare "  stages these tools so builds can run without egress. Piecemeal jettison is"
  buc_bare "  allowed but unrecoverable surgically: re-conclave the whole cohort."
  buc_bare ""
  for z_tool in "${z_missing[@]}"; do
    buc_bare "  PRECHECK: GAR image not found at ${RBGL_LODES_ROOT}/${z_reliquary}:${RBGC_LODE_TAG_SPRUE}${z_tool}"
    buc_bare "    Required by ${RBRV_SIGIL}'s RBRV_RELIQUARY=${z_reliquary}."
  done
  buc_bare ""
  buc_bare "  Recover by re-conclaving the reliquary, then re-yoking and re-ordaining:"
  buc_tabtarget "${RBZ_CONCLAVE_RELIQUARY}"
  buc_tabtarget "${RBZ_YOKE_RELIQUARY}" "<new-touchmark>"
  buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir}"
  buc_die_now "Registry preflight failed — ${#z_missing[@]} of ${#z_canonical_tools[@]} reliquary tool images missing from GAR"
}


# Internal: the host-side registry preflight (reliquary layer, then base-image layer).
# Must be called after vessel load (reads RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR)
# and authentication (needs token for registry API).
zrbfd_registry_preflight() {
  zrbfd_sentinel

  local -r z_token="${1:-}"
  local -r z_vessel_dir="${2:-}"
  test -n "${z_token}"      || buc_die_now "zrbfd_registry_preflight: token required"
  test -n "${z_vessel_dir}" || buc_die_now "zrbfd_registry_preflight: vessel_dir required"

  # --- Layer 1: Reliquary tool images ---
  zrbfd_preflight_reliquary "${z_token}" "${z_vessel_dir}"

  # --- Layer 2: Base images — anchor check ---

  buc_step "Verifying base images exist in GAR"

  local z_n=""
  local z_anchor_var=""
  local z_anchor=""
  local z_pkg_path=""
  local z_tag=""
  local z_origin_var=""
  local z_origin=""
  local z_any_checked="false"
  local z_status_file=""
  local z_response_file=""
  local z_stderr_file=""
  local z_http_code=""

  for z_n in 1 2 3; do
    z_origin_var="RBRV_IMAGE_${z_n}_ORIGIN"
    z_anchor_var="RBRV_IMAGE_${z_n}_ANCHOR"
    z_origin="${!z_origin_var:-}"
    z_anchor="${!z_anchor_var:-}"

    # Skip slots without an origin (no base image to capture).
    test -n "${z_origin}" || continue

    # Egress-mode anchor rule.
    if test -z "${z_anchor}"; then
      if test "${RBRV_EGRESS_MODE:-}" = "rbnve_airgap"; then
        # Bole vs hallmark-pin discrimination.
        if test -d "${RBRR_VESSEL_DIR}/${z_origin}"; then
          buc_warn "Airgap vessel ${RBRV_SIGIL} has empty ${z_anchor_var}; origin ${z_origin} names a producer vessel"
          buc_bare "  ${z_anchor_var} is a hallmark-pin, not a bole locator — ensconce is not invoked on this vessel."
          buc_bare "  Ordain the producer vessel first, then write its hallmark into ${z_anchor_var}."
          buc_bare "  Canonical handbook path:"
          buc_tabtarget "${RBZ_ONBOARD_DIR_AIRGAP}"
          buc_bare "  Minimal manual sequence:"
          buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${RBRR_VESSEL_DIR}/${z_origin}"
          buc_bare "    export PRODUCER_HALLMARK=\$(cat \${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK})"
          buc_bare "    # set ${z_anchor_var}=rbi_hm/\${PRODUCER_HALLMARK}/image:\${PRODUCER_HALLMARK}"
          buc_bare "    # in ${z_vessel_dir}/rbrv.env, then:"
          buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir}"
          buc_die_now "Registry preflight failed — airgap vessel missing hallmark-pin anchor"
        else
          buc_warn "Airgap vessel ${RBRV_SIGIL} has empty ${z_anchor_var} but non-empty ${z_origin_var}=${z_origin}"
          buc_bare "  Airgap conjure cannot reach upstream — base images must be captured (ensconced) first."
          buc_bare "  The anchor locator points at the captured base Lode inside GAR. Without it,"
          buc_bare "  the airgap worker pool has no source for the base image and the build fails."
          buc_bare "  Run ensconce, then re-run ordain:"
          buc_tabtarget "${RBZ_ENSCONCE_BOLE}" "${z_vessel_dir}"
          buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir}"
          buc_die_now "Registry preflight failed — airgap vessel missing required anchor"
        fi
      fi
      continue
    fi

    case "${z_anchor}" in
      *:*) : ;;
      *)   buc_die_now "Invalid ${z_anchor_var} locator format (expected package-path:tag): ${z_anchor}" ;;
    esac
    z_pkg_path="${z_anchor%:*}"
    z_tag="${z_anchor##*:}"
    test -n "${z_pkg_path}" || buc_die_now "Package path is empty in ${z_anchor_var}: ${z_anchor}"
    test -n "${z_tag}"      || buc_die_now "Tag is empty in ${z_anchor_var}: ${z_anchor}"

    z_any_checked="true"
    z_status_file="${ZRBFD_PREFLIGHT_PREFIX}base_${z_n}_status.txt"
    z_response_file="${ZRBFD_PREFLIGHT_PREFIX}base_${z_n}_response.txt"
    z_stderr_file="${ZRBFD_PREFLIGHT_PREFIX}base_${z_n}_stderr.txt"

    local z_curl_status=0
    curl --head -sS \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      -H "Accept: ${ZRBFC_ACCEPT_MANIFEST_MTYPES}" \
      -w "%{http_code}" \
      -o "${z_response_file}" \
      "${ZRBFC_REGISTRY_API_BASE}/${z_pkg_path}/manifests/${z_tag}" \
      > "${z_status_file}" 2>"${z_stderr_file}" \
      || z_curl_status=$?
    test "${z_curl_status}" -eq 0 \
      || buc_die_now "HEAD request failed for base image: ${z_anchor} (curl exit ${z_curl_status}) — see ${z_stderr_file}"

    z_http_code=$(<"${z_status_file}")
    test -n "${z_http_code}" || buc_die_now "HTTP status code is empty for base image check"

    if test "${z_http_code}" = "404"; then
      buc_warn "Base image Lode not found: ${z_anchor} (from ${z_origin})"
      buc_bare "  Ensconce captures upstream base images (e.g., busybox:latest from Docker Hub) into"
      buc_bare "  a bole Lode in your private GAR, pinned by content hash. Like the reliquary, this"
      buc_bare "  ensures air-gapped builds never reach the public internet. The anchor locator is"
      buc_bare "  stable until you deliberately re-ensconce to pick up a newer upstream version."
      buc_bare "  Multiple vessels sharing the same base image reuse one Lode."
      buc_bare "  Run ensconce, then re-run ordain:"
      buc_tabtarget "${RBZ_ENSCONCE_BOLE}" "${z_vessel_dir}"
      buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir}"
      buc_die_now "Registry preflight failed — base image Lode missing from GAR"
    elif test "${z_http_code}" != "200"; then
      buc_die_now "Unexpected HTTP ${z_http_code} when checking base image: ${z_anchor}"
    fi

    buc_log_args "Base image verified: ${z_anchor}"
  done

  if test "${z_any_checked}" = "true"; then
    buc_info "All base images verified in GAR"
  fi
}

# eof
