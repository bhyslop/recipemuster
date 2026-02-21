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

  # Set defaults for all fields (enrollment enforces required-ness)
  RBRR_VESSEL_DIR="${RBRR_VESSEL_DIR:-}"
  RBRR_DNS_SERVER="${RBRR_DNS_SERVER:-}"
  RBRR_IGNITE_MACHINE_NAME="${RBRR_IGNITE_MACHINE_NAME:-}"
  RBRR_DEPLOY_MACHINE_NAME="${RBRR_DEPLOY_MACHINE_NAME:-}"
  RBRR_CRANE_TAR_GZ="${RBRR_CRANE_TAR_GZ:-}"
  RBRR_MANIFEST_PLATFORMS="${RBRR_MANIFEST_PLATFORMS:-}"
  RBRR_CHOSEN_PODMAN_VERSION="${RBRR_CHOSEN_PODMAN_VERSION:-}"
  RBRR_CHOSEN_VMIMAGE_ORIGIN="${RBRR_CHOSEN_VMIMAGE_ORIGIN:-}"
  RBRR_CHOSEN_IDENTITY="${RBRR_CHOSEN_IDENTITY:-}"
  RBRR_DEPOT_PROJECT_ID="${RBRR_DEPOT_PROJECT_ID:-}"
  RBRR_GCP_REGION="${RBRR_GCP_REGION:-}"
  RBRR_GAR_REPOSITORY="${RBRR_GAR_REPOSITORY:-}"
  RBRR_GCB_MACHINE_TYPE="${RBRR_GCB_MACHINE_TYPE:-}"
  RBRR_GCB_TIMEOUT="${RBRR_GCB_TIMEOUT:-}"
  RBRR_GCB_MIN_CONCURRENT_BUILDS="${RBRR_GCB_MIN_CONCURRENT_BUILDS:-}"
  RBRR_GOVERNOR_RBRA_FILE="${RBRR_GOVERNOR_RBRA_FILE:-}"
  RBRR_RETRIEVER_RBRA_FILE="${RBRR_RETRIEVER_RBRA_FILE:-}"
  RBRR_DIRECTOR_RBRA_FILE="${RBRR_DIRECTOR_RBRA_FILE:-}"
  RBRR_GCB_ORAS_IMAGE_REF="${RBRR_GCB_ORAS_IMAGE_REF:-}"
  RBRR_GCB_GCLOUD_IMAGE_REF="${RBRR_GCB_GCLOUD_IMAGE_REF:-}"
  RBRR_GCB_DOCKER_IMAGE_REF="${RBRR_GCB_DOCKER_IMAGE_REF:-}"
  RBRR_GCB_ALPINE_IMAGE_REF="${RBRR_GCB_ALPINE_IMAGE_REF:-}"
  RBRR_GCB_SYFT_IMAGE_REF="${RBRR_GCB_SYFT_IMAGE_REF:-}"
  RBRR_GCB_BINFMT_IMAGE_REF="${RBRR_GCB_BINFMT_IMAGE_REF:-}"

  # Detect unexpected RBRR_ variables
  local z_known="RBRR_VESSEL_DIR RBRR_DNS_SERVER RBRR_IGNITE_MACHINE_NAME RBRR_DEPLOY_MACHINE_NAME RBRR_CRANE_TAR_GZ RBRR_MANIFEST_PLATFORMS RBRR_CHOSEN_PODMAN_VERSION RBRR_CHOSEN_VMIMAGE_ORIGIN RBRR_CHOSEN_IDENTITY RBRR_DEPOT_PROJECT_ID RBRR_GCP_REGION RBRR_GAR_REPOSITORY RBRR_GCB_MACHINE_TYPE RBRR_GCB_TIMEOUT RBRR_GCB_MIN_CONCURRENT_BUILDS RBRR_GOVERNOR_RBRA_FILE RBRR_RETRIEVER_RBRA_FILE RBRR_DIRECTOR_RBRA_FILE RBRR_GCB_ORAS_IMAGE_REF RBRR_GCB_GCLOUD_IMAGE_REF RBRR_GCB_DOCKER_IMAGE_REF RBRR_GCB_ALPINE_IMAGE_REF RBRR_GCB_SYFT_IMAGE_REF RBRR_GCB_BINFMT_IMAGE_REF"
  ZRBRR_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v RBRR_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZRBRR_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  # Die on unexpected variables
  if test ${#ZRBRR_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected RBRR_ variables: ${ZRBRR_UNEXPECTED[*]}"
  fi

  # Enroll all RBRR variables for validation via buv_vet/buv_report

  # Container Registry Configuration
  buv_string_enroll  RBRR  RBRR_VESSEL_DIR              "" ""  1  255
  buv_ipv4_enroll    RBRR  RBRR_DNS_SERVER              "" ""

  # Machine Configuration
  buv_xname_enroll   RBRR  RBRR_IGNITE_MACHINE_NAME     "" ""  1   64
  buv_xname_enroll   RBRR  RBRR_DEPLOY_MACHINE_NAME     "" ""  1   64
  buv_string_enroll  RBRR  RBRR_CRANE_TAR_GZ            "" ""  1  512
  buv_string_enroll  RBRR  RBRR_MANIFEST_PLATFORMS       "" ""  1  512
  buv_string_enroll  RBRR  RBRR_CHOSEN_PODMAN_VERSION    "" ""  1   16
  buv_fqin_enroll    RBRR  RBRR_CHOSEN_VMIMAGE_ORIGIN    "" ""  1  256
  buv_string_enroll  RBRR  RBRR_CHOSEN_IDENTITY          "" ""  1  128
  buv_gname_enroll   RBRR  RBRR_DEPOT_PROJECT_ID         "" ""  6   63
  buv_gname_enroll   RBRR  RBRR_GCP_REGION               "" ""  1   32

  # Google Artifact Registry Configuration
  buv_gname_enroll   RBRR  RBRR_GAR_REPOSITORY           "" ""  1   63

  # Google Cloud Build Configuration
  buv_string_enroll  RBRR  RBRR_GCB_MACHINE_TYPE         "" ""  3   64
  buv_string_enroll  RBRR  RBRR_GCB_TIMEOUT              "" ""  2   10
  buv_decimal_enroll RBRR  RBRR_GCB_MIN_CONCURRENT_BUILDS "" "" 1  999

  # Service Account Configuration Files
  buv_string_enroll  RBRR  RBRR_GOVERNOR_RBRA_FILE       "" ""  1  512
  buv_string_enroll  RBRR  RBRR_RETRIEVER_RBRA_FILE      "" ""  1  512
  buv_string_enroll  RBRR  RBRR_DIRECTOR_RBRA_FILE       "" ""  1  512

  # GCB image pins (digest-pinned)
  buv_odref_enroll   RBRR  RBRR_GCB_ORAS_IMAGE_REF      "" ""
  buv_odref_enroll   RBRR  RBRR_GCB_GCLOUD_IMAGE_REF    "" ""
  buv_odref_enroll   RBRR  RBRR_GCB_DOCKER_IMAGE_REF    "" ""
  buv_odref_enroll   RBRR  RBRR_GCB_ALPINE_IMAGE_REF    "" ""
  buv_odref_enroll   RBRR  RBRR_GCB_SYFT_IMAGE_REF      "" ""
  buv_odref_enroll   RBRR  RBRR_GCB_BINFMT_IMAGE_REF    "" ""

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

# Enforce all RBRR enrollment validations and custom format checks
zrbrr_enforce() {
  zrbrr_sentinel

  buv_vet RBRR

  buv_dir_exists "${RBRR_VESSEL_DIR}"

  local z_platform=""
  for z_platform in ${RBRR_MANIFEST_PLATFORMS}; do
    [[ "${z_platform}" =~ ^[a-z0-9_]+$ ]] \
      || buc_die "Invalid platform format in RBRR_MANIFEST_PLATFORMS: ${z_platform}"
  done

  [[ "${RBRR_GCB_TIMEOUT}" =~ ^[0-9]+s$ ]] \
    || buc_die "Invalid RBRR_GCB_TIMEOUT format: ${RBRR_GCB_TIMEOUT} (expected NNNs)"

  [[ "${RBRR_CHOSEN_PODMAN_VERSION}" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]] \
    || buc_die "Invalid RBRR_CHOSEN_PODMAN_VERSION format: ${RBRR_CHOSEN_PODMAN_VERSION} (expected N.N or N.N.N)"
}

# eof
