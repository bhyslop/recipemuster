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
# BKCW Whistle - the thin bash bootstrap that builds the kennel and execs it
#
# THE ONE BASH THAT REMAINS, and it remains because the kennel cannot build
# itself: something outside the binary has to exist before the binary does. It
# binds the toolchain once, minimally, and grows nothing else. Every other door
# of this kit is a rust interior; this is the bootstrap the founding cinch
# carved out (BKSNC-Kennelcraft.adoc "The form of each launcher").
#
# ONE TABTARGET PER DOOR, AND THE DOOR WORD IS NEVER TYPED (operator, 260905).
# What stood here was a single tabtarget taking the door word as its argument,
# so an operator wanting `heel <collar>` typed the whistle, then the word, then
# the collar - a layer of indirection between the person and the door. Now each
# door wears its own tabtarget, the collar is its argument, and the function
# behind it supplies the word. The bash gained eight small functions and no
# logic: every one of them converges and execs, and which door is being reached
# is the only thing that differs between them.
#
# THE ONE DOOR THAT CONVERGES UNASKED. Every door the kennel exposes reports and
# refuses. This one rebuilds, because a launcher that execs a stale binary has
# lied about everything downstream of it, and no report it could print would
# repair that. The verify-only posture a routine invocation holds is not this
# door's: the whistle IS the converge, elected as such.
#
# WHY THE BOOTSTRAP SPELLS CARGO ITSELF, when the leash exists precisely so that
# nothing else does. The leash is compiled into the binary this script is trying
# to bring into being, so at this one call site it is not yet reachable — the
# bootstrap paradox in its plainest form. The duplication is therefore bounded to
# this file and to one invocation, and it is spelled to the same three
# disciplines the leash holds: the pin STATED rather than inherited, --locked on
# the invocation, and the tackroom fence stood on first.
#
# WHAT MAKES THE KENNEL STALE is the seat's position walked over the election,
# and this door no longer reads it. build.rs does, over the same election the
# crate reads, and tells cargo to watch it; the reasoning for that reading — why
# the seat's own line rather than the trunk counterpart, and why the election
# rather than bare HEAD — is homed there and at src/bkcf_guard.rs. What this
# door adds is what it DOES about staleness: it always runs cargo. Every other
# door takes its own reading and refuses instead.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKCW_SOURCED:-}" || return 0
ZBKCW_SOURCED=1

zbkcw_kindle() {
  test -z "${ZBKCW_KINDLED:-}" || buc_die_now "Module bkcw already kindled"

  test -n "${BURC_TOOLS_DIR:-}" || buc_die_now "BURC_TOOLS_DIR is unset"

  # z-launcher.sh normalizes cwd to the repo root before any workbench runs, so
  # PWD is the repository the kennel is being built in and driven against.
  readonly ZBKCW_REPO_ROOT="${PWD}"
  readonly ZBKCW_KIT_DIR="${ZBKCW_REPO_ROOT}/${BURC_TOOLS_DIR}/bkk"
  # The kit directory is a delivery name and constrains nothing beneath it. The
  # crate stands at its own address, and the root items its toolchain fixes
  # answer to the letters before that address's `0`.
  readonly ZBKCW_CRATE_DIR="${ZBKCW_KIT_DIR}/bk0"
  readonly ZBKCW_MANIFEST="${ZBKCW_CRATE_DIR}/Cargo.toml"
  readonly ZBKCW_PIN_FILE="${ZBKCW_CRATE_DIR}/rust-toolchain.toml"
  readonly ZBKCW_BINARY="${ZBKCW_CRATE_DIR}/target/release/bkx"

  readonly ZBKCW_KINDLED=1
}

######################################################################
# Internal

# The converge, performed. Stand on the fence, verify the channel, and hand the
# rebuild decision to cargo.
#
# EVERY DOOR OF THIS FILE PASSES THROUGH IT, which is what the operator's ruling
# leaves the bash doing: a tabtarget routes to a function, the function converges
# and execs, and no logic stands anywhere between them. Written once here rather
# than once per door, because nine copies of a bootstrap are nine chances for one
# door to be reached under terms the others are not.
#
# IT ALWAYS RUNS CARGO, AND CARGO DECIDES. What stood here walked the seat's
# position, asked the standing binary where it was struck, compared the two and
# built on a mismatch - a staleness reading in bash, ahead of the tool whose
# whole trade is staleness readings. build.rs takes that walk now and tells cargo
# what to watch, so a commit touching the election moves the fingerprint and the
# binary is rebuilt, while a commit elsewhere leaves both untouched. The bash
# gained nothing by deciding first except a second answer free to disagree.
#
# THE DOOR LAW MOVED WITH IT, and had to: a refusal the bash held would refuse
# only the doors that ran the bash, where build.rs refuses every build of this
# crate - the kennelman's test build and a bare cargo invocation among them.
zbkcw_converge() {
  bkcb_ensure

  local z_channel=""
  z_channel=$(bkcp_pin_capture "${ZBKCW_PIN_FILE}") \
    || buc_die_now "The kennel's pin at ${ZBKCW_PIN_FILE} names no channel"

  bkcb_channel_present_or_die "${z_channel}"

  cargo "+${z_channel}" build --release --manifest-path "${ZBKCW_MANIFEST}" --locked \
    || buc_die_now "The kennel did not build"

  test -x "${ZBKCW_BINARY}" \
    || buc_die_now "The build reported success but no binary stands at ${ZBKCW_BINARY}"
}

# One door: converge, then exec the kennel with the door's own word ahead of
# whatever the caller typed.
#
# THE FOLIO IS THE COLLAR, and the param1 channel is what carries it. That
# channel lifts the tabtarget's first argument into BUZ_FOLIO and hands the
# command only what remains (buz_exec_lookup), which is exactly the shape of a
# door whose folio names a target - so the collar is put back at the head of the
# argument list and the binary sees the shape the frozen command surface
# declares: the verb, the collar, then what the collar's kind owns.
#
# A DOOR THAT TAKES NOTHING IS THE SAME CALL, and its argument is refused by the
# KENNEL rather than dropped by the substrate: every colophon is enrolled on
# param1, so what was typed reaches the binary and the door says what it takes.
# No door of this file knows whether it is one of those, which is the point -
# the grammar is stated once, in the kennel, and never a second time here.
zbkcw_door() {
  local z_word="$1"
  shift

  zbkcw_converge

  exec "${ZBKCW_BINARY}" "${z_word}" ${BUZ_FOLIO:+"${BUZ_FOLIO}"} "$@"
}

######################################################################
# External
#
# NINE FUNCTIONS, ONE SHAPE. Each names its door's word and hands everything
# else to zbkcw_door; what differs between them is the word and the help text.
# That is the whole of what the one-tabtarget-per-door ruling asks the bash to
# hold, and the reason it can hold so little: the grammar of each door is the
# kennel's, stated in the binary and nowhere here.
#
# THE BASH DOES NOT KNOW WHICH DOORS STAND. Five of these words reach a blank
# and are refused there by name, and a bash that held a second copy of that fact
# would be free to disagree with the binary about it - saying a door is absent
# after its pace has landed, or worse, saying it stands when it does not. Each
# function converges and execs; the door answers for itself.

# The whistle, bare: converge the kennel and let it report its own making.
#
# IT PASSES NO WORD AT ALL, which is what makes it the one call here that is not
# a door. The kennel handed no argument reports the position it was built at,
# the channel its pin asked for and the compiler that answered - the one question
# that needs no collar.
bkcw_kennel() {
  buc_doc_brief "Report the kennel's own making, converging the binary first where the seat has outrun it"
  buc_doc_shown || return 0

  # THE DOOR-WORD ARGUMENT MODE IS REFUSED, NOT IGNORED. Until the operator's
  # ruling this door took a door word and passed it on, so the call below worked
  # yesterday and someone will type it tomorrow; a whistle that dropped the word
  # and reported its own making would answer a question nobody asked and look
  # like it had obeyed. The refusal carries the forwarding address, because the
  # roster of tabtargets IS the command surface now.
  test -z "${BUZ_FOLIO:-}" || buc_die_now \
    "The whistle takes nothing. Every door of the kennel wears its own tabtarget with the collar as its argument, so '${BUZ_FOLIO}' is reached at tt/bkw-<letter>.<Door>.sh - list them with: ls tt/bkw-*"

  zbkcw_converge

  exec "${ZBKCW_BINARY}"
}

# mush - the launch seam.
bkcw_mush() {
  buc_doc_brief "Launch what one collar names - a suite under the runner it declares, an app under the binary election"
  buc_doc_param "collar" "the collar to launch, followed by what the collar's kind owns"
  buc_doc_shown || return 0

  zbkcw_door mush "$@"
}

# derby - the whole-kennel run.
bkcw_derby() {
  buc_doc_brief "Run every suite collar the discovery walk finds, sequentially, one verdict per collar and one for the set"
  buc_doc_shown || return 0

  zbkcw_door derby "$@"
}

# lineup - the no-run listing.
bkcw_lineup() {
  buc_doc_brief "List a suite's hurdles as its runner names them, running nothing"
  buc_doc_param "collar" "the suite collar to list"
  buc_doc_shown || return 0

  zbkcw_door lineup "$@"
}

# tattoo - the read-only examination.
bkcw_tattoo() {
  buc_doc_brief "Proclaim one collar and report every conformance finding, changing nothing on disk"
  buc_doc_param "collar" "the collar to read"
  buc_doc_shown || return 0

  zbkcw_door tattoo "$@"
}

# heel - the converge.
bkcw_heel() {
  buc_doc_brief "Build a collar's launchable current through the leash and install it at its residence"
  buc_doc_param "collar" "the collar to converge"
  buc_doc_shown || return 0

  zbkcw_door heel "$@"
}

# muzzle - the lint step.
bkcw_muzzle() {
  buc_doc_brief "Lint every collar the walk finds at its declared muzzle directory, each bar naming the surface to adopt"
  buc_doc_shown || return 0

  zbkcw_door muzzle "$@"
}

# gangline - the lock re-derivation.
bkcw_gangline() {
  buc_doc_brief "Re-derive a collar's lock - the one act in which the leash lifts the lock flag"
  buc_doc_param "collar" "the collar whose manifest is re-locked"
  buc_doc_shown || return 0

  zbkcw_door gangline "$@"
}

# scoop - the whole yard's clean.
bkcw_scoop() {
  buc_doc_brief "Remove every collar's target directory, on the operator's word alone - no stamp, no cadence"
  buc_doc_shown || return 0

  zbkcw_door scoop "$@"
}

# eof
