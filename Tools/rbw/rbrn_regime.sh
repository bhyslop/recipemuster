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

  # Set defaults for all fields (validate enforces required-ness)
  RBRN_MONIKER="${RBRN_MONIKER:-}"
  RBRN_DESCRIPTION="${RBRN_DESCRIPTION:-}"
  RBRN_RUNTIME="${RBRN_RUNTIME:-}"
  RBRN_SENTRY_VESSEL="${RBRN_SENTRY_VESSEL:-}"
  RBRN_BOTTLE_VESSEL="${RBRN_BOTTLE_VESSEL:-}"
  RBRN_SENTRY_CONSECRATION="${RBRN_SENTRY_CONSECRATION:-}"
  RBRN_BOTTLE_CONSECRATION="${RBRN_BOTTLE_CONSECRATION:-}"
  RBRN_ENTRY_MODE="${RBRN_ENTRY_MODE:-}"
  RBRN_ENTRY_PORT_WORKSTATION="${RBRN_ENTRY_PORT_WORKSTATION:-}"
  RBRN_ENTRY_PORT_ENCLAVE="${RBRN_ENTRY_PORT_ENCLAVE:-}"
  RBRN_ENCLAVE_BASE_IP="${RBRN_ENCLAVE_BASE_IP:-}"
  RBRN_ENCLAVE_NETMASK="${RBRN_ENCLAVE_NETMASK:-}"
  RBRN_ENCLAVE_SENTRY_IP="${RBRN_ENCLAVE_SENTRY_IP:-}"
  RBRN_ENCLAVE_BOTTLE_IP="${RBRN_ENCLAVE_BOTTLE_IP:-}"
  RBRN_UPLINK_PORT_MIN="${RBRN_UPLINK_PORT_MIN:-}"
  RBRN_UPLINK_DNS_MODE="${RBRN_UPLINK_DNS_MODE:-}"
  RBRN_UPLINK_ACCESS_MODE="${RBRN_UPLINK_ACCESS_MODE:-}"
  RBRN_UPLINK_ALLOWED_CIDRS="${RBRN_UPLINK_ALLOWED_CIDRS:-}"
  RBRN_UPLINK_ALLOWED_DOMAINS="${RBRN_UPLINK_ALLOWED_DOMAINS:-}"
  RBRN_VOLUME_MOUNTS="${RBRN_VOLUME_MOUNTS:-}"

  # Detect unexpected RBRN_ variables
  local z_known="RBRN_MONIKER RBRN_DESCRIPTION RBRN_RUNTIME RBRN_SENTRY_VESSEL RBRN_BOTTLE_VESSEL RBRN_SENTRY_CONSECRATION RBRN_BOTTLE_CONSECRATION RBRN_ENTRY_MODE RBRN_ENTRY_PORT_WORKSTATION RBRN_ENTRY_PORT_ENCLAVE RBRN_ENCLAVE_BASE_IP RBRN_ENCLAVE_NETMASK RBRN_ENCLAVE_SENTRY_IP RBRN_ENCLAVE_BOTTLE_IP RBRN_UPLINK_PORT_MIN RBRN_UPLINK_DNS_MODE RBRN_UPLINK_ACCESS_MODE RBRN_UPLINK_ALLOWED_CIDRS RBRN_UPLINK_ALLOWED_DOMAINS RBRN_VOLUME_MOUNTS"
  ZRBRN_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v RBRN_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZRBRN_UNEXPECTED+=("${z_var}") ;;
    esac
  done

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

# Validate RBRN variables via buv_env_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zrbrn_validate_fields() {
  zrbrn_sentinel

  # Die on unexpected variables
  if test ${#ZRBRN_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected RBRN_ variables: ${ZRBRN_UNEXPECTED[*]}"
  fi

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

  # Verify IPs fall within declared subnet
  zrbrn_ip_in_subnet RBRN_ENCLAVE_SENTRY_IP "${RBRN_ENCLAVE_SENTRY_IP}" "${RBRN_ENCLAVE_BASE_IP}" "${RBRN_ENCLAVE_NETMASK}"
  zrbrn_ip_in_subnet RBRN_ENCLAVE_BOTTLE_IP "${RBRN_ENCLAVE_BOTTLE_IP}" "${RBRN_ENCLAVE_BASE_IP}" "${RBRN_ENCLAVE_NETMASK}"

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
}

######################################################################
# Public Functions (rbrn_*)

# Load nameplate regime by moniker
# Usage: rbrn_load_moniker <moniker>
# Constructs path, verifies file exists, sources, kindles, and validates
rbrn_load_moniker() {
  local z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbrn_load_moniker: moniker argument required"

  local z_nameplate_file="${RBCC_KIT_DIR}/${RBCC_RBRN_PREFIX}${z_moniker}${RBCC_RBRN_EXT}"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"

  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"
  zrbrn_kindle
  zrbrn_validate_fields
}

# Load nameplate regime by file path
# Usage: rbrn_load_file <path>
# Takes explicit file path, sources, kindles, and validates
rbrn_load_file() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_load_file: file argument required"
  test -f "${z_file}" || buc_die "rbrn_load_file: file not found: ${z_file}"

  source "${z_file}" || buc_die "Failed to source nameplate: ${z_file}"
  zrbrn_kindle
  zrbrn_validate_fields
}

# List available nameplate monikers
# Usage: rbrn_list
# Returns list of concrete nameplate monikers by globbing rbrn_*.env files
rbrn_list() {
  local z_nameplate_files=("${RBCC_KIT_DIR}/${RBCC_RBRN_PREFIX}"*"${RBCC_RBRN_EXT}")

  for z_file in "${z_nameplate_files[@]}"; do
    test -f "${z_file}" || continue
    local z_basename="${z_file##*/}"
    local z_moniker="${z_basename#${RBCC_RBRN_PREFIX}}"
    z_moniker="${z_moniker%${RBCC_RBRN_EXT}}"
    echo "${z_moniker}"
  done
}

######################################################################
# Cross-Nameplate Functions
#
# These iterate all nameplates by direct-sourcing .env files in
# subshells to avoid kindle-once guard conflicts.
#
# rbrn_preflight: Requires RBCC kindled
# rbrn_survey:    Requires RBCC, RBGC, RBGD kindled + RBRR loaded
# rbrn_audit:     Requires RBCC, RBGC, RBGD kindled + RBRR loaded

# Convert dotted-quad IPv4 to integer for subnet arithmetic
zrbrn_ip_to_int() {
  local z_a z_b z_c z_d
  IFS='.' read -r z_a z_b z_c z_d <<< "$1"
  echo $(( (z_a << 24) + (z_b << 16) + (z_c << 8) + z_d ))
}

# Validate that an IP falls within a subnet (dies if not)
# Usage: zrbrn_ip_in_subnet LABEL IP BASE MASK
zrbrn_ip_in_subnet() {
  local z_label="$1" z_ip="$2" z_base="$3" z_mask="$4"
  local z_ip_int=$(zrbrn_ip_to_int "${z_ip}")
  local z_base_int=$(zrbrn_ip_to_int "${z_base}")
  local z_net_mask=$(( (0xFFFFFFFF << (32 - z_mask)) & 0xFFFFFFFF ))
  if [[ $(( z_ip_int & z_net_mask )) -ne $(( z_base_int & z_net_mask )) ]]; then
    buc_die "${z_label}=${z_ip} is not within subnet ${z_base}/${z_mask}"
  fi
}

# Cross-nameplate conflict validation (silent on success, dies on conflict)
# Checks: port uniqueness, subnet non-overlap, enclave IP uniqueness
rbrn_preflight() {
  zrbcc_sentinel

  # Collect structured data from all nameplates via subshell sourcing
  local z_data
  z_data=$(
    for z_m in $(rbrn_list); do
      z_f="${RBCC_KIT_DIR}/${RBCC_RBRN_PREFIX}${z_m}${RBCC_RBRN_EXT}"
      (
        source "${z_f}"
        echo "${RBRN_MONIKER}|${RBRN_ENTRY_MODE}|${RBRN_ENTRY_PORT_WORKSTATION:-0}|${RBRN_ENTRY_PORT_ENCLAVE:-0}|${RBRN_ENCLAVE_BASE_IP}|${RBRN_ENCLAVE_NETMASK}|${RBRN_ENCLAVE_SENTRY_IP}|${RBRN_ENCLAVE_BOTTLE_IP}"
      )
    done
  )

  # Parallel arrays for conflict detection (bash 3.2 compatible)
  local z_ws_port_keys=()
  local z_ws_port_vals=()
  local z_enc_port_keys=()
  local z_enc_port_vals=()
  local z_ip_keys=()
  local z_ip_vals=()
  local z_net_starts=()
  local z_net_ends=()
  local z_net_owners=()

  local z_mon z_entry z_ws z_enc z_base z_mask z_sentry z_bottle
  while IFS='|' read -r z_mon z_entry z_ws z_enc z_base z_mask z_sentry z_bottle; do
    test -n "${z_mon}" || continue

    # Workstation and enclave port uniqueness (enabled entries only)
    if [[ "${z_entry}" == "enabled" ]]; then
      local z_i
      for z_i in "${!z_ws_port_keys[@]}"; do
        if [[ "${z_ws_port_keys[$z_i]}" == "${z_ws}" ]]; then
          buc_die "Port conflict: RBRN_ENTRY_PORT_WORKSTATION=${z_ws} claimed by both ${z_ws_port_vals[$z_i]} and ${z_mon}"
        fi
      done
      z_ws_port_keys+=("${z_ws}")
      z_ws_port_vals+=("${z_mon}")

      for z_i in "${!z_enc_port_keys[@]}"; do
        if [[ "${z_enc_port_keys[$z_i]}" == "${z_enc}" ]]; then
          buc_die "Port conflict: RBRN_ENTRY_PORT_ENCLAVE=${z_enc} claimed by both ${z_enc_port_vals[$z_i]} and ${z_mon}"
        fi
      done
      z_enc_port_keys+=("${z_enc}")
      z_enc_port_vals+=("${z_mon}")
    fi

    # Enclave IP uniqueness (all sentry and bottle IPs across nameplates)
    local z_j
    for z_j in "${!z_ip_keys[@]}"; do
      if [[ "${z_ip_keys[$z_j]}" == "${z_sentry}" ]]; then
        buc_die "IP conflict: ${z_sentry} claimed by ${z_mon} (sentry) and ${z_ip_vals[$z_j]}"
      fi
    done
    z_ip_keys+=("${z_sentry}")
    z_ip_vals+=("${z_mon}:sentry")

    for z_j in "${!z_ip_keys[@]}"; do
      if [[ "${z_ip_keys[$z_j]}" == "${z_bottle}" ]]; then
        buc_die "IP conflict: ${z_bottle} claimed by ${z_mon} (bottle) and ${z_ip_vals[$z_j]}"
      fi
    done
    z_ip_keys+=("${z_bottle}")
    z_ip_vals+=("${z_mon}:bottle")

    # Subnet non-overlap
    local z_net_int=$(zrbrn_ip_to_int "${z_base}")
    local z_net_mask_bits=$(( (0xFFFFFFFF << (32 - z_mask)) & 0xFFFFFFFF ))
    local z_net_addr=$(( z_net_int & z_net_mask_bits ))
    local z_net_size=$(( 1 << (32 - z_mask) ))
    local z_net_end=$(( z_net_addr + z_net_size - 1 ))

    local z_k
    for z_k in "${!z_net_starts[@]}"; do
      if [[ ${z_net_addr} -le ${z_net_ends[$z_k]} ]] && [[ ${z_net_starts[$z_k]} -le ${z_net_end} ]]; then
        buc_die "Subnet overlap: ${z_base}/${z_mask} (${z_mon}) overlaps with network of ${z_net_owners[$z_k]}"
      fi
    done
    z_net_starts+=("${z_net_addr}")
    z_net_ends+=("${z_net_end}")
    z_net_owners+=("${z_mon}")

  done <<< "${z_data}"
}

# Fleet info table — non-opinionated display of all nameplate configuration
# Requires: RBCC, RBGC, RBGD kindled + RBRR loaded
rbrn_survey() {
  zrbcc_sentinel
  zrbgc_sentinel
  zrbgd_sentinel
  zrbrr_sentinel

  local z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"

  echo ""
  printf "%-10s %-8s %6s %6s  %-17s %-14s %-14s  %3s %3s\n" \
    "Moniker" "Entry" "WS" "Enc" "Subnet" "Sentry IP" "Bottle IP" "Snt" "Btl"
  printf "%-10s %-8s %6s %6s  %-17s %-14s %-14s  %3s %3s\n" \
    "--------" "-----" "------" "------" "-----------------" "--------------" "--------------" "---" "---"

  local z_moniker z_file
  for z_moniker in $(rbrn_list); do
    z_file="${RBCC_KIT_DIR}/${RBCC_RBRN_PREFIX}${z_moniker}${RBCC_RBRN_EXT}"
    (
      source "${z_file}"

      local z_sentry_img="${z_gar_base}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}"
      local z_bottle_img="${z_gar_base}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}"

      local z_sentry_local="--"
      local z_bottle_local="--"
      if command -v "${RBRN_RUNTIME}" >/dev/null 2>&1; then
        ${RBRN_RUNTIME} image inspect "${z_sentry_img}" >/dev/null 2>&1 && z_sentry_local="ok"
        ${RBRN_RUNTIME} image inspect "${z_bottle_img}" >/dev/null 2>&1 && z_bottle_local="ok"
      else
        z_sentry_local="??"
        z_bottle_local="??"
      fi

      local z_ws_port="${RBRN_ENTRY_PORT_WORKSTATION:-}"
      local z_enc_port="${RBRN_ENTRY_PORT_ENCLAVE:-}"
      if [[ "${RBRN_ENTRY_MODE}" != "enabled" ]]; then
        z_ws_port="-"
        z_enc_port="-"
      fi

      printf "%-10s %-8s %6s %6s  %-17s %-14s %-14s  %3s %3s\n" \
        "${RBRN_MONIKER}" "${RBRN_ENTRY_MODE}" "${z_ws_port}" "${z_enc_port}" \
        "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
        "${RBRN_ENCLAVE_SENTRY_IP}" "${RBRN_ENCLAVE_BOTTLE_IP}" \
        "${z_sentry_local}" "${z_bottle_local}"
    )
  done
  echo ""
}

# Audit — survey display then preflight validation
rbrn_audit() {
  rbrn_survey
  rbrn_preflight
  buc_step "Cross-nameplate audit passed"
}

# eof
