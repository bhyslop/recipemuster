#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Runner - Command Line Interface

set -euo pipefail

ZRBHR_CLI_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

# Source all dependencies (CLI files handle all sourcing)
source "${ZRBHR_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZRBHR_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${ZRBHR_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBHR_CLI_SCRIPT_DIR}/rbcr_ContainerRegistry.sh"
source "${ZRBHR_CLI_SCRIPT_DIR}/rbhr_GithubRemote.sh"

# CLI-specific environment function
zrbhr_furnish() {
  # Handle documentation mode
  buc_doc_env "BURD_TEMP_DIR   " "Empty temporary directory"
  buc_doc_env "BURD_NOW_STAMP  " "Timestamp for per run branding"
  buc_doc_env "GITHUB_TOKEN   " "GitHub token for API access"
  buc_doc_env "GITHUB_SHA     " "Git commit SHA (optional, defaults to 'unknown')"

  # Get regime file location
  zrbl_kindle

  buv_dir_exists  "${BURD_TEMP_DIR}"
  buv_env_string     BURD_NOW_STAMP  1 128

  # Validate and source regime file
  test -f "${RBL_RBRR_FILE}" || buc_die "Regime file not found: ${RBL_RBRR_FILE}"
  source  "${RBL_RBRR_FILE}"
  source "${ZRBHR_CLI_SCRIPT_DIR}/rbrr.validator.sh"

  # Validate GitHub environment
  buv_env_string "GITHUB_TOKEN" 1 512

  # Optional environment
  GITHUB_SHA="${GITHUB_SHA:-unknown}"

  # Start dependent modules
  zrbcr_kindle

  # Start implementation module
  zrbhr_kindle
}

# Execute command
buc_execute rbhr_ "Recipe Bottle GitHub Remote - Remote runner steps" zrbhr_furnish "$@"

# eof


