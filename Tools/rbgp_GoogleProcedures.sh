#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Google Procedures - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGP_SOURCED:-}" || bcu_die "Module rbgp multiply sourced - check sourcing hierarchy"
ZRBGP_SOURCED=1

######################################################################
# Internal Functions (zrbgp_*)

zrbgp_kindle() {
  test -z "${ZRBGP_KINDLED:-}" || bcu_die "Module rbgp already kindled"

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || bcu_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || bcu_die "RBRR_GCP_PROJECT_ID is empty"

  bcu_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  local z_use_color=0
  if [ -z "${NO_COLOR:-}" ] && [ "${BDU_COLOR:-0}" = "1" ]; then
    z_use_color=1
  fi

  if [ "$z_use_color" = "1" ]; then
    ZRBGP_R="\033[0m"         # Reset
    ZRBGP_S="\033[1;37m"      # Section (bright white)
    ZRBGP_C="\033[36m"        # Command (cyan)
    ZRBGP_W="\033[35m"        # Website (magenta)
    ZRBGP_WN="\033[1;33m"     # Warning (bright yellow)
    ZRBGP_CR="\033[1;31m"     # Critical (bright red)
  else
    ZRBGP_R=""                # No color, or disabled
    ZRBGP_S=""                # No color, or disabled
    ZRBGP_C=""                # No color, or disabled
    ZRBGP_W=""                # No color, or disabled
    ZRBGP_WN=""               # No color, or disabled
    ZRBGP_CR=""               # No color, or disabled
  fi

  ZRBGP_RBRR_FILE="./rbrr_RecipeBottleRegimeRepo.sh"

  ZRBGP_PREFIX="${BDU_TEMP_DIR}/rbgp_"
  ZRBGP_LIST_RESPONSE="${ZRBGP_PREFIX}list_response.json"
  ZRBGP_LIST_CODE="${ZRBGP_PREFIX}list_code.txt"
  ZRBGP_CREATE_REQUEST="${ZRBGP_PREFIX}create_request.json"
  ZRBGP_CREATE_RESPONSE="${ZRBGP_PREFIX}create_response.json"
  ZRBGP_CREATE_CODE="${ZRBGP_PREFIX}create_code.txt"
  ZRBGP_DELETE_RESPONSE="${ZRBGP_PREFIX}delete_response.json"
  ZRBGP_DELETE_CODE="${ZRBGP_PREFIX}delete_code.txt"
  ZRBGP_KEY_RESPONSE="${ZRBGP_PREFIX}key_response.json"
  ZRBGP_KEY_CODE="${ZRBGP_PREFIX}key_code.txt"
  ZRBGP_ROLE_RESPONSE="${ZRBGP_PREFIX}role_response.json"
  ZRBGP_ROLE_CODE="${ZRBGP_PREFIX}role_code.txt"
  ZRBGP_REPO_ROLE_RESPONSE="${ZRBGP_PREFIX}repo_role_response.json"
  ZRBGP_REPO_ROLE_CODE="${ZRBGP_PREFIX}repo_role_code.txt"

  ZRBGP_KINDLED=1
}

zrbgp_sentinel() {
  test "${ZRBGP_KINDLED:-}" = "1" || bcu_die "Module rbgp not kindled - call zrbgp_kindle first"
}

zrbgp_show() {
  zrbgp_sentinel
  echo -e "${1:-}"
}

zrbgp_s1()      { zrbgp_show "${ZRBGP_S}${1}${ZRBGP_R}"; }
zrbgp_s2()      { zrbgp_show "${ZRBGP_S}${1}${ZRBGP_R}"; }
zrbgp_s3()      { zrbgp_show "${ZRBGP_S}${1}${ZRBGP_R}"; }

zrbgp_e()       { zrbgp_show "";                                                             }
zrbgp_n()       { zrbgp_show "${1}";                                                         }
zrbgp_nc()      { zrbgp_show "${1}${ZRBGP_C}${2}${ZRBGP_R}";                                 }
zrbgp_ncn()     { zrbgp_show "${1}${ZRBGP_C}${2}${ZRBGP_R}${3}";                             }
zrbgp_nw()      { zrbgp_show "${1}${ZRBGP_W}${2}${ZRBGP_R}";                                 }
zrbgp_nwn()     { zrbgp_show "${1}${ZRBGP_W}${2}${ZRBGP_R}${3}";                             }
zrbgp_nwne()    { zrbgp_show "${1}${ZRBGP_W}${2}${ZRBGP_R}${3}${ZRBGP_CR}${4}${ZRBGP_R}";    }
zrbgp_nwnw()    { zrbgp_show "${1}${ZRBGP_W}${2}${ZRBGP_R}${3}${ZRBGP_W}${4}${ZRBGP_R}";     }
zrbgp_nwnwn()   { zrbgp_show "${1}${ZRBGP_W}${2}${ZRBGP_R}${3}${ZRBGP_W}${4}${ZRBGP_R}${5}"; }

zrbgp_ne()      { zrbgp_show "${1}${ZRBGP_CR}${2}${ZRBGP_R}"; }

zrbgp_cmd()     { zrbgp_show "${ZRBGP_C}${1}${ZRBGP_R}"; }
zrbgp_warning() { zrbgp_show "\n${ZRBGP_WN}  WARNING: ${1}${ZRBGP_R}\n"; }
zrbgp_critic()  { zrbgp_show "\n${ZRBGP_CR} CRITICAL SECURITY WARNING: ${1}${ZRBGP_R}\n"; }


######################################################################
# External Functions (rbgp_*)

rbgp_show_setup() {
  zrbgp_sentinel

  bcu_doc_brief "Display the manual GCP admin setup procedure"
  bcu_doc_shown || return 0

  zrbgp_s1     "# Google Cloud Platform Setup"
  zrbgp_s2     "## Overview"
  zrbgp_n      "Bootstrap GCP infrastructure by creating an admin service account with Project Owner privileges."
  zrbgp_n      "The admin account will manage operational service accounts and infrastructure configuration."
  zrbgp_s2     "## Prerequisites"
  zrbgp_n      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbgp_n      "- Email address not already associated with GCP"
  zrbgp_e
  zrbgp_n      "---"
  zrbgp_s1     "Manual Admin Setup Procedure"
  zrbgp_n      "Recipe Bottle setup requires a manual bootstrap procedure to enable admin control"
  zrbgp_e
  zrbgp_nc     "Open a web browser to " "${RBGC_SIGNUP_URL}"

  zrbgp_critic "This procedure is for PERSONAL Google accounts only."
  zrbgp_n      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbgp_n      "you must follow your IT/admin process to create projects, attach billing,"
  zrbgp_n      "and assign permissions - those steps are NOT covered here."
  zrbgp_e
  zrbgp_s2     "1. Establish Account:"
  zrbgp_nc     "   Open a browser to: " "${RBGC_SIGNUP_URL}"
  zrbgp_nw     "   1. Click -> " "Get started for free"
  zrbgp_n      "   2. Sign in with your Google account or create a new one"
  zrbgp_n      "   3. Provide:"
  zrbgp_n      "      - Country"
  zrbgp_nw     "      - Organization type: " "Individual"
  zrbgp_n      "      - Credit card (verification only)"
  zrbgp_nw     "   4. Accept terms -> " "Start my free trial"
  zrbgp_n      "   5. Expect Google Cloud Console to open"
  zrbgp_nwn    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbgp_e
  local z_configure_pid_step="2. Configure Project ID and Region"
  zrbgp_s2     "${z_configure_pid_step}:"
  zrbgp_n      "   Before creating the project, choose a unique Project ID."
  zrbgp_nc     "   1. Edit your RBRR configuration file: " "${ZRBGP_RBRR_FILE}"
  zrbgp_n      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbgp_n      "      - Must be globally unique across all GCP"
  zrbgp_n      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbgp_n      "      - Cannot start/end with hyphen"
  zrbgp_n      "   3. Set RBRR_GCP_REGION based on your location (see project documentation)"
  zrbgp_n      "   4. Save the file before proceeding"
  zrbgp_e
  zrbgp_s2     "3. Create New Project:"
  zrbgp_nc     "   Go directly to: " "${RBGC_CONSOLE_URL}"
  zrbgp_n      "   Sign in with the same Google account you just set up"
  zrbgp_n      "   1. Open the Google Cloud Console main menu:"
  zrbgp_nwn    "      - Click the " "->" " hamburger menu in the top-left corner"
  zrbgp_nw     "      - Scroll down to " "IAM & Admin"
  zrbgp_nw     "      - Click -> " "Manage resources"
  zrbgp_n      "        (Alternatively, type 'manage resources' in the top search bar and press Enter)"
  zrbgp_nw     "   2. On the Manage resources page, click -> " "CREATE PROJECT"
  zrbgp_n      "   3. Configure:"
  zrbgp_nc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbgp_nw     "      - Organization: " "No organization"
  zrbgp_nw     "   4. Click " "CREATE"
  zrbgp_nwne   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbgp_nwn    "   6. Wait for notification -> " "Creating project..." " to complete"
  zrbgp_n      "   7. Select project from dropdown when ready"
  zrbgp_e
  zrbgp_s2     "4. Create the Admin Service Account:"
  zrbgp_nwnwn  "   1. Ensure project " "${RBRR_GCP_PROJECT_ID}" " is selected in the top dropdown (button with hovertext " "Open project picker (Ctrl O)" ")"
  zrbgp_nwnw   "   2. Left sidebar -> " "IAM & Admin" " -> " "Service Accounts"
  zrbgp_n      "   3. If prompted about APIs:"
  zrbgp_nw     "      1. click -> " "Enable API"
  zrbgp_n      "          TODO: This step is brittle - enabling IAM API may happen automatically or be blocked by org policy."
  zrbgp_nw     "      2. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbgp_nw     "   4. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbgp_n      "   5. Service account details:"
  zrbgp_nc     "      - Service account name: " "${RBGC_ADMIN_ROLE}"
  zrbgp_nwn    "      - Service account ID: (auto-fills as " "${RBGC_ADMIN_ROLE}" ")"
  zrbgp_nc     "      - Description: " "Admin account for infrastructure management"
  zrbgp_nw     "   6. Click -> " "Create and continue"
  zrbgp_nwnw   "   7. At " "Permissions (optional)" " pick dropdown " "Select a role"
  zrbgp_nc     "      - In filter box, type: " "owner"
  zrbgp_nwnw   "      - Select: " "Basic" " -> " "Owner"
  zrbgp_nw     "   8. Click -> " "Continue"
  zrbgp_nwnw   "   9. Skip " "Principals with access" " by clicking -> " "Done"
  zrbgp_e
  zrbgp_s2     "7. Generate Service Account Key:"
  zrbgp_n      "From service accounts list:"
  zrbgp_nw     "   1. Click on text of " "${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  zrbgp_nw     "   2. Top tabs -> " "Keys"
  zrbgp_nwnw   "   3. Click " "Add key" " -> " "Create new key"
  zrbgp_nwn    "   4. Key type: " "JSON" " (should be selected)"
  zrbgp_nw     "   5. Click " "Create"
  zrbgp_e
  zrbgp_nw     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbgp_nwn    "   6. Click " "CLOSE" " on download confirmation"
  zrbgp_e
  zrbgp_s2     "8. Configure Local Environment:"
  zrbgp_n      "Browser downloaded key.  Run the command to ingest it into your ADMIN RBRA file."
  zrbgp_e

  bcu_success "Manual setup procedure displayed"
}


# eof
