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
# JJSL CLI - the shell face of the JJ operator doors over vvx: the dispatch
# stiles (saddle, lunge) plus the lock-hygiene (sight, cashier) and studbook
# founding (found) ceremonies.
#
# The operator-facing doors are the jjy_ trampolines this CLI installs into the
# infield (jjy_saddle, jjy_lunge). Each trampoline captures the operator's cwd
# into JJSL_INVOKE_DIR (cwd elects the clone; BUK dispatch then self-anchors to
# the kit repo root, losing it) and sets BURD_NO_LOG so the launched Claude
# session owns the terminal - an interactive TUI cannot run under the log tee.
# The spine itself is Rust: vvx jjx_dispatch (jjrds_spine.rs).

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

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

# Sight every JJ blotter lock and report — read-only, always safe (JJSVD
# jjdd_cashier, sight-and-report mode). Breaks nothing; an agent or a script may
# run this freely.
jjsl_sight() {
  zjjsl_sentinel
  "${ZJJSL_VVX}" jjx_cashier --cwd "$(zjjsl_invoke_dir)"
}

# Cashier a derelict lock-holder (JJSVD jjdd_cashier, break mode). The confirm
# gate is THIS door's contract, not the break sequence's: the sequence is
# mechanism, the deliberateness is the door's. So the report is shown first, and
# the operator answers before anything is plucked.
jjsl_cashier() {
  zjjsl_sentinel
  local -r z_cwd="$(zjjsl_invoke_dir)"

  # The report the operator judges from - the Rust verb owns its format.
  "${ZJJSL_VVX}" jjx_cashier --cwd "${z_cwd}"

  buc_require "About to CASHIER the locks reported above - dismissing whoever holds them." "cashier"

  "${ZJJSL_VVX}" jjx_cashier --cwd "${z_cwd}" --break
}

# Muck: destroy a named billet's destroy arm (JJSVD jjdd_muck). Plan-then-confirm:
# the report is shown first, the operator answers, then the destroy runs — the
# constellation's one deliberate data-loss surface.
jjsl_muck() {
  zjjsl_sentinel
  zjjsl_muck_confirmed ""
}

# Muck's salvage-then-destroy arm: open only on a dirty pace billet (the plan's
# own report names which arms are open) — lodges the non-JJ-owned dirty paths
# onto the pace's own seated branch, consigns, then destroys.
jjsl_muck_salvage() {
  zjjsl_sentinel
  zjjsl_muck_confirmed "--salvage"
}

# Shared plan-then-confirm body for both muck verbs. The confirm gate is THIS
# door's contract, not the reap's: the report is shown, the operator answers,
# and only then does the SAME resolution get destroyed.
zjjsl_muck_confirmed() {
  local -r z_arm_flag="${1}"
  local -r z_cwd="$(zjjsl_invoke_dir)"
  local -r z_target="${BUZ_FOLIO:-}"
  test -n "${z_target}" || buc_die "jjsl muck: no target supplied (a yard dirname, or the coronet/firemark identity behind one)"

  # The report the operator judges from - the Rust door owns its format.
  "${ZJJSL_VVX}" jjx_muck --cwd "${z_cwd}" --target "${z_target}"

  buc_require "About to MUCK the billet reported above - deliberate data loss, dismissing whatever it carries." "muck"

  if [ -z "${z_arm_flag}" ]; then
    "${ZJJSL_VVX}" jjx_muck --cwd "${z_cwd}" --target "${z_target}" --execute
  else
    "${ZJJSL_VVX}" jjx_muck --cwd "${z_cwd}" --target "${z_target}" --execute "${z_arm_flag}"
  fi
}

# Found the studbook from nothing (JJSAS Founding-and-cutover, jjdb_found_studbook).
# An irreversible ceremony — a genesis commit pushed to the studbook remote — so
# the confirm gate is THIS door's contract, mirroring cashier: the resolved plan
# is shown first (the door's own dry run), the operator answers, then the real
# found runs. The invocation cwd elects the hippodrome (a bare tabtarget run
# self-anchors to the kit repo, itself a legitimate hippodrome).
jjsl_found() {
  zjjsl_sentinel
  local -r z_cwd="$(zjjsl_invoke_dir)"

  # The plan the operator judges from - the Rust door owns its format.
  "${ZJJSL_VVX}" jjx_found --cwd "${z_cwd}" --dry-run

  buc_require "About to FOUND the studbook shown above - a genesis commit pushed to its remote, irreversible." "found"

  "${ZJJSL_VVX}" jjx_found --cwd "${z_cwd}"
}

# The directory that elects the clone. A trampoline captures the operator's cwd;
# a bare tabtarget run has already lost it to BUK's self-anchoring, and there PWD
# is the kit repo itself - which is a legitimate hippodrome, so this door works
# either way rather than refusing (unlike the dispatch doors, which need the
# operator's actual clone).
zjjsl_invoke_dir() {
  printf '%s' "${JJSL_INVOKE_DIR:-${PWD}}"
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

# The lock doors (sight, cashier) sweep an engine-known roster and take no
# folio, so asserting one for them would open the very report an operator reads
# at their worst moment with a spurious warning.
zjjsl_furnish() {
  local -r z_command="${1:-}"

  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  case "${z_command}" in
    jjsl_sight|jjsl_cashier|jjsl_found) ;;
    *) buc_doc_env "BUZ_FOLIO             " "Dispatch target or infield directory (param1 channel)" ;;
  esac
  buc_doc_env_done || return 0

  zjjsl_kindle
}

buc_execute jjsl_ "JJ Operator Doors" zjjsl_furnish "$@"

# eof
