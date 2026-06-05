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
# Recipe Bottle Lode - capture-assembly spine (guard-free cluster, sourced by
# rbldk_): the data-driven Cloud Build composer shared by every Lode capture
# kind. Takes a recipe (ordered, pre-resolved step rows) plus an opaque
# substitutions blob, composes the Build resource, submits, polls, and decodes a
# step's buildStepOutputs slot. Owns NO kind knowledge: the recipe and the
# substitutions are data; the only kind-flavored build knob (the poll ceiling) is
# passed by the body. Capture-domain constants (the mason capture SA, the TETHER
# pool, the regime build timeout) are spine-owned, because every Lode capture
# fetches upstream bytes as mason over the network — that is what capture is, not
# a per-kind choice. Built on the rbfcb_ build primitives (write-script-body,
# wait-build-completion), which it calls rather than absorbs.
#
# Contract: RBSCJ "Capture Composition Contract".

set -euo pipefail

######################################################################
# Capture-assembly spine (zrbld_spine_*)

# Internal: compose, submit, and poll a Lode capture Cloud Build from a recipe
# plus an opaque substitutions blob.
#
# A recipe row is a |-delimited 4-tuple: script_path|builder_image|id|entrypoint
#   - script_path:   absolute path to the step script, pre-resolved by the body
#                    (the spine owns no steps-directory knowledge)
#   - builder_image: the step's builder image ref (colons allowed — hence the
#                    | delimiter, since image tags/digests contain colons)
#   - id:            the Cloud Build step id
#   - entrypoint:    bash | sh | python3 — selects the composed shebang line
#
# The substitutions file holds a JSON object the spine slots verbatim into the
# Build envelope's `substitutions` field; the spine reads no key from it. The
# envelope shape (serviceAccount, options.automapSubstitutions, options.logging,
# options.pool, timeout) is the spine's; the values that vary by capture domain
# (mason SA, TETHER pool, regime timeout) are spine-owned constants.
#
# Args: token label poll_ceiling subs_file temp_prefix recipe_row...
zrbld_spine_dispatch() {
  zrbld_sentinel

  local -r z_token="${1:?Token required}";                  shift
  local -r z_label="${1:?Label required}";                  shift
  local -r z_poll_ceiling="${1:?Poll ceiling required}";    shift
  local -r z_subs_file="${1:?Substitutions file required}"; shift
  local -r z_temp_prefix="${1:?Temp prefix required}";      shift
  test "$#" -ge 1 || buc_die "zrbld_spine_dispatch: recipe requires at least one step row"

  test -s "${z_subs_file}" || buc_die "Substitutions file missing or empty: ${z_subs_file}"

  buc_step "Composing ${z_label} Cloud Build steps from recipe"
  local -r z_steps_file="${z_temp_prefix}steps.json"
  echo "[]" > "${z_steps_file}" || buc_die "Failed to initialize ${z_label} steps JSON"

  local z_row=""
  local z_script_path=""
  local z_builder=""
  local z_id=""
  local z_entrypoint=""
  local z_body_file=""
  local z_escaped_file=""
  local z_steps_built=""
  local z_body=""
  local z_shebang=""
  for z_row in "$@"; do
    IFS='|' read -r z_script_path z_builder z_id z_entrypoint <<<"${z_row}"
    test -n "${z_script_path}" || buc_die "Recipe row missing script_path: ${z_row}"
    test -n "${z_builder}"     || buc_die "Recipe row missing builder_image: ${z_row}"
    test -n "${z_id}"          || buc_die "Recipe row missing id: ${z_row}"
    test -f "${z_script_path}" || buc_die "Step script not found: ${z_script_path}"

    z_body_file="${z_temp_prefix}${z_id}_body.txt"
    z_escaped_file="${z_temp_prefix}${z_id}_escaped.txt"
    z_steps_built="${z_temp_prefix}${z_id}_steps.json"

    zrbfc_write_script_body "${z_script_path}" "${z_body_file}" \
      || buc_die "Failed to read step script: ${z_script_path}"
    zrbfc_expand_includes "${z_body_file}" "${ZRBFC_RBGJS_SNIPPETS_DIR}" \
      || buc_die "Failed to expand snippet includes in step: ${z_script_path}"
    z_body=$(<"${z_body_file}")
    test -n "${z_body}" || buc_die "Empty step script body: ${z_script_path}"

    case "${z_entrypoint}" in
      bash)    z_shebang="#!/bin/bash" ;;
      sh)      z_shebang="#!/bin/sh" ;;
      python3) z_shebang="#!/usr/bin/env python3" ;;
      *)       buc_die "Unknown entrypoint '${z_entrypoint}' in recipe row: ${z_row}" ;;
    esac
    printf '%s\n%s' "${z_shebang}" "${z_body}" > "${z_escaped_file}" \
      || buc_die "Failed to write escaped step body for ${z_id}"

    jq \
      --arg name "${z_builder}" \
      --arg id "${z_id}" \
      --rawfile script "${z_escaped_file}" \
      '. + [{name: $name, id: $id, script: $script}]' \
      "${z_steps_file}" > "${z_steps_built}" \
      || buc_die "Failed to append step ${z_id}"
    mv "${z_steps_built}" "${z_steps_file}" \
      || buc_die "Failed to update steps JSON for ${z_id}"
  done

  buc_log_args "Composing ${z_label} Build resource JSON"
  local -r z_build_file="${z_temp_prefix}build.json"
  local -r z_mason_sa="projects/${RBDC_DEPOT_PROJECT_ID}/serviceAccounts/${RBGD_MASON_EMAIL}"

  jq -n \
    --slurpfile zjq_steps   "${z_steps_file}" \
    --slurpfile zjq_subs    "${z_subs_file}" \
    --arg       zjq_sa      "${z_mason_sa}" \
    --arg       zjq_pool    "${RBDC_POOL_TETHER}" \
    --arg       zjq_timeout "${RBRR_GCB_TIMEOUT}" \
    '{
      steps: $zjq_steps[0],
      substitutions: $zjq_subs[0],
      serviceAccount: $zjq_sa,
      options: {
        automapSubstitutions: true,
        logging: "CLOUD_LOGGING_ONLY",
        pool: { name: $zjq_pool }
      },
      timeout: $zjq_timeout
    }' > "${z_build_file}" \
    || buc_die "Failed to compose ${z_label} build JSON"

  buc_log_args "${z_label} build JSON: ${z_build_file}"

  rbrd_check "${z_token}"

  buc_step "Submitting ${z_label} Cloud Build"
  rbuh_json "POST" "${ZRBFC_GCB_PROJECT_BUILDS_URL}" "${z_token}" \
    "lode_build_create" "${z_build_file}"
  rbuh_require_ok "${z_label} build submission" "lode_build_create"

  local z_build_id
  z_build_id=$(rbuh_json_field_capture "lode_build_create" '.metadata.build.id') \
    || buc_die "Failed to capture build ID from builds.create response"
  test -n "${z_build_id}" || buc_die "Build ID empty in builds.create response"
  echo "${z_build_id}" > "${ZRBFC_BUILD_ID_FILE}" || buc_die "Failed to persist build ID"

  local -r z_console_url="${ZRBFC_CLOUD_QUERY_BASE};region=${RBGD_GCB_REGION}/${z_build_id}?project=${RBGD_GCB_PROJECT_ID}"
  buc_info "${z_label} Cloud Build submitted: ${z_build_id}"
  buc_link "Click to " "Open build in Cloud Console" "${z_console_url}"

  zrbfc_wait_build_completion "${z_poll_ceiling}" "${z_label}"
}

# Internal: decode the base64 JSON payload a step wrote to its buildStepOutputs
# slot into a destination file. The step index is the only generic parameter;
# what the decoded JSON means (its member-envelope shape) is the body's
# knowledge, not the spine's. Reads ZRBFC_BUILD_STATUS_FILE (the terminal build
# result registered by zrbfc_wait_build_completion).
# Args: step_index dest_file
zrbld_spine_extract() {
  zrbld_sentinel

  local -r z_step_index="${1:?Step index required}"
  local -r z_dest_file="${2:?Destination file required}"
  local -r z_b64_file="${z_dest_file}.b64"

  jq -r ".results.buildStepOutputs[${z_step_index}] // empty" "${ZRBFC_BUILD_STATUS_FILE}" \
    > "${z_b64_file}" || buc_die "Failed to extract buildStepOutputs[${z_step_index}] from build result"
  test -s "${z_b64_file}" || buc_die "No buildStepOutputs[${z_step_index}] in build result — step produced no output"

  rbgo_base64_decode_file_to_file "${z_b64_file}" "${z_dest_file}" \
    || buc_die "Failed to decode buildStepOutputs[${z_step_index}] base64"
  test -s "${z_dest_file}" || buc_die "Empty decoded buildStepOutputs[${z_step_index}]"
}

# eof
