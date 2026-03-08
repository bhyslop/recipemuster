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

  buc_log_args 'Define stitched build JSON temp file'
  readonly ZRBF_STITCHED_BUILD_FILE="${BURD_TEMP_DIR}/rbf_stitched_build.json"

  buc_log_args 'Define temp files for build operations'
  readonly ZRBF_BUILD_ID_FILE="${BURD_TEMP_DIR}/rbf_build_id.txt"
  readonly ZRBF_BUILD_STATUS_FILE="${BURD_TEMP_DIR}/rbf_build_status.json"
  readonly ZRBF_BUILD_RUBRIC_LS="${BURD_TEMP_DIR}/rbf_rubric_ls_remote.txt"
  readonly ZRBF_BUILD_TRIGGER_BODY="${BURD_TEMP_DIR}/rbf_trigger_run_body.json"

  buc_log_args 'Define git info file (used by inscribe -> stitch)'
  readonly ZRBF_GIT_INFO_FILE="${BURD_TEMP_DIR}/rbf_git_info.json"

  buc_log_args 'Define validation files'
  readonly ZRBF_STATUS_CHECK_FILE="${BURD_TEMP_DIR}/rbf_status_check.txt"

  buc_log_args 'Define delete operation files'
  readonly ZRBF_DELETE_PREFIX="${BURD_TEMP_DIR}/rbf_delete_"
  readonly ZRBF_TOKEN_FILE="${BURD_TEMP_DIR}/rbf_token.txt"

  buc_log_args 'Vessel-related files'
  readonly ZRBF_VESSEL_SIGIL_FILE="${BURD_TEMP_DIR}/rbf_vessel_sigil.txt"

  buc_log_args 'Define stitch operation file prefix (postfixed per step id)'
  readonly ZRBF_STITCH_PREFIX="${BURD_TEMP_DIR}/rbf_stitch_"

  buc_log_args 'Define inscribe operation files'
  readonly ZRBF_INSCRIBE_PREFIX="${BURD_TEMP_DIR}/rbf_inscribe_"
  readonly ZRBF_INSCRIBE_CLONE_DIR="${RBGC_RUBRIC_CLONE_DIR}"
  readonly ZRBF_INSCRIBE_STALENESS_SEC=86400

  buc_log_args 'Define output files (BURD_OUTPUT_DIR — persists after dispatch)'
  readonly ZRBF_OUTPUT_VESSEL_DIR="${BURD_OUTPUT_DIR}/rbf_vessel_dir.txt"
  readonly ZRBF_OUTPUT_CONSECRATION="${BURD_OUTPUT_DIR}/rbf_consecration.txt"

  buc_log_args 'For now lets double check these'
  test -n "${RBRG_ORAS_IMAGE_REF:-}"   || buc_die "RBRG_ORAS_IMAGE_REF not set"

  readonly ZRBF_KINDLED=1
}

zrbf_sentinel() {
  test "${ZRBF_KINDLED:-}" = "1" || buc_die "Module rbf not kindled - call zrbf_kindle first"
}

# Check concurrent build quota against regime requirements
# Args: token mode
#   mode: "gate" (die if insufficient) or "advisory" (warn if insufficient)
zrbf_quota_preflight() {
  zrbf_sentinel

  local -r z_token="${1:-}"
  local -r z_mode="${2:-}"

  test -n "${z_token}" || buc_die "zrbf_quota_preflight: token required"
  test -n "${z_mode}"  || buc_die "zrbf_quota_preflight: mode required (gate|advisory)"

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
    buc_tabtarget "${RBZ_QUOTA_BUILD}"
    if test "${z_mode}" = "gate"; then
      buc_die "Quota preflight failed -- review capacity settings above"
    fi
  else
    buc_info "Quota OK: ${z_limit} vCPU / ${z_vcpus} per build = ${z_max_concurrent} concurrent (need ${RBRR_GCB_MIN_CONCURRENT_BUILDS})"
  fi
}

zrbf_stitch_build_json() {
  zrbf_sentinel

  buc_log_args 'Stitching trigger-compatible build JSON from step scripts'

  # Preconditions: vessel loaded and git state captured
  test -s "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Vessel not loaded — call zrbf_load_vessel first"
  test -s "${ZRBF_GIT_INFO_FILE}"     || buc_die "Git info not captured — ensure git metadata is captured before stitch"

  buc_log_args 'Read vessel state for substitutions'
  local -r z_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}")
  test -n "${z_sigil}" || buc_die "Empty vessel sigil"
  local -r z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"
  local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"

  # Platform count detection
  local z_platform_count=0
  local z_remaining_count="${z_platforms}"
  local z_p_count=""
  while test -n "${z_remaining_count}"; do
    z_p_count="${z_remaining_count%%,*}"
    z_platform_count=$((z_platform_count + 1))
    test "${z_remaining_count}" = "${z_p_count}" && break
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

  # Step definitions: script|builder|entrypoint|id
  # Entrypoint 'bash' uses args ["-lc", script], 'sh' uses ["-c", script]
  # Delimiter is | because image refs contain colons (sha256 digests)
  # Pipeline: buildx --push → per-platform pullback → SLSA provenance via images: field
  local z_step_defs=(
    "rbgjb01-derive-tag-base.sh|${RBRG_GCLOUD_IMAGE_REF}|bash|derive-tag-base"
    "rbgjb02-qemu-binfmt.sh|${RBRG_DOCKER_IMAGE_REF}|bash|qemu-binfmt"
    "rbgjb03-buildx-push-multi.sh|${RBRG_DOCKER_IMAGE_REF}|bash|buildx-push-multi"
    "rbgjb04-per-platform-pullback.sh|${RBRG_DOCKER_IMAGE_REF}|bash|per-platform-pullback"
    "rbgjb05-push-per-platform.sh|${RBRG_DOCKER_IMAGE_REF}|bash|push-per-platform"
    "rbgjb06-syft-per-platform.sh|${RBRG_DOCKER_IMAGE_REF}|bash|syft-per-platform"
    "rbgjb07-build-info-per-platform.sh|${RBRG_ALPINE_IMAGE_REF}|sh|build-info-per-platform"
    "rbgjb08-buildx-push-about.sh|${RBRG_DOCKER_IMAGE_REF}|bash|buildx-push-about"
    "rbgjb09-imagetools-create.sh|${RBRG_DOCKER_IMAGE_REF}|bash|imagetools-create"
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
    test "${z_remaining_plats}" = "${z_plat}" && break
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
  local z_arg_flag=""
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

    buc_log_args "Baking pinned image refs into script text (GCB containers lack RBRR vars)"
    z_body="${z_body//\$\{RBRG_SYFT_IMAGE_REF\}/${RBRG_SYFT_IMAGE_REF}}"
    z_body="${z_body//\$\{RBRG_BINFMT_IMAGE_REF\}/${RBRG_BINFMT_IMAGE_REF}}"

    buc_log_args "Escaping dollars for Cloud Build, preserving RBGY substitutions"
    printf '%s' "${z_body}" | sed 's/\$/\$\$/g; s/\$\${_RBGY_/${_RBGY_/g' \
      > "${z_escaped_file}" || buc_die "Failed to escape script body for ${z_id}"

    case "${z_entrypoint}" in
      bash) z_entrypoint="/bin/bash"; z_arg_flag="-lc" ;;
      sh)   z_entrypoint="/bin/sh";   z_arg_flag="-c" ;;
      *)    buc_die "Unknown entrypoint: ${z_entrypoint}" ;;
    esac

    buc_log_args "Appending step ${z_id} to JSON array"
    jq \
      --arg name "${z_builder}" \
      --arg id "${z_id}" \
      --arg ep "${z_entrypoint}" \
      --arg flag "${z_arg_flag}" \
      --arg dir "${z_sigil}" \
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, entrypoint: $ep, dir: $dir, args: [$flag, $script]}]' \
      "${z_accumulator_file}" > "${z_steps_file}" \
      || buc_die "Failed to append step ${z_id} to JSON"
    mv "${z_steps_file}" "${z_accumulator_file}" \
      || buc_die "Failed to update steps JSON for ${z_id}"
  done

  # Compose complete trigger-compatible Build resource
  # Steps from accumulator, substitutions from module state, options/timeout from RBRR
  # _RBGY_RUBRIC_REPO, _RBGY_RUBRIC_COMMIT, _RBGY_INSCRIBE_TIMESTAMP, _RBGY_GIT_COMMIT
  # are placeholders in the main-repo copy; inscribe fills them in the rubric repo copy
  buc_log_args "Composing complete trigger-compatible Build resource"
  local -r z_build_file="${ZRBF_STITCH_PREFIX}build.json"

  # images: field uses inscribe-time-predictable alias tags for SLSA provenance.
  # CB pushes these after all steps complete, generating provenance attestations
  # keyed to image digests. The consecration-tagged copies (pushed in step 05)
  # share the same digests, so provenance applies to them too.
  # Alias tags are ephemeral (overwritten on re-conjure); consecration tags persist.

  # Build images: array — one entry per platform using inscribe-time alias tag
  local z_images_file="${ZRBF_STITCH_PREFIX}images.json"
  local z_image_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${z_sigil}"
  local z_remaining_suffixes="${z_platform_suffixes_csv}"
  local z_img_suffix=""
  echo "[]" > "${z_images_file}" || buc_die "Failed to initialize images JSON"
  while test -n "${z_remaining_suffixes}"; do
    z_img_suffix="${z_remaining_suffixes%%,*}"
    jq --arg uri "${z_image_base}:__INSCRIBE_TIMESTAMP__-image${z_img_suffix}" \
      '. + [$uri]' "${z_images_file}" > "${z_images_file}.tmp" \
      || buc_die "Failed to append image URI"
    mv "${z_images_file}.tmp" "${z_images_file}" \
      || buc_die "Failed to update images JSON"
    test "${z_remaining_suffixes}" = "${z_img_suffix}" && break
    z_remaining_suffixes="${z_remaining_suffixes#*,}"
  done

  jq -n \
    --slurpfile zjq_steps  "${z_accumulator_file}" \
    --slurpfile zjq_images "${z_images_file}" \
    --arg zjq_dockerfile     "${z_dockerfile_name}" \
    --arg zjq_moniker        "${z_sigil}" \
    --arg zjq_platforms      "${z_platforms}" \
    --arg zjq_platform_suffixes "${z_platform_suffixes_csv}" \
    --arg zjq_gar_location   "${RBGD_GAR_LOCATION}" \
    --arg zjq_gar_project    "${RBGD_GAR_PROJECT_ID}" \
    --arg zjq_gar_repository "${RBRR_GAR_REPOSITORY}" \
    --arg zjq_git_commit     "__INSCRIBE_GIT_COMMIT__" \
    --arg zjq_git_branch     "${z_git_branch}" \
    --arg zjq_git_repo       "${z_git_repo}" \
    --arg zjq_gar_host_suffix  "${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_rubric_repo      "__INSCRIBE_RUBRIC_REPO__" \
    --arg zjq_rubric_commit    "__INSCRIBE_RUBRIC_COMMIT__" \
    --arg zjq_inscribe_ts      "__INSCRIBE_TIMESTAMP__" \
    --arg zjq_pool   "${RBRR_GCB_WORKER_POOL}" \
    --arg zjq_timeout "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
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
        _RBGY_GIT_REPO:            $zjq_git_repo,
        _RBGY_GAR_HOST_SUFFIX:     $zjq_gar_host_suffix,
        _RBGY_ARK_SUFFIX_IMAGE:    $zjq_ark_suffix_image,
        _RBGY_ARK_SUFFIX_ABOUT:    $zjq_ark_suffix_about,
        _RBGY_RUBRIC_REPO:         $zjq_rubric_repo,
        _RBGY_RUBRIC_COMMIT:       $zjq_rubric_commit,
        _RBGY_INSCRIBE_TIMESTAMP:  $zjq_inscribe_ts
      },
      options: {
        requestedVerifyOption: "VERIFIED",
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose build JSON"

  mv "${z_build_file}" "${ZRBF_STITCHED_BUILD_FILE}" \
    || buc_die "Failed to write final stitched build JSON"

  buc_log_args "Stitched ${#z_step_defs[@]} steps to ${ZRBF_STITCHED_BUILD_FILE}"
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

zrbf_wait_build_completion() {
  zrbf_sentinel

  buc_step 'Waiting for build completion'

  local z_build_id=""
  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}") || buc_die "No build ID found"
  test -n "${z_build_id}" || buc_die "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=960  # 80 minutes with 5 second intervals
  local z_consecutive_failures=0
  local z_max_consecutive_failures=3
  local z_err_check_file="${ZRBF_STITCH_PREFIX}poll_err_check.txt"

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_attempts=$((z_attempts + 1))
    test ${z_attempts} -le ${z_max_attempts} || buc_die "Build timeout after ${z_max_attempts} attempts"

    buc_log_args "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
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

    buc_info "Build status: ${z_status} (attempt ${z_attempts}/${z_max_attempts})"
  done

  test "${z_status}" = "SUCCESS" || buc_die "Build failed with status: ${z_status}"

  buc_success 'Build completed successfully'
}

######################################################################
# External Functions (rbf_*)

rbf_build() {
  zrbf_sentinel

  local -r z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Build container image from vessel via trigger dispatch"
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

  buc_log_args "Enforce vessel binfmt policy"
  if test "${RBRV_CONJURE_BINFMT_POLICY}" = "forbid"; then
    if test "${RBRV_CONJURE_PLATFORMS}" != "${RBGC_BUILD_RUNNER_PLATFORM}"; then
      buc_die "Vessel '${RBRV_SIGIL}' forbids binfmt but RBRV_CONJURE_PLATFORMS='${RBRV_CONJURE_PLATFORMS}' extends beyond Cloud Build runner platform (${RBGC_BUILD_RUNNER_PLATFORM})"
    fi
  fi

  buc_info "Building vessel image: ${RBRV_SIGIL}"

  # Source Director RBRA for credentials (still needed for OAuth token)
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"
  test -n "${RBRR_RUBRIC_REPO_URL:-}" || buc_die "RBRR_RUBRIC_REPO_URL not set in rbrr.env"

  # Authenticate as Director
  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Quota preflight -- die if insufficient capacity
  zrbf_quota_preflight "${z_token}" "gate"

  # Fetch GitLab token from Secret Manager (api token secret)
  buc_step "Fetching rubric repo token from Secret Manager"
  local -r z_secret_access_url="${RBGC_API_ROOT_SECRETMANAGER}${RBGC_SECRETMANAGER_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/secrets/${RBGC_CBV2_API_TOKEN_SECRET_NAME}/versions/latest:access"
  rbgu_http_json "GET" "${z_secret_access_url}" "${z_token}" "build_token_fetch"
  rbgu_http_require_ok "Fetch token from Secret Manager" "build_token_fetch"
  local z_token_b64
  z_token_b64=$(rbgu_json_field_capture "build_token_fetch" '.payload.data') \
    || buc_die "Failed to extract token payload"
  local z_gitlab_token_value
  z_gitlab_token_value=$(printf '%s' "${z_token_b64}" | base64 -d) \
    || buc_die "Failed to decode token"

  # Construct authenticated URL: https://gitlab.com/... → https://oauth2:TOKEN@gitlab.com/...
  local -r z_rubric_auth_url="https://oauth2:${z_gitlab_token_value}@${RBRR_RUBRIC_REPO_URL#https://}"

  # Resolve rubric repo HEAD via ls-remote (no clone needed — build only needs commit hash)
  buc_step "Resolving rubric repo HEAD commit"
  git ls-remote "${z_rubric_auth_url}" HEAD > "${ZRBF_BUILD_RUBRIC_LS}" \
    || buc_die "Failed to reach rubric repo — check RBRR_RUBRIC_REPO_URL and GitLab token in Secret Manager"
  local z_rubric_commit=$(<"${ZRBF_BUILD_RUBRIC_LS}")
  test -n "${z_rubric_commit}" || buc_die "Rubric repo HEAD is empty — has inscribe been run?"
  z_rubric_commit="${z_rubric_commit%%	*}"
  buc_info "Rubric repo HEAD: ${z_rubric_commit:0:8}"

  # Resolve trigger identity by direct GET (trigger name = trigger ID, set at create time)
  buc_step "Resolving vessel trigger"
  local -r z_trigger_name="${RBGC_RUBRIC_TRIGGER_PREFIX}${RBRV_SIGIL}"
  rbgu_http_json "GET" "${ZRBF_TRIGGERS_URL}/${z_trigger_name}" "${z_token}" "build_trigger_check"
  local z_trigger_code=""
  z_trigger_code=$(rbgu_http_code_capture "build_trigger_check") || z_trigger_code=""
  test "${z_trigger_code}" = "200" \
    || buc_die "Vessel trigger '${z_trigger_name}' not found (HTTP ${z_trigger_code}) — run rubric inscribe first"
  buc_info "Trigger resolved: ${z_trigger_name}"

  # Dispatch build via triggers.run — zero substitution overrides
  buc_step "Dispatching trigger build for ${RBRV_SIGIL}"
  local -r z_run_url="${ZRBF_TRIGGERS_URL}/${z_trigger_name}:run"
  jq -n --arg sha "${z_rubric_commit}" '{"source": {"commitSha": $sha}}' \
    > "${ZRBF_BUILD_TRIGGER_BODY}" || buc_die "Failed to compose triggers.run body"

  rbgu_http_json "POST" "${z_run_url}" "${z_token}" "build_trigger_run" "${ZRBF_BUILD_TRIGGER_BODY}"
  rbgu_http_require_ok "Trigger dispatch" "build_trigger_run"

  # Extract build ID from Operation response
  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "build_trigger_run" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in triggers.run response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Build dispatched: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  # Wait for completion (5s intervals, up to 80 minutes)
  zrbf_wait_build_completion

  # Discover consecration from build step output (strong tie — no GAR scanning)
  # Step 01 (derive-tag-base) writes consecration to /builder/outputs/output,
  # which appears base64-encoded in results.buildStepOutputs[0].
  buc_step "Discovering consecration from build step output"

  local z_step0_output=""
  z_step0_output=$(jq -r '.results.buildStepOutputs[0] // empty' "${ZRBF_BUILD_STATUS_FILE}") \
    || buc_die "Failed to extract buildStepOutputs[0] from build response"
  test -n "${z_step0_output}" || buc_die "Build step 0 output empty — step 01 may not have written to /builder/outputs/output"

  local z_found_consecration=""
  z_found_consecration=$(echo "${z_step0_output}" | base64 -d) \
    || buc_die "Failed to base64-decode build step output"
  test -n "${z_found_consecration}" || buc_die "Decoded consecration is empty"
  buc_info "Discovered consecration: ${z_found_consecration}"

  local z_inscribe_ts=""
  z_inscribe_ts=$(jq -r '.substitutions._RBGY_INSCRIBE_TIMESTAMP // empty' "${ZRBF_BUILD_STATUS_FILE}") \
    || buc_die "Failed to extract inscribe timestamp from build response"
  test -n "${z_inscribe_ts}" || buc_die "Inscribe timestamp empty in build response"
  buc_info "Inscribe timestamp: ${z_inscribe_ts}"

  # Persist to output directory for test harness consumption
  echo "${z_vessel_dir}" > "${ZRBF_OUTPUT_VESSEL_DIR}" \
    || buc_die "Failed to write vessel dir to output"
  echo "${z_found_consecration}" > "${ZRBF_OUTPUT_CONSECRATION}" \
    || buc_die "Failed to write consecration to output"

  # Write primary image reference fact file
  local z_first_plat="${RBRV_CONJURE_PLATFORMS%%,*}"
  z_first_plat="${z_first_plat%% *}"
  local z_first_suffix="${z_first_plat#linux/}"
  z_first_suffix="${z_first_suffix//\//}"
  local z_primary_image_tag="${z_found_consecration}${RBGC_ARK_SUFFIX_IMAGE}-${z_first_suffix}"
  local z_primary_image_ref="${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}/${RBRV_SIGIL}:${z_primary_image_tag}"
  echo "${z_primary_image_ref}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_IMAGE_REF}" \
    || buc_die "Failed to write image ref fact file"

  # Write build ID fact file (dispatched build ID for cross-check with vouch provenance)
  echo "${z_build_id}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}" \
    || buc_die "Failed to write build ID fact file"

  buc_info "Output: ${ZRBF_OUTPUT_VESSEL_DIR}"
  buc_info "Output: ${ZRBF_OUTPUT_CONSECRATION}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_IMAGE_REF}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_BUILD_ID}"

  buc_success "Vessel image built: ${RBRV_SIGIL}"
}

rbf_delete() {
  zrbf_sentinel

  local z_locator="${1:-}"
  local z_force="${2:-}"

  # Documentation block
  buc_doc_brief "Delete an image tag from the registry by locator"
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

  # Confirm deletion unless --force
  if test "${z_skip_confirm}" = "false"; then
    buc_require "Will delete: ${z_locator}" "yes"
  fi

  buc_step "Deleting: ${z_locator}"

  # Delete by tag reference
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
    buc_warn "Response body: $(cat "${z_response_file}" 2>/dev/null || echo 'empty')"
    buc_die "Delete failed with HTTP ${z_http_code}"
  fi

  buc_success "Deleted or nonexistent: ${z_locator}"
}

rbf_list() {
  zrbf_sentinel

  # Documentation block
  buc_doc_brief "List all locators (moniker:tag) in the Artifact Registry repository"
  buc_doc_shown || return 0

  buc_step "Authenticating as Retriever"

  # Prefer Retriever credentials, fallback to Director
  local z_rbra_file=""
  if test -n "${RBDC_RETRIEVER_RBRA_FILE:-}" && test -f "${RBDC_RETRIEVER_RBRA_FILE}"; then
    z_rbra_file="${RBDC_RETRIEVER_RBRA_FILE}"
    buc_info "Using Retriever credentials"
  else
    z_rbra_file="${RBDC_DIRECTOR_RBRA_FILE}"
    buc_info "Retriever not configured, using Director credentials"
  fi

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || buc_die "Failed to get OAuth token"

  buc_step "Listing all images in repository"

  local z_packages_file="${BURD_TEMP_DIR}/rbf_list_packages.json"
  local z_gar_api="https://artifactregistry.googleapis.com/v1"
  local z_repo_path="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${z_gar_api}/${z_repo_path}/packages" \
    > "${z_packages_file}" || buc_die "Failed to fetch packages"

  # Check for error response
  if jq -e '.error' "${z_packages_file}" >/dev/null 2>&1; then
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${z_packages_file}")
    buc_die "API error: ${z_error}"
  fi

  # Extract package monikers
  local z_packages_list="${BURD_TEMP_DIR}/rbf_list_packages.txt"
  jq -r '.packages[]?.name // empty' "${z_packages_file}" | while read -r pkg; do
    echo "${pkg##*/}"
  done | sort > "${z_packages_list}"

  echo "Repository: ${ZRBF_REGISTRY_HOST}/${ZRBF_REGISTRY_PATH}"

  # For each moniker, fetch tags and emit locators
  while IFS= read -r z_moniker; do
    local z_tags_file="${BURD_TEMP_DIR}/rbf_list_tags_${z_moniker}.json"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBF_REGISTRY_API_BASE}/${z_moniker}/tags/list" \
      > "${z_tags_file}" 2>/dev/null || continue

    # Skip if error response
    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      continue
    fi

    # Emit locators: moniker:tag
    jq -r '.tags[]? // empty' "${z_tags_file}" | sort | while read -r z_tag; do
      echo "${z_moniker}:${z_tag}"
    done
  done < "${z_packages_list}"

  # Advisory quota check -- warn if insufficient but do not block listing
  zrbf_quota_preflight "${z_token}" "advisory"

  buc_success "List complete"
}

rbf_beseech() {
  zrbf_sentinel

  # Documentation block
  buc_doc_brief "Petition registry to reveal consecrated arks, correlating artifact pairs by shared consecration timestamp"
  buc_doc_param "<vessel>" "Optional vessel filter - show only arks for this vessel"
  buc_doc_shown || return 0

  # Optional vessel filter (strip path prefix — accept directory path or bare moniker)
  local z_filter_vessel="${1:-}"
  z_filter_vessel="${z_filter_vessel##*/}"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Enumerating arks from repository"

  local z_packages_file="${BURD_TEMP_DIR}/rbf_beseech_packages.json"
  local z_gar_api="https://artifactregistry.googleapis.com/v1"
  local z_repo_path="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${z_gar_api}/${z_repo_path}/packages" \
    > "${z_packages_file}" || buc_die "Failed to fetch packages"

  # Check for error response
  if jq -e '.error' "${z_packages_file}" >/dev/null 2>&1; then
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${z_packages_file}")
    buc_die "API error: ${z_error}"
  fi

  # Extract package monikers, optionally filter
  local z_packages_list="${BURD_TEMP_DIR}/rbf_beseech_packages.txt"
  if [[ -n "${z_filter_vessel}" ]]; then
    jq -r '.packages[]?.name // empty' "${z_packages_file}" | while read -r pkg; do
      local moniker="${pkg##*/}"
      if [[ "${moniker}" == "${z_filter_vessel}" ]]; then
        echo "${moniker}"
      fi
    done | sort > "${z_packages_list}"
  else
    jq -r '.packages[]?.name // empty' "${z_packages_file}" | while read -r pkg; do
      echo "${pkg##*/}"
    done | sort > "${z_packages_list}"
  fi

  # Build ark correlation data
  local z_arks_raw="${BURD_TEMP_DIR}/rbf_beseech_arks_raw.txt"
  > "${z_arks_raw}"  # Clear file

  while IFS= read -r z_moniker; do
    local z_tags_file="${BURD_TEMP_DIR}/rbf_beseech_tags_${z_moniker}.json"
    curl -sL \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
      --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
      -H "Authorization: Bearer ${z_token}" \
      "${ZRBF_REGISTRY_API_BASE}/${z_moniker}/tags/list" \
      > "${z_tags_file}" 2>/dev/null || continue

    # Skip if error response
    if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
      continue
    fi

    # Process tags: extract consecrations and artifact types
    jq -r '.tags[]? // empty' "${z_tags_file}" | while read -r z_tag; do
      # Check for -image suffix
      if [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_IMAGE}" ]]; then
        local z_consecration="${z_tag%${RBGC_ARK_SUFFIX_IMAGE}}"
        echo "${z_moniker}|${z_consecration}|image"
      # Check for -about suffix
      elif [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_ABOUT}" ]]; then
        local z_consecration="${z_tag%${RBGC_ARK_SUFFIX_ABOUT}}"
        echo "${z_moniker}|${z_consecration}|about"
      fi
      # Skip tags that are neither (plain image tags)
    done >> "${z_arks_raw}"
  done < "${z_packages_list}"

  # Correlate artifacts into arks
  local z_arks_correlated="${BURD_TEMP_DIR}/rbf_beseech_arks_correlated.txt"
  sort "${z_arks_raw}" | awk -F'|' '
    {
      key = $1 "|" $2
      if ($3 == "image") {
        has_image[key] = 1
      } else if ($3 == "about") {
        has_about[key] = 1
      }
      seen[key] = 1
    }
    END {
      for (k in seen) {
        split(k, parts, "|")
        vessel = parts[1]
        consecration = parts[2]
        image_mark = (has_image[k] ? "✓" : "✗")
        about_mark = (has_about[k] ? "✓" : "✗")
        printf "%-30s %-20s %-8s %-8s\n", vessel, consecration, image_mark, about_mark
      }
    }
  ' | sort -k1,1 -k2,2r > "${z_arks_correlated}"

  # Display header and results
  printf "%-30s %-20s %-8s %-8s\n" "VESSEL" "CONSECRATION" "-image" "-about"
  cat "${z_arks_correlated}"

  buc_success "Beseech complete"
}

rbf_retrieve() {
  zrbf_sentinel

  local z_locator="${1:-}"

  # Documentation block
  buc_doc_brief "Pull an image from the registry to local container runtime by locator"
  buc_doc_param "locator" "Image locator in moniker:tag format (from rbf_list output)"
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

  # Prefer Retriever credentials, fallback to Director
  local z_rbra_file=""
  if test -n "${RBDC_RETRIEVER_RBRA_FILE:-}" && test -f "${RBDC_RETRIEVER_RBRA_FILE}"; then
    z_rbra_file="${RBDC_RETRIEVER_RBRA_FILE}"
    buc_info "Using Retriever credentials"
  else
    z_rbra_file="${RBDC_DIRECTOR_RBRA_FILE}"
    buc_info "Retriever not configured, using Director credentials"
  fi

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || buc_die "Failed to get OAuth token"

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
  z_image_id=$(docker inspect --format='{{.Id}}' "${z_full_ref}" 2>/dev/null) \
    || buc_die "Failed to get image ID"

  # Display results
  echo ""
  echo "Image retrieved: ${z_full_ref}"
  echo "Local image ID: ${z_image_id}"

  buc_success "Image pull complete"
}

rbf_summon() {
  zrbf_sentinel

  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # strip path prefix — accept directory path or bare moniker
  local z_consecration="${2:-}"

  # Documentation block
  buc_doc_brief "Summon an ark (pull both -image and -about artifacts as a coherent unit)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "consecration" "Full consecration (e.g., i20260305_133650-b20260305_160530)"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  test -n "${z_vessel}" || buc_die "Vessel parameter required"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_step "Authenticating for retrieval"

  # Prefer Retriever credentials, fallback to Director
  local z_rbra_file=""
  if test -n "${RBDC_RETRIEVER_RBRA_FILE:-}" && test -f "${RBDC_RETRIEVER_RBRA_FILE}"; then
    z_rbra_file="${RBDC_RETRIEVER_RBRA_FILE}"
    buc_info "Using Retriever credentials"
  else
    z_rbra_file="${RBDC_DIRECTOR_RBRA_FILE}"
    buc_info "Retriever not configured, using Director credentials"
  fi

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || buc_die "Failed to get OAuth token"

  # Construct ark tags — both use full consecration
  local z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"

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

  # Evaluate ark state
  if test "${z_image_exists}" = "false" && test "${z_about_exists}" = "false"; then
    buc_die "Ark not found: neither -image nor -about exists"
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

  # Display results
  echo ""
  buc_success "Ark summoned: ${z_vessel}/${z_consecration}"
  if test "${z_image_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_image_tag} retrieved"
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_about_tag} retrieved"
  fi
}

rbf_rubric_inscribe() {
  zrbf_sentinel

  buc_doc_brief "Inscribe all conjure vessel build definitions to rubric repo and ensure triggers"
  buc_doc_shown || return 0

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
  z_gitlab_token_value=$(printf '%s' "${z_token_b64}" | base64 -d) \
    || buc_die "Failed to decode token"

  # Construct authenticated URL: https://gitlab.com/... → https://oauth2:TOKEN@gitlab.com/...
  local -r z_rubric_auth_url="https://oauth2:${z_gitlab_token_value}@${RBRR_RUBRIC_REPO_URL#https://}"

  # RBRR_RUBRIC_REPO_URL is already clean (no embedded PAT)
  local -r z_rubric_url_clean="${RBRR_RUBRIC_REPO_URL}"

  # Phase 0: Pin staleness gate
  buc_step "Checking GCB pin freshness"
  local -r z_now_epoch="${BURD_NOW_EPOCH}"
  local -r z_pins_epoch="${RBRG_IMAGE_PINS_REFRESHED_AT:-0}"
  local -r z_age=$((z_now_epoch - z_pins_epoch))
  if test "${z_age}" -gt "${ZRBF_INSCRIBE_STALENESS_SEC}"; then
    buc_warn "GCB pins are stale (${z_age}s old, limit ${ZRBF_INSCRIBE_STALENESS_SEC}s)"
    buc_info "Refresh pins first, commit, then re-run inscribe:"
    buc_tabtarget "${RBZ_REFRESH_GCB_PINS}"
    buc_die "Cannot inscribe with stale pins"
  fi
  buc_info "Pin freshness verified (${z_age}s old)"

  # Phase 1: Capture git metadata for build substitutions
  buc_step "Capturing git metadata for build substitutions"
  local z_git_commit=""
  local z_git_branch=""
  local z_git_remote=""
  local z_git_repo_url=""
  local z_git_repo=""
  local z_git_remote_file="${ZRBF_INSCRIBE_PREFIX}git_remote.txt"
  z_git_commit=$(git rev-parse HEAD) || buc_die "Failed to get commit SHA"
  z_git_branch=$(git rev-parse --abbrev-ref HEAD) || buc_die "Failed to get branch"
  git remote > "${z_git_remote_file}" || buc_die "Failed to list git remotes"
  z_git_remote=$(head -1 "${z_git_remote_file}")
  test -n "${z_git_remote}" || buc_die "No git remotes found"
  z_git_repo_url=$(git config --get "remote.${z_git_remote}.url") || buc_die "Failed to get repo URL"
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

  # Phase 2: Generate all rubric JSON
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

  buc_step "Generating rubric JSON for all conjure vessels"
  local z_target=""
  local z_loaded_sigil=""
  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    # Isolation subshell: each vessel's rbrv.env sets different RBRV_* values that
    # would collide without isolation. Outputs survive via filesystem:
    # ZRBF_VESSEL_SIGIL_FILE and ZRBF_STITCHED_BUILD_FILE.
    (
      zrbf_load_vessel "${z_vessel_dir}" || buc_die "Failed to load vessel: ${z_vessel_dir}"
      buc_step "Stitching build JSON for ${RBRV_SIGIL}"
      zrbf_stitch_build_json || buc_die "Failed to stitch build JSON for ${RBRV_SIGIL}"
    ) || buc_die "Isolation subshell failed for vessel: ${z_vessel_dir}"

    z_loaded_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}") || buc_die "Failed to read vessel sigil after isolation subshell"
    z_vessel_sigils+=("${z_loaded_sigil}")

    z_target="${z_vessel_dir}/cloudbuild.json"
    cp "${ZRBF_STITCHED_BUILD_FILE}" "${z_target}" \
      || buc_die "Failed to copy stitched JSON to ${z_target}"
    buc_info "Generated: ${z_target}"
  done

  # Phase 3: Verify all committed
  buc_step "Verifying all generated JSON matches committed copies"
  local z_stale_vessels=""
  local z_sigil=""
  local z_json_path=""
  local z_idx=0
  local z_ls_stderr="${ZRBF_INSCRIBE_PREFIX}ls_stderr.txt"
  local z_diff_stderr="${ZRBF_INSCRIBE_PREFIX}diff_stderr.txt"

  for ((z_idx=0; z_idx<${#z_vessel_dirs[@]}; z_idx++)); do
    z_json_path="${z_vessel_dirs[z_idx]}/cloudbuild.json"

    # Check if file is tracked by git
    if ! git ls-files --error-unmatch "${z_json_path}" >/dev/null 2>"${z_ls_stderr}"; then
      z_stale_vessels="${z_stale_vessels}  ${z_json_path} (NEW — not yet committed)\n"
      continue
    fi
    # Compare working copy against HEAD
    if ! git diff --quiet HEAD -- "${z_json_path}" 2>"${z_diff_stderr}"; then
      z_stale_vessels="${z_stale_vessels}  ${z_json_path} (modified — not committed)\n"
    fi
  done

  if test -n "${z_stale_vessels}"; then
    buc_warn "The following vessel JSON files differ from committed copies:"
    printf "%b" "${z_stale_vessels}"
    buc_die "Review diffs, commit, and re-run inscribe"
  fi
  buc_step "All vessel JSON files match committed copies"

  # Phase 4: Fresh clone and sync to rubric repo
  buc_step "Cloning rubric repo (always-fresh)"
  rm -rf "${ZRBF_INSCRIBE_CLONE_DIR}" || buc_die "Failed to remove old rubric clone"
  mkdir -p "${ZRBF_INSCRIBE_CLONE_DIR%/*}" || buc_die "Failed to create clone parent directory"

  git clone --depth 1 "${z_rubric_auth_url}" "${ZRBF_INSCRIBE_CLONE_DIR}" \
    || buc_die "Failed to clone rubric repo"
  buc_info "Rubric repo cloned to ${ZRBF_INSCRIBE_CLONE_DIR}"

  # Copy per-vessel directories to rubric clone
  # Rubric per-vessel dir = vessel dir minus rbrv.env (regime config, not build material)
  buc_step "Syncing vessel build material to rubric repo clone"
  local z_rubric_vessel_dir=""
  for ((z_idx=0; z_idx<${#z_vessel_dirs[@]}; z_idx++)); do
    z_vessel_dir="${z_vessel_dirs[z_idx]}"
    z_sigil="${z_vessel_sigils[z_idx]}"
    z_rubric_vessel_dir="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}"

    rm -rf "${z_rubric_vessel_dir}"
    cp -R "${z_vessel_dir}" "${z_rubric_vessel_dir}" \
      || buc_die "Failed to copy vessel directory for ${z_sigil}"
    rm -f "${z_rubric_vessel_dir}/rbrv.env"

    test -f "${z_rubric_vessel_dir}/cloudbuild.json" \
      || buc_die "cloudbuild.json missing after copy for ${z_sigil}"
    test -f "${z_rubric_vessel_dir}/Dockerfile" \
      || buc_die "Dockerfile missing after copy for ${z_sigil}"

    buc_info "Synced: ${z_sigil}"
  done

  # Fill pre-commit placeholders in rubric clone copies
  # _RBGY_RUBRIC_REPO, _RBGY_GIT_COMMIT, _RBGY_INSCRIBE_TIMESTAMP filled now;
  # _RBGY_RUBRIC_COMMIT filled after rubric repo commit (needs content hash)
  buc_step "Filling pre-commit placeholders in clone copies"
  local z_rubric_json=""
  local z_filled_file=""
  local z_inscribe_ts="i${BURD_NOW_STAMP:0:8}_${BURD_NOW_STAMP:9:6}"
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
       | .images = [.images[] | gsub("__INSCRIBE_TIMESTAMP__"; $inscribe_ts)]' \
      "${z_rubric_json}" > "${z_filled_file}" \
      || buc_die "Failed to fill pre-commit placeholders for ${z_sigil}"
    mv "${z_filled_file}" "${z_rubric_json}" \
      || buc_die "Failed to write pre-commit-filled JSON for ${z_sigil}"
  done

  # Commit (not pushed yet) to get a content hash
  buc_step "Committing rubric repo changes"
  local z_inscribe_ts
  z_inscribe_ts="i${BURD_NOW_STAMP:0:8}_${BURD_NOW_STAMP:9:6}"

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
    z_content_commit=$(git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD) \
      || buc_die "Failed to get content commit hash"

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
    z_rubric_commit=$(git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD) \
      || buc_die "Failed to get final rubric commit"
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
    || buc_die "CB v2 repository '${RBGC_CBV2_REPOSITORY_ID}' not found (HTTP ${z_cbv2_repo_code}) — run depot_create to establish CB v2 connection"
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

  local -r z_vessel_dir="${1:-}"
  local z_consecration="${2:-}"
  local z_force="${3:-}"

  # Documentation block
  buc_doc_brief "Abjure an ark (delete all per-platform image, about, and vouch artifacts)"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_param "consecration" "Full consecration (e.g., i20260305_133650-b20260305_160530)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
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

  # Load vessel
  zrbf_load_vessel "${z_vessel_dir}"

  # Validate remaining parameters
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  # Derive inscribe timestamp from full consecration (needed for -multi intermediate tag)
  local -r z_inscribe_ts="${z_consecration%%-b*}"
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

  # Compute platform suffixes from vessel config
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
    test "${z_remaining_plats}" = "${z_plat}" && break
    z_remaining_plats="${z_remaining_plats#*,}"
  done

  # Build list of image tags to check/delete:
  # - Per-platform suffixed tags use full consecration
  # - Consumer-facing bare tag (multi-platform manifest list only)
  # - Intermediate -multi tag uses inscribe TS only (multi-platform only)
  local z_image_tags=()
  local z_idx=0
  for z_idx in "${!z_platform_suffixes[@]}"; do
    z_image_tags+=("${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}${z_platform_suffixes[$z_idx]}")
  done
  if test "${#z_platform_suffixes[@]}" -gt 1; then
    z_image_tags+=("${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}")
    z_image_tags+=("${z_inscribe_ts}-multi")
  fi

  # About and vouch tags use full consecration
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"

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

  # Evaluate ark state
  if test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "false"; then
    buc_die "Ark not found: no image tags and no -about exists for ${RBRV_SIGIL}/${z_consecration}"
  fi

  if test "${#z_existing_image_tags[@]}" -gt 0 && test "${z_about_exists}" = "false"; then
    buc_warn "Orphaned artifact detected: image tags exist but -about is missing"
  elif test "${#z_existing_image_tags[@]}" -eq 0 && test "${z_about_exists}" = "true"; then
    buc_warn "Orphaned artifact detected: -about exists but no image tags found"
  fi

  # Confirm abjuration unless --force
  if test "${z_skip_confirm}" = "false"; then
    local z_confirm_msg="Will abjure ark ${RBRV_SIGIL}/${z_consecration}:"
    for z_img_tag in "${z_existing_image_tags[@]}"; do
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_img_tag}"
    done
    if test "${z_about_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_about_tag}"
    fi
    if test "${z_vouch_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${RBRV_SIGIL}:${z_vouch_tag}"
    fi
    buc_require "${z_confirm_msg}" "yes"
  fi

  # Delete all existing image tags
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
      buc_warn "Response body: $(cat "${z_delete_img_response}" 2>/dev/null || echo 'empty')"
      buc_die "Failed to delete image tag ${z_img_tag} (HTTP ${z_delete_img_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_img_tag}"
    z_img_del_idx=$((z_img_del_idx + 1))
  done

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
      buc_warn "Response body: $(cat "${z_delete_about_response}" 2>/dev/null || echo 'empty')"
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
      buc_warn "Response body: $(cat "${z_delete_vouch_response}" 2>/dev/null || echo 'empty')"
      buc_die "Failed to delete -vouch artifact (HTTP ${z_delete_vouch_code})"
    fi

    buc_info "Deleted: ${RBRV_SIGIL}:${z_vouch_tag}"
  fi

  # Display results
  echo ""
  buc_success "Ark abjured: ${RBRV_SIGIL}/${z_consecration}"
  for z_img_tag in "${z_existing_image_tags[@]}"; do
    echo "  - ${RBRV_SIGIL}:${z_img_tag} deleted"
  done
  if test "${z_about_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_about_tag} deleted"
  fi
  if test "${z_vouch_exists}" = "true"; then
    echo "  - ${RBRV_SIGIL}:${z_vouch_tag} deleted"
  fi
}

######################################################################
# Consecration Check (rbw-Dc)

rbf_check_consecrations() {
  zrbf_sentinel

  local -r z_vessel_dir="${1:-}"

  buc_doc_brief "List consecrations for a vessel by querying GAR tags"
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

  # Load vessel
  zrbf_load_vessel "${z_vessel_dir}"

  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Querying GAR tags for ${RBRV_SIGIL}"
  local -r z_tags_file="${BURD_TEMP_DIR}/rbf_dc_tags.json"
  curl -sL \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/tags/list" \
    > "${z_tags_file}" 2>/dev/null || buc_die "Failed to fetch tags"

  if jq -e '.errors' "${z_tags_file}" >/dev/null 2>&1; then
    local z_err
    z_err=$(jq -r '.errors[0].message // "Unknown error"' "${z_tags_file}")
    buc_die "Registry API error: ${z_err}"
  fi

  # Extract tags and identify full consecrations
  # Full consecration format: iYYYYMMDD_HHMMSS-bYYYYMMDD_HHMMSS
  # Tags: {CONSECRATION}-image[-suffix], {CONSECRATION}-about, {INSCRIBE_TS}-multi
  local -r z_all_tags_file="${BURD_TEMP_DIR}/rbf_dc_all_tags.txt"
  jq -r '.tags[]? // empty' "${z_tags_file}" > "${z_all_tags_file}"

  # Collect unique full consecrations and classify tags
  local -r z_consecrations_file="${BURD_TEMP_DIR}/rbf_dc_consecrations.txt"
  local -r z_tag_data_file="${BURD_TEMP_DIR}/rbf_dc_tag_data.txt"
  > "${z_consecrations_file}"
  > "${z_tag_data_file}"

  while IFS= read -r z_tag || test -n "${z_tag}"; do
    # Match full consecration: iYYYYMMDD_HHMMSS-bYYYYMMDD_HHMMSS
    local z_consec=""
    if [[ "${z_tag}" =~ ^(i[0-9]{8}_[0-9]{6}-b[0-9]{8}_[0-9]{6}) ]]; then
      z_consec="${BASH_REMATCH[1]}"
    else
      continue
    fi

    # Classify the tag — vouch checked before about since -vouch could false-match -about pattern
    if [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_IMAGE}" ]] || [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_IMAGE}"-* ]]; then
      local z_suffix="${z_tag#*${RBGC_ARK_SUFFIX_IMAGE}}"
      if test -z "${z_suffix}"; then
        echo "${z_consec}|image|consumer" >> "${z_tag_data_file}"
      else
        echo "${z_consec}|image|${z_suffix#-}" >> "${z_tag_data_file}"
      fi
      echo "${z_consec}" >> "${z_consecrations_file}"
    elif [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_VOUCH}" ]]; then
      echo "${z_consec}|vouch|" >> "${z_tag_data_file}"
      echo "${z_consec}" >> "${z_consecrations_file}"
    elif [[ "${z_tag}" == *"${RBGC_ARK_SUFFIX_ABOUT}" ]]; then
      echo "${z_consec}|about|" >> "${z_tag_data_file}"
      echo "${z_consec}" >> "${z_consecrations_file}"
    fi
  done < "${z_all_tags_file}"

  # Deduplicate and sort consecrations (newest first)
  local -r z_unique_consec_file="${BURD_TEMP_DIR}/rbf_dc_unique_consec.txt"
  sort -ur "${z_consecrations_file}" > "${z_unique_consec_file}"

  if ! test -s "${z_unique_consec_file}"; then
    buc_info "No consecrations found for ${RBRV_SIGIL}"
    return 0
  fi

  # Display each consecration with its artifacts
  printf "\nVessel: %s\n" "${RBRV_SIGIL}"
  printf "  %-42s %-30s %-6s %s\n" "Consecration" "Platforms" "About" "Vouch"

  while IFS= read -r z_consecration || test -n "${z_consecration}"; do
    # Collect platform-suffixed image tags, about, and vouch status for this consecration
    local z_consec_platforms=""
    local z_has_about="no"
    local z_has_vouch="no"
    local z_image_count=0
    while IFS='|' read -r z_c z_type z_detail; do
      test "${z_c}" = "${z_consecration}" || continue
      if test "${z_type}" = "image" && test "${z_detail}" != "consumer"; then
        if test -n "${z_consec_platforms}"; then
          z_consec_platforms="${z_consec_platforms},${z_detail}"
        else
          z_consec_platforms="${z_detail}"
        fi
        z_image_count=$((z_image_count + 1))
      elif test "${z_type}" = "about"; then
        z_has_about="yes"
      elif test "${z_type}" = "vouch"; then
        z_has_vouch="yes"
      fi
    done < "${z_tag_data_file}"

    local z_plat_display="${z_consec_platforms:-single}"

    printf "  %-42s %-30s %-6s %s\n" "${z_consecration}" "${z_plat_display}" "${z_has_about}" "${z_has_vouch}"
  done < "${z_unique_consec_file}"

  echo ""
  buc_success "Consecration check complete"
}

######################################################################
# Vouch (rbw-Rv)

rbf_vouch() {
  zrbf_sentinel

  local -r z_vessel_dir="${1:-}"

  buc_doc_brief "Vouch for a vessel consecration via SLSA verification in Cloud Build"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_param "consecration" "Full consecration (e.g., i20260305_133650-b20260305_160530)"
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

  # Load vessel
  zrbf_load_vessel "${z_vessel_dir}"

  local -r z_consecration="${2:-}"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_info "Vouching consecration: ${z_consecration}"

  # Auth as Director
  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Verify -about exists (vouch depends on about)
  buc_step "Verifying -about artifact exists (prerequisite)"
  local -r z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"
  local -r z_vouch_about_status="${BURD_TEMP_DIR}/rbf_vouch_about_status.txt"
  local -r z_vouch_about_resp="${BURD_TEMP_DIR}/rbf_vouch_about_response.json"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_vouch_about_resp}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_about_tag}" \
    > "${z_vouch_about_status}" || buc_die "HEAD request failed for -about artifact"

  local z_about_code=""
  z_about_code=$(<"${z_vouch_about_status}")
  test -n "${z_about_code}" || buc_die "HTTP status code is empty for -about check"
  test "${z_about_code}" = "200" \
    || buc_die "About artifact not found (HTTP ${z_about_code}): ${RBRV_SIGIL}:${z_about_tag} — conjure must complete before vouch"

  # Check if -vouch already exists (for idempotent re-vouch)
  local -r z_vouch_tag="${z_consecration}${RBGC_ARK_SUFFIX_VOUCH}"
  local -r z_vouch_exist_status="${BURD_TEMP_DIR}/rbf_vouch_exist_status.txt"
  local -r z_vouch_exist_resp="${BURD_TEMP_DIR}/rbf_vouch_exist_response.json"

  curl --head -s \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBF_ACCEPT_MANIFEST_MTYPES}" \
    -w "%{http_code}" \
    -o "${z_vouch_exist_resp}" \
    "${ZRBF_REGISTRY_API_BASE}/${RBRV_SIGIL}/manifests/${z_vouch_tag}" \
    > "${z_vouch_exist_status}" || buc_die "HEAD request failed for -vouch artifact"

  local z_exist_vouch_code=""
  z_exist_vouch_code=$(<"${z_vouch_exist_status}")
  test -n "${z_exist_vouch_code}" || buc_die "HTTP status code is empty for -vouch check"
  if test "${z_exist_vouch_code}" = "200"; then
    buc_warn "Vouch artifact already exists — re-vouching (will verify idempotency)"
  fi

  # Compute platform suffixes from vessel config
  local z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"
  local z_platform_suffixes=""
  local z_remaining_plats="${z_platforms}"
  local z_plat=""
  local z_suffix=""
  while test -n "${z_remaining_plats}"; do
    z_plat="${z_remaining_plats%%,*}"
    z_suffix="${z_plat#linux/}"
    z_suffix="${z_suffix//\//}"
    z_suffix="-${z_suffix}"
    if test -n "${z_platform_suffixes}"; then
      z_platform_suffixes="${z_platform_suffixes},${z_suffix}"
    else
      z_platform_suffixes="${z_suffix}"
    fi
    test "${z_remaining_plats}" = "${z_plat}" && break
    z_remaining_plats="${z_remaining_plats#*,}"
  done
  buc_info "Platform suffixes: ${z_platform_suffixes}"

  # Construct vouch Cloud Build job
  # Three steps: download-verifier (Alpine), verify-provenance (gcloud), push-vouch (Docker)
  # Steps are written to temp files, escaped for GCB substitution, then stitched into Build JSON.
  buc_step "Constructing vouch Cloud Build job"

  local -r z_vp="${BURD_TEMP_DIR}/rbf_vouch_"

  # Step 1: Download and verify slsa-verifier binary
  local -r z_step1_file="${z_vp}step1.sh"
  local -r z_step1_escaped="${z_vp}step1_escaped.sh"
  cat > "${z_step1_file}" <<'VOUCHSTEP1'
set -euo pipefail
echo "=== Downloading slsa-verifier ==="
wget -q "${_RBGV_SLSA_VERIFIER_URL}" -O /workspace/slsa-verifier
COMPUTED=$(sha256sum /workspace/slsa-verifier | cut -d' ' -f1)
echo "Expected: ${_RBGV_SLSA_VERIFIER_SHA256}"
echo "Computed: ${COMPUTED}"
test "${COMPUTED}" = "${_RBGV_SLSA_VERIFIER_SHA256}" || { echo "CHECKSUM MISMATCH"; exit 1; }
chmod +x /workspace/slsa-verifier
/workspace/slsa-verifier version || true
echo "slsa-verifier ready"
VOUCHSTEP1

  # Step 2: Configure auth and run slsa-verifier verify-image for each platform
  local -r z_step2_file="${z_vp}step2.sh"
  local -r z_step2_escaped="${z_vp}step2_escaped.sh"
  cat > "${z_step2_file}" <<'VOUCHSTEP2'
set -euo pipefail
GAR_HOST="${_RBGV_GAR_LOCATION}${_RBGV_GAR_HOST_SUFFIX}"
REGISTRY="${GAR_HOST}/${_RBGV_GAR_PROJECT}/${_RBGV_GAR_REPOSITORY}"

echo "=== Configuring registry auth ==="
export DOCKER_CONFIG=/workspace/.docker
mkdir -p "${DOCKER_CONFIG}"
gcloud auth configure-docker "${GAR_HOST}" --quiet

echo "=== Verifying SLSA provenance per platform ==="
RESULT_DIR="/workspace/vouch_results"
mkdir -p "${RESULT_DIR}"

IFS=',' read -ra SUFFIXES <<< "${_RBGV_PLATFORM_SUFFIXES}"
for SUFFIX in "${SUFFIXES[@]}"; do
  TAG="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}${SUFFIX}"
  URI="${REGISTRY}/${_RBGV_MONIKER}:${TAG}"
  echo "--- Verifying: ${URI} ---"
  RESULT_FILE="${RESULT_DIR}/verify${SUFFIX}.json"
  /workspace/slsa-verifier verify-image "${URI}" \
    --source-uri "${_RBGV_SOURCE_URI}" \
    --print-provenance \
    > "${RESULT_FILE}" 2>&1
  echo "Passed: ${SUFFIX}"
done

echo "=== Assembling vouch summary ==="
{
  echo "{"
  echo "  \"consecration\": \"${_RBGV_CONSECRATION}\","
  echo "  \"vessel\": \"${_RBGV_MONIKER}\","
  echo "  \"verifier\": {"
  echo "    \"url\": \"${_RBGV_SLSA_VERIFIER_URL}\","
  echo "    \"sha256\": \"${_RBGV_SLSA_VERIFIER_SHA256}\""
  echo "  },"
  echo "  \"platforms\": ["
  FIRST=true
  for SUFFIX in "${SUFFIXES[@]}"; do
    if test "${FIRST}" != "true"; then printf ",\n"; fi
    FIRST=false
    printf "    {\"suffix\": \"%s\", \"verified\": true}" "${SUFFIX}"
  done
  echo ""
  echo "  ]"
  echo "}"
} > "${RESULT_DIR}/vouch_summary.json"

echo "All platform verifications passed"
VOUCHSTEP2

  # Step 3: Build scratch container with vouch data and push as -vouch tag
  local -r z_step3_file="${z_vp}step3.sh"
  local -r z_step3_escaped="${z_vp}step3_escaped.sh"
  cat > "${z_step3_file}" <<'VOUCHSTEP3'
set -euo pipefail
GAR_HOST="${_RBGV_GAR_LOCATION}${_RBGV_GAR_HOST_SUFFIX}"
REGISTRY="${GAR_HOST}/${_RBGV_GAR_PROJECT}/${_RBGV_GAR_REPOSITORY}"
VOUCH_TAG="${REGISTRY}/${_RBGV_MONIKER}:${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_VOUCH}"

cd /workspace/vouch_results

echo "=== Building vouch container ==="
{
  echo "FROM scratch"
  echo "LABEL org.opencontainers.image.title=\"rbia-vouch\""
  echo "COPY vouch_summary.json /vouch_summary.json"
} > Dockerfile.vouch

for f in verify-*.json; do
  test -f "${f}" || continue
  echo "COPY ${f} /${f}" >> Dockerfile.vouch
done

docker build -t "${VOUCH_TAG}" -f Dockerfile.vouch .

echo "=== Pushing vouch container ==="
docker push "${VOUCH_TAG}"

echo "Vouch pushed: ${VOUCH_TAG}"
VOUCHSTEP3

  # Escape step scripts for GCB: $$ for literal $, preserve _RBGV_ substitutions
  buc_log_args "Escaping step scripts for GCB substitution"
  sed 's/\$/\$\$/g; s/\$\${_RBGV_/${_RBGV_/g' < "${z_step1_file}" > "${z_step1_escaped}" \
    || buc_die "Failed to escape step 1"
  sed 's/\$/\$\$/g; s/\$\${_RBGV_/${_RBGV_/g' < "${z_step2_file}" > "${z_step2_escaped}" \
    || buc_die "Failed to escape step 2"
  sed 's/\$/\$\$/g; s/\$\${_RBGV_/${_RBGV_/g' < "${z_step3_file}" > "${z_step3_escaped}" \
    || buc_die "Failed to escape step 3"

  # Assemble Build resource JSON
  buc_log_args "Assembling Build resource JSON"
  local -r z_build_file="${z_vp}build.json"
  local -r z_source_uri="${RBRR_RUBRIC_REPO_URL:-}"
  test -n "${z_source_uri}" || buc_die "RBRR_RUBRIC_REPO_URL not set — required for SLSA source verification"

  jq -n \
    --rawfile zjq_step1 "${z_step1_escaped}" \
    --rawfile zjq_step2 "${z_step2_escaped}" \
    --rawfile zjq_step3 "${z_step3_escaped}" \
    --arg zjq_alpine   "${RBRG_ALPINE_IMAGE_REF}" \
    --arg zjq_gcloud   "${RBRG_GCLOUD_IMAGE_REF}" \
    --arg zjq_docker   "${RBRG_DOCKER_IMAGE_REF}" \
    --arg zjq_moniker  "${RBRV_SIGIL}" \
    --arg zjq_consec   "${z_consecration}" \
    --arg zjq_gar_loc  "${RBGD_GAR_LOCATION}" \
    --arg zjq_gar_proj "${RBGD_GAR_PROJECT_ID}" \
    --arg zjq_gar_repo "${RBRR_GAR_REPOSITORY}" \
    --arg zjq_gar_host "${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_plat_sfx "${z_platform_suffixes}" \
    --arg zjq_ver_url  "${RBRG_SLSA_VERIFIER_URL}" \
    --arg zjq_ver_sha  "${RBRG_SLSA_VERIFIER_SHA256}" \
    --arg zjq_sfx_vouch "${RBGC_ARK_SUFFIX_VOUCH}" \
    --arg zjq_sfx_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_sfx_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_src_uri   "${z_source_uri}" \
    --arg zjq_pool      "${RBRR_GCB_WORKER_POOL}" \
    --arg zjq_timeout   "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: [
        {
          name: $zjq_alpine,
          id: "download-verifier",
          entrypoint: "/bin/sh",
          args: ["-c", $zjq_step1]
        },
        {
          name: $zjq_gcloud,
          id: "verify-provenance",
          entrypoint: "/bin/bash",
          args: ["-lc", $zjq_step2]
        },
        {
          name: $zjq_docker,
          id: "push-vouch",
          entrypoint: "/bin/bash",
          args: ["-lc", $zjq_step3]
        }
      ],
      substitutions: {
        _RBGV_MONIKER:              $zjq_moniker,
        _RBGV_CONSECRATION:         $zjq_consec,
        _RBGV_GAR_LOCATION:         $zjq_gar_loc,
        _RBGV_GAR_PROJECT:          $zjq_gar_proj,
        _RBGV_GAR_REPOSITORY:       $zjq_gar_repo,
        _RBGV_GAR_HOST_SUFFIX:      $zjq_gar_host,
        _RBGV_PLATFORM_SUFFIXES:    $zjq_plat_sfx,
        _RBGV_SLSA_VERIFIER_URL:    $zjq_ver_url,
        _RBGV_SLSA_VERIFIER_SHA256: $zjq_ver_sha,
        _RBGV_ARK_SUFFIX_VOUCH:     $zjq_sfx_vouch,
        _RBGV_ARK_SUFFIX_ABOUT:     $zjq_sfx_about,
        _RBGV_ARK_SUFFIX_IMAGE:     $zjq_sfx_image,
        _RBGV_SOURCE_URI:           $zjq_src_uri
      },
      options: {
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose vouch Build JSON"

  # Submit vouch build via RunBuild API (direct build, not trigger dispatch)
  buc_step "Submitting vouch build to Cloud Build"
  rbgu_http_json "POST" "${ZRBF_GCB_PROJECT_BUILDS_URL}" "${z_token}" "vouch_build_submit" "${z_build_file}"
  rbgu_http_require_ok "Vouch build submission" "vouch_build_submit"

  # Extract build ID from Operation response
  local z_build_id=""
  z_build_id=$(rbgu_json_field_capture "vouch_build_submit" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in vouch build response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist vouch build ID"

  local -r z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Vouch build dispatched: ${z_build_id}"
  buc_link "Click to " "Open vouch build in Cloud Console" "${z_console_url}"

  # Wait for completion (reuses existing build wait infrastructure)
  zrbf_wait_build_completion

  # Write fact files for test harness consumption
  echo "${z_consecration}" > "${BURD_OUTPUT_DIR}/rbf_vouch_consecration.txt" \
    || buc_die "Failed to write vouch consecration fact file"

  buc_success "Vouch complete: ${RBRV_SIGIL}/${z_consecration}"
  buc_info "Vouch artifact: ${RBRV_SIGIL}:${z_vouch_tag}"
}

# eof

