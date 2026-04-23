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
# Recipe Bottle Handbook Onboarding - Install Retriever Credentials

set -euo pipefail

test -z "${ZRBHOCR_SOURCED:-}" || return 0
ZRBHOCR_SOURCED=1

rbho_credential_retriever() {
  zrbho_sentinel

  buc_doc_brief "Install retriever credentials — place RBRA key, validate, confirm pull access"
  buc_doc_shown || return 0

  buh_section "Install Retriever Credentials"
  buh_e
  buh_line "A ${RBYC_RETRIEVER} pulls container images from the"
  buh_line "  ${RBYC_DEPOT} — read-only access to what others have built."
  buh_e

  zrbho_credential_install "${RBCC_role_retriever}"

  buh_step1 "Confirm live access"
  buh_e
  buh_line "Run ${RBYC_TALLY} to list ${RBYC_HALLMARKS} in the registry using your retriever credential:"
  buh_e
  buh_tt   "   " "${RBZ_TALLY_HALLMARKS}"
  buh_e
  buh_line "If the command succeeds you have working pull access to the ${RBYC_DEPOT}."
  buh_line "If it fails, re-check the file placement in Step 2."
  buh_e

  buh_step1 "Next steps"
  buh_e
  buh_line "Return to the start menu:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
