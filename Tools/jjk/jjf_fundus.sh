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
# JJF Fundus - Fundus test account provisioning and scenario dispatch

set -euo pipefail

# Multiple inclusion detection
test -z "${ZJJF_SOURCED:-}" || buc_die "Module jjf multiply sourced - check sourcing hierarchy"
ZJJF_SOURCED=1

######################################################################
# Internal Functions (zjjf_*)

zjjf_kindle() {
  test -z "${ZJJF_KINDLED:-}" || buc_die "Module jjf already kindled"

  test -n "${BURD_TEMP_DIR:-}"  || buc_die "BURD_TEMP_DIR is unset"
  test -n "${BURD_TOOLS_DIR:-}" || buc_die "BURD_TOOLS_DIR is unset"

  readonly ZJJF_SCRIPT_DIR="${BURD_TOOLS_DIR}/jjk"
  readonly ZJJF_MANIFEST_PATH="${ZJJF_SCRIPT_DIR}/vov_veiled/Cargo.toml"
  readonly ZJJF_RELDIR="projects/rbm_alpha_recipemuster"
  readonly ZJJF_ACCOUNTS="jjfu_full jjfu_nokey jjfu_norepo jjfu_nogit"
  readonly ZJJF_TEMP_PREFIX="${BURD_TEMP_DIR}/jjf_"

  readonly ZJJF_KINDLED=1
}

zjjf_sentinel() {
  test "${ZJJF_KINDLED:-}" = "1" || buc_die "Module jjf not kindled - call zjjf_kindle first"
}

######################################################################
# Internal helpers — platform detection

zjjf_detect_platform() {
  zjjf_sentinel

  local -r z_uname_file="${ZJJF_TEMP_PREFIX}uname.txt"
  uname -s > "${z_uname_file}" || buc_die "Failed to detect platform"
  local -r z_uname=$(<"${z_uname_file}")
  test -n "${z_uname}" || buc_die "Empty uname output"

  case "${z_uname}" in
    Darwin) echo "macos" ;;
    Linux)  echo "linux" ;;
    *)      buc_die "Unsupported platform: ${z_uname}" ;;
  esac
}

zjjf_home_dir() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  case "${z_platform}" in
    macos) echo "/Users/${z_user}" ;;
    linux) echo "/home/${z_user}" ;;
  esac
}

zjjf_primary_group() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  case "${z_platform}" in
    macos) echo "staff" ;;
    linux) echo "${z_user}" ;;
  esac
}

######################################################################
# Internal helpers — account management

zjjf_delete_account() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  test -n "${z_user}" || buc_die "zjjf_delete_account: user required"

  if ! id "${z_user}" >/dev/null 2>&1; then
    buc_log_args "Account ${z_user} does not exist — skip delete"
    return 0
  fi

  buc_step "Deleting account: ${z_user}"
  local -r z_stderr="${ZJJF_TEMP_PREFIX}delete_${z_user}_stderr.txt"
  case "${z_platform}" in
    macos)
      sudo sysadminctl -deleteUser "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to delete macOS account: ${z_user} — see ${z_stderr}"
      if test -d "/Users/${z_user}"; then
        sudo rm -rf "/Users/${z_user}" \
          || buc_die "Failed to remove residual home dir: /Users/${z_user}"
      fi
      ;;
    linux)
      sudo userdel -r "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to delete Linux account: ${z_user} — see ${z_stderr}"
      if test -d "/home/${z_user}"; then
        sudo rm -rf "/home/${z_user}" \
          || buc_die "Failed to remove residual home dir: /home/${z_user}"
      fi
      ;;
  esac
  buc_log_args "Account ${z_user} deleted"
}

zjjf_create_account() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  test -n "${z_user}" || buc_die "zjjf_create_account: user required"

  buc_step "Creating account: ${z_user}"
  local -r z_stderr="${ZJJF_TEMP_PREFIX}create_${z_user}_stderr.txt"
  case "${z_platform}" in
    macos)
      local -r z_pw_file="${ZJJF_TEMP_PREFIX}password.txt"
      openssl rand -base64 16 > "${z_pw_file}" \
        || buc_die "Failed to generate random password"
      local -r z_password=$(<"${z_pw_file}")
      test -n "${z_password}" || buc_die "Generated password is empty"

      sudo sysadminctl -addUser "${z_user}" \
        -password "${z_password}" \
        -home "/Users/${z_user}" \
        -shell /bin/bash \
        2>"${z_stderr}" \
        || buc_die "Failed to create macOS account: ${z_user} — see ${z_stderr}"
      ;;
    linux)
      sudo useradd -m -s /bin/bash "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to create Linux account: ${z_user} — see ${z_stderr}"
      sudo passwd -l "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to lock password for: ${z_user} — see ${z_stderr}"
      ;;
  esac
  buc_log_args "Account ${z_user} created"
}

######################################################################
# Internal helpers — SSH key authorization

zjjf_authorize_ssh() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  local -r z_pubkey="${3:-}"
  test -n "${z_user}"   || buc_die "zjjf_authorize_ssh: user required"
  test -n "${z_pubkey}"  || buc_die "zjjf_authorize_ssh: pubkey required"

  buc_step "Authorizing SSH for: ${z_user}"

  local -r z_home="$(zjjf_home_dir "${z_platform}" "${z_user}")"
  local -r z_ssh_dir="${z_home}/.ssh"
  local -r z_group="$(zjjf_primary_group "${z_platform}" "${z_user}")"

  sudo mkdir -p "${z_ssh_dir}" \
    || buc_die "Failed to create .ssh dir for: ${z_user}"
  echo "${z_pubkey}" | sudo tee "${z_ssh_dir}/authorized_keys" >/dev/null \
    || buc_die "Failed to write authorized_keys for: ${z_user}"
  sudo chmod 700 "${z_ssh_dir}" \
    || buc_die "Failed to chmod .ssh for: ${z_user}"
  sudo chmod 600 "${z_ssh_dir}/authorized_keys" \
    || buc_die "Failed to chmod authorized_keys for: ${z_user}"
  sudo chown -R "${z_user}:${z_group}" "${z_ssh_dir}" \
    || buc_die "Failed to chown .ssh for: ${z_user}"

  buc_log_args "SSH authorized for ${z_user}"
}

######################################################################
# Internal helpers — repository and BUK setup

zjjf_setup_repo() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  local -r z_with_origin="${3:-1}"
  local -r z_curia_origin="${4:-}"
  test -n "${z_user}" || buc_die "zjjf_setup_repo: user required"

  buc_step "Setting up repo for: ${z_user} (origin=${z_with_origin})"

  local -r z_home="$(zjjf_home_dir "${z_platform}" "${z_user}")"
  local -r z_project_dir="${z_home}/${ZJJF_RELDIR}"
  local -r z_project_parent="${z_project_dir%/*}"
  local -r z_stderr="${ZJJF_TEMP_PREFIX}repo_${z_user}_stderr.txt"

  # Resolve curia repo absolute path
  local -r z_curia_root_file="${ZJJF_TEMP_PREFIX}curia_root.txt"
  (cd "${BURD_TOOLS_DIR}/.." && pwd) > "${z_curia_root_file}" \
    || buc_die "Failed to resolve curia repo root"
  local -r z_curia_repo=$(<"${z_curia_root_file}")
  test -n "${z_curia_repo}" || buc_die "Empty curia repo root"

  sudo -u "${z_user}" mkdir -p "${z_project_parent}" \
    || buc_die "Failed to create projects dir for: ${z_user}"

  sudo -u "${z_user}" git clone "${z_curia_repo}" "${z_project_dir}" 2>"${z_stderr}" \
    || buc_die "Failed to clone repo for: ${z_user} — see ${z_stderr}"

  if test "${z_with_origin}" = "1"; then
    test -n "${z_curia_origin}" || buc_die "zjjf_setup_repo: curia_origin required when with_origin=1"
    sudo -u "${z_user}" git -C "${z_project_dir}" remote set-url origin "${z_curia_origin}" 2>"${z_stderr}" \
      || buc_die "Failed to set origin for: ${z_user} — see ${z_stderr}"
    buc_log_args "Repo cloned with origin: ${z_curia_origin}"
  else
    sudo -u "${z_user}" git -C "${z_project_dir}" remote remove origin 2>"${z_stderr}" \
      || buc_die "Failed to remove origin for: ${z_user} — see ${z_stderr}"
    buc_log_args "Repo cloned with origin removed"
  fi

  zjjf_install_buk "${z_platform}" "${z_user}" "${z_project_dir}" "${z_curia_repo}"
}

zjjf_install_buk() {
  zjjf_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  local -r z_project_dir="${3:-}"
  local -r z_curia_repo="${4:-}"

  buc_step "Installing BUK for: ${z_user}"

  local -r z_buk_dir="${z_project_dir}/.buk"
  local -r z_group="$(zjjf_primary_group "${z_platform}" "${z_user}")"

  sudo -u "${z_user}" mkdir -p "${z_buk_dir}" \
    || buc_die "Failed to create .buk dir for: ${z_user}"

  # Write burc.env — same relative paths as curia
  sudo -u "${z_user}" tee "${z_buk_dir}/burc.env" >/dev/null <<'BURC'
BURC_STATION_FILE=../station-files/burs.env
BURC_TABTARGET_DIR=tt
BURC_TABTARGET_DELIMITER=.
BURC_TOOLS_DIR=Tools
BURC_PROJECT_ROOT=..
BURC_MANAGED_KITS=buk,cmk,jjk,vvk
BURC_TEMP_ROOT_DIR=../temp-buk
BURC_OUTPUT_ROOT_DIR=../output-buk
BURC_LOG_LAST=last
BURC_LOG_EXT=txt
BURC

  # Copy launcher files from curia
  local -r z_curia_buk="${z_curia_repo}/.buk"
  local z_launcher=""
  for z_launcher in "${z_curia_buk}"/launcher.*.sh; do
    test -f "${z_launcher}" || continue
    local z_launcher_name="${z_launcher##*/}"
    sudo cp "${z_launcher}" "${z_buk_dir}/${z_launcher_name}" \
      || buc_die "Failed to copy launcher: ${z_launcher_name}"
  done

  sudo chown -R "${z_user}:${z_group}" "${z_buk_dir}" \
    || buc_die "Failed to chown .buk for: ${z_user}"
  sudo chmod -R u+x "${z_buk_dir}" \
    || buc_die "Failed to chmod .buk for: ${z_user}"

  buc_log_args "BUK installed for ${z_user}"
}

######################################################################
# External Functions (jjf_*)

jjf_provision() {
  zjjf_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  test -n "${z_host}" || buc_die "jjf_provision: no host (BUZ_FOLIO empty)"

  buc_doc_brief "Provision all 4 jjfu_* fundus test accounts per JJSTF"
  buc_doc_shown || return 0

  # Remote provisioning: SSH to host and run the localhost tabtarget there
  if test "${z_host}" != "localhost"; then
    buc_step "Provisioning fundus accounts on remote host: ${z_host}"
    local -r z_remote_stderr="${ZJJF_TEMP_PREFIX}remote_provision_stderr.txt"
    ssh "${z_host}" "cd ~/${ZJJF_RELDIR} && tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh" \
      2>"${z_remote_stderr}" \
      || buc_die "Remote provisioning failed on ${z_host} — see ${z_remote_stderr}"
    buc_success "Remote provisioning complete on ${z_host}"
    return 0
  fi

  # Local provisioning
  buc_step "Provisioning fundus test accounts on localhost"

  # Discover platform
  local z_platform=""
  z_platform=$(zjjf_detect_platform) || buc_die "Platform detection failed"

  # Discover curia SSH public key
  local -r z_ed25519="${HOME}/.ssh/id_ed25519.pub"
  local -r z_rsa="${HOME}/.ssh/id_rsa.pub"
  local z_pubkey=""
  if test -f "${z_ed25519}"; then
    z_pubkey=$(<"${z_ed25519}")
  elif test -f "${z_rsa}"; then
    z_pubkey=$(<"${z_rsa}")
  else
    buc_die "No SSH public key found at ${z_ed25519} or ${z_rsa}"
  fi
  test -n "${z_pubkey}" || buc_die "SSH public key file is empty"
  buc_log_args "Curia SSH key found"

  # Discover git origin URL
  local -r z_origin_file="${ZJJF_TEMP_PREFIX}origin.txt"
  git -C "${BURD_TOOLS_DIR}/.." remote get-url origin > "${z_origin_file}" 2>/dev/null \
    || buc_die "Failed to get git origin URL from curia repo"
  local -r z_curia_origin=$(<"${z_origin_file}")
  test -n "${z_curia_origin}" || buc_die "Git origin URL is empty"
  buc_log_args "Curia origin: ${z_curia_origin}"

  # Phase 1: Delete all existing accounts (clean slate)
  buc_step "Phase 1: Deleting existing accounts"
  local z_acct=""
  for z_acct in ${ZJJF_ACCOUNTS}; do
    zjjf_delete_account "${z_platform}" "${z_acct}"
  done

  # Phase 2: Create accounts with correct precondition shapes per JJSTF
  buc_step "Phase 2: Creating accounts"

  # jjfu_full: SSH + repo + BUK + origin
  zjjf_create_account  "${z_platform}" "jjfu_full"
  zjjf_authorize_ssh   "${z_platform}" "jjfu_full" "${z_pubkey}"
  zjjf_setup_repo      "${z_platform}" "jjfu_full" 1 "${z_curia_origin}"

  # jjfu_nokey: OS account only, no SSH key access from curia
  zjjf_create_account  "${z_platform}" "jjfu_nokey"

  # jjfu_norepo: SSH access, no project directory at RELDIR
  zjjf_create_account  "${z_platform}" "jjfu_norepo"
  zjjf_authorize_ssh   "${z_platform}" "jjfu_norepo" "${z_pubkey}"

  # jjfu_nogit: SSH + repo + BUK, origin removed
  zjjf_create_account  "${z_platform}" "jjfu_nogit"
  zjjf_authorize_ssh   "${z_platform}" "jjfu_nogit" "${z_pubkey}"
  zjjf_setup_repo      "${z_platform}" "jjfu_nogit" 0

  buc_success "All 4 fundus accounts provisioned on localhost"
}

jjf_scenario() {
  zjjf_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  test -n "${z_host}" || buc_die "jjf_scenario: no host (BUZ_FOLIO empty)"

  buc_doc_brief "Run all fundus scenario profiles against a host"
  buc_doc_shown || return 0

  export JJTEST_HOST="${z_host}"
  exec cargo test \
    --manifest-path "${ZJJF_MANIFEST_PATH}" \
    --test fundus_scenario \
    -- --test-threads=1 --ignored
}

jjf_single() {
  zjjf_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  local -r z_test="${1:-}"
  test -n "${z_host}" || buc_die "jjf_single: no host (BUZ_FOLIO empty)"
  test -n "${z_test}" || buc_die "jjf_single: no test function name (pass as argument)"

  buc_doc_brief "Run a single fundus scenario test function against a host"
  buc_doc_param "test_function" "Rust test function name (e.g., full::bind_send)"
  buc_doc_shown || return 0

  export JJTEST_HOST="${z_host}"
  exec cargo test \
    --manifest-path "${ZJJF_MANIFEST_PATH}" \
    --test fundus_scenario \
    -- --test-threads=1 --ignored "${z_test}"
}

# eof
