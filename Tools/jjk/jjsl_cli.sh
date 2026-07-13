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
# JJSL CLI - Dispatch stiles: the shell face of the JJ dispatch doors.
#
# The operator-facing doors are the jjy_ trampolines this CLI installs into the
# infield (jjy_saddle, jjy_lunge). Each trampoline captures the operator's cwd
# into JJSL_INVOKE_DIR (cwd elects the clone; BUK dispatch then self-anchors to
# the kit repo root, losing it) and sets BURD_NO_LOG so the launched Claude
# session owns the terminal - an interactive TUI cannot run under the log tee.
# The spine itself is Rust: vvx jjx_dispatch (jjrds_spine.rs).

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Internal

zjjsl_kindle() {
  test -z "${ZJJSL_KINDLED:-}" || buc_die "jjsl already kindled"
  ZJJSL_VVX="${PWD}/Tools/vvk/bin/vvx"
  test -x "${ZJJSL_VVX}" || buc_die "vvx binary not found at ${ZJJSL_VVX} - build first (tt/vow-b.Build.sh)"
  readonly ZJJSL_VVX
  readonly ZJJSL_KINDLED=1
}

zjjsl_sentinel() {
  test "${ZJJSL_KINDLED:-}" = "1" || buc_die "Module jjsl not kindled"
}

# Route one door through the Rust spine. The invocation directory must have
# been captured by a trampoline - a bare tabtarget run has already lost the
# operator's cwd to BUK's self-anchoring, so it refuses with advice.
zjjsl_door() {
  zjjsl_sentinel
  local -r z_door="${1}"
  local -r z_target="${BUZ_FOLIO:-}"
  test -n "${z_target}" || buc_die "jjsl ${z_door}: no target supplied (coronet or firemark)"
  local -r z_invoke_dir="${JJSL_INVOKE_DIR:-}"
  test -n "${z_invoke_dir}" || buc_die "jjsl ${z_door}: JJSL_INVOKE_DIR unset - the doors are the jjy_ trampolines; run jjy_${z_door} from within the elected clone"
  exec "${ZJJSL_VVX}" jjx_dispatch --door "${z_door}" --target "${z_target}" --cwd "${z_invoke_dir}" --kit-root "${PWD}"
}

# Write one trampoline: a two-line POSIX sh script (never a symlink - Windows
# stations), the kit repo's tabtarget path baked absolute. Overwrite is the
# idempotence: re-running always converges on the current kit location.
zjjsl_stamp() {
  local -r z_path="${1}"
  local -r z_tabtarget="${2}"
  printf '#!/bin/sh\nJJSL_INVOKE_DIR="$PWD" BURD_NO_LOG=1 exec "%s" "$@"\n' "${z_tabtarget}" > "${z_path}"
  chmod 755 "${z_path}"
  buc_step "stamped ${z_path}"
}

######################################################################
# Commands

jjsl_saddle() {
  zjjsl_door "saddle"
}

jjsl_lunge() {
  zjjsl_door "lunge"
}

jjsl_install() {
  zjjsl_sentinel
  local -r z_infield="${BUZ_FOLIO:-}"
  test -n "${z_infield}" || buc_die "jjsl_install: no infield directory supplied"
  test -d "${z_infield}" || buc_die "jjsl_install: infield directory not found: ${z_infield}"
  zjjsl_stamp "${z_infield}/jjy_saddle" "${PWD}/tt/jjw-ds.Saddle.sh"
  zjjsl_stamp "${z_infield}/jjy_lunge"  "${PWD}/tt/jjw-dl.Lunge.sh"
  buc_success "stiles installed into ${z_infield}"
}

######################################################################
# Furnish and Main

zjjsl_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BUZ_FOLIO             " "Dispatch target or infield directory (param1 channel)"
  buc_doc_env_done || return 0

  zjjsl_kindle
}

buc_execute jjsl_ "Dispatch Stiles" zjjsl_furnish "$@"

# eof
