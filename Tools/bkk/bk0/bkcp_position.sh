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
# BKCP Position - what a bash door has to know to build or drive the kennel
#
# THE BUILDING DOORS TAKE ONE READING FROM HERE AND NO MORE. The whistle and the
# kennelman each build the kennel crate, and each reads the pin - stated at the
# invocation rather than inherited. The door law and the position they used to
# take here are build.rs's now, refused and walked once inside the build rather
# than once per door that reaches it.
#
# THE READERS THAT REMAIN STAND OUTSIDE THE KIT: a door that DRIVES the kennel
# rather than building it, as every repointed cargo site does. Such a door asks
# the currency question in bash and refuses where the whistle would have run
# cargo, and it has no build script of its own to ask - so the door law, the
# election walk and the standing-seat capture stay homed here for it. The reach
# runs inward, from a foreign door into this kit, which is the direction the
# kit's closure leaves open: what the closure forbids is this kit reaching out.
#
# NOTHING HERE KINDLES, and nothing here reads a constant of its own. Every
# function takes what it needs as an argument, so a door states its own
# coordinates and this module holds no opinion about where a repository, an
# election or a pin file stands. That is what lets the kennelman, which
# deliberately reaches no kennel door, share these readings with the whistle
# without either becoming the other's dependency.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKCP_SOURCED:-}" || return 0
ZBKCP_SOURCED=1

# The door law (BKSCL-Collar.adoc "Door Law"): every kennel door refuses a
# repository carrying uncommitted work, so that each record maps to a position.
#
# CLEANLINESS IS THE WHOLE REPOSITORY'S, untracked files included. Elected roots
# decide currency, a different question; they do not narrow this reading, because
# the record a commit makes is a record of the whole tree and a partial one would
# be a lie about the rest.
#
# Args: <repository>
bkcp_door_law_or_die() {
  local z_repo="$1"

  local z_standing=""
  z_standing=$(git -C "${z_repo}" status --porcelain) \
    || buc_die_now "Could not read the repository at ${z_repo} - the kennel refuses what it cannot survey"

  test -n "${z_standing}" || return 0

  buc_warn "The repository carries uncommitted work:"
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    buc_warn "  ${z_line}"
  done <<< "${z_standing}"

  # GIT'S OWN WORD, not the estate's, and the crate's own guard already reasons
  # it out (src/bkcf_guard.rs): the kennel ships as source and is read by
  # consumers who hold no part of this estate's vocabulary, so the act is named
  # in the dialect anyone with a repository already speaks. What changed here is
  # only that the bash may no longer say it the other way - the estate's word is
  # generated vocabulary homed in a kit no parcel carries, and the source line
  # reaching for it killed this bootstrap on every receiving station.
  buc_die_now "Every kennel door refuses an uncommitted repository, so that each record maps to a position - commit it and drive again"
}

# Read the channel a repository pins, from its own pin file.
#
# Read and then STATED at the invocation, never inherited: a toolchain file is
# discovered from the working directory, and these doors run from the repository
# root where a pin standing at a subdirectory is out of scope entirely. A build
# that inherited its channel would be pinned only by coincidence, which is the
# rust-build memo's silently-inert-bump finding.
#
# Args: <pin_file>
bkcp_pin_capture() {
  local z_pin_file="$1"

  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" =~ ^channel[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
      test -n "${BASH_REMATCH[1]}" || return 1
      echo "${BASH_REMATCH[1]}"
      return 0
    fi
  done < "${z_pin_file}"

  return 1
}

# What a standing binary says it was built at, or nothing where it cannot say.
#
# THE ARTIFACT IS ASKED, never inferred from a timestamp. An mtime comparison
# answers a question about a file; this asks the binary itself, which is the only
# reading that stays true once the artifact has been copied, emplaced, or handed
# to a long-running process (BKSNC-Kennelcraft.adoc "Currency by git
# position").
#
# Homed here rather than in the whistle because the whistle is not its only
# reader: every door that DRIVES the kennel owes the same currency question the
# whistle asks before it execs, and a door that answered it differently would act
# on a binary the whistle would have rebuilt. A stale kennel does not fail — it
# succeeds under the discipline it carried when it was struck, which is the one
# failure a report cannot catch after the fact.
#
# THE CALLER OWES THE DOOR LAW FIRST. A kennel refusing an uncommitted repository
# never reaches the line that states its seat, so this answers empty on a dirty
# tree — which a caller reads as an outrun binary unless it has already refused
# for the true reason.
#
# Args: <binary>
bkcp_standing_seat_capture() {
  local z_binary="$1"

  test -x "${z_binary}" || return 0

  local z_report=""
  z_report=$("${z_binary}" 2>&1) || return 0

  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      *"] seat "*) echo "${z_line##* }"; return 0 ;;
    esac
  done <<< "${z_report}"
}

# The seat's position: its newest first-parent commit touching an elected root.
#
# WALKED OVER THE ELECTION, never bare HEAD. What stales an artifact is a change
# to what it is made from; bare HEAD would report a new position for every commit
# anywhere in the repository, so a binary that had not changed would read as
# outrun and be relinked for nothing.
#
# THE SEAT'S OWN LINE, never the trunk counterpart. Tools/buk/bue_exergue.sh
# walks landings so a reader holding a record rather than the repository can
# resolve its answer - the DELIVERED reading (BKSNC-Kennelcraft.adoc "The
# Binary Election"). A commit made at a billet never enters that walk, so a
# seat-built binary carrying it would sit still while its own source moved.
#
# The election's grammar is git's own - a directory reaches everything beneath
# it, an exclusion pathspec carves back out - so nothing here interprets a line.
#
# Args: <repository> <election_file>
bkcp_position_capture() {
  local z_repo="$1"
  local z_election="$2"

  local -a z_roots=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    z_line="${z_line%%#*}"
    z_line="${z_line#"${z_line%%[![:space:]]*}"}"
    z_line="${z_line%"${z_line##*[![:space:]]}"}"
    test -n "${z_line}" || continue
    z_roots+=("${z_line}")
  done < "${z_election}"

  test "${#z_roots[@]}" -gt 0 \
    || buc_die_now "The election at ${z_election} names no root - an empty election would measure the position over nothing"

  local z_position=""
  z_position=$(git -C "${z_repo}" log -n 1 --first-parent --format=%H HEAD -- "${z_roots[@]}") \
    || buc_die_now "Could not walk the seat's own line over the election in ${z_election}"

  test -n "${z_position}" \
    || buc_die_now "No commit on this seat's line touches any elected root - the election in ${z_election} names nothing this tree has ever carried"

  echo "${z_position}"
}

# eof
