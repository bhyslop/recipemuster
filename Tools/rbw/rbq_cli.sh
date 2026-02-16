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
# RBQ CLI - Command line interface for RBQ qualification operations

set -euo pipefail

ZRBQ_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBQ_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/rbz_zipper.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/rbrn_regime.sh"
source "${ZRBQ_CLI_SCRIPT_DIR}/rbq_Qualify.sh"

######################################################################
# CLI Functions

zrbq_cli_kindle() {
  test -z "${ZRBQ_CLI_KINDLED:-}" || buc_die "RBQ CLI already kindled"

  zbuz_kindle
  zrbz_kindle
  zrbcc_kindle

  zrbq_kindle

  ZRBQ_CLI_KINDLED=1
}

######################################################################
# Main dispatch

buc_context "${0##*/}"

z_command="${1:-}"

case "${z_command}" in
  qualify_all)
    zrbq_cli_kindle
    rbq_qualify_all
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbq_cli.sh {qualify_all}"
    ;;
esac

# eof
