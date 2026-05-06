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

  # Compose project identity for this nameplate. The runtime-prefixed moniker
  # is the authoritative project name compose labels containers and networks
  # under; every -p invocation and every container/network name derives from it.
  readonly ZRBOB_PROJECT="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}"

  # Container names (for connect and info commands)
  readonly ZRBOB_SENTRY="${ZRBOB_PROJECT}-sentry"
  readonly ZRBOB_PENTACLE="${ZRBOB_PROJECT}-pentacle"
  readonly ZRBOB_BOTTLE="${ZRBOB_PROJECT}-bottle"

  # Network name (compose names as {project}_{network}; used by observe and info)
  readonly ZRBOB_NETWORK="${ZRBOB_PROJECT}_enclave"

  # Compose file paths (relative to project root, where compose runs)
  readonly ZRBOB_COMPOSE_BASE="${RBBC_dot_dir}/rbob_compose.yml"
  test -f "${ZRBOB_COMPOSE_BASE}" || buc_die "Base compose file not found: ${ZRBOB_COMPOSE_BASE}"

  readonly ZRBOB_COMPOSE_FRAGMENT="${RBBC_dot_dir}/${RBRN_MONIKER}/compose.yml"
  # Fragment is optional — existence checked at compose invocation time

  # Env file paths (for compose --env-file: YAML interpolation + container env forwarding)
  readonly ZRBOB_ENV_RBRR="${RBBC_dot_dir}/rbrr.env"
  readonly ZRBOB_ENV_RBJE="${RBBC_dot_dir}/rbje_compose_probe.env"
  readonly ZRBOB_ENV_RBRN="${RBBC_dot_dir}/${RBRN_MONIKER}/${RBCC_rbrn_file}"

  # RBDC env file — RBDC_* are bash-kindle constants invisible to compose
  # without an env-file bridge. Write the subset compose interpolates into
  # a temp file and feed it via --env-file.
  local z_rbdc_env="${BURD_TEMP_DIR}/rbob_rbdc_compose.env"
  printf 'RBDC_DEPOT_PROJECT_ID=%s\nRBDC_GAR_REPOSITORY=%s\n' \
    "${RBDC_DEPOT_PROJECT_ID}" "${RBDC_GAR_REPOSITORY}" > "${z_rbdc_env}" \
    || buc_die "Failed to write RBDC compose env file: ${z_rbdc_env}"
  readonly ZRBOB_ENV_RBDC="${z_rbdc_env}"


  # GAR image references (computed once, used by preflight and auto-summon)
  local z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  readonly ZRBOB_SENTRY_IMAGE="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_SENTRY_HALLMARK}/${RBGC_ARK_BASENAME_IMAGE}:${RBRN_SENTRY_HALLMARK}"
  readonly ZRBOB_BOTTLE_IMAGE="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_BOTTLE_HALLMARK}/${RBGC_ARK_BASENAME_IMAGE}:${RBRN_BOTTLE_HALLMARK}"

  # GAR vouch references (local presence verified on every start)
  readonly ZRBOB_SENTRY_VOUCH="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_SENTRY_HALLMARK}/${RBGC_ARK_BASENAME_VOUCH}:${RBRN_SENTRY_HALLMARK}"
  readonly ZRBOB_BOTTLE_VOUCH="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${RBRN_BOTTLE_HALLMARK}/${RBGC_ARK_BASENAME_VOUCH}:${RBRN_BOTTLE_HALLMARK}"

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

  # Host UID/GID for bind-mount ownership alignment.
  # The person running charge IS the host identity — no configuration needed.
  local z_uid_file="${BURD_TEMP_DIR}/rbob_host_uid"
  local z_gid_file="${BURD_TEMP_DIR}/rbob_host_gid"
  id -u > "${z_uid_file}" || buc_die "Failed to determine host UID"
  id -g > "${z_gid_file}" || buc_die "Failed to determine host GID"
  local z_host_uid=""
  z_host_uid=$(<"${z_uid_file}")
  local z_host_gid=""
  z_host_gid=$(<"${z_gid_file}")
  export RBOB_HOST_UID="${z_host_uid}"
  export RBOB_HOST_GID="${z_host_gid}"

  # Load bottle vessel user for compose and SSH (optional — empty means image default)
  local z_bottle_rbrv="${RBRR_VESSEL_DIR}/${RBRN_BOTTLE_VESSEL}/rbrv.env"
  local z_bottle_user=""
  if test -f "${z_bottle_rbrv}"; then
    z_bottle_user=$(grep '^RBRV_USER=' "${z_bottle_rbrv}" | head -1 | cut -d= -f2) || true
  fi
  readonly ZRBOB_BOTTLE_USER="${z_bottle_user}"
  export RBRV_USER="${z_bottle_user}"

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
  z_args+=("--env-file" "${ZRBOB_ENV_RBDC}")
  z_args+=("--env-file" "${ZRBOB_ENV_RBJE}")
  z_args+=("--env-file" "${ZRBOB_ENV_RBRN}")
  z_args+=("-f" "${ZRBOB_COMPOSE_BASE}")

  # Include nameplate fragment if it exists
  if test -f "${ZRBOB_COMPOSE_FRAGMENT}"; then
    z_args+=("-f" "${ZRBOB_COMPOSE_FRAGMENT}")
  fi

  z_args+=("-p" "${ZRBOB_PROJECT}")
  z_args+=("$@")

  buc_log_args "${ZRBOB_RUNTIME} ${z_args[*]}"
  "${ZRBOB_RUNTIME}" "${z_args[@]}"
}

######################################################################
# Auto-Summon Helpers

# Pull all three arks (image, about, vouch) for a hallmark to local runtime.
# Used when charge detects a missing local artifact and needs to bootstrap.
# Mirrors rbfr_summon's all-three-arks semantics; called inline rather than
# as a tabtarget invocation because rbob is the sole caller and avoids the
# launcher-dispatch round-trip.
zrbob_summon_full_hallmark() {
  local z_hallmark="${1:-}"

  test -n "${z_hallmark}" || buc_die "zrbob_summon_full_hallmark: hallmark required"

  local z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local z_image_ref="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_IMAGE}:${z_hallmark}"
  local z_about_ref="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_ABOUT}:${z_hallmark}"
  local z_vouch_ref="${z_gar_base}/${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${RBGC_ARK_BASENAME_VOUCH}:${z_hallmark}"

  local z_registry_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"

  local z_token
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token for auto-summon"

  buc_step "Auto-summoning hallmark ${z_hallmark} (image + about + vouch)"

  echo "${z_token}" | docker login -u oauth2accesstoken --password-stdin "https://${z_registry_host}" \
    || buc_die "Container registry authentication failed during auto-summon"

  docker pull "${z_image_ref}" || buc_die "Failed to pull image ark: ${z_image_ref}"
  docker pull "${z_about_ref}" || buc_die "Failed to pull about ark: ${z_about_ref}"
  docker pull "${z_vouch_ref}" || buc_die "Failed to pull vouch ark: ${z_vouch_ref}"

  buc_info "Auto-summoned: ${z_hallmark}"
}

# Pull a single image ref to local runtime — used by the image-only auto-summon
# branch in rbob_charge as defense-in-depth for partial-state cases (vouch
# present but image missing). The full-hallmark helper above covers the common
# path where all arks are missing together.
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
# Charge Invariant — Network Collision Detection

# Die if any compose-managed network with our nameplate exists from a foreign
# project. Catches pre-prefix-era residue (project=tadmor without RBRR
# prefix) and any other compose project whose name is the bare moniker or
# ends with "-{moniker}". Match is by labels — independent of how compose
# joined project + network into the network name.
zrbob_detect_nameplate_collision() {
  zrbob_sentinel

  local -r z_networks_file="${BURD_TEMP_DIR}/zrbob_collision_networks.txt"
  local -r z_networks_stderr="${BURD_TEMP_DIR}/zrbob_collision_networks_stderr.txt"
  local -r z_inspect_prefix="${BURD_TEMP_DIR}/zrbob_collision_inspect_"

  ${ZRBOB_RUNTIME} network ls \
    --filter 'label=com.docker.compose.project' \
    --filter 'label=com.docker.compose.network' \
    --format '{{.Name}}' \
    > "${z_networks_file}" 2>"${z_networks_stderr}" \
    || buc_die "Failed to list Docker networks for collision detection — see ${z_networks_stderr}"

  local z_names=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_names+=("${z_line}")
  done < "${z_networks_file}"

  local z_offenders=()
  local z_index=0
  local z_name=""
  local z_inspect_file=""
  local z_inspect_stderr=""
  local z_labels=""
  local z_net_label=""
  local z_proj_label=""

  if (( ${#z_names[@]} )); then
    for z_name in "${z_names[@]}"; do
      z_inspect_file="${z_inspect_prefix}${z_index}.txt"
      z_inspect_stderr="${z_inspect_prefix}${z_index}_stderr.txt"
      z_index=$((z_index + 1))

      ${ZRBOB_RUNTIME} network inspect "${z_name}" \
        --format '{{index .Labels "com.docker.compose.network"}}|{{index .Labels "com.docker.compose.project"}}' \
        > "${z_inspect_file}" 2>"${z_inspect_stderr}" \
        || continue

      z_labels=$(<"${z_inspect_file}")
      z_net_label="${z_labels%%|*}"
      z_proj_label="${z_labels#*|}"

      # Network must be enclave or transit
      test "${z_net_label}" = "enclave" || test "${z_net_label}" = "transit" \
        || continue

      # Skip our own current project; compose-down already removed it (defensive)
      test "${z_proj_label}" != "${ZRBOB_PROJECT}" || continue

      # Match: bare moniker, or any prefix ending in "-{moniker}"
      if test "${z_proj_label}" = "${RBRN_MONIKER}"; then
        z_offenders+=("${z_name}")
      elif test "${z_proj_label}" != "${z_proj_label%-"${RBRN_MONIKER}"}"; then
        z_offenders+=("${z_name}")
      fi
    done
  fi

  test "${#z_offenders[@]}" -gt 0 || return 0

  local z_msg="Nameplate '${RBRN_MONIKER}' collision: ${#z_offenders[@]} foreign compose network(s) match — clean up before charging:"
  local z_offender=""
  for z_offender in "${z_offenders[@]}"; do
    z_msg="${z_msg}"$'\n'"  docker rm \$(docker ps -aq --filter network=${z_offender}) 2>/dev/null; docker network rm ${z_offender}"
  done
  buc_die "${z_msg}"
}

# Die if another Docker network already occupies this nameplate's subnet.
# Catches non-compose networks (manual `docker network create`) that the
# label-based scan above can't see. Our own network is excluded.
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

  # Preflight: ensure all hallmark arks exist locally. Vouch presence is the
  # hallmark-level signal — if vouch is missing, the full hallmark is treated
  # as not-yet-summoned and all three arks are pulled together. Image-only
  # auto-summon below is defense-in-depth for partial-state cases.
  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_SENTRY_VOUCH}" >/dev/null 2>&1; then
    buc_warn "Sentry vouch artifact missing locally: ${ZRBOB_SENTRY_VOUCH}"
    zrbob_summon_full_hallmark "${RBRN_SENTRY_HALLMARK}"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_VOUCH}" >/dev/null 2>&1; then
    buc_warn "Bottle vouch artifact missing locally: ${ZRBOB_BOTTLE_VOUCH}"
    zrbob_summon_full_hallmark "${RBRN_BOTTLE_HALLMARK}"
  fi

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

  # Charge invariant: at most one crucible per nameplate per system. The
  # collision scan refuses on any foreign compose project whose network
  # bears our moniker (catches pre-prefix-era residue independent of the
  # current run's project name); the subnet scan catches non-compose
  # networks that bypass label-based detection.
  zrbob_detect_nameplate_collision
  zrbob_detect_subnet_conflict

  # Start services via compose (--profile sessile includes bottle)
  # --wait blocks until all health checks pass (sentry → pentacle → bottle chain)
  buc_step "Starting services via compose"
  zrbob_compose --profile sessile up -d --wait

  buc_step "Crucible started: ${RBRN_MONIKER}"
}

# Check whether the crucible is charged (compose project has running containers).
# BCG predicate: returns 0 if charged, 1 if not. Never dies, no output.
rbob_charged_predicate() {
  zrbob_sentinel
  "${ZRBOB_RUNTIME}" compose -p "${ZRBOB_PROJECT}" ps -q --status running 2>/dev/null | grep -q .
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

# SSH into bottle — proper terminal session via sentry DNAT
rbob_ssh() {
  zrbob_sentinel
  local z_ssh_user="${ZRBOB_BOTTLE_USER:-root}"
  buc_step "SSH to bottle via port ${RBRN_ENTRY_PORT_WORKSTATION} as ${z_ssh_user}"
  exec ssh -t -p "${RBRN_ENTRY_PORT_WORKSTATION}" \
    -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
    "${z_ssh_user}@localhost"
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

  # Delegate build to foundry kludge — override BUZ_FOLIO with the vessel sigil
  # for the duration of the call (rbfd_kludge reads BUZ_FOLIO as its operand).
  BUZ_FOLIO="${RBRN_BOTTLE_VESSEL}" rbfd_kludge

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
# Kludge Sentry — build sentry vessel locally and drive hallmark into nameplate

rbob_kludge_sentry() {
  zrbob_sentinel
  zrbfc_sentinel

  buc_doc_brief "Build sentry vessel locally and drive hallmark into nameplate"
  buc_doc_shown || return 0

  buc_step "Kludge: ${RBRN_MONIKER} (${RBRN_SENTRY_VESSEL})"

  BUZ_FOLIO="${RBRN_SENTRY_VESSEL}" rbfd_kludge

  # Read hallmark from fact file
  local z_hallmark=""
  z_hallmark=$(<"${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}") \
    || buc_die "Failed to read hallmark from kludge output"
  test -n "${z_hallmark}" || buc_die "Empty hallmark from kludge output"

  # Drive hallmark into nameplate
  zrbob_drive_hallmark "${ZRBOB_ENV_RBRN}" "RBRN_SENTRY_HALLMARK" "${z_hallmark}"

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

  BUZ_FOLIO="${RBRN_BOTTLE_VESSEL}" rbfd_ordain

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
