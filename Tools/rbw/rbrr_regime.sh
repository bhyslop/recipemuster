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
  RBRR_GDC_CONNECTION_NAME="${RBRR_GDC_CONNECTION_NAME:-}"
  RBRR_GDC_REGION="${RBRR_GDC_REGION:-}"
  RBRR_GDC_REPO_LINK="${RBRR_GDC_REPO_LINK:-}"
  RBRR_GCB_TRIGGER_PATTERN="${RBRR_GCB_TRIGGER_PATTERN:-}"
  RBRR_GCB_ORAS_IMAGE_REF="${RBRR_GCB_ORAS_IMAGE_REF:-}"
  RBRR_GCB_GCLOUD_IMAGE_REF="${RBRR_GCB_GCLOUD_IMAGE_REF:-}"
  RBRR_GCB_DOCKER_IMAGE_REF="${RBRR_GCB_DOCKER_IMAGE_REF:-}"
  RBRR_GCB_ALPINE_IMAGE_REF="${RBRR_GCB_ALPINE_IMAGE_REF:-}"
  RBRR_GCB_SYFT_IMAGE_REF="${RBRR_GCB_SYFT_IMAGE_REF:-}"
  RBRR_GCB_BINFMT_IMAGE_REF="${RBRR_GCB_BINFMT_IMAGE_REF:-}"

  # Enroll all RBRR variables — single source of truth for validation and rendering

  buv_regime_enroll RBRR

  buv_group_enroll "Vessel and Local Configuration"
  buv_string_enroll  RBRR_VESSEL_DIR           1  255  "Vessel definitions directory"
  buv_ipv4_enroll    RBRR_DNS_SERVER                   "DNS server for containers"

  buv_group_enroll "Build Tool Configuration"
  buv_xname_enroll   RBRR_IGNITE_MACHINE_NAME      1   64  "Podman machine for ignite operations"
  buv_xname_enroll   RBRR_DEPLOY_MACHINE_NAME      1   64  "Podman machine for deploy operations"
  buv_string_enroll  RBRR_CRANE_TAR_GZ             1  512  "Crane binary archive path"
  buv_string_enroll  RBRR_MANIFEST_PLATFORMS       1  512  "Target platforms for manifests"
  buv_string_enroll  RBRR_CHOSEN_PODMAN_VERSION    1   16  "Podman version (semantic version)"
  buv_fqin_enroll    RBRR_CHOSEN_VMIMAGE_ORIGIN    1  256  "VM image origin reference"
  buv_string_enroll  RBRR_CHOSEN_IDENTITY          1  128  "Identity for operations"

  buv_group_enroll "GCP Infrastructure"
  buv_gname_enroll   RBRR_DEPOT_PROJECT_ID         6   63  "GCP project ID for depot"
  buv_gname_enroll   RBRR_GCP_REGION               1   32  "GCP region"
  buv_gname_enroll   RBRR_GAR_REPOSITORY           1   63  "Google Artifact Registry repository name"

  buv_group_enroll "Google Developer Connect"
  buv_gname_enroll   RBRR_GDC_CONNECTION_NAME       0   63  "Developer Connect connection resource name"
  buv_gname_enroll   RBRR_GDC_REGION                0   32  "Developer Connect region"
  buv_gname_enroll   RBRR_GDC_REPO_LINK             0   63  "Git repository link resource name"

  buv_group_enroll "Google Cloud Build Configuration"
  buv_string_enroll  RBRR_GCB_TRIGGER_PATTERN        1  128  "Naming pattern for per-vessel triggers"
  buv_string_enroll  RBRR_GCB_MACHINE_TYPE           3   64  "Machine type for Cloud Build"
  buv_string_enroll  RBRR_GCB_TIMEOUT                2   10  "Build timeout (e.g., 1200s)"
  buv_decimal_enroll RBRR_GCB_MIN_CONCURRENT_BUILDS  1  999  "Min concurrent builds required"
  buv_odref_enroll   RBRR_GCB_ORAS_IMAGE_REF                 "oras image reference (digest-pinned)"
  buv_odref_enroll   RBRR_GCB_GCLOUD_IMAGE_REF               "gcloud image reference (digest-pinned)"
  buv_odref_enroll   RBRR_GCB_DOCKER_IMAGE_REF               "docker image reference (digest-pinned)"
  buv_odref_enroll   RBRR_GCB_ALPINE_IMAGE_REF               "alpine image reference (digest-pinned)"
  buv_odref_enroll   RBRR_GCB_SYFT_IMAGE_REF                 "syft image reference (digest-pinned)"
  buv_odref_enroll   RBRR_GCB_BINFMT_IMAGE_REF               "binfmt image reference (digest-pinned)"

  buv_group_enroll "Service Account Configuration"
  buv_string_enroll  RBRR_GOVERNOR_RBRA_FILE       1  512  "Governor service account key file"
  buv_string_enroll  RBRR_RETRIEVER_RBRA_FILE      1  512  "Retriever service account key file"
  buv_string_enroll  RBRR_DIRECTOR_RBRA_FILE       1  512  "Director service account key file"

  # Guard against unexpected RBRR_ variables not in enrollment
  buv_scope_sentinel RBRR RBRR_

  # Build docker env args array for container injection
  # Usage: docker run "${ZRBRR_DOCKER_ENV[@]}" ...
  # Currently only RBRR_DNS_SERVER is needed by sentry; add others as needed
  ZRBRR_DOCKER_ENV=()
  ZRBRR_DOCKER_ENV+=("-e" "RBRR_DNS_SERVER=${RBRR_DNS_SERVER}")

  # Temp file prefixes for rbrr_refresh_gcb_pins
  ZRBRR_REFRESH_PREFIX="${BURD_TEMP_DIR}/rbrr_refresh_"
  ZRBRR_REFRESH_SED_PREFIX="${BURD_TEMP_DIR}/rbrr_refresh_sed_"

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

  [[ "${RBRR_GCB_TRIGGER_PATTERN}" == *"{vessel}"* ]] \
    || buc_die "RBRR_GCB_TRIGGER_PATTERN must contain {vessel} placeholder: ${RBRR_GCB_TRIGGER_PATTERN}"
}

# eof
