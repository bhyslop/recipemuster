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
  buz_enroll RBZ_INVEST_RETRIEVER        "rbw-arI" "${z_mod}" "rbgg_invest_retriever"      "param1"  "Invest a Retriever service account for an identity"
  buz_enroll RBZ_INVEST_DIRECTOR         "rbw-adI" "${z_mod}" "rbgg_invest_director"       "param1"  "Invest a Director service account for an identity"
  buz_enroll RBZ_DIVEST_RETRIEVER        "rbw-arD" "${z_mod}" "rbgg_divest_retriever"      "param1"  "Divest a Retriever service account by identity"
  buz_enroll RBZ_DIVEST_DIRECTOR         "rbw-adD" "${z_mod}" "rbgg_divest_director"       "param1"  "Divest a Director service account by identity"
  buz_enroll RBZ_ROSTER_RETRIEVERS       "rbw-arr" "${z_mod}" "rbgg_roster_retrievers"     ""        "Roster Retriever service accounts (emit per-identity fact files)"
  buz_enroll RBZ_ROSTER_DIRECTORS        "rbw-adr" "${z_mod}" "rbgg_roster_directors"      ""        "Roster Director service accounts (emit per-identity fact files)"

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
  buz_enroll RBZ_CRUCIBLE_KLUDGE_SENTRY "rbw-cKS" "${z_mod}" "rbob_kludge_sentry" "param1" "Kludge sentry vessel and drive hallmark into nameplate"

  # Depot — GCP project infrastructure (rbw-d, UPPER=mutates, lower=read)
  buz_group RBZ__GROUP_DEPOT      "rbw-d"   "Depot — GCP project infrastructure"
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_LEVY_DEPOT             "rbw-dL"  "${z_mod}" "rbgp_depot_levy"   ""  "Provision GCP depot project"
  buz_enroll RBZ_UNMAKE_DEPOT           "rbw-dU"  "${z_mod}" "rbgp_depot_unmake"  ""  "Permanently remove a depot"
  buz_enroll RBZ_LIST_DEPOT             "rbw-dl"  "${z_mod}" "rbgp_depot_list"    ""  "List all active depots"
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ENSHRINE_VESSEL        "rbw-dE"  "${z_mod}" "rbfd_enshrine"      "param1"  "Enshrine upstream base images to GAR via Cloud Build"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_INSCRIBE_RELIQUARY     "rbw-dI"  "${z_mod}" "rbfl_inscribe"      ""  "Create reliquary: mirror tool images from upstream to GAR"
  buz_enroll RBZ_YOKE_RELIQUARY         "rbw-dY"  "${z_mod}" "rbfl_yoke"          "param1"  "Yoke a reliquary stamp into an ordain-path vessel's rbrv.env"

  # Guide — human-directed procedures (rbw-g)
  buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Guide — Human-directed procedures"
  z_mod="rbgp_cli.sh"
  buz_enroll RBZ_PAYOR_INSTALL          "rbw-gPI" "${z_mod}" "rbgp_payor_install"   ""  "Ingest OAuth credentials from JSON key file"
  z_mod="rbh0/rbhp0_cli.sh"
  buz_enroll RBZ_PAYOR_ESTABLISH        "rbw-gPE" "${z_mod}" "rbhp_establish"       ""  "Guided Manor establishment — GCP project + OAuth consent screen"
  buz_enroll RBZ_PAYOR_REFRESH          "rbw-gPR" "${z_mod}" "rbhp_refresh"         ""  "Refresh expired OAuth tokens"
  buz_enroll RBZ_QUOTA_BUILD            "rbw-gq"  "${z_mod}" "rbhp_quota_build"     ""  "Display Cloud Build capacity review procedure"
  # Onboarding — handbook tracks (rbw-o terminal + rbw-O* family, see ₣A6 paddock)
  buz_group RBZ__GROUP_ONBOARDING "rbw-o"   "Onboarding — Handbook restart"
  z_mod="rbh0/rbho0_cli.sh"
  buz_enroll RBZ_ONBOARD_START_HERE    "rbw-o"   "${z_mod}" "rbho_start_here"            ""  "Onboarding start — probe-aware menu into handbook tracks"
  buz_enroll RBZ_ONBOARD_CRASH_COURSE  "rbw-Occ" "${z_mod}" "rbho_crash_course"          ""  "Crash Course — universal prerequisite: tabtargets, regimes, diagnostic failure"
  buz_enroll RBZ_ONBOARD_CRED_RETRIEVER "rbw-Ocr" "${z_mod}" "rbho_credential_retriever" ""  "Install retriever credentials — place RBRA key file"
  buz_enroll RBZ_ONBOARD_CRED_DIRECTOR  "rbw-Ocd" "${z_mod}" "rbho_credential_director"  ""  "Install director credentials — place RBRA key file"
  buz_enroll RBZ_ONBOARD_FIRST_CRUCIBLE "rbw-Ofc" "${z_mod}" "rbho_first_crucible"       ""  "Start a Crucible using local builds — kludge, charge, rack"
  buz_enroll RBZ_ONBOARD_TADMOR_SECURITY "rbw-Ots" "${z_mod}" "rbho_tadmor_security"     ""  "Verify Crucible containment under attack — charge tadmor and run the adversarial suite"
  buz_enroll RBZ_ONBOARD_DIR_FIRST_BUILD "rbw-Odf" "${z_mod}" "rbho_director_first_build" "" "Your First Cloud Build — inscribe, conjure, tour, summon, abjure"
  buz_enroll RBZ_ONBOARD_DIR_AIRGAP     "rbw-Oda" "${z_mod}" "rbho_director_airgap"      ""  "Airgap Cloud Build — enshrine, conjure base, conjure airgap, charge moriah, compare plumb"
  buz_enroll RBZ_ONBOARD_DIR_BIND       "rbw-Odb" "${z_mod}" "rbho_director_bind"        ""  "Bind Cloud Build — pin upstream image by digest, mode-mixture pluml Crucible"
  buz_enroll RBZ_ONBOARD_DIR_GRAFT      "rbw-Odg" "${z_mod}" "rbho_director_graft"       ""  "Graft Cloud Build — push locally-built image, inspect GRAFTED Vouch verdict"
  buz_enroll RBZ_ONBOARD_PAYOR_HB      "rbw-Op"  "${z_mod}" "rbho_payor_handbook"       ""  "Payor — establish a Manor and provision the Depot"
  buz_enroll RBZ_ONBOARD_GOVERNOR_HB   "rbw-Og"  "${z_mod}" "rbho_governor_handbook"    ""  "Governor — administer service accounts for directors and retrievers"

  # Foundry — registry artifact lifecycle (rbw-f, UPPER=mutates GAR, lower=read/local)
  buz_group RBZ__GROUP_FOUNDRY    "rbw-f"   "Foundry — Registry artifact lifecycle"
  z_mod="rbfd_cli.sh"
  buz_enroll RBZ_ORDAIN_HALLMARK        "rbw-fO"  "${z_mod}" "rbfd_ordain"          "param1"  "Ordain hallmark: conjure, bind, or graft based on vessel mode"
  buz_enroll RBZ_KLUDGE_VESSEL          "rbw-fk"  "${z_mod}" "rbfd_kludge"          "param1"  "Kludge a vessel image locally for development"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_ABJURE_HALLMARK        "rbw-fA"  "${z_mod}" "rbfl_abjure"          "param1"  "Abjure a hallmark (delete artifacts from GAR)"
  buz_enroll RBZ_TALLY_HALLMARKS        "rbw-ft"  "${z_mod}" "rbfl_tally"           ""  "Tally hallmarks by health state"
  z_mod="rbfv_cli.sh"
  buz_enroll RBZ_VOUCH_HALLMARKS        "rbw-fV"  "${z_mod}" "rbfv_batch_vouch"     ""  "Mode-aware vouch: SLSA (conjure), digest-pin (bind), GRAFTED (graft)"
  z_mod="rbfr_cli.sh"
  buz_enroll RBZ_SUMMON_HALLMARK        "rbw-fs"  "${z_mod}" "rbfr_summon"          "param1"  "Summon vouched hallmark image locally"
  z_mod="rbfc_cli.sh"
  buz_enroll RBZ_PLUMB_FULL             "rbw-fpf" "${z_mod}" "rbfc_plumb_full"      "param1"  "Full provenance display (SBOM, build info, Dockerfile)"
  buz_enroll RBZ_PLUMB_COMPACT          "rbw-fpc" "${z_mod}" "rbfc_plumb_compact"   "param1"  "Compact provenance summary"

  # Ifrit — attack binary (rbw-I)
  buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit — Attack binary"
  z_mod="rbob_cli.sh"
  buz_enroll RBZ_BOTTLE_IFRIT   "rbw-Ic"  "${z_mod}" "rbob_ifrit_client"  "imprint"  "Launch Claude Code inside a running bottle for escape testing"
  buz_enroll RBZ_BOTTLE_SORTIE  "rbw-Is"  "${z_mod}" "rbob_ifrit_sortie"  "imprint"  "Run automated security test scripts inside the bottle"

  # Image — container image operations (rbw-i, UPPER=mutates, lower=read)
  # Three-domain symmetric: hallmarks (h), reliquaries (r), enshrinements (e).
  # Verbs: rekon (member-list), audit (catalog-list), Jettison (delete), wrest (pull).
  buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Image — Container image operations"
  z_mod="rbfl_cli.sh"
  buz_enroll RBZ_REKON_HALLMARK         "rbw-irh" "${z_mod}" "rbfl_rekon_hallmark"        "param1"  "List ark basenames present under a hallmark's GAR subtree"
  buz_enroll RBZ_REKON_RELIQUARY        "rbw-irr" "${z_mod}" "rbfl_rekon_reliquary"       "param1"  "List tool images present under a reliquary stamp's GAR subtree"
  buz_enroll RBZ_AUDIT_HALLMARKS        "rbw-iah" "${z_mod}" "rbfl_audit_hallmarks"       ""  "Audit hallmarks — list all hallmark identifiers"
  buz_enroll RBZ_AUDIT_RELIQUARIES      "rbw-iar" "${z_mod}" "rbfl_audit_reliquaries"     ""  "Audit reliquaries — list all reliquary stamps"
  buz_enroll RBZ_AUDIT_ENSHRINEMENTS    "rbw-iae" "${z_mod}" "rbfl_audit_enshrinements"   ""  "Audit enshrinements — list all enshrined base anchors"
  buz_enroll RBZ_WREST_HALLMARK_IMAGE   "rbw-iwh" "${z_mod}" "rbfl_wrest"                 "param1"  "Wrest a hallmark image from registry"
  buz_enroll RBZ_WREST_RELIQUARY_IMAGE  "rbw-iwr" "${z_mod}" "rbfl_wrest"                 "param1"  "Wrest a reliquary tool image from registry"
  buz_enroll RBZ_WREST_ENSHRINED_IMAGE  "rbw-iwe" "${z_mod}" "rbfl_wrest"                 "param1"  "Wrest an enshrined base image from registry"
  buz_enroll RBZ_JETTISON_HALLMARK_IMAGE  "rbw-iJh" "${z_mod}" "rbfl_jettison"            "param1"  "Jettison a hallmark image tag from registry"
  buz_enroll RBZ_JETTISON_RELIQUARY_IMAGE "rbw-iJr" "${z_mod}" "rbfl_jettison"            "param1"  "Jettison a reliquary tool image from registry"
  buz_enroll RBZ_JETTISON_ENSHRINEMENT    "rbw-iJe" "${z_mod}" "rbfl_jettison"            "param1"  "Jettison an enshrinement from registry"

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
  buz_enroll RBZ_QUALIFY_FAST     "rbw-tf"   "${z_mod}" "rbq_qualify_fast"     ""        "Fast qualify: tabtargets, colophons, nameplate health"
  buz_enroll RBZ_QUALIFY_RELEASE  "rbw-tr"   "${z_mod}" "rbq_qualify_release"  ""        "Release qualify: + shellcheck, full test suite"
  buz_enroll RBZ_QUALIFY_PRISTINE "rbw-tP"   "${z_mod}" "rbq_qualify_pristine" ""        "Pristine qualify: gauntlet test suite (release gate)"

  # Handbook — human-facing procedures (rbw-h0 index, rbw-hw/HW* windows)
  buz_group RBZ__GROUP_HANDBOOK   "rbw-HW"  "Handbook — Human-facing procedures"
  z_mod="rbh0/rbhw0_cli.sh"
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
