#!/bin/bash
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
# Recipe Bottle Container Registry - Implementation Layer

set -euo pipefail

ZRBCRI_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

######################################################################
# Environment Validation

rbcri_validate_env() {
  # Core variables required by all functions
  test -n "${RBRR_REGISTRY_OWNER:-}" || { echo "Error: RBRR_REGISTRY_OWNER not set" >&2; return 1; }
  test -n "${RBRR_REGISTRY_NAME:-}"  || { echo "Error: RBRR_REGISTRY_NAME not set" >&2; return 1; }
  test -n "${RBRR_REGISTRY:-}"       || { echo "Error: RBRR_REGISTRY not set" >&2; return 1; }
  test -n "${RBC_TEMP_DIR:-}"        || { echo "Error: RBC_TEMP_DIR not set" >&2; return 1; }
  test -n "${RBC_RUNTIME:-}"         || { echo "Error: RBC_RUNTIME not set" >&2; return 1; }
  test -n "${RBC_RUNTIME_ARG+x}"     || { echo "Error: RBC_RUNTIME_ARG not set" >&2; return 1; }

  # Registry-specific validation
  case "${RBRR_REGISTRY}" in
    ghcr)
      test -n "${RBRG_PAT:-}"      || { echo "Error: RBRG_PAT not set for GHCR" >&2; return 1; }
      test -n "${RBRG_USERNAME:-}" || { echo "Error: RBRG_USERNAME not set for GHCR" >&2; return 1; }
      ;;
    ecr|acr|quay)
      # Additional registry-specific validations would go here
      ;;
    *)
      echo "Error: Unknown registry: ${RBRR_REGISTRY}" >&2
      return 1
      ;;
  esac

  return 0
}

######################################################################
# Module Variables

# OCI Registry v2 API Standard Media Types
RBC_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
RBC_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
RBC_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
RBC_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"

# Media type for GitHub API
RBC_MTYPE_GHV3="application/vnd.github.v3+json"

######################################################################
# Registry Dispatcher Functions (rbcri_*)

rbcri_start() {
  rbcri_validate_env || return 1

  # Load registry-specific implementation
  case "${RBRR_REGISTRY}" in
    ghcr)  source "${ZRBCRI_SCRIPT_DIR}/rbcg_GHCR.sh" ;;
    ecr)   source "${ZRBCRI_SCRIPT_DIR}/rbce_ECR.sh"  ;;
    acr)   source "${ZRBCRI_SCRIPT_DIR}/rbca_ACR.sh"  ;;
    quay)  source "${ZRBCRI_SCRIPT_DIR}/rbcq_Quay.sh" ;;
  esac

  # Call registry-specific start
  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_start ;;
    ecr)  rbce_start ;;
    acr)  rbca_start ;;
    quay) rbcq_start ;;
  esac
}

rbcri_push() {
  rbcri_validate_env || return 1
  local tag="$1"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_push "${tag}" ;;
    ecr)  rbce_push "${tag}" ;;
    acr)  rbca_push "${tag}" ;;
    quay) rbcq_push "${tag}" ;;
  esac
}

rbcri_pull() {
  rbcri_validate_env || return 1
  local tag="$1"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_pull "${tag}" ;;
    ecr)  rbce_pull "${tag}" ;;
    acr)  rbca_pull "${tag}" ;;
    quay) rbcq_pull "${tag}" ;;
  esac
}

rbcri_delete() {
  rbcri_validate_env || return 1
  local tag="$1"

  case "${RBRR_REGISTRY}" in
    ghcr) { echo "Error: GHCR deletion disabled due to layer management issues" >&2; return 1; } ;;
    ecr)  rbce_delete "${tag}" ;;
    acr)  rbca_delete "${tag}" ;;
    quay) rbcq_delete "${tag}" ;;
  esac
}

rbcri_exists() {
  rbcri_validate_env || return 1
  local tag="$1"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_exists "${tag}" ;;
    ecr)  rbce_exists "${tag}" ;;
    acr)  rbca_exists "${tag}" ;;
    quay) rbcq_exists "${tag}" ;;
  esac
}

rbcri_tags() {
  rbcri_validate_env || return 1
  local output_file="$1"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_tags "${output_file}" ;;
    ecr)  rbce_tags "${output_file}" ;;
    acr)  rbca_tags "${output_file}" ;;
    quay) rbcq_tags "${output_file}" ;;
  esac
}

rbcri_fetch_manifest() {
  rbcri_validate_env || return 1
  local tag="$1"
  local output_file="$2"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_fetch_manifest "${tag}" "${output_file}" ;;
    ecr)  rbce_fetch_manifest "${tag}" "${output_file}" ;;
    acr)  rbca_fetch_manifest "${tag}" "${output_file}" ;;
    quay) rbcq_fetch_manifest "${tag}" "${output_file}" ;;
  esac
}

rbcri_fetch_config_blob() {
  rbcri_validate_env || return 1
  local digest="$1"
  local output_file="$2"

  case "${RBRR_REGISTRY}" in
    ghcr) rbcg_fetch_config_blob "${digest}" "${output_file}" ;;
    ecr)  rbce_fetch_config_blob "${digest}" "${output_file}" ;;
    acr)  rbca_fetch_config_blob "${digest}" "${output_file}" ;;
    quay) rbcq_fetch_config_blob "${digest}" "${output_file}" ;;
  esac
}

# Process a single manifest
rbcri_process_single_manifest() {
  rbcri_validate_env || return 1

  local tag="$1"
  local manifest_file="$2"
  local platform="$3"
  local temp_detail="$4"

  local config_digest
  config_digest=$(jq -r '.config.digest' "${manifest_file}")

  test -n "${config_digest}" && test "${config_digest}" != "null" || {
    echo "Warning: null config.digest in manifest" >&2
    return 1
  }

  local config_out="${manifest_file%.json}_config.json"

  rbcri_fetch_config_blob "${config_digest}" "${config_out}" || {
    echo "Warning: Failed to fetch config blob" >&2
    return 1
  }

  local manifest_json config_json
  manifest_json="$(<"${manifest_file}")"
  config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${config_out}")

  if [ -n "${platform}" ]; then
    jq -n \
      --arg tag          "${tag}"           \
      --arg platform     "${platform}"      \
      --arg digest       "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
      {
        tag: $tag,
        platform: $platform,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${temp_detail}"
  else
    jq -n \
      --arg     tag      "${tag}"           \
      --arg     digest   "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
      {
        tag: $tag,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${temp_detail}"
  fi

  return 0
}

# eof
