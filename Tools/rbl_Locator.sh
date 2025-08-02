#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Image Management - Command Line Interface

set -euo pipefail

zrbl_kindle() {
  ZRBL_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
  RBL_RBRR_FILE="${ZRBL_SCRIPT_DIR}/../rbrr_RecipeBottleRegimeRepo.sh"
  export RBL_RBRR_FILE

  ZRBL_KINDLED=1
}

zrbl_sentinel() {
  test "${ZRBGH_KINDLED:-}" = "1" || bcu_die "Module rbgh not kindled - call zrbgh_kindle first"
}

# eof

