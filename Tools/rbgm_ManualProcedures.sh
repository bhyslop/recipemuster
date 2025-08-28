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
# Recipe Bottle GCP Manual Procedures - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGM_SOURCED:-}" || bcu_die "Module rbgm multiply sourced - check sourcing hierarchy"
ZRBGM_SOURCED=1

######################################################################
# Internal Functions (zrbgm_*)

zrbgm_kindle() {
  test -z "${ZRBGM_KINDLED:-}" || bcu_die "Module rbgm already kindled"

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || bcu_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || bcu_die "RBRR_GCP_PROJECT_ID is empty"

  bcu_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  local z_use_color=0
  if [ -z "${NO_COLOR:-}" ] && [ "${BDU_COLOR:-0}" = "1" ]; then
    z_use_color=1
  fi

  if [ "$z_use_color" = "1" ]; then
    ZRBGM_R="\033[0m"         # Reset
    ZRBGM_S="\033[1;37m"      # Section (bright white)
    ZRBGM_C="\033[36m"        # Command (cyan)
    ZRBGM_W="\033[35m"        # Website (magenta)
    ZRBGM_Y="\033[1;33m"      # Warning (bright yellow)
    ZRBGM_CR="\033[1;31m"     # Critical (bright red)
  else
    ZRBGM_R=""                # No color, or disabled
    ZRBGM_S=""                # No color, or disabled
    ZRBGM_C=""                # No color, or disabled
    ZRBGM_W=""                # No color, or disabled
    ZRBGM_Y=""                # No color, or disabled
    ZRBGM_CR=""               # No color, or disabled
  fi

  # ITCH_LINK_TO_RBL
  ZRBGM_RBRP_FILE="./rbrp.env"
  ZRBGM_RBRR_FILE="./rbrr_RecipeBottleRegimeRepo.sh"


  ZRBGM_PREFIX="${BDU_TEMP_DIR}/rbgm_"
  ZRBGM_LIST_RESPONSE="${ZRBGM_PREFIX}list_response.json"
  ZRBGM_LIST_CODE="${ZRBGM_PREFIX}list_code.txt"
  ZRBGM_CREATE_REQUEST="${ZRBGM_PREFIX}create_request.json"
  ZRBGM_CREATE_RESPONSE="${ZRBGM_PREFIX}create_response.json"
  ZRBGM_CREATE_CODE="${ZRBGM_PREFIX}create_code.txt"
  ZRBGM_DELETE_RESPONSE="${ZRBGM_PREFIX}delete_response.json"
  ZRBGM_DELETE_CODE="${ZRBGM_PREFIX}delete_code.txt"
  ZRBGM_KEY_RESPONSE="${ZRBGM_PREFIX}key_response.json"
  ZRBGM_KEY_CODE="${ZRBGM_PREFIX}key_code.txt"
  ZRBGM_ROLE_RESPONSE="${ZRBGM_PREFIX}role_response.json"
  ZRBGM_ROLE_CODE="${ZRBGM_PREFIX}role_code.txt"
  ZRBGM_REPO_ROLE_RESPONSE="${ZRBGM_PREFIX}repo_role_response.json"
  ZRBGM_REPO_ROLE_CODE="${ZRBGM_PREFIX}repo_role_code.txt"

  ZRBGM_KINDLED=1
}

zrbgm_sentinel() {
  test "${ZRBGM_KINDLED:-}" = "1" || bcu_die "Module rbgm not kindled - call zrbgm_kindle first"
}

zrbgm_show() {
  zrbgm_sentinel
  echo -e "${1:-}"
}

zrbgm_s1()      { zrbgm_show "${ZRBGM_S}${1}${ZRBGM_R}"; }
zrbgm_s2()      { zrbgm_show "${ZRBGM_S}${1}${ZRBGM_R}"; }
zrbgm_s3()      { zrbgm_show "${ZRBGM_S}${1}${ZRBGM_R}"; }

zrbgm_e()       { zrbgm_show "";                                                             }
zrbgm_d()       { zrbgm_show "${1}";                                                         }
zrbgm_dc()      { zrbgm_show "${1}${ZRBGM_C}${2}${ZRBGM_R}";                                 }
zrbgm_dcd()     { zrbgm_show "${1}${ZRBGM_C}${2}${ZRBGM_R}${3}";                             }
zrbgm_dcdm()    { zrbgm_show "${1}${ZRBGM_C}${2}${ZRBGM_R}${3}${ZRBGM_W}${4}${ZRBGM_R}";     }
zrbgm_dm()      { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}";                                 }
zrbgm_dmd()     { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}";                             }
zrbgm_dmdr()    { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_CR}${4}${ZRBGM_R}";    }
zrbgm_dmdm()    { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_W}${4}${ZRBGM_R}";     }
zrbgm_dwdwd()   { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_W}${4}${ZRBGM_R}${5}"; }
zrbgm_dy()      { zrbgm_show "${1}${ZRBGM_Y}${2}${ZRBGM_R}";                                 }

zrbgm_de()      { zrbgm_show "${1}${ZRBGM_CR}${2}${ZRBGM_R}"; }

zrbgm_cmd()     { zrbgm_show "${ZRBGM_C}${1}${ZRBGM_R}"; }
zrbgm_critic()  { zrbgm_show "\n${ZRBGM_CR} CRITICAL SECURITY WARNING: ${1}${ZRBGM_R}\n"; }

zrbgm_dld() {
  local z_default="${1:-}"
  local z_label="${2:-}"
  local z_url="${3:-}"
  local z_coda="${4:-}"
  test -n "${z_label}" || bcu_die "missing label"
  test -n "${z_url}"   || bcu_die "missing url"

  # Blue + underline style, wrapped in OSC 8 hyperlink
  printf '%s\e[34;4m\e]8;;%s\a%s\e]8;;\a\e[0m%s\n' "${z_default}" "${z_url}" "${z_label}" "${z_coda}"
}


######################################################################
# External Functions (rbgm_*)

rbgm_show_payor_establishment() {
  zrbgm_sentinel

  bcu_doc_brief "Display the manual Payor Establishment procedure"
  bcu_doc_shown || return 0

  zrbgm_s1     "Manual Payor Establishment Procedure"
  zrbgm_d      "Recipe Bottle Payor setup requires manual configuration before API operations can proceed."
  zrbgm_e
  zrbgm_s2     "Key:"
  zrbgm_dm     "   Magenta text refers to " "precise words you see on the web page."
  zrbgm_dc     "   Cyan text is " "something you might copy from here."
  zrbgm_d      "   Default text is this color."
  zrbgm_dld    "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, Ctrl + mouse click)"
  zrbgm_e
  zrbgm_s2     "1. Open a text editor on your Payor Regime file:"
  zrbgm_dc     "   1. File found at -> " "${ZRBGM_RBRP_FILE}"
  zrbgm_e
  zrbgm_s2     "2. Create Payor Project:"
  zrbgm_dld    "   1. Open browser to: " "Google Cloud Project Create" "https://console.cloud.google.com/projectcreate"
  zrbgm_d      "   2. Ensure signed in with intended Google account (check top-right avatar)"
  zrbgm_d      "   3. Configure new project:"
  zrbgm_dc     "      - Project name: " "Recipe Bottle Payor"
  zrbgm_dc     "      - Project ID: " "use your RBRP_PAYOR_PROJECT_ID value"
  zrbgm_d      "      - Parent: select your Organization or Folder if present"
  zrbgm_d      "        If your account shows 'No organization', record 'none' for both type and ID"
  zrbgm_dm     "   4. Click " "CREATE"
  zrbgm_dmdr   "   5. If " "The project ID is already taken" " : " "FAIL - choose different RBRP_PAYOR_PROJECT_ID"
  zrbgm_dmd    "   6. Wait for " "Creating project..." " notification to complete"
  zrbgm_e
  zrbgm_s2     "3. Record Parent Resource:"
  zrbgm_dld    "   1. Go to: " "Google Cloud Resource Manager" "https://console.cloud.google.com/cloud-resource-manager"
  zrbgm_dmdm   "   2. To the right of your project, click " "hamburger menu" " -> " "Settings"
  zrbgm_dm     "   3. Expect display of 'Project Info Card' displaying " "Project name, Project ID, Project number"
  zrbgm_dmd    "   4. Update " "${ZRBGM_RBRP_FILE}" " with payor project:"
  zrbgm_dcdm   "          " "RBRP_PAYOR_PROJECT_ID=" " # " "Value from Project ID"
  zrbgm_dmd    "   4. Update " "${ZRBGM_RBRP_FILE}" " with parent info:"
  zrbgm_d      "      If 'Project Info Card' has a clickable field 'Organization':"
  zrbgm_dc     "          " "RBRP_PARENT_TYPE=organization"
  zrbgm_dcdm   "          " "RBRP_PARENT_ID=" " # " "Value from Organization id"
  zrbgm_d      "      else if 'Project Info Card' has a clickable field 'Parent folder':"
  zrbgm_dc     "          " "RBRP_PARENT_TYPE=folder"
  zrbgm_dcdm   "          " "RBRP_PARENT_ID=" " # " "Value from Folder id"
  zrbgm_dm     "      else, place the following in " "${ZRBGM_RBRP_FILE}"
  zrbgm_dc     "          " "RBRP_PARENT_TYPE=none"
  zrbgm_dc     "          " "RBRP_PARENT_ID=none"
  zrbgm_e
  zrbgm_s2     "4. Configure Billing Account:"
  zrbgm_dy     "    " "not sure this is right"
  zrbgm_dld    "   1. Go to: " "Google Cloud Billing" "https://console.cloud.google.com/billing"
  zrbgm_d      "      If no billing accounts exist:"
  zrbgm_dm     "          a. Click " "CREATE ACCOUNT"
  zrbgm_d      "          b. Configure payment method and submit"
  zrbgm_dm     "          c. Copy new " "Account ID" " from table"
  zrbgm_d      "      else if single Open account exists:"
  zrbgm_dm     "          a. Copy the " "Account ID" " value"
  zrbgm_d      "      else if multiple Open accounts exist:"
  zrbgm_d      "          a. Choose account for Recipe Bottle funding"
  zrbgm_dm     "          b. Copy chosen " "Account ID" " value"
  zrbgm_dld    "   5. Go to: " "Google Cloud Billing Projects" "https://console.cloud.google.com/billing/projects"
  zrbgm_dmd    "   6. Find project row with ID matching your " "RBRP_PAYOR_PROJECT_ID" "(not name)"
  zrbgm_dcdm   "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  zrbgm_dm     "   7. Save your " "${ZRBGM_RBRP_FILE}" " and re-display this procedure."
  zrbgm_e
  zrbgm_s2     "4. Confirm your settings: if below are not correct, re-display: it enables following links"
  zrbgm_dm     "          " "RBRP_PAYOR_PROJECT_ID=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "          " "RBRP_PARENT_TYPE=${RBRP_PARENT_TYPE}"
  zrbgm_dm     "          " "RBRP_PARENT_ID=${RBRP_PARENT_ID}"
  zrbgm_dm     "          " "RBRP_BILLING_ACCOUNT_ID=${RBRP_BILLING_ACCOUNT_ID}"
  zrbgm_e
  zrbgm_s2     "5. Link Billing to Payor Project:"
  zrbgm_dy     "   " "my free trial seemed to have autolinked"
  zrbgm_d      "   Now link your billing account to the newly created project."
  zrbgm_dld    "   Go to: " "Google Cloud Billing Linked Accounts" "https://console.cloud.google.com/billing/linkedaccount"
  zrbgm_d      "   1. Select the correct billing account:"
  zrbgm_dm     "      - Use the dropdown at top to select billing account " "matching RBRP_BILLING_ACCOUNT_ID"
  zrbgm_d      "      - The dropdown shows account name and ID (format: Account Name - XXXXXX-XXXXXX-XXXXXX)"
  zrbgm_d      "      - Choose the account you configured in step 4"
  zrbgm_dm     "   2. Click " "LINK A PROJECT"
  zrbgm_dmd    "   3. Select project: " "use your RBRP_PAYOR_PROJECT_ID" " from dropdown"
  zrbgm_d      "      - Type the project name or ID to filter the list if needed"
  zrbgm_dm     "   4. Click " "SET ACCOUNT"
  zrbgm_dmd    "   5. Verify project appears in " "Linked projects" " table with 'Active' status"
  zrbgm_d      "   6. If linking fails, check that:"
  zrbgm_d      "      - Project exists and matches RBRP_PAYOR_PROJECT_ID"
  zrbgm_d      "      - Billing account is active and you have permissions"
  zrbgm_e
  zrbgm_s2     "6. Create Payor Service Account:"
  zrbgm_dld    "   Go to: " "Google Cloud Service Accounts for ${RBRP_PAYOR_PROJECT_ID}" "https://console.cloud.google.com/iam-admin/serviceaccounts?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dmd    "   1. Ensure project with name corresponding to " "RBRP_PAYOR_PROJECT_ID" " in top project picker"
  zrbgm_dm     "   2. Click " "+ CREATE SERVICE ACCOUNT"
  zrbgm_d      "   3. Service account details:"
  zrbgm_dc     "      - Service account name: " "${RBGC_PAYOR_ROLE}"
  zrbgm_dc     "      - Service account ID: " "${RBGC_PAYOR_ROLE}"
  zrbgm_dc     "      - Description: " "Payor role for billing and project lifecycle operations"
  zrbgm_dm     "   4. Click " "CREATE AND CONTINUE"
  zrbgm_dmd    "   5. Grant roles - select from dropdown " "Select a role" ":"
  zrbgm_dm     "      - Select from right side -> " "Owner"
  zrbgm_dm     "   6. Click " "CONTINUE"
  zrbgm_dm     "   7. Skip Principals with access and click " "DONE"
  zrbgm_e
  zrbgm_s2     "7. Grant Billing Permissions to Payor:"
  zrbgm_dld    "   Return to: " "Google Cloud Billing" "https://console.cloud.google.com/billing"
  zrbgm_dmd    "   1. Select billing account matching " "RBRP_BILLING_ACCOUNT_ID" " from list"
  zrbgm_dm     "   2. Left sidebar -> " "Account Management"
  zrbgm_dm     "   3. Assure RHS info panel open; if you see " "Show info panel" " then click it"
  zrbgm_dm     "   4. From info panel click " "+ ADD PRINCIPAL"
  zrbgm_d      "   5. Configure IAM binding:"
  zrbgm_dy     "    "  "TODO HERE"
  zrbgm_dc     "      - New principals: " "${RBGC_PAYOR_ROLE}@${RBRP_PAYOR_PROJECT_ID}.iam.gserviceaccount.com"
  zrbgm_dc     "      - Role: " "Billing Admin"
  zrbgm_dm     "   6. Click " "SAVE"
  zrbgm_e
  zrbgm_s2     "8. Generate Payor Service Account Key:"
  zrbgm_dld    "   Return to: " "Google Cloud Service Accounts for ${RBRP_PAYOR_PROJECT_ID}" "https://console.cloud.google.com/iam-admin/serviceaccounts?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dmd    "   1. Ensure project " "matches RBRP_PAYOR_PROJECT_ID" " in top dropdown"
  zrbgm_dm     "   2. Click on the " "${RBGC_PAYOR_ROLE}" " service account email"
  zrbgm_dm     "   3. Top tabs -> " "KEYS"
  zrbgm_dmdm   "   4. Click " "ADD KEY" " -> " "Create new key"
  zrbgm_dmd    "   5. Key type: " "JSON" " (should be selected)"
  zrbgm_dm     "   6. Click " "CREATE"
  zrbgm_e
  zrbgm_dm     "Browser downloads: " "[RBRP_PAYOR_PROJECT_ID]-[random].json"
  zrbgm_dmd    "   7. Click " "CLOSE" " on download confirmation"
  zrbgm_e
  zrbgm_s2     "9. Configure Payor RBRA File:"
  zrbgm_d      "   The downloaded JSON key needs to be converted to RBRA format."
  zrbgm_d      "   Use the appropriate Recipe Bottle tool to generate the Payor RBRA file"
  # ITCH_THIS_OUGHT_TO_SAY_RIGHT_BASH_COMMAND
  zrbgm_d      "   from the downloaded JSON key file."
  zrbgm_e
  zrbgm_dy     " " "Manual setup complete. You can now run Recipe Bottle Payor API operations."
  zrbgm_d      "Verify configuration by testing Payor operations before proceeding to depot creation."

  bcu_success "Payor establishment procedure displayed"
}

rbgm_LEGACY_setup_admin() { # ITCH_DELETE_THIS_AFTER_ABOVE_TESTED
  zrbgm_sentinel

  bcu_doc_brief "Display the manual GCP admin setup procedure"
  bcu_doc_shown || return 0

  zrbgm_s1     "# Google Cloud Platform Setup"
  zrbgm_s2     "## Overview"
  zrbgm_d      "Bootstrap GCP infrastructure by creating an admin service account with Project Owner privileges."
  zrbgm_d      "The admin account will manage operational service accounts and infrastructure configuration."
  zrbgm_s2     "## Prerequisites"
  zrbgm_d      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbgm_d      "- Email address not already associated with GCP"
  zrbgm_e
  zrbgm_d      "---"
  zrbgm_s1     "Manual Admin Setup Procedure"
  zrbgm_d      "Recipe Bottle setup requires a manual bootstrap procedure to enable admin control"
  zrbgm_e
  zrbgm_dc     "Open a web browser to " "${RBGC_SIGNUP_URL}"

  zrbgm_critic "This procedure is for PERSONAL Google accounts only."
  zrbgm_d      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbgm_d      "you must follow your IT/admin process to create projects, attach billing,"
  zrbgm_d      "and assign permissions - those steps are NOT covered here."
  zrbgm_e
  zrbgm_s2     "1. Establish Account:"
  zrbgm_dc     "   Open a browser to: " "${RBGC_SIGNUP_URL}"
  zrbgm_dm     "   1. Click -> " "Get started for free"
  zrbgm_d      "   2. Sign in with your Google account or create a new one"
  zrbgm_d      "   3. Provide:"
  zrbgm_d      "      - Country"
  zrbgm_dm     "      - Organization type: " "Individual"
  zrbgm_d      "      - Credit card (verification only)"
  zrbgm_dm     "   4. Accept terms -> " "Start my free trial"
  zrbgm_d      "   5. Expect Google Cloud Console to open"
  zrbgm_dmd    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbgm_e
  local z_configure_pid_step="2. Configure Project ID and Region"
  zrbgm_s2     "${z_configure_pid_step}:"
  zrbgm_d      "   Before creating the project, choose a unique Project ID."
  zrbgm_dc     "   1. Edit your RBRR configuration file: " "${ZRBGM_RBRR_FILE}"
  zrbgm_d      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbgm_d      "      - Must be globally unique across all GCP"
  zrbgm_d      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbgm_d      "      - Cannot start/end with hyphen"
  zrbgm_d      "   3. Set RBRR_GCP_REGION based on your location (see project documentation)"
  zrbgm_d      "   4. Save the file before proceeding"
  zrbgm_e
  zrbgm_s2     "3. Create New Project:"
  zrbgm_dc     "   Go directly to: " "${RBGC_CONSOLE_URL}"
  zrbgm_d      "   Sign in with the same Google account you just set up"
  zrbgm_d      "   1. Open the Google Cloud Console main menu:"
  zrbgm_dmd    "      - Click the " "->" " hamburger menu in the top-left corner"
  zrbgm_dm     "      - Scroll down to " "IAM & Admin"
  zrbgm_dm     "      - Click -> " "Manage resources"
  zrbgm_d      "        (Alternatively, type 'manage resources' in the top search bar and press Enter)"
  zrbgm_dm     "   2. On the Manage resources page, click -> " "CREATE PROJECT"
  zrbgm_d      "   3. Configure:"
  zrbgm_dc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbgm_dm     "      - Organization: " "No organization"
  zrbgm_dm     "   4. Click " "CREATE"
  zrbgm_dmdr   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbgm_dmd    "   6. If popup, wait for notification -> " "Creating project..." " to complete"
  zrbgm_e
  zrbgm_s2     "4. Create the Admin Service Account:"
  zrbgm_dwdwd  "   1. Ensure project " "${RBRR_GCP_PROJECT_ID}" " is selected in the top dropdown (button with hovertext " "Open project picker (Ctrl O)" ")"
  zrbgm_dmdm   "   2. Left sidebar -> " "IAM & Admin" " -> " "Service Accounts"
  zrbgm_d      "   3. If prompted about APIs:"
  zrbgm_dm     "      1. click -> " "Enable API"
  zrbgm_d      "          TODO: This step is brittle - enabling IAM API may happen automatically or be blocked by org policy."
  zrbgm_dm     "      2. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbgm_dm     "   4. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbgm_d      "   5. Service account details:"
  zrbgm_dc     "      - Service account name: " "${RBGC_ADMIN_ROLE}"
  zrbgm_dmd    "      - Service account ID: (auto-fills as " "${RBGC_ADMIN_ROLE}" ")"
  zrbgm_dc     "      - Description: " "Admin account for infrastructure management"
  zrbgm_dm     "   6. Click -> " "Create and continue"
  zrbgm_dmdm   "   7. At " "Permissions (optional)" " pick dropdown " "Select a role"
  zrbgm_dc     "      - In filter box, type: " "owner"
  zrbgm_dmdm   "      - Select: " "Basic" " -> " "Owner"
  zrbgm_dm     "   8. Click -> " "Continue"
  zrbgm_dmdm   "   9. Skip " "Principals with access" " by clicking -> " "Done"
  zrbgm_e
  zrbgm_s2     "7. Generate Service Account Key:"
  zrbgm_d      "From service accounts list:"
  zrbgm_dm     "   1. Click on text of " "${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  zrbgm_dm     "   2. Top tabs -> " "Keys"
  zrbgm_dmdm   "   3. Click " "Add key" " -> " "Create new key"
  zrbgm_dmd    "   4. Key type: " "JSON" " (should be selected)"
  zrbgm_dm     "   5. Click " "Create"
  zrbgm_e
  zrbgm_dm     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbgm_dmd    "   6. Click " "CLOSE" " on download confirmation"
  zrbgm_e
  zrbgm_s2     "8. Configure Local Environment:"
  zrbgm_d      "Browser downloaded key.  Run the command to ingest it into your ADMIN RBRA file."
  zrbgm_e

  bcu_success "Manual setup procedure displayed"
}


# eof
