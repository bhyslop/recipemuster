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

source "${BURD_BUK_DIR}/buc_command.sh"

ZAPCC_APCD_DIR="${BASH_SOURCE[0]%/*}/apcd"
ZAPCC_MANIFEST="${ZAPCC_APCD_DIR}/Cargo.toml"

# Neural Stanford spike — model artifact + venv colocated outside project tree.
# Zap removes the whole directory so model and its exporter toolchain go together.
ZAPCC_STANFORD_DIR="/Users/bhyslop/models/stanford-deidentifier"
ZAPCC_STANFORD_VENV="${ZAPCC_STANFORD_DIR}/.venv"
ZAPCC_STANFORD_MODEL_ID="StanfordAIMI/stanford-deidentifier-base"

######################################################################
# Commands

apcc_build() {
  buc_step "Building Tauri app (release)"
  (
    cd "${ZAPCC_APCD_DIR}" || buc_die "Failed to cd to ${ZAPCC_APCD_DIR}"
    cargo tauri build     || buc_die "cargo tauri build failed"
  ) || buc_die "Build subshell failed"
}

apcc_run() {
  buc_step "Running Tauri app (development)"
  cargo run --bin apcap --manifest-path "${ZAPCC_MANIFEST}" \
    || buc_die "cargo run failed"
}

apcc_deploy() {
  buc_step "Building for deployment"
  (
    cd "${ZAPCC_APCD_DIR}" || buc_die "Failed to cd to ${ZAPCC_APCD_DIR}"
    cargo tauri build     || buc_die "cargo tauri build failed"
  ) || buc_die "Build subshell failed"

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
  cargo run --bin apcal --manifest-path "${ZAPCC_MANIFEST}" -- "${z_fixture_file}" \
    || buc_die "cargo run apcal failed"
}

apcc_test() {
  buc_step "Running cargo test"
  local -r z_output_file="${BURD_TEMP_DIR}/apcc_test_output.txt"
  cargo test --manifest-path "${ZAPCC_MANIFEST}" \
    > "${z_output_file}" 2>&1 \
    || { cat "${z_output_file}"; buc_die "cargo test failed"; }
  cat "${z_output_file}"
  local -r z_total_passed=$(grep -c '^test .* ok$' "${z_output_file}" || true)
  buc_step "Tests complete: ${z_total_passed} passed"
}

apcc_dictionary_refresh() {
  buc_step "Refreshing dictionaries from public sources"
  cargo run --bin apcad --manifest-path "${ZAPCC_MANIFEST}" \
    || buc_die "cargo run apcad failed"
}

apcc_batch_assay() {
  local z_folio="${BUZ_FOLIO:?input directory required}"
  buc_step "Running batch assay on ${z_folio}"
  cargo run --bin apcab --manifest-path "${ZAPCC_MANIFEST}" -- "${z_folio}" \
    || buc_die "cargo run apcab failed"
}

######################################################################
# Neural Stanford spike

apcc_neural_stanford_export() {
  command -v python3 >/dev/null \
    || buc_die "python3 not found on PATH — required for optimum-cli export"

  local -r z_model_dir="${ZAPCC_STANFORD_DIR}"
  local -r z_venv_dir="${ZAPCC_STANFORD_VENV}"
  local -r z_venv_python="${z_venv_dir}/bin/python"
  local -r z_optimum_cli="${z_venv_dir}/bin/optimum-cli"

  mkdir -p "${z_model_dir}" || buc_die "Failed to create ${z_model_dir}"

  if [[ ! -x "${z_venv_python}" ]]; then
    buc_step "Creating venv: ${z_venv_dir}"
    python3 -m venv "${z_venv_dir}" \
      || buc_die "python3 -m venv ${z_venv_dir} failed"
  fi

  # optimum 2.x moved ONNX export into a separate package (optimum-onnx).
  # Check for the onnx subcommand directly — presence of optimum-cli alone is insufficient.
  local z_onnx_registered=0
  if [[ -x "${z_optimum_cli}" ]]; then
    "${z_optimum_cli}" export --help 2>&1 | grep -q '^\s*onnx\b' && z_onnx_registered=1
  fi
  if [[ "${z_onnx_registered}" != "1" ]]; then
    buc_step "Installing optimum + optimum-onnx into venv"
    "${z_venv_python}" -m pip install --upgrade pip \
      || buc_die "pip upgrade failed"
    "${z_venv_python}" -m pip install optimum optimum-onnx onnxruntime \
      || buc_die "pip install optimum optimum-onnx onnxruntime failed"
  fi

  if [[ -f "${z_model_dir}/model.onnx" \
     && -f "${z_model_dir}/tokenizer.json" \
     && -f "${z_model_dir}/config.json" ]]; then
    buc_step "Model artifacts already present at ${z_model_dir} — skipping export"
    return 0
  fi

  buc_step "Exporting ${ZAPCC_STANFORD_MODEL_ID} to ${z_model_dir}"
  "${z_optimum_cli}" export onnx \
      --model "${ZAPCC_STANFORD_MODEL_ID}" \
      --task token-classification \
      "${z_model_dir}" \
    || buc_die "optimum-cli export onnx failed"

  buc_step "Export complete"
  ls -la "${z_model_dir}" || true
}

apcc_neural_stanford_zap() {
  local -r z_model_dir="${ZAPCC_STANFORD_DIR}"
  if [[ -d "${z_model_dir}" ]]; then
    buc_step "Removing ${z_model_dir}"
    rm -rf "${z_model_dir}" || buc_die "Failed to remove ${z_model_dir}"
  else
    buc_step "No artifacts to remove at ${z_model_dir}"
  fi
  apcc_neural_stanford_export
}

apcc_neural_stanford_assay() {
  local z_folio="${BUZ_FOLIO:?input directory required}"
  local -r z_model_dir="${ZAPCC_STANFORD_DIR}"
  for z_file in model.onnx tokenizer.json config.json; do
    test -f "${z_model_dir}/${z_file}" \
      || buc_die "Missing ${z_model_dir}/${z_file} — run apcw-nsx first"
  done
  buc_step "Running neural stanford assay on ${z_folio}"
  APCNSA_MODEL_DIR="${z_model_dir}" \
    cargo run --bin apcnsa --manifest-path "${ZAPCC_MANIFEST}" -- "${z_folio}" \
    || buc_die "cargo run apcnsa failed"
}

######################################################################
# Furnish and dispatch

zapcc_furnish() {
  buc_doc_env "BURD_TEMP_DIR" "Temporary directory (dispatch-provided)"
  buc_doc_env_done || return 0
}

buc_execute apcc_ "APCK CLI" zapcc_furnish "$@"

# eof
