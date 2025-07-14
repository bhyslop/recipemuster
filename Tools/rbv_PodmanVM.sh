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

ZRBV_GENERATED_BRAND_FILE="${RBV_TEMP_DIR}/brand_generated.txt"
ZRBV_FOUND_BRAND_FILE="${RBV_TEMP_DIR}/brand_found.txt"
ZRBV_INIT_OUTPUT_FILE="${RBV_TEMP_DIR}/podman_init_output.txt"

ZRBV_STASH_INIT_STDOUT="${RBV_TEMP_DIR}/stash_init_stdout.txt"
ZRBV_STASH_INIT_STDERR="${RBV_TEMP_DIR}/stash_init_stderr.txt"
ZRBV_OPERATIONAL_INIT_STDOUT="${RBV_TEMP_DIR}/operational_init_stdout.txt"
ZRBV_OPERATIONAL_INIT_STDERR="${RBV_TEMP_DIR}/operational_init_stderr.txt"

ZRBV_NATURAL_TAG_FILE="${RBV_TEMP_DIR}/natural_tag.txt"
ZRBV_MIRROR_TAG_FILE="${RBV_TEMP_DIR}/mirror_tag.txt"
ZRBV_CRANE_DIGEST_FILE="${RBV_TEMP_DIR}/crane_digest.txt"
ZRBV_CRANE_MANIFEST_CHECK_FILE="${RBV_TEMP_DIR}/crane_manifest_check.txt"
ZRBV_CRANE_COPY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_copy_output.txt"
ZRBV_CRANE_VERIFY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_verify_output.txt"

ZRBV_EMPLACED_BRAND_FILE=/etc/brand-emplaced.txt


######################################################################
# Internal Functions (zrbv_*)

# Generate brand file content
zrbv_generate_brand_file() {
  echo "# Recipe Bottle VM Brand File"                   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "#"                                               >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "PODMAN_VERSION: ${RBRR_CHOSEN_PODMAN_VERSION}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_ORIGIN: ${RBRR_CHOSEN_VMIMAGE_ORIGIN}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_FQIN:   ${RBRR_CHOSEN_VMIMAGE_FQIN}"     >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_SHA:    ${RBRR_CHOSEN_VMIMAGE_SHA}"      >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "IDENTITY:       ${RBRR_CHOSEN_IDENTITY}"         >> "${ZRBV_GENERATED_BRAND_FILE}"
}

# Confirm YES parameter or prompt user
zrbv_confirm_yes() {
  local confirm_param="$1"
  
  if [[ "$confirm_param" == "YES" ]]; then
    return 0
  fi
  
  read -p "Type YES to confirm: " confirm
  if [[ "$confirm" == "YES" ]]; then
    return 0
  else
    return 1
  fi
}

# Extract natural tag from podman init output
zrbv_extract_natural_tag() {
  local init_output_file="$1"

  grep "Looking up Podman Machine image at" "$init_output_file" | \
    sed 's/.*Looking up Podman Machine image at \(.*\) to create VM/\1/' \
    > "${ZRBV_NATURAL_TAG_FILE}"

  test -s "${ZRBV_NATURAL_TAG_FILE}" || bcu_die "Failed to extract natural tag from init output"
}

# Generate mirror tag using yq and RBRR values
zrbv_generate_mirror_tag() {
  # Get digest and extract short SHA
  local sha_short=$(yq eval '.digest' "${ZRBV_CRANE_DIGEST_FILE}" | cut -c8-19)

  # Build canonical mirror tag
  local raw="stash-quay.io-podman-machine-os-wsl-${RBRR_CHOSEN_PODMAN_VERSION}-${sha_short}"

  # Sanitize for use as tag
  raw=${raw//\//-}
  raw=${raw//:/-}

  # Write full FQIN to file
  echo "${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:${raw}" > "${ZRBV_MIRROR_TAG_FILE}"
}

# Compare two files and return error if different
zrbv_error_if_different() {
  local file1="$1"
  local file2="$2"

  if [[ "$(cat "$file1")" == "$(cat "$file2")" ]]; then
    return 0
  else
    bcu_warn "File content mismatch detected!"
    bcu_info "File 1 ($file1) contents:"
    cat              "$file1"
    bcu_info "File 2 ($file2) contents:"
    cat              "$file2"
    return 1
  fi
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

# Reset stash VM - stop, remove, and reinit with captured output
zrbv_reset_stash() {
  zrbv_remove_vm "$RBRR_STASH_MACHINE"

  bcu_step "Creating stash VM with natural podman init..."
  podman machine init --log-level=debug "$RBRR_STASH_MACHINE"      \
                                     >  "$ZRBV_STASH_INIT_STDOUT"  \
                                     2> "$ZRBV_STASH_INIT_STDERR"

  bcu_step "Starting stash VM..."
  podman machine start "$RBRR_STASH_MACHINE" || bcu_die "Failed to start stash VM"

  return 0
}

# Install crane in VM
zrbv_install_crane() {
  local vm_name="$1"

  bcu_info "Installing crane in $vm_name..."
  podman machine ssh "$vm_name" "curl -o crane.tar.gz -L ${RBRR_CRANE_TAR_GZ}"
  podman machine ssh "$vm_name" "sudo tar -xzf crane.tar.gz -C /usr/local/bin/ crane"
  podman machine ssh "$vm_name" "rm crane.tar.gz"
  podman machine ssh "$vm_name" "crane version"
}

# Login to registry in VM
zrbv_registry_login() {
  local vm_name="$1"

  source "${RBRR_GITHUB_PAT_ENV}"

  bcu_step "Login with podman..."
  podman -c "$vm_name" login "${ZRBV_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"

  bcu_step "Login with crane..."
  podman machine ssh "$vm_name" "crane auth login ${ZRBV_GIT_REGISTRY} -u ${RBRG_USERNAME} -p ${RBRG_PAT}"
}

######################################################################
# External Functions (rbv_*)

zrbv_validate_envvars() {
  # Handle documentation mode
  bcu_doc_env "RBV_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBV_RBRR_FILE " "File containing the RBRR constants"
  bcu_doc_env "RBV_RBRS_FILE " "File containing the RBRS constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBV_TEMP_DIR}"
  bvu_dir_empty   "${RBV_TEMP_DIR}"
  bvu_file_exists "${RBV_RBRR_FILE}"
  bvu_file_exists "${RBV_RBRS_FILE}"

  source              "${RBV_RBRR_FILE}"
  source "${ZRBV_SCRIPT_DIR}/rbrr.validator.sh"

  source              "${RBV_RBRS_FILE}"
  source "${ZRBV_SCRIPT_DIR}/rbrs.validator.sh"

  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
}

rbv_nuke() {
  # Handle documentation mode
  bcu_doc_brief "Completely reset the podman virtual machine environment"
  bcu_doc_lines "Destroys ALL containers, VMs, and VM cache"
  bcu_doc_lines "Requires explicit YES confirmation"
  bcu_doc_shown || return 0

  # Perform command
  bvu_dir_exists "${RBRS_PODMAN_ROOT_DIR}" || bcu_warn "Podman directory not found."

  bcu_step "WARNING: This will destroy all podman VMs and cache found in ${RBRS_PODMAN_ROOT_DIR}"
  zrbv_confirm_yes "$1" || bcu_die "Nuke not confirmed, exit without change"

  bcu_step "Stopping all containers..."
  podman stop -a  || bcu_warn "Attempt to stop all containers did not succeed; okay if machine not started."

  bcu_step "Removing all containers..."
  podman rm -a -f || bcu_warn "Attempt to remove all containers failed; okay if machine not started."

  bcu_step "Removing all podman machines..."
  for vm in $(podman machine list -q); do
      zrbv_remove_vm "$vm" || bcu_die "Attempt to remove VM $vm failed."
  done

  bcu_step "Deleting VM cache directory..."
  rm -rf "${RBRS_PODMAN_ROOT_DIR}/machine"/*

  bcu_success "Podman VM environment reset complete"
}

rbv_check() {
  # Handle documentation mode
  bcu_doc_brief "Compare RBRR_CHOSEN values against podman's natural choice"
  bcu_doc_lines "Creates temporary stash VM to discover latest"
  bcu_doc_lines "Compares against RBRR_CHOSEN environment variables"
  bcu_doc_lines "Shows what mirror tag would be for latest version"
  bcu_doc_lines "Does NOT affect operational VM"
  bcu_doc_shown || return 0

  # Perform command
  zrbv_validate_pat

  zrbv_reset_stash || bcu_die "Failed to reset stash VM"

  bcu_step "Extracting natural tag from init output..."
  zrbv_extract_natural_tag "$ZRBV_STASH_INIT_STDOUT"
  local natural_tag=$(cat "${ZRBV_NATURAL_TAG_FILE}")
  bcu_info "Natural choice: $natural_tag"

  bcu_step "Installing crane in userspace..."
  zrbv_install_crane  "$RBRR_STASH_MACHINE"
  zrbv_registry_login "$RBRR_STASH_MACHINE"

  bcu_step "Using crane to get digest of natural_tag -> $natural_tag"
  podman machine ssh "$RBRR_STASH_MACHINE" "crane digest $natural_tag" > "${ZRBV_CRANE_DIGEST_FILE}"
  local natural_digest=$(cat "${ZRBV_CRANE_DIGEST_FILE}")
  bcu_info "Natural digest: $natural_digest"

  bcu_step "Comparing natural tag with expected RBRR values..."
  local expected_tag="${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}"
  if [ "$natural_tag" = "$expected_tag" ]; then
    bcu_info "Tag matches expected: $expected_tag"
  else
    bcu_warn "Tag mismatch: natural=$natural_tag, expected=$expected_tag"
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

  bcu_step "Generating canonical mirror tag for latest..."
  zrbv_generate_mirror_tag
  local mirror_fqin=$(cat "${ZRBV_MIRROR_TAG_FILE}")
  bcu_info "Canonical mirror tag: $mirror_fqin"

  bcu_step "Checking if this mirror exists in GHCR..."
  podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $mirror_fqin" \
    > "${ZRBV_CRANE_MANIFEST_CHECK_FILE}" 2>&1

  if [ $? -eq 0 ]; then
    bcu_info "Status: CURRENT (mirror exists)"
  else
    if [ "$natural_tag" != "$expected_tag" ]; then
      bcu_info "Status: UPDATE_AVAILABLE (newer version available)"
    else
      bcu_info "Status: NOT_MIRRORED (need to run rbv_stash)"
    fi
  fi

  bcu_step "Stopping stash VM..."
  podman machine stop "$RBRR_STASH_MACHINE"

  bcu_success "Check complete"
}

rbv_stash() {
  # Handle documentation mode
  bcu_doc_brief "Validate RBRR_CHOSEN values and create GHCR mirror"
  bcu_doc_lines "Ensures RBRR_CHOSEN values match podman's natural choice"
  bcu_doc_lines "Copies exact version to GHCR with canonical name"
  bcu_doc_lines "Destructive: removes all VMs before starting"
  bcu_doc_shown || return 0

  # Perform command
  zrbv_validate_pat

  zrbv_remove_vm "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Creating operational VM with natural podman init..."
  podman machine init --log-level=debug "$RBRR_OPERATIONAL_MACHINE"     \
                                     >  "$ZRBV_OPERATIONAL_INIT_STDOUT" \
                                     2> "$ZRBV_OPERATIONAL_INIT_STDERR"

  bcu_step "Starting operational VM..."
  podman machine start "$RBRR_OPERATIONAL_MACHINE"

  bcu_step "Stopping operational VM to proceed with stash..."
  podman machine stop "$RBRR_OPERATIONAL_MACHINE"

  zrbv_reset_stash || bcu_die "Failed to reset stash VM"

  bcu_step "Comparing operational and stash init stdout..."
  zrbv_error_if_different "${ZRBV_OPERATIONAL_INIT_STDOUT}" "${ZRBV_STASH_INIT_STDOUT}" || \
    bcu_die "Operational and stash VMs have different init outputs"

  bcu_step "Extracting tag from init output..."
  zrbv_extract_natural_tag "$ZRBV_STASH_INIT_STDOUT"
  local natural_tag=$(cat "${ZRBV_NATURAL_TAG_FILE}")
  bcu_info "Natural tag: $natural_tag"

  bcu_step "Validating matches RBRR_CHOSEN values..."
  local expected_tag="${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}"
  test "$natural_tag" = "$expected_tag" || \
    bcu_die "Natural choice ($natural_tag) doesn't match expected ($expected_tag)"

  bcu_step "Installing crane in userspace..."
  zrbv_install_crane  "$RBRR_STASH_MACHINE"
  zrbv_registry_login "$RBRR_STASH_MACHINE"

  bcu_step "Getting digest with crane..."
  podman machine ssh "$RBRR_STASH_MACHINE" "crane digest $natural_tag" > "${ZRBV_CRANE_DIGEST_FILE}"
  local digest=$(cat "${ZRBV_CRANE_DIGEST_FILE}")
  bcu_info "Digest: $digest"

  bcu_step "Validating matches RBRR_CHOSEN_VMIMAGE_SHA..."
  if [ -n "$RBRR_CHOSEN_VMIMAGE_SHA" ]; then
    test "$digest" = "$RBRR_CHOSEN_VMIMAGE_SHA" || \
      bcu_die "Digest ($digest) doesn't match RBRR_CHOSEN_VMIMAGE_SHA ($RBRR_CHOSEN_VMIMAGE_SHA)"
  fi

  bcu_step "Generating canonical mirror tag..."
  zrbv_generate_mirror_tag
  local mirror_fqin=$(cat "${ZRBV_MIRROR_TAG_FILE}")
  bcu_info "Mirror FQIN: $mirror_fqin"

  bcu_step "Checking if already exists in GHCR..."
  podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $mirror_fqin" \
    > "${ZRBV_CRANE_MANIFEST_CHECK_FILE}" 2>&1

  if [ $? -eq 0 ]; then
    bcu_info "Mirror already exists, skipping copy"
  else
    bcu_step "Copying with crane from quay to GHCR..."
    podman machine ssh "$RBRR_STASH_MACHINE" "crane copy $natural_tag $mirror_fqin" \
      > "${ZRBV_CRANE_COPY_OUTPUT_FILE}" 2>&1

    bcu_step "Verifying copy with crane manifest..."
    podman machine ssh "$RBRR_STASH_MACHINE" "crane manifest $mirror_fqin" \
      > "${ZRBV_CRANE_VERIFY_OUTPUT_FILE}" 2>&1 || \
      bcu_die "Failed to verify mirror in GHCR"
  fi

  bcu_step "Stopping stash VM..."
  podman machine stop "$RBRR_STASH_MACHINE"

  bcu_success "Stash complete: $mirror_fqin"
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

  bcu_step "Writing brand file to -> ${ZRBV_EMPLACED_BRAND_FILE}"
  zrbv_generate_brand_file
  podman machine ssh "$RBRR_OPERATIONAL_MACHINE" "sudo tee ${ZRBV_EMPLACED_BRAND_FILE}" \
                                                       < "${ZRBV_GENERATED_BRAND_FILE}"

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

  bcu_step "Reading brand file from -> ${ZRBV_EMPLACED_BRAND_FILE}"
  podman machine ssh "$RBRR_OPERATIONAL_MACHINE" "sudo cat ${ZRBV_EMPLACED_BRAND_FILE}" \
                                                           > "${ZRBV_FOUND_BRAND_FILE}" || \
    bcu_die "Failed to read brand file.  Is VM initialized?"

  bcu_step "Comparing generated and found brand files..."
  zrbv_generate_brand_file

  zrbv_error_if_different "${ZRBV_GENERATED_BRAND_FILE}" "${ZRBV_FOUND_BRAND_FILE}" || \
    bcu_die "Brand file doesn't match current RBRR_CHOSEN values"

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

