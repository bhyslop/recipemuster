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

  test -n "${RBRR_DEPOT_PROJECT_ID:-}"     || buc_die "RBRR_DEPOT_PROJECT_ID is not set"
  test   "${#RBRR_DEPOT_PROJECT_ID}" -gt 0 || buc_die "RBRR_DEPOT_PROJECT_ID is empty"

  buc_log_args "Ensure RBGC is kindled first"
  zrbgc_sentinel

  local z_use_color=0
  if [ -z "${NO_COLOR:-}" ] && [ "${BURD_COLOR:-0}" = "1" ]; then
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


  ZRBGM_PREFIX="${BURD_TEMP_DIR}/rbgm_"
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

  bug_section  "Manual Payor OAuth Establishment Procedure"
  bug_t        "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  bug_t        "This resolves project creation limitations for personal Google accounts."
  bug_e
  bug_section  "Key:"
  bug_tu       "   Magenta text refers to " "precise words you see on the web page."
  bug_tc       "   Cyan text is " "something you might copy from here."
  bug_t        "   Default text is this color."
  bug_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, Ctrl + mouse click)"
  bug_e
  bug_section  "1. Confirm Payor Regime:"
  bug_tc       "   File: " "${ZRBGM_RBRP_FILE}"
  bug_tc       "   RBRP_PAYOR_PROJECT_ID: " "${RBRP_PAYOR_PROJECT_ID}"
  bug_t        "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  bug_e
  bug_t        "   First time setup? Set a timestamped project ID with:"
  bug_c        "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date ${RBGC_GLOBAL_TIMESTAMP_FORMAT})/' ${ZRBGM_RBRP_FILE}"
  bug_e
  bug_section  "2. Check if Project Already Exists:"
  bug_t        "   Before creating a new project, verify the configured ID is not already in use:"
  bug_link     "   1. Check existing projects: " "Google Cloud Project List" "https://console.cloud.google.com/cloud-resource-manager"
  bug_tu       "   2. Look for a project with ID " "${RBRP_PAYOR_PROJECT_ID}"
  bug_t        "      - Hover over project IDs to verify the full ID matches your configured value"
  bug_tctu     "   3. If you " "find the project" " with matching ID, it already exists - edit " "${ZRBGM_RBRP_FILE_BASENAME}"
  bug_t        "      and re-run this procedure"
  bug_t        "   4. If you don't find it, proceed to step 3 to create it"
  bug_e
  bug_section  "3. Create Payor Project:"
  bug_link     "   1. Open browser to: " "Google Cloud Project Create" "https://console.cloud.google.com/projectcreate"
  bug_t        "   2. Ensure signed in with intended Google account (check top-right avatar)"
  bug_t        "   3. Configure new project:"
  bug_tc       "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  bug_t        "      - Project ID: Google will auto-generate a value; click Edit to replace it with:"
  bug_tc       "        " "${RBRP_PAYOR_PROJECT_ID}"
  bug_tut      "      - Location: " "No organization" " (required for this guide; organization affiliation is advanced)"
  bug_tu       "   4. Click " "CREATE"
  bug_tut      "   5. Wait for " "Creating project..." " notification to complete"
  bug_e
  bug_section  "4. Verify Project Creation:"
  bug_t        "   Verify that your rbrp.env configuration matches the created project:"
  bug_link     "   1. Test this link: " "Google Cloud APIs Dashboard" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  bug_t        "   2. If the page loads and shows your project, your configuration is correct"
  bug_tut      "   3. If you see " "You need additional access" ", wait a few minutes and refresh the page"
  bug_link     "      GCP IAM changes are eventually consistent: " "Access Change Propagation" "https://cloud.google.com/iam/docs/access-change-propagation"
  bug_e
  bug_section  "5. Configure Billing Account:"
  bug_link     "   1. Go to: " "Google Cloud Billing" "https://console.cloud.google.com/billing"
  bug_t        "      If no billing accounts exist:"
  bug_tu       "          a. Click " "CREATE ACCOUNT"
  bug_t        "          b. Configure payment method and submit"
  bug_tu       "          c. Copy new " "Account ID" " from table"
  bug_t        "      else if single Open account exists:"
  bug_tu       "          a. Copy the " "Account ID" " value"
  bug_t        "      else if multiple Open accounts exist:"
  bug_t        "          a. Choose account for Recipe Bottle funding"
  bug_tu       "          b. Copy chosen " "Account ID" " value"
  bug_link     "   2. Go to: " "Google Cloud Billing Projects" "https://console.cloud.google.com/billing/projects"
  bug_tu       "   3. Save the billing account ID to your " "${ZRBGM_RBRP_FILE}"
  bug_tctu     "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  bug_t        "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  bug_tu       "   5. Update " "${ZRBGM_RBRP_FILE}" " and re-display this procedure."
  bug_e
  bug_section  "6. Link Billing to Payor Project:"
  bug_t        "   Link your billing account to the newly created project."
  bug_link     "   Go to: " "Google Cloud Billing Account Management" "https://console.cloud.google.com/billing/manage"
  bug_t        "   1. The page loads to your default billing account."
  bug_tu       "   2. If you have multiple billing accounts, use the " "Select a billing account" " dropdown at top"
  bug_t        "      - Choose the account matching your RBRP_BILLING_ACCOUNT_ID"
  bug_t        "   3. Look for the section " "Projects linked to this billing account"
  bug_t        "   4. Verify your payor project appears in the table:"
  bug_tc       "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  bug_tc       "      - Project ID: " "${RBRP_PAYOR_PROJECT_ID}"
  bug_t        "   5. If project is NOT listed and billing needs to be enabled:"
  bug_link     "      - Go to: " "Project Billing" "https://console.cloud.google.com/billing/linkedaccount?project=${RBRP_PAYOR_PROJECT_ID}"
  bug_tu       "      - Click " "Link a Billing Account"
  bug_t        "      - Select your billing account and confirm"
  bug_e
  bug_section  "7. Enable Required APIs:"
  bug_link     "   Go to: " "APIs & Services for your payor project" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  bug_tu       "   1. Click " "+ ENABLE APIS AND SERVICES"
  bug_t        "   2. Search for and enable these APIs:"
  bug_tc       "      - " "Cloud Resource Manager API"
  bug_tc       "      - " "Cloud Billing API"
  bug_tct      "      - " "Service Usage API" " (often enabled by default, look for green check)"
  bug_tc       "      - " "IAM Service Account Credentials API"
  bug_tc       "      - " "Artifact Registry API"
  bug_t        "   These enable programmatic depot management operations."
  bug_e
  bug_section  "8. Configure OAuth Consent Screen:"
  bug_link     "   Go to: " "OAuth consent screen" "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID}"
  bug_tu       "   1. The console displays " "Google Auth Platform not configured yet"
  bug_tu       "   2. Click " "Get started"
  bug_t        "   3. Complete the Project Configuration wizard:"
  bug_t        "      Step 1 - App Information:"
  bug_tc       "        - App name: " "${RBGC_PAYOR_APP_NAME}"
  bug_t        "        - User support email: (your email)"
  bug_tu       "        - Click " "Next"
  bug_t        "      Step 2 - Audience:"
  bug_tu       "        - Select " "External"
  bug_tu       "        - Click " "Next"
  bug_t        "      Step 3 - Contact Information:"
  bug_t        "        - Email addresses: (your email), press Enter"
  bug_tu       "        - Click " "Next"
  bug_t        "      Step 4 - Finish:"
  bug_tu       "        - Check " "I agree to the Google API Services: User Data Policy"
  bug_tu       "        - Click " "Continue"
  bug_tu       "        - Click " "Create"
  bug_e
  bug_t        "   4. Add your email as a test user (avoids Google app verification process):"
  bug_tu       "      1. Click " "Audience" " in left sidebar"
  bug_tu       "      2. Scroll down to section " "Test users"
  bug_tu       "      3. Click " "+ Add users"
  bug_tut      "      4. Right-side panel titled " "Add Users" " slides in"
  bug_t        "      5. Enter your email address in the field"
  bug_tu       "      6. Click " "Save"
  bug_tut      "      7. Verify " "1 user" " appears in OAuth user cap"
  bug_e
  bug_section  "9. Create OAuth 2.0 Client ID:"
  bug_link     "   Go to: " "Credentials" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  bug_tu       "   1. From top bar, click " "+ Create credentials"
  bug_tu       "   2. Select " "OAuth client ID"
  bug_tu       "   3. Application type: " "Desktop app"
  bug_tc       "   4. Name: " "${RBGC_PAYOR_APP_NAME}"
  bug_tu       "   5. Click " "CREATE"
  bug_tut      "   6. Popup titled " "OAuth client created" " displays client ID and secret"
  bug_tu       "   7. Click " "Download JSON"
  bug_tutu     "   8. Click " "OK" " ; browser downloads " "client_secret_[id].apps.googleusercontent.com.json"
  bug_tW       "      " "CRITICAL: Save securely - contains client secret"
  bug_e
  bug_section  "10. Install OAuth Credentials:"
  bug_t        "   Run:"
  bug_c        "   rbgp_payor_install ~/Downloads/payor-oauth.json"
  bug_t        "   This will guide you through OAuth authorization and complete the setup."

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
  zrbgm_d      "      - Click the OAuth client name to open details"
  zrbgm_d      "      - To rotate secret if compromised:"
  zrbgm_dm     "        a. Click " "+ Add secret"
  zrbgm_d      "        b. Click the download icon next to the NEW secret"
  zrbgm_dm     "           Browser downloads: " "client_secret_[id].apps.googleusercontent.com.json"
  zrbgm_dmd    "        c. Click " "Disable" " on the secret with the older creation date"
  zrbgm_d      "        d. Click the trash icon to delete that disabled secret"
  zrbgm_e
  zrbgm_s2     "2. Install/Refresh OAuth Credentials:"
  zrbgm_d      "   Run the payor install command with the downloaded JSON:"
  zrbgm_dc     "      " "rbgp_payor_install ~/Downloads/client_secret_*.json"
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
  zrbgm_d      "   2. Set RBRR_DEPOT_PROJECT_ID to a unique value:"
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
  zrbgm_dc     "      - Project name: " "${RBRR_DEPOT_PROJECT_ID}"
  zrbgm_dm     "      - Organization: " "No organization"
  zrbgm_dm     "   4. Click " "CREATE"
  zrbgm_dmdr   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbgm_dmd    "   6. If popup, wait for notification -> " "Creating project..." " to complete"
  zrbgm_e
  zrbgm_s2     "4. Create the Admin Service Account:"
  zrbgm_dwdwd  "   1. Ensure project " "${RBRR_DEPOT_PROJECT_ID}" " is selected in the top dropdown (button with hovertext " "Open project picker (Ctrl O)" ")"
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
  zrbgm_dm     "Browser downloads: " "${RBRR_DEPOT_PROJECT_ID}-[random].json"
  zrbgm_dmd    "   6. Click " "CLOSE" " on download confirmation"
  zrbgm_e
  zrbgm_s2     "8. Configure Local Environment:"
  zrbgm_d      "Browser downloaded key.  Run the command to ingest it into your ADMIN RBRA file."
  zrbgm_e

  buc_success "Manual setup procedure displayed"
}


# eof
