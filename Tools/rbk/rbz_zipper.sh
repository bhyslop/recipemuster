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
# RBW Zipper - Colophon registry for RBW workbench dispatch

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBZ_SOURCED:-}" || return 0
ZRBZ_SOURCED=1

######################################################################
# Colophon registry initialization

zrbz_kindle() {
  test -z "${ZRBZ_KINDLED:-}" || buc_die "rbz already kindled"

  # Verify buz zipper is kindled (CLI furnish must kindle buz first)
  zbuz_sentinel

  # Accounts — Google Cloud service accounts (rbw-a)
  buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Accounts — Service account management"
  local z_mod="rbgp_cli.sh"
  buz_enroll RBZ_MANTLE_GOVERNOR         "rbw-aM"  "${z_mod}" "rbgp_governor_mantle"       ""        "Create/replace governor service account"
  z_mod="rbgg_cli.sh"
  buz_enroll RBZ_CHARTER_RETRIEVER       "rbw-aC"  "${z_mod}" "rbgg_charter_retriever"     ""        "Charter retriever (image pull) service account"
  buz_enroll RBZ_KNIGHT_DIRECTOR         "rbw-aK"  "${z_mod}" "rbgg_knight_director"       ""        "Knight director (build) service account"
  buz_enroll RBZ_LIST_SERVICE_ACCOUNTS   "rbw-aL"  "${z_mod}" "rbgg_list_service_accounts" ""        "List issued service accounts"
  buz_enroll RBZ_FORFEIT_SERVICE_ACCOUNT "rbw-aF"  "${z_mod}" "rbgg_forfeit_service_account" ""      "Forfeit a service account"

  # Crucible — container runtime (rbw-c)
  buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible — Container runtime"
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_CRUCIBLE_CHARGE  "rbw-cC"  "${z_mod}" "rbob_charge"       "imprint"  "Charge crucible (sentry + pentacle + bottle containers)"
  buz_enroll RBZ_CRUCIBLE_QUENCH  "rbw-cQ"  "${z_mod}" "rbob_quench"       "imprint"  "Quench crucible"
  buz_enroll RBZ_CRUCIBLE_SSH     "rbw-cS"  "${z_mod}" "rbob_ssh"          "imprint"  "SSH into the bottle container"
  buz_enroll RBZ_CRUCIBLE_HAIL    "rbw-ch"  "${z_mod}" "rbob_hail"         "param1"   "Shell into the sentry container"
  buz_enroll RBZ_CRUCIBLE_RACK    "rbw-cr"  "${z_mod}" "rbob_rack"         "param1"   "Shell into the bottle container"
  buz_enroll RBZ_CRUCIBLE_SCRY    "rbw-cs"  "${z_mod}" "rbob_scry"         "param1"   "Observe network traffic on crucible containers"
  buz_enroll RBZ_CRUCIBLE_WRIT    "rbw-cw"  "${z_mod}" "rbob_writ"         "imprint"  "Non-interactive exec in sentry container"
  buz_enroll RBZ_CRUCIBLE_FIAT    "rbw-cf"  "${z_mod}" "rbob_fiat"         "imprint"  "Non-interactive exec in pentacle container"
  buz_enroll RBZ_CRUCIBLE_BARK    "rbw-cb"  "${z_mod}" "rbob_bark"         "imprint"  "Non-interactive exec in bottle container"
  buz_enroll RBZ_CRUCIBLE_ACTIVE  "rbw-cic" "${z_mod}" "rbob_charged"      "param1"   "Check if crucible is currently charged"
  buz_enroll RBZ_CRUCIBLE_KLUDGE_BOTTLE "rbw-cKB" "${z_mod}" "rbob_kludge_bottle" "param1" "Kludge bottle vessel and drive hallmark into nameplate"

  # Depot — GCP project infrastructure (rbw-d, UPPER=mutates, lower=read)
  buz_group RBZ__GROUP_DEPOT      "rbw-d"   "Depot — GCP project infrastructure"
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_LEVY_DEPOT             "rbw-dL"  "${z_mod}" "rbgp_depot_levy"   ""  "Provision GCP depot project"
  buz_enroll RBZ_UNMAKE_DEPOT           "rbw-dU"  "${z_mod}" "rbgp_depot_unmake"  ""  "Permanently remove a depot"
  buz_enroll RBZ_LIST_DEPOT             "rbw-dl"  "${z_mod}" "rbgp_depot_list"    ""  "List all active depots"
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ENSHRINE_VESSEL        "rbw-dE"  "${z_mod}" "rbfd_enshrine"      ""  "Enshrine upstream base images to GAR via Cloud Build"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_INSCRIBE_RELIQUARY     "rbw-dI"  "${z_mod}" "rbfl_inscribe"      ""  "Create reliquary: mirror tool images from upstream to GAR"

  # Guide — human-directed procedures (rbw-g)
  buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Guide — Human-directed procedures"
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_PAYOR_INSTALL          "rbw-gPI" "${z_mod}" "rbgp_payor_install"   ""  "Ingest OAuth credentials from JSON key file"
  z_mod="rbhp_cli.sh"
  buz_enroll RBZ_PAYOR_ESTABLISH        "rbw-gPE" "${z_mod}" "rbhp_establish"       ""  "Guided Manor establishment — GCP project + OAuth consent screen"
  buz_enroll RBZ_PAYOR_REFRESH          "rbw-gPR" "${z_mod}" "rbhp_refresh"         ""  "Refresh expired OAuth tokens"
  buz_enroll RBZ_QUOTA_BUILD            "rbw-gq"  "${z_mod}" "rbhp_quota_build"     ""  "Display Cloud Build capacity review procedure"
  z_mod="rbho_cli.sh"
  buz_enroll RBZ_ONBOARD_TRIAGE        "rbw-go"  "${z_mod}" "rbho_triage"     ""  "Triage — detect roles, route to per-role walkthrough"
  buz_enroll RBZ_ONBOARD_RETRIEVER     "rbw-gOR" "${z_mod}" "rbho_retriever"  ""  "Retriever walkthrough — pull and run vessel images"
  buz_enroll RBZ_ONBOARD_DIRECTOR      "rbw-gOD" "${z_mod}" "rbho_director"   ""  "Director walkthrough — build and publish vessel images"
  buz_enroll RBZ_ONBOARD_GOVERNOR      "rbw-gOG" "${z_mod}" "rbho_governor"   ""  "Governor walkthrough — manage service accounts and access"
  buz_enroll RBZ_ONBOARD_PAYOR         "rbw-gOP" "${z_mod}" "rbho_payor"      ""  "Payor walkthrough — GCP project, billing, and OAuth setup"
  buz_enroll RBZ_ONBOARD_REFERENCE     "rbw-gOr" "${z_mod}" "rbho_reference"  ""  "Reference — all roles, all units, single health dashboard"

  # Onboarding — handbook restart (rbw-o terminal + rbw-O* family, see ₣A6 paddock)
  buz_group RBZ__GROUP_ONBOARDING "rbw-o"   "Onboarding — Handbook restart"
  z_mod="rbho_cli.sh"
  buz_enroll RBZ_ONBOARD_START_HERE    "rbw-o"   "${z_mod}" "rbho_start_here"            ""  "Onboarding start — probe-aware menu into handbook tracks"
  buz_enroll RBZ_ONBOARD_CRASH_COURSE  "rbw-Occ" "${z_mod}" "rbho_crash_course"          ""  "Crash Course — universal prerequisite: tabtargets, regimes, diagnostic failure"
  buz_enroll RBZ_ONBOARD_CRED_RETRIEVER "rbw-Ocr" "${z_mod}" "rbho_credential_retriever" ""  "Install retriever credentials — place RBRA key file"
  buz_enroll RBZ_ONBOARD_CRED_DIRECTOR  "rbw-Ocd" "${z_mod}" "rbho_credential_director"  ""  "Install director credentials — place RBRA key file"
  buz_enroll RBZ_ONBOARD_FIRST_CRUCIBLE "rbw-Ofc" "${z_mod}" "rbho_first_crucible"       ""  "Start a Crucible using local builds — kludge, charge, rack"
  buz_enroll RBZ_ONBOARD_PAYOR_HB      "rbw-Op"  "${z_mod}" "rbho_payor_handbook"       ""  "Payor — establish a Manor and provision the Depot"
  buz_enroll RBZ_ONBOARD_GOVERNOR_HB   "rbw-Og"  "${z_mod}" "rbho_governor_handbook"    ""  "Governor — administer service accounts for directors and retrievers"

  # Foundry — registry artifact lifecycle (rbw-f, UPPER=mutates GAR, lower=read/local)
  buz_group RBZ__GROUP_FOUNDRY    "rbw-f"   "Foundry — Registry artifact lifecycle"
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ORDAIN_HALLMARK        "rbw-fO"  "${z_mod}" "rbfd_ordain"          ""  "Ordain hallmark: conjure, bind, or graft based on vessel mode"
  buz_enroll RBZ_KLUDGE_VESSEL          "rbw-fk"  "${z_mod}" "rbfd_kludge"          ""  "Kludge a vessel image locally for development"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_ABJURE_HALLMARK        "rbw-fA"  "${z_mod}" "rbfl_abjure"          ""  "Abjure a hallmark (delete artifacts from GAR)"
  buz_enroll RBZ_TALLY_HALLMARKS        "rbw-ft"  "${z_mod}" "rbfl_tally"           ""  "Tally hallmarks by health state"
  z_mod="rbfv_cli.sh"
  buz_enroll RBZ_VOUCH_HALLMARKS        "rbw-fV"  "${z_mod}" "rbfv_batch_vouch"     ""  "Mode-aware vouch: SLSA (conjure), digest-pin (bind), GRAFTED (graft)"
  z_mod="rbfr_cli.sh"
  buz_enroll RBZ_SUMMON_HALLMARK        "rbw-fs"  "${z_mod}" "rbfr_summon"          ""  "Summon vouched hallmark image locally"
  z_mod="rbfc_cli.sh"
  buz_enroll RBZ_PLUMB_FULL             "rbw-fpf" "${z_mod}" "rbfc_plumb_full"      ""  "Full provenance display (SBOM, build info, Dockerfile)"
  buz_enroll RBZ_PLUMB_COMPACT          "rbw-fpc" "${z_mod}" "rbfc_plumb_compact"   ""  "Compact provenance summary"

  # Ifrit — attack binary (rbw-I)
  buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit — Attack binary"
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_BOTTLE_IFRIT   "rbw-Ic"  "${z_mod}" "rbob_ifrit_client"  "imprint"  "Launch Claude Code inside a running bottle for escape testing"
  buz_enroll RBZ_BOTTLE_SORTIE  "rbw-Is"  "${z_mod}" "rbob_ifrit_sortie"  "imprint"  "Run automated security test scripts inside the bottle"

  # Image — container image operations (rbw-i, UPPER=mutates, lower=read)
  buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Image — Container image operations"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_JETTISON_IMAGE         "rbw-iJ"  "${z_mod}" "rbfl_jettison"  ""  "Jettison a specific image tag from registry"
  buz_enroll RBZ_REKON_IMAGE            "rbw-ir"  "${z_mod}" "rbfl_rekon"     ""  "Raw GAR tag listing for a vessel package"
  z_mod="rbfr_cli.sh"
  buz_enroll RBZ_WREST_IMAGE            "rbw-iw"  "${z_mod}" "rbfr_wrest"     ""  "Wrest a specific image from registry"

  # Marshal — lifecycle (rbw-M)
  buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Marshal — Lifecycle"
  buz_enroll RBZ_MARSHAL_ZERO           "rbw-MZ"  "rblm_cli.sh" "rblm_zero"      ""  "Zero regime to blank template"
  buz_enroll RBZ_MARSHAL_PROOF          "rbw-MP"  "rblm_cli.sh" "rblm_proof"     ""  "Proof repo for release testing"
  buz_enroll RBZ_MARSHAL_GENERATE       "rbw-MG"  "rblm_cli.sh" "rblm_generate"  ""  "Generate tabtarget context from zipper registry"

  # Nameplate — cross-nameplate operations (rbw-n)
  buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Nameplate — Cross-nameplate operations"
  z_mod="rbrn_cli.sh"
  buz_enroll RBZ_LIST_NAMEPLATES        "rbw-rnl" "${z_mod}" "rbrn_list"    ""  "List all nameplates"
  buz_enroll RBZ_SURVEY_NAMEPLATES      "rbw-ni"  "${z_mod}" "rbrn_survey"  ""  "Survey nameplate status"
  buz_enroll RBZ_AUDIT_NAMEPLATES       "rbw-nv"  "${z_mod}" "rbrn_audit"   ""  "Validate all nameplates"

  # Regime — config files (rbw-r)
  buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime — Config files"
  z_mod="rbrn_cli.sh"
  buz_enroll RBZ_RENDER_NAMEPLATE       "rbw-rnr" "${z_mod}" "rbrn_render"    "param1"  "Render nameplate regime"
  buz_enroll RBZ_VALIDATE_NAMEPLATE     "rbw-rnv" "${z_mod}" "rbrn_validate"  "param1"  "Validate nameplate regime"
  z_mod="rbrv_cli.sh"
  buz_enroll RBZ_LIST_VESSELS           "rbw-rvl" "${z_mod}" "rbrv_list"      ""        "List vessel regimes"
  buz_enroll RBZ_RENDER_VESSEL          "rbw-rvr" "${z_mod}" "rbrv_render"    "param1"  "Render vessel regime"
  buz_enroll RBZ_VALIDATE_VESSEL        "rbw-rvv" "${z_mod}" "rbrv_validate"  "param1"  "Validate vessel regime"
  z_mod="rbrr_cli.sh"
  buz_enroll RBZ_RENDER_REPO            "rbw-rrr" "${z_mod}" "rbrr_render"    ""  "Render repo regime"
  buz_enroll RBZ_VALIDATE_REPO          "rbw-rrv" "${z_mod}" "rbrr_validate"  ""  "Validate repo regime"
  z_mod="rbrp_cli.sh"
  buz_enroll RBZ_RENDER_PAYOR           "rbw-rpr" "${z_mod}" "rbrp_render"    ""  "Render payor regime"
  buz_enroll RBZ_VALIDATE_PAYOR         "rbw-rpv" "${z_mod}" "rbrp_validate"  ""  "Validate payor regime"
  z_mod="rbro_cli.sh"
  buz_enroll RBZ_RENDER_OAUTH           "rbw-ror" "${z_mod}" "rbro_render"    ""  "Render OAuth regime"
  buz_enroll RBZ_VALIDATE_OAUTH         "rbw-rov" "${z_mod}" "rbro_validate"  ""  "Validate OAuth regime"
  z_mod="rbrs_cli.sh"
  buz_enroll RBZ_RENDER_STATION         "rbw-rsr" "${z_mod}" "rbrs_render"    ""  "Render station regime"
  buz_enroll RBZ_VALIDATE_STATION       "rbw-rsv" "${z_mod}" "rbrs_validate"  ""  "Validate station regime"
  z_mod="rbra_cli.sh"
  buz_enroll RBZ_RENDER_AUTH            "rbw-rar" "${z_mod}" "rbra_render"    "param1"  "Render auth regime"
  buz_enroll RBZ_VALIDATE_AUTH          "rbw-rav" "${z_mod}" "rbra_validate"  "param1"  "Validate auth regime"
  buz_enroll RBZ_LIST_AUTH              "rbw-ral" "${z_mod}" "rbra_list"      ""        "List auth regimes"

  # Theurge — test infrastructure (rbw-t)
  buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge — Test infrastructure"
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_THEURGE_KLUDGE  "rbw-tK"  "${z_mod}" "rbob_kludge"        "imprint"  "Local kludge build + install hallmark into nameplate"
  buz_enroll RBZ_THEURGE_ORDAIN  "rbw-tO"  "${z_mod}" "rbob_ordain"        "imprint"  "Cloud build + install hallmark into nameplate"
  z_mod="rbq_cli.sh"
  buz_enroll RBZ_QUALIFY_FAST    "rbw-tf"   "${z_mod}" "rbq_qualify_fast"   ""         "Fast qualify: tabtargets, colophons, nameplate health"
  buz_enroll RBZ_QUALIFY_RELEASE "rbw-tr"   "${z_mod}" "rbq_qualify_release" ""        "Release qualify: + shellcheck, full test suite"

  # Handbook — human-facing procedures (rbw-h0 index, rbw-hw/HW* windows)
  buz_group RBZ__GROUP_HANDBOOK   "rbw-HW"  "Handbook — Human-facing procedures"
  z_mod="rbhw_cli.sh"
  buz_enroll RBZ_HANDBOOK_TOP           "rbw-h0"    "${z_mod}" "rbhw_handbook_top"              ""        "Top-level handbook index across all groups"
  buz_enroll RBZ_HANDBOOK_WINDOWS       "rbw-hw"    "${z_mod}" "rbhw_top"                       ""        "Windows test infrastructure orchestrator"
  buz_enroll RBZ_HW_DOCKER_DESKTOP     "rbw-HWdd"  "${z_mod}" "rbhw_docker_desktop"            ""        "Docker Desktop install"
  buz_enroll RBZ_HW_DOCKER_WSL_NATIVE  "rbw-HWdw"  "${z_mod}" "rbhw_docker_wsl_native"         "param1"  "Native dockerd in WSL"
  buz_enroll RBZ_HW_DOCKER_CONTEXT     "rbw-HWdc"  "${z_mod}" "rbhw_docker_context_discipline" ""        "Deterministic daemon selection"

  readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"

  readonly ZRBZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zrbz_healthcheck() {
  zrbz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zrbz_sentinel() {
  test "${ZRBZ_KINDLED:-}" = "1" || buc_die "Module rbz not kindled - call zrbz_kindle first"
}

# eof
