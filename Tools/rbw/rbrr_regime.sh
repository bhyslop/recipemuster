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
# Recipe Bottle Regime Repo - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRR_SOURCED:-}" || buc_die "Module rbrr multiply sourced - check sourcing hierarchy"
ZRBRR_SOURCED=1

######################################################################
# Internal Functions (zrbrr_*)

zrbrr_kindle() {
  test -z "${ZRBRR_KINDLED:-}" || buc_die "Module rbrr already kindled"

  # Container Registry Configuration
  buv_env_xname       RBRR_REGISTRY_OWNER          2     64
  buv_env_xname       RBRR_REGISTRY_NAME           2     64
  buv_env_string      RBRR_HISTORY_DIR             1    255
  buv_env_string      RBRR_NAMEPLATE_PATH          1    255
  buv_env_string      RBRR_VESSEL_DIR              1    255
  buv_env_ipv4        RBRR_DNS_SERVER

  # Machine Configuration
  buv_env_xname       RBRR_IGNITE_MACHINE_NAME     1     64
  buv_env_xname       RBRR_DEPLOY_MACHINE_NAME     1     64
  buv_env_string      RBRR_CRANE_TAR_GZ            1    512
  buv_env_string      RBRR_MANIFEST_PLATFORMS      1    512
  buv_env_string      RBRR_CHOSEN_PODMAN_VERSION   1     16
  buv_env_fqin        RBRR_CHOSEN_VMIMAGE_ORIGIN   1    256
  buv_env_string      RBRR_CHOSEN_IDENTITY         1    128
  buv_env_gname       RBRR_DEPOT_PROJECT_ID          6     63
  buv_env_gname       RBRR_GCP_REGION              1     32

  # Google Artifact Registry Configuration
  buv_env_gname       RBRR_GAR_REPOSITORY          1     63

  # Google Cloud Build Configuration
  buv_env_string      RBRR_GCB_MACHINE_TYPE        3     64
  buv_env_string      RBRR_GCB_TIMEOUT             2     10

  # Service Account Configuration Files
  buv_env_string      RBRR_GOVERNOR_RBRA_FILE         1    512
  buv_env_string      RBRR_RETRIEVER_RBRA_FILE     1    512
  buv_env_string      RBRR_DIRECTOR_RBRA_FILE      1    512

  # GCB image pins (digest-pinned)
  buv_env_odref       RBRR_GCB_GCRANE_IMAGE_REF
  buv_env_odref       RBRR_GCB_ORAS_IMAGE_REF

  # Validate directories exist
  buv_dir_exists "${RBRR_HISTORY_DIR}"
  buv_dir_exists "${RBRR_NAMEPLATE_PATH}"

  # Validate manifest platforms format (space-separated identifiers)
  local z_platform=""
  for z_platform in ${RBRR_MANIFEST_PLATFORMS}; do
    if ! printf '%s' "${z_platform}" | grep -q '^[a-z0-9_]\+$'; then
      buc_warn "Invalid platform format in RBRR_MANIFEST_PLATFORMS: ${z_platform}"
      buc_step "Expected format: lowercase alphanumeric with underscores"
      buc_die "Invalid RBRR_MANIFEST_PLATFORMS"
    fi
  done

  # Validate timeout format (number followed by 's' for seconds)
  if ! printf '%s' "${RBRR_GCB_TIMEOUT}" | grep -q '^[0-9]\+s$'; then
    buc_warn "Invalid RBRR_GCB_TIMEOUT format: ${RBRR_GCB_TIMEOUT}"
    buc_step "Expected format: number followed by 's' (e.g., 1200s)"
    buc_die "Invalid RBRR_GCB_TIMEOUT"
  fi

  # Validate Podman version format (e.g., 5.5 or 5.5.1)
  if ! printf '%s' "${RBRR_CHOSEN_PODMAN_VERSION}" | grep -q '^[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?$'; then
    buc_warn "Invalid RBRR_CHOSEN_PODMAN_VERSION format: ${RBRR_CHOSEN_PODMAN_VERSION}"
    buc_step "Expected format: semantic version like 5.5 or 5.5.1"
    buc_die "Invalid RBRR_CHOSEN_PODMAN_VERSION"
  fi

  # Build docker env args array for container injection
  # Usage: docker run "${ZRBRR_DOCKER_ENV[@]}" ...
  # Currently only RBRR_DNS_SERVER is needed by sentry; add others as needed
  ZRBRR_DOCKER_ENV=()
  ZRBRR_DOCKER_ENV+=("-e" "RBRR_DNS_SERVER=${RBRR_DNS_SERVER}")

  ZRBRR_KINDLED=1
}

zrbrr_sentinel() {
  test "${ZRBRR_KINDLED:-}" = "1" || buc_die "Module rbrr not kindled - call zrbrr_kindle first"
}

######################################################################
# Public Functions (rbrr_*)

# Load RBRR regime
# Usage: rbrr_load
# Constructs path to rbrr_RecipeBottleRegimeRepo.sh, verifies exists, sources, kindles
rbrr_load() {
  local z_rbrr_file="${BURC_PROJECT_ROOT}/rbrr_RecipeBottleRegimeRepo.sh"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"

  source "${z_rbrr_file}" || buc_die "Failed to source RBRR config: ${z_rbrr_file}"
  zrbrr_kindle
}

# eof
