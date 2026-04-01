#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Derived Constants - Credential file path resolution

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBDC_SOURCED:-}" || buc_die "Module rbdc multiply sourced - check sourcing hierarchy"
ZRBDC_SOURCED=1

######################################################################
# Internal Functions (zrbdc_*)

zrbdc_kindle() {
  test -z "${ZRBDC_KINDLED:-}" || buc_die "Module rbdc already kindled"
  zrbrr_sentinel

  # Ensure secrets directory and role subdirectories exist
  mkdir -p "${RBRR_SECRETS_DIR}/${RBCC_role_governor}" \
           "${RBRR_SECRETS_DIR}/${RBCC_role_retriever}" \
           "${RBRR_SECRETS_DIR}/${RBCC_role_director}" \
    || buc_die "Failed to create secrets directories under: ${RBRR_SECRETS_DIR}"

  # One-shot migration: move old flat rbra-{role}.env to {role}/rbra.env
  local z_mig_role=""
  for z_mig_role in "${RBCC_role_governor}" "${RBCC_role_retriever}" "${RBCC_role_director}"; do
    local z_mig_old="${RBRR_SECRETS_DIR}/rbra-${z_mig_role}.env"
    local z_mig_new="${RBRR_SECRETS_DIR}/${z_mig_role}/${RBCC_rbra_file}"
    if test -f "${z_mig_old}" && ! test -f "${z_mig_new}"; then
      mv "${z_mig_old}" "${z_mig_new}" || buc_die "Failed to migrate: ${z_mig_old} → ${z_mig_new}"
      local z_has_role=0
      local z_mig_line
      while IFS= read -r z_mig_line; do
        case "${z_mig_line}" in RBRA_ROLE=*) z_has_role=1; break ;; esac
      done < "${z_mig_new}"
      if test "${z_has_role}" = "0"; then
        printf 'RBRA_ROLE=%s\n' "${z_mig_role}" >> "${z_mig_new}"
      fi
    fi
  done

  # Derive credential file paths from RBRR_SECRETS_DIR
  readonly RBDC_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_governor}/${RBCC_rbra_file}"
  readonly RBDC_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_retriever}/${RBCC_rbra_file}"
  readonly RBDC_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_director}/${RBCC_rbra_file}"
  readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"

  # Derive full pool resource paths from stem (suffixes match RBGC_POOL_SUFFIX_TETHER/AIRGAP)
  readonly RBDC_POOL_TETHER="projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBRR_GCP_REGION}/workerPools/${RBRR_GCB_POOL_STEM}-tether"
  readonly RBDC_POOL_AIRGAP="projects/${RBRR_DEPOT_PROJECT_ID}/locations/${RBRR_GCP_REGION}/workerPools/${RBRR_GCB_POOL_STEM}-airgap"

  readonly ZRBDC_KINDLED=1
}

zrbdc_sentinel() {
  test "${ZRBDC_KINDLED:-}" = "1" || buc_die "Module rbdc not kindled - call zrbdc_kindle first"
}

# eof
