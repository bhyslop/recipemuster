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

######################################################################
# WARNING: This implementation targets GHCR (GitHub Container Registry).
# It has NOT been ported to Google Artifact Registry (GAR/Depot).
# The operations are functionally correct but the registry target is
# obsolete for this project.
# See lenses/RBSPV-PodmanVmSupplyChain.adoc for context.
######################################################################

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBV_INCLUDED:-}" || buc_die "Module rbv multiply included - check sourcing hierarchy"
ZRBV_INCLUDED=1

######################################################################
# Internal Functions (zrbv_*)

zrbv_kindle() {
  # Build module variables from furnished environment (already validated by CLI)

  # Module Variables (ZRBV_*)
  ZRBV_GIT_REGISTRY="ghcr.io"

  ZRBV_IGNITE_INIT_STDOUT="${RBV_TEMP_DIR}/ignite_init_stdout.txt"
  ZRBV_IGNITE_INIT_STDERR="${RBV_TEMP_DIR}/ignite_init_stderr.txt"
  ZRBV_DEPLOY_INIT_STDOUT="${RBV_TEMP_DIR}/deploy_init_stdout.txt"
  ZRBV_DEPLOY_INIT_STDERR="${RBV_TEMP_DIR}/deploy_init_stderr.txt"

  ZRBV_FACT_DIGEST_PREFIX="${RBV_TEMP_DIR}/fact-digest-"
  ZRBV_FACT_SOURCE_PREFIX="${RBV_TEMP_DIR}/fact-source-"
  ZRBV_FACT_EXTENSION_PREFIX="${RBV_TEMP_DIR}/fact-extension-"
  ZRBV_FACT_ARCH_PREFIX="${RBV_TEMP_DIR}/fact-arch-"
  ZRBV_FACT_DISKTYPE_PREFIX="${RBV_TEMP_DIR}/fact-disktype-"

  ZRBV_EMPLACED_BRAND_FILE="${RBV_TEMP_DIR}/emplaced_brand_file.txt"
  ZRBV_GENERATED_BRAND_FILE="${RBV_TEMP_DIR}/generated_brand_file.txt"
  ZRBV_NATURAL_TAG_FILE="${RBV_TEMP_DIR}/natural_tag.txt"

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

  ZRBV_BLOB_INFO="${RBV_TEMP_DIR}/blob_info.txt"
  ZRBV_LAYERS_JSON="${RBV_TEMP_DIR}/layers.json"

  ZRBV_VMIMAGE_TAG_PREFIX="${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_PODMAN_VERSION}-"

  # Additional temp file paths
  ZRBV_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

  ZRBV_KINDLED=1
}

zrbv_sentinel() {
  # Guard that kindle ran before function execution
  test "${ZRBV_KINDLED:-}" = "1" || buc_die "Module rbv not kindled - call zrbv_kindle first"
}

# Generate brand file content
zrbv_generate_brand_file() {
  test ! -f "${ZRBV_GENERATED_BRAND_FILE}" || buc_die 'Brand file already exists'

  echo "# Recipe Bottle VM Brand File"                    > "${ZRBV_GENERATED_BRAND_FILE}"
  echo "#"                                               >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "PODMAN_VERSION: ${RBRR_CHOSEN_PODMAN_VERSION}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_ORIGIN: ${RBRR_CHOSEN_VMIMAGE_ORIGIN}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_FQIN:   ${RBRR_CHOSEN_VMIMAGE_FQIN}"     >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_DIGEST: ${RBRR_CHOSEN_VMIMAGE_DIGEST}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "IDENTITY:       ${RBRR_CHOSEN_IDENTITY}"         >> "${ZRBV_GENERATED_BRAND_FILE}"
}

# Extract natural tag from podman init output
zrbv_extract_natural_tag() {
  local z_init_output_file="$1"

  grep "Looking up Podman Machine image at" "${z_init_output_file}"        \
    | sed 's/.*Looking up Podman Machine image at \(.*\) to create VM/\1/' \
    > "${ZRBV_NATURAL_TAG_FILE}"

  test -s "${ZRBV_NATURAL_TAG_FILE}" || buc_die "Failed to extract natural tag from init output"
}

# Compare two files and return error code if different
zrbv_is_different_predicate() {
  local z_file1="$1"
  local z_file2="$2"

  local z_content1="$(<"${z_file1}")"
  local z_content2="$(<"${z_file2}")"

  if test "${z_content1}" = "${z_content2}"; then
    return 0
  else
    buc_warn "File content mismatch detected!"
    buc_warn "File 1 (${z_file1}) contents:"
    echo "${z_content1}"
    buc_warn "File 2 (${z_file2}) contents:"
    echo "${z_content2}"
    return 1
  fi
}

# Validate GitHub PAT environment
zrbv_validate_pat() {
  test -f "${RBRR_GITHUB_PAT_ENV}" || buc_die "GitHub PAT env file not found at ${RBRR_GITHUB_PAT_ENV}"

  test -n "${RBRG_PAT:-}"      || buc_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || buc_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"
}

# Stop and remove a VM if it exists
zrbv_remove_vm() {
  local z_vm_name="$1"

  if podman machine inspect "${z_vm_name}" > "${ZRBV_PODMAN_REMOVE_PREFIX}${z_vm_name}.txt"; then
    buc_info       "Stopping ${z_vm_name}..."
    podman machine stop     "${z_vm_name}" || buc_warn "Failed to stop ${z_vm_name} during _remove_vm"
    buc_info       "Removing ${z_vm_name}..."
    podman machine rm -f    "${z_vm_name}" || buc_die "Failed to remove ${z_vm_name}"
  else
    buc_info             "VM ${z_vm_name} does not exist. Nothing to remove."
  fi
}

# Prepare ignite machine: start if present, or fully reinit if force_reinit=true
zrbv_ignite_bootstrap() {
  local z_force_reinit="${1}"

  buc_info "Bootstrapping ignite machine: ${RBRR_IGNITE_MACHINE_NAME}"

  if podman machine inspect "${RBRR_IGNITE_MACHINE_NAME}" &>/dev/null; then
    buc_info "Stopping existing ignite machine (if running)..."
    podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" 2>/dev/null \
      || buc_die "Could not stop existing ignite VM cleanly"
  fi

  if test "${z_force_reinit}" = "true"; then
    buc_step "Removing existing ignite machine due to force_reinit..."
    zrbv_remove_vm "${RBRR_IGNITE_MACHINE_NAME}" || buc_die "Removal failed."
  fi

  if ! podman machine inspect "${RBRR_IGNITE_MACHINE_NAME}" &>/dev/null; then
    buc_step "Creating ignite VM with natural podman init..."
    podman machine init --log-level=debug      "${RBRR_IGNITE_MACHINE_NAME}" \
                                            2> "${ZRBV_IGNITE_INIT_STDERR}"  \
         | "${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh" "${ZRBV_IGNITE_INIT_STDOUT}"  \
      || buc_die "Bad init."

    buc_step "Starting ignite machine..."
    podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || buc_die "Failed to start ignite machine"

    buc_step "Installing crane..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                        \
        "curl -sL ${RBRR_CRANE_TAR_GZ} | sudo tar -xz -C /usr/local/bin crane" \
        || buc_die "crane fail"

    buc_step "Verify crane installation..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "crane version" || buc_die "crane confirm fail."

    buc_step "Installing tools..."
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "sudo dnf install -y jq" || buc_die "tools install fail"
  else
    buc_step "Restarting ignite machine..."
    podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || buc_die "Failed to restart ignite machine"
  fi

  buc_step "Ignite machine ready with skopeo installed"
}

# Login podman to github container registry in podman VM
zrbv_login_ghcr() {
  local z_vm_name="$1"

  buc_step "Login with podman..."
  podman -c "${z_vm_name}" login "${ZRBV_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
}

# Helper function to process one image type
zrbv_process_image_type() {
  local z_manifest_file="$1"
  local z_entries_file="$2"
  local z_decoded_prefix="$3"
  local z_prefix="$4"
  local z_fqin="$5"
  local z_family_name="$6"

  buc_step "Retrieving manifest for ${z_family_name} image: ${z_fqin}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --        \
        "crane manifest ${z_fqin} | jq ." > "${z_manifest_file}" \
      || buc_die "Failed to retrieve ${z_family_name} manifest"

  buc_step "${z_family_name} Manifest:"
  cat "${z_manifest_file}"

  buc_step "Extract manifest entries for ${z_family_name}..."
  jq -r '.manifests[] | @base64' "${z_manifest_file}" > "${z_entries_file}" \
    || buc_die "Failed to extract ${z_family_name} entries"

  buc_step "${z_family_name} family images:"
  local z_entry_num=0
  while IFS= read -r z_entry; do
    z_entry_num=$((z_entry_num + 1))
    local       z_decoded="${z_decoded_prefix}${z_entry_num}.json"
    local     z_arch_file="${z_decoded_prefix}${z_entry_num}_arch.txt"
    local z_disktype_file="${z_decoded_prefix}${z_entry_num}_disktype.txt"
    local   z_digest_file="${z_decoded_prefix}${z_entry_num}_digest.txt"

    base64 -d <<< "${z_entry}"              > "${z_decoded}"
    jq -r '.platform.architecture'          "${z_decoded}" > "${z_arch_file}"
    jq -r '.annotations.disktype // "base"' "${z_decoded}" > "${z_disktype_file}"
    jq -r '.digest'                         "${z_decoded}" > "${z_digest_file}"

    local z_arch z_disktype z_digest
    z_arch=$(<"${z_arch_file}")
    z_disktype=$(<"${z_disktype_file}")
    z_digest=$(<"${z_digest_file}")
    local z_platform_spec="${z_prefix}_${z_arch}_${z_disktype}"

    echo "${z_digest}"      > "${ZRBV_FACT_DIGEST_PREFIX}${z_platform_spec}"
    echo "${z_fqin}"        > "${ZRBV_FACT_SOURCE_PREFIX}${z_platform_spec}"
    echo "${z_arch}"        > "${ZRBV_FACT_ARCH_PREFIX}${z_platform_spec}"
    echo "${z_disktype}"    > "${ZRBV_FACT_DISKTYPE_PREFIX}${z_platform_spec}"

    echo "  ${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}-${z_platform_spec}"
  done < "${z_entries_file}"
}

######################################################################
# External Functions (rbv_*)

rbv_nuke() {
  # Name parameters
  local z_yes_opt="${1:-}"

  # Handle documentation mode
  buc_doc_brief "Completely reset the podman virtual machine environment"
  buc_doc_oparm "yes_opt" "If 'YES', then skip confirmation step"
  buc_doc_lines "Destroys ALL containers, VMs, and VM cache"
  buc_doc_lines "Requires explicit YES confirmation"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buv_dir_exists "${RBRS_PODMAN_ROOT_DIR}" || buc_warn "Podman directory not found."

  buc_step "WARNING: This will destroy all podman VMs and cache found in ${RBRS_PODMAN_ROOT_DIR}"

  # Skip confirmation if YES was passed as parameter
  test "${z_yes_opt}" = "YES" || buc_require "This will destroy all podman VMs and cache!" "YES"

  buc_step "Stopping all containers..."
  podman stop -a  || buc_warn "Attempt to stop all containers failed; normal if machine not started."

  buc_step "Removing all containers..."
  podman rm -a -f || buc_warn "Attempt to remove all containers failed; normal if machine not started."

  buc_step "Removing all podman machines..."
  for z_vm in $(podman machine list -q); do
    z_vm="${z_vm%\*}"  # Remove trailing asterisk indicating 'current vm'
    zrbv_remove_vm "${z_vm}" || buc_die "Attempt to remove VM ${z_vm} failed."
  done

  buc_step "Deleting VM cache directory..."
  rm -rf "${RBRS_PODMAN_ROOT_DIR}/machine"/*

  buc_success "Podman VM environment reset complete"
}

# Mirror VM images to GHCR
rbv_mirror() {
  # Handle documentation mode
  buc_doc_brief "Process manifests for WSL and standard machine-os images, then upload needed disk images to GHCR"
  buc_doc_lines "Queries quay.io/podman/machine-os-wsl and quay.io/podman/machine-os"
  buc_doc_lines "Uses crane manifest to retrieve raw manifests, formatted with jq"
  buc_doc_lines "Validates that all RBRR_MANIFEST_PLATFORMS are available"
  buc_doc_lines "Downloads needed disk images and packages them as container images"
  buc_doc_lines "Builds all images locally, creates local manifest list, then single push to GHCR"
  buc_doc_lines "Tags each platform image individually for direct access"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buc_step "Prepare fresh ignite machine with crane and tools..."
  zrbv_ignite_bootstrap false || buc_die "Failed to create temp machine"

  buc_step "Generate new identity for this build..."
  local z_new_identity
  z_new_identity=$(date +'%Y%m%d-%H%M%S')
  buc_info "New identity: ${z_new_identity}"

  buc_step "Potential container image names for caching:"

  # Process each image type
  zrbv_process_image_type        \
    "${ZRBV_MOW_MANIFEST_JSON}"  \
    "${ZRBV_MOW_ENTRIES_JSON}"   \
    "${ZRBV_MOW_DECODED_PREFIX}" \
    "mow"                        \
    "quay.io/podman/machine-os-wsl:${RBRR_CHOSEN_PODMAN_VERSION}" \
    "Machine-OS-WSL"

  zrbv_process_image_type        \
    "${ZRBV_MOS_MANIFEST_JSON}"  \
    "${ZRBV_MOS_ENTRIES_JSON}"   \
    "${ZRBV_MOS_DECODED_PREFIX}" \
    "mos"                        \
    "quay.io/podman/machine-os:${RBRR_CHOSEN_PODMAN_VERSION}" \
    "Machine-OS standard"

  buc_step "Validating RBRR_MANIFEST_PLATFORMS availability..."

  local z_missing_images=""
  for z_needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    if test ! -f "${ZRBV_FACT_DIGEST_PREFIX}${z_needed_image}"; then
      z_missing_images="${z_missing_images} ${z_needed_image}"
    else
      buc_info "Found needed image: ${z_needed_image}"
    fi
  done

  test -z "${z_missing_images}" || buc_die "Missing required disk images:${z_missing_images}"

  buc_step "All needed disk images are available in upstream manifests"

  buc_step "Setting up VM temporary dir..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
      "rm -rf ${ZRBV_VM_TEMP_DIR} && mkdir -p ${ZRBV_VM_TEMP_DIR}" \
    || buc_die "Failed to create VM temp directory"

  zrbv_validate_pat || buc_die "PAT validation failed"
  zrbv_login_ghcr "${RBRR_IGNITE_MACHINE_NAME}" || buc_die "Podman login failed"

  buc_step "Building container images..."
  for z_needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    buc_step "Processing: ${z_needed_image}"

    # Read facts
    local z_digest z_source_fqin
    z_digest=$(<"${ZRBV_FACT_DIGEST_PREFIX}${z_needed_image}")
    z_source_fqin=$(<"${ZRBV_FACT_SOURCE_PREFIX}${z_needed_image}")

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                       \
        "crane manifest ${z_source_fqin}@${z_digest} | jq -r '.layers[]'" > "${ZRBV_LAYERS_JSON}" \
      || buc_die "Failed to extract layers for ${z_needed_image}"

    jq -r 'select(.annotations."org.opencontainers.image.title" // .mediaType | test("disk|raw|tar|qcow2|machine")) | .digest + ":" + .mediaType' \
           "${ZRBV_LAYERS_JSON}" > "${ZRBV_BLOB_INFO}" \
      || buc_die "Failed to find disk blob for ${z_needed_image}"

    local z_blob_info
    IFS= read -r z_blob_info < "${ZRBV_BLOB_INFO}" || true

    test -n "${z_blob_info}" || buc_die "No disk blob found in manifest for ${z_needed_image}"

    local z_blob_digest="${z_blob_info%:*}"
    local z_temp="${z_blob_info#*:}"
    local z_media_type="${z_temp#*:}"

    local z_extension="tar"
    case "${z_media_type}" in
      *zstd*) z_extension="tar.zst" ;;
      *gzip*) z_extension="tar.gz"  ;;
      *xz*)   z_extension="tar.xz"  ;;
    esac

    echo "${z_extension}" > "${ZRBV_FACT_EXTENSION_PREFIX}${z_needed_image}"

    local z_vm_blob_file="${ZRBV_VM_BLOB_PREFIX}${z_needed_image}.${z_extension}"
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
        "crane blob ${z_source_fqin}@${z_blob_digest} > ${z_vm_blob_file}" \
      || buc_die "Failed to download blob for ${z_needed_image}"

    local z_dockerfile_name="Dockerfile.${z_needed_image}"
    local z_dockerfile_path="${ZRBV_VM_TEMP_DIR}/${z_dockerfile_name}"

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
        "echo '${z_new_identity}' > ${ZRBV_VM_TEMP_DIR}/identity.txt" \
      || buc_die "Failed to create identity file"

    {
        echo "FROM scratch"
        echo "COPY blob_${z_needed_image}.${z_extension} /disk-image.tar"
        echo "COPY identity.txt /identity.txt"
    } | podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "cat > ${z_dockerfile_path}"                          \
      || buc_die "Failed to create Dockerfile for ${z_needed_image}"

    local z_local_tag="localhost/podvm-${z_needed_image}:${z_new_identity}"

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                  \
        "cd ${ZRBV_VM_TEMP_DIR} && podman build -f ${z_dockerfile_name} -t ${z_local_tag} ." \
      || buc_die "Failed to build image for ${z_needed_image}"

  done

  buc_step "All container images built"

  buc_step "Tagging individual platform images..."
  for z_needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    local z_platform_tag="${ZRBV_VMIMAGE_TAG_PREFIX}${z_new_identity}-${z_needed_image}"
    local z_local_tag="localhost/podvm-${z_needed_image}:${z_new_identity}"

    buc_step "Tagging ${z_needed_image} as ${z_platform_tag}"
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
        "podman push ${z_local_tag} docker://${z_platform_tag}"         \
      || buc_die "Failed to push platform tag for ${z_needed_image}"
  done

  buc_step "Update your RBRR configuration:"
  buc_code ""
  buc_code "# Add to ${RBV_RBRR_FILE}:"
  buc_code "export RBRR_CHOSEN_IDENTITY=${z_new_identity}  # ${RBRR_MANIFEST_PLATFORMS}"
  buc_code ""

  buc_success "Platform tags created for: ${RBRR_MANIFEST_PLATFORMS}"
}

# Fetch VM image from GHCR container
rbv_fetch() {
  # Handle documentation mode
  buc_doc_brief "Fetch platform-specific VM image from GHCR to local cache"
  buc_doc_lines "Pulls platform-specific container image by tag"
  buc_doc_lines "Extracts disk image to RBRS_VMIMAGE_CACHE_DIR"
  buc_doc_lines "Always overwrites existing cached image"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buc_step "Validating platform configuration..."
  test -n "${RBRS_VM_PLATFORM}" || buc_die "RBRS_VM_PLATFORM not set in station config"

  buc_step "Ensuring cache directory exists..."
  mkdir -p "${RBRS_VMIMAGE_CACHE_DIR}" || buc_die "Failed to create cache directory"

  buc_step "Starting ignite VM to access podman image cache..."
  zrbv_ignite_bootstrap false || buc_die "Failed to start ignite machine"

  buc_step "Login to GitHub Container Registry..."
  zrbv_validate_pat || buc_die "PAT validation failed"
  zrbv_login_ghcr "${RBRR_IGNITE_MACHINE_NAME}" || buc_die "GHCR login failed"

  local z_platform_tag="${ZRBV_VMIMAGE_TAG_PREFIX}${RBRR_CHOSEN_IDENTITY}-${RBRS_VM_PLATFORM}"

  buc_step "Pulling platform-specific container: ${z_platform_tag}"
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" pull "${z_platform_tag}" \
    || buc_die "Failed to pull platform container: ${z_platform_tag}"

  buc_step "Creating temporary container from platform image..."
  local z_temp_container_id="${RBV_TEMP_DIR}/platform_container_id.txt"
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" create "${z_platform_tag}" > "${z_temp_container_id}" \
    || buc_die "Failed to create temporary container"

  local z_container_id
  z_container_id=$(<"${z_temp_container_id}")

  buc_step "Setting up VM temporary dir..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
      "rm -rf ${ZRBV_VM_TEMP_DIR} && mkdir -p ${ZRBV_VM_TEMP_DIR}" \
    || buc_die "Failed to create VM temp directory"

  buc_step "Extracting disk image from container..."
  local z_vm_temp_disk="${ZRBV_VM_TEMP_DIR}/extracted_disk_image.tar"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
      "podman cp ${z_container_id}:/disk-image.tar ${z_vm_temp_disk}" \
    || buc_die "Failed to extract disk image from container"

  local z_cache_file="${RBRS_VMIMAGE_CACHE_DIR}/${RBRS_VM_PLATFORM}-${RBRR_CHOSEN_IDENTITY}.tar"
  buc_step "Copying disk image to cache directory as -> ${z_cache_file}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "cat ${z_vm_temp_disk}" > "${z_cache_file}"         \
    || buc_die "Failed to copy disk image to cache"

  buc_step "Cleaning up temporary container..."
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" rm "${z_container_id}" \
    || buc_warn "Failed to remove temporary container"

  buc_step "Removing temporary files in VM..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "rm -f ${z_vm_temp_disk}"                         \
    || buc_warn "Failed to remove temporary disk file"

  buc_success "VM image cached: ${z_cache_file}"
}

rbv_init() {
  # Handle documentation mode
  buc_doc_brief "Initialize deploy VM from cached platform-specific image"
  buc_doc_lines "Uses cached disk image from RBRS_VMIMAGE_CACHE_DIR"
  buc_doc_lines "Initializes deploy VM with rootful mode"
  buc_doc_lines "Writes brand file with all RBRR_CHOSEN values"
  buc_doc_lines "Refuses if deploy VM already exists"
  buc_doc_lines "Requires rbv_fetch to be run first"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buc_step "Checking if deploy VM exists..."
  if podman machine list | grep -q "${RBRR_DEPLOY_MACHINE_NAME}"; then
    buc_die "Deploy VM already exists. Remove it first with rbv_nuke or manually"
  fi

  buc_step "Validating platform configuration..."
  test -n "${RBRS_VM_PLATFORM}" || buc_die "RBRS_VM_PLATFORM not set in station config"
  echo "${RBRR_MANIFEST_PLATFORMS}" | grep -q "${RBRS_VM_PLATFORM}" \
    || buc_die "Platform ${RBRS_VM_PLATFORM} not in manifest platforms: ${RBRR_MANIFEST_PLATFORMS}"

  local z_cache_file="${RBRS_VMIMAGE_CACHE_DIR}/${RBRS_VM_PLATFORM}-${RBRR_CHOSEN_IDENTITY}.tar"

  buc_step "Checking for cached VM image..."
  test -f "${z_cache_file}" \
    || buc_die "VM image not found in cache: ${z_cache_file}" \
               "Run 'rbv_fetch' first to download the VM image"

  buc_step "Initializing machine from cached image..."
  podman machine init --rootful --image "${z_cache_file}" "${RBRR_DEPLOY_MACHINE_NAME}" \
                                          2> "${ZRBV_DEPLOY_INIT_STDERR}"             \
       | "${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh" "${ZRBV_DEPLOY_INIT_STDOUT}"             \
    || buc_die "Failed to initialize VM"

  buc_step "Starting VM to write brand file..."
  podman machine start "${RBRR_DEPLOY_MACHINE_NAME}" || buc_die "Failed to start deploy VM"

  buc_step "Generating brand file content..."
  local z_brand_file="${RBV_TEMP_DIR}/brand.txt"
  zrbv_generate_brand_file "${z_brand_file}" \
    || buc_die "Failed to generate brand file"

  buc_step "Writing brand file to VM: ${ZRBV_EMPLACED_BRAND_FILE}"
  podman machine ssh "${RBRR_DEPLOY_MACHINE_NAME}" \
      "sudo tee ${ZRBV_EMPLACED_BRAND_FILE}" < "${z_brand_file}" > /dev/null \
    || buc_die "Failed to write brand file to VM"

  buc_step "Stopping VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}" \
    || buc_die "Failed to stop deploy VM"

  buc_success "Deploy VM initialized with brand file"
}

rbv_start() {
  # Handle documentation mode
  buc_doc_brief "Start Deploy VM"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buc_step "Starting Deploy VM..."
  podman machine start "${RBRR_DEPLOY_MACHINE_NAME}"

  buc_success "Deploy VM started"
}

rbv_stop() {
  # Handle documentation mode
  buc_doc_brief "Stop Deploy VM"
  buc_doc_shown || return 0

  # Ensure module started
  zrbv_sentinel

  # Perform command
  buc_step "Stopping deploy VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}"

  buc_success "Deploy VM stopped"
}

# eof

