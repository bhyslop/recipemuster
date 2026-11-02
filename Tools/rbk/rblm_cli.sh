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
# RBLM CLI - Lifecycle Marshal operations (zero, feign, expede, harbinger)

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || exit 1
source "${BASH_SOURCE[0]%/*}/rbcg_breviary.sh" || buc_die_now "Failed to source rbcg_breviary.sh"

# Command: zero - Zero regime to blank template for release

rblm_zero() {
  buc_doc_brief "Zero regime to blank template for release qualification"
  buc_doc_param "tree" "Basename of the repository root this call intends to blank"
  buc_doc_shown || return 0

  local -r z_rbrr="${RBCC_rbrr_file}"
  local -r z_rbrd="${RBCC_rbrd_file}"
  test -f "${z_rbrr}" || buc_die_now "RBRR file not found: ${z_rbrr}"
  test -f "${z_rbrd}" || buc_die_now "RBRD file not found: ${z_rbrd}"

  mkdir -p "${BURD_TEMP_DIR}" || buc_die_now "Failed to create temp directory"

  # Tree-identity gate. Zero blanks the regime of whatever tree it runs in, so
  # the caller must NAME that tree and be right. Git attests the identity; the
  # caller's claim is checked against it. This refuses before buc_require, so
  # BURE_CONFIRM=skip cannot reach past it — the guard is the naming, never the
  # prompt. Both refusals are the marshal gate's.
  local -r z_claimed_tree="${BUZ_FOLIO:-}"
  test -n "${z_claimed_tree}" \
    || buc_reject "${RBCG_MARSHAL_DENTIL}" "Marshal zero requires the intended tree's basename as its argument — it blanks the regime of the tree it runs in, so that tree must be named, not assumed"

  local -r z_toplevel_temp="${BURD_TEMP_DIR}/rblm_zero_toplevel.txt"
  git rev-parse --show-toplevel > "${z_toplevel_temp}" || buc_die_now "git rev-parse --show-toplevel failed — marshal zero must run inside a git repository"
  local z_toplevel
  z_toplevel=$(<"${z_toplevel_temp}") || buc_die_now "Failed to read: ${z_toplevel_temp}"
  local -r z_actual_tree="${z_toplevel##*/}"
  test "${z_claimed_tree}" = "${z_actual_tree}" \
    || buc_reject "${RBCG_MARSHAL_DENTIL}" "Marshal zero refuses: caller named tree '${z_claimed_tree}', but this repository root is '${z_actual_tree}' (${z_toplevel})"

  # The two directories zero reads are bash's own reading of the repo regime,
  # sourced, kindled and enforced in an isolation subshell so its locked fields
  # never reach this shell. A vessel or secrets directory that does not stand
  # dies there, at the regime's own enforce and under its own code.
  local -r z_rbk_dir="${BASH_SOURCE[0]%/*}"
  local -r z_vessel_temp="${BURD_TEMP_DIR}/rblm_zero_vessel_dir.txt"
  local -r z_secrets_temp="${BURD_TEMP_DIR}/rblm_zero_secrets_dir.txt"
  (
    source "${z_rbk_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
    source "${z_rbrr}"                   || buc_die_now "Failed to source RBRR: ${z_rbrr}"
    zrbrr_kindle
    zrbrr_enforce
    printf '%s' "${RBRR_VESSEL_DIR}"  > "${z_vessel_temp}"  || buc_die_now "Failed to write: ${z_vessel_temp}"
    printf '%s' "${RBRR_SECRETS_DIR}" > "${z_secrets_temp}" || buc_die_now "Failed to write: ${z_secrets_temp}"
  ) || buc_die_now "Marshal zero could not read the repo regime: ${z_rbrr}"
  local z_vessel_dir
  z_vessel_dir=$(<"${z_vessel_temp}") || buc_die_now "Failed to read: ${z_vessel_temp}"
  local z_secrets_dir
  z_secrets_dir=$(<"${z_secrets_temp}") || buc_die_now "Failed to read: ${z_secrets_temp}"

  # Pre-checks: working tree clean and HEAD pushed.
  # Marshal-zero auto-commits its mutations; both invariants must hold to keep
  # the resulting commit purely the marshal-zero state.
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_zero_status.txt"
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_zero_upstream.txt"
  local -r z_unpushed_temp="${BURD_TEMP_DIR}/rblm_zero_unpushed.txt"

  git status --porcelain > "${z_status_temp}" || buc_die_now "git status failed"
  test ! -s "${z_status_temp}" || buc_die_now "Working tree not clean — commit or discard before marshal-zero. See: ${z_status_temp}"

  if ! git rev-parse --abbrev-ref --symbolic-full-name '@{u}' > "${z_upstream_temp}" 2>&1; then
    buc_die_now "Current branch has no upstream — set tracking and push before marshal-zero. See: ${z_upstream_temp}"
  fi

  git rev-list '@{u}..HEAD' > "${z_unpushed_temp}" || buc_die_now "git rev-list failed"
  test ! -s "${z_unpushed_temp}" || buc_die_now "HEAD has unpushed commits — push before marshal-zero. See: ${z_unpushed_temp}"

  # Shellcheck gate: the codebase must be lint-clean before marshal-zero mints
  # the pristine baseline commit, so gauntlet entry-state cannot be shellcheck-
  # dirty by construction. Runs the release-tier shellcheck via the sibling
  # rbq_cli; any finding aborts here, before any mutation.
  "${z_rbk_dir}/rbq_cli.sh" rbq_shellcheck || buc_die_now "Shellcheck findings present — fix before marshal-zero; the pristine baseline must be lint-clean"

  # Colophon-completeness gate: every enrolled colophon (RBW + BUW) must have a
  # tabtarget on disk before marshal-zero mints the pristine baseline. rbw-MZ is
  # withheld from delivery, so this is the source-only completeness proof — a
  # stripped consumer never has this tabtarget and never runs it.
  "${z_rbk_dir}/rbq_cli.sh" rbq_completeness || buc_die_now "Enrolled colophon without a tabtarget — the source tabtarget set must be complete before marshal-zero"

  buh_section "Marshal Zero"
  buh_line "  Tree:    ${z_actual_tree} (${z_toplevel})"
  buh_line "  Targets: ${z_rbrr}"
  buh_line "           ${z_rbrd}"
  buh_e
  buh_line "  RBRR fields blanked (zeroed to onboarding start):"
  buh_line "    RBRR_RUNTIME_PREFIX, RBRR_SUBSTRATE_RELIQUARY"
  buh_e
  buh_line "  RBRR fields pre-filled to defaults:"
  buh_line "    RBRR_DNS_SERVER, RBRR_GCB_TIMEOUT,"
  buh_line "    RBRR_GCB_MIN_CONCURRENT_BUILDS,"
  buh_line "    RBRR_VESSEL_DIR, RBRR_SECRETS_DIR"
  buh_e
  buh_line "  RBRD fields blanked (depot identity — operator decision):"
  buh_line "    RBRD_CLOUD_PREFIX, RBRD_DEPOT_MONIKER"
  buh_e
  buh_line "  RBRD fields pre-filled to defaults:"
  buh_line "    RBRD_GCP_REGION, RBRD_GCB_MACHINE_TYPE"
  buh_e
  buh_line "  Vessel hallmarks BLANKED (stale after depot change):"
  local z_np_preview=""
  local z_any_np=0
  for z_np_preview in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_np_preview}" || continue
    buh_line "    ${z_np_preview}"
    z_any_np=1
  done
  test "${z_any_np}" = "1" || buh_line "    (no nameplates found)"
  buh_e
  buh_line "  Vessel regime fields BLANKED (depot-scoped, stale after depot change):"
  buh_line "    RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR in all rbrv.env"
  local z_vr_preview=""
  local z_any_vr=0
  for z_vr_preview in "${z_vessel_dir}"/*/"${RBCC_rbrv_file}"; do
    test -f "${z_vr_preview}" || continue
    buh_line "    ${z_vr_preview}"
    z_any_vr=1
  done
  test "${z_any_vr}" = "1" || buh_line "    (no vessel regimes found)"
  buh_e
  buh_line "  Preserved (payor-scoped, survives depot change):"
  buh_line "    ${z_secrets_dir}/${RBCC_account_unhewn_payor}/${RBCC_rbro_file}"
  buh_e
  buh_line "  On completion, marshal-zero auto-commits the in-tree mutations."
  buh_e
  buc_require "Proceed with marshal zero?" "zero"

  # Every rewrite is written to its temp first, outside the tree, and every
  # fixed field it meets is counted. A fixed field never found dies before any
  # file is replaced, so a line the patterns miss can never pass through into a
  # commit that reads complete. The image-anchor family takes no count.
  local -a z_temps=()
  local -a z_targets=()
  local z_line=""
  local z_out=""
  local z_counted=0
  local z_found=""

  local -r z_rbrr_tmp="${BURD_TEMP_DIR}/rblm_zero_rbrr.env"
  z_found=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_counted=1
    case "${z_line}" in
      # Pre-selected defaults
      RBRR_DNS_SERVER=*)                    z_out="RBRR_DNS_SERVER=8.8.8.8"                     ;;
      RBRR_GCB_TIMEOUT=*)                   z_out="RBRR_GCB_TIMEOUT=2700s"                      ;;
      RBRR_GCB_MIN_CONCURRENT_BUILDS=*)     z_out="RBRR_GCB_MIN_CONCURRENT_BUILDS=3"            ;;
      RBRR_VESSEL_DIR=*)                    z_out="RBRR_VESSEL_DIR=${RBCC_moorings_dir}/${RBCC_vessels_subdir}" ;;
      RBRR_SECRETS_DIR=*)                   z_out="RBRR_SECRETS_DIR=../station-files/secrets"   ;;
      # Site-specific fields blanked
      RBRR_RUNTIME_PREFIX=*)                z_out="RBRR_RUNTIME_PREFIX="                        ;;
      RBRR_SUBSTRATE_RELIQUARY=*)           z_out="RBRR_SUBSTRATE_RELIQUARY="                   ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    z_out="${z_line}"; z_counted=0                      ;;
    esac
    test "${z_counted}" = "0" || z_found="${z_found} ${z_line%%=*}"
    printf '%s\n' "${z_out}" || buc_die_now "Failed to write: ${z_rbrr_tmp}"
  done < "${z_rbrr}" > "${z_rbrr_tmp}" || buc_die_now "Failed to zero: ${z_rbrr}"
  zrblm_zero_require_found "${z_rbrr}" "${z_found}" \
    RBRR_DNS_SERVER RBRR_GCB_TIMEOUT RBRR_GCB_MIN_CONCURRENT_BUILDS \
    RBRR_VESSEL_DIR RBRR_SECRETS_DIR RBRR_RUNTIME_PREFIX RBRR_SUBSTRATE_RELIQUARY
  z_temps+=("${z_rbrr_tmp}")
  z_targets+=("${z_rbrr}")

  # Apply equivalent transform to rbrd.env.
  local -r z_rbrd_tmp="${BURD_TEMP_DIR}/rblm_zero_rbrd.env"
  z_found=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_counted=1
    case "${z_line}" in
      # Pre-selected defaults
      RBRD_GCP_REGION=*)                    z_out="RBRD_GCP_REGION=us-central1"                 ;;
      RBRD_GCB_MACHINE_TYPE=*)              z_out="RBRD_GCB_MACHINE_TYPE=e2-standard-2"         ;;
      # Site-specific fields blanked
      RBRD_CLOUD_PREFIX=*)                  z_out="RBRD_CLOUD_PREFIX="                          ;;
      RBRD_DEPOT_MONIKER=*)                 z_out="RBRD_DEPOT_MONIKER="                         ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    z_out="${z_line}"; z_counted=0                      ;;
    esac
    test "${z_counted}" = "0" || z_found="${z_found} ${z_line%%=*}"
    printf '%s\n' "${z_out}" || buc_die_now "Failed to write: ${z_rbrd_tmp}"
  done < "${z_rbrd}" > "${z_rbrd_tmp}" || buc_die_now "Failed to zero: ${z_rbrd}"
  zrblm_zero_require_found "${z_rbrd}" "${z_found}" \
    RBRD_GCP_REGION RBRD_GCB_MACHINE_TYPE RBRD_CLOUD_PREFIX RBRD_DEPOT_MONIKER
  z_temps+=("${z_rbrd_tmp}")
  z_targets+=("${z_rbrd}")

  # Blank hallmark values in all vessel nameplates.
  # Hallmarks reference images built against the prior depot — they
  # become stale after reset.  Blanking them causes the onboarding guide
  # to require conjure & vouch before declaring setup complete.
  local z_np=""
  local z_np_tmp=""
  local z_np_index=0
  for z_np in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_np}" || continue
    z_np_index=$((z_np_index + 1))
    z_np_tmp="${BURD_TEMP_DIR}/rblm_zero_rbrn_${z_np_index}.env"
    z_found=""
    while IFS= read -r z_line || test -n "${z_line}"; do
      z_counted=1
      case "${z_line}" in
        RBRN_SENTRY_HALLMARK=*)  z_out="RBRN_SENTRY_HALLMARK=" ;;
        RBRN_BOTTLE_HALLMARK=*)  z_out="RBRN_BOTTLE_HALLMARK=" ;;
        *)                       z_out="${z_line}"; z_counted=0 ;;
      esac
      test "${z_counted}" = "0" || z_found="${z_found} ${z_line%%=*}"
      printf '%s\n' "${z_out}" || buc_die_now "Failed to write: ${z_np_tmp}"
    done < "${z_np}" > "${z_np_tmp}" || buc_die_now "Failed to blank hallmarks: ${z_np}"
    zrblm_zero_require_found "${z_np}" "${z_found}" RBRN_SENTRY_HALLMARK RBRN_BOTTLE_HALLMARK
    z_temps+=("${z_np_tmp}")
    z_targets+=("${z_np}")
  done

  # Blank depot-scoped fields in all vessel regime files.
  # RBRV_RELIQUARY references a reliquary-kind Lode in the prior depot's GAR.
  # RBRV_IMAGE_*_ANCHOR references bole Lodes in the prior depot's GAR.
  # Both become stale after depot change — onboarding requires conclave + ensconce.
  local z_vr=""
  local z_vr_tmp=""
  local z_vr_index=0
  for z_vr in "${z_vessel_dir}"/*/"${RBCC_rbrv_file}"; do
    test -f "${z_vr}" || continue
    z_vr_index=$((z_vr_index + 1))
    z_vr_tmp="${BURD_TEMP_DIR}/rblm_zero_rbrv_${z_vr_index}.env"
    z_found=""
    while IFS= read -r z_line || test -n "${z_line}"; do
      z_counted=1
      case "${z_line}" in
        RBRV_RELIQUARY=*)       z_out="RBRV_RELIQUARY="                  ;;
        RBRV_IMAGE_*_ANCHOR=*)  z_out="${z_line%%=*}="; z_counted=0      ;;
        *)                      z_out="${z_line}"; z_counted=0           ;;
      esac
      test "${z_counted}" = "0" || z_found="${z_found} ${z_line%%=*}"
      printf '%s\n' "${z_out}" || buc_die_now "Failed to write: ${z_vr_tmp}"
    done < "${z_vr}" > "${z_vr_tmp}" || buc_die_now "Failed to blank depot-scoped fields: ${z_vr}"
    zrblm_zero_require_found "${z_vr}" "${z_found}" RBRV_RELIQUARY
    z_temps+=("${z_vr_tmp}")
    z_targets+=("${z_vr}")
  done

  # Every field was found: replace each target from its temp.
  local z_index=0
  for z_index in "${!z_targets[@]}"; do
    mv "${z_temps[${z_index}]}" "${z_targets[${z_index}]}" || buc_die_now "Failed to replace: ${z_targets[${z_index}]}"
    buh_line "  Zeroed: ${z_targets[${z_index}]}"
  done
  buh_e

  # Auto-commit the in-tree mutations so post-zero state is captured as a single
  # commit.
  buc_step "Committing marshal-zero state"
  git add "${z_targets[@]}" || buc_die_now "Failed to stage the zeroed files"

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already in marshal-zero state"
  else
    git commit -m "Marshal Zero — release qualification reset" || buc_die_now "Marshal-zero commit failed"
    buh_line "  Marshal-zero state committed"
  fi
  buh_e

  buh_line "  Next: verify onboarding guide detects blank state:"
  buc_tabtarget "${RBZ_ONBOARD_START_HERE}"
  buc_success "Regime zeroed to blank template"
}

# zrblm_zero_require_found FILE FOUND FIELD... — die naming the first FIELD the
# rewrite of FILE never met. FOUND is the space-separated list of field names the
# rewrite counted. Zero calls it before any file is replaced.
zrblm_zero_require_found() {
  local -r z_file="${1:-}"
  local -r z_found="${2:-}"
  test -n "${z_file}" || buc_die_now "zrblm_zero_require_found: the rewritten file is required"
  shift 2 || buc_die_now "zrblm_zero_require_found: the field list is required"
  local z_field=""
  for z_field in "$@"; do
    case " ${z_found} " in
      *" ${z_field} "*) ;;
      *) buc_die_now "Marshal zero found no ${z_field} line in ${z_file} — no file has been replaced" ;;
    esac
  done
}

######################################################################
# Command: feign - Invent a false station so the candidate can validate

rblm_feign() {
  buc_doc_brief "Feign a station on the probe branch - write shape-valid stand-ins over the lustrated site fields"
  buc_doc_shown || return 0

  # Marshal gate. Feigning writes false-but-valid identity into the regime tree
  # and commits it, so it runs only where that commit can never ship: on the
  # ceremony's probe branch, in a clone naming no remote. Both rules refuse
  # ahead of the clean-tree gate and the confirmation.
  mkdir -p "${BURD_TEMP_DIR}" || buc_die_now "Failed to create temp directory"
  local -r z_branch_temp="${BURD_TEMP_DIR}/rblm_feign_branch.txt"
  git rev-parse --abbrev-ref HEAD > "${z_branch_temp}" || buc_die_now "git rev-parse failed"
  local z_branch
  z_branch=$(<"${z_branch_temp}") || buc_die_now "Failed to read: ${z_branch_temp}"
  test "${z_branch}" = "${RBCC_probe_branch}" \
    || buc_reject "${RBCG_MARSHAL_DENTIL}" "Refusing to feign on branch '${z_branch}' — feigned values ride only the probe branch '${RBCC_probe_branch}'. Cut it from the candidate first."

  local -r z_remotes_temp="${BURD_TEMP_DIR}/rblm_feign_remotes.txt"
  git remote > "${z_remotes_temp}" || buc_die_now "git remote failed"
  local z_remotes
  z_remotes=$(<"${z_remotes_temp}") || buc_die_now "Failed to read: ${z_remotes_temp}"
  test -z "${z_remotes}" \
    || buc_reject "${RBCG_MARSHAL_DENTIL}" "Refusing to feign in a clone naming remote(s) ${z_remotes//$'\n'/ } — feigned values must have nowhere to be pushed"

  # Clean-tree gate. Feigning auto-commits, and the commit must carry the seed
  # alone — the probe branch exists to be the candidate plus exactly this.
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_feign_status.txt"
  git status --porcelain > "${z_status_temp}" || buc_die_now "git status failed"
  test ! -s "${z_status_temp}" || buc_die_now "Working tree not clean — commit or discard before feigning. See: ${z_status_temp}"

  buh_section "Marshal Feign"
  buh_line "  Branch: ${z_branch}"
  buh_e
  buh_line "  Writes a shape-valid stand-in over every site-scoped field the"
  buh_line "  proscription (Tools/rbk/rblm_proscription.sh) carries a feigned value"
  buh_line "  for. A lustrated tree is correctly sterile and therefore cannot"
  buh_line "  validate; this invents a false station so the candidate can run the"
  buh_line "  consumer's own reveille from the consumer's own seat."
  buh_e
  buh_line "  Every value is visibly false. None is borrowed from a live station."
  buh_e
  buh_line "  THIS BRANCH IS A THROWAWAY. It is never pushed, never merged, and"
  buh_line "  never becomes the candidate. Delete it when the probe is read."
  buh_e
  buc_require "Proceed with feigning?" "feign"

  rblm_feign_apply
  buh_e

  buc_step "Committing feigned state"
  git add --update || buc_die_now "Failed to stage feigned files"

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already feigned"
  else
    git commit -m "Marshal Feign — false station for the consumer-seat probe (throwaway)" || buc_die_now "Feign commit failed"
    buh_line "  Feigned state committed"
  fi
  buh_e

  buh_line "  Next: run the consumer's credless suite against this tree:"
  buc_tabtarget "${RBZ_THEURGE_SUITE}" "reveille"
  buc_success "Station feigned — the candidate can now validate"
}


######################################################################
# Furnish and Main

zrblm_furnish() {
  local z_command="${1:-}"

  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temporary directory for this invocation (dispatch-provided)"
  # BUZ_FOLIO (param1 channel) carries expede's target directory and zero's tree
  # identity, and is legitimately empty for the folioless verbs, so it is not a
  # buc_doc_env_row here: an empty doc_env var warns (rbgv_cli optional-folio
  # precedent).
  buc_doc_env_done || return 0

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${z_rbk_kit_dir}/rbcc_constants.sh"      || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rblm_proscription.sh"       || buc_die_now "Failed to source rblm_proscription.sh"

  # Differential furnish: a withheld verb's module is sourced only for that verb.
  # Harbinger is a release-rig verb whose module is withheld from delivery, so a
  # hard source here would leave every delivered marshal verb dangling on a module
  # the candidate does not carry. Keying on the COMMAND rather than on the file's
  # existence keeps the source unconditional given the verb — and in the delivered
  # tree, where the module is absent, an invoked verb dies naming what is missing
  # instead of furnishing cleanly and then failing on an absent function. The verb
  # is unreachable there in any case: tt/rbw-MH is withheld by the census (the
  # tt/rbw-M stem).
  case "${z_command}" in
    rblm_harbinger)
      source "${z_rbk_kit_dir}/rblm_harbinger.sh" || buc_die_now "Failed to source rblm_harbinger.sh"
      ;;
  esac

  source "${BURD_BUK_DIR}/buh_handbook.sh"      || buc_die_now "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"     || buc_die_now "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"    || buc_die_now "Failed to source rbz_zipper.sh"
  zbuz_kindle
  zrbz_kindle
  zrblm_kindle
}

buc_execute rblm_ "Lifecycle Marshal" zrblm_furnish "$@"

# eof
