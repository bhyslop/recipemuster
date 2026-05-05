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
# Recipe Bottle Windows Handbook - rbhw_top function

set -euo pipefail

test -z "${ZRBHW0_SOURCED:-}" || return 0
ZRBHW0_SOURCED=1

rbhw_top() {
  zrbhw_sentinel

  buc_doc_brief "Display Windows handbook orchestrator — full setup sequence"
  buc_doc_shown || return 0

  buh_section  "Windows Test Infrastructure Setup"
  buh_line     "Complete setup sequence for running Recipe Bottle tests on a Windows host."
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; local -r z_wsl_distro_yelp="${z_buym_yelp}"
  buh_line "   Target WSL distro: ${z_wsl_distro_yelp}"
  buyy_cmd_yawp "${ZRBHW_DOCKER_CONTEXT}"; local -r z_docker_context_yelp="${z_buym_yelp}"
  buh_line "   Docker context:    ${z_docker_context_yelp}"
  buh_e
  buh_section  "Phase 1: SSH Reachability (BUK — operator-manual prerequisite)"
  buh_tt       "  1. Jurisdiction handbook walkthrough:  " "${BUWZ_HJ0_TOP}"
  buh_line     "  2. SSH client key generation:          Generate SSH client key per vendor docs (e.g., 'ssh-keygen -t ed25519')"
  buh_e
  buh_section  "Phase 2: Admin Trust + sshd Harden (BUK — jurisdiction)"
  buh_tt       "  3. Fenestrate (admin SSH + harden):    " "${BUWZ_JP_FENESTRATE}" "" " <investiture>"
  buh_e
  buh_section  "Phase 3: Environments (BUK — generic OS install)"
  buh_line     "  4. WSL distribution setup:             Install WSL and create distribution per vendor docs"
  buh_line     "     Use distribution name: ${z_wsl_distro_yelp}"
  buh_line     "  5. Cygwin installation:                Install Cygwin POSIX userland per vendor docs"
  buh_e
  buh_section  "Phase 4: Workload Provisioning (BUK — jurisdiction)"
  buh_tt       "  6. Garrison-Cygwin (workload):         " "${BUWZ_JP_GARRISON_CYGWIN}" "" " <investiture>"
  buh_tt       "  7. Garrison-WSL (workload):            " "${BUWZ_JP_GARRISON_WSL}" "" " <investiture>"
  buh_e
  buh_section  "Phase 5: User Provisioning (JJK — fundus accounts)"
  buh_line     "  8. Fundus user provisioning runs inside WSL:"
  buh_tt       "     Phase 1 (create users):             " "${JJZ_FUNDUS_PHASE1}"
  buh_tt       "     Phase 2 (clone repos):              " "${JJZ_FUNDUS_PHASE2}"
  buh_e
  buh_section  "Phase 6: Docker (RBK — project-specific)"
  buh_tt       "  9. Docker Desktop install:             " "${RBZ_HW_DOCKER_DESKTOP}"
  buh_tt       " 10. Native dockerd in WSL:              " "${RBZ_HW_DOCKER_WSL_NATIVE}"
  buh_line "     Pass distro name: ${z_wsl_distro_yelp}"
  buh_tt       " 11. Docker context discipline:          " "${RBZ_HW_DOCKER_CONTEXT}"

}

# eof
