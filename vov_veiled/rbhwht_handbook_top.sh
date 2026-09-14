#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
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
  buh_section  "Onboarding — role-based walkthroughs"
  buh_line     "  Per-role setup guides with health probes."
  buh_tt       "  Start here:   " "${RBZ_ONBOARD_START_HERE}"
  buh_tt       "  Crash course: " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_e
  buh_section  "Payor — billing and OAuth ceremonies"
  buh_line     "  GCP project ownership, OAuth consent, credential refresh."
  buh_tt       "  Establish: " "${RBZ_PAYOR_ESTABLISH}"
  buh_tt       "  Quota:     " "${RBZ_QUOTA_BUILD}"
  buh_e
  buh_section  "Windows — test infrastructure"
  buh_line     "  SSH access, WSL, Cygwin, Docker for Windows-hosted testing."
  buh_tt       "  Full setup: " "${RBZ_HANDBOOK_WINDOWS}"

}

# eof
