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
# RBGV CLI - Recipe Bottle Access Probe command-line interface
#
# Surfaces the credential access probes implemented by the rbgv library
# module as four operator tabtargets (one per role). Theurge consumes these
# as plain subprocesses, owning no colophons of its own.
#
# Commands:
#   check_governor   JWT SA access probe for the governor credential
#   check_retriever  JWT SA access probe for the retriever credential
#   check_director   JWT SA access probe for the director credential
#   check_payor      OAuth access probe for the payor credential

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# CLI Commands

# JWT SA propagation-absorbing budget: 60 attempts × 3000ms ≈ 180s worst case.
# Happy path exits on first 2xx; budget consumed only when post-mint IAM
# propagation lags. Each retry mints a fresh JWT-OAuth token, so the inner
# rbgo_get_token_capture race (BBAAp) is exercised independently per attempt.
zrbgv_jwt_check() {
  local -r z_role="$1"
  local -r z_iterations=60
  local -r z_delay_ms=3000
  buc_step "JWT SA access probe: ${z_role}"
  rbgv_jwt_sa_probe "${z_role}" "${z_iterations}" "${z_delay_ms}"
  buc_success "${z_role} JWT access probe passed"
}

rbgv_check_governor() {
  zrbgv_sentinel
  buc_doc_brief "Check the governor credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "governor"
}

rbgv_check_retriever() {
  zrbgv_sentinel
  buc_doc_brief "Check the retriever credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "retriever"
}

rbgv_check_director() {
  zrbgv_sentinel
  buc_doc_brief "Check the director credential reaches Google Cloud (JWT SA access probe)"
  buc_doc_shown || return 0
  zrbgv_jwt_check "director"
}

rbgv_check_payor() {
  zrbgv_sentinel
  buc_doc_brief "Check the payor credential reaches Google Cloud (OAuth access probe)"
  buc_doc_shown || return 0

  # Payor probe semantic: stability-sample loop.
  local -r z_iterations=5
  local -r z_delay_ms=1500
  buc_step "Payor OAuth access probe"
  source "${RBCC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBCC_rbrp_file}"
  zrbrp_kindle
  zrbrp_enforce
  rbgv_payor_oauth_probe "${z_iterations}" "${z_delay_ms}"
  buc_success "Payor OAuth access probe passed"
}

# Federated access probe — triggers the accessor's compearance step (Legs 1+2):
# cache-hit reuses a live assize; a miss with a terminal runs the device flow +
# STS exchange and caches the federated token; a headless miss fails loud. This
# is the cloud tabtarget that exercises compearance — compearance itself owns no
# colophon. Depot-agnostic: needs only the RBRF trust, not RBRR/RBRD.
rbgv_check_compearance() {
  zrbgv_sentinel
  buc_doc_brief "Check federated access — open or reuse an assize via device flow + STS (Legs 1+2) against the RBRF trust"
  buc_doc_shown || return 0

  buc_step "Federated access probe — compearance against the RBRF trust"
  source "${RBCC_rbrf_file}" || buc_die "Failed to source RBRF: ${RBCC_rbrf_file}"
  zrbrf_kindle
  zrbrf_enforce

  rba_compear

  local z_token
  z_token=$(zrba_assize_read_capture) || buc_die "Assize not readable after compearance"
  test -n "${z_token}" || buc_die "Assize holds an empty federated token"
  buc_success "Federated assize live — federated token obtained (${#z_token} chars)"
}

# Don-mantle probe — exercises the freehold rig end to end: resolve the freehold
# subject (the operator's standing Entra oid, RBPC_freehold_subject), compear as
# that identity to open or reuse an assize, then don the named mantle (Leg 3).
# Reports the minted mantle token, or surfaces the admission deficit the accessor
# already characterized (the Leg-3 403: a citizen not brevetted onto the mantle,
# or a missing depot serviceUsageConsumer — a standing state, not a propagation
# race). Depot-coupled, unlike the depot-agnostic compearance probe: the don
# derives the depot project (RBDC_DEPOT_PROJECT_ID), so furnish enforces RBRR/RBRD
# for this command; the RBRF trust is sourced here as compearance does.
rbgv_check_mantle() {
  zrbgv_sentinel

  # The mantle operand arrives via the BUZ_FOLIO env channel (param1 colophon —
  # buz_exec_lookup shifts the folio off the positional args and exports it), NOT
  # as a positional. Documented in zrbgv_furnish's buc_doc_env block.
  local -r z_mantle="${BUZ_FOLIO:-}"

  buc_doc_brief "Don a mantle as the freehold subject — compear, then don the named mantle (or surface the admission deficit)"
  buc_doc_shown || return 0

  case "${z_mantle}" in
    governor|director|retriever) ;;
    *) buc_die "rbgv_check_mantle: mantle parameter required (governor | director | retriever), got '${z_mantle:-<empty>}'" ;;
  esac

  buc_step "Don-mantle probe — ${z_mantle} mantle as the freehold subject"

  buc_step "Resolve the freehold subject"
  test -n "${RBPC_freehold_subject:-}" || buc_die "RBPC_freehold_subject is not set — rbpc_constants.sh must be sourced"
  buc_info "Freehold subject (compear as this identity): ${RBPC_freehold_subject}"

  buc_step "Compear against the RBRF trust to open or reuse the assize"
  source "${RBCC_rbrf_file}" || buc_die "Failed to source RBRF: ${RBCC_rbrf_file}"
  zrbrf_kindle
  zrbrf_enforce

  rba_compear

  # Confirm the human compeared as the freehold subject. The cached subject is
  # the decoded oid (best-effort, informational per the accessor): a mismatch
  # warns — the rig is being exercised under a different identity — but does not
  # gate the don; an undecodable subject is skipped with a log note.
  local z_cached_subject=""
  z_cached_subject=$(zrba_assize_subject_capture) || z_cached_subject=""
  if test -z "${z_cached_subject}"; then
    buc_log_args "Compeared subject not decodable from the assize cache — skipping the freehold-identity confirmation (informational only)"
  elif test "${z_cached_subject}" = "${RBPC_freehold_subject}"; then
    buc_info "Compeared subject matches the freehold subject"
  else
    buc_warn "Compeared subject '${z_cached_subject}' is NOT the freehold subject '${RBPC_freehold_subject}' — donning anyway, but this is not the freehold identity"
  fi

  buc_step "Don the ${z_mantle} mantle (Leg 3)"
  # rba_don_capture emits the mantle token on success or returns 1 having already
  # logged the admission-deficit / lapsed-assize forensic line; the probe surfaces
  # that as its verdict (the accessor owns the diagnosis). The token is held in a
  # process-local var, never persisted, and only its length is reported.
  local z_mantle_token
  z_mantle_token=$(rba_don_capture "${z_mantle}") \
    || buc_die "Don of the ${z_mantle} mantle failed — see the transcript (admission deficit or lapsed assize)"
  test -n "${z_mantle_token}" || buc_die "Don of the ${z_mantle} mantle returned an empty token"

  # Exercise the minted token against Artifact Registry (repositories.list). This
  # proves the donned token actually REACHES AR — not merely that it minted — and
  # writes the spike-V3 use-hop Data-Access audit entry that attributes the act to
  # the human (serviceAccountDelegationInfo[].principalSubject). Read that trail
  # back with rbw-da to see the freehold subject named at the using service.
  buc_step "Exercise the ${z_mantle} mantle token against Artifact Registry (repositories.list)"
  local z_ar_code
  z_ar_code=$(zrbgv_mantle_ar_call_capture "${z_mantle_token}") \
    || buc_die "Mantle AR call failed for the ${z_mantle} mantle"
  case "${z_ar_code}" in
    200|206)
      buc_info "Artifact Registry reachable under the ${z_mantle} mantle (HTTP ${z_ar_code})"
      ;;
    403)
      buc_die "The ${z_mantle} mantle donned but Artifact Registry denied access (HTTP 403) — the mantle SA lacks artifactregistry.reader, or its capability-set was not granted at levy"
      ;;
    *)
      buc_die "Mantle AR call: unexpected HTTP ${z_ar_code} for the ${z_mantle} mantle"
      ;;
  esac

  buc_success "Donned the ${z_mantle} mantle, minted a token (${#z_mantle_token} chars), and reached Artifact Registry — the attributed use-hop audit entry is written; read it with rbw-da"
}

######################################################################
# Furnish and Main

zrbgv_furnish() {
  local -r z_command="${1:-}"

  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env "BUZ_FOLIO             " "Mantle to don (rbgv_check_mantle only): governor | director | retriever"
  buc_doc_env_done || return 0

  local z_rbk="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbk}/rbrr_regime.sh"
  source "${z_rbk}/rbrd_regime.sh"
  source "${z_rbk}/rbrp_regime.sh"
  source "${z_rbk}/rbrf_regime.sh"
  source "${z_rbk}/rbcc_constants.sh"
  source "${z_rbk}/rbpc_constants.sh"
  source "${z_rbk}/rbgc_constants.sh"
  source "${z_rbk}/rbdc_derived.sh"
  source "${z_rbk}/rbgo_oauth.sh"
  source "${z_rbk}/rbuh_http.sh"
  source "${z_rbk}/rbge_rest.sh"
  source "${z_rbk}/rba_auth.sh"
  source "${z_rbk}/rbgi_iam.sh"
  source "${z_rbk}/rbgp_payor.sh"
  source "${z_rbk}/rbgv_probe.sh"

  zbuv_kindle
  zburd_kindle

  source "${RBCC_rbrr_file}" || buc_die "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die "Failed to source RBRD: ${RBCC_rbrd_file}"
  # RBRP values load here too so zrbgp_kindle can derive RBGP_TERRIER_BUCKET from
  # RBRP_PAYOR_PROJECT_ID; rbrp's kindle+enforce stay per-probe (rbgv_check_payor),
  # since the payor probe enforces RBRP while the depot probes enforce RBRR/RBRD.
  source "${RBCC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBCC_rbrp_file}"
  zrbrr_kindle
  zrbrd_kindle

  # Payor probe is depot-agnostic: skip RBRR enforcement so it runs against
  # blank-template RBRR. zrbdc_kindle still runs to derive RBDC_PAYOR_RBRO_FILE
  # (credential path needed by the probe); depot-identity RBDC_* values it
  # also composes are unread on the Payor path. Mirrors BBAAS pattern in
  # rbgp_cli.sh:56-60 for rbgp_depot_list.
  if test "${z_command}" != "rbgv_check_payor" && test "${z_command}" != "rbgv_check_compearance"; then
    zrbrr_enforce
    zrbrd_enforce
  fi
  zrbcc_kindle
  zrbdc_kindle
  zrbgc_kindle
  zrbgo_kindle
  zrbuh_kindle
  zrbge_kindle
  zrba_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbgv_kindle
}

buc_execute rbgv_ "Recipe Bottle Access Probe" zrbgv_furnish "$@"

# eof
