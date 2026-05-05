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
# BUW Zipper - Colophon registry for BUK workbench dispatch

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUWZ_SOURCED:-}" || return 0
ZBUWZ_SOURCED=1

######################################################################
# Colophon registry initialization

zbuwz_kindle() {
  test -z "${ZBUWZ_KINDLED:-}" || buc_die "buwz already kindled"

  # Verify buz zipper is kindled (CLI furnish must kindle buz first)
  zbuz_sentinel

  # TabTarget subsystem (buut_cli.sh)
  local z_mod="buut_cli.sh"
  buz_enroll BUWZ_TT_LIST_LAUNCHERS      "buw-tt-ll"  "${z_mod}" "buut_list_launchers"               ""  "List all registered launchers"
  buz_enroll BUWZ_TT_BATCH_LOGGING       "buw-tt-cbl" "${z_mod}" "buut_tabtarget_batch_logging"      ""  "Create batch tabtarget with logging"
  buz_enroll BUWZ_TT_BATCH_NOLOG         "buw-tt-cbn" "${z_mod}" "buut_tabtarget_batch_nolog"        ""  "Create batch tabtarget without logging"
  buz_enroll BUWZ_TT_INTERACTIVE_LOGGING "buw-tt-cil" "${z_mod}" "buut_tabtarget_interactive_logging" ""  "Create interactive tabtarget with logging"
  buz_enroll BUWZ_TT_INTERACTIVE_NOLOG   "buw-tt-cin" "${z_mod}" "buut_tabtarget_interactive_nolog"  ""  "Create interactive tabtarget without logging"
  buz_enroll BUWZ_TT_LAUNCHER            "buw-tt-cl"  "${z_mod}" "buut_launcher"                     ""  "Create launcher script"

  # Config Regime subsystem (burc_cli.sh)
  z_mod="burc_cli.sh"
  buz_enroll BUWZ_RC_VALIDATE "buw-rcv" "${z_mod}" "burc_validate"  ""  "Validate BURC regime"
  buz_enroll BUWZ_RC_RENDER   "buw-rcr" "${z_mod}" "burc_render"    ""  "Render BURC regime"

  # Station Regime subsystem (burs_cli.sh)
  z_mod="burs_cli.sh"
  buz_enroll BUWZ_RS_VALIDATE "buw-rsv" "${z_mod}" "burs_validate"  ""  "Validate BURS regime"
  buz_enroll BUWZ_RS_RENDER   "buw-rsr" "${z_mod}" "burs_render"    ""  "Render BURS regime"

  # Environment Regime subsystem (bure_cli.sh)
  z_mod="bure_cli.sh"
  buz_enroll BUWZ_RE_VALIDATE "buw-rev" "${z_mod}" "bure_validate"  ""  "Validate BURE regime"
  buz_enroll BUWZ_RE_RENDER   "buw-rer" "${z_mod}" "bure_render"    ""  "Render BURE regime"

  # Node Regime subsystem (burn_cli.sh)
  z_mod="burn_cli.sh"
  buz_enroll BUWZ_RN_VALIDATE "buw-rnv" "${z_mod}" "burn_validate" "param1" "Validate BURN profile"
  buz_enroll BUWZ_RN_RENDER   "buw-rnr" "${z_mod}" "burn_render"   "param1" "Render BURN profile"
  buz_enroll BUWZ_RN_LIST     "buw-rnl" "${z_mod}" "burn_list"     ""       "List BURN profiles"

  # Privileged Regime subsystem (burp_cli.sh)
  z_mod="burp_cli.sh"
  buz_enroll BUWZ_RP_VALIDATE "buw-rpv" "${z_mod}" "burp_validate" "param1" "Validate BURP profile"
  buz_enroll BUWZ_RP_RENDER   "buw-rpr" "${z_mod}" "burp_render"   "param1" "Render BURP profile"
  buz_enroll BUWZ_RP_LIST     "buw-rpl" "${z_mod}" "burp_list"     ""       "List BURP profiles"

  # Qualification subsystem (buq_cli.sh)
  z_mod="buq_cli.sh"
  buz_enroll BUWZ_QUALIFY_SHELLCHECK "buw-qsc" "${z_mod}" "buq_shellcheck"  ""  "Run shellcheck on all tools"

  # Test fixtures (bux_cli.sh)
  z_mod="bux_cli.sh"
  buz_enroll BUWZ_DELAY "buw-xd" "${z_mod}" "bux_delay"  ""  "Sleep 20 seconds (timing fixture)"

  # Self-test (butt_testbench.sh)
  z_mod="butt_testbench.sh"
  buz_enroll BUWZ_SELF_TEST "buw-st" "${z_mod}" "buw-st"  ""  "BUK self-test (kick-tires + bure-tweak)"

  # Handbook — top index + Windows OS procedures (buhw_cli.sh)
  z_mod="buhw_cli.sh"
  buz_enroll BUWZ_H0_TOP            "buw-h0"    "${z_mod}" "buhw_handbook_top"        ""        "BUK top-level handbook index"
  buz_enroll BUWZ_HW_TOP            "buw-hw"    "${z_mod}" "buhw_top"                ""        "BUK-level Windows procedures checklist"
  buz_enroll BUWZ_HW_ACCESS_BASE    "buw-HWab"  "${z_mod}" "buhw_access_base"        ""        "OpenSSH server install + lockdown"
  buz_enroll BUWZ_HW_ACCESS_REMOTE  "buw-HWar"  "${z_mod}" "buhw_access_remote"      ""        "Client key generation"
  buz_enroll BUWZ_HW_ACCESS_ENTRY   "buw-HWax"  "${z_mod}" "buhw_access_entrypoints" ""        "SSH command= routing + icacls"
  buz_enroll BUWZ_HW_ENV_WSL        "buw-HWew"  "${z_mod}" "buhw_environment_wsl"    "param1"  "WSL distro creation"
  buz_enroll BUWZ_HW_ENV_CYGWIN     "buw-HWec"  "${z_mod}" "buhw_environment_cygwin" ""        "Cygwin install"

  # Handbook — jurisdiction (buhj_cli.sh)
  z_mod="buhj_cli.sh"
  buz_enroll BUWZ_HJ0_TOP           "buw-hj0"   "${z_mod}" "buhj_top"                ""        "Jurisdiction handbook landing + admin SSH bootstrap"

  # Jurisdiction operational — workload ceremonies (bujb_cli.sh)
  z_mod="bujb_cli.sh"
  buz_enroll BUWZ_JW_KNOCK          "buw-jwk"   "${z_mod}" "bujb_knock"               "param1"  "Knock — probe workload SSH reachability"
  buz_enroll BUWZ_JW_COMMAND_FILE   "buw-jwc"   "${z_mod}" "bujb_command_file"        "param1"  "Run command file as workload, capture outputs"
  buz_enroll BUWZ_JW_INTERACTIVE    "buw-jws"   "${z_mod}" "bujb_interactive_session" "param1"  "Interactive SSH session as workload"

  # Jurisdiction operational — fenestrate (bujb_cli.sh)
  buz_enroll BUWZ_JP_FENESTRATE       "buw-jpF"   "${z_mod}" "bujb_fenestrate_command" "param1"  "Fenestrate — admin SSH trust + sshd_config harden (Windows)"

  # Jurisdiction operational — garrison ceremonies (bujb_cli.sh)
  buz_enroll BUWZ_JP_GARRISON_BASH    "buw-jpGb"  "${z_mod}" "bujb_garrison_bash"     "param1"  "Garrison workload (shell-letter b: native bash, Linux/Mac)"
  buz_enroll BUWZ_JP_GARRISON_CYGWIN  "buw-jpGc"  "${z_mod}" "bujb_garrison_cygwin"   "param1"  "Garrison workload (shell-letter c: Cygwin bash, Windows)"
  buz_enroll BUWZ_JP_GARRISON_WSL     "buw-jpGw"  "${z_mod}" "bujb_garrison_wsl"      "param1"  "Garrison workload (shell-letter w: WSL bash, Windows)"

  readonly ZBUWZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zbuwz_healthcheck() {
  zbuwz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zbuwz_sentinel() {
  test "${ZBUWZ_KINDLED:-}" = "1" || buc_die "Module buwz not kindled - call zbuwz_kindle first"
}

# eof
