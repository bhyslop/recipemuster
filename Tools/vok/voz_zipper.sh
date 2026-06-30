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
# VOZ Zipper - Colophon registry for VOK workbench dispatch
#
# Dispatch-only (no buz_tome): VOK colophons are consumed as Rust consts
# nowhere, and the generated RBTDGC_/BUWGC_ const blocks are emitted solely by
# rbz_generate_consts (rbte_engine), which never sources this zipper. Sibling
# of apcz; vow-r is not enrolled (raw-binary passthrough, hand-routed in the
# workbench).

set -euo pipefail

# Multiple inclusion guard
test -z "${ZVOZ_SOURCED:-}" || return 0
ZVOZ_SOURCED=1

######################################################################
# Colophon registry initialization

zvoz_kindle() {
  test -z "${ZVOZ_KINDLED:-}" || buc_die "voz already kindled"

  # Verify buz zipper is kindled (workbench kindles buz first)
  zbuz_sentinel

  # VOK management — build, test, and parcel release (vow-)
  local z_mod="vob_cli.sh"
  buz_group VOZ__GROUP_MANAGE "vow-" "VOK build, test, and parcel-release management"
  buz_enroll VOZ_BUILD    "vow-b" "${z_mod}" "vob_build"   ""        "Build the vvr binary and install to VVK bin"
  buz_enroll VOZ_CLEAN    "vow-c" "${z_mod}" "vob_clean"   ""        "Remove Rust build artifacts from kit target directories"
  buz_enroll VOZ_TEST     "vow-t" "${z_mod}" "vob_test"    ""        "Run all kit crate tests"
  buz_enroll VOZ_RELEASE  "vow-R" "${z_mod}" "vob_release" "imprint" "Build a VVK parcel for the imprinted kit set (full|buk-only|buk-jjk)"
  buz_enroll VOZ_FRESHEN  "vow-F" "${z_mod}" "vob_freshen" ""        "Freshen CLAUDE.md @-includes from installed kits"

  readonly ZVOZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zvoz_healthcheck() {
  zvoz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zvoz_sentinel() {
  test "${ZVOZ_KINDLED:-}" = "1" || buc_die "Module voz not kindled - call zvoz_kindle first"
}

# eof
