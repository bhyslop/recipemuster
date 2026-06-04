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
# Recipe Bottle Foundry Ledger - inscribe cluster (guard-free, sourced by rbflk_):
# mirror all tool images from upstream into a datestamped GAR reliquary namespace
# via Cloud Build (Director credentials).

set -euo pipefail

######################################################################
# Inscribe (zrbfl_* / rbfl_*)

# Internal: Submit inscribe Cloud Build job.
# Single step: docker pull each upstream tool image, tag for GAR, push.
# Uses gcr.io/cloud-builders/docker as step image (always pullable — Google-hosted).
zrbfl_inscribe_submit() {
  zrbfl_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_reliquary="${2:?Reliquary datestamp required}"

  buc_step "Constructing inscribe Cloud Build resource"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  # Inscribe step image: Google-hosted docker builder (always pullable, even under NO_PUBLIC_EGRESS)
  local -r z_step_image="gcr.io/cloud-builders/docker"

  # Assemble inscribe step from script
  local -r z_script_path="${ZRBFL_RBGJI_STEPS_DIR}/rbgji01-inscribe-mirror.sh"
  test -f "${z_script_path}" || buc_die "Inscribe step script not found: ${z_script_path}"

  local -r z_body_file="${ZRBFL_RELIQUARY_PREFIX}body.txt"
  local -r z_escaped_file="${ZRBFL_RELIQUARY_PREFIX}escaped.txt"

  buc_log_args "Reading inscribe step script (skip shebang)"
  zrbfc_write_script_body "${z_script_path}" "${z_body_file}" \
    || buc_die "Failed to read inscribe step script"
  local z_body=""
  z_body=$(<"${z_body_file}")
  test -n "${z_body}" || buc_die "Empty inscribe script body"

  printf '#!/bin/bash\n%s' "${z_body}" > "${z_escaped_file}" \
    || buc_die "Failed to write escaped inscribe script body"

  local -r z_step_file="${ZRBFL_RELIQUARY_PREFIX}step.json"
  echo "[]" > "${z_step_file}" || buc_die "Failed to initialize inscribe step JSON"

  local -r z_step_built="${ZRBFL_RELIQUARY_PREFIX}step_built.json"
  jq \
    --arg name "${z_step_image}" \
    --arg id "inscribe-mirror" \
    --rawfile script "${z_escaped_file}" \
    '. + [{name: $name, id: $id, script: $script}]' \
    "${z_step_file}" > "${z_step_built}" \
    || buc_die "Failed to build inscribe step JSON"
  mv "${z_step_built}" "${z_step_file}" \
    || buc_die "Failed to finalize inscribe step JSON"

  # Compose Build resource JSON
  buc_log_args "Composing inscribe Build resource JSON"
  local -r z_build_file="${ZRBFL_RELIQUARY_PREFIX}build.json"

  jq -n \
    --slurpfile zjq_steps  "${z_step_file}" \
    --arg zjq_sa           "${z_mason_sa}" \
    --arg zjq_gar_host         "${z_gar_host}" \
    --arg zjq_gar_path         "${z_gar_path}" \
    --arg zjq_reliquaries_root "${RBGL_RELIQUARIES_ROOT}" \
    --arg zjq_reliquary        "${z_reliquary}" \
    --arg zjq_pool             "${RBDC_POOL_TETHER}" \
    --arg zjq_timeout          "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: {
        _RBGN_GAR_HOST:         $zjq_gar_host,
        _RBGN_GAR_PATH:         $zjq_gar_path,
        _RBGN_RELIQUARIES_ROOT: $zjq_reliquaries_root,
        _RBGN_RELIQUARY:        $zjq_reliquary
      },
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose inscribe build JSON"

  buc_log_args "Inscribe build JSON: ${z_build_file}"

  rbrd_check "${z_token}"

  buc_step "Submitting inscribe Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "reliquary_build_create" "${z_build_file}"
  rbuh_require_ok "Inscribe build submission" "reliquary_build_create"

  local z_build_id=""
  z_build_id=$(rbuh_json_field_capture "reliquary_build_create" '.metadata.build.id') || z_build_id=""
  test -n "${z_build_id}" || buc_die "Build ID not found in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "Inscribe build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${ZRBFC_BUILD_POLL_CEILING_INSCRIBE}" "Inscribe"
}

rbfl_inscribe() {
  zrbfl_sentinel

  buc_doc_brief "Inscribe a reliquary: mirror all tool images from upstream to a datestamped GAR namespace"
  buc_doc_shown || return 0

  # Authenticate as Director
  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Compute reliquary datestamp: r + YYMMDDHHMMSS
  local -r z_reliquary="r${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"
  buc_info "Reliquary: ${z_reliquary}"

  # Submit inscribe as a Cloud Build job (docker pull/tag/push on GCB)
  zrbfl_inscribe_submit "${z_token}" "${z_reliquary}"

  buf_write_fact_single "${RBF_FACT_RELIQUARY}" "${z_reliquary}"

  buc_success "Inscribe complete — reliquary ${z_reliquary} created"
  buc_info "Add RBRV_RELIQUARY=${z_reliquary} to vessel rbrv.env files to use this reliquary"
}

# eof
