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
# Recipe Bottle Windows Handbook - Base module (kindle, sentinel, source guard)

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHW_SOURCED:-}" || buc_die "Module rbhw multiply sourced - check sourcing hierarchy"
ZRBHW_SOURCED=1

######################################################################
# Internal: Kindle and Sentinel

zrbhw_kindle() {
  test -z "${ZRBHW_KINDLED:-}" || buc_die "Module rbhw already kindled"

  # Kindle — project-specific constants for Windows test infrastructure
  readonly ZRBHW_WSL_DISTRO="rbtww-main"
  readonly ZRBHW_DOCKER_CONTEXT="wsl-native"

  readonly ZRBHW_KINDLED=1
}

zrbhw_sentinel() {
  test "${ZRBHW_KINDLED:-}" = "1" || buc_die "Module rbhw not kindled - call zrbhw_kindle first"
}

# eof
