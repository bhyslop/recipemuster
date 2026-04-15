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
# Recipe Bottle Windows Handbook - rbhw_handbook_top function

set -euo pipefail

test -z "${ZRBHWHT_SOURCED:-}" || return 0
ZRBHWHT_SOURCED=1

rbhw_handbook_top() {
  zrbhw_sentinel

  buc_doc_brief "Display top-level handbook index across all groups"
  buc_doc_shown || return 0

  buh_section  "Recipe Bottle Handbook"
  buh_line     "Three handbook groups covering setup, operations, and maintenance."
  buh_e
  buh_index_buk
  buh_e
  buh_section  "Onboarding — role-based walkthroughs"
  buh_line     "  Per-role setup guides with health probes."
  buh_tt       "  Start here:   " "${RBZ_ONBOARD_START_HERE}"
  buh_tt       "  Crash course: " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_e
  buh_section  "Payor — billing and OAuth ceremonies"
  buh_line     "  GCP project ownership, OAuth consent, credential refresh."
  buh_tt       "  Establish: " "${RBZ_PAYOR_ESTABLISH}"
  buh_tt       "  Refresh:   " "${RBZ_PAYOR_REFRESH}"
  buh_tt       "  Quota:     " "${RBZ_QUOTA_BUILD}"
  buh_e
  buh_section  "Windows — test infrastructure"
  buh_line     "  SSH access, WSL, Cygwin, Docker for Windows-hosted testing."
  buh_tt       "  Full setup: " "${RBZ_HANDBOOK_WINDOWS}"

}

# eof
