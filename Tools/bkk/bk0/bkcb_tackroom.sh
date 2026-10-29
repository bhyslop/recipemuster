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
# BKCB Tackroom - the toolchain binding, and the whole of what the bootstrap
# knows about a toolchain
#
# THE KIT SHIPS AND ITS BOOTSTRAP MUST RUN WHERE IT LANDS. What stood here was a
# source line into another kit's toolchain module, and no parcel carries that
# kit: on a receiving station the whistle died at the source line, ahead of every
# refusal it was written to give. So the binding the bootstrap actually needs is
# stated here, under this kit's own head, and this file names nothing outside the
# kit and the substrate.
#
# TWO ACTS AND NO THIRD. Read the station's declaration, and refuse one that
# names the station user's own homes. The two homes beneath it are composed here
# (BKCB_CARGO_HOME, BKCB_RUSTUP_HOME) and EXPORTED NOWHERE: each toolchain command
# the kit's bash runs states them on itself, and the kennel derives the same two
# beneath the tackroom its dispatch file names and states them on every command
# it composes. A home exported here would reach every process the door spawns,
# and a course is handed the dispatch's pointer and nothing else of the kit.
#
# IT PROVISIONS NOTHING, and that is an election rather than an omission: a
# routine invocation verifies and refuses, and never surprises the operator with
# a download. A station whose tackroom's pinned toolchain cannot answer is told
# so by name, and told the converge that lays it - rebed, which stands beside
# the whistle and lays only a channel that does not answer. What the estate's
# own trees additionally do — fetch a channel on first use, under a directory
# lock — is theirs and stays theirs; a kit that provisioned on a receiving station
# would be reaching for the network from a bootstrap the operator ran to build
# one binary.
#
# THE GUARD IS LEXICAL AND RUNS BEFORE ANYTHING IS CREATED. What it catches is a
# declaration that would build the store in the operator's personal rustup or
# cargo homes, and creating those directories and then complaining is worse than
# useless - the mess outlives the refusal.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKCB_SOURCED:-}" || buc_die_now "Module bkcb multiply sourced - check sourcing hierarchy"
ZBKCB_SOURCED=1

zbkcb_kindle() {
  test -z "${ZBKCB_KINDLED:-}" || buc_die_now "Module bkcb already kindled"

  # Carried across the dispatch exec boundary by the substrate. Empty here is not
  # fatal at kindle: bkcb_ensure is the one entry point that goes near a
  # toolchain, and that is where the requirement bites.
  readonly ZBKCB_TACKROOM="${BURD_TACKROOM:-}"

  readonly BKCB_CARGO_HOME="${ZBKCB_TACKROOM}/cargo"
  readonly BKCB_RUSTUP_HOME="${ZBKCB_TACKROOM}/rustup"

  # Where the channel's proof keeps what the compiler answered, and what it said
  # on stderr, kept for forensics. Empty here is not fatal at kindle, on the
  # tackroom's own ground: the proof is where it bites.
  readonly ZBKCB_TEMP_DIR="${BURD_TEMP_DIR:-}"
  readonly ZBKCB_ANSWER_PREFIX="${ZBKCB_TEMP_DIR}/bkcb_answer_"

  # Where the proof leaves the condition a channel failed on, one file per
  # channel: read back by the refusal here and by rebed's note of what it lays.
  readonly BKCB_CONDITION_PREFIX="${ZBKCB_TEMP_DIR}/bkcb_condition_"

  # What every refusal of the channel's proof hands its reader, ahead of the door
  # it names. Spelled once in this register; the rust register's spelling is
  # BKCL_CONVERGE_SENTENCE in src/bkcl_leash.rs, and a hurdle holds the two equal.
  readonly ZBKCB_CONVERGE_SENTENCE="A routine invocation verifies and refuses; it never downloads unasked. Every toolchain the kennel requires is laid by its readiness converge,"
  readonly ZBKCB_REBED_DOOR="rebed - tt/bkw-r.Rebed.sh"

  readonly ZBKCB_KINDLED=1
}

######################################################################
# Internal

zbkcb_sentinel() {
  test "${ZBKCB_KINDLED:-}" = "1" || buc_die_now "Module bkcb not kindled - call zbkcb_kindle first"
}

# Refuses an inadmissible tackroom BEFORE anything is created inside it. Purely
# lexical: no directory is made, no path is resolved, nothing in the station
# user's home is touched.
zbkcb_admissible_or_die() {
  zbkcb_sentinel
  local -r z_declared="${1}"

  local z_home_dir="${HOME:-}"
  test -n "${z_home_dir}" || return 0

  test "${z_declared}" != "${z_home_dir}" \
    || buc_die_now "BURS_TACKROOM names the station user's home directory itself (${z_declared}) - it must name a store of its own"

  local z_personal
  for z_personal in "${z_home_dir}/.rustup" "${z_home_dir}/.cargo"; do
    case "${z_declared}" in
      "${z_personal}"|"${z_personal}"/*)
        buc_die_now "BURS_TACKROOM resolves inside the personal rustup/cargo home (${z_declared}) - the kennel keeps its store apart from the station user's own toolchain, which is the whole point of the redirection" ;;
    esac
  done
}

######################################################################
# External

# Prove the station's tackroom admissible. Call before any cargo or rustup
# invocation, which then states BKCB_CARGO_HOME and BKCB_RUSTUP_HOME on itself.
bkcb_ensure() {
  zbkcb_sentinel

  test -n "${ZBKCB_TACKROOM}" \
    || buc_die_now "BURS_TACKROOM is unset - the kennel homes the pinned Rust toolchain and the cargo registry in the station's tackroom and will not fall back to a per-clone copy. Declare it in the station file (inspect with tt/buw-rsr.RenderStationRegime.sh), e.g. BURS_TACKROOM=../tackroom-buk"

  zbkcb_admissible_or_die "${ZBKCB_TACKROOM}"
}

# Refuse the channel's proof: the condition met, then the door that re-lays the
# toolchain.
#
# Args: <condition>
zbkcb_rebed_or_die() {
  zbkcb_sentinel
  local -r z_condition="${1:-}"

  buc_warn "${z_condition}"
  buc_warn "${ZBKCB_CONVERGE_SENTENCE} ${ZBKCB_REBED_DOOR}"
  buc_die_now "The pinned toolchain does not answer in the tackroom"
}

# Whether the channel's toolchain in the tackroom ANSWERS, the condition met
# left at the channel's condition file where it does not.
#
# THE PROOF IS THE COMPILER'S OWN ANSWER, NEVER RUSTUP'S LEDGER. A listing says
# a channel was installed, and it went on saying so through a storage purge that
# had removed the compiler's own libraries, so the first thing to run the
# compiler was the kennel's build script, which panicked. The toolchain's rustc
# is asked at its absolute path, in rustup's own layout, for its verbose
# version; the proof is that `release:` names the channel and `host:` names
# something.
#
# NEVER THROUGH A PROXY. Told to run a channel it does not hold, rustup fetches
# one, and a routine invocation never downloads unasked. A compiler reached at
# its own path fetches nothing, and an absent channel is refused here before
# anything is spawned. Nothing is asked of a home that does not stand, so the
# proof makes no store on the way to refusing one.
#
# THE DRIVER LIBRARY IS READ BEFORE THE COMPILER IS SPAWNED, and this is a
# membrane at the Palisade rather than a rule of ours. The signature: on macOS a
# storage purge removed lib/librustc_driver-<hash>.dylib and left bin/rustc, and
# a compiler spawned without its driver aborts at load - which the station
# answers with a crash dialog a person must dismiss, so a proof that spawned it
# to find out would stop an unattended road on a hand. A toolchain whose
# directory holds no driver library is not answering, said so, and nothing is
# spawned. Windows lays the driver beside the compiler in bin/, so either seat
# admits it. It retires when the station no longer raises a dialog for a
# compiler aborting at load; the spawn below then reaches the same verdict.
#
# A PREDICATE, so rebed can ask it of each channel and lay only what does not
# answer; the whistle's refusal is bkcb_channel_answers_or_die below, which
# reads the condition this left. Writes nothing to either stream.
#
# The rust register holds the same proof at bkcl_channel_answers and cannot be
# called from here: the whistle runs it before the binary carrying it exists.
#
# Args: <channel>
bkcb_channel_answers_predicate() {
  zbkcb_sentinel

  local -r z_channel="${1:-}"
  test -n "${z_channel}" || return 1
  test -n "${ZBKCB_TEMP_DIR}" || return 1

  local -r z_condition_file="${BKCB_CONDITION_PREFIX}${z_channel}.txt"

  # A CHANNEL THE ANSWER CANNOT EQUAL IS REFUSED, NOT GUESSED AT: a named channel
  # or a two-part version resolves to a release only rustup's ledger knows.
  if ! [[ "${z_channel}" =~ ^[0123456789]+\.[0123456789]+\.[0123456789]+$ ]]; then
    printf '%s\n' "The kennel pins ${z_channel}, which is no release a compiler answers with: the kennel proves a channel by the release its compiler names, so the pin is a three-part version" > "${z_condition_file}" || return 1
    return 1
  fi

  local z_laid=()
  local z_candidate=""
  for z_candidate in "${BKCB_RUSTUP_HOME}/toolchains/${z_channel}-"*; do
    test -d "${z_candidate}" || continue
    z_laid+=("${z_candidate}")
  done

  if test "${#z_laid[@]}" -eq 0; then
    printf '%s\n' "The tackroom holds no toolchain for ${z_channel} under ${BKCB_RUSTUP_HOME}/toolchains" > "${z_condition_file}" || return 1
    return 1
  fi
  if test "${#z_laid[@]}" -ne 1; then
    printf '%s\n' "The tackroom holds ${#z_laid[@]} toolchains answering ${z_channel} under ${BKCB_RUSTUP_HOME}/toolchains, and the kennel will not pick one on the operator's behalf" > "${z_condition_file}" || return 1
    return 1
  fi

  local z_driver=0
  for z_candidate in "${z_laid[0]}/lib/librustc_driver-"* "${z_laid[0]}/bin/rustc_driver-"*; do
    test -f "${z_candidate}" || continue
    z_driver=1
  done
  if test "${z_driver}" -eq 0; then
    printf '%s\n' "The toolchain for ${z_channel} stands at ${z_laid[0]} without its compiler's driver library (lib/librustc_driver-*), so its compiler would abort at load and is not spawned" > "${z_condition_file}" || return 1
    return 1
  fi

  local -r z_rustc="${z_laid[0]}/bin/rustc"
  local -r z_answer_file="${ZBKCB_ANSWER_PREFIX}${z_channel}.txt"
  local -r z_stderr_file="${ZBKCB_ANSWER_PREFIX}${z_channel}_stderr.txt"

  local z_rustc_status=0
  "${z_rustc}" -vV > "${z_answer_file}" 2> "${z_stderr_file}" \
    || z_rustc_status=$?
  if test "${z_rustc_status}" -ne 0; then
    printf '%s\n' "The toolchain for ${z_channel} stands at ${z_laid[0]} but its compiler did not answer (exit ${z_rustc_status}) - see ${z_stderr_file}" > "${z_condition_file}" || return 1
    return 1
  fi

  local z_said=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_said+=("${z_line}")
  done < "${z_answer_file}"

  # The labels are rustc's own and are matched rather than counted to: the
  # answer's line order is a layout nobody promised.
  local z_release=""
  local z_host=""
  local z_i=0
  for z_i in "${!z_said[@]}"; do
    case "${z_said[${z_i}]}" in
      "release: "*) z_release="${z_said[${z_i}]#release: }" ;;
      "host: "*)    z_host="${z_said[${z_i}]#host: }" ;;
    esac
  done

  if test "${z_release}" != "${z_channel}"; then
    printf '%s\n' "${z_rustc} -vV answered release '${z_release}' where the kennel pins ${z_channel} - see ${z_answer_file}" > "${z_condition_file}" || return 1
    return 1
  fi
  if test -z "${z_host}"; then
    printf '%s\n' "${z_rustc} -vV answered no host line - see ${z_answer_file}" > "${z_condition_file}" || return 1
    return 1
  fi

  return 0
}

# Prove the pinned channel's toolchain in the tackroom ANSWERS, and refuse
# naming the door that re-lays it where it does not: the predicate above, its
# condition read back and made the refusal.
#
# Args: <channel>
bkcb_channel_answers_or_die() {
  zbkcb_sentinel
  buc_doc_brief "Prove the pinned channel's toolchain in the tackroom answers, refusing naming rebed where it does not"
  buc_doc_param "channel" "the channel the pin names, a three-part version"
  buc_doc_shown || return 0

  local -r z_channel="${1:-}"
  test -n "${z_channel}" || buc_die_now "The channel's proof needs the channel the pin names"

  test -n "${ZBKCB_TEMP_DIR}" \
    || buc_die_now "BURD_TEMP_DIR is unset - the channel's proof keeps the compiler's answer in the dispatch's temp directory"

  if bkcb_channel_answers_predicate "${z_channel}"; then
    return 0
  fi

  local z_condition=""
  z_condition=$(<"${BKCB_CONDITION_PREFIX}${z_channel}.txt") \
    || buc_die_now "The toolchain for ${z_channel} does not answer, and the proof left no condition at ${BKCB_CONDITION_PREFIX}${z_channel}.txt"

  zbkcb_rebed_or_die "${z_condition}"
}

# eof
