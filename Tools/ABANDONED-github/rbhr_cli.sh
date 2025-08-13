#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle GitHub Runner - Command Line Interface

set -euo pipefail

ZRBGR_CLI_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

# Source all dependencies (CLI files handle all sourcing)
source "${ZRBGR_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBGR_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBGR_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBGR_CLI_SCRIPT_DIR}/rbcr_ContainerRegistry.sh"
source "${ZRBGR_CLI_SCRIPT_DIR}/rbgr_GithubRemote.sh"

# CLI-specific environment function
zrbgr_furnish() {
  # Handle documentation mode
  bcu_doc_env "BDU_TEMP_DIR   " "Empty temporary directory"
  bcu_doc_env "BDU_NOW_STAMP  " "Timestamp for per run branding"
  bcu_doc_env "GITHUB_TOKEN   " "GitHub token for API access"
  bcu_doc_env "GITHUB_SHA     " "Git commit SHA (optional, defaults to 'unknown')"

  # Get regime file location
  zrbl_kindle

  bvu_dir_exists  "${BDU_TEMP_DIR}"
  bvu_env_string     BDU_NOW_STAMP  1 128

  # Validate and source regime file
  test -f "${RBL_RBRR_FILE}" || bcu_die "Regime file not found: ${RBL_RBRR_FILE}"
  source  "${RBL_RBRR_FILE}"
  source "${ZRBGR_CLI_SCRIPT_DIR}/rbrr.validator.sh"

  # Validate GitHub environment
  bvu_env_string "GITHUB_TOKEN" 1 512

  # Optional environment
  GITHUB_SHA="${GITHUB_SHA:-unknown}"

  # Start dependent modules
  zrbcr_kindle

  # Start implementation module
  zrbgr_kindle
}

# Execute command
bcu_execute rbgr_ "Recipe Bottle GitHub Remote - Remote runner steps" zrbgr_furnish "$@"

# eof


