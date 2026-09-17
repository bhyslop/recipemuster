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
# hand, through the toolchain fence directly, with no kennel door in its path
# (BKSLR-Lure.adoc "The Kennelman").
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
test -z "${ZBKCK_SOURCED:-}" || return 0
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

  # The regime reader, a crate of its own standing beside the kennel
  # (BKSNC-Kennelcraft.adoc "The Rust Backend"). Its hurdles run under this
  # door and not under the kennel's, for the same reason the kennel's own do: a
  # reader whose only proof ran through its first consumer could not be the crate
  # the estate's other readers converge onto.
  #
  # IT STATES NO PIN OF ITS OWN, and that is the leash's law rather than an
  # omission - the nearest pin at or above a crate's directory answers for it, so
  # the kit's own pin governs both crates and a bump moves them together.
  readonly ZBKCK_READER_MANIFEST="${ZBKCK_CRATE_DIR}/bkl/Cargo.toml"

  # The substrate's own suite, which runs under this door for the same reason the
  # two above do, carried one step further. A broken kennel cannot launch the
  # hurdles that would say so - and the substrate's suite is where the WHISTLE's
  # own behaviours are proven, the whistle being the door that builds the kennel.
  # Launching that suite through the kennel would ask a broken artifact to prove
  # the door that builds it.
  readonly ZBKCK_SUBSTRATE_MANIFEST="${ZBKCK_REPO_ROOT}/${BURC_TOOLS_DIR}/buk/Cargo.toml"

  readonly ZBKCK_PIN_FILE="${ZBKCK_CRATE_DIR}/rust-toolchain.toml"

  # The composing surface, and the name of the declaration standing in it. The
  # kennelman verifies the required toolchains before it runs a hurdle
  # (BKSLR-Lure.adoc "The Kennelman"), and it READS that requirement rather than
  # restating it: a second list here would be free to drift from the one the
  # suite asserts against, and the drift would present as a station this door
  # cleared and the hurdles then failed - which is the reading this check exists
  # to move earlier.
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
  local z_declaration=""
  z_declaration=$(grep -m1 "^pub const ${ZBKCK_TOOLCHAINS_CONST}" "${ZBKCK_SURFACE}") || return 1

  local z_channels=""
  z_channels=$(grep -o '"[^"]*"' <<< "${z_declaration}" | tr -d '"') || return 1

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
# election carried through (BKSNC-Kennelcraft.adoc "Settling Register"). The
# converge is named so the refusal is a remedy rather than a report.
zbkck_toolchain_fence_or_die() {
  local z_required=""
  z_required=$(zbkck_required_toolchains_capture) \
    || buc_die_now "Could not read ${ZBKCK_TOOLCHAINS_CONST} from ${ZBKCK_SURFACE} - the kennelman reads the hurdles' own declaration rather than holding a list of its own, and an unread requirement would clear every station without asking one"

  local z_held=""
  z_held=$(rustup toolchain list) \
    || buc_die_now "Could not ask rustup what the tackroom at ${RUSTUP_HOME:-<unset>} holds"

  local z_missing=""
  local z_channel=""
  while IFS= read -r z_channel; do
    test -n "${z_channel}" || continue
    if grep -qF -- "${z_channel}" <<< "${z_held}"; then
      continue
    fi
    z_missing="${z_missing:+${z_missing} }${z_channel}"
  done <<< "${z_required}"

  test -z "${z_missing}" \
    || buc_die_now "The station is missing the toolchain(s) the kennel's hurdles require: ${z_missing} - the converge is RUSTUP_HOME=${RUSTUP_HOME:-<tackroom>/rustup} rustup toolchain install <channel> --profile minimal"

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
  local z_manifest="$1"
  local z_whose="$2"
  local z_channel="$3"
  local z_filter="${4:-}"

  if test -n "${z_filter}"; then
    cargo "+${z_channel}" test --manifest-path "${z_manifest}" --locked -- "${z_filter}" \
      || buc_die_now "${z_whose} hurdles did not all clear"
  else
    cargo "+${z_channel}" test --manifest-path "${z_manifest}" --locked \
      || buc_die_now "${z_whose} hurdles did not all clear"
  fi
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
  # the fence exported rather than of whatever rustup the station user holds, and
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
