#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle VM - Podman Virtual Machine Management

set -e

ZRBV_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBV_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBV_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Module Variables (ZRBV_*)
ZRBV_GIT_REGISTRY="ghcr.io"

######################################################################
# Internal Functions (zrbv_*)

# Generate brand file content
zrbv_generate_brand_content() {
  local temp_file="${RBV_TEMP_DIR}/brand_content.txt"

  echo "Write all RBV_CHOSEN_* variables to temp file" || bgc_die "TODO"

  (
    echo "# Recipe Bottle VM Brand File"
    echo ""
    env | grep "^RBV_CHOSEN_" | sort
  ) > "$temp_file"

  echo "$temp_file"
}

# Parse podman init output for natural choice
zrbv_parse_natural_choice() {
  local init_output="$1"

  # Look for "Looking up Podman Machine image at" line
  local natural_tag=$(echo "$init_output" | grep "Looking up Podman Machine image at" | \
    sed 's/.*Looking up Podman Machine image at \(.*\) to create VM/\1/')

  test -n "$natural_tag" || bcu_die "Failed to parse natural choice from init output"
  echo "$natural_tag"
}

# Extract version from tag (e.g., "5.5" from "quay.io/podman/machine-os-wsl:5.5")
zrbv_extract_version() {
    local tag="$1"
    echo "$tag" | cut -d: -f2
}

# Generate canonical stash name
zrbv_generate_stash_name() {
  local registry="$1"
  local repo="$2"
  local tag="$3"
  local sha_short="$4"

  local raw="stash-${registry}-${repo}-${tag}-${sha_short}"
  raw=${raw//[\/:]/-}  # Replace all '/' and ':' with '-'

  printf '%s\n' "$raw"
}

# Validate GitHub PAT environment
zrbv_validate_pat() {
  test -f "${RBRR_GITHUB_PAT_ENV}" || bcu_die "GitHub PAT env file not found at ${RBRR_GITHUB_PAT_ENV}"
  source  "${RBRR_GITHUB_PAT_ENV}"

  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"
}

# Stop and remove a VM if it exists
zrbv_remove_vm() {
  local vm_name="$1"

  if podman machine inspect "$vm_name" &>/dev/null; then
    bcu_info       "Stopping $vm_name..."
    podman machine stop     "$vm_name" || bcu_warn "Failed to stop $vm_name"
    bcu_info       "Removing $vm_name..."
    podman machine rm -f    "$vm_name" || bcu_die "Failed to remove $vm_name"
  else
    bcu_info             "VM $vm_name does not exist. Nothing to remove."
  fi
}

# Install crane in VM
zrbv_install_crane() {
  local vm_name="$1"

  bcu_info "Installing crane in $vm_name..."
  podman machine ssh "$vm_name" "curl -o crane.tar.gz -L $ZRBV_CRANE_URL"
  podman machine ssh "$vm_name" "sudo tar -xzf crane.tar.gz -C /usr/local/bin/ crane"
  podman machine ssh "$vm_name" "rm crane.tar.gz"
  podman machine ssh "$vm_name" "crane version"
}

# Login to registry in VM
zrbv_registry_login() {
  local vm_name="$1"

  source "${RBRR_GITHUB_PAT_ENV}"

  # Login with podman
  podman -c "$vm_name" login "${ZRBV_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"

  # Login with crane
  podman machine ssh "$vm_name" "crane auth login ${ZRBV_GIT_REGISTRY} -u ${RBRG_USERNAME} -p ${RBRG_PAT}"
}

######################################################################
# External Functions (rbv_*)

zrbv_validate_envvars() {
  # Handle documentation mode
  bcu_doc_env "RBV_TEMP_DIR               " "Empty temporary directory"
  bcu_doc_env "RBV_RBRR_FILE              " "File containing the RBRR constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBV_TEMP_DIR}"
  bvu_dir_empty   "${RBV_TEMP_DIR}"
  bvu_file_exists "${RBV_RBRR_FILE}"
  source          "${RBV_RBRR_FILE}"
  source "${ZRBV_SCRIPT_DIR}/rbrr.validator.sh"

  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
}

rbv_nuke() {
  # Handle documentation mode
  bcu_doc_brief "Completely reset the podman virtual machine environment"
  bcu_doc_lines "Destroys ALL containers, VMs, and VM cache"
  bcu_doc_lines "Requires explicit YES confirmation"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "WARNING: This will destroy all podman VMs and cache"
  read -p "Type YES to confirm: " confirm
  test "$confirm" = "YES" || bcu_die "Nuke not confirmed, exit without change"

  bcu_step "Stopping all containers..."
  podman stop -a  || bcu_warn "Attempt to stop all containers did not succeed."

  bcu_step "Removing all containers..."
  podman rm -a -f || bcu_die "Attempt to remove all containers failed."

  bcu_step "Removing all podman machines..."
  for vm in $(podman machine list -q); do
      zrbv_remove_vm "$vm" || bcu_die "Attempt to remove VM $vm failed."
  done

  bcu_step "Deleting VM cache directory..."
  rm -rf "$HOME/.local/share/containers/podman/machine"/*
  rm -rf "$HOME/.config/containers/podman/machine"/*

  bcu_success "Podman VM environment reset complete"
}

rbv_check() {
  # Handle documentation mode
  bcu_doc_brief "Compare RBRR_CHOSEN values against podman's natural choice"
  bcu_doc_lines "Creates temporary stash VM to discover latest"
  bcu_doc_lines "Compares against RBRR_CHOSEN environment variables"
  bcu_doc_lines "Shows what stash name would be for latest version"
  bcu_doc_lines "Does NOT affect operational VM"
  bcu_doc_shown || return 0

  # Perform command
  zrbv_validate_pat

  bcu_step "Removing any existing stash VM..."
  zrbv_remove_vm "$RBRR_STASH_MACHINE"

  bcu_step "Creating stash VM with natural podman init..."
  local init_output="${RBV_TEMP_DIR}/podman_init_output.txt"
  podman machine init "$RBRR_STASH_MACHINE" > "$init_output" 2>&1

  bcu_step "Parsing 'Looking up' line for actual tag..."
  local natural_tag=$(zrbv_parse_natural_choice "$(cat "$init_output")")
  local natural_version=$(zrbv_extract_version "$natural_tag")
  bcu_info "Natural choice: $natural_tag"

  bcu_step "Starting stash VM..."
  podman machine start "$RBRR_STASH_MACHINE"

  bcu_step "Installing crane in userspace..."
  zrbv_install_crane  "$RBRR_STASH_MACHINE"
  zrbv_registry_login "$RBRR_STASH_MACHINE"

  bcu_step "Using crane to get digest of natural choice..."
  local natural_digest=$(podman machine ssh "$RBRR_STASH_MACHINE" "crane digest $natural_tag")
  local natural_sha_short=$(echo "$natural_digest" | cut -c8-19)
  bcu_info "Natural digest: $natural_digest"

  bcu_step "Comparing natural choice with RBRR_CHOSEN_PODMAN_VERSION..."
  if [ "$natural_version" = "$RBRR_CHOSEN_PODMAN_VERSION" ]; then
    bcu_info "Version matches: $natural_version"
  else
    bcu_warn "Version mismatch: natural=$natural_version, chosen=$RBRR_CHOSEN_PODMAN_VERSION"
  fi

  bcu_step "Comparing digest with RBRR_CHOSEN_VMIMAGE_SHA..."
  if [ -n "$RBRR_CHOSEN_VMIMAGE_SHA" ]; then
    if [ "$natural_digest" = "$RBRR_CHOSEN_VMIMAGE_SHA" ]; then
      bcu_info "SHA matches"
    else
      bcu_warn "SHA mismatch: natural=$natural_digest, chosen=$RBRR_CHOSEN_VMIMAGE_SHA"
    fi
  else
    bcu_info "No RBRR_CHOSEN_VMIMAGE_SHA to compare"
  fi

  bcu_step "Generating canonical stash name for latest..."
  local stash_name=$(zrbv_generate_stash_name "quay.io" "podman/machine-os-wsl" "$natural_version" "$natural_sha_short")
  local stash_fqin="${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:${stash_name}"
  bcu_info "Canonical stash name: $stash_fqin"

  bcu_step "Checking if this stash exists in GHCR..."
  if podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $stash_fqin" >/dev/null 2>&1; then
    bcu_info "Status: CURRENT (stash exists)"
  else
    if [ "$natural_version" != "$RBRR_CHOSEN_PODMAN_VERSION" ]; then
      bcu_info "Status: UPDATE_AVAILABLE (newer version: $natural_version)"
    else
      bcu_info "Status: NOT_STASHED (need to run rbv_stash)"
    fi
  fi

  bcu_step "Stopping stash VM..."
  podman machine stop "$RBRR_STASH_MACHINE"

  bcu_success "Check complete"
}

rbv_stash() {
  # Handle documentation mode
  bcu_doc_brief "Validate RBRR_CHOSEN values and create GHCR stash"
  bcu_doc_lines "Ensures RBRR_CHOSEN values match podman's natural choice"
  bcu_doc_lines "Copies exact version to GHCR with canonical name"
  bcu_doc_lines "Destructive: removes all VMs before starting"
  bcu_doc_shown || return 0

  # Perform command
  zrbv_validate_pat

  bcu_step "Removing operational VM..."
  zrbv_remove_vm "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Removing stash VM..."
  zrbv_remove_vm "$RBRR_STASH_MACHINE"

  bcu_step "Creating stash VM with natural podman init..."
  local init_output="${RBV_TEMP_DIR}/podman_init_output.txt"
  podman machine init "$RBRR_STASH_MACHINE" > "$init_output" 2>&1

  bcu_step "Parsing init output for tag..."
  local natural_tag=$(zrbv_parse_natural_choice "$(cat "$init_output")")
  local natural_version=$(zrbv_extract_version "$natural_tag")
  bcu_info "Natural tag: $natural_tag"

  bcu_step "Validating matches RBRR_CHOSEN_PODMAN_VERSION..."
  local expected_tag="${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}"
  test "$natural_tag" = "$expected_tag" || \
    bcu_die "Natural choice ($natural_tag) doesn't match expected ($expected_tag)"

  bcu_step "Starting stash VM..."
  podman machine start "$RBRR_STASH_MACHINE"

  bcu_step "Installing crane in userspace..."
  zrbv_install_crane "$RBRR_STASH_MACHINE"
  zrbv_registry_login "$RBRR_STASH_MACHINE"

  bcu_step "Getting digest with crane..."
  local digest=$(podman machine ssh "$RBRR_STASH_MACHINE" "crane digest $natural_tag")
  local sha_short=$(echo "$digest" | cut -c8-19)
  bcu_info "Digest: $digest"

  bcu_step "Validating matches RBRR_CHOSEN_VMIMAGE_SHA..."
  if [ -n "$RBRR_CHOSEN_VMIMAGE_SHA" ]; then
    test "$digest" = "$RBRR_CHOSEN_VMIMAGE_SHA" || \
      bcu_die "Digest ($digest) doesn't match RBRR_CHOSEN_VMIMAGE_SHA ($RBRR_CHOSEN_VMIMAGE_SHA)"
  fi

  bcu_step "Generating canonical stash name..."
  local stash_name=$(zrbv_generate_stash_name "quay.io" "podman/machine-os-wsl" "$natural_version" "$sha_short")
  local stash_fqin="${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:${stash_name}"
  bcu_info "Stash FQIN: $stash_fqin"

  bcu_step "Checking if already exists in GHCR..."
  if podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $stash_fqin" >/dev/null 2>&1; then
    bcu_info "Stash already exists, skipping copy"
  else
    bcu_step "Copying with crane from quay to GHCR..."
    podman machine ssh "$RBRR_STASH_MACHINE" "crane copy $natural_tag $stash_fqin"

    bcu_step "Verifying copy with crane manifest..."
    podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $stash_fqin" >/dev/null || \
      bcu_die "Failed to verify stash in GHCR"
  fi

  bcu_step "Stopping stash VM..."
  podman machine stop "$RBRR_STASH_MACHINE"

  bcu_success "Stash complete: $stash_fqin"
}

rbv_init() {
  # Handle documentation mode
  bcu_doc_brief "Initialize operational VM using RBRR_CHOSEN_VMIMAGE_FQIN"
  bcu_doc_lines "Creates new operational VM from configured image"
  bcu_doc_lines "Writes brand file with all RBRR_CHOSEN values"
  bcu_doc_lines "Refuses if operational VM already exists"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Checking if operational VM exists..."
  podman machine list | grep -q "$RBRR_OPERATIONAL_MACHINE" && \
    bcu_die "Operational VM already exists. Remove it first with rbv_nuke or manually"

  bcu_step "Validating RBRR_CHOSEN_VMIMAGE_FQIN format..."
  echo "$RBRR_CHOSEN_VMIMAGE_FQIN" | grep -q ":" || \
    bcu_die "Invalid FQIN format: $RBRR_CHOSEN_VMIMAGE_FQIN"

  bcu_step "Initializing: podman machine init --image docker://${RBRR_CHOSEN_VMIMAGE_FQIN}..."
  podman machine init --rootful --image "docker://${RBRR_CHOSEN_VMIMAGE_FQIN}" "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Starting VM temporarily..."
  podman machine start "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Writing brand file to /etc/recipe-bottle-brand-file.txt..."
  local brand_content=$(zrbv_generate_brand_content)
  podman machine ssh "$RBRR_OPERATIONAL_MACHINE" "sudo tee /etc/recipe-bottle-brand-file.txt" < "$brand_content"

  bcu_step "Stopping VM..."
  podman machine stop "$RBRR_OPERATIONAL_MACHINE"

  bcu_success "VM initialized with brand file"
}

rbv_start() {
  # Handle documentation mode
  bcu_doc_brief "Start operational podman machine"
  bcu_doc_lines "Verifies brand file matches RBRR_CHOSEN values"
  bcu_doc_lines "Fails if brand file missing or mismatched"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Starting operational VM..."
  podman machine start "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Reading brand file from /etc/recipe-bottle-brand-file.txt..."
  local brand_file="${RBV_TEMP_DIR}/current_brand.txt"
  podman machine ssh "$RBRR_OPERATIONAL_MACHINE" "sudo cat /etc/recipe-bottle-brand-file.txt" > "$brand_file" || \
    bcu_die "Failed to read brand file. VM may not have been initialized with rbv_init"

  bcu_step "Comparing brand file with current RBRR_CHOSEN values..."
  local expected_brand=$(zrbv_generate_brand_content)

  # Compare only the RBV_CHOSEN lines
  bcu_die "TODO transform to better pattern and no sort please"
  grep "^RBV_CHOSEN_" "$brand_file"     | sort > "${RBV_TEMP_DIR}/brand_actual.txt"
  grep "^RBV_CHOSEN_" "$expected_brand" | sort > "${RBV_TEMP_DIR}/brand_expected.txt"

  if ! cmp -s "${RBV_TEMP_DIR}/brand_actual.txt" "${RBV_TEMP_DIR}/brand_expected.txt"; then
    bcu_warn "Brand file mismatch detected!"
    bcu_info "Expected:"
    cat "${RBV_TEMP_DIR}/brand_expected.txt"
    bcu_info "Actual:"
    cat "${RBV_TEMP_DIR}/brand_actual.txt"
    bcu_die "Brand file doesn't match current RBRR_CHOSEN values"
  fi

  bcu_success "VM started and brand verified"
}

rbv_stop() {
  # Handle documentation mode
  bcu_doc_brief "Stop operational podman machine"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Stopping operational VM..."
  podman machine stop "$RBRR_OPERATIONAL_MACHINE"

  bcu_success "VM stopped"
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_validate_envvars "$@"

# eof

