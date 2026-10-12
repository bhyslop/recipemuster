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
# Recipe Bottle Handbook Payor - Base (kindle, sentinel, enforce)

set -euo pipefail

test -z "${ZRBHP0_SOURCED:-}" || buc_die_now "Module rbhp multiply sourced - check sourcing hierarchy"
ZRBHP0_SOURCED=1

zrbhp0_kindle() {
  test -z "${ZRBHP0_KINDLED:-}" || buc_die_now "Module rbhp already kindled"

  # Kernel discrimination for click-modifier display via the bash $OSTYPE
  # builtin (darwin* on macOS) — no external uname dependency.
  case "${OSTYPE}" in
    darwin*) readonly ZRBHP0_CLICK_MOD="Cmd" ;;
    *)       readonly ZRBHP0_CLICK_MOD="Ctrl" ;;
  esac

  readonly ZRBHP0_RBRP_FILE="${RBCC_rbrp_file}"
  readonly ZRBHP0_RBRP_FILE_BASENAME="${ZRBHP0_RBRP_FILE##*/}"

  readonly ZRBHP0_KINDLED=1
}

zrbhp0_sentinel() {
  test "${ZRBHP0_KINDLED:-}" = "1" || buc_die_now "Module rbhp not kindled - call zrbhp0_kindle first"
}

zrbhp0_enforce() {
  zrbhp0_sentinel
  test -n "${RBDC_DEPOT_PROJECT_ID:-}"     || buc_die_now "RBDC_DEPOT_PROJECT_ID is not set"
  zrbgc_sentinel
}

# eof
