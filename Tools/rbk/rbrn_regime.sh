#!/bin/bash
#
# Copyright 2024 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
test -z "${ZRBRN_SOURCED:-}" || buc_die_now "Module rbrn multiply sourced - check sourcing hierarchy"
ZRBRN_SOURCED=1

######################################################################
# Internal Functions (zrbrn_*)

zrbrn_kindle() {
  test -z "${ZRBRN_KINDLED:-}" || buc_die_now "Module rbrn already kindled"

  buv_dir_exists "${BURD_TEMP_DIR}"

  # The drive's rewrite of a nameplate's rbrn.env lands here before it moves into place
  readonly ZRBRN_DRIVE_PREFIX="${BURD_TEMP_DIR}/rbrn_drive_"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRN variables — single source of truth for validation and rendering

  buv_regime_enroll RBRN

  buv_group_enroll "Core Service Identity"
  buv_xname_enroll   RBRN_MONIKER                 2   12  "Unique identifier for Crucible"
  buv_string_enroll  RBRN_DESCRIPTION              0  120  "Human-readable description"
  buv_enum_enroll    RBRN_RUNTIME                              "Container runtime: docker or podman" \
                     docker podman

  buv_group_enroll "Container Image Configuration"
  buv_fqin_enroll    RBRN_SENTRY_VESSEL            1  128  "Vessel identifier for Sentry Image"
  buv_fqin_enroll    RBRN_BOTTLE_VESSEL            1  128  "Vessel identifier for Bottle Image"
  buv_fqin_enroll    RBRN_SENTRY_HALLMARK      0  128  "Hallmark tag for Sentry Image (vacant until a build is driven; marshal-zero blanks it)"
  buv_fqin_enroll    RBRN_BOTTLE_HALLMARK      0  128  "Hallmark tag for Bottle Image (vacant until a build is driven; marshal-zero blanks it)"

  buv_group_enroll "Service Readiness"
  buv_decimal_enroll RBRN_BOTTLE_READINESS_DELAY_SEC  0  300  "Seconds to wait after compose-up for bottle services to become ready (0 = no wait)"

  buv_group_enroll "Entry Service Configuration"
  buv_enum_enroll    RBRN_ENTRY_MODE                           "Entry functionality: disabled or enabled" \
                     rbnne_disabled rbnne_enabled
  buv_gate_enroll    RBRN_ENTRY_MODE  rbnne_enabled
  buv_port_enroll    RBRN_ENTRY_PORT_WORKSTATION               "External port on Transit Network"
  buv_port_enroll    RBRN_ENTRY_PORT_ENCLAVE                   "Enclave port between Sentry and Bottle"

  buv_group_enroll "Enclave Network Configuration"
  buv_ipv4_enroll    RBRN_ENCLAVE_BASE_IP                      "Base IPv4 for enclave network"
  buv_decimal_enroll RBRN_ENCLAVE_NETMASK          8   30  "Network mask width (8-30)"
  buv_ipv4_enroll    RBRN_ENCLAVE_SENTRY_IP                    "IP address for Sentry Container"
  buv_ipv4_enroll    RBRN_ENCLAVE_BOTTLE_IP                    "IP address for Bottle Container"

  buv_group_enroll "Uplink Core"
  buv_port_enroll    RBRN_UPLINK_PORT_MIN                      "Minimum port for outbound connections"
  buv_enum_enroll    RBRN_UPLINK_DNS_MODE                      "DNS mode: disabled, global, or allowlist" \
                     rbnne_disabled rbnne_global rbnne_allowlist
  buv_enum_enroll    RBRN_UPLINK_ACCESS_MODE                   "IP access mode: disabled, global, or allowlist" \
                     rbnne_disabled rbnne_global rbnne_allowlist

  buv_group_enroll "Uplink DNS Allowlist"
  buv_gate_enroll    RBRN_UPLINK_DNS_MODE  rbnne_allowlist
  buv_list_domain_enroll RBRN_UPLINK_ALLOWED_DOMAINS           "Allowed DNS domains"

  buv_group_enroll "Uplink Access Allowlist"
  buv_gate_enroll    RBRN_UPLINK_ACCESS_MODE  rbnne_allowlist
  buv_list_cidr_enroll   RBRN_UPLINK_ALLOWED_CIDRS             "Allowed CIDR ranges"

  # Guard against unexpected RBRN_ variables not in enrollment
  buv_scope_sentinel RBRN RBRN_

  # Lock all enrolled RBRN_ variables against mutation
  buv_lock RBRN

  readonly ZRBRN_KINDLED=1
}

zrbrn_sentinel() {
  test "${ZRBRN_KINDLED:-}" = "1" || buc_die_now "Module rbrn not kindled - call zrbrn_kindle first"
}

# Enforce all RBRN enrollment validations and custom format checks
zrbrn_enforce() {
  zrbrn_sentinel

  buv_vet RBRN

  # Verify IPs fall within declared subnet
  zrbrn_ip_in_subnet RBRN_ENCLAVE_SENTRY_IP "${RBRN_ENCLAVE_SENTRY_IP}" "${RBRN_ENCLAVE_BASE_IP}" "${RBRN_ENCLAVE_NETMASK}"
  zrbrn_ip_in_subnet RBRN_ENCLAVE_BOTTLE_IP "${RBRN_ENCLAVE_BOTTLE_IP}" "${RBRN_ENCLAVE_BASE_IP}" "${RBRN_ENCLAVE_NETMASK}"

  # Cross-port check (entry ports must be less than uplink port min)
  if test "${RBRN_ENTRY_MODE}" = "rbnne_enabled"; then
    test "${RBRN_ENTRY_PORT_WORKSTATION}" -lt "${RBRN_UPLINK_PORT_MIN}" || \
      buc_reject "${BUBC_band_regime}" "RBRN_ENTRY_PORT_WORKSTATION must be less than RBRN_UPLINK_PORT_MIN"
    test "${RBRN_ENTRY_PORT_ENCLAVE}" -lt "${RBRN_UPLINK_PORT_MIN}" || \
      buc_reject "${BUBC_band_regime}" "RBRN_ENTRY_PORT_ENCLAVE must be less than RBRN_UPLINK_PORT_MIN"
  fi

  # Build docker env args array from validated values
  # Usage: docker run "${ZRBRN_DOCKER_ENV[@]}" ...
  buv_docker_env RBRN ZRBRN_DOCKER_ENV
}

######################################################################
# Public Functions (rbrn_*)

# List available nameplate monikers as space-separated tokens
# Prerequisite: RBCC sourced (needs RBCC_moorings_dir, RBCC_rbrn_file)
rbrn_list_capture() {
  zrbrn_sentinel

  local z_result=""
  local -r z_files=("${RBCC_moorings_dir}/"*"/${RBCC_rbrn_file}")
  local z_i=""
  local z_dir=""
  local z_inscription=""
  local z_moniker=""
  for z_i in "${!z_files[@]}"; do
    test -f "${z_files[${z_i}]}" || continue
    z_dir="${z_files[${z_i}]%/*}"
    z_inscription="${z_dir##*/}"
    z_moniker="${z_inscription#"${RBCC_nameplate_sprue}"}"
    z_result="${z_result}${z_result:+ }${z_moniker}"
  done
  test -n "${z_result}" || return 1
  echo "${z_result}"
}

# Armament — a hallmark is "armed" when non-empty. Vacant (empty) is the legal
# marshal-zero cold state, not a defect: the fleet survey reports it and
# consumption (charge) refuses it.
rbrn_hallmark_armed_flint_predicate() {
  test -n "${1:-}" || return 1
  return 0
}

# Armament gate — the homed consumption refusal a charge cites in place of its
# own inline checks. Vacancy is a legal shape; this is the single point where an
# unarmed nameplate is refused at consumption. Half-armed is a legitimate
# transient (rbw-nd drives one field at a time) — each field is gated
# independently, so the message names the vacant one.
rbrn_require_armed() {
  zrbrn_sentinel

  local -r z_moniker="${1:-}"
  local -r z_sentry_hallmark="${2:-}"
  local -r z_bottle_hallmark="${3:-}"

  buc_doc_brief "Refuse a nameplate whose Sentry or Bottle hallmark is vacant"
  buc_doc_param "moniker" "The nameplate moniker the refusal names"
  buc_doc_param "sentry_hallmark" "The nameplate's RBRN_SENTRY_HALLMARK value"
  buc_doc_param "bottle_hallmark" "The nameplate's RBRN_BOTTLE_HALLMARK value"
  buc_doc_shown || return 0

  rbrn_hallmark_armed_flint_predicate "${z_sentry_hallmark}" \
    || buc_die_now "Nameplate '${z_moniker}' is not armed: RBRN_SENTRY_HALLMARK is vacant — kludge or ordain the Sentry vessel and drive its hallmark (rbw-nd), then recharge"
  rbrn_hallmark_armed_flint_predicate "${z_bottle_hallmark}" \
    || buc_die_now "Nameplate '${z_moniker}' is not armed: RBRN_BOTTLE_HALLMARK is vacant — kludge or ordain the Bottle vessel and drive its hallmark (rbw-nd), then recharge"
}

######################################################################
# Cross-Nameplate Functions
#
# rbrn_preflight:  Requires RBCC kindled (called from CLI rbrn_audit)

# Convert dotted-quad IPv4 to integer for subnet arithmetic. Exactly four
# octets, each decimal with no leading zero and at most 255, else return 1:
# a leading zero would read as octal and an octet over 255 would spill into
# its neighbour. Every octet is read in base 10.
zrbrn_ip_int_capture() {
  zrbrn_sentinel

  local -r z_ip="${1:-}"
  local -r z_octet_pattern='(0|[123456789][0123456789]{0,2})'
  local -r z_ip_pattern="^${z_octet_pattern}[.]${z_octet_pattern}[.]${z_octet_pattern}[.]${z_octet_pattern}\$"
  [[ "${z_ip}" =~ ${z_ip_pattern} ]] || return 1

  local -r z_a="${BASH_REMATCH[1]}"
  local -r z_b="${BASH_REMATCH[2]}"
  local -r z_c="${BASH_REMATCH[3]}"
  local -r z_d="${BASH_REMATCH[4]}"
  case "${z_a}" in ''|*[!0123456789]*) return 1 ;; esac
  case "${z_b}" in ''|*[!0123456789]*) return 1 ;; esac
  case "${z_c}" in ''|*[!0123456789]*) return 1 ;; esac
  case "${z_d}" in ''|*[!0123456789]*) return 1 ;; esac
  test "$(( 10#${z_a} ))" -le 255 || return 1
  test "$(( 10#${z_b} ))" -le 255 || return 1
  test "$(( 10#${z_c} ))" -le 255 || return 1
  test "$(( 10#${z_d} ))" -le 255 || return 1

  echo "$(( (10#${z_a} << 24) + (10#${z_b} << 16) + (10#${z_c} << 8) + 10#${z_d} ))"
}

# Validate that an IP falls within a subnet (dies if not)
# Usage: zrbrn_ip_in_subnet LABEL IP BASE MASK
zrbrn_ip_in_subnet() {
  zrbrn_sentinel

  local -r z_label="${1:-}"
  local -r z_ip="${2:-}"
  local -r z_base="${3:-}"
  local -r z_mask="${4:-}"
  case "${z_mask}" in ''|*[!0123456789]*) buc_die_now "Subnet mask ${z_mask} for ${z_label} is not a decimal number" ;; esac

  local z_ip_status=0
  local z_ip_int
  z_ip_int=$(zrbrn_ip_int_capture "${z_ip}") || z_ip_status=$?
  test "${z_ip_status}" -eq 0 \
    || buc_reject "${BUBC_band_regime}" "${z_label}=${z_ip} is not a dotted-quad IPv4 address (four octets 0-255, no leading zeros)"
  case "${z_ip_int}" in ''|*[!0123456789]*) buc_die_now "Address ${z_ip} for ${z_label} did not convert to a decimal integer: ${z_ip_int}" ;; esac

  local z_base_status=0
  local z_base_int
  z_base_int=$(zrbrn_ip_int_capture "${z_base}") || z_base_status=$?
  test "${z_base_status}" -eq 0 \
    || buc_reject "${BUBC_band_regime}" "Subnet base ${z_base} for ${z_label} is not a dotted-quad IPv4 address (four octets 0-255, no leading zeros)"
  case "${z_base_int}" in ''|*[!0123456789]*) buc_die_now "Subnet base ${z_base} for ${z_label} did not convert to a decimal integer: ${z_base_int}" ;; esac

  local -r z_net_mask=$(( (0xFFFFFFFF << (32 - 10#${z_mask})) & 0xFFFFFFFF ))
  test "$(( 10#${z_ip_int} & z_net_mask ))" -eq "$(( 10#${z_base_int} & z_net_mask ))" \
    || buc_reject "${BUBC_band_regime}" "${z_label}=${z_ip} is not within subnet ${z_base}/${z_mask}"
}

# Cross-nameplate conflict validation (silent on success, dies on conflict)
# Checks: port uniqueness, subnet non-overlap, enclave IP uniqueness
rbrn_preflight() {
  zrbrn_sentinel

  buc_doc_brief "Refuse a nameplate set with a port conflict, an enclave IP conflict or overlapping subnets"
  buc_doc_shown || return 0

  # Collect structured data from all nameplates via isolation subshells
  local -r z_nameplate_files=("${RBCC_moorings_dir}/"*"/${RBCC_rbrn_file}")
  local z_data_lines=()
  local z_data_files=()
  local -r z_isolation_program='
        source "$1" || exit 1
        echo "${RBRN_MONIKER}|${RBRN_ENTRY_MODE}|${RBRN_ENTRY_PORT_WORKSTATION}|${RBRN_ENTRY_PORT_ENCLAVE}|${RBRN_ENCLAVE_BASE_IP}|${RBRN_ENCLAVE_NETMASK}|${RBRN_ENCLAVE_SENTRY_IP}|${RBRN_ENCLAVE_BOTTLE_IP}" || exit 1
      '
  local z_nf_i=""
  local z_line=""
  for z_nf_i in "${!z_nameplate_files[@]}"; do
    test -f "${z_nameplate_files[${z_nf_i}]}" || continue
    z_line=$(bash -c "${z_isolation_program}" _ "${z_nameplate_files[${z_nf_i}]}") || buc_die_now "Preflight isolation failed for: ${z_nameplate_files[${z_nf_i}]}"
    z_data_lines+=("${z_line}")
    z_data_files+=("${z_nameplate_files[${z_nf_i}]}")
  done

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

  local -r z_port_pattern='^[123456789][0123456789]{0,4}$'
  local -r z_mask_pattern='^[123456789][0123456789]?$'

  local z_file=""
  local z_mon=""
  local z_entry=""
  local z_ws=""
  local z_enc=""
  local z_base=""
  local z_mask=""
  local z_sentry=""
  local z_bottle=""
  local z_overflow=""
  local z_sentry_int=""
  local z_bottle_int=""
  local z_ip_status=0
  local z_enc_key=""
  local z_i=""
  local z_j=""
  local z_k=""
  local z_net_int=""
  local z_net_mask_bits=0
  local z_net_addr=0
  local z_net_size=0
  local z_net_end=0
  for z_nf_i in "${!z_data_lines[@]}"; do
    z_file="${z_data_files[${z_nf_i}]}"
    z_overflow=""
    IFS='|' read -r z_mon z_entry z_ws z_enc z_base z_mask z_sentry z_bottle z_overflow <<< "${z_data_lines[${z_nf_i}]}" \
      || buc_die_now "Failed to parse nameplate data line from: ${z_file}"

    # Every field checked before use: the line crossed a fresh interpreter, so
    # nothing here has met the regime's enrollment validation.
    test -z "${z_overflow}" \
      || buc_die_now "Nameplate ${z_file} yields more than eight fields — a value carries '|'"
    test -n "${z_mon}" \
      || buc_die_now "Nameplate ${z_file} has an empty RBRN_MONIKER"
    case "${z_entry}" in
      rbnne_enabled|rbnne_disabled) ;;
      *) buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_MODE '${z_entry}' — expected rbnne_enabled or rbnne_disabled" ;;
    esac
    if test "${z_entry}" = "rbnne_enabled"; then
      [[ "${z_ws}" =~ ${z_port_pattern} ]] \
        || buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_WORKSTATION '${z_ws}' — expected a port 1-65535"
      case "${z_ws}" in ''|*[!0123456789]*) buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_WORKSTATION '${z_ws}' — expected a port 1-65535" ;; esac
      test "$(( 10#${z_ws} ))" -le 65535 \
        || buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_WORKSTATION '${z_ws}' — expected a port 1-65535"
      [[ "${z_enc}" =~ ${z_port_pattern} ]] \
        || buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_ENCLAVE '${z_enc}' — expected a port 1-65535"
      case "${z_enc}" in ''|*[!0123456789]*) buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_ENCLAVE '${z_enc}' — expected a port 1-65535" ;; esac
      test "$(( 10#${z_enc} ))" -le 65535 \
        || buc_die_now "Nameplate ${z_mon} has RBRN_ENTRY_PORT_ENCLAVE '${z_enc}' — expected a port 1-65535"
    fi
    [[ "${z_mask}" =~ ${z_mask_pattern} ]] \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_NETMASK '${z_mask}' — expected 8-30"
    case "${z_mask}" in ''|*[!0123456789]*) buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_NETMASK '${z_mask}' — expected 8-30" ;; esac
    test "$(( 10#${z_mask} ))" -ge 8 \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_NETMASK '${z_mask}' — expected 8-30"
    test "$(( 10#${z_mask} ))" -le 30 \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_NETMASK '${z_mask}' — expected 8-30"
    z_ip_status=0
    z_net_int=$(zrbrn_ip_int_capture "${z_base}") || z_ip_status=$?
    test "${z_ip_status}" -eq 0 \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_BASE_IP '${z_base}' — expected a dotted-quad IPv4 address (four octets 0-255, no leading zeros)"
    case "${z_net_int}" in ''|*[!0123456789]*) buc_die_now "Nameplate ${z_mon} RBRN_ENCLAVE_BASE_IP '${z_base}' did not convert to a decimal integer: ${z_net_int}" ;; esac
    z_ip_status=0
    z_sentry_int=$(zrbrn_ip_int_capture "${z_sentry}") || z_ip_status=$?
    test "${z_ip_status}" -eq 0 \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_SENTRY_IP '${z_sentry}' — expected a dotted-quad IPv4 address (four octets 0-255, no leading zeros)"
    z_ip_status=0
    z_bottle_int=$(zrbrn_ip_int_capture "${z_bottle}") || z_ip_status=$?
    test "${z_ip_status}" -eq 0 \
      || buc_die_now "Nameplate ${z_mon} has RBRN_ENCLAVE_BOTTLE_IP '${z_bottle}' — expected a dotted-quad IPv4 address (four octets 0-255, no leading zeros)"

    # Workstation and enclave port uniqueness (enabled entries only)
    if test "${z_entry}" = "rbnne_enabled"; then
      for z_i in "${!z_ws_port_keys[@]}"; do
        if test "${z_ws_port_keys[${z_i}]}" = "${z_ws}"; then
          buc_die_now "Port conflict: RBRN_ENTRY_PORT_WORKSTATION=${z_ws} claimed by both ${z_ws_port_vals[${z_i}]} and ${z_mon}"
        fi
      done
      z_ws_port_keys+=("${z_ws}")
      z_ws_port_vals+=("${z_mon}")

      # Enclave ports are scoped to their Docker network — key on base_ip:port
      z_enc_key="${z_base}:${z_enc}"
      for z_i in "${!z_enc_port_keys[@]}"; do
        if test "${z_enc_port_keys[${z_i}]}" = "${z_enc_key}"; then
          buc_die_now "Port conflict: RBRN_ENTRY_PORT_ENCLAVE=${z_enc} on network ${z_base} claimed by both ${z_enc_port_vals[${z_i}]} and ${z_mon}"
        fi
      done
      z_enc_port_keys+=("${z_enc_key}")
      z_enc_port_vals+=("${z_mon}")
    fi

    # Enclave IP uniqueness (all sentry and bottle IPs across nameplates),
    # keyed on the integer so one address has one key
    for z_j in "${!z_ip_keys[@]}"; do
      if test "${z_ip_keys[${z_j}]}" = "${z_sentry_int}"; then
        buc_die_now "IP conflict: ${z_sentry} claimed by ${z_mon} (sentry) and ${z_ip_vals[${z_j}]}"
      fi
    done
    z_ip_keys+=("${z_sentry_int}")
    z_ip_vals+=("${z_mon}:sentry")

    for z_j in "${!z_ip_keys[@]}"; do
      if test "${z_ip_keys[${z_j}]}" = "${z_bottle_int}"; then
        buc_die_now "IP conflict: ${z_bottle} claimed by ${z_mon} (bottle) and ${z_ip_vals[${z_j}]}"
      fi
    done
    z_ip_keys+=("${z_bottle_int}")
    z_ip_vals+=("${z_mon}:bottle")

    # Subnet non-overlap
    z_net_mask_bits=$(( (0xFFFFFFFF << (32 - 10#${z_mask})) & 0xFFFFFFFF ))
    z_net_addr=$(( 10#${z_net_int} & z_net_mask_bits ))
    z_net_size=$(( 1 << (32 - 10#${z_mask}) ))
    z_net_end=$(( z_net_addr + z_net_size - 1 ))

    for z_k in "${!z_net_starts[@]}"; do
      if test "${z_net_addr}" -le "${z_net_ends[${z_k}]}" && test "${z_net_starts[${z_k}]}" -le "${z_net_end}"; then
        buc_die_now "Subnet overlap: ${z_base}/${z_mask} (${z_mon}) overlaps with network of ${z_net_owners[${z_k}]}"
      fi
    done
    z_net_starts+=("${z_net_addr}")
    z_net_ends+=("${z_net_end}")
    z_net_owners+=("${z_mon}")

  done
}

######################################################################
# Drive (rbrn_drive)
#
# Resolve a hallmark express-or-chain and write it into one target nameplate's
# RBRN_BOTTLE_HALLMARK or RBRN_SENTRY_HALLMARK line. The durable-config LINK for
# the nameplate/hallmark chain — sibling of feoff/anoint/yoke on the rbsbc_enchase
# surface, but writing the rbrn_regime family rather than rbrv_regime. Called by
# rbrn_cli (the cloud operator verb) and by rbob (the local kludge composes it).
# Operator-committed, never self-committing.

rbrn_drive() {
  zrbrn_sentinel

  local -r z_nameplate="${BUZ_FOLIO:-}"
  local -r z_field="${1:-}"
  local -r z_express="${2:-}"

  buc_doc_brief "Drive a freshly-built hallmark into a nameplate — resolve the hallmark express-or-chain and rewrite RBRN_BOTTLE_HALLMARK or RBRN_SENTRY_HALLMARK in the nameplate's rbrn.env"
  buc_doc_param "nameplate" "Target nameplate moniker"
  buc_doc_param "field"     "Which hallmark to drive: 'bottle' or 'sentry'"
  buc_doc_param "hallmark"  "Hallmark tag (e.g., k260327172456); optional — absent, falls back to the hallmark a build handed forward through the depth-1 chain"
  buc_doc_shown || return 0

  # Relay-then-read (RBr_3e7): forward the chain baton before any read or failure point.
  buf_relay || buc_die_now "Failed to relay chained facts"

  test -n "${z_nameplate}" || buc_die_now "Nameplate required (param1)"

  # Map the operator-facing field selector to the RBRN variable. A two-value
  # ashlar selector (bottle|sentry), never the raw variable name.
  local z_var_name=""
  case "${z_field}" in
    bottle) z_var_name="RBRN_BOTTLE_HALLMARK" ;;
    sentry) z_var_name="RBRN_SENTRY_HALLMARK" ;;
    "")     buc_die_now "Field required (param2): 'bottle' or 'sentry'" ;;
    *)      buc_die_now "Unknown field '${z_field}' — expected 'bottle' or 'sentry'" ;;
  esac

  # Resolve the target nameplate's rbrn.env by moniker WITHOUT loading it. The
  # drive rewrites one line; loading the regime would enforce the field's
  # min-length and reject a still-blank hallmark — the very state the drive fills.
  # Mirrors feoff, which never loads the vessel whose rbrv.env it rewrites.
  local -r z_rbrn_file="${RBCC_moorings_dir}/${RBCC_nameplate_sprue}${z_nameplate}/${RBCC_rbrn_file}"
  test -f "${z_rbrn_file}" || buc_die_now "Nameplate regime file not found: ${z_rbrn_file}"

  # Resolve the hallmark express-or-chain: an express argument wins; absent, the
  # value a build (kludge or ordain) handed forward through the depth-1 chain.
  # No clean-tree gate here (RBr_a52).
  local z_hallmark=""
  local z_hallmark_status=0
  z_hallmark=$(buf_elect_fact_capture "${z_express}" "${RBF_FACT_HALLMARK}") || z_hallmark_status=$?
  test "${z_hallmark_status}" -eq 0 \
    || buc_reject "${BUBC_band_chain}" "No hallmark — pass one (param3) or run a build (kludge or ordain) immediately before drive"
  local z_source="chain"
  test -z "${z_express}" || z_source="express"

  buc_step "Driving ${z_var_name} into ${z_nameplate} (source ${z_source})"

  # Replace-or-die the chosen field's line. RBRN_{BOTTLE,SENTRY}_HALLMARK is a
  # required enrolled field always present in a well-formed nameplate — a missing
  # line is schema drift, a hard die (the find-or-err schema-drift catch), never a
  # silent append (contrast anoint, whose RBRV_GRAFT_IMAGE may legitimately be
  # absent and is appended).
  local -r z_tmp_file="${ZRBRN_DRIVE_PREFIX}${z_nameplate}_${RBCC_rbrn_file}.new"
  local z_line=""
  local z_found=false
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" == "${z_var_name}"=* ]]; then
      printf '%s\n' "${z_var_name}=${z_hallmark}" || buc_die_now "Failed to write ${z_tmp_file}"
      z_found=true
    else
      printf '%s\n' "${z_line}" || buc_die_now "Failed to write ${z_tmp_file}"
    fi
  done < "${z_rbrn_file}" > "${z_tmp_file}" \
    || buc_die_now "Failed to rewrite ${z_rbrn_file} for ${z_var_name}"
  test "${z_found}" = "true" \
    || buc_die_now "Field ${z_var_name} not found in ${z_rbrn_file} — nameplate schema drift"
  mv "${z_tmp_file}" "${z_rbrn_file}" || buc_die_now "Failed to finalize ${z_rbrn_file}"

  # Loud on success: the driven field, value, and source named prominently, so a
  # wrong drive shows at the moment of action rather than only in the git diff.
  buc_success "Drove ${z_nameplate}: ${z_var_name}=${z_hallmark} (source: ${z_source})"
  buc_info "Commit the rbrn.env change with your usual git workflow."
}

# eof
