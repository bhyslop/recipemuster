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
# Recipe Bottle Foundry Ledger - inventory cluster (guard-free, sourced by rbflk_):
# tally hallmark health, rekon a hallmark/reliquary subtree, and audit hallmarks
# and reliquaries (Retriever for tally; Director for the rest).

set -euo pipefail

######################################################################
# Inventory (rbfl_*)

rbfl_tally() {
  zrbfl_sentinel

  buc_doc_brief "Tally hallmarks with health status (vouched / pending / incomplete)"
  buc_doc_shown || return 0

  buc_step "Authenticating as Retriever"
  test -f "${RBDC_RETRIEVER_RBRA_FILE}" \
    || buc_die "Retriever credential not found: ${RBDC_RETRIEVER_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_RETRIEVER_RBRA_FILE}") \
    || buc_die "Failed to get Retriever OAuth token"

  buc_step "Enumerating hallmarks under ${RBGL_HALLMARKS_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No hallmarks found under ${RBGL_HALLMARKS_ROOT}/"
    buc_success "Tally complete — 0 hallmarks"
    return 0
  fi

  # Load-then-iterate. A synthetic sentinel element appended to the array
  # lets the final hallmark flush through the same boundary branch as every
  # intermediate one (single flush site).
  local z_lines=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${ZRBFC_PACKAGE_LIST_FILE}"
  z_lines+=("__SENTINEL__ __SENTINEL__")

  echo ""
  printf "  %-30s  %-11s  %s\n" "HALLMARK" "HEALTH" "BASENAMES"
  printf "  %-30s  %-11s  %s\n" "------------------------------" "-----------" "---------"

  # State machine over <hallmark> <basename> pairs (file was sorted by the
  # capture helper). Vessel is no longer encoded in the GAR path —
  # restoration via about/vouch metadata is AAL territory.
  local z_prev_h="" z_prev_bns=""
  local z_prev_img=0 z_prev_abt=0 z_prev_vch=0
  local z_count=0 z_vouched_n=0 z_pending_n=0 z_incomplete_n=0
  local z_i="" z_h="" z_b="" z_health=""

  for z_i in "${!z_lines[@]}"; do
    z_line="${z_lines[$z_i]}"
    test -n "${z_line}" || continue

    z_h="${z_line%% *}"
    z_b="${z_line#* }"
    test -n "${z_h}" || continue
    test -n "${z_b}" || continue

    if test "${z_h}" != "${z_prev_h}"; then
      if test -n "${z_prev_h}"; then
        if test "${z_prev_img}" = "1" \
          && test "${z_prev_abt}" = "1" \
          && test "${z_prev_vch}" = "1"; then
          z_health="vouched"
          z_vouched_n=$(( z_vouched_n + 1 ))
        elif test "${z_prev_img}" = "1" \
          && test "${z_prev_abt}" = "1"; then
          z_health="pending"
          z_pending_n=$(( z_pending_n + 1 ))
        else
          z_health="incomplete"
          z_incomplete_n=$(( z_incomplete_n + 1 ))
        fi
        printf "  %-30s  %-11s  %s\n" "${z_prev_h}" "${z_health}" "${z_prev_bns}"
        z_count=$(( z_count + 1 ))
      fi

      case "${z_h}" in
        __SENTINEL__) break ;;
      esac

      z_prev_h="${z_h}"
      z_prev_bns=""
      z_prev_img=0
      z_prev_abt=0
      z_prev_vch=0
    fi

    z_prev_bns="${z_prev_bns}${z_prev_bns:+ }${z_b}"
    case "${z_b}" in
      "${RBGC_ARK_BASENAME_IMAGE}") z_prev_img=1 ;;
      "${RBGC_ARK_BASENAME_ABOUT}") z_prev_abt=1 ;;
      "${RBGC_ARK_BASENAME_VOUCH}") z_prev_vch=1 ;;
    esac
  done

  echo ""
  buc_info "Total hallmarks: ${z_count}  (vouched: ${z_vouched_n}, pending: ${z_pending_n}, incomplete: ${z_incomplete_n})"

  case "${z_pending_n}" in
    0) ;;
    *) buc_info "To vouch pending hallmarks:"
       buc_tabtarget "rbw-fV"
       ;;
  esac

  case "${z_incomplete_n}" in
    0) ;;
    *) buc_info "To abjure incomplete hallmarks:"
       buc_tabtarget "rbw-fA"
       ;;
  esac

  buc_success "Tally complete"
}

rbfl_rekon_hallmark() {
  zrbfl_sentinel

  local -r z_hallmark="${BUZ_FOLIO:-}"

  buc_doc_brief "List ark basenames present under a hallmark's GAR subtree"
  buc_doc_param "hallmark" "Hallmark identifier"
  buc_doc_shown || return 0

  test -n "${z_hallmark}" || buc_die "Usage: rbw-irh <hallmark>"

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating arks under ${RBGL_HALLMARKS_ROOT}/${z_hallmark}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  # Filter the full hallmark enumeration to rows for this hallmark.
  local z_found=""
  local z_line=""
  local z_h=""
  local z_b=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_h="${z_line%% *}"
    z_b="${z_line#* }"
    if test "${z_h}" = "${z_hallmark}"; then
      z_found="${z_found}${z_found:+ }${z_b}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  test -n "${z_found}" || buc_die "Hallmark not found: ${z_hallmark}"

  echo ""
  printf "  %-10s  %-6s  %s\n" "BASENAME" "EXISTS" "PACKAGE-PATH"
  printf "  %-10s  %-6s  %s\n" "----------" "------" "------------"

  local z_canon=""
  local z_mark=""
  local z_path=""
  for z_canon in \
    "${RBGC_ARK_BASENAME_IMAGE}" \
    "${RBGC_ARK_BASENAME_ABOUT}" \
    "${RBGC_ARK_BASENAME_VOUCH}" \
    "${RBGC_ARK_BASENAME_ATTEST}" \
    "${RBGC_ARK_BASENAME_POUCH}" \
    "${RBGC_ARK_BASENAME_DIAGS}"; do
    z_mark="no"
    case " ${z_found} " in
      *" ${z_canon} "*) z_mark="yes" ;;
    esac
    if test "${z_mark}" = "yes"; then
      z_path="${RBGL_HALLMARKS_ROOT}/${z_hallmark}/${z_canon}"
    else
      z_path="(absent)"
    fi
    printf "  %-10s  %-6s  %s\n" "${z_canon}" "${z_mark}" "${z_path}"
  done

  echo ""
  buc_success "Rekon complete for ${z_hallmark}"
}

rbfl_rekon_reliquary() {
  zrbfl_sentinel

  local -r z_stamp="${BUZ_FOLIO:-}"

  buc_doc_brief "List tool images present under a reliquary stamp's GAR subtree"
  buc_doc_param "stamp" "Reliquary datestamp (e.g., r260327172456)"
  buc_doc_shown || return 0

  test -n "${z_stamp}" || buc_die "Usage: rbw-irr <stamp>"

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating tool images under ${RBGL_RELIQUARIES_ROOT}/${z_stamp}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_RELIQUARIES_ROOT}"

  # Filter the full reliquary enumeration to rows for this stamp.
  local z_found=""
  local z_line=""
  local z_s=""
  local z_t=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_s="${z_line%% *}"
    z_t="${z_line#* }"
    if test "${z_s}" = "${z_stamp}"; then
      z_found="${z_found}${z_found:+ }${z_t}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  test -n "${z_found}" || buc_die "Reliquary stamp not found: ${z_stamp}"

  echo ""
  printf "  %-10s  %-6s  %s\n" "TOOL" "EXISTS" "PACKAGE-PATH"
  printf "  %-10s  %-6s  %s\n" "----------" "------" "------------"

  local z_canon=""
  local z_mark=""
  local z_path=""
  for z_canon in \
    "${RBGC_RELIQUARY_TOOL_GCLOUD}" \
    "${RBGC_RELIQUARY_TOOL_DOCKER}" \
    "${RBGC_RELIQUARY_TOOL_ALPINE}" \
    "${RBGC_RELIQUARY_TOOL_SYFT}" \
    "${RBGC_RELIQUARY_TOOL_BINFMT}" \
    "${RBGC_RELIQUARY_TOOL_SKOPEO}"; do
    z_mark="no"
    case " ${z_found} " in
      *" ${z_canon} "*) z_mark="yes" ;;
    esac
    if test "${z_mark}" = "yes"; then
      z_path="${RBGL_RELIQUARIES_ROOT}/${z_stamp}/${z_canon}"
    else
      z_path="(absent)"
    fi
    printf "  %-10s  %-6s  %s\n" "${z_canon}" "${z_mark}" "${z_path}"
  done

  echo ""
  buc_success "Rekon complete for ${z_stamp}"
}

rbfl_audit_hallmarks() {
  zrbfl_sentinel

  buc_doc_brief "Audit hallmarks — list all hallmark identifiers in registry"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating hallmarks under ${RBGL_HALLMARKS_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_HALLMARKS_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No hallmarks found under ${RBGL_HALLMARKS_ROOT}/"
    buc_success "Audit complete — 0 hallmarks"
    return 0
  fi

  echo ""
  printf "  %s\n" "HALLMARK"
  printf "  %s\n" "------------------------------"

  local z_count=0
  local z_prev=""
  local z_line=""
  local z_h=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_h="${z_line%% *}"
    test -n "${z_h}" || continue
    if test "${z_h}" != "${z_prev}"; then
      printf "  %s\n" "${z_h}"
      buf_write_fact_multi "${z_h}" "${RBCC_fact_ext_audit_hallmark}" "${z_h}"
      z_count=$(( z_count + 1 ))
      z_prev="${z_h}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  echo ""
  buc_info "Total hallmarks: ${z_count}"
  buc_success "Audit complete"
}

rbfl_audit_reliquaries() {
  zrbfl_sentinel

  buc_doc_brief "Audit reliquaries — list all reliquary stamps in registry"
  buc_doc_shown || return 0

  buc_step "Authenticating as Director"
  test -f "${RBDC_DIRECTOR_RBRA_FILE}" \
    || buc_die "Director credential not found: ${RBDC_DIRECTOR_RBRA_FILE}"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get Director OAuth token"

  buc_step "Enumerating reliquaries under ${RBGL_RELIQUARIES_ROOT}/"
  zrbfc_list_packages_capture "${z_token}" "${RBGL_RELIQUARIES_ROOT}"

  if ! test -s "${ZRBFC_PACKAGE_LIST_FILE}"; then
    buc_info "No reliquaries found under ${RBGL_RELIQUARIES_ROOT}/"
    buc_success "Audit complete — 0 reliquaries"
    return 0
  fi

  echo ""
  printf "  %s\n" "RELIQUARY-STAMP"
  printf "  %s\n" "------------------------------"

  local z_count=0
  local z_prev=""
  local z_line=""
  local z_s=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_s="${z_line%% *}"
    test -n "${z_s}" || continue
    if test "${z_s}" != "${z_prev}"; then
      printf "  %s\n" "${z_s}"
      z_count=$(( z_count + 1 ))
      z_prev="${z_s}"
    fi
  done < "${ZRBFC_PACKAGE_LIST_FILE}"

  echo ""
  buc_info "Total reliquaries: ${z_count}"
  buc_success "Audit complete"
}

# eof
