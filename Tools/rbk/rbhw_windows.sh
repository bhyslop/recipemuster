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
# Recipe Bottle Handbook - Windows Test Infrastructure
#
# Project-specific Docker topology and orchestration for Windows-hosted
# testing. Generic OS mechanisms (SSH, WSL, Cygwin) live in buhw_windows.sh.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHW_SOURCED:-}" || buc_die "Module rbhw multiply sourced - check sourcing hierarchy"
ZRBHW_SOURCED=1

######################################################################
# Internal: Kindle and Sentinel

zrbhw_kindle() {
  test -z "${ZRBHW_KINDLED:-}" || buc_die "Module rbhw already kindled"

  # Kindle — project-specific constants for Windows test infrastructure
  readonly ZRBHW_WSL_DISTRO="rbtww-main"
  readonly ZRBHW_DOCKER_CONTEXT="wsl-native"

  readonly ZRBHW_KINDLED=1
}

zrbhw_sentinel() {
  test "${ZRBHW_KINDLED:-}" = "1" || buc_die "Module rbhw not kindled - call zrbhw_kindle first"
}

######################################################################
# External Functions (rbhw_*)

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

rbhw_docker_desktop() {
  zrbhw_sentinel

  buc_doc_brief "Display Docker Desktop installation procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Desktop Installation"
  buh_line     "Provide Windows-hosted Docker daemon for Windows and Cygwin environments."
  buh_e
  buh_step1    "Download and Install Docker Desktop:"
  buh_link     "" "Docker Desktop for Windows" "https://www.docker.com/products/docker-desktop/"
  buh_e
  buh_step1    "Enable WSL Integration:"
  buyy_ui_yawp "WSL Integration"; buh_line "In Docker Desktop Settings > Resources > ${z_buym_yelp}"
  buyy_ui_yawp "Enable integration with my default WSL distro"; buh_line "Enable ${z_buym_yelp}"
  buh_e
  buh_step1    "Start Docker Desktop:"
  buh_line     "Launch Docker Desktop from the Start menu or system tray."
  buh_e
  buh_section  "Verification:"
  buh_line     "In a Windows PowerShell:"
  buh_code     "docker ps"
  buh_line     "Expect: empty container list (no errors)."

}

rbhw_docker_wsl_native() {
  zrbhw_sentinel

  buc_doc_brief "Display native Docker daemon installation in WSL (arg: distro-name)"
  buc_doc_shown || return 0

  local z_distro="${BUZ_FOLIO:-}"
  test -n "${z_distro}" || buc_die "rbhw_docker_wsl_native: distro-name required"

  buh_section  "Native Docker Daemon in WSL"
  buyy_cmd_yawp "${z_distro}"; buh_line "Install and run dockerd natively inside WSL distro ${z_buym_yelp}."
  buh_e
  buh_step1    "Enter the Distribution:"
  buh_code     "wsl -d ${z_distro}"
  buh_e
  buh_step1    "Install Docker Engine:"
  buh_code     "sudo apt update"
  buh_code     "sudo apt install -y docker.io"
  buh_code     "sudo systemctl enable --now docker"
  buh_e
  buh_step1    "Grant Docker Access to Users:"
  buh_line     "Add each user that needs Docker access to the docker group:"
  buh_code     "sudo usermod -aG docker USERNAME"
  buh_line     "Replace USERNAME with each fundus user account."
  buh_e
  buh_section  "Verification:"
  buh_code     "docker ps"
  buh_line     "Expect: empty container list (no errors)."

}

rbhw_docker_context_discipline() {
  zrbhw_sentinel

  buc_doc_brief "Display deterministic Docker daemon selection procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Context Discipline"
  buh_line     "Ensure deterministic daemon selection across environments."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- Docker Desktop installed (docker-desktop procedure)"
  buh_line     "- Native dockerd in WSL (docker-wsl-native procedure)"
  buh_e
  buh_step1    "Inside WSL — Create Named Context:"
  buyy_cmd_yawp "${ZRBHW_DOCKER_CONTEXT}"; buh_line "Create context ${z_buym_yelp} for the native daemon:"
  buh_code     "docker context create ${ZRBHW_DOCKER_CONTEXT} --docker \"host=unix:///var/run/docker.sock\""
  buh_code     "docker context use ${ZRBHW_DOCKER_CONTEXT}"
  buh_e
  buh_step1    "On Windows (PowerShell) — Confirm Default Context:"
  buh_line     "Ensure the default context is active (Docker Desktop):"
  buh_code     "docker context use default"
  buh_e
  buh_section  "Result:"
  buh_line     "- WSL shells use native daemon (via context)"
  buh_line     "- Windows and Cygwin use Docker Desktop daemon (default)"
  buh_e
  buh_section  "Verification:"
  buh_code     "docker context ls"
  buh_code     "docker info | grep \"Server\""
  buh_line     "Expect: active context matches the environment."

}

rbhw_top() {
  zrbhw_sentinel

  buc_doc_brief "Display Windows handbook orchestrator — full setup sequence"
  buc_doc_shown || return 0

  buh_section  "Windows Test Infrastructure Setup"
  buh_line     "Complete setup sequence for running Recipe Bottle tests on a Windows host."
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; buh_line "   Target WSL distro: ${z_buym_yelp}"
  buyy_cmd_yawp "${ZRBHW_DOCKER_CONTEXT}"; buh_line "   Docker context:    ${z_buym_yelp}"
  buh_e
  buh_section  "Phase 1: SSH Access (BUK — generic OS)"
  buh_tt       "  1. OpenSSH server install & lockdown:  " "${BUWZ_HW_ACCESS_BASE}"
  buh_tt       "  2. SSH client key & host config:       " "${BUWZ_HW_ACCESS_REMOTE}"
  buh_tt       "  3. SSH entrypoint routing (command=):  " "${BUWZ_HW_ACCESS_ENTRY}"
  buh_e
  buh_section  "Phase 2: Environments (BUK — generic OS)"
  buh_tt       "  4. WSL distribution setup:             " "${BUWZ_HW_ENV_WSL}"
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; buh_line "     Pass distro name: ${z_buym_yelp}"
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
  buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"; buh_line "     Pass distro name: ${z_buym_yelp}"
  buh_tt       "  9. Docker context discipline:          " "${RBZ_HW_DOCKER_CONTEXT}"

}

# eof
