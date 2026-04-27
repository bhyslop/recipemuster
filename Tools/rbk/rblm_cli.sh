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
# RBLM CLI - Lifecycle Marshal operations (zero, proof)

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command: zero - Zero regime to blank template for release

rblm_zero() {
  buc_doc_brief "Zero regime to blank template for release qualification"
  buc_doc_shown || return 0

  local -r z_rbrr="${RBBC_rbrr_file}"
  test -f "${z_rbrr}" || buc_die "RBRR file not found: ${z_rbrr}"

  # Pre-checks: working tree clean and HEAD pushed.
  # Marshal-zero auto-commits its mutations; both invariants must hold to keep
  # the resulting commit purely the marshal-zero state.
  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_zero_status.txt"
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_zero_upstream.txt"
  local -r z_unpushed_temp="${BURD_TEMP_DIR}/rblm_zero_unpushed.txt"

  git status --porcelain > "${z_status_temp}" || buc_die "git status failed"
  test ! -s "${z_status_temp}" || buc_die "Working tree not clean — commit or discard before marshal-zero. See: ${z_status_temp}"

  if ! git rev-parse --abbrev-ref --symbolic-full-name '@{u}' > "${z_upstream_temp}" 2>&1; then
    buc_die "Current branch has no upstream — set tracking and push before marshal-zero. See: ${z_upstream_temp}"
  fi

  git rev-list '@{u}..HEAD' > "${z_unpushed_temp}" || buc_die "git rev-list failed"
  test ! -s "${z_unpushed_temp}" || buc_die "HEAD has unpushed commits — push before marshal-zero. See: ${z_unpushed_temp}"

  # Discover secrets dir and vessel dir for pre-confirmation inventory
  local z_secrets_dir=""
  local z_vessel_dir=""
  local z_secrets_line=""
  while IFS= read -r z_secrets_line || test -n "${z_secrets_line}"; do
    case "${z_secrets_line}" in
      RBRR_SECRETS_DIR=*) z_secrets_dir="${z_secrets_line#RBRR_SECRETS_DIR=}" ;;
      RBRR_VESSEL_DIR=*)  z_vessel_dir="${z_secrets_line#RBRR_VESSEL_DIR=}"  ;;
    esac
  done < "${z_rbrr}"

  buh_section "Marshal Zero"
  buh_line "  Target: ${z_rbrr}"
  buh_e
  buh_line "  RBRR fields blanked (zeroed to onboarding start):"
  buh_line "    RBRR_CLOUD_PREFIX, RBRR_RUNTIME_PREFIX,"
  buh_line "    RBRR_DEPOT_MONIKER"
  buh_e
  buh_line "  RBRR fields pre-filled to defaults:"
  buh_line "    RBRR_DNS_SERVER, RBRR_GCB_MACHINE_TYPE, RBRR_GCB_TIMEOUT,"
  buh_line "    RBRR_GCB_MIN_CONCURRENT_BUILDS, RBRR_GCP_REGION,"
  buh_line "    RBRR_VESSEL_DIR, RBRR_SECRETS_DIR"
  buh_e
  buh_line "  Depot credentials DELETED (tied to prior depot):"
  if test -n "${z_secrets_dir}"; then
    local z_preview=""
    local z_any_cred=0
    for z_preview in "${RBCC_role_governor}/${RBCC_rbra_file}" "${RBCC_role_director}/${RBCC_rbra_file}" "${RBCC_role_retriever}/${RBCC_rbra_file}" "${RBCC_role_assay}/${RBCC_rbra_file}"; do
      if test -f "${z_secrets_dir}/${z_preview}"; then
        buh_line "    ${z_secrets_dir}/${z_preview}"
        z_any_cred=1
      fi
    done
    test "${z_any_cred}" = "1" || buh_line "    (none present)"
  else
    buh_line "    (secrets dir not configured)"
  fi
  buh_e
  buh_line "  Vessel hallmarks BLANKED (stale after depot change):"
  local z_np_preview=""
  local z_any_np=0
  for z_np_preview in "${RBBC_dot_dir}"/*/rbrn.env; do
    test -f "${z_np_preview}" || continue
    buh_line "    ${z_np_preview}"
    z_any_np=1
  done
  test "${z_any_np}" = "1" || buh_line "    (no nameplates found)"
  buh_e
  buh_line "  Vessel regime fields BLANKED (depot-scoped, stale after depot change):"
  buh_line "    RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR in all rbrv.env"
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    local z_vr_preview=""
    local z_any_vr=0
    for z_vr_preview in "${z_vessel_dir}"/*/rbrv.env; do
      test -f "${z_vr_preview}" || continue
      buh_line "    ${z_vr_preview}"
      z_any_vr=1
    done
    test "${z_any_vr}" = "1" || buh_line "    (no vessel regimes found)"
  else
    buh_line "    (vessel dir not configured or missing)"
  fi
  buh_e
  buh_line "  Preserved (payor-scoped, survives depot change):"
  buh_line "    ${z_secrets_dir}/${RBCC_role_payor}/${RBCC_rbro_file}"
  buh_e
  buh_line "  On completion, marshal-zero auto-commits the in-tree mutations."
  buh_e
  buc_require "Proceed with marshal zero?" "zero"

  local -r z_tmp="${z_rbrr}.tmp"
  local z_line=""
  while IFS= read -r z_line; do
    case "${z_line}" in
      # Pre-selected defaults
      RBRR_DNS_SERVER=*)                    printf '%s\n' "RBRR_DNS_SERVER=8.8.8.8"                     ;;
      RBRR_GCB_MACHINE_TYPE=*)              printf '%s\n' "RBRR_GCB_MACHINE_TYPE=e2-standard-2"         ;;
      RBRR_GCB_TIMEOUT=*)                   printf '%s\n' "RBRR_GCB_TIMEOUT=2700s"                      ;;
      RBRR_GCB_MIN_CONCURRENT_BUILDS=*)     printf '%s\n' "RBRR_GCB_MIN_CONCURRENT_BUILDS=3"            ;;
      RBRR_GCP_REGION=*)                    printf '%s\n' "RBRR_GCP_REGION=us-central1"                 ;;
      RBRR_VESSEL_DIR=*)                    printf '%s\n' "RBRR_VESSEL_DIR=rbev-vessels"                ;;
      RBRR_SECRETS_DIR=*)                   printf '%s\n' "RBRR_SECRETS_DIR=../station-files/secrets"   ;;
      # Site-specific fields blanked
      RBRR_CLOUD_PREFIX=*)                  printf '%s\n' "RBRR_CLOUD_PREFIX="                          ;;
      RBRR_RUNTIME_PREFIX=*)                printf '%s\n' "RBRR_RUNTIME_PREFIX="                        ;;
      RBRR_DEPOT_MONIKER=*)                 printf '%s\n' "RBRR_DEPOT_MONIKER="                         ;;
      RBRR_GCB_WORKER_POOL=*)               continue                                                      ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    printf '%s\n' "${z_line}"                                   ;;
    esac
  done < "${z_rbrr}" > "${z_tmp}" && mv "${z_tmp}" "${z_rbrr}"

  # Remove depot-scoped RBRA files (governor, director, retriever).
  # z_secrets_dir already extracted above for pre-confirmation inventory.
  if test -n "${z_secrets_dir}"; then
    local z_rbra=""
    for z_rbra in "${RBCC_role_governor}/${RBCC_rbra_file}" "${RBCC_role_director}/${RBCC_rbra_file}" "${RBCC_role_retriever}/${RBCC_rbra_file}" "${RBCC_role_assay}/${RBCC_rbra_file}"; do
      if test -f "${z_secrets_dir}/${z_rbra}"; then
        rm "${z_secrets_dir}/${z_rbra}" || buc_die "Failed to remove: ${z_secrets_dir}/${z_rbra}"
        buh_line "  Removed stale depot credential: ${z_rbra}"
      fi
    done
  fi

  # Blank hallmark values in all vessel nameplates.
  # Hallmarks reference images built against the prior depot — they
  # become stale after reset.  Blanking them causes the onboarding guide
  # to require conjure & vouch before declaring setup complete.
  local z_np=""
  local z_np_tmp=""
  for z_np in "${RBBC_dot_dir}"/*/rbrn.env; do
    test -f "${z_np}" || continue
    z_np_tmp="${z_np}.tmp"
    while IFS= read -r z_line; do
      case "${z_line}" in
        RBRN_SENTRY_HALLMARK=*)  printf '%s\n' "RBRN_SENTRY_HALLMARK=" ;;
        RBRN_BOTTLE_HALLMARK=*)  printf '%s\n' "RBRN_BOTTLE_HALLMARK=" ;;
        *)                           printf '%s\n' "${z_line}"                  ;;
      esac
    done < "${z_np}" > "${z_np_tmp}" && mv "${z_np_tmp}" "${z_np}"
    buh_line "  Blanked hallmarks: ${z_np}"
  done

  # Blank depot-scoped fields in all vessel regime files.
  # RBRV_RELIQUARY references a reliquary inscribed to the prior depot's GAR.
  # RBRV_IMAGE_*_ANCHOR references enshrined base images in the prior depot's GAR.
  # Both become stale after depot change — onboarding requires inscribe + enshrine.
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    local z_vr=""
    local z_vr_tmp=""
    for z_vr in "${z_vessel_dir}"/*/rbrv.env; do
      test -f "${z_vr}" || continue
      z_vr_tmp="${z_vr}.tmp"
      while IFS= read -r z_line; do
        case "${z_line}" in
          RBRV_RELIQUARY=*)       printf '%s\n' "RBRV_RELIQUARY="       ;;
          RBRV_IMAGE_*_ANCHOR=*)  printf '%s\n' "${z_line%%=*}="        ;;
          *)                      printf '%s\n' "${z_line}"             ;;
        esac
      done < "${z_vr}" > "${z_vr_tmp}" && mv "${z_vr_tmp}" "${z_vr}"
      buh_line "  Blanked depot-scoped fields: ${z_vr}"
    done
  fi

  buh_line "  Zero complete: ${z_rbrr}"
  buh_e

  # Auto-commit the in-tree mutations so post-zero state is captured as a single
  # commit. RBRA deletions (outside repo) are not staged.
  buc_step "Committing marshal-zero state"
  git add "${z_rbrr}" || buc_die "Failed to stage RBRR file"

  local z_stage=""
  for z_stage in "${RBBC_dot_dir}"/*/rbrn.env; do
    test -f "${z_stage}" || continue
    git add "${z_stage}" || buc_die "Failed to stage: ${z_stage}"
  done
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    for z_stage in "${z_vessel_dir}"/*/rbrv.env; do
      test -f "${z_stage}" || continue
      git add "${z_stage}" || buc_die "Failed to stage: ${z_stage}"
    done
  fi

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already in marshal-zero state"
  else
    git commit -m "Marshal Zero — release qualification reset" || buc_die "Marshal-zero commit failed"
    buh_line "  Marshal-zero state committed"
  fi
  buh_e

  buh_line "  Next: verify onboarding guide detects blank state:"
  buc_tabtarget "${RBZ_ONBOARD_START_HERE}"
  buc_success "Regime zeroed to blank template"
}

######################################################################
# Command: proof - Create isolated clone for release ceremony testing

rblm_proof() {
  buc_doc_brief "Create proof copy of repository for release ceremony testing"
  buc_doc_param "target_dir" "Absolute path to target directory (must not exist)"
  buc_doc_shown || return 0

  local z_target_dir="${1:-}"
  test -n "${z_target_dir}" || buc_die "Target directory path is required"

  # Validate absolute path
  case "${z_target_dir}" in
    /*) ;;
    *)  buc_die "Target directory must be an absolute path: ${z_target_dir}" ;;
  esac

  # Must not exist
  test ! -e "${z_target_dir}" || buc_die "Target directory already exists: ${z_target_dir}"

  # Get origin URL via temp file (BCG: temp file instead of command substitution)
  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"
  local -r z_origin_temp="${BURD_TEMP_DIR}/rblm_origin_url.txt"
  git remote get-url origin > "${z_origin_temp}" || buc_die "Failed to get origin URL"
  local z_origin_url=$(<"${z_origin_temp}")
  test -n "${z_origin_url}" || buc_die "Origin URL is empty"

  # Derive repo name from origin URL (BCG: parameter expansion, not basename)
  local z_repo_name="${z_origin_url##*/}"
  z_repo_name="${z_repo_name%.git}"
  test -n "${z_repo_name}" || buc_die "Could not derive repo name from origin: ${z_origin_url}"

  # Resolve source station-files directory from BURD_STATION_FILE
  # BURD_STATION_FILE is absolute (set by launcher from BURC_STATION_FILE)
  local -r z_station_file_dir="${BURD_STATION_FILE%/*}"
  local -r z_source_station_dir="${z_station_file_dir}"
  local -r z_source_secrets="${z_station_file_dir}/secrets"

  # Compute target paths
  local -r z_clone_dir="${z_target_dir}/${z_repo_name}"
  local -r z_target_station_dir="${z_target_dir}/station-files"
  local -r z_target_secrets="${z_target_station_dir}/secrets"

  # Get OPEN_SOURCE_UPSTREAM URL if configured
  local z_upstream_url=""
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_upstream_url.txt"
  if git remote get-url OPEN_SOURCE_UPSTREAM > "${z_upstream_temp}" 2>/dev/null; then
    z_upstream_url=$(<"${z_upstream_temp}")
  fi

  # Present plan
  buh_section "Marshal Proof"
  buh_line "  Target directory:     ${z_target_dir}"
  buh_line "  Clone subdirectory:   ${z_clone_dir}"
  buh_line "  Station files:        ${z_target_station_dir}"
  buh_line "  Repo name:            ${z_repo_name}"
  buh_e
  buh_line "  Source station dir:   ${z_source_station_dir}"
  buh_line "  Source secrets:       ${z_source_secrets}"
  buh_e
  buh_line "  Clone origin:         ${z_origin_url}"
  if test -n "${z_upstream_url}"; then
    buh_line "  OPEN_SOURCE_UPSTREAM: ${z_upstream_url}"
  else
    buh_line "  OPEN_SOURCE_UPSTREAM: (not configured)"
  fi
  buh_e

  # Create target directory
  buc_step "Creating target directory"
  mkdir "${z_target_dir}" || buc_die "Failed to create target directory: ${z_target_dir}"

  # Clone repository
  buc_step "Cloning repository to ${z_clone_dir}"
  git clone . "${z_clone_dir}" || buc_die "Failed to clone repository"

  # Set origin in clone to real origin (not local path from git clone .)
  buc_step "Configuring remotes in clone"
  git -C "${z_clone_dir}" remote set-url origin "${z_origin_url}" || buc_die "Failed to set origin URL in clone"

  # Set OPEN_SOURCE_UPSTREAM if configured on source
  if test -n "${z_upstream_url}"; then
    if git -C "${z_clone_dir}" remote get-url OPEN_SOURCE_UPSTREAM >/dev/null 2>&1; then
      git -C "${z_clone_dir}" remote set-url OPEN_SOURCE_UPSTREAM "${z_upstream_url}" || buc_die "Failed to set OPEN_SOURCE_UPSTREAM in clone"
    else
      git -C "${z_clone_dir}" remote add OPEN_SOURCE_UPSTREAM "${z_upstream_url}" || buc_die "Failed to add OPEN_SOURCE_UPSTREAM to clone"
    fi
  fi

  # Copy station files (all .env files from source station directory)
  buc_step "Copying station files"
  mkdir -p "${z_target_secrets}" || buc_die "Failed to create secrets directory"

  local z_env_file=""
  local z_env_name=""
  for z_env_file in "${z_source_station_dir}"/*.env; do
    test -f "${z_env_file}" || continue
    z_env_name="${z_env_file##*/}"
    cp "${z_env_file}" "${z_target_station_dir}/${z_env_name}" || buc_die "Failed to copy: ${z_env_name}"
    buh_line "  Copied: ${z_env_name}"
  done

  # Copy secrets (credential files)
  if test -d "${z_source_secrets}"; then
    local z_cred=""
    local z_cred_name=""
    local z_any_copied=0
    for z_cred in "${z_source_secrets}"/*.env; do
      test -f "${z_cred}" || continue
      z_cred_name="${z_cred##*/}"
      cp "${z_cred}" "${z_target_secrets}/${z_cred_name}" || buc_die "Failed to copy credential: ${z_cred_name}"
      chmod 600 "${z_target_secrets}/${z_cred_name}" || buc_die "Failed to set permissions on: ${z_cred_name}"
      buh_line "  Copied: secrets/${z_cred_name}"
      z_any_copied=1
    done
    test "${z_any_copied}" = "1" || buh_line "  No credential files found in: ${z_source_secrets}"
  else
    buh_line "  Warning: source secrets directory not found: ${z_source_secrets}"
  fi

  buh_e
  buh_line "  Proof complete: ${z_clone_dir}"
  buh_e
  buh_line "  To use the duplicate, start Claude Code from:"
  buh_line "    ${z_clone_dir}"
  buc_success "Proof copy created at ${z_target_dir}"
}

######################################################################
# Command: generate - Generate tabtarget context markdown from zipper registry

rblm_generate() {
  buc_doc_brief "Generate tabtarget context markdown from zipper registry"
  buc_doc_shown || return 0

  local -r z_target="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}/rbk-claude-tabtarget-context.md"
  local -r z_tmp="${BURD_TEMP_DIR}/rbk-claude-tabtarget-context.md"

  buz_emit_context "${BURD_TABTARGET_DIR}" > "${z_tmp}" || buc_die "Failed to generate context"

  if test -f "${z_target}" && diff -q "${z_tmp}" "${z_target}" >/dev/null 2>&1; then
    buc_log_args "Context file already up to date: ${z_target}"
  else
    cp "${z_tmp}" "${z_target}" || buc_die "Failed to write: ${z_target}"
    buc_log_args "Generated: ${z_target}"
  fi

  buc_success "Context generation complete"
}

######################################################################
# Furnish and Main

zrblm_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for this invocation (dispatch-provided)"
  buc_doc_env "BURD_TABTARGET_DIR    " "Tabtarget directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh" || buc_die "Failed to source rbcc_Constants.sh"

  source "${BURD_BUK_DIR}/buym_yelp.sh"         || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"      || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"     || buc_die "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"    || buc_die "Failed to source rbz_zipper.sh"
  zbuz_kindle
  zrbz_kindle
}

buc_execute rblm_ "Lifecycle Marshal" zrblm_furnish "$@"

# eof
