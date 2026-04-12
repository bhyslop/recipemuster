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
  buh_t        "Three handbook groups covering setup, operations, and maintenance."
  buh_e
  buh_index_buk
  buh_e
  buh_section  "Onboarding — role-based walkthroughs"
  buh_t        "  Per-role setup guides with health probes."
  buh_tT       "  Start here:   " "rbw-o"
  buh_tT       "  Crash course: " "rbw-Occ"
  buh_e
  buh_section  "Payor — billing and OAuth ceremonies"
  buh_t        "  GCP project ownership, OAuth consent, credential refresh."
  buh_tT       "  Establish: " "rbw-gPE"
  buh_tT       "  Refresh:   " "rbw-gPR"
  buh_tT       "  Quota:     " "rbw-gq"
  buh_e
  buh_section  "Windows — test infrastructure"
  buh_t        "  SSH access, WSL, Cygwin, Docker for Windows-hosted testing."
  buh_tT       "  Full setup: " "rbw-hw"

}

rbhw_docker_desktop() {
  zrbhw_sentinel

  buc_doc_brief "Display Docker Desktop installation procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Desktop Installation"
  buh_t        "Provide Windows-hosted Docker daemon for Windows and Cygwin environments."
  buh_e
  buh_step1    "Download and Install Docker Desktop:"
  buh_link     "" "Docker Desktop for Windows" "https://www.docker.com/products/docker-desktop/"
  buh_e
  buh_step1    "Enable WSL Integration:"
  buh_tu       "In Docker Desktop Settings > Resources > " "WSL Integration"
  buh_tu       "Enable " "Enable integration with my default WSL distro"
  buh_e
  buh_step1    "Start Docker Desktop:"
  buh_t        "Launch Docker Desktop from the Start menu or system tray."
  buh_e
  buh_section  "Verification:"
  buh_t        "In a Windows PowerShell:"
  buh_c        "docker ps"
  buh_t        "Expect: empty container list (no errors)."

}

rbhw_docker_wsl_native() {
  zrbhw_sentinel

  buc_doc_brief "Display native Docker daemon installation in WSL (arg: distro-name)"
  buc_doc_shown || return 0

  local z_distro="${BUZ_FOLIO:-}"
  test -n "${z_distro}" || buc_die "rbhw_docker_wsl_native: distro-name required"

  buh_section  "Native Docker Daemon in WSL"
  buh_tct      "Install and run dockerd natively inside WSL distro " "${z_distro}" "."
  buh_e
  buh_step1    "Enter the Distribution:"
  buh_c        "wsl -d ${z_distro}"
  buh_e
  buh_step1    "Install Docker Engine:"
  buh_c        "sudo apt update"
  buh_c        "sudo apt install -y docker.io"
  buh_c        "sudo systemctl enable --now docker"
  buh_e
  buh_step1    "Grant Docker Access to Users:"
  buh_t        "Add each user that needs Docker access to the docker group:"
  buh_c        "sudo usermod -aG docker USERNAME"
  buh_t        "Replace USERNAME with each fundus user account."
  buh_e
  buh_section  "Verification:"
  buh_c        "docker ps"
  buh_t        "Expect: empty container list (no errors)."

}

rbhw_docker_context_discipline() {
  zrbhw_sentinel

  buc_doc_brief "Display deterministic Docker daemon selection procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Context Discipline"
  buh_t        "Ensure deterministic daemon selection across environments."
  buh_e
  buh_section  "Preconditions:"
  buh_t        "- Docker Desktop installed (docker-desktop procedure)"
  buh_t        "- Native dockerd in WSL (docker-wsl-native procedure)"
  buh_e
  buh_step1    "Inside WSL — Create Named Context:"
  buh_tct      "Create context " "${ZRBHW_DOCKER_CONTEXT}" " for the native daemon:"
  buh_c        "docker context create ${ZRBHW_DOCKER_CONTEXT} --docker \"host=unix:///var/run/docker.sock\""
  buh_c        "docker context use ${ZRBHW_DOCKER_CONTEXT}"
  buh_e
  buh_step1    "On Windows (PowerShell) — Confirm Default Context:"
  buh_t        "Ensure the default context is active (Docker Desktop):"
  buh_c        "docker context use default"
  buh_e
  buh_section  "Result:"
  buh_t        "- WSL shells use native daemon (via context)"
  buh_t        "- Windows and Cygwin use Docker Desktop daemon (default)"
  buh_e
  buh_section  "Verification:"
  buh_c        "docker context ls"
  buh_c        "docker info | grep \"Server\""
  buh_t        "Expect: active context matches the environment."

}

rbhw_top() {
  zrbhw_sentinel

  buc_doc_brief "Display Windows handbook orchestrator — full setup sequence"
  buc_doc_shown || return 0

  buh_section  "Windows Test Infrastructure Setup"
  buh_t        "Complete setup sequence for running Recipe Bottle tests on a Windows host."
  buh_tc       "   Target WSL distro: " "${ZRBHW_WSL_DISTRO}"
  buh_tc       "   Docker context:    " "${ZRBHW_DOCKER_CONTEXT}"
  buh_e
  buh_section  "Phase 1: SSH Access (BUK — generic OS)"
  buh_tT       "  1. OpenSSH server install & lockdown:  " "buw-HWab"
  buh_tT       "  2. SSH client key & host config:       " "buw-HWar"
  buh_tT       "  3. SSH entrypoint routing (command=):  " "buw-HWax"
  buh_e
  buh_section  "Phase 2: Environments (BUK — generic OS)"
  buh_tT       "  4. WSL distribution setup:             " "buw-HWew"
  buh_tc       "     Pass distro name: " "${ZRBHW_WSL_DISTRO}"
  buh_tT       "  5. Cygwin installation:                " "buw-HWec"
  buh_e
  buh_section  "Phase 3: User Provisioning (JJK — fundus accounts)"
  buh_t        "  6. Fundus user provisioning runs inside WSL:"
  buh_tT       "     Phase 1 (create users):             " "jjw-tfP1"
  buh_t        "     Phase 2 (clone repos):              tt/jjw-tfP2.ProvisionPhase2.{host}.sh"
  buh_e
  buh_section  "Phase 4: Docker (RBK — project-specific)"
  buh_tT       "  7. Docker Desktop install:             " "rbw-HWdd"
  buh_tT       "  8. Native dockerd in WSL:              " "rbw-HWdw"
  buh_tc       "     Pass distro name: " "${ZRBHW_WSL_DISTRO}"
  buh_tT       "  9. Docker context discipline:          " "rbw-HWdc"

}

# eof
