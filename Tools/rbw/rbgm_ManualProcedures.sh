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
test -z "${ZRBGM_SOURCED:-}" || buc_die "Module rbgm multiply sourced - check sourcing hierarchy"
ZRBGM_SOURCED=1

######################################################################
# Internal Functions (zrbgm_*)

zrbgm_kindle() {
  test -z "${ZRBGM_KINDLED:-}" || buc_die "Module rbgm already kindled"

  test -n "${RBRR_GCP_PROJECT_ID:-}"     || buc_die "RBRR_GCP_PROJECT_ID is not set"
  test   "${#RBRR_GCP_PROJECT_ID}" -gt 0 || buc_die "RBRR_GCP_PROJECT_ID is empty"

  buc_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  local z_use_color=0
  if [ -z "${NO_COLOR:-}" ] && [ "${BUD_COLOR:-0}" = "1" ]; then
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
  ZRBGM_RBRP_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrp.env"
  ZRBGM_RBRP_FILE_BASENAME="${ZRBGM_RBRP_FILE##*/}"
  ZRBGM_RBRR_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrr_RecipeBottleRegimeRepo.sh"


  ZRBGM_PREFIX="${BUD_TEMP_DIR}/rbgm_"
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
  test "${ZRBGM_KINDLED:-}" = "1" || buc_die "Module rbgm not kindled - call zrbgm_kindle first"
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
  test -n "${z_label}" || buc_die "missing label"
  test -n "${z_url}"   || buc_die "missing url"

  # Blue + underline style, wrapped in OSC 8 hyperlink
  printf '%s\e[34;4m\e]8;;%s\a%s\e]8;;\a\e[0m%s\n' "${z_default}" "${z_url}" "${z_label}" "${z_coda}"
}


######################################################################
# External Functions (rbgm_*)

rbgm_payor_establish() {
  zrbgm_sentinel

  buc_doc_brief "Display the manual Payor Establishment procedure for OAuth authentication"
  buc_doc_shown || return 0

  zrbgm_s1     "Manual Payor OAuth Establishment Procedure"
  zrbgm_d      "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  zrbgm_d      "This resolves project creation limitations for personal Google accounts."
  zrbgm_e
  zrbgm_s2     "Key:"
  zrbgm_dm     "   Magenta text refers to " "precise words you see on the web page."
  zrbgm_dc     "   Cyan text is " "something you might copy from here."
  zrbgm_d      "   Default text is this color."
  zrbgm_dld    "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, Ctrl + mouse click)"
  zrbgm_e
  zrbgm_s2     "1. Confirm Payor Regime:"
  zrbgm_dc     "   File: " "${ZRBGM_RBRP_FILE}"
  zrbgm_dc     "   RBRP_PAYOR_PROJECT_ID: " "${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_d      "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  zrbgm_e
  zrbgm_d      "   First time setup? Set a timestamped project ID with:"
  zrbgm_cmd    "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})/' ${ZRBGM_RBRP_FILE}"
  zrbgm_e
  zrbgm_s2     "2. Check if Project Already Exists:"
  zrbgm_d      "   Before creating a new project, verify the configured ID is not already in use:"
  zrbgm_dld    "   1. Check existing projects: " "Google Cloud Project List" "https://console.cloud.google.com/cloud-resource-manager"
  zrbgm_dm     "   2. Look for a project with ID " "${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_d      "      - Hover over project IDs to verify the full ID matches your configured value"
  zrbgm_dcdm   "   3. If you " "find the project" " with matching ID, it already exists - edit " "${ZRBGM_RBRP_FILE_BASENAME}"
  zrbgm_d      "      and re-run this procedure"
  zrbgm_d      "   4. If you don't find it, proceed to step 3 to create it"
  zrbgm_e
  zrbgm_s2     "3. Create Payor Project:"
  zrbgm_dld    "   1. Open browser to: " "Google Cloud Project Create" "https://console.cloud.google.com/projectcreate"
  zrbgm_d      "   2. Ensure signed in with intended Google account (check top-right avatar)"
  zrbgm_d      "   3. Configure new project:"
  zrbgm_dc     "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  zrbgm_d      "      - Project ID: Google will auto-generate a value; click Edit to replace it with:"
  zrbgm_dc     "        " "${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dmd    "      - Location: " "No organization" " (required for this guide; organization affiliation is advanced)"
  zrbgm_dm     "   4. Click " "CREATE"
  zrbgm_dmd    "   5. Wait for " "Creating project..." " notification to complete"
  zrbgm_e
  zrbgm_s2     "4. Verify Project Creation:"
  zrbgm_d      "   Verify that your rbrp.env configuration matches the created project:"
  zrbgm_dld    "   1. Test this link: " "Google Cloud APIs Dashboard" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_d      "   2. If the page loads and shows your project, your configuration is correct"
  zrbgm_dmd    "   3. If you see " "You need additional access" ", wait a few minutes and refresh the page"
  zrbgm_dld    "      GCP IAM changes are eventually consistent: " "Access Change Propagation" "https://cloud.google.com/iam/docs/access-change-propagation"
  zrbgm_e
  zrbgm_s2     "5. Configure Billing Account:"
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
  zrbgm_dld    "   2. Go to: " "Google Cloud Billing Projects" "https://console.cloud.google.com/billing/projects"
  zrbgm_dm     "   3. Save the billing account ID to your " "${ZRBGM_RBRP_FILE}"
  zrbgm_dcdm   "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  zrbgm_d      "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  zrbgm_dm     "   5. Update " "${ZRBGM_RBRP_FILE}" " and re-display this procedure."
  zrbgm_e
  zrbgm_s2     "6. Link Billing to Payor Project:"
  zrbgm_d      "   Link your billing account to the newly created project."
  zrbgm_dld    "   Go to: " "Google Cloud Billing Account Management" "https://console.cloud.google.com/billing/manage"
  zrbgm_d      "   1. The page loads to your default billing account."
  zrbgm_dm     "   2. If you have multiple billing accounts, use the " "Select a billing account" " dropdown at top"
  zrbgm_d      "      - Choose the account matching your RBRP_BILLING_ACCOUNT_ID"
  zrbgm_d      "   3. Look for the section " "Projects linked to this billing account"
  zrbgm_d      "   4. Verify your payor project appears in the table:"
  zrbgm_dc     "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  zrbgm_dc     "      - Project ID: " "${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_d      "   5. If project is NOT listed and billing needs to be enabled:"
  zrbgm_dld    "      - Go to: " "Project Billing" "https://console.cloud.google.com/billing/linkedaccount?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "      - Click " "Link a Billing Account"
  zrbgm_d      "      - Select your billing account and confirm"
  zrbgm_e
  zrbgm_s2     "7. Enable Required APIs:"
  zrbgm_dld    "   Go to: " "APIs & Services for your payor project" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "   1. Click " "+ ENABLE APIS AND SERVICES"
  zrbgm_d      "   2. Search for and enable these APIs:"
  zrbgm_dc     "      - " "Cloud Resource Manager API"
  zrbgm_dc     "      - " "Cloud Billing API"
  zrbgm_dcd    "      - " "Service Usage API" " (often enabled by default, look for green check)"
  zrbgm_dc     "      - " "IAM Service Account Credentials API"
  zrbgm_dc     "      - " "Artifact Registry API"
  zrbgm_d      "   These enable programmatic depot management operations."
  zrbgm_e
  zrbgm_s2     "8. Configure OAuth Consent Screen:"
  zrbgm_dld    "   Go to: " "OAuth consent screen" "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "   1. The console displays " "Google Auth Platform not configured yet"
  zrbgm_dm     "   2. Click " "Get started"
  zrbgm_d      "   3. Complete the Project Configuration wizard:"
  zrbgm_d      "      Step 1 - App Information:"
  zrbgm_dc     "        - App name: " "${RBGC_PAYOR_APP_NAME}"
  zrbgm_d      "        - User support email: (your email)"
  zrbgm_dm     "        - Click " "Next"
  zrbgm_d      "      Step 2 - Audience:"
  zrbgm_dm     "        - Select " "External"
  zrbgm_dm     "        - Click " "Next"
  zrbgm_d      "      Step 3 - Contact Information:"
  zrbgm_d      "        - Email addresses: (your email), press Enter"
  zrbgm_dm     "        - Click " "Next"
  zrbgm_d      "      Step 4 - Finish:"
  zrbgm_dm     "        - Check " "I agree to the Google API Services: User Data Policy"
  zrbgm_dm     "        - Click " "Continue"
  zrbgm_dm     "        - Click " "Create"
  zrbgm_e
  zrbgm_d      "   4. Add your email as a test user (avoids Google app verification process):"
  zrbgm_dm     "      1. Click " "Audience" " in left sidebar"
  zrbgm_dm     "      2. Scroll down to section " "Test users"
  zrbgm_dm     "      3. Click " "+ Add users"
  zrbgm_dmd    "      4. Right-side panel titled " "Add Users" " slides in"
  zrbgm_d      "      5. Enter your email address in the field"
  zrbgm_dm     "      6. Click " "Save"
  zrbgm_dmd    "      7. Verify " "1 user" " appears in OAuth user cap"
  zrbgm_e
  zrbgm_s2     "9. Create OAuth 2.0 Client ID:"
  zrbgm_dld    "   Go to: " "Credentials" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "   1. From top bar, click " "+ Create credentials"
  zrbgm_dm     "   2. Select " "OAuth client ID"
  zrbgm_dm     "   3. Application type: " "Desktop app"
  zrbgm_dc     "   4. Name: " "${RBGC_PAYOR_APP_NAME}"
  zrbgm_dm     "   5. Click " "CREATE"
  zrbgm_dmd    "   6. Popup titled " "OAuth client created" " displays client ID and secret"
  zrbgm_dm     "   7. Click " "Download JSON"
  zrbgm_dmdm   "   8. Click " "OK" " ; browser downloads " "client_secret_[id].apps.googleusercontent.com.json"
  zrbgm_dy     "      " "CRITICAL: Save securely - contains client secret"
  zrbgm_e
  zrbgm_s2     "10. Install OAuth Credentials:"
  zrbgm_d      "   Run:"
  zrbgm_cmd    "   rbgp_payor_install ~/Downloads/payor-oauth.json"
  zrbgm_d      "   The command will:"
  zrbgm_d      "   - Display the OAuth authorization URL to open in your browser"
  zrbgm_d      "   - Guide you through three consent screens:"
  zrbgm_d      "     1. Unverified app warning - Click Continue"
  zrbgm_d      "     2. Permission request - Check the permission checkboxes, click Continue"
  zrbgm_d      "     3. Authorization code display - Copy the code"
  zrbgm_d      "   - Prompt for you to paste the authorization code"
  zrbgm_d      "   - Store credentials securely in ~/.rbw/rbro.env"
  zrbgm_d      "   - Test authentication with Google Cloud"

  buc_success "OAuth Payor establishment procedure displayed"
}

rbgm_payor_refresh() {
  zrbgm_sentinel

  buc_doc_brief "Display the manual Payor OAuth credential installation/refresh procedure"
  buc_doc_shown || return 0

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
  zrbgm_dld    "      - Go to: " "Credentials for Payor Project" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  zrbgm_dm     "      - Find existing " "${RBGC_PAYOR_APP_NAME}" " OAuth client"
  zrbgm_d      "      - If client missing: re-run rbgm_payor_establish"
  zrbgm_dm     "      - Click the " "download" " icon next to OAuth client"
  zrbgm_d      "      - Or to regenerate secret if compromised:"
  zrbgm_dm     "        a. Click the " "edit" " icon (pencil)"
  zrbgm_dm     "        b. Click " "RESET SECRET" " if available"
  zrbgm_dm     "        c. Click " "SAVE"
  zrbgm_dm     "        d. Click " "DOWNLOAD JSON"
  zrbgm_dm     "      - Save as: " "payor-oauth-$(date +%Y%m%d).json"
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

  buc_success "OAuth credential installation/refresh procedure displayed"
}

rbgm_LEGACY_setup_admin() { # ITCH_DELETE_THIS_AFTER_ABOVE_TESTED
  zrbgm_sentinel

  buc_doc_brief "Display the manual GCP admin setup procedure"
  buc_doc_shown || return 0

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

  buc_success "Manual setup procedure displayed"
}


# eof
