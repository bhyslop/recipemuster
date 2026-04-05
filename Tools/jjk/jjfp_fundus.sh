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
# JJFP Fundus - Fundus test account provisioning and scenario dispatch
#
# Two-phase provisioning:
#   Phase 1 (root):     sudo tt/jjw-tfP1.ProvisionPhase1.sh <keypath>
#   Phase 2 (operator): tt/jjw-tfP2.ProvisionPhase2.<host>.sh

set -euo pipefail

# Multiple inclusion detection
test -z "${ZJJFP_SOURCED:-}" || buc_die "Module jjfp multiply sourced - check sourcing hierarchy"
ZJJFP_SOURCED=1

# Tinder constant: SSH key path param (operator passes private key path; .pub derived)
JJFP_keypath_param="keypath"

######################################################################
# Internal Functions (zjjfp_*)

zjjfp_kindle() {
  test -z "${ZJJFP_KINDLED:-}" || buc_die "Module jjfp already kindled"

  test -n "${BURD_TEMP_DIR:-}"  || buc_die "BURD_TEMP_DIR is unset"
  test -n "${BURD_TOOLS_DIR:-}" || buc_die "BURD_TOOLS_DIR is unset"

  readonly ZJJFP_SCRIPT_DIR="${BURD_TOOLS_DIR}/jjk"
  readonly ZJJFP_MANIFEST_PATH="${ZJJFP_SCRIPT_DIR}/vov_veiled/Cargo.toml"
  readonly ZJJFP_RELDIR="projects/rbm_alpha_recipemuster"
  readonly ZJJFP_ACCOUNTS="jjfu_full jjfu_nokey jjfu_norepo jjfu_nogit"
  readonly ZJJFP_TEMP_PREFIX="${BURD_TEMP_DIR}/jjfp_"

  # Shared temp file paths for helper results (sequential access only)
  readonly ZJJFP_RESOLVE_HOME="${ZJJFP_TEMP_PREFIX}home_dir.txt"
  readonly ZJJFP_RESOLVE_GROUP="${ZJJFP_TEMP_PREFIX}primary_group.txt"
  readonly ZJJFP_RESOLVE_PLATFORM="${ZJJFP_TEMP_PREFIX}platform.txt"
  readonly ZJJFP_RESOLVE_UID="${ZJJFP_TEMP_PREFIX}uid.txt"

  readonly ZJJFP_KINDLED=1
}

zjjfp_sentinel() {
  test "${ZJJFP_KINDLED:-}" = "1" || buc_die "Module jjfp not kindled - call zjjfp_kindle first"
}

######################################################################
# Internal helpers — resolvers (write to temp file, caller reads)

zjjfp_resolve_platform() {
  zjjfp_sentinel

  local -r z_uname_file="${ZJJFP_TEMP_PREFIX}uname.txt"
  uname -s > "${z_uname_file}" || buc_die "Failed to detect platform"
  local -r z_uname=$(<"${z_uname_file}")
  test -n "${z_uname}" || buc_die "Empty uname output"

  case "${z_uname}" in
    Darwin) echo "macos" > "${ZJJFP_RESOLVE_PLATFORM}" ;;
    Linux)  echo "linux" > "${ZJJFP_RESOLVE_PLATFORM}" ;;
    *)      buc_die "Unsupported platform: ${z_uname}" ;;
  esac
}

zjjfp_resolve_home_dir() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  case "${z_platform}" in
    macos) echo "/Users/${z_user}" > "${ZJJFP_RESOLVE_HOME}" ;;
    linux) echo "/home/${z_user}" > "${ZJJFP_RESOLVE_HOME}" ;;
  esac
}

zjjfp_resolve_primary_group() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  case "${z_platform}" in
    macos) echo "staff" > "${ZJJFP_RESOLVE_GROUP}" ;;
    linux) echo "${z_user}" > "${ZJJFP_RESOLVE_GROUP}" ;;
  esac
}

######################################################################
# Internal helpers — Phase 1: account management (running as root)

zjjfp_delete_account() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  test -n "${z_user}" || buc_die "zjjfp_delete_account: user required"

  local -r z_id_stderr="${ZJJFP_TEMP_PREFIX}id_${z_user}_stderr.txt"
  if ! id "${z_user}" >/dev/null 2>"${z_id_stderr}"; then
    buc_log_args "Account ${z_user} does not exist — skip delete"
    return 0
  fi

  buc_step "Deleting account: ${z_user}"
  local -r z_stderr="${ZJJFP_TEMP_PREFIX}delete_${z_user}_stderr.txt"
  case "${z_platform}" in
    macos)
      sysadminctl -deleteUser "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to delete macOS account: ${z_user} — see ${z_stderr}"
      if test -d "/Users/${z_user}"; then
        rm -rf "/Users/${z_user}" \
          || buc_die "Failed to remove residual home dir: /Users/${z_user}"
      fi
      ;;
    linux)
      userdel -r "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to delete Linux account: ${z_user} — see ${z_stderr}"
      if test -d "/home/${z_user}"; then
        rm -rf "/home/${z_user}" \
          || buc_die "Failed to remove residual home dir: /home/${z_user}"
      fi
      ;;
  esac
  buc_log_args "Account ${z_user} deleted"
}

zjjfp_create_account() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  test -n "${z_user}" || buc_die "zjjfp_create_account: user required"

  buc_step "Creating account: ${z_user}"
  local -r z_stderr="${ZJJFP_TEMP_PREFIX}create_${z_user}_stderr.txt"
  case "${z_platform}" in
    macos)
      local -r z_pw_file="${ZJJFP_TEMP_PREFIX}password.txt"
      openssl rand -base64 16 > "${z_pw_file}" \
        || buc_die "Failed to generate random password"
      local -r z_password=$(<"${z_pw_file}")
      test -n "${z_password}" || buc_die "Generated password is empty"

      sysadminctl -addUser "${z_user}" \
        -password "${z_password}" \
        -home "/Users/${z_user}" \
        -shell /bin/bash \
        2>"${z_stderr}" \
        || buc_die "Failed to create macOS account: ${z_user} — see ${z_stderr}"

      # sysadminctl may not create the home dir — ensure it exists with correct ownership
      mkdir -p "/Users/${z_user}" \
        || buc_die "Failed to create home dir for: ${z_user}"
      chown -R "${z_user}:staff" "/Users/${z_user}" \
        || buc_die "Failed to chown home dir for: ${z_user}"
      ;;
    linux)
      useradd -m -s /bin/bash "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to create Linux account: ${z_user} — see ${z_stderr}"
      passwd -l "${z_user}" 2>"${z_stderr}" \
        || buc_die "Failed to lock password for: ${z_user} — see ${z_stderr}"
      ;;
  esac
  buc_log_args "Account ${z_user} created"
}

######################################################################
# Internal helpers — Phase 1: SSH login access (running as root)

zjjfp_enable_ssh_access() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  test -n "${z_user}" || buc_die "zjjfp_enable_ssh_access: user required"

  case "${z_platform}" in
    macos)
      buc_step "Enabling SSH access for: ${z_user}"
      local -r z_stderr="${ZJJFP_TEMP_PREFIX}sshaccess_${z_user}_stderr.txt"
      dseditgroup -o edit -a "${z_user}" -t user com.apple.access_ssh 2>"${z_stderr}" \
        || buc_die "Failed to add ${z_user} to com.apple.access_ssh — see ${z_stderr}"
      buc_log_args "SSH access enabled for ${z_user}"
      ;;
    linux)
      # Linux sshd allows all local users by default
      ;;
  esac
}

######################################################################
# Internal helpers — Phase 1: SSH keypair installation (running as root)

zjjfp_install_keypair() {
  zjjfp_sentinel
  local -r z_platform="${1:-}"
  local -r z_user="${2:-}"
  local -r z_privkey_path="${3:-}"
  test -n "${z_user}"         || buc_die "zjjfp_install_keypair: user required"
  test -n "${z_privkey_path}" || buc_die "zjjfp_install_keypair: private key path required"

  buc_step "Installing SSH keypair for: ${z_user}"

  local -r z_pubkey_path="${z_privkey_path}.pub"
  local -r z_key_name="${z_privkey_path##*/}"

  zjjfp_resolve_home_dir "${z_platform}" "${z_user}"
  local -r z_home=$(<"${ZJJFP_RESOLVE_HOME}")
  local -r z_ssh_dir="${z_home}/.ssh"

  zjjfp_resolve_primary_group "${z_platform}" "${z_user}"
  local -r z_group=$(<"${ZJJFP_RESOLVE_GROUP}")

  mkdir -p "${z_ssh_dir}" \
    || buc_die "Failed to create .ssh dir for: ${z_user}"

  # Install private key (for GitHub access from the account)
  cp "${z_privkey_path}" "${z_ssh_dir}/${z_key_name}" \
    || buc_die "Failed to copy private key for: ${z_user}"
  chmod 600 "${z_ssh_dir}/${z_key_name}" \
    || buc_die "Failed to chmod private key for: ${z_user}"

  # Install public key (for GitHub access from the account)
  cp "${z_pubkey_path}" "${z_ssh_dir}/${z_key_name}.pub" \
    || buc_die "Failed to copy public key for: ${z_user}"
  chmod 644 "${z_ssh_dir}/${z_key_name}.pub" \
    || buc_die "Failed to chmod public key for: ${z_user}"

  # Authorize operator SSH access to this account
  cp "${z_pubkey_path}" "${z_ssh_dir}/authorized_keys" \
    || buc_die "Failed to write authorized_keys for: ${z_user}"
  chmod 600 "${z_ssh_dir}/authorized_keys" \
    || buc_die "Failed to chmod authorized_keys for: ${z_user}"

  chmod 700 "${z_ssh_dir}" \
    || buc_die "Failed to chmod .ssh for: ${z_user}"
  chown -R "${z_user}:${z_group}" "${z_ssh_dir}" \
    || buc_die "Failed to chown .ssh for: ${z_user}"

  buc_log_args "SSH keypair installed for ${z_user}"
}

######################################################################
# Internal helpers — Phase 2: repo setup (running as operator, via SSH)

zjjfp_ssh_setup_repo() {
  zjjfp_sentinel
  local -r z_host="${1:-}"
  local -r z_user="${2:-}"
  local -r z_with_origin="${3:-1}"
  local -r z_curia_origin="${4:-}"
  local -r z_curia_repo="${5:-}"
  test -n "${z_host}" || buc_die "zjjfp_ssh_setup_repo: host required"
  test -n "${z_user}" || buc_die "zjjfp_ssh_setup_repo: user required"
  test -n "${z_curia_repo}" || buc_die "zjjfp_ssh_setup_repo: curia_repo required"

  buc_step "Setting up repo for: ${z_user}@${z_host} (origin=${z_with_origin})"

  local -r z_stderr="${ZJJFP_TEMP_PREFIX}repo_${z_user}_stderr.txt"
  local -r z_ssh_target="${z_user}@${z_host}"
  local -r z_project_dir="${ZJJFP_RELDIR}"

  # Clean slate: remove existing project dir, then clone fresh
  ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_ssh_target}" \
    "rm -rf '${z_project_dir}' && git config --global --add safe.directory '${z_curia_repo}/.git' && mkdir -p '${z_project_dir%/*}' && git clone '${z_curia_repo}' '${z_project_dir}'" \
    2>"${z_stderr}" \
    || buc_die "Failed to clone repo for: ${z_ssh_target} — see ${z_stderr}"

  # Configure origin
  if test "${z_with_origin}" = "1"; then
    test -n "${z_curia_origin}" || buc_die "zjjfp_ssh_setup_repo: curia_origin required when with_origin=1"
    ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_ssh_target}" \
      "git -C '${z_project_dir}' remote set-url origin '${z_curia_origin}'" \
      2>"${z_stderr}" \
      || buc_die "Failed to set origin for: ${z_ssh_target} — see ${z_stderr}"
    buc_log_args "Repo cloned with origin: ${z_curia_origin}"
  else
    ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_ssh_target}" \
      "git -C '${z_project_dir}' remote remove origin" \
      2>"${z_stderr}" \
      || buc_die "Failed to remove origin for: ${z_ssh_target} — see ${z_stderr}"
    buc_log_args "Repo cloned with origin removed"
  fi

  zjjfp_ssh_install_buk "${z_host}" "${z_user}" "${z_curia_repo}"
}

zjjfp_ssh_install_buk() {
  zjjfp_sentinel
  local -r z_host="${1:-}"
  local -r z_user="${2:-}"
  local -r z_curia_repo="${3:-}"

  buc_step "Installing BUK for: ${z_user}@${z_host}"

  local -r z_ssh_target="${z_user}@${z_host}"
  local -r z_buk_dir="${ZJJFP_RELDIR}/.buk"
  local -r z_burc_stderr="${ZJJFP_TEMP_PREFIX}burc_${z_user}_stderr.txt"

  # Create .buk dir and write burc.env
  ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_ssh_target}" \
    "mkdir -p '${z_buk_dir}' && cat > '${z_buk_dir}/burc.env'" <<'BURC' || buc_die "Failed to write burc.env for: ${z_ssh_target} — see ${z_burc_stderr}"
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

  # Copy launcher files via SSH
  local -r z_curia_buk="${z_curia_repo}/.buk"
  local z_launcher=""
  local z_index=0
  for z_launcher in "${z_curia_buk}"/launcher.*.sh; do
    test -f "${z_launcher}" || continue
    local z_launcher_name="${z_launcher##*/}"
    local z_scp_stderr="${ZJJFP_TEMP_PREFIX}buk_${z_user}_${z_index}_scp_stderr.txt"
    scp -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_launcher}" "${z_ssh_target}:${z_buk_dir}/${z_launcher_name}" \
      2>"${z_scp_stderr}" \
      || buc_die "Failed to copy launcher ${z_launcher_name} for: ${z_ssh_target} — see ${z_scp_stderr}"
    z_index=$((z_index + 1))
  done

  ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "${z_ssh_target}" "chmod -R u+x '${z_buk_dir}'" \
    || buc_die "Failed to chmod .buk for: ${z_ssh_target}"

  buc_log_args "BUK installed for ${z_user}@${z_host}"
}

######################################################################
# External Functions (jjfp_*)

jjfp_provision() {
  zjjfp_sentinel

  local -r z_keypath="${1:-}"

  buc_doc_brief "Phase 1: Create OS accounts and install SSH keypairs (requires root)"
  buc_doc_param "${JJFP_keypath_param}" "Path to curia user SSH private key (e.g., /Users/you/.ssh/id_ed25519)"
  buc_doc_shown || return 0

  # Root gate
  id -u > "${ZJJFP_RESOLVE_UID}" || buc_die "Failed to get uid"
  local -r z_uid=$(<"${ZJJFP_RESOLVE_UID}")
  test "${z_uid}" = "0" || buc_die "jjfp_provision: must run as root (use: sudo tt/jjw-tfP1.ProvisionPhase1.sh <keypath>)"

  # Validate keypair
  test -n "${z_keypath}" || buc_die "jjfp_provision: private key path required (pass as argument)"
  test -f "${z_keypath}" || buc_die "jjfp_provision: private key not found: ${z_keypath}"
  test -f "${z_keypath}.pub" || buc_die "jjfp_provision: public key not found: ${z_keypath}.pub"

  buc_step "Phase 1: Provisioning fundus test accounts"

  # Discover platform
  zjjfp_resolve_platform
  local -r z_platform=$(<"${ZJJFP_RESOLVE_PLATFORM}")
  test -n "${z_platform}" || buc_die "Empty platform result"

  # Delete all existing accounts (clean slate)
  buc_step "Deleting existing accounts"
  local z_acct=""
  for z_acct in ${ZJJFP_ACCOUNTS}; do
    zjjfp_delete_account "${z_platform}" "${z_acct}"
  done

  # Create accounts with SSH keypairs per JJSTF
  buc_step "Creating accounts"

  # jjfu_full: SSH keypair (for operator access + GitHub)
  zjjfp_create_account      "${z_platform}" "jjfu_full"
  zjjfp_enable_ssh_access   "${z_platform}" "jjfu_full"
  zjjfp_install_keypair     "${z_platform}" "jjfu_full" "${z_keypath}"

  # jjfu_nokey: OS account only, no SSH key access from curia
  zjjfp_create_account      "${z_platform}" "jjfu_nokey"

  # jjfu_norepo: SSH keypair, no project directory at RELDIR
  zjjfp_create_account      "${z_platform}" "jjfu_norepo"
  zjjfp_enable_ssh_access   "${z_platform}" "jjfu_norepo"
  zjjfp_install_keypair     "${z_platform}" "jjfu_norepo" "${z_keypath}"

  # jjfu_nogit: SSH keypair (repo setup deferred to phase 2)
  zjjfp_create_account      "${z_platform}" "jjfu_nogit"
  zjjfp_enable_ssh_access   "${z_platform}" "jjfu_nogit"
  zjjfp_install_keypair     "${z_platform}" "jjfu_nogit" "${z_keypath}"

  # Restore ownership of dispatch dirs so operator's next dispatch isn't poisoned
  test -n "${SUDO_USER:-}" || buc_die "SUDO_USER not set — cannot restore ownership"
  zjjfp_resolve_primary_group "${z_platform}" "${SUDO_USER}"
  local -r z_sudo_group=$(<"${ZJJFP_RESOLVE_GROUP}")
  chown -R "${SUDO_USER}:${z_sudo_group}" "${BURD_TEMP_DIR}" \
    || buc_die "Failed to restore ownership of temp dir"
  chown -R "${SUDO_USER}:${z_sudo_group}" "${BURD_OUTPUT_DIR}" \
    || buc_die "Failed to restore ownership of output dir"
  chown -R "${SUDO_USER}:${z_sudo_group}" "${BURD_OUTPUT_DIR%/*}" \
    || buc_die "Failed to restore ownership of output root dir"

  buc_success "Phase 1 complete — accounts and keypairs provisioned. Run phase 2 to set up repos."
}

jjfp_repo() {
  zjjfp_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  test -n "${z_host}" || buc_die "jjfp_repo: no host (BUZ_FOLIO empty)"

  buc_doc_brief "Phase 2: Clone repos and install BUK via SSH to provisioned accounts"
  buc_doc_shown || return 0

  buc_step "Phase 2: Setting up repos on ${z_host}"

  # Resolve curia repo absolute path
  local -r z_curia_root_file="${ZJJFP_TEMP_PREFIX}curia_root.txt"
  (cd "${BURD_TOOLS_DIR}/.." && pwd) > "${z_curia_root_file}" \
    || buc_die "Failed to resolve curia repo root"
  local -r z_curia_repo=$(<"${z_curia_root_file}")
  test -n "${z_curia_repo}" || buc_die "Empty curia repo root"

  # Discover git origin URL
  local -r z_origin_file="${ZJJFP_TEMP_PREFIX}origin.txt"
  local -r z_origin_stderr="${ZJJFP_TEMP_PREFIX}origin_stderr.txt"
  git -C "${BURD_TOOLS_DIR}/.." remote get-url origin > "${z_origin_file}" 2>"${z_origin_stderr}" \
    || buc_die "Failed to get git origin URL from curia repo — see ${z_origin_stderr}"
  local -r z_curia_origin=$(<"${z_origin_file}")
  test -n "${z_curia_origin}" || buc_die "Git origin URL is empty"
  buc_log_args "Curia origin: ${z_curia_origin}"

  # jjfu_full: clone with origin (for jjx_plant — needs git fetch origin)
  zjjfp_ssh_setup_repo "${z_host}" "jjfu_full" 1 "${z_curia_origin}" "${z_curia_repo}"

  # jjfu_full needs GitHub host key for git fetch origin
  local -r z_ghkey_stderr="${ZJJFP_TEMP_PREFIX}ghkey_jjfu_full_stderr.txt"
  ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new "jjfu_full@${z_host}" \
    "ssh-keyscan github.com >> ~/.ssh/known_hosts 2>/dev/null" \
    2>"${z_ghkey_stderr}" \
    || buc_die "Failed to add GitHub host key for jjfu_full — see ${z_ghkey_stderr}"
  buc_log_args "GitHub host key added for jjfu_full"

  # jjfu_nogit: clone with origin removed (tests plant failure path)
  zjjfp_ssh_setup_repo "${z_host}" "jjfu_nogit" 0 "" "${z_curia_repo}"

  buc_success "Phase 2 complete — repos and BUK installed"
}

jjfp_scenario() {
  zjjfp_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  test -n "${z_host}" || buc_die "jjfp_scenario: no host (BUZ_FOLIO empty)"

  buc_doc_brief "Run all fundus scenario profiles against a host"
  buc_doc_shown || return 0

  export JJTEST_HOST="${z_host}"
  exec cargo test \
    --manifest-path "${ZJJFP_MANIFEST_PATH}" \
    --test fundus_scenario \
    -- --test-threads=1 --ignored
}

jjfp_single() {
  zjjfp_sentinel

  local -r z_host="${BUZ_FOLIO:-}"
  local -r z_test="${1:-}"
  test -n "${z_host}" || buc_die "jjfp_single: no host (BUZ_FOLIO empty)"
  test -n "${z_test}" || buc_die "jjfp_single: no test function name (pass as argument)"

  buc_doc_brief "Run a single fundus scenario test function against a host"
  buc_doc_param "test_function" "Rust test function name (e.g., full::bind_send)"
  buc_doc_shown || return 0

  export JJTEST_HOST="${z_host}"
  exec cargo test \
    --manifest-path "${ZJJFP_MANIFEST_PATH}" \
    --test fundus_scenario \
    -- --test-threads=1 --ignored "${z_test}"
}

# eof
