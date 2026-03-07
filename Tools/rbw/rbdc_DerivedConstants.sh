#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Derived Constants - Credential file path resolution

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBDC_SOURCED:-}" || buc_die "Module rbdc multiply sourced - check sourcing hierarchy"
ZRBDC_SOURCED=1

######################################################################
# Internal Functions (zrbdc_*)

zrbdc_kindle() {
  test -z "${ZRBDC_KINDLED:-}" || buc_die "Module rbdc already kindled"
  zrbrr_sentinel

  # Derive credential file paths from RBRR_SECRETS_DIR
  readonly RBDC_GOVERNOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-governor.env"
  readonly RBDC_RETRIEVER_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-retriever.env"
  readonly RBDC_DIRECTOR_RBRA_FILE="${RBRR_SECRETS_DIR}/rbra-director.env"
  readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/rbro-payor.env"

  readonly ZRBDC_KINDLED=1
}

zrbdc_sentinel() {
  test "${ZRBDC_KINDLED:-}" = "1" || buc_die "Module rbdc not kindled - call zrbdc_kindle first"
}

# eof
