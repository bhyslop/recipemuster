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
# Recipe Bottle Foundry Core - plumb cluster (guard-free, sourced by rbfck_):
# render a hallmark's trust posture (full and compact) from its GAR about/vouch
# arks - the rbw-fpf / rbw-fpc surface.

set -euo pipefail

# The gate codes this module rejects with, from rb's generated constants. The
# sourced file carries its own inclusion guard, so a sibling sourcing it costs
# nothing.
source "${BASH_SOURCE[0]%/*}/rbcg_breviary.sh" || buc_die_now "Failed to source rbcg_breviary.sh"

######################################################################
# Plumb (zrbfcp_* / rbfcp_*)

# Internal: core plumb logic shared by full and compact modes.
# Queries GAR directly via Registry API — no local docker required.
# Authenticates as Retriever. Vessel is resolved from the hallmark's vouch ark.
# Args: hallmark mode
zrbfcp_plumb_core() {
  zrbfc0_sentinel

  local -r z_express="${1:-}"
  local -r z_mode="${2:-}"
  test -n "${z_mode}" || buc_die_now "zrbfcp_plumb_core: mode required"

  # Relay-then-read (RBr_3e7): forward the chain baton before any read or failure point.
  buf_relay || buc_die_now "Failed to relay chained facts"

  # Resolve the hallmark express-or-chain: an express argument wins; absent, fall
  # back to the hallmark a prior build (ordain or kludge) handed forward through
  # the depth-1 chain — so a no-arg plumb immediately after a build inspects the
  # just-built hallmark.
  local z_hallmark=""
  local z_hallmark_status=0
  z_hallmark=$(buf_elect_fact_capture "${z_express}" "${RBF_FACT_HALLMARK}") || z_hallmark_status=$?
  test "${z_hallmark_status}" -eq 0 || buc_reject "${RBCG_CHAIN_DENTIL}" "No hallmark — pass one or run a build (ordain/kludge) immediately before plumb"

  buc_step "Authenticating as Retriever"
  local zSECRET_token=""
  zSECRET_token=$(rbau_token_capture "${RBCC_mantle_retriever}") \
    || buc_die_now "Failed to get Retriever OAuth token"

  local -r z_vouch_pkg="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}"
  zrbfcg_manifest_code_fetch "${zSECRET_token}" "${z_vouch_pkg}" "${z_hallmark}" "${ZRBFC0_PLUMB_PREFIX}_vouch_head_code.txt"
  local z_vouch_code
  z_vouch_code=$(<"${ZRBFC0_PLUMB_PREFIX}_vouch_head_code.txt") || buc_die_now "Failed to read: ${ZRBFC0_PLUMB_PREFIX}_vouch_head_code.txt"
  test "${z_vouch_code}" = "200" \
    || buc_reject "${RBCG_VACANT_DENTIL}" "Hallmark not found: ${z_hallmark} (no vouch ark at ${z_vouch_pkg}:${z_hallmark})"

  buc_step "Resolving vessel from vouch ark"
  local z_vessel=""
  z_vessel=$(rbfcg_vessel_for_hallmark_capture "${z_hallmark}") \
    || buc_die_now "Failed to resolve vessel for hallmark: ${z_hallmark}"
  buc_info "Resolved hallmark to vessel: ${z_vessel}"

  rbfcv_require_vessel_sigil "${z_vessel}"

  # Load vessel config (sets RBRV_VESSEL_MODE, RBRV_BIND_IMAGE, etc.)
  local -r z_vessel_dir="${RBRR_VESSEL_DIR}/${z_vessel}"
  zrbfcv_load_vessel "${z_vessel_dir}"

  local -r z_extract="${ZRBFC0_PLUMB_PREFIX}"
  mkdir -p "${z_extract}" || buc_die_now "Failed to create extraction directory"
  local z_has_about=false
  local z_has_vouch=false

  # Fetch about ark from GAR (tag = hallmark)
  buc_step "Fetching about ark from GAR"
  local -r z_about_pkg="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ABOUT}"
  local -r z_about_dir="${ZRBFC0_PLUMB_PREFIX}_about"
  zrbfcg_manifest_code_fetch "${zSECRET_token}" "${z_about_pkg}" "${z_hallmark}" "${ZRBFC0_PLUMB_PREFIX}_about_head_code.txt"
  local z_about_code
  z_about_code=$(<"${ZRBFC0_PLUMB_PREFIX}_about_head_code.txt") || buc_die_now "Failed to read: ${ZRBFC0_PLUMB_PREFIX}_about_head_code.txt"
  if test "${z_about_code}" = "200"; then
    zrbfcg_gar_extract_artifact "${zSECRET_token}" "${z_about_pkg}" "${z_hallmark}" "${z_about_dir}"
    z_has_about=true
    # Non-fatal by grant: the ark may hold no files, and the plumb display shows what it finds
    cp "${z_about_dir}"/* "${z_extract}/" 2>"${ZRBFC0_PLUMB_PREFIX}_about_cp_stderr.txt" \
      || buc_warn "Copied nothing from ${z_about_dir} — see ${ZRBFC0_PLUMB_PREFIX}_about_cp_stderr.txt"
  fi

  # Fetch vouch ark for content (vessel resolution already extracted it once above)
  buc_step "Fetching vouch ark from GAR"
  local -r z_vouch_dir="${ZRBFC0_PLUMB_PREFIX}_vouch"
  if test "${z_vouch_code}" = "200"; then
    zrbfcg_gar_extract_artifact "${zSECRET_token}" "${z_vouch_pkg}" "${z_hallmark}" "${z_vouch_dir}"
    z_has_vouch=true
    # Non-fatal by grant: the ark may hold no files, and the plumb display shows what it finds
    cp "${z_vouch_dir}"/* "${z_extract}/" 2>"${ZRBFC0_PLUMB_PREFIX}_vouch_cp_stderr.txt" \
      || buc_warn "Copied nothing from ${z_vouch_dir} — see ${ZRBFC0_PLUMB_PREFIX}_vouch_cp_stderr.txt"
  fi

  # Bind vessels: fallback to static display if no about ark
  if test "${RBRV_VESSEL_MODE}" = "rbnve_bind" && test "${z_has_about}" = "false"; then
    zrbfcp_plumb_show_bind "${z_vessel}" "${z_hallmark}" "${z_mode}"
    return 0
  fi

  # Require about ark for non-bind vessels
  if test "${z_has_about}" = "false"; then
    buc_die_now "About ark not found in GAR: ${z_about_pkg}:${z_hallmark}"
  fi

  # Resolved-base labels (conjure only): read back from the first built
  # platform's attest tag; RBr_b4e.
  if test "${RBRV_VESSEL_MODE}" = "rbnve_conjure" && test -n "${RBRV_CONJURE_PLATFORMS:-}"; then
    buc_step "Reading resolved-base labels from attest image config"
    local z_first_plat="${RBRV_CONJURE_PLATFORMS//,/ }"
    z_first_plat="${z_first_plat%% *}"
    local z_attest_suffix="${z_first_plat#linux/}"
    z_attest_suffix="${z_attest_suffix//\//}"
    local -r z_attest_pkg="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ATTEST}"
    local -r z_attest_tag="${z_hallmark}-${z_attest_suffix}"
    zrbfcg_manifest_code_fetch "${zSECRET_token}" "${z_attest_pkg}" "${z_attest_tag}" "${ZRBFC0_PLUMB_PREFIX}_attest_head_code.txt"
    local z_attest_code
    z_attest_code=$(<"${ZRBFC0_PLUMB_PREFIX}_attest_head_code.txt") || buc_die_now "Failed to read: ${ZRBFC0_PLUMB_PREFIX}_attest_head_code.txt"
    if test "${z_attest_code}" = "200"; then
      zrbfcg_image_config_fetch "${zSECRET_token}" "${z_attest_pkg}" "${z_attest_tag}" \
        "${z_extract}/attest_config.json"
    else
      buc_info "Resolved-base labels not available (attest ${z_attest_tag} absent — pre-resolved-base build?)"
    fi
  fi

  # Display results
  if test "${z_mode}" = "compact"; then
    zrbfcp_plumb_show_compact "${z_vessel}" "${z_hallmark}" "${z_extract}" "${z_has_vouch}"
  else
    zrbfcp_plumb_show_full "${z_vessel}" "${z_hallmark}" "${z_extract}" "${z_has_vouch}"
  fi
}

# Internal: display bind vessel info
# Args: vessel hallmark mode
zrbfcp_plumb_show_bind() {
  zrbfc0_sentinel

  local -r z_vessel="${1:-}"
  local -r z_hallmark="${2:-}"
  local -r z_mode="${3:-}"
  test -n "${z_vessel}" || buc_die_now "zrbfcp_plumb_show_bind: vessel required"
  test -n "${z_hallmark}" || buc_die_now "zrbfcp_plumb_show_bind: hallmark required"
  test -n "${z_mode}" || buc_die_now "zrbfcp_plumb_show_bind: mode required"

  if test "${z_mode}" = "compact"; then
    echo "" || buc_die_now
    echo "=== ${z_vessel} / ${z_hallmark} ===" || buc_die_now
    echo "  Type: bind | Trust: digest-pin only" || buc_die_now
    if test -n "${RBRV_BIND_IMAGE:-}"; then
      echo "  Source: ${RBRV_BIND_IMAGE}" || buc_die_now
    fi
    echo "  No SLSA provenance, SBOM, or build transcript (not built by GCB)" || buc_die_now
    echo "" || buc_die_now
    return 0
  fi

  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "  HALLMARK PLUMB: ${z_vessel} / ${z_hallmark}" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "" || buc_die_now
  echo "  Vessel type:  BIND (external image pinned by digest)" || buc_die_now
  echo "  Trust model:  Digest-pin only" || buc_die_now
  echo "" || buc_die_now
  if test -n "${RBRV_BIND_IMAGE:-}"; then
    echo "  Bind source:  ${RBRV_BIND_IMAGE}" || buc_die_now
  fi
  echo "" || buc_die_now
  echo "  TRUST BOUNDARY" || buc_die_now
  echo "  This is a bind vessel. The image was not built by Google Cloud" || buc_die_now
  echo "  Build. No SLSA provenance, no SBOM, and no build transcript" || buc_die_now
  echo "  exist because GCB did not produce this image." || buc_die_now
  echo "" || buc_die_now
  echo "  Trust is based solely on digest pinning of a known-good" || buc_die_now
  echo "  external image from its source registry." || buc_die_now
  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "" || buc_die_now
}

# Internal: shared section rendering used by both compact and full modes
# Args: extract_dir has_vouch
# Outputs: vessel type, source, builder, SLSA, SBOM summary, vouch results
zrbfcp_plumb_show_sections() {
  zrbfc0_sentinel

  local -r z_dir="${1:-}"
  local -r z_has_vouch="${2:-}"
  test -n "${z_dir}" || buc_die_now "zrbfcp_plumb_show_sections: dir required"

  local -r z_bi="${z_dir}/build_info.json"
  local -r z_sbom="${z_dir}/sbom.json"
  local -r z_vs="${z_dir}/vouch_summary.json"
  local -r z_bkmeta="${z_dir}/buildkit_metadata.json"

  # Determine vessel mode from build_info.json
  local z_vessel_mode="rbnve_conjure"
  if test -f "${z_bi}"; then
    local z_mode_raw
    local -r z_mode_stderr="${ZRBFC0_PLUMB_PREFIX}_mode_stderr.txt"
    jq -r '.mode // "rbnve_conjure"' "${z_bi}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_mode_stderr}" \
      || echo "rbnve_conjure" > "${ZRBFC0_SCRATCH_FILE}" \
      || buc_die_now "Failed to write fallback vessel mode to ${ZRBFC0_SCRATCH_FILE}"
    local z_mode_raw_status=0
    z_mode_raw=$(<"${ZRBFC0_SCRATCH_FILE}") || z_mode_raw_status=$?
    test "${z_mode_raw_status}" -eq 0 || z_mode_raw=""
    z_vessel_mode="${z_mode_raw}"
  fi

  if test -f "${z_bi}" && test "${z_vessel_mode}" = "rbnve_bind"; then
    # ── Bind vessel sections ──────────────────────────────────────────
    # Batch extract bind fields from build_info.json
    local z_bi_moniker="" z_bi_source_img="" z_bi_mirror_ts="" z_bi_hallmark=""
    local z_bi_image_uri="" z_bi_git_repo="" z_bi_git_branch="" z_bi_git_commit=""
    local -r z_bi_stderr="${ZRBFC0_PLUMB_PREFIX}_bind_fields_stderr.txt"
    jq -r '
      (.moniker // "?"),
      (.source.image_ref // "?"),
      (.build.inscribe_timestamp // "?"),
      (.build.hallmark // "?"),
      (.image.uri // "?"),
      (.git.repo // "?"),
      (.git.branch // "?"),
      (.git.commit // "?")
    ' "${z_bi}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_bi_stderr}" \
      || buc_die_now "Failed to extract bind fields from ${z_bi} — see ${z_bi_stderr}"
    { read -r z_bi_moniker     || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_source_img  || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_mirror_ts   || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_hallmark    || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_image_uri   || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_git_repo    || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_git_branch  || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bi_git_commit  || buc_die_now "Failed to read bind fields from ${ZRBFC0_SCRATCH_FILE}"
    } < "${ZRBFC0_SCRATCH_FILE}"

    echo "" || buc_die_now
    echo "  -- Vessel Type -------------------------------------------------" || buc_die_now
    echo "  How this image was produced." || buc_die_now
    echo "" || buc_die_now
    echo "  Mode:           bind (upstream image mirrored to GAR)" || buc_die_now
    echo "  Moniker:        ${z_bi_moniker}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Upstream Source -----------------------------------------------" || buc_die_now
    echo "  The digest-pinned upstream image that was mirrored." || buc_die_now
    echo "" || buc_die_now
    echo "  Source image:   ${z_bi_source_img}" || buc_die_now
    echo "  Trust model:    digest-pin (image identity is the digest itself)" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Mirror -------------------------------------------------------" || buc_die_now
    echo "  When the image was mirrored from upstream into GAR." || buc_die_now
    echo "" || buc_die_now
    echo "  Mirror time:    ${z_bi_mirror_ts}" || buc_die_now
    echo "  Hallmark:   ${z_bi_hallmark}" || buc_die_now
    echo "  Image URI:      ${z_bi_image_uri}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Git Context --------------------------------------------------" || buc_die_now
    echo "  The repository state when the mirror operation was performed." || buc_die_now
    echo "" || buc_die_now
    echo "  Repository:     ${z_bi_git_repo}" || buc_die_now
    echo "  Branch:         ${z_bi_git_branch}" || buc_die_now
    echo "  Commit:         ${z_bi_git_commit}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Trust --------------------------------------------------------" || buc_die_now
    echo "  Bind vessels are NOT built by Cloud Build. Trust comes from the" || buc_die_now
    echo "  digest pin in rbrv.env — the image is exactly the bytes specified." || buc_die_now
    echo "" || buc_die_now
    echo "  SLSA provenance:  not applicable (no build step)" || buc_die_now
    echo "  Verification:     image digest matches the pin in the vessel definition" || buc_die_now

  elif test -f "${z_bi}"; then
    # ── Conjure vessel sections ───────────────────────────────────────
    # Batch extract build-info fields from build_info.json
    local z_platform="" z_qemu="" z_cj_moniker=""
    local z_cj_git_repo="" z_cj_git_branch="" z_cj_git_commit=""
    local z_cj_build_id="" z_cj_build_ts="" z_cj_inscribe_ts="" z_cj_image_uri=""
    local z_slsa_level="" z_slsa_invocation="" z_slsa_builder=""
    local -r z_cj_stderr="${ZRBFC0_PLUMB_PREFIX}_conjure_fields_stderr.txt"
    jq -r '
      (.platform // "unknown"),
      (.qemu_used // "false"),
      (.moniker // "?"),
      (.git.repo // "?"),
      (.git.branch // "?"),
      (.git.commit // "?"),
      (.build.build_id // "?"),
      (.build.timestamp // "?"),
      (.build.inscribe_timestamp // "?"),
      (.image.uri // "?"),
      (.slsa.build_level // "?"),
      (.slsa.build_invocation_id // "?"),
      (.slsa.provenance_builder_id // "?")
    ' "${z_bi}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_cj_stderr}" \
      || buc_die_now "Failed to extract build-info fields from ${z_bi} — see ${z_cj_stderr}"
    { read -r z_platform         || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_qemu             || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_moniker       || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_git_repo      || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_git_branch    || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_git_commit    || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_build_id      || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_build_ts      || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_inscribe_ts   || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_cj_image_uri     || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_slsa_level       || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_slsa_invocation  || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_slsa_builder     || buc_die_now "Failed to read build-info fields from ${ZRBFC0_SCRATCH_FILE}"
    } < "${ZRBFC0_SCRATCH_FILE}"

    echo "" || buc_die_now
    echo "  -- Vessel Type -------------------------------------------------" || buc_die_now
    echo "  How this image was produced and for which CPU architecture." || buc_die_now
    echo "" || buc_die_now
    echo "  Mode:           conjure (built by Google Cloud Build)" || buc_die_now
    local z_strategy="native"
    if test "${z_qemu}" = "true"; then z_strategy="emulated (QEMU)"; fi
    echo "  Platform:       ${z_platform} (host-platform view)" || buc_die_now
    echo "  Build strategy: ${z_strategy}" || buc_die_now
    echo "  Moniker:        ${z_cj_moniker}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Source -------------------------------------------------------" || buc_die_now
    echo "  The git repository, branch, and commit that produced this build." || buc_die_now
    echo "" || buc_die_now
    echo "  Repository:     ${z_cj_git_repo}" || buc_die_now
    echo "  Branch:         ${z_cj_git_branch}" || buc_die_now
    echo "  Commit:         ${z_cj_git_commit}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- Builder ------------------------------------------------------" || buc_die_now
    echo "  The Cloud Build job that executed this build, with timestamps." || buc_die_now
    echo "" || buc_die_now
    echo "  Build ID:       ${z_cj_build_id}" || buc_die_now
    echo "  Build time:     ${z_cj_build_ts}" || buc_die_now
    echo "  Inscribe time:  ${z_cj_inscribe_ts}" || buc_die_now
    echo "  Image URI:      ${z_cj_image_uri}" || buc_die_now

    echo "" || buc_die_now
    echo "  -- SLSA Provenance ----------------------------------------------" || buc_die_now
    echo "  Cryptographic proof linking this exact image digest to its build." || buc_die_now
    echo "" || buc_die_now
    echo "  Build level:    ${z_slsa_level}" || buc_die_now
    echo "  Invocation ID:  ${z_slsa_invocation}" || buc_die_now
    echo "  Builder ID:     ${z_slsa_builder}" || buc_die_now
    echo "  Predicate types:" || buc_die_now
    local -r z_pt_stderr="${ZRBFC0_PLUMB_PREFIX}_predicate_types_stderr.txt"
    jq -r '.slsa.provenance_predicate_types[]?' "${z_bi}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_pt_stderr}" \
      || buc_die_now "Failed to extract SLSA predicate types from ${z_bi} — see ${z_pt_stderr}"
    local z_pt=""
    while IFS= read -r z_pt || test -n "${z_pt}"; do
      echo "                    ${z_pt}" || buc_die_now
    done < "${ZRBFC0_SCRATCH_FILE}" || buc_die_now "Failed to read SLSA predicate types from ${ZRBFC0_SCRATCH_FILE}"

    echo "" || buc_die_now
    echo "  SLSA Build L${z_slsa_level} attests:" || buc_die_now
    echo "    + This digest was produced by this Cloud Build invocation" || buc_die_now
    echo "    + From this source repo and commit" || buc_die_now
    echo "    + On Google's hosted builder (tamper-resistant environment)" || buc_die_now
    echo "" || buc_die_now
    echo "  SLSA Build L${z_slsa_level} does NOT attest:" || buc_die_now
    echo "    - Base image security or supply chain" || buc_die_now
    echo "    - Package integrity within the image" || buc_die_now
    echo "    - Absence of vulnerabilities" || buc_die_now
    echo "    - Correctness or security of the Dockerfile" || buc_die_now
  else
    echo "" || buc_die_now
    echo "  build_info.json not found in -about artifact" || buc_die_now
  fi

  # Base image section — conjure only (bind has no Dockerfile)
  if test "${z_vessel_mode}" != "rbnve_bind"; then
    echo "" || buc_die_now
    echo "  -- Base Image ---------------------------------------------------" || buc_die_now
    echo "  The upstream image this build started FROM and the OS syft detected." || buc_die_now
    echo "" || buc_die_now
    local -r z_recipe="${z_dir}/recipe.txt"
    if test -f "${z_recipe}"; then
      local z_from_line=""
      local z_recipe_line=""
      while IFS= read -r z_recipe_line || test -n "${z_recipe_line}"; do
        case "${z_recipe_line}" in [Ff][Rr][Oo][Mm]\ *) z_from_line="${z_recipe_line}"; break ;; esac
      done < "${z_recipe}" || buc_die_now "Failed to read recipe ${z_recipe}"
      if test -n "${z_from_line}"; then
        echo "  Dockerfile FROM: ${z_from_line#FROM }" || buc_die_now
      fi
    fi
    if test -f "${z_sbom}"; then
      local z_distro_name="" z_distro_ver=""
      local -r z_distro_stderr="${ZRBFC0_PLUMB_PREFIX}_distro_stderr.txt"
      jq -r '(.distro.name // empty), (.distro.version // empty)' "${z_sbom}" \
        > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_distro_stderr}" \
        || buc_die_now "Failed to extract distro from ${z_sbom} — see ${z_distro_stderr}"
      { read -r z_distro_name || buc_die_now "Failed to read distro name from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_distro_ver  || buc_die_now "Failed to read distro version from ${ZRBFC0_SCRATCH_FILE}"
      } < "${ZRBFC0_SCRATCH_FILE}"
      if test -n "${z_distro_name}"; then
        echo "  Detected distro: ${z_distro_name} ${z_distro_ver}" || buc_die_now
      fi
    fi

    # Resolved base (signed); RBr_b4e.
    local -r z_attest_config="${z_dir}/attest_config.json"
    if test -f "${z_attest_config}"; then
      local z_rb_n=""
      local z_rb_val=""
      local z_rb_any="false"
      local z_rb_stderr=""
      for z_rb_n in 1 2 3; do
        # (.config.Labels // {}) tolerates an image with no labels (null) in-jq, so
        # only a malformed config or a jq fault reaches the guard. buc_warn (not
        # buc_die_now) is deliberate: plumb is a read-only trust-posture display, and a
        # single unreadable optional label must not abort a report already in flight.
        z_rb_stderr="${ZRBFC0_PLUMB_PREFIX}_resolved_base_${z_rb_n}_stderr.txt"
        # Non-fatal by grant: the label is optional attest metadata, and the display reports its absence
        jq -r --arg k "${RBGC_IMAGE_LABEL_RESOLVED_BASE}_${z_rb_n}" \
          '(.config.Labels // {})[$k] // empty' "${z_attest_config}" \
          > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_rb_stderr}" \
          || buc_warn "Could not read ${RBGC_IMAGE_LABEL_RESOLVED_BASE}_${z_rb_n} from attest config — see ${z_rb_stderr}"
        local z_rb_val_status=0
        z_rb_val=$(<"${ZRBFC0_SCRATCH_FILE}") || z_rb_val_status=$?
        test "${z_rb_val_status}" -eq 0 || z_rb_val=""
        if test -n "${z_rb_val}"; then
          echo "  Resolved base ${z_rb_n} (signed): ${z_rb_val}" || buc_die_now
          z_rb_any="true"
        fi
      done
      if test "${z_rb_any}" = "false"; then
        echo "  Resolved base:   (no rbi_resolved_base_n labels — pre-resolved-base build)" || buc_die_now
      fi
    fi
  fi

  # Build output — conjure only (bind has no buildx step)
  if test "${z_vessel_mode}" != "rbnve_bind" && test -f "${z_bkmeta}"; then
    echo "" || buc_die_now
    echo "  -- Build Output -------------------------------------------------" || buc_die_now
    echo "  The container image manifest produced by this buildx invocation." || buc_die_now
    echo "" || buc_die_now
    local z_bk_digest="" z_bk_mediatype="" z_bk_ref="" z_bk_imgname=""
    local -r z_bk_stderr="${ZRBFC0_PLUMB_PREFIX}_buildkit_stderr.txt"
    jq -r '
      (."containerimage.digest" // ""),
      (."containerimage.descriptor".mediaType // ""),
      (."buildx.build.ref" // ""),
      (."image.name" // "")
    ' "${z_bkmeta}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_bk_stderr}" \
      || buc_die_now "Failed to extract buildkit metadata from ${z_bkmeta} — see ${z_bk_stderr}"
    { read -r z_bk_digest     || buc_die_now "Failed to read buildkit metadata from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bk_mediatype  || buc_die_now "Failed to read buildkit metadata from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bk_ref        || buc_die_now "Failed to read buildkit metadata from ${ZRBFC0_SCRATCH_FILE}"
      read -r z_bk_imgname    || buc_die_now "Failed to read buildkit metadata from ${ZRBFC0_SCRATCH_FILE}"
    } < "${ZRBFC0_SCRATCH_FILE}"
    if test -n "${z_bk_digest}"; then
      echo "  Output digest:  ${z_bk_digest}" || buc_die_now
    fi
    if test -n "${z_bk_mediatype}"; then
      echo "  Media type:     ${z_bk_mediatype}" || buc_die_now
    fi
    if test -n "${z_bk_ref}"; then
      echo "  Build ref:      ${z_bk_ref}" || buc_die_now
    fi
    if test -n "${z_bk_imgname}"; then
      echo "  Image name:     ${z_bk_imgname}" || buc_die_now
    fi
    # Per-platform digests if present
    local z_bk_platforms=""
    # Non-fatal by grant: per-platform keys are present only on multi-platform builds
    jq -r 'keys[] | select(contains("/"))' "${z_bkmeta}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${ZRBFC0_PLUMB_PREFIX}_bk_platforms_stderr.txt" \
      || buc_warn "No per-platform keys read from ${z_bkmeta} — see ${ZRBFC0_PLUMB_PREFIX}_bk_platforms_stderr.txt"
    local z_bk_platforms_status=0
    z_bk_platforms=$(<"${ZRBFC0_SCRATCH_FILE}") || z_bk_platforms_status=$?
    test "${z_bk_platforms_status}" -eq 0 || z_bk_platforms=""
    if test -n "${z_bk_platforms}"; then
      echo "  Per-platform digests:" || buc_die_now
      local z_bk_plat_roll=()
      local z_bk_plat_line=""
      while IFS= read -r z_bk_plat_line || test -n "${z_bk_plat_line}"; do
        z_bk_plat_roll+=("${z_bk_plat_line}")
      done <<< "${z_bk_platforms}"
      local z_bk_plat=""
      local z_bk_pd=""
      local z_bk_pd_i=0
      for z_bk_pd_i in "${!z_bk_plat_roll[@]}"; do
        z_bk_plat="${z_bk_plat_roll[${z_bk_pd_i}]}"
        # Non-fatal by grant: per-platform digests are present only on multi-platform builds
        jq -r --arg p "${z_bk_plat}" '.[$p]["containerimage.digest"] // empty' "${z_bkmeta}" \
          > "${ZRBFC0_SCRATCH_FILE}" 2>"${ZRBFC0_PLUMB_PREFIX}_bk_digest_${z_bk_pd_i}_stderr.txt" \
          || buc_warn "No digest read for ${z_bk_plat} — see ${ZRBFC0_PLUMB_PREFIX}_bk_digest_${z_bk_pd_i}_stderr.txt"
        local z_bk_pd_status=0
        z_bk_pd=$(<"${ZRBFC0_SCRATCH_FILE}") || z_bk_pd_status=$?
        test "${z_bk_pd_status}" -eq 0 || z_bk_pd=""
        if test -n "${z_bk_pd}"; then
          echo "    ${z_bk_plat}: ${z_bk_pd}" || buc_die_now
        fi
      done
    fi
  fi

  # Build cache delta — conjure only
  if test "${z_vessel_mode}" != "rbnve_bind"; then
    local -r z_cache_before="${z_dir}/cache_before.json"
    local -r z_cache_after="${z_dir}/cache_after.json"
    if test -f "${z_cache_after}"; then
      echo "" || buc_die_now
      echo "  -- Build Cache Delta --------------------------------------------" || buc_die_now
      echo "  Images on the Cloud Build worker after vs before this build." || buc_die_now
      echo "" || buc_die_now
      local z_before_count="n/a"
      local z_after_count=""
      local -r z_before_stderr="${ZRBFC0_PLUMB_PREFIX}_cache_before_stderr.txt"
      local -r z_after_stderr="${ZRBFC0_PLUMB_PREFIX}_cache_after_stderr.txt"
      if test -f "${z_cache_before}"; then
        jq '.host_daemon_images | length' "${z_cache_before}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_before_stderr}" \
          || echo "?" > "${ZRBFC0_SCRATCH_FILE}" \
          || buc_die_now "Failed to write fallback image count to ${ZRBFC0_SCRATCH_FILE}"
        local z_before_count_status=0
        z_before_count=$(<"${ZRBFC0_SCRATCH_FILE}") || z_before_count_status=$?
        test "${z_before_count_status}" -eq 0 || z_before_count=""
      fi
      jq '.host_daemon_images | length' "${z_cache_after}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_after_stderr}" \
        || echo "?" > "${ZRBFC0_SCRATCH_FILE}" \
        || buc_die_now "Failed to write fallback image count to ${ZRBFC0_SCRATCH_FILE}"
      local z_after_count_status=0
      z_after_count=$(<"${ZRBFC0_SCRATCH_FILE}") || z_after_count_status=$?
      test "${z_after_count_status}" -eq 0 || z_after_count=""
      echo "  Images before: ${z_before_count}" || buc_die_now
      echo "  Images after:  ${z_after_count}" || buc_die_now
      if test -f "${z_cache_before}"; then
        local z_new_images=""
        # Non-fatal by grant: a display of what the daemon gained, nothing depends on it
        jq -r --slurpfile before "${z_cache_before}" '
          ($before[0].host_daemon_images // [] | map(.ID) | unique) as $before_ids |
          [(.host_daemon_images // [])[] |
           select(.ID as $id | $before_ids | index($id) | not)] |
          group_by(.ID) |
          map(.[0] |
            (if (.Repository | split("/") | length) > 2 then
              (.Repository | split("/") | .[-1])
            else .Repository end) as $short |
            [$short, .Tag, .Size, .ID[7:19]] | @tsv) |
          .[]
        ' "${z_cache_after}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${ZRBFC0_PLUMB_PREFIX}_new_images_stderr.txt" \
          || buc_warn "New-images listing unreadable — see ${ZRBFC0_PLUMB_PREFIX}_new_images_stderr.txt"
        local z_new_images_status=0
        z_new_images=$(<"${ZRBFC0_SCRATCH_FILE}") || z_new_images_status=$?
        test "${z_new_images_status}" -eq 0 || z_new_images=""
        if test -n "${z_new_images}"; then
          local z_new_count=0
          local z_count_line=""
          while IFS= read -r z_count_line || test -n "${z_count_line}"; do
            z_new_count=$((z_new_count + 1))
          done <<< "${z_new_images}"
          echo "" || buc_die_now
          echo "  New images (${z_new_count} unique):" || buc_die_now
          printf '%s\n' "${z_new_images}" > "${ZRBFC0_SCRATCH_FILE}" \
            || buc_die_now "Failed to write new images to ${ZRBFC0_SCRATCH_FILE}"
          local z_repo=""
          local z_tag=""
          local z_size=""
          local z_id=""
          while IFS=$'\t' read -r z_repo z_tag z_size z_id || test -n "${z_repo}"; do
            echo "    ${z_id}  ${z_repo}:${z_tag}  ${z_size}" || buc_die_now
          done < "${ZRBFC0_SCRATCH_FILE}" || buc_die_now "Failed to read new images from ${ZRBFC0_SCRATCH_FILE}"
        else
          echo "  No new images (cache unchanged)" || buc_die_now
        fi
      fi
    fi
  fi

  # SBOM — present for both bind and conjure (if syft was available)
  echo "" || buc_die_now
  echo "  -- SBOM Summary (syft) ------------------------------------------" || buc_die_now
  echo "  Software bill of materials: every package syft found installed." || buc_die_now
  echo "" || buc_die_now
  if test -f "${z_sbom}"; then
    local z_pkg_count=""
    local -r z_pkg_count_stderr="${ZRBFC0_PLUMB_PREFIX}_pkg_count_stderr.txt"
    jq '.artifacts | length' "${z_sbom}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_pkg_count_stderr}" \
      || echo "?" > "${ZRBFC0_SCRATCH_FILE}" \
      || buc_die_now "Failed to write fallback package count to ${ZRBFC0_SCRATCH_FILE}"
    local z_pkg_count_status=0
    z_pkg_count=$(<"${ZRBFC0_SCRATCH_FILE}") || z_pkg_count_status=$?
    test "${z_pkg_count_status}" -eq 0 || z_pkg_count=""
    echo "  Package count:  ${z_pkg_count}" || buc_die_now

    echo "  Package types:" || buc_die_now
    local -r z_pkg_types_stderr="${ZRBFC0_PLUMB_PREFIX}_pkg_types_stderr.txt"
    jq -r '
      [.artifacts[]?.type // empty] | group_by(.) |
      map({type: .[0], count: length}) |
      sort_by(-.count)[] |
      "    \(.count)\t\(.type)"
    ' "${z_sbom}" 2>"${z_pkg_types_stderr}" \
      || echo "    (unable to parse)" \
      || buc_die_now

    echo "" || buc_die_now
    echo "  Syft inventories installed packages. This is not a security" || buc_die_now
    echo "  assessment, vulnerability scan, or license audit." || buc_die_now
  else
    echo "  sbom.json not found in -about artifact" || buc_die_now
  fi

  # Vouch — branched by vessel mode
  echo "" || buc_die_now
  echo "  -- Vouch Results ------------------------------------------------" || buc_die_now
  if test "${z_vessel_mode}" = "rbnve_bind"; then
    echo "  Bind verification: was the mirrored image verified against its digest pin?" || buc_die_now
    echo "" || buc_die_now
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_vf_method="" z_vf_result="" z_vf_pin="" z_vf_gar=""
      local z_vf_match="" z_vf_ts="" z_vf_source=""
      local -r z_vf_stderr="${ZRBFC0_PLUMB_PREFIX}_bind_vouch_stderr.txt"
      jq -r '
        (.verification.method // "?"),
        (.verification.result // .verification.verdict // "?"),
        (.verification.pin_digest // .verification.pinned_digest // "?"),
        (.verification.gar_digest // .verification.actual_digest // "?"),
        (.verification.digest_match // "?"),
        (.verification.timestamp // "?"),
        (.verification.source_image // .verification.bind_source // "?")
      ' "${z_vs}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_vf_stderr}" \
        || buc_die_now "Failed to extract bind verification from ${z_vs} — see ${z_vf_stderr}"
      { read -r z_vf_method  || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_result  || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_pin     || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_gar     || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_match   || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_ts      || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_vf_source  || buc_die_now "Failed to read bind verification from ${ZRBFC0_SCRATCH_FILE}"
      } < "${ZRBFC0_SCRATCH_FILE}"
      echo "  Method:      ${z_vf_method}" || buc_die_now
      echo "  Verdict:     ${z_vf_result}" || buc_die_now
      echo "  Pin digest:  ${z_vf_pin}" || buc_die_now
      echo "  GAR digest:  ${z_vf_gar}" || buc_die_now
      echo "  Match:       ${z_vf_match}" || buc_die_now
      echo "  Timestamp:   ${z_vf_ts}" || buc_die_now
      echo "  Source:      ${z_vf_source}" || buc_die_now
    else
      echo "  Vouch artifact not found in GAR" || buc_die_now
    fi
  elif test "${z_vessel_mode}" = "rbnve_graft"; then
    echo "  Graft acknowledgment: no provenance chain — GRAFTED verdict" || buc_die_now
    echo "" || buc_die_now
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_gf_verdict="" z_gf_source="" z_gf_method=""
      local -r z_gf_stderr="${ZRBFC0_PLUMB_PREFIX}_graft_vouch_stderr.txt"
      jq -r '
        (.verification.verdict // "?"),
        (.verification.graft_source // "?"),
        (.verification.method // "?")
      ' "${z_vs}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_gf_stderr}" \
        || buc_die_now "Failed to extract verification fields from ${z_vs} — see ${z_gf_stderr}"
      { read -r z_gf_verdict  || buc_die_now "Failed to read verification fields from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_gf_source   || buc_die_now "Failed to read verification fields from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_gf_method   || buc_die_now "Failed to read verification fields from ${ZRBFC0_SCRATCH_FILE}"
      } < "${ZRBFC0_SCRATCH_FILE}"
      echo "  Verdict:     ${z_gf_verdict}" || buc_die_now
      echo "  Method:      ${z_gf_method}" || buc_die_now
      echo "  Source:      ${z_gf_source}" || buc_die_now
    else
      echo "  Vouch artifact not found in GAR" || buc_die_now
    fi
  else
    echo "  Independent SLSA verification: did this image pass provenance checks?" || buc_die_now
    echo "" || buc_die_now
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_verifier_url="" z_verifier_sha=""
      local -r z_verifier_stderr="${ZRBFC0_PLUMB_PREFIX}_verifier_stderr.txt"
      jq -r '(.verifier.url // "?"), (.verifier.sha256 // "?")' "${z_vs}" \
        > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_verifier_stderr}" \
        || buc_die_now "Failed to extract verifier from ${z_vs} — see ${z_verifier_stderr}"
      { read -r z_verifier_url  || buc_die_now "Failed to read verifier from ${ZRBFC0_SCRATCH_FILE}"
        read -r z_verifier_sha  || buc_die_now "Failed to read verifier from ${ZRBFC0_SCRATCH_FILE}"
      } < "${ZRBFC0_SCRATCH_FILE}"
      echo "  Verifier:" || buc_die_now
      echo "    URL:    ${z_verifier_url}" || buc_die_now
      echo "    SHA256: ${z_verifier_sha}" || buc_die_now
      echo "" || buc_die_now
      echo "  Per-platform verdicts:" || buc_die_now
      local -r z_verdicts_stderr="${ZRBFC0_PLUMB_PREFIX}_verdicts_stderr.txt"
      jq -r '.platforms[]? | "    \(.platform): \(.verdict)"' "${z_vs}" 2>"${z_verdicts_stderr}" \
        || echo "    (unable to parse)" \
        || buc_die_now
    else
      echo "  Vouch artifact not found in GAR" || buc_die_now
    fi
  fi
}

# Internal: display compact vessel info (conjure or bind)
# Args: vessel hallmark extract_dir has_vouch
zrbfcp_plumb_show_compact() {
  zrbfc0_sentinel

  local -r z_vessel="${1:-}"
  local -r z_hallmark="${2:-}"
  local -r z_dir="${3:-}"
  local -r z_has_vouch="${4:-}"
  test -n "${z_vessel}" || buc_die_now "zrbfcp_plumb_show_compact: vessel required"
  test -n "${z_hallmark}" || buc_die_now "zrbfcp_plumb_show_compact: hallmark required"
  test -n "${z_dir}" || buc_die_now "zrbfcp_plumb_show_compact: dir required"

  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "  HALLMARK PLUMB: ${z_vessel} / ${z_hallmark}" || buc_die_now
  echo "================================================================" || buc_die_now

  zrbfcp_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "" || buc_die_now
}

# Internal: display full vessel info (conjure or bind)
# Adds per-package inventory and Dockerfile (conjure only) to the compact sections.
# Args: vessel hallmark extract_dir has_vouch
zrbfcp_plumb_show_full() {
  zrbfc0_sentinel

  local -r z_vessel="${1:-}"
  local -r z_hallmark="${2:-}"
  local -r z_dir="${3:-}"
  local -r z_has_vouch="${4:-}"
  test -n "${z_vessel}" || buc_die_now "zrbfcp_plumb_show_full: vessel required"
  test -n "${z_hallmark}" || buc_die_now "zrbfcp_plumb_show_full: hallmark required"
  test -n "${z_dir}" || buc_die_now "zrbfcp_plumb_show_full: dir required"

  local -r z_sbom="${z_dir}/sbom.json"

  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "  HALLMARK PLUMB (FULL): ${z_vessel} / ${z_hallmark}" || buc_die_now
  echo "================================================================" || buc_die_now

  zrbfcp_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo "" || buc_die_now
  echo "  -- Package Inventory --------------------------------------------" || buc_die_now
  echo "  Every package syft detected, sorted by ecosystem type." || buc_die_now
  echo "" || buc_die_now
  if test -f "${z_sbom}"; then
    printf "    %-12s %-36s %s\n" "TYPE" "NAME" "VERSION" || buc_die_now
    printf "    %-12s %-36s %s\n" "----" "----" "-------" || buc_die_now
    local -r z_inv_stderr="${ZRBFC0_PLUMB_PREFIX}_inventory_stderr.txt"
    local -r z_inv_sorted="${ZRBFC0_PLUMB_PREFIX}_inventory_sorted.txt"
    local -r z_inv_sort_stderr="${ZRBFC0_PLUMB_PREFIX}_inventory_sort_stderr.txt"
    jq -r '
      .artifacts[]? |
      [.type // "?", .name // "?", .version // "?"] |
      @tsv
    ' "${z_sbom}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_inv_stderr}" \
      || buc_die_now "Failed to extract package inventory from ${z_sbom} — see ${z_inv_stderr}"
    sort "${ZRBFC0_SCRATCH_FILE}" > "${z_inv_sorted}" 2>"${z_inv_sort_stderr}" \
      || buc_die_now "Failed to sort package inventory — see ${z_inv_sort_stderr}"
    local z_type=""
    local z_name=""
    local z_ver=""
    while IFS=$'\t' read -r z_type z_name z_ver || test -n "${z_type}"; do
      printf "    %-12s %-36s %s\n" "${z_type}" "${z_name}" "${z_ver}" || buc_die_now
    done < "${z_inv_sorted}" || buc_die_now "Failed to read package inventory from ${z_inv_sorted}"
  else
    echo "    sbom.json not found in -about artifact" || buc_die_now
  fi

  echo "" || buc_die_now
  echo "  -- Package Licensing & Identity ---------------------------------" || buc_die_now
  echo "  License and Package URL for each package (for compliance review)." || buc_die_now
  echo "" || buc_die_now
  if test -f "${z_sbom}"; then
    printf "    %-36s %-20s %s\n" "NAME" "LICENSE" "PURL" || buc_die_now
    printf "    %-36s %-20s %s\n" "----" "-------" "----" || buc_die_now
    local -r z_lic_stderr="${ZRBFC0_PLUMB_PREFIX}_licensing_stderr.txt"
    local -r z_lic_sorted="${ZRBFC0_PLUMB_PREFIX}_licensing_sorted.txt"
    local -r z_lic_sort_stderr="${ZRBFC0_PLUMB_PREFIX}_licensing_sort_stderr.txt"
    jq -r '
      .artifacts[]? |
      [
        (.name // "?"),
        ((.licenses // []) | map(.value // .expression // empty) | join(", ") | if . == "" then "-" else . end),
        (.purl // "-")
      ] |
      @tsv
    ' "${z_sbom}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_lic_stderr}" \
      || buc_die_now "Failed to extract package licensing from ${z_sbom} — see ${z_lic_stderr}"
    sort "${ZRBFC0_SCRATCH_FILE}" > "${z_lic_sorted}" 2>"${z_lic_sort_stderr}" \
      || buc_die_now "Failed to sort package licensing — see ${z_lic_sort_stderr}"
    local z_lic_name=""
    local z_lic=""
    local z_purl=""
    while IFS=$'\t' read -r z_lic_name z_lic z_purl || test -n "${z_lic_name}"; do
      printf "    %-36s %-20s %s\n" "${z_lic_name}" "${z_lic}" "${z_purl}" || buc_die_now
    done < "${z_lic_sorted}" || buc_die_now "Failed to read package licensing from ${z_lic_sorted}"
  else
    echo "    sbom.json not found in -about artifact" || buc_die_now
  fi

  local -r z_recipe="${z_dir}/recipe.txt"
  if test -f "${z_recipe}"; then
    echo "" || buc_die_now
    echo "  -- Recipe (Dockerfile) ------------------------------------------" || buc_die_now
    echo "  The exact Dockerfile used to build this image." || buc_die_now
    echo "" || buc_die_now
    local z_line=""
    while IFS= read -r z_line || test -n "${z_line}"; do
      echo "    ${z_line}" || buc_die_now
    done < "${z_recipe}" || buc_die_now "Failed to read recipe ${z_recipe}"
  fi

  echo "" || buc_die_now
  echo "================================================================" || buc_die_now
  echo "" || buc_die_now
}

rbfcp_plumb_full() {
  zrbfc0_sentinel

  local -r z_express="${BUZ_FOLIO:-}"

  buc_doc_brief "Plumb a hallmark's trust posture (full detail)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530); optional — absent, falls back to the hallmark the prior build chained forward"
  buc_doc_shown || return 0

  zrbfcp_plumb_core "${z_express}" "full"
}

rbfcp_plumb_compact() {
  zrbfc0_sentinel

  local -r z_express="${BUZ_FOLIO:-}"

  buc_doc_brief "Plumb a hallmark's trust posture (compact summary)"
  buc_doc_param "hallmark" "Full hallmark (e.g., c260305133650-r260305160530); optional — absent, falls back to the hallmark the prior build chained forward"
  buc_doc_shown || return 0

  zrbfcp_plumb_core "${z_express}" "compact"
}

# eof
