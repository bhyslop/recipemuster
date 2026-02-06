#!/bin/bash
#
# Copyright 2024 Scale Invariant, Inc.
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
# Recipe Bottle Regime Nameplate - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRN_SOURCED:-}" || buc_die "Module rbrn multiply sourced - check sourcing hierarchy"
ZRBRN_SOURCED=1

######################################################################
# Internal Functions (zrbrn_*)

zrbrn_kindle() {
  test -z "${ZRBRN_KINDLED:-}" || buc_die "Module rbrn already kindled"

  # Set defaults for optional fields (Required: No per RBRN spec)
  RBRN_DESCRIPTION="${RBRN_DESCRIPTION:-}"
  RBRN_VOLUME_MOUNTS="${RBRN_VOLUME_MOUNTS:-}"

  # Set defaults for conditional fields (may not be provided if feature disabled)
  RBRN_ENTRY_PORT_WORKSTATION="${RBRN_ENTRY_PORT_WORKSTATION:-}"
  RBRN_ENTRY_PORT_ENCLAVE="${RBRN_ENTRY_PORT_ENCLAVE:-}"
  RBRN_UPLINK_ALLOWED_CIDRS="${RBRN_UPLINK_ALLOWED_CIDRS:-}"
  RBRN_UPLINK_ALLOWED_DOMAINS="${RBRN_UPLINK_ALLOWED_DOMAINS:-}"

  # Core nameplate identification
  buv_env_xname       RBRN_MONIKER                 2     12
  buv_env_string      RBRN_DESCRIPTION             0    120
  buv_env_string      RBRN_RUNTIME                 1     16

  # Validate runtime is docker or podman
  case "${RBRN_RUNTIME}" in
    docker|podman) : ;;
    *) buc_die "Invalid RBRN_RUNTIME: '${RBRN_RUNTIME}' (must be 'docker' or 'podman')" ;;
  esac

  # Container image configuration
  buv_env_fqin        RBRN_SENTRY_VESSEL        1    128
  buv_env_fqin        RBRN_BOTTLE_VESSEL        1    128
  buv_env_fqin        RBRN_SENTRY_CONSECRATION        1    128
  buv_env_fqin        RBRN_BOTTLE_CONSECRATION        1    128

  # Entry point configuration
  case "${RBRN_ENTRY_MODE}" in
    disabled|enabled) : ;;
    *) buc_die "Invalid RBRN_ENTRY_MODE: '${RBRN_ENTRY_MODE}' (must be 'disabled' or 'enabled')" ;;
  esac

  # Enclave network configuration
  buv_env_ipv4        RBRN_ENCLAVE_BASE_IP
  buv_env_decimal     RBRN_ENCLAVE_NETMASK         8     30
  buv_env_ipv4        RBRN_ENCLAVE_SENTRY_IP
  buv_env_ipv4        RBRN_ENCLAVE_BOTTLE_IP

  # Uplink configuration
  buv_env_port        RBRN_UPLINK_PORT_MIN
  case "${RBRN_UPLINK_DNS_MODE}" in
    disabled|global|allowlist) : ;;
    *) buc_die "Invalid RBRN_UPLINK_DNS_MODE: '${RBRN_UPLINK_DNS_MODE}' (must be 'disabled', 'global', or 'allowlist')" ;;
  esac
  case "${RBRN_UPLINK_ACCESS_MODE}" in
    disabled|global|allowlist) : ;;
    *) buc_die "Invalid RBRN_UPLINK_ACCESS_MODE: '${RBRN_UPLINK_ACCESS_MODE}' (must be 'disabled', 'global', or 'allowlist')" ;;
  esac

  # Conditional entry port validation (Required: When ENTRY_MODE=enabled)
  if [[ $RBRN_ENTRY_MODE == enabled ]]; then
    buv_env_port    RBRN_ENTRY_PORT_WORKSTATION
    buv_env_port    RBRN_ENTRY_PORT_ENCLAVE

    test ${RBRN_ENTRY_PORT_WORKSTATION} -lt ${RBRN_UPLINK_PORT_MIN} || \
      buc_die "RBRN_ENTRY_PORT_WORKSTATION must be less than RBRN_UPLINK_PORT_MIN"
    test ${RBRN_ENTRY_PORT_ENCLAVE} -lt ${RBRN_UPLINK_PORT_MIN} || \
      buc_die "RBRN_ENTRY_PORT_ENCLAVE must be less than RBRN_UPLINK_PORT_MIN"
  fi

  # Conditional allowlist validation (Required: When *_MODE=allowlist)
  if [[ ${RBRN_UPLINK_ACCESS_MODE} == allowlist ]]; then
    buv_env_list_cidr RBRN_UPLINK_ALLOWED_CIDRS
  fi
  if [[ ${RBRN_UPLINK_DNS_MODE} == allowlist ]]; then
    buv_env_list_domain RBRN_UPLINK_ALLOWED_DOMAINS
  fi

  # Volume mount configuration (Required: No)
  buv_env_string      RBRN_VOLUME_MOUNTS           0    240

  # Build rollup of all RBRN_ variables for passing to scripts/containers
  ZRBRN_ROLLUP=""
  ZRBRN_ROLLUP+="RBRN_MONIKER='${RBRN_MONIKER}' "
  ZRBRN_ROLLUP+="RBRN_DESCRIPTION='${RBRN_DESCRIPTION}' "
  ZRBRN_ROLLUP+="RBRN_RUNTIME='${RBRN_RUNTIME}' "
  ZRBRN_ROLLUP+="RBRN_SENTRY_VESSEL='${RBRN_SENTRY_VESSEL}' "
  ZRBRN_ROLLUP+="RBRN_BOTTLE_VESSEL='${RBRN_BOTTLE_VESSEL}' "
  ZRBRN_ROLLUP+="RBRN_SENTRY_CONSECRATION='${RBRN_SENTRY_CONSECRATION}' "
  ZRBRN_ROLLUP+="RBRN_BOTTLE_CONSECRATION='${RBRN_BOTTLE_CONSECRATION}' "
  ZRBRN_ROLLUP+="RBRN_ENTRY_MODE='${RBRN_ENTRY_MODE}' "
  ZRBRN_ROLLUP+="RBRN_ENTRY_PORT_WORKSTATION='${RBRN_ENTRY_PORT_WORKSTATION}' "
  ZRBRN_ROLLUP+="RBRN_ENTRY_PORT_ENCLAVE='${RBRN_ENTRY_PORT_ENCLAVE}' "
  ZRBRN_ROLLUP+="RBRN_ENCLAVE_BASE_IP='${RBRN_ENCLAVE_BASE_IP}' "
  ZRBRN_ROLLUP+="RBRN_ENCLAVE_NETMASK='${RBRN_ENCLAVE_NETMASK}' "
  ZRBRN_ROLLUP+="RBRN_ENCLAVE_SENTRY_IP='${RBRN_ENCLAVE_SENTRY_IP}' "
  ZRBRN_ROLLUP+="RBRN_ENCLAVE_BOTTLE_IP='${RBRN_ENCLAVE_BOTTLE_IP}' "
  ZRBRN_ROLLUP+="RBRN_UPLINK_PORT_MIN='${RBRN_UPLINK_PORT_MIN}' "
  ZRBRN_ROLLUP+="RBRN_UPLINK_DNS_MODE='${RBRN_UPLINK_DNS_MODE}' "
  ZRBRN_ROLLUP+="RBRN_UPLINK_ACCESS_MODE='${RBRN_UPLINK_ACCESS_MODE}' "
  ZRBRN_ROLLUP+="RBRN_UPLINK_ALLOWED_CIDRS='${RBRN_UPLINK_ALLOWED_CIDRS}' "
  ZRBRN_ROLLUP+="RBRN_UPLINK_ALLOWED_DOMAINS='${RBRN_UPLINK_ALLOWED_DOMAINS}' "
  ZRBRN_ROLLUP+="RBRN_VOLUME_MOUNTS='${RBRN_VOLUME_MOUNTS}'"

  # Build docker env args array for container injection
  # Usage: docker run "${ZRBRN_DOCKER_ENV[@]}" ...
  ZRBRN_DOCKER_ENV=()
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_MONIKER=${RBRN_MONIKER}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_DESCRIPTION=${RBRN_DESCRIPTION}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_RUNTIME=${RBRN_RUNTIME}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_SENTRY_VESSEL=${RBRN_SENTRY_VESSEL}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_BOTTLE_VESSEL=${RBRN_BOTTLE_VESSEL}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_SENTRY_CONSECRATION=${RBRN_SENTRY_CONSECRATION}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_BOTTLE_CONSECRATION=${RBRN_BOTTLE_CONSECRATION}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENTRY_MODE=${RBRN_ENTRY_MODE}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENTRY_PORT_WORKSTATION=${RBRN_ENTRY_PORT_WORKSTATION}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENTRY_PORT_ENCLAVE=${RBRN_ENTRY_PORT_ENCLAVE}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENCLAVE_BASE_IP=${RBRN_ENCLAVE_BASE_IP}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENCLAVE_NETMASK=${RBRN_ENCLAVE_NETMASK}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENCLAVE_SENTRY_IP=${RBRN_ENCLAVE_SENTRY_IP}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_ENCLAVE_BOTTLE_IP=${RBRN_ENCLAVE_BOTTLE_IP}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_UPLINK_PORT_MIN=${RBRN_UPLINK_PORT_MIN}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_UPLINK_DNS_MODE=${RBRN_UPLINK_DNS_MODE}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_UPLINK_ACCESS_MODE=${RBRN_UPLINK_ACCESS_MODE}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_UPLINK_ALLOWED_CIDRS=${RBRN_UPLINK_ALLOWED_CIDRS}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_UPLINK_ALLOWED_DOMAINS=${RBRN_UPLINK_ALLOWED_DOMAINS}")
  ZRBRN_DOCKER_ENV+=("-e" "RBRN_VOLUME_MOUNTS=${RBRN_VOLUME_MOUNTS}")

  ZRBRN_KINDLED=1
}

zrbrn_sentinel() {
  test "${ZRBRN_KINDLED:-}" = "1" || buc_die "Module rbrn not kindled - call zrbrn_kindle first"
}

# eof
