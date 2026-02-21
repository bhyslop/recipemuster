#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Common Constants - File paths and naming conventions

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCC_SOURCED:-}" || buc_die "Module rbcc multiply sourced - check sourcing hierarchy"
ZRBCC_SOURCED=1

# Literal constants (pure string literals, no variable expansion — available at source time)
RBCC_rbrr_file="rbrr.env"
RBCC_rbrp_file="rbrp.env"
RBCC_rbrn_prefix="rbrn_"
RBCC_rbrn_ext=".env"

######################################################################
# Internal Functions (zrbcc_*)

zrbcc_kindle() {
  test -z "${ZRBCC_KINDLED:-}" || buc_die "Module rbcc already kindled"
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR not set - rbcc requires BURC environment"

  # Kindle constants (depend on runtime state)
  RBCC_KIT_DIR="${BURC_TOOLS_DIR}/rbw"

  ZRBCC_KINDLED=1
}

zrbcc_sentinel() {
  test "${ZRBCC_KINDLED:-}" = "1" || buc_die "Module rbcc not kindled - call zrbcc_kindle first"
}

# eof
