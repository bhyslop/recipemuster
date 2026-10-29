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
# BKCW Whistle - the thin bash bootstrap that builds the kennel and execs it,
# and beside it the toolchain half of rebed, the kennel's readiness converge
#
# TWO BASH ENTRIES REMAIN, and both remain because they must run before the
# kennel binary exists. The whistle is the bootstrap the founding cinch carved
# out: the kennel cannot build itself, so something outside the binary has to
# exist before the binary does, and it binds the toolchain once, minimally, and
# grows nothing else. Rebed's toolchain half lays what does not answer: a gutted
# toolchain is exactly when the kennel cannot be built, so that half needs only
# rustup and never the binary, and the rest of rebed is the binary's. Every
# other door of this kit is a rust interior.
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
test -z "${ZBKCW_SOURCED:-}" || buc_die_now "Module bkcw multiply sourced - check sourcing hierarchy"
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

  # The hurdles' own declaration of the toolchains they demand, which rebed
  # reads as the kennelman does rather than restating: a second list here would
  # be free to drift from the one the suite asserts against, and the drift would
  # present as a tackroom rebed called whole and the hurdles then found short.
  readonly ZBKCW_SURFACE="${ZBKCW_CRATE_DIR}/src/bktu_lure.rs"
  readonly ZBKCW_TOOLCHAINS_CONST="BKTU_REQUIRED_TOOLCHAINS"

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

  bkcb_channel_answers_or_die "${z_channel}"

  local z_cargo_status=0
  CARGO_HOME="${BKCB_CARGO_HOME}" RUSTUP_HOME="${BKCB_RUSTUP_HOME}" \
    cargo "+${z_channel}" build --release --manifest-path "${ZBKCW_MANIFEST}" --locked \
    || z_cargo_status=$?
  test "${z_cargo_status}" -eq 0 \
    || buc_die_now "The kennel did not build (exit ${z_cargo_status})"

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
  local -r z_word="${1}"
  test "$#" -eq 0 || shift

  zbkcw_converge

  exec "${ZBKCW_BINARY}" "${z_word}" ${BUZ_FOLIO:+"${BUZ_FOLIO}"} "$@" \
    || buc_die_now "Could not exec the kennel at ${ZBKCW_BINARY}"
}

# One rustup act of rebed's, its status contained: rustup's own exit codes are
# not the kennel's to pass through the band, so any failure is named and the
# door dies on it.
#
# RUSTUP RUNS UNDER ITS OWN CARGO HOME, NEVER THE TACKROOM'S. A build states
# CARGO_HOME as the tackroom's registry store, which is every build's; rustup
# reads the same name as the place it is itself installed, and refuses a
# toolchain install where it does not stand there - "rustup is not installed at"
# the tackroom's cargo home, met on a real station after the toolchain had
# landed. So each act states CARGO_HOME as the home the rustup on the path
# stands in, the directory above the one holding it, and leaves RUSTUP_HOME the
# tackroom's, where every toolchain lands. Rustup and its shims are the one
# thing admitted from the station user's own home.
#
# Args: <rustup argument>...
zbkcw_rustup_or_die() {
  # The rustup the path offers first, run by the path it was found at, so the
  # home stated and the program run are one answer.
  local z_rustup
  z_rustup=$(buc_executable_path_capture rustup) \
    || buc_die_now "No rustup stands on the path - rebed lays every toolchain through rustup and nothing else"
  local -r z_rustup_home="${z_rustup%/*/*}"

  local z_rustup_status=0
  CARGO_HOME="${z_rustup_home}" RUSTUP_HOME="${BKCB_RUSTUP_HOME}" "${z_rustup}" "$@" || z_rustup_status=$?
  test "${z_rustup_status}" -eq 0 \
    || buc_die_now "rustup $* failed under ${BKCB_RUSTUP_HOME}, rustup's own home ${z_rustup_home} (exit ${z_rustup_status})"
}

# The channels the hurdles demand, space-separated, read off the declaration the
# suite asserts against.
#
# READ FROM THE SOURCE AND NEVER ASKED OF THE BINARY, for two reasons that do not
# bend. The declaration is a hurdle's demand and is compiled into the hurdles
# alone, so no binary carries it; and rebed runs exactly when the kennel may not
# build. Anchored on the declaration's own line, as the kennelman's reading is,
# so a mention of the name in prose above it cannot answer.
#
# Returns non-zero where the declaration cannot be found or reads empty, and the
# caller dies on that: an unread demand would re-lay the pin alone and call the
# tackroom whole.
zbkcw_hurdle_channels_capture() {
  test -f "${ZBKCW_SURFACE}" || return 1

  local z_lines=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${ZBKCW_SURFACE}"

  local z_declaration=""
  local z_i=0
  for z_i in "${!z_lines[@]}"; do
    case "${z_lines[${z_i}]}" in
      "pub const ${ZBKCW_TOOLCHAINS_CONST}"*) z_declaration="${z_lines[${z_i}]}"; break ;;
    esac
  done
  test -n "${z_declaration}" || return 1

  # The declaration runs on through the line that closes it, since the formatter
  # holds a list of two or more one item per line.
  while [[ "${z_declaration}" != *";"* ]]; do
    z_i=$(( z_i + 1 ))
    test "${z_i}" -lt "${#z_lines[@]}" || return 1
    z_declaration="${z_declaration} ${z_lines[${z_i}]}"
  done

  # Each quoted literal in the declaration, in order, walked off its front.
  local -r z_quoted_pattern='"([^"]*)"(.*)'
  local z_rest="${z_declaration}"
  local z_channels=""
  while [[ "${z_rest}" =~ ${z_quoted_pattern} ]]; do
    z_channels="${z_channels:+${z_channels} }${BASH_REMATCH[1]}"
    z_rest="${BASH_REMATCH[2]}"
  done

  test -n "${z_channels}" || return 1

  echo "${z_channels}"
}

# Lay one channel again: uninstall it, install it fresh, prove its compiler
# answers, and say so.
#
# Args: <channel>
zbkcw_lay_or_die() {
  local -r z_channel="${1:-}"
  test -n "${z_channel}" || buc_die_now "Laying a channel needs the channel"

  buc_step "Uninstalling ${z_channel} from the tackroom's rustup home ${BKCB_RUSTUP_HOME}"
  zbkcw_rustup_or_die toolchain uninstall "${z_channel}"

  buc_step "Installing ${z_channel} into ${BKCB_RUSTUP_HOME}"
  zbkcw_rustup_or_die toolchain install "${z_channel}" --profile minimal --component clippy --component rustfmt --no-self-update

  bkcb_channel_answers_or_die "${z_channel}"

  buc_success "The tackroom's ${z_channel} is laid again, and its compiler answers"
}

# Rebed's toolchain half: every channel the kennel requires asked whether it
# answers, and only one that does not laid again.
#
# EVERY CHANNEL THE KENNEL REQUIRES, AND NOT THE PIN'S ALONE. The pin's channel
# builds the kennel; the hurdles demand a second, because proving a bumped pin
# reaches the compiler needs a toolchain to bump to. So the pin is read first,
# as the whistle reads it, and then every other channel the hurdles' own
# declaration names.
#
# ONLY WHAT DOES NOT ANSWER IS LAID, which is what makes the converge safe to
# drive at any time: the proof the whistle takes is asked of each channel as a
# predicate, and an answering channel is left standing and asked nothing more of
# rustup. A gutted one is refused by that proof before its compiler is spawned,
# so deciding to lay it never raises the station's crash dialog.
#
# THE FENCE FIRST, because without it rustup writes the station user's own
# home: the hand repair this door replaces landed there for want of it. Both
# readings stand ahead of the first proof, so a demand that cannot be read
# refuses before anything is taken down.
zbkcw_rebed_toolchains() {
  bkcb_ensure

  local z_pin=""
  z_pin=$(bkcp_pin_capture "${ZBKCW_PIN_FILE}") \
    || buc_die_now "The kennel's pin at ${ZBKCW_PIN_FILE} names no channel"

  local z_demanded=""
  z_demanded=$(zbkcw_hurdle_channels_capture) \
    || buc_die_now "Could not read ${ZBKCW_TOOLCHAINS_CONST} from ${ZBKCW_SURFACE} - rebed reads the hurdles' own declaration rather than holding a list of its own, and an unread demand would lay the pin alone and call the tackroom whole"

  local z_others=()
  read -r -a z_others <<< "${z_demanded}" \
    || buc_die_now "Could not split the channels read from ${ZBKCW_SURFACE}"
  (( ${#z_others[@]} )) || buc_die_now "No channel was read from ${ZBKCW_SURFACE}"

  local z_channels=("${z_pin}")
  local z_other=""
  for z_other in "${z_others[@]}"; do
    case " ${z_channels[*]} " in
      *" ${z_other} "*) continue ;;
    esac
    z_channels+=("${z_other}")
  done

  buc_step "Asking ${#z_channels[@]} channel(s) under ${BKCB_RUSTUP_HOME} whether each answers: the pin's ${z_pin}, then each other the hurdles demand"

  local z_laid=0
  local z_channel=""
  local z_condition=""
  for z_channel in "${z_channels[@]}"; do
    if bkcb_channel_answers_predicate "${z_channel}"; then
      buc_step "The tackroom's ${z_channel} answers, and is left as it stands"
      continue
    fi
    z_condition=$(<"${BKCB_CONDITION_PREFIX}${z_channel}.txt") \
      || buc_die_now "The tackroom's ${z_channel} does not answer, and its proof left no condition at ${BKCB_CONDITION_PREFIX}${z_channel}.txt"
    buc_step "The tackroom's ${z_channel} does not answer: ${z_condition}"
    zbkcw_lay_or_die "${z_channel}"
    z_laid=$(( z_laid + 1 ))
  done

  buc_step "Laid ${z_laid} of ${#z_channels[@]} channel(s); every other answered as it stood"
}

######################################################################
# External
#
# ONE SHAPE FOR EVERY FUNCTION BELOW. Each names its door's word and hands
# everything else to zbkcw_door; what differs between them is the word and the
# help text. Rebed, at the foot, takes its toolchain half in bash first, for the
# reason that half is bash at all, and then takes the same shape.
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

  exec "${ZBKCW_BINARY}" \
    || buc_die_now "Could not exec the kennel at ${ZBKCW_BINARY}"
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

# futurity - the derby's set, narrowed by patterns.
bkcw_futurity() {
  buc_doc_brief "Run the hurdles the patterns name across every suite collar the walk finds, naming each course not launched"
  buc_doc_param "pattern" "one or more selection patterns, matched once over every course's listing"
  buc_doc_shown || return 0

  zbkcw_door futurity "$@"
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

# Empties what the kennel keeps on the station for every tree, and this
# checkout's python environments. Run it deliberately, on your own word:
# nothing calls it, and rebed lays it all again.
bkcw_bleach() {
  buc_doc_brief "Empties what the kennel keeps on the station for every tree, and this checkout's python environments. Run it deliberately, on your own word: nothing calls it, and rebed lays it all again."
  buc_doc_shown || return 0

  zbkcw_door bleach "$@"
}

# delouse - the day's sweep, taken ahead of a build.
bkcw_delouse() {
  buc_doc_brief "Take the day's sweep of every collar's build directory if no sweep has landed today - recorded once a day, a no-op after"
  buc_doc_shown || return 0

  zbkcw_door delouse "$@"
}

# rebed - the kennel's readiness converge.
#
# THE ONE CONVERGE THE VERIFY NAMES, safe to drive at any time. Every refusal
# of the channel's proof - the whistle's here in bash, the leash's in rust, and
# the hurdles' own fence - and the derby's refusal over a tree that cannot be
# green send their reader here. It makes a cold station and a cold tree ready
# for the derby in one act: every toolchain the kennel requires answering, the
# kennel converged, and then the binary's own rebed - every heel and every
# strike the derby's preflight names, every manifest's registry closure under
# its lock.
#
# TWO HALVES, AND THE SEAM IS THE WHISTLE'S OWN. The toolchain half stays bash,
# because a toolchain that cannot answer is exactly when no kennel can be built;
# everything after it is the binary's, reached through the same converge-and-
# exec every door takes, so the kennel it execs is current.
bkcw_rebed() {
  buc_doc_brief "Make the station and the tree ready for the derby: lay every toolchain the kennel requires that does not answer, converge the kennel, then perform every heel and strike the derby's preflight names and fetch every manifest's registry closure"
  buc_doc_shown || return 0

  test -z "${BUZ_FOLIO:-}" || buc_die_now \
    "Rebed takes nothing - it converges what the kennel's own pin, hurdles and the tree's declarations name, and '${BUZ_FOLIO}' is no part of that"
  test "$#" -eq 0 || buc_die_now \
    "Rebed takes nothing - it converges what the kennel's own pin, hurdles and the tree's declarations name"

  zbkcw_rebed_toolchains

  zbkcw_door rebed
}

# eof
