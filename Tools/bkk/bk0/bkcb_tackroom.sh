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
# refusal it was written to give (BKSNC-Kennelcraft.adoc "The form of each
# launcher"). So the binding the bootstrap actually needs is stated here, under
# this kit's own head, and this file names nothing outside the kit and the
# substrate.
#
# THREE ACTS AND NO FOURTH. Read the station's declaration, refuse one that names
# the station user's own homes, export the two homes under it. That is the whole
# of the fence the leash stands on.
#
# IT PROVISIONS NOTHING, and that is an election rather than an omission
# (BKSNC-Kennelcraft.adoc "Open Elections"): a routine invocation verifies
# and refuses, and never surprises the operator with a download. A station whose
# tackroom lacks the pinned channel is told so by name, with the deliberate
# converge spelled out for it to run. What the estate's own trees additionally do
# — fetch a channel on first use, under a directory lock — is theirs and stays
# theirs; a kit that provisioned on a receiving station would be reaching for the
# network from a bootstrap the operator ran to build one binary.
#
# THE GUARD IS LEXICAL AND RUNS BEFORE ANYTHING IS CREATED. What it catches is a
# declaration that would build the store in the operator's personal rustup or
# cargo homes, and creating those directories and then complaining is worse than
# useless - the mess outlives the refusal.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKCB_SOURCED:-}" || return 0
ZBKCB_SOURCED=1

zbkcb_kindle() {
  test -z "${ZBKCB_KINDLED:-}" || buc_die_now "Module bkcb already kindled"

  # Carried across the dispatch exec boundary by the substrate. Empty here is not
  # fatal at kindle: bkcb_ensure is the one entry point that goes near a
  # toolchain, and that is where the requirement bites.
  readonly ZBKCB_TACKROOM="${BURD_TACKROOM:-}"

  readonly BKCB_CARGO_HOME="${ZBKCB_TACKROOM}/cargo"
  readonly BKCB_RUSTUP_HOME="${ZBKCB_TACKROOM}/rustup"

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
  local z_declared="$1"

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

# Bind the toolchain homes to the station's tackroom. Call before any cargo or
# rustup invocation.
bkcb_ensure() {
  zbkcb_sentinel

  test -n "${ZBKCB_TACKROOM}" \
    || buc_die_now "BURS_TACKROOM is unset - the kennel homes the pinned Rust toolchain and the cargo registry in the station's tackroom and will not fall back to a per-clone copy. Declare it in the station file (inspect with tt/buw-rsr.RenderStationRegime.sh), e.g. BURS_TACKROOM=../tackroom-buk"

  zbkcb_admissible_or_die "${ZBKCB_TACKROOM}"

  export CARGO_HOME="${BKCB_CARGO_HOME}"
  export RUSTUP_HOME="${BKCB_RUSTUP_HOME}"
}

# Verify the pinned channel stands in the tackroom, and refuse naming the
# converge where it does not.
#
# THE FENCE PROVISIONS NOTHING, so this is the whole of what a station short a
# channel is told: the exact command, to run deliberately. Refused HERE rather
# than left to cargo, which would die about a toolchain where the operator's
# actual question is about an election.
#
# Args: <channel>
bkcb_channel_present_or_die() {
  zbkcb_sentinel
  local z_channel="$1"

  local z_installed=""
  z_installed=$(RUSTUP_HOME="${BKCB_RUSTUP_HOME}" rustup toolchain list) \
    || buc_die_now "Could not list the tackroom's toolchains under ${BKCB_RUSTUP_HOME}"

  case "${z_installed}" in
    *"${z_channel}"*) return 0 ;;
  esac

  buc_warn "The kennel pins ${z_channel}, which the tackroom does not hold."
  buc_warn "A routine invocation verifies and refuses; it never downloads unasked."
  buc_warn "The converge, run deliberately:"
  buc_warn "  RUSTUP_HOME=${BKCB_RUSTUP_HOME} rustup toolchain install ${z_channel} --profile minimal --component clippy --component rustfmt"
  buc_die_now "The pinned channel ${z_channel} is absent from the tackroom"
}

# eof
