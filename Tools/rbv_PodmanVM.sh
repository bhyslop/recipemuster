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

# Generate mirror tag using crane digest
zrbv_generate_mirror_tag() {
  # Read digest directly from file and extract short SHA
  local digest
  read -r digest < "${ZRBV_CRANE_DIGEST_FILE}" || bcu_die "Failed to read digest from ${ZRBV_CRANE_DIGEST_FILE}"
  test -n "$digest" || bcu_die "Failed to read digest from ${ZRBV_CRANE_DIGEST_FILE}"

  # Extract short SHA using parameter expansion instead of cut
  local sha_short="${digest:7:12}"  # Extract characters 8-19 (0-indexed)
  test -n "$sha_short" || bcu_die "Failed to extract short SHA from digest: $digest"

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
  podman machine init --log-level=debug      "$RBRR_STASH_MACHINE"      \
                                          2> "$ZRBV_STASH_INIT_STDERR"  \
       | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "$ZRBV_STASH_INIT_STDOUT"  \
    || bcu_die "Bad init."

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

# rbv_check - Check for newer upstream VM images
function rbv_check() {
    local zrbv_upstream_sha
    local zrbv_fqin_sha
    local zrbv_temp_machine="rbv-check-temp"
    local zrbv_temp_file="${RBV_TEMP_DIR}/rbv_check_output.txt"
    
    echo "Checking for newer VM images..."
    
    # Create temporary VM with crane (use existing stash machine logic)
    rbv_stash_create "${zrbv_temp_machine}" || bcu_die "Failed to create temp machine"
    
    # Query upstream for latest SHA
    echo "Querying ${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}..."
    podman machine ssh "${zrbv_temp_machine}" -- \
        crane digest "${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}" > "${zrbv_temp_file}" 2>&1
    
    if [[ ! -s "${zrbv_temp_file}" ]]; then
        podman machine rm -f "${zrbv_temp_machine}" >/dev/null 2>&1
        bcu_die "Failed to query upstream image"
    fi
    
    zrbv_upstream_sha=`cat "${zrbv_temp_file}"`
    
    # Check if FQIN differs from standard origin:version
    if [[ ! "${RBRR_CHOSEN_VMIMAGE_FQIN}" =~ ^"${RBRR_CHOSEN_VMIMAGE_ORIGIN}:" ]]; then
        echo "Checking custom FQIN: ${RBRR_CHOSEN_VMIMAGE_FQIN}..."
        podman machine ssh "${zrbv_temp_machine}" -- \
            crane digest "${RBRR_CHOSEN_VMIMAGE_FQIN}" > "${zrbv_temp_file}" 2>&1
        
        if [[ ! -s "${zrbv_temp_file}" ]]; then
            podman machine rm -f "${zrbv_temp_machine}" >/dev/null 2>&1
            bcu_die "Failed to query FQIN image"
        fi
        
        zrbv_fqin_sha=`cat "${zrbv_temp_file}"`
        
        if [[ "${zrbv_fqin_sha}" != "${zrbv_upstream_sha}" ]]; then
            echo "WARNING: FQIN SHA (${zrbv_fqin_sha}) differs from upstream SHA (${zrbv_upstream_sha})"
        fi
    fi
    
    # Cleanup temp VM
    podman machine rm -f "${zrbv_temp_machine}" >/dev/null 2>&1
    
    # Report findings
    echo "Current configured SHA: ${RBRR_CHOSEN_VMIMAGE_SHA:-<not set>}"
    echo "Latest upstream SHA: ${zrbv_upstream_sha}"
    
    if [[ -z "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
        echo "ACTION REQUIRED: Set RBRR_CHOSEN_VMIMAGE_SHA to: ${zrbv_upstream_sha}"
    elif [[ "${RBRR_CHOSEN_VMIMAGE_SHA}" != "${zrbv_upstream_sha}" ]]; then
        echo "UPDATE AVAILABLE: New SHA available: ${zrbv_upstream_sha}"
    else
        echo "Up to date!"
    fi
}

# rbv_mirror - Mirror VM image to GHCR
function rbv_mirror() {
    local zrbv_ghcr_tag
    local zrbv_ghcr_sha
    local zrbv_origin_sha
    local zrbv_init_output
    local zrbv_init_sha
    local zrbv_temp_file="${RBV_TEMP_DIR}/rbv_mirror_output.txt"
    
    echo "Mirroring VM image to GHCR..."
    
    # Validate preconditions if using standard origin
    if [[ "${RBRR_CHOSEN_VMIMAGE_FQIN}" =~ ^"${RBRR_CHOSEN_VMIMAGE_ORIGIN}:" ]]; then
        if [[ -n "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
            # Get origin SHA to verify
            crane digest "${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}" > "${zrbv_temp_file}" 2>&1
            if [[ ! -s "${zrbv_temp_file}" ]]; then
                bcu_die "Failed to query origin image"
            fi
            zrbv_origin_sha=`cat "${zrbv_temp_file}"`
            
            if [[ "${zrbv_origin_sha}" != "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
                bcu_die "RBRR_CHOSEN_VMIMAGE_SHA (${RBRR_CHOSEN_VMIMAGE_SHA}) does not match origin SHA (${zrbv_origin_sha}). Run rbv_check first"
            fi
        fi
    fi
    
    # Generate GHCR tag
    zrbv_ghcr_tag=`zrbv_generate_mirror_tag`
    
    # Check if already mirrored
    echo "Checking if already mirrored to ${zrbv_ghcr_tag}..."
    crane digest "${zrbv_ghcr_tag}" > "${zrbv_temp_file}" 2>&1
    
    if [[ -s "${zrbv_temp_file}" ]]; then
        zrbv_ghcr_sha=`cat "${zrbv_temp_file}"`
        if [[ -n "${RBRR_CHOSEN_VMIMAGE_SHA}" ]] && [[ "${zrbv_ghcr_sha}" != "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
            bcu_die "Existing GHCR image SHA (${zrbv_ghcr_sha}) does not match expected SHA (${RBRR_CHOSEN_VMIMAGE_SHA})"
        fi
        echo "Image already mirrored with matching SHA"
        return 0
    fi
    
    # Mirror the image
    echo "Copying ${RBRR_CHOSEN_VMIMAGE_FQIN} to ${zrbv_ghcr_tag}..."
    crane copy "${RBRR_CHOSEN_VMIMAGE_FQIN}" "${zrbv_ghcr_tag}" || bcu_die "Failed to copy image"
    
    # Verify the copy if SHA is defined
    if [[ -n "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
        crane digest "${zrbv_ghcr_tag}" > "${zrbv_temp_file}" 2>&1
        if [[ ! -s "${zrbv_temp_file}" ]]; then
            bcu_die "Failed to query mirrored image"
        fi
        zrbv_ghcr_sha=`cat "${zrbv_temp_file}"`
        
        if [[ "${zrbv_ghcr_sha}" != "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
            bcu_die "Mirrored image SHA (${zrbv_ghcr_sha}) does not match expected SHA (${RBRR_CHOSEN_VMIMAGE_SHA})"
        fi
    fi
    
    # Verify podman sees it correctly
    echo "Verifying podman machine init with mirrored image..."
    podman machine init --image "${RBRR_CHOSEN_VMIMAGE_FQIN}" test-mirror > "${zrbv_temp_file}" 2>&1
    cat "${zrbv_temp_file}" | ./Tools/rbupmis_Scrub.sh > "${RBV_TEMP_DIR}/rbv_mirror_sha.txt" 2>&1
    podman machine rm -f test-mirror >/dev/null 2>&1
    
    if [[ -s "${RBV_TEMP_DIR}/rbv_mirror_sha.txt" ]]; then
        zrbv_init_sha=`cat "${RBV_TEMP_DIR}/rbv_mirror_sha.txt"`
        if [[ -n "${RBRR_CHOSEN_VMIMAGE_SHA}" ]] && [[ "${zrbv_init_sha}" != "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
            bcu_die "Podman init SHA (${zrbv_init_sha}) does not match expected SHA (${RBRR_CHOSEN_VMIMAGE_SHA})"
        fi
    fi
    
    echo "Mirror successful!"
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

