#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Image Management - Top-level image operations

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBIM_INCLUDED:-}" || bcu_die "Module rbim multiply included - check sourcing hierarchy"
ZRBIM_INCLUDED=1

######################################################################
# Internal Functions (zrbim_*)

zrbim_kindle() {
  # Module variables
  ZRBIM_KINDLED=1
}

zrbim_sentinel() {
  test "${ZRBIM_KINDLED:-}" = "1" || bcu_die "Module rbim not kindled - call zrbim_kindle first"
}

######################################################################
# External Functions (rbim_*)

rbim_build() {
  # Name parameters
  local z_recipe_file="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Build image from recipe"
  bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Validate parameters
  test -n "${z_recipe_file}" || bcu_usage_die
  test -f "${z_recipe_file}" || bcu_die "Recipe file not found: ${z_recipe_file}"

  # Validate recipe basename
  local z_recipe_basename
  z_recipe_basename=$(basename "${z_recipe_file}")
  echo "${z_recipe_basename}" | grep -q '[A-Z]' && \
    bcu_die "Basename of '${z_recipe_file}' contains uppercase letters so cannot use in image name"

  # Perform command
  bcu_step "Build image from ${z_recipe_file}"

  rbgh_check_git_status
  rbgh_build_workflow "${z_recipe_file}"

  bcu_success "No errors."
}

rbim_list() {
  # Handle documentation mode
  bcu_doc_brief "List registry images"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Perform command
  bcu_step "List Current Registry Images"

  # Get all image records
  rbcr_list_tags

  # Display results
  local z_records_file="${ZRBCR_IMAGE_RECORDS_FILE}"

  echo "Package: ${RBRR_REGISTRY_NAME}"
  echo -e "${ZBCU_YELLOW}    https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/pkgs/container/${RBRR_REGISTRY_NAME}${ZBCU_RESET}"
  echo "Versions:"

  printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"

  jq -r '.[] | [.version_id, .fqin] | @tsv' "${z_records_file}" | \
    sort -k2 -r | while IFS=$'\t' read -r id fqin; do
    printf "%-13s %s\n" "$id" "${fqin}"
  done

  echo "${ZBCU_RESET}"

  local z_total
  z_total=$(jq '. | length' "${z_records_file}")
  bcu_info "Total image versions: ${z_total}"

  bcu_success "No errors."
}


rbim_retrieve() {
  # Name parameters
  local z_tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from registry"
  bcu_doc_param "tag" "Image tag"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_usage_die
  bvu_val_string "tag" "${z_tag}" 1 256

  # Perform command
  bcu_step "Pull image from registry"

  rbcr_pull "${z_tag}"

  bcu_success "No errors."
}

rbim_image_info() {
  bcu_die "ELIDED DURING BUILD REDO."
}

# eof

