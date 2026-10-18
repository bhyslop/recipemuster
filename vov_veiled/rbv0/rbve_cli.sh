#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVE CLI - the hierophant's thin bash front. Builds the veiled rbv crate
# and runs its subcommands. All ceremony logic is Rust (RBSHC "as the theurge
# conducts the tests"); this file only builds and launches the binary.
#
# VEILED, withheld from delivery. Dispatched by the hierophant's own veiled
# workbench (rbvw_workbench.sh) via its own zipper (rbvz_zipper.sh) — never the
# shipped rbw workbench, so no shipped manifest references the veiled crate.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# Build helper

# The hierophant's two collars (rbmm_moorings/bkd_weaned/). Every cargo
# invocation is the kennel's, through the door each collar is handed to: the
# app converged by the heel, the suite launched by the mush. No codegen: essai
# reaches its workers by tabtarget, needing no generated colophon constants.
readonly ZRBVE_APP_COLLAR="rbthdn_app"
readonly ZRBVE_SUITE_COLLAR="rbthdn_suite"

# The kennel stands and is current, or refuse naming the whistle. THE KENNEL
# BINARY IS DRIVEN, never a nested kennel tabtarget, so this dispatch stays the
# one dispatch; REPORT AND REFUSE, NEVER CONVERGE, the whistle being the one door
# that rebuilds the kennel. The door law stands first, since a kennel refusing a
# dirty tree states no seat and would read as outrun.
z_rbve_kennel=""
zrbve_kennel_or_die() {
  local -r z_repo="${PWD}"
  local -r z_crate="${z_repo}/${BURD_TOOLS_DIR}/bkk/bk0"
  local -r z_election="${z_crate}/bkce_roots.txt"
  local -r z_whistle=("${BURD_TABTARGET_DIR}/bkw-w."*)
  z_rbve_kennel="${z_crate}/target/release/bkx"

  bkcp_door_law_or_die "${z_repo}"

  test -x "${z_rbve_kennel}" || buc_die_now \
    "No kennel stands at ${z_rbve_kennel}, and every cargo invocation of this crate passes through it. The converge, run deliberately: ${z_whistle[0]}"

  local z_seat=""
  z_seat=$(bkcp_position_capture "${z_repo}" "${z_election}") \
    || buc_die_now "Could not read the seat's position over ${z_election}"

  local z_standing=""
  z_standing=$(bkcp_standing_seat_capture "${z_rbve_kennel}") \
    || buc_die_now "Could not ask the kennel at ${z_rbve_kennel} where it was struck"

  test "${z_standing}" = "${z_seat}" || buc_die_now \
    "The seat has outrun the kennel: the binary at ${z_rbve_kennel} stands at ${z_standing:-no position it could state}, the seat at ${z_seat}. The converge, run deliberately: ${z_whistle[0]}"

  bkcb_ensure
}

# Converge the app collar through the kennel's heel, and resolve the binary at
# the residence the collar declares.
z_rbve_binary=""
zrbve_heel() {
  local -r z_dir="${BASH_SOURCE[0]%/*}"
  zrbve_kennel_or_die
  buc_step "Building hierophant"
  "${z_rbve_kennel}" heel "${ZRBVE_APP_COLLAR}" || buc_die_now "The kennel's heel of ${ZRBVE_APP_COLLAR} failed"
  z_rbve_binary="${z_dir}/target/debug/rbv"
  test -x "${z_rbve_binary}" || buc_die_now "Hierophant binary not found: ${z_rbve_binary}"
}

######################################################################
# Commands

rbve_build() {
  buc_doc_brief "Build the hierophant crate"
  buc_doc_shown || return 0

  zrbve_heel
  buc_success "Hierophant built"
}

rbve_essai() {
  buc_doc_brief "Essai — the reversible repair lap (gate, cut, prove, rig; zero remote acts)"
  buc_doc_shown || return 0

  zrbve_heel
  buc_step "Conducting essai"
  "${z_rbve_binary}" essai
}

rbve_docimasy() {
  buc_doc_brief "Docimasy — the reveal's reversible proving act (narthex gate, freshness, preview, credential preflight, gauntlet; grants the cachet)"
  buc_doc_oparm "rehearse" "Prove the reversible stages only: skip credential preflight, the gauntlet, and the cachet grant"
  buc_doc_shown || return 0

  zrbve_heel
  buc_step "Conducting docimasy"
  if [ -n "${BUZ_FOLIO:-}" ]; then
    "${z_rbve_binary}" docimasy "${BUZ_FOLIO}"
  else
    "${z_rbve_binary}" docimasy
  fi
}

rbve_ostend() {
  buc_doc_brief "Ostend — the reveal's irreversible showing (re-asserted ground, file-list eyes, kerygma, promotion)"
  buc_doc_oparm "rehearse" "Prove the reversible stages only: stop before the kerygma line"
  buc_doc_shown || return 0

  zrbve_heel
  buc_step "Conducting ostend"
  if [ -n "${BUZ_FOLIO:-}" ]; then
    "${z_rbve_binary}" ostend "${BUZ_FOLIO}"
  else
    "${z_rbve_binary}" ostend
  fi
}

rbve_harbinger() {
  buc_doc_brief "Harbinger — the stranger rig against promoted public main (zero remote acts)"
  buc_doc_shown || return 0

  zrbve_heel
  buc_step "Conducting harbinger"
  "${z_rbve_binary}" harbinger
}

rbve_test() {
  buc_doc_brief "Run the hierophant crate tests — the cut's self-proofs (totality, planted-leak sweep)"
  buc_doc_shown || return 0

  zrbve_kennel_or_die
  buc_step "Testing hierophant"
  "${z_rbve_kennel}" mush "${ZRBVE_SUITE_COLLAR}" || buc_die_now "The kennel's mush of ${ZRBVE_SUITE_COLLAR} failed"
  buc_success "Hierophant tests green"
}

######################################################################
# Furnish and Main

zrbve_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temp directory (dispatch-provided)"
  buc_doc_env_row "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided; the kennel's home)"
  buc_doc_env_row "BURD_TACKROOM         " "The station's shared toolchain and registry store (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcp_position.sh"
  source "${BURD_TOOLS_DIR}/bkk/bk0/bkcb_tackroom.sh"
  zbkcb_kindle
}

buc_execute rbve_ "Hierophant ceremony conductor" zrbve_furnish "$@"

# eof
