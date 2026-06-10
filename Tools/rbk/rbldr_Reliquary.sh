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
# Recipe Bottle Lode - reliquary body (guard-free cluster, sourced by rbld0_Lode):
#   conclave — convene the build-tool cohort into a Lode (Director credentials)
# The reliquary rides the capture-assembly spine (rblds_): this body owns only the
# kind-specific data — the conclave recipe (gcrane cohort capture + vouch-push),
# the substitutions blob, and the touchmark-fact extract — and composes
# them through zrbld_spine_dispatch / zrbld_spine_extract. No build-submission or
# step-composition machinery lives here.
#
# Conclave absorbs today's inscribe pull machinery (rbfli_Inscribe + rbgji01),
# retargeted from the rbi_rq/<date>/<tool> sibling-package layout to one rbi_ld
# package holding N member tags (:rbi_<tool>) plus the :rbi_vouch envelope. Both
# steps ride the floating gcrane builder (ZRBLD_GCRANE_BUILDER): the tools captured
# here ARE the reliquary, so capture cannot bootstrap from one — conclave is the
# generation phase the pinning rule permits to run unpinned (RBS0 rbsk_pinning_boundary).

set -euo pipefail

# Conclave is capture-pure: it writes no consumer config. It hands the captured
# touchmark to a later explicit yoke-stamp election (the reliquary cutover, a
# separate pace) through two bare single-form chaining facts
# (RBF_FACT_LODE_TOUCHMARK + RBF_FACT_LODE_BRAND/RBGC_LODE_BRAND_RELIQUARY). The
# provenance envelope lives only in GAR (:rbi_vouch tag, pushed cloud-side by
# rbgjl02), never host-side.

######################################################################
# Internal Helpers (zrbld_*)

# Internal: compose the conclave capture recipe (gcrane cohort capture + vouch-push)
# and its substitutions blob, then ride the capture spine to submit
# and poll. The spine owns the capture-domain build knobs (mason SA, TETHER pool,
# regime timeout); this body chooses only the recipe, the substitutions, and the
# inscribe-borrowed poll ceiling (the cohort copy needs the larger ceiling — same
# work inscribe performs today).
# Args: token stamp
zrbld_conclave_submit() {
  zrbld_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_stamp="${2:?Stamp required}"

  buc_step "Constructing conclave capture recipe"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"

  # Recipe rows: script_path|builder_image|id|entrypoint, pre-resolved for the
  # spine. Both steps ride the floating gcrane builder (busybox entrypoint —
  # gcrane:debug's only shell) — no reliquary bootstrap (conclave IS what captures
  # the reliquary tools; generation-tier, the one phase allowed unpinned).
  local -r z_recipe=(
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl03-conclave-capture.sh|${ZRBLD_GCRANE_BUILDER}|conclave-capture|busybox"
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl02-assemble-push-vouch.sh|${ZRBLD_GCRANE_BUILDER}|assemble-push-vouch|busybox"
  )

  buc_log_args "Composing conclave substitutions blob"
  local -r z_subs_file="${ZRBLD_CONCLAVE_PREFIX}subs.json"
  jq -n \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_lodes_root   "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_sprue    "${RBGC_LODE_TAG_SPRUE}" \
    --arg zjq_tag_vouch    "${RBGC_LODE_TAG_VOUCH}" \
    --arg zjq_trust_grade  "${RBGC_LODE_TRUST_VERIFIED}" \
    --arg zjq_vouch_schema "${RBGC_LODE_VOUCH_SCHEMA}" \
    --arg zjq_acquired_by  "${RBGD_MASON_EMAIL}" \
    --arg zjq_stamp        "${z_stamp}" \
    '{
      _RBGL_GAR_HOST:     $zjq_gar_host,
      _RBGL_GAR_PATH:     $zjq_gar_path,
      _RBGL_LODES_ROOT:   $zjq_lodes_root,
      _RBGL_TAG_SPRUE:    $zjq_tag_sprue,
      _RBGL_TAG_VOUCH:    $zjq_tag_vouch,
      _RBGL_TRUST_GRADE:  $zjq_trust_grade,
      _RBGL_VOUCH_SCHEMA: $zjq_vouch_schema,
      _RBGL_ACQUIRED_BY:  $zjq_acquired_by,
      _RBGL_LODE_STAMP:   $zjq_stamp
    }' > "${z_subs_file}" \
    || buc_die "Failed to compose conclave substitutions blob"

  zrbld_spine_dispatch \
    "${z_token}" "${RBGD_MASON_EMAIL}" "Conclave" "${ZRBFC_BUILD_POLL_CEILING_INSCRIBE}" \
    "${z_subs_file}" "${ZRBLD_CONCLAVE_PREFIX}" \
    "${z_recipe[@]}"
}

# Internal: extract the captured touchmark from the completed conclave build and
# emit the two bare single-form chaining facts (touchmark value + kind-brand
# enum). The capture step (step 0) authors the base64 JSON carrying the
# host-minted stamp in slot_1; the vouch-push step writes no output. Conclave
# captures exactly one Lode (the cohort is one package), so exactly one slot is
# populated. The provenance envelope is NOT read host-side: it lives only in GAR
# (rbgjl02 pushed it under :rbi_vouch), so the host hands forward only the
# touchmark a consumer needs.
zrbld_conclave_extract() {
  zrbld_sentinel

  buc_step "Extracting capture results from build step outputs"

  local -r z_output_file="${ZRBLD_CONCLAVE_PREFIX}output.json"
  zrbld_spine_extract 0 "${z_output_file}"

  buc_log_args "Conclave output:"
  buc_log_pipe < "${z_output_file}"

  local -r z_stamp_file="${ZRBLD_CONCLAVE_PREFIX}stamp.txt"
  jq -r '.rbls_slot_1.rbls_stamp // empty' "${z_output_file}" > "${z_stamp_file}" \
    || buc_die "Failed to read reliquary stamp from conclave output"
  local -r z_stamp=$(<"${z_stamp_file}")
  test -n "${z_stamp}" || buc_die "Conclave output carried no stamp in rbls_slot_1"

  buf_write_fact_single "${RBF_FACT_LODE_TOUCHMARK}" "${z_stamp}" \
    || buc_die "Failed to write touchmark fact for ${z_stamp}"
  buf_write_fact_single "${RBF_FACT_LODE_BRAND}" "${RBGC_LODE_BRAND_RELIQUARY}" \
    || buc_die "Failed to write kind-brand fact for ${z_stamp}"
  buc_success "Conclave captured Lode ${z_stamp} — touchmark fact emitted (${RBGC_LODE_BRAND_RELIQUARY})"
}

######################################################################
# External Functions (rbld_*)

rbld_conclave() {
  zrbld_sentinel

  buc_doc_brief "Convene the build-tool cohort into one Lode (reliquary kind, rbi_ld capture)"
  buc_doc_shown || return 0

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Mint the Lode stamp on the host: <kind-letter><YYMMDDHHMMSS>. The host owns
  # the stamp so the touchmark is known before the build for the capture-file.
  # The reliquary kind-letter 'r' matches the legacy inscribe datestamp prefix,
  # but the namespaces differ (rbi_ld vs rbi_rq), so no collision.
  local -r z_stamp="${RBGC_LODE_KIND_RELIQUARY}${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"

  buc_info "Lode: ${RBGL_LODES_ROOT}/${z_stamp}"

  zrbld_conclave_submit "${z_token}" "${z_stamp}"
  zrbld_conclave_extract

  buc_success "Conclave complete: build-tool cohort -> ${RBGL_LODES_ROOT}/${z_stamp}"
}

# eof
