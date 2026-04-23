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
# Recipe Bottle Handbook Onboarding - Base (kindle, sentinel, probes, shared helpers)

set -euo pipefail

test -z "${ZRBHO_SOURCED:-}" || buc_die "Module rbho multiply sourced - check sourcing hierarchy"
ZRBHO_SOURCED=1

# rbho walkthroughs do not require rbho-local state (they probe the
# filesystem directly and render via buh_*). The kindle exists to assert
# the dependency ordering is correct at furnish time.

zrbho_kindle() {
  test -z "${ZRBHO_KINDLED:-}" || buc_die "Module rbho already kindled"
  zrbgc_sentinel
  zbuz_sentinel
  zrbz_sentinel

  # BCG stderr-capture prefixes for docker probes — discriminator appended at use site.
  # BURD_TEMP_DIR is dispatcher-provided (rbho is thin furnish — does not kindle burd).
  readonly ZRBHO_DOCKER_IMAGES_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_images_"
  readonly ZRBHO_DOCKER_PS_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_ps_"
  readonly ZRBHO_DOCKER_STDERR_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_stderr_"

  readonly ZRBHO_KINDLED=1
}

zrbho_sentinel() {
  test "${ZRBHO_KINDLED:-}" = "1" || buc_die "Module rbho not kindled - call zrbho_kindle first"
}

# Probe utilities — no sentinels, all work pre-kindle. Filesystem probes
# for onboarding status; callers declare caller-scope variables locally.

zrbho_po_status() {
  local -r z_flag="${1:-}"
  local -r z_text="${2:-}"
  if test "${z_flag}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${z_text}"
  else
    buh_line "${RBYC_PROBE_NO}${z_text}"
  fi
}

# Extract a KEY=VALUE from a file; stdout empty if missing.  No sourcing.
zrbho_po_extract_capture() {
  local -r z_file="${1:-}"
  local -r z_key="${2:-}"
  test -n "${z_key}"  || return 1
  test -f "${z_file}" || return 1
  local z_line=""
  while IFS= read -r z_line; do
    case "${z_line}" in "${z_key}="*) echo "${z_line#"${z_key}="}"; return 0 ;; esac
  done < "${z_file}"
  return 1
}

zrbho_credential_install() {
  local -r z_role_constant="${1}"

  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_cred_present=0
  if test -n "${z_secrets_dir}" \
     && test -f "${z_secrets_dir}/${z_role_constant}/${RBCC_rbra_file}"; then
    z_cred_present=1
  fi

  buh_step_style "Step " " — "

  buh_step1 "Get the key file"
  buh_e
  buh_line "A ${RBYC_GOVERNOR} produces ${RBYC_RBRA} credential files for ${RBYC_DIRECTORS} and ${RBYC_RETRIEVERS}."
  buh_line "Your ${RBYC_GOVERNOR} hands you this file out-of-band — it is a"
  buh_line "secret, never committed to the repo."
  buh_e

  buh_step1 "Install the key file"
  buh_e
  if test -n "${z_secrets_dir}"; then
    buh_line "Place the file at the path derived from ${RBYC_RBRR}:"
    buh_e
    buh_code "   ${z_secrets_dir}/${z_role_constant}/${RBCC_rbra_file}"
    buh_e
    buh_line "Create the directory if it does not exist."
  else
    buh_warn "RBRR not populated — cannot determine credential path."
    buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "BURC" "Configure your Repo's Environment"; local -r z_env_link="${z_buym_yelp}"
    buh_line "Run ${z_env_link} first."
  fi
  buh_e
  if test "${z_cred_present}" = "1"; then
    zrbho_po_status 1 "Credential file present"
  else
    zrbho_po_status 0 "Credential file not found"
  fi
  buh_e

  buh_step1 "Validate"
  buh_e
  buh_line "Run the ${RBYC_RBRA} validator for your role:"
  buh_e
  buh_tt   "   " "${RBZ_VALIDATE_AUTH}" "" " ${z_role_constant}"
  buh_e
  buh_line "Read the output — it checks the file format and reports"
  buh_line "what the credential grants."
  buh_e

  # Callers append role-specific verification and closing steps
}

# eof
