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

  # Module Variables (ZRBV_*)
  ZRBV_GIT_REGISTRY="ghcr.io"

  ZRBV_VERSION_FILE="${RBV_TEMP_DIR}/podman_version.txt"

  ZRBV_GENERATED_BRAND_FILE="${RBV_TEMP_DIR}/brand_generated.txt"
  ZRBV_FOUND_BRAND_FILE="${RBV_TEMP_DIR}/brand_found.txt"
  ZRBV_INIT_OUTPUT_FILE="${RBV_TEMP_DIR}/podman_init_output.txt"

  ZRBV_IGNITE_INIT_STDOUT="${RBV_TEMP_DIR}/ignite_init_stdout.txt"
  ZRBV_IGNITE_INIT_STDERR="${RBV_TEMP_DIR}/ignite_init_stderr.txt"
  ZRBV_DEPLOY_INIT_STDOUT="${RBV_TEMP_DIR}/deploy_init_stdout.txt"
  ZRBV_DEPLOY_INIT_STDERR="${RBV_TEMP_DIR}/deploy_init_stderr.txt"

  ZRBV_PODMAN_REMOVE_PREFIX="${RBV_TEMP_DIR}/podman_inspect_remove_"
  ZRBV_IDENTITY_FILE="${RBV_TEMP_DIR}/identity_date.txt"
  ZRBV_NATURAL_TAG_FILE="${RBV_TEMP_DIR}/natural_tag.txt"
  ZRBV_MIRROR_TAG_FILE="${RBV_TEMP_DIR}/mirror_tag.txt"
  ZRBV_CRANE_ORIGIN_DIGEST_FILE="${RBV_TEMP_DIR}/crane_origin_digest.txt"
  ZRBV_CRANE_CHOSEN_DIGEST_FILE="${RBV_TEMP_DIR}/crane_chosen_digest.txt"
  ZRBV_CRANE_MIRROR_DIGEST_FILE="${RBV_TEMP_DIR}/crane_mirror_digest.txt"
  ZRBV_CRANE_MANIFEST_CHECK_FILE="${RBV_TEMP_DIR}/crane_manifest_check.txt"
  ZRBV_CRANE_COPY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_copy_output.txt"
  ZRBV_CRANE_VERIFY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_verify_output.txt"

  ZRBV_TEMPORARY_CONTAINER_FILE="${RBV_TEMP_DIR}/temporary_container_id.txt"

  ZRBV_EMPLACED_BRAND_FILE=/etc/brand-emplaced.txt

  ZRBV_TARBALL_FILENAME="vm-image.tar"
  ZRBV_BRAND_FILENAME="brand.txt"
  ZRBV_MACH_IMAGE_FILENAME="/tmp/${RBRR_CHOSEN_VMIMAGE_DIGEST}.tar"
  ZRBV_HOST_IMAGE_FILENAME="${RBRS_VMIMAGE_CACHE_DIR}/${RBRR_CHOSEN_VMIMAGE_DIGEST}.tar"

  ZRBV_VM_BUILD_DIR="vm-build"
}

zrbv_verify_podman_version() {
  podman --version > "${ZRBV_VERSION_FILE}" || bcu_die "Podman not in PATH"

  local                   host_podman_version
  read -r _ _             host_podman_version < "${ZRBV_VERSION_FILE}"
  local host_podman_mm="${host_podman_version%.*}"

  bcu_step "Podman version check: chosen is $RBRR_CHOSEN_PODMAN_VERSION, found is ${host_podman_mm}"

  if [[ "$host_podman_mm" != "$RBRR_CHOSEN_PODMAN_VERSION" ]]; then
    bcu_die "Podman version mismatch: host has ${host_podman_mm};" "  RBRR_CHOSEN_PODMAN_VERSION=${RBRR_CHOSEN_PODMAN_VERSION}"
  fi

  return 0
}

# Generate brand file content
zrbv_generate_brand_file() {
  echo "# Recipe Bottle VM Brand File"                   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "#"                                               >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "PODMAN_VERSION: ${RBRR_CHOSEN_PODMAN_VERSION}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_ORIGIN: ${RBRR_CHOSEN_VMIMAGE_ORIGIN}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_FQIN:   ${RBRR_CHOSEN_VMIMAGE_FQIN}"     >> "${ZRBV_GENERATED_BRAND_FILE}"
  echo "VMIMAGE_DIGEST: ${RBRR_CHOSEN_VMIMAGE_DIGEST}"   >> "${ZRBV_GENERATED_BRAND_FILE}"
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
  local     digest
  read -r   digest < "${ZRBV_CRANE_ORIGIN_DIGEST_FILE}" || bcu_die "Failed to read crane file"
  test -n "$digest"                                     || bcu_die "Failed to test crane file"

  local     sha_short="${digest:7:12}"  # Extract characters 8-19 (0-indexed)
  test -n "$sha_short" || bcu_die "Failed to extract short SHA from digest: $digest"

  local raw="mirror-${RBRR_CHOSEN_VMIMAGE_ORIGIN}-${RBRR_CHOSEN_PODMAN_VERSION}-${sha_short}"
  raw=${raw//\//-}
  raw=${raw//:/-}

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
  else
    bcu_step "Restarting ignite machine..."
    podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to restart ignite machine"
  fi

  bcu_success "Ignite machine ready with crane and skopeo installed"
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
  # Handle documentation mode
  bcu_doc_brief "Check to see if a new podman VM is available"
  bcu_doc_lines "TODO"
  bcu_doc_shown || return 0

  # Perform command
  local warning_count=0

  # There are many things I want this to do here but now is not the time.
  #
  # This has gotten sophisticated since I was bitten hard by the RedHat practice
  # of generating new podman VM images quite frequently, moving the tag each
  # time: I suffered a serious regression when a retag of their 5.2 VM image
  # stopped supporting critical features upon which I depended.  That is the
  # primary motivation for using VM images that I cache rather than trusting
  # theirs.  The charitable interpretation here is that they are sealing critical
  # security holes and I want them to be brilliant at that after all.
  #
  # On to my solution:
  #
  # For now, I'll support precisely my use case, where I want this function
  # to tell me if there is a new origin VM base image and how to configure to
  # capture a mirror of it if I want it.  I also want it to validate my current
  # mirror image by digest.  I'll always work out of my mirror, and I'll always
  # lock the sha.  My chosen FQIN will always be my latest mirror.  I'll delete
  # mirrors when I darn well choose.
  #
  # In the glorious future, I'd like this to support other configurations:
  #
  # * Users can set their chosen FQIN to the quay.io tagged version and leave
  #   their chosen digest unset: go with God and latest!  I wish them the best of
  #   luck with this.
  #
  # * Users can set their chosen FQIN to the quay.io tagged version but then also
  #   configure their chosen digest.  This and perhaps some deft chosen identity
  #   can give them more information when checking available VM images: how old
  #   is their image if it isn't latest?  They assume the risk and obligation to
  #   manage their own cache and exposure to VM evolution.

  bcu_step "Verify host Podman major.minor version..."
  zrbv_verify_podman_version || bcu_die "Podman version mismatch."

  bcu_step "Checking for newer VM images..."

  bcu_step "Prepare fresh ignite machine with crane and skopeo..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to create temp machine"

  local origin_fqin=$(printf '%q' "${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}")
  local chosen_fqin=$(printf '%q' "${RBRR_CHOSEN_VMIMAGE_FQIN}")

  bcu_step "Querying origin ${origin_fqin}..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                                 \
      "skopeo inspect docker://${origin_fqin} --raw | sha256sum | cut -d' ' -f1 | sed 's/^/sha256:/'" \
      > "${ZRBV_CRANE_ORIGIN_DIGEST_FILE}" || bcu_die "Failed to query origin image"

  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                                 \
      "skopeo inspect docker://${chosen_fqin} --raw | sha256sum | cut -d' ' -f1 | sed 's/^/sha256:/'" \
      > "${ZRBV_CRANE_CHOSEN_DIGEST_FILE}" 2>/dev/null || echo "NOT_FOUND" > "${ZRBV_CRANE_CHOSEN_DIGEST_FILE}"

  podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Failed to stop ignite during _check"

  local     chosen_digest="NOT_FOUND"
  if [[ -f                  "${ZRBV_CRANE_CHOSEN_DIGEST_FILE}" ]]; then
    read -r chosen_digest < "${ZRBV_CRANE_CHOSEN_DIGEST_FILE}" || bcu_warn "Failed chosen digest retrieve"
  fi

  bcu_step "Prepare ultra bash safe latest vars..."
  zrbv_generate_mirror_tag
  local   mirror_tag
  read -r mirror_tag < "${ZRBV_MIRROR_TAG_FILE}"

  local   origin_digest
  read -r origin_digest < "${ZRBV_CRANE_ORIGIN_DIGEST_FILE}"

  local   proposed_identity; date +'%Y%m%d-%H%M%S' > "${ZRBV_IDENTITY_FILE}"
  read -r proposed_identity                        < "${ZRBV_IDENTITY_FILE}"

  local advise_change=false
  local mirror_missing=false

  if [[ -z "${RBRR_CHOSEN_VMIMAGE_DIGEST}" ]]; then
    bcu_warn "RBRR_CHOSEN_VMIMAGE_DIGEST is not set!"
    ((warning_count++)) || true
    advise_change=true
  fi

  if [[ "${chosen_digest}" == "NOT_FOUND" ]]; then
    bcu_warn "Mirror image not found: ${RBRR_CHOSEN_VMIMAGE_FQIN}"
    ((warning_count++)) || true
    mirror_missing=true
  elif [[ "${origin_digest}" != "${chosen_digest}" ]]; then
    bcu_warn "Digest mismatch: origin (${origin_digest}) does not match chosen (${chosen_digest})"
    ((warning_count++)) || true
    advise_change=true
  fi

  if $mirror_missing; then
    bcu_warn "Mirror image needs to be created. Run: rbv_mirror"
  fi

  if $advise_change; then
    bcu_code "# New contents for -> ${RBV_RBRR_FILE}"
    bcu_code "#"
    bcu_code "export RBRR_CHOSEN_VMIMAGE_FQIN=${mirror_tag}"
    bcu_code "export RBRR_CHOSEN_VMIMAGE_DIGEST=${origin_digest}"
    bcu_code "export RBRR_CHOSEN_IDENTITY=${proposed_identity}  # update date"
  fi

  if [[ $warning_count -gt 0 ]]; then
    bcu_die "Found $warning_count warning(s)."
  fi

  bcu_success "No updates available."
}

# Mirror VM image to GHCR
rbv_mirror() {
  # Handle documentation mode
  bcu_doc_brief "Package VM image and brand file into container image, push to GHCR"
  bcu_doc_lines "TODO"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Prepare fresh ignite machine with crane and skopeo..."
  zrbv_ignite_bootstrap false || bcu_die "Failed to create temp machine"

  bcu_step "Creating build directory..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "rm -rf ${ZRBV_VM_BUILD_DIR} && mkdir -p ${ZRBV_VM_BUILD_DIR}" \
      || bcu_die "Failed to create build directory"

  bcu_step "Login to registry..."
  zrbv_login_ghcr  "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to login podman to registry"
  zrbv_login_crane "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to login crane to registry"

  local origin_fqin=$(printf '%q' "${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}")
  bcu_step "Querying origin ${origin_fqin}..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                                 \
      "skopeo inspect docker://${origin_fqin} --raw | sha256sum | cut -d' ' -f1 | sed 's/^/sha256:/'" \
      > "${ZRBV_CRANE_ORIGIN_DIGEST_FILE}" || bcu_die "Failed to query origin image"

  bcu_step "Prepare mirror tag..."
  zrbv_generate_mirror_tag
  local   mirror_tag
  read -r mirror_tag < "${ZRBV_MIRROR_TAG_FILE}"

  local   origin_digest
  read -r origin_digest < "${ZRBV_CRANE_ORIGIN_DIGEST_FILE}"

  # Validate configuration matches what we're about to mirror
  bcu_step "Validating configuration matches origin..."
  if [[ "${RBRR_CHOSEN_VMIMAGE_FQIN}" != "${mirror_tag}" ]]; then
    bcu_die "Configuration mismatch: RBRR_CHOSEN_VMIMAGE_FQIN=${RBRR_CHOSEN_VMIMAGE_FQIN}" "  Expected: ${mirror_tag}"
  fi

  if [[ "${RBRR_CHOSEN_VMIMAGE_DIGEST}" != "${origin_digest}" ]]; then
    bcu_die "Configuration mismatch: RBRR_CHOSEN_VMIMAGE_DIGEST=${RBRR_CHOSEN_VMIMAGE_DIGEST}" "  Expected: ${origin_digest}"
  fi

  bcu_step "Pulling VM image to OCI archive in build directory..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "skopeo copy --all docker://${origin_fqin} oci-archive:${ZRBV_VM_BUILD_DIR}/oci-archive" \
      || bcu_die "Failed to pull VM image to OCI archive"

  bcu_step "Extracting VM image from OCI archive..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "cd ${ZRBV_VM_BUILD_DIR} && skopeo copy oci-archive:oci-archive docker-archive:${ZRBV_TARBALL_FILENAME}" \
      || bcu_die "Failed to extract VM image from OCI archive"

  bcu_step "Generating brand file..."
  zrbv_generate_brand_file
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --     \
      "cat > ${ZRBV_VM_BUILD_DIR}/${ZRBV_BRAND_FILENAME}" \
                     < "${ZRBV_GENERATED_BRAND_FILE}"

  bcu_step "Creating container image with wildcard ADD..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                       \
      "printf 'FROM scratch\nADD * /\n' > ${ZRBV_VM_BUILD_DIR}/Dockerfile"

  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                            \
      "cd ${ZRBV_VM_BUILD_DIR} && podman build -t ${mirror_tag} -f Dockerfile ." \
      || bcu_die "Failed to build container image"

  bcu_step "Pushing container image to GHCR..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "podman push ${mirror_tag}"                     \
      || bcu_die "Failed to push container image"

  podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Failed to stop ignite during _mirror"

  bcu_success "Successfully packaged and pushed image to ${mirror_tag}"
}

# Fetch VM image from GHCR container
rbv_fetch() {
  # Handle documentation mode
  bcu_doc_brief "Fetch VM image from GHCR container package"
  bcu_doc_lines "TODO"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Ensure ignite VM is running..."
  zrbv_ignite_bootstrap false

  bcu_step "Login to registry..."
  zrbv_login_ghcr "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to login podman to registry"

  bcu_step "Generating expected brand file..."
  zrbv_generate_brand_file

  bcu_step "Pulling container image from GHCR..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "podman pull ${RBRR_CHOSEN_VMIMAGE_FQIN}"       \
      || bcu_die "Failed to pull container image"

  bcu_step "Creating temporary container from image..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                  \
      "podman create ${RBRR_CHOSEN_VMIMAGE_FQIN}" > "${ZRBV_TEMPORARY_CONTAINER_FILE}" \
      || bcu_die "Failed to create container from image"

  local   container_id
  read -r container_id < "${ZRBV_TEMPORARY_CONTAINER_FILE}" || bcu_die "Failed read ID"

  # DEBUG: List container contents
  bcu_step "DEBUG: Container contents:"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- "podman cp ${container_id}:/ - | tar -t"

  bcu_step "Extracting VM tarball from container..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                   \
      "podman cp ${container_id}:/${ZRBV_TARBALL_FILENAME} ${ZRBV_MACH_IMAGE_FILENAME}" \
      || bcu_die "Failed to extract VM tarball"

  bcu_step "Extracting brand file to host..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" --                                    \
      "podman cp ${container_id}:/${ZRBV_BRAND_FILENAME} -" > "${ZRBV_FOUND_BRAND_FILE}" \
      || bcu_die "Failed to extract brand file"

  bcu_step "Removing temporary container..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "podman rm -f ${container_id}"                  \
      || bcu_warn "Failed to remove temp container"

  bcu_step "Comparing brand files..."

  zrbv_error_if_different "${ZRBV_GENERATED_BRAND_FILE}" "${ZRBV_FOUND_BRAND_FILE}" || \
    bcu_die "Brand file mismatch - container package doesn't match current RBRR settings"

  bcu_step "Computing VM-side checksum..."
  local    vm_checksum
  read -r  vm_checksum _ < <(podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- sha256sum "${ZRBV_MACH_IMAGE_FILENAME}") || bcu_die "vm checksum"
  test "${#vm_checksum}" -eq 64 || bcu_die "Invalid VM checksum length: ${#vm_checksum}"

  if [ -f "${ZRBV_HOST_IMAGE_FILENAME}" ]; then
    bcu_step "Host cache already exists - verifying checksum..."
    local    host_checksum
    read -r  host_checksum _ < <(sha256sum "${ZRBV_HOST_IMAGE_FILENAME}") || bcu_die "host checksum"
    test "${#host_checksum}" -eq 64 || bcu_die "Invalid host checksum length: ${#host_checksum}"

    if [ "$vm_checksum" = "$host_checksum" ]; then
      bcu_info "Host cache valid for ${RBRR_CHOSEN_VMIMAGE_DIGEST}"
      podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- rm -f "${ZRBV_MACH_IMAGE_FILENAME}"
      podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Failed to stop ignite during _fetch"
      return 0
    fi

    bcu_die "Checksum mismatch for cached image:"          \
            "  VM:   $vm_checksum"                         \
            "  Host: $host_checksum"                       \
            "To fix: rm \"${ZRBV_HOST_IMAGE_FILENAME}\""
  fi

  bcu_step "Copying image from VM to host cache..."
  mkdir -p "${RBRS_VMIMAGE_CACHE_DIR}" || bcu_die "Failed to assure cache directory"
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- cat "${ZRBV_MACH_IMAGE_FILENAME}" > "${ZRBV_HOST_IMAGE_FILENAME}" || \
    bcu_die "Failed to copy image from VM to host"

  bcu_step "Verifying host-side checksum..."
  local    verify_checksum
  read -r  verify_checksum _ < <(sha256sum "${ZRBV_HOST_IMAGE_FILENAME}") || bcu_die "final checksum"
  test "${#verify_checksum}" -eq 64 || bcu_die "Invalid verification checksum length: ${#verify_checksum}"

  test "${vm_checksum}" = "${verify_checksum}" || bcu_die "Copy verification failed"

  bcu_success "Cached ${RBRR_CHOSEN_VMIMAGE_DIGEST} at ${ZRBV_HOST_IMAGE_FILENAME}"

  bcu_step "Cleaning up VM temporary file..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- rm -f "${ZRBV_MACH_IMAGE_FILENAME}" || true

  podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Failed to stop ignite during _fetch"
}

rbv_init() {
  # Handle documentation mode
  bcu_doc_brief "Initialize deploy VM using local cached VM image"
  bcu_doc_lines "Creates new deploy VM from cached image tarball"
  bcu_doc_lines "Writes brand file with all RBRR_CHOSEN values"
  bcu_doc_lines "Refuses if deploy VM already exists"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Checking if deploy VM exists..."
  podman machine list | grep -q "${RBRR_DEPLOY_MACHINE_NAME}" && \
    bcu_die "Deploy VM already exists. Remove it first with rbv_nuke or manually"

  bcu_step "Verifying cached image exists..."
  test -f "${ZRBV_HOST_IMAGE_FILENAME}" || bcu_die "Cached VM image not found at ${ZRBV_HOST_IMAGE_FILENAME}. Run rbv_fetch first."

  bcu_step "Initializing: init machine from cached image ${ZRBV_HOST_IMAGE_FILENAME}..."
  podman machine init --rootful --image "${ZRBV_HOST_IMAGE_FILENAME}" \
                                            "${RBRR_DEPLOY_MACHINE_NAME}"      \
                                          2> "$ZRBV_DEPLOY_INIT_STDERR"        \
       | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "$ZRBV_DEPLOY_INIT_STDOUT"        \
    || bcu_die "Bad init."

  bcu_step "Starting VM temporarily..."
  podman machine start "${RBRR_DEPLOY_MACHINE_NAME}"

  bcu_step "Writing brand file to -> ${ZRBV_EMPLACED_BRAND_FILE}"
  zrbv_generate_brand_file
  podman machine ssh "${RBRR_DEPLOY_MACHINE_NAME}" "sudo tee ${ZRBV_EMPLACED_BRAND_FILE}" \
                                                         < "${ZRBV_GENERATED_BRAND_FILE}"

  bcu_step "Stopping VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}"

  bcu_success "VM initialized with brand file"
}

rbv_start() {
  # Handle documentation mode
  bcu_doc_brief "Start deploy podman VM"
  bcu_doc_lines "Verifies brand file matches RBRR_CHOSEN values"
  bcu_doc_lines "Fails if brand file missing or mismatched"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Starting deploy VM..."
  podman machine start "${RBRR_DEPLOY_MACHINE_NAME}"

  bcu_step "Reading brand file from -> ${ZRBV_EMPLACED_BRAND_FILE}"
  podman machine ssh "${RBRR_DEPLOY_MACHINE_NAME}" "sudo cat ${ZRBV_EMPLACED_BRAND_FILE}" \
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
  bcu_doc_brief "Stop deploy podman machine"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Stopping deploy VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}"

  bcu_success "VM stopped"
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_validate_envvars "$@"


# eof

