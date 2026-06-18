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
# Recipe Bottle Federation Terrier - muniment access sub-operations
#
# The data layer over a provisioned terrier: the three atomic sub-operations
# engross / expunge / peruse that touch the Manor-homed terrier bucket, each a
# single conditioned REST call whose atomicity Cloud Storage adjudicates (no
# external lock, no cloud-build invocation). brevet / unseat / rehearse are the
# RBS0-side civic wrappers that compose these; this module carries no lock logic
# and no IAM — it is glue over a service. Contract: RBSTR-Terrier.adoc.
#
# A muniment is one GCS object per (principal subject, mantle held) pair — the
# settled per-entry granularity. Its object name indexes the pair under the
# polity managed folder; its content is the authoritative record (peruse
# reconstructs the holding from content, never by parsing the key). Per-entry
# muniments are immutable: a holding exists or it does not, so engross is a
# create (ifGenerationMatch=0) and expunge a delete — the RBSTR
# generation-conditional update path is unexercised under this granularity.
#
# Callers authenticate and pass the bearer token (token-first), like the rbgb_
# bucket primitives: the payor reads/writes as project owner today; a donned
# governor mantle writes own-polity once admission lands. The muniment wire keys
# live under the rbgft_ sprue (rbgft_subject, rbgft_mantle).

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGFT_SOURCED:-}" || buc_die "Module rbgft multiply sourced - check sourcing hierarchy"
ZRBGFT_SOURCED=1

######################################################################
# Internal Functions (zrbgft_*)

zrbgft_kindle() {
  test -z "${ZRBGFT_KINDLED:-}" || buc_die "Module rbgft already kindled"

  buc_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbuh_sentinel

  readonly ZRBGFT_PREFIX="${BURD_TEMP_DIR}/rbgft_"
  readonly ZRBGFT_MUNIMENT_BODY="${ZRBGFT_PREFIX}muniment.json"

  # Infix values for HTTP operations
  readonly ZRBGFT_INFIX_ENGROSS="terrier_engross"
  readonly ZRBGFT_INFIX_EXPUNGE="terrier_expunge"
  readonly ZRBGFT_INFIX_PERUSE_LIST="terrier_peruse_list"
  readonly ZRBGFT_INFIX_PERUSE_GET="terrier_peruse_get"

  readonly ZRBGFT_KINDLED=1
}

zrbgft_sentinel() {
  test "${ZRBGFT_KINDLED:-}" = "1" || buc_die "Module rbgft not kindled - call zrbgft_kindle first"
}

# Compose the muniment object name: the per-entry index under the polity managed
# folder. Three structural segments — <depot>/<mantle>/<subject> — with the raw
# principal subject carrying its own slashes; the whole name is percent-encoded
# once at transit time by the caller (rbuh_urlencode_capture), matching the
# rbgb_ object idiom.
zrbgft_muniment_name_capture() {
  zrbgft_sentinel
  local -r z_depot="${1}"
  local -r z_mantle="${2}"
  local -r z_subject="${3}"
  test -n "${z_depot}"   || return 1
  test -n "${z_mantle}"  || return 1
  test -n "${z_subject}" || return 1
  printf '%s/%s/%s' "${z_depot}" "${z_mantle}" "${z_subject}"
}

######################################################################
# External Functions (rbgft_*)
#
# engross / expunge echo a one-word disposition on stdout (their only stdout
# output; all human logging routes to stderr); peruse echoes one muniment per
# line. Callers capture the disposition to assert the precondition outcome.

# rbgft_engross <token> <bucket> <depot_project_id> <mantle> <subject>
# Write the muniment for (subject, mantle) into the depot's polity slice.
# ifGenerationMatch=0 create — Cloud Storage writes only if absent. Echoes
# "created" on a fresh write (200/201) or "present" on the 412 precondition
# (RBSTR: a duplicate create is idempotent success, the muniment already holds).
# buc_die on any other code.
rbgft_engross() {
  zrbgft_sentinel

  local -r z_token="${1:-}"
  local -r z_bucket="${2:-}"
  local -r z_depot="${3:-}"
  local -r z_mantle="${4:-}"
  local -r z_subject="${5:-}"

  test -n "${z_token}"   || buc_die "Token required"
  test -n "${z_bucket}"  || buc_die "Bucket required"
  test -n "${z_depot}"   || buc_die "Depot project id required"
  test -n "${z_mantle}"  || buc_die "Mantle required"
  test -n "${z_subject}" || buc_die "Principal subject required"

  buc_step "Engross muniment (${z_mantle}) for ${z_subject}"

  buc_log_args 'Build the authoritative muniment body — the key is only the index'
  jq -n --arg subject "${z_subject}" --arg mantle "${z_mantle}" \
    '{rbgft_subject: $subject, rbgft_mantle: $mantle}' > "${ZRBGFT_MUNIMENT_BODY}" \
    || buc_die "Failed to build muniment JSON"

  local z_objname
  z_objname=$(zrbgft_muniment_name_capture "${z_depot}" "${z_mantle}" "${z_subject}") \
    || buc_die "Failed to compose muniment object name"
  local z_name_enc
  z_name_enc=$(rbuh_urlencode_capture "${z_objname}") || buc_die "Failed to encode object name"

  buc_log_args 'Media upload with ifGenerationMatch=0 — create only if absent; concurrent creators race cleanly (RBSTR)'
  local -r z_url="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_UPLOAD}/b/${z_bucket}/o?uploadType=media&name=${z_name_enc}&ifGenerationMatch=0"
  rbuh_json "POST" "${z_url}" "${z_token}" "${ZRBGFT_INFIX_ENGROSS}" "${ZRBGFT_MUNIMENT_BODY}"

  local z_code
  z_code=$(rbuh_code_capture "${ZRBGFT_INFIX_ENGROSS}") || buc_die "Bad engross HTTP code"
  case "${z_code}" in
    200|201) buc_success "Muniment engrossed (${z_mantle}, ${z_subject})"; echo "created" ;;
    412)     buc_info    "Muniment already present, idempotent (${z_mantle}, ${z_subject})"; echo "present" ;;
    *)       local z_err
             z_err=$(rbuh_json_field_capture "${ZRBGFT_INFIX_ENGROSS}" '.error.message') || z_err="HTTP ${z_code}"
             buc_die "Failed to engross muniment (HTTP ${z_code}): ${z_err}" ;;
  esac
}

# rbgft_expunge <token> <bucket> <depot_project_id> <mantle> <subject>
# Withdraw the muniment for (subject, mantle). Echoes "deleted" (204) or
# "absent" (404 — idempotent, already struck from the record). buc_die otherwise.
rbgft_expunge() {
  zrbgft_sentinel

  local -r z_token="${1:-}"
  local -r z_bucket="${2:-}"
  local -r z_depot="${3:-}"
  local -r z_mantle="${4:-}"
  local -r z_subject="${5:-}"

  test -n "${z_token}"   || buc_die "Token required"
  test -n "${z_bucket}"  || buc_die "Bucket required"
  test -n "${z_depot}"   || buc_die "Depot project id required"
  test -n "${z_mantle}"  || buc_die "Mantle required"
  test -n "${z_subject}" || buc_die "Principal subject required"

  buc_step "Expunge muniment (${z_mantle}) for ${z_subject}"

  local z_objname
  z_objname=$(zrbgft_muniment_name_capture "${z_depot}" "${z_mantle}" "${z_subject}") \
    || buc_die "Failed to compose muniment object name"
  local z_name_enc
  z_name_enc=$(rbuh_urlencode_capture "${z_objname}") || buc_die "Failed to encode object name"

  local -r z_url="${RBGC_API_BASE_GCS}/b/${z_bucket}/o/${z_name_enc}"
  rbuh_json "DELETE" "${z_url}" "${z_token}" "${ZRBGFT_INFIX_EXPUNGE}"

  local z_code
  z_code=$(rbuh_code_capture "${ZRBGFT_INFIX_EXPUNGE}") || buc_die "Bad expunge HTTP code"
  case "${z_code}" in
    204) buc_success "Muniment expunged (${z_mantle}, ${z_subject})"; echo "deleted" ;;
    404) buc_info    "Muniment already absent, idempotent (${z_mantle}, ${z_subject})"; echo "absent" ;;
    *)   local z_err
         z_err=$(rbuh_json_field_capture "${ZRBGFT_INFIX_EXPUNGE}" '.error.message') || z_err="HTTP ${z_code}"
         buc_die "Failed to expunge muniment (HTTP ${z_code}): ${z_err}" ;;
  esac
}

# rbgft_peruse <token> <bucket> <depot_project_id>
# The pure list-and-fetch read of one polity's muniments — no precondition. Lists
# every object under the polity folder prefix, fetches each object's content, and
# echoes one tab-separated "<mantle>\t<subject>" line per muniment (read from the
# rbgft_ content fields, never by parsing the key). The read side rehearse
# composes and the read side of the reconciliation diff.
rbgft_peruse() {
  zrbgft_sentinel

  local -r z_token="${1:-}"
  local -r z_bucket="${2:-}"
  local -r z_depot="${3:-}"

  test -n "${z_token}"  || buc_die "Token required"
  test -n "${z_bucket}" || buc_die "Bucket required"
  test -n "${z_depot}"  || buc_die "Depot project id required"

  buc_step "Peruse muniments for polity ${z_depot}"

  local -r z_prefix="${z_depot}/"
  local z_prefix_enc
  z_prefix_enc=$(rbuh_urlencode_capture "${z_prefix}") || buc_die "Failed to encode polity prefix"

  buc_log_args 'Page through the polity prefix, fetching each muniment body'
  local z_page_token=""
  local z_page=0
  while :; do
    z_page=$((z_page + 1))
    local z_url="${RBGC_API_BASE_GCS}/b/${z_bucket}/o?prefix=${z_prefix_enc}"
    if test -n "${z_page_token}"; then
      local z_tok_enc
      z_tok_enc=$(rbuh_urlencode_capture "${z_page_token}") || buc_die "Failed to encode pageToken"
      z_url="${z_url}&pageToken=${z_tok_enc}"
    fi

    local z_list_infix="${ZRBGFT_INFIX_PERUSE_LIST}${z_page}"
    rbuh_json "GET" "${z_url}" "${z_token}" "${z_list_infix}"
    rbuh_require_ok "Peruse: list polity muniments" "${z_list_infix}"

    local z_list_file="${ZRBUH_PREFIX}${z_list_infix}${ZRBUH_POSTFIX_JSON}"
    local z_names
    z_names=$(jq -r '.items[]?.name // empty' "${z_list_file}") || buc_die "Failed to read muniment listing"

    local z_name=""
    while IFS= read -r z_name; do
      test -n "${z_name}" || continue
      local z_name_enc
      z_name_enc=$(rbuh_urlencode_capture "${z_name}") || buc_die "Failed to encode muniment name"
      rbuh_json "GET" "${RBGC_API_BASE_GCS}/b/${z_bucket}/o/${z_name_enc}?alt=media" \
        "${z_token}" "${ZRBGFT_INFIX_PERUSE_GET}"
      rbuh_require_ok "Peruse: fetch muniment ${z_name}" "${ZRBGFT_INFIX_PERUSE_GET}"

      local z_get_file="${ZRBUH_PREFIX}${ZRBGFT_INFIX_PERUSE_GET}${ZRBUH_POSTFIX_JSON}"
      jq -r '[.rbgft_mantle, .rbgft_subject] | @tsv' "${z_get_file}" \
        || buc_die "Muniment ${z_name} missing rbgft_ fields"
    done <<< "${z_names}"

    z_page_token=$(jq -r '.nextPageToken // empty' "${z_list_file}") || buc_die "Failed to read nextPageToken"
    test -n "${z_page_token}" || break
  done
}

# eof
