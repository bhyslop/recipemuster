#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle Windows Handbook - Base module (kindle, sentinel, source guard)

set -euo pipefail

test -z "${ZRBHW_SOURCED:-}" || buc_die_now "Module rbhw multiply sourced - check sourcing hierarchy"
ZRBHW_SOURCED=1

zrbhw_kindle() {
  test -z "${ZRBHW_KINDLED:-}" || buc_die_now "Module rbhw already kindled"

  readonly ZRBHW_WSL_DISTRO="rbtww-main"
  readonly ZRBHW_DOCKER_CONTEXT="wsl-native"

  readonly ZRBHW_KINDLED=1
}

zrbhw_sentinel() {
  test "${ZRBHW_KINDLED:-}" = "1" || buc_die_now "Module rbhw not kindled - call zrbhw_kindle first"
}

# eof
