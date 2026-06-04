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
# Requires: rbrd_regime.sh sourced

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
  readonly ZRBOB_SENTRY="${ZRBOB_PROJECT}-${RBCC_container_sentry}"
  readonly ZRBOB_PENTACLE="${ZRBOB_PROJECT}-${RBCC_container_pentacle}"
  readonly ZRBOB_BOTTLE="${ZRBOB_PROJECT}-${RBCC_container_bottle}"

  # Network name (compose names as {project}_{network}; used by observe and info)
  readonly ZRBOB_NETWORK="${ZRBOB_PROJECT}_enclave"

  # Base compose file and the compose-quoting probe are kit machinery, not
  # consumer config — they live with the kit (RBCC_KIT_DIR), not in moorings.
  readonly ZRBOB_COMPOSE_BASE="${RBCC_KIT_DIR}/rbob_compose.yml"
  test -f "${ZRBOB_COMPOSE_BASE}" || buc_die "Base compose file not found: ${ZRBOB_COMPOSE_BASE}"

  readonly ZRBOB_COMPOSE_FRAGMENT="${RBCC_moorings_dir}/${RBRN_MONIKER}/rbnnh_compose.yml"
  # Fragment is optional — existence checked at compose invocation time

  readonly ZRBOB_POST_CHARGE_HOOK="${RBCC_moorings_dir}/${RBRN_MONIKER}/rbnnh_post_charge.sh"
  # Hook is optional — existence + executable bit checked at charge tail

  # Env file paths (for compose --env-file: YAML interpolation + container env forwarding)
  readonly ZRBOB_ENV_RBRR="${RBCC_rbrr_file}"
  readonly ZRBOB_ENV_RBRD="${RBCC_rbrd_file}"
  readonly ZRBOB_ENV_RBJE="${RBCC_KIT_DIR}/rbje_compose_probe.env"
  readonly ZRBOB_ENV_RBRN="${RBCC_moorings_dir}/${RBRN_MONIKER}/${RBCC_rbrn_file}"

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

# Internal: normalize a path argument for the Windows-native compose CLI under
# Cygwin. docker compose opens -f / --env-file / --project-directory as Windows
# paths, so a /cygdrive/X/... absolute is misread ("does not exist") exactly as
# docker build/cp were; hand it the drive-letter form (X:/... — forward slashes,
# which Windows accepts). DRAFT — third copy of zrbfc_native_path_capture
# (rbfcb_BuildHost.sh); the scattered per-module copies are slated to
# consolidate into one Windows-docker path adapter (heat ₣BV). Pure /cygdrive
# parameter expansion, gated on BURD_OSTYPE; identity off Cygwin; a bare-absolute
# POSIX path is unsurveyed and returns 1. Kindle-independent.
zrbob_native_path_capture() {
  local -r z_path="${1:?zrbob_native_path_capture: path required}"

  if test "${BURD_OSTYPE:-}" != "cygwin"; then
    printf '%s\n' "${z_path}"
    return 0
  fi

  case "${z_path}" in
    /cygdrive/?/*)
      local -r z_drive_rest="${z_path#/cygdrive/}"
      local -r z_drive="${z_drive_rest%%/*}"
      local -r z_drive_tail="${z_drive_rest#"${z_drive}/"}"
      printf '%s\n' "${z_drive}:/${z_drive_tail}"
      ;;
    /*)
      return 1
      ;;
    *)
      printf '%s\n' "${z_path}"
      ;;
  esac
}

# Build and execute a compose command with standard args
# Usage: zrbob_compose <compose_subcommand> [args...]
zrbob_compose() {
  zrbob_sentinel

  # Under Cygwin the runtime is Windows-native docker; the compose CLI opens
  # -f / --env-file / --project-directory as Windows paths, so a /cygdrive
  # absolute is misread ("does not exist") exactly as docker build/cp were.
  # Normalize every path arg to drive-letter form first; off Cygwin each
  # capture is identity. Pinning --project-directory to the native form
  # additionally makes compose resolve the fragment's relative volume mounts to
  # Windows-native paths — what Docker Desktop's WSL2 backend needs to bind-mount.
  local z_native_projdir=""
  z_native_projdir=$(zrbob_native_path_capture "${RBCC_moorings_dir}") \
    || buc_die "Cannot normalize compose --project-directory: ${RBCC_moorings_dir}"
  local z_native_rbrr=""
  z_native_rbrr=$(zrbob_native_path_capture "${ZRBOB_ENV_RBRR}") \
    || buc_die "Cannot normalize compose --env-file (RBRR): ${ZRBOB_ENV_RBRR}"
  local z_native_rbrd=""
  z_native_rbrd=$(zrbob_native_path_capture "${ZRBOB_ENV_RBRD}") \
    || buc_die "Cannot normalize compose --env-file (RBRD): ${ZRBOB_ENV_RBRD}"
  local z_native_rbdc=""
  z_native_rbdc=$(zrbob_native_path_capture "${ZRBOB_ENV_RBDC}") \
    || buc_die "Cannot normalize compose --env-file (RBDC): ${ZRBOB_ENV_RBDC}"
  local z_native_rbje=""
  z_native_rbje=$(zrbob_native_path_capture "${ZRBOB_ENV_RBJE}") \
    || buc_die "Cannot normalize compose --env-file (RBJE): ${ZRBOB_ENV_RBJE}"
  local z_native_rbrn=""
  z_native_rbrn=$(zrbob_native_path_capture "${ZRBOB_ENV_RBRN}") \
    || buc_die "Cannot normalize compose --env-file (RBRN): ${ZRBOB_ENV_RBRN}"
  local z_native_base=""
  z_native_base=$(zrbob_native_path_capture "${ZRBOB_COMPOSE_BASE}") \
    || buc_die "Cannot normalize compose -f base: ${ZRBOB_COMPOSE_BASE}"

  local z_args=()
  z_args+=("compose")
  # Pin the project directory to the moorings (consumer-config) root. Compose
  # otherwise derives it from the first -f file's location — and the base
  # compose now lives with the kit (Tools/rbk), not in moorings. Nameplate
  # fragment (rbnnh_compose.yml) relative paths — env_file, volume mounts —
  # are authored relative to this root, so it must be set explicitly. CLI
  # --env-file paths resolve against CWD (repo root) and are unaffected.
  z_args+=("--project-directory" "${z_native_projdir}")
  z_args+=("--env-file" "${z_native_rbrr}")
  z_args+=("--env-file" "${z_native_rbrd}")
  z_args+=("--env-file" "${z_native_rbdc}")
  z_args+=("--env-file" "${z_native_rbje}")
  z_args+=("--env-file" "${z_native_rbrn}")
  z_args+=("-f" "${z_native_base}")

  # Include nameplate fragment if it exists (existence checked on the POSIX
  # path; the normalized native form is what compose receives)
  if test -f "${ZRBOB_COMPOSE_FRAGMENT}"; then
    local z_native_fragment=""
    z_native_fragment=$(zrbob_native_path_capture "${ZRBOB_COMPOSE_FRAGMENT}") \
      || buc_die "Cannot normalize compose -f fragment: ${ZRBOB_COMPOSE_FRAGMENT}"
    z_args+=("-f" "${z_native_fragment}")
  fi

  z_args+=("-p" "${ZRBOB_PROJECT}")
  z_args+=("$@")

  buc_log_args "${ZRBOB_RUNTIME} ${z_args[*]}"
  "${ZRBOB_RUNTIME}" "${z_args[@]}"
}

######################################################################
# Auto-Summon Helpers

# Discriminate kludge hallmarks (local-only, no GAR presence) from conjure /
# bind / graft hallmarks. Used by charge preflight to redirect the diagnostic
# away from the summon path, which is guaranteed to fail for kludge stamps.
zrbob_hallmark_is_kludge() {
  case "${1}" in
    "${RBGC_HALLMARK_PREFIX_KLUDGE}"*) return 0 ;;
    *) return 1 ;;
  esac
}

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

  rbgo_docker_login "${z_token}" "${z_registry_host}"

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

  rbgo_docker_login "${z_token}" "${z_registry_host}"

  docker pull "${z_image_ref}" || buc_die "Failed to pull image: ${z_image_ref}"

  buc_info "Auto-summoned: ${z_image_ref}"
}

######################################################################
# Charge Invariant — Subnet-Keyed Reclaim

# At most one crucible per nameplate per system. A crucible's subnet derives
# from the nameplate (RBRN_ENCLAVE_BASE_IP), not from the project prefix, so
# two same-nameplate crucibles can never coexist on one daemon. Free the
# nameplate's subnet by tearing down whatever network occupies it — regardless
# of prefix or how the network was created (compose or manual `docker network
# create`). Match on the subnet, a structural fact, not on inferred name
# lineage. Our own compose network is excluded; compose-down already handled it.
zrbob_reclaim_subnet() {
  zrbob_sentinel

  local -r z_target_subnet="${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}"
  local -r z_networks_file="${BURD_TEMP_DIR}/zrbob_reclaim_networks.txt"
  local -r z_networks_stderr="${BURD_TEMP_DIR}/zrbob_reclaim_networks_stderr.txt"
  local -r z_inspect_prefix="${BURD_TEMP_DIR}/zrbob_reclaim_inspect_"
  local -r z_containers_prefix="${BURD_TEMP_DIR}/zrbob_reclaim_containers_"
  local -r z_crm_prefix="${BURD_TEMP_DIR}/zrbob_reclaim_crm_"
  local -r z_nrm_prefix="${BURD_TEMP_DIR}/zrbob_reclaim_nrm_"

  ${ZRBOB_RUNTIME} network ls --format '{{.Name}}' \
    > "${z_networks_file}" 2>"${z_networks_stderr}" \
    || buc_die "Failed to list Docker networks — see ${z_networks_stderr}"

  # Load network names (load-then-iterate per BCG; names are captured before any
  # reclaim, so removing networks mid-loop cannot disturb the iteration)
  local z_names=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_names+=("${z_line}")
  done < "${z_networks_file}"

  local z_index=0
  local z_name=""
  local z_subnet_file=""
  local z_subnet_stderr=""
  local z_subnet=""
  local z_containers_file=""
  local z_containers_stderr=""
  local z_crm_file=""
  local z_nrm_file=""

  if (( ${#z_names[@]} )); then
    for z_name in "${z_names[@]}"; do
      z_subnet_file="${z_inspect_prefix}${z_index}.txt"
      z_subnet_stderr="${z_inspect_prefix}${z_index}_stderr.txt"
      z_containers_file="${z_containers_prefix}${z_index}.txt"
      z_containers_stderr="${z_containers_prefix}${z_index}_stderr.txt"
      z_crm_file="${z_crm_prefix}${z_index}.txt"
      z_nrm_file="${z_nrm_prefix}${z_index}.txt"
      z_index=$((z_index + 1))

      # Skip our own compose network — compose down already handled it
      test "${z_name}" != "${ZRBOB_NETWORK}" || continue

      # A network in the list may vanish or lack IPAM config between ls and
      # inspect; tolerate by skipping (stderr preserved for forensics)
      ${ZRBOB_RUNTIME} network inspect "${z_name}" \
        --format '{{range .IPAM.Config}}{{.Subnet}}{{end}}' \
        > "${z_subnet_file}" 2>"${z_subnet_stderr}" \
        || continue

      z_subnet=$(<"${z_subnet_file}")
      test "${z_subnet}" = "${z_target_subnet}" || continue

      buc_warn "Subnet ${z_target_subnet} occupied by stale network '${z_name}' — reclaiming"

      ${ZRBOB_RUNTIME} ps -aq --filter "network=${z_name}" \
        > "${z_containers_file}" 2>"${z_containers_stderr}" \
        || buc_die "Failed to list containers on '${z_name}' — see ${z_containers_stderr}"

      local z_containers=()
      local z_cline=""
      while IFS= read -r z_cline || test -n "${z_cline}"; do
        test -n "${z_cline}" || continue
        z_containers+=("${z_cline}")
      done < "${z_containers_file}"

      # Force-remove attached containers first (a mid-run-killed crucible may
      # leave them running). Non-fatal: a container may already be exiting; the
      # network rm below is the real gate — if removal truly failed there, that
      # rm fails and dies.
      if (( ${#z_containers[@]} )); then
        ${ZRBOB_RUNTIME} rm -f "${z_containers[@]}" \
          > "${z_crm_file}" 2>&1 \
          || buc_warn "Some containers on '${z_name}' resisted removal — see ${z_crm_file}"
      fi

      ${ZRBOB_RUNTIME} network rm "${z_name}" \
        > "${z_nrm_file}" 2>&1 \
        || buc_die "Failed to reclaim subnet ${z_target_subnet}: could not remove network '${z_name}' — see ${z_nrm_file}"

      buc_info "Reclaimed subnet ${z_target_subnet} from '${z_name}'"
    done
  fi
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
    if zrbob_hallmark_is_kludge "${RBRN_SENTRY_HALLMARK}"; then
      buc_die "Kludge sentry hallmark not built locally: ${RBRN_SENTRY_HALLMARK}"
    fi
    buc_warn "Sentry vouch artifact missing locally: ${ZRBOB_SENTRY_VOUCH}"
    zrbob_summon_full_hallmark "${RBRN_SENTRY_HALLMARK}"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_VOUCH}" >/dev/null 2>&1; then
    if zrbob_hallmark_is_kludge "${RBRN_BOTTLE_HALLMARK}"; then
      buc_die "Kludge bottle hallmark not built locally: ${RBRN_BOTTLE_HALLMARK}"
    fi
    buc_warn "Bottle vouch artifact missing locally: ${ZRBOB_BOTTLE_VOUCH}"
    zrbob_summon_full_hallmark "${RBRN_BOTTLE_HALLMARK}"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_SENTRY_IMAGE}" >/dev/null 2>&1; then
    if zrbob_hallmark_is_kludge "${RBRN_SENTRY_HALLMARK}"; then
      buc_die "Kludge sentry hallmark not built locally: ${RBRN_SENTRY_HALLMARK}"
    fi
    buc_warn "Sentry image not found locally: ${ZRBOB_SENTRY_IMAGE}"
    zrbob_vouch_gate_and_summon "${RBRN_SENTRY_VESSEL}" "${RBRN_SENTRY_HALLMARK}" "${ZRBOB_SENTRY_IMAGE}"
  fi

  if ! ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_IMAGE}" >/dev/null 2>&1; then
    if zrbob_hallmark_is_kludge "${RBRN_BOTTLE_HALLMARK}"; then
      buc_die "Kludge bottle hallmark not built locally: ${RBRN_BOTTLE_HALLMARK}"
    fi
    buc_warn "Bottle image not found locally: ${ZRBOB_BOTTLE_IMAGE}"
    zrbob_vouch_gate_and_summon "${RBRN_BOTTLE_VESSEL}" "${RBRN_BOTTLE_HALLMARK}" "${ZRBOB_BOTTLE_IMAGE}"
  fi

  # Tear down any prior state (tolerates missing project)
  buc_step "Cleaning up any prior state (quenching any preexisting can take a couple of minutes)"
  zrbob_compose --profile sessile down --remove-orphans 2>/dev/null || true

  # Charge invariant: at most one crucible per nameplate per system. A
  # crucible's subnet derives from the nameplate, not the prefix, so freeing
  # the nameplate's subnet — tearing down whatever network occupies it,
  # regardless of prefix or creation method — is sufficient and subsumes any
  # name-lineage collision check.
  zrbob_reclaim_subnet

  # Start services via compose (--profile sessile includes bottle)
  # --wait blocks until all health checks pass (sentry → pentacle → bottle chain)
  buc_step "Starting services via compose"
  zrbob_compose --profile sessile up -d --wait

  buc_step "Crucible started: ${RBRN_MONIKER}"

  if test "${RBRN_BOTTLE_READINESS_DELAY_SEC}" -gt 0; then
    buc_step "Waiting ${RBRN_BOTTLE_READINESS_DELAY_SEC}s for bottle service readiness"
    sleep "${RBRN_BOTTLE_READINESS_DELAY_SEC}"
  fi

  if test -x "${ZRBOB_POST_CHARGE_HOOK}"; then
    buc_step "Running post-charge hook: ${ZRBOB_POST_CHARGE_HOOK}"
    "${ZRBOB_POST_CHARGE_HOOK}" || buc_die "Post-charge hook failed: ${ZRBOB_POST_CHARGE_HOOK}"
  fi
}

# Check whether the crucible is charged — sentry, pentacle, and bottle must each
# be individually `running`. BCG predicate: returns 0 if charged, 1 if not.
# Never dies, no output. Compose stderr captured to BURD_TEMP_DIR for operator
# inspection on verify-active failures.
rbob_charged_predicate() {
  zrbob_sentinel

  local z_service=""
  local z_ids_file=""
  local z_stderr_file=""

  for z_service in "${RBCC_container_sentry}" "${RBCC_container_pentacle}" "${RBCC_container_bottle}"; do
    z_ids_file="${BURD_TEMP_DIR}/zrbob_charged_${z_service}_ids.txt"
    z_stderr_file="${BURD_TEMP_DIR}/zrbob_charged_${z_service}_stderr.txt"

    "${ZRBOB_RUNTIME}" compose -p "${ZRBOB_PROJECT}" ps "${z_service}" --status running -q \
      > "${z_ids_file}" 2>"${z_stderr_file}" \
      || return 1

    test -s "${z_ids_file}" || return 1
  done

  return 0
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
  # for the duration of the call (rbfk_kludge reads BUZ_FOLIO as its operand).
  BUZ_FOLIO="${RBRN_BOTTLE_VESSEL}" rbfk_kludge

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

  BUZ_FOLIO="${RBRN_SENTRY_VESSEL}" rbfk_kludge

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
