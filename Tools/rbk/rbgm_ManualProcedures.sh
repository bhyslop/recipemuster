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

  local z_use_color=0
  if test -z "${NO_COLOR:-}" && test "${BURE_COLOR:-0}" = "1"; then
    z_use_color=1
  fi

  if test "${z_use_color}" = "1"; then
    readonly ZRBGM_R="\033[0m"         # Reset
    readonly ZRBGM_S="\033[1;37m"      # Section (bright white)
    readonly ZRBGM_C="\033[36m"        # Command (cyan)
    readonly ZRBGM_W="\033[35m"        # Website (magenta)
    readonly ZRBGM_Y="\033[1;33m"      # Warning (bright yellow)
    readonly ZRBGM_CR="\033[1;31m"     # Critical (bright red)
  else
    readonly ZRBGM_R=""                # No color, or disabled
    readonly ZRBGM_S=""                # No color, or disabled
    readonly ZRBGM_C=""                # No color, or disabled
    readonly ZRBGM_W=""                # No color, or disabled
    readonly ZRBGM_Y=""                # No color, or disabled
    readonly ZRBGM_CR=""               # No color, or disabled
  fi

  # Click modifier: Cmd on macOS, Ctrl elsewhere
  case "$(uname -s)" in
    Darwin) readonly ZRBGM_CLICK_MOD="Cmd" ;;
    *)      readonly ZRBGM_CLICK_MOD="Ctrl" ;;
  esac

  readonly ZRBGM_RBRP_FILE="${RBBC_rbrp_file}"
  readonly ZRBGM_RBRP_FILE_BASENAME="${ZRBGM_RBRP_FILE##*/}"
  readonly ZRBGM_RBRR_FILE="${RBBC_rbrr_file}"


  readonly ZRBGM_PREFIX="${BURD_TEMP_DIR}/rbgm_"
  readonly ZRBGM_LIST_RESPONSE="${ZRBGM_PREFIX}list_response.json"
  readonly ZRBGM_LIST_CODE="${ZRBGM_PREFIX}list_code.txt"
  readonly ZRBGM_CREATE_REQUEST="${ZRBGM_PREFIX}create_request.json"
  readonly ZRBGM_CREATE_RESPONSE="${ZRBGM_PREFIX}create_response.json"
  readonly ZRBGM_CREATE_CODE="${ZRBGM_PREFIX}create_code.txt"
  readonly ZRBGM_DELETE_RESPONSE="${ZRBGM_PREFIX}delete_response.json"
  readonly ZRBGM_DELETE_CODE="${ZRBGM_PREFIX}delete_code.txt"
  readonly ZRBGM_KEY_RESPONSE="${ZRBGM_PREFIX}key_response.json"
  readonly ZRBGM_KEY_CODE="${ZRBGM_PREFIX}key_code.txt"
  readonly ZRBGM_ROLE_RESPONSE="${ZRBGM_PREFIX}role_response.json"
  readonly ZRBGM_ROLE_CODE="${ZRBGM_PREFIX}role_code.txt"
  readonly ZRBGM_REPO_ROLE_RESPONSE="${ZRBGM_PREFIX}repo_role_response.json"
  readonly ZRBGM_REPO_ROLE_CODE="${ZRBGM_PREFIX}repo_role_code.txt"

  # Onboarding default nameplate moniker
  readonly ZRBGM_ONBOARDING_MONIKER="tadmor"
  readonly ZRBGM_ONBOARDING_NAMEPLATE="${RBBC_dot_dir}/${ZRBGM_ONBOARDING_MONIKER}/${RBCC_rbrn_file}"

  readonly ZRBGM_KINDLED=1
}

zrbgm_sentinel() {
  test "${ZRBGM_KINDLED:-}" = "1" || buc_die "Module rbgm not kindled - call zrbgm_kindle first"
}

zrbgm_enforce() {
  zrbgm_sentinel
  test -n "${RBRR_DEPOT_PROJECT_ID:-}"     || buc_die "RBRR_DEPOT_PROJECT_ID is not set"
  test   "${#RBRR_DEPOT_PROJECT_ID}" -gt 0 || buc_die "RBRR_DEPOT_PROJECT_ID is empty"
  zrbgc_sentinel
}

zrbgm_show() {
  zrbgm_sentinel
  printf '%b\n' "${1:-}"
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

  buh_section  "Manual Payor OAuth Establishment Procedure"
  buh_t        "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  buh_t        "This resolves project creation limitations for personal Google accounts."
  buh_e
  buh_section  "Key:"
  buh_tu       "   Magenta text refers to " "precise words you see on the web page."
  buh_tc       "   Cyan text is " "something you might copy from here."
  buh_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBGM_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Confirm Payor Regime:"
  buh_tc       "   File: " "${ZRBGM_RBRP_FILE}"
  buh_tc       "   RBRP_PAYOR_PROJECT_ID: " "${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  buh_e
  buh_t        "   First time setup? Set a timestamped project ID with:"
  buh_c        "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}")/' ${ZRBGM_RBRP_FILE}"
  buh_e
  buh_section  "2. Check if Project Already Exists:"
  buh_t        "   Before creating a new project, verify the configured ID is not already in use:"
  buh_link     "   1. Check existing projects: " "Google Cloud Project List" "https://console.cloud.google.com/cloud-resource-manager"
  buh_tu       "   2. Look for a project with ID " "${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "      - Hover over project IDs to verify the full ID matches your configured value"
  buh_tctu     "   3. If you " "find the project" " with matching ID, it already exists - edit " "${ZRBGM_RBRP_FILE_BASENAME}"
  buh_t        "      and re-run this procedure"
  buh_t        "   4. If you don't find it, proceed to step 3 to create it"
  buh_e
  buh_section  "3. Create Payor Project:"
  buh_link     "   1. Open browser to: " "Google Cloud Project Create" "https://console.cloud.google.com/projectcreate"
  buh_t        "   2. Ensure signed in with intended Google account (check top-right avatar)"
  buh_t        "   3. Configure new project:"
  buh_tc       "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  buh_t        "      - Project ID: Google will auto-generate a value; click Edit to replace it with:"
  buh_tc       "        " "${RBRP_PAYOR_PROJECT_ID}"
  buh_tut      "      - Location: " "No organization" " (required for this guide; organization affiliation is advanced)"
  buh_tu       "   4. Click " "CREATE"
  buh_tut      "   5. Wait for " "Creating project..." " notification to complete"
  buh_e
  buh_section  "4. Verify Project Creation:"
  buh_t        "   Verify that your rbrp.env configuration matches the created project:"
  buh_link     "   1. Test this link: " "Google Cloud APIs Dashboard" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "   2. If the page loads and shows your project, your configuration is correct"
  buh_tut      "   3. If you see " "You need additional access" ", wait a few minutes and refresh the page"
  buh_link     "      GCP IAM changes are eventually consistent: " "Access Change Propagation" "https://cloud.google.com/iam/docs/access-change-propagation"
  buh_e
  buh_section  "5. Configure Billing Account:"
  buh_link     "   1. Go to: " "Google Cloud Billing" "https://console.cloud.google.com/billing"
  buh_t        "      If no billing accounts exist:"
  buh_tu       "          a. Click " "CREATE ACCOUNT"
  buh_t        "          b. Configure payment method and submit"
  buh_tu       "          c. Copy new " "Account ID" " from table"
  buh_t        "      else if single Open account exists:"
  buh_tu       "          a. Copy the " "Account ID" " value"
  buh_t        "      else if multiple Open accounts exist:"
  buh_t        "          a. Choose account for Recipe Bottle funding"
  buh_tu       "          b. Copy chosen " "Account ID" " value"
  buh_link     "   2. Go to: " "Google Cloud Billing Projects" "https://console.cloud.google.com/billing/projects"
  buh_tu       "   3. Save the billing account ID to your " "${ZRBGM_RBRP_FILE}"
  buh_tctu     "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  buh_t        "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  buh_tu       "   5. Update " "${ZRBGM_RBRP_FILE}" " and re-display this procedure."
  buh_e
  buh_section  "6. Link Billing to Payor Project:"
  buh_t        "   Link your billing account to the newly created project."
  buh_link     "   Go to: " "Google Cloud Billing Account Management" "https://console.cloud.google.com/billing/manage"
  buh_t        "   1. The page loads to your default billing account."
  buh_tu       "   2. If you have multiple billing accounts, use the " "Select a billing account" " dropdown at top"
  buh_t        "      - Choose the account matching your RBRP_BILLING_ACCOUNT_ID"
  buh_t        "   3. Look for the section " "Projects linked to this billing account"
  buh_t        "   4. Verify your payor project appears in the table:"
  buh_tc       "      - Project name: " "${RBGC_PAYOR_APP_NAME}"
  buh_tc       "      - Project ID: " "${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "   5. If project is NOT listed and billing needs to be enabled:"
  buh_link     "      - Go to: " "Project Billing" "https://console.cloud.google.com/billing/linkedaccount?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_tu       "      - Click " "Link a Billing Account"
  buh_t        "      - Select your billing account and confirm"
  buh_e
  buh_section  "7. Enable Required APIs:"
  buh_link     "   Go to: " "APIs & Services for your payor project" "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_tu       "   1. Click " "+ ENABLE APIS AND SERVICES"
  buh_t        "   2. Search for and enable these APIs:"
  buh_tc       "      - " "Cloud Resource Manager API"
  buh_tc       "      - " "Cloud Billing API"
  buh_tct      "      - " "Service Usage API" " (often enabled by default, look for green check)"
  buh_tc       "      - " "IAM Service Account Credentials API"
  buh_tc       "      - " "Artifact Registry API"
  buh_t        "   These enable programmatic depot management operations."
  buh_e
  buh_section  "8. Configure OAuth Consent Screen:"
  buh_link     "   Go to: " "OAuth consent screen" "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_tu       "   1. The console displays " "Google Auth Platform not configured yet"
  buh_tu       "   2. Click " "Get started"
  buh_t        "   3. Complete the Project Configuration wizard:"
  buh_t        "      Step 1 - App Information:"
  buh_tc       "        - App name: " "${RBGC_PAYOR_APP_NAME}"
  buh_t        "        - User support email: (your email)"
  buh_tu       "        - Click " "Next"
  buh_t        "      Step 2 - Audience:"
  buh_tu       "        - Select " "External"
  buh_tu       "        - Click " "Next"
  buh_t        "      Step 3 - Contact Information:"
  buh_t        "        - Email addresses: (your email), press Enter"
  buh_tu       "        - Click " "Next"
  buh_t        "      Step 4 - Finish:"
  buh_tu       "        - Check " "I agree to the Google API Services: User Data Policy"
  buh_tu       "        - Click " "Continue"
  buh_tu       "        - Click " "Create"
  buh_e
  buh_t        "   4. Add your email as a test user (avoids Google app verification process):"
  buh_tu       "      1. Click " "Audience" " in left sidebar"
  buh_tu       "      2. Scroll down to section " "Test users"
  buh_tu       "      3. Click " "+ Add users"
  buh_tut      "      4. Right-side panel titled " "Add Users" " slides in"
  buh_t        "      5. Enter your email address in the field"
  buh_tu       "      6. Click " "Save"
  buh_tut      "      7. Verify " "1 user" " appears in OAuth user cap"
  buh_e
  buh_section  "9. Create OAuth 2.0 Client ID:"
  buh_link     "   Go to: " "Credentials" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_tu       "   1. From top bar, click " "+ Create credentials"
  buh_tu       "   2. Select " "OAuth client ID"
  buh_tu       "   3. Application type: " "Desktop app"
  buh_tc       "   4. Name: " "${RBGC_PAYOR_APP_NAME}"
  buh_tu       "   5. Click " "CREATE"
  buh_tut      "   6. Popup titled " "OAuth client created" " displays client ID and secret"
  buh_tu       "   7. Click " "Download JSON"
  buh_tutu     "   8. Click " "OK" " ; browser downloads " "client_secret_[id].apps.googleusercontent.com.json"
  buh_tW       "      " "CRITICAL: Save securely - contains client secret"
  buh_e
  buh_section  "10. Install OAuth Credentials:"
  buh_t        "   Run:"
  buh_c        "   rbgp_payor_install ~/Downloads/payor-oauth.json"
  buh_t        "   This will guide you through OAuth authorization and complete the setup."

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
  zrbgm_d      "   - Store secure credentials in RBRR_SECRETS_DIR/rbro-payor.env"
  zrbgm_d      "   - Update RBRP_OAUTH_CLIENT_ID in rbrp.env"
  zrbgm_d      "   - Test the authentication"
  zrbgm_d      "   - Initialize depot tracking"
  zrbgm_d      "   - Reset the 6-month expiration timer"
  zrbgm_e
  zrbgm_s2     "3. Verify Installation:"
  zrbgm_d      "   Test with a simple operation:"
  buc_tabtarget "${RBZ_LIST_DEPOT}"
  zrbgm_d      "   Should display current depots without authentication errors."
  zrbgm_e
  zrbgm_d      "Prevention: Run any Payor operation monthly to prevent expiration."

  buc_success "OAuth credential installation/refresh procedure displayed"
}

rbgm_quota_build() {
  zrbgm_sentinel

  buc_doc_brief "Display the Cloud Build capacity review procedure to verify machine type and quota settings"
  buc_doc_shown || return 0

  buh_section  "Cloud Build Concurrent Build Capacity"
  buh_t        "Review your build capacity settings to ensure sufficient concurrent build execution."
  buh_t        "Recipe Bottle uses a private worker pool — quota is tracked per private pool host project."
  buh_e
  buh_t        "   Private pool machine types vs concurrency at 10-CPU quota:"
  buh_tc       "     e2-standard-2  " "(2 vCPU)   → 5 concurrent builds"
  buh_tc       "     e2-standard-8  " "(8 vCPU)   → 1 concurrent build"
  buh_tc       "     e2-standard-32 " "(32 vCPU)  → needs 32+ CPU quota"
  buh_e
  buh_section  "Key:"
  buh_tu       "   Magenta text refers to " "precise words you see on the web page."
  buh_tc       "   Cyan text is " "something you might copy from here."
  buh_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBGM_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Current Regime Configuration:"
  buh_tc       "   RBRR_DEPOT_PROJECT_ID:          " "${RBRR_DEPOT_PROJECT_ID}"
  buh_tc       "   RBRR_GCP_REGION:                " "${RBRR_GCP_REGION}"
  buh_tc       "   RBRR_GCB_MACHINE_TYPE:          " "${RBRR_GCB_MACHINE_TYPE}"
  buh_tc       "   RBRR_GCB_POOL_STEM:             " "${RBRR_GCB_POOL_STEM}"
  buh_tc       "   RBRR_GCB_MIN_CONCURRENT_BUILDS: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS}"
  buh_e
  buh_t        "   The build preflight gate checks quota automatically before each build."
  buh_t        "   It computes: quota_vCPUs / machine_vCPUs >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  buh_e
  buh_section  "2. Check CPU Quota:"
  buh_t        "   Private pool quota is tracked under the depot project."
  buh_e
  buh_link     "   Go to: " "Quotas & System Limits (opens to depot project)" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  buh_tu       "   1. Verify project " "${RBRR_DEPOT_PROJECT_ID}" " is selected in the project picker"
  buh_tut      "   2. In the " "Enter property name or value" " filter bar, type:"
  buh_tc       "      " "concurrent_private"
  buh_tut      "   3. Select " "cloudbuild.googleapis.com/concurrent_private_pool_build_cpus" " from the autocomplete"
  buh_t        "   4. Multiple rows appear. Look for the row with Type column showing"
  buh_tut      "      " "Quota" " (not System limit) and your region in the Dimensions column"
  buh_t        "   5. Note the quota value and current usage percentage"
  buh_t        "      If usage is near 100% with one build, the machine type is too large for the quota"
  buh_e
  buh_section  "3. Request a Quota Increase (if needed):"
  buh_tut      "   On the " "Quota" " row identified above:"
  buh_tut      "   1. Click the three-dot menu " "⋮" " at the right end of the row"
  buh_tu       "   2. Select " "Edit quota"
  buh_tu       "   3. In the side panel, enter the new value in the " "New value" " field"
  buh_t        "      Recommended: 10 (allows 5 concurrent e2-standard-2 builds across both pools)"
  buh_tu       "   4. A " "Request description" " field appears. Enter:"
  buh_tc       "      " "Need parallel builds on private worker pool for CI/CD pipeline testing."
  buh_tu       "   5. Click " "Next"
  buh_t        "   6. Step 2/2 shows contact details (pre-filled from your Google account)"
  buh_tu       "   7. Click " "Submit request"
  buh_t        "      Increases are typically approved within minutes."
  buh_e
  buh_section  "4. Confirm Quota Increase:"
  buh_t        "   After approval, quotas can take up to 15 minutes to propagate."
  buh_link     "   Return to: " "Quotas & System Limits (opens to depot project)" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  buh_tut      "   Filter for " "concurrent_private" " again and verify the new value"
  buh_t        "   Verify: quota / vCPUs per machine type >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  buh_tc       "     Current target: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS} concurrent builds"
  buh_e

  buc_success "Cloud Build quota guide displayed"
}


# Dashboard status line — no sentinel (used pre-kindle by onboarding guide)
zrbgm_po_status() {
  local -r z_flag="${1:-}"
  local -r z_text="${2:-}"
  if test "${z_flag}" = "1"; then
    buh_ct " [*] " "${z_text}"
  else
    buh_t " [ ] ${z_text}"
  fi
}

# Extract a KEY=VALUE from a file; stdout empty if missing.  No sourcing.
zrbgm_po_extract_capture() {
  local -r z_file="${1:-}"
  local -r z_key="${2:-}"
  test -n "${z_key}"  || return 1
  test -f "${z_file}" || return 1
  local z_line=""
  while IFS= read -r z_line; do
    case "${z_line}" in "${z_key}="*) echo "${z_line#"${z_key}="}"; return 0 ;; esac
  done < "${z_file}"
  return 1
}

######################################################################
# Probe retriever walkthrough units — sets caller-scope z_ru1..z_ru4
# Requires: z_secrets_dir already set (from zrbgm_probe_role_credentials
#           or direct extraction).
# No sentinel — works pre-kindle

zrbgm_probe_retriever_units() {
  z_ru1=0; z_ru2=0; z_ru3=0; z_ru4=0

  # Unit 1: Retriever credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"; then
    z_ru1=1
  fi

  # Units 2-4 require Docker
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  # Unit 2: Any image from this depot's GAR exists locally
  local z_project_id="" z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -n "${z_region}" && test -n "${z_project_id}"; then
    local z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    if docker images --format "{{.Repository}}" 2>/dev/null | grep -q "^${z_gar_prefix}"; then
      z_ru2=1
    fi
  fi

  # Unit 3: Any crucible is charged (bottle container running)
  if docker ps --format "{{.Names}}" 2>/dev/null | grep -q -- "-bottle$"; then
    z_ru3=1
  fi

  # Unit 4: Kludge-tagged image exists (k-prefixed hallmark)
  if docker images --format "{{.Tag}}" 2>/dev/null | grep -q "^k[0-9]"; then
    z_ru4=1
  fi
}

######################################################################
# Probe director walkthrough units — sets caller-scope z_du1..z_du7
# Requires: z_secrets_dir already set (from zrbgm_probe_role_credentials
#           or direct extraction).
# No sentinel — works pre-kindle
#
# Vessel assignments per docket:
#   Unit 2,4,6: rbev-sentry-debian-slim (conjure vessel)
#   Unit 5:     rbev-bottle-plantuml    (bind vessel)
#
# Probes use local filesystem + docker only (no GAR API, no gcloud).
# Units 3-7 check docker image tags — these require the user to have
# summoned (pulled) the result locally after cloud operations.
# Unit 6 (graft) is detectable without summoning because graft tags
# the local image before pushing.

zrbgm_probe_director_units() {
  z_du1=0
  z_du2=0
  z_du3=0
  z_du4=0
  z_du5=0
  z_du6=0
  z_du7=0

  # Unit 1: Director credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_du1=1
  fi

  # Units 2-7 require Docker
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  # Build GAR image prefix for this depot
  local z_project_id=""
  local z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -z "${z_region}" || test -z "${z_project_id}"; then return 0; fi

  local -r z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
  local -r z_docker_images="${BURD_TEMP_DIR}/zrbgm_probe_director_images.txt"
  local -r z_docker_stderr="${BURD_TEMP_DIR}/zrbgm_probe_director_stderr.txt"

  # Collect docker images into temp file
  docker images --format "{{.Repository}}:{{.Tag}}" \
    > "${z_docker_images}" 2>"${z_docker_stderr}" \
    || return 0

  # Load lines matching this depot's GAR prefix into array
  local z_depot_images=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      "${z_gar_prefix}"*) z_depot_images+=("${z_line}") ;;
    esac
  done < "${z_docker_images}"

  # Single pass: match hallmark tag patterns via case
  local z_conjure_found=0
  local z_i=0
  for z_i in "${!z_depot_images[@]}"; do
    case "${z_depot_images[$z_i]}" in
      *"rbev-sentry-debian-slim:k"[0-9]*) z_du2=1 ;;
      *"rbev-sentry-debian-slim:c"[0-9]*) z_conjure_found=1 ;;
      *"rbev-bottle-plantuml:b"[0-9]*)    z_du5=1 ;;
      *"rbev-sentry-debian-slim:g"[0-9]*) z_du6=1 ;;
    esac
  done

  # Conjure implies depot foundation (reliquary + enshrine are prerequisites)
  if test "${z_conjure_found}" = "1"; then
    z_du3=1
    z_du4=1
  fi

  # Unit 7: All three modes present
  if test "${z_du4}" = "1" && test "${z_du5}" = "1" && test "${z_du6}" = "1"; then
    z_du7=1
  fi
}

######################################################################
# Probe governor walkthrough units — sets caller-scope z_gu1..z_gu3
# Requires: z_secrets_dir already set (from zrbgm_probe_role_credentials
#           or direct extraction).
# No sentinel — works pre-kindle
#
# Unit 1: Governor credential installed
# Unit 2: Retriever AND director credentials installed (governor created them)
# Unit 3: Functional verification — a GAR image has been pulled locally
#         (proves the full charter/knight → IAM → access chain works)

zrbgm_probe_governor_units() {
  z_gu1=0; z_gu2=0; z_gu3=0

  # Unit 1: Governor credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"; then
    z_gu1=1
  fi

  # Unit 2: Both retriever AND director credential files exist
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_gu2=1
  fi

  # Unit 3: Functional verification — a GAR image exists locally
  # This proves the SAs the governor created actually work (IAM grants applied).
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  local z_project_id="" z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -n "${z_region}" && test -n "${z_project_id}"; then
    local z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    if docker images --format "{{.Repository}}" 2>/dev/null | grep -q "^${z_gar_prefix}"; then
      z_gu3=1
    fi
  fi
}

######################################################################
# Probe payor walkthrough units — sets caller-scope z_pu1..z_pu4
# Requires: z_secrets_dir already set.
# No sentinel — works pre-kindle
#
# Unit 1: OAuth credential present (rbro-payor.env in secrets dir)
# Unit 2: Payor project configured (RBRP_PAYOR_PROJECT_ID non-empty)
# Unit 3: Depot provisioned (RBRR_DEPOT_PROJECT_ID non-empty)
# Unit 4: Governor SA exists (governor credential file present)

zrbgm_probe_payor_units() {
  z_pu1=0
  z_pu2=0
  z_pu3=0
  z_pu4=0

  # Unit 1: OAuth credential present
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/rbro-payor.env"; then
    z_pu1=1
  fi

  # Unit 2: Payor project configured
  if test -f "${RBBC_rbrp_file}"; then
    local z_probe_line=""
    while IFS= read -r z_probe_line; do
      case "${z_probe_line}" in RBRP_PAYOR_PROJECT_ID=?*) z_pu2=1; break ;; esac
    done < "${RBBC_rbrp_file}"
  fi

  # Unit 3: Depot provisioned
  if test -f "${RBBC_rbrr_file}"; then
    local z_probe_line=""
    while IFS= read -r z_probe_line; do
      case "${z_probe_line}" in RBRR_DEPOT_PROJECT_ID=?*) z_pu3=1; break ;; esac
    done < "${RBBC_rbrr_file}"
  fi

  # Unit 4: Governor SA credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"; then
    z_pu4=1
  fi
}

# Configuration review — displayed at level 0 (first thing a newcomer sees)
# Shows pre-selected RBRR defaults with orientation, plus BURC project structure.
# No sentinel: runs pre-kindle like the rest of the onboarding guide.
zrbgm_po_review_defaults() {
  local z_rbrr_file="${RBBC_rbrr_file}"

  # --- Pre-selected RBRR defaults ---
  buh_section "Pre-selected Defaults"
  buh_tc "  File: " "${z_rbrr_file}"
  buh_t  "  Edit this file if any defaults don't fit, then re-run this guide."
  buh_e
  buh_t  "  Infrastructure:"
  buh_tc "    RBRR_DNS_SERVER                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_DNS_SERVER)"
  buh_t  "      Resolver for bottle network connectivity checks (Google Public DNS)."
  buh_tc "    RBRR_GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
  buh_t  "      Compute Engine type for Cloud Build workers. Smallest reasonable option;"
  buh_t  "      balances cost and build speed."
  buh_tc "    RBRR_GCB_TIMEOUT               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_TIMEOUT)"
  buh_t  "      Maximum time for a single Cloud Build job (20 minutes)."
  buh_tc "    RBRR_GCB_MIN_CONCURRENT_BUILDS " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MIN_CONCURRENT_BUILDS)"
  buh_t  "      Preflight gate: require capacity for this many parallel builds."
  buh_t  "      With e2-standard-2 (2 vCPU) and default 10-CPU quota, 5 are possible."
  buh_e
  buh_t  "  Conventions:"
  buh_tc "    RBRR_VESSEL_DIR                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_VESSEL_DIR)"
  buh_t  "      Directory containing vessel specifications — what gets built into"
  buh_t  "      container images."
  buh_tc "    RBRR_GCP_REGION                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCP_REGION)"
  buh_t  "      GCP region for all infrastructure. Change if you need geographic"
  buh_t  "      proximity to a different region."
  buh_tc "    RBRR_SECRETS_DIR               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_SECRETS_DIR)"
  buh_t  "      Local directory for credential files, outside the repo for security."
  buh_t  "      Created automatically when credentials are first installed."
  buh_e

  # --- BURC project structure (read-only orientation) ---
  buh_section "Project Structure (BURC regime — rarely needs changing)"
  buh_tc "    BURC_TOOLS_DIR                 " "${BURC_TOOLS_DIR}"
  buh_t  "      Tool modules — the kit scripts that implement every operation."
  buh_tc "    BURC_TABTARGET_DIR             " "${BURC_TABTARGET_DIR}"
  buh_t  "      Launcher scripts. Tab-complete these in your terminal to run commands."
  buh_tc "    BURC_STATION_FILE              " "${BURC_STATION_FILE}"
  buh_t  "      Developer-local settings (log directory). Lives outside the repo."
  buh_tc "    BURC_TEMP_ROOT_DIR             " "${BURC_TEMP_ROOT_DIR}"
  buh_t  "      Temporary working files for each command run. Outside the repo."
  buh_tc "    BURC_OUTPUT_ROOT_DIR           " "${BURC_OUTPUT_ROOT_DIR}"
  buh_t  "      Persistent command output. Outside the repo."
  buh_tc "    BURC_MANAGED_KITS              " "${BURC_MANAGED_KITS}"
  buh_t  "      Kit modules managed by the vvx toolchain."
  buh_e
}

rbgm_onboarding() {
  # RETIRED — replaced by per-role onboarding guides (₢A3AAA)
  # Colophon rbw-gO deregistered; old body below preserved during transition.

  buc_doc_brief "RETIRED — use tt/rbw-go.OnboardMAIN.sh"
  buc_doc_shown || return 0

  buh_t "This command has been replaced by per-role onboarding guides."
  buh_tc "  Triage:    " "tt/rbw-go.OnboardMAIN.sh"
  buh_tc "  Reference: " "tt/rbw-gOr.OnboardReference.sh"
  return 0

  # --- Dead code: original onboarding body (colophon deregistered) ---

  # --- Shared state extracted once from config files ---
  local z_secrets_dir=""
  local z_vessel_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
    z_vessel_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_VESSEL_DIR") || z_vessel_dir=""
  fi

  # Vessel sigils (constants — not dependent on regime sourcing)
  local -r z_busybox_sigil="rbev-busybox"
  local -r z_sentry_sigil="rbev-sentry-debian-slim"
  local -r z_bottle_sigil="rbev-bottle-ifrit"

  # ===================================================================
  # Phase 1: Role Inventory — probe credential files independently
  # ===================================================================
  # Role = credential file present on THIS machine.  Committed repo config
  # (rbrp.env, rbrr.env) is context, not role declaration.
  local z_has_payor=0
  local z_has_governor=0
  local z_has_director=0
  local z_has_retriever=0

  # Payor: OAuth credential present (rbro-payor.env in secrets dir)
  if test -n "${z_secrets_dir}" && test -f "${z_secrets_dir}/rbro-payor.env"; then
    z_has_payor=1
  fi

  # Service account credentials: RBRA files in secrets dir
  if test -n "${z_secrets_dir}"; then
    test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"  && z_has_governor=1
    test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"   && z_has_director=1
    test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"  && z_has_retriever=1
  fi

  # ===================================================================
  # Header
  # ===================================================================
  buh_section "Recipe Bottle Onboarding Dashboard"
  buh_e

  # ===================================================================
  # Role Inventory Display
  # ===================================================================
  buh_section "Role Inventory"
  buh_t "  Credential file presence IS the role declaration. No flags, no parameters."
  buh_t "  The filesystem is the configuration."
  buh_e

  zrbgm_po_status "${z_has_payor}"     "Payor       — OAuth credential: creates/funds GCP infrastructure"
  zrbgm_po_status "${z_has_governor}"  "Governor    — Service account: administers director/retriever credentials"
  zrbgm_po_status "${z_has_director}"  "Director    — Service account: submits builds, manages images"
  zrbgm_po_status "${z_has_retriever}" "Retriever   — Service account: pulls images for local bottles"
  buh_e

  # ===================================================================
  # Credential Guidance — for absent roles
  # ===================================================================
  if test "${z_has_payor}" = "0" || test "${z_has_governor}" = "0" || \
     test "${z_has_director}" = "0" || test "${z_has_retriever}" = "0"; then
    buh_section "Credential Guidance"

    if test "${z_has_payor}" = "0"; then
      buh_t "  Payor: OAuth credential (rbro-payor.env) not found."
      if test -n "${z_secrets_dir}"; then
        buh_tc "    Expected at: " "${z_secrets_dir}/rbro-payor.env"
      fi
      buh_t "    To become the payor, run Payor Establish then Payor Install:"
      buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
      buc_tabtarget "${RBZ_PAYOR_INSTALL}" "\${HOME}/Downloads/client_secret_*.json"
      buh_e
    fi

    if test "${z_has_governor}" = "0" && test "${z_has_director}" = "0" && \
       test "${z_has_retriever}" = "0" && test -n "${z_secrets_dir}"; then
      buh_t "  Service accounts (governor, director, retriever) authenticate via"
      buh_t "  RBRA credential files. Each is a shell-sourceable .env file placed in:"
      buh_tc "    " "${z_secrets_dir}/"
      buh_t "  Required permissions: 600. Expected filenames:"
      buh_t "    ${RBCC_role_governor}/${RBCC_rbra_file}   — admin for depot project"
      buh_t "    ${RBCC_role_director}/${RBCC_rbra_file}   — executes Cloud Build operations"
      buh_t "    ${RBCC_role_retriever}/${RBCC_rbra_file}  — pulls images for local bottles"
      buh_e
      buh_t "  If you received a credential file, place it at the path above."
      buh_t "  If you are the payor, create credentials via the payor track below."
      buh_e
    fi
  fi

  # ===================================================================
  # Phase 2: Per-role Status Tracks
  # ===================================================================

  # --- Payor Track ---
  if test "${z_has_payor}" = "1"; then
    buh_section "Payor Track"

    # Sub-probes: repo-level config facts (committed, not role-specific)
    local z_has_project=0
    if test -f "${RBBC_rbrp_file}"; then
      local z_probe_line
      while IFS= read -r z_probe_line; do
        case "${z_probe_line}" in RBRP_PAYOR_PROJECT_ID=?*) z_has_project=1; break ;; esac
      done < "${RBBC_rbrp_file}"
    fi
    local z_has_depot=0
    if test -f "${RBBC_rbrr_file}"; then
      local z_probe_line
      while IFS= read -r z_probe_line; do
        case "${z_probe_line}" in RBRR_DEPOT_PROJECT_ID=?*) z_has_depot=1; break ;; esac
      done < "${RBBC_rbrr_file}"
    fi

    zrbgm_po_status "${z_has_payor}"    "  OAuth installed"
    zrbgm_po_status "${z_has_project}"  "  Project configured"
    zrbgm_po_status "${z_has_depot}"    "  Depot created"
    zrbgm_po_status "${z_has_governor}" "  Governor reset"
    buh_e

    # Next step for payor
    if test "${z_has_project}" = "0"; then
      buh_t "  Next: Configure the payor project identity."
      buh_tc "    Edit: " "${RBBC_rbrp_file}"
      buh_t "    Set RBRP_PAYOR_PROJECT_ID, then run:"
      buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
    elif test "${z_has_depot}" = "0"; then
      buh_t "  Next: Create the GCP depot project."
      buh_t "  Review RBRR defaults before proceeding — one RBRR is tied to one depot."
      buh_e
      buh_tc "    RBRR_GCP_REGION                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCP_REGION)"
      buh_tc "    RBRR_GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
      buh_tc "    RBRR_VESSEL_DIR                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_VESSEL_DIR)"
      buh_tc "    RBRR_SECRETS_DIR               " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_SECRETS_DIR)"
      buh_e
      buh_t "  Run (~2 min):"
      buc_tabtarget "${RBZ_LEVY_DEPOT}" "<depot-name>"
    elif test "${z_has_governor}" = "0"; then
      buh_t "  Next: Mantle the governor service account."
      buh_t "  Run:"
      buc_tabtarget "${RBZ_MANTLE_GOVERNOR}"
    else
      buh_t "  Payor track complete. Governor, director, and retriever credentials"
      buh_t "  are issued from the governor track (below) or distributed to machines."
    fi
    buh_e
  fi

  # --- Director Track ---
  if test "${z_has_director}" = "1"; then
    buh_section "Director Track"

    # Sub-probe: reliquary inscribed (busybox RBRV_RELIQUARY non-empty)
    local z_has_reliquary=0
    local z_busybox_reliquary=""
    if test -n "${z_vessel_dir}"; then
      z_busybox_reliquary=$(zrbgm_po_extract_capture "${z_vessel_dir}/${z_busybox_sigil}/rbrv.env" "RBRV_RELIQUARY") || z_busybox_reliquary=""
      test -n "${z_busybox_reliquary}" && z_has_reliquary=1
    fi

    # Sub-probe: vessels built (IMAGE_1_ANCHOR non-empty for each)
    # Preserve bottle/sentry anchors for enshrine dedup detection
    local z_has_busybox_built=0
    local z_has_bottle_built=0
    local z_has_sentry_built=0
    local z_bottle_anchor=""
    local z_sentry_anchor=""
    if test -n "${z_vessel_dir}"; then
      local z_tmp=""
      z_tmp=$(zrbgm_po_extract_capture "${z_vessel_dir}/${z_busybox_sigil}/rbrv.env" "RBRV_IMAGE_1_ANCHOR") || z_tmp=""
      test -n "${z_tmp}" && z_has_busybox_built=1
      z_bottle_anchor=$(zrbgm_po_extract_capture "${z_vessel_dir}/${z_bottle_sigil}/rbrv.env" "RBRV_IMAGE_1_ANCHOR") || z_bottle_anchor=""
      test -n "${z_bottle_anchor}" && z_has_bottle_built=1
      z_sentry_anchor=$(zrbgm_po_extract_capture "${z_vessel_dir}/${z_sentry_sigil}/rbrv.env" "RBRV_IMAGE_1_ANCHOR") || z_sentry_anchor=""
      test -n "${z_sentry_anchor}" && z_has_sentry_built=1
    fi

    local z_all_built=0
    if test "${z_has_busybox_built}" = "1" && test "${z_has_bottle_built}" = "1" && \
       test "${z_has_sentry_built}" = "1"; then
      z_all_built=1
    fi

    zrbgm_po_status "${z_has_director}"     "  Credentials present"
    zrbgm_po_status "${z_has_reliquary}"    "  Reliquary inscribed"
    zrbgm_po_status "${z_has_busybox_built}" "  Busybox built (airgap)"
    zrbgm_po_status "${z_has_bottle_built}"  "  Ifrit bottle built (Debian slim, tether)"
    zrbgm_po_status "${z_has_sentry_built}"  "  Sentry built (Debian slim, tether)"
    buh_e

    # Next step for director
    if test "${z_has_reliquary}" = "0"; then
      buh_t "  Next: Inscribe reliquary — freeze tool images in GAR."
      buh_t "  Run (~6 min):"
      buc_tabtarget "${RBZ_INSCRIBE_RELIQUARY}"
      buh_t "  Then record the reliquary ID in the busybox vessel:"
      buh_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}/rbrv.env"
      buh_tc "    Set RBRV_RELIQUARY=" "<reliquary-id>"
    elif test "${z_has_busybox_built}" = "0"; then
      buh_t "  Next: Airgap build — busybox on the air-gapped pool."
      buh_t "  1. Enshrine busybox base image (~2 min):"
      buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}"
      buh_t "  2. Conjure busybox (~8 min):"
      buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}"
      buh_t "  3. Vouch (verify SLSA provenance):"
      buc_tabtarget "${RBZ_VOUCH_HALLMARKS}"
    elif test "${z_has_bottle_built}" = "0"; then
      buh_t "  Next: Ifrit bottle — Debian bookworm-slim on tether pool."
      buh_t "  1. Record reliquary in bottle vessel:"
      buh_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}/rbrv.env"
      buh_tc "    Set RBRV_RELIQUARY=" "${z_busybox_reliquary:-<reliquary-id>}"
      buh_t "  2. Enshrine bottle base image (~2 min):"
      buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}"
      buh_t "  3. Conjure ifrit bottle (~8 min):"
      buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}"
      buh_t "  4. Vouch:"
      buc_tabtarget "${RBZ_VOUCH_HALLMARKS}"
    elif test "${z_has_sentry_built}" = "0"; then
      buh_t "  Next: Sentry — Debian bookworm-slim on tether pool."
      buh_t "  1. Record reliquary in sentry vessel:"
      buh_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}/rbrv.env"
      buh_tc "    Set RBRV_RELIQUARY=" "${z_busybox_reliquary:-<reliquary-id>}"
      if test -n "${z_sentry_anchor}" && test -n "${z_bottle_anchor}" && \
         test "${z_sentry_anchor}" = "${z_bottle_anchor}"; then
        buh_t "  2. Enshrine: SKIP — same base image already enshrined (shared enshrine namespace)"
      else
        buh_t "  2. Enshrine sentry base image (~2 min):"
        buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}"
      fi
      buh_t "  3. Conjure sentry (~5 min):"
      buc_tabtarget "${RBZ_ORDAIN_HALLMARK}" "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}"
      buh_t "  4. Vouch:"
      buc_tabtarget "${RBZ_VOUCH_HALLMARKS}"
    else
      buh_t "  Director track complete. All vessels built and vouched."
    fi
    buh_e
  fi

  # --- Retriever Track ---
  if test "${z_has_retriever}" = "1"; then
    buh_section "Retriever Track"

    # Sub-probe: nameplate hallmarks populated
    local z_has_bottle_hallmark=0
    local z_has_sentry_hallmark=0
    if test -f "${ZRBGM_ONBOARDING_NAMEPLATE}"; then
      source "${ZRBGM_ONBOARDING_NAMEPLATE}"
      test -n "${RBRN_BOTTLE_HALLMARK:-}" && z_has_bottle_hallmark=1
      test -n "${RBRN_SENTRY_HALLMARK:-}" && z_has_sentry_hallmark=1
    fi

    # Sub-probe: vouch images present in local runtime
    local z_has_bottle_summoned=0
    local z_has_sentry_summoned=0
    if test "${z_has_bottle_hallmark}" = "1" && test "${z_has_sentry_hallmark}" = "1" && \
       test -f "${RBBC_rbrr_file}"; then
      source "${RBBC_rbrr_file}"
      local z_gar_host="${RBRR_GCP_REGION}-docker.pkg.dev"
      local z_bottle_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_HALLMARK}-vouch"
      local z_sentry_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-vouch"
      "${RBRN_RUNTIME}" image inspect "${z_bottle_vouch_ref}" >/dev/null 2>&1 && z_has_bottle_summoned=1
      "${RBRN_RUNTIME}" image inspect "${z_sentry_vouch_ref}" >/dev/null 2>&1 && z_has_sentry_summoned=1
    fi

    zrbgm_po_status "${z_has_retriever}"           "  Credentials present"
    zrbgm_po_status "${z_has_bottle_hallmark}"  "  Bottle hallmark recorded"
    zrbgm_po_status "${z_has_sentry_hallmark}"  "  Sentry hallmark recorded"
    zrbgm_po_status "${z_has_bottle_summoned}"      "  Bottle image summoned"
    zrbgm_po_status "${z_has_sentry_summoned}"      "  Sentry image summoned"
    buh_e

    # Next step for retriever
    if test "${z_has_bottle_hallmark}" = "0"; then
      buh_t "  Next: Record bottle hallmark in the tadmor nameplate."
      buh_t "  1. Edit the nameplate:"
      buh_tc "    " "${ZRBGM_ONBOARDING_NAMEPLATE}"
      buh_t "    Set (substitute your actual hallmark value):"
      buh_tc "    RBRN_BOTTLE_HALLMARK=" "<hallmark>"
      buh_t "  2. Summon bottle:"
      buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_BOTTLE_VESSEL:-${z_bottle_sigil}} <hallmark>"
    elif test "${z_has_sentry_hallmark}" = "0"; then
      buh_t "  Next: Record sentry hallmark in the tadmor nameplate."
      buh_t "  1. Edit the nameplate:"
      buh_tc "    " "${ZRBGM_ONBOARDING_NAMEPLATE}"
      buh_t "    Set (substitute your actual hallmark value):"
      buh_tc "    RBRN_SENTRY_HALLMARK=" "<hallmark>"
      buh_t "  2. Summon sentry:"
      buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_SENTRY_VESSEL:-${z_sentry_sigil}} <hallmark>"
    elif test "${z_has_bottle_summoned}" = "0" || test "${z_has_sentry_summoned}" = "0"; then
      buh_t "  Next: Summon missing images locally."
      if test "${z_has_bottle_summoned}" = "0"; then
        buh_t "  Summon bottle:"
        buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_BOTTLE_VESSEL:-${z_bottle_sigil}} ${RBRN_BOTTLE_HALLMARK:-<hallmark>}"
      fi
      if test "${z_has_sentry_summoned}" = "0"; then
        buh_t "  Summon sentry:"
        buc_tabtarget "${RBZ_SUMMON_HALLMARK}" "${RBRN_SENTRY_VESSEL:-${z_sentry_sigil}} ${RBRN_SENTRY_HALLMARK:-<hallmark>}"
      fi
    else
      buh_t "  Retriever track complete. Run the tadmor security tests:"
      buh_tc "    " "tt/rbw-tf.TestFixture.tadmor-security.sh"
      buh_e
      buh_t "  Or start a bottle:"
      buc_tabtarget "${RBZ_CRUCIBLE_CHARGE}" "tadmor"
    fi
    buh_e
  fi

  # ===================================================================
  # No roles detected — first-time user guidance
  # ===================================================================
  if test "${z_has_payor}" = "0" && test "${z_has_governor}" = "0" && \
     test "${z_has_director}" = "0" && test "${z_has_retriever}" = "0"; then
    buh_section "Next Step"
    buh_t "  No roles detected on this machine. Two paths forward:"
    buh_e
    buh_t "  A. Full setup (you are the payor):"
    buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
    buh_e
    buh_t "  B. Retriever-only (you received credential files):"
    buh_t "     Place ${RBCC_role_retriever}/${RBCC_rbra_file} in RBRR_SECRETS_DIR and re-run this guide."
  fi

}

######################################################################
# Onboarding — shared credential probing (no sentinel, pre-kindle)
#
# Sets caller-scoped: z_has_payor, z_has_governor, z_has_director,
# z_has_retriever, z_secrets_dir

zrbgm_probe_role_credentials() {
  z_has_payor=0
  z_has_governor=0
  z_has_director=0
  z_has_retriever=0
  z_secrets_dir=""

  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  if test -n "${z_secrets_dir}"; then
    test -f "${z_secrets_dir}/rbro-payor.env"                          && z_has_payor=1
    test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"  && z_has_governor=1
    test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"  && z_has_director=1
    test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}" && z_has_retriever=1
  fi
}

######################################################################
# Onboarding triage — helper and entry point

# Args: detected(0|1) role_name colophon
zrbgm_triage_role() {
  local -r z_detected="${1}" z_name="${2}" z_colophon="${3}"
  local -r z_url="${RBGC_PUBLIC_DOCS_URL}#${z_name}"
  # Column pad lives OUTSIDE the link envelope so the underline/OSC-8 stop
  # at the end of the role name.  Width 13 = 12-char column + 1 separator.
  local z_pad=""
  printf -v z_pad '%*s' $((13 - ${#z_name})) ''
  if test "${z_detected}" = "1"; then
    buh_tltT " [*] " "${z_name}" "${z_url}" "${z_pad}" "${z_colophon}"
  else
    buh_tl   " [ ] " "${z_name}" "${z_url}"
  fi
}

rbgm_onboard_triage() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Detect credential roles and route to per-role onboarding walkthrough"
  buc_doc_shown || return 0

  local z_has_payor z_has_governor z_has_director z_has_retriever z_secrets_dir
  zrbgm_probe_role_credentials

  local -r z_docs="${RBGC_PUBLIC_DOCS_URL}"

  buh_section "Recipe Bottle Onboarding"
  buh_e
  buh_tlt "  " "Recipe Bottle" "${z_docs}" " builds container images with supply-chain provenance"
  buh_t   "  and runs untrusted containers behind enforced network isolation."
  buh_e

  # Each role: detected → walkthrough tabtarget, absent → docs link
  zrbgm_triage_role "${z_has_retriever}" "Retriever" "${RBZ_ONBOARD_RETRIEVER}"
  zrbgm_triage_role "${z_has_director}"  "Director"  "${RBZ_ONBOARD_DIRECTOR}"
  zrbgm_triage_role "${z_has_governor}"  "Governor"  "${RBZ_ONBOARD_GOVERNOR}"
  zrbgm_triage_role "${z_has_payor}"     "Payor"     "${RBZ_ONBOARD_PAYOR}"

  buh_e
  buh_t  "  For a full health dashboard across all roles:"
  buh_tT "    " "${RBZ_ONBOARD_REFERENCE}"

}

######################################################################
# Onboarding reference — all roles, all units, single health dashboard

rbgm_onboard_reference() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Reference dashboard — all roles, all units, current probe status"
  buc_doc_shown || return 0

  local z_has_payor z_has_governor z_has_director z_has_retriever z_secrets_dir
  zrbgm_probe_role_credentials

  buh_section "Recipe Bottle — Onboarding Reference"
  buh_e
  buh_t  "  Health dashboard across all roles. Re-run anytime to check status."
  buh_e

  # Retriever — full per-unit probes
  buh_section "Retriever"
  local z_ru1 z_ru2 z_ru3 z_ru4
  zrbgm_probe_retriever_units
  zrbgm_po_status "${z_ru1}" "  Credential gate — SA key installed"
  zrbgm_po_status "${z_ru2}" "  First artifact — hallmark summoned locally"
  zrbgm_po_status "${z_ru3}" "  Container runtime — crucible charged"
  zrbgm_po_status "${z_ru4}" "  Local experimentation — kludge image present"
  buh_tc "  Walkthrough: " "tt/rbw-gOR.OnboardRetriever.sh"
  buh_e

  # Director — full per-unit probes
  buh_section "Director"
  local z_du1=0
  local z_du2=0
  local z_du3=0
  local z_du4=0
  local z_du5=0
  local z_du6=0
  local z_du7=0
  zrbgm_probe_director_units
  zrbgm_po_status "${z_du1}" "  Credential gate — director SA key installed"
  zrbgm_po_status "${z_du2}" "  Local build — kludge image present"
  zrbgm_po_status "${z_du3}" "  Depot foundation — base images enshrined"
  zrbgm_po_status "${z_du4}" "  Conjure — production build image present"
  zrbgm_po_status "${z_du5}" "  Bind — pinned upstream image present"
  zrbgm_po_status "${z_du6}" "  Graft — locally-built image pushed"
  zrbgm_po_status "${z_du7}" "  Full ark — all three modes compared"
  buh_tc "  Walkthrough: " "tt/rbw-gOD.OnboardDirector.sh"
  buh_e

  # Governor — full per-unit probes
  buh_section "Governor"
  local z_gu1 z_gu2 z_gu3
  zrbgm_probe_governor_units
  zrbgm_po_status "${z_gu1}" "  Project access — governor credentials installed"
  zrbgm_po_status "${z_gu2}" "  Service accounts — retriever and director SAs provisioned"
  zrbgm_po_status "${z_gu3}" "  Verification — downstream roles can access the depot"
  buh_tc "  Walkthrough: " "tt/rbw-gOG.OnboardGovernor.sh"
  buh_e

  # Payor — full per-unit probes
  buh_section "Payor"
  local z_pu1=0
  local z_pu2=0
  local z_pu3=0
  local z_pu4=0
  zrbgm_probe_payor_units
  zrbgm_po_status "${z_pu1}" "  OAuth bootstrap — credentials installed"
  zrbgm_po_status "${z_pu2}" "  Project setup — GCP project configured"
  zrbgm_po_status "${z_pu3}" "  Depot provisioning — infrastructure levied"
  zrbgm_po_status "${z_pu4}" "  Governor handoff — governor SA created"
  buh_tc "  Walkthrough: " "tt/rbw-gOP.OnboardPayor.sh"
  buh_e

}

######################################################################
# Onboarding role walkthroughs — dual-mode rendering (₢A3AAB-E)

rbgm_onboard_retriever() {
  buc_doc_brief "Retriever walkthrough — pull and run vessel images"
  buc_doc_shown || return 0

  local -r z_docs="${RBGC_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_ru1 z_ru2 z_ru3 z_ru4
  zrbgm_probe_retriever_units

  # --- Count progress ---
  local z_done=0
  if test "${z_ru1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru4}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=4

  # --- Header ---
  buh_section "Retriever Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbgm_po_status 1 "  Credential gate — SA key installed"
    zrbgm_po_status 1 "  First artifact — hallmark summoned locally"
    zrbgm_po_status 1 "  Container runtime — crucible charged"
    zrbgm_po_status 1 "  Local experimentation — kludge image present"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_ru1}" = "0"; then
      # ---- Unit 1: Credential Gate ----
      buh_section "  Credential Gate"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "retriever" "${z_docs}#Retriever" " is a role with read access to a depot — you pull and run"
      buh_t   "  container images that others have built."
      buh_e
      buh_t   "  To access a depot, you need a service account key. Your governor creates"
      buh_t   "  one by running:"
      buh_tc  "    " "tt/rbw-aC.GovernorChartersRetriever.sh"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_ru2}" = "0"; then
      # ---- Unit 2: First Artifact ----
      buh_section "  First Artifact"
      buh_e
      buh_tlt "  A " "vessel" "${z_docs}#Vessel" " is a specification for a container image."
      buh_tlt "  A " "hallmark" "${z_docs}#Hallmark" " is a specific build instance of a vessel, identified by"
      buh_t   "  timestamp."
      buh_e
      buh_tlt "  " "Summon" "${z_docs}#Summon" " pulls a hallmark image from the depot to your local machine:"
      buh_tc  "    " "tt/rbw-hs.RetrieverSummonsHallmark.sh"
      buh_e
      buh_t   "  After summoning, inspect the artifact's provenance:"
      buh_tc  "    " "tt/rbw-hpf.RetrieverPlumbsFull.sh"
      buh_tc  "    " "tt/rbw-hpc.RetrieverPlumbsCompact.sh"
      buh_e
      buh_tlt "  A " "vouch" "${z_docs}#Vouch" " is cryptographic attestation proving the artifact was built"
      buh_t   "  by trusted infrastructure."
      buh_tlt "  " "Plumb" "${z_docs}#Plumb" " lets you inspect the SBOM, build info, and vouch chain —"
      buh_t   "  this is how you know what you're running."

    elif test "${z_ru3}" = "0"; then
      # ---- Unit 3: Container Runtime ----
      buh_section "  Container Runtime"
      buh_e
      buh_tlt "  A " "bottle" "${z_docs}#Bottle" " is your workload container, running unmodified in a controlled"
      buh_t   "  network environment."
      buh_tlt "  A " "nameplate" "${z_docs}#Nameplate" " ties a sentry and bottle together into a runnable unit."
      buh_e
      buh_tlt "  The " "sentry" "${z_docs}#Sentry" " enforces network policies via iptables and dnsmasq."
      buh_tlt "  The " "pentacle" "${z_docs}#Pentacle" " establishes the network namespace shared with the bottle."
      buh_e
      buh_tlt "  " "Charge" "${z_docs}#Charge" " starts the sentry/pentacle/bottle triad:"
      buh_tc  "    " "tt/rbw-cC.Charge.tadmor.sh"
      buh_e
      buh_t   "  Shell into the bottle and look around:"
      buh_tc  "    " "tt/rbw-cr.Rack.sh tadmor"
      buh_e
      buh_tlt "  When done, " "quench" "${z_docs}#Quench" " stops and cleans up:"
      buh_tc  "    " "tt/rbw-cQ.Quench.tadmor.sh"

    else
      # ---- Unit 4: Local Experimentation ----
      buh_section "  Local Experimentation"
      buh_e
      buh_tlt "  " "Kludge" "${z_docs}#Kludge" " builds a vessel image locally for fast iteration — no registry"
      buh_t   "  push, no director credentials needed:"
      buh_tc  "    " "tt/rbw-hk.LocalKludge.sh"
      buh_e
      buh_t   "  After kludging, charge a nameplate to test your local build, then rack in"
      buh_t   "  and look around. Kludge is the retriever's experimentation tool — iterate"
      buh_t   "  on your local environment without Cloud Build."
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

rbgm_onboard_director() {
  buc_doc_brief "Director walkthrough — build and publish vessel images"
  buc_doc_shown || return 0

  local -r z_docs="${RBGC_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_du1=0
  local z_du2=0
  local z_du3=0
  local z_du4=0
  local z_du5=0
  local z_du6=0
  local z_du7=0
  zrbgm_probe_director_units

  # --- Count progress ---
  local z_done=0
  if test "${z_du1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du4}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du5}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du6}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du7}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=7

  # --- Header ---
  buh_section "Director Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbgm_po_status 1 "  Credential gate — director SA key installed"
    zrbgm_po_status 1 "  Local build — kludge image present"
    zrbgm_po_status 1 "  Depot foundation — base images enshrined"
    zrbgm_po_status 1 "  Conjure — production build image present"
    zrbgm_po_status 1 "  Bind — pinned upstream image present"
    zrbgm_po_status 1 "  Graft — locally-built image pushed"
    zrbgm_po_status 1 "  Full ark — all three modes compared"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_du1}" = "0"; then
      # ---- Unit 1: Credential Gate ----
      buh_section "  Credential Gate"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "director" "${z_docs}#Director" " is a role with build and publish access to a depot —"
      buh_t   "  you create container images and push them to the registry."
      buh_e
      buh_t   "  Where a retriever can only pull, a director can build, push, and manage"
      buh_t   "  artifacts. Your governor knighted this service account for build operations."
      buh_e
      buh_t   "  To access a depot, you need a service account key. Your governor creates"
      buh_t   "  one by running:"
      buh_tc  "    " "tt/rbw-aK.GovernorKnightsDirector.sh"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_du2}" = "0"; then
      # ---- Unit 2: Kludge — Local Build ----
      buh_section "  Kludge: Local Build"
      buh_e
      buh_tlt "  A " "vessel" "${z_docs}#Vessel" " is a specification for a container image — a Dockerfile,"
      buh_t   "  build context, and metadata defining what gets built."
      buh_e
      buh_tlt "  " "Kludge" "${z_docs}#Kludge" " builds a vessel image locally using Docker — no Cloud Build"
      buh_t   "  setup needed, no registry push. The fastest way to see a vessel come to life."
      buh_e
      buh_t   "  Build the sentry vessel locally:"
      buh_tc  "    " "tt/rbw-hk.LocalKludge.sh"
      buh_e
      buh_tlt "  After kludging, test by " "charging" "${z_docs}#Charge" " a crucible and shelling in:"
      buh_tc  "    " "tt/rbw-cC.Charge.tadmor.sh"
      buh_tc  "    " "tt/rbw-cr.Rack.sh tadmor"
      buh_e
      buh_t   "  Later units teach how to build this same vessel via Cloud Build for production,"
      buh_t   "  and how to push your local build to the registry."

    elif test "${z_du3}" = "0"; then
      # ---- Unit 3: Depot Foundation — Reliquary and Enshrine ----
      buh_section "  Depot Foundation: Reliquary and Enshrine"
      buh_e
      buh_t   "  Before Cloud Build can create production images, the depot needs two kinds"
      buh_t   "  of upstream images mirrored into your registry:"
      buh_e
      buh_tlt "  An " "ark" "${z_docs}#Ark" " is an immutable container image artifact in the registry,"
      buh_t   "  produced from a vessel."
      buh_e
      buh_t   "  Tool images (reliquary): gcloud, docker, syft, skopeo, binfmt — the"
      buh_t   "  tools that Cloud Build steps consume during a build."
      buh_e
      buh_tlt "  Base images (" "enshrine" "${z_docs}#Enshrine" "): the upstream images that vessels build FROM."
      buh_t   "  Mirrored into your depot's registry with content-addressed anchors."
      buh_e
      buh_t   "  Mirror tool images into the depot:"
      buh_tc  "    " "tt/rbw-dI.DirectorInscribesReliquary.sh"
      buh_e
      buh_t   "  Enshrine base images for the sentry vessel:"
      buh_tc  "    " "tt/rbw-dE.DirectorEnshrinesVessel.sh"
      buh_e
      buh_t   "  Reliquary provides the tools; enshrine provides the foundations."
      buh_tltlt "  Both must be in place before " "conjure" "${z_docs}#Conjure" " or " "bind" "${z_docs}#Bind" "."
      buh_e
      buh_t   "  After completing both, proceed to conjure your first production build."
      buh_tlt "  The probe for this step turns green when a conjure " "hallmark" "${z_docs}#Hallmark" ""
      buh_tlt "  is " "summoned" "${z_docs}#Summon" " locally (next step)."

    elif test "${z_du4}" = "0"; then
      # ---- Unit 4: Conjure — Production Build ----
      # (Frontier only if du3 green but du4 red — rare due to shared probe,
      #  but shown in reference mode as separate unit)
      buh_section "  Conjure: Production Build"
      buh_e
      buh_tlt "  A " "hallmark" "${z_docs}#Hallmark" " is a specific build instance of a vessel, identified by"
      buh_t   "  timestamp."
      buh_e
      buh_tlt "  " "Ordain" "${z_docs}#Ordain" " creates a hallmark with full attestation — the production build"
      buh_t   "  command."
      buh_tlt "  " "Conjure" "${z_docs}#Conjure" " is the ordain mode where Cloud Build creates the image from"
      buh_t   "  source. Every conjure produces a three-part ark: image, about (SBOM + build"
      buh_t   "  info), and vouch (DSSE signature verification)."
      buh_e
      buh_t   "  This is the same vessel you kludged locally — now Cloud Build creates it"
      buh_t   "  with full SLSA provenance:"
      buh_tc  "    " "tt/rbw-hO.DirectorOrdainsHallmark.sh"
      buh_e
      buh_tlt "  Verify with " "vouch" "${z_docs}#Vouch" " (cryptographic attestation) and"
      buh_tlt "  " "tally" "${z_docs}#Tally" " (registry inventory):"
      buh_tc  "    " "tt/rbw-hV.DirectorVouchesHallmarks.sh"
      buh_tc  "    " "tt/rbw-ht.DirectorTalliesHallmarks.sh"
      buh_e
      buh_tlt "  Then " "summon" "${z_docs}#Summon" " the hallmark locally to confirm the full pipeline:"
      buh_tc  "    " "tt/rbw-hs.RetrieverSummonsHallmark.sh"

    elif test "${z_du5}" = "0"; then
      # ---- Unit 5: Bind — Pin Upstream Image ----
      buh_section "  Bind: Pin Upstream Image"
      buh_e
      buh_tlt "  " "Bind" "${z_docs}#Bind" " mirrors a pinned upstream image into your depot. No Dockerfile,"
      buh_t   "  no build — just a content-addressed copy."
      buh_e
      buh_t   "  PlantUML is useful for rendering architecture diagrams, but its Docker Hub"
      buh_tlt "  image could send your private diagrams anywhere. Bind pins it by " "digest" "${z_docs}#Bind" " —"
      buh_tlt "  no silent updates. Then " "charge" "${z_docs}#Charge" " it as a bottle: the sentry blocks"
      buh_t   "  all egress. You get the tool without the risk."
      buh_e
      buh_t   "  Ordain the plantuml vessel in bind mode:"
      buh_tc  "    " "tt/rbw-hO.DirectorOrdainsHallmark.sh"
      buh_e
      buh_t   "  The upstream image is pulled by digest, pushed to GAR, about metadata"
      buh_tlt "  generated, and " "vouch" "${z_docs}#Vouch" " records a digest-pin verdict. No SLSA provenance —"
      buh_t   "  the image was not built here, but it is pinned and bottled."
      buh_e
      buh_tlt "  Verify and " "summon" "${z_docs}#Summon" ":"
      buh_tc  "    " "tt/rbw-hV.DirectorVouchesHallmarks.sh"
      buh_tc  "    " "tt/rbw-hs.RetrieverSummonsHallmark.sh"

    elif test "${z_du6}" = "0"; then
      # ---- Unit 6: Graft — Push Local to Registry ----
      buh_section "  Graft: Push Local to Registry"
      buh_e
      buh_tlt "  " "Graft" "${z_docs}#Graft" " pushes a locally-built image to GAR. The image push is local"
      buh_t   "  (docker push), but about and vouch still run in Cloud Build."
      buh_e
      buh_t   "  You kludged the sentry in step 2 and conjured it in step 4. Now push your"
      buh_t   "  local build to the registry via graft:"
      buh_tc  "    " "tt/rbw-hO.DirectorOrdainsHallmark.sh"
      buh_e
      buh_t   "  One combined Cloud Build job runs about + vouch. The vouch verdict is"
      buh_t   "  GRAFTED — meaning this image was locally built, trust it at your own"
      buh_t   "  assessment."
      buh_e
      buh_t   "  The development cycle: kludge, test, graft when satisfied."

    else
      # ---- Unit 7: Full Ark — About and Vouch Pipeline ----
      buh_section "  The Full Ark: About and Vouch Pipeline"
      buh_e
      buh_t   "  Every hallmark — regardless of mode — produces the same three-part"
      buh_t   "  structure: image, about, and vouch. About contains the SBOM and"
      buh_t   "  build_info.json. Vouch contains the mode-specific verification."
      buh_e
      buh_tlt "  " "Plumb" "${z_docs}#Plumb" " lets you inspect an artifact's provenance — SBOM, build info,"
      buh_t   "  and vouch chain:"
      buh_tc  "    " "tt/rbw-hpf.RetrieverPlumbsFull.sh"
      buh_e
      buh_t   "  Run plumb against each mode's hallmark and compare:"
      buh_tlt "    - " "Conjure" "${z_docs}#Conjure" " (sentry): DSSE vouch, SLSA provenance"
      buh_tlt "    - " "Bind" "${z_docs}#Bind" " (plantuml): digest-pin vouch, no provenance"
      buh_tlt "    - " "Graft" "${z_docs}#Graft" " (sentry): GRAFTED vouch, no provenance chain"
      buh_e
      buh_t   "  The tally command shows the full registry health view — the director's"
      buh_t   "  operational dashboard:"
      buh_tc  "    " "tt/rbw-ht.DirectorTalliesHallmarks.sh"
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

rbgm_onboard_governor() {
  buc_doc_brief "Governor walkthrough — manage service accounts and access"
  buc_doc_shown || return 0

  local -r z_docs="${RBGC_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_gu1 z_gu2 z_gu3
  zrbgm_probe_governor_units

  # --- Count progress ---
  local z_done=0
  if test "${z_gu1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_gu2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_gu3}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=3

  # --- Header ---
  buh_section "Governor Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbgm_po_status 1 "  Project access — governor credentials installed"
    zrbgm_po_status 1 "  Service accounts — retriever and director SAs provisioned"
    zrbgm_po_status 1 "  Verification — downstream roles can access the depot"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_gu1}" = "0"; then
      # ---- Unit 1: Project Access ----
      buh_section "  Project Access"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "governor" "${z_docs}#Governor" " administers a depot — creating service accounts and"
      buh_t   "  managing access for those who build and run container images."
      buh_e
      buh_t   "  The governor works within a depot that the payor created. If no depot exists"
      buh_t   "  yet, that is a payor responsibility:"
      buh_tc  "    " "tt/rbw-gOP.OnboardPayor.sh"
      buh_e
      buh_t   "  To administer a depot, you need a governor service account key. Your payor"
      buh_t   "  creates one by running:"
      buh_tc  "    " "tt/rbw-aM.PayorMantlesGovernor.sh"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_gu2}" = "0"; then
      # ---- Unit 2: Service Account Lifecycle ----
      buh_section "  Service Account Lifecycle"
      buh_e
      buh_t   "  The governor provisions access for two downstream roles:"
      buh_e
      buh_tlt "  A " "retriever" "${z_docs}#Retriever" " has read access to the depot — they pull and run"
      buh_t   "  container images that others have built."
      buh_tlt "  A " "director" "${z_docs}#Director" " has build and publish access — they create container"
      buh_t   "  images and push them to the registry."
      buh_e
      buh_tlt "  " "Charter" "${z_docs}#Charter" " creates a retriever service account with read access:"
      buh_tc  "    " "tt/rbw-aC.GovernorChartersRetriever.sh"
      buh_e
      buh_tlt "  " "Knight" "${z_docs}#Knight" " creates a director service account with build access:"
      buh_tc  "    " "tt/rbw-aK.GovernorKnightsDirector.sh"
      buh_e
      buh_t   "  Each command creates the service account and applies the IAM grants it needs."
      buh_t   "  The output is an RBRA key file — hand it to the retriever or director user."
      buh_e
      buh_t   "  List issued service accounts:"
      buh_tc  "    " "tt/rbw-aL.GovernorListsServiceAccounts.sh"
      buh_e
      buh_t   "  Install both credentials locally to advance this walkthrough."
      if test -n "${z_secrets_dir}"; then
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"
      fi

    else
      # ---- Unit 3: Verification ----
      buh_section "  Verification"
      buh_e
      buh_t   "  The service accounts you created include IAM grants — each SA gets exactly"
      buh_t   "  the permissions its role requires, no more. Retriever gets read access."
      buh_t   "  Director gets read, write, and build trigger access."
      buh_e
      buh_t   "  Verify the complete chain works by pulling an artifact with the retriever"
      buh_t   "  credentials. If the retriever can access the depot, your grants are correct."
      buh_e
      buh_t   "  Run the retriever walkthrough to summon a hallmark:"
      buh_tc  "    " "tt/rbw-gOR.OnboardRetriever.sh"
      buh_e
      buh_t   "  This probe turns green when a GAR image from your depot exists locally —"
      buh_t   "  proving the retriever SA you chartered can actually access the registry."
    fi
  fi

  buh_e
  buh_tc "  Triage: " "tt/rbw-go.OnboardMAIN.sh"

}

rbgm_onboard_payor() {
  buc_doc_brief "Payor walkthrough — GCP project, billing, and OAuth setup"
  buc_doc_shown || return 0

  local -r z_docs="${RBGC_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_pu1=0
  local z_pu2=0
  local z_pu3=0
  local z_pu4=0
  zrbgm_probe_payor_units

  # --- Count progress ---
  local z_done=0
  if test "${z_pu1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu4}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=4

  # --- Header ---
  buh_section "Payor Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbgm_po_status 1 "  OAuth bootstrap — credentials installed"
    zrbgm_po_status 1 "  Project setup — GCP project configured"
    zrbgm_po_status 1 "  Depot provisioning — infrastructure levied"
    zrbgm_po_status 1 "  Governor handoff — governor SA created"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_pu1}" = "0"; then
      # ---- Unit 1: OAuth Bootstrap ----
      buh_section "  OAuth Bootstrap"
      buh_e
      buh_tlt "  The " "payor" "${z_docs}#Payor" " owns the GCP project and funds it. Unlike other roles"
      buh_t   "  that use service account keys, the payor authenticates via OAuth — representing"
      buh_t   "  the human project owner."
      buh_e
      buh_t   "  To get started, download an OAuth client secret JSON file from your GCP"
      buh_t   "  project's API credentials page, then run:"
      buh_tc  "    " "tt/rbw-gPI.PayorInstall.sh \${HOME}/Downloads/client_secret_*.json"
      buh_e
      buh_t   "  This walks you through the OAuth authorization flow and stores the credential"
      buh_t   "  securely. If you have an existing credential that has expired:"
      buh_tc  "    " "tt/rbw-gPR.PayorRefresh.sh"
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_pu2}" = "0"; then
      # ---- Unit 2: Project Setup ----
      buh_section "  Project Setup"
      buh_e
      buh_t   "  A funded GCP project is required before any infrastructure can be provisioned."
      buh_t   "  The project must have billing enabled and the OAuth consent screen configured."
      buh_e
      buh_t   "  Run the guided setup:"
      buh_tc  "    " "tt/rbw-gPE.PayorEstablish.sh"
      buh_e
      buh_t   "  This will guide you through project creation, billing enablement, and OAuth"
      buh_t   "  consent screen configuration. The project ID is recorded in regime files"
      buh_t   "  and becomes the identity for all depot operations."
      buh_e
      buh_t   "  Once complete, re-run this walkthrough to continue."

    elif test "${z_pu3}" = "0"; then
      # ---- Unit 3: Depot Provisioning ----
      buh_section "  Depot Provisioning"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored"
      buh_t   "  — a GCP project with a registry, storage bucket, and build infrastructure."
      buh_e
      buh_tlt "  To " "levy" "${z_docs}#Levy" " a depot is to provision this infrastructure. Run:"
      buh_tc  "    " "tt/rbw-dL.PayorLeviesDepot.sh"
      buh_e
      buh_t   "  This enables APIs, creates the Artifact Registry repository and Cloud Storage"
      buh_t   "  bucket, and configures Cloud Build. The depot is now ready for use."
      buh_e
      buh_t   "  List your depots to verify:"
      buh_tc  "    " "tt/rbw-dl.PayorListsDepots.sh"
      buh_e
      buh_t   "  Once provisioned, re-run this walkthrough to continue."

    else
      # ---- Unit 4: Governor Handoff ----
      buh_section "  Governor Handoff"
      buh_e
      buh_tlt "  A " "governor" "${z_docs}#Governor" " administers a depot — creating service accounts and"
      buh_t   "  managing access for those who build and run container images."
      buh_e
      buh_t   "  The payor funds the infrastructure; the governor operates it. After this"
      buh_t   "  handoff, the governor can charter retrievers and knight directors"
      buh_t   "  independently. Run:"
      buh_tc  "    " "tt/rbw-aM.PayorMantlesGovernor.sh"
      buh_e
      buh_t   "  This creates the governor service account with administrative permissions"
      buh_t   "  over the depot. Hand the resulting key file to the person who will"
      buh_t   "  administer this depot."
      buh_e
      buh_t   "  The payor's job for this depot is done unless billing or project-level"
      buh_t   "  changes are needed."
    fi
  fi

  buh_e
  buh_tc "  Triage: " "tt/rbw-go.OnboardMAIN.sh"

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
