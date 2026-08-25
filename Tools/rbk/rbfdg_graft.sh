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
# Recipe Bottle Foundry Director Build - graft body (guard-free cluster,
# sourced by rbfd0_director): rbfd_graft — a locally-built image tagged and
# pushed into GAR under a hallmark minted from the image's own creation
# timestamp. Host-side docker throughout, no Cloud Build submission. Reads
# the entry's ZRBFD_GRAFT_PREFIX kindle constant; the registry preflight it
# calls lives in rbfdp_preflight.sh.

set -euo pipefail

######################################################################
# External Functions (rbfd_*)

rbfd_graft() {
  zrbfd_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Graft a locally-built image into GAR"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  # No-arg: list available vessels
  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die_now "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die_now "Vessel directory required"
  fi

  # Load and validate vessel
  zrbfc_load_vessel "${z_vessel_dir}"
  test "${RBRV_VESSEL_MODE:-}" = "rbnve_graft" \
    || buc_die_now "Vessel '${RBRV_SIGIL}' is not a graft vessel (mode: ${RBRV_VESSEL_MODE:-unset})"

  test -n "${RBRV_GRAFT_IMAGE:-}" \
    || buc_die_now "RBRV_GRAFT_IMAGE not set for graft vessel '${RBRV_SIGIL}' — anoint the vessel from a build, or set the slot by hand"

  # Resolve tool images from reliquary (graft about+vouch steps use tool images)
  zrbfc_resolve_tool_images

  local -r z_local_image="${RBRV_GRAFT_IMAGE}"

  # No dirty-tree guard — deliberate; RBr_d71.

  # Verify local image exists
  buc_step "Verifying local image exists"
  docker image inspect "${z_local_image}" > /dev/null 2>&1 \
    || buc_die_now "Local image not found: ${z_local_image} — build the image before grafting"
  buc_info "Local image confirmed: ${z_local_image}"

  # Extract image creation timestamp for hallmark T1
  buc_step "Reading image creation timestamp"
  local -r z_created_file="${ZRBFD_GRAFT_PREFIX}created.txt"
  docker image inspect --format '{{.Created}}' "${z_local_image}" > "${z_created_file}" \
    || buc_die_now "Failed to inspect image creation timestamp"
  local z_created=""
  z_created=$(<"${z_created_file}")
  test -n "${z_created}" || buc_die_now "Empty creation timestamp from docker inspect"
  buc_info "Image created: ${z_created}"

  # Parse ISO 8601 timestamp to YYMMDDHHMMSS
  # Input formats: 2024-01-15T10:30:45.123456789Z or 1970-01-01T00:00:00Z
  local z_created_clean="${z_created%%.*}"  # Remove fractional seconds
  z_created_clean="${z_created_clean%%Z}"   # Remove trailing Z if no fractional part
  z_created_clean="${z_created_clean%Z}"    # Handle edge case
  local -r z_cdate="${z_created_clean%%T*}"
  local -r z_ctime="${z_created_clean##*T}"
  local -r z_graft_ts="${RBGC_HALLMARK_PREFIX_GRAFT}${z_cdate:2:2}${z_cdate:5:2}${z_cdate:8:2}${z_ctime:0:2}${z_ctime:3:2}${z_ctime:6:2}"

  # Authenticate as Director
  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  # Registry preflight -- verify reliquary tool images exist (graft about+vouch use them)
  zrbfd_registry_preflight "${z_token}" "${z_vessel_dir}"

  # GAR coordinates
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_base="${z_gar_host}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"

  # Generate push timestamp (T2) for hallmark
  local -r z_push_ts_file="${ZRBFD_GRAFT_PREFIX}push_ts.txt"
  date -u +'%y%m%d%H%M%S' > "${z_push_ts_file}" || buc_die_now "Failed to generate push timestamp"
  local z_push_ts
  z_push_ts="r$(<"${z_push_ts_file}")"
  test -n "${z_push_ts}" || buc_die_now "Empty push timestamp from ${z_push_ts_file}"
  local -r z_hallmark="${z_graft_ts}-${z_push_ts}"
  local -r z_image_ref="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"

  buc_info "Hallmark: ${z_hallmark}"

  # Tag and push
  buc_step "Logging into GAR"
  rbgo_docker_login "${z_token}" "${z_gar_host}"

  buc_step "Tagging local image"
  docker tag "${z_local_image}" "${z_image_ref}" \
    || buc_die_now "Failed to tag local image as ${z_image_ref}"

  buc_step "Pushing to GAR"
  buc_info "Target: ${z_image_ref}"
  docker push "${z_image_ref}" \
    || buc_die_now "Failed to push image to GAR"

  buc_info "Image pushed: ${z_image_ref}"

  # Persist to output directory for downstream consumption
  echo "${z_vessel_dir}" > "${ZRBFC_OUTPUT_VESSEL_DIR}" \
    || buc_die_now "Failed to write vessel dir to output"
  buf_write_fact_single "${RBF_FACT_HALLMARK}" "${z_hallmark}"

  # Write GAR root fact file
  buf_write_fact_single "${RBF_FACT_GAR_ROOT}" "${z_gar_base}"

  # Write ark stem fact file (hallmark subtree under HALLMARKS_ROOT)
  buf_write_fact_single "${RBF_FACT_ARK_STEM}" "${RBGL_HALLMARKS_ROOT}/${z_hallmark}"

  # Write yield fact file (single-platform graft image)
  buf_write_fact_single "${RBF_FACT_ARK_YIELD}-${RBGC_ARK_BASENAME_IMAGE}" \
    "${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"

  # Summary
  echo ""
  buc_success "Graft complete: ${RBRV_SIGIL}"
  echo "  Hallmark: ${z_hallmark}"
  echo "  Source:  ${z_local_image}"
  echo "  Image:   ${z_image_ref}"
}

# eof
