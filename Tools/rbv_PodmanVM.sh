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

zrbv_environment() {
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

  # Module Variables (ZRBV_*)
  ZRBV_GIT_REGISTRY="ghcr.io"

  ZRBV_IGNITE_INIT_STDOUT="${RBV_TEMP_DIR}/ignite_init_stdout.txt"
  ZRBV_IGNITE_INIT_STDERR="${RBV_TEMP_DIR}/ignite_init_stderr.txt"
  ZRBV_AVAILABLE_IMAGES="${RBV_TEMP_DIR}/available_disk_images.txt"
  ZRBV_PLATFORM_DIGESTS="${RBV_TEMP_DIR}/platform_digests.txt"

  ZRBV_PODMAN_REMOVE_PREFIX="${RBV_TEMP_DIR}/podman_inspect_remove_"
  ZRBV_MOW_MANIFEST_JSON="${RBV_TEMP_DIR}/mow_manifest.json"
  ZRBV_MOS_MANIFEST_JSON="${RBV_TEMP_DIR}/mos_manifest.json"
  ZRBV_MOW_ENTRIES_JSON="${RBV_TEMP_DIR}/mow_entries.json"
  ZRBV_MOS_ENTRIES_JSON="${RBV_TEMP_DIR}/mos_entries.json"
  ZRBV_MOW_DECODED_PREFIX="${RBV_TEMP_DIR}/mow_decoded_"
  ZRBV_MOS_DECODED_PREFIX="${RBV_TEMP_DIR}/mos_decoded_"

  ZRBV_VM_TEMP_DIR="/tmp/rbv-upload"
  ZRBV_VM_MANIFEST_PREFIX="${ZRBV_VM_TEMP_DIR}/manifest_"
  ZRBV_VM_BLOB_PREFIX="${ZRBV_VM_TEMP_DIR}/blob_"
  ZRBV_VM_DOCKERFILE_PREFIX="${ZRBV_VM_TEMP_DIR}/Dockerfile."

  ZRBV_BLOB_INFO="${RBV_TEMP_DIR}/blob_info.txt"
  ZRBV_LAYERS_JSON="${RBV_TEMP_DIR}/layers.json"
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
    podman machine init --log-level=debug      "${RBRR_IGNITE_MACHINE_NAME}" \
                                            2> "${ZRBV_IGNITE_INIT_STDERR}"  \
         | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "${ZRBV_IGNITE_INIT_STDOUT}"  \
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
        "sudo dnf install -y jq" || bcu_die "tools install fail"
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

# Helper function to process one image type
zrbv_process_image_type() {
  local manifest_file="$1"      # Manifest file path
  local entries_file="$2"       # Entries file path  
  local decoded_prefix="$3"     # Decoded prefix for output files
  local prefix="$4"             # Prefix for platform spec (mow/mos)
  local fqin="$5"               # Full image name
  local family_name="$6"        # Display name

  bcu_step "Retrieving manifest for ${family_name} image: ${fqin}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --        \
        "crane manifest ${fqin} | jq ." > "${manifest_file}" \
      || bcu_die "Failed to retrieve ${family_name} manifest"

  bcu_step "${family_name} Manifest:"
  cat "${manifest_file}"

  bcu_step "Extract manifest entries for ${family_name}..."
  jq -r '.manifests[] | @base64' "${manifest_file}" > "${entries_file}" || bcu_die "Failed to extract ${family_name} entries"

  zrbv_process_manifest_family "${entries_file}" "${decoded_prefix}" "${prefix}" "${family_name}"
}

# Updated zrbv_process_manifest_family using ZRBV_ variables
zrbv_process_manifest_family() {
  local entries_file="$1"
  local decoded_prefix="$2"
  local prefix="$3"
  local family_name="$4"

  bcu_step "${family_name} family images:"
  local entry_num=0
  while IFS= read -r entry; do
    entry_num=$((entry_num + 1))
    local       decoded="${decoded_prefix}${entry_num}.json"
    local     arch_file="${decoded_prefix}${entry_num}_arch.txt"
    local disktype_file="${decoded_prefix}${entry_num}_disktype.txt"
    local   digest_file="${decoded_prefix}${entry_num}_digest.txt"

    base64 -d <<< "${entry}"              > "${decoded}"
    jq -r '.platform.architecture'          "${decoded}" > "${arch_file}"
    jq -r '.annotations.disktype // "base"' "${decoded}" > "${disktype_file}"
    jq -r '.digest'                         "${decoded}" > "${digest_file}"

    local arch disktype digest
    arch=$(<"${arch_file}")
    disktype=$(<"${disktype_file}")
    digest=$(<"${digest_file}")
    local platform_spec="${prefix}_${arch}_${disktype}"

    echo "  ${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}-${platform_spec}"
    echo "${platform_spec}" >> "${ZRBV_AVAILABLE_IMAGES}"
    echo "${platform_spec}:${digest}" >> "${ZRBV_PLATFORM_DIGESTS}"
  done < "${entries_file}"
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

rbv_experiment() {
  # Handle documentation mode
  bcu_doc_brief "Process manifests for WSL and standard machine-os images, then upload needed disk images to GHCR"
  bcu_doc_lines "Queries quay.io/podman/machine-os-wsl and quay.io/podman/machine-os"
  bcu_doc_lines "Uses crane manifest to retrieve raw manifests, formatted with jq"
  bcu_doc_lines "Validates that all RBRR_NEEDED_DISK_IMAGES are available"
  bcu_doc_lines "Downloads needed disk images and packages them as container images"
  bcu_doc_lines "Builds all images locally, creates local manifest list, then single push to GHCR"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Prepare fresh ignite machine with crane and tools..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to create temp machine"

  bcu_step "Clear available images and platform digests files..."
  > "${ZRBV_AVAILABLE_IMAGES}"
  > "${ZRBV_PLATFORM_DIGESTS}"

  bcu_step "Potential container image names for caching:"

  # Process each image type
  zrbv_process_image_type        \
    "${ZRBV_MOW_MANIFEST_JSON}"  \
    "${ZRBV_MOW_ENTRIES_JSON}"   \
    "${ZRBV_MOW_DECODED_PREFIX}" \
    "mow" \
    "quay.io/podman/machine-os-wsl:${RBRR_CHOSEN_PODMAN_VERSION}" \
    "Machine-OS-WSL"

  zrbv_process_image_type        \
    "${ZRBV_MOS_MANIFEST_JSON}"  \
    "${ZRBV_MOS_ENTRIES_JSON}"   \
    "${ZRBV_MOS_DECODED_PREFIX}" \
    "mos" \
    "quay.io/podman/machine-os:${RBRR_CHOSEN_PODMAN_VERSION}" \
    "Machine-OS standard"

  bcu_step "Validating RBRR_NEEDED_DISK_IMAGES availability..."

  local missing_images=""
  for needed_image in ${RBRR_NEEDED_DISK_IMAGES}; do
    if ! grep -q "^${needed_image}$" "${ZRBV_AVAILABLE_IMAGES}"; then
      missing_images="${missing_images} ${needed_image}"
    else
      bcu_info "Found needed image: ${needed_image}"
    fi
  done

  if [[ -n "${missing_images}" ]]; then
    bcu_die "Missing required disk images:${missing_images}"      \
            "Available images:"                                   \
            "$(cat "${ZRBV_AVAILABLE_IMAGES}" | sed 's/^/  /')"
  fi

  bcu_success "All needed disk images are available in upstream manifests"

  local vm_name="${RBRR_IGNITE_MACHINE_NAME}"
  local timestamp=$(date +%Y%m%d_%H%M%S)
  local base_tag="${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}"
  local final_tag="${base_tag}-${timestamp}"
  local manifest_name="podvm-manifest-${timestamp}"

  bcu_step "Final manifest will be: ${final_tag}"

  bcu_step "Setting up VM upload directory..."
  podman machine ssh "$vm_name" --                            \
      "rm -rf ${ZRBV_VM_TEMP_DIR} && mkdir -p ${ZRBV_VM_TEMP_DIR}" \
    || bcu_die "Failed to create VM temp directory"

  zrbv_validate_pat || bcu_die "PAT validation failed"
  zrbv_login_ghcr "$vm_name" || bcu_die "Podman login failed"

  bcu_step "Creating local manifest list: ${manifest_name}"
  podman machine ssh "$vm_name" --               \
      "podman manifest create ${manifest_name}"  \
    || bcu_die "Failed to create local manifest"

  bcu_step "Building container images and adding to local manifest..."
  for needed_image in ${RBRR_NEEDED_DISK_IMAGES}; do
    bcu_step "Processing: ${needed_image}"

    local      digest=$(grep "^${needed_image}:" "${ZRBV_PLATFORM_DIGESTS}" | cut -d: -f2-)
    test -n "${digest}" || bcu_die "No digest found for ${needed_image}"

    local source_fqin
    if [[ "$needed_image" == mow_* ]]; then
      source_fqin="quay.io/podman/machine-os-wsl:${RBRR_CHOSEN_PODMAN_VERSION}"
    else
      source_fqin="quay.io/podman/machine-os:${RBRR_CHOSEN_PODMAN_VERSION}"
    fi

    local manifest_file="${ZRBV_VM_MANIFEST_PREFIX}${needed_image}.json"
    podman machine ssh "$vm_name" --                                 \
        "crane manifest ${source_fqin}@${digest} > ${manifest_file}" \
      || bcu_die "Failed to fetch manifest for ${needed_image}"

    bcu_step "Extracting disk blob info for ${needed_image}..."
    podman machine ssh "$vm_name" --                                 \
        "jq -r '.layers[]' ${manifest_file}" > "${ZRBV_LAYERS_JSON}" \
      || bcu_die "Failed to extract layers for ${needed_image}"

    jq -r 'select(.annotations."org.opencontainers.image.title" // .mediaType | test("disk|raw|tar|qcow2|machine")) | .digest + ":" + .mediaType' \
           "${ZRBV_LAYERS_JSON}" > "${ZRBV_BLOB_INFO}" \
      || bcu_die "Failed to find disk blob for ${needed_image}"

    local blob_info
    IFS= read -r blob_info < "${ZRBV_BLOB_INFO}" || true

    test -n "$blob_info" || bcu_die "No disk blob found in manifest for ${needed_image}"

    local blob_digest="${blob_info%:*}"
    local temp="${blob_info#*:}"
    local media_type="${temp#*:}"

    local extension="tar"
    case "$media_type" in
      *zstd*) extension="tar.zst" ;;
      *gzip*) extension="tar.gz"  ;;
      *xz*)   extension="tar.xz"  ;;
    esac

    local vm_blob_file="${ZRBV_VM_BLOB_PREFIX}${needed_image}.${extension}"
    podman machine ssh "$vm_name" --                                 \
        "crane blob ${source_fqin}@${blob_digest} > ${vm_blob_file}" \
      || bcu_die "Failed to download blob for ${needed_image}"

    local dockerfile="${ZRBV_VM_DOCKERFILE_PREFIX}${needed_image}"
    podman machine ssh "$vm_name" --           \
        "echo 'FROM scratch' > ${dockerfile}"  \
      || bcu_die "Failed to create Dockerfile for ${needed_image}"

    podman machine ssh "$vm_name" --                                                              \
        "echo 'COPY blob_${needed_image}.${extension} /disk-image.${extension}' >> ${dockerfile}" \
      || bcu_die "Failed to add COPY to Dockerfile for ${needed_image}"

    local local_tag="localhost/podvm-${needed_image}:${timestamp}"

    podman machine ssh "$vm_name" --                                                              \
        "cd ${ZRBV_VM_TEMP_DIR} && podman build -f Dockerfile.${needed_image} -t ${local_tag} ."  \
      || bcu_die "Failed to build image for ${needed_image}"

    local arch=$(echo "$needed_image" | cut -d_ -f2)
    local variant=""
    local os="linux"

    # Add to manifest with proper platform info
    podman machine ssh "$vm_name" --                                                  \
        "podman manifest add ${manifest_name} ${local_tag} --arch ${arch} --os ${os}" \
      || bcu_die "Failed to add ${local_tag} to manifest"

    bcu_info "Added to manifest: ${needed_image} (${arch})"
  done

  bcu_success "All ${#RBRR_NEEDED_DISK_IMAGES} container images built and added to manifest"

  bcu_step "Pushing manifest to GHCR: ${final_tag}"
  podman machine ssh "$vm_name" --                                  \
      "podman manifest push ${manifest_name} docker://${final_tag}" \
    || bcu_die "Failed to push manifest to GHCR"

  bcu_step "Cleaning up local manifest..."
  podman machine ssh "$vm_name" --          \
      "podman manifest rm ${manifest_name}" \
    || bcu_warn "Failed to remove local manifest"

  bcu_step "Cleaning up VM directory..."
  podman machine ssh "$vm_name" -- \
      "rm -rf ${ZRBV_VM_TEMP_DIR}" \
    || bcu_warn "Failed to clean up VM temp directory"

  bcu_success "All disk images uploaded to GHCR as: ${final_tag}"
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_environment "$@"


# eof

