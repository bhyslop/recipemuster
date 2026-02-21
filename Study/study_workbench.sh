#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Study Workbench - Routes study commands
#
# Colophon pattern: study-{id}
# Imprint selects execution mode (e.g., smoke, FULL)

set -euo pipefail

# Get script directory (Study/)
STUDYW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${STUDYW_SCRIPT_DIR}/../Tools/buk/buc_command.sh"
source "${STUDYW_SCRIPT_DIR}/../Tools/buk/burd_regime.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zburd_kindle

# Verbose output if BURE_VERBOSE is set
studyw_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "STUDYWSHOW: $*"
}

# Build a study's Rust binary. Arg: study directory path.
# Returns the binary path via stdout.
studyw_build() {
  local z_study_dir="$1"
  local z_cargo_toml="${z_study_dir}/Cargo.toml"

  test -f "${z_cargo_toml}" || buc_die "No Cargo.toml at ${z_cargo_toml}"

  studyw_show "Building ${z_cargo_toml}"
  cargo build --manifest-path "${z_cargo_toml}" >&2

  # Extract binary name from Cargo.toml [[bin]] section
  local z_bin_name
  z_bin_name=$(grep '^name' "${z_cargo_toml}" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
  echo "${z_study_dir}/target/debug/${z_bin_name}"
}

# Route based on colophon and imprint
studyw_route() {
  local z_command="$1"
  shift

  studyw_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  local z_imprint="${BURD_TOKEN_3:-}"

  studyw_show "Imprint: '${z_imprint}'"

  case "${z_command}" in

    study-mpt)
      local z_study_dir="${STUDYW_SCRIPT_DIR}/study-model-prompt-tuning"
      local z_binary
      z_binary=$(studyw_build "${z_study_dir}")

      case "${z_imprint}" in
        smoke)  exec "${z_binary}" smoke ;;
        FULL)   exec "${z_binary}" run   ;;
        *)      exec "${z_binary}" "$@"  ;;
      esac
      ;;

    # Add future studies here:
    # study-xyz)
    #   ...
    #   ;;

    *)  buc_die "Unknown study command: ${z_command}" ;;
  esac
}

studyw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  studyw_route "${z_command}" "$@"
}

studyw_main "$@"

# eof
