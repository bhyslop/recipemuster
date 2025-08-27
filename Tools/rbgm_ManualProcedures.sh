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
    ZRBGM_WN="\033[1;33m"     # Warning (bright yellow)
    ZRBGM_CR="\033[1;31m"     # Critical (bright red)
  else
    ZRBGM_R=""                # No color, or disabled
    ZRBGM_S=""                # No color, or disabled
    ZRBGM_C=""                # No color, or disabled
    ZRBGM_W=""                # No color, or disabled
    ZRBGM_WN=""               # No color, or disabled
    ZRBGM_CR=""               # No color, or disabled
  fi

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
zrbgm_n()       { zrbgm_show "${1}";                                                         }
zrbgm_nc()      { zrbgm_show "${1}${ZRBGM_C}${2}${ZRBGM_R}";                                 }
zrbgm_ncn()     { zrbgm_show "${1}${ZRBGM_C}${2}${ZRBGM_R}${3}";                             }
zrbgm_nw()      { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}";                                 }
zrbgm_nwn()     { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}";                             }
zrbgm_nwne()    { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_CR}${4}${ZRBGM_R}";    }
zrbgm_nwnw()    { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_W}${4}${ZRBGM_R}";     }
zrbgm_nwnwn()   { zrbgm_show "${1}${ZRBGM_W}${2}${ZRBGM_R}${3}${ZRBGM_W}${4}${ZRBGM_R}${5}"; }

zrbgm_ne()      { zrbgm_show "${1}${ZRBGM_CR}${2}${ZRBGM_R}"; }

zrbgm_cmd()     { zrbgm_show "${ZRBGM_C}${1}${ZRBGM_R}"; }
zrbgm_warning() { zrbgm_show "\n${ZRBGM_WN}  WARNING: ${1}${ZRBGM_R}\n"; }
zrbgm_critic()  { zrbgm_show "\n${ZRBGM_CR} CRITICAL SECURITY WARNING: ${1}${ZRBGM_R}\n"; }


######################################################################
# External Functions (rbgm_*)

rbgm_show_payor_establishment() {
  zrbgm_sentinel

  bcu_doc_brief "Display the manual Payor Establishment procedure"
  bcu_doc_shown || return 0

  zrbgm_s1     "# Payor Establishment Procedure"
  zrbgm_s2     "## Overview"
  zrbgm_n      "Establish the Payor role for Recipe Bottle infrastructure by creating a dedicated"
  zrbgm_n      "administrative project, configuring billing, and creating the Payor service account."
  zrbgm_n      "The Payor manages billing and project lifecycle operations for Recipe Bottle depots."
  zrbgm_s2     "## Prerequisites"
  zrbgm_n      "- Active Google Cloud account with billing enabled"
  zrbgm_n      "- Organization or Folder permissions for project creation"
  zrbgm_n      "- Credit card attached to Google Cloud billing account"
  zrbgm_e

  zrbgm_critic "This procedure is for PERSONAL Google accounts only."
  zrbgm_n      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbgm_n      "you must follow your IT/admin process to create projects, attach billing,"
  zrbgm_n      "and assign permissions - those steps are NOT covered here."
  zrbgm_e

  zrbgm_s1     "Manual Payor Establishment Procedure"
  zrbgm_n      "Recipe Bottle Payor setup requires manual configuration before API operations can proceed."
  zrbgm_e

  zrbgm_s2     "1. Configure Payor Regime Variables:"
  zrbgm_n      "   Before creating resources, configure the Payor regime in your environment."
  zrbgm_nc     "   1. Edit your RBRP configuration file: " "${ZRBGM_RBRP_FILE}"
  zrbgm_n      "   2. Set the following RBRP (Recipe Bottle Regime Payor) variables:"
  zrbgm_nc     "      - RBRP_PAYOR_PROJECT_ID: " "unique-payor-project-id"
  zrbgm_n      "        (6-30 characters, lowercase letters, numbers, hyphens)"
  zrbgm_n      "        (Cannot start/end with hyphen, globally unique across GCP)"
  zrbgm_nc     "      - RBRP_PARENT_TYPE: " "organization"
  zrbgm_nc     "        or RBRP_PARENT_TYPE: " "folder"
  zrbgm_nc     "      - RBRP_PARENT_ID: " "123456789012"
  zrbgm_n      "        (numeric ID of your organization or folder)"
  zrbgm_n      "   3. Leave RBRP_BILLING_ACCOUNT_ID blank for now - will set in step 4"
  zrbgm_n      "   4. Save the file before proceeding"
  zrbgm_e

  zrbgm_s2     "2. Create Payor Project:"
  zrbgm_nc     "   Open browser to: " "https://console.cloud.google.com/projectcreate"
  zrbgm_n      "   1. Configure new project:"
  zrbgm_nc     "      - Project name: " "Recipe Bottle Payor"
  zrbgm_nc     "      - Project ID: " "use your RBRP_PAYOR_PROJECT_ID value"
  zrbgm_n      "      - Organization/Folder: select your parent resource"
  zrbgm_nw     "   2. Click " "CREATE"
  zrbgm_nwne   "   3. If " "The project ID is already taken" " : " "FAIL - choose different RBRP_PAYOR_PROJECT_ID"
  zrbgm_nwn    "   4. Wait for " "Creating project..." " notification to complete"
  zrbgm_e

  zrbgm_s2     "3. Enable Billing Account:"
  zrbgm_nc     "   Go to: " "https://console.cloud.google.com/billing"
  zrbgm_n      "   1. Check existing billing accounts:"
  zrbgm_nw     "      - Look for accounts with status " "Open"
  zrbgm_n      "      - Note the billing account ID (format: XXXXXX-XXXXXX-XXXXXX)"
  zrbgm_n      "   2. If no open billing accounts exist:"
  zrbgm_nc     "      - Go to: " "https://console.cloud.google.com/billing/create"
  zrbgm_n      "      - Follow prompts to create new billing account"
  zrbgm_n      "      - Attach credit card for payment method"
  zrbgm_n      "   3. Record the billing account ID for configuration"
  zrbgm_e

  zrbgm_s2     "4. Update Billing Configuration:"
  zrbgm_nc     "   Return to your RBRR configuration file: " "${ZRBGM_RBRR_FILE}"
  zrbgm_nc     "   1. Set RBRP_BILLING_ACCOUNT_ID: " "XXXXXX-XXXXXX-XXXXXX"
  zrbgm_n      "      (use the billing account ID from step 3)"
  zrbgm_n      "   2. Save the file"
  zrbgm_e

  zrbgm_s2     "5. Link Billing to Payor Project:"
  zrbgm_nc     "   Go to: " "https://console.cloud.google.com/billing/linkedaccount"
  zrbgm_nwnwn  "   1. Ensure billing account " "matches RBRP_BILLING_ACCOUNT_ID" " in top dropdown"
  zrbgm_nw     "   2. Click " "LINK A PROJECT"
  zrbgm_nwnw   "   3. Select project: " "use your RBRP_PAYOR_PROJECT_ID" " from dropdown"
  zrbgm_nw     "   4. Click " "SET ACCOUNT"
  zrbgm_nwn    "   5. Verify project appears in " "Linked projects" " table"
  zrbgm_e

  zrbgm_s2     "6. Create Payor Service Account:"
  zrbgm_nc     "   Go to: " "https://console.cloud.google.com/iam-admin/serviceaccounts"
  zrbgm_nwnwn  "   1. Ensure project " "matches RBRP_PAYOR_PROJECT_ID" " in top dropdown"
  zrbgm_nw     "   2. Click " "+ CREATE SERVICE ACCOUNT"
  zrbgm_n      "   3. Service account details:"
  zrbgm_nc     "      - Service account name: " "payor"
  zrbgm_nc     "      - Service account ID: " "payor"
  zrbgm_nc     "      - Description: " "Payor role for billing and project lifecycle operations"
  zrbgm_nw     "   4. Click " "CREATE AND CONTINUE"
  zrbgm_nwnw   "   5. Grant roles - select from dropdown " "Select a role" ":"
  zrbgm_nc     "      - Type in filter: " "owner"
  zrbgm_nwnw   "      - Select: " "Basic" " -> " "Owner"
  zrbgm_nw     "   6. Click " "CONTINUE"
  zrbgm_nwnw   "   7. Skip optional access settings - click " "DONE"
  zrbgm_e

  zrbgm_s2     "7. Grant Billing Permissions to Payor:"
  zrbgm_nc     "   Return to: " "https://console.cloud.google.com/billing"
  zrbgm_nwnwn  "   1. Select billing account " "matching RBRP_BILLING_ACCOUNT_ID" " from list"
  zrbgm_nw     "   2. Left sidebar -> " "Account Management"
  zrbgm_nw     "   3. Click " "+ ADD PRINCIPAL"
  zrbgm_n      "   4. Configure IAM binding:"
  zrbgm_nc     "      - New principals: " "payor@[RBRP_PAYOR_PROJECT_ID].iam.gserviceaccount.com"
  zrbgm_nc     "      - Role: " "Billing Admin"
  zrbgm_nw     "   5. Click " "SAVE"
  zrbgm_e

  zrbgm_s2     "8. Generate Payor Service Account Key:"
  zrbgm_nc     "   Return to: " "https://console.cloud.google.com/iam-admin/serviceaccounts"
  zrbgm_nwnwn  "   1. Ensure project " "matches RBRP_PAYOR_PROJECT_ID" " in top dropdown"
  zrbgm_nw     "   2. Click on the " "payor" " service account email"
  zrbgm_nw     "   3. Top tabs -> " "KEYS"
  zrbgm_nwnw   "   4. Click " "ADD KEY" " -> " "Create new key"
  zrbgm_nwn    "   5. Key type: " "JSON" " (should be selected)"
  zrbgm_nw     "   6. Click " "CREATE"
  zrbgm_e
  zrbgm_nw     "Browser downloads: " "[RBRP_PAYOR_PROJECT_ID]-[random].json"
  zrbgm_nwn    "   7. Click " "CLOSE" " on download confirmation"
  zrbgm_e

  zrbgm_s2     "9. Configure Payor RBRA File:"
  zrbgm_n      "   The downloaded JSON key needs to be converted to RBRA format."
  zrbgm_n      "   Use the appropriate Recipe Bottle tool to generate the Payor RBRA file"
  zrbgm_n      "   from the downloaded JSON key file."
  zrbgm_e

  zrbgm_warning "Manual setup complete. You can now run Recipe Bottle Payor API operations."
  zrbgm_n      "Verify configuration by testing Payor operations before proceeding to depot creation."

  bcu_success "Payor establishment procedure displayed"
}

rbgm_LEGACY_setup_admin() {
  zrbgm_sentinel

  bcu_doc_brief "Display the manual GCP admin setup procedure"
  bcu_doc_shown || return 0

  zrbgm_s1     "# Google Cloud Platform Setup"
  zrbgm_s2     "## Overview"
  zrbgm_n      "Bootstrap GCP infrastructure by creating an admin service account with Project Owner privileges."
  zrbgm_n      "The admin account will manage operational service accounts and infrastructure configuration."
  zrbgm_s2     "## Prerequisites"
  zrbgm_n      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbgm_n      "- Email address not already associated with GCP"
  zrbgm_e
  zrbgm_n      "---"
  zrbgm_s1     "Manual Admin Setup Procedure"
  zrbgm_n      "Recipe Bottle setup requires a manual bootstrap procedure to enable admin control"
  zrbgm_e
  zrbgm_nc     "Open a web browser to " "${RBGC_SIGNUP_URL}"

  zrbgm_critic "This procedure is for PERSONAL Google accounts only."
  zrbgm_n      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbgm_n      "you must follow your IT/admin process to create projects, attach billing,"
  zrbgm_n      "and assign permissions - those steps are NOT covered here."
  zrbgm_e
  zrbgm_s2     "1. Establish Account:"
  zrbgm_nc     "   Open a browser to: " "${RBGC_SIGNUP_URL}"
  zrbgm_nw     "   1. Click -> " "Get started for free"
  zrbgm_n      "   2. Sign in with your Google account or create a new one"
  zrbgm_n      "   3. Provide:"
  zrbgm_n      "      - Country"
  zrbgm_nw     "      - Organization type: " "Individual"
  zrbgm_n      "      - Credit card (verification only)"
  zrbgm_nw     "   4. Accept terms -> " "Start my free trial"
  zrbgm_n      "   5. Expect Google Cloud Console to open"
  zrbgm_nwn    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbgm_e
  local z_configure_pid_step="2. Configure Project ID and Region"
  zrbgm_s2     "${z_configure_pid_step}:"
  zrbgm_n      "   Before creating the project, choose a unique Project ID."
  zrbgm_nc     "   1. Edit your RBRR configuration file: " "${ZRBGM_RBRR_FILE}"
  zrbgm_n      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbgm_n      "      - Must be globally unique across all GCP"
  zrbgm_n      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbgm_n      "      - Cannot start/end with hyphen"
  zrbgm_n      "   3. Set RBRR_GCP_REGION based on your location (see project documentation)"
  zrbgm_n      "   4. Save the file before proceeding"
  zrbgm_e
  zrbgm_s2     "3. Create New Project:"
  zrbgm_nc     "   Go directly to: " "${RBGC_CONSOLE_URL}"
  zrbgm_n      "   Sign in with the same Google account you just set up"
  zrbgm_n      "   1. Open the Google Cloud Console main menu:"
  zrbgm_nwn    "      - Click the " "->" " hamburger menu in the top-left corner"
  zrbgm_nw     "      - Scroll down to " "IAM & Admin"
  zrbgm_nw     "      - Click -> " "Manage resources"
  zrbgm_n      "        (Alternatively, type 'manage resources' in the top search bar and press Enter)"
  zrbgm_nw     "   2. On the Manage resources page, click -> " "CREATE PROJECT"
  zrbgm_n      "   3. Configure:"
  zrbgm_nc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbgm_nw     "      - Organization: " "No organization"
  zrbgm_nw     "   4. Click " "CREATE"
  zrbgm_nwne   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbgm_nwn    "   6. If popup, wait for notification -> " "Creating project..." " to complete"
  zrbgm_e
  zrbgm_s2     "4. Create the Admin Service Account:"
  zrbgm_nwnwn  "   1. Ensure project " "${RBRR_GCP_PROJECT_ID}" " is selected in the top dropdown (button with hovertext " "Open project picker (Ctrl O)" ")"
  zrbgm_nwnw   "   2. Left sidebar -> " "IAM & Admin" " -> " "Service Accounts"
  zrbgm_n      "   3. If prompted about APIs:"
  zrbgm_nw     "      1. click -> " "Enable API"
  zrbgm_n      "          TODO: This step is brittle - enabling IAM API may happen automatically or be blocked by org policy."
  zrbgm_nw     "      2. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbgm_nw     "   4. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbgm_n      "   5. Service account details:"
  zrbgm_nc     "      - Service account name: " "${RBGC_ADMIN_ROLE}"
  zrbgm_nwn    "      - Service account ID: (auto-fills as " "${RBGC_ADMIN_ROLE}" ")"
  zrbgm_nc     "      - Description: " "Admin account for infrastructure management"
  zrbgm_nw     "   6. Click -> " "Create and continue"
  zrbgm_nwnw   "   7. At " "Permissions (optional)" " pick dropdown " "Select a role"
  zrbgm_nc     "      - In filter box, type: " "owner"
  zrbgm_nwnw   "      - Select: " "Basic" " -> " "Owner"
  zrbgm_nw     "   8. Click -> " "Continue"
  zrbgm_nwnw   "   9. Skip " "Principals with access" " by clicking -> " "Done"
  zrbgm_e
  zrbgm_s2     "7. Generate Service Account Key:"
  zrbgm_n      "From service accounts list:"
  zrbgm_nw     "   1. Click on text of " "${RBGC_ADMIN_ROLE}@${RBGC_SA_EMAIL_FULL}"
  zrbgm_nw     "   2. Top tabs -> " "Keys"
  zrbgm_nwnw   "   3. Click " "Add key" " -> " "Create new key"
  zrbgm_nwn    "   4. Key type: " "JSON" " (should be selected)"
  zrbgm_nw     "   5. Click " "Create"
  zrbgm_e
  zrbgm_nw     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbgm_nwn    "   6. Click " "CLOSE" " on download confirmation"
  zrbgm_e
  zrbgm_s2     "8. Configure Local Environment:"
  zrbgm_n      "Browser downloaded key.  Run the command to ingest it into your ADMIN RBRA file."
  zrbgm_e

  bcu_success "Manual setup procedure displayed"
}


# eof
