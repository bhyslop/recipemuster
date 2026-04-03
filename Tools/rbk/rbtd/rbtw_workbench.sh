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
# RBTW Workbench - Routes theurge colophons to RBTE CLI
#
# Pure router: no inline logic, no fixture knowledge, no kindle chains.
# Fully orthogonal from VOW (vvk/jjk/cmk kit pipeline).
# Theurge is Recipe Bottle's own test infrastructure — permanently part of rbk.

set -euo pipefail

RBTW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

source "${BURD_BUK_DIR}/buc_command.sh"

buc_context "${0##*/}"

# Source CLI (performs light furnish: buv + burd + buz + rbz + rbte kindle)
source "${RBTW_SCRIPT_DIR}/rbte_cli.sh"

rbtw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbte_dispatch "${z_command}" "$@"
}

rbtw_main "$@"

# eof
