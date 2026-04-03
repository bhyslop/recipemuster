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
# RBOB - Recipe Bottle Orchestration Bottle
# Container lifecycle management via Docker Compose
#
# Requires: buc_command.sh sourced
# Requires: rbrn_regime.sh sourced
# Requires: rbrr_regime.sh sourced

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBOB_SOURCED:-}" || buc_die "Module rbob multiply sourced - check sourcing hierarchy"
ZRBOB_SOURCED=1


######################################################################
# Compose-env Validation

# Validate that compose-consumed RBRN fields contain no quotes or ${VAR} references.
# Compose --env-file does not strip quotes (they become part of the value) and
# interpolates ${VAR} references (causing unintended expansion). Fields not consumed
# by compose (e.g., RBRN_DESCRIPTION) are exempt.
zrbob_validate_compose_env() {
  # Explicit list of compose-consumed field names (the nameplate↔compose contract)
  local z_compose_fields="
    RBRN_MONIKER
    RBRN_RUNTIME
    RBRN_SENTRY_VESSEL
    RBRN_BOTTLE_VESSEL
    RBRN_SENTRY_HALLMARK
    RBRN_BOTTLE_HALLMARK
    RBRN_ENTRY_MODE
    RBRN_ENTRY_PORT_WORKSTATION
    RBRN_ENTRY_PORT_ENCLAVE
    RBRN_ENCLAVE_BASE_IP
    RBRN_ENCLAVE_NETMASK
    RBRN_ENCLAVE_SENTRY_IP
    RBRN_ENCLAVE_BOTTLE_IP
    RBRN_UPLINK_PORT_MIN
    RBRN_UPLINK_DNS_MODE
    RBRN_UPLINK_ACCESS_MODE
    RBRN_UPLINK_ALLOWED_CIDRS
    RBRN_UPLINK_ALLOWED_DOMAINS
  "

  local z_field=""
  for z_field in ${z_compose_fields}; do
    local z_value="${!z_field:-}"
    # Check for quotes (compose does not strip them)
    case "${z_value}" in
      *\"*|*\'*)
        buc_die "Compose compatibility: ${z_field} contains quotes — compose --env-file does not strip quotes, they become part of the value"
        ;;
    esac
    # Check for ${VAR} references (compose interpolates them)
    case "${z_value}" in
      *'${'*)
        buc_die "Compose compatibility: ${z_field} contains \${VAR} reference — compose --env-file interpolates these, causing unintended expansion"
        ;;
    esac
  done
}

######################################################################
# Kindle and Sentinel

zrbob_kindle() {
  test -z "${ZRBOB_KINDLED:-}" || buc_die "Module rbob already kindled"

  # Verify RBRN regime is kindled (provides nameplate config)
  zrbrn_sentinel

  # Validate compose-consumed fields before proceeding
  zrbob_validate_compose_env

  # Verify RBRR regime is kindled (provides repo config like DNS_SERVER)
  zrbrr_sentinel

  # Runtime command (docker or podman)
  case "${RBRN_RUNTIME}" in
    docker) readonly ZRBOB_RUNTIME="docker" ;;
    podman) readonly ZRBOB_RUNTIME="podman" ;;
    *) buc_die "Unknown RBRN_RUNTIME: ${RBRN_RUNTIME}" ;;
  esac

  # Container names (for connect and info commands)
  readonly ZRBOB_SENTRY="${RBRN_MONIKER}-sentry"
  readonly ZRBOB_PENTACLE="${RBRN_MONIKER}-pentacle"
  readonly ZRBOB_BOTTLE="${RBRN_MONIKER}-bottle"

  # Network name (compose names as {project}_{network}; used by observe and info)
  readonly ZRBOB_NETWORK="${RBRN_MONIKER}_enclave"

  # Compose file paths (relative to project root, where compose runs)
  readonly ZRBOB_COMPOSE_BASE="${RBBC_dot_dir}/rbob_compose.yml"
  test -f "${ZRBOB_COMPOSE_BASE}" || buc_die "Base compose file not found: ${ZRBOB_COMPOSE_BASE}"

  readonly ZRBOB_COMPOSE_FRAGMENT="${RBBC_dot_dir}/${RBRN_MONIKER}/compose.yml"
  # Fragment is optional — existence checked at compose invocation time

  # Env file paths (for compose --env-file: YAML interpolation + container env forwarding)
  readonly ZRBOB_ENV_RBRR="${RBBC_dot_dir}/rbrr.env"
  readonly ZRBOB_ENV_RBJE="${RBBC_dot_dir}/rbje_compose_probe.env"
  readonly ZRBOB_ENV_RBRN="${RBBC_dot_dir}/${RBRN_MONIKER}/${RBCC_rbrn_file}"


  # GAR image references (computed once, used by preflight and auto-summon)
  local z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  readonly ZRBOB_SENTRY_IMAGE="${z_gar_base}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}${RBGC_ARK_SUFFIX_IMAGE}"
  readonly ZRBOB_BOTTLE_IMAGE="${z_gar_base}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_HALLMARK}${RBGC_ARK_SUFFIX_IMAGE}"

  # GAR vouch references (local presence verified on every start)
  readonly ZRBOB_SENTRY_VOUCH="${z_gar_base}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}${RBGC_ARK_SUFFIX_VOUCH}"
  readonly ZRBOB_BOTTLE_VOUCH="${z_gar_base}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_HALLMARK}${RBGC_ARK_SUFFIX_VOUCH}"

  # Export RBRN/RBRR vars for compose environment: bare name forwarding.
  # Compose --env-file populates the compose environment for both YAML interpolation
  # and bare name forwarding; explicit exports provide defense-in-depth.
  export RBRN_ENCLAVE_BASE_IP
  export RBRN_ENCLAVE_NETMASK
  export RBRN_ENCLAVE_SENTRY_IP
  export RBRN_ENCLAVE_BOTTLE_IP
  export RBRN_ENTRY_MODE
  export RBRN_ENTRY_PORT_WORKSTATION
  export RBRN_ENTRY_PORT_ENCLAVE
  export RBRN_UPLINK_DNS_MODE
  export RBRN_UPLINK_PORT_MIN
  export RBRN_UPLINK_ACCESS_MODE
  export RBRN_UPLINK_ALLOWED_CIDRS
  export RBRN_UPLINK_ALLOWED_DOMAINS
  export RBRR_DNS_SERVER
  export RBRR_BOTTLE_WORKSPACE

  readonly ZRBOB_DRIVE_PREFIX="${BURD_TEMP_DIR}/rbob_drive_"

  readonly ZRBOB_KINDLED=1
}

zrbob_sentinel() {
  test "${ZRBOB_KINDLED:-}" = "1" || buc_die "Module rbob not kindled - call zrbob_kindle first"
}

######################################################################
# Compose Command Helper

# Build and execute a compose command with standard args
# Usage: zrbob_compose <compose_subcommand> [args...]
zrbob_compose() {
  zrbob_sentinel

  local z_args=()
  z_args+=("compose")
  z_args+=("--env-file" "${ZRBOB_ENV_RBRR}")
  z_args+=("--env-file" "${ZRBOB_ENV_RBJE}")
  z_args+=("--env-file" "${ZRBOB_ENV_RBRN}")
  z_args+=("-f" "${ZRBOB_COMPOSE_BASE}")

  # Include nameplate fragment if it exists
  if test -f "${ZRBOB_COMPOSE_FRAGMENT}"; then
    z_args+=("-f" "${ZRBOB_COMPOSE_FRAGMENT}")
  fi

  z_args+=("-p" "${RBRN_MONIKER}")
  z_args+=("$@")

  buc_log_args "${ZRBOB_RUNTIME} ${z_args[*]}"
  "${ZRBOB_RUNTIME}" "${z_args[@]}"
}

######################################################################
# Auto-Summon Helper

# Verify vouch exists and pull image if missing locally
# Usage: zrbob_vouch_gate_and_summon <vessel> <hallmark> <image_ref>
zrbob_vouch_gate_and_summon() {
  local z_vessel="${1:-}"
  local z_hallmark="${2:-}"
  local z_image_ref="${3:-}"

  test -n "${z_vessel}"       || buc_die "zrbob_vouch_gate_and_summon: vessel required"
  test -n "${z_hallmark}" || buc_die "zrbob_vouch_gate_and_summon: hallmark required"
  test -n "${z_image_ref}"    || buc_die "zrbob_vouch_gate_and_summon: image_ref required"

  local z_registry_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"

  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token for auto-summon"

  # Pull the image
  buc_step "Auto-summoning ${z_image_ref}"

  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${z_registry_host}" \
    || buc_die "Container registry authentication failed during auto-summon"

  docker pull "${z_image_ref}" || buc_die "Failed to pull image: ${z_image_ref}"

  buc_info "Auto-summoned: ${z_image_ref}"
}

######################################################################
# Subnet Conflict Detection

# Die if another Docker network already occupies this nameplate's subnet.
# This catches stale networks from prior compose projects that died without
# compose-down (reboot, Docker Desktop restart, crash). Our own network
# (from the compose-down above) is excluded.
zrbob_detect_subnet_conflict() {
  zrbob_sentinel

  local z_target_subnet="${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}"
  local z_networks_file="${BURD_TEMP_DIR}/zrbob_detect_networks.txt"
  local z_subnet_file="${BURD_TEMP_DIR}/zrbob_detect_subnet.txt"

  # List all Docker network names
  ${ZRBOB_RUNTIME} network ls --format '{{.Name}}' > "${z_networks_file}" \
    || buc_die "Failed to list Docker networks"

  # Load network names (load-then-iterate per BCG)
  local z_names=()
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_names+=("${z_line}")
  done < "${z_networks_file}"

  # Inspect each network's subnet
  local z_name=""
  for z_name in "${z_names[@]}"; do
    # Skip our own compose network — compose down already handled it
    test "${z_name}" != "${ZRBOB_NETWORK}" || continue

    ${ZRBOB_RUNTIME} network inspect "${z_name}" \
      --format '{{range .IPAM.Config}}{{.Subnet}}{{end}}' \
      > "${z_subnet_file}" 2>/dev/null || continue

    local z_subnet=""
    z_subnet=$(<"${z_subnet_file}")

    if test "${z_subnet}" = "${z_target_subnet}"; then
      buc_die "Subnet ${z_target_subnet} already claimed by stale network '${z_name}' — clean up manually: docker rm \$(docker ps -aq --filter network=${z_name}) 2>/dev/null; docker network rm ${z_name}"
    fi
  done
}

######################################################################
# Public API

# Start the crucible (sentry + pentacle + bottle) via compose
# Requires: RBOB kindled (which requires RBRN and RBRR)
rbob_charge() {
  zrbob_sentinel

  buc_step "Starting crucible: ${RBRN_MONIKER}"

  # Gate: nameplate must have no uncommitted changes
  if ! git diff --quiet -- "${ZRBOB_ENV_RBRN}" 2>/dev/null; then
    buc_die "Nameplate has uncommitted changes: ${ZRBOB_ENV_RBRN} — commit before charging"
  fi

  # Cross-nameplate validation (silent on success, dies on conflict)
  rbrn_preflight

  # Preflight: verify vouch artifacts exist locally — no network calls, fatal if missing
  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_SENTRY_VOUCH}" >/dev/null 2>&1; then
    buc_warn "Sentry vouch artifact missing locally: ${ZRBOB_SENTRY_VOUCH}"
    buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_SENTRY_VESSEL}" "${RBRN_SENTRY_HALLMARK}"
    buc_die "Run summon to pull vouch artifact before starting"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_VOUCH}" >/dev/null 2>&1; then
    buc_warn "Bottle vouch artifact missing locally: ${ZRBOB_BOTTLE_VOUCH}"
    buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_BOTTLE_VESSEL}" "${RBRN_BOTTLE_HALLMARK}"
    buc_die "Run summon to pull vouch artifact before starting"
  fi

  # Preflight: verify container images exist locally, auto-summon if missing
  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_SENTRY_IMAGE}" >/dev/null 2>&1; then
    buc_warn "Sentry image not found locally: ${ZRBOB_SENTRY_IMAGE}"
    zrbob_vouch_gate_and_summon "${RBRN_SENTRY_VESSEL}" "${RBRN_SENTRY_HALLMARK}" "${ZRBOB_SENTRY_IMAGE}"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_IMAGE}" >/dev/null 2>&1; then
    buc_warn "Bottle image not found locally: ${ZRBOB_BOTTLE_IMAGE}"
    zrbob_vouch_gate_and_summon "${RBRN_BOTTLE_VESSEL}" "${RBRN_BOTTLE_HALLMARK}" "${ZRBOB_BOTTLE_IMAGE}"
  fi

  # Tear down any prior state (tolerates missing project)
  buc_step "Cleaning up any prior state"
  zrbob_compose --profile sessile down --remove-orphans 2>/dev/null || true

  # Defensive check: another compose project may hold a network on our subnet
  # (e.g., nsproto containers died without compose-down, blocking tadmor's subnet)
  zrbob_detect_subnet_conflict

  # Start services via compose (--profile sessile includes bottle)
  buc_step "Starting services via compose"
  zrbob_compose --profile sessile up -d

  buc_step "Crucible started: ${RBRN_MONIKER}"
}

# Check whether the crucible is charged (compose project has running containers).
# BCG predicate: returns 0 if charged, 1 if not. Never dies, no output.
rbob_charged_predicate() {
  zrbob_sentinel
  "${ZRBOB_RUNTIME}" compose -p "${RBRN_MONIKER}" ps -q --status running 2>/dev/null | grep -q .
}

# Stop the crucible via compose
rbob_quench() {
  zrbob_sentinel

  buc_step "Stopping crucible: ${RBRN_MONIKER}"

  zrbob_compose --profile sessile down --remove-orphans

  buc_step "Crucible stopped: ${RBRN_MONIKER}"
}

# Hail sentry — call out to the guard (interactive shell)
rbob_hail() {
  zrbob_sentinel
  buc_step "Hailing sentry: ${ZRBOB_SENTRY}"
  exec ${ZRBOB_RUNTIME} exec -it "${ZRBOB_SENTRY}" /bin/bash
}

# Rack bottle — compel the demon to reveal state (interactive shell)
rbob_rack() {
  zrbob_sentinel
  buc_step "Racking bottle: ${ZRBOB_BOTTLE}"
  exec ${ZRBOB_RUNTIME} exec -it "${ZRBOB_BOTTLE}" /bin/bash
}

# Writ sentry — non-interactive command execution in sentry
rbob_writ() {
  zrbob_sentinel
  buc_step "Writ to sentry: ${ZRBOB_SENTRY}"
  exec ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" "$@"
}

# Fiat pentacle — non-interactive command execution in pentacle
rbob_fiat() {
  zrbob_sentinel
  buc_step "Fiat to pentacle: ${ZRBOB_PENTACLE}"
  exec ${ZRBOB_RUNTIME} exec "${ZRBOB_PENTACLE}" "$@"
}

# Bark bottle — non-interactive command execution in bottle
rbob_bark() {
  zrbob_sentinel
  buc_step "Bark to bottle: ${ZRBOB_BOTTLE}"
  exec ${ZRBOB_RUNTIME} exec "${ZRBOB_BOTTLE}" "$@"
}

# Launch Claude Code client inside bottle (ifrit engagement)
rbob_ifrit_client() {
  zrbob_sentinel
  buc_step "Launching Claude Code client in bottle: ${ZRBOB_BOTTLE}"
  exec ${ZRBOB_RUNTIME} exec -it -w "${RBRR_BOTTLE_WORKSPACE}" "${ZRBOB_BOTTLE}" claude
}

# Run ifrit sortie adjutant inside bottle (security test dispatch)
rbob_ifrit_sortie() {
  zrbob_sentinel
  buc_step "Running ifrit sortie adjutant in bottle: ${ZRBOB_BOTTLE}"
  ${ZRBOB_RUNTIME} exec -w "${RBRR_BOTTLE_WORKSPACE}" "${ZRBOB_BOTTLE}" \
    python3 rbtia_adjutant.py "$@"
}

######################################################################
# Drive Hallmark — rewrite a single RBRN_*_HALLMARK line in nameplate env
# Args: nameplate_file variable_name new_hallmark

zrbob_drive_hallmark() {
  local -r z_file="$1"
  local -r z_var_name="$2"
  local -r z_new_value="$3"

  test -f "${z_file}" || buc_die "Nameplate file not found: ${z_file}"
  test -n "${z_var_name}" || buc_die "Variable name required"
  test -n "${z_new_value}" || buc_die "New hallmark value required"

  # Load file into array (BCG load-then-iterate)
  local z_lines=()
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${z_file}"

  # Rewrite with substitution
  local -r z_temp="${ZRBOB_DRIVE_PREFIX}${z_var_name}"
  : > "${z_temp}" || buc_die "Failed to create temp file: ${z_temp}"

  local z_found=0
  local z_i="" z_current=""
  for z_i in "${!z_lines[@]}"; do
    z_current="${z_lines[$z_i]}"
    case "${z_current}" in
      "${z_var_name}"=*)
        z_current="${z_var_name}=${z_new_value}"
        z_found=1
        ;;
    esac
    printf '%s\n' "${z_current}" >> "${z_temp}" || buc_die "Failed to write line"
  done

  test "${z_found}" = "1" || buc_die "Variable ${z_var_name} not found in ${z_file}"

  # Atomic replace
  mv "${z_temp}" "${z_file}" || buc_die "Failed to replace nameplate file: ${z_file}"

  buc_info "Drove ${z_var_name}=${z_new_value} → ${z_file}"
}

######################################################################
# Kludge Bottle — param1 dispatch target delegating to rbob_kludge

rbob_kludge_bottle() {
  zrbob_sentinel
  rbob_kludge
}

######################################################################
# Kludge — build bottle vessel locally and drive hallmark into nameplate

rbob_kludge() {
  zrbob_sentinel
  zrbfc_sentinel

  buc_doc_brief "Build bottle vessel locally and drive hallmark into nameplate"
  buc_doc_shown || return 0

  buc_step "Kludge: ${RBRN_MONIKER} (${RBRN_BOTTLE_VESSEL})"

  # Resolve vessel from nameplate config
  local -r z_vessel_dir="${RBRR_VESSEL_DIR}/${RBRN_BOTTLE_VESSEL}"
  test -d "${z_vessel_dir}" || buc_die "Bottle vessel directory not found: ${z_vessel_dir}"

  # Delegate build to foundry kludge
  rbfd_kludge "${z_vessel_dir}"

  # Read hallmark from fact file
  local z_hallmark=""
  z_hallmark=$(<"${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}") \
    || buc_die "Failed to read hallmark from kludge output"
  test -n "${z_hallmark}" || buc_die "Empty hallmark from kludge output"

  # Drive hallmark into nameplate
  zrbob_drive_hallmark "${ZRBOB_ENV_RBRN}" "RBRN_BOTTLE_HALLMARK" "${z_hallmark}"

  buc_success "Kludge installed: ${z_hallmark} → ${RBRN_MONIKER}"
}

######################################################################
# Ordain — cloud-build bottle vessel and drive hallmark into nameplate

rbob_ordain() {
  zrbob_sentinel
  zrbfd_sentinel

  buc_doc_brief "Ordain bottle vessel via cloud build and drive hallmark into nameplate"
  buc_doc_shown || return 0

  buc_step "Ordain: ${RBRN_MONIKER} (${RBRN_BOTTLE_VESSEL})"

  # Resolve vessel from nameplate config
  local -r z_vessel_dir="${RBRR_VESSEL_DIR}/${RBRN_BOTTLE_VESSEL}"
  test -d "${z_vessel_dir}" || buc_die "Bottle vessel directory not found: ${z_vessel_dir}"

  # Delegate to foundry ordain (mode-dispatched: conjure, bind, or graft)
  rbfd_ordain "${z_vessel_dir}"

  # Read hallmark from fact file
  local z_hallmark=""
  z_hallmark=$(<"${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}") \
    || buc_die "Failed to read hallmark from ordain output"
  test -n "${z_hallmark}" || buc_die "Empty hallmark from ordain output"

  # Drive hallmark into nameplate
  zrbob_drive_hallmark "${ZRBOB_ENV_RBRN}" "RBRN_BOTTLE_HALLMARK" "${z_hallmark}"

  buc_success "Ordain installed: ${z_hallmark} → ${RBRN_MONIKER}"
}

# eof
