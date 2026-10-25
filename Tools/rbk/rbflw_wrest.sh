#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Foundry Ledger - wrest cluster (guard-free, sourced by rbflk_):
# pull an image from the registry to the local container runtime by locator
# (Director credentials).

set -euo pipefail

######################################################################
# Wrest (rbflw_*)

rbflw_wrest() {
  zrbfl0_sentinel

  local -r z_locator="${BUZ_FOLIO:-}"

  buc_doc_brief "Wrest an image from the registry to local container runtime by raw ref (type-blind)"
  buc_doc_param "ref" "Image ref: package:tag or package@sha256:<hex>. Type-blind over any rbi_* namespace (e.g. rbi_hm/H/image:H, rbi_ld/b260602120000:rbi_bole)"
  buc_doc_shown || return 0

  test -n "${z_locator}" || buc_die_now "Image ref required (package:tag or package@sha256:<hex>)"

  case "${z_locator}" in
    *:*) : ;;
    *)   buc_die_now "Invalid image ref. Expected package:tag or package@sha256:<hex>" ;;
  esac
  local -r z_pkg_path="${z_locator%:*}"
  local -r z_tag="${z_locator##*:}"
  test -n "${z_pkg_path}" || buc_die_now "Package path is empty in locator"
  test -n "${z_tag}"      || buc_die_now "Tag is empty in locator"

  buc_step "Authenticating as Director"
  local zSECRET_token
  zSECRET_token=$(rba_token_capture "${RBCC_mantle_director}") \
    || buc_die_now "Failed to get OAuth token"

  buc_step "Logging into container registry"
  local -r z_full_ref="${ZRBFC0_REGISTRY_HOST}/${ZRBFC0_REGISTRY_PATH}/${z_locator}"

  rbgo_docker_login "${zSECRET_token}" "${ZRBFC0_REGISTRY_HOST}"

  buc_step "Pulling image: ${z_full_ref}"
  local z_pull_status=0
  docker pull "${z_full_ref}" || z_pull_status=$?
  local z_logout_status=0
  rbgo_docker_logout_predicate "${ZRBFC0_REGISTRY_HOST}" || z_logout_status=$?
  test "${z_pull_status}" -eq 0 || buc_die_now "Image pull failed (status ${z_pull_status})"
  test "${z_logout_status}" -eq 0 || buc_die_now "Docker logout from ${ZRBFC0_REGISTRY_HOST} failed — the credential may still stand in docker's store"

  local z_image_id
  local -r z_inspect_stderr="${ZRBFL0_INSPECT_PREFIX}wrest_stderr.txt"
  docker inspect --format='{{.Id}}' "${z_full_ref}" > "${ZRBFC0_SCRATCH_FILE}" 2>"${z_inspect_stderr}" \
    || buc_die_now "Failed to get image ID — see ${z_inspect_stderr}"
  z_image_id=$(<"${ZRBFC0_SCRATCH_FILE}") || buc_die_now "Failed to read: ${ZRBFC0_SCRATCH_FILE}"

  echo "" || buc_die_now
  echo "Image wrested: ${z_full_ref}" || buc_die_now
  echo "Local image ID: ${z_image_id}" || buc_die_now

  buc_success "Image wrest complete"
}

# eof
