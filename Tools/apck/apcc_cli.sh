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

# Container build context + runtime constants. Bind-mount target lives under
# $HOME so the container writes land in the same journal directory the Tauri
# app reads from (apcrj_journal_path()).
ZAPCC_CONTAINER_DIR="${BASH_SOURCE[0]%/*}/apcd/container"
ZAPCC_CONTAINER_IMAGE="apck-container:local"
ZAPCC_CONTAINER_NAME="apck-container"
ZAPCC_CONTAINER_BINDMOUNT="/work/apcjd"

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
  # find (not rm *.app) because Ann's remote shell is zsh, which errors on
  # empty glob match (NOMATCH) — breaks the first-deploy case when the dir is empty.
  ssh anns-macbook-air 'mkdir -p /Users/Shared/apcua && find /Users/Shared/apcua -maxdepth 1 -name "*.app" -exec rm -rf {} +' 2>"${z_ssh_stderr}" \
    || buc_die "Failed to clean staging dir — see ${z_ssh_stderr}"
  scp -r "${z_app_name}" anns-macbook-air:/Users/Shared/apcua/ 2>"${z_scp_stderr}" \
    || buc_die "Failed to deploy bundle — see ${z_scp_stderr}"
  buc_step "Deploy complete: ${z_app_name##*/} (${z_bundle_size}) → anns-macbook-air:/Users/Shared/apcua/"

  local -r z_app_basename="${z_app_name##*/}"
  printf '%s\n' '' \
                '=== Forward to Ann ===' \
                '' \
                'A new Clipbuddy build is staged on your Mac.' \
                '' \
                'To run it:' \
                '  1. If Clipbuddy is already running, quit it (Cmd-Q).' \
                "  2. In Finder: Go → Go to Folder… → /Users/Shared/apcua/" \
                "     Double-click ${z_app_basename}" \
                '' \
                '  Or from Terminal:' \
                "    open \"/Users/Shared/apcua/${z_app_basename}\"" \
                '' \
                '=== End ==='
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

apcc_neural_stanford_install() {
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

  # Convergent install: clear any prior model artifacts (preserving the venv)
  # and re-export from scratch. Each invocation yields a fresh, valid model dir.
  # The HuggingFace cache at ~/.cache/huggingface/ keeps the weight download fast
  # on repeat runs; the ONNX trace is the only re-computed step.
  buc_step "Clearing prior model artifacts from ${z_model_dir}"
  find "${z_model_dir}" -mindepth 1 -maxdepth 1 ! -name '.venv' -exec rm -rf {} + \
    || buc_die "Failed to clear ${z_model_dir}"

  buc_step "Exporting ${ZAPCC_STANFORD_MODEL_ID} to ${z_model_dir}"
  "${z_optimum_cli}" export onnx \
      --model "${ZAPCC_STANFORD_MODEL_ID}" \
      --task token-classification \
      "${z_model_dir}" \
    || buc_die "optimum-cli export onnx failed"

  buc_step "Install complete"
  ls -la "${z_model_dir}" || true
}

apcc_neural_stanford_assay() {
  local z_folio="${BUZ_FOLIO:?input directory required}"
  local -r z_model_dir="${ZAPCC_STANFORD_DIR}"
  for z_file in model.onnx tokenizer.json config.json; do
    test -f "${z_model_dir}/${z_file}" \
      || buc_die "Missing ${z_model_dir}/${z_file} — run apcw-nsi first"
  done
  buc_step "Running neural stanford assay on ${z_folio}"
  APCNSA_MODEL_DIR="${z_model_dir}" \
    cargo run --bin apcnsa --manifest-path "${ZAPCC_MANIFEST}" -- "${z_folio}" \
    || buc_die "cargo run apcnsa failed"
}

######################################################################
# Container lifecycle

apcc_container_build() {
  command -v docker >/dev/null \
    || buc_die "docker not found on PATH"
  test -f "${ZAPCC_CONTAINER_DIR}/Dockerfile" \
    || buc_die "Dockerfile not found at ${ZAPCC_CONTAINER_DIR}"

  buc_step "Building ${ZAPCC_CONTAINER_IMAGE} (first build pulls ML models — ~5-10 min)"
  docker build -t "${ZAPCC_CONTAINER_IMAGE}" "${ZAPCC_CONTAINER_DIR}" \
    || buc_die "docker build failed"
  buc_step "Build complete"
}

apcc_container_start() {
  command -v docker >/dev/null \
    || buc_die "docker not found on PATH"

  local -r z_journal_dir="${HOME}/apcjd"
  mkdir -p "${z_journal_dir}" \
    || buc_die "Failed to create journal dir ${z_journal_dir}"

  # Truncate container log so each session starts with a clean journal.
  : > "${z_journal_dir}/container-log.txt" \
    || buc_die "Failed to truncate ${z_journal_dir}/container-log.txt"

  # Best-effort cleanup of any stopped predecessor.
  docker rm -f "${ZAPCC_CONTAINER_NAME}" >/dev/null 2>&1 || true

  buc_step "Starting ${ZAPCC_CONTAINER_NAME} (--network=none, --cap-drop=all, --read-only, non-root)"
  docker run -d \
    --name "${ZAPCC_CONTAINER_NAME}" \
    --network=none \
    --cap-drop=all \
    --read-only \
    --tmpfs /tmp \
    --user nobody:nogroup \
    -v "${z_journal_dir}:${ZAPCC_CONTAINER_BINDMOUNT}" \
    -e "APCS_BINDMOUNT=${ZAPCC_CONTAINER_BINDMOUNT}" \
    "${ZAPCC_CONTAINER_IMAGE}" \
    || buc_die "docker run failed"
  buc_step "Container started — model load runs for ~30-90s before discerners are ready"
}

apcc_container_stop() {
  command -v docker >/dev/null \
    || buc_die "docker not found on PATH"

  if docker inspect "${ZAPCC_CONTAINER_NAME}" >/dev/null 2>&1; then
    buc_step "Stopping ${ZAPCC_CONTAINER_NAME}"
    docker stop "${ZAPCC_CONTAINER_NAME}" >/dev/null \
      || buc_die "docker stop failed"
    docker rm "${ZAPCC_CONTAINER_NAME}" >/dev/null \
      || buc_die "docker rm failed"
    buc_step "Container stopped and removed"
  else
    buc_step "Container ${ZAPCC_CONTAINER_NAME} not present — nothing to stop"
  fi
}

apcc_container_status() {
  command -v docker >/dev/null \
    || buc_die "docker not found on PATH"

  buc_step "Container status"
  docker ps --all --filter "name=^/${ZAPCC_CONTAINER_NAME}$" \
        --format 'table {{.Names}}\t{{.Status}}\t{{.Image}}' \
    || buc_die "docker ps failed"

  buc_step "Bind-mount reachability"
  local -r z_journal_dir="${HOME}/apcjd"
  if test -d "${z_journal_dir}"; then
    echo "journal-dir: ${z_journal_dir} present"
  else
    echo "journal-dir: ${z_journal_dir} ABSENT"
  fi
  local -r z_log="${z_journal_dir}/container-log.txt"
  if test -r "${z_log}"; then
    echo "container-log.txt: $(wc -l < "${z_log}") lines"
    echo "--- last 5 log lines ---"
    tail -n 5 "${z_log}" || true
  else
    echo "container-log.txt: not readable (container may not have started yet)"
  fi
}

######################################################################
# Furnish and dispatch

zapcc_furnish() {
  buc_doc_env "BURD_TEMP_DIR" "Temporary directory (dispatch-provided)"
  buc_doc_env_done || return 0
}

buc_execute apcc_ "APCK CLI" zapcc_furnish "$@"

# eof
