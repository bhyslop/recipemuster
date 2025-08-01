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
  ZRBIM_STARTED=1
}

zrbim_sentinel() {
  test "${ZRBIM_STARTED:-}" = "1" || bcu_die "Module rbim not started - call zrbim_kindle first"
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

rbim_delete() {
  # Name parameters
  local z_fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from registry and clean orphans"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Validate parameters
  test -n "${z_fqin}" || bcu_usage_die
  bvu_val_fqin "fqin" "${z_fqin}" 1 512

  # Perform command
  bcu_step "Delete image from registry"
  
  rbgh_check_git_status
  
  # Confirm deletion unless skipped
  if test "${RBG_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP"; then
    bcu_warn "BE AWARE THAT GHCR DELETIONS CAN DAMAGE OTHER IMAGES."
    bcu_require "Confirm delete image ${z_fqin}?" "YES"
  fi
  
  rbgh_delete_workflow "${z_fqin}"
  
  bcu_success "No errors."
}

rbim_retrieve() {
  # Name parameters
  local z_fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from registry"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Validate parameters
  test -n "${z_fqin}" || bcu_usage_die
  bvu_val_fqin "fqin" "${z_fqin}" 1 512

  # Perform command
  bcu_step "Pull image from registry"
  
  rbcr_pull "${z_fqin}"
  
  bcu_success "No errors."
}

rbim_image_info() {
  # Name parameters
  local z_filter="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Extracts per-image and per-layer info from registry tags"
  bcu_doc_lines \
    "Creates image detail entries for each tag/platform combination, extracts creation date," \
    "layers, and layer sizes. Handles both single and multi-platform images."
  bcu_doc_oparm "filter" "only process tags containing this string, if provided"
  bcu_doc_shown || return 0

  # Ensure module started
  zrbim_sentinel

  # Perform command
  bcu_step "Analyzing image information"
  
  # Get all tags
  rbcr_list_tags
  
  # Initialize detail file
  echo "[]" > "${ZRBCR_IMAGE_DETAIL_FILE}"
  
  local z_tag
  for z_tag in $(jq -r '.[].tag' "${ZRBCR_IMAGE_RECORDS_FILE}" | sort -u); do
    
    if test -n "${z_filter}" && [[ "${z_tag}" != *"${z_filter}"* ]]; then
      bcu_info "Skipping tag: ${z_tag}"
      continue
    fi
    
    bcu_info "Processing tag: ${z_tag}"
    rbcr_get_manifest "${z_tag}"
    
  done
  
  # Generate statistics
  bcu_step "Generating layer statistics"
  
  jq '
    .[]
    | {tag} as $t
    | .layers[]
    | {digest, size, tag: $t.tag}
  ' "${ZRBCR_IMAGE_DETAIL_FILE}" |
  jq -s '
    group_by([.digest, .tag])
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag: .[0].tag,
        count: length
      })
    | group_by(.digest)
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag_count: length,
        total_usage: (map(.count) | add),
        tag_details: map({tag: .tag, count: .count})
      })
    | sort_by(-.size)
  ' > "${ZRBCR_IMAGE_STATS_FILE}" || bcu_die "Failed to generate statistics"
  
  # Display results
  bcu_step "Listing layers per tag..."
  jq -r '
    .[] |
    "\nTag: \(.tag)" +
    if .platform then " (\(.platform))" else "" end +
    "\nCreated: \(.config.created)" +
    "\nTotal size: \((.layers | map(.size) | add) // 0) bytes" +
    "\nLayers:" +
    (.layers | to_entries | map(
      "\n  [\(.key + 1)] \(.value.digest[0:19])... \(.value.size) bytes"
    ) | join(""))
  ' "${ZRBCR_IMAGE_DETAIL_FILE}"
  
  bcu_success "No errors."
}

# eof

