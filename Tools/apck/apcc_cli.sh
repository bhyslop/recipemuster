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
# APCC CLI - Command implementations for APCK workbench dispatch

set -euo pipefail

ZAPCC_APCD_DIR="${BASH_SOURCE[0]%/*}/apcd"

apcc_build() {
  buc_step "Building Tauri app (release)"
  (cd "${ZAPCC_APCD_DIR}" && cargo tauri build)
}

apcc_run() {
  buc_step "Running Tauri app (development)"
  (cd "${ZAPCC_APCD_DIR}" && cargo run --bin apcap)
}

apcc_deploy() {
  buc_step "Building for deployment"
  (cd "${ZAPCC_APCD_DIR}" && cargo tauri build)

  local -r z_bundle_dir="${ZAPCC_APCD_DIR}/target/release/bundle/macos"
  local -r z_size_file="${BURD_TEMP_DIR}/apcc_deploy_size.txt"
  local -r z_size_stderr="${BURD_TEMP_DIR}/apcc_deploy_size_stderr.txt"
  local -r z_ssh_stderr="${BURD_TEMP_DIR}/apcc_deploy_ssh_stderr.txt"
  local -r z_scp_stderr="${BURD_TEMP_DIR}/apcc_deploy_scp_stderr.txt"

  # Find first .app bundle via glob
  local z_app_name=""
  for z_app_name in "${z_bundle_dir}"/*.app; do
    test -d "${z_app_name}" && break
    z_app_name=""
  done
  test -n "${z_app_name}" || buc_die "No .app bundle found in ${z_bundle_dir}"

  # Measure bundle size via temp file
  du -sh "${z_app_name}" > "${z_size_file}" 2>"${z_size_stderr}" \
    || buc_die "Failed to measure bundle — see ${z_size_stderr}"
  local -r z_size_raw=$(<"${z_size_file}")
  test -n "${z_size_raw}" || buc_die "Empty size output from ${z_size_file}"
  local -r z_bundle_size="${z_size_raw%%$'\t'*}"

  buc_step "Deploying to anns-macbook-air:/Users/Shared/apcua/"
  ssh anns-macbook-air 'rm -rf /Users/Shared/apcua/*.app' 2>"${z_ssh_stderr}" \
    || buc_die "Failed to clean staging dir — see ${z_ssh_stderr}"
  scp -r "${z_app_name}" anns-macbook-air:/Users/Shared/apcua/ 2>"${z_scp_stderr}" \
    || buc_die "Failed to deploy bundle — see ${z_scp_stderr}"
  buc_step "Deploy complete: ${z_app_name##*/} (${z_bundle_size}) → anns-macbook-air:/Users/Shared/apcua/"
}

apcc_fixture_load() {
  local z_folio="${BUZ_FOLIO:?fixture name required}"
  local z_fixture_dir="${BASH_SOURCE[0]%/*}/test_fixtures"
  local z_fixture_file
  case "${z_folio}" in
    progress)  z_fixture_file="${z_fixture_dir}/epic_progress_note.html" ;;
    geriatric) z_fixture_file="${z_fixture_dir}/epic_geriatric_consult.html" ;;
    *) buc_die "Unknown fixture: ${z_folio}" ;;
  esac
  test -f "${z_fixture_file}" || buc_die "Fixture file not found: ${z_fixture_file}"
  buc_step "Loading fixture '${z_folio}' onto clipboard"
  (cd "${ZAPCC_APCD_DIR}" && cargo run --bin apcal -- "${z_fixture_file}")
}

apcc_test() {
  buc_step "Running cargo test"
  (cd "${ZAPCC_APCD_DIR}" && cargo test)
}

# eof
