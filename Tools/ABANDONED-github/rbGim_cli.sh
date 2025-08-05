#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Image Management - Command Line Interface

set -euo pipefail

ZRBIM_CLI_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

# Source all dependencies (CLI files handle all sourcing)
source "${ZRBIM_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbim_ImageManagement.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbcr_ContainerRegistry.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbga_GithubActions.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbgh_GithubHost.sh"

# CLI-specific environment function
zrbim_furnish() {
  # Handle documentation mode
  bcu_doc_env "RBG_TEMP_DIR    " "Empty temporary directory"
  bcu_doc_env "RBG_NOW_STAMP   " "Timestamp for per run branding"
  bcu_doc_env "RBG_RUNTIME     " "Container runtime (docker/podman)"
  bcu_doc_env "RBG_RUNTIME_ARG " "Container runtime arguments (optional)"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBG_TEMP_DIR}"
  bvu_dir_empty   "${RBG_TEMP_DIR}"
  bvu_env_string  RBG_NOW_STAMP  1 128
  bvu_env_string  RBG_RUNTIME    1 20 "podman"

  # Optional runtime args
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  # Get regime file location
  zrbl_kindle

  # Validate and source regime file
  test -f      "${RBL_RBRR_FILE}" \
    || bcu_die "${RBL_RBRR_FILE} Regime file not found"
  source       "${RBL_RBRR_FILE}"
  source "${ZRBIM_CLI_SCRIPT_DIR}/rbrr.validator.sh"

  # Start all modules
  zrbcr_kindle
  zrbga_kindle
  zrbgh_kindle
  zrbim_kindle
}

# Execute command
bcu_execute rbim_ "Recipe Bottle Image Management" zrbim_furnish "$@"

# eof

