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
# Recipe Bottle Foundry - GCB image creation and GAR deletion

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBF_SOURCED:-}" || buc_die "Module rbf multiply sourced - check sourcing hierarchy"
ZRBF_SOURCED=1

######################################################################
# Internal Functions (zrbf_*)

zrbf_kindle() {
  test -z "${ZRBF_KINDLED:-}" || buc_die "Module rbf already kindled"

  # Validate environment
  zburd_sentinel

  buc_log_args 'Check required GCB/GAR environment variables'
  zrbgc_sentinel

  buc_log_args 'Verify service account files'
  test -n "${RBDC_DIRECTOR_RBRA_FILE:-}" || buc_die "RBDC_DIRECTOR_RBRA_FILE not set"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBDC_DIRECTOR_RBRA_FILE}"

  buc_log_args 'Module Variables (ZRBF_*)'
  readonly ZRBF_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  readonly ZRBF_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  readonly ZRBF_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  readonly ZRBF_GCB_PROJECT_BUILDS_URL="${ZRBF_GCB_API_BASE}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  readonly ZRBF_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  buc_log_args 'Trigger dispatch endpoints'
  readonly ZRBF_TRIGGERS_URL="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBGD_GCB_REGION}/triggers"

  buc_log_args 'Registry API endpoints for delete'
  readonly ZRBF_REGISTRY_HOST="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  readonly ZRBF_REGISTRY_PATH="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  readonly ZRBF_REGISTRY_API_BASE="https://${ZRBF_REGISTRY_HOST}/v2/${ZRBF_REGISTRY_PATH}"

  buc_log_args 'Media types for delete operation'
  readonly ZRBF_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  buc_log_args 'RBGJ files in same Tools directory as this implementation'
  # Acronym: rbgjb = Recipe Bottle Google Json Build (step scripts in rbgjb/ dir)
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBF_RBGJB_STEPS_DIR="${z_self_dir}/rbgjb"
  test -d "${ZRBF_RBGJB_STEPS_DIR}"   || buc_die "RBGJB steps directory not found: ${ZRBF_RBGJB_STEPS_DIR}"

  buc_log_args 'RBGJV vouch step scripts (same Tools directory)'
  # Acronym: rbgjv = Recipe Bottle Google Json Vouch (step scripts in rbgjv/ dir)
  readonly ZRBF_RBGJV_STEPS_DIR="${z_self_dir}/rbgjv"
  test -d "${ZRBF_RBGJV_STEPS_DIR}"   || buc_die "RBGJV steps directory not found: ${ZRBF_RBGJV_STEPS_DIR}"

  buc_log_args 'RBGJA about step scripts (same Tools directory)'
  # Acronym: rbgja = Recipe Bottle Google Json About (step scripts in rbgja/ dir)
  readonly ZRBF_RBGJA_STEPS_DIR="${z_self_dir}/rbgja"
  test -d "${ZRBF_RBGJA_STEPS_DIR}"   || buc_die "RBGJA steps directory not found: ${ZRBF_RBGJA_STEPS_DIR}"

  buc_log_args 'RBGJM mirror step scripts (same Tools directory)'
  # Acronym: rbgjm = Recipe Bottle Google Json Mirror (step scripts in rbgjm/ dir)
  readonly ZRBF_RBGJM_STEPS_DIR="${z_self_dir}/rbgjm"
  test -d "${ZRBF_RBGJM_STEPS_DIR}"   || buc_die "RBGJM steps directory not found: ${ZRBF_RBGJM_STEPS_DIR}"

  buc_log_args 'RBGJE enshrine step scripts (same Tools directory)'
  # Acronym: rbgje = Recipe Bottle Google Json Enshrine (step scripts in rbgje/ dir)
  readonly ZRBF_RBGJE_STEPS_DIR="${z_self_dir}/rbgje"
  test -d "${ZRBF_RBGJE_STEPS_DIR}"   || buc_die "RBGJE steps directory not found: ${ZRBF_RBGJE_STEPS_DIR}"

  buc_log_args 'RBGJI inscribe step scripts (same Tools directory)'
  # Acronym: rbgji = Recipe Bottle Google Json Inscribe (step scripts in rbgji/ dir)
  readonly ZRBF_RBGJI_STEPS_DIR="${z_self_dir}/rbgji"
  test -d "${ZRBF_RBGJI_STEPS_DIR}"   || buc_die "RBGJI steps directory not found: ${ZRBF_RBGJI_STEPS_DIR}"

  buc_log_args 'Define temp files for build operations'
  readonly ZRBF_BUILD_ID_FILE="${BURD_TEMP_DIR}/rbf_build_id.txt"
  readonly ZRBF_BUILD_STATUS_FILE="${BURD_TEMP_DIR}/rbf_build_status.json"
  readonly ZRBF_BUILD_RUBRIC_LS="${BURD_TEMP_DIR}/rbf_rubric_ls_remote.txt"
  readonly ZRBF_BUILD_TRIGGER_BODY="${BURD_TEMP_DIR}/rbf_trigger_run_body.json"

  buc_log_args 'Define git info file (used by inscribe -> stitch)'
  readonly ZRBF_GIT_INFO_FILE="${BURD_TEMP_DIR}/rbf_git_info.json"

  buc_log_args 'Define git metadata files (shared across about/mirror submissions)'
  readonly ZRBF_GIT_PREFIX="${BURD_TEMP_DIR}/rbf_git_"
  readonly ZRBF_GIT_COMMIT_FILE="${ZRBF_GIT_PREFIX}commit.txt"
  readonly ZRBF_GIT_BRANCH_FILE="${ZRBF_GIT_PREFIX}branch.txt"
  readonly ZRBF_GIT_REPO_FILE="${ZRBF_GIT_PREFIX}repo.txt"

  buc_log_args 'Define validation files'
  readonly ZRBF_STATUS_CHECK_FILE="${BURD_TEMP_DIR}/rbf_status_check.txt"

  buc_log_args 'Define delete operation files'
  readonly ZRBF_DELETE_PREFIX="${BURD_TEMP_DIR}/rbf_delete_"
  readonly ZRBF_TOKEN_FILE="${BURD_TEMP_DIR}/rbf_token.txt"

  buc_log_args 'Define vouch operation file prefix (postfixed per step id)'
  readonly ZRBF_VOUCH_PREFIX="${BURD_TEMP_DIR}/rbf_vouch_"

  buc_log_args 'Define about operation file prefix (postfixed per step id)'
  readonly ZRBF_ABOUT_PREFIX="${BURD_TEMP_DIR}/rbf_about_"

  buc_log_args 'Vessel-related files'
  readonly ZRBF_VESSEL_SIGIL_FILE="${BURD_TEMP_DIR}/rbf_vessel_sigil.txt"
  readonly ZRBF_VESSEL_RESOLVED_DIR_FILE="${BURD_TEMP_DIR}/rbf_vessel_resolved_dir.txt"

  buc_log_args 'Define stitch operation file prefix (postfixed per step id)'
  readonly ZRBF_STITCH_PREFIX="${BURD_TEMP_DIR}/rbf_stitch_"

  buc_log_args 'Define inscribe operation files'
  readonly ZRBF_INSCRIBE_PREFIX="${BURD_TEMP_DIR}/rbf_inscribe_"
  readonly ZRBF_INSCRIBE_CLONE_DIR="${RBGC_RUBRIC_CLONE_DIR}"
  readonly ZRBF_INSCRIBE_STALENESS_SEC=86400

  buc_log_args 'Define mirror operation files'
  readonly ZRBF_MIRROR_PREFIX="${BURD_TEMP_DIR}/rbf_mirror_"

  buc_log_args 'Define graft operation files'
  readonly ZRBF_GRAFT_PREFIX="${BURD_TEMP_DIR}/rbf_graft_"

  buc_log_args 'Define enshrine operation files'
  readonly ZRBF_ENSHRINE_PREFIX="${BURD_TEMP_DIR}/rbf_enshrine_"

  buc_log_args 'Define enshrine preflight files'
  readonly ZRBF_PREFLIGHT_PREFIX="${BURD_TEMP_DIR}/rbf_preflight_"

  buc_log_args 'Define reliquary inscribe operation files'
  readonly ZRBF_RELIQUARY_PREFIX="${BURD_TEMP_DIR}/rbf_reliquary_"

  buc_log_args 'Define context push operation files'
  readonly ZRBF_CONTEXT_PREFIX="${BURD_TEMP_DIR}/rbf_context_"

  buc_log_args 'Define output files (BURD_OUTPUT_DIR — persists after dispatch)'
  readonly ZRBF_OUTPUT_VESSEL_DIR="${BURD_OUTPUT_DIR}/rbf_vessel_dir.txt"

  buc_log_args 'Scratch file for sequential temp-file patterns'
  readonly ZRBF_SCRATCH_FILE="${BURD_TEMP_DIR}/rbf_scratch.txt"

  readonly ZRBF_KINDLED=1
}

zrbf_sentinel() {
  test "${ZRBF_KINDLED:-}" = "1" || buc_die "Module rbf not kindled - call zrbf_kindle first"
}

# Capture: decode base64 value (secret — never temp file)
# Args: base64_encoded_value
zrbf_base64_decode_capture() {
  printf '%s' "$1" | base64 -d
}

# Check concurrent build quota against regime requirements
# Args: token mode
#   mode: "gate" (die if insufficient) or "advisory" (warn if insufficient)
zrbf_quota_preflight() {
  zrbf_sentinel

  local -r z_token="${1:-}"

  test -n "${z_token}" || buc_die "zrbf_quota_preflight: token required"

  # Extract vCPU count from machine type (last segment after final hyphen)
  local -r z_vcpus="${RBRR_GCB_MACHINE_TYPE##*-}"
  case "${z_vcpus}" in
    ""|0|*[!0-9]*)
      buc_warn "Cannot parse vCPU count from RBRR_GCB_MACHINE_TYPE='${RBRR_GCB_MACHINE_TYPE}' -- skipping quota preflight"
      return 0
      ;;
  esac

  buc_log_args "Machine type ${RBRR_GCB_MACHINE_TYPE} = ${z_vcpus} vCPUs"

  # Query Service Usage consumer quota API for concurrent_private_pool_build_cpus
  local -r z_metric_encoded="cloudbuild.googleapis.com%2Fconcurrent_private_pool_build_cpus"
  local -r z_url="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1BETA1}/projects/${RBRR_DEPOT_PROJECT_ID}/services/cloudbuild.googleapis.com/consumerQuotaMetrics/${z_metric_encoded}"

  buc_step "Checking concurrent build quota"
  rbgu_http_json "GET" "${z_url}" "${z_token}" "quota_preflight"

  local z_code=""
  z_code=$(rbgu_http_code_capture "quota_preflight") || z_code=""
  if test "${z_code}" != "200"; then
    buc_warn "Could not query build quota (HTTP ${z_code}) -- skipping preflight check"
    return 0
  fi

  # Filter quota response to region-specific bucket via intermediate file
  rbgu_jq_file_to_file_ok "quota_preflight" "quota_region" \
    "[.consumerQuotaLimits[0].quotaBuckets[] | select(.dimensions.region == \"${RBRR_GCP_REGION}\")] | .[0] // {}" \
    || true

  # Extract effective limit from region bucket, then fallback to first bucket
  local z_limit=""
  z_limit=$(rbgu_json_field_capture "quota_region" '.effectiveLimit') || z_limit=""
  if test -z "${z_limit}"; then
    z_limit=$(rbgu_json_field_capture "quota_preflight" \
      '.consumerQuotaLimits[0].quotaBuckets[0].effectiveLimit') || z_limit=""
  fi

  if test -z "${z_limit}"; then
    buc_warn "Could not extract quota limit -- skipping preflight check"
    return 0
  fi

  # -1 means unlimited
  if test "${z_limit}" = "-1"; then
    buc_info "Quota: unlimited concurrent private pool build CPUs"
    return 0
  fi

  # Compute max concurrent builds
  local -r z_max_concurrent=$((z_limit / z_vcpus))

  buc_log_args "Quota ${z_limit} vCPUs, machine ${z_vcpus} vCPUs, max concurrent ${z_max_concurrent}, required ${RBRR_GCB_MIN_CONCURRENT_BUILDS}"

  if test "${z_max_concurrent}" -lt "${RBRR_GCB_MIN_CONCURRENT_BUILDS}"; then
    buc_warn "Build quota insufficient: ${z_limit} vCPU quota / ${z_vcpus} vCPUs per build = ${z_max_concurrent} concurrent (need ${RBRR_GCB_MIN_CONCURRENT_BUILDS})"
    buc_warn "Fresh depots start with a low quota. After some build activity, the Edit Quotas option becomes available."
    buc_tabtarget "${RBZ_QUOTA_BUILD}"
  else
    buc_info "Quota OK: ${z_limit} vCPU / ${z_vcpus} per build = ${z_max_concurrent} concurrent (need ${RBRR_GCB_MIN_CONCURRENT_BUILDS})"
  fi
}

# Internal: Verify that all required images exist in GAR before submitting
# to Cloud Build. Checks two layers in dependency order:
#
#   1. Reliquary — co-versioned builder tool images (gcloud, docker, syft, etc.)
#      created by inscribe. One reliquary per depot setup. Enshrine depends on it.
#   2. Enshrine — upstream base images copied into private GAR, pinned by content
#      hash. One enshrine per base image anchor. Multiple vessels sharing the same
#      anchor only need one enshrine.
#
# A missing image at either layer causes a Cloud Build failure minutes later.
# This preflight catches it immediately with copy-paste remediation commands.
#
# Must be called after vessel load (reads RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR)
# and authentication (needs token for registry API).
zrbf_registry_preflight() {
  zrbf_sentinel

  local -r z_token="${1:-}"
  local -r z_vessel_dir="${2:-}"
  test -n "${z_token}"      || buc_die "zrbf_registry_preflight: token required"
  test -n "${z_vessel_dir}" || buc_die "zrbf_registry_preflight: vessel_dir required"

  # --- Layer 1: Reliquary tool images ---
  # Inscribe creates all 6 tool images atomically in one GCB job, so checking
  # one (docker:latest) is sufficient as a canary for the entire reliquary.

  local -r z_reliquary="${RBRV_RELIQUARY:-}"
  if test -n "${z_reliquary}"; then
    buc_step "Verifying reliquary tool images exist in GAR"

    local -r z_rqy_canary="${z_reliquary}/docker"
    local -r z_rqy_status_file="${ZRBF_PREFLIGHT_PREFIX}reliquary_status.txt"
    local -r z_rqy_response_file="${ZRBF_PREFLIGHT_PREFIX}reliquary_response.txt"
    local -r z_rqy_stderr_file="${ZRBF_PREFLIGHT_PREFIX}reliquary_stderr.txt"

    curl --head -sS \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
      -w "%{http_code}" \
      -o "${z_rqy_response_file}" \
      "${ZRBF_REGISTRY_API_BASE}/${z_rqy_canary}/manifests/latest" \
      > "${z_rqy_status_file}" 2>"${z_rqy_stderr_file}" \
      || buc_die "HEAD request failed for reliquary canary: ${z_rqy_canary}:latest — see ${z_rqy_stderr_file}"

    local z_rqy_http_code=""
    z_rqy_http_code=$(<"${z_rqy_status_file}")
    test -n "${z_rqy_http_code}" || buc_die "HTTP status code is empty for reliquary check"

    if test "${z_rqy_http_code}" = "404"; then
      buc_warn "Reliquary not found: ${z_reliquary}"
      buc_bare "  The reliquary is a co-versioned set of builder tool images (gcloud, docker,"
      buc_bare "  syft, alpine, binfmt, skopeo) inscribed from upstream into your private GAR."
      buc_bare "  Air-gapped worker pools cannot pull from the public internet — the reliquary"
      buc_bare "  stages these tools so builds can run without egress. All vessels in a depot"
      buc_bare "  typically share one reliquary. Inscribe creates a new datestamped set:"
      buc_tabtarget "${RBZ_INSCRIBE_RELIQUARY}"
      buc_tabtarget "${RBZ_ORDAIN_CONSECRATION}" "${z_vessel_dir}"
      buc_die "Registry preflight failed — reliquary missing from GAR"
    elif test "${z_rqy_http_code}" != "200"; then
      buc_die "Unexpected HTTP ${z_rqy_http_code} when checking reliquary: ${z_rqy_canary}:latest"
    fi

    buc_info "Reliquary verified: ${z_reliquary}"
  fi

  # --- Layer 2: Enshrined base images ---
  # Each vessel declares base images via RBRV_IMAGE_n_ORIGIN (upstream tag) and
  # RBRV_IMAGE_n_ANCHOR (content-addressed GAR tag). Enshrine copies the upstream
  # image into the enshrine namespace, pinned by the anchor hash. The conjure
  # Dockerfile's FROM references the anchor, not the upstream tag.

  buc_step "Verifying enshrined base images exist in GAR"

  local z_n=""
  local z_anchor_var=""
  local z_anchor=""
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

    # Skip slots without an origin or without an anchor (pass-through images don't need enshrine)
    test -n "${z_origin}" || continue
    test -n "${z_anchor}" || continue

    z_any_checked="true"
    z_status_file="${ZRBF_PREFLIGHT_PREFIX}enshrine_${z_n}_status.txt"
    z_response_file="${ZRBF_PREFLIGHT_PREFIX}enshrine_${z_n}_response.txt"
    z_stderr_file="${ZRBF_PREFLIGHT_PREFIX}enshrine_${z_n}_stderr.txt"

    curl --head -sS \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
      -w "%{http_code}" \
      -o "${z_response_file}" \
      "${ZRBF_REGISTRY_API_BASE}/enshrine/manifests/${z_anchor}" \
      > "${z_status_file}" 2>"${z_stderr_file}" \
      || buc_die "HEAD request failed for enshrined image: enshrine:${z_anchor} — see ${z_stderr_file}"

    z_http_code=$(<"${z_status_file}")
    test -n "${z_http_code}" || buc_die "HTTP status code is empty for enshrine check"

    if test "${z_http_code}" = "404"; then
      buc_warn "Enshrined base image not found: enshrine:${z_anchor} (from ${z_origin})"
      buc_bare "  Enshrine copies upstream base images into your private GAR, pinned by content"
      buc_bare "  hash. Multiple vessels sharing the same base image anchor need only one enshrine."
      buc_bare "  Run enshrine, then re-run ordain:"
      buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir}"
      buc_tabtarget "${RBZ_ORDAIN_CONSECRATION}" "${z_vessel_dir}"
      buc_die "Registry preflight failed — enshrined base image missing from GAR"
    elif test "${z_http_code}" != "200"; then
      buc_die "Unexpected HTTP ${z_http_code} when checking enshrined image: enshrine:${z_anchor}"
    fi

    buc_log_args "Enshrined image verified: enshrine:${z_anchor}"
  done

  if test "${z_any_checked}" = "true"; then
    buc_info "All enshrined base images verified in GAR"
  fi
}

# Internal: Resolve tool image references from reliquary.
# Must be called after vessel load (reads RBRV_RELIQUARY).
# Sets module-level ZRBF_TOOL_* variables for downstream step assembly.
# Idempotent — safe to call multiple times per invocation.
zrbf_resolve_tool_images() {
  zrbf_sentinel

  local -r z_reliquary="${RBRV_RELIQUARY:-}"
  test -n "${z_reliquary}" \
    || buc_die "RBRV_RELIQUARY is required — run inscribe to create a reliquary first"

  local -r z_rqy_prefix="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_reliquary}"
  ZRBF_TOOL_GCLOUD="${z_rqy_prefix}/gcloud:latest"
  ZRBF_TOOL_DOCKER="${z_rqy_prefix}/docker:latest"
  ZRBF_TOOL_ALPINE="${z_rqy_prefix}/alpine:latest"
  ZRBF_TOOL_SYFT="${z_rqy_prefix}/syft:latest"
  ZRBF_TOOL_BINFMT="${z_rqy_prefix}/binfmt:latest"
  ZRBF_TOOL_SKOPEO="${z_rqy_prefix}/skopeo:latest"
  buc_log_args "Tool images resolved from reliquary: ${z_reliquary}"
}

zrbf_stitch_build_json() {
  zrbf_sentinel

  local -r z_output_path="${1:?Output path required}"
  local -r z_inscribe_ts="${2:?Inscribe timestamp required}"
  local -r z_context_tag="${3:?Context image tag required}"

  buc_log_args "Stitching builds.create JSON to ${z_output_path}"

  # Preconditions: vessel loaded and git state captured
  test -s "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Vessel not loaded — call zrbf_load_vessel first"
  test -s "${ZRBF_GIT_INFO_FILE}"     || buc_die "Git info not captured — ensure git metadata is captured before stitch"

  buc_log_args 'Read vessel state for substitutions'
  local -r z_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}")
  test -n "${z_sigil}" || buc_die "Empty vessel sigil"
  local -r z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"
  local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"

  # Resolve base images: ANCHOR → full GAR reference, or pass ORIGIN through
  # Spec: RBSAC step "Resolve Base Images"
  local -r z_gar_image_prefix="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/enshrine"
  local z_image_ref_1="" z_image_ref_2="" z_image_ref_3=""
  local z_ri_n="" z_ri_origin_var="" z_ri_anchor_var="" z_ri_origin="" z_ri_anchor=""
  for z_ri_n in 1 2 3; do
    z_ri_origin_var="RBRV_IMAGE_${z_ri_n}_ORIGIN"
    z_ri_anchor_var="RBRV_IMAGE_${z_ri_n}_ANCHOR"
    z_ri_origin="${!z_ri_origin_var:-}"
    z_ri_anchor="${!z_ri_anchor_var:-}"
    test -n "${z_ri_origin}" || continue
    local z_ri_ref=""
    if test -n "${z_ri_anchor}"; then
      z_ri_ref="${z_gar_image_prefix}:${z_ri_anchor}"
      buc_log_args "Image slot ${z_ri_n} (anchored): ${z_ri_ref}"
    else
      z_ri_ref="${z_ri_origin}"
      buc_log_args "Image slot ${z_ri_n} (pass-through): ${z_ri_ref}"
    fi
    case "${z_ri_n}" in
      1) z_image_ref_1="${z_ri_ref}" ;; 2) z_image_ref_2="${z_ri_ref}" ;; 3) z_image_ref_3="${z_ri_ref}" ;;
    esac
  done

  # Platform count detection
  local z_platform_count=0
  local z_remaining_count="${z_platforms}"
  local z_p_count=""
  while test -n "${z_remaining_count}"; do
    z_p_count="${z_remaining_count%%,*}"
    z_platform_count=$((z_platform_count + 1))
    test "${z_remaining_count}" != "${z_p_count}" || break
    z_remaining_count="${z_remaining_count#*,}"
  done
  buc_log_args "Vessel platforms: ${z_platform_count} (${z_platforms})"

  buc_log_args 'Extract git state for substitutions'
  local -r z_stitch_git_commit_file="${ZRBF_STITCH_PREFIX}git_commit.txt"
  local -r z_stitch_git_branch_file="${ZRBF_STITCH_PREFIX}git_branch.txt"
  local -r z_stitch_git_repo_file="${ZRBF_STITCH_PREFIX}git_repo.txt"

  jq -r '.commit' "${ZRBF_GIT_INFO_FILE}" > "${z_stitch_git_commit_file}" \
    || buc_die "Failed to extract git commit from info file"
  jq -r '.branch' "${ZRBF_GIT_INFO_FILE}" > "${z_stitch_git_branch_file}" \
    || buc_die "Failed to extract git branch from info file"
  jq -r '.repo'   "${ZRBF_GIT_INFO_FILE}" > "${z_stitch_git_repo_file}" \
    || buc_die "Failed to extract git repo from info file"

  local -r z_git_commit=$(<"${z_stitch_git_commit_file}")
  local -r z_git_branch=$(<"${z_stitch_git_branch_file}")
  local -r z_git_repo=$(<"${z_stitch_git_repo_file}")

  test -n "${z_git_commit}" || buc_die "Git commit is empty"
  test -n "${z_git_branch}" || buc_die "Git branch is empty"
  test -n "${z_git_repo}"   || buc_die "Git repo is empty"

  # Build strategy: compare vessel platforms against runner platform
  # If platforms exactly match the runner, no QEMU emulation is needed (native build).
  # Any difference (multi-platform or non-native single-platform) requires binfmt.
  local z_needs_binfmt="true"
  if test "${RBRV_CONJURE_PLATFORMS// /,}" = "${RBGC_BUILD_RUNNER_PLATFORM}"; then
    z_needs_binfmt="false"
  fi

  local z_build_strategy=""
  if test "${z_needs_binfmt}" = "true"; then
    z_build_strategy="emulated multi-platform via QEMU (${RBRV_CONJURE_PLATFORMS// /,})"
    buc_log_args "Build strategy: ${z_build_strategy} — rbgjb02 included"
  else
    z_build_strategy="native single-platform (${RBGC_BUILD_RUNNER_PLATFORM})"
    buc_log_args "Build strategy: ${z_build_strategy} — rbgjb02 excluded"
  fi

  # Step definitions: script|builder|entrypoint|id
  # Entrypoint 'bash' → #!/bin/bash shebang, 'sh' → #!/bin/sh shebang (GCB script field)
  # Delimiter is | because image refs contain colons (sha256 digests)
  # Pipeline: buildx --push → per-platform pullback → SLSA provenance via images: field
  local z_step_defs=(
    "rbgjb01-derive-tag-base.sh|${ZRBF_TOOL_GCLOUD}|bash|derive-tag-base"
  )
  if test "${z_needs_binfmt}" = "true"; then
    z_step_defs+=("rbgjb02-qemu-binfmt.sh|${ZRBF_TOOL_DOCKER}|bash|qemu-binfmt")
  fi
  z_step_defs+=(
    "rbgjb03-buildx-push-multi.sh|${ZRBF_TOOL_DOCKER}|bash|buildx-push-multi"
    "rbgjb04-per-platform-pullback.sh|${ZRBF_TOOL_DOCKER}|bash|per-platform-pullback"
    "rbgjb05-push-per-platform.sh|${ZRBF_TOOL_DOCKER}|bash|push-per-platform"
    "rbgjb06-imagetools-create.sh|${ZRBF_TOOL_DOCKER}|bash|imagetools-create"
    "rbgjb07-push-diags.sh|${ZRBF_TOOL_DOCKER}|bash|push-diags"
  )

  # Compute platform suffixes (used in images: field and substitutions)
  # Always computed: linux/amd64 → -amd64, linux/arm64 → -arm64, linux/arm/v7 → -armv7
  local z_platform_suffixes=""
  local z_platform_suffixes_csv=""
  local z_remaining_plats="${z_platforms}"
  local z_plat=""
  local z_suffix=""
  while test -n "${z_remaining_plats}"; do
    z_plat="${z_remaining_plats%%,*}"
    # Strip linux/ prefix, collapse remaining slashes: linux/arm/v7 → armv7
    z_suffix="${z_plat#linux/}"
    z_suffix="${z_suffix//\//}"
    z_suffix="-${z_suffix}"
    if test -n "${z_platform_suffixes}"; then
      z_platform_suffixes="${z_platform_suffixes},${z_suffix}"
    else
      z_platform_suffixes="${z_suffix}"
    fi
    test "${z_remaining_plats}" != "${z_plat}" || break
    z_remaining_plats="${z_remaining_plats#*,}"
  done
  z_platform_suffixes_csv="${z_platform_suffixes}"
  buc_log_args "Platform suffixes: ${z_platform_suffixes_csv}"

  local z_def=""
  local z_script=""
  local z_builder=""
  local z_entrypoint=""
  local z_id=""
  local z_script_path=""
  local z_body=""
  local z_shebang=""
  local z_body_file=""
  local z_escaped_file=""
  local z_steps_file=""
  local z_accumulator_file="${ZRBF_STITCH_PREFIX}steps.json"

  buc_log_args "Initializing empty steps array"
  echo "[]" > "${z_accumulator_file}" || buc_die "Failed to initialize steps JSON"

  for z_def in "${z_step_defs[@]}"; do
    IFS='|' read -r z_script z_builder z_entrypoint z_id <<< "${z_def}"
    z_script_path="${ZRBF_RBGJB_STEPS_DIR}/${z_script}"
    z_body_file="${ZRBF_STITCH_PREFIX}${z_id}_body.txt"
    z_escaped_file="${ZRBF_STITCH_PREFIX}${z_id}_escaped.txt"
    z_steps_file="${ZRBF_STITCH_PREFIX}${z_id}_steps.json"

    test -f "${z_script_path}" || buc_die "Step script not found: ${z_script_path}"

    buc_log_args "Reading script body for ${z_id} (skip shebang, comments pass through)"
    tail -n +2 "${z_script_path}" > "${z_body_file}" || buc_die "Failed to read step script: ${z_script_path}"
    z_body=$(<"${z_body_file}")
    test -n "${z_body}" || buc_die "Empty script body: ${z_script_path}"

    buc_log_args "Baking pinned image refs and build strategy into script text"
    z_body="${z_body//\$\{ZRBF_TOOL_BINFMT\}/${ZRBF_TOOL_BINFMT}}"
    z_body="${z_body//\$\{ZRBF_BUILD_STRATEGY\}/${z_build_strategy}}"

    case "${z_entrypoint}" in
      bash) z_shebang="#!/bin/bash" ;;
      sh)   z_shebang="#!/bin/sh" ;;
      *)    buc_die "Unknown entrypoint: ${z_entrypoint}" ;;
    esac
    printf '%s\n%s' "${z_shebang}" "${z_body}" > "${z_escaped_file}" \
      || buc_die "Failed to write script body for ${z_id}"

    buc_log_args "Appending step ${z_id} to JSON array"
    jq \
      --arg name "${z_builder}" \
      --arg id "${z_id}" \
      --arg dir "${z_sigil}" \
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, dir: $dir, script: $script}]' \
      "${z_accumulator_file}" > "${z_steps_file}" \
      || buc_die "Failed to append step ${z_id} to JSON"
    mv "${z_steps_file}" "${z_accumulator_file}" \
      || buc_die "Failed to update steps JSON for ${z_id}"
  done

  # === Combined conjure: embed about steps after image steps ===
  # About steps use _RBGA_* substitutions. Most are added to the substitutions block.
  # Two require special handling (not known at inscribe time):
  #   - _RBGA_CONSECRATION: computed at build time by rbgjb01, read from workspace
  #   - _RBGA_BUILD_ID: Cloud Build job ID, available as built-in $BUILD_ID

  buc_log_args "Assembling about steps for combined conjure"
  local -r z_about_steps_file="${ZRBF_STITCH_PREFIX}about_steps.json"
  zrbf_assemble_about_steps "${z_about_steps_file}" "${ZRBF_STITCH_PREFIX}about_"

  # About steps run in vessel dir so .consecration from rbgjb01 is accessible
  buc_log_args "Adding dir field to about steps for vessel directory ${z_sigil}"
  local -r z_about_with_dir="${ZRBF_STITCH_PREFIX}about_with_dir.json"
  jq --arg dir "${z_sigil}" '[.[] | . + {dir: $dir}]' \
    "${z_about_steps_file}" > "${z_about_with_dir}" \
    || buc_die "Failed to add dir to about steps"

  # Consecration: $(cat .consecration) → bash reads workspace file written by rbgjb01
  # Build ID: $BUILD_ID → GCB built-in available as env var
  buc_log_args "Post-processing about steps: consecration from workspace, build ID from env"
  local -r z_about_processed="${ZRBF_STITCH_PREFIX}about_processed.json"
  local z_about_content
  z_about_content=$(<"${z_about_with_dir}") \
    || buc_die "Failed to read about steps for post-processing"
  z_about_content="${z_about_content//\$\{_RBGA_CONSECRATION\}/\$(cat .consecration)}"
  z_about_content="${z_about_content//\$\{_RBGA_BUILD_ID:-\}/\$BUILD_ID}"
  printf '%s' "${z_about_content}" > "${z_about_processed}" \
    || buc_die "Failed to post-process about steps for conjure"

  buc_log_args "Combining image steps and about steps"
  local -r z_combined_steps_file="${ZRBF_STITCH_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1]' "${z_accumulator_file}" "${z_about_processed}" \
    > "${z_combined_steps_file}" || buc_die "Failed to combine image and about steps"
  z_accumulator_file="${z_combined_steps_file}"

  # Fallback for -diags extraction failure; -diags is the primary path for conjure
  buc_log_args "Reading Dockerfile content for _RBGA_DOCKERFILE_CONTENT substitution"
  local z_stitch_dockerfile_content=""
  local -r z_stitch_df_max_bytes=4000
  if test -f "${RBRV_CONJURE_DOCKERFILE:-}"; then
    local -r z_stitch_df_size_file="${ZRBF_STITCH_PREFIX}df_size.txt"
    wc -c < "${RBRV_CONJURE_DOCKERFILE}" > "${z_stitch_df_size_file}" \
      || buc_die "Failed to measure Dockerfile size"
    local z_stitch_df_size=""
    z_stitch_df_size=$(<"${z_stitch_df_size_file}")
    z_stitch_df_size="${z_stitch_df_size// /}"
    if test "${z_stitch_df_size}" -le "${z_stitch_df_max_bytes}"; then
      z_stitch_dockerfile_content=$(<"${RBRV_CONJURE_DOCKERFILE}")
    else
      buc_warn "Dockerfile exceeds 4KB substitution limit (${z_stitch_df_size} bytes) — recipe.txt via -diags only"
    fi
  fi

  # Compose builds.create Build resource
  # All values resolved directly — no placeholders, no post-processing jq surgery.
  # Context extraction step prepended; mason SA included; images: field uses inscribe_ts.
  buc_log_args "Composing builds.create Build resource"
  local -r z_build_file="${ZRBF_STITCH_PREFIX}build.json"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Context extraction step (first step — extracts build context from pouch in GAR)
  local -r z_extract_step_file="${ZRBF_STITCH_PREFIX}extract_step.json"
  jq -n \
    --arg name "${ZRBF_TOOL_DOCKER}" \
    --arg ctx_tag "${z_context_tag}" \
    --arg sigil "${z_sigil}" \
    '{
      name: $name,
      id: "extract-context",
      entrypoint: "/bin/bash",
      args: ["-lc", ("set -euo pipefail\necho \"Extracting build context from GAR\"\nCONTAINER=$$(docker create " + $ctx_tag + " /nonexistent)\nmkdir -p /workspace/" + $sigil + "\ndocker cp $${CONTAINER}:/build-context/. /workspace/" + $sigil + "/\ndocker rm $${CONTAINER}\necho \"Context extracted:\"\nls -la /workspace/" + $sigil + "/")]
    }' > "${z_extract_step_file}" \
    || buc_die "Failed to compose context extraction step"

  # Combine: [extract-context] + image steps + about steps
  local -r z_all_steps_file="${ZRBF_STITCH_PREFIX}all_steps.json"
  jq -s '.[0] + .[1]' <(jq -s '.' "${z_extract_step_file}") "${z_accumulator_file}" \
    > "${z_all_steps_file}" || buc_die "Failed to prepend context extraction step"

  # images: field — one entry per platform for SLSA provenance via CB images: push
  local z_images_file="${ZRBF_STITCH_PREFIX}images.json"
  local z_image_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${z_sigil}"
  local z_remaining_suffixes="${z_platform_suffixes_csv}"
  local z_img_suffix=""
  echo "[]" > "${z_images_file}" || buc_die "Failed to initialize images JSON"
  while test -n "${z_remaining_suffixes}"; do
    z_img_suffix="${z_remaining_suffixes%%,*}"
    jq --arg uri "${z_image_base}:${z_inscribe_ts}-image${z_img_suffix}" \
      '. + [$uri]' "${z_images_file}" > "${z_images_file}.tmp" \
      || buc_die "Failed to append image URI"
    mv "${z_images_file}.tmp" "${z_images_file}" \
      || buc_die "Failed to update images JSON"
    test "${z_remaining_suffixes}" != "${z_img_suffix}" || break
    z_remaining_suffixes="${z_remaining_suffixes#*,}"
  done

  # shellcheck disable=SC2016
  local -r z_cb_build_id='$BUILD_ID'

  # Pool routing: conjure/bind use vessel's egress mode
  local z_conjure_pool=""
  case "${RBRV_EGRESS_MODE}" in
    tether) z_conjure_pool="${RBDC_POOL_TETHER}" ;;
    airgap) z_conjure_pool="${RBDC_POOL_AIRGAP}" ;;
    *) buc_die "Unknown RBRV_EGRESS_MODE: ${RBRV_EGRESS_MODE}" ;;
  esac

  jq -n \
    --slurpfile zjq_steps  "${z_all_steps_file}" \
    --slurpfile zjq_images "${z_images_file}" \
    --arg zjq_dockerfile     "${z_dockerfile_name}" \
    --arg zjq_moniker        "${z_sigil}" \
    --arg zjq_platforms      "${z_platforms}" \
    --arg zjq_platform_suffixes "${z_platform_suffixes_csv}" \
    --arg zjq_gar_location   "${RBGD_GAR_LOCATION}" \
    --arg zjq_gar_project    "${RBGD_GAR_PROJECT_ID}" \
    --arg zjq_gar_repository "${RBRR_GAR_REPOSITORY}" \
    --arg zjq_git_commit     "${z_git_commit}" \
    --arg zjq_git_branch     "${z_git_branch}" \
    --arg zjq_git_repo       "${z_git_repo}" \
    --arg zjq_gar_host_suffix  "${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_diags "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_inscribe_ts      "${z_inscribe_ts}" \
    --arg zjq_pool   "${z_conjure_pool}" \
    --arg zjq_timeout "${RBRR_GCB_TIMEOUT}" \
    --arg zjq_mason_sa         "${z_mason_sa}" \
    --arg zjq_cb_build_id      "${z_cb_build_id}" \
    --arg zjq_rbga_gar_host       "${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_rbga_gar_path       "${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}" \
    --arg zjq_rbga_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_rbga_dockerfile     "${z_stitch_dockerfile_content}" \
    --arg zjq_image_1             "${z_image_ref_1}" \
    --arg zjq_image_2             "${z_image_ref_2}" \
    --arg zjq_image_3             "${z_image_ref_3}" \
    '{
      steps: [$zjq_steps[0][] |
        if .args then
          .args = [.args[] | gsub("\\$\\{_RBGA_BUILD_ID\\}"; $zjq_cb_build_id) | gsub("\\$\\{_RBGA_BUILD_ID:-\\}"; $zjq_cb_build_id)]
        elif .script then
          .script = (.script | gsub("\\$\\{_RBGA_BUILD_ID\\}"; $zjq_cb_build_id) | gsub("\\$\\{_RBGA_BUILD_ID:-\\}"; $zjq_cb_build_id))
        else . end],
      images: $zjq_images[0],
      substitutions: {
        _RBGY_DOCKERFILE:          $zjq_dockerfile,
        _RBGY_MONIKER:             $zjq_moniker,
        _RBGY_PLATFORMS:           $zjq_platforms,
        _RBGY_PLATFORM_SUFFIXES:   $zjq_platform_suffixes,
        _RBGY_GAR_LOCATION:        $zjq_gar_location,
        _RBGY_GAR_PROJECT:         $zjq_gar_project,
        _RBGY_GAR_REPOSITORY:      $zjq_gar_repository,
        _RBGY_GIT_COMMIT:          $zjq_git_commit,
        _RBGY_GIT_BRANCH:          $zjq_git_branch,
        _RBGY_GAR_HOST_SUFFIX:     $zjq_gar_host_suffix,
        _RBGY_ARK_SUFFIX_IMAGE:    $zjq_ark_suffix_image,
        _RBGY_ARK_SUFFIX_DIAGS:    $zjq_ark_suffix_diags,
        _RBGY_INSCRIBE_TIMESTAMP:  $zjq_inscribe_ts,
        _RBGY_IMAGE_1:             $zjq_image_1,
        _RBGY_IMAGE_2:             $zjq_image_2,
        _RBGY_IMAGE_3:             $zjq_image_3,
        _RBGA_GAR_HOST:            $zjq_rbga_gar_host,
        _RBGA_GAR_PATH:            $zjq_rbga_gar_path,
        _RBGA_VESSEL:              $zjq_moniker,
        _RBGA_VESSEL_MODE:         "conjure",
        _RBGA_GIT_COMMIT:          $zjq_git_commit,
        _RBGA_GIT_BRANCH:          $zjq_git_branch,
        _RBGA_GIT_REPO:            $zjq_git_repo,
        _RBGA_INSCRIBE_TIMESTAMP:  $zjq_inscribe_ts,
        _RBGA_ARK_SUFFIX_IMAGE:    $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:    $zjq_rbga_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:    $zjq_ark_suffix_diags,
        _RBGA_BIND_SOURCE:         "",
        _RBGA_GRAFT_SOURCE:        "",
        _RBGA_DOCKERFILE_CONTENT:  $zjq_rbga_dockerfile
      },
      serviceAccount: $zjq_mason_sa,
      options: {
        requestedVerifyOption: "VERIFIED",
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose build JSON"

  mv "${z_build_file}" "${z_output_path}" \
    || buc_die "Failed to write final build JSON to ${z_output_path}"

  buc_log_args "Stitched ${#z_step_defs[@]} + context + about steps to ${z_output_path}"
}

# Resolve vessel argument: accepts a sigil (e.g., rbev-sentry-debian-slim) or a path
# (e.g., rbev-vessels/rbev-sentry-debian-slim).  On no-arg or invalid arg, lists
# available vessels and dies.  On success, writes resolved path to ZRBF_VESSEL_RESOLVED_DIR_FILE.
zrbf_resolve_vessel() {
  zrbf_sentinel

  local -r z_arg="${1:-}"

  # Try as path first, then as sigil under RBRR_VESSEL_DIR
  if test -n "${z_arg}" && test -d "${z_arg}" && test -f "${z_arg}/rbrv.env"; then
    printf '%s' "${z_arg}" > "${ZRBF_VESSEL_RESOLVED_DIR_FILE}" \
      || buc_die "Failed to write resolved vessel path"
    return 0
  fi
  if test -n "${z_arg}" && test -d "${RBRR_VESSEL_DIR}/${z_arg}" && test -f "${RBRR_VESSEL_DIR}/${z_arg}/rbrv.env"; then
    printf '%s' "${RBRR_VESSEL_DIR}/${z_arg}" > "${ZRBF_VESSEL_RESOLVED_DIR_FILE}" \
      || buc_die "Failed to write resolved vessel path"
    return 0
  fi

  # Resolution failed — list available vessels and die
  local z_sigils=""
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
  buc_step "Available vessels:"
  local z_sigil=""
  for z_sigil in ${z_sigils}; do
    buc_bare "        ${z_sigil}"
  done
  if test -z "${z_arg}"; then
    buc_die "Vessel argument required (sigil or path)"
  fi
  buc_die "Vessel not found: ${z_arg}"
}

zrbf_load_vessel() {
  zrbf_sentinel

  local z_vessel_dir="$1"

  buc_log_args 'Validate vessel directory exists'
  test -d "${z_vessel_dir}" || buc_die "Vessel directory not found: ${z_vessel_dir}"

  buc_log_args 'Check for rbrv.env file'
  local z_vessel_env="${z_vessel_dir}/rbrv.env"
  test -f "${z_vessel_env}" || buc_die "Vessel configuration not found: ${z_vessel_env}"

  buc_log_args 'Source vessel configuration'
  source "${z_vessel_env}" || buc_die "Failed to source vessel config: ${z_vessel_env}"

  buc_log_args 'Validate vessel directory matches sigil'
  local z_vessel_dir_clean="${z_vessel_dir%/}"  # Strip any trailing slash
  local z_dir_name="${z_vessel_dir_clean##*/}"  # Extract directory name
  buc_log_args "  z_vessel_dir = ${z_vessel_dir}"
  buc_log_args "  z_dir_name   = ${z_dir_name}"
  test "${z_dir_name}" = "${RBRV_SIGIL}" || buc_die "Vessel sigil '${RBRV_SIGIL}' does not match directory name '${z_dir_name}'"

  buc_log_args 'Validate vessel path matches expected pattern'
  local z_expected_vessel_dir="${RBRR_VESSEL_DIR}/${RBRV_SIGIL}"
  local z_vessel_realpath=""
  z_vessel_realpath=$(cd "${z_vessel_dir}" && pwd) || buc_die "Failed to resolve vessel directory path"
  local z_expected_realpath=""
  z_expected_realpath=$(cd "${z_expected_vessel_dir}" && pwd) || buc_die "Failed to resolve expected vessel path"
  test "${z_vessel_realpath}" = "${z_expected_realpath}" || buc_die "Vessel directory '${z_vessel_dir}' does not match expected location '${z_expected_vessel_dir}'"

  buc_log_args 'Store loaded vessel info for use by commands'
  echo "${RBRV_SIGIL}" > "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Failed to store vessel sigil"

  buc_info "Loaded vessel: ${RBRV_SIGIL}"
}

# Push vessel build context to GAR as a FROM SCRATCH OCI image.
# The context image carries the Dockerfile and supporting files that GCB
# needs in /workspace during the build.  This replaces the rubric repo's
# role as the build-context delivery mechanism.
#
# Args: token  sigil  build_context_path
# Side-effect: writes context image tag to ZRBF_CONTEXT_TAG_FILE
zrbf_push_build_context() {
  zrbf_sentinel

  local -r z_token="$1"
  local -r z_sigil="$2"
  local -r z_bldctx="$3"

  test -d "${z_bldctx}" || buc_die "Build context directory not found: ${z_bldctx}"

  local -r z_gar_host="${ZRBF_REGISTRY_HOST}"
  local -r z_context_tag_file="${ZRBF_CONTEXT_PREFIX}tag.txt"
  local -r z_context_dockerfile="${ZRBF_CONTEXT_PREFIX}Dockerfile"

  # Generate context timestamp
  local -r z_ctx_ts_file="${ZRBF_CONTEXT_PREFIX}ts.txt"
  date -u +'%y%m%d%H%M%S' > "${z_ctx_ts_file}" || buc_die "Failed to generate context timestamp"
  local z_ctx_ts=""
  z_ctx_ts=$(<"${z_ctx_ts_file}")
  test -n "${z_ctx_ts}" || buc_die "Empty context timestamp"

  local -r z_context_tag="${z_gar_host}/${ZRBF_REGISTRY_PATH}/${z_sigil}:context-${z_ctx_ts}"

  # Build FROM SCRATCH image containing build context
  buc_step "Building context image for ${z_sigil}"
  printf 'FROM scratch\nCOPY . /build-context/\n' > "${z_context_dockerfile}" \
    || buc_die "Failed to write context Dockerfile"

  docker build --platform "${RBGC_BUILD_RUNNER_PLATFORM}" -f "${z_context_dockerfile}" -t "${z_context_tag}" "${z_bldctx}" \
    || buc_die "Failed to build context image"

  # Push to GAR
  buc_step "Pushing context image to GAR"
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${z_gar_host}" \
    || buc_die "GAR authentication failed for context push"

  docker push "${z_context_tag}" \
    || buc_die "Failed to push context image to GAR"

  echo "${z_context_tag}" > "${z_context_tag_file}" \
    || buc_die "Failed to persist context image tag"

  buc_info "Context image pushed: ${z_context_tag}"
}

zrbf_wait_build_completion() {
  zrbf_sentinel

  local z_max_polls="${1:?zrbf_wait_build_completion: max_polls required}"
  local z_label="${2:?zrbf_wait_build_completion: label required}"

  buc_step "${z_label}: Waiting for build completion"

  local z_build_id=""
  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}") || buc_die "No build ID found"
  test -n "${z_build_id}" || buc_die "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_polls=0
  local z_queued_advisory_shown=0
  local z_consecutive_failures=0
  local z_max_consecutive_failures=3
  local z_err_check_file="${ZRBF_STITCH_PREFIX}poll_err_check.txt"

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_polls=$((z_polls + 1))
    test "${z_polls}" -le "${z_max_polls}" || buc_die "${z_label}: Build timeout after ${z_max_polls} polls"

    buc_log_args "Fetch build status (poll ${z_polls}/${z_max_polls})"
    curl -s                                                \
         --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
         --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
         -H "Authorization: Bearer ${z_token}"             \
         "${ZRBF_GCB_PROJECT_BUILDS_URL}/${z_build_id}"    \
         > "${ZRBF_BUILD_STATUS_FILE}"
    if test $? -ne 0; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Curl failed (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "Failed to get build status after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Validate response is non-empty
    if ! test -s "${ZRBF_BUILD_STATUS_FILE}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "Empty response (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "Empty build status after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Check for HTTP error responses (401/403/etc) — write to temp file, no subshell
    jq -r '.error.code // empty' "${ZRBF_BUILD_STATUS_FILE}" > "${z_err_check_file}" 2>/dev/null
    if test -s "${z_err_check_file}"; then
      z_consecutive_failures=$((z_consecutive_failures + 1))
      buc_warn "HTTP error $(<"${z_err_check_file}") (${z_consecutive_failures}/${z_max_consecutive_failures} consecutive)"
      test ${z_consecutive_failures} -ge ${z_max_consecutive_failures} \
        && buc_die "HTTP errors after ${z_max_consecutive_failures} consecutive failures"
      continue
    fi

    # Successful response — reset failure counter
    z_consecutive_failures=0

    jq -r '.status' "${ZRBF_BUILD_STATUS_FILE}" > "${ZRBF_STATUS_CHECK_FILE}" || buc_die "Failed to extract status"
    z_status=$(<"${ZRBF_STATUS_CHECK_FILE}")
    test -n "${z_status}" || buc_die "Status is empty"

    buc_info "${z_label}: ${z_status} (poll ${z_polls}/${z_max_polls})"

    if test "${z_status}" = "QUEUED" && test "${z_polls}" -ge 20 && test "${z_queued_advisory_shown}" = "0"; then
      z_queued_advisory_shown=1
      buc_warn "Build queued longer than normal — another build may be holding the private pool"
      buc_tabtarget "${RBZ_QUOTA_BUILD}"
    fi
  done

  test "${z_status}" = "SUCCESS" || buc_die "${z_label}: Build failed with status: ${z_status}"

  buc_success "${z_label}: Build completed successfully"
}

######################################################################
# External Functions (rbf_*)

rbf_inscribe() {
  zrbf_sentinel

  buc_doc_brief "Inscribe a reliquary: mirror all tool images from upstream to a datestamped GAR namespace"
  buc_doc_shown || return 0

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Compute reliquary datestamp: r + YYMMDDHHMMSS
  local -r z_reliquary="r${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  buc_info "Reliquary: ${z_reliquary}"

  # Submit inscribe as a Cloud Build job (docker pull/tag/push on GCB)
  zrbf_inscribe_submit "${z_token}" "${z_reliquary}"

  buc_success "Inscribe complete — reliquary ${z_reliquary} created"
  buc_info "Add RBRV_RELIQUARY=${z_reliquary} to vessel rbrv.env files to use this reliquary"
}

# Internal: Submit inscribe Cloud Build job.
# Single step: docker pull each upstream tool image, tag for GAR, push.
# Uses gcr.io/cloud-builders/docker as step image (always pullable — Google-hosted).
zrbf_inscribe_submit() {
  zrbf_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_reliquary="${2:?Reliquary datestamp required}"

  buc_step "Constructing inscribe Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Inscribe step image: Google-hosted docker builder (always pullable, even under NO_PUBLIC_EGRESS)
  local -r z_step_image="gcr.io/cloud-builders/docker"

  # Assemble inscribe step from script
  local -r z_script_path="${ZRBF_RBGJI_STEPS_DIR}/rbgji01-inscribe-mirror.sh"
  test -f "${z_script_path}" || buc_die "Inscribe step script not found: ${z_script_path}"

  local -r z_body_file="${ZRBF_RELIQUARY_PREFIX}body.txt"
  local -r z_escaped_file="${ZRBF_RELIQUARY_PREFIX}escaped.txt"

  buc_log_args "Reading inscribe step script (skip shebang)"
  tail -n +2 "${z_script_path}" > "${z_body_file}" \
    || buc_die "Failed to read inscribe step script"
  local z_body=""
  z_body=$(<"${z_body_file}")
  test -n "${z_body}" || buc_die "Empty inscribe script body"

  printf '#!/bin/bash\n%s' "${z_body}" > "${z_escaped_file}" \
    || buc_die "Failed to write escaped inscribe script body"

  local -r z_step_file="${ZRBF_RELIQUARY_PREFIX}step.json"
  echo "[]" > "${z_step_file}" || buc_die "Failed to initialize inscribe step JSON"

  local -r z_step_built="${ZRBF_RELIQUARY_PREFIX}step_built.json"
  jq \
    --arg name "${z_step_image}" \
    --arg id "inscribe-mirror" \
    --rawfile script "${z_escaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_step_file}" > "${z_step_built}" \
    || buc_die "Failed to build inscribe step JSON"
  mv "${z_step_built}" "${z_step_file}" \
    || buc_die "Failed to finalize inscribe step JSON"

  # Compose Build resource JSON
  buc_log_args "Composing inscribe Build resource JSON"
  local -r z_build_file="${ZRBF_RELIQUARY_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_step_file}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_reliquary    "${z_reliquary}" \
    --arg zjq_pool         "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGN_GAR_HOST:     $zjq_gar_host,
        _RBGN_GAR_PATH:     $zjq_gar_path,
        _RBGN_RELIQUARY:    $zjq_reliquary
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose inscribe build JSON"

  buc_log_args "Inscribe build JSON: ${z_build_file}"

  buc_step "Submitting inscribe Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "reliquary_build_create" "${z_build_file}"
  rbgu_http_require_ok "Inscribe build submission" "reliquary_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "reliquary_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Inscribe build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 120 "Inscribe"  # ~10 minutes at 5s intervals (7 images to pull+push)
}

rbf_enshrine() {
  zrbf_sentinel

  buc_doc_brief "Enshrine upstream base images to GAR via Cloud Build"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load
  zrbf_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBF_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbf_load_vessel "${z_vessel_dir}"
  test "${RBRV_VESSEL_MODE:-}" = "conjure" \
    || buc_die "Vessel '${RBRV_SIGIL}' is not a conjure vessel (mode: ${RBRV_VESSEL_MODE:-unset})"

  # Check for at least one ORIGIN declaration
  local z_has_origin=false
  test -z "${RBRV_IMAGE_1_ORIGIN:-}" || z_has_origin=true
  test -z "${RBRV_IMAGE_2_ORIGIN:-}" || z_has_origin=true
  test -z "${RBRV_IMAGE_3_ORIGIN:-}" || z_has_origin=true
  test "${z_has_origin}" = "true" \
    || buc_die "Vessel '${RBRV_SIGIL}' has no RBRV_IMAGE_n_ORIGIN declarations"

  # Resolve tool images from reliquary (enshrine uses skopeo from reliquary)
  zrbf_resolve_tool_images

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Submit enshrine as a Cloud Build job (skopeo runs on GCB, not locally)
  zrbf_enshrine_submit "${z_token}"

  # Extract anchor results from build step outputs
  local -r z_rbrv_file="${z_vessel_dir}/rbrv.env"
  zrbf_enshrine_extract_anchors "${z_rbrv_file}"

  buc_success "Enshrine complete for vessel: ${RBRV_SIGIL}"
}

# Internal: Submit enshrine Cloud Build job
# Single step: skopeo inspect + copy for each ORIGIN slot, returning anchors via buildStepOutputs.
zrbf_enshrine_submit() {
  zrbf_sentinel

  local -r z_token="$1"

  buc_step "Constructing enshrine Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Assemble enshrine step from script
  local -r z_script_path="${ZRBF_RBGJE_STEPS_DIR}/rbgje01-enshrine-copy.sh"
  test -f "${z_script_path}" || buc_die "Enshrine step script not found: ${z_script_path}"

  local -r z_body_file="${ZRBF_ENSHRINE_PREFIX}body.txt"
  local -r z_escaped_file="${ZRBF_ENSHRINE_PREFIX}escaped.txt"

  buc_log_args "Reading enshrine step script (skip shebang)"
  tail -n +2 "${z_script_path}" > "${z_body_file}" \
    || buc_die "Failed to read enshrine step script"
  local z_body=""
  z_body=$(<"${z_body_file}")
  test -n "${z_body}" || buc_die "Empty enshrine script body"

  printf '#!/bin/bash\n%s' "${z_body}" > "${z_escaped_file}" \
    || buc_die "Failed to write escaped enshrine script body"

  local -r z_step_file="${ZRBF_ENSHRINE_PREFIX}step.json"
  echo "[]" > "${z_step_file}" || buc_die "Failed to initialize enshrine step JSON"

  local -r z_step_built="${ZRBF_ENSHRINE_PREFIX}step_built.json"
  jq \
    --arg name "${ZRBF_TOOL_SKOPEO}" \
    --arg id "enshrine-copy" \
    --rawfile script "${z_escaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_step_file}" > "${z_step_built}" \
    || buc_die "Failed to build enshrine step JSON"
  mv "${z_step_built}" "${z_step_file}" \
    || buc_die "Failed to finalize enshrine step JSON"

  # Compose Build resource JSON
  buc_log_args "Composing enshrine Build resource JSON"
  local -r z_build_file="${ZRBF_ENSHRINE_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_step_file}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_origin_1     "${RBRV_IMAGE_1_ORIGIN:-}" \
    --arg zjq_origin_2     "${RBRV_IMAGE_2_ORIGIN:-}" \
    --arg zjq_origin_3     "${RBRV_IMAGE_3_ORIGIN:-}" \
    --arg zjq_pool         "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGE_GAR_HOST:          $zjq_gar_host,
        _RBGE_GAR_PATH:          $zjq_gar_path,
        _RBGE_IMAGE_1_ORIGIN:    $zjq_origin_1,
        _RBGE_IMAGE_2_ORIGIN:    $zjq_origin_2,
        _RBGE_IMAGE_3_ORIGIN:    $zjq_origin_3
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose enshrine build JSON"

  buc_log_args "Enshrine build JSON: ${z_build_file}"

  buc_step "Submitting enshrine Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "enshrine_build_create" "${z_build_file}"
  rbgu_http_require_ok "Enshrine build submission" "enshrine_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "enshrine_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Enshrine build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 50 "Enshrine"  # ~4 minutes at 5s intervals
}

# Internal: Extract anchor results from completed enshrine build and write to vessel regime
# Reads buildStepOutputs from the build status response (populated by /builder/outputs/output)
zrbf_enshrine_extract_anchors() {
  zrbf_sentinel

  local -r z_rbrv_file="$1"
  test -f "${z_rbrv_file}" || buc_die "Vessel regime file not found: ${z_rbrv_file}"

  buc_step "Extracting anchor results from build step outputs"

  # buildStepOutputs[0] is base64-encoded JSON from the enshrine step
  local -r z_b64_file="${ZRBF_ENSHRINE_PREFIX}output_b64.txt"
  local -r z_output_file="${ZRBF_ENSHRINE_PREFIX}output.json"

  jq -r '.results.buildStepOutputs[0] // empty' "${ZRBF_BUILD_STATUS_FILE}" \
    > "${z_b64_file}" || buc_die "Failed to extract buildStepOutputs from build result"
  test -s "${z_b64_file}" || buc_die "No buildStepOutputs in build result — enshrine step did not produce output"

  base64 -d < "${z_b64_file}" > "${z_output_file}" \
    || buc_die "Failed to decode buildStepOutputs base64"
  test -s "${z_output_file}" || buc_die "Empty decoded buildStepOutputs"

  buc_log_args "Enshrine output:"
  buc_log_pipe < "${z_output_file}"

  # Write each anchor back to the vessel regime file
  local z_n=""
  local z_slot_key=""
  local z_anchor=""
  local z_anchor_var=""
  local z_anchor_line=""
  local z_updated_file=""

  for z_n in 1 2 3; do
    z_slot_key="slot_${z_n}"
    z_anchor=$(jq -r ".${z_slot_key}.anchor // empty" "${z_output_file}") || z_anchor=""
    test -n "${z_anchor}" || continue

    z_anchor_var="RBRV_IMAGE_${z_n}_ANCHOR"
    z_anchor_line="${z_anchor_var}=${z_anchor}"
    z_updated_file="${ZRBF_ENSHRINE_PREFIX}${z_n}_updated_rbrv.env"

    buc_step "Writing anchor slot ${z_n}: ${z_anchor}"

    # Replace existing ANCHOR line or append — bash-native read/match/write
    local z_found=false
    while IFS= read -r z_line; do
      if [[ "${z_line}" == ${z_anchor_var}=* ]]; then
        printf '%s\n' "${z_anchor_line}"
        z_found=true
      else
        printf '%s\n' "${z_line}"
      fi
    done < "${z_rbrv_file}" > "${z_updated_file}" \
      || buc_die "Failed to process ${z_rbrv_file} for ${z_anchor_var}"

    if [[ "${z_found}" != "true" ]]; then
      printf '%s\n' "${z_anchor_line}" >> "${z_updated_file}" \
        || buc_die "Failed to append ${z_anchor_var}"
    fi

    cp "${z_updated_file}" "${z_rbrv_file}" || buc_die "Failed to write updated rbrv.env"

    buc_success "Slot ${z_n} enshrined: ${z_anchor}"
  done
}

rbf_ordain() {
  zrbf_sentinel

  buc_doc_brief "Ordain a consecration from a vessel (conjure, mirror, or graft based on vessel mode)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path)
  zrbf_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBF_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"

  # Peek at vessel mode without sourcing (sourcing makes vars readonly,
  # and the downstream function will source again via zrbf_load_vessel)
  local -r z_rbrv_file="${z_vessel_dir}/rbrv.env"
  local z_mode=""
  local z_mode_line=""
  while IFS= read -r z_mode_line || test -n "${z_mode_line}"; do
    case "${z_mode_line}" in
      RBRV_VESSEL_MODE=*) z_mode="${z_mode_line#RBRV_VESSEL_MODE=}"; break ;;
    esac
  done < "${z_rbrv_file}"
  z_mode="${z_mode:-conjure}"
  case "${z_mode}" in
    conjure) rbf_build "${z_vessel_dir}" ;;
    bind)    rbf_mirror "${z_vessel_dir}" ;;
    graft)   rbf_graft "${z_vessel_dir}" ;;
    *)       buc_die "Unknown vessel mode: ${z_mode}" ;;
  esac

  # Chaining: read consecration persisted by mode dispatch
  buc_step "Reading consecration from mode dispatch output"
  local z_consecration=""
  z_consecration=$(<"${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}") \
    || buc_die "Failed to read consecration from output"
  test -n "${z_consecration}" || buc_die "Empty consecration in output"

  # Metadata pipeline: graft uses combined about+vouch; conjure/bind already have about, need standalone vouch
  case "${z_mode}" in
    conjure)
      buc_info "About produced by combined conjure job — proceeding to vouch"
      rbf_vouch "${z_vessel_dir}" "${z_consecration}"
      ;;
    graft)
      zrbf_graft_metadata_submit "${z_vessel_dir}" "${z_consecration}"
      ;;
    bind)
      buc_info "About produced by combined bind job — proceeding to vouch"
      rbf_vouch "${z_vessel_dir}" "${z_consecration}"
      ;;
    *)
      buc_die "Unknown vessel mode in chaining: ${z_mode}"
      ;;
  esac
}

rbf_build() {
  zrbf_sentinel

  local -r z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Build container image from vessel via direct builds.create submission"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die "Vessel directory required"
  fi

  # Load and validate vessel
  zrbf_load_vessel "${z_vessel_dir}"

  buc_log_args "Verify vessel has conjuring configuration"
  test -n "${RBRV_CONJURE_DOCKERFILE:-}" || buc_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_DOCKERFILE)"
  test -n "${RBRV_CONJURE_BLDCONTEXT:-}" || buc_die "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_BLDCONTEXT)"

  buc_log_args "Resolve paths from vessel configuration"
  test -f "${RBRV_CONJURE_DOCKERFILE}" || buc_die "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
  test -d "${RBRV_CONJURE_BLDCONTEXT}" || buc_die "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"

  buc_info "Building vessel image: ${RBRV_SIGIL}"

  # Resolve tool images from reliquary (required for step image references)
  zrbf_resolve_tool_images

  # Source Director RBRA for credentials
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  # Authenticate as Director
  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Quota preflight -- warn if insufficient capacity
  zrbf_quota_preflight "${z_token}"

  # Registry preflight -- verify reliquary and enshrined base images exist before expensive operations
  zrbf_registry_preflight "${z_token}" "${z_vessel_dir}"

  # Capture git metadata (stitch needs ZRBF_GIT_INFO_FILE)
  buc_step "Capturing git metadata"
  zrbf_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")
  jq -n \
    --arg commit "${z_git_commit}" \
    --arg branch "${z_git_branch}" \
    --arg repo   "${z_git_repo}" \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBF_GIT_INFO_FILE}" || buc_die "Failed to write git info JSON for stitch"
  buc_info "Git: ${z_git_commit:0:8} on ${z_git_branch}"

  # Push build context to GAR as FROM SCRATCH image
  zrbf_push_build_context "${z_token}" "${RBRV_SIGIL}" "${RBRV_CONJURE_BLDCONTEXT}"
  local z_context_tag=""
  z_context_tag=$(<"${ZRBF_CONTEXT_PREFIX}tag.txt")
  test -n "${z_context_tag}" || buc_die "Empty context image tag after push"

  # Stitch build JSON — generates complete builds.create resource directly
  buc_step "Stitching build JSON"
  local -r z_build_file="${ZRBF_CONTEXT_PREFIX}build.json"
  local -r z_inscribe_ts="c${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  zrbf_stitch_build_json "${z_build_file}" "${z_inscribe_ts}" "${z_context_tag}"

  buc_info "Inscribe timestamp: ${z_inscribe_ts}"

  # Submit via builds.create (no source — context delivered via GAR image)
  buc_step "Submitting build via builds.create"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "build_direct_create" "${z_build_file}"
  rbgu_http_require_ok "Direct build submission" "build_direct_create"

  # Extract build ID from Operation response
  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "build_direct_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Build dispatched: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 960 "Conjure"  # 80 minutes at 5s intervals

  # Discover consecration from build step output (strong tie — no GAR scanning)
  # Step[0] is extract-context (no output).  Step[1] is derive-tag-base which
  # writes consecration to /builder/outputs/output (base64-encoded in buildStepOutputs[1]).
  buc_step "Discovering consecration from build step output"

  local z_step_output=""
  jq -r '.results.buildStepOutputs[1] // empty' "${ZRBF_BUILD_STATUS_FILE}" > "${ZRBF_SCRATCH_FILE}" \
    || buc_die "Failed to extract buildStepOutputs[1] from build response"
  z_step_output=$(<"${ZRBF_SCRATCH_FILE}")
  test -n "${z_step_output}" || buc_die "Build step 1 output empty — derive-tag-base may not have written to /builder/outputs/output"

  local -r z_step_b64_file="${BURD_TEMP_DIR}/rbf_step_b64.txt"
  local -r z_step_decoded_file="${BURD_TEMP_DIR}/rbf_step_decoded.txt"
  printf '%s' "${z_step_output}" > "${z_step_b64_file}" \
    || buc_die "Failed to write step output for decoding"
  base64 -d < "${z_step_b64_file}" > "${z_step_decoded_file}" \
    || buc_die "Failed to base64-decode build step output"
  local z_found_consecration=""
  z_found_consecration=$(<"${z_step_decoded_file}")
  test -n "${z_found_consecration}" || buc_die "Decoded consecration is empty"
  buc_info "Discovered consecration: ${z_found_consecration}"

  # Persist to output directory for test harness consumption
  echo "${z_vessel_dir}" > "${ZRBF_OUTPUT_VESSEL_DIR}" \
    || buc_die "Failed to write vessel dir to output"
  echo "${z_found_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}" \
    || buc_die "Failed to write consecration to output"

  # Write GAR root fact file (registry prefix for composing full refs)
  echo "${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_GAR_ROOT}" \
    || buc_die "Failed to write GAR root fact file"

  # Write ark stem fact file (sigil:consecration base for composing artifact refs)
  echo "${RBRV_SIGIL}:${z_found_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_STEM}" \
    || buc_die "Failed to write ark stem fact file"

  # Write per-platform yield fact files
  local z_plat=""
  local z_plat_suffix=""
  local z_yield_tag=""
  for z_plat in ${RBRV_CONJURE_PLATFORMS//,/ }; do
    z_plat_suffix="${z_plat#linux/}"
    z_plat_suffix="${z_plat_suffix//\//}"
    z_yield_tag="${z_found_consecration}${RBGC_ARK_SUFFIX_IMAGE}-${z_plat_suffix}"
    echo "${RBRV_SIGIL}:${z_yield_tag}" \
      > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_YIELD}${RBGC_ARK_SUFFIX_IMAGE}-${z_plat_suffix}" \
      || buc_die "Failed to write yield fact file for ${z_plat}"
    buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_YIELD}${RBGC_ARK_SUFFIX_IMAGE}-${z_plat_suffix}"
  done

  # Write build ID fact file (dispatched build ID for cross-check with vouch provenance)
  echo "${z_build_id}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}" \
    || buc_die "Failed to write build ID fact file"

  buc_info "Output: ${ZRBF_OUTPUT_VESSEL_DIR}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_GAR_ROOT}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_STEM}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}"

  buc_success "Vessel image built: ${RBRV_SIGIL}"
}

######################################################################
# Kludge Build - Local image build for development
#
# Builds a vessel image locally using docker build, tags it with a
# kludge consecration (k-prefixed timestamp) in the same GAR-style
# format that compose and rbob_charge expect. Also creates a fake
# vouch tag (same image, aliased) so the vouch gate passes.
#
# No Cloud Build, no GAR push, no credentials consumed.
# Host platform only (no multi-arch).

rbf_kludge() {
  zrbf_sentinel

  buc_doc_brief "Build vessel image locally for development (no Cloud Build, no GAR push)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path)
  zrbf_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBF_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"

  # Load vessel configuration
  zrbf_load_vessel "${z_vessel_dir}"

  # Validate conjure mode (bind and graft don't have local Dockerfiles)
  test "${RBRV_VESSEL_MODE}" = "conjure" \
    || buc_die "Kludge only supports conjure vessels (got: ${RBRV_VESSEL_MODE})"
  test -n "${RBRV_CONJURE_DOCKERFILE:-}" \
    || buc_die "Vessel '${RBRV_SIGIL}' has no RBRV_CONJURE_DOCKERFILE"
  test -n "${RBRV_CONJURE_BLDCONTEXT:-}" \
    || buc_die "Vessel '${RBRV_SIGIL}' has no RBRV_CONJURE_BLDCONTEXT"
  test -f "${RBRV_CONJURE_DOCKERFILE}" \
    || buc_die "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
  test -d "${RBRV_CONJURE_BLDCONTEXT}" \
    || buc_die "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"

  # Resolve base images (use ORIGIN directly — no GAR enshrine lookup for local builds)
  local z_build_args=()
  local z_slot="" z_origin_var="" z_origin=""
  for z_slot in 1 2 3; do
    z_origin_var="RBRV_IMAGE_${z_slot}_ORIGIN"
    z_origin="${!z_origin_var:-}"
    test -n "${z_origin}" || continue
    z_build_args+=("--build-arg" "RBF_IMAGE_${z_slot}=${z_origin}")
    buc_info "Image slot ${z_slot}: ${z_origin}"
  done
  test ${#z_build_args[@]} -gt 0 || buc_die "No RBRV_IMAGE_n_ORIGIN found in vessel config"

  # Generate kludge consecration (k prefix distinguishes from conjure c, bind b)
  local -r z_consecration="k${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"

  # Construct image refs matching compose/vouch-gate format
  local -r z_image_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${RBRV_SIGIL}:${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_vouch_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${RBRV_SIGIL}:${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"

  buc_step "Kludge build: ${RBRV_SIGIL}"
  buc_info "Consecration: ${z_consecration}"
  buc_info "Image tag: ${z_image_ref}"

  # Build locally (host platform only — no multi-arch for dev builds)
  buc_step "Building image locally"
  docker build \
    "${z_build_args[@]}" \
    -f "${RBRV_CONJURE_DOCKERFILE}" \
    -t "${z_image_ref}" \
    "${RBRV_CONJURE_BLDCONTEXT}" \
    || buc_die "Local build failed for ${RBRV_SIGIL}"

  # Create fake vouch tag (same image, aliased — satisfies rbob_charge vouch gate)
  buc_step "Creating vouch tag"
  docker tag "${z_image_ref}" "${z_vouch_ref}" \
    || buc_die "Failed to create vouch tag"

  # Persist consecration to output directory
  echo "${z_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}" \
    || buc_die "Failed to write consecration to output"

  buc_success "Kludge build complete: ${RBRV_SIGIL}"
  buc_bare ""
  buc_bare "  Consecration: ${z_consecration}"
  buc_bare "  Image:        ${z_image_ref}"
  buc_bare "  Vouch:        ${z_vouch_ref}"
  buc_bare ""
  buc_bare "  Update nameplate .env file:"
  case "${RBRV_SIGIL}" in
    *sentry*) buc_bare "    RBRN_SENTRY_CONSECRATION=${z_consecration}" ;;
    *bottle*|*ifrit*) buc_bare "    RBRN_BOTTLE_CONSECRATION=${z_consecration}" ;;
    *) buc_bare "    RBRN_*_CONSECRATION=${z_consecration}" ;;
  esac
}

rbf_jettison() {
  zrbf_sentinel

  local z_locator="${1:-}"
  local z_force="${2:-}"

  # Documentation block
  buc_doc_brief "Jettison an image tag from the registry by locator"
  buc_doc_param "locator" "Image locator in moniker:tag format (e.g., rbev-busybox:20251231T160211Z-img)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
  buc_doc_shown || return 0

  # Validate locator parameter
  test -n "${z_locator}" || buc_die "Locator parameter required (moniker:tag)"

  # Parse locator into moniker and tag
  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected moniker:tag" ;;
  esac
  local z_moniker="${z_locator%%:*}"
  local z_tag="${z_locator#*:}"
  test -n "${z_moniker}" || buc_die "Moniker is empty in locator"
  test -n "${z_tag}" || buc_die "Tag is empty in locator"

  # Check for --force flag
  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Confirm jettison unless --force
  if test "${z_skip_confirm}" = "false"; then
    buc_require "Will jettison: ${z_locator}" "yes"
  fi

  buc_step "Jettisoning: ${z_locator}"

  # Jettison by tag reference
  local z_status_file="${ZRBF_DELETE_PREFIX}status.txt"
  local z_response_file="${ZRBF_DELETE_PREFIX}response.json"

  curl -X DELETE -s                                   \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
    -H "Authorization: Bearer ${z_token}"             \
    -w "%{http_code}"                                 \
    -o "${z_response_file}"                           \
    "${ZRBF_REGISTRY_API_BASE}/${z_moniker}/manifests/${z_tag}" \
    > "${z_status_file}" || buc_die "DELETE request failed"

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

rbf_wrest() {
  zrbf_sentinel

  local z_locator="${1:-}"

  # Documentation block
  buc_doc_brief "Wrest an image from the registry to local container runtime by locator"
  buc_doc_param "locator" "Image locator in moniker:tag format"
  buc_doc_shown || return 0

  # Validate locator parameter
  test -n "${z_locator}" || buc_die "Locator parameter required (moniker:tag)"

  # Parse locator into moniker and tag
  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die "Invalid locator format. Expected moniker:tag" ;;
  esac
  local z_moniker="${z_locator%%:*}"
  local z_tag="${z_locator#*:}"
  test -n "${z_moniker}" || buc_die "Moniker is empty in locator"
  test -n "${z_tag}" || buc_die "Tag is empty in locator"

  buc_step "Authenticating as Retriever"

  test -f "${RBDC_RETRIEVER_RBRA_FILE}" || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Logging into container registry"

  # Construct full image reference
  local z_full_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_locator}"

  # Docker login to GAR
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${ZRBF_REGISTRY_HOST}" \
    || buc_die "Container runtime authentication failed"

  buc_step "Pulling image: ${z_full_ref}"

  # Pull image
  docker pull "${z_full_ref}" || buc_die "Image pull failed"

  # Get local image ID
  local z_image_id
  docker inspect --format='{{.Id}}' "${z_full_ref}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
    || buc_die "Failed to get image ID"
  z_image_id=$(<"${ZRBF_SCRATCH_FILE}")

  # Display results
  echo ""
  echo "Image wrested: ${z_full_ref}"
  echo "Local image ID: ${z_image_id}"

  buc_success "Image wrest complete"
}

######################################################################
# Mirror (bind vessel → GAR)

rbf_mirror() {
  zrbf_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Mirror a bind vessel image from upstream to GAR via combined Cloud Build (skopeo copy + about)"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  # No-arg: list available vessels
  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die "Vessel directory required"
  fi

  # Load and validate vessel
  zrbf_load_vessel "${z_vessel_dir}"
  test "${RBRV_VESSEL_MODE:-}" = "bind" \
    || buc_die "Vessel '${RBRV_SIGIL}' is not a bind vessel (mode: ${RBRV_VESSEL_MODE:-unset})"
  test -n "${RBRV_BIND_IMAGE:-}" \
    || buc_die "RBRV_BIND_IMAGE not set for bind vessel '${RBRV_SIGIL}'"

  # Resolve tool images from reliquary (mirror uses skopeo + about steps from reliquary)
  zrbf_resolve_tool_images

  # Dirty-tree guard (same as inscribe — mirror should match a committed state)
  buc_step "Verifying clean working tree"
  git diff --quiet \
    || buc_die "Working tree has unstaged changes — commit before mirroring"
  git diff --cached --quiet \
    || buc_die "Index has staged changes — commit before mirroring"

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # GAR coordinates
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_base="${z_gar_host}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"

  # Generate consecration timestamps: bYYMMDDHHMMSS-rYYMMDDHHMMSS
  local -r z_mirror_ts="b${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  local -r z_build_ts_file="${ZRBF_MIRROR_PREFIX}build_ts.txt"
  date -u +'%y%m%d%H%M%S' > "${z_build_ts_file}" || buc_die "Failed to generate build timestamp"
  local z_build_ts
  z_build_ts="r$(<"${z_build_ts_file}")"
  test -n "${z_build_ts}" || buc_die "Empty build timestamp from ${z_build_ts_file}"
  local -r z_consecration="${z_mirror_ts}-${z_build_ts}"

  buc_info "Consecration: ${z_consecration}"

  # Persist to output directory for chaining by rbf_ordain
  echo "${z_vessel_dir}" > "${ZRBF_OUTPUT_VESSEL_DIR}" \
    || buc_die "Failed to write vessel dir to output"
  echo "${z_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}" \
    || buc_die "Failed to write consecration to output"

  # Write GAR root fact file
  echo "${z_gar_base}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_GAR_ROOT}" \
    || buc_die "Failed to write GAR root fact file"

  # Write ark stem fact file
  echo "${RBRV_SIGIL}:${z_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_STEM}" \
    || buc_die "Failed to write ark stem fact file"

  # Write yield fact file (single-platform bind image)
  local -r z_bind_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  echo "${RBRV_SIGIL}:${z_bind_image_tag}" \
    > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_YIELD}${RBGC_ARK_SUFFIX_IMAGE}" \
    || buc_die "Failed to write yield fact file"

  # Submit combined Cloud Build (skopeo image copy + about steps)
  zrbf_mirror_submit "${z_consecration}" "${z_token}"

  # Summary
  echo ""
  buc_success "Mirror complete: ${RBRV_SIGIL}"
  echo "  Consecration: ${z_consecration}"
}

# Internal: submit combined mirror Cloud Build job (skopeo image copy + about steps)
# Args: consecration token
zrbf_mirror_submit() {
  zrbf_sentinel

  local -r z_consecration="$1"
  local -r z_token="$2"

  buc_step "Constructing combined mirror Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Step 0: Mirror image via skopeo
  local -r z_mscript_path="${ZRBF_RBGJM_STEPS_DIR}/rbgjm01-mirror-image.sh"
  test -f "${z_mscript_path}" || buc_die "Mirror step script not found: ${z_mscript_path}"

  local -r z_mbody_file="${ZRBF_MIRROR_PREFIX}mirror_body.txt"
  local -r z_mescaped_file="${ZRBF_MIRROR_PREFIX}mirror_escaped.txt"
  local -r z_mirror_step_file="${ZRBF_MIRROR_PREFIX}mirror_step.json"
  local -r z_mirror_step_built="${ZRBF_MIRROR_PREFIX}mirror_step_built.json"

  buc_log_args "Reading mirror step script (skip shebang)"
  tail -n +2 "${z_mscript_path}" > "${z_mbody_file}" \
    || buc_die "Failed to read mirror step script"
  local z_mbody
  z_mbody=$(<"${z_mbody_file}")
  test -n "${z_mbody}" || buc_die "Empty mirror script body"

  printf '#!/bin/bash\n%s' "${z_mbody}" > "${z_mescaped_file}" \
    || buc_die "Failed to escape mirror script body"

  echo "[]" > "${z_mirror_step_file}" || buc_die "Failed to initialize mirror step JSON"
  jq \
    --arg name "${ZRBF_TOOL_SKOPEO}" \
    --arg id "mirror-image" \
    --rawfile script "${z_mescaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_mirror_step_file}" > "${z_mirror_step_built}" \
    || buc_die "Failed to build mirror step JSON"
  mv "${z_mirror_step_built}" "${z_mirror_step_file}" \
    || buc_die "Failed to finalize mirror step JSON"

  # Steps 1-4: About (shared with standalone about pipeline)
  local -r z_about_steps_file="${ZRBF_MIRROR_PREFIX}about_steps.json"
  zrbf_assemble_about_steps "${z_about_steps_file}" "${ZRBF_MIRROR_PREFIX}about_"

  # Combine: mirror step + about steps
  local -r z_combined_steps="${ZRBF_MIRROR_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1]' "${z_mirror_step_file}" "${z_about_steps_file}" \
    > "${z_combined_steps}" || buc_die "Failed to combine mirror and about steps"

  # Git metadata (shared temp files, idempotent)
  zrbf_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")

  # Mode-specific substitution values for bind
  local -r z_bind_source="${RBRV_BIND_IMAGE:-}"
  local z_dockerfile_content=""
  local -r z_dockerfile_max_bytes=4000
  if test -n "${RBRV_BIND_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_BIND_OPTIONAL_DOCKERFILE}"; then
    local -r z_df_size_file="${ZRBF_MIRROR_PREFIX}df_size.txt"
    wc -c < "${RBRV_BIND_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
      || buc_die "Failed to measure Dockerfile size"
    local z_df_size=""
    z_df_size=$(<"${z_df_size_file}")
    z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
    if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
      z_dockerfile_content=$(<"${RBRV_BIND_OPTIONAL_DOCKERFILE}")
    else
      buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
    fi
  fi

  # Compose Build resource JSON
  buc_log_args "Composing combined mirror Build resource JSON"
  local -r z_mirror_build_file="${ZRBF_MIRROR_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_combined_steps}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_vessel       "${RBRV_SIGIL}" \
    --arg zjq_consecration "${z_consecration}" \
    --arg zjq_vessel_mode  "bind" \
    --arg zjq_git_commit   "${z_git_commit}" \
    --arg zjq_git_branch   "${z_git_branch}" \
    --arg zjq_git_repo     "${z_git_repo}" \
    --arg zjq_build_id     "" \
    --arg zjq_inscribe_ts  "" \
    --arg zjq_bind_source  "${z_bind_source}" \
    --arg zjq_graft_source "" \
    --arg zjq_dockerfile   "${z_dockerfile_content}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_ark_suffix_diags "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_pool         "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_CONSECRATION:          $zjq_consecration,
        _RBGA_VESSEL_MODE:           $zjq_vessel_mode,
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              $zjq_build_id,
        _RBGA_INSCRIBE_TIMESTAMP:    $zjq_inscribe_ts,
        _RBGA_BIND_SOURCE:           $zjq_bind_source,
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:      $zjq_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:      $zjq_ark_suffix_diags
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_mirror_build_file}" \
    || buc_die "Failed to compose mirror build JSON"

  buc_log_args "Mirror build JSON: ${z_mirror_build_file}"

  buc_step "Submitting combined mirror Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "mirror_build_create" "${z_mirror_build_file}"
  rbgu_http_require_ok "Mirror build submission" "mirror_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "mirror_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Mirror build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 100 "Mirror"  # ~8 minutes at 5s intervals (image copy + about steps)
}

######################################################################
# Graft (graft vessel → GAR)

rbf_graft() {
  zrbf_sentinel

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Graft a locally-built image into GAR"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  # No-arg: list available vessels
  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die "Vessel directory required"
  fi

  # Load and validate vessel
  zrbf_load_vessel "${z_vessel_dir}"
  test "${RBRV_VESSEL_MODE:-}" = "graft" \
    || buc_die "Vessel '${RBRV_SIGIL}' is not a graft vessel (mode: ${RBRV_VESSEL_MODE:-unset})"

  # Tweak override: test infrastructure can inject graft image via ambient regime variable
  test "${BURE_TWEAK_NAME:-}" != "threemodegraft" || RBRV_GRAFT_IMAGE="${BURE_TWEAK_VALUE}"

  test -n "${RBRV_GRAFT_IMAGE:-}" \
    || buc_die "RBRV_GRAFT_IMAGE not set for graft vessel '${RBRV_SIGIL}'"

  # Resolve tool images from reliquary (graft about+vouch steps use tool images)
  zrbf_resolve_tool_images

  local -r z_local_image="${RBRV_GRAFT_IMAGE}"

  # No dirty-tree guard — image already built; git state irrelevant to container

  # Verify local image exists
  buc_step "Verifying local image exists"
  docker image inspect "${z_local_image}" > /dev/null 2>&1 \
    || buc_die "Local image not found: ${z_local_image} — build the image before grafting"
  buc_info "Local image confirmed: ${z_local_image}"

  # Extract image creation timestamp for consecration T1
  buc_step "Reading image creation timestamp"
  local -r z_created_file="${ZRBF_GRAFT_PREFIX}created.txt"
  docker image inspect --format '{{.Created}}' "${z_local_image}" > "${z_created_file}" \
    || buc_die "Failed to inspect image creation timestamp"
  local z_created=""
  z_created=$(<"${z_created_file}")
  test -n "${z_created}" || buc_die "Empty creation timestamp from docker inspect"
  buc_info "Image created: ${z_created}"

  # Parse ISO 8601 timestamp to YYMMDDHHMMSS
  # Input formats: 2024-01-15T10:30:45.123456789Z or 1970-01-01T00:00:00Z
  local z_created_clean="${z_created%%.*}"  # Remove fractional seconds
  z_created_clean="${z_created_clean%%Z}"   # Remove trailing Z if no fractional part
  z_created_clean="${z_created_clean%Z}"    # Handle edge case
  local -r z_cdate="${z_created_clean%%T*}"
  local -r z_ctime="${z_created_clean##*T}"
  local -r z_graft_ts="g${z_cdate:2:2}${z_cdate:5:2}${z_cdate:8:2}${z_ctime:0:2}${z_ctime:3:2}${z_ctime:6:2}"

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # GAR coordinates
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_base="${z_gar_host}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"

  # Generate push timestamp (T2) for consecration
  local -r z_push_ts_file="${ZRBF_GRAFT_PREFIX}push_ts.txt"
  date -u +'%y%m%d%H%M%S' > "${z_push_ts_file}" || buc_die "Failed to generate push timestamp"
  local z_push_ts
  z_push_ts="r$(<"${z_push_ts_file}")"
  test -n "${z_push_ts}" || buc_die "Empty push timestamp from ${z_push_ts_file}"
  local -r z_consecration="${z_graft_ts}-${z_push_ts}"
  local -r z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_image_ref="${z_gar_base}/${RBRV_SIGIL}:${z_image_tag}"

  buc_info "Consecration: ${z_consecration}"

  # Tag and push
  buc_step "Logging into GAR"
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${z_gar_host}" \
    || buc_die "GAR authentication failed"

  buc_step "Tagging local image"
  docker tag "${z_local_image}" "${z_image_ref}" \
    || buc_die "Failed to tag local image as ${z_image_ref}"

  buc_step "Pushing to GAR"
  buc_info "Target: ${z_image_ref}"
  docker push "${z_image_ref}" \
    || buc_die "Failed to push image to GAR"

  buc_info "Image pushed: ${z_image_ref}"

  # Persist to output directory for downstream consumption
  echo "${z_vessel_dir}" > "${ZRBF_OUTPUT_VESSEL_DIR}" \
    || buc_die "Failed to write vessel dir to output"
  echo "${z_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_CONSECRATION}" \
    || buc_die "Failed to write consecration to output"

  # Write GAR root fact file
  echo "${z_gar_base}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_GAR_ROOT}" \
    || buc_die "Failed to write GAR root fact file"

  # Write ark stem fact file
  echo "${RBRV_SIGIL}:${z_consecration}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_STEM}" \
    || buc_die "Failed to write ark stem fact file"

  # Write yield fact file (single-platform graft image)
  echo "${RBRV_SIGIL}:${z_image_tag}" \
    > "${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_YIELD}${RBGC_ARK_SUFFIX_IMAGE}" \
    || buc_die "Failed to write yield fact file"

  # Summary
  echo ""
  buc_success "Graft complete: ${RBRV_SIGIL}"
  echo "  Consecration: ${z_consecration}"
  echo "  Source:  ${z_local_image}"
  echo "  Image:   ${z_image_ref}"
}

rbf_summon() {
  zrbf_sentinel

  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # strip path prefix — accept directory path or bare moniker
  local z_consecration="${2:-}"

  # Documentation block
  buc_doc_brief "Summon an ark (pull -image, -about, and -vouch artifacts as a coherent unit)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  test -n "${z_vessel}" || buc_die "Vessel parameter required"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_step "Authenticating for retrieval"

  test -f "${RBDC_RETRIEVER_RBRA_FILE}" || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Construct ark tags — all use full consecration
  local z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"

  buc_step "Verifying ark existence"

  # Check if -image artifact exists
  local z_image_status_file="${ZRBF_DELETE_PREFIX}summon_image_status.txt"
  local z_image_response_file="${ZRBF_DELETE_PREFIX}summon_image_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_image_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_image_tag}" \
    > "${z_image_status_file}" || buc_die "HEAD request failed for -image artifact"

  local z_image_http_code
  z_image_http_code=$(<"${z_image_status_file}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"

  local z_image_exists=false
  if test "${z_image_http_code}" = "200"; then
    z_image_exists=true
  elif test "${z_image_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_image_http_code} when checking -image artifact"
  fi

  # Check if -about artifact exists
  local z_about_status_file="${ZRBF_DELETE_PREFIX}summon_about_status.txt"
  local z_about_response_file="${ZRBF_DELETE_PREFIX}summon_about_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_about_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_about_tag}" \
    > "${z_about_status_file}" || buc_die "HEAD request failed for -about artifact"

  local z_about_http_code
  z_about_http_code=$(<"${z_about_status_file}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"

  local z_about_exists=false
  if test "${z_about_http_code}" = "200"; then
    z_about_exists=true
  elif test "${z_about_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_about_http_code} when checking -about artifact"
  fi

  # Check if -vouch artifact exists
  local z_vouch_status_file="${ZRBF_DELETE_PREFIX}summon_vouch_status.txt"
  local z_vouch_response_file="${ZRBF_DELETE_PREFIX}summon_vouch_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_vouch_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_vouch_tag}" \
    > "${z_vouch_status_file}" || buc_die "HEAD request failed for -vouch artifact"

  local z_vouch_http_code
  z_vouch_http_code=$(<"${z_vouch_status_file}")
  test -n "${z_vouch_http_code}" || buc_die "HTTP status code is empty for -vouch"

  local z_vouch_exists=false
  if test "${z_vouch_http_code}" = "200"; then
    z_vouch_exists=true
  elif test "${z_vouch_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_vouch_http_code} when checking -vouch artifact"
  fi

  # Evaluate ark state
  if test "${z_image_exists}" = "false" && test "${z_about_exists}" = "false"; then
    buc_die "Consecration not found: neither -image nor -about exists"
  fi

  if test "${z_image_exists}" = "true" && test "${z_about_exists}" = "false"; then
    buc_warn "Orphaned artifact detected: -image exists but -about is missing"
  elif test "${z_image_exists}" = "false" && test "${z_about_exists}" = "true"; then
    buc_warn "Orphaned artifact detected: -about exists but -image is missing"
  fi

  buc_step "Logging into container registry"

  # Docker login to GAR
  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${ZRBF_REGISTRY_HOST}" \
    || buc_die "Container runtime authentication failed"

  # Pull -image artifact if exists
  if test "${z_image_exists}" = "true"; then
    buc_step "Pulling -image artifact"

    local z_image_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_vessel}:${z_image_tag}"
    docker pull "${z_image_ref}" || buc_die "Failed to pull -image artifact"
    buc_info "Retrieved: ${z_image_ref}"
  fi

  # Pull -about artifact if exists
  if test "${z_about_exists}" = "true"; then
    buc_step "Pulling -about artifact"

    local z_about_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_vessel}:${z_about_tag}"
    docker pull "${z_about_ref}" || buc_die "Failed to pull -about artifact"
    buc_info "Retrieved: ${z_about_ref}"
  fi

  # Pull -vouch artifact if exists
  if test "${z_vouch_exists}" = "true"; then
    buc_step "Pulling -vouch artifact"

    local z_vouch_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_vessel}:${z_vouch_tag}"
    docker pull "${z_vouch_ref}" || buc_die "Failed to pull -vouch artifact"
    buc_info "Retrieved: ${z_vouch_ref}"
  fi

  # Display results
  echo ""
  buc_success "Consecration summoned: ${z_vessel}/${z_consecration}"
  if test "${z_image_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_image_tag} retrieved"
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_about_tag} retrieved"
  fi
  if test "${z_vouch_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_vouch_tag} retrieved"
  fi
}

rbf_rubric_inscribe() {
  zrbf_sentinel

  buc_doc_brief "DEFERRED: Inscribe will become reliquary generation (₣Av). GitLab rubric path eliminated."
  buc_doc_shown || return 0

  buc_die "Inscribe is deferred — builds.create + pouch replaces the trigger/rubric path. Reliquary inscribe will be implemented in a future pace."

  # Dirty-tree guard: inscribed scripts must match a committed state
  buc_step "Verifying clean working tree"
  git diff --quiet \
    || buc_die "Working tree has unstaged changes — commit or stash before inscribing"
  git diff --cached --quiet \
    || buc_die "Index has staged changes — commit before inscribing"

  # Source Director RBRA for credentials (still needed for OAuth token)
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"
  test -n "${RBRR_RUBRIC_REPO_URL:-}" || buc_die "RBRR_RUBRIC_REPO_URL not set in rbrr.env"

  # Authenticate as Director early (needed for Secret Manager PAT retrieval)
  buc_step "Authenticating as Director"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Fetch GitLab token from Secret Manager (api token secret)
  buc_step "Fetching rubric repo token from Secret Manager"
  local -r z_secret_access_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/secrets/${RBGC_CBV2_API_TOKEN_SECRET_NAME}/versions/latest:access"
  rbgu_http_json "GET" "${z_secret_access_url}" "${z_token}" "inscribe_token_fetch"
  rbgu_http_require_ok "Fetch token from Secret Manager" "inscribe_token_fetch"
  local z_token_b64
  z_token_b64=$(rbgu_json_field_capture "inscribe_token_fetch" '.payload.data') \
    || buc_die "Failed to extract token payload"
  local z_gitlab_token_value
  z_gitlab_token_value=$(zrbf_base64_decode_capture "${z_token_b64}") \
    || buc_die "Failed to decode token"

  # Construct authenticated URL: https://gitlab.com/... → https://oauth2:TOKEN@gitlab.com/...
  local -r z_rubric_auth_url="https://oauth2:${z_gitlab_token_value}@${RBRR_RUBRIC_REPO_URL#https://}"

  # RBRR_RUBRIC_REPO_URL is already clean (no embedded PAT)
  local -r z_rubric_url_clean="${RBRR_RUBRIC_REPO_URL}"

  # Phase 0: Pin staleness gate eliminated (₣Av) — RBRG regime removed

  # Phase 1: Capture git metadata for build substitutions
  buc_step "Capturing git metadata for build substitutions"
  local z_git_commit=""
  local z_git_branch=""
  local z_git_remote=""
  local z_git_repo_url=""
  local z_git_repo=""
  local -r z_inscribe_git_commit_file="${ZRBF_INSCRIBE_PREFIX}git_commit.txt"
  local -r z_inscribe_git_branch_file="${ZRBF_INSCRIBE_PREFIX}git_branch.txt"
  local -r z_inscribe_git_remote_file="${ZRBF_INSCRIBE_PREFIX}git_remote.txt"
  local -r z_inscribe_git_repo_url_file="${ZRBF_INSCRIBE_PREFIX}git_repo_url.txt"
  git rev-parse HEAD > "${z_inscribe_git_commit_file}" || buc_die "Failed to get commit SHA"
  z_git_commit=$(<"${z_inscribe_git_commit_file}")
  test -n "${z_git_commit}" || buc_die "Empty commit SHA from ${z_inscribe_git_commit_file}"
  git rev-parse --abbrev-ref HEAD > "${z_inscribe_git_branch_file}" || buc_die "Failed to get branch"
  z_git_branch=$(<"${z_inscribe_git_branch_file}")
  test -n "${z_git_branch}" || buc_die "Empty branch from ${z_inscribe_git_branch_file}"
  git remote > "${z_inscribe_git_remote_file}" || buc_die "Failed to list git remotes"
  read -r z_git_remote < "${z_inscribe_git_remote_file}" || buc_die "Failed to read git remote from ${z_inscribe_git_remote_file}"
  test -n "${z_git_remote}" || buc_die "No git remotes found"
  git config --get "remote.${z_git_remote}.url" > "${z_inscribe_git_repo_url_file}" || buc_die "Failed to get repo URL"
  z_git_repo_url=$(<"${z_inscribe_git_repo_url_file}")
  test -n "${z_git_repo_url}" || buc_die "Empty repo URL from ${z_inscribe_git_repo_url_file}"
  # Extract repo path from remote URL (works for any git host: github.com, gitlab.com, etc.)
  # Strip protocol + host: https://gitlab.com/org/repo.git → org/repo.git
  # Then strip .git suffix
  z_git_repo="${z_git_repo_url#*://*/}"
  z_git_repo="${z_git_repo%.git}"

  jq -n \
    --arg commit "${z_git_commit}" \
    --arg branch "${z_git_branch}" \
    --arg repo   "${z_git_repo}" \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBF_GIT_INFO_FILE}" || buc_die "Failed to write git info"
  buc_info "Git: ${z_git_commit:0:8} on ${z_git_branch}"

  # Phase 2: Clone rubric repo, copy build contexts, generate JSON directly
  buc_step "Enumerating conjure vessels"
  local z_vessel_dir=""
  local z_vessel_dirs=()
  local z_vessel_sigils=()

  local z_grep_stderr="${ZRBF_INSCRIBE_PREFIX}grep_stderr.txt"
  for z_vessel_dir in "${RBRR_VESSEL_DIR}"/*/; do
    test -f "${z_vessel_dir}rbrv.env" || continue
    # Check for conjure mode by scanning for RBRV_CONJURE_DOCKERFILE
    grep -q "^RBRV_CONJURE_DOCKERFILE=" "${z_vessel_dir}rbrv.env" 2>"${z_grep_stderr}" || {
      buc_info "Skipping non-conjure vessel: ${z_vessel_dir##*/}"
      continue
    }
    z_vessel_dirs+=("${z_vessel_dir%/}")
  done
  test "${#z_vessel_dirs[@]}" -gt 0 || buc_die "No conjure vessels found in ${RBRR_VESSEL_DIR}"
  buc_info "Found ${#z_vessel_dirs[@]} conjure vessel(s)"

  buc_step "Cloning rubric repo (always-fresh)"
  rm -rf "${ZRBF_INSCRIBE_CLONE_DIR}" || buc_die "Failed to remove old rubric clone"
  mkdir -p "${ZRBF_INSCRIBE_CLONE_DIR%/*}" || buc_die "Failed to create clone parent directory"

  git clone --depth 1 "${z_rubric_auth_url}" "${ZRBF_INSCRIBE_CLONE_DIR}" \
    || buc_die "Failed to clone rubric repo"
  buc_info "Rubric repo cloned to ${ZRBF_INSCRIBE_CLONE_DIR}"

  buc_step "Copying build contexts and generating rubric JSON"
  local z_loaded_sigil=""
  local z_sigil=""
  local z_idx=0
  local z_rubric_vessel_dir=""
  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    # Isolation subshell: each vessel's rbrv.env sets different RBRV_* values that
    # would collide without isolation. Sigil survives via ZRBF_VESSEL_SIGIL_FILE.
    # Build context copy and stitch both happen inside the subshell.
    (
      zrbf_load_vessel "${z_vessel_dir}" || buc_die "Failed to load vessel: ${z_vessel_dir}"

      # Purge stale rubric content, copy fresh build context
      z_rubric_vessel_dir="${ZRBF_INSCRIBE_CLONE_DIR}/${RBRV_SIGIL}"
      rm -rf "${z_rubric_vessel_dir}" || buc_die "Failed to remove rubric vessel dir: ${z_rubric_vessel_dir}"
      cp -R "${z_vessel_dir}" "${z_rubric_vessel_dir}" \
        || buc_die "Failed to copy vessel build context for ${RBRV_SIGIL}"

      test -f "${z_rubric_vessel_dir}/Dockerfile" \
        || buc_die "Dockerfile missing in build context for ${RBRV_SIGIL}"

      buc_step "Stitching build JSON for ${RBRV_SIGIL}"
      zrbf_stitch_build_json "${z_rubric_vessel_dir}/cloudbuild.json" \
        || buc_die "Failed to stitch build JSON for ${RBRV_SIGIL}"
    ) || buc_die "Isolation subshell failed for vessel: ${z_vessel_dir}"

    z_loaded_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}") || buc_die "Failed to read vessel sigil after isolation subshell"
    z_vessel_sigils+=("${z_loaded_sigil}")

    buc_info "Synced and stitched: ${z_loaded_sigil}"
  done

  # Fill pre-commit placeholders in rubric clone copies
  # _RBGY_RUBRIC_REPO, _RBGY_GIT_COMMIT, _RBGY_INSCRIBE_TIMESTAMP filled now;
  # _RBGY_RUBRIC_COMMIT filled after rubric repo commit (needs content hash)
  buc_step "Filling pre-commit placeholders in clone copies"
  local z_rubric_json=""
  local z_filled_file=""
  local z_inscribe_ts="c${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  for ((z_idx=0; z_idx<${#z_vessel_dirs[@]}; z_idx++)); do
    z_sigil="${z_vessel_sigils[z_idx]}"
    z_rubric_json="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}/cloudbuild.json"
    z_filled_file="${ZRBF_INSCRIBE_PREFIX}${z_sigil}_precommit_filled.json"

    jq --arg repo "${z_rubric_url_clean}" \
       --arg git_commit "${z_git_commit}" \
       --arg inscribe_ts "${z_inscribe_ts}" \
      '.substitutions._RBGY_RUBRIC_REPO = $repo
       | .substitutions._RBGY_GIT_COMMIT = $git_commit
       | .substitutions._RBGY_INSCRIBE_TIMESTAMP = $inscribe_ts
       | .substitutions._RBGA_GIT_COMMIT = $git_commit
       | .substitutions._RBGA_INSCRIBE_TIMESTAMP = $inscribe_ts
       | .images = [.images[] | gsub("__INSCRIBE_TIMESTAMP__"; $inscribe_ts)]' \
      "${z_rubric_json}" > "${z_filled_file}" \
      || buc_die "Failed to fill pre-commit placeholders for ${z_sigil}"
    mv "${z_filled_file}" "${z_rubric_json}" \
      || buc_die "Failed to write pre-commit-filled JSON for ${z_sigil}"
  done

  # Commit (not pushed yet) to get a content hash
  buc_step "Committing rubric repo changes"
  local z_inscribe_ts
  z_inscribe_ts="c${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"

  git -C "${ZRBF_INSCRIBE_CLONE_DIR}" add -A \
    || buc_die "Failed to stage rubric repo changes"

  if git -C "${ZRBF_INSCRIBE_CLONE_DIR}" diff --cached --quiet; then
    buc_info "Rubric repo already up to date — no changes to commit"
  else
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" \
      commit -m "inscribe: ${z_inscribe_ts} — ${#z_vessel_dirs[@]} vessel(s)" \
      || buc_die "Failed to commit rubric repo"

    # Get content commit hash, then fill commit placeholder
    local z_content_commit
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD > "${ZRBF_SCRATCH_FILE}" \
      || buc_die "Failed to get content commit hash"
    z_content_commit=$(<"${ZRBF_SCRATCH_FILE}")

    buc_step "Filling rubric commit hash in clone copies"
    local z_commit_filled=""
    for ((z_idx=0; z_idx<${#z_vessel_dirs[@]}; z_idx++)); do
      z_sigil="${z_vessel_sigils[z_idx]}"
      z_rubric_json="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}/cloudbuild.json"
      z_commit_filled="${ZRBF_INSCRIBE_PREFIX}${z_sigil}_commit_filled.json"

      jq --arg commit "${z_content_commit}" \
        '.substitutions._RBGY_RUBRIC_COMMIT = $commit' \
        "${z_rubric_json}" > "${z_commit_filled}" \
        || buc_die "Failed to fill commit placeholder for ${z_sigil}"
      mv "${z_commit_filled}" "${z_rubric_json}" \
        || buc_die "Failed to write commit-filled JSON for ${z_sigil}"
      buc_info "Filled metadata for ${z_sigil}: commit=${z_content_commit:0:8}"
    done

    # Amend with filled placeholders, then push
    # Safe: shallow clone not yet pushed; amend modifies only our local commit
    # _RBGY_RUBRIC_COMMIT records the pre-amend content hash (build material is identical)
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" add -A \
      || buc_die "Failed to stage filled rubric JSON"
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" commit --amend --no-edit \
      || buc_die "Failed to amend rubric commit with filled placeholders"
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" push \
      || buc_die "Failed to push rubric repo"

    local z_rubric_commit
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD > "${ZRBF_SCRATCH_FILE}" \
      || buc_die "Failed to get final rubric commit"
    z_rubric_commit=$(<"${ZRBF_SCRATCH_FILE}")
    buc_info "Rubric repo pushed: ${z_rubric_commit:0:8}"
  fi

  # Phase 5: Verify CB v2 infrastructure
  buc_step "Verifying CB v2 repository exists"
  local -r z_cbv2_parent="projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBGD_GCB_REGION}"
  local -r z_cbv2_conn="${z_cbv2_parent}/connections/${RBRR_CBV2_CONNECTION_NAME}"
  local -r z_cbv2_repo_resource="${z_cbv2_conn}/repositories/${RBGC_CBV2_REPOSITORY_ID}"
  local -r z_cbv2_repo_url="${RBGC_API_ROOT_CLOUDBUILD_V2}${RBGC_CLOUDBUILD_V2}/${z_cbv2_repo_resource}"

  rbgu_http_json "GET" "${z_cbv2_repo_url}" "${z_token}" "inscribe_cbv2_repo_check"
  local z_cbv2_repo_code
  z_cbv2_repo_code=$(rbgu_http_code_capture "inscribe_cbv2_repo_check") || z_cbv2_repo_code=""
  test "${z_cbv2_repo_code}" = "200" \
    || buc_die "CB v2 repository '${RBGC_CBV2_REPOSITORY_ID}' not found (HTTP ${z_cbv2_repo_code}) — run depot_levy to establish CB v2 connection"
  buc_info "CB v2 repository verified: ${RBGC_CBV2_REPOSITORY_ID}"

  buc_step "Listing existing triggers (batch pre-fetch)"
  local z_triggers_url="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBGD_GCB_REGION}/triggers"
  local z_trigger_name=""
  local z_trigger_body=""
  local z_trigger_create_infix=""
  local z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  local z_trigger_list_infix="inscribe_trigger_list"
  rbgu_http_json "GET" "${z_triggers_url}" "${z_token}" "${z_trigger_list_infix}"
  rbgu_http_require_ok "List existing triggers" "${z_trigger_list_infix}"

  local z_trigger_names_file="${ZRBF_INSCRIBE_PREFIX}trigger_names.txt"
  local z_trigger_list_json="${ZRBGU_PREFIX}${z_trigger_list_infix}${ZRBGU_POSTFIX_JSON}"
  local z_trigger_jq_stderr="${ZRBF_INSCRIBE_PREFIX}trigger_jq_stderr.txt"
  jq -r '[.triggers[]?.name // empty] | .[]' \
    "${z_trigger_list_json}" \
    > "${z_trigger_names_file}" 2>"${z_trigger_jq_stderr}" \
    || buc_die "Failed to extract trigger names from list response"

  for z_sigil in "${z_vessel_sigils[@]}"; do
    z_trigger_name="${RBGC_RUBRIC_TRIGGER_PREFIX}${z_sigil}"
    buc_step "Ensuring trigger: ${z_trigger_name}"

    local z_trigger_found=""
    local z_line=""
    while IFS= read -r z_line; do
      test "${z_line}" = "${z_trigger_name}" && { z_trigger_found=1; break; }
    done < "${z_trigger_names_file}"

    test -z "${z_trigger_found}" || {
      buc_info "Trigger already exists: ${z_trigger_name}"
      continue
    }

    buc_step "Creating trigger: ${z_trigger_name}"
    z_trigger_body="${ZRBF_INSCRIBE_PREFIX}trigger_${z_sigil}.json"
    z_trigger_create_url="${z_triggers_url}"

    jq -n \
      --arg name    "${z_trigger_name}" \
      --arg sigil   "${z_sigil}" \
      --arg repo    "${z_cbv2_repo_resource}" \
      --arg sa      "${z_mason_sa}" \
      '{
        name: $name,
        description: ("Recipe Bottle rubric trigger for " + $sigil),
        repositoryEventConfig: {
          repository: $repo,
          push: { branch: "^MANUAL-DISPATCH-ONLY$" }
        },
        filename: ($sigil + "/cloudbuild.json"),
        serviceAccount: $sa
      }' > "${z_trigger_body}" || buc_die "Failed to compose trigger body for ${z_sigil}"

    z_trigger_create_infix="inscribe_trigger_create_${z_sigil}"
    rbgu_http_json "POST" "${z_trigger_create_url}" "${z_token}" \
      "${z_trigger_create_infix}" "${z_trigger_body}"
    rbgu_http_require_ok "Create trigger ${z_trigger_name}" "${z_trigger_create_infix}"

    buc_info "Trigger created: ${z_trigger_name}"
  done

  # Clean up rubric clone
  rm -rf "${ZRBF_INSCRIBE_CLONE_DIR}" || buc_warn "Failed to clean up rubric clone"

  buc_success "Inscribe complete: ${#z_vessel_dirs[@]} vessel(s), timestamp ${z_inscribe_ts}"
}

rbf_abjure() {
  zrbf_sentinel

  local z_consecration="${2:-}"
  local z_force="${3:-}"

  # Documentation block
  buc_doc_brief "Abjure a consecration (delete all per-platform image, about, and vouch artifacts)"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load
  zrbf_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBF_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbf_load_vessel "${z_vessel_dir}"

  # Validate remaining parameters
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  # Derive inscribe timestamp from full consecration (needed for -multi intermediate tag)
  local -r z_inscribe_ts="${z_consecration%%-r*}"
  test -n "${z_inscribe_ts}" || buc_die "Failed to derive inscribe timestamp from consecration"

  # Check for --force flag
  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Build list of image tags to check/delete
  local z_image_tags=()
  if test "${RBRV_VESSEL_MODE:-conjure}" = "bind"; then
    # Bind vessels have a single multi-arch manifest list tag (no per-platform suffixes)
    z_image_tags+=("${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}")
  else
    # Conjure vessels have per-platform suffixed tags + consumer-facing + intermediate
    local z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"
    local z_platform_suffixes=()
    local z_remaining_plats="${z_platforms}"
    local z_plat=""
    local z_suffix=""
    while test -n "${z_remaining_plats}"; do
      z_plat="${z_remaining_plats%%,*}"
      z_suffix="${z_plat#linux/}"
      z_suffix="${z_suffix//\//}"
      z_platform_suffixes+=("-${z_suffix}")
      test "${z_remaining_plats}" != "${z_plat}" || break
      z_remaining_plats="${z_remaining_plats#*,}"
    done

    local z_idx=0
    for z_idx in "${!z_platform_suffixes[@]}"; do
      z_image_tags+=("${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}${z_platform_suffixes[$z_idx]}")
    done
    if test "${#z_platform_suffixes[@]}" -gt 1; then
      z_image_tags+=("${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}")
      z_image_tags+=("${z_inscribe_ts}-multi")
    fi
  fi

  # About, vouch, and diags tags use full consecration
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_diags_tag="${z_consecration}${RBGC_ARK_SUFFIX_DIAGS}"

  buc_step "Verifying ark existence"

  # Check all image tags
  local z_existing_image_tags=()
  local z_img_tag=""
  local z_img_check_idx=0
  for z_img_tag in "${z_image_tags[@]}"; do
    local z_img_status_file="${ZRBF_DELETE_PREFIX}image_${z_img_check_idx}_status.txt"
    local z_img_response_file="${ZRBF_DELETE_PREFIX}image_${z_img_check_idx}_response.json"

    curl --head -s                                     \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
      -H "Authorization: Bearer ${z_token}"           \
      -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
      -w "%{http_code}"                               \
      -o "${z_img_response_file}"                     \
      "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_img_tag}" \
      > "${z_img_status_file}" || buc_die "HEAD request failed for image tag: ${z_img_tag}"

    local z_img_http_code
    z_img_http_code=$(<"${z_img_status_file}")
    test -n "${z_img_http_code}" || buc_die "HTTP status code is empty for image tag: ${z_img_tag}"

    if test "${z_img_http_code}" = "200"; then
      z_existing_image_tags+=("${z_img_tag}")
    elif test "${z_img_http_code}" != "404"; then
      buc_die "Unexpected HTTP status ${z_img_http_code} for image tag: ${z_img_tag}"
    fi
    z_img_check_idx=$((z_img_check_idx + 1))
  done

  # Check if -about artifact exists
  local z_about_status_file="${ZRBF_DELETE_PREFIX}about_status.txt"
  local z_about_response_file="${ZRBF_DELETE_PREFIX}about_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_about_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_about_status_file}" || buc_die "HEAD request failed for -about artifact"

  local z_about_http_code
  z_about_http_code=$(<"${z_about_status_file}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"

  local z_about_exists=false
  if test "${z_about_http_code}" = "200"; then
    z_about_exists=true
  elif test "${z_about_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_about_http_code} when checking -about artifact"
  fi

  # Check if -vouch artifact exists (optional — older arks won't have one)
  local z_vouch_status_file="${ZRBF_DELETE_PREFIX}vouch_status.txt"
  local z_vouch_response_file="${ZRBF_DELETE_PREFIX}vouch_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_vouch_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
    > "${z_vouch_status_file}" || buc_die "HEAD request failed for -vouch artifact"

  local z_vouch_http_code
  z_vouch_http_code=$(<"${z_vouch_status_file}")
  test -n "${z_vouch_http_code}" || buc_die "HTTP status code is empty for -vouch"

  local z_vouch_exists=false
  if test "${z_vouch_http_code}" = "200"; then
    z_vouch_exists=true
  elif test "${z_vouch_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_vouch_http_code} when checking -vouch artifact"
  fi

  # Check if -diags artifact exists (optional — conjure-only, absent for bind/graft)
  local z_diags_status_file="${ZRBF_DELETE_PREFIX}diags_status.txt"
  local z_diags_response_file="${ZRBF_DELETE_PREFIX}diags_response.json"

  curl --head -s                                     \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}"           \
    -H "Authorization: Bearer ${z_token}"           \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}"     \
    -w "%{http_code}"                               \
    -o "${z_diags_response_file}"                   \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_diags_tag}" \
    > "${z_diags_status_file}" || buc_die "HEAD request failed for -diags artifact"

  local z_diags_http_code
  z_diags_http_code=$(<"${z_diags_status_file}")
  test -n "${z_diags_http_code}" || buc_die "HTTP status code is empty for -diags"

  local z_diags_exists=false
  if test "${z_diags_http_code}" = "200"; then
    z_diags_exists=true
  elif test "${z_diags_http_code}" != "404"; then
    buc_die "Unexpected HTTP status ${z_diags_http_code} when checking -diags artifact"
  fi

  # Evaluate ark state
  if test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "false"; then
    buc_die "Consecration not found: no image tags and no -about exists for ${RBRV_SIGIL}/${z_consecration}"
  fi

  if test "${#z_existing_image_tags[@]}" -gt 0 && test "${z_about_exists}" = "false"; then
    buc_warn "Orphaned artifact detected: image tags exist but -about is missing"
  elif test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "true"; then
    buc_warn "Orphaned artifact detected: -about exists but no image tags found"
  fi

  # Confirm abjuration unless --force
  if test "${z_skip_confirm}" = "false"; then
    local z_confirm_msg="Will abjure ark ${RBRV_SIGIL}/${z_consecration}:"
    if (( ${#z_existing_image_tags[@]} )); then
      for z_img_tag in "${z_existing_image_tags[@]}"; do
        z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_img_tag}"
      done
    fi
    if test "${z_about_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_about_tag}"
    fi
    if test "${z_vouch_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_vouch_tag}"
    fi
    if test "${z_diags_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_diags_tag}"
    fi
    buc_require "${z_confirm_msg}" "yes"
  fi

  # Delete all existing image tags
  if (( ${#z_existing_image_tags[@]} )); then
    local z_img_del_idx=0
    for z_img_tag in "${z_existing_image_tags[@]}"; do
      buc_step "Deleting image tag: ${z_img_tag}"

      local z_delete_img_status="${ZRBF_DELETE_PREFIX}delete_image_${z_img_del_idx}_status.txt"
      local z_delete_img_response="${ZRBF_DELETE_PREFIX}delete_image_${z_img_del_idx}_response.json"

      curl -X DELETE -s                                   \
        --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
        --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
        -H "Authorization: Bearer ${z_token}"             \
        -w "%{http_code}"                                 \
        -o "${z_delete_img_response}"                     \
        "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_img_tag}" \
        > "${z_delete_img_status}" || buc_die "DELETE request failed for image tag: ${z_img_tag}"

      local z_delete_img_code
      z_delete_img_code=$(<"${z_delete_img_status}")
      test -n "${z_delete_img_code}" || buc_die "HTTP status code is empty for image tag delete: ${z_img_tag}"

      if test "${z_delete_img_code}" != "202" && test "${z_delete_img_code}" != "204"; then
        local z_body="empty"
        if test -f "${z_delete_img_response}"; then z_body=$(<"${z_delete_img_response}"); fi
        buc_warn "Response body: ${z_body}"
        buc_die "Failed to delete image tag ${z_img_tag} (HTTP ${z_delete_img_code})"
      fi

      buc_info "Deleted: ${RBRV_SIGIL}:${z_img_tag}"
      z_img_del_idx=$((z_img_del_idx + 1))
    done
  fi

  # Delete -about artifact if exists
  if test "${z_about_exists}" = "true"; then
    buc_step "Deleting -about artifact"

    local z_delete_about_status="${ZRBF_DELETE_PREFIX}delete_about_status.txt"
    local z_delete_about_response="${ZRBF_DELETE_PREFIX}delete_about_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_about_response}"                   \
      "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
      > "${z_delete_about_status}" || buc_die "DELETE request failed for -about"

    local z_delete_about_code
    z_delete_about_code=$(<"${z_delete_about_status}")
    test -n "${z_delete_about_code}" || buc_die "HTTP status code is empty for -about delete"

    if test "${z_delete_about_code}" != "202" && test "${z_delete_about_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_about_response}"; then z_body=$(<"${z_delete_about_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -about artifact (HTTP ${z_delete_about_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_about_tag}"
  fi

  # Delete -vouch artifact if exists (optional — older arks won't have one)
  if test "${z_vouch_exists}" = "true"; then
    buc_step "Deleting -vouch artifact"

    local z_delete_vouch_status="${ZRBF_DELETE_PREFIX}delete_vouch_status.txt"
    local z_delete_vouch_response="${ZRBF_DELETE_PREFIX}delete_vouch_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_vouch_response}"                   \
      "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
      > "${z_delete_vouch_status}" || buc_die "DELETE request failed for -vouch"

    local z_delete_vouch_code
    z_delete_vouch_code=$(<"${z_delete_vouch_status}")
    test -n "${z_delete_vouch_code}" || buc_die "HTTP status code is empty for -vouch delete"

    if test "${z_delete_vouch_code}" != "202" && test "${z_delete_vouch_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_vouch_response}"; then z_body=$(<"${z_delete_vouch_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -vouch artifact (HTTP ${z_delete_vouch_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_vouch_tag}"
  fi

  # Delete -diags artifact if exists (optional — conjure-only)
  if test "${z_diags_exists}" = "true"; then
    buc_step "Deleting -diags artifact"

    local z_delete_diags_status="${ZRBF_DELETE_PREFIX}delete_diags_status.txt"
    local z_delete_diags_response="${ZRBF_DELETE_PREFIX}delete_diags_response.json"

    curl -X DELETE -s                                   \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}"             \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_diags_response}"                   \
      "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_diags_tag}" \
      > "${z_delete_diags_status}" || buc_die "DELETE request failed for -diags"

    local z_delete_diags_code
    z_delete_diags_code=$(<"${z_delete_diags_status}")
    test -n "${z_delete_diags_code}" || buc_die "HTTP status code is empty for -diags delete"

    if test "${z_delete_diags_code}" != "202" && test "${z_delete_diags_code}" != "204"; then
      local z_body="empty"
      if test -f "${z_delete_diags_response}"; then z_body=$(<"${z_delete_diags_response}"); fi
      buc_warn "Response body: ${z_body}"
      buc_die "Failed to delete -diags artifact (HTTP ${z_delete_diags_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_diags_tag}"
  fi

  # Display results
  echo ""
  buc_success "Consecration abjured: ${RBRV_SIGIL}/${z_consecration}"
  if (( ${#z_existing_image_tags[@]} )); then
    for z_img_tag in "${z_existing_image_tags[@]}"; do
      echo "  - ${RBRV_SIGIL}:${z_img_tag} deleted"
    done
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_about_tag} deleted"
  fi
  if test "${z_vouch_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_vouch_tag} deleted"
  fi
  if test "${z_diags_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_diags_tag} deleted"
  fi
}

######################################################################
# Vouch Gate (consumer-side vouch verification)
#
# Standalone — does not require foundry kindle; uses regime-level constants only.
# Callers: rbob auto-summon preflight, test case vouch gate verification.

rbf_vouch_gate() {
  local -r z_vessel="${1:-}"
  local -r z_consecration="${2:-}"

  test -n "${z_vessel}"       || buc_die "rbf_vouch_gate: vessel required"
  test -n "${z_consecration}" || buc_die "rbf_vouch_gate: consecration required"

  local -r z_registry_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_registry_api_base="https://${z_registry_host}/v2/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"

  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  buc_step "Vouch gate: checking ${z_vessel}:${z_vouch_tag}"

  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "rbf_vouch_gate: failed to get Director OAuth token"

  local z_vouch_http_code
  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -o /dev/null \
    -w "%{http_code}" \
    "${z_registry_api_base}/${z_vessel}/manifests/${z_vouch_tag}" \
    > "${ZRBF_SCRATCH_FILE}" \
    || buc_die "rbf_vouch_gate: HEAD request failed for ${z_vessel}:${z_vouch_tag}"
  z_vouch_http_code=$(<"${ZRBF_SCRATCH_FILE}")

  if test "${z_vouch_http_code}" != "200"; then
    buc_die "Consecration not vouched: ${z_vessel}:${z_consecration} (HTTP ${z_vouch_http_code} — refusing to use unvouched image)"
  fi

  buc_info "Vouch verified: ${z_vessel}:${z_vouch_tag}"
}

######################################################################
# About (rbw-Db)

rbf_about() {
  zrbf_sentinel

  local -r z_consecration="${2:-}"
  local -r z_conjure_build_id="${3:-}"  # Optional: conjure BUILD_ID for provenance

  buc_doc_brief "Assemble about metadata artifact for an existing consecration image"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_param "conjure_build_id" "(Optional) Cloud Build job ID from conjure"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) and load
  zrbf_resolve_vessel "${1:-}"
  local -r z_vessel_dir=$(<"${ZRBF_VESSEL_RESOLVED_DIR_FILE}")
  test -n "${z_vessel_dir}" || buc_die "Empty resolved vessel path"
  zrbf_load_vessel "${z_vessel_dir}"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Gate: require -image exists
  buc_step "Gating on image artifact existence"
  local -r z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_image_gate_status="${ZRBF_ABOUT_PREFIX}image_status.txt"
  local -r z_image_gate_response="${ZRBF_ABOUT_PREFIX}image_response.json"
  local -r z_image_gate_stderr="${ZRBF_ABOUT_PREFIX}image_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_image_gate_response}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_image_tag}" \
    > "${z_image_gate_status}" 2>"${z_image_gate_stderr}" \
    || buc_die "HEAD request failed for -image artifact — see ${z_image_gate_stderr}"

  local -r z_image_http_code=$(<"${z_image_gate_status}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"
  test "${z_image_http_code}" = "200" \
    || buc_die "Image artifact not found (HTTP ${z_image_http_code}) — image must exist before about"

  buc_info "Image artifact confirmed: ${z_image_tag}"

  # Gate: warn if -about already exists (re-about is idempotent overwrite)
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_about_gate_status="${ZRBF_ABOUT_PREFIX}about_status.txt"
  local -r z_about_gate_response="${ZRBF_ABOUT_PREFIX}about_response.json"
  local -r z_about_gate_stderr="${ZRBF_ABOUT_PREFIX}about_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_about_gate_response}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_about_gate_status}" 2>"${z_about_gate_stderr}" \
    || buc_die "HEAD request failed for -about artifact — see ${z_about_gate_stderr}"

  local -r z_about_http_code=$(<"${z_about_gate_status}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"
  if test "${z_about_http_code}" = "200"; then
    buc_warn "Re-about in progress: ${z_about_tag} already exists"
  fi

  # Submit about Cloud Build
  zrbf_about_submit "${z_consecration}" "${z_token}" "${z_conjure_build_id}"

  buc_success "About complete: ${RBRV_SIGIL}/${z_consecration}"
  buc_info "About artifact: ${RBRV_SIGIL}:${z_about_tag}"
}

# Internal: capture git metadata to module temp files (idempotent)
# No args — reads from git, writes to ZRBF_GIT_*_FILE kindle constants
zrbf_ensure_git_metadata() {
  zrbf_sentinel

  # Idempotent — skip if already captured
  test ! -s "${ZRBF_GIT_COMMIT_FILE}" || return 0

  buc_log_args "Capturing git metadata to temp files"

  local -r z_remote_file="${ZRBF_GIT_PREFIX}remote.txt"
  local -r z_url_file="${ZRBF_GIT_PREFIX}url.txt"

  git rev-parse HEAD > "${ZRBF_GIT_COMMIT_FILE}" \
    || buc_die "Failed to get git commit"
  test -s "${ZRBF_GIT_COMMIT_FILE}" || buc_die "Empty git commit file"

  git rev-parse --abbrev-ref HEAD > "${ZRBF_GIT_BRANCH_FILE}" \
    || buc_die "Failed to get git branch"
  test -s "${ZRBF_GIT_BRANCH_FILE}" || buc_die "Empty git branch file"

  git remote > "${z_remote_file}" \
    || buc_die "Failed to list git remotes"
  local z_remote=""
  read -r z_remote < "${z_remote_file}" \
    || buc_die "Failed to read git remote from ${z_remote_file}"
  test -n "${z_remote}" || buc_die "No git remotes found"

  git config --get "remote.${z_remote}.url" > "${z_url_file}" \
    || buc_die "Failed to get git repo URL"

  local z_url=""
  z_url=$(<"${z_url_file}")
  test -n "${z_url}" || buc_die "Empty git repo URL from ${z_url_file}"
  local z_repo="${z_url#*://*/}"
  z_repo="${z_repo%.git}"
  echo "${z_repo}" > "${ZRBF_GIT_REPO_FILE}" \
    || buc_die "Failed to write derived git repo"
}

# Internal: assemble about step scripts into JSON array file
# Args: output_file temp_prefix
# Reads ZRBF_RBGJA_STEPS_DIR and ZRBF_TOOL_* image refs from module state
zrbf_assemble_about_steps() {
  zrbf_sentinel

  local -r z_output_file="$1"
  local -r z_temp_prefix="$2"

  # Step definitions: script|builder|entrypoint|id
  # Delimiter is | because image refs contain colons (sha256 digests)
  local -r z_about_step_defs=(
    "rbgja01-discover-platforms.py|${ZRBF_TOOL_GCLOUD}|python3|discover-platforms"
    "rbgja02-syft-per-platform.sh|${ZRBF_TOOL_DOCKER}|bash|syft-per-platform"
    "rbgja03-build-info-per-platform.py|${ZRBF_TOOL_GCLOUD}|python3|build-info-per-platform"
    "rbgja04-assemble-push-about.sh|${ZRBF_TOOL_DOCKER}|bash|assemble-push-about"
  )

  echo "[]" > "${z_output_file}" || buc_die "Failed to initialize about steps JSON"

  local z_adef=""
  local z_ascript=""
  local z_abuilder=""
  local z_aentrypoint=""
  local z_aid=""
  local z_ascript_path=""
  local z_abody_file=""
  local z_aescaped_file=""
  local z_asteps_file=""
  local z_abody=""
  local z_ashebang=""

  for z_adef in "${z_about_step_defs[@]}"; do
    IFS='|' read -r z_ascript z_abuilder z_aentrypoint z_aid <<< "${z_adef}"
    z_ascript_path="${ZRBF_RBGJA_STEPS_DIR}/${z_ascript}"
    z_abody_file="${z_temp_prefix}${z_aid}_body.txt"
    z_aescaped_file="${z_temp_prefix}${z_aid}_escaped.txt"
    z_asteps_file="${z_temp_prefix}${z_aid}_steps.json"

    test -f "${z_ascript_path}" || buc_die "About step script not found: ${z_ascript_path}"

    buc_log_args "Reading script body for ${z_aid} (skip shebang)"
    tail -n +2 "${z_ascript_path}" > "${z_abody_file}" \
      || buc_die "Failed to read about step script: ${z_ascript_path}"
    z_abody=$(<"${z_abody_file}")
    test -n "${z_abody}" || buc_die "Empty about script body: ${z_ascript_path}"

    buc_log_args "Baking pinned image refs into script text"
    z_abody="${z_abody//\$\{ZRBF_TOOL_SYFT\}/${ZRBF_TOOL_SYFT}}"

    case "${z_aentrypoint}" in
      bash)    z_ashebang="#!/bin/bash" ;;
      sh)      z_ashebang="#!/bin/sh" ;;
      python3) z_ashebang="#!/usr/bin/env python3" ;;
      *)       buc_die "Unknown entrypoint: ${z_aentrypoint}" ;;
    esac
    printf '%s\n%s' "${z_ashebang}" "${z_abody}" > "${z_aescaped_file}" \
      || buc_die "Failed to write about script body for ${z_aid}"

    buc_log_args "Appending about step ${z_aid} to JSON array"
    jq \
      --arg name "${z_abuilder}" \
      --arg id "${z_aid}" \
      --rawfile script "${z_aescaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_output_file}" > "${z_asteps_file}" \
      || buc_die "Failed to append about step ${z_aid} to JSON"
    mv "${z_asteps_file}" "${z_output_file}" \
      || buc_die "Failed to update about steps JSON for ${z_aid}"
  done
}

# Internal: assemble vouch step scripts into JSON array file
# Args: output_file temp_prefix
# Reads ZRBF_RBGJV_STEPS_DIR and ZRBF_TOOL_* image refs from module state
zrbf_assemble_vouch_steps() {
  zrbf_sentinel

  local -r z_output_file="$1"
  local -r z_temp_prefix="$2"

  # Step definitions: script|builder|entrypoint|id
  # Delimiter is | because image refs contain colons (sha256 digests)
  local -r z_vouch_step_defs=(
    "rbgjv01-download-verifier.sh|${ZRBF_TOOL_ALPINE}|sh|prepare-keys"
    "rbgjv02-verify-provenance.py|${ZRBF_TOOL_GCLOUD}|python3|verify-provenance"
    "rbgjv03-assemble-push-vouch.sh|${ZRBF_TOOL_DOCKER}|bash|assemble-push-vouch"
  )

  echo "[]" > "${z_output_file}" || buc_die "Failed to initialize vouch steps JSON"

  local z_vdef=""
  local z_vscript=""
  local z_vbuilder=""
  local z_ventrypoint=""
  local z_vid=""
  local z_vscript_path=""
  local z_vbody_file=""
  local z_vescaped_file=""
  local z_vsteps_file=""
  local z_vbody=""
  local z_vshebang=""

  for z_vdef in "${z_vouch_step_defs[@]}"; do
    IFS='|' read -r z_vscript z_vbuilder z_ventrypoint z_vid <<< "${z_vdef}"
    z_vscript_path="${ZRBF_RBGJV_STEPS_DIR}/${z_vscript}"
    z_vbody_file="${z_temp_prefix}${z_vid}_body.txt"
    z_vescaped_file="${z_temp_prefix}${z_vid}_escaped.txt"
    z_vsteps_file="${z_temp_prefix}${z_vid}_steps.json"

    test -f "${z_vscript_path}" || buc_die "Vouch step script not found: ${z_vscript_path}"

    buc_log_args "Reading script body for ${z_vid} (skip shebang)"
    tail -n +2 "${z_vscript_path}" > "${z_vbody_file}" \
      || buc_die "Failed to read vouch step script: ${z_vscript_path}"
    z_vbody=$(<"${z_vbody_file}")
    test -n "${z_vbody}" || buc_die "Empty vouch script body: ${z_vscript_path}"

    case "${z_ventrypoint}" in
      bash)    z_vshebang="#!/bin/bash" ;;
      sh)      z_vshebang="#!/bin/sh" ;;
      python3) z_vshebang="#!/usr/bin/env python3" ;;
      *)       buc_die "Unknown entrypoint: ${z_ventrypoint}" ;;
    esac
    printf '%s\n%s' "${z_vshebang}" "${z_vbody}" > "${z_vescaped_file}" \
      || buc_die "Failed to write vouch script body for ${z_vid}"

    buc_log_args "Appending vouch step ${z_vid} to JSON array"
    jq \
      --arg name "${z_vbuilder}" \
      --arg id "${z_vid}" \
      --rawfile script "${z_vescaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_output_file}" > "${z_vsteps_file}" \
      || buc_die "Failed to append vouch step ${z_vid} to JSON"
    mv "${z_vsteps_file}" "${z_output_file}" \
      || buc_die "Failed to update vouch steps JSON for ${z_vid}"
  done
}

# Internal: submit combined about+vouch Cloud Build job for graft mode.
# Eliminates the orphan gap between standalone about and vouch by running
# both step sets in a single GCB submission.
# Args: vessel_dir consecration
zrbf_graft_metadata_submit() {
  zrbf_sentinel

  local -r z_vessel_dir="$1"
  local -r z_consecration="$2"

  # Load vessel (follows reload pattern used by rbf_about/rbf_vouch)
  zrbf_load_vessel "${z_vessel_dir}"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Constructing combined about+vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Gate: require -image exists (graft push must have completed)
  buc_step "Gating on image artifact existence"
  local -r z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_image_gate_status="${ZRBF_GRAFT_PREFIX}meta_image_status.txt"
  local -r z_image_gate_response="${ZRBF_GRAFT_PREFIX}meta_image_response.json"
  local -r z_image_gate_stderr="${ZRBF_GRAFT_PREFIX}meta_image_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_image_gate_response}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_image_tag}" \
    > "${z_image_gate_status}" 2>"${z_image_gate_stderr}" \
    || buc_die "HEAD request failed for -image artifact — see ${z_image_gate_stderr}"

  local -r z_image_http_code=$(<"${z_image_gate_status}")
  test -n "${z_image_http_code}" || buc_die "HTTP status code is empty for -image"
  test "${z_image_http_code}" = "200" \
    || buc_die "Image artifact not found (HTTP ${z_image_http_code}) — graft push must complete before about+vouch"

  buc_info "Image artifact confirmed: ${z_image_tag}"

  # Git metadata (shared temp files, idempotent)
  zrbf_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")

  # Graft-specific about substitution values
  local -r z_graft_source="${RBRV_GRAFT_IMAGE:-}"
  local z_dockerfile_content=""
  local -r z_dockerfile_max_bytes=4000
  if test -n "${RBRV_GRAFT_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}"; then
    local -r z_df_size_file="${ZRBF_GRAFT_PREFIX}meta_df_size.txt"
    wc -c < "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
      || buc_die "Failed to measure Dockerfile size"
    local z_df_size=""
    z_df_size=$(<"${z_df_size_file}")
    z_df_size="${z_df_size// /}"
    if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
      z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
    else
      buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
    fi
  fi

  # === Assemble about steps ===
  local -r z_about_steps_file="${ZRBF_GRAFT_PREFIX}meta_about_steps.json"
  zrbf_assemble_about_steps "${z_about_steps_file}" "${ZRBF_GRAFT_PREFIX}meta_about_"

  # === Resolve base image provenance (for vouch summary) ===
  local -r z_vi_gar_prefix="${z_gar_host}/${z_gar_path}/${RBRV_SIGIL}"
  local z_vi_ref_1="" z_vi_ref_2="" z_vi_ref_3=""
  local z_vi_prov_1="" z_vi_prov_2="" z_vi_prov_3=""
  local z_vi_n="" z_vi_origin_var="" z_vi_anchor_var="" z_vi_origin="" z_vi_anchor=""
  for z_vi_n in 1 2 3; do
    z_vi_origin_var="RBRV_IMAGE_${z_vi_n}_ORIGIN"
    z_vi_anchor_var="RBRV_IMAGE_${z_vi_n}_ANCHOR"
    z_vi_origin="${!z_vi_origin_var:-}"
    z_vi_anchor="${!z_vi_anchor_var:-}"
    test -n "${z_vi_origin}" || continue
    local z_vi_ref="" z_vi_prov=""
    if test -n "${z_vi_anchor}"; then
      z_vi_ref="${z_vi_gar_prefix}:${z_vi_anchor}"
      z_vi_prov="anchored"
    else
      z_vi_ref="${z_vi_origin}"
      z_vi_prov="pass-through"
    fi
    case "${z_vi_n}" in
      1) z_vi_ref_1="${z_vi_ref}"; z_vi_prov_1="${z_vi_prov}" ;;
      2) z_vi_ref_2="${z_vi_ref}"; z_vi_prov_2="${z_vi_prov}" ;;
      3) z_vi_ref_3="${z_vi_ref}"; z_vi_prov_3="${z_vi_prov}" ;;
    esac
  done

  # === Assemble vouch steps ===
  local -r z_vouch_steps_file="${ZRBF_GRAFT_PREFIX}meta_vouch_steps.json"
  zrbf_assemble_vouch_steps "${z_vouch_steps_file}" "${ZRBF_GRAFT_PREFIX}meta_vouch_"

  # === Combine: about steps + vouch steps ===
  local -r z_combined_steps="${ZRBF_GRAFT_PREFIX}meta_combined_steps.json"
  jq -s '.[0] + .[1]' "${z_about_steps_file}" "${z_vouch_steps_file}" \
    > "${z_combined_steps}" || buc_die "Failed to combine about and vouch steps"

  # Compose Build resource JSON with both _RBGA_ and _RBGV_ substitutions
  buc_log_args "Composing combined about+vouch Build resource JSON"
  local -r z_build_file="${ZRBF_GRAFT_PREFIX}meta_build.json"

  jq -n \
    --slurpfile zjq_steps       "${z_combined_steps}" \
    --arg zjq_sa                "${z_mason_sa}" \
    --arg zjq_gar_host          "${z_gar_host}" \
    --arg zjq_gar_path          "${z_gar_path}" \
    --arg zjq_vessel            "${RBRV_SIGIL}" \
    --arg zjq_consecration      "${z_consecration}" \
    --arg zjq_git_commit        "${z_git_commit}" \
    --arg zjq_git_branch        "${z_git_branch}" \
    --arg zjq_git_repo          "${z_git_repo}" \
    --arg zjq_graft_source      "${z_graft_source}" \
    --arg zjq_dockerfile        "${z_dockerfile_content}" \
    --arg zjq_ark_suffix_image  "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about  "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_ark_suffix_vouch  "${RBGC_ARK_SUFFIX_VOUCH}" \
    --arg zjq_ark_suffix_diags  "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_vi_ref_1          "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1         "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2          "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2         "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3          "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3         "${z_vi_prov_3}" \
    --arg zjq_pool              "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_CONSECRATION:          $zjq_consecration,
        _RBGA_VESSEL_MODE:           "graft",
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              "",
        _RBGA_INSCRIBE_TIMESTAMP:    "",
        _RBGA_BIND_SOURCE:           "",
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:      $zjq_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:      $zjq_ark_suffix_diags,
        _RBGV_GAR_HOST:              $zjq_gar_host,
        _RBGV_GAR_PATH:              $zjq_gar_path,
        _RBGV_VESSEL:                $zjq_vessel,
        _RBGV_CONSECRATION:          $zjq_consecration,
        _RBGV_VESSEL_MODE:           "graft",
        _RBGV_BIND_SOURCE:           "",
        _RBGV_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGV_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGV_ARK_SUFFIX_VOUCH:      $zjq_ark_suffix_vouch,
        _RBGV_IMAGE_1:               $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE:    $zjq_vi_prov_1,
        _RBGV_IMAGE_2:               $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE:    $zjq_vi_prov_2,
        _RBGV_IMAGE_3:               $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE:    $zjq_vi_prov_3
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose combined about+vouch build JSON"

  buc_log_args "Combined about+vouch build JSON: ${z_build_file}"

  buc_step "Submitting combined about+vouch Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "graft_meta_build_create" "${z_build_file}"
  rbgu_http_require_ok "Combined about+vouch build submission" "graft_meta_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "graft_meta_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Combined about+vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 100 "About+Vouch"  # ~8 minutes at 5s intervals

  buc_success "About+Vouch complete: ${RBRV_SIGIL}/${z_consecration}"
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  buc_info "About artifact: ${RBRV_SIGIL}:${z_about_tag}"
  buc_info "Vouch artifact: ${RBRV_SIGIL}:${z_vouch_tag}"
}

# Internal: submit about Cloud Build job and wait for completion
zrbf_about_submit() {
  zrbf_sentinel

  local -r z_consecration="$1"
  local -r z_token="$2"
  local -r z_conjure_build_id="${3:-}"

  buc_step "Constructing about Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Determine mode-specific substitution values
  local z_vessel_mode="${RBRV_VESSEL_MODE}"
  local z_bind_source=""
  local z_graft_source=""
  local z_inscribe_ts=""
  local z_dockerfile_content=""
  # Cloud Build substitution values are limited to 4096 bytes. We use 4000 as a
  # conservative guard to account for encoding overhead and avoid edge-case failures.
  local -r z_dockerfile_max_bytes=4000
  local -r z_df_size_file="${ZRBF_ABOUT_PREFIX}df_size.txt"

  case "${z_vessel_mode}" in
    conjure)
      # Extract inscribe timestamp from consecration (e.g., c260305133650 from c260305133650-r260305160530)
      z_inscribe_ts="${z_consecration%%-r*}"
      # Read Dockerfile content for recipe.txt
      if test -f "${RBRV_CONJURE_DOCKERFILE:-}"; then
        wc -c < "${RBRV_CONJURE_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_CONJURE_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    bind)
      z_bind_source="${RBRV_BIND_IMAGE:-}"
      if test -n "${RBRV_BIND_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_BIND_OPTIONAL_DOCKERFILE}"; then
        wc -c < "${RBRV_BIND_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_BIND_OPTIONAL_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    graft)
      z_graft_source="${RBRV_GRAFT_IMAGE:-}"
      if test -n "${RBRV_GRAFT_OPTIONAL_DOCKERFILE:-}" && test -f "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}"; then
        wc -c < "${RBRV_GRAFT_OPTIONAL_DOCKERFILE}" > "${z_df_size_file}" \
          || buc_die "Failed to measure Dockerfile size"
        local z_df_size=""
        z_df_size=$(<"${z_df_size_file}")
        z_df_size="${z_df_size// /}"  # Strip spaces (wc output varies by platform)
        if test "${z_df_size}" -le "${z_dockerfile_max_bytes}"; then
          z_dockerfile_content=$(<"${RBRV_GRAFT_OPTIONAL_DOCKERFILE}")
        else
          buc_warn "Dockerfile exceeds 4KB substitution limit (${z_df_size} bytes) — recipe.txt omitted"
        fi
      fi
      ;;
    *)
      buc_die "Unknown vessel mode: ${z_vessel_mode}"
      ;;
  esac

  # Git metadata (shared temp files, idempotent)
  zrbf_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBF_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBF_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBF_GIT_REPO_FILE}")

  # Assemble about steps via shared helper
  local -r z_about_steps_accumulator="${ZRBF_ABOUT_PREFIX}steps.json"
  zrbf_assemble_about_steps "${z_about_steps_accumulator}" "${ZRBF_ABOUT_PREFIX}"

  buc_log_args "Composing about Build resource JSON"
  local -r z_about_build_file="${ZRBF_ABOUT_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_about_steps_accumulator}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_vessel       "${RBRV_SIGIL}" \
    --arg zjq_consecration "${z_consecration}" \
    --arg zjq_vessel_mode  "${z_vessel_mode}" \
    --arg zjq_git_commit   "${z_git_commit}" \
    --arg zjq_git_branch   "${z_git_branch}" \
    --arg zjq_git_repo     "${z_git_repo}" \
    --arg zjq_build_id     "${z_conjure_build_id}" \
    --arg zjq_inscribe_ts  "${z_inscribe_ts}" \
    --arg zjq_bind_source  "${z_bind_source}" \
    --arg zjq_graft_source "${z_graft_source}" \
    --arg zjq_dockerfile   "${z_dockerfile_content}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_ark_suffix_diags "${RBGC_ARK_SUFFIX_DIAGS}" \
    --arg zjq_pool         "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout      "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGA_GAR_HOST:              $zjq_gar_host,
        _RBGA_GAR_PATH:              $zjq_gar_path,
        _RBGA_VESSEL:                $zjq_vessel,
        _RBGA_CONSECRATION:          $zjq_consecration,
        _RBGA_VESSEL_MODE:           $zjq_vessel_mode,
        _RBGA_GIT_COMMIT:            $zjq_git_commit,
        _RBGA_GIT_BRANCH:            $zjq_git_branch,
        _RBGA_GIT_REPO:              $zjq_git_repo,
        _RBGA_BUILD_ID:              $zjq_build_id,
        _RBGA_INSCRIBE_TIMESTAMP:    $zjq_inscribe_ts,
        _RBGA_BIND_SOURCE:           $zjq_bind_source,
        _RBGA_GRAFT_SOURCE:          $zjq_graft_source,
        _RBGA_DOCKERFILE_CONTENT:    $zjq_dockerfile,
        _RBGA_ARK_SUFFIX_IMAGE:      $zjq_ark_suffix_image,
        _RBGA_ARK_SUFFIX_ABOUT:      $zjq_ark_suffix_about,
        _RBGA_ARK_SUFFIX_DIAGS:      $zjq_ark_suffix_diags
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_about_build_file}" \
    || buc_die "Failed to compose about build JSON"

  buc_log_args "About build JSON: ${z_about_build_file}"

  buc_step "Submitting about Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "about_build_create" "${z_about_build_file}"
  rbgu_http_require_ok "About build submission" "about_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "about_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "About build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 50 "About"  # ~4 minutes at 5s intervals (private pool)
}

######################################################################
# Vouch (rbw-DV)

rbf_vouch() {
  zrbf_sentinel

  local -r z_vessel_dir="${1:-}"
  local -r z_consecration="${2:-}"

  buc_doc_brief "Vouch for an ark by mode-aware verification in Cloud Build"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  if test -z "${z_vessel_dir}"; then
    local z_sigils
    z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"
    buc_step "Available vessels:"
    local z_sigil=""
    for z_sigil in ${z_sigils}; do
      buc_bare "        ${RBRR_VESSEL_DIR}/${z_sigil}"
    done
    buc_die "Vessel directory required"
  fi

  zrbf_load_vessel "${z_vessel_dir}"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  # Resolve tool images from reliquary (vouch steps use tool images)
  zrbf_resolve_tool_images

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Gate: require -about exists (about must complete before vouch)
  buc_step "Gating on about artifact existence"
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_about_gate_status="${ZRBF_VOUCH_PREFIX}about_status.txt"
  local -r z_about_gate_response="${ZRBF_VOUCH_PREFIX}about_response.json"
  local -r z_about_gate_stderr="${ZRBF_VOUCH_PREFIX}about_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_about_gate_response}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_about_gate_status}" 2>"${z_about_gate_stderr}" \
    || buc_die "HEAD request failed for -about artifact — see ${z_about_gate_stderr}"

  local -r z_about_http_code=$(<"${z_about_gate_status}")
  test -n "${z_about_http_code}" || buc_die "HTTP status code is empty for -about"
  test "${z_about_http_code}" = "200" \
    || buc_die "About artifact not found (HTTP ${z_about_http_code}) — about must complete before vouch"

  buc_info "About artifact confirmed: ${z_about_tag}"

  # Gate: warn if -vouch already exists (re-vouch)
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_vouch_gate_status="${ZRBF_VOUCH_PREFIX}vouch_status.txt"
  local -r z_vouch_gate_response="${ZRBF_VOUCH_PREFIX}vouch_response.json"
  local -r z_vouch_gate_stderr="${ZRBF_VOUCH_PREFIX}vouch_stderr.txt"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_vouch_gate_response}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
    > "${z_vouch_gate_status}" 2>"${z_vouch_gate_stderr}" \
    || buc_die "HEAD request failed for -vouch artifact — see ${z_vouch_gate_stderr}"

  local -r z_vouch_http_code=$(<"${z_vouch_gate_status}")
  test -n "${z_vouch_http_code}" || buc_die "HTTP status code is empty for -vouch"
  if test "${z_vouch_http_code}" = "200"; then
    buc_warn "Re-vouch in progress: ${z_vouch_tag} already exists"
  fi

  # All modes use Cloud Build for vouch (mode-aware verification inside the build)
  zrbf_vouch_submit "${z_consecration}" "${z_vouch_tag}" "${z_token}"

  buc_success "Vouch complete: ${RBRV_SIGIL}/${z_consecration}"
  buc_info "Vouch artifact: ${RBRV_SIGIL}:${z_vouch_tag}"
}

# Internal: Submit vouch Cloud Build job (mode-aware verification)
# All vessel modes use Cloud Build. The build scripts branch on _RBGV_VESSEL_MODE:
#   conjure: DSSE envelope signature verification (jq + openssl)
#   bind: digest-pin comparison against upstream reference
#   graft: GRAFTED stamp (no verification)
zrbf_vouch_submit() {
  zrbf_sentinel

  local -r z_consecration="$1"
  local -r z_vouch_tag="$2"
  local -r z_token="$3"

  buc_step "Constructing vouch Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Mode-specific substitution values (empty strings for non-applicable modes)
  local z_bind_source=""
  local z_graft_source=""

  case "${RBRV_VESSEL_MODE}" in
    conjure) : ;;  # DSSE verification uses embedded keys, no extra substitutions
    bind)    z_bind_source="${RBRV_BIND_IMAGE:-}" ;;
    graft)   z_graft_source="${RBRV_GRAFT_IMAGE:-}" ;;
    *)       buc_die "Unknown vessel mode: ${RBRV_VESSEL_MODE}" ;;
  esac

  # Resolve base image provenance (for vouch summary recording)
  local -r z_vi_gar_prefix="${z_gar_host}/${z_gar_path}/${RBRV_SIGIL}"
  local z_vi_ref_1="" z_vi_ref_2="" z_vi_ref_3=""
  local z_vi_prov_1="" z_vi_prov_2="" z_vi_prov_3=""
  local z_vi_n="" z_vi_origin_var="" z_vi_anchor_var="" z_vi_origin="" z_vi_anchor=""
  for z_vi_n in 1 2 3; do
    z_vi_origin_var="RBRV_IMAGE_${z_vi_n}_ORIGIN"
    z_vi_anchor_var="RBRV_IMAGE_${z_vi_n}_ANCHOR"
    z_vi_origin="${!z_vi_origin_var:-}"
    z_vi_anchor="${!z_vi_anchor_var:-}"
    test -n "${z_vi_origin}" || continue
    local z_vi_ref="" z_vi_prov=""
    if test -n "${z_vi_anchor}"; then
      z_vi_ref="${z_vi_gar_prefix}:${z_vi_anchor}"
      z_vi_prov="anchored"
    else
      z_vi_ref="${z_vi_origin}"
      z_vi_prov="pass-through"
    fi
    case "${z_vi_n}" in
      1) z_vi_ref_1="${z_vi_ref}"; z_vi_prov_1="${z_vi_prov}" ;;
      2) z_vi_ref_2="${z_vi_ref}"; z_vi_prov_2="${z_vi_prov}" ;;
      3) z_vi_ref_3="${z_vi_ref}"; z_vi_prov_3="${z_vi_prov}" ;;
    esac
  done

  # Assemble vouch steps via shared helper
  local -r z_vouch_steps_accumulator="${ZRBF_VOUCH_PREFIX}steps.json"
  zrbf_assemble_vouch_steps "${z_vouch_steps_accumulator}" "${ZRBF_VOUCH_PREFIX}"

  buc_log_args "Composing vouch Build resource JSON"
  local -r z_vouch_build_file="${ZRBF_VOUCH_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps       "${z_vouch_steps_accumulator}" \
    --arg zjq_sa                "${z_mason_sa}" \
    --arg zjq_gar_host          "${z_gar_host}" \
    --arg zjq_gar_path          "${z_gar_path}" \
    --arg zjq_vessel            "${RBRV_SIGIL}" \
    --arg zjq_consecration      "${z_consecration}" \
    --arg zjq_vessel_mode       "${RBRV_VESSEL_MODE}" \
    --arg zjq_bind_source       "${z_bind_source}" \
    --arg zjq_graft_source      "${z_graft_source}" \
    --arg zjq_ark_suffix_image  "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_vouch  "${RBGC_ARK_SUFFIX_VOUCH}" \
    --arg zjq_vi_ref_1          "${z_vi_ref_1}" \
    --arg zjq_vi_prov_1         "${z_vi_prov_1}" \
    --arg zjq_vi_ref_2          "${z_vi_ref_2}" \
    --arg zjq_vi_prov_2         "${z_vi_prov_2}" \
    --arg zjq_vi_ref_3          "${z_vi_ref_3}" \
    --arg zjq_vi_prov_3         "${z_vi_prov_3}" \
    --arg zjq_pool              "${RBDC_POOL_AIRGAP}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGV_GAR_HOST:          $zjq_gar_host,
        _RBGV_GAR_PATH:          $zjq_gar_path,
        _RBGV_VESSEL:            $zjq_vessel,
        _RBGV_CONSECRATION:      $zjq_consecration,
        _RBGV_VESSEL_MODE:       $zjq_vessel_mode,
        _RBGV_BIND_SOURCE:       $zjq_bind_source,
        _RBGV_GRAFT_SOURCE:      $zjq_graft_source,
        _RBGV_ARK_SUFFIX_IMAGE:  $zjq_ark_suffix_image,
        _RBGV_ARK_SUFFIX_VOUCH:  $zjq_ark_suffix_vouch,
        _RBGV_IMAGE_1:           $zjq_vi_ref_1,
        _RBGV_IMAGE_1_PROVENANCE: $zjq_vi_prov_1,
        _RBGV_IMAGE_2:           $zjq_vi_ref_2,
        _RBGV_IMAGE_2_PROVENANCE: $zjq_vi_prov_2,
        _RBGV_IMAGE_3:           $zjq_vi_ref_3,
        _RBGV_IMAGE_3_PROVENANCE: $zjq_vi_prov_3
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_vouch_build_file}" \
    || buc_die "Failed to compose vouch build JSON"

  buc_log_args "Vouch build JSON: ${z_vouch_build_file}"

  buc_step "Submitting vouch Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "vouch_build_create" "${z_vouch_build_file}"
  rbgu_http_require_ok "Vouch build submission" "vouch_build_create"

  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "vouch_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Vouch build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbf_wait_build_completion 50 "Vouch"  # ~4 minutes at 5s intervals (private pool is slower)
}

######################################################################
# Consecration Tally (rbw-Dt)

rbf_tally() {
  zrbf_sentinel

  buc_doc_brief "List consecrations across all vessels with health status"
  buc_doc_shown || return 0

  buc_step "Enumerating vessels"
  local z_sigils
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_any_pending=0
  local z_any_incomplete=0

  local z_sigil=""
  for z_sigil in ${z_sigils}; do

    buc_step "Querying GAR tags for ${z_sigil}"
    local z_tags_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_tags.json"
    local z_stderr_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBF_REGISTRY_API_BASE}/${z_sigil}/tags/list" \
      > "${z_tags_file}" 2>"${z_stderr_file}" \
      || buc_die "Failed to fetch tags for ${z_sigil} — see ${z_stderr_file}"

    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      local z_err
      jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}" > "${ZRBF_SCRATCH_FILE}" \
        || buc_die "Failed to extract error message from registry response for ${z_sigil}"
      z_err=$(<"${ZRBF_SCRATCH_FILE}")
      buc_die "Registry API error for ${z_sigil}: ${z_err}"
    fi

    # Extract tags and identify full consecrations
    local z_all_tags_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_all_tags.txt"
    jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
      || buc_die "Failed to extract tags for ${z_sigil}"

    local z_consec_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_consecrations.txt"
    local z_tag_data_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_tag_data.txt"
    : > "${z_consec_file}"
    : > "${z_tag_data_file}"

    while IFS= read -r z_tag || test -n "${z_tag}"; do
      local z_consec=""
      if [[ "${z_tag}" =~ ^([cbg][0-9]{12}-r[0-9]{12}) ]]; then
        z_consec="${BASH_REMATCH[1]}"
      else
        continue
      fi

      case "${z_tag}" in
        *"${RBGC_ARK_SUFFIX_IMAGE}"-*|*"${RBGC_ARK_SUFFIX_IMAGE}")
          local z_suffix="${z_tag#*"${RBGC_ARK_SUFFIX_IMAGE}"}"
          if test -z "${z_suffix}"; then
            echo "${z_consec}|image|consumer" >> "${z_tag_data_file}"
          else
            echo "${z_consec}|image|${z_suffix#-}" >> "${z_tag_data_file}"
          fi
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_VOUCH}")
          echo "${z_consec}|vouch|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_ABOUT}")
          echo "${z_consec}|about|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_DIAGS}")
          echo "${z_consec}|diags|" >> "${z_tag_data_file}"
          echo "${z_consec}" >> "${z_consec_file}"
          ;;
      esac
    done < "${z_all_tags_file}"

    local z_unique_file="${BURD_TEMP_DIR}/rbf_dc_${z_sigil}_unique.txt"
    sort -ur "${z_consec_file}" > "${z_unique_file}" \
      || buc_die "Failed to sort consecrations for ${z_sigil}"

    if ! test -s "${z_unique_file}"; then
      buc_info "No consecrations found for ${z_sigil}"
      continue
    fi

    # Display per-vessel table with health states
    printf "\nVessel: %s\n" "${z_sigil}"
    printf "  %-42s %-30s %-10s\n" "Consecration" "Platforms" "Health"

    while IFS= read -r z_consecration || test -n "${z_consecration}"; do
      local z_consec_platforms=""
      local z_has_about="no"
      local z_has_vouch="no"
      local z_has_image="no"

      while IFS='|' read -r z_c z_type z_detail; do
        test "${z_c}" = "${z_consecration}" || continue
        if test "${z_type}" = "image"; then
          z_has_image="yes"
          if test "${z_detail}" != "consumer"; then
            if test -n "${z_consec_platforms}"; then
              z_consec_platforms="${z_consec_platforms},${z_detail}"
            else
              z_consec_platforms="${z_detail}"
            fi
          fi
        elif test "${z_type}" = "about"; then
          z_has_about="yes"
        elif test "${z_type}" = "vouch"; then
          z_has_vouch="yes"
        fi
      done < "${z_tag_data_file}"

      local z_plat_display="${z_consec_platforms:-single}"
      local z_health=""
      if test "${z_has_about}" = "yes" && test "${z_has_vouch}" = "yes"; then
        z_health="vouched"
      elif test "${z_has_about}" = "yes" && test "${z_has_vouch}" = "no"; then
        z_health="pending"
        z_any_pending=1
      elif test "${z_has_image}" = "yes" && test "${z_has_about}" = "no"; then
        z_health="incomplete"
        z_any_incomplete=1
      fi

      printf "  %-42s %-30s %-10s\n" "${z_consecration}" "${z_plat_display}" "${z_health}"

      # Write per-consecration fact file for test observability
      test -n "${z_consecration}" || buc_die "Empty consecration in unique file for ${z_sigil}"
      echo "${z_sigil}" > "${BURD_OUTPUT_DIR}/${z_sigil}${RBCC_FACT_CONSEC_INFIX}${z_consecration}"
    done < "${z_unique_file}"

  done

  echo ""

  # Tabtarget recommendations
  if test "${z_any_pending}" = "1"; then
    buc_step "Pending consecrations can be vouched:"
    buc_tabtarget "${RBZ_VOUCH_CONSECRATIONS}"
  fi
  if test "${z_any_incomplete}" = "1"; then
    buc_step "Incomplete consecrations should be abjured and re-conjured:"
    buc_tabtarget "${RBZ_ABJURE_CONSECRATION}"
  fi

  buc_success "Consecration check complete"
}

######################################################################
# Batch Vouch (rbw-DV)

rbf_batch_vouch() {
  zrbf_sentinel

  buc_doc_brief "Vouch all pending consecrations across all vessels"
  buc_doc_shown || return 0

  buc_step "Enumerating vessels"
  local z_sigils
  z_sigils=$(rbrv_list_capture) || buc_die "No vessels found"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_vouched_count=0
  local z_already_count=0
  local z_failed_count=0

  local z_sigil=""
  for z_sigil in ${z_sigils}; do

    buc_step "Scanning ${z_sigil} for pending consecrations"
    local z_tags_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_tags.json"
    local z_stderr_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_stderr.txt"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBF_REGISTRY_API_BASE}/${z_sigil}/tags/list" \
      > "${z_tags_file}" 2>"${z_stderr_file}" \
      || buc_die "Failed to fetch tags for ${z_sigil} — see ${z_stderr_file}"

    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      local z_err
      jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}" > "${ZRBF_SCRATCH_FILE}" \
        || buc_die "Failed to extract error message from registry response for ${z_sigil}"
      z_err=$(<"${ZRBF_SCRATCH_FILE}")
      buc_die "Registry API error for ${z_sigil}: ${z_err}"
    fi

    local z_all_tags_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_all_tags.txt"
    jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}" \
      || buc_die "Failed to extract tags for ${z_sigil}"

    # Classify: find consecrations with -about but no -vouch
    local z_about_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_has_about.txt"
    local z_vouch_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_has_vouch.txt"
    : > "${z_about_file}"
    : > "${z_vouch_file}"

    while IFS= read -r z_tag || test -n "${z_tag}"; do
      local z_consec=""
      if [[ "${z_tag}" =~ ^([cbg][0-9]{12}-r[0-9]{12}) ]]; then
        z_consec="${BASH_REMATCH[1]}"
      else
        continue
      fi
      case "${z_tag}" in
        *"${RBGC_ARK_SUFFIX_VOUCH}")
          echo "${z_consec}" >> "${z_vouch_file}"
          ;;
        *"${RBGC_ARK_SUFFIX_ABOUT}")
          echo "${z_consec}" >> "${z_about_file}"
          ;;
      esac
    done < "${z_all_tags_file}"

    # Find pending: has about, no vouch
    local z_pending_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_pending.txt"
    sort -u "${z_about_file}" > "${z_about_file}.sorted" \
      || buc_die "Failed to sort about file for ${z_sigil}"
    sort -u "${z_vouch_file}" > "${z_vouch_file}.sorted" \
      || buc_die "Failed to sort vouch file for ${z_sigil}"
    comm -23 "${z_about_file}.sorted" "${z_vouch_file}.sorted" > "${z_pending_file}" \
      || buc_die "Failed to compute pending consecrations for ${z_sigil}"

    # Count already vouched for this vessel
    local z_count_file="${BURD_TEMP_DIR}/rbf_bv_${z_sigil}_vouch_count.txt"
    wc -l < "${z_vouch_file}.sorted" > "${z_count_file}" \
      || buc_die "Failed to count vouched consecrations for ${z_sigil}"
    local z_vessel_already=0
    z_vessel_already=$(<"${z_count_file}")
    z_vessel_already="${z_vessel_already// /}"
    z_already_count=$((z_already_count + z_vessel_already))

    if ! test -s "${z_pending_file}"; then
      buc_info "No pending consecrations for ${z_sigil}"
      continue
    fi

    # Load pending consecrations into array (load-then-iterate)
    local z_pending_items=()
    while IFS= read -r z_pline || test -n "${z_pline}"; do
      z_pending_items+=("${z_pline}")
    done < "${z_pending_file}"

    local z_vessel_dir="${RBRR_VESSEL_DIR}/${z_sigil}"
    local z_pi=""
    for z_pi in "${!z_pending_items[@]}"; do
      local z_consecration="${z_pending_items[$z_pi]}"
      test -n "${z_consecration}" || continue

      buc_step "Vouching ${z_sigil}/${z_consecration}"

      # Run vouch in isolation subshell — buc_die inside kills only the subshell
      local z_vouch_status=0
      (
        rbf_vouch "${z_vessel_dir}" "${z_consecration}" \
          || buc_die "rbf_vouch failed for ${z_sigil}/${z_consecration}"
      ) || z_vouch_status=$?

      if test "${z_vouch_status}" = "0"; then
        z_vouched_count=$((z_vouched_count + 1))
      else
        buc_warn "Vouch failed for ${z_sigil}/${z_consecration} (exit ${z_vouch_status}) — skipping"
        z_failed_count=$((z_failed_count + 1))
      fi
    done
  done

  echo ""
  buc_step "Batch vouch summary"
  buc_info "  Vouched:        ${z_vouched_count}"
  buc_info "  Already vouched: ${z_already_count}"
  buc_info "  Failed/skipped:  ${z_failed_count}"

  if test "${z_failed_count}" -gt 0; then
    buc_warn "Some consecrations failed — older builds may lack SLSA provenance"
  fi

  buc_success "Batch vouch complete"
}

######################################################################
# Plumb (rbw-RpF / rbw-Rpc)

# Internal: core plumb logic shared by full and compact modes
# Args: vessel consecration mode
zrbf_plumb_core() {
  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # accept directory path or bare moniker
  local -r z_consecration="${2:-}"
  local -r z_mode="${3}"

  test -n "${z_vessel}"       || buc_die "Vessel parameter required"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  # Load vessel config (sets RBRV_VESSEL_MODE, RBRV_BIND_IMAGE, etc.)
  local -r z_vessel_dir="${RBRR_VESSEL_DIR}/${z_vessel}"
  zrbf_load_vessel "${z_vessel_dir}"

  # Construct local image references (as tagged by docker pull / summon)
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_about_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_vessel}:${z_about_tag}"
  local -r z_vouch_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${z_vessel}:${z_vouch_tag}"

  # Check local availability of artifacts
  local z_has_about=false z_has_vouch=false
  docker image inspect "${z_about_ref}" >/dev/null 2>&1 && z_has_about=true
  docker image inspect "${z_vouch_ref}" >/dev/null 2>&1 && z_has_vouch=true

  # Bind vessels: use -about if available, fallback to static display
  if test "${RBRV_VESSEL_MODE}" = "bind"; then
    if test "${z_has_about}" = "false"; then
      zrbf_plumb_show_bind "${z_vessel}" "${z_consecration}" "${z_mode}"
      return 0
    fi
    # Bind vessel with -about: fall through to shared extract+display path
  fi

  # Require -about locally present (summon must have been run)
  if test "${z_has_about}" = "false"; then
    buc_die "About artifact not locally present — run summon first"
  fi

  # Extract -about contents into temp directory
  buc_step "Extracting -about artifact"
  local -r z_extract="${BURD_TEMP_DIR}/plumb"
  mkdir -p "${z_extract}" || buc_die "Failed to create extraction directory: ${z_extract}"
  local z_cid=""
  docker create "${z_about_ref}" x > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
    || buc_die "Failed to create container from -about artifact"
  z_cid=$(<"${ZRBF_SCRATCH_FILE}")
  docker cp "${z_cid}:/build_info.json"          "${z_extract}/" 2>/dev/null || true
  docker cp "${z_cid}:/sbom.json"                "${z_extract}/" 2>/dev/null || true
  docker cp "${z_cid}:/recipe.txt"               "${z_extract}/" 2>/dev/null || true
  docker cp "${z_cid}:/buildkit_metadata.json"   "${z_extract}/" 2>/dev/null || true
  docker cp "${z_cid}:/cache_before.json"        "${z_extract}/" 2>/dev/null || true
  docker cp "${z_cid}:/cache_after.json"         "${z_extract}/" 2>/dev/null || true
  docker rm "${z_cid}" >/dev/null 2>&1

  # Extract -vouch contents if locally present
  if test "${z_has_vouch}" = "true"; then
    buc_step "Extracting -vouch artifact"
    docker create "${z_vouch_ref}" x > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
      || buc_die "Failed to create container from -vouch artifact"
    z_cid=$(<"${ZRBF_SCRATCH_FILE}")
    docker cp "${z_cid}:/vouch_summary.json" "${z_extract}/" 2>/dev/null || true
    docker rm "${z_cid}" >/dev/null 2>&1
  fi

  # Display results
  if test "${z_mode}" = "compact"; then
    zrbf_plumb_show_compact "${z_vessel}" "${z_consecration}" "${z_extract}" "${z_has_vouch}"
  else
    zrbf_plumb_show_full "${z_vessel}" "${z_consecration}" "${z_extract}" "${z_has_vouch}"
  fi
}

# Internal: display bind vessel info
# Args: vessel consecration mode
zrbf_plumb_show_bind() {
  local -r z_vessel="$1"
  local -r z_consecration="$2"
  local -r z_mode="$3"

  if test "${z_mode}" = "compact"; then
    echo ""
    echo "=== ${z_vessel} / ${z_consecration} ==="
    echo "  Type: bind | Trust: digest-pin only"
    test -n "${RBRV_BIND_IMAGE:-}" && echo "  Source: ${RBRV_BIND_IMAGE}"
    echo "  No SLSA provenance, SBOM, or build transcript (not built by GCB)"
    echo ""
    return 0
  fi

  echo ""
  echo "================================================================"
  echo "  CONSECRATION PLUMB: ${z_vessel} / ${z_consecration}"
  echo "================================================================"
  echo ""
  echo "  Vessel type:  BIND (external image pinned by digest)"
  echo "  Trust model:  Digest-pin only"
  echo ""
  test -n "${RBRV_BIND_IMAGE:-}" && echo "  Bind source:  ${RBRV_BIND_IMAGE}"
  echo ""
  echo "  TRUST BOUNDARY"
  echo "  This is a bind vessel. The image was not built by Google Cloud"
  echo "  Build. No SLSA provenance, no SBOM, and no build transcript"
  echo "  exist because GCB did not produce this image."
  echo ""
  echo "  Trust is based solely on digest pinning of a known-good"
  echo "  external image from its source registry."
  echo ""
  echo "================================================================"
  echo ""
}

# Internal: shared section rendering used by both compact and full modes
# Args: extract_dir has_vouch
# Outputs: vessel type, source, builder, SLSA, SBOM summary, vouch results
zrbf_plumb_show_sections() {
  local -r z_dir="$1"
  local -r z_has_vouch="$2"

  local -r z_bi="${z_dir}/build_info.json"
  local -r z_sbom="${z_dir}/sbom.json"
  local -r z_vs="${z_dir}/vouch_summary.json"
  local -r z_bkmeta="${z_dir}/buildkit_metadata.json"

  # Determine vessel mode from build_info.json
  local z_vessel_mode="conjure"
  if test -f "${z_bi}"; then
    local z_mode_raw
    jq -r '.mode // "conjure"' "${z_bi}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
      || echo "conjure" > "${ZRBF_SCRATCH_FILE}"
    z_mode_raw=$(<"${ZRBF_SCRATCH_FILE}")
    z_vessel_mode="${z_mode_raw}"
  fi

  if test -f "${z_bi}" && test "${z_vessel_mode}" = "bind"; then
    # ── Bind vessel sections ──────────────────────────────────────────
    # Batch extract bind fields from build_info.json
    local z_bi_moniker="" z_bi_source_img="" z_bi_mirror_ts="" z_bi_consecration=""
    local z_bi_image_uri="" z_bi_git_repo="" z_bi_git_branch="" z_bi_git_commit=""
    jq -r '
      (.moniker // "?"),
      (.source.image_ref // "?"),
      (.build.inscribe_timestamp // "?"),
      (.build.consecration // "?"),
      (.image.uri // "?"),
      (.git.repo // "?"),
      (.git.branch // "?"),
      (.git.commit // "?")
    ' "${z_bi}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_bi_moniker
      read -r z_bi_source_img
      read -r z_bi_mirror_ts
      read -r z_bi_consecration
      read -r z_bi_image_uri
      read -r z_bi_git_repo
      read -r z_bi_git_branch
      read -r z_bi_git_commit
    } < "${ZRBF_SCRATCH_FILE}"

    echo ""
    echo "  -- Vessel Type -------------------------------------------------"
    echo "  How this image was produced."
    echo ""
    echo "  Mode:           bind (upstream image mirrored to GAR)"
    echo "  Moniker:        ${z_bi_moniker}"

    echo ""
    echo "  -- Upstream Source -----------------------------------------------"
    echo "  The digest-pinned upstream image that was mirrored."
    echo ""
    echo "  Source image:   ${z_bi_source_img}"
    echo "  Trust model:    digest-pin (image identity is the digest itself)"

    echo ""
    echo "  -- Mirror -------------------------------------------------------"
    echo "  When the image was mirrored from upstream into GAR."
    echo ""
    echo "  Mirror time:    ${z_bi_mirror_ts}"
    echo "  Consecration:   ${z_bi_consecration}"
    echo "  Image URI:      ${z_bi_image_uri}"

    echo ""
    echo "  -- Git Context --------------------------------------------------"
    echo "  The repository state when the mirror operation was performed."
    echo ""
    echo "  Repository:     ${z_bi_git_repo}"
    echo "  Branch:         ${z_bi_git_branch}"
    echo "  Commit:         ${z_bi_git_commit}"

    echo ""
    echo "  -- Trust --------------------------------------------------------"
    echo "  Bind vessels are NOT built by Cloud Build. Trust comes from the"
    echo "  digest pin in rbrv.env — the image is exactly the bytes specified."
    echo ""
    echo "  SLSA provenance:  not applicable (no build step)"
    echo "  Verification:     image digest matches the pin in the vessel definition"

  elif test -f "${z_bi}"; then
    # ── Conjure vessel sections ───────────────────────────────────────
    # Batch extract conjure fields from build_info.json
    local z_platform="" z_qemu="" z_cj_moniker=""
    local z_cj_git_repo="" z_cj_git_branch="" z_cj_git_commit=""
    local z_cj_build_id="" z_cj_build_ts="" z_cj_inscribe_ts="" z_cj_image_uri=""
    local z_slsa_level="" z_slsa_invocation="" z_slsa_builder=""
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
    ' "${z_bi}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_platform
      read -r z_qemu
      read -r z_cj_moniker
      read -r z_cj_git_repo
      read -r z_cj_git_branch
      read -r z_cj_git_commit
      read -r z_cj_build_id
      read -r z_cj_build_ts
      read -r z_cj_inscribe_ts
      read -r z_cj_image_uri
      read -r z_slsa_level
      read -r z_slsa_invocation
      read -r z_slsa_builder
    } < "${ZRBF_SCRATCH_FILE}"

    echo ""
    echo "  -- Vessel Type -------------------------------------------------"
    echo "  How this image was produced and for which CPU architecture."
    echo ""
    echo "  Mode:           conjure (built by Google Cloud Build)"
    local z_strategy="native"
    if test "${z_qemu}" = "true"; then z_strategy="emulated (QEMU)"; fi
    echo "  Platform:       ${z_platform} (host-platform view)"
    echo "  Build strategy: ${z_strategy}"
    echo "  Moniker:        ${z_cj_moniker}"

    echo ""
    echo "  -- Source -------------------------------------------------------"
    echo "  The git repository, branch, and commit that produced this build."
    echo ""
    echo "  Repository:     ${z_cj_git_repo}"
    echo "  Branch:         ${z_cj_git_branch}"
    echo "  Commit:         ${z_cj_git_commit}"

    echo ""
    echo "  -- Builder ------------------------------------------------------"
    echo "  The Cloud Build job that executed this build, with timestamps."
    echo ""
    echo "  Build ID:       ${z_cj_build_id}"
    echo "  Build time:     ${z_cj_build_ts}"
    echo "  Inscribe time:  ${z_cj_inscribe_ts}"
    echo "  Image URI:      ${z_cj_image_uri}"

    echo ""
    echo "  -- SLSA Provenance ----------------------------------------------"
    echo "  Cryptographic proof linking this exact image digest to its build."
    echo ""
    echo "  Build level:    ${z_slsa_level}"
    echo "  Invocation ID:  ${z_slsa_invocation}"
    echo "  Builder ID:     ${z_slsa_builder}"
    echo "  Predicate types:"
    jq -r '.slsa.provenance_predicate_types[]?' "${z_bi}" 2>/dev/null | while IFS= read -r z_pt; do
      echo "                    ${z_pt}"
    done

    echo ""
    echo "  SLSA Build L${z_slsa_level} attests:"
    echo "    + This digest was produced by this Cloud Build invocation"
    echo "    + From this source repo and commit"
    echo "    + On Google's hosted builder (tamper-resistant environment)"
    echo ""
    echo "  SLSA Build L${z_slsa_level} does NOT attest:"
    echo "    - Base image security or supply chain"
    echo "    - Package integrity within the image"
    echo "    - Absence of vulnerabilities"
    echo "    - Correctness or security of the Dockerfile"
  else
    echo ""
    echo "  build_info.json not found in -about artifact"
  fi

  # Base image section — conjure only (bind has no Dockerfile)
  if test "${z_vessel_mode}" != "bind"; then
    echo ""
    echo "  -- Base Image ---------------------------------------------------"
    echo "  The upstream image this build started FROM and the OS syft detected."
    echo ""
    local -r z_recipe="${z_dir}/recipe.txt"
    if test -f "${z_recipe}"; then
      local z_from_line=""
      grep -i '^FROM ' "${z_recipe}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
      read -r z_from_line < "${ZRBF_SCRATCH_FILE}" 2>/dev/null || z_from_line=""
      if test -n "${z_from_line}"; then
        echo "  Dockerfile FROM: ${z_from_line#FROM }"
      fi
    fi
    if test -f "${z_sbom}"; then
      local z_distro_name="" z_distro_ver=""
      jq -r '(.distro.name // empty), (.distro.version // empty)' "${z_sbom}" \
        > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_distro_name; read -r z_distro_ver; } < "${ZRBF_SCRATCH_FILE}"
      if test -n "${z_distro_name}"; then
        echo "  Detected distro: ${z_distro_name} ${z_distro_ver}"
      fi
    fi
  fi

  # Build output — conjure only (bind has no buildx step)
  if test "${z_vessel_mode}" != "bind" && test -f "${z_bkmeta}"; then
    echo ""
    echo "  -- Build Output -------------------------------------------------"
    echo "  The container image manifest produced by this buildx invocation."
    echo ""
    local z_bk_digest="" z_bk_mediatype="" z_bk_ref="" z_bk_imgname=""
    jq -r '
      (."containerimage.digest" // ""),
      (."containerimage.descriptor".mediaType // ""),
      (."buildx.build.ref" // ""),
      (."image.name" // "")
    ' "${z_bkmeta}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
    { read -r z_bk_digest
      read -r z_bk_mediatype
      read -r z_bk_ref
      read -r z_bk_imgname
    } < "${ZRBF_SCRATCH_FILE}"
    test -n "${z_bk_digest}"    && echo "  Output digest:  ${z_bk_digest}"
    test -n "${z_bk_mediatype}" && echo "  Media type:     ${z_bk_mediatype}"
    test -n "${z_bk_ref}"       && echo "  Build ref:      ${z_bk_ref}"
    test -n "${z_bk_imgname}"   && echo "  Image name:     ${z_bk_imgname}"
    # Per-platform digests if present
    local z_bk_platforms=""
    jq -r 'keys[] | select(contains("/"))' "${z_bkmeta}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
    z_bk_platforms=$(<"${ZRBF_SCRATCH_FILE}")
    if test -n "${z_bk_platforms}"; then
      echo "  Per-platform digests:"
      local z_bk_plat=""
      local z_bk_pd=""
      while IFS= read -r z_bk_plat; do
        jq -r --arg p "${z_bk_plat}" '.[$p]["containerimage.digest"] // empty' "${z_bkmeta}" \
          > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
        z_bk_pd=$(<"${ZRBF_SCRATCH_FILE}")
        test -n "${z_bk_pd}" && echo "    ${z_bk_plat}: ${z_bk_pd}"
      done <<< "${z_bk_platforms}"
    fi
  fi

  # Build cache delta — conjure only
  if test "${z_vessel_mode}" != "bind"; then
    local -r z_cache_before="${z_dir}/cache_before.json"
    local -r z_cache_after="${z_dir}/cache_after.json"
    if test -f "${z_cache_after}"; then
      echo ""
      echo "  -- Build Cache Delta --------------------------------------------"
      echo "  Images on the Cloud Build worker after vs before this build."
      echo ""
      local z_before_count="n/a"
      local z_after_count=""
      if test -f "${z_cache_before}"; then
        jq '.host_daemon_images | length' "${z_cache_before}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
          || echo "?" > "${ZRBF_SCRATCH_FILE}"
        z_before_count=$(<"${ZRBF_SCRATCH_FILE}")
      fi
      jq '.host_daemon_images | length' "${z_cache_after}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
        || echo "?" > "${ZRBF_SCRATCH_FILE}"
      z_after_count=$(<"${ZRBF_SCRATCH_FILE}")
      echo "  Images before: ${z_before_count}"
      echo "  Images after:  ${z_after_count}"
      if test -f "${z_cache_before}"; then
        local z_new_images=""
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
        ' "${z_cache_after}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
        z_new_images=$(<"${ZRBF_SCRATCH_FILE}")
        if test -n "${z_new_images}"; then
          local z_new_count=0
          local z_count_line=""
          while IFS= read -r z_count_line; do
            z_new_count=$((z_new_count + 1))
          done <<< "${z_new_images}"
          echo ""
          echo "  New images (${z_new_count} unique):"
          printf '%s\n' "${z_new_images}" | while IFS=$'\t' read -r z_repo z_tag z_size z_id; do
            echo "    ${z_id}  ${z_repo}:${z_tag}  ${z_size}"
          done
        else
          echo "  No new images (cache unchanged)"
        fi
      fi
    fi
  fi

  # SBOM — present for both bind and conjure (if syft was available)
  echo ""
  echo "  -- SBOM Summary (syft) ------------------------------------------"
  echo "  Software bill of materials: every package syft found installed."
  echo ""
  if test -f "${z_sbom}"; then
    local z_pkg_count=""
    jq '.artifacts | length' "${z_sbom}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null \
      || echo "?" > "${ZRBF_SCRATCH_FILE}"
    z_pkg_count=$(<"${ZRBF_SCRATCH_FILE}")
    echo "  Package count:  ${z_pkg_count}"

    echo "  Package types:"
    jq -r '
      [.artifacts[]?.type // empty] | group_by(.) |
      map({type: .[0], count: length}) |
      sort_by(-.count)[] |
      "    \(.count)\t\(.type)"
    ' "${z_sbom}" 2>/dev/null || echo "    (unable to parse)"

    echo ""
    echo "  Syft inventories installed packages. This is not a security"
    echo "  assessment, vulnerability scan, or license audit."
  else
    echo "  sbom.json not found in -about artifact"
  fi

  # Vouch — branched by vessel mode
  echo ""
  echo "  -- Vouch Results ------------------------------------------------"
  if test "${z_vessel_mode}" = "bind"; then
    echo "  Bind verification: was the mirrored image verified against its digest pin?"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_vf_method="" z_vf_result="" z_vf_pin="" z_vf_gar=""
      local z_vf_match="" z_vf_ts="" z_vf_source=""
      jq -r '
        (.verification.method // "?"),
        (.verification.result // .verification.verdict // "?"),
        (.verification.pin_digest // .verification.pinned_digest // "?"),
        (.verification.gar_digest // .verification.actual_digest // "?"),
        (.verification.digest_match // "?"),
        (.verification.timestamp // "?"),
        (.verification.source_image // .verification.bind_source // "?")
      ' "${z_vs}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_vf_method
        read -r z_vf_result
        read -r z_vf_pin
        read -r z_vf_gar
        read -r z_vf_match
        read -r z_vf_ts
        read -r z_vf_source
      } < "${ZRBF_SCRATCH_FILE}"
      echo "  Method:      ${z_vf_method}"
      echo "  Verdict:     ${z_vf_result}"
      echo "  Pin digest:  ${z_vf_pin}"
      echo "  GAR digest:  ${z_vf_gar}"
      echo "  Match:       ${z_vf_match}"
      echo "  Timestamp:   ${z_vf_ts}"
      echo "  Source:      ${z_vf_source}"
    else
      echo "  Vouch artifact not locally present — run summon to retrieve"
    fi
  elif test "${z_vessel_mode}" = "graft"; then
    echo "  Graft acknowledgment: no provenance chain — GRAFTED verdict"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_gf_verdict="" z_gf_source="" z_gf_method=""
      jq -r '
        (.verification.verdict // "?"),
        (.verification.graft_source // "?"),
        (.verification.method // "?")
      ' "${z_vs}" > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_gf_verdict
        read -r z_gf_source
        read -r z_gf_method
      } < "${ZRBF_SCRATCH_FILE}"
      echo "  Verdict:     ${z_gf_verdict}"
      echo "  Method:      ${z_gf_method}"
      echo "  Source:      ${z_gf_source}"
    else
      echo "  Vouch artifact not locally present — run summon to retrieve"
    fi
  else
    echo "  Independent SLSA verification: did this image pass provenance checks?"
    echo ""
    if test "${z_has_vouch}" = "true" && test -f "${z_vs}"; then
      local z_verifier_url="" z_verifier_sha=""
      jq -r '(.verifier.url // "?"), (.verifier.sha256 // "?")' "${z_vs}" \
        > "${ZRBF_SCRATCH_FILE}" 2>/dev/null || true
      { read -r z_verifier_url; read -r z_verifier_sha; } < "${ZRBF_SCRATCH_FILE}"
      echo "  Verifier:"
      echo "    URL:    ${z_verifier_url}"
      echo "    SHA256: ${z_verifier_sha}"
      echo ""
      echo "  Per-platform verdicts:"
      jq -r '.platforms[]? | "    \(.platform): \(.verdict)"' "${z_vs}" 2>/dev/null \
        || echo "    (unable to parse)"
    else
      echo "  Vouch artifact not locally present — run summon to retrieve"
    fi
  fi
}

# Internal: display compact vessel info (conjure or bind)
# Args: vessel consecration extract_dir has_vouch
zrbf_plumb_show_compact() {
  local -r z_vessel="$1"
  local -r z_consecration="$2"
  local -r z_dir="$3"
  local -r z_has_vouch="$4"

  echo ""
  echo "================================================================"
  echo "  CONSECRATION PLUMB: ${z_vessel} / ${z_consecration}"
  echo "================================================================"

  zrbf_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo ""
  echo "================================================================"
  echo ""
}

# Internal: display full vessel info (conjure or bind)
# Adds per-package inventory and Dockerfile (conjure only) to the compact sections.
# Args: vessel consecration extract_dir has_vouch
zrbf_plumb_show_full() {
  local -r z_vessel="$1"
  local -r z_consecration="$2"
  local -r z_dir="$3"
  local -r z_has_vouch="$4"

  local -r z_sbom="${z_dir}/sbom.json"

  echo ""
  echo "================================================================"
  echo "  CONSECRATION PLUMB (FULL): ${z_vessel} / ${z_consecration}"
  echo "================================================================"

  zrbf_plumb_show_sections "${z_dir}" "${z_has_vouch}"

  echo ""
  echo "  -- Package Inventory --------------------------------------------"
  echo "  Every package syft detected, sorted by ecosystem type."
  echo ""
  if test -f "${z_sbom}"; then
    printf "    %-12s %-36s %s\n" "TYPE" "NAME" "VERSION"
    printf "    %-12s %-36s %s\n" "----" "----" "-------"
    jq -r '
      .artifacts[]? |
      [.type // "?", .name // "?", .version // "?"] |
      @tsv
    ' "${z_sbom}" 2>/dev/null | sort | while IFS=$'\t' read -r z_type z_name z_ver; do
      printf "    %-12s %-36s %s\n" "${z_type}" "${z_name}" "${z_ver}"
    done
  else
    echo "    sbom.json not found in -about artifact"
  fi

  echo ""
  echo "  -- Package Licensing & Identity ---------------------------------"
  echo "  License and Package URL for each package (for compliance review)."
  echo ""
  if test -f "${z_sbom}"; then
    printf "    %-36s %-20s %s\n" "NAME" "LICENSE" "PURL"
    printf "    %-36s %-20s %s\n" "----" "-------" "----"
    jq -r '
      .artifacts[]? |
      [
        (.name // "?"),
        ((.licenses // []) | map(.value // .expression // empty) | join(", ") | if . == "" then "-" else . end),
        (.purl // "-")
      ] |
      @tsv
    ' "${z_sbom}" 2>/dev/null | sort | while IFS=$'\t' read -r z_name z_lic z_purl; do
      printf "    %-36s %-20s %s\n" "${z_name}" "${z_lic}" "${z_purl}"
    done
  else
    echo "    sbom.json not found in -about artifact"
  fi

  local -r z_recipe="${z_dir}/recipe.txt"
  if test -f "${z_recipe}"; then
    echo ""
    echo "  -- Recipe (Dockerfile) ------------------------------------------"
    echo "  The exact Dockerfile used to build this image."
    echo ""
    while IFS= read -r z_line; do
      echo "    ${z_line}"
    done < "${z_recipe}"
  fi

  echo ""
  echo "================================================================"
  echo ""
}

rbf_plumb_full() {
  zrbf_sentinel

  local z_vessel="${1:-}"
  local z_consecration="${2:-}"

  buc_doc_brief "Plumb a consecration's trust posture (full detail)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  zrbf_plumb_core "${z_vessel}" "${z_consecration}" "full"
}

rbf_plumb_compact() {
  zrbf_sentinel

  local z_vessel="${1:-}"
  local z_consecration="${2:-}"

  buc_doc_brief "Plumb a consecration's trust posture (compact summary)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "consecration" "Full consecration (e.g., c260305133650-r260305160530)"
  buc_doc_shown || return 0

  zrbf_plumb_core "${z_vessel}" "${z_consecration}" "compact"
}

# eof

