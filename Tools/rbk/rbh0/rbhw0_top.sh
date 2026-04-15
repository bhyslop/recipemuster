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
  buh_section  "Phase 1: SSH Access (BUK — generic OS)"
  buh_tt       "  1. OpenSSH server install & lockdown:  " "${BUWZ_HW_ACCESS_BASE}"
  buh_tt       "  2. SSH client key & host config:       " "${BUWZ_HW_ACCESS_REMOTE}"
  buh_tt       "  3. SSH entrypoint routing (command=):  " "${BUWZ_HW_ACCESS_ENTRY}"
  buh_e
  buh_section  "Phase 2: Environments (BUK — generic OS)"
  buh_tt       "  4. WSL distribution setup:             " "${BUWZ_HW_ENV_WSL}"
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; local -r z_wsl_distro_env_yelp="${z_buym_yelp}"
  buh_line "     Pass distro name: ${z_wsl_distro_env_yelp}"
  buh_tt       "  5. Cygwin installation:                " "${BUWZ_HW_ENV_CYGWIN}"
  buh_e
  buh_section  "Phase 3: User Provisioning (JJK — fundus accounts)"
  buh_line     "  6. Fundus user provisioning runs inside WSL:"
  buh_tt       "     Phase 1 (create users):             " "${JJZ_FUNDUS_PHASE1}"
  buh_line     "     Phase 2 (clone repos):              tt/jjw-tfP2.ProvisionPhase2.{host}.sh"
  buh_e
  buh_section  "Phase 4: Docker (RBK — project-specific)"
  buh_tt       "  7. Docker Desktop install:             " "${RBZ_HW_DOCKER_DESKTOP}"
  buh_tt       "  8. Native dockerd in WSL:              " "${RBZ_HW_DOCKER_WSL_NATIVE}"
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; local -r z_wsl_distro_docker_yelp="${z_buym_yelp}"
  buh_line "     Pass distro name: ${z_wsl_distro_docker_yelp}"
  buh_tt       "  9. Docker context discipline:          " "${RBZ_HW_DOCKER_CONTEXT}"

}

# eof
