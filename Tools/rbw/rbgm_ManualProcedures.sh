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

rbgm_payor_establish() {
  zrbgm_sentinel

  bcu_doc_brief "Display the manual Payor Establishment procedure for OAuth authentication"
  bcu_doc_shown || return 0

  zrbgm_s1     "Manual Payor OAuth Establishment Procedure"
  zrbgm_d      "Recipe Bottle Payor now uses OAuth 2.0 for individual developer accounts."
  zrbgm_d      "This resolves project creation limitations for personal Google accounts."
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
  zrbgm_dc     "      - Project ID: " "rbw-payor (or rbw-payor-[suffix] if taken)"
  zrbgm_d      "      - Parent: OAuth users create without parent (no organization required)"
  zrbgm_dm     "   4. Click " "CREATE"
  zrbgm_dmdr   "   5. If " "The project ID is already taken" " : " "FAIL - try rbw-payor-[random]"
  zrbgm_dmd    "   6. Wait for " "Creating project..." " notification to complete"
  zrbgm_e
  zrbgm_s2     "3. Configure Billing Account:"
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
  zrbgm_dmd    "   6. Find project row with ID matching your payor project (not name)"
  zrbgm_dcdm   "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  zrbgm_dm     "   7. Save your " "${ZRBGM_RBRP_FILE}" " and re-display this procedure."
  zrbgm_e
  zrbgm_s2     "4. Link Billing to Payor Project:"
  zrbgm_d      "   Link your billing account to the newly created project."
  zrbgm_dld    "   Go to: " "Google Cloud Billing Linked Accounts" "https://console.cloud.google.com/billing/linkedaccount"
  zrbgm_d      "   1. Select the correct billing account:"
  zrbgm_dm     "      - Use the dropdown at top to select billing account " "matching RBRP_BILLING_ACCOUNT_ID"
  zrbgm_d      "      - The dropdown shows account name and ID (format: Account Name - XXXXXX-XXXXXX-XXXXXX)"
  zrbgm_dm     "   2. Click " "LINK A PROJECT"
  zrbgm_dmd    "   3. Select project: " "your rbw-payor project" " from dropdown"
  zrbgm_d      "      - Type the project name or ID to filter the list if needed"
  zrbgm_dm     "   4. Click " "SET ACCOUNT"
  zrbgm_dmd    "   5. Verify project appears in " "Linked projects" " table with 'Active' status"
  zrbgm_e
  zrbgm_s2     "5. Enable Required APIs:"
  zrbgm_dld    "   Go to: " "APIs & Services for your payor project" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID:-rbw-payor}"
  zrbgm_dm     "   1. Click " "+ ENABLE APIS AND SERVICES"
  zrbgm_d      "   2. Search for and enable these APIs:"
  zrbgm_dc     "      - " "Cloud Resource Manager API"
  zrbgm_dc     "      - " "Cloud Billing API"
  zrbgm_dc     "      - " "Service Usage API"
  zrbgm_dc     "      - " "IAM Service Account Credentials API"
  zrbgm_dc     "      - " "Artifact Registry API"
  zrbgm_d      "   These enable programmatic depot management operations."
  zrbgm_e
  zrbgm_s2     "6. Configure OAuth Consent Screen:"
  zrbgm_dld    "   Go to: " "OAuth consent screen" "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID:-rbw-payor}"
  zrbgm_dm     "   1. User Type: Select " "External" " (no Google Workspace required)"
  zrbgm_dm     "   2. Click " "CREATE"
  zrbgm_d      "   3. App information:"
  zrbgm_dc     "      - App name: " "Recipe Bottle Payor"
  zrbgm_d      "      - User support email: (your email)"
  zrbgm_d      "      - Developer contact information: (your email)"
  zrbgm_dm     "   4. Click " "SAVE AND CONTINUE"
  zrbgm_d      "   5. Scopes: Add these OAuth scopes:"
  zrbgm_dc     "      - " "https://www.googleapis.com/auth/cloud-platform"
  zrbgm_dc     "      - " "https://www.googleapis.com/auth/cloud-billing"
  zrbgm_dm     "   6. Click " "SAVE AND CONTINUE"
  zrbgm_d      "   7. Test users: Add your email if needed"
  zrbgm_dm     "   8. Click " "SAVE AND CONTINUE"
  zrbgm_dm     "   9. Review and click " "BACK TO DASHBOARD"
  zrbgm_e
  zrbgm_s2     "7. Create OAuth 2.0 Client ID:"
  zrbgm_dld    "   Go to: " "Credentials" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID:-rbw-payor}"
  zrbgm_dm     "   1. Click " "+ CREATE CREDENTIALS"
  zrbgm_dm     "   2. Select " "OAuth client ID"
  zrbgm_dm     "   3. Application type: " "Desktop application"
  zrbgm_dc     "   4. Name: " "Recipe Bottle Payor"
  zrbgm_dm     "   5. Click " "CREATE"
  zrbgm_d      "   6. In the popup that appears:"
  zrbgm_dm     "      - Click " "DOWNLOAD JSON"
  zrbgm_dm     "      - Click " "OK"
  zrbgm_e
  zrbgm_dm     "Browser downloads: " "client_secret_[id].apps.googleusercontent.com.json"
  zrbgm_dy     "   " "CRITICAL: Save this file securely - client secret cannot be recovered"
  zrbgm_d      "   Rename to something memorable like " "payor-oauth.json"
  zrbgm_e
  zrbgm_s2     "8. Update Configuration:"
  zrbgm_dmd    "   Update " "${ZRBGM_RBRP_FILE}" " with your payor project values:"
  zrbgm_dc     "      " "RBRP_PAYOR_PROJECT_ID=rbw-payor  # or your chosen project ID"
  zrbgm_dc     "      " "RBRP_BILLING_ACCOUNT_ID=XXXXXX-XXXXXX-XXXXXX  # from step 3"
  zrbgm_d      "   Note: RBRP_OAUTH_CLIENT_ID will be set by credential installation"
  zrbgm_e
  zrbgm_dy     " " "Project setup complete. Use rbgm_payor_refresh to install credentials."
  zrbgm_d      "Next step: Run rbgm_payor_refresh to install OAuth credentials and complete setup."

  bcu_success "OAuth Payor establishment procedure displayed"
}

rbgm_payor_refresh() {
  zrbgm_sentinel

  bcu_doc_brief "Display the manual Payor OAuth credential installation/refresh procedure"
  bcu_doc_shown || return 0

  zrbgm_s1     "Manual Payor OAuth Credential Installation/Refresh Procedure"
  zrbgm_d      "Use this for initial credential setup after payor establishment or to refresh expired/compromised credentials."
  zrbgm_d      "Testing mode refresh tokens expire after 6 months of non-use."
  zrbgm_e
  zrbgm_s2     "When to use this procedure:"
  zrbgm_d      "  - Initial setup after running rbgm_payor_establish"
  zrbgm_d      "  - Payor operations return 401/403 errors"
  zrbgm_d      "  - OAuth client secret compromised"
  zrbgm_d      "  - 6+ months since last Payor operation"
  zrbgm_e
  zrbgm_s2     "1. Obtain OAuth Credentials:"
  zrbgm_d      "   For initial setup:"
  zrbgm_d      "      - Use JSON file downloaded during rbgm_payor_establish"
  zrbgm_d      "   For refresh/renewal:"
  zrbgm_dld    "      - Go to: " "Credentials for Payor Project" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID:-rbw-payor}"
  zrbgm_dm     "      - Find existing " "Recipe Bottle Payor" " OAuth client"
  zrbgm_d      "      - If client missing: re-run rbgm_payor_establish"
  zrbgm_dm     "      - Click the " "download" " icon next to OAuth client"
  zrbgm_d      "      - Or to regenerate secret if compromised:"
  zrbgm_dm     "        a. Click the " "edit" " icon (pencil)"
  zrbgm_dm     "        b. Click " "RESET SECRET" " if available"
  zrbgm_dm     "        c. Click " "SAVE"
  zrbgm_dm     "        d. Click " "DOWNLOAD JSON"
  zrbgm_d      "      - Save as: " "payor-oauth-$(date +%Y%m%d).json"
  zrbgm_e
  zrbgm_s2     "2. Install/Refresh OAuth Credentials:"
  zrbgm_d      "   Run the payor install command:"
  zrbgm_dc     "      " "rbgp_payor_install /path/to/payor-oauth.json"
  zrbgm_d      "   This will:"
  zrbgm_d      "   - Guide you through OAuth authorization flow"
  zrbgm_d      "   - Store secure credentials in ~/.rbw/rbro.env"
  zrbgm_d      "   - Update RBRP_OAUTH_CLIENT_ID in rbrp.env"
  zrbgm_d      "   - Test the authentication"
  zrbgm_d      "   - Initialize depot tracking"
  zrbgm_d      "   - Reset the 6-month expiration timer"
  zrbgm_e
  zrbgm_s2     "3. Verify Installation:"
  zrbgm_d      "   Test with a simple operation:"
  zrbgm_dc     "      " "rbgp_depot_list"
  zrbgm_d      "   Should display current depots without authentication errors."
  zrbgm_e
  zrbgm_dy     " " "OAuth credentials installed/refreshed. Payor operations should work normally."
  zrbgm_d      "Prevention: Run any Payor operation monthly to prevent expiration."

  bcu_success "OAuth credential installation/refresh procedure displayed"
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
