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
# Recipe Bottle Regime Station - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRS_SOURCED:-}" || buc_die "Module rbrs multiply sourced - check sourcing hierarchy"
ZRBRS_SOURCED=1

######################################################################
# Internal Functions (zrbrs_*)

zrbrs_kindle() {
  test -z "${ZRBRS_KINDLED:-}" || buc_die "Module rbrs already kindled"

  buv_env_string      RBRS_PODMAN_ROOT_DIR         1     64
  buv_env_string      RBRS_VMIMAGE_CACHE_DIR       1     64
  buv_env_string      RBRS_VM_PLATFORM             1     64

  ZRBRS_KINDLED=1
}

zrbrs_sentinel() {
  test "${ZRBRS_KINDLED:-}" = "1" || buc_die "Module rbrs not kindled - call zrbrs_kindle first"
}

# eof
