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
# Recipe Bottle Foundry Director Build - conjure body (guard-free cluster,
# sourced by rbfd0_director): rbfd_build — direct builds.create conjure
# submission — with the support that partitions to it alone: the quota
# preflight, the builds.create JSON stitcher, and the pouch (build-context)
# push. Reads the entry's ZRBFD_STITCH_PREFIX / ZRBFD_CONTEXT_PREFIX /
# ZRBFD_RBGJB_STEPS_DIR kindle constants and the RBFD_hallmark_echo_step_id
# tinder; the registry preflight it calls lives in rbfdp_preflight.sh.

set -euo pipefail

######################################################################
# Internal Functions (zrbfd_*)

# Check concurrent build quota against regime requirements
# Args: token mode
#   mode: "gate" (die if insufficient) or "advisory" (warn if insufficient)
zrbfd_quota_preflight() {
  zrbfd_sentinel

  local -r z_token="${1:-}"

  test -n "${z_token}" || buc_die_now "zrbfd_quota_preflight: token required"

  # Extract vCPU count from machine type (last segment after final hyphen)
  local -r z_vcpus="${RBRD_GCB_MACHINE_TYPE##*-}"
  case "${z_vcpus}" in
    ""|0|*[!0-9]*)
      buc_warn "Cannot parse vCPU count from RBRD_GCB_MACHINE_TYPE='${RBRD_GCB_MACHINE_TYPE}' -- skipping quota preflight"
      return 0
      ;;
  esac

  buc_log_args "Machine type ${RBRD_GCB_MACHINE_TYPE} = ${z_vcpus} vCPUs"

  # Query Service Usage consumer quota API for concurrent_private_pool_build_cpus
  local -r z_metric_encoded="cloudbuild.googleapis.com%2Fconcurrent_private_pool_build_cpus"
  local -r z_url="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1BETA1}/projects/${RBDC_DEPOT_PROJECT_ID}/services/cloudbuild.googleapis.com/consumerQuotaMetrics/${z_metric_encoded}"

  buc_step "Checking concurrent build quota"
  rbuh_json "GET" "${z_url}" "${z_token}" "quota_preflight"

  local z_code=""
  z_code=$(rbuh_code_capture "quota_preflight") || z_code=""
  if test "${z_code}" != "200"; then
    buc_warn "Could not query build quota (HTTP ${z_code}) -- skipping preflight check"
    return 0
  fi

  # Filter quota response to region-specific bucket via intermediate file
  rbuh_jq_file_to_file_ok "quota_preflight" "quota_region" \
    "[.consumerQuotaLimits[0].quotaBuckets[] | select(.dimensions.region == \"${RBRD_GCP_REGION}\")] | .[0] // {}" \
    || true

  # Extract effective limit from region bucket, then fallback to first bucket
  local z_limit=""
  z_limit=$(rbuh_json_field_capture "quota_region" '.effectiveLimit') || z_limit=""
  if test -z "${z_limit}"; then
    z_limit=$(rbuh_json_field_capture "quota_preflight" \
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

zrbfd_stitch_build_json() {
  zrbfd_sentinel

  local -r z_output_path="${1:?Output path required}"
  local -r z_hallmark="${2:?Hallmark required}"
  local -r z_context_tag="${3:?Context image tag required}"

  buc_log_args "Stitching builds.create JSON to ${z_output_path}"

  # Preconditions: vessel loaded and git state captured
  test -s "${ZRBFC_VESSEL_SIGIL_FILE}" || buc_die_now "Vessel not loaded — call zrbfc_load_vessel first"
  test -s "${ZRBFC_GIT_INFO_FILE}"     || buc_die_now "Git info not captured — ensure git metadata is captured before stitch"

  buc_log_args 'Read vessel state for substitutions'
  local -r z_sigil=$(<"${ZRBFC_VESSEL_SIGIL_FILE}")
  test -n "${z_sigil}" || buc_die_now "Empty vessel sigil"
  local -r z_dockerfile_name="${RBRV_CONJURE_DOCKERFILE##*/}"
  local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"

  # Resolve base images: ANCHOR (locator) → full GAR reference, or pass ORIGIN through.
  # The locator carries its own namespace path (e.g. rbi_ld/<touchmark>:rbi_bole);
  # paths within a GAR repo are prefix-free, so the locator is used as-is.
  # Locator captures (z_image_locator_n) feed _RBGR_BASE_LOCATOR_n substitutions
  # for the in-pool preflight step — anchored slots get HEAD-checked, pass-through
  # slots stay empty (preflight cannot reach upstream from the worker pool).
  local -r z_gar_repo_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local z_image_ref_1="" z_image_ref_2="" z_image_ref_3=""
  local z_image_locator_1="" z_image_locator_2="" z_image_locator_3=""
  local z_ri_n="" z_ri_origin_var="" z_ri_anchor_var="" z_ri_origin="" z_ri_anchor=""
  local z_ri_pkg_path=""
  local z_ri_tag=""
  for z_ri_n in 1 2 3; do
    z_ri_origin_var="RBRV_IMAGE_${z_ri_n}_ORIGIN"
    z_ri_anchor_var="RBRV_IMAGE_${z_ri_n}_ANCHOR"
    z_ri_origin="${!z_ri_origin_var:-}"
    z_ri_anchor="${!z_ri_anchor_var:-}"
    test -n "${z_ri_origin}" || continue
    local z_ri_ref=""
    local z_ri_locator=""
    if test -n "${z_ri_anchor}"; then
      case "${z_ri_anchor}" in
        *:*) : ;;
        *)   buc_die_now "Invalid ${z_ri_anchor_var} locator format (expected package-path:tag): ${z_ri_anchor}" ;;
      esac
      z_ri_pkg_path="${z_ri_anchor%:*}"
      z_ri_tag="${z_ri_anchor##*:}"
      test -n "${z_ri_pkg_path}" || buc_die_now "Package path is empty in ${z_ri_anchor_var}: ${z_ri_anchor}"
      test -n "${z_ri_tag}"      || buc_die_now "Tag is empty in ${z_ri_anchor_var}: ${z_ri_anchor}"
      z_ri_ref="${z_gar_repo_base}/${z_ri_pkg_path}:${z_ri_tag}"
      z_ri_locator="${z_ri_anchor}"
      buc_log_args "Image slot ${z_ri_n} (anchored): ${z_ri_ref}"
    else
      z_ri_ref="${z_ri_origin}"
      buc_log_args "Image slot ${z_ri_n} (pass-through): ${z_ri_ref}"
    fi
    case "${z_ri_n}" in
      1) z_image_ref_1="${z_ri_ref}"; z_image_locator_1="${z_ri_locator}" ;;
      2) z_image_ref_2="${z_ri_ref}"; z_image_locator_2="${z_ri_locator}" ;;
      3) z_image_ref_3="${z_ri_ref}"; z_image_locator_3="${z_ri_locator}" ;;
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
  local -r z_stitch_git_commit_file="${ZRBFD_STITCH_PREFIX}git_commit.txt"
  local -r z_stitch_git_branch_file="${ZRBFD_STITCH_PREFIX}git_branch.txt"
  local -r z_stitch_git_repo_file="${ZRBFD_STITCH_PREFIX}git_repo.txt"

  jq -r '.commit' "${ZRBFC_GIT_INFO_FILE}" > "${z_stitch_git_commit_file}" \
    || buc_die_now "Failed to extract git commit from info file"
  jq -r '.branch' "${ZRBFC_GIT_INFO_FILE}" > "${z_stitch_git_branch_file}" \
    || buc_die_now "Failed to extract git branch from info file"
  jq -r '.repo'   "${ZRBFC_GIT_INFO_FILE}" > "${z_stitch_git_repo_file}" \
    || buc_die_now "Failed to extract git repo from info file"

  local -r z_git_commit=$(<"${z_stitch_git_commit_file}")
  local -r z_git_branch=$(<"${z_stitch_git_branch_file}")
  local -r z_git_repo=$(<"${z_stitch_git_repo_file}")

  test -n "${z_git_commit}" || buc_die_now "Git commit is empty"
  test -n "${z_git_branch}" || buc_die_now "Git branch is empty"
  test -n "${z_git_repo}"   || buc_die_now "Git repo is empty"

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
  # Entrypoint 'bash' → #!/bin/bash, 'sh' → #!/bin/sh, 'busybox' → #!/busybox/sh
  # (the reliquary gcrane :debug builder carries only /busybox/sh) — GCB script field
  # Delimiter is | because image refs contain colons (sha256 digests)
  # Pipeline: resolve base digests → buildx --push → per-platform pullback → SLSA
  # provenance via images: field
  local z_step_defs=(
    "rbgjb01-derive-tag-base.sh|${z_rbfc_tool_gcloud}|bash|${RBFD_hallmark_echo_step_id}"
  )
  if test "${z_needs_binfmt}" = "true"; then
    z_step_defs+=("rbgjb02-qemu-binfmt.sh|${z_rbfc_tool_docker}|bash|qemu-binfmt")
  fi
  z_step_defs+=(
    "rbgjb03-resolve-base-digests.sh|${z_rbfc_tool_gcrane}|busybox|resolve-base-digests"
    "rbgjb04-buildx-push-image.sh|${z_rbfc_tool_docker}|bash|buildx-push-image"
    "rbgjb05-per-platform-pullback.sh|${z_rbfc_tool_docker}|bash|per-platform-pullback"
    "rbgjb06-push-per-platform.sh|${z_rbfc_tool_docker}|bash|push-per-platform"
    "rbgjb07-push-diags.sh|${z_rbfc_tool_docker}|bash|push-diags"
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
  local z_accumulator_file="${ZRBFD_STITCH_PREFIX}steps.json"

  buc_log_args "Initializing empty steps array"
  echo "[]" > "${z_accumulator_file}" || buc_die_now "Failed to initialize steps JSON"

  for z_def in "${z_step_defs[@]}"; do
    IFS='|' read -r z_script z_builder z_entrypoint z_id <<< "${z_def}"
    z_script_path="${ZRBFD_RBGJB_STEPS_DIR}/${z_script}"
    z_body_file="${ZRBFD_STITCH_PREFIX}${z_id}_body.txt"
    z_escaped_file="${ZRBFD_STITCH_PREFIX}${z_id}_escaped.txt"
    z_steps_file="${ZRBFD_STITCH_PREFIX}${z_id}_steps.json"

    test -f "${z_script_path}" || buc_die_now "Step script not found: ${z_script_path}"

    buc_log_args "Reading script body for ${z_id} (skip shebang, comments pass through)"
    zrbfc_write_script_body "${z_script_path}" "${z_body_file}" || buc_die_now "Failed to read step script: ${z_script_path}"
    z_body=$(<"${z_body_file}")
    test -n "${z_body}" || buc_die_now "Empty script body: ${z_script_path}"

    buc_log_args "Baking pinned image refs and build strategy into script text"
    z_body="${z_body//\$\{ZRBF_TOOL_BINFMT\}/${z_rbfc_tool_binfmt}}"
    z_body="${z_body//\$\{ZRBF_BUILD_STRATEGY\}/${z_build_strategy}}"

    case "${z_entrypoint}" in
      bash)    z_shebang="#!/bin/bash" ;;
      sh)      z_shebang="#!/bin/sh" ;;
      busybox) z_shebang="#!/busybox/sh" ;;
      *)       buc_die_now "Unknown entrypoint: ${z_entrypoint}" ;;
    esac
    printf '%s\n%s' "${z_shebang}" "${z_body}" > "${z_escaped_file}" \
      || buc_die_now "Failed to write script body for ${z_id}"

    buc_log_args "Appending step ${z_id} to JSON array"
    jq \
      --arg name "${z_builder}" \
      --arg id "${z_id}" \
      --arg dir "${z_sigil}" \
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, dir: $dir, script: $script}]' \
      "${z_accumulator_file}" > "${z_steps_file}" \
      || buc_die_now "Failed to append step ${z_id} to JSON"
    mv "${z_steps_file}" "${z_accumulator_file}" \
      || buc_die_now "Failed to update steps JSON for ${z_id}"
  done

  # === Combined conjure: embed about steps after image steps ===
  # About steps use _RBGA_* substitutions delivered via the substitutions block.
  # _RBGA_BUILD_ID is the lone exception — Cloud Build job ID, only available
  # at runtime as the built-in $BUILD_ID env var; rewritten below.

  buc_log_args "Assembling about steps for combined conjure"
  local -r z_about_steps_file="${ZRBFD_STITCH_PREFIX}about_steps.json"
  zrbfc_assemble_about_steps "${z_about_steps_file}" "${ZRBFD_STITCH_PREFIX}about_"

  # About steps run in vessel dir so .hallmark from rbgjb01 is accessible
  buc_log_args "Adding dir field to about steps for vessel directory ${z_sigil}"
  local -r z_about_with_dir="${ZRBFD_STITCH_PREFIX}about_with_dir.json"
  jq --arg dir "${z_sigil}" '[.[] | . + {dir: $dir}]' \
    "${z_about_steps_file}" > "${z_about_with_dir}" \
    || buc_die_now "Failed to add dir to about steps"

  # Build ID: $BUILD_ID → GCB built-in available as env var
  buc_log_args "Post-processing about steps: build ID from env"
  local -r z_about_processed="${ZRBFD_STITCH_PREFIX}about_processed.json"
  local z_about_content
  z_about_content=$(<"${z_about_with_dir}") \
    || buc_die_now "Failed to read about steps for post-processing"
  z_about_content="${z_about_content//\$\{_RBGA_BUILD_ID:-\}/\$BUILD_ID}"
  printf '%s' "${z_about_content}" > "${z_about_processed}" \
    || buc_die_now "Failed to post-process about steps for conjure"

  buc_log_args "Combining image steps and about steps"
  local -r z_combined_steps_file="${ZRBFD_STITCH_PREFIX}combined_steps.json"
  jq -s '.[0] + .[1]' "${z_accumulator_file}" "${z_about_processed}" \
    > "${z_combined_steps_file}" || buc_die_now "Failed to combine image and about steps"
  z_accumulator_file="${z_combined_steps_file}"

  # Fallback for -diags extraction failure; -diags is the primary path for conjure
  buc_log_args "Reading Dockerfile content for _RBGA_DOCKERFILE_CONTENT substitution"
  local z_stitch_dockerfile_content=""
  local -r z_stitch_df_max_bytes=4000
  if test -f "${RBRV_CONJURE_DOCKERFILE:-}"; then
    z_stitch_dockerfile_content=$(<"${RBRV_CONJURE_DOCKERFILE}")
    if test "${#z_stitch_dockerfile_content}" -gt "${z_stitch_df_max_bytes}"; then
      buc_warn "Dockerfile exceeds 4KB substitution limit (${#z_stitch_dockerfile_content} bytes) — recipe.txt via -diags only"
      z_stitch_dockerfile_content=""
    fi
  fi

  # Compose builds.create Build resource — all substitution values resolved
  # host-side; _RBGA_BUILD_ID alone rewrites to the GCB $BUILD_ID builtin.
  buc_log_args "Composing builds.create Build resource"
  local -r z_build_file="${ZRBFD_STITCH_PREFIX}build.json"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Context extraction step (first step — extracts build context from pouch in GAR)
  local -r z_extract_step_file="${ZRBFD_STITCH_PREFIX}extract_step.json"
  jq -n \
    --arg name "${z_rbfc_tool_docker}" \
    --arg ctx_tag "${z_context_tag}" \
    --arg sigil "${z_sigil}" \
    '{
      name: $name,
      id: "extract-context",
      entrypoint: "/bin/bash",
      args: ["-lc", ("set -euo pipefail\necho \"Extracting build context from GAR\"\nCONTAINER=$$(docker create " + $ctx_tag + " /nonexistent)\nmkdir -p /workspace/" + $sigil + "\ndocker cp $${CONTAINER}:/build-context/. /workspace/" + $sigil + "/\ndocker rm $${CONTAINER}\necho \"Context extracted:\"\nls -la /workspace/" + $sigil + "/")]
    }' > "${z_extract_step_file}" \
    || buc_die_now "Failed to compose context extraction step"

  # Step 0: in-pool reliquary preflight (defense-in-depth)
  local -r z_preflight_step_file="${ZRBFD_STITCH_PREFIX}preflight_step.json"
  zrbfc_assemble_preflight_step "${z_preflight_step_file}" "${ZRBFD_STITCH_PREFIX}"

  # Combine: [preflight] + [extract-context] + image steps + about steps
  local -r z_all_steps_file="${ZRBFD_STITCH_PREFIX}all_steps.json"
  jq -s '.[0] + .[1] + .[2]' \
    "${z_preflight_step_file}" \
    <(jq -s '.' "${z_extract_step_file}") \
    "${z_accumulator_file}" \
    > "${z_all_steps_file}" || buc_die_now "Failed to combine preflight, context extraction, and image+about steps"

  # images: field — one per-platform attest tag per platform for SLSA provenance via CB images: push
  # These are durable provenance-carrying tags on the single attest package
  # (rbi_hm/<H>/attest); deleted only by abjure.
  local z_images_file="${ZRBFD_STITCH_PREFIX}images.json"
  local z_attest_pkg="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/attest"
  local z_remaining_suffixes="${z_platform_suffixes_csv}"
  local z_img_suffix=""
  echo "[]" > "${z_images_file}" || buc_die_now "Failed to initialize images JSON"
  while test -n "${z_remaining_suffixes}"; do
    z_img_suffix="${z_remaining_suffixes%%,*}"
    jq --arg uri "${z_attest_pkg}:${z_hallmark}${z_img_suffix}" \
      '. + [$uri]' "${z_images_file}" > "${z_images_file}.tmp" \
      || buc_die_now "Failed to append image URI"
    mv "${z_images_file}.tmp" "${z_images_file}" \
      || buc_die_now "Failed to update images JSON"
    test "${z_remaining_suffixes}" != "${z_img_suffix}" || break
    z_remaining_suffixes="${z_remaining_suffixes#*,}"
  done

  local -r z_cb_build_id='$BUILD_ID'

  # Pool routing: conjure/bind use vessel's egress mode
  local z_conjure_pool=""
  case "${RBRV_EGRESS_MODE}" in
    rbnve_tether) z_conjure_pool="${RBDC_POOL_TETHER}" ;;
    rbnve_airgap) z_conjure_pool="${RBDC_POOL_AIRGAP}" ;;
    *) buc_die_now "Unknown RBRV_EGRESS_MODE: ${RBRV_EGRESS_MODE}" ;;
  esac

  jq -n \
    --slurpfile zjq_steps  "${z_all_steps_file}" \
    --slurpfile zjq_images "${z_images_file}" \
    --arg zjq_dockerfile        "${z_dockerfile_name}" \
    --arg zjq_vessel            "${z_sigil}" \
    --arg zjq_platforms         "${z_platforms}" \
    --arg zjq_platform_suffixes "${z_platform_suffixes_csv}" \
    --arg zjq_gar_location      "${RBGD_GAR_LOCATION}" \
    --arg zjq_gar_project       "${RBGD_GAR_PROJECT_ID}" \
    --arg zjq_gar_repository    "${RBDC_GAR_REPOSITORY}" \
    --arg zjq_git_commit        "${z_git_commit}" \
    --arg zjq_git_branch        "${z_git_branch}" \
    --arg zjq_git_repo          "${z_git_repo}" \
    --arg zjq_gar_host_suffix   "${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_hallmarks_root    "${RBGL_HALLMARKS_ROOT}" \
    --arg zjq_hallmark          "${z_hallmark}" \
    --arg zjq_inscribe_ts       "${z_hallmark%%-r*}" \
    --arg zjq_pool              "${z_conjure_pool}" \
    --arg zjq_timeout           "${RBRR_GCB_TIMEOUT}" \
    --arg zjq_mason_sa          "${z_mason_sa}" \
    --arg zjq_cb_build_id       "${z_cb_build_id}" \
    --arg zjq_rbga_gar_host     "${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}" \
    --arg zjq_rbga_gar_path     "${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}" \
    --arg zjq_rbga_dockerfile   "${z_stitch_dockerfile_content}" \
    --arg zjq_image_1           "${z_image_ref_1}" \
    --arg zjq_image_2           "${z_image_ref_2}" \
    --arg zjq_image_3           "${z_image_ref_3}" \
    --arg zjq_basename_image    "${RBGC_ARK_BASENAME_IMAGE}" \
    --arg zjq_basename_about    "${RBGC_ARK_BASENAME_ABOUT}" \
    --arg zjq_basename_attest   "${RBGC_ARK_BASENAME_ATTEST}" \
    --arg zjq_basename_diags    "${RBGC_ARK_BASENAME_DIAGS}" \
    --arg zjq_lodes_root        "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_sprue         "${RBGC_LODE_TAG_SPRUE}" \
    --arg zjq_reliquary         "${RBRV_RELIQUARY}" \
    --arg zjq_locator_1         "${z_image_locator_1}" \
    --arg zjq_locator_2         "${z_image_locator_2}" \
    --arg zjq_locator_3         "${z_image_locator_3}" \
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
        _RBGY_PLATFORMS:           $zjq_platforms,
        _RBGY_PLATFORM_SUFFIXES:   $zjq_platform_suffixes,
        _RBGY_GAR_LOCATION:        $zjq_gar_location,
        _RBGY_GAR_PROJECT:         $zjq_gar_project,
        _RBGY_GAR_REPOSITORY:      $zjq_gar_repository,
        _RBGY_GIT_COMMIT:          $zjq_git_commit,
        _RBGY_GIT_BRANCH:          $zjq_git_branch,
        _RBGY_GAR_HOST_SUFFIX:     $zjq_gar_host_suffix,
        _RBGY_HALLMARKS_ROOT:      $zjq_hallmarks_root,
        _RBGY_HALLMARK:            $zjq_hallmark,
        _RBGY_IMAGE_1:             $zjq_image_1,
        _RBGY_IMAGE_2:             $zjq_image_2,
        _RBGY_IMAGE_3:             $zjq_image_3,
        _RBGY_ARK_BASENAME_IMAGE:  $zjq_basename_image,
        _RBGY_ARK_BASENAME_ATTEST: $zjq_basename_attest,
        _RBGY_ARK_BASENAME_DIAGS:  $zjq_basename_diags,
        _RBGA_GAR_HOST:            $zjq_rbga_gar_host,
        _RBGA_GAR_PATH:            $zjq_rbga_gar_path,
        _RBGA_HALLMARKS_ROOT:      $zjq_hallmarks_root,
        _RBGA_HALLMARK:            $zjq_hallmark,
        _RBGA_VESSEL:              $zjq_vessel,
        _RBGA_VESSEL_MODE:         "rbnve_conjure",
        _RBGA_GIT_COMMIT:          $zjq_git_commit,
        _RBGA_GIT_BRANCH:          $zjq_git_branch,
        _RBGA_GIT_REPO:            $zjq_git_repo,
        _RBGA_INSCRIBE_TIMESTAMP:  $zjq_inscribe_ts,
        _RBGA_BIND_SOURCE:         "",
        _RBGA_GRAFT_SOURCE:        "",
        _RBGA_DOCKERFILE_CONTENT:  $zjq_rbga_dockerfile,
        _RBGA_ARK_BASENAME_IMAGE:  $zjq_basename_image,
        _RBGA_ARK_BASENAME_ABOUT:  $zjq_basename_about,
        _RBGA_ARK_BASENAME_DIAGS:  $zjq_basename_diags,
        _RBGR_GAR_HOST:            $zjq_rbga_gar_host,
        _RBGR_GAR_PATH:            $zjq_rbga_gar_path,
        _RBGR_LODES_ROOT:          $zjq_lodes_root,
        _RBGR_TAG_SPRUE:           $zjq_tag_sprue,
        _RBGR_RELIQUARY:           $zjq_reliquary,
        _RBGR_BASE_LOCATOR_1:  $zjq_locator_1,
        _RBGR_BASE_LOCATOR_2:  $zjq_locator_2,
        _RBGR_BASE_LOCATOR_3:  $zjq_locator_3
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
    || buc_die_now "Failed to compose build JSON"

  mv "${z_build_file}" "${z_output_path}" \
    || buc_die_now "Failed to write final build JSON to ${z_output_path}"

  buc_log_args "Stitched ${#z_step_defs[@]} + context + about steps to ${z_output_path}"
}


# Push vessel build context to GAR as a FROM SCRATCH OCI image (the pouch).
# The pouch carries the Dockerfile and supporting files that GCB
# needs in /workspace during the build.
#
# Args: token  sigil  build_context_path  hallmark
# Side-effect: writes context image tag to ${ZRBFD_CONTEXT_PREFIX}tag.txt
zrbfd_push_build_context() {
  zrbfd_sentinel

  local -r z_token="$1"
  local -r z_sigil="$2"
  local -r z_bldctx="$3"
  local -r z_hallmark="$4"

  test -d "${z_bldctx}" || buc_die_now "Build context directory not found: ${z_bldctx}"
  test -n "${z_hallmark}" || buc_die_now "Hallmark required for pouch tag"

  local -r z_gar_host="${ZRBFC_REGISTRY_HOST}"
  local -r z_context_tag_file="${ZRBFD_CONTEXT_PREFIX}tag.txt"
  local -r z_context_dockerfile="${ZRBFD_CONTEXT_PREFIX}Dockerfile"

  local -r z_context_tag="${z_gar_host}/${ZRBFC_REGISTRY_PATH}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/pouch:${z_hallmark}"

  # Build FROM SCRATCH image containing build context
  buc_step "Building context image for ${z_sigil}"
  printf 'FROM scratch\nCOPY . /build-context/\n' > "${z_context_dockerfile}" \
    || buc_die_now "Failed to write context Dockerfile"

  # The generated absolute -f is the proven docker failure under Cygwin; the
  # context positional is routed too because dockerfile-inside-context is not
  # guaranteed (buc_native_path_capture no-ops when already relative/native).
  local z_norm_dockerfile=""
  z_norm_dockerfile=$(buc_native_path_capture "${z_context_dockerfile}") \
    || buc_die_now "Cannot normalize context Dockerfile path for docker: ${z_context_dockerfile}"
  local z_norm_context=""
  z_norm_context=$(buc_native_path_capture "${z_bldctx}") \
    || buc_die_now "Cannot normalize build-context path for docker: ${z_bldctx}"

  docker build --platform "${RBGC_BUILD_RUNNER_PLATFORM}" -f "${z_norm_dockerfile}" -t "${z_context_tag}" "${z_norm_context}" \
    || buc_die_now "Failed to build context image"

  # Push to GAR
  buc_step "Pushing context image to GAR"
  rbgo_docker_login "${z_token}" "${z_gar_host}"

  docker push "${z_context_tag}" \
    || buc_die_now "Failed to push context image to GAR"

  echo "${z_context_tag}" > "${z_context_tag_file}" \
    || buc_die_now "Failed to persist context image tag"

  buc_info "Context image pushed: ${z_context_tag}"
}


######################################################################
# External Functions (rbfd_*)

rbfd_build() {
  zrbfd_sentinel

  local -r z_vessel_dir="${1:-}"

  # Documentation block
  buc_doc_brief "Build container image from vessel via direct builds.create submission"
  buc_doc_param "vessel_dir" "Path to vessel directory containing rbrv.env"
  buc_doc_shown || return 0

  buc_log_args "Validate parameters"
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

  # Dirty-tree guard + pure-chain-head posture: RBr_9c2.
  bug_require_clean_tree_creed "${RBCC_creed_clean_build}"

  # Load and validate vessel
  zrbfc_load_vessel "${z_vessel_dir}"

  buc_log_args "Verify vessel has conjuring configuration"
  test -n "${RBRV_CONJURE_DOCKERFILE:-}" || buc_die_now "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_DOCKERFILE)"
  test -n "${RBRV_CONJURE_BLDCONTEXT:-}" || buc_die_now "Vessel '${RBRV_SIGIL}' is not configured for conjuring (no RBRV_CONJURE_BLDCONTEXT)"

  buc_log_args "Resolve paths from vessel configuration"
  test -f "${RBRV_CONJURE_DOCKERFILE}" || buc_die_now "Dockerfile not found: ${RBRV_CONJURE_DOCKERFILE}"
  test -d "${RBRV_CONJURE_BLDCONTEXT}" || buc_die_now "Build context not found: ${RBRV_CONJURE_BLDCONTEXT}"

  buc_step "Validating Dockerfile hygiene"
  rbfh_dockerfile_check "${RBRV_CONJURE_DOCKERFILE}"

  buc_info "Building vessel image: ${RBRV_SIGIL}"

  # Resolve tool images from reliquary (required for step image references)
  zrbfc_resolve_tool_images

  # Authenticate as Director
  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get Director OAuth token"

  # Quota preflight -- warn if insufficient capacity
  zrbfd_quota_preflight "${z_token}"

  # Registry preflight -- verify reliquary and base images exist before expensive operations
  zrbfd_registry_preflight "${z_token}" "${z_vessel_dir}"

  # Capture git metadata (stitch needs ZRBFC_GIT_INFO_FILE)
  buc_step "Capturing git metadata"
  zrbfc_ensure_git_metadata
  local z_git_commit=""
  z_git_commit=$(<"${ZRBFC_GIT_COMMIT_FILE}")
  local z_git_branch=""
  z_git_branch=$(<"${ZRBFC_GIT_BRANCH_FILE}")
  local z_git_repo=""
  z_git_repo=$(<"${ZRBFC_GIT_REPO_FILE}")
  jq -n \
    --arg commit "${z_git_commit}" \
    --arg branch "${z_git_branch}" \
    --arg repo   "${z_git_repo}" \
    '{"commit": $commit, "branch": $branch, "repo": $repo}' \
    > "${ZRBFC_GIT_INFO_FILE}" || buc_die_now "Failed to write git info JSON for stitch"
  buc_info "Git: ${z_git_commit:0:8} on ${z_git_branch}"

  # Mint hallmark on host — same pattern as bind/graft: inscribe + realized
  buc_step "Minting hallmark on host"
  local -r z_inscribe_ts="${RBGC_HALLMARK_PREFIX_CONJURE}${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  local -r z_realized_ts_file="${BURD_TEMP_DIR}/rbfd_realized_ts.txt"
  date -u +%y%m%d%H%M%S > "${z_realized_ts_file}" \
    || buc_die_now "Failed to generate realized timestamp"
  local -r z_realized_ts=$(<"${z_realized_ts_file}")
  test -n "${z_realized_ts}" || buc_die_now "Empty realized timestamp"
  local -r z_hallmark="${z_inscribe_ts}-r${z_realized_ts}"
  buc_info "Host-minted hallmark: ${z_hallmark}"

  # Push build context (pouch) to GAR as FROM SCRATCH image
  zrbfd_push_build_context "${z_token}" "${RBRV_SIGIL}" "${RBRV_CONJURE_BLDCONTEXT}" "${z_hallmark}"
  local z_context_tag=""
  z_context_tag=$(<"${ZRBFD_CONTEXT_PREFIX}tag.txt")
  test -n "${z_context_tag}" || buc_die_now "Empty context image tag after push"

  # Stitch build JSON — generates complete builds.create resource directly
  buc_step "Stitching build JSON"
  local -r z_build_file="${ZRBFD_CONTEXT_PREFIX}build.json"
  zrbfd_stitch_build_json "${z_build_file}" "${z_hallmark}" "${z_context_tag}"

  rbndb_check "${z_token}"

  # Submit via builds.create (no source — context delivered via GAR image)
  buc_step "Submitting build via builds.create"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "build_direct_create" "${z_build_file}"
  rbuh_require_ok "Direct build submission" "build_direct_create"

  # Extract build ID from Operation response
  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "build_direct_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die_now "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die_now "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Build dispatched: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_CONJURE}" "Conjure"

  # Consistency assert: verify Cloud Build echoed back the same hallmark we
  # minted. buildStepOutputs is index-aligned with the steps array — a step
  # that writes no output still holds a slot — so the slot index must come
  # from .steps of the same response, never a baked-in position.
  buc_step "Verifying hallmark consistency"

  jq -r --arg id "${RBFD_hallmark_echo_step_id}" \
    '.steps | map(.id) | index($id) // empty' \
    "${ZRBFC_BUILD_STATUS_FILE}" > "${ZRBFC_SCRATCH_FILE}" \
    || buc_die_now "Failed to locate step ${RBFD_hallmark_echo_step_id} in build response"
  local -r z_step_index=$(<"${ZRBFC_SCRATCH_FILE}")
  test -n "${z_step_index}" \
    || buc_die_now "Step ${RBFD_hallmark_echo_step_id} not found in build response steps"

  jq -r ".results.buildStepOutputs[${z_step_index}] // empty" "${ZRBFC_BUILD_STATUS_FILE}" > "${ZRBFC_SCRATCH_FILE}" \
    || buc_die_now "Failed to extract buildStepOutputs[${z_step_index}] from build response"
  local -r z_step_output=$(<"${ZRBFC_SCRATCH_FILE}")
  test -n "${z_step_output}" \
    || buc_die_now "Build echoed no hallmark (buildStepOutputs[${z_step_index}] empty) — cannot corroborate host-minted hallmark"

  local -r z_step_b64_file="${BURD_TEMP_DIR}/rbfd_step_b64.txt"
  local -r z_step_decoded_file="${BURD_TEMP_DIR}/rbfd_step_decoded.txt"
  printf '%s\n' "${z_step_output}" > "${z_step_b64_file}" \
    || buc_die_now "Failed to write step output for decoding"
  rbgo_base64_decode_file_to_file "${z_step_b64_file}" "${z_step_decoded_file}" \
    || buc_die_now "Failed to base64-decode build step output"
  local -r z_found_hallmark=$(<"${z_step_decoded_file}")
  test "${z_found_hallmark}" = "${z_hallmark}" \
    || buc_die_now "Hallmark mismatch: host minted '${z_hallmark}' but build returned '${z_found_hallmark}'"
  buc_info "Hallmark consistency verified: ${z_hallmark}"

  # Persist to output directory for test harness consumption
  echo "${z_vessel_dir}" > "${ZRBFC_OUTPUT_VESSEL_DIR}" \
    || buc_die_now "Failed to write vessel dir to output"
  buf_write_fact_single "${RBF_FACT_HALLMARK}" "${z_hallmark}"

  # Write GAR root fact file (registry prefix for composing full refs)
  buf_write_fact_single "${RBF_FACT_GAR_ROOT}" "${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}"

  # Write ark stem fact file (hallmark subtree under HALLMARKS_ROOT)
  buf_write_fact_single "${RBF_FACT_ARK_STEM}" "${RBGL_HALLMARKS_ROOT}/${z_hallmark}"

  # Write per-platform yield fact files (attest tags — durable provenance-carrying artifacts)
  # Single attest package; per-platform tag = ${HALLMARK}-${arch}
  local z_plat=""
  local z_plat_suffix=""
  local z_yield_ref=""
  for z_plat in ${RBRV_CONJURE_PLATFORMS//,/ }; do
    z_plat_suffix="${z_plat#linux/}"
    z_plat_suffix="${z_plat_suffix//\//}"
    z_yield_ref="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ATTEST}:${z_hallmark}-${z_plat_suffix}"
    buf_write_fact_single "${RBF_FACT_ARK_YIELD}-${RBGC_ARK_BASENAME_ATTEST}-${z_plat_suffix}" "${z_yield_ref}"
    buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_YIELD}-${RBGC_ARK_BASENAME_ATTEST}-${z_plat_suffix}"
  done

  buc_info "Output: ${ZRBFC_OUTPUT_VESSEL_DIR}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_GAR_ROOT}"
  buc_info "Output: ${BURD_OUTPUT_DIR}/${RBF_FACT_ARK_STEM}"

  buc_success "Vessel image built: ${RBRV_SIGIL}"
}

# eof
