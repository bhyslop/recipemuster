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
# BKCK Kennelman - the one door that stands outside the kennel
#
# The kennel's own suite is a suite collar like any crate's, and one day the
# kennel will launch it. But A KENNEL THAT IS BROKEN CANNOT LAUNCH THE HURDLES
# THAT WOULD SAY SO, which is the whole reason this door exists: it runs them by
# hand, through the toolchain fence directly, with no kennel door in its path.
#
# It is the founding pace's test door and it STAYS after the founding. That is
# not a transitional note — it is permanently how the kennel is proven when the
# kennel cannot prove itself, and retiring it would leave the suite reachable
# only through the artifact under test.
#
# IT IS NOT THE WHISTLE. The whistle builds the kennel and execs it; this never
# reaches a kennel door at all. What the two share is the door law and the fence,
# which every door in the estate honors alike.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKCK_SOURCED:-}" || buc_die_now "Module bkck multiply sourced - check sourcing hierarchy"
ZBKCK_SOURCED=1

zbkck_kindle() {
  test -z "${ZBKCK_KINDLED:-}" || buc_die_now "Module bkck already kindled"

  test -n "${BURC_TOOLS_DIR:-}" || buc_die_now "BURC_TOOLS_DIR is unset"

  readonly ZBKCK_REPO_ROOT="${PWD}"
  readonly ZBKCK_KIT_DIR="${ZBKCK_REPO_ROOT}/${BURC_TOOLS_DIR}/bkk"
  # The kit directory is a delivery name and constrains nothing beneath it. The
  # crate stands at its own address, and the root items its toolchain fixes
  # answer to the letters before that address's `0`.
  readonly ZBKCK_CRATE_DIR="${ZBKCK_KIT_DIR}/bk0"
  readonly ZBKCK_MANIFEST="${ZBKCK_CRATE_DIR}/Cargo.toml"

  # The floor crate, standing beside the kennel and carrying the regime reader.
  # Its hurdles run under this door and not under the kennel's, for the same
  # reason the kennel's own do: a floor whose only proof ran through its first
  # consumer could not be the crate the estate's other crates stand on.
  #
  # IT STATES NO PIN OF ITS OWN, and that is the leash's law rather than an
  # omission - the nearest pin at or above a crate's directory answers for it, so
  # the kit's own pin governs both crates and a bump moves them together.
  readonly ZBKCK_READER_MANIFEST="${ZBKCK_CRATE_DIR}/bkp/Cargo.toml"

  # The substrate's own suite, which runs under this door for the same reason the
  # two above do, carried one step further. A broken kennel cannot launch the
  # hurdles that would say so - and the substrate's suite is where the WHISTLE's
  # own behaviours are proven, the whistle being the door that builds the kennel.
  # Launching that suite through the kennel would ask a broken artifact to prove
  # the door that builds it. The substrate is bash alone, so its hurdles stand in
  # this kit beside the kennel's, at the kit's own `but` address.
  readonly ZBKCK_SUBSTRATE_MANIFEST="${ZBKCK_KIT_DIR}/but0/Cargo.toml"

  readonly ZBKCK_PIN_FILE="${ZBKCK_CRATE_DIR}/rust-toolchain.toml"

  # The composing surface, and the name of the declaration standing in it. The
  # kennelman verifies the required toolchains before it runs a hurdle, and it
  # READS that requirement rather than restating it: a second list here would be
  # free to drift from the one the suite asserts against, and the drift would
  # present as a station this door cleared and the hurdles then failed - which
  # is the reading this check exists to move earlier.
  readonly ZBKCK_SURFACE="${ZBKCK_CRATE_DIR}/src/bktu_lure.rs"
  readonly ZBKCK_TOOLCHAINS_CONST="BKTU_REQUIRED_TOOLCHAINS"

  readonly ZBKCK_KINDLED=1
}

######################################################################
# Internal

# The channels the composing surface declares, one per line.
#
# Parsed rather than imported, because the declaration is rust and this door is
# bash; what makes the parse safe is that it is anchored on the declaration
# itself, so a mention of the name in prose above it cannot answer.
#
# Returns non-zero where the declaration cannot be found or reads empty. The
# caller dies on that: an empty requirement would clear every station without
# reading one, which is indistinguishable on the console from a station that
# holds everything.
zbkck_required_toolchains_capture() {
  test -f "${ZBKCK_SURFACE}" || return 1

  local z_lines=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${ZBKCK_SURFACE}"

  # The first line that opens the declaration, matched from the line's start.
  local z_declaration=""
  local z_i
  for z_i in "${!z_lines[@]}"; do
    case "${z_lines[${z_i}]}" in
      "pub const ${ZBKCK_TOOLCHAINS_CONST}"*) z_declaration="${z_lines[${z_i}]}"; break ;;
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
  local -r z_newline=$'\n'
  local z_rest="${z_declaration}"
  local z_channels=""
  while [[ "${z_rest}" =~ ${z_quoted_pattern} ]]; do
    z_channels="${z_channels:+${z_channels}${z_newline}}${BASH_REMATCH[1]}"
    z_rest="${BASH_REMATCH[2]}"
  done

  test -n "${z_channels}" || return 1

  echo "${z_channels}"
}

# Refuse a station that cannot run the hurdles, naming what it lacks.
#
# A HURDLE'S DEMAND, NEVER THE KENNEL'S: at runtime the kennel binds whatever pin
# a collar's repository names and asks nothing of the station beyond that pin.
# The hurdles ask more, because proving that a bumped pin changes the compiler
# needs a second toolchain present to bump TO, and a station holding one cannot
# demonstrate the property at all.
#
# IT REFUSES AND DOES NOT FETCH, which is the envelope's routine-download
# election carried through. The converge is named so the refusal is a remedy
# rather than a report.
zbkck_toolchain_fence_or_die() {
  local z_required=""
  z_required=$(zbkck_required_toolchains_capture) \
    || buc_die_now "Could not read ${ZBKCK_TOOLCHAINS_CONST} from ${ZBKCK_SURFACE} - the kennelman reads the hurdles' own declaration rather than holding a list of its own, and an unread requirement would clear every station without asking one"

  # Asked only of a home that stands, on the fence's own ground: rustup creates
  # the RUSTUP_HOME it is pointed at, and a home that does not stand holds none
  # of the required toolchains, which the refusal below then names.
  local z_held=""
  local z_held_status=0
  if test -d "${BKCB_RUSTUP_HOME}"; then
    z_held=$(CARGO_HOME="${BKCB_CARGO_HOME}" RUSTUP_HOME="${BKCB_RUSTUP_HOME}" rustup toolchain list) \
      || z_held_status=$?
  fi
  test "${z_held_status}" -eq 0 \
    || buc_die_now "Could not ask rustup what the tackroom at ${BKCB_RUSTUP_HOME:-<unset>} holds (exit ${z_held_status})"

  # A channel name carries no whitespace, so the one-per-line requirement splits
  # on one line; each is matched against rustup's listing as a fixed substring.
  local -r z_newline=$'\n'
  local z_channels=()
  read -r -a z_channels <<< "${z_required//"${z_newline}"/ }" \
    || buc_die_now "Could not split the required toolchains read from ${ZBKCK_SURFACE}"

  local z_missing=""
  local z_c
  for z_c in "${!z_channels[@]}"; do
    case "${z_held}" in
      *"${z_channels[${z_c}]}"*) continue ;;
    esac
    z_missing="${z_missing:+${z_missing} }${z_channels[${z_c}]}"
  done

  test -z "${z_missing}" \
    || buc_die_now "The station is missing the toolchain(s) the kennel's hurdles require: ${z_missing} - the converge is rebed - tt/bkw-r.Rebed.sh, which lays the pin's channel and every one the hurdles demand"

  buc_step "The station holds every toolchain the hurdles require"
}

# Drive one crate's hurdles through the fence, and die naming whose they were.
#
# The three disciplines the leash holds are spelled here for the same reason the
# whistle spells them: this door deliberately does not reach the kennel, so it
# cannot reach the compiled chokepoint either. What it must not do is spell them
# once per crate - a second copy is a second chance for one crate to be driven
# under terms the other is not.
#
# THE FILTER REACHES BOTH CRATES, and a filter matching nothing in one of them is
# not an error: cargo's harness reports an empty selection as a pass, which is
# the behavior the kennel's own selection pipeline exists to correct and which
# this door, standing outside the kennel, deliberately does not correct.
zbkck_hurdles_or_die() {
  local -r z_manifest="${1}"
  local -r z_whose="${2}"
  local -r z_channel="${3}"
  local -r z_filter="${4:-}"

  local z_cargo_status=0
  if test -n "${z_filter}"; then
    CARGO_HOME="${BKCB_CARGO_HOME}" RUSTUP_HOME="${BKCB_RUSTUP_HOME}" \
      cargo "+${z_channel}" test --manifest-path "${z_manifest}" --locked -- "${z_filter}" \
      || z_cargo_status=$?
  else
    CARGO_HOME="${BKCB_CARGO_HOME}" RUSTUP_HOME="${BKCB_RUSTUP_HOME}" \
      cargo "+${z_channel}" test --manifest-path "${z_manifest}" --locked \
      || z_cargo_status=$?
  fi
  test "${z_cargo_status}" -eq 0 \
    || buc_die_now "${z_whose} hurdles did not all clear (exit ${z_cargo_status})"
}

######################################################################
# External

# Run the kennel's own suite through the fence, by hand.
#
# The optional filter is the runner's own test-name filter, passed through
# untouched, so one hurdle or one module can be driven during a falsification
# cycle. It narrows WHAT RUNS and is meant to.
#
# IT IS READ FROM THE FOLIO AND NOT FROM $1. This colophon is enrolled on the
# param1 channel, which lifts what follows the tabtarget's first token into
# BUZ_FOLIO and hands the command only what remains - so a positional read finds
# nothing and a filtered drive silently runs the whole suite, which is the one
# failure a filter has: it does not refuse, it over-runs. The whistle beside this
# door reads the same channel the same way.
bkck_suite() {
  local z_filter="${BUZ_FOLIO:-}"

  bkcb_ensure

  # AHEAD OF THE CARGO CALL, and after the fence: the reading is of the tackroom
  # the fence proved rather than of whatever rustup the station user holds, and
  # a station that cannot run the hurdles is told so before it spends a compile
  # finding out.
  zbkck_toolchain_fence_or_die

  local z_channel=""
  z_channel=$(bkcp_pin_capture "${ZBKCK_PIN_FILE}") \
    || buc_die_now "The kennel's pin at ${ZBKCK_PIN_FILE} names no channel"

  # THE DOOR LAW AND THE POSITION ARE NOT SPELLED HERE, and their absence is the
  # closure rather than an omission. Both are build.rs's, taken over the same
  # election the crate reads and refused there once - so a test build that could
  # not state where it was taken from dies exactly as a binary build does, and no
  # door of this kit holds a second copy of the reading to disagree with.
  buc_step "Running the kit's hurdles under ${z_channel}, outside the kennel"

  # THE READER FIRST, then the kennel. The kennel is the reader's first consumer,
  # so a reader that cannot read refuses the kennel's hurdles for a reason that
  # is not the kennel's - and reading that verdict off the kennel's own failures
  # is exactly the search this ordering spares.
  zbkck_hurdles_or_die "${ZBKCK_READER_MANIFEST}"    "the reader's"    "${z_channel}" "${z_filter}"
  zbkck_hurdles_or_die "${ZBKCK_MANIFEST}"           "the kennel's"    "${z_channel}" "${z_filter}"
  zbkck_hurdles_or_die "${ZBKCK_SUBSTRATE_MANIFEST}" "the substrate's" "${z_channel}" "${z_filter}"

  buc_success "The kit's hurdles cleared"
}

# eof
