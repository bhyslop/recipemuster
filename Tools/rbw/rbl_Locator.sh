#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Image Management - Command Line Interface

set -euo pipefail

test -z "${ZRBL_SOURCED:-}" || buc_die "Module rbl multiply sourced - check sourcing hierarchy"
ZRBL_SOURCED=1

zrbl_kindle() {
  test -z "${ZRBL_KINDLED:-}" || buc_die "Module rbl already kindled"

  buc_log_args "Validate required tools"
  command -v openssl >/dev/null 2>&1 || buc_die "openssl not found - required for JWT signing"
  command -v curl    >/dev/null 2>&1 || buc_die "curl not found - required for OAuth exchange"
  command -v base64  >/dev/null 2>&1 || buc_die "base64 not found - required for encoding"
  command -v jq      >/dev/null 2>&1 || buc_die "jq not found - required for JSON parsing"

  # ITCH_BASH_BASENAME
  ZRBL_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

  RBL_RBRR_FILE="${ZRBL_SCRIPT_DIR}/../../rbrr.env"
  export RBL_RBRR_FILE

  RBL_RBRP_FILE="${ZRBL_SCRIPT_DIR}/../../rbrp.env"
  export RBL_RBRP_FILE

  ZRBL_KINDLED=1
}

zrbl_sentinel() {
  test "${ZRBL_KINDLED:-}" = "1" || buc_die "Module rbl not kindled - call zrbl_kindle first"
}

# eof

