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

ZRBV_VERSION_FILE="${RBV_TEMP_DIR}/podman_version.txt"

ZRBV_GENERATED_BRAND_FILE="${RBV_TEMP_DIR}/brand_generated.txt"
ZRBV_FOUND_BRAND_FILE="${RBV_TEMP_DIR}/brand_found.txt"
ZRBV_INIT_OUTPUT_FILE="${RBV_TEMP_DIR}/podman_init_output.txt"

ZRBV_IGNITE_INIT_STDOUT="${RBV_TEMP_DIR}/ignite_init_stdout.txt"
ZRBV_IGNITE_INIT_STDERR="${RBV_TEMP_DIR}/ignite_init_stderr.txt"
ZRBV_OPERATIONAL_INIT_STDOUT="${RBV_TEMP_DIR}/operational_init_stdout.txt"
ZRBV_OPERATIONAL_INIT_STDERR="${RBV_TEMP_DIR}/operational_init_stderr.txt"

ZRBV_NATURAL_TAG_FILE="${RBV_TEMP_DIR}/natural_tag.txt"
ZRBV_MIRROR_TAG_FILE="${RBV_TEMP_DIR}/mirror_tag.txt"
ZRBV_CRANE_ORIGIN_DIGEST_FILE="${RBV_TEMP_DIR}/crane_origin_digest.txt"
ZRBV_CRANE_FQIN_DIGEST_FILE="${RBV_TEMP_DIR}/crane_fqin_digest.txt"
ZRBV_CRANE_MANIFEST_CHECK_FILE="${RBV_TEMP_DIR}/crane_manifest_check.txt"
ZRBV_CRANE_COPY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_copy_output.txt"
ZRBV_CRANE_VERIFY_OUTPUT_FILE="${RBV_TEMP_DIR}/crane_verify_output.txt"

ZRBV_EMPLACED_BRAND_FILE=/etc/brand-emplaced.txt



######################################################################
# Internal Functions (zrbv_*)

function zrbv_verify_podman_version() {
  podman --version 2>/dev/null > "${ZRBV_VERSION_FILE}" || bcu_die "Podman not in PATH"
  
  local host_podman_version
  read -r _ _ host_podman_version < "${ZRBV_VERSION_FILE}"

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

  if podman machine inspect "$vm_name" &>/dev/null; then
    bcu_info       "Stopping $vm_name..."
    podman machine stop     "$vm_name" || bcu_warn "Failed to stop $vm_name"
    bcu_info       "Removing $vm_name..."
    podman machine rm -f    "$vm_name" || bcu_die "Failed to remove $vm_name"
  else
    bcu_info             "VM $vm_name does not exist. Nothing to remove."
  fi
}

# Create a fresh ignite machine with crane installed
function rbv_ignite_create() {
  bcu_info "Creating ignite machine: ${RBRR_IGNITE_MACHINE_NAME}"

  bcu_step "Stop existing ignite machine if running"
  podman machine stop "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Attempt to stop existing did nothing."

  bcu_step "Removing any existing ignite machine..."
  podman machine rm -f "${RBRR_IGNITE_MACHINE_NAME}" || bcu_warn "Attempt to rm existing did nothing."

  bcu_step "Creating ignite VM with natural podman init..."
  podman machine init --log-level=debug     "${RBRR_IGNITE_MACHINE_NAME}" \
                                          2> "$ZRBV_IGNITE_INIT_STDERR"   \
       | ${ZRBV_SCRIPT_DIR}/rbupmis_Scrub.sh "$ZRBV_IGNITE_INIT_STDOUT"   \
    || bcu_die "Bad init."

  bcu_step "Starting ignite machine..."
  podman machine start "${RBRR_IGNITE_MACHINE_NAME}" || bcu_die "Failed to start ignite machine"

  bcu_step "Installing crane..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
      "curl -sL ${RBRR_CRANE_TAR_GZ} | sudo tar -xz -C /usr/local/bin crane" || bcu_die "crane fail"

  bcu_step "Verify crane installation..."
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- "crane version" || bcu_die "crane confirm fail."

  bcu_success "Ignite machine ready with crane installed"
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

function rbv_check() {
  local warning_count=0

  # There are many things I want this to do but now is not the time for this.
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
  #   their chosen SHA unset: go with God and latest!  I wish them the best of
  #   luck with this.
  #
  # * Users can set their chosen FQIN to the quay.io tagged version but then also
  #   configure their chosen SHA.  This and perhaps some deft chosen identity 
  #   can give them more information when checking available VM images: how old
  #   is their image if it isn't latest?  They assume the risk and obligation to
  #   manage their own cache and exposure to VM evolution.

  bcu_step "Verify host Podman major.minor version..." #
  zrbv_verify_podman_version || bcu_die "Podman version mismatch."

  bcu_step "Checking for newer VM images..." #

  ### bcu_step "Prepare fresh ignite machine with crane..."
  ### rbv_ignite_create || bcu_die "Failed to create temp machine"

  bcu_step "Querying origin ${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}..." #
  podman machine ssh "${RBRR_IGNITE_MACHINE_NAME}" -- \
    crane digest "${RBRR_CHOSEN_VMIMAGE_ORIGIN}:${RBRR_CHOSEN_PODMAN_VERSION}" \
      > ${ZRBV_CRANE_ORIGIN_DIGEST_FILE} || bcu_die "Failed to query origin image"

  bcu_step "Prepare mirror tag..." #
  zrbv_generate_mirror_tag
  local   mirror_tag
  read -r mirror_tag < "${ZRBV_MIRROR_TAG_FILE}"

  if [[ -z "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
    bcu_warn "RBRR_CHOSEN_VMIMAGE_SHA is not set!"
    ((warning_count++)) || true
    bcu_code "export RBRR_CHOSEN_VMIMAGE_SHA=$(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})"
    bcu_code "export RBRR_CHOSEN_VMIMAGE_FQIN=${mirror_tag}"
    bcu_warn "Did we get here?"
  fi

  bcu_warn "STOPPING HERE TO FOCUS ON BASE CASES."
  bcu_die "Should not get here."

  bcu_step "If chosen FQIN differs from standard origin:version, do more checks..."
  if [[ ! "${RBRR_CHOSEN_VMIMAGE_FQIN}" =~ ^"${RBRR_CHOSEN_VMIMAGE_ORIGIN}:" ]]; then
    bcu_info "Checking custom FQIN: ${RBRR_CHOSEN_VMIMAGE_FQIN}..."
    podman machine ssh "${zrbv_temp_machine}" -- \
      crane digest "${RBRR_CHOSEN_VMIMAGE_FQIN}" \
        > ${ZRBV_CRANE_FQIN_DIGEST_FILE} || bcu_die "Failed to query fqin image"

    if [[ "$(cat ${ZRBV_CRANE_FQIN_DIGEST_FILE})" != "$(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})" ]]; then
      bcu_warn "FQIN SHA ($(cat ${ZRBV_CRANE_FQIN_DIGEST_FILE})) differs from upstream SHA ($(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE}))"
      ((warning_count++)) || true
    fi
  fi

  bcu_info "Current configured SHA: ${RBRR_CHOSEN_VMIMAGE_SHA:-<not set>}"
  bcu_info "Latest upstream SHA: $(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})"

  if [[ -z "${RBRR_CHOSEN_VMIMAGE_SHA}" ]]; then
    bcu_warn "ACTION REQUIRED: Set RBRR_CHOSEN_VMIMAGE_SHA to: $(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})"
    ((warning_count++)) || true
  elif [[ "${RBRR_CHOSEN_VMIMAGE_SHA}" != "$(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})" ]]; then
    bcu_info "UPDATE AVAILABLE: New SHA available: $(cat ${ZRBV_CRANE_ORIGIN_DIGEST_FILE})"
  else
    bcu_info "Up to date!"
  fi

  if [[ $warning_count -gt 0 ]]; then
    bcu_die "Found $warning_count warning(s) during VM image check. Please address the issues above."
  fi

  bcu_die "MUST RESTORE COMMENTED OUT DOWNLOADS"
}

# rbv_mirror - Mirror VM image to GHCR
function rbv_mirror() {
  local zrbv_ghcr_tag
  local zrbv_ghcr_sha
  local zrbv_origin_sha
  local zrbv_init_output
  local zrbv_init_sha
  local zrbv_temp_file="${RBV_TEMP_DIR}/rbv_mirror_output.txt"

  bcu_step "Verify host Podman major.minor version..."
  zrbv_verify_podman_version || bcu_die "Podman version mismatch."


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
  podman machine list | grep -q "${RBRR_DEPLOY_MACHINE_NAME}" && \
    bcu_die "Operational VM already exists. Remove it first with rbv_nuke or manually"

  bcu_step "Validating RBRR_CHOSEN_VMIMAGE_FQIN format..."
  echo "$RBRR_CHOSEN_VMIMAGE_FQIN" | grep -q ":" || \
    bcu_die "Invalid FQIN format: $RBRR_CHOSEN_VMIMAGE_FQIN"

  bcu_step "Initializing: podman machine init --image docker://${RBRR_CHOSEN_VMIMAGE_FQIN}..."
  podman machine init --rootful --image "docker://${RBRR_CHOSEN_VMIMAGE_FQIN}" "${RBRR_DEPLOY_MACHINE_NAME}"

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
  bcu_doc_brief "Start operational podman machine"
  bcu_doc_lines "Verifies brand file matches RBRR_CHOSEN values"
  bcu_doc_lines "Fails if brand file missing or mismatched"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Starting operational VM..."
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
  bcu_doc_brief "Stop operational podman machine"
  bcu_doc_shown || return 0

  # Perform command
  bcu_step "Stopping operational VM..."
  podman machine stop "${RBRR_DEPLOY_MACHINE_NAME}"

  bcu_success "VM stopped"
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_validate_envvars "$@"


# eof

