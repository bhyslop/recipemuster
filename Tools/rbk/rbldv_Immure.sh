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
# Recipe Bottle Lode - podvm body (guard-free cluster, sourced by rbld0_Lode):
#   immure — wall in the selected podman-machine disk leaves of one quay family into
#            a Lode (Director credentials)
# The podvm kind rides the capture-assembly spine (rblds_): this body owns only the
# kind-specific data — the immure recipe (anon-quay index select + gcrane cp-by-digest
# + blob-residency guard + vouch-push), the substitutions blob, and the touchmark-fact
# extract — and composes them through zrbld_spine_dispatch / zrbld_spine_extract. No
# build-submission or step-composition machinery lives here.
#
# Shape: opaque-blob (like wsl/underpin) x multi-member (like reliquary/conclave). Each
# selected disk leaf is a single-platform OCI artifact (empty config + one zstd blob);
# the cohort of leaves rides as N member tags within one GAR package. One verb spans
# BOTH quay families (podvm-wsl: quay.io/podman/machine-os-wsl, podvm-native:
# quay.io/podman/machine-os) via the family argument — not two verbs.
#
# Recorded-at-acquisition grade: quay rotates podvm out within days and publishes no
# durable checksum, so RB attests only the leaf digest captured (trust-on-first-
# acquisition). Cloud-side only; the workstation assembles no bytes, only the
# declarative family + version + curated leaf-set.

set -euo pipefail

# Immure is capture-pure: it writes no consumer config. It hands the captured
# touchmark forward through two bare single-form chaining facts
# (RBF_FACT_LODE_TOUCHMARK + RBF_FACT_LODE_BRAND/the podvm brand). The provenance
# envelope lives only in GAR (:rbi_vouch tag, pushed cloud-side by rbgjl02), never
# host-side. Consumption (a host's `podman machine init` from the captured seed) is a
# separate, deferred layer that reads these facts — not part of immure.

######################################################################
# Internal Helpers (zrbld_*)

# Internal: resolve the family argument to its (kind-letter, quay-family, selection,
# brand) tuple. The brand IS the operator-typed argument and the envelope kind field.
# podvm-wsl is fixture-proven this pace; podvm-native is wired but its full curation +
# refresh mode are the FOLLOWING pace (see rbgc_Constants podvm selection block).
# Args: family   Sets: z_kind, z_quay_family, z_selection (caller-scoped locals)
zrbld_immure_resolve_family() {
  zrbld_sentinel
  local -r zz_family="${1:?Family required}"
  case "${zz_family}" in
    "${RBGC_LODE_BRAND_PODVM_WSL}")
      z_kind="${RBGC_LODE_KIND_PODVM_WSL}"
      z_quay_family="${RBGC_LODE_PODVM_FAMILY_WSL}"
      z_selection="${RBGC_LODE_PODVM_WSL_SELECTION}"
      ;;
    "${RBGC_LODE_BRAND_PODVM_NATIVE}")
      z_kind="${RBGC_LODE_KIND_PODVM_NATIVE}"
      z_quay_family="${RBGC_LODE_PODVM_FAMILY_NATIVE}"
      z_selection="${RBGC_LODE_PODVM_NATIVE_SELECTION}"
      ;;
    *)
      buc_die "Unknown podvm family '${zz_family}' (expected ${RBGC_LODE_BRAND_PODVM_WSL} or ${RBGC_LODE_BRAND_PODVM_NATIVE})"
      ;;
  esac
}

# Internal: compose the immure capture recipe (anon-quay index select + gcrane
# cp-by-digest + blob-residency guard + vouch-push) and its substitutions blob, then
# ride the capture spine to submit and poll. The spine owns the capture-domain build
# knobs (mason SA, TETHER pool, regime timeout); this body chooses only the recipe,
# the substitutions, and the inscribe-grade poll ceiling (the multi-GB leaf copies
# want the larger ceiling). Four steps across three builders: index-select rides the
# gcloud builder (python3 — parses the upstream OCI index, which the no-jq bash GCB
# discipline does not cover; rbgjl06 precedent); gcrane cp and the vouch-push ride the
# floating gcrane builder (busybox); the residency HEAD rides the Debian docker builder
# (curl, allowlisted). The recipe-row ORDER is part of the contract: vouch (rbgjl02)
# runs strictly after residency (rbgjl09) — the vouch artifact never precedes the
# anti-hollow-mirror guard. podvm is vessel-less (no reliquary slot), so its gcrane
# rides the floating bootstrap builder, same tier as conclave/wsl — pinning defers to
# the bootstrap-builder digest-pin itch (RBS0 rbsk_pinning_boundary).
# Args: token brand quay_family version selection stamp
zrbld_immure_submit() {
  zrbld_sentinel

  local -r z_token="${1:?Token required}"
  local -r z_brand="${2:?Brand required}"
  local -r z_quay_family="${3:?Family required}"
  local -r z_version="${4:?Version required}"
  local -r z_selection="${5:?Selection required}"
  local -r z_stamp="${6:?Stamp required}"

  buc_step "Constructing immure capture recipe"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_gar_path="${RBGD_GAR_PROJECT_ID}/${RBDC_GAR_REPOSITORY}"

  # Recipe rows: script_path|builder_image|id|entrypoint, pre-resolved for the spine.
  # Select + residency on the Debian Google builder (curl + apt jq); cp + vouch on the
  # floating gcrane builder (busybox). The gcrane builder reads public quay anonymously
  # and pushes GAR ambiently (google.Keychain -> Mason SA).
  local -r z_recipe=(
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl07-immure-select.py|${ZRBLD_GCLOUD_BUILDER}|immure-select|python3"
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl08-immure-capture.sh|${ZRBLD_GCRANE_BUILDER}|immure-capture|busybox"
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl09-immure-residency.sh|${ZRBLD_GOOGLE_DOCKER_BUILDER}|immure-residency|bash"
    "${ZRBLD_RBGJL_STEPS_DIR}/rbgjl02-assemble-push-vouch.sh|${ZRBLD_GCRANE_BUILDER}|assemble-push-vouch|busybox"
  )

  buc_log_args "Composing immure substitutions blob"
  local -r z_subs_file="${ZRBLD_IMMURE_PREFIX}subs.json"
  jq -n \
    --arg zjq_gar_host     "${z_gar_host}" \
    --arg zjq_gar_path     "${z_gar_path}" \
    --arg zjq_lodes_root   "${RBGL_LODES_ROOT}" \
    --arg zjq_tag_sprue    "${RBGC_LODE_TAG_SPRUE}" \
    --arg zjq_tag_vouch    "${RBGC_LODE_TAG_VOUCH}" \
    --arg zjq_trust_grade  "${RBGC_LODE_TRUST_RECORDED}" \
    --arg zjq_vouch_schema "${RBGC_LODE_VOUCH_SCHEMA}" \
    --arg zjq_acquired_by  "${RBGD_MASON_EMAIL}" \
    --arg zjq_stamp        "${z_stamp}" \
    --arg zjq_brand        "${z_brand}" \
    --arg zjq_family       "${z_quay_family}" \
    --arg zjq_version      "${z_version}" \
    --arg zjq_selection    "${z_selection}" \
    '{
      _RBGL_GAR_HOST:        $zjq_gar_host,
      _RBGL_GAR_PATH:        $zjq_gar_path,
      _RBGL_LODES_ROOT:      $zjq_lodes_root,
      _RBGL_TAG_SPRUE:       $zjq_tag_sprue,
      _RBGL_TAG_VOUCH:       $zjq_tag_vouch,
      _RBGL_TRUST_GRADE:     $zjq_trust_grade,
      _RBGL_VOUCH_SCHEMA:    $zjq_vouch_schema,
      _RBGL_ACQUIRED_BY:     $zjq_acquired_by,
      _RBGL_LODE_STAMP:      $zjq_stamp,
      _RBGL_PODVM_BRAND:     $zjq_brand,
      _RBGL_PODVM_FAMILY:    $zjq_family,
      _RBGL_PODVM_VERSION:   $zjq_version,
      _RBGL_PODVM_SELECTION: $zjq_selection
    }' > "${z_subs_file}" \
    || buc_die "Failed to compose immure substitutions blob"

  zrbld_spine_dispatch \
    "${z_token}" "${RBGD_MASON_EMAIL}" "Immure" "${ZRBFC_BUILD_POLL_CEILING_INSCRIBE}" \
    "${z_subs_file}" "${ZRBLD_IMMURE_PREFIX}" \
    "${z_recipe[@]}"
}

# Internal: extract the captured touchmark from the completed immure build and emit
# the two bare single-form chaining facts (touchmark value + kind-brand enum). The
# select step (step 0) authors the base64 JSON carrying the host-minted stamp in
# slot_1; the capture/residency/vouch steps write no output. Immure captures exactly
# one Lode (the cohort is one package), so exactly one slot is populated. The
# provenance envelope is NOT read host-side: it lives only in GAR (rbgjl02 pushed it
# under :rbi_vouch), so the host hands forward only the touchmark a consumer needs.
# Args: brand
zrbld_immure_extract() {
  zrbld_sentinel
  local -r z_brand="${1:?Brand required}"

  buc_step "Extracting capture results from build step outputs"

  local -r z_output_file="${ZRBLD_IMMURE_PREFIX}output.json"
  zrbld_spine_extract 0 "${z_output_file}"

  buc_log_args "Immure output:"
  buc_log_pipe < "${z_output_file}"

  local -r z_stamp_file="${ZRBLD_IMMURE_PREFIX}stamp.txt"
  jq -r '.slot_1.stamp // empty' "${z_output_file}" > "${z_stamp_file}" \
    || buc_die "Failed to read podvm stamp from immure output"
  local -r z_stamp=$(<"${z_stamp_file}")
  test -n "${z_stamp}" || buc_die "Immure output carried no stamp in slot_1"

  buf_write_fact_single "${RBF_FACT_LODE_TOUCHMARK}" "${z_stamp}" \
    || buc_die "Failed to write touchmark fact for ${z_stamp}"
  buf_write_fact_single "${RBF_FACT_LODE_BRAND}" "${z_brand}" \
    || buc_die "Failed to write kind-brand fact for ${z_stamp}"
  buc_success "Immure captured Lode ${z_stamp} — touchmark fact emitted (${z_brand})"
}

######################################################################
# External Functions (rbld_*)

rbld_immure() {
  zrbld_sentinel

  buc_doc_brief "Wall in the selected podman-machine disk leaves of one quay family into a Lode (podvm kind, rbi_ld capture)"
  buc_doc_param "family"  "Quay family — ${RBGC_LODE_BRAND_PODVM_WSL} or ${RBGC_LODE_BRAND_PODVM_NATIVE}"
  buc_doc_param "version" "Podman version tag on the family index (e.g. 5.6)"
  buc_doc_shown || return 0

  # Two declarative arguments (no FQIN — see RBSLI): the param1 channel routes the
  # first to BUZ_FOLIO and forwards the rest, so family is the folio and version the
  # first positional. The curated leaf-set is a per-family constant; the cloud step
  # resolves leaf digests from the live index at capture time.
  local -r z_brand="${BUZ_FOLIO:-}"
  local -r z_version="${1:-}"
  test -n "${z_brand}"   || buc_die "family argument required (${RBGC_LODE_BRAND_PODVM_WSL} or ${RBGC_LODE_BRAND_PODVM_NATIVE})"
  test -n "${z_version}" || buc_die "version argument required (e.g. 5.6)"

  local z_kind="" z_quay_family="" z_selection=""
  zrbld_immure_resolve_family "${z_brand}"
  buc_info "Immure family: ${z_brand} (${z_quay_family}:${z_version}), leaves: ${z_selection}"

  buc_step "Loading Director RBRA credentials"
  source "${RBDC_DIRECTOR_RBRA_FILE}" || buc_die "Failed to source Director RBRA"

  buc_step "Authenticating as Director"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  # Mint the Lode stamp on the host: <kind-letter><YYMMDDHHMMSS>. The host owns the
  # stamp so the touchmark is known before the build for the capture-file. podvm
  # kind-letters are two characters (vw/vn); the augur touchmark regex accepts the
  # multi-letter prefix.
  local -r z_stamp="${z_kind}${BURD_NOW_STAMP:2:6}${BURD_NOW_STAMP:9:6}"

  buc_info "Lode: ${RBGL_LODES_ROOT}/${z_stamp}"

  zrbld_immure_submit "${z_token}" "${z_brand}" "${z_quay_family}" "${z_version}" "${z_selection}" "${z_stamp}"
  zrbld_immure_extract "${z_brand}"

  buc_success "Immure complete: ${z_quay_family}:${z_version} -> ${RBGL_LODES_ROOT}/${z_stamp}"
}

# eof
