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
# Recipe Bottle Regime GitHub - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRG_SOURCED:-}" || buc_die "Module rbrg multiply sourced - check sourcing hierarchy"
ZRBRG_SOURCED=1

######################################################################
# Internal Functions (zrbrg_*)

zrbrg_kindle() {
  test -z "${ZRBRG_KINDLED:-}" || buc_die "Module rbrg already kindled"

  buv_env_string      RBRG_PAT                    40    100
  buv_env_xname       RBRG_USERNAME                1     39

  ZRBRG_KINDLED=1
}

zrbrg_sentinel() {
  test "${ZRBRG_KINDLED:-}" = "1" || buc_die "Module rbrg not kindled - call zrbrg_kindle first"
}

# eof
