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

# Source RBBC bootstrap constants (source-time, before kindle)
test -n "${BURD_CONFIG_DIR:-}" || buc_die "BURD_CONFIG_DIR not set - rbcc requires launcher environment"
source "${BURD_CONFIG_DIR}/rbbc_constants.sh" || buc_die "Failed to source rbbc_constants.sh"

# Literal constants (pure string literals, no variable expansion — available at source time)
RBCC_rbrs_file="../station-files/rbrs.env"
RBCC_rbrn_prefix="rbrn_"
RBCC_rbrn_ext=".env"

######################################################################
# Internal Functions (zrbcc_*)

zrbcc_kindle() {
  test -z "${ZRBCC_KINDLED:-}" || buc_die "Module rbcc already kindled"
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR not set - rbcc requires BURC environment"

  # Kindle constants (depend on runtime state)
  readonly RBCC_KIT_DIR="${BURC_TOOLS_DIR}/rbw"

  # Curl timeout bounds — all actionable curl sites use these
  readonly RBCC_CURL_CONNECT_TIMEOUT_SEC=10
  readonly RBCC_CURL_MAX_TIME_SEC=60

  # Bottle service readiness — max seconds to wait for HTTP 200 after rbob_start
  readonly RBCC_GENERIC_SERVICE_START_SECONDS=30

  # Fact-file infix for per-vessel-consecration presence files
  # Composed as: ${vessel}${RBCC_FACT_CONSEC_INFIX}${consecration}
  # Producer: rbf_check_consecrations  Consumer: test cases via test -f
  readonly RBCC_FACT_CONSEC_INFIX="_fact_consec_"

  readonly ZRBCC_KINDLED=1
}

zrbcc_sentinel() {
  test "${ZRBCC_KINDLED:-}" = "1" || buc_die "Module rbcc not kindled - call zrbcc_kindle first"
}

# eof
