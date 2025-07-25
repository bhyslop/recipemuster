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
  ZRBV_DEPLOY_INIT_STDOUT="${RBV_TEMP_DIR}/deploy_init_stdout.txt"
  ZRBV_DEPLOY_INIT_STDERR="${RBV_TEMP_DIR}/deploy_init_stderr.txt"

  ZRBV_FACT_DIGEST_PREFIX="${RBV_TEMP_DIR}/fact-digest-"
  ZRBV_FACT_SOURCE_PREFIX="${RBV_TEMP_DIR}/fact-source-"
  ZRBV_FACT_EXTENSION_PREFIX="${RBV_TEMP_DIR}/fact-extension-"
  ZRBV_FACT_ARCH_PREFIX="${RBV_TEMP_DIR}/fact-arch-"
  ZRBV_FACT_DISKTYPE_PREFIX="${RBV_TEMP_DIR}/fact-disktype-"

  ZRBV_EMPLACED_BRAND_FILE="${RBV_TEMP_DIR}/emplaced_brand_file.txt"

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

  if podman machine inspect "${vm_name}" > "${ZRBV_PODMAN_REMOVE_PREFIX}${vm_name}.txt"; then
    bcu_info       "Stopping ${vm_name}..."
    podman machine stop     "${vm_name}" || bcu_warn "Failed to stop ${vm_name} during _remove_vm"
    bcu_info       "Removing ${vm_name}..."
    podman machine rm -f    "${vm_name}" || bcu_die "Failed to remove ${vm_name}"
  else
    bcu_info             "VM ${vm_name} does not exist. Nothing to remove."
  fi
}

# Prepare ignite machine: start if present, or fully reinit if force_reinit=true
zrbv_ignite_bootstrap() {
  local force_reinit="${1}"

  bcu_info "Bootstrapping ignite machine: ${RBRR_IGNITE_MACHINE_NAME}"

  if podman machine inspect "${RBRR_IGNITE_MACHINE_NAME}" &>/dev/null; then
    bcu_info "Stopping existing ignite machine (if running)..."
    podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" 2>/dev/null \
      || bcu_die "Could not stop existing ignite VM cleanly"
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
  podman -c "${vm_name}" login "${ZRBV_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
}

# Helper function to process one image type
zrbv_process_image_type() {
  local manifest_file="$1"
  local entries_file="$2"
  local decoded_prefix="$3"
  local prefix="$4"
  local fqin="$5"
  local family_name="$6"

  bcu_step "Retrieving manifest for ${family_name} image: ${fqin}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --        \
        "crane manifest ${fqin} | jq ." > "${manifest_file}" \
      || bcu_die "Failed to retrieve ${family_name} manifest"

  bcu_step "${family_name} Manifest:"
  cat "${manifest_file}"

  bcu_step "Extract manifest entries for ${family_name}..."
  jq -r '.manifests[] | @base64' "${manifest_file}" > "${entries_file}" \
    || bcu_die "Failed to extract ${family_name} entries"

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

    echo "${digest}"      > "${ZRBV_FACT_DIGEST_PREFIX}${platform_spec}"
    echo "${fqin}"        > "${ZRBV_FACT_SOURCE_PREFIX}${platform_spec}"
    echo "${arch}"        > "${ZRBV_FACT_ARCH_PREFIX}${platform_spec}"
    echo "${disktype}"    > "${ZRBV_FACT_DISKTYPE_PREFIX}${platform_spec}"

    echo "  ${ZRBV_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:podvm-${RBRR_CHOSEN_IDENTITY}-${RBRR_CHOSEN_PODMAN_VERSION}-${platform_spec}"
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

zrbv_check() {
  bcu_die "Unimplemented"
}

# Mirror VM images to GHCR
rbv_mirror() {
  # Handle documentation mode
  bcu_doc_brief "Process manifests for WSL and standard machine-os images, then upload needed disk images to GHCR"
  bcu_doc_lines "Queries quay.io/podman/machine-os-wsl and quay.io/podman/machine-os"
  bcu_doc_lines "Uses crane manifest to retrieve raw manifests, formatted with jq"
  bcu_doc_lines "Validates that all RBRR_MANIFEST_PLATFORMS are available"
  bcu_doc_lines "Downloads needed disk images and packages them as container images"
  bcu_doc_lines "Builds all images locally, creates local manifest list, then single push to GHCR"
  bcu_doc_lines "Tags each platform image individually for direct access"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Prepare fresh ignite machine with crane and tools..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to create temp machine"

  bcu_step "Generate new identity for this build..."
  local new_identity=$(date +'%Y%m%d-%H%M%S')
  bcu_info "New identity: ${new_identity}"

  bcu_step "Potential container image names for caching:"

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

  bcu_step "Validating RBRR_MANIFEST_PLATFORMS availability..."

  local missing_images=""
  for needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    if [[ ! -f "${ZRBV_FACT_DIGEST_PREFIX}${needed_image}" ]]; then
      missing_images="${missing_images} ${needed_image}"
    else
      bcu_info "Found needed image: ${needed_image}"
    fi
  done

  if [[ -n "${missing_images}" ]]; then
    bcu_die "Missing required disk images:${missing_images}"
  fi

  bcu_step "All needed disk images are available in upstream manifests"

  bcu_step "Setting up VM temporary dir..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
      "rm -rf ${ZRBV_VM_TEMP_DIR} && mkdir -p ${ZRBV_VM_TEMP_DIR}" \
    || bcu_die "Failed to create VM temp directory"

  zrbv_validate_pat || bcu_die "PAT validation failed"
  zrbv_login_ghcr "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Podman login failed"

  bcu_step "Building container images..."
  for needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    bcu_step "Processing: ${needed_image}"

    # Read facts
    local digest=$(<"${ZRBV_FACT_DIGEST_PREFIX}${needed_image}")
    local source_fqin=$(<"${ZRBV_FACT_SOURCE_PREFIX}${needed_image}")

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                       \
        "crane manifest ${source_fqin}@${digest} | jq -r '.layers[]'" > "${ZRBV_LAYERS_JSON}" \
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

    echo "${extension}" > "${ZRBV_FACT_EXTENSION_PREFIX}${needed_image}"

    local vm_blob_file="${ZRBV_VM_BLOB_PREFIX}${needed_image}.${extension}"
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
        "crane blob ${source_fqin}@${blob_digest} > ${vm_blob_file}" \
      || bcu_die "Failed to download blob for ${needed_image}"

    local dockerfile_name="Dockerfile.${needed_image}"
    local dockerfile_path="${ZRBV_VM_TEMP_DIR}/${dockerfile_name}"

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
        "echo '${new_identity}' > ${ZRBV_VM_TEMP_DIR}/identity.txt" \
      || bcu_die "Failed to create identity file"

    {
        echo "FROM scratch"
        echo "COPY blob_${needed_image}.${extension} /disk-image.tar"
        echo "COPY identity.txt /identity.txt"
    } | podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
        "cat > ${dockerfile_path}"                          \
      || bcu_die "Failed to create Dockerfile for ${needed_image}"

    local local_tag="localhost/podvm-${needed_image}:${new_identity}"

    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                  \
        "cd ${ZRBV_VM_TEMP_DIR} && podman build -f ${dockerfile_name} -t ${local_tag} ." \
      || bcu_die "Failed to build image for ${needed_image}"

  done

  bcu_step "All container images built"

  bcu_step "Tagging individual platform images..."
  for needed_image in ${RBRR_MANIFEST_PLATFORMS}; do
    local platform_tag="${ZRBV_VMIMAGE_TAG_PREFIX}${new_identity}-${needed_image}"
    local local_tag="localhost/podvm-${needed_image}:${new_identity}"

    bcu_step "Tagging ${needed_image} as ${platform_tag}"
    podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
        "podman push ${local_tag} docker://${platform_tag}"         \
      || bcu_die "Failed to push platform tag for ${needed_image}"
  done

  bcu_step "Update your RBRR configuration:"
  bcu_code ""
  bcu_code "# Add to ${RBV_RBRR_FILE}:"
  bcu_code "export RBRR_CHOSEN_IDENTITY=${new_identity}  # ${RBRR_MANIFEST_PLATFORMS}"
  bcu_code ""

  bcu_success "Platform tags created for: ${RBRR_MANIFEST_PLATFORMS}"
}

# Fetch VM image from GHCR container
rbv_fetch() {
  # Handle documentation mode
  bcu_doc_brief "Fetch platform-specific VM image from GHCR to local cache"
  bcu_doc_lines "Pulls platform-specific container image by tag"
  bcu_doc_lines "Extracts disk image to RBRS_VMIMAGE_CACHE_DIR"
  bcu_doc_lines "Always overwrites existing cached image"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Validating platform configuration..."
  test -n "${RBRS_VM_PLATFORM}" || bcu_die "RBRS_VM_PLATFORM not set in station config"

  bcu_step "Ensuring cache directory exists..."
  mkdir -p "${RBRS_VMIMAGE_CACHE_DIR}" || bcu_die "Failed to create cache directory"

  bcu_step "Starting ignite VM to access podman image cache..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to start ignite machine"

  bcu_step "Login to GitHub Container Registry..."
  zrbv_validate_pat || bcu_die "PAT validation failed"
  zrbv_login_ghcr "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "GHCR login failed"

  local platform_tag="${ZRBV_VMIMAGE_TAG_PREFIX}${RBRR_CHOSEN_IDENTITY}-${RBRS_VM_PLATFORM}"

  bcu_step "Pulling platform-specific container: ${platform_tag}"
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" pull "${platform_tag}" \
    || bcu_die "Failed to pull platform container: ${platform_tag}"

  bcu_step "Creating temporary container from platform image..."
  local temp_container_id="${RBV_TEMP_DIR}/platform_container_id.txt"
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" create "${platform_tag}" > "${temp_container_id}" \
    || bcu_die "Failed to create temporary container"

  local container_id=$(<"${temp_container_id}")

  bcu_step "Setting up VM temporary dir..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --              \
      "rm -rf ${ZRBV_VM_TEMP_DIR} && mkdir -p ${ZRBV_VM_TEMP_DIR}" \
    || bcu_die "Failed to create VM temp directory"

  bcu_step "Extracting disk image from container..."
  local vm_temp_disk="${ZRBV_VM_TEMP_DIR}/extracted_disk_image.tar"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --             \
      "podman cp ${container_id}:/disk-image.tar ${vm_temp_disk}" \
    || bcu_die "Failed to extract disk image from container"

  local cache_file="${RBRS_VMIMAGE_CACHE_DIR}/${RBRS_VM_PLATFORM}-${RBRR_CHOSEN_IDENTITY}.tar"
  bcu_step "Copying disk image to cache directory as -> ${cache_file}"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "cat ${vm_temp_disk}" > "${cache_file}"         \
    || bcu_die "Failed to copy disk image to cache"

  bcu_step "Cleaning up temporary container..."
  podman -c "${RBRR_IGNITE_MACHINE_NAME}" rm "${container_id}" \
    || bcu_warn "Failed to remove temporary container"

  bcu_step "Removing temporary files in VM..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "rm -f ${vm_temp_disk}"                         \
    || bcu_warn "Failed to remove temporary disk file"

  bcu_success "VM image cached: ${cache_file}"
}

rbv_init() {
  # Handle documentation mode
  bcu_doc_brief "Initialize deploy VM from cached platform-specific image"
  bcu_doc_lines "Uses cached disk image from RBRS_VMIMAGE_CACHE_DIR"
  bcu_doc_lines "Initializes deploy VM with rootful mode"
  bcu_doc_lines "Writes brand file with all RBRR_CHOSEN values"
  bcu_doc_lines "Refuses if deploy VM already exists"
  bcu_doc_lines "Requires rbv_fetch to be run first"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Checking if deploy VM exists..."
  if podman machine list | grep -q "${RBRR_DEPLOY_MACHINE_NAME}"; then
    bcu_die "Deploy VM already exists. Remove it first with rbv_nuke or manually"
  fi

  bcu_step "Validating platform configuration..."
  test -n "${RBRS_VM_PLATFORM}" || bcu_die "RBRS_VM_PLATFORM not set in station config"
  echo "${RBRR_MANIFEST_PLATFORMS}" | grep -q "${RBRS_VM_PLATFORM}" \
    || bcu_die "Platform ${RBRS_VM_PLATFORM} not in manifest platforms: ${RBRR_MANIFEST_PLATFORMS}"

  local cache_file="${RBRS_VMIMAGE_CACHE_DIR}/${RBRS_VM_PLATFORM}-${RBRR_CHOSEN_IDENTITY}.tar"

  bcu_step "Checking for cached VM image..."
  test -f "${cache_file}" \
    || bcu_die "VM image not found in cache: ${cache_file}" \
               "Run 'rbv_fetch' first to download the VM image"

  bcu_step "Initializing machine from cached image..."
  podman machine init --rootful --image "${cache_file}" "${RBRR_DEPLOY_MACHINE_NAME}" \
                                          2> "${ZRBV_DEPLOY_INIT_STDERR}"             \
       | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "${ZRBV_DEPLOY_INIT_STDOUT}"             \
    || bcu_die "Failed to initialize VM"

  bcu_step "Starting VM to write brand file..."
  podman machine start "${RBRR_DEPLOY_MACHINE_NAME}" || bcu_die "Failed to start deploy VM"

  bcu_step "Generating brand file content..."
  local brand_file="${RBV_TEMP_DIR}/brand.txt"
  zrbv_generate_brand_file > "${brand_file}" \
    || bcu_die "Failed to generate brand file"

  bcu_step "Writing brand file to VM: ${ZRBV_EMPLACED_BRAND_FILE}"
  podman machine ssh "${RBRR_DEPLOY_MACHINE_NAME}" \
      "sudo tee ${ZRBV_EMPLACED_BRAND_FILE}" < "${brand_file}" > /dev/null \
    || bcu_die "Failed to write brand file to VM"

  bcu_step "Stopping VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}" \
    || bcu_die "Failed to stop deploy VM"

  bcu_success "Deploy VM initialized with brand file"
}

rbv_start() {
  bcu_die "BRADTODO: ELIDED."
}

rbv_stop() {
  bcu_die "BRADTODO: ELIDED."
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_environment "$@"


# eof

