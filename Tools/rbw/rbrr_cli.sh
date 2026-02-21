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
# RBRR CLI - Command line interface for RBRR repo regime operations

set -euo pipefail

ZRBRR_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRR_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBCC_rbrr_file}"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# Internal Functions

zrbrr_cli_kindle() {
  test -z "${ZRBRR_CLI_KINDLED:-}" || buc_die "RBRR CLI already kindled"

  # GCB runs on linux/amd64 — pin digests must match this platform
  ZRBRR_CLI_PIN_OS="linux"
  ZRBRR_CLI_PIN_ARCH="amd64"

  ZRBRR_CLI_KINDLED=1
}

######################################################################
# Command Functions

# Command: validate - enrollment-based validation report
rbrr_validate() {
  buc_doc_brief "Validate RBRR repo regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRR repo regime file: ${RBCC_rbrr_file}"
  buv_report RBRR "Repository Regime"
  buc_step "RBRR repo regime valid"
}

# Command: render - diagnostic display of all RBRR fields
rbrr_render() {
  buc_doc_brief "Display diagnostic view of RBRR repo regime configuration"
  buc_doc_shown || return 0

  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRR - Recipe Bottle Regime Repo${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${RBCC_rbrr_file}${ZBUC_RESET}"
  echo ""

  # Vessel and Local Configuration
  rbcr_section_begin "Vessel and Local Configuration"
  rbcr_section_item RBRR_VESSEL_DIR              string  req  "Vessel definitions directory"
  rbcr_section_item RBRR_DNS_SERVER              ipv4    req  "DNS server for containers"
  rbcr_section_item RBRR_IGNITE_MACHINE_NAME     xname   req  "Podman machine for ignite operations"
  rbcr_section_item RBRR_DEPLOY_MACHINE_NAME     xname   req  "Podman machine for deploy operations"
  rbcr_section_end

  # Build Tool Configuration
  rbcr_section_begin "Build Tool Configuration"
  rbcr_section_item RBRR_CRANE_TAR_GZ            string  req  "Crane binary archive path"
  rbcr_section_item RBRR_MANIFEST_PLATFORMS      string  req  "Target platforms for manifests"
  rbcr_section_item RBRR_CHOSEN_PODMAN_VERSION   string  req  "Podman version (semantic version)"
  rbcr_section_item RBRR_CHOSEN_VMIMAGE_ORIGIN   fqin    req  "VM image origin reference"
  rbcr_section_item RBRR_CHOSEN_IDENTITY         string  req  "Identity for operations"
  rbcr_section_end

  # GCP Infrastructure
  rbcr_section_begin "GCP Infrastructure"
  rbcr_section_item RBRR_DEPOT_PROJECT_ID      gname   req  "GCP project ID for depot"
  rbcr_section_item RBRR_GCP_REGION            gname   req  "GCP region"
  rbcr_section_item RBRR_GAR_REPOSITORY        gname   req  "Google Artifact Registry repository name"
  rbcr_section_end

  # Google Cloud Build Configuration
  rbcr_section_begin "Google Cloud Build Configuration"
  rbcr_section_item RBRR_GCB_MACHINE_TYPE            string  req  "Machine type for Cloud Build"
  rbcr_section_item RBRR_GCB_TIMEOUT                 string  req  "Build timeout (e.g., 1200s)"
  rbcr_section_item RBRR_GCB_MIN_CONCURRENT_BUILDS   decimal req "Min concurrent builds required"
  rbcr_section_item RBRR_GCB_ORAS_IMAGE_REF          odref   req  "oras image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_GCLOUD_IMAGE_REF        odref   req  "gcloud image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_DOCKER_IMAGE_REF        odref   req  "docker image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_ALPINE_IMAGE_REF        odref   req  "alpine image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_SYFT_IMAGE_REF          odref   req  "syft image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_BINFMT_IMAGE_REF        odref   req  "binfmt image reference (digest-pinned)"
  rbcr_section_end

  # Service Account Configuration
  rbcr_section_begin "Service Account Configuration"
  rbcr_section_item RBRR_GOVERNOR_RBRA_FILE    string  req  "Governor service account key file"
  rbcr_section_item RBRR_RETRIEVER_RBRA_FILE   string  req  "Retriever service account key file"
  rbcr_section_item RBRR_DIRECTOR_RBRA_FILE    string  req  "Director service account key file"
  rbcr_section_end

  echo "${ZBUC_GREEN}RBRR repo regime valid${ZBUC_RESET}"
}

# Command: refresh_gcb_pins - resolve image tags to digests and update RBRR file
# Every step must succeed or the function dies — no partial updates.
# Requires: docker, jq, curl, BURD_NOW_STAMP set, BURD_TEMP_DIR set
rbrr_refresh_gcb_pins() {
  buc_doc_brief "Resolve image tags to digests and update RBRR configuration file"
  buc_doc_shown || return 0

  local z_rbrr_file="${RBCC_rbrr_file}"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  zburd_sentinel

  buc_countdown 5 "NOTE: Docker anonymous login is heavily rate limited.  Try again in 6 hours if you see that fail.  Continuing in:"

  local z_vintage="${BURD_NOW_STAMP}"
  local z_oras_token_file="${BURD_TEMP_DIR}/rbrr_oras_token.json"
  local z_oras_tags_file="${BURD_TEMP_DIR}/rbrr_oras_tags.json"
  local z_oras_token_value_file="${BURD_TEMP_DIR}/rbrr_oras_token_value.txt"
  local z_oras_tag_file="${BURD_TEMP_DIR}/rbrr_oras_tag.txt"
  local z_refresh_prefix="${BURD_TEMP_DIR}/rbrr_refresh_"
  local z_refresh_sed_prefix="${BURD_TEMP_DIR}/rbrr_refresh_sed_"

  # Discover latest oras stable version from GHCR API.
  # oras doesn't publish a :latest tag, so we must find the newest semver release.
  # GHCR requires a bearer token even for public images.
  buc_step "Discovering latest oras version from GHCR (no :latest tag published)"

  buc_log_args "Obtaining GHCR bearer token (anonymous, scoped to oras repo)"
  curl -sS \
    "https://ghcr.io/token?scope=repository:oras-project/oras:pull&service=ghcr.io" \
    -o "${z_oras_token_file}" 2>/dev/null \
    || buc_die "Failed to fetch GHCR bearer token"

  jq -r '.token // empty' "${z_oras_token_file}" > "${z_oras_token_value_file}" \
    || buc_die "Failed to extract token from GHCR response"
  local z_oras_token
  z_oras_token=$(<"${z_oras_token_value_file}")
  test -n "${z_oras_token}" || buc_die "GHCR returned empty bearer token"

  buc_log_args "Fetching tag list using bearer token"
  curl -sS \
    -H "Authorization: Bearer ${z_oras_token}" \
    "https://ghcr.io/v2/oras-project/oras/tags/list?n=1000" \
    -o "${z_oras_tags_file}" 2>/dev/null \
    || buc_die "Failed to fetch oras tags from GHCR"

  buc_log_args "Extracting newest stable semver tag"
  # Filters to exact vN.N.N (no pre-release suffixes), sorts numerically, takes last
  jq -r '
    [.tags[] | select(test("^v[0-9]+\\.[0-9]+\\.[0-9]+$"))]
    | map(ltrimstr("v") | split(".") | map(tonumber))
    | sort_by(.[0], .[1], .[2])
    | last
    | "v\(.[0]).\(.[1]).\(.[2])"
  ' "${z_oras_tags_file}" > "${z_oras_tag_file}" \
    || buc_die "Failed to extract semver tag from oras tags response"
  local z_oras_tag
  z_oras_tag=$(<"${z_oras_tag_file}")
  test -n "${z_oras_tag}" -a "${z_oras_tag}" != "null" \
    || buc_die "No stable oras semver tag found in GHCR tag list"
  buc_info "oras discovered tag: ${z_oras_tag}"

  # Discover latest crane release from GitHub API.
  # crane is distributed as a tarball (not a container image), so it needs
  # its own freshening path separate from the image-pin loop below.
  local z_crane_releases_file="${BURD_TEMP_DIR}/rbrr_crane_releases.json"
  local z_crane_tag_file="${BURD_TEMP_DIR}/rbrr_crane_tag.txt"
  local z_crane_old_url_file="${BURD_TEMP_DIR}/rbrr_crane_old_url.txt"
  local z_crane_sed_file="${BURD_TEMP_DIR}/rbrr_crane_sed.sh"

  buc_step "Discovering latest crane release from GitHub"

  curl -sS \
    "https://api.github.com/repos/google/go-containerregistry/releases/latest" \
    -o "${z_crane_releases_file}" 2>/dev/null \
    || buc_die "Failed to fetch crane releases from GitHub API"

  buc_log_args "Extracting tag from latest release"
  jq -r '.tag_name // empty' "${z_crane_releases_file}" > "${z_crane_tag_file}" \
    || buc_die "Failed to extract tag from crane releases response"
  local z_crane_tag=$(<"${z_crane_tag_file}")
  test -n "${z_crane_tag}" || buc_die "No tag found in crane latest release"
  buc_info "crane discovered tag: ${z_crane_tag}"

  local z_crane_url="https://github.com/google/go-containerregistry/releases/download/${z_crane_tag}/go-containerregistry_Linux_x86_64.tar.gz"

  buc_log_args "Reading current RBRR_CRANE_TAR_GZ from rbrr file"
  grep "^RBRR_CRANE_TAR_GZ=" "${z_rbrr_file}" | cut -d'=' -f2- > "${z_crane_old_url_file}" \
    || buc_die "No existing value for RBRR_CRANE_TAR_GZ in rbrr file"
  local z_old_crane_url=$(<"${z_crane_old_url_file}")
  test -n "${z_old_crane_url}" || buc_die "Empty RBRR_CRANE_TAR_GZ value in rbrr file"

  if test "${z_old_crane_url}" = "${z_crane_url}"; then
    buc_info "RBRR_CRANE_TAR_GZ: unchanged (${z_crane_tag})"
  else
    buc_info "RBRR_CRANE_TAR_GZ: -> ${z_crane_url}"
    sed "s|^RBRR_CRANE_TAR_GZ=.*|RBRR_CRANE_TAR_GZ=${z_crane_url}|" "${z_rbrr_file}" > "${z_crane_sed_file}" \
      || buc_die "Failed to sed value for RBRR_CRANE_TAR_GZ"
    mv "${z_crane_sed_file}" "${z_rbrr_file}" || buc_die "Failed to update RBRR_CRANE_TAR_GZ in rbrr file"
  fi

  # Image specifications: VARNAME|BASE_IMAGE|TAG
  # Most images use :latest which always points to newest version.
  # oras uses discovered semver tag above (no :latest published).
  local z_specs=(
    "RBRR_GCB_ORAS_IMAGE_REF|ghcr.io/oras-project/oras|${z_oras_tag}"
    "RBRR_GCB_GCLOUD_IMAGE_REF|gcr.io/cloud-builders/gcloud|latest"
    "RBRR_GCB_DOCKER_IMAGE_REF|gcr.io/cloud-builders/docker|latest"
    "RBRR_GCB_ALPINE_IMAGE_REF|docker.io/library/alpine|latest"
    "RBRR_GCB_SYFT_IMAGE_REF|docker.io/anchore/syft|latest"
    "RBRR_GCB_BINFMT_IMAGE_REF|docker.io/tonistiigi/binfmt|latest"
  )

  buc_step "Refreshing GCB tool image pins (vintage: ~${z_vintage})"

  local z_updated=0
  local z_unchanged=0

  local z_spec=""
  local z_varname=""
  local z_image=""
  local z_tag=""
  local z_manifest_file=""
  local z_digest_file=""
  local z_oldref_file=""
  local z_digest=""
  local z_full_ref=""
  local z_old_ref=""
  local z_sed_value_file=""
  local z_sed_vintage_file=""
  for z_spec in "${z_specs[@]}"; do
    IFS='|' read -r z_varname z_image z_tag <<< "${z_spec}"

    z_manifest_file="${z_refresh_prefix}${z_varname}_manifest.json"
    z_digest_file="${z_refresh_prefix}${z_varname}_digest.txt"
    z_oldref_file="${z_refresh_prefix}${z_varname}_oldref.txt"
    z_sed_value_file="${z_refresh_sed_prefix}${z_varname}_value.sh"
    z_sed_vintage_file="${z_refresh_sed_prefix}${z_varname}_vintage.sh"

    buc_step "Inspecting ${z_image}:${z_tag}"

    docker manifest inspect --verbose "${z_image}:${z_tag}" > "${z_manifest_file}" 2>/dev/null \
      || buc_die "Failed to fetch manifest for ${z_image}:${z_tag}"

    buc_log_args "Extracting ${ZRBRR_CLI_PIN_OS}/${ZRBRR_CLI_PIN_ARCH} digest from manifest"
    jq -r --arg os "${ZRBRR_CLI_PIN_OS}" --arg arch "${ZRBRR_CLI_PIN_ARCH}" '
      if type == "array" then
        [.[] | select(.Descriptor.platform.architecture == $arch
                  and .Descriptor.platform.os == $os)][0].Descriptor.digest
      else
        .Descriptor.digest
      end' \
      "${z_manifest_file}" > "${z_digest_file}" \
      || buc_die "Failed to extract digest for ${z_image}:${z_tag}"
    z_digest=$(<"${z_digest_file}")
    test -n "${z_digest}" || buc_die "Empty digest for ${z_image}:${z_tag}"
    z_full_ref="${z_image}@${z_digest}"

    grep "^${z_varname}=" "${z_rbrr_file}" | cut -d'"' -f2 > "${z_oldref_file}" \
      || buc_die "No existing value for ${z_varname} in rbrr file"
    z_old_ref=$(<"${z_oldref_file}")

    if test "${z_old_ref}" = "${z_full_ref}"; then
      buc_info "${z_varname}: unchanged"
      z_unchanged=$((z_unchanged + 1))
    else
      buc_info "${z_varname}: ${z_old_ref} -> ${z_full_ref}"

      sed "s|^${z_varname}=.*|${z_varname}=\"${z_full_ref}\"|" "${z_rbrr_file}" > "${z_sed_value_file}" \
        || buc_die "Failed to sed value for ${z_varname}"
      mv "${z_sed_value_file}" "${z_rbrr_file}" || buc_die "Failed to update ${z_varname} in rbrr file"

      buc_log_args "Updating vintage comment for ${z_varname}"
      sed "/${z_varname}=/{
        x
        s|(~[^)]*)|(~${z_vintage})|
        x
      }" "${z_rbrr_file}" > "${z_sed_vintage_file}" \
        || buc_die "Failed to update vintage comment for ${z_varname}"
      test -s "${z_sed_vintage_file}" || buc_die "Vintage sed produced empty output for ${z_varname}"
      mv "${z_sed_vintage_file}" "${z_rbrr_file}" || buc_die "Failed to update vintage for ${z_varname}"

      z_updated=$((z_updated + 1))
    fi
  done

  buc_step "Refresh complete: ${z_updated} updated, ${z_unchanged} unchanged"
}

######################################################################
# Furnish and Main

zrbrr_furnish() {
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce

  zrbrr_cli_kindle
}

buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"

# eof
