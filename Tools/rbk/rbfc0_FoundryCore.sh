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
# Recipe Bottle Foundry Core - kindle entry: the single rbfc inclusion-guard and
# kindle/sentinel, sourcing the build-host primitives (rbfcb_) and the guard-free
# body clusters (rbfcv_ vessel-resolution, rbfca_ step-assembly, rbfcg_ GAR-REST,
# rbfcp_ plumb). The readonly ZRBFC_* constants the kindle sets are read globally
# by the clusters; the mutable z_rbfc_tool_* refs live in the kindle, populated
# downstream by zrbfc_resolve_tool_images.

set -euo pipefail

# Multiple inclusion guard (silent skip - rbfc is sourced by multiple child modules)
test -z "${ZRBFC_SOURCED:-}" || return 0
ZRBFC_SOURCED=1

# Build-host primitives and the guard-free body clusters; sourced here so every
# rbfc consumer reaches them unchanged.
source "${BASH_SOURCE[0]%/*}/rbfcb_BuildHost.sh"
source "${BASH_SOURCE[0]%/*}/rbfcv_VesselResolution.sh"
source "${BASH_SOURCE[0]%/*}/rbfca_StepAssembly.sh"
source "${BASH_SOURCE[0]%/*}/rbfcg_GarRest.sh"
source "${BASH_SOURCE[0]%/*}/rbfcp_Plumb.sh"

######################################################################
# Internal Functions (zrbfc_*)

zrbfc_kindle() {
  test -z "${ZRBFC_KINDLED:-}" || buc_die "Module rbfc already kindled"

  # Validate environment
  zburd_sentinel

  buc_log_args 'Check required GCB/GAR environment variables'
  zrbgc_sentinel

  buc_log_args 'Module Variables (ZRBFC_*)'
  readonly ZRBFC_GCB_API_BASE="https://cloudbuild.googleapis.com/v1"
  readonly ZRBFC_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  readonly ZRBFC_CLOUD_QUERY_BASE="https://console.cloud.google.com/cloud-build/builds"

  readonly ZRBFC_GCB_PROJECT_BUILDS_URL="${ZRBFC_GCB_API_BASE}/projects/${RBGD_GCB_PROJECT_ID}/locations/${RBGD_GCB_REGION}/builds"
  readonly ZRBFC_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBDC_GAR_REPOSITORY}"

  buc_log_args 'Trigger dispatch endpoints'
  readonly ZRBFC_TRIGGERS_URL="${RBGC_API_ROOT_CLOUDBUILD}${RBGC_CLOUDBUILD_V1}/projects/${RBDC_DEPOT_PROJECT_ID}/locations/${RBGD_GCB_REGION}/triggers"

  buc_log_args 'Registry API endpoints'
  readonly ZRBFC_REGISTRY_HOST="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  readonly ZRBFC_REGISTRY_PATH="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  readonly ZRBFC_REGISTRY_API_BASE="https://${ZRBFC_REGISTRY_HOST}/v2/${ZRBFC_REGISTRY_PATH}"

  buc_log_args 'Media types for registry operations'
  readonly ZRBFC_ACCEPT_MANIFEST_MTYPES="application/vnd.docker.distribution.manifest.v2+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.index.v1+json,application/vnd.oci.image.manifest.v1+json"

  buc_log_args 'Define build polling files'
  readonly ZRBFC_BUILD_ID_FILE="${BURD_TEMP_DIR}/rbfc_build_id.txt"
  readonly ZRBFC_BUILD_STATUS_FILE="${BURD_TEMP_DIR}/rbfc_build_status.json"
  readonly ZRBFC_BUILD_START_FILE="${BURD_TEMP_DIR}/rbfc_build_start.txt"
  readonly ZRBFC_BUILD_FINISH_FILE="${BURD_TEMP_DIR}/rbfc_build_finish.txt"

  buc_log_args 'Per-poll-attempt temp file prefixes (suffixed by poll counter for forensics)'
  readonly ZRBFC_POLL_RESPONSE_PREFIX="${BURD_TEMP_DIR}/rbfc_poll_response_"
  readonly ZRBFC_POLL_CODE_PREFIX="${BURD_TEMP_DIR}/rbfc_poll_code_"
  readonly ZRBFC_POLL_STDERR_PREFIX="${BURD_TEMP_DIR}/rbfc_poll_stderr_"
  readonly ZRBFC_POLL_ERR_CHECK_PREFIX="${BURD_TEMP_DIR}/rbfc_poll_err_check_"
  readonly ZRBFC_POLL_STATUS_PREFIX="${BURD_TEMP_DIR}/rbfc_poll_status_"

  buc_log_args 'Define git info file (used by stitch)'
  readonly ZRBFC_GIT_INFO_FILE="${BURD_TEMP_DIR}/rbfc_git_info.json"

  buc_log_args 'Define git metadata files (shared across about/mirror submissions)'
  readonly ZRBFC_GIT_PREFIX="${BURD_TEMP_DIR}/rbfc_git_"
  readonly ZRBFC_GIT_COMMIT_FILE="${ZRBFC_GIT_PREFIX}commit.txt"
  readonly ZRBFC_GIT_BRANCH_FILE="${ZRBFC_GIT_PREFIX}branch.txt"
  readonly ZRBFC_GIT_REPO_FILE="${ZRBFC_GIT_PREFIX}repo.txt"

  buc_log_args 'Vessel-related files'
  readonly ZRBFC_VESSEL_SIGIL_FILE="${BURD_TEMP_DIR}/rbfc_vessel_sigil.txt"
  readonly ZRBFC_VESSEL_RESOLVED_DIR_FILE="${BURD_TEMP_DIR}/rbfc_vessel_resolved_dir.txt"

  buc_log_args 'Define output files (BURD_OUTPUT_DIR — persists after dispatch)'
  readonly ZRBFC_OUTPUT_VESSEL_DIR="${BURD_OUTPUT_DIR}/rbfc_vessel_dir.txt"

  buc_log_args 'Scratch file for sequential temp-file patterns'
  readonly ZRBFC_SCRATCH_FILE="${BURD_TEMP_DIR}/rbfc_scratch.txt"

  buc_log_args 'GAR package enumeration output (zrbfc_list_packages_capture)'
  readonly ZRBFC_PACKAGE_LIST_FILE="${BURD_TEMP_DIR}/rbfc_package_list.txt"

  buc_log_args 'Step script directories (used by shared about/vouch/preflight assembly)'
  local z_self_dir="${BASH_SOURCE[0]%/*}"
  readonly ZRBFC_RBGJA_STEPS_DIR="${z_self_dir}/rbgja"
  test -d "${ZRBFC_RBGJA_STEPS_DIR}" || buc_die "RBGJA steps directory not found: ${ZRBFC_RBGJA_STEPS_DIR}"
  readonly ZRBFC_RBGJV_STEPS_DIR="${z_self_dir}/rbgjv"
  test -d "${ZRBFC_RBGJV_STEPS_DIR}" || buc_die "RBGJV steps directory not found: ${ZRBFC_RBGJV_STEPS_DIR}"
  readonly ZRBFC_RBGJR_STEPS_DIR="${z_self_dir}/rbgjr"
  test -d "${ZRBFC_RBGJR_STEPS_DIR}" || buc_die "RBGJR steps directory not found: ${ZRBFC_RBGJR_STEPS_DIR}"

  buc_log_args 'Shared cloud-step snippet library (zrbfc_expand_includes #@rbgjs_include)'
  readonly ZRBFC_RBGJS_SNIPPETS_DIR="${z_self_dir}/rbgjs"
  test -d "${ZRBFC_RBGJS_SNIPPETS_DIR}" || buc_die "RBGJS snippets directory not found: ${ZRBFC_RBGJS_SNIPPETS_DIR}"

  buc_log_args 'Tool image refs — mutable kindle state, populated by zrbfc_resolve_tool_images'
  z_rbfc_tool_gcloud=""
  z_rbfc_tool_docker=""
  z_rbfc_tool_alpine=""
  z_rbfc_tool_syft=""
  z_rbfc_tool_binfmt=""
  z_rbfc_tool_gcrane=""

  # Foundry build LRO poll policy — host-side wait_build_completion governance.
  # Each *_CEILING is the per-build-kind poll budget (wall clock = ceiling ×
  # INTERVAL_SEC). RETRY_TOLERANCE absorbs transient curl/empty-response
  # glitches before declaring the poll fatal. Distinct from the pool-build
  # policy in rbgp (ZRBGP_POOL_BUILD_POLL_*) — foundry builds carry user
  # payload; pool builds are diagnostic.
  readonly ZRBFC_BUILD_POLL_INTERVAL_SEC=5
  readonly ZRBFC_BUILD_POLL_RETRY_TOLERANCE=3
  readonly ZRBFC_BUILD_POLL_CEILING_INSCRIBE=120
  readonly ZRBFC_BUILD_POLL_CEILING_ENSHRINE=50
  readonly ZRBFC_BUILD_POLL_CEILING_CONJURE=960
  readonly ZRBFC_BUILD_POLL_CEILING_MIRROR=100
  readonly ZRBFC_BUILD_POLL_CEILING_ABOUT_VOUCH=100
  readonly ZRBFC_BUILD_POLL_CEILING_ABOUT=50
  readonly ZRBFC_BUILD_POLL_CEILING_VOUCH=50
  # Cloud-dispatched tool-plane delete (banish/abjure). The host waits for one
  # build that loops the package list in-pool (one build per abjure, never per
  # package), each package a convergence loop (rbgjl06: fire deletes, poll the
  # package GET to 404); generous so a multi-package hallmark abjure clears
  # within the budget.
  readonly ZRBFC_BUILD_POLL_CEILING_DELETE=600

  # Floating bootstrap builder for the cloud-dispatched delete step. The delete
  # build (banish/abjure) consumes no reliquary, so — like underpin/wsl — it
  # rides a floating Google-hosted image rather than a pinned reliquary tool
  # (RBS0 rbsk_pinning_boundary; the bootstrap-pin is the accepted itch). This is
  # the floating form of the gcloud image the reliquary mirrors and the existing
  # python steps ride, so python3 + urllib + json are known-present. rbfc-level
  # (not ZRBLD_) because the delete body rbldd_ runs in both the rbld and rbfl
  # processes and both kindle rbfc. Auth canon: RBSCB.
  readonly ZRBFC_DELETE_BUILDER="gcr.io/cloud-builders/gcloud:latest"

  readonly ZRBFC_KINDLED=1
}

zrbfc_sentinel() {
  test "${ZRBFC_KINDLED:-}" = "1" || buc_die "Module rbfc not kindled - call zrbfc_kindle first"
}

# eof
