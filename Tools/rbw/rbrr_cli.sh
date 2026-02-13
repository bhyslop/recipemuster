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
source "${ZRBRR_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbcr_render.sh"
source "${ZRBRR_CLI_SCRIPT_DIR}/rbru_update.sh"

######################################################################
# CLI Functions

zrbrr_cli_kindle() {
  test -z "${ZRBRR_CLI_KINDLED:-}" || buc_die "RBRR CLI already kindled"
  ZRBRR_CLI_KINDLED=1
}

# Command: validate - source file and validate (dies on first error)
rbrr_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrr_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrr_validate: file not found: ${z_file}"

  buc_step "Validating RBRR repo regime file: ${z_file}"

  # Use rbrr_load for standardized loading
  rbrr_load

  buc_step "RBRR repo regime valid"
}

# Command: render - diagnostic display then validate
rbrr_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrr_render: file argument required"
  test -f "${z_file}" || buc_die "rbrr_render: file not found: ${z_file}"

  # Source and kindle (no dying — show all fields before validation)
  source "${z_file}" || buc_die "rbrr_render: failed to source ${z_file}"
  zrbrr_broach
  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRR - Recipe Bottle Regime Repo${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Container Registry Configuration
  rbcr_section_begin "Container Registry Configuration"
  rbcr_section_item RBRR_REGISTRY_OWNER  xname   req  "Container registry owner identifier"
  rbcr_section_item RBRR_REGISTRY_NAME   xname   req  "Container registry name"
  rbcr_section_end

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
  rbcr_section_item RBRR_DEPOT_PROJECT_ID        gname   req  "GCP project ID for depot"
  rbcr_section_item RBRR_GCP_REGION              gname   req  "GCP region"
  rbcr_section_item RBRR_GAR_REPOSITORY          gname   req  "Google Artifact Registry repository name"
  rbcr_section_end

  # Google Cloud Build Configuration
  rbcr_section_begin "Google Cloud Build Configuration"
  rbcr_section_item RBRR_GCB_MACHINE_TYPE        string  req  "Machine type for Cloud Build"
  rbcr_section_item RBRR_GCB_TIMEOUT             string  req  "Build timeout (e.g., 1200s)"
  rbcr_section_item RBRR_GCB_GCRANE_IMAGE_REF    odref   req  "gcrane image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_ORAS_IMAGE_REF      odref   req  "oras image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_GCLOUD_IMAGE_REF    odref   req  "gcloud image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_DOCKER_IMAGE_REF    odref   req  "docker image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_SKOPEO_IMAGE_REF    odref   req  "skopeo image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_ALPINE_IMAGE_REF    odref   req  "alpine image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_SYFT_IMAGE_REF      odref   req  "syft image reference (digest-pinned)"
  rbcr_section_item RBRR_GCB_BINFMT_IMAGE_REF    odref   req  "binfmt image reference (digest-pinned)"
  rbcr_section_end

  # Service Account Configuration
  rbcr_section_begin "Service Account Configuration"
  rbcr_section_item RBRR_GOVERNOR_RBRA_FILE      string  req  "Governor service account key file"
  rbcr_section_item RBRR_RETRIEVER_RBRA_FILE     string  req  "Retriever service account key file"
  rbcr_section_item RBRR_DIRECTOR_RBRA_FILE      string  req  "Director service account key file"
  rbcr_section_end

  # Unexpected variables (from kindle, not gated)
  if test ${#ZRBRR_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected RBRR_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRR_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate (dies on first error, after full display)
  zrbrr_validate_fields
  echo "${ZBUC_GREEN}RBRR repo regime valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbrr_cli_kindle
zrbcc_kindle

z_command="${1:-}"

case "${z_command}" in
  validate|render)
    z_file="${RBCC_RBRR_FILE}"
    test -f "${z_file}" || buc_die "RBRR regime file not found: ${z_file}"
    case "${z_command}" in
      validate) rbrr_validate "${z_file}" ;;
      render)   rbrr_render "${z_file}" ;;
    esac
    ;;
  refresh_gcb_pins)
    zrbru_kindle
    rbru_refresh_gcb_pins
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrr_cli.sh {validate|render|refresh_gcb_pins}"
    ;;
esac

# eof
