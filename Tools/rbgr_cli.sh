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
source "${ZRBGR_CLI_SCRIPT_DIR}/rbcr_ContainerRegistry.sh"
source "${ZRBGR_CLI_SCRIPT_DIR}/rbgr_GithubRemote.sh"

# CLI-specific environment function
zrbgr_furnish() {
  # Handle documentation mode
  bcu_doc_env "RBRR_BUILD_ARCHITECTURES  " "Platform list for multi-arch builds (e.g., linux/amd64,linux/arm64)"
  bcu_doc_env "RBRR_HISTORY_DIR         " "Directory for build history artifacts"
  bcu_doc_env "RBRR_REGISTRY_OWNER      " "GitHub registry owner/organization"
  bcu_doc_env "RBRR_REGISTRY_NAME       " "GitHub registry repository name"
  bcu_doc_env "RBRR_REGISTRY            " "Registry type (e.g., ghcr)"
  bcu_doc_env "RBRR_GITHUB_PAT_ENV      " "Path to file containing GitHub PAT credentials"
  bcu_doc_env "RBG_RUNTIME              " "Container runtime (docker/podman)"
  bcu_doc_env "RBG_TEMP_DIR             " "Empty temporary directory"
  bcu_doc_env "GITHUB_TOKEN             " "GitHub token for API access"
  bcu_doc_env "GITHUB_SHA               " "Git commit SHA (optional, defaults to 'unknown')"
  
  bcu_env_done || return 0
  
  # Validate environment
  bvu_env_string "RBRR_BUILD_ARCHITECTURES" 1 512
  bvu_dir_exists "${RBRR_HISTORY_DIR}"
  bvu_env_string "RBRR_REGISTRY_OWNER" 1 128
  bvu_env_string "RBRR_REGISTRY_NAME" 1 128
  bvu_env_string "RBRR_REGISTRY" 1 32
  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
  bvu_env_string "RBG_RUNTIME" 1 20 "docker"
  bvu_dir_exists "${RBG_TEMP_DIR}"
  bvu_dir_empty "${RBG_TEMP_DIR}"
  bvu_env_string "GITHUB_TOKEN" 1 512
  
  # Optional environment
  GITHUB_SHA="${GITHUB_SHA:-unknown}"
  
  # Start dependent modules
  zrbcr_kindle
  
  # Start implementation module
  zrbgr_kindle
}

# Execute command
bcu_execute rbgr_ "Recipe Bottle GitHub Runner - Remote runner steps" zrbgr_furnish "$@"

# eof

