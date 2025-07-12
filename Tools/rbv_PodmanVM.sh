#!/bin/bash

rbv_check() {
    # Handle documentation mode
    bcu_doc_brief "Compare current stash against podman's natural choice"
    bcu_doc_lines "Determines what VM version podman would naturally select"
    bcu_doc_lines "Compares against user's current stash (if configured)"
    bcu_doc_lines "Reports if an update is available"
    bcu_doc_shown || return 0

    # Perform command
    bcu_step "Creating temporary podman machine to discover natural choice..."
    bcu_step "Parsing 'Looking up' line for tag..."
    bcu_step "Getting digest with: crane digest quay.io/podman/machine-os-wsl:TAG"
    bcu_step "Checking if RBRR_VMDIST_FQIN is set..."
    bcu_step "If stash configured, extracting digest from stash name..."
    bcu_step "Comparing natural digest vs stash digest..."
    bcu_step "Reporting: [CURRENT|UPDATE_AVAILABLE|NO_STASH]"
}

rbv_stash() {
    # Handle documentation mode
    bcu_doc_brief "Create/update stash from podman's natural choice"
    bcu_doc_lines "Discovers what VM podman would naturally select"
    bcu_doc_lines "Copies that exact version to user's GHCR with canonical name"
    bcu_doc_lines "Updates RBRR_VMDIST_FQIN to point to new stash"
    bcu_doc_shown || return 0

    # Perform command
    bcu_step "Assuring all previous podman VMs deleted..."
    bcu_step "Creating temporary podman machine to discover natural choice..."
    bcu_step "Parsing 'Looking up' line for tag..."
    bcu_step "Getting digest with: crane digest quay.io/podman/machine-os-wsl:TAG"
    bcu_step "Generating canonical name: stash-quay.io-podman-machine-os-wsl-TAG-SHORTDIGEST"
    bcu_step "Checking if already exists in GHCR..."
    bcu_step "Copying with: crane copy SOURCE DEST"
    bcu_step "Verifying copy succeeded..."
    bcu_step "Updating RBRR_VMDIST_FQIN in config..."
}

rbv_init() {
    # Name parameters
    local machine_name="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Initialize podman machine using RBRR_VMDIST_FQIN"
    bcu_doc_lines "Creates new podman machine with the configured VM image"
    bcu_doc_lines "Supports direct quay references or GHCR stash references"
    bcu_doc_lines "Fails if RBRR_VMDIST_FQIN not set"
    bcu_doc_param "machine_name" "Name for the podman machine"
    bcu_doc_shown || return 0

    # Argument validation
    TODO

    # Perform command
    bcu_step "Validating RBRR_VMDIST_FQIN is set..."
    bcu_step "Checking if machine already exists..."
    bcu_step "Initializing with: podman machine init --image docker://FQIN MACHINE_NAME"
    bcu_step "Verifying successful init..."
    bcu_step "Extracting build date from VM..."
}

# eof
