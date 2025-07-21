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

set -euo pipefail

ZRBV_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBV_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBV_SCRIPT_DIR}/bvu_BashValidationUtility.sh"


######################################################################
# Internal Functions (zrbv_*)

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

  # Module Variables (ZRBV_*) - only keeping used ones
  ZRBV_GIT_REGISTRY="ghcr.io"

  ZRBV_IGNITE_INIT_STDOUT="${RBV_TEMP_DIR}/ignite_init_stdout.txt"
  ZRBV_IGNITE_INIT_STDERR="${RBV_TEMP_DIR}/ignite_init_stderr.txt"

  ZRBV_PODMAN_REMOVE_PREFIX="${RBV_TEMP_DIR}/podman_inspect_remove_"
}

# Generate brand file content
zrbv_generate_brand_file() {
  bcu_die "BRADTODO: ELIDED."
}

# Confirm YES parameter or prompt user
zrbv_confirm_yes() {
  bcu_step "BRADTODO: ELIDED."
  return 0
}

# Extract natural tag from podman init output
zrbv_extract_natural_tag() {
  bcu_die "BRADTODO: ELIDED."
}

# Get image digest using skopeo inspect
zrbv_get_image_digest() {
  bcu_die "BRADTODO: ELIDED."
}

# Generate mirror tag using crane digest
zrbv_generate_mirror_tag() {
  bcu_die "BRADTODO: ELIDED."
}

# Compare two files and return error if different
zrbv_error_if_different() {
  bcu_die "BRADTODO: ELIDED."
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

  if podman machine inspect "$vm_name" > "${ZRBV_PODMAN_REMOVE_PREFIX}${vm_name}.txt"; then
    bcu_info       "Stopping $vm_name..."
    podman machine stop     "$vm_name" || bcu_warn "Failed to stop $vm_name during _remove_vm"
    bcu_info       "Removing $vm_name..."
    podman machine rm -f    "$vm_name" || bcu_die "Failed to remove $vm_name"
  else
    bcu_info             "VM $vm_name does not exist. Nothing to remove."
  fi
}

# Prepare ignite machine: start if present, or fully reinit if force_reinit=true
zrbv_ignite_bootstrap() {
  local force_reinit="${1}"

  bcu_info "Bootstrapping ignite machine: ${RBRR_IGNITE_MACHINE_NAME}"

  if podman machine inspect "${RBRR_IGNITE_MACHINE_NAME}" &>/dev/null; then
    bcu_info "Stopping existing ignite machine (if running)..."
    podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" 2>/dev/null || \
      bcu_die "Could not stop existing ignite VM cleanly"
  fi

  if [ "$force_reinit" = true ]; then
    bcu_step "Removing existing ignite machine due to force_reinit..."
    zrbv_remove_vm "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Removal failed."
  fi

  if ! podman machine inspect "${RBRR_IGNITE_MACHINE_NAME}" &>/dev/null; then
    bcu_step "Creating ignite VM with natural podman init..."
    podman machine init --log-level=debug     "${RBRR_IGNITE_MACHINE_NAME}" \
                                            2> "$ZRBV_IGNITE_INIT_STDERR"   \
         | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "$ZRBV_IGNITE_INIT_STDOUT"   \
      || bcu_die "Bad init."

    bcu_step "Starting ignite machine..."
    podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to start ignite machine"

    bcu_step "Installing crane..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                        \
        "curl -sL ${RBRR_CRANE_TAR_GZ} | sudo tar -xz -C /usr/local/bin crane" \
        || bcu_die "crane fail"

    bcu_step "Verify crane installation..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "crane version" || bcu_die "crane confirm fail."

    bcu_step "Installing tools..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "sudo dnf install -y skopeo jq" || bcu_die "tools install fail"

    bcu_step "Verify skopeo installation..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "skopeo --version" || bcu_die "skopeo confirm fail."

    bcu_step "Installing oras v1.2.3 into ignite VM..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "curl -sSfL https://github.com/oras-project/oras/releases/download/v1.2.3/oras_1.2.3_linux_amd64.tar.gz | tar -xz oras && sudo install -m 0755 oras /usr/local/bin/oras" \
      || bcu_die "oras install fail"

    bcu_step "Verifying oras installation..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "oras version" || bcu_die "oras confirm fail"


  else
    bcu_step "Restarting ignite machine..."
    podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to restart ignite machine"
  fi

  bcu_step "Ignite machine ready with skopeo installed"
}

# Login podman to github container registry in podman VM
zrbv_login_ghcr() {
  local vm_name="$1"

  source "${RBRR_GITHUB_PAT_ENV}"

  bcu_step "Login with podman..."
  podman -c "$vm_name" login "${ZRBV_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
}

# Login crane to github container registry in podman VM
zrbv_login_crane() {
  local vm_name="$1"

  source "${RBRR_GITHUB_PAT_ENV}"

  bcu_step "Login with crane..."
  podman machine ssh "$vm_name" "crane auth login ${ZRBV_GIT_REGISTRY} -u ${RBRG_USERNAME} -p ${RBRG_PAT}"
}

######################################################################
# External Functions (rbv_*)

rbv_nuke() {
  # Name parameters
  local yes_opt="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Completely reset the podman virtual machine environment"
  bcu_doc_oparm "yes_opt" "If 'YES', then skip confirmation step"
  bcu_doc_lines "Destroys ALL containers, VMs, and VM cache"
  bcu_doc_lines "Requires explicit YES confirmation"
  bcu_doc_shown || return 0

  # Perform command
  bvu_dir_exists "${RBRS_PODMAN_ROOT_DIR}" || bcu_warn "Podman directory not found."

  bcu_step "WARNING: This will destroy all podman VMs and cache found in ${RBRS_PODMAN_ROOT_DIR}"
  zrbv_confirm_yes "$yes_opt" || bcu_die "Nuke not confirmed, exit without change"

  bcu_step "Stopping all containers..."
  podman stop -a  || bcu_warn "Attempt to stop all containers did not succeed; okay if machine not started."

  bcu_step "Removing all containers..."
  podman rm -a -f || bcu_warn "Attempt to remove all containers failed; okay if machine not started."

  bcu_step "Removing all podman machines..."
  for vm in $(podman machine list -q); do
    vm="${vm%\*}"  # Remove trailing asterisk indicating 'current vm'
    zrbv_remove_vm "$vm" || bcu_die "Attempt to remove VM $vm failed."
  done

  bcu_step "Deleting VM cache directory..."
  rm -rf "${RBRS_PODMAN_ROOT_DIR}/machine"/*

  bcu_success "Podman VM environment reset complete"
}

rbv_check() {
  bcu_die "BRADTODO: ELIDED."
}

# Mirror VM image to GHCR
rbv_mirror() {
  bcu_die "BRADTODO: ELIDED."
}

# Fetch VM image from GHCR container
rbv_fetch() {
  bcu_die "BRADTODO: ELIDED."
}

rbv_init() {
  bcu_die "BRADTODO: ELIDED."
}

rbv_start() {
  bcu_die "BRADTODO: ELIDED."
}

rbv_stop() {
  bcu_die "BRADTODO: ELIDED."
}

# Process manifest entries and add platform specs to available_images_file
zrbv_process_manifest_family() {
  local entries_file="$1"
  local prefix="$2"
  local family_name="$3"

  bcu_step "${family_name} family images:"
  while IFS= read -r entry; do
    local decoded=$(echo "${entry}" | base64 -d)
    local arch=$(echo "${decoded}" | jq -r '.platform.architecture')
    local disktype=$(echo "${decoded}" | jq -r '.annotations.disktype // "base"')
    local platform_spec="${prefix}_${arch}_${disktype}"
    local digest=$(echo "${decoded}" | jq -r '.digest')

    echo "  ${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}-${platform_spec}"
    echo "${platform_spec}" >> "${available_images_file}"

    # Store digest for later download
    echo "${platform_spec}:${digest}" >> "${RBV_TEMP_DIR}/platform_digests.txt"
  done < "${entries_file}"
}

# Download needed disk images from VM to host
zrbv_download_needed_images() {
  local vm_name="${RBRR_IGNITE_MACHINE_NAME}"
  local vm_temp_dir="/tmp/rbv-downloads"
  local platform_digests_file="${RBV_TEMP_DIR}/platform_digests.txt"

  bcu_step "Setting up VM download directory..."
  podman machine ssh "$vm_name" --                       \
      "rm -rf ${vm_temp_dir} && mkdir -p ${vm_temp_dir}" \
    || bcu_die "Failed to create VM temp directory"

  for needed_image in ${RBRR_NEEDED_DISK_IMAGES}; do
    bcu_step "Processing needed image: ${needed_image}"

    bcu_step "re: ${needed_image}: Find the digest for this platform spec..."
    local digest=$(grep "^${needed_image}:" "${platform_digests_file}" | cut -d: -f2-)
    test -n "$digest" || bcu_die "No digest found for ${needed_image}"

    bcu_info "Manifest digest: ${digest}"

    bcu_step "re: ${needed_image}: Determine source FQIN based on prefix..."
    local source_fqin
    if [[ "$needed_image" == mow_* ]]; then
      source_fqin="quay.io/podman/machine-os-wsl:${RBRR_CHOSEN_PODMAN_VERSION}"
    else
      source_fqin="quay.io/podman/machine-os:${RBRR_CHOSEN_PODMAN_VERSION}"
    fi

    bcu_step "re: ${needed_image}: Fetch individual manifest to get blob info..."
    local manifest_file="${vm_temp_dir}/${needed_image}_manifest.json"
    bcu_step "Fetching individual manifest for ${needed_image}..."
    podman machine ssh "$vm_name" -- \
      "crane manifest ${source_fqin}@${digest} > ${manifest_file}" \
      || bcu_die "Failed to fetch manifest for ${needed_image}"

    bcu_step "re: ${needed_image}: Extract blob digest and mediaType for disk image layer..."
    local blob_info=$(podman machine ssh "$vm_name" -- \
      "jq -r '.layers[] | select(.annotations.\"org.opencontainers.image.title\" // .mediaType | test(\"disk|raw|tar\")) | .digest + \":\" + .mediaType' ${manifest_file} | head -1") \
      || bcu_die "Failed to extract blob info for ${needed_image}"

    test -n "$blob_info" || bcu_die "No disk blob found in manifest for ${needed_image}"

    local blob_digest=$(echo "$blob_info" | cut -d: -f1-2)  # Include sha256: prefix
    local media_type=$(echo "$blob_info" | cut -d: -f3-)

    bcu_info "Blob digest: ${blob_digest}"
    bcu_info "Media type: ${media_type}"

    bcu_step "re: ${needed_image}: Determine file extension from media type..."
    local extension="tar"
    if [[ "$media_type" == *"zstd"* ]]; then
      extension="tar.zst"
    elif [[ "$media_type" == *"gzip"* ]]; then
      extension="tar.gz"
    elif [[ "$media_type" == *"xz"* ]]; then
      extension="tar.xz"
    fi

    bcu_step "re: ${needed_image}: Download blob to VM..."
    local vm_blob_file="${vm_temp_dir}/${needed_image}.${extension}"
    bcu_step "Downloading blob for ${needed_image}..."
    podman machine ssh "$vm_name" --                                 \
        "crane blob ${source_fqin}@${blob_digest} > ${vm_blob_file}" \
      || bcu_die "Failed to download blob for ${needed_image}"

    bcu_step "re: ${needed_image}: Copy from VM to host..."
    local host_file="${RBRS_VMIMAGE_CACHE_DIR}/podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}-${needed_image}.${extension}"
    bcu_step "Copying ${needed_image} from VM to host..."

    mkdir -p "${RBRS_VMIMAGE_CACHE_DIR}" || bcu_die "Failed to create cache directory"

    podman machine ssh "$vm_name" -- "cat ${vm_blob_file}" > "${host_file}" \
      || bcu_die "Failed to copy ${needed_image} to host"

    bcu_info "Downloaded: ${host_file}"
    bcu_info "Expected blob digest for podman machine init: ${blob_digest}"
  done

  bcu_step "Cleaning up VM download directory..."
  podman machine ssh "$vm_name" -- \
      "rm -rf ${vm_temp_dir}"      \
    || bcu_warn "Failed to clean up VM temp directory"
}

rbv_experiment() {
  # Handle documentation mode
  bcu_doc_brief "Display raw manifests for WSL and standard machine-os images, then download needed disk images"
  bcu_doc_lines "Queries quay.io/podman/machine-os-wsl and quay.io/podman/machine-os"
  bcu_doc_lines "Uses crane manifest to retrieve raw manifests, formatted with jq"
  bcu_doc_lines "Generates all potential container image names for caching"
  bcu_doc_lines "Validates that all RBRR_NEEDED_DISK_IMAGES are available"
  bcu_doc_lines "Downloads needed disk images to RBRS_VMIMAGE_CACHE_DIR"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Prepare fresh ignite machine with crane and tools..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to create temp machine"

  local wsl_fqin="quay.io/podman/machine-os-wsl:${RBRR_CHOSEN_PODMAN_VERSION}"
  local std_fqin="quay.io/podman/machine-os:${RBRR_CHOSEN_PODMAN_VERSION}"

  # Temporary files for manifest processing
  local wsl_manifest_file="${RBV_TEMP_DIR}/wsl_manifest.json"
  local std_manifest_file="${RBV_TEMP_DIR}/std_manifest.json"
  local wsl_entries_file="${RBV_TEMP_DIR}/wsl_entries.json"
  local std_entries_file="${RBV_TEMP_DIR}/std_entries.json"
  local available_images_file="${RBV_TEMP_DIR}/available_disk_images.txt"

  bcu_step "Retrieving manifest for WSL image: ${wsl_fqin}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                \
        "crane manifest ${wsl_fqin} | jq ." > "${wsl_manifest_file}" \
      || bcu_die "Failed to retrieve WSL manifest"

  bcu_step "Retrieving manifest for standard image: ${std_fqin}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                \
        "crane manifest ${std_fqin} | jq ." > "${std_manifest_file}" \
      || bcu_die "Failed to retrieve standard manifest"

  # Display the manifests
  bcu_step "WSL Manifest:"
  cat "${wsl_manifest_file}"

  bcu_step "Standard Manifest:"
  cat "${std_manifest_file}"

  bcu_step "Extract manifest entries for processing..."
  jq -r '.manifests[] | @base64' "${wsl_manifest_file}" > "${wsl_entries_file}" || bcu_die "Failed to extract WSL entries"
  jq -r '.manifests[] | @base64' "${std_manifest_file}" > "${std_entries_file}" || bcu_die "Failed to extract standard entries"

  bcu_step "Clear available images and platform digests files..."
  > "${available_images_file}"
  > "${RBV_TEMP_DIR}/platform_digests.txt"

  bcu_step "Potential container image names for caching:"
  zrbv_process_manifest_family "${wsl_entries_file}" "mow" "Machine-OS-WSL"
  zrbv_process_manifest_family "${std_entries_file}" "mos" "Machine-OS standard"

  bcu_step "Validating RBRR_NEEDED_DISK_IMAGES availability..."

  local missing_images=""
  for needed_image in ${RBRR_NEEDED_DISK_IMAGES}; do
    if ! grep -q "^${needed_image}$" "${available_images_file}"; then
      missing_images="${missing_images} ${needed_image}"
    else
      bcu_info "? Found needed image: ${needed_image}"
    fi
  done

  if [[ -n "${missing_images}" ]]; then
    bcu_die "Missing required disk images:${missing_images}"      \
            "Available images:"                                   \
            "$(cat "${available_images_file}" | sed 's/^/  /')"
  fi

  bcu_success "All needed disk images are available in upstream manifests"

  bcu_step "Downloading needed disk images..."
  zrbv_download_needed_images || bcu_die "Failed to download needed images"

  bcu_success "All needed disk images downloaded to ${RBRS_VMIMAGE_CACHE_DIR}"
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_validate_envvars "$@"


# eof

