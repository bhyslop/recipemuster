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
  test -n "${RBRR_DIRECTOR_RBRA_FILE:-}" || buc_die "RBRR_DIRECTOR_RBRA_FILE not set"
  test -f "${RBRR_DIRECTOR_RBRA_FILE}"   || buc_die "GCB service env file not found: ${RBRR_DIRECTOR_RBRA_FILE}"

  buc_log_args 'Module Variables (ZRBF_*)'
  readonly ZRBF_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  readonly ZRBF_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  readonly ZRBF_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  readonly ZRBF_GCB_API_BASE_UPLOAD="https://cloudbuild.googleapis.com/upload/v1"

  # GCS endpoints (build-source uploads)
  readonly ZRBF_GCS_API_BASE="https://storage.googleapis.com/storage/v1"
  readonly ZRBF_GCS_UPLOAD_BASE="https://storage.googleapis.com/upload/storage/v1"

  # Temp files for object naming and requests
  readonly ZRBF_TARBALL_NAME_FILE="${BURD_TEMP_DIR}/rbf_tarball_name.txt"
  readonly ZRBF_GCS_OBJECT_FILE="${BURD_TEMP_DIR}/rbf_gcs_object.txt"
  readonly ZRBF_BUILD_REQUEST_FILE="${BURD_TEMP_DIR}/rbf_build_request.json"
  readonly ZRBF_GCS_UPLOAD_RESP="${BURD_TEMP_DIR}/rbf_gcs_upload_resp.json"
  readonly ZRBF_GCS_UPLOAD_HTTP="${BURD_TEMP_DIR}/rbf_gcs_upload_http.txt"

  readonly ZRBF_GCB_PROJECT_BUILDS_URL="${ZRBF_GCB_API_BASE}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  readonly ZRBF_GCB_PROJECT_BUILDS_UPLOAD_URL="${ZRBF_GCB_API_BASE_UPLOAD}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  readonly ZRBF_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

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
  readonly ZRBF_BUILD_CONTEXT_TAR="${BURD_TEMP_DIR}/rbf_build_context.tar.gz"
  readonly ZRBF_BUILD_ID_FILE="${BURD_TEMP_DIR}/rbf_build_id.txt"
  readonly ZRBF_BUILD_STATUS_FILE="${BURD_TEMP_DIR}/rbf_build_status.json"
  readonly ZRBF_BUILD_LOG_FILE="${BURD_TEMP_DIR}/rbf_build_log.txt"
  readonly ZRBF_BUILD_RESPONSE_FILE="${BURD_TEMP_DIR}/rbf_build_response.json"
  readonly ZRBF_BUILD_HTTP_CODE="${BURD_TEMP_DIR}/rbf_build_http_code.txt"

  buc_log_args 'Define copy staging files'
  readonly ZRBF_COPY_STAGING_DIR="${BURD_TEMP_DIR}/rbf_copy_staging"
  readonly ZRBF_COPY_CONTEXT_TAR="${BURD_TEMP_DIR}/rbf_copy_context.tar.gz"

  buc_log_args 'Define git info files'
  readonly ZRBF_GIT_INFO_FILE="${BURD_TEMP_DIR}/rbf_git_info.json"
  readonly ZRBF_GIT_COMMIT_FILE="${BURD_TEMP_DIR}/rbf_git_commit.txt"
  readonly ZRBF_GIT_BRANCH_FILE="${BURD_TEMP_DIR}/rbf_git_branch.txt"
  readonly ZRBF_GIT_REPO_FILE="${BURD_TEMP_DIR}/rbf_git_repo_url.txt"
  readonly ZRBF_GIT_UNTRACKED_FILE="${BURD_TEMP_DIR}/rbf_git_untracked.txt"
  readonly ZRBF_GIT_REMOTE_FILE="${BURD_TEMP_DIR}/rbf_git_remote.txt"

  buc_log_args 'Define staging and size files'
  readonly ZRBF_STAGING_DIR="${BURD_TEMP_DIR}/rbf_staging"
  readonly ZRBF_CONTEXT_SIZE_FILE="${BURD_TEMP_DIR}/rbf_context_size_bytes.txt"

  buc_log_args 'Define validation files'
  readonly ZRBF_STATUS_CHECK_FILE="${BURD_TEMP_DIR}/rbf_status_check.txt"
  readonly ZRBF_BUILD_ID_TMP_FILE="${BURD_TEMP_DIR}/rbf_build_id_tmp.txt"

  buc_log_args 'Define delete operation files'
  readonly ZRBF_DELETE_PREFIX="${BURD_TEMP_DIR}/rbf_delete_"
  readonly ZRBF_TOKEN_FILE="${BURD_TEMP_DIR}/rbf_token.txt"

  buc_log_args 'Define copy operation files'
  readonly ZRBF_COPY_CONFIG_FILE="${BURD_TEMP_DIR}/rbf_copy_config.json"
  readonly ZRBF_COPY_RESPONSE_FILE="${BURD_TEMP_DIR}/rbf_copy_response.json"

  buc_log_args 'Vessel-related files'
  readonly ZRBF_VESSEL_ENV_FILE="${BURD_TEMP_DIR}/rbf_vessel_env.txt"
  readonly ZRBF_VESSEL_SIGIL_FILE="${BURD_TEMP_DIR}/rbf_vessel_sigil.txt"

  buc_log_args 'Define stitch operation file prefix (postfixed per step id)'
  readonly ZRBF_STITCH_PREFIX="${BURD_TEMP_DIR}/rbf_stitch_"

  buc_log_args 'Define inscribe operation files'
  readonly ZRBF_INSCRIBE_PREFIX="${BURD_TEMP_DIR}/rbf_inscribe_"
  readonly ZRBF_INSCRIBE_CLONE_DIR="${RBGC_RUBRIC_CLONE_DIR}"
  readonly ZRBF_INSCRIBE_STALENESS_SEC=86400

  buc_log_args 'For now lets double check these'
  test -n "${RBRR_GCB_ORAS_IMAGE_REF:-}"   || buc_die "RBRR_GCB_ORAS_IMAGE_REF not set"

  readonly ZRBF_KINDLED=1
}

zrbf_sentinel() {
  test "${ZRBF_KINDLED:-}" = "1" || buc_die "Module rbf not kindled - call zrbf_kindle first"
}

zrbf_stitch_build_json() {
  zrbf_sentinel

  buc_log_args 'Stitching trigger-compatible build JSON from step scripts'

  # Preconditions: vessel loaded and git state captured
  test -s "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Vessel not loaded — call zrbf_load_vessel first"
  test -s "${ZRBF_GIT_INFO_FILE}"     || buc_die "Git info not captured — call zrbf_verify_git_clean first"

  buc_log_args 'Read vessel state for substitutions'
  local -r z_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}")
  test -n "${z_sigil}" || buc_die "Empty vessel sigil"
  local -r z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"
  local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"

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
  # Note: Step 05 (buildx-create) was merged into step 06 because Cloud Build
  # steps run in isolated containers - builder state doesn't persist across steps
  #
  # OCI Layout Bridge pattern (steps 06-08):
  # - Step 06: buildx exports to /workspace/oci-layout (no auth needed)
  # - Step 07: crane pushes from oci-layout to GAR (with metadata auth)
  # - Step 08: Syft analyzes from oci-layout (faster, more accurate)
  # - Step 10: Assembles metadata JSON from .image_uri
  # - Step 09: Builds and pushes metadata container (depends on step 10)
  # Delimiter is | because image refs contain colons (sha256 digests)
  # Note: builder images pinned via RBRR_GCB_*_IMAGE_REF variables
  local z_step_defs=(
    "rbgjb01-derive-tag-base.sh|${RBRR_GCB_GCLOUD_IMAGE_REF}|bash|derive-tag-base"
    "rbgjb02-get-docker-token.sh|${RBRR_GCB_GCLOUD_IMAGE_REF}|bash|get-docker-token"
    "rbgjb03-docker-login-gar.sh|${RBRR_GCB_DOCKER_IMAGE_REF}|bash|docker-login-gar"
    "rbgjb04-qemu-binfmt.sh|${RBRR_GCB_DOCKER_IMAGE_REF}|bash|qemu-binfmt"
    "rbgjb06-build-and-export.sh|${RBRR_GCB_DOCKER_IMAGE_REF}|bash|build-and-export"
    "rbgjb07-push-with-crane.sh|${RBRR_GCB_ALPINE_IMAGE_REF}|sh|push-with-crane"
    "rbgjb08-sbom-and-summary.sh|${RBRR_GCB_DOCKER_IMAGE_REF}|bash|sbom-and-summary"
    "rbgjb10-assemble-metadata.sh|${RBRR_GCB_ALPINE_IMAGE_REF}|sh|assemble-metadata"
    "rbgjb09-build-and-push-metadata.sh|${RBRR_GCB_DOCKER_IMAGE_REF}|bash|build-and-push-metadata"
  )

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
    z_body="${z_body//\$\{RBRR_GCB_SYFT_IMAGE_REF\}/${RBRR_GCB_SYFT_IMAGE_REF}}"
    z_body="${z_body//\$\{RBRR_GCB_BINFMT_IMAGE_REF\}/${RBRR_GCB_BINFMT_IMAGE_REF}}"

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
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, entrypoint: $ep, args: [$flag, $script]}]' \
      "${z_accumulator_file}" > "${z_steps_file}" \
      || buc_die "Failed to append step ${z_id} to JSON"
    mv "${z_steps_file}" "${z_accumulator_file}" \
      || buc_die "Failed to update steps JSON for ${z_id}"
  done

  # Compose complete trigger-compatible Build resource
  # Steps from accumulator, substitutions from module state, options/timeout from RBRR
  # _RBGY_RUBRIC_REPO, _RBGY_RUBRIC_COMMIT, _RBGY_INSCRIBE_TIMESTAMP are placeholders
  # in the main-repo copy; inscribe fills them in the rubric repo copy after push
  buc_log_args "Composing complete trigger-compatible Build resource"
  local -r z_build_file="${ZRBF_STITCH_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_accumulator_file}" \
    --arg zjq_dockerfile     "${z_dockerfile_name}" \
    --arg zjq_moniker        "${z_sigil}" \
    --arg zjq_platforms      "${z_platforms}" \
    --arg zjq_gar_location   "${RBGD_GAR_LOCATION}" \
    --arg zjq_gar_project    "${RBGD_GAR_PROJECT_ID}" \
    --arg zjq_gar_repository "${RBRR_GAR_REPOSITORY}" \
    --arg zjq_git_commit     "${z_git_commit}" \
    --arg zjq_git_branch     "${z_git_branch}" \
    --arg zjq_git_repo       "${z_git_repo}" \
    --arg zjq_gar_host_suffix  "${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_ark_suffix_image "${RBGC_ARK_SUFFIX_IMAGE}" \
    --arg zjq_ark_suffix_about "${RBGC_ARK_SUFFIX_ABOUT}" \
    --arg zjq_crane_tar_gz     "${RBRR_CRANE_TAR_GZ}" \
    --arg zjq_rubric_repo      "__INSCRIBE_RUBRIC_REPO__" \
    --arg zjq_rubric_commit    "__INSCRIBE_RUBRIC_COMMIT__" \
    --arg zjq_inscribe_ts      "i${BURD_NOW_STAMP:0:8}_${BURD_NOW_STAMP:9:6}" \
    --arg zjq_mtype  "${RBRR_GCB_MACHINE_TYPE}" \
    --arg zjq_timeout "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGY_DOCKERFILE:          $zjq_dockerfile,
        _RBGY_MONIKER:             $zjq_moniker,
        _RBGY_PLATFORMS:           $zjq_platforms,
        _RBGY_GAR_LOCATION:        $zjq_gar_location,
        _RBGY_GAR_PROJECT:         $zjq_gar_project,
        _RBGY_GAR_REPOSITORY:      $zjq_gar_repository,
        _RBGY_GIT_COMMIT:          $zjq_git_commit,
        _RBGY_GIT_BRANCH:          $zjq_git_branch,
        _RBGY_GIT_REPO:            $zjq_git_repo,
        _RBGY_GAR_HOST_SUFFIX:     $zjq_gar_host_suffix,
        _RBGY_ARK_SUFFIX_IMAGE:    $zjq_ark_suffix_image,
        _RBGY_ARK_SUFFIX_ABOUT:    $zjq_ark_suffix_about,
        _RBGY_CRANE_TAR_GZ:        $zjq_crane_tar_gz,
        _RBGY_RUBRIC_REPO:         $zjq_rubric_repo,
        _RBGY_RUBRIC_COMMIT:       $zjq_rubric_commit,
        _RBGY_INSCRIBE_TIMESTAMP:  $zjq_inscribe_ts
      },
      options: {
        logging: "CLOUD_LOGGING_ONLY",
        machineType: $zjq_mtype
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose trigger-compatible build JSON"

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

  buc_log_args 'Validate vessel configuration'
  local z_validator_dir="${BASH_SOURCE[0]%/*}"
  source "${z_validator_dir}/rbrv_regime.sh" || buc_die "Failed to validate vessel configuration"

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

zrbf_verify_git_clean() {
  zrbf_sentinel

  buc_step "Verifying git repository state"

  buc_log_args 'Check for uncommitted changes'
  git diff-index --quiet HEAD -- || buc_die "Uncommitted changes detected - commit or stash first"

  buc_log_args 'Check for untracked files'
  git ls-files --others --exclude-standard > "${ZRBF_GIT_UNTRACKED_FILE}" || buc_die "Failed to check untracked files"
  local z_untracked=""
  z_untracked=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || buc_die "Failed to read untracked list"
  test -z "${z_untracked}" || buc_die "Untracked files present - commit or clean first"

  buc_log_args 'Check if all commits are pushed'
  git fetch --quiet || buc_die "git fetch failed"
  git rev-list @{u}..HEAD --count > "${ZRBF_GIT_UNTRACKED_FILE}" 2>/dev/null || echo "0" > "${ZRBF_GIT_UNTRACKED_FILE}"
  local z_unpushed=""
  z_unpushed=$(<"${ZRBF_GIT_UNTRACKED_FILE}") || buc_die "Failed to read ahead count"
  test "${z_unpushed}" -eq 0 || buc_die "Local commits not pushed (${z_unpushed} commits ahead)"

  buc_log_args 'Get git metadata'
  git rev-parse HEAD              > "${ZRBF_GIT_COMMIT_FILE}" || buc_die "Failed to get commit SHA"
  git rev-parse --abbrev-ref HEAD > "${ZRBF_GIT_BRANCH_FILE}" || buc_die "Failed to get branch name"

  buc_log_args 'Get first available remote'
  git remote | head -1 > "${ZRBF_GIT_REMOTE_FILE}" || buc_die "No git remotes configured"
  local z_remote=""
  z_remote=$(<"${ZRBF_GIT_REMOTE_FILE}") || buc_die "Failed to read remote name"
  test -n "${z_remote}" || buc_die "No git remotes found"

  buc_log_args 'Get repo URL from remote: ${z_remote}'
  git config --get "remote.${z_remote}.url" > "${ZRBF_GIT_REPO_FILE}" || buc_die "Failed to get repo URL"

  local z_commit=""
  local z_branch=""
  local z_repo_url=""
  z_commit=$(<"${ZRBF_GIT_COMMIT_FILE}") || buc_die "Failed to read commit"
  z_branch=$(<"${ZRBF_GIT_BRANCH_FILE}") || buc_die "Failed to read branch"
  z_repo_url=$(<"${ZRBF_GIT_REPO_FILE}") || buc_die "Failed to read repo url"

  test -n "${z_commit}"   || buc_die "Git commit is empty"
  test -n "${z_branch}"   || buc_die "Git branch is empty"
  test -n "${z_repo_url}" || buc_die "Git repo URL is empty"

  buc_log_args 'Extract owner/repo from URL (handles both HTTPS and SSH)'
  # Example HTTPS: https://github.com/owner/repo.git
  # Example SSH  : git@github.com:owner/repo.git
  local z_repo="${z_repo_url#*github.com[:/]}"
  z_repo="${z_repo%.git}"

  buc_log_args 'Write git info JSON'
  jq -n                        \
    --arg commit "${z_commit}" \
    --arg branch "${z_branch}" \
    --arg repo   "${z_repo}"   \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBF_GIT_INFO_FILE}" || buc_die "Failed to write git info"

  buc_info "Git state clean - commit: ${z_commit:0:8} on ${z_branch}"
}

zrbf_package_context() {
  zrbf_sentinel

  local z_dockerfile="$1"
  local z_context_dir="$2"

  buc_step 'Packaging build context (source only, no build config)'

  buc_log_args 'Create temp directory for context'
  rm -rf "${ZRBF_STAGING_DIR}" || buc_warn "Failed to clean existing staging directory"
  mkdir -p "${ZRBF_STAGING_DIR}" || buc_die "Failed to create staging directory"

  buc_log_args 'Copy context to staging'
  cp -r "${z_context_dir}/." "${ZRBF_STAGING_DIR}/" || buc_die "Failed to copy context"

  buc_log_args 'Copy Dockerfile to context root if not already there'
  local z_dockerfile_name="${z_dockerfile##*/}"
  cp "${z_dockerfile}" "${ZRBF_STAGING_DIR}/${z_dockerfile_name}" || buc_die "Failed to copy Dockerfile"

  buc_log_args "Create tarball"
  tar -czf "${ZRBF_BUILD_CONTEXT_TAR}" -C "${ZRBF_STAGING_DIR}" . || buc_die "Failed to create context archive"

  buc_log_args "Clean up staging"
  rm -rf "${ZRBF_STAGING_DIR}" || buc_warn "Failed to cleanup staging directory"

  buc_log_args 'Compute archive size (bytes) using temp file'
  wc -c < "${ZRBF_BUILD_CONTEXT_TAR}" > "${ZRBF_CONTEXT_SIZE_FILE}" || buc_die "Failed to compute context size"
  local z_size_bytes=""
  z_size_bytes=$(<"${ZRBF_CONTEXT_SIZE_FILE}") || buc_die "Failed to read context size"
  test -n "${z_size_bytes}" || buc_die "Context size is empty"

  # Size policy: warn >1MB; die >10MB
  if test "${z_size_bytes}" -gt 10485760; then
    buc_die  "Context tarball too large: ${z_size_bytes} bytes (>10MB limit)"
  elif test "${z_size_bytes}" -gt 1048576; then
    buc_warn "Context tarball large: ${z_size_bytes} bytes (>1MB warning)"
  fi

  buc_info "Build context packaged: ${z_size_bytes} bytes"
}

zrbf_package_mirror_context() {
  zrbf_sentinel
  buc_step 'Packaging mirror context'

  # NOTE: When rbf_mirror is implemented, this should use inline steps like rbf_build.
  # For now, this packages a minimal context for the mirror operation.
  rm -rf "${ZRBF_COPY_STAGING_DIR}" || buc_warn "Failed to clean existing mirror staging directory"
  mkdir -p "${ZRBF_COPY_STAGING_DIR}" || buc_die "Failed to create mirror staging directory"

  # Mirror operation needs no source files - steps will be inlined in API request
  tar -czf "${ZRBF_COPY_CONTEXT_TAR}" -C "${ZRBF_COPY_STAGING_DIR}" . \
    || buc_die "Failed to create mirror context archive"

  rm -rf "${ZRBF_COPY_STAGING_DIR}" || buc_warn "Failed to cleanup mirror staging directory"
}

zrbf_compose_tarball_name() {
  zrbf_sentinel
  test -s "${ZRBF_VESSEL_SIGIL_FILE}" || buc_die "Vessel sigil file missing"
  local z_sigil=""
  z_sigil=$(<"${ZRBF_VESSEL_SIGIL_FILE}") || buc_die "Failed to read vessel sigil"
  test -n "${z_sigil}" || buc_die "Empty vessel sigil"

  # Flat namespace (no subdirectories), BURD_NOW_STAMP for source artifact
  local z_name="${z_sigil}.${BURD_NOW_STAMP}.source.tar.gz"
  echo "${z_name}" > "${ZRBF_TARBALL_NAME_FILE}"      || buc_die "Failed to write tarball name"
  echo "${RBGD_GCS_BUCKET}/${z_name}" > "${ZRBF_GCS_OBJECT_FILE}" || buc_die "Failed to write bucket/object"
  buc_log_args "Tarball object: ${z_name}"
}

zrbf_upload_context_to_gcs() {
  zrbf_sentinel
  buc_step "Uploading build context tarball to GCS staging bucket"

  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  local z_obj_name=""
  z_obj_name=$(<"${ZRBF_TARBALL_NAME_FILE}") || buc_die "Missing tarball name"
  test -s "${ZRBF_BUILD_CONTEXT_TAR}" || buc_die "Context tar is empty"

  local z_url="${ZRBF_GCS_UPLOAD_BASE}/b/${RBGD_GCS_BUCKET}/o?uploadType=media&name=${z_obj_name}"

  curl -sS -X POST                                 \
    -H "Authorization: Bearer ${z_token}"          \
    -H "Content-Type: application/gzip"            \
    -H "Accept: application/json"                  \
    --data-binary @"${ZRBF_BUILD_CONTEXT_TAR}"     \
    -o "${ZRBF_GCS_UPLOAD_RESP}"                   \
    -w "%{http_code}"                              \
    "${z_url}" > "${ZRBF_GCS_UPLOAD_HTTP}" || buc_die "Upload request failed"

  local z_http=""
  z_http=$(<"${ZRBF_GCS_UPLOAD_HTTP}") || buc_die "No HTTP status from GCS upload"
  test "${z_http}" = "200" || buc_die "GCS upload failed (HTTP ${z_http})"

  buc_success "Uploaded: gs://${RBGD_GCS_BUCKET}/${z_obj_name}"
}

zrbf_compose_build_request_json() {
  zrbf_sentinel

  # Stitch produces complete trigger-compatible Build resource (steps + subs + options + timeout)
  zrbf_stitch_build_json

  jq empty "${ZRBF_STITCHED_BUILD_FILE}" || buc_die "Stitched build file is not valid JSON"

  local -r z_obj_name=$(<"${ZRBF_TARBALL_NAME_FILE}")
  test -n "${z_obj_name}" || buc_die "Missing tarball name"

  # Add builds.create-specific fields (source + serviceAccount) to stitch output
  # These are NOT part of the trigger-compatible cloudbuild.json — only needed for direct API submission
  local -r z_sa_resource="projects/${RBGD_GAR_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"
  local -r z_request_file="${BURD_TEMP_DIR}/rbf_build_request_tmp.json"

  jq \
    --arg bucket "${RBGD_GCS_BUCKET}" \
    --arg object "${z_obj_name}" \
    --arg sa     "${z_sa_resource}" \
    '. + {
      source: { storageSource: { bucket: $bucket, object: $object } },
      serviceAccount: $sa
    }' "${ZRBF_STITCHED_BUILD_FILE}" > "${z_request_file}" \
    || buc_die "Failed to compose build request json"

  mv "${z_request_file}" "${ZRBF_BUILD_REQUEST_FILE}" \
    || buc_die "Failed to write build request file"
}

zrbf_submit_build_json() {
  zrbf_sentinel
  buc_step 'Submitting build (JSON) to Google Cloud Build'

  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  curl -sS -X POST                              \
    -H "Authorization: Bearer ${z_token}"       \
    -H "Content-Type: application/json"         \
    -H "Accept: application/json"               \
    --data-binary @"${ZRBF_BUILD_REQUEST_FILE}" \
    -o "${ZRBF_BUILD_RESPONSE_FILE}"            \
    -w "%{http_code}"                           \
    "${ZRBF_GCB_PROJECT_BUILDS_URL}" > "${ZRBF_BUILD_HTTP_CODE}" || buc_die "Build create request failed"

  local z_http=""
  z_http=$(<"${ZRBF_BUILD_HTTP_CODE}") || buc_die "No HTTP status from Cloud Build create"
  test "${z_http}" = "200" -o "${z_http}" = "201" || buc_die "Cloud Build create failed (HTTP ${z_http})"
  test -s "${ZRBF_BUILD_RESPONSE_FILE}" || buc_die "Empty Cloud Build response"

  # Accept Operation or Build; prefer metadata.build.id else .id
  local z_build_id=""
  z_build_id=$(jq -r '(.metadata.build.id // .id // empty)' "${ZRBF_BUILD_RESPONSE_FILE}") || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build id not found in response"
  echo "${z_build_id}" > "${ZRBF_BUILD_ID_FILE}" || buc_die "Failed to persist build id"

  local z_console_url="${ZRBF_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"
}



zrbf_wait_build_completion() {
  zrbf_sentinel

  buc_step 'Waiting for build completion'

  local z_build_id=""
  z_build_id=$(<"${ZRBF_BUILD_ID_FILE}") || buc_die "No build ID found"
  test -n "${z_build_id}" || buc_die "Build ID file empty"

  buc_log_args 'Get fresh token for polling'
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  local z_status="PENDING"
  local z_attempts=0
  local z_max_attempts=960  # 80 minutes with 5 second intervals

  while true; do
    case "${z_status}" in PENDING|QUEUED|WORKING) : ;; *) break;; esac
    sleep 5

    z_attempts=$((z_attempts + 1))
    test ${z_attempts} -le ${z_max_attempts} || buc_die "Build timeout after ${z_max_attempts} attempts"

    buc_log_args "Fetch build status (attempt ${z_attempts}/${z_max_attempts})"
    curl -s                                                \
         -H "Authorization: Bearer ${z_token}"             \
         "${ZRBF_GCB_PROJECT_BUILDS_URL}/${z_build_id}"    \
         > "${ZRBF_BUILD_STATUS_FILE}"                     \
      || buc_die "Failed to get build status"

    test -f "${ZRBF_BUILD_STATUS_FILE}" || buc_die "Build status file not created"
    test -s "${ZRBF_BUILD_STATUS_FILE}" || buc_die "Build status file is empty"

    jq -r '.status' "${ZRBF_BUILD_STATUS_FILE}" > "${ZRBF_STATUS_CHECK_FILE}" || buc_die "Failed to extract status"
    z_status=$(<"${ZRBF_STATUS_CHECK_FILE}") || buc_die "Failed to read status"
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

  local z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Build container image from vessel using Google Cloud Build"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  test -n "${z_vessel_dir}" || buc_die "Vessel directory required"

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
    local z_native="$(uname -s | tr A-Z a-z)/$(uname -m)"
    case " ${RBRV_CONJURE_PLATFORMS} " in
      *"${z_native}"*) : ;;  # native arch allowed
      *) buc_die "Vessel '${RBRV_SIGIL}' forbids binfmt but RBRV_CONJURE_PLATFORMS='${RBRV_CONJURE_PLATFORMS}'" ;;
    esac
  fi

  buc_log_args "Generate build tag using vessel sigil"
  local z_tag="${RBRV_SIGIL}.${BURD_NOW_STAMP}"

  buc_info "Building vessel image: ${RBRV_SIGIL} -> ${z_tag}"

  # Verify GCB quota headroom before expensive packaging
  buc_log_args "Check GCB quota headroom"
  local z_quota_token=""
  z_quota_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get token for GCB quota check"
  rbgd_check_gcb_quota "${z_quota_token}"

  # Verify git state + capture metadata
  zrbf_verify_git_clean

  # Package build context (source code only; build config inlined in API request)
  zrbf_package_context "${RBRV_CONJURE_DOCKERFILE}" "${RBRV_CONJURE_BLDCONTEXT}"

  # Substitutions now composed inside zrbf_stitch_build_json() from module state
  # Stage & submit new flow
  zrbf_compose_tarball_name
  zrbf_upload_context_to_gcs
  zrbf_compose_build_request_json
  zrbf_submit_build_json

  # Wait for completion (5s x 960 = 80m, no backoff)
  zrbf_wait_build_completion

  buc_success "Vessel image built: ${z_tag}"
}

rbf_mirror() {
  zrbf_sentinel

  buc_die 'ELIDED FOR NOW - mirror public images to depot registry'
}

rbf_study() {
  zrbf_sentinel

  buc_doc_brief "Run minimal Cloud Build multipart study in-place"
  buc_doc_shown || return 0

  buc_step "Mint Cloud Build OAuth token (Director)"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get GCB OAuth token"

  zrbgc_sentinel

  buc_step "Execute study script from its own directory with parameters"
  local z_tools_dir="${BASH_SOURCE[0]%/*}"
  local z_repo_root="${z_tools_dir%/*}"
  local z_study_dir="${z_repo_root}/Study/study-gcb-build-submit-debug"
  local z_script="${z_study_dir}/sgbs-debug.sh"
  test -f "${z_script}" || buc_die "Study script not found: ${z_script}"

  ( cd "${z_study_dir}" && ./sgbs-debug.sh "${z_token}" ) || buc_die "Study script failed"

  buc_success "Study run completed"
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
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Confirm deletion unless --force
  if test "${z_skip_confirm}" = "false"; then
    buc_require "Will delete: ${z_locator}" "yes"
  fi

  buc_step "Deleting: ${z_locator}"

  # Delete by tag reference
  local z_status_file="${ZRBF_DELETE_PREFIX}status.txt"
  local z_response_file="${ZRBF_DELETE_PREFIX}response.json"

  curl -X DELETE -s                                   \
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

  # Note: Ideally uses Retriever role, but Director also has read access
  buc_step "Fetching OAuth token (Director)"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Listing all images in repository"

  local z_packages_file="${BURD_TEMP_DIR}/rbf_list_packages.json"
  local z_gar_api="https://artifactregistry.googleapis.com/v1"
  local z_repo_path="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  curl -sL \
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
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  buc_step "Enumerating arks from repository"

  local z_packages_file="${BURD_TEMP_DIR}/rbf_beseech_packages.json"
  local z_gar_api="https://artifactregistry.googleapis.com/v1"
  local z_repo_path="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  curl -sL \
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
  if test -n "${RBRR_RETRIEVER_RBRA_FILE:-}" && test -f "${RBRR_RETRIEVER_RBRA_FILE}"; then
    z_rbra_file="${RBRR_RETRIEVER_RBRA_FILE}"
    buc_info "Using Retriever credentials"
  else
    z_rbra_file="${RBRR_DIRECTOR_RBRA_FILE}"
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
  buc_doc_param "consecration" "Consecration timestamp (e.g., 20250206T120000Z)"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
  test -n "${z_vessel}" || buc_die "Vessel parameter required"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  buc_step "Authenticating for retrieval"

  # Prefer Retriever credentials, fallback to Director
  local z_rbra_file=""
  if test -n "${RBRR_RETRIEVER_RBRA_FILE:-}" && test -f "${RBRR_RETRIEVER_RBRA_FILE}"; then
    z_rbra_file="${RBRR_RETRIEVER_RBRA_FILE}"
    buc_info "Using Retriever credentials"
  else
    z_rbra_file="${RBRR_DIRECTOR_RBRA_FILE}"
    buc_info "Retriever not configured, using Director credentials"
  fi

  # Get OAuth token
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || buc_die "Failed to get OAuth token"

  # Construct ark tags
  local z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"

  buc_step "Verifying ark existence"

  # Check if -image artifact exists
  local z_image_status_file="${ZRBF_DELETE_PREFIX}summon_image_status.txt"
  local z_image_response_file="${ZRBF_DELETE_PREFIX}summon_image_response.json"

  curl --head -s                                     \
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

  # Source Director RBRA for rubric repo URL
  buc_step "Loading Director RBRA credentials"
  source "${RBRR_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"
  rbgu_check_rubric_repo_url "${RBRA_RUBRIC_REPO_URL:-}"

  # Sanitize rubric repo URL (strip embedded PAT for metadata and API use)
  local z_rubric_url_clean
  z_rubric_url_clean=$(printf '%s' "${RBRA_RUBRIC_REPO_URL}" | sed 's|://[^@]*@|://|')

  # Phase 0: Pin staleness gate
  buc_step "Checking GCB pin freshness"
  local -r z_now_epoch="${BURD_NOW_EPOCH}"
  local -r z_pins_epoch="${RBRR_GCB_PINS_REFRESHED_AT:-0}"
  local -r z_age=$((z_now_epoch - z_pins_epoch))
  if test "${z_age}" -gt "${ZRBF_INSCRIBE_STALENESS_SEC}"; then
    buc_die "GCB pins are stale (${z_age}s old, limit ${ZRBF_INSCRIBE_STALENESS_SEC}s) — run ./tt/rbw-rrg.RefreshGcbPins.sh first, commit, then re-run inscribe"
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
  z_git_repo="${z_git_repo_url#*github.com[:/]}"
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

  for z_vessel_dir in "${RBRR_VESSEL_DIR}"/*/; do
    test -f "${z_vessel_dir}rbrv.env" || continue
    # Check for conjure mode by scanning for RBRV_CONJURE_DOCKERFILE
    local z_grep_stderr="${ZRBF_INSCRIBE_PREFIX}grep_stderr.txt"
    grep -q "^RBRV_CONJURE_DOCKERFILE=" "${z_vessel_dir}rbrv.env" 2>"${z_grep_stderr}" || {
      buc_info "Skipping non-conjure vessel: ${z_vessel_dir##*/}"
      continue
    }
    z_vessel_dirs+=("${z_vessel_dir%/}")
  done
  test "${#z_vessel_dirs[@]}" -gt 0 || buc_die "No conjure vessels found in ${RBRR_VESSEL_DIR}"
  buc_info "Found ${#z_vessel_dirs[@]} conjure vessel(s)"

  buc_step "Generating rubric JSON for all conjure vessels"
  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    zrbf_load_vessel "${z_vessel_dir}"
    z_vessel_sigils+=("${RBRV_SIGIL}")

    buc_step "Stitching build JSON for ${RBRV_SIGIL}"
    zrbf_stitch_build_json

    local z_target="${z_vessel_dir}/cloudbuild.json"
    cp "${ZRBF_STITCHED_BUILD_FILE}" "${z_target}" \
      || buc_die "Failed to copy stitched JSON to ${z_target}"
    buc_info "Generated: ${z_target}"
  done

  # Phase 3: Verify all committed
  buc_step "Verifying all generated JSON matches committed copies"
  local z_stale_vessels=""
  local z_sigil=""
  local z_json_path=""

  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    z_sigil="${z_vessel_dir##*/}"
    z_json_path="${z_vessel_dir}/cloudbuild.json"

    # Check if file is tracked by git
    local z_ls_stderr="${ZRBF_INSCRIBE_PREFIX}ls_stderr.txt"
    if ! git ls-files --error-unmatch "${z_json_path}" >/dev/null 2>"${z_ls_stderr}"; then
      z_stale_vessels="${z_stale_vessels}  ${z_json_path} (NEW — not yet committed)\n"
      continue
    fi
    # Compare working copy against HEAD
    local z_diff_stderr="${ZRBF_INSCRIBE_PREFIX}diff_stderr.txt"
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

  git clone --depth 1 "${RBRA_RUBRIC_REPO_URL}" "${ZRBF_INSCRIBE_CLONE_DIR}" \
    || buc_die "Failed to clone rubric repo"
  buc_info "Rubric repo cloned to ${ZRBF_INSCRIBE_CLONE_DIR}"

  # Copy per-vessel directories to rubric clone
  buc_step "Syncing vessel build material to rubric repo clone"
  local z_rubric_vessel_dir=""
  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    z_sigil="${z_vessel_dir##*/}"
    z_rubric_vessel_dir="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}"

    rm -rf "${z_rubric_vessel_dir}"
    mkdir -p "${z_rubric_vessel_dir}" || buc_die "Failed to create ${z_rubric_vessel_dir}"

    # Copy cloudbuild.json
    cp "${z_vessel_dir}/cloudbuild.json" "${z_rubric_vessel_dir}/cloudbuild.json" \
      || buc_die "Failed to copy cloudbuild.json for ${z_sigil}"

    # Re-source vessel config to get Dockerfile and context paths
    source "${z_vessel_dir}/rbrv.env" || buc_die "Failed to re-source vessel config: ${z_vessel_dir}/rbrv.env"
    test -f "${RBRV_CONJURE_DOCKERFILE}" || buc_die "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
    cp "${RBRV_CONJURE_DOCKERFILE}" "${z_rubric_vessel_dir}/Dockerfile" \
      || buc_die "Failed to copy Dockerfile for ${z_sigil}"

    # Copy build context (everything in vessel dir except rbrv.env and cloudbuild.json)
    test -d "${RBRV_CONJURE_BLDCONTEXT}" || buc_die "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"
    local z_tar_stderr="${ZRBF_INSCRIBE_PREFIX}tar_stderr_${z_sigil}.txt"
    tar -cf - -C "${RBRV_CONJURE_BLDCONTEXT}" \
      --exclude='rbrv.env' --exclude='cloudbuild.json' . 2>"${z_tar_stderr}" \
      | tar -xf - -C "${z_rubric_vessel_dir}" 2>>"${z_tar_stderr}" \
      || buc_die "Failed to copy build context for ${z_sigil} — see ${z_tar_stderr}"

    buc_info "Synced: ${z_sigil}"
  done

  # Fill URL placeholder in rubric clone copies (commit placeholder stays for now)
  buc_step "Filling rubric repo URL in clone copies"
  for z_vessel_dir in "${z_vessel_dirs[@]}"; do
    z_sigil="${z_vessel_dir##*/}"
    local z_rubric_json="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}/cloudbuild.json"
    local z_filled_file="${ZRBF_INSCRIBE_PREFIX}${z_sigil}_url_filled.json"

    jq --arg repo "${z_rubric_url_clean}" \
      '.substitutions._RBGY_RUBRIC_REPO = $repo' \
      "${z_rubric_json}" > "${z_filled_file}" \
      || buc_die "Failed to fill URL placeholder for ${z_sigil}"
    mv "${z_filled_file}" "${z_rubric_json}" \
      || buc_die "Failed to write URL-filled JSON for ${z_sigil}"
  done

  # Commit (not pushed yet) to get a content hash
  buc_step "Committing rubric repo changes"
  local z_inscribe_ts
  z_inscribe_ts="i${BURD_NOW_STAMP:0:8}_${BURD_NOW_STAMP:9:6}"

  git -C "${ZRBF_INSCRIBE_CLONE_DIR}" add -A \
    || buc_die "Failed to stage rubric repo changes"

  if git -C "${ZRBF_INSCRIBE_CLONE_DIR}" diff --cached --quiet; then
    buc_info "Rubric repo already up to date — no changes to commit"
    # Still need to fill commit placeholder and ensure infra
    local z_rubric_commit
    z_rubric_commit=$(git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD) \
      || buc_die "Failed to get rubric HEAD"
  else
    git -C "${ZRBF_INSCRIBE_CLONE_DIR}" \
      commit -m "inscribe: ${z_inscribe_ts} — ${#z_vessel_dirs[@]} vessel(s)" \
      || buc_die "Failed to commit rubric repo"

    # Get content commit hash, then fill commit placeholder
    local z_content_commit
    z_content_commit=$(git -C "${ZRBF_INSCRIBE_CLONE_DIR}" rev-parse HEAD) \
      || buc_die "Failed to get content commit hash"

    buc_step "Filling rubric commit hash in clone copies"
    for z_vessel_dir in "${z_vessel_dirs[@]}"; do
      z_sigil="${z_vessel_dir##*/}"
      local z_rubric_json="${ZRBF_INSCRIBE_CLONE_DIR}/${z_sigil}/cloudbuild.json"
      local z_commit_filled="${ZRBF_INSCRIBE_PREFIX}${z_sigil}_commit_filled.json"

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

  # Phase 5: Ensure infrastructure (source link + triggers)
  buc_step "Authenticating for infrastructure operations"
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Developer Connect resource paths
  local -r z_gdc_parent="projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBRR_GDC_REGION}"
  local -r z_gdc_conn="${z_gdc_parent}/connections/${RBRR_GDC_CONNECTION_NAME}"

  # Derive source link ID from rubric repo URL
  # Extract org/repo from URL, replace / with -
  local z_link_id
  z_link_id=$(echo "${z_rubric_url_clean}" | sed 's|.*://[^/]*/||; s|\.git$||; s|/|-|g')
  test -n "${z_link_id}" || buc_die "Failed to derive source link ID from rubric repo URL"
  local -r z_link_resource="${z_gdc_conn}/gitRepositoryLinks/${z_link_id}"

  # Ensure source link exists
  buc_step "Ensuring Developer Connect source link: ${z_link_id}"
  local z_link_check_url="${RBGC_API_ROOT_DEVELOPERCONNECT}${RBGC_DEVELOPERCONNECT_V1}/${z_link_resource}"

  rbgu_http_json "GET" "${z_link_check_url}" "${z_token}" "inscribe_link_check"
  local z_link_code
  z_link_code=$(rbgu_http_code_capture "inscribe_link_check") || z_link_code=""

  if test "${z_link_code}" = "200"; then
    buc_info "Source link already exists: ${z_link_id}"
  elif test "${z_link_code}" = "404"; then
    buc_step "Creating source link: ${z_link_id}"
    local z_link_create_url="${RBGC_API_ROOT_DEVELOPERCONNECT}${RBGC_DEVELOPERCONNECT_V1}/${z_gdc_conn}/gitRepositoryLinks?gitRepositoryLinkId=${z_link_id}"
    local z_link_body="${ZRBF_INSCRIBE_PREFIX}link_body.json"
    jq -n --arg uri "${z_rubric_url_clean}" '{"cloneUri": $uri}' \
      > "${z_link_body}" || buc_die "Failed to compose source link body"

    rbgu_http_json_lro_ok \
      "Create source link" \
      "${z_token}" \
      "${z_link_create_url}" \
      "inscribe_link_create" \
      "${z_link_body}" \
      ".name" \
      "${RBGC_API_ROOT_DEVELOPERCONNECT}${RBGC_DEVELOPERCONNECT_V1}" \
      "" \
      "${RBGC_EVENTUAL_CONSISTENCY_SEC}" \
      "${RBGC_MAX_CONSISTENCY_SEC}"

    buc_info "Source link created: ${z_link_id}"
  else
    buc_die "Source link check failed (HTTP ${z_link_code})"
  fi

  # Ensure per-vessel triggers exist
  # List all triggers once and check for each vessel's trigger by name
  local z_triggers_url="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBRR_GDC_REGION}/triggers"

  buc_step "Listing existing Cloud Build triggers"
  rbgu_http_json "GET" "${z_triggers_url}" "${z_token}" "inscribe_trigger_list"
  rbgu_http_require_ok "List Cloud Build triggers" "inscribe_trigger_list"

  # Copy trigger list to our own temp file for repeated jq queries
  local z_trigger_list_file="${ZRBF_INSCRIBE_PREFIX}trigger_list.json"
  cp "${ZRBGU_PREFIX}inscribe_trigger_list${ZRBGU_POSTFIX_JSON}" "${z_trigger_list_file}" \
    || buc_die "Failed to copy trigger list response"

  for z_sigil in "${z_vessel_sigils[@]}"; do
    local z_trigger_name="${RBGC_RUBRIC_TRIGGER_PREFIX}${z_sigil}"

    buc_step "Ensuring trigger: ${z_trigger_name}"

    # Check if trigger exists by searching the list response for matching name
    local z_trigger_match_file="${ZRBF_INSCRIBE_PREFIX}trigger_match_${z_sigil}.txt"
    jq -r --arg tname "${z_trigger_name}" \
      '[.triggers // [] | .[] | select(.name == $tname) | .id] | length' \
      "${z_trigger_list_file}" > "${z_trigger_match_file}" \
      || buc_die "Failed to search trigger list for ${z_trigger_name}"
    local z_trigger_exists=""
    z_trigger_exists=$(<"${z_trigger_match_file}")

    if test "${z_trigger_exists:-0}" != "0"; then
      buc_info "Trigger already exists: ${z_trigger_name}"
      continue
    fi

    buc_step "Creating trigger: ${z_trigger_name}"
    local z_trigger_body="${ZRBF_INSCRIBE_PREFIX}trigger_${z_sigil}.json"

    local z_mason_sa="projects/${RBRR_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"
    jq -n \
      --arg name    "${z_trigger_name}" \
      --arg sigil   "${z_sigil}" \
      --arg repo    "${z_link_resource}" \
      --arg sa      "${z_mason_sa}" \
      '{
        name: $name,
        description: ("Recipe Bottle rubric trigger for " + $sigil),
        sourceToBuild: {
          repository: $repo,
          ref: "refs/heads/main",
          repoType: "GITHUB"
        },
        gitFileSource: {
          path: ($sigil + "/cloudbuild.json"),
          repository: $repo,
          revision: "refs/heads/main",
          repoType: "GITHUB"
        },
        serviceAccount: $sa
      }' > "${z_trigger_body}" || buc_die "Failed to compose trigger body for ${z_sigil}"

    local z_trigger_create_infix="inscribe_trigger_create_${z_sigil}"
    rbgu_http_json "POST" "${z_triggers_url}" "${z_token}" \
      "${z_trigger_create_infix}" "${z_trigger_body}"
    rbgu_http_require_ok "Create trigger ${z_trigger_name}" "${z_trigger_create_infix}" "409" "already exists"

    buc_info "Trigger created: ${z_trigger_name}"
  done

  # Clean up rubric clone
  rm -rf "${ZRBF_INSCRIBE_CLONE_DIR}" || buc_warn "Failed to clean up rubric clone"

  buc_success "Inscribe complete: ${#z_vessel_dirs[@]} vessel(s), timestamp ${z_inscribe_ts}"
}

rbf_abjure() {
  zrbf_sentinel

  local z_vessel="${1:-}"
  z_vessel="${z_vessel##*/}"  # strip path prefix — accept directory path or bare moniker
  local z_consecration="${2:-}"
  local z_force="${3:-}"

  # Documentation block
  buc_doc_brief "Abjure an ark (delete both -image and -about artifacts as a coherent unit)"
  buc_doc_param "vessel" "Vessel name (e.g., rbev-busybox)"
  buc_doc_param "consecration" "Consecration timestamp (e.g., 20250206T120000Z)"
  buc_doc_param "--force" "Optional: skip confirmation prompt"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_vessel}" || buc_die "Vessel parameter required"
  test -n "${z_consecration}" || buc_die "Consecration parameter required"

  # Check for --force flag in remaining arguments
  local z_skip_confirm=false
  if test "${z_force}" = "--force"; then
    z_skip_confirm=true
  fi

  buc_step "Authenticating as Director"

  # Get OAuth token using Director credentials
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") || buc_die "Failed to get OAuth token"

  # Construct ark tags
  local z_image_tag="${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  local z_about_tag="${z_consecration}${RBGC_ARK_SUFFIX_ABOUT}"

  buc_step "Verifying ark existence"

  # Check if -image artifact exists
  local z_image_status_file="${ZRBF_DELETE_PREFIX}image_status.txt"
  local z_image_response_file="${ZRBF_DELETE_PREFIX}image_response.json"

  curl --head -s                                     \
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
  local z_about_status_file="${ZRBF_DELETE_PREFIX}about_status.txt"
  local z_about_response_file="${ZRBF_DELETE_PREFIX}about_response.json"

  curl --head -s                                     \
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

  # Confirm abjuration unless --force
  if test "${z_skip_confirm}" = "false"; then
    local z_confirm_msg="Will abjure ark ${z_vessel}/${z_consecration}:"
    if test "${z_image_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${z_vessel}:${z_image_tag}"
    fi
    if test "${z_about_exists}" = "true"; then
      z_confirm_msg="${z_confirm_msg}\n  - ${z_vessel}:${z_about_tag}"
    fi
    buc_require "${z_confirm_msg}" "yes"
  fi

  # Delete -image artifact if exists
  if test "${z_image_exists}" = "true"; then
    buc_step "Deleting -image artifact"

    local z_delete_image_status="${ZRBF_DELETE_PREFIX}delete_image_status.txt"
    local z_delete_image_response="${ZRBF_DELETE_PREFIX}delete_image_response.json"

    curl -X DELETE -s                                   \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_image_response}"                   \
      "${ZRBF_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_image_tag}" \
      > "${z_delete_image_status}" || buc_die "DELETE request failed for -image"

    local z_delete_image_code
    z_delete_image_code=$(<"${z_delete_image_status}")
    test -n "${z_delete_image_code}" || buc_die "HTTP status code is empty for -image delete"

    if test "${z_delete_image_code}" != "202" && test "${z_delete_image_code}" != "204"; then
      buc_warn "Response body: $(cat "${z_delete_image_response}" 2>/dev/null || echo 'empty')"
      buc_die "Failed to delete -image artifact (HTTP ${z_delete_image_code})"
    fi

    buc_info "Deleted: ${z_vessel}:${z_image_tag}"
  fi

  # Delete -about artifact if exists
  if test "${z_about_exists}" = "true"; then
    buc_step "Deleting -about artifact"

    local z_delete_about_status="${ZRBF_DELETE_PREFIX}delete_about_status.txt"
    local z_delete_about_response="${ZRBF_DELETE_PREFIX}delete_about_response.json"

    curl -X DELETE -s                                   \
      -H "Authorization: Bearer ${z_token}"             \
      -w "%{http_code}"                                 \
      -o "${z_delete_about_response}"                   \
      "${ZRBF_REGISTRY_API_BASE}/${z_vessel}/manifests/${z_about_tag}" \
      > "${z_delete_about_status}" || buc_die "DELETE request failed for -about"

    local z_delete_about_code
    z_delete_about_code=$(<"${z_delete_about_status}")
    test -n "${z_delete_about_code}" || buc_die "HTTP status code is empty for -about delete"

    if test "${z_delete_about_code}" != "202" && test "${z_delete_about_code}" != "204"; then
      buc_warn "Response body: $(cat "${z_delete_about_response}" 2>/dev/null || echo 'empty')"
      buc_die "Failed to delete -about artifact (HTTP ${z_delete_about_code})"
    fi

    buc_info "Deleted: ${z_vessel}:${z_about_tag}"
  fi

  # Display results
  echo ""
  buc_success "Ark abjured: ${z_vessel}/${z_consecration}"
  if test "${z_image_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_image_tag} deleted"
  fi
  if test "${z_about_exists}" = "true"; then
    echo "  - ${z_vessel}:${z_about_tag} deleted"
  fi
}

# eof

