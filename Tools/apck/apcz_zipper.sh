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
# APCZ Zipper - Colophon registry for APCK workbench dispatch

set -euo pipefail

# Multiple inclusion guard
test -z "${ZAPCZ_SOURCED:-}" || return 0
ZAPCZ_SOURCED=1

######################################################################
# Colophon registry initialization

zapcz_kindle() {
  test -z "${ZAPCZ_KINDLED:-}" || buc_die "apcz already kindled"

  # Verify buz zipper is kindled
  zbuz_sentinel

  # Build — Tauri build and development (apcw-)
  buz_group APCZ__GROUP_BUILD   "apcw-"   "Build — Tauri build and development"
  buz_enroll APCZ_BUILD          "apcw-b"  "apcc_cli.sh" "apcc_build"         ""  "cargo tauri build (release)"
  buz_enroll APCZ_RUN            "apcw-r"  "apcc_cli.sh" "apcc_run"           ""  "cargo tauri dev (local development)"
  buz_enroll APCZ_DEPLOY         "apcw-d"  "apcc_cli.sh" "apcc_deploy"        ""  "Build + scp to anns-macbook-air"
  buz_enroll APCZ_FIXTURE_LOAD   "apcw-fl" "apcc_cli.sh" "apcc_fixture_load"  "imprint"  "Run apcal to load fixture HTML onto clipboard"
  buz_enroll APCZ_TEST           "apcw-t"  "apcc_cli.sh" "apcc_test"          ""  "cargo test in apcd/"
  buz_enroll APCZ_DICT_REFRESH  "apcw-dr" "apcc_cli.sh" "apcc_dictionary_refresh" "" "Refresh dictionaries from public sources"
  buz_enroll APCZ_BATCH_ASSAY  "apcw-ba" "apcc_cli.sh" "apcc_batch_assay"       "param1" "Batch assay — run detection pipeline on HTML directory"
  buz_enroll APCZ_NS_INSTALL   "apcw-nsi" "apcc_cli.sh" "apcc_neural_stanford_install" "" "Neural Stanford install — convergent venv + ONNX export (always reaches a working state)"
  buz_enroll APCZ_NS_ASSAY     "apcw-nsa" "apcc_cli.sh" "apcc_neural_stanford_assay"  "param1" "Neural Stanford assay — run apcnsa on HTML directory"

  readonly ZAPCZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zapcz_healthcheck() {
  zapcz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zapcz_sentinel() {
  test "${ZAPCZ_KINDLED:-}" = "1" || buc_die "Module apcz not kindled - call zapcz_kindle first"
}

# eof
