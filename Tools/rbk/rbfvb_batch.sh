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
# Recipe Bottle Foundry Verify - batch vouch body (guard-free cluster, sourced
# by rbfv0_verify): rbfv_batch_vouch — the two-pass driver that rolls every
# pending hallmark (image and about present, vouch absent) and calls rbfv_vouch
# on each. Composes rbfv_vouch from rbfvv_vouch.sh and owns no support and no
# kindle constant of its own.

set -euo pipefail

######################################################################
# External Functions (rbfv_*)

rbfv_batch_vouch() {
  zrbfv_sentinel

  buc_doc_brief "Vouch every pending hallmark (image+about present, vouch absent)"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  buc_step "Enumerating hallmarks under ${RBGL_HALLMARKS_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  # Load-then-iterate. A synthetic sentinel element appended to the array
  # lets the final hallmark flush through the same boundary branch as every
  # intermediate one (single flush site).
  local z_lines=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${ZRBFC_PACKAGE_LIST_FILE}"
  z_lines+=("__SENTINEL__ __SENTINEL__")

  # Pass 1: state machine identifies pending hallmarks (image+about present,
  # vouch absent). Accumulate into an array rather than a file so the later
  # processing loop can iterate in-memory (no FD held open across rbfv_vouch
  # calls, which issue curl/source and would silently consume loop stdin).
  local z_pending=()
  local z_prev_h=""
  local z_prev_img=0 z_prev_abt=0 z_prev_vch=0
  local z_i="" z_h="" z_b=""

  for z_i in "${!z_lines[@]}"; do
    z_line="${z_lines[$z_i]}"
    test -n "${z_line}" || continue

    z_h="${z_line%% *}"
    z_b="${z_line#* }"
    test -n "${z_h}" || continue
    test -n "${z_b}" || continue

    if test "${z_h}" != "${z_prev_h}"; then
      if test -n "${z_prev_h}"; then
        if test "${z_prev_img}" = "1" \
          && test "${z_prev_abt}" = "1" \
          && test "${z_prev_vch}" != "1"; then
          z_pending+=("${z_prev_h}")
        fi
      fi

      case "${z_h}" in
        __SENTINEL__) break ;;
      esac

      z_prev_h="${z_h}"
      z_prev_img=0
      z_prev_abt=0
      z_prev_vch=0
    fi

    case "${z_b}" in
      "${RBGC_ARK_BASENAME_IMAGE}") z_prev_img=1 ;;
      "${RBGC_ARK_BASENAME_ABOUT}") z_prev_abt=1 ;;
      "${RBGC_ARK_BASENAME_VOUCH}") z_prev_vch=1 ;;
    esac
  done

  # Forensic record of pending set.
  local -r z_pending_file="${BURD_TEMP_DIR}/rbfv_batch_pending.txt"
  : > "${z_pending_file}" || buc_die_now "Failed to initialize ${z_pending_file}"
  local z_j=""
  for z_j in "${!z_pending[@]}"; do
    printf '%s\n' "${z_pending[$z_j]}" >> "${z_pending_file}" \
      || buc_die_now "Failed to record pending hallmark ${z_pending[$z_j]}"
  done

  local -r z_total="${#z_pending[@]}"
  case "${z_total}" in
    0)
      buc_info "No pending hallmarks found (no hallmark has image+about without vouch)"
      buc_success "Batch vouch complete — 0 hallmarks processed"
      return 0
      ;;
  esac

  buc_info "Found ${z_total} pending hallmark(s) to vouch"

  # Pass 2: for each pending hallmark, extract about ark, read vessel_name
  # from build_info.json, resolve vessel_dir, call rbfv_vouch. rbfv_vouch
  # fails via buc_die_now on any vouch failure — batch terminates fail-fast at
  # the broken hallmark rather than masking problems behind a summary count.
  local z_vouched_n=0 z_idx=0 z_vessel=""
  local z_hallmark="" z_about_extract="" z_about_pkg=""
  local z_build_info="" z_vessel_scratch="" z_vessel_dir=""

  for z_j in "${!z_pending[@]}"; do
    z_hallmark="${z_pending[$z_j]}"
    test -n "${z_hallmark}" || continue
    z_idx=$(( z_idx + 1 ))

    buc_step "[${z_idx}/${z_total}] Resolving vessel for ${z_hallmark}"

    z_about_extract="${BURD_TEMP_DIR}/rbfv_batch_about_${z_hallmark}"
    z_about_pkg="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ABOUT}"

    zrbfc_gar_extract_artifact "${z_token}" "${z_about_pkg}" "${z_hallmark}" "${z_about_extract}" \
      || buc_die_now "Failed to extract about ark for ${z_hallmark} (expected at ${z_about_pkg}:${z_hallmark})"

    z_build_info="${z_about_extract}/build_info.json"
    test -f "${z_build_info}" \
      || buc_die_now "build_info.json missing in about ark for ${z_hallmark}"

    z_vessel_scratch="${BURD_TEMP_DIR}/rbfv_batch_vessel.txt"
    jq -r '.vessel_name // empty' "${z_build_info}" > "${z_vessel_scratch}" \
      || buc_die_now "Failed to read vessel_name from ${z_build_info}"
    z_vessel=$(<"${z_vessel_scratch}")
    test -n "${z_vessel}" \
      || buc_die_now "vessel_name empty in about ark for ${z_hallmark}"

    z_vessel_dir="${RBRR_VESSEL_DIR}/${z_vessel}"
    test -d "${z_vessel_dir}" \
      || buc_die_now "Vessel directory not found: ${z_vessel_dir} (from about build_info.vessel_name for ${z_hallmark})"

    buc_info "Vouching ${z_hallmark} (vessel: ${z_vessel})"
    rbfv_vouch "${z_vessel_dir}" "${z_hallmark}"
    z_vouched_n=$(( z_vouched_n + 1 ))
  done

  echo ""
  buc_success "Batch vouch complete — ${z_vouched_n}/${z_total} hallmark(s) vouched"
}

# eof
