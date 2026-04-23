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
# Recipe Bottle Handbook Onboarding - Install Director Credentials

set -euo pipefail

test -z "${ZRBHOCD_SOURCED:-}" || return 0
ZRBHOCD_SOURCED=1

rbho_credential_director() {
  zrbho_sentinel

  buc_doc_brief "Install director credentials — place RBRA key, validate, confirm build access"
  buc_doc_shown || return 0

  buh_section "Install Director Credentials"
  buh_e
  buh_line "A ${RBYC_DIRECTOR} causes cloud builds and publishes container images to the"
  buh_line "  ${RBYC_DEPOT} — write access to the registry."
  buh_e

  zrbho_credential_install "${RBCC_role_director}"

  buh_step1 "Confirm live access"
  buh_e
  buh_line "Run ${RBYC_REKON} to list raw image tags in the registry"
  buh_line "using your director credential:"
  buh_e
  buh_tt   "   " "${RBZ_REKON_IMAGE}" "" " rbev-busybox"
  buh_e
  buh_line "If the command succeeds you have working build access to the"
  buh_line "  ${RBYC_DEPOT}."
  buh_line "If it fails, re-check the file placement in Step 2."
  buh_e

  buh_step1 "Next steps"
  buh_e
  buh_line "Return to the start menu:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
